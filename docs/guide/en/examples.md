# Worked examples

This chapter is generated from the example library by `uncertainty catalogue library --lang en`; every result is recomputed by the program from the inputs stored in the project file, and the test suite fails if this file is out of date. Monte Carlo results are not listed here: run a project to obtain them together with the full budget, charts and the comparison with the published reference values.

- Chemistry (41)
- Electrical and RF (15)
- Flow and volume (4)
- General (method validation) (8)
- Length (8)
- Mass (4)
- Physical properties (6)
- Temperature (4)

## Chemistry

### Arsenic in chocolate: homogeneity and stability of PT items (ISO 13528 E.2)

`iso13528-e2-arsenic-homogeneity` — ISO 13528:2022, E.2, Tables E.2 and E.3

ISO 13528:2022 E.2: between-sample standard deviation from 10 items measured in duplicate, compared with 0.3σ_pt, and a stability check after six weeks at 60 °C.

**Approach:** statistics for proficiency testing (ISO 13528)

**Steps:**

1. Ten vials, selected by stratified random sampling from 1 000, are each tested twice under repeatability conditions (Table E.2).
2. Analysis of variance (B.3): general average 0.18715 mg/kg, standard deviation of the item averages s_x = 0.00398, within-sample s_w = 0.00556 and between-sample s_s = √(s_x² − s_w²/2) = 0.00060 mg/kg.
3. With the provisional σ_pt = 15 % of the average = 0.02807 mg/kg, s_s is far below 0.3σ_pt = 0.00842 mg/kg: the items are sufficiently homogeneous. The expanded criterion of B.2.3 (F₁ = 1.88, F₂ = 1.01 for g = 10) gives √c = 0.0133 mg/kg.
4. Stability: two items stored at 60 °C for six weeks give an average of 0.19375 mg/kg; the difference 0.00660 mg/kg from the homogeneity mean is below 0.3σ_pt, so stability is sufficient (B.5.1).

**Run:** `uncertainty run library/chemistry/iso13528-e2-arsenic-homogeneity.json --lang en`

### Atrazine in drinking water: robust average by Algorithm A (ISO 13528 E.3)

`iso13528-e3-atrazine-algorithm-a` — ISO 13528:2022, E.3, Tables E.4 and E.5

ISO 13528:2022 E.3: assigned value and standard deviation for proficiency assessment from 34 participant results with Algorithm A; z scores.

**Approach:** statistics for proficiency testing (ISO 13528)

**Steps:**

1. 34 participants reported atrazine in drinking water (Table E.4, ordered by value). Two very low results (0.040 and 0.055 mg/L) and one high result (0.4246 mg/L) lie far from the majority.
2. Algorithm A (C.3.1) starts from x* = median = 0.2620 and s* = MADe = 0.0386 mg/L, replaces every result outside x* ± 1.5s* by the nearer limit and recomputes x* as the mean and s* as 1.134 × the standard deviation of the modified values.
3. After six iterations the third significant figures no longer change: x* = 0.2570 mg/L, s* = 0.0395 mg/L. The arithmetic mean 0.2512 and standard deviation 0.0672 mg/L are pulled by the three outlying results.
4. The assigned value is x_pt = x* with u(x_pt) = 1.25 s*/√p = 1.25 × 0.0395/√34 = 0.0085 mg/L (Formula 6) and σ_pt = s*. Since u(x_pt) < 0.3σ_pt = 0.0119 mg/L the uncertainty of the assigned value is negligible (9.2.1) and z scores are used; laboratories 1, 2 and 34 receive action signals.

**Result:**

- x_pt = 0.257 mg/L, U(x_pt) = 0.017 mg/L (k = 2.00)

**Notes:**

- The stopping rule of C.3.1 (no change in the third significant figures) is used; iterating to full convergence gives x* = 0.257011 and s* = 0.039520 mg/L, the same to the printed digits. The nIQR of Table E.5 (0.0402) corresponds to quartiles by linear interpolation between order statistics; other quartile definitions give 0.0379 to 0.0423 (Table E.5, note).

**Run:** `uncertainty run library/chemistry/iso13528-e3-atrazine-algorithm-a.json --lang en`

### Atrazine in drinking water: median and nIQR (ISO 13528 E.3)

`iso13528-e3-atrazine-median` — ISO 13528:2022, E.3, Table E.5

ISO 13528:2022 E.3, first row of Table E.5: the median as assigned value with u(x_pt) = 1.25 nIQR/√p and the nIQR as σ_pt.

**Approach:** statistics for proficiency testing (ISO 13528)

**Steps:**

1. 34 participants reported atrazine in drinking water (Table E.4, ordered by value). Two very low results (0.040 and 0.055 mg/L) and one high result (0.4246 mg/L) lie far from the majority.
2. The simple robust estimators of C.2: median 0.2620 mg/L, MADe = 1.483 × median absolute deviation = 0.0386 mg/L and nIQR = 0.7413 (Q₃ − Q₁) = 0.0402 mg/L.
3. With the median as assigned value, u(x_pt) = 1.25 × 0.0402/√34 = 0.0086 mg/L, practically the same as with Algorithm A (0.0085 mg/L); the nIQR has a breakdown point of 25 %, the MADe of 50 % (C.2.3, Note 1).

**Result:**

- x_pt = 0.262 mg/L, U(x_pt) = 0.017 mg/L (k = 2.00)

**Run:** `uncertainty run library/chemistry/iso13528-e3-atrazine-median.json --lang en`

### Mercury in animal feed: z, z′, ζ and E_n scores (ISO 13528 E.4)

`iso13528-e4-mercury-feed` — ISO 13528:2022, E.4, Tables E.6 and E.7

ISO 13528:2022 E.4 (IMEP 111): scores of 21 laboratories against a reference value with a fitness-for-purpose σ_pt, using the uncertainties the laboratories reported.

**Approach:** statistics for proficiency testing (ISO 13528)

**Steps:**

