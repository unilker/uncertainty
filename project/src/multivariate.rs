//! Projects with several output quantities (JCGM 102:2011): explicit or
//! implicit models, the generalized GUM uncertainty framework, the Monte
//! Carlo method with coverage regions and the validation of the GUM results.

use crate::schema::{AnalysisSpec, Check, CorrelationSpec, InputSpec, NamedExpression, Text};
use crate::{to_distribution, Overrides, ProjectError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uncertainty_core::multivariate::{
    self, MultiMcmOptions, MultiTrials, MultiValidation, MultivariateGum, MultivariateMcm, MultivariateModel,
    ObservationCovariance,
};
use uncertainty_core::InputQuantity;

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: what.to_string(), message: message.into() }
}

/// Points of the Monte Carlo cloud kept for the chart of a bivariate model.
pub const CHART_POINTS: usize = 2000;

// ---------------------------------------------------------------------------
// Specification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultivariateSpec {
    /// Output quantities: the measurement function f_j (explicit model) or
    /// the equation h_j(Y, X) = 0 (implicit model).
    pub outputs: Vec<MultiOutputSpec>,
    /// The expressions are equations h(Y, X) = 0 in the inputs and outputs.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub implicit: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intermediates: Vec<NamedExpression>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<InputSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub correlations: Vec<CorrelationSpec>,
    /// Input quantities given by series of vector indication values (5.3.2).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indications: Vec<IndicationsSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiOutputSpec {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub expression: String,
    /// Starting value for solving an implicit model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndicationsSpec {
    /// Names of the input quantities, one per column.
    pub inputs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub units: Vec<String>,
    /// One row per set of simultaneous indication values.
    pub values: Vec<Vec<f64>>,
    /// Covariance matrix for the GUM framework: `mean` (M/(n(n − 1)), GUM
    /// 5.2.3) or `t` (that of the multivariate t-distribution, 9.4.2.5).
    #[serde(default)]
    pub covariance: CovarianceSpec,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CovarianceSpec {
    #[default]
    Mean,
    T,
}

// ---------------------------------------------------------------------------
// Expected values
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedMultivariate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gum: Option<ExpectedMultiResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcm: Option<ExpectedMultiResult>,
    /// Outcome of the validation (clause 8) for the ellipsoid and the
    /// rectangle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ellipsoid_validated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rectangle_validated: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedMultiResult {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub estimate: BTreeMap<String, Check>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub standard_uncertainty: BTreeMap<String, Check>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub correlations: Vec<ExpectedCorrelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k_ellipsoid: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k_rectangle: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smallest_region_area: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedCorrelation {
    pub between: [String; 2],
    pub value: f64,
    pub tolerance: f64,
}

// ---------------------------------------------------------------------------
// Evaluation
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct MultivariateAnalysis {
    pub model: MultivariateModel,
    pub gum: Option<MultivariateGum>,
    pub mcm: Option<MultivariateMcm>,
    pub validation: Option<MultiValidation>,
    /// Up to [`CHART_POINTS`] Monte Carlo points (bivariate models).
    pub points: Vec<[f64; 2]>,
}

impl MultivariateAnalysis {
    pub fn output_names(&self) -> Vec<String> {
        self.model.output_names()
    }
}

/// Builds the engine model of a multivariate section.
pub fn build(spec: &MultivariateSpec) -> Result<MultivariateModel, ProjectError> {
    let mut b = MultivariateModel::builder().implicit(spec.implicit);
    for q in &spec.inputs {
        let mut iq = InputQuantity::new(&q.name, to_distribution(&q.name, &q.distribution)?);
        if let Some(nu) = q.dof {
            iq = iq.with_dof(nu);
        }
        if let Some(u) = &q.unit {
            iq = iq.with_unit(u);
        }
        b = b.input(iq);
    }
    for g in &spec.indications {
        if !g.units.is_empty() && g.units.len() != g.inputs.len() {
            return Err(err("indications", "give one unit per input quantity, or none"));
        }
        let names: Vec<&str> = g.inputs.iter().map(String::as_str).collect();
        let kind = match g.covariance {
            CovarianceSpec::Mean => ObservationCovariance::Mean,
            CovarianceSpec::T => ObservationCovariance::StudentT,
        };
        b = b.indications(&names, g.values.clone(), kind);
    }
    for i in &spec.intermediates {
        b = b.intermediate(&i.name, &i.expression);
    }
    for c in &spec.correlations {
        b = b.correlation(&c.a, &c.b, c.r);
    }
    for o in &spec.outputs {
        b = b.output(&o.name, o.unit.as_deref(), &o.expression, o.start);
    }
    Ok(b.build()?)
}

