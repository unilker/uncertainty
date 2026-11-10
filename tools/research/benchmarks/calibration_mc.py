"""Monte Carlo propagation through the ISO 6143 calibration fit, compared with the two linearisations.

For every calibration project of the library, the contents x and responses y of the calibration points are drawn
from normal distributions with their standard uncertainties and the covariances of the project (the distributions
of JCGM 101 for the input quantities; Lira, Measurement 44 (2011) 2156), the generalized least-squares fit of
ISO 6143 (Formula A.3, the same weights as the program) is repeated for every draw, and the standard deviations of
the parameters are compared with

* KAPSAM's covariance: linear propagation with the exact derivatives of the solution, from the full Hessian of the
  sum of squares (ISO 6143 A.2, A.6, A.7), and
* the Gauss–Newton covariance, which drops the second derivatives of the residuals (Suncal's York and ODR fits).

The derivatives of the solution are also computed here by numerical differentiation of the refit (central
differences, the procedure of ISO 6143 A.2), as an independent check of the program's Hessian. The contents of the
samples are propagated as well (stage three of Lira): x = G(y; b) with y drawn from N(y, u²(y)) and b from the
Monte Carlo draws.

    python3 tools/research/benchmarks/calibration_mc.py [kapsam binary] [output.md] [trials]

Needs numpy. Writes a Markdown table (default docs/research/calibration-mc.md).
"""
import glob
import json
import os
import subprocess
import sys
import tempfile

import numpy as np

BIN = sys.argv[1] if len(sys.argv) > 1 else "target/release/kapsam"
OUT = sys.argv[2] if len(sys.argv) > 2 else "docs/research/calibration-mc.md"
TRIALS = int(float(sys.argv[3])) if len(sys.argv) > 3 else 1_000_000
BATCH = 50_000
DEGREE = {"linear": 1, "quadratic": 2, "cubic": 3}


class Function:
    """Analysis function x = G(y; b) of ISO 6143, vectorised over leading axes."""

    def __init__(self, name):
        self.name = name
        self.m = DEGREE[name] + 1 if name in DEGREE else 3

    def eval(self, y, b):
        if self.name in DEGREE:
            return sum(b[..., j, None] * y ** j for j in range(self.m))
        if self.name == "power":
            return b[..., 0, None] + b[..., 1, None] * y ** (1 + b[..., 2, None])
        return b[..., 0, None] + b[..., 1, None] * np.exp(b[..., 2, None] * y)

    def d_db(self, y, b):
        """∂G/∂b, shape (..., n, m)."""
        if self.name in DEGREE:
            return np.stack([np.broadcast_to(y ** j, np.broadcast_shapes(y.shape, b[..., 0, None].shape))
                             for j in range(self.m)], -1)
        one = np.ones(np.broadcast_shapes(y.shape, b[..., 0, None].shape))
        if self.name == "power":
            p = y ** (1 + b[..., 2, None])
            return np.stack([one, p * one, b[..., 1, None] * p * np.log(y)], -1)
        e = np.exp(b[..., 2, None] * y)
        return np.stack([one, e * one, b[..., 1, None] * y * e], -1)

    def d_dy(self, y, b):
        if self.name in DEGREE:
            return sum(j * b[..., j, None] * y ** (j - 1) for j in range(1, self.m))
        if self.name == "power":
            return b[..., 1, None] * (1 + b[..., 2, None]) * y ** b[..., 2, None]
        return b[..., 1, None] * b[..., 2, None] * np.exp(b[..., 2, None] * y)


