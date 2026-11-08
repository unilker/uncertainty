//! Statistical methods for proficiency testing by interlaboratory comparison
//! [ISO 13528:2022].
//!
//! * simple robust estimators: median, MADe and nIQR (Annex C.2);
//! * Algorithm A with iterated scale (C.3.1) and Algorithm S (C.4);
//! * standard uncertainty of a consensus assigned value, Formula (6);
//! * homogeneity (B.2, B.3) and stability (B.5) checks;
//! * performance statistics D, D %, PA, z, z′, ζ and E_n (Clause 9) and the
//!   screening of reported uncertainties (9.8).

use crate::error::UncError;
use crate::special::{chi_squared_quantile, f_quantile};

fn sorted(x: &[f64]) -> Result<Vec<f64>, UncError> {
    if x.is_empty() {
        return Err(UncError::InvalidOption("no data".into()));
    }
    if x.iter().any(|v| !v.is_finite()) {
        return Err(UncError::InvalidOption("data must be finite".into()));
    }
    let mut v = x.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(v)
}

fn median_sorted(v: &[f64]) -> f64 {
    let p = v.len();
    if p % 2 == 1 {
        v[p / 2]
    } else {
        0.5 * (v[p / 2 - 1] + v[p / 2])
    }
}

/// Median of the data, Formula (C.1).
pub fn median(x: &[f64]) -> Result<f64, UncError> {
    Ok(median_sorted(&sorted(x)?))
}

/// Scaled median absolute deviation MADe(x) = 1.483 med|xᵢ − med(x)|,
/// Formulae (C.2) and (C.3).
pub fn made(x: &[f64]) -> Result<f64, UncError> {
    let m = median(x)?;
    let d: Vec<f64> = x.iter().map(|v| (v - m).abs()).collect();
    Ok(1.483 * median(&d)?)
}

/// First and third quartiles by linear interpolation between order
/// statistics (the definition of QUARTILE.INC and of type 7 of Hyndman and
/// Fan; it reproduces the nIQR of ISO 13528 Table E.5).
pub fn quartiles(x: &[f64]) -> Result<(f64, f64), UncError> {
    let v = sorted(x)?;
    let q = |p: f64| {
        let h = (v.len() - 1) as f64 * p;
        let i = h.floor() as usize;
        if i + 1 < v.len() {
            v[i] + (h - i as f64) * (v[i + 1] - v[i])
        } else {
            v[i]
        }
    };
    Ok((q(0.25), q(0.75)))
}

/// Normalised interquartile range nIQR(x) = 0.7413 (Q₃ − Q₁), Formula (C.4).
pub fn niqr(x: &[f64]) -> Result<f64, UncError> {
    let (q1, q3) = quartiles(x)?;
    Ok(0.7413 * (q3 - q1))
}

/// Arithmetic mean and sample standard deviation.
pub fn mean_sd(x: &[f64]) -> Result<(f64, f64), UncError> {
    sorted(x)?;
    let n = x.len() as f64;
    let m = x.iter().sum::<f64>() / n;
    let sd = if x.len() > 1 { (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (n - 1.0)).sqrt() } else { f64::NAN };
    Ok((m, sd))
}

/// When Algorithm A stops iterating.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stopping {
    /// No change in the third significant figures of x* and s* (C.3.1).
    ThirdSignificantFigure,
    /// Iterate until x* and s* change by less than 10⁻¹² relative.
    Converged,
}

/// Result of Algorithm A.
#[derive(Clone, Debug)]
pub struct AlgorithmA {
    /// Robust average x*.
    pub mean: f64,
    /// Robust standard deviation s*.
    pub sd: f64,
    /// Number of updates of x* and s*.
    pub iterations: usize,
    /// (x*, s*) after each update, starting with the initial values.
    pub history: Vec<(f64, f64)>,
    /// True if the initial s* was the sample standard deviation because
    /// MADe was zero (C.3.1 Note 2).
    pub initial_sd_substituted: bool,
}

fn round_sig(x: f64, digits: i32) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return x;
    }
    let e = x.abs().log10().floor() as i32;
    let f = 10f64.powi(digits - 1 - e);
    (x * f).round() / f
}

