//! Top-down evaluation of measurement uncertainty from in-house validation
//! and quality-control data.
//!
//! Implements
//!
//! * Nordtest TR 537 ed. 4 (2017), *Handbook for calculation of measurement
//!   uncertainty in environmental laboratories* — within-laboratory
//!   reproducibility u(Rw) (§5) and method/laboratory bias u(bias) from CRMs,
//!   proficiency tests and recovery (§6), u_c = √(u(Rw)² + u(bias)²) (§3);
//! * Eurachem/CITAC MUPR (2026), *Evaluation of measurement uncertainty from
//!   in-house precision and recovery data* — pooled precision (Eq. 1–7),
//!   precision models over the working range (Appendix A), mean recovery and
//!   its uncertainty (Eq. 8–12), recovery significance and homogeneity tests
//!   (Eq. 13–16), uncorrected-recovery allowance (Eq. 17–18) and combination
//!   with contributions (Eq. 19–24).
//!
//! Relative quantities may be given as fractions or in per cent, provided the
//! same convention is used throughout one calculation.

use crate::error::UncError;
use crate::special::{chi_squared_quantile, coverage_factor};

fn invalid(msg: impl Into<String>) -> UncError {
    UncError::InvalidOption(msg.into())
}

fn check_finite(values: &[f64], what: &str) -> Result<(), UncError> {
    if values.iter().all(|v| v.is_finite()) {
        Ok(())
    } else {
        Err(invalid(format!("{what}: values must be finite")))
    }
}

// ---------------------------------------------------------------------------
// Precision
// ---------------------------------------------------------------------------

/// A standard deviation together with its degrees of freedom.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PooledSd {
    pub sd: f64,
    pub dof: f64,
}

/// Pools N standard deviations s_i obtained from n_i results each
/// [MUPR Eq. (1); relative values with Eq. (2)]:
/// s_p = √(Σ(n_i − 1)s_i² / Σ(n_i − 1)), ν = Σ(n_i − 1).
pub fn pool_standard_deviations(groups: &[(f64, usize)]) -> Result<PooledSd, UncError> {
    if groups.is_empty() {
        return Err(invalid("no standard deviations to pool"));
    }
    let mut num = 0.0;
    let mut dof = 0.0;
    for &(s, n) in groups {
        if !(s.is_finite() && s >= 0.0) || n < 2 {
            return Err(invalid("each group needs s ≥ 0 and n ≥ 2"));
        }
        num += (n as f64 - 1.0) * s * s;
        dof += n as f64 - 1.0;
    }
    Ok(PooledSd { sd: (num / dof).sqrt(), dof })
}

/// Standard deviation from N duplicate pairs (x₁, x₂):
/// s = √(Σ(x₁ − x₂)² / 2N), ν = N [Nordtest TR 537 Appendices 5–6].
pub fn sd_from_duplicates(pairs: &[(f64, f64)]) -> Result<PooledSd, UncError> {
    if pairs.is_empty() {
        return Err(invalid("no duplicate pairs"));
    }
    check_finite(&pairs.iter().flat_map(|&(a, b)| [a, b]).collect::<Vec<_>>(), "duplicates")?;
    let n = pairs.len() as f64;
    let ss: f64 = pairs.iter().map(|&(a, b)| (a - b).powi(2)).sum();
    Ok(PooledSd { sd: (ss / (2.0 * n)).sqrt(), dof: n })
}

/// Relative standard deviation from duplicate pairs, pooling the relative
/// standard deviations |x₁ − x₂|/√2 / x̄ of the pairs (fraction, not per cent)
/// [Nordtest TR 537 §5.3, Appendix 5]; ν = N.
pub fn relative_sd_from_duplicates(pairs: &[(f64, f64)]) -> Result<PooledSd, UncError> {
    if pairs.is_empty() {
        return Err(invalid("no duplicate pairs"));
    }
    let mut ss = 0.0;
    for &(a, b) in pairs {
        let mean = 0.5 * (a + b);
        if !(mean.is_finite() && mean > 0.0) || !(a - b).is_finite() {
            return Err(invalid("relative duplicates need finite values with a positive mean"));
        }
        ss += ((a - b).powi(2) / 2.0) / (mean * mean);
    }
    let n = pairs.len() as f64;
    Ok(PooledSd { sd: (ss / n).sqrt(), dof: n })
}

