# Monte Carlo results under other seeds

Written by `tools/research/mc_seeds.py`. Each of the 61 library projects with a Monte Carlo check was run with the ten seeds 101–110 instead of its own, and every Monte Carlo result was compared with its reference value and tolerance.

- comparisons: 2670 (267 Monte Carlo values × 10 seeds); runs that failed: 0
- |Δ|/tolerance: median 0.155, 90 % 0.519, 99 % 0.873, maximum 0.909
- outside the tolerance: 0

History (D110, D112): the first run of this test found 20 comparisons from 7 values outside their tolerance (at most 1.26 times it): GUM-5 E14 and E15 (estimate), NUM §3 (lower interval end) and §8 (estimate), TN 1900 E8 (upper interval end) and the correlations r(R, X) and r(R, Z) of JCGM 102 9.4 (two projects). Their tolerances had been set from the program's own seed; they were reset to half a unit in the last printed digit plus five times the standard deviation over eleven seeds (the project's own and 101–110), and the run above was repeated. Because those seeds set the seven tolerances, the run above is not a test of them. A test with the ten seeds 201–210 (second internal review, D112) then found 31 comparisons outside tolerance: 24 from the Monte Carlo standard deviations and correlations of JCGM 102 9.4, whose inputs are t-distributed with ν = 3 (no finite fourth moment), and 7 from tolerances that were narrower than the rule; and 3 changes of the GUM validation outcome (NUM §10), which that run did not count. The rule was then applied to every Monte Carlo value by `tools/research/mc_tolerances.py` from the project's own seed and the seeds 101–110 only (69 tolerances widened, by at most 2.9 times); the Monte Carlo standard deviations and correlations of JCGM 102 9.4 and TN 1900 E13 (t₃ inputs) and the GUM validation outcome of NUM §10 and TN 1900 E1, which depends on the seed, are no longer reference values (14 values). The run above and the runs below are with these tolerances.

## Seeds not used to set any tolerance

The same projects with the twenty seeds 201–210 and 301–310 (D112), which were not used to set any tolerance:

- comparisons: 5340 (267 Monte Carlo values × 20 seeds); runs that failed: 0
- |Δ|/tolerance: median 0.158, 90 % 0.526, 99 % 0.865, maximum 0.931
- outside the tolerance: 0

The seven values whose tolerances were reset in D110 (largest |Δ|/tolerance over the twenty seeds):

| Project | Value | max ∣Δ∣/tol |
|---|---|---|
| gum5-e14-radioactive-decay-bi207 | mcm.estimate | 0.60 |
| gum5-e15-wire-rope-breaking-force | mcm.estimate | 0.36 |
| num-3-ratio-abc | mcm.interval_low | 0.83 |
| num-8-end-gauge-gaussian | mcm.estimate | 0.78 |
| nist-tn1900-e8-molecular-weight-co2 | mcm.interval_high | 0.58 |
| jcgm102-9.4-resistance-reactance | multivariate.mcm.r(…) | withdrawn as reference values (t₃ inputs) |
| jcgm102-9.4-resistance-reactance-alternative | multivariate.mcm.r(…) | withdrawn as reference values (t₃ inputs) |
