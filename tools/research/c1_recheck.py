"""Blind recomputation form for the confirmed c1 findings (Table 4 of the article; Section 9: "the recomputation
of at least ten of the c1 findings by a person who has not seen the code").

    python3 tools/research/c1_recheck.py OUT_DIR

writes OUT_DIR/c1-recheck-form.docx (instructions in Turkish and English; for every finding the document, the
example and the printed quantity, with empty columns for the recomputed value, the method and remarks; the
program's values are not shown) and OUT_DIR/c1-recheck-key.docx (the program's recomputed values and the explanation
of each finding, to be opened only after the form has been filled in). Data: docs/research/class-c-impact.tsv.
"""
import sys
from pathlib import Path

from docx import Document
from docx.enum.section import WD_ORIENT
from docx.shared import Cm, Pt

TSV = Path("docs/research/class-c-impact.tsv")


def findings():
    rows = [ln.split("\t") for ln in TSV.read_text(encoding="utf-8").splitlines()]
    head = rows[0]
    out = [dict(zip(head, r)) for r in rows[1:]]
    out = [r for r in out if r["impact"] == "c1" and r["status"] != "open"]
    assert len(out) == 13, len(out)
    return out


def document(landscape=True):
    doc = Document()
    sec = doc.sections[0]
    if landscape:
        sec.orientation = WD_ORIENT.LANDSCAPE
        sec.page_width, sec.page_height = sec.page_height, sec.page_width
    for s in doc.sections:
        s.left_margin = s.right_margin = Cm(1.8)
        s.top_margin = s.bottom_margin = Cm(1.8)
    st = doc.styles["Normal"]
    st.font.name, st.font.size = "Calibri", Pt(10)
    doc.core_properties.author = ""
    return doc


def table(doc, head, rows, widths):
    t = doc.add_table(rows=1, cols=len(head))
    t.style = "Table Grid"
    for c, h in zip(t.rows[0].cells, head):
        c.text = h
        for r in c.paragraphs[0].runs:
            r.bold = True
    for row in rows:
        cells = t.add_row().cells
        for c, v in zip(cells, row):
            c.text = v
    for row in t.rows:
        for c, w in zip(row.cells, widths):
            c.width = Cm(w)
    return t


def main():
    out = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
    out.mkdir(parents=True, exist_ok=True)
    f = findings()

    doc = document()
    doc.add_heading("Basılı nihai sonucu değiştiren bulguların (c1) bağımsız yeniden hesabı / "
                    "Independent recomputation of the c1 findings", level=1)
    doc.add_paragraph(
        "Amaç: aşağıdaki 13 çözümlü örnekte belgede basılı olan nihai sonucun, belgenin kendi verilerinden yeniden "
        "hesaplandığında aynı çıkıp çıkmadığını bağımsız olarak denetlemek. Lütfen KAPSAM programını, depodaki "
        "dosyaları veya makaleyi kullanmayın; yalnızca belgenin kendisini (gerekirse güncel baskısını ve yayımlanmış "
        "düzeltmelerini) ve kendi tercih ettiğiniz araçları (el hesabı, Excel, R, Python …) kullanın. Her satır için "
        "bulduğunuz değeri, kullandığınız yöntemi ve dikkatinizi çeken noktaları yazın. En az on satır yeterlidir.")
    doc.add_paragraph(
        "Purpose: to check independently whether the final result printed in each of the 13 worked examples below "
        "follows from the document's own data. Please do not use the KAPSAM program, the files of its repository or the "
        "manuscript; use only the document itself (with its current edition and published errata, if any) and tools "
        "of your choice. For every row, record the value you obtain, the method, and any remark. At least ten rows "
        "are needed.")
    rows = [(r["F"], r["document"], r["quantity"], r["printed"], "", "", "") for r in f]
    table(doc, ["F", "Belge, bölüm / Document, section", "Büyüklük / Quantity", "Basılı / Printed",
                "Yeniden hesaplanan / Recomputed", "Yöntem, araç / Method, tool", "Not / Remarks"],
          rows, [1.2, 4.2, 3.6, 2.4, 3.2, 4.2, 6.0])
    doc.add_paragraph("")
    doc.add_paragraph("Hesabı yapan / Recomputed by: ______________________   Kurum / Affiliation: "
                      "______________________   Tarih / Date: ____________")
    doc.add_paragraph("Bu kişi KAPSAM'ın kodunu görmemiş olmalı ve makalenin yazarı olmamalıdır. / The person must not "
                      "have seen the code of KAPSAM and must not be an author of the manuscript.")
    doc.save(out / "c1-recheck-form.docx")

    key = document()
    key.add_heading("Anahtar: programın yeniden hesapladığı değerler / Key: the program's recomputed values", level=1)
    key.add_paragraph("Bu dosyayı form doldurulduktan sonra açın. / Open this file only after the form has been filled "
                      "in. Kaynak / Source: docs/research/class-c-impact.tsv (research log D84).")
    rows = [(r["F"], r["document"], r["quantity"], r["printed"], r["recomputed"], r["status"], r["note"]) for r in f]
    table(key, ["F", "Document", "Quantity", "Printed", "KAPSAM", "Status", "Explanation"],
          rows, [1.2, 4.0, 3.4, 2.2, 2.2, 2.8, 9.0])
    key.save(out / "c1-recheck-key.docx")
    print("wrote", out / "c1-recheck-form.docx", "and", out / "c1-recheck-key.docx")


if __name__ == "__main__":
    main()
