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
    })
}

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
