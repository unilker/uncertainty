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
        }
        Ok(())
    }

    /// Recovery R_i = c̄/C, or (c̄ − c̄₀)/c₊ for a spiked sample.
    pub fn recovery(&self) -> f64 {
        match *self {
            RecoveryTest::Reference { mean, reference, .. } => mean / reference,
            RecoveryTest::Spiked { mean_after, mean_before, spike, .. } => (mean_after - mean_before) / spike,
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
            RecoveryTest::Spiked { .. } => {
                return Err(UncError::Unsupported(
                    "MUPR Eq. (17)–(18) are stated for reference materials and spiked blanks".into(),
                ))
            }
        }
    }
    let inner = if tests.len() == 1 { mse + precision + ref_rel } else { mse / n + ref_rel / n };
    Ok(mr.mean.abs() * inner.sqrt())
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
}
