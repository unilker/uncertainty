"""Monte Carlo reference values under other seeds (internal review, D110): every library project with a Monte
Carlo check is run with ten seeds other than its own, and every Monte Carlo result is compared with its reference
value and tolerance. Writes docs/research/mc-seeds.md.

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
with ThreadPoolExecutor(max_workers=8) as ex:
    out = list(ex.map(run, [(f, s) for f in files for s in SEEDS]))
ratios, fails, errors = [], [], 0
for f, seed, checks in out:
    if checks is None:
        errors += 1
        continue
    for c in checks:
        q = abs(c["computed"] - c["expected"]) / c["tolerance"] if c["tolerance"] > 0 else 0.0
        ratios.append(q)
        if q > 1:
            fails.append((f, seed, c["what"], c["computed"], c["expected"], c["tolerance"], q))
ratios.sort()
pct = lambda p: ratios[min(len(ratios) - 1, int(p * len(ratios)))]  # noqa: E731
lines = ["# Monte Carlo results under other seeds", "",
         "Written by `tools/research/mc_seeds.py`. Each of the " f"{len(files)} library projects with a Monte Carlo "
         f"check was run with the ten seeds {SEEDS.start}–{SEEDS.stop - 1} instead of its own, and every Monte Carlo "
         "result was compared with its reference value and tolerance.", "",
         f"- comparisons: {len(ratios)} ({len(ratios) // len(SEEDS)} Monte Carlo values × {len(SEEDS)} seeds); runs that failed: {errors}",
         f"- |Δ|/tolerance: median {statistics.median(ratios):.3f}, 90 % {pct(0.9):.3f}, 99 % {pct(0.99):.3f}, "
         f"maximum {ratios[-1]:.3f}",
         f"- outside the tolerance: {len(fails)}", "",
         "History (D110): the first run of this test found 20 comparisons from 7 values outside their tolerance (at "
         "most 1.26 times it): GUM-5 E14 and E15 (estimate), NUM §3 (lower interval end) and §8 (estimate), TN 1900 E8 "
         "(upper interval end) and the correlations r(R, X) and r(R, Z) of JCGM 102 9.4 (two projects). Their "
         "tolerances had been set from the program's own seed; they were reset to half a unit in the last printed "
         "digit plus five times the standard deviation over eleven seeds, and the run above was repeated.", ""]
if fails:
    lines += ["| Project | Seed | Value | Computed | Reference | Tolerance | ∣Δ∣/tol |", "|---|---|---|---|---|---|---|"]
    lines += [f"| {Path(f).stem} | {s} | {w} | {c:.6g} | {e:.6g} | {t:.3g} | {q:.2f} |" for f, s, w, c, e, t, q in fails]
Path("docs/research/mc-seeds.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
print("\n".join(lines[:12]))
