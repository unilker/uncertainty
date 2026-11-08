//! Consensus values from interlaboratory studies and key comparisons, after
//! the NIST Consensus Builder (NICOB user manual, 2020, Appendix A) and
//! Koepke, Lafarge, Possolo & Toman, *Metrologia* 54 (2017) S34.
//!
//! * DerSimonian–Laird random-effects estimate x_j = μ + λ_j + ε_j with the
//!   method-of-moments τ̂ (NICOB §A.2), the naive u_DL(μ) = 1/√Σw_j, the
//!   Knapp–Hartung adjustment, and the NICOB parametric bootstrap that
//!   draws τ from the Biggerstaff–Tweedie (1997) gamma approximation of the
//!   distribution of Cochran's Q and lets u_j vary as u_j √(ν_j/χ²_ν_j);
//! * the Linear Pool (Stone 1961; NICOB §A.4): the mixture of Gaussian or
//!   rescaled Student t distributions with means x_j and standard
//!   deviations u_j;
//! * unilateral degrees of equivalence D_j = x_j − μ̂ with U95 %(D_j) as the
//!   half-length of the shortest interval centred at the mean of the
//!   simulated D_j,k containing 95 % of them (NICOB §A.5.1, §A.5.3).

use crate::error::UncError;
use crate::mcm::{coverage_intervals, half_width_about};
use crate::rng::Rng;
use crate::special::{chi_squared_quantile, gamma_q, student_t_quantile};

fn invalid(msg: &str) -> UncError {
    UncError::InvalidOption(format!("consensus: {msg}"))
}

/// Checks the measured values, standard uncertainties and (optional)
/// degrees of freedom.
fn validate(x: &[f64], u: &[f64], dof: &[f64]) -> Result<(), UncError> {
    let n = x.len();
    if n < 2 {
        return Err(invalid("give at least two measured values"));
    }
    if u.len() != n || (!dof.is_empty() && dof.len() != n) {
        return Err(invalid("give one standard uncertainty (and degrees of freedom) per measured value"));
    }
    if x.iter().any(|v| !v.is_finite()) {
        return Err(invalid("every measured value must be finite"));
    }
    if u.iter().any(|v| !(*v > 0.0) || !v.is_finite()) {
        return Err(invalid("every standard uncertainty must be positive and finite"));
    }
    if dof.iter().any(|v| !(*v > 0.0)) {
        return Err(invalid("degrees of freedom must be positive (∞ for none)"));
    }
    Ok(())
}

/// The DerSimonian–Laird estimate.
#[derive(Clone, Debug, PartialEq)]
pub struct DerSimonianLaird {
    /// Consensus value μ̂ = Σw_j x_j / Σw_j, w_j = 1/(τ̂² + u_j²).
    pub mean: f64,
    /// Naive standard uncertainty u_DL(μ) = 1/√Σw_j (Higgins et al. 2009).
    pub u: f64,
    /// τ̂ = √max(0, (Q − n + 1)/(Σu⁻² − Σu⁻⁴/Σu⁻²)).
    pub tau: f64,
    /// Cochran's Q about the weighted mean with weights u_j⁻².
    pub q: f64,
    /// Probability that χ²_{n−1} ≥ Q (test of homogeneity).
    pub q_p_value: f64,
    /// Knapp–Hartung (2003) standard uncertainty u_DL √(Σw_j(x_j − μ̂)²/(n − 1)).
    pub u_knapp_hartung: f64,
    /// Normalised weights w_j/Σw.
    pub weights: Vec<f64>,
}

