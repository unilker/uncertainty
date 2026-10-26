# Validation matrix

Every reference case is an automated test (`cargo test`). A case passes only
if the engine reproduces the published value within the stated tolerance.
For Monte Carlo outputs the tolerance is ≈ 5 standard deviations of the
engine's own dispersion, measured with `core/examples/mcm_dispersion.rs`
(20 independent seeds, M = 2 × 10⁶), plus the rounding of the published
value. Where published MCM values carry their own sampling noise, the
primary reference is an analytic solution or an independent 2 × 10⁸-trial
PCG64 run (`validation/reference_values.py` → `reference_values.json`).

Test files: `core/tests/gum_examples.rs`, `core/tests/jcgm101_examples.rs`,
`core/tests/eurachem_examples.rs`, `core/tests/topdown_examples.rs`,
`core/tests/ea402_examples.rs`; unit tests in `core/src/*`.

## Numerical building blocks

| Item | Reference | Agreement | Test |
|---|---|---|---|
| erf, erfc, Φ⁻¹ | SciPy | ≤ 1e-12 relative | `special::tests` |
| Student t quantiles ν = 1…100, 2.5, 16.7 | SciPy `stats.t.ppf` | ≤ 1e-11 relative | `special::tests` |
| GUM Table G.2 (t₉₉(16) = 2.92 …) | GUM | 2 decimals | `special::tests` |
| Wichmann–Hill 64-bit vs 32-bit form | JCGM 101 C.3.3.2/3 | identical for 10⁵ draws | `rng::tests` |
| t, gamma, normal generators | exact cdf/moments | statistical tests | `rng::tests` |
| Distribution moments (10 types) | JCGM 101 6.4 | 10⁶ draws | `distribution::tests` |
| CTrap example 10.0 V ± 0.1 V → 0.060 V | JCGM 101 6.4.3 | exact | `distribution::tests` |
| Numerical tolerance δ, examples 1–3 | JCGM 101 7.9.2 | exact | `mcm::tests` |
| Coverage interval indices | JCGM 101 7.7.2 | exact | `mcm::tests` |
| χ² quantiles ν = 1…30 | SciPy `stats.chi2.ppf` | ≤ 1e-9 relative | `special::tests` |
| Rounding rules (MUPR Table 2, Nordtest round-up) | MUPR §12, Nordtest | exact strings | `report::tests` |
| Symbolic derivatives | central differences | ≤ 1e-6 relative | `expr::tests` |

## Reference examples

