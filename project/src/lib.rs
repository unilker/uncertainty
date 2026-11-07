//! # uncertainty-project
//!
//! Project files (JSON) for *Uncertainty Calculations / Belirsizlik
//! Hesaplamaları*: reading and validating them ([`schema`]), running the GUM
//! and Monte Carlo evaluations, the top-down evaluation ([`topdown`]) or the
//! statistics of a proficiency test round ([`proficiency`]), a
//! calibration with uncertainties in both variables ([`calibration`]), a
//! conformity assessment ([`conformity`]), an interlaboratory precision
//! experiment ([`interlab`]), a model with several outputs
//! ([`multivariate`]) or a control chart ([`qc`]) defined in them ([`run`]), checking results
//! against expected reference values ([`check`]) and producing reports in
//! English or Turkish ([`report`]).

pub mod calibration;
pub mod conformity;
pub mod interlab;
pub mod multiconformity;
pub mod multivariate;
pub mod proficiency;
pub mod qc;
pub mod report;
pub mod sampling;
pub mod schema;
pub mod topdown;
pub mod weighing;

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
    let primary = [
        p.model.is_some(),
        p.topdown.is_some(),
        p.proficiency.is_some(),
        p.calibration.is_some(),
        p.precision_experiment.is_some(),
        p.multivariate.is_some(),
        p.control_chart.is_some(),
        p.sampling.is_some(),
        p.error_function.is_some(),
        p.multicomponent_conformity.is_some(),
    ]
    .iter()
    .filter(|&&b| b)
    .count();
    if primary > 1 || (primary == 0 && p.conformity.is_none()) {
        return Err(ProjectError::Format(
            "a project contains exactly one of the sections \"model\", \"topdown\", \"proficiency\", \"calibration\", \
             \"precision_experiment\", \"multivariate\", \"control_chart\", \"sampling\", \"error_function\", \"multicomponent_conformity\" and \"conformity\" (which may also accompany \"model\")"
                .into(),
        ));
    }
    if p.conformity.is_some() && primary == 1 && p.model.is_none() {
        return Err(ProjectError::Format("\"conformity\" can accompany a \"model\" section only".into()));
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
    for e in &spec.ensembles {
        b = b.ensemble(&e.iter().map(String::as_str).collect::<Vec<_>>());
    }
    b = b.output(&spec.output.name, &spec.output.expression);
    if let Some(u) = &spec.output.unit {
        b = b.output_unit(u);
    }
    Ok(b.build()?)
}

/// The expanded uncertainty to report: the evaluated U, or the CMC of the
/// laboratory if U is smaller (ILAC P14:09/2020, 5.5); true if raised.
pub fn reported_expanded(p: &Project, y: f64, u: f64) -> (f64, bool) {
    match p.reporting.cmc.as_ref().and_then(|c| c.at(y)) {
        Some(cmc) if cmc > u => (cmc, true),
        _ => (u, false),
    }
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
    /// Conformity assessment of the result (JCGM 106), if requested.
    pub conformity: Option<conformity::ConformityAnalysis>,
}

