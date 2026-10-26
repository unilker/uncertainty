# ISO/IEC 17025:2017 — requirements served by the program

Mapping of the clauses of ISO/IEC 17025:2017 (read in TS EN ISO/IEC 17025:2017, licensed copy, used locally
only) that concern measurement uncertainty, validity of results, reporting and software. The program helps a
laboratory meet them; it does not by itself make a laboratory compliant. Clause content is paraphrased.

| Clause | Requirement (paraphrased) | Program | Gap / plan |
|---|---|---|---|
| 7.2.2 | Validate non-standard and modified methods; record performance characteristics (precision, trueness, range, …) | top-down projects from raw validation data: precision (control charts, replicates, duplicates, precision models), bias (CRMs, PT, recovery), ranges; the report records the data and the evaluation | — |
| 7.6.1 | Identify the contributions to uncertainty, including sampling, and account for all significant ones by appropriate analysis | GUM budget with variance shares, Kragten view, zero-sensitivity warning, Monte Carlo check of the GUM result | sampling uncertainty (Eurachem/CITAC UfS guide, duplicate method, ISO 3085-type designs) not yet implemented |
| 7.6.2 | Calibration laboratories evaluate the uncertainty of all calibrations | model projects (EA-4/02 S2–S13 and GUM H examples in the library); ISO 6143 calibration with uncertainties in both variables | multivariate outputs (JCGM 102) planned |
| 7.6.3 | Testing laboratories evaluate the uncertainty, by estimation where a rigorous evaluation is impossible; Note 3 names ISO/IEC Guide 98-3, ISO 21748 and the ISO 5725 series | bottom-up (GUM, MCM) and top-down (Nordtest, MUPR, ISO 21748, ISO 5725-3) evaluations | ISO 5725-2 outlier tests and s_r/s_R from collaborative data (awaiting ISO 5725-2) |
| 7.7.1 | Monitor the validity of results (reference materials, control charts, replicate tests, intralaboratory comparisons …) with statistical techniques | control-chart and replicate data are evaluated for R_w and bias | control-chart construction and rules (ISO 7870-2, Nordtest TR 569) not implemented |
| 7.7.2 | Monitor performance by comparison with other laboratories (proficiency testing, interlaboratory comparisons) | proficiency projects (ISO 13528): z, z′, ζ, E_n with signals; PT data as bias evidence in top-down projects | — |
| 7.8.3.1 c) | Test reports give the uncertainty, in the unit of the measurand or relative, when relevant to validity, required by the customer or affecting conformity | reported statement with U, k and coverage probability; relative uncertainty in top-down reports; rounding rules (GUM 7.2.6, MUPR §12) | — |
| 7.8.4.1 a) | Calibration certificates give the uncertainty of the result in the unit of the measurand or relative | as above; EA-4/02 statement wording | — |
| 7.8.6 | Conformity statements: document and apply a decision rule with its risk; state the rule and which results it covers (Note: ISO/IEC Guide 98-4) | `conformity` (JCGM 106 = ISO/IEC Guide 98-4): tolerance, decision rule (simple acceptance, guarded acceptance/rejection), guard band, p_c and specific risk per result, global risks; the report states the rule (D38) | — |
| 7.11.2 | Validate data-processing systems, including software, before use and after changes | every reference example is a self-checking project (`uncertainty check library`, 107 projects); the test suite; reports carry the program version and citation; validation matrix and research log document each comparison | a printable validation report (summary of `check` with versions and tolerances) for the laboratory's records — planned |
| 7.11.3 d) | Ensure integrity of data and information | project files are plain JSON with schema validation (`deny_unknown_fields`); results are recomputed, never stored | — |
| 7.11.6 | Check calculations and data transfers systematically | independent cross-checks recorded in the validation matrix (published examples, NIST Uncertainty Machine, ODRPACK, Monte Carlo) | — |

## Results not corrected for bias

7.6 and 7.8 do not address uncorrected bias directly. The GUM assumes correction (3.2.4). The
`reporting.uncorrected_bias` option (D34) gives the laboratory a documented way to report an uncorrected
result. It implements the strategies reviewed by Magnusson & Ellison (2008) and adds a statement that names
the strategy.
