"""Review copies of the article as Word files, in English and Turkish.

    python3 tools/research/article_docx.py OUT_DIR

reads docs/article/manuscript.md (English, the text of record) and docs/article/manuscript.tr.md (its Turkish
translation), puts Figures 1-4 (docs/article/figures/*.png) and Tables 2-3 (docs/article/tables.md) at the
markers <!-- fig1 --> ... <!-- table3 --> and converts both with pandoc. The Word files have A4 pages, 1.5 line
spacing, continuous line numbers and page numbers, so that comments can cite a line. Run
`python3 tools/research/article_figures.py` first if the figures or tables are out of date. Needs pandoc.
"""
import re
import subprocess
import sys
import tempfile
import zipfile
from datetime import date
from pathlib import Path

ART = Path("docs/article")
FIG = ART / "figures"

CAPTIONS = {
    "en": {
        "fig1": "Figure 1. Architecture of the program: generators write the project files of the library from the numerical "
                "data of their sources; one engine evaluates them for the command-line tool, the application and the tests.",
        "fig2": "Figure 2. Results of a measurement model in the application (Eurachem QUAM Example A1, preparation of a "
                "cadmium calibration standard): reported statement, GUM result, uncertainty budget and contributions to the "
                "combined variance.",
        "fig3": "Figure 3. Findings per source document by class (a: intermediate rounding, b: misprint without "
                "effect, c: inconsistency, n: no discrepancy); documents with fewer than three findings are combined, "
                "and a finding with two classes counts in both.",
        "fig4": "Figure 4. (a) Consensus value of the Newtonian constant of gravitation by procedure (16 results, "
                "Merkatas et al. [52]), 95 % intervals. (b) Leave-one-out degrees of equivalence of the ⁶⁰Co activity "
                "comparison of the NICOB manual [23] by procedure, with the number of significant bilateral pairs.",
        "table2a": "**Table 2a.** Library projects by evaluation type.",
        "table2b": "**Table 2b.** Library projects by field.",
        "table3": "**Table 3.** From sources to findings; each step defined.",
        "note": "Review copy generated on {date} from commit {commit} (`docs/article/manuscript.md`). Lines are "
                "numbered for comments.",
    },
    "tr": {
        "fig1": "Şekil 1. Programın mimarisi: üreteçler kütüphanenin proje dosyalarını kaynaklarının sayısal verilerinden "
                "yazar; tek bir motor onları komut satırı aracı, uygulama ve testler için değerlendirir (şekildeki metin "
                "İngilizcedir).",
        "fig2": "Şekil 2. Uygulamada bir ölçüm modelinin sonuçları (Eurachem QUAM Örnek A1, kadmiyum kalibrasyon "
                "standardının hazırlanması): raporlanan sonuç, GUM sonucu, belirsizlik bütçesi ve birleşik varyansa "
                "katkılar (ekran İngilizce arayüzle alınmıştır).",
        "fig3": "Şekil 3. Kaynak belge başına bulgular, sınıfa göre (a: ara yuvarlama, b: etkisiz baskı hatası, "
                "c: tutarsızlık, n: tutarsızlık yok); üçten az bulgusu olan belgeler birleştirilmiştir ve iki sınıflı "
                "bir bulgu her ikisinde de sayılır (şekildeki metin İngilizcedir).",
        "fig4": "Şekil 4. (a) Newton kütle çekim sabitinin yönteme göre uzlaşı değeri (16 sonuç, Merkatas ve ark. "
                "[52]), %95 aralıkları. (b) NICOB kılavuzundaki [23] ⁶⁰Co aktivite karşılaştırmasının yönteme göre "
                "birini dışarıda bırakma eşdeğerlik dereceleri ve anlamlı ikili çiftlerin sayısı (şekildeki metin "
                "İngilizcedir).",
        "table2a": "**Tablo 2a.** Kütüphanedeki projeler, değerlendirme türüne göre.",
        "table2b": "**Tablo 2b.** Kütüphanedeki projeler, alana göre.",
        "table3": "**Tablo 3.** Kaynaklardan bulgulara; her adım tanımlanmıştır.",
        "note": "Gözden geçirme kopyası; {date} tarihinde {commit} commit'inden üretilmiştir "
                "(`docs/article/manuscript.tr.md`). Yorumlarda satıra atıf için satırlar numaralıdır.",
    },
}

