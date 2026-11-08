//! Conformity assessment [JCGM 106:2012]: probability that a measured
//! property lies in its tolerance interval, measurement capability index,
//! acceptance intervals from guard bands or from a required probability, and
//! the global consumer's and producer's risks of a production process
//! inspected by a measuring system.

use crate::error::UncError;
use crate::special::{gamma_p, ln_gamma, normal_cdf, normal_pdf, student_t_cdf};

fn invalid(m: impl Into<String>) -> UncError {
    UncError::InvalidOption(format!("conformity: {}", m.into()))
}

/// Tolerance interval [T_L, T_U]; one of the limits may be absent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tolerance {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

impl Tolerance {
    pub fn new(lower: Option<f64>, upper: Option<f64>) -> Result<Self, UncError> {
        match (lower, upper) {
            (None, None) => Err(invalid("give a lower or an upper tolerance limit")),
            (Some(l), Some(u)) if !(l < u) => Err(invalid("the lower tolerance limit must be below the upper one")),
            _ if lower.is_some_and(|x| !x.is_finite()) || upper.is_some_and(|x| !x.is_finite()) => {
                Err(invalid("tolerance limits must be finite"))
            }
            _ => Ok(Self { lower, upper }),
        }
    }

    /// Tolerance T = T_U − T_L of a two-sided interval.
    pub fn width(&self) -> Option<f64> {
        Some(self.upper? - self.lower?)
    }

    pub fn contains(&self, x: f64) -> bool {
        self.lower.map_or(true, |l| x >= l) && self.upper.map_or(true, |u| x <= u)
    }
}

/// Interval of measured values for which an item is accepted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acceptance {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

impl Acceptance {
    pub fn contains(&self, x: f64) -> bool {
        self.lower.map_or(true, |l| x >= l) && self.upper.map_or(true, |u| x <= u)
    }

    /// Probability that a measured value from N(η, u_m²) is accepted.
    fn probability(&self, eta: f64, um: f64) -> f64 {
        let hi = self.upper.map_or(1.0, |a| normal_cdf((a - eta) / um));
        let lo = self.lower.map_or(0.0, |a| normal_cdf((a - eta) / um));
        (hi - lo).max(0.0)
    }
}

/// Knowledge of the measurand after measurement (JCGM 106 7.2): a normal
/// PDF, or a scaled and shifted t distribution with scale u (JCGM 101 6.4.9).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Knowledge {
    Normal {
        y: f64,
        u: f64,
    },
    StudentT {
        y: f64,
        u: f64,
        nu: f64,
    },
    /// Lognormal PDF with median y and standard deviation `sdlog` of ln Y
    /// (Eurachem/CITAC compliance guide 2021, Annex A, Case 4).
    Lognormal {
        y: f64,
        sdlog: f64,
    },
}

impl Knowledge {
    pub fn estimate(&self) -> f64 {
        match *self {
            Knowledge::Normal { y, .. } | Knowledge::StudentT { y, .. } | Knowledge::Lognormal { y, .. } => y,
        }
    }

    pub fn scale(&self) -> f64 {
        match *self {
            Knowledge::Normal { u, .. } | Knowledge::StudentT { u, .. } => u,
            Knowledge::Lognormal { y, sdlog } => y.abs() * sdlog,
        }
    }

