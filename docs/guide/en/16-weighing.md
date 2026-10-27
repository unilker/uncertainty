# 16. Calibration of weighing instruments (EURAMET cg-18)

EURAMET Calibration Guide No. 18 v4.0 (2015) is the reference for calibrating non-automatic weighing
instruments, from analytical balances to weighbridges. The program covers the two parts of a calibration
certificate:

- the **error of indication** at each test load and its uncertainty, as a measurement model (16.1–16.3);
- the **error approximation function** E(I) fitted to all test points, which gives the error and its
  uncertainty at any reading (Appendix C; the `error_function` section, 16.4).

All four examples of Appendix H are library projects: `cg18-h1-*` (220 g / 0.1 mg), `cg18-h2-*` (60 kg
multi-interval), `cg18-h3-*` (30 t weighbridge with substitution loads) and `cg18-h4-*` (error function of a
400 g balance).

## 16.1 The error of indication as a model (7.1)

E = I − m_ref (7.1-1). Each term of the guide's budget is an input quantity of the model:

| Term | Input | Formula |
|---|---|---|
| rounding at zero and at the load | rectangular, half-width d₀/2 and d_L/2 | 7.1.1-2a, -3a |
| repeatability | s from the repeatability test, Type A with ν = n − 1 (one reading) | 7.1.1-5 |
| eccentricity | rectangular, half-width ΔI_ecc,max·L/(2L_ecc) | 7.1.1-10 |
| creep and hysteresis | rectangular, half-width E₀·L/Max | 7.4.4-7 |
| reference mass | weights at nominal value: rectangular, half-width mpe; or the certificate value with U/k | 7.1.2-1 to -3 |
| drift of the weights | rectangular, half-width D = k_D·U or D = mpe | 7.1.2-10, -11 |
| air buoyancy | correction −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) in the model (option 2), or the limits 7.1.2-5c, -5d, -5e | 7.1.2-4 to -5e |

For a multi-interval instrument (Example H2), d_L is the scale interval of the interval that contains the load,
and the repeatability is that of the test in the same interval. The coverage factor follows from ν_eff
(Welch–Satterthwaite, B3-1) and the t-distribution for 95.45 %. The guide reads k from GUM Table G.2 at the next
lower tabulated ν, which can give a slightly larger k than the exact quantile (e.g. 2.05 instead of 2.03 at
ν = 90). The projects note the difference.

## 16.2 Buoyancy: option 1 and option 2

Without information on the air density (option 1), the buoyancy is a limit. Formula 7.1.2-5c applies if the
instrument is adjusted immediately before calibration, 7.1.2-5d if not, and 7.1.2-5e if the temperature
variation at the site is known. With the air density measured during calibration (option 2), the correction is
part of the model and its uncertainty (7.1.2-5a) follows from the propagation. Examples H1 and H2 show both. For
H2 (d = 10 g) the correction is negligible, for H1 (d = 0.1 mg) it is not.

## 16.3 Substitution loads (4.3.3, 7.1.2-15, Example H3)

Large instruments are tested with standard weights for part of the load and substitution loads for the rest.
The substitution load is adjusted until the instrument shows the same indication as with the weights:
L_sub1 = m_ref + I(L_sub1) − I(m_ref). Every indication used in this way brings its own rounding, repeatability,
eccentricity and creep terms. In the model these are inputs of their own, and the same standard weights
appear in every step (fully correlated). This reproduces formula 7.1.2-15 for u(L_sub). The guide's ν_eff counts
only the repeatability of the indication at the load. The model gives each repeatability term ν = n − 1 and
therefore a smaller ν_eff (research log F69).

## 16.4 The error approximation function (`error_function`, Appendix C2.2)

The errors E_j at the test points I_j are fitted by a polynomial, by default the straight line through zero
E = a₁I (C2.2-16), or with a constant term and higher degrees (C2.2-3). The covariance matrix of the errors is

U(e) = U(m_ref) + U(I_Cal) + U(mod)  (C2.2-3a).

Here U(m_ref) = s sᵀ for fully correlated reference masses (C2.2-3b; `"reference_correlation": "none"` for
uncorrelated ones), U(I_Cal) is the diagonal of u²(I_j), and U(mod) = s_m²·I is a model term (C2.2-3c). The
generalized least-squares solution gives â = (XᵀPX)⁻¹XᵀPe with P = U(e)⁻¹, U(â) = (XᵀPX)⁻¹, the residuals
v = Xâ − e and χ²_obs = vᵀPv with ν = n − n_par (C2.2-6 to -9).

Two goodness-of-fit criteria are reported:

- **C2.2-2a**: χ²_obs ≤ ν: the function is consistent with the data;
- **C2.2-2b**: every residual within its expanded uncertainty, |v_j| < k·u(E_appr,j), which is much more
  restrictive.

When a criterion fails, the guide enlarges s_m until it is met. The report gives the smallest s_m for each
criterion (found by bisection), so the choice can be documented. In Example H4 the data need s_m = 0.044 mg for
C2.2-2a and 0.22 mg for C2.2-2b; the guide uses 0.05 and 0.25 mg. A larger s_m means a larger uncertainty of
the approximated error: at 400 g, U(E_appr) is 0.190 mg with s_m = 0.05 mg and 0.334 mg with 0.25 mg.

The approximated error at any reading R (`readings`) is E_appr(R) = rᵀâ with
u²(E_appr) = (f′(R))²u²(R) + rᵀU(â)r (C2.2-12, -13). Points are given either with their error (`error`) or with
the reference mass (`reference`); `error_factor` converts I − m_ref to the unit of the errors (1000 for g to
mg).
