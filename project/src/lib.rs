//! # uncertainty-project
//!
//! Project files (JSON) for *Uncertainty Calculations / Belirsizlik
//! Hesaplamaları*: reading and validating them ([`schema`]), running the GUM
//! and Monte Carlo evaluations, the top-down evaluation ([`topdown`]) or the
//! statistics of a proficiency test round ([`proficiency`]) or a
//! calibration with uncertainties in both variables ([`calibration`])
//! defined in them ([`run`]), checking results
//! against expected reference values ([`check`]) and producing reports in
//! English or Turkish ([`report`]).

pub mod calibration;
pub mod proficiency;
pub mod report;
pub mod schema;
pub mod topdown;

use schema::*;
use std::fmt;
use uncertainty_core::distribution::TypeA;
use uncertainty_core::gum::{self, CoverageSpec, DofRounding, GumOptions, GumResult, Kragten};
use uncertainty_core::mcm::{self, IntervalKind, McmOptions, McmResult, Trials, Validation};
use uncertainty_core::{Distribution, InputQuantity, Model, UncError};

/// Software identification printed in every report (attribution terms,
/// ADDITIONAL-TERMS.md).
pub const SOFTWARE_NAME: &str = "Uncertainty Calculations (Belirsizlik Hesaplamaları)";
pub const SOFTWARE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CITATION: &str = "Ün, İ. Uncertainty Calculations (Belirsizlik Hesaplamaları): measurement uncertainty \
evaluation software. TÜBİTAK UME. https://github.com/unilker/uncertainty";

/// Report language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Lang {
    #[default]
    En,
    Tr,
}

impl std::str::FromStr for Lang {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "en" => Ok(Lang::En),
            "tr" => Ok(Lang::Tr),
            other => Err(format!("unknown language '{other}' (use en or tr)")),
        }
    }
}

