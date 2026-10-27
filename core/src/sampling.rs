//! Measurement uncertainty arising from sampling — Eurachem/EUROLAB/CITAC/
//! Nordtest/AMC Guide *Measurement uncertainty arising from sampling*,
//! 2nd edition (UfS:2019).
//!
//! * the duplicate method (Section 9.4.2): a classical nested analysis of
//!   variance of duplicate samples with duplicate analyses from several
//!   sampling targets (Equations 1–2), for balanced and unbalanced designs
//!   (Appendix D2), with negative variance estimates set to zero;
//! * the range method for the balanced design (Example A3, Table A3.7 as
//!   corrected in the errata, version 1.1);
//! * log-normally distributed values: the ANOVA of ln x, the uncertainty
//!   factor F_U = exp(k s_G) (Equation 5) and the relative standard
//!   uncertainty u′ = √(exp(s_G²) − 1) (Section 9.5.3);
//! * the fundamental sampling error of Gy's sampling theory (Section
//!   10.2.8, Equations 6–8) for the modelling approach (Example A5).
//!
//! The robust ANOVA of the programs RANOVA and RANOVA2 cited by the Guide is
//! not implemented: its algorithm is not published in the Guide, and the
//! robust results of Examples A1 and A2 could not be reproduced from the
//! published descriptions of the method (research log, finding F63).

use crate::error::UncError;
use crate::qc::range_factors;

fn invalid(msg: impl Into<String>) -> UncError {
    UncError::InvalidOption(msg.into())
}

/// The levels of the nested design, from the top.
pub const LEVELS: usize = 3;

/// Classical nested ANOVA of a duplicate design: targets, samples within
/// targets and analyses within samples. Index 0 is between-target, 1
/// sampling (between samples within a target) and 2 analysis.
#[derive(Clone, Debug, PartialEq)]
pub struct NestedAnova {
    pub targets: usize,
    pub samples: usize,
    pub observations: usize,
    /// Mean of all observations, the reference for relative uncertainties.
    pub mean: f64,
    pub ss: [f64; LEVELS],
    pub dof: [f64; LEVELS],
    pub ms: [f64; LEVELS],
    /// Coefficients of the expected mean squares: E(MS_samp) = σ_A² +
    /// k₁σ_S² and E(MS_target) = σ_A² + k₂σ_S² + k₃σ_T² (2, 2 and 4 for the
    /// balanced design).
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    /// Method-of-moments variance estimates, before negative values are set
    /// to zero.
    pub variance_raw: [f64; LEVELS],
    /// Standard deviations s_T, s_S, s_A (negative variances set to zero).
    pub sd: [f64; LEVELS],
}

impl NestedAnova {
    /// s_meas = √(s_S² + s_A²) (Equation 1).
    pub fn measurement_sd(&self) -> f64 {
        self.sd[1].hypot(self.sd[2])
    }

    /// s_total = √(s_T² + s_S² + s_A²).
    pub fn total_sd(&self) -> f64 {
        (self.sd.iter().map(|s| s * s).sum::<f64>()).sqrt()
    }

    /// Per cent of the total variance for between-target, sampling, analysis
    /// and measurement.
    pub fn percent_of_total(&self) -> [f64; 4] {
        let t = self.total_sd().powi(2);
        let p = |v: f64| if t > 0.0 { 100.0 * v / t } else { 0.0 };
        [p(self.sd[0].powi(2)), p(self.sd[1].powi(2)), p(self.sd[2].powi(2)), p(self.measurement_sd().powi(2))]
    }

    /// Whether a variance estimate was negative and has been set to zero.
    pub fn negative(&self, level: usize) -> bool {
        self.variance_raw[level] < 0.0
    }
}

