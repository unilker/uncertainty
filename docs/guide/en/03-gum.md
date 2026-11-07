# 3. The GUM uncertainty framework

This chapter summarises the evaluation of measurement uncertainty according to JCGM 100:2008 (the GUM), as
implemented in the program. Clause numbers refer to the GUM unless stated otherwise.

## 3.1 Measurand and measurement model

The first step is to define the **measurand** — the quantity intended to be measured — and to express it by a
**measurement model**

$$Y = f(X_1, X_2, \dots, X_N),$$

where the input quantities Xᵢ include everything that influences the result: readings, calibration values,
corrections for systematic effects, reference values, physical constants (GUM 4.1). A correction whose best
estimate is zero is still an input quantity, because its uncertainty contributes to that of Y.

In a project file the model is typed as a formula:

```json
"output": { "name": "c_Cd", "unit": "mg/L", "expression": "1000 * m * P / V" }
```

Expressions may use `+ - * /`, powers (`^` or `**`, or `pow(x, y)`), parentheses, the constant `pi`, and the
functions `sqrt`, `exp`, `ln`, `log10`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `sinh`, `cosh`, `tanh`,
`abs` and the four-quadrant inverse tangent `atan2(y, x)`. Names may contain letters of any alphabet, digits and `_` (e.g. `ρ_a`, `dV_cal`). Long models are
easier to read with **intermediate quantities**, which are expanded into the model symbolically (no
approximation is involved):

```json
"intermediates": [ { "name": "V", "expression": "V_nom + dV_cal + dV_rep + dV_temp" } ]
```

## 3.2 Standard uncertainties of the inputs

Each input quantity is described by a probability distribution that expresses the available knowledge about it;
its estimate xᵢ is the expectation and its **standard uncertainty** u(xᵢ) the standard deviation of that
distribution (GUM 4.1.6, 3.3.5).

### Type A evaluation

For n independent observations q₁ … qₙ of a quantity under the same conditions (GUM 4.2):

$$\bar q = \frac{1}{n}\sum q_k,\qquad s(q) = \sqrt{\frac{\sum (q_k-\bar q)^2}{n-1}},\qquad u(\bar q) = \frac{s(q)}{\sqrt n},\qquad \nu = n-1.$$

If the standard deviation is known from an earlier, larger study (a pooled standard deviation s_p with ν_p
degrees of freedom), the uncertainty of the mean of the current n observations is s_p/√n with ν_p degrees of
freedom (GUM 4.2.4). In the program: `"type": "type_a", "observations": [...]` or
`"type": "type_a_pooled", "mean": ..., "pooled_sd": ..., "pooled_dof": ..., "n": ...`.

### Type B evaluation

Other information — certificates, specifications, handbooks, experience — is converted to a standard
uncertainty through an assumed distribution (GUM 4.3). With a the half-width of the limits:

| Information | Distribution | Standard uncertainty | `type` |
|---|---|---|---|
| expanded uncertainty U with coverage factor k (certificate) | normal | U/k | `normal` with `expanded`, `k` |
| standard uncertainty stated | normal | u | `normal` with `sd` |
| limits ±a, every value equally probable | rectangular | a/√3 | `rectangular` |
| limits ±a, values near the centre more probable | triangular | a/√6 | `triangular` |
| limits ±a, flat top of half-width βa | trapezoidal | a√((1 + β²)/6) | `trapezoidal` (`beta`) |
| limits ±a known only to ±d | curvilinear trapezoid (JCGM 101 6.4.3) | √(a²/3 + d²/9) | `curvilinear_trapezoid` (`d`) |
| sinusoidal variation between ±a (e.g. cyclic temperature, RF mismatch) | arc sine (U-shaped) | a/√2 | `arcsine` |
| t-distributed knowledge (scale s, ν degrees of freedom) | scaled and shifted t | s (GUM framework) | `t` |
| non-negative quantity with known expectation only | exponential | equal to the expectation | `exponential` |
| non-negative quantity, shape α and scale θ | gamma | √α · θ | `gamma` |
| exactly known (defined constant) | — | 0 | `constant` |

Limits can be entered as `"lower"`/`"upper"` or as `"value"`/`"half_width"`.

**Reliability of a Type B uncertainty.** If the standard uncertainty itself is only known to a relative
precision Δu/u, assign the degrees of freedom ν ≈ ½ (Δu/u)⁻² (GUM G.4.2): 10 % → ν = 50, 25 % → ν = 8,
50 % → ν = 2. In a project file this is the field `"dof"` of the input.

