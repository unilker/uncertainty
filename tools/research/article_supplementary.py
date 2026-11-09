"""Supplementary material of the article (docs/article/supplementary/).

    cargo build --release -p uncertainty-cli
    python3 tools/research/article_supplementary.py

S1  library projects with their source, field, evaluation type and number of compared values (CSV);
S2  the findings F1-F100 with class, document, and for class c their effect and status (CSV);
S3  the validation matrix (copy of docs/research/validation-matrix.md);
S4  the comparison with Suncal (copy of docs/research/benchmark-suncal.md);
S5  the software validation report written by `kapsam check library` (PDF);
S6  pointer to the beginner's guide and the step-by-step explanations (docs/guide/beginner).
"""
import csv
import glob
import json
import shutil
import subprocess
from pathlib import Path

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

# S3, S4
shutil.copy("docs/research/validation-matrix.md", OUT / "S3_validation_matrix.md")
shutil.copy("docs/research/benchmark-suncal.md", OUT / "S4_suncal_benchmark.md")

# S5
subprocess.run([EXE, "check", "library", "--report", str(OUT / "S5_validation_report.pdf"), "--lang", "en"],
               check=False, capture_output=True)
print("S5:", (OUT / "S5_validation_report.pdf").exists())

# S6
(OUT / "S6_guide.md").write_text(
    "# S6. The guide *Uncertainty from scratch*\n\n"
    "The guide is part of the application and of the repository, in English and Turkish:\n\n"
    "- chapters: `docs/guide/beginner/en/` and `docs/guide/beginner/tr/`;\n"
    "- every library example explained step by step, grouped by lesson and level: "
    "`docs/guide/beginner/en/examples.md` and `docs/guide/beginner/tr/examples.md`.\n\n"
    "They are written by `app/tool/write_learning_docs.dart` from `app/assets/guide/` and the library.\n",
    encoding="utf-8")
(OUT / "README.md").write_text(
    "# Supplementary material\n\nWritten by `tools/research/article_supplementary.py`; do not edit by hand.\n\n"
    "- S1 `S1_library.csv`: the library projects\n- S2 `S2_findings.csv`: the findings\n"
    "- S3 `S3_validation_matrix.md`: the validation matrix\n- S4 `S4_suncal_benchmark.md`: the comparison with Suncal\n"
    "- S5 `S5_validation_report.pdf`: the software validation report of the library\n"
    "- S6 `S6_guide.md`: the beginner's guide and the explanation of every example\n", encoding="utf-8")
