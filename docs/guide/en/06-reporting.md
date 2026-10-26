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

## 6.3 Contents of a report

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