#[derive(Debug)]
pub enum ProjectError {
    Json(serde_json::Error),
    Format(String),
    Input { name: String, message: String },
    Engine(UncError),
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectError::Json(e) => write!(f, "invalid project file: {e}"),
            ProjectError::Format(m) => write!(f, "invalid project file: {m}"),
            ProjectError::Input { name, message } => write!(f, "input '{name}': {message}"),
            ProjectError::Engine(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ProjectError {}

impl From<UncError> for ProjectError {
    fn from(e: UncError) -> Self {
        ProjectError::Engine(e)
    }
}

/// Parses and checks a project file.
pub fn parse(json: &str) -> Result<Project, ProjectError> {
    let p: Project = serde_json::from_str(json).map_err(ProjectError::Json)?;
    if p.format != FORMAT_ID {
        return Err(ProjectError::Format(format!("format must be \"{FORMAT_ID}\", found \"{}\"", p.format)));
    }
    if p.version != FORMAT_VERSION {
        return Err(ProjectError::Format(format!(
            "unsupported version {} (this program reads {FORMAT_VERSION})",
            p.version
        )));
    }
    if [p.model.is_some(), p.topdown.is_some(), p.proficiency.is_some(), p.calibration.is_some()]
        .iter()
        .filter(|&&b| b)
        .count()
        != 1
    {
        return Err(ProjectError::Format(
            "a project contains exactly one of the sections \"model\", \"topdown\", \"proficiency\" and \"calibration\""
                .into(),
        ));
    }
    Ok(p)
}

fn input_err(name: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: name.to_string(), message: message.into() }
}

fn limits(
    name: &str,
    lower: Option<f64>,
    upper: Option<f64>,
    value: Option<f64>,
    half_width: Option<f64>,
) -> Result<(f64, f64), ProjectError> {
    match (lower, upper, value, half_width) {
        (Some(l), Some(u), None, None) => Ok((l, u)),
        (None, None, Some(v), Some(a)) => Ok((v - a, v + a)),
        _ => Err(input_err(name, "give either lower and upper, or value and half_width")),
    }
}

fn std_or_expanded(name: &str, sd: Option<f64>, expanded: Option<f64>, k: Option<f64>) -> Result<f64, ProjectError> {
    match (sd, expanded, k) {
        (Some(s), None, None) => Ok(s),
        (None, Some(u), Some(k)) if k > 0.0 => Ok(u / k),
        _ => Err(input_err(name, "give either a standard uncertainty, or expanded and k")),
    }
}

/// Converts a distribution specification to the engine's distribution.
pub fn to_distribution(name: &str, d: &DistributionSpec) -> Result<Distribution, ProjectError> {
    Ok(match *d {
        DistributionSpec::Constant { value } => Distribution::Constant { value },
        DistributionSpec::Normal { mean, sd, expanded, k } => {
            Distribution::Normal { mean, sd: std_or_expanded(name, sd, expanded, k)? }
        }
        DistributionSpec::T { mean, scale, expanded, k, dof } => {
            Distribution::StudentT { mean, scale: std_or_expanded(name, scale, expanded, k)?, dof }
        }
        DistributionSpec::Rectangular { lower, upper, value, half_width } => {
            let (lower, upper) = limits(name, lower, upper, value, half_width)?;
            Distribution::Rectangular { lower, upper }
        }
        DistributionSpec::Triangular { lower, upper, value, half_width } => {
            let (lower, upper) = limits(name, lower, upper, value, half_width)?;
            Distribution::Triangular { lower, upper }
        }
        DistributionSpec::Arcsine { lower, upper, value, half_width } => {
            let (lower, upper) = limits(name, lower, upper, value, half_width)?;
            Distribution::Arcsine { lower, upper }
        }
        DistributionSpec::Trapezoidal { lower, upper, value, half_width, beta } => {
            let (lower, upper) = limits(name, lower, upper, value, half_width)?;
            Distribution::Trapezoidal { lower, upper, beta }
        }
        DistributionSpec::CurvilinearTrapezoid { lower, upper, value, half_width, d } => {
            let (lower, upper) = limits(name, lower, upper, value, half_width)?;
            Distribution::CurvilinearTrapezoid { lower, upper, d }
        }
        DistributionSpec::Exponential { mean } => Distribution::Exponential { mean },
        DistributionSpec::Gamma { shape, scale } => Distribution::Gamma { shape, scale },
        DistributionSpec::TypeA { ref observations } => TypeA::from_observations(observations)?.distribution(),
        DistributionSpec::TypeAPooled { mean, pooled_sd, pooled_dof, n } => {
            if n == 0 {
                return Err(input_err(name, "n must be at least 1"));
            }
            TypeA::pooled(mean, pooled_sd, pooled_dof, n).distribution()
        }
    })
}

/// Builds the engine model described by a project.
pub fn build_model(p: &Project) -> Result<Model, ProjectError> {
    let spec = p.model.as_ref().ok_or_else(|| ProjectError::Format("the project has no \"model\" section".into()))?;
    let mut b = Model::builder();
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
    for i in &spec.intermediates {
        b = b.intermediate(&i.name, &i.expression);
    }
    for c in &spec.correlations {
        b = b.correlation(&c.a, &c.b, c.r);
    }
    b = b.output(&spec.output.name, &spec.output.expression);
    if let Some(u) = &spec.output.unit {
        b = b.output_unit(u);
    }
    Ok(b.build()?)
}

/// Options that may override the project file (e.g. from the command line).
#[derive(Clone, Debug, Default)]
pub struct Overrides {
    pub skip_mcm: bool,
    pub trials: Option<usize>,
    pub seed: Option<u64>,
}

/// Everything computed for one project.
#[derive(Clone, Debug)]
pub struct Analysis {
    pub model: Model,
    pub gum: Option<GumResult>,
    pub mcm: Option<McmResult>,
    pub validation: Option<Validation>,
    pub interval_kind: IntervalKind,
    /// Symbolic sensitivity coefficients ∂f/∂x_i.
    pub sensitivities: Vec<String>,
    /// Kragten's spreadsheet approximation, if requested.
    pub kragten: Option<Kragten>,
}

pub fn gum_options(spec: &GumSpec) -> GumOptions {
    GumOptions {
        coverage: match (spec.coverage_factor, spec.coverage_probability) {
            (Some(k), _) => CoverageSpec::Factor(k),
            (None, Some(p)) => CoverageSpec::Probability(p),
            (None, None) => CoverageSpec::Probability(0.95),
        },
        higher_order: spec.higher_order,
        dof_rounding: match spec.dof_rounding {
            DofRoundingSpec::Truncate => DofRounding::Truncate,
            DofRoundingSpec::Exact => DofRounding::Exact,
        },
    }
}

/// The outcome of running a project.
#[derive(Clone, Debug)]
pub enum Evaluation {
    /// GUM and Monte Carlo evaluation of a measurement model.
    Model(Box<Analysis>),
    /// Top-down evaluation from validation data.
    TopDown(Box<topdown::TopDownAnalysis>),
    /// Statistics of one round of a proficiency testing scheme.
    Proficiency(Box<proficiency::ProficiencyAnalysis>),
    /// Calibration with uncertainties in both variables (ISO 6143).
    Calibration(Box<calibration::CalibrationAnalysis>),
}

impl Evaluation {
    pub fn model(&self) -> Option<&Analysis> {
        match self {
            Evaluation::Model(a) => Some(a),
            _ => None,
        }
    }