1. x_pt = 0.044 mg/kg is a reference value with U(x_pt) = 0.0082 mg/kg (k = 2, characterisation and inhomogeneity); σ_pt = 0.0066 mg/kg (15 %) is a fitness-for-purpose value and δ_E = 3σ_pt for the PA scores. Three results reported as 'less than' are not scored.
2. The laboratories' standard uncertainties are their expanded uncertainties divided by their coverage factors (L23: k = 1.732); unrounded values are used.
3. z = D/σ_pt, z′ = D/√(σ_pt² + u²(x_pt)), ζ = D/√(u²(x) + u²(x_pt)) and E_n = D/√(U²(x) + U²(x_pt)). Here u(x_pt) = 0.0041 > 0.3σ_pt = 0.0020, so the uncertainty of the assigned value is not negligible and z′ is about 15 % smaller than z.
4. The results are bimodal by method: the eight laboratories using a direct mercury analyser (AMA) and one using AAS obtain 30 % to 54 % of the reference value and receive action signals (|z| ≥ 3), whereas all laboratories using CV-AAS, HG-AAS or ICP-MS are acceptable.

**Result:**

- x_pt = 0.0440 mg/kg, U(x_pt) = 0.0082 mg/kg (k = 2.00)

**Notes:**

- Table E.6 flags the reported uncertainties a/b/c 'following criteria discussed in 9.8', but the flags are not reproduced with u_min = u(x_pt) = 0.0041 mg/kg (9.8.3): L05 (u = 0.0035) and L09 (u = 0.0040) are flagged 'a'. The flags imply a lower limit between 0.0018 and 0.0035 mg/kg (possibly the characterisation uncertainty without inhomogeneity) and an upper limit between 0.005 and 0.015 mg/kg (consistent with 1.5σ_pt = 0.0099). The flags are therefore not checked here.

**Run:** `uncertainty run library/chemistry/iso13528-e4-mercury-feed.json --lang en`

### Meat content from nitrogen and fat (ISO 21748 C.2)

`iso21748-c2-meat-content` — ISO 21748:2017, Annex C.2, Table C.1

ISO 21748:2017 C.2: a combined result w_meat = 100 w_N/f_N + w_fat whose inputs each carry reproducibility-based uncertainties (Clause 12), written as a measurement model with relative error terms.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. w_meat = w_pro + w_fat with w_pro = 100 w_N/f_N (Formulae C.1, C.2); w_N = 3.29 % (Kjeldahl, mean of duplicates), f_N = 3.65 ± 0.052, w_fat = 5.50 %.
2. The nitrogen result carries the between-laboratory term s_L = 1.1 % and the repeatability s_r = 1.8 % of a single result divided by √2 for the duplicate (ISO 21748 Table 1): e_L ~ N(0, 0.011²), e_r ~ N(0, (0.018/√2)²), so u(w_N)/w_N = 1.7 % (Formula C.3). Fat carries s_R = 2.0 %.
3. The law of propagation gives w_meat = 95.6 % with u = 1.99 % and U = 4.0 % (k = 2); w_pro contributes 99.7 % of the variance.

**Result:**

- w_meat = (95.6 ± 4.0) %, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

**Notes:**

- Table C.1 rounds the relative uncertainties (0.017, 0.014, 0.022) before combining and prints u = 1.98 %; with the unrounded terms u = 1.99 %. Both give U = 4.0 %.

**Run:** `uncertainty run library/chemistry/iso21748-c2-meat-content.json --lang en`

### Molecular weight of carbon dioxide (NIST TN 1900 E8)

`nist-tn1900-e8-molecular-weight-co2` — NIST Technical Note 1900, Example E8, Exhibit 12

TN 1900 E8: M_r(CO₂) = A_r(C) + 2A_r(O) from the IUPAC interval atomic weights; the output distribution is trapezoidal and the GUM interval is too wide.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The standard atomic weights are intervals: A_r(C) ∈ [12.0096, 12.0116] and A_r(O) ∈ [15.99903, 15.99977] (Wieser et al. 2013). Each is modelled as a rectangular distribution over its interval.
2. The estimate is 12.0106 + 2 × 15.9994 = 44.0094 and u² = u(A_r(C))² + 4u(A_r(O))², u = 0.00072.
3. The sum of two rectangular variables has a symmetric trapezoidal distribution with half-widths 0.00174 (base) and 0.00026 (top). Its shortest 95 % interval is 44.0094 ± 0.00136 = [44.0080, 44.0108], narrower than the GUM interval ± 1.96u = ± 0.00141; the Monte Carlo method reproduces the exact interval.

**Result:**

- M_r = (44.0094 ± 0.0014), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- TN 1900 prints u = 0.000721 from the rounded components 0.0006 and 0.0002; the unrounded components 0.000577 and 0.000214 give 0.000718. The exact interval end points follow from the trapezoid: 44.0094 ± (0.00174 − √(0.2 × 0.001 × 0.00074)) = [44.008045, 44.010755].

**Run:** `uncertainty run library/chemistry/nist-tn1900-e8-molecular-weight-co2.json --lang en`

### Cadmium calibration standard, Monte Carlo (NIST TN 1900 E9)

`nist-tn1900-e9-cadmium-standard` — NIST Technical Note 1900, Example E9

TN 1900 E9: the QUAM A1 cadmium standard c = 1000 m P / V re-evaluated with the Monte Carlo method, with a rectangular purity and a triangular volume.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. m = 100.28 mg (u = 0.05 mg, Gaussian), P = 0.9999 (rectangular on [0.9998, 1], u = 0.000058) and V = 100.0 mL (triangular, u = 0.07 mL); c = 1002.7 mg/L.
2. For a product and quotient the relative uncertainties add in quadrature (GUM 5.1.6): u(c)/c = √((0.05/100.28)² + (0.000058/0.9999)² + (0.07/100)²), u = 0.86 mg/L.
3. The Monte Carlo method gives the same standard uncertainty and, to the digits printed in TN 1900, the same 95 % interval [1001.0, 1004.4] mg/L as with all inputs Gaussian. The triangular and rectangular inputs make the output slightly less spread in the tails: the Monte Carlo end points lie about 0.02–0.03 mg/L inside the GUM interval, so the GUM result is not validated for two significant digits of u (δ = 0.005 mg/L), although the difference has no practical importance.

**Result:**

- c_Cd = (1002.7 ± 1.7) mg/L, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- TN 1900 describes V as a symmetric triangular distribution 'with mean 100 mL and standard deviation 0.7 mL'; only 0.07 mL, the value used in the same example and in QUAM A1, is consistent with the printed interval. The triangular half-width is 0.07 × √6 = 0.1715 mL.
- The project 'quam-a1-cadmium-standard' builds the uncertainties of m, P and V from their components as in QUAM A1.

