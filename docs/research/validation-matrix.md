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
`core/tests/eurachem_examples.rs`; unit tests in `core/src/*`.

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

EURACHEM A2–A7; EA-4/02 S-examples; NIST TN 1900 examples; Nordtest TR 537
worked examples; Eurachem MUPR (2026) examples; ISO/TS 28037 data sets;
cross-checks against the NIST Uncertainty Machine and Suncal.
