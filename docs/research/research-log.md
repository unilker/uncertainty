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
reports +0.10 mg.

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
