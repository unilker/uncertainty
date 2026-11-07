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

## JCGM 102:2011 — library projects and core tests

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| JCGM102-T1–T2 | coverage factors, m = 1 … 50 | k_p (Table 1), k_q (Table 2) | same | ✅ (core) |
| JCGM102-6.5.2.3 | ellipse and rectangle areas | 26.6, 28.4; 11.8, 40.1 unit² | 26.6, 28.4; 11.8, 40.2 (40.1 with k_q = 2.24) | ✅ (F50) |
| JCGM102-6.5.4 | coverage region for an expectation | F₂,₁₀(0.95) = 4.10 | same | ✅ (core) |
| JCGM102-7.7 | L, k_p, smallest region | L (transposed, F50); k_p 2.33 and 10.6 unit² from 1 000 draws; smallest region 9.4 unit² (10⁶ draws) | k_p 2.27, 10.1 unit² (10⁶ draws); smallest region 9.36 unit² | ✅ |
| JCGM102-9.2.2 | additive model, Gaussian | Table 3: GUF 1.414, 0.500, 2.45, 2.24; MCM 2.45, 2.21; validated | same | ✅ |
| JCGM102-9.2.3 | rectangular common effect | Table 4: MCM k_p 2.38, k_q 2.15; regions not validated | same | ✅ |
| JCGM102-9.2.4 | dominant rectangular effect | Table 5: GUF 3.162, 0.900; MCM 2.28, 1.87 | same | ✅ |
| JCGM102-9.3 | polar co-ordinates, six cases | Tables 6, 7 (MCM, M = 10⁷) | same (M = 10⁶) | ✅ |
| JCGM102-9.4 | resistance, reactance, impedance | Table 11: GUF, Alt. GUF, MCM (t₃) | same; MCM within the seed-to-seed spread of t₃ | ✅ (F51) |
| JCGM102-9.5.2 | resistance thermometer, implicit | θ 20.023 2 °C, u 0.004 5 °C | same | ✅ |
| JCGM102-9.5.3 | ten temperatures, implicit | Tables 15, 16 | same | ✅ |

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

## Bettencourt da Silva (2026), MUPR spreadsheet — library projects

Fluoranthene in river sediments, Supplementary Material 06: 20 mean recoveries with u(R_k) from the sheet
"Trueness", two-interval precision model, s(R) added (χ² test failed), k = 2.093 (ν = 19). Expected values are
recomputed by an independent script. The spreadsheet's own uncertainties differ for the reasons of F72 and F73.

| ID | Mean recovery | χ² / s(R) | Spreadsheet | Engine (0.1 / 10 mg/kg) | Status |
|---|---|---|---|---|---|
| Bet-weighted | R̿ = 74.887 % (Eq. 16.1), u = 8.812 % | 40.973 / 8.755 % | R̿ 74.887 %, χ² 40.973, s 8.755 %; U 0.161 / 3.857 | (0.1335 ± 0.2127) / (13.353 ± 3.575) | ✅ R̿, χ², s; U by design (F72, F73) |
| Bet-arithmetic | R̄ = 76.307 % (Eq. 8–12), u = 9.629 % | 40.973 / 8.755 % | — | (0.1311 ± 0.2091) / (13.105 ± 3.724) | ✅ recomputed |

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