| ID | Source | Quantity | Published | Engine | Status |
|---|---|---|---|---|---|
| GUM-H1a | GUM H.1.6 | u(d), ν_eff(d) | 9.7 nm, 25.6 | 9.68 nm, 25.45 (25.6 with rounded u) | ✅ (F1) |
| GUM-H1b | GUM H.1, Table H.1 | contributions | 25, 9.7, 0, 0, 2.9, 16.6 nm | same | ✅ |
| GUM-H1c | GUM H.6, H.1.6 | u_c, ν_eff, k₉₉, U₉₉ | 32 nm, 16.7, 2.92, 93 nm | 31.67 nm, 16.76, 2.92, 92.5 nm | ✅ (F1) |
| GUM-H1d | GUM H.1.7 | u_c with 2nd-order terms | 34 nm (terms 11.7, 1.7 nm) | 33.8 nm (11.7, 1.7) | ✅ |
| S1-9.2.2 | JCGM 101 Table 2 | Gaussian additive | u = 2.00, [−3.92, 3.92] | agrees; GUF validated | ✅ |
| S1-9.2.3 | JCGM 101 Table 3, Annex E | rectangular additive | analytic ±3.88 | agrees | ✅ |
| S1-9.2.4 | JCGM 101 Table 4 | dominant rectangular | GUF ±19.9, MCM ±17.0; not validated (δ = 0.5) | agrees; validated only at ndig = 1 | ✅ |
| S1-9.3 | JCGM 101 Tables 6–7 | mass calibration GUF1/GUF2/MCM | 0.0539, 0.0750, 0.0754 mg | agrees; ref. 2×10⁸: u = 0.075 484 | ✅ (F2) |
| S1-9.3a | JCGM 101 9.3.2.2 | adaptive MCM δ/5 | 0.72×10⁶ trials | converges, same results | ✅ |
| S1-9.4.2 | JCGM 101 Table 8 | comparison loss r = 0 (3 cases) | G1, G2, MCM | agrees with analytic χ² | ✅ (F2, F3) |
| S1-9.4.3 | JCGM 101 Table 9 | comparison loss r = 0.9 (3 cases) | MCM | agrees with 2×10⁸ ref. | ✅ |
| S1-9.5 | JCGM 101 Tables 10–11 | gauge block (models 36, 37) | GUF 838, 32, [745, 931]; MCM 838, 36, [745, 932] | agrees | ✅ |
| EU-A1a | QUAM:2012 Table A1.3 | Cd standard, u(V) = 0.07 | 1002.7 mg/L, u = 0.86 (0.9) | 1002.6997, 0.8638 | ✅ |
| EU-A1b | QUAM:2012 A1.4 | volume sub-model | u(V) = 0.066 (0.07) | 0.0665; u_c 0.835 | ✅ (F4) |
| EU-A2a | QUAM:2012 Tables A2.2–A2.3 | NaOH standardisation | 0.102136 mol/L, u = 0.000099, 5 contributions | identical | ✅ |
| EU-A2b | QUAM:2012 A2.4 | sub-models (weighing, IUPAC molar mass, burette) | u(m) 0.13 mg, u(M) 0.0038, u(V) 0.013 mL, u_c 0.000099 | 0.122 mg, 0.0038, 0.0137 mL, 0.000101; MCM agrees | ✅ (F10) |
| EU-A2c | QUAM:2012 Table A2.4 | distribution choice | 0.00011 / 0.000099 / 0.000085 | not reproducible | ⚠️ not tested (F10) |
| EU-A3a | QUAM:2012 Tables A3.2–A3.3 | HCl titration (Kragten) | 0.101387, u 0.00018, 7 contributions, (0.1014 ± 0.0004) | identical | ✅ |
| EU-A3b | QUAM:2012 A3.1, A3.4, A3.6 | summary table, triple determination, 25 °C, visual end-point | 0.00016 (summary), 0.00016, 0.10149, 0.0003 | 0.00019 from summary inputs; others reproduced | ✅ (F12) |
| EU-A4 | QUAM:2012 Tables A4.2–A4.5, A4.6 | pesticides: precision 0.27 (from raw duplicates), recovery t-test, binomial inhomogeneity (a)–(c), u_c/P 0.34, u 0.377 | reproduced | ✅ (F13) |
| EU-A5a | QUAM:2012 Table A5.2, App. E.4 | calibration from 15 raw points; u(c₀) | b₁ 0.2410 (0.0050), b₀ 0.0087 (0.0029), r 0.997, S 0.005486, u 0.018 | identical | ✅ |
| EU-A5b | QUAM:2012 Tables A5.3–A5.4 | Cd release budget and sub-models | r 0.015065, u 0.001465, u(V_L) 1.83, u(a_V) 0.19 | 0.001470 (exact), 1.83, u(a_V) 0.152 | ✅ (F14) |
| EU-A6 | QUAM:2012 Tables A6.2–A6.5 | crude fibre: s_R + drying allowance | 0.31/0.4/0.6; U 0.62/0.8/0.12 | U at 10 % = 1.2 | ✅ (F15) |
| EU-A7a | QUAM:2012 Tables A7.3–A7.8 | lead by double IDMS, Eq. (5) | c_x 0.053738, u_c 0.00018 / 0.00041, ΣK_bias 37.6 % | reproduced | ✅ (F16) |
| EU-A7b | QUAM:2012 Eq. (8), (12) | c_z, M(Pb) | 0.092605 (0.000028), 207.2103 | reproduced | ✅ |

## Top-down examples (relative values in %)

