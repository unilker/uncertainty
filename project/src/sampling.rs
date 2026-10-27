//! Measurement uncertainty arising from sampling after the Eurachem Guide
//! UfS:2019: the duplicate method with a classical nested ANOVA or the range
//! method, optionally on log-transformed values, and the modelling approach
//! with the fundamental sampling error of Gy's sampling theory.

use crate::schema::Check;
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use uncertainty_core::sampling::{self, GyMaterial, GyStep, NestedAnova, RangeEstimate};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: format!("sampling.{what}"), message: message.into() }
}

// ---------------------------------------------------------------------------
// Specification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SamplingSpec {
    /// Analyte and sampling target, e.g. "Nitrate in lettuce, one bay".
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Balanced duplicate design: S1A1, S1A2, S2A1, S2A2 for each target.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub duplicates: Vec<[f64; 4]>,
    /// General (also unbalanced) design: for each target the samples, for
    /// each sample its analyses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<Vec<Vec<f64>>>,
    /// Labels of the sampling targets, for the report.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(default, skip_serializing_if = "is_anova")]
    pub method: MethodSpec,
    /// Also evaluate the natural logarithms of the values (uncertainty
    /// factor, Section 9.5.3).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub log: bool,
    /// Modelling approach with Gy's sampling theory instead of duplicates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gy: Option<GySpec>,
    /// Coverage factor for the expanded uncertainties (2 in the Guide).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_factor: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MethodSpec {
    /// Classical nested analysis of variance.
    #[default]
    Anova,
    /// Range calculations (balanced design only, Example A3).
    Range,
}