    pub fn topdown(&self) -> Option<&topdown::TopDownAnalysis> {
        match self {
            Evaluation::TopDown(t) => Some(t),
            _ => None,
        }
    }

    pub fn proficiency(&self) -> Option<&proficiency::ProficiencyAnalysis> {
        match self {
            Evaluation::Proficiency(a) => Some(a),
            _ => None,
        }
    }

    pub fn calibration(&self) -> Option<&calibration::CalibrationAnalysis> {
        match self {
            Evaluation::Calibration(a) => Some(a),
            _ => None,
        }
    }
}

/// Runs the evaluation a project describes.
pub fn run(p: &Project, overrides: &Overrides) -> Result<Evaluation, ProjectError> {
    if let Some(spec) = &p.topdown {
        return Ok(Evaluation::TopDown(Box::new(topdown::evaluate(spec)?)));
    }
    if let Some(spec) = &p.proficiency {
        return Ok(Evaluation::Proficiency(Box::new(proficiency::evaluate(spec)?)));
    }
    if let Some(spec) = &p.calibration {
        return Ok(Evaluation::Calibration(Box::new(calibration::evaluate(spec)?)));
    }
    Ok(Evaluation::Model(Box::new(run_model(p, overrides)?)))
}

/// Runs the GUM and Monte Carlo evaluations of a model project.
pub fn run_model(p: &Project, overrides: &Overrides) -> Result<Analysis, ProjectError> {
    let model = build_model(p)?;
    let gum = match &p.analysis.gum {
        Some(spec) => Some(gum::evaluate(&model, &gum_options(spec))?),
        None => None,
    };
    let mut interval_kind = IntervalKind::Shortest;
    let mut validation = None;
    let mcm = match (&p.analysis.mcm, overrides.skip_mcm) {
        (Some(spec), false) => {
            interval_kind = match spec.interval {
                IntervalSpec::Shortest => IntervalKind::Shortest,
                IntervalSpec::Symmetric => IntervalKind::Symmetric,
            };
            let p_cov =
                spec.coverage_probability.or_else(|| gum.as_ref().map(|g| g.coverage_probability)).unwrap_or(0.95);
            let trials = if let Some(m) = overrides.trials {
                Trials::Fixed(m)
            } else if let Some(a) = &spec.adaptive {
                Trials::Adaptive { ndig: a.ndig, tolerance_divisor: a.tolerance_divisor, max_trials: a.max_trials }
            } else {
                Trials::Fixed(spec.trials.unwrap_or(1_000_000))
            };
            let opts = McmOptions {
                trials,
                coverage_probability: p_cov,
                seed: overrides.seed.unwrap_or(spec.seed),
                interval: interval_kind,
                histogram_bins: 60,
                keep_values: false,
            };
            let r = mcm::evaluate(&model, &opts)?;
            if let Some(g) = &gum {
                if (g.coverage_probability - p_cov).abs() < 1e-12 {
                    validation = Some(mcm::validate_gum(g, &r, spec.validation_ndig, interval_kind));
                }
            }
            Some(r)
        }
        _ => None,
    };
    let sensitivities = gum::sensitivity_expressions(&model);
    let kragten = match &p.analysis.gum {
        Some(spec) if spec.kragten => Some(gum::kragten(&model)?),
        _ => None,
    };
    Ok(Analysis { model, gum, mcm, validation, interval_kind, sensitivities, kragten })
}

/// Result of comparing one computed value with an expected reference value.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckOutcome {
    pub what: String,
    pub computed: f64,
    pub expected: f64,
    pub tolerance: f64,
    pub passed: bool,
}

