# Roadmap

| Phase | Content | Status |
|---|---|---|
| 1 | Core engine: GUM (1st/higher order, correlations, W–S), MCM (fixed/adaptive, intervals, GUF validation), distributions, model parser with symbolic derivatives; validation against GUM H.1, JCGM 101 §9, EURACHEM A1 | **done (v0.1.0)** |
| 2a | Top-down module (Nordtest TR 537, Eurachem MUPR 2026), reporting/rounding rules; EURACHEM A2; validation of Nordtest §3–8 and MUPR Ex. 2, 3, B1 | **done** |
| 2b | EURACHEM A3–A7, MUPR B2–B6 and App. C, OLS/WLS calibration with inverse prediction (QUAM E.4), EA-4/02 S2–S13 | **done** |
| 2c | Top-down project files with raw validation data (Nordtest, MUPR, QUAM A4/A6) | **done** |
| 2d | Linear and continuous precision models (MUPR B3, App. A) | **done** |
| 2e | ISO/TS 28037 (both-axes uncertainty; awaiting the standard), NIST TN 1900 examples | next |
| 3 | JSON project files; CLI (`run`, `check`, `template`); TR/EN reports (Markdown, HTML with SVG charts, JSON) with citation footer | **done** (PDF/CSV export: planned) |
| 4 | Flutter UI (desktop + mobile), Rust bridge, TR/EN localisation | planned |
| 5 | Calibration (EA-4/02 examples), conformity assessment (JCGM 106), multivariate outputs (JCGM 102) | planned |
| 6 | Example library (hundreds of validated, explained cases) and the TR/EN guide | **started**: 59 self-checking projects (GUM, JCGM 101, QUAM, EA-4/02, Nordtest, MUPR); guide planned |
| 7 | Independent review, benchmarks vs NUM/Suncal, release with DOI, manuscript | planned |