**Run:** `uncertainty run library/chemistry/nist-tn1900-e9-cadmium-standard.json --lang en`

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
- The Kragten spreadsheet view (QUAM Appendix E.2), enabled in this project, reproduces the guide's printed spreadsheet value 0.8633 mg/L to all printed digits; the exact first-order value differs only through the finite-difference approximation.

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

**Notes:**

- The Kragten spreadsheet view (QUAM Appendix E.2), enabled in this project, reproduces the guide's printed spreadsheet value 0.000099 mol/L to all printed digits; the exact first-order value differs only through the finite-difference approximation.

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
- The Kragten spreadsheet view (QUAM Appendix E.2), enabled in this project, reproduces the guide's printed spreadsheet value √3.34 × 10⁻⁸ mol/L to all printed digits; the exact first-order value differs only through the finite-difference approximation.

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
- The Kragten spreadsheet view (QUAM Appendix E.2), enabled in this project, reproduces the guide's printed spreadsheet value 0.377 to all printed digits; the exact first-order value differs only through the finite-difference approximation.

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
- The Kragten spreadsheet view (QUAM Appendix E.2), enabled in this project, reproduces the guide's printed spreadsheet value 0.001465 mg/dm² to all printed digits; the exact first-order value differs only through the finite-difference approximation.

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

### Crude fibre in animal feed from reproducibility data (ISO 21748 C.4)

`iso21748-c4-crude-fibre` — ISO 21748:2017, Annex C.4, Tables C.6–C.9

ISO 21748:2017 C.4: level-dependent reproducibility from a collaborative study plus the uncertainty of drying to constant mass; the same example as QUAM A6 with a different treatment of the drying term.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Collaborative study (Table C.6): s_R = 0.293 % at 2.3 %, 0.390 % at 5.4 % and 0.575 % at 10.1 % fibre. Bias was checked on a CRM (9.3 %, laboratory 9.16 %, Δ_l = −0.14 %) and the laboratory's repeatability agrees with the study (Table C.7).
2. Drying to constant mass is not covered by the study: the fixed-time drying stays within 0.002 g of extended drying, a rectangular half-width giving 0.002/√3 = 0.00115 g, i.e. 0.115 % fibre for a 1 g sample.
3. u = √(s_R² + 0.115²) = 0.31, 0.41 and 0.59 % in the three ranges (Table C.9), U = 0.6, 0.8 and 1.2 % (k = 2).

**Result:**

- w(crude fibre) = (2.30 ± 0.63) % m/m (k = 2.00)
- w(crude fibre) = (4.00 ± 0.81) % m/m (k = 2.00)
- w(crude fibre) = (8.0 ± 1.2) % m/m (k = 2.00)

**Notes:**

- ISO 21748 C.4.7 says that the uncertainty of 0.115 % was obtained by 'dividing this maximum estimated deviation by 3'; 0.002 g/3 would give 0.067 %. The printed value corresponds to division by √3 (rectangular distribution), which the project uses.
- QUAM A6 (project 'quam-a6-crude-fibre') uses the rounded s_R 0.29, 0.40, 0.60 and includes the drying term only at the lowest level.

**Run:** `uncertainty run library/chemistry/iso21748-c4-crude-fibre.json --lang en`

### Carbon in steel: [time + operator]-different intermediate precision (ISO 5725-3 J.1)

`iso5725-3-j1-carbon-steel` — ISO 5725-3:2023, Annex J.1, Table J.1

ISO 5725-3:2023 J.1: one routine sample a day re-analysed the next day by another analyst, 29 pairs in a month; s_I(TO) from the pairs, Formula (5).

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Each pair (y_j1, y_j2) is the same sample measured on two days by two analysts, so the differences contain the time and operator effects as well as repeatability (6.2.2).
2. Pairs 20 (0.042/0.146 %) and 24 (0.100/0.161 %) are removed: their differences, probably recording errors, are outliers by Cochran's test (C = 0.72 for pair 20 among all 29, C = 0.89 for pair 24 after removing 20).
3. s_I(TO) = √(Σw_j²/(2 × 27)) = 2.87 × 10⁻³ % (mass fraction), Formula (5), with 27 degrees of freedom.

**Result:**

- w(C) = (0.10000 ± 0.00574) % m/m (k = 2.00)

**Notes:**

- The 27 retained pairs are stored; the two removed pairs are given in the explanation. The Cochran statistics quoted are computed by the program's authors from Table J.1 (ISO 5725-3 gives only the conclusion).

**Run:** `uncertainty run library/chemistry/iso5725-3-j1-carbon-steel.json --lang en`

### Vanadium in steel: staggered-nested design, level 1 (ISO 5725-3 J.2)

`iso5725-3-j2-vanadium-steel` — ISO 5725-3:2023, Annex J.2, Tables J.2–J.5

ISO 5725-3:2023 J.2: 20 laboratories, two results on one day and one on the next; analysis of variance gives s_r, the time-different intermediate precision s_I(T) and s_R.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Each laboratory reports y₁, y₂ (repeatability conditions, day 1) and y₃ (day 2). Laboratory 20 is an outlier at level 1 (Cochran) and is removed.
2. Annex C.1: w(1) = |y₁ − y₂|, w(2) = |(y₁ + y₂)/2 − y₃|; SS₀ = 3Σ(ȳᵢ − ȳ)², SS₁ = (2/3)Σw(2)², SS_e = (1/2)Σw(1)² with 18, 19 and 19 degrees of freedom (Table J.4).
3. From the expected mean squares, s_r² = MS_e = 0.145, s₍₁₎² = (3/4)(MS₁ − MS_e) = 0.218 and s₀² = (MS₀ − MS_e − (5/3)s₍₁₎²)/3 = 0.278 (× 10⁻⁶): s_r = 0.381, s_I(T) = 0.603 and s_R = 0.801 (× 10⁻³ %).

**Result:**

- w(V) = (0.00980 ± 0.00121) % m/m (k = 2.00)

**Notes:**

- Levels 1 and 2 of Table J.5 are reproduced exactly from Table J.2; levels 3, 4 and 5, and s_R at level 6, are not reproduced from the printed data with the stated outlier exclusions (research log F40). The column 'Average (%)' of Table J.5 carries a spurious factor 10⁻³.

**Run:** `uncertainty run library/chemistry/iso5725-3-j2-vanadium-steel.json --lang en`

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

### Impedance magnitude from correlated V and I (GUM H.2)

