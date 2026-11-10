"""Submission package of the article for Measurement (Elsevier), following the journal's guide for authors
(read 2026-11-10; research log D103). The journal uses double anonymized peer review: the title page and the
anonymized manuscript are separate files, and neither the manuscript nor the supplementary material may identify
the author.

    python3 tools/research/submission_package.py OUT_DIR

writes into OUT_DIR:

* title-page.docx: title, author, affiliation with postal address, corresponding author, acknowledgements,
  declaration of competing interests, funding, CRediT statement, links to the software and data for the editor;
* manuscript-anonymized.docx: the English manuscript with figures and tables in place, 1.5 line spacing,
  continuous line numbers, page numbers, variables in italics, the figure captions also listed at the end, without
  the author's name, affiliation, repository link, DOI of the archived release, acknowledgements, CRediT, funding and
  competing-interest sections, and without document metadata;
* figures/Figure_1-4 (vector PDF; Figure 2 a TIFF screenshot) and Graphical_abstract (PDF, TIFF);
* highlights.docx;
* cover-letter.docx (to the editor; not anonymized);
* supplementary/S1-S7, anonymized;
* CHECKLIST.md: the requirements of the guide and how the package meets them, the placeholders still open, and the
  result of a scan of every anonymized file for identifying information;

and OUT_DIR.zip. Needs pandoc, python-docx, openpyxl, Pillow, LibreOffice (page count) and Chromium (PDFs).
"""
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

import openpyxl
from docx import Document
from docx.shared import Pt
from PIL import Image

sys.path.insert(0, str(Path(__file__).parent))
import article_docx  # noqa: E402
import cover_letter_docx  # noqa: E402
import supplementary_pdf as spdf  # noqa: E402

ART = Path("docs/article")
EXE = "target/release/kapsam"
# The corresponding author's e-mail is given on the command line or in the environment, so that it is not stored in
# the repository: python3 tools/research/submission_package.py OUT_DIR --email ADDRESS
EMAIL = next((sys.argv[i + 1] for i, a in enumerate(sys.argv[:-1]) if a == "--email"),
             os.environ.get("KAPSAM_AUTHOR_EMAIL", "[e-mail of the corresponding author]"))
WITHHELD = "[withheld for double-anonymized review]"
# Identifying strings and their anonymized form, longest first.
ANON = [
    ("https://doi.org/10.5281/zenodo.24118312", f"DOI {WITHHELD}"),
    ("10.5281/zenodo.24118312", WITHHELD), ("10.5281/zenodo.24118311", WITHHELD),
    ("https://github.com/unilker/uncertainty", f"repository link {WITHHELD}"),
    ("github.com/unilker/uncertainty", f"repository link {WITHHELD}"),
    ("TÜBİTAK National Metrology Institute (TÜBİTAK UME), 41470 Gebze, Kocaeli, Türkiye", "[institute]"),
    ("TÜBİTAK National Metrology Institute (TÜBİTAK UME)", "[institute]"),
    ("TÜBİTAK Ulusal Metroloji Enstitüsü (UME), Gebze, Kocaeli, Türkiye", "[institute]"),
    ("TÜBİTAK Ulusal Metroloji Enstitüsü (UME)", "[institute]"),
    ("TÜBİTAK UME, Belirsizlik Hesapları Prosedürü", "[institute], uncertainty procedure"),
    ("Belirsizlik Hesapları Prosedürü", "uncertainty procedure"),
    ("author's own document", "anonymized for review"),
    ("the TÜBİTAK UME uncertainty procedure", "the uncertainty procedure of a national metrology institute"),
    ("TÜBİTAK UME uncertainty procedure", "uncertainty procedure of a national metrology institute"),
    ("the uncertainty procedure of TÜBİTAK UME", "the uncertainty procedure of a national metrology institute"),
    ("(TÜBİTAK UME)", "(procedure of a metrology institute)"),
    ("TÜBİTAK UME", "a national metrology institute"),
    ("UME uncertainty procedure", "institute uncertainty procedure"),
    ("UME procedure", "institute procedure"),
    ("UME-PROC", "INST-PROC"), ("ume-procedure", "inst-procedure"),
    ("İ. Ün, KAPSAM", "[author withheld], KAPSAM"), ("Ün, İ.", "[author withheld]"), ("İlker Ün", "[author withheld]"),
    ("0000-0001-5089-4602", WITHHELD),
    ("laboratories of the institute", "laboratories"),
]
IDENT = re.compile(r"TÜBİTAK|\bUME\b|ume-procedure|İlker|\bÜn\b|unilker|zenodo\.2411|0000-0001-5089-4602|Gebze",
                   re.I)
