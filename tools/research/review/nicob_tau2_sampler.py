"""DerSimonian-Laird parametric bootstrap: the program vs the NIST Consensus
Builder source (review 2026-11-08, item 4).

NICOB, sampleFromTau2Dist.R (Koepke & Possolo, 2016-03-31; public domain):
  - the gamma approximation of Q is set at tau2_hat_M = (Q - (K-1))/c, which
    is negative when Q < K - 1 (then E Q = Q), not at max(0, tau2_hat_M);
  - Var Q = 2(K-1) + 4 c tau2 + 2 (S2 - 2 S3/S1 + S2^2/S1^2) tau2^2 with
    c = S1 - S2/S1, whereas Biggerstaff & Tweedie (1997) eq. (7) has
    4 (S1 - 2 S2/S1 + S3/S1^2) tau2 in the linear term.
The program (core/src/consensus.rs) used tau2_DL >= 0 and the paper's form.

This script evaluates u and the 95 % interval of the DL consensus value and the
degrees of equivalence with the four combinations, on Koepke et al. (2017) G
and on a homogeneous set (Q < K - 1).
Run: python3 tools/research/review/nicob_tau2_sampler.py
"""
import numpy as np

G = np.array([6.67248, 6.6729, 6.67398, 6.674255, 6.67559, 6.67422, 6.67387, 6.67222, 6.67425, 6.67349,
              6.67234, 6.67554, 6.67191, 6.67435])
UG = np.array([0.00043, 0.0005, 0.00070, 0.000092, 0.00027, 0.00098, 0.00027, 0.00087, 0.00012, 0.00018,
               0.00014, 0.00016, 0.00099, 0.00013])
HOM_X = np.array([10.0, 10.1, 9.95, 10.02])
HOM_U = np.array([0.1, 0.2, 0.15, 0.12])


def dl(x, u):
    w = u**-2
    s1, s2 = w.sum(), (w**2).sum()
    mu = (w * x).sum() / s1
    q = (w * (x - mu) ** 2).sum()
    t2m = (q - (len(x) - 1)) / (s1 - s2 / s1)
    t2 = max(0.0, t2m)
    ww = 1 / (t2 + u**2)
    return (ww * x).sum() / ww.sum(), t2, t2m, q


def gamma_params(u, tau2, linear):
    w = u**-2
    k = len(u)
    s1, s2, s3 = w.sum(), (w**2).sum(), (w**3).sum()
    c = s1 - s2 / s1
    eq = (k - 1) + c * tau2
    lin = 4 * c if linear == "nicob" else 4 * (s1 - 2 * s2 / s1 + s3 / s1**2)
    vq = 2 * (k - 1) + lin * tau2 + 2 * (s2 - 2 * s3 / s1 + s2**2 / s1**2) * tau2**2
    return eq**2 / vq, vq / eq, c


def bootstrap(x, u, at, linear, n=200_000, seed=1):
    rng = np.random.default_rng(seed)
    mu, t2, t2m, q = dl(x, u)
    shape, scale, c = gamma_params(u, t2m if at == "tau2_M" else t2, linear)
    k = len(x)
    tau2 = np.maximum(0.0, (scale * rng.gamma(shape, size=n) - (k - 1)) / c)
    xb = mu + rng.standard_normal((n, k)) * np.sqrt(tau2[:, None] + u**2)
    w = 1 / u**2
    s1, s2 = w.sum(), (w**2).sum()
    mub0 = (xb * w).sum(1) / s1
    qb = (w * (xb - mub0[:, None]) ** 2).sum(1)
    t2b = np.maximum(0.0, (qb - (k - 1)) / (s1 - s2 / s1))
    ww = 1 / (t2b[:, None] + u**2)
    mub = (ww * xb).sum(1) / ww.sum(1)
    d = xb - mub[:, None]
    doe_u = np.array([np.quantile(np.abs(d[:, j] - d[:, j].mean()), 0.95) for j in range(k)])
    return mu, mub.std(ddof=1), np.quantile(mub, [0.025, 0.975]), doe_u, q, t2, t2m


for name, x, u in [("Koepke G", G, UG), ("homogeneous (Q < K-1)", HOM_X, HOM_U)]:
    print(f"\n{name}")
    for at in ["tau2_DL", "tau2_M"]:
        for linear in ["paper", "nicob"]:
            mu, sd, ci, doe, q, t2, t2m = bootstrap(x, u, at, linear)
            print(f"  gamma at {at:7s}, Var Q linear term {linear:5s}: u = {sd:.6g}, 95 % ({ci[0]:.6g}, {ci[1]:.6g}),"
                  f" U95(D) first two {doe[0]:.4g} {doe[1]:.4g}")
    print(f"  Q = {q:.4g}, K - 1 = {len(x) - 1}, tau2_DL = {t2:.4g}, tau2_M = {t2m:.4g}")
