//! Verification workbook of an interlaboratory precision experiment
//! [ISO 5725-2:2019]: Forms B and C (cell means and standard deviations, 8.2),
//! Mandel's h and k (7.3.1), the first Cochran test (7.3.3), Grubbs' single
//! tests (7.3.4) and, for the analysis of variance, the general mean and the
//! repeatability and reproducibility standard deviations (Formulae 23–31).
//! The restricted maximum likelihood estimates (Annex B.2), Grubbs' double
//! tests and the functional relationships (8.5) are not recomputed.

use super::{info_sheet, t, Checks, Diff};
use crate::interlab::{InterlabAnalysis, MethodSpec};
use crate::schema::Project;
use crate::xlsx::{cell_ref, sheet_ref, Sheet, Style, Workbook};
use crate::{Lang, ProjectError};

/// A sheet name of at most 31 characters without the characters Excel forbids.
fn sheet_name(prefix: &str, name: &str) -> String {
    let clean: String =
        format!("{prefix} {name}").chars().map(|c| if "[]:*?/\\'".contains(c) { '_' } else { c }).collect();
    clean.chars().take(31).collect()
}

pub(super) fn workbook(p: &Project, a: &InterlabAnalysis, lang: Lang) -> Result<Workbook, ProjectError> {
    let spec = p
        .precision_experiment
        .as_ref()
        .ok_or_else(|| ProjectError::Format("no precision_experiment section".into()))?;
    let mut ch = Checks::default();
    let mut sheets = Vec::new();
    let labs = &spec.laboratories;
    let np = labs.len() as u32;
    let round = |f: String| match spec.form_decimals {
        Some(d) => format!("ROUND({f},{d})"),
        None => f,
    };
    // Which results of a cell are left out: None for the whole cell.
    let dropped = |level: &str, lab: &str, k: usize| {
        spec.exclude.iter().any(|x| {
            x.laboratory == lab
                && x.level.as_deref().map_or(true, |l| l == level)
                && x.result.map_or(true, |r| r == k + 1)
        })
    };
    for (lj, level) in spec.levels.iter().enumerate() {
        let res = &a.levels[lj];
        let name = sheet_name(t(lang, "Level", "Düzey"), &level.name);
        let mut sh = Sheet::new(&name);
        sh.text(0, 0, format!("{} — {}", spec.quantity, level.name), Style::Title);
        let nmax = level.results.iter().map(Vec::len).max().unwrap_or(1).max(1) as u32;
        // Columns after the results.
        let (c_n, c_m, c_s) = (1 + nmax, 2 + nmax, 3 + nmax);
        let (c_h, c_k, c_dev) = (4 + nmax, 5 + nmax, 6 + nmax);
        let (c_ns, c_nm, c_nm2) = (7 + nmax, 8 + nmax, 9 + nmax);
        let block = |sh: &mut Sheet, top: u32, title: &str, keep: bool| -> (u32, u32) {
            sh.text(top, 0, title.to_string(), Style::Bold);
            let hr = top + 1;
            sh.text(hr, 0, t(lang, "Laboratory", "Laboratuvar"), Style::Heading);
            for k in 0..nmax {
                sh.text(hr, 1 + k, format!("y{}", k + 1), Style::Heading);
            }
            sh.text(hr, c_n, "n", Style::Heading);
            sh.text(hr, c_m, t(lang, "ȳ (Form B)", "ȳ (Form B)"), Style::Heading);
            sh.text(hr, c_s, t(lang, "s (Form C)", "s (Form C)"), Style::Heading);
            let r0 = hr + 1;
            for (i, lab) in labs.iter().enumerate() {
                let r = r0 + i as u32;
                sh.text(r, 0, lab.clone(), Style::Bold);
                for (k, y) in level.results[i].iter().enumerate() {
                    if !keep || !dropped(&level.name, lab, k) {
                        sh.number(r, 1 + k as u32, *y, Style::Normal);
                    }
                }
                let ys = format!("{}:{}", cell_ref(r, 1), cell_ref(r, nmax));
                sh.formula(r, c_n, format!("COUNT({ys})"), Style::Normal);
                sh.formula(
                    r,
                    c_m,
                    format!("IF({n}=0,\"\",{})", round(format!("AVERAGE({ys})")), n = cell_ref(r, c_n)),
                    Style::Number,
                );
                sh.formula(
                    r,
                    c_s,
                    format!("IF({n}<2,\"\",{})", round(format!("STDEV({ys})")), n = cell_ref(r, c_n)),
                    Style::Number,
                );
            }
            (r0, r0 + np - 1)
        };
        let col = |c: u32, r0: u32, r1: u32| format!("{}:{}", cell_ref(r0, c), cell_ref(r1, c));

        // --- All results: scrutiny (Mandel h and k, Cochran, Grubbs) -----------------
        let (a0, a1) = block(&mut sh, 2, t(lang, "All results (7.3)", "Bütün sonuçlar (7.3)"), false);
        let hr = a0 - 1;
        sh.text(hr, c_h, "h (6)", Style::Heading);
        sh.text(hr, c_k, "k (7, 8)", Style::Heading);
        sh.text(hr, c_dev, "(ȳ − m̂)²", Style::Heading);
        let (ns, ms, ss) = (col(c_n, a0, a1), col(c_m, a0, a1), col(c_s, a0, a1));
        let sa = a1 + 2;
        let lines: Vec<(String, String)> = vec![
            ("p".into(), format!("COUNT({ms})")),
            ("m̂ = Σ nȳ / Σ n".into(), format!("SUMPRODUCT({ns},{ms})/SUM({ns})")),
            ("√(Σ(ȳ − m̂)² / (p − 1))".into(), format!("SQRT(SUM({})/({}-1))", col(c_dev, a0, a1), cell_ref(sa, 1))),
            (
                t(lang, "Cochran C = s²_max / Σ s² (9)", "Cochran C = s²_max / Σ s² (9)").into(),
                format!("MAX({ss})^2/SUMSQ({ss})"),
            ),
            (
                t(lang, "Grubbs, lowest (11)", "Grubbs, en küçük (11)").into(),
                format!("(AVERAGE({ms})-MIN({ms}))/STDEV({ms})"),
            ),
            (
                t(lang, "Grubbs, highest (12)", "Grubbs, en büyük (12)").into(),
                format!("(MAX({ms})-AVERAGE({ms}))/STDEV({ms})"),
            ),
        ];
        for (k, (label, f)) in lines.iter().enumerate() {
            sh.text(sa + k as u32, 0, label.clone(), Style::Bold);
            sh.formula(sa + k as u32, 1, f.clone(), Style::Number);
        }
        let (mw, sh_) = (cell_ref(sa + 1, 1), cell_ref(sa + 2, 1));
        for i in 0..np {
            let r = a0 + i;
            let (n, m, s) = (cell_ref(r, c_n), cell_ref(r, c_m), cell_ref(r, c_s));
            sh.formula(r, c_dev, format!("IF({n}=0,\"\",({m}-{mw})^2)"), Style::Number);
            sh.formula(r, c_h, format!("IF({n}=0,\"\",IF({sh_}=0,0,({m}-{mw})/{sh_}))"), Style::Number);
            sh.formula(
                r,
                c_k,
                format!("IF(ISNUMBER({s}),IF(SUMSQ({ss})=0,0,{s}*SQRT(COUNT({ss}))/SQRT(SUMSQ({ss}))),\"\")"),
                Style::Number,
            );
        }
        let at = |r: u32, c: u32| sheet_ref(&name, r, c);
        let lv = &level.name;
        for (idx, &lab) in res.initial.labs.iter().enumerate() {
            let r = a0 + lab as u32;
            ch.add(format!("h ({lv}, {})", labs[lab]), res.initial.h[idx], at(r, c_h), Diff::Absolute, 1e-9);
            if let Some(k) = res.initial.k[idx] {
                ch.add(format!("k ({lv}, {})", labs[lab]), k, at(r, c_k), Diff::Absolute, 1e-9);
            }
        }
        if let Some(c) = res.initial.cochran.first() {
            ch.add(format!("Cochran C ({lv})"), c.statistic, at(sa + 3, 1), Diff::Relative, 1e-9);
        }
        if let Some(g) = &res.initial.grubbs {
            ch.add(
                format!("Grubbs, {} ({lv})", t(lang, "lowest", "en küçük")),
                g.low.statistic,
                at(sa + 4, 1),
                Diff::Absolute,
                1e-9,
            );
            ch.add(
                format!("Grubbs, {} ({lv})", t(lang, "highest", "en büyük")),
                g.high.statistic,
                at(sa + 5, 1),
                Diff::Absolute,
                1e-9,
            );
        }

        // --- Retained results: general mean and precision (8.4) ----------------------
        if spec.method == MethodSpec::Anova {
            let top = sa + lines.len() as u32 + 1;
            let (b0, b1) = block(
                &mut sh,
                top,
                t(
                    lang,
                    "Retained results: analysis of variance (8.4.4, 8.4.5)",
                    "Kalan sonuçlar: varyans analizi (8.4.4, 8.4.5)",
                ),
                true,
            );
            let hr = b0 - 1;
            sh.text(hr, c_ns, "(n − 1) s²", Style::Heading);
            sh.text(hr, c_nm, "n − 1", Style::Heading);
            sh.text(hr, c_nm2, "n (ȳ − m̂)²", Style::Heading);
            let est = b1 + 2;
            for i in 0..np {
                let r = b0 + i;
                let (n, m, s) = (cell_ref(r, c_n), cell_ref(r, c_m), cell_ref(r, c_s));
                sh.formula(r, c_ns, format!("IF(ISNUMBER({s}),({n}-1)*{s}^2,0)"), Style::Number);
                sh.formula(r, c_nm, format!("IF(ISNUMBER({s}),{n}-1,0)"), Style::Normal);
                sh.formula(r, c_nm2, format!("IF({n}=0,0,{n}*({m}-{})^2)", cell_ref(est + 2, 1)), Style::Number);
            }
            let (ns, ms) = (col(c_n, b0, b1), col(c_m, b0, b1));
            let e: Vec<(String, String)> = vec![
                ("p".into(), format!("COUNT({ms})")),
                ("Σ n".into(), format!("SUM({ns})")),
                ("m̂ (23)".into(), format!("SUMPRODUCT({ns},{ms})/{}", cell_ref(est + 1, 1))),
                ("s_r² (24)".into(), format!("SUM({})/SUM({})", col(c_ns, b0, b1), col(c_nm, b0, b1))),
                ("s_d² (27)".into(), format!("SUM({})/({}-1)", col(c_nm2, b0, b1), cell_ref(est, 1))),
                (
                    "n̄ (28)".into(),
                    format!("({N}-SUMSQ({ns})/{N})/({p}-1)", N = cell_ref(est + 1, 1), p = cell_ref(est, 1)),
                ),
                (
                    "s_L² (29)".into(),
                    format!("MAX(0,({}-{})/{})", cell_ref(est + 4, 1), cell_ref(est + 3, 1), cell_ref(est + 5, 1)),
                ),
                ("s_r".into(), format!("SQRT({})", cell_ref(est + 3, 1))),
                ("s_L".into(), format!("SQRT({})", cell_ref(est + 6, 1))),
                ("s_R (31)".into(), format!("SQRT({}+{})", cell_ref(est + 3, 1), cell_ref(est + 6, 1))),
            ];
            for (k, (label, f)) in e.iter().enumerate() {
                sh.text(est + k as u32, 0, label.clone(), Style::Bold);
                sh.formula(est + k as u32, 1, f.clone(), Style::Number);
            }
            let x = &res.estimates;
            ch.add(format!("m̂ ({lv})"), x.mean, at(est + 2, 1), Diff::Relative, 1e-12);
            ch.add(format!("s_r ({lv})"), x.s_r, at(est + 7, 1), Diff::Relative, 1e-9);
            ch.add(
                format!("s_L ({lv})"),
                x.s_l,
                at(est + 8, 1),
                if x.s_l == 0.0 { Diff::Absolute } else { Diff::Relative },
                1e-9,
            );
            ch.add(format!("s_R ({lv})"), x.s_rr, at(est + 9, 1), Diff::Relative, 1e-9);
        }
        sh.width(0, 30.0);
        for c in 1..=c_nm2 {
            sh.width(c, 13.0);
        }
        sheets.push(sh);
    }
    let mut sc = Sheet::new(t(lang, "Check", "Karşılaştırma"));
    let fails = ch.write(&mut sc, lang);
    let mut how = vec![
        t(lang,
          "One sheet per level. All results: the cell means and standard deviations (Forms B and C, rounded as recorded when the project gives form_decimals), Mandel's h and k (Formulae 6–8), the first Cochran test (Formula 9) and Grubbs' single tests (Formulae 11, 12).",
          "Her düzey bir sayfada. Bütün sonuçlar: hücre ortalamaları ve standart sapmaları (Form B ve C; projede form_decimals verilmişse kaydedildiği gibi yuvarlanmış), Mandel h ve k (Formül 6–8), ilk Cochran testi (Formül 9) ve Grubbs tekli testleri (Formül 11, 12)."),
    ];
    how.push(if spec.method == MethodSpec::Anova {
        t(lang,
          "Retained results: the results left out after scrutiny are removed (exclude in the project file); general mean and repeatability, between-laboratory and reproducibility standard deviations by Formulae 23–31.",
          "Kalan sonuçlar: incelemeden sonra çıkarılan sonuçlar alınmaz (proje dosyasındaki exclude); genel ortalama ile tekrarlanabilirlik, laboratuvarlar arası ve tekrar üretilebilirlik standart sapmaları Formül 23–31 ile.")
    } else {
        t(lang,
          "The project uses restricted maximum likelihood (Annex B.2), which needs an iterative optimisation: its estimates are not recomputed here.",
          "Proje kısıtlı en çok olabilirlik (Ek B.2) kullanıyor; bu yinelemeli bir eniyileme gerektirdiğinden kestirimleri burada yeniden hesaplanmaz.")
    });
    how.push(t(lang,
        "Not recomputed: repeated Cochran tests, Grubbs' double tests, the critical values (Tables 4–8) and the functional relationships between s and m (8.5).",
        "Yeniden hesaplanmayanlar: tekrarlanan Cochran testleri, Grubbs çiftli testleri, kritik değerler (Çizelge 4–8) ve s ile m arasındaki fonksiyonel ilişkiler (8.5)."));
    let info = info_sheet(p, lang, &fails, &how);
    let mut all = vec![info, sc];
    all.extend(sheets);
    Ok(Workbook { sheets: all })
}