    /// Pr(Y ≤ x).
    pub fn cdf(&self, x: f64) -> f64 {
        match *self {
            Knowledge::Normal { y, u } => normal_cdf((x - y) / u),
            Knowledge::StudentT { y, u, nu } if nu.is_finite() => student_t_cdf((x - y) / u, nu),
            Knowledge::StudentT { y, u, .. } => normal_cdf((x - y) / u),
            // A non-positive median (reached only while searching for a limit)
            // is treated as all probability at zero.
            Knowledge::Lognormal { y, .. } if y <= 0.0 => {
                if x >= 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Knowledge::Lognormal { y, sdlog } => {
                if x <= 0.0 {
                    0.0
                } else {
                    normal_cdf((x / y).ln() / sdlog)
                }
            }
        }
    }

    fn validate(&self) -> Result<(), UncError> {
        let ok = match *self {
            Knowledge::Normal { y, u } => y.is_finite() && u > 0.0 && u.is_finite(),
            Knowledge::StudentT { y, u, nu } => y.is_finite() && u > 0.0 && u.is_finite() && nu > 0.0,
            Knowledge::Lognormal { y, sdlog } => y > 0.0 && y.is_finite() && sdlog > 0.0 && sdlog.is_finite(),
        };
        if ok {
            Ok(())
        } else {
            Err(invalid("finite estimate, positive uncertainty and degrees of freedom required"))
        }
    }
}

/// Conformance probability p_c = Pr(T_L ≤ Y ≤ T_U), JCGM 106 Formulae (6)–(11).
pub fn conformance_probability(k: &Knowledge, t: &Tolerance) -> Result<f64, UncError> {
    k.validate()?;
    let hi = t.upper.map_or(1.0, |x| k.cdf(x));
    let lo = t.lower.map_or(0.0, |x| k.cdf(x));
    Ok((hi - lo).max(0.0))
}

/// Conformance probability from Monte Carlo values of the measurand.
pub fn conformance_probability_from_values(values: &[f64], t: &Tolerance) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().filter(|&&v| t.contains(v)).count() as f64 / values.len() as f64)
}

/// Measurement capability index C_m = T/(4u), JCGM 106 Formula (12).
pub fn capability_index(t: &Tolerance, u: f64) -> Option<f64> {
    t.width().map(|w| w / (4.0 * u))
}

/// Guard-band methods of calibration practice that depend only on the test
/// uncertainty ratio TUR = (T_U − T_L)/2 / U₉₅ with U₉₅ = 2u (ANSI/NCSL Z540.3
/// handbook; Delker & Peguero 2022; implemented as in Sandia's Suncal). The
/// guard-band factor GBF gives the acceptance limits at ± GBF·(T_U − T_L)/2
/// about the centre of the tolerance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurMethod {
    /// Root-sum-square: GBF = √(1 − 1/TUR²).
    Rss,
    /// Dobbert's managed guard band ("method 6"): GBF = 1 − M/TUR with
    /// M = 1.04 − exp(0.38 ln TUR − 0.54); it keeps the global PFA at or below
    /// about 2 % for any in-tolerance probability. GBF exceeds 1 for
    /// TUR > 4.59, where M < 0 (acceptance limits outside the tolerance); the
    /// project layer limits it to 1 unless the decision rule allows more.
    Dobbert,
    /// NCSLI RP-10: GBF = 1.25 − 1/TUR for TUR ≤ 4, otherwise 1.
    Rp10,
    /// Subtracting U₉₅ from the tolerance: GBF = 1 − 1/TUR for TUR ≤ 4,
    /// otherwise 1.
    Test95,
}

pub fn tur_guard_band_factor(method: TurMethod, tur: f64) -> Result<f64, UncError> {
    if !(tur > 0.0) || !tur.is_finite() {
        return Err(UncError::InvalidOption("the test uncertainty ratio must be positive".into()));
    }
    Ok(match method {
        TurMethod::Rss => {
            if tur < 1.0 {
                return Err(UncError::InvalidOption("the RSS guard band needs TUR ≥ 1".into()));
            }
            (1.0 - 1.0 / (tur * tur)).sqrt()
        }
        TurMethod::Dobbert => 1.0 - (1.04 - (0.38 * tur.ln() - 0.54).exp()) / tur,
        TurMethod::Rp10 => {
            if tur <= 4.0 {
                1.25 - 1.0 / tur
            } else {
                1.0
            }
        }
        TurMethod::Test95 => {
            if tur <= 4.0 {
                1.0 - 1.0 / tur
            } else {
                1.0
            }
        }
    })
}

/// Acceptance interval from a guard band w (positive: guarded acceptance,
/// negative: guarded rejection, zero: simple acceptance) applied to the
/// present tolerance limits, JCGM 106 8.3.
pub fn acceptance_from_guard_band(t: &Tolerance, w: f64) -> Acceptance {
    Acceptance { lower: t.lower.map(|l| l + w), upper: t.upper.map(|u| u - w) }
}

/// Decision rules of JCGM 106 clause 8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    SimpleAcceptance,
    GuardedAcceptance,
    GuardedRejection,
}

