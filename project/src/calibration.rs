//! Calibration projects with uncertainties in both the contents of the
//! reference materials and the responses [ISO 6143:2025]: the analysis
//! function fitted by generalized least squares, its compatibility with the
//! calibration data, the contents of samples and their uncertainties and
//! covariances, the upper bound of the uncertainty over the calibration range,
//! drift control and the validation with a mismatching mixture.

use crate::schema::Check;
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uncertainty_core::gls::{self, AnalysisFunction, CalibrationData, DriftTest, GlsFit};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: what.to_string(), message: message.into() }
}

fn default_two() -> f64 {
    2.0
}

fn default_true() -> bool {
    true
}

fn default_ten() -> usize {
    10
}

fn is_true(b: &bool) -> bool {
    *b
}

// ---------------------------------------------------------------------------
// Specification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalibrationSpec {
    /// Name of the analyte, e.g. "CO₂ in N₂".
    pub analyte: String,
    /// Unit of the contents x.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Unit of the responses y.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_unit: Option<String>,
    /// Reference materials (6.1, steps A–G).
    pub points: Vec<PointSpec>,
    /// Covariances u(xᵢ, xₕ) of the contents (common parent mixtures).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub covariances: Vec<CovarianceSpec>,
    pub function: FunctionSpec,
    /// Also fit the other analysis functions and compare their goodness of
    /// fit (6.2.2).
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub compare: bool,
    /// Mixtures to be analysed (6.3).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub samples: Vec<SampleSpec>,
    #[serde(default = "default_two")]
    pub coverage_factor: f64,
    /// Acceptable standard uncertainty of the results (6.2.3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_uncertainty: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift: Option<DriftSpec>,
    /// Mismatching reference mixtures for validation (6.2.5).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation: Vec<ValidationSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PointSpec {
    pub name: String,
    pub x: f64,
    pub u_x: f64,
    pub y: f64,
    pub u_y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CovarianceSpec {
    /// Names of two calibration points.
    pub between: [String; 2],
    pub value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionSpec {
    Linear,
    Quadratic,
    Cubic,
    Power,
    Exponential,
}

impl FunctionSpec {
    pub fn function(self) -> AnalysisFunction {
        match self {
            FunctionSpec::Linear => AnalysisFunction::Polynomial(1),
            FunctionSpec::Quadratic => AnalysisFunction::Polynomial(2),
            FunctionSpec::Cubic => AnalysisFunction::Polynomial(3),
            FunctionSpec::Power => AnalysisFunction::Power,
            FunctionSpec::Exponential => AnalysisFunction::Exponential,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            FunctionSpec::Linear => "linear",
            FunctionSpec::Quadratic => "quadratic",
            FunctionSpec::Cubic => "cubic",
            FunctionSpec::Power => "power",
            FunctionSpec::Exponential => "exponential",
        }
    }

    pub const ALL: [FunctionSpec; 5] = [
        FunctionSpec::Linear,
        FunctionSpec::Quadratic,
        FunctionSpec::Cubic,
        FunctionSpec::Power,
        FunctionSpec::Exponential,
    ];
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SampleSpec {
    pub name: String,
    pub y: f64,
    pub u_y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DriftSpec {
    /// Name of the bracketing calibration point measured before and after.
    pub reference: String,
    /// Mean responses before and after the sample.
    pub before: f64,
    pub after: f64,
    /// Number of measurements before and after (default 10).
    #[serde(default = "default_ten")]
    pub n: usize,
    /// Number of measurements at calibration (default 10).
    #[serde(default = "default_ten")]
    pub n_calib: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationSpec {
    pub name: String,
    pub y: f64,
    pub u_y: f64,
    pub x_ref: f64,
    pub u_x_ref: f64,
}

// ---------------------------------------------------------------------------
// Expected values
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedCalibration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<Check>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub u_params: Vec<Check>,
    /// Covariances of the parameters, upper triangle by rows: (0,1), (0,2), …
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cov_params: Vec<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_res: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gamma: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admissible: Option<bool>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub samples: BTreeMap<String, ExpectedSample>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_passed: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedSample {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<Check>,
}

// ---------------------------------------------------------------------------
// Results
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Candidate {
    pub function: FunctionSpec,
    /// Γ and S_res, or the reason the function could not be fitted.
    pub fit: Result<(f64, f64), String>,
}

#[derive(Clone, Debug)]
pub struct SampleResult {
    pub name: String,
    pub y: f64,
    pub u_y: f64,
    pub x: f64,
    pub u: f64,
    pub expanded: f64,
    /// Inside the range of the calibration responses.
    pub in_range: bool,
}

#[derive(Clone, Debug)]
pub struct ValidationResult {
    pub name: String,
    pub x_obs: f64,
    pub u_obs: f64,
    pub x_ref: f64,
    pub u_ref: f64,
    pub difference: f64,
    pub limit: f64,
    pub passed: bool,
}

#[derive(Clone, Debug)]
pub struct CalibrationAnalysis {
    pub function: FunctionSpec,
    pub point_names: Vec<String>,
    pub fit: GlsFit,
    /// Γ ≤ 2 (6.2.2).
    pub admissible: bool,
    pub candidates: Vec<Candidate>,
    pub samples: Vec<SampleResult>,
    /// Correlation coefficients between the sample results.
    pub sample_correlations: Vec<Vec<f64>>,
    pub coverage_factor: f64,
    /// u(x_lo), u(x_hi) at the lowest and highest calibration responses
    /// (6.2.3).
    pub u_lo: f64,
    pub u_hi: f64,
    pub target_met: Option<bool>,
    pub drift: Option<(String, f64, DriftTest)>,
    pub validation: Vec<ValidationResult>,
}

impl CalibrationAnalysis {
    pub fn upper_bound(&self) -> f64 {
        self.u_lo.max(self.u_hi)
    }
}

/// Evaluates a calibration project.
pub fn evaluate(spec: &CalibrationSpec) -> Result<CalibrationAnalysis, ProjectError> {
    let n = spec.points.len();
    let mut names: Vec<&str> = spec.points.iter().map(|p| p.name.as_str()).collect();
    names.sort_unstable();
    if names.windows(2).any(|w| w[0] == w[1]) {
        return Err(err("points", "names must be unique"));
    }
    let index = |name: &str, what: &str| {
        spec.points
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| err(what, format!("no calibration point named \"{name}\"")))
    };
    let mut cov_x = Vec::new();
    for c in &spec.covariances {
        let (i, h) = (index(&c.between[0], "covariances")?, index(&c.between[1], "covariances")?);
        if i == h {
            return Err(err("covariances", "a covariance needs two different points"));
        }
        let limit = spec.points[i].u_x * spec.points[h].u_x;
        if !c.value.is_finite() || c.value.abs() > limit {
            return Err(err("covariances", format!("|u(x{}, x{})| must not exceed u(x{0})·u(x{1})", i + 1, h + 1)));
        }
        cov_x.push((i, h, c.value));
    }
    let data = CalibrationData {
        x: spec.points.iter().map(|p| p.x).collect(),
        ux: spec.points.iter().map(|p| p.u_x).collect(),
        y: spec.points.iter().map(|p| p.y).collect(),
        uy: spec.points.iter().map(|p| p.u_y).collect(),
        cov_x,
    };
    if spec.coverage_factor.is_nan() || spec.coverage_factor <= 0.0 {
        return Err(err("coverage_factor", "must be positive"));
    }
    let fit = gls::gls_fit(spec.function.function(), &data).map_err(|e| err("points", e.to_string()))?;
    let candidates = if spec.compare {
        // Functions with as many parameters as points fit exactly and are left out.
        FunctionSpec::ALL
            .iter()
            .filter(|&&f| f == spec.function || n > f.function().parameters())
            .map(|&f| Candidate {
                function: f,
                fit: if f == spec.function {
                    Ok((fit.gamma, fit.s_res))
                } else {
                    gls::gls_fit(f.function(), &data).map(|r| (r.gamma, r.s_res)).map_err(|e| e.to_string())
                },
            })
            .collect()
    } else {
        Vec::new()
    };
    let (y_min, y_max) = data.y.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &y| (a.min(y), b.max(y)));
    let k = spec.coverage_factor;
    let mut samples = Vec::new();
    for s in &spec.samples {
        if !(s.y.is_finite() && s.u_y.is_finite() && s.u_y >= 0.0) {
            return Err(err(&format!("samples[{}]", s.name), "finite y and non-negative u_y required"));
        }
        let (x, u) = fit.predict(s.y, s.u_y);
        if !x.is_finite() {
            return Err(err(&format!("samples[{}]", s.name), "the analysis function is not defined at this response"));
        }
        samples.push(SampleResult {
            name: s.name.clone(),
            y: s.y,
            u_y: s.u_y,
            x,
            u,
            expanded: k * u,
            in_range: s.y >= y_min && s.y <= y_max,
        });
    }
    let sample_correlations = samples
        .iter()
        .map(|a| {
            samples
                .iter()
                .map(|b| {
                    if a.name == b.name {
                        1.0
                    } else {
                        let c = fit.prediction_covariance(a.y, b.y);
                        if a.u > 0.0 && b.u > 0.0 {
                            c / (a.u * b.u)
                        } else {
                            0.0
                        }
                    }
                })
                .collect()
        })
        .collect();
    // 6.2.3: responses and uncertainties of the lowest and highest standards.
    let lo = spec.points.iter().min_by(|a, b| a.x.total_cmp(&b.x)).ok_or_else(|| err("points", "no points"))?;
    let hi = spec.points.iter().max_by(|a, b| a.x.total_cmp(&b.x)).ok_or_else(|| err("points", "no points"))?;
    let u_lo = fit.predict(lo.y, lo.u_y).1;
    let u_hi = fit.predict(hi.y, hi.u_y).1;
    let target_met = spec.target_uncertainty.map(|t| u_lo.max(u_hi) <= t);
    let drift = match &spec.drift {
        None => None,
        Some(d) => {
            let i = index(&d.reference, "drift.reference")?;
            let p = &spec.points[i];
            let t = gls::drift_test(p.y, p.u_y, d.before, d.after, d.n, d.n_calib)
                .map_err(|e| err("drift", e.to_string()))?;
            Some((p.name.clone(), p.y, t))
        }
    };
    let validation = spec
        .validation
        .iter()
        .map(|v| {
            let (x_obs, u_obs) = fit.predict(v.y, v.u_y);
            let (difference, limit, passed) = gls::mismatch_check(x_obs, u_obs, v.x_ref, v.u_x_ref);
            ValidationResult {
                name: v.name.clone(),
                x_obs,
                u_obs,
                x_ref: v.x_ref,
                u_ref: v.u_x_ref,
                difference,
                limit,
                passed,
            }
        })
        .collect();
    Ok(CalibrationAnalysis {
        function: spec.function,
        point_names: spec.points.iter().map(|p| p.name.clone()).collect(),
        admissible: fit.gamma <= 2.0,
        fit,
        candidates,
        samples,
        sample_correlations,
        coverage_factor: k,
        u_lo,
        u_hi,
        target_met,
        drift,
        validation,
    })
}

/// Compares the analysis with the expected values.
pub fn check(e: &ExpectedCalibration, a: &CalibrationAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut push = |what: String, computed: Option<f64>, c: &Check| {
        let computed = computed.unwrap_or(f64::NAN);
        out.push(crate::CheckOutcome {
            what,
            computed,
            expected: c.value,
            tolerance: c.tolerance,
            passed: (computed - c.value).abs() <= c.tolerance,
        });
    };
    let m = a.fit.params.len();
    for (j, c) in e.params.iter().enumerate() {
        push(format!("calibration.b{j}"), a.fit.params.get(j).copied(), c);
    }
    for (j, c) in e.u_params.iter().enumerate() {
        push(format!("calibration.u(b{j})"), (j < m).then(|| a.fit.covariance[j][j].sqrt()), c);
    }
    let pairs: Vec<(usize, usize)> = (0..m).flat_map(|j| (j + 1..m).map(move |l| (j, l))).collect();
    for (c, &(j, l)) in e.cov_params.iter().zip(pairs.iter().chain(std::iter::repeat(&(m, m)))) {
        push(format!("calibration.u(b{j}, b{l})"), (l < m).then(|| a.fit.covariance[j][l]), c);
    }
    if let Some(c) = &e.s_res {
        push("calibration.s_res".into(), Some(a.fit.s_res), c);
    }
    if let Some(c) = &e.gamma {
        push("calibration.gamma".into(), Some(a.fit.gamma), c);
    }
    for (name, es) in &e.samples {
        let s = a.samples.iter().find(|s| &s.name == name);
        if let Some(c) = &es.x {
            push(format!("calibration.samples[{name}].x"), s.map(|s| s.x), c);
        }
        if let Some(c) = &es.u {
            push(format!("calibration.samples[{name}].u"), s.map(|s| s.u), c);
        }
    }
    let mut verdict = |what: &str, got: Option<bool>, want: bool| {
        let computed = got.map_or(f64::NAN, |b| if b { 1.0 } else { 0.0 });
        let expected = if want { 1.0 } else { 0.0 };
        out.push(crate::CheckOutcome {
            what: what.into(),
            computed,
            expected,
            tolerance: 0.0,
            passed: computed == expected,
        });
    };
    if let Some(v) = e.admissible {
        verdict("calibration.admissible", Some(a.admissible), v);
    }
    if let Some(v) = e.drift_passed {
        verdict("calibration.drift_passed", a.drift.as_ref().map(|d| d.2.passed), v);
    }
}

pub fn results_json(a: &CalibrationAnalysis) -> serde_json::Value {
    use serde_json::json;
    let f = &a.fit;
    json!({
        "function": a.function.code(),
        "point_names": a.point_names,
        "params": f.params,
        "covariance": f.covariance,
        "s_res": f.s_res,
        "dof": f.dof,
        "gamma": f.gamma,
        "admissible": a.admissible,
        "adjusted": f.adjusted.iter().map(|p| json!({"x": p.0, "y": p.1})).collect::<Vec<_>>(),
        "weighted_deviations": f.weighted_deviations.iter().map(|p| json!({"x": p.0, "y": p.1})).collect::<Vec<_>>(),
        "candidates": a.candidates.iter().map(|c| match &c.fit {
            Ok((g, s)) => json!({"function": c.function.code(), "gamma": g, "s_res": s, "admissible": *g <= 2.0}),
            Err(e) => json!({"function": c.function.code(), "error": e}),
        }).collect::<Vec<_>>(),
        "coverage_factor": a.coverage_factor,
        "samples": a.samples.iter().map(|s| json!({"name": s.name, "y": s.y, "u_y": s.u_y, "x": s.x, "u": s.u,
            "expanded": s.expanded, "in_range": s.in_range})).collect::<Vec<_>>(),
        "sample_correlations": a.sample_correlations,
        "u_lo": a.u_lo,
        "u_hi": a.u_hi,
        "target_met": a.target_met,
        "drift": a.drift.as_ref().map(|(name, y, t)| json!({"reference": name, "y_calib": y,
            "differences": t.differences, "critical": t.critical, "passed": t.passed})),
        "validation": a.validation.iter().map(|v| json!({"name": v.name, "x_obs": v.x_obs, "u_obs": v.u_obs,
            "x_ref": v.x_ref, "u_ref": v.u_ref, "difference": v.difference, "limit": v.limit, "passed": v.passed}))
            .collect::<Vec<_>>(),
    })
}
