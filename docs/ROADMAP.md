# Roadmap

| Phase | Content | Status |
|---|---|---|
| 1 | Core engine: GUM (1st/higher order, correlations, W–S), MCM (fixed/adaptive, intervals, GUF validation), distributions, model parser with symbolic derivatives; validation against GUM H.1, JCGM 101 §9, EURACHEM A1 | **done (v0.1.0)** |
| 2a | Top-down module (Nordtest TR 537, Eurachem MUPR 2026), reporting/rounding rules; EURACHEM A2; validation of Nordtest §3–8 and MUPR Ex. 2, 3, B1 | **done** |
| 2b | EURACHEM A3–A7, MUPR B2–B6 and App. C, OLS/WLS calibration with inverse prediction (QUAM E.4), EA-4/02 S2–S13 | **done** |
| 2c | Top-down project files with raw validation data (Nordtest, MUPR, QUAM A4/A6) | **done** |
| 2d | Linear and continuous precision models (MUPR B3, App. A) | **done** |
| 2e | NIST TN 1900 examples with measurement equations (E1–E4, E8, E9, E11, E13) | **done** |
| 2g | Proficiency testing (ISO 13528:2022): robust statistics, assigned value, scores, homogeneity and stability | **done** |
| 2h | Calibration with uncertainties in both variables (ISO 6143:2025): GLS fit of linear, polynomial, power and exponential analysis functions, covariances, validation, drift control | **done** |
| 2i | Interlaboratory precision experiments (ISO 5725-2:2019): outlier tests, s_r/s_R by ANOVA or REML, relationships I–IV | **done** |
| 2f | ISO/TS 28037 data sets (awaiting the standard); TN 1900 observation-equation examples | planned |
| 3 | JSON project files; CLI (`run`, `check`, `template`); TR/EN reports (Markdown, HTML with SVG charts, JSON) with citation footer | **done** (PDF/CSV export: planned) |
| 4 | Flutter UI (desktop + mobile), Rust bridge, TR/EN localisation | **in progress**: C interface crate, Flutter app (library, editors, results, export), Linux build tested; Android APK (engine for three ABIs), Windows/Linux packages, universal macOS app and unsigned iOS build (engine linked statically) built by the Package workflow; signing/store distribution pending |
| 5 | Calibration (EA-4/02 examples), conformity assessment (JCGM 106), multivariate outputs (JCGM 102) | **in progress**: EA-4/02 S2–S13 and JCGM 106 clauses 7–9 done; JCGM 102 planned |
| 6 | Example library (hundreds of validated, explained cases) and the TR/EN guide | **in progress**: 120 self-checking projects; TR/EN user guide (12 chapters + generated examples chapter) |
| 7 | Independent review, benchmarks vs NUM/Suncal, release with DOI, manuscript | planned |
