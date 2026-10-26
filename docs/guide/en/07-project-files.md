# 7. Project file reference

A project is a JSON file. Unknown fields are errors, so a misspelt name never goes unnoticed. Text fields
(titles, descriptions, explanations, notes, reference texts) are written either as a plain string (English) or
as `{ "en": "…", "tr": "…" }`.

## 7.1 Top level

| Field | Required | Content |
|---|---|---|
| `format` | yes | `"uncertainty-calculations/project"` |
| `version` | yes | `1` |
| `metadata` | yes | title and documentation (7.2) |
| `model` | one of the five | measurement model (7.3) |
| `topdown` | one of the five | top-down evaluation (7.6) |
| `proficiency` | one of the five | proficiency test round, ISO 13528 (7.7) |
| `calibration` | one of the five | calibration with uncertainties in both variables, ISO 6143 (7.8) |
| `precision_experiment` | one of the five | interlaboratory precision experiment, ISO 5725-2 (7.10) |
| `conformity` | with `model`, or alone | conformity assessment, JCGM 106 (7.9) |
| `analysis` | no | GUM and Monte Carlo options (7.4); only for model projects |
| `reporting` | no | rounding of the reported result (7.5) |
| `expected` | no | reference values with tolerances (7.12) |

## 7.2 `metadata`

`title` (required), `id` (in the library: the file name), `description`, `explanation` (list of steps),
`field` (`chemistry`, `mass`, `length`, …), `sources` (list of `{ "document", "section", "url" }`), `tags`,
`notes` (list).

## 7.3 `model`

```json
"model": {
  "output": { "name": "c", "unit": "mg/L", "expression": "1000 * m * P / V", "description": "…" },
  "intermediates": [ { "name": "V", "expression": "V_nom + dV_cal", "unit": "mL" } ],
  "inputs": [ { "name": "m", "unit": "mg", "description": "…", "distribution": { … }, "dof": 50 } ],
  "correlations": [ { "a": "X1", "b": "X2", "r": 0.9 } ]
}
```

`dof` of an input is its degrees of freedom for the GUM framework (default ∞; Type A distributions set it
themselves).

### Distributions (`"type"`)

| `type` | Fields |
|---|---|
| `constant` | `value` |
| `normal` | `mean` and either `sd`, or `expanded` and `k` |
| `t` | `mean`, `dof` and either `scale`, or `expanded` and `k` |
| `rectangular`, `triangular`, `arcsine` | `lower` and `upper`, or `value` and `half_width` |
| `trapezoidal` | limits as above and `beta` (ratio of the half-widths of top and base) |
| `curvilinear_trapezoid` | limits as above and `d` (uncertainty of the limits) |
| `exponential` | `mean` |
| `gamma` | `shape`, `scale` |
| `type_a` | `observations` (list) |
| `type_a_pooled` | `mean`, `pooled_sd`, `pooled_dof`, `n` |

## 7.4 `analysis`

```json
"analysis": {
  "gum": { "coverage_probability": 0.95, "coverage_factor": null, "higher_order": false, "dof_rounding": "truncate",
           "kragten": false },
  "mcm": { "trials": 1000000, "adaptive": { "ndig": 2, "tolerance_divisor": 1, "max_trials": 10000000 },
           "coverage_probability": null, "seed": 1, "interval": "shortest", "validation_ndig": 2 }
}
```

Omitted fields take the defaults shown. `coverage_factor` overrides `coverage_probability`; `adaptive`
overrides `trials`; the Monte Carlo coverage probability defaults to that of the GUM result. Set `"gum": null`
or `"mcm": null` to skip one of the methods.

## 7.5 `reporting`

`significant_digits` (default 2), `decimal_places` (instead of significant digits), `rounding`
(`half_even`, `half_up`, `up`), `uncorrected_bias` (model projects: `bias`, `u_bias`, `strategy` `ue` | `rssu` |
`rssu_k` | `sumu` | `sumu_max`; section 6.3).

## 7.6 `topdown`

