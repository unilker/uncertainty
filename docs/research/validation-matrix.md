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
`core/tests/eurachem_examples.rs`, `core/tests/topdown_examples.rs`; unit
tests in `core/src/*`.

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

## Planned cases

EURACHEM A3–A7; MUPR B2–B6; EA-4/02 S-examples; NIST TN 1900 examples; Nordtest TR 537
worked examples; Eurachem MUPR (2026) examples; ISO/TS 28037 data sets;
cross-checks against the NIST Uncertainty Machine and Suncal.
