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
EXE = next((e for e in ("target/debug/kapsam", "target/release/kapsam") if Path(e).exists()), "target/debug/kapsam")
SLOTS = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]
INK, INK2, GRID = "#0b0b0b", "#52514e", "#e4e3df"

plt.rcParams.update({
    "font.family": ["Liberation Sans", "DejaVu Sans"], "font.size": 8, "axes.edgecolor": INK2, "axes.labelcolor": INK,
    "xtick.color": INK2, "ytick.color": INK2, "axes.spines.top": False, "axes.spines.right": False,
    "axes.linewidth": 0.6, "xtick.major.width": 0.6, "ytick.major.width": 0.6, "legend.frameon": False,
    "pdf.fonttype": 42, "svg.fonttype": "none",
})


# Elsevier artwork sizing: printed lettering at least 7 pt (6 pt for sub- and superscripts) at the final width; a
# full-width figure is 190 mm. Every text of a figure is raised to 7 pt, and the saved width is checked so that the
# figure is not reduced when it is printed at 190 mm.
MIN_PT, FULL_WIDTH_IN = 7.0, 190 / 25.4


def save(fig, name):
    import matplotlib.text
    for t in fig.findobj(matplotlib.text.Text):
        if t.get_text() and t.get_fontsize() < MIN_PT:
            t.set_fontsize(MIN_PT)
    fig.canvas.draw()
    bbox = fig.get_tightbbox(fig.canvas.get_renderer())
    scale = FULL_WIDTH_IN / bbox.width
    assert scale >= 0.999, f"{name}: {bbox.width:.2f} in wide, would be reduced to {scale:.3f} at 190 mm"
    fig.savefig(OUT / f"{name}.pdf", bbox_inches="tight")
    fig.savefig(OUT / f"{name}.png", dpi=600, bbox_inches="tight")
    plt.close(fig)
    print(f"wrote {OUT / name}.pdf; {bbox.width * 25.4:.0f} mm wide, smallest text {MIN_PT * scale:.1f} pt at 190 mm")


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


# --- Fig. 3: findings per document, and the effect of the confirmed c1 findings ------------------------------------
C1_DOCS = {"JCGM GUM-5": "JCGM GUM-5:2026", "EMPIR EMUE": "EMPIR EMUE", "UKAS M3003": "UKAS M3003", "EA-4/02": "EA-4/02",
           "NIST TN 1900": "NIST TN 1900", "ISO 21748": "ISO 21748", "ISO 5725": "ISO 5725", "ISO 6143": "ISO 6143"}


def c1_findings():
    """Confirmed c1 findings (docs/research/class-c-impact.tsv, D84): F, document of Fig. 3, label, |Δy|/U, |ΔU|/U."""
    rows = [l.split("\t") for l in Path("docs/research/class-c-impact.tsv").read_text(encoding="utf-8").splitlines()[1:]]
    out = []
    for r in rows:
        f, doc, impact, status, quantity = r[0], r[1], r[2], r[3], r[4]
        if impact != "c1" or status == "open":
            continue
        key = next(k for k in C1_DOCS if doc.startswith(k))
        num = lambda v: float(v) if re.fullmatch(r"[0-9.]+", v) else None  # noqa: E731
        out.append((f, C1_DOCS[key], doc, num(r[7]), num(r[8])))
    assert len(out) == 13, len(out)
    return out


