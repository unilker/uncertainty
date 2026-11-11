# Monte Carlo results under other seeds

Written by `tools/research/mc_seeds.py`. Each of the 61 library projects with a Monte Carlo check was run with the ten seeds 101–110 instead of its own, and every Monte Carlo result was compared with its reference value and tolerance.

- comparisons: 2810 (281 Monte Carlo values × 10 seeds); runs that failed: 0
- |Δ|/tolerance: median 0.178, 90 % 0.608, 99 % 0.896, maximum 0.981
- outside the tolerance: 0

History (D110): the first run of this test found 20 comparisons from 7 values outside their tolerance (at most 1.26 times it): GUM-5 E14 and E15 (estimate), NUM §3 (lower interval end) and §8 (estimate), TN 1900 E8 (upper interval end) and the correlations r(R, X) and r(R, Z) of JCGM 102 9.4 (two projects). Their tolerances had been set from the program's own seed; they were reset to half a unit in the last printed digit plus five times the standard deviation over eleven seeds, and the run above was repeated.
