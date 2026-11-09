"""Figures of the article (docs/article/manuscript.md), drawn from generated data only: the program's JSON output
for the library projects and the tables of docs/article/numbers.md. Run from the repository root after
`cargo build -p uncertainty-cli` and `python3 tools/research/article_stats.py`; needs matplotlib.

Writes docs/article/figures/fig{1,2,3}.{pdf,png}:
  Fig. 1  architecture of the program;
  Fig. 2  findings per source document by class (a, b, c, n);
  Fig. 3  (a) consensus value of G by procedure (Merkatas et al. 2019 data);
          (b) leave-one-out degrees of equivalence of the 60Co comparison by procedure (NICOB manual data).

Colours: the validated categorical palette of the data-visualisation guidance (slots 1-4, fixed order, checked
with its validator for adjacent pairs and, for the three procedures, all pairs); identity is never carried by
colour alone (legends, direct labels and the tables of numbers.md).
"""
import json
import re
import subprocess
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.patches import FancyArrowPatch, FancyBboxPatch  # noqa: E402

OUT = Path("docs/article/figures")
OUT.mkdir(parents=True, exist_ok=True)
EXE = "target/debug/uncertainty"
SLOTS = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]
INK, INK2, GRID = "#0b0b0b", "#52514e", "#e4e3df"

plt.rcParams.update({
    "font.family": "DejaVu Sans", "font.size": 8, "axes.edgecolor": INK2, "axes.labelcolor": INK,
    "xtick.color": INK2, "ytick.color": INK2, "axes.spines.top": False, "axes.spines.right": False,
    "axes.linewidth": 0.6, "xtick.major.width": 0.6, "ytick.major.width": 0.6, "legend.frameon": False,
    "pdf.fonttype": 42, "svg.fonttype": "none",
})


def save(fig, name):
    fig.savefig(OUT / f"{name}.pdf", bbox_inches="tight")
    fig.savefig(OUT / f"{name}.png", dpi=300, bbox_inches="tight")
    plt.close(fig)
    print("wrote", OUT / f"{name}.pdf")


def run_json(project):
    out = subprocess.run([EXE, "run", project, "--format", "json"], capture_output=True, text=True, check=True).stdout
    j = json.loads(out)
    return j.get("consensus", j)


# --- Fig. 1: architecture ----------------------------------------------------------------------------------------
def fig1():
    fig, ax = plt.subplots(figsize=(7.0, 3.3))
    ax.set_xlim(0, 100)
    ax.set_ylim(0, 46)
    ax.axis("off")

    def box(x, y, w, h, title, body, accent=False):
        ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.4,rounding_size=1.2", linewidth=0.8,
                                    edgecolor=SLOTS[0] if accent else INK2, facecolor="#eef4fc" if accent else "#f6f5f2"))
        ax.text(x + w / 2, y + h - 2.2, title, ha="center", va="top", fontsize=8, fontweight="bold", color=INK)
        ax.text(x + w / 2, y + h - 6.2, body, ha="center", va="top", fontsize=6.6, color=INK2, linespacing=1.35)

    def arrow(x1, y1, x2, y2, text=None):
        ax.add_patch(FancyArrowPatch((x1, y1), (x2, y2), arrowstyle="-|>", mutation_scale=8, linewidth=0.8, color=INK2))
        if text:
            ax.text((x1 + x2) / 2, (y1 + y2) / 2 + 1.2, text, ha="center", fontsize=6.2, color=INK2)

    box(2, 27, 25, 17, "Sources", "guidance documents,\nstandards, papers\n(numerical data only)")
    box(37, 27, 26, 17, "Generators", "Python scripts write each\nproject with its source,\nexpected values, tolerances")
    box(73, 27, 25, 17, "Project files", "JSON: inputs, metadata\n(EN/TR), reference values\n286 in the library")
    box(35, 2, 30, 20, "Engine (Rust)", "core: GUM, MCM, JCGM 102, 106,\nconsensus, top-down, ISO 13528,\nISO 6143, ISO 5725, TR 569, UfS\nproject: run, check, reports", accent=True)
    box(2, 2, 25, 20, "Tests", "cargo test reruns all\nprojects (1865 values);\nSuncal benchmark;\nindependent recomputations")
    box(73, 2, 25, 20, "Interfaces", "CLI, C interface, app\n(desktop, mobile);\nreports in Markdown,\nHTML, PDF, JSON")
    arrow(27.5, 35.5, 36.5, 35.5)
    arrow(63.5, 35.5, 72.5, 35.5)
    arrow(80, 26.5, 60, 22.6)
    ax.text(73, 25.4, "evaluate", ha="right", fontsize=6.2, color=INK2)
    arrow(65.5, 12, 72.5, 12)
    arrow(34.5, 12, 27.5, 12)
    ax.text(31, 14.6, "compare", ha="center", fontsize=6.2, color=INK2)
    save(fig, "fig1_architecture")


