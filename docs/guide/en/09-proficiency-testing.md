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

## 9.7 Consensus value of an interlaboratory comparison

A key comparison, an interlaboratory study for the characterisation of a reference material or a
meta-analysis of published results gives n values x_j with standard uncertainties u_j. When the values are more
dispersed than the u_j allow, the weighted mean with weights 1/u_j² and its uncertainty 1/√Σu_j⁻² are too
optimistic. The `consensus` section (7.14b) follows the NIST Consensus Builder (NICOB, user manual 2020,
Appendix A; Koepke et al., Metrologia 54 (2017) S34) and uses the random-effects model
x_j = μ + λ_j + ε_j: the laboratory effects λ_j have mean 0 and standard deviation τ, the *dark uncertainty*.

**DerSimonian–Laird.** Cochran's Q = Σu_j⁻²(x_j − x̄_w)² gives τ̂² = max{0, (Q − n + 1)/(Σu_j⁻² − Σu_j⁻⁴/Σu_j⁻²)};
the consensus value is the weighted mean with weights w_j = 1/(τ̂² + u_j²), with the naive u_DL = 1/√Σw_j. The
report gives the p-value of Q (χ² with n − 1 degrees of freedom) and the Knapp–Hartung standard uncertainty
u_DL·√(Σw_j(x_j − μ̂)²/(n − 1)), expanded with t_{n−1}. When τ̂ = 0 the procedure is the weighted mean
(Procedure A of Cox 2002).

**Parametric bootstrap (NICOB §A.2).** u_DL ignores the uncertainty of τ̂, which rests on few degrees of
freedom. In each of K replicates τ_k is drawn from the gamma approximation of the distribution of Q
(Biggerstaff and Tweedie 1997: the gamma with the exact mean and variance of Q), x_jk from N(μ̂, τ_k² + u_j²) and,
when ν_j is given, u_jk = u_j√(ν_j/χ²_ν_j); the DerSimonian–Laird μ_k is recomputed. The standard deviation and
the 2.5 % and 97.5 % percentiles of the μ_k give u and the coverage interval.

**Degrees of equivalence (NICOB §A.5.1).** D_j = x_j − μ̂ with U95 %(D_j), the half-length of the shortest
interval centred at the mean of the bootstrap values D_jk = x_jk − μ_k that contains 95 % of them. A result with
|D_j| > U95 %(D_j) lies beyond the range allowed by the model (Jones and Spiegelhalter 2011). The HTML report
draws them.

**Linear Pool (NICOB §A.4).** A sample from the mixture of Gaussian (or, with ν_j, rescaled Student t)
distributions with means x_j and standard deviations u_j, with equal or given weights. Its mean and standard
deviation are the consensus value and its uncertainty; the interval is from percentiles. The mixture can be
multimodal; its uncertainty covers the whole spread of the results.

**Maximum likelihood (IUPAC 2024, Example 4F).** The likelihood of μ and τ under Gaussian effects and errors
with known u_j, maximised over τ with μ̂(τ) the weighted mean; standard uncertainties from the inverse Hessian.
They are too small with few results; the bootstrap is preferable.

**Bayesian random-effects model (NICOB §A.3; IUPAC 2024 Examples 4M and 4N).** Gaussian likelihood with
σ_j = u_j, a half-Cauchy prior on τ and a Gaussian or flat prior on μ. With `"bayes": {}` the NICOB defaults are
used: the median of the prior on τ is the median of |x_j − median(x)|, and μ is flat (NICOB: N(0, 10⁵)). μ is
integrated out analytically, and the posterior of τ is evaluated on a grid of 10⁵ points uniform in the prior
probability (τ = s·tan(πθ/2)). The result is exact and the same at every run; NICOB and the guide use MCMC. The
report gives the posterior mean and standard deviation of μ, its quantile interval, and the posterior median and
interval of τ, which is skewed to the right. NICOB's priors on the σ_j for finite ν_j are not implemented.

Library: Koepke et al. 2017 (G, half-lives of ¹³⁷Cs and ⁹⁰Sr) and IUPAC 2024 Examples 3E/4M/4N (iridium) and 4F
(CCQM-K145). Not yet implemented: leave-one-out and bilateral degrees of equivalence.