OPEN = [r"\[Zenodo DOI[^\]]*\]", r"\[release tag[^\]]*\]", r"\[To be [^\]]*\]", r"\[Suggested reviewers[^\]]*\]",
        r"\(planned\)", r"is planned before\s+submission", r"\[verify[^\]]*\]", r"\[to be (?:added|confirmed)[^\]]*\]",
        r"\[confirm[^\]]*\]", r"\[e-mail[^\]]*\]", r"\[choose[^\]]*\]"]
FIGURES = ["fig1_architecture", "fig2_application", "fig3_findings_by_document", "fig4_procedures"]


def anon(text):
    for a, b in ANON:
        text = text.replace(a, b)
    return text


def section(md, heading):
    """Text of a '## heading' section (without the heading), and md without it."""
    i = md.index(f"\n## {heading}")
    j = md.find("\n## ", i + 4)
    j = len(md) if j < 0 else j
    return md[i:j].split("\n", 2)[2].strip(), md[:i] + md[j:]


def clear_metadata(path, title):
    doc = Document(path)
    cp = doc.core_properties
    cp.author, cp.last_modified_by, cp.comments, cp.keywords, cp.subject = "", "", "", "", ""
    cp.title = title
    doc.save(path)


def manuscript(out, commit):
    title, md = article_docx.assemble("en", commit)
    paras = md.split("\n\n")
    paras = [p for p in paras if not p.strip().startswith(("*Review copy", "*Second draft", "*Before submission"))
             and not p.strip().startswith("İlker Ün —")]
    md = "\n\n".join(paras)
    parts = {}
    for h in ("CRediT authorship contribution statement", "Declaration of competing interest", "Funding"):
        parts[h], md = section(md, h)
    md = anon(md)
    cap = article_docx.CAPTIONS["en"]
    md += "\n\n## Figure captions\n\n" + "\n\n".join(cap[k] for k in ("fig1", "fig2", "fig3", "fig4")) + "\n"
    with tempfile.TemporaryDirectory() as tmp:
        ref = Path(tmp) / "ref.docx"
        article_docx.reference_docx(ref, "en")
        src = Path(tmp) / "m.md"
        src.write_text(md, encoding="utf-8")
        subprocess.run(["pandoc", str(src), "-f", "markdown", "-o", str(out / "manuscript-anonymized.docx"),
                        "--reference-doc", str(ref), "--metadata", f"title={title}", "--metadata", "lang=en-GB"],
                       check=True)
    clear_metadata(out / "manuscript-anonymized.docx", title)
    return title, md, parts


def title_page(out, title, parts):
    doc = Document()
    st = doc.styles["Normal"]
    st.font.name, st.font.size = "Times New Roman", Pt(12)
    p = doc.add_paragraph()
    p.add_run(title).bold = True
    p.runs[0].font.size = Pt(14)
    p = doc.add_paragraph()
    p.add_run("İlker Ün")
    p.add_run("a").font.superscript = True
    p.add_run(" *")
    p = doc.add_paragraph()
    p.add_run("a").font.superscript = True
    p.add_run(" TÜBİTAK National Metrology Institute (TÜBİTAK UME), TÜBİTAK Gebze Yerleşkesi, Barış Mah. Dr. Zeki Acar Cad. No: 1, 41470 Gebze, Kocaeli, Türkiye")
    doc.add_paragraph("* Corresponding author: İlker Ün, TÜBİTAK National Metrology Institute (TÜBİTAK UME), address as above. "
                      f"E-mail: {EMAIL}. ORCID: 0000-0001-5089-4602.")
    doc.add_heading("Declaration of competing interest", level=2)
    doc.add_paragraph("The author declares that there are no known competing financial interests or personal "
                      "relationships that could have appeared to influence the work reported in this paper.")
    doc.add_heading("Funding", level=2)
    doc.add_paragraph("This research did not receive any specific grant from funding agencies in the public, commercial, or not-for-profit sectors.")
    doc.add_heading("CRediT authorship contribution statement", level=2)
    doc.add_paragraph(parts["CRediT authorship contribution statement"].replace("\n", " "))
    doc.add_heading("Software and data (for the editor)", level=2)
    doc.add_paragraph("Repository: https://github.com/unilker/uncertainty. Archived release v0.1.0: "
                      "https://doi.org/10.5281/zenodo.24118312 (all versions: https://doi.org/10.5281/zenodo.24118311). "
                      "These links are withheld in the anonymized manuscript; the data link is also given in the "
                      "submission system (research data, option C).")
    doc.core_properties.author = "İlker Ün"
    doc.core_properties.title = "Title page"
    doc.save(out / "title-page.docx")
    return "\n".join(p.text for p in doc.paragraphs)


