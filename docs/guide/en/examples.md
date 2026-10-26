# Worked examples

This chapter is generated from the example library by `uncertainty catalogue library --lang en`; every result is recomputed by the program from the inputs stored in the project file, and the test suite fails if this file is out of date. Monte Carlo results are not listed here: run a project to obtain them together with the full budget, charts and the comparison with the published reference values.

- Chemistry (31)
- Electrical and RF (10)
- Flow and volume (3)
- General (method validation) (3)
- Length (7)
- Mass (3)
- Temperature (3)

## Chemistry

### Preparation of a cadmium calibration standard

`quam-a1-cadmium-standard` — EURACHEM/CITAC Guide CG 4, Example A1, Tables A1.2 and A1.3

EURACHEM/CITAC QUAM Example A1: 1000 mg/L Cd standard from high-purity metal (summary budget).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. c_Cd = 1000 · m · P / V: 100.28 mg of Cd of purity 0.9999 dissolved and made up to 100 mL.
2. The purity is given as limits without a confidence level: rectangular distribution, u(P) = 0.0001/√3.
3. The volume of the 100 mL flask combines calibration, repeatability and temperature effects into u(V) = 0.07 mL (see the project with the volume sub-model).
4. The volume dominates; u_c = 0.86 mg/L and U = 2 × 0.86 ≈ 1.7 mg/L. The guide rounds u_c to 0.9 and reports (1002.7 ± 1.8) mg/L.

**Result:**

- c_Cd = (1002.7 ± 1.7) mg/L, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- The guide's spreadsheet (Kragten) value 0.8633 mg/L differs from the exact first-order value 0.8638 mg/L only by the finite-difference approximation; the reported result is unchanged.
- Doubling the rounded u_c (0.9 → U = 1.8 mg/L) is more conservative than 2 × 0.86 = 1.7 mg/L; the program rounds U itself to two significant digits.

**Run:** `uncertainty run library/chemistry/quam-a1-cadmium-standard.json --lang en`

### Cadmium standard with the volume sub-model

`quam-a1-cadmium-standard-volume` — EURACHEM/CITAC Guide CG 4, Example A1, A1.4

QUAM Example A1 with the volume of the flask modelled from its three components.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. c_Cd = 1000 · m · P / V: 100.28 mg of Cd of purity 0.9999 dissolved and made up to 100 mL.
2. The purity is given as limits without a confidence level: rectangular distribution, u(P) = 0.0001/√3.
3. V = V_nom + δV_cal + δV_rep + δV_temp: calibration ±0.1 mL (triangular, 0.04 mL), repeatability s = 0.02 mL, temperature ±4 °C × 2.1 × 10⁻⁴ °C⁻¹ × 100 mL = ±0.084 mL (rectangular, 0.05 mL) → u(V) = 0.066 mL.
4. Rectangular and triangular components dominate, so the output PDF has lighter tails than a Gaussian: the Monte Carlo interval is about 0.015 mg/L shorter at each end, and the GUM interval is validated only for one significant digit of u.

**Result:**

- c_Cd = (1002.7 ± 1.7) mg/L, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- The guide rounds u(V) = 0.066 mL to 0.07 mL and u_c to 0.9 mg/L before multiplying by k; with unrounded values u_c = 0.835 mg/L and U = 1.7 mg/L, against the printed (1002.7 ± 1.8) mg/L.

**Run:** `uncertainty run library/chemistry/quam-a1-cadmium-standard-volume.json --lang en`

### Standardising a sodium hydroxide solution

`quam-a2-naoh-standardisation` — EURACHEM/CITAC Guide CG 4, Example A2, Tables A2.2 and A2.3

QUAM Example A2: NaOH standardised against potassium hydrogen phthalate (KHP); summary budget with the guide's standard uncertainties.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. c_NaOH = 1000 · m_KHP · P_KHP / (M_KHP · V_T) · rep (mol/L), where rep is a repeatability factor with relative standard uncertainty 0.05 %.
2. For a product/quotient model, relative uncertainties combine in quadrature: u_c/c = √[(0.0005)² + (0.00013/0.3888)² + (0.00029)² + (0.0038/204.2212)² + (0.013/18.64)²].
3. The titration volume and the repeatability dominate; c = (0.10214 ± 0.00020) mol/L with k = 2.

**Result:**

- c_NaOH = (0.10214 ± 0.00020) mol/L, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/chemistry/quam-a2-naoh-standardisation.json --lang en`

### NaOH standardisation with sub-models

`quam-a2-naoh-standardisation-submodels` — EURACHEM/CITAC Guide CG 4, Example A2, A2.4

QUAM Example A2 with mass, molar mass and volume built from their components.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. m_KHP = m_gross − m_tare; each weighing carries a balance linearity term ±0.15 mg (rectangular).
2. M_KHP = 8A_C + 5A_H + 4A_O + A_K from IUPAC atomic weights; the atoms of one element are fully correlated, which the model expresses by multiplying the single atomic weight by the number of atoms.
3. V_T = 18.64 mL + burette calibration (±0.03 mL, triangular) + temperature effect (19 mL × 2.1 × 10⁻⁴ °C⁻¹ × 3 °C / 1.96, normal).
4. Result: (0.1021 ± 0.0002) mol/L with k = 2, as in the guide.

**Result:**

- c_NaOH = (0.1021 ± 0.0002) mol/L, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Finding: the guide rounds 0.15/√3 = 0.087 mg to 0.09 mg, giving u(m) = 0.13 mg; unrounded u(m) = 0.12 mg. Likewise u(V_T) = 0.0137 mL (guide 0.013). The unrounded combined uncertainty is 0.000101 mol/L (guide 0.000099); the reported result is unchanged.

**Run:** `uncertainty run library/chemistry/quam-a2-naoh-standardisation-submodels.json --lang en`

### Acid–base titration of hydrochloric acid

`quam-a3-hcl-titration` — EURACHEM/CITAC Guide CG 4, Example A3, Tables A3.2 and A3.3

QUAM Example A3: HCl titrated with NaOH that was itself standardised against KHP.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. c_HCl = 1000 · m_KHP · P_KHP · V_T2 / (V_T1 · M_KHP · V_HCl) · rep: V_T1 is the NaOH volume in the standardisation, V_T2 in the titration of 15 mL of HCl.
2. The repeatability of the whole two-step procedure is 0.1 % (rep), the largest single contribution.
3. c = (0.1014 ± 0.0004) mol/L with k = 2.

**Result:**

- c_HCl = (0.1014 ± 0.0004) mol/L, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Finding: summary Table A3.1 states u_c = 0.00016 mol/L, but its own inputs give 0.00019 mol/L; 0.00016 is the result of the triple determination (Table A3.4).

**Run:** `uncertainty run library/chemistry/quam-a3-hcl-titration.json --lang en`

### Organophosphorus pesticides in bread: combination

`quam-a4-pesticides-combination` — EURACHEM/CITAC Guide CG 4, Example A4, Tables A4.4 and A4.5

QUAM Example A4: the relative uncertainty from in-house validation data (precision, recovery, homogeneity) combined in a multiplicative model at P = 1.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. P_op = F_I · F_hom / Rec, evaluated at nominal values: precision 0.27 (from 15 duplicates), recovery 0.90 with u = 0.043 (correction applied because t = 2.31 > t_crit), homogeneity 0.20.
2. Relative uncertainty: √(0.27² + 0.048² + 0.20²) = 0.34; at P = 1.1111 the guide's spreadsheet gives u = 0.377.
3. With relative uncertainties of 20–27 % the model is visibly non-linear: the Monte Carlo distribution is skewed and its standard deviation differs from the GUM value.

**Result:**

- P_op = (1.11 ± 0.74), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- Finding: Table A4.2 contains typographical errors in derived columns (mean 0.75 for 0.075, difference −0.10 for −0.01, mean 0.10 for 0.01); the guide quotes t_crit = 2.021, i.e. t(40), for 41 degrees of freedom (t(41) = 2.0195). Neither changes the conclusion.

**Run:** `uncertainty run library/chemistry/quam-a4-pesticides-combination.json --lang en`

### Cadmium released from ceramic ware

`quam-a5-cadmium-release` — EURACHEM/CITAC Guide CG 4, Example A5, Tables A5.3 and A5.4

QUAM Example A5: empirical method (BS 6748); amount of Cd leached per unit area.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. r = c₀ · V_L / a_V · f_acid · f_time · f_temp (mg/dm²): c₀ from a straight-line AAS calibration (u(c₀) = 0.018 mg/L, see calibration module), leach volume 0.332 L, surface area 5.73 dm².
2. The f factors represent the effects of the empirical method's conditions (acid concentration, time, temperature); for an empirical method they are 1 by definition but carry uncertainty.
3. The calibration (c₀) and the temperature factor dominate; r = (0.015 ± 0.003) mg/dm² with k = 2.

**Result:**

- r = (0.015 ± 0.003) mg/dm², where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Finding: a_V = πd²/4 with d = 2.70 dm (the guide's formula shows 2.77, a typo); its components 0.042 and 0.146 dm² combine to 0.152 dm², not the 0.19 dm² printed and used here. With 0.152 dm², u_c = 0.00144 mg/dm² instead of 0.00147; the reported result is unchanged.
- The exact first-order u_c is 0.001470 mg/dm²; the spreadsheet value 0.001465 differs because of the 1/a_V non-linearity in the finite differences.

**Run:** `uncertainty run library/chemistry/quam-a5-cadmium-release.json --lang en`

### Lead assay standard for IDMS

`quam-a7-assay-standard` — EURACHEM/CITAC Guide CG 4, Example A7, Eq. (8), Table A7.3

QUAM Example A7, Eq. (8): amount content of the primary lead assay standard prepared gravimetrically.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. c_z = 10⁶ · (m₂/d₂) · m₁ · w / (d₁ · M): a mass m₁ of lead of purity w dissolved, diluted by mass (d₁, d₂) and an aliquot m₂ taken; M is the molar mass of lead in the assay material.
2. All inputs are Gaussian with the standard uncertainties of Table A7.3; c_z = 0.092605 µmol/g, u = 0.000028 µmol/g.

**Result:**

- c_z = (0.092605 ± 0.000056) µmol/g, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/chemistry/quam-a7-assay-standard.json --lang en`