/// Runs the GUM framework and the Monte Carlo method as `analysis` asks.
pub fn evaluate(
    spec: &MultivariateSpec,
    analysis: &AnalysisSpec,
    overrides: &Overrides,
) -> Result<MultivariateAnalysis, ProjectError> {
    let model = build(spec)?;
    let p = analysis.gum.as_ref().and_then(|g| g.coverage_probability).unwrap_or(0.95);
    if analysis.gum.as_ref().is_some_and(|g| g.coverage_factor.is_some() || g.higher_order) {
        return Err(err("analysis.gum", "coverage_factor and higher_order do not apply to several output quantities"));
    }
    let gum = match &analysis.gum {
        Some(_) => Some(multivariate::gum(&model, p)?),
        None => None,
    };
    let m = model.n_outputs();
    let (mut mcm, mut validation, mut points) = (None, None, Vec::new());
    if let (Some(spec_mcm), false) = (&analysis.mcm, overrides.skip_mcm) {
        let p_mcm = spec_mcm.coverage_probability.unwrap_or(p);
        let trials = if let Some(t) = overrides.trials {
            MultiTrials::Fixed(t)
        } else if let Some(a) = &spec_mcm.adaptive {
            MultiTrials::Adaptive { ndig: a.ndig, tolerance_divisor: a.tolerance_divisor, max_trials: a.max_trials }
        } else {
            MultiTrials::Fixed(spec_mcm.trials.unwrap_or(1_000_000))
        };
        let opts = MultiMcmOptions {
            trials,
            coverage_probability: p_mcm,
            seed: overrides.seed.unwrap_or(spec_mcm.seed),
            mesh: 100,
            keep_values: m == 2,
        };
        let mut r =
            multivariate::mcm(&model, &opts, gum.as_ref().filter(|g| (g.coverage_probability - p_mcm).abs() < 1e-12))?;
        if let Some(v) = r.values.take() {
            points = v.chunks_exact(2).take(CHART_POINTS).map(|c| [c[0], c[1]]).collect();
        }
        if let Some(g) = &gum {
            if (g.coverage_probability - p_mcm).abs() < 1e-12 {
                validation = Some(multivariate::validate(g, &r, spec_mcm.validation_ndig));
            }
        }
        mcm = Some(r);
    }
    if gum.is_none() && mcm.is_none() {
        return Err(err("analysis", "neither the GUM framework nor the Monte Carlo method is requested"));
    }
    Ok(MultivariateAnalysis { model, gum, mcm, validation, points })
}