| ID | Source | Quantity | Published | Engine | Status |
|---|---|---|---|---|---|
| NT-A5 | Nordtest App. 5 | NH₄-N duplicates: s_r < 30 µg/L; s′_r > 30 µg/L | 0.44 µg/L; 3.8 % | 0.441 (46 pairs); 3.82 (26 pairs) | ✅ (F7) |
| NT-A6 | Nordtest App. 6, §5.3 | O₂ duplicates: pooled s, mean, s′; u(Rw) | 0.0252 mg/L, 7.50, 0.34 %; 0.60 % | same | ✅ |
| NT-3 | Nordtest §3.2, App. 4 (raw PT data) | NH₄-N: RMS_bias, ū(C_ref), u(bias), u_c, U | 2.26, 1.52, 2.73, 3.20, 6.4 → 7 % | same | ✅ |
| NT-6.1a | Nordtest §6.1 | one CRM | u(C_ref) 2.16, u(bias) 4.1 % | 2.17, 4.15 % (4.1 with printed 2.16) | ✅ (F5) |
| NT-6.1b | Nordtest §6.1 | three CRMs | 2.53, 1.92, 3.2 % | same | ✅ |
| NT-6.2 | Nordtest §6.2 | proficiency tests | 1.8; 2.7/2.6; 4.6; 5.3 % | 1.81; 2.60; 4.60; 5.28 | ✅ (F6) |
| NT-6.3 | Nordtest §6.3 | recovery | 3.44, 0.76, 1.0, 3.6 % | same | ✅ |
| NT-8.2 | Nordtest §8.2, App. 7 (raw data) | BOD: mean, s, s′; Examples A and B | 214.8, 5.6, 2.6 %; 4.5/5.2/11 %; 1.69/3.76/4.12/4.87/10 % | same (RMS 3.77 unrounded) | ✅ (F8) |
| NT-8.3 | Nordtest §8.3 | PCB Examples C and D | 7.22/10.8/22 %; 7.6/8.1/11.4/23 % | same | ✅ (F8) |
| MUPR-E2 | MUPR Example 2 | mean recovery, 2 CRMs + PT | R̄ 1.008, u 0.03705 (3 terms) | same | ✅ |
| MUPR-E3 | MUPR Example 3 | spiked samples with native analyte | 96.2 %, 10.02 %; 1.5 % (A only) | same | ✅ |
| MUPR-B1 | MUPR Example B1 | nitrate by IC: R̄, u_R̄, Eq. 13, limits, U⟨I⟩, U⟨II⟩, contributions, B1.16, B1.18, reporting | 97.2 %, 0.0285, 0.982 ≤ 2.093, 0.794/1.15, 0.729/1.21, 0.0676, 0.185c, 94.1/4.8/1.1 %, 82.9/9.5/2.1/5.6 %, 7.13, 0.57 ≤ 1.27, (0.261 ± 0.068), (50.0 ± 9.3) | same (0.963 and 0.0849 unrounded) | ✅ (F9) |
| MUPR-12 | MUPR §12, Table 2 | rounding of reported results | (0.156 ± 0.012), (5360 ± 240), (5360 ± 260) | same | ✅ |
| MUPR-B2 | MUPR B2 (raw Tables B2.1, B2.3) | pooled precision, two recoveries, compatibility, overall recovery, U, replicates | 0.0285, 7.73 %, 0.048 ≤ 0.134, 99.6 %, 2.24 %, 0.169c, 7.41, 7.06 | same | ✅ (F17) |
| MUPR-B3 | MUPR B3 | regression precision model; U with F = 100; k = t(5) | 0.0233 + 0.0128c (r 0.7266), 7.28, 0.137c | intercept 0.02324 | ✅ (F17) |
| MUPR-B4 | MUPR B4 | As in sediments, one CRM | 4.52 %, 0.0381, 2.24 ≤ 2.26, (16.0 ± 1.9), U_tg | same | ✅ |
| MUPR-B5 | MUPR B5 | CRM + 2 spiked samples, matrix variability | R̿ 0.9498, s(R) 0.0732, u 0.0800 / 0.0323, pairwise 0.275/1.32/1.50 | same | ✅ |
| MUPR-B6 | MUPR B6 (raw Tables B6.1–B6.3) | heterogeneity from 30 duplicates, 22 PT recoveries, χ², intervals | 7.34 %, 0.2157, 3.00 %, 0.5828, 7.808 %, R̿ 0.9174, u 0.01585 / 0.04872, χ² 8.80, 0.160, 0.1157, 21.5, 29.7 | same (χ² 8.70 with weighted mean) | ✅ (F17) |
| MUPR-C1 | MUPR App. C, Example C1 | allowance for an accepted recovery interval | Δa 27.87 %, u_a 8.1 % | 8.05 % | ✅ (F17) |

