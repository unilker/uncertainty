//! Internal quality control with control charts — Nordtest TR 569 ed. 6
//! (2026), *Handbook of Internal Quality Control* ("Trollbook").
//!
//! * X-charts (single values, means or blanks), R-charts (ranges of n
//!   replicates) and r%-charts (ranges relative to the mean) (Chapter 5);
//! * statistical and target control limits, a mean or reference central line,
//!   and charts with action limits only (Chapter 7, Table 4);
//! * the daily interpretation after each run — action limit and "two out of
//!   three" — and the rules for being out of *statistical* control (Chapter 9);
//! * the long-term review: count outside the warning limits, the 0.37 s
//!   check, F- and t-tests (Chapter 10, Equations 5–7, 9);
//! * pooling of standard deviations (Equations 8–10) and the one-way analysis
//!   of variance of Example 12 for s_r and s_Rw.
//!
//! The range factors are those of ISO 8258 (d₂, d₃, D₂ = d₂ + 3d₃); the
//! warning-limit factor D_WL = d₂ + (2/3)(D_AL − d₂) was developed for
//! TR 569 (Table 4).

use crate::error::UncError;
use crate::special::{f_quantile, student_t_quantile};

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
// Range factors (Table 4)
// ---------------------------------------------------------------------------

/// d₂ = E(R)/σ and d₃ = sd(R)/σ for the range R of n = 2 … 10 normal values,
/// computed by numerical integration (they agree with ISO 8258 Table 2 to
/// the three decimals printed there; the test below recomputes them). For
/// n = 2, d₂ = 2/√π exactly.
const D2: [f64; 9] =
    [std::f64::consts::FRAC_2_SQRT_PI, 1.692569, 2.058751, 2.325929, 2.534413, 2.704357, 2.847201, 2.970026, 3.077505];
const D3: [f64; 9] = [0.852502, 0.888368, 0.879808, 0.864082, 0.848040, 0.833205, 0.819831, 0.807834, 0.797051];

/// Factors for a range chart of n replicates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeFactors {
    pub n: usize,
    /// s = mean range / d₂ (Equation 4 for n = 2); the expected range is d₂σ.
    pub d2: f64,
    pub d3: f64,
    /// Upper warning limit D_WL·s, D_WL = d₂ + (2/3)(D_AL − d₂).
    pub warning: f64,
    /// Upper action limit D_AL·s, D_AL = d₂ + 3d₃ (ISO 8258 D₂).
    pub action: f64,
}

/// Range factors for n = 2 … 10 replicates.
pub fn range_factors(n: usize) -> Result<RangeFactors, UncError> {
    if !(2..=10).contains(&n) {
        return Err(invalid("range charts need 2 to 10 replicates"));
    }
    let (d2, d3) = (D2[n - 2], D3[n - 2]);
    let action = d2 + 3.0 * d3;
    Ok(RangeFactors { n, d2, d3, warning: d2 + 2.0 / 3.0 * (action - d2), action })
}

// ---------------------------------------------------------------------------
// Control values
// ---------------------------------------------------------------------------

/// The type of control chart (Chapter 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartKind {
    /// X-chart: single control values or means of replicates.
    X,
    /// R-chart: range (largest − smallest) of n replicates.
    Range,
    /// r%-chart: range as per cent of the mean of the replicates.
    RelativeRange,
}

impl ChartKind {
    /// Whether the chart has only upper limits (ranges are never negative).
    pub fn one_sided(self) -> bool {
        !matches!(self, ChartKind::X)
    }
}