/// Compares the analysis with the expected values.
pub fn check(e: &ExpectedMultivariate, a: &MultivariateAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let names = a.output_names();
    let index = |n: &str| names.iter().position(|x| x == n);
    let mut push = |what: String, computed: Option<f64>, value: f64, tolerance: f64| {
        let computed = computed.unwrap_or(f64::NAN);
        out.push(crate::CheckOutcome {
            what,
            computed,
            expected: value,
            tolerance,
            passed: (computed - value).abs() <= tolerance,
        });
    };
    struct View<'a> {
        y: &'a [f64],
        u: &'a [f64],
        r: &'a [Vec<f64>],
        kp: f64,
        kq: f64,
        area: Option<f64>,
    }
    let views = [
        (
            "gum",
            e.gum.as_ref(),
            a.gum.as_ref().map(|g| View {
                y: &g.estimate,
                u: &g.standard_uncertainties,
                r: &g.correlation,
                kp: g.k_ellipsoid,
                kq: g.k_rectangle,
                area: None,
            }),
        ),
        (
            "mcm",
            e.mcm.as_ref(),
            a.mcm.as_ref().map(|m| View {
                y: &m.estimate,
                u: &m.standard_uncertainties,
                r: &m.correlation,
                kp: m.k_ellipsoid,
                kq: m.k_rectangle,
                area: m.smallest_region_area,
            }),
        ),
    ];
    for (tag, exp, view) in views {
        let Some(x) = exp else { continue };
        for (name, c) in &x.estimate {
            push(
                format!("multivariate.{tag}.estimate[{name}]"),
                view.as_ref().and_then(|v| index(name).map(|j| v.y[j])),
                c.value,
                c.tolerance,
            );
        }
        for (name, c) in &x.standard_uncertainty {
            push(
                format!("multivariate.{tag}.u[{name}]"),
                view.as_ref().and_then(|v| index(name).map(|j| v.u[j])),
                c.value,
                c.tolerance,
            );
        }
        for c in &x.correlations {
            let got = view.as_ref().and_then(|v| match (index(&c.between[0]), index(&c.between[1])) {
                (Some(i), Some(j)) => Some(v.r[i][j]),
                _ => None,
            });
            push(format!("multivariate.{tag}.r({}, {})", c.between[0], c.between[1]), got, c.value, c.tolerance);
        }
        if let Some(c) = &x.k_ellipsoid {
            push(format!("multivariate.{tag}.k_ellipsoid"), view.as_ref().map(|v| v.kp), c.value, c.tolerance);
        }
        if let Some(c) = &x.k_rectangle {
            push(format!("multivariate.{tag}.k_rectangle"), view.as_ref().map(|v| v.kq), c.value, c.tolerance);
        }
        if let Some(c) = &x.smallest_region_area {
            push(
                format!("multivariate.{tag}.smallest_region_area"),
                view.as_ref().and_then(|v| v.area),
                c.value,
                c.tolerance,
            );
        }
    }
    for (what, want, got) in [
        (
            "multivariate.ellipsoid_validated",
            e.ellipsoid_validated,
            a.validation.as_ref().map(|v| v.ellipsoid_validated),
        ),
        (
            "multivariate.rectangle_validated",
            e.rectangle_validated,
            a.validation.as_ref().map(|v| v.rectangle_validated),
        ),
    ] {
        if let Some(w) = want {
            let b = |x: bool| if x { 1.0 } else { 0.0 };
            push(what.into(), got.map(b), b(w), 0.0);
        }
    }
}

pub fn results_json(a: &MultivariateAnalysis) -> serde_json::Value {
    use serde_json::json;
    let names = a.output_names();
    json!({
        "outputs": a.model.outputs.iter().map(|o| json!({"name": o.name, "unit": o.unit, "expression": o.source})).collect::<Vec<_>>(),
        "implicit": a.model.implicit,
        "inputs": a.model.inputs.iter().map(|q| q.name.clone()).collect::<Vec<_>>(),
        "gum": a.gum.as_ref().map(|g| json!({
            "estimate": g.estimate,
            "standard_uncertainties": g.standard_uncertainties,
            "covariance": g.covariance,
            "correlation": g.correlation,
            "lambda_max": g.lambda_max,
            "sensitivity": g.sensitivity,
            "coverage_probability": g.coverage_probability,
            "k_ellipsoid": g.k_ellipsoid,
            "k_rectangle": g.k_rectangle,
        })),
        "mcm": a.mcm.as_ref().map(|m| json!({
            "estimate": m.estimate,
            "standard_uncertainties": m.standard_uncertainties,
            "covariance": m.covariance,
            "correlation": m.correlation,
            "lambda_max": m.lambda_max,
            "coverage_probability": m.coverage_probability,
            "k_ellipsoid": m.k_ellipsoid,
            "k_rectangle": m.k_rectangle,
            "smallest_region_area": m.smallest_region_area,
            "gum_ellipsoid_coverage": m.gum_ellipsoid_coverage,
            "gum_rectangle_coverage": m.gum_rectangle_coverage,
            "trials": m.trials,
            "runs": m.runs,
            "converged": m.converged,
            "seed": m.seed,
        })),
        "validation": a.validation.as_ref().map(|v| json!({
            "ndig": v.ndig,
            "delta": v.delta, "d_estimate": v.d_estimate, "d_uncertainty": v.d_uncertainty,
            "rho": v.rho, "d_lambda": v.d_lambda,
            "kappa_p": v.kappa_p, "d_kp": v.d_kp, "kappa_q": v.kappa_q, "d_kq": v.d_kq,
            "moments_validated": v.moments_validated,
            "ellipsoid_validated": v.ellipsoid_validated,
            "rectangle_validated": v.rectangle_validated,
        })),
        "names": names,
    })
}
