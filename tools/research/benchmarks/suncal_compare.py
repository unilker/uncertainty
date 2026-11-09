"""Benchmark of KAPSAM against Sandia's Suncal (PyPI `suncal`, version printed below) on every library project
that Suncal can express (research log D74, extended in D93).

1. Measurement models with one output (`model`). Correlated inputs are passed to Suncal as correlation
   coefficients, intermediate quantities are substituted into the output expression, log10 is written as
   ln(·)/ln(10). Compared: the estimate and the first-order GUM standard uncertainty (Suncal's GUM is first
   order) and, where Suncal can sample every input as the program does, the Monte Carlo standard deviation with
   10⁶ draws. t and type_a inputs enter the GUM comparison with their standard uncertainty only (the Monte Carlo
   conventions for t inputs differ: research log F31); beta inputs likewise (Suncal's beta has no location);
   correlated inputs are sampled only when they are all normal (both programs then sample a multivariate normal).
2. Models with several outputs (`multivariate`, JCGM 102): every output estimate and standard uncertainty, every
   correlation coefficient between outputs, and the Monte Carlo standard deviations.
3. Calibration with uncertainties in both variables (`calibration`, ISO 6143): straight lines with Suncal's York
   fit, quadratic, power and exponential functions with Suncal's orthogonal distance regression (its objective,
   Σ(δx/u_x)² + (δy/u_y)², is the generalised least-squares criterion of ISO 6143). Compared: the parameters,
   their standard uncertainties and the uncertainty of every sample result. Projects with covariances between
   calibration points are outside both Suncal methods.
4. Conformity assessment (`conformity`, JCGM 106): the conformance probability of every result with a normal or
   scaled-t measurement distribution (Suncal `specific_risk`), and the global consumer and producer risks for a
   normal or gamma process and a measurement uncertainty that does not depend on the value (Suncal `PFA`,
   `PFR`).

Run with a Python that has suncal:
    PYTHONHASHSEED=0 python3 tools/research/benchmarks/suncal_compare.py [path to the kapsam binary] > S4.md
(Suncal draws from the global NumPy generator, seeded below; the hash seed fixes the order of its internal sets.)
"""
import glob
import json
import math
import re
import statistics
import subprocess
import sys

import numpy as np
from scipy import stats

import suncal
from suncal import curvefit, risk

np.random.seed(1)  # Suncal draws from the global NumPy generator

BIN = sys.argv[1] if len(sys.argv) > 1 else "target/release/kapsam"
FUNCS = {"sqrt", "exp", "log", "sin", "cos", "tan", "atan", "asin", "acos", "atan2", "Abs", "pi"}
SAMPLED = {"normal", "rectangular", "triangular", "arcsine", "constant", "curvilinear_trapezoid"}
GUM_ONLY = {"t", "type_a", "type_a_pooled", "beta"}
N_SUNCAL = 1_000_000


def run(f):
    return json.loads(subprocess.run([BIN, "run", f, "--format", "json"], capture_output=True, text=True, check=True).stdout)


def centre_half(d):
    if d.get("half_width") is not None:
        return d.get("value", 0.0), d["half_width"]
    return 0.5 * (d["lower"] + d["upper"]), 0.5 * (d["upper"] - d["lower"])


def beta_sd(d):
    if d.get("sd") is not None:
        return d["mean"], d["sd"]
    a, b = d["alpha"], d["beta"]
    lo, hi = d.get("lower", 0.0), d.get("upper", 1.0)
    w = hi - lo
    return lo + w * a / (a + b), w * math.sqrt(a * b / ((a + b) ** 2 * (a + b + 1)))


def translate(expr, names):
    """The program's expression syntax in Suncal's (SymPy) syntax, inputs renamed v0, v1, …"""
    e = expr.replace("^", "**")
    e = re.sub(r"\blog10\(", "(1/log(10))*log(", e)
    e = re.sub(r"\bln\(", "log(", e)
    e = re.sub(r"\babs\(", "Abs(", e)
    for name in sorted(names, key=len, reverse=True):
        e = re.sub(rf"(?<![\w.]){re.escape(name)}(?![\w])", names[name], e)
    words = re.findall(r"[A-Za-z_]\w*", e)
    if any(w not in FUNCS and not re.fullmatch(r"v\d+|e\d*", w) for w in words):
        return None
    return e


