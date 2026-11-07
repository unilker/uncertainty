//! Project section `consensus`: consensus value of an interlaboratory study
//! or key comparison with degrees of equivalence (NIST Consensus Builder;
//! Koepke et al. 2017).

use crate::schema::Check;
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use uncertainty_core::consensus::{self, ConsensusSample, DerSimonianLaird, MaximumLikelihood};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: format!("consensus.{what}"), message: message.into() }
}

fn default_coverage() -> f64 {
    0.95
}

fn default_trials() -> usize {
    100_000
}

fn default_seed() -> u64 {
    1
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabResult {
    pub name: String,
    pub value: f64,
    /// Standard uncertainty.
    pub u: f64,
    /// Degrees of freedom of u (absent: infinitely many).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dof: Option<f64>,
    /// Weight in the Linear Pool (absent: 1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsensusSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub results: Vec<LabResult>,
    #[serde(default = "default_coverage")]
    pub coverage: f64,
    /// DerSimonian–Laird with the parametric bootstrap.
    #[serde(default = "default_true")]
    pub dersimonian_laird: bool,
    /// Linear Pool.
    #[serde(default)]
    pub linear_pool: bool,
    /// Maximum-likelihood estimate of the random-effects model.
    #[serde(default)]
    pub maximum_likelihood: bool,
    /// Bootstrap replicates (and Linear Pool draws).
    #[serde(default = "default_trials")]
    pub trials: usize,
    #[serde(default = "default_seed")]
    pub seed: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedDoe {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedConsensus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<Check>,
    /// Naive u_DL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tau: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_knapp_hartung: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bootstrap_u: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_lower: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_upper: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear_pool_mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear_pool_u: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear_pool_lower: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear_pool_upper: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_u: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_tau: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_u_tau: Option<Check>,
    /// Unilateral degrees of equivalence of the DerSimonian–Laird procedure.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub degrees_of_equivalence: BTreeMap<String, ExpectedDoe>,
}

#[derive(Clone, Debug)]
pub struct ConsensusAnalysis {
    pub measurand: Option<String>,
    pub unit: Option<String>,
    pub names: Vec<String>,
    pub x: Vec<f64>,
    pub u: Vec<f64>,
    pub dof: Vec<f64>,
    pub coverage: f64,
    pub dl: DerSimonianLaird,
    pub knapp_hartung_expanded: f64,
    pub bootstrap: Option<ConsensusSample>,
    pub linear_pool: Option<ConsensusSample>,
    pub maximum_likelihood: Option<MaximumLikelihood>,
}

pub fn evaluate(spec: &ConsensusSpec) -> Result<ConsensusAnalysis, ProjectError> {
    let r = &spec.results;
    let x: Vec<f64> = r.iter().map(|a| a.value).collect();
    let u: Vec<f64> = r.iter().map(|a| a.u).collect();
    let dof: Vec<f64> = if r.iter().any(|a| a.dof.is_some()) {
        r.iter().map(|a| a.dof.unwrap_or(f64::INFINITY)).collect()
    } else {
        Vec::new()
    };
    let weights: Vec<f64> = if r.iter().any(|a| a.weight.is_some()) {
        r.iter().map(|a| a.weight.unwrap_or(1.0)).collect()
    } else {
        Vec::new()
    };
    if !spec.dersimonian_laird && !spec.linear_pool && !spec.maximum_likelihood {
        return Err(err(
            "dersimonian_laird",
            "choose at least one of DerSimonian–Laird, the Linear Pool and maximum likelihood",
        ));
    }
    let mut names: Vec<&str> = r.iter().map(|a| a.name.as_str()).collect();
    names.sort_unstable();
    if names.windows(2).any(|w| w[0] == w[1]) {
        return Err(err("results", "the names of the results must differ"));
    }
    let dl = consensus::dersimonian_laird(&x, &u).map_err(|e| err("results", e.to_string()))?;
    let bootstrap = if spec.dersimonian_laird {
        Some(
            consensus::dl_bootstrap(&x, &u, &dof, spec.coverage, spec.trials, spec.seed)
                .map_err(|e| err("trials", e.to_string()))?,
        )
    } else {
        None
    };
    let linear_pool = if spec.linear_pool {
        Some(
            consensus::linear_pool(&x, &u, &dof, &weights, spec.coverage, spec.trials, spec.seed)
                .map_err(|e| err("linear_pool", e.to_string()))?,
        )
    } else {
        None
    };
    let maximum_likelihood = if spec.maximum_likelihood {
        Some(consensus::maximum_likelihood(&x, &u).map_err(|e| err("maximum_likelihood", e.to_string()))?)
    } else {
        None
    };
    Ok(ConsensusAnalysis {
        measurand: spec.measurand.clone(),
        unit: spec.unit.clone(),
        names: r.iter().map(|a| a.name.clone()).collect(),
        knapp_hartung_expanded: consensus::knapp_hartung_expanded(&dl, x.len(), spec.coverage),
        x,
        u,
        dof,
        coverage: spec.coverage,
        dl,
        bootstrap,
        linear_pool,
        maximum_likelihood,
    })
}

pub fn check(e: &ExpectedConsensus, a: &ConsensusAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut push = |what: String, computed: f64, c: &Option<Check>| {
        if let Some(c) = c {
            out.push(crate::CheckOutcome {
                what: format!("consensus.{what}"),
                computed,
                expected: c.value,
                tolerance: c.tolerance,
                passed: (computed - c.value).abs() <= c.tolerance,
            });
        }
    };
    let b = a.bootstrap.as_ref();
    let lp = a.linear_pool.as_ref();
    push("mean".into(), a.dl.mean, &e.mean);
    push("u".into(), a.dl.u, &e.u);
    push("tau".into(), a.dl.tau, &e.tau);
    push("q".into(), a.dl.q, &e.q);
    push("u_knapp_hartung".into(), a.dl.u_knapp_hartung, &e.u_knapp_hartung);
    push("bootstrap_u".into(), b.map_or(f64::NAN, |b| b.u), &e.bootstrap_u);
    push("interval_lower".into(), b.map_or(f64::NAN, |b| b.interval.0), &e.interval_lower);
    push("interval_upper".into(), b.map_or(f64::NAN, |b| b.interval.1), &e.interval_upper);
    push("linear_pool_mean".into(), lp.map_or(f64::NAN, |b| b.mean), &e.linear_pool_mean);
    push("linear_pool_u".into(), lp.map_or(f64::NAN, |b| b.u), &e.linear_pool_u);
    push("linear_pool_lower".into(), lp.map_or(f64::NAN, |b| b.interval.0), &e.linear_pool_lower);
    push("linear_pool_upper".into(), lp.map_or(f64::NAN, |b| b.interval.1), &e.linear_pool_upper);
    let ml = a.maximum_likelihood.as_ref();
    push("ml_mean".into(), ml.map_or(f64::NAN, |m| m.mean), &e.ml_mean);
    push("ml_u".into(), ml.map_or(f64::NAN, |m| m.u_mean), &e.ml_u);
    push("ml_tau".into(), ml.map_or(f64::NAN, |m| m.tau), &e.ml_tau);
    push("ml_u_tau".into(), ml.map_or(f64::NAN, |m| m.u_tau), &e.ml_u_tau);
    for (name, ed) in &e.degrees_of_equivalence {
        let j = a.names.iter().position(|n| n == name);
        let doe = j.and_then(|j| b.map(|b| &b.degrees_of_equivalence[j]));
        push(format!("degrees_of_equivalence[{name}].d"), doe.map_or(f64::NAN, |d| d.d), &ed.d);
        push(format!("degrees_of_equivalence[{name}].expanded"), doe.map_or(f64::NAN, |d| d.expanded), &ed.expanded);
    }
}

fn sample_json(s: &ConsensusSample, names: &[String]) -> Value {
    json!({
        "mean": s.mean, "standard_uncertainty": s.u,
        "interval": [s.interval.0, s.interval.1], "coverage": s.coverage, "trials": s.trials,
        "degrees_of_equivalence": s.degrees_of_equivalence.iter().zip(names).map(|(d, n)| json!({
            "name": n, "d": d.d, "expanded": d.expanded,
        })).collect::<Vec<_>>(),
    })
}

pub fn results_json(a: &ConsensusAnalysis) -> Value {
    json!({
        "results": a.names.iter().enumerate().map(|(j, n)| json!({
            "name": n, "value": a.x[j], "u": a.u[j], "dof": a.dof.get(j).filter(|v| v.is_finite()),
            "weight": a.dl.weights[j],
        })).collect::<Vec<_>>(),
        "dersimonian_laird": {
            "mean": a.dl.mean, "standard_uncertainty": a.dl.u, "tau": a.dl.tau,
            "q": a.dl.q, "q_p_value": a.dl.q_p_value,
            "u_knapp_hartung": a.dl.u_knapp_hartung, "expanded_knapp_hartung": a.knapp_hartung_expanded,
            "bootstrap": a.bootstrap.as_ref().map(|b| sample_json(b, &a.names)),
        },
        "linear_pool": a.linear_pool.as_ref().map(|b| sample_json(b, &a.names)),
        "maximum_likelihood": a.maximum_likelihood.as_ref().map(|m| json!({
            "mean": m.mean, "standard_uncertainty": m.u_mean, "tau": m.tau, "u_tau": m.u_tau,
        })),
    })
}
