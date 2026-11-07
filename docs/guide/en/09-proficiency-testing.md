# 9. Proficiency testing (ISO 13528)

A `proficiency` project holds one round of a proficiency testing scheme for one measurand: the participants'
results (with their uncertainties if reported), the way the assigned value and the standard deviation for
proficiency assessment are obtained, and optionally the homogeneity and stability checks of the proficiency
test items. The program implements the statistical methods of ISO 13528:2022 and is validated against its
worked examples E.2, E.3 and E.4 (library projects `iso13528-*`).

## 9.1 Summary statistics (Annex C)

For the p numerical results (censored results such as "< 0.015" are listed but not used) the program gives
the median, the scaled median absolute deviation MADe = 1.483 med|xᵢ − med(x)|, the normalised interquartile
range nIQR = 0.7413 (Q₃ − Q₁), the arithmetic mean and standard deviation, and the robust average x* and
robust standard deviation s* of **Algorithm A** (C.3.1).

Algorithm A starts from x* = median and s* = MADe; each result outside x* ± 1.5s* is replaced by the nearer
limit, and x* and s* are recomputed as the mean and 1.134 × the standard deviation of the modified values. The
iteration stops when the third significant figures of x* and s* no longer change (C.3.1); `"algorithm_a":
"converged"` iterates to full convergence instead. If MADe is zero, the initial s* is the sample standard
deviation (C.3.1, Note 2).

Quartiles are computed by linear interpolation between order statistics, which reproduces the nIQR of
ISO 13528 Table E.5; other software may use other definitions (Table E.5, note). Algorithm S (C.4), the robust
pooling of standard deviations, is available in the engine (`pt::algorithm_s`).

## 9.2 Assigned value and its uncertainty (Clause 7)

| `assigned_value.method` | x_pt | u(x_pt) |
|---|---|---|
| `algorithm_a` | x* | 1.25 s*/√p, Formula (6) |
| `median` | median | 1.25 nIQR/√p (or MADe) |
| `mean` | arithmetic mean | s/√p |
| `reference` | value from formulation, a CRM or a reference laboratory (7.3–7.6) | as stated |

Uncertainty components for inhomogeneity, transport and instability can be added (Formula 3). For a consensus
value they are normally already reflected in the spread of the results (7.2.2, Note 2). The factor 1.25 is
conservative; it can be changed with `u_factor`.

The program states whether u(x_pt) < 0.3 σ_pt (Formula 10). If not, the uncertainty of the assigned value is
not negligible and z′, ζ or E_n scores should be used (9.2.2).

## 9.3 Standard deviation for proficiency assessment (Clause 8)

σ_pt is either given (fitness for purpose, experience from previous rounds or a general model such as the
Horwitz equation) or taken from the results of the round (`robust`, `made`, `niqr`, `sd`). Without σ_pt no z
or z′ scores are calculated.

## 9.4 Performance statistics (Clause 9)

| Score | Formula | Interpretation |
|---|---|---|
| D, D % | x − x_pt, 100 (x − x_pt)/x_pt | compared with δ_E |
| PA | 100 D/δ_E (δ_E = 3σ_pt by default) | ≥ 100 % or ≤ −100 %: action |
| z | (x − x_pt)/σ_pt | \|z\| ≤ 2 acceptable, 2 < \|z\| < 3 warning, \|z\| ≥ 3 action |
| z′ | (x − x_pt)/√(σ_pt² + u²(x_pt)) | as z |
| ζ | (x − x_pt)/√(u²(x) + u²(x_pt)) | as z; needs the participant's u(x) |
| E_n | (x − x_pt)/√(U²(x) + U²(x_pt)) | \|E_n\| < 1 satisfactory |

The participant's standard uncertainty is its expanded uncertainty divided by its coverage factor. With
`uncertainty_screening` the reported uncertainties are flagged a (plausible), b (below u_min) or c (above
u_max); ISO 13528 9.8 suggests u_min = u(x_pt) and u_max = 1.5 s*.

ζ and E_n scores, and z′ with a consensus value, ignore the correlation between a result and an assigned value
computed from it; ISO 13528 (9.5.1, 9.6.1) accepts this approximation.

## 9.5 Homogeneity and stability (Annex B)

For g items measured m times each the analysis of variance of B.3 gives the standard deviation of the item
averages s_x, the within-sample standard deviation s_w and the between-sample standard deviation
s_s = √(s_x² − s_w²/m) (zero if negative). The items are sufficiently homogeneous if s_s ≤ 0.3 σ_pt (B.1). The
expanded criterion √(F₁(0.3σ_pt)² + F₂s_w²) of B.2.3 is also given, with F₁ = χ²₀.₉₅(g − 1)/(g − 1) and
F₂ = (F₀.₉₅(g − 1, g(m − 1)) − 1)/m computed exactly (Table B.1 for m = 2).

The mean squares MS_between = m·s_x² and MS_within = s_w² are listed. So is u*_bb = √(MS_within/m)·(2/ν_within)^¼,
the between-unit standard deviation that the repeatability of the method can hide (Linsinger et al. 2001; ISO
Guide 35). It is the usual value for the uncertainty of a reference material when MS_between < MS_within, where
s_s is zero. Without σ_pt, for the homogeneity study of a reference material, the ISO 13528 criteria are not
evaluated.

**Bayesian between-unit standard deviation.** With `"bayes": { "tau_scale", "sigma_scale", "mu_mean", "mu_sd" }`
the program also evaluates the hierarchical model y_ij = µ + B_i + ε_ij, B_i ~ N(0, τ²), ε_ij ~ N(0, σ²). τ and σ
have half-Cauchy priors with the given scales, typically the specification of the preparation and the known
repeatability. µ has a normal prior, or a flat one if `mu_mean` and `mu_sd` are omitted. The unit effects and µ
are integrated out exactly, and the posterior of (τ, σ) is evaluated on a grid. The result is therefore the
same at every run, without MCMC. The report gives the mean, standard deviation, median and mode of τ and σ,
and their symmetric and shortest 95 % intervals (JCGM GUM-5 Example 12). The posterior of τ is sensitive to
its prior scale (GUM-5 12.4.6), so the scale must be justified.

The stability check compares the averages before and after: |ȳ₁ − ȳ₂| ≤ 0.3 σ_pt (B.17), widened by
2√(u²(ȳ₁) + u²(ȳ₂)) when the uncertainties of the averages are given (B.18).

## 9.6 In the application

*New proficiency test round* starts from a template. On the *Data* tab the results are pasted from a
spreadsheet, one laboratory per line: code, result, expanded uncertainty, coverage factor and, optionally, the
method (tab, semicolon or space separated; decimal comma accepted; "<0.015" for a censored result). The
*Results* tab shows the summary statistics, the assigned value, the table of scores with their signals and a
chart of the z scores; the HTML report adds the homogeneity chart.