def substitute(expr, intermediates):
    """Writes the intermediate quantities into an expression (they may refer to earlier ones)."""
    for _ in range(len(intermediates) + 1):
        changed = False
        for q in sorted(intermediates, key=lambda q: len(q["name"]), reverse=True):
            new = re.sub(rf"(?<![\w.]){re.escape(q['name'])}(?![\w])", f"({q['expression']})", expr)
            changed |= new != expr
            expr = new
        if not changed:
            return expr
    return None


def build(m, exprs):
    """A Suncal model for the inputs of `m` and the output expressions `exprs` (name → expression). Returns
    (model, sampled, features) or a reason the project is skipped."""
    if any(k in m for k in ("ensembles", "implicit", "indications")):
        return "ensembles or implicit model"
    names = {i["name"]: f"v{k}" for k, i in enumerate(m["inputs"])}
    funcs = []
    for out, e in exprs.items():
        e = substitute(e, m.get("intermediates", []))
        e = e and translate(e, names)
        if e is None:
            return "expression"
        funcs.append(f"{re.sub(r'[^A-Za-z0-9]', '_', 'f_' + out)} = {e}")
    types = {i["distribution"]["type"] for i in m["inputs"]}
    if not types <= SAMPLED | GUM_ONLY:
        return "distribution " + ", ".join(sorted(types - SAMPLED - GUM_ONLY))
    model = suncal.Model(*funcs)
    sampled = types <= SAMPLED
    for i in m["inputs"]:
        d, s, t = i["distribution"], names[i["name"]], i["distribution"]["type"]
        if s not in model.variables.names:  # input that does not enter the outputs
            continue
        if t == "constant":
            model.var(s).measure(d["value"])
        elif t == "normal":
            sd = d["sd"] if d.get("sd") is not None else d["expanded"] / d["k"]
            model.var(s).measure(d["mean"]).typeb(dist="normal", std=sd)
        elif t in ("rectangular", "triangular", "arcsine"):
            c, a = centre_half(d)
            model.var(s).measure(c).typeb(dist={"rectangular": "uniform"}.get(t, t), a=a)
        elif t == "curvilinear_trapezoid":
            c, a = centre_half(d)
            model.var(s).measure(c).typeb(dist="curvtrap", a=a, d=d["d"])
        elif t == "t":
            sc = d["scale"] if d.get("scale") is not None else d["expanded"] / d["k"]
            model.var(s).measure(d["mean"]).typeb(dist="normal", std=sc)
        elif t == "type_a_pooled":
            model.var(s).measure(d["mean"]).typeb(dist="normal", std=d["pooled_sd"] / math.sqrt(d["n"]))
        elif t == "type_a":
            v = d["observations"]
            model.var(s).measure(statistics.mean(v)).typeb(dist="normal", std=statistics.stdev(v) / math.sqrt(len(v)))
        elif t == "beta":
            mean, sd = beta_sd(d)
            model.var(s).measure(mean).typeb(dist="normal", std=sd)
    features = []
    if m.get("intermediates"):
        features.append("intermediates")
    corr = [c for c in m.get("correlations", []) if names[c["a"]] in model.variables.names and names[c["b"]] in model.variables.names]
    if corr:
        features.append("correlations")
        for c in corr:
            model.variables.correlate(names[c["a"]], names[c["b"]], c["r"])
            kinds = {t for i in m["inputs"] if i["name"] in (c["a"], c["b"]) for t in [i["distribution"]["type"]]}
            sampled &= kinds <= {"normal"}
    return model, sampled, features


def z_score(rel, kurt, n1):
    """Relative difference of two sample standard deviations divided by its standard error
    √((κ + 2)/(4N₁) + (κ + 2)/(4N₂)), κ the excess kurtosis of the output."""
    return rel / math.sqrt((kurt + 2) / (4 * n1) + (kurt + 2) / (4 * N_SUNCAL))