/// Acceptance limits such that, for a measured value at a limit, the
/// probability of conformity (guarded acceptance) or of non-conformity
/// (guarded rejection) with respect to the corresponding tolerance limit is
/// `p` (JCGM 106 8.3.3, Examples 1 and 2). `knowledge` gives the PDF for the
/// measurand for an estimate y, so that the uncertainty may depend on y.
pub fn acceptance_from_probability(
    t: &Tolerance,
    rule: Rule,
    p: f64,
    knowledge: impl Fn(f64) -> Knowledge,
) -> Result<Acceptance, UncError> {
    if !(p > 0.5 && p < 1.0) {
        return Err(invalid("the probability for the acceptance limits must lie between 0.5 and 1"));
    }
    if rule == Rule::SimpleAcceptance {
        return Ok(acceptance_from_guard_band(t, 0.0));
    }
    // f(y) = probability of the required event for an estimate y at the limit.
    let solve = |limit: f64, upper: bool| -> Result<f64, UncError> {
        let event = |y: f64| {
            let k = knowledge(y);
            let below = k.cdf(limit);
            match (upper, rule) {
                (true, Rule::GuardedAcceptance) | (false, Rule::GuardedRejection) => below,
                _ => 1.0 - below,
            }
        };
        // Search outwards from the limit in the direction of increasing probability.
        let inward = matches!((upper, rule), (true, Rule::GuardedAcceptance) | (false, Rule::GuardedRejection));
        let dir = if inward { -1.0 } else { 1.0 };
        let mut step = knowledge(limit).scale().max(1e-300);
        let mut far = limit + dir * step;
        let mut n = 0;
        while event(far) < p {
            step *= 2.0;
            far = limit + dir * step;
            n += 1;
            if n > 200 || !far.is_finite() {
                return Err(invalid("no acceptance limit reaches the required probability"));
            }
        }
        let (mut a, mut b) = (limit, far);
        for _ in 0..200 {
            let m = 0.5 * (a + b);
            if event(m) < p {
                a = m;
            } else {
                b = m;
            }
        }
        Ok(0.5 * (a + b))
    };
    Ok(Acceptance {
        lower: t.lower.map(|l| solve(l, false)).transpose()?,
        upper: t.upper.map(|u| solve(u, true)).transpose()?,
    })
}

/// Prior (process) PDF of the property of produced items, JCGM 106 9.2 and
/// Annex B.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Process {
    Normal {
        mean: f64,
        sd: f64,
    },
    /// Gamma PDF with shape α = (mean/sd)² and rate λ = mean/sd² (B.3, B.14).
    Gamma {
        mean: f64,
        sd: f64,
    },
    /// Lognormal PDF: ln Y is normal with mean `meanlog` and standard
    /// deviation `sdlog` (JCGM GUM-5:2026 Example 10, Formula 10.1).
    Lognormal {
        meanlog: f64,
        sdlog: f64,
    },
}