# Turkish cells of Tables 2 and 3 (the evaluation types are file-format names and stay as they are).
TR_CELLS = {
    "Section": "Tür", "Projects": "Proje", "Field": "Alan", "Step": "Adım", "Count": "Sayı", "Definition": "Tanım",
    "chemistry": "kimya", "electrical": "elektrik", "mass": "kütle", "general": "genel", "physical": "fiziksel",
    "length": "uzunluk", "temperature": "sıcaklık", "flow": "akış", "pressure": "basınç",
    "Source documents": "Kaynak belgeler",
    "documents cited as the source of at least one project": "en az bir projenin kaynağı olarak gösterilen belgeler",
    "project files in `library/`; one worked example may give several (variants)":
        "`library/` içindeki proje dosyaları; bir çözümlü örnek birden çok proje verebilir (türevler)",
    "Validation-matrix rows ✅": "Validasyon matrisi satırları ✅",
    "worked examples or variants reproduced; 25 carry a finding":
        "yeniden üretilen çözümlü örnekler veya türevleri; 25'inde bulgu var",
    "Compared values": "Karşılaştırılan değerler",
    "reference values checked at every test run": "her test çalıştırmasında denetlenen referans değerleri",
    "Findings": "Bulgular",
    "F1–F100 of the research log (a, b, c, n; a finding may cover several examples)":
        "araştırma günlüğünün F1–F100 bulguları (a, b, c, n; bir bulgu birden çok örneği kapsayabilir)",
    "Class-c findings": "c sınıfı bulgular",
    "of which c1 confirmed 13 (table above)": "bunların 13'ü doğrulanmış c1 (Tablo 4)",
    "Projects": "Projeler",
}

# Relative column widths (pandoc reads them from the dashes of the separator line).
WIDTHS = {
    "| Project type |": [16, 64, 20], "| Proje türü |": [16, 64, 20],
    "| F | Document |": [6, 16, 17, 10, 15, 36], "| F | Belge |": [6, 16, 17, 10, 15, 36],
    "| Step |": [22, 8, 70], "| Adım |": [22, 8, 70],
    "| Laboratory |": [14, 24, 17, 25, 20], "| Laboratuvar |": [14, 24, 17, 25, 20],
}


def tables_md():
    md = (ART / "tables.md").read_text(encoding="utf-8")
    out = {}
    for key, title in (("table2a", "Table 2a."), ("table2b", "Table 2b."), ("table3", "Table 3.")):
        body = md.split(f"**{title}**")[1].split("\n\n", 2)[1]
        out[key] = body.strip()
    return out


def translate_table(body):
    rows = []
    for line in body.splitlines():
        if re.fullmatch(r"\|[-:| ]+\|", line):
            rows.append(line)
            continue
        cells = [c.strip() for c in line.strip("|").split("|")]
        cells = [TR_CELLS.get(c, c) for c in cells]
        rows.append("| " + " | ".join(cells) + " |")
    return "\n".join(rows)


def set_widths(md):
    lines = md.splitlines()
    for i, line in enumerate(lines[:-1]):
        for head, w in WIDTHS.items():
            if line.startswith(head) and re.fullmatch(r"\|[-:| ]+\|", lines[i + 1]):
                lines[i + 1] = "|" + "|".join("-" * n for n in w) + "|"
    return "\n".join(lines)