`gum-h2-impedance` — JCGM 100:2008, H.2, Tables H.2 and H.3

GUM H.2: the impedance magnitude Z = V/I from correlated simultaneous measurements of voltage, current and phase angle.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. Five simultaneous sets of readings of the voltage V across an element, the current I through it and the phase angle φ give the means of GUM Table H.2 with s(x̄) = 0.0032 V, 0.0095 mA and 0.00075 rad, and the correlation coefficients r(V, I) = −0.36, r(V, φ) = 0.86, r(I, φ) = −0.65 (GUM 5.2.3). The project uses these values unrounded, recomputed from the five sets of Table H.2 (s(x̄) = 0.003209 V, 0.009471 mA, 0.000752 rad; r = −0.3553, 0.8576, −0.6451).
2. Approach 1 of GUM H.2.3 evaluates the model at the means and propagates with Equation (16), which includes the covariance terms 2c_ic_ju(x_i)u(x_j)r(x_i, x_j).
3. For Z = V/I, GUM Equation (H.8c) gives u_r²(Z) = u_r²(V) + u_r²(I) − 2u_r(V)u_r(I)r(V, I); with r(V, I) = −0.36 the last term adds to the variance, and u_c(Z) = 0.236 Ω.

**Result:**

- Z = (254.26 ± 0.46) Ω, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/electrical/gum-h2-impedance.json --lang en`

### Reactance from correlated V, I and φ (GUM H.2)

`gum-h2-reactance` — JCGM 100:2008, H.2, Tables H.2 and H.3

GUM H.2: the reactance X = (V/I) sin φ from correlated simultaneous measurements of voltage, current and phase angle.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. Five simultaneous sets of readings of the voltage V across an element, the current I through it and the phase angle φ give the means of GUM Table H.2 with s(x̄) = 0.0032 V, 0.0095 mA and 0.00075 rad, and the correlation coefficients r(V, I) = −0.36, r(V, φ) = 0.86, r(I, φ) = −0.65 (GUM 5.2.3). The project uses these values unrounded, recomputed from the five sets of Table H.2 (s(x̄) = 0.003209 V, 0.009471 mA, 0.000752 rad; r = −0.3553, 0.8576, −0.6451).
2. Approach 1 of GUM H.2.3 evaluates the model at the means and propagates with Equation (16), which includes the covariance terms 2c_ic_ju(x_i)u(x_j)r(x_i, x_j).
3. For X = (V/I) sin φ the correlation of V and φ increases the variance; u_c(X) = 0.295 Ω (relative 0.13 %).

**Result:**

- X = (219.85 ± 0.58) Ω, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- Approach 1 with the unrounded data gives u_c(X) = 0.29558 Ω, approach 2 0.29549 Ω; GUM Table H.3 prints 0.295, so the tolerance is 0.0006 Ω.

**Run:** `uncertainty run library/electrical/gum-h2-reactance.json --lang en`

### Resistance from correlated V, I and φ (GUM H.2)

`gum-h2-resistance` — JCGM 100:2008, H.2, Tables H.2 and H.3

GUM H.2: the resistance R = (V/I) cos φ from correlated simultaneous measurements of voltage, current and phase angle.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. Five simultaneous sets of readings of the voltage V across an element, the current I through it and the phase angle φ give the means of GUM Table H.2 with s(x̄) = 0.0032 V, 0.0095 mA and 0.00075 rad, and the correlation coefficients r(V, I) = −0.36, r(V, φ) = 0.86, r(I, φ) = −0.65 (GUM 5.2.3). The project uses these values unrounded, recomputed from the five sets of Table H.2 (s(x̄) = 0.003209 V, 0.009471 mA, 0.000752 rad; r = −0.3553, 0.8576, −0.6451).
2. Approach 1 of GUM H.2.3 evaluates the model at the means and propagates with Equation (16), which includes the covariance terms 2c_ic_ju(x_i)u(x_j)r(x_i, x_j).
3. For R = (V/I) cos φ the covariance terms reduce the variance: the sum of the squared contributions alone is about 3.6 times u_c²(R), and u_c(R) = 0.071 Ω. Approach 2 (R computed from each set, GUM Table H.4) gives the same value.

**Result:**

- R = (127.73 ± 0.14) Ω, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- Because the covariance terms cancel most of the variance of R, u_c(R) is sensitive to the rounding of the correlation coefficients: with the two-digit values printed in Table H.2 the law of propagation gives 0.0700 Ω instead of 0.0711 Ω (project 'num-10-resistance', which reproduces the NIST Uncertainty Machine). X and Z change by less than 0.1 %.

**Run:** `uncertainty run library/electrical/gum-h2-resistance.json --lang en`

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

### Microwave step attenuator, U-shaped inputs (NIST TN 1900 E11)

`nist-tn1900-e11-step-attenuator` — NIST Technical Note 1900, Example E11, Exhibits 16 and 17

TN 1900 E11: EA-4/02 S7 re-evaluated with the Monte Carlo method; the dominant arcsine mismatch term makes the distribution of a linear sum bimodal.

**Approach:** Monte Carlo method (JCGM 101) only; run the project for its results

**Steps:**

1. L_X = L_S + δL_S + δL_D + δL_M + δL_K + δL_ib − δL_ia + δL_0b − δL_0a, the 30 dB step of the attenuator measured against a calibrated reference attenuator (EA-4/02 S7).
2. L_S is the mean of four readings, 30.0402 dB, u = 0.0091 dB, modelled by TN 1900 as a t₃ variable rescaled to standard deviation 0.0091 dB. Mismatch (u = 0.02 dB), drift and leakage (±0.003 dB) are arcsine (U-shaped); δL_S and the reference detector resolutions are rectangular; the null detector terms are Gaussian (TN 1900 Exhibit 16).
3. The Monte Carlo method gives 30.043 dB, u = 0.0224 dB and the 95 % interval [30.006, 30.081] dB. u²(δL_M) is 79 % of u²(L_X), so the output density has two modes: the conventional interval ± 2u is conservative (99 % effective coverage).

**Notes:**

- TN 1900 rescales the t₃ distribution of L_S to standard deviation u = 0.0091 dB (scale 0.0091/√3 dB). JCGM 101 6.4.9 instead uses the scale s/√n = 0.0091 dB, i.e. a standard deviation of 0.0158 dB. Because the GUM uncertainty of a t input is its scale, the GUM evaluation is switched off; the EA-4/02 GUM result is in 'ea402-s7-step-attenuator-30db'.
- TN 1900 models the leakage δL_K as arcsine on ±0.003 dB (u = 0.0021 dB) and remarks that the 0.0017 dB of EA-4/02 S7.12 corresponds to a rectangular distribution, although EA-4/02 calls the term U-shaped.

**Run:** `uncertainty run library/electrical/nist-tn1900-e11-step-attenuator.json --lang en`

### Resistance with the rounded values of GUM Table H.2 (NIST Uncertainty Machine §10)

`num-10-resistance` — JCGM 100:2008, H.2, Tables H.2 and H.3; T. Lafarge, §10, Figure 6

GUM H.2: the resistance R = (V/I) cos φ from correlated simultaneous measurements of voltage, current and phase angle.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. Five simultaneous sets of readings of V, I and φ give the means of GUM Table H.2 with s(x̄) = 0.0032 V, 0.0095 mA and 0.00075 rad and r(V, I) = −0.36, r(V, φ) = 0.86, r(I, φ) = −0.65, entered here as printed.
2. This project enters the rounded values printed in GUM Table H.2, as the NIST Uncertainty Machine manual does. The strong cancellation of the covariance terms makes u_c(R) = 0.0700 Ω, against 0.071 Ω from the unrounded data (project 'gum-h2-resistance'); the Monte Carlo method with a trivariate Gaussian distribution agrees with the law of propagation.

**Result:**

- R = (127.73 ± 0.14) Ω, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/electrical/num-10-resistance.json --lang en`

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

