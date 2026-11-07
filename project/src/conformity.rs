//! Conformity assessment [JCGM 106:2012]: the probability that a measured
//! property conforms to a tolerance interval, decision rules with guard bands,
//! and the global consumer's and producer's risks of an inspected production
//! process. The section `conformity` is either attached to a model project
//! (the result of the model is assessed) or forms a project of its own
//! (measured values, a measuring system and optionally a process).

use crate::schema::Check;
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uncertainty_core::conformity::{self as cf, Acceptance, GlobalRisks, Knowledge, Process, Rule, Tolerance};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: format!("conformity.{what}"), message: message.into() }
}

// ---------------------------------------------------------------------------
// Specification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConformitySpec {
    /// Name and unit of the property (project of its own only; a model
    /// project uses its output).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Tolerance limits T_L, T_U (at least one).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper: Option<f64>,
    #[serde(default)]
    pub rule: RuleSpec,
    /// Guard band w = r·U with U = 2u (JCGM 106 8.3.2.3); default r = 1 for
    /// the guarded rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Guard band length |w| in the unit of the property.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guard_band: Option<f64>,
    /// Acceptance limits set so that the probability of conformity (guarded
    /// acceptance) or non-conformity (guarded rejection) at the limit is this
    /// value (JCGM 106 8.3.3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,
    /// Acceptance limits given directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acceptance_lower: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acceptance_upper: Option<f64>,
    /// Standard uncertainty of the measuring system, absolute or relative to
    /// the measured value (project of its own only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_relative: Option<f64>,
    /// Degrees of freedom: the PDF for the measurand is then a scaled and
    /// shifted t distribution with scale u (JCGM 101 6.4.9).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dof: Option<f64>,
    /// Measured values to be assessed (project of its own only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<MeasuredSpec>,
    /// PDF of the property over the production process, for global risks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessSpec>,
    /// Conformance probabilities of the measured results from the posterior
    /// PDF, with the process PDF as prior (JCGM 106 6.2, Annex A); needs a
    /// process and a project without a model.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub posterior: bool,
    /// Required global consumer's risk: the guard-band multiplier r is found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_consumer_risk: Option<f64>,
    /// Guard band from the test uncertainty ratio: `rss`, `dobbert`, `rp10`
    /// or `test95` (two-sided tolerance, fixed u; TUR = (T_U − T_L)/2 / 2u).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tur_method: Option<TurMethodSpec>,
    /// PDF for the measurand: `normal` (default), `t` with the effective
    /// degrees of freedom (model projects; give `dof` otherwise) or
    /// `lognormal` with median y and s_G = u/|y| (Eurachem/CITAC 2021,
    /// Annex A, Case 4, Equation 2).
    #[serde(default)]
    pub distribution: KnowledgeSpec,
    /// Lognormal: s_G = √ln(1 + (u/y)²) instead of the approximation u/|y|.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub exact_sdlog: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleSpec {
    #[default]
    SimpleAcceptance,
    GuardedAcceptance,
    GuardedRejection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TurMethodSpec {
    Rss,
    Dobbert,
    Rp10,
    Test95,
}

