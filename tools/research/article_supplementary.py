"""Supplementary material of the article (docs/article/supplementary/).

    cargo build --release -p uncertainty-cli
    python3 tools/research/article_supplementary.py

S1  library projects with their source, field, evaluation type and number of compared values (Excel);
S2  the findings with class, document, and for class c their effect and status (Excel);
S3  the validation matrix (PDF of docs/research/validation-matrix.md);
S4  the comparison with Suncal and the Monte Carlo propagation through the calibration fit (PDF of
    docs/research/benchmark-suncal.md and calibration-mc.md);
S5  the software validation report written by `kapsam check library` (PDF);
S6  the beginner's guide and the step-by-step explanation of every example (PDF of docs/guide/beginner/en);
S7  the recalculation of the spreadsheet verification workbooks in LibreOffice (PDF of
    docs/research/spreadsheet-check.md) and the workbook of QUAM Example A1, saved by LibreOffice with its
    computed values so that previews show them (Excel recalculates it on opening);
S8  the recomputation record of the c1 findings and the errata check (PDF of docs/research/recomputation-record.md
    and errata-check.md).
Every file states the build of the program it was generated with (version, commit and date).
"""
import csv
import glob
import json
import re
import shutil
import subprocess
from pathlib import Path

import supplementary_pdf as pdf  # same directory; needs pandoc and Node with Playwright
import supplementary_xlsx as xlsx  # same directory; needs openpyxl
import sources  # same directory
import spreadsheet_check  # LibreOffice with its Python bridge (uno)

OUT = Path("docs/article/supplementary")
OUT.mkdir(parents=True, exist_ok=True)
EXE = "target/release/kapsam"
# The build of the program, e.g. "KAPSAM 0.1.1 (commit 0123456789ab, 2026-11-11)", stated in every supplementary file.
BUILD = subprocess.run([EXE, "--version"], capture_output=True, text=True, check=True).stdout.splitlines()[0].strip()
assert "-modified" not in BUILD, f"generate the supplementary material from a committed tree: {BUILD}"
xlsx.SUBTITLE = f"Generated with {BUILD} — https://github.com/unilker/uncertainty"
_md_to_pdf = pdf.md_to_pdf
pdf.md_to_pdf = lambda md, out, header, **kw: _md_to_pdf(md, out, f"{header} — {BUILD}", **kw)  # noqa: E731
SECTIONS = ["model", "multivariate", "topdown", "proficiency", "calibration", "conformity", "precision_experiment",
            "control_chart", "sampling", "error_function", "consensus", "multicomponent_conformity"]

# S1
rows = []
total = 0
for f in sorted(glob.glob("library/*/*.json")):
    p = json.load(open(f, encoding="utf-8"))
    m = p["metadata"]
    kind = " + ".join(s for s in SECTIONS if s in p)
    r = subprocess.run([EXE, "run", f, "--format", "json"], capture_output=True, text=True, check=True)
    checks = json.loads(r.stdout).get("checks", [])
    passed = sum(1 for c in checks if c.get("passed", True))
    total += len(checks)
    src = (m.get("sources") or [{}])[0]
    _, skind = sources.primary(p)
    rows.append([m["id"], m["title"]["en"], m.get("field", ""), kind, src.get("document", ""), src.get("section", ""),
                 sources.KIND_TEXT[skind], sources.reference_basis(p), p["expected"]["reference"]["en"],
                 len(checks), passed])