### Airspeed with a Pitot tube (NIST TN 1900 E4)

`nist-tn1900-e4-pitot-tube` — NIST Technical Note 1900, Example E4

TN 1900 E4 (after Kline and McClintock, 1953): airspeed from the differential pressure, the air pressure and the temperature.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. v = √(2ΔR_sT/p), where the air density ρ = p/(R_sT) comes from the ideal gas law; R_s = 287.058 J kg⁻¹ K⁻¹ is the specific gas constant of dry air.
2. Δ = 1.993 kPa (U-tube manometer), p = 101.4 kPa (Bourdon gauge) and T = 292.8 K (mercury-in-glass thermometer) with 95 % expanded uncertainties 0.025 kPa, 2.1 kPa and 0.11 K; the standard uncertainties are half of these.
3. For a power law the relative uncertainties add in quadrature with weights ½: u(v)/v = ½ √((u_Δ/Δ)² + (u_p/p)² + (u_T/T)²) = 0.61 %, dominated by the air pressure. The Monte Carlo interval is slightly shifted towards lower values (v ∝ p^−½), and the GUM interval is not validated for two significant digits of u (end-point differences 0.007 and 0.009 m/s against δ = 0.005 m/s), as the second-digit differences between the two intervals printed in TN 1900 also show.

**Result:**

- v = (57.48 ± 0.68) m/s, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- With the stated equation and inputs, v = 57.48 m/s and u(v) = 0.348 m/s. TN 1900 prints v = 40.64 m/s and u(v) = 0.25 m/s, which are those values divided by √2, i.e. the result of v = √(ΔR_sT/p) without the factor 2. The relative uncertainty, 0.61 %, is the same in both.

**Run:** `uncertainty run library/flow/nist-tn1900-e4-pitot-tube.json --lang en`

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

### Product and ratio y = ab/c with a wide triangular divisor (NIST Uncertainty Machine §3)

`num-3-ratio-abc` — T. Lafarge, §3, Figure 1

NUM manual §3: Gaussian a, rectangular b and a triangular divisor c with a 30 % relative uncertainty; the first-order GUM result is far from the Monte Carlo result.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. a ~ N(32, 0.5²); b rectangular with mean 0.9 and standard deviation 0.025 (half-width 0.025√3); c symmetric triangular with mean 1 and standard deviation 0.3 (half-width 0.3√6 = 0.735).
2. Gauss's formula at the estimates: y = 32 × 0.9 / 1 = 28.8 and u(y)/y = √((0.5/32)² + (0.025/0.9)² + 0.3²), u(y) = 8.7; c contributes 99 % of the variance.
3. Because 1/c is strongly convex over [0.27, 1.73], the Monte Carlo mean is 32.2, the standard deviation 13 and the 95 % interval (18.2, 67): the distribution of y is markedly right-skewed and the GUM interval 28.8 ± 17.1 is not validated.

**Result:**

- y = (29 ± 17), where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- The text of §3 says that the GUM method gives y = 32.2 and u(y) = 12.5, but the output of the NIST Uncertainty Machine printed in Figure 1 gives y = 28.8 and u(y) = 8.7 for Gauss's formula (32.2 and 13 are the Monte Carlo mean and standard deviation). The program reproduces the values of Figure 1.

**Run:** `uncertainty run library/general/num-3-ratio-abc.json --lang en`

### Carbon monoxide in vehicle emissions from reproducibility data (ISO 21748 C.1)

`iso21748-c1-co-emissions` — ISO 21748:2017, Annex C.1

ISO 21748:2017 C.1: type-approval test of CO emissions; with bias and precision under control the reproducibility standard deviation of the collaborative study is the standard uncertainty.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. An interlaboratory study according to ISO 5725-2 gave s_r = 0.22 g/km and s_R = 0.28 g/km, so s_L = √(s_R² − s_r²) = 0.17 g/km.
2. Trueness is controlled by calibration of the gas analyser and the roller bench, whose uncertainties are negligible against s_R; the laboratory's repeatability (0.20 g/km) is within that of the study (ISO 21748 7.2, 7.3).
3. No operation is outside the collaborative study, so u = s_R = 0.28 g/km (Formula 3) and U = 0.56 g/km (k = 2), evaluated near the limit 2.2 g/km.

**Result:**

- CO = (2.20 ± 0.56) g/km (k = 2.00)

**Notes:**

- The laboratory's repeatability is not substituted for s_r here (as in ISO 21748 C.1); with lab_repeatability = 0.20 g/km, 7.3.2 would give s′_R = √(s_L² + s_l²) = 0.26 g/km.

**Run:** `uncertainty run library/general/iso21748-c1-co-emissions.json --lang en`

### Aerobic plate count, flour: adjusted reproducibility (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-flour` — ISO 21748:2017, Annex C.3, Tables C.2–C.4