## 3.3 Sensitivity coefficients

The sensitivity coefficient cᵢ = ∂f/∂Xᵢ, evaluated at the estimates, converts the uncertainty of an input into
a contribution to the uncertainty of the output (GUM 5.1.3). The program differentiates the model
**symbolically**, so the coefficients are exact (spreadsheet methods such as Kragten's use finite differences,
which differ slightly for non-linear models; see the notes of QUAM A4 and A5). The symbolic expression of each
coefficient is included in the JSON results.

**Kragten view.** Many published examples (EURACHEM/CITAC QUAM in particular) were computed with Kragten's
spreadsheet method (QUAM Appendix E.2), which replaces the derivative by the finite difference
Δᵢ = f(…, xᵢ + u(xᵢ), …) − f(x). With `"gum": { "kragten": true }` the program adds this column to the budget
and reports the Kragten u_c next to the exact first-order value. For the QUAM spreadsheet examples (A1–A5) it
reproduces the printed values to all printed digits, which shows that the small differences between those values
and the exact law of propagation come only from the finite differences.

For a pure product or quotient Y = c · X₁^{p₁} X₂^{p₂} ⋯ the relative uncertainties combine directly:
(u(y)/y)² = Σ pᵢ² (u(xᵢ)/xᵢ)² (GUM 5.1.6).

## 3.4 The law of propagation of uncertainty

For a model that is sufficiently linear over the range of the input uncertainties (GUM 5.1.2, 5.2.2):

$$u_c^2(y) = \sum_{i=1}^{N} c_i^2\,u^2(x_i) + 2\sum_{i=1}^{N-1}\sum_{j=i+1}^{N} c_i c_j\, u(x_i)\,u(x_j)\, r(x_i,x_j).$$

The **uncertainty budget** lists for each input the contribution uᵢ(y) = cᵢ u(xᵢ) and its share of the
combined variance, uᵢ²(y)/u_c²(y). The largest shares show where an improvement of the measurement is worth the
effort.

**Correlations.** Inputs that depend on common effects (the same balance, the same reference standard, a
common temperature) are correlated. Enter the correlation coefficients in `"correlations"`. Often a correlation
is better removed by modelling the common effect explicitly as an input quantity of its own — the IDMS example
(QUAM A7) does this with the mass-bias factors.

## 3.5 Non-linear models and second-order terms

When the model is significantly non-linear, the first-order law of propagation can be inadequate. The GUM
(note to 5.1.2) gives the most important second-order terms for independent, normally distributed inputs:

$$u_c^2(y) = \sum_i c_i^2 u^2(x_i) + \sum_i\sum_j \left[\tfrac12\left(\frac{\partial^2 f}{\partial x_i\,\partial x_j}\right)^2 + \frac{\partial f}{\partial x_i}\frac{\partial^3 f}{\partial x_i\,\partial x_j^2}\right] u^2(x_i)\,u^2(x_j).$$

These terms matter in particular when a **sensitivity coefficient is zero** at the estimates although the input
has an uncertainty — typically a product of two quantities whose expectations are zero, such as a temperature
deviation and a difference of expansion coefficients (GUM H.1, EA-4/02 S4 and S13, JCGM 101 9.3). The
first-order result then ignores the contribution entirely. **The program detects this case:** when an uncertain
input has a zero sensitivity coefficient but contributes through second-order terms, the report warns and states
the share of the variance the first-order evaluation misses (12 % in GUM H.1). Enable the terms with
`"gum": { "higher_order": true }`; the program then warns that the note assumes Gaussian inputs. In such cases the
Monte Carlo method (chapter 4) is the reliable check.

**The estimate.** Amendment 1 of the GUM (JCGM 100:2008/Amd.1:2026) adds note 1 to 4.1.4: when the nonlinearity is
significant, the second-order term of the Taylor series must also be added to the estimate,

$$y = f(x_1, \dots, x_N) + \tfrac12\sum_i\sum_j \frac{\partial^2 f}{\partial x_i\,\partial x_j}\,u(x_i, x_j),$$

which is GUM Equation (H.10); for independent inputs only the terms ∂²f/∂xᵢ² u²(xᵢ) remain. The program always
evaluates this term (`second_order_estimate_term` in the JSON results). With `"higher_order": true` it adds the term
to y and centres the interval on the corrected estimate. Without it, the report warns when the term exceeds a tenth
of u_c(y). For the comparison loss δY = X₁² + X₂² of JCGM 101 9.4 the corrected estimate equals the analytic mean
(50·10⁻⁶ at x₁ = 0), where the first-order estimate 0 is invalid (JCGM 101 9.4.2.2.8). In GUM H.1 the term is zero,
as Amd.1 states.

