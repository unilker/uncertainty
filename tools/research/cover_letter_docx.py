"""Writes the cover letter as a Word document in letter layout (python-docx).

    python3 tools/research/cover_letter_docx.py docs/article/cover-letter.md out.docx [en|tr]
"""
import re
import sys
from datetime import date

from docx import Document
from docx.enum.text import WD_ALIGN_PARAGRAPH
from docx.shared import Cm, Pt, RGBColor

NAVY = RGBColor(0x1B, 0x2F, 0x5E)
MONTHS_TR = ["Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran", "Temmuz", "Ağustos", "Eylül", "Ekim", "Kasım",
             "Aralık"]


def runs(par, text):
    """Adds text with **bold** and *italic* spans to a paragraph."""
    for part in re.split(r"(\*\*[^*]+\*\*|\*[^*]+\*)", text):
        if part.startswith("**") and part.endswith("**"):
            par.add_run(part[2:-2]).bold = True
        elif part.startswith("*") and part.endswith("*") and len(part) > 1:
            par.add_run(part[1:-1]).italic = True
        elif part:
            par.add_run(part)


def main(src, out, lang="en"):
    text = open(src, encoding="utf-8").read()
    # The Turkish file starts with a note for the author in italics.
    blocks = [b.strip() for b in text.split("\n\n") if b.strip() and not (b.startswith("*") and "cover-letter.md" in b)]
    doc = Document()
    for s in doc.sections:
        s.top_margin = s.bottom_margin = Cm(2.2)
        s.left_margin = s.right_margin = Cm(2.5)
    st = doc.styles["Normal"]
    st.font.name = "Calibri"
    st.font.size = Pt(11)
    st.paragraph_format.space_after = Pt(6)
    st.paragraph_format.line_spacing = 1.15

    head = doc.add_paragraph()
    r = head.add_run("İlker Ün")
    r.bold = True
    r.font.color.rgb = NAVY
    r.font.size = Pt(12)
    for line in ["TÜBİTAK Ulusal Metroloji Enstitüsü (UME)", "Gebze, Kocaeli, Türkiye", "ORCID 0000-0001-5089-4602"]:
        head.add_run("\n" + line).font.size = Pt(10)
    today = date.today()
    d = doc.add_paragraph(f"{today.day} {MONTHS_TR[today.month - 1]} {today.year}" if lang == "tr"
                          else today.strftime("%d %B %Y").lstrip("0"))
    d.alignment = WD_ALIGN_PARAGRAPH.RIGHT
    to = doc.add_paragraph()
    to.add_run("Editör, Measurement (Elsevier)" if lang == "tr" else "The Editor-in-Chief\nMeasurement (Elsevier)")
    subject = doc.add_paragraph()
    subject.add_run("Konu: " if lang == "tr" else "Subject: ").bold = True
    subject.add_run("Makale başvurusu — " if lang == "tr" else "Submission of a research article — ")
    subject.add_run("KAPSAM: open-source software for measurement uncertainty, validated against the worked examples "
                    "of the guidance it implements").italic = True
    doc.add_paragraph()

    signature = False
    for b in blocks:
        if b.startswith(("Yours sincerely", "Saygılarımla")):
            signature = True
        lines = b.split("\n")
        if re.match(r"^\d+\. ", lines[0]):
            items, cur = [], ""
            for ln in lines:
                if re.match(r"^\d+\. ", ln):
                    if cur:
                        items.append(cur)
                    cur = re.sub(r"^\d+\. ", "", ln)
                else:
                    cur += " " + ln.strip()
            items.append(cur)
            for it in items:
                p = doc.add_paragraph(style="List Number")
                p.alignment = WD_ALIGN_PARAGRAPH.JUSTIFY
                runs(p, it)
            continue
        if signature:
            p = doc.add_paragraph()
            for k, ln in enumerate(lines):
                if k:
                    p.add_run("\n")
                rr = p.add_run(ln.strip())
                rr.bold = ln.strip() == "İlker Ün"
            if b.startswith(("Yours sincerely", "Saygılarımla")):
                p.paragraph_format.space_after = Pt(30)
            continue
        p = doc.add_paragraph()
        p.alignment = WD_ALIGN_PARAGRAPH.JUSTIFY
        runs(p, " ".join(ln.strip() for ln in lines))
    doc.core_properties.author = "İlker Ün"
    doc.core_properties.title = "Cover letter" if lang == "en" else "Editöre mektup"
    doc.save(out)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else "en")
