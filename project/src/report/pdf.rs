//! PDF reports: the Markdown report of any project laid out on A4 pages
//! (ISO/IEC 17025:2017 7.8 reports, 7.5 records). The subset of Markdown the
//! report writers produce is rendered: headings, paragraphs with bold text,
//! bullet and numbered lists, tables with column alignment (the header row is
//! repeated after a page break), horizontal rules. Charts stay in the HTML
//! report. Text is set in DejaVu Sans (regular and bold, embedded, Bitstream
//! Vera licence in `project/fonts/LICENSE`), which covers the Turkish, Greek
//! and mathematical characters of the reports. The output is deterministic:
//! the same project gives the same bytes.

use pdf_writer::types::{CidFontType, FontFlags, SystemInfo, UnicodeCmap};
use pdf_writer::{Content, Filter, Finish, Name, Pdf, Rect, Ref, Str, TextStr};
use std::collections::BTreeMap;

static REGULAR: &[u8] = include_bytes!("../../fonts/DejaVuSans.ttf");
static BOLD: &[u8] = include_bytes!("../../fonts/DejaVuSans-Bold.ttf");

const PAGE_W: f32 = 595.28;
const PAGE_H: f32 = 841.89;
const MARGIN_X: f32 = 54.0;
const MARGIN_TOP: f32 = 56.0;
const MARGIN_BOTTOM: f32 = 62.0;
const BODY: f32 = 9.5;
const TABLE: f32 = 7.6;

struct Font {
    face: ttf_parser::Face<'static>,
    data: &'static [u8],
    name: &'static str,
    upem: f32,
    /// Glyphs used, with a character for the ToUnicode map.
    used: BTreeMap<u16, char>,
}

impl Font {
    fn new(data: &'static [u8], name: &'static str) -> Self {
        let face = ttf_parser::Face::parse(data, 0).expect("embedded font");
        let upem = face.units_per_em() as f32;
        Font { face, data, name, upem, used: BTreeMap::new() }
    }

    fn glyph(&self, c: char) -> u16 {
        self.face.glyph_index(c).map_or(0, |g| g.0)
    }

    fn advance(&self, g: u16) -> f32 {
        self.face.glyph_hor_advance(ttf_parser::GlyphId(g)).unwrap_or(0) as f32 / self.upem
    }

    fn width(&self, text: &str, size: f32) -> f32 {
        text.chars().map(|c| self.advance(self.glyph(c))).sum::<f32>() * size
    }

    /// Glyph ids as 2-byte big-endian codes (Identity-H), recording the use.
    fn encode(&mut self, text: &str) -> Vec<u8> {
        let mut out = Vec::with_capacity(2 * text.len());
        for c in text.chars() {
            let g = self.glyph(c);
            self.used.entry(g).or_insert(c);
            out.extend_from_slice(&g.to_be_bytes());
        }
        out
    }
}