xlsx.table(
    OUT / "S1_library.xlsx", "Library",
    "S1. The 286 projects of the KAPSAM library",
    [("ID", 34), ("Title", 60), ("Field", 14), ("Evaluation type", 18), ("Source document", 60),
     ("Source section", 34), ("Kind of source", 18), ("Basis of the reference values", 26),
     ("Reference statement of the project", 70), ("Compared values", 11), ("Within tolerance", 11)],
    rows,
    notes=[
        ("ID", "File name of the project in library/<field>/ of the repository; the project can be opened in the "
               "application or run with `kapsam run`."),
        ("Title", "Title of the project (the application also shows it in Turkish)."),
        ("Field", "Field of measurement (directory of the library)."),
        ("Evaluation type", "Section of the project file: model (GUM and Monte Carlo), multivariate (JCGM 102), "
                            "topdown, proficiency (ISO 13528), calibration (ISO 6143), conformity (JCGM 106), "
                            "precision_experiment (ISO 5725-2), control_chart (TR 569), sampling (UfS), "
                            "error_function (EURAMET cg-18), consensus, multicomponent_conformity."),
        ("Source document, section", "The worked example the project reproduces (the first source of the project)."),
        ("Kind of source", "guidance document or standard; BIPM reference data report; journal paper or book; "
                           "test case constructed for this work (guard bands from the test uncertainty ratio, from "
                           "the published formulas)."),
        ("Basis of the reference values", "printed values: every reference value is printed in the source and "
                                          "reproduced from its inputs within its rounding; printed values, some of them "
                                          "Monte Carlo or MCMC estimates: the tolerance includes their sampling error; "
                                          "partly or wholly an independent recomputation: the source prints no value, or "
                                          "a value that does not follow from its inputs (a finding), and the reference is "
                                          "a recomputation by a script of the repository (NumPy, SciPy). Classified from "
                                          "the reference statement in the next column."),
        ("Reference statement", "The statement in the project file of where its reference values come from."),
        ("Compared values", "Number of results compared with the reference values of the source at every test run."),
        ("Within tolerance", "Number of those results within their documented tolerance."),
    ],
    total_row=["Total", f"{len(rows)} projects", "", "", "", "", "", "", "", total, sum(r[10] for r in rows)],
)
print("S1:", len(rows), "projects,", total, "compared values")

# S2
impact = {}
with open("docs/research/class-c-impact.tsv", encoding="utf-8") as fh:
    for r in csv.DictReader(fh, delimiter="\t"):
        impact[r["F"]] = r
findings = []
with open("docs/research/finding-classes.tsv", encoding="utf-8") as fh:
    for r in csv.DictReader(fh, delimiter="\t"):
        i = impact.get(r["F"], {})
        title = re.sub(r"\s*\((?:[^()]*\b(?:class|article|methodological)\b[^()]*)\)\.?", "", r["title"]).strip()
        # printed and recomputed values are kept as text, so that their trailing zeros (printed digits) stay
        findings.append([r["F"], r["class"], title, i.get("document", ""), i.get("impact", ""),
                         i.get("status", ""), i.get("quantity", ""), i.get("printed", ""),
                         i.get("recomputed", ""), i.get("note", "")])
for old in OUT.glob("S[12]_*.csv"):
    old.unlink()
xlsx.table(
    OUT / "S2_findings.xlsx", "Findings",
    f"S2. The {len(findings)} findings of the recomputation",
    [("F", 7), ("Class", 7), ("Title", 70), ("Document (class c)", 34), ("Effect", 8), ("Status", 12),
     ("Quantity", 24), ("Printed", 14), ("Recomputed", 14), ("Note", 60)],
    findings,
    notes=[
        ("F", "Number of the finding in the research log of the repository (docs/research/research-log.md), where "
              "the derivation of each finding is given."),
        ("Class", "a: intermediate rounding that changes a printed digit; b: a misprint or transcription error "
                  "without effect on the result; c: an inconsistency; n: no discrepancy (a methodological note or a "
                  "full reproduction). Classes fixed before the audit (research log D12, D73)."),
        ("Effect (class c)", "c1: a final printed result (the estimate, its uncertainty or the stated outcome) is not "
                             "reproduced at its printed digits; c2: only an intermediate value is not; m: a difference "
                             "of method or interpretation rather than an inconsistency; e: already corrected by a "
                             "published erratum (D84)."),
        ("Status", "confirmed: the recomputation from the document's own data leaves no open question; independently "
                   "checked: confirmed, and also reproduced by a person who had not seen the code or reported by "
                   "another author; erratum or partly erratum: corrected, wholly or in part, by a published erratum; "
                   "open: awaits a check against another source and is not counted."),
        ("Quantity, printed, recomputed", "The quantity concerned, the value printed in the document and the value "
                                          "recomputed from its data."),
    ],
)
print("S2:", len(findings), "findings")