## Nordtest TR 569 ed. 6 (2026) — control charts (library projects and core tests)

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| TR569-T4 | range factors, n = 2 … 4 | d₂ 1.128; D_WL 2.833, 3.470, 3.818; D_AL 3.686, 4.358, 4.698 | d₂, d₃ by numerical integration (n = 2 … 10); D_WL 2.8334, 3.4693, 3.8184; D_AL 3.6859, 4.3577, 4.6982 | ✅ (core, F54) |
| TR569-T1 | Zn, 60 raw values, X-chart | Figure 1: no value out of control | x̄ 60.278, s 2.598; WL 55.08/65.47, AL 52.48/68.07; 3 values in the warning zone, 0 out of control; t = 0.83 against 60.0 | ✅ |
| TR569-7-X1–X3 | X-chart cases 1–3 | 54.5/63.9, 52.1/66.3; 53.3/65.1, 50.3/68.1; 54.0/66.0, 51.0/69.0 µg/L | same | ✅ |
| TR569-7-R1–R2 | r%-chart cases 1–2 | WL 1.0 %, AL 1.3 %; s_r = r/2.8 = 0.357 % | 1.01/1.31 %; 0.357 % | ✅ (F52) |
| TR569-E1 | Ni by XRF, target from U = 4 % | s 0.0458; 4.49/4.67, 4.44/4.72 | same | ✅ |
| TR569-E2 | Co by XRF, target from LOQ | 0.0748/0.0788, 0.0738/0.0798 | same | ✅ |
| TR569-E3 | NH₄-N, X- and R-chart | 18.95/21.03, 18.43/21.55; s 0.496, WL 1.40, AL 1.83 | same (s = 0.4954 from 0.559/1.128379) | ✅ |
| TR569-E4 | Pb by ICP-MS | 0.278/0.310, 0.270/0.318 | same | ✅ |
| TR569-E5 | As in CRM, tolerance ± 15 % | AL 15.3/20.7 | same | ✅ |
| TR569-E6 | total P, r%-chart | CV 1.67; WL 4.73, AL 6.16 % | same with the rounded CV (4.72/6.14 % unrounded) | ✅ (F54) |
| TR569-E7 | β-HCH, target 15 % around the reference value | 11.2/20.8, 8.8/23.2 | same | ✅ |
| TR569-E8 | Cu by ICP-OES: preliminary and new limits, review | 0.92/1.19, 0.85/1.26; F 1.563 (crit 1.67), s_p 0.07545, t 1.012 (crit 1.98); 0.88/1.21, 0.80/1.29; R-chart 0.28/0.36, F 1.037 | same; F_crit 1.677 exact; s_p 0.07544; F (R) 1.0380 | ✅ (F54) |
| TR569-E9 | Zn blank | −0.051/0.129, −0.096/0.174 | same | ✅ |
| TR569-E10 | B, target 10 % around the reference value | 4/6, 3.5/6.5 | same | ✅ |
| TR569-E11 | micropipette, tolerance ± 0.01 mL | AL 0.99/1.01 | same | ✅ |
| TR569-E12 | 3 × 8 replicate control results | s_r 0.15 (Eq. 10), s_Rw 0.270 (simplified), 0.272 (ANOVA; footnote: s_r 0.1541, s_I 0.2246) | 0.1541, 0.2705, 0.2724 (s_r 0.1541, s_I 0.2247) | ✅ (F53) |
| TR569-9 | daily and statistical-control rules | Chapter 9 | core tests: action limit, two out of three (D43), trend of seven, ten of eleven (Example 4: twelve values on one side) | ✅ (core) |

