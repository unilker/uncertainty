# Research log (laboratory notebook)

Chronological record of decisions, sources consulted, findings and open
questions. Entries are never rewritten after the fact; corrections are added
as new entries. This log is the primary source for the methods and validation
sections of the planned article (see `publication-plan.md`).

---

## 2026-10-25 — Project definition and first engine

### Decisions

**D1. Distribution model: free, open software with strong attribution.**
The author decided against a paid product. Goal: a free program of a quality
suitable for publication in a Q1 journal, with the author's rights protected
and citation required.
*Implementation:* GPL-3.0-or-later (copyleft: derivatives must remain open
under the same licence and cannot be closed and sold as proprietary
software) plus additional terms under GPL §7(b), (c), (e)
(`ADDITIONAL-TERMS.md`): attribution must be preserved in the source, the
About screen and every generated report; modified versions must be marked;
no trademark rights. Citation metadata in `CITATION.cff`.
*Rationale:* software journals (e.g. SoftwareX, JOSS) require an
OSI-approved licence, so a custom "must cite" licence would make the
software ineligible. GPL §7(b) is the strongest attribution requirement
compatible with an OSI licence. A licence cannot legally force citation in
papers that only *use results*; this is handled by the citation request and
by embedding the citation in every report the program produces.
*Replaces:* the Apache-2.0 licence of the initial commit (permissive; would
allow closed commercial forks).

**D2. Architecture: Rust computation core + Flutter user interface.**
One numerical core (`core/`, Rust, no external numerical dependencies) shared
by desktop (Windows, macOS, Linux) and mobile (Android, iOS) front-ends in
Flutter. Rationale: deterministic, fast Monte Carlo (10⁶–10⁷ trials on a
phone), every algorithm implemented and tested in-house (traceability of
the numerical method), reusable for a CLI and the validation suite.

**D3. Priority of application domains.** 1) analytical chemistry (bottom-up
EURACHEM/CITAC CG4 and top-down Nordtest/ISO 11352/Eurachem MUPR), 2)
calibration (EA-4/02), 3) accredited laboratories (ISO/IEC 17025 reporting,
conformity assessment JCGM 106).

**D4. Primary sources are read, not recalled.** All algorithms and reference
values are taken from the original documents (downloaded 2026-10-25):
- JCGM 100:2008 — https://www.bipm.org/documents/20126/2071204/JCGM_100_2008_E.pdf
- JCGM 101:2008 — https://www.bipm.org/documents/20126/2071204/JCGM_101_2008_E.pdf
- EURACHEM/CITAC CG4 QUAM:2012.P1 — https://www.eurachem.org/images/stories/Guides/pdf/QUAM2012_P1.pdf

**D5. Copyright of guidance documents.** JCGM documents state that economic
and moral copyright is protected and that third parties may not rewrite,
re-brand or republish them (JCGM 101 front matter). QUAM:2012 is "Copyright
© 2012". Therefore: the program and its guide **cite** clause numbers and
reproduce only numerical data needed for validation; explanatory text and
example narratives are written independently. Permission requests to
JCGM/Eurachem should be considered before any example is reproduced in full.

### Implemented (core v0.1.0)

| Module | Content | Source |
|---|---|---|
| `special` | ln Γ, incomplete Γ/B, erf/erfc, Φ, Φ⁻¹, Student t pdf/cdf/quantile, coverage factor | standard numerical methods; checked against SciPy |
| `rng` | enhanced Wichmann–Hill (64-bit form), Box–Muller, t generator, gamma (Marsaglia–Tsang) | JCGM 101 C.3.3, C.4, C.6 |
| `distribution` | normal, t (scaled/shifted), rectangular, triangular, trapezoidal, curvilinear trapezoid, arc sine, exponential, gamma, constant; Type A summaries | JCGM 101 6.4; GUM 4.2–4.3 |
| `expr` | model parser, exact symbolic derivatives (any order), compiled evaluation | GUM 5.1.3 |
| `model` | inputs, intermediate quantities (sub-models), correlations | GUM 5.2; GUM-6 |
| `gum` | LPU 1st order + correlations, GUM 5.1.2-note higher-order terms, Welch–Satterthwaite, k from t | GUM 5, G.4, G.6 |
| `mcm` | fixed-M and adaptive MCM, symmetric & shortest intervals, multivariate Gaussian (Cholesky), numerical tolerance, GUF validation | JCGM 101 7, 7.9, 8 |

### Findings

**F1. GUM H.1 uses unrounded uncertainties.** Table H.1 prints
u(δθ) = 0.029 °C, but the contribution 16.6 nm in H.6b requires
u(δθ) = 0.05/√3 = 0.028 87 °C (0.029 gives 16.7 nm). Likewise H.1.7 uses
u(δα) = 10⁻⁶/√3 and u(θ) = √(0.2² + 0.5²/2). Modelling the inputs with the
distributions described in the GUM text (rectangular, arc sine) reproduces
all published values. ν_eff(d) = 25.6 in H.1.6 follows from the rounded
u(d) = 9.7 nm; unrounded it is 25.4. U₉₉ = 93 nm is 2.92 × the rounded
u_c = 32 nm (2.92 × 31.67 = 92.5 nm).

**F2. JCGM 101 MCM table values carry Monte Carlo dispersion.** The tabulated
MCM results (M ≈ 10⁶, Mersenne Twister) differ from exact values by amounts
consistent with sampling noise, e.g. comparison loss x₁ = 0.050 (Table 8):
shortest interval [1590, 3543]×10⁻⁶ vs analytic [1593.6, 3548.6]×10⁻⁶; mass
calibration (Table 6): shortest interval midpoint 1.2330 mg although the
output PDF is exactly symmetric about 1.2340 mg. Independent references were
therefore computed analytically (noncentral χ²) or by 2 × 10⁸-trial PCG64
Monte Carlo (`validation/reference_values.py`). Our engine agrees with these
references within its own measured dispersion.

**F3. Shortest coverage intervals are poorly located by MCM for
near-symmetric outputs.** Dispersion study (`core/examples/mcm_dispersion.rs`,
20 seeds, M = 2 × 10⁶): for comparison loss x₁ = 0.050 the standard deviation
of the shortest-interval endpoints is 3.2–5.0 × 10⁻⁶, versus 0.8–1.0 × 10⁻⁶
for the probabilistically symmetric endpoints and 1.0 × 10⁻⁶ for the
shortest-interval *length*; mass calibration: 7.4 × 10⁻⁴ vs 1.0 × 10⁻⁴ mg.
The length function is flat near its minimum, so its location is noisy
while its length is stable. Consequences: (a) validation of the GUF against
MCM (JCGM 101 8.1) should use the symmetric interval when the PDF is
symmetric; (b) reported shortest intervals need the adaptive procedure with
endpoint stabilization (7.9.4), which the engine implements; (c) this is a
publishable methodological observation — **check the literature for prior
reports before claiming novelty** (open question Q3).

**F4. EURACHEM A1 (Cd calibration standard): GUF slightly conservative.**
With the volume sub-model (triangular + normal + rectangular components),
MCM gives a 95 % interval ≈ 0.015 mg/L narrower at each end than the GUF
interval (lighter-than-Gaussian tails). GUF is validated at one significant
digit of u (δ = 0.05 mg/L) but not at two (δ = 0.005 mg/L). Typical of
chemistry budgets dominated by rectangular/triangular Type B components;
worth a systematic study across the example library (open question Q4).

### Open questions

- **Q1.** Welch–Satterthwaite with higher-order terms: the engine treats the
  higher-order variance as having infinite degrees of freedom. GUM H.1.6
  computes ν_eff from first-order terms only. Document; consider offering both.
- **Q2.** MCM with correlated non-Gaussian inputs is refused (JCGM 101 6.4.8
  covers only the multivariate Gaussian). Options: Gaussian copula with
  documented caveats, or keep refusing.
- **Q3.** Literature search on the dispersion of MCM shortest coverage
  intervals (F3).
- **Q4.** Systematic GUF-vs-MCM study over the chemistry example library (F4).
- **Q5.** Author metadata (full name, ORCID, affiliation) for `CITATION.cff`. *(closed 2026-10-25, D6)*

### Next steps

1. Top-down module: Nordtest TR 537 ed. 4 (2017), ISO 11352, ISO 21748 and the
   new Eurachem/CITAC guide *Evaluation of measurement uncertainty from
   in-house precision and recovery data* (MUPR, 1st ed., March 2026) —
   obtain and read the primary documents first.
2. Remaining EURACHEM CG4 examples A2–A7 as validation cases.
3. Calibration curves (ISO/TS 28037), multivariate output (JCGM 102).
4. Report generation (budget table, contribution chart, citation footer).
5. Flutter UI and the bilingual guide.

---

## 2026-10-25 (session 2) — Author metadata, top-down module, EURACHEM A2

### Decisions

**D6. Author and name.** Author: İlker Ün, TÜBİTAK Ulusal Metroloji Enstitüsü
(UME), ORCID 0000-0001-5089-4602. Program name: *Uncertainty Calculations*;
Turkish name: *Belirsizlik Hesaplamaları*. Entered in `CITATION.cff`, `NOTICE`
and `README.md` (closes Q5).

**D7. Sources for the top-down module.** Primary documents obtained and read
(2026-10-25):
- Nordtest TR 537 ed. 4 (2017) —
  https://nordtest.info/images/documents/nt-technical-reports/NT_TR_537_edition4_English_Handbook_for_calculation_of_measurement_uncertainty_in_environmental_laboratories.pdf
- Eurachem/CITAC MUPR, 1st ed. (2026), ISBN 978-972-9348-30-3 —
  https://eurachem.org/images/stories/Guides/pdf/MUPR_2026_EN_v01.pdf
ISO 11352, ISO 21748 and ISO 5725 are paywalled and have **not** been read;
the module implements them only as far as Nordtest and MUPR reproduce them.

**D8. Recomputation from raw data.** Wherever a guide gives raw data
(Nordtest Appendices 4–7), the tests recompute the statistics from the raw
data instead of using the printed summaries. Raw data were extracted
programmatically from the PDF text and every derived per-pair value was
checked against the printed column.

**D9. Rounding for reports is a selectable rule.** MUPR §12 rounds a
discarded 5 to even and ignores figures beyond the first discarded one;
GUM 7.2.6 allows rounding uncertainties up; Nordtest rounds U up to whole
per cent. All three are implemented (`report::Rounding`), operating on the
decimal representation (so 0.285 is treated as 0.285, not 0.28499…).

### Implemented

| Module | Content | Source |
|---|---|---|
| `special` | χ² quantile (critical values for Eq. 16) | checked against SciPy |
| `topdown` | pooled s (Eq. 1–2), s from duplicates and mean ranges, u(Rw) from control limits, s of means (Eq. 5–7), precision models (App. A), u(bias) from CRM/PT/recovery, recovery from RMs and spikes (Eq. 8–12), significance (Eq. 13), equivalence and χ² homogeneity (Eq. 15–16), uncorrected-recovery allowance (Eq. 17–18), acceptance limits and compatibility (B1), combination with contributions (Eq. 19–24) | Nordtest TR 537; MUPR 2026 |
| `report` | significant-figure and decimal-place rounding, three rules, value/uncertainty formatting | GUM 7.2.6; MUPR §12 |

### Findings

**F5. Nordtest TR 537 §6.1 (one CRM): arithmetic slip.** u(C_ref) is
printed as 0.25/11.5 × 100 = 2.16 %; the quotient is 2.17 %. The printed
u(bias) = 4.1 % follows from 2.16 %; with 2.17 % it is 4.15 % (4.2 % to two
figures).

**F6. Nordtest §6.2 (PT).** The mean of the six listed u(C_ref,i) is first
printed as 2.7 % and then used as 2.6 %; the mean of the listed values is
2.60 %. The final u(bias) = 5.3 % is consistent with 2.6 %.

**F7. Nordtest Appendix 5.** 46 duplicate pairs are listed below 30 µg/L
while §5.3 states n = 47; the pooled s = 0.44 µg/L is nonetheless reproduced
(0.441). Above 30 µg/L the 26 pairs reproduce 3.82 % (printed 3.8 %).
Oxygen (Appendix 6, 51 pairs): 0.0252 mg/L reproduced; BOD (Appendix 7):
214.75 mg/L, s = 5.58 mg/L, 2.60 % reproduced.

**F8. Nordtest §8.** BOD Example B: RMS_bias = 3.76 % is obtained from the
biases rounded to one decimal; from the raw PT values it is 3.77 %. PCB
Example C uses u(C_ref) = 7/152 = 4.6 % (u(bias) 7.22 %), whereas the summary
table prints 4.7 % and 7.29 % — internally consistent but not with the
example.

**F9. MUPR Example B1: intermediate rounding.** The guide rounds
R̄ = 0.389/0.400 = 0.9725 to 0.972 before using it, so the test statistic of
Eq. (B1.3) is printed as 0.982 (0.963 unrounded) and u(R) for one test as
0.0848 (0.0849). Conclusions are unaffected. The 99 % duplicate-range factor
is printed as 3.69, whereas z₀.₉₉₅·√2 = 3.64 — **to verify** against the
source the guide relies on (e.g. ISO 5725-6 critical range factors).

**F10. EURACHEM CG4 Example A2.** The guide rounds the balance linearity
0.15/√3 = 0.087 mg to 0.09 mg, giving u(m) = 0.13 mg instead of 0.12 mg, and
u(V_T) = 0.013 mL instead of 0.0137 mL; u_c becomes 0.000099 instead of
0.000101 mol/L, with the same reported result (0.1021 ± 0.0002) mol/L.
Table A2.4 (effect of the distribution chosen for the burette calibration)
could not be reproduced under any of three readings tried (u(V) from
calibration and temperature; the tabulated u(V) column; with volume
repeatability added). Its u(V) column appears shifted by one row relative to
the text (the triangular value 0.013 mL appears in the *Normal* row). **To
check against the printed edition / errata before citing.**

**F11. Guidance documents round at intermediate steps.** F1, F5–F10 show a
recurring pattern: published worked examples round intermediate values, so
exact recomputation differs in the last printed digit, and occasionally
(F5, F10) arithmetic or tabulation slips occur. For software validation this
means (a) tolerances must be justified case by case and (b) a validation
suite that recomputes from raw data adds value beyond reproducing printed
numbers. This is a candidate contribution for the article (method section
"validation against guidance documents") and should be reported neutrally,
with the corrected values, and ideally communicated to the document owners.

### Open questions (new)

- **Q6.** Should results be corrected for recovery by default when Eq. (13)
  indicates a significant deviation? MUPR leaves this to the documented
  procedure; the UI must ask, not decide.
- **Q7.** Top-down budgets currently combine by root sum of squares only.
  Linking them to the general GUM/MCM model (precision and recovery as
  multiplicative factors f ~ N(1, u′)) would allow MCM validation of top-down
  budgets — design decision pending.
- **Q8.** Permission to reproduce raw data tables from Nordtest/Eurachem in
  the program's example library (copyright of both documents is held by the
  authors).

### Next steps

1. EURACHEM A3–A7 (A4–A6 contain in-house validation / empirical-method data
   relevant to the top-down workflow).
2. MUPR examples B2–B6 (precision modelling over the range, heterogeneity,
   matrix-dependent recovery).
3. Project file format (JSON), CLI and report generation.
4. Flutter UI.

---

## 2026-10-25 (session 3) — EURACHEM A3–A7, MUPR B2–B6, EA-4/02 S2–S13

### Decisions

**D10. Division of labour on sources.** Paywalled or unavailable documents
are listed in `documents-needed.md`; the author obtains them (UME library /
TSE). Freely available documents are downloaded directly: JCGM 102, 104,
106, 200, GUM-6, EA-4/02 M:2022, NIST TN 1900 and the Suncal paper
(Delker, SAND2021-8022J) were obtained on 2026-10-25; of these EA-4/02 has
been read in full so far.

**D11. Analytic sensitivities, not Kragten differences.** The engine uses
exact derivatives. Where a guide uses Kragten's spreadsheet (one-sided
finite differences), tests check both the guide's numbers (with a stated
tolerance) and the exact value. Offering a Kragten view for comparability
with QUAM is a possible UI feature (Q9).

### Implemented

| Module | Content | Source |
|---|---|---|
| `calibration` | OLS/WLS straight line, parameter covariance, inverse prediction and its uncertainty | QUAM 2012 App. E.4 |
| `topdown` | normalised-difference precision, binomial inhomogeneity model, s(R) of recoveries, pairwise recovery statistic, allowance for accepted recovery intervals, target uncertainty from class width | QUAM A4; MUPR §8.2.7, B5, B6, App. C, B4.4 |
| `special` | coverage factor for dominant trapezoidal/rectangular distributions | EA-4/02 S9.14, S10.13 |

### Findings

**F12. EURACHEM A3.** The summary Table A3.1 states u_c = 0.00016 mol/L,
but its own inputs give 0.00019; 0.00016 is the triple-determination result
(Table A3.4); the detailed single-run value is 0.00018. u(V_HCl) contains an
unexplained 0.0037 mL term; with unrounded components it is 0.0115 mL
(0.011 printed). Typos: M = 204.2236 in the 25 °C formula, "0.12 mL" for
0.012 mL and "2 × 0.87²" for 0.087² in the replicated section; Table A3.4
uses u(m) = 0.00013 g while the text gives 0.12 mg.

**F13. EURACHEM A4.** Table A4.2 has errors in derived columns (mean 0.75
for 0.075, difference −0.10 for −0.01, mean 0.10 for 0.01); recomputed from
D1/D2 the statistic 0.382 is reproduced. t_crit is quoted as 2.021 (t(40))
for 41 degrees of freedom (2.0195). "0.373/1.1111" is printed for
0.377/1.1111. With 27 % relative uncertainties, Kragten's one-sided
difference for Rec (−0.0507) differs from the derivative (−0.0531).

**F14. EURACHEM A5.** The calibration line (b₁, b₀, their s, r, S, S_xx)
and u(c₀) = 0.018 mg/L are reproduced from the raw absorbances. The area
uncertainty components 0.042 and 0.146 dm² combine to 0.152 dm², but
0.19 dm² is printed and used (u_c 0.00147 vs 0.00144 mg/dm²; reported
result unchanged). The area formula shows d = 2.77 dm for the measured
2.70 dm, and Table A5.4 cells C2/C3 show 10.01 and 0.27 for a_V.
*Update 2026-11-08 (review item 6):* all of F14 is corrected in QUAM_ERRATA_v2 (Eurachem, 21 January 2019, pp.
72–80, amended Table A5.1): u(a_V) 0.15 dm², d = 2.70 dm, Table A5.4 column E. It is a known erratum, class e,
not a finding of this work. Likewise the first part of F10 (u(m_KHP) 0.12 mg, errata p. 46) and the extra term in
u(V_HCl) of F12 (errata p. 55, "the rounded result of the calculation is unaffected"); their remaining parts
(Table A2.4; Table A3.1 mixing single-run inputs with the triplicate u_c) are not in the errata.

**F15. EURACHEM A6.** Table A6.5 prints U = 0.12 % m/m at 10 % fibre; it is
1.2 % m/m (the 12 % column is right).

**F16. EURACHEM A7 (double IDMS).** Fully reproduced from Tables A7.3–A7.8:
c_z, M(Pb), c_x = 0.053738 µmol/g, u_c = 0.00018 (0.00041 without the √8
division), ΣK_bias = 37.6 % of the variance. No discrepancy found.

**F17. MUPR B2–B6, Appendix C.** All examples reproduced from the raw tables
(B2.1, B2.3, B6.1, B6.2, all 22 rows of B6.3). Minor points: values computed
from rounded intermediates (B2 s′_I(2), B5 statistic 1.554, B6 R̿); B3
regression intercept 0.02324 printed 0.0233; B6 χ² = 8.80 is computed
about the unweighted mean, whereas Eq. (16) prescribes the weighted mean
(8.70; same conclusion); Eq. (B6.8) omits the 1/R̿ factor on the absolute
precision term of the recovery-corrected result; Example C1 uses √3 ≈ 1.73
(u_a 8.1 % instead of 8.05 %).

**F18. EA-4/02 M:2022.** All twelve examples reproduced; Table E.1 agrees
with t₉₅.₄₅(ν) for all 26 entries. Printed inconsistencies: S2 total
combines rounded contributions (29.2 vs 29.26 mg); S5 δt contribution
24.5 µV vs 0.641 × 38.46 = 24.65 µV (the printed total uses 24.65) and the
δt estimate is −0.5 °C, not +0.5 °C; S6 heading says 19 GHz, text and result
18 GHz; S7 δL_ib contribution printed 0.0019 dB for u = 0.0003 dB, and the
printed total 0.0223 dB includes it (0.0222 dB without); S11 k = 1.81 vs
1.80 from Eq. (S10.9); S13 mean Δl 49.999 54 vs 49.999 55 mm in the budget,
s = 0.33 µm vs "0.18 µm" in the text, √(0.15² + 0.25²) = 0.29 vs 0.30 µm,
δl_P sign, and U = 0.82 µm reported as 0.9 µm (round-up). None of these
changes a reported result materially.

**F19. Second-order GUM terms for products.** EA-4/02 S4 and S13 need the
variance of products whose factors have zero expectation; the GUM 5.1.2-note
terms implemented in `gum` give these variances exactly (they are exact for
products of independent quantities). First-order LPU silently gives zero for
such terms — the UI should warn when a sensitivity coefficient is zero but
the input has non-zero uncertainty (Q10).

**F20. Overall picture (candidate result for the article).** About 45
worked examples from seven guidance documents (GUM, JCGM 101, EURACHEM
QUAM, Nordtest TR 537, Eurachem MUPR, EA-4/02) have now been recomputed,
from raw data wherever given. Every reported *result* was reproduced, but
roughly half of the examples contain at least one printed inconsistency
(intermediate rounding, transcription slips, or a value inconsistent with
the stated inputs). Only two affect a printed final value beyond rounding
(Nordtest §6.1: 4.1 → 4.2 %; EURACHEM A5: u(a_V) 0.19 → 0.15 dm²; see also F21, EA-4/02 S13). This
supports the argument that validation of uncertainty software should
recompute examples from their inputs, with documented tolerances, rather
than match printed numbers.

### Open questions (new)

- **Q9.** *(closed 2026-10-26, D24)* Offer a Kragten (one-sided difference) view next to the exact
  sensitivities, for comparability with QUAM-style spreadsheets?
- **Q10.** *(closed 2026-10-26, D23)* Warn when a first-order sensitivity vanishes at the estimate but
  the model is non-linear in that input (S4, S13, JCGM 101 9.4).
- **Q11.** Report discrepancies F5, F10, F12–F15, F18 to the document owners
  (Nordtest, Eurachem, EA)? Decision for the author. *(closed 2026-10-25, D12)*

### Next steps

1. Project file format (JSON) and CLI, so every example can be stored and
   reported reproducibly.
2. Report generation (budget table, contributions, citation footer).
3. NIST TN 1900 examples (downloaded) and remaining JCGM 102 multivariate
   cases.
4. Flutter UI.

---

## 2026-10-25 (session 3, addendum) — Reporting of discrepancies