```json
"topdown": {
  "quantity": "c(NH4-N)", "unit": "µg/L",
  "levels": [10, 100],
  "precision": [ … ], "bias": [ … ], "recovery": { … }, "components": [ … ],
  "coverage_factor": 2
}
```

Instead of `coverage_factor` (default 2): `coverage_probability` with `coverage_dof` (k from the t-distribution).
Every precision, bias, recovery and further component may have a `"range": { "min": …, "max": … }`
(min inclusive, max exclusive) and a `description`. Component names must be unique.

### Precision entries

`{ "name": …, "data": { "type": … }, "range", "mean_of", "to_relative_at", "to_absolute_at" }` with `data`:

| `type` | Fields |
|---|---|
| `series` | `values`, `relative` |
| `groups` | `groups`: list of `{ "values" }` or `{ "sd", "n" }` / `{ "sd_percent", "n" }`; `relative` |
| `duplicates` | `pairs` (list of `[x1, x2]`), `relative`, `method` (`pooled`, `normalised_differences`, `mean_range`), `min_mean`, `max_mean` |
| `control_limits` | `limit` or `limit_percent`, `k` (default 2) |
| `given` | `sd` or `sd_percent`, `dof` |
| `linear` | `points` (list of `[c, s]`) or `intercept` and `slope`; `dilution_factor` |
| `continuous` | `sd0`, `rsd1_percent` |
| `reproducibility` | `sd_reproducibility` or `sd_between_labs`, `sd_repeatability`, `lab_repeatability` (all optionally `_percent`), `replicates`; `method_bias`: `{ "labs", "replicates", "u_reference" }`; `bias_check`: `{ "bias", "sd_within", "n" }` |
| `staggered_nested` | `results` (rows `[y₁, y₂, y₃]`), `estimate` (`repeatability`, `intermediate`, `reproducibility`), `relative` |

`mean_of`: `{ "replicates": n, "days": p, "repeatability" | "repeatability_percent" }`.
`to_relative_at`: a number or `"mean"`; `to_absolute_at`: a number.

### Bias entries

`{ "name": "u(bias)", "kind": "crm" | "proficiency_tests" | "recovery", "relative": true, "references": [ … ],
"u_reference" | "u_reference_percent" | "u_reference_components_percent" }` — the last three give a reference
uncertainty shared by all references. Each reference:

| Field | Meaning |
|---|---|
| `label` | name in the report |
| `reference` | certified or assigned value C |
| `u_reference`, `u_reference_percent`, or `expanded` with `k` | uncertainty of C |
| `sd_reproducibility` or `sd_reproducibility_percent` with `labs` | proficiency test: u(C) = s_R/√n_Lab |
| `measured`, or `mean` with `sd`/`sd_percent` and `n` | the laboratory's results |
| `bias`, `bias_percent`, `recovery_percent` | a bias stated directly |

### Recovery

`{ "name", "tests": [ … ], "include_sd_of_recoveries": false, "dof", "confidence": 0.95, "correction":
"if_significant" | "always" | "never" }` with tests:

| `type` | Fields |
|---|---|
| `reference` | `measured` or `mean`, `sd`, `n`; `reference`; `u_reference` or `expanded` with `k` |
| `spiked` | `after` or `mean_after`, `sd_after`, `n_after`; the same for `before`; `spike`; `u_spike` or `u_spike_percent` |
| `recoveries` | `values_percent`, or `mean_percent`, `sd_percent`, `n` |

The default degrees of freedom of the significance test are Σ(nᵢ − 1) over the tests, or N − 1 if that sum is
zero or s(R) is included.

### Further components

`{ "name", "u" | "u_percent" | "half_width" | "half_width_percent", "distribution": "rectangular" |
"triangular", "multiplicity": 1, "range" }`.

## 7.7 `proficiency`

```json
"proficiency": {
  "quantity": "atrazine", "unit": "mg/L",
  "results": [ { "lab": "L01", "value": 0.262, "expanded": 0.03, "k": 2, "method": "GC-MS" },
               { "lab": "L17", "less_than": 0.015 } ],
  "assigned_value": { "method": "algorithm_a" },
  "sd_pt": { "method": "robust" }
}
```

