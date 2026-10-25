# Roadmap

| Phase | Content | Status |
|---|---|---|
| 1 | Core engine: GUM (1st/higher order, correlations, W–S), MCM (fixed/adaptive, intervals, GUF validation), distributions, model parser with symbolic derivatives; validation against GUM H.1, JCGM 101 §9, EURACHEM A1 | **done (v0.1.0)** |
| 2a | Top-down module (Nordtest TR 537, Eurachem MUPR 2026), reporting/rounding rules; EURACHEM A2; validation of Nordtest §3–8 and MUPR Ex. 2, 3, B1 | **done** |
| 2b | EURACHEM A3–A7, MUPR B2–B6, calibration curves (ISO/TS 28037), link top-down budgets to GUM/MCM models | next |
| 3 | Reporting: budget table, contribution chart, histogram, PDF/CSV export, citation footer; JSON project files; CLI | planned |
| 4 | Flutter UI (desktop + mobile), Rust bridge, TR/EN localisation | planned |
| 5 | Calibration (EA-4/02 examples), conformity assessment (JCGM 106), multivariate outputs (JCGM 102) | planned |
| 6 | Example library (hundreds of validated, explained cases) and the TR/EN guide | planned |
| 7 | Independent review, benchmarks vs NUM/Suncal, release with DOI, manuscript | planned |
