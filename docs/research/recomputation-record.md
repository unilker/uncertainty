# S8. Recomputation record of the c1 findings

This supplement gives, for every finding that was classed c1 at some stage, the value printed in the document, the
value computed by the program and the value of the recomputation by a person who had not seen the code (Table S8.1),
and, for every settled c1 finding, the inputs from the document, the expression used and the result (Section 3). The
complete derivations, with the history of each finding, are in the research log of the repository
(`docs/research/research-log.md`; findings F…, decisions D106–D112).

## 1. How the recomputation was made

The recomputer is a metrologist of the same institute as the author, not an author of the paper, who had not seen the
program's code or its results. For each finding, a form gave the document, the example, the printed quantity and the
point at which to start (Table S8.2); the start was always the data of the example, not a total or table printed after
them, because a recomputation that starts from a printed total reproduces it by construction. The program's values
were in a separate key, opened after the form had been filled in. The recomputer worked from the documents alone,
with hand calculation, Python (NumPy, SciPy) and Monte Carlo simulation up to 10⁷ trials, and wrote every
intermediate step on the form. An AI assistant of the Claude family (Anthropic), the family of the assistant with which
the program was written, was used in this work; the form does not record which steps it performed. The author checked
every written step against the document before comparing the result with the program (research log D108, D109).

The recomputation is therefore independent of the program's code and of its author, but not of the assistant. For
this reason Section 3 gives each settled c1 finding as a few lines of arithmetic with the numbers printed in the
document, which a reader can repeat by hand.

## 2. Printed, program and recomputed values

**Table S8.1.** The 13 findings of the recomputation form (classed c1 when the form was written) and F101, which was
split from F40 afterwards. Printed: the value in the document; program: KAPSAM; recomputer: the value written on the
form. Class: after the recomputation and the second internal review.

| F | Document | Quantity | Printed | Program | Recomputer | Class | Remark |
|---|---|---|---|---|---|---|---|
| F21 | EA-4/02 S13 | d_X / mm | 90.0003 | 90.000 23 | 90.000 23 | c1 | |
| F28 | NIST TN 1900 E4 | v / (m/s) | 40.64 | 57.48 | 57.48 (u 0.35) | c1 | |
| F39 | ISO 21748 C.3 | U(flour) / % | 12.8 | 12.6 | 12.6 | c1 | |
| F42 | ISO 6143:2025 Ex. 3 | u(x) / (µmol/mol) | 0.013280 | 0.014246 | 0.014246 | c1 | the recomputer's own errors-in-variables fit |
| F56 | UKAS M3003 K.5 | U / mg | 0.30 | 0.31 (u_c 0.1525) | 0.31 (u_c 0.1528) | c1 | |
| F57 | UKAS M3003 K.3 | U / dB | 0.049 | 0.040 | 0.040 | c1 | |
| F58 | UKAS M3003 K.6 | U / nm | 81.5 | 82.04 | 82.06 | c1 | 82.0 and 82.1 at the printed precision; effect on U 0.0066 and 0.0069 of U |
| F77 | GUM-5 Ex. 15 | MCM s.d. / kN | 10.1 | 9.93 | 9.93 (t, ν = 5); 9.91 (normal) | c1 | |
| F78 | GUM-5 Ex. 1 | pH_X | 7.0109 | 7.0043 | 7.0043 | c1 | |
| F82 | GUM-5 Ex. 4 | u(y_CO₂) / (µmol/mol) | 5 | 3.96 → 4 | 3.91 → 4 | c1 | program from Table 4.2, recomputer from Table 4.1 |
| F40 | ISO 5725-3:2023 J.5, level 6 | s_R / (% × 10⁻³) | 15.962 | 16.331 | 15.962, 16.331 or 16.781 | m | three defensible treatments of a negative variance (Section 4) |
| F87 | EMUE E3.4 | U(Pb) / (mg/kg) | 2.8 | 2.9 | 2.87 → 2.9 | c2 | with u′ rounded to 5 %, as the example rounds, 2.8 follows too |
| F90 | EMUE E3.3 | u (validation) / (nmol/mol) | 4.7 | 4.52 | 4.47 | open | the printed (rounded) inputs allow 4.14–4.78 |
| F101 | ISO 5725-3:2023 J.2/J.5, level 5 | s_r / (% × 10⁻³) | 6.237 | 14.715 (2023 data); 6.237 (1994 data) | not reproducible from the 2023 data | c1 | the result table does not follow from the data table printed with it; which table is wrong is not stated |

The recomputer's value equals the program's at the printed digits in all rows except F40, where it gives the three
treatments, F82, where the two start from different tables of the document and agree after rounding, and F90, where
both lie inside the range that the printed inputs allow.

**Table S8.2.** Where the form asked the recomputation to start.

