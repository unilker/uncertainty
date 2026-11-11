"""Blind recomputation form for the confirmed c1 findings (Table 4 of the article; Section 9: "the recomputation
of at least ten of the c1 findings by a person who has not seen the code").

    python3 tools/research/c1_recheck.py OUT_DIR [--round2]

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
# Second round (D106): where to start for each finding. The first round showed that a recomputation which starts
# from the totals or tables printed in the document reproduces the printed result by construction; each finding is
# about how a printed value follows from the data before it. The program's values are still not shown.
START = {
    "F21": "Δl as the mean of the five observations and the coaxiality correction from Eq. (S13.5); not the values "
           "entered in the budget S13.10.",
    "F28": "The equation printed in E4 with the printed Δ, p, T and R_s, evaluated by hand; not the NUM example file.",
    "F39": "The flour components of Table C.4, combined by the formula of C.3; compare with the printed u and U.",
    "F40": "The raw data of Table J.2 with the stated outlier exclusions; not the summary in Table J.5.",
    "F42": "Formula (15) for y = 4950.6 with the parameters and their covariances of the example; not Table B.12.",
    "F56": "Each u_i of K.5.9 from its stated limit and distribution, then their combination; not the printed u_c.",
    "F57": "The mismatch term from formula E5 (E.9.5) with the coefficients of K.3.6, then u_c and U.",
    "F58": "The sensitivity coefficients from the model of K.6.1, −L(α·Δt + Δα·ΔT), then u_c and U.",
    "F77": "An own Monte Carlo evaluation (e.g. 10⁶ trials) with the input distributions stated in the example.",
    "F78": "The measured data of the example and its measurement model; not the pH value printed in the result.",
    "F82": "The input quantities of the example (Table 4.1), propagated by the law of propagation of uncertainty.",
    "F87": "The column u(δ̂)/x_ref of Table E3.4.2 summarised as the text says (root mean square), then U for Pb.",
    "F90": "Eq. (E3.3.19) with the inputs of the example; report the result to three significant digits.",
}


def findings():
    rows = [ln.split("\t") for ln in TSV.read_text(encoding="utf-8").splitlines()]
    head = rows[0]
    out = [dict(zip(head, r)) for r in rows[1:]]
    out = [r for r in out if r["impact"] == "c1" and r["status"] != "open"]
    assert len(out) == 11, len(out)  # F40 moved to class m (D108), F87 to c2 (D110)
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
    t.autofit = False
    for col, w in zip(t.columns, widths):
        col.width = Cm(w)
    for row in t.rows:
        for c, w in zip(row.cells, widths):
            c.width = Cm(w)
    return t


ROUND2 = "--round2" in sys.argv


def main():
    out = Path(next((a for a in sys.argv[1:] if not a.startswith("--")), "."))
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
    if ROUND2:
        doc.add_paragraph(
            "İkinci tur: her satırda belgede basılı ara toplamlardan veya özet tablolardan değil, \"Başlangıç\" "
            "sütunundaki verilerden başlayın ve ara adımları da yazın. / Second round: in every row, start from the "
            "data named in the column \"Start from\", not from totals or summary tables printed in the document, and "
            "record the intermediate steps.")
        rows = [(r["F"], r["document"], r["quantity"], r["printed"], START[r["F"]], "", "", "") for r in f]
        table(doc, ["F", "Belge / Document", "Büyüklük / Quantity", "Basılı / Printed", "Başlangıç / Start from",
                    "Yeniden hesaplanan / Recomputed", "Ara adımlar, araç / Steps, tool", "Not / Remarks"],
              rows, [1.0, 3.0, 2.8, 1.8, 6.2, 2.6, 4.2, 3.2])
    else:
        rows = [(r["F"], r["document"], r["quantity"], r["printed"], "", "", "") for r in f]
        table(doc, ["F", "Belge, bölüm / Document, section", "Büyüklük / Quantity", "Basılı / Printed",
                    "Yeniden hesaplanan / Recomputed", "Yöntem, araç / Method, tool", "Not / Remarks"],
              rows, [1.2, 4.2, 3.6, 2.4, 3.2, 4.2, 6.0])
    doc.add_paragraph("")
    doc.add_paragraph("Hesabı yapan / Recomputed by: ______________________   Kurum / Affiliation: "
                      "______________________   Tarih / Date: ____________")
    doc.add_paragraph("Bu kişi KAPSAM'ın kodunu görmemiş olmalı ve makalenin yazarı olmamalıdır. / The person must not "
                      "have seen the code of KAPSAM and must not be an author of the manuscript.")
    doc.save(out / ("c1-recheck-form-round2.docx" if ROUND2 else "c1-recheck-form.docx"))

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
