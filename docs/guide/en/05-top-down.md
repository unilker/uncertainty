# 5. Top-down evaluation

In routine testing a complete measurement model is often impractical: sampling, extraction, matrix effects and
day-to-day variation are difficult to model input by input. The **top-down** approach estimates the
uncertainty from data the laboratory already has — method validation, internal quality control, reference
materials, proficiency tests and recovery experiments. Its two main components are the **within-laboratory
reproducibility** u(Rw), which captures random effects over time, and the uncertainty of the **bias**
(trueness), which captures systematic effects.

The program follows Nordtest TR 537 (ed. 4, 2017) and the Eurachem/CITAC guide *Evaluation of measurement
uncertainty from in-house precision and recovery data* (MUPR, 2026). Both reduce to the law of propagation of
uncertainty for a model of the form y = x · (1 + δ_bias) · (1 + δ_precision) ⋯, so absolute and relative
components combine as

$$u_c^2(y) = \sum_k m_k\,u_{\text{abs},k}^2 + y^2 \sum_k m_k\,u'^2_{\text{rel},k},$$

with m_k the number of times a component enters (for example 2 for a stock solution used both for the
calibrators and for the spike, MUPR B1).

**Convention in project files:** relative quantities are always entered and reported in per cent, in fields
whose names end in `_percent`; every other value is in the unit of the measurand.

## 5.1 Within-laboratory reproducibility u(Rw)

The precision component should include all steps and all variations of routine work over a long period
(different days, analysts, calibrations, reagent lots). Sources, in a `"precision"` list:

| Data | `type` | Estimate |
|---|---|---|
| results of a control sample over time (control chart) | `series` | s, ν = n − 1; with `"relative": true` s/x̄ |
| several materials or periods, raw values or (s, n) | `groups` | pooled s_p = √(Σ(nᵢ − 1)sᵢ²/Σ(nᵢ − 1)) (MUPR Eq. 1–2) |
| duplicates of real samples | `duplicates` | `pooled`: s = √(Σd²/2N) (Nordtest App. 5–6); relative: pooled relative s of the pairs; `normalised_differences`: s of (x₁ − x₂)/x̄ divided by √2 (QUAM A4); `mean_range`: mean range / 1.128 (MUPR §8.1.3) |
| control-chart limits ±L at k standard deviations | `control_limits` | L/k (Nordtest §5.1) |
| a value stated elsewhere | `given` | s (and optionally ν) |
| s depends linearly on the level | `linear` | s(c) = a + b·c, stated or fitted to (c, s) points (MUPR B3) |
| s(c) = √(s₀² + (c s′₁)²) | `continuous` | MUPR Eq. (A.1) |

**Control samples and real samples.** A stable control sample does not show the variation between different
real samples (inhomogeneity, matrix). Nordtest therefore adds the repeatability from duplicates of real
samples: u(Rw) = √(u(Rw, control)² + s_r(duplicates)²); MUPR B6 does the same with the ranges of duplicates of
real samples.

**Precision over the working range.** Precision is usually constant in absolute terms near the limit of
quantification and constant in relative terms at higher concentrations. Restrict components with `"range"`
(min inclusive, max exclusive), for example an absolute s below 30 µg/L and a relative s above (Nordtest §5.3),
or the two intervals of MUPR with the transition c_T = s⟨L⟩/s′⟨H⟩ (MUPR Appendix A). Duplicate pairs can be
selected by their mean with `"min_mean"`/`"max_mean"`.

**Conversions.** `"to_relative_at"` expresses an absolute s relative to a value (or to `"mean"`, the mean of
the data); `"to_absolute_at"` expresses a relative s as an absolute one at a value — e.g. MUPR B4 uses
4.52 % × 6.46 mg/kg as the absolute precision of the low interval.

**Means of replicates.** If the reported result is the mean of n results on each of p days, its precision is
s_I(x̿) = √(s_I²/p + s_r²(1 − n)/(n p)) (MUPR Eq. 5): `"mean_of": { "replicates": n, "days": p,
"repeatability_percent": … }`.

## 5.2 Bias u(bias) (Nordtest §6)

The uncertainty of the bias combines the observed biases with the uncertainty of the reference values. In a
`"bias"` entry with `"kind"` `crm`, `proficiency_tests` or `recovery`, each reference gives a bias bᵢ and a
reference uncertainty u(C_ref,i):

* **One CRM measured n times** (mean x̄, standard deviation s): b = (x̄ − C)/C,

  $$u(\text{bias}) = \sqrt{b^2 + \left(\frac{s}{\sqrt n}\right)^2 + u^2(C_\text{ref})}.$$