/// A run of text in one style.
#[derive(Clone, Debug, PartialEq)]
struct Span {
    text: String,
    bold: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Align {
    Left,
    Right,
    Center,
}

#[derive(Clone, Debug, PartialEq)]
enum Block {
    Heading(usize, Vec<Span>),
    Paragraph(Vec<Vec<Span>>),
    Item(String, Vec<Span>),
    Table(Vec<Align>, Vec<Vec<Vec<Span>>>),
    Rule,
}

/// Inline Markdown: **bold**, *italic* (set upright), `code`, [text](url)
/// and <url> are reduced to text runs.
fn spans(text: &str, bold: bool) -> Vec<Span> {
    let mut out: Vec<Span> = Vec::new();
    let mut cur = String::new();
    let mut b = bold;
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    let push = |out: &mut Vec<Span>, cur: &mut String, b: bool| {
        if !cur.is_empty() {
            out.push(Span { text: std::mem::take(cur), bold: b });
        }
    };
    while i < chars.len() {
        let c = chars[i];
        if c == '*' && chars.get(i + 1) == Some(&'*') {
            push(&mut out, &mut cur, b);
            b = !b;
            i += 2;
            continue;
        }
        if c == '\\' && i + 1 < chars.len() {
            cur.push(chars[i + 1]);
            i += 2;
            continue;
        }
        if c == '`' {
            i += 1;
            continue;
        }
        if c == '*' {
            // *italic*: drop the markers between word characters only.
            let prev_space = i == 0 || chars[i - 1].is_whitespace();
            let next_space = chars.get(i + 1).is_none_or_space();
            if prev_space != next_space {
                i += 1;
                continue;
            }
        }
        if c == '<' {
            if let Some(end) = chars[i..].iter().position(|&x| x == '>') {
                let inner: String = chars[i + 1..i + end].iter().collect();
                if inner.starts_with("http") {
                    cur.push_str(&inner);
                    i += end + 1;
                    continue;
                }
            }
        }
        if c == '[' {
            if let Some(close) = chars[i..].iter().position(|&x| x == ']') {
                if chars.get(i + close + 1) == Some(&'(') {
                    if let Some(end) = chars[i + close..].iter().position(|&x| x == ')') {
                        let label: String = chars[i + 1..i + close].iter().collect();
                        cur.push_str(&label);
                        i += close + end + 1;
                        continue;
                    }
                }
            }
        }
        cur.push(c);
        i += 1;
    }
    push(&mut out, &mut cur, b);
    out
}

trait SpaceCheck {
    fn is_none_or_space(&self) -> bool;
}

impl SpaceCheck for Option<&char> {
    fn is_none_or_space(&self) -> bool {
        match self {
            None => true,
            Some(c) => c.is_whitespace() || c.is_ascii_punctuation(),
        }
    }
}

fn table_cells(line: &str) -> Vec<String> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').unwrap_or(t);
    t.split('|').map(|c| c.trim().to_string()).collect()
}

fn is_separator(line: &str) -> bool {
    let cells = table_cells(line);
    !cells.is_empty() && cells.iter().all(|c| !c.is_empty() && c.chars().all(|x| x == '-' || x == ':'))
}

/// Parses the Markdown of a report into blocks.
fn parse(md: &str) -> Vec<Block> {
    let lines: Vec<&str> = md.lines().collect();
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let t = line.trim();
        if t.is_empty() {
            i += 1;
            continue;
        }
        if t == "---" || t == "***" {
            blocks.push(Block::Rule);
            i += 1;
            continue;
        }
        if let Some(level) = (1..=4).find(|&n| t.starts_with(&format!("{} ", "#".repeat(n)))) {
            blocks.push(Block::Heading(level, spans(t[level + 1..].trim(), true)));
            i += 1;
            continue;
        }
        if t.starts_with('|') && i + 1 < lines.len() && is_separator(lines[i + 1]) {
            let align = table_cells(lines[i + 1])
                .iter()
                .map(|c| match (c.starts_with(':'), c.ends_with(':')) {
                    (true, true) => Align::Center,
                    (false, true) => Align::Right,
                    _ => Align::Left,
                })
                .collect::<Vec<_>>();
            let mut rows = vec![table_cells(t).iter().map(|c| spans(c, true)).collect::<Vec<_>>()];
            i += 2;
            while i < lines.len() && lines[i].trim().starts_with('|') {
                rows.push(table_cells(lines[i]).iter().map(|c| spans(c, false)).collect());
                i += 1;
            }
            blocks.push(Block::Table(align, rows));
            continue;
        }
        let item = if let Some(rest) = t.strip_prefix("- ").or_else(|| t.strip_prefix("* ")) {
            Some(("•".to_string(), rest))
        } else {
            let digits = t.chars().take_while(|c| c.is_ascii_digit()).count();
            (digits > 0 && t[digits..].starts_with(". ")).then(|| (t[..digits + 1].to_string(), &t[digits + 2..]))
        };
        if let Some((marker, rest)) = item {
            let mut text = rest.trim().to_string();
            i += 1;
            while i < lines.len() && lines[i].starts_with("  ") && !lines[i].trim().is_empty() {
                text.push(' ');
                text.push_str(lines[i].trim());
                i += 1;
            }
            blocks.push(Block::Item(marker, spans(&text, false)));
            continue;
        }
        // Paragraph: lines up to a blank line or another block; a line ending
        // in two spaces is a line break.
        let mut para: Vec<Vec<Span>> = Vec::new();
        let mut text = String::new();
        while i < lines.len() {
            let l = lines[i];
            let lt = l.trim();
            if lt.is_empty() || lt.starts_with('#') || lt.starts_with('|') || lt == "---" || lt.starts_with("- ") {
                break;
            }
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(lt);
            i += 1;
            if l.ends_with("  ") {
                para.push(spans(&text, false));
                text.clear();
            }
        }
        if !text.is_empty() {
            para.push(spans(&text, false));
        }
        if para.is_empty() {
            i += 1;
        } else {
            blocks.push(Block::Paragraph(para));
        }
    }
    blocks
}