def models(skipped):
    rows = []
    for f in sorted(glob.glob("library/*/*.json")):
        p = json.load(open(f, encoding="utf-8"))
        m = p.get("model")
        if not m:
            continue
        b = build(m, {m["output"]["name"]: m["output"]["expression"]})
        if isinstance(b, str):
            skipped.append((p["metadata"]["id"], b))
            continue
        model, sampled, features = b
        try:
            g = model.calculate_gum()
            us = float(list(g.uncertainty.values())[0])
            ys = float(list(g.expected.values())[0])
            ms = model.monte_carlo(samples=N_SUNCAL) if sampled else None
        except Exception as ex:  # noqa: BLE001
            skipped.append((p["metadata"]["id"], f"Suncal: {ex}"))
            continue
        out = run(f)
        gum = out.get("gum") or {}
        uo = gum.get("first_order_uncertainty", gum.get("standard_uncertainty"))
        yo = gum.get("estimate")
        if yo is None or uo is None:
            skipped.append((p["metadata"]["id"], "Monte Carlo only"))
            continue
        r = {"id": p["metadata"]["id"], "features": features, "y": yo, "dy": (yo - ys) / us if us else 0.0, "u": uo,
             "du": uo / us - 1 if us else 0.0}
        mo = out.get("mcm")
        if ms is not None and mo is not None:
            r["mc"] = mo["standard_uncertainty"] / float(list(ms.uncertainty.values())[0]) - 1
            r["z"] = z_score(r["mc"], mo.get("excess_kurtosis") or 0.0, mo["trials"])
        rows.append(r)
    return rows


def multivariate(skipped):
    rows = []
    for f in sorted(glob.glob("library/*/*.json")):
        p = json.load(open(f, encoding="utf-8"))
        m = p.get("multivariate")
        if not m:
            continue
        b = build(m, {o["name"]: o["expression"] for o in m["outputs"]})
        if isinstance(b, str):
            skipped.append((p["metadata"]["id"], b))
            continue
        model, sampled, features = b
        try:
            g = model.calculate_gum()
            fn = list(g.uncertainty.keys())
            us = np.array([float(g.uncertainty[k]) for k in fn])
            ys = np.array([float(g.expected[k]) for k in fn])
            cs = g.correlation()
            rs = np.array([[float(cs[a][b]) for b in fn] for a in fn])
            ms = model.monte_carlo(samples=N_SUNCAL) if sampled else None
        except Exception as ex:  # noqa: BLE001
            skipped.append((p["metadata"]["id"], f"Suncal: {ex}"))
            continue
        out = run(f)["multivariate"]
        go = out["gum"]
        uo, yo, ro = np.array(go["standard_uncertainties"]), np.array(go["estimate"]), np.array(go["correlation"])
        r = {"id": p["metadata"]["id"], "features": features, "n": len(fn),
             "dy": float(np.max(np.abs((yo - ys) / np.where(us > 0, us, 1)))),
             "du": float(np.max(np.abs(np.where(us > 0, uo / np.where(us > 0, us, 1) - 1, 0)))),
             "dr": float(np.max(np.abs(ro - rs)))}
        mo = out.get("mcm")
        if ms is not None and mo is not None:
            umc = np.array([float(ms.uncertainty[k]) for k in fn])
            rel = np.array(mo["standard_uncertainties"]) / umc - 1
            k = int(np.argmax(np.abs(rel)))
            r["mc"] = float(rel[k])
            r["z"] = z_score(r["mc"], 0.0, mo["trials"])
        rows.append(r)
    return rows