/// Classical nested ANOVA. `targets[i][j]` holds the analyses of sample j
/// from target i; each sample needs at least one analysis. Some target needs
/// two samples and some sample two analyses, so that the sampling and
/// analytical variances can be estimated. For unequal numbers the
/// coefficients of the expected mean squares are those of the
/// method of moments for the unbalanced two-stage nested design.
pub fn nested_anova(targets: &[Vec<Vec<f64>>]) -> Result<NestedAnova, UncError> {
    let a = targets.len();
    if a < 2 {
        return Err(invalid("the analysis of variance needs at least two sampling targets"));
    }
    if targets.iter().any(|t| t.is_empty() || t.iter().any(|s| s.is_empty())) {
        return Err(invalid("every sampling target needs a sample and every sample an analysis"));
    }
    let all: Vec<f64> = targets.iter().flatten().flatten().cloned().collect();
    if all.iter().any(|v| !v.is_finite()) {
        return Err(invalid("measured values must be finite"));
    }
    let n = all.len() as f64;
    let b: usize = targets.iter().map(|t| t.len()).sum();
    if b == a {
        return Err(invalid("at least one sampling target needs duplicate samples"));
    }
    if all.len() == b {
        return Err(invalid("at least one sample needs duplicate analyses"));
    }
    let mean = all.iter().sum::<f64>() / n;
    let (mut ss_t, mut ss_s, mut ss_a) = (0.0, 0.0, 0.0);
    let (mut sum_nij2_over_ni, mut sum_nij2, mut sum_ni2) = (0.0, 0.0, 0.0);
    for t in targets {
        let ni = t.iter().map(|s| s.len()).sum::<usize>() as f64;
        let ti = t.iter().flatten().sum::<f64>() / ni;
        ss_t += ni * (ti - mean).powi(2);
        let mut nij2 = 0.0;
        for s in t {
            let nij = s.len() as f64;
            let sij = s.iter().sum::<f64>() / nij;
            ss_s += nij * (sij - ti).powi(2);
            ss_a += s.iter().map(|x| (x - sij).powi(2)).sum::<f64>();
            nij2 += nij * nij;
        }
        sum_nij2_over_ni += nij2 / ni;
        sum_nij2 += nij2;
        sum_ni2 += ni * ni;
    }
    let (af, bf) = (a as f64, b as f64);
    let dof = [af - 1.0, bf - af, n - bf];
    let ms = [ss_t / dof[0], ss_s / dof[1], ss_a / dof[2]];
    let k1 = (n - sum_nij2_over_ni) / dof[1];
    let k2 = (sum_nij2_over_ni - sum_nij2 / n) / dof[0];
    let k3 = (n - sum_ni2 / n) / dof[0];
    let va = ms[2];
    let vs = (ms[1] - ms[2]) / k1;
    // σ_T² from E(MS_T) with the unclamped σ_S² estimate (method of moments).
    let vt = (ms[0] - ms[2] - k2 * vs) / k3;
    let variance_raw = [vt, vs, va];
    let sd = variance_raw.map(|v| v.max(0.0).sqrt());
    Ok(NestedAnova {
        targets: a,
        samples: b,
        observations: all.len(),
        mean,
        ss: [ss_t, ss_s, ss_a],
        dof,
        ms,
        k1,
        k2,
        k3,
        variance_raw,
        sd,
    })
}

/// The balanced duplicate design S1A1, S1A2, S2A1, S2A2 for each target.
pub fn balanced(rows: &[[f64; 4]]) -> Vec<Vec<Vec<f64>>> {
    rows.iter().map(|r| vec![vec![r[0], r[1]], vec![r[2], r[3]]]).collect()
}

/// The range method for the balanced duplicate design (Example A3).
#[derive(Clone, Debug, PartialEq)]
pub struct RangeEstimate {
    pub targets: usize,
    /// Mean of the target averages.
    pub mean: f64,
    /// d₂ = 2/√π for ranges of two values (ISO 8258; the Guide uses 1.128).
    pub d2: f64,
    /// Mean of the analytical ranges |S1A1 − S1A2| and |S2A1 − S2A2|.
    pub mean_range_analysis: f64,
    /// Mean of the ranges between the sample means, |x̄_S1 − x̄_S2|.
    pub mean_range_samples: f64,
    /// Standard deviation of the target averages.
    pub sd_target_means: f64,
    /// s_A = R̄_A/d₂.
    pub s_analysis: f64,
    /// s_S+A = R̄_S/d₂, the standard deviation of a sample mean of two
    /// analyses.
    pub s_samples: f64,
    /// Variances before negative values are set to zero: s_T², s_S², s_A².
    pub variance_raw: [f64; LEVELS],
    pub sd: [f64; LEVELS],
}

impl RangeEstimate {
    pub fn measurement_sd(&self) -> f64 {
        self.sd[1].hypot(self.sd[2])
    }
}