**D12. Discrepancies are reported in the article, not to the document
owners beforehand (author's decision, closes Q11).** The article documents
the recomputation of the guidance examples and the corrected values; the
owners can revise their guides after publication. Conditions adopted:

1. Classify every finding as (a) intermediate rounding (legitimate
   practice), (b) typographical/transcription slip without effect on the
   result, or (c) inconsistency affecting a printed result (currently
   Nordtest §6.1 and EURACHEM A5). Only (c) is called an error.
2. Before submission, check the current editions and any errata
   (documents-needed.md §D); drop or annotate findings already corrected.
3. Report each finding with document edition, page/table, printed value,
   corrected value and the test that reproduces it; neutral wording.
4. Frame the contribution as a validation methodology (recompute from
   inputs with documented tolerances), not as criticism of the guides.
5. A courtesy notice to the owners before submission remains optional;
   default is none.

---

## 2026-10-25 (session 4) — Project files, reports, CLI, example library

**D13. Project file format.** Projects are JSON documents
(`"format": "uncertainty-calculations/project"`, `"version": 1`) read with
`deny_unknown_fields`, so a misspelt key is an error rather than a silently
ignored setting. A file holds bilingual metadata (title, description,
step-by-step explanation, notes, sources), the model, analysis options,
reporting rule and — optionally — reference values with tolerances. The
format lives in a separate crate (`project`, serde) so that the
computation core keeps no dependencies. Distributions accept the forms in
which information is found in certificates and guides (limits or
value ± half-width; u or U with k; raw observations → Type A, JCGM 101
6.4.9).

**D14. Reports.** Markdown and self-contained HTML (inline SVG, no external
resources, light/dark) in English or Turkish; Turkish uses the decimal
comma and the prefixed percent sign (%95), following TDK usage. Every
report states the methods, sources, the MCM seed and number of trials
(reproducibility), the JCGM 101 clause 8 validation, and the citation
required by ADDITIONAL-TERMS.md. The MCM coverage probability defaults to
that of the GUM result (k = 2 → 95.45 %), so the validation compares like
with like.

**D15. Reference values in the example library.** A library file states a
published value only where the document reproduces it from its own inputs
(within its printed rounding); otherwise the value is recomputed (analytic
solution or independent high-precision MC, `validation/`) and the
discrepancy is recorded in `notes`. An engine's own Monte Carlo output is
never used as its own reference. MC tolerances ≈ 5σ of the measured
dispersion (unchanged from D9). `cargo test` runs the whole library
(`project/tests/library.rs`); 38 projects, all passing.

**F21. EA-4/02 S13 — printed diameter not reproducible from the data
(class c).** The budget (S13.10) uses Δl = 49.999 55 mm although the mean
of the five observations is 49.999 536 mm, and the coaxiality correction
+0.000 004 mm although Eq. (S13.5) gives −0.0037 µm. It obtains
d_X = 90.000 25 mm, reported as (90.000 3 ± 0.000 9) mm. From the data,
d_X = 90.000 232 mm → (90.000 2 ± 0.000 9) mm. The change (0.1 µm) is far
inside U but alters the printed value; F20 now lists three class (c)
findings (Nordtest §6.1, EURACHEM A5, EA-4/02 S13). Also: u(Δl) is printed
0.30 µm, √(0.15² + 0.25²) = 0.29 µm, which gives u_c = 0.404 instead of
0.411 µm (U still reported 0.9 µm with round-up).

**F22. EA-4/02 S12 — s rounded before use (class a).** s of the three
relative errors is 0.00104 (printed 0.001); u = s/√3 = 0.60 × 10⁻³ is used,
consistent with u_c = 0.91 × 10⁻³ and ν_eff = 10. Computing from the
printed s = 0.001 gives 0.89 × 10⁻³ and ν_eff = 11.5 (→ 11), k = 2.25.

**F23. JCGM 101 9.2.3 — validation on the threshold.** For four rectangular
inputs the exact endpoint difference (0.039) lies just below δ = 0.05;
JCGM 101 reports the GUM result validated in one adaptive run and not in
the other (9.2.3.4). With an analytic reference the correct verdict is
"validated". This illustrates that the clause 8 verdict is itself a random
quantity near the threshold — a point worth making in the article
(possible recommendation: report d_low, d_high with their MC standard
deviations).

**F24. QUAM A1 with the volume sub-model (class a).** u(V) = 0.066 mL is
rounded to 0.07 mL and u_c = 0.86 to 0.9 mg/L before k is applied, so the
printed U is 1.8 mg/L; unrounded U = 1.7 mg/L.

### Open questions (new)

- **Q12.** Should `expected.reference` (and source titles) be bilingual
  `Text` like the rest of the metadata? Currently English only.
- **Q13.** Top-down evaluations (Nordtest, MUPR) are not yet expressible
  as project files; design a `topdown` project type with raw validation
  data so that those examples join the library.

### Next steps

1. Top-down project type (Q13) and the Nordtest/MUPR examples in the
   library; NIST TN 1900 examples.
2. Flutter UI on top of the project format (the JSON file is the UI's
   document model).
3. Bilingual user guide generated from the library explanations.

---

## 2026-10-25 (session 5) — Bilingual reference texts; top-down projects from raw data

Author's decisions on Q12 and Q13: reference texts are bilingual; top-down
projects accept raw validation data directly.

**D16. Bilingual text everywhere (closes Q12).** Every `Text` field — now
including `expected.reference` — is written `{"en": …, "tr": …}`; a plain
string is accepted and read as English only, so hand-written files stay
short. The library test requires a Turkish title and reference text in every
library file.

**D17. Top-down project type (closes Q13).** A project contains either a
`model` or a `topdown` section (checked when the file is read). The top-down
section lists the budget components:

* `precision` — from a control-chart `series`, pooled `groups` (raw values or
  s with n), duplicate `pairs` (Nordtest pooled formula, EURACHEM normalised
  differences, MUPR mean range / 1.128; optional selection by pair mean),
  `control_limits` (L/k) or a `given` value; optionally the precision of a
  mean of n results on p days (MUPR Eq. 5) and conversion between absolute
  and relative (at a value or at the data mean);
* `bias` — CRMs, proficiency-test rounds (u(C_ref) = s_R/√n_Lab) or recovery
  experiments; single-CRM formula √(b² + s²/n + u²(C_ref)) when one CRM with
  measured results is given, otherwise √(RMS_bias² + ū²(C_ref));
* `recovery` — reference-material, spiked-sample and recovery-list tests
  (MUPR Eq. 8–12), optional s(R), the significance test (Eq. 13) with a
  documented default for ν, correction rule, and the χ² equivalence test with
  the weighted mean of Eq. (16);
* `components` — further standard uncertainties or rectangular/triangular
  limits, with multiplicity (e.g. a stock solution used twice).

Any component can be restricted to a concentration range [min, max), which
expresses the two-interval precision models of Nordtest and MUPR Appendix A.
The budget is evaluated at the listed `levels`; without levels every
component must be relative and a relative uncertainty is reported.
**Convention:** relative quantities are entered and reported in per cent, in
fields whose names end in `_percent`; all other values are in the unit of the
measurand (QUAM A6 shows why: its values are in % m/m, an absolute unit).

**D18. Recovery-corrected results.** When results are corrected (y = x/R̄),
absolute components are divided by R̄ and relative components are
unchanged: u(y)² = Σ(uᵢ/R̄)² + y²Σu′ᵢ². This is the correct propagation for
y = x/R̄ (cf. finding on MUPR Eq. B6.8, where 1/R̄ is missing on the
precision term); for all-relative budgets it coincides with the guide.

**D19. Rounding to decimal places.** `reporting.decimal_places` rounds U to
a fixed decimal place instead of to significant digits; with
`"rounding": "up"` and 0 decimal places it reproduces Nordtest's "round up
to a whole per cent" (6.4 → 7, 10.4 → 11, 9.7 → 10, 21.5 → 22, 22.8 → 23).

**F25. Nordtest TR 537 §8.2 Example A — n (class b).** s_bias is stated with
n = 19, but Appendix 7 lists 18 duplicate control results; u(bias) is
4.5 % either way.

**F26. Rounded intermediates in MUPR B1 and B5 (class a).** B1 computes U at
50 mg/L with u′(R̄) rounded to 0.0285 (9.26 mg/L; unrounded 9.27 mg/L); B5
uses t₀.₉₇₅(2) rounded to 4.30 (U = 0.395 w; 0.3955 w with 4.303). The
library tolerances are widened accordingly, with the reason in the file.

**Library.** 21 top-down projects added (Nordtest 11, MUPR 8, QUAM 2), all
from raw data where the documents give them; 59 projects in total, all
passing (`cargo test`, `uncertainty check library`). The previously recorded
findings — Nordtest F5 (§6.1), §6.2 mean u(C_ref) 2.7 vs 2.6 %, Example C
summary table, the MUPR B1 statistic (0.982 vs 0.963), B6 χ² (8.80 vs 8.70)
and Eq. (B6.8), QUAM A4 and A6 table errors — are attached to the
respective files as notes.

### Open questions (new)

- **Q14.** MUPR B3 uses a linear precision model s(c) = a + b·c fitted to
  (c, s) pairs; add a `linear` precision model (fit from raw pairs) or keep
  the two-interval and continuous models only?
- **Q15.** *(closed 2026-10-26, D32)* ISO 21748 / ISO 5725 reproducibility-based budgets (s_R with
  bias and additional terms) fit the top-down type already; validate against
  the standards' examples when the documents are available.

### Next steps

1. Bilingual user guide generated from the library explanations (TR/EN).
2. Flutter UI using the project file as the document model.
3. NIST TN 1900 examples; ISO-based parts when the standards arrive.

---

## 2026-10-26 (session 6) — Level-dependent precision models; user guide

**D20. Linear and continuous precision models (closes Q14, author's
decision).** Two precision data types give an absolute standard deviation
that depends on the level: `linear`, s(c) = a + b·c, stated or fitted by
least squares to raw (c, s) points (MUPR Example B3), optionally for a
diluted test solution, s(c) = F·(a + b·c/F); and `continuous`,
s(c) = √(s₀² + (c·s′₁)²) (MUPR Eq. A.1). They are evaluated at each level
and divided by R̄ for recovery-corrected results (D18). The library gains
MUPR B3 from the raw points of Table B3.1 (U = 7.27 mg/L; printed 7.28 with
the intercept rounded to 0.0233).

**D21. User guide (docs/guide, EN and TR).** Eight hand-written chapters per
language — introduction, getting started, GUM framework, Monte Carlo method,
top-down evaluation, reporting, project-file reference, English–Turkish
glossary — and a ninth, *Worked examples*, generated from the library by
`uncertainty catalogue` (results recomputed without Monte Carlo so that the
text is identical on every platform). A test fails if the generated chapter
is out of date, so the guide cannot drift from the program. Every statement
about the program in the guide was checked against the code (supported
functions, defaults, distribution formulas, the behaviour of `"gum": null`).
The method chapters cite clause numbers of the reference documents, so they
double as a compact guide to uncertainty evaluation; they will be the basis
of the in-app help of the graphical interface. The Turkish terminology
(glossary) remains to be confirmed against UME/TSE usage
(documents-needed.md §E).

**D22. Graphical interface architecture.** Flutter (stable 3.47.8) for
desktop and mobile, on top of the Rust engine through a C interface
(`ffi` crate, JSON in / JSON out: `uc_evaluate`, `uc_validate`,
`uc_template`, `uc_version`, `uc_free`; panics are caught and returned as
errors). Reasons: (1) every number shown in the interface comes from the
same validated code as the command-line tool and the reports — the
interface never re-implements a formula (even the reported statement and
its rounding are returned by the engine); (2) the project file is the
interface's document model, so anything the interface can do can be saved,
reviewed and checked with `uncertainty check`; (3) the interface stays thin
and testable. Calculations run on a background isolate. The 60 library
projects are bundled (`app/assets/library.json`, kept in sync by a test).
Top-down data are entered directly — control-chart series, duplicate pairs
and precision-model points as pasted columns (decimal comma accepted) —
while bias, recovery and further components are edited as JSON entries for
now. Verified: `flutter analyze`, 4 Flutter tests (formatting, engine
binding, an end-to-end test that opens a library example, calculates it
and checks the reported result), Linux release build run under Xvfb.
Windows has a build rule but is untested; macOS, Android and iOS packaging
are documented as next steps.

**D23. Zero-sensitivity warning (closes Q10).** GUM warnings are now
structured (`GumWarning`) and localised in reports and in the interface.
When an input with non-zero uncertainty has a zero first-order contribution,
the engine evaluates the second-order terms of the GUM 5.1.2 note that
involve it and, if they exceed 1 % of the variance, warns with the share the
first-order evaluation misses (GUM H.1: α_s, θ̄, Δ, 12 % — exactly
(11.7² + 1.7²)/(31.7² + 11.7² + 1.7²); JCGM 101 9.3 mass; EA-4/02 S4, S13).
Inputs on which the model does not depend at all raise no warning.

**D24. Kragten view (closes Q9).** `"gum": {"kragten": true}` adds Kragten's
finite-difference contributions Δᵢ = f(xᵢ + uᵢ) − f(x) and u_c = √ΣΔᵢ²
(with correlations) to the budget, for comparison with spreadsheet
evaluations; the exact symbolic result remains the GUM result.

**F27. The QUAM spreadsheet values are Kragten values (class a,
explanatory).** With the Kragten view the program reproduces the printed
spreadsheet results of QUAM A1 (0.8633 mg/L), A2 (0.000099 mol/L and all five
contributions), A3 (√3.34 × 10⁻⁸ mol/L), A4 (0.377; contribution of Rec
−0.0507) and A5 (0.001465 mg/dm²; a_V −0.000483) to all printed digits. The
small differences against the exact first-order law of propagation noted
earlier (e.g. A1 0.8638, A4 0.3771, A5 0.001470) are therefore entirely due to
the finite-difference approximation, not to errors. For the article: a
validation that compares software with spreadsheet-based examples must state
which method the reference used; the library now checks both. Large relative
uncertainties (A4: 27 %) make the difference visible in the third digit.

**Estimates in reports** are now shown with enough significant digits to
resolve two digits of their standard uncertainty (e.g. 50 000 838 nm instead
of 5.00008 × 10⁷ nm).

---

## 2026-10-26 (session 7) — NIST TN 1900 examples

**D25. NIST Technical Note 1900 in the library.** TN 1900 (Possolo 2015) is
the third independent family of worked examples after the JCGM/EA documents
and the Eurachem guides, and the only one that systematically states its
input distributions for the Monte Carlo method. Eleven projects reproduce
the examples whose measurand is given by a measurement equation with scalar
inputs: E1 weighing, E2 surface temperature (Type A, ν = 21), E3
falling-ball viscometer, E4 Pitot tube, E8 molecular weight of CO₂, E9
cadmium standard (QUAM A1 with rectangular P and triangular V), E11
microwave step attenuator (EA-4/02 S7 with U-shaped inputs) and E13
thermal expansion coefficient with Gaussian and with t₃ inputs at 95 % and
99 %. Not included: E5 (needs truncated Gaussian inputs), and the
observation-equation examples that require regression, random-effects
meta-analysis, Bayesian or bootstrap methods (E6, E7, E10, E12, E14–E35);
they are candidates for later modules. A new field, *physical properties*,
holds E3 and E13. `tools/library_index.py` now regenerates the tables of
`library/README.md` from the project files.

Reference values are the printed TN 1900 values with a tolerance of half a
unit of the last printed digit plus the Monte Carlo dispersion; where a
printed value is not reproducible from the printed inputs the project uses
the recomputed value and explains the difference in its notes (F28–F30).
For E8 the exact interval end points of the trapezoidal output distribution
are used (44.008045, 44.010755; TN 1900 prints [44.0080, 44.0108]).

**D26. Projects without a GUM evaluation in the interface.** The settings
tab of the interface had silently re-created `"gum": {}` in projects that set
`"gum": null`; it now has a switch for the GUM framework like the one for the
Monte Carlo method, and the catalogue marks such projects as "Monte Carlo
method only". A widget test opens TN 1900 E11, checks that the switch is
off, runs it and checks that only the Monte Carlo result is shown. The
test also exposed an overflow of the drop-down fields of the settings tab
on narrow widths (fixed with `isExpanded`).

### Findings

**F28. TN 1900 E4 (Pitot tube) — results inconsistent with the stated
equation by a factor √2 (class c).** TN 1900 states v = √(2ΔR_sT/p) with
Δ = 1.993 kPa, p = 101.4 kPa, T = 292.8 K, R_s = 287.058 J kg⁻¹ K⁻¹ and prints
v = 40.64 m/s, u(v) = 0.25 m/s, intervals [40.15, 41.14] and [40.17, 41.13]
m/s. The stated equation and inputs give v = 57.48 m/s and u(v) = 0.348 m/s;
the printed values are exactly these divided by √2 (40.645, 0.2461), i.e.
the result of √(ΔR_sT/p). The relative uncertainty (0.61 %) is unaffected.
The project uses the stated equation and checks the printed values × √2.
To check before submission: whether the original example of Kline and
McClintock (1953) defines Δ differently (e.g. as a head in other units), and
any TN 1900 errata.

**F29. TN 1900 E3 (falling-ball viscometer) — "conventional formula" value
not that of GUM eq. (10) (class c, to be confirmed).** TN 1900 states that
Gauss's formula as implemented in the NIST Uncertainty Machine gives
u(μ_M) = 1.11 mPa s, "identical" to the Monte Carlo value. The first-order
law of propagation applied to the stated model and inputs gives 1.027 mPa s
(the t_C and t_M terms alone give 5.687 × √(0.15² + 0.10²) = 1.026 mPa s);
with the second-order terms of GUM 5.1.2 the program gives 1.10 mPa s. The
printed Monte Carlo results (mean 5.82 mPa s, u = 1.11 mPa s, 95 % interval
[4.05, 8.39] mPa s) are reproduced. To check: how the NUM evaluates its
"Gauss" result (possibly not at the estimates, or with a different
derivative approximation) — the point matters for the article because it
concerns what a "GUM result" is in software comparisons.

**F30. TN 1900 E9 — "standard deviation 0.7 mL" for V (class b).** The
triangular distribution of V is described with standard deviation 0.7 mL;
the example's own u(V) = 0.07 mL and the printed interval [1001.0, 1004.4]
mg/L are only consistent with 0.07 mL (QUAM A1). With the stated inputs the
GUM result is not validated for two significant digits of u (end-point
differences 0.026 and 0.022 mg/L against δ = 0.005 mg/L): the rectangular
and triangular inputs shorten the tails slightly. TN 1900's statement that
the results "are still the same" holds to the digits it prints.

**F31. Two conventions for t-distributed inputs (methodological, for the
article).** For an input estimated by the mean of n observations, JCGM 101
6.4.9 assigns t_{n−1}(x̄, s²/n), whose standard deviation is
(s/√n)·√((n−1)/(n−3)) — √3 × s/√n for n = 4. TN 1900 (E11, E13) instead
rescales the t distribution so that its standard deviation equals s/√n. The
two conventions give different Monte Carlo results for the same data (E11:
standard deviation of L_S 0.0091 dB vs 0.0158 dB). Because the GUM
standard uncertainty of a `t` input in this program is its scale (JCGM 101
convention), the TN 1900 projects with rescaled t inputs run the Monte Carlo
method only. A comparison of software packages for Type A inputs must state
which convention each uses.

**F32. TN 1900 E8 — u from rounded components (class a).** TN 1900 prints
u(M_r(CO₂)) = 0.000721 from the rounded components 0.0006 and 0.0002; the
unrounded components (0.000577, 0.000214) give 0.000718. Both round to the
printed 0.0007. The Gaussian interval ±1.96u = ±0.00141 is wider than the
exact trapezoidal interval ±0.00136, so the GUM result is not validated
(n_dig = 2) — a compact example of a linear model with only rectangular
inputs where the Gaussian approximation is conservative.

**D27. Android and Windows packaging.** Android: a Gradle task
(`buildEngine`, run before every build) cross-compiles the engine with cargo
and the NDK's clang (API level = minSdk 24) for arm64-v8a, armeabi-v7a and
x86_64 and packages it as `jniLibs`; the release APK was built in this
environment (NDK 28.2, 57 MB) and contains the engine for the three ABIs,
16 KB-aligned and linked only against the Android system libraries. It has
not yet been run on a device or emulator (no hardware virtualisation here).
Because Android and iOS have no save dialog, projects are saved in the
application's documents folder there, while reports and "Save as" go to the
system share sheet (`share_plus`). Windows: the CMake rule builds
`uncertainty_ffi.dll` with cargo; the engine type-checks for Windows, and the
new *Package* workflow (`windows-latest`) runs the widget tests against the
DLL and builds the Windows zip, together with the Linux bundle and the APK.
The APK is signed with the debug key until a release key exists.
First run of the workflow: all three jobs succeeded; on Windows the six
widget tests (including the end-to-end calculation of a library example and
the Monte-Carlo-only project) pass against the MSVC-built DLL, and the zip
contains `uncertainty_app.exe`, `uncertainty_ffi.dll` and the bundled
library.

### Next steps

- Run the APK on a device; release signing key; macOS and iOS packaging.
- Observation-equation examples of TN 1900 (regression, random effects) as
  a later module; E5 after truncated distributions are supported.
- Errata check of TN 1900 before submission (F28–F30).

---

## 2026-10-26 (session 8) — Documents supplied by the author; NIST Uncertainty Machine manual

**D28. Documents received.** The author supplied 13 documents (licensed
copies of standards are used locally only and are not stored in the
repository): ISO 21748:2017, ISO 5725-3:2023, ISO 13528:2022,
ISO 6143:2025 (BS EN), TS EN ISO/IEC 17025:2017, the NIST Uncertainty
Machine user's manual v1.4 (2020), Wichmann & Hill (2006), Marsaglia &
Tsang (2000), Magnusson & Ellison (2008), Palma et al. (2019), Mateos et
al. (2020), Zeier et al. (2012) and Kallner (2002, eJIFCC 13). The list of
documents still missing is in `documents-needed.md`.

**D29. Random number generators checked against the primary sources.** The
enhanced Wichmann–Hill generator in `core/src/rng.rs` agrees with §3 of
Wichmann & Hill (2006) in the multipliers, moduli and both the 64-bit and
the 32-bit (Schrage) forms; a new test checks the inverses of the
multipliers and the factorisation of the period (≈ 2.66 × 10³⁶) printed in
§4. The gamma generator agrees with the procedure of Marsaglia & Tsang
(2000, §5) including the squeeze 1 − 0.0331x⁴ and the boost
γ_a = γ_{a+1}U^{1/a} for a < 1. Two remarks for the article: (i) the paper
notes that the generator can return 0.0 in floating point; the program
rejects 0 (JCGM 101 C.3.3 requires values in (0, 1) for Box–Muller); (ii)
seeds are derived from one 64-bit seed by SplitMix64, whereas Wichmann &
Hill recommend fixing three seeds and numbering the fourth for parallel
streams — equivalent for single-stream use, to be documented.

**D30. Examples of the NIST Uncertainty Machine manual in the library.**
Seven projects: §3 ratio y = ab/c, §8 end gauge with Gaussian inputs, §10
resistance (GUM H.2 with the rounded values), §11 Stefan–Boltzmann
constant, and GUM H.2 R, X, Z from the unrounded data; §7 and §9 are TN 1900
E13 and E3, already in the library. §12 (complex-valued output) and §13
(implicit model, resampled input) need features not yet available
(multivariate outputs, implicit equations, sample inputs) and are noted for
JCGM 102 work. 78 projects in total.

### Findings

**F29 resolved (class c).** The NIST Uncertainty Machine's own output for
the viscosity example (manual Figure 5) gives u(y) = 1 mPa s for Gauss's
formula, consistent with the first-order value 1.027 mPa s; the 1.11 mPa s
stated in the text of TN 1900 E3 and of the manual §9 is the Monte Carlo
standard deviation (printed as 1.1 in the same figure). The statement that
both methods give the same u is therefore incorrect.

**F33. NUM manual §3 — text inconsistent with its figure (class c).** The
text says the GUM method gives y = 32.2 and u(y) = 12.5 for y = ab/c; the
output in Figure 1 gives y = 28.8 and u(y) = 8.7 for Gauss's formula, which
the program reproduces (32.2 and 13 are the Monte Carlo mean and standard
deviation). The value 12.5 matches neither method.

**F34. GUM H.2 — sensitivity of u_c(R) to the rounding of r (class a,
methodological).** With the two-digit correlation coefficients and standard
uncertainties printed in GUM Table H.2, the law of propagation gives
u_c(R) = 0.0700 Ω (as does the NIST Uncertainty Machine, 0.0699 Ω); from the
five raw sets of Table H.2 it gives 0.0711 Ω, the GUM's 0.071 Ω. The
covariance terms cancel about 70 % of the variance of R, so rounding r to
two digits changes u_c(R) by 1.5 %. For the article: an example of why
software validation should start from raw data where they are available,
and why rounded published correlations can make a reference example fail.
X and Z change by less than 0.1 % (GUM prints u_c(X) = 0.295 Ω; approach 1
gives 0.29558, approach 2 0.29549).

**F35. NUM convention for t-distributed inputs.** The manual (§7) assigns
Student t₃ inputs "with standard deviations" u, i.e. the rescaled
convention of TN 1900 (F31), not JCGM 101 6.4.9. Its results for the thermal
expansion coefficient (95 % interval (1.4, 2.05) × 10⁻⁵ K⁻¹) agree with the
library project `nist-tn1900-e13-thermal-expansion-student-95`.

**D31. Proficiency-testing projects (ISO 13528:2022).** A third project type,
`proficiency`, evaluates one round of a proficiency testing scheme: median,
MADe, nIQR, Algorithm A (C.3.1, with the stopping rule of the third
significant figure or full convergence) and Algorithm S (C.4); the assigned
value from Algorithm A, the median, the mean or a reference value, with
u(x_pt) = 1.25 s*/√p (Formula 6) and optional inhomogeneity, transport and
stability components (Formula 3); σ_pt from the results or given; the scores
D, D %, PA, z, z′, ζ and E_n with their signals; the screening of reported
uncertainties (9.8); and the homogeneity (B.3, with the expanded criterion
of B.2.3, F₁ and F_m computed exactly from χ² and F quantiles) and stability
(B.5) checks. Validation: Tables E.4 and E.5 (Algorithm A reproduced at every
one of the six iterations, including the limits x* ± δ), Table E.7 (all 126
scores of the 21 laboratories) and E.2 (homogeneity and stability); Table
B.1's F₁ and F₂ are reproduced from their definitions. Reports (EN/TR) with a
z-score chart and a homogeneity chart; the interface has a data editor in
which results are pasted from a spreadsheet, and a results view. Relevance:
TÜBİTAK UME organises proficiency tests and the same statistics are needed by
participants (ζ and E_n against their own uncertainty). 82 library projects.

**F36. ISO 13528 E.4 — uncertainty flags not reproducible from 9.8
(class b, to clarify).** Table E.6 flags the reported uncertainties a/b/c
"following criteria discussed in 9.8", but with u_min = u(x_pt) = 0.0041
mg/kg (9.8.3) L05 (u = 0.0035) and L09 (u = 0.0040) would be flagged b,
whereas the table flags them a. The printed flags imply a lower limit between
0.0018 and 0.0035 mg/kg (perhaps the characterisation uncertainty without the
inhomogeneity component) and an upper limit between 0.005 and 0.015 mg/kg
(consistent with 1.5σ_pt = 0.0099 mg/kg rather than with 1.5 s*). The scores
of Table E.7 are all reproduced; the flags are not checked.

**F37. Bimodal results and Algorithm A.** In ISO 13528 E.4 (two groups of
methods) Algorithm A converges in three iterations to the arithmetic mean
and 1.134 × the standard deviation: with a robust scale that grows at each
step no result remains outside x* ± 1.5s*. A robust average is therefore no
safeguard against a mixture of populations; ISO 13528 handles this case by a
reference value (as in E.4) or separate assigned values per method (9.2.2 c).
A note for the guide chapter and the article.

**D32. Precision from collaborative studies (closes Q15).** Two top-down
precision types: `reproducibility` (ISO 21748:2017) — √(s_L² + s²/n_r) with the
laboratory's verified repeatability s_l in place of s_r (7.3.2), n_r averaged
replicates (Table 1), the method-bias uncertainty of Formula (15) and the
bias-control check |Δ_l| < 2s_D of Formulae (4)–(5); and `staggered_nested`
(ISO 5725-3:2023, Annex C.1), whose ANOVA gives s_r, s_I(T) and s_R. The
library gains ISO 21748 C.1 (CO emissions), C.2 (meat content, as a GUM model
with relative error terms e_L, e_r, e_fat — the model of Formula (1) written
explicitly), C.3 (aerobic plate count, three foods) and C.4 (crude fibre), and
ISO 5725-3 J.1 (carbon in steel) and J.2 (vanadium, level 1). 90 projects.

**F38. ISO 21748 C.4.7 — "dividing … by 3" (class b).** The uncertainty of
drying to constant mass, 0.115 % for 0.002 g on 1 g, is said to be obtained by
dividing the maximum deviation by 3; it is 0.002/√3 (rectangular). With the
text's 3 it would be 0.067 %.

**F39. ISO 21748 C.3 — flour (class c).** Table C.4 prints u = 6.4 % and C.3.7
U = 12.8 % for flour; the table's own inputs (2.4, 5.0, 3.0 %) give 6.30 % (6.29 %
with the unrounded s_L) and U = 12.6 %. Shrimp (7.8 %) and vegetables (8.9 %)
are reproduced. The intervals of Table C.5 for flour follow from 12.8 %.

**F40. ISO 5725-3 J.2 — Table J.5 not reproducible at levels 3–5 (class c,
to verify against the printed standard; resolved in D41: the 2023 Table J.2
has transcription errors, and s_R at level 6 keeps a negative variance).** From the data of Table J.2 (as
extracted from the PDF) with the stated outlier exclusions, levels 1 and 2 of
Table J.5 are reproduced exactly (e.g. 0.381, 0.603, 0.801 × 10⁻³ %), and level 6
gives s_r = s_I(T) = 9.545 × 10⁻³ as printed but s_R = 16.33 instead of 15.962.
Levels 3, 4 and 5 differ in all three standard deviations (level 3: 0.652,
2.348, 2.777 against 1.739, 2.305, 2.650; level 5: lab 3's 0.599 % dominates
s_r). No exclusion of up to three laboratories and no permutation of the
columns reproduces them. Also, the column "Average (%)" of Table J.5 carries
a spurious "× 10⁻³". Before reporting, the extracted data should be compared
with the printed table.

## 2026-10-26 (session 9) — Calibration with uncertainties in both variables (ISO 6143:2025)

**D33. Generalized least squares with exact covariance propagation.** New
core module `gls` and project type `calibration`. The analysis function
x = G(y; b) — linear, quadratic, cubic, power b₀ + b₁y^(1+b₂) or exponential
b₀ + b₁e^(b₂y) (ISO 6143 Formulae 1–5) — is fitted by minimising
S = Σ[(G(ŷᵢ) − xᵢ)²/u²(xᵢ) + (ŷᵢ − yᵢ)²/u²(yᵢ)] over b and the adjusted
responses (A.3), with Levenberg–Marquardt from a weighted least-squares
start. ISO 6143 A.2 propagates the input uncertainties with derivatives
∂b/∂z obtained by numerical differentiation of the whole fit. A first
implementation that way gave covariances varying with the step (for the power function u(b₀, b₁)
changed by 6 % at h = 10⁻⁵u) because each refit stops at its own tolerance; the program therefore computes ∂θ/∂z = −H⁻¹ ∂(∇S)/∂z
at the minimum, θ = (b, ŷ), with the full Hessian H including the second
derivatives of G (implicit-function theorem). This is the exact limit of the
standard's procedure, needs no refits and no step, and handles covariances of
the contents (A.7). Validation: all printed results of Examples 1 and 2
(linear, quadratic, with covariances) to the printed digits; Example 3 power
function within 0.3 % in the covariances (B.1 itself says recalculation "can
exhibit slight deviations … in particular for the power function and the
exponential function"); ODRPACK's Gauss–Newton covariance differs by 0.9 %
for the power function, as expected for a non-zero residual. Reports (EN/TR)
give the weighted deviations, Γ, S_res against 2(n − N − 1), the comparison
of the candidate functions (6.2.2), the contents of samples with their
correlation matrix, the upper bound of 6.2.3, drift control (6.2.4) and the
check of Formula (13) for mismatching mixtures. Six library projects (96 in
total); an editor and results view in the application.

**F41. ISO 6143:2025 Formula (12) — critical value of the drift test (class c).**
The Note to 6.2.4 states that with n = 10 measurements before and after,
the critical value for each of the three differences is 2√2 u[yᵢ(calib)]. The
generalisation for n < 10 prints 2√(1 + 10/n) u for the differences from the
calibration mean (Formulae 10, 11; 2√2 u at n = 10 ✓) and 2√(1 + 20/n) u for
the difference between the means before and after (Formula 12), which gives
2√3 u = 3.46 u at n = 10, not 2√2 u. Both means have standard deviation
u√(10/n), so their difference has u√(20/n), and Formula (12) should read
2√(20/n) u (2√2 u at n = 10, consistent with the Note). The program uses the
derived form, with the number of calibration measurements n_calib (default
10) in place of the 10 of the formulae. Verified on the rendered page of the
licensed copy (BS EN ISO 6143:2025, p. 11).

**F42. ISO 6143:2025 Example 3 — exponential function: parameter
uncertainties and u(x) (class c; partly acknowledged by the standard).** B.1
and B.2.4 state that B_LEAST "provides incorrect values for the parameter
uncertainties of the exponential function" (found at METAS) and print them
anyway: u(b) = 8.1404 × 10⁻², 7.3965 × 10⁻², 3.7068 × 10⁻¹¹. The correct values
are 3.18, 3.17 and 1.274 × 10⁻⁶ (this program), 3.18, 3.17, 1.274 × 10⁻⁶
(ODRPACK, scipy.odr cov_beta) and 3.13, 3.12, 1.24 × 10⁻⁶ (Monte Carlo
simulation of the calibration, 4000 refits with all xᵢ, yᵢ and y drawn from
their normal distributions). The printed values are about 39 times too small
for b₀, b₁ and 3.4 × 10⁴ times for b₂. Not acknowledged: the content of
the mixture in Table B.12 with the exponential function carries
u(x) = 0.013280 µmol/mol, whereas the correct first-order value is 0.014246 and
the Monte Carlo value 0.01406 µmol/mol. The printed value is 6.8 % (5.6 %)
too low. This exceeds the "about 3 % relative" that B.2.4 gives as the largest
effect of iteration settings on assigned contents. The standard's conclusion
that the two functions give the same content within their uncertainties
holds either way (|5.3456 − 5.3357| = 0.0099 < 0.0142). For the power
function, Table B.12 (0.014169) agrees with the program (0.014171) and with
Monte Carlo (0.01420). Note also that the quadratic polynomial, not tried in
the standard, fits Example 3 with Γ = 0.44 (exponential 0.35, power 1.16).

**Note on ISO/TS 28037 (roadmap 2f).** The GLS engine covers the
straight-line cases of ISO/TS 28037 (uncertainties in both variables,
correlated x). The standard's own data sets are still needed for validation
(documents-needed.md).

## 2026-10-26 (session 10) — Uncorrected bias; ISO/IEC 17025 mapping; literature

**D34. Results not corrected for a known bias.** New core module `bias` and
the model-project option `reporting.uncorrected_bias` (`bias`, `u_bias`,
`strategy`): the five strategies of Magnusson & Ellison (2008, Table 3) —
RSSu, RSSU, SUMU (asymmetric), SUMU_MAX and Ue. Ue (Synek 2005) is
implemented from its definition: the symmetric interval about the
uncorrected result that contains the fraction 2Φ(k) − 1 of the distribution
N(y + Δ, u_c²) of the corrected value. It is the default, as Magnusson & Ellison
recommend it for large degrees of freedom. Validation:
(i) the formulae; (ii) Ue tends to 1.645u_c + |Δ| for |Δ| > 0.8u_c, as stated;
(iii) a simulation of their Fig. 1 (u(y) = 1, u_Δ = 0.5, 40 000 trials; the test uses 20 000) gives
SUMU 95.2 % at zero bias and 97.5 % at 3u_c, Ue 98.1 % and 95.0 %, and RSSu
and SUMU_MAX above 95 % at 0, 1 and 3u_c. All of these agree with their text and
contradict the poorer SUMU coverage reported by O'Donnell & Hibbert (2005);
(iv) the ordering of interval widths in their Fig. 2. Reports add a second
statement naming the strategy, e.g. "c_Cd = 1002.7 mg/L (−3.3, +0.3), not
corrected for the estimated bias b = 1.50 mg/L …". The correction with its
uncertainty remains the recommended treatment (GUM 6.3.1).

**F43. Magnusson & Ellison (2008), Table 3 — sign of Δ in SUMU (class b).**
The table defines Δ = −b as the correction, but gives U₊ = max(0, ku_c − Δ),
U₋ = max(0, ku_c + Δ). That is the form of Phillips, Eberhardt & Parry (1997),
U₊ = U − d, U₋ = U + d, but there d is the bias, so with Δ = −d it should read
U₊ = max(0, ku_c + Δ), U₋ = max(0, ku_c − Δ). Taken literally with their Δ, a
positive bias would move the interval further up, away from the measurand.
The program uses the primary source. The table also states that the equation
applies when k·u(y_obs) ≥ Δ, whereas Phillips et al. define it for all d (with
the max(0, ·) limits).

**D35. ISO/IEC 17025 mapping.** `iso17025-mapping.md` maps clauses 7.2.2,
7.6, 7.7, 7.8.3, 7.8.4, 7.8.6 and 7.11 to the program and names the gaps:
sampling uncertainty (7.6.1), control-chart rules (7.7.1), conformity
statements with decision rules (7.8.6, JCGM 106) and a printable software
validation report (7.11.2). These are candidates for the next phases.

**Literature read in this session** (literature-review.md): Palma et al.
(2019) on matrix effects and Mateos et al. (2020) on recovery correction are
both top-down methods from the Lisbon group, with worked data (Pb in
sediments; PAH in water) that could become library projects. They need the
recovery-compatibility tests and the ν-dependent weighted mean recovery,
which the program does not have yet. Kallner (2002) is an introduction for
clinical laboratories. Zeier et al. (2012) describes METAS UncLib, the
METAS library whose authors found the B_LEAST error of ISO 6143 (F42).

**D36. Errata check of NIST TN 1900 and the NUM manual (2026-10-26).** The
TN 1900 PDF that NIST distributes today
(nvlpubs.nist.gov/nistpubs/TechnicalNotes/NIST.TN.1900.pdf) is byte-identical
(MD5) to the copy used for the library (PDF modification date 2015-11-03).
The NIST publication page (DOI 10.6028/NIST.TN.1900, page updated
2021-06-02) lists no erratum or revision. The User's Manual on
uncertainty.nist.gov (software version 1.6.4) is version 1.4 of 2020-11-26,
identical to the supplied copy. F28–F32 (TN 1900) and F29, F33, F35 (NUM)
therefore refer to the current versions. No companion code or errata
page for the TN 1900 examples was found.

**D37. ISO 13528 E.1 and E.5 in the library (100 projects).** E.1: the 23
results with five censored values in its three treatments (‘<’ ignored,
censored results deleted, half the limit), Algorithm A reproduced. E.5:
assigned value from a comparison with a CRM, written as the model
x_pt = x_CRM + d with d a Type A input from the 20 differences of Table E.8;
the differences recomputed from the duplicate results agree with the
table's last column, and x_pt = 23.35, u = 0.35 LA units are reproduced.

**F44. ISO 13528 Table E.1 — x* for "0.5 × ‘<’ value" (class a).** With the
stopping rule of C.3.1 Algorithm A stops after 11 iterations at
x* = 23.960, s* = 8.591; iterated to convergence x* = 23.9585, s* = 8.600.
The table prints 23.95 and 8.60, i.e. the converged values with x* truncated
(23.9585 rounds to 23.96). The other two columns (26.01/7.23, 26.81/5.29) are
reproduced with the C.3.1 stopping rule.

## 2026-10-26 (session 11) — Conformity assessment (JCGM 106:2012)

**D38. Conformity assessment.** New core module `conformity` and section
`conformity`. It can accompany a model project, in which case the GUM result
(normal, or t with ν_eff) and the Monte Carlo values are assessed, or form a
project of its own. Implemented:
- the probability of conformity for one- and two-sided tolerances (7.1–7.4)
  and the capability index C_m (7.6);
- simple acceptance and guarded acceptance/rejection, with the guard band set
  by w = rU (U = 2u, ISO 14253-1 default r = 1), by its length, by a required
  probability at the acceptance limit (8.3.3; the uncertainty may be relative
  to the measured value, as in the speed-limit example) or by explicit
  acceptance limits;
- specific risks (9.3.2);
- global consumer's and producer's risks for normal and gamma process PDFs,
  by adaptive quadrature of Formulae (17)–(20) (9.5);
- the guard-band multiplier for a required consumer's risk (9.5.4).

All numerical examples of clauses 7–9 are reproduced: seven library projects
(107 in total) and core tests for 7.7.5, 8.3.2 and Figure 17. An independent
check with scipy (quad) agrees to all printed digits. Reports (EN/TR) state the
rule, the acceptance interval, p_c and the decision per result, and a chart in
the style of Figure 8, or the PDF at the acceptance limit for design studies.
The application has an editor for conformity projects, a conformity block in
the settings of model projects and a results section. This closes the
ISO/IEC 17025 7.8.6 gap of D35.

**F45. JCGM 106:2012 9.5.4 (ball bearings) — acceptance interval inconsistent
between R_C and R_P (class b).** The text defines the acceptance interval as
0 ≤ Y_m ≤ A, and the integrand of R_C,
Φ((A − η)/u_m) − Φ(−η/u_m), includes the lower limit 0. Formula (24) for R_P
uses 1 − Φ((A − η)/u_m), i.e. no lower acceptance limit. With the lower limit
0, R_P(r = 0.656) = 8.94 %. Without it, R_P = 7.55 %, the "about 7.5 %" of the
text and of Figure 16. R_C is the same either way (the term Φ(−η/u_m) is
negligible for η ≥ 2 µm), and so are r ≈ 0.65 and A ≈ 1.7 µm. The library
project uses the acceptance interval Y_m ≤ A, which reproduces the printed
numbers, and states the inconsistency.

**D39. Palma et al. (2019) in the library (114 projects).** Seven top-down
projects (Cr, Cu, Li, Mn, Zn by OSPAR; Ni, Pb by EPA 3051A in marine
sediments) combine the components of Table 9 in two intervals with the
existing range mechanism. The components are subsampling-inclusive
repeatability, intermediate precision of a homogeneous sample, matrix
effects from the recoveries of 36 proficiency tests, and trueness
u(R̄)/R̄_w. The combined uncertainties of both intervals and the matrix-effect
shares χ_ME are reproduced within the rounding of the printed components.
The recovery significance test of Formula (10) classifies Cr, Ni and Pb as
different from 100 %, as in the paper. Two small inconsistencies are recorded
in the Pb project: u′_P is 6.38 % in the text and 6.37 % in Table 9, and
u_c(60 mg/kg) is 6.04 against 10.08 % × 60 = 6.05. The raw data needed to
implement the paper's recovery-compatibility tests (Formulae 11–12) and the
weighted mean recovery are only in the electronic supplement (Cordeiro et al.
2018 describes the method; both are on the missing-documents list).

**F46. Mateos et al. (2020), Tables 2 and 3 — Interval II not reproducible
from the listed components (class b, to verify with the supplementary
data).** The relative combined uncertainty above the spiking level should be
u′ = √(u′_I² + (u_T/R̄_w)²), the relative form of the Interval I combination
u = √(u_I² + (u_T γ_SL/R̄_w)²) (Section 2). Interval I reproduces for all
eight cases (4 PAH × neat and adjusted calibrators) within rounding.
Interval II reproduces for only two of them; for neat calibrators:

| PAH | printed u′ | computed u′ |
|---|---|---|
| Pyr | 1.9 % | 1.75 % |
| BaP | 2.8 % | 2.62 % |
| Phe | 5.9 % | 6.06 % |
| Nap | 5.0 % | 5.04 % |

The headline result, naphthalene U′ falling from 9.9 % to 3.5 %, does not
follow from the tables either:
- neat calibrators: the printed components give U′ between 9.99 % and
  10.16 % for any u_T that rounds to 0.010;
- adjusted calibrators: 2 × 1.7 % = 3.4 %, not 3.5 %.

The paper's conclusion (a large reduction for Nap) is unaffected. No library
projects are made from this paper until the electronic supplement is
available.

## 2026-10-26 (session 12) — ISO 5725-2:2019 basic method; F40 resolved

The author supplied ISO 5725-2:1994, BS ISO 5725-2:2019, ISO 5725-3:1994
(with Technical Corrigendum 1:2001) and BS ISO 5725-3:2023 (licensed copies,
used locally only; only numerical example data and results enter the
repository).

**D40. Interlaboratory precision experiments (ISO 5725-2:2019).** New core
module `interlab` and project section `precision_experiment` (120 library
projects). The module covers:

- Critical values of Cochran's test (Formula D.1), Grubbs' single test
  (D.2), Grubbs' double test (Table 6 for p = 4 … 40, otherwise D.3 with the
  one-tailed level α/2 and the coefficients of Table D.1) and the indicators
  of Mandel's h and k (D.5, D.6). All values of Tables 5 to 8 checked in the
  core tests are reproduced. The D.3 approximation agrees with Table 6 to
  within its stated ±0.003 for p = 6 … 40. Table 6 itself is embedded, since
  D.3 is not exact.
- Cell statistics (Forms B and C), with optional rounding to the recorded
  decimals (`form_decimals`, 8.2.10–8.2.11). The standard's own tests are
  computed from the rounded forms (C.1.6). Without this rounding the Grubbs
  statistics of Table C.4 differ by up to 0.011, and Cochran's C at level 1
  is 0.350 instead of 0.341.
- h and k with deviations from the weighted general mean m̂, as 8.3.2.2
  prescribes (ȳ_j of 8.4.4), not from the unweighted mean.
- Cochran's test with n the number of results in the majority of cells
  (8.3.4.3), repeated while the highest value is an outlier (8.3.4.6).
- Grubbs' procedure of 8.3.5.3: single low and high. After a single outlier
  the other extreme is tested without it and the double tests are skipped;
  otherwise the double tests are applied.
- Formulae (23) to (31), including unequal n_ij and s_L² < 0 → 0.
- REML (Annex B.2): the restricted likelihood of the one-way random-effects
  model, profiled analytically over σ_r² (σ_r² = Q(γ)/(N − 1)) and maximized
  over γ = σ_L²/σ_r² ≥ 0 by a grid on ln γ and golden-section refinement,
  with the boundary γ = 0 checked explicitly. This reproduces Tables C.6,
  C.13 and C.19 to all printed digits, as 8.4.6.2 requires.
- Relationships I to IV (8.5.2–8.5.4). II and III use two weighted fits, the
  second being the final result; this reproduces Tables 1 to 3.
- A_r and A_R of Annex A (checked against Table A.1).