ISO 21748:2017 C.3 (AOAC 990.12, flour): relative uncertainty of log10(count) from collaborative-study s_R and s_r, the laboratory's better repeatability (7.3.2) and a sample-preparation term.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Collaborative study (12 laboratories, ISO 5725-2): s_R = 5.8 %, s_r = 5.3 % relative to log10(count), so s_L = √(s_R² − s_r²) = 2.4 %.
2. The laboratory's repeatability from 10 replicates is s_l = 5.0 %; ISO 21748 7.3.2 allows it in place of s_r: s′_R = √(s_L² + s_l²) = 5.5 %.
3. Sub-sampling and weighing, not covered by the study, add 3.0 % (expert estimate); u = √(s′_R² + 3.0²) = 6.3 % and U = 12.6 % (k = 2).

**Result:**

- U = 12 % (k = 2.00), relative to the measured value

**Notes:**

- ISO 21748 Table C.4 prints u = 6.4 % and C.3.7 U = 12.8 % for flour, but its own inputs give √(2.4² + 5.0² + 3.0²) = 6.30 % (unrounded s_L: 6.29 %) and U = 12.6 %. The intervals of Table C.5 for flour follow from the printed 12.8 %. The project checks the recomputed values.

**Run:** `uncertainty run library/general/iso21748-c3-aerobic-plate-count-flour.json --lang en`

### Aerobic plate count, shrimp: adjusted reproducibility (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-shrimp` — ISO 21748:2017, Annex C.3, Tables C.2–C.4

ISO 21748:2017 C.3 (AOAC 990.12, shrimp): relative uncertainty of log10(count) from collaborative-study s_R and s_r, the laboratory's better repeatability (7.3.2) and a sample-preparation term.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Collaborative study (12 laboratories, ISO 5725-2): s_R = 11.1 %, s_r = 9.8 % relative to log10(count), so s_L = √(s_R² − s_r²) = 5.2 %.
2. The laboratory's repeatability from 10 replicates is s_l = 5.0 %; ISO 21748 7.3.2 allows it in place of s_r: s′_R = √(s_L² + s_l²) = 7.2 %.
3. Sub-sampling and weighing, not covered by the study, add 3.0 % (expert estimate); u = √(s′_R² + 3.0²) = 7.8 % and U = 15.6 % (k = 2).

**Result:**

- U = 16 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/general/iso21748-c3-aerobic-plate-count-shrimp.json --lang en`

### Aerobic plate count, vegetables: adjusted reproducibility (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-vegetables` — ISO 21748:2017, Annex C.3, Tables C.2–C.4

ISO 21748:2017 C.3 (AOAC 990.12, vegetables): relative uncertainty of log10(count) from collaborative-study s_R and s_r, the laboratory's better repeatability (7.3.2) and a sample-preparation term.

**Approach:** top-down (validation and quality-control data)

**Steps:**

1. Collaborative study (12 laboratories, ISO 5725-2): s_R = 9.2 %, s_r = 6.3 % relative to log10(count), so s_L = √(s_R² − s_r²) = 6.7 %.
2. The laboratory's repeatability from 10 replicates is s_l = 5.0 %; ISO 21748 7.3.2 allows it in place of s_r: s′_R = √(s_L² + s_l²) = 8.4 %.
3. Sub-sampling and weighing, not covered by the study, add 3.0 % (expert estimate); u = √(s′_R² + 3.0²) = 8.9 % and U = 17.8 % (k = 2).

**Result:**

- U = 18 % (k = 2.00), relative to the measured value

**Run:** `uncertainty run library/general/iso21748-c3-aerobic-plate-count-vegetables.json --lang en`

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

### End-gauge calibration with Gaussian inputs (NIST Uncertainty Machine §8)

`num-8-end-gauge-gaussian` — T. Lafarge, §8, Table 3 and Figure 3; JCGM 100:2008, H.1, Table H.1

NUM manual §8: GUM H.1 with the six inputs of the GUM's first-order budget, all modelled as Gaussian; the Monte Carlo method finds the second-order contributions that Gauss's formula misses.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. l = l_S + d − l_S(δα·θ + α_S·δθ) with l_S = 50 000 623(25) nm, d = 215(9.7) nm, δα = 0(0.58 × 10⁻⁶) °C⁻¹, θ = −0.1(0.41) °C, α_S = 11.5(1.2) × 10⁻⁶ °C⁻¹ and δθ = 0(0.029) °C (GUM Table H.1), all Gaussian.
2. At the estimates δα = δθ = 0, so θ and α_S have zero sensitivity coefficients and Gauss's formula gives u(l) = 31.7 nm; the program warns that the second-order products δα·θ and α_S·δθ are missed.
3. The Monte Carlo method gives a standard deviation of 33.9 nm and the 99 % probabilistically symmetric interval (50 000 749, 50 000 927) nm, close to the GUM's own interval (50 000 745, 50 000 931) nm (k = 2.92 from ν_eff = 16.7). The interval of this project's GUM evaluation, 50 000 838 ± 82 nm (k = 2.58), is too short by 7 nm at each end and is not validated.

**Result:**

- l = (50000838 ± 82) nm, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.58, corresponding to a coverage probability of approximately 99 %.

**Notes:**

- The project 'gum-h1-end-gauge' reproduces the GUM's own evaluation (θ split into its mean and cyclic part, Welch–Satterthwaite, k = 2.92).

**Run:** `uncertainty run library/length/num-8-end-gauge-gaussian.json --lang en`

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

### Mass of a powder by weighing (NIST TN 1900 E1)

`nist-tn1900-e1-weighing` — NIST Technical Note 1900, Example E1, Exhibit 1

TN 1900 E1: mass of a powder from three weighings with a single-pan balance; linear model, GUM and MCM agree.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The mass of the powder is m_P = c_P − c_E − (c_R2 − c_R1): the reading with the powder in the container, minus the reading of the empty container, minus the drift of the balance between two readings of a reference weight.
2. The standard uncertainties of c_P and c_E (0.005015 g) combine the uncertainty of the container (0.005 g) with those of the balance (calibration, repeatability, linearity, temperature: 0.1, 0.1, 0.2 and 0.3 mg); that of the drift term is 0.0005477 g (TN 1900 Exhibit 1).
3. All sensitivity coefficients are ±1, so u(m_P)² = u(c_P)² + u(c_E)² + u(c_R2 − c_R1)², u = 0.0071 g; the output is Gaussian and the Monte Carlo method gives the same 95 % interval [50.270, 50.298] g.