/// Range calculations of Table A3.7: s_A = R̄_A/d₂; s_S = √(s_S+A² − s_A²/2)
/// with s_S+A = R̄_S/d₂; s_T = √(s_T+S+A² − s_S+A²/2) with s_T+S+A the
/// standard deviation of the target averages.
pub fn duplicate_range(rows: &[[f64; 4]]) -> Result<RangeEstimate, UncError> {
    let n = rows.len();
    if n < 2 {
        return Err(invalid("the range method needs at least two sampling targets"));
    }
    if rows.iter().flatten().any(|v| !v.is_finite()) {
        return Err(invalid("measured values must be finite"));
    }
    let nf = n as f64;
    let d2 = range_factors(2)?.d2;
    let ra = rows.iter().map(|r| ((r[0] - r[1]).abs() + (r[2] - r[3]).abs()) / 2.0).sum::<f64>() / nf;
    let rs = rows.iter().map(|r| ((r[0] + r[1]) / 2.0 - (r[2] + r[3]) / 2.0).abs()).sum::<f64>() / nf;
    let means: Vec<f64> = rows.iter().map(|r| r.iter().sum::<f64>() / 4.0).collect();
    let mean = means.iter().sum::<f64>() / nf;
    let st = (means.iter().map(|m| (m - mean).powi(2)).sum::<f64>() / (nf - 1.0)).sqrt();
    let s_a = ra / d2;
    let s_sa = rs / d2;
    let variance_raw = [st * st - s_sa * s_sa / 2.0, s_sa * s_sa - s_a * s_a / 2.0, s_a * s_a];
    Ok(RangeEstimate {
        targets: n,
        mean,
        d2,
        mean_range_analysis: ra,
        mean_range_samples: rs,
        sd_target_means: st,
        s_analysis: s_a,
        s_samples: s_sa,
        variance_raw,
        sd: variance_raw.map(|v| v.max(0.0).sqrt()),
    })
}

/// Natural logarithms of positive measured values (Section 9.5.3).
pub fn ln_values(targets: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<Vec<f64>>>, UncError> {
    if targets.iter().flatten().flatten().any(|v| !(*v > 0.0)) {
        return Err(invalid("the log-transformation needs positive measured values"));
    }
    Ok(targets.iter().map(|t| t.iter().map(|s| s.iter().map(|v| v.ln()).collect()).collect()).collect())
}

/// Expanded uncertainty factor F_U = exp(k s_G) for the standard deviation
/// s_G of ln x (Equation 5 with k = 2); the interval is x/F_U to x·F_U.
pub fn uncertainty_factor(s_g: f64, k: f64) -> f64 {
    (k * s_g).exp()
}

/// Relative standard uncertainty u′ = √(exp(s_G²) − 1) of a log-normal
/// variable with standard deviation s_G of ln x (Section 9.5.3).
pub fn relative_from_log(s_g: f64) -> f64 {
    (s_g * s_g).exp_m1().sqrt()
}

/// Material properties for the constitution factor (Equation 8).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GyMaterial {
    /// a_L, the mean concentration of the analyte in the lot.
    pub lot_concentration: f64,
    /// α, the concentration of the analyte in the critical particles (same
    /// unit as a_L).
    pub particle_concentration: f64,
    /// ρ_c, density of the critical particles.
    pub critical_density: f64,
    /// ρ_m, density of the matrix particles.
    pub matrix_density: f64,
}

/// Constitution factor c = (1 − a_L/α)²/(a_L/α)·ρ_c + (1 − a_L/α)·ρ_m.
pub fn constitution_factor(m: &GyMaterial) -> Result<f64, UncError> {
    let r = m.lot_concentration / m.particle_concentration;
    if !(r > 0.0 && r <= 1.0) || !(m.critical_density > 0.0) || !(m.matrix_density > 0.0) {
        return Err(invalid("the constitution factor needs 0 < a_L ≤ α and positive densities"));
    }
    Ok((1.0 - r).powi(2) / r * m.critical_density + (1.0 - r) * m.matrix_density)
}

/// One sampling or sample-splitting step.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GyStep {
    /// M_S, mass of the sample taken in this step.
    pub sample_mass: f64,
    /// M_L, mass of the lot (or the sample) it is taken from.
    pub lot_mass: f64,
    /// d, the characteristic particle size (95 % upper limit), in units
    /// consistent with the densities and masses (cm with g cm⁻³ and g).
    pub particle_size: f64,
    /// g, size-distribution factor (0.25 wide, 1 uniform sizes).
    pub size_factor: f64,
    /// f, shape factor (0.5 default for spheroidal particles).
    pub shape_factor: f64,
    /// β, liberation factor (1 for liberated particles).
    pub liberation: f64,
}