impl Process {
    fn pdf(&self, x: f64) -> f64 {
        match *self {
            Process::Normal { mean, sd } => normal_pdf((x - mean) / sd) / sd,
            Process::Gamma { mean, sd } => {
                if x <= 0.0 {
                    return 0.0;
                }
                let (a, l) = ((mean / sd).powi(2), mean / (sd * sd));
                (a * l.ln() + (a - 1.0) * x.ln() - l * x - ln_gamma(a)).exp()
            }
            Process::Lognormal { meanlog, sdlog } => {
                if x <= 0.0 {
                    return 0.0;
                }
                normal_pdf((x.ln() - meanlog) / sdlog) / (sdlog * x)
            }
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        match *self {
            Process::Normal { mean, sd } => normal_cdf((x - mean) / sd),
            Process::Gamma { mean, sd } => {
                if x <= 0.0 {
                    0.0
                } else {
                    gamma_p((mean / sd).powi(2), x * mean / (sd * sd))
                }
            }
            Process::Lognormal { meanlog, sdlog } => {
                if x <= 0.0 {
                    0.0
                } else {
                    normal_cdf((x.ln() - meanlog) / sdlog)
                }
            }
        }
    }

    /// Interval that holds all but a negligible part of the probability.
    fn support(&self) -> (f64, f64) {
        match *self {
            Process::Normal { mean, sd } => (mean - 12.0 * sd, mean + 12.0 * sd),
            Process::Gamma { mean, sd } => (0.0, mean + 40.0 * sd),
            Process::Lognormal { meanlog, sdlog } => (0.0, (meanlog + 12.0 * sdlog).exp()),
        }
    }

    fn validate(&self) -> Result<(), UncError> {
        let (m, s) = match *self {
            Process::Normal { mean, sd } | Process::Gamma { mean, sd } => (mean, sd),
            Process::Lognormal { meanlog, sdlog } => {
                if !(meanlog.is_finite() && sdlog > 0.0 && sdlog.is_finite()) {
                    return Err(invalid("process PDF: finite meanlog and positive sdlog"));
                }
                return Ok(());
            }
        };
        if !(m.is_finite() && s > 0.0 && s.is_finite()) || matches!(self, Process::Gamma { .. }) && m <= 0.0 {
            return Err(invalid("process PDF: finite mean and positive standard deviation (gamma: positive mean)"));
        }
        Ok(())
    }

    /// Probability that a produced item conforms, JCGM 106 Formula (21).
    pub fn conforming(&self, t: &Tolerance) -> f64 {
        t.upper.map_or(1.0, |u| self.cdf(u)) - t.lower.map_or(0.0, |l| self.cdf(l))
    }
}

/// Global risks of a binary decision rule, JCGM 106 9.5.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalRisks {
    /// Probability that an item conforms (before inspection).
    pub conforming: f64,
    /// Probability that an item is accepted.
    pub accepted: f64,
    /// Global consumer's risk R_C: accepted and not conforming (17).
    pub consumer: f64,
    /// Global producer's risk R_P: rejected although conforming (18).
    pub producer: f64,
}

/// Adaptive Simpson step on [a, b] with the function values (fa, fm, fb) at
/// a, the midpoint and b.
fn simpson(
    f: &impl Fn(f64) -> f64,
    (a, b): (f64, f64),
    (fa, fm, fb): (f64, f64, f64),
    whole: f64,
    tol: f64,
    depth: u32,
) -> f64 {
    let m = 0.5 * (a + b);
    let (lm, rm) = (0.5 * (a + m), 0.5 * (m + b));
    let (flm, frm) = (f(lm), f(rm));
    let left = (m - a) / 6.0 * (fa + 4.0 * flm + fm);
    let right = (b - m) / 6.0 * (fm + 4.0 * frm + fb);
    if depth == 0 || (left + right - whole).abs() <= 15.0 * tol {
        left + right + (left + right - whole) / 15.0
    } else {
        simpson(f, (a, m), (fa, flm, fm), left, tol / 2.0, depth - 1)
            + simpson(f, (m, b), (fm, frm, fb), right, tol / 2.0, depth - 1)
    }
}

/// ∫ f over [a, b] split at `breaks`, by adaptive Simpson quadrature.
fn integrate(f: impl Fn(f64) -> f64, a: f64, b: f64, breaks: &[f64]) -> f64 {
    let mut pts = vec![a, b];
    pts.extend(breaks.iter().copied().filter(|x| *x > a && *x < b));
    pts.sort_by(f64::total_cmp);
    pts.windows(2)
        .map(|w| {
            // Further subdivision so that narrow features are not missed.
            let n = 64;
            (0..n)
                .map(|i| {
                    let (x0, x1) =
                        (w[0] + (w[1] - w[0]) * i as f64 / n as f64, w[0] + (w[1] - w[0]) * (i + 1) as f64 / n as f64);
                    let (f0, fm, f1) = (f(x0), f(0.5 * (x0 + x1)), f(x1));
                    let whole = (x1 - x0) / 6.0 * (f0 + 4.0 * fm + f1);
                    simpson(&f, (x0, x1), (f0, fm, f1), whole, 1e-15, 40)
                })
                .sum::<f64>()
        })
        .sum()
}

/// Global consumer's and producer's risks for a process PDF, a normal
/// measuring-system PDF with standard uncertainty u_m and an acceptance
/// interval, JCGM 106 Formulae (17)–(20).
pub fn global_risks(process: &Process, um: f64, t: &Tolerance, a: &Acceptance) -> Result<GlobalRisks, UncError> {
    if !(um > 0.0 && um.is_finite()) {
        return Err(invalid("the standard uncertainty of the measuring system must be positive"));
    }
    global_risks_with(process, |_| um, t, a)
}

