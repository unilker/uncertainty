"""Figures of the article (docs/article/manuscript.md), drawn from generated data only: the program's JSON output
for the library projects and the tables of docs/article/numbers.md. Run from the repository root after
`cargo build -p uncertainty-cli` and `python3 tools/research/article_stats.py`; needs matplotlib.

Writes docs/article/figures/fig{1,3,4}.{pdf,png} (Fig. 2, a screenshot of the application, is taken from the app):
  Fig. 1  architecture of the program;
  Fig. 3  findings per source document by class (a, b, c, n);
  Fig. 4  (a) consensus value of G by procedure (Merkatas et al. 2019 data);
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
EXE = "target/debug/kapsam"
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
    """Architecture: data pipeline (left), engine with its evaluation types (centre), interfaces (right) and the
    validation that runs at every change (bottom band), drawn as one loop."""
    NAVY, NAVY2, PAPER, LINE = "#1b2f5e", "#e9edf6", "#f6f5f2", "#c9c7c1"
    GREEN, GREEN2 = "#13805a", "#e6f4ee"
    fig, ax = plt.subplots(figsize=(7.2, 4.9))
    fig.subplots_adjust(0, 0, 1, 1)
    ax.set_xlim(0, 100)
    ax.set_ylim(0, 68)
    ax.axis("off")

    def card(x, y, w, h, face=PAPER, edge=LINE, lw=0.7, r=1.0, z=1):
        ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle=f"round,pad=0,rounding_size={r}", linewidth=lw,
                                    edgecolor=edge, facecolor=face, zorder=z))

    def arrow(p1, p2, color=INK2, lw=0.9):
        ax.add_patch(FancyArrowPatch(p1, p2, arrowstyle="-|>", mutation_scale=8, linewidth=lw, color=color,
                                     zorder=4, shrinkA=0, shrinkB=0))

    def label(x, y, text, size=5.4, color=INK2, weight="normal", ha="center", style="normal"):
        ax.text(x, y, text, fontsize=size * 1.2, color=color, fontweight=weight, ha=ha, va="center", style=style,
                linespacing=1.3, zorder=5)

    for x, text in ((12.5, "LIBRARY"), (50, "ENGINE"), (87.5, "USE")):
        label(x, 66.2, text, size=6.0, weight="bold")

    def column(x0, items, stack_last=False):
        for k, (y, title, body) in enumerate(items):
            if stack_last and k == len(items) - 1:
                card(x0 + 0.9, y - 0.9, 21, 11, face="white")
                card(x0 + 0.45, y - 0.45, 21, 11, face="white")
            card(x0, y, 21, 11, z=2)
            label(x0 + 10.5, y + 8.0, title, size=6.6, color=INK, weight="bold")
            label(x0 + 10.5, y + 3.7, body)

    # Left: from the guidance to the project files.
    column(2, [(51, "38 documents", "GUM suite, Eurachem, Nordtest,\nEA, UKAS, EURAMET, ISO, NIST"),
               (37.5, "Generators", "Python scripts: the numbers\nand clause of each example"),
               (24, "286 projects", "JSON: inputs, sources and\nreference values ± tolerance")], True)
    arrow((12.5, 50.8), (12.5, 48.7))
    arrow((12.5, 37.3), (12.5, 35.2))

    # Centre: the engine and its evaluation types.
    card(28.5, 20.0, 43, 43.5, face=NAVY2, edge=NAVY, lw=1.0, r=1.4)
    ax.add_patch(FancyBboxPatch((28.5, 57.0), 43, 6.5, boxstyle="round,pad=0,rounding_size=1.4", linewidth=0,
                                facecolor=NAVY, zorder=2))
    ax.add_patch(plt.Rectangle((28.5, 57.0), 43, 2.0, facecolor=NAVY, linewidth=0, zorder=2))
    label(50, 60.3, "KAPSAM engine  ·  Rust", size=7.6, color="white", weight="bold")
    label(50, 54.3, "one engine for every interface, so that all give the same numbers", size=5.1, color=NAVY)
    chips = [("GUM, LPU", "JCGM 100"), ("Monte Carlo", "JCGM 101"), ("Several outputs", "JCGM 102"),
             ("Conformity", "JCGM 106"), ("Top-down", "Nordtest, MUPR"), ("Proficiency", "ISO 13528"),
             ("Calibration", "ISO 6143"), ("Precision", "ISO 5725"), ("Control charts", "Nordtest TR 569"),
             ("Sampling", "Eurachem UfS"), ("Weighing", "EURAMET cg-18"), ("Consensus", "NIST NICOB")]
    cw, ch, gx, gy = 13.0, 6.2, 1.0, 1.0
    for n, (name, ref) in enumerate(chips):
        col, row = n % 3, n // 3
        x = 29.5 + col * (cw + gx)
        y = 45.2 - row * (ch + gy)
        card(x, y, cw, ch, face="white", edge="#aeb8cf", lw=0.6, r=0.8, z=3)
        label(x + cw / 2, y + 4.2, name, size=5.6, color=NAVY, weight="bold")
        label(x + cw / 2, y + 1.8, ref, size=4.8)
    label(50, 22.3, "each report names version, commit and SHA-256 of the project", size=5.0, color=NAVY,
          style="italic")
    arrow((23.3, 29.5), (28.2, 29.5), color=NAVY)
    label(25.75, 31.3, "run", size=5.0, color=NAVY)

    # Right: how it is used.
    column(77, [(51, "Command line", "kapsam run, kapsam check;\nvalidation report (17025)"),
                (37.5, "Application", "Windows, Linux, macOS,\nAndroid, iOS; guide, library"),
                (24, "Reports, EN and TR", "Markdown, HTML, PDF, JSON;\nspreadsheet workbook (.xlsx)")])
    for y in (56.5, 43.0, 29.5):
        arrow((71.8, y), (76.7, y), color=NAVY)

    # Bottom: the validation; two checks run at every change of the code, two are benchmarks of the release.
    card(2, 1.0, 96, 15.0, face=GREEN2, edge=GREEN, lw=0.9, r=1.2)
    label(50, 14.2, "VALIDATION", size=5.8, color=GREEN, weight="bold")
    tiles = [("every change (CI)", "1865", "reference values of 286 projects,\nall within their tolerances"),
             ("release benchmark", "171", "projects reproduced by an\nindependent program (Suncal)"),
             ("every change (CI)", "189", "spreadsheet workbooks,\n4678 values (LibreOffice)"),
             ("release benchmark", "8 × 10⁶", "Monte Carlo draws through the\nfit of 8 calibrations (ISO 6143)")]
    tw, gap = 22.5, 1.3
    for n, (when, big, text) in enumerate(tiles):
        x = 3.15 + n * (tw + gap)
        card(x, 2.0, tw, 10.4, face="white", edge="#b9dccb", lw=0.6, r=0.8, z=3)
        label(x + tw / 2, 10.9, when.upper(), size=4.2, color=GREEN if "CI" in when else INK2, weight="bold")
        label(x + tw / 2, 8.2, big, size=8.2, color=GREEN, weight="bold")
        label(x + tw / 2, 4.3, text, size=4.9)
    arrow((50, 19.7), (50, 16.3), color=GREEN, lw=1.0)
    arrow((12.5, 22.8), (12.5, 16.3), color=GREEN, lw=1.0)
    label(13.5, 19.6, "compare with the\nprinted results", size=4.9, color=GREEN, ha="left")
    label(51.0, 18.0, "recompute", size=4.9, color=GREEN, ha="left")
    save(fig, "fig1_architecture")


# --- Fig. 3: findings per document by class ----------------------------------------------------------------------
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
    save(fig, "fig3_findings_by_document")


# --- Fig. 4: procedures --------------------------------------------------------------------------------------------
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
    save(fig, "fig4_procedures")


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
