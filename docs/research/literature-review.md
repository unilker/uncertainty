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
| JCGM 102:2011 (GUM-S2) | multivariate output quantities | planned | obtained (not yet read) |
| JCGM 104:2009 | introduction to the GUM documents | guide | obtained (not yet read) |
| JCGM GUM-1:2023 | Part 1: Introduction (ISO/IEC Guide 98-1) | guide | listed |
| JCGM GUM-6:2020 | Developing and using measurement models | intermediate quantities, guide | obtained (not yet read) |
| JCGM 106:2012 | conformity assessment, decision rules | planned (ISO 17025 statements) | obtained (not yet read) |
| JCGM 200:2012 (VIM 3) | vocabulary | TR/EN glossary | obtained (not yet read) |
| EURACHEM/CITAC CG4 QUAM:2012 (3rd ed.) | uncertainty in analytical chemistry (bottom-up, Kragten spreadsheet, examples A1–A7) | chemistry module, validation A1–A7 | read (A1–A7, App. E.4) |
| Eurachem/CITAC MUPR 1st ed. (2026) | uncertainty from in-house precision and recovery data | `topdown` module; validation Ex. 2, 3, B1–B6, C1 | read (complete) |
| Nordtest TR 537 ed. 4 (2017) | top-down for environmental labs (R_w, bias, RMS bias) | `topdown` module; validation §3–8, App. 4–7 | read |
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
| Palma, Morgado & Bettencourt da Silva (2019), *Talanta* 192:278 — matrix effects | top-down contribution of matrix effects: intermediate precision vs dispersion of recoveries across matrices; two concentration intervals (LOD–2LOQ, above); Pb in marine sediment worked through | read; candidate library example (two-interval model, recovery compatibility tests) |
| Mateos et al. (2020), *Talanta* 207:120274 — recovery correction vs adjusted calibrators | PAH in water: neat vs processed calibrators; trueness from a weighted mean recovery with weights depending on u and ν; naphthalene U 9.9 % → 3.5 % | read; the ν-dependent weighting is not in the program |
| Kallner (2002), *eJIFCC* 13 — uncertainty in laboratory medicine | introduction and examples for clinical laboratories | read; starting point for ISO/TS 20914 |

## 3. Existing software (competitor and benchmark landscape)

| Software | Developer | Methods | Platform / licence | Notes | Status |
|---|---|---|---|---|---|
| NIST Uncertainty Machine | NIST (Lafarge & Possolo, *NCSLI Measure* 10(3):20–27, 2015, doi:10.1080/19315775.2015.11721732) | GUM + MCM (S1, S2), correlations | web | reference implementation for cross-checks | listed |
| Suncal (Sandia Uncertainty Calculator) | Sandia National Laboratories (Delker, SAND2021-8022J) | GUM + MCM side by side; validated against GUM and TN 1900 | Python, open source, GUI | **closest competitor**; must benchmark | paper obtained (not yet read) |
| GUM Tree Calculator (GTC) | MSL New Zealand (Hall, Borbely; *Metrology* 2(1):9, 2022, MDPI) | GUM via uncertain numbers (automatic differentiation) | Python library | no GUI; not MCM-centred | listed |
| METAS UncLib | METAS (Zeier, Hoffmann, Wollensack, *Metrologia* 49:809, 2012) | linear propagation by automatic differentiation of uncertain objects, complex-valued and multivariate quantities, matrix operations; MC | .NET library, MATLAB/Python wrappers, free | "does not help to build a model, no fancy UI" (METAS); *CHIMIA* 2022 chemistry article; the group that found the B_LEAST error of ISO 6143 (F42) | read |
| LNE-MCM, LNE Uncertainty | LNE France | MCM S1, multivariate, sensitivity | free (MATLAB-based) | listed |
| NPLUnc_101 | NPL | GUM + S1, uncorrelated inputs | MATLAB, open source | listed |
| metRology | R package | uncertainty, interlaboratory comparisons | R, open source | listed |
| MUkit | SYKE Finland | Nordtest top-down | free, Windows | top-down only | listed |
| GUM Workbench | Metrodata | GUM, EA-4/02; MC support unconfirmed | Windows, commercial | listed |
| QMSys GUM | Qualisyst | GUM, MC | Windows, commercial | to verify |

Note: the Eurachem MUPR working group includes P. Yolcu Omeroglu (Eurachem
TR, Türkiye); the Nordtest handbook has an official Turkish translation
(nordtest.info). Both are relevant for Turkish terminology in the guide.

### Gap analysis (working hypothesis — to be confirmed by a systematic search)

No single tool found so far combines all of: (1) native mobile *and* desktop
operation offline; (2) bottom-up (GUM/MCM) and top-down (Nordtest/ISO
11352/MUPR) in one budget workflow aimed at analytical chemistry; (3) a large,
individually validated library of worked examples with step-by-step
explanations; (4) full Turkish and English user interface and tutorial;
(5) a public validation matrix with high-accuracy references and measured
Monte Carlo dispersion. Each claim must be checked against Suncal, NUM,
METAS UncLib and MUkit before it appears in the article.

## 4. Search protocol for the systematic review (to run before submission)

Databases: Scopus, Web of Science, Google Scholar, IEEE Xplore, IMEKO
proceedings. Query blocks: ("measurement uncertainty" OR "uncertainty
budget") AND (software OR tool OR calculator OR package) AND (GUM OR "Monte
Carlo" OR "Supplement 1" OR "top-down" OR Nordtest). Period 2008–present.
Record: tool, methods, validation evidence, platform, licence, domain.