**Correlated inputs.** The GUM gives the higher-order variance terms for independent inputs only, and GUM-5
Example 16 therefore does not apply them to correlated reflection coefficients. For normally distributed inputs with
covariance matrix Σ the same Taylor expansion gives

$$u_c^2(y) = \mathbf c^\top\Sigma\,\mathbf c + \tfrac12\,\mathrm{tr}\big((H\Sigma)^2\big) + \sum_{i,j,k,l} c_l\,f_{ijk}\,\Sigma_{li}\,\Sigma_{jk}$$

(Mana & Pennecchi, *Metrologia* 44 (2007) 246), with H the matrix of second derivatives and f_ijk the third
derivatives. The program uses it whenever `"higher_order": true`; for independent inputs it reduces to the 5.1.2
note. For the comparison loss with r = 0.9 it gives u = 67, 121 and 505·10⁻⁶ at x₁ = 0, 0.010 and 0.050, the
analytic values of GUM-5 Table 16.2.

## 3.6 Degrees of freedom and the Welch–Satterthwaite formula

The **effective degrees of freedom** of the combined standard uncertainty (GUM G.4.1, Eq. G.2b):

$$\nu_\text{eff} = \frac{u_c^4(y)}{\sum_i u_i^4(y)/\nu_i}.$$

Inputs with ν = ∞ (most Type B evaluations) contribute nothing to the denominator. By default the program
truncates ν_eff to the next lower integer before looking up the t-distribution, as the GUM recommends (G.4.1
note); `"dof_rounding": "exact"` uses the fractional value.

## 3.7 Coverage factor and expanded uncertainty

The **expanded uncertainty** U = k u_c(y) defines an interval y ± U expected to contain a large fraction p of
the values that could reasonably be attributed to the measurand (GUM 6.2, 6.3).

* With `"coverage_probability": 0.95` (the default) k = t_p(ν_eff), the two-sided Student t quantile
  (GUM G.3, G.4). For ν_eff = ∞, k = 1.96.
* EA-4/02 uses p = 95.45 %, for which t(∞) = 2 exactly; its Table E.1 is reproduced by
  `"coverage_probability": 0.9545`.
* With `"coverage_factor": 2` the factor is fixed and the program states the corresponding coverage
  probability under the normal assumption (95.45 %).

**When the normal assumption fails.** If one non-Gaussian component dominates, the output distribution is far
from normal and k = 2 can be badly wrong. EA-4/02 (S9, S10) gives the rule for one or two dominant rectangular
contributions (trapezoidal output, k = 1.65 for one rectangular term, 1.83 for β = 0.33), but the general and
more reliable remedy is the Monte Carlo method, which gives the coverage interval directly.

UKAS M3003 (Appendix C) treats the common case of one dominant rectangular term, such as the resolution of a
digital indicator, with the other terms taken as normal. With `"dominant_rectangular": true` the program finds
the largest rectangular contribution u_R, sets u_N = √(u_c² − u_R²) and solves P(|R + N| ≤ k u_c) = p for the
convolution of the two distributions. This reproduces the table of M3003 C.7 for p = 95.45 %: k = 1.65 when
u_N = 0, 1.77 for u_N/u_R = 0.35, 1.90 for 0.70, and 2.00 when the rectangular term no longer dominates. The
report names the input and the ratio. In the example of M3003 C.5 (library project
`m3003-c5-dvm-dominant-rectangular`), the Monte Carlo interval agrees with the resulting y ± U.

## 3.8 Reading the budget

* A **share of variance** close to 100 % for one input means that the result is as good as that input; the
  others are irrelevant. Improving a 2 % contribution changes nothing.
* A **zero sensitivity coefficient** for an uncertain input is a warning sign: check whether second-order
  terms or the Monte Carlo method are needed (3.5).
* **Contributions with a sign** (cᵢu(xᵢ) < 0) matter only through correlations; their squares add.
* **Effective degrees of freedom** below about 10 increase the coverage factor noticeably; more observations
  of the dominant Type A input reduce it.

Examples to study: QUAM A1–A3 (products and quotients), A7 (a large model with 33 inputs), GUM H.1
(degrees of freedom and 99 % coverage), EA-4/02 S4 and S13 (second-order terms), S12 (Welch–Satterthwaite).
