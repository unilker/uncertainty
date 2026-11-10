"""Supplementary material of the article (docs/article/supplementary/).

    cargo build --release -p uncertainty-cli
    python3 tools/research/article_supplementary.py

S1  library projects with their source, field, evaluation type and number of compared values (CSV);
S2  the findings F1-F100 with class, document, and for class c their effect and status (CSV);
S3  the validation matrix (PDF of docs/research/validation-matrix.md);
S4  the comparison with Suncal (PDF of docs/research/benchmark-suncal.md);
S5  the software validation report written by `kapsam check library` (PDF);
S6  the beginner's guide and the step-by-step explanation of every example (PDF of docs/guide/beginner/en);
S7  the recalculation of the spreadsheet verification workbooks in LibreOffice (PDF of
    docs/research/spreadsheet-check.md) and the workbook of QUAM Example A1.
"""
import csv
import glob
import json
import shutil
import subprocess
from pathlib import Path

import supplementary_pdf as pdf  # same directory; needs pandoc and Node with Playwright

OUT = Path("docs/article/supplementary")
OUT.mkdir(parents=True, exist_ok=True)
EXE = "target/release/kapsam"
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
    rows.append([m["id"], m["title"]["en"], m.get("field", ""), kind, src.get("document", ""), src.get("section", ""),
                 len(checks), passed])
with open(OUT / "S1_library.csv", "w", newline="", encoding="utf-8") as fh:
    w = csv.writer(fh)
    w.writerow(["id", "title", "field", "evaluation type", "source document", "source section", "compared values",
                "within tolerance"])
    w.writerows(rows)
print("S1:", len(rows), "projects,", total, "compared values")

# S2
impact = {}
with open("docs/research/class-c-impact.tsv", encoding="utf-8") as fh:
    for r in csv.DictReader(fh, delimiter="\t"):
        impact[r["F"]] = r
with open("docs/research/finding-classes.tsv", encoding="utf-8") as fh, \
        open(OUT / "S2_findings.csv", "w", newline="", encoding="utf-8") as out:
    w = csv.writer(out)
    w.writerow(["F", "class", "title", "document (class c)", "effect", "status", "quantity", "printed", "recomputed",
                "note"])
    n = 0
    for r in csv.DictReader(fh, delimiter="\t"):
        i = impact.get(r["F"], {})
        w.writerow([r["F"], r["class"], r["title"], i.get("document", ""), i.get("impact", ""), i.get("status", ""),
                    i.get("quantity", ""), i.get("printed", ""), i.get("recomputed", ""), i.get("note", "")])
        n += 1
print("S2:", n, "findings")

# S3, S4 (PDF from the Markdown tables of docs/research)
for old in OUT.glob("S[3467]_*.md"):
    old.unlink()
pdf.md_to_pdf(Path("docs/research/validation-matrix.md").read_text(encoding="utf-8"), OUT / "S3_validation_matrix.pdf",
              "KAPSAM — Supplementary material S3: validation matrix", landscape=True)
pdf.md_to_pdf(Path("docs/research/benchmark-suncal.md").read_text(encoding="utf-8"), OUT / "S4_suncal_benchmark.pdf",
              "KAPSAM — Supplementary material S4: comparison with Suncal", landscape=True)
print("S3, S4: PDF")

# S5
subprocess.run([EXE, "check", "library", "--report", str(OUT / "S5_validation_report.pdf"), "--lang", "en"],
               check=False, capture_output=True)
print("S5:", (OUT / "S5_validation_report.pdf").exists())

# S6: the English beginner's guide and the explanation of every example, as one PDF
pdf.md_to_pdf("# Uncertainty from scratch\n\n" + pdf.FRONT["en"] + "\n\n" + pdf.guide_markdown("en"), OUT / "S6_guide.pdf",
              "KAPSAM — Supplementary material S6: Uncertainty from scratch", toc=True)
print("S6: PDF")
# S7
pdf.md_to_pdf(Path("docs/research/spreadsheet-check.md").read_text(encoding="utf-8"), OUT / "S7_spreadsheet_check.pdf",
              "KAPSAM — Supplementary material S7: spreadsheet verification", landscape=True)
subprocess.run([EXE, "run", "library/chemistry/quam-a1-cadmium-standard.json", "--no-mcm", "-o",
                str(OUT / "S7_workbook_quam_a1.xlsx")], check=True, capture_output=True)
print("S7:", (OUT / "S7_workbook_quam_a1.xlsx").exists())
(OUT / "README.md").write_text(
    "# Supplementary material\n\nWritten by `tools/research/article_supplementary.py`; do not edit by hand.\n\n"
    "- S1 `S1_library.csv`: the library projects\n- S2 `S2_findings.csv`: the findings\n"
    "- S3 `S3_validation_matrix.pdf`: the validation matrix\n- S4 `S4_suncal_benchmark.pdf`: the comparison with Suncal\n"
    "- S5 `S5_validation_report.pdf`: the software validation report of the library\n"
    "- S6 `S6_guide.pdf`: the beginner's guide and the step-by-step explanation of every example (English)\n"
    "- S7 `S7_spreadsheet_check.pdf`: the recalculation of the spreadsheet verification workbooks, and "
    "`S7_workbook_quam_a1.xlsx`, the workbook of QUAM Example A1\n", encoding="utf-8")