/// Relative intermediate precision from duplicates measured in different
/// runs: the standard deviation of the normalised differences
/// (x₁ − x₂)/x̄ divided by √2 [EURACHEM CG4 (2012) Example A4.4]; ν = N − 1.
///
/// Unlike [`relative_sd_from_duplicates`] (Nordtest), the mean normalised
/// difference is subtracted, so a systematic difference between the two runs
/// is not counted as imprecision.
pub fn relative_sd_from_normalised_differences(pairs: &[(f64, f64)]) -> Result<PooledSd, UncError> {
    if pairs.len() < 2 {
        return Err(invalid("at least two duplicate pairs are needed"));
    }
    let mut r = Vec::with_capacity(pairs.len());
    for &(a, b) in pairs {
        let mean = 0.5 * (a + b);
        if !(mean.is_finite() && mean != 0.0) || !(a - b).is_finite() {
            return Err(invalid("duplicates need finite values with a non-zero mean"));
        }
        r.push((a - b) / mean);
    }
    let n = r.len() as f64;
    let m = r.iter().sum::<f64>() / n;
    let s = (r.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
    Ok(PooledSd { sd: s / 2f64.sqrt(), dof: n - 1.0 })
}

/// Relative standard deviation of the analyte amount in n portions drawn at
/// random from a sample in which a fraction p₁ of the portions contains an
/// amount l₁ and the rest l₂ (binomial model) [EURACHEM CG4 (2012) A4.6]:
/// μ = n p₁ (l₁ − l₂) + n l₂, σ² = n p₁ (1 − p₁)(l₁ − l₂)².
pub fn binomial_inhomogeneity_rsd(n_portions: f64, p1: f64, l1: f64, l2: f64) -> f64 {
    let mu = n_portions * p1 * (l1 - l2) + n_portions * l2;
    let sigma = (n_portions * p1 * (1.0 - p1)).sqrt() * (l1 - l2).abs();
    sigma / mu
}

/// d₂ constant for ranges of two values (expected range / σ = 2/√π).
pub const D2_DUPLICATES: f64 = 1.128;

/// Standard deviation from the mean range Ā of duplicates: s = Ā/1.128
/// [MUPR §8.1.3].
pub fn sd_from_mean_range(mean_range: f64) -> f64 {
    mean_range / D2_DUPLICATES
}

/// Standard uncertainty from control-chart limits ±L set at k standard
/// deviations (warning limits: k = 2) [Nordtest TR 537 §5.1, step 4].
pub fn sd_from_control_limits(limit: f64, k: f64) -> f64 {
    limit / k
}

/// Root sum of squares √(Σ u_i²) of independent standard uncertainties.
/// Two-factor staggered-nested design [ISO 5725-3:2023 7.2, Annex C.1]:
/// for each laboratory (or group) two results on day 1 and one on day 2.
#[derive(Clone, Debug, PartialEq)]
pub struct StaggeredNested {
    pub groups: usize,
    pub mean: f64,
    /// Sums of squares, degrees of freedom and mean squares for factor 0
    /// (between groups), factor 1 (between days within a group) and the
    /// residual.
    pub ss: [f64; 3],
    pub dof: [f64; 3],
    pub ms: [f64; 3],
    /// Variance estimates s₀², s₍₁₎² (negative estimates set to zero) and s_r².
    pub var_0: f64,
    pub var_1: f64,
    pub var_r: f64,
}

impl StaggeredNested {
    /// Repeatability standard deviation s_r.
    pub fn repeatability(&self) -> f64 {
        self.var_r.sqrt()
    }
    /// Time-different intermediate precision s_I(T) = √(s_r² + s₍₁₎²).
    pub fn intermediate(&self) -> f64 {
        (self.var_r + self.var_1).sqrt()
    }
    /// Reproducibility (between-group) s_R = √(s_r² + s₍₁₎² + s₀²).
    pub fn reproducibility(&self) -> f64 {
        (self.var_r + self.var_1 + self.var_0).sqrt()
    }
}

/// Analysis of variance of a two-factor staggered-nested design: each row
/// is (y₁, y₂ on day 1, y₃ on day 2). Expected mean squares: MS₀ = σ_r² +
/// (5/3)σ₍₁₎² + 3σ₀², MS₁ = σ_r² + (4/3)σ₍₁₎², MS_e = σ_r².
pub fn staggered_nested(rows: &[[f64; 3]]) -> Result<StaggeredNested, UncError> {
    let p = rows.len();
    if p < 2 {
        return Err(UncError::InvalidOption("staggered-nested design: at least two groups are needed".into()));
    }
    if rows.iter().flatten().any(|v| !v.is_finite()) {
        return Err(UncError::InvalidOption("staggered-nested design: data must be finite".into()));
    }
    let y2: Vec<f64> = rows.iter().map(|r| (r[0] + r[1] + r[2]) / 3.0).collect();
    let mean = y2.iter().sum::<f64>() / p as f64;
    let ss0 = 3.0 * y2.iter().map(|y| (y - mean).powi(2)).sum::<f64>();
    let ss1 = 2.0 / 3.0 * rows.iter().map(|r| (0.5 * (r[0] + r[1]) - r[2]).powi(2)).sum::<f64>();
    let sse = 0.5 * rows.iter().map(|r| (r[0] - r[1]).powi(2)).sum::<f64>();
    let pf = p as f64;
    let dof = [pf - 1.0, pf, pf];
    let ms = [ss0 / dof[0], ss1 / dof[1], sse / dof[2]];
    let var_r = ms[2];
    let var_1 = (0.75 * (ms[1] - ms[2])).max(0.0);
    let var_0 = ((ms[0] - ms[2] - 5.0 / 3.0 * var_1) / 3.0).max(0.0);
    Ok(StaggeredNested { groups: p, mean, ss: [ss0, ss1, sse], dof, ms, var_0, var_1, var_r })
}

pub fn root_sum_of_squares(components: &[f64]) -> f64 {
    components.iter().map(|u| u * u).sum::<f64>().sqrt()
}

/// Root mean square √(Σ x_i² / N), used for RMS_bias [Nordtest TR 537 §6].
pub fn root_mean_square(values: &[f64]) -> f64 {
    (values.iter().map(|x| x * x).sum::<f64>() / values.len() as f64).sqrt()
}

/// Intermediate-precision standard deviation of the mean of n·p results
/// obtained as n replicates on each of p days [MUPR Eq. (5)]:
/// s_I(x̿) = √(s_I²/p + s_r²(1 − n)/(n p)). With p = 1 this is Eq. (6).
pub fn intermediate_precision_of_mean(s_r: f64, s_i: f64, n: usize, p: usize) -> Result<f64, UncError> {
    if n == 0 || p == 0 {
        return Err(invalid("n and p must be at least 1"));
    }
    if s_i < s_r {
        return Err(invalid("intermediate precision s_I must not be smaller than repeatability s_r"));
    }
    let (n, p) = (n as f64, p as f64);
    Ok((s_i * s_i / p + s_r * s_r * (1.0 - n) / (n * p)).sqrt())
}

/// Intermediate precision of the mean of n·p results on heterogeneous
/// samples [MUPR Eq. (7)]: √(s_r(h)²/(n p) + (s_I² − s_r²)/p).
pub fn intermediate_precision_heterogeneous(
    s_r_h: f64,
    s_r: f64,
    s_i: f64,
    n: usize,
    p: usize,
) -> Result<f64, UncError> {
    if n == 0 || p == 0 {
        return Err(invalid("n and p must be at least 1"));
    }
    if s_i < s_r {
        return Err(invalid("intermediate precision s_I must not be smaller than repeatability s_r"));
    }
    let (n, p) = (n as f64, p as f64);
    Ok((s_r_h * s_r_h / (n * p) + (s_i * s_i - s_r * s_r) / p).sqrt())
}

/// Model of the precision standard deviation as a function of concentration
/// [MUPR Appendix A; Nordtest TR 537 §4].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrecisionModel {
    /// Constant absolute standard deviation.
    Constant { sd: f64 },
    /// Constant relative standard deviation (fraction).
    Relative { rsd: f64 },
    /// Absolute sd below `split`, relative sd at and above it (Modelling 1–3;
    /// Nordtest division of the measurement range).
    TwoIntervals { split: f64, sd_low: f64, rsd_high: f64 },
    /// s(c) = √(s₀² + (c s′₁)²) [MUPR Eq. (A.1)–(A.4)].
    Continuous { sd0: f64, rsd1: f64 },
}

