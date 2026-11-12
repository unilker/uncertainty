"""Monte Carlo reference values under other seeds (internal review, D110, D112): every library project with a Monte
Carlo check is run with ten seeds other than its own (101-110), and again with ten seeds that were not used to set any
tolerance (201-210), and every Monte Carlo result is compared with its reference value and tolerance. Writes
docs/research/mc-seeds.md.

    cargo build --release -p uncertainty-cli
    python3 tools/research/mc_seeds.py
"""
import glob
import json
import statistics
import subprocess
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

EXE = "target/release/kapsam"
SEEDS = range(101, 111)
FRESH = list(range(201, 211)) + list(range(301, 311))  # not used to set any tolerance (R1.5; D112)
# The seven values whose tolerances were reset from the program's seed and the seeds 101-110 (D110).
RESET = [("gum5-e14-radioactive-decay-bi207", "mcm.estimate"), ("gum5-e15-wire-rope-breaking-force", "mcm.estimate"),
         ("num-3-ratio-abc", "mcm.interval_low"), ("num-8-end-gauge-gaussian", "mcm.estimate"),
         ("nist-tn1900-e8-molecular-weight-co2", "mcm.interval_high"),
         ("jcgm102-9.4-resistance-reactance", "multivariate.mcm.r("),
         ("jcgm102-9.4-resistance-reactance-alternative", "multivariate.mcm.r(")]


def is_mc(what):
    return what.startswith("mcm.") or ".mcm" in what or "bootstrap" in what or "linear_pool" in what


def run(job):
    f, seed = job
    r = subprocess.run([EXE, "run", f, "--format", "json", "--seed", str(seed)], capture_output=True, text=True)
    if r.returncode != 0:
        return f, seed, None
    return f, seed, [c for c in json.loads(r.stdout).get("checks", []) if is_mc(c["what"])]


files = []
for f in sorted(glob.glob("library/*/*.json")):
    r = subprocess.run([EXE, "run", f, "--format", "json"], capture_output=True, text=True)
    if r.returncode == 0 and any(is_mc(c["what"]) for c in json.loads(r.stdout).get("checks", [])):
        files.append(f)


def evaluate(seeds):
    with ThreadPoolExecutor(max_workers=8) as ex:
        out = list(ex.map(run, [(f, s) for f in files for s in seeds]))
    ratios, fails, errors, by_value = [], [], 0, {}
    for f, seed, checks in out:
        if checks is None:
            errors += 1
            continue
        for c in checks:
            if c["tolerance"] > 0:
                q = abs(c["computed"] - c["expected"]) / c["tolerance"]
            else:  # a yes/no outcome (the GUM validation): must not change with the seed
                q = 0.0 if c["computed"] == c["expected"] else float("inf")
            ratios.append(q)
            key = (Path(f).stem, c["what"])
            by_value[key] = max(by_value.get(key, 0.0), q)
            if q > 1:
                fails.append((f, seed, c["what"], c["computed"], c["expected"], c["tolerance"], q))
    ratios.sort()
    return ratios, fails, errors, by_value


def summary(ratios, fails, errors, seeds):
    pct = lambda p: ratios[min(len(ratios) - 1, int(p * len(ratios)))]  # noqa: E731
    return [f"- comparisons: {len(ratios)} ({len(ratios) // len(seeds)} Monte Carlo values × {len(seeds)} seeds); "
            f"runs that failed: {errors}",
            f"- |Δ|/tolerance: median {statistics.median(ratios):.3f}, 90 % {pct(0.9):.3f}, 99 % {pct(0.99):.3f}, "
            f"maximum {ratios[-1]:.3f}",
            f"- outside the tolerance: {len(fails)}"]


def fail_table(fails):
    if not fails:
        return []
    return (["", "| Project | Seed | Value | Computed | Reference | Tolerance | ∣Δ∣/tol |", "|---|---|---|---|---|---|---|"]
            + [f"| {Path(f).stem} | {s} | {w} | {c:.6g} | {e:.6g} | {t:.3g} | {q:.2f} |" for f, s, w, c, e, t, q in fails])


ratios, fails, errors, _ = evaluate(SEEDS)
fr, ff, fe, fby = evaluate(FRESH)
lines = ["# Monte Carlo results under other seeds", "",
         "Written by `tools/research/mc_seeds.py`. Each of the " f"{len(files)} library projects with a Monte Carlo "
         f"check was run with the ten seeds {SEEDS.start}–{SEEDS.stop - 1} instead of its own, and every Monte Carlo "
         "result was compared with its reference value and tolerance.", ""]
lines += summary(ratios, fails, errors, SEEDS)
lines += ["", "History (D110, D112): the first run of this test found 20 comparisons from 7 values outside their tolerance (at "
          "most 1.26 times it): GUM-5 E14 and E15 (estimate), NUM §3 (lower interval end) and §8 (estimate), TN 1900 E8 "
          "(upper interval end) and the correlations r(R, X) and r(R, Z) of JCGM 102 9.4 (two projects). Their "
          "tolerances had been set from the program's own seed; they were reset to half a unit in the last printed "
          "digit plus five times the standard deviation over eleven seeds (the project's own and 101–110), and the run "
          "above was repeated. Because those seeds set the seven tolerances, the run above is not a test of them. A test with "
          "the ten seeds 201–210 (second internal review, D112) then found 31 comparisons outside tolerance: 24 from "
          "the Monte Carlo standard deviations and correlations of JCGM 102 9.4, whose inputs are t-distributed with "
          "ν = 3 (no finite fourth moment), and 7 from tolerances that were narrower than the rule; and 3 changes of the "
          "GUM validation outcome (NUM §10), which that run did not count. The rule was then "
          "applied to every Monte Carlo value by `tools/research/mc_tolerances.py` from the project's own seed and the "
          "seeds 101–110 only (69 tolerances widened, by at most 2.9 times); the Monte Carlo standard deviations and "
          "correlations of JCGM 102 9.4 and TN 1900 E13 (t₃ inputs) and the GUM validation outcome of NUM §10 and "
          "TN 1900 E1, which depends on the seed, are no longer reference values (14 values). The run above and the "
          "runs below are with these tolerances."]
lines += fail_table(fails)
lines += ["", "## Seeds not used to set any tolerance", "",
          "The same projects with the twenty seeds 201–210 and 301–310 (D112), which were not used to set any "
          "tolerance:", ""]
lines += summary(fr, ff, fe, FRESH)
lines += ["", "The seven values whose tolerances were reset in D110 (largest |Δ|/tolerance over the twenty seeds):", "",
          "| Project | Value | max ∣Δ∣/tol |", "|---|---|---|"]
for stem, prefix in RESET:
    keys = sorted(k for k in fby if k[0] == stem and k[1].startswith(prefix))
    q = max((fby[k] for k in keys), default=float("nan"))
    if keys:
        lines.append(f"| {stem} | {', '.join(k[1] for k in keys)} | {q:.2f} |")
    else:
        lines.append(f"| {stem} | {prefix}…) | withdrawn as reference values (t₃ inputs) |")
lines += fail_table(ff)
Path("docs/research/mc-seeds.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
print("\n".join(lines))