/// Algorithm A with iterated scale [ISO 13528:2022 C.3.1].
pub fn algorithm_a(x: &[f64], stopping: Stopping) -> Result<AlgorithmA, UncError> {
    if x.len() < 2 {
        return Err(UncError::InvalidOption("Algorithm A needs at least two results".into()));
    }
    let mut xs = median(x)?;
    let mut ss = made(x)?;
    let mut substituted = false;
    if ss == 0.0 {
        ss = mean_sd(x)?.1;
        substituted = true;
    }
    if !(ss > 0.0) {
        return Err(UncError::InvalidOption("all results are equal: no robust standard deviation".into()));
    }
    let p = x.len() as f64;
    let mut history = vec![(xs, ss)];
    for it in 1..=1000 {
        let delta = 1.5 * ss;
        let w: Vec<f64> = x.iter().map(|&v| v.clamp(xs - delta, xs + delta)).collect();
        let xn = w.iter().sum::<f64>() / p;
        let sn = 1.134 * (w.iter().map(|v| (v - xn).powi(2)).sum::<f64>() / (p - 1.0)).sqrt();
        history.push((xn, sn));
        let done = match stopping {
            Stopping::ThirdSignificantFigure => {
                round_sig(xn, 3) == round_sig(xs, 3) && round_sig(sn, 3) == round_sig(ss, 3)
            }
            Stopping::Converged => (xn - xs).abs() <= 1e-12 * xn.abs().max(sn) && (sn - ss).abs() <= 1e-12 * sn,
        };
        xs = xn;
        ss = sn;
        if done {
            return Ok(AlgorithmA { mean: xs, sd: ss, iterations: it, history, initial_sd_substituted: substituted });
        }
    }
    Err(UncError::InvalidOption("Algorithm A did not converge".into()))
}

/// Factors (η, ξ) of Table C.1 for Algorithm S, ν = 1 … 10.
const ALGORITHM_S_FACTORS: [(f64, f64); 10] = [
    (1.645, 1.097),
    (1.517, 1.054),
    (1.444, 1.039),
    (1.395, 1.032),
    (1.359, 1.027),
    (1.332, 1.024),
    (1.310, 1.021),
    (1.292, 1.019),
    (1.277, 1.018),
    (1.264, 1.017),
];

/// Algorithm S [ISO 13528:2022 C.4]: robust pooled value w* of standard
/// deviations (or ranges, ν = 1) each with ν degrees of freedom.
pub fn algorithm_s(w: &[f64], dof: usize) -> Result<f64, UncError> {
    let (eta, xi) = *ALGORITHM_S_FACTORS
        .get(dof.wrapping_sub(1))
        .ok_or_else(|| UncError::InvalidOption("Algorithm S: Table C.1 covers 1 to 10 degrees of freedom".into()))?;
    if w.iter().any(|v| !(*v >= 0.0)) {
        return Err(UncError::InvalidOption("Algorithm S: standard deviations must be non-negative".into()));
    }
    let mut ws = median(w)?;
    if ws == 0.0 {
        ws = (w.iter().map(|v| v * v).sum::<f64>() / w.len() as f64).sqrt();
    }
    if ws == 0.0 {
        return Ok(0.0);
    }
    for _ in 0..1000 {
        let psi = eta * ws;
        let new = xi * (w.iter().map(|&v| v.min(psi).powi(2)).sum::<f64>() / w.len() as f64).sqrt();
        let done = round_sig(new, 3) == round_sig(ws, 3);
        ws = new;
        if done {
            return Ok(ws);
        }
    }
    Err(UncError::InvalidOption("Algorithm S did not converge".into()))
}

/// Standard uncertainty of an assigned value obtained as a robust average
/// of p participant results: u(x_pt) = factor · s*/√p, factor = 1.25 in
/// Formula (6).
pub fn u_assigned_consensus(robust_sd: f64, p: usize, factor: f64) -> f64 {
    factor * robust_sd / (p as f64).sqrt()
}

/// Result of a homogeneity check [ISO 13528:2022 B.2, B.3].
#[derive(Clone, Debug)]
pub struct Homogeneity {
    /// Number of proficiency test items g and replicates m.
    pub items: usize,
    pub replicates: usize,
    /// General average x̄ (B.6).
    pub mean: f64,
    /// Standard deviation of the item averages s_x (B.7).
    pub s_x: f64,
    /// Within-sample standard deviation s_w (B.8).
    pub s_w: f64,
    /// Between-sample standard deviation s_s (B.10, zero if negative).
    pub s_s: f64,
    /// Mean squares between and within items of the one-way ANOVA
    /// (MS_between = m·s_x², MS_within = s_w²).
    pub ms_between: f64,
    pub ms_within: f64,
    /// Upper bound of the between-unit standard deviation that the
    /// repeatability can hide, u*_bb = √(MS_within/m)·(2/ν_within)^¼
    /// (Linsinger et al., Accred. Qual. Assur. 6 (2001) 20; ISO Guide 35).
    pub u_bb_star: f64,
    /// Criterion 0.3 σ_pt (B.1), when σ_pt is given.
    pub criterion: Option<f64>,
    pub sufficient: Option<bool>,
    /// Factors F₁ and F_m and the expanded criterion √c of B.2.3.
    pub f1: f64,
    pub fm: f64,
    pub expanded_criterion: Option<f64>,
    pub sufficient_expanded: Option<bool>,
}