/// The control value of one run from its replicate results: the mean (X),
/// the range (R) or the range in per cent of the mean (r%).
pub fn control_value(kind: ChartKind, run: &[f64]) -> Result<f64, UncError> {
    if run.is_empty() {
        return Err(invalid("a run has no results"));
    }
    check_finite(run, "replicates")?;
    let mean = run.iter().sum::<f64>() / run.len() as f64;
    let range =
        run.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - run.iter().cloned().fold(f64::INFINITY, f64::min);
    match kind {
        ChartKind::X => Ok(mean),
        _ if run.len() < 2 => Err(invalid("a range needs at least two replicates")),
        ChartKind::Range => Ok(range),
        ChartKind::RelativeRange => {
            if mean == 0.0 {
                Err(invalid("relative range of replicates with mean zero"))
            } else {
                Ok(100.0 * range / mean.abs())
            }
        }
    }
}

/// Mean x̄, standard deviation s (Equations 1, 2) and number of values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Summary {
    pub mean: f64,
    pub sd: f64,
    pub n: usize,
    /// Degrees of freedom of `sd` (n − 1 for Equation 2).
    pub dof: f64,
}

/// Mean and standard deviation of a series of values (Equations 1 and 2).
pub fn summary(values: &[f64]) -> Result<Summary, UncError> {
    if values.len() < 2 {
        return Err(invalid("at least two values are needed"));
    }
    check_finite(values, "values")?;
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let ss: f64 = values.iter().map(|x| (x - mean).powi(2)).sum();
    Ok(Summary { mean, sd: (ss / (n - 1.0)).sqrt(), n: values.len(), dof: n - 1.0 })
}

// ---------------------------------------------------------------------------
// Control limits (Chapter 7)
// ---------------------------------------------------------------------------

/// Central line and control limits of a chart. Range charts have upper
/// limits only; charts with action limits only have no warning limits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlLimits {
    pub center: f64,
    pub lower_warning: Option<f64>,
    pub upper_warning: Option<f64>,
    pub lower_action: Option<f64>,
    pub upper_action: f64,
}

impl ControlLimits {
    /// X-chart: warning limits CL ± 2s and action limits CL ± 3s, with s the
    /// standard deviation of the control values (statistical limits) or the
    /// target s_target (target limits).
    pub fn x(center: f64, s: f64) -> Result<Self, UncError> {
        positive(s, "standard deviation")?;
        Ok(ControlLimits {
            center,
            lower_warning: Some(center - 2.0 * s),
            upper_warning: Some(center + 2.0 * s),
            lower_action: Some(center - 3.0 * s),
            upper_action: center + 3.0 * s,
        })
    }

    /// X-chart with action limits only, CL ± h: h = k·s with k = 3 "or
    /// lower" (2.5 gives about the false-alarm rate of a chart with both
    /// limits, footnote 9), or h = the tolerance (target limits).
    pub fn x_action_only(center: f64, half_width: f64) -> Result<Self, UncError> {
        positive(half_width, "action limit")?;
        Ok(ControlLimits {
            center,
            lower_warning: None,
            upper_warning: None,
            lower_action: Some(center - half_width),
            upper_action: center + half_width,
        })
    }

    /// R- or r%-chart of n replicates: upper warning limit D_WL·s and upper
    /// action limit D_AL·s (Table 4). The central line is the mean range.
    pub fn range(n: usize, s: f64, center: f64) -> Result<Self, UncError> {
        positive(s, "standard deviation")?;
        let f = range_factors(n)?;
        Ok(ControlLimits {
            center,
            lower_warning: None,
            upper_warning: Some(f.warning * s),
            lower_action: None,
            upper_action: f.action * s,
        })
    }

    /// Range chart with an upper action limit only (D_AL·s "or lower", 3.5 s
    /// for duplicates (footnote 11), or the tolerance).
    pub fn range_action_only(center: f64, upper: f64) -> Result<Self, UncError> {
        positive(upper, "action limit")?;
        Ok(ControlLimits { center, lower_warning: None, upper_warning: None, lower_action: None, upper_action: upper })
    }

    pub fn has_warning_limits(&self) -> bool {
        self.upper_warning.is_some()
    }

