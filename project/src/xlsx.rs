//! A minimal writer for Office Open XML spreadsheets (.xlsx, ECMA-376):
//! text, numbers and formulas in several sheets, with a few cell styles.
//!
//! Formulas are stored without cached results and the workbook asks for a
//! full calculation when it is opened (`calcPr fullCalcOnLoad`), so the
//! values a user sees are computed by the spreadsheet program itself, not by
//! this software. The file is a ZIP archive with fixed timestamps, so the
//! same workbook always gives the same bytes.

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Cell styles (indices into `cellXfs` of the style sheet).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Normal = 0,
    Bold = 1,
    /// Scientific notation with ten significant digits.
    Number = 2,
    BoldNumber = 3,
    /// Bold white text on navy, for table headings.
    Heading = 4,
    /// Italic grey text, for notes.
    Note = 5,
    /// Large bold text, for the title.
    Title = 6,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Text(String, Style),
    Number(f64, Style),
    /// A formula without the leading `=`.
    Formula(String, Style),
}

#[derive(Clone, Debug, Default)]
pub struct Sheet {
    pub name: String,
    cells: BTreeMap<(u32, u32), Cell>,
    widths: BTreeMap<u32, f64>,
}

impl Sheet {
    pub fn new(name: &str) -> Sheet {
        Sheet { name: name.to_string(), ..Sheet::default() }
    }

    pub fn text(&mut self, row: u32, col: u32, s: impl Into<String>, style: Style) {
        self.cells.insert((row, col), Cell::Text(s.into(), style));
    }

    /// A number; non-finite values are written as text, which spreadsheets
    /// cannot store as numbers.
    pub fn number(&mut self, row: u32, col: u32, v: f64, style: Style) {
        if v.is_finite() {
            self.cells.insert((row, col), Cell::Number(v, style));
        } else {
            self.cells.insert((row, col), Cell::Text(format!("{v}"), style));
        }
    }

    pub fn formula(&mut self, row: u32, col: u32, f: impl Into<String>, style: Style) {
        self.cells.insert((row, col), Cell::Formula(f.into(), style));
    }

    pub fn width(&mut self, col: u32, chars: f64) {
        self.widths.insert(col, chars);
    }

    pub fn get(&self, row: u32, col: u32) -> Option<&Cell> {
        self.cells.get(&(row, col))
    }