/// As [`global_risks`], with a standard uncertainty proportional to the true
/// value, u_m(η) = u_rel·|η|: the measuring-system PDF h(η_m | η) is normal
/// with that standard deviation.
pub fn global_risks_relative(
    process: &Process,
    u_rel: f64,
    t: &Tolerance,
    a: &Acceptance,
) -> Result<GlobalRisks, UncError> {
    if !(u_rel > 0.0 && u_rel.is_finite()) {
        return Err(invalid("the relative standard uncertainty of the measuring system must be positive"));
    }
    global_risks_with(process, |x| u_rel * x.abs(), t, a)
}

fn global_risks_with(
    process: &Process,
    um: impl Fn(f64) -> f64,
    t: &Tolerance,
    a: &Acceptance,
) -> Result<GlobalRisks, UncError> {
    process.validate()?;
    let (lo, hi) = process.support();
    let breaks: Vec<f64> = [t.lower, t.upper, a.lower, a.upper].into_iter().flatten().collect();
    let g = |x: f64| process.pdf(x);
    let pa = |x: f64| {
        let u = um(x);
        if u > 0.0 {
            a.probability(x, u)
        } else if a.contains(x) {
            1.0
        } else {
            0.0
        }
    };
    let accepted = integrate(|x| g(x) * pa(x), lo, hi, &breaks);
    let consumer = integrate(|x| if t.contains(x) { 0.0 } else { g(x) * pa(x) }, lo, hi, &breaks);
    let producer = integrate(|x| if t.contains(x) { g(x) * (1.0 - pa(x)) } else { 0.0 }, lo, hi, &breaks);
    Ok(GlobalRisks { conforming: process.conforming(t), accepted, consumer, producer })
}

/// Conformance probability from the posterior PDF g(η | η_m) ∝ g₀(η)·h(η_m | η)
/// with the process PDF as prior g₀ and a normal likelihood of standard
/// deviation u (JCGM 106 6.2, Annex A; JCGM GUM-5:2026 10.6.4).
pub fn posterior_conformance_probability(process: &Process, y: f64, u: f64, t: &Tolerance) -> Result<f64, UncError> {
    process.validate()?;
    if !(y.is_finite() && u > 0.0 && u.is_finite()) {
        return Err(invalid("finite measured value and positive uncertainty required"));
    }
    let (lo, hi) = process.support();
    let (lo, hi) = (lo.max(y - 40.0 * u), hi.min(y + 40.0 * u));
    if !(lo < hi) {
        return Err(invalid("the measured value lies outside the support of the process PDF"));
    }
    let mut breaks: Vec<f64> = [t.lower, t.upper].into_iter().flatten().collect();
    breaks.extend((-8..=8).map(|k| y + k as f64 * u));
    let post = |x: f64| process.pdf(x) * normal_pdf((y - x) / u);
    let total = integrate(post, lo, hi, &breaks);
    if !(total > 0.0) {
        return Err(invalid("the prior and the likelihood do not overlap"));
    }
    let inside = integrate(|x| if t.contains(x) { post(x) } else { 0.0 }, lo, hi, &breaks);
    Ok((inside / total).clamp(0.0, 1.0))
}

