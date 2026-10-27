//! Interlaboratory precision experiments by the basic method of
//! ISO 5725-2:2019: the results of p laboratories at q levels (Form A), the
//! scrutiny for consistency and outliers, the exclusions decided by the panel,
//! the repeatability and reproducibility standard deviations at each level
//! and their dependence on the level.

use crate::schema::{Check, Text};
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uncertainty_core::interlab::{
    self, Cell, CochranTest, GrubbsTests, LevelEstimates, Outcome, Relationship, RelationshipKind,
};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: what.to_string(), message: message.into() }
}

// ---------------------------------------------------------------------------
// Specification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterlabSpec {
    /// The property measured, e.g. "Sulfur content of coal".
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Names of the laboratories, in the order of the results of each level.
    pub laboratories: Vec<String>,
    /// Form A: the results of each laboratory at each level.
    pub levels: Vec<LevelSpec>,
    /// Cells, laboratories or single results left out after scrutiny
    /// (8.3.3.1, 8.6.8–8.6.10).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<ExclusionSpec>,
    /// Calculation of the mean and the variances (8.4.6).
    #[serde(default, skip_serializing_if = "is_anova")]
    pub method: MethodSpec,
    /// Decimal places of the cell means and standard deviations as recorded
    /// in Forms B and C (8.2.10, 8.2.11); full precision when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_decimals: Option<u32>,
    /// Relationships between s and m fitted when there are three or more
    /// levels (8.5); all four by default.
    #[serde(default = "all_relationships", skip_serializing_if = "is_all_relationships")]
    pub relationships: Vec<RelationshipSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelSpec {
    pub name: String,
    /// One list per laboratory; an empty list is a missing cell.
    pub results: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExclusionSpec {
    pub laboratory: String,
    /// The level; all levels of the laboratory when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// A single result of the cell (1-based); the whole cell when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Text>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MethodSpec {
    /// Formulae 23–31 (8.4.4, 8.4.5).
    #[default]
    Anova,
    /// Restricted maximum likelihood (Annex B.2).
    Reml,
}

fn is_anova(m: &MethodSpec) -> bool {
    *m == MethodSpec::Anova
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipSpec {
    I,
    II,
    III,
    IV,
}

impl RelationshipSpec {
    pub const ALL: [RelationshipSpec; 4] =
        [RelationshipSpec::I, RelationshipSpec::II, RelationshipSpec::III, RelationshipSpec::IV];

    pub fn kind(self) -> RelationshipKind {
        match self {
            RelationshipSpec::I => RelationshipKind::Proportional,
            RelationshipSpec::II => RelationshipKind::Linear,
            RelationshipSpec::III => RelationshipKind::Quadratic,
            RelationshipSpec::IV => RelationshipKind::Power,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            RelationshipSpec::I => "I",
            RelationshipSpec::II => "II",
            RelationshipSpec::III => "III",
            RelationshipSpec::IV => "IV",
        }
    }
}

fn all_relationships() -> Vec<RelationshipSpec> {
    RelationshipSpec::ALL.to_vec()
}

fn is_all_relationships(v: &[RelationshipSpec]) -> bool {
    v == RelationshipSpec::ALL
}

// ---------------------------------------------------------------------------
// Expected values
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedInterlab {
    /// By level name.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub levels: BTreeMap<String, ExpectedLevel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<ExpectedRelationship>,
}

/// Expected values at one level. The test statistics refer to the scrutiny of
/// all results (before exclusions); those prefixed `final_` to the retained
/// results.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_r: Option<Check>,
    #[serde(default, rename = "s_L", skip_serializing_if = "Option::is_none")]
    pub s_l: Option<Check>,
    #[serde(default, rename = "s_R", skip_serializing_if = "Option::is_none")]
    pub s_rr: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cochran: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grubbs_low: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grubbs_high: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grubbs_double_low: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grubbs_double_high: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_cochran: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedRelationship {
    /// "s_r" or "s_R".
    pub of: String,
    pub relationship: RelationshipSpec,
    /// As printed: b (I); a, b (II); a_v, b_v (III); c, d (IV).
    pub params: Vec<Check>,
}

// ---------------------------------------------------------------------------
// Results
// ---------------------------------------------------------------------------

/// Scrutiny of one set of cells (8.3); laboratory indices refer to
/// [`InterlabSpec::laboratories`].
#[derive(Clone, Debug)]
pub struct Scrutiny {
    /// Laboratories with a non-empty cell.
    pub labs: Vec<usize>,
    pub h: Vec<f64>,
    pub k: Vec<Option<f64>>,
    /// Indicators of h (two-sided) and k at 5 % and 1 % (Tables 7, 8).
    pub h_indicators: Option<(f64, f64)>,
    pub k_indicators: Option<(f64, f64)>,
    /// Cochran's tests, cells mapped to laboratories.
    pub cochran: Vec<CochranTest>,
    /// Grubbs' tests, indices mapped to laboratories.
    pub grubbs: Option<GrubbsTests>,
    /// Majority number of results per cell.
    pub n: usize,
}

impl Scrutiny {
    /// The worst outcome of the tests for the cell of laboratory `lab`:
    /// (Cochran, Grubbs).
    pub fn outcomes(&self, lab: usize) -> (Outcome, Outcome) {
        let worst = |a: Outcome, b: Outcome| if rank(b) > rank(a) { b } else { a };
        let c = self.cochran.iter().filter(|t| t.cell == lab).fold(Outcome::Correct, |o, t| worst(o, t.outcome));
        let mut g = Outcome::Correct;
        if let Some(t) = &self.grubbs {
            for s in [Some(&t.low), Some(&t.high), t.repeated.as_ref()].into_iter().flatten() {
                if s.index == lab {
                    g = worst(g, s.outcome);
                }
            }
            for d in [t.double_low.as_ref(), t.double_high.as_ref()].into_iter().flatten() {
                if d.indices.contains(&lab) {
                    g = worst(g, d.outcome);
                }
            }
        }
        (c, g)
    }
}

fn rank(o: Outcome) -> u8 {
    match o {
        Outcome::Correct => 0,
        Outcome::Straggler => 1,
        Outcome::Outlier => 2,
    }
}

#[derive(Clone, Debug)]
pub struct LevelResult {
    pub name: String,
    /// Forms B and C of all results, by laboratory (rounded as recorded).
    pub cells: Vec<Option<Cell>>,
    /// Forms B and C of the retained results.
    pub retained: Vec<Option<Cell>>,
    /// Laboratories with results left out at this level.
    pub excluded: Vec<usize>,
    pub initial: Scrutiny,
    /// Scrutiny of the retained results, when results were left out.
    pub final_scrutiny: Option<Scrutiny>,
    pub estimates: LevelEstimates,
    /// Annex A: A_r and A_R (with γ = s_R/s_r) for p and the majority n.
    pub a_r: Option<f64>,
    pub a_rr: Option<f64>,
}

impl LevelResult {
    /// The scrutiny of the data the estimates are based on.
    pub fn retained_scrutiny(&self) -> &Scrutiny {
        self.final_scrutiny.as_ref().unwrap_or(&self.initial)
    }
}

#[derive(Clone, Debug)]
pub struct RelationshipFit {
    pub of_reproducibility: bool,
    pub relationship: RelationshipSpec,
    pub fit: Result<Relationship, String>,
}

#[derive(Clone, Debug)]
pub struct InterlabAnalysis {
    pub quantity: String,
    pub unit: Option<String>,
    pub laboratories: Vec<String>,
    pub method: MethodSpec,
    pub levels: Vec<LevelResult>,
    pub relationships: Vec<RelationshipFit>,
}

fn scrutiny(cells: &[Option<Cell>]) -> Scrutiny {
    let labs: Vec<usize> = (0..cells.len()).filter(|&i| cells[i].is_some()).collect();
    let present: Vec<Cell> = labs.iter().filter_map(|&i| cells[i]).collect();
    let p = present.len();
    let with_sd: Vec<Cell> = present.iter().copied().filter(|c| c.sd.is_some()).collect();
    let n = interlab::majority_n(&with_sd);
    let mut cochran = interlab::cochran_tests(&present);
    for t in &mut cochran {
        t.cell = labs[t.cell];
    }
    let means: Vec<f64> = present.iter().map(|c| c.mean).collect();
    let grubbs = interlab::grubbs_tests(&means).map(|mut g| {
        g.low.index = labs[g.low.index];
        g.high.index = labs[g.high.index];
        if let Some(r) = &mut g.repeated {
            r.index = labs[r.index];
        }
        for d in [&mut g.double_low, &mut g.double_high].into_iter().flatten() {
            d.indices = [labs[d.indices[0]], labs[d.indices[1]]];
        }
        g
    });
    Scrutiny {
        h: interlab::mandel_h(&present),
        k: interlab::mandel_k(&present),
        h_indicators: (p >= 3).then(|| (interlab::mandel_h_indicator(p, 0.05), interlab::mandel_h_indicator(p, 0.01))),
        k_indicators: (with_sd.len() >= 2 && n >= 2).then(|| {
            (interlab::mandel_k_indicator(with_sd.len(), n, 0.05), interlab::mandel_k_indicator(with_sd.len(), n, 0.01))
        }),
        labs,
        cochran,
        grubbs,
        n,
    }
}

/// Evaluates a precision experiment.
pub fn evaluate(spec: &InterlabSpec) -> Result<InterlabAnalysis, ProjectError> {
    let nl = spec.laboratories.len();
    if nl < 2 {
        return Err(err("laboratories", "at least two laboratories are needed"));
    }
    let mut names: Vec<&str> = spec.laboratories.iter().map(String::as_str).collect();
    names.sort_unstable();
    if names.windows(2).any(|w| w[0] == w[1]) {
        return Err(err("laboratories", "names must be unique"));
    }
    if spec.levels.is_empty() {
        return Err(err("levels", "at least one level is needed"));
    }
    let mut level_names: Vec<&str> = spec.levels.iter().map(|l| l.name.as_str()).collect();
    level_names.sort_unstable();
    if level_names.windows(2).any(|w| w[0] == w[1]) {
        return Err(err("levels", "names must be unique"));
    }
    for l in &spec.levels {
        if l.results.len() != nl {
            return Err(err(
                &format!("levels[{}]", l.name),
                format!("{} lists of results given for {nl} laboratories", l.results.len()),
            ));
        }
        if l.results.iter().flatten().any(|y| !y.is_finite()) {
            return Err(err(&format!("levels[{}]", l.name), "results must be finite numbers"));
        }
    }
    // Exclusions: (level, lab) → None for the whole cell, or result indices.
    let mut drop: BTreeMap<(usize, usize), Option<Vec<usize>>> = BTreeMap::new();
    for x in &spec.exclude {
        let lab = spec
            .laboratories
            .iter()
            .position(|l| *l == x.laboratory)
            .ok_or_else(|| err("exclude", format!("no laboratory named \"{}\"", x.laboratory)))?;
        let levels: Vec<usize> = match &x.level {
            None => (0..spec.levels.len()).collect(),
            Some(name) => vec![spec
                .levels
                .iter()
                .position(|l| l.name == *name)
                .ok_or_else(|| err("exclude", format!("no level named \"{name}\"")))?],
        };
        for j in levels {
            match x.result {
                None => {
                    drop.insert((j, lab), None);
                }
                Some(r) => {
                    if x.level.is_none() {
                        return Err(err("exclude", "a single result needs its level"));
                    }
                    if r == 0 || r > spec.levels[j].results[lab].len() {
                        return Err(err(
                            "exclude",
                            format!("cell ({}, {}) has no result {r}", x.laboratory, spec.levels[j].name),
                        ));
                    }
                    if let Some(v) = drop.entry((j, lab)).or_insert_with(|| Some(Vec::new())) {
                        v.push(r - 1);
                    }
                }
            }
        }
    }
    let form = |results: &[f64]| Cell::from_results(results).map(|c| spec.form_decimals.map_or(c, |d| c.rounded(d)));
    let mut levels = Vec::new();
    for (j, l) in spec.levels.iter().enumerate() {
        let cells: Vec<Option<Cell>> = l.results.iter().map(|r| form(r)).collect();
        let mut kept: Vec<Vec<f64>> = Vec::new();
        let mut excluded = Vec::new();
        for (i, r) in l.results.iter().enumerate() {
            match drop.get(&(j, i)) {
                None => kept.push(r.clone()),
                Some(None) => {
                    excluded.push(i);
                    kept.push(Vec::new());
                }
                Some(Some(idx)) => {
                    excluded.push(i);
                    kept.push(r.iter().enumerate().filter(|(k, _)| !idx.contains(k)).map(|(_, y)| *y).collect());
                }
            }
        }
        let retained: Vec<Option<Cell>> = kept.iter().map(|r| form(r)).collect();
        let present: Vec<Cell> = retained.iter().flatten().copied().collect();
        let what = format!("levels[{}]", l.name);
        let estimates = match spec.method {
            MethodSpec::Anova => interlab::level_estimates(&present),
            MethodSpec::Reml => {
                let slices: Vec<&[f64]> = kept.iter().map(Vec::as_slice).collect();
                interlab::reml_estimates(&slices)
            }
        }
        .map_err(|e| err(&what, e.to_string()))?;
        let initial = scrutiny(&cells);
        let final_scrutiny = (!excluded.is_empty()).then(|| scrutiny(&retained));
        let n = final_scrutiny.as_ref().unwrap_or(&initial).n;
        let (a_r, a_rr) = if n >= 2 && estimates.p >= 2 {
            let ar = interlab::uncertainty_factor_repeatability(estimates.p, n);
            let arr = (estimates.s_r > 0.0)
                .then(|| interlab::uncertainty_factor_reproducibility(estimates.p, n, estimates.s_rr / estimates.s_r));
            (Some(ar), arr)
        } else {
            (None, None)
        };
        levels.push(LevelResult {
            name: l.name.clone(),
            cells,
            retained,
            excluded,
            initial,
            final_scrutiny,
            estimates,
            a_r,
            a_rr,
        });
    }
    let mut relationships = Vec::new();
    if levels.len() >= 3 {
        let m: Vec<f64> = levels.iter().map(|l| l.estimates.mean).collect();
        for of_reproducibility in [false, true] {
            let s: Vec<f64> =
                levels.iter().map(|l| if of_reproducibility { l.estimates.s_rr } else { l.estimates.s_r }).collect();
            for &r in &spec.relationships {
                relationships.push(RelationshipFit {
                    of_reproducibility,
                    relationship: r,
                    fit: interlab::fit_relationship(r.kind(), &m, &s).map_err(|e| e.to_string()),
                });
            }
        }
    }
    Ok(InterlabAnalysis {
        quantity: spec.quantity.clone(),
        unit: spec.unit.clone(),
        laboratories: spec.laboratories.clone(),
        method: spec.method,
        levels,
        relationships,
    })
}

/// Compares the analysis with the expected values.
pub fn check(e: &ExpectedInterlab, a: &InterlabAnalysis, out: &mut Vec<crate::CheckOutcome>) {
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
    for (name, x) in &e.levels {
        let l = a.levels.iter().find(|l| &l.name == name);
        let est = l.map(|l| &l.estimates);
        let w = |s: &str| format!("precision_experiment.levels[{name}].{s}");
        if let Some(p) = x.p {
            push(w("p"), est.map(|e| e.p as f64), &Some(Check { value: p as f64, tolerance: 0.0 }));
        }
        push(w("m"), est.map(|e| e.mean), &x.m);
        push(w("s_r"), est.map(|e| e.s_r), &x.s_r);
        push(w("s_L"), est.map(|e| e.s_l), &x.s_l);
        push(w("s_R"), est.map(|e| e.s_rr), &x.s_rr);
        let g = l.and_then(|l| l.initial.grubbs.as_ref());
        push(w("cochran"), l.and_then(|l| l.initial.cochran.first()).map(|t| t.statistic), &x.cochran);
        push(w("grubbs_low"), g.map(|g| g.low.statistic), &x.grubbs_low);
        push(w("grubbs_high"), g.map(|g| g.high.statistic), &x.grubbs_high);
        push(w("grubbs_double_low"), g.and_then(|g| g.double_low.as_ref()).map(|d| d.statistic), &x.grubbs_double_low);
        push(
            w("grubbs_double_high"),
            g.and_then(|g| g.double_high.as_ref()).map(|d| d.statistic),
            &x.grubbs_double_high,
        );
        push(
            w("final_cochran"),
            l.and_then(|l| l.retained_scrutiny().cochran.first()).map(|t| t.statistic),
            &x.final_cochran,
        );
    }
    for r in &e.relationships {
        let rep = r.of == "s_R";
        let fit = a
            .relationships
            .iter()
            .find(|f| f.of_reproducibility == rep && f.relationship == r.relationship)
            .and_then(|f| f.fit.as_ref().ok());
        for (j, c) in r.params.iter().enumerate() {
            push(
                format!("precision_experiment.relationships[{} {}].{j}", r.of, r.relationship.code()),
                fit.and_then(|f| f.parameters().get(j).copied()),
                &Some(*c),
            );
        }
    }
}

fn outcome_code(o: Outcome) -> &'static str {
    match o {
        Outcome::Correct => "correct",
        Outcome::Straggler => "straggler",
        Outcome::Outlier => "outlier",
    }
}

fn scrutiny_json(s: &Scrutiny, labs: &[String]) -> serde_json::Value {
    use serde_json::json;
    let single = |t: &interlab::GrubbsSingle| {
        json!({"laboratory": labs[t.index], "p": t.p, "statistic": t.statistic, "critical_5": t.critical_5,
            "critical_1": t.critical_1, "outcome": outcome_code(t.outcome)})
    };
    let double = |t: &interlab::GrubbsDouble| {
        json!({"laboratories": [labs[t.indices[0]], labs[t.indices[1]]], "statistic": t.statistic,
            "critical_5": t.critical_5, "critical_1": t.critical_1, "outcome": outcome_code(t.outcome)})
    };
    json!({
        "laboratories": s.labs.iter().map(|&i| &labs[i]).collect::<Vec<_>>(),
        "n": s.n,
        "h": s.h,
        "k": s.k,
        "h_indicators": s.h_indicators.map(|(a, b)| json!({"5%": a, "1%": b})),
        "k_indicators": s.k_indicators.map(|(a, b)| json!({"5%": a, "1%": b})),
        "cochran": s.cochran.iter().map(|t| json!({"laboratory": labs[t.cell], "statistic": t.statistic, "p": t.p,
            "n": t.n, "critical_5": t.critical_5, "critical_1": t.critical_1, "outcome": outcome_code(t.outcome)}))
            .collect::<Vec<_>>(),
        "grubbs": s.grubbs.as_ref().map(|g| json!({
            "low": single(&g.low),
            "high": single(&g.high),
            "repeated": g.repeated.as_ref().map(single),
            "double_low": g.double_low.as_ref().map(double),
            "double_high": g.double_high.as_ref().map(double),
        })),
    })
}

pub fn results_json(a: &InterlabAnalysis) -> serde_json::Value {
    use serde_json::json;
    let cell = |c: &Option<Cell>| c.map(|c| json!({"n": c.n, "mean": c.mean, "sd": c.sd}));
    json!({
        "method": match a.method { MethodSpec::Anova => "anova", MethodSpec::Reml => "reml" },
        "laboratories": a.laboratories,
        "levels": a.levels.iter().map(|l| json!({
            "name": l.name,
            "cells": l.cells.iter().map(cell).collect::<Vec<_>>(),
            "retained": l.retained.iter().map(cell).collect::<Vec<_>>(),
            "excluded": l.excluded.iter().map(|&i| &a.laboratories[i]).collect::<Vec<_>>(),
            "scrutiny": scrutiny_json(&l.initial, &a.laboratories),
            "final_scrutiny": l.final_scrutiny.as_ref().map(|s| scrutiny_json(s, &a.laboratories)),
            "p": l.estimates.p,
            "m": l.estimates.mean,
            "s_r": l.estimates.s_r,
            "s_L": l.estimates.s_l,
            "s_R": l.estimates.s_rr,
            "u_m": l.estimates.u_mean,
            "A_r": l.a_r,
            "A_R": l.a_rr,
        })).collect::<Vec<_>>(),
        "relationships": a.relationships.iter().map(|r| match &r.fit {
            Ok(f) => json!({"of": if r.of_reproducibility { "s_R" } else { "s_r" }, "relationship": r.relationship.code(),
                "params": f.parameters(),
                "fitted": a.levels.iter().map(|l| f.sd_at(l.estimates.mean)).collect::<Vec<_>>()}),
            Err(e) => json!({"of": if r.of_reproducibility { "s_R" } else { "s_r" }, "relationship": r.relationship.code(),
                "error": e}),
        }).collect::<Vec<_>>(),
    })
}
