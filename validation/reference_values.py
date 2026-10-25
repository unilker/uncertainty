"""Independent high-accuracy reference values for the validation suite.

The MCM results tabulated in JCGM 101:2008 are themselves Monte Carlo
estimates (M ~ 1e6, Mersenne Twister, MATLAB) and carry sampling dispersion
in their last digit. Where possible this script computes the reference
analytically; otherwise it uses a large independent Monte Carlo run with
NumPy's PCG64 generator (a different generator and implementation from the
engine under test).

Usage:  python validation/reference_values.py   (needs numpy, scipy)
Output: validation/reference_values.json
"""

import json
import math
from pathlib import Path

import numpy as np
from scipy import optimize, stats

OUT = Path(__file__).with_name("reference_values.json")
P = 0.95


def shortest_interval_from_ppf(ppf, p=P):
    """Shortest interval [ppf(a), ppf(a + p)] by minimising its length."""
    res = optimize.minimize_scalar(lambda a: ppf(a + p) - ppf(a), bounds=(1e-12, 1 - p - 1e-12),
                                   method="bounded", options={"xatol": 1e-14})
    a = res.x
    return [float(ppf(a)), float(ppf(a + p))]


def mc_intervals(sampler, n_total, chunk=10_000_000, p=P, seed=20261025):
    """Large-M Monte Carlo: mean, sd, symmetric and shortest intervals."""
    rng = np.random.Generator(np.random.PCG64(seed))
    parts = [sampler(rng, min(chunk, n_total - i)) for i in range(0, n_total, chunk)]
    y = np.sort(np.concatenate(parts))
    m = y.size
    q = int(math.floor(p * m + 0.5))
    r = (m - q) // 2 if (m - q) % 2 == 0 else (m - q + 1) // 2
    sym = [float(y[r - 1]), float(y[r + q - 1])]
    lengths = y[q:] - y[:m - q]
    k = int(np.argmin(lengths))
    short = [float(y[k]), float(y[k + q])]
    return {"M": m, "mean": float(y.mean()), "sd": float(y.std(ddof=1)), "symmetric": sym, "shortest": short}


def mass_calibration(rng, n):
    m_rc = rng.normal(100_000.000, 0.050, n)
    dm_rc = rng.normal(1.234, 0.020, n)
    rho_a = rng.uniform(1.10, 1.30, n)
    rho_w = rng.uniform(7_000.0, 9_000.0, n)
    rho_r = rng.uniform(7_950.0, 8_050.0, n)
    return (m_rc + dm_rc) * (1 + (rho_a - 1.2) * (1 / rho_w - 1 / rho_r)) - 100_000.0


def comparison_loss(x1, r):
    u = 0.005
    cov = np.array([[u * u, r * u * u], [r * u * u, u * u]])
    chol = np.linalg.cholesky(cov)

    def sampler(rng, n):
        z = rng.standard_normal((2, n))
        x = np.array([[x1], [0.0]]) + chol @ z
        return x[0] ** 2 + x[1] ** 2
    return sampler


def comparison_loss_uncorrelated_exact(x1):
    """δY/u² ~ noncentral χ²(2, λ = x1²/u²) (JCGM 101 Annex F)."""
    u = 0.005
    lam = (x1 / u) ** 2
    dist = stats.chi2(2) if lam == 0 else stats.ncx2(2, lam)
    mean = u * u * (2 + lam)
    sd = u * u * math.sqrt(2 * (2 + 2 * lam))
    if lam == 0:
        short = [0.0, float(-2 * u * u * math.log(1 - P))]
    else:
        lo, hi = shortest_interval_from_ppf(dist.ppf)
        short = [lo * u * u, hi * u * u]
    sym = [float(dist.ppf((1 - P) / 2)) * u * u, float(dist.ppf((1 + P) / 2)) * u * u]
    return {"mean": mean, "sd": sd, "symmetric": sym, "shortest": short, "method": "analytic"}


def main():
    ref = {}
    ref["mass_calibration"] = mc_intervals(mass_calibration, 200_000_000)
    ref["mass_calibration"]["method"] = "PCG64 Monte Carlo"
    for x1 in (0.0, 0.010, 0.050):
        ref[f"comparison_loss_r0_x1_{x1:.3f}"] = comparison_loss_uncorrelated_exact(x1)
        d = mc_intervals(comparison_loss(x1, 0.9), 200_000_000)
        d["method"] = "PCG64 Monte Carlo"
        ref[f"comparison_loss_r0.9_x1_{x1:.3f}"] = d
    OUT.write_text(json.dumps(ref, indent=2) + "\n")
    print(json.dumps(ref, indent=2))


if __name__ == "__main__":
    main()
