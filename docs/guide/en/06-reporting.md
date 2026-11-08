# 6. Reporting

## 6.1 Rounding

The GUM (7.2.6) recommends giving the expanded uncertainty with at most two significant digits and the result
to the same decimal place. The `"reporting"` section of a project controls this:

| Field | Meaning |
|---|---|
| `"significant_digits"` | significant digits of U (default 2) |
| `"decimal_places"` | round U to a fixed number of decimal places instead (e.g. 0 for whole per cent) |
| `"rounding"` | `ea4_02` (default): ordinary rounding of U, but upwards when ordinary rounding would lower U by more than 5 % (EA-4/02 M:2022, 6.3); `half_even`: a discarded 5 rounds to even, later digits ignored (MUPR §12; can lower U by up to about 5.7 %, e.g. 0.10599 → 0.10); `half_up`; `up`: always round U up (conservative, GUM 7.2.6) |

The measured value is rounded to the decimal place of the rounded U. The statement of the result names the rule
used, and gives the coverage probability as "approximately 95 %" (whole per cent; 0.1 % above 99.5 %), the form of
EA-4/02 and ILAC P14, also for k = 2 with ν_eff = ∞ (95.45 %). The default changed from `half_even` to `ea4_02` after
the independent pre-review of 2026-11-08; projects that follow the Eurachem/CITAC MUPR rule set `"half_even"`. The full-precision values remain
available in the budget and in the JSON results; never round intermediate values before the end of a
calculation — several published examples change in the last digit because they did.

## 6.2 Statement of the result

The program writes the result in the form recommended by EA-4/02 (clause 6):

> c_Cd = (1002.7 ± 1.7) mg/L, where the expanded uncertainty is the combined standard uncertainty multiplied by
> the coverage factor k = 2.00, corresponding to a coverage probability of approximately 95.45 %.

When the Monte Carlo method is used the coverage interval itself is the result; it is stated with its coverage
probability and the kind of interval (shortest or probabilistically symmetric).

**Calibration certificates (ILAC P14:09/2020).**

- The result is stated as y ± U with k and the coverage probability, as above (5.2).
- U has at most two significant digits (5.3). The program warns in the statement when
  `reporting.significant_digits` is larger.
- An accredited laboratory shall not report an uncertainty smaller than its CMC (5.5). With
  `"reporting": { "cmc": { "expanded": … } }` (or `"relative_percent"`) the program reports max(U, CMC) and says
  so in the statement. The library project `ilac-p14-cmc-floor-weight-10kg` shows this.

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

## 6.3b Results near a physical bound

A mass fraction cannot exceed 1 g/g, a concentration cannot be negative. When y ± U crosses such a bound,
`"lower_bound"` and/or `"upper_bound"` in the `"output"` of the model state it. The program then

- truncates the GUM interval at the bound and states the result asymmetrically, e.g. w = (0.9998 +0.0002/−0.0015)
  g/g: "a w ± U95 % symmetric interval can be treated as specifying the asymmetric interval w − U95 % ≤ w ≤ 1 g/g"
  (Westwood et al. 2023, IUPAC Technical Report on SI-traceable purity, 4.1.1; the BIPM ISRD reports do so);
- if the Monte Carlo method runs, also reports the result restricted to the bounds: the draws outside are
  discarded, which corresponds to a rectangular prior on the measurand over the bounds (Toman et al. 2016;
  Westwood et al. 2023, 4.2.5). It gives the fraction of draws inside, their mean and standard deviation and the
  coverage interval; the estimate moves away from the bound and the interval is usually narrower than the
  truncated GUM interval.

If the estimate itself lies outside the bounds, the statement says so and the restricted Monte Carlo result
should be reported. Library project: `bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od` (999.8 +0.2/−1.5 mg/g, as printed).

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