impl PrecisionModel {
    /// Two-interval model split at the transition c_T = s⟨L⟩/s′⟨H⟩, where both
    /// branches agree [MUPR Appendix A, Modelling 3].
    pub fn transition(sd_low: f64, rsd_high: f64) -> Self {
        PrecisionModel::TwoIntervals { split: sd_low / rsd_high, sd_low, rsd_high }
    }

    /// Absolute precision standard deviation at concentration c.
    pub fn sd_at(&self, c: f64) -> f64 {
        match *self {
            PrecisionModel::Constant { sd } => sd,
            PrecisionModel::Relative { rsd } => rsd * c.abs(),
            PrecisionModel::TwoIntervals { split, sd_low, rsd_high } => {
                if c < split {
                    sd_low
                } else {
                    rsd_high * c.abs()
                }
            }
            PrecisionModel::Continuous { sd0, rsd1 } => (sd0 * sd0 + (c * rsd1).powi(2)).sqrt(),
        }
    }
}

// ---------------------------------------------------------------------------
// Bias (Nordtest TR 537 §6)
// ---------------------------------------------------------------------------

/// Standard uncertainty of a proficiency-test assigned value from the
/// reproducibility standard deviation s_R and number of laboratories:
/// u(C_ref,i) = s_R/√n_Lab [Nordtest TR 537 §6.2]. (If the provider reports
/// the expanded uncertainty U of the assigned value, use U/2 instead.)
pub fn u_cref_from_proficiency_test(s_r: f64, n_labs: usize) -> f64 {
    s_r / (n_labs as f64).sqrt()
}

/// u(bias) = √(RMS_bias² + ū(C_ref)²) from several CRMs, proficiency tests
/// or recovery tests, with ū(C_ref) the mean of the individual reference
/// uncertainties [Nordtest TR 537 §6].
pub fn u_bias_from_references(biases: &[f64], u_crefs: &[f64]) -> Result<f64, UncError> {
    if biases.is_empty() || u_crefs.is_empty() {
        return Err(invalid("at least one bias and one reference uncertainty are needed"));
    }
    check_finite(biases, "biases")?;
    check_finite(u_crefs, "reference uncertainties")?;
    let u_cref = u_crefs.iter().sum::<f64>() / u_crefs.len() as f64;
    Ok((root_mean_square(biases).powi(2) + u_cref * u_cref).sqrt())
}

/// u(bias) from a single CRM measured n times with mean bias b and standard
/// deviation s_bias: √(b² + (s_bias/√n)² + u(C_ref)²) [Nordtest TR 537 §6].
pub fn u_bias_single_reference(bias: f64, s_bias: f64, n: usize, u_cref: f64) -> f64 {
    (bias * bias + s_bias * s_bias / n as f64 + u_cref * u_cref).sqrt()
}

// ---------------------------------------------------------------------------
// Recovery (Eurachem/CITAC MUPR §8.2–8.3)
// ---------------------------------------------------------------------------

/// One recovery study.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RecoveryTest {
    /// Reference material independent of the laboratory, or spiked blank:
    /// mean c̄ of n results with standard deviation s, reference value C with
    /// standard uncertainty u(C) [MUPR §8.2.4].
    Reference { mean: f64, sd: f64, n: usize, reference: f64, u_reference: f64 },
    /// Spiked sample with native analyte: means after (n results) and before
    /// (m results) spiking, spiked value c₊ with u(c₊) [MUPR §8.2.5].
    Spiked {
        mean_after: f64,
        sd_after: f64,
        n_after: usize,
        mean_before: f64,
        sd_before: f64,
        n_before: usize,
        spike: f64,
        u_spike: f64,
    },
    /// A recovery R with its standard uncertainty u(R) evaluated elsewhere,
    /// e.g. by a validation spreadsheet or a published table.
    Given { recovery: f64, u: f64 },
}

impl RecoveryTest {
    fn validate(&self) -> Result<(), UncError> {
        match *self {
            RecoveryTest::Reference { mean, sd, n, reference, u_reference } => {
                check_finite(&[mean, sd, reference, u_reference], "recovery test")?;
                if n == 0 || reference == 0.0 || mean == 0.0 {
                    return Err(invalid("recovery test needs n ≥ 1 and non-zero mean and reference"));
                }
            }
            RecoveryTest::Spiked {
                mean_after,
                sd_after,
                n_after,
                mean_before,
                sd_before,
                n_before,
                spike,
                u_spike,
            } => {
                check_finite(&[mean_after, sd_after, mean_before, sd_before, spike, u_spike], "spike test")?;
                if n_after == 0 || n_before == 0 || spike == 0.0 || mean_after == mean_before {
                    return Err(invalid("spike test needs n, m ≥ 1, c₊ ≠ 0 and c̄ ≠ c̄₀"));
                }
            }
            RecoveryTest::Given { recovery, u } => {
                check_finite(&[recovery, u], "recovery")?;
                if recovery == 0.0 || u < 0.0 {
                    return Err(invalid("a given recovery needs R ≠ 0 and u(R) ≥ 0"));
                }
            }
        }
        Ok(())
    }