/// Homogeneity check from g items measured m times each (equal m),
/// against σ_pt if given [ISO 13528:2022 B.2.2, B.2.3, B.3].
pub fn homogeneity(items: &[Vec<f64>], sigma_pt: Option<f64>) -> Result<Homogeneity, UncError> {
    let g = items.len();
    if g < 2 {
        return Err(UncError::InvalidOption("homogeneity check: at least two items are needed".into()));
    }
    let m = items[0].len();
    if m < 2 || items.iter().any(|t| t.len() != m) {
        return Err(UncError::InvalidOption(
            "homogeneity check: every item needs the same number (≥ 2) of replicates".into(),
        ));
    }
    if items.iter().flatten().any(|v| !v.is_finite()) || sigma_pt.is_some_and(|s| !(s > 0.0)) {
        return Err(UncError::InvalidOption("homogeneity check: finite data and σ_pt > 0 required".into()));
    }
    let means: Vec<f64> = items.iter().map(|t| t.iter().sum::<f64>() / m as f64).collect();
    let var_t: Vec<f64> = items
        .iter()
        .zip(&means)
        .map(|(t, &xt)| t.iter().map(|v| (v - xt).powi(2)).sum::<f64>() / (m as f64 - 1.0))
        .collect();
    let mean = means.iter().sum::<f64>() / g as f64;
    let s_x2 = means.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (g as f64 - 1.0);
    let s_w2 = var_t.iter().sum::<f64>() / g as f64;
    let s_s = (s_x2 - s_w2 / m as f64).max(0.0).sqrt();
    let criterion = sigma_pt.map(|s| 0.3 * s);
    let f1 = chi_squared_quantile(0.95, g as f64 - 1.0) / (g as f64 - 1.0);
    let fm = (f_quantile(0.95, g as f64 - 1.0, (g * (m - 1)) as f64) - 1.0) / m as f64;
    let expanded_criterion = criterion.map(|c| (f1 * c * c + fm * s_w2).sqrt());
    let nu_w = (g * (m - 1)) as f64;
    Ok(Homogeneity {
        items: g,
        replicates: m,
        mean,
        s_x: s_x2.sqrt(),
        s_w: s_w2.sqrt(),
        s_s,
        ms_between: m as f64 * s_x2,
        ms_within: s_w2,
        u_bb_star: (s_w2 / m as f64).sqrt() * (2.0 / nu_w).powf(0.25),
        criterion,
        sufficient: criterion.map(|c| s_s <= c),
        f1,
        fm,
        expanded_criterion,
        sufficient_expanded: expanded_criterion.map(|c| s_s <= c),
    })
}

/// Priors of the Bayesian between-unit model y_ij = µ + B_i + ε_ij with
/// B_i ~ N(0, τ²), ε_ij ~ N(0, σ²): half-Cauchy priors with scales τ₀ and
/// σ₀, and a normal prior for µ (flat if not given) [JCGM GUM-5:2026
/// Example 12, (12.2)–(12.4); van der Veen 2017].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BetweenUnitPrior {
    pub tau_scale: f64,
    pub sigma_scale: f64,
    pub mu: Option<(f64, f64)>,
}

/// Summary of a marginal posterior PDF.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PosteriorSummary {
    pub mean: f64,
    pub sd: f64,
    pub median: f64,
    pub mode: f64,
    /// Probabilistically symmetric 95 % interval.
    pub q025: f64,
    pub q975: f64,
    /// Shortest 95 % interval (highest posterior density).
    pub hpd: (f64, f64),
}

