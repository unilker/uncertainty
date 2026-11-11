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
        "fig1": "Figure 1. Architecture and verification of KAPSAM. Generators write the 286 project files of the library "
                "from the numerical data of their sources (37 documents, 25 of them guidance documents and standards); one "
                "engine, with the evaluation types and the documents they implement, serves the command-line tool, the "
                "application and the reports. The results are compared with the reference values and recalculated by "
                "spreadsheet formulas at every change of the code, and were compared with Suncal and with a Monte Carlo "
                "propagation through the calibration fits for this release.",
        "fig2": "Figure 2. Results of a measurement model in the application (Eurachem QUAM Example A1, preparation of a "
                "cadmium calibration standard): reported statement, GUM result and uncertainty budget. The table gives "
                "the coverage probability of k = 2 for a normal distribution (95.45 %) and U unrounded; the statement "
                "uses the conventional wording and U rounded by the rule it names. The budget has further columns (the "
                "Kragten contributions) to the right, and the contributions to the variance are drawn below it.",
        "fig3": "Figure 3. Findings in the worked examples. (a) The 100 findings per source document, each counted once, "
                "by its most severe class; the confirmed findings that change a final printed result (c1) are counted "
                "at the left. Documents with fewer than three findings and no c1 finding are combined. (b) Effect of "
                "each of the 11 c1 findings, in units of the printed expanded uncertainty: change of the estimate "
                "|Δy|/U and of the uncertainty |ΔU|/U.",
        "table2a": "**Table 2a.** Library projects by evaluation type (the eleven types of Table 1; a measurement model "
                   "with a conformity assessment and the conformity of a multicomponent item are shown separately).",
        "table2b": "**Table 2b.** Library projects by field.",
        "table3": "**Table 3.** From sources to findings; each step defined.",
        "note": "Review copy generated on {date} from commit {commit} (`docs/article/manuscript.md`). Lines are "
                "numbered for comments.",
    },
    "tr": {
        "fig1": "Şekil 1. KAPSAM'ın mimarisi ve doğrulanması. Üreteçler kütüphanenin 286 proje dosyasını kaynaklarının "
                "(25'i kılavuz belge ve standart olan 37 belge) sayısal verilerinden yazar; değerlendirme türleri ve "
                "uyguladıkları belgelerle tek bir motor komut satırı aracına, uygulamaya ve raporlara hizmet eder. "
                "Sonuçlar kodun her değişikliğinde referans değerlerle karşılaştırılır ve elektronik tablo formülleriyle "
                "yeniden hesaplanır; bu sürüm için ayrıca Suncal ile ve kalibrasyon uydurmaları boyunca Monte Carlo "
                "yayılımıyla karşılaştırılmıştır (şekildeki metin İngilizcedir).",
        "fig2": "Şekil 2. Uygulamada bir ölçüm modelinin sonuçları (Eurachem QUAM Örnek A1, kadmiyum kalibrasyon "
                "standardının hazırlanması): raporlanan sonuç, GUM sonucu ve belirsizlik bütçesi. Tablo, normal dağılım "
                "için k = 2'nin kapsama olasılığını (%95,45) ve yuvarlanmamış U'yu verir; sonuç ifadesi alışılmış "
                "ifadeyi ve adını verdiği kuralla yuvarlanmış U'yu kullanır. Bütçenin sağında başka sütunlar (Kragten "
                "katkıları), altında varyansa katkıların grafiği vardır (ekran İngilizce arayüzle alınmıştır).",
        "fig3": "Şekil 3. Çözümlü örneklerdeki bulgular. (a) Kaynak belge başına 100 bulgu, her biri bir kez ve en ağır "
                "sınıfıyla sayılmıştır; basılı bir nihai sonucu değiştiren doğrulanmış bulgular (c1) solda sayılmıştır. "
                "Üçten az bulgusu olan ve c1 bulgusu olmayan belgeler birleştirilmiştir. (b) 11 c1 bulgusunun her "
                "birinin basılı genişletilmiş belirsizlik birimiyle etkisi: kestirimdeki değişiklik |Δy|/U ve "
                "belirsizlikteki değişiklik |ΔU|/U (şekildeki metin İngilizcedir).",
        "table2a": "**Tablo 2a.** Kütüphanedeki projeler, değerlendirme türüne göre (Tablo 1'in on bir türü; uygunluk "
                   "değerlendirmeli ölçüm modeli ve çok bileşenli ürünün uygunluğu ayrı gösterilmiştir).",
        "table2b": "**Tablo 2b.** Kütüphanedeki projeler, alana göre.",
        "table3": "**Tablo 3.** Kaynaklardan bulgulara; her adım tanımlanmıştır.",
        "note": "Gözden geçirme kopyası; {date} tarihinde {commit} commit'inden üretilmiştir "
                "(`docs/article/manuscript.tr.md`). Yorumlarda satıra atıf için satırlar numaralıdır.",
    },
}