    /// Where a control value lies: inside the warning limits (0), between
    /// the warning and action limits (±1) or outside the action limits
    /// (±2), with the sign of the side of the central line. A value on a
    /// limit is inside it.
    pub fn zone(&self, x: f64) -> i8 {
        if x > self.upper_action {
            return 2;
        }
        if self.lower_action.is_some_and(|l| x < l) {
            return -2;
        }
        if self.upper_warning.is_some_and(|w| x > w) {
            return 1;
        }
        if self.lower_warning.is_some_and(|w| x < w) {
            return -1;
        }
        0
    }
}

fn positive(x: f64, what: &str) -> Result<(), UncError> {
    if x.is_finite() && x > 0.0 {
        Ok(())
    } else {
        Err(invalid(format!("{what} must be positive")))
    }
}

/// Target standard deviation from a required relative expanded uncertainty
/// U (k = 2): s_Rw = u_c/2 = U/4, "half of the standard uncertainty"
/// (Chapter 7 and Example 1).
pub fn target_sd_from_expanded(u_expanded: f64) -> f64 {
    u_expanded / 4.0
}

/// Repeatability standard deviation from a repeatability limit r:
/// s_r = r/2.8 (Chapter 7, case 2 for range charts; 2.8 ≈ 2√2).
pub fn sd_from_repeatability_limit(r: f64) -> f64 {
    r / 2.8
}

// ---------------------------------------------------------------------------
// Daily interpretation (Chapter 9)
// ---------------------------------------------------------------------------

/// The outcome of the daily interpretation of a control value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Within the warning limits (or the action limits of a chart without
    /// warning limits): the results can be reported.
    InControl,
    /// Between the warning and action limits, the two previous values not on
    /// the same side outside the warning limits: in control.
    Warning,
    /// Outside the action limits: out of control.
    OutsideAction,
    /// Between the warning and action limits and at least one of the two
    /// previous values outside the warning limit on the same side: out of
    /// control ("two out of three").
    TwoOfThree,
}

impl Status {
    pub fn out_of_control(self) -> bool {
        matches!(self, Status::OutsideAction | Status::TwoOfThree)
    }
}

/// Daily interpretation of a series of control values in time order.
///
/// For "two out of three", a previous value counts if it lies outside the
/// warning limit on the same side, including beyond the action limit, as in
/// the corresponding test of ISO 7870-2 (research log D43).
pub fn daily(values: &[f64], limits: &ControlLimits) -> Vec<Status> {
    let zones: Vec<i8> = values.iter().map(|&x| limits.zone(x)).collect();
    (0..values.len())
        .map(|i| {
            let z = zones[i];
            if z.abs() == 2 {
                Status::OutsideAction
            } else if z == 0 {
                Status::InControl
            } else if zones[i.saturating_sub(2)..i].iter().any(|&p| p.signum() == z.signum() && p != 0) {
                Status::TwoOfThree
            } else {
                Status::Warning
            }
        })
        .collect()
}

/// Signals that the method is out of *statistical* control, though not out
/// of control (Chapter 9, rules from ISO 8258).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StatisticalSignals {
    /// The value completes seven consecutive values that steadily increase
    /// or decrease.
    pub trend: bool,
    /// The value completes ten out of eleven consecutive values on the same
    /// side of the central line.
    pub run: bool,
}