/// Posterior of the Bayesian between-unit model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BetweenUnitPosterior {
    /// Between-unit standard deviation τ.
    pub tau: PosteriorSummary,
    /// Within-unit (repeatability) standard deviation σ.
    pub sigma: PosteriorSummary,
    /// Posterior mean and standard deviation of µ.
    pub mu_mean: f64,
    pub mu_sd: f64,
}

/// Summary of a PDF tabulated at increasing points x with trapezoidal weights.
fn summarize(x: &[f64], p: &[f64]) -> PosteriorSummary {
    let n = x.len();
    let mut w = vec![0.0; n];
    for i in 0..n - 1 {
        let h = 0.5 * (x[i + 1] - x[i]);
        w[i] += h;
        w[i + 1] += h;
    }
    let mass: Vec<f64> = (0..n).map(|i| p[i] * w[i]).collect();
    let total: f64 = mass.iter().sum();
    let mean = (0..n).map(|i| x[i] * mass[i]).sum::<f64>() / total;
    let sd = ((0..n).map(|i| (x[i] - mean).powi(2) * mass[i]).sum::<f64>() / total).sqrt();
    // Cumulative distribution by the trapezoidal rule, then linear interpolation.
    let mut cdf = vec![0.0; n];
    for i in 1..n {
        cdf[i] = cdf[i - 1] + 0.5 * (p[i - 1] + p[i]) * (x[i] - x[i - 1]) / total;
    }
    let quantile = |q: f64| {
        let i = cdf.partition_point(|&c| c < q).clamp(1, n - 1);
        let (c0, c1) = (cdf[i - 1], cdf[i]);
        if c1 > c0 {
            x[i - 1] + (q - c0) / (c1 - c0) * (x[i] - x[i - 1])
        } else {
            x[i]
        }
    };
    let imax = (0..n).max_by(|&a, &b| p[a].total_cmp(&p[b])).unwrap_or(0);
    // Highest density region: the points of highest density holding 95 %.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| p[b].total_cmp(&p[a]));
    let (mut acc, mut lo, mut hi) = (0.0, x[imax], x[imax]);
    for &i in &order {
        if acc >= 0.95 * total {
            break;
        }
        acc += mass[i];
        lo = lo.min(x[i]);
        hi = hi.max(x[i]);
    }
    PosteriorSummary {
        mean,
        sd,
        median: quantile(0.5),
        mode: x[imax],
        q025: quantile(0.025),
        q975: quantile(0.975),
        hpd: (lo, hi),
    }
}

