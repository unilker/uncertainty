"""Sensitivity of the Bayesian random-effects consensus to the scale of the
half-Cauchy prior on tau (review 2026-11-08, item 3), with the exact grid of
tools/research/consensus/bayes_grid.py (which the program reproduces).

IUPAC 2024 3E/4M/4N (iridium, three results, prior median 0.00042 g/mol and
mu ~ N(192.217, 0.0015^2)) and Koepke et al. G (14 results, NICOB default
median = MAD). Each prior median is multiplied by 1/4, 1/2, 1, 2 and 4.
Run from the repository root: python3 tools/research/review/bayes_prior_sensitivity.py
"""
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "consensus"))
import io, contextlib

with contextlib.redirect_stdout(io.StringIO()):
    import bayes_grid as bg  # noqa: E402  (its module code prints a check)
from koepke2017 import G, uG  # noqa: E402

ir_x = np.array([192.21677, 192.21662, 192.21763])
ir_u = np.array([0.00081, 0.00029, 0.00017])
print("Iridium, M(Ir)/(g/mol): factor, prior median, mean, sd, 95 % interval, median tau")
for f in [0.25, 0.5, 1, 2, 4]:
    m, s, med, q, *_ = bg.post(ir_x, ir_u, 192.217, 0.0015, 0.00042 * f)
    print(f"  {f:5}  {0.00042*f:.6f}  {m:.6f}  {s:.6f}  ({q[0]:.6f}, {q[1]:.6f})  {med:.6f}")
c = np.median(G)
mad = np.median(np.abs(G - c))
print("G /1e-11 m3 kg-1 s-2 (NICOB default prior median = MAD = %.5f)" % mad)
for f in [0.25, 0.5, 1, 2, 4]:
    m, s, med, q, *_ = bg.post(G - c, uG, 0, 0, mad * f)
    print(f"  {f:5}  {mad*f:.6f}  {m+c:.6f}  {s:.6f}  ({q[0]+c:.6f}, {q[1]+c:.6f})  {med:.6f}")