def calibration(skipped):
    rows = []
    for f in sorted(glob.glob("library/*/*.json")):
        p = json.load(open(f, encoding="utf-8"))
        c = p.get("calibration")
        if not c:
            continue
        if c.get("covariances"):
            skipped.append((p["metadata"]["id"], "covariances between calibration points"))
            continue
        # The analysis function gives x (content) from y (response): fit x against y.
        y = np.array([q["y"] for q in c["points"]])
        uy = np.array([q["u_y"] for q in c["points"]])
        x = np.array([q["x"] for q in c["points"]])
        ux = np.array([q["u_x"] for q in c["points"]])
        out = run(f)["calibration"]
        b0 = np.array(out["params"])
        fun = c.get("function", "linear")
        if fun == "linear":
            (slope, icept), cov = curvefit.linefitYork(y, x, sigx=uy, sigy=ux)
            bs, cs = np.array([icept, slope]), np.asarray(cov)[::-1, ::-1]
            method = "York"
        else:
            f_ = {"quadratic": lambda t, a, b, q: a + b * t + q * t * t,
                  "cubic": lambda t, a, b, q, s: a + b * t + q * t * t + s * t ** 3,
                  "power": lambda t, a, b, q: a + b * np.power(t, 1 + q),
                  "exponential": lambda t, a, b, q: a + b * np.exp(q * t)}[fun]
            bs, cs = curvefit.odrfit(f_, y, x, uy, ux, p0=b0)
            bs, cs = np.asarray(bs), np.asarray(cs)
            method = "ODR"
        ub0, ubs = np.sqrt(np.diag(out["covariance"])), np.sqrt(np.diag(cs))

        def predict(b, cov, yy, u_yy):
            if fun in ("linear", "quadratic", "cubic"):
                deg = {"linear": 1, "quadratic": 2, "cubic": 3}[fun]
                g = np.array([yy ** j for j in range(deg + 1)])
                d = sum(j * b[j] * yy ** (j - 1) for j in range(1, deg + 1))
            elif fun == "power":
                pw = yy ** (1 + b[2])
                g = np.array([1.0, pw, b[1] * pw * math.log(yy)])
                d = b[1] * (1 + b[2]) * yy ** b[2]
            else:
                ex = math.exp(b[2] * yy)
                g = np.array([1.0, ex, b[1] * yy * ex])
                d = b[1] * b[2] * ex
            return math.sqrt(g @ cov @ g + (d * u_yy) ** 2)

        du_s = [predict(bs, cs, s["y"], s["u_y"]) for s in c.get("samples", [])]
        uo_s = [s["u"] for s in out["samples"]]
        rows.append({"id": p["metadata"]["id"], "function": fun, "method": method,
                     "db": float(np.max(np.abs((b0 - bs) / ubs))),
                     "du": float(np.max(np.abs(ub0 / ubs - 1))),
                     "ds": max((abs(a / b - 1) for a, b in zip(uo_s, du_s)), default=None), "ns": len(du_s)})
    return rows


def conformity(skipped):
    rows = []
    for f in sorted(glob.glob("library/*/*.json")):
        p = json.load(open(f, encoding="utf-8"))
        c = p.get("conformity")
        if not c:
            continue
        out = run(f)["conformity"]
        lo = out["tolerance"]["lower"]
        hi = out["tolerance"]["upper"]
        lo_, hi_ = (-np.inf if lo is None else lo), (np.inf if hi is None else hi)
        pid = p["metadata"]["id"]
        if c.get("distribution") == "lognormal" or out.get("posterior"):
            skipped.append((pid, "specific risks: lognormal measurement or posterior with a process prior"))
        else:
            for res in out["results"]:
                if res.get("conformance_probability") is None or not res.get("u"):
                    continue
                dist = (stats.t(res["dof"], loc=res["value"], scale=res["u"]) if res.get("dof")
                        else stats.norm(res["value"], res["u"]))
                ps = 1 - risk.specific_risk(dist, lo_, hi_).total
                rows.append({"id": pid, "what": f"p_c, {res['name']}", "ours": res["conformance_probability"], "suncal": ps})
        gl, proc = out.get("global"), c.get("process")
        if gl and proc:
            if c.get("u") is None:
                skipped.append((pid, "global risks: uncertainty relative to the value"))
                continue
            if proc["distribution"] == "normal":
                dp = stats.norm(proc["mean"], proc["sd"])
            elif proc["distribution"] == "gamma":
                shape, scale = (proc["mean"] / proc["sd"]) ** 2, proc["sd"] ** 2 / proc["mean"]
                dp = stats.gamma(shape, scale=scale)
            else:
                skipped.append((pid, f"global risks: {proc['distribution']} process"))
                continue
            acc = out["acceptance"]
            gbl = 0.0 if acc["lower"] is None else acc["lower"] - lo_
            gbu = 0.0 if acc["upper"] is None else hi_ - acc["upper"]
            dt = stats.norm(0, c["u"])
            rows.append({"id": pid, "what": "consumer risk", "ours": gl["consumer_risk"], "suncal": risk.PFA(dp, dt, lo_, hi_, gbl, gbu)})
            rows.append({"id": pid, "what": "producer risk", "ours": gl["producer_risk"], "suncal": risk.PFR(dp, dt, lo_, hi_, gbl, gbu)})
    return rows