## UKAS M3003 ed. 6 (2024) — library projects and core tests

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| M3003-B | t-table for p = 68.27, 95, 95.45, 99.73 %; B.10 | table; ν_eff 21.1, k 2.13 | same | ✅ (core) |
| M3003-C7 | k(95.45 %) for rectangular ⊛ normal, u_N/u_R = 0 … 2.5 | 26 values, 1.65 … 2.00 | all within ±0.005 | ✅ (core) |
| M3003-C5 | DVM with dominant resolution | u_c 0.305 mV, k 1.77, U 0.54 mV | same; MCM interval ±0.54 mV, validated | ✅ |
| M3003-K1 | 10 kΩ resistor | u_c 0.445 ppm; 10 000.1050 Ω ± 0.0089 Ω | same | ✅ |
| M3003-K2 | power sensor, 18 GHz | u_c 1.69 %, U 3.39 % | same | ✅ |
| M3003-K3 | 30 dB attenuator | u_c 0.0245 dB, U 0.0491 dB | same (0.024 57); with E5 mismatch 0.0202 dB | ✅ (F57) |
| M3003-K4 | 10 kg weight | u_c 24.56 mg, U 49.12 mg | same | ✅ (F55) |
| M3003-K5 | weighing machine, 200 g | u_c 0.150 mg, U 0.300 mg, +0.10 mg | 0.153 mg, 0.306 mg, −0.10 mg | ❌ printed (F56) |
| M3003-K6 | 10 mm gauge block | u_c 40.7 nm, U 81.5 nm | same; with the model's temperature terms 41.0 nm | ✅ (F58) |
| M3003-K7 | N-type thermocouple, 1000 °C | u_c(t) 0.641 °C; u_c(V) 25.9 µV, U 51.9 µV, 36 230 µV | 0.641; 25.84, 51.7, 36 228.8 | ✅ (F59) |
| M3003-K8 | DPI at 2 MPa | u_c 43.0 ppm, U 86.0 ppm, +17 ppm | same | ✅ (F60) |
| M3003-K9 | flowrate at standard conditions | 12.53 L/s, u_c 0.275, ν_eff 130, U 0.55 | same | ✅ |
| M3003-K10 | sheet thickness | 1.514 mm, u_c 0.009 63 mm, ν_eff 50, U 0.019 mm | same | ✅ |
| M3003-K11 | PT100 probe; gas temperature | u_c 0.042 25 °C; 22.97 °C, u_c 0.086 38 °C | same; 0.086 35 °C | ✅ |
| CG18-H1 | 220 g / 0.1 mg balance: zero, 220 g (5d, 5e), 100 g and 220 g with buoyancy correction | u(E) 0.000 118, 0.001 971, 0.000 491, 0.000 149, 0.000 226 g; k 2.87, 2.00, 2.00, 2.25, 2.05 | same; k = 2.04 exact at ν = 62 (guide uses GUM Table G.2) | ✅ |
| CG18-H2 | 60 kg multi-interval balance (2/5/10 g): zero, 10 kg, 60 kg (5d, 5e, option 2), adjusted 60 kg (5c) | u(E) 1.238, 1.552, 5.978, 5.948, 5.941, 5.739 g; k 2.52, 2.17, 2.05, 2.05, 2.05, 2.025 | 1.238, 1.552; 5.973, 5.944, 5.938, 5.725 (mpe 0.90 g for 3 × 20 kg F2); k exact 2.03, 2.01 | ✅ (F68) |
| CG18-H3 | 30 t weighbridge, substitution loads: zero, 10 t, 15 t, 30 t | u(E) 6.75, 7.98, 14.60, 23.85 kg; ν_eff 5, 9, 109, 783; U 18, 19, 29, 48 kg | 6.75, 7.98, 14.60, 23.86; ν_eff 5, 9.8, 37, 157; U 17.9, 18.5, 30.3, 48.1 | ✅ u(E); ν_eff differs (F69) |
| CG18-H4 | error function E = a₁I, 400 g balance, s_m = 0, 0.05, 0.25 mg | a₁ 0.00083/0.00084; U(â) 5.109·10⁻⁸, 5.637·10⁻⁸, 1.745·10⁻⁷; χ² 12.5, 7.3; u(E_appr) Tables H4.6, H4.8; 2 points fail C2.2-2b at s_m = 0.05 | same; minimal s_m 0.044 mg (C2.2-2a), 0.22 mg (C2.2-2b) | ✅ (F70) |
| CG18-H1.4/A | weighing result in use, 220 g balance (5e fit, built-in adjustment, ΔT = 3 K) | a₁ 6.709·10⁻⁶, u(a₁) 1.242·10⁻⁶; α² 1.467·10⁻⁸ g², β² 8.390·10⁻¹²; U_gl = 2.422·10⁻⁴ g + 1.150·10⁻⁵R; R_min,SF 0.0729 g (1 %, SF 3) | 6.710·10⁻⁶, 1.242·10⁻⁶; 1.466·10⁻⁸, 8.389·10⁻¹²; 2.422·10⁻⁴ + 1.150·10⁻⁵R; 0.07292 g | ✅ |
| CG18-H1.4/B | same, adjusted before calibration (5c fit) | a₁ 3.895·10⁻⁷ (sign omitted), u(a₁) 6.337·10⁻⁷; tare 5.774·10⁻⁷; β² 6.433·10⁻¹²; U_gl slope 4.479·10⁻⁶; R_min,SF 0.0727 g | −3.898·10⁻⁷, 6.332·10⁻⁷; tare 1.155·10⁻⁶ (7.4.4-5); 7.432·10⁻¹²; 4.851·10⁻⁶; 0.07277 g | ✅ except tare (F76) |
| CG18-H2.4/A | weighing result in use, 60 kg multi-interval (5d fit), three PWI | a₁ 1.717·10⁻⁴ (sign omitted), u 6.459·10⁻⁵; α² 1.867/9.917/16.167 g²; β² 4.589·10⁻⁸; slopes 2.574/3.434/3.923·10⁻⁴; U_gl 2.733 + 4.291·10⁻⁴R, 10.190 + 5.151·10⁻⁴(R − 12 kg), 20.311 + 5.641·10⁻⁴(R − 30 kg); R_min,SF 598 g | same; U(W) at 12 and 30 kg 8.129 and 15.16 g (printed 10.190, 20.311) | ✅ except two intercepts (F75) |
| CG18-H2.4/B | same, adjusted before calibration (5c fit) | a₁ 0, u 6.043·10⁻⁵; α² 1.467/7.417/13.667 g²; β² 2.449·10⁻⁸; U_gl 2.422 + 1.706·10⁻⁴R, 6.616 + 2.355·10⁻⁴(R − 12 kg), 11.951 + 2.744·10⁻⁴(R − 30 kg); R_min,SF 502 g | same; 501.5 g | ✅ |
| GUM5-5 | GUM-5 Example 5, GHG inventory (agriculture A4ci, UK 2018), independent and shared activity data | E 4282.04 kt, u 835.16 kt; Table 5.3 contributions | E 4282.14 (F₁ rounded), u 835.16; contributions to 10⁻⁴ kt; u_cor − u_ind as (5.9) | ✅ (F84 class a/b) |
| GUM5-7 | JCGM GUM-5:2026 Example 7, mass calibration (= JCGM 101 9.3) | GUF1 1.2340, 0.0539, [1.1285, 1.3395] mg; GUF2 0.0750, [1.0870, 1.3810]; MCM 0.0755 | same; second-order estimate term 0 | ✅ |
| GUM5-16 | JCGM GUM-5:2026 Example 16, comparison loss, r = 0 and 0.9 | Table 16.1 GUF2 δy 50/150/2550, u 50/112/502; Table 16.2 analytic u 67/121/505 (×10⁻⁶) | δy with the Amd.1 term 50/150/2550; u 50/111.8/502.5; correlated GUF2 (D57) 67.27/120.52/504.50 | ✅ (D56, D57) |
| GUM5-1 | GUM-5 Example 1, pH by interpolation (25 °C; correlated standards; 23.7 °C) | 7.0002, u 0.0041; u 0.0051; 7.0043 (printed 7.0109), u 0.0041 | 7.0002, 0.0041; 0.0051; 7.0043, u 0.0032 independent / 0.0041 same-standard correlated | ✅ except F78 |
| GUM5-2 | GUM-5 Example 2, benzo[a]pyrene by internal standard (correlated pairs) | 0.014 ng, u² 4.1·10⁻⁶ ng², ν_eff 5.2 (G.2b), 4.8 (eq. 2.3); MCM [≈ 0, 0.032] ng | 0.01403, 4.08·10⁻⁶, 5.20, 4.81; MCM script [1·10⁻⁷, 0.0321]; ensemble ν_eff 2.11 | ✅ except F79 |
| GUM5-3 | GUM-5 Example 3, molar mass of glucose | 180.1557, u 0.0038 | same | ✅ |
| GUM5-4 | GUM-5 Example 4, gravimetric CO₂/N₂ mixture (ISO 6142-1), 8 outputs | Table 4.6: Ar 4.0 ± 2.4 … CO₂ 200 345 ± 5, H₂O 0.379, N₂ 799 649 ± 5; r(CO₂, N₂) −0.86, r(Ar, N₂) −0.50; R_M (4.3.4.6) | same except CO₂ ± 3.96, H₂O 0.389, r −0.85 and −0.55; R_M identical | ✅ except F82 |
| GUM5-9 | GUM-5 Example 9, torque slope by OLS/WLS (GUF) and Bayes | OLS 1.0107 ± 0.0015 [1.0055, 1.0158], ν 2.7; WLS 1.0085 ± 0.0008 [1.0069, 1.0101], ν 14; NI-Bayes 1.0092 ± 0.0011; Inf-Bayes 1.0091 ± 0.0009 | same; WLS ν 13.31; Bayes by quadrature (script) | ✅ except F83 |
| GUM5-10 | GUM-5 Example 10, conformity of TSPM with a lognormal prior | R_C 0.5 %, R_P 0.9 % (A_U = T_U); 0.002 %, 5.8 % (0.17); 1.7 %, < 0.1 % (0.22); specific R_P ≈ 11 % at 0.225 | printed form: 0.498/0.909, 0.0020/5.78, 1.69/0.115 %; conditional h: 0.579/0.739, 0.0103/5.37, 1.80/0.055 %; posterior 11.33 % | ✅ except F80 |
| GUM5-11 | GUM-5 Example 11, tumour area from boundary pixels (Fourier series, MCM) | Table 11.1 areas; MCM n = 6 65.4 ± 1.6 (11.6.1: 65.0); n = 5 65.3, n = 7 65.0; AIC/AICc/BIC minimal at n = 2 | data confirmed by EMUE E5.1.5; areas ±0.1 (EMUE Table E5.1.1 likewise); MCM 65.45 ± 1.55 (EMUE 65.5 ± 1.6); IC with χ² = S/u₀²: n = 6/4/4; unweighted S gives n = 2 | script only; F85 (+ addendum) |
| GUM5-12 | GUM-5 Example 12, between-bottle homogeneity, Bayesian τ | MS 3.25·10⁻⁷/4.90·10⁻⁷; τ 0.00014 ± 0.00011 [0.00001, 0.00040]; σ 0.00069; HPD τ/µ [0, 0.0008], σ/µ [0.0013, 0.0020]; median 1.1, mode 0.3 µmol/mol | same; τ 1.356·10⁻⁴ ± 1.07·10⁻⁴ [5.2·10⁻⁶, 3.97·10⁻⁴]; median 1.13; mode 0; u*_bb 1.48 µmol/mol | ✅ except F81 |
| GUM5-14 | GUM-5 Example 14, ²⁰⁷Bi decay correction | 10.810 kBq, u 0.054 kBq; Table 14.1 | same | ✅ |
| GUM5-15 | GUM-5 Example 15, wire rope breaking force | 10 005.7 kN, u 9.9, U 19.4 kN; MCM u 10.1 kN | GUF same; MCM 9.93 kN | ✅ GUF; MCM differs (F77) |
| CG19-E1 | 1000 mL flask (Tanaka density) | V₂₀ 999.628 mL, u 0.017 mL, ν_eff 19, k 2.15, U 0.036 mL | 999.622; 0.0169; 19.5; 2.14; 0.036 | ✅ (F62) |
| CG19-E2 | 100 mL pycnometer | 100.014 mL, u 0.89 µL, ν_eff 13, k 2.21, U 2.0 µL | 100.0142; 0.90 µL; 12.9; 2.23 (G.6.4 truncation); 2.0 µL | ✅ (F62) |
| M3003-L3 | DMM, 1 V range | 0.95 V: u_c 5.37 µV; 500 mV: U 6.2 µV; 1 V: k 1.898 | same | ✅ (F61) |