/// Bayesian estimate of the between-unit standard deviation from g units
/// measured m times each. The unit effects and µ are integrated out
/// analytically; the joint posterior of (τ, σ) is evaluated on a grid, so
/// the result is deterministic (no MCMC). It exists also when
/// MS_between < MS_within, where the ANOVA estimate does not.
pub fn between_unit_posterior(items: &[Vec<f64>], prior: &BetweenUnitPrior) -> Result<BetweenUnitPosterior, UncError> {
    let h = homogeneity(items, None)?;
    let invalid = |m: &str| Err(UncError::InvalidOption(format!("between-unit posterior: {m}")));
    if !(prior.tau_scale > 0.0 && prior.sigma_scale > 0.0) {
        return invalid("the scales of the half-Cauchy priors must be positive");
    }
    if prior.mu.is_some_and(|(_, s)| !(s > 0.0)) {
        return invalid("the prior standard deviation of µ must be positive");
    }
    let (g, m) = (h.items as f64, h.replicates as f64);
    let means: Vec<f64> = items.iter().map(|t| t.iter().sum::<f64>() / m).collect();
    let s1: f64 = means.iter().sum();
    let s2: f64 = means.iter().map(|v| v * v).sum();
    let ssw = h.ms_within * g * (m - 1.0);
    let nu_w = g * (m - 1.0);
    // log posterior of (τ, σ) and the conditional mean and variance of µ.
    let point = |tau: f64, sig: f64| -> (f64, f64, f64) {
        let v = tau * tau + sig * sig / m;
        let (p0, m0) = prior.mu.map_or((0.0, 0.0), |(mu0, s0)| (1.0 / (s0 * s0), mu0));
        let prec = g / v + p0;
        let mean_mu = (s1 / v + p0 * m0) / prec;
        let quad = s2 / v + p0 * m0 * m0 - prec * mean_mu * mean_mu;
        let lp = -0.5 * g * v.ln()
            - 0.5 * quad
            - 0.5 * prec.ln()
            - nu_w * sig.ln()
            - ssw / (2.0 * sig * sig)
            - (1.0 + (tau / prior.tau_scale).powi(2)).ln()
            - (1.0 + (sig / prior.sigma_scale).powi(2)).ln();
        (lp, mean_mu, 1.0 / prec)
    };
    let sw = h.ms_within.sqrt().max(1e-300);
    let (nt, ns) = (3001, 1001);
    let tau_max = 12.0 * ((h.ms_between.max(h.ms_within)) / m).sqrt() + 6.0 * prior.tau_scale.min(sw);
    // Quadratic spacing: fine near τ = 0, where the density of τ is often largest.
    let taus: Vec<f64> = (0..nt).map(|i| tau_max * (i as f64 / (nt - 1) as f64).powi(2)).collect();
    let (ls0, ls1) = ((sw / 6.0).ln(), (sw * 6.0).ln());
    let sigs: Vec<f64> = (0..ns).map(|j| (ls0 + (ls1 - ls0) * j as f64 / (ns - 1) as f64).exp()).collect();
    let mut lp = vec![0.0; nt * ns];
    let mut mu_c = vec![(0.0, 0.0); nt * ns];
    for (i, &t) in taus.iter().enumerate() {
        for (j, &sg) in sigs.iter().enumerate() {
            let (l, a, b) = point(t, sg);
            lp[i * ns + j] = l;
            mu_c[i * ns + j] = (a, b);
        }
    }
    let lmax = lp.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !lmax.is_finite() {
        return invalid("the posterior could not be evaluated");
    }
    // Trapezoidal weights in σ.
    let mut ws = vec![0.0; ns];
    for j in 0..ns - 1 {
        let d = 0.5 * (sigs[j + 1] - sigs[j]);
        ws[j] += d;
        ws[j + 1] += d;
    }
    let mut wt = vec![0.0; nt];
    for i in 0..nt - 1 {
        let d = 0.5 * (taus[i + 1] - taus[i]);
        wt[i] += d;
        wt[i + 1] += d;
    }
    let dens: Vec<f64> = lp.iter().map(|l| (l - lmax).exp()).collect();
    let p_tau: Vec<f64> = (0..nt).map(|i| (0..ns).map(|j| dens[i * ns + j] * ws[j]).sum()).collect();
    let p_sig: Vec<f64> = (0..ns).map(|j| (0..nt).map(|i| dens[i * ns + j] * wt[i]).sum()).collect();
    let (mut z, mut e1, mut e2) = (0.0, 0.0, 0.0);
    for i in 0..nt {
        for j in 0..ns {
            let w = dens[i * ns + j] * wt[i] * ws[j];
            let (a, b) = mu_c[i * ns + j];
            z += w;
            e1 += w * a;
            e2 += w * (b + a * a);
        }
    }
    let mu_mean = e1 / z;
    Ok(BetweenUnitPosterior {
        tau: summarize(&taus, &p_tau),
        sigma: summarize(&sigs, &p_sig),
        mu_mean,
        mu_sd: (e2 / z - mu_mean * mu_mean).max(0.0).sqrt(),
    })
}

/// Result of a stability check [ISO 13528:2022 B.5].
#[derive(Clone, Debug)]
pub struct Stability {
    pub difference: f64,
    /// 0.3 σ_pt (B.17), widened by 2√(u²(ȳ₁) + u²(ȳ₂)) when the
    /// uncertainties are given (B.18).
    pub criterion: f64,
    pub sufficient: bool,
}

pub fn stability(y1: f64, y2: f64, sigma_pt: f64, u: Option<(f64, f64)>) -> Stability {
    let difference = (y1 - y2).abs();
    let criterion = 0.3 * sigma_pt + u.map_or(0.0, |(u1, u2)| 2.0 * (u1 * u1 + u2 * u2).sqrt());
    Stability { difference, criterion, sufficient: difference <= criterion }
}

/// Performance statistics of one result [ISO 13528:2022 9.3–9.7].
#[derive(Clone, Debug, Default)]
pub struct Scores {
    /// D = x − x_pt (11) and D % (12).
    pub d: f64,
    pub d_percent: Option<f64>,
    /// PA = D/δ_E · 100 % (13).
    pub pa: Option<f64>,
    /// z (14) and z′ (15).
    pub z: Option<f64>,
    pub z_prime: Option<f64>,
    /// ζ (19) and E_n (20); need the participant's uncertainty.
    pub zeta: Option<f64>,
    pub en: Option<f64>,
}

