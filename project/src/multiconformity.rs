//! Project section `multicomponent_conformity`: total risks of false
//! decisions for an item with several correlated properties (EMUE E2.1).

use crate::schema::Check;
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use uncertainty_core::multiconformity::{GlobalTotal, Multicomponent, SpecificTotal};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: format!("multicomponent_conformity.{what}"), message: message.into() }
}

fn default_samples() -> usize {
    200_000
}

fn default_seed() -> u64 {
    1
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentSpec {
    pub name: String,
    /// Mean and standard deviation of the prior (production) PDF.
    pub prior_mean: f64,
    pub prior_sd: f64,
    /// Tolerance limits, which are also the acceptance limits (simple
    /// acceptance).
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiResultSpec {
    pub name: String,
    /// One test result per component, in the order of `components`.
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MulticomponentSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub components: Vec<ComponentSpec>,
    /// Correlation matrix of the true values, also used for the test results.
    pub correlation: Vec<Vec<f64>>,
    /// Relative standard uncertainty of every test result.
    pub u_relative: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<MultiResultSpec>,
    /// Monte Carlo trials for the total global risks (none if absent).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_trials: Option<usize>,
    /// Draws of the Genz integration for the specific risks.
    #[serde(default = "default_samples")]
    pub samples: usize,
    #[serde(default = "default_seed")]
    pub seed: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedMultiResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedMulticomponent {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub results: BTreeMap<String, ExpectedMultiResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_consumer_risk: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_producer_risk: Option<Check>,
}

#[derive(Clone, Debug)]
pub struct MulticomponentAnalysis {
    pub unit: Option<String>,
    pub names: Vec<String>,
    pub model: Multicomponent,
    pub results: Vec<(String, Vec<f64>, SpecificTotal)>,
    pub global: Option<GlobalTotal>,
    pub samples: usize,
}

pub fn evaluate(spec: &MulticomponentSpec) -> Result<MulticomponentAnalysis, ProjectError> {
    let d = spec.components.len();
    if d < 2 {
        return Err(err("components", "give at least two components"));
    }
    let model = Multicomponent {
        prior_mean: spec.components.iter().map(|c| c.prior_mean).collect(),
        prior_sd: spec.components.iter().map(|c| c.prior_sd).collect(),
        correlation: spec.correlation.clone(),
        u_rel: spec.u_relative,
        lower: spec.components.iter().map(|c| c.lower).collect(),
        upper: spec.components.iter().map(|c| c.upper).collect(),
    };
    if spec.results.is_empty() && spec.global_trials.is_none() {
        return Err(err("results", "give test results or global_trials"));
    }
    let mut results = Vec::new();
    for (k, r) in spec.results.iter().enumerate() {
        if r.values.len() != d {
            return Err(err("results", format!("{}: give one value per component", r.name)));
        }
        let s = model
            .specific(&r.values, spec.samples, spec.seed.wrapping_add(k as u64))
            .map_err(|e| err("results", e.to_string()))?;
        results.push((r.name.clone(), r.values.clone(), s));
    }
    let global = match spec.global_trials {
        Some(n) => Some(model.global(n, spec.seed).map_err(|e| err("global_trials", e.to_string()))?),
        None => {
            // Validate the model even without global risks.
            model.global(0, spec.seed).map_err(|e| err("components", e.to_string()))?;
            None
        }
    };
    Ok(MulticomponentAnalysis {
        unit: spec.unit.clone(),
        names: spec.components.iter().map(|c| c.name.clone()).collect(),
        model,
        results,
        global,
        samples: spec.samples,
    })
}

pub fn check(e: &ExpectedMulticomponent, a: &MulticomponentAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut push = |what: String, computed: f64, expected: f64, tolerance: f64| {
        out.push(crate::CheckOutcome {
            what: format!("multicomponent_conformity.{what}"),
            computed,
            expected,
            tolerance,
            passed: (computed - expected).abs() <= tolerance,
        });
    };
    for (name, er) in &e.results {
        let r = a.results.iter().find(|r| &r.0 == name);
        if let Some(c) = &er.risk {
            push(format!("results[{name}].risk"), r.map_or(f64::NAN, |r| r.2.risk), c.value, c.tolerance);
        }
        if let Some(acc) = er.accepted {
            let v = |b: bool| if b { 1.0 } else { 0.0 };
            push(format!("results[{name}].accepted"), r.map_or(f64::NAN, |r| v(r.2.accepted)), v(acc), 0.0);
        }
    }
    if let Some(c) = &e.global_consumer_risk {
        push("global_consumer_risk".into(), a.global.map_or(f64::NAN, |g| g.consumer), c.value, c.tolerance);
    }
    if let Some(c) = &e.global_producer_risk {
        push("global_producer_risk".into(), a.global.map_or(f64::NAN, |g| g.producer), c.value, c.tolerance);
    }
}

pub fn results_json(a: &MulticomponentAnalysis) -> Value {
    json!({
        "components": a.names,
        "u_relative": a.model.u_rel,
        "results": a.results.iter().map(|(name, v, s)| json!({
            "name": name, "values": v, "posterior_mean": s.posterior_mean,
            "conformance_probability": s.conformance_probability, "standard_error": s.standard_error,
            "accepted": s.accepted, "risk": s.risk,
        })).collect::<Vec<_>>(),
        "global": a.global.map(|g| json!({"consumer_risk": g.consumer, "producer_risk": g.producer, "trials": g.trials})),
    })
}