**Result:**

- m_P = (50.284 ± 0.014) g, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Run:** `uncertainty run library/mass/nist-tn1900-e1-weighing.json --lang en`

## Physical properties

### Thermal expansion coefficient, Gaussian inputs, 95 % (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-gaussian-95` — NIST Technical Note 1900, Example E13, Exhibit 20

TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; gaussian inputs, 95 % coverage.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. α = (L₁ − L₀)/(L₀(T₁ − T₀)) with L₀ = 1.4999 m at T₀ = 288.15 K and L₁ = 1.5021 m at T₁ = 373.10 K; u(L₀) = 0.0001 m, u(L₁) = 0.0002 m, u(T₀) = 0.02 K, u(T₁) = 0.05 K. The result is expressed in 10⁻⁶ K⁻¹.
2. The length difference, 2.2 mm, carries almost all the uncertainty: u(α) = 1.8 × 10⁻⁶ K⁻¹, about 10 % of α = 17.3 × 10⁻⁶ K⁻¹.
3. With Gaussian inputs the GUM and Monte Carlo results agree; the 95 % interval from the percentiles of the Monte Carlo sample is [13.8, 20.7] × 10⁻⁶ K⁻¹.

**Result:**

- alpha = (17.3 ± 3.4) 10⁻⁶ K⁻¹, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- The companion projects nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} compare the two input models at 95 % and 99 %: with t₃ inputs the 95 % interval is shorter and the 99 % interval longer than with Gaussian inputs.

**Run:** `uncertainty run library/physical/nist-tn1900-e13-thermal-expansion-gaussian-95.json --lang en`

### Thermal expansion coefficient, Gaussian inputs, 99 % (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-gaussian-99` — NIST Technical Note 1900, Example E13, Exhibit 20

TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; gaussian inputs, 99 % coverage.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. α = (L₁ − L₀)/(L₀(T₁ − T₀)) with L₀ = 1.4999 m at T₀ = 288.15 K and L₁ = 1.5021 m at T₁ = 373.10 K; u(L₀) = 0.0001 m, u(L₁) = 0.0002 m, u(T₀) = 0.02 K, u(T₁) = 0.05 K. The result is expressed in 10⁻⁶ K⁻¹.
2. The length difference, 2.2 mm, carries almost all the uncertainty: u(α) = 1.8 × 10⁻⁶ K⁻¹, about 10 % of α = 17.3 × 10⁻⁶ K⁻¹.
3. With Gaussian inputs the GUM and Monte Carlo results agree; the 99 % interval from the percentiles of the Monte Carlo sample is [12.7, 21.8] × 10⁻⁶ K⁻¹.

**Result:**

- alpha = (17.3 ± 4.5) 10⁻⁶ K⁻¹, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.58, corresponding to a coverage probability of approximately 99 %.

**Notes:**

- The companion projects nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} compare the two input models at 95 % and 99 %: with t₃ inputs the 95 % interval is shorter and the 99 % interval longer than with Gaussian inputs.

**Run:** `uncertainty run library/physical/nist-tn1900-e13-thermal-expansion-gaussian-99.json --lang en`

### Thermal expansion coefficient, Student t₃ inputs, 95 % (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-student-95` — NIST Technical Note 1900, Example E13, Exhibit 20

TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; student t₃ inputs, 95 % coverage.

**Approach:** Monte Carlo method (JCGM 101) only; run the project for its results

**Steps:**

1. α = (L₁ − L₀)/(L₀(T₁ − T₀)) with L₀ = 1.4999 m at T₀ = 288.15 K and L₁ = 1.5021 m at T₁ = 373.10 K; u(L₀) = 0.0001 m, u(L₁) = 0.0002 m, u(T₀) = 0.02 K, u(T₁) = 0.05 K. The result is expressed in 10⁻⁶ K⁻¹.
2. The length difference, 2.2 mm, carries almost all the uncertainty: u(α) = 1.8 × 10⁻⁶ K⁻¹, about 10 % of α = 17.3 × 10⁻⁶ K⁻¹.
3. Each estimate is taken as the mean of four repeated observations, so TN 1900 assigns t₃ distributions rescaled to standard deviations equal to the standard uncertainties. α and u(α) are unchanged, but the output is more peaked and heavier-tailed: the 95 % interval is [14.0, 20.5] × 10⁻⁶ K⁻¹.

**Notes:**

- The companion projects nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} compare the two input models at 95 % and 99 %: with t₃ inputs the 95 % interval is shorter and the 99 % interval longer than with Gaussian inputs.
- TN 1900 rescales each t₃ distribution to standard deviation u (scale u/√3), whereas JCGM 101 6.4.9 uses the scale s/√n = u. Because the GUM uncertainty of a t input is its scale, the GUM evaluation is switched off here; the Gaussian projects give the GUM result.

**Run:** `uncertainty run library/physical/nist-tn1900-e13-thermal-expansion-student-95.json --lang en`

### Thermal expansion coefficient, Student t₃ inputs, 99 % (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-student-99` — NIST Technical Note 1900, Example E13, Exhibit 20

TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; student t₃ inputs, 99 % coverage.

**Approach:** Monte Carlo method (JCGM 101) only; run the project for its results

**Steps:**

1. α = (L₁ − L₀)/(L₀(T₁ − T₀)) with L₀ = 1.4999 m at T₀ = 288.15 K and L₁ = 1.5021 m at T₁ = 373.10 K; u(L₀) = 0.0001 m, u(L₁) = 0.0002 m, u(T₀) = 0.02 K, u(T₁) = 0.05 K. The result is expressed in 10⁻⁶ K⁻¹.
2. The length difference, 2.2 mm, carries almost all the uncertainty: u(α) = 1.8 × 10⁻⁶ K⁻¹, about 10 % of α = 17.3 × 10⁻⁶ K⁻¹.
3. Each estimate is taken as the mean of four repeated observations, so TN 1900 assigns t₃ distributions rescaled to standard deviations equal to the standard uncertainties. α and u(α) are unchanged, but the output is more peaked and heavier-tailed: the 99 % interval is [11.5, 23.0] × 10⁻⁶ K⁻¹.

**Notes:**