Removing data is the panel's decision (8.3.3.1 b, 8.6). The program never
removes anything itself. `exclude` records each laboratory, cell or result
left out, with its reason. The report repeats the scrutiny on the retained
data, and it warns when statistical outliers remain among them
(8.3.3.1 c).

The library has six projects: C.1 sulfur in coal (unequal replicates, Forms
rounded to three decimals), C.2 softening point of pitch (missing cells, a
discarded single result, n = 2) and C.3 creosote oil (an outlying laboratory,
a rejected cell, relationships I–IV of Tables 1–4), each also with REML.
Every printed Cochran and Grubbs statistic, m̂, s_r and s_R is checked,
together with the relationship parameters that are reproducible (F49).

The user interface has an editor and a results view (flags of the scrutiny,
s_r/s_R table, relationships). The guide has a new chapter 12 and section
7.10.

The ISO/IEC 17025 7.6.3 gap ("ISO 5725-2 outlier tests and s_r/s_R from
collaborative data") is closed.

**F47. ISO 5725-2:2019 C.1.5 and C.1.8 — stragglers in Example 1 (class a).**
The text says that the double-high results at levels 2 and 4 (laboratories 3
and 6) are stragglers. At level 4, however, Table C.4 prints 0.132, which is
above the 5 % critical value 0.1101 (the engine gives 0.1298 from the forms
and 0.132 from the rounded forms). Only level 2 (0.108) is a straggler pair.
C.1.8 then reports "four stragglers detected and retained". Table C.4 and
Formula (C.3) give three: the pair at level 2 and the Cochran straggler of
laboratory 5 at level 3. The same text and table are in ISO 5725-2:1994,
B.1.5 and Table B.4. In the same example, Formulae (C.2) to (C.4) print two
different sums of squares in each line (0.006 36 and 0.006 390; 0.001 72 and
0.001 765; 0.004 63 and 0.004 649). The test values follow from the second.

**F48. ISO 5725-2:2019 Table C.12 — s_R at level 4 of Example 2 (class a).**
Formulae (29) to (31), applied to the data of Table C.7 with p = 16, give
s_R = 1.9175 °C. Table C.12 prints 1.915, while the REML Table C.13 prints
1.918. For data with two results in every cell, REML and ANOVA coincide when
σ_L² > 0, so 1.918 is correct and 1.915 is a slip. The other fifteen values
of Table C.12 are reproduced, and ISO 5725-2:1994 prints the same 1.915.

**F49. ISO 5725-2:2019 Table 4 — slope of relationship IV (class a).** The
regression of the tabulated lg s on lg m gives c = −1.5060 and d = 0.7688.
The unrounded s_r of Table C.18 give c = −1.5069 and d = 0.7696. The table
prints c = −1.5065 and d = 0.772. Its row of fitted values (0.089, 0.158,
0.239, 0.257, 0.316) follows from d ≈ 0.768–0.769; d = 0.772 would give 0.090,
0.159, 0.241, 0.260, 0.320. The slope is therefore misprinted (or comes from
another computation), and the library checks c only. The table is the same
in ISO 5725-2:1994 (8.5, Table 4).

Minor points in ISO 5725-2:2019, not separately numbered:

- Formula D.2 writes the degrees of freedom of t as p − 2, while its key says
  n − 2.
- Table 6 (single test) differs from Formula D.2 by up to one unit in the
  third decimal, e.g. p = 15, 5 %: 2.549 against 2.5483. This is within the
  tolerance of the D.2 approximation stated in D.2.1.
- Table 2 rounds the slope of the second fit to 0.0154, whereas the
  tabulated m and s give 0.01554.

**D41. F40 resolved with ISO 5725-3:1994 (class a, typesetting of the 2023
edition).** Table J.2 of ISO 5725-3:2023 differs from Table D.2 of ISO
5725-3:1994 in 15 entries:

- 14 at level 3, where for laboratories 7 to 20 the second day-1 result
  repeats the first;
- lab 2, level 5, result 1: 0.529 instead of 0.520;
- lab 3, level 5, result 1: 0.599 instead of 0.500;
- lab 18, level 4, result 1: 0.214 instead of 0.224.

With the 1994 data and the stated exclusions, Table J.5 (= Table D.5) is
reproduced exactly at levels 1 to 5. Level 6 has a negative estimate
s₍₁₎² = −2.68 × 10⁻⁵. The table prints s_I(T) = s_r = 9.545 × 10⁻³, that is,
with s₍₁₎² set to zero. Its s_R = 15.962 × 10⁻³ instead keeps the negative
s₍₁₎² both in s₀² and in s_R². With s₍₁₎² = 0 throughout, s_R = 16.331 ×
10⁻³, and the engine reports that value. The core test
`staggered_nested_iso_5725_3_1994_levels_3_to_6` covers levels 3–6. The
J.2 library project (level 1, identical in both editions) records the
finding. It goes into the article; the institutions are not contacted (D12).

**F9 (status).** The 99 % duplicate-range factor 3.69 of MUPR B1 is a
critical range factor f(n) of ISO 5725-6. ISO 5725-2 does not tabulate these
factors, and the point stays open until ISO 5725-6 is available.

## 2026-10-26 (session 13) — Several output quantities (JCGM 102:2011)

**D42. Multivariate models (JCGM 102).** New core module `multivariate` and
project section `multivariate` (133 library projects). It covers:

- Explicit models Y = f(X) and implicit models h(Y, X) = 0. Implicit models
  are solved by Newton's method with exact symbolic derivatives. The step
  test is relative to the size of the variable or of its starting value,
  with a stagnation rule at the level of rounding (needed for θ₁ = 0.01 °C in
  9.5.3).
- The generalized law of propagation, U_y = C_x U_x C_xᵀ (formula 3). For
  implicit models ∂Y/∂X = −C_y⁻¹C_x is obtained by solving the linear system,
  as Annex B recommends.
- Coverage factors for N(y, U_y): k_p from χ²_m and k_q from q = 1 − (1 − p)/m.
  Tables 1 and 2 are reproduced.
- Monte Carlo coverage factors: k_p from the draws transformed with the
  Cholesky factor of U_y (7.7.2), k_q from the largest standardized deviation
  (7.7.3), and the area of the smallest coverage region on a 100 × 100 mesh
  for m = 2 (7.7.4). The fraction of the draws inside the GUM ellipsoid and
  rectangle is also given.
- The adaptive procedure of 7.8.3, with at least eleven batches; it requires
  y_j, u(y_j), λ_max, k_p and k_q to stabilize, as in 9.2.2.3.
- The validation of clause 8, reported separately for the ellipsoid and the
  rectangle.
- Inputs from series of vector indication values (5.3.2). The GUM framework
  uses U_x = M/(n(n − 1)) or V(X) = M/((ν − 2)n); the Monte Carlo method draws
  from t_ν(x̄, S/n) as ξ = x̄ + Lz√(ν/w), w ~ χ²_ν, with the existing gamma
  generator.
- The expression language gains `atan2(y, x)`, with its derivative
  ∂atan2(a, b) = (b a′ − a b′)/(a² + b²).

The library has 13 projects. 9.2.2–9.2.4: the GUM and Monte Carlo rows of
Tables 3–5 and the validation outcomes. 9.3: six co-ordinate-transformation
cases (Tables 6 and 7, Monte Carlo with 10⁶ instead of 10⁷ trials). 9.4: the
GUF, Alt. GUF and MCM rows of Table 11. 9.5.2 and 9.5.3: implicit models with
θ, u(θ), Table 15 and the correlations of Table 16. Core tests also cover
6.5.2.3 (areas 26.6, 28.4, 11.8), 6.5.4 (F₂,₁₀ = 4.10) and 7.7.4 (smallest
region 9.4 unit² for 10⁶ draws).

Two observations on Monte Carlo values printed from few draws:

- The 7.7.2 example gives k_p = 2.33 and an area of 10.6 from 1 000 draws.
  With 10⁶ draws the values are k_p = 2.27 and 10.1. This is the counterpart
  of k_p = 2.28 in Table 5 for a similarly dominant rectangular effect, so
  the difference is sampling variation, not an error.
- In 9.4 the inputs follow t₃, which has no fourth moment, so the sample
  standard deviations of the outputs converge slowly. With 10⁶ trials, seeds
  1–6 give u(R) between 0.128 and 0.136 Ω (printed 0.130) and r(R, X) between
  −0.580 and −0.600. The library tolerances cover this spread, and the guide
  (13.5) warns about ν ≤ 4.

**F50. JCGM 102:2011 7.7.2 example — transposed Cholesky factor (class a).**
Step a) of 7.7.2 requires L to be the lower triangular matrix with U_y = LLᵀ.
The example prints L = [[1.414, 1.344], [0.000, 0.442]], which is upper
triangular, and its inverse [[0.707, −2.151], [0.000, 2.265]]. This L
satisfies LᵀL = U_y = [[2.0, 1.9], [1.9, 2.0]], while LLᵀ = [[3.806, 0.594],
[0.594, 0.195]] ≠ U_y. The printed matrices are therefore the transposes of the
Cholesky factor and of its inverse. Applying the printed L⁻¹ in formula (21)
would sort the draws by (y − ŷ)ᵀ(LLᵀ)⁻¹(y − ŷ), the wrong metric. The figure
and the stated results are consistent with the correct factor. A related
small point: 6.5.2.3 example 2 gives the rectangle area 40.1 unit², which
follows from k_q rounded to 2.24; the unrounded k_q = 2.2414 gives 40.2.

**F51. JCGM 102:2011 Table 8 — set 6 is a rounded average (class a).** The
note to 9.4.1.3 states that set 6 is the average of sets 1 to 5. φ is printed
as 1.0445 rad, but the average is 1.04446 rad, the estimate of Table 9. With
the printed 1.0445 the average over six sets is 1.044467 rad, and R̂ becomes
127.7307 Ω instead of the printed 127.732 Ω (X̂ and Ẑ are unaffected at the
printed digits). The library uses 1.04446 and records the point. Table 15
also labels its seventh column θ6 instead of θ7.

## 2026-10-26 (session 14) — Software validation report; control charts (Nordtest TR 569 ed. 6)

**D43. "Two out of three" counts previous values beyond the action limit.**
TR 569 Chapter 9 calls a value out of control when it lies between the
warning and action limits and "at least one of the two previous control
values on the same side is also between warning and action limit". The text
does not say how a previous value *beyond* the action limit counts; that run
was itself out of control. The program counts any previous value outside the
warning limit on the same side, as test 5 of ISO 7870-2 does ("two out of
three points in zone A or beyond"). This is the more cautious reading. A value
exactly on a limit counts as inside it. The trend rule counts seven values
(six steady increases or decreases), and the run rule counts ten of eleven
values strictly on one side.

**D44. Control charts (Nordtest TR 569 ed. 6, 2026).** New core module `qc`
and project section `control_chart` (154 library projects). It covers:

- X, R and r%-charts, from control values or from the replicate results of
  each run.
- Statistical, target and tolerance limits, with a mean, given or reference
  central line.
- Charts with action limits only.
- The daily rules (action limit, two out of three) and the rules for being
  out of statistical control.
- The long-term review: F-test against the exact F₁₋α/₂(ν₁, ν₂), t-test with
  the pooled s_p, the 0.37 s and "6 of 60" checks, and exclusion beyond 4 s.
- s_r and s_Rw from replicate control results by pooling (Equations 9, 10),
  by the simplified method of Example 12 and by one-way analysis of variance.

The range factors d₂ and d₃ for n = 2–10 are stored to six decimals and
recomputed in a test by numerical integration of the distribution of the
range (agreement 2·10⁻⁶ and 10⁻⁴). D_AL = d₂ + 3d₃ and D_WL follow from them.
The library has 21 projects: Table 1 (raw data, daily rules), the five cases
of Chapter 7 and Examples 1–12. Most examples print only summary statistics
(the charts are figures), so their projects check the central line, the
limits and the review statistics.

**D45. Software validation report (ISO/IEC 17025 7.11.2).**
`uncertainty check … --report FILE.md|.html --lang en|tr` writes a record of
the validation. It gives the version, platform, date and options; results by
method and by project with the largest |Δ|/tolerance; the values outside
tolerance; and a signature line. With `--no-mcm`, the Monte Carlo reference
values are no longer compared; before this change they counted as failures.

**F9 resolved.** The factor 3.69 of MUPR B1 is the action-limit factor of a
range chart for duplicates, D_AL = d₂ + 3d₃ = 3.686 (ISO 8258 D₂). It is not
a quantile of the range. TR 569 Table 4 lists it with the warning-limit
factor 2.833 and notes that it corresponds to a confidence level of 99.07 %,
not 99 %. The exact 99 % quantile of the range of two normal values is
z₀.₉₉₅·√2 = 3.643. The MUPR label "99 %" is therefore a rounded description
of the ISO 8258 factor (class a).

**F52. TR 569 ed. 6 Chapter 7, range charts, case 1 (class b).** The warning
limit is printed as "+ 3·0.356 = 1.0 %". The factor of Table 4 is 2.83, and
2.83·0.356 = 1.01 % (3·0.356 = 1.07 %). The printed result 1.0 % is that of
2.83. Case 2 of the same page uses 2.83 correctly.

**F53. TR 569 ed. 6 Example 12, footnote 24 (class b).** The footnote gives
s_Rw = √(0.01541² + 0.02246²) = 0.0272. The analysis of variance of the 3 × 8
data gives s_r = 0.1541, s_I = 0.2246 and s_Rw = 0.2724, so all three
footnote values are ten times too small. The text value 0.272 is correct.

**F54. TR 569 ed. 6 — rounded intermediates (class a).**

- Example 6 multiplies CV = 1.67 % by the factors (4.73 %, 6.16 %), but
  1.88/1.128 = 1.667 % gives 4.72 % and 6.14 %.
- Example 8 reads F_crit = 1.67 from Table 3 at 60/60 degrees of freedom; the
  exact F₀.₉₇₅(58, 59) is 1.677. It prints F = 1.037 for the R-chart, where
  0.0975²/0.0957² = 1.0380, and s_p = 0.07545, where 0.07544 follows from the
  printed s values.
- In Table 4, D_WL = 3.470 for n = 3 follows from the rounded d₂ = 1.693 and
  D_AL = 4.358; the unrounded constants give 3.469.

None of these changes a conclusion. In Examples 3 and 8, the s called the
"pooled repeatability standard deviation" is the mean range divided by 1.128
(Equation 4): 0.559/1.128 = 0.496 and 0.11/1.128 = 0.0975. Table 1 has three
values outside the warning limits, not four: 65.4 µg/L is just inside the upper
warning limit of 65.47 µg/L.

## 2026-10-26 (session 15) — UKAS M3003 ed. 6 examples; coverage factor for a dominant rectangular term

**D46. UKAS M3003 ed. 6 (2024) in the library.** M3003 is free from UKAS and
is the uncertainty guide most accredited calibration laboratories in the UK
follow. Its Appendix K says the examples "may also be used for the purpose of
software validation". The library gains 19 projects: K.1–K.11 (K.7 and K.11 as
both of their budgets), the dominant-term example of Appendix C and three
points of the range example L.3. Each model is written as the example states
it, with M3003's distributions and degrees of freedom; Type A terms from an
earlier evaluation use `type_a_pooled`. Two variants show what the model
implies where the budget departs from it: K.3 with the mismatch term from
formula E5 (F57), and K.6 with the temperature terms of its model (F58).

**D47. Coverage factor for a dominant rectangular term (M3003 Appendix C).**
`analysis.gum.dominant_rectangular` takes the largest rectangular
contribution u_R, sets u_N = √(u_c² − u_R²) and solves
P(|R + N| ≤ k u_c) = p for the convolution of a rectangular and a normal
distribution. The distribution function uses ψ(z) = zΦ(z) + φ(z), and k is
found by bisection. All 26 entries of the C.7 table (p = 95.45 %) and the
values of C.8 (k = 1.77) and L.3.20 (k = 1.898) are reproduced to their
printed decimals. The Appendix B t-table is checked as well. In C.5 the Monte
Carlo interval agrees with the resulting y ± U (validated, n_dig = 2).

**F55. M3003 K.4.7 — degrees of freedom (class b).** "𝜐 = (𝑚 − 1) = 10" for
m = 10 comparisons; the budget uses 9.

**F56. M3003 K.5 — combined uncertainty and sign of the result (class c).**
The u_i column of K.5.9 (0.05, 0.058, 0.029, 0.029, 0.115, 0.05 mg)
combines to 0.153 mg, not the printed 0.150 mg. The printed value would
follow if the drift limit 0.1 mg were divided by 2 instead of √3. U is
therefore 0.31 mg, not 0.30 mg. By the model of K.5.1, the error of
indication is I_X − W_S = 199.9999 g − 200 g = −0.10 mg, while K.5.10
reports +0.10 mg. *Qualified 2026-11-08 (review item 16, which recomputed 0.153 and 0.150 mg independently):*
the sign part holds only if W_S is taken as exactly 200.0000 g; K.5 gives no certificate value of the standard.

**F57. M3003 K.3 — mismatch term (class c).** K.3.6 gives the coefficients
for formula E5 of E.9.5 (Γ_G = Γ_L = 0.03, s11a = s11b = 0.05, s22a = 0.05,
s22b = 0.01, s21a = 1, s21b = 0.31). E5 gives M = 0.0170 dB, but the budget
uses 0.022 dB. With 0.0170 dB, u_c = 0.0202 dB and U = 0.040 dB instead of
0.049 dB. Raising Γ_G and Γ_L to 0.036 by E.9.6 gives 0.0209 dB, still not
0.022 dB. EA-4/02 S7 (the same attenuator) uses 0.020 dB (library project
`ea402-s7-step-attenuator-30db`).

**F58. M3003 K.6 — temperature terms of the gauge-block model (class c).**
The model has the term −L(α·Δt + Δα·ΔT). K.6.4 evaluates both influences
through Δα: ±1.6 nm for Δt = ±0.08 °C at 20 nm/°C, and ±4 nm for
ΔT = ±0.2 °C. It then adds them linearly as "correlated" (±5.6 nm,
triangular). By the model, Δt acts through α: L·α = 115 nm/°C, so ±9.2 nm
(rectangular, u = 5.3 nm). The product Δα·ΔT has zero first-order
sensitivity and contributes only L·u(Δα)·u(ΔT) = 0.94 nm through the
second-order term. EA-4/02 S4 evaluates the same model in this way. The
effect is small here (u_c = 41.0 nm instead of 40.7 nm, U = 82 nm instead of
81.5 nm), but the reasoning would underestimate the Δt term for longer
blocks.

**F59. M3003 K.7 — EMF estimate and rounding (class a).** The EMF of the
unknown thermocouple is corrected by scaling, 36 248 × 1000/1000.5 =
36 230 µV. The uncertainty budget uses the sensitivity 1/c_X = 38.5 µV/°C,
which gives 36 248 − 0.5 × 38.46 = 36 228.8 µV. The printed u_c = 25.9 µV
and U = 51.9 µV follow from the rounded sensitivities 38.5 and 26.3; the
exact ones give 25.84 µV and 51.7 µV.

**F60. M3003 K.8 — distribution of the mass term (class b).** K.8.5.1 says
the 7.3 ppm relative mass uncertainty is the semi-range of a rectangular
distribution. The budget instead treats it as an expanded uncertainty with
divisor 2 (3.65 ppm instead of 4.21 ppm). u_c is 43.0 ppm either way.

**F61. M3003 — typographical slips (class b).**

- C.5 gives the drift and temperature limit as "±50 V" (it is 50 µV).
- The L.3 budget is headed "evaluated at ΔI_DMM = 0.95 V" (it means I_DMM).
- L.3.15 defines Q[a, b] = [a² + ab]^½, which should be [a² + b²]^½.
- The note in Appendix Q writes u(R) = u(R) × R, which should be w(R) × R.
- K.4.1 lists W_S twice.
- K.8.17 drops the ± in "34 Pa  170 Pa".
- K.9.12 reports T_s = 293 K, not 293.15 K.

None of these affects a result.

**Note on k = 2 (not a finding).** K.9 (ν_eff = 130) and K.10 (ν_eff = 50)
report U with k = 2. By Appendix B, k for 95.45 % would be 2.03 and 2.05,
giving 0.56 L/s and 0.020 mm instead of 0.55 L/s and 0.019 mm. Appendix B
asks for the t-factor when a Type A term is significant but sets no
threshold. The projects use k = 2 as printed and note the difference.

**D48. EURAMET cg-19 v4.1 (12/2025), gravimetric volume calibration.** The
guide is free from EURAMET; its version 4 was written with contributions from
UME. Its two numerical examples, a 1000 mL flask and a 100 mL pycnometer,
are library projects. The water density is computed from Tanaka's formula at
the water temperature (intermediate quantity). The water temperature then
enters both the density and the expansion of the instrument, with the
correlation the guide's budget leaves out; the effect is below the printed
digits. Both examples use k = t(ν_eff) for 95.45 % while the text says
"approximately 95 %"; the projects use p = 95.45 %.

**F62. EURAMET cg-19 v4.1 Examples 1 and 2 (classes a and b).**

- With the inputs stated in 7.1.1.4 and 7.2.1.4, the air-density formula
  gives u(ρ_A) = 6.0·10⁻⁷ and 4.8·10⁻⁷ g/mL, not the printed 1.76·10⁻⁶ and
  1.74·10⁻⁶ g/mL. The inputs are 0.16 °C, 6 Pa (60 in the formula of
  Example 1), 0.6 %rh and the formula uncertainty 2.4·10⁻⁴.
- u(ρ_B) = 0.06/2 = 0.03 g/mL in the text, but 0.0346 g/mL (= 0.06/√3) in
  Tables 2 and 4.
- u(ρ_W,t) is 1.024·10⁻⁶ g/mL in the text of 7.1.1.3 and 1.24·10⁻⁶ g/mL in
  its formula. The combined value 3.09·10⁻⁶ follows from 1.024·10⁻⁶, while
  β·ρ_W·u(t_W) is 8.9·10⁻⁷ g/mL.
- The sensitivity to γ is +890.66 °C·mL in 7.1.2.6 and −890.66 in Table 2.
- Table 4 gives ν_eff = 1727, whereas 7.2.5 gives 13. Welch–Satterthwaite
  gives 12.9; GUM G.6.4 truncates it to 12 (k = 2.23), and the example
  rounds it to 13 (k = 2.21). U is 2.0 µL either way.
- In Table 1, V₂₀ = 999.628 mL rests on unrounded densities. With Tanaka's
  formula and ρ_A = 0.0012 g/mL the model gives 999.622 mL. Example 2
  (100.014 mL) is reproduced.

None of these changes a printed u_c or U.

**D49. EURAMET cg-18 v4.0 (2015), Example H1.** Five library projects cover
the error of indication of a 220 g / 0.1 mg laboratory balance:

- at zero (ν = 4 from the repeatability test, so k = 2.87);
- at 220 g without buoyancy correction, with the conservative formula
  7.1.2-5d and with the ΔT-based formula 7.1.2-5e;
- at 100 g and 220 g with the buoyancy correction of option 2.

Each contribution is built from the guide's formulas rather than copied
from the budget: the repeatability s from the five readings; the
eccentricity from 7.1.1-10; the drift D = k_D·U(m_c); the buoyancy limits
from 7.1.2-5d and 5e with the E2 mpe. In option 2 the correction
−m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) is part of the model, so its uncertainty
(7.1.2-5a) follows from the propagation. Every printed u(E), ν_eff, k and U
is reproduced. One difference is practice: the guide reads k from GUM
Table G.2 (2.05 at ν = 62, from the row for ν = 50), where the exact
t-value is 2.04. Examples H2–H4 (multi-interval, 30 t with substitution
loads, error-approximation function) remain for a later session.

## 2026-10-26 (session 16) — Uncertainty arising from sampling (Eurachem UfS:2019)

**D50. `sampling` section (Eurachem UfS:2019).** ISO/IEC 17025 7.6.1 requires
contributions from sampling to be considered, and the chemistry focus of the
project made the Eurachem Guide the next document. The core module
`sampling` implements:

- the classical nested ANOVA of the duplicate method, for balanced and
  unbalanced designs (method of moments with the coefficients k₁, k₂, k₃ of
  the expected mean squares; 2, 2, 4 when balanced), with negative variance
  estimates set to zero;
- the range method of Example A3;
- the ANOVA of ln x with the uncertainty factor F_U = exp(k s_G) and
  u′ = √(exp(s_G²) − 1);
- the fundamental sampling error of Gy's theory (Equations 6–8).

Ten library projects cover Examples A1–A6, including a control chart of the
duplicate differences (A4, Section 7.3) and the two modelling budgets of A6
as GUM models. Every classical value printed in the Guide is reproduced: A1
(Figure A1.2), A2 (Figure A2.2 and Table A2.3, F_U = 2.6207), A3 (Table A3.5
and the corrected Table A3.7), A4 (Tables A4.5–A4.9), A5 and A6.

**F63. The robust ANOVA of RANOVA/RANOVA2 is not reproducible from the Guide
(class c, method).** The Guide reports robust estimates as the main results
of A1 and A2 but does not give the algorithm; reference [53] points to the
programs on the RSC website, which this session could not reach. Variants
tried on both examples:

- Huber's H15 at each level (c = 1.5, β = 0.7785) with the scale of each
  level combined from the levels below (the AMC one-way approach extended);
- Algorithm A on the analytical differences, the sample differences and
  the target means;
- H15 on all values.

Two partial results: H15 on the target means with divisor n reproduces the
robust means of both examples (4408.319 against 4408.3237; 297.310 against
297.31). The per-level variant reproduces s_anal of A1 (167.947 against
167.943) but not A2 (15.33 against 11.144). No variant reproduces s_samp or
s_T. The two examples also come from different programs (RANOVA in A1,
RANOVA2 in A2), and A3 used a third one (ROBAN 1.0.1). The program therefore
evaluates the classical ANOVA, which the Guide prints alongside, and says so
in every report. For the article, the point stands: an uncertainty method
whose numerical results depend on an unpublished algorithm cannot be
validated independently.

**F64. UfS Example A3 (class a/b).** The errata (version 1.1, 2020) correct
s_T+S+A in Table A3.7 from 0.55 to 0.604 mg/L, s_T from 0.548 to
0.601 mg/L, and the between-target U from 64 % to 70 %. The program
reproduces the corrected values from the raw data. The summary table still
gives 1.9 % for the analytical U of the validation, while Table A3.4 and the
range calculation give 1.8 % (2 × 0.89 %).

**F65. UfS Example A4 (class a/b).** Errata 1.4a (2023) correct the
between-batch RSD_B of the summary from 8.2 % to 6.1 %; the classical ANOVA
of Table A4.3 gives 6.11 %. In Section 7.3 the warning-limit factor is
printed as 2.38 but the computation uses 2.83, the D_WL of TR 569 for
duplicates. The summary's U′ = 16.6 % and 19.4 % are twice the rounded u
(8.3 % and 9.7 %); unrounded, U′_meas = 19.30 %.

**F66. UfS Example A5 and Section 10.2.8 (class b).** "Equation 6" labels
both the FSE formula and C = f g β c. Example A5 calls the constitution
factor Equation 7, while Section 10.2.8 numbers it Equation 8. The printed
total U′ = 28.6 % comes from the rounded components (3.3 %, 13 %, 5 %);
unrounded it is 28.53 %. The constitution factor is 2158.5 g cm⁻³ (printed
2160).

**F67. UfS Example A6 (class a/b).**

- Cadmium: Table A6.2 gives u_depth = 3.5 %, but its own data
  (x₋ = 0.339, x₊ = 0.299 mg/kg) give 3.6 %. The combined uncertainty is
  then 9.2 % (printed 9.1 %) and U = 18.4 % (printed 18.2 %); the rounded
  result 0.32 ± 0.06 mg/kg is unchanged.
- Phosphorus: Table A6.1 prints s_sqr = 8.0 mg/kg, while the five values give
  7.43 mg/kg. The printed 6.5 % and u_b-loc = 2.9 % follow from 7.43. The
  mean of the splitting RSDs in Table A6.3 is 3.16 %, not 3.3 %. The combined
  11.3 % and U = 26 mg/kg are unchanged.

## 2026-10-27 (session 17) — EURAMET cg-18 Examples H2–H4; error approximation function

**D51. Examples H2 and H3 as models.** As for H1 (D49), each term of the
budgets is an input of E = I − m_ref:

- **H2** (60 kg, multi-interval): six projects. The rounding at the load uses
  d of the interval that contains the load, and the repeatability the test in
  the same interval. The buoyancy follows 7.1.2-5d, -5e or -5c, or the
  correction is applied (option 2).
- **H3** (30 t weighbridge): four projects. Each substitution step
  L_sub = m_ref + I(L_sub) − I(m_ref) brings the indications used for the
  adjustment as inputs of their own (rounding, repeatability, eccentricity,
  creep), and the same standard weights enter every step. This reproduces
  u(L_sub1) = 11.28 kg and u(L_sub1 + L_sub2) = 19.02 kg of formula
  7.1.2-15b.

Every printed u(E) of the selected points is reproduced. The coverage factors
differ where the guide reads Table G.2 at a lower ν, as in H1.

**D52. `error_function` section (cg-18 Appendix C2.2).** The core module
`weighing` fits a polynomial E(I) (with or without a₀) by generalized least
squares. The covariance matrix is U(e) = s sᵀ + diag(u²(I)) + s_m²·I. The
module computes â, U(â), the residuals, χ²_obs with ν = n − n_par, both
criteria C2.2-2a (χ²_obs ≤ ν) and C2.2-2b (each residual within k·u(E_appr)),
u(E_appr) at the points and at readings (C2.2-11 to -14), and, by bisection,
the smallest s_m meeting each criterion. Example H4 is reproduced for
s_m = 0, 0.05 and 0.25 mg: a₁, U(â), χ²_obs, u(E_appr) of Tables H4.6 and
H4.8, and the two points (150 g, 250 g) failing C2.2-2b. Its data need
s_m = 0.044 mg for C2.2-2a and 0.22 mg for C2.2-2b; the guide's 0.05 and
0.25 mg are rounded-up choices.

**F68. cg-18 H2 — mpe of the 60 kg test load (class b).** The budgets use
u(m_c) = 0.554 g, u(m_D) = 0.277 g and u(m_B) = 0.658 g at 60 kg, i.e.
mpe = 0.96 g. The test load is three 20 kg F2 weights, mpe 3 × 0.30 g = 0.90 g
(OIML R111). The 10, 20 and 40 kg columns use the R111 values (0.16, 0.30,
0.60 g). The projects use 0.90 g: u(E) = 5.973 g instead of 5.978 g (A,
option 1), and 5.725 g instead of 5.739 g (B). The printed B value 5.739 g
does not follow from the table's own entries either: √(5.694² + 0.635²) =
5.729 g. Also, with mpe = 0.96 g formula 7.1.2-5e gives 0.278 and 0.180 g at
60 and 40 kg; the table prints 0.276 and 0.178 g.

**F69. cg-18 H3 — degrees of freedom with substitution and repeatability
(class c, method).**

- The ν_eff of the budget (109 at 15 t, 783 at 30 t) counts only the
  repeatability of the indication at the load. The repeatability terms of the
  indications that adjust the substitution loads (inside u(L_sub),
  7.1.2-15b) are treated as exact. Counting each with ν = 5 gives ν_eff = 37
  and 157 (k = 2.07 and 2.02; U = 30.3 kg instead of 29 kg at 15 t).
- Strictly, all these terms use the same s (ν = 5). Their variances then
  have to be summed before Welch–Satterthwaite, which gives ν_eff ≈ 12 and
  k = 2.23 at 15 t (U = 32.6 kg).
- The budget also uses s = 6.74 kg (test at 10 420 kg) at every load,
  although the test at 24 160 kg gave s = 9.03 kg. With 9.03 kg for the three
  indications at 20 and 30 t, u(E) at 30 t would be 26.0 kg instead of
  23.9 kg.
- Formula 7.1.3-1c adds the reference-mass uncertainty of the current
  weights and that inside u(L_sub) in quadrature, although they are the same
  weights. Adding them linearly gives 1.32 kg instead of 0.98 kg at 30 t, with
  no effect on u(E).

**F70. cg-18 H4 — tables of the error function (class b).**

- Table H4.1 lists e = 0.213, 0.274 and 0.181 mg at 100, 150 and 250 g.
  Tables H4.6 and H4.8 list the indications 100.000 200, 150.000 267 and
  250.000 200 g. The budget gives 0.113, 0.240 and 0.081 mg (100.000 100,
  150.000 233, 250.000 100 g). The printed a₁, U(â), χ²_obs and residuals
  follow from the budget values; with the H4.1 values a₁ would be
  0.000 91 mg/g and χ²_obs = 15.4.
- Table H4.8 prints the H4.1 errors next to residuals from the budget values.
- Table H4.2 is not symmetric: (8,2) = 1.077·10⁻⁴ against (2,8) =
  1.077·10⁻³, and (9,2) = 1.091·10⁻⁴ against (2,9) = 1.091·10⁻³.
- Table H4.5 prints "5,322·10⁻³³" for 5.322·10⁻³.
- The budget labels the drift row at 250–400 g "u(m_c)".
- The buoyancy contribution of 7.1.2-5b is printed as a negative variance.
  It is the removal of the part of u(ρ) already contained in the calibration
  value of the weights (calibrated at ρ_a1): (ρ_a − ρ₀)(ρ_a − ρ₀ − 2(ρ_a1 − ρ₀))
  = (ρ_a − ρ_a1)² − (ρ_a1 − ρ₀)². It is a covariance between m_c and the
  buoyancy term rather than a variance, and is best presented that way.

## 2026-10-27 (session 18) — Software tools: full texts and the MUPR spreadsheet

The author supplied the full texts of four papers on software tools
(Hall 2022, GTC; Lobsiger et al. 2022, METAS UncLib; Pennecchi & Harris
2023, Mathmet survey; Bettencourt da Silva 2026, MUPR spreadsheet) and the
nine electronic supplements of the last. They are used locally only. The
comparison and the gap analysis are in `literature-review.md` §3.

The spreadsheet workbooks are protected with the password given in the
paper. In the second copy received, supplements 01, 02 and 05–08 are
complete. Supplements 03 and 04 (soil pH) are truncated (no zip central
directory). Of 03, the sheet "Analysis sequence" can be read up to row 6 943,
which holds all the data. The sheets "Performance Model" and "Trueness" are
missing. Of 04, only two sheets are present.

**D53. Recomputation of Supplementary Material 06** (fluoranthene,
low spikes excluded). The values were read from the workbook and recomputed
independently:

- The weighted mean recovery R̿ = 0.748871 and χ² = 40.97 > χ²₀.₀₅,₁₉ =
  30.14 (Eq. 16.1–16.2) follow from the 20 mean recoveries and their
  uncertainties (sheet "Trueness", CF16:CG35).
- The recoveries are not compatible, so s(R_m) = 0.087552 is added:
  u(R̄) = √((√Σu²(R_k)/N)² + s²(R_m)) = 0.096291.
- The recovery differs from 1: t = 2.608 > 2.093 (ν = 19).
- For a single result of 10 mg/kg: c = 13.3534 mg/kg, u = 1.84285 mg/kg
  (precision 0.66932, trueness 1.71701) and U = 3.857 mg/kg with k = 2.093.
  This is the row "Sample 10 [1]" of the sheet "Analysis sequence".