    /// Recovery R_i = c̄/C, or (c̄ − c̄₀)/c₊ for a spiked sample.
    pub fn recovery(&self) -> f64 {
        match *self {
            RecoveryTest::Reference { mean, reference, .. } => mean / reference,
            RecoveryTest::Spiked { mean_after, mean_before, spike, .. } => (mean_after - mean_before) / spike,
            RecoveryTest::Given { recovery, .. } => recovery,
        }
    }

    /// Standard uncertainty of R_i: the i-th term of MUPR Eq. (9) or (11).
    pub fn standard_uncertainty(&self) -> f64 {
        let r = self.recovery();
        match *self {
            RecoveryTest::Reference { mean, sd, n, reference, u_reference } => {
                r.abs() * ((sd / (mean * (n as f64).sqrt())).powi(2) + (u_reference / reference).powi(2)).sqrt()
            }
            RecoveryTest::Spiked {
                mean_after,
                sd_after,
                n_after,
                mean_before,
                sd_before,
                n_before,
                spike,
                u_spike,
            } => {
                let var_diff = sd_after * sd_after / n_after as f64 + sd_before * sd_before / n_before as f64;
                r.abs() * (var_diff / (mean_after - mean_before).powi(2) + (u_spike / spike).powi(2)).sqrt()
            }
            RecoveryTest::Given { u, .. } => u,
        }
    }
}

/// Mean recovery R̄ and its standard uncertainty u_R̄.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeanRecovery {
    pub mean: f64,
    pub standard_uncertainty: f64,
    pub tests: usize,
}

/// Unweighted mean recovery R̄ = ΣR_i/N with u_R̄ = √(Σu²(R_i))/N
/// [MUPR Eq. (8)–(12)]. Reference-material and spiked tests may be mixed.
pub fn mean_recovery(tests: &[RecoveryTest]) -> Result<MeanRecovery, UncError> {
    if tests.is_empty() {
        return Err(invalid("no recovery tests"));
    }
    for t in tests {
        t.validate()?;
    }
    let n = tests.len() as f64;
    let mean = tests.iter().map(RecoveryTest::recovery).sum::<f64>() / n;
    let u = tests.iter().map(|t| t.standard_uncertainty().powi(2)).sum::<f64>().sqrt() / n;
    Ok(MeanRecovery { mean, standard_uncertainty: u, tests: tests.len() })
}

/// Weighted mean recovery R̿ = Σ(R_i/u²(R_i))/Σ(1/u²(R_i)) [MUPR Eq. (16.1)]
/// with the standard uncertainty of a weighted mean, u(R̿) = (Σ1/u²(R_i))^(−1/2).
/// The footnote to MUPR Eq. (8) allows it when the u(R_i) differ markedly; it
/// gives the less uncertain recoveries more weight.
pub fn weighted_mean_recovery(tests: &[RecoveryTest]) -> Result<MeanRecovery, UncError> {
    if tests.is_empty() {
        return Err(invalid("no recovery tests"));
    }
    for t in tests {
        t.validate()?;
    }
    let mut w = 0.0;
    let mut wr = 0.0;
    for t in tests {
        let u = t.standard_uncertainty();
        if !(u > 0.0) {
            return Err(invalid("a weighted mean recovery needs u(R_i) > 0 for every test"));
        }
        w += 1.0 / (u * u);
        wr += t.recovery() / (u * u);
    }
    Ok(MeanRecovery { mean: wr / w, standard_uncertainty: w.powf(-0.5), tests: tests.len() })
}

/// Outcome of the test of MUPR Eq. (13) for a mean recovery equal to 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RecoverySignificance {
    /// |1 − R̄| / u_R̄.
    pub statistic: f64,
    /// Two-sided Student t value for the stated confidence level and ν.
    pub critical: f64,
    /// True if R̄ differs significantly from 1 (results should be corrected,
    /// c_C = c/R̄, Eq. (14)).
    pub significant: bool,
    /// Relative uncertainty u′_R̄ to combine: u_R̄/1 if not significant,
    /// u_R̄/R̄ otherwise [MUPR §8.2.6.2].
    pub relative_uncertainty: f64,
}

/// Tests whether R̄ differs from 1 [MUPR Eq. (13)]; `dof` is the degrees of
/// freedom associated with u_R̄ (MUPR: ν of the pooled standard deviation;
/// with more than ten tests t = 2 may be used — pass `f64::INFINITY` with
/// confidence 0.9545 for that convention).
pub fn recovery_significance(r: &MeanRecovery, dof: f64, confidence: f64) -> RecoverySignificance {
    let statistic = (1.0 - r.mean).abs() / r.standard_uncertainty;
    let critical = coverage_factor(confidence, dof);
    let significant = statistic > critical;
    let relative_uncertainty = if significant { r.standard_uncertainty / r.mean } else { r.standard_uncertainty };
    RecoverySignificance { statistic, critical, significant, relative_uncertainty }
}

/// Recovery uncertainty allowing for an *uncorrected* recovery deviation
/// [MUPR Eq. (17) for one reference, Eq. (18) for several]:
/// u_R̄ = R̄ √(Σ(R̄_i − 1)²/N + Σ(u(C_i)/C_i)²/N), with the precision term of
/// Eq. (17) included for a single reference material.
pub fn recovery_uncertainty_uncorrected(tests: &[RecoveryTest]) -> Result<f64, UncError> {
    let mr = mean_recovery(tests)?;
    let n = tests.len() as f64;
    let mut mse = 0.0;
    let mut ref_rel = 0.0;
    let mut precision = 0.0;
    for t in tests {
        match *t {
            RecoveryTest::Reference { mean, sd, n: ni, reference, u_reference } => {
                mse += (mean / reference - 1.0).powi(2);
                ref_rel += (u_reference / reference).powi(2);
                precision = (sd / (mean * (ni as f64).sqrt())).powi(2);
            }
            RecoveryTest::Spiked { .. } | RecoveryTest::Given { .. } => {
                return Err(UncError::Unsupported(
                    "MUPR Eq. (17)–(18) are stated for reference materials and spiked blanks".into(),
                ))
            }
        }
    }
    let inner = if tests.len() == 1 { mse + precision + ref_rel } else { mse / n + ref_rel / n };
    Ok(mr.mean.abs() * inner.sqrt())
}