| Field | Content |
|---|---|
| `results` | one entry per participant: `lab` (unique), `value` or `less_than`/`greater_than` (censored, not scored), `u` or `expanded` with `k` (default 2), `method` |
| `assigned_value` | `method`: `algorithm_a` (x*, u = `u_factor`·s*/√p, `u_factor` 1.25), `median` (u = 1.25·`scale`/√p, `scale` `niqr` or `made`), `mean` (u = s/√p), `reference` (`value` with `u`, or `expanded` and `k`); optional `u_hom`, `u_trans`, `u_stab` added in quadrature (ISO 13528 Formula 3) |
| `sd_pt` | `method`: `robust` (s*), `made`, `niqr`, `sd`, or `given` with `value` or `percent` (of x_pt) |
| `delta_e`, `delta_e_percent` | allowance for measurement error δ_E for PA scores (default 3σ_pt) |
| `coverage_factor` | k of U(x_pt) for E_n (default 2; a stated expanded uncertainty of a reference value is used as given) |
| `uncertainty_screening` | `u_min`, `u_max`: numbers, or `"u_assigned"`, `"1.5_robust_sd"`, `"1.5_sd_pt"` (ISO 13528 9.8) |
| `homogeneity` | `items` (one list of replicates per item, equal numbers), `sd_pt` or `sd_pt_percent_of_mean` (default: the σ_pt of the round) |
| `stability` | `values`; `reference` (default: the homogeneity average); `u_reference` and `u_values` for Formula (B.18) |
| `algorithm_a` | `"third_significant_figure"` (stopping rule of C.3.1, default) or `"converged"` |

`results` may be empty for a homogeneity and stability study before the round.

## 7.8 `calibration`

```json
"calibration": {
  "analyte": "NO", "unit": "µmol/mol", "response_unit": "absorbance",
  "points": [ { "name": "M1", "x": 4.5, "u_x": 0.045, "y": 0.1969, "u_y": 0.003938 }, … ],
  "covariances": [ { "between": ["M4", "M7"], "value": 0.00016 } ],
  "function": "linear",
  "samples": [ { "name": "S1", "y": 0.6, "u_y": 0.012 } ]
}
```

| Field | Content |
|---|---|
| `points` | reference mixtures: `name` (unique), content `x` with standard uncertainty `u_x`, response `y` with `u_y` (> 0) |
| `covariances` | optional covariances u(xᵢ, xₕ) of the contents of two named points |
| `function` | `linear`, `quadratic`, `cubic`, `power` or `exponential` (chapter 10) |
| `compare` | fit and list the other functions too (default `true`) |
| `samples` | mixtures to analyse: `name`, `y`, `u_y` |
| `coverage_factor` | k of the expanded uncertainties of the samples (default 2) |
| `target_uncertainty` | acceptable standard uncertainty, compared with the upper bound of 6.2.3 |
| `drift` | `reference` (a point name), mean responses `before` and `after`, `n` and `n_calib` (default 10) |
| `validation` | mismatching reference mixtures: `name`, `y`, `u_y`, `x_ref`, `u_x_ref` (Formula 13) |

## 7.9 `conformity`

Next to `model` (the result is assessed) or as the only section of a project (JCGM 106; chapter 11).

| Field | Content |
|---|---|
| `lower`, `upper` | tolerance limits (at least one) |
| `rule` | `simple_acceptance` (default), `guarded_acceptance`, `guarded_rejection` |
| `r` / `guard_band` / `probability` | guard band w = r·2u (default r = 1), w directly, or acceptance limits where the probability at the limit is given |
| `acceptance_lower`, `acceptance_upper` | acceptance limits given directly (the rule follows from them) |
| `distribution` | model projects: `normal` (default) or `t` (effective degrees of freedom) |
| `quantity`, `unit`, `u` or `u_relative`, `dof`, `results` | projects of their own: property, measuring system, measured values (`name`, `value`, optional `u`) |
| `process`, `target_consumer_risk` | global risks: `distribution` `normal` or `gamma`, `mean`, `sd`; required R_C (finds r) |

## 7.10 `precision_experiment`

