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
    /// Criterion 0.3 σ_pt (B.1).
    pub criterion: f64,
    pub sufficient: bool,
    /// Factors F₁ and F_m and the expanded criterion √c of B.2.3.
    pub f1: f64,
    pub fm: f64,
    pub expanded_criterion: f64,
    pub sufficient_expanded: bool,
}

/// Homogeneity check from g items measured m times each (equal m),
/// against σ_pt [ISO 13528:2022 B.2.2, B.2.3, B.3].
pub fn homogeneity(items: &[Vec<f64>], sigma_pt: f64) -> Result<Homogeneity, UncError> {
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
    if items.iter().flatten().any(|v| !v.is_finite()) || !(sigma_pt > 0.0) {
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
    let criterion = 0.3 * sigma_pt;
    let f1 = chi_squared_quantile(0.95, g as f64 - 1.0) / (g as f64 - 1.0);
    let fm = (f_quantile(0.95, g as f64 - 1.0, (g * (m - 1)) as f64) - 1.0) / m as f64;
    let expanded_criterion = (f1 * criterion * criterion + fm * s_w2).sqrt();
    Ok(Homogeneity {
        items: g,
        replicates: m,
        mean,
        s_x: s_x2.sqrt(),
        s_w: s_w2.sqrt(),
        s_s,
        criterion,
        sufficient: s_s <= criterion,
        f1,
        fm,
        expanded_criterion,
        sufficient_expanded: s_s <= expanded_criterion,
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
        let h = homogeneity(&items, 0.15 * mean).unwrap();
        assert!((h.mean - 0.18715).abs() < 5e-6);
        assert!((h.s_x - 0.00398).abs() < 5e-6 && (h.s_w - 0.00556).abs() < 5e-6 && (h.s_s - 0.00060).abs() < 5e-6);
        assert!((h.criterion - 0.00842).abs() < 5e-6 && h.sufficient);
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
}