The spreadsheet's ν is the minimum of the ν of the components (19), not
Welch–Satterthwaite. The program reproduces that with `coverage_dof`.

**F71. Bettencourt da Silva (2026), text against its supplement
(class b).**

- Section 3.2 reports (13.4 ± 3.9) mg/kg "for k = 2.01". The supplement gives
  k = 2.093 (ν = 19), and 3.9 follows only from that value:
  2.01 × 1.843 = 3.70 would round to 3.7.
- Section 3.1 gives s_r = 0.238 (ν = 56) and s_I = 0.0685 (ν = 82) for the pH
  example. That would make s_r larger than s_I by a factor of 3.5, while the
  text says that "no significant difference is observed" between them.
- Supplement 03 settles it. Every row of "Analysis sequence" uses
  s_r = 0.023773 and s_I = 0.068460 (columns BH, BI), so s_r = 0.0238 is
  meant.
- The precision term of a duplicate, √(s_I² − s_r²/2) = 0.066364, and
  k = 2.0032 for ν = 56 follow from these values.
- The values without the control standards (s_r = 0.229, s_I = 0.0573) are
  presumably 0.0229 and 0.0573. Supplement 04 is truncated, so this could not
  be checked.

**F72. Bettencourt da Silva (2026) — absolute precision of a
recovery-corrected result (class a).**

- For relative precision the spreadsheet uses s = s′·c at the corrected value
  c = x/R̄. That is the correct propagation.
- For absolute precision (interval I, s_I = 0.075185 mg/kg on the
  measured-value scale) it uses s_I itself, not s_I/R̄.
- At x = 0.1 mg/kg this gives (0.13 ± 0.16) mg/kg, the value printed in the
  paper. The propagation for y = x/R̄ gives u = √((0.075185/0.748871)² +
  0.01717²) = 0.10186 mg/kg and U = 0.21 mg/kg: a relative expanded
  uncertainty of 160 % instead of 121 %.
- This is the omission already found in MUPR Eq. (B6.8) (F17), here carried
  into the software. The program divides absolute components by R̄ (D18).

**F73. Bettencourt da Silva (2026) — mean recovery and its uncertainty
(class c, method).**

- R̄ is the weighted mean of Eq. (16.1), which the MUPR footnote to Eq. (8)
  allows. Its uncertainty, however, is the arithmetic-mean form of Eq. (12),
  √Σu²(R_k)/N = 0.04008.
- The uncertainty of the weighted mean itself would be
  (Σ1/u²(R_k))^(−1/2) = 0.00996.
- One test (SS95(SP1), spiked at 0.1 mg/kg, u = 0.751) contributes 0.0375 of
  the 0.0401 but has almost no weight in R̄. Without it √Σu²/N = 0.0141.
- With s(R_m) added, the three consistent choices give at 10 mg/kg:
  - spreadsheet (weighted R̄, Eq. 12 u): 13.35 mg/kg, U = 3.86 mg/kg;
  - Eq. (8)–(12) throughout (arithmetic R̄ = 0.76307, as in the program):
    13.10 mg/kg, U = 3.72 mg/kg;
  - weighted R̄ with its own uncertainty: 13.35 mg/kg, U = 3.57 mg/kg.
- The program had no weighted mean recovery; D54 adds it, with the
  uncertainty of the weighted mean itself.

**Implications for the article.**

- The closest tool in chemistry is a spreadsheet by the author of the
  guidance it implements. Recomputing one published example from its
  supplement found one text–software discrepancy and two departures from the
  guide's equations.
- This supports the article's argument that each implementation should be
  checked automatically against published examples, and that such checks
  should be repeatable by anyone.
- The Mathmet figures (54 % of tools with any validation evidence; 20 %, 6 %
  and 31 % with JCGM 102 support) quantify the gap in the wider landscape.

**D54. Weighted mean recovery and given recoveries.**

- `recovery.mean = "weighted"` combines the tests by R̿ = Σ(R_k/u²)/Σ(1/u²)
  (MUPR Eq. 16.1, allowed by the footnote to Eq. 8). The uncertainty is that
  of a weighted mean, u(R̿) = (Σ1/u²)^(−1/2).
- The spreadsheet's combination, R̿ with the Eq. (12) uncertainty (F73), is
  deliberately not offered: it pairs an estimator with the uncertainty of
  another.
- s(R), the significance test (ν = N − 1 with s(R)), the correction and the
  χ² test are unchanged. The default stays the arithmetic mean of
  Eq. (8)–(12), so existing projects give the same results.
- A new test type `given` takes a recovery and its standard uncertainty
  evaluated elsewhere (`recovery_percent`, `u_percent`), e.g. the mean
  recoveries of a validation spreadsheet.
- The ν-dependent weights of Mateos et al. (2020) (F46) remain outside the
  program.

Two library projects take the 20 recoveries of Supplementary Material 06:
`bettencourt2026-fluoranthene-weighted` and `-arithmetic`. Their expected
values are recomputed by an independent script. R̿ = 74.887 %, χ² = 40.973
and s(R) = 8.755 % agree with the workbook to all printed digits. The
expanded uncertainties at 0.1 and 10 mg/kg are:

| Variant | 0.1 mg/kg | 10 mg/kg |
|---|---|---|
| weighted | 0.213 | 3.575 |
| arithmetic | 0.209 | 3.724 |
| spreadsheet | 0.161 | 3.857 |

The report, the app's recovery form (mean: arithmetic or weighted; test type
"recovery with its uncertainty") and guide chapters 5 and 7 describe the
option. The library has 205 projects and the workspace 180 tests.

## 2026-10-27 (session 19) — Open-access papers: Suncal, the new definition, cg-18, conformity

Open-access papers on the reading list were downloaded where the publisher
serves the PDF directly:

- Possolo (2025);
- Fritsch (2019);
- Shirono et al. (2022, parts 1 and 2);
- from OSTI (public reports): the Suncal paper (Delker, *NCSLI Measure*
  13(3), 2021), the Suncal software-QA slides (NCSLI 2021) and the slides on
  guard bands under process bias (Delker & Peguero, NCSLI 2022).

Springer, De Gruyter, MDPI and Taylor & Francis did not return a PDF to a
plain request. Those papers are left for the author to download. The
comparison is in `literature-review.md` §2–3.

**Suncal (Delker 2021).**

- The closest competitor for calibration laboratories:
  - GUM solved symbolically, with MCM in parallel and the GUM validity
    check of DEM-ES-010;
  - several outputs with joint densities;
  - units;
  - ANOVA and autocorrelation of measured data;
  - curve fits with prediction bands;
  - decision risk with TUR-based and risk-based guard bands;
  - calibration intervals.
- Its software QA is a graded approach. Requirements are traced to
  automated black-box tests against GUM and TN 1900 examples, with the
  tolerance set by the published significant figures. This is the same
  policy as the library's `expected` blocks (D15).
- It does not cover chemistry top-down, PT, sampling or QC charts. It has no
  mobile version and is in English only.

**F28 confirmed independently.** Delker (2021, footnote 1) states that
TN 1900 "reported the wrong answer for both value and uncertainty" of the
airspeed example because the factor 2 was left out of the equation. That is
finding F28, made here from the equation and the printed values (v = 57.48
against 40.64 m/s, ratio √2). The Suncal result, (57.49 ± 0.67) m/s for
95 %, is close to the library project: 57.47 m/s, u = 0.348 m/s,
1.96u = 0.68 m/s, Monte Carlo interval [56.81, 58.17] m/s.

**F74. Delker (2021), airspeed example — inputs exchanged (class b).** The
text lists Δp = 14.7 psia ± 0.3 psia and p_a = 8.0 inches H₂O ± 0.1 inches
H₂O. In Kline & McClintock (1953) and TN 1900 E4 it is the other way round:
the pitot pressure difference is 8.0 in H₂O (1.993 kPa) and the atmospheric
pressure 14.7 psia (101.4 kPa). The printed result, 57.49 m/s, follows only
from the latter: √(2 · 1992.7 · 287.06 · 292.83/101 353) = 57.49 m/s. With
the inputs as printed, v would be 2 924 m/s, about 50 times larger.

**Possolo (2025).** The JCGM presented a new definition of measurement
uncertainty for public discussion (July 2025): "the doubt about the true value
of the measurand that remains after making a measurement", "described fully
and quantitatively by a probability distribution". It separates what
uncertainty is from how it is evaluated and expressed. The glossary (chapter
8, EN and TR) now notes it beside VIM 2.26.

**Gaps found, candidates for the next sessions.**

- *cg-18 §8, uncertainty of a weighing result in use* (Fritsch 2019):
  - U(W) = k√(α² + β²R²);
  - the global uncertainty U_gl = U(W) + |a₁|R ≈ α_gl + β_gl·R, with the
    error function of D52;
  - minimum weight m_min = U/TOL and, with a safety factor,
    m_min,SF = α·SF/(TOL − β·SF);
  - the safe weighing range.

  Examples H1 and H2 of cg-18 give the values needed to validate it.
- *TUR-based guard bands* (Suncal; Delker & Peguero 2022): the RSS guard
  band and Dobbert's managed guard band ("Method 6", PFA ≤ 2 % for any
  in-tolerance probability). The conformity module has the JCGM 106 guard
  bands and the guard band for a target consumer risk, but not these
  calibration-practice rules (ANSI/NCSL Z540.3).
- *Economic optimisation of the acceptance interval* (Shirono et al. 2022):
  a later extension of the conformity module, with the summary tables as
  test cases.

## 2026-10-27 (session 20) — Uncertainty of a weighing result in use and minimum weight (cg-18 7.4, 7.5, G)

**D55. `error_function.in_use`.** The core functions follow cg-18 7.4.1–7.4.4
for the terms of the uncertainty in use:

- the absolute term per interval, α_W² = d₀²/12 + d²/12 + s² (7.4.1-6);
- the relative terms:
  - u(a₁) of the fit (C2.2-16d);
  - temperature (7.4.3-1);
  - buoyancy (7.4.3-4 or -5);
  - adjustment (7.4.3-6);
  - tare from the slopes between the test points (7.4.4-4, -5);
  - creep and hysteresis (7.4.4-7, -9);
  - eccentricity taken in full (7.4.4-10).

They give u²(W) = α_W² + β_W²R² (7.4.5-2) and U(W) = k·u(W), with k = 2 by
default (7.5.1-2b). The linear forms per interval (7.5.2-3d, -3f) and the
global uncertainty U_gl(W) = U(W) + |a₁|R in its linear form (7.5.2-3a, -3e)
follow. The minimum weight is R_min = α_gl/(Req − β_gl) (G-7) and
R_min,SF = α_gl·SF/(Req − β_gl·SF) (G-9), searched interval by interval for
multi-interval instruments.

Design choices:

- The block needs the straight line through zero, E = a₁R, which 7.5.2-3e
  prescribes. With a constant term, u²(E_appr) is not of the form α² + β²R².
- The term a₁²u²(R) of C2.2-16d is neglected, as the guide does (footnotes
  10–12).
- A reading equal to Max_i belongs to interval i (Max_{i−1} < R ≤ Max_i).
- Only the uncertainty of the general weighing result W is given. W*
  (7.4.5-1a, conditions of the calibration) differs only in the eccentricity
  term and the environmental terms.

The report has a section with the intervals, the relative terms, the linear
forms and the minimum weight (EN and TR); the app shows the same. Four library
projects reproduce H1.4/A, H1.4/B, H2.4/A and H2.4/B from the test points and
the repeatability readings. Their generator recomputes every printed value
independently and requires agreement except for the two findings below.

The fits behind the examples were identified by recomputation:

- H1/A: the 7.1.2-5e budget, a₁ = 6.7096·10⁻⁶, u(a₁) = 1.2416·10⁻⁶.
- H1/B and H2/B: 7.1.2-5c.
- H2/A: the conservative 7.1.2-5d.

All four use uncorrelated weights p_j = 1/u²(E_j) (C2.2-18b). The text does not
say which budget each fit uses.

**F75. cg-18 H2.4/A — first-order U(W) of intervals 2 and 3 (class b).**

- The table prints U(W) = 10.190 g + 3.434·10⁻⁴(R − 12 000 g) and
  20.311 g + 3.923·10⁻⁴(R − 30 000 g).
- 10.190 and 20.311 g are the global uncertainties at 12 000 and 30 000 g,
  U(W) + |a₁|R: 8.129 + 2.060 and 15.158 + 5.151 g.
- U(W) itself is 2√(9.917 + 4.589·10⁻⁸·12 000²) = 8.129 g and 15.16 g.
- The slopes and the global lines are right.
- In H2.4/B, where a₁ = 0, both lines coincide and the printed 6.616 and
  11.951 g are right, which confirms the reading.

**F76. cg-18 H1.4/B — tare term (class b).**

- The errors 0, 0, −0.1, 0, −0.1 mg at 0, 50, 100, 150, 220 g give slopes
  q_E = 0, −2·10⁻⁶, +2·10⁻⁶ and −1.43·10⁻⁶.
- 7.4.4-5 then gives (q_E,max − q_E,min)/√12 = 4·10⁻⁶/√12 = 1.155·10⁻⁶. The
  guide prints 5.774·10⁻⁷ = 2·10⁻⁶/√12, as if max|q| − min|q| were taken.
- With the formula, β_W² = 7.432·10⁻¹² instead of 6.433·10⁻¹², the U_gl slope
  is 4.851·10⁻⁶ instead of 4.479·10⁻⁶, and the minimum weight 0.0728 g
  instead of 0.0727 g.
- In H1/A and H2/A, where all slopes have one sign, both readings agree with
  the printed values.

Minor points:

- H1/B and H2/A print a₁ without its sign (both are negative).
- H1/B prints the eccentricity term as 1.154·10⁻⁶; 0.0002/(100√3) =
  1.1547·10⁻⁶.
- H1/A and H1/B print α_W² as 1.467 and 1.466·10⁻⁸ for the same d and s;
  the value is 1.4663·10⁻⁸.

The library has 209 projects.

## 2026-10-27 (session 20, addendum) — Classes of F72, F75, F76; preserving the evidence

**Classes corrected.** Under D12, class (a) is intermediate rounding, (b) a slip
without effect on the result and (c) an inconsistency affecting a printed
result.

- **F72** is class **c**, not a: the printed (0.13 ± 0.16) mg/kg becomes
  (0.13 ± 0.21) mg/kg.
- **F75** is class **c**: the printed first-order U(W) of H2.4/A intervals 2
  and 3 is wrong, although the global lines and the minimum weight are right.
- **F76** is class **c**: β_W², the U_gl slope and the minimum weight of
  H1.4/B change slightly.

The class labels of all findings are to be audited against D12 before
submission. Some earlier entries use (a) for printed values that are not
rounding effects, e.g. F47–F49.

**Evidence kept in the repository** so that nothing needed for the article is
lost with the temporary working area:

- the raw data and scripts of the literature search
  (`docs/research/search-2026-10/`);
- the scripts behind D53/F71–F73, F63 and F70 (`tools/research/`);
- the library generators (`tools/generators/`);
- the provenance of every document read (`docs/research/sources-provenance.md`);
- a generated index of all findings and decisions
  (`docs/research/findings-index.md`).

Licensed documents and their text extracts remain outside the repository.

## 2026-11-07 (session 21) — New document folder; JCGM 100:2008/Amd.1:2026

The author moved all papers and reference documents to a new Google Drive
folder (92 files, `sources-provenance.md`). The previous folder is no longer
shared. The inventory and the order of use are in `drive-inventory-2026-11.md`.

**D56. Second-order term of the estimate (JCGM 100:2008/Amd.1:2026).**

- Amendment 1 of the GUM adds note 1 to 4.1.4: when the nonlinearity of f is
  significant, a Monte Carlo method can be applied, or higher-order Taylor
  terms "must be included in the expression for y". For independent normal
  inputs the next term is ½Σ ∂²f/∂xᵢ² u²(xᵢ). Equation (H.10) generalizes it
  to ½ΣΣ ∂²f/∂xᵢ∂xⱼ u(xᵢ, xⱼ).
- The amendment also rewrites H.1.7: the term is zero for the gauge block, so
  only the variance terms of the 5.1.2 note matter there.
- The program now always evaluates the term (`second_order_estimate_term`).
  With `higher_order` it is added to y and the interval is centred on the
  corrected estimate. Otherwise a warning `nonlinear_estimate` is given when
  the term exceeds u_c(y)/10.
- The threshold of a tenth of u_c is the program's choice: such a shift
  changes the second significant digit of a result reported with u to two
  digits.
- The variance terms of the 5.1.2 note stay restricted to independent inputs.
  The estimate term uses the correlations (H.10).

Effect on the library:

- In JCGM 101 9.4 (comparison loss δY = X₁² + X₂², higher-order terms on)
  the GUM estimate becomes 50, 150 and 2550·10⁻⁶ for x₁ = 0, 0.010 and 0.050.
  These are the analytic means, which MCM reproduces. Table 8 of JCGM 101:2008
  prints 0, 100 and 2500·10⁻⁶, which 9.4.2.2.8 itself calls invalid. The
  three projects now check the analytic estimates.
- GUM H.1: the term is 0 (core test), as Amd.1 H.1.7 states.
- EA-4/02 S4 and S13: the term is 0 (products of independent inputs).
- Without higher-order terms the warning now appears for:
  - NUM §3 ratio a/b·c (term 0.30 u_c);
  - TN 1900 E3, the falling-ball viscometer (0.125 u_c);
  - the correlated JCGM 101 9.4 cases (0.5 and 0.1 u_c).

  Their printed results are first-order, so the expected values do not
  change.

**D57. Higher-order terms for correlated inputs.**

- JCGM 100:2008 gives the variance terms of the 5.1.2 note for independent
  inputs only. The program refused `higher_order` with correlations, and
  GUM-5 Example 16 (16.4.1.1) does not apply GUF2 to the correlated
  reflection coefficients for the same reason, while noting that the
  calculation is possible [85].
- For normal inputs with covariance matrix Σ the expansion to third order
  gives u² = cᵀΣc + ½tr((HΣ)²) + Σ c_l f_ijk Σ_li Σ_jk (Mana & Pennecchi,
  *Metrologia* 44 (2007) 246). For independent inputs this is the 5.1.2
  note.
- The program now uses it. For the comparison loss with r = 0.9 it gives
  u = 67.27, 120.52 and 504.50·10⁻⁶. These are exactly the analytic values
  of column A of GUM-5 Table 16.2 (67, 121, 505) and agree with MCM (67, 121,
  504). Three new projects
  `jcgm101-9.4-comparison-loss-x0n0-correlated-higher-order` check them.

**GUM-5:2026, Examples 7 and 16** reproduce here without change:

- **Example 7**, mass calibration, Table 7.2: GUF1 1.2340, 0.0539,
  [1.1285, 1.3395] mg; GUF2 0.0750, [1.0870, 1.3810] mg; MCM 0.0755 mg; the
  estimate term is 0.
- **Example 16**, Table 16.1: GUF2 estimates 50/150/2550·10⁻⁶, as with D56,
  and the GUF2 intervals [−48, 148], [−70, 370], [1566, 3534]. The program
  gives [−48, 148], [−69, 369], [1565, 3535]; the guide rounds u to
  112·10⁻⁶ and 502·10⁻⁶ before multiplying by 1.96.
- GUM-5 calls the use of the first-order estimate with the 5.1.2 variance
  terms "inconsistent working" (7.3.2). It states that the effect of
  higher-order terms on the estimate "is not taken into account in
  JCGM 100:2008" (16.3.1.5), which Amd.1 corrects.

GUM-5 is added as a source of the JCGM 101 9.3 and 9.4 projects. The library
has 212 projects.

**GUM-5:2026, new library projects.** `tools/generators/gum5.py` writes the
examples not yet in the library. It recomputes each printed value and asserts
the agreement:

- Example 3 (molar mass of glucose): M_r = 180.1557, u = 0.0038.
- Example 14 (²⁰⁷Bi decay): A₁ = 10.810 kBq, u = 0.054 kBq, and Table 14.1
  of the linearization check to ±0.0006 kBq.
- Example 15 (wire rope): F₀ = 10 005.7 kN, u = 9.9 kN, U = 19.4 kN.
- Example 1 (pH):
  - two-point interpolation at 25 °C: 7.0002, u = 0.0041;
  - fully correlated standards: u = 0.0051;
  - temperature interpolation: two variants, see F78.

Examples 6, 8 and 13 equal JCGM 101 9.2 and 9.5 and JCGM 102 9.5, which are
in the library; GUM-5 is added as their source. The three projects
`jcgm101-9.4-comparison-loss-x0n0-correlated` said that second-order terms
are not available for correlated inputs; the sentence now refers to D57.

**F77. GUM-5 Example 15 — Monte Carlo standard deviation (class c).**

- 15.3.1 assigns to F a t-distribution with mean 10 005.7 kN, scale
  u(F̄) = s/√6 = 0.7 kN and ν = 5. For this model the MCM standard deviation
  is then √(u_c² − u²(F̄) + u²(F̄)·ν/(ν − 2)) = 9.93 kN. The program's MCM
  gives 9.93 kN (10⁶ trials).
- 15.5.2 prints 10.1 kN and the interval [9986.0, 10 025.3] kN, half-width
  19.6 kN.
- 10.14 kN results if the scale is the standard deviation of one test,
  s = 1.75 kN, instead of that of the mean.
- The conclusion (GUF valid) is unaffected. The difference 9.9 against
  10.1 kN that 15.6.1 attributes to the shape of the PDF is, however, this
  input inconsistency.

**F78. GUM-5 Example 1 — pH values and uncertainties (class c).**

- 1.9.1 obtains pH_X,T1 = 7.0002, pH_X,T2 = 7.0161 and pH_X = 7.0043 by the
  third interpolation, which the program reproduces. It then gives
  "pH_X = 7.0109, u(pH_X) = 0.0041"; 7.0109 does not follow from the data.
  1.10.3.5 repeats 7.0109 for the result of the 25 °C data of 1.8, which
  give 7.0002.
- For the temperature case 1.7.1 assumes all inputs independent. That gives
  u = 0.0032: the interpolation in temperature averages the independent
  certified values at 20 and 25 °C. The printed 0.0041 results when the
  values of each standard at the two temperatures are fully correlated
  (0.0041). Both variants are library projects.
- 1.10.2.2 states that with the uncertainties of the pH standards set to
  zero u(pH_X) becomes 0.0040, and that repeating the potential measurements
  would reduce u. The reverse holds. Without the standards' uncertainties
  u = 0.0006, and without the potentials' uncertainties u = 0.0040. The
  standards contribute 0.0038 and 0.0013 of the 0.0041 (1.10.3.5 lists
  these contributions itself).


## 2026-11-07 (session 21, continued) — GUM-5 Example 2; ensembles in ν_eff

**GUM-5:2026 Example 2 (benzo[a]pyrene, internal standard).** The budget of
Table 2.2 reproduces: m_E = 0.014 03 ng, u² = 4.08·10⁻⁶ ng² with covariances
and 3.14·10⁻⁶ ng² without, r(f, m_ISE) = −0.54, r(A_E, A_ISE) = −0.48 and
ν_eff = 5.20 by G.2b. GUM-5 eq. (2.3) as printed,
Σ_i Σ_j r_ij² c_i² c_j² u_i² u_j²/ν_i, gives 4.81. The MCM of 2.4.2 also
reproduces (`tools/research/gum5/ex2_mcm.py`): 9.32·10⁶ of 10⁷ draws kept,
median 0.0142 ng, shortest 95 % interval [1·10⁻⁷, 0.0321] ng against the
printed [≈ 0, 0.032] ng.

**D58. Ensembles in the Welch–Satterthwaite formula.**

- A model may declare `ensembles`: groups of inputs evaluated from the same
  data with a common finite ν. Each ensemble is one component of the G.2b
  denominator, with variance Σ_{i,j∈E} c_i c_j u(x_i, x_j) and that ν
  (Willink, *Metrologia* 44 (2007) 340, §4.1.1, "Result 2"; GTC 1.5.1
  `real_ensemble` and `welch_satterthwaite`, read in the PyPI package).
- For a single ensemble the result is exactly ν whatever the correlation, as
  it must be: the sample covariance matrix of paired means has a Wishart
  distribution, so cᵀSc ~ cᵀΣc·χ²_ν/ν. A core test checks this for
  r = −0.9 … 0.95.
- The warning that G.2b is not valid for correlated inputs is now raised per
  pair: only for a correlation between inputs that are not both of infinite
  ν and not in a common ensemble (GTC does the same). Before, any correlation
  in a model with a finite ν raised it.
- The members of an ensemble must have the same finite ν; the model is
  rejected otherwise. The report lists each ensemble below ν_eff.

**F79. GUM-5 Example 2 — ν_eff for correlated inputs (class c).**

- 2.4.1.3 uses eq. (2.3), attributed to Willink (2007), and obtains
  ν_eff,r = 4.8, k = 2.6, U = 0.0052 ng. The program reproduces 4.81 with
  that formula.
- A_E and A_ISE are the means of the same three paired injections (2.3.4)
  and their covariance is estimated from the same data, so they form an
  ensemble with ν = 2 in Willink's sense. Their joint component is
  3.97·10⁻⁶ ng², 97 % of u². ν_eff is then 2.11, k = 4.10 and
  U = 0.0083 ng.
- Eq. (2.3) cannot be general for such inputs. For one ensemble of two
  components with equal shares and r = 0.5 it gives 3.6ν, although the
  exact value is ν. The asymmetric form, with ν_i but not ν_j, also
  matters: with independent variance estimates and known r, the delta
  method gives Σ_i (c_i u_i Σ_j r_ij c_j u_j)²/ν_i, which here is 3.35.
- The printed interval [0.009, 0.019] ng is 3.2 times narrower than the
  MCM interval. GUM-5 attributes the difference to the t-distribution with
  ν = 1 and the truncation (2.6.2). With the ensemble ν the GUF interval
  becomes [0.0057, 0.0223] ng. Part of the gap is the ν_eff, not only the
  shape.
- To check before submission: the text of Willink (2007), §§3–4. GTC is
  the only reading here, and the paper may state eq. (2.3) for another
  case, e.g. independent variance estimates.
- *Coverage (2026-11-08, review item 1; `tools/research/review/f79_coverage.py`, 4 × 10⁵ simulated data sets
  of three paired injections with the printed means, standard deviations and r).* With ν fixed at the values of
  the printed budget, the GUF interval covers 89.3 % with ν = 4.81 (eq. 2.3) and 96.0 % with ν = 2.11 (ensemble),
  as the reviewer found. When ν is recomputed from each data set, as a laboratory would, the coverage is 94.2 %
  with eq. (2.3) (median ν 4.5, quartiles 2.6–6.6), 93.4 % with the ensemble rule (median 2.15) and 93.6 % with
  G.2b; the linearised model gives the same, so the shortfall is that of the Welch–Satterthwaite approximation
  with one dominant component of two degrees of freedom, not of the nonlinearity. The case for the ensemble
  rule is its derivation (the block c^T S c has exactly ν = 2), not a better coverage. F79 is therefore
  reclassified as a method difference (m, open until Willink 2007 is read), not as a class-c inconsistency.
  The per-injection cross-check the reviewer proposed (m_E for each injection, s/√3) needs the three pairs of
  areas, which GUM-5 does not print.
- Wording: "a strong linear relationship was observed between the areas"
  (2.3.4) refers to r = −0.48 from three pairs. A negative correlation
  between analyte and internal-standard areas of the same injection is
  unusual, since the internal standard is meant to cancel injection
  variation. This is not an error, only noted.

Library projects:

- `gum5-e2-benzo-a-pyrene`: G.2b as printed, ν_eff = 5.2, k = 2.5.
- `gum5-e2-benzo-a-pyrene-ensemble`: ν_eff = 2.11.
- Both have no MCM, since there is no joint t-distribution for a scalar
  model. That is a candidate extension (see status.md).

The library has 221 projects.

**GUM-5:2026 Example 10 (conformity of suspended particulate matter, lognormal prior).**

The prior is lognormal with µ = −2.325 and σ = 0.434 for ln ρ. Its quantiles
0.042 and 0.229 mg/m³ match Figure 10.1. T_U = 0.2 mg/m³ and the relative
uncertainty is 7 %.

**D59. Lognormal process PDF, relative uncertainty in global risks, posterior p_c.**

- `process.distribution = "lognormal"` with `meanlog` and `sdlog`.
- With `u_relative`, global risks use a measuring-system PDF h(η_m | η)
  that is normal with standard deviation u_rel·|η|, proportional to the true
  value. That is a PDF of η_m for every η, as JCGM 106 9.5.1 requires.
  Before, the program refused global risks with `u_relative`. A required
  consumer's risk (`target_consumer_risk`) still needs a fixed u.
- `posterior: true` (conformity project with a process) computes p_c from
  the posterior g(η | η_m) ∝ g₀(η)·h(η_m | η), with the process PDF as
  prior and a normal likelihood of standard deviation u. That u is
  u_rel·η_m, the value at the result (GUM-5 10.6.4.3; JCGM 106 6.2,
  Annex A).
- A test checks that a flat prior gives the normal p_c back.
- The app offers the lognormal PDF and the posterior switch.

**F80. GUM-5 Example 10 — joint PDF and statements on specific risks
(modelling; class to be decided in the D12 audit).**

1. **Joint PDF.** The R code of 10.6.3.1 writes the joint PDF as
   g₀(ρ)·N(ρ̂; ρ, (0.07ρ̂)²): the standard deviation depends on the measured
   value, not on the true value. That is not a conditional PDF of ρ̂ given
   ρ, and its integral is 1.005, hence the constant C.
   - This form (`tools/research/gum5/ex10_risks.py`, mode `hat`) reproduces
     every printed global risk. A_U = T_U: 0.498 % and 0.909 % (printed
     0.5 % and 0.9 %). A_U = 0.17: 0.0020 % and 5.78 % (0.002 %, 5.8 %).
     A_U = 0.22: 1.69 % and 0.115 % (1.7 %, "smaller than 0.1 %").
   - With the conditional PDF, standard deviation 0.07ρ, the risks are
     0.579 % and 0.739 %; 0.0103 % and 5.37 %; 1.80 % and 0.055 %. The
     library projects use these and quote the printed ones.
   - The two forms differ by up to a factor 5 for the small consumer's risk
     at A_U = 0.17. The statement "A_U ≤ 0.17 mg/m³ for a consumer's risk of
     0.002 %" therefore depends on this choice.
2. **"At least 0.22 for a producer's risk smaller than 0.1 %"** (10.6.3.4).
   With the printed form R_P(0.22) = 0.115 %; 0.1 % needs A_U ≈ 0.223. This
   is rounding of A_U (class a).
3. **Specific risks.** The likelihood with u = 0.07·ρ̂ is the usual one and
   the program reproduces it: producer's risk 11.33 % at ρ̂ = 0.225 (printed
   "about 11 %") and consumer's risk 14.26 % at 0.190.
   - 10.6.4 adds that the posterior is "essentially determined by the
     likelihood", so that frequentist risks "would be very close". Near T_U
     the normal PDF alone gives 22.6 % and 5.6 %. The limit lies in the
     upper tail of the prior, where it falls by a factor of about 3 over
     0.19–0.23 mg/m³.
   - The sentence holds for the body of the distribution, not for the
     results the figure is about.

Library projects:

- `gum5-e10-tspm-global-risks-shared-risk`, `-acceptance-0.17` and
  `-acceptance-0.22`;
- `gum5-e10-tspm-specific-risks` (posterior).

The library has 225 projects.

**GUM-5:2026 Example 12 (between-bottle homogeneity, Bayesian hierarchical model).**
Table 12.2 reproduces: SS 2.92·10⁻⁶ and 1.96·10⁻⁵, MS 3.25·10⁻⁷ and
4.90·10⁻⁷ (cmol/mol)². MS_between < MS_within.

**D60. Homogeneity without σ_pt; u*_bb; Bayesian between-unit standard deviation.**

- The homogeneity section no longer needs σ_pt. Without it the ISO 13528
  criteria are not evaluated, as for a reference material study.
- MS_between, MS_within and u*_bb = √(MS_within/m)·(2/ν_within)^¼
  (Linsinger et al. 2001; ISO Guide 35) are always given. Here
  u*_bb = 1.481·10⁻⁴ cmol/mol (0.035 %).
- `homogeneity.bayes` evaluates the model of GUM-5 (12.1)–(12.4), with
  half-Cauchy priors for τ and σ and a normal prior for µ. B_i and µ are
  integrated analytically: the group means are N(µ, τ² + σ²/m), the
  within-group sum of squares is σ²χ²_{g(m−1)}, and µ is normal-conjugate.
  The posterior of (τ, σ) is then tabulated on a 3001 × 1001 grid, with
  quadratic spacing in τ and logarithmic spacing in σ, so no MCMC is needed.
- Comparison with Stan (12.4.5):

  | | program | Stan |
  |---|---|---|
  | τ mean | 1.356·10⁻⁴ | 0.00014 |
  | τ s.d. | 1.07·10⁻⁴ | 0.00011 |
  | τ 2.5 % | 5.2·10⁻⁶ | 0.00001 |
  | τ 97.5 % | 3.97·10⁻⁴ | 0.00040 |
  | σ mean | 6.885·10⁻⁴ | 0.00069 |
  | σ s.d. | 7.2·10⁻⁵ | 0.00007 |
  | σ 95 % interval | [5.64, 8.46]·10⁻⁴ | [0.00056, 0.00085] |
  | µ | 0.425144 | 0.42514 |

- Table 12.3 HPD: τ/µ [0, 0.0008], σ/µ [0.0013, 0.0020]. The median of τ is
  1.13 µmol/mol (printed 1.1). An independent numpy grid gives the same
  values (`tools/research/gum5/ex12_posterior.py`).

**F81. GUM-5 Example 12 — mode of τ and likelihood (12.5) (classes c and b).**

1. **Mode of τ (class c).** 12.4.8 prints "the mode of the posterior of τ
   is 0.3 µmol mol⁻¹". The marginal posterior density of τ is largest at
   τ = 0. MS_between < MS_within, so the likelihood decreases in τ, and the
   half-Cauchy prior is largest at 0. The function `getmode` of 12.5.2
   returns the most frequent exact value of the draws. For continuous MCMC
   output all draws are distinct, so it returns the first draw, and
   0.3 µmol/mol is an arbitrary draw. 12.5.2's remark that "even using τ = 0
   could be justified" agrees with mode 0. Mean and median are not affected.
2. **Likelihood (class b).** (12.5) writes y_ij | µ, τ, σ ~ N(µ, τ² + σ²),
   as if the 50 values were independent. The replicates of a bottle share
   B_i and are correlated: Cov(y_ij, y_ik) = τ². The Stan code (12.4.2) is
   hierarchical and correct, so the results are not affected.
3. **Note for the article.** The Bayesian mean τ = 1.36 µmol/mol is close to
   the classical u*_bb = 1.48 µmol/mol, which GUM-5 does not mention. Where
   "the classical analysis does not provide a solution" (12.1.3), the
   standard RM practice does give one.

Library project: `gum5-e12-between-bottle-homogeneity-bayes`. The library
has 226 projects.

