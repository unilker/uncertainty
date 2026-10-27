# 6. Reporting

## 6.1 Rounding

The GUM (7.2.6) recommends giving the expanded uncertainty with at most two significant digits and the result
to the same decimal place. The `"reporting"` section of a project controls this:

| Field | Meaning |
|---|---|
| `"significant_digits"` | significant digits of U (default 2) |
| `"decimal_places"` | round U to a fixed number of decimal places instead (e.g. 0 for whole per cent) |
| `"rounding"` | `half_even` (default): a discarded 5 rounds to even, later digits ignored (MUPR §12); `half_up`; `up`: always round U up (conservative, GUM 7.2.6) |

The measured value is rounded to the decimal place of the rounded U. The full-precision values remain
available in the budget and in the JSON results; never round intermediate values before the end of a
calculation — several published examples change in the last digit because they did.

## 6.2 Statement of the result

The program writes the result in the form recommended by EA-4/02 (clause 6):

> c_Cd = (1002.7 ± 1.7) mg/L, where the expanded uncertainty is the combined standard uncertainty multiplied by
> the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

When the Monte Carlo method is used the coverage interval itself is the result; it is stated with its coverage
probability and the kind of interval (shortest or probabilistically symmetric).

## 6.3 Results not corrected for a known bias

The GUM assumes that results are corrected for all recognised significant systematic effects (GUM 3.2.4).
In routine testing a bias is often known or suspected but not corrected, for example because the correction
is not required by the method or the bias varies between matrices. Reporting the uncorrected result with its
ordinary uncertainty would then mislead. With `reporting.uncorrected_bias` (`bias` b, its standard
uncertainty `u_bias` and a `strategy`), model reports add a second statement whose interval includes the
bias. The strategies are those of Magnusson & Ellison (2008, Table 3). In the formulae Δ = −b and
u_c = √(u²(y) + u²(b)):

| `strategy` | Interval | Remark |
|---|---|---|
| `ue` (default) | y ± U, U the half-width that contains 2Φ(k) − 1 of N(y + Δ, u_c²) | Synek; smallest symmetric interval with correct coverage; ≈ 1.65u_c + \|Δ\| for \|Δ\| > 0.8u_c |
| `rssu` | y ± k√(u²(y) + Δ² + u²(b)) | Nordtest, NIST; conservative for large bias |
| `rssu_k` | y ± k√(u²(y) + (Δ/k)² + u²(b)) | IUPAC; coverage drops for large bias |
| `sumu` | [y − max(0, ku_c − Δ), y + max(0, ku_c + Δ)] | Phillips et al. (NIST, 1997); asymmetric |
| `sumu_max` | y ± (ku_c + \|Δ\|) | GUM F.2.4.5; symmetric, wide |

Example: `"reporting": { "uncorrected_bias": { "bias": 1.5, "u_bias": 0.3, "strategy": "sumu" } }` added to the
cadmium standard gives

> c_Cd = 1002.7 mg/L (−3.3, +0.3), not corrected for the estimated bias b = 1.50 mg/L (u(b) = 0.30 mg/L) …

The test suite reproduces the coverage simulation of Magnusson & Ellison (Fig. 1). SUMU covers about 95 % at
zero bias and 97.5 % at 3u_c. Ue covers about 98 % at zero bias and 95 % for larger bias. RSSu and SUMU_MAX
always cover more than 95 %. The interval widths of their Fig. 2 are reproduced as well. A correction with
its uncertainty, when it can be justified, remains the preferred treatment (GUM 6.3.1; research log F43
records a sign error in their Table 3).

## 6.4 Contents of a report

A report is meant to make the evaluation reproducible and reviewable (ISO/IEC 17025:2017, 7.6 and 7.8; GUM 7.2):

* the measurand and the model, or for top-down evaluations the data and formulas used;
* every input with its value, distribution and the source of the information;
* the uncertainty budget with sensitivity coefficients, contributions and variance shares;
* the coverage factor and how it was obtained (degrees of freedom, coverage probability);
* for the Monte Carlo method the number of trials, the seed and the validation of the GUM result;
* explanations, notes on the sources, and the references;
* the software version and the citation.

Reports are available as Markdown, self-contained HTML (charts as SVG; light and dark themes; printable) and
JSON (all values in full precision, for further processing). Turkish reports use the decimal comma and the
prefixed per-cent sign (%95).