/// Standard deviation s(R̄) of the individual recoveries R_i, used as an
/// additional component when recoveries vary with the matrix and the scope
/// cannot be divided [MUPR §8.2.7; Examples B5, B6]: combine as
/// √(u²_R̄ + s²(R̄)).
pub fn recovery_standard_deviation(tests: &[RecoveryTest]) -> Result<f64, UncError> {
    if tests.len() < 2 {
        return Err(invalid("at least two recovery tests are needed"));
    }
    let r: Vec<f64> = tests.iter().map(RecoveryTest::recovery).collect();
    let n = r.len() as f64;
    let m = r.iter().sum::<f64>() / n;
    Ok((r.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1.0)).sqrt())
}

/// |R_i − R_j| / √(u²(R_i) + u²(R_j)), the statistic of MUPR Eq. (15) and
/// (B5.7); compare with a coverage factor (e.g. 2).
pub fn recovery_difference_statistic(r1: f64, u1: f64, r2: f64, u2: f64) -> f64 {
    (r1 - r2).abs() / (u1 * u1 + u2 * u2).sqrt()
}

/// Pairwise equivalence of two recoveries [MUPR Eq. (15)]:
/// |R_i − R_j| ≤ k √(u²(R_i) + u²(R_j)).
pub fn recoveries_equivalent(r1: f64, u1: f64, r2: f64, u2: f64, k: f64) -> bool {
    (r1 - r2).abs() <= k * (u1 * u1 + u2 * u2).sqrt()
}

/// χ² test of the equivalence of N recoveries [MUPR Eq. (16)].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RecoveryHomogeneity {
    /// Weighted mean R̿ = Σ(R_k/u²_k)/Σ(1/u²_k) (Eq. 16.1).
    pub weighted_mean: f64,
    /// χ² = Σ((R_k − R̿)/u_k)² (Eq. 16.2).
    pub chi_squared: f64,
    /// Critical value χ²₀.₀₅,N−1 for the stated confidence (CHISQ.INV(0.95; N−1)).
    pub critical: f64,
    pub equivalent: bool,
}

pub fn recovery_homogeneity(values: &[(f64, f64)], confidence: f64) -> Result<RecoveryHomogeneity, UncError> {
    if values.len() < 2 {
        return Err(invalid("at least two recoveries are needed"));
    }
    if values.iter().any(|&(r, u)| !r.is_finite() || !(u > 0.0)) {
        return Err(invalid("recoveries must be finite with positive uncertainties"));
    }
    let w: f64 = values.iter().map(|&(_, u)| 1.0 / (u * u)).sum();
    let weighted_mean = values.iter().map(|&(r, u)| r / (u * u)).sum::<f64>() / w;
    let chi_squared = values.iter().map(|&(r, u)| ((r - weighted_mean) / u).powi(2)).sum();
    let critical = chi_squared_quantile(confidence, values.len() as f64 - 1.0);
    Ok(RecoveryHomogeneity { weighted_mean, chi_squared, critical, equivalent: chi_squared <= critical })
}

/// Acceptance limits R̄ ∓ t·u(R) for a single daily recovery check
/// [MUPR Example B1, Eq. (B1.4)–(B1.5)]; `u_single` is the uncertainty of one
/// recovery (Eq. (B1.2) with n = 1).
pub fn recovery_acceptance_limits(mean: f64, u_single: f64, dof: f64, confidence: f64) -> (f64, f64) {
    let t = coverage_factor(confidence, dof);
    (mean - t * u_single, mean + t * u_single)
}

/// Metrological compatibility of a measured and a reference value
/// [MUPR Eq. (B1.17)]: |c − C| ≤ k √(u(c)² + u(C)²). Returns (|c − C|, limit,
/// compatible).
pub fn metrological_compatibility(
    measured: f64,
    u_measured: f64,
    reference: f64,
    u_reference: f64,
    k: f64,
) -> (f64, f64, bool) {
    let diff = (measured - reference).abs();
    let limit = k * (u_measured * u_measured + u_reference * u_reference).sqrt();
    (diff, limit, diff <= limit)
}

/// Criterion used to accept a measured recovery against a permitted
/// interval [MUPR Appendix C1.2].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryAcceptance {
    /// Stringent acceptance (e.g. TOST): Δa = ΔP − 2 k_a u_R.
    Stringent,
    /// Simple acceptance: Δa = ΔP.
    Simple,
    /// Relaxed acceptance: Δa = ΔP + 2 k_a u_R.
    Relaxed,
}

/// Additional standard uncertainty allowing for an accepted but uncorrected
/// recovery, for a permitted interval of width ΔP symmetric about 100 %:
/// u_a = Δa/(2√3) [MUPR Eq. (C1)–(C2)].
pub fn acceptable_recovery_allowance(permitted_width: f64, k_a: f64, u_recovery: f64, rule: RecoveryAcceptance) -> f64 {
    let delta_a = match rule {
        RecoveryAcceptance::Stringent => permitted_width - 2.0 * k_a * u_recovery,
        RecoveryAcceptance::Simple => permitted_width,
        RecoveryAcceptance::Relaxed => permitted_width + 2.0 * k_a * u_recovery,
    };
    delta_a.max(0.0) / (2.0 * 3f64.sqrt())
}

/// Target (maximum admissible) expanded uncertainty from the width of a
/// classification interval, U_tg = (Q_max − Q_min)/8 [Eurachem/CITAC guide
/// on target uncertainty (2015), as applied in MUPR Example B4.4].
pub fn target_uncertainty_from_interval(q_min: f64, q_max: f64) -> f64 {
    (q_max - q_min) / 8.0
}