/// A drawing operation on a page.
#[derive(Clone, Debug)]
enum Op {
    Text { x: f32, y: f32, size: f32, bold: bool, text: String },
    Line { x1: f32, y1: f32, x2: f32, y2: f32, width: f32, gray: f32 },
    Fill { x: f32, y: f32, w: f32, h: f32, gray: f32 },
}

struct Layout<'f> {
    regular: &'f Font,
    bold: &'f Font,
    pages: Vec<Vec<Op>>,
    y: f32,
}

/// A word with its style (spaces are implicit between words).
#[derive(Clone)]
struct Word {
    text: String,
    bold: bool,
    space_before: bool,
}

impl<'f> Layout<'f> {
    fn font(&self, bold: bool) -> &'f Font {
        if bold {
            self.bold
        } else {
            self.regular
        }
    }

    fn new_page(&mut self) {
        self.pages.push(Vec::new());
        self.y = PAGE_H - MARGIN_TOP;
    }

    fn ensure(&mut self, h: f32) {
        if self.y - h < MARGIN_BOTTOM {
            self.new_page();
        }
    }

    fn push(&mut self, op: Op) {
        self.pages.last_mut().expect("page").push(op);
    }

    fn words(spans: &[Span]) -> Vec<Word> {
        let mut out = Vec::new();
        let mut space = false;
        for s in spans {
            let mut first = true;
            for (k, part) in s.text.split(' ').enumerate() {
                if k > 0 {
                    space = true;
                }
                if part.is_empty() {
                    continue;
                }
                let attach = first && !space && !out.is_empty();
                out.push(Word { text: part.to_string(), bold: s.bold, space_before: !attach && !out.is_empty() });
                first = false;
                space = false;
            }
            if s.text.ends_with(' ') {
                space = true;
            }
        }
        out
    }

    /// Breaks the words into lines of at most `width`.
    fn lines(&self, words: &[Word], size: f32, width: f32) -> Vec<Vec<Word>> {
        let mut lines: Vec<Vec<Word>> = vec![Vec::new()];
        let mut w = 0.0;
        for word in words {
            let ww = self.font(word.bold).width(&word.text, size);
            let sp = if word.space_before { self.regular.width(" ", size) } else { 0.0 };
            if !lines.last().unwrap().is_empty() && w + sp + ww > width {
                lines.push(Vec::new());
                let mut first = word.clone();
                first.space_before = false;
                // A word longer than the line is split by characters.
                let mut rest = first.text.clone();
                while self.font(first.bold).width(&rest, size) > width && rest.chars().count() > 1 {
                    let mut cut = String::new();
                    for c in rest.chars() {
                        if self.font(first.bold).width(&format!("{cut}{c}"), size) > width {
                            break;
                        }
                        cut.push(c);
                    }
                    if cut.is_empty() {
                        break;
                    }
                    rest = rest[cut.len()..].to_string();
                    lines.last_mut().unwrap().push(Word { text: cut, bold: first.bold, space_before: false });
                    lines.push(Vec::new());
                }
                w = self.font(first.bold).width(&rest, size);
                lines.last_mut().unwrap().push(Word { text: rest, bold: first.bold, space_before: false });
                continue;
            }
            w += sp + ww;
            lines.last_mut().unwrap().push(word.clone());
        }
        lines
    }

    fn line_width(&self, line: &[Word], size: f32) -> f32 {
        line.iter()
            .map(|w| {
                self.font(w.bold).width(&w.text, size)
                    + if w.space_before { self.regular.width(" ", size) } else { 0.0 }
            })
            .sum()
    }

    fn draw_line(&mut self, line: &[Word], x: f32, y: f32, size: f32) {
        let mut cx = x;
        for w in line {
            if w.space_before {
                cx += self.regular.width(" ", size);
            }
            let ww = self.font(w.bold).width(&w.text, size);
            self.push(Op::Text { x: cx, y, size, bold: w.bold, text: w.text.clone() });
            cx += ww;
        }
    }

    fn text_block(&mut self, spans: &[Span], x: f32, width: f32, size: f32, gap_after: f32) {
        let leading = size * 1.32;
        for line in self.lines(&Self::words(spans), size, width) {
            self.ensure(leading);
            self.y -= leading;
            self.draw_line(&line, x, self.y + 0.25 * size, size);
        }
        self.y -= gap_after;
    }

    fn table(&mut self, align: &[Align], rows: &[Vec<Vec<Span>>]) {
        let ncol = rows.iter().map(Vec::len).max().unwrap_or(0);
        if ncol == 0 {
            return;
        }
        let avail = PAGE_W - 2.0 * MARGIN_X;
        let pad = 3.0;
        let size = TABLE;
        let cell_words: Vec<Vec<Vec<Word>>> =
            rows.iter().map(|r| (0..ncol).map(|c| r.get(c).map_or(Vec::new(), |s| Self::words(s))).collect()).collect();
        let natural: Vec<f32> = (0..ncol)
            .map(|c| cell_words.iter().map(|r| self.line_width(&r[c], size)).fold(0.0_f32, f32::max) + 2.0 * pad)
            .collect();
        let minimum: Vec<f32> = (0..ncol)
            .map(|c| {
                cell_words
                    .iter()
                    .flat_map(|r| r[c].iter().map(|w| self.font(w.bold).width(&w.text, size)))
                    .fold(0.0_f32, f32::max)
                    .min(avail / ncol as f32)
                    + 2.0 * pad
            })
            .collect();
        let total: f32 = natural.iter().sum();
        let widths: Vec<f32> = if total <= avail {
            natural
        } else {
            let base: f32 = minimum.iter().sum();
            let extra = (avail - base).max(0.0);
            let flex: f32 = natural.iter().zip(&minimum).map(|(n, m)| n - m).sum::<f32>().max(1e-6);
            natural.iter().zip(&minimum).map(|(n, m)| m + extra * (n - m) / flex).collect()
        };
        let leading = size * 1.28;
        let layout_row = |me: &Self, r: &[Vec<Word>]| -> Vec<Vec<Vec<Word>>> {
            (0..ncol).map(|c| me.lines(&r[c], size, widths[c] - 2.0 * pad)).collect()
        };
        let row_h =
            |cells: &[Vec<Vec<Word>>]| cells.iter().map(Vec::len).max().unwrap_or(1) as f32 * leading + 2.0 * pad;
        let header = layout_row(self, &cell_words[0]);
        let hh = row_h(&header);
        let tw: f32 = widths.iter().sum();
        let draw_row = |me: &mut Self, cells: &[Vec<Vec<Word>>], h: f32, head: bool| {
            let top = me.y;
            if head {
                me.push(Op::Fill { x: MARGIN_X, y: top - h, w: tw, h, gray: 0.92 });
            }
            let mut x = MARGIN_X;
            for (c, lines) in cells.iter().enumerate() {
                let mut y = top - pad;
                for line in lines {
                    y -= leading;
                    let lw = me.line_width(line, size);
                    let a = align.get(c).copied().unwrap_or(Align::Left);
                    let lx = match a {
                        Align::Left => x + pad,
                        Align::Right => x + widths[c] - pad - lw,
                        Align::Center => x + (widths[c] - lw) / 2.0,
                    };
                    me.draw_line(line, lx, y + 0.28 * size, size);
                }
                x += widths[c];
            }
            me.y -= h;
            me.push(Op::Line { x1: MARGIN_X, y1: me.y, x2: MARGIN_X + tw, y2: me.y, width: 0.4, gray: 0.6 });
        };
        self.ensure(hh + 3.0 * leading);
        self.push(Op::Line { x1: MARGIN_X, y1: self.y, x2: MARGIN_X + tw, y2: self.y, width: 0.6, gray: 0.4 });
        draw_row(self, &header, hh, true);
        for r in &cell_words[1..] {
            let cells = layout_row(self, r);
            let h = row_h(&cells);
            if self.y - h < MARGIN_BOTTOM {
                self.new_page();
                self.push(Op::Line { x1: MARGIN_X, y1: self.y, x2: MARGIN_X + tw, y2: self.y, width: 0.6, gray: 0.4 });
                draw_row(self, &header, hh, true);
            }
            draw_row(self, &cells, h, false);
        }
        self.y -= 8.0;
    }

    fn run(&mut self, blocks: &[Block]) {
        let width = PAGE_W - 2.0 * MARGIN_X;
        for b in blocks {
            match b {
                Block::Heading(level, s) => {
                    let size = match level {
                        1 => 15.0,
                        2 => 12.0,
                        _ => 10.5,
                    };
                    self.ensure(size * 1.4 + 3.0 * BODY * 1.32);
                    self.y -= if *level == 1 { 2.0 } else { 6.0 };
                    self.text_block(s, MARGIN_X, width, size, 4.0);
                    if *level <= 2 {
                        self.push(Op::Line {
                            x1: MARGIN_X,
                            y1: self.y + 2.0,
                            x2: PAGE_W - MARGIN_X,
                            y2: self.y + 2.0,
                            width: 0.5,
                            gray: 0.75,
                        });
                        self.y -= 3.0;
                    }
                }
                Block::Paragraph(lines) => {
                    for (k, l) in lines.iter().enumerate() {
                        let gap = if k + 1 == lines.len() { 5.0 } else { 0.0 };
                        self.text_block(l, MARGIN_X, width, BODY, gap);
                    }
                }
                Block::Item(marker, s) => {
                    let indent = 14.0;
                    let leading = BODY * 1.32;
                    let lines = self.lines(&Self::words(s), BODY, width - indent);
                    for (k, line) in lines.iter().enumerate() {
                        self.ensure(leading);
                        self.y -= leading;
                        if k == 0 {
                            let m = marker.clone();
                            self.push(Op::Text {
                                x: MARGIN_X + 2.0,
                                y: self.y + 0.25 * BODY,
                                size: BODY,
                                bold: false,
                                text: m,
                            });
                        }
                        self.draw_line(line, MARGIN_X + indent, self.y + 0.25 * BODY, BODY);
                    }
                    self.y -= 1.5;
                }
                Block::Table(align, rows) => {
                    self.y -= 2.0;
                    self.table(align, rows);
                }
                Block::Rule => {
                    self.ensure(10.0);
                    self.y -= 5.0;
                    self.push(Op::Line {
                        x1: MARGIN_X,
                        y1: self.y,
                        x2: PAGE_W - MARGIN_X,
                        y2: self.y,
                        width: 0.5,
                        gray: 0.6,
                    });
                    self.y -= 5.0;
                }
            }
        }
    }
}