/// Inputs to the performance statistics of a round.
#[derive(Clone, Copy, Debug)]
pub struct Criteria {
    pub assigned: f64,
    pub u_assigned: f64,
    /// Expanded uncertainty of the assigned value for E_n (k = 2 if not
    /// stated otherwise).
    pub expanded_assigned: f64,
    pub sigma_pt: Option<f64>,
    pub delta_e: Option<f64>,
}

pub fn scores(x: f64, u_x: Option<f64>, expanded_x: Option<f64>, c: &Criteria) -> Scores {
    let d = x - c.assigned;
    Scores {
        d,
        d_percent: (c.assigned != 0.0).then(|| 100.0 * d / c.assigned),
        pa: c.delta_e.map(|de| 100.0 * d / de),
        z: c.sigma_pt.map(|s| d / s),
        z_prime: c.sigma_pt.map(|s| d / (s * s + c.u_assigned * c.u_assigned).sqrt()),
        zeta: u_x.map(|u| d / (u * u + c.u_assigned * c.u_assigned).sqrt()),
        en: expanded_x.map(|u| d / (u * u + c.expanded_assigned * c.expanded_assigned).sqrt()),
    }
}

/// Interpretation of z, z′ and ζ scores (9.4.2): acceptable, warning or
/// action signal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signal {
    Acceptable,
    Warning,
    Action,
}

pub fn z_signal(z: f64) -> Signal {
    if z.abs() <= 2.0 {
        Signal::Acceptable
    } else if z.abs() < 3.0 {
        Signal::Warning
    } else {
        Signal::Action
    }
}

/// E_n is satisfactory for |E_n| < 1 (9.7.2).
pub fn en_satisfactory(en: f64) -> bool {
    en.abs() < 1.0
}