**GUM-5:2026 Example 4 (gravimetric CO₂ in N₂ mixture, ISO 6142-1, multivariate LPU).**

- The library project `gum5-e4-gravimetric-gas-mixture` is a `multivariate`
  model with 25 inputs: 3 weighings, 5 atomic weights and 12 impurity
  fractions. Its 15 intermediates are the molar masses, the major
  components 1 − Σ impurities, MP₁, MP₂, m₁, m₂ and n_tot, and its outputs
  are the 8 amount fractions.
- The correlations of 4.3.1 (masses through r₁, molar masses through
  shared elements, mass balance of the parent gases) follow from the
  shared inputs and are not entered.
- The correlation matrix of the molar masses (4.3.4.6) reproduces exactly
  (0.825, 0.707, 0.960, …).
- A numpy check with central differences agrees
  (`tools/research/gum5/ex4_composition.py`).

**F82. GUM-5 Example 4 — Tables 4.2 and 4.6 (classes c and b).**

1. **Table 4.2 does not follow from Table 4.1.**
   - The averages of the seven differences are −152.0247, 93.4951 and
     717.2743 g; printed r̂ are −152.0243, 93.4951 and 717.2740 g.
   - The standard deviations of the means are 0.18, 0.40 and 0.42 mg;
     printed u_rpt are 0.4, 0.7 and 0.6 mg, which are not s either (0.49,
     1.07, 1.11 mg).
   - u(r̂) = √(u_rpt² + u_res²) is consistent within Table 4.2. Perhaps
     Table 4.1 is a rounded excerpt of other data.
   - Effect: with Table 4.1, m₁ is 0.5 mg larger. Table 4.2 is used, as in
     the guide.
2. **u(y_CO₂) (class c).** Recomputed 3.96 µmol/mol, which rounds to 4;
   printed 5 µmol/mol and u_rel 0.003 % (recomputed 0.0020 %).
   - The budget is dominated by the atomic weights of N (2.81), C (2.10) and
     O (1.56), then r₁ (0.73) and O₂ in the CO₂ gas (0.41).
   - u(y_N₂) = 4.72, printed 5, agrees with rounding.
   - Treating the major components as independent inputs does not change
     u(y_CO₂), because y is invariant to the normalization.
3. **y_H₂O (class b).** 0.2003 × 1.9 + 0.7996 × 0.010 = 0.389 µmol/mol;
   printed 0.379. u = 0.021 agrees, and u_rel 5 % holds for both values.
   This is likely a transcription slip.
4. **Correlations.** r(CO₂, N₂) is −0.854 (printed −0.86) and r(Ar, N₂)
   −0.552 (printed −0.50); also r(Ar, CO₂) 0.051 (0.04) and r(CO₂, O₂)
   −0.103 (−0.08).
   - The larger printed u(y_CO₂) cannot explain these; the source of the
     differences is not found.
   - The other five components, all estimates except H₂O, and the
     molar-mass correlations agree.

The library has 227 projects.

**GUM-5:2026 Example 9 (torque sensor, OLS, WLS and Bayesian slope).**

- Table 9.3 reproduces: OLS 1.01065 ± 0.00152 with ν_eff = 2.72 and
  [1.0055, 1.0158]; WLS 1.00849 ± 0.00077 and [1.0069, 1.0101].
- The Bayesian posterior (9.17) was evaluated by quadrature:
  - NI-Bayes 1.00921 ± 0.00106, [1.00701, 1.01121];
  - Inf-Bayes 1.00909 ± 0.00087, [1.00731, 1.01076].
  - Script: `tools/research/gum5/ex9_bayes.py`.
- Both estimators are linear in the means Y_i, so the library projects
  `gum5-e9-torque-slope-ols` and `-wls` are ordinary model projects.
  Their coefficients are written into the expression, with ν_i = n_i − 1
  on the inputs.
- The Bayesian slope (a product of scaled t-densities and a normal prior)
  is not a program feature. It is a candidate for a "calibration with
  unknown per-level variances" option.

**F83. GUM-5 Example 9 — ν_eff of WLS (class a).** 9.5.1.4 gives
ν_eff = 14 for WLS. G.2b gives 13.31 with the unrounded u(y_i), and 13.17
with those of Table 9.2. The interval at four decimals is the same.

The library has 229 projects.

**GUM-5:2026 Example 5 (GHG inventory, mobile machinery in agriculture).**

- The relative expanded uncertainties (95 %) of Table 5.1 are converted with
  k = 1.96. That reproduces Table 5.3 to four decimals (833.1844, 2.0329,
  57.3860, 1.2198, 0.0334, 0.0074 kt) and u = 835.16 kt.
- With shared activity data the increase of u² by (5.9) is 0.0345 kt², so
  u_cor = 835.161 71 against 835.161 69 kt.
- Library projects: `gum5-e5-ghg-inventory-agriculture` and
  `-shared-activity` (A₂ = A₃, A₅ = A₆ as one input). Both include an MCM
  check.

**F84. GUM-5 Example 5 — estimate, notation and data (classes a and b).**

1. **Estimate (class a).** Table 5.2 sums rounded contributions to
   4282.0409 kt. From Table 5.1 the model gives 4282.144 kt, because
   F₁ = 2.0438·10⁻² is rounded: E₁ = 1150.9806 kt corresponds to
   F₁ = 2.043 75·10⁻². u is unaffected at the printed digits.
2. **Notation (class b).** 5.4.14 defines "u_rel(A) = u_F/A"; it should be
   u_A/A.
3. **Data (noted, not an error).**
   - Table 5.1 also gives A₁ the same value as A₂ and A₃ (56 317.0920 TJ of
     gas oil), but with U_rel = 38.6 % instead of 1.6 %.
   - If the three lines share one activity, the CO₂ line carries an extra
     uncertainty that is not the activity's. The guide discusses only
     A₂ = A₃.
   - 5.4.5 quotes the inventory's U = 1635 kt; 1.96 × 835.16 = 1637 kt.

The library has 231 projects.

**GUM-5:2026 Example 11 (tumour section area from boundary pixels).**

- **Data.** The pixel coordinates are not tabulated in the guide. The 38
  boundary pixels were read from the grey cells of Figure 11.1 (left),
  rendered at 200 dpi. They match the text: 38 boundary pixels and 47
  interior pixels give the extreme areas 47 and 85 of 11.5.1, and the
  crude u = (85 − 47)/√12 = 10.97 ≈ 11.
- **Areas.** The parametric Fourier series fitted with t = cumulative chord
  length, normalized with the closing chord included, reproduce the areas
  of Table 11.1 to ±0.1 for n = 1–8: 65.12, 63.99, 64.15, 65.72, 65.72,
  65.27, 65.23, 65.38. Normalizing by t_m as written in 11.3.8 maps the
  last and the first point to the same t and is slightly worse.
- **MCM (10⁵ trials).** The 11.5.4 Monte Carlo for n = 6 gives 65.45 ± 1.55
  (printed 65.4 ± 1.6).
- Script: `tools/research/gum5/ex11_fourier_area.py`. No library project:
  the model is a least-squares fit inside the Monte Carlo loop, which the
  project format cannot express.

**F85. GUM-5 Example 11 — results, information criteria and text
(classes c and b; the numerical points rest on data read from the figure).**

1. **Reported result (class c).** 11.6.1 reports â = 65.0, u = 1.6 for the
   degree-6 MCM. 11.5.5 gives 65.4, and the recomputation gives 65.45.
2. **Neighbouring degrees (11.6.2).** Printed: n = 5 gives 65.3 ± 1.5 and
   n = 7 gives 65.0 ± 1.5. Recomputed: 65.93 ± 1.51 and 65.40 ± 1.57.
   - The MCM means lie 0.17–0.21 above the point estimates of Table 11.1
     for every degree.
   - A mean of 65.3 for n = 5 (point estimate 65.7) would be a shift of the
     opposite sign.