/// Renders a Markdown report to PDF. `title` is the document title (Info
/// dictionary and page footer), `producer` the software identity.
pub fn from_markdown(md: &str, title: &str, producer: &str, lang: crate::Lang) -> Vec<u8> {
    let mut regular = Font::new(REGULAR, "DejaVuSans");
    let mut bold = Font::new(BOLD, "DejaVuSans-Bold");
    let blocks = parse(md);
    let pages = {
        let mut l = Layout { regular: &regular, bold: &bold, pages: Vec::new(), y: 0.0 };
        l.new_page();
        l.run(&blocks);
        l.pages
    };
    let n = pages.len();
    let footer_title: String = if title.chars().count() > 80 {
        format!("{}…", title.chars().take(79).collect::<String>())
    } else {
        title.to_string()
    };

    let mut pdf = Pdf::new();
    let mut next = 1;
    let mut alloc = || {
        let r = Ref::new(next);
        next += 1;
        r
    };
    let catalog = alloc();
    let tree = alloc();
    let info = alloc();
    let fonts: Vec<[Ref; 5]> = (0..2).map(|_| [alloc(), alloc(), alloc(), alloc(), alloc()]).collect();
    let page_refs: Vec<(Ref, Ref)> = (0..n).map(|_| (alloc(), alloc())).collect();
    pdf.catalog(catalog).pages(tree);
    pdf.pages(tree).kids(page_refs.iter().map(|p| p.0)).count(n as i32);
    pdf.document_info(info).title(TextStr(title)).producer(TextStr(producer)).creator(TextStr(producer));

    for (k, ops) in pages.iter().enumerate() {
        let mut c = Content::new();
        let text = |c: &mut Content, f: &mut Font, name: &[u8], x: f32, y: f32, size: f32, s: &str| {
            let bytes = f.encode(s);
            c.begin_text();
            c.set_font(Name(name), size);
            c.next_line(x, y);
            c.show(Str(&bytes));
            c.end_text();
        };
        for op in ops {
            match op {
                Op::Text { x, y, size, bold: b, text: s } => {
                    if *b {
                        text(&mut c, &mut bold, b"F2", *x, *y, *size, s);
                    } else {
                        text(&mut c, &mut regular, b"F1", *x, *y, *size, s);
                    }
                }
                Op::Line { x1, y1, x2, y2, width, gray } => {
                    c.set_stroke_gray(*gray);
                    c.set_line_width(*width);
                    c.move_to(*x1, *y1);
                    c.line_to(*x2, *y2);
                    c.stroke();
                }
                Op::Fill { x, y, w, h, gray } => {
                    c.set_fill_gray(*gray);
                    c.rect(*x, *y, *w, *h);
                    c.fill_nonzero();
                    c.set_fill_gray(0.0);
                }
            }
        }
        // Footer: title and page number.
        let page_label = match lang {
            crate::Lang::En => format!("Page {} of {n}", k + 1),
            crate::Lang::Tr => format!("Sayfa {} / {n}", k + 1),
        };
        c.set_fill_gray(0.35);
        let fy = MARGIN_BOTTOM - 28.0;
        text(&mut c, &mut regular, b"F1", MARGIN_X, fy, 7.0, &footer_title);
        let pw = regular.width(&page_label, 7.0);
        text(&mut c, &mut regular, b"F1", PAGE_W - MARGIN_X - pw, fy, 7.0, &page_label);
        c.set_fill_gray(0.0);
        let data = miniz_oxide::deflate::compress_to_vec_zlib(&c.finish(), 6);
        let (page, content) = page_refs[k];
        let mut p = pdf.page(page);
        p.media_box(Rect::new(0.0, 0.0, PAGE_W, PAGE_H)).parent(tree).contents(content);
        let mut res = p.resources();
        let mut fd = res.fonts();
        fd.pair(Name(b"F1"), fonts[0][0]);
        fd.pair(Name(b"F2"), fonts[1][0]);
        fd.finish();
        res.finish();
        p.finish();
        pdf.stream(content, &data).filter(Filter::FlateDecode);
    }

    for (font, refs) in [(&regular, fonts[0]), (&bold, fonts[1])] {
        let [type0, cid, desc, file, cmap] = refs;
        let name = Name(font.name.as_bytes());
        let sys = SystemInfo { registry: Str(b"Adobe"), ordering: Str(b"Identity"), supplement: 0 };
        pdf.type0_font(type0)
            .base_font(name)
            .encoding_predefined(Name(b"Identity-H"))
            .descendant_font(cid)
            .to_unicode(cmap);
        let mut cf = pdf.cid_font(cid);
        cf.subtype(CidFontType::Type2).base_font(name).system_info(sys).font_descriptor(desc);
        cf.cid_to_gid_map_predefined(Name(b"Identity")).default_width(0.0);
        let mut w = cf.widths();
        for &g in font.used.keys() {
            w.consecutive(g, [font.advance(g) * 1000.0]);
        }
        w.finish();
        cf.finish();
        let s = 1000.0 / font.upem;
        let bb = font.face.global_bounding_box();
        pdf.font_descriptor(desc)
            .name(name)
            .flags(FontFlags::NON_SYMBOLIC)
            .bbox(Rect::new(bb.x_min as f32 * s, bb.y_min as f32 * s, bb.x_max as f32 * s, bb.y_max as f32 * s))
            .italic_angle(0.0)
            .ascent(font.face.ascender() as f32 * s)
            .descent(font.face.descender() as f32 * s)
            .cap_height(font.face.capital_height().unwrap_or(font.face.ascender()) as f32 * s)
            .stem_v(if font.name.ends_with("Bold") { 140.0 } else { 80.0 })
            .font_file2(file);
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(font.data, 6);
        pdf.stream(file, &compressed).filter(Filter::FlateDecode).pair(Name(b"Length1"), font.data.len() as i32);
        let mut map = UnicodeCmap::new(Name(b"Custom"), sys);
        for (&g, &ch) in &font.used {
            map.pair(g, ch);
        }
        pdf.cmap(cmap, &map.finish());
    }
    pdf.finish()
}

