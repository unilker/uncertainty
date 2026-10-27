# Literature and software review

Status column: **read** = primary document obtained and the relevant clauses
read; **listed** = existence and scope confirmed (publisher page or
abstract), full text not yet read; **to verify** = recalled, not yet checked.
Nothing marked *to verify* may be cited in the article until it is checked.

## 1. Normative and guidance documents

| Document | Scope | Use in this project | Status |
|---|---|---|---|
| JCGM 100:2008 (GUM) | uncertainty framework, LPU, Type A/B, ν_eff, k | `gum` module; validation H.1 | read |
| JCGM 101:2008 (GUM-S1) | Monte Carlo propagation of distributions | `rng`, `distribution`, `mcm`; validation clause 9 | read |
| JCGM 102:2011 (GUM-S2) | multivariate output quantities | implemented (D42) | read; examples reproduced (F50, F51) |
| JCGM 104:2009 | introduction to the GUM documents | guide | obtained (not yet read) |
| JCGM GUM-1:2023 | Part 1: Introduction (ISO/IEC Guide 98-1) | guide | listed |
| JCGM GUM-6:2020 | Developing and using measurement models | intermediate quantities, guide | obtained (not yet read) |
| JCGM 106:2012 | conformity assessment, decision rules | `conformity` module and projects; validation clauses 7–9 (F45) | read |
| JCGM 200:2012 (VIM 3) | vocabulary | TR/EN glossary | obtained (not yet read) |
| EURACHEM/CITAC CG4 QUAM:2012 (3rd ed.) | uncertainty in analytical chemistry (bottom-up, Kragten spreadsheet, examples A1–A7) | chemistry module, validation A1–A7 | read (A1–A7, App. E.4) |
| Eurachem/CITAC MUPR 1st ed. (2026) | uncertainty from in-house precision and recovery data | `topdown` module; validation Ex. 2, 3, B1–B6, C1 | read (complete) |
| Nordtest TR 537 ed. 4 (2017) | top-down for environmental labs (R_w, bias, RMS bias) | `topdown` module; validation §3–8, App. 4–7 | read |
| Nordtest TR 569 ed. 6 (2026), Handbook of Internal Quality Control | control charts: X, R, r%; statistical and target limits; daily rules; long-term review | `qc` module and `control_chart` projects; validation Table 1, Chapter 7, Examples 1–12 (F52–F54) | read (complete) |
| UKAS M3003 ed. 6 (2024), The Expression of Uncertainty and Confidence in Measurement | calibration budgets as used by accredited laboratories; coverage factors for dominant Type A and Type B terms; ranges of values | library projects K.1–K.11, C.5, L.3; `dominant_rectangular` coverage factor (D46, D47, F55–F61) | read (complete) |
| EURAMET Calibration Guide No. 19 v4.1 (2025), gravimetric volume calibration | volume of glassware by weighing water: Tanaka density, air buoyancy, meniscus, repeatability | library projects for Examples 1 and 2 (D48, F62) | read (complete) |
| EURAMET Calibration Guide No. 18 v4.0 (2015), non-automatic weighing instruments | calibration of balances: repeatability, eccentricity, errors of indication, buoyancy, convection, minimum weight | library projects for Examples H1–H4 (D49, D51, D52); Appendix C error function implemented | read (Appendices C, H) |
| ISO 11352:2012 | water quality — uncertainty from validation and QC data | top-down module | to verify |
| ISO 21748:2017 | use of repeatability, reproducibility and trueness estimates | top-down `reproducibility` precision; validation Annex C | read |
| ISO 5725-3:2023 | intermediate precision, staggered-nested designs | top-down `staggered_nested`; validation Annex J | read |
| ISO 5725 (parts 1, 2, 4, 6) | accuracy (trueness and precision) | top-down inputs | to verify |
| ISO 13528:2022 | statistical methods for proficiency testing | `pt` module, proficiency projects; validation Annex E | read |
| ISO 6143:2025 | gas analysis, calibration with uncertainties in both variables | `gls` module, calibration projects; validation Annex B (F41, F42) | read |
| EA-4/02 M:2022 | uncertainty in calibration (examples S2–S13) | validation S2–S13, Table E.1, trapezoidal k | read (Supplements 1–2, §5, App. E) |
| ISO/TS 28037:2010 | straight-line calibration functions | calibration curves | to verify |
| ISO/IEC 17025:2017 (7.2.2, 7.6, 7.7, 7.8, 7.11) | lab requirements, reporting, software validation | mapping in `iso17025-mapping.md` | read (TS EN translation) |
| NIST TN 1297 (1994), NIST TN 1900 (2015) | US guidance, worked examples | TN 1900 examples in the library (F28–F32) | read |
| NIST Uncertainty Machine user manual v1.4 (2020) | reference software, worked examples | library projects, F29, F33, F35 | read |