/// Rules for being out of statistical control, evaluated at each value.
/// Values equal to the central line are on neither side.
pub fn statistical_signals(values: &[f64], center: f64) -> Vec<StatisticalSignals> {
    let n = values.len();
    let mut out = vec![StatisticalSignals::default(); n];
    let (mut up, mut down) = (1usize, 1usize);
    for i in 0..n {
        if i > 0 {
            up = if values[i] > values[i - 1] { up + 1 } else { 1 };
            down = if values[i] < values[i - 1] { down + 1 } else { 1 };
        }
        out[i].trend = up >= 7 || down >= 7;
        if i >= 10 {
            let w = &values[i - 10..=i];
            let above = w.iter().filter(|&&x| x > center).count();
            let below = w.iter().filter(|&&x| x < center).count();
            out[i].run = above >= 10 || below >= 10;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Long-term review (Chapter 10)
// ---------------------------------------------------------------------------

/// The comparison of the current control values with the values the limits
/// were based on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Review {
    /// F = s₁²/s₂² with s₁ > s₂ (Equation 5), its degrees of freedom and the
    /// two-sided critical value F₁₋α/₂(ν₁, ν₂) (Table 3).
    pub f: f64,
    pub f_dof: (f64, f64),
    pub f_critical: f64,
    /// Whether the current standard deviation is the larger one.
    pub sd_increased: bool,
    /// Pooled standard deviation of the two series (Equation 9) and its
    /// degrees of freedom n₁ + n₂ − 2.
    pub pooled_sd: f64,
    pub pooled_dof: f64,
    /// t = |x̄₁ − x̄₂|/s_p · √(n₁n₂/(n₁ + n₂)) (Equation 7) and the two-sided
    /// critical value (Table 2).
    pub t: f64,
    pub t_critical: f64,
    /// |x̄₁ − x̄₂| in units of the previous s; with about 60 values in each
    /// series a shift below 0.37 s needs no action (Chapter 10).
    pub mean_shift: f64,
}

impl Review {
    pub fn sd_significant(&self) -> bool {
        self.f > self.f_critical
    }
    pub fn mean_significant(&self) -> bool {
        self.t > self.t_critical
    }
}

/// Reviews the current series against the previous one at the given
/// confidence level (0.95 in TR 569).
pub fn review(previous: &Summary, current: &Summary, confidence: f64) -> Result<Review, UncError> {
    if !(confidence > 0.0 && confidence < 1.0) {
        return Err(invalid("confidence level must be between 0 and 1"));
    }
    for s in [previous, current] {
        positive(s.sd, "standard deviation")?;
        if s.n < 2 || !(s.dof > 0.0) {
            return Err(invalid("each series needs at least two values"));
        }
    }
    let alpha = 1.0 - confidence;
    let sd_increased = current.sd > previous.sd;
    let (big, small) = if sd_increased { (current, previous) } else { (previous, current) };
    let f = (big.sd / small.sd).powi(2);
    let f_dof = (big.dof, small.dof);
    let f_critical = f_quantile(1.0 - alpha / 2.0, f_dof.0, f_dof.1);
    let (n1, n2) = (previous.n as f64, current.n as f64);
    let pooled_dof = n1 + n2 - 2.0;
    let pooled_sd = (((n1 - 1.0) * previous.sd.powi(2) + (n2 - 1.0) * current.sd.powi(2)) / pooled_dof).sqrt();
    let diff = (previous.mean - current.mean).abs();
    let t = diff / pooled_sd * (n1 * n2 / (n1 + n2)).sqrt();
    let t_critical = student_t_quantile(1.0 - alpha / 2.0, pooled_dof);
    Ok(Review {
        f,
        f_dof,
        f_critical,
        sd_increased,
        pooled_sd,
        pooled_dof,
        t,
        t_critical,
        mean_shift: diff / previous.sd,
    })
}

/// t = |x̄ − T|/s · √n (Equation 6) for the mean of n values against a
/// reference value T, with ν = n − 1, and its two-sided critical value.
pub fn t_against_reference(s: &Summary, reference: f64, confidence: f64) -> (f64, f64) {
    let t = (s.mean - reference).abs() / s.sd * (s.n as f64).sqrt();
    (t, student_t_quantile(1.0 - (1.0 - confidence) / 2.0, s.dof))
}

/// Indices of the values more than k standard deviations from the central
/// line, which TR 569 excludes when new limits are calculated (k = 4,
/// Chapter 10).
pub fn beyond(values: &[f64], center: f64, s: f64, k: f64) -> Vec<usize> {
    values.iter().enumerate().filter(|(_, &x)| (x - center).abs() > k * s).map(|(i, _)| i).collect()
}

// ---------------------------------------------------------------------------
// Pooling (Equations 8–10, Example 12)
// ---------------------------------------------------------------------------

/// Weighted mean Σn_i x̄_i / n_tot of k series (Equation 8).
pub fn weighted_mean(series: &[Summary]) -> f64 {
    let n: usize = series.iter().map(|s| s.n).sum();
    series.iter().map(|s| s.n as f64 * s.mean).sum::<f64>() / n as f64
}

/// Pooled standard deviation √(Σ(n_i − 1)s_i²/(n_tot − k)) with
/// ν = n_tot − k (Equation 9).
pub fn pooled_sd(series: &[Summary]) -> Result<(f64, f64), UncError> {
    let dof: f64 = series.iter().map(|s| s.n as f64 - 1.0).sum();
    if series.is_empty() || !(dof > 0.0) {
        return Err(invalid("no standard deviations to pool"));
    }
    let ss: f64 = series.iter().map(|s| (s.n as f64 - 1.0) * s.sd * s.sd).sum();
    Ok(((ss / dof).sqrt(), dof))
}

/// Root mean square √(Σs_i²/k) of k standard deviations from series of
/// about the same size (Equation 10).
pub fn pooled_sd_equal(sds: &[f64]) -> f64 {
    (sds.iter().map(|s| s * s).sum::<f64>() / sds.len() as f64).sqrt()
}

/// One-way analysis of variance of control results with replicates in each
/// run (Example 12, footnote 24).
#[derive(Clone, Debug, PartialEq)]
pub struct RunAnova {
    pub runs: usize,
    pub results: usize,
    pub mean: f64,
    /// Within-run (repeatability) standard deviation s_r, the square root of
    /// the within-run mean square, with ν = N − k.
    pub s_r: f64,
    pub dof_r: f64,
    /// Between-run standard deviation s_I = √((MS_between − MS_within)/n₀),
    /// zero when MS_between < MS_within.
    pub s_between: f64,
    pub ms_between: f64,
    pub ms_within: f64,
    /// n₀ = (N − Σn_i²/N)/(k − 1), the number of replicates for a balanced
    /// design.
    pub n0: f64,
    /// s_Rw = √(s_r² + s_I²).
    pub s_rw: f64,
}

/// One-way ANOVA of runs of replicate results.
pub fn run_anova(runs: &[Vec<f64>]) -> Result<RunAnova, UncError> {
    let k = runs.len();
    if k < 2 {
        return Err(invalid("the analysis of variance needs at least two runs"));
    }
    let all: Vec<f64> = runs.iter().flatten().cloned().collect();
    check_finite(&all, "results")?;
    let total = all.len();
    if runs.iter().any(|r| r.is_empty()) || total <= k {
        return Err(invalid("every run needs results, and some run needs replicates"));
    }
    let mean = all.iter().sum::<f64>() / total as f64;
    let (mut ssb, mut ssw) = (0.0, 0.0);
    for r in runs {
        let m = r.iter().sum::<f64>() / r.len() as f64;
        ssb += r.len() as f64 * (m - mean).powi(2);
        ssw += r.iter().map(|x| (x - m).powi(2)).sum::<f64>();
    }
    let (nt, kf) = (total as f64, k as f64);
    let ms_between = ssb / (kf - 1.0);
    let ms_within = ssw / (nt - kf);
    let n0 = (nt - runs.iter().map(|r| (r.len() as f64).powi(2)).sum::<f64>() / nt) / (kf - 1.0);
    let var_between = ((ms_between - ms_within) / n0).max(0.0);
    Ok(RunAnova {
        runs: k,
        results: total,
        mean,
        s_r: ms_within.sqrt(),
        dof_r: nt - kf,
        s_between: var_between.sqrt(),
        ms_between,
        ms_within,
        n0,
        s_rw: (ms_within + var_between).sqrt(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::special::{normal_cdf, normal_pdf};

    fn close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    /// d₂ = ∫(1 − Φⁿ − (1 − Φ)ⁿ)dx and E(R²) = ∫2r(1 − F_R(r))dr with
    /// F_R(r) = n∫φ(x)(Φ(x + r) − Φ(x))ⁿ⁻¹dx, by the trapezoidal rule.
    #[test]
    fn range_constants_by_integration() {
        let h = 0.02;
        let xs: Vec<f64> = (0..=800).map(|i| -8.0 + i as f64 * h).collect();
        for n in 2..=10 {
            let nf = n as i32;
            let d2: f64 =
                xs.iter().map(|&x| 1.0 - normal_cdf(x).powi(nf) - (1.0 - normal_cdf(x)).powi(nf)).sum::<f64>() * h;
            let mut er2 = 0.0;
            for j in 1..=600 {
                let r = j as f64 * h;
                let fr: f64 =
                    xs.iter().map(|&x| normal_pdf(x) * (normal_cdf(x + r) - normal_cdf(x)).powi(nf - 1)).sum::<f64>()
                        * h
                        * n as f64;
                er2 += 2.0 * r * (1.0 - fr) * h;
            }
            let f = range_factors(n).unwrap();
            close(f.d2, d2, 2e-6);
            close(f.d3, (er2 - d2 * d2).sqrt(), 1e-4);
        }
        // Table 4: D_WL and D_AL for duplicates, triplicates, quadruplicates.
        for (n, wl, al) in [(2, 2.833, 3.686), (3, 3.470, 4.358), (4, 3.818, 4.698)] {
            let f = range_factors(n).unwrap();
            close(f.warning, wl, 1.1e-3);
            close(f.action, al, 1e-3);
        }
        close(range_factors(2).unwrap().d2, 1.128, 5e-4);
        assert!(range_factors(1).is_err() && range_factors(11).is_err());
    }

    #[test]
    fn x_chart_limits_examples() {
        // Example 1: target s_Rw = 4 %/4 = 1 % of 4.58 weight-%.
        let s = target_sd_from_expanded(0.04) * 4.58;
        close(s, 0.0458, 1e-12);
        let l = ControlLimits::x(4.58, s).unwrap();
        close(l.lower_warning.unwrap(), 4.49, 0.005);
        close(l.upper_warning.unwrap(), 4.67, 0.005);
        close(l.lower_action.unwrap(), 4.44, 0.005);
        close(l.upper_action, 4.72, 0.005);
        // Example 9: blank chart with negative lower limits.
        let l = ControlLimits::x(0.039, 0.045).unwrap();
        close(l.lower_warning.unwrap(), -0.051, 1e-12);
        close(l.lower_action.unwrap(), -0.096, 1e-12);
        // Chapter 7, case 1: s_Rw = 4 % of 59.2 µg/L.
        let l = ControlLimits::x(59.2, 0.04 * 59.2).unwrap();
        close(l.lower_warning.unwrap(), 54.5, 0.05);
        close(l.upper_action, 66.3, 0.05);
        // Example 5: action limits only, tolerance 15 % of 18.0 µg/g.
        let l = ControlLimits::x_action_only(18.0, 0.15 * 18.0).unwrap();
        assert!(!l.has_warning_limits());
        close(l.lower_action.unwrap(), 15.3, 1e-12);
        close(l.upper_action, 20.7, 1e-12);
    }

    #[test]
    fn range_chart_limits_examples() {
        // Example 3: mean range 0.559 µg/L, s = 0.496 µg/L, WL 1.40, AL 1.83.
        let f = range_factors(2).unwrap();
        let s = 0.559 / f.d2;
        close(s, 0.496, 1e-3);
        let l = ControlLimits::range(2, s, 0.559).unwrap();
        close(l.upper_warning.unwrap(), 1.40, 0.005);
        close(l.upper_action, 1.83, 0.005);
        assert!(l.lower_action.is_none() && l.lower_warning.is_none());
        // Example 6: mean r% 1.88 %, CV 1.67 %, WL 4.73 %, AL 6.16 %.
        let cv = 1.88 / f.d2;
        close(cv, 1.67, 0.005);
        let l = ControlLimits::range(2, 1.67, 1.88).unwrap();
        close(l.upper_warning.unwrap(), 4.73, 0.005);
        close(l.upper_action, 6.16, 0.005);
        // Chapter 7, range case 2: r = 1 % gives s_r = 0.357 %.
        close(sd_from_repeatability_limit(1.0), 0.357, 5e-4);
        assert_eq!(control_value(ChartKind::Range, &[7.1, 6.9, 7.0]).unwrap(), 7.1 - 6.9);
        close(control_value(ChartKind::RelativeRange, &[9.0, 11.0]).unwrap(), 20.0, 1e-12);
    }

    #[test]
    fn daily_rules() {
        let l = ControlLimits::x(0.0, 1.0).unwrap();
        let v = [0.5, 2.5, 0.1, 2.2, -2.5, 3.5, -2.1, 2.4, 2.6, -0.3, 2.0];
        let s = daily(&v, &l);
        use Status::*;
        assert_eq!(
            s,
            vec![
                InControl,
                Warning,
                InControl,
                TwoOfThree,
                Warning,
                OutsideAction,
                TwoOfThree,
                TwoOfThree,
                TwoOfThree,
                InControl,
                InControl
            ]
        );
        // A value beyond the action limit counts for the next two values.
        let s = daily(&[3.2, 0.0, 2.5], &l);
        assert_eq!(s[2], TwoOfThree);
        let s = daily(&[3.2, 0.0, 0.0, 2.5], &l);
        assert_eq!(s[3], Warning);
        // Without warning limits only the action limit applies.
        let l = ControlLimits::x_action_only(0.0, 2.5).unwrap();
        assert_eq!(daily(&[2.4, 2.4, 2.6], &l), vec![InControl, InControl, OutsideAction]);
        // Range chart: upper limits only.
        let l = ControlLimits::range(2, 1.0, 1.128).unwrap();
        assert_eq!(daily(&[0.0, 3.0, 3.7], &l), vec![InControl, Warning, OutsideAction]);
    }

    #[test]
    fn statistical_control_rules() {
        let v = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 6.0];
        let s = statistical_signals(&v, 4.0);
        assert!(!s[5].trend && s[6].trend && !s[7].trend);
        // Example 4: twelve consecutive values above the central line.
        let mut v = vec![-1.0; 3];
        v.extend([1.0; 12]);
        let s = statistical_signals(&v, 0.0);
        assert!(!s[11].run && s[12].run && s[14].run);
        // Ten of eleven, one on the other side.
        let v = [1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        assert!(statistical_signals(&v, 0.0)[10].run);
        let v = [1.0, 1.0, 1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        assert!(!statistical_signals(&v, 0.0)[10].run);
    }

    #[test]
    fn review_example_8() {
        // X-chart: preliminary x̄ 1.055, s 0.0667 (60 values); new x̄ 1.041,
        // s 0.0834 (59 values after excluding one beyond 4 s).
        let prev = Summary { mean: 1.055, sd: 0.0667, n: 60, dof: 59.0 };
        let cur = Summary { mean: 1.041, sd: 0.0834, n: 59, dof: 58.0 };
        let r = review(&prev, &cur, 0.95).unwrap();
        close(r.f, 1.563, 5e-4);
        assert!(r.sd_increased && !r.sd_significant());
        assert_eq!(r.f_dof, (58.0, 59.0));
        // TR 569 reads 1.67 from Table 3 at (60, 60); the exact value is:
        close(r.f_critical, f_quantile(0.975, 58.0, 59.0), 1e-12);
        close(r.f_critical, 1.68, 0.01);
        close(r.pooled_sd, 0.07545, 5e-5);
        close(r.pooled_dof, 117.0, 0.0);
        close(r.t, 1.012, 5e-4);
        close(r.t_critical, 1.98, 0.005);
        assert!(!r.mean_significant());
        close(r.mean_shift, 0.21, 0.005);
        // R-chart: s_r 0.0975 against 0.0957.
        let prev = Summary { mean: 0.11, sd: 0.0975, n: 60, dof: 60.0 };
        let cur = Summary { mean: 0.108, sd: 0.0957, n: 60, dof: 60.0 };
        let r = review(&prev, &cur, 0.95).unwrap();
        close(r.f, 1.037, 1.5e-3);
        assert!(!r.sd_increased && !r.sd_significant());
        assert_eq!(beyond(&[1.0, 1.4, 0.9], 1.055, 0.0667, 4.0), vec![1]);
    }

    #[test]
    fn pooling_example_12() {
        let days = [
            [7.1, 7.1, 7.0],
            [6.9, 6.7, 6.8],
            [6.6, 6.5, 6.9],
            [6.7, 6.5, 6.6],
            [7.0, 6.9, 6.6],
            [7.3, 7.4, 7.3],
            [7.1, 7.1, 6.9],
            [7.0, 6.5, 6.8],
        ];
        let by_day: Vec<Summary> = days.iter().map(|d| summary(d).unwrap()).collect();
        close(by_day[0].sd, 0.058, 5e-4);
        close(by_day[7].sd, 0.252, 5e-4);
        let sds: Vec<f64> = by_day.iter().map(|s| s.sd).collect();
        close(pooled_sd_equal(&sds), 0.15, 0.005);
        let (sp, dof) = pooled_sd(&by_day).unwrap();
        close(sp, pooled_sd_equal(&sds), 1e-12);
        assert_eq!(dof, 16.0);
        // Simplified s_Rw: pool the standard deviations of the first, second
        // and third measurements over the days.
        let by_position: Vec<f64> =
            (0..3).map(|j| summary(&days.iter().map(|d| d[j]).collect::<Vec<_>>()).unwrap().sd).collect();
        close(by_position[0], 0.226, 5e-4);
        close(by_position[1], 0.342, 5e-4);
        close(pooled_sd_equal(&by_position), 0.270, 5e-4);
        // ANOVA: s_r 0.1541, s_I 0.2246, s_Rw 0.272 (footnote 24, whose
        // printed values are ten times too small; research log F53).
        let a = run_anova(&days.iter().map(|d| d.to_vec()).collect::<Vec<_>>()).unwrap();
        close(a.s_r, 0.1541, 5e-5);
        close(a.s_between, 0.2246, 1e-4);
        close(a.s_rw, 0.272, 5e-4);
        close(a.n0, 3.0, 1e-12);
        close(weighted_mean(&by_day), a.mean, 1e-12);
    }

    #[test]
    fn table_1_zinc() {
        let v = [
            64.5, 66.3, 61.1, 59.7, 57.4, 56.2, 58.4, 58.2, 63.0, 59.5, 56.0, 59.4, 60.2, 62.9, 60.5, 60.8, 61.5, 58.5,
            58.9, 60.5, 61.2, 57.8, 63.4, 60.2, 61.5, 62.3, 60.5, 61.7, 64.0, 62.7, 61.0, 65.4, 60.0, 59.2, 57.0, 62.5,
            57.7, 56.2, 62.9, 62.5, 56.5, 60.2, 58.2, 56.5, 64.7, 54.5, 60.5, 59.5, 61.6, 60.8, 58.7, 54.4, 62.2, 59.0,
            60.3, 60.8, 59.5, 60.0, 61.8, 63.8,
        ];
        let s = summary(&v).unwrap();
        close(s.mean, 60.278, 5e-4);
        close(s.sd, 2.598, 5e-4);
        let (t, tc) = t_against_reference(&s, 60.0, 0.95);
        assert!(t < tc);
    }
}