/// DerSimonian–Laird estimate for measured values x with standard
/// uncertainties u (NICOB §A.2).
pub fn dersimonian_laird(x: &[f64], u: &[f64]) -> Result<DerSimonianLaird, UncError> {
    validate(x, u, &[])?;
    let n = x.len();
    let (q, tau2) = q_and_tau2(x, u);
    let w: Vec<f64> = u.iter().map(|ui| 1.0 / (tau2 + ui * ui)).collect();
    let sw: f64 = w.iter().sum();
    let mean = w.iter().zip(x).map(|(a, b)| a * b).sum::<f64>() / sw;
    let um = 1.0 / sw.sqrt();
    let s_kh = w.iter().zip(x).map(|(a, b)| a * (b - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    Ok(DerSimonianLaird {
        mean,
        u: um,
        tau: tau2.sqrt(),
        q,
        q_p_value: gamma_q(0.5 * (n - 1) as f64, 0.5 * q),
        u_knapp_hartung: um * s_kh.sqrt(),
        weights: w.iter().map(|a| a / sw).collect(),
    })
}

/// Cochran's Q and the DerSimonian–Laird τ̂².
fn q_and_tau2(x: &[f64], u: &[f64]) -> (f64, f64) {
    let n = x.len();
    let (mut s1, mut s2, mut sx) = (0.0, 0.0, 0.0);
    for (xi, ui) in x.iter().zip(u) {
        let w = 1.0 / (ui * ui);
        s1 += w;
        s2 += w * w;
        sx += w * xi;
    }
    let xw = sx / s1;
    let q: f64 = x.iter().zip(u).map(|(xi, ui)| ((xi - xw) / ui).powi(2)).sum();
    (q, ((q - (n - 1) as f64) / (s1 - s2 / s1)).max(0.0))
}

fn dl_mean(x: &[f64], u: &[f64]) -> f64 {
    let (_, tau2) = q_and_tau2(x, u);
    let (mut sw, mut swx) = (0.0, 0.0);
    for (xi, ui) in x.iter().zip(u) {
        let w = 1.0 / (tau2 + ui * ui);
        sw += w;
        swx += w * xi;
    }
    swx / sw
}

/// Shape and scale of the Biggerstaff–Tweedie (1997) gamma approximation of
/// the distribution of Cochran's Q for the between-laboratory variance τ²,
/// from the exact mean and variance of Q with known u_j.
pub fn q_gamma_approximation(u: &[f64], tau2: f64) -> (f64, f64) {
    let n = u.len() as f64;
    let (mut s1, mut s2, mut s3) = (0.0, 0.0, 0.0);
    for ui in u {
        let w = 1.0 / (ui * ui);
        s1 += w;
        s2 += w * w;
        s3 += w * w * w;
    }
    let mean = (n - 1.0) + tau2 * (s1 - s2 / s1);
    let var = 2.0 * (n - 1.0)
        + 4.0 * tau2 * (s1 - 2.0 * s2 / s1 + s3 / (s1 * s1))
        + 2.0 * tau2 * tau2 * (s2 - 2.0 * s3 / s1 + s2 * s2 / (s1 * s1));
    (mean * mean / var, var / mean)
}

/// Shape and scale of the gamma approximation of Q as the NIST Consensus
/// Builder computes them (sampleFromTau2Dist.R, Koepke & Possolo 2016): at
/// τ²_M, which may be negative (then E Q equals the observed Q), and with
/// Var Q = 2(n − 1) + 4(S1 − S2/S1)τ² + 2(S2 − 2S3/S1 + S2²/S1²)τ⁴, whose
/// linear term differs from Biggerstaff & Tweedie (1997) eq. (7) (see
/// [`q_gamma_approximation`]). With τ² > 0 the two differ by about 0.1 % in u
/// for Koepke's G; with Q < n − 1 the choice of τ²_M instead of 0 lowers u by
/// about 13 % in a four-result example (research log D82). None when the mean
/// or variance is not positive (NICOB would fail there); τ_k is then 0.
pub fn q_gamma_approximation_nicob(u: &[f64], tau2_m: f64) -> Option<(f64, f64)> {
    let n = u.len() as f64;
    let (mut s1, mut s2, mut s3) = (0.0, 0.0, 0.0);
    for ui in u {
        let w = 1.0 / (ui * ui);
        s1 += w;
        s2 += w * w;
        s3 += w * w * w;
    }
    let c = s1 - s2 / s1;
    let mean = (n - 1.0) + c * tau2_m;
    let var = 2.0 * (n - 1.0) + 4.0 * c * tau2_m + 2.0 * (s2 - 2.0 * s3 / s1 + s2 * s2 / (s1 * s1)) * tau2_m * tau2_m;
    (mean > 0.0 && var > 0.0 && c > 0.0).then(|| (mean * mean / var, var / mean))
}

/// A unilateral degree of equivalence (D_j, U95 %(D_j)).
#[derive(Clone, Debug, PartialEq)]
pub struct DegreeOfEquivalence {
    pub d: f64,
    pub expanded: f64,
}

/// Result of a Monte Carlo evaluation (bootstrap or Linear Pool).
#[derive(Clone, Debug, PartialEq)]
pub struct ConsensusSample {
    /// Mean of the simulated consensus values (the consensus value of the
    /// Linear Pool; for the bootstrap the DerSimonian–Laird μ̂ is reported).
    pub mean: f64,
    /// Standard deviation of the simulated values.
    pub u: f64,
    /// Probabilistically symmetric coverage interval (percentiles).
    pub interval: (f64, f64),
    pub coverage: f64,
    pub trials: usize,
    pub degrees_of_equivalence: Vec<DegreeOfEquivalence>,
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let m = v.iter().sum::<f64>() / v.len() as f64;
    let s = (v.iter().map(|a| (a - m).powi(2)).sum::<f64>() / (v.len() - 1) as f64).sqrt();
    (m, s)
}

fn check_trials(trials: usize, p: f64) -> Result<(), UncError> {
    if trials < 1000 {
        return Err(invalid("use at least 1000 Monte Carlo trials"));
    }
    if !(p > 0.0 && p < 1.0) {
        return Err(invalid("the coverage probability must lie strictly between 0 and 1"));
    }
    Ok(())
}

/// The NICOB parametric bootstrap for the DerSimonian–Laird procedure
/// (§A.2 steps (a)–(d), §A.5.1 step (e)). `dof` is empty or holds ν_j
/// (∞ when unspecified).
pub fn dl_bootstrap(
    x: &[f64],
    u: &[f64],
    dof: &[f64],
    p: f64,
    trials: usize,
    seed: u64,
) -> Result<ConsensusSample, UncError> {
    dl_bootstrap_samples(x, u, dof, p, trials, seed).map(|(s, _)| s)
}

/// The draws of τ² from the NICOB approximation (sampleFromTau2Dist.R) for
/// the results x, u: the gamma at the untruncated τ²_M, then max(0, ·).
fn tau2_sampler(x: &[f64], u: &[f64]) -> impl Fn(&mut Rng) -> f64 {
    let n = x.len();
    let (q, _) = q_and_tau2(x, u);
    let (mut s1, mut s2) = (0.0, 0.0);
    for ui in u {
        let w = 1.0 / (ui * ui);
        s1 += w;
        s2 += w * w;
    }
    let c = s1 - s2 / s1;
    let gamma = if c > 0.0 { q_gamma_approximation_nicob(u, (q - (n - 1) as f64) / c) } else { None };
    move |rng: &mut Rng| match gamma {
        Some((shape, scale)) => ((scale * rng.gamma(shape) - (n - 1) as f64) / c).max(0.0),
        None => 0.0,
    }
}

/// The bootstrap and its columns D_j,k = x_j,k − μ_k (for the bilateral DoE).
fn dl_bootstrap_samples(
    x: &[f64],
    u: &[f64],
    dof: &[f64],
    p: f64,
    trials: usize,
    seed: u64,
) -> Result<(ConsensusSample, Vec<Vec<f64>>), UncError> {
    validate(x, u, dof)?;
    check_trials(trials, p)?;
    let n = x.len();
    let dl = dersimonian_laird(x, u)?;
    let (mut s1, mut s2) = (0.0, 0.0);
    for ui in u {
        let w = 1.0 / (ui * ui);
        s1 += w;
        s2 += w * w;
    }
    // As in NICOB (sampleFromTau2Dist.R): the gamma is set at the untruncated
    // τ²_M = (Q − (n − 1))/(S1 − S2/S1), negative when Q < n − 1.
    let tau2_m = (dl.q - (n - 1) as f64) / (s1 - s2 / s1);
    let gamma = q_gamma_approximation_nicob(u, tau2_m);
    let mut rng = Rng::new(seed);
    let mut mus = Vec::with_capacity(trials);
    let mut ds: Vec<Vec<f64>> = vec![Vec::with_capacity(trials); n];
    let mut xk = vec![0.0; n];
    let mut uk = vec![0.0; n];
    for _ in 0..trials {
        // (a) τ_k from the approximate distribution of τ̂_DL (τ_k = 0 when
        // the moments give no valid gamma, see q_gamma_approximation_nicob).
        let tau2 = match gamma {
            Some((shape, scale)) => ((scale * rng.gamma(shape) - (n - 1) as f64) / (s1 - s2 / s1)).max(0.0),
            None => 0.0,
        };
        for j in 0..n {
            // (b) x_jk ~ N(μ̂, τ_k² + u_j²); (c) u_jk.
            xk[j] = dl.mean + rng.normal() * (tau2 + u[j] * u[j]).sqrt();
            uk[j] = match dof.get(j) {
                Some(&nu) if nu.is_finite() => u[j] * (nu / (2.0 * rng.gamma(0.5 * nu))).sqrt(),
                _ => u[j],
            };
        }
        // (d) the DerSimonian–Laird μ_k; (e) D_jk = x_jk − μ_k.
        let mk = dl_mean(&xk, &uk);
        mus.push(mk);
        for j in 0..n {
            ds[j].push(xk[j] - mk);
        }
    }
    let (_, sd) = mean_sd(&mus);
    mus.sort_by(f64::total_cmp);
    let (sym, _) = coverage_intervals(&mus, p);
    let degrees_of_equivalence = (0..n)
        .map(|j| {
            let (c, _) = mean_sd(&ds[j]);
            DegreeOfEquivalence { d: x[j] - dl.mean, expanded: half_width_about(&ds[j], c, p) }
        })
        .collect();
    Ok((ConsensusSample { mean: dl.mean, u: sd, interval: sym, coverage: p, trials, degrees_of_equivalence }, ds))
}

/// A bilateral degree of equivalence B_ij = D_i − D_j with U95 %(B_ij),
/// for i < j.
#[derive(Clone, Debug, PartialEq)]
pub struct BilateralDoE {
    pub i: usize,
    pub j: usize,
    pub b: f64,
    pub expanded: f64,
}

/// Unilateral and bilateral degrees of equivalence of the DerSimonian–Laird
/// procedure.
#[derive(Clone, Debug, PartialEq)]
pub struct DegreesOfEquivalence {
    /// True for the leave-one-out version D*_j = x_j − μ̂_−j.
    pub leave_one_out: bool,
    pub unilateral: Vec<DegreeOfEquivalence>,
    pub bilateral: Vec<BilateralDoE>,
}

/// NICOB's factor that turns a Student t draw into one with unit standard
/// deviation; for ν ≤ 2, where the variance is infinite, an approximation
/// coded in DoEUnilateralDerSimonianLaird.R (a = 0.4668994, b = −0.3998882).
fn t_scale(nu: f64) -> f64 {
    if nu <= 2.0 {
        let (a, b) = (0.466_899_4, -0.399_888_2);
        1.5 * (1.0 - 0.75 * (a - 4.0 * b * (nu.powf(-0.75) - 1.0) / 3.0)).powf(-4.0 / 3.0)
    } else {
        (nu / (nu - 2.0)).sqrt()
    }
}

/// Degrees of equivalence of the DerSimonian–Laird procedure as the NIST
/// Consensus Builder computes them (manual §3.2.8 and §A.5.1; source
/// DoEUnilateralDerSimonianLaird.R and DoEBilateralDerSimonianLaird.R,
/// Koepke & Possolo 2020):
///
/// * MRA version: D_j = x_j − μ̂ from the parametric bootstrap of
///   [`dl_bootstrap`] (the same draws for the same seed);
/// * leave-one-out version: D*_j = x_j − μ̂_−j, with μ̂_−j the
///   DerSimonian–Laird value of the other results. Its draws are
///   x_j + e_j,k − μ_−j,k, with μ_−j,k from the modified Knapp–Hartung t
///   approximation (Röver, Knapp & Friede 2015; q* = max(1, Q_w/(n − 2)),
///   ν = n − 2) and e_j,k from N(0, τ²_−j,k + u_j²), or a rescaled t_νj when
///   ν_j is given, τ²_−j,k drawn as in the bootstrap from the other results;
/// * bilateral: B_ij = D_i − D_j with the draws D_i,k − D_j,k.
///
/// U95 % is the half-width of the shortest interval symmetric about the
/// estimate (MRA: the mean of the draws) that holds the fraction p.
pub fn dl_degrees_of_equivalence(
    x: &[f64],
    u: &[f64],
    dof: &[f64],
    p: f64,
    trials: usize,
    seed: u64,
    leave_one_out: bool,
) -> Result<DegreesOfEquivalence, UncError> {
    let n = x.len();
    let (unilateral, ds) = if !leave_one_out {
        let (s, ds) = dl_bootstrap_samples(x, u, dof, p, trials, seed)?;
        // Centre the columns on D_j, as NICOB does before the bilateral DoE.
        let ds: Vec<Vec<f64>> = ds
            .into_iter()
            .zip(&s.degrees_of_equivalence)
            .map(|(col, d)| {
                let (m, _) = mean_sd(&col);
                col.into_iter().map(|v| v - m + d.d).collect()
            })
            .collect();
        (s.degrees_of_equivalence, ds)
    } else {
        validate(x, u, dof)?;
        check_trials(trials, p)?;
        if n < 3 {
            return Err(invalid("leave-one-out degrees of equivalence need at least three results"));
        }
        let mut rng = Rng::new(seed);
        let mut uni = Vec::with_capacity(n);
        let mut ds = Vec::with_capacity(n);
        for j in 0..n {
            let xo: Vec<f64> = (0..n).filter(|&i| i != j).map(|i| x[i]).collect();
            let uo: Vec<f64> = (0..n).filter(|&i| i != j).map(|i| u[i]).collect();
            let draw_tau2 = tau2_sampler(&xo, &uo);
            let tau2s: Vec<f64> = (0..trials).map(|_| draw_tau2(&mut rng)).collect();
            // Modified Knapp–Hartung for μ̂_−j.
            let (_, t2) = q_and_tau2(&xo, &uo);
            let w: Vec<f64> = uo.iter().map(|a| 1.0 / (a * a + t2)).collect();
            let sw: f64 = w.iter().sum();
            let mu = w.iter().zip(&xo).map(|(a, b)| a * b).sum::<f64>() / sw;
            let nj = xo.len() as f64;
            let qstar = (w.iter().zip(&xo).map(|(a, b)| a * (b - mu).powi(2)).sum::<f64>() / (nj - 1.0)).max(1.0);
            let (mu_u, mu_df) = ((qstar / sw).sqrt(), nj - 1.0);
            let mu_phi = t_scale(mu_df);
            let nu_j = dof.get(j).copied().unwrap_or(f64::INFINITY);
            let col: Vec<f64> = tau2s
                .iter()
                .map(|t2k| {
                    let mujk = mu + mu_u * rng.student_t(mu_df) / mu_phi;
                    let sj = (t2k + u[j] * u[j]).sqrt();
                    let e = if nu_j.is_finite() { sj * rng.student_t(nu_j) / t_scale(nu_j) } else { sj * rng.normal() };
                    x[j] + e - mujk
                })
                .collect();
            let d = x[j] - mu;
            uni.push(DegreeOfEquivalence { d, expanded: half_width_about(&col, d, p) });
            ds.push(col);
        }
        (uni, ds)
    };
    let mut bilateral = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in i + 1..n {
            let bk: Vec<f64> = ds[i].iter().zip(&ds[j]).map(|(a, b)| a - b).collect();
            let (m, _) = mean_sd(&bk);
            bilateral.push(BilateralDoE {
                i,
                j,
                b: unilateral[i].d - unilateral[j].d,
                expanded: half_width_about(&bk, m, p),
            });
        }
    }
    Ok(DegreesOfEquivalence { leave_one_out, unilateral, bilateral })
}

/// The Linear Pool (NICOB §A.4): a sample from the mixture Σ w_j φ_j of
/// Gaussian (ν_j = ∞ or unspecified) or rescaled Student t distributions
/// with means x_j and standard deviations u_j (for ν_j ≤ 2, where the t
/// variance is infinite, scaled by NICOB's approximation, linearOP.R). `weights` is empty for equal
/// weights; it is normalised to sum to 1.
pub fn linear_pool(
    x: &[f64],
    u: &[f64],
    dof: &[f64],
    weights: &[f64],
    p: f64,
    trials: usize,
    seed: u64,
) -> Result<ConsensusSample, UncError> {
    validate(x, u, dof)?;
    check_trials(trials, p)?;
    let n = x.len();
    let w: Vec<f64> = if weights.is_empty() { vec![1.0; n] } else { weights.to_vec() };
    if w.len() != n || w.iter().any(|a| !(*a >= 0.0) || !a.is_finite()) || w.iter().sum::<f64>() <= 0.0 {
        return Err(invalid("give one non-negative weight per measured value"));
    }
    let total: f64 = w.iter().sum();
    let mut cum = Vec::with_capacity(n);
    let mut acc = 0.0;
    for a in &w {
        acc += a / total;
        cum.push(acc);
    }
    let scale: Vec<f64> = (0..n)
        .map(|j| match dof.get(j) {
            // u_j/√(ν/(ν − 2)); for ν ≤ 2 NICOB's approximation (linearOP.R).
            Some(&nu) if nu.is_finite() => u[j] / t_scale(nu),
            _ => u[j],
        })
        .collect();
    let mut rng = Rng::new(seed);
    let draw = |rng: &mut Rng, j: usize| match dof.get(j) {
        Some(&nu) if nu.is_finite() => scale[j] * rng.student_t(nu),
        _ => scale[j] * rng.normal(),
    };
    let mut s = Vec::with_capacity(trials);
    for _ in 0..trials {
        let r = rng.uniform();
        let j = cum.iter().position(|c| r < *c).unwrap_or(n - 1);
        s.push(x[j] + draw(&mut rng, j));
    }
    let (mean, sd) = mean_sd(&s);
    s.sort_by(f64::total_cmp);
    let (sym, _) = coverage_intervals(&s, p);
    // D_j = x_j − μ̂; the D_j,k = x_j + e_j,k − μ̂ are symmetric about D_j, so
    // U95 % is the p-quantile of |e| (NICOB §A.5.3).
    let degrees_of_equivalence = (0..n)
        .map(|j| {
            let k = match dof.get(j) {
                Some(&nu) if nu.is_finite() => student_t_quantile(0.5 * (1.0 + p), nu) / t_scale(nu),
                _ => crate::special::normal_quantile(0.5 * (1.0 + p)),
            };
            DegreeOfEquivalence { d: x[j] - mean, expanded: k * u[j] }
        })
        .collect();
    Ok(ConsensusSample { mean, u: sd, interval: sym, coverage: p, trials, degrees_of_equivalence })
}

/// Expanded uncertainty of the DerSimonian–Laird μ̂ from the Knapp–Hartung
/// adjustment: t_{n−1} quantile times u_KH.
pub fn knapp_hartung_expanded(dl: &DerSimonianLaird, n: usize, p: f64) -> f64 {
    student_t_quantile(0.5 * (1.0 + p), (n - 1) as f64) * dl.u_knapp_hartung
}

/// Critical value of Cochran's Q for the homogeneity test at level α.
pub fn q_critical(n: usize, alpha: f64) -> f64 {
    chi_squared_quantile(1.0 - alpha, (n - 1) as f64)
}

/// Maximum-likelihood estimate of the Gaussian random-effects model
/// x_j ~ N(μ, τ² + u_j²) with known u_j (IUPAC 2024 Example 4F; Koepke et
/// al. 2017 Table 2).
#[derive(Clone, Debug, PartialEq)]
pub struct MaximumLikelihood {
    pub mean: f64,
    pub tau: f64,
    /// Standard uncertainties from the inverse of the Hessian of the
    /// negative log-likelihood at the estimate.
    pub u_mean: f64,
    pub u_tau: f64,
}

/// Negative log-likelihood profiled over μ (μ̂(τ) is the weighted mean with
/// weights 1/(τ² + u_j²)), without the constant term.
fn profile_nll(x: &[f64], u: &[f64], t: f64) -> (f64, f64) {
    let (mut sw, mut swx) = (0.0, 0.0);
    for (xi, ui) in x.iter().zip(u) {
        let w = 1.0 / (t * t + ui * ui);
        sw += w;
        swx += w * xi;
    }
    let m = swx / sw;
    let nll = x.iter().zip(u).map(|(xi, ui)| {
        let v = t * t + ui * ui;
        0.5 * (v.ln() + (xi - m).powi(2) / v)
    });
    (nll.sum(), m)
}

pub fn maximum_likelihood(x: &[f64], u: &[f64]) -> Result<MaximumLikelihood, UncError> {
    validate(x, u, &[])?;
    let n = x.len() as f64;
    let xbar = x.iter().sum::<f64>() / n;
    let sd = (x.iter().map(|a| (a - xbar).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
    let umax = u.iter().copied().fold(0.0, f64::max);
    // Grid over [0, T], then golden-section refinement of the best bracket.
    let top = 10.0 * sd.max(umax);
    let m = 4000;
    let f = |t: f64| profile_nll(x, u, t).0;
    let mut best = 0;
    let mut fbest = f(0.0);
    for i in 1..=m {
        let v = f(top * i as f64 / m as f64);
        if v < fbest {
            fbest = v;
            best = i;
        }
    }
    let h = top / m as f64;
    let (mut a, mut b) = ((best as f64 - 1.0).max(0.0) * h, (best as f64 + 1.0) * h);
    let g = 0.5 * (5f64.sqrt() - 1.0);
    let (mut c, mut d) = (b - g * (b - a), a + g * (b - a));
    for _ in 0..200 {
        if f(c) < f(d) {
            b = d;
        } else {
            a = c;
        }
        c = b - g * (b - a);
        d = a + g * (b - a);
    }
    let mut t = 0.5 * (a + b);
    if f(0.0) <= f(t) {
        t = 0.0;
    }
    let (_, mean) = profile_nll(x, u, t);
    // Hessian of −ln L in (μ, τ).
    let (mut hmm, mut hmt, mut htt) = (0.0, 0.0, 0.0);
    for (xi, ui) in x.iter().zip(u) {
        let v = t * t + ui * ui;
        let e = xi - mean;
        hmm += 1.0 / v;
        hmt += 2.0 * t * e / (v * v);
        htt += 1.0 / v - 2.0 * t * t / (v * v) - e * e / (v * v) + 4.0 * t * t * e * e / (v * v * v);
    }
    let det = hmm * htt - hmt * hmt;
    let (u_mean, u_tau) =
        if det > 0.0 { ((htt / det).sqrt(), (hmm / det).sqrt()) } else { ((1.0 / hmm).sqrt(), f64::NAN) };
    Ok(MaximumLikelihood { mean, tau: t, u_mean, u_tau })
}

/// Posterior of the Gaussian random-effects model x_j ~ N(μ, τ² + u_j²)
/// with known u_j, a half-Cauchy prior on τ and a Gaussian (or flat) prior
/// on μ (IUPAC 2024 Examples 4M and 4N). μ is integrated out analytically;
/// the posterior of τ is evaluated on a grid uniform in the half-Cauchy
/// probability, τ = s·tan(πθ/2), where the prior density is constant. The
/// result is the same at every run, without MCMC.
#[derive(Clone, Debug, PartialEq)]
pub struct BayesConsensus {
    pub mean: f64,
    pub sd: f64,
    /// 2.5 % and 97.5 % quantiles of the marginal posterior of μ (for the
    /// coverage probability requested).
    pub interval: (f64, f64),
    pub tau_median: f64,
    pub tau_interval: (f64, f64),
}

/// Median of |x_j − median(x)|: the NICOB default for the median of the
/// half-Cauchy prior on τ (§A.3).
pub fn median_absolute_deviation(x: &[f64]) -> f64 {
    let med = |v: &mut Vec<f64>| {
        v.sort_by(f64::total_cmp);
        let n = v.len();
        if n % 2 == 1 {
            v[n / 2]
        } else {
            0.5 * (v[n / 2 - 1] + v[n / 2])
        }
    };
    let m = med(&mut x.to_vec());
    med(&mut x.iter().map(|v| (v - m).abs()).collect())
}

/// `tau_median` is the median (= scale) of the half-Cauchy prior on τ;
/// `mu_prior` is (mean, sd) of a Gaussian prior on μ, or None for a flat one.
pub fn bayes_known_u(
    x: &[f64],
    u: &[f64],
    tau_median: f64,
    mu_prior: Option<(f64, f64)>,
    p: f64,
) -> Result<BayesConsensus, UncError> {
    bayes_known_u_grid(x, u, tau_median, mu_prior, p, 100_000)
}

/// [`bayes_known_u`] with `m` grid points in θ ∈ (0, 1) (midpoints). The
/// grid covers the whole range of τ, so no posterior mass lies outside it;
/// the convergence test doubles `m`.
pub fn bayes_known_u_grid(
    x: &[f64],
    u: &[f64],
    tau_median: f64,
    mu_prior: Option<(f64, f64)>,
    p: f64,
    m: usize,
) -> Result<BayesConsensus, UncError> {
    validate(x, u, &[])?;
    if !(tau_median > 0.0) || !tau_median.is_finite() {
        return Err(invalid("the median of the half-Cauchy prior on τ must be positive"));
    }
    if let Some((m0, s0)) = mu_prior {
        if !m0.is_finite() || !(s0 > 0.0) || !s0.is_finite() {
            return Err(invalid("the prior on μ needs a finite mean and a positive standard deviation"));
        }
    }
    if !(p > 0.0 && p < 1.0) {
        return Err(invalid("the coverage probability must lie strictly between 0 and 1"));
    }
    // Centre the data for numerical stability.
    let c = x.iter().sum::<f64>() / x.len() as f64;
    let xc: Vec<f64> = x.iter().map(|v| v - c).collect();
    let prior = mu_prior.map(|(m0, s0)| (m0 - c, s0));
    let mut taus = Vec::with_capacity(m);
    let mut ll = Vec::with_capacity(m);
    let mut mt = Vec::with_capacity(m);
    let mut prec = Vec::with_capacity(m);
    for i in 0..m {
        let th = (i as f64 + 0.5) / m as f64;
        let t = tau_median * (0.5 * std::f64::consts::PI * th).tan();
        let (mut pr, mut num, mut sx2, mut slog) = (0.0, 0.0, 0.0, 0.0);
        for (xi, ui) in xc.iter().zip(u) {
            let v = t * t + ui * ui;
            pr += 1.0 / v;
            num += xi / v;
            sx2 += xi * xi / v;
            slog += v.ln();
        }
        if let Some((m0, s0)) = prior {
            pr += 1.0 / (s0 * s0);
            num += m0 / (s0 * s0);
            sx2 += m0 * m0 / (s0 * s0);
        }
        let mu_t = num / pr;
        taus.push(t);
        ll.push(-0.5 * slog - 0.5 * pr.ln() - 0.5 * (sx2 - pr * mu_t * mu_t));
        mt.push(mu_t);
        prec.push(pr);
    }
    let lmax = ll.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut w: Vec<f64> = ll.iter().map(|l| (l - lmax).exp()).collect();
    let sw: f64 = w.iter().sum();
    w.iter_mut().for_each(|a| *a /= sw);
    let mean = w.iter().zip(&mt).map(|(a, b)| a * b).sum::<f64>();
    let second = w.iter().zip(&mt).zip(&prec).map(|((a, b), q)| a * (1.0 / q + b * b)).sum::<f64>();
    let sd = (second - mean * mean).max(0.0).sqrt();
    let tau_q = |q: f64| {
        let mut acc = 0.0;
        for (a, t) in w.iter().zip(&taus) {
            acc += a;
            if acc >= q {
                return *t;
            }
        }
        *taus.last().unwrap()
    };
    // Quantiles of the mixture of normals by bisection on its CDF; only the
    // grid points with non-negligible weight are kept.
    let parts: Vec<(f64, f64, f64)> = w
        .iter()
        .zip(&mt)
        .zip(&prec)
        .filter(|((a, _), _)| **a > 1e-12)
        .map(|((a, b), q)| (*a, *b, 1.0 / q.sqrt()))
        .collect();
    let cdf_pdf = |z: f64| {
        parts.iter().fold((0.0, 0.0), |(f, d), (a, b, s)| {
            let t = (z - b) / s;
            (f + a * crate::special::normal_cdf(t), d + a * crate::special::normal_pdf(t) / s)
        })
    };
    // Newton's method from the normal approximation, safeguarded by a
    // bracket that shrinks with every step.
    let quantile = |q: f64| {
        let (mut lo, mut hi) = (mean - 60.0 * sd, mean + 60.0 * sd);
        let mut z = mean + sd * crate::special::normal_quantile(q);
        for _ in 0..60 {
            let (f, d) = cdf_pdf(z);
            if f < q {
                lo = z;
            } else {
                hi = z;
            }
            let mut next = if d > 0.0 { z - (f - q) / d } else { 0.5 * (lo + hi) };
            if !(next > lo && next < hi) {
                next = 0.5 * (lo + hi);
            }
            if (next - z).abs() <= 1e-12 * sd {
                return next;
            }
            z = next;
        }
        z
    };
    let a = 0.5 * (1.0 - p);
    Ok(BayesConsensus {
        mean: mean + c,
        sd,
        interval: (quantile(a) + c, quantile(1.0 - a) + c),
        tau_median: tau_q(0.5),
        tau_interval: (tau_q(a), tau_q(1.0 - a)),
    })
}

/// ln erfcx(z) = z² + ln erfc(z) for z ≥ 5 by the asymptotic series
/// erfcx(z) = 1/(z√π) Σ_k (−1)^k (2k − 1)!!/(2z²)^k, summed while the terms
/// decrease (relative error below 10⁻¹⁰).
fn ln_erfcx_large(z: f64) -> f64 {
    let t = 1.0 / (2.0 * z * z);
    let (mut sum, mut term) = (1.0, 1.0);
    for k in 1..40 {
        let next = -term * (2 * k - 1) as f64 * t;
        if next.abs() >= term.abs() || next.abs() < 1e-17 {
            break;
        }
        term = next;
        sum += term;
    }
    (sum / (z * std::f64::consts::PI.sqrt())).ln()
}

/// ln of the density at d of a Laplace(0, b) plus an independent N(0, s²)
/// variable: (1/4b)[e^{a₁} erfc(z₁) + e^{a₂} erfc(z₂)] with
/// a₁,₂ = r²/2 ∓ d/b, z₁,₂ = (r ∓ d/s)/√2, r = s/b. For large z the identity
/// a − z² = −d²/2s² avoids the cancellation of two large terms.
fn ln_normal_laplace(d: f64, b: f64, s: f64) -> f64 {
    let r = s / b;
    let term = |sign: f64| {
        let z = (r - sign * d / s) / std::f64::consts::SQRT_2;
        if z < 5.0 {
            0.5 * r * r - sign * d / b + crate::special::erfc(z).ln()
        } else {
            -0.5 * (d / s).powi(2) + ln_erfcx_large(z)
        }
    };
    let (l1, l2) = (term(1.0), term(-1.0));
    let m = l1.max(l2);
    -(4.0 * b).ln() + m + ((l1 - m).exp() + (l2 - m).exp()).ln()
}

/// As [`bayes_known_u`] with Laplace laboratory effects of standard
/// deviation τ (scale τ/√2), the model the NIST Decision Tree recommends for
/// heavy-tailed effects (Possolo & Meija 2022, Hubble–Lemaître constant).
/// μ cannot be integrated analytically: the posterior is evaluated on a
/// grid in (θ, μ), τ = s·tan(πθ/2), with a second pass on μ about the mass of
/// the first.
pub fn bayes_laplace_known_u(
    x: &[f64],
    u: &[f64],
    tau_median: f64,
    mu_prior: Option<(f64, f64)>,
    p: f64,
) -> Result<BayesConsensus, UncError> {
    bayes_laplace_known_u_grid(x, u, tau_median, mu_prior, p, 300, 300)
}

/// [`bayes_laplace_known_u`] with `nt` points in θ and `nm` in μ. The θ grid
/// covers all τ; the second μ pass spans ±12 posterior standard deviations
/// about the mean of the first pass, which leaves a negligible mass outside.
pub fn bayes_laplace_known_u_grid(
    x: &[f64],
    u: &[f64],
    tau_median: f64,
    mu_prior: Option<(f64, f64)>,
    p: f64,
    nt: usize,
    nm: usize,
) -> Result<BayesConsensus, UncError> {
    validate(x, u, &[])?;
    if !(tau_median > 0.0) || !tau_median.is_finite() {
        return Err(invalid("the median of the half-Cauchy prior on τ must be positive"));
    }
    if !(p > 0.0 && p < 1.0) {
        return Err(invalid("the coverage probability must lie strictly between 0 and 1"));
    }
    let taus: Vec<f64> =
        (0..nt).map(|i| tau_median * (0.5 * std::f64::consts::PI * (i as f64 + 0.5) / nt as f64).tan()).collect();
    let xmin = x.iter().copied().fold(f64::INFINITY, f64::min);
    let xmax = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let umax = u.iter().copied().fold(0.0, f64::max);
    let pass = |lo: f64, hi: f64| {
        let mus: Vec<f64> = (0..nm).map(|k| lo + (hi - lo) * (k as f64 + 0.5) / nm as f64).collect();
        let mut ll = vec![0.0; nt * nm];
        for (i, t) in taus.iter().enumerate() {
            let b = t / std::f64::consts::SQRT_2;
            for (k, m) in mus.iter().enumerate() {
                let mut l: f64 = x.iter().zip(u).map(|(xi, ui)| ln_normal_laplace(xi - m, b, *ui)).sum();
                if let Some((m0, s0)) = mu_prior {
                    l -= 0.5 * ((m - m0) / s0).powi(2);
                }
                ll[i * nm + k] = l;
            }
        }
        let lmax = ll.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let w: Vec<f64> = ll.iter().map(|l| (l - lmax).exp()).collect();
        let sw: f64 = w.iter().sum();
        let mut pm = vec![0.0; nm];
        let mut pt = vec![0.0; nt];
        for i in 0..nt {
            for k in 0..nm {
                let a = w[i * nm + k] / sw;
                pm[k] += a;
                pt[i] += a;
            }
        }
        (mus, pm, pt)
    };
    let span = (xmax - xmin) + umax;
    let (mus, pm, _) = pass(xmin - 2.0 * span, xmax + 2.0 * span);
    let m1 = mus.iter().zip(&pm).map(|(a, b)| a * b).sum::<f64>();
    let s1 = mus.iter().zip(&pm).map(|(a, b)| b * (a - m1).powi(2)).sum::<f64>().sqrt();
    let h = (mus[1] - mus[0]).max(s1 / 50.0);
    let (mus, pm, pt) = pass(m1 - 12.0 * s1.max(h), m1 + 12.0 * s1.max(h));
    let mean = mus.iter().zip(&pm).map(|(a, b)| a * b).sum::<f64>();
    let sd = mus.iter().zip(&pm).map(|(a, b)| b * (a - mean).powi(2)).sum::<f64>().sqrt();
    // Quantiles: the grid masses are spread uniformly over their cells.
    let dm = mus[1] - mus[0];
    let mu_q = |q: f64| {
        let mut acc = 0.0;
        for (m, w) in mus.iter().zip(&pm) {
            if acc + w >= q {
                return m - 0.5 * dm + dm * (q - acc) / w;
            }
            acc += w;
        }
        *mus.last().unwrap()
    };
    // τ: interpolation in θ, where the grid is uniform.
    let tau_q = |q: f64| {
        let mut acc = 0.0;
        for (i, w) in pt.iter().enumerate() {
            if acc + w >= q {
                let th = (i as f64 + (q - acc) / w) / nt as f64;
                return tau_median * (0.5 * std::f64::consts::PI * th).tan();
            }
            acc += w;
        }
        *taus.last().unwrap()
    };
    let a = 0.5 * (1.0 - p);
    Ok(BayesConsensus {
        mean,
        sd,
        interval: (mu_q(a), mu_q(1.0 - a)),
        tau_median: tau_q(0.5),
        tau_interval: (tau_q(a), tau_q(1.0 - a)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Koepke et al. (2017) Table 1: 14 measurements of G /10⁻¹¹ m³ kg⁻¹ s⁻².
    const G: [f64; 14] = [
        6.67248, 6.6729, 6.67398, 6.674255, 6.67559, 6.67422, 6.67387, 6.67222, 6.67425, 6.67349, 6.67234, 6.67554,
        6.67191, 6.67435,
    ];
    const UG: [f64; 14] = [
        0.00043, 0.0005, 0.00070, 0.000092, 0.00027, 0.00098, 0.00027, 0.00087, 0.00012, 0.00018, 0.00014, 0.00016,
        0.00099, 0.00013,
    ];

    #[test]
    fn koepke_2017_g_dersimonian_laird() {
        // Table 2: 6.673 80, u 0.000 28; independent check (numpy):
        // μ = 6.6737896, u = 0.00027917, τ = 0.00094584, Q = 319.12.
        let r = dersimonian_laird(&G, &UG).unwrap();
        assert!((r.mean - 6.673_789_609).abs() < 1e-9, "{}", r.mean);
        assert!((r.u - 0.000_279_169_4).abs() < 1e-10, "{}", r.u);
        assert!((r.tau - 0.000_945_841_4).abs() < 1e-10, "{}", r.tau);
        assert!((r.q - 319.123_07).abs() < 1e-4, "{}", r.q);
        assert!((r.u_knapp_hartung - 0.000_306_615_6).abs() < 1e-10);
        assert!(r.q_p_value < 1e-50);
        assert!((r.weights.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn koepke_2017_g_bootstrap_and_linear_pool() {
        // Table 2: DL interval (6.673 20, 6.674 40); numpy bootstrap with
        // 10⁵ draws (6.67322, 6.67435), sd 0.000279.
        let b = dl_bootstrap(&G, &UG, &[], 0.95, 100_000, 1).unwrap();
        assert!((b.u - 0.000_279).abs() < 0.000_006, "{}", b.u);
        assert!((b.interval.0 - 6.673_224).abs() < 0.000_01, "{:?}", b.interval);
        assert!((b.interval.1 - 6.674_354).abs() < 0.000_01, "{:?}", b.interval);
        // Linear Pool: 6.673 67, 0.001 24, (6.671 18, 6.675 79). The mean and
        // sd of the equal-weight Gaussian mixture are mean(x) and
        // √(mean(u²) + mean((x − x̄)²)) = 6.6736711, 0.00124381.
        let lp = linear_pool(&G, &UG, &[], &[], 0.95, 1_000_000, 1).unwrap();
        assert!((lp.mean - 6.673_671).abs() < 0.000_005, "{}", lp.mean);
        assert!((lp.u - 0.001_243_8).abs() < 0.000_005, "{}", lp.u);
        assert!((lp.interval.0 - 6.671_18).abs() < 0.000_02, "{:?}", lp.interval);
        assert!((lp.interval.1 - 6.675_79).abs() < 0.000_02, "{:?}", lp.interval);
    }

    #[test]
    fn maximum_likelihood_iupac_2024_4f_and_koepke_g() {
        // IUPAC 2024 Example 4F (CCQM-K145, Ni in bovine liver, 17 NMIs):
        // μ̂ = 2.043(15) mg/kg, τ̂ = 0.044(14) mg/kg. scipy: 2.042939,
        // 0.043541, 0.015442, 0.014239.
        let x = [
            1.930, 1.940, 1.942, 1.958, 1.984, 1.993, 2.010, 2.020, 2.022, 2.050, 2.055, 2.070, 2.077, 2.080, 2.131,
            2.150, 2.180,
        ];
        let u = [
            0.110, 0.064, 0.092, 0.075, 0.020, 0.033, 0.060, 0.050, 0.023, 0.020, 0.052, 0.050, 0.035, 0.059, 0.042,
            0.030, 0.080,
        ];
        let r = maximum_likelihood(&x, &u).unwrap();
        assert!((r.mean - 2.042_939).abs() < 2e-6, "{}", r.mean);
        assert!((r.tau - 0.043_541).abs() < 2e-6, "{}", r.tau);
        assert!((r.u_mean - 0.015_442).abs() < 2e-6, "{}", r.u_mean);
        assert!((r.u_tau - 0.014_239).abs() < 2e-6, "{}", r.u_tau);
        // Koepke et al. Table 2, maximum likelihood: 6.673 78, 0.000 30.
        let g = maximum_likelihood(&G, &UG).unwrap();
        assert!((g.mean - 6.673_778).abs() < 2e-6 && (g.u_mean - 0.000_300).abs() < 2e-6, "{g:?}");
    }

    #[test]
    fn bayes_iupac_2024_4m_iridium() {
        // IUPAC 2024 Examples 3E, 4M, 4N: M = 192.2171(4) g/mol, median of
        // τ 0.0005 g/mol (MCMC, 10⁴ draws). Grid in numpy (2 × 10⁵ points):
        // mean 192.217 1188, sd 0.000 4398, (192.216 164, 192.217 946),
        // τ median 0.000 5061, (0.000 1008, 0.001 781).
        let x = [192.21677, 192.21662, 192.21763];
        let u = [0.00081, 0.00029, 0.00017];
        let b = bayes_known_u(&x, &u, 0.00042, Some((192.217, 0.0015)), 0.95).unwrap();
        assert!((b.mean - 192.217_118_83).abs() < 2e-9, "{b:?}");
        assert!((b.sd - 0.000_439_833).abs() < 5e-9, "{b:?}");
        assert!((b.interval.0 - 192.216_164).abs() < 2e-6 && (b.interval.1 - 192.217_946).abs() < 2e-6, "{b:?}");
        assert!((b.tau_median - 0.000_506_1).abs() < 2e-7, "{b:?}");
        assert!((b.tau_interval.1 - 0.001_781).abs() < 2e-6, "{b:?}");
        // Cochran's Q: p = 0.0082 printed (3E); 0.008 39 from the data.
        let d = dersimonian_laird(&x, &u).unwrap();
        assert!((d.q_p_value - 0.008_389).abs() < 2e-6, "{}", d.q_p_value);
    }

    #[test]
    fn nicob_hierarchical_bayes_koepke_g() {
        // NICOB §A.3 with unspecified ν (σ_j = u_j): τ half-Cauchy with median
        // MAD = 0.000 73, μ ~ N(0, 10⁵) (flat). Koepke Table 2: 6.673 77,
        // 0.000 33, (6.673 10, 6.674 42); grid in numpy 6.673 7733,
        // 0.000 328 63, (6.673 105, 6.674 409).
        let mad = median_absolute_deviation(&G);
        assert!((mad - 0.000_73).abs() < 1e-12, "{mad}");
        let b = bayes_known_u(&G, &UG, mad, None, 0.95).unwrap();
        assert!((b.mean - 6.673_773_32).abs() < 5e-9, "{b:?}");
        assert!((b.sd - 0.000_328_63).abs() < 2e-8, "{b:?}");
        assert!((b.interval.0 - 6.673_105).abs() < 2e-6 && (b.interval.1 - 6.674_409).abs() < 2e-6, "{b:?}");
    }

    #[test]
    fn normal_laplace_density_integrates_to_one_and_has_the_limits() {
        // ∫ f = 1; b → 0 gives N(0, s²); s → 0 gives Laplace(0, b).
        let (b, s) = (0.7, 0.4);
        let h = 1e-3;
        let total: f64 = (-20_000..20_000).map(|i| ln_normal_laplace(i as f64 * h, b, s).exp() * h).sum();
        assert!((total - 1.0).abs() < 1e-9, "{total}");
        let n = ln_normal_laplace(0.3, 1e-9, 0.4);
        let expect = -0.5 * (0.3f64 / 0.4).powi(2) - (0.4 * (2.0 * std::f64::consts::PI).sqrt()).ln();
        assert!((n - expect).abs() < 1e-6, "{n} {expect}");
        let l = ln_normal_laplace(0.3, 0.7, 1e-6);
        assert!((l - (-(0.3f64 / 0.7) - (1.4f64).ln())).abs() < 1e-5, "{l}");
    }

    #[test]
    fn bayes_laplace_possolo_meija_2022_hubble() {
        // Possolo & Meija (2022), Hubble–Lemaître constant: 22 estimates
        // (Planck excluded), Laplace effects, τ half-Cauchy with median
        // mad(x) = 1.4826 × 1.95, H0 ~ N(67.4, 100²). Stan: 71.69, 0.66,
        // median τ 2.03 (km/s)/Mpc. numpy grid: 71.6915, 0.6603, 2.0336.
        let x = [
            72.5, 67.8, 69.32, 74.03, 67.4, 70.3, 73.3, 69.8, 73.5, 70.3, 76.8, 74.2, 73.9, 75.8, 67.6, 67.4, 71.8,
            73.5, 73.2, 72.1, 69.8, 73.3,
        ];
        let u = [
            2.2, 1.3, 1.42, 1.42, 6.1, 5.15, 1.75, 1.9, 1.4, 1.35, 2.6, 2.85, 3.0, 5.05, 4.25, 1.0, 3.59, 5.3, 1.3,
            2.0, 1.7, 1.4,
        ];
        let gamma = 1.4826 * median_absolute_deviation(&x);
        let b = bayes_laplace_known_u(&x, &u, gamma, Some((67.4, 100.0)), 0.95).unwrap();
        assert!((b.mean - 71.6915).abs() < 0.0005, "{b:?}");
        assert!((b.sd - 0.6603).abs() < 0.0005, "{b:?}");
        assert!((b.tau_median - 2.034).abs() < 0.01, "{b:?}");
    }

    #[test]
    fn bayes_grids_have_converged() {
        // Independent review 2026-11-08, item 3: doubling the grid changes
        // the mean, the standard deviation and the quantiles of μ and τ by
        // less than 10⁻³ of the posterior standard deviation.
        let close = |a: &BayesConsensus, b: &BayesConsensus, what: &str| {
            let s = a.sd;
            for (x, y) in [
                (a.mean, b.mean),
                (a.sd, b.sd),
                (a.interval.0, b.interval.0),
                (a.interval.1, b.interval.1),
                (a.tau_median, b.tau_median),
                (a.tau_interval.1, b.tau_interval.1),
            ] {
                assert!((x - y).abs() < 1e-3 * s, "{what}: {x} vs {y} (sd {s})");
            }
        };
        let x = [192.21677, 192.21662, 192.21763];
        let u = [0.00081, 0.00029, 0.00017];
        let prior = Some((192.217, 0.0015));
        let a = bayes_known_u_grid(&x, &u, 0.00042, prior, 0.95, 100_000).unwrap();
        let b = bayes_known_u_grid(&x, &u, 0.00042, prior, 0.95, 200_000).unwrap();
        close(&a, &b, "iridium");
        let mad = median_absolute_deviation(&G);
        let a = bayes_known_u_grid(&G, &UG, mad, None, 0.95, 100_000).unwrap();
        let b = bayes_known_u_grid(&G, &UG, mad, None, 0.95, 200_000).unwrap();
        close(&a, &b, "G");
        // Laplace effects (G, a smaller set than Hubble's for speed).
        let a = bayes_laplace_known_u_grid(&G, &UG, mad, None, 0.95, 300, 300).unwrap();
        let b = bayes_laplace_known_u_grid(&G, &UG, mad, None, 0.95, 600, 600).unwrap();
        let s = a.sd;
        for (x, y) in [(a.mean, b.mean), (a.sd, b.sd), (a.interval.0, b.interval.0), (a.interval.1, b.interval.1)] {
            assert!((x - y).abs() < 1e-2 * s, "Laplace: {x} vs {y} (sd {s})");
        }
        assert!((a.tau_median - b.tau_median).abs() < 1e-2 * a.tau_median, "{a:?} {b:?}");
    }

    #[test]
    fn homogeneous_data_reduce_to_the_weighted_mean() {
        // τ̂ = 0 when Q ≤ n − 1 (Procedure A of Cox 2002).
        let x = [10.0, 10.1, 9.95];
        let u = [0.1, 0.2, 0.15];
        let r = dersimonian_laird(&x, &u).unwrap();
        assert_eq!(r.tau, 0.0);
        let w: Vec<f64> = u.iter().map(|a| 1.0 / (a * a)).collect();
        let sw: f64 = w.iter().sum();
        let m = w.iter().zip(&x).map(|(a, b)| a * b).sum::<f64>() / sw;
        assert!((r.mean - m).abs() < 1e-12);
        assert!((r.u - 1.0 / sw.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn bootstrap_with_q_below_n_minus_one() {
        // Q = 0.377 < n − 1 = 3: τ̂_DL = 0, τ²_M = −0.0158. As in NICOB the
        // gamma is set at τ²_M (E Q = Q), so most τ_k are 0 and u lies close
        // to the fixed-effects value 0.0647; numpy with 2 × 10⁵ draws gives
        // 0.06645 (0.0766 with the gamma at τ² = 0).
        let x = [10.0, 10.1, 9.95, 10.02];
        let u = [0.1, 0.2, 0.15, 0.12];
        let d = dersimonian_laird(&x, &u).unwrap();
        assert_eq!(d.tau, 0.0);
        let (shape, scale) = q_gamma_approximation_nicob(
            &u,
            (d.q - 3.0) / {
                let w: Vec<f64> = u.iter().map(|a| 1.0 / (a * a)).collect();
                let s1: f64 = w.iter().sum();
                s1 - w.iter().map(|a| a * a).sum::<f64>() / s1
            },
        )
        .unwrap();
        assert!((shape * scale - d.q).abs() < 1e-12, "E Q = Q");
        let b = dl_bootstrap(&x, &u, &[], 0.95, 200_000, 1).unwrap();
        assert!((b.u - 0.066_45).abs() < 0.0006, "{}", b.u);
        // With τ² > 0 the NICOB variance differs little from eq. (7).
        let t2 = dersimonian_laird(&G, &UG).unwrap().tau.powi(2);
        let (a1, s1) = q_gamma_approximation(&UG, t2);
        let (a2, s2) = q_gamma_approximation_nicob(&UG, t2).unwrap();
        assert!((a1 * s1 - a2 * s2).abs() < 1e-9 && ((a1 * s1 * s1) / (a2 * s2 * s2) - 1.0).abs() < 0.01);
    }

    #[test]
    fn leave_one_out_and_bilateral_degrees_of_equivalence() {
        // D*_j = x_j − μ̂_DL(others) exactly; B_ij = D_i − D_j; the reference
        // U95 % values (numpy port of the NICOB source, 10⁶ draws) are checked
        // in the library projects nicob-co60-* and nicob-pcb28-*.
        let x = [10.0, 10.4, 9.7, 10.1, 11.0];
        let u = [0.2, 0.3, 0.25, 0.1, 0.3];
        let r = dl_degrees_of_equivalence(&x, &u, &[], 0.95, 20_000, 3, true).unwrap();
        for j in 0..5 {
            let xo: Vec<f64> = (0..5).filter(|&i| i != j).map(|i| x[i]).collect();
            let uo: Vec<f64> = (0..5).filter(|&i| i != j).map(|i| u[i]).collect();
            let m = dersimonian_laird(&xo, &uo).unwrap().mean;
            assert!((r.unilateral[j].d - (x[j] - m)).abs() < 1e-12);
            assert!(r.unilateral[j].expanded > 1.96 * u[j]);
        }
        assert_eq!(r.bilateral.len(), 10);
        let b = &r.bilateral[3];
        assert_eq!((b.i, b.j), (0, 4));
        assert!((b.b - (r.unilateral[0].d - r.unilateral[4].d)).abs() < 1e-12);
        // MRA version: the same unilateral DoE as the bootstrap for the same seed.
        let m = dl_degrees_of_equivalence(&x, &u, &[], 0.95, 20_000, 3, false).unwrap();
        let s = dl_bootstrap(&x, &u, &[], 0.95, 20_000, 3).unwrap();
        assert_eq!(m.unilateral, s.degrees_of_equivalence);
        assert!(dl_degrees_of_equivalence(&x[..2], &u[..2], &[], 0.95, 20_000, 3, true).is_err());
        // NICOB's scale for ν ≤ 2 joins √(ν/(ν − 2)) only above 2; at ν = 2 it is finite.
        assert!(t_scale(2.0).is_finite() && t_scale(2.0) > 1.0 && (t_scale(10.0) - 1.25f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn gamma_approximation_has_the_chi_squared_limit() {
        // For τ = 0 the mean and variance of Q are n − 1 and 2(n − 1): the
        // gamma is χ²_{n−1} (shape (n−1)/2, scale 2).
        let (a, s) = q_gamma_approximation(&[1.0, 2.0, 3.0, 4.0], 0.0);
        assert!((a - 1.5).abs() < 1e-12 && (s - 2.0).abs() < 1e-12);
    }

    #[test]
    fn degrees_of_equivalence_with_degrees_of_freedom() {
        let b = dl_bootstrap(&G, &UG, &[f64::INFINITY; 14], 0.95, 20_000, 3).unwrap();
        let c = dl_bootstrap(&G, &UG, &[4.0; 14], 0.95, 20_000, 3).unwrap();
        for (a, b) in b.degrees_of_equivalence.iter().zip(&c.degrees_of_equivalence) {
            assert_eq!(a.d, b.d);
            assert!(a.expanded > 0.0 && b.expanded > 0.0);
        }
        assert!(dl_bootstrap(&G, &UG, &[0.0; 14], 0.95, 20_000, 3).is_err());
        // ν = 2 (infinite t variance): rescaled with NICOB's approximation (linearOP.R).
        let lp = linear_pool(&G, &UG, &[2.0; 14], &[], 0.95, 20_000, 3).unwrap();
        assert!(lp.u.is_finite() && lp.degrees_of_equivalence.iter().all(|d| d.expanded.is_finite()));
    }
}