## 2. Methodological literature

| Reference | Relevance | Status |
|---|---|---|
| Wichmann & Hill (2006), *Comput. Stat. Data Anal.* 51:1614 — enhanced generator | RNG used; constants checked (D29) | read |
| Box & Muller (1958) | normal variates | cited in JCGM 101 [3] |
| Marsaglia & Tsang (2000), *ACM TOMS* 26:363 | gamma variates; procedure checked (D29) | read |
| Kragten (1994), *Analyst* 119:2161 | spreadsheet numerical differentiation (used in QUAM) | to verify |
| Welch (1947); Satterthwaite (1946) | ν_eff | cited in GUM G.4 |
| Possolo & Iyer (2017), *Rev. Sci. Instrum.* — concepts and tools for uncertainty evaluation | Bayesian / modern view | to verify |
| Magnusson & Ellison (2008), *Anal. Bioanal. Chem.* 390:201 — uncorrected bias | review of RSSu, RSSU, SUMU, SUMU_MAX, Ue (Table 3); coverage simulations (Fig. 1), interval widths (Fig. 2); implemented as `reporting.uncorrected_bias` (D34); sign error in Table 3 (F43) | read |
| Phillips, Eberhardt & Parry (1997), *J. Res. NIST* 102:577 — uncorrected bias | primary source of SUMU, RSSu_c, RSSU | read |
| Synek (2005), *Talanta* 65:829 — Ue | primary source of Ue (exact-coverage symmetric interval); implemented from its definition as given by Magnusson & Ellison | to verify |
| O'Donnell & Hibbert (2005), *Analyst* 130:721 | coverage of bias strategies (conflicting with Magnusson & Ellison) | to verify |
| Palma, Morgado & Bettencourt da Silva (2019), *Talanta* 192:278 — matrix effects | top-down contribution of matrix effects: intermediate precision vs dispersion of recoveries across matrices; two concentration intervals (LOD–2LOQ, above); Pb in marine sediment worked through | read; seven projects from Table 9 (D39); compatibility tests need the electronic supplement |
| Mateos et al. (2020), *Talanta* 207:120274 — recovery correction vs adjusted calibrators | PAH in water: neat vs processed calibrators; trueness from a weighted mean recovery with weights depending on u and ν; naphthalene U 9.9 % → 3.5 % | read; Interval II of Tables 2–3 not reproducible (F46); the ν-dependent weighting is not in the program |
| Kallner (2002), *eJIFCC* 13 — uncertainty in laboratory medicine | introduction and examples for clinical laboratories | read; starting point for ISO/TS 20914 |

## 3. Existing software (competitor and benchmark landscape)

Full texts read on 2026-10-27 (supplied by the author; licensed copies kept
locally, not in the repository): Hall (2022), Lobsiger et al. (2022),
Pennecchi & Harris (2023), Bettencourt da Silva (2026) with its electronic
supplement. Bibliographic details and links are in
`literature-search-2026-10.md` (theme A).