def loose_lists(md):
    """Blank lines between list items, so that pandoc sets them in the body style rather than the small style
    of table cells."""
    lines = md.split("\n")
    out = []
    item = re.compile(r"^(- |\d+\. )")
    in_code = False
    for i, line in enumerate(lines):
        if line.startswith("```"):
            in_code = not in_code
        out.append(line)
        nxt = lines[i + 1] if i + 1 < len(lines) else ""
        if not in_code and item.match(nxt) and line.strip() and (item.match(line) or line.startswith("   ")):
            out.append("")
    return "\n".join(out)


def assemble(lang, commit):
    src = ART / ("manuscript.md" if lang == "en" else "manuscript.tr.md")
    md = src.read_text(encoding="utf-8")
    cap = CAPTIONS[lang]
    title, rest = md.split("\n", 1)
    title = title.lstrip("# ").strip()
    tabs = tables_md()
    if lang == "tr":
        tabs = {k: translate_table(v) for k, v in tabs.items()}
    figs = {"fig1": "fig1_architecture", "fig2": "fig2_application", "fig3": "fig3_findings_by_document",
            "fig4": "fig4_procedures"}
    for key, name in figs.items():
        img = (FIG / f"{name}.png").resolve()
        rest = rest.replace(f"<!-- {key} -->", f"![{cap[key]}]({img}){{width=16cm}}")
    rest = rest.replace("<!-- table2 -->", f"{cap['table2a']}\n\n{tabs['table2a']}\n\n{cap['table2b']}\n\n{tabs['table2b']}")
    rest = rest.replace("<!-- table3 -->", f"{cap['table3']}\n\n{tabs['table3']}")
    assert "<!--" not in rest, f"unreplaced marker in {src}"
    note = "*" + cap["note"].format(date=date.today().isoformat(), commit=commit) + "*\n\n"
    return title, note + loose_lists(set_widths(rest))


STYLE_FONT = '<w:rFonts w:ascii="Times New Roman" w:hAnsi="Times New Roman" w:eastAsia="Times New Roman" w:cs="Times New Roman" />'
FOOTER = ('<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
          '<w:ftr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">'
          '<w:p><w:pPr><w:jc w:val="center"/></w:pPr>'
          '<w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText xml:space="preserve"> PAGE </w:instrText></w:r>'
          '<w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>1</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r>'
          '</w:p></w:ftr>')