## Calibration examples (EA-4/02 M:2022)

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| EA-E1 | Table E.1 coverage factors (26 values) | 13.97 … 2.00 | t₉₅.₄₅(ν) agrees | ✅ |
| EA-S2 | weight 10 kg | 29.2 mg, U 58 mg | 29.26 (29.2 from rounded contributions) | ✅ (F18) |
| EA-S3 | resistor 10 kΩ (Type A from 5 ratios) | 10 000.178 Ω, 8.33 mΩ, U 17 mΩ | same | ✅ |
| EA-S4 | gauge block 50 mm (second-order δα·θ) | 49.999 926 mm, 34.3 nm, U 69 nm | same | ✅ (F19) |
| EA-S5 | thermocouple 1000 °C (two budgets) | 0.641 °C; 36 229 µV, 25.0 µV | same (δt term 24.65) | ✅ (F18) |
| EA-S6 | power sensor (U-shaped mismatch) | 0.933, 0.01623, U 0.032, ν_eff ≈ 310 | same | ✅ (F18) |
| EA-S7 | step attenuator | 30.043 dB, 0.0223 dB, U 0.045 dB | 0.0222 dB | ✅ (F18) |
| EA-S9 | DMM 100 V, rectangular output | 0.030 V, k 1.65, (0.10 ± 0.05) V | same | ✅ |
| EA-S10 | vernier calliper, trapezoid β = 0.33 | 32 µm, k 1.83, (0.10 ± 0.06) mm | same | ✅ |
| EA-S11 | temperature block, β = 0.43 | 164 mK, k 1.81, (180.1 ± 0.3) °C | k = 1.80 | ✅ (F18) |
| EA-S12 | water meter (ν_eff = 10) | 199.93 l, 0.109 l; 0.68e-3; 0.91e-3, k 2.28, U 0.002 | same | ✅ |
| EA-S13 | ring gauge 90 mm (temperature products) | 0.15 µm sub-budget, 0.411 µm, U 0.9 µm | 0.148, 0.41, 0.82 → 0.9 only with round-up | ✅ (F18) |

## NIST Technical Note 1900 (2015) — library projects

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| TN-E1 | weighing (linear, Gaussian) | 50.284 g, u 0.0071 g, [50.270, 50.298] g | same; GUM validated | ✅ |
| TN-E2 | surface temperature (Type A, ν = 21) | 25.59 °C, u 0.872 °C, k 2.08, (23.8, 27.4) °C | same (GUM and MCM) | ✅ |
| TN-E3 | falling-ball viscometer (skewed) | MCM 5.82, u 1.11, [4.05, 8.39] mPa s; "Gauss" 1.11 | MCM same; 1st-order GUM 1.027, 2nd-order 1.10 | ✅ (F29) |
| TN-E4 | Pitot tube (power law) | 40.64 m/s, u 0.25 m/s, [40.17, 41.13] | 57.48 m/s, u 0.348 m/s (= printed × √2) | ✅ (F28) |
| TN-E8 | M_r(CO₂), two rectangular inputs | 44.0094, u 0.0007, exact [44.0080, 44.0108] | same (exact 44.008045, 44.010755) | ✅ (F32) |
| TN-E9 | cadmium standard (QUAM A1, rectangular P, triangular V) | 1002.7 mg/L, u ≈ 0.9, [1001.0, 1004.4] | 0.864; same interval | ✅ (F30) |
| TN-E11 | step attenuator (EA S7, arcsine δL_M, δL_D, δL_K; t₃ L_S) | 30.043 dB, u 0.0224 dB, [30.006, 30.081] | same (MCM only) | ✅ (F31) |
| TN-E13 | thermal expansion, Gaussian / t₃ inputs, 95 % / 99 % | 1.73e-5, u 0.18e-5 K⁻¹; (1.38, 2.07), (1.27, 2.18), (1.40, 2.05), (1.15, 2.30) | same (4 projects) | ✅ (F31) |