/// Compares an evaluation with the project's expected values. Returns an
/// empty list if the project states none.
pub fn check(p: &Project, e: &Evaluation) -> Vec<CheckOutcome> {
    match e {
        Evaluation::Model(a) => check_model(p, a),
        Evaluation::TopDown(t) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.topdown.as_ref()) {
                topdown::check(exp, t, &mut out);
            }
            out
        }
        Evaluation::Proficiency(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.proficiency.as_ref()) {
                proficiency::check(exp, a, &mut out);
            }
            out
        }
        Evaluation::Calibration(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.calibration.as_ref()) {
                calibration::check(exp, a, &mut out);
            }
            out
        }
    }
}

fn check_model(p: &Project, a: &Analysis) -> Vec<CheckOutcome> {
    let mut out = Vec::new();
    let Some(exp) = &p.expected else { return out };
    let mut push = |what: &str, computed: Option<f64>, c: &Option<Check>| {
        if let Some(c) = c {
            let computed = computed.unwrap_or(f64::NAN);
            out.push(CheckOutcome {
                what: what.to_string(),
                computed,
                expected: c.value,
                tolerance: c.tolerance,
                passed: (computed - c.value).abs() <= c.tolerance,
            });
        }
    };
    if let Some(e) = &exp.gum {
        let g = a.gum.as_ref();
        push("gum.estimate", g.map(|g| g.estimate), &e.estimate);
        push("gum.standard_uncertainty", g.map(|g| g.combined_standard_uncertainty), &e.standard_uncertainty);
        push("gum.expanded_uncertainty", g.map(|g| g.expanded_uncertainty), &e.expanded_uncertainty);
        push("gum.coverage_factor", g.map(|g| g.coverage_factor), &e.coverage_factor);
        push("gum.effective_dof", g.map(|g| g.effective_dof), &e.effective_dof);
        push(
            "gum.kragten_uncertainty",
            a.kragten.as_ref().map(|k| k.combined_standard_uncertainty),
            &e.kragten_uncertainty,
        );
    }
    if let Some(e) = &exp.mcm {
        let m = a.mcm.as_ref();
        let iv = m.map(|m| m.interval(a.interval_kind));
        push("mcm.estimate", m.map(|m| m.estimate), &e.estimate);
        push("mcm.standard_uncertainty", m.map(|m| m.standard_uncertainty), &e.standard_uncertainty);
        push("mcm.interval_low", iv.map(|i| i.0), &e.interval_low);
        push("mcm.interval_high", iv.map(|i| i.1), &e.interval_high);
        if let Some(v) = e.gum_validated {
            let got = a.validation.as_ref().map(|x| x.validated);
            out.push(CheckOutcome {
                what: "mcm.gum_validated".into(),
                computed: got.map(|b| b as u8 as f64).unwrap_or(f64::NAN),
                expected: v as u8 as f64,
                tolerance: 0.0,
                passed: got == Some(v),
            });
        }
    }
    out
}

/// Stable identifier of a GUM warning in machine-readable results.
pub fn warning_code(w: &gum::GumWarning) -> &'static str {
    match w {
        gum::GumWarning::HigherOrderNonGaussian => "higher_order_non_gaussian",
        gum::GumWarning::WelchSatterthwaiteCorrelated => "welch_satterthwaite_correlated",
        gum::GumWarning::ZeroSensitivity { .. } => "zero_sensitivity",
    }
}