// ---------------------------------------------------------------------------
// Combination (Nordtest TR 537 §3; MUPR §9–11)
// ---------------------------------------------------------------------------

/// An uncertainty component of a top-down budget.
#[derive(Clone, Debug, PartialEq)]
pub struct Component {
    pub name: String,
    pub kind: ComponentKind,
    /// Number of times the component enters the budget (e.g. 2 for a stock
    /// solution used for both calibrators and spikes, MUPR Eq. (B1.6)).
    pub multiplicity: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComponentKind {
    /// Standard uncertainty in the unit of the measured value.
    Absolute(f64),
    /// Relative standard uncertainty (fraction).
    Relative(f64),
}

impl Component {
    pub fn absolute(name: &str, u: f64) -> Self {
        Self { name: name.into(), kind: ComponentKind::Absolute(u), multiplicity: 1 }
    }

    pub fn relative(name: &str, u_rel: f64) -> Self {
        Self { name: name.into(), kind: ComponentKind::Relative(u_rel), multiplicity: 1 }
    }

    pub fn times(mut self, multiplicity: u32) -> Self {
        self.multiplicity = multiplicity;
        self
    }

    fn variance_at(&self, c: f64) -> f64 {
        let m = self.multiplicity as f64;
        match self.kind {
            ComponentKind::Absolute(u) => m * u * u,
            ComponentKind::Relative(u) => m * (c * u).powi(2),
        }
    }
}

/// Result of a top-down budget at one measured value.
#[derive(Clone, Debug, PartialEq)]
pub struct TopDownResult {
    pub value: f64,
    pub combined_standard_uncertainty: f64,
    pub relative_combined_uncertainty: f64,
    pub coverage_factor: f64,
    pub expanded_uncertainty: f64,
    /// Variance shares χ of each component (MUPR Eq. (21)–(24)).
    pub contributions: Vec<(String, f64)>,
}

/// A budget of absolute and relative components combined by the law of
/// propagation of uncertainty: u_c = √(Σ u_abs² + c² Σ u′_rel²)
/// [Nordtest TR 537 §3; MUPR Eq. (19)–(20)].
#[derive(Clone, Debug, PartialEq)]
pub struct TopDownBudget {
    pub components: Vec<Component>,
    pub coverage_factor: f64,
}

impl TopDownBudget {
    pub fn new(components: Vec<Component>, coverage_factor: f64) -> Self {
        Self { components, coverage_factor }
    }