## NIST Uncertainty Machine manual v1.4 (2020) and GUM H.2 — library projects

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| NUM-3 | y = ab/c, triangular divisor | Gauss 28.8, u 8.7; MCM 32.2, sd 13, (18.2, 67) | same | ✅ (F33) |
| NUM-8 | end gauge, Gaussian inputs | Gauss u 31.7 nm; MCM sd 33.9 nm, 99 % (50 000 749, 50 000 927) | same; zero-sensitivity warning 13 % | ✅ |
| NUM-10 | GUM H.2 R with rounded Table H.2 values | Gauss 0.07; MCM sd 0.0699, (127.595, 127.869) Ω | 0.0700; same | ✅ (F34) |
| NUM-11 | Stefan–Boltzmann constant (CODATA 2010) | 5.670 3725e-8, u 2.05e-13 | same | ✅ |
| GUM-H.2 | R, X, Z from the raw sets of Table H.2 | 0.071, 0.295, 0.236 Ω | 0.0711, 0.2956, 0.2363 | ✅ (F34) |
| NUM-7, NUM-9 | thermal expansion (t₃), viscosity | = TN 1900 E13, E3 | see above | ✅ (F29, F35) |

## ISO 13528:2022 Annex E — library projects

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| ISO13528-E.2 | homogeneity and stability, As in chocolate | x̄ 0.18715, s_x 0.00398, s_w 0.00556, s_s 0.00060, 0.3σ_pt 0.00842; Δ 0.00660 | same | ✅ |
| ISO13528-E.3 | atrazine, 34 results, Algorithm A | x* 0.2570, s* 0.0395 (6 iterations, x* ± δ of each), u 0.0085; median 0.2620, MADe 0.0386, nIQR 0.0402 | same | ✅ |
| ISO13528-E.3 | median and nIQR | 0.2620, 0.0402, u 0.0086 | same | ✅ |
| ISO13528-E.4 | mercury, reference value, 21 laboratories | D %, PA, z, z′, ζ, E_n (126 values) | same | ✅ (flags: F36) |
| ISO13528-B.1 | factors F₁, F₂ | Table B.1 (g = 7 … 20) | from χ² and F quantiles | ✅ |
| ISO13528-E.1 | 23 results, 5 censored, Algorithm A | ‘<’ ignored 26.01/7.23; deleted 26.81/5.29; 0.5 × limit 23.95/8.60 | same; 0.5 × limit converged 23.959/8.600 | ✅ (F44) |
| ISO13528-E.7 | robust mean of E.4 vs reference value | x* 0.03161, s* 0.0164 (21 results) | same | ✅ |
| ISO13528-E.5 | reference value from a CRM, 20 paired samples | d 1.73, s 1.07, u(d) 0.24; x_pt 23.35, u 0.35 | same | ✅ |

## ISO 21748:2017 Annex C and ISO 5725-3:2023 Annex J — library projects

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| ISO21748-C.1 | CO in emissions, s_R = 0.28 g/km | U 0.56 g/km | same | ✅ |
| ISO21748-C.2 | meat content (combined result) | 95.6 %, u 1.98 %, U 4.0 % | 95.6, 1.99, 4.0 | ✅ |
| ISO21748-C.3 | aerobic plate count, s′_R with s_l (7.3.2) + 3 % | shrimp 7.2/7.8, vegetables 8.4/8.9, flour 5.5/6.4 % | flour 6.29 % | ✅ (F39) |
| ISO21748-C.4 | crude fibre, s_R by range + drying | u 0.31, 0.41, 0.59 %; U 0.6, 0.8, 1.2 % | same | ✅ (F38) |
| ISO5725-3-J.1 | carbon in steel, 27 day-pairs | s_I(TO) 2.87 × 10⁻³ % | same | ✅ |
| ISO5725-3-J.2 | vanadium, staggered-nested, levels 1–2 | SS, MS, s₀², s₍₁₎², s_r, s_I(T), s_R | same | ✅ |
| ISO5725-3:1994-D.2 | vanadium, levels 3–6 from the 1994 data (core test) | Table D.5 = J.5: s_r, s_I(T), s_R | same at levels 3–5; level 6 s_R 16.331 against 15.962 (negative s₍₁₎² kept, D41) | ✅ (F40, D41) |