/// Machine-readable results (JSON).
pub fn results_json(p: &Project, e: &Evaluation) -> serde_json::Value {
    use serde_json::json;
    let checks: Vec<_> = check(p, e)
        .into_iter()
        .map(|c| json!({"what": c.what, "computed": c.computed, "expected": c.expected, "tolerance": c.tolerance, "passed": c.passed}))
        .collect();
    let software = json!({ "name": SOFTWARE_NAME, "version": SOFTWARE_VERSION, "citation": CITATION });
    let project = json!({ "id": p.metadata.id, "title": p.metadata.title.en });
    let a = match e {
        Evaluation::Model(a) => a,
        Evaluation::TopDown(t) => {
            let spec = p.topdown.as_ref();
            return json!({
                "software": software,
                "project": project,
                "output": { "name": spec.map(|s| &s.quantity), "unit": spec.and_then(|s| s.unit.as_ref()) },
                "topdown": topdown::results_json(t),
                "checks": checks,
            });
        }
        Evaluation::Proficiency(a) => {
            let spec = p.proficiency.as_ref();
            return json!({
                "software": software,
                "project": project,
                "output": { "name": spec.map(|s| &s.quantity), "unit": spec.and_then(|s| s.unit.as_ref()) },
                "proficiency": proficiency::results_json(a),
                "checks": checks,
            });
        }
        Evaluation::Calibration(a) => {
            let spec = p.calibration.as_ref();
            return json!({
                "software": software,
                "project": project,
                "output": { "name": spec.map(|s| &s.analyte), "unit": spec.and_then(|s| s.unit.as_ref()) },
                "calibration": calibration::results_json(a),
                "checks": checks,
            });
        }
    };
    let gum = a.gum.as_ref().map(|g| {
        json!({
            "estimate": g.estimate,
            "standard_uncertainty": g.combined_standard_uncertainty,
            "first_order_uncertainty": g.first_order_uncertainty,
            "higher_order_variance": g.higher_order_variance,
            "correlation_variance": g.correlation_variance,
            "effective_dof": if g.effective_dof.is_finite() { json!(g.effective_dof) } else { json!("inf") },
            "coverage_factor": g.coverage_factor,
            "coverage_probability": g.coverage_probability,
            "expanded_uncertainty": g.expanded_uncertainty,
            "interval": [g.interval.0, g.interval.1],
            "budget": g.budget.iter().zip(&a.sensitivities).map(|(r, s)| json!({
                "name": r.name,
                "estimate": r.estimate,
                "standard_uncertainty": r.standard_uncertainty,
                "distribution": r.distribution,
                "dof": if r.dof.is_finite() { json!(r.dof) } else { json!("inf") },
                "sensitivity": r.sensitivity,
                "sensitivity_expression": s,
                "contribution": r.contribution,
                "variance_share": r.variance_share,
            })).collect::<Vec<_>>(),
            "warnings": g.warnings.iter().map(|w| json!({ "code": warning_code(w), "message": w.to_string() })).collect::<Vec<_>>(),
            "kragten": a.kragten.as_ref().map(|k| json!({
                "contributions": k.contributions,
                "combined_standard_uncertainty": k.combined_standard_uncertainty,
            })),
        })
    });
    let mcm = a.mcm.as_ref().map(|m| {
        json!({
            "trials": m.trials,
            "runs": m.runs,
            "converged": m.converged,
            "seed": m.seed,
            "estimate": m.estimate,
            "standard_uncertainty": m.standard_uncertainty,
            "coverage_probability": m.coverage_probability,
            "symmetric_interval": [m.symmetric_interval.0, m.symmetric_interval.1],
            "shortest_interval": [m.shortest_interval.0, m.shortest_interval.1],
            "median": m.median,
            "skewness": m.skewness,
            "excess_kurtosis": m.excess_kurtosis,
            "histogram": {
                "lower": m.histogram.lower,
                "bin_width": m.histogram.bin_width,
                "counts": m.histogram.counts,
                "underflow": m.histogram.underflow,
                "overflow": m.histogram.overflow,
            },
        })
    });
    // GUM probability density over the histogram range, for plotting.
    let gum_pdf = match (&a.gum, &a.mcm) {
        (Some(g), Some(m)) if g.combined_standard_uncertainty > 0.0 && m.histogram.bin_width > 0.0 => {
            let x0 = m.histogram.lower;
            let x1 = x0 + m.histogram.bin_width * m.histogram.counts.len() as f64;
            let u = g.combined_standard_uncertainty;
            let pts: Vec<[f64; 2]> = (0..=200)
                .map(|i| {
                    let x = x0 + (x1 - x0) * i as f64 / 200.0;
                    let z = (x - g.estimate) / u;
                    let d = if g.effective_dof.is_finite() {
                        uncertainty_core::special::student_t_pdf(z, g.effective_dof)
                    } else {
                        uncertainty_core::special::normal_pdf(z)
                    };
                    [x, d / u]
                })
                .collect();
            Some(pts)
        }
        _ => None,
    };
    let validation = a.validation.as_ref().map(|v| {
        json!({"ndig": v.ndig, "tolerance": v.tolerance, "d_low": v.d_low, "d_high": v.d_high, "validated": v.validated})
    });
    json!({
        "software": software,
        "project": project,
        "output": { "name": a.model.output_name, "unit": a.model.output_unit },
        "gum": gum,
        "mcm": mcm,
        "validation": validation,
        "gum_pdf": gum_pdf,
        "checks": checks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const A1: &str = r#"{
      "format": "uncertainty-calculations/project",
      "version": 1,
      "metadata": { "title": { "en": "Cd standard", "tr": "Cd standardı" } },
      "model": {
        "output": { "name": "c", "unit": "mg/L", "expression": "1000 * m * P / V" },
        "inputs": [
          { "name": "m", "unit": "mg", "distribution": { "type": "normal", "mean": 100.28, "sd": 0.05 } },
          { "name": "P", "distribution": { "type": "rectangular", "value": 0.9999, "half_width": 0.0001 } },
          { "name": "V", "unit": "mL", "distribution": { "type": "normal", "mean": 100.0, "expanded": 0.14, "k": 2 } }
        ]
      },
      "analysis": { "gum": { "coverage_factor": 2 }, "mcm": { "trials": 100000 } },
      "expected": { "gum": { "estimate": { "value": 1002.7, "tolerance": 0.05 },
                             "standard_uncertainty": { "value": 0.864, "tolerance": 0.001 } } }
    }"#;

    #[test]
    fn parse_run_and_check() {
        let p = parse(A1).unwrap();
        let e = run(&p, &Overrides::default()).unwrap();
        let checks = check(&p, &e);
        let a = e.model().unwrap();
        assert_eq!(checks.len(), 2);
        assert!(checks.iter().all(|c| c.passed), "{checks:?}");
        // k = 2 with ν_eff = ∞ gives p = 95.45 %, which the MCM inherits, so validation is performed.
        assert!(a.validation.is_some());
        let j = results_json(&p, &e);
        assert_eq!(j["gum"]["budget"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn rejects_bad_files() {
        assert!(matches!(parse("{}"), Err(ProjectError::Json(_))));
        let wrong = A1.replace("\"version\": 1", "\"version\": 2");
        assert!(matches!(parse(&wrong), Err(ProjectError::Format(_))));
        let unknown = A1.replace("\"unit\": \"mg\",", "\"unit\": \"mg\", \"colour\": \"red\",");
        assert!(matches!(parse(&unknown), Err(ProjectError::Json(_))));
        let both = A1.replace("\"value\": 0.9999, \"half_width\": 0.0001", "\"lower\": 0.9998, \"value\": 0.9999");
        let p = parse(&both).unwrap();
        assert!(matches!(run(&p, &Overrides::default()), Err(ProjectError::Input { .. })));
    }

    #[test]
    fn type_a_from_observations() {
        let d = to_distribution("x", &DistributionSpec::TypeA { observations: vec![1.0, 2.0, 3.0] }).unwrap();
        assert_eq!(d, Distribution::StudentT { mean: 2.0, scale: 1.0 / 3f64.sqrt(), dof: 2.0 });
    }
}