def main():
    skipped = []
    mrows = models(skipped)
    vrows = multivariate(skipped)
    crows = calibration(skipped)
    krows = conformity(skipped)
    n_projects = len(mrows) + len(vrows) + len(crows) + len({r["id"] for r in krows})
    print(f"Suncal {suncal.__version__}; {n_projects} projects compared ({len(mrows)} measurement models, {len(vrows)} "
          f"multivariate models, {len(crows)} calibrations, {len({r['id'] for r in krows})} conformity assessments).\n")

    print("## 1. Measurement models\n")
    print("| Project | features | estimate | Δy/u | u (first order) | u/u_Suncal − 1 | MCM s.d./Suncal MC − 1 | z |")
    print("|---|---|---:|---:|---:|---:|---:|---:|")
    for r in mrows:
        mc = "—" if r.get("mc") is None else f"{r['mc']:+.2e}"
        z = "—" if r.get("z") is None else f"{r['z']:+.1f}"
        print(f"| {r['id']} | {', '.join(r['features']) or '—'} | {r['y']:.6g} | {r['dy']:+.1e} | {r['u']:.4g} | "
              f"{r['du']:+.1e} | {mc} | {z} |")
    du = [abs(r["du"]) for r in mrows]
    zs = [abs(r["z"]) for r in mrows if r.get("z") is not None]
    mcs = [abs(r["mc"]) for r in mrows if r.get("mc") is not None]
    print(f"\nmax |u/u_Suncal − 1| = {max(du):.1e}; max |Δy|/u = {max(abs(r['dy']) for r in mrows):.1e}; "
          f"with correlations or intermediates: {sum(1 for r in mrows if r['features'])}; "
          f"MC compared: {len(mcs)}, max |Δs.d.|/s.d. = {max(mcs):.1e}, max |z| = {max(zs):.1f}")

    print("\n## 2. Models with several outputs (JCGM 102)\n")
    print("| Project | outputs | max Δy/u | max |u/u_Suncal − 1| | max |Δr| | MCM s.d./Suncal MC − 1 (largest) | z |")
    print("|---|---:|---:|---:|---:|---:|---:|")
    for r in vrows:
        mc = "—" if r.get("mc") is None else f"{r['mc']:+.2e}"
        z = "—" if r.get("z") is None else f"{r['z']:+.1f}"
        print(f"| {r['id']} | {r['n']} | {r['dy']:.1e} | {r['du']:.1e} | {r['dr']:.1e} | {mc} | {z} |")
    if vrows:
        print(f"\nmax |u/u_Suncal − 1| = {max(r['du'] for r in vrows):.1e}; max |Δr| = {max(r['dr'] for r in vrows):.1e}")
        zs += [abs(r["z"]) for r in vrows if r.get("z") is not None]

    print("\n## 3. Calibration with uncertainties in both variables (ISO 6143)\n")
    print("| Project | function | Suncal method | max |Δb|/u(b) | max |u(b)/u_Suncal(b) − 1| | samples | max |u(x)/u_Suncal(x) − 1| |")
    print("|---|---|---|---:|---:|---:|---:|")
    for r in crows:
        ds = "—" if r["ds"] is None else f"{r['ds']:.1e}"
        print(f"| {r['id']} | {r['function']} | {r['method']} | {r['db']:.1e} | {r['du']:.1e} | {r['ns']} | {ds} |")

    print("\n## 4. Conformity assessment (JCGM 106)\n")
    print("| Project | quantity | KAPSAM | Suncal | relative difference |")
    print("|---|---|---:|---:|---:|")
    for r in krows:
        rel = abs(r["ours"] / r["suncal"] - 1) if r["suncal"] else abs(r["ours"])
        r["rel"] = rel
        print(f"| {r['id']} | {r['what']} | {r['ours']:.6g} | {r['suncal']:.6g} | {rel:.1e} |")
    if krows:
        print(f"\nmax relative difference = {max(r['rel'] for r in krows):.1e} ({len(krows)} values)")

    if zs:
        phi = 0.5 * math.erfc(max(zs) / math.sqrt(2))
        print(f"\nMonte Carlo, all sections: {len(zs)} comparisons, max |z| = {max(zs):.1f}; "
              f"P(max |z| ≥ {max(zs):.1f} among {len(zs)} independent N(0, 1)) = {1 - (1 - 2 * phi) ** len(zs):.2f}")

    print("\n## Not compared\n")
    print("| Project | reason |")
    print("|---|---|")
    for pid, why in skipped:
        print(f"| {pid} | {why} |")


if __name__ == "__main__":
    main()