## ISO 5725-2:2019 — library projects and core tests

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| ISO5725-2-T5–T8 | critical values and indicators | Table 5 (p = 8, 9; n = 2, 3), Table 6 (p = 8, 9, 15, 16, 40), Tables 7, 8 (p = 9) | D.1, D.2, D.5, D.6 within the printed digits (Table 6 single: ≤ 0.001); D.3 within ±0.003 of Table 6 for p = 6 … 40 | ✅ (core) |
| ISO5725-2-C.1 | sulfur in coal, 8 labs × 4 levels, unequal n | Cochran C (C.1–C.4), Grubbs (Table C.4), m̂, s_r, s_R (Table C.5) | same from Forms rounded to 3 decimals | ✅ (F47) |
| ISO5725-2-C.1-REML | REML | Table C.6 | same | ✅ |
| ISO5725-2-C.2 | softening point of pitch, 16 labs, n = 2, missing cells | Cochran (Table C.10), Grubbs (Table C.11), m̂, s_r, s_R (Table C.12) | same; s_R at level 4 1.9175 against 1.915 | ✅ (F48) |
| ISO5725-2-C.2-REML | REML | Table C.13 | same | ✅ |
| ISO5725-2-C.3 | creosote oil, outlying laboratory | Cochran 0.667, 0.636; Grubbs (Table C.17); m̂, s_r, s_R (Table C.18); relationships I–IV (Tables 1–4) | same; IV slope 0.770 against 0.772 | ✅ (F49) |
| ISO5725-2-C.3-REML | REML | Table C.19 | same | ✅ |
| ISO5725-2-A.1 | A_r, A_R | Table A.1 (5 entries) | same | ✅ (core) |

## ISO 6143:2025 Annex B — library projects

Reference: B_LEAST output printed in the standard. Independent checks with ODRPACK (scipy.odr 1.18,
`cov_beta`) and a Monte Carlo simulation of the whole calibration (4000 refits).

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| ISO6143-B.2.2 | NO by UV, linear, 3 points | b, u(b), cov, S_res 0.6743, Γ 0.568, weighted deviations (Table B.4), 3 contents with u and covariances (Table B.6) | same | ✅ |
| ISO6143-B.2.3 | N₂ by GC-TCD, linear, 8 points | b, u(b), cov, S_res 6.0455, Γ 1.6266, Table B.8 | same | ✅ |
| ISO6143-B.2.3 | quadratic | b, u(b), 3 covariances, S_res 1.3964, Γ 0.8664, Table B.9 | same | ✅ |
| ISO6143-B.2.3 | linear with u(x₄, x₇), u(x₅, x₈) | u(b), cov, Table B.10 | same | ✅ |
| ISO6143-B.2.4 | NO by NDIR, 12 points, linear (rejected) | S_res 272.6392, Γ 6.8352 | same | ✅ |
| ISO6143-B.2.4 | power function | b, u(b) and covariances within 0.3 % (ODRPACK 0.9 %), S_res 8.3804, Γ 1.1594, x 5.3456, u 0.014169 | same | ✅ |
| ISO6143-B.2.4 | exponential function | b, S_res 0.6572, Γ 0.3529, x 5.3357; u(b) 8.14 × 10⁻², 7.40 × 10⁻², 3.71 × 10⁻¹¹; u(x) 0.013280 | b, S_res, Γ, x same; u(b) 3.18, 3.17, 1.27 × 10⁻⁶ and u(x) 0.014246 as ODRPACK (MC: 3.13, 3.12, 1.24 × 10⁻⁶; 0.01406) | ✅ (F42) |
| ISO6143-6.2.4 | drift-test critical values | 2√2 u for n = 10 (Note); Formula (12) 2√(1 + 20/n) u | 2√(1 + 10/n) u, 2√(20/n) u | ✅ (F41) |