pub fn gum_options(spec: &GumSpec) -> GumOptions {
    GumOptions {
        coverage: match (spec.coverage_factor, spec.coverage_probability, spec.dominant_rectangular) {
            (Some(k), _, _) => CoverageSpec::Factor(k),
            (None, p, true) => CoverageSpec::DominantRectangular(p.unwrap_or(0.95)),
            (None, Some(p), false) => CoverageSpec::Probability(p),
            (None, None, false) => CoverageSpec::Probability(0.95),
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
    /// Conformity assessment study (JCGM 106) without a model.
    Conformity(Box<conformity::ConformityAnalysis>),
    /// Interlaboratory precision experiment (ISO 5725-2).
    Interlab(Box<interlab::InterlabAnalysis>),
    /// Model with several output quantities (JCGM 102).
    Multivariate(Box<multivariate::MultivariateAnalysis>),
    /// Control chart for internal quality control (Nordtest TR 569).
    ControlChart(Box<qc::ControlChartAnalysis>),
    /// Measurement uncertainty arising from sampling (Eurachem UfS:2019).
    Sampling(Box<sampling::SamplingAnalysis>),
    /// Error approximation function of a weighing instrument (EURAMET cg-18).
    ErrorFunction(Box<weighing::ErrorFunctionAnalysis>),
    /// Total risks for an item with several correlated properties (EMUE E2.1).
    Multicomponent(Box<multiconformity::MulticomponentAnalysis>),
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

/// The interval [y − U₋, y + U₊] of a model result with an uncorrected bias
/// (`reporting.uncorrected_bias`): the strategy, U₋ and U₊.
pub fn uncorrected_bias_interval(p: &Project, a: &Analysis) -> Option<(schema::BiasStrategySpec, f64, f64)> {
    let b = p.reporting.uncorrected_bias.as_ref()?;
    let g = a.gum.as_ref()?;
    uncertainty_core::bias::uncorrected_interval(
        g.combined_standard_uncertainty,
        -b.bias,
        b.u_bias,
        g.coverage_factor,
        b.strategy.strategy(),
    )
    .ok()
    .map(|(lo, hi)| (b.strategy, lo, hi))
}

/// Runs the evaluation a project describes.
pub fn run(p: &Project, overrides: &Overrides) -> Result<Evaluation, ProjectError> {
    if let Some(b) = &p.reporting.uncorrected_bias {
        if p.model.is_none() {
            return Err(input_err("reporting.uncorrected_bias", "applies to model projects only"));
        }
        if !(b.bias.is_finite() && b.u_bias.is_finite() && b.u_bias >= 0.0) {
            return Err(input_err("reporting.uncorrected_bias", "finite bias and non-negative u_bias required"));
        }
    }
    if let Some(spec) = &p.topdown {
        return Ok(Evaluation::TopDown(Box::new(topdown::evaluate(spec)?)));
    }
    if let Some(spec) = &p.proficiency {
        return Ok(Evaluation::Proficiency(Box::new(proficiency::evaluate(spec)?)));
    }
    if let Some(spec) = &p.calibration {
        return Ok(Evaluation::Calibration(Box::new(calibration::evaluate(spec)?)));
    }
    if let Some(spec) = &p.multivariate {
        return Ok(Evaluation::Multivariate(Box::new(multivariate::evaluate(spec, &p.analysis, overrides)?)));
    }
    if let Some(spec) = &p.precision_experiment {
        return Ok(Evaluation::Interlab(Box::new(interlab::evaluate(spec)?)));
    }
    if let Some(spec) = &p.control_chart {
        return Ok(Evaluation::ControlChart(Box::new(qc::evaluate(spec)?)));
    }
    if let Some(spec) = &p.sampling {
        return Ok(Evaluation::Sampling(Box::new(sampling::evaluate(spec)?)));
    }
    if let Some(spec) = &p.multicomponent_conformity {
        return Ok(Evaluation::Multicomponent(Box::new(multiconformity::evaluate(spec)?)));
    }
    if let Some(spec) = &p.error_function {
        return Ok(Evaluation::ErrorFunction(Box::new(weighing::evaluate(spec)?)));
    }
    if let (Some(spec), None) = (&p.conformity, &p.model) {
        return Ok(Evaluation::Conformity(Box::new(conformity::evaluate(spec, None)?)));
    }
    Ok(Evaluation::Model(Box::new(run_model(p, overrides)?)))
}

/// Runs the GUM and Monte Carlo evaluations of a model project.
pub fn run_model(p: &Project, overrides: &Overrides) -> Result<Analysis, ProjectError> {
    if p.analysis.gum.as_ref().is_some_and(|g| g.dominant_rectangular && g.coverage_factor.is_some()) {
        return Err(input_err("analysis.gum.dominant_rectangular", "cannot be combined with coverage_factor"));
    }
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
                keep_values: p.conformity.is_some(),
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
    let mut mcm = mcm;
    let conformity = match (&p.conformity, &gum) {
        (Some(spec), Some(g)) => Some(conformity::evaluate(
            spec,
            Some(conformity::Subject {
                name: model.output_name.clone(),
                unit: model.output_unit.clone(),
                value: g.estimate,
                u: g.combined_standard_uncertainty,
                dof: Some(g.effective_dof),
                values: mcm.as_ref().and_then(|m| m.values.as_deref()),
            }),
        )?),
        (Some(_), None) => {
            return Err(input_err("conformity", "needs the GUM evaluation of the model"));
        }
        _ => None,
    };
    // The Monte Carlo values were kept only for the conformity assessment.
    if let Some(m) = mcm.as_mut() {
        m.values = None;
    }
    Ok(Analysis { model, gum, mcm, validation, interval_kind, sensitivities, kragten, conformity })
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
        Evaluation::Conformity(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.conformity.as_ref()) {
                conformity::check(exp, a, &mut out);
            }
            out
        }
        Evaluation::Interlab(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.precision_experiment.as_ref()) {
                interlab::check(exp, a, &mut out);
            }
            out
        }
        Evaluation::Multivariate(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.multivariate.as_ref()) {
                multivariate::check(exp, a, &mut out);
            }
            out
        }
        Evaluation::ControlChart(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.control_chart.as_ref()) {
                qc::check(exp, a, &mut out);
            }
            out
        }
        Evaluation::Sampling(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.sampling.as_ref()) {
                sampling::check(exp, a, &mut out);
            }
            out
        }
        Evaluation::ErrorFunction(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.error_function.as_ref()) {
                weighing::check(exp, a, &mut out);
            }
            out
        }
        Evaluation::Multicomponent(a) => {
            let mut out = Vec::new();
            if let Some(exp) = p.expected.as_ref().and_then(|x| x.multicomponent_conformity.as_ref()) {
                multiconformity::check(exp, a, &mut out);
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
        push(
            "gum.reported_expanded_uncertainty",
            g.map(|g| reported_expanded(p, g.estimate, g.expanded_uncertainty).0),
            &e.reported_expanded_uncertainty,
        );
        push("gum.coverage_factor", g.map(|g| g.coverage_factor), &e.coverage_factor);
        push("gum.effective_dof", g.map(|g| g.effective_dof), &e.effective_dof);
        push(
            "gum.kragten_uncertainty",
            a.kragten.as_ref().map(|k| k.combined_standard_uncertainty),
            &e.kragten_uncertainty,
        );
    }
    // Monte Carlo reference values are not compared when the Monte Carlo
    // method was not run (`--no-mcm`).
    if let (Some(e), true) = (&exp.mcm, a.mcm.is_some()) {
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
    if let (Some(e), Some(c)) = (&exp.conformity, &a.conformity) {
        conformity::check(e, c, &mut out);
    }
    out
}

/// Stable identifier of a GUM warning in machine-readable results.
pub fn warning_code(w: &gum::GumWarning) -> &'static str {
    match w {
        gum::GumWarning::HigherOrderNonGaussian => "higher_order_non_gaussian",
        gum::GumWarning::WelchSatterthwaiteCorrelated => "welch_satterthwaite_correlated",
        gum::GumWarning::ZeroSensitivity { .. } => "zero_sensitivity",
        gum::GumWarning::NonlinearEstimate { .. } => "nonlinear_estimate",
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
        Evaluation::Conformity(a) => {
            return json!({
                "software": software,
                "project": project,
                "output": { "name": a.quantity, "unit": a.unit },
                "conformity": conformity::results_json(a),
                "checks": checks,
            });
        }
        Evaluation::Interlab(a) => {
            return json!({
                "software": software,
                "project": project,
                "output": { "name": a.quantity, "unit": a.unit },
                "precision_experiment": interlab::results_json(a),
                "checks": checks,
            });
        }
        Evaluation::Multivariate(a) => {
            return json!({
                "software": software,
                "project": project,
                "output": { "name": a.output_names().join(", "), "unit": serde_json::Value::Null },
                "multivariate": multivariate::results_json(a),
                "checks": checks,
            });
        }
        Evaluation::ControlChart(a) => {
            return json!({
                "software": software,
                "project": project,
                "output": { "name": a.quantity, "unit": a.unit },
                "control_chart": qc::results_json(a),
                "checks": checks,
            });
        }
        Evaluation::Sampling(a) => {
            return json!({
                "software": software,
                "project": project,
                "output": { "name": a.quantity, "unit": a.unit },
                "sampling": sampling::results_json(a),
                "checks": checks,
            });
        }
        Evaluation::ErrorFunction(a) => {
            return json!({
                "software": software,
                "project": project,
                "output": { "name": a.instrument, "unit": a.error_unit },
                "error_function": weighing::results_json(a),
                "checks": checks,
            });
        }
        Evaluation::Multicomponent(a) => {
            return json!({
                "software": software,
                "project": project,
                "output": { "name": a.names.join(", "), "unit": a.unit },
                "multicomponent_conformity": multiconformity::results_json(a),
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
            "second_order_estimate_term": g.second_order_estimate_term,
            "correlation_variance": g.correlation_variance,
            "effective_dof": if g.effective_dof.is_finite() { json!(g.effective_dof) } else { json!("inf") },
            "coverage_factor": g.coverage_factor,
            "coverage_probability": g.coverage_probability,
            "dominant_rectangular": g.dominant_rectangular.as_ref().map(|(name, r)| json!({"input": name, "ratio_uN_uR": r})),
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
            "expanded_about_estimate": m.expanded_about_estimate,
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
        "conformity": a.conformity.as_ref().map(conformity::results_json),
        "uncorrected_bias": uncorrected_bias_interval(p, a).map(|(s, lo, hi)| json!({
            "strategy": s.label(), "bias": p.reporting.uncorrected_bias.as_ref().map(|b| b.bias),
            "u_minus": lo, "u_plus": hi,
        })),
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
    fn uncorrected_bias_statement() {
        let with = |strategy: &str| {
            let j = A1.replace(
                "\"expected\":",
                &format!(r#""reporting": {{ "uncorrected_bias": {{ "bias": 1.5, "u_bias": 0.3, "strategy": "{strategy}" }} }}, "expected":"#),
            );
            let p = parse(&j).unwrap();
            let e = run(&p, &Overrides { skip_mcm: true, ..Default::default() }).unwrap();
            let a = e.model().unwrap();
            (uncorrected_bias_interval(&p, a).unwrap(), report::reported_statements(&p, &e, Lang::En))
        };
        // u_c = √(0.8633² + 0.3²), Δ = −1.5: SUMU moves the interval down.
        let uc = (0.8633f64.powi(2) + 0.09).sqrt();
        let ((_, lo, hi), st) = with("sumu");
        assert!((lo - (2.0 * uc + 1.5)).abs() < 2e-3 && (hi - (2.0 * uc - 1.5)).abs() < 2e-3, "{lo} {hi}");
        assert!(st[1].contains("1002.7 mg/L (−3.3, +0.3)"), "{}", st[1]);
        let ((_, lo, hi), st) = with("rssu");
        assert_eq!(lo, hi);
        assert!((lo - 2.0 * (0.8633f64.powi(2) + 2.25 + 0.09).sqrt()).abs() < 2e-3);
        assert!(st[1].contains("(1002.7 ± 3.5) mg/L") && st[1].contains("RSSu"), "{}", st[1]);
        let td = parse(&A1.replace(
            "\"expected\":",
            r#""reporting": { "uncorrected_bias": { "bias": 1, "u_bias": -1 } }, "expected":"#,
        ))
        .unwrap();
        assert!(matches!(run(&td, &Overrides::default()), Err(ProjectError::Input { .. })));
    }

    #[test]
    fn conformity_of_a_model_result() {
        let j = A1.replace(
            "\"expected\":",
            r#""conformity": { "lower": 1000, "upper": 1005, "rule": "guarded_acceptance" }, "expected":"#,
        );
        let p = parse(&j).unwrap();
        let e = run(&p, &Overrides::default()).unwrap();
        let a = e.model().unwrap();
        let c = a.conformity.as_ref().unwrap();
        let g = a.gum.as_ref().unwrap();
        // w = U = 2u_c on each side; p_c from N(y, u_c²) and from the Monte Carlo values.
        assert!((c.acceptance.upper.unwrap() - (1005.0 - 2.0 * g.combined_standard_uncertainty)).abs() < 1e-9);
        let r = &c.results[0];
        let pc = uncertainty_core::special::normal_cdf((1005.0 - g.estimate) / g.combined_standard_uncertainty)
            - uncertainty_core::special::normal_cdf((1000.0 - g.estimate) / g.combined_standard_uncertainty);
        assert!((r.conformance_probability - pc).abs() < 1e-12 && r.accepted);
        assert!((r.conformance_probability_mcm.unwrap() - pc).abs() < 0.003, "{r:?}");
        assert!(a.mcm.as_ref().unwrap().values.is_none());
        let st = report::reported_statements(&p, &e, Lang::En);
        assert!(st.last().unwrap().starts_with("Probability that c conforms to 1000 mg/L ≤ c ≤ 1005 mg/L"), "{st:?}");
        // conformity cannot accompany a top-down project.
        let bad = r#"{"format": "uncertainty-calculations/project", "version": 1, "metadata": {"title": {"en": "x", "tr": "x"}},
            "topdown": {"quantity": "c", "precision": [], "bias": []}, "conformity": {"upper": 1}}"#;
        assert!(matches!(parse(bad), Err(ProjectError::Format(_))));
    }

    #[test]
    fn precision_experiment_exclusions_and_validation() {
        let base = r#"{"format": "uncertainty-calculations/project", "version": 1, "metadata": {"title": "x"},
            "precision_experiment": {"quantity": "w", "laboratories": ["A", "B", "C", "D"],
              "levels": [{"name": "1", "results": [[1.0, 1.2], [1.1, 1.3], [0.9, 1.0], [2.0, 2.1, 5.0]]}]
              EXCLUDE }}"#;
        let p = parse(&base.replace("EXCLUDE", "")).unwrap();
        let a = match run(&p, &Overrides::default()).unwrap() {
            Evaluation::Interlab(a) => a,
            _ => panic!("not an interlaboratory evaluation"),
        };
        let l = &a.levels[0];
        assert_eq!((l.estimates.p, l.initial.n, l.excluded.len()), (4, 2, 0));
        assert!(l.final_scrutiny.is_none() && a.relationships.is_empty());
        // A single result of laboratory D, then the whole laboratory.
        let one = base.replace("EXCLUDE", r#", "exclude": [{"laboratory": "D", "level": "1", "result": 3}]"#);
        let a = match run(&parse(&one).unwrap(), &Overrides::default()).unwrap() {
            Evaluation::Interlab(a) => a,
            _ => unreachable!(),
        };
        let l = &a.levels[0];
        assert_eq!(l.retained[3].unwrap().n, 2);
        assert!((l.retained[3].unwrap().mean - 2.05).abs() < 1e-12);
        assert_eq!(l.excluded, vec![3]);
        assert!(l.final_scrutiny.is_some());
        let lab = base.replace("EXCLUDE", r#", "exclude": [{"laboratory": "D"}]"#);
        let a = match run(&parse(&lab).unwrap(), &Overrides::default()).unwrap() {
            Evaluation::Interlab(a) => a,
            _ => unreachable!(),
        };
        assert_eq!(a.levels[0].estimates.p, 3);
        for bad in [
            r#", "exclude": [{"laboratory": "E"}]"#,
            r#", "exclude": [{"laboratory": "A", "result": 1}]"#,
            r#", "exclude": [{"laboratory": "A", "level": "1", "result": 3}]"#,
        ] {
            let p = parse(&base.replace("EXCLUDE", bad)).unwrap();
            assert!(matches!(run(&p, &Overrides::default()), Err(ProjectError::Input { .. })), "{bad}");
        }
        let short = base.replace(", [2.0, 2.1, 5.0]]", "]").replace("EXCLUDE", "");
        assert!(matches!(run(&parse(&short).unwrap(), &Overrides::default()), Err(ProjectError::Input { .. })));
        // precision_experiment is a primary section of its own.
        let both = base.replace("EXCLUDE", "").replace(
            r#""precision_experiment""#,
            r#""topdown": {"quantity": "c", "precision": [], "bias": []}, "precision_experiment""#,
        );
        assert!(matches!(parse(&both), Err(ProjectError::Format(_))));
    }

    #[test]
    fn multivariate_projects_and_errors() {
        let base = r#"{"format": "uncertainty-calculations/project", "version": 1, "metadata": {"title": "x"},
            "multivariate": {"outputs": OUT, "inputs": [{"name": "a", "distribution": {"type": "normal", "mean": 2, "sd": 0.1}},
              {"name": "b", "distribution": {"type": "normal", "mean": 3, "sd": 0.2}}] EXTRA },
            "analysis": {"mcm": {"trials": 20000}}}"#;
        let p = parse(
            &base
                .replace("OUT", r#"[{"name": "s", "expression": "a + b"}, {"name": "d", "expression": "a - b"}]"#)
                .replace("EXTRA", ""),
        )
        .unwrap();
        let e = run(&p, &Overrides::default()).unwrap();
        let Evaluation::Multivariate(a) = &e else { panic!("not multivariate") };
        let g = a.gum.as_ref().unwrap();
        // u² = 0.01 + 0.04 for both; r = (0.01 − 0.04)/0.05 = −0.6.
        assert!((g.correlation[0][1] + 0.6).abs() < 1e-12);
        assert!(a.mcm.is_some() && a.validation.is_some() && a.points.len() == multivariate::CHART_POINTS);
        assert_eq!(report::reported_statements(&p, &e, Lang::En).len(), 3);
        // An implicit model needs starting values.
        let imp = base
            .replace("OUT", r#"[{"name": "y", "expression": "y^2 - a"}]"#)
            .replace("EXTRA", r#", "implicit": true"#);
        assert!(run(&parse(&imp).unwrap(), &Overrides::default()).is_err());
        // Indications need more sets than quantities, and one unit per quantity.
        let few = base
            .replace("OUT", r#"[{"name": "s", "expression": "a + u + v"}]"#)
            .replace("EXTRA", r#", "indications": [{"inputs": ["u", "v"], "values": [[1, 2], [1.1, 2.1]]}]"#);
        assert!(run(&parse(&few).unwrap(), &Overrides::default()).is_err());
        let units = few.replace(r#""values""#, r#""units": ["V"], "values""#);
        assert!(matches!(run(&parse(&units).unwrap(), &Overrides::default()), Err(ProjectError::Input { .. })));
        // Higher-order terms do not apply.
        let ho = base
            .replace("OUT", r#"[{"name": "s", "expression": "a * b"}]"#)
            .replace("EXTRA", "")
            .replace(r#""analysis": {"mcm": {"trials": 20000}}"#, r#""analysis": {"gum": {"higher_order": true}}"#);
        assert!(matches!(run(&parse(&ho).unwrap(), &Overrides::default()), Err(ProjectError::Input { .. })));
    }

    #[test]
    fn type_a_from_observations() {
        let d = to_distribution("x", &DistributionSpec::TypeA { observations: vec![1.0, 2.0, 3.0] }).unwrap();
        assert_eq!(d, Distribution::StudentT { mean: 2.0, scale: 1.0 / 3f64.sqrt(), dof: 2.0 });
    }
}
