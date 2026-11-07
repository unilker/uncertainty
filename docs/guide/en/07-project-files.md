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
| `model` | one of the nine | measurement model (7.3) |
| `topdown` | one of the nine | top-down evaluation (7.6) |
| `proficiency` | one of the nine | proficiency test round, ISO 13528 (7.7) |
| `calibration` | one of the nine | calibration with uncertainties in both variables, ISO 6143 (7.8) |
| `precision_experiment` | one of the nine | interlaboratory precision experiment, ISO 5725-2 (7.10) |
| `multivariate` | one of the nine | model with several output quantities, JCGM 102 (7.11) |
| `control_chart` | one of the nine | control chart for internal quality control, Nordtest TR 569 (7.12) |
| `sampling` | one of the nine | uncertainty arising from sampling, Eurachem UfS:2019 (7.13) |
| `error_function` | one of the nine | error approximation function of a weighing instrument, EURAMET cg-18 (7.14) |
| `conformity` | with `model`, or alone | conformity assessment, JCGM 106 (7.9) |
| `analysis` | no | GUM and Monte Carlo options (7.4); for model and multivariate projects |
| `reporting` | no | rounding of the reported result (7.5) |
| `expected` | no | reference values with tolerances (7.16) |

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

`ensembles` (optional) lists groups of inputs evaluated from the same data, e.g. `[["AE", "AISE"]]`. The
members of a group must have the same finite `dof`. Each group is one component in the Welch–Satterthwaite
formula (§3.6).

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
           "kragten": false, "dominant_rectangular": false },
  "mcm": { "trials": 1000000, "adaptive": { "ndig": 2, "tolerance_divisor": 1, "max_trials": 10000000 },
           "coverage_probability": null, "seed": 1, "interval": "shortest", "validation_ndig": 2 }
}
```

Omitted fields take the defaults shown. `coverage_factor` overrides `coverage_probability`; `"dominant_rectangular": true` takes k from the convolution of the largest rectangular contribution with a normal distribution (UKAS M3003 Appendix C, section 3.7) and cannot be combined with `coverage_factor`; `adaptive`
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

`{ "name", "tests": [ … ], "mean": "arithmetic" | "weighted", "include_sd_of_recoveries": false, "dof",
"confidence": 0.95, "correction": "if_significant" | "always" | "never" }` with tests:

| `type` | Fields |
|---|---|
| `reference` | `measured` or `mean`, `sd`, `n`; `reference`; `u_reference` or `expanded` with `k` |
| `spiked` | `after` or `mean_after`, `sd_after`, `n_after`; the same for `before`; `spike`; `u_spike` or `u_spike_percent` |
| `recoveries` | `values_percent`, or `mean_percent`, `sd_percent`, `n` |
| `given` | `recovery_percent`, `u_percent` (a recovery and its standard uncertainty evaluated elsewhere) |

`"mean": "weighted"` combines the tests by the inverse-variance weighted mean of MUPR Eq. (16.1) with
u(R̿) = (Σ1/u²)^−½; the default is the arithmetic mean of Eq. (8)–(12).

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
| `homogeneity` | `items` (one list of replicates per item, equal numbers), `sd_pt` or `sd_pt_percent_of_mean` (default: the σ_pt of the round; without σ_pt the criteria are not evaluated), `bayes` (`tau_scale`, `sigma_scale`, optional `mu_mean`, `mu_sd`) |
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
| `process`, `target_consumer_risk` | global risks: `distribution` `normal` or `gamma` with `mean`, `sd`, or `lognormal` with `meanlog`, `sdlog`; required R_C (finds r) |
| `posterior` | `true`: p_c of the results from the posterior with the process PDF as prior (projects of their own) |

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

## 7.11 `multivariate`

A model with several output quantities (JCGM 102; chapter 13). The options of `analysis` apply: `gum.coverage_probability`
and `mcm` (`trials`, `adaptive`, `seed`, `validation_ndig`, `coverage_probability`).

```json
"multivariate": {
  "outputs": [ { "name": "R", "unit": "Ω", "expression": "V/I*cos(phi)" }, { "name": "X", "unit": "Ω", "expression": "V/I*sin(phi)" } ],
  "indications": [ { "inputs": ["V", "I", "phi"], "units": ["V", "A", "rad"], "values": [[5.007, 0.019663, 1.0456], …], "covariance": "mean" } ]
}
```

| Field | Content |
|---|---|
| `outputs` | `name`, `unit`, `expression`: f_j(X), or the equation h_j(Y, X) = 0 when `implicit`; `start`: starting value of an implicit model |
| `implicit` | `true` for a model h(Y, X) = 0 solved by Newton's method (default `false`) |
| `inputs`, `correlations`, `intermediates` | as in `model` (7.3) |
| `indications` | input quantities from series of vector indication values: `inputs` (names), optional `units`, `values` (one row per set, n > N), `covariance` `mean` (M/(n(n − 1)), default) or `t` (M/((ν − 2)n)); the Monte Carlo method draws them from the multivariate t-distribution |

## 7.12 `control_chart`

A control chart for internal quality control (Nordtest TR 569; chapter 14).

```json
"control_chart": {
  "quantity": "Zn, control solution 60.0 µg/L", "unit": "µg/L",
  "values": [64.5, 66.3, 61.1, 59.7, 57.4, …], "labels": ["1 Feb", "8 Feb", …],
  "reference": 60.0,
  "limits": { "basis": "statistical" },
  "establish": 25, "review": {}
}
```

| Field | Content |
|---|---|
| `chart` | `x` (default), `range` or `relative_range` |
| `values` | control values in time order: single values or means (X), ranges (R), relative ranges in % (r%) |
| `runs` | instead of `values`: the replicate results of each run; the control value is their mean, range or relative range |
| `labels` | dates or names of the runs, one per control value |
| `replicates` | number of replicates per range; needed for range charts given as `values` |
| `central_line` | `{ "mean": … }` (established earlier) or `{ "reference": … }` (reference or nominal value); the mean of the establishing values if absent |
| `reference` | reference value for the t-test of the mean (Equation 6) when the central line is the mean |
| `limits` | `basis`: `statistical` (default), `target` or `tolerance`. Statistical: optional `sd` or `sd_percent` established earlier, `sd_from` `mean_range` (default) or `pooled` for range charts. Target: one of `sd`, `sd_percent` (of the central line), `expanded_percent` (s = U/4), `repeatability_limit` (s = r/2.8, range charts). Tolerance: `tolerance` or `tolerance_percent` (action limits only). `action_only` (`true`: no warning limits) with an optional `action_factor` |
| `establish` | the first N control values set the central line and the statistical limits (default all) |
| `exclude` | run numbers (1-based) left out of the statistics |
| `review` | long-term review of the values after the establishing period: optional `previous` and `current` summaries (`mean`, `sd`, `n`, `dof`), `exclude_beyond` (default 4 s), `confidence` (default 0.95) |

## 7.13 `sampling`

Uncertainty arising from sampling (Eurachem UfS:2019; chapter 15): the duplicate method or the modelling
approach with Gy's sampling theory.

```json
"sampling": {
  "quantity": "Lead in top soil, one target", "unit": "mg/kg",
  "duplicates": [[787, 769, 811, 780], [338, 327, 651, 563], …], "labels": ["A4", "B7", …],
  "method": "anova", "log": true
}
```

| Field | Content |
|---|---|
| `duplicates` | balanced design: for each sampling target the four results S1A1, S1A2, S2A1, S2A2 |
| `targets` | instead of `duplicates`, any nested design: for each target the samples, for each sample its analyses, e.g. `[[[x, x], [x]], …]` (unbalanced design, Appendix D2) |
| `labels` | names of the targets |
| `method` | `anova` (default: classical nested analysis of variance) or `range` (range method, balanced design only) |
| `log` | `true`: also the ANOVA of ln x and the uncertainty factors F_U = exp(k·s_G) |
| `gy` | instead of duplicates, the fundamental sampling error: `lot_concentration` a_L and `particle_concentration` α (same unit), `critical_density` ρ_c and `matrix_density` ρ_m (g cm⁻³), `steps` (each with `name`, `sample_mass` and `lot_mass` in g, `particle_size` d in cm, `size_factor` g, optional `shape_factor` f = 0.5 and `liberation` β = 1), optional `analytical_rsd` (fraction) |
| `coverage_factor` | k for the relative expanded uncertainties (default 2) |

## 7.14 `error_function`

The error approximation function of a weighing instrument (EURAMET cg-18 Appendix C2.2; chapter 16).

```json
"error_function": {
  "instrument": "Balance 400 g / 0.1 mg", "unit": "g", "error_unit": "mg", "error_factor": 1000,
  "points": [{ "indication": 50.000067, "reference": 50.000006, "u_reference": 0.0142, "u_indication": 0.0512 }, …],
  "through_zero": true, "s_m": 0.05, "readings": [{ "reading": 120 }]
}
```

| Field | Content |
|---|---|
| `points` | test points: `indication`, either `error` (in `error_unit`) or `reference` (the reference mass, in `unit`), and `u_reference`, `u_indication` (in `error_unit`) |
| `error_factor` | E = (I − m_ref)·factor for points given with `reference` (default 1) |
| `degree` | degree nₐ of the polynomial (default 1) |
| `through_zero` | `true`: no constant term, E = a₁I (+ a₂I² …) (C2.2-16) |
| `reference_correlation` | `full` (default, U(m_ref) = s sᵀ, C2.2-3b) or `none` |
| `s_m` | model term on the diagonal of U(e) (C2.2-3c), default 0 |
| `coverage_factor` | k of U(E_appr) and of the residual test C2.2-2b (default 2) |
| `readings` | readings R (`reading`, optional `u_reading`) at which E_appr(R) and its uncertainty are reported |
| `in_use` | uncertainty of a weighing result in use and minimum weight (cg-18 7.4, 7.5, Appendix G; 16.5); needs `degree` 1 and `through_zero` |

```json
"in_use": {
  "intervals": [{ "max": 12000, "d": 2, "repeatability": [9998, 10000, 9998, 10000, 10000] },
                { "max": 30000, "d": 5, "s": 2.739 }],
  "eccentricity": { "deviation": 5, "load": 20000 }, "temperature": { "coefficient": 2e-6, "delta_t": 3 },
  "buoyancy": { "delta_t": 3 }, "tare": true, "requirement": 0.01, "safety_factor": 2, "readings": [12000, 30000]
}
```

| Field of `in_use` | Content (masses in `unit`) |
|---|---|
| `intervals` | weighing intervals or ranges in ascending order: `max`, scale interval `d`, and `s` or the `repeatability` readings |
| `d0` | scale interval at zero (default: `d` of the first interval) |
| `eccentricity` | `deviation` ΔI_ecc,max and `load` L_ecc: u_rel = ΔI_ecc,max/(L_ecc√3) (7.4.4-10) |
| `temperature` | `coefficient` K_T (1/K) and `delta_t` ΔT (K): K_T·ΔT/√12 (7.4.3-1) |
| `buoyancy` | `delta_t` (7.4.3-4), `conservative: true` (7.4.3-5) or `relative`; `rho_0` (1.2) and `rho_c` (8000 kg/m³) |
| `adjustment` | `delta_e_max` ΔE(Max) between calibrations: ΔE/(Max√3) (7.4.3-6) |
| `tare` | `true`: tare balancing in use, (q_E,max − q_E,min)/√12 from the test points (7.4.4-5) |
| `time` | creep and hysteresis: `e0` (7.4.4-7) or `mpe_max` with `multiplier` (7.4.4-9a, -9b) |
| `relative` | further relative standard uncertainties `{ "name", "u" }` |
| `coverage_factor` | k of U(W) and U_gl(W) (default 2, 7.5.1) |
| `requirement`, `safety_factor` | relative weighing tolerance Req (0.01 = 1 %) and SF (default 1) for the minimum weight (G-7, G-9) |
| `readings` | readings at which U(W) and U_gl(W) are reported |

## 7.15 Errors

The program refuses, with a message naming the field: unknown fields; a project with more or fewer than one
of `model`, `topdown`, `proficiency`, `calibration`, `precision_experiment`, `multivariate`, `control_chart`, `sampling` and `error_function`; inconsistent alternatives (e.g. `sd` together with `expanded`); absolute top-down
components without `levels`; a level where no component applies; correlated non-Gaussian inputs in the Monte
Carlo method; non-finite data.

## 7.16 `expected`

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
  `homogeneity`: `mean`, `s_x`, `s_w`, `s_s`, `criterion`, `sufficient`, `ms_between`, `ms_within`, `u_bb_star`,
  `tau_mean`, `tau_sd`, `tau_median`, `tau_mode`, `tau_q975`, `tau_hpd_upper`, `sigma_mean`, `sigma_hpd_lower`,
  `sigma_hpd_upper`, `mu_mean`; `stability`: `difference`, `sufficient`
* `calibration`: `params`, `u_params` and `cov_params` (lists; covariances in the order (0,1), (0,2), …, (1,2), …),
  `s_res`, `gamma`, `admissible` (true/false), `samples`: `{ "<name>": { "x", "u" } }`, `drift_passed`
* `conformity`: `acceptance_lower`, `acceptance_upper`, `capability_index`, `r`, `conforming`, `accepted`,
  `consumer_risk`, `producer_risk`; `results`: `{ "<name>": { "conformance_probability", "accepted" } }`
* `precision_experiment`: `levels`: `{ "<level>": { "p", "m", "s_r", "s_L", "s_R", "cochran", "grubbs_low",
  "grubbs_high", "grubbs_double_low", "grubbs_double_high", "final_cochran" } }` (`p` is an integer; the test
  statistics refer to all results, `final_cochran` to the retained ones); `relationships`: list of
  `{ "of": "s_r" | "s_R", "relationship": "I" … "IV", "params": [checks] }`
* `multivariate`: `gum` and `mcm`, each with `estimate` and `standard_uncertainty` (`{ "<output>": check }`),
  `correlations` (list of `{ "between": [a, b], "value", "tolerance" }`), `k_ellipsoid`, `k_rectangle`,
  `smallest_region_area` (Monte Carlo, two outputs); `ellipsoid_validated`, `rectangle_validated` (true/false)
* `control_chart`: `mean`, `sd` (the s of the limits), `center`, `lower_warning`, `upper_warning`, `lower_action`,
  `upper_action`; the counts `outside_warning`, `out_of_control`, `statistical_signals` (integers); `review`: `mean`,
  `sd` (current series), `f`, `f_critical`, `pooled_sd`, `t`, `t_critical`, `sd_significant`, `mean_significant`
  (true/false); `precision`: `s_r_pooled` (Equation 10), `s_rw_simplified`, `s_r`, `s_between`, `s_rw` (analysis
  of variance)
* `sampling`: `mean`, `s_between_target`, `s_sampling`, `s_analysis`, `s_measurement`, `s_total`;
  `expanded_relative_sampling`, `expanded_relative_analysis`, `expanded_relative_measurement`,
  `expanded_relative_between_target` (U′ in %); `percent_between_target`, `percent_sampling`, `percent_analysis`
  (of the total variance); `sampling_variance_negative` (true/false); `log`: `mean`, `s_between_target`,
  `s_sampling`, `s_analysis`, `s_measurement`, `s_total`, `factor_sampling`, `factor_analysis`,
  `factor_measurement`, `relative_measurement`; `gy`: `constitution_factor`, `sampling_constants` and `rsd` (lists,
  one per step), `rsd_sampling`, `rsd_total`, `expanded_relative_sampling`, `expanded_relative_total`
* `error_function`: `coefficients` and `u_coefficients` (lists, in the order of the powers), `chi_squared`,
  `chi_squared_passed` (true/false), `residual_failures` (number of points failing C2.2-2b), `u_approximated` (list,
  one per point), `readings` (list of `{ "error", "u" }`); `in_use`: `alpha_squared`, `expanded_at_lower`, `slope`,
  `global_at_lower`, `global_slope` (lists, one per interval), `beta_squared`, `minimum_weight` (with the safety
  factor)
