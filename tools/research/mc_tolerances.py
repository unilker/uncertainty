"""Tolerances of the Monte Carlo reference values from the dispersion over seeds (pre-submission review, D112, D113).

    cargo build --release -p uncertainty-cli
    python3 tools/research/mc_tolerances.py [--write]

Every library project with a Monte Carlo check is run with its own seed and the ten seeds 101-110. For every Monte
Carlo value the rule of Section 4.2 of the article is applied, with terms that do not depend on how far the program's
result lies from the reference value (D113; the rule of D112 contained that distance and was withdrawn):

    tolerance = max(documented tolerance, h + 5 x standard deviation over the eleven seeds),

rounded up to two significant digits. The documented tolerance is the one the project had before D112 (it contains
half a unit in the last printed digit and, where the reference is itself a Monte Carlo or MCMC estimate, its sampling
error, as stated in the project); h is half a unit in the last digit of the reference value, at most the documented
tolerance. A value that falls outside this tolerance with other seeds is listed, not widened. With --write the project files in
library/ are updated; tools/research/mc_seeds.py then tests the tolerances with the seeds 201-210 and 301-310, which are
not used here. Values whose sample standard deviation has no finite variance (inputs from t-distributions with three
degrees of freedom) are not used as reference values at all: see HEAVY_TAILED.

The generators in tools/generators write the tolerances they were written with; run this script after regenerating a
project with a Monte Carlo check.
"""
import glob
from decimal import Decimal
import json
import math
import statistics
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

EXE = "target/release/kapsam"
SEEDS = [None] + list(range(101, 111))  # None: the project's own seed

# Monte Carlo standard deviations and correlations of models whose inputs are t-distributed with ν = 3: the fourth
# moment does not exist, so their sample standard deviations converge slowly and have no finite variance; with 10^6
# trials single seeds give u(R) up to 1.9 times the printed value (JCGM 102 9.4, seed 207). These are not usable as
# reference values with any fixed tolerance (D112). The estimates and the coverage intervals are kept.
HEAVY_TAILED = {
    "jcgm102-9.4-resistance-reactance": ["multivariate.mcm.u[", "multivariate.mcm.r("],
    "jcgm102-9.4-resistance-reactance-alternative": ["multivariate.mcm.u[", "multivariate.mcm.r("],
    "nist-tn1900-e13-thermal-expansion-student-95": ["mcm.standard_uncertainty"],
    "nist-tn1900-e13-thermal-expansion-student-99": ["mcm.standard_uncertainty"],
}
# Monte Carlo results compared with a printed value of another quantity: GUM-5 Example 14 prints only the GUM
# estimate and uncertainty, which differ from the Monte Carlo mean by the nonlinearity of the model (D113).
OTHER_QUANTITY = {"gum5-e14-radioactive-decay-bi207": ["mcm.estimate", "mcm.standard_uncertainty"]}
# The outcome of the GUM validation (JCGM 101 clause 8) of these projects depends on the seed (NUM §10: fails with 6 of
# 31 seeds; TN 1900 E1: 1 of 31), because the GUM and Monte Carlo intervals differ by about the numerical tolerance.
# The outcome is not a reference value there (D112).
SEED_DEPENDENT_VALIDATION = ["num-10-resistance", "nist-tn1900-e1-weighing"]


def is_mc(what):
    return what.startswith("mcm.") or ".mcm" in what or "bootstrap" in what or "linear_pool" in what


def run(job):
    f, seed = job
    args = [EXE, "run", f, "--format", "json"] + ([] if seed is None else ["--seed", str(seed)])
    r = subprocess.run(args, capture_output=True, text=True)
    if r.returncode != 0:
        raise SystemExit(f"{f} seed {seed}: {r.stderr}")
    return f, seed, [c for c in json.loads(r.stdout).get("checks", []) if is_mc(c["what"])]


def documented(f):
    """Tolerances of a project as committed before D112 (commit 43620f5^), keyed by check name; {} if it did not
    exist then."""
    r = subprocess.run(["git", "show", f"43620f5^:{f}"], capture_output=True, text=True)
    if r.returncode != 0:
        return {}
    import tempfile
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, encoding="utf-8") as fh:
        fh.write(r.stdout)
        tmp = fh.name
    out = subprocess.run([EXE, "run", tmp, "--format", "json", "--trials", "1000"], capture_output=True, text=True)
    Path(tmp).unlink()
    if out.returncode != 0:
        return {}
    return {c["what"]: c["tolerance"] for c in json.loads(out.stdout).get("checks", [])}


def half_unit(x):
    e = Decimal(repr(x)).normalize().as_tuple().exponent
    return 0.5 * 10.0 ** e if e < 0 else 0.5


def round_up(x):
    if x <= 0:
        return x
    e = math.floor(math.log10(x)) - 1
    return math.ceil(x / 10**e - 1e-9) * 10**e