/// Screening of a reported standard uncertainty against u_min and u_max
/// (9.8.3, 9.8.4): `Some(Less)` below u_min, `Some(Greater)` above u_max.
pub fn screen_uncertainty(u: f64, u_min: Option<f64>, u_max: Option<f64>) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    if u_min.is_some_and(|m| u < m) {
        Ordering::Less
    } else if u_max.is_some_and(|m| u > m) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ISO 13528:2022 Table E.4, atrazine in drinking water (mg/L).
    const ATRAZINE: [f64; 34] = [
        0.040, 0.055, 0.178, 0.202, 0.206, 0.227, 0.228, 0.230, 0.230, 0.235, 0.236, 0.237, 0.243, 0.244, 0.245,
        0.2555, 0.260, 0.264, 0.267, 0.270, 0.273, 0.274, 0.274, 0.278, 0.2811, 0.287, 0.287, 0.288, 0.289, 0.295,
        0.296, 0.311, 0.331, 0.4246,
    ];

    #[test]
    fn atrazine_simple_estimators_table_e5() {
        assert!((median(&ATRAZINE).unwrap() - 0.2620).abs() < 5e-5);
        assert!((made(&ATRAZINE).unwrap() - 0.0386).abs() < 5e-5);
        assert!((niqr(&ATRAZINE).unwrap() - 0.0402).abs() < 5e-5);
        let u = u_assigned_consensus(niqr(&ATRAZINE).unwrap(), 34, 1.25);
        assert!((u - 0.0086).abs() < 5e-5);
        let (m, s) = mean_sd(&ATRAZINE).unwrap();
        assert!((m - 0.2512).abs() < 5e-5 && (s - 0.0672).abs() < 5e-5);
    }

    #[test]
    fn atrazine_algorithm_a_table_e4() {
        let a = algorithm_a(&ATRAZINE, Stopping::ThirdSignificantFigure).unwrap();
        // Six iterations, x* = 0.2570, s* = 0.0395 (Table E.4).
        assert_eq!(a.iterations, 6);
        assert!((a.mean - 0.2570).abs() < 5e-5 && (a.sd - 0.0395).abs() < 5e-5);
        // The limits x* ± δ of every iteration of Table E.4.
        let limits = [
            (0.204163, 0.319837),
            (0.199732, 0.315969),
            (0.198466, 0.315871),
            (0.198037, 0.316065),
            (0.197865, 0.316185),
            (0.197790, 0.316243),
        ];
        for (i, &(lo, hi)) in limits.iter().enumerate() {
            let (xs, ss) = a.history[i];
            assert!((xs - 1.5 * ss - lo).abs() < 5e-7 && (xs + 1.5 * ss - hi).abs() < 5e-7, "iteration {}", i + 1);
        }
        assert!((u_assigned_consensus(a.sd, 34, 1.25) - 0.0085).abs() < 5e-5);
        let c = algorithm_a(&ATRAZINE, Stopping::Converged).unwrap();
        assert!((c.mean - a.mean).abs() < 1e-5 && (c.sd - a.sd).abs() < 5e-5);
    }

    #[test]
    fn arsenic_homogeneity_table_e2() {
        let d = [
            [0.185, 0.194],
            [0.187, 0.189],
            [0.182, 0.186],
            [0.188, 0.196],
            [0.191, 0.181],
            [0.188, 0.180],
            [0.187, 0.196],
            [0.177, 0.186],
            [0.179, 0.187],
            [0.188, 0.196],
        ];
        let items: Vec<Vec<f64>> = d.iter().map(|r| r.to_vec()).collect();
        let mean = items.iter().map(|t| t.iter().sum::<f64>() / 2.0).sum::<f64>() / 10.0;
        let h = homogeneity(&items, Some(0.15 * mean)).unwrap();
        assert!((h.mean - 0.18715).abs() < 5e-6);
        assert!((h.s_x - 0.00398).abs() < 5e-6 && (h.s_w - 0.00556).abs() < 5e-6 && (h.s_s - 0.00060).abs() < 5e-6);
        assert!((h.criterion.unwrap() - 0.00842).abs() < 5e-6 && h.sufficient == Some(true));
        // Table B.1 for g = 10: F₁ = 1.88, F₂ = 1.01.
        assert!((h.f1 - 1.88).abs() < 0.005 && (h.fm - 1.01).abs() < 0.005);
        // Stability, Table E.3.
        let y2 = (0.191 + 0.198 + 0.190 + 0.196) / 4.0;
        let st = stability(h.mean, y2, 0.15 * mean, None);
        assert!((st.difference - 0.0066).abs() < 5e-6 && st.sufficient);
    }

    #[test]
    fn table_b1_factors() {
        // F₁ = χ²₀.₉₅(g − 1)/(g − 1), F₂ = (F₀.₉₅(g − 1, g) − 1)/2.
        for &(g, f1, f2) in &[(20.0, 1.59, 0.57), (15.0, 1.69, 0.71), (7.0, 2.10, 1.43)] {
            let a = chi_squared_quantile(0.95, g - 1.0) / (g - 1.0);
            let b = (f_quantile(0.95, g - 1.0, g) - 1.0) / 2.0;
            assert!((a - f1).abs() < 0.005 && (b - f2).abs() < 0.005, "g = {g}");
        }
    }

    #[test]
    fn mercury_scores_table_e7() {
        // ISO 13528:2022 E.4 (IMEP 111): x_pt = 0.044, U(x_pt) = 0.0082,
        // σ_pt = 0.0066 mg/kg, δ_E = 3σ_pt.
        let c = Criteria {
            assigned: 0.044,
            u_assigned: 0.0041,
            expanded_assigned: 0.0082,
            sigma_pt: Some(0.0066),
            delta_e: Some(0.0198),
        };
        // L04: 0.013, U = 0.003 (k = 2): D % −70.5, PA −156.6, z −4.70, z′ −3.99, ζ −7.10, E_n −3.55.
        let s = scores(0.013, Some(0.0015), Some(0.003), &c);
        let r = |v: Option<f64>, d: i32| (v.unwrap() * 10f64.powi(d)).round() / 10f64.powi(d);
        assert_eq!(
            (r(s.d_percent, 1), r(s.pa, 1), r(s.z, 2), r(s.z_prime, 2), r(s.zeta, 2), r(s.en, 2)),
            (-70.5, -156.6, -4.70, -3.99, -7.10, -3.55)
        );
        // L01: 0.053, U = 0.007: 20.5 %, 45.5 %, 1.36, 1.16, 1.67, 0.83.
        let s = scores(0.053, Some(0.0035), Some(0.007), &c);
        assert_eq!(
            (r(s.d_percent, 1), r(s.pa, 1), r(s.z, 2), r(s.z_prime, 2), r(s.zeta, 2), r(s.en, 2)),
            (20.5, 45.5, 1.36, 1.16, 1.67, 0.83)
        );
        assert_eq!(z_signal(-4.70), Signal::Action);
        assert_eq!(z_signal(2.5), Signal::Warning);
        assert!(!en_satisfactory(-3.55));
    }

    #[test]
    fn algorithm_s_pools_standard_deviations() {
        // Equal values are never clipped (η > 1), so w* = ξ·w.
        let w = [0.5; 8];
        assert!((algorithm_s(&w, 1).unwrap() - 1.097 * 0.5).abs() < 1e-12);
        assert!((algorithm_s(&w, 4).unwrap() - 1.032 * 0.5).abs() < 1e-12);
        // A gross outlier is clipped at ψ = η·w*: the result stays near the
        // bulk instead of following the root mean square.
        let w = [0.40, 0.45, 0.50, 0.52, 0.55, 0.60, 0.48, 5.0];
        let ws = algorithm_s(&w, 3).unwrap();
        let rms = (w.iter().map(|v| v * v).sum::<f64>() / 8.0).sqrt();
        assert!(ws < 0.75 && rms > 1.7, "w* = {ws}");
        assert!(algorithm_s(&w, 11).is_err());
    }

    /// JCGM GUM-5:2026 Example 12, Table 12.1 (amount fraction of N₂ in 10
    /// gas mixtures, cmol/mol). ANOVA of Table 12.2 and the Stan output of
    /// 12.4.5: τ mean 0.00014, sd 0.00011, [0.00001, 0.00040]; σ mean 0.00069,
    /// sd 0.00007, [0.00056, 0.00085]; µ 0.42514; Table 12.3 HPD of τ/µ
    /// [0, 0.0008], of σ/µ [0.0013, 0.0020]; 12.4.8 median of τ 1.1 µmol/mol.
    #[test]
    fn gum5_example12_between_bottle_bayes() {
        let y = [
            [0.424577, 0.425167, 0.425379, 0.424522, 0.424805],
            [0.425572, 0.425411, 0.423638, 0.425301, 0.424527],
            [0.424152, 0.425517, 0.425638, 0.424207, 0.425135],
            [0.426320, 0.424672, 0.425211, 0.425533, 0.425864],
            [0.424855, 0.425079, 0.425413, 0.424729, 0.424725],
            [0.425104, 0.424773, 0.426424, 0.424266, 0.424632],
            [0.425750, 0.424917, 0.424779, 0.425086, 0.425318],
            [0.425547, 0.426483, 0.424631, 0.425968, 0.424620],
            [0.426326, 0.424646, 0.425205, 0.426302, 0.425020],
            [0.425968, 0.424069, 0.425988, 0.425489, 0.423936],
        ];
        let items: Vec<Vec<f64>> = y.iter().map(|r| r.to_vec()).collect();
        let h = homogeneity(&items, None).unwrap();
        assert!((h.ms_between - 3.25e-7).abs() < 0.005e-7 && (h.ms_within - 4.90e-7).abs() < 0.005e-7);
        assert_eq!(h.s_s, 0.0);
        assert!(h.criterion.is_none());
        assert!((h.u_bb_star - 1.481e-4).abs() < 0.001e-4, "{}", h.u_bb_star);
        let mu0 = 0.4250;
        let prior =
            BetweenUnitPrior { tau_scale: 0.0015 * mu0, sigma_scale: 0.0020 * mu0, mu: Some((mu0, 0.025 * mu0)) };
        let p = between_unit_posterior(&items, &prior).unwrap();
        let c = |a: f64, b: f64, t: f64| assert!((a - b).abs() <= t, "{a} vs {b}");
        c(p.tau.mean, 0.0001356, 1e-6);
        c(p.tau.sd, 0.000107, 2e-6);
        c(p.tau.median, 0.0001126, 1e-6);
        c(p.tau.q975, 0.000397, 1e-6);
        c(p.tau.q025, 0.0000052, 0.2e-6);
        c(p.tau.mode, 0.0, 1e-12);
        c(p.tau.hpd.0, 0.0, 1e-12);
        c(p.tau.hpd.1 / 0.42514, 0.0008, 0.00005);
        c(p.sigma.mean, 0.000688, 2e-6);
        c(p.sigma.sd, 0.0000722, 2e-6);
        c(p.sigma.q025, 0.000564, 2e-6);
        c(p.sigma.q975, 0.000846, 2e-6);
        c(p.sigma.hpd.0 / 0.42514, 0.0013, 0.00005);
        c(p.sigma.hpd.1 / 0.42514, 0.0020, 0.00005);
        c(p.mu_mean, 0.42514, 5e-6);
        c(p.mu_sd, 0.00011, 0.000005);
    }
}
