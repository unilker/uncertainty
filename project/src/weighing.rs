//! Approximation of the errors of indication of a weighing instrument
//! (EURAMET cg-18 v4.0, Appendix C2.2): the coefficients of the error
//! function, the goodness of fit and the approximated errors at readings.

use crate::schema::Check;
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use uncertainty_core::weighing::{self, Approximation, Criterion, Polynomial, ReferenceCorrelation, TestPoint};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: format!("error_function.{what}"), message: message.into() }
}

fn default_one() -> usize {
    1
}

fn default_two() -> f64 {
    2.0
}

fn is_one(x: &usize) -> bool {
    *x == 1
}

fn is_full(c: &CorrelationSpec) -> bool {
    *c == CorrelationSpec::Full
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorFunctionSpec {
    /// The instrument, e.g. "Balance 400 g / 0.1 mg".
    pub instrument: String,
    /// Unit of the indications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Unit of the errors and their uncertainties (default: `unit`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_unit: Option<String>,
    /// Errors E = (I − m_ref)·error_factor when points give `reference`
    /// (e.g. 1000 for indications in g and errors in mg).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_factor: Option<f64>,
    pub points: Vec<PointSpec>,
    /// Degree nₐ of the polynomial (default 1).
    #[serde(default = "default_one", skip_serializing_if = "is_one")]
    pub degree: usize,
    /// Without the constant term: E = a₁I (+ a₂I² …) (C2.2-16).
    #[serde(default)]
    pub through_zero: bool,
    #[serde(default, skip_serializing_if = "is_full")]
    pub reference_correlation: CorrelationSpec,
    /// Model term s_m (C2.2-3c), in the unit of the errors (default 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_m: Option<f64>,
    /// Coverage factor of U(E_appr) and of the residual test C2.2-2b.
    #[serde(default = "default_two")]
    pub coverage_factor: f64,
    /// Readings at which the approximated error is reported (C2.2-12, -13).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readings: Vec<ReadingSpec>,
    /// Uncertainty of a weighing result in use and minimum weight (7.4, 7.5,
    /// Appendix G); needs the straight line through zero (C2.2-16).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_use: Option<InUseSpec>,
}

fn default_rho_0() -> f64 {
    1.2
}

fn default_rho_c() -> f64 {
    8000.0
}

fn default_f64_one() -> f64 {
    1.0
}

/// Conditions of use of the instrument (cg-18 7.4.1 to 7.4.4). Lengths and
/// masses are in the unit of the indications.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InUseSpec {
    /// Weighing intervals (or ranges) in ascending order; one for a
    /// single-interval instrument.
    pub intervals: Vec<IntervalSpec>,
    /// Scale interval at zero (default: d of the first interval).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d0: Option<f64>,
    /// Maximum deviation of the eccentricity test and its load (7.4.4-10).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eccentricity: Option<EccentricitySpec>,
    /// Temperature coefficient K_T (1/K) and variation ΔT (K) (7.4.3-1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<TemperatureSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buoyancy: Option<BuoyancySpec>,
    /// Change of the adjustment since calibration (7.4.3-6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustment: Option<AdjustmentSpec>,
    /// Tare balancing in use: non-linearity from the test points (7.4.4-5).
    #[serde(default)]
    pub tare: bool,
    /// Creep and hysteresis (7.4.4-7, -9).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<TimeSpec>,
    /// Further relative standard uncertainties.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relative: Vec<RelativeTermSpec>,
    /// Coverage factor of U(W) and U_gl(W) (default 2, 7.5.1).
    #[serde(default = "default_two")]
    pub coverage_factor: f64,
    /// Relative weighing tolerance requirement Req (e.g. 0.01 for 1 %)
    /// for the minimum weight (Appendix G).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirement: Option<f64>,
    #[serde(default = "default_f64_one")]
    pub safety_factor: f64,
    /// Readings at which U(W) and U_gl(W) are reported.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readings: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntervalSpec {
    pub max: f64,
    pub d: f64,
    /// Standard deviation of a single reading (7.4.1-4)…
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s: Option<f64>,
    /// …or the readings of the repeatability test.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeatability: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EccentricitySpec {
    pub deviation: f64,
    pub load: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemperatureSpec {
    pub coefficient: f64,
    pub delta_t: f64,
}

/// Air buoyancy in use: from the temperature variation (7.4.3-4), the most
/// conservative estimate (7.4.3-5) or a relative value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuoyancySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta_t: Option<f64>,
    #[serde(default)]
    pub conservative: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative: Option<f64>,
    #[serde(default = "default_rho_0")]
    pub rho_0: f64,
    #[serde(default = "default_rho_c")]
    pub rho_c: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdjustmentSpec {
    /// Largest change ΔE(Max) between calibrations, or mpe(Max).
    pub delta_e_max: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeSpec {
    /// Error on return to zero after loading upwards (7.4.4-7)…
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e0: Option<f64>,
    /// …or mpe(Max), times `multiplier` for instruments without type
    /// approval (7.4.4-9a, -9b).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mpe_max: Option<f64>,
    #[serde(default = "default_f64_one")]
    pub multiplier: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelativeTermSpec {
    pub name: String,
    pub u: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PointSpec {
    pub indication: f64,
    /// Error of indication, in the unit of the errors…
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<f64>,
    /// …or the reference mass m_ref, in the unit of the indications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<f64>,
    /// u(m_ref) and u(I), in the unit of the errors.
    pub u_reference: f64,
    pub u_indication: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CorrelationSpec {
    /// U(m_ref) = s sᵀ (C2.2-3b).
    #[default]
    Full,
    None,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadingSpec {
    pub reading: f64,
    /// u(R) in the unit of the indications (C2.2-14), default 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_reading: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedErrorFunction {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coefficients: Vec<Check>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub u_coefficients: Vec<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chi_squared: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chi_squared_passed: Option<bool>,
    /// Number of test points failing the residual test C2.2-2b.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub residual_failures: Option<usize>,
    /// u(E_appr,j) at the test points, in order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub u_approximated: Vec<Check>,
    /// Approximated errors and their standard uncertainties at the readings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readings: Vec<ExpectedReading>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_use: Option<ExpectedInUse>,
}

/// Expected values of the uncertainty in use, per interval where a list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedInUse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alpha_squared: Vec<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beta_squared: Option<Check>,
    /// U(W) at the lower limit of each interval (0 for the first).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expanded_at_lower: Vec<Check>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slope: Vec<Check>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub global_at_lower: Vec<Check>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub global_slope: Vec<Check>,
    /// Minimum weight with the safety factor (G-9).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_weight: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedReading {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<Check>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReadingResult {
    pub reading: f64,
    pub u_reading: f64,
    pub error: f64,
    pub u: f64,
}

/// A term of the uncertainty in use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InUseTerm {
    /// u(a₁) of the approximated error (C2.2-16d).
    ErrorFunction,
    Temperature,
    Buoyancy,
    BuoyancyConservative,
    Adjustment,
    Tare,
    TimeReturnToZero,
    TimeMpe,
    Eccentricity,
    Other,
}

impl InUseTerm {
    pub fn formula(self) -> &'static str {
        match self {
            InUseTerm::ErrorFunction => "C2.2-16d",
            InUseTerm::Temperature => "7.4.3-1",
            InUseTerm::Buoyancy => "7.4.3-4",
            InUseTerm::BuoyancyConservative => "7.4.3-5",
            InUseTerm::Adjustment => "7.4.3-6",
            InUseTerm::Tare => "7.4.4-5",
            InUseTerm::TimeReturnToZero => "7.4.4-7",
            InUseTerm::TimeMpe => "7.4.4-9",
            InUseTerm::Eccentricity => "7.4.4-10",
            InUseTerm::Other => "",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InUseAnalysis {
    pub intervals: Vec<weighing::UseInterval>,
    pub d0: f64,
    /// Relative standard uncertainties with their kind and, for `Other`, name.
    pub terms: Vec<(InUseTerm, String, f64)>,
    /// a₁ in the unit of the indications per unit of indication.
    pub a1: f64,
    pub coverage_factor: f64,
    pub result: weighing::InUseResult,
    pub requirement: Option<f64>,
    pub safety_factor: f64,
    /// Minimum weight without and with the safety factor (G-7, G-9).
    pub minimum_weight: Option<f64>,
    pub minimum_weight_sf: Option<f64>,
    /// Reading, U(W) = k√(α² + β²R²), its linear form, U_gl(W).
    pub readings: Vec<(f64, f64, f64, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorFunctionAnalysis {
    pub instrument: String,
    pub unit: Option<String>,
    pub error_unit: Option<String>,
    pub points: Vec<TestPoint>,
    pub correlation: ReferenceCorrelation,
    pub coverage_factor: f64,
    pub fit: Approximation,
    /// Indices of the points failing C2.2-2b.
    pub residual_failures: Vec<usize>,
    /// Smallest s_m meeting C2.2-2a and C2.2-2b (None if the search fails).
    pub min_s_m_chi_squared: Option<f64>,
    pub min_s_m_residuals: Option<f64>,
    pub readings: Vec<ReadingResult>,
    pub in_use: Option<InUseAnalysis>,
}

fn in_use(
    u: &InUseSpec,
    spec: &ErrorFunctionSpec,
    points: &[TestPoint],
    fit: &Approximation,
    factor: f64,
) -> Result<InUseAnalysis, ProjectError> {
    let e = |what: &str, m: &str| err(&format!("in_use.{what}"), m);
    if !(spec.degree == 1 && spec.through_zero) {
        return Err(e("", "needs the straight line through zero, E = a₁R (C2.2-16): degree 1 with through_zero"));
    }
    let mut intervals = Vec::new();
    for (i, iv) in u.intervals.iter().enumerate() {
        let s = match (iv.s, iv.repeatability.len()) {
            (Some(s), 0) => s,
            (None, n) if n >= 2 => {
                let m = iv.repeatability.iter().sum::<f64>() / n as f64;
                (iv.repeatability.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt()
            }
            _ => return Err(e(&format!("intervals[{i}]"), "give s, or at least two repeatability readings")),
        };
        intervals.push(weighing::UseInterval { max: iv.max, d: iv.d, s });
    }
    let max = intervals.last().map(|i| i.max).ok_or_else(|| e("intervals", "at least one interval is needed"))?;
    let a1 = fit.coefficients[0] / factor;
    let mut terms = vec![(InUseTerm::ErrorFunction, String::new(), fit.covariance[0][0].sqrt() / factor.abs())];
    if let Some(t) = &u.temperature {
        terms.push((InUseTerm::Temperature, String::new(), weighing::relative_temperature(t.coefficient, t.delta_t)));
    }
    if let Some(b) = &u.buoyancy {
        let term = match (b.delta_t, b.conservative, b.relative) {
            (Some(dt), false, None) => (InUseTerm::Buoyancy, weighing::relative_buoyancy(dt, b.rho_0, b.rho_c)),
            (None, true, None) => {
                (InUseTerm::BuoyancyConservative, weighing::relative_buoyancy_conservative(b.rho_0, b.rho_c))
            }
            (None, false, Some(r)) => (InUseTerm::Other, r),
            _ => return Err(e("buoyancy", "give one of delta_t, conservative or relative")),
        };
        terms.push((term.0, if term.0 == InUseTerm::Other { "δR_buoy".into() } else { String::new() }, term.1));
    }
    if let Some(a) = &u.adjustment {
        terms.push((InUseTerm::Adjustment, String::new(), weighing::relative_adjustment(a.delta_e_max, max)));
    }
    if u.tare {
        let pts: Vec<(f64, f64)> = points.iter().map(|p| (p.indication, p.error / factor)).collect();
        let t = weighing::relative_tare(&pts).map_err(|x| e("tare", &x.to_string()))?;
        terms.push((InUseTerm::Tare, String::new(), t));
    }
    if let Some(t) = &u.time {
        let term = match (t.e0, t.mpe_max) {
            (Some(e0), None) => (InUseTerm::TimeReturnToZero, weighing::relative_time(e0, max)),
            (None, Some(mpe)) => (InUseTerm::TimeMpe, weighing::relative_time(t.multiplier * mpe, max)),
            _ => return Err(e("time", "give e0 or mpe_max")),
        };
        terms.push((term.0, String::new(), term.1));
    }
    if let Some(x) = &u.eccentricity {
        if x.load.is_nan() || x.load <= 0.0 {
            return Err(e("eccentricity", "the load must be positive"));
        }
        terms.push((InUseTerm::Eccentricity, String::new(), weighing::relative_eccentricity(x.deviation, x.load)));
    }
    for r in &u.relative {
        terms.push((InUseTerm::Other, r.name.clone(), r.u));
    }
    let model = weighing::WeighingInUse {
        intervals: intervals.clone(),
        d0: u.d0.unwrap_or(intervals[0].d),
        relative: terms.iter().map(|t| t.2).collect(),
        a1,
        k: u.coverage_factor,
    };
    let result = model.evaluate().map_err(|x| e("", &x.to_string()))?;
    if u.safety_factor.is_nan() || u.safety_factor < 1.0 {
        return Err(e("safety_factor", "must be at least 1"));
    }
    let (mw, mw_sf) = match u.requirement {
        Some(req) if req > 0.0 => {
            (result.minimum_weight(req, 1.0, max), result.minimum_weight(req, u.safety_factor, max))
        }
        Some(_) => return Err(e("requirement", "must be positive (e.g. 0.01 for 1 %)")),
        None => (None, None),
    };
    let readings = u
        .readings
        .iter()
        .map(|&r| (r, result.expanded(r, u.coverage_factor), result.expanded_linear(r), result.global(r)))
        .collect();
    Ok(InUseAnalysis {
        intervals,
        d0: model.d0,
        terms,
        a1,
        coverage_factor: u.coverage_factor,
        result,
        requirement: u.requirement,
        safety_factor: u.safety_factor,
        minimum_weight: mw,
        minimum_weight_sf: mw_sf,
        readings,
    })
}

pub fn evaluate(spec: &ErrorFunctionSpec) -> Result<ErrorFunctionAnalysis, ProjectError> {
    let k = spec.coverage_factor;
    if !(k > 0.0 && k.is_finite()) {
        return Err(err("coverage_factor", "must be positive"));
    }
    let factor = spec.error_factor.unwrap_or(1.0);
    if !(factor.is_finite() && factor != 0.0) {
        return Err(err("error_factor", "must be finite and non-zero"));
    }
    let mut points = Vec::new();
    for (i, p) in spec.points.iter().enumerate() {
        let error = match (p.error, p.reference) {
            (Some(e), None) => e,
            (None, Some(m)) => (p.indication - m) * factor,
            _ => return Err(err(&format!("points[{i}]"), "give either \"error\" or \"reference\"")),
        };
        if !(p.u_reference >= 0.0 && p.u_indication >= 0.0) {
            return Err(err(&format!("points[{i}]"), "uncertainties must be non-negative"));
        }
        points.push(TestPoint {
            indication: p.indication,
            error,
            u_reference: p.u_reference,
            u_indication: p.u_indication,
        });
    }
    let function = Polynomial { degree: spec.degree, through_zero: spec.through_zero };
    let correlation = match spec.reference_correlation {
        CorrelationSpec::Full => ReferenceCorrelation::Full,
        CorrelationSpec::None => ReferenceCorrelation::None,
    };
    let s_m = spec.s_m.unwrap_or(0.0);
    let fe = |e: uncertainty_core::UncError| err("points", e.to_string());
    let fit = weighing::approximate(&points, function, correlation, s_m).map_err(fe)?;
    let residual_failures = fit.residual_failures(k);
    let min_s_m_chi_squared = weighing::minimum_s_m(&points, function, correlation, Criterion::ChiSquared, k).ok();
    let min_s_m_residuals = weighing::minimum_s_m(&points, function, correlation, Criterion::Residuals, k).ok();
    let mut readings = Vec::new();
    for r in &spec.readings {
        let u_reading = r.u_reading.unwrap_or(0.0);
        if u_reading.is_nan() || u_reading < 0.0 || !r.reading.is_finite() {
            return Err(err("readings", "readings must be finite with non-negative u_reading"));
        }
        let (error, u) = fit.at(r.reading, u_reading);
        readings.push(ReadingResult { reading: r.reading, u_reading, error, u });
    }
    let in_use = spec.in_use.as_ref().map(|u| in_use(u, spec, &points, &fit, factor)).transpose()?;
    Ok(ErrorFunctionAnalysis {
        instrument: spec.instrument.clone(),
        unit: spec.unit.clone(),
        error_unit: spec.error_unit.clone().or_else(|| spec.unit.clone()),
        points,
        correlation,
        coverage_factor: k,
        fit,
        residual_failures,
        min_s_m_chi_squared,
        min_s_m_residuals,
        readings,
        in_use,
    })
}

/// A list of expected values with its name and the computed list.
type CheckList<'a> = (&'a str, &'a Vec<Check>, Option<&'a Vec<f64>>);

pub fn check(e: &ExpectedErrorFunction, a: &ErrorFunctionAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut push = |what: String, computed: Option<f64>, c: &Check| {
        let computed = computed.unwrap_or(f64::NAN);
        out.push(crate::CheckOutcome {
            what: format!("error_function.{what}"),
            computed,
            expected: c.value,
            tolerance: c.tolerance,
            passed: (computed - c.value).abs() <= c.tolerance,
        });
    };
    let f = &a.fit;
    for (i, c) in e.coefficients.iter().enumerate() {
        push(format!("coefficients[{i}]"), f.coefficients.get(i).copied(), c);
    }
    for (i, c) in e.u_coefficients.iter().enumerate() {
        push(format!("u_coefficients[{i}]"), f.covariance.get(i).map(|r| r[i].sqrt()), c);
    }
    if let Some(c) = &e.chi_squared {
        push("chi_squared".into(), Some(f.chi_squared), c);
    }
    let b = |x: bool| if x { 1.0 } else { 0.0 };
    if let Some(v) = e.chi_squared_passed {
        push("chi_squared_passed".into(), Some(b(f.chi_squared_passed())), &Check { value: b(v), tolerance: 0.0 });
    }
    if let Some(v) = e.residual_failures {
        push(
            "residual_failures".into(),
            Some(a.residual_failures.len() as f64),
            &Check { value: v as f64, tolerance: 0.0 },
        );
    }
    for (i, c) in e.u_approximated.iter().enumerate() {
        push(format!("u_approximated[{i}]"), f.u_approximated.get(i).copied(), c);
    }
    for (i, r) in e.readings.iter().enumerate() {
        let got = a.readings.get(i);
        if let Some(c) = &r.error {
            push(format!("readings[{i}].error"), got.map(|g| g.error), c);
        }
        if let Some(c) = &r.u {
            push(format!("readings[{i}].u"), got.map(|g| g.u), c);
        }
    }
    if let Some(eu) = &e.in_use {
        let r = a.in_use.as_ref().map(|u| &u.result);
        let lists: [CheckList; 5] = [
            ("alpha_squared", &eu.alpha_squared, r.map(|r| &r.alpha_squared)),
            ("expanded_at_lower", &eu.expanded_at_lower, r.map(|r| &r.expanded_at_lower)),
            ("slope", &eu.slope, r.map(|r| &r.slope)),
            ("global_at_lower", &eu.global_at_lower, r.map(|r| &r.global_at_lower)),
            ("global_slope", &eu.global_slope, r.map(|r| &r.global_slope)),
        ];
        for (name, checks, got) in lists {
            for (i, c) in checks.iter().enumerate() {
                push(format!("in_use.{name}[{i}]"), got.and_then(|g| g.get(i).copied()), c);
            }
        }
        if let Some(c) = &eu.beta_squared {
            push("in_use.beta_squared".into(), r.map(|r| r.beta_squared), c);
        }
        if let Some(c) = &eu.minimum_weight {
            push("in_use.minimum_weight".into(), a.in_use.as_ref().and_then(|u| u.minimum_weight_sf), c);
        }
    }
}

pub fn results_json(a: &ErrorFunctionAnalysis) -> serde_json::Value {
    use serde_json::json;
    let f = &a.fit;
    json!({
        "function": {"degree": f.function.degree, "through_zero": f.function.through_zero, "powers": f.function.powers()},
        "reference_correlation": match a.correlation { ReferenceCorrelation::Full => "full", ReferenceCorrelation::None => "none" },
        "coverage_factor": a.coverage_factor,
        "s_m": f.s_m,
        "coefficients": f.coefficients,
        "covariance": f.covariance,
        "u_coefficients": (0..f.coefficients.len()).map(|i| f.covariance[i][i].sqrt()).collect::<Vec<_>>(),
        "chi_squared": f.chi_squared,
        "dof": f.dof,
        "chi_squared_passed": f.chi_squared_passed(),
        "residual_failures": a.residual_failures,
        "min_s_m": {"chi_squared": a.min_s_m_chi_squared, "residuals": a.min_s_m_residuals},
        "points": a.points.iter().enumerate().map(|(j, p)| json!({
            "indication": p.indication, "error": p.error, "u_reference": p.u_reference, "u_indication": p.u_indication,
            "u_error": f.u_error[j], "approximated": f.approximated[j], "residual": f.residuals[j],
            "u_approximated": f.u_approximated[j],
        })).collect::<Vec<_>>(),
        "readings": a.readings.iter().map(|r| json!({
            "reading": r.reading, "u_reading": r.u_reading, "error": r.error, "u": r.u,
            "expanded": a.coverage_factor * r.u,
        })).collect::<Vec<_>>(),
        "in_use": a.in_use.as_ref().map(|u| json!({
            "intervals": u.intervals.iter().map(|i| json!({"max": i.max, "d": i.d, "s": i.s})).collect::<Vec<_>>(),
            "d0": u.d0,
            "terms": u.terms.iter().map(|(t, name, v)| json!({"formula": t.formula(), "name": name, "u_relative": v})).collect::<Vec<_>>(),
            "a1": u.a1,
            "coverage_factor": u.coverage_factor,
            "alpha_squared": u.result.alpha_squared,
            "beta_squared": u.result.beta_squared,
            "lower": u.result.lower,
            "expanded_at_lower": u.result.expanded_at_lower,
            "slope": u.result.slope,
            "global_at_lower": u.result.global_at_lower,
            "global_slope": u.result.global_slope,
            "requirement": u.requirement,
            "safety_factor": u.safety_factor,
            "minimum_weight": u.minimum_weight,
            "minimum_weight_sf": u.minimum_weight_sf,
            "readings": u.readings.iter().map(|(r, x, l, g)| json!({"reading": r, "expanded": x, "expanded_linear": l, "global": g})).collect::<Vec<_>>(),
        })),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_from_references_and_validation() {
        let s: ErrorFunctionSpec = serde_json::from_str(
            r#"{"instrument":"x","unit":"g","error_unit":"mg","error_factor":1000,"through_zero":true,
                "points":[{"indication":0,"error":0,"u_reference":0,"u_indication":0.04},
                          {"indication":100.0002,"reference":100.0,"u_reference":0.02,"u_indication":0.05},
                          {"indication":200.0004,"reference":200.0,"u_reference":0.04,"u_indication":0.06}],
                "readings":[{"reading":150}]}"#,
        )
        .unwrap();
        let a = evaluate(&s).unwrap();
        assert!((a.points[1].error - 0.2).abs() < 1e-9);
        assert!((a.fit.coefficients[0] - 0.002).abs() < 1e-7);
        assert!((a.readings[0].error - 0.3).abs() < 1e-5);
        assert_eq!(a.min_s_m_chi_squared, Some(0.0));
        let bad: ErrorFunctionSpec = serde_json::from_str(
            r#"{"instrument":"x","points":[{"indication":1,"error":0,"reference":1,"u_reference":0,"u_indication":1}]}"#,
        )
        .unwrap();
        assert!(evaluate(&bad).is_err());
    }
}