## Palma et al. (2019), marine sediments — library projects

Two-interval top-down models (LOD … 2w_LOQ absolute, above 2w_LOQ relative) combined from the components of
Table 9 (raw data only in the electronic supplement).

| ID | Element | Published u_c⟨I⟩ / u′_c⟨II⟩ | Engine | χ_ME⟨I⟩ / ⟨II⟩ published / engine | Status |
|---|---|---|---|---|---|
| Palma-Cr | Cr (OSPAR) | 1.05 mg/kg / 10.5 % | 1.049 / 10.49 | 60.8 / 60.9 — 60.8 / 60.8 | ✅ |
| Palma-Cu | Cu (OSPAR) | 0.612 / 6.13 | 0.6128 / 6.123 | 64.7 / 64.5 — 64.7 / 64.6 | ✅ (rounded components) |
| Palma-Li | Li (OSPAR) | 0.127 / 12.7 | 0.1267 / 12.67 | 62.3 / 62.4 — 62.3 / 62.3 | ✅ |
| Palma-Mn | Mn (OSPAR) | 0.920 / 9.20 | 0.9198 / 9.198 | 42.6 / 42.6 — 42.6 / 42.6 | ✅ |
| Palma-Ni | Ni (EPA 3051A) | 1.25 / 8.44 | 1.253 / 8.442 | 7.9 / 7.7 — 7.9 / 7.7 | ✅ |
| Palma-Pb | Pb (EPA 3051A) | 1.90 / 10.1 (10.08 in Eq. 21) | 1.895 / 10.08 | 65.2 / 57.6 — 65.2 / 57.6 | ✅ |
| Palma-Zn | Zn (OSPAR) | 0.365 / 9.13 | 0.3655 / 9.132 | 78.7 / 78.6 — 78.6 / 78.7 | ✅ |

## JCGM 106:2012 — conformity assessment (library projects and core tests)

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| JCGM106-7.3-E1 | Zener diode, V_b ≤ −5.40 V | p_c 0.92 | 0.919 | ✅ |
| JCGM106-7.3-E2 | bursting strength, B ≥ 490 kPa | p_c 0.99 | 0.989 | ✅ |
| JCGM106-7.4 | motor oil, two-sided | p_c 0.66 (0.93 − 0.27) | 0.663 | ✅ |
| JCGM106-7.7.5 | C_m = 1, 95 % locus | 0.45 ≤ ỹ ≤ 0.55 | 0.4491 … 0.5509 | ✅ (core) |
| JCGM106-8.3.2 | w = U = 2u (ISO 14253-1) | false acceptance ≤ 2.3 % | 2.28 % | ✅ (core) |
| JCGM106-8.3.3-E1 | speed limit, u = 2 % v, 99.9 % | v_max ≈ 107 km/h | 106.59 | ✅ |
| JCGM106-8.3.3-E2 | nandrolone, t₉, 95 % | A = 2.37 µg/L | 2.367 | ✅ |
| JCGM106-9.5.3 | precision resistors | p_c 90 %, R_C 1 %, R_P 7 %; 83/7/9/1 per 100 | 90.4, 0.99, 6.90 %; 83.5/6.9/8.6/1.0 | ✅ |
| JCGM106-9.5.4 | ball bearings, gamma prior | 4.2 % non-conforming; r ≈ 0.65, A ≈ 1.7 µm, R_P ≈ 7.5 % | 4.24 %; 0.656, 1.672, 7.55 % | ✅ (F45) |
| JCGM106-9.5.6 | u₀ = T/6, w = 0, C_m = 2 and 10 | R_C ≈ 0.1/0.04 %, R_P ≈ 1.5/0.07 % | 0.098/0.041 %, 1.47/0.072 % | ✅ (core) |