## Eurachem UfS:2019 — uncertainty from sampling (library projects and core tests)

| ID | Example | Published | Engine | Status |
|---|---|---|---|---|
| UFS-A1 | nitrate in lettuce, 8 targets, classical ANOVA | x̄ 4345.5625; s_T 556.2804, s_samp 518.16089, s_anal 148.18063; 51.58/44.76/3.66 % | same (s_samp 518.16087) | ✅ |
| UFS-A1R | same, robust ANOVA (RANOVA) | 4408.3237; 565.40, 319.05, 167.94 | robust mean by H15 on target means 4408.319; s not reproduced | ❌ method unpublished (F63) |
| UFS-A2 | lead in soil, 10 targets, classical and ln x | 317.8; 197.55, 135.43, 17.99; U′ 85.23/11.32/85.98 %; s_G 0.66775, 0.4784, 0.0567, 0.4817; F_U 2.6032, 1.12, 2.6207; u′ 0.5111 | same | ✅ |
| UFS-A3 | dissolved iron, 6 wells, range method | errata: s_A 0.015, s_S 0.090, s_T 0.601 mg/L; U 1.8/10.5/70 % | same | ✅ (F64) |
| UFS-A3A | same, classical ANOVA | U 1.6/9.6/70 % | same | ✅ |
| UFS-A4 | vitamin A, 40 g test samples | SS 16595/14231; s_anal 28.805, s_samp 17.224; RSD_B 6.1 % (errata) | same | ✅ (F65) |
| UFS-A4S | vitamin A, 4 g test samples | V_samp −2662.15 → 0; RSD_anal 36.68 % | same | ✅ |
| UFS-A4Q | duplicate control chart | CL 11, WL 27, AL 36 %; all D % below WL | 10.88, 27.33, 35.56 %; 0 outside WL | ✅ (F65) |
| UFS-A5 | enzyme in feed, Gy | c 2160; C 540, 270; s_r 0.033, 0.13; s_t 0.143; U′ 26.8/28.6 % | 2158.5; 539.6, 269.8; 0.0325, 0.1296; 0.1427; 26.7/28.5 % | ✅ (F66) |
| UFS-A6 | Cd and P in soil, modelling (GUM) | 0.32 ± 0.06, 116 ± 26 mg/kg | 0.319 ± 0.059, 115.8 ± 26.1 | ✅ (F67) |
| UFS-D2 | unbalanced nested design | method of moments | core test: EMS coefficients and MS_anal | ✅ (core) |

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
The two projects from Bettencourt da Silva (2026) start from the 20 mean
recoveries of the spreadsheet (`"type": "given"`).
Tolerances reflect the documents' printed rounding; where a guide computes
from rounded intermediates the tolerance is widened and the reason stated in
the file (MUPR B1 at 50 mg/L, B5 with t = 4.30; Nordtest §8 B, C).

