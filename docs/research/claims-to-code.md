# Functional statements of the article and where the program implements them

Third pre-submission review, item S2.4 (research log D113). Every statement in Sections 2, 3 and Appendix A of
`docs/article/manuscript.md` that says what the program does is listed with the code that does it and the test or
library project that checks it. Two earlier statements had no counterpart in the code and were removed (D112): the
"published form kept as an option" of the gamma approximation, and the use of the general function
`q_gamma_approximation` by the bootstrap (it uses `q_gamma_approximation_nicob`). Functions that are public but not
used by any evaluation have unit tests as well (e.g. `gamma_approximation_uses_the_exact_variance_of_q`).

| Section | Statement | Code | Checked by |
|---|---|---|---|
| 2.1, Table 1 | eleven evaluation types, one per project section | `project/src/lib.rs` `run`, `Evaluation` | `project/tests/library.rs` (every library project) |
| 2.1 | where a document leaves a choice open, the choice is an option of the project and named in the report | `project/src/schema.rs` (options with defaults); `project/src/report*.rs` | library projects with each option (e.g. `rounding`, `coverage`) |
| 2.2 | symbolic derivatives; sensitivity coefficients printed | `core/src/expr.rs` (differentiation, simplification); `project/src/lib.rs` `sensitivities` | `core/tests/gum_examples.rs` |
| 2.2 | combined uncertainty with correlations | `core/src/gum.rs` | `correlation_terms`; GUM H.2 and JCGM 101 9.4 projects |
| 2.2, A.1 | second-order term of the estimate (Amd. 1:2026) always computed, added on request; variance with the terms of Mana and Pennecchi | `core/src/gum.rs` (`higher_order`, estimate term) | `second_order_term_of_the_estimate_amd1_2026`, `zero_sensitivity_warning_only_when_second_order_matters` |
| 2.2, A.1 | higher-order part with infinite degrees of freedom | `core/src/gum.rs` (Welch–Satterthwaite) | `core/tests/gum_examples.rs` |
| 2.2, A.2 | ensembles enter Welch–Satterthwaite as one component | `core/src/gum.rs`, `core/src/model.rs` (`ensemble`) | `ensemble_of_paired_means_keeps_its_dof`; GUM-5 Example 2 projects |
| 2.2 | rounding of U by EA-4/02 (default), MUPR, half up or always up; the statement names the rule | `project/src/schema.rs` `RoundingSpec`; `core/src/report.rs` `Rounding` | `decimal_rather_than_binary_rounding`, `rounding_up_as_in_nordtest` |
| 2.2 | CMC as a lower limit of the reported uncertainty | `project/src/schema.rs` `CmcSpec`; `project/src/lib.rs` | library projects with `cmc` |
| 2.3 | adaptive procedure of JCGM 101 7.9; the report gives the numerical tolerance; with fixed trials, the number and the seed | `core/src/mcm.rs`; `project/src/report.rs` (`numerical tolerance`) | `adaptive_procedure_stabilizes`, `mass_calibration_adaptive` |
| 2.3 | symmetric and shortest intervals; half-width of the interval symmetric about the estimate | `core/src/mcm.rs` `coverage_intervals`, `about_estimate` | `coverage_interval_indices`; JCGM 101 9.2–9.5 projects |
| 2.3 | validation of the GUM result (JCGM 101 clause 8); note on the different input variances for repeated observations | `core/src/mcm.rs` (validation); `project/src/report.rs` (note) | JCGM 101 projects with `gum_validated` |
| 2.4 | conformance probability, global risks (JCGM 106) | `core/src/conformity.rs` | JCGM 106 and GUM-5 Example 10 projects; Suncal comparison |
| 2.4 | decision rule and guard band stated in the report | `project/src/report/conformity.rs` | conformity projects (report text) |
| 2.4 | guard bands from the TUR, Dobbert's managed guard band; U₉₅ = k₉₅u by default, 2u as an option; factor limited to 1 unless allowed | `core/src/conformity.rs` `TurMethod`, `project/src/conformity.rs` | `tur_guard_band_factors_match_suncal`, `dobbert_factor_above_one_is_limited_unless_allowed`; `tools/research/conformity/tur_risks.py` |
| 2.4 | ISO 13528 robust statistics, scores, homogeneity checks | `core/src/pt.rs`, `project/src/proficiency.rs` | ISO 13528 E.1–E.5 projects |
| 2.5 | DerSimonian–Laird with Knapp–Hartung and the NICOB parametric bootstrap | `core/src/consensus.rs` `dersimonian_laird`, `dl_bootstrap` | NICOB, Koepke, Merkatas projects |
| 2.5, A.3 | gamma approximation of Q evaluated at the untruncated τ²_M, exact moments | `core/src/consensus.rs` `q_gamma_approximation_nicob` | `gamma_approximation_uses_the_exact_variance_of_q`, `gamma_approximation_has_the_chi_squared_limit` |
| 2.5, A.4 | hierarchical Bayes (half-Cauchy prior on τ) on deterministic grids | `core/src/consensus.rs` (Bayes grid) | `bayes_grids_have_converged`; Koepke 2017 projects |
| 2.5 | degrees of equivalence (MRA, leave-one-out with the modified Knapp–Hartung adjustment), unilateral and bilateral, for DL, Linear Pool and Bayes | `core/src/consensus.rs`, `project/src/consensus.rs` | `leave_one_out_and_bilateral_degrees_of_equivalence`; NICOB projects against the NumPy port |
| 2.5 | outlying laboratories identified by the leave-one-out degrees of equivalence; no result excluded automatically | `project/src/consensus.rs` (all results enter) | NICOB ⁶⁰Co project |
| 2.5 | bounded outputs: truncated GUM interval y +U_U/−U_L; Monte Carlo distribution restricted to the bound with mean, s.d. and interval | `project/src/lib.rs` (`lower_bound`, `upper_bound`), `core/src/mcm.rs` | `bounded_output_truncates_the_gum_interval_and_restricts_the_mcm`; BIPM ISRD-06 project |
| 3 | no network code | dependencies: serde, serde_json, pdf-writer, ttf-parser, miniz_oxide (`*/Cargo.toml`) | review of `Cargo.toml` |
| 3 | the schema rejects unknown fields; results are not stored in the file | `project/src/schema.rs` (`deny_unknown_fields`) | parse tests in `project/src/lib.rs` |
| 3 | enhanced Wichmann–Hill generator, Box–Muller, Marsaglia–Tsang | `core/src/rng.rs` | `constants_agree_with_wichmann_hill_2006`, `normal_moments`, `gamma_moments` |
| 3 | several outputs evaluated from one joint draw | `core/src/multivariate.rs` | JCGM 102 projects |
| 3 | every report states version, commit and date, compiler and SHA-256 of the project | `project/build.rs`, `project/src/lib.rs` (`SOFTWARE_*`), `project/src/sha256.rs` | report tests; S5 |
| 3, 4.4 | spreadsheet workbook: model at xᵢ ± 0.001 u(xᵢ), Algorithm A by iteration, ANOVA of ISO 5725-2, TR 537 components | `project/src/workbook/` | `model_workbook_has_the_checks`, `workbook_is_a_zip_with_the_parts`; CI job `spreadsheets` |
| 7 | `kapsam check --lab`: the laboratory's projects in a section of their own; digest of every file and of the set | `cli/src/main.rs`, `project/src/report/validation.rs` | S5; `laboratory/` |
| 4.2 | `--seed` and `--trials` apply to every Monte Carlo evaluation, the consensus bootstrap and multicomponent risks included | `project/src/lib.rs` `run` (D113) | `the_seed_on_the_command_line_reaches_the_consensus_bootstrap` |