    fn xml(&self) -> String {
        let mut s = String::from(XML_HEAD);
        s.push_str(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">"#);
        if !self.widths.is_empty() {
            s.push_str("<cols>");
            for (c, w) in &self.widths {
                let _ = write!(s, r#"<col min="{0}" max="{0}" width="{1}" customWidth="1"/>"#, c + 1, w);
            }
            s.push_str("</cols>");
        }
        s.push_str("<sheetData>");
        let mut current: Option<u32> = None;
        for (&(r, c), cell) in &self.cells {
            if current != Some(r) {
                if current.is_some() {
                    s.push_str("</row>");
                }
                let _ = write!(s, r#"<row r="{}">"#, r + 1);
                current = Some(r);
            }
            let at = cell_ref(r, c);
            match cell {
                Cell::Text(t, st) => {
                    let _ = write!(
                        s,
                        r#"<c r="{at}" s="{}" t="inlineStr"><is><t xml:space="preserve">{}</t></is></c>"#,
                        *st as u8,
                        escape(t)
                    );
                }
                Cell::Number(v, st) => {
                    let _ = write!(s, r#"<c r="{at}" s="{}"><v>{v:E}</v></c>"#, *st as u8);
                }
                Cell::Formula(f, st) => {
                    let _ = write!(s, r#"<c r="{at}" s="{}"><f>{}</f></c>"#, *st as u8, escape(f));
                }
            }
        }
        if current.is_some() {
            s.push_str("</row>");
        }
        s.push_str("</sheetData></worksheet>");
        s
    }
}

#[derive(Clone, Debug, Default)]
pub struct Workbook {
    pub sheets: Vec<Sheet>,
}

impl Workbook {
    /// The workbook as the bytes of an .xlsx file.
    pub fn to_bytes(&self) -> Vec<u8> {
        let n = self.sheets.len();
        let mut parts: Vec<(String, Vec<u8>)> = Vec::new();
        let mut types = String::from(XML_HEAD);
        types.push_str(r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"/>"#);
        for i in 1..=n {
            let _ = write!(
                types,
                r#"<Override PartName="/xl/worksheets/sheet{i}.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>"#
            );
        }
        types.push_str("</Types>");
        parts.push(("[Content_Types].xml".into(), types.into_bytes()));
        parts.push((
            "_rels/.rels".into(),
            format!(r#"{XML_HEAD}<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#)
                .into_bytes(),
        ));
        let mut wb = String::from(XML_HEAD);
        wb.push_str(r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets>"#);
        for (i, sh) in self.sheets.iter().enumerate() {
            let _ = write!(wb, r#"<sheet name="{}" sheetId="{}" r:id="rId{}"/>"#, escape(&sh.name), i + 1, i + 1);
        }
        wb.push_str(r#"</sheets><calcPr calcId="191029" fullCalcOnLoad="1"/></workbook>"#);
        parts.push(("xl/workbook.xml".into(), wb.into_bytes()));
        let mut rels = String::from(XML_HEAD);
        rels.push_str(r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#);
        for i in 1..=n {
            let _ = write!(
                rels,
                r#"<Relationship Id="rId{i}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet{i}.xml"/>"#
            );
        }
        let _ = write!(
            rels,
            r#"<Relationship Id="rId{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/></Relationships>"#,
            n + 1
        );
        parts.push(("xl/_rels/workbook.xml.rels".into(), rels.into_bytes()));
        parts.push(("xl/styles.xml".into(), format!("{XML_HEAD}{STYLES}").into_bytes()));
        for (i, sh) in self.sheets.iter().enumerate() {
            parts.push((format!("xl/worksheets/sheet{}.xml", i + 1), sh.xml().into_bytes()));
        }
        zip(&parts)
    }
}

const XML_HEAD: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n";

const STYLES: &str = r#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><numFmts count="1"><numFmt numFmtId="164" formatCode="0.000000000E+00"/></numFmts><fonts count="5"><font><sz val="11"/><name val="Calibri"/></font><font><b/><sz val="11"/><name val="Calibri"/></font><font><b/><sz val="11"/><color rgb="FFFFFFFF"/><name val="Calibri"/></font><font><i/><sz val="10"/><color rgb="FF595959"/><name val="Calibri"/></font><font><b/><sz val="14"/><color rgb="FF1B2F5E"/><name val="Calibri"/></font></fonts><fills count="3"><fill><patternFill patternType="none"/></fill><fill><patternFill patternType="gray125"/></fill><fill><patternFill patternType="solid"><fgColor rgb="FF1B2F5E"/><bgColor indexed="64"/></patternFill></fill></fills><borders count="1"><border><left/><right/><top/><bottom/><diagonal/></border></borders><cellStyleXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0"/></cellStyleXfs><cellXfs count="7"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/><xf numFmtId="0" fontId="1" fillId="0" borderId="0" xfId="0" applyFont="1"/><xf numFmtId="164" fontId="0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/><xf numFmtId="164" fontId="1" fillId="0" borderId="0" xfId="0" applyNumberFormat="1" applyFont="1"/><xf numFmtId="0" fontId="2" fillId="2" borderId="0" xfId="0" applyFont="1" applyFill="1"/><xf numFmtId="0" fontId="3" fillId="0" borderId="0" xfId="0" applyFont="1"/><xf numFmtId="0" fontId="4" fillId="0" borderId="0" xfId="0" applyFont="1"/></cellXfs><cellStyles count="1"><cellStyle name="Normal" xfId="0" builtinId="0"/></cellStyles></styleSheet>"#;

/// "A1"-style reference of a zero-based cell.
pub fn cell_ref(row: u32, col: u32) -> String {
    format!("{}{}", col_name(col), row + 1)
}

/// Absolute reference "$A$1".
pub fn abs_ref(row: u32, col: u32) -> String {
    format!("${}${}", col_name(col), row + 1)
}

/// Reference to a cell of another sheet, `'Sheet'!$A$1`.
pub fn sheet_ref(sheet: &str, row: u32, col: u32) -> String {
    format!("'{}'!{}", sheet.replace('\'', "''"), abs_ref(row, col))
}

pub fn col_name(mut col: u32) -> String {
    let mut s = Vec::new();
    loop {
        s.push(b'A' + (col % 26) as u8);
        if col < 26 {
            break;
        }
        col = col / 26 - 1;
    }
    s.reverse();
    String::from_utf8(s).unwrap_or_default()
}

/// A number literal for formulas: the shortest form that
/// reads back to the same double, negative numbers in parentheses.
pub fn number(v: f64) -> String {
    let s = format!("{v:E}");
    // Rust writes "1.5E0", "1E-6"; both forms are valid in OOXML and in formulas.
    if v < 0.0 {
        format!("({s})")
    } else {
        s
    }
}

fn escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            c if (c as u32) < 0x20 && c != '\n' && c != '\t' => {}
            c => o.push(c),
        }
    }
    o
}

fn crc32(data: &[u8]) -> u32 {
    let mut table = [0u32; 256];
    for (i, t) in table.iter_mut().enumerate() {
        let mut c = i as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
        }
        *t = c;
    }
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c = table[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

/// A ZIP archive of the parts, deflated, with the timestamp 1980-01-01 00:00.
fn zip(parts: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut central = Vec::new();
    let le16 = |v: &mut Vec<u8>, x: u16| v.extend_from_slice(&x.to_le_bytes());
    let le32 = |v: &mut Vec<u8>, x: u32| v.extend_from_slice(&x.to_le_bytes());
    for (name, data) in parts {
        let crc = crc32(data);
        let packed = miniz_oxide::deflate::compress_to_vec(data, 6);
        let offset = out.len() as u32;
        le32(&mut out, 0x0403_4b50);
        le16(&mut out, 20);
        le16(&mut out, 0);
        le16(&mut out, 8);
        le16(&mut out, 0);
        le16(&mut out, 0x0021);
        le32(&mut out, crc);
        le32(&mut out, packed.len() as u32);
        le32(&mut out, data.len() as u32);
        le16(&mut out, name.len() as u16);
        le16(&mut out, 0);
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&packed);

        le32(&mut central, 0x0201_4b50);
        le16(&mut central, 20);
        le16(&mut central, 20);
        le16(&mut central, 0);
        le16(&mut central, 8);
        le16(&mut central, 0);
        le16(&mut central, 0x0021);
        le32(&mut central, crc);
        le32(&mut central, packed.len() as u32);
        le32(&mut central, data.len() as u32);
        le16(&mut central, name.len() as u16);
        le16(&mut central, 0);
        le16(&mut central, 0);
        le16(&mut central, 0);
        le16(&mut central, 0);
        le32(&mut central, 0);
        le32(&mut central, offset);
        central.extend_from_slice(name.as_bytes());
    }
    let cd_offset = out.len() as u32;
    let cd_size = central.len() as u32;
    out.extend_from_slice(&central);
    le32(&mut out, 0x0605_4b50);
    le16(&mut out, 0);
    le16(&mut out, 0);
    le16(&mut out, parts.len() as u16);
    le16(&mut out, parts.len() as u16);
    le32(&mut out, cd_size);
    le32(&mut out, cd_offset);
    le16(&mut out, 0);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_and_names() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(col_name(0), "A");
        assert_eq!(col_name(25), "Z");
        assert_eq!(col_name(26), "AA");
        assert_eq!(col_name(701), "ZZ");
        assert_eq!(col_name(702), "AAA");
        assert_eq!(number(-0.5), "(-5E-1)");
        assert_eq!("1E-6".parse::<f64>().unwrap(), 1e-6);
    }

    #[test]
    fn workbook_is_a_zip_with_the_parts() {
        let mut s = Sheet::new("A & B");
        s.text(0, 0, "x < y", Style::Bold);
        s.number(1, 0, 2.5, Style::Number);
        s.formula(1, 1, "A2*2", Style::Number);
        let bytes = Workbook { sheets: vec![s] }.to_bytes();
        assert_eq!(&bytes[..4], b"PK\x03\x04");
        assert_eq!(
            Workbook { sheets: vec![Sheet::new("x")] }.to_bytes(),
            Workbook { sheets: vec![Sheet::new("x")] }.to_bytes()
        );
    }
}