# --- Fig. 2: findings per document by class ----------------------------------------------------------------------
def fig2():
    md = Path("docs/article/numbers.md").read_text(encoding="utf-8")
    sec = md.split("## Findings by document and class")[1].split("\n## ")[0]
    rows = []
    for line in sec.splitlines():
        m = re.match(r"\| (.+?) \| (\d+) \| (\d+) \| (\d+) \| (\d+) \|$", line)
        if m and m.group(1) != "Document":
            rows.append((m.group(1), *map(int, m.groups()[1:])))
    rows = [r for r in rows if r[0] not in ("summary", "methodological")]
    rows.sort(key=lambda r: sum(r[1:]))
    # Documents with fewer than three entries are folded into "Other documents".
    big = [r for r in rows if sum(r[1:]) >= 3]
    small = [r for r in rows if sum(r[1:]) < 3]
    other = ("Other documents (%d)" % len(small), *[sum(r[k] for r in small) for k in range(1, 5)])
    data = [other] + big
    fig, ax = plt.subplots(figsize=(6.6, 0.24 * len(data) + 1.0))
    names = [r[0] for r in data]
    left = [0] * len(data)
    labels = ["a: intermediate rounding", "b: misprint without effect", "c: inconsistency", "n: no discrepancy"]
    for k in range(4):
        vals = [r[k + 1] for r in data]
        ax.barh(names, vals, left=left, color=SLOTS[k], edgecolor="white", linewidth=1.0, height=0.68, label=labels[k])
        left = [a + b for a, b in zip(left, vals)]
    for y, tot in enumerate(left):
        ax.text(tot + 0.2, y, str(tot), va="center", fontsize=7, color=INK)
    ax.set_xlabel("Findings (a finding with two classes counts in both)")
    ax.xaxis.grid(True, color=GRID, linewidth=0.6)
    ax.set_axisbelow(True)
    ax.tick_params(axis="y", length=0)
    ax.legend(loc="center right", fontsize=7, handlelength=1.2)
    save(fig, "fig2_findings_by_document")