def fig2():
    md = Path("docs/article/numbers.md").read_text(encoding="utf-8")
    sec = md.split("## Findings by document and class")[1].split("\n## ")[0]
    rows = []
    for line in sec.splitlines():
        m = re.match(r"\| (.+?) \| (\d+) \| (\d+) \| (\d+) \| (\d+) \|$", line)
        if m and m.group(1) != "Document":
            rows.append((m.group(1), *map(int, m.groups()[1:])))
    rows = [r for r in rows if r[0] not in ("summary", "methodological")]
    c1 = c1_findings()
    c1_by_doc = {}
    for _, doc, *_ in c1:
        c1_by_doc[doc] = c1_by_doc.get(doc, 0) + 1
    # Documents with at least three findings, or with a confirmed c1 finding, are shown; the others are combined.
    shown = [r for r in rows if sum(r[1:]) >= 3 or r[0] in c1_by_doc]
    rest = [r for r in rows if r not in shown]
    shown.sort(key=lambda r: (sum(r[1:]), c1_by_doc.get(r[0], 0)))
    other = (f"{len(rest)} further documents", *[sum(r[k] for r in rest) for k in range(1, 5)])

    MUTED = {"a": "#c5d4ea", "b": "#e9dcc6", "n": "#dddcd7"}
    CRED, CORANGE = "#b8321f", "#f0a079"
    fig = plt.figure(figsize=(7.2, 4.7))
    gs = fig.add_gridspec(1, 2, width_ratios=[1.25, 1], wspace=0.62, top=0.93)

    ax = fig.add_subplot(gs[0])
    data = [other] + shown
    ys = [0] + [k + 1.6 for k in range(len(shown))]
    for y, r in zip(ys, data):
        a, b, c, n = r[1:]
        k1 = c1_by_doc.get(r[0], 0)
        segs = [(k1, CRED), (c - k1, CORANGE), (a, MUTED["a"]), (b, MUTED["b"]), (n, MUTED["n"])]
        x = 0
        for w, col in segs:
            if w:
                ax.barh(y, w, left=x, height=0.72, color=col, edgecolor="white", linewidth=0.8)
                x += w
        ax.text(x + 0.25, y, str(sum(r[1:])), va="center", fontsize=6.6, color=INK)
        if k1:
            ax.text(-0.35, y, f"{k1}", va="center", ha="right", fontsize=6.6, color=CRED, fontweight="bold")
    ax.set_yticks(ys)
    ax.set_yticklabels([r[0] for r in data], fontsize=6.8)
    ax.get_yticklabels()[0].set_color(INK2)
    ax.get_yticklabels()[0].set_style("italic")
    ax.axhline(0.8, color=GRID, linewidth=0.8)
    ax.set_xlim(0, 27)
    ax.set_xlabel("Findings (a finding with two classes counts in both)", fontsize=7)
    ax.xaxis.grid(True, color=GRID, linewidth=0.6)
    ax.set_axisbelow(True)
    ax.tick_params(axis="y", length=0, pad=14)
    ax.text(-0.35, ys[-1] + 1.05, "c1", ha="right", va="center", fontsize=6.4, color=CRED, fontweight="bold")
    from matplotlib.patches import Patch
    handles = [Patch(color=CRED, label="c1: changes a final printed result"),
               Patch(color=CORANGE, label="other c: c2, m, e, open"),
               Patch(color=MUTED["a"], label="a: intermediate rounding"),
               Patch(color=MUTED["b"], label="b: misprint without effect"),
               Patch(color=MUTED["n"], label="n: no discrepancy")]
    ax.legend(handles=handles, loc="lower right", bbox_to_anchor=(1.0, 0.10), fontsize=6.2, handlelength=1.0,
              borderaxespad=0.2)
    fig.text(0.01, 0.985, "(a) Findings by source document", fontsize=7.8, color=INK, va="top")

    ax = fig.add_subplot(gs[1])
    eff = sorted(c1, key=lambda r: max(v for v in r[3:] if v))
    for y, (f, doc, label, dy, du) in enumerate(eff):
        top = max(v for v in (dy, du) if v)
        ax.plot([0.004, top], [y, y], color=GRID, linewidth=1.0, zorder=1)
        if du:
            ax.plot(du, y, "D", color=CRED, markersize=4.2, markeredgecolor="white", markeredgewidth=0.6, zorder=3)
        if dy:
            ax.plot(dy, y, "o", color=INK, markersize=4.6, markeredgecolor="white", markeredgewidth=0.6, zorder=3)
    ax.set_xscale("log")
    ax.set_xlim(0.004, 80)
    ax.set_yticks(range(len(eff)))
    ax.set_yticklabels([f"{r[0]}  {r[2]}" for r in eff], fontsize=6.2)
    ax.axvline(1, color=INK2, linewidth=0.7, linestyle=(0, (3, 2)))
    ax.text(1.12, len(c1) - 0.25, "one expanded uncertainty", fontsize=5.8, color=INK2, va="center")
    ax.set_xticks([0.01, 0.1, 1, 10])
    ax.set_xticklabels(["1 %", "10 %", "1", "10"])
    ax.set_xlabel("Change of the printed result, relative to its U", fontsize=7)
    ax.xaxis.grid(True, color=GRID, linewidth=0.6, which="major")
    ax.set_axisbelow(True)
    ax.tick_params(axis="y", length=0)
    ax.set_ylim(-0.7, len(eff) + 0.2)
    ax.plot([], [], "o", color=INK, markersize=4.6, label="estimate y: |Δy|/U")
    ax.plot([], [], "D", color=CRED, markersize=4.2, label="uncertainty: |ΔU|/U")
    ax.legend(loc="lower right", bbox_to_anchor=(1.0, 0.06), fontsize=6.2, handlelength=1.0)
    fig.text(0.55, 0.985, "(b) How far the 13 confirmed c1 findings move the printed result", fontsize=7.8,
             color=INK, va="top")
    save(fig, "fig3_findings_by_document")


