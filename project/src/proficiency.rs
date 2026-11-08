//! Proficiency-testing projects [ISO 13528:2022]: the results of one round
//! (one measurand, one proficiency test item), the assigned value and its
//! standard uncertainty, the standard deviation for proficiency assessment,
//! the performance statistics of every participant and, optionally, the
//! homogeneity and stability checks of the proficiency test items.

use crate::schema::Check;
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uncertainty_core::pt::{self, AlgorithmA, Criteria, Homogeneity, Scores, Signal, Stability, Stopping};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: what.to_string(), message: message.into() }
}

fn default_two() -> f64 {
    2.0
}

fn default_factor() -> f64 {
    1.25
}

// ---------------------------------------------------------------------------
// Specification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProficiencySpec {
    /// Name of the measurand, e.g. "atrazine".
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Participant results (one per participant: the result or the average
    /// of its replicates, 9.3.1). May be empty for a homogeneity and
    /// stability study before the round.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<ParticipantSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_value: Option<AssignedSpec>,
    /// Standard deviation for proficiency assessment σ_pt (Clause 8).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_pt: Option<SdPtSpec>,
    /// Allowance for measurement error δ_E for PA scores (9.3); default 3σ_pt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta_e: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta_e_percent: Option<f64>,
    /// Coverage factor of U(x_pt) for E_n scores (default 2, 9.5.2).
    #[serde(default = "default_two")]
    pub coverage_factor: f64,
    /// Screening limits for reported standard uncertainties (9.8).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncertainty_screening: Option<ScreeningSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homogeneity: Option<HomogeneitySpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stability: Option<StabilitySpec>,
    /// Stopping rule of Algorithm A (default: third significant figure, C.3.1).
    #[serde(default)]
    pub algorithm_a: StopSpec,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopSpec {
    #[default]
    ThirdSignificantFigure,
    Converged,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParticipantSpec {
    /// Laboratory code.
    pub lab: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Censored result "< limit" (5.5.3): excluded from statistics and scores.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub less_than: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<f64>,
    /// Reported standard uncertainty, or expanded uncertainty with k.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k: Option<f64>,
    /// Measurement method (for reports).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

/// How the assigned value x_pt and its standard uncertainty are obtained
/// (Clause 7).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssignedSpec {
    pub method: AssignedMethodSpec,
    /// Reference value and its standard uncertainty, or expanded
    /// uncertainty with k (method "reference" only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k: Option<f64>,
    /// Factor of Formula (6), u = factor · s/√p (default 1.25).
    #[serde(default = "default_factor")]
    pub u_factor: f64,
    /// Scale estimator for the uncertainty of the median (default nIQR).
    #[serde(default)]
    pub scale: ScaleEstimator,
    /// Components added in quadrature, Formula (3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_hom: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_trans: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_stab: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssignedMethodSpec {
    /// Robust average x* of Algorithm A, u = 1.25 s*/√p (7.7.7).
    AlgorithmA,
    /// Median, u = 1.25 nIQR/√p (or MADe).
    Median,
    /// Arithmetic mean, u = s/√p.
    Mean,
    /// Formulation, CRM, reference or expert laboratories (7.3–7.6).
    Reference,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaleEstimator {
    #[default]
    Niqr,
    Made,
}

/// Standard deviation for proficiency assessment σ_pt (Clause 8).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SdPtSpec {
    pub method: SdPtMethodSpec,
    /// σ_pt for method "given", or per cent of the assigned value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SdPtMethodSpec {
    /// Robust standard deviation s* of Algorithm A (8.6).
    Robust,
    Made,
    Niqr,
    /// Arithmetic standard deviation of the results.
    Sd,
    /// Fitness for purpose, experience or a general model (8.2–8.4).
    Given,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Limit {
    Value(f64),
    /// "u_assigned" (u_min = u(x_pt), 9.8.3) or "1.5_robust_sd"
    /// (u_max = 1.5 s*, 9.8.4).
    Rule(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScreeningSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_min: Option<Limit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_max: Option<Limit>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HomogeneitySpec {
    /// Results of g items, each measured m times (B.3).
    pub items: Vec<Vec<f64>>,
    /// σ_pt for the check (default: the σ_pt of the round), or per cent of
    /// the general average of the homogeneity data (a provisional σ_pt,
    /// ISO 13528 E.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_pt: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_pt_percent_of_mean: Option<f64>,
    /// Bayesian estimate of the between-unit standard deviation (JCGM GUM-5
    /// Example 12): scales of the half-Cauchy priors of τ and σ and an
    /// optional normal prior for the mean.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bayes: Option<BayesHomogeneitySpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BayesHomogeneitySpec {
    pub tau_scale: f64,
    pub sigma_scale: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mu_mean: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mu_sd: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StabilitySpec {
    /// Results of the stability check.
    pub values: Vec<f64>,
    /// General average before distribution (default: the homogeneity mean).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<f64>,
    /// Standard uncertainties of the two averages for Formula (B.18).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_reference: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_values: Option<f64>,
}

// ---------------------------------------------------------------------------
// Expected values
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedProficiency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_value: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_assigned: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_pt: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub median: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub made: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub niqr: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub robust_mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub robust_sd: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iterations: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd: Option<Check>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scores: BTreeMap<String, ExpectedScores>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homogeneity: Option<ExpectedHomogeneity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stability: Option<ExpectedStability>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedScores {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d_percent: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pa: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_prime: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zeta: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedHomogeneity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_x: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_w: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_s: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criterion: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sufficient: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ms_between: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ms_within: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_bb_star: Option<Check>,
    /// Posterior of τ and σ (Bayesian model).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tau_mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tau_sd: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tau_median: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tau_mode: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tau_q975: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tau_hpd_upper: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigma_mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigma_hpd_lower: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigma_hpd_upper: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mu_mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mu_sd: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedStability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub difference: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sufficient: Option<bool>,
}

// ---------------------------------------------------------------------------
// Results
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssignedMethod {
    AlgorithmA,
    Median,
    Mean,
    Reference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdPtMethod {
    Robust,
    Made,
    Niqr,
    Sd,
    Given,
}

/// Summary statistics of the participant results (Table E.5).
#[derive(Clone, Debug)]
pub struct Summary {
    pub p: usize,
    pub median: f64,
    pub made: f64,
    pub niqr: f64,
    pub mean: f64,
    pub sd: f64,
    pub robust: Option<AlgorithmA>,
}

/// Screening flag of a reported uncertainty (9.8, ISO 13528 E.4: a, b, c).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UncertaintyFlag {
    /// u_min ≤ u ≤ u_max.
    Plausible,
    /// u < u_min.
    BelowMin,
    /// u > u_max.
    AboveMax,
}

#[derive(Clone, Debug)]
pub struct ParticipantResult {
    pub lab: String,
    pub value: Option<f64>,
    /// "< x" or "> x" for censored results.
    pub censored: Option<(char, f64)>,
    pub u: Option<f64>,
    pub expanded: Option<f64>,
    pub method: Option<String>,
    pub scores: Option<Scores>,
    pub flag: Option<UncertaintyFlag>,
}

/// The assigned value and its uncertainty.
#[derive(Clone, Debug)]
pub struct Assigned {
    pub method: AssignedMethod,
    pub value: f64,
    pub u: f64,
    pub expanded: f64,
}

#[derive(Clone, Debug)]
pub struct ProficiencyAnalysis {
    /// None for a homogeneity/stability study without participant results.
    pub summary: Option<Summary>,
    pub assigned: Option<Assigned>,
    pub coverage_factor: f64,
    pub sd_pt_method: Option<SdPtMethod>,
    pub sigma_pt: Option<f64>,
    pub delta_e: Option<f64>,
    /// u(x_pt) < 0.3 σ_pt (Formula 10).
    pub u_negligible: Option<bool>,
    pub u_min: Option<f64>,
    pub u_max: Option<f64>,
    pub participants: Vec<ParticipantResult>,
    pub homogeneity: Option<Homogeneity>,
    /// Bayesian between-unit posterior, if requested.
    pub between_unit: Option<pt::BetweenUnitPosterior>,
    pub stability: Option<Stability>,
}

fn standard_or_expanded(
    what: &str,
    u: Option<f64>,
    expanded: Option<f64>,
    k: Option<f64>,
) -> Result<Option<(f64, f64, f64)>, ProjectError> {
    // Returns (u, U, k).
    match (u, expanded) {
        (Some(_), Some(_)) => Err(err(what, "give either u or expanded with k, not both")),
        (Some(u), None) if u >= 0.0 => Ok(Some((u, 2.0 * u, 2.0))),
        (None, Some(e)) if e >= 0.0 => {
            let k = k.unwrap_or(2.0);
            if k.is_nan() || k <= 0.0 {
                return Err(err(what, "k must be positive"));
            }
            Ok(Some((e / k, e, k)))
        }
        (None, None) => Ok(None),
        _ => Err(err(what, "uncertainties must be non-negative")),
    }
}

fn add_extra(u: f64, e: &AssignedSpec) -> f64 {
    let sq = |x: Option<f64>| x.map_or(0.0, |v| v * v);
    (u * u + sq(e.u_hom) + sq(e.u_trans) + sq(e.u_stab)).sqrt()
}

fn assigned_value(av: &AssignedSpec, summary: &Option<Summary>, k_pt: f64) -> Result<Assigned, ProjectError> {
    if av.method != AssignedMethodSpec::Reference
        && (av.value.is_some() || av.u.is_some() || av.expanded.is_some() || av.k.is_some())
    {
        return Err(err("assigned_value", "value, u, expanded and k belong to the method \"reference\""));
    }
    if av.u_factor.is_nan() || av.u_factor <= 0.0 {
        return Err(err("assigned_value", "u_factor must be positive"));
    }
    let need_summary =
        || summary.as_ref().ok_or_else(|| err("assigned_value", "this method needs participant results"));
    let p = summary.as_ref().map_or(0, |s| s.p);
    Ok(match av.method {
        AssignedMethodSpec::AlgorithmA => {
            let a = need_summary()?
                .robust
                .clone()
                .ok_or_else(|| err("results", "Algorithm A cannot be applied to these results"))?;
            let u = add_extra(pt::u_assigned_consensus(a.sd, p, av.u_factor), av);
            Assigned { method: AssignedMethod::AlgorithmA, value: a.mean, u, expanded: k_pt * u }
        }
        AssignedMethodSpec::Median => {
            let s = match av.scale {
                ScaleEstimator::Niqr => need_summary()?.niqr,
                ScaleEstimator::Made => need_summary()?.made,
            };
            let u = add_extra(pt::u_assigned_consensus(s, p, av.u_factor), av);
            Assigned { method: AssignedMethod::Median, value: need_summary()?.median, u, expanded: k_pt * u }
        }
        AssignedMethodSpec::Mean => {
            let sm = need_summary()?;
            let u = add_extra(sm.sd / (p as f64).sqrt(), av);
            Assigned { method: AssignedMethod::Mean, value: sm.mean, u, expanded: k_pt * u }
        }
        AssignedMethodSpec::Reference => {
            let value = av.value.ok_or_else(|| err("assigned_value", "the method \"reference\" needs a value"))?;
            let (uc, big_u, _) = standard_or_expanded("assigned_value", av.u, av.expanded, av.k)?
                .ok_or_else(|| err("assigned_value", "a reference value needs u or expanded with k"))?;
            let total = add_extra(uc, av);
            // A stated expanded uncertainty is used as given when no component is added.
            let big_u = if total == uc && av.expanded.is_some() { big_u } else { k_pt * total };
            Assigned { method: AssignedMethod::Reference, value, u: total, expanded: big_u }
        }
    })
}

/// Evaluates one round of a proficiency testing scheme.
pub fn evaluate(spec: &ProficiencySpec) -> Result<ProficiencyAnalysis, ProjectError> {
    let mut values = Vec::new();
    let mut parts = Vec::new();
    for r in &spec.results {
        let what = format!("results[{}]", r.lab);
        let censored = match (r.value, r.less_than, r.greater_than) {
            (Some(v), None, None) if v.is_finite() => {
                values.push(v);
                None
            }
            (None, Some(l), None) => Some(('<', l)),
            (None, None, Some(g)) => Some(('>', g)),
            _ => return Err(err(&what, "give exactly one of value, less_than and greater_than (finite)")),
        };
        let unc = standard_or_expanded(&what, r.u, r.expanded, r.k)?;
        parts.push(ParticipantResult {
            lab: r.lab.clone(),
            value: r.value,
            censored,
            u: unc.map(|x| x.0),
            expanded: unc.map(|x| x.1),
            method: r.method.clone(),
            scores: None,
            flag: None,
        });
    }
    if spec.results.is_empty() && spec.homogeneity.is_none() && spec.stability.is_none() {
        return Err(err("results", "give participant results, a homogeneity check or a stability check"));
    }
    if !spec.results.is_empty() && values.len() < 2 {
        return Err(err("results", "at least two numerical results are needed"));
    }
    let mut labs: Vec<&str> = spec.results.iter().map(|r| r.lab.as_str()).collect();
    labs.sort_unstable();
    if labs.windows(2).any(|w| w[0] == w[1]) {
        return Err(err("results", "laboratory codes must be unique"));
    }
    let p = values.len();
    let stop = match spec.algorithm_a {
        StopSpec::ThirdSignificantFigure => Stopping::ThirdSignificantFigure,
        StopSpec::Converged => Stopping::Converged,
    };
    let summary = if values.len() >= 2 {
        let (mean, sd) = pt::mean_sd(&values)?;
        Some(Summary {
            p,
            median: pt::median(&values)?,
            made: pt::made(&values)?,
            niqr: pt::niqr(&values)?,
            mean,
            sd,
            robust: pt::algorithm_a(&values, stop).ok(),
        })
    } else {
        None
    };
    let need_summary = || summary.as_ref().ok_or_else(|| err("results", "this method needs participant results"));
    let need_robust = || {
        need_summary()?.robust.clone().ok_or_else(|| err("results", "Algorithm A cannot be applied to these results"))
    };

    let k_pt = spec.coverage_factor;
    if k_pt.is_nan() || k_pt <= 0.0 {
        return Err(err("coverage_factor", "must be positive"));
    }
    let assigned = match &spec.assigned_value {
        // A homogeneity/stability study before the round has no consensus value yet.
        None if spec.results.is_empty() => None,
        Some(av) if spec.results.is_empty() && av.method != AssignedMethodSpec::Reference => None,
        None => return Err(err("assigned_value", "missing: needed to score the participant results")),
        Some(av) => Some(assigned_value(av, &summary, k_pt)?),
    };
    let assigned_x = assigned.as_ref().map(|a| a.value);

    let (sd_pt_method, sigma_pt) = match &spec.sd_pt {
        None => (None, None),
        Some(sp) => {
            if sp.method != SdPtMethodSpec::Given && (sp.value.is_some() || sp.percent.is_some()) {
                return Err(err("sd_pt", "value and percent belong to the method \"given\""));
            }
            match sp.method {
                SdPtMethodSpec::Robust => (Some(SdPtMethod::Robust), Some(need_robust()?.sd)),
                SdPtMethodSpec::Made => (Some(SdPtMethod::Made), Some(need_summary()?.made)),
                SdPtMethodSpec::Niqr => (Some(SdPtMethod::Niqr), Some(need_summary()?.niqr)),
                SdPtMethodSpec::Sd => (Some(SdPtMethod::Sd), Some(need_summary()?.sd)),
                SdPtMethodSpec::Given => match (sp.value, sp.percent) {
                    (Some(v), None) => (Some(SdPtMethod::Given), Some(v)),
                    (None, Some(pc)) => {
                        let x = assigned_x.ok_or_else(|| err("sd_pt", "a percentage needs an assigned value"))?;
                        (Some(SdPtMethod::Given), Some(pc / 100.0 * x.abs()))
                    }
                    _ => return Err(err("sd_pt", "the method \"given\" needs either value or percent")),
                },
            }
        }
    };
    if sigma_pt.is_some_and(|s| s.is_nan() || s <= 0.0) {
        return Err(err("sd_pt", "σ_pt must be positive"));
    }
    let delta_e = match (spec.delta_e, spec.delta_e_percent) {
        (Some(_), Some(_)) => return Err(err("delta_e", "give either delta_e or delta_e_percent")),
        (Some(d), None) => Some(d),
        (None, Some(pc)) => {
            Some(pc / 100.0 * assigned_x.ok_or_else(|| err("delta_e_percent", "needs an assigned value"))?.abs())
        }
        (None, None) => sigma_pt.map(|s| 3.0 * s),
    };
    let crit = assigned.as_ref().map(|a| Criteria {
        assigned: a.value,
        u_assigned: a.u,
        expanded_assigned: a.expanded,
        sigma_pt,
        delta_e,
    });
    let u_assigned = assigned.as_ref().map(|a| a.u);

    let limit = |l: &Option<Limit>, what: &str| -> Result<Option<f64>, ProjectError> {
        match l {
            None => Ok(None),
            Some(Limit::Value(v)) => Ok(Some(*v)),
            Some(Limit::Rule(r)) if r == "u_assigned" => Ok(u_assigned),
            Some(Limit::Rule(r)) if r == "1.5_robust_sd" => Ok(Some(1.5 * need_robust()?.sd)),
            Some(Limit::Rule(r)) if r == "1.5_sd_pt" => {
                Ok(Some(1.5 * sigma_pt.ok_or_else(|| err(what, "\"1.5_sd_pt\" needs sd_pt"))?))
            }
            Some(Limit::Rule(r)) => Err(err(
                what,
                format!("unknown rule \"{r}\" (use a number, \"u_assigned\", \"1.5_robust_sd\" or \"1.5_sd_pt\")"),
            )),
        }
    };
    let (u_min, u_max) = match &spec.uncertainty_screening {
        Some(s) => (limit(&s.u_min, "uncertainty_screening.u_min")?, limit(&s.u_max, "uncertainty_screening.u_max")?),
        None => (None, None),
    };
    for part in &mut parts {
        if let (Some(v), Some(c)) = (part.value, crit.as_ref()) {
            part.scores = Some(pt::scores(v, part.u, part.expanded, c));
        }
        if part.value.is_some() && (u_min.is_some() || u_max.is_some()) {
            part.flag = part.u.map(|u| match pt::screen_uncertainty(u, u_min, u_max) {
                std::cmp::Ordering::Less => UncertaintyFlag::BelowMin,
                std::cmp::Ordering::Greater => UncertaintyFlag::AboveMax,
                std::cmp::Ordering::Equal => UncertaintyFlag::Plausible,
            });
        }
    }

    let homogeneity = match &spec.homogeneity {
        None => None,
        Some(h) => {
            let n: usize = h.items.iter().map(|t| t.len()).sum();
            let mean_h = h.items.iter().flatten().sum::<f64>() / n.max(1) as f64;
            let s = match (h.sd_pt, h.sd_pt_percent_of_mean) {
                (Some(_), Some(_)) => return Err(err("homogeneity", "give either sd_pt or sd_pt_percent_of_mean")),
                (Some(v), None) => Some(v),
                (None, Some(pc)) => Some(pc / 100.0 * mean_h.abs()),
                (None, None) => sigma_pt,
            };
            Some(pt::homogeneity(&h.items, s)?)
        }
    };
    let between_unit = match spec.homogeneity.as_ref().and_then(|h| h.bayes.as_ref().map(|b| (h, b))) {
        None => None,
        Some((h, b)) => {
            let mu = match (b.mu_mean, b.mu_sd) {
                (Some(m), Some(sd)) => Some((m, sd)),
                (None, None) => None,
                _ => return Err(err("homogeneity", "give both mu_mean and mu_sd, or neither")),
            };
            let prior = pt::BetweenUnitPrior { tau_scale: b.tau_scale, sigma_scale: b.sigma_scale, mu };
            Some(pt::between_unit_posterior(&h.items, &prior)?)
        }
    };
    let stability = match &spec.stability {
        None => None,
        Some(st) => {
            if st.values.is_empty() {
                return Err(err("stability", "no values"));
            }
            let y2 = st.values.iter().sum::<f64>() / st.values.len() as f64;
            let y1 = st
                .reference
                .or_else(|| homogeneity.as_ref().map(|h| h.mean))
                .ok_or_else(|| err("stability", "give reference or a homogeneity check"))?;
            let s = homogeneity
                .as_ref()
                .and_then(|h| h.criterion.map(|c| c / 0.3))
                .or(sigma_pt)
                .ok_or_else(|| err("stability", "needs σ_pt"))?;
            let u = match (st.u_reference, st.u_values) {
                (Some(a), Some(b)) => Some((a, b)),
                (None, None) => None,
                _ => return Err(err("stability", "give both u_reference and u_values, or neither")),
            };
            Some(pt::stability(y1, y2, s, u))
        }
    };

    Ok(ProficiencyAnalysis {
        u_negligible: sigma_pt.zip(u_assigned).map(|(s, u)| u < 0.3 * s),
        summary,
        assigned,
        coverage_factor: k_pt,
        sd_pt_method,
        sigma_pt,
        delta_e,
        u_min,
        u_max,
        participants: parts,
        homogeneity,
        between_unit,
        stability,
    })
}

pub fn signal_code(s: Signal) -> &'static str {
    match s {
        Signal::Acceptable => "acceptable",
        Signal::Warning => "warning",
        Signal::Action => "action",
    }
}

pub fn flag_code(f: UncertaintyFlag) -> &'static str {
    match f {
        UncertaintyFlag::Plausible => "a",
        UncertaintyFlag::BelowMin => "b",
        UncertaintyFlag::AboveMax => "c",
    }
}

/// Compares the analysis with the expected values.
pub fn check(e: &ExpectedProficiency, a: &ProficiencyAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut push = |what: String, computed: Option<f64>, c: Option<&Check>| {
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
    let s = a.summary.as_ref();
    let r = s.and_then(|s| s.robust.as_ref());
    push("proficiency.assigned_value".into(), a.assigned.as_ref().map(|x| x.value), e.assigned_value.as_ref());
    push("proficiency.u_assigned".into(), a.assigned.as_ref().map(|x| x.u), e.u_assigned.as_ref());
    push("proficiency.sd_pt".into(), a.sigma_pt, e.sd_pt.as_ref());
    push("proficiency.median".into(), s.map(|s| s.median), e.median.as_ref());
    push("proficiency.made".into(), s.map(|s| s.made), e.made.as_ref());
    push("proficiency.niqr".into(), s.map(|s| s.niqr), e.niqr.as_ref());
    push("proficiency.robust_mean".into(), r.map(|r| r.mean), e.robust_mean.as_ref());
    push("proficiency.robust_sd".into(), r.map(|r| r.sd), e.robust_sd.as_ref());
    push("proficiency.iterations".into(), r.map(|r| r.iterations as f64), e.iterations.as_ref());
    push("proficiency.mean".into(), s.map(|s| s.mean), e.mean.as_ref());
    push("proficiency.sd".into(), s.map(|s| s.sd), e.sd.as_ref());
    for (lab, es) in &e.scores {
        let sc = a.participants.iter().find(|p| &p.lab == lab).and_then(|p| p.scores.clone());
        let g = |f: fn(&Scores) -> Option<f64>| sc.as_ref().and_then(f);
        push(format!("proficiency.scores[{lab}].d"), sc.as_ref().map(|s| s.d), es.d.as_ref());
        push(format!("proficiency.scores[{lab}].d_percent"), g(|s| s.d_percent), es.d_percent.as_ref());
        push(format!("proficiency.scores[{lab}].pa"), g(|s| s.pa), es.pa.as_ref());
        push(format!("proficiency.scores[{lab}].z"), g(|s| s.z), es.z.as_ref());
        push(format!("proficiency.scores[{lab}].z_prime"), g(|s| s.z_prime), es.z_prime.as_ref());
        push(format!("proficiency.scores[{lab}].zeta"), g(|s| s.zeta), es.zeta.as_ref());
        push(format!("proficiency.scores[{lab}].en"), g(|s| s.en), es.en.as_ref());
    }
    let mut verdicts = Vec::new();
    if let Some(eh) = &e.homogeneity {
        let h = a.homogeneity.as_ref();
        push("proficiency.homogeneity.mean".into(), h.map(|h| h.mean), eh.mean.as_ref());
        push("proficiency.homogeneity.s_x".into(), h.map(|h| h.s_x), eh.s_x.as_ref());
        push("proficiency.homogeneity.s_w".into(), h.map(|h| h.s_w), eh.s_w.as_ref());
        push("proficiency.homogeneity.s_s".into(), h.map(|h| h.s_s), eh.s_s.as_ref());
        push("proficiency.homogeneity.criterion".into(), h.and_then(|h| h.criterion), eh.criterion.as_ref());
        push("proficiency.homogeneity.ms_between".into(), h.map(|h| h.ms_between), eh.ms_between.as_ref());
        push("proficiency.homogeneity.ms_within".into(), h.map(|h| h.ms_within), eh.ms_within.as_ref());
        push("proficiency.homogeneity.u_bb_star".into(), h.map(|h| h.u_bb_star), eh.u_bb_star.as_ref());
        let b = a.between_unit.as_ref();
        push("proficiency.homogeneity.tau_mean".into(), b.map(|b| b.tau.mean), eh.tau_mean.as_ref());
        push("proficiency.homogeneity.tau_sd".into(), b.map(|b| b.tau.sd), eh.tau_sd.as_ref());
        push("proficiency.homogeneity.tau_median".into(), b.map(|b| b.tau.median), eh.tau_median.as_ref());
        push("proficiency.homogeneity.tau_mode".into(), b.map(|b| b.tau.mode), eh.tau_mode.as_ref());
        push("proficiency.homogeneity.tau_q975".into(), b.map(|b| b.tau.q975), eh.tau_q975.as_ref());
        push("proficiency.homogeneity.tau_hpd_upper".into(), b.map(|b| b.tau.hpd.1), eh.tau_hpd_upper.as_ref());
        push("proficiency.homogeneity.sigma_mean".into(), b.map(|b| b.sigma.mean), eh.sigma_mean.as_ref());
        push("proficiency.homogeneity.sigma_hpd_lower".into(), b.map(|b| b.sigma.hpd.0), eh.sigma_hpd_lower.as_ref());
        push("proficiency.homogeneity.sigma_hpd_upper".into(), b.map(|b| b.sigma.hpd.1), eh.sigma_hpd_upper.as_ref());
        push("proficiency.homogeneity.mu_mean".into(), b.map(|b| b.mu_mean), eh.mu_mean.as_ref());
        push("proficiency.homogeneity.mu_sd".into(), b.map(|b| b.mu_sd), eh.mu_sd.as_ref());
        if let Some(v) = eh.sufficient {
            verdicts.push(("proficiency.homogeneity.sufficient", h.and_then(|h| h.sufficient), v));
        }
    }
    if let Some(es) = &e.stability {
        let st = a.stability.as_ref();
        push("proficiency.stability.difference".into(), st.map(|s| s.difference), es.difference.as_ref());
        if let Some(v) = es.sufficient {
            verdicts.push(("proficiency.stability.sufficient", st.map(|s| s.sufficient), v));
        }
    }
    for (what, got, want) in verdicts {
        let computed = got.map_or(f64::NAN, |b| if b { 1.0 } else { 0.0 });
        let expected = if want { 1.0 } else { 0.0 };
        out.push(crate::CheckOutcome {
            what: what.into(),
            computed,
            expected,
            tolerance: 0.0,
            passed: computed == expected,
        });
    }
}

pub fn results_json(a: &ProficiencyAnalysis) -> serde_json::Value {
    use serde_json::json;
    let s = a.summary.as_ref();
    let participants: Vec<_> = a
        .participants
        .iter()
        .map(|p| {
            let sc = p.scores.as_ref();
            json!({
                "lab": p.lab,
                "value": p.value,
                "censored": p.censored.map(|(c, v)| format!("{c} {v}")),
                "u": p.u,
                "expanded": p.expanded,
                "method": p.method,
                "d": sc.map(|s| s.d),
                "d_percent": sc.and_then(|s| s.d_percent),
                "pa": sc.and_then(|s| s.pa),
                "z": sc.and_then(|s| s.z),
                "z_prime": sc.and_then(|s| s.z_prime),
                "zeta": sc.and_then(|s| s.zeta),
                "en": sc.and_then(|s| s.en),
                "z_signal": sc.and_then(|s| s.z).map(|z| signal_code(pt::z_signal(z))),
                "zeta_signal": sc.and_then(|s| s.zeta).map(|z| signal_code(pt::z_signal(z))),
                "en_satisfactory": sc.and_then(|s| s.en).map(pt::en_satisfactory),
                "flag": p.flag.map(flag_code),
            })
        })
        .collect();
    json!({
        "summary": s.map(|s| json!({
            "p": s.p, "median": s.median, "made": s.made, "niqr": s.niqr, "mean": s.mean, "sd": s.sd,
            "robust": s.robust.as_ref().map(|r| json!({"mean": r.mean, "sd": r.sd, "iterations": r.iterations,
                "history": r.history, "initial_sd_substituted": r.initial_sd_substituted})),
        })),
        "assigned": a.assigned.as_ref().map(|x| json!({"method": format!("{:?}", x.method), "value": x.value, "u": x.u, "expanded": x.expanded})),
        "coverage_factor": a.coverage_factor,
        "sd_pt": a.sigma_pt,
        "delta_e": a.delta_e,
        "u_negligible": a.u_negligible,
        "u_min": a.u_min,
        "u_max": a.u_max,
        "participants": participants,
        "homogeneity": a.homogeneity.as_ref().map(|h| json!({"items": h.items, "replicates": h.replicates, "mean": h.mean,
            "s_x": h.s_x, "s_w": h.s_w, "s_s": h.s_s, "criterion": h.criterion, "sufficient": h.sufficient,
            "f1": h.f1, "fm": h.fm, "expanded_criterion": h.expanded_criterion, "sufficient_expanded": h.sufficient_expanded,
            "ms_between": h.ms_between, "ms_within": h.ms_within, "u_bb_star": h.u_bb_star})),
        "between_unit": a.between_unit.as_ref().map(|b| {
            let m = |p: &pt::PosteriorSummary| json!({"mean": p.mean, "sd": p.sd, "median": p.median, "mode": p.mode,
                "q025": p.q025, "q975": p.q975, "hpd": [p.hpd.0, p.hpd.1]});
            json!({"tau": m(&b.tau), "sigma": m(&b.sigma), "mu_mean": b.mu_mean, "mu_sd": b.mu_sd})
        }),
        "stability": a.stability.as_ref().map(|s| json!({"difference": s.difference, "criterion": s.criterion, "sufficient": s.sufficient})),
    })
}
