# 10. Calibration with uncertainties in both variables (ISO 6143)

A `calibration` project determines an analysis function x = G(y) from reference materials whose contents x
**and** responses y are uncertain, and uses it to assign contents, with uncertainties and covariances, to
samples measured on the same instrument. The program implements the generalized least-squares method of
ISO 6143:2025 (gas analysis, but the method is general) and is validated against its worked examples 1–3
(library projects `iso6143-*`). If only the responses are uncertain, give the contents very small
uncertainties: the curve is then forced through the xᵢ and the fit becomes weighted least squares in y.

## 10.1 Analysis functions (6.2.1)

| `function` | G(y; b) | parameters |
|---|---|---:|
| `linear` | b₀ + b₁y | 2 |
| `quadratic` | b₀ + b₁y + b₂y² | 3 |
| `cubic` | b₀ + b₁y + b₂y² + b₃y³ | 4 |
| `power` | b₀ + b₁y^(1 + b₂) (y > 0) | 3 |
| `exponential` | b₀ + b₁e^(b₂y) | 3 |

The analysis function gives the content from the response; it is the inverse of the calibration function.

## 10.2 The fit (Annex A)

The parameters b and the adjusted responses ŷᵢ minimise

S = Σᵢ [(G(ŷᵢ; b) − xᵢ)²/u²(xᵢ) + (ŷᵢ − yᵢ)²/u²(yᵢ)],

so the adjusted points (x̂ᵢ, ŷᵢ) = (G(ŷᵢ), ŷᵢ) lie on the curve. The minimisation uses a Levenberg–Marquardt
iteration from a weighted least-squares start. The covariance matrix of b propagates the uncertainties of all
calibration data, including covariances u(xᵢ, xₕ) of contents that share their history (a dilution series,
a common set of primary standards; Formula A.7). The derivatives ∂b/∂xᵢ and ∂b/∂yᵢ that ISO 6143 A.2 obtains
by numerical differentiation of the fit are computed exactly by the implicit-function theorem at the minimum.
This removes the dependence on step size and stopping rule that the standard warns about in B.1 (research
log D33).

## 10.3 Validation of the function (6.2.2, 6.2.3)

- **Weighted deviations** (xᵢ − x̂ᵢ)/u(xᵢ) and (yᵢ − ŷᵢ)/u(yᵢ) for each point; the **goodness of fit** Γ is the
  largest of them in absolute value. A function is admissible if Γ ≤ 2, i.e. the curve passes through every
  calibration rectangle xᵢ ± 2u(xᵢ), yᵢ ± 2u(yᵢ) (Formula 9).
- The residual sum S_res is reported against the acceptance limit 2(n − N − 1), where N + 1 is the number of
  parameters.
- With `compare` (the default) every other analysis function with fewer parameters than points is also
  fitted. Its Γ and S_res are listed so that the choice of 6.2.2 can be made: a physical model first;
  otherwise the simplest function among those with a similar fit; otherwise the best fit. A visual check of
  the curve stays necessary.
- **Upper bound of the uncertainty** (6.2.3): u(x) at the responses of the lowest and highest standards. If
  `target_uncertainty` is given, it is compared with the larger of the two.

## 10.4 Contents of samples (6.3)

For each sample with response y and u(y):

u²(x) = (∂G/∂y)² u²(y) + Σⱼ Σₗ (∂G/∂bⱼ)(∂G/∂bₗ) u(bⱼ, bₗ).

The expanded uncertainty uses `coverage_factor` (default 2). Results of several samples from the same
calibration are correlated through b. The report gives their correlation coefficients, which are needed when
the mixtures are later used together as a set of standards. Samples whose response lies outside the
calibration range are flagged.

## 10.5 Drift control and mismatching mixtures (6.2.4, 6.2.5)

`drift` compares the means of n measurements of a bracketing reference mixture before and after the samples
with its calibration response y(calib) and with each other. The critical values are 2u√(1 + n_calib/n) for
the two differences from y(calib) and 2u√(2n_calib/n) for the difference between them, where u = u(y(calib))
and n = n_calib = 10 by default. These give 2√2 u for n = 10, as stated in the Note to 6.2.4. Formula (12) of
the standard reads 2u√(1 + 20/n), which gives 2√3 u for n = 10 and contradicts that Note. The program uses
the derivation (research log F41).

`validation` lists reference mixtures whose matrix differs from that of the standards. Each passes if
|x_obs − x_ref| ≤ 2√(u²(x_obs) + u²(x_ref)) (Formula 13).

## 10.6 A remark on the exponential function

ISO 6143 B.1 notes that the reference software B_LEAST gives wrong parameter uncertainties for the
exponential function. The values printed in Example 3 are about 39 times too small. This program, ODRPACK
and a Monte Carlo simulation of the whole calibration agree with each other. The content assigned with the
exponential function in Table B.12 has a printed u(x) that is 6 % to 7 % too low (research log F42).
For strongly non-linear functions, first-order propagation is itself an approximation (B.2.4, final remark);
library project `iso6143-b3-no-ndir-exponential` documents the Monte Carlo check.

## 10.7 In the application

*New calibration (both axes)* starts from a template. On the *Data* tab the calibration points are pasted
one per line (name, x, u(x), y, u(y)), followed by the analysis function, the samples (name, y, u(y)) and
optional covariances (point, point, value). *Results* shows the parameters, Γ and S_res, the comparison of
functions, the contents of the samples, the upper bound and a chart of the weighted deviations with the
limits ±2.