| F | Start |
|---|---|
| F21 | Δl as the mean of the five observations and the coaxiality correction from Eq. (S13.5); not the values entered in the budget S13.10. |
| F28 | The equation printed in E4 with the printed Δ, p, T and R_s, evaluated by hand; not the NUM example file. |
| F39 | The flour components of Table C.4, combined by the formula of C.3. |
| F40 | The raw data of Table J.2 with the stated outlier exclusions; not the summary in Table J.5. |
| F42 | Formula (15) for y = 4950.6 with the parameters and their covariances of the example; not Table B.12. |
| F56 | Each u_i of K.5.9 from its stated limit and distribution, then their combination; not the printed u_c. |
| F57 | The mismatch term from formula E5 (E.9.5) with the coefficients of K.3.6, then u_c and U. |
| F58 | The sensitivity coefficients from the model of K.6.1, −L(α·Δt + Δα·ΔT), then u_c and U. |
| F77 | An own Monte Carlo evaluation with the input distributions stated in the example. |
| F78 | The measured data of the example and its measurement model; not the pH value printed in the result. |
| F82 | The input quantities of the example (Table 4.1), propagated by the law of propagation of uncertainty. |
| F87 | The column u(δ̂)/x_ref of Table E3.4.2 summarised as the text says (root mean square), then U for Pb. |
| F90 | Eq. (E3.3.19) with the inputs of the example; the result to three significant digits. |

## 3. Derivations of the settled c1 findings

The criterion (Section 4.3 of the paper): a final printed result either changes at its printed digits when the
inconsistency is corrected, even with the document's own rounding, or does not follow from the data printed in the
document itself (F101).

**F21. EA-4/02 S13 (gauge block, diameter of a ring).** Data: five observations of Δl with mean 49.999 536 mm; the
coaxiality correction of Eq. (S13.5), δl_P = −0.0037 µm. The budget S13.10 enters Δl = 49.999 55 mm and
δl_P = +0.000 004 mm and obtains d_X = 40.0007 + 49.999 55 + 0.000 004 = 90.000 25 mm, reported as
(90.000 3 ± 0.000 9) mm. With the data: d_X = 40.0007 + 49.999 536 − 0.000 004 = 90.000 232 mm → (90.000 2 ± 0.000 9)
mm. The printed digit changes (0.1 µm = 0.11 U).

**F28. NIST TN 1900 E4 (Pitot tube).** Stated equation v = √(2ΔR_sT/p) with Δ = 1.993 kPa, p = 101.4 kPa,
T = 292.8 K, R_s = 287.058 J kg⁻¹ K⁻¹: v = √(2 × 1993 × 287.058 × 292.8 / 101 400) = 57.48 m/s. Printed
v = 40.64 m/s = 57.48/√2, the value of √(ΔR_sT/p); the printed u(v) = 0.25 m/s and the intervals scale in the same
way (relative uncertainty 0.61 % in both). Reported independently by Delker (2021, footnote 1).

**F39. ISO 21748 C.3 (flour).** Table C.4 components 2.4 %, 5.0 % and 3.0 %: u = √(2.4² + 5.0² + 3.0²) = 6.31 %
(6.29 % with the unrounded s_L), U = 2u = 12.6 %. Printed u = 6.4 %, U = 12.8 %; the intervals of Table C.5 follow from
12.8 % (with 12.6 % the flour interval is 80 to 282 CFU, not 79 to 285). Shrimp (7.8 %) and vegetables (8.9 %) are
reproduced from the same table.

**F42. ISO 6143:2025 Example 3 (exponential function).** The standard states (B.1, B.2.4) that its program B_LEAST gives
incorrect parameter uncertainties for the exponential function and prints them (u(b) = 8.1404 × 10⁻², 7.3965 × 10⁻²,
3.7068 × 10⁻¹¹); the errors-in-variables fit of the printed calibration data gives 3.18, 3.17 and 1.274 × 10⁻⁶
(identical with ODRPACK). Formula (15) for the sample at y = 4950.6 with the correct covariance matrix gives
u(x) = 0.014 246 µmol/mol; Table B.12 prints 0.013 280, which follows from the printed (incorrect) covariances
(0.013 283). The printed u(x) is 6.8 % too low; the standard does not correct it. A Monte Carlo simulation of the
calibration (4000 refits) gives 0.014 06.

**F56. UKAS M3003 K.5 (mass).** The u_i column of K.5.9: 0.05, 0.058, 0.029, 0.029, 0.115, 0.05 mg.
Σu_i² = 0.0025 + 0.003 364 + 0.000 841 + 0.000 841 + 0.013 225 + 0.0025 = 0.023 271 mg², u_c = 0.1525 mg, U = 2u_c =
0.305 → 0.31 mg. Printed u_c = 0.150 mg, U = 0.30 mg. The printed u_c follows if the drift limit is divided by 2
instead of √3, or if one of the two rounding terms is left out; the cause cannot be identified.

**F57. UKAS M3003 K.3 (attenuator, mismatch).** Formula E5 of E.9.5 with the coefficients of K.3.6 (Γ_G = Γ_L = 0.03,
s11a = s11b = 0.05, s22a = 0.05, s22b = 0.01, s21a = 1, s21b = 0.31) gives a mismatch limit M = 0.0170 dB; the budget
uses 0.022 dB. With 0.0170 dB and the other components of the budget unchanged, u_c = 0.0202 dB and U = 0.040 dB;
printed U = 0.049 dB. (0.022 dB follows with s11 = 0.09 for one of the coefficients; not identifiable from the
example.)