## Planned cases

TN 1900 observation-equation examples (regression, random effects); TN 1900
E5 (truncated Gaussian inputs); ISO/TS 28037 data sets (the GLS engine of ISO 6143 covers the method); MUPR Appendix C
allowance as a project file; cross-checks against the NIST Uncertainty
Machine (F29) and Suncal.
| EUR-CA-B1–B3 | Eurachem/CITAC compliance guide 2021, Annex B Examples 1–3 and Table 1 | B1 16.2–17.8 % Ni, non-compliant; B2 204.1 ng/g, compliant; B3 3.6 (lognormal) / 3.2 (normal) ng/g; Table 1 51/149, 61/164, 18/182, 44/227 | 16.164–17.836; 204.09; 3.557 / 3.151 (exact s_G 3.498); Table 1 with k = 1.6449 (227.6) | ✅ except F86 |
| EMUE-E3.4 | EMUE E3.4, metals in soil (ISO 21748, ruggedness coefficients, zero-intercept calibration) | Tables E3.4.1–2; u′(Pb)² = 0.002 526; (27.8 ± 2.8) mg/kg; calibration u_rel 0.019 | same to rounding; RMS of u(δ̂)/x_ref 3.50 % (printed 3.3 %) → ± 2.9 | ✅ except F87 |
| EMUE-E2.1 | EMUE E2.1, total risks of a four-component medication | R*_tot 0.0029, 0.0117, 0.0002; R_tot 0.0018 | 0.002 88, 0.011 72, 0.000 21; 0.001 80 (program), 0.001 82 (numpy) | ✅ |
