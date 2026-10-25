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
| JCGM 102:2011 (GUM-S2) | multivariate output quantities | planned | to verify |
| JCGM 104:2009 | introduction to the GUM documents | guide | to verify |
| JCGM GUM-1:2023 | Part 1: Introduction (ISO/IEC Guide 98-1) | guide | listed |
| JCGM GUM-6:2020 | Developing and using measurement models | intermediate quantities, guide | listed |
| JCGM 106:2012 | conformity assessment, decision rules | planned (ISO 17025 statements) | to verify |
| JCGM 200:2012 (VIM 3) | vocabulary | TR/EN glossary | to verify |
| EURACHEM/CITAC CG4 QUAM:2012 (3rd ed.) | uncertainty in analytical chemistry (bottom-up, Kragten spreadsheet, examples A1–A7) | chemistry module, validation A1–A2 | read (A1, A2); A3–A7 pending |
| Eurachem/CITAC MUPR 1st ed. (2026) | uncertainty from in-house precision and recovery data | `topdown` module; validation Ex. 2, 3, B1 | read (§1–12, App. A, B1); B2–B6, App. C pending |
| Nordtest TR 537 ed. 4 (2017) | top-down for environmental labs (R_w, bias, RMS bias) | `topdown` module; validation §3–8, App. 4–7 | read |
| ISO 11352:2012 | water quality — uncertainty from validation and QC data | top-down module | to verify |
| ISO 21748:2017 | use of repeatability, reproducibility and trueness estimates | top-down module | to verify |
| ISO 5725 (parts 1–6) | accuracy (trueness and precision) | top-down inputs | to verify |
| EA-4/02 M:2022 | uncertainty in calibration (with worked examples) | calibration module | to verify |
| ISO/TS 28037:2010 | straight-line calibration functions | calibration curves | to verify |
| ISO/IEC 17025:2017 (7.6, 7.8, 7.11) | lab requirements, reporting, software validation | reporting, validation report | to verify |
| NIST TN 1297 (1994), NIST TN 1900 (2015) | US guidance, worked examples | further validation cases | listed (TN 1900) |

## 2. Methodological literature

| Reference | Relevance | Status |
|---|---|---|
| Wichmann & Hill (2006), *Comput. Stat. Data Anal.* 51:1614 — enhanced generator | RNG used | cited in JCGM 101 [54]; to read |
| Box & Muller (1958) | normal variates | cited in JCGM 101 [3] |
| Marsaglia & Tsang (2000), *ACM TOMS* 26:363 | gamma variates | to verify |
| Kragten (1994), *Analyst* 119:2161 | spreadsheet numerical differentiation (used in QUAM) | to verify |
| Welch (1947); Satterthwaite (1946) | ν_eff | cited in GUM G.4 |
| Possolo & Iyer (2017), *Rev. Sci. Instrum.* — concepts and tools for uncertainty evaluation | Bayesian / modern view | to verify |

## 3. Existing software (competitor and benchmark landscape)

| Software | Developer | Methods | Platform / licence | Notes | Status |
|---|---|---|---|---|---|
| NIST Uncertainty Machine | NIST (Lafarge & Possolo, *NCSLI Measure* 10(3):20–27, 2015, doi:10.1080/19315775.2015.11721732) | GUM + MCM (S1, S2), correlations | web | reference implementation for cross-checks | listed |
| Suncal (Sandia Uncertainty Calculator) | Sandia National Laboratories | GUM + MCM side by side; validated against GUM and TN 1900 | Python, open source, GUI | **closest competitor**; must benchmark | listed |
| GUM Tree Calculator (GTC) | MSL New Zealand (Hall, Borbely; *Metrology* 2(1):9, 2022, MDPI) | GUM via uncertain numbers (automatic differentiation) | Python library | no GUI; not MCM-centred | listed |
| METAS UncLib | METAS (Zeier, Hoffmann, Wollensack, *Metrologia* 2012) | linear/higher-order propagation, MC, complex & multivariate | .NET library, MATLAB/Python wrappers, free | "does not help to build a model, no fancy UI" (METAS); *CHIMIA* 2022 chemistry article | listed |
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