def fit(f, x, ux, y, uy, b0, yh0, ub, iterations=300):
    """Minimises S = Σ (G(ŷ) − x)²/u²(x) + (ŷ − y)²/u²(y) over (b, ŷ) for a batch of data sets (rows of x, y) by
    damped Gauss–Newton steps from (b0, ŷ0). A data set has converged when the Gauss–Newton step is below 10⁻⁹ of
    u(b) and of u(y). Returns b, ŷ and a flag for the data sets that converged."""
    nb, n = x.shape
    m = f.m
    b = np.repeat(b0[None, :], nb, 0).astype(float)
    yh = np.repeat(yh0[None, :], nb, 0).astype(float)
    tol = 1e-9 * np.concatenate([ub, uy[0]])

    def residuals(b, yh, idx):
        return np.concatenate([(f.eval(yh, b) - x[idx]) / ux[idx], (yh - y[idx]) / uy[idx]], 1)

    r = residuals(b, yh, slice(None))
    s = (r * r).sum(1)
    lam = np.full(nb, 1e-9)
    done = np.zeros(nb, bool)
    for _ in range(iterations):
        act = np.flatnonzero(~done)
        if not len(act):
            break
        ba, ya, ra = b[act], yh[act], r[act]
        uxa, uya = ux[act], uy[act]
        jac = np.zeros((len(act), 2 * n, m + n))
        jac[:, :n, :m] = f.d_db(ya, ba) / uxa[:, :, None]
        jac[:, np.arange(n), m + np.arange(n)] = f.d_dy(ya, ba) / uxa
        jac[:, n + np.arange(n), m + np.arange(n)] = 1 / uya
        jtj = np.einsum("tia,tib->tab", jac, jac)
        g = np.einsum("tia,ti->ta", jac, ra)
        diag = np.einsum("taa->ta", jtj)
        a = jtj + (lam[act][:, None] * diag)[:, :, None] * np.eye(m + n)
        step = np.linalg.solve(a, -g[..., None])[..., 0]
        small = (np.abs(step) <= tol).all(1)
        bn, yn = ba + step[:, :m], ya + step[:, m:]
        with np.errstate(all="ignore"):
            rn = residuals(bn, yn, act)
            sn = (rn * rn).sum(1)
        ok = np.isfinite(sn) & (sn <= s[act])
        acc = act[ok]
        b[acc], yh[acc], r[acc], s[acc] = bn[ok], yn[ok], rn[ok], sn[ok]
        lam[act] = np.where(ok, np.maximum(lam[act] * 0.3, 1e-12), lam[act] * 10)
        done[act[small]] = True
    return b, yh, done


def linearised(f, x, ux, y, uy, cov_x, b, yh, ub):
    """Covariance of b from (i) numerical derivatives of the refit and (ii) the Gauss–Newton approximation."""
    n, m = len(x), f.m
    sigma = np.diag(np.concatenate([ux ** 2, uy ** 2]))
    for i, h, c in cov_x:
        sigma[i, h] = sigma[h, i] = c
    # (i) central differences of the solution with respect to each x_i, y_i (ISO 6143 A.2).
    z0 = np.concatenate([x, y])
    u = np.sqrt(np.diag(sigma))
    rows = []
    for k in range(2 * n):
        for sgn in (1, -1):
            z = z0.copy()
            z[k] += sgn * 1e-4 * u[k]
            rows.append(z)
    zs = np.array(rows)
    bs, _, conv = fit(f, zs[:, :n], ux[None, :].repeat(len(zs), 0), zs[:, n:], uy[None, :].repeat(len(zs), 0), b,
                      yh, ub)
    assert conv.all()
    jb = ((bs[0::2] - bs[1::2]) / (2e-4 * u[:, None])).T  # (m, 2n)
    cov_num = jb @ sigma @ jb.T
    # (ii) Gauss–Newton: dθ/dz = −(JᵀJ)⁻¹ Jᵀ ∂r/∂z at the solution.
    jac = np.zeros((2 * n, m + n))
    jac[:n, :m] = f.d_db(yh, b) / ux[:, None]
    jac[np.arange(n), m + np.arange(n)] = f.d_dy(yh, b) / ux
    jac[n + np.arange(n), m + np.arange(n)] = 1 / uy
    drdz = -np.diag(1 / np.concatenate([ux, uy]))
    dtheta = -np.linalg.solve(jac.T @ jac, jac.T @ drdz)
    cov_gn = dtheta[:m] @ sigma @ dtheta[:m].T
    return cov_num, cov_gn, sigma