def reference_docx(path, lang):
    """pandoc's default reference document with Times New Roman, 1.5 spacing, ruled tables, A4, line numbers and
    a page-number footer."""
    default = subprocess.run(["pandoc", "--print-default-data-file", "reference.docx"], capture_output=True,
                             check=True).stdout
    tmp = Path(path).with_suffix(".src.docx")
    tmp.write_bytes(default)
    with zipfile.ZipFile(tmp) as zin, zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as zout:
        for item in zin.infolist():
            data = zin.read(item.filename)
            if item.filename == "word/styles.xml":
                s = data.decode("utf-8")
                s = re.sub(r"<w:rFonts [^>]*/>", STYLE_FONT, s)
                s = re.sub(r'<w:color w:val="[0-9A-F]+" w:themeColor="accent1"( w:themeShade="[0-9A-F]+")? />', "", s)
                s = s.replace('<w:sz w:val="24" />\n        <w:szCs w:val="24" />',
                              '<w:sz w:val="22" />\n        <w:szCs w:val="22" />', 1)
                s = s.replace('w:val="en-US" w:eastAsia', f'w:val="{"tr-TR" if lang == "tr" else "en-GB"}" w:eastAsia', 1)
                s = s.replace('<w:pPrDefault>\n      <w:pPr>\n        <w:spacing w:after="200" />',
                              '<w:pPrDefault>\n      <w:pPr>\n        <w:spacing w:after="120" w:line="360" w:lineRule="auto" />', 1)
                s = s.replace('<w:spacing w:before="180" w:after="180" />',
                              '<w:spacing w:before="0" w:after="120" w:line="360" w:lineRule="auto" />')
                s = re.sub(r'(<w:style [^>]*w:styleId="Compact">.*?)<w:spacing w:before="36" w:after="36" />'
                           r'(\s*</w:pPr>)', r'\1<w:spacing w:before="20" w:after="20" w:line="240" '
                           r'w:lineRule="auto" />\2<w:rPr><w:sz w:val="18" /><w:szCs w:val="18" /></w:rPr>', s,
                           count=1, flags=re.S)
                s = s.replace('<w:tblInd w:w="0" w:type="dxa" />\n      <w:tblCellMar>',
                              '<w:tblInd w:w="0" w:type="dxa" />\n      <w:tblBorders>'
                              '<w:top w:val="single" w:sz="8" w:space="0" w:color="000000"/>'
                              '<w:bottom w:val="single" w:sz="8" w:space="0" w:color="000000"/>'
                              '<w:insideH w:val="single" w:sz="2" w:space="0" w:color="BFBFBF"/>'
                              '</w:tblBorders>\n      <w:tblCellMar>', 1)
                data = s.encode("utf-8")
            elif item.filename == "word/document.xml":
                s = data.decode("utf-8")
                sect = ('<w:sectPr><w:footerReference w:type="default" r:id="rIdFooterPage"/>'
                        '<w:pgSz w:w="11906" w:h="16838"/>'
                        '<w:pgMar w:top="1418" w:right="1418" w:bottom="1418" w:left="1418" w:header="709" '
                        'w:footer="709" w:gutter="0"/><w:lnNumType w:countBy="1" w:distance="284" '
                        'w:restart="continuous"/></w:sectPr>')
                s = re.sub(r"<w:sectPr.*?</w:sectPr>|<w:sectPr[^>]*/>", sect, s, flags=re.S)
                if "<w:sectPr>" not in s:
                    s = s.replace("</w:body>", sect + "</w:body>")
                if 'xmlns:r=' not in s.split(">", 2)[1]:
                    s = s.replace("<w:document ", '<w:document xmlns:r="http://schemas.openxmlformats.org/'
                                  'officeDocument/2006/relationships" ', 1)
                data = s.encode("utf-8")
            elif item.filename == "word/_rels/document.xml.rels":
                s = data.decode("utf-8").replace(
                    "</Relationships>",
                    '<Relationship Id="rIdFooterPage" Type="http://schemas.openxmlformats.org/officeDocument/2006/'
                    'relationships/footer" Target="footer1.xml"/></Relationships>')
                data = s.encode("utf-8")
            elif item.filename == "[Content_Types].xml":
                s = data.decode("utf-8").replace(
                    "</Types>",
                    '<Override PartName="/word/footer1.xml" ContentType="application/vnd.openxmlformats-'
                    'officedocument.wordprocessingml.footer+xml"/></Types>')
                data = s.encode("utf-8")
            if not item.filename.endswith("/"):
                zout.writestr(item.filename, data)
        zout.writestr("word/footer1.xml", FOOTER)
    tmp.unlink()


def main():
    out = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
    out.mkdir(parents=True, exist_ok=True)
    commit = subprocess.run(["git", "rev-parse", "--short", "HEAD"], capture_output=True, text=True).stdout.strip()
    names = {"en": "manuscript-en.docx", "tr": "manuscript-tr.docx"}
    with tempfile.TemporaryDirectory() as tmp:
        for lang, name in names.items():
            title, md = assemble(lang, commit)
            src = Path(tmp) / f"{lang}.md"
            src.write_text(md, encoding="utf-8")
            ref = Path(tmp) / f"ref-{lang}.docx"
            reference_docx(ref, lang)
            subprocess.run(["pandoc", str(src), "-f", "markdown", "-o",
                            str(out / name), "--reference-doc", str(ref), "--metadata", f"title={title}",
                            "--metadata", f"lang={'tr-TR' if lang == 'tr' else 'en-GB'}"], check=True)
            print("wrote", out / name)


if __name__ == "__main__":
    main()