The basic method of ISO 5725-2 (chapter 12).

```json
"precision_experiment": {
  "quantity": "Sulfur content of coal", "unit": "%",
  "laboratories": ["1", "2", "3"],
  "levels": [ { "name": "1", "results": [[0.71, 0.71, 0.70], [0.69, 0.67, 0.68], [0.66, 0.65, 0.69]] } ],
  "exclude": [ { "laboratory": "3", "level": "1", "reason": "wrong sample" } ]
}
```

| Field | Content |
|---|---|
| `laboratories` | names (unique), in the order of the results of every level |
| `levels` | `name` (unique) and `results`: one list per laboratory, `[]` for an empty cell (Form A) |
| `exclude` | results left out after scrutiny: `laboratory`, optional `level` (all levels if absent), optional `result` (1-based; the whole cell if absent), `reason` |
| `method` | `anova` (Formulae 23–31, default) or `reml` (Annex B.2) |
| `form_decimals` | decimals of the cell means and standard deviations recorded in Forms B and C; used for the tests and the variances, as in the examples of the standard (full precision if absent) |
| `relationships` | relationships fitted when there are three or more levels: any of `"I"`, `"II"`, `"III"`, `"IV"` (default all four; `[]` for none) |

## 7.11 Errors

The program refuses, with a message naming the field: unknown fields; a project with more or fewer than one
of `model`, `topdown`, `proficiency`, `calibration` and `precision_experiment`; inconsistent alternatives (e.g. `sd` together with `expanded`); absolute top-down
components without `levels`; a level where no component applies; correlated non-Gaussian inputs in the Monte
Carlo method; non-finite data.

## 7.12 `expected`

Reference values for self-checking projects; every value is `{ "value": …, "tolerance": … }` and is checked by
`uncertainty check`. `reference` is a text naming where the values come from.

* `gum`: `estimate`, `standard_uncertainty`, `expanded_uncertainty`, `coverage_factor`, `effective_dof`,
  `kragten_uncertainty`
* `mcm`: `estimate`, `standard_uncertainty`, `interval_low`, `interval_high`, `gum_validated` (true/false)
* `topdown`:
  * `components`: `{ "<name>": check }` — standard uncertainty of a component (per cent if relative)
  * `bias`: `{ "<name>": { "mean", "rms", "mean_u_reference" } }`
  * `recovery`: `mean_percent`, `standard_uncertainty_percent`, `sd_percent`, `statistic`, `critical`,
    `chi_squared`, `significant`
  * `levels`: list of `{ "level", "value", "standard_uncertainty", "expanded_uncertainty",
    "relative_standard_percent", "relative_expanded_percent" }`
* `proficiency`: `assigned_value`, `u_assigned`, `sd_pt`, `median`, `made`, `niqr`, `robust_mean`, `robust_sd`,
  `iterations`, `mean`, `sd`; `scores`: `{ "<lab>": { "d", "d_percent", "pa", "z", "z_prime", "zeta", "en" } }`;
  `homogeneity`: `mean`, `s_x`, `s_w`, `s_s`, `criterion`, `sufficient`; `stability`: `difference`, `sufficient`
* `calibration`: `params`, `u_params` and `cov_params` (lists; covariances in the order (0,1), (0,2), …, (1,2), …),
  `s_res`, `gamma`, `admissible` (true/false), `samples`: `{ "<name>": { "x", "u" } }`, `drift_passed`
* `conformity`: `acceptance_lower`, `acceptance_upper`, `capability_index`, `r`, `conforming`, `accepted`,
  `consumer_risk`, `producer_risk`; `results`: `{ "<name>": { "conformance_probability", "accepted" } }`
* `precision_experiment`: `levels`: `{ "<level>": { "p", "m", "s_r", "s_L", "s_R", "cochran", "grubbs_low",
  "grubbs_high", "grubbs_double_low", "grubbs_double_high", "final_cochran" } }` (`p` is an integer; the test
  statistics refer to all results, `final_cochran` to the retained ones); `relationships`: list of
  `{ "of": "s_r" | "s_R", "relationship": "I" … "IV", "params": [checks] }`