3. **Information criteria (class c).**
   - The printed AIC column equals exactly half of AIC = χ² + 2(4n + 2)
     when χ² is taken from the printed RMSR by (11.5), χ² = RMSR²(2m − 4n −
     2): 38.3, 23.1, 25.2, 23.4, 27.0, 29.8, 33.7, 37.2 against 38, 23, 25,
     23, 27, 30, 33, 37. The printed RMSR are about √2 times
     √(Σres²/(2m − 4n − 2)), i.e. they use half the degrees of freedom.
   - The printed BIC and AICc follow from neither form.
   - With the formulas as stated and the figure data, AIC is minimal at
     n = 6 and AICc and BIC at n = 4, not n = 2.
   - 11.4.4 ("curiously, the three information criteria all exhibit minimum
     values for n = 2") and the conclusion of 11.7.3 that RMSR saturation
     is better than the information criteria therefore rest on the
     computation, not on the data. Even with the printed numbers, AIC at
     n = 2 and n = 4 differs by 0.3.
   - To confirm with the original coordinates (compendium EMUE [1]).
4. **Text (class b).**
   - 11.3.2 gives 224 and 1840 pixels; the caption of Figure 11.1 gives 85
     and 585.
   - 11.3.8 takes the 38 midpoints from "figure 11.1 (right panel)",
     whereas 11.4.1 uses the left panel.
   - 11.6.1 refers to "figure 11.1 (centre)"; there is no centre panel.
   - 11.7.4 calls the 113 fine pixels "of dimension 0.35 × 0.35" while
     "taking each pixel as unit size".

All 16 GUM-5 examples have now been worked through: 13 as library
projects, Example 11 by script.

## 2026-11-07 (session 21, continued) — JCGM-WG1 News, December 2025: proposed definitions

The WG1–WG2 joint meetings discussed definitions for the next VIM:
measurement uncertainty, standard uncertainty, coverage interval, coverage
probability, expanded uncertainty, coverage factor, and relative standard
and relative expanded uncertainty. The July 2025 webinar had about 900
attendees; two thirds of 450 respondents were in favour.

- **Measurement uncertainty.** "Doubt about the value of the measurand that
  remains after making a measurement." The Possolo (2025) text said "true
  value". Note 1 now says "can be described" by a probability distribution.
- **Expanded uncertainty.** "Half width of a coverage interval symmetric
  about the measured value". VIM 2.35 defines it as k·u; k·u becomes a way
  to obtain it.

**D61. Monte Carlo: half-width of the interval symmetric about the
estimate.**

- `McmResult.expanded_about_estimate` is the q-th smallest |y_r − ỹ|, with
  q as for the coverage intervals, so that Pr(|Y − ỹ| ≤ U) = p. It is
  reported, with U/u, next to the probabilistically symmetric and the
  shortest interval, and is also given in JSON and in the app.
- Tests:
  - symmetric output: equals the half-width of the probabilistically
    symmetric interval (17.0 for JCGM 101 9.2.4);
  - Y = X², X ~ N(1, 0.5²): U = 2.0712 against the exact value by
    quadrature, and differs from the half-width of the probabilistically
    symmetric interval.
- For a skewed output the three intervals differ, and under the proposed
  definition only the new one is "the" expanded uncertainty. Worth a
  sentence in the article. GUM-5 Examples 2 and 10 report U = k·u next to
  asymmetric MCM intervals.
- Glossary (EN/TR) extended with the table of proposed definitions and
  their Turkish wording.

## 2026-11-07 (session 21, continued) — Eurachem/CITAC compliance assessment guide (2021)

**D62. Lognormal PDF for the measurand in conformity assessment.**

- `conformity.distribution = "lognormal"`: median equal to the measured
  value, s_G = u/|y| (Equation 2 of Annex A, Case 4), or
  s_G = √ln(1 + (u/y)²) with `exact_sdlog`.
- It is allowed in conformity projects of their own and in model projects,
  but not together with `dof` or `posterior`. The report states the PDF.
- A core test reproduces Annex A Table 1, with lognormal limits 61/164 and
  44/227 for u_rel = 0.3 and 0.5, and the Example B3 limit of 3.557 ng/g.

Library (`tools/generators/eurachem_compliance.py`, 5 projects):

- B1, nickel in steel: acceptance zone 16.164–17.836 % Ni; the batch at
  16.1 % is non-compliant, p_c = 84.1 %.
- B2, upper limit with the t-distribution, ν = 8: acceptance limit 204.09
  ng/g; the batch at 203.7 ng/g is compliant, exceedance probability
  93.4 %.
- B3, banned substance, u_rel = 35 %, in three variants:
  - lognormal, 3.557 ng/g;
  - lognormal with the exact s_G, 3.498 ng/g;
  - normal with u = u_rel·L = 0.7 ng/g, 3.151 ng/g.
  - The sample at 3.3 ng/g is compliant only under the lognormal PDF.

**F86. Eurachem/CITAC compliance guide 2021 — rounding and the normal
limits for a relative uncertainty (classes a, b and noted).**

1. **Rounding (class a).**
   - Example B1 gives the guard band as "1.64u ≈ 0.17 %"; 0.164 rounds to
     0.16.
   - Example B3 rounds 3.55 up to 3.6 and 3.148 up to 3.2; 3.148 rounds to
     3.1.
   - Table 1 uses k = 1.64: with k = 1.6449 the lognormal limit for
     u_rel = 0.5 is 227.6, i.e. 228, not 227.
   - The rounded B1 zone (16.2–17.8) holds 97.7 %, not 95 %.
   - No decision changes.
2. **Wording (class b).** The table of Example B2 states that the
   distribution "is assumed to be Normal", but the guard band uses the
   t-distribution with ν = 8.
3. **Normal limits for a relative uncertainty (noted, for the article).**
   - Table 1 and Example B3 give the normal acceptance limits as
     L(1 ± k·u_rel), with u taken at the limit (u = u_rel·L), whereas the
     lognormal limits L·exp(±k·s_G) apply the relative uncertainty at the
     measured value.
   - If u = u_rel·y at the measured value, a normal PDF needs
     L/(1 ∓ k·u_rel): 4.71 ng/g in B3. At 3.151 ng/g, u = 1.10 ng/g and the
     probability above 2 ng/g is 85 %, not 95 %.
   - The comparison "normal against lognormal" in Table 1 and Figure 5 thus
     mixes two uncertainty models. Part of the difference the guide
     attributes to the shape of the distribution comes from where u is
     evaluated.
   - The program applies `u_relative` at the measured value (JCGM 106 8.3.3,
     speed-limit example); the library's normal B3 variant enters
     u = 0.7 ng/g to reproduce the guide.

The library has 236 projects.

**F85, addendum — source data and the cause of the "curious" n = 2
(EMUE compendium 2021, Example E5.1).**

- **Data confirmed.** EMUE E5.1.5 lists the boundary pixels as (4, 2),
  (5, 2), (6, 2), (7, 2), (7, 3), (8, 3), (8, 4), …, (3, 3), (4, 3), with
  the origin at the centre of the bottom-left pixel. The 38 pixels read
  from GUM-5 Figure 11.1, shifted by (−1, −1) and traversed
  counter-clockwise from (4, 2), give exactly that sequence.
- **Areas.** Recomputed for n = 1–8: 65.12, 63.99, 64.15, 65.72, 65.72,
  65.27, 65.23, 65.38. EMUE Table E5.1.1 gives 65.11, 63.88, 64.21, 65.73,
  65.67, 65.25, 65.21, 65.38.
- **MCM.** EMUE gives 65.5 ± 1.6; the recomputation 65.45 ± 1.55. GUM-5
  11.5.5 gives 65.4 and 11.6.1 gives 65.0.
- **EMUE computes everything per coordinate and unweighted.**
  RMSR = √(S/(m − k)) and AIC = S + 2k, with S the residual sum of squares
  in pixel² units, m = 38 and k = 2n + 1. Recomputed per-coordinate values
  agree, e.g. n = 6: RMSR 0.26/0.27 (EMUE 0.26/0.29) and AIC 28/28 (EMUE
  28/28). The per-coordinate AIC minimum is at n = 2, as in EMUE.
- **GUM-5 merged the two coordinates inconsistently.**
  - Its RMSR is √(RMSR_x² + RMSR_y²): 0.96 = √(0.64² + 0.72²), 0.39 =
    √(0.26² + 0.29²).
  - Its AIC is S_x + S_y + 2(2n + 1): 38.5 for n = 1, 29.8 for n = 6. That
    is the total misfit with the parameter penalty of one coordinate.
  - Neither matches its own text (χ² weighted by 1/u₀², 2m data, 4n + 2
    parameters).
  - EMUE also gives the extreme areas as 48 and 86 (mean 67, u = 11.0);
    GUM-5 gives 47 and 85 (mean 66). The figure has 47 interior pixels.
- **Cause of n = 2.**
  - An information criterion needs −2 ln L. For a known standard deviation
    u₀ that is χ² = S/u₀² = 12 S. Using the unweighted S understates the
    misfit term by a factor of 12, so the parameter penalty dominates.
  - With χ² the criteria select n = 6 (AIC) and n = 4 (AICc, BIC). n = 6 is
    the RMSR choice of both documents.
  - The conclusion of GUM-5 11.7.3 and EMUE that RMSR saturation is better
    than the information criteria for small data sets is therefore not
    supported: the criteria were misapplied.
  - Class c for the article; it applies to both documents. The RMSR values
    of 0.26–0.29 at n = 6 are consistent with u₀ = 0.29, as EMUE notes.

## 2026-11-07 (session 21, continued) — EMUE compendium: E3.4 and E2.1

**EMUE E3.4 (aqua regia extractable metals in soil, ISO 21748 budget).**

- Tables E3.4.1 and E3.4.2 follow from their formulas to the printed
  rounding:
  - s_I = √(s_w² + s_b²);
  - s_δ̂ = √((s_I² − (1 − 1/n)s_r²)/p) with p = 11 and n = 2;
  - u(δ̂) = √(s_δ̂² + u_ref²).
  - Median RSD 3.2 % and root-mean-square RSD 3.6 % (zinc in C279a
    excluded); u′(Pb)² = 0.002 526 (E3.4.18).
  - Script: `tools/research/emue/e34_soil_tables.py`.
- Library: `emue-e3.4-soil-lead-iso21748`, a product model of relative
  terms that gives U = 2.79 mg/kg as printed, and the `-rms-bias` variant.

**F87. EMUE E3.4 — summary of the bias uncertainty (class c, small).**

- The text gives "the root mean square relative standard uncertainty, 3.3 %;
  for comparison the median and mean are 2.9 % and 3.3 %".
  - The root mean square of the u(δ̂)/x_ref column of Table E3.4.2 is
    3.50 %; 3.3 % is the mean.
  - With 3.5 %, u′(Pb) = 0.0516 and the reported (27.8 ± 2.8) mg/kg becomes
    (27.8 ± 2.9) mg/kg.
- The reported result 27.8 mg/kg for Pb in C279a is not the mean of Table
  E3.4.1 (27.29 mg/kg); it may be a single result, which the text does not
  say.
- The time coefficient 9·10⁻⁴ min⁻¹ is called "reasonably conservative",
  but it is below the largest unsigned value of Table E3.4.3
  (1.6·10⁻³ min⁻¹, Cu in C282a). The temperature coefficient 0.004 °C⁻¹
  does cover its maximum (0.0041).

**D63. Multicomponent conformity: total risks (EMUE E2.1).**

- New core module `multiconformity`:
  - `mvn_box_probability`: the Genz (1992) separation-of-variables
    integral of a multivariate normal over a box, with antithetic uniform
    draws and a standard error. A test checks it against an exact product
    for independent components.
  - `Multicomponent::specific`: normal posterior (E2.1.1)–(E2.1.2) and the
    total specific consumer's or producer's risk.
  - `Multicomponent::global`: total global risks by Monte Carlo, with u_i
    proportional to the true value.
- New project section `multicomponent_conformity` with report (EN/TR), JSON,
  checks and an app results view.
- EMUE E2.1 reproduces exactly. By scipy and numpy:
  R*_tot = 0.002 881, 0.011 716 and 0.000 212 (printed 0.0029, 0.0117,
  0.0002), R_tot = 0.001 82 (printed 0.0018).
- The program gives 0.288 %, 1.17 %, 0.0213 % and 0.180 %.
- Library: `emue-e2.1-multicomponent-medication`. Script:
  `tools/research/emue/e21_total_risks.py`.
- Relevance: pharmaceutical release testing and reference materials
  certified for several properties. Correlated production can make the
  total risk several times the largest single-component risk.

The library has 239 projects.

**EMUE E2.2 (vacuum gauge, sum, quotient and combined models; three
scenarios; correlation).** Tables E2.2.2–E2.2.7 reproduce point by point
(`tools/research/emue/e22_vacuum_tables.py`). The conformance probabilities
of the 30 tabulated points agree to 0.1 % except two rounding cases: scenario
1 point 9, 87.4 against 87.3, and scenario 2 point 12, 98.2 against 98.3.
E2.2.7 reproduces: u(p_std) = 0.894 Pa, u(p_UUC) = 0.500 Pa (printed 0.51),
u(Δp) = 1.140 and 0.909 Pa, p_c = 0.669 and 0.745. The library has five
projects:

- `emue-e2.2-vacuum-gauge-correction-applied`,
  `-correction-in-uncertainty` and `-correction-ignored`: point 8, with
  p_c = 74.5, 77.8 and 83.8 %.
- `emue-e2.2-vacuum-gauge-sum-model-correlated` and
  `-correlation-ignored`: the common temperature error ΔT is entered once
  in the first and as two independent inputs in the second.

**F88. EMUE E2.2 — table slips and the worked example (class b).**

1. **U(e) in Table E2.2.2.** Points 4–6 are printed as 0.0025, 0.0017 and
   0.0011, a factor of ten too small; the values are 0.0254, 0.0170 and
   0.0117. U(f) of point 6 is 0.0117, printed 0.011.
2. **Column headings.** Tables E2.2.3, E2.2.5 and E2.2.7 head a column
   "p_UUC/Pa" that holds the p_std values (161.30, 256.40, …).
3. **Worked example of E2.2.6.1.** It takes "σ = u(f) = 0.002" for point
   13, the expanded U(f) of the table. It obtains p_c = 97.6 %, whereas the
   table, with u(f) = 0.001, gives 100.0 %. The example says "say", so it
   is illustrative, but the label u(f) is wrong.
4. **For the article.** At point 8 the poor practices give higher
   conformance probabilities: 77.8 % when the correction is folded into the
   uncertainty, 83.8 % when it is ignored, against 74.5 % with the
   correction applied. This illustrates why folding a known bias into the
   uncertainty (cf. Magnusson & Ellison 2008, our `uncorrected_bias`) is
   not conservative for decisions.

The library has 244 projects.

**F78, addendum — the source is EMUE E1.1 (compendium 2021).**

- EMUE E1.1.5.4 gives the same results: "pH_X = 7.0109 and
  u(pH_X) = 0.0041".
- Its own intermediate values give a different result. The temperature
  interpolations pH_X1 = 6.8681 (S1) and pH_X2 = 7.4200 (S2) at 23.7 °C,
  followed by interpolation in potential at E_X = −1.875 mV between
  (6.15 mV, 6.8681) and (−26.35 mV, 7.4200), give
  6.8681 + (8.025/32.5) × 0.5519 = 7.0044.
- The text describes the first two steps as interpolations between
  potentials ("(E_S1, pH^T1_S1) ≡ (6.15 mV, 6.8640) and
  (E_S2, pH^T1_S1) ≡ (−26.35 mV, 7.4157)"). The numbers 6.8681 and 7.4200
  are, however, interpolations in temperature of each standard; GUM-5 1.9
  corrected the wording.
- EMUE E1.1.7.1 contains the same reversed statement: with the standards'
  uncertainties set to zero u becomes 0.0040, "implying that the further
  repeated observations of the three potentials would do much to reduce
  u(pH_X)".
- The value 0.0041 for independent inputs and the fully correlated 0.0051
  (E1.1.7.2) also come from EMUE.
- GUM-5 Example 1 therefore inherited the inconsistencies of F78 from its
  source; the article should cite both documents.

**EMUE E3.8 (NH₃ in N₂ by permeation, ISO 6145-10).**

- y(NH₃) = 70.796 µmol/mol with u = 0.598 µmol/mol reproduces Table E3.8.5
  (7.08·10⁻⁵, 5.99·10⁻⁷), using the OLS u(ṁ) = 0.108 µg/min.
- With the errors-in-variables u = 0.088 µg/min it would be 0.50 µmol/mol;
  the guide argues that OLS is adequate but its OLS u is the larger one.
- Library project: `emue-e3.8-ammonia-permeation`, a model with
  intermediates for the molar masses of both parent gases. Script:
  `tools/research/emue/e38_permeation.py`.

**F89. EMUE E3.8 — Arrhenius parameters and notation (classes c and b; no
effect on the result).**

1. **Arrhenius parameters.** From the two rates stated in E3.8.5.2,
   7.4921 µg/min at 52.45 °C and 13.599 µg/min at 62.00 °C,
   E_A = R ln(ṁ₂/ṁ₁)/(1/T₁ − 1/T₂) = 56.64 kJ/mol and
   k₀ = ṁ₁ exp(E_A/RT₁) = 9.1·10⁹ µg/min.
   - The guide prints 59.164 kJ/mol and k₀ = 43.069 µg/min; k₀ below the
     rates is impossible for ṁ = k₀ exp(−E_A/RT).
   - The sensitivity coefficient at 62 °C is ṁE_A/(RT²) = 0.82 µg min⁻¹
     K⁻¹; the guide prints 4.81·10⁻⁷ µg min⁻¹ K⁻¹.
   - The stated contribution of 0.0022 µg/min matches 0.82 with
     u(T) = 0.0027 K, so the printed coefficient and k₀ are slips.
   - The contribution is negligible against u(ṁ) = 0.108 µg/min.
2. **Notation (class b).** "M̄₁ = 17.040 63 g mol⁻¹ for ammonia and
   M̄₁ = 28.013 77 g mol⁻¹ for nitrogen": the second is M̄₂.

The library has 245 projects.

**EMUE E3.3 (NO₂ analyser calibrated with mixtures from dynamic dilution).**

- The amount fractions 708.1, 974.1 and 1290.1 nmol/mol reproduce from
  x₁q₁/(q₁ + q₂). The flows of Table E3.3.2 follow from the MFC curves to
  0.03 % (curve parameters rounded).
- The program's ISO 6143 calibration with covariances of the contents gives
  V_AB = (285.247, −1.30008, 0.0063648), exactly the printed (E3.3.18),
  with A = 89.208 and B = 4.18480. The validation sample gives
  981.95 ± 4.519 nmol/mol, and 4.350 without covariances.
- Library projects: `emue-e3.3-no2-analyser-dynamic-dilution` and
  `-covariances-ignored`. Scripts: `tools/research/emue/e33_dilution.py`
  and `e33_wtls.py`.

**F90. EMUE E3.3 — covariance matrix of the reference values and the
validation uncertainty (class c).**

1. **Diagonal of (E3.3.17).** The covariance matrix is computed "according
   to (E3.3.14) and (E3.3.16)" from u(x₁) = 16 nmol/mol and the flow
   uncertainties of Tables E3.3.2–E3.3.3.
   - Those inputs give the diagonal 1.34, 2.53 and 4.48 nmol²/mol², i.e.
     u = 1.16, 1.59, 2.12 nmol/mol.
   - The printed diagonal is 39.5, 37.0 and 41.2, i.e. u = 6.3, 6.1 and
     6.4 (Table E3.3.4).
   - The off-diagonal terms do follow: 1.76, 2.32 and 3.19 against 1.7,
     2.2 and 3.0.
   - A contribution of about 38 nmol²/mol² per mixture is therefore
     included but not described, and it is uncorrelated between the
     mixtures. Its source (e.g. the impurity of the diluent, which the text
     takes as zero) cannot be identified from the example.
2. **Validation uncertainty.** Eq. (E3.3.19) with the printed A = 88.94,
   B = 4.19, V_AB, w = 213.33 and u(w) = 0.11 gives y = 982.8 and
   u = 4.47 nmol/mol; printed 982.0 and 4.7.
   - With unrounded parameters u = 4.52 (both the program and a full WTLS).
   - Without covariances u = 4.35, so the covariances raise u by 3.9 %;
     printed "up to 3.4 %" with 4.5 against 4.7.
3. **Two procedures mixed (noted).** A full WTLS with the covariances in the
   weights reproduces the printed A = 88.94 but gives V_AB = (288.5,
   −1.318, 0.00646). The printed V_AB equals the ISO 6143 estimate with
   diagonal weights and propagated covariances, whose A is 89.21. The
   printed A and V_AB appear to come from different procedures. The
   difference in A (0.27 nmol/mol) is far below u(A) = 16.9.

The library has 247 projects.

**EMUE E6.5 (ephedrine in urine, WADA).**

- Library projects:
  - `emue-e6.5-ephedrine-wada-topdown` (Nordtest one-CRM bias):
    u_prec = 2.425 %, u_bias = 3.036 %, u_c = 3.89 %.
  - `emue-e6.5-ephedrine-wada-decision-limit` (guarded rejection, p = 0.95,
    u = 0.5 µg/mL): DL = 10.822 µg/mL, which WADA sets at 11; all three
    EQAS means are rejected.
- Steps 8–9 of Table E6.5.2 reproduce: |Δ| = 0.47, 0.63, 0.37 and
  U(Δ) = 2√(u²(x) + u²(X)) with u(X) = 1.25 s*/√p = 0.93, 0.92, 0.89.

**F91. EMUE E6.5 — Tables E6.5.1 and E6.5.2 (classes a and b).**

1. **u_c (class a).** Table E6.5.1 gives u_c = 3.8 % from the rounded
   2.4 % and 3.0 %; unrounded √(2.425² + 3.036²) = 3.89 %. The table then
   uses u_c(x) = 0.038x. The conclusion u_c < u_c,max is unaffected.
2. **Table E6.5.2 (class b).**
   - The mean of round 2, (11.5, 11.9, 11.6), is printed 11.6; it is
     11.67.
   - The standard deviation of round 3, (11.5, 11.4, 11.5), is printed
     0.45 µg/mL; it is 0.058 µg/mL, 0.50 %, whereas the relative value
     printed is 0.4 %.
3. **z-scores (class c).** The printed 0.27, 0.71 and 0.47 follow from none
   of the stated scales.
   - With σ = u_c,max = 0.5 µg/mL ("the target standard uncertainty as the
     standard deviation for performance rating") they are −0.93, −1.27 and
     0.73.
   - With the robust s_R they are −0.88, −1.19 and 1.05.
   - All |z| < 2, so the conclusion "satisfactory" holds.
4. **For the article.** u_bias includes Δ² (the 3 % bias against the CRM is
   not corrected but entered as an uncertainty, Nordtest style). It
   dominates u_c, the same practice that E2.2 (scenario 2) calls poor when
   a known correction exists.

The library has 249 projects.

## 2026-11-07 (session 21, continued) — IUPAC brief guide 2024 (Possolo et al., PAC 96, 113)

**Text extraction.** The PDF encodes every glyph separately, so pdftotext
returns "T h e" for "The". `tools/research/joinwords.py` rejoins the
fragments from `pdftotext -bbox` by their horizontal gaps.

**Library (`tools/generators/iupac2024.py`, 4 projects).**

- `iupac2024-4a-water-hardness`: H = 96.65 mg/L, u = 3.18 mg/L (printed
  ≈ 3).
- `iupac2024-4b-silicon-isotopic-abundances` (multivariate): x₂₈, x₂₉, x₃₀
  as printed, 0.923 3873(36), 0.046 2300(22) and 0.030 3827(30), with their
  correlations. The MCM is off because the program refuses the coverage
  region of a singular output covariance (abundances summing to 1). A
  candidate extension is to report the MCM covariance without the region.
- `iupac2024-4i-argentometric-titration`: c = 10.000 mmol/L,
  u = 0.0255 mmol/L (0.26 %); GUM and MCM.
- `iupac2024-3a-argentometric-titration-ksp`: an implicit model with the
  solubility products, c = 9.9760 mmol/L.

Not in the library:

- 4H (QUAM A1 cadmium) needs a beta distribution, which the program does
  not have; QUAM A1 itself is in the library.
- 4F, 4M–4P (maximum likelihood, Bayes, MCMC, consensus) belong to the
  planned consensus/Bayes module.

**F92. IUPAC 2024 Example 4I — sign of the endpoint correction (class c;
to check against Meija et al. [14]).**

- The guide writes Meija et al.'s equation as [Ag⁺] − [Cl⁻] =
  (c(NaCl)V(NaCl) − c(AgNO₃)V_E)/(V(NaCl) + V_E). Here [Ag⁺] =
  √(K_sp2(V + V_E)/(c(K₂CrO₄)V)) and [Cl⁻] = K_sp1/[Ag⁺].
- At the endpoint [Ag⁺] = 28.1 µmol/L and [Cl⁻] = 6.3 µmol/L, so the left
  side is positive (21.8 µmol/L). Solved as printed, the equation gives
  c(NaCl) = 10.024 mmol/L, 0.24 % above the conventional value.
- The mass balance (silver added − chloride present = [Ag⁺] − [Cl⁻] in
  solution) has the opposite right side and gives 9.976 mmol/L. That is
  0.24 % below, as expected because the chromate endpoint follows the
  equivalence point.
- The guide reports only "the relative difference … is 0.24 %", which holds
  for either sign. The library project uses the mass balance.

The library has 253 projects.

## 2026-11-07 (session 21, continued) — ILAC P14:09/2020

The reporting already meets 5.2: y ± U with k, coverage probability and the
explanatory sentence. 5.3 (at most two significant digits) is the default.

**D64. CMC as a floor and the 5.3 note.**

- `reporting.cmc` (`expanded` or `relative_percent`): the reported expanded
  uncertainty is max(U, CMC) (5.5: no smaller uncertainty than the CMC).
  The statement names the evaluated U and cites the clause.
- `expected.gum.reported_expanded_uncertainty` checks it.
- With `significant_digits` > 2 the statement adds a note on 5.3.
- Library: `ilac-p14-cmc-floor-weight-10kg`, the EA-4/02 S2 weight with an
  illustrative CMC of 80 mg, marked as hypothetical in the project.
- The 5.2 note on asymmetric (Monte Carlo) results is covered by the
  coverage intervals and, since D61, the half-width about the estimate.

The library has 254 projects.

## 2026-11-07 (session 21, continued) — Turkish edition of Nordtest TR 537

**D65. Turkish spelling of reproducibility.** The Turkish Nordtest TR 537
writes *tekrar üretilebilirlik* as two words (32 times; *laboratuvar içi
tekrar üretilebilirlik* for u(R_w)), as do TÜRKAK documents. The program had
*tekrarüretilebilirlik*.

- All 129 occurrences were changed in 35 tracked files: reports, app
  strings, generators, library projects and the guide.
- The glossary notes the other wording of the Turkish edition: *kontrol
  kartı* beside *kontrol grafiği*, and *iki tekrarlı analiz* for
  duplicates. *SRM* for CRM and *kontrol numunesi* were already used.

**Mari et al. 2025 and Pradella 2026 (definition of measurement
uncertainty).**

- Mari et al. (Metrologia 62, 062101) object to three changes made by
  GUM-1:2023 §3.4 and proposed for VIM 4:
  - a psychological instead of a mathematical entity;
  - doubt about a true value instead of dispersion of attributed values;
  - only single-valued measurands, excluding definitional uncertainty.
  They propose generalizing the GUM definition ("mathematical entity
  conveying information concerning the doubt about the validity of the
  measured value").
- Pradella (2026) supports the change for medical and testing laboratories
  with nominal and ordinal results (ISO 15189, ISO 20914, ISO 22692).
- WG1–WG2 kept "doubt" but dropped "true" (D61). Note 2 accepts summaries
  such as the standard uncertainty, interval or quantiles. That answers
  part of the second objection.
- For the article: the program's design (the distribution as the complete
  result; standard uncertainty, intervals and, since D61, the symmetric
  half-width as summaries) is neutral in this debate. Nominal properties
  (Ellisor 2025, Bilson 2025) are an outlook.
- Glossary EN/TR: paragraph "The debate".

## 2026-11-07 (session 21, continued) — Consensus values: NIST Consensus Builder, Koepke et al. 2017, IUPAC 2024 4F

**D66. Consensus-value module (`consensus`).** Core `consensus.rs`, project section `consensus`, reports EN/TR
with a chart of the degrees of equivalence, JSON and an app view.

- DerSimonian–Laird after NICOB §A.2: Cochran's Q about the 1/u² weighted mean, the method-of-moments τ̂,
  weights 1/(τ̂² + u_j²), the naive u_DL = 1/√Σw, the p-value of Q, the Knapp–Hartung u and its t_{n−1}
  expansion.
- The NICOB parametric bootstrap (§A.2 (a)–(d)): τ_k from the gamma approximation of Q with the exact mean and
  variance of Biggerstaff & Tweedie (1997), E Q = (n − 1) + τ²(S₁ − S₂/S₁),
  Var Q = 2(n − 1) + 4τ²(S₁ − 2S₂/S₁ + S₃/S₁²) + 2τ⁴(S₂ − 2S₃/S₁ + S₂²/S₁²) with S_r = Σu_j^{−2r}, evaluated at
  τ̂ (the manual does not say at which τ; τ̂ is the plug-in choice); τ_k² = max{0, (Q_k − n + 1)/(S₁ − S₂/S₁)};
  x_jk ~ N(μ̂, τ_k² + u_j²); u_jk = u_j√(ν_j/χ²_ν_j) when ν_j is given. u is the s.d. of the μ_k, the interval the
  probabilistically symmetric one (2.5 % and 97.5 % percentiles).
- Unilateral degrees of equivalence (§A.5.1 (e)): D_j = x_j − μ̂; U95 %(D_j) is the half-length of the shortest
  interval centred at the mean of the D_jk = x_jk − μ_k that holds 95 % of them (`mcm::half_width_about`).
- Linear Pool (§A.4): mixture of Gaussian or rescaled Student t (s.d. u_j, so ν_j > 2) components with
  optional weights; mean, s.d. and percentiles of the sample. DoE of the Linear Pool: D_j = x_j − μ̂ with
  U95 % = k·u_j (§A.5.3: the D_jk = x_j + e_jk − μ̂ are symmetric about D_j).
- Maximum likelihood (IUPAC 2024 Example 4F; Koepke Table 2): profile likelihood in τ (grid of 4000 points on
  [0, 10 max(s, u_max)] and golden-section refinement), μ̂(τ) the weighted mean, u from the inverse Hessian of
  −ln L in (μ, τ).
- Not implemented: the hierarchical Bayesian procedure (§A.3; half-Cauchy priors on τ and on the σ_j, MCMC),
  the leave-one-out DoE and the bilateral DoE B_ij. Recorded as open items.
- Library (258 projects at D66): `koepke2017-newtonian-constant-g` (DL, bootstrap, Linear Pool, ML),
  `koepke2017-half-life-cs137`, `koepke2017-half-life-sr90` (in days, as Table 3; duplicate labels Fl65 split
  into Fl65a/b/c), `iupac2024-4f-ccqm-k145-nickel-bovine-liver`. Evidence: `tools/research/consensus/
  koepke2017.py` (numpy, independent implementation of DL, bootstrap and Linear Pool).

Results against the printed values:

| Case | Printed | Program |
|---|---|---|
| G, DL | 6.673 80, u 0.000 28, (6.673 20, 6.674 40) | 6.673 790, u_DL 0.000 279, bootstrap u 0.000 280, (6.673 226, 6.674 352); τ 0.000 946, Q 319.1 |
| G, Linear Pool | 6.673 67, 0.001 24, (6.671 18, 6.675 79) | 6.673 667, 0.001 243, (6.671 166, 6.675 791) (10⁵ draws) |
| G, ML | 6.673 78, 0.000 30 | 6.673 778, 0.000 300 |
| ¹³⁷Cs, DL / LP | 30.04 (0.04) a; 29.94 (0.93) a | 30.037 (0.045) a; 29.94 (0.93) a |
| ⁹⁰Sr, DL / LP | 28.73 (0.11) a; 28.68 (0.77) a | 28.730 (0.110) a; 28.69 (0.77) a |
| K145 Ni, ML | μ 2.043(15), τ 0.044(14) mg/kg | 2.042 94 (0.015 44), 0.043 54 (0.014 24) |

For the article:

- With heterogeneous results the bootstrap u of the DL estimate hardly differs from u_DL in these three data
  sets (0.000 279 vs 0.000 280 for G; 0.0446 vs 0.0449 a for ¹³⁷Cs), while the Knapp–Hartung u is larger
  (0.000 307; 0.106 a for ¹³⁷Cs, 2.4 × u_DL). The choice among DL-based evaluations matters more than the
  bootstrap itself; the Linear Pool is 4.4 (G), 7.0 (⁹⁰Sr) and 21 (¹³⁷Cs) times wider. This is the "procedure-choice" spread the
  article can show with the program on the same data.
- CCQM-K145: with the DL bootstrap, one participant has a marginal |D| > U95 % (0.107 vs 0.106 mg/kg) and the
  others are within. Following the independent review (2026-11-08, item 19) the participants are coded Lab 1–17 in
  the project and no degree of equivalence is attributed to a named institute: the official KCRV and degrees of
  equivalence are those of the final report in the KCDB, which may rest on another reference value and procedure.
  The computation illustrates the procedures on the same data, not a statement on the comparison.

**F93. Koepke et al. 2017 Table 2 — DerSimonian–Laird consensus value of G (class a, rounding).** From the 14
values of Table 1 the DerSimonian–Laird estimate is 6.673 790 (numpy and the program agree to 10⁻⁹), which
rounds to 6.673 79, not the printed 6.673 80. The printed interval (6.673 20, 6.674 40) is consistent both with
the bootstrap percentiles (6.673 22, 6.674 35) and with the t_{13} interval (6.673 19, 6.674 39) rounded to 10⁻⁴,
so it cannot tell which was used. The ML row (6.673 78) and the Linear Pool row (6.673 67) reproduce exactly.
Likely cause: rounding of an intermediate value, or digits of Table 1 not printed (CODATA 2014 Table XXVII
carries the same digits). Impact: one unit in the fifth decimal, 0.04 u.

## 2026-11-07 (session 21, continued) — Bayesian consensus; IUPAC 2024 Examples 3E, 4M, 4N

**D67. Bayesian random-effects model without MCMC (`consensus.bayes`).**

- Model: x_j ~ N(μ, τ² + σ_j²) with σ_j = u_j, τ half-Cauchy with median s, μ Gaussian (m₀, s₀) or flat.
  This is NICOB's hierarchical Bayesian procedure (§A.3) when no ν_j are given (the manual: "when they are
  based on infinitely many degrees of freedom … σ_j = u_j") and IUPAC 2024 Example 4M.
- For fixed τ the posterior of μ is Gaussian with precision P(τ) = Σ1/v_j (+ 1/s₀²), v_j = τ² + u_j²; the
  marginal likelihood of τ is ∏v_j^{−1/2} P^{−1/2} exp{−½(Σx_j²/v_j (+ m₀²/s₀²) − P μ̂_τ²)}.
- The grid τ = s·tan(πθ/2), θ uniform on (0, 1) (10⁵ midpoints), makes the half-Cauchy prior density constant,
  so the posterior weights are the marginal likelihoods. The tails of the prior are covered without a cut-off.
- μ: posterior mean and s.d. of the Gaussian mixture; quantiles by Newton's method on the mixture CDF with a
  bisection safeguard. τ: posterior median and quantiles from the cumulative weights.
- Default prior on τ (`"bayes": {}`): median of |x_j − median(x)|, as NICOB; μ flat (NICOB: N(0, 10⁵)).
- Results are deterministic (no seed) and exact up to the grid. Not implemented: NICOB's half-Cauchy priors on
  the σ_j when ν_j are given (needs n + 2 dimensions; MCMC or nested quadrature).

| Case | Printed (MCMC) | Program (grid) |
|---|---|---|
| IUPAC 4M/4N, iridium | M = 192.2171(4) g/mol; median τ 0.0005 g/mol | 192.217 119 (0.000 440); median τ 0.000 506; 95 % (192.216 16, 192.217 95) |
| Koepke Table 2, G | 6.673 77, 0.000 33, (6.673 10, 6.674 42) | 6.673 773, 0.000 329, (6.673 105, 6.674 409) |
| Koepke Table 4, ¹³⁷Cs | 29.94 (0.19) a | 29.944 (0.185) a |
| Koepke Table 4, ⁹⁰Sr | 28.71 (0.16) a | 28.708 (0.160) a |

For the article: the grid reproduces the MCMC results of NICOB and of the IUPAC guide to their printed digits
(one exception, F95) in a fraction of a second, without convergence diagnostics or a seed. For the two-parameter
random-effects model the MCMC of the guide (5 × 10⁵ Metropolis steps) is not needed.

Library: `iupac2024-3e-4m-iridium-molar-mass-bayes`; the three Koepke projects now include the Bayesian row.

**F94. IUPAC 2024 Example 3E — rounding of s and of the p-value of Q (class a).** The three molar masses have
s = 0.000 545 01 g/mol, printed 0.000 54 (rounds to 0.000 55); the naive τ = √(s² − ū_g²) is 0.000 4245 with the
unrounded s (printed 0.000 42, which also follows from the rounded inputs). Cochran's Q about the weighted mean is
9.562, p = 0.008 39 with 2 degrees of freedom; printed p = 0.0082 (Q = 9.61 would give it). Impact: none on the
conclusions.

**F95. Koepke et al. 2017 Table 4 — Bayesian u for ¹³⁷Cs (class a, Monte Carlo).** The exact posterior s.d. of
the hierarchical model with the NICOB defaults is 0.185 a; printed 0.19 a. The posterior mean (29.944 a) and the
⁹⁰Sr row (28.708, 0.160 a) and the G row agree. The difference, 3 %, is of the size of the Monte Carlo error of a
standard deviation from a few thousand effectively independent MCMC draws; the printed value is the rounding of
an MCMC estimate, not of the exact posterior. To note in the article as an example of the reproducibility of
MCMC-based reference values.

## 2026-11-07 (session 21, continued) — Possolo & Meija 2022 (Measurement Uncertainty: A Reintroduction, 2nd ed.)

The book is CC BY-SA 4.0. Five examples fit the program; the others need methods it does not have (below).

**D68. Laplace laboratory effects in the Bayesian consensus (`"effects": "laplace"`).**

- The NIST Decision Tree (Possolo et al. 2021) recommends Laplace effects when the effects are symmetric but
  heavier-tailed than Gaussian. The book fits it with Stan (theta ~ double_exponential(H0, tau/√2), so τ is the
  standard deviation of the effects).
- The likelihood of one result is the normal–Laplace convolution
  f(d) = (1/4b)[e^{r²/2 − d/b} erfc((r − d/s)/√2) + e^{r²/2 + d/b} erfc((r + d/s)/√2)], b = τ/√2, r = s/b.
  For z ≥ 5 the program uses a − z² = −d²/(2s²) with the asymptotic series of erfcx, which avoids the
  cancellation of two large terms when b → 0 (tested: unit integral, Gaussian limit b → 0, Laplace limit s → 0).
- μ cannot be integrated analytically: (θ, μ) grid of 300 × 300, τ = s·tan(πθ/2), with a second pass on μ over
  ±12 posterior s.d. Quantiles by interpolation within the cells. About 1.5 s for 22 results.
- Possolo & Meija, Hubble–Lemaître constant (22 estimates, Planck excluded; τ prior median = R's mad(x),
  1.4826 × median|x − median|; H₀ ~ N(67.4, 100²)): Stan 71.69, 0.66, median τ 2.03 (km/s)/Mpc; program
  71.6915, 0.6603, 2.0336 (numpy grid 2000 × 1500 agrees to 10⁻⁴).

Model examples (GUM and Monte Carlo, existing sections): plasma anion gap (25, u 1.85 mmol/L), storage tank
(7204 ± 54 m³ by Gauss and by MCM), Wheatstone bridge (957 ± 7 Ω; contributions E 52 %, F 48 %, G 0.6 %,
H 0.004 % reproduced), atomic weight of lead in HIPB-1 (multivariate: four amount fractions and A_r(Pb) =
207.17915, u 0.000 084, U95 0.000 17, printed 0.0002). The three isotope ratios and the closure determine the
fractions exactly, so the book's constrained MLE (R solnp) and its 5 × 10⁴ Monte Carlo refits reduce to explicit
expressions and the analytic Jacobian.

Not taken into the library (methods not in the program), recorded for the outlook of the article:

- Arsenic in kudzu (NIST SRM 3268): Laplace effects and half-Cauchy priors on the σ_j with ν_j = 2 (n + 2
  parameters; MCMC, or a 1-D table per laboratory and a convolution, too costly for the test suite). Result
  0.895(29) mg/kg, median τ 0.167 mg/kg, 95 % (0.839, 0.951).
- Thermal bath (ARMA model of autocorrelated readings), counting examples (Poisson, negative binomial),
  ranking, categorical data, model averaging (Machu Picchu, influenza), model uncertainty (Pluto, Everest).

**F96. Possolo & Meija 2022, Atomic Weight of Lead — correlations of the amount fractions (class a, Monte
Carlo).** Linearised (and Monte Carlo with 10⁶ draws): r(²⁰⁴Pb, ²⁰⁶Pb) = −0.011, r(²⁰⁴Pb, ²⁰⁸Pb) = −0.179;
printed −0.02 and −0.17 from 5 × 10⁴ draws, whose standard error for a correlation near 0 is about 0.0045. The
other four correlations and the four u(x) agree. Impact: none on A_r(Pb).

## 2026-11-07 (session 21, continued) — BIPM ISRD qNMR purity budgets

**D69. qNMR purity budgets in the library.** The BIPM Internal Standard Reference Data documents (ISRD-01, -02,
-03, -06, -07, -08, -09) give worked uncertainty budgets for the purity of an analyte by ¹H (or ¹⁹F) qNMR with an
internal standard: w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S (Eq. 1). Nine of the eleven ¹H tables are in the
library (`tools/generators/bipm_isrd.py`) as model projects with the printed values and standard uncertainties
as inputs and the printed purity and u_c as checks. The budgets of ISRD-02 and -03 are images in the PDFs and were
read from rendered pages. Excluded: ISRD-03 Table 4 (normalised ratio: the product of the printed values gives
0.999 66, printed 0.9995) and ISRD-08 Table 4 (product 1.1078, printed 0.9974); ISRD-09 (¹⁹F) gives rounded inputs
and relative sensitivities only. Evidence: `tools/research/qnmr/isrd_budgets.py`.

For TÜBİTAK UME the projects are templates for its own qNMR purity assignments: the same equation, with the
masses, the purity of the internal standard and the integral ratio as the main components.

**F97. BIPM ISRD qNMR budgets — internal inconsistencies of the spreadsheet tables (class b; for the article,
not to be reported).**

1. *Component of the integral ratio.* In 7 of 11 tables the printed component c·u of the ratio exceeds the
   product of the printed sensitivity coefficient and standard uncertainty by 3–17 %: ISRD-01 T4 1.95 vs 1.74,
   ISRD-02 T4 1.75 vs 1.65, ISRD-03 T4 1.80 vs 1.66, ISRD-06 T4 1.49 vs 1.34, T6 1.95 vs 1.73, ISRD-07 T4 1.47 vs
   1.26, T6 1.99 vs 1.77 (× 10⁻⁴). The combined uncertainties follow the components in most tables and the printed
   u in ISRD-01 T4 (7.32 × 10⁻⁴ in the row "Assigned value", 7.30 × 10⁻⁴ as "Combined Uncertainty").
2. *Combined uncertainty against its own components.* ISRD-01 T6: the components give 4.12 × 10⁻⁴, the printed
   u_c is 3.96 × 10⁻⁴ (the printed inputs give 4.10 × 10⁻⁴).
3. *Degrees of freedom.* The printed ν_eff (ISRD-01 T4 784, T6 131; ISRD-02 T4 1655, T6 967; ISRD-03 394) are
   reproduced with ν(R) = 4 in all five tables, whereas the text describes the ratio as the mean of four, five or
   six replicates (ν = 3, 4 or 5 by GUM 4.2.6) or of sixteen normalised values. k = 2 is used throughout, so the
   effect on U is nil.
4. *Misprints.* ISRD-08 Table 2 labels the ratio "IS/IA" although its value 1.4313 is I_A/I_S (only that gives
   998.8 mg/g), and its m_S component is printed 3.28 × 10⁻⁵ for c·u = 3.84 × 10⁻⁵. ISRD-08 Table 4 prints the two
   ¹H-nuclei components as 1.50 × 10⁻⁵; the combined 4.2 × 10⁻⁴ needs 1.50 × 10⁻⁴; and its printed values do not
   reproduce the purity (above). ISRD-06 Table 4 gives the IS purity 0.9991 the unit "mg.g-1" (g/g).
5. *Same molecule, different constants.* ISRD-02 Table 4 re-evaluates the measurement of ISRD-01 Table 4 with the
   roles exchanged (same integrals 17834/97943 and masses 2.3545/3.5063 mg), but with N(¹H, MA) 1.9996 vs 1.9994,
   N(¹H, DMSO₂) 5.9988 vs 5.9934, M(MA) 116.080 vs 116.0724 g/mol (116.072 in ISRD-03, -07, -08) and M(DMSO₂)
   94.1284 vs 94.136 g/mol (94.128 in ISRD-06, -07).
6. *A bounded result.* ISRD-06 Table 6 reports 999.8 +0.2/−1.5 mg/g: the interval is truncated at 1000 mg/g.
   This is the situation that GUM Supplement 1 and a Bayesian treatment with a bounded prior handle consistently;
   the program does not yet model the bound (outlook).

None changes a certified value materially (all within 0.2 u_c, except the excluded tables). The pattern —
spreadsheet budgets whose displayed numbers do not reproduce one another — is the article's argument for
budgets recomputed from their inputs by validated software.

## 2026-11-07 (session 21, continued) — Bayesian tutorials, Toman 2016, Degenhardt 2025

**D70. What was taken from the Bayesian and "alternative to MCM" documents.**

- *Possolo & Bodnar 2018* (approximate Bayesian evaluations), §4.4 chromium in soil (ISO Guide 35 Example C.1,
  20 bottles × 3, reprinted in their Table 3): taken into the library as a homogeneity project
  (`possolo-bodnar-2018-chromium-soil-homogeneity-bayes`). The model and priors (τ, σ half-Cauchy with medians
  the s.d. of the bottle averages and the median within-bottle s.d.; ω flat) are those of D60, so the existing
  grid evaluation applies: ω = 121.624, u = 0.9986 mg/kg (paper: MCMC 121.62 (1), Laplace 0.98, REML 0.95);
  ANOVA s_bb = 3.93, s_r = 2.87 mg/kg (= Guide 35). New expected field `homogeneity.mu_sd`.
  Not taken: the cryptosporidiosis odds ratio (binomial data), the manometer (non-identifiable tilt angle with a
  beta prior; Lira & Grientschnig 2010), and GUM H.5 by NICOB with σ_j priors on 4 degrees of freedom (needs the
  σ_j priors of NICOB §A.3, not implemented; D67).
- *Toman, Nelson & Lippa 2016* (qNMR, hierarchical Bayes): two-level model (samples within internal standards,
  internal standards about the purity) fitted by OpenBUGS with vague priors whose parameters are not given
  ("e and f set as large numbers"); the per-sample results are "essentially identical to the usual uncertainty
  analysis obtained with the GUM procedure". Not reproducible exactly; recorded for the article as the
  hierarchical alternative to the single-sample budgets of D69. The beta prior on a purity bounded by 1 g/g
  (their eq. (8)) is the same point as F97 item 6.
- *Klauenberg et al. 2015* (Bayesian normal linear regression, sonic nozzle): conjugate normal–inverse-gamma
  posterior, analytic and therefore a candidate for the calibration module, but the data are in online
  supplement B, which is not in the folder (added to `documents-needed.md`).
- *Degenhardt et al. 2025* (SSPRC): systematic samples per input, propagated one at a time, reconstructed and
  combined by convolution. It assumes additive, independent effects (their §4); the examples (AFM indentation)
  are not reproducible from the paper. Not implemented: the program's models are cheap to evaluate, and the
  second-order check of JCGM 100 Amd.1 (D56) already flags non-additivity. For the article's related work.
- *Possolo 2019* (asymmetric uncertainties): symmetrisation of asymmetric results (Audi et al.) and the ozone
  cross-section; outlook, together with bounded results (F97 item 6).

## 2026-11-07 (session 21, continued) — Merkatas et al. 2019; Possolo 2023; Thompson & Ellison 2011; Toman & Elster 2013

- *Merkatas et al. 2019* (shades of dark uncertainty, G): Table 1 (16 values: Koepke's 14 with JILA-10 corrected
  to 6.672 60(25), and HUST-TOS-18, HUST-AAF-18) and four rows of Table 4 reproduced in the library project
  `merkatas2019-newtonian-constant-g-shades`: DL with Knapp–Hartung 6.673 99, u 0.000 25, τ 0.000 56; ML 6.673 90,
  u 0.000 25, τ 0.000 91; BG (NICOB Bayes) 6.673 89, u 0.000 27, (6.673 33, 6.674 42), median τ 0.000 95 (grid
  6.673 896, 0.000 271, (6.673 342, 6.674 415), 0.000 936); LOP 6.673 77, 0.001 17, (6.671 27, 6.675 77).
  The other ten procedures of the table (Birge, CODATA expansion, Mandel–Paule, REML, Student t, Laplace effects
  and errors, mixture model) are not in the program. For the article: on the same data the consensus values of
  the fourteen procedures span 6.673 77–6.674 29 and their uncertainties 0.000 11–0.001 17, a factor of ten; the
  procedure is part of the measurement model (Merkatas et al.; Koepke et al.).
- *Possolo 2023* (reproducibility challenge): SiO₂ in granite G-1, 34 results without uncertainties (Fairbairn
  1951), consensus by Rukhin's weights; the program needs u_j. Outlook.
- *Thompson & Ellison 2011* (dark uncertainty, Accred. Qual. Assur.): the concept and its size in proficiency
  tests; no data set. Glossary and article introduction.
- *Toman & Elster 2013* (key comparison analysis): Bayesian treatment of key comparisons with a
  reference-value prior; no reproducible data set in the text. Outlook.

## 2026-11-07 (session 21, continued) — The author's UME procedure (draft, November 2026)

**D71. The two worked examples of the procedure in the library; notes for the author.**

- `ume-procedure-19-1-dmm-10v-calibration` (additive model, Welch–Satterthwaite with ν = 11 for the readings) and
  `ume-procedure-19-2-qnmr-purity-internal-standard` (multiplicative model, relative uncertainties; illustrative
  values). All printed numbers are reproduced: 19.1 u_c = 35.1 µV, ν_eff ≈ 4000 (3965), U = 70.2 µV, the
  dominant-component check u_N/u_R = 1.14; 19.2 P_a = 995.0 mg/g, relative u_c 1575 × 10⁻⁶, ν_eff = 121,
  U = 3.13 → 3.1 mg/g, variance shares 18/54/23/4/2 %.
- Notes for the author (own document; not findings):
  1. The model equations of Sections 19.1 and 19.2 are embedded equation objects; in the text extraction the
     numbers of ¹H nuclei of 19.2 are missing. Only N_std/N_a = 2/3 reproduces 995.0 mg/g (maleic acid, 2 H, and
     an analyte of 194.19 g/mol with a methyl singlet, e.g. caffeine). Stating N_a and N_std in the table would
     make the example self-contained.
  2. The scope includes CCQM/EURAMET comparisons and reference-material value assignment, but the procedure has no
     section on consensus values, dark uncertainty and degrees of equivalence; the program now covers them (D66–D68)
     and could be cited there.
  3. A purity close to 1000 mg/g with U reaching the bound (BIPM ISRD-06 reports +0.2/−1.5 mg/g, F97 item 6)
     deserves a sentence on asymmetric intervals at a physical limit.
  4. Terminology agrees with the guide: *ölçülen büyüklük*, *duyarlılık katsayısı*, *birleşik standart
     belirsizlik*, *etkin serbestlik derecesi*, *kapsam aralığı/olasılığı/faktörü*, *koruma bandı*, *özgül risk*.
     The procedure says *tolerans aralığı* and *kabul aralığı* where the guide mostly says *tolerans sınırı* and
     *kabul sınırı* (both are used correctly: the interval is bounded by the limits).

## 2026-11-07 (session 21, continued) — Article skeleton

**D72. One main paper; skeleton in `docs/article/outline.md`.** Software, validation and the audit of the worked
examples go into one Q1 paper (first choice *Measurement*; *Metrologia* or *Talanta* as alternatives, quartiles
to be verified at submission). The numbers of the paper are generated (`tools/research/article_stats.py` →
`docs/article/numbers.md`): 277 projects, 36 source documents, 222 validated rows (23 with a finding), F1–F97.
The skeleton maps every section to its evidence (F, D, validation rows), lists Figures 1–3 and Tables 1–4, and
keeps the pre-submission checklist; the D12 class audit is now its first item (25 findings carry no class in
their heading).

## 2026-11-07 (session 21, continued) — D12 audit of the finding classes

**D73. Classes for the findings without a class in their heading; class n.** D12 defines (a) intermediate
rounding, (b) misprint or transcription without effect on the result, (c) inconsistency that changes a printed
result. Some early entries are not discrepancies at all (methodological notes, summaries, a full reproduction):
they get the new label **n**. The classes of all findings are kept in `docs/research/finding-classes.tsv`
(heading classes, overridden by this audit); `tools/research/article_stats.py` reads it.

| F | Class | Reason |
|---|---|---|
| F1 | a | GUM H.1 values follow from unrounded u; printed rounded values reproduce with them |
| F2 | n | Monte Carlo dispersion of printed MCM values (inherent) |
| F3 | n | methodological: location of shortest intervals |
| F4 | n | methodological: GUF slightly conservative |
| F5 | c | arithmetic slip changes u(bias) 4.1 → 4.2 % |
| F6 | b | 2.7 % printed, 2.6 % used; result consistent |
| F7 | b | n = 47 stated, 46 pairs listed; result reproduced |
| F8 | a, b | BOD from rounded biases; PCB summary table differs from the example |
| F9 | a | rounded R̄; the 3.69 factor stays "to verify" |
| F10 | a, c | rounded linearity term; Table A2.4 not reproducible |
| F11 | n | summary of F1, F5–F10 |
| F12 | c, b | summary u_c inconsistent with its inputs; typos |
| F13 | b | derived columns and t value misprinted; statistic reproduced |
| F14 | c | 0.19 dm² printed and used for 0.152 dm² |
| F15 | b | 0.12 for 1.2 % m/m |
| F16 | n | fully reproduced |
| F17 | a, b | rounded intermediates; χ² about the unweighted mean; Eq. (B6.8) factor omitted (same conclusions) |
| F18 | a, b | rounded contributions and misprints (S13 is F21) |
| F19 | n | methodological: second-order terms |
| F20 | n | summary |
| F23 | n | methodological: verdict near the threshold |
| F31, F35 | n | methodological: conventions for t inputs |
| F37 | n | methodological: Algorithm A with bimodal data |
| F80 | c | printed global risks follow from a joint PDF whose s.d. depends on the measured value; the conditional form gives other values |

## 2026-11-07 (session 21, continued) — Benchmark against Suncal

**D74. Library-wide comparison with Sandia's Suncal 1.7.1** (`tools/research/benchmarks/suncal_compare.py`,
table in `docs/research/benchmark-suncal.md`). The script translates every model project that Suncal can express
(scalar output; no correlations, intermediates or ensembles; normal, rectangular, triangular, arcsine, constant,
t, type_a, type_a_pooled inputs; no log10) into a Suncal model, computes Suncal's first-order GUM and, where all
inputs are normal/rectangular/triangular/arcsine/constant, its Monte Carlo with 10⁶ draws, and compares them with
the program's JSON output.

- 107 projects compared, 30 model projects skipped.
- First-order standard uncertainty: max relative difference 6.4 × 10⁻¹² (rounding only).
- Estimates: identical except the two JCGM 101 comparison-loss projects that request the second-order estimate
  term of JCGM 100 Amd.1 (Δy = 0.5u and 0.1u by design; Suncal is first order).
- Monte Carlo standard deviation (48 projects): max relative difference 3.0 × 10⁻³; divided by the sampling
  standard error √((κ + 2)/(4N₁) + (κ + 2)/(4N₂)) the largest |z| is 3.0 (GUM-5 Ex. 1, pH), of the size expected
  for the largest of 48 comparisons. The program's 5 × 10⁶-trial runs with two other seeds give 0.004 0803 and
  0.004 0820 (GUM 0.004 083), so the deviation is on Suncal's single run.
- Suncal draws from the global NumPy generator (seeded) and needs PYTHONHASHSEED=0 for a reproducible table.
- The NIST Uncertainty Machine is a web service; its manual examples were reproduced earlier (task 22, F29, F33,
  F35). A scripted NUM benchmark is not possible offline; the article states the NUM comparison as the manual's
  examples.

## 2026-11-07 (session 21, continued) — Beta distribution; IUPAC 2024 Examples 4H and 5C

**D75. Beta input distribution.** `{"type": "beta", "alpha", "beta"}` or `{"mean", "sd"}` (method of moments) on
[lower, upper] (default [0, 1]). GUM: the mean and standard deviation of the distribution; MCM: G₁/(G₁ + G₂) with
gamma draws, the complement taken from G₂ when α > β so that values near the upper bound keep their digits.
Core test: the sample moments of beta(88 873.7, 7.11) and of a skewed beta on [2, 5].

- IUPAC 2024 4H (QUAM A1 cadmium standard, purity beta, volume triangular): GUM u 0.000 864 mg/mL, MCM mean 1.0027,
  s.d. 0.000 864, 95 % interval 1.001 03–1.004 37 mg/mL, half-width 0.001 67 (printed 0.0009 and 0.0017).
- IUPAC 2024 5C (NIST PS1 benzoic acid, beta(88 873.7, 7.11)): 0.999 920 ± 0.000 030 g/g; 95 % interval
  0.999 851–0.999 968 g/g, 0.069 mg/g below and 0.048 mg/g above the mean. This is the library's example of a
  bounded, asymmetric result (F97 item 6; the qNMR budgets of D69).

## 2026-11-07 (session 21, continued) — Consensus in the application

**D76. Consensus template and editor.** `uncertainty template --consensus` and the FFI template `consensus`
(six illustrative results with the K145 values); the app has a *New consensus value* card and a *Data* tab where the
results are pasted one per line (name, value, u, optional ν; tab, semicolon or space; decimal comma) and the
procedures are switched on (DL with bootstrap by default; Linear Pool, ML, Bayes with Gaussian or Laplace effects).
Linear Pool weights of participants that remain are kept when the list is pasted again. The library list labels
consensus projects "NICOB" and multicomponent projects "JCGM 106" (both were shown as "GUM + MCM"). Flutter test
for the parser (22 tests).

## 2026-11-07 (session 21, continued) — Guard bands from the test uncertainty ratio

**D77. `conformity.tur_method`: RSS, Dobbert, NCSLI RP-10, U₉₅.** TUR = (T_U − T_L)/2 / U₉₅ with U₉₅ = 2u; the
acceptance limits are ± GBF·(T_U − T_L)/2 about the centre; the rule (guarded acceptance or rejection) follows
from the sign of the guard band. Formulas as implemented in Suncal (`suncal.risk.guardband_tur`): RSS √(1 − 1/TUR²);
Dobbert 1 − M/TUR, M = 1.04 − exp(0.38 ln TUR − 0.54) (the status note had written M as the guard-band factor;
corrected); RP-10 1.25 − 1/TUR and U₉₅ 1 − 1/TUR, both 1 above TUR = 4. Dobbert's factor exceeds 1 above TUR ≈ 4.6
(limits outside the tolerance); Suncal does not cap it and neither does the program. The PFA-target and
4:1-equivalent methods are covered by `target_consumer_risk` (D27). The min-cost and minimax methods (Easterling
1991) are not implemented.

- Core test: the four factors at TUR = 1.5, 2, 3, 4, 6 against Suncal to 10⁻⁹.
- Library (`tools/generators/tur_guard_bands.py`): Dobbert at TUR 2.5, RSS at 1.5, RP-10 at 2.0, with a normal
  process of 95 % in-tolerance probability; TUR, GBF, acceptance limits and the global PFA and PFR agree with
  Suncal's PFA_norm/PFR_norm to 2 × 10⁻⁶ (an independent implementation of the global risks). Dobbert at TUR 2.5:
  PFA 0.69 % (1.18 % without a guard band), and at most 1.93 % over all in-tolerance probabilities, consistent
  with the method's 2 % design.
- Primary sources still to read for the article: Dobbert (2008) and the Z540.3 handbook; Delker & Peguero (2022)
  is cited as the Sandia reference.

## 2026-11-07 (session 21, continued) — EURAMET cg-18 H3.4/A and H3.4/B (30 t weighbridge in use)

The two weighbridge examples are now in the library (`cg18-h3-a-weighing-result`, `cg18-h3-b-weighing-result`,
`tools/generators/cg18inuse.py`), with the calibration points of H3.3 (substitution loads: u(m_ref) ⊕ u(L_sub) as
the reference term), the repeatability at 10 420 kg (s = 6.74 and 2.94 kg), the eccentricity test at 10 420 kg
(15 and 5 kg), ΔT = 40 K, the change in adjustment due to drift ΔE(Max)/(Max√3) with 30 kg (A) and 15 kg (B, mpe
at initial verification), tare from the test points, no buoyancy and no creep in use. Reproduced: α_W² = 62.133
and 25.333 kg², the temperature, drift, tare and eccentricity terms, β_W² (A), U(W) and U_gl(W) lines, minimum
weights 2169 kg and 1124 kg (printed 1123; SF = 1).

**F98. EURAMET cg-18 H3.4/A and H3.4/B — u(a₁) of the error function (class a, small).** The guide prints
u(a₁) = 3.627 × 10⁻⁴ (A) and 1.485 × 10⁻⁴ (B). The weighted fit through zero (C2.2-18b) with p_j = 1/u²(E_j) from
the H3.3 budgets gives 3.637 × 10⁻⁴ and 1.508 × 10⁻⁴, and the same with the printed u(E_j) or with the nominal
loads instead of the indications. a₁ itself agrees (9.377 vs 9.379 × 10⁻⁴; 4.286 vs 4.280 × 10⁻⁴). Not resolved:
possibly unrounded u(E_j) of the authors' spreadsheet. Effect: β_W² of B 1.967 vs 1.960 × 10⁻⁷ and its first-order
slope 6.128 vs 6.113 × 10⁻⁴; the minimum weights agree.

## 2026-11-07 (session 21, continued) — Monte Carlo with dependent outputs

**D78. Multivariate MCM with a singular output covariance.** Outputs that are exactly dependent (amount fractions
that sum to 1) made the Monte Carlo evaluation of JCGM 102 stop with an error, so IUPAC 2024 4B and the lead
project of Possolo & Meija had MCM switched off. Now the hyper-ellipsoid is reported as not defined (JCGM 102
9.4.2.3 note 2; k_p = NaN, JSON null, "not defined" in the reports and in the validation), while the estimates,
covariance, correlations, λ_max and the hyper-rectangle k_q are given as usual; the adaptive procedure skips the
k_p criterion. Singularity is detected by Cholesky pivots below 10⁻⁷ of the standard deviations, because the
covariance of the draws is singular only up to rounding. The reported region is then the hyper-rectangle.
Core test with three abundances; MCM enabled in `iupac2024-4b-silicon-isotopic-abundances` (u(x) 3.59, 2.20,
3.04 × 10⁻⁶, k_q 2.32) and `possolo-meija-2022-atomic-weight-lead-hipb1` (k_q 2.44).

## 2026-11-08 (session 22) — Pre-review of the review package and the first fixes

The review package of 2026-11-07 (Claude Doc "Bağımsız İnceleme Dosyası") came back with 22 observations. The
reviewer's own closing note says the pre-review was made with an AI assistant (Claude), like the development, so
it does **not** replace the independent review by a metrologist outside the project (outline §9); its item 14 asks
for at least ten class-c findings to be recomputed by a person who has not seen the code. The observations and the
response to each are in `docs/research/review-2026-11-08.md`.

**D79. Reporting defaults for accredited calibration (review items 9, 10, 21).** (1) Rounding of U: the default is
now `ea4_02` (EA-4/02 M:2022 6.3: ordinary rounding unless it lowers U by more than 5 %, then upwards) instead of
the MUPR rule, which looks only at the first discarded digit and can lower U by up to 5.7 % (0.10599 → 0.10;
EA-4/02 S2 58.52 mg → 58 mg, now 59 mg). Library reports that change: EA-4/02 S2 (59 mg), S4 (69 nm, as printed),
M3003 K.5 (0.31 mg, as in the project's note), Nordtest nitrate (8.5, 7.1 mg/L), cg-18 H3 (19 kg), the Possolo &
Meija tank (110 m³) and three others; no reference value changes, only the rounded statement. (2) The statement
gives "approximately 95 %" (whole per cent, 0.1 % above 99.5 %) instead of "95.45 %" and names the rounding rule.
(3) Software identity: `build.rs` records the git commit ("-modified" with uncommitted changes), its date and the
compiler; every report ends with them and with the SHA-256 of the project as evaluated (compact JSON; FIPS 180-4,
own implementation tested against the FIPS examples); the JSON results carry the same. The validation report
(TR title now "Yazılım validasyon raporu") gives the commit, compiler, the SHA-256 of every file and of the set,
and a separate section for the laboratory's own projects (`check --lab`). The version number stays 0.1.0 until
the first tagged release; the commit identifies the build. (4) EA-4/02 S2 text: 10 000.025 g, not kg.

**D80. TUR with k₉₅·u and Dobbert above TUR 4.59 (review item 5).** U₉₅ in the TUR is now k₉₅·u with k₉₅ the
97.5 % t quantile for ν_eff (model) or `dof` (1.960 for ν = ∞), the expanded uncertainty for 95 % of Z540.3;
`"tur_u95": "two_u"` keeps Suncal's 2u (the three library projects that reproduce Suncal set it). Dobbert's
M = 1.04 − exp(0.38 ln TUR − 0.54) is negative above TUR = 4.59 and the factor exceeds 1 (acceptance outside
the tolerance); it is now limited to 1 with a statement in the report unless `tur_allow_factor_above_one` is set
under an agreed decision rule (ISO/IEC 17025 7.1.3, ILAC G8). Tests: TUR 5.102 vs 5 at u = 0.1, k₉₅ = 2.776 for
ν = 4; Dobbert at TUR 6: 1.018 546 limited to 1. The primary sources (Dobbert 2008, the Z540.3 handbook) remain
to be read (documents-needed).

**D81. CCQM-K145 anonymised (review item 19).** The participants of `iupac2024-4f-ccqm-k145-nickel-bovine-liver`
are coded Lab 1–17 in the order of the guide's table; the project notes that the official KCRV and degrees of
equivalence are those of the final report in the KCDB. No degree of equivalence is attributed to a named
institute in the library, the guide or the article.

**D82. DL bootstrap as in the NICOB source (review item 4).** The NIST Consensus Builder is public (GitHub
usnistgov/NIST-Consensus-Builder, R; read in the scratchpad, not copied). `sampleFromTau2Dist.R` (Koepke &
Possolo, 2016-03-31) differs from the reading of the manual in two ways: (1) the gamma approximation of Q is set
at the untruncated τ²_M = (Q − (K − 1))/(S1 − S2/S1), which is negative when Q < K − 1, so that E Q equals the
observed Q, not at τ̂²_DL = max(0, τ²_M); (2) its variance is Var Q = 2(K − 1) + 4(S1 − S2/S1)τ² + 2(S2 − 2S3/S1 +
S2²/S1²)τ⁴, whose linear term is not that of Biggerstaff & Tweedie (1997) eq. (7), 4(S1 − 2S2/S1 + S3/S1²)τ².
`tools/research/review/nicob_tau2_sampler.py`: on Koepke's G the four combinations agree within 0.1 % (u 0.000 2790
vs 0.000 2787); on a homogeneous set (Q = 0.377 < 3) u is 0.0766 with the gamma at 0 and 0.0665 with NICOB's choice
(the fixed-effects u is 0.0647), a 13 % difference. The program now follows the NICOB source
(`q_gamma_approximation_nicob`; τ_k = 0 when the moments give no valid gamma, where NICOB would fail); the paper's
form is kept as `q_gamma_approximation`. The difference in the linear term of Var Q is a discrepancy between the
NICOB code and its cited source (noted for the article; it hardly matters when τ̂ > 0). The library values (G,
¹³⁷Cs, ⁹⁰Sr, IUPAC 4F) are unchanged within their tolerances. Test `bootstrap_with_q_below_n_minus_one`.

**D83. Convergence and prior sensitivity of the Bayesian grids (review item 3).** The θ grid τ = s·tan(πθ/2)
covers all τ ∈ [0, ∞), so no posterior mass lies outside it; with flat μ the likelihood falls as τ^−(n−1), so the
last cell carries a negligible mass for n ≥ 2. Test `bayes_grids_have_converged`: doubling the grid (10⁵ → 2 × 10⁵
points; Laplace 300² → 600²) changes the mean, s.d. and quantiles by less than 10⁻³ (Gaussian) and 10⁻² (Laplace) of
the posterior s.d. for iridium and G. Prior sensitivity (`tools/research/review/bayes_prior_sensitivity.py`), the
prior median multiplied by 1/4 … 4: iridium (3 results) u(M) 0.000 360 – 0.000 573 g/mol, median τ 0.000 33 –
0.000 75 (the published 0.000 44 at the stated prior); G (14 results) u 0.000 320 – 0.000 348, mean within
0.000 011. With three results the posterior of τ is driven by the prior, as the reviewer said; this is a property
of the model, and the article will show the table. The grid becomes impractical when the σ_j are also unknown
(dimension grows with the number of laboratories); that case (NICOB with finite ν_j) stays unimplemented (D68).

**D84. Findings by effect; errata; tolerance basis (review items 6, 7, 12, 13).** `docs/research/class-c-impact.tsv`
classifies the 29 findings with a class-c component by effect: c1 (a final printed y, U or outcome is not
reproduced at its printed digits), c2 (only intermediate values), m (difference of method or interpretation), e
(published erratum); with the status confirmed / independently checked (the reviewer's recomputations of F56, F79,
F81, F82) / partly erratum / open, the printed and recomputed values and |Δy|/U, |ΔU|/U. Result: 12 confirmed c1,
7 c2 (two of them partly corrected by errata), 5 m (F63, F69, F73, F79, F80), 1 e (F14), and 4 c1 open (F28, F29,
F85, F92), which the article will not count. Errata checked: QUAM_ERRATA_v2 (2019, downloaded from eurachem.org,
not committed); UfS:2019 errata 1.1, 1.4a were already used (F64, F65). Still to check for errata: M3003 ed. 6,
ISO 5725-3:2023, ISO 6143:2025, EA-4/02 M:2022, EMUE, TN 1900, GUM-5:2026. `tools/research/article_stats.py`
now writes the impact table, the tolerance basis of all 1754 compared values (931 at half a unit of the last
digit, 239 up to one unit, 238 Monte Carlo, 346 wider with a reason in the project; 112 projects have at least
one wider tolerance), the distribution of |Δ|/tolerance (median 0.26, 99 % 0.98; the values near 1 are reference
values at half a unit, e.g. 93.155 printed 93.16) and the counting chain sources → projects → matrix rows →
compared values → findings. The basis is inferred from the decimal form of the reference value; a per-value
label in the project files would be better and is left open. ISRD-06 T6 (F97): the integral ratio is printed as
1.5, which allows ±0.033 g/g on the purity; the estimate is now checked against the independent recomputation
(0.999 774 014 g/g ± 5 × 10⁻⁹) instead. The 5σ tolerance of Monte Carlo values (D15) is stated in the
validation report; a family-wise false-alarm rate is open (with 238 values at 5σ the chance of one false failure
is about 1.4 × 10⁻⁴ for normal errors).

**D85. The t convention in the report (review item 2).** When a model has inputs from repeated observations or
with a t distribution, the validation of the GUM result (JCGM 101 8) is followed by a note: the GUM framework uses
s/√n with ν = n − 1, the MCM samples t_ν(x̄, s²/n) (6.4.9.2; certificates 6.4.9.7) with standard deviation
√(ν/(ν − 2))·s/√n (not defined for ν ≤ 2), so a failed validation at small ν need not mean an error; TN 1900 and
the NUM rescale. An option for the TN 1900 convention is open (F31, F35).

## 2026-11-08 (session 22, continued) — Leave-one-out and bilateral degrees of equivalence

**D86. Leave-one-out and bilateral DoE of the DerSimonian–Laird procedure, as in the NICOB source (review item
20).** `consensus.leave_one_out` and `consensus.bilateral`; core `dl_degrees_of_equivalence`. Followed line by line
from DoEUnilateralDerSimonianLaird.R and DoEBilateralDerSimonianLaird.R (Koepke & Possolo, 2020-05-01):

- MRA: the bootstrap columns D_j,k = x_j,k − μ_k, centred on D_j (NICOB "sweep"); bilateral draws D_i,k − D_j,k,
  whose μ_k cancels.
- Leave-one-out: D*_j = x_j − μ̂_−j (DL of the others). Draws x_j + e_j,k − μ_−j,k with μ_−j,k = μ̂_−j +
  u*·t_{n−2}/φ(n − 2), u* = √(q*/Σw), q* = max(1, Σw(x − μ̂_−j)²/(n − 2)) (modified Knapp–Hartung, Röver, Knapp &
  Friede 2015), e ~ N(0, τ²_−j,k + u_j²) or σ·t_νj/φ(ν_j), τ²_−j,k drawn from the gamma approximation of the other
  results. U95 % about D*_j.
- φ(ν) = √(ν/(ν − 2)), and for ν ≤ 2 NICOB's approximation 1.5(1 − ¾(a − 4b(ν^−¾ − 1)/3))^−4/3, a = 0.4668994,
  b = −0.3998882. The Linear Pool now uses the same factor instead of refusing ν ≤ 2 (linearOP.R does so).
- NICOB notes in its own code: the manual says the same τ² sample serves all j, the code draws one per j (the
  program follows the code); the treatment of excluded participants is marked "AP CHECK" in the source (not
  implemented here: all results enter the consensus).

Independent check: `tools/research/consensus/nicob_doe.py`, a numpy port of the R code (10⁶ draws), on the two
datasets shipped with NICOB. The program (10⁵ draws) reproduces every D and B exactly and every U95 % within
0.7 % (⁶⁰Co) and 1.7 % (PCB 28, with ν = 2 for one result); the significant bilateral pairs agree: 4 (MRA) and 9
(leave-one-out) of 171 for ⁶⁰Co, 0 and 1 of 15 for PCB 28. Library: `nicob-co60-activity-leave-one-out-bilateral`
(manual §4.4, Exhibit 21: DL 7062 kBq, u 4, (7053, 7071); LP 7064, 29, (7012, 7127), all reproduced) and
`nicob-pcb28-sediment-degrees-of-freedom` (§4.2, Exhibit 7: DL 33.6 ng/g, (32.0, 35.2); LP 33.6, (31.4, 36.3)).
Participants coded as in D81.

**F99. NICOB manual Exhibit 7 — bootstrap u of PCB 28 (class n, Monte Carlo dispersion).** Printed u = 0.77 ng/g;
the program gives 0.751 and an independent numpy evaluation with 10⁶ replicates 0.752. Subsamples of 10⁴
replicates spread from 0.739 to 0.767, so the printed value is a Monte Carlo estimate with few replicates (as
F95, F96). The printed Linear Pool u, 1.55 against 1.51, is the sample standard deviation of a mixture with a
t₂ component, which has heavy tails; the interval (31.4, 36.3) is reproduced.

## 2026-11-08 (session 22, continued) — Results near a physical bound

**D87. Bounded outputs (review item 20, first NMI gap).** `model.output.lower_bound` / `upper_bound`. Sources:
Westwood et al. 2023 (IUPAC Technical Report, *Pure Appl. Chem.*, methods for the SI-traceable value assignment of
purity), 4.1.1: "a w ± U95 % symmetric interval can be treated as specifying the asymmetric interval
w − U95 % ≤ w ≤ 1 g/g", and 4.2.5, which lists as options a Monte Carlo evaluation "that enables constraining the
estimate to be between limits" (its refs. Toman et al. 2016 and Elster 2014) or truncating the reported interval;
Toman, Nelson & Lippa 2016 (*Metrologia* 53, 1193), §3, constrain a purity with a rectangular prior R(0, 1).
Implemented:

- GUM: y ± U (after the CMC rule) truncated at the bound it crosses; the statement reads y +U_U/−U_L at the
  decimal place of the rounded U, with a sentence on the truncation; a warning when y itself is outside.
- MCM: the draws outside the bounds are discarded (the posterior for a rectangular prior over the bounds, when
  the MCM PDF is taken as the likelihood-based state of knowledge); fraction inside, mean, standard deviation,
  symmetric and shortest intervals; at least 100 draws must remain.
- Report section, JSON `bounded`, expected values `gum.bounded_low/high`, `mcm.bounded_*`; app fields and results.

Library: `bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od` now has `upper_bound` 1 g/g. The statement reproduces the printed
999.8 +0.2/−1.5 mg/g (as (0.9998 +0.0002/−0.0015) g/g). The restricted Monte Carlo result (independent numpy, 10⁷
draws, `tools/research/qnmr/isrd06_bounded.py`): 62.0 % of the draws inside, mean 0.999 320 g/g, u 0.000 488 g/g,
shortest 95 % interval [0.998 394, 1]; the program (10⁶ draws, five seeds) agrees within 1 × 10⁻⁶ in the mean and
3 × 10⁻⁶ in the lower end. The two evaluations answer different questions: the truncated GUM interval keeps the
estimate 0.999 77 and only cuts the interval, the restricted MCM moves the estimate to 0.999 32 and narrows the
interval to 1.6 mg/g; for a purity this close to 1 the choice should be stated. Unit test: a normal input with
bounds [0, 1] against the truncated normal (mass 0.904 419, s.d. 0.238 75).

## 2026-11-08 (session 22, continued) — Degrees of equivalence of the Linear Pool and the Bayesian procedure

**D88. Linear Pool and Bayes degrees of equivalence (MRA, leave-one-out, bilateral).** `leave_one_out` and
`bilateral` now apply to every procedure requested; the report has a table "Degrees of equivalence by procedure",
the JSON `linear_pool_degrees_of_equivalence` and `bayes_degrees_of_equivalence`, the expected values `doe` →
`linear_pool` / `bayes` → `unilateral` / `bilateral`.

- Bayes with known u (NICOB DoEUnilateralBayes.R with σ_j = u_j, manual §A.5.2): D_j = x_j − μ̂ (posterior mean),
  draws x_j − μ_k + N(0, τ_k² + u_j²); leave-one-out with the posterior of the other results and the same prior
  (NICOB passes the same τ prior scale); U95 % symmetric about the mean of the draws, as in the code. (μ, τ) are
  drawn from the exact grid posterior (τ from the grid weights, μ | τ normal), so D is exact and only U carries
  Monte Carlo error. Not for Laplace effects.
- Linear Pool: the definitions of the manual §A.5.3 (see F100).
- Independent numpy check (`tools/research/consensus/nicob_doe.py`, 10⁶ draws) on ⁶⁰Co and PCB 28: U95 % within
  1.3 % (rechecked in session 24 after the fix of D112: within 1.04 %), Bayes D exact, Linear Pool D within the Monte Carlo error of the pool mean; significant bilateral pairs
  ⁶⁰Co: LP MRA 22/171 (port 22), LP LOO 0 (0), Bayes MRA 3 (3), Bayes LOO 7 (6, one pair at the edge); PCB 28 LP
  MRA 6/15 (6), LOO 2 (2). The ⁶⁰Co project now also checks the Bayes consensus value of Exhibit 21: 7062.03,
  u 4.59, (7053.1, 7071.4) kBq against 7062, 5, (7053, 7072) (MCMC, printed to the unit).
- Observation for the article: the MRA bilateral DoE of the Linear Pool contain only u_i and u_j, not the dark
  uncertainty, so with heterogeneous results 22 of 171 pairs of ⁶⁰Co are significant against 3–9 by the other
  procedures; the procedure matters as much for the DoE as for the consensus value (D66).

**F100. NICOB R code vs manual — Linear Pool degrees of equivalence (class n, methodological; for the article).**
The manual (§A.5.3) defines D_j,k = x_j + e_j,k − μ̂ (MRA), D*_j,k = x_j + e_j,k − x̃_−j,k (leave-one-out) and
B_ij = D_i − D_j from the same draws, e_j from N(0, u_j²) or a t. The code differs: DoEUnilateralLinearPool.R in
the MRA version subtracts the pool draws z_k instead of μ̂ (so U also contains the spread of the pool), and
DoEBilateralLinearPool.R always uses leave-one-out pools, draws e_j by resampling the centred pool of the other
results (the participant's own u_j does not enter) and sets B_ij = x_i − x_j. The program implements the manual.
Also noted in the code: "AP CHECK" comments on the treatment of excluded participants (D86).

## 2026-11-08 (session 22, continued) — PDF reports

**D89. PDF reports (review item 20, third NMI gap).** `uncertainty run … --format pdf` (or `-o report.pdf`),
`check --report validation.pdf`, the FFI option `pdf` (base64) and *Export report (PDF)* in the app. The Markdown
report of every project type is laid out on A4 by `project/src/report/pdf.rs`: headings, paragraphs with bold
runs, bullet and numbered lists, tables with the column alignment of the Markdown (column widths from the content,
cells wrapped, the header row repeated after a page break), rules, and a footer with the title and "page n of N".
Charts stay in the HTML report. Text is set in DejaVu Sans and DejaVu Sans Bold 2.37 (Bitstream Vera licence,
`project/fonts/LICENSE`), embedded whole and compressed (Type 0/CIDFontType2, Identity-H, with a ToUnicode map
so that the text can be searched and copied); about 0.75 MB per report. The fonts are not subset, because the
subsetting crate needs Rust 1.85 and the workspace declares 1.75. Dependencies: pdf-writer 0.12, ttf-parser 0.25,
miniz_oxide 0.8 (MIT/Apache/Zlib, pure Rust). The output is deterministic (no creation date; the generation date
is in the text), so the SHA-256 of a filed PDF can be recomputed. A library test checks that the font has a glyph
for every character of every library report in both languages. Signed packages (7.11.3) remain open: the PDF and
the project digest give integrity checks, not signatures.

**D90. Reporting the findings to the issuing bodies (review items 8 and 18; decision of the author, 2026-11-09).**
The findings are not communicated to the issuing bodies before submission. The c1 findings of the article
(Table 4) are sent to them as technical notes after acceptance, so that the notes can cite a peer-reviewed record;
the replies are recorded here. The pre-review had recommended reporting before or with submission, partly so that
the replies could confirm the findings; that check is taken over by the independent recomputation of the c1
findings by a person who has not seen the code, which remains required before submission. The article states the
decision (Section 7 and the note to Table 4). The F97 question to the BIPM qNMR team follows the same rule.

**D91. The article as one paper in four parts (decision of the author, 2026-11-09).** The software and its use in
laboratories are not split into separate papers (AQA, SoftwareX) but added to the article: Sections 2–3 the program
(methods; software design: architecture, project file, numerical implementation, quality assurance, interface),
4–5 the validation, 6–7 what the recomputation revealed, 8 the use in laboratories (typical uses with the ISO/IEC
17025 clauses, Table 5; software validation and technical records; statements; learning with the guide), 9
discussion, 10 availability. Measurement's guide for authors (as indexed; the ScienceDirect page refuses automated
access): research papers up to 30 pages, abstract at most 250 words, 3–5 highlights of at most 85 characters. The
abstract has 222 words; the review copy has 20 pages (A4, 1.5 line spacing, 11 pt). New Figure 2 is a screenshot of
the application; figures renumbered (1 architecture, 2 application, 3 findings by document, 4 procedures).
Supplementary material S1–S6 is written by `tools/research/article_supplementary.py` (S1: 286 projects, 1865
compared values, all within tolerance; S5: the validation report of the library, to be regenerated at the release
commit).

**D92. The program is named KAPSAM (decision of the author, 2026-11-09).** The working name *Uncertainty
Calculations / Belirsizlik Hesaplamaları* described the program but could not be cited, searched for or remembered
as a name. *Kapsam* is Turkish for *scope, coverage*: the coverage factor and the coverage interval of the GUM are
*kapsam faktörü* and *kapsam aralığı* in Turkish, and the program aims to cover the GUM suite and its sector guides.
It is written in capitals, uses only ASCII letters (no İ/I problem across keyboards and file names) and is read the
same way in English and the other European languages ("KAP-sam"); *Mihenk* (touchstone) was considered and set
aside because English readers tend to say "my-henk" and its capital form needs the dotted İ. A web search found no
software of that name in metrology. User-visible names change: the window and application titles, the Windows
executable (`kapsam.exe`, file description and product name), the Linux binary, the macOS bundle, the Android
and iOS labels, the command-line tool (`kapsam run …`, `kapsam check …`), the package names (`kapsam-windows-x64.zip`
…), the software name and citation in every report, `CITATION.cff`, `NOTICE`, the guides and the article title
(*KAPSAM: open-source software for measurement uncertainty, validated against the worked examples of the guidance it
implements*). Internal identifiers stay, so that existing files and builds keep working: the project file format
`uncertainty-calculations/project`, the Rust crate and Dart package names, the engine library
`uncertainty_ffi`, the application ids (`org.tubitak.ume…`) and the repository address. Entries above this one keep
the old name, as recorded at the time.

**D93. Suncal benchmark extended to every project Suncal can express (decision of the author, 2026-11-09).** The
first benchmark (D74) covered 107 measurement models without correlations or intermediate quantities. The script
now also passes correlated inputs as correlation coefficients (sampled only when the correlated inputs are all
normal, so that both programs draw from a multivariate normal), substitutes intermediate quantities into the output
expression, writes log10 as ln(·)/ln(10), passes curvilinear trapezoids exactly and beta inputs (GUM only) by their
standard deviation, and adds three new sections. Results (`docs/research/benchmark-suncal.md`, supplementary
material S4): 171 projects.

- 135 measurement models (24 with correlations or intermediates): first-order u to 6.4 × 10⁻¹², estimates
  identical except the two second-order projects of D74.
- 12 models with several outputs (JCGM 102): u to 1.6 × 10⁻¹⁴, correlation coefficients to 3.1 × 10⁻¹⁴.
- Monte Carlo: 79 comparisons (68 single-output, 11 multivariate), largest relative difference of the standard
  deviations 2.9 × 10⁻³, largest |z| 2.4, probability 0.75 for 79 independent comparisons. (The largest |z| of
  D74, 3.0 for GUM-5 Ex. 1, is now 0.6: Suncal's global generator is consumed in a different order.)
- 6 ISO 6143 calibrations (straight lines with York's fit, quadratic, exponential and power functions with
  orthogonal distance regression): parameters to 1.8 × 10⁻⁴ of their standard uncertainties; standard
  uncertainties of the parameters differ by up to 7.5 × 10⁻³ (NO₂ analyser, Γ = 1.27; ISO 6143 B.3 power
  function, Γ = 1.16). The cause is the definition of the covariance: York and ODR give the Gauss–Newton
  approximation (JᵀJ)⁻¹, KAPSAM propagates the inputs through the minimum with the full Hessian (ISO 6143 A.2,
  A.6, A.7), which adds terms proportional to the residuals and reproduces the covariances printed in ISO 6143
  (core tests). The Gauss–Newton covariance computed at KAPSAM's own solution agrees with Suncal's to 6.3 × 10⁻⁵
  (8 × 10⁻⁹ for the York fits). Not a finding; stated in the article.
- 18 conformity assessments: 32 conformance probabilities and global consumer and producer risks (normal and gamma
  processes) to 1.4 × 10⁻¹⁰.
- Not compared (listed in S4): ensembles and implicit models (6), Monte Carlo-only models (3), calibrations with
  covariances between points (2), lognormal measurement distributions, posterior specific risks and global risks
  with an uncertainty relative to the value (GUM-5 Ex. 10, EURACHEM compliance B3). Top-down evaluation,
  proficiency testing, precision experiments, control charts, sampling, consensus values and error functions have
  no counterpart in Suncal; an independent check of these by spreadsheet formulas is planned (next entry).

**D94. Spreadsheet verification workbooks (decision of the author, 2026-11-09).** For an independent check of
the project types that Suncal cannot express, and for laboratories that validate software against a spreadsheet,
KAPSAM writes an .xlsx workbook that recomputes a result with ordinary spreadsheet formulas
(`kapsam run p.json -o p.xlsx`; module `project/src/workbook.rs`, writer `project/src/xlsx.rs`, no new dependency).
KAPSAM writes the data of the project and the formulas, never a computed value except its own results on the sheet
Check, which are fixed numbers beside the spreadsheet's with the difference and a tolerance; the workbook asks the
spreadsheet program to calculate everything on opening. Excel functions introduced after 2007 are avoided (STDEV,
NORMSINV, BETAINV), so the formulas work in every version and in LibreOffice.

- Measurement models (first stage): inputs with x and u(x) from the distribution parameters (GUM 4.2, 4.3; JCGM
  101 6.4), the model and its intermediate quantities as formulas in one column at the estimates and two per input
  (xᵢ ± h, h = 0.001 u(xᵢ)), sensitivity coefficients as central differences (the idea of Kragten's spreadsheet,
  QUAM:2012 Appendix E, with a step small enough for 10⁻⁶ of u(y) and large enough for the rounding of 15 digits:
  0.01 u left a truncation error of 9 × 10⁻⁶ in the ratio model of NUM 3), the law of propagation with the
  correlation matrix, Welch–Satterthwaite and k from the t-distribution through BETAINV (non-integer ν, as KAPSAM's
  exact rounding; INT(ν) for truncation). ν_eff, k and U are not compared when higher-order terms are requested
  (KAPSAM's include them) or with correlations (Welch–Satterthwaite does not apply).
- `tools/research/spreadsheet_check.py` writes the workbook of every library project, recalculates it in
  LibreOffice Calc (headless, UNO) and reads the sheet Check: 135 model workbooks, 3257 compared values, all within
  tolerance (`docs/research/spreadsheet-check.md`). Largest ratio of difference to tolerance 0.44 (ν_eff of M3003
  K1).
- Proficiency tests (ISO 13528): median, MADe, nIQR (QUARTILE, the linear interpolation KAPSAM uses), Algorithm A
  with one column per iteration and the stopping rule of C.3.1 written as a formula (ROUND to three significant
  figures; the result is the first iteration that meets it, so the iteration count is compared too), the assigned
  value and u(x_pt) with the components of Formula (3), σ_pt, δ_E, z, z′, ζ and Eₙ; homogeneity (B.3 analysis,
  F₁ and F_m with CHIINV and FINV, √c) and stability (B.5). The Bayesian between-unit posterior is not covered.
- Precision experiments (ISO 5725-2): Forms B and C (rounded with ROUND when form_decimals is given), Mandel's h
  and k, the first Cochran test, Grubbs' single tests, and for the analysis of variance m̂, s_r, s_L and s_R from
  the retained results. Not covered: REML estimates, repeated Cochran and double Grubbs tests, critical values,
  relationships (8.5).
- Top-down (Nordtest, MUPR, ISO 21748): precision from series, groups, duplicates (pooled, normalised, mean range,
  with the selection by mean), control limits, stated values and level-dependent models, mean of n results on p
  days, conversions; bias components from their references (single-CRM formula included); further components; the
  combination at every level with ranges, multiplicities and the recovery correction; k. Recovery, the
  staggered-nested design and the ISO 21748 reproducibility component enter with KAPSAM's values, marked as such.
- Library: 189 workbooks (135 models, 9 proficiency, 6 precision, 39 top-down), 4678 compared values, all within
  tolerance on the first run of each new type.
- Laboratory use: `kapsam check <files> --workbooks DIR [--report FILE]` writes the workbook of every project
  that has one into DIR, and the validation report gains a section *Independent recalculation in a spreadsheet*
  listing each workbook with its SHA-256 and an empty column for the person who opens it in Excel to record the
  outcome (initials, date). CI: a job `spreadsheets` installs LibreOffice Calc and runs
  `tools/research/spreadsheet_check.py` on the whole library at every push; the table goes to the job summary.

**D95. Use of AI declared for Measurement; independence of the checks (decision of the author, 2026-11-10).**
*Measurement* desk-rejects papers whose research method is machine learning unless they meet four criteria; the
paper uses no such method, but its development with an AI coding assistant needs a precise statement. Changes:
Section 3.4 gains a bullet on development with the assistant (its output treated as unverified; correctness rests on
values it did not produce and on the review of every discrepancy against the source); Section 9 separates two
meanings of *independent* (another implementation, another person) and adds Table 6 (each check, who produced it,
what it shares with the program); the claim of an AI pre-review is removed (Sections 4.3, 9, 10); Section 10 keeps
only the availability; the Elsevier declaration on generative AI in the manuscript preparation process is added
before the CRediT statement. A cover letter (`docs/article/cover-letter.md`, Turkish translation
`cover-letter.tr.md`) states that the work is not a machine-learning study and why it is submitted to *Measurement*.
The recomputation of at least ten c1 findings by a metrologist who has not seen the code remains required before
submission; it is the one check that is independent of both the program and the assistant.

**D96. Ten papers from *Measurement* read and cited (decision of the author, 2026-11-10).** The author supplied the
full texts of the ten candidates of `docs/article/measurement-literature.md` (licensed; used locally, not in the
repository). Each was read in full and cited only for what it states:

- Greif, Schrepf & Richter 2006 (PTB): validation as objective evidence that refined requirements are fulfilled; the
  case study of a GUM-supporting program included the reproduction of the GUM appendix examples, with several
  hundred black-box cases checked for value, significant digits and rounding. Cited in Section 1 as the closest
  precedent of the library and in 8.2 for validation as the specification of testable requirements.
- Désenfant & Priel 2006 and 2017 (LNE): the four routes to an uncertainty (modelling, single-laboratory validation,
  interlaboratory validation, proficiency testing) and the statistical-model approach as a simplification of the
  measurement model; a model uncertainty much smaller than interlaboratory spread points to a missing component.
  Section 2.1.
- Shirono et al. 2016: the two causes of inconsistent interlaboratory data (unknown random effect, unskilled
  laboratories), different reference values from consistent-subset and random-effects models. Section 2.4.
- Lira 2011: Monte Carlo propagation through a least-squares fit and the use of the curve (Section 5, Limitations);
  its first example is CCQM-K25 PCB 28, the same six results as the library project
  `nicob-pcb28-sediment-degrees-of-freedom` (checked value by value against Lira's Table 1). Section 7.1 now compares
  the reference values of that comparison: KCRV 33.64 ng/g (u 0.60, the arithmetic mean), Lira 33.64 (0.35, equal
  weights) and 34.05 (0.26, inverse-variance weights, ν = 2 laboratory without weight), the program's
  DerSimonian–Laird with bootstrap 33.60 (0.75; τ̂ = 1.71, Q = 68.2 for 5 degrees of freedom) and Linear Pool 33.64
  (1.51): a factor 5.8 in u. Lira's residuals flag laboratories 5 and 6; the leave-one-out DoE flags Laboratory 6 and
  the bilateral DoE only the pair 5–6.
- Huang 2019: argues against the scaled and shifted t of JCGM 101 (s.d. undefined for n < 4; U up to 50.9 % larger at
  n = 4 in his example) and proposes N(x̄, s/(c₄√n)). Cited in 6.5 as a contested position, not as an endorsement.
- Carobbi 2017: ISO 13528 assigned values ignore u(x_i), weighted means use only u(x_i); a modified Algorithm A.
  Sections 7.1 and 9 (not implemented).
- Frahm & Wright 2023: |E_n| ≤ 1 is easily passed when the comparison uncertainty is large; conclusiveness criteria B
  and D, also for proficiency testing. Section 7.1 (not implemented).
- Pendrill 2006 and Forbes 2006: rules of thumb for target uncertainty and guard bands examined in economic terms;
  acceptance limits from the costs of decisions. Section 7.3 (the program reports the risks, it does not optimise
  costs).

The references are renumbered in order of first citation (they were not: [35], [36] came after [37]–[41]), now
[1]–[58] in both languages. Two earlier slips are corrected: Röver, Knapp & Friede 2015 was cited for "the books and
papers of Possolo" in Section 4.1 instead of Possolo & Bodnar 2018 and Possolo & Meija 2022; it is now cited in Section 2.4 for the modified Knapp–Hartung
adjustment of the leave-one-out degrees of equivalence (D86), where it belongs.

**D97. Monte Carlo propagation through the calibration fit (decision of the author, 2026-11-10).** The Suncal
comparison (D93) left open which of two linearisations of the ISO 6143 covariance is closer when the fit is poor:
KAPSAM's exact derivatives of the solution (full Hessian, A.2, A.6, A.7) or the Gauss–Newton approximation of Suncal's
York and ODR fits, which differ by up to 0.77 % in u(b). `tools/research/benchmarks/calibration_mc.py` draws the
calibration data of the eight calibration projects 10⁶ times from normal distributions with their uncertainties and
covariances, repeats the fit (Formula A.3, the program's weights; vectorised damped Gauss–Newton, a draw counts when
the step is below 10⁻⁹ u; no draw failed) and propagates the samples' responses (Lira, Measurement 44 (2011) 2156,
stages two and three). Result (`docs/research/calibration-mc.md`, part of S4):

- Where the linearisations differ (EMUE E3.3, Γ = 1.27; ISO 6143 B.3 power, Γ = 1.16) the Monte Carlo standard
  deviations of the parameters are within 0.15 % of KAPSAM's (standard error 0.05–0.06 %, from 20 batches), and
  0.6–0.8 % below the Gauss–Newton values. For the other fits all three agree within 0.12 %.
- All 13 sample contents: Monte Carlo u(x) within 0.15 % of KAPSAM's.
- ISO 6143 B.3 exponential function: r(b₀, b₁) = −0.999 997; the Monte Carlo s.d. of b₀ and b₁ are 1.4 % above both
  linear values and their means shift by 0.06 u: a nonlinearity of the parameterisation that no linearisation
  captures. The content of the sample, which depends on the well-determined combination, agrees to 0.04 %.
- Numerical differentiation of the refit (central differences, step 10⁻⁴ u, A.2) reproduces the program's
  derivatives to 1.2 × 10⁻⁴ (exponential function 3.9 × 10⁻⁴, where the step meets the ill-conditioning).

The program keeps the full-Hessian covariance; the paper reports the result in Section 5 and in Table 6. A Monte
Carlo option for calibrations in the program itself stays a limitation (Section 9).

**D98. UncertCalc cited (decision of the author, 2026-11-10).** B. Yalçınkaya, UncertCalc: a web-based tool for
measurement uncertainty calculation, J. Chem. Metrol. 19(2) (2025) 128–135 (TÜBİTAK UME; full text supplied by the
author). A Streamlit application with a top-down module (one-way ANOVA of multi-day QC data: repeatability,
between-day intermediate precision, an extra relative component, k = 2) and a bottom-up root-sum-of-squares budget,
for clinical laboratories; verified against Excel and an R script on randomly generated data. Cited in Section 1
(software landscape: another tool that joins top-down and bottom-up evaluation; the statement that no tool covers
the evaluation types of KAPSAM together still holds) and in Section 4.4 (spreadsheet and R recalculation as a check
of software; the KAPSAM workbooks make it on data with published results). References renumbered, now [1]–[59].

**D99. Bibliographic details checked against Crossref and DataCite (2026-11-10).** Every reference with a DOI was
compared with its Crossref record (volume, first page, year); the rest were looked up. Changes: [20] Delker, NCSLI
Measure 13(3) (2021) 45–60, doi 10.51843/measure.13.3.7, title as published ("open-source R software", also in the
OSTI record 1830511); [44] Westwood et al., Pure Appl. Chem. 95 (2023) 1–77 (the issue; online 2022); [24]
Bettencourt da Silva, Talanta 314 (2027) 130826 (issue of April 2027); DOIs added to DerSimonian & Laird, Knapp &
Hartung, Biggerstaff & Tweedie, Wichmann & Hill, Marsaglia & Tsang and Pennecchi & Harris. Possolo & Meija 2022 is a
DataCite DOI (SIM), record consistent. No [verify] mark remains. Standards and guides without DOIs are cited by
their edition.

**D100. Release v0.1.0 and its DOI (2026-11-10).** PR #11 merged into `main`; the author published the release
v0.1.0 on GitHub (the session cannot push tags). Zenodo archived it: version DOI 10.5281/zenodo.24118312, concept
DOI (all versions) 10.5281/zenodo.24118311, record checked through the Zenodo API (title, version, author). The
version DOI is cited as reference [60] in Section 10 and in the data availability statement, and is given in the
cover letter; `CITATION.cff` carries both DOIs and the release date, the README a DOI badge.

**D101. Figures 1, 3 and 4 redrawn (decision of the author, 2026-11-10).** Figure 1 shows the library pipeline, the
engine with its evaluation types and the documents they implement, the interfaces and the validation band (worked
examples and spreadsheets at every change; Suncal and the calibration Monte Carlo as release benchmarks). Figure 3:
(a) findings per document with the 13 confirmed c1 findings marked and counted, the documents without c1 and with
fewer than three findings combined; (b) the effect of each c1 finding, |Δy|/U and |ΔU|/U from
`class-c-impact.tsv` (0.6 % of U for F58 to 34 U for F28). Figure 4: (a) the 16 values of G, four consensus values
and the range of the fourteen procedures of Merkatas et al.; (b) the significant bilateral pairs of the ⁶⁰Co
comparison as 19 × 19 matrices by procedure and version (MRA: DL 4, Bayes 3, Linear Pool 22; leave-one-out: 9, 7, 0),
recomputed by the program from the library project. Sections 6.1 and 7.1 state the two new numbers (effect range of
c1; no significant pair with the Linear Pool in the leave-one-out version).

**D102. Elsevier's general author rules applied (2026-11-10).** The journal's own guide for authors
(sciencedirect.com) refuses automated access (HTTP 403) and was not read; it is to be supplied by the author. Applied
from Elsevier's general pages (elsevier.com, read 2026-11-10): (1) generative AI policy (updated June 2026): the
declaration is a separate section at the end of the manuscript immediately before the references (moved there, after
CRediT, competing interests and funding), and states that no image was generated with a generative AI tool (Figures
1, 3, 4 are drawn by `article_figures.py` from the program's output; Figure 2 is a screenshot); (2) highlights: three
to five, at most 85 characters, no acronyms or abbreviations, a separate Word file (rewritten: no GUM, ISO/IEC, c1);
(3) artwork: fonts Arial/Helvetica, Times, Courier or Symbol (figures now in Liberation Sans, metric-compatible with
Arial, embedded), printed lettering at least 7 pt at the final width (`save()` raises every text to 7 pt and checks
that the figure is not reduced at 190 mm: 7.1–7.3 pt), RGB, colour-blind-safe palette, vector PDF for line art and
600 dpi raster copies, figures as separate files and the captions listed at the end of the submission manuscript;
(4) graphical abstract (optional): 5:2, at least 1328 × 531 px at 300 dpi, text in the image, Arial-type font
(`graphical_abstract()`, 3984 × 1594 px); (5) CRediT before the acknowledgements/funding, as in the manuscript.
Journal-specific limits (abstract length, number of keywords, article length, reference style details) wait for the
guide.

**D103. The guide for authors of Measurement applied (2026-11-10).** The author supplied the guide (PDF of the
ScienceDirect page, 20 pages, read in full). Requirements and what was done:

- *Scope:* metrological terms as in the VIM and the GUM; the measurement context and a critical review of the state
  of the art; the cover letter must say how the paper meets these. VIM (JCGM 200:2012) cited in Section 2.1; the cover
  letter (EN, TR) gains a paragraph on the three requirements, the length and the anonymization.
- *Article type:* research paper of at most 30 pages: the anonymized manuscript has 26 (1.5 spacing, figures and
  tables in place, line and page numbers).
- *Double anonymized peer review:* title page and anonymized manuscript are separate files; the manuscript and the
  supplementary material must not identify the author. `tools/research/submission_package.py` now writes
  `title-page.docx` (author, affiliation with postal address, corresponding author, acknowledgements, competing
  interests, funding, CRediT, links for the editor) and `manuscript-anonymized.docx` (no author line, repository link,
  DOI or name in [65], no CRediT, funding or competing-interest sections), rebuilds S1–S7 with the identifying strings
  replaced (two library projects come from the institute's draft procedure), clears the document metadata, and scans
  every file the reviewers see (no match).
- *Abstract* ≤ 250 words without references (238), non-standard abbreviations defined (GUM spelled out; no committee
  codes); *keywords* 1–7 (7); *highlights* 3–5, ≤ 85 characters, file name with "highlights"; *graphical abstract*
  readable at 13 cm (now drawn at 13 cm, lettering ≥ 7 pt).
- *Math:* variables in italics: `article_docx.math_italics` sets quantity symbols with subscripts and single-letter
  symbols in a mathematical context in italics (outside code, URLs and the reference list).
- *Figures:* separate files named Figure_1–4, captions with a brief title; generative AI and figures: the captions of
  Figures 1, 3 and 4 state that they are drawn by a script written with an AI coding assistant.
- *References:* numbered by first citation; DOIs; journal abbreviations (LTWA); software cited as software with
  version, venue, date and identifier: Suncal 1.7.1 [21] beside its article [20], LibreOffice Calc 24.2 [50], NumPy
  [57] and SciPy [58] (the libraries of the recomputations), the program itself [65] as "[software]"; web references
  with the date of access ([24], [21], [50]).
- *Declarations:* generative AI statement in its own section immediately before the references (done in D102);
  CRediT, competing interests and funding on the title page; funding wording from the guide offered to the author.
- *Research data, option C:* data deposited (Zenodo), link to be given in the submission system.
- Spelling: British spelling throughout the body; "numpy" written NumPy.

Open for the author: the e-mail and postal address on the title page, acknowledgements, the funding sentence, the
competing-interest declaration, the recomputation of the c1 findings ("planned"), suggested reviewers.

**D104. Affiliation in English (decision of the author, 2026-11-10).** The 2026 papers with a TÜBİTAK UME affiliation
in Crossref (about 35: MDPI, IEEE, ACS, RSC, IOP, Wiley, AIP, De Gruyter, BIPM) use several forms; the English name
with the abbreviation and postal code 41470 is the most common, and the Elsevier guide asks for the full standard name
or the standard abbreviation. Used from now on: "TÜBİTAK National Metrology Institute (TÜBİTAK UME), TÜBİTAK Gebze
Yerleşkesi, Barış Mah. Dr. Zeki Acar Cad. No: 1, 41470 Gebze, Kocaeli, Türkiye" (title page, cover letter), short form
"…, 41470 Gebze, Kocaeli, Türkiye" (manuscript author line, CITATION.cff, .zenodo.json); the street address agrees with
the CPEM 2026 papers of the institute.

**D105. Suggested reviewers (decision of the author, 2026-11-10).** Five researchers whose recent work matches the
paper and who are not authors of a document with a c1 finding nor developers of the software compared: B. D. Hall
(MSL New Zealand), F. R. Pennecchi (INRiM), P. M. Harris (NPL), K. Shirono (NMIJ/AIST), G. Kok (VSL). C. Elster (PTB),
first proposed, died on 16 April 2025 (dedication in Wübbeler et al., Meas. Sci. Technol. 36 (2025) 085501) and was
replaced by G. Kok. The cover letters list names, affiliations and expertise; the e-mail addresses (taken from the
correspondence addresses printed in their open-access papers) are entered in the submission system and are not
stored in the repository.

**D106. First round of the independent recomputation of the c1 findings (2026-11-11).** A colleague of the author's
institute who has not seen the code filled in the blind form (`tools/research/c1_recheck.py`) for all 13 findings and
marked every printed value as reproduced. The form does not settle the findings either way:

- In nine rows the value was taken from a total or table printed in the document (F40 "matches the tabulated value",
  F42 "from Table B.12", F56 u_c 0.150 mg and F57 u_c 0.0245 dB as printed, F21 from the values entered in the budget,
  F77, F78, F82, F87 "reported value"), i.e. from the very values whose derivation each finding questions.
- In four rows the recomputer's own numbers disagree with the printed result and agree with the program: F28, the
  stated equation with the inputs written in the form gives 57.48 m/s (40.64 only without the factor 2); F39, the
  components written (2.4, 5.0, 3.0 %) give 6.31 % and U = 12.6 %, not 6.4 % and 12.8 %; F58, "2 × 40.7 = 81.4"
  written for the printed 81.5 nm; F90, "u(y) ≈ 4.47" written for the printed 4.7 nmol/mol.
- The remark on F87 describes the example wrongly ("influenza medication or related"; it is Pb in C279a).

Not counted as an independent check; the manuscript keeps "planned". A second-round form (`--round2`) names for each
finding the data to start from (raw data, equations, components) instead of the printed totals, without showing the
program's values, and asks for the intermediate steps.

**D107. Second round of the c1 recomputation; every finding rechecked from the documents (2026-11-11).** The second
form (starting data named) again marks all 13 printed values as reproduced, mostly without intermediate numbers. Each
finding was rechecked from the documents' own printed numbers, without the program:

- From the documents themselves (downloaded 2026-11-11): TN 1900 E4 states v = √(2ΔR_sT/p) with the inputs → 57.48 m/s
  (F28); GUM-5 1.9.1 itself computes pH_X = 7.0043 by the third interpolation and then prints 7.0109 (F78); GUM-5
  15.3.1 and Table 15.1 → independent numpy Monte Carlo 9.930 kN, 10.14 kN only with scale s = 1.75 kN (F77); GUM-5
  Table 4.6 with its correlation matrix and the sum-to-one constraint → u(CO₂) ≈ 4.1, and the printed r(CO₂, N₂) = −0.86
  is consistent only with ≈ 4.1 (F82); EA-4/02 M:2022 S13.9 lists the five observations (mean 49.999 536 mm, the text
  itself says 49.999 54) while the budget S13.10 uses 49.999 55 → d_X = 90.000 23 mm (F21).
- Arithmetic of printed components: F39 √(2.4² + 5.0² + 3.0²) = 6.31 %; F56 unrounded 0.1528 mg → U = 0.3055 → 0.31
  (0.150 only with the drift limit divided by 2); F57 formula E5 → 0.0170 dB; F58 2 × 40.7 = 81.4 and the model gives
  41.0 nm → 82 nm; F87 RMS 3.49 % (mean 3.27 %) → U = 2.87 mg/kg; F90 the form's own 4.47 nmol/mol.
- F40: ISO 5725-3:2023 states "If the difference is negative, set the estimate = 0"; the printed s_R keeps the negative
  component that the same table sets to zero for s_I(T). F42: the form's 0.013283 follows from the parameter
  uncertainties the standard itself calls incorrect, as the finding says.

All 13 stand. The form is not counted as an independent recomputation; in four rows its own arithmetic does not give
the printed value. Evaluation for the author: `c1_ikinci_tur_degerlendirme.docx` (not in the repository).

**D108. Third recomputation of the c1 findings: twelve reproduced independently, F40 reclassified (2026-11-11).** A
second metrologist of the author's institute, not an author and without having seen the code, filled in the
second-round form for all 13 findings from the documents alone (hand calculation, Python, NumPy/SciPy, Monte Carlo up
to 10⁷ trials), with every intermediate step written down. Unlike D106 and D107, every row starts from the data:

- Reproduced (recomputed value = the program's): F21 90.0002 mm (90.000 23), F28 57.48 m/s (u = 0.35), F39 12.6 %,
  F42 0.014246 µmol/mol (own errors-in-variables fit, parameters, S_res and Γ as printed; the printed 0.013283 follows
  from the printed B_LEAST covariances), F56 0.31 mg (u_c 0.1528), F57 0.040 dB (E5 → 0.0170 dB), F58 82.1 nm, F77
  9.91 kN (normal) / 9.93 kN (t, ν = 5), F78 7.0043, F82 3.91 → 4 µmol/mol (20-input LPU), F87 2.87 → 2.9 mg/kg, F90
  4.47 nmol/mol.
- **F40 reclassified c1 → m.** At level 6, MS₀ = 617.90, MS₁ = 55.39, MS_e = 91.11 (× 10⁻⁶) give s₍₁₎² = −26.79 × 10⁻⁶.
  The printed s_R = 15.962 × 10⁻³ % = √((MS₀ + MS₁ + MS_e)/3) is the formulas applied as printed and equals the
  REML estimate with variances ≥ 0 (pooling MS₁ and MS_e, s_r = 8.559 × 10⁻³); 16.331 is s₍₁₎² = 0 throughout (the
  program, D41); 16.781 is s₍₁₎² = 0 in the sum only (ISO 5725-2, 8.4.5.4). The printed row mixes treatments
  (s_I(T) = s_r = 9.545 with s₍₁₎² = 0, s_R with it kept), but its s_R is one of three defensible values, so the finding
  concerns the consistency of the row, not a final result. D107 ("all 13 stand") is corrected accordingly. The 2023
  Table J.2 transcription errors (D41) are confirmed independently: levels 3–5 of Table J.5 cannot be reproduced
  from the 2023 data.
- **F90:** the printed (rounded) inputs of (E3.3.19) alone allow u = 4.14–4.78, since three large terms cancel;
  with the unrounded parameters (full WTLS, `tools/research/emue/e33_wtls.py`) u = 4.52. The finding stands with
  4.5; Table 4 gives both values.
- **F87:** the 3.3 % called root mean square in the text is the arithmetic mean (3.28 %); RMS 3.50 %. With x = 27.8
  (text) U = 2.87 → 2.9; with x_obs = 27.29 (Table E3.4.2) 2.82 → 2.8. The finding stands; its effect is at the last
  digit.
- Further remarks of the recomputer (not yet checked by the program): F82, the printed 5 follows with u(r) = 2 mg,
  the weighing uncertainty of the 2024 committee draft; F77, 10.1 follows with the s.d. of a single observation
  (1.8 kN) instead of the mean's; F57, 0.022 dB follows with s11b = 0.09; F56, 0.150 also follows if one of the two
  rounding terms is left out (the same sum as the drift limit divided by 2); F40, laboratory 2 is a Cochran outlier
  at level 6 (C = 0.585 > 0.496) and was kept (s_R = 14.7 without it); F21, δl_P enters the budget S13.10 with the
  wrong sign.

Counted as the independent recomputation of the c1 findings: 12 confirmed c1 (was 13), 6 m (was 5). Manuscript:
abstract, Section 6.1 (new paragraph on F40), Table 4 (F40 removed, F87 and F90 notes, the "before submission" note
replaced), Section 9 and Table 6 (no longer "planned"; a review outside the institute has not been made), Figure 3b,
highlights and cover letters. The form and its evaluation stay outside the repository.

**D109. The recomputer's further remarks (D108), checked from the documents (2026-11-11).**

- **F40, laboratory 2 at level 6.** With the 1994 data (level 6 is the same in 2023) and laboratory 20 excluded,
  Cochran's statistic on the day-1 differences w(1) is 0.585 for laboratory 2 against the critical values 0.496 (1 %)
  and 0.403 (5 %) for p = 19, n = 2 (ISO 5725-2:1994, 8.1); on w(2) it is 0.479 (straggler). ISO 5725-3 C refers to
  ISO 5725-2 7.3, under which an outlier is discarded "unless the panel for good reason decides to retain" it; the
  example gives no reason, and laboratory 2 is excluded at level 2 only. Without it: s_r = 6.318, s_R = 14.725 ×
  10⁻³ % (formulas as printed; 14.832 with s₍₁₎² = 0 throughout, 14.964 in the sum only), and s₍₁₎² is still
  negative (−7.1 × 10⁻⁶). Recorded with F40 (class m).
- **ISO 5725-3:2023 Table J.2 against the 1994 data: 15 entries**, 12 at level 3 (the second day-1 result of
  laboratories 7–20 repeats the first; for laboratories 11 and 16 the two were equal already), 1 at level 4 (lab 18)
  and 2 at level 5 (labs 2 and 3). D41 ("14 at level 3") and the library note ("one each at levels 4 and 5") are
  corrected. With the 2023 data, level 5 gives s_r = 14.715 × 10⁻³ % against the printed 6.237.
- **F82.** With u(r̂_j) = 2 mg for all three weighings, u(y_CO₂) = 4.50 → 5 and u(y_N₂) = 5.18 → 5, so the printed 5
  could come from an earlier weighing uncertainty (the recomputer names the 2024 committee draft, which is not
  available here). Table 4.1 gives r̂₀ = −152.0247 g and r̂₂ = 717.2743 g (printed −152.0243, 717.2740) and
  u_rpt = s/√7 = 0.18, 0.40, 0.42 mg (printed 0.4, 0.7, 0.6 mg), as already recorded with F82; with these,
  u(y_CO₂) = 3.91.
- **F57.** E5 with s11 = 0.09 for one of the two coefficients (0.05 printed) gives 0.0219 dB, the printed 0.022 dB. One
  possible cause; not identifiable from the example.
- **F56.** Leaving out one of the two rounding terms (0.05/√3 mg) gives Σu_i² = 0.0225 mg², exactly as the drift limit
  divided by 2 does, since (0.1/√3)² − 0.05² = (0.05/√3)². D107's "only with the drift limit divided by 2" is
  corrected: the cause cannot be identified.
- **F77.** A t input with scale 1.75–1.8 kN (the standard deviation of a single observation) gives 10.05 (normal) to
  10.15 kN (t, ν = 5), as in D107.
- **F21.** EA-4/02 S13.6 gives δl_P = −0.004 µm, the budget S13.10 enters +0.000 004 mm; the printed 90.000 25 mm is
  40.0007 + 49.999 55 + 0.000 004. Already part of F21.
- **F39.** With U = 12.6 % the flour interval of Table C.5 is 80 to 282 CFU (log₁₀ half-width 0.2742), not 79 to 285.
- **F28.** The printed u(v) = 0.25 m/s and interval 40.15–41.14 m/s keep the relative uncertainty of 0.605 %, so they
  scale with the estimate; no separate finding.
- **F87.** Equation (E3.4.18) itself uses 0.033 for the root mean square and rounds u′ to "about 5 %"; the reported
  27.8 mg/kg is not the mean 27.29 of Table E3.4.1. Already in the note of F87 (D108).

None of these changes a class or a count. The manuscript adds the Cochran remark to the paragraph on F40 (Section
6.1).

**D110. Revision after an internal review by three metrologists (2026-11-11).** Three reviewers (uncertainty methods;
standards and accreditation; originality and suitability for the journal) read the submission package. Decisions of
the author: one focused paper (the consensus, bound and guard-band discussion of former Section 7 is left for a later
paper); the findings are reported to the issuing bodies after acceptance (D90 kept); the project files keep the
numerical data of the examples; the recomputer's tools are stated as recorded (whether an AI assistant was used was
not recorded). Changes:

- *Counts and supplements.* The findings are partitioned by class and status (Table 5 of the manuscript). F87 moved
  from c1 to c2: with the combined relative uncertainty rounded to 5 % as the example does, the printed U = 2.8 mg/kg
  follows also from the correct root mean square, so a reasonable rounding path gives the printed result (the c1
  criterion now requires that none does). F97 (BIPM qNMR budgets) gained an open c1 part for the combined uncertainty
  of ISRD-01 Table 6 (3.96 against 4.10 × 10⁻⁴). Result: 30 class-c findings, 24 settled (11 c1, 8 c2, 4 m, 1 e),
  6 open. F90: |Δy|/U 0.011, |ΔU|/U 0.038 (unrounded parameters); F58 recomputed 82.1 nm. S2 is regenerated with the
  printed and recomputed values as text (trailing zeros kept), titles without working notes, and every status
  defined; working notes were removed from the impact table (F28, F29, F79, F92).
- *Sources.* Primary source of each project (`tools/research/sources.py`): 37 documents — 25 guidance documents and
  standards (256 projects), 5 BIPM ISRD reports (9), 5 papers and a book (19), 1 unpublished procedure (2). S1 gives
  the kind of source, the basis of the reference values (printed: 207 projects, 1239 values; printed with Monte Carlo
  or MCMC estimates: 21, 149; partly or wholly independent recomputation: 58, 477) and the reference statement of
  each project.
- *Guard-band projects.* The reference values of the three TUR projects were Suncal's output (circular with the
  Suncal comparison). They are now computed from the published guard-band formulas and the JCGM 106 risk integrals in
  SciPy (`tools/research/conformity/tur_risks.py`); the values are identical, the reference statements changed.
- *Monte Carlo tolerances.* `tools/research/mc_seeds.py` runs the 61 projects with Monte Carlo checks with ten other
  seeds (2810 comparisons). First run: 20 outside tolerance from 7 values (GUM-5 E14 and E15, NUM §3 and §8,
  TN 1900 E8, JCGM 102 9.4 correlations in two projects), whose tolerances had been set from the program's own seed;
  in GUM-5 E14 the Monte Carlo mean differs from the printed GUM estimate by the nonlinearity of the model. Their
  tolerances were reset to half a unit in the last digit plus five standard deviations over eleven seeds (generators
  gum5.py, num.py, tn1900.py, jcgm102.py); second run: 0 outside, largest |Δ|/tolerance 0.98. The values with
  |Δ| > 0.9 tolerance (74, 51 of them at half a unit) are listed in `docs/research/near-tolerance.md` (S3).
- *Suncal comparison.* 171 comparisons of 166 projects (five EMUE E2.2 projects compared as models and as conformity
  assessments); Δy/u shown as undefined where the first-order u is zero; the list of projects not compared completed
  (multicomponent conformity, an acceptance-limit-only decision rule) and its duplicate merged; the probability of the
  largest |z| (2.38, P = 0.75) is called a guide because many comparisons are variants of one model.
- *Spreadsheet workbooks.* 3684 of the 4678 compared values are computed by spreadsheet formulas; 994 repeat an input
  or a fixed factor (a cell that is a constant or a chain of bare references to one). Pipes in quantity names no
  longer break the S7 table.
- *Validation report.* Consensus projects are reported as "consensus (NIST Consensus Builder)" and the multicomponent
  project as "JCGM 106, multicomponent" instead of "JCGM 106"; the note on tolerances states the seed rule.
- *NICOB port.* The program's degrees of equivalence (10⁵ draws) vary between seeds by up to 0.8 % (PCB 28) and
  0.44 % (⁶⁰Co) in U95 (relative standard deviation over 20 seeds); the differences from the port (1.7 % and 0.7 %)
  are of that size.
- *Manuscript.* Title "verified" instead of "validated"; related work (Greif and Schrepf 2013, TraCIM, GUM Workbench,
  MUkit and ISO 11352, QUAM errata); former Section 7 removed with Figure 4; Sections 3 and 7 (former 8) shortened;
  Section 5 as a table of comparisons with the interpretations corrected (the calibration differences do not follow
  Γ; the Monte Carlo propagation lies closer to the full-Hessian values, about three standard errors from them);
  Section 6.3 softened and its counts corrected (7 of 11 ¹H tables of six reports, nine of them in the library);
  F79 presented as open; "pre-registered" removed; the ISO/IEC 17025 clauses corrected (7.11.6 added, 7.8.6 and the
  proficiency-testing provider row corrected, ILAC G8, G17, ISO/IEC 17043, ISO 17034, ISO 33405 added); Appendix A
  with the equations not stated in the guidance; a Conclusions section; references 65 → 83, all new ones checked
  against Crossref or the publisher's page except the two software entries (GUM Workbench, MUkit) and the standards.
- *Figures.* Figure 3a counts each finding once by its most severe class; Figure 3b axis in units of U; Figure 2
  without the clipped column and chart; the AI sentence removed from the captions (it is in the declaration).
- *Library.* Bettencourt da Silva (2026) cited as Talanta 314 (2027) 130826.

**D111. Tools of the recomputation of the c1 findings (2026-11-11).** Asked by the author, the recomputer (D108) stated
that an AI assistant of the Claude family (Anthropic), the same family as the coding assistant of this project, was
used in the recomputation. The manuscript now says so (Section 8, Table 8, the declaration on generative AI and the
cover letters): the recomputation is independent of the program's code and of its author but not of the assistant, so
a misreading of a document that the assistant tends to make could be common to both. Against this, most c1 findings
rest on a few lines of arithmetic with the document's own printed numbers (Table 6); for F78 the document prints the
correct intermediate value itself, and F28 was reported independently by Delker (2021). A recomputation without an AI
assistant, or by a metrologist outside the institute, would remove this common mode.

## 2026-11-11 (session 24) — Second round of the internal review

**F101. ISO 5725-3:2023 Table J.2 — transcription errors at levels 3–5 (class c, c1).** Split from F40 at the
reviewers' request (D112). Table J.2 of the 2023 edition differs from Table D.2 of the 1994 edition in 15 entries: 12 at
level 3 (the second day-1 result of laboratories 7–20 repeats the first, which for laboratories 11 and 16 changes
nothing), 1 at level 4 (laboratory 18) and 2 at level 5 (laboratories 2 and 3). Table J.5, which the 2023 edition carries
over unchanged, is reproduced exactly from the 1994 data with the stated exclusions, but not from the 2023 Table J.2:
level 3 s_r 0.652 against 1.739, level 5 s_r 14.715 against 6.237 (× 10⁻³ %). The printed results are therefore not
reproduced from the document's own data; the cause is the data table, not the evaluation. Confirmed independently by
the recomputer of D108 (levels 3–5 of Table J.5 cannot be reproduced from the 2023 data). No corrigendum to ISO 5725-3:2023 was found (D112).

**D112. Second round of the internal review (2026-11-11).** The three reviewers of D110 read the revised package and
asked for nine changes before submission. Decisions and changes:

- *c1 criterion (R1.1).* Rewritten: the inconsistency, once corrected, changes the final printed result at its
  printed digits even with the document's own rounding. Applied to every row: F42 and F58 stay c1; **F90 is open**,
  because the printed (rounded) inputs of (E3.3.19) allow 4.14–4.78 nmol/mol, which contains the printed 4.7; F101
  (below) is c1. Settled c1: 11 (F21, F28, F39, F42, F56, F57, F58, F77, F78, F82, F101); open c1: F29, F85, F90, F92,
  F97.
- *Statuses (R1.8).* Every finding that the recomputer of D108 reproduced is now "independently checked" (all 11
  settled c1, F40, F80, F81, F87).
- *F101 (R2.2)* split from F40 (above): ISO 5725-3:2023 Table J.5 is not reproducible from its own Table J.2.
- *F79 closed as m (R1.3).* GUM-5 eq. (2.3) was transcribed wrongly in Appendix A.2 of the D110 draft
  ([Σⱼ r c u]² instead of Σᵢ Σⱼ r²c²c²u²u²/νᵢ); with the verbatim form the 3.6ν example holds. Eq. (2.3) is
  Willink's (2007) generalisation for correlated estimates with independently estimated variances (his Section 4.2);
  for estimates from the same simultaneous observations his Result 1 (Section 4.1) combines them into one component
  with n − 1 degrees of freedom, as summarised by Willink and Hall (arXiv:1311.0343, 2013, not published in a
  journal). The example applies the rule for independent variance estimates to estimates from the same data: a
  difference of method, not an inconsistency.
- *Variance of Q (R1.4).* Var Q = 2 tr[(AΣ)²] = 2(K − 1) + 4(S₁ − S₂/S₁)τ² + 2(S₂ − 2S₃/S₁ + S₂²/S₁²)τ⁴ is exact; the
  NICOB manual attributes the exact moments to Biggerstaff and Tweedie (1997). The linear term
  4(S₁ − 2S₂/S₁ + S₃/S₁²)τ² that D82 and the D110 draft attributed to their eq. (7) is not the variance of Q, and the
  attribution is withdrawn (the paper was not available here to reread). The public function
  `q_gamma_approximation` had that wrong term; it now has the exact variance, with a unit test against the matrix
  expression. It was not used by any evaluation: the bootstrap uses `q_gamma_approximation_nicob`, which already had
  the exact variance at τ²_M, so no result of the library changes. The "option" of the published form named in the
  D110 draft did not exist and is removed from the text.
- *Monte Carlo tolerances (R1.5).* The seeds 101–110 had set the seven tolerances reset in D110, so their rerun was
  not a test. Seeds 201–210 found 31 comparisons outside tolerance (24 from the Monte Carlo standard deviations and
  correlations of JCGM 102 9.4, t₃ inputs, whose sample standard deviations have no finite variance: u(R) 0.249 Ω
  with seed 207 against 0.13; 7 from tolerances narrower than five standard deviations) and 3 seed-dependent GUM
  validation outcomes (NUM §10), which mc_seeds.py did not count because their tolerance is 0. New script
  `tools/research/mc_tolerances.py` applies one rule to every Monte Carlo value from the project's own seed and
  101–110: tolerance ≥ |mean − reference| + 5 s.d. (69 tolerances widened, at most 2.9 times); 14 values withdrawn
  as reference values (MCM u and r of JCGM 102 9.4, both projects; MCM u of TN 1900 E13 with t₃ inputs, two
  projects; the GUM validation outcome of NUM §10 and TN 1900 E1, which fails with 6 and 1 of 31 seeds). Test with
  the twenty seeds 201–210 and 301–310, not used to set any tolerance: 0 of 5340 comparisons outside, largest
  |Δ|/tolerance 0.93. mc_seeds.py now counts a changed yes/no outcome as a failure. The generators keep the tolerances
  they were written with: run mc_tolerances.py after regenerating a Monte Carlo project.
- *Errata (R2.1).* The errata, corrigenda and amendments of every document with a settled c1 finding were checked on
  2026-11-11 (`docs/research/errata-check.md`): none found except the known QUAM errata; ISO 6143:2025/DAmd 1
  (B_LEAST performance for nonlinear functions) is at the DIS stage and its text was not consulted. Reporting to the
  issuing bodies stays after acceptance (D90); the reviewers' suggestion to inform JCGM-WG1 of the GUM-5 findings
  before its November meeting was not taken up.
- *Version (R2.3).* 0.1.1 (Cargo, app, CITATION.cff, release notes `docs/release/v0.1.1.md`). Every supplementary
  file now states the build it was generated with; the generator refuses a modified tree. The article cites the
  concept DOI until Zenodo assigns the version DOI.
- *S6 (R2.4)* regenerated from the guide (`app/tool/write_learning_docs.dart`), so the guard-band projects show
  their current reference statements.
- *Unpublished procedure (R2.5).* Its two projects moved to `laboratory/` (284 projects, 36 sources); the header of
  the supplementary workbooks no longer names the institute.
- *Source kinds (R1.6).* The three guard-band projects are "test cases constructed for this work" (253 projects and
  1672 values from guidance documents and standards); the reference-value basis is stated per project, so 477 is an
  upper bound for values resting on a recomputation.
- *Recomputation record (R1.2, R2.6, R3.2).* New supplement S8 (`docs/research/recomputation-record.md` with the
  errata check): printed, program and recomputed value of the 13 findings of the form and F101, the start given for
  each, and the derivation of every settled c1 finding. The sentence "every recomputed value agreed" is replaced by
  the three exceptions (F40, F82, F90). The text says what the form records: the recomputer wrote every step, the
  author checked each step against the document; which steps the AI assistant performed is not recorded.
- *Smaller items (R1.7, R3.1, R3.3, R3.5).* NICOB port: DL U95 within 0.7 % (⁶⁰Co) and 1.7 % (PCB 28), Linear Pool
  and Bayes within 1.04 % (rechecked by laboratory code, after a first script compared unordered entries); Figure 3a
  counts the 97 findings that concern a document; Figure 3b shows the 10 c1 findings with a printed U; S4 prints
  max |z| to two decimals; the cover letter takes its subject line from the manuscript title and keeps its lists;
  graphical abstract 253 projects and 1672 values; highlight 4 names the 11 findings; references: GUM Workbench
  Professional 2.4 and MUkit 3.0.12 with their addresses, ISRD-03 (potassium hydrogen phthalate, Rapport
  BIPM-2018/05), Willink and Hall 2013 added, Forbes 2006 and Frahm and Wright 2023 removed (82 references); the
  report numbers of ISRD-06 and ISRD-07 corrected to Rapport BIPM-2019/03 and 2019/04 in the article, the generator
  and the four projects; Section 2.5 states that outlying laboratories are identified by the leave-one-out degrees
  of equivalence and none is excluded automatically.