**F58. UKAS M3003 K.6 (gauge block).** The model contains −L(α·Δt + Δα·ΔT). By the model, the temperature difference
Δt = ±0.08 °C acts through L·α = 115 nm/°C: ±9.2 nm, rectangular, u = 5.3 nm; the product Δα·ΔT has zero first-order
sensitivity and contributes L·u(Δα)·u(ΔT) = 0.94 nm through the second-order term. The budget instead evaluates Δt
through Δα (±1.6 nm) and adds it linearly to the ΔT term (±4 nm) as "correlated" (±5.6 nm, triangular). With the
model's coefficients, u_c = 41.0 nm and U = 82.0 nm; printed u_c = 40.7 nm, U = 81.5 nm (0.6 % of U). EA-4/02 S4
evaluates the same model as the stated model requires.

**F77. JCGM GUM-5 Example 15 (wire rope).** 15.3.1 assigns to F a t-distribution with mean 10 005.7 kN, scale
u(F̄) = s/√6 = 0.7 kN and ν = 5. The Monte Carlo standard deviation is then √(u_c² − u²(F̄) + u²(F̄)·ν/(ν − 2)) =
9.93 kN (10⁶ trials: 9.93 kN); 15.5.2 prints 10.1 kN. 10.1 kN follows if the scale is the standard deviation of one
test, s = 1.75 kN, instead of that of the mean. The difference 9.9 against 10.1 kN that 15.6.1 attributes to the shape
of the distribution is this input inconsistency.

**F78. JCGM GUM-5 Example 1 (pH).** 1.9.1 obtains pH_X,T1 = 7.0002 and pH_X,T2 = 7.0161 and, by the third
interpolation, pH_X = 7.0043; it then states "pH_X = 7.0109, u(pH_X) = 0.0041". 7.0109 does not follow from the data;
the document prints the correct intermediate value 7.0043 itself (0.80 U).

**F82. JCGM GUM-5 Example 4 (gas mixture).** The law of propagation of uncertainty with the inputs of Table 4.2
(20 inputs; dominated by the atomic weights of N, C and O) gives u(y_CO₂) = 3.96 µmol/mol, which rounds to 4; with the
means and standard deviations of the means of Table 4.1 it gives 3.91 → 4. Printed: 5 µmol/mol (u_rel 0.003 %,
recomputed 0.0020 %). With u(r̂_j) = 2 mg for the three weighings the result would be 4.50 → 5; the document does
not state such a value. u(y_N₂) = 4.72 → 5 agrees with the printed value.

**F101. ISO 5725-3:2023 Tables J.2 and J.5 (vanadium in steel).** Table J.2 of the 2023 edition differs from Table D.2
of the 1994 edition in 15 entries: 12 at level 3 (the second day-1 result of laboratories 7–20 repeats the first), 1 at
level 4 (laboratory 18) and 2 at level 5 (laboratories 2 and 3). Table J.5, carried over unchanged, is reproduced
exactly from the 1994 data with the stated exclusions, but not from the 2023 Table J.2: at level 5, s_r = 14.715 ×
10⁻³ % from the 2023 data against the printed 6.237 × 10⁻³ %; at level 3, 0.652 against 1.739. The printed results do
not follow from the document's own data. Which of the two tables is wrong cannot be decided from the 2023 edition:
⟦CORS8⟧

## 4. Findings reclassified or open

**F40 (class m).** At level 6 of ISO 5725-3 J.5, the mean squares MS₀ = 617.90, MS₁ = 55.39 and MS_e = 91.11 (× 10⁻⁶)
give a negative day variance s₍₁₎² = −26.79 × 10⁻⁶. The printed s_R = 15.962 × 10⁻³ % = √((MS₀ + MS₁ + MS_e)/3) applies
the formulas as printed and equals the REML estimate with variances ≥ 0; 16.331 sets s₍₁₎² = 0 throughout (the
program); 16.781 sets it to zero in the sum only (ISO 5725-2, 8.4.5.4). All three are defensible, so the finding
concerns the consistency of the printed row (s_I(T) = s_r with s₍₁₎² = 0, s_R with it kept), not a final result.

**F87 (class c2).** The text calls 3.3 % the root mean square of the relative bias uncertainties of Table E3.4.2; it is
their arithmetic mean (3.28 %), the root mean square is 3.50 %. With x = 27.8 mg/kg, U = 2.87 → 2.9 mg/kg; with u′
rounded to 5 %, as the example rounds it, the printed 2.8 mg/kg follows either way.

**F90 (open).** Eq. (E3.3.19) with the printed (rounded) inputs gives values between 4.14 and 4.78 nmol/mol, because
three large terms cancel; the printed 4.7 lies in this range. With the unrounded parameters of a full weighted total
least-squares fit, u = 4.52 (4.35 without the covariances). Whether the printed 4.7 follows from the document's own
unrounded values cannot be decided from the example; the finding is open and not counted.