# Turkish cells of Tables 2 and 3 (the evaluation types are file-format names and stay as they are).
TR_CELLS = {
    "Evaluation type": "Değerlendirme türü", "Projects": "Proje", "Field": "Alan", "Step": "Adım", "Count": "Sayı",
    "Definition": "Tanım",
    "measurement model (GUM, Monte Carlo)": "ölçüm modeli (GUM, Monte Carlo)", "top-down evaluation": "yukarıdan aşağı değerlendirme",
    "control chart": "kontrol grafiği", "conformity assessment": "uygunluk değerlendirmesi",
    "several output quantities": "birden çok çıktı büyüklüğü", "proficiency testing": "yeterlilik testi",
    "consensus value": "uzlaşı değeri", "error of indication of a weighing instrument": "tartı aletinin gösterge hatası",
    "calibration function": "kalibrasyon fonksiyonu", "uncertainty from sampling": "numune almadan kaynaklanan belirsizlik",
    "precision experiment": "kesinlik deneyi",
    "measurement model with conformity assessment": "uygunluk değerlendirmeli ölçüm modeli",
    "conformity of a multicomponent item": "çok bileşenli ürünün uygunluğu",
    "chemistry": "kimya", "electrical": "elektrik", "mass": "kütle", "general": "genel", "physical": "fiziksel",
    "length": "uzunluk", "temperature": "sıcaklık", "flow": "akış", "pressure": "basınç",
    "Primary sources": "Birincil kaynaklar",
    "the first source of each project: 25 guidance documents and standards, 5 BIPM reference data reports, 6 journal papers and books (one book), 1 unpublished procedure":
        "her projenin ilk kaynağı: 25 kılavuz belge ve standart, 5 BIPM referans veri raporu, 6 dergi makalesi ve kitap (biri kitap), 1 yayımlanmamış prosedür",
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
    "24 settled (11 c1, 8 c2, 4 m, 1 e; Table 6 lists the c1 findings), 6 open":
        "24'ü sonuçlandı (11 c1, 8 c2, 4 m, 1 e; c1 bulguları Tablo 6'dadır), 6'sı açık",
}

# Relative column widths (pandoc reads them from the dashes of the separator line).
WIDTHS = {
    "| Project type |": [16, 64, 20], "| Proje türü |": [16, 64, 20],
    "| F | Document |": [5, 13, 15, 9, 9, 9, 40], "| F | Belge |": [5, 13, 15, 9, 9, 9, 40],
    "| Class | Confirmed |": [10, 14, 14, 14, 14, 34], "| Sınıf | Doğrulanmış |": [10, 14, 14, 14, 14, 34],
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


SUBSCRIPTS = {"D*_j": "*D*\\*~*j*~", "E_n": "*E*~n~", "x_j": "*x*~*j*~", "u_j": "*u*~*j*~", "σ_j": "*σ*~*j*~",
              "s_R": "*s*~R~", "s_r": "*s*~r~", "y_CO₂": "*y*~CO₂~", "d_X": "*d*~X~", "c_Cd": "*c*~Cd~", "U_U": "*U*~U~",
              "U_L": "*U*~L~", "T_U": "*T*~U~", "T_L": "*T*~L~", "R_w": "*R*~w~", "pH_X": "pH~X~", "D_j": "*D*~*j*~",
              "ν_eff": "*ν*~eff~"}
_PROTECT = re.compile(r"```.*?```|`[^`\n]*`|https?://\S+|!\[[^\]]*\]\([^)]*\)|<!--.*?-->", re.S)
_VAR = re.compile(r"(?<![\w*/\\.~-])([uUkyxnQ])(?=\s?=\s?[-−\d.(]|\s?=\s?[kuU]\b|\s?±|\(|₉₅|\s?[<>≤≥]\s?[\w(])")
_AFTER = re.compile(r"(?<=[\d₉₅])u\b|(?<=\s)U/k\b")


def math_italics(md):
    """Variables in italics (Elsevier, math formulae): quantity symbols with subscripts and single-letter symbols
    in a mathematical context, outside code, URLs, comments and the reference list."""
    head, sep, refs = md.partition("\n## Reference")
    if not sep:
        head, sep, refs = md.partition("\n## Kaynaklar")
    out, last = [], 0
    for m in _PROTECT.finditer(head):
        out.append(_italic(head[last:m.start()]))
        out.append(m.group(0))
        last = m.end()
    out.append(_italic(head[last:]))
    return "".join(out) + sep + refs


def _italic(s):
    for a, b in SUBSCRIPTS.items():
        s = re.sub(r"(?<![\w*])" + re.escape(a) + r"(?![A-Za-z0-9])", lambda m: b, s)
    s = _VAR.sub(r"*\1*", s)
    s = _AFTER.sub(lambda m: "*U*/*k*" if m.group(0) == "U/k" else "*u*", s)
    return s


def assemble(lang, commit):
    src = ART / ("manuscript.md" if lang == "en" else "manuscript.tr.md")
    md = src.read_text(encoding="utf-8")
    cap = CAPTIONS[lang]
    title, rest = md.split("\n", 1)
    title = title.lstrip("# ").strip()
    tabs = tables_md()
    if lang == "tr":
        tabs = {k: translate_table(v) for k, v in tabs.items()}
    figs = {"fig1": "fig1_architecture", "fig2": "fig2_application", "fig3": "fig3_findings_by_document"}
    for key, name in figs.items():
        img = (FIG / f"{name}.png").resolve()
        rest = rest.replace(f"<!-- {key} -->", f"![{cap[key]}]({img}){{width=16cm}}")
    rest = rest.replace("<!-- table2 -->", f"{cap['table2a']}\n\n{tabs['table2a']}\n\n{cap['table2b']}\n\n{tabs['table2b']}")
    rest = rest.replace("<!-- table3 -->", f"{cap['table3']}\n\n{tabs['table3']}")
    assert "<!--" not in rest, f"unreplaced marker in {src}"
    note = "*" + cap["note"].format(date=date.today().isoformat(), commit=commit) + "*\n\n"
    return title, note + loose_lists(set_widths(math_italics(rest)))


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