/// Characters of `text` that the embedded fonts cannot show.
pub fn missing_glyphs(text: &str) -> Vec<char> {
    let f = Font::new(REGULAR, "DejaVuSans");
    let mut out: Vec<char> = text.chars().filter(|c| !c.is_control() && f.face.glyph_index(*c).is_none()).collect();
    out.sort_unstable();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_report_markdown() {
        let md = "# Title\n\n*Report*\n\nA **bold** word and `code`.  \nNext line.\n\n- item one\n  continued\n1. first\n\n| A | B |\n|---|---:|\n| x | 1,5 |\n\n---\n";
        let b = parse(md);
        assert_eq!(b[0], Block::Heading(1, vec![Span { text: "Title".into(), bold: true }]));
        assert!(matches!(&b[2], Block::Paragraph(l) if l.len() == 2));
        assert!(matches!(&b[3], Block::Item(m, s) if m == "•" && s[0].text == "item one continued"));
        assert!(matches!(&b[4], Block::Item(m, _) if m == "1."));
        assert!(matches!(&b[5], Block::Table(a, r) if a == &vec![Align::Left, Align::Right] && r.len() == 2));
        assert_eq!(b[6], Block::Rule);
        let s = spans("a **b** c", false);
        assert_eq!(s.len(), 3);
        assert!(s[1].bold && !s[2].bold);
    }

    #[test]
    fn writes_a_deterministic_pdf_with_turkish_text() {
        let md = "# Ölçüm belirsizliği\n\nGenişletilmiş belirsizlik U = 0,0015 g/g; ğüşıöç İ μ σ ν √ ± ≤ ².\n";
        let a = from_markdown(md, "Deneme", "test", crate::Lang::Tr);
        let b = from_markdown(md, "Deneme", "test", crate::Lang::Tr);
        assert_eq!(a, b);
        assert!(a.starts_with(b"%PDF-1.7"));
        assert!(a.len() > 100_000 && a.len() < 1_500_000, "{}", a.len());
        assert!(missing_glyphs(md).is_empty(), "{:?}", missing_glyphs(md));
    }

    #[test]
    fn long_tables_break_across_pages_with_the_header_repeated() {
        let mut md = String::from("| Name | Value |\n|---|---:|\n");
        for i in 0..200 {
            md.push_str(&format!("| row {i} | {i}.5 |\n"));
        }
        let blocks = parse(&md);
        let regular = Font::new(REGULAR, "DejaVuSans");
        let bold = Font::new(BOLD, "DejaVuSans-Bold");
        let mut l = Layout { regular: &regular, bold: &bold, pages: Vec::new(), y: 0.0 };
        l.new_page();
        l.run(&blocks);
        assert!(l.pages.len() >= 3);
        for p in &l.pages {
            assert!(p.iter().any(|op| matches!(op, Op::Text { text, bold: true, .. } if text == "Name")));
        }
    }
}