    pub fn evaluate(&self, value: f64) -> TopDownResult {
        let variances: Vec<f64> = self.components.iter().map(|c| c.variance_at(value)).collect();
        let total: f64 = variances.iter().sum();
        let uc = total.sqrt();
        let contributions = self
            .components
            .iter()
            .zip(&variances)
            .map(|(c, v)| (c.name.clone(), if total > 0.0 { v / total } else { 0.0 }))
            .collect();
        TopDownResult {
            value,
            combined_standard_uncertainty: uc,
            relative_combined_uncertainty: if value != 0.0 { uc / value.abs() } else { f64::INFINITY },
            coverage_factor: self.coverage_factor,
            expanded_uncertainty: self.coverage_factor * uc,
            contributions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pooling_and_duplicates() {
        let p = pool_standard_deviations(&[(1.0, 3), (2.0, 5)]).unwrap();
        assert!((p.sd - ((2.0 + 16.0) / 6.0f64).sqrt()).abs() < 1e-15);
        assert_eq!(p.dof, 6.0);
        let d = sd_from_duplicates(&[(1.0, 3.0), (2.0, 2.0)]).unwrap();
        assert!((d.sd - 1.0).abs() < 1e-15);
        assert!(pool_standard_deviations(&[(1.0, 1)]).is_err());
    }

    #[test]
    fn precision_of_means() {
        // One result per day on one day reproduces s_I; n → ∞ on one day leaves s_b.
        assert!((intermediate_precision_of_mean(1.0, 2.0, 1, 1).unwrap() - 2.0).abs() < 1e-15);
        let s = intermediate_precision_of_mean(1.0, 2.0, 1_000_000, 1).unwrap();
        assert!((s - 3f64.sqrt()).abs() < 1e-6);
        assert!(intermediate_precision_of_mean(2.0, 1.0, 1, 1).is_err());
        // Eq. (7) with s_r(h) = s_r equals Eq. (5).
        let a = intermediate_precision_heterogeneous(1.0, 1.0, 2.0, 3, 2).unwrap();
        let b = intermediate_precision_of_mean(1.0, 2.0, 3, 2).unwrap();
        assert!((a - b).abs() < 1e-15);
    }

    #[test]
    fn precision_models() {
        let m = PrecisionModel::transition(0.033, 0.084);
        if let PrecisionModel::TwoIntervals { split, .. } = m {
            assert!((split - 0.033 / 0.084).abs() < 1e-15);
        }
        assert_eq!(m.sd_at(0.2), 0.033);
        assert!((m.sd_at(1.0) - 0.084).abs() < 1e-15);
        let c = PrecisionModel::Continuous { sd0: 3.0, rsd1: 0.1 };
        assert!((c.sd_at(40.0) - 5.0).abs() < 1e-15);
    }

    #[test]
    fn weighted_mean_recovery_matches_eq_16_1() {
        let tests = [RecoveryTest::Given { recovery: 0.90, u: 0.01 }, RecoveryTest::Given { recovery: 1.00, u: 0.02 }];
        let w = weighted_mean_recovery(&tests).unwrap();
        assert!((w.mean - (0.90 / 1e-4 + 1.00 / 4e-4) / (1.0 / 1e-4 + 1.0 / 4e-4)).abs() < 1e-15);
        assert!((w.standard_uncertainty - (1.0f64 / (1e4 + 2.5e3)).sqrt()).abs() < 1e-15);
        let h = recovery_homogeneity(&[(0.90, 0.01), (1.00, 0.02)], 0.95).unwrap();
        assert!((w.mean - h.weighted_mean).abs() < 1e-15);
        let a = mean_recovery(&tests).unwrap();
        assert!((a.mean - 0.95).abs() < 1e-15);
        assert!((a.standard_uncertainty - (5e-4f64).sqrt() / 2.0).abs() < 1e-15);
        assert!(weighted_mean_recovery(&[RecoveryTest::Given { recovery: 0.9, u: 0.0 }]).is_err());
    }

    #[test]
    fn homogeneity_of_identical_recoveries() {
        let h = recovery_homogeneity(&[(0.98, 0.01), (0.98, 0.02), (0.98, 0.03)], 0.95).unwrap();
        assert!(h.chi_squared.abs() < 1e-20 && h.equivalent);
        assert!((h.weighted_mean - 0.98).abs() < 1e-15);
        let h = recovery_homogeneity(&[(0.90, 0.01), (1.00, 0.01)], 0.95).unwrap();
        assert!(!h.equivalent);
    }

    #[test]
    fn budget_shares_sum_to_one() {
        let b = TopDownBudget::new(
            vec![
                Component::absolute("s_I", 0.03),
                Component::relative("R", 0.03),
                Component::relative("DSS", 0.01).times(2),
            ],
            2.0,
        );
        let r = b.evaluate(0.5);
        let s: f64 = r.contributions.iter().map(|c| c.1).sum();
        assert!((s - 1.0).abs() < 1e-15);
        assert!((r.expanded_uncertainty - 2.0 * r.combined_standard_uncertainty).abs() < 1e-15);
    }

    #[test]
    fn staggered_nested_iso_5725_3_annex_j2() {
        // Vanadium in steel, level 1 without laboratory 20 (Tables J.2–J.5).
        let lv1: [[f64; 3]; 19] = [
            [0.0091, 0.0102, 0.0098],
            [0.01, 0.01, 0.009],
            [0.0095, 0.009, 0.0094],
            [0.008, 0.0083, 0.0077],
            [0.01, 0.01, 0.01],
            [0.0089, 0.0094, 0.0094],
            [0.0098, 0.0099, 0.0101],
            [0.0096, 0.0094, 0.0099],
            [0.0104, 0.0094, 0.01],
            [0.0107, 0.0118, 0.0097],
            [0.01, 0.01, 0.009],
            [0.0109, 0.0115, 0.0097],
            [0.01, 0.0095, 0.0095],
            [0.0096, 0.0096, 0.01],
            [0.0099, 0.0091, 0.0082],
            [0.0098, 0.01, 0.0095],
            [0.0105, 0.0102, 0.0112],
            [0.0103, 0.0105, 0.0118],
            [0.0098, 0.0096, 0.0104],
        ];
        let a = staggered_nested(&lv1).unwrap();
        assert!((a.mean - 0.00979825).abs() < 5e-9);
        let close = |x: f64, y: f64, t: f64| (x - y).abs() < t;
        assert!(
            close(a.ss[0], 24.16e-6, 0.005e-6)
                && close(a.ss[1], 8.29e-6, 0.005e-6)
                && close(a.ss[2], 2.76e-6, 0.005e-6)
        );
        assert!(
            close(a.ms[0], 1.342e-6, 0.0005e-6)
                && close(a.ms[1], 0.436e-6, 0.0005e-6)
                && close(a.ms[2], 0.145e-6, 0.0005e-6)
        );
        assert!(close(a.var_0, 0.278e-6, 0.0005e-6) && close(a.var_1, 0.218e-6, 0.0005e-6));
        assert!(close(a.reproducibility(), 0.801e-3, 0.0005e-3));
        assert!(close(a.intermediate(), 0.603e-3, 0.0005e-3) && close(a.repeatability(), 0.381e-3, 0.0005e-3));
        // Level 2 without laboratory 2 (Table J.5).
        let lv2: [[f64; 3]; 19] = [
            [0.0382, 0.0388, 0.0385],
            [0.039, 0.038, 0.037],
            [0.0374, 0.0361, 0.0382],
            [0.035, 0.037, 0.037],
            [0.0368, 0.0368, 0.0377],
            [0.0376, 0.038, 0.0384],
            [0.0379, 0.0366, 0.0379],
            [0.0365, 0.037, 0.0367],
            [0.037, 0.0375, 0.038],
            [0.038, 0.038, 0.0375],
            [0.039, 0.039, 0.039],
            [0.0375, 0.0375, 0.0375],
            [0.0374, 0.0374, 0.0389],
            [0.0381, 0.0375, 0.0392],
            [0.0373, 0.0377, 0.0397],
            [0.0389, 0.0382, 0.0373],
            [0.0382, 0.038, 0.0374],
            [0.0383, 0.0375, 0.0366],
            [0.037, 0.0408, 0.0369],
        ];
        let b = staggered_nested(&lv2).unwrap();
        assert!(close(b.repeatability(), 0.820e-3, 0.0005e-3));
        assert!(close(b.intermediate(), 0.902e-3, 0.0005e-3) && close(b.reproducibility(), 0.954e-3, 0.0005e-3));
    }

    /// ISO 5725-3:1994 (with Technical Corrigendum 1:2001) Table D.2: the vanadium
    /// data of all six levels, results of laboratories 1–20 at levels 2–6.
    const VANADIUM_1994: [[[f64; 3]; 5]; 20] = [
        [
            [0.0382, 0.0388, 0.0385],
            [0.101, 0.103, 0.102],
            [0.214, 0.211, 0.21],
            [0.514, 0.51, 0.513],
            [0.755, 0.753, 0.751],
        ],
        [[0.041, 0.041, 0.039], [0.111, 0.111, 0.108], [0.22, 0.22, 0.215], [0.52, 0.54, 0.54], [0.8, 0.755, 0.75]],
        [[0.039, 0.038, 0.037], [0.108, 0.11, 0.107], [0.213, 0.215, 0.215], [0.5, 0.514, 0.504], [0.738, 0.73, 0.724]],
        [
            [0.0374, 0.0361, 0.0382],
            [0.109, 0.106, 0.104],
            [0.214, 0.222, 0.201],
            [0.519, 0.518, 0.518],
            [0.744, 0.742, 0.732],
        ],
        [[0.035, 0.037, 0.037], [0.103, 0.103, 0.11], [0.21, 0.21, 0.205], [0.495, 0.5, 0.512], [0.743, 0.753, 0.75]],
        [
            [0.0368, 0.0368, 0.0377],
            [0.106, 0.106, 0.108],
            [0.232, 0.24, 0.221],
            [0.526, 0.532, 0.513],
            [0.733, 0.74, 0.746],
        ],
        [
            [0.0376, 0.038, 0.0384],
            [0.107, 0.105, 0.108],
            [0.215, 0.215, 0.216],
            [0.521, 0.519, 0.526],
            [0.754, 0.756, 0.756],
        ],
        [
            [0.0379, 0.0366, 0.0379],
            [0.108, 0.107, 0.108],
            [0.193, 0.195, 0.21],
            [0.507, 0.493, 0.511],
            [0.732, 0.729, 0.732],
        ],
        [
            [0.0365, 0.037, 0.0367],
            [0.104, 0.106, 0.105],
            [0.211, 0.205, 0.213],
            [0.509, 0.515, 0.515],
            [0.734, 0.738, 0.747],
        ],
        [[0.037, 0.0375, 0.038], [0.105, 0.11, 0.105], [0.21, 0.22, 0.225], [0.52, 0.52, 0.525], [0.76, 0.76, 0.765]],
        [
            [0.038, 0.038, 0.0375],
            [0.102, 0.102, 0.102],
            [0.213, 0.211, 0.214],
            [0.513, 0.516, 0.514],
            [0.746, 0.748, 0.746],
        ],
        [
            [0.039, 0.039, 0.039],
            [0.101, 0.108, 0.105],
            [0.208, 0.215, 0.21],
            [0.509, 0.528, 0.51],
            [0.758, 0.748, 0.75],
        ],
        [
            [0.0375, 0.0375, 0.0375],
            [0.103, 0.104, 0.108],
            [0.212, 0.222, 0.215],
            [0.51, 0.52, 0.505],
            [0.735, 0.755, 0.75],
        ],
        [
            [0.0374, 0.0374, 0.0389],
            [0.104, 0.106, 0.11],
            [0.218, 0.218, 0.212],
            [0.52, 0.528, 0.522],
            [0.74, 0.735, 0.742],
        ],
        [
            [0.0381, 0.0375, 0.0392],
            [0.109, 0.106, 0.107],
            [0.214, 0.21, 0.211],
            [0.51, 0.51, 0.515],
            [0.749, 0.729, 0.744],
        ],
        [
            [0.0373, 0.0377, 0.0397],
            [0.105, 0.105, 0.104],
            [0.215, 0.212, 0.218],
            [0.519, 0.517, 0.531],
            [0.754, 0.751, 0.759],
        ],
        [
            [0.0389, 0.0382, 0.0373],
            [0.107, 0.108, 0.104],
            [0.214, 0.21, 0.209],
            [0.517, 0.515, 0.514],
            [0.735, 0.728, 0.741],
        ],
        [
            [0.0382, 0.038, 0.0374],
            [0.103, 0.104, 0.103],
            [0.224, 0.218, 0.217],
            [0.515, 0.514, 0.517],
            [0.788, 0.798, 0.787],
        ],
        [
            [0.0383, 0.0375, 0.0366],
            [0.11, 0.109, 0.104],
            [0.217, 0.215, 0.215],
            [0.53, 0.525, 0.52],
            [0.755, 0.745, 0.74],
        ],
        [
            [0.037, 0.0408, 0.0369],
            [0.104, 0.106, 0.107],
            [0.214, 0.214, 0.203],
            [0.518, 0.518, 0.481],
            [0.73, 0.737, 0.658],
        ],
    ];

    /// Table D.5 of ISO 5725-3:1994 (= Table J.5 of ISO 5725-3:2023) at levels 3–6 from the 1994 data;
    /// Table J.2 of the 2023 edition differs from Table D.2 in 15 entries (research log F40).
    #[test]
    fn staggered_nested_iso_5725_3_1994_levels_3_to_6() {
        let close = |x: f64, y: f64| (x - y).abs() < 0.0005e-3;
        let level = |j: usize, out: &[usize]| {
            let rows: Vec<[f64; 3]> =
                (0..20).filter(|i| !out.contains(&(i + 1))).map(|i| VANADIUM_1994[i][j - 2]).collect();
            staggered_nested(&rows).unwrap()
        };
        for (j, out, sr, si, sr_) in [
            (3, &[][..], 1.739e-3, 2.305e-3, 2.650e-3),
            (4, &[6, 8][..], 3.524e-3, 4.710e-3, 4.826e-3),
            (5, &[20][..], 6.237e-3, 6.436e-3, 9.412e-3),
        ] {
            let a = level(j, out);
            assert!(
                close(a.repeatability(), sr) && close(a.intermediate(), si) && close(a.reproducibility(), sr_),
                "level {j}"
            );
        }
        // Level 6: s₍₁₎² < 0, so s_I(T) = s_r = 9.545 × 10⁻³ as printed. The printed s_R = 15.962 × 10⁻³
        // keeps the negative s₍₁₎² in s₀² and s_R²; with s₍₁₎² = 0 throughout, s_R = 16.331 × 10⁻³.
        let a = level(6, &[20]);
        assert_eq!(a.var_1, 0.0);
        assert!(close(a.repeatability(), 9.545e-3) && close(a.intermediate(), 9.545e-3));
        assert!(close(a.reproducibility(), 16.331e-3));
        let s1 = 0.75 * (a.ms[1] - a.ms[2]);
        let s0 = (a.ms[0] - a.ms[2] - 5.0 / 3.0 * s1) / 3.0;
        assert!(s1 < 0.0 && close((a.var_r + s1 + s0).sqrt(), 15.962e-3));
    }
}