# --- Fig. 3: procedures --------------------------------------------------------------------------------------------
def fig3():
    g = run_json("library/physical/merkatas2019-newtonian-constant-g-shades.json")
    dl, ml, by, lp = g["dersimonian_laird"], g["maximum_likelihood"], g["bayes"], g["linear_pool"]
    proc = [
        ("DerSimonian–Laird,\nKnapp–Hartung", dl["mean"], dl["expanded_knapp_hartung"]),
        ("Maximum likelihood", ml["mean"], 1.959964 * ml["standard_uncertainty"]),
        ("Hierarchical Bayes\n(exact grid)", by["mean"], None, by["interval"]),
        ("Linear Pool", lp["mean"], None, lp["interval"]),
    ]
    co = run_json("library/physical/nicob-co60-activity-leave-one-out-bilateral.json")
    sets = [("DerSimonian–Laird", co["degrees_of_equivalence"]), ("Linear Pool", co["linear_pool_degrees_of_equivalence"]),
            ("Bayes", co["bayes_degrees_of_equivalence"])]

    fig = plt.figure(figsize=(7.0, 4.4))
    gs = fig.add_gridspec(1, 2, width_ratios=[1, 2.1], wspace=0.35)
    ax = fig.add_subplot(gs[0])
    for i, p in enumerate(proc):
        y = len(proc) - 1 - i
        lo, hi = (p[1] - p[2], p[1] + p[2]) if p[2] is not None else p[3]
        ax.plot([lo, hi], [y, y], color=SLOTS[0], linewidth=2, solid_capstyle="round")
        ax.plot(p[1], y, "o", color=SLOTS[0], markersize=5, markeredgecolor="white", markeredgewidth=1)
    ax.set_yticks(range(len(proc)))
    ax.set_yticklabels([p[0] for p in proc][::-1], fontsize=7)
    ax.set_xlabel("G / (10⁻¹¹ m³ kg⁻¹ s⁻²), 95 % interval")
    ax.xaxis.grid(True, color=GRID, linewidth=0.6)
    ax.set_axisbelow(True)
    ax.tick_params(axis="y", length=0)
    ax.set_title("(a) Consensus value of G, 16 results", fontsize=8, loc="left", color=INK)

    ax = fig.add_subplot(gs[1])
    names = [u["name"] for u in sets[0][1]["unilateral"]]
    n = len(names)
    for k, (label, s) in enumerate(sets):
        sig = sum(1 for b in s["bilateral"] if b["significant"])
        for j, u in enumerate(s["unilateral"]):
            y = n - 1 - j + (1 - k) * 0.26
            ax.plot([u["d"] - u["expanded"], u["d"] + u["expanded"]], [y, y], color=SLOTS[k], linewidth=1.6,
                    solid_capstyle="round")
            ax.plot(u["d"], y, "o", color=SLOTS[k], markersize=3.6, markeredgecolor="white", markeredgewidth=0.6,
                    label=f"{label} ({sig} of {len(s['bilateral'])} pairs significant)" if j == 0 else None)
    ax.axvline(0, color=INK2, linewidth=0.6)
    ax.set_yticks(range(n))
    ax.set_yticklabels(names[::-1], fontsize=6.5)
    ax.set_xlabel("D*ⱼ = xⱼ − μ̂₋ⱼ / kBq, U95 %")
    ax.xaxis.grid(True, color=GRID, linewidth=0.6)
    ax.set_axisbelow(True)
    ax.tick_params(axis="y", length=0)
    ax.set_title("(b) ⁶⁰Co, 19 laboratories: leave-one-out degrees of equivalence", fontsize=8, loc="left", color=INK)
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.11), fontsize=6.8, ncol=1, handlelength=1.0)
    save(fig, "fig3_procedures")


def tables():
    """Tables 2 and 3 of the article, copied from numbers.md with their captions (docs/article/tables.md)."""
    md = Path("docs/article/numbers.md").read_text(encoding="utf-8")

    def section(title):
        return md.split(f"## {title}")[1].split("\n## ")[0].strip()

    out = ["# Tables 2 and 3 of the article", "",
           "Generated by `tools/research/article_figures.py` from `numbers.md`; do not edit by hand.", "",
           "**Table 2a.** Library projects by evaluation type.", "", section("Projects by section"), "",
           "**Table 2b.** Library projects by field.", "", section("Projects by field"), "",
           "**Table 3.** From sources to findings; each step defined.", "",
           section("From sources to findings"), ""]
    Path("docs/article/tables.md").write_text("\n".join(out), encoding="utf-8")
    print("wrote docs/article/tables.md")


if __name__ == "__main__":
    tables()
    fig1()
    fig2()
    fig3()