def locate(expected, what):
    """The {"value", "tolerance"} entry of `expected` that a check named `what` compares with, or None."""
    parts = what.split(".")
    if parts[0] == "mcm" and len(parts) == 2:
        return expected.get("mcm", {}).get(parts[1])
    if parts[0] in ("consensus", "multicomponent_conformity", "conformity"):
        # e.g. consensus.bootstrap_u, consensus.leave_one_out[Lab 1].d, consensus.doe.linear_pool.unilateral[Lab 1].d
        import re as _re
        node = expected
        for seg in _re.findall(r"[^.\[\]]+(?:\[[^\]]+\])?", what):
            m = _re.fullmatch(r"([^\[]+)(?:\[([^\]]+)\])?", seg)
            node = node.get(m.group(1)) if isinstance(node, dict) else None
            if node is not None and m.group(2) is not None:
                node = node.get(m.group(2))
            if node is None:
                return None
        return node if isinstance(node, dict) and "tolerance" in node else None
    if parts[:2] == ["multivariate", "mcm"]:
        mc = expected.get("multivariate", {}).get("mcm", {})
        name = parts[2]
        if name.startswith("r("):
            a, b = name[2:-1].split(", ")
            return next((c for c in mc.get("correlations", []) if c["between"] in ([a, b], [b, a])), None)
        if "[" in name:
            q, out = name[:-1].split("[")
            q = {"u": "standard_uncertainty"}.get(q, q)
            return mc.get(q, {}).get(out)
        return mc.get(name)
    return None


def remove(expected, what):
    parts = what.split(".")
    if parts[0] == "mcm" and len(parts) == 2:
        expected.get("mcm", {}).pop(parts[1], None)
        return
    mc = expected["multivariate"]["mcm"]
    name = parts[2]
    if name.startswith("r("):
        a, b = name[2:-1].split(", ")
        mc["correlations"] = [c for c in mc.get("correlations", []) if c["between"] not in ([a, b], [b, a])]
        if not mc["correlations"]:
            del mc["correlations"]
        return
    q, out = name[:-1].split("[")
    q = {"u": "standard_uncertainty"}.get(q, q)
    mc[q].pop(out, None)
    if not mc[q]:
        del mc[q]


def main():
    write = "--write" in sys.argv
    files = []
    for f in sorted(glob.glob("library/*/*.json")):
        r = subprocess.run([EXE, "run", f, "--format", "json"], capture_output=True, text=True)
        if r.returncode == 0 and any(is_mc(c["what"]) for c in json.loads(r.stdout).get("checks", [])):
            files.append(f)
    with ThreadPoolExecutor(max_workers=8) as ex:
        out = list(ex.map(run, [(f, s) for f in files for s in SEEDS]))
    values, tol, ref = {}, {}, {}
    for f, seed, checks in out:
        for c in checks:
            k = (f, c["what"])
            values.setdefault(k, []).append(c["computed"])
            tol[k], ref[k] = c["tolerance"], c["expected"]
    changes, removed = {}, {}
    docs = {f: documented(f) for f in files}
    for (f, what), v in values.items():
        stem = Path(f).stem
        if any(what.startswith(p) for p in HEAVY_TAILED.get(stem, [])) or (
                what == "mcm.gum_validated" and stem in SEED_DEPENDENT_VALIDATION) or (
                what in OTHER_QUANTITY.get(stem, [])):
            removed.setdefault(f, []).append(what)
            continue
        if tol[(f, what)] == 0:  # a yes/no outcome
            continue
        base = docs[f].get(what, tol[(f, what)])
        h = min(half_unit(ref[(f, what)]), base)
        new = max(base, round_up(h + 5 * statistics.stdev(v)))
        if abs(new - tol[(f, what)]) > 1e-12 * max(1.0, abs(new)):
            changes.setdefault(f, []).append((what, tol[(f, what)], new))
    n = sum(len(c) for c in changes.values())
    print(f"{len(values)} Monte Carlo values in {len(files)} projects; {n} tolerances changed; "
          f"{sum(len(r) for r in removed.values())} values removed as reference values")
    for f in sorted(set(changes) | set(removed)):
        for what, old, new in changes.get(f, []):
            print(f"  {Path(f).stem}: {what} {old:.4g} -> {new:.2g} ({new / old:.2f}x)")
        for what in removed.get(f, []):
            print(f"  {Path(f).stem}: {what} removed")
        if not write:
            continue
        p = json.loads(Path(f).read_text(encoding="utf-8"))
        exp = p["expected"]
        for what, old, new in changes.get(f, []):
            entry = locate(exp, what)
            assert entry is not None and abs(entry["tolerance"] - old) <= 1e-12 * max(1, abs(old)), (f, what, entry)
            entry["tolerance"] = new
        for what in removed.get(f, []):
            remove(exp, what)
        Path(f).write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
