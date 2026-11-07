# 11. Conformity assessment (JCGM 106)

ISO/IEC 17025:2017 7.8.6 requires a laboratory that states conformity with a specification to document and
apply a *decision rule* that takes the measurement uncertainty into account. The `conformity` section
implements JCGM 106:2012 for this purpose. It is validated against all numerical examples of clauses 7–9 of
that guide (library projects `jcgm106-*`).

## 11.1 Probability of conformity (clause 7)

Specifications are given as a tolerance interval: a lower limit T_L, an upper limit T_U or both. For a result
y with standard uncertainty u and a normal PDF for the measurand, the probability of conformity is

p_c = Φ((T_U − y)/u) − Φ((T_L − y)/u)

(Formula 11; one-sided limits: Formulae 8 and 9). With `dof` (projects of their own) or
`"distribution": "t"` (model projects, with the effective degrees of freedom), the PDF is a scaled and shifted
t distribution (JCGM 101 6.4.9). In a model project with the Monte Carlo method, p_c is also given as the
fraction of the Monte Carlo values inside the tolerance interval. For a two-sided tolerance, the report gives
the measurement capability index C_m = (T_U − T_L)/(4u) (Formula 12).

## 11.2 Decision rules and guard bands (clause 8)

| `rule` | Acceptance limits | Use |
|---|---|---|
| `simple_acceptance` (default) | = tolerance limits | shared risk; requires an uncertainty judged acceptable (8.2) |
| `guarded_acceptance` | inside the tolerance limits by w | lower risk of accepting a non-conforming item (8.3.2) |
| `guarded_rejection` | outside the tolerance limits by w | clear evidence before a negative action (8.3.3) |

The guard band is set in one of these ways:
- `r`: w = r·U with U = 2u, default r = 1 (ISO 14253-1);
- `guard_band`: w given directly;
- `probability`: the acceptance limit is placed where the probability of conformity (guarded acceptance) or
  of non-conformity (guarded rejection) reaches the given value. With `u_relative` the uncertainty may depend
  on the measured value, as in the speed-limit example (8.3.3, Example 1).

For a positive measurand with a large relative uncertainty (above about 20 %), `"distribution": "lognormal"`
uses a lognormal PDF with median equal to the measured value and s_G = u/|y| (Eurachem/CITAC compliance guide
2021, Annex A, Case 4, Equation 2), or s_G = √ln(1 + (u/y)²) with `"exact_sdlog": true`. With a constant
relative uncertainty the acceptance limits are then exactly L·exp(±k·s_G). The guide's normal limits L(1 ± k·u_rel)
take u at the limit, i.e. a fixed `u` = u_rel·L. With `u_relative` and a normal PDF the program applies the
relative uncertainty at the measured value, which gives L/(1 ∓ k·u_rel).

Acceptance limits can also be given directly with `acceptance_lower` and `acceptance_upper`; the rule is then
inferred from them.

**Guard bands from the test uncertainty ratio.** Calibration practice (ANSI/NCSL Z540.3 and its handbook) sets
the acceptance limits from TUR = (T_U − T_L)/2 / U₉₅ with U₉₅ = 2u (equal to C_m of JCGM 106). With `tur_method`
the limits are ± GBF·(T_U − T_L)/2 about the centre: `rss` GBF = √(1 − 1/TUR²); `dobbert` (managed guard band,
"method 6") GBF = 1 − M/TUR with M = 1.04 − exp(0.38 ln TUR − 0.54), which keeps the global PFA at or below about 2 %
for any in-tolerance probability (GBF > 1 above TUR ≈ 4.6: limits outside the tolerance); `rp10` (NCSLI RP-10)
GBF = 1.25 − 1/TUR; `test95` GBF = 1 − 1/TUR (both 1 above TUR = 4). A two-sided tolerance and a fixed u are
needed. The factors and the global risks agree with Sandia's Suncal (library projects `tur-guard-band-*`). A
guard band for a required global PFA is `target_consumer_risk` (11.3).

For each result the report gives p_c, the decision and the specific risk. If the item is accepted, this is
the consumer's risk 1 − p_c; if it is rejected, it is the producer's risk p_c (9.3.2).

## 11.3 Global risks of a production process (clause 9)

With a `process` (a `normal` or `gamma` PDF with `mean` and `sd`, or a `lognormal` PDF with `meanlog` and
`sdlog` of ln Y) and the `u` of the measuring system, the program integrates Formulae (17)–(20) numerically.
With `u_relative` the standard deviation of the measuring-system PDF is u_rel·η, proportional to the true value. It reports:
- the fraction of conforming items;
- the fraction of accepted items;
- the global consumer's risk R_C (accepted although non-conforming);
- the global producer's risk R_P (rejected although conforming);
- the same results per 100 items.

With `target_consumer_risk` it finds the guard-band multiplier r that gives the required R_C (9.5.4). A gamma
PDF suits properties that cannot be negative, such as the radial error motion of the ball-bearing example
(Annex B). A lognormal PDF suits environmental concentrations (JCGM GUM-5 Example 10). A required R_C needs a
fixed `u`.

**Specific risks with a prior.** In a project of its own with a `process`, `"posterior": true` computes the
conformance probability of each result from the posterior PDF g(η | η_m) ∝ g₀(η)·h(η_m | η). The prior g₀ is
the process PDF; the likelihood is normal with the u of the result (u_rel·η_m with `u_relative`; JCGM 106 6.2,
Annex A). Near a limit in the tail of the prior this matters: in GUM-5 Example 10, at 0.225 mg/m³, the specific
producer's risk is 11.3 % with the lognormal prior and 5.6 % without it.

## 11.4 Two ways to use it

- **With a model project**: add `"conformity": { "upper": …, "rule": … }` next to `model`. The result of the
  GUM evaluation, and of the Monte Carlo method if it is run, is assessed. A second reported statement gives
  p_c and the decision. In the application, *Settings → Conformity assessment* sets the tolerance and the rule.
- **On its own**: a project with only `conformity`, holding the measured values (`results`), the measuring
  system (`u`, `u_relative`, `dof`) and optionally the process. This suits inspection plans, decision limits
  such as the screening limit of the nandrolone example, and studies of global risks.

The ball-bearing example of JCGM 106 is inconsistent. The text states the acceptance interval as
0 ≤ Y_m ≤ A and the formula for R_C uses it, but the printed R_P (7.5 %) only follows if the lower limit is
left out; with it, R_P is 8.9 % (research log F45).

## 11.5 Items with several correlated properties (total risks)

A medicine with four active components, or a reference material certified for several elements, conforms only if
*all* its properties conform. Even when every component is assessed with small risks, the probability that at least
one true value is out of tolerance can be appreciable, and correlations between the components change it. The
section `multicomponent_conformity` (EMUE compendium 2021, Example E2.1; IUPAC project on total risks) uses:

- a multivariate normal prior of the true values, from production data: means, standard deviations, correlation
  matrix;
- a multivariate normal measuring-system PDF with u_i = u_rel·c_mi and the same correlations;
- simple acceptance: an item is accepted when every test result lies within its tolerance interval.

For each vector of test results the posterior is normal. The **total specific consumer's risk** of an accepted
item is the probability that at least one true value lies outside its interval. It is computed by integrating the
posterior over the tolerance box with the method of Genz, which gives a standard error of about 10⁻⁶. For a
rejected item the total specific producer's risk is given. The **total global risks** come from a Monte Carlo
simulation of production and measurement. The EMUE example (NyQuil tablets) reproduces with 0.29 %, 1.17 % and
0.02 % specific and 0.18 % global consumer's risk.

