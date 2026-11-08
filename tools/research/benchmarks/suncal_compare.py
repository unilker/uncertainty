"""Benchmark of the program against Sandia's Suncal (PyPI `suncal`, version printed below) on the model projects
of the library that Suncal can express directly: scalar output, no correlations, intermediates or ensembles, and
inputs among normal, rectangular, triangular, arcsine, constant, t, type_a and type_a_pooled.

For every project: the first-order GUM standard uncertainty (Suncal's GUM is first order) and, where all inputs
are normal/rectangular/triangular/arcsine/constant, the Monte Carlo standard deviation with 10⁶ draws.
t and type_a inputs enter the GUM comparison with their standard uncertainty only (the Monte Carlo conventions for
t inputs differ: research log F31). Run with a Python that has suncal:
    PYTHONHASHSEED=0 python3 tools/research/benchmarks/suncal_compare.py <path to the uncertainty binary> > table.md
(Suncal draws from the global NumPy generator, seeded below; the hash seed fixes the order of its internal sets.)
"""
import json
import glob
import math
import re
import statistics
import subprocess
import sys

import numpy as np
import suncal

np.random.seed(1)  # Suncal draws from the global NumPy generator

BIN = sys.argv[1] if len(sys.argv) > 1 else "target/release/uncertainty"
FUNCS = {"sqrt", "exp", "ln", "sin", "cos", "tan", "atan", "asin", "acos", "abs", "pi"}
MC_TYPES = {"normal", "rectangular", "triangular", "arcsine", "constant"}


def half_width(d):
    if d.get("half_width") is not None:
        return d.get("value", 0.0), d["half_width"]
    return 0.5 * (d["lower"] + d["upper"]), 0.5 * (d["upper"] - d["lower"])


def main():
    rows = []
    skipped = 0
    for f in sorted(glob.glob("library/*/*.json")):
        p = json.load(open(f, encoding="utf-8"))
        m = p.get("model")
        if not m or any(k in m for k in ("correlations", "intermediates", "ensembles")):
            skipped += m is not None
            continue
        types = {i["distribution"]["type"] for i in m["inputs"]}
        if not types <= MC_TYPES | {"t", "type_a", "type_a_pooled"}:
            skipped += 1
            continue
        expr = m["output"]["expression"]
        if "log10" in expr or re.search(r"\blog\(", expr):
            skipped += 1
            continue
        names = {i["name"]: f"v{k}" for k, i in enumerate(m["inputs"])}
        consts = {}
        e = expr.replace("^", "**")
        e = re.sub(r"\bln\(", "log(", e)
        for name in sorted(names, key=len, reverse=True):
            e = re.sub(rf"(?<![\w.]){re.escape(name)}(?![\w])", names[name], e)
        if any(w not in FUNCS and not re.fullmatch(r"v\d+|log|e\d*", w) for w in re.findall(r"[A-Za-z_]\w*", e)):
            skipped += 1
            continue
        model = suncal.Model(f"y = {e}")
        mc_ok = types <= MC_TYPES
        for i in m["inputs"]:
            d, s = i["distribution"], names[i["name"]]
            t = d["type"]
            if t == "constant":
                model.var(s).measure(d["value"])
            elif t == "normal":
                sd = d["sd"] if d.get("sd") is not None else d["expanded"] / d["k"]
                model.var(s).measure(d["mean"]).typeb(dist="normal", std=sd)
            elif t in ("rectangular", "triangular", "arcsine"):
                c, a = half_width(d)
                model.var(s).measure(c).typeb(dist={"rectangular": "uniform"}.get(t, t), a=a)
            elif t == "t":
                sc = d["scale"] if d.get("scale") is not None else d["expanded"] / d["k"]
                model.var(s).measure(d["mean"]).typeb(dist="normal", std=sc)
            elif t == "type_a_pooled":
                model.var(s).measure(d["mean"]).typeb(dist="normal", std=d["pooled_sd"] / math.sqrt(d["n"]))
            elif t == "type_a":
                v = d["observations"]
                model.var(s).measure(statistics.mean(v)).typeb(dist="normal", std=statistics.stdev(v) / math.sqrt(len(v)))
        try:
            g = model.calculate_gum()
            us = float(list(g.uncertainty.values())[0])
            ys = float(list(g.expected.values())[0])
            ms = model.monte_carlo(samples=1_000_000) if mc_ok else None
        except Exception as ex:  # noqa: BLE001
            print(f"<!-- {f}: suncal failed: {ex} -->")
            skipped += 1
            continue
        out = json.loads(subprocess.run([BIN, "run", f, "--format", "json"], capture_output=True, text=True, check=True).stdout)
        gum = out.get("gum") or {}
        uo = gum.get("first_order_uncertainty", gum.get("standard_uncertainty"))
        yo = gum.get("estimate")
        if yo is None or uo is None:  # Monte Carlo-only project
            skipped += 1
            continue
        r = {"id": p["metadata"]["id"], "y": yo, "dy": (yo - ys) / us if us else 0.0, "u": uo, "du": uo / us - 1 if us else 0.0}
        if ms is not None:
            mo = out.get("mcm")
            if mo is None:
                # The project has no Monte Carlo evaluation: compare Suncal MC with our GUM u only.
                r["mc"] = None
            else:
                r["mc"] = mo["standard_uncertainty"] / float(list(ms.uncertainty.values())[0]) - 1
                # Relative standard error of a sample s.d. is about √((κ + 2)/(4N)) (κ the excess kurtosis).
                k = mo.get("excess_kurtosis") or 0.0
                se = math.sqrt((k + 2) / (4 * mo["trials"]) + (k + 2) / (4 * 1_000_000))
                r["z"] = r["mc"] / se
        rows.append(r)
    print(f"Suncal {suncal.__version__}; {len(rows)} projects compared, {skipped} model projects skipped.\n")
    print("| Project | estimate | Δy/u | u (first order) | u/u_Suncal − 1 | MCM s.d./Suncal MC − 1 | z |")
    print("|---|---:|---:|---:|---:|---:|---:|")
    for r in rows:
        mc = "—" if r.get("mc") is None else f"{r['mc']:+.2e}"
        z = "—" if r.get("z") is None else f"{r['z']:+.1f}"
        print(f"| {r['id']} | {r['y']:.6g} | {r['dy']:+.1e} | {r['u']:.4g} | {r['du']:+.1e} | {mc} | {z} |")
    du = [abs(r["du"]) for r in rows]
    mcs = [abs(r["mc"]) for r in rows if r.get("mc") is not None]
    print(f"\nmax |u/u_Suncal − 1| = {max(du):.1e}; max |Δy|/u = {max(abs(r['dy']) for r in rows):.1e}; "
          f"MC compared: {len(mcs)}, max |Δs.d.|/s.d. = {max(mcs) if mcs else float('nan'):.1e}, "
          f"max |z| = {max(abs(r['z']) for r in rows if r.get('z') is not None):.1f}")
    # Probability of a max |z| at least this large among m independent standard normal z (review 2026-11-08,
    # item 15): 1 − (1 − 2Φ(−|z|max))^m.
    zs = [abs(r["z"]) for r in rows if r.get("z") is not None]
    if zs:
        phi = 0.5 * math.erfc(max(zs) / math.sqrt(2))
        print(f"P(max |z| ≥ {max(zs):.1f} among {len(zs)} independent N(0, 1)) = {1 - (1 - 2 * phi) ** len(zs):.2f}")


if __name__ == "__main__":
    main()