| Software | Developer | Methods | Platform / licence | Validation evidence published | Notes | Status |
|---|---|---|---|---|---|---|
| NIST Uncertainty Machine | NIST (Lafarge & Possolo, *NCSLI Measure* 10(3):20–27, 2015, doi:10.1080/19315775.2015.11721732) | GUM + MCM (S1, S2), correlations | web | user manual examples | reference implementation for cross-checks; manual examples in the library (F29, F33, F35) | manual read; paper listed |
| Suncal (Sandia Uncertainty Calculator) | Sandia National Laboratories (Delker, *NCSLI Measure* 13:34–39, 2021) | GUM + MCM side by side; validated against GUM and TN 1900 | Python, open source, GUI | graded software QA (the paper) | **closest competitor in calibration**; must benchmark | paper obtained (not yet read) |
| GUM Tree Calculator (GTC) | MSL New Zealand (Hall, *Metrology* 2:128–149, 2022) | LPU by "uncertain numbers" (forward automatic differentiation, stage by stage); ν_eff with Willink's extension for correlated inputs from one fit (ensembles); complex quantities (2 × 2 covariance, ν for complex results); intermediate components; storage of uncertain numbers in JSON archives for traceability chains | Python library (≥ 2.7/3), open source (GitHub MSLNZ/GTC) | unit-test suite; checks against GUM annexes and published regressions; >10 years of in-house use (App. A.4) | no GUI, no MCM, no top-down; the Willink ensemble ν and the JSON archive of uncertain numbers are features this program lacks | read |
| METAS UncLib | METAS (Zeier, Hoffmann, Wollensack, *Metrologia* 49:809, 2012; Lobsiger, Wollensack, Zeier, *CHIMIA* 76:596, 2022) | linear propagation by automatic differentiation of uncertain objects, complex-valued and multivariate quantities, matrix operations; MC | C# (.NET) library, MATLAB/Python wrappers, free of charge | not described in the CHIMIA note | chemistry use: PAHs in food by GC-MS/MS, > 45 inputs per PAH, conformity with the Swiss maximum levels; "does not help to build a model, no fancy UI" (METAS); intermediate components may fail unless preconditions hold (Hall 2022, A.5); the group that found the B_LEAST error of ISO 6143 (F42) | read |
| MUPR spreadsheet | Bettencourt da Silva (*Talanta*, 2026, doi:10.1016/j.talanta.2026.130826; Zenodo 23043687) | top-down from in-house precision and recovery (Eurachem/CITAC MUPR 2026, the author's own guide): dated analysis sequence of blanks, samples, RMs and spikes (2 000 or 10 000 lines); outliers; precision models in up to five intervals (constant absolute or relative, or s = a + b·x + c·x^d); recovery from RMs and spikes with and without native analyte, χ² compatibility (Eq. 16), test against 100 %, correction; duplicate acceptance (range factors for low ν by Monte Carlo); QC limits and charts for each RM; R-charts; result and U for each line with the percentage contributions; ν = minimum of the components' ν instead of Welch–Satterthwaite; significant figures | MS Excel, password-protected sheets, free; instructions in English and Portuguese; video | a parallel ad hoc spreadsheet and five working days of checks; the user is responsible for the output | **closest competitor in chemistry** (same guidance, same intended users). Two application examples (soil pH, fluoranthene in river sediments). Recomputed here from Supplementary Material 06: three discrepancies (F71–F73) | read; supplement 06 recomputed |
| LNE-MCM, LNE Uncertainty | LNE France | MCM S1, multivariate, sensitivity | free (MATLAB-based) | | | listed |
| NPLUnc_101 | NPL | GUM + S1, uncorrelated inputs | MATLAB, open source | | | listed |
| metRology | R package (Ellison) | uncertainty, interlaboratory comparisons | R, open source | | cited by Bettencourt da Silva (2026) | listed |
| MUkit | SYKE Finland | Nordtest top-down | free, Windows | | top-down only | listed |
| GUM Workbench | Metrodata | GUM, EA-4/02; MC support unconfirmed | Windows, commercial | | | listed |
| QMSys GUM | Qualisyst | GUM, MC | Windows, commercial | | | to verify |

**The Mathmet survey (Pennecchi & Harris, *Acta IMEKO* 12(2), 2023).** The
European Metrology Network Mathmet listed 50 programs and analysed 35 of them
against JCGM 100/101/102:

- 74 % are cross-platform, 85 % run on a computer (15 % are web applications).
- Only 54 % give some evidence of validation.
- 71 % implement first-order LPU, 55 % give sensitivity coefficients and
  54 % the expanded uncertainty.
- About 60 % follow JCGM 101 in assigning the input PDFs and in summarising
  the output distribution.
- Support for JCGM 102 is the largest gap: 20 % do multivariate LPU for
  explicit models, 6 % for implicit models, and 31 % do multivariate MCM.

Training shows the same gap. Of 41 courses, only 20 % teach multivariate LPU
and 15 % multivariate MCM, although 46 % are aimed at calibration and testing
laboratories.

The survey's examples are 69 classroom examples from the courses. The EMUE
compendium adds 41 examples (8 in chemistry) covering Bayesian, regression
and top-down approaches (van der Veen & Cox, eds., 2021,
http://empir.npl.co.uk/emue/). Mathmet is developing a quality management
system for metrology software (Lines et al., *Acta IMEKO* 11(4), 2022,
doi:10.21014/actaimeko.v11i4.1348).

Note: the Eurachem MUPR working group includes P. Yolcu Omeroglu (Eurachem
TR, Türkiye); the Nordtest handbook has an official Turkish translation
(nordtest.info). Both are relevant for Turkish terminology in the guide.

### Gap analysis (based on the full texts read so far)

| Capability | This program | GTC | UncLib | MUPR spreadsheet | Mathmet survey (35 tools) |
|---|---|---|---|---|---|
| LPU with correlations | yes | yes | yes | — | 71 % |
| MCM (JCGM 101), adaptive | yes | no | MC only | Monte Carlo only for range factors | ≈ 60 % follow S1 |
| Multivariate (JCGM 102), explicit and implicit | yes (D42) | through uncertain numbers | yes | — | 20 % / 6 % (LPU), 31 % (MCM) |
| Top-down from validation and QC data (Nordtest, MUPR, ISO 21748, ISO 11352) | yes | — | — | MUPR only | not surveyed |
| Sampling (UfS), PT (ISO 13528), QC charts (TR 569), conformity (JCGM 106), calibration with errors in both variables (ISO 6143), weighing (cg-18) | yes | — | — | QC charts and limits | not surveyed |
| Complex quantities; ν for correlated inputs (Willink); storage for traceability chains | no | yes | complex, storage | — | — |
| Graphical interface; desktop *and* mobile, offline | yes (Flutter; Linux, Windows, Android) | no GUI | no GUI | Excel only | 85 % computer, 15 % web |
| Published, self-checking worked examples | 203 projects, each with `expected` values from the source | GUM annexes in the test suite | — | 2 applications | 69 classroom + 41 EMUE examples (not machine-checked) |
| Validation evidence for ISO/IEC 17025 7.11 | public validation matrix and report | test suite, in-house use | — | parallel spreadsheet, 5 days of checks | 54 % give some evidence |
| Languages | English, Turkish (UI, reports, guide) | English | English | English, Portuguese (instructions) | — |
| Licence | GPL-3.0-or-later | open source | free, closed | free, password-protected | — |

The full texts confirm the working hypothesis for the four tools read. None
of them combines the bottom-up methods (GUM, JCGM 101, JCGM 102) with the
chemistry top-down, sampling, PT and QC methods in one program. None runs
offline on mobile devices. None checks a library of hundreds of published
examples automatically against the printed results.

The recomputation of the MUPR spreadsheet (F71–F73) shows why that check
matters: an independently implemented spreadsheet from the guide's own author
departs from the guide's equations in two places, and the article's text
departs from the spreadsheet in one.

Two features of GTC are worth adopting:

- the Willink (2007) extension of Welch–Satterthwaite for inputs that are
  correlated and have finite ν (for example the slope and intercept of one
  fit);
- the transfer of an uncertain result with its components to a later stage
  of a traceability chain.

The comparison is still open for Suncal (paper obtained), the NIST
Uncertainty Machine paper and VNA Tools (Zeier et al., 2025).

## 4. Search protocol for the systematic review (to run before submission)

Databases: Scopus, Web of Science, Google Scholar, IEEE Xplore, IMEKO
proceedings. Query blocks: ("measurement uncertainty" OR "uncertainty
budget") AND (software OR tool OR calculator OR package) AND (GUM OR "Monte
Carlo" OR "Supplement 1" OR "top-down" OR Nordtest). Period 2008–present.
Record: tool, methods, validation evidence, platform, licence, domain.