### Lead in water by double isotope dilution

`quam-a7-lead-double-idms` — EURACHEM/CITAC Guide CG 4, Example A7, Eq. (5), Tables A7.4–A7.8

QUAM Example A7, Eq. (5): double IDMS with mass-bias-corrected isotope ratios (replicate 1).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. c_x = c_z · (m_y/m_x) · (m_z/m_y') · (K_y1R_y1 − K_bR_b)/(K_bR_b − K_x1R_x1) · (K_b'R_b' − K_z1R_z1)/(K_y1R_y1 − K_b'R_b') · ΣK_xiR_xi/ΣK_ziR_zi − c_blank, with every K = K₀ + K_bias.
2. Type A uncertainties of the ratios and K₀ are divided by √8 (mean of eight measurements); the bias terms K_bias (u = 0.001 each) contribute about 38 % of the variance.
3. c_x = 0.053738 µmol/g with u_c = 0.00018 µmol/g.

**Result:**

- c_x = (0.05374 ± 0.00036) µmol/g, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- The reported (0.05370 ± 0.00036) µmol/g uses the mean of four replicates; without the division by √8 the experimental u_c would be 0.00041 µmol/g.

**Run:** `uncertainty run library/chemistry/quam-a7-lead-double-idms.json --lang en`

### Nitrate in drinking water: two precision intervals

`mupr-b1-nitrate` — Eurachem/CITAC Guide, Example B1

MUPR Example B1: absolute precision below 0.4 mg/L, relative above; recovery from 20 spiked waters; stock solution and dilution components.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Interval I (c < 0.4 mg/L): s_I = 0.0328 mg/L; interval II: s′_I = 8.43 % (with dilution F = 100, u′_F = 2.2 %).
2. R̄ = 0.389/0.400 = 97.25 %, u(R̄) = 2.85 %; |1 − R̄|/u = 0.96 ≤ t₁₉ = 2.09: no correction, u′ = u(R̄).
3. U = 2√(s² + c²(u′²_R + 2u′²_DSS [+ u′²_F])): (0.261 ± 0.068) mg/L and (50.0 ± 9.3) mg/L.

**Result:**

- c(NO₃⁻) = (0.261 ± 0.068) mg/L (k = 2.00)
- c(NO₃⁻) = (50.0 ± 9.3) mg/L (k = 2.00)

**Notes:**

- Finding: the guide prints |1 − R̄|/u = 0.982 using R̄ rounded to 0.972; with R̄ = 0.9725 the statistic is 0.963. The conclusion is the same.

**Run:** `uncertainty run library/chemistry/mupr-b1-nitrate.json --lang en`

### Nitrate: pooled precision and two recovery levels

`mupr-b2-pooled-precision` — Eurachem/CITAC Guide, Example B2, Tables B2.1–B2.3

MUPR Example B2: precision pooled from several materials (raw data for two of them), transition concentration and overall recovery.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Pooled s_I⟨I⟩ = 0.0285 mg/L (Eq. B2.1) and s′_I⟨II⟩ = 7.73 % (Eq. B2.2); the two models meet at c_T = 0.0285/0.0773 = 0.37 mg/L.
2. Recoveries 97.2 % and 102.0 % are compatible; R̿ = 99.6 %, u = 2.24 %, not significant (38 degrees of freedom).
3. At 50 mg/L: U = 0.169 c = 8.45 mg/L.

**Result:**

- c(NO₃⁻) = (50.0 ± 8.4) mg/L (k = 2.00)

**Notes:**

- The guide divides the rounded s_I(2) = 0.0250 by 0.3033 (8.24 %); unrounded 8.25 %. Group 2 enters the absolute pool, so the budget is unaffected.

**Run:** `uncertainty run library/chemistry/mupr-b2-pooled-precision.json --lang en`

### Nitrate: result as the mean of 3 replicates on 2 days

`mupr-b2-replicates-on-two-days` — Eurachem/CITAC Guide, Example B2, Eq. (B2.8)–(B2.9)

MUPR Example B2, Eq. (B2.9): precision of a mean of n results on each of p days (Eq. 5).

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. s′_I(x̿) = √(s′_I²/p + s′_r²(1 − n)/(n p)) with s′_I = 7.73 %, s′_r = 3.4 %, n = 3, p = 2.
2. At 57.5 mg/L: U = 7.06 mg/L (7.41 mg/L for one result on each of two days).

**Result:**

- c(NO₃⁻) = (57.5 ± 7.0) mg/L (k = 2.00)

**Run:** `uncertainty run library/chemistry/mupr-b2-replicates-on-two-days.json --lang en`

### Nitrate: precision as a linear function of concentration

`mupr-b3-linear-precision` — Eurachem/CITAC Guide, Example B3, Table B3.1, Eq. (B3.1)

MUPR Example B3: the intermediate precision of four materials is modelled as s_I = a + b·c by least squares and applied to a diluted sample.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Table B3.1: (c, s_I) = (0.389, 0.0328), (0.303, 0.0250), (0.204, 0.0245), (0.785, 0.0322) mg/L give s_I = 0.0232 + 0.0128 c (r = 0.727).
2. The sample is diluted 100-fold, so the model is applied to the test solution: s(c_s) = 100·(a + b·c_s/100); at c_s = 57.5 mg/L, s = 3.06 mg/L.
3. With the recovery (2.24 %), stock solution (2 × 0.95 %) and dilution (2.2 %) components: U = 7.28 mg/L.

**Result:**

- c(NO₃⁻) = (57.5 ± 7.3) mg/L (k = 2.00)

**Notes:**

- Finding: least squares gives the intercept 0.02324 (printed 0.0233); slope 0.0128 and r = 0.7266 agree.

**Run:** `uncertainty run library/chemistry/mupr-b3-linear-precision.json --lang en`

### Arsenic in sediment: one CRM

`mupr-b4-arsenic-one-crm` — Eurachem/CITAC Guide, Example B4

MUPR Example B4: pooled relative precision and the recovery of one CRM.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. s′_I(P) = 4.52 % pooled from three materials (Eq. B4.1); below 6.46 mg/kg it is used as an absolute 4.52 % × 6.46 = 0.292 mg/kg.
2. CRM WQB-1 (23.00 ± 1.84) mg/kg, mean 21.040 mg/kg: R̄ = 91.5 %, u = 3.81 %; 2.24 ≤ t₉ = 2.26: not corrected.
3. U = 2w√(0.0452² + 0.0381²) = 0.118 w: (16.0 ± 1.9) mg/kg.

**Result:**

- w(As) = (16.0 ± 1.9) mg/kg (k = 2.00)

**Run:** `uncertainty run library/chemistry/mupr-b4-arsenic-one-crm.json --lang en`

### Arsenic: CRM and spiked samples with matrix-dependent recovery

`mupr-b5-crm-and-spiked-samples` — Eurachem/CITAC Guide, Example B5

MUPR Example B5: recoveries vary with the matrix, so their standard deviation is included and k = t(2).

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. R̿ = 94.98 % from the CRM and two spiked sediments; s(R) = 7.32 %, u(R̿) with s(R) = 8.00 % (Eq. B5.1–B5.3).
2. |1 − R̿|/u = 0.63 ≤ t₂ = 4.30: not corrected; with ν = 2 the coverage factor is t₀.₉₇₅(2) = 4.30 and U = 0.395 w.

**Result:**

- w(As) = (16.0 ± 6.3) mg/kg (k = 4.30)

**Run:** `uncertainty run library/chemistry/mupr-b5-crm-and-spiked-samples.json --lang en`

### Trace element in sediment: heterogeneity and 22 proficiency tests

`mupr-b6-sediment-22-pt` — Eurachem/CITAC Guide, Example B6, Tables B6.1–B6.3

MUPR Example B6: precision from a homogeneous material plus the heterogeneity of real samples (duplicate ranges), recovery from 22 PT results (raw data).

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. A homogeneous material measured 30 times gives s′_I(DS) = 7.34 %; the ranges of 30 duplicate real samples give s_r(h) = Ā/1.128 = 0.191 mg/kg below 7.5 mg/kg and s′_r(h) = 2.66 % above.
2. Interval I: s_I = √((7.5 × 7.34 %)² + 0.191²) = 0.583 mg/kg; interval II: s′_I = √(7.34² + 2.66²) = 7.81 %.
3. The 22 PT recoveries are equivalent (χ² = 8.70 ≤ 32.7), so s(R) is not added; R̿ = 91.7 %, u = 1.59 %, |1 − R̿|/u = 5.2 > t₂₁: results are corrected (w/R̿) and u′ = u/R̿.
4. Interval II: U = 2(w/R̿)√(0.0781² + 0.0173²) = 0.160 w/R̿.

**Result:**

- w = (5.4 ± 1.3) mg/kg (k = 2.00)
- w = (21.8 ± 3.5) mg/kg (k = 2.00)

**Notes:**

- Finding: Eq. (B6.7) computes χ² about the unweighted mean (8.80); Eq. (16) prescribes the weighted mean, giving 8.70. The conclusion is the same.
- Finding: Eq. (B6.8) as printed omits 1/R̿ on the precision term of the corrected result; for absolute components the program uses u(w/R̿)² = (s_I/R̿)² + (w u_R̿/R̿²)². In interval II (all relative) both agree.
- The standard deviation of each single PT result is taken from the homogeneous-material model (7.34 % above 7.5 mg/kg, 7.5 × 7.34 % below), as in Table B6.3.

**Run:** `uncertainty run library/chemistry/mupr-b6-sediment-22-pt.json --lang en`

### Recovery from two CRMs and a proficiency-test sample

`mupr-ex2-reference-materials` — Eurachem/CITAC Guide, Example 2

MUPR Example 2: ortho-phosphate; mean recovery and its uncertainty from three reference materials.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Rᵢ = c̄ᵢ/Cᵢ with u²(Rᵢ) = Rᵢ²[(sᵢ/(c̄ᵢ√nᵢ))² + (u(Cᵢ)/Cᵢ)²] (Eq. 9); terms 0.000731, 0.001476, 0.01015.
2. R̄ = 1.008, u(R̄) = 0.0371: not significantly different from 1.
3. The mean recovery R̄ = ΣRᵢ/N has u(R̄) = √Σu²(Rᵢ)/N (MUPR Eq. 8–12); it is corrected for only if |1 − R̄|/u(R̄) exceeds the two-sided t value (Eq. 13).

**Result:**

- U = 7.4 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/chemistry/mupr-ex2-reference-materials.json --lang en`

### Recovery from spiked river waters

`mupr-ex3-spiked-samples` — Eurachem/CITAC Guide, Example 3

MUPR Example 3: two spiked samples with native analyte (Eq. 10–11).

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Rᵢ = (c̄ − c̄₀)/c₊: 103.6 % and 88.8 %; the uncertainty of B (one result before spiking, small spike) dominates.
2. R̄ = 96.2 %, u(R̄) = 10.0 % (1.5 % with sample A alone).
3. The mean recovery R̄ = ΣRᵢ/N has u(R̄) = √Σu²(Rᵢ)/N (MUPR Eq. 8–12); it is corrected for only if |1 − R̄|/u(R̄) exceeds the two-sided t value (Eq. 13).

**Result:**

- U = 20 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/chemistry/mupr-ex3-spiked-samples.json --lang en`

### Ammonium in water from control limits and proficiency tests

`nordtest-3-ammonium-pt` — Nordtest TR 537 ed. 4 (2017), §3.2 and Appendix 4

Nordtest TR 537 §3.2 and Appendix 4: NH₄-N, u(Rw) from the control chart and u(bias) from six proficiency tests (raw PT data).

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. The control chart warning limits are ±3.34 % (2s), so u(Rw) = 3.34/2 = 1.67 %.
2. For each PT round the bias is (x_lab − x_assigned)/x_assigned and the uncertainty of the assigned value u(C_ref,i) = s_R/√n_Lab; RMS_bias = 2.26 %, ū(C_ref) = 1.52 %.
3. u(bias) = √(2.26² + 1.52²) = 2.73 %; u_c = √(1.67² + 2.73²) = 3.20 %; U = 6.4 % → 7 %.
4. u_c = √(u(Rw)² + u(bias)²) and U = 2u_c, rounded up to a whole per cent (Nordtest §3).

**Result:**

- U = 7 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/chemistry/nordtest-3-ammonium-pt.json --lang en`

### Ammonium: within-laboratory reproducibility from control samples and duplicates

`nordtest-5.3-ammonium-duplicates` — Nordtest TR 537 ed. 4 (2017), §5.3 and Appendix 5

Nordtest TR 537 §5.3 and Appendix 5: NH₄-N duplicates of natural samples in two ranges (raw data), combined with the control-sample reproducibility.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. The control sample does not cover sample-to-sample variation, so the repeatability from duplicates of real samples is added: u(Rw) = √(u(Rw, control)² + s_r(duplicates)²).
2. Below 30 µg/L the standard deviation is constant: s_r = √(Σd²/2N) = 0.44 µg/L from 46 pairs; with the control sample (0.5 µg/L) u(Rw) = 0.67 µg/L.
3. At and above 30 µg/L it is proportional to the concentration: pooled relative s_r = 3.8 % from 26 pairs; with the control sample (1.5 %) u(Rw) = 4.1 %.
4. Each component is restricted to its range; the budget is shown at 10 and 100 µg/L.

**Result:**

- c(NH₄-N) = (10.0 ± 1.3) µg/L (k = 2.00)
- c(NH₄-N) = (100.0 ± 8.2) µg/L (k = 2.00)

**Notes:**

- Appendix 5 lists 46 pairs below 30 µg/L; §5.3 states n = 47. The statistics are recomputed from the 46 listed pairs.

**Run:** `uncertainty run library/chemistry/nordtest-5.3-ammonium-duplicates.json --lang en`

### Dissolved oxygen: reproducibility from duplicates

`nordtest-5.3-oxygen-duplicates` — Nordtest TR 537 ed. 4 (2017), §5.3 and Appendix 6

Nordtest TR 537 §5.3 and Appendix 6: dissolved oxygen in sea water, 51 duplicate pairs (raw data) and a control-sample component.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. s_r = √(Σd²/2N) = 0.0252 mg/L at a mean of 7.50 mg/L, i.e. 0.34 %.
2. With the control-sample reproducibility (0.5 %) u(Rw) = √(0.34² + 0.5²) = 0.60 %.

**Result:**

- U = 1.2 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/chemistry/nordtest-5.3-oxygen-duplicates.json --lang en`

### Bias from one certified reference material

`nordtest-6.1-one-crm` — Nordtest TR 537 ed. 4 (2017), §6.1

Nordtest TR 537 §6.1: u(bias) from a CRM (11.5 ± 0.5) measured 12 times.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Bias b = (11.9 − 11.5)/11.5 = 3.48 %; s_bias = 2.2 % (n = 12); u(C_ref) = 0.25/11.5 = 2.17 %.
2. For one reference material u(bias) = √(b² + (s_bias/√n)² + u(C_ref)²) = 4.15 %.

**Result:**

- U = 8.3 % (k = 2.00), relative to the measured value

**Notes:**

- Finding (F5): the handbook prints u(C_ref) = 0.25/11.5 = 2.16 % but 0.25/11.5 = 2.17 %, and its result u(bias) = 4.1 % follows from 2.16 %; the correct value is 4.15 %.

**Run:** `uncertainty run library/chemistry/nordtest-6.1-one-crm.json --lang en`

### Bias from several certified reference materials

`nordtest-6.1-three-crms` — Nordtest TR 537 ed. 4 (2017), §6.1

Nordtest TR 537 §6.1: u(bias) from three CRMs by the RMS of the biases.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Biases 3.48, −0.9 and 2.5 %: RMS_bias = √(Σbᵢ²/N) = 2.53 %; ū(C_ref) = (2.16 + 1.8 + 1.8)/3 = 1.92 %.
2. u(bias) = √(RMS_bias² + ū(C_ref)²) = 3.2 %.

**Result:**

- U = 6.3 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/chemistry/nordtest-6.1-three-crms.json --lang en`

### Bias from proficiency tests

`nordtest-6.2-proficiency-tests` — Nordtest TR 537 ed. 4 (2017), §6.2

Nordtest TR 537 §6.2: u(bias) from six proficiency tests.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. u(C_ref,1) = s_R/√n_Lab = 8.7/√23 = 1.8 %; the other five values are 2.9, 1.7, 4.1, 3.0 and 2.1 %; ū(C_ref) = 2.6 %.
2. Biases +2, +7, −2, +3, +6, +5 %: RMS_bias = 4.6 %; u(bias) = √(4.6² + 2.6²) = 5.3 %.

**Result:**

- U = 10 % (k = 2.00), relative to the measured value

**Notes:**

- Finding: the mean of the six u(C_ref,i) is 2.60 %; the handbook prints 2.7 % in the step table and then uses 2.6 %.

**Run:** `uncertainty run library/chemistry/nordtest-6.2-proficiency-tests.json --lang en`

### Bias from recovery experiments

`nordtest-6.3-recovery` — Nordtest TR 537 ed. 4 (2017), §6.3

Nordtest TR 537 §6.3: u(bias) from six spike recoveries.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Recoveries 95, 98, 97, 96, 99, 96 %: biases R − 100 %, RMS_bias = 3.44 %.
2. Uncertainty of the added amount: concentration of the spike solution 1.2/2 = 0.6 %, volume (±1 % rectangular → 1/√3 = 0.58 %, and 0.5 %) → u(C_recovery) = √(0.6² + 0.58² + 0.5²) = 0.97 % ≈ 1.0 %.
3. u(bias) = √(3.44² + 0.97²) = 3.6 %.

**Result:**

- U = 7.1 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/chemistry/nordtest-6.3-recovery.json --lang en`

### BOD in wastewater: control chart and one CRM

`nordtest-8-bod-crm` — Nordtest TR 537 ed. 4 (2017), §8.2 Example A, Appendix 7

Nordtest TR 537 §8.2 Example A with the raw control data of Appendix 7.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. The averages of the 18 duplicate control analyses of the CRM give s = 5.6 mg/L, i.e. u(Rw) = 2.6 % at 214.8 mg/L.
2. The same results give the bias against the certified value (206 ± 5) mg/L: b = 4.3 %, u(C_ref) = 2.5/206 = 1.2 %, u(bias) = √(b² + s²/n + u(C_ref)²) = 4.5 %.
3. u_c = √(2.6² + 4.5²) = 5.2 %; U = 10.4 % → 11 %.

**Result:**

- U = 11 % (k = 2.00), relative to the measured value

**Notes:**

- The handbook states n = 19 for s_bias; Appendix 7 lists 18 pairs, which are used here. The averages are computed from the pairs (the appendix prints them rounded to 1 mg/L).

**Run:** `uncertainty run library/chemistry/nordtest-8-bod-crm.json --lang en`

### BOD in wastewater: control chart and proficiency tests

`nordtest-8-bod-pt` — Nordtest TR 537 ed. 4 (2017), §8.2 Example B, Table 6

Nordtest TR 537 §8.2 Example B: u(bias) from three proficiency tests (Table 6, raw data).

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. u(Rw) = 2.6 % as in Example A.
2. u(C_ref,i) = s_R/√n: 1.50, 1.32, 2.25 % (mean 1.69 %); RMS of the biases 3.77 % (3.76 % from the rounded biases of Table 6).
3. u(bias) = 4.13 %, u_c = 4.88 %, U = 9.8 % → 10 %.

**Result:**

- U = 10 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/chemistry/nordtest-8-bod-pt.json --lang en`

### PCB in sediment: control sample and one CRM

`nordtest-8-pcb-crm` — Nordtest TR 537 ed. 4 (2017), §8.3 Example C

Nordtest TR 537 §8.3 Example C.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. u(Rw) = 8 % from the control chart; CRM (152 ± 14) µg/kg, mean result 144 µg/kg: b = −5.3 %, s_bias = 8 % (n = 22), u(C_ref) = 7/152 = 4.6 %.
2. u(bias) = 7.2 %, u_c = √(8² + 7.2²) = 10.8 %, U = 21.5 % → 22 %.

**Result:**

- U = 22 % (k = 2.00), relative to the measured value

**Notes:**

- From the stated values b = 5.26 % and u(C_ref) = 4.61 % give u(bias) = 7.20 %; the handbook uses the rounded 5.3 % and 4.6 % (7.22 %). Its summary table prints u(C_ref) = 4.7 % and u(bias) = 7.29 %.

**Run:** `uncertainty run library/chemistry/nordtest-8-pcb-crm.json --lang en`

### PCB in sediment: control sample and proficiency tests

`nordtest-8-pcb-pt` — Nordtest TR 537 ed. 4 (2017), §8.3 Example D

Nordtest TR 537 §8.3 Example D.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Biases −2, −12, −5 %: RMS_bias = 7.6 %; u(C_ref,i) 2.7, 2.5, 3.5 % → 2.9 %; u(bias) = 8.1 %.
2. u_c = √(8² + 8.1²) = 11.4 %, U = 22.8 % → 23 %.

**Result:**

- U = 23 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/chemistry/nordtest-8-pcb-pt.json --lang en`

### Organophosphorus pesticides in bread from validation data

`quam-a4-pesticides-topdown` — EURACHEM/CITAC Guide CG 4, Example A4, Tables A4.2–A4.5

EURACHEM QUAM Example A4 as a top-down evaluation: precision from 15 duplicates (raw data), recovery from 42 spikes, homogeneity.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Duplicates measured in different runs: the standard deviation of the normalised differences (x₁ − x₂)/x̄ is 0.382, so the relative intermediate precision is 0.382/√2 = 27 %.
2. Recovery: mean 90 %, s = 28 % from 42 spikes, u = 28/√42 = 4.3 %; t = 2.31 > t₄₁ = 2.02: the result is corrected and u′ = 4.3/90 = 4.8 %.
3. Inhomogeneity of the loaf (binomial model, scenario b): 20 %.
4. At a measured level of 1: P = 1/0.90 = 1.111 with u = 1.111 × √(0.27² + 0.048² + 0.20²) = 0.377.

**Result:**

- P_op = (1.11 ± 0.75) (k = 2.00)

**Notes:**

- Finding: Table A4.2 contains typographical errors in derived columns (mean 0.75 for 0.075, difference −0.10 for −0.01, mean 0.10 for 0.01); the statistics are recomputed from D1 and D2. The guide quotes t_crit = 2.021, i.e. t(40), for 41 degrees of freedom (t(41) = 2.0195).

**Run:** `uncertainty run library/chemistry/quam-a4-pesticides-topdown.json --lang en`

### Crude fibre in animal feed (empirical method)

`quam-a6-crude-fibre` — EURACHEM/CITAC Guide CG 4, Example A6, Tables A6.4–A6.5

EURACHEM QUAM Example A6: reproducibility s_R from a collaborative trial at three levels plus a drying allowance.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. For an empirical method the collaborative-trial reproducibility s_R (Table A6.4) is the main component; values are in % m/m (absolute), so the fields have no _percent suffix.
2. Drying to constant weight: ±2 mg on a 1 g sample = ±0.2 % m/m, rectangular, u = 0.115 % m/m; it is included only where it exceeds s_R/3, i.e. at the 2.5 % level.
3. U = 2u_c: 0.62, 0.8 and 1.2 % m/m at 2.5, 5 and 10 % m/m.

**Result:**

- w(crude fibre) = (2.50 ± 0.62) % m/m (k = 2.00)
- w(crude fibre) = (5.00 ± 0.80) % m/m (k = 2.00)
- w(crude fibre) = (10.0 ± 1.2) % m/m (k = 2.00)

**Notes:**

- Finding: Table A6.5 prints U = 0.12 % m/m at 10 % fibre; 2 × 0.6 = 1.2 % m/m (the percentage column, 12 %, is correct).

**Run:** `uncertainty run library/chemistry/quam-a6-crude-fibre.json --lang en`

## Electrical and RF

### Calibration of a 10 kΩ standard resistor

`ea402-s3-resistor-10kohm` — EA-4/02 M:2022, S3

EA-4/02 S3: four-terminal resistor calibrated by substitution against a standard, using a DMM as a ratio device.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. R_X = (R_S + δR_D + δR_TS) · r_C · r − δR_TX (values in mΩ relative to 10 kΩ).
2. The ratio r is the mean of five observations: a Type A evaluation (s/√5 = 0.071 × 10⁻⁶, 4 degrees of freedom).
3. The drift of the standard (+20 ± 10 mΩ) and the temperature effects are rectangular; the ratio correction r_C is triangular (±1 × 10⁻⁶).
4. R_X = (10 000.178 ± 0.017) Ω (k = 2).

**Result:**

- R_X = (10000178 ± 17) mΩ, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/electrical/ea402-s3-resistor-10kohm.json --lang en`

### Calibration of a power sensor at 18 GHz

`ea402-s6-power-sensor-18ghz` — EA-4/02 M:2022, S6

EA-4/02 S6: calibration factor of a thermistor power sensor against a standard sensor; mismatch terms are U-shaped.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. K_X = (K_S + δK_D) · (M_Sr·M_Xc)/(M_Sc·M_Xr) · p_Cr · p_Cc · p, where p is the observed power ratio.
2. Mismatch factors have limits 1 ± 2|Γ_G||Γ| and an arc sine (U-shaped) distribution because the phase is unknown (u = a/√2).
3. p is a Type A estimate from three repeated measurements (2 degrees of freedom), which gives ν_eff ≈ 310.
4. K_X = 0.933 ± 0.032 (k = 2).

**Result:**

- K_X = (0.933 ± 0.032), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- The heading of the example says 19 GHz; the text and the result refer to 18 GHz.
- With three observations the t distribution for p has ν = 2 and no finite variance; the Monte Carlo standard deviation is therefore unstable, while the coverage interval remains well defined (JCGM 101 6.4.9).

**Run:** `uncertainty run library/electrical/ea402-s6-power-sensor-18ghz.json --lang en`

### Calibration of a coaxial step attenuator (30 dB)

`ea402-s7-step-attenuator-30db` — EA-4/02 M:2022, S7

EA-4/02 S7: 30 dB step of a coaxial attenuator at 10 GHz, with mismatch from S-parameters.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. L_X = L_S + δL_S + δL_D + δL_M + δL_K + δL_ib − δL_ia + δL_0b − δL_0a (dB).
2. L_S is the mean of four observations (Type A, 3 degrees of freedom); the mismatch term is computed from the reflection coefficients of source, load and attenuator (Eq. S7.2): u(δL_M) = 0.0198 dB.
3. L_X = (30.043 ± 0.045) dB (k = 2).

**Result:**

- L_X = (30.043 ± 0.044) dB, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Finding: the budget prints the contribution of δL_ib as 0.0019 dB although its standard uncertainty is 0.0003 dB; the printed total 0.0223 dB includes the 0.0019 value (0.0222 dB without it). The reported result is unchanged.

**Run:** `uncertainty run library/electrical/ea402-s7-step-attenuator-30db.json --lang en`

### Calibration of a hand-held digital multimeter at 100 V

`ea402-s9-dmm-100v` — EA-4/02 M:2022, S9

EA-4/02 S9: error of indication of a 3½-digit DMM; the resolution term dominates and the output is nearly rectangular.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. E_X = V_iX − V_S + δV_iX − δV_S (V); the resolution of 0.1 V gives ±0.05 V (rectangular), u = 0.029 V.
2. Because one rectangular term dominates (ratio of the others 0.22 ≤ 0.3), EA-4/02 uses k = 0.95·√3 = 1.65 for 95 % instead of k = 2: U = 0.05 V (S9.14).
3. The Monte Carlo 95 % interval shows directly that ±2u = ±0.06 V overstates the coverage; no rule of thumb is needed.

**Result:**

- E_X = (0.10 ± 0.06) V, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/electrical/ea402-s9-dmm-100v.json --lang en`

### Comparison loss, x₁ = 0.0

`jcgm101-9.4-comparison-loss-x000` — JCGM 101:2008, 9.4, Table 8

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The comparison loss in microwave power meter calibration is δY = X₁² + X₂², where X₁ and X₂ are the real and imaginary parts of the voltage reflection coefficient, each with u = 0.005.
2. Because ∂δY/∂X₂ = 2x₂ = 0 (and ∂δY/∂X₁ = 0 when x₁ = 0), the first-order GUM result understates or even misses the uncertainty; the output PDF is a (scaled) non-central χ² distribution, strongly asymmetric.
3. Because the PDF is asymmetric and bounded below by zero, the shortest 95 % coverage interval is reported.
4. The GUM result here includes the second-order terms (higher_order), which for this quadratic model give the exact variance; the 95 % interval based on a symmetric distribution is still inappropriate.

**Result:**

- dY = (0 ± 640) 10⁻⁶, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 12.7, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x000.json --lang en`

### Comparison loss, x₁ = 0.0, r = 0.9

`jcgm101-9.4-comparison-loss-x000-correlated` — JCGM 101:2008, 9.4, Table 9

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The comparison loss in microwave power meter calibration is δY = X₁² + X₂², where X₁ and X₂ are the real and imaginary parts of the voltage reflection coefficient, each with u = 0.005.
2. Because ∂δY/∂X₂ = 2x₂ = 0 (and ∂δY/∂X₁ = 0 when x₁ = 0), the first-order GUM result understates or even misses the uncertainty; the output PDF is a (scaled) non-central χ² distribution, strongly asymmetric.
3. Because the PDF is asymmetric and bounded below by zero, the shortest 95 % coverage interval is reported.
4. With r(X₁, X₂) = 0.9 the inputs are drawn from a bivariate Gaussian (Cholesky factor). The first-order GUM result is unaffected by the correlation because c₂ = 0; second-order terms are not available for correlated inputs (GUM 5.1.2 note), so only the MCM captures it.

**Result:**

- dY = (0.0 ± 0.0) 10⁻⁶, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 12.7, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x000-correlated.json --lang en`

### Comparison loss, x₁ = 0.01

`jcgm101-9.4-comparison-loss-x010` — JCGM 101:2008, 9.4, Table 8

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The comparison loss in microwave power meter calibration is δY = X₁² + X₂², where X₁ and X₂ are the real and imaginary parts of the voltage reflection coefficient, each with u = 0.005.
2. Because ∂δY/∂X₂ = 2x₂ = 0 (and ∂δY/∂X₁ = 0 when x₁ = 0), the first-order GUM result understates or even misses the uncertainty; the output PDF is a (scaled) non-central χ² distribution, strongly asymmetric.
3. Because the PDF is asymmetric and bounded below by zero, the shortest 95 % coverage interval is reported.
4. The GUM result here includes the second-order terms (higher_order), which for this quadratic model give the exact variance; the 95 % interval based on a symmetric distribution is still inappropriate.

**Result:**

- dY = (100 ± 220) 10⁻⁶, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x010.json --lang en`

### Comparison loss, x₁ = 0.01, r = 0.9

`jcgm101-9.4-comparison-loss-x010-correlated` — JCGM 101:2008, 9.4, Table 9

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The comparison loss in microwave power meter calibration is δY = X₁² + X₂², where X₁ and X₂ are the real and imaginary parts of the voltage reflection coefficient, each with u = 0.005.
2. Because ∂δY/∂X₂ = 2x₂ = 0 (and ∂δY/∂X₁ = 0 when x₁ = 0), the first-order GUM result understates or even misses the uncertainty; the output PDF is a (scaled) non-central χ² distribution, strongly asymmetric.
3. Because the PDF is asymmetric and bounded below by zero, the shortest 95 % coverage interval is reported.
4. With r(X₁, X₂) = 0.9 the inputs are drawn from a bivariate Gaussian (Cholesky factor). The first-order GUM result is unaffected by the correlation because c₂ = 0; second-order terms are not available for correlated inputs (GUM 5.1.2 note), so only the MCM captures it.

**Result:**

- dY = (100 ± 200) 10⁻⁶, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x010-correlated.json --lang en`

### Comparison loss, x₁ = 0.05

`jcgm101-9.4-comparison-loss-x050` — JCGM 101:2008, 9.4, Table 8

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The comparison loss in microwave power meter calibration is δY = X₁² + X₂², where X₁ and X₂ are the real and imaginary parts of the voltage reflection coefficient, each with u = 0.005.
2. Because ∂δY/∂X₂ = 2x₂ = 0 (and ∂δY/∂X₁ = 0 when x₁ = 0), the first-order GUM result understates or even misses the uncertainty; the output PDF is a (scaled) non-central χ² distribution, strongly asymmetric.
3. Because the PDF is asymmetric and bounded below by zero, the shortest 95 % coverage interval is reported.
4. The GUM result here includes the second-order terms (higher_order), which for this quadratic model give the exact variance; the 95 % interval based on a symmetric distribution is still inappropriate.

**Result:**

- dY = (2500 ± 980) 10⁻⁶, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x050.json --lang en`

### Comparison loss, x₁ = 0.05, r = 0.9

`jcgm101-9.4-comparison-loss-x050-correlated` — JCGM 101:2008, 9.4, Table 9

JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The comparison loss in microwave power meter calibration is δY = X₁² + X₂², where X₁ and X₂ are the real and imaginary parts of the voltage reflection coefficient, each with u = 0.005.
2. Because ∂δY/∂X₂ = 2x₂ = 0 (and ∂δY/∂X₁ = 0 when x₁ = 0), the first-order GUM result understates or even misses the uncertainty; the output PDF is a (scaled) non-central χ² distribution, strongly asymmetric.
3. Because the PDF is asymmetric and bounded below by zero, the shortest 95 % coverage interval is reported.
4. With r(X₁, X₂) = 0.9 the inputs are drawn from a bivariate Gaussian (Cholesky factor). The first-order GUM result is unaffected by the correlation because c₂ = 0; second-order terms are not available for correlated inputs (GUM 5.1.2 note), so only the MCM captures it.

**Result:**

- dY = (2500 ± 980) 10⁻⁶, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x050-correlated.json --lang en`

## Flow and volume

### Water meter calibration: relative error of one run

`ea402-s12-water-meter-error` — EA-4/02 M:2022, S12

EA-4/02 S12 (second budget): relative error of indication of the meter for a single run.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. e_X = (V_iX + δV_iX2 − δV_iX1)/V_X − 1: the meter's start and stop readings each carry a resolution term ±0.1 L.
2. V_X comes from the first budget (199.93 L, u = 0.109 L).

**Result:**

- e_X = (0.0003 ± 0.0014), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/flow/ea402-s12-water-meter-error.json --lang en`

### Water meter calibration: mean of three runs

`ea402-s12-water-meter-mean-error` — EA-4/02 M:2022, S12

EA-4/02 S12 (third budget): mean relative error of three runs, with the Welch–Satterthwaite formula and k from the t-distribution.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. e_X,av = ē_X + δe_X: the scatter of the three runs (s = 0.001, u = s/√3, 2 degrees of freedom) is added to the uncertainty of a single run (0.68 × 10⁻³).
2. s = 0.00104, so u = s/√3 = 0.60 × 10⁻³ (EA-4/02 prints s = 0.001); combined u = 0.91 × 10⁻³ with ν_eff = 10, so k = t₀.₉₅₄₅(10) = 2.28 (EA-4/02 Table E.1) and U = 0.002.

**Result:**

- e_Xav = (0.001 ± 0.002), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.28, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/flow/ea402-s12-water-meter-mean-error.json --lang en`

### Water meter calibration: reference volume

`ea402-s12-water-meter-volume` — EA-4/02 M:2022, S12

EA-4/02 S12 (first budget): volume passed through a household water meter, measured in a volumetric standard at 2500 L/h.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. V_X = V_S[1 + β_S(t_S − 20)][1 + γ_W(t_X − t_S)][1 − κ_W p_X], correcting the standard's reading to the temperature and pressure of the water at the meter.
2. The temperature of the water at the meter (±2 °C) gives the largest correction uncertainty: c(t_X) = 0.030 L/K.
3. V_X = 199.93 L with u = 0.109 L.

**Result:**

- V_X = (199.93 ± 0.22) L, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/flow/ea402-s12-water-meter-volume.json --lang en`

## General (method validation)

### Additive model with Gaussian inputs

`jcgm101-9.2.2-additive-gaussian` — JCGM 101:2008, 9.2.2

JCGM 101 9.2.2: four independent inputs, each N(0, 1).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The model Y = X1 + X2 + X3 + X4 is linear with unit sensitivity coefficients, so the GUM law of propagation gives u(y)² = Σ u(xᵢ)² exactly.
2. The output is exactly Gaussian, N(0, 2²); the GUM 95 % interval ±1.96 × 2 = ±3.92 is exact and the Monte Carlo method reproduces it, so the GUM result is validated.

**Result:**

- Y = (0.0 ± 3.9), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/general/jcgm101-9.2.2-additive-gaussian.json --lang en`

### Additive model with rectangular inputs

`jcgm101-9.2.3-additive-rectangular` — JCGM 101:2008, 9.2.3

JCGM 101 9.2.3: four independent inputs, each rectangular with standard deviation 1.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The model Y = X1 + X2 + X3 + X4 is linear with unit sensitivity coefficients, so the GUM law of propagation gives u(y)² = Σ u(xᵢ)² exactly.
2. The output PDF is a convolution of four rectangular PDFs: its 95 % endpoints are ±2√3[2 − (3/5)^{1/4}] = ±3.88 (JCGM 101 Annex E), slightly inside the GUM interval ±3.92.
3. With two significant digits of u(y) = 2.0 the numerical tolerance is δ = 0.05; the exact endpoint difference 3.920 − 3.881 = 0.039 is below δ, so the GUM interval is validated — but only just.

**Result:**

- Y = (0.0 ± 3.9), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- JCGM 101 Table 3 reports the GUM interval validated in one of its two adaptive runs and not in the other (9.2.3.4): with d ≈ 0.039 against δ = 0.05, Monte Carlo noise in the endpoints can decide the outcome.

**Run:** `uncertainty run library/general/jcgm101-9.2.3-additive-rectangular.json --lang en`

### Additive model with a dominant rectangular input

`jcgm101-9.2.4-additive-dominant-rectangular` — JCGM 101:2008, 9.2.4

JCGM 101 9.2.4: as 9.2.3 but X4 has standard deviation 10.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The model Y = X1 + X2 + X3 + X4 is linear with unit sensitivity coefficients, so the GUM law of propagation gives u(y)² = Σ u(xᵢ)² exactly.
2. The output is nearly rectangular: its 95 % interval is about ±17.0, while the GUM interval (normal assumption) is ±19.9 — the central limit theorem does not apply when one input dominates.
3. The GUM result is not validated for n_dig = 2 (δ = 0.5) but would be for n_dig = 1 (δ = 5).

**Result:**

- Y = (0 ± 20), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/general/jcgm101-9.2.4-additive-dominant-rectangular.json --lang en`

## Length

### Calibration of a vernier calliper at 150 mm

`ea402-s10-vernier-calliper` — EA-4/02 M:2022, S10

EA-4/02 S10: error of a vernier calliper measured with a gauge block; two rectangular terms dominate (trapezoidal output).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. E_X = l_iX − l_S + L·ᾱ·Δt + δl_iX + δl_M (µm).
2. The resolution (±25 µm) and the mechanical effects (±50 µm) are rectangular; their convolution is trapezoidal with β = (50 − 25)/(50 + 25) = 0.33, giving k = 1.83 for 95 % (Eq. S10.9) and U ≈ 60 µm.

**Result:**

- E_X = (100 ± 65) µm, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/length/ea402-s10-vernier-calliper.json --lang en`

### Calibration of a 90 mm ring gauge

`ea402-s13-ring-gauge-90mm` — EA-4/02 M:2022, S13

EA-4/02 S13: diameter of a ring gauge compared with a 40 mm setting ring on a one-coordinate measuring machine (µm relative to 90 mm).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. d_X = d_S + Δl + δl_0 + δl_i + δl_T + δl_P + δl_E + δl_A: the measured diameter difference Δl (Type A from 5 observations, 0.15 µm) and the zeroing of the comparator (0.25 µm) dominates together with the temperature term of the sub-budget (0.15 µm).
2. The coaxiality error is a second-order (cosine) effect with a non-zero mean: δl_P = 2(1/D_X − 1/D_S)u²(Δc) = −0.004 µm, u = 0.0065 µm.
3. u(d_X) = 0.41 µm; U = 0.82 µm, which EA-4/02 rounds up to 0.9 µm: d_X = (90.000 3 ± 0.000 9) mm. The project uses one significant digit with rounding up to reproduce this.

**Result:**

- d_X = (0.2 ± 0.9) µm, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Finding (affects the reported value): the budget uses Δl = 49.999 55 mm instead of the mean 49.999 536 mm of the observations and δl_P = +0.004 µm instead of −0.0037 µm, and so obtains d_X = 90.000 25 mm, reported as 90.000 3 mm. From the data, d_X = 90.000 232 mm, i.e. (90.000 2 ± 0.000 9) mm. The difference, 0.1 µm, is well inside U.
- Findings: the mean Δl is 49.999 54 mm but the budget uses 49.999 55; the text quotes s = 0.18 µm for the standard deviation of a single observation (0.33 µm from the data, 0.15 µm for the mean); √(0.15² + 0.25²) = 0.29 µm (printed 0.30); the coaxiality correction is −0.004 µm (budget +0.000 004 mm).

**Run:** `uncertainty run library/length/ea402-s13-ring-gauge-90mm.json --lang en`

### Ring gauge calibration: temperature correction

`ea402-s13-ring-gauge-temperature` — EA-4/02 M:2022, S13

EA-4/02 S13: temperature sub-budget of the ring-gauge comparison; products of zero-mean temperature deviations with expansion coefficients.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. δl_T = [D_S(α_S − α_R) − D_X(α_X − α_R)]Δt_A + D_S α_S δt_S − D_X α_X δt_X + (D_S − D_X) α_R δt_R (µm), with D_S = 40 mm and D_X = 90 mm.
2. All temperature deviations have zero expectation, so the first term has zero first-order sensitivity; the second-order terms give the exact variance of the products: 0.018, 0.053, 0.12, 0.066 µm → u(δl_T) = 0.15 µm.

**Result:**

- dl_T = (0.00 ± 0.30) µm, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/length/ea402-s13-ring-gauge-temperature.json --lang en`

### Calibration of a 50 mm gauge block

`ea402-s4-gauge-block-50mm` — EA-4/02 M:2022, S4

EA-4/02 S4: grade 0 steel gauge block compared with a calibrated standard; a product of two zero-mean quantities needs second-order terms.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. l_X = l_S + δl_D + δl + δl_C − L(ᾱ·δt + δα·θ) − δl_V, in nm relative to 50 mm (L = 50 mm).
2. δα (difference of expansion coefficients) and θ (deviation from 20 °C) both have zero expectation, so their product has zero first-order sensitivity: its contribution L·u(δα)·u(θ) = 11.8 nm appears only through the second-order terms (S4.13), which the project enables.
3. l_X = 49.999 926 mm ± 69 nm (k = 2).

**Result:**

- l_X = (49999926 ± 68) nm, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/length/ea402-s4-gauge-block-50mm.json --lang en`

### End-gauge calibration (GUM H.1)

`gum-h1-end-gauge` — JCGM 100:2008, H.1, Table H.1

JCGM 100 Annex H.1: calibration of a 50 mm end gauge; Welch–Satterthwaite effective degrees of freedom and k for 99 %.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The length of an end gauge is obtained by comparison with a standard of the same nominal length (50 mm): l = l_s + d − l_s(δα θ + α_s δθ), with θ = θ̄ + Δ the temperature deviation from 20 °C.
2. At the estimates δα = 0, δθ = 0, θ = −0.1 °C, the sensitivity coefficients of α_s, θ̄ and Δ vanish: their contributions appear only in second order (GUM H.1.7).
3. Degrees of freedom: l_s 18, d 25.6, δα 50 (10 % reliable), δθ 2 (50 % reliable, GUM G.4.2). The Welch–Satterthwaite formula gives ν_eff = 16.7, truncated to 16, and k₉₉ = t₀.₉₉(16) = 2.92.
4. u_c = 32 nm and U₉₉ = 2.92 × 32 nm = 93 nm; l = (50.000 838 ± 0.000 093) mm.

**Result:**

- l = (50000838 ± 92) nm, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.92, corresponding to a coverage probability of approximately 99 %.

**Notes:**

- The GUM multiplies the rounded u_c = 32 nm (2.92 × 32 = 93 nm); the unrounded value 31.7 nm gives U = 92.5 nm. Including the second-order terms 11.7 and 1.7 nm (H.1.7) raises u_c to 34 nm.
- The Monte Carlo result treats d as Gaussian; JCGM 101 9.5 (projects jcgm101-9.5-*) uses t distributions and curvilinear trapezoids for the inputs instead.

**Run:** `uncertainty run library/length/gum-h1-end-gauge.json --lang en`

### Gauge block calibration, approximate model

`jcgm101-9.5-gauge-block-approximate` — JCGM 101:2008, 9.5, Tables 10 and 11; JCGM 100:2008, H.1

JCGM 101 9.5: the GUM H.1 gauge block with the input PDFs assigned by JCGM 101 (t, arc sine, curvilinear trapezoid); 99 % coverage.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The length of an end gauge is obtained by comparison with a standard of the same nominal length (50 mm): l = l_s + d − l_s(δα θ + α_s δθ), with θ = θ̄ + Δ the temperature deviation from 20 °C.
2. At the estimates δα = 0, δθ = 0, θ = −0.1 °C, the sensitivity coefficients of α_s, θ̄ and Δ vanish: their contributions appear only in second order (GUM H.1.7).
3. Type A inputs become scaled and shifted t distributions (JCGM 101 6.4.9); limits that are themselves inexactly known become curvilinear trapezoids (6.4.3), whose GUM degrees of freedom express the reliability of the limits.
4. The MCM gives u = 36 nm (GUM: 32 nm) because the product terms δα·θ and α_s·δθ are neglected to first order; the 99 % shortest interval is [745, 932] nm relative to 50 mm.

**Result:**

- dL = (838 ± 94) nm, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.92, corresponding to a coverage probability of approximately 99 %.

**Run:** `uncertainty run library/length/jcgm101-9.5-gauge-block-approximate.json --lang en`

### Gauge block calibration, non-linear model

`jcgm101-9.5-gauge-block-nonlinear` — JCGM 101:2008, 9.5, Tables 10 and 11; JCGM 100:2008, H.1

JCGM 101 9.5: the GUM H.1 gauge block with the input PDFs assigned by JCGM 101 (t, arc sine, curvilinear trapezoid); 99 % coverage.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The length of an end gauge is obtained by comparison with a standard of the same nominal length (50 mm): l = l_s + d − l_s(δα θ + α_s δθ), with θ = θ̄ + Δ the temperature deviation from 20 °C.
2. At the estimates δα = 0, δθ = 0, θ = −0.1 °C, the sensitivity coefficients of α_s, θ̄ and Δ vanish: their contributions appear only in second order (GUM H.1.7).
3. Type A inputs become scaled and shifted t distributions (JCGM 101 6.4.9); limits that are themselves inexactly known become curvilinear trapezoids (6.4.3), whose GUM degrees of freedom express the reliability of the limits.
4. The MCM gives u = 36 nm (GUM: 32 nm) because the product terms δα·θ and α_s·δθ are neglected to first order; the 99 % shortest interval is [745, 932] nm relative to 50 mm.

**Result:**

- dL = (838 ± 94) nm, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.92, corresponding to a coverage probability of approximately 99 %.

**Run:** `uncertainty run library/length/jcgm101-9.5-gauge-block-nonlinear.json --lang en`

## Mass

### Calibration of a 10 kg weight

`ea402-s2-weight-10kg` — EA-4/02 M:2022, S2

EA-4/02 S2: OIML class M1 weight calibrated against an F2 standard with a mass comparator (ABBA).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. m_X = m_S + δm_D + δm + δm_C + δB (deviations from 10 kg, in mg).
2. The observed difference is the mean of three ABBA comparisons (+10, +30, +20 mg); its standard uncertainty uses the pooled standard deviation s_p = 25 mg from an earlier evaluation: u(δm) = 25/√3 = 14.4 mg.
3. Drift ±15 mg, comparator ±10 mg and air buoyancy ±10 mg (1 × 10⁻⁶ of the nominal value) are rectangular.
4. m_X = 10 000.025 kg ± 58 mg (k = 2).

**Result:**

- m_X = (25 ± 58) mg, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Finding: the guide combines contributions rounded to three significant figures (14.4 for 14.43 mg), giving 29.2 mg and U = 58 mg; unrounded, u = 29.26 mg and U = 58.5 mg (59 mg when rounded to two digits).

**Run:** `uncertainty run library/mass/ea402-s2-weight-10kg.json --lang en`

### Mass calibration (first-order GUM)

`jcgm101-9.3-mass-calibration` — JCGM 101:2008, 9.3, Tables 6 and 7

JCGM 101 9.3: calibration of a 100 g weight by comparison with a reference weight; first-order GUM vs MCM.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The conventional mass of W follows from weighing against R in air: m_W = (m_R,c + δm_R,c)[1 + (ρ_a − ρ_a0)(1/ρ_W − 1/ρ_R)], reported as the deviation δm = m_W − 100 g (in mg).
2. At the estimates, ρ_a = ρ_a0 = 1.2 kg/m³, so the sensitivity coefficients of the three densities are zero: a first-order evaluation ignores the buoyancy correction entirely (u = 0.0539 mg).
3. The Monte Carlo method propagates the full distributions and gives u = 0.0755 mg: the first-order GUM result is not validated even for one significant digit (n_dig = 1, δ = 0.005 mg).
4. The project 'jcgm101-9.3-mass-calibration-higher-order' repeats the evaluation with the second-order terms of GUM 5.1.2, which are validated.

**Result:**

- dm = (1.23 ± 0.10) mg, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- JCGM 101 Table 6 reports the shortest interval [1.0834, 1.3825] from M = 0.72 × 10⁶; the output PDF is symmetric about 1.2340, so the probabilistically symmetric interval is used here (far less dispersed).

**Run:** `uncertainty run library/mass/jcgm101-9.3-mass-calibration.json --lang en`

### Mass calibration (GUM with second-order terms)

`jcgm101-9.3-mass-calibration-higher-order` — JCGM 101:2008, 9.3, Table 6; JCGM 100:2008, 5.1.2

JCGM 101 9.3: the same model evaluated with the higher-order terms of GUM 5.1.2.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The conventional mass of W follows from weighing against R in air: m_W = (m_R,c + δm_R,c)[1 + (ρ_a − ρ_a0)(1/ρ_W − 1/ρ_R)], reported as the deviation δm = m_W − 100 g (in mg).
2. At the estimates, ρ_a = ρ_a0 = 1.2 kg/m³, so the sensitivity coefficients of the three densities are zero: a first-order evaluation ignores the buoyancy correction entirely (u = 0.0539 mg).
3. The second-order terms ½ΣΣ(∂²f/∂xᵢ∂xⱼ)²u²(xᵢ)u²(xⱼ) + ... capture the products (ρ_a − 1.2)(1/ρ_W − 1/ρ_R) and give u = 0.0750 mg, in agreement with the Monte Carlo result: validated for n_dig = 1.
4. GUM 5.1.2 note derives these terms for Gaussian inputs; the program warns because the densities are rectangular. The Monte Carlo method has no such restriction.

**Result:**

- dm = (1.23 ± 0.15) mg, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/mass/jcgm101-9.3-mass-calibration-higher-order.json --lang en`

## Temperature

### Calibration of a temperature block calibrator at 180 °C

`ea402-s11-block-calibrator-180c` — EA-4/02 M:2022, S11

EA-4/02 S11: temperature of the block measured with a calibrated platinum resistance thermometer (values in mK).

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. t_X = t_S + δt_S + δt_D − δt_iX + δt_R + δt_A + δt_H + δt_V (mK).
2. Axial inhomogeneity (±250 mK) and radial differences (±100 mK) dominate: trapezoid with β = 0.43; EA-4/02 quotes k = 1.81 (Eq. S10.9 gives 1.80); t_X = (180.1 ± 0.3) °C.

**Result:**

- t_X = (180100 ± 330) mK, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Finding: the guide quotes k = 1.81 for β = 0.43; Eq. (S10.9) gives 1.80. The reported (180.1 ± 0.3) °C is unchanged.

**Run:** `uncertainty run library/temperature/ea402-s11-block-calibrator-180c.json --lang en`

### Thermocouple calibration: emf of the unknown

`ea402-s5-thermocouple-emf` — EA-4/02 M:2022, S5

EA-4/02 S5 (second budget): emf of the type N thermocouple under calibration at 1000 °C.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. V_X = V_iX + δV_iX1 + δV_iX2 + δV_R + δV_LX + Δt/C_X − δt_0X/C_X0, where Δt = t − t_X is the difference between the calibration point (1000 °C) and the furnace temperature of the first budget, with u = 0.641 °C.
2. The furnace temperature dominates (24.65 µV of 25.0 µV); V_X = (36 229 ± 50) µV (k = 2).

**Result:**

- V_X = (36229 ± 50) µV, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Finding: the printed contribution of δt (24.5 µV) is inconsistent with 0.641 °C × 38.46 µV/°C = 24.65 µV; the printed total 25.0 µV follows from 24.65. The estimate of Δt is −0.5 °C (printed 0.5 °C), giving V_X = 36 248 − 19.2 = 36 229 µV as stated.

**Run:** `uncertainty run library/temperature/ea402-s5-thermocouple-emf.json --lang en`

### Thermocouple calibration: furnace temperature

`ea402-s5-thermocouple-furnace` — EA-4/02 M:2022, S5

EA-4/02 S5 (first budget): temperature of a furnace at 1000 °C measured with a type N reference thermocouple.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. t_X = t_S(V_iS + δV_iS1 + δV_iS2 + δV_R − δt_0S/C_S0) + δt_S + δt_D + δt_F, linearised with the sensitivity C_S = 0.077 °C/µV of the reference thermocouple and C_S0 = 0.189 °C/µV at the reference junction.
2. The non-uniformity of the furnace temperature (±1 °C, rectangular) dominates: u(t_X) = 0.641 °C.

**Result:**

- t_X = (1000.5 ± 1.3) °C, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Run:** `uncertainty run library/temperature/ea402-s5-thermocouple-furnace.json --lang en`