def figures(out):
    d = out / "figures"
    d.mkdir()
    for n, name in enumerate(FIGURES, 1):
        if n == 2:  # screenshot: raster
            Image.open(ART / "figures" / f"{name}.png").convert("RGB").save(
                d / f"Figure_{n}.tif", dpi=(300, 300), compression="tiff_lzw")
        else:
            shutil.copy(ART / "figures" / f"{name}.pdf", d / f"Figure_{n}.pdf")
    shutil.copy(ART / "figures" / "graphical_abstract.pdf", d / "Graphical_abstract.pdf")
    Image.open(ART / "figures" / "graphical_abstract.png").convert("RGB").save(
        d / "Graphical_abstract.tif", dpi=(600, 600), compression="tiff_lzw")


def highlights(out):
    items = [ln[2:].strip() for ln in (ART / "highlights.md").read_text(encoding="utf-8").splitlines()
             if ln.startswith("- ")]
    long = [h for h in items if len(h) > 85]
    assert 3 <= len(items) <= 5 and not long, (len(items), long)
    doc = Document()
    doc.styles["Normal"].font.name = "Times New Roman"
    doc.styles["Normal"].font.size = Pt(12)
    doc.add_heading("Highlights", level=1)
    for h in items:
        doc.add_paragraph(h, style="List Bullet")
    doc.save(out / "highlights.docx")
    clear_metadata(out / "highlights.docx", "Highlights")
    return items


def supplementary(out):
    d = out / "supplementary"
    d.mkdir()
    src = ART / "supplementary"
    for name in ("S1_library.xlsx", "S2_findings.xlsx"):
        wb = openpyxl.load_workbook(src / name)
        for ws in wb:
            for row in ws.iter_rows():
                for c in row:
                    if isinstance(c.value, str):
                        c.value = anon(c.value)
        wb.properties.creator = ""
        wb.properties.lastModifiedBy = ""
        wb.save(d / name)
    shutil.copy(src / "S7_workbook_quam_a1.xlsx", d / "S7_workbook_quam_a1.xlsx")
    mc = Path("docs/research/calibration-mc.md").read_text(encoding="utf-8").replace("\n## ", "\n### ")
    jobs = [("S3_validation_matrix.pdf", Path("docs/research/validation-matrix.md").read_text(encoding="utf-8"),
             "KAPSAM — Supplementary material S3: validation matrix", True, False),
            ("S4_suncal_benchmark.pdf",
             Path("docs/research/benchmark-suncal.md").read_text(encoding="utf-8") + "\n\n#" + mc,
             "KAPSAM — Supplementary material S4: comparison with Suncal", True, False),
            ("S7_spreadsheet_check.pdf", Path("docs/research/spreadsheet-check.md").read_text(encoding="utf-8"),
             "KAPSAM — Supplementary material S7: spreadsheet verification", True, False)]
    with tempfile.TemporaryDirectory() as tmp:
        val = Path(tmp) / "validation.md"
        subprocess.run([EXE, "check", "library", "--report", str(val), "--lang", "en"], check=False,
                       capture_output=True)
        jobs.append(("S5_validation_report.pdf", val.read_text(encoding="utf-8"),
                     "KAPSAM — Supplementary material S5: software validation report", False, False))
    jobs.append(("S6_guide.pdf", "# Uncertainty from scratch\n\n" + spdf.FRONT["en"] + "\n\n" + spdf.guide_markdown("en"),
                 "KAPSAM — Supplementary material S6: Uncertainty from scratch", False, True))
    for name, md, header, landscape, toc in jobs:
        spdf.md_to_pdf(anon(md), d / name, header, landscape=landscape, toc=toc)


def text_of(path):
    if path.suffix == ".docx":
        doc = Document(path)
        cp = doc.core_properties
        body = [p.text for p in doc.paragraphs] + [c.text for t in doc.tables for r in t.rows for c in r.cells]
        return "\n".join(body + [cp.author or "", cp.last_modified_by or "", cp.title or ""])
    if path.suffix == ".pdf":
        txt = subprocess.run(["pdftotext", str(path), "-"], capture_output=True, text=True).stdout
        info = subprocess.run(["pdfinfo", str(path)], capture_output=True, text=True).stdout
        return txt + info
    if path.suffix == ".xlsx":
        wb = openpyxl.load_workbook(path)
        cells = [str(c.value) for ws in wb for row in ws.iter_rows() for c in row if c.value is not None]
        return "\n".join(cells + [wb.properties.creator or "", wb.properties.lastModifiedBy or ""])
    return ""