# S3, S4 (PDF from the Markdown tables of docs/research)
for old in OUT.glob("S[3467]_*.md"):
    old.unlink()
parts = [Path(f).read_text(encoding="utf-8") for f in ("docs/research/validation-matrix.md", "docs/research/near-tolerance.md",
                                                        "docs/research/mc-seeds.md")]
s3 = parts[0] + "\n\n" + "\n\n".join(q.replace("\n## ", "\n### ").replace("# ", "## ", 1) for q in parts[1:])
pdf.md_to_pdf(s3, OUT / "S3_validation_matrix.pdf",
              "KAPSAM — Supplementary material S3: validation matrix", landscape=True)
# S4: the comparison with Suncal and, as its last part, the Monte Carlo propagation through the calibration fit
mc = Path("docs/research/calibration-mc.md").read_text(encoding="utf-8").replace("\n## ", "\n### ")
pdf.md_to_pdf(Path("docs/research/benchmark-suncal.md").read_text(encoding="utf-8") + "\n\n#" + mc,
              OUT / "S4_suncal_benchmark.pdf", "KAPSAM — Supplementary material S4: comparison with Suncal",
              landscape=True)
print("S3, S4: PDF")

# S5
subprocess.run([EXE, "check", "library", "--report", str(OUT / "S5_validation_report.pdf"), "--lang", "en"],
               check=False, capture_output=True)
print("S5:", (OUT / "S5_validation_report.pdf").exists())

# S6: the English beginner's guide and the explanation of every example, as one PDF
pdf.md_to_pdf("# Uncertainty from scratch\n\n" + pdf.FRONT["en"].format(n=len(rows)) + "\n\n" + pdf.guide_markdown("en"), OUT / "S6_guide.pdf",
              "KAPSAM — Supplementary material S6: Uncertainty from scratch", toc=True)
print("S6: PDF")
# S7
pdf.md_to_pdf(Path("docs/research/spreadsheet-check.md").read_text(encoding="utf-8"), OUT / "S7_spreadsheet_check.pdf",
              "KAPSAM — Supplementary material S7: spreadsheet verification", landscape=True)
raw = OUT / "S7_workbook_quam_a1.raw.xlsx"
subprocess.run([EXE, "run", "library/chemistry/quam-a1-cadmium-standard.json", "--no-mcm", "-o", str(raw)],
               check=True, capture_output=True)
spreadsheet_check.recalculated_copy(raw, OUT / "S7_workbook_quam_a1.xlsx")
raw.unlink()
print("S7:", (OUT / "S7_workbook_quam_a1.xlsx").exists())
# S8: the recomputation record of the c1 findings and the errata check (second internal review, D112)
errata = Path("docs/research/errata-check.md").read_text(encoding="utf-8").replace("# Errata", "## 5. Errata", 1)
pdf.md_to_pdf(Path("docs/research/recomputation-record.md").read_text(encoding="utf-8") + "\n\n" + errata,
              OUT / "S8_recomputation_record.pdf", "KAPSAM — Supplementary material S8: recomputation record",
              landscape=True)
print("S8: PDF")
(OUT / "README.md").write_text(
    "# Supplementary material\n\nWritten by `tools/research/article_supplementary.py`; do not edit by hand.\n\n"
    "- S1 `S1_library.xlsx`: the library projects\n- S2 `S2_findings.xlsx`: the findings\n"
    "- S3 `S3_validation_matrix.pdf`: the validation matrix\n- S4 `S4_suncal_benchmark.pdf`: the comparison with Suncal "
    "and the Monte Carlo propagation through the calibration fit\n"
    "- S5 `S5_validation_report.pdf`: the software validation report of the library\n"
    "- S6 `S6_guide.pdf`: the beginner's guide and the step-by-step explanation of every example (English)\n"
    "- S7 `S7_spreadsheet_check.pdf`: the recalculation of the spreadsheet verification workbooks, and "
    "`S7_workbook_quam_a1.xlsx`, the workbook of QUAM Example A1\n"
    "- S8 `S8_recomputation_record.pdf`: the recomputation record of the c1 findings (printed, program and "
    "recomputed values; derivations) and the check of errata and amendments\n\n"
    f"All files were generated with {BUILD}.\n", encoding="utf-8")