* **Several CRMs, proficiency tests or recovery experiments:**

  $$u(\text{bias}) = \sqrt{\text{RMS}_\text{bias}^2 + \bar u^2(C_\text{ref})},\qquad \text{RMS}_\text{bias} = \sqrt{\frac{\sum b_i^2}{N}},$$

  with ū(C_ref) the mean of the reference uncertainties. For a proficiency test u(C_ref,i) = s_R/√n_Lab
  (`"sd_reproducibility_percent"` and `"labs"`); a robust standard deviation reported by the provider is
  multiplied by 1.25 first (Nordtest §6.2 note). For recovery experiments bᵢ = Rᵢ − 100 % and u(C_ref) is the
  uncertainty of the added amount.

Each reference can be given with raw results (`"measured"`), with mean, s and n, or with a stated bias. At
least six proficiency tests are recommended for a reliable estimate (Nordtest §6.2).

The combined uncertainty is then u_c = √(u(Rw)² + u(bias)²), and Nordtest reports U = 2u_c rounded up to a
whole per cent (`"reporting": { "decimal_places": 0, "rounding": "up" }`).

## 5.3 Recovery (MUPR §8.2)

MUPR treats trueness through the **mean recovery** R̄ from reference materials, spiked blanks or spiked
samples (a `"recovery"` section):

* reference material or spiked blank: Rᵢ = c̄ᵢ/Cᵢ, u²(Rᵢ) = Rᵢ²[(sᵢ/(c̄ᵢ√nᵢ))² + (u(Cᵢ)/Cᵢ)²] (Eq. 8–9);
* spiked sample with native analyte: Rᵢ = (c̄ − c̄₀)/c₊, with the uncertainties of both means and of the
  spike (Eq. 10–11);
* R̄ = ΣRᵢ/N and u(R̄) = √(Σu²(Rᵢ))/N (Eq. 12).

**Significance test (Eq. 13).** If |1 − R̄|/u(R̄) is larger than the two-sided t value for the chosen
confidence (default 95 %), the recovery differs significantly from 1 and results are corrected, c_corr = c/R̄,
with the relative component u(R̄)/R̄; otherwise results are not corrected and the component is u(R̄). The rule
can be forced with `"correction": "always"` or `"never"`. For a corrected result absolute components are divided
by R̄ as well, which is the correct propagation for y = x/R̄.

**Matrix-dependent recoveries.** When recoveries vary between matrices, their standard deviation s(R) is added:
u = √(u²(R̄) + s²(R)) (`"include_sd_of_recoveries": true`, MUPR B5); the significance test then has N − 1
degrees of freedom. The χ² test of Eq. (16) shows whether the individual recoveries are equivalent
(MUPR B6: χ² = 8.70 ≤ 32.7, so s(R) need not be added).

### Precision from collaborative studies (ISO 21748, ISO 5725-3)

* `reproducibility` — a standard method with collaborative-study s_R and s_r (ISO 21748:2017). The precision
  component is √(s_L² + s²/n_r) with s_L² = s_R² − s_r² (or `sd_between_labs`), s = s_r or the laboratory's own
  repeatability s_l if it has been verified (7.3.2), and n_r the number of complete replicates averaged
  (Table 1). `method_bias` adds the uncertainty of a method bias estimated in the study,
  u²(δ̂) = (s_R² − (1 − 1/n)s_r²)/p + u²(μ̂) (Formula 15). `bias_check` reports whether the laboratory bias on a
  reference material satisfies |Δ_l| < 2s_D, s_D² = s_L² + s_w²/n_l (Formulae 4–5). Effects not covered by the
  study (sampling, pre-treatment, drying) are added as further components (Clause 8–9).
* `staggered_nested` — rows of two results on day 1 and one on day 2 (ISO 5725-3:2023, 7.2 and Annex C.1). The
  analysis of variance gives s_r, the time-different intermediate precision s_I(T) and, between groups, s_R;
  `estimate` selects which is used (default `intermediate`).

## 5.4 Additional components

Components not covered by the data — a stock solution, a dilution, inhomogeneity of the sample, drying — are
entered in `"components"` as a standard uncertainty (`"u"`, `"u_percent"`) or as limits (`"half_width"`,
`"half_width_percent"`, rectangular or triangular), optionally with `"multiplicity"` and `"range"`.

## 5.5 Where to evaluate

List the measured values in `"levels"`; the budget, the result (corrected for recovery where applicable) and
the variance shares are given at each. Without levels every component must be relative and the result is a
relative expanded uncertainty.

Examples to study: Nordtest §3 (the complete procedure), §5.3 (duplicates in two ranges), §6.1–6.3 (the three
ways of estimating bias), §8 (four combinations of control chart, CRM and PT); MUPR B1–B6 (recovery,
precision models, matrix effects, 22 proficiency tests); QUAM A4 (duplicates and recovery) and A6
(reproducibility of an empirical method).