# --- Fig. 4: the consensus procedure is part of the model ---------------------------------------------------------
def fig3():
    import numpy as np
    from matplotlib.colors import ListedColormap
    PROC = {"DL": SLOTS[0], "ML": "#7d5ba6", "Bayes": SLOTS[2], "LP": SLOTS[1]}
    gp = "library/physical/merkatas2019-newtonian-constant-g-shades.json"
    g = run_json(gp)
    res = json.loads(Path(gp).read_text(encoding="utf-8"))["consensus"]["results"]
    dl, ml, by, lp = g["dersimonian_laird"], g["maximum_likelihood"], g["bayes"], g["linear_pool"]
    proc = [("DerSimonian–Laird", "DL", dl["mean"], dl["mean"] - dl["expanded_knapp_hartung"],
             dl["mean"] + dl["expanded_knapp_hartung"], dl["u_knapp_hartung"]),
            ("Max. likelihood", "ML", ml["mean"], ml["mean"] - 1.959964 * ml["standard_uncertainty"],
             ml["mean"] + 1.959964 * ml["standard_uncertainty"], ml["standard_uncertainty"]),
            ("Hierarchical Bayes", "Bayes", by["mean"], *by["interval"], by["standard_uncertainty"]),
            ("Linear Pool", "LP", lp["mean"], *lp["interval"], lp["standard_uncertainty"])]

    fig = plt.figure(figsize=(7.2, 4.3))
    outer = fig.add_gridspec(1, 2, width_ratios=[1.0, 1.25], wspace=0.12, left=0.13, right=0.99, top=0.90,
                             bottom=0.11)
    ax = fig.add_subplot(outer[0])
    # The 16 measured values with 2u.
    res = sorted(res, key=lambda r: r["value"])
    y0, dy = 4.4, 0.30
    for k, r in enumerate(res):
        y = y0 + k * dy
        ax.plot([r["value"] - 2 * r["u"], r["value"] + 2 * r["u"]], [y, y], color="#b9b7b1", linewidth=0.9)
        ax.plot(r["value"], y, "o", color=INK2, markersize=2.4)
    # Range of the fourteen procedures of Merkatas et al. (2019), Table 4 (manuscript Section 7.1).
    ax.axvspan(6.67377, 6.67429, color="#ecebe6", zorder=0)
    ax.text(6.67403, -0.75, "range of the 14 procedures\nof Merkatas et al.", fontsize=5.8, color=INK2, ha="center",
            va="center")
    ticks, labels = [y0 + 7.5 * dy], ["16 measured\nvalues (± 2u)"]
    for k, (name, key, m, lo, hi, u) in enumerate(proc):
        y = 3.2 - k * 0.95
        ax.plot([lo, hi], [y, y], color=PROC[key], linewidth=2.8, solid_capstyle="round")
        ax.plot(m, y, "o", color=PROC[key], markersize=5.5, markeredgecolor="white", markeredgewidth=1)
        ax.text(hi + 0.00012, y, f"u = {u * 1e5:.0f}", fontsize=5.9, color=PROC[key], va="center")
        ticks.append(y)
        labels.append(name)
    ax.set_yticks(ticks)
    ax.set_yticklabels(labels, fontsize=6.6)
    for lab, key in zip(ax.get_yticklabels()[1:], [p[1] for p in proc]):
        lab.set_color(PROC[key])
        lab.set_fontweight("bold")
    ax.get_yticklabels()[0].set_color(INK2)
    ax.tick_params(axis="y", length=0)
    ax.spines["left"].set_visible(False)
    ax.set_xlim(6.6707, 6.6765)
    ax.set_ylim(-1.5, y0 + 16 * dy)
    ax.set_xticks([6.671, 6.673, 6.675])
    ax.set_xlabel("G / (10⁻¹¹ m³ kg⁻¹ s⁻²), 95 % intervals; u in 10⁻⁵", fontsize=6.8)
    ax.xaxis.grid(True, color=GRID, linewidth=0.6)
    ax.set_axisbelow(True)
    fig.text(0.01, 0.975, "(a) G: one data set, four consensus values", fontsize=7.8,
             color=INK, va="top")

    # (b) Significant bilateral degrees of equivalence of the 60Co comparison, by procedure and version.
    cp = "library/physical/nicob-co60-activity-leave-one-out-bilateral.json"
    spec = json.loads(Path(cp).read_text(encoding="utf-8"))
    versions = {}
    for loo in (False, True):
        s = json.loads(json.dumps(spec))
        s["consensus"]["leave_one_out"] = loo
        s.pop("expected", None)
        tmp = OUT / "_co60.json"
        tmp.write_text(json.dumps(s), encoding="utf-8")
        versions[loo] = run_json(str(tmp))
        tmp.unlink()
    names = [r["name"] for r in spec["consensus"]["results"]]
    idx = {n: i for i, n in enumerate(names)}
    inner = outer[1].subgridspec(2, 4, width_ratios=[0.16, 1, 1, 1], wspace=0.10, hspace=0.30)
    keys = [("DerSimonian–Laird", "degrees_of_equivalence", "DL"),
            ("Hierarchical Bayes", "bayes_degrees_of_equivalence", "Bayes"),
            ("Linear Pool", "linear_pool_degrees_of_equivalence", "LP")]
    n = len(names)
    for row, loo in enumerate((False, True)):
        lab = fig.add_subplot(inner[row, 0])
        lab.axis("off")
        lab.text(0.5, 0.5, "leave-one-out" if loo else "MRA version", rotation=90, fontsize=6.6, color=INK,
                 ha="center", va="center", fontweight="bold")
        for col, (title, key, pk) in enumerate(keys):
            a = fig.add_subplot(inner[row, col + 1])
            m = np.full((n, n), np.nan)
            sig = 0
            for b in versions[loo][key]["bilateral"]:
                i, j = sorted((idx[b["a"]], idx[b["b"]]))
                m[j, i] = 1.0 if b["significant"] else 0.0
                sig += b["significant"]
            a.imshow(m, cmap=ListedColormap(["#efeee9", PROC[pk]]), vmin=0, vmax=1, interpolation="nearest")
            a.set_xticks([])
            a.set_yticks([])
            for s_ in a.spines.values():
                s_.set_visible(False)
            a.text(n - 1.0, 1.0, f"{sig}", fontsize=13, fontweight="bold", color=PROC[pk], ha="right", va="top")
            a.text(n - 1.0, 5.2, "of 171\npairs", fontsize=5.6, color=INK2, ha="right", va="top")
            if row == 0:
                a.set_title(title, fontsize=6.6, color=PROC[pk], fontweight="bold", pad=4)
    fig.text(0.50, 0.975, "(b) ⁶⁰Co, 19 laboratories: pairs with a significant\n"
             "     bilateral degree of equivalence (filled squares)", fontsize=7.8, color=INK, va="top")
    save(fig, "fig4_procedures")


