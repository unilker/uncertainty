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

- **Q9.** Offer a Kragten (one-sided difference) view next to the exact
  sensitivities, for comparability with QUAM-style spreadsheets?
- **Q10.** Warn when a first-order sensitivity vanishes at the estimate but
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
- **Q15.** ISO 21748 / ISO 5725 reproducibility-based budgets (s_R with
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
