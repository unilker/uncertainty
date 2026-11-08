"""Independent check of the consensus values of Koepke et al., Metrologia 54 (2017) S34, Tables 1–4, with the
procedures of the NIST Consensus Builder manual (2020) Appendix A: DerSimonian–Laird, Knapp–Hartung, the NICOB
parametric bootstrap (Biggerstaff–Tweedie gamma approximation of Q) and the Linear Pool. numpy, seed 1.
Usage: python3 -I koepke2017.py
"""
import numpy as np
from scipy import stats

G = np.array([6.67248, 6.6729, 6.67398, 6.674255, 6.67559, 6.67422, 6.67387, 6.67222, 6.67425, 6.67349, 6.67234,
              6.67554, 6.67191, 6.67435])
uG = np.array([0.00043, 0.0005, 0.00070, 0.000092, 0.00027, 0.00098, 0.00027, 0.00087, 0.00012, 0.00018, 0.00014,
               0.00016, 0.00099, 0.00013])
Cs = np.array([9715, 10957, 11103, 10994, 10840, 10665, 11220, 10921, 11286, 11191, 11023, 11020.8, 11034, 10906,
               11009, 10967.8, 10940.8, 11018.3, 10970])
uCs = np.array([146, 146, 146, 256, 18, 110, 47, 183, 256, 157, 37, 4.1, 29, 33, 11, 4.5, 6.9, 9.5, 20])
Sr = np.array([10120, 10700, 10230, 10410, 10636, 10282, 10588, 10665, 10561, 10495, 10557])
uSr = np.array([150, 580, 150, 330, 88, 12, 91, 37, 14, 4, 11])


def dl(x, u):
    w = 1 / u ** 2
    n = len(x)
    xw = (w * x).sum() / w.sum()
    Q = (w * (x - xw) ** 2).sum()
    tau2 = max(0, (Q - (n - 1)) / (w.sum() - (w ** 2).sum() / w.sum()))
    ws = 1 / (u ** 2 + tau2)
    mu = (ws * x).sum() / ws.sum()
    um = 1 / np.sqrt(ws.sum())
    ukh = um * np.sqrt((ws * (x - mu) ** 2).sum() / (n - 1))
    return mu, um, np.sqrt(tau2), Q, ukh


def bootstrap(x, u, K=100000, seed=1):
    rng = np.random.default_rng(seed)
    n = len(x)
    mu, um, tau, Q, ukh = dl(x, u)
    w = 1 / u ** 2
    S1, S2, S3 = w.sum(), (w ** 2).sum(), (w ** 3).sum()
    t2 = tau ** 2
    EQ = (n - 1) + t2 * (S1 - S2 / S1)
    VQ = 2 * (n - 1) + 4 * t2 * (S1 - 2 * S2 / S1 + S3 / S1 ** 2) + 2 * t2 ** 2 * (S2 - 2 * S3 / S1 + S2 ** 2 / S1 ** 2)
    Qk = rng.gamma(EQ ** 2 / VQ, VQ / EQ, K)
    t2k = np.maximum(0, (Qk - (n - 1)) / (S1 - S2 / S1))
    xs = mu + rng.standard_normal((K, n)) * np.sqrt(t2k[:, None] + u ** 2)
    xw = (xs * w).sum(1) / S1
    Qs = ((xs - xw[:, None]) ** 2 * w).sum(1)
    t2s = np.maximum(0, (Qs - (n - 1)) / (S1 - S2 / S1))
    ws = 1 / (u ** 2 + t2s[:, None])
    muk = (ws * xs).sum(1) / ws.sum(1)
    return np.std(muk, ddof=1), np.percentile(muk, [2.5, 97.5])


def linear_pool(x, u, K=10 ** 6, seed=1):
    rng = np.random.default_rng(seed)
    j = rng.integers(0, len(x), K)
    s = x[j] + u[j] * rng.standard_normal(K)
    exact = (x.mean(), np.sqrt((u ** 2).mean() + ((x - x.mean()) ** 2).mean()))
    return exact, np.percentile(s, [2.5, 97.5])


if __name__ == "__main__":
    for name, x, u, f in (("G", G, uG, 1), ("Cs-137 /a", Cs, uCs, 1 / 365.25), ("Sr-90 /a", Sr, uSr, 1 / 365.25)):
        mu, um, tau, Q, ukh = dl(x, u)
        ub, ci = bootstrap(x, u)
        (m, s), lci = linear_pool(x, u)
        t = stats.t.ppf(0.975, len(x) - 1)
        print(f"{name}: DL mu={mu * f:.7g} u={um * f:.5g} tau={tau * f:.5g} Q={Q:.4f} uKH={ukh * f:.5g}"
              f" t-interval=({(mu - t * um) * f:.7g}, {(mu + t * um) * f:.7g})")
        print(f"   bootstrap u={ub * f:.5g} interval=({ci[0] * f:.7g}, {ci[1] * f:.7g})")
        print(f"   Linear Pool mean={m * f:.7g} u={s * f:.5g} interval=({lci[0] * f:.7g}, {lci[1] * f:.7g})")