## Uncorrected bias (Magnusson & Ellison 2008) — core tests

| ID | Claim | Published | Engine | Status |
|---|---|---|---|---|
| ME2008-T3 | formulae of Table 3 | RSSu, RSSU, SUMU, SUMU_MAX, Ue | same (SUMU signs per Phillips et al. 1997, F43) | ✅ |
| ME2008-Ue | Ue ≈ 1.65u_c + \|Δ\| for \|Δ\| > 0.8u_c | stated | within 3 % for \|Δ\|/u_c = 0.8 … 4 | ✅ |
| ME2008-F1 | coverage, u(y) = 1, u_Δ = 0.5 | SUMU ≈ 95 % → 97.5 % at 3u_c; Ue ≈ 98 % → 95 %; RSSu, SUMU_MAX > 95 % | 95.2/97.5 %; 98.1/95.0 %; > 95 % | ✅ |
| ME2008-F2 | interval widths, u_Δ = 0.3 | RSSU < Ue ≈ RSSu < SUMU_MAX for \|b\| ≤ u(y); RSSu largest at 2u(y) | same | ✅ |

## Measured MCM dispersion (σ over 20 seeds, M = 2 × 10⁶)

| Case | y | u | shortest lo | shortest hi | shortest length | symmetric lo | symmetric hi |
|---|---|---|---|---|---|---|---|
| mass calibration [mg] | 4.0e-5 | 3.7e-5 | 7.4e-4 | 7.4e-4 | 1.8e-4 | 1.0e-4 | 1.2e-4 |
| loss x₁ = 0, r = 0 [10⁻⁶] | 0.04 | 0.04 | 6e-5 | 0.16 | 0.16 | 0.006 | 0.23 |
| loss x₁ = 0.01, r = 0 | 0.10 | 0.07 | 4e-4 | 0.29 | 0.29 | 0.03 | 0.30 |
| loss x₁ = 0.05, r = 0 | 0.48 | 0.20 | 3.2 | 3.3 | 1.0 | 0.78 | 0.85 |
| loss x₁ = 0, r = 0.9 | 0.04 | 0.08 | 2e-5 | 0.20 | 0.20 | 0.003 | 0.30 |
| loss x₁ = 0.01, r = 0.9 | 0.10 | 0.12 | 0.34 | 0.50 | 0.34 | 0.03 | 0.40 |
| loss x₁ = 0.05, r = 0.9 | 0.48 | 0.23 | 5.0 | 4.9 | 1.0 | 0.77 | 0.97 |

## Example library (project files)

The GUM/MCM cases above are also stored as 38 self-checking project files (49 with the eleven NIST TN 1900 projects) in
`library/` (see `library/README.md`), run by `project/tests/library.rs` and
`uncertainty check library` through the complete path a user takes (JSON →
model → GUM/MCM → comparison → TR/EN reports). Reference-value policy: D15
in the research log. EA-4/02 S13 in the library uses the raw observations
(estimate 0.2323 µm, F21), whereas the core test reproduces the printed budget.

The top-down cases (Nordtest §3, §5.3, §6.1–6.3, §8 A–D; MUPR Ex. 2, 3, B1,
B2, B4, B5, B6; QUAM A4, A6) are stored as 22 top-down project files that
start from the raw data wherever the documents give them (duplicate pairs,
control-chart results, replicate groups, PT rounds, the 22 PT results of B6).
Tolerances reflect the documents' printed rounding; where a guide computes
from rounded intermediates the tolerance is widened and the reason stated in
the file (MUPR B1 at 50 mg/L, B5 with t = 4.30; Nordtest §8 B, C).

## Planned cases

TN 1900 observation-equation examples (regression, random effects); TN 1900
E5 (truncated Gaussian inputs); ISO/TS 28037 data sets (the GLS engine of ISO 6143 covers the method); MUPR Appendix C
allowance as a project file; cross-checks against the NIST Uncertainty
Machine (F29) and Suncal.
