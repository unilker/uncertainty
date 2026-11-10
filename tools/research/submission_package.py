"""Submission package of the article for Measurement (Elsevier).

    python3 tools/research/submission_package.py OUT_DIR

writes into OUT_DIR:

* manuscript.docx: the English manuscript (docs/article/manuscript.md) with figures and tables in place, double
  spacing, continuous line numbers and page numbers, without the draft notes for the author;
* figures/: Figures 1-4 as separate files (vector PDF for 1, 3 and 4; TIFF at 300 dpi for the screenshot);
* highlights.docx: the highlights (docs/article/highlights.md), each checked against the limit of 85 characters;
* cover-letter.docx: the cover letter (tools/research/cover_letter_docx.py);
* supplementary/: S1-S7 from docs/article/supplementary/;
* CHECKLIST.md: what is still open before the files can be uploaded (placeholders found in the texts);

and OUT_DIR.zip. Needs pandoc, python-docx and Pillow. The package is generated, not stored in the repository.
"""
import re
import shutil
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path

from docx import Document
from docx.shared import Pt
from PIL import Image

sys.path.insert(0, str(Path(__file__).parent))
import article_docx  # noqa: E402
import cover_letter_docx  # noqa: E402

ART = Path("docs/article")
OPEN = [r"\[Zenodo DOI[^\]]*\]", r"\[release tag[^\]]*\]", r"\[To be [^\]]*\]", r"\[Suggested reviewers[^\]]*\]",
        r"\(planned\)", r"is planned before\s+submission", r"\[verify[^\]]*\]", r"\[corresponding author[^\]]*\]"]


def manuscript(out, commit):
    title, md = article_docx.assemble("en", commit)
    # Draft notes for the author: the review-copy note, the italic note under the title and the note under Table 4.
    paras = md.split("\n\n")
    drop = [p for p in paras if p.strip().startswith(("*Review copy", "*Second draft", "*Before submission"))]
    for p in drop:
        paras.remove(p)
    md = "\n\n".join(paras)
    md = md.replace("İlker Ün — TÜBİTAK Ulusal Metroloji Enstitüsü (UME), Gebze, Kocaeli, Türkiye",
                    "İlker Ün\n\nTÜBİTAK Ulusal Metroloji Enstitüsü (UME), Gebze, Kocaeli, Türkiye\n\n"
                    "Corresponding author: İlker Ün, ORCID 0000-0001-5089-4602, "
                    "[corresponding author e-mail and postal address]", 1)
    with tempfile.TemporaryDirectory() as tmp:
        ref = Path(tmp) / "ref.docx"
        article_docx.reference_docx(ref, "en")
        double = Path(tmp) / "ref2.docx"
        with zipfile.ZipFile(ref) as zin, zipfile.ZipFile(double, "w", zipfile.ZIP_DEFLATED) as zout:
            for item in zin.infolist():
                data = zin.read(item.filename)
                if item.filename == "word/styles.xml":
                    data = data.decode("utf-8").replace('w:line="360"', 'w:line="480"').encode("utf-8")
                zout.writestr(item, data)
        src = Path(tmp) / "m.md"
        src.write_text(md, encoding="utf-8")
        subprocess.run(["pandoc", str(src), "-f", "markdown", "-o", str(out / "manuscript.docx"), "--reference-doc",
                        str(double), "--metadata", f"title={title}", "--metadata", "lang=en-GB"], check=True)
    return md


def figures(out):
    d = out / "figures"
    d.mkdir()
    for n, name in enumerate(["fig1_architecture", "fig2_application", "fig3_findings_by_document",
                              "fig4_procedures"], 1):
        pdf = ART / "figures" / f"{name}.pdf"
        png = ART / "figures" / f"{name}.png"
        if n == 2:  # screenshot: raster
            Image.open(png).convert("RGB").save(d / f"Figure{n}.tif", dpi=(300, 300), compression="tiff_lzw")
        else:
            shutil.copy(pdf, d / f"Figure{n}.pdf")


def highlights(out):
    items = [l[2:].strip() for l in (ART / "highlights.md").read_text(encoding="utf-8").splitlines()
             if l.startswith("- ")]
    long = [h for h in items if len(h) > 85]
    assert 3 <= len(items) <= 5 and not long, (len(items), long)
    doc = Document()
    doc.styles["Normal"].font.name = "Times New Roman"
    doc.styles["Normal"].font.size = Pt(12)
    doc.add_heading("Highlights", level=1)
    for h in items:
        doc.add_paragraph(h, style="List Bullet")
    doc.save(out / "highlights.docx")


def main():
    out = Path(sys.argv[1] if len(sys.argv) > 1 else "submission")
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    commit = subprocess.run(["git", "rev-parse", "--short", "HEAD"], capture_output=True, text=True).stdout.strip()
    md = manuscript(out, commit)
    figures(out)
    highlights(out)
    cover_letter_docx.main(str(ART / "cover-letter.md"), str(out / "cover-letter.docx"), "en")
    shutil.copytree(ART / "supplementary", out / "supplementary", ignore=shutil.ignore_patterns("README.md"))
    letter = (ART / "cover-letter.md").read_text(encoding="utf-8")
    found = []
    for name, text in (("manuscript", md), ("cover letter", letter)):
        for pat in OPEN:
            for m in re.finditer(pat, text):
                line = text[:m.start()].count("\n") + 1
                found.append(f"- {name}, line {line}: `{' '.join(m.group(0).split())}`")
    words = len(re.sub(r"\n## References.*", "", md, flags=re.S).split())
    (out / "CHECKLIST.md").write_text(
        f"# Submission checklist (generated from commit {commit})\n\n"
        "Files: manuscript.docx (double spacing, line numbers), figures/Figure1-4, highlights.docx, cover-letter.docx, "
        "supplementary/S1-S7. Measurement asks in addition for the declaration of competing interest (Elsevier's "
        "online tool) and accepts an optional graphical abstract.\n\n"
        f"Manuscript: about {words} words before the references.\n\n"
        "## Still open\n\n" + ("\n".join(found) if found else "Nothing found.") + "\n", encoding="utf-8")
    shutil.make_archive(str(out), "zip", out)
    print(f"wrote {out} and {out}.zip; {len(found)} open items (CHECKLIST.md)")


if __name__ == "__main__":
    main()
