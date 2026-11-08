"""Leave-one-out and bilateral degrees of equivalence of the DerSimonian-Laird
procedure: a numpy port of the NIST Consensus Builder source
(DoEUnilateralDerSimonianLaird.R, DoEBilateralDerSimonianLaird.R,
sampleFromTau2Dist.R; Koepke & Possolo, public domain), used as the
independent reference for the program (research log D86).

Data: the two examples shipped with NICOB, 60Co (BIPM.RI(II)-K1.Co-60, 19
laboratories, no degrees of freedom; manual §4.4) and PCB 28 (6 laboratories
with degrees of freedom; manual §4.2). Writes the values with 10^6 draws.
Run: python3 tools/research/consensus/nicob_doe.py
"""
import json

import numpy as np

CO = dict(lab="LNMRI ENEA ANSTO KRISS MKEH LNE-LNHB CIEMAT NPL IRA BARC PTB NMISA CNEA RC NMIJ IRMM IFIN-HH NIST BEV".split(),
          x=[7077, 7065, 7056, 7047, 7051, 7060, 7090, 7053, 7037, 7099, 7057, 7098, 7050, 7040, 7050, 7039, 7101, 7083, 7057],
          u=[8, 26, 10, 22, 18, 4, 11, 21, 8, 46, 16, 16, 15, 40, 8, 17, 24, 14, 17], nu=None)
PCB = dict(lab="IRMM KRISS NARL NIST NMIJ NRC".split(), x=[34.30, 32.90, 34.53, 32.42, 31.90, 35.80],
           u=[1.03, 0.69, 0.83, 0.29, 0.40, 0.38], nu=[60, 4, 18, 2, 13, 60])


def gamma_par(u, t2):
    w = u**-2
    k = len(u)
    s1, s2, s3 = w.sum(), (w**2).sum(), (w**3).sum()
    c = s1 - s2 / s1
    eq = (k - 1) + c * t2
    vq = 2 * (k - 1) + 4 * c * t2 + 2 * (s2 - 2 * s3 / s1 + s2**2 / s1**2) * t2**2
    return eq**2 / vq, vq / eq, c


def tau2_draws(rng, x, u, n):
    w = u**-2
    k = len(x)
    s1, s2 = w.sum(), (w**2).sum()
    c = s1 - s2 / s1
    m0 = (w * x).sum() / s1
    q = (w * (x - m0) ** 2).sum()
    shape, scale, _ = gamma_par(u, (q - (k - 1)) / c)
    return np.maximum(0, (rng.gamma(shape, scale, n) - (k - 1)) / c)


def dl(x, u):
    w = u**-2
    s1, s2 = w.sum(), (w**2).sum()
    m0 = (w * x).sum() / s1
    t2 = max(0, ((w * (x - m0) ** 2).sum() - len(x) + 1) / (s1 - s2 / s1))
    ww = 1 / (u**2 + t2)
    return (ww * x).sum() / ww.sum(), t2


def phi(df):
    if df <= 2:
        a, b = 0.4668994, -0.3998882
        return 1.5 * ((1 - 0.75 * (a - 4 * b * (df ** -0.75 - 1) / 3)) ** (-4 / 3))
    return np.sqrt(df / (df - 2))


def sym_u(v, est, p=0.95):
    return np.quantile(np.abs(v - est), p)


def mra(rng, x, u, nu, K):
    mu, _ = dl(x, u)
    n = len(x)
    t2 = tau2_draws(rng, x, u, K)
    xb = mu + rng.standard_normal((K, n)) * np.sqrt(t2[:, None] + u**2)
    if nu is None:
        ub = np.broadcast_to(u, (K, n))
    else:
        nu = np.asarray(nu, float)
        ub = u * np.sqrt(nu / rng.chisquare(nu, (K, n)))
    w = 1 / ub**2
    s1, s2 = w.sum(1), (w**2).sum(1)
    m0 = (w * xb).sum(1) / s1
    t2b = np.maximum(0, ((w * (xb - m0[:, None]) ** 2).sum(1) - n + 1) / (s1 - s2 / s1))
    ww = 1 / (ub**2 + t2b[:, None])
    mub = (ww * xb).sum(1) / ww.sum(1)
    D = xb - mub[:, None]
    dx = x - mu
    D = D - D.mean(0) + dx
    return dx, D


def loo(rng, x, u, nu, K):
    n = len(x)
    dx = np.zeros(n)
    D = np.zeros((K, n))
    for j in range(n):
        xo, uo = np.delete(x, j), np.delete(u, j)
        t2 = tau2_draws(rng, xo, uo, K)
        nj = len(xo)
        w0 = 1 / uo**2
        x0 = (w0 * xo).sum() / w0.sum()
        tj2 = max(0, ((w0 * (xo - x0) ** 2).sum() - nj + 1) / (w0.sum() - (w0**2).sum() / w0.sum()))
        wj = 1 / (uo**2 + tj2)
        muj = (wj * xo).sum() / wj.sum()
        qs = max(1, (wj * (xo - muj) ** 2).sum() / (nj - 1))
        mujB = muj + np.sqrt(qs / wj.sum()) * rng.standard_t(nj - 1, K) / phi(nj - 1)
        sj = np.sqrt(t2 + u[j] ** 2)
        if nu is None or not np.isfinite(nu[j]):
            e = rng.standard_normal(K) * sj
        else:
            e = sj * rng.standard_t(nu[j], K) / phi(nu[j])
        dx[j] = x[j] - muj
        D[:, j] = x[j] + e - mujB
    return dx, D


def summarize(dx, D, labs, leave_one_out):
    n = len(dx)
    uni = {labs[j]: [float(dx[j]), float(sym_u(D[:, j], dx[j]))] for j in range(n)}
    bil = {}
    for i in range(n):
        for j in range(i + 1, n):
            b = D[:, i] - D[:, j]
            bil[f"{labs[i]}|{labs[j]}"] = [float(dx[i] - dx[j]), float(sym_u(b, b.mean()))]
    return uni, bil


if __name__ == "__main__":
    rng = np.random.default_rng(20261108)
    out = {}
    for name, d in [("co60", CO), ("pcb28", PCB)]:
        x, u = np.array(d["x"], float), np.array(d["u"], float)
        mu, t2 = dl(x, u)
        r = {"dl_mean": mu, "tau": np.sqrt(t2)}
        for ver, f in [("mra", mra), ("loo", loo)]:
            dx, D = f(rng, x, u, d["nu"], 1_000_000)
            r[ver] = summarize(dx, D, d["lab"], ver == "loo")
        out[name] = r
        print(name, "mu", round(mu, 4), "tau", round(np.sqrt(t2), 4))
        for ver in ["mra", "loo"]:
            sig = [k for k, (b, U) in r[ver][1].items() if abs(b) > U]
            print(" ", ver, "first three", {k: [round(a, 3) for a in v] for k, v in list(r[ver][0].items())[:3]},
                  "significant bilateral", len(sig), sig[:8])
    json.dump(out, open("tools/research/consensus/nicob_doe.json", "w"), indent=1)