def main():
    rows = []
    for path in sorted(glob.glob("library/*/*.json")):
        project = json.load(open(path, encoding="utf-8"))
        if "calibration" not in project:
            continue
        pid = project["metadata"]["id"]
        spec = project["calibration"]
        f = Function(spec.get("function", "linear"))
        pts = spec["points"]
        names = [p["name"] for p in pts]
        x = np.array([p["x"] for p in pts])
        ux = np.array([p["u_x"] for p in pts])
        y = np.array([p["y"] for p in pts])
        uy = np.array([p["u_y"] for p in pts])
        cov_x = [(names.index(c["between"][0]), names.index(c["between"][1]), c["value"])
                 for c in spec.get("covariances", [])]
        with tempfile.TemporaryDirectory() as tmp:
            out = os.path.join(tmp, "r.json")
            subprocess.run([BIN, "run", path, "-o", out], check=True, capture_output=True)
            res = json.load(open(out, encoding="utf-8"))["calibration"]
        b = np.array(res["params"])
        yh = np.array([a["y"] if isinstance(a, dict) else a[1] for a in res["adjusted"]])
        cov_k = np.array(res["covariance"])
        cov_num, cov_gn, sigma = linearised(f, x, ux, y, uy, cov_x, b, yh, np.sqrt(np.diag(cov_k)))
        # Monte Carlo.
        rng = np.random.default_rng(20261110)
        chol = np.linalg.cholesky(sigma)
        n = len(x)
        samples = spec.get("samples", [])
        sd_batches, xs_batches, nonconv, bsum = [], [], 0, 0
        for start in range(0, TRIALS, BATCH):
            nb = min(BATCH, TRIALS - start)
            z = np.concatenate([x, y]) + rng.standard_normal((nb, 2 * n)) @ chol.T
            bb, _, good = fit(f, z[:, :n], np.repeat(ux[None], nb, 0), z[:, n:], np.repeat(uy[None], nb, 0), b, yh,
                              np.sqrt(np.diag(cov_k)))
            nonconv += int((~good).sum())
            bb = bb[good]
            bsum = bsum + bb.sum(0)
            sd_batches.append(bb.std(0, ddof=1))
            if samples:
                ys = np.array([s["y"] for s in samples])
                uys = np.array([s["u_y"] for s in samples])
                yy = ys + rng.standard_normal((len(bb), len(ys))) * uys
                xs_batches.append(f.eval(yy, bb).std(0, ddof=1))
        sd = np.array(sd_batches)
        sd_mc, se_mc = sd.mean(0), sd.std(0, ddof=1) / np.sqrt(len(sd))
        u_k, u_num, u_gn = (np.sqrt(np.diag(c)) for c in (cov_k, cov_num, cov_gn))
        sample_rows = []
        if samples:
            xs = np.array(xs_batches)
            for j, s in enumerate(samples):
                ks = next(q for q in res["samples"] if q["name"] == s["name"])
                g_b = f.d_db(np.array([s["y"]]), b)[0]
                g_y = f.d_dy(np.array([s["y"]]), b)[0]
                u_gn_x = np.sqrt(g_b @ cov_gn @ g_b + (g_y * s["u_y"]) ** 2)
                sample_rows.append((s["name"], ks["u"], float(u_gn_x), xs[:, j].mean(), xs[:, j].std(ddof=1)
                                    / np.sqrt(len(xs))))
        rows.append(dict(pid=pid, function=f.name, gamma=res["gamma"], n=n, cov=bool(cov_x), u_k=u_k, u_num=u_num,
                         u_gn=u_gn, sd=sd_mc, se=se_mc, nonconv=nonconv, samples=sample_rows,
                         bias=(bsum / (TRIALS - nonconv) - b) / u_k))
        print(pid, "done", flush=True)
    write(rows)


def write(rows):
    L = ["# Monte Carlo propagation through the calibration fit", "",
         "Generated by `python3 tools/research/benchmarks/calibration_mc.py` (research log D97). For every "
         "calibration project of the library the calibration data are drawn "
         f"{TRIALS:,} times from normal distributions with their standard uncertainties and covariances and the "
         "ISO 6143 fit is repeated for every draw (Lira, Measurement 44 (2011) 2156). The standard deviations of the "
         "parameters are compared with KAPSAM's linear propagation (exact derivatives of the solution, full Hessian; "
         "ISO 6143 A.2, A.6, A.7) and with the Gauss–Newton approximation (Suncal's York and ODR covariances). The "
         "standard error of the Monte Carlo value comes from 20 batches.".replace(",", " "), "",
         "## Parameters", "",
         "| Project | function | Γ | parameter | u KAPSAM | u numerical refit / KAPSAM − 1 | u Gauss–Newton / KAPSAM − 1 | "
         "s.d. Monte Carlo / KAPSAM − 1 | standard error | (mean − b)/u |",
         "|---|---|---:|---|---:|---:|---:|---:|---:|---:|"]
    for r in rows:
        for j in range(len(r["u_k"])):
            L.append(f"| {r['pid'] if j == 0 else ''} | {r['function'] if j == 0 else ''} | "
                     f"{r['gamma']:.2f} | b{j} | {r['u_k'][j]:.5g} | {r['u_num'][j] / r['u_k'][j] - 1:+.1e} | "
                     f"{r['u_gn'][j] / r['u_k'][j] - 1:+.2%} | {r['sd'][j] / r['u_k'][j] - 1:+.2%} | "
                     f"{r['se'][j] / r['u_k'][j]:.2%} | {r['bias'][j]:+.3f} |")
    L += ["", "## Contents of the samples", "",
          "| Project | sample | u(x) KAPSAM | Gauss–Newton / KAPSAM − 1 | s.d. Monte Carlo / KAPSAM − 1 | standard error |",
          "|---|---|---:|---:|---:|---:|"]
    for r in rows:
        for name, uk, ugn, sdm, se in r["samples"]:
            L.append(f"| {r['pid']} | {name} | {uk:.5g} | {ugn / uk - 1:+.2%} | {sdm / uk - 1:+.2%} | {se / uk:.2%} |")
    L += ["", "Draws for which the fit did not converge: " + ", ".join(f"{r['pid']} {r['nonconv']}" for r in rows) + "."]
    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write("\n".join(L) + "\n")
    print("table in", OUT)


if __name__ == "__main__":
    main()