- The companion projects nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} compare the two input models at 95 % and 99 %: with t₃ inputs the 95 % interval is shorter and the 99 % interval longer than with Gaussian inputs.
- TN 1900 rescales each t₃ distribution to standard deviation u (scale u/√3), whereas JCGM 101 6.4.9 uses the scale s/√n = u. Because the GUM uncertainty of a t input is its scale, the GUM evaluation is switched off here; the Gaussian projects give the GUM result.

**Run:** `uncertainty run library/physical/nist-tn1900-e13-thermal-expansion-student-99.json --lang en`

### Falling-ball viscometer (NIST TN 1900 E3)

`nist-tn1900-e3-falling-ball-viscometer` — NIST Technical Note 1900, Example E3, Exhibit 3

TN 1900 E3: dynamic viscosity of a NaOH solution by comparison with a calibration liquid; large relative uncertainties of the travel times make the output skewed.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. μ_M = μ_C (ρ_B − ρ_M)/(ρ_B − ρ_C) · t_M/t_C with μ_C = 4.63 mPa s, ρ_B = 2217, ρ_C = 810, ρ_M = 1180 kg/m³, t_C = 36.6 s and t_M = 61 s; all inputs Gaussian.
2. The relative standard uncertainties are 1 % for μ_C, 15 % for t_C and 10 % for t_M; the densities have u = 0.5 kg/m³. TN 1900 notes that these are deliberately large.
3. Substituting the estimates gives 5.69 mPa s and the first-order law of propagation u = 1.03 mPa s. Because 1/t_C is strongly non-linear over ±15 %, the Monte Carlo mean is 5.82 mPa s, u = 1.11 mPa s, and the 95 % interval [4.05, 8.39] mPa s is asymmetric about the estimate.
4. The GUM interval 5.69 ± 2.01 mPa s is therefore not validated: its end points lie 0.38 mPa s and 0.69 mPa s below those of the Monte Carlo interval. The shortest Monte Carlo interval, [3.84, 8.05] mPa s, differs again from the probabilistically symmetric one used by TN 1900.

**Result:**

- mu_M = (5.7 ± 2.0) mPa s, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- TN 1900 states that the conventional propagation formula, as implemented in the NIST Uncertainty Machine, gives 1.11 mPa s, identical to the Monte Carlo value. The first-order law of propagation (GUM eq. 10) applied to the stated model and inputs gives 1.027 mPa s; with the second-order terms of GUM 5.1.2 it gives about 1.10 mPa s. The NIST Uncertainty Machine manual (v1.4, §9) repeats the statement, but the output it prints in Figure 5 gives u(y) = 1 mPa s for Gauss's formula, consistent with 1.027 mPa s: the 1.11 mPa s of the text is the Monte Carlo value.

**Run:** `uncertainty run library/physical/nist-tn1900-e3-falling-ball-viscometer.json --lang en`

### Stefan–Boltzmann constant from CODATA 2010 values (NIST Uncertainty Machine §11)

`num-11-stefan-boltzmann` — T. Lafarge, §11, Table 5 and Figure 7

NUM manual §11: σ = 32π⁵hR⁴R∞⁴/(15A_r(e)⁴M_u⁴c⁶α⁸) from the 2010 CODATA values; the molar gas constant R contributes almost all the uncertainty.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. Inputs: h = 6.626 069 57(29) × 10⁻³⁴ J s, R = 8.314 462 1(75) J mol⁻¹ K⁻¹, R∞ = 10 973 731.568 539(55) m⁻¹, A_r(e) = 5.485 799 094 6(22) × 10⁻⁴, α = 7.297 352 569 8(24) × 10⁻³; M_u = 10⁻³ kg/mol and c = 299 792 458 m/s are exact. The result is expressed in 10⁻⁸ W m⁻² K⁻⁴.
2. For a product of powers the relative variance is Σ(p_i u_r(x_i))²: R enters with power 4 and u_r(R) = 9.0 × 10⁻⁷, so u_r(σ) = 3.6 × 10⁻⁶ and u(σ) = 2.05 × 10⁻¹³ W m⁻² K⁻⁴; the other inputs contribute less than 0.02 %.

**Result:**

- sigma = (5.6703725 ± 0.0000401) 10⁻⁸ W m⁻² K⁻⁴, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 1.96, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- The CODATA adjustment correlates these constants; as noted in the manual, including the correlations does not change u(σ) at the reported digit. Since the 2019 redefinition of the SI, h is exact and σ = 5.670 374 419… × 10⁻⁸ W m⁻² K⁻⁴ has no uncertainty.

**Run:** `uncertainty run library/physical/num-11-stefan-boltzmann.json --lang en`

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

### Monthly mean of daily maximum temperature (NIST TN 1900 E2)

`nist-tn1900-e2-surface-temperature` — NIST Technical Note 1900, Example E2, Exhibit 2

TN 1900 E2: an observation equation t_i = τ + E_i for 22 daily readings; Type A evaluation with 21 degrees of freedom.

**Approach:** GUM uncertainty framework (JCGM 100) and Monte Carlo method (JCGM 101)

**Steps:**

1. The measurand τ is the mean of the true daily maxima in May 2012; each reading deviates from it by an error E_i, modelled as independent and Gaussian with mean 0 (the observation equation of TN 1900).
2. The estimate is the mean of the m = 22 readings, 25.59 °C; s = 4.09 °C, so u(τ) = s/√22 = 0.872 °C with ν = 21 (GUM 4.2.3).
3. The coverage factor for 95 % is the t quantile t₀.₉₇₅(21) = 2.08, giving [23.8, 27.4] °C. The Monte Carlo method draws τ from the scaled and shifted t₂₁ distribution of JCGM 101 6.4.9 and gives the same interval.

**Result:**

- tau = (25.6 ± 1.8) °C, where the expanded uncertainty is the combined standard uncertainty multiplied by the coverage factor k = 2.08, corresponding to a coverage probability of approximately 95 %.

**Notes:**

- The Monte Carlo standard deviation is s/√m · √(ν/(ν − 2)) = 0.917 °C, larger than u = 0.872 °C, because the t distribution of JCGM 101 6.4.9 has scale s/√m; the coverage intervals coincide.
- TN 1900 also gives the distribution-free Wilcoxon interval [23.6, 27.6] °C, which is not a GUM or Monte Carlo result and is not reproduced here.

**Run:** `uncertainty run library/temperature/nist-tn1900-e2-surface-temperature.json --lang en`