fn is_anova(m: &MethodSpec) -> bool {
    *m == MethodSpec::Anova
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GySpec {
    /// a_L: mean concentration of the analyte in the lot.
    pub lot_concentration: f64,
    /// α: concentration of the analyte in the critical particles (same unit).
    pub particle_concentration: f64,
    /// ρ_c and ρ_m: densities of critical and matrix particles (g cm⁻³).
    pub critical_density: f64,
    pub matrix_density: f64,
    /// Sampling and sample-splitting steps in order.
    pub steps: Vec<GyStepSpec>,
    /// Relative standard uncertainty of the analytical determination
    /// (fraction), e.g. from quality-control data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytical_rsd: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GyStepSpec {
    pub name: String,
    /// M_S and M_L in g.
    pub sample_mass: f64,
    pub lot_mass: f64,
    /// d in cm (95 % upper limit of the particle sizes).
    pub particle_size: f64,
    /// g: 0.25 for a wide, 1 for a uniform size distribution.
    pub size_factor: f64,
    #[serde(default = "half", skip_serializing_if = "is_half")]
    pub shape_factor: f64,
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub liberation: f64,
}

fn half() -> f64 {
    0.5
}

fn one() -> f64 {
    1.0
}

fn is_half(x: &f64) -> bool {
    *x == 0.5
}

fn is_one(x: &f64) -> bool {
    *x == 1.0
}

// ---------------------------------------------------------------------------
// Expected values
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedSampling {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_between_target: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_sampling: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_analysis: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_measurement: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_total: Option<Check>,
    /// Relative expanded uncertainties U′ = k·100·s/x̄ in per cent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_relative_sampling: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_relative_analysis: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_relative_measurement: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_relative_between_target: Option<Check>,
    /// Per cent of the total variance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent_between_target: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent_sampling: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent_analysis: Option<Check>,
    /// Whether the sampling variance estimate was negative (set to zero).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling_variance_negative: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<ExpectedLog>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gy: Option<ExpectedGy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedLog {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_between_target: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_sampling: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_analysis: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_measurement: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_total: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor_sampling: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor_analysis: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor_measurement: Option<Check>,
    /// u′ = √(exp(s_G²) − 1) for the measurement, as a fraction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_measurement: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedGy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constitution_factor: Option<Check>,
    /// Sampling constants C and relative standard deviations s_r of the
    /// steps, in order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sampling_constants: Vec<Check>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rsd: Vec<Check>,
    /// Combined relative standard uncertainty of sampling, and of the whole
    /// measurement (fractions); relative expanded uncertainties in per cent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsd_sampling: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsd_total: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_relative_sampling: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_relative_total: Option<Check>,
}

// ---------------------------------------------------------------------------
// Evaluation
// ---------------------------------------------------------------------------

/// Estimates of the duplicate method, from either calculation.
#[derive(Clone, Debug, PartialEq)]
pub enum Estimate {
    Anova(NestedAnova),
    Range(RangeEstimate),
}

impl Estimate {
    pub fn mean(&self) -> f64 {
        match self {
            Estimate::Anova(a) => a.mean,
            Estimate::Range(r) => r.mean,
        }
    }

    /// s_T, s_S, s_A.
    pub fn sd(&self) -> [f64; 3] {
        match self {
            Estimate::Anova(a) => a.sd,
            Estimate::Range(r) => r.sd,
        }
    }

    pub fn variance_raw(&self) -> [f64; 3] {
        match self {
            Estimate::Anova(a) => a.variance_raw,
            Estimate::Range(r) => r.variance_raw,
        }
    }

    pub fn measurement_sd(&self) -> f64 {
        let s = self.sd();
        s[1].hypot(s[2])
    }

    pub fn total_sd(&self) -> f64 {
        self.sd().iter().map(|s| s * s).sum::<f64>().sqrt()
    }

    /// Per cent of the total variance: between-target, sampling, analysis.
    pub fn percent(&self) -> [f64; 3] {
        let t = self.total_sd().powi(2);
        self.sd().map(|s| if t > 0.0 { 100.0 * s * s / t } else { 0.0 })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DuplicateAnalysis {
    pub method: MethodSpec,
    /// The design as targets → samples → analyses.
    pub targets: Vec<Vec<Vec<f64>>>,
    pub balanced: bool,
    pub estimate: Estimate,
    /// Classical ANOVA of ln x when requested.
    pub log: Option<NestedAnova>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GyStepResult {
    pub name: String,
    pub step: GyStep,
    pub sampling_constant: f64,
    pub rsd: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GyAnalysis {
    pub material: GyMaterial,
    pub constitution_factor: f64,
    pub steps: Vec<GyStepResult>,
    pub analytical_rsd: Option<f64>,
}

impl GyAnalysis {
    pub fn rsd_sampling(&self) -> f64 {
        self.steps.iter().map(|s| s.rsd * s.rsd).sum::<f64>().sqrt()
    }

    pub fn rsd_total(&self) -> f64 {
        self.rsd_sampling().hypot(self.analytical_rsd.unwrap_or(0.0))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SamplingResult {
    Duplicate(Box<DuplicateAnalysis>),
    Gy(GyAnalysis),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SamplingAnalysis {
    pub quantity: String,
    pub unit: Option<String>,
    pub labels: Vec<String>,
    pub coverage_factor: f64,
    pub result: SamplingResult,
}

impl SamplingAnalysis {
    pub fn duplicate(&self) -> Option<&DuplicateAnalysis> {
        match &self.result {
            SamplingResult::Duplicate(d) => Some(d),
            SamplingResult::Gy(_) => None,
        }
    }

    pub fn gy(&self) -> Option<&GyAnalysis> {
        match &self.result {
            SamplingResult::Gy(g) => Some(g),
            SamplingResult::Duplicate(_) => None,
        }
    }
}

pub fn evaluate(spec: &SamplingSpec) -> Result<SamplingAnalysis, ProjectError> {
    let k = spec.coverage_factor.unwrap_or(2.0);
    if !(k > 0.0 && k.is_finite()) {
        return Err(err("coverage_factor", "must be positive"));
    }
    let designs = [!spec.duplicates.is_empty(), !spec.targets.is_empty(), spec.gy.is_some()];
    if designs.iter().filter(|&&b| b).count() != 1 {
        return Err(err("duplicates", "give exactly one of \"duplicates\", \"targets\" and \"gy\""));
    }
    let result = if let Some(g) = &spec.gy {
        if spec.log || spec.method != MethodSpec::Anova {
            return Err(err("gy", "\"method\" and \"log\" apply to the duplicate method only"));
        }
        SamplingResult::Gy(gy(g)?)
    } else {
        let targets =
            if spec.duplicates.is_empty() { spec.targets.clone() } else { sampling::balanced(&spec.duplicates) };
        if !spec.labels.is_empty() && spec.labels.len() != targets.len() {
            return Err(err("labels", "one label per sampling target"));
        }
        let balanced = targets.iter().all(|t| t.len() == 2 && t.iter().all(|s| s.len() == 2));
        let estimate = match spec.method {
            MethodSpec::Anova => {
                Estimate::Anova(sampling::nested_anova(&targets).map_err(|e| err("targets", e.to_string()))?)
            }
            MethodSpec::Range => {
                if !balanced {
                    return Err(err(
                        "method",
                        "the range method needs the balanced design of two samples with two analyses",
                    ));
                }
                let rows: Vec<[f64; 4]> = targets.iter().map(|t| [t[0][0], t[0][1], t[1][0], t[1][1]]).collect();
                Estimate::Range(sampling::duplicate_range(&rows).map_err(|e| err("duplicates", e.to_string()))?)
            }
        };
        let log = if spec.log {
            let ln = sampling::ln_values(&targets).map_err(|e| err("log", e.to_string()))?;
            Some(sampling::nested_anova(&ln).map_err(|e| err("log", e.to_string()))?)
        } else {
            None
        };
        SamplingResult::Duplicate(Box::new(DuplicateAnalysis { method: spec.method, targets, balanced, estimate, log }))
    };
    Ok(SamplingAnalysis {
        quantity: spec.quantity.clone(),
        unit: spec.unit.clone(),
        labels: spec.labels.clone(),
        coverage_factor: k,
        result,
    })
}

fn gy(g: &GySpec) -> Result<GyAnalysis, ProjectError> {
    let material = GyMaterial {
        lot_concentration: g.lot_concentration,
        particle_concentration: g.particle_concentration,
        critical_density: g.critical_density,
        matrix_density: g.matrix_density,
    };
    let c = sampling::constitution_factor(&material).map_err(|e| err("gy", e.to_string()))?;
    if g.steps.is_empty() {
        return Err(err("gy.steps", "at least one sampling step is needed"));
    }
    if let Some(a) = g.analytical_rsd {
        if !(a >= 0.0 && a.is_finite()) {
            return Err(err("gy.analytical_rsd", "must be a non-negative fraction"));
        }
    }
    let mut steps = Vec::new();
    for (i, s) in g.steps.iter().enumerate() {
        let step = GyStep {
            sample_mass: s.sample_mass,
            lot_mass: s.lot_mass,
            particle_size: s.particle_size,
            size_factor: s.size_factor,
            shape_factor: s.shape_factor,
            liberation: s.liberation,
        };
        let rsd = sampling::fundamental_error(c, &step).map_err(|e| err(&format!("gy.steps[{i}]"), e.to_string()))?;
        steps.push(GyStepResult {
            name: s.name.clone(),
            step,
            sampling_constant: sampling::sampling_constant(c, &step),
            rsd,
        });
    }
    Ok(GyAnalysis { material, constitution_factor: c, steps, analytical_rsd: g.analytical_rsd })
}

// ---------------------------------------------------------------------------
// Checks and JSON
// ---------------------------------------------------------------------------

pub fn check(e: &ExpectedSampling, a: &SamplingAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut push = |what: &str, computed: Option<f64>, c: &Option<Check>| {
        if let Some(c) = c {
            let computed = computed.unwrap_or(f64::NAN);
            out.push(crate::CheckOutcome {
                what: format!("sampling.{what}"),
                computed,
                expected: c.value,
                tolerance: c.tolerance,
                passed: (computed - c.value).abs() <= c.tolerance,
            });
        }
    };
    let k = a.coverage_factor;
    let d = a.duplicate();
    let est = d.map(|d| &d.estimate);
    let rel = |s: f64, m: f64| 100.0 * k * s / m;
    push("mean", est.map(|x| x.mean()), &e.mean);
    push("s_between_target", est.map(|x| x.sd()[0]), &e.s_between_target);
    push("s_sampling", est.map(|x| x.sd()[1]), &e.s_sampling);
    push("s_analysis", est.map(|x| x.sd()[2]), &e.s_analysis);
    push("s_measurement", est.map(|x| x.measurement_sd()), &e.s_measurement);
    push("s_total", est.map(|x| x.total_sd()), &e.s_total);
    push("expanded_relative_sampling", est.map(|x| rel(x.sd()[1], x.mean())), &e.expanded_relative_sampling);
    push("expanded_relative_analysis", est.map(|x| rel(x.sd()[2], x.mean())), &e.expanded_relative_analysis);
    push(
        "expanded_relative_measurement",
        est.map(|x| rel(x.measurement_sd(), x.mean())),
        &e.expanded_relative_measurement,
    );
    push(
        "expanded_relative_between_target",
        est.map(|x| rel(x.sd()[0], x.mean())),
        &e.expanded_relative_between_target,
    );
    push("percent_between_target", est.map(|x| x.percent()[0]), &e.percent_between_target);
    push("percent_sampling", est.map(|x| x.percent()[1]), &e.percent_sampling);
    push("percent_analysis", est.map(|x| x.percent()[2]), &e.percent_analysis);
    if let Some(v) = e.sampling_variance_negative {
        let b = |x: bool| if x { 1.0 } else { 0.0 };
        push(
            "sampling_variance_negative",
            est.map(|x| b(x.variance_raw()[1] < 0.0)),
            &Some(Check { value: b(v), tolerance: 0.0 }),
        );
    }
    if let Some(x) = &e.log {
        let g = d.and_then(|d| d.log.as_ref());
        push("log.mean", g.map(|g| g.mean), &x.mean);
        push("log.s_between_target", g.map(|g| g.sd[0]), &x.s_between_target);
        push("log.s_sampling", g.map(|g| g.sd[1]), &x.s_sampling);
        push("log.s_analysis", g.map(|g| g.sd[2]), &x.s_analysis);
        push("log.s_measurement", g.map(|g| g.measurement_sd()), &x.s_measurement);
        push("log.s_total", g.map(|g| g.total_sd()), &x.s_total);
        push("log.factor_sampling", g.map(|g| sampling::uncertainty_factor(g.sd[1], k)), &x.factor_sampling);
        push("log.factor_analysis", g.map(|g| sampling::uncertainty_factor(g.sd[2], k)), &x.factor_analysis);
        push(
            "log.factor_measurement",
            g.map(|g| sampling::uncertainty_factor(g.measurement_sd(), k)),
            &x.factor_measurement,
        );
        push(
            "log.relative_measurement",
            g.map(|g| sampling::relative_from_log(g.measurement_sd())),
            &x.relative_measurement,
        );
    }
    if let Some(x) = &e.gy {
        let g = a.gy();
        push("gy.constitution_factor", g.map(|g| g.constitution_factor), &x.constitution_factor);
        for (i, c) in x.sampling_constants.iter().enumerate() {
            let v = g.and_then(|g| g.steps.get(i)).map(|s| s.sampling_constant);
            push(&format!("gy.sampling_constants[{i}]"), v, &Some(*c));
        }
        for (i, c) in x.rsd.iter().enumerate() {
            let v = g.and_then(|g| g.steps.get(i)).map(|s| s.rsd);
            push(&format!("gy.rsd[{i}]"), v, &Some(*c));
        }
        push("gy.rsd_sampling", g.map(|g| g.rsd_sampling()), &x.rsd_sampling);
        push("gy.rsd_total", g.map(|g| g.rsd_total()), &x.rsd_total);
        push("gy.expanded_relative_sampling", g.map(|g| 100.0 * k * g.rsd_sampling()), &x.expanded_relative_sampling);
        push("gy.expanded_relative_total", g.map(|g| 100.0 * k * g.rsd_total()), &x.expanded_relative_total);
    }
}

pub fn method_code(m: MethodSpec) -> &'static str {
    match m {
        MethodSpec::Anova => "anova",
        MethodSpec::Range => "range",
    }
}

fn anova_json(a: &NestedAnova) -> serde_json::Value {
    use serde_json::json;
    json!({
        "mean": a.mean,
        "targets": a.targets, "samples": a.samples, "observations": a.observations,
        "ss": a.ss, "dof": a.dof, "ms": a.ms,
        "ems_coefficients": [a.k1, a.k2, a.k3],
        "variance_estimates": a.variance_raw,
    })
}

pub fn results_json(a: &SamplingAnalysis) -> serde_json::Value {
    use serde_json::json;
    let k = a.coverage_factor;
    match &a.result {
        SamplingResult::Duplicate(d) => {
            let e = &d.estimate;
            let (m, sd) = (e.mean(), e.sd());
            let rel = |s: f64| 100.0 * k * s / m;
            let table = match e {
                Estimate::Anova(x) => anova_json(x),
                Estimate::Range(r) => json!({
                    "mean": r.mean, "d2": r.d2,
                    "mean_range_analysis": r.mean_range_analysis,
                    "mean_range_samples": r.mean_range_samples,
                    "sd_target_means": r.sd_target_means,
                    "s_samples": r.s_samples,
                    "variance_estimates": r.variance_raw,
                }),
            };
            json!({
                "design": "duplicate",
                "method": method_code(d.method),
                "balanced": d.balanced,
                "coverage_factor": k,
                "labels": a.labels,
                "targets": d.targets,
                "mean": m,
                "s_between_target": sd[0], "s_sampling": sd[1], "s_analysis": sd[2],
                "s_measurement": e.measurement_sd(), "s_total": e.total_sd(),
                "percent_of_total_variance": e.percent(),
                "expanded_relative": {
                    "between_target": rel(sd[0]), "sampling": rel(sd[1]), "analysis": rel(sd[2]),
                    "measurement": rel(e.measurement_sd()),
                },
                "calculation": table,
                "log": d.log.as_ref().map(|g| json!({
                    "mean": g.mean, "geometric_mean": g.mean.exp(),
                    "s_between_target": g.sd[0], "s_sampling": g.sd[1], "s_analysis": g.sd[2],
                    "s_measurement": g.measurement_sd(), "s_total": g.total_sd(),
                    "factor": {
                        "sampling": sampling::uncertainty_factor(g.sd[1], k),
                        "analysis": sampling::uncertainty_factor(g.sd[2], k),
                        "measurement": sampling::uncertainty_factor(g.measurement_sd(), k),
                    },
                    "relative_measurement": sampling::relative_from_log(g.measurement_sd()),
                    "calculation": anova_json(g),
                })),
            })
        }
        SamplingResult::Gy(g) => json!({
            "design": "gy",
            "coverage_factor": k,
            "constitution_factor": g.constitution_factor,
            "steps": g.steps.iter().map(|s| json!({
                "name": s.name, "sample_mass": s.step.sample_mass, "lot_mass": s.step.lot_mass,
                "particle_size": s.step.particle_size, "size_factor": s.step.size_factor,
                "shape_factor": s.step.shape_factor, "liberation": s.step.liberation,
                "sampling_constant": s.sampling_constant, "rsd": s.rsd,
            })).collect::<Vec<_>>(),
            "analytical_rsd": g.analytical_rsd,
            "rsd_sampling": g.rsd_sampling(),
            "rsd_total": g.rsd_total(),
            "expanded_relative_sampling": 100.0 * k * g.rsd_sampling(),
            "expanded_relative_total": 100.0 * k * g.rsd_total(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(json: &str) -> SamplingSpec {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn designs_are_exclusive_and_range_needs_balance() {
        let s = spec(r#"{"quantity":"x","duplicates":[[1,2,3,4],[2,3,5,4]],"targets":[[[1,2],[3]]]}"#);
        assert!(evaluate(&s).is_err());
        let s = spec(r#"{"quantity":"x","targets":[[[1,2],[3]],[[2,3],[5,4]]],"method":"range"}"#);
        assert!(evaluate(&s).is_err());
        let s = spec(r#"{"quantity":"x","targets":[[[1,2],[3]],[[2,3],[5,4]],[[4,4.5],[4.2]]]}"#);
        let a = evaluate(&s).unwrap();
        assert!(!a.duplicate().unwrap().balanced);
        let s = spec(r#"{"quantity":"x","duplicates":[[1,2,3,4],[2,3,5,4]],"labels":["a"]}"#);
        assert!(evaluate(&s).is_err());
        let s = spec(r#"{"quantity":"x","duplicates":[[1,2,3,4],[2,3,5,-4]],"log":true}"#);
        assert!(evaluate(&s).is_err());
    }

    #[test]
    fn range_and_anova_agree_roughly() {
        let rows = r#"[[0.815,0.834,0.912,0.893],[1.8,1.83,1.94,1.93],[1.69,1.68,1.79,1.77],
            [2.62,2.61,2.83,2.84],[1.66,1.63,1.58,1.59],[1.52,1.53,1.47,1.50]]"#;
        let a = evaluate(&spec(&format!(r#"{{"quantity":"Fe","duplicates":{rows}}}"#))).unwrap();
        let r = evaluate(&spec(&format!(r#"{{"quantity":"Fe","duplicates":{rows},"method":"range"}}"#))).unwrap();
        let (a, r) = (&a.duplicate().unwrap().estimate, &r.duplicate().unwrap().estimate);
        assert!((a.mean() - r.mean()).abs() < 1e-12);
        assert!((a.sd()[0] / r.sd()[0] - 1.0).abs() < 0.01);
        let j =
            results_json(&evaluate(&spec(&format!(r#"{{"quantity":"Fe","duplicates":{rows},"log":true}}"#))).unwrap());
        assert_eq!(j["method"], "anova");
        assert!(j["log"]["factor"]["measurement"].as_f64().unwrap() > 1.0);
    }
}
