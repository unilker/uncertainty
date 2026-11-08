"""Coverage of the GUF interval in JCGM GUM-5:2026 Example 2 with the two
readings of the effective degrees of freedom (research log F79; independent
review 2026-11-08, item 1).

A_E and A_ISE are the means of the same three paired injections; their
variances and covariance are estimated from those three pairs (nu = 2). The
calibration factor f has nu = 9; m_ISE is known (certificate, nu = inf), with
the stated correlation r(f, m_ISE) = -0.54.

For each simulated data set the GUF is applied as in GUM-5 2.4.1: estimate,
u_c with the estimated covariances, and nu_eff from
  (a) G.2b, ignoring correlations,
  (b) eq. (2.3) of GUM-5 (attributed to Willink 2007),
  (c) Willink's ensemble rule: A_E and A_ISE form one ensemble with nu = 2,
      contributing (c^T S c)^2 / 2 to the denominator.
The 95 % interval is y +/- t_{0.975,nu} u_c; the coverage is the fraction of
data sets whose interval contains the true m_E.

Run: python3 tools/research/review/f79_coverage.py
"""
import numpy as np
from scipy import stats

rng = np.random.default_rng(20261108)
N = 400_000
f0, m0, AE0, AI0 = 0.616, 0.2455, 85114.0, 917546.0
uf, um = 0.017, 0.0036
r_fm = -0.5392
sE, sI, rho = 9564.0 * np.sqrt(3), 44492.0 * np.sqrt(3), -0.4771  # per injection
true = f0 * AE0 / AI0 * m0

# f and m_ISE: jointly normal estimates; s_f from nu = 9.
cov_fm = np.array([[uf**2, r_fm * uf * um], [r_fm * uf * um, um**2]])
fm = rng.multivariate_normal([f0, m0], cov_fm, size=N)
f, m = fm[:, 0], fm[:, 1]
uf_hat = uf * np.sqrt(rng.chisquare(9, N) / 9)
# Three paired injections.
cov_a = np.array([[sE**2, rho * sE * sI], [rho * sE * sI, sI**2]])
inj = rng.multivariate_normal([AE0, AI0], cov_a, size=(N, 3))
AE, AI = inj[:, :, 0].mean(1), inj[:, :, 1].mean(1)
S = np.einsum("nki,nkj->nij", inj - inj.mean(1, keepdims=True), inj - inj.mean(1, keepdims=True)) / 2 / 3
uE, uI, cEI = np.sqrt(S[:, 0, 0]), np.sqrt(S[:, 1, 1]), S[:, 0, 1]

y = f * AE / AI * m
cf, cm, cE, cI = AE / AI * m, f * AE / AI, f / AI * m, -f * AE / AI**2 * m
tf, tm, tE, tI = cf * uf_hat, cm * um, cE * uE, cI * uI
rfm = r_fm
rEI = cEI / (uE * uI)
u2 = tf**2 + tm**2 + tE**2 + tI**2 + 2 * rfm * tf * tm + 2 * rEI * tE * tI
u = np.sqrt(u2)

nu_g2b = u2**2 / (tf**4 / 9 + tE**4 / 2 + tI**4 / 2)
# eq. (2.3): sum_i sum_j r_ij^2 c_i^2 c_j^2 u_i^2 u_j^2 / nu_i (r_ii = 1).
t = {"f": (tf, 9.0), "m": (tm, np.inf), "E": (tE, 2.0), "I": (tI, 2.0)}
r = {("f", "m"): rfm, ("m", "f"): rfm, ("E", "I"): rEI, ("I", "E"): rEI}
den = 0
for i, (ti, ni) in t.items():
    for j, (tj, _) in t.items():
        rij = 1.0 if i == j else r.get((i, j), 0.0)
        den = den + rij**2 * ti**2 * tj**2 / ni
nu_23 = u2**2 / den
# Ensemble rule: the A_E, A_ISE block contributes (c^T S c)^2 / 2.
block = tE**2 + tI**2 + 2 * rEI * tE * tI
nu_ens = u2**2 / (tf**4 / 9 + block**2 / 2)

print(f"true m_E = {true:.6f} ng; data sets: {N}")
for name, nu in [("G.2b (no correlations)", nu_g2b), ("eq. (2.3) GUM-5", nu_23), ("ensemble rule", nu_ens)]:
    k = stats.t.ppf(0.975, nu)
    cover = np.mean(np.abs(y - true) <= k * u)
    print(f"{name:24s} median nu {np.median(nu):6.2f}  coverage {cover:.4f} (+/- {1.96*np.sqrt(cover*(1-cover)/N):.4f})")
k_inf = 1.959964
print(f"{'k = 1.96':24s} {'':16s} coverage {np.mean(np.abs(y - true) <= k_inf * u):.4f}")
# At the printed data (one data set) for reference.
print("printed budget: nu (2.3) = 4.81, ensemble 2.11 (program)")

# The same with nu fixed at the values of the printed budget (the reading of
# the review): k from nu = 4.81 and from nu = 2.11 applied to every data set.
for name, nu in [("fixed nu = 4.81", 4.81), ("fixed nu = 2.11", 2.11), ("fixed nu = 5.2", 5.2)]:
    k = stats.t.ppf(0.975, nu)
    print(f"{name:24s} k {k:5.2f}       coverage {np.mean(np.abs(y - true) <= k * u):.4f}")
print(f"fraction of data sets with nu_ens > nu_(2.3): {np.mean(nu_ens > nu_23):.3f}")
print("quartiles nu (2.3):", np.round(np.percentile(nu_23, [25, 50, 75]), 2),
      " ensemble:", np.round(np.percentile(nu_ens, [25, 50, 75]), 2))
# Share of the A block in u^2 per data set.
print("median share of the A_E, A_ISE block in u^2:", round(float(np.median(block / u2)), 3))

# Separate the effect of the nonlinearity of the model: the same with the
# linearised model y_lin = m_E + sum c_i(x0) (x_i - x0_i) and u from the
# sensitivity coefficients at the true values.
c0 = np.array([AE0 / AI0 * m0, f0 * AE0 / AI0, f0 / AI0 * m0, -f0 * AE0 / AI0**2 * m0])
y_lin = true + c0[0] * (f - f0) + c0[1] * (m - m0) + c0[2] * (AE - AE0) + c0[3] * (AI - AI0)
lf, lm, lE, lI = c0[0] * uf_hat, c0[1] * um, c0[2] * uE, c0[3] * uI
lu2 = lf**2 + lm**2 + lE**2 + lI**2 + 2 * rfm * lf * lm + 2 * rEI * lE * lI
lblock = lE**2 + lI**2 + 2 * rEI * lE * lI
lden = 0
lt = {"f": (lf, 9.0), "m": (lm, np.inf), "E": (lE, 2.0), "I": (lI, 2.0)}
for i, (ti, ni) in lt.items():
    for j, (tj, _) in lt.items():
        rij = 1.0 if i == j else r.get((i, j), 0.0)
        lden = lden + rij**2 * ti**2 * tj**2 / ni
print("linearised model:")
for name, nu in [("eq. (2.3)", lu2**2 / lden), ("ensemble rule", lu2**2 / (lf**4 / 9 + lblock**2 / 2)),
                 ("fixed nu = 4.81", 4.81), ("fixed nu = 2.11", 2.11)]:
    k = stats.t.ppf(0.975, nu)
    print(f"  {name:22s} coverage {np.mean(np.abs(y_lin - true) <= k * np.sqrt(lu2)):.4f}")