# --- Graphical abstract (Elsevier: at least 1328 x 531 px at 300 dpi, ratio 5:2, all text in the image) -------------
def graphical_abstract():
    NAVY, NAVY2, CRED, GREEN = "#1b2f5e", "#e9edf6", "#b8321f", "#13805a"
    fig, ax = plt.subplots(figsize=(6.64, 2.656))
    fig.subplots_adjust(0, 0, 1, 1)
    ax.set_xlim(0, 100)
    ax.set_ylim(0, 40)
    ax.axis("off")

    def card(x, y, w, h, face, edge, lw=0.9):
        ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0,rounding_size=1.6", linewidth=lw,
                                    edgecolor=edge, facecolor=face))

    def txt(x, y, s, size, color=INK, weight="normal"):
        ax.text(x, y, s, fontsize=size, color=color, fontweight=weight, ha="center", va="center", linespacing=1.25)

    card(2, 6, 27, 30, "#f6f5f2", "#c9c7c1")
    txt(15.5, 29.5, "Worked examples", 8.5, INK2, "bold")
    txt(15.5, 21.5, "38", 22, INK, "bold")
    txt(15.5, 15.0, "guidance documents", 8, INK2)
    txt(15.5, 10.0, "286 projects · 1865 printed values", 7, INK2)
    card(36.5, 6, 27, 30, NAVY2, NAVY, 1.1)
    ax.add_patch(FancyBboxPatch((36.5, 27.5), 27, 8.5, boxstyle="round,pad=0,rounding_size=1.6", linewidth=0,
                                facecolor=NAVY))
    ax.add_patch(plt.Rectangle((36.5, 27.5), 27, 3, facecolor=NAVY, linewidth=0))
    txt(50, 31.8, "KAPSAM", 11, "white", "bold")
    txt(50, 21.0, "recomputes every example\nat every change of the code", 8, NAVY)
    txt(50, 11.0, "checked against Suncal\nand spreadsheet formulas", 7, NAVY)
    card(71, 6, 27, 30, "#fbeeeb", CRED, 1.1)
    txt(84.5, 29.5, "What it revealed", 8.5, CRED, "bold")
    txt(84.5, 21.5, "13", 22, CRED, "bold")
    txt(84.5, 15.0, "confirmed inconsistencies\nchange a final printed result", 8, INK)
    txt(84.5, 9.0, "among 100 findings", 7, INK2)
    for x0, x1 in ((29.6, 35.9), (64.1, 70.4)):
        ax.add_patch(FancyArrowPatch((x0, 21), (x1, 21), arrowstyle="-|>", mutation_scale=12, linewidth=1.4,
                                     color=NAVY))
    txt(50, 2.4, "Open source · GUM, Monte Carlo, top-down, proficiency testing, conformity, consensus values",
        7, GREEN, "bold")
    fig.savefig(OUT / "graphical_abstract.pdf")
    fig.savefig(OUT / "graphical_abstract.png", dpi=600)
    plt.close(fig)
    print("wrote", OUT / "graphical_abstract.pdf", "(3984 x 1594 px at 600 dpi)")


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
    graphical_abstract()