impl TurMethodSpec {
    pub fn method(self) -> cf::TurMethod {
        match self {
            TurMethodSpec::Rss => cf::TurMethod::Rss,
            TurMethodSpec::Dobbert => cf::TurMethod::Dobbert,
            TurMethodSpec::Rp10 => cf::TurMethod::Rp10,
            TurMethodSpec::Test95 => cf::TurMethod::Test95,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeSpec {
    #[default]
    Normal,
    T,
    Lognormal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasuredSpec {
    pub name: String,
    pub value: f64,
    /// Standard uncertainty of this value (default: that of the measuring system).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessSpec {
    pub distribution: ProcessKind,
    /// Mean and standard deviation (normal, gamma).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd: Option<f64>,
    /// Mean and standard deviation of ln Y (lognormal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meanlog: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sdlog: Option<f64>,
}

impl ProcessSpec {
    fn process(&self) -> Result<Process, ProjectError> {
        let (a, b, names) = match self.distribution {
            ProcessKind::Lognormal => (self.meanlog, self.sdlog, ["meanlog", "sdlog"]),
            _ => (self.mean, self.sd, ["mean", "sd"]),
        };
        let other = match self.distribution {
            ProcessKind::Lognormal => self.mean.is_some() || self.sd.is_some(),
            _ => self.meanlog.is_some() || self.sdlog.is_some(),
        };
        match (a, b) {
            (Some(a), Some(b)) if !other => Ok(match self.distribution {
                ProcessKind::Normal => Process::Normal { mean: a, sd: b },
                ProcessKind::Gamma => Process::Gamma { mean: a, sd: b },
                ProcessKind::Lognormal => Process::Lognormal { meanlog: a, sdlog: b },
            }),
            _ => Err(err("process", format!("this distribution takes {} and {} only", names[0], names[1]))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessKind {
    Normal,
    Gamma,
    Lognormal,
}

// ---------------------------------------------------------------------------
// Expected values
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedConformity {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub results: BTreeMap<String, ExpectedResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acceptance_lower: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acceptance_upper: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_index: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tur: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guard_band_factor: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conforming: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumer_risk: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_risk: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_probability: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
}

// ---------------------------------------------------------------------------
// Results
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct AssessedResult {
    pub name: String,
    pub value: f64,
    pub u: f64,
    pub dof: Option<f64>,
    /// p_c from the normal or t PDF, or from the posterior with the process
    /// PDF as prior.
    pub conformance_probability: f64,
    /// p_c from the Monte Carlo values (model projects).
    pub conformance_probability_mcm: Option<f64>,
    pub accepted: bool,
    /// Specific consumer's risk 1 − p_c if accepted, specific producer's
    /// risk p_c if rejected (JCGM 106 9.3.2).
    pub specific_risk: f64,
}

#[derive(Clone, Debug)]
pub struct ConformityAnalysis {
    pub quantity: String,
    pub unit: Option<String>,
    pub tolerance: Tolerance,
    pub rule: RuleSpec,
    /// Guard band w (positive: guarded acceptance), when set from r or |w|.
    pub guard_band: Option<f64>,
    pub r: Option<f64>,
    pub probability: Option<f64>,
    pub acceptance: Acceptance,
    /// C_m = T/(4u) for a two-sided tolerance and a fixed u.
    pub capability_index: Option<f64>,
    /// TUR and guard-band factor when the acceptance limits come from a TUR method.
    pub tur: Option<(TurMethodSpec, f64, f64)>,
    pub results: Vec<AssessedResult>,
    pub process: Option<ProcessSpec>,
    /// The conformance probabilities are posterior ones.
    pub posterior: bool,
    /// Lognormal PDF for the measurand, with the exact s_G if true.
    pub lognormal: Option<bool>,
    pub global: Option<GlobalRisks>,
    /// Standard uncertainty (or scale) and degrees of freedom of the measuring
    /// system at the acceptance limits, for charts of design studies.
    pub limit_knowledge: Vec<(f64, f64, Option<f64>)>,
}

fn knowledge(y: f64, u: f64, dof: Option<f64>, lognormal: Option<bool>) -> Knowledge {
    if let Some(exact) = lognormal {
        let rel = u / y.abs();
        let sdlog = if exact { (rel * rel).ln_1p().sqrt() } else { rel };
        return Knowledge::Lognormal { y, sdlog };
    }
    match dof {
        Some(nu) if nu.is_finite() => Knowledge::StudentT { y, u, nu },
        _ => Knowledge::Normal { y, u },
    }
}

fn rule(r: RuleSpec) -> Rule {
    match r {
        RuleSpec::SimpleAcceptance => Rule::SimpleAcceptance,
        RuleSpec::GuardedAcceptance => Rule::GuardedAcceptance,
        RuleSpec::GuardedRejection => Rule::GuardedRejection,
    }
}

/// A result to be assessed: name, value, standard uncertainty, degrees of
/// freedom and Monte Carlo values (model projects).
pub struct Subject<'a> {
    pub name: String,
    pub unit: Option<String>,
    pub value: f64,
    pub u: f64,
    pub dof: Option<f64>,
    pub values: Option<&'a [f64]>,
}

/// Evaluates the conformity section; `subject` is the model result for a
/// model project and None for a project of its own.
pub fn evaluate(spec: &ConformitySpec, subject: Option<Subject>) -> Result<ConformityAnalysis, ProjectError> {
    let tolerance = Tolerance::new(spec.lower, spec.upper).map_err(|e| err("lower", e.to_string()))?;
    let model = subject.is_some();
    if model {
        let own = [
            spec.quantity.is_some(),
            spec.unit.is_some(),
            spec.u.is_some(),
            spec.u_relative.is_some(),
            spec.dof.is_some(),
            !spec.results.is_empty(),
        ];
        if own.iter().any(|&b| b) {
            return Err(err(
                "results",
                "quantity, unit, u, u_relative, dof and results belong to a conformity project without a model",
            ));
        }
    } else if spec.distribution == KnowledgeSpec::T {
        return Err(err("distribution", "t applies to model projects; give dof for a t distribution"));
    }
    let lognormal = (spec.distribution == KnowledgeSpec::Lognormal).then_some(spec.exact_sdlog);
    if lognormal.is_some() && (spec.dof.is_some() || spec.posterior) {
        return Err(err("distribution", "a lognormal PDF takes neither dof nor posterior"));
    }
    if spec.exact_sdlog && lognormal.is_none() {
        return Err(err("exact_sdlog", "applies to a lognormal PDF"));
    }
    if spec.u.is_some() && spec.u_relative.is_some() {
        return Err(err("u", "give either u or u_relative"));
    }
    for (name, v) in [("u", spec.u), ("u_relative", spec.u_relative), ("dof", spec.dof)] {
        if v.is_some_and(|x| !(x > 0.0 && x.is_finite())) {
            return Err(err(name, "must be positive"));
        }
    }
    // Uncertainty of the measuring system at a value y.
    let system_u = |y: f64| -> Option<f64> {
        match (&subject, spec.u, spec.u_relative) {
            (Some(s), _, _) => Some(s.u),
            (None, Some(u), _) => Some(u),
            (None, None, Some(r)) => Some(r * y.abs()),
            _ => None,
        }
    };
    let dof = match &subject {
        Some(s) if spec.distribution == KnowledgeSpec::T => s.dof,
        Some(_) => None,
        None => spec.dof,
    };
    let need_u = |what: &str| err(what, "needs the standard uncertainty of the measuring system (u or u_relative)");

    // Acceptance interval.
    let explicit = spec.acceptance_lower.is_some() || spec.acceptance_upper.is_some();
    let ways = [
        spec.r.is_some(),
        spec.guard_band.is_some(),
        spec.probability.is_some(),
        explicit,
        spec.target_consumer_risk.is_some(),
        spec.tur_method.is_some(),
    ];
    if ways.iter().filter(|&&b| b).count() > 1 {
        return Err(err(
            "r",
            "give only one of r, guard_band, probability, acceptance limits, target_consumer_risk and tur_method",
        ));
    }
    if spec.rule == RuleSpec::SimpleAcceptance
        && (spec.r.is_some() || spec.guard_band.is_some() || spec.probability.is_some())
    {
        return Err(err("rule", "r, guard_band and probability need a guarded rule"));
    }
    if spec.guard_band.is_some_and(|w| !(w >= 0.0 && w.is_finite())) {
        return Err(err("guard_band", "give the length |w| ≥ 0; the rule sets its sign"));
    }
    let sign = if spec.rule == RuleSpec::GuardedRejection { -1.0 } else { 1.0 };
    // u for a guard band w = r·2u: the fixed u, or u at the tolerance limit.
    let band_u = |what: &str| -> Result<f64, ProjectError> {
        let at = tolerance.upper.or(tolerance.lower).unwrap_or(0.0);
        system_u(at).ok_or_else(|| need_u(what))
    };
    let process = spec.process.as_ref().map(ProcessSpec::process).transpose()?;
    if spec.posterior && (process.is_none() || model) {
        return Err(err("posterior", "needs a process PDF in a conformity project without a model"));
    }
    let mut r_used = None;
    let mut rule_used = spec.rule;
    let mut tur_used = None;
    let (acceptance, guard_band) = if explicit {
        let a = Acceptance { lower: spec.acceptance_lower, upper: spec.acceptance_upper };
        if a.lower.is_some() != tolerance.lower.is_some() || a.upper.is_some() != tolerance.upper.is_some() {
            return Err(err("acceptance_lower", "give an acceptance limit for each tolerance limit"));
        }
        // The rule and the guard band follow from the limits (w > 0 inside).
        let ws: Vec<f64> =
            [a.lower.zip(tolerance.lower).map(|(a, t)| a - t), a.upper.zip(tolerance.upper).map(|(a, t)| t - a)]
                .into_iter()
                .flatten()
                .collect();
        let inferred = if ws.iter().all(|w| *w == 0.0) {
            RuleSpec::SimpleAcceptance
        } else if ws.iter().all(|w| *w >= 0.0) {
            RuleSpec::GuardedAcceptance
        } else if ws.iter().all(|w| *w <= 0.0) {
            RuleSpec::GuardedRejection
        } else {
            return Err(err(
                "acceptance_lower",
                "the acceptance limits lie on different sides of the tolerance limits",
            ));
        };
        if spec.rule != RuleSpec::SimpleAcceptance && spec.rule != inferred {
            return Err(err("rule", "the rule does not match the acceptance limits"));
        }
        rule_used = inferred;
        let w = (ws.windows(2).all(|p| (p[0] - p[1]).abs() <= 1e-12 * p[0].abs().max(1.0))
            && inferred != RuleSpec::SimpleAcceptance)
            .then(|| ws[0]);
        (a, w)
    } else if let Some(p) = spec.probability {
        let a = cf::acceptance_from_probability(&tolerance, rule(spec.rule), p, |y| {
            knowledge(y, system_u(y).unwrap_or(f64::NAN), dof, lognormal)
        })
        .map_err(|e| err("probability", e.to_string()))?;
        if system_u(tolerance.upper.or(tolerance.lower).unwrap_or(0.0)).is_none() {
            return Err(need_u("probability"));
        }
        (a, None)
    } else if let Some(m) = spec.tur_method {
        let (Some(tl), Some(tu)) = (tolerance.lower, tolerance.upper) else {
            return Err(err("tur_method", "needs a two-sided tolerance"));
        };
        if spec.u_relative.is_some() {
            return Err(err("tur_method", "needs a fixed u of the measuring system"));
        }
        let half = 0.5 * (tu - tl);
        let tur = half / (2.0 * band_u("tur_method")?);
        let gbf = cf::tur_guard_band_factor(m.method(), tur).map_err(|e| err("tur_method", e.to_string()))?;
        let w = (1.0 - gbf) * half;
        rule_used = if w > 0.0 {
            RuleSpec::GuardedAcceptance
        } else if w < 0.0 {
            RuleSpec::GuardedRejection
        } else {
            RuleSpec::SimpleAcceptance
        };
        tur_used = Some((m, tur, gbf));
        (cf::acceptance_from_guard_band(&tolerance, w), (w != 0.0).then_some(w))
    } else if let Some(target) = spec.target_consumer_risk {
        let proc_ = process.as_ref().ok_or_else(|| err("target_consumer_risk", "needs a process PDF"))?;
        if spec.u_relative.is_some() {
            return Err(err("target_consumer_risk", "needs a fixed u of the measuring system"));
        }
        let um = band_u("target_consumer_risk")?;
        let r = cf::guard_band_for_consumer_risk(proc_, um, &tolerance, target)
            .map_err(|e| err("target_consumer_risk", e.to_string()))?;
        r_used = Some(r);
        rule_used = if r >= 0.0 { RuleSpec::GuardedAcceptance } else { RuleSpec::GuardedRejection };
        let w = 2.0 * r * um;
        (cf::acceptance_from_guard_band(&tolerance, w), Some(w))
    } else {
        let w = match (spec.rule, spec.r, spec.guard_band) {
            (RuleSpec::SimpleAcceptance, _, _) => 0.0,
            (_, _, Some(g)) => sign * g,
            (_, r, None) => {
                let r = r.unwrap_or(1.0);
                if !(r >= 0.0 && r.is_finite()) {
                    return Err(err("r", "give r ≥ 0; the rule sets the sign of the guard band"));
                }
                r_used = Some(r);
                sign * r * 2.0 * band_u("r")?
            }
        };
        (cf::acceptance_from_guard_band(&tolerance, w), (spec.rule != RuleSpec::SimpleAcceptance).then_some(w))
    };
    if let (Some(l), Some(u)) = (acceptance.lower, acceptance.upper) {
        if l > u {
            return Err(err("r", "the guard bands leave no acceptance interval"));
        }
    }

    // Results.
    let mut results = Vec::new();
    let mut assess = |name: String, value: f64, u: f64, values: Option<&[f64]>| -> Result<(), ProjectError> {
        let pc = match (&process, spec.posterior) {
            (Some(p), true) => {
                if dof.is_some() {
                    return Err(err("posterior", "uses a normal likelihood; remove dof"));
                }
                cf::posterior_conformance_probability(p, value, u, &tolerance)
            }
            _ => cf::conformance_probability(&knowledge(value, u, dof, lognormal), &tolerance),
        }
        .map_err(|e| err("results", e.to_string()))?;
        let accepted = acceptance.contains(value);
        results.push(AssessedResult {
            name,
            value,
            u,
            dof,
            conformance_probability: pc,
            conformance_probability_mcm: values.and_then(|v| cf::conformance_probability_from_values(v, &tolerance)),
            accepted,
            specific_risk: if accepted { 1.0 - pc } else { pc },
        });
        Ok(())
    };
    let (quantity, unit) = match &subject {
        Some(s) => {
            assess(s.name.clone(), s.value, s.u, s.values)?;
            (s.name.clone(), s.unit.clone())
        }
        None => {
            for m in &spec.results {
                let u = m.u.or_else(|| system_u(m.value)).ok_or_else(|| need_u("results"))?;
                assess(m.name.clone(), m.value, u, None)?;
            }
            (spec.quantity.clone().unwrap_or_else(|| "Y".into()), spec.unit.clone())
        }
    };
    // C_m needs one u: the model result's, or the measuring system's fixed u.
    let fixed_u = if model { results.first().map(|r| r.u) } else { spec.u };
    let global = match &process {
        None => None,
        Some(p) => {
            let g = match (subject.as_ref(), spec.u_relative) {
                (None, Some(ur)) => cf::global_risks_relative(p, ur, &tolerance, &acceptance),
                _ => cf::global_risks(p, band_u("process")?, &tolerance, &acceptance),
            };
            Some(g.map_err(|e| err("process", e.to_string()))?)
        }
    };
    if spec.results.is_empty()
        && !model
        && global.is_none()
        && spec.probability.is_none()
        && !explicit
        && guard_band.is_none()
    {
        return Err(err("results", "give measured results, a process or a rule that sets acceptance limits"));
    }
    let limit_knowledge = [acceptance.lower, acceptance.upper]
        .into_iter()
        .flatten()
        .filter_map(|a| if model { results.first().map(|r| (a, r.u, dof)) } else { system_u(a).map(|u| (a, u, dof)) })
        .collect();
    Ok(ConformityAnalysis {
        limit_knowledge,
        quantity,
        unit,
        tolerance,
        rule: rule_used,
        guard_band,
        r: r_used,
        probability: spec.probability,
        acceptance,
        capability_index: fixed_u.and_then(|u| cf::capability_index(&tolerance, u)),
        tur: tur_used,
        results,
        process: spec.process.clone(),
        posterior: spec.posterior,
        lognormal,
        global,
    })
}

/// Compares the analysis with the expected values.
pub fn check(e: &ExpectedConformity, a: &ConformityAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut push = |what: String, computed: Option<f64>, c: &Option<Check>| {
        if let Some(c) = c {
            let computed = computed.unwrap_or(f64::NAN);
            out.push(crate::CheckOutcome {
                what,
                computed,
                expected: c.value,
                tolerance: c.tolerance,
                passed: (computed - c.value).abs() <= c.tolerance,
            });
        }
    };
    push("conformity.acceptance_lower".into(), a.acceptance.lower, &e.acceptance_lower);
    push("conformity.acceptance_upper".into(), a.acceptance.upper, &e.acceptance_upper);
    push("conformity.capability_index".into(), a.capability_index, &e.capability_index);
    push("conformity.tur".into(), a.tur.map(|t| t.1), &e.tur);
    push("conformity.guard_band_factor".into(), a.tur.map(|t| t.2), &e.guard_band_factor);
    push("conformity.r".into(), a.r, &e.r);
    let g = a.global.as_ref();
    push("conformity.conforming".into(), g.map(|g| g.conforming), &e.conforming);
    push("conformity.accepted".into(), g.map(|g| g.accepted), &e.accepted);
    push("conformity.consumer_risk".into(), g.map(|g| g.consumer), &e.consumer_risk);
    push("conformity.producer_risk".into(), g.map(|g| g.producer), &e.producer_risk);
    let mut verdicts = Vec::new();
    for (name, er) in &e.results {
        let r = a.results.iter().find(|r| &r.name == name);
        push(
            format!("conformity.results[{name}].conformance_probability"),
            r.map(|r| r.conformance_probability),
            &er.conformance_probability,
        );
        if let Some(want) = er.accepted {
            verdicts.push((format!("conformity.results[{name}].accepted"), r.map(|r| r.accepted), want));
        }
    }
    for (what, got, want) in verdicts {
        let computed = got.map_or(f64::NAN, |b| if b { 1.0 } else { 0.0 });
        let expected = if want { 1.0 } else { 0.0 };
        out.push(crate::CheckOutcome { what, computed, expected, tolerance: 0.0, passed: computed == expected });
    }
}

pub fn rule_code(r: RuleSpec) -> &'static str {
    match r {
        RuleSpec::SimpleAcceptance => "simple_acceptance",
        RuleSpec::GuardedAcceptance => "guarded_acceptance",
        RuleSpec::GuardedRejection => "guarded_rejection",
    }
}

pub fn results_json(a: &ConformityAnalysis) -> serde_json::Value {
    use serde_json::json;
    json!({
        "quantity": a.quantity,
        "unit": a.unit,
        "tolerance": { "lower": a.tolerance.lower, "upper": a.tolerance.upper },
        "rule": rule_code(a.rule),
        "guard_band": a.guard_band,
        "r": a.r,
        "probability": a.probability,
        "acceptance": { "lower": a.acceptance.lower, "upper": a.acceptance.upper },
        "capability_index": a.capability_index,
        "tur": a.tur.map(|(m, t, g)| json!({"method": m, "tur": t, "guard_band_factor": g})),
        "posterior": a.posterior,
        "results": a.results.iter().map(|r| json!({
            "name": r.name, "value": r.value, "u": r.u, "dof": r.dof,
            "conformance_probability": r.conformance_probability,
            "conformance_probability_mcm": r.conformance_probability_mcm,
            "accepted": r.accepted, "specific_risk": r.specific_risk,
        })).collect::<Vec<_>>(),
        "process": a.process.as_ref().map(|p| json!({
            "distribution": match p.distribution {
                ProcessKind::Normal => "normal",
                ProcessKind::Gamma => "gamma",
                ProcessKind::Lognormal => "lognormal",
            },
            "mean": p.mean, "sd": p.sd, "meanlog": p.meanlog, "sdlog": p.sdlog,
        })),
        "global": a.global.map(|g| json!({
            "conforming": g.conforming, "accepted": g.accepted,
            "consumer_risk": g.consumer, "producer_risk": g.producer,
        })),
    })
}
