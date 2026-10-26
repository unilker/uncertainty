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
| `model` | one of the two | measurement model (7.3) |
| `topdown` | one of the two | top-down evaluation (7.6) |
| `analysis` | no | GUM and Monte Carlo options (7.4); ignored for top-down projects |
| `reporting` | no | rounding of the reported result (7.5) |
| `expected` | no | reference values with tolerances (7.8) |

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
  "gum": { "coverage_probability": 0.95, "coverage_factor": null, "higher_order": false, "dof_rounding": "truncate" },
  "mcm": { "trials": 1000000, "adaptive": { "ndig": 2, "tolerance_divisor": 1, "max_trials": 10000000 },
           "coverage_probability": null, "seed": 1, "interval": "shortest", "validation_ndig": 2 }
}
```

Omitted fields take the defaults shown. `coverage_factor` overrides `coverage_probability`; `adaptive`
overrides `trials`; the Monte Carlo coverage probability defaults to that of the GUM result. Set `"gum": null`
or `"mcm": null` to skip one of the methods.

## 7.5 `reporting`

`significant_digits` (default 2), `decimal_places` (instead of significant digits), `rounding`
(`half_even`, `half_up`, `up`).

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

## 7.7 Errors

The program refuses, with a message naming the field: unknown fields; a project with both or neither of
`model` and `topdown`; inconsistent alternatives (e.g. `sd` together with `expanded`); absolute top-down
components without `levels`; a level where no component applies; correlated non-Gaussian inputs in the Monte
Carlo method; non-finite data.

## 7.8 `expected`

Reference values for self-checking projects; every value is `{ "value": …, "tolerance": … }` and is checked by
`uncertainty check`. `reference` is a text naming where the values come from.

* `gum`: `estimate`, `standard_uncertainty`, `expanded_uncertainty`, `coverage_factor`, `effective_dof`
* `mcm`: `estimate`, `standard_uncertainty`, `interval_low`, `interval_high`, `gum_validated` (true/false)
* `topdown`:
  * `components`: `{ "<name>": check }` — standard uncertainty of a component (per cent if relative)
  * `bias`: `{ "<name>": { "mean", "rms", "mean_u_reference" } }`
  * `recovery`: `mean_percent`, `standard_uncertainty_percent`, `sd_percent`, `statistic`, `critical`,
    `chi_squared`, `significant`
  * `levels`: list of `{ "level", "value", "standard_uncertainty", "expanded_uncertainty",
    "relative_standard_percent", "relative_expanded_percent" }`