/// Guard-band multiplier r (w = rU, U = 2u_m, applied to every tolerance
/// limit) for which the global consumer's risk equals `target`
/// (JCGM 106 9.5.4).
pub fn guard_band_for_consumer_risk(process: &Process, um: f64, t: &Tolerance, target: f64) -> Result<f64, UncError> {
    if !(target > 0.0 && target < 1.0) {
        return Err(invalid("the target consumer's risk must lie between 0 and 1"));
    }
    let rc = |r: f64| global_risks(process, um, t, &acceptance_from_guard_band(t, 2.0 * r * um)).map(|g| g.consumer);
    let (mut a, mut b) = (-10.0, 10.0);
    if rc(a)? < target || rc(b)? > target {
        return Err(invalid("the target consumer's risk cannot be reached with |r| ≤ 10"));
    }
    for _ in 0..100 {
        let m = 0.5 * (a + b);
        if rc(m)? > target {
            a = m;
        } else {
            b = m;
        }
    }
    Ok(0.5 * (a + b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tur_guard_band_factors_match_suncal() {
        // Sandia Suncal 1.7.1, suncal.risk.guardband_tur, at TUR = 1.5, 2, 3, 4, 6.
        let cases = [
            (1.5, [0.7453559925, 0.759881796, 0.5833333333, 0.3333333333]),
            (2.0, [0.8660254038, 0.859177346, 0.75, 0.5]),
            (3.0, [0.9428090416, 0.948227242, 0.9166666667, 0.6666666667]),
            (4.0, [0.9682458366, 0.9867196755, 1.0, 0.75]),
            (6.0, [0.9860132972, 1.0185455598, 1.0, 1.0]),
        ];
        for (tur, want) in cases {
            for (m, w) in [TurMethod::Rss, TurMethod::Dobbert, TurMethod::Rp10, TurMethod::Test95].iter().zip(want) {
                let g = tur_guard_band_factor(*m, tur).unwrap();
                assert!((g - w).abs() < 1e-9, "{m:?} at {tur}: {g} vs {w}");
            }
        }
        assert!(tur_guard_band_factor(TurMethod::Rss, 0.5).is_err());
    }

    fn close(a: f64, b: f64, t: f64) -> bool {
        (a - b).abs() <= t
    }

    #[test]
    fn conformance_probability_examples_of_clause_7() {
        // 7.3 Example 1: Zener diode, V_b ≤ −5.40 V, v_b = −5.47 V, u = 0.05 V → 0.92.
        let t = Tolerance::new(None, Some(-5.40)).unwrap();
        let p = conformance_probability(&Knowledge::Normal { y: -5.47, u: 0.05 }, &t).unwrap();
        assert!(close(p, 0.919, 0.001), "{p}");
        // Example 2: bursting strength B ≥ 490 kPa, b = 509.7, u = 8.6 → 0.99.
        let t = Tolerance::new(Some(490.0), None).unwrap();
        let p = conformance_probability(&Knowledge::Normal { y: 509.7, u: 8.6 }, &t).unwrap();
        assert!(close(p, 0.989, 0.001), "{p}");
        // 7.4 Example: motor oil 12.5 … 16.3 mm²/s, y = 13.6, u = 1.8 → 0.66 (0.93 − 0.27).
        let t = Tolerance::new(Some(12.5), Some(16.3)).unwrap();
        let p = conformance_probability(&Knowledge::Normal { y: 13.6, u: 1.8 }, &t).unwrap();
        assert!(close(p, 0.66, 0.005), "{p}");
        // 7.7.5: C_m = 1 gives p_c ≥ 95 % only for 0.45 ≤ ỹ ≤ 0.55.
        let t = Tolerance::new(Some(0.0), Some(1.0)).unwrap();
        assert_eq!(capability_index(&t, 0.25), Some(1.0));
        // The boundaries are ỹ = 0.4491 and 0.5509.
        let pc = |y| conformance_probability(&Knowledge::Normal { y, u: 0.25 }, &t).unwrap();
        assert!(pc(0.448) < 0.95 && pc(0.45) > 0.95 && pc(0.55) > 0.95 && pc(0.552) < 0.95);
    }

    #[test]
    fn guard_bands_of_clause_8() {
        // 8.3.2 (ISO 14253-1): w = U = 2u limits the false-acceptance probability to 2.3 %.
        let t = Tolerance::new(Some(0.0), Some(10.0)).unwrap();
        let a = acceptance_from_guard_band(&t, 2.0 * 0.5);
        let p = conformance_probability(&Knowledge::Normal { y: a.upper.unwrap(), u: 0.5 }, &t).unwrap();
        assert!(close(1.0 - p, 0.0228, 0.0001));
        // 8.3.3 Example 1: speed limit 100 km/h, u = 2 % of v, 99.9 % → v_max ≈ 107 km/h.
        let t = Tolerance::new(None, Some(100.0)).unwrap();
        let a =
            acceptance_from_probability(&t, Rule::GuardedRejection, 0.999, |v| Knowledge::Normal { y: v, u: 0.02 * v })
                .unwrap();
        let v = a.upper.unwrap();
        assert!(close(v, 100.0 / (1.0 - 0.02 * 3.0902), 0.01) && v.round() == 107.0, "{v}");
        // Example 2: nandrolone, T = 2.00 µg/L, s = 0.20, t with ν = 9, 95 % → A = 2.37 µg/L.
        let t = Tolerance::new(None, Some(2.0)).unwrap();
        let a = acceptance_from_probability(&t, Rule::GuardedRejection, 0.95, |y| Knowledge::StudentT {
            y,
            u: 0.2,
            nu: 9.0,
        })
        .unwrap();
        assert!(close(a.upper.unwrap(), 2.0 + 1.8331 * 0.2, 0.0005), "{:?}", a);
        // Guarded acceptance at the same probability lies symmetrically inside.
        let g = acceptance_from_probability(&t, Rule::GuardedAcceptance, 0.95, |y| Knowledge::StudentT {
            y,
            u: 0.2,
            nu: 9.0,
        })
        .unwrap();
        assert!(close(g.upper.unwrap(), 2.0 - 1.8331 * 0.2, 0.0005));
    }

    #[test]
    fn global_risks_of_clause_9() {
        // 9.5.3 Example: precision resistors, T = 1499.8 … 1500.2 Ω, process N(1500, 0.12²),
        // u_m = 0.04 Ω, acceptance 1499.82 … 1500.18 Ω: p_c ≈ 90 %, R_C ≈ 1 %, R_P ≈ 7 %.
        let t = Tolerance::new(Some(1499.8), Some(1500.2)).unwrap();
        let a = Acceptance { lower: Some(1499.82), upper: Some(1500.18) };
        let r = global_risks(&Process::Normal { mean: 1500.0, sd: 0.12 }, 0.04, &t, &a).unwrap();
        assert!(
            close(r.conforming, 0.9044, 0.0001)
                && close(r.consumer, 0.00988, 0.00005)
                && close(r.producer, 0.0690, 0.0001),
            "{r:?}"
        );
        // Of every 100 resistors 83 conforming and 1 non-conforming are accepted
        // (the text's 84 adds the rounded counts; the probability is 0.845).
        assert!(close(r.accepted - r.consumer, 0.835, 0.001) && close(r.accepted, 0.845, 0.001), "{r:?}");
        assert!(close(r.accepted, r.conforming - r.producer + r.consumer, 1e-9));
        // 9.5.4 Example: ball bearings, Y ≤ 2 µm, gamma prior (mean 1, sd 0.5 µm), u_m = 0.25 µm:
        // 4.2 % non-conforming, R_C = 0.1 % for r ≈ 0.65, A ≈ 1.7 µm, R_P ≈ 7.5 %.
        let t = Tolerance::new(None, Some(2.0)).unwrap();
        let p = Process::Gamma { mean: 1.0, sd: 0.5 };
        assert!(close(1.0 - p.conforming(&t), 0.04238, 0.00001));
        let r = guard_band_for_consumer_risk(&p, 0.25, &t, 0.001).unwrap();
        assert!(close(r, 0.656, 0.001), "{r}");
        let g = global_risks(&p, 0.25, &t, &acceptance_from_guard_band(&t, 2.0 * r * 0.25)).unwrap();
        assert!(close(g.consumer, 0.001, 1e-6) && close(g.producer, 0.0755, 0.0005), "{g:?}");
        // 9.5.6: u_0 = T/6, centred process, w = 0: C_m = 2 → R_C ≈ 0.1 %, R_P ≈ 1.5 %;
        // C_m = 10 → R_C ≈ 0.04 %, R_P ≈ 0.07 %.
        let t = Tolerance::new(Some(0.0), Some(1.0)).unwrap();
        let p = Process::Normal { mean: 0.5, sd: 1.0 / 6.0 };
        let a = acceptance_from_guard_band(&t, 0.0);
        let g2 = global_risks(&p, 1.0 / 8.0, &t, &a).unwrap();
        let g10 = global_risks(&p, 1.0 / 40.0, &t, &a).unwrap();
        assert!(close(g2.consumer, 0.000982, 0.000002) && close(g2.producer, 0.01468, 0.00002), "{g2:?}");
        assert!(close(g10.consumer, 0.000408, 0.000002) && close(g10.producer, 0.000717, 0.000002), "{g10:?}");
    }

    /// JCGM GUM-5:2026 Example 10 (suspended particulate matter, T_U =
    /// 0.2 mg/m³): lognormal prior with µ = −2.325 and σ = 0.434 of ln ρ,
    /// relative measurement uncertainty 7 %. Values recomputed by quadrature
    /// (scipy): global risks with u ∝ true value and specific risks with
    /// u = 0.07·ρ̂, the likelihood of GUM-5 10.6.4.
    #[test]
    fn gum5_example10_lognormal_prior() {
        let p = Process::Lognormal { meanlog: -2.325, sdlog: 0.434 };
        let t = Tolerance::new(None, Some(0.2)).unwrap();
        // Quantiles of Figure 10.1: 0.042 and 0.229 mg/m³.
        assert!(close(p.cdf(0.04177), 0.025, 1e-4) && close(p.cdf(0.22892), 0.975, 1e-4));
        let g = global_risks_relative(&p, 0.07, &t, &Acceptance { lower: None, upper: Some(0.2) }).unwrap();
        assert!(close(g.consumer, 0.005790, 2e-6), "{}", g.consumer);
        assert!(close(g.producer, 0.007392, 2e-6), "{}", g.producer);
        let g = global_risks_relative(&p, 0.07, &t, &Acceptance { lower: None, upper: Some(0.17) }).unwrap();
        assert!(close(g.consumer, 0.000103, 1e-6) && close(g.producer, 0.053702, 2e-6), "{g:?}");
        // Specific producer's risk for ρ̂ = 0.225 (rejected): p_c = 11.3 % (GUM-5: about 11 %).
        let pc = posterior_conformance_probability(&p, 0.225, 0.07 * 0.225, &t).unwrap();
        assert!(close(pc, 0.113340, 2e-6), "{pc}");
        // Specific consumer's risk for ρ̂ = 0.190 (accepted).
        let pc = posterior_conformance_probability(&p, 0.19, 0.07 * 0.19, &t).unwrap();
        assert!(close(1.0 - pc, 0.142616, 2e-6), "{pc}");
        // A flat prior over the likelihood gives the normal p_c back.
        let wide = Process::Normal { mean: 0.2, sd: 100.0 };
        let pc = posterior_conformance_probability(&wide, 0.225, 0.01575, &t).unwrap();
        let k = conformance_probability(&Knowledge::Normal { y: 0.225, u: 0.01575 }, &t).unwrap();
        assert!(close(pc, k, 1e-6), "{pc} {k}");
    }

    /// Eurachem/CITAC, Use of uncertainty information in compliance
    /// assessment (2021), Annex A Table 1 and Annex B Example 3: acceptance
    /// limits L·exp(±k·u_rel) for a lognormal PDF with s_G = u_rel.
    #[test]
    fn eurachem_compliance_lognormal_acceptance_limits() {
        let t = Tolerance::new(None, Some(100.0)).unwrap();
        for (ur, acc, rej) in [(0.3, 61.0, 164.0), (0.5, 44.0, 227.0)] {
            let kn = |y: f64| Knowledge::Lognormal { y, sdlog: ur };
            let a = acceptance_from_probability(&t, Rule::GuardedAcceptance, 0.95, kn).unwrap().upper.unwrap();
            let r = acceptance_from_probability(&t, Rule::GuardedRejection, 0.95, kn).unwrap().upper.unwrap();
            // Exact k = 1.6449; the guide uses 1.64 (227.6 rounds to 228 with the exact k).
            assert!(close(a, 100.0 * (-1.6448536 * ur).exp(), 1e-6) && close(a, acc, 0.6), "{a}");
            assert!(close(r, 100.0 * (1.6448536 * ur).exp(), 1e-5) && close(r, rej, 0.7), "{r}");
        }
        // Example 3: L = 2 ng/g, u_rel = 35 %: acceptance limit for correct rejection 3.56 (printed 3.6).
        let t = Tolerance::new(None, Some(2.0)).unwrap();
        let r =
            acceptance_from_probability(&t, Rule::GuardedRejection, 0.95, |y| Knowledge::Lognormal { y, sdlog: 0.35 })
                .unwrap()
                .upper
                .unwrap();
        assert!(close(r, 3.557, 0.001), "{r}");
    }
}
