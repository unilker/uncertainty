"""Independent reference for the three TUR guard-band projects (library/general/tur-guard-band-*.json): the
guard-band factor from the published formulas (Dobbert 2008, NCSLI RP-10, root-sum-square) and the global consumer's
and producer's risks of JCGM 106:2012 (9.5.2, 9.5.3) by numerical integration with SciPy, without Suncal.

    python3 -I tools/research/conformity/tur_risks.py
"""
import math

from scipy import integrate, stats

T = 1.0                                  # tolerance limits ±1
SP = T / stats.norm.ppf(0.975)           # process s.d. for a 95 % in-tolerance probability


def gbf(method, tur):
    if method == "dobbert":
        m = 1.04 - math.exp(0.38 * math.log(tur) - 0.54)
        return 1 - m / tur
    if method == "rp10":
        return 1.25 - 1 / tur
    if method == "rss":
        return math.sqrt(1 - 1 / tur ** 2)
    raise ValueError(method)


def risks(u, a):
    """Global consumer's risk R_C (accept although out of tolerance) and producer's risk R_P for acceptance limits ±a."""
    g0 = stats.norm(0, SP)
    acc = lambda y: stats.norm.cdf(a, y, u) - stats.norm.cdf(-a, y, u)  # noqa: E731
    rc = 2 * integrate.quad(lambda y: g0.pdf(y) * acc(y), T, math.inf, epsabs=1e-14, epsrel=1e-12)[0]
    rp = integrate.quad(lambda y: g0.pdf(y) * (1 - acc(y)), -T, T, epsabs=1e-14, epsrel=1e-12)[0]
    return rc, rp


for method, tur in (("dobbert", 2.5), ("rp10", 2.0), ("rss", 1.5)):
    u = T / tur / 2                     # TUR = T / U95 with U95 = 2u
    g = gbf(method, tur)
    rc, rp = risks(u, g * T)
    print(f"{method:8s} TUR {tur}: GBF {g:.12f}  R_C {rc:.10f}  R_P {rp:.10f}  process s.d. {SP:.10f}")
