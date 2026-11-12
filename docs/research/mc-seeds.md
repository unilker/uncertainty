# Monte Carlo results under other seeds

Written by `tools/research/mc_seeds.py`. Each of the 60 library projects with a Monte Carlo check was run with the ten seeds 101–110 instead of its own, and every Monte Carlo result was compared with its reference value and tolerance.

- comparisons: 2650 (265 Monte Carlo values × 10 seeds); runs that failed: 0
- |Δ|/tolerance: median 0.133, 90 % 0.485, 99 % 0.851, maximum 0.876
- outside the tolerance: 0

The rule (D113): tolerance = max(documented tolerance, h + 5 s), where the documented tolerance is the one stated in the project (half a unit in the last printed digit and, for a reference that is itself a Monte Carlo or MCMC estimate, its sampling error), h is half a unit in the last digit of the reference value and s is the standard deviation of the program's result over the project's own seed and the seeds 101–110 (`tools/research/mc_tolerances.py`). No term depends on the distance between the program's result and the reference value. Because the seeds 101–110 enter the tolerance, the run above is not a test of it; the test is the run with seeds that set no tolerance, below.

History. D110: the first run of this test found 20 comparisons from 7 values outside their tolerance; their tolerances had been set from the program's own seed and were reset from eleven seeds. D112: a test with the seeds 201–210 found 31 comparisons outside tolerance, 24 of them from the Monte Carlo standard deviations and correlations of JCGM 102 9.4, whose inputs are t-distributed with ν = 3 (their sample standard deviations have no finite variance), and 3 changes of the GUM validation outcome of NUM §10; these values were withdrawn as reference values (14 values, finding F102), and a rule that added the program's distance from the reference was applied to every value. D113: that distance term was removed (it lets any systematic difference pass); the GUM-5 Example 14 comparison of the Monte Carlo result with the printed GUM estimate, a different quantity, was withdrawn; and `--seed` now reaches the consensus bootstrap and the multicomponent risks, which earlier runs of this test had left at their own seed.

## Seeds not used to set any tolerance

The same projects with the twenty seeds 201–210 and 301–310 (D112, D113), which were not used to set any tolerance:

- comparisons: 5300 (265 Monte Carlo values × 20 seeds); runs that failed: 0
- |Δ|/tolerance: median 0.132, 90 % 0.494, 99 % 0.847, maximum 0.973
- outside the tolerance: 0

The seven values whose tolerances were reset in D110 (largest |Δ|/tolerance over the twenty seeds):

| Project | Value | max ∣Δ∣/tol |
|---|---|---|
| gum5-e14-radioactive-decay-bi207 | mcm.estimate | withdrawn as reference values (compared with the printed GUM estimate) |
| gum5-e15-wire-rope-breaking-force | mcm.estimate | 0.36 |
| num-3-ratio-abc | mcm.interval_low | 0.83 |
| num-8-end-gauge-gaussian | mcm.estimate | 0.97 |
| nist-tn1900-e8-molecular-weight-co2 | mcm.interval_high | 0.58 |
| jcgm102-9.4-resistance-reactance | multivariate.mcm.r( | withdrawn as reference values (t₃ inputs) |
| jcgm102-9.4-resistance-reactance-alternative | multivariate.mcm.r( | withdrawn as reference values (t₃ inputs) |