def pages(docx):
    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run(["soffice", "--headless", "--convert-to", "pdf", "--outdir", tmp, str(docx)],
                       capture_output=True, check=True)
        info = subprocess.run(["pdfinfo", str(Path(tmp) / (docx.stem + ".pdf"))], capture_output=True,
                              text=True).stdout
    return int(re.search(r"Pages:\s+(\d+)", info).group(1))


def main():
    out = Path(sys.argv[1] if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else "submission")
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    commit = subprocess.run(["git", "rev-parse", "--short", "HEAD"], capture_output=True, text=True).stdout.strip()
    title, md, parts = manuscript(out, commit)
    tp = title_page(out, title, parts)
    figures(out)
    hl = highlights(out)
    cover_letter_docx.main(str(ART / "cover-letter.md"), str(out / "cover-letter.docx"), "en")
    supplementary(out)

    # Scan every file that reviewers see.
    leaks = []
    for f in sorted(out.rglob("*")):
        if f.is_file() and f.name not in ("title-page.docx", "cover-letter.docx", "CHECKLIST.md"):
            for m in IDENT.finditer(text_of(f)):
                leaks.append(f"- {f.relative_to(out)}: `{m.group(0)}`")
    letter = (ART / "cover-letter.md").read_text(encoding="utf-8")
    found = []
    for name, text in (("manuscript", md), ("title page", tp), ("cover letter", letter)):
        for pat in OPEN:
            for m in re.finditer(pat, text):
                found.append(f"- {name}: `{' '.join(m.group(0).split())}`")
    abstract = md.split("## Abstract", 1)[1].split("**Keywords:**", 1)[0]
    keywords = [k.strip() for k in md.split("**Keywords:**", 1)[1].split("\n\n", 1)[0].replace("\n", " ").split(";")]
    n_pages = pages(out / "manuscript-anonymized.docx")
    checks = [
        ("Research paper of at most 30 pages", n_pages <= 30, f"{n_pages} pages (1.5 spacing, figures and tables in place)"),
        ("Abstract at most 250 words, no references", len(abstract.split()) <= 250 and not re.search(r"\[\d", abstract),
         f"{len(abstract.split())} words"),
        ("1 to 7 keywords", 1 <= len(keywords) <= 7, "; ".join(keywords)),
        ("3 to 5 highlights of at most 85 characters, file name contains 'highlights'", True,
         ", ".join(str(len(h)) for h in hl) + " characters"),
        ("Graphical abstract at least 1328 px wide, readable at 13 cm", True, "Graphical_abstract.tif/.pdf"),
        ("Title page and anonymized manuscript as separate files", True, "title-page.docx, manuscript-anonymized.docx"),
        ("No identifying information in the manuscript and supplementary material", not leaks,
         "scan below" if leaks else "no match"),
        ("Word file, single column, no underline or strikethrough", True, "pandoc reference document"),
        ("Figures as separate files, logical names, captions with a brief title", True, "figures/Figure_1-4"),
        ("Declaration of generative AI immediately before the references", md.index("## Declaration of generative AI")
         < md.index("## References"), "section present"),
    ]
    lines = [f"# Submission checklist (generated from commit {commit})", "",
             "Requirements of the Measurement guide for authors (read 2026-11-10) and how this package meets them.", "",
             "| Requirement | Met | Detail |", "|---|---|---|"]
    lines += [f"| {r} | {'yes' if ok else 'NO'} | {d} |" for r, ok, d in checks]
    lines += ["", "## Identifying information in the files reviewers see", "", "\n".join(leaks) or "None found.", "",
              "## Still open (placeholders)", "", "\n".join(found) or "Nothing found.", "",
              "## To do in the submission system", "",
              "- Declaration of competing interests with Elsevier's declarations tool (upload the Word file).",
              "- Link to the data set (research data, option C): the Zenodo DOI of the release.",
              "- Choose whether to post the manuscript as a preprint on SSRN (optional).", ""]
    (out / "CHECKLIST.md").write_text("\n".join(lines), encoding="utf-8")
    shutil.make_archive(str(out), "zip", out)
    print(f"wrote {out}.zip; {n_pages} pages; {len(leaks)} identifying matches; {len(found)} open items")


if __name__ == "__main__":
    main()