/// Sampling constant C = f g β c (Equation 7).
pub fn sampling_constant(c: f64, step: &GyStep) -> f64 {
    step.shape_factor * step.size_factor * step.liberation * c
}

/// Relative standard deviation of the fundamental sampling error,
/// s_r = √(C d³ (1/M_S − 1/M_L)) (Equation 6).
pub fn fundamental_error(c: f64, step: &GyStep) -> Result<f64, UncError> {
    let s = step;
    let positive = [s.sample_mass, s.particle_size, s.size_factor, s.shape_factor, s.liberation];
    if positive.iter().any(|v| !(*v > 0.0)) || !(s.lot_mass >= s.sample_mass) {
        return Err(invalid("a sampling step needs positive factors and a lot at least as large as the sample"));
    }
    let v = sampling_constant(c, s) * s.particle_size.powi(3) * (1.0 / s.sample_mass - 1.0 / s.lot_mass);
    Ok(v.max(0.0).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    // Example A1, Table A1.1: nitrate in lettuce (mg kg⁻¹).
    const A1: [[f64; 4]; 8] = [
        [3898.0, 4139.0, 4466.0, 4693.0],
        [3910.0, 3993.0, 4201.0, 4126.0],
        [5708.0, 5903.0, 4061.0, 3782.0],
        [5028.0, 4754.0, 5450.0, 5416.0],
        [4640.0, 4401.0, 4248.0, 4191.0],
        [5182.0, 5023.0, 4662.0, 4839.0],
        [3028.0, 3224.0, 3023.0, 2901.0],
        [3966.0, 4283.0, 4131.0, 3788.0],
    ];

    // Example A2, Table A2.2: lead in top soil (mg kg⁻¹).
    const A2: [[f64; 4]; 10] = [
        [787.0, 769.0, 811.0, 780.0],
        [338.0, 327.0, 651.0, 563.0],
        [289.0, 297.0, 211.0, 204.0],
        [662.0, 702.0, 238.0, 246.0],
        [229.0, 215.0, 208.0, 218.0],
        [346.0, 374.0, 525.0, 520.0],
        [324.0, 321.0, 77.0, 73.0],
        [56.0, 61.0, 116.0, 120.0],
        [189.0, 189.0, 176.0, 168.0],
        [61.0, 61.0, 91.0, 119.0],
    ];

    // Example A3, Table A3.7: dissolved iron in groundwater (mg l⁻¹).
    const A3: [[f64; 4]; 6] = [
        [0.815, 0.834, 0.912, 0.893],
        [1.8, 1.83, 1.94, 1.93],
        [1.69, 1.68, 1.79, 1.77],
        [2.62, 2.61, 2.83, 2.84],
        [1.66, 1.63, 1.58, 1.59],
        [1.52, 1.53, 1.47, 1.50],
    ];

    #[test]
    fn example_a1_classical() {
        let r = nested_anova(&balanced(&A1)).unwrap();
        close(r.mean, 4345.5625, 1e-9);
        // Figure A1.2: SS 12577113 / 4471511 / 351320.
        close(r.ss[0], 12577112.875, 1e-4);
        close(r.ss[1], 4471511.0, 1e-4);
        close(r.ss[2], 351320.0, 1e-6);
        assert_eq!(r.dof, [7.0, 8.0, 16.0]);
        close(r.k1, 2.0, 1e-12);
        close(r.k2, 2.0, 1e-12);
        close(r.k3, 4.0, 1e-12);
        close(r.sd[0], 556.2804, 1e-4);
        close(r.sd[1], 518.16089, 1e-4);
        close(r.sd[2], 148.18063, 1e-5);
        close(r.total_sd(), 774.5296, 1e-4);
        let p = r.percent_of_total();
        close(p[0], 51.58, 0.005);
        close(p[1], 44.76, 0.005);
        close(p[2], 3.66, 0.005);
    }

    #[test]
    fn example_a2_classical_and_log() {
        let r = nested_anova(&balanced(&A2)).unwrap();
        close(r.mean, 317.8, 1e-9);
        close(r.sd[0], 197.55, 0.005);
        close(r.sd[1], 135.43, 0.005);
        close(r.sd[2], 17.99, 0.005);
        close(r.measurement_sd(), 136.62, 0.005);
        close(r.total_sd(), 240.19, 0.005);
        // Expanded relative uncertainties (k = 2): 85.23, 11.32, 85.98 %.
        close(200.0 * r.sd[1] / r.mean, 85.23, 0.005);
        close(200.0 * r.sd[2] / r.mean, 11.32, 0.005);
        close(200.0 * r.measurement_sd() / r.mean, 85.98, 0.005);
        // Table A2.3: ANOVA of ln x.
        let g = nested_anova(&ln_values(&balanced(&A2)).unwrap()).unwrap();
        close(g.mean, 5.478, 5e-4);
        close(g.total_sd(), 0.82337, 5e-6);
        close(g.sd[0], 0.66775, 5e-6);
        close(g.sd[1], 0.4784, 5e-5);
        close(g.sd[2], 0.0567, 5e-5);
        close(g.measurement_sd(), 0.4817, 5e-5);
        // Uncertainty factors 2.6032, 1.12, 2.6207 and u′ = 0.5111.
        close(uncertainty_factor(g.sd[1], 2.0), 2.6032, 5e-5);
        close(uncertainty_factor(g.sd[2], 2.0), 1.12, 5e-3);
        close(uncertainty_factor(g.measurement_sd(), 2.0), 2.6207, 5e-5);
        close(relative_from_log(g.measurement_sd()), 0.5111, 5e-5);
        assert!(ln_values(&[vec![vec![1.0, 0.0]]]).is_err());
    }

    #[test]
    fn example_a3_range_and_anova() {
        // Errata 1.1, Table A3.7: R̄_A 0.017, s_A 0.015 (CV 0.89 %),
        // s_S+A 0.091, s_S 0.090 (5.23 %), s_T+S+A 0.604, s_T 0.601 (35 %).
        let r = duplicate_range(&A3).unwrap();
        close(r.mean, 1.719, 5e-4);
        close(r.mean_range_analysis, 0.017, 5e-4);
        close(r.s_analysis, 0.015, 5e-4);
        close(100.0 * r.sd[2] / r.mean, 0.89, 5e-3);
        close(r.s_samples, 0.091, 5e-4);
        close(r.sd[1], 0.090, 5e-4);
        close(100.0 * r.sd[1] / r.mean, 5.23, 5e-3);
        close(r.sd_target_means, 0.604, 5e-4);
        close(r.sd[0], 0.601, 5e-4);
        close(100.0 * r.sd[0] / r.mean, 35.0, 0.5);
        // Table A3.4 (errata) and summary: U 1.8 %, 10.5 %, 70 %; 11 %.
        close(200.0 * r.sd[2] / r.mean, 1.8, 0.05);
        close(200.0 * r.sd[1] / r.mean, 10.5, 0.05);
        close(200.0 * r.sd[0] / r.mean, 70.0, 0.5);
        close(200.0 * r.measurement_sd() / r.mean, 11.0, 0.5);
        // Table A3.5, ANOVA: 1.6 %, 9.6 %, 70 %.
        let a = nested_anova(&balanced(&A3)).unwrap();
        close(200.0 * a.sd[2] / a.mean, 1.6, 0.05);
        close(200.0 * a.sd[1] / a.mean, 9.6, 0.05);
        close(200.0 * a.sd[0] / a.mean, 70.0, 0.5);
    }

    #[test]
    fn example_a4_negative_variance() {
        // Tables A4.3/A4.5/A4.6: 40 g test samples.
        let a40: [[f64; 4]; 10] = [
            [402.0, 325.0, 361.0, 351.0],
            [382.0, 319.0, 349.0, 362.0],
            [332.0, 291.0, 397.0, 348.0],
            [280.0, 278.0, 358.0, 321.0],
            [370.0, 409.0, 378.0, 460.0],
            [344.0, 318.0, 381.0, 392.0],
            [297.0, 333.0, 341.0, 315.0],
            [336.0, 320.0, 292.0, 306.0],
            [372.0, 353.0, 332.0, 337.0],
            [407.0, 361.0, 322.0, 382.0],
        ];
        let r = nested_anova(&balanced(&a40)).unwrap();
        close(r.mean, 347.85, 1e-9);
        close(r.ss[2], 16595.0, 1e-6);
        close(r.ss[1], 14231.0, 1e-6);
        close(r.variance_raw[1], 296.675, 1e-9);
        close(r.sd[2], 28.80538, 1e-5);
        close(100.0 * r.sd[2] / r.mean, 8.280978, 1e-6);
        close(100.0 * r.sd[1] / r.mean, 4.95, 5e-3);
        // Errata 1.4a: between-batch RSD 6.1 %.
        close(100.0 * r.sd[0] / r.mean, 6.1, 0.05);
        // Tables A4.4/A4.8/A4.9: 4 g test samples, V_samp = −2662.15 → 0.
        let a4: [[f64; 4]; 10] = [
            [400.0, 491.0, 323.0, 355.0],
            [413.0, 159.0, 392.0, 434.0],
            [315.0, 391.0, 252.0, 454.0],
            [223.0, 220.0, 357.0, 469.0],
            [462.0, 343.0, 262.0, 293.0],
            [353.0, 265.0, 305.0, 456.0],
            [298.0, 234.0, 152.0, 323.0],
            [425.0, 263.0, 417.0, 353.0],
            [622.0, 189.0, 291.0, 272.0],
            [292.0, 397.0, 142.0, 568.0],
        ];
        let r = nested_anova(&balanced(&a4)).unwrap();
        close(r.ss[2], 312206.5, 1e-6);
        close(r.ss[1], 102860.25, 1e-6);
        close(r.variance_raw[1], -2662.15, 1e-9);
        assert!(r.negative(1) && r.sd[1] == 0.0);
        close(100.0 * r.sd[2] / r.mean, 36.68, 5e-3);
    }

    #[test]
    fn unbalanced_design() {
        // Removing analyses leaves the moments consistent: a design where
        // every S2 has one analysis (Appendix D2) and a check against the
        // one-way ANOVA when every target has one sample except the first.
        let rows: Vec<Vec<Vec<f64>>> = A1.iter().map(|r| vec![vec![r[0], r[1]], vec![r[2]]]).collect();
        let r = nested_anova(&rows).unwrap();
        assert_eq!(r.dof, [7.0, 8.0, 8.0]);
        // n_ij = (2, 1): k₁ = (24 − 8·5/3)/8, k₂ = (8·5/3 − 40/24)/7,
        // k₃ = (24 − 72/24)/7.
        close(r.k1, (24.0 - 40.0 / 3.0) / 8.0, 1e-12);
        close(r.k2, (40.0 / 3.0 - 40.0 / 24.0) / 7.0, 1e-12);
        close(r.k3, 3.0, 1e-12);
        let ms_a = A1.iter().map(|r| (r[0] - r[1]).powi(2) / 2.0).sum::<f64>() / 8.0;
        close(r.ms[2], ms_a, 1e-9);
        assert!(nested_anova(&[vec![vec![1.0, 2.0]], vec![vec![3.0, 4.0]]]).is_err());
        assert!(nested_anova(&[vec![vec![1.0], vec![2.0]], vec![vec![3.0], vec![4.0]]]).is_err());
    }

    #[test]
    fn example_a5_fundamental_sampling_error() {
        let m = GyMaterial {
            lot_concentration: 0.05,
            particle_concentration: 100.0,
            critical_density: 1.08,
            matrix_density: 0.67,
        };
        let c = constitution_factor(&m).unwrap();
        close(c, 2160.0, 2.0);
        let s1 = GyStep {
            sample_mass: 500.0,
            lot_mass: 25000.0,
            particle_size: 0.1,
            size_factor: 0.5,
            shape_factor: 0.5,
            liberation: 1.0,
        };
        let s2 = GyStep { sample_mass: 2.0, lot_mass: 500.0, particle_size: 0.05, size_factor: 0.25, ..s1 };
        close(sampling_constant(c, &s1), 540.0, 0.5);
        close(sampling_constant(c, &s2), 270.0, 0.3);
        let r1 = fundamental_error(c, &s1).unwrap();
        let r2 = fundamental_error(c, &s2).unwrap();
        close(r1, 0.033, 5e-4);
        close(r2, 0.13, 5e-3);
        // s_t = 0.143 with the analytical 5 %; U = 28.6 % from the rounded
        // components, 28.5 % unrounded.
        let st = (r1 * r1 + r2 * r2 + 0.05f64.powi(2)).sqrt();
        close(st, 0.143, 5e-4);
        close(200.0 * st, 28.53, 0.01);
        assert!(fundamental_error(c, &GyStep { lot_mass: 1.0, ..s1 }).is_err());
    }
}
