//! Reports of interlaboratory precision experiments (ISO 5725-2:2019).

use super::*;
use crate::interlab::{InterlabAnalysis, LevelResult, MethodSpec, RelationshipSpec, Scrutiny};
use uncertainty_core::interlab::{Outcome, Relationship};

pub(super) struct IlLabels {
    pub title: &'static str,
    pub quantity: &'static str,
    pub design: &'static str,
    pub method_anova: &'static str,
    pub method_reml: &'static str,
    pub laboratory: &'static str,
    pub level: &'static str,
    pub form_b: &'static str,
    pub form_c: &'static str,
    pub legend: &'static str,
    pub mandel: &'static str,
    pub mandel_text: &'static str,
    pub indicators: &'static str,
    pub cochran: &'static str,
    pub cochran_text: &'static str,
    pub grubbs: &'static str,
    pub grubbs_text: &'static str,
    pub single_low: &'static str,
    pub single_high: &'static str,
    pub repeated: &'static str,
    pub double_low: &'static str,
    pub double_high: &'static str,
    pub statistic: &'static str,
    pub critical: &'static str,
    pub outcome: &'static str,
    pub outcomes: [&'static str; 3],
    pub not_applied: &'static str,
    pub exclusions: &'static str,
    pub whole_lab: &'static str,
    pub cell: &'static str,
    pub result: &'static str,
    pub retained: &'static str,
    pub retained_outliers: &'static str,
    pub precision: &'static str,
    pub a_note: &'static str,
    pub relationships: &'static str,
    pub relationships_text: &'static str,
    pub relationship: &'static str,
    pub observed: &'static str,
    pub not_fitted: &'static str,
    pub repeatability: &'static str,
    pub reproducibility: &'static str,
    pub h_chart: &'static str,
    pub k_chart: &'static str,
    pub s_chart: &'static str,
}

const IL_EN: IlLabels = IlLabels {
    title: "Interlaboratory precision experiment (ISO 5725-2, basic method)",
    quantity: "Property",
    design: "{p} laboratories, {q} levels.",
    method_anova: "General mean and variances by Formulae (23) to (31) of ISO 5725-2:2019 (8.4); s_L is set to zero when its estimate is negative (8.4.5.4).",
    method_reml: "General mean and variances by restricted maximum likelihood (ISO 5725-2:2019, 8.4.6.2 and Annex B.2); the standard error of the mean by Formula (B.7).",
    laboratory: "Laboratory",
    level: "Level",
    form_b: "Cell means (Form B)",
    form_c: "Cell standard deviations (Form C)",
    legend: "\\* straggler, \\*\\* statistical outlier (Grubbs' test in Form B, Cochran's test in Form C; 8.3.3.1); values in brackets were left out; — empty cell.",
    mandel: "Mandel's consistency statistics h and k (8.3.2)",
    mandel_text: "All results, before exclusions. h compares the cell mean with the general mean; k compares the cell standard deviation with the pooled one.",
    indicators: "Indicators (Tables 7 and 8, Formulae D.5 and D.6): 5 % / 1 %",
    cochran: "Cochran's test of the cell standard deviations (8.3.4)",
    cochran_text: "C = s²max / Σs²; critical values for the number of results in the majority of cells (8.3.4.3); repeated without the highest value while it is an outlier (8.3.4.6).",
    grubbs: "Grubbs' tests of the cell means (8.3.5)",
    grubbs_text: "Two-sided critical values of Table 6 (Formula D.2; D.3 beyond the table). If a single test finds an outlier, the other extreme is tested without it and the double tests are not applied (8.3.5.3).",
    single_low: "single low",
    single_high: "single high",
    repeated: "other extreme, repeated",
    double_low: "double low",
    double_high: "double high",
    statistic: "Statistic",
    critical: "Critical 5 % / 1 %",
    outcome: "Outcome",
    outcomes: ["correct", "straggler *", "outlier **"],
    not_applied: "not applied",
    exclusions: "Results left out",
    whole_lab: "all levels",
    cell: "cell",
    result: "result",
    retained: "Scrutiny of the retained results",
    retained_outliers: "Statistical outliers remain among the retained results. ISO 5725-2 8.3.3.1 c) discards them unless the statistician, for good reason, decides to retain them; state that reason in the exclusion list or the notes.",
    precision: "Repeatability and reproducibility",
    a_note: "A_r, A_R: relative amount by which s_r and s_R can be expected to differ from the true values with a probability of about 95 % (Annex A, Formulae A.3 and A.4, with γ = s_R/s_r); for planning, not confidence limits.",
    relationships: "Dependence of the precision on the level (8.5)",
    relationships_text: "Relationships I to IV fitted to the standard deviations of all levels: I and IV in closed form or by unweighted regression of lg s on lg m, II and III by weighted regression iterated once (8.5.2 to 8.5.4). Choose the relationship from the plot and the purpose of the method (8.5.1).",
    relationship: "Relationship",
    observed: "observed",
    not_fitted: "not fitted",
    repeatability: "Repeatability s_r",
    reproducibility: "Reproducibility s_R",
    h_chart: "Mandel's h by laboratory (bars: levels), indicators at 5 % (dotted) and 1 % (dashed)",
    k_chart: "Mandel's k by laboratory (bars: levels), indicators at 5 % (dotted) and 1 % (dashed)",
    s_chart: "s_r (●) and s_R (■) against the general mean, with the fitted relationships",
};

const IL_TR: IlLabels = IlLabels {
    title: "Laboratuvarlar arası kesinlik deneyi (ISO 5725-2, temel yöntem)",
    quantity: "Özellik",
    design: "{p} laboratuvar, {q} düzey.",
    method_anova: "Genel ortalama ve varyanslar ISO 5725-2:2019 Formül (23)–(31) ile (8.4); tahmini negatif çıkan s_L sıfır alınır (8.4.5.4).",
    method_reml: "Genel ortalama ve varyanslar kısıtlı en çok olabilirlik (REML) ile (ISO 5725-2:2019, 8.4.6.2 ve Ek B.2); ortalamanın standart hatası Formül (B.7) ile.",
    laboratory: "Laboratuvar",
    level: "Düzey",
    form_b: "Hücre ortalamaları (Form B)",
    form_c: "Hücre standart sapmaları (Form C)",
    legend: "\\* şüpheli değer (straggler), \\*\\* istatistiksel aykırı değer (Form B'de Grubbs, Form C'de Cochran testi; 8.3.3.1); köşeli parantez içindeki değerler dışarıda bırakılmıştır; — boş hücre.",
    mandel: "Mandel tutarlılık istatistikleri h ve k (8.3.2)",
    mandel_text: "Tüm sonuçlar, dışarıda bırakmalardan önce. h hücre ortalamasını genel ortalamayla, k hücre standart sapmasını birleştirilmiş standart sapmayla karşılaştırır.",
    indicators: "Gösterge değerleri (Tablo 7 ve 8, Formül D.5 ve D.6): %5 / %1",
    cochran: "Hücre standart sapmalarına Cochran testi (8.3.4)",
    cochran_text: "C = s²maks / Σs²; kritik değerler hücrelerin çoğunluğundaki sonuç sayısı için (8.3.4.3); en yüksek değer aykırı olduğu sürece o değer çıkarılarak tekrarlanır (8.3.4.6).",
    grubbs: "Hücre ortalamalarına Grubbs testleri (8.3.5)",
    grubbs_text: "Tablo 6'nın iki taraflı kritik değerleri (Formül D.2; tablo dışında D.3). Tekli test aykırı değer bulursa diğer uç bu değer çıkarılarak test edilir ve ikili testler uygulanmaz (8.3.5.3).",
    single_low: "tekli, en düşük",
    single_high: "tekli, en yüksek",
    repeated: "diğer uç, tekrar",
    double_low: "ikili, en düşük",
    double_high: "ikili, en yüksek",
    statistic: "İstatistik",
    critical: "Kritik %5 / %1",
    outcome: "Sonuç",
    outcomes: ["doğru", "şüpheli *", "aykırı **"],
    not_applied: "uygulanmadı",
    exclusions: "Dışarıda bırakılan sonuçlar",
    whole_lab: "tüm düzeyler",
    cell: "hücre",
    result: "sonuç",
    retained: "Kalan sonuçların incelenmesi",
    retained_outliers: "Kalan sonuçlar arasında istatistiksel aykırı değerler var. ISO 5725-2 8.3.3.1 c) istatistikçi geçerli bir nedenle tutmaya karar vermedikçe bunların atılmasını öngörür; bu nedeni dışarıda bırakma listesinde veya notlarda belirtin.",
    precision: "Tekrarlanabilirlik ve tekrarüretilebilirlik",
    a_note: "A_r, A_R: s_r ve s_R'nin gerçek değerlerden yaklaşık %95 olasılıkla ne kadar farklı olabileceği (göreli; Ek A, Formül A.3 ve A.4, γ = s_R/s_r); planlama içindir, güven sınırı değildir.",
    relationships: "Kesinliğin düzeye bağlılığı (8.5)",
    relationships_text: "Tüm düzeylerin standart sapmalarına I–IV ilişkileri uydurulmuştur: I kapalı biçimde, IV lg s'nin lg m'ye ağırlıksız regresyonuyla, II ve III bir kez yinelenen ağırlıklı regresyonla (8.5.2–8.5.4). İlişkiyi grafiğe ve yöntemin amacına göre seçin (8.5.1).",
    relationship: "İlişki",
    observed: "gözlenen",
    not_fitted: "uydurulamadı",
    repeatability: "Tekrarlanabilirlik s_r",
    reproducibility: "Tekrarüretilebilirlik s_R",
    h_chart: "Laboratuvara göre Mandel h (çubuklar: düzeyler), %5 (noktalı) ve %1 (kesikli) göstergeleri",
    k_chart: "Laboratuvara göre Mandel k (çubuklar: düzeyler), %5 (noktalı) ve %1 (kesikli) göstergeleri",
    s_chart: "Genel ortalamaya karşı s_r (●) ve s_R (■), uydurulan ilişkilerle",
};

pub(super) fn il_labels(lang: Lang) -> &'static IlLabels {
    match lang {
        Lang::En => &IL_EN,
        Lang::Tr => &IL_TR,
    }
}

fn fixed(x: f64, k: usize, lang: Lang) -> String {
    let t = format!("{x:.k$}");
    if lang == Lang::Tr {
        t.replace('.', ",")
    } else {
        t
    }
}

fn outcome_text(o: Outcome, d: &IlLabels) -> &'static str {
    match o {
        Outcome::Correct => d.outcomes[0],
        Outcome::Straggler => d.outcomes[1],
        Outcome::Outlier => d.outcomes[2],
    }
}

fn mark(o: Outcome) -> &'static str {
    match o {
        Outcome::Correct => "",
        Outcome::Straggler => "\\*",
        Outcome::Outlier => "\\*\\*",
    }
}

fn unit_suffix(a: &InterlabAnalysis) -> String {
    a.unit.clone().map(|u| format!(" {u}")).unwrap_or_default()
}

/// One statement per level: the general mean with s_r and s_R.
pub(super) fn statements(a: &InterlabAnalysis, lang: Lang) -> Vec<String> {
    let unit = unit_suffix(a);
    let d = il_labels(lang);
    a.levels
        .iter()
        .map(|l| {
            let e = &l.estimates;
            format!(
                "{} {}: m̂ = {}{unit}, s_r = {}{unit}, s_R = {}{unit} (p = {})",
                d.level,
                l.name,
                fmt_num(e.mean, 5, lang),
                fmt_num(e.s_r, 4, lang),
                fmt_num(e.s_rr, 4, lang),
                e.p
            )
        })
        .collect()
}

fn relationship_formula(r: &Relationship, lang: Lang) -> String {
    let n = |x: f64| fmt_num(x, 4, lang);
    match *r {
        Relationship::Proportional { b } => format!("s = {}·m", n(b)),
        Relationship::Linear { a, b } => format!("s = {} + {}·m", n(a), n(b)),
        Relationship::Quadratic { a2, b2 } => {
            if a2 >= 0.0 {
                format!("s² = {}² + ({}·m)²", n(a2.sqrt()), n(b2.max(0.0).sqrt()))
            } else {
                format!("s² = {} + ({}·m)² (a_v² < 0)", n(a2), n(b2.max(0.0).sqrt()))
            }
        }
        Relationship::Power { c, d } => {
            format!("lg s = {} + {}·lg m (s = {}·m^{})", n(c), n(d), n(10f64.powf(c)), n(d))
        }
    }
}

/// Decimal places of the results as given, plus one (8.2.10, 8.2.11).
fn form_decimals(p: &Project) -> usize {
    let Some(spec) = p.precision_experiment.as_ref() else { return 4 };
    if let Some(d) = spec.form_decimals {
        return d as usize;
    }
    let given = spec
        .levels
        .iter()
        .flat_map(|l| l.results.iter().flatten())
        .map(|y| format!("{y}").split_once('.').map_or(0, |(_, f)| f.len()))
        .max()
        .unwrap_or(0);
    (given + 1).min(8)
}

/// The forms with their marks: Form B (means) when `means`, else Form C.
fn form_table(s: &mut String, a: &InterlabAnalysis, means: bool, lang: Lang, decimals: usize) {
    let d = il_labels(lang);
    let mut head = format!("| {} |", d.laboratory);
    let mut rule = "|---|".to_string();
    for l in &a.levels {
        head += &format!(" {} |", l.name);
        rule += "---:|";
    }
    let _ = writeln!(s, "{head}\n{rule}");
    let num = |x: f64| fixed(x, decimals, lang);
    for (i, lab) in a.laboratories.iter().enumerate() {
        let mut row = format!("| {} |", lab);
        for l in &a.levels {
            let (c_out, g_out) = l.initial.outcomes(i);
            let value = l.cells[i].and_then(|c| if means { Some(c.mean) } else { c.sd });
            let text = match value {
                None => "—".to_string(),
                Some(v) => {
                    let m = if means { mark(g_out) } else { mark(c_out) };
                    let changed = l.excluded.contains(&i);
                    let shown = format!("{}{m}", num(v));
                    if changed {
                        format!("[{shown}]")
                    } else {
                        shown
                    }
                }
            };
            row += &format!(" {text} |");
        }
        let _ = writeln!(s, "{row}");
    }
    let _ = writeln!(s);
}

fn cochran_rows(s: &mut String, level: &LevelResult, sc: &Scrutiny, labs: &[String], lang: Lang) {
    let d = il_labels(lang);
    if sc.cochran.is_empty() {
        let _ = writeln!(s, "| {} | | | | — | — | {} |", level.name, d.not_applied);
    }
    for t in &sc.cochran {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} / {} | {} |",
            level.name,
            t.p,
            t.n,
            labs[t.cell],
            fixed(t.statistic, 3, lang),
            fixed(t.critical_5, 3, lang),
            fixed(t.critical_1, 3, lang),
            outcome_text(t.outcome, d)
        );
    }
}

fn grubbs_rows(s: &mut String, level: &LevelResult, sc: &Scrutiny, labs: &[String], lang: Lang) {
    let d = il_labels(lang);
    let Some(g) = &sc.grubbs else {
        let _ = writeln!(s, "| {} | | | — | — | {} |", level.name, d.not_applied);
        return;
    };
    let single = |s: &mut String, name: &str, t: &uncertainty_core::interlab::GrubbsSingle| {
        let _ = writeln!(
            s,
            "| {} | {name} | {} | {} | {} / {} | {} |",
            level.name,
            labs[t.index],
            fixed(t.statistic, 3, lang),
            fixed(t.critical_5, 3, lang),
            fixed(t.critical_1, 3, lang),
            outcome_text(t.outcome, d)
        );
    };
    single(s, d.single_low, &g.low);
    single(s, d.single_high, &g.high);
    if let Some(r) = &g.repeated {
        single(s, d.repeated, r);
    }
    for (name, t) in [(d.double_low, &g.double_low), (d.double_high, &g.double_high)] {
        match t {
            Some(t) => {
                let _ = writeln!(
                    s,
                    "| {} | {name} | {}, {} | {} | {} / {} | {} |",
                    level.name,
                    labs[t.indices[0]],
                    labs[t.indices[1]],
                    fixed(t.statistic, 4, lang),
                    fixed(t.critical_5, 4, lang),
                    fixed(t.critical_1, 4, lang),
                    outcome_text(t.outcome, d)
                );
            }
            None if g.repeated.is_none() => {
                let _ = writeln!(s, "| {} | {name} | | — | — | {} |", level.name, d.not_applied);
            }
            None => {}
        }
    }
}

fn pick(l: &LevelResult, final_: bool) -> Option<&Scrutiny> {
    if final_ {
        l.final_scrutiny.as_ref()
    } else {
        Some(&l.initial)
    }
}

fn tests_tables(s: &mut String, a: &InterlabAnalysis, final_: bool, lang: Lang) {
    let d = il_labels(lang);
    let _ = writeln!(s, "**{}**\n", d.cochran);
    if !final_ {
        let _ = writeln!(s, "{}\n", d.cochran_text);
    }
    let _ = writeln!(
        s,
        "| {} | p | n | {} | C | {} | {} |\n|---|---:|---:|---|---:|---:|---|",
        d.level, d.laboratory, d.critical, d.outcome
    );
    for l in &a.levels {
        if let Some(sc) = pick(l, final_) {
            cochran_rows(s, l, sc, &a.laboratories, lang);
        }
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "**{}**\n", d.grubbs);
    if !final_ {
        let _ = writeln!(s, "{}\n", d.grubbs_text);
    }
    let _ = writeln!(
        s,
        "| {} | | {} | {} | {} | {} |\n|---|---|---|---:|---:|---|",
        d.level, d.laboratory, d.statistic, d.critical, d.outcome
    );
    for l in &a.levels {
        if let Some(sc) = pick(l, final_) {
            grubbs_rows(s, l, sc, &a.laboratories, lang);
        }
    }
    let _ = writeln!(s);
}

pub(super) fn markdown(
    p: &Project,
    a: &InterlabAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = il_labels(lang);
    let Some(spec) = p.precision_experiment.as_ref() else { return String::new() };
    let unit = unit_suffix(a);
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", d.title);
    let unit_b = a.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default();
    let _ = writeln!(
        s,
        "{}: {}{unit_b}. {}\n",
        d.quantity,
        a.quantity,
        d.design.replace("{p}", &a.laboratories.len().to_string()).replace("{q}", &a.levels.len().to_string())
    );
    let _ = writeln!(s, "{}\n", if a.method == MethodSpec::Reml { d.method_reml } else { d.method_anova });

    let _ = writeln!(s, "## {}\n", d.form_b);
    form_table(&mut s, a, true, lang, form_decimals(p));
    let _ = writeln!(s, "## {}\n", d.form_c);
    form_table(&mut s, a, false, lang, form_decimals(p));
    let _ = writeln!(s, "{}\n", d.legend);

    let _ = writeln!(s, "## {}\n", d.mandel);
    let _ = writeln!(s, "{}\n", d.mandel_text);
    for (title, is_h) in [("h", true), ("k", false)] {
        let mut head = format!("| {} |", d.laboratory);
        let mut rule = "|---|".to_string();
        for lv in &a.levels {
            head += &format!(" {title} ({}) |", lv.name);
            rule += "---:|";
        }
        let _ = writeln!(s, "{head}\n{rule}");
        for (i, lab) in a.laboratories.iter().enumerate() {
            let mut row = format!("| {lab} |");
            for lv in &a.levels {
                let sc = &lv.initial;
                let v = sc.labs.iter().position(|&j| j == i).and_then(|k| if is_h { Some(sc.h[k]) } else { sc.k[k] });
                row += &format!(" {} |", v.map_or("—".to_string(), |v| fixed(v, 2, lang)));
            }
            let _ = writeln!(s, "{row}");
        }
        let mut ind = format!("| {} |", d.indicators);
        for lv in &a.levels {
            let i = if is_h { lv.initial.h_indicators } else { lv.initial.k_indicators };
            ind += &format!(
                " {} |",
                i.map_or("—".to_string(), |(a5, a1)| format!("{} / {}", fixed(a5, 2, lang), fixed(a1, 2, lang)))
            );
        }
        let _ = writeln!(s, "{ind}\n");
    }

    tests_tables(&mut s, a, false, lang);

    if !spec.exclude.is_empty() {
        let _ = writeln!(s, "## {}\n", d.exclusions);
        for x in &spec.exclude {
            let what = match (&x.level, x.result) {
                (None, _) => format!("{} {} ({})", d.laboratory, x.laboratory, d.whole_lab),
                (Some(lv), None) => format!("{} {} – {} {}, {}", d.laboratory, x.laboratory, d.level, lv, d.cell),
                (Some(lv), Some(r)) => {
                    format!("{} {} – {} {}, {} {r}", d.laboratory, x.laboratory, d.level, lv, d.result)
                }
            };
            let reason = x.reason.as_ref().map(|r| format!(": {}", r.get(lang))).unwrap_or_default();
            let _ = writeln!(s, "- {what}{reason}");
        }
        let _ = writeln!(s);
        if a.levels.iter().any(|l| l.final_scrutiny.is_some()) {
            let _ = writeln!(s, "## {}\n", d.retained);
            tests_tables(&mut s, a, true, lang);
        }
    }
    let outliers_left = a.levels.iter().any(|l| {
        let sc = l.retained_scrutiny();
        sc.labs.iter().any(|&i| {
            let (c, g) = sc.outcomes(i);
            c == Outcome::Outlier || g == Outcome::Outlier
        })
    });
    if outliers_left {
        let _ = writeln!(s, "{}\n", d.retained_outliers);
    }

    let _ = writeln!(s, "## {}\n", d.precision);
    let reml = a.method == MethodSpec::Reml;
    let _ = writeln!(
        s,
        "| {} | p | m̂ | s_r | s_R | s_L |{} A_r | A_R |\n|---|---:|---:|---:|---:|---:|{}---:|---:|",
        d.level,
        if reml { " s(m̂) |" } else { "" },
        if reml { "---:|" } else { "" }
    );
    for lv in &a.levels {
        let e = &lv.estimates;
        let _ = writeln!(
            s,
            "| {} | {} | {}{unit} | {}{unit} | {}{unit} | {}{unit} |{} {} | {} |",
            lv.name,
            e.p,
            fmt_num(e.mean, 5, lang),
            fmt_num(e.s_r, 4, lang),
            fmt_num(e.s_rr, 4, lang),
            fmt_num(e.s_l, 4, lang),
            if reml {
                format!(" {}{unit} |", e.u_mean.map_or("—".into(), |u| fmt_num(u, 3, lang)))
            } else {
                String::new()
            },
            lv.a_r.map_or("—".to_string(), |x| fixed(x, 2, lang)),
            lv.a_rr.map_or("—".to_string(), |x| fixed(x, 2, lang)),
        );
    }
    let _ = writeln!(s, "\n{}\n", d.a_note);
    for st in statements(a, lang) {
        let _ = writeln!(s, "**{}:** {st}\n", l.reported);
    }

    if !a.relationships.is_empty() {
        let _ = writeln!(s, "## {}\n", d.relationships);
        let _ = writeln!(s, "{}\n", d.relationships_text);
        let mut head = format!("| | {} |", d.relationship);
        let mut rule = "|---|---|".to_string();
        for lv in &a.levels {
            head += &format!(" {} |", lv.name);
            rule += "---:|";
        }
        let _ = writeln!(s, "{head}\n{rule}");
        for rep in [false, true] {
            let name = if rep { d.reproducibility } else { d.repeatability };
            let mut row = format!("| {name} | ({}) |", d.observed);
            for lv in &a.levels {
                row += &format!(" {} |", fmt_num(if rep { lv.estimates.s_rr } else { lv.estimates.s_r }, 3, lang));
            }
            let _ = writeln!(s, "{row}");
            for r in a.relationships.iter().filter(|r| r.of_reproducibility == rep) {
                let mut row = format!("| {} | ", r.relationship.code());
                match &r.fit {
                    Ok(f) => {
                        row += &format!("{} |", relationship_formula(f, lang));
                        for lv in &a.levels {
                            row += &format!(" {} |", fmt_num(f.sd_at(lv.estimates.mean), 3, lang));
                        }
                    }
                    Err(e) => {
                        row += &format!("{}: {e} |", d.not_fitted);
                        for _ in &a.levels {
                            row += " — |";
                        }
                    }
                }
                let _ = writeln!(s, "{row}");
            }
        }
        let _ = writeln!(s);
    }
    tail(&mut s, p, checks, lang, "ISO 5725-2:2019", generated);
    s
}

/// Mandel's h or k grouped by laboratory, one bar per level (SVG).
pub(super) fn mandel_svg(a: &InterlabAnalysis, is_h: bool, lang: Lang) -> Option<String> {
    let d = il_labels(lang);
    let q = a.levels.len();
    let nl = a.laboratories.len();
    let value = |lv: &LevelResult, i: usize| {
        let sc = &lv.initial;
        sc.labs.iter().position(|&j| j == i).and_then(|k| if is_h { Some(sc.h[k]) } else { sc.k[k] })
    };
    // Indicators of the level with the most laboratories.
    let lv0 = a.levels.iter().max_by_key(|l| l.initial.labs.len())?;
    let ind = if is_h { lv0.initial.h_indicators } else { lv0.initial.k_indicators };
    let vmax = a
        .levels
        .iter()
        .flat_map(|lv| (0..nl).filter_map(move |i| value(lv, i)))
        .map(f64::abs)
        .chain(ind.map(|i| i.1 * 1.1))
        .fold(1.0_f64, f64::max)
        .min(10.0);
    if !vmax.is_finite() {
        return None;
    }
    let (w, h, left, top, bottom) = (640.0, 240.0, 40.0, 10.0, 26.0);
    let plot_w = w - left - 10.0;
    let (lo, hi) = if is_h { (-vmax, vmax) } else { (0.0, vmax) };
    let y_of = |v: f64| top + (h - top - bottom) * (hi - v.clamp(lo, hi)) / (hi - lo);
    let group = plot_w / nl as f64;
    let bw = (group * 0.8 / q as f64).max(1.0);
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(if is_h { d.h_chart } else { d.k_chart })
    );
    let y0 = y_of(0.0);
    let _ = write!(s, r#"<line x1="{left}" x2="{:.1}" y1="{y0:.1}" y2="{y0:.1}" class="axis"/>"#, left + plot_w);
    if let Some((i5, i1)) = ind {
        let lines: Vec<(f64, &str)> = if is_h {
            vec![(i5, "gumline"), (-i5, "gumline"), (i1, "mcm"), (-i1, "mcm")]
        } else {
            vec![(i5, "gumline"), (i1, "mcm")]
        };
        for (v, class) in lines {
            let y = y_of(v);
            let _ = write!(
                s,
                r#"<line x1="{left}" x2="{:.1}" y1="{y:.1}" y2="{y:.1}" class="{class}"/><text x="{}" y="{:.1}" text-anchor="end" class="val">{}</text>"#,
                left + plot_w,
                left - 4.0,
                y + 4.0,
                fixed(v, 2, lang)
            );
        }
    }
    for (i, lab) in a.laboratories.iter().enumerate() {
        let gx = left + group * i as f64 + group * 0.1;
        for (j, lv) in a.levels.iter().enumerate() {
            if let Some(v) = value(lv, i) {
                let y = y_of(v);
                let _ = write!(
                    s,
                    r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" class="bar"/>"#,
                    gx + bw * j as f64,
                    y.min(y0),
                    (bw - 1.0).max(0.5),
                    (y - y0).abs().max(0.5)
                );
            }
        }
        let _ = write!(
            s,
            r#"<text x="{:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#,
            left + group * (i as f64 + 0.5),
            h - 8.0,
            esc(lab)
        );
    }
    s.push_str("</svg>");
    Some(s)
}

/// s_r and s_R against m̂ with the fitted relationships (SVG).
pub(super) fn precision_svg(a: &InterlabAnalysis, lang: Lang) -> Option<String> {
    if a.levels.len() < 2 {
        return None;
    }
    let d = il_labels(lang);
    let m: Vec<f64> = a.levels.iter().map(|l| l.estimates.mean).collect();
    let (m_lo, m_hi) = m.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)));
    let x_lo = if m_lo > 0.0 { 0.0 } else { m_lo };
    let x_hi = m_hi * 1.05;
    let s_max = a.levels.iter().map(|l| l.estimates.s_rr.max(l.estimates.s_r)).fold(0.0_f64, f64::max) * 1.15;
    if s_max.is_nan() || s_max <= 0.0 || x_hi.is_nan() || x_hi <= x_lo {
        return None;
    }
    let (w, h, left, top, bottom) = (640.0, 260.0, 56.0, 10.0, 26.0);
    let plot_w = w - left - 12.0;
    let x_of = |x: f64| left + plot_w * (x - x_lo) / (x_hi - x_lo);
    let y_of = |y: f64| top + (h - top - bottom) * (1.0 - (y / s_max).clamp(0.0, 1.0));
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(d.s_chart)
    );
    let _ = write!(
        s,
        r#"<line x1="{left}" x2="{:.1}" y1="{:.1}" y2="{:.1}" class="axis"/><line x1="{left}" x2="{left}" y1="{top}" y2="{:.1}" class="axis"/>"#,
        left + plot_w,
        y_of(0.0),
        y_of(0.0),
        y_of(0.0)
    );
    for t in [0.0, 0.5, 1.0] {
        let v = s_max * t / 1.15;
        let _ = write!(
            s,
            r#"<text x="{}" y="{:.1}" text-anchor="end" class="val">{}</text>"#,
            left - 4.0,
            y_of(v) + 4.0,
            fmt_num(v, 2, lang)
        );
    }
    for x in &m {
        let _ = write!(
            s,
            r#"<text x="{:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#,
            x_of(*x),
            h - 8.0,
            fmt_num(*x, 3, lang)
        );
    }
    for r in &a.relationships {
        let Ok(f) = &r.fit else { continue };
        let class = if r.of_reproducibility { "pdf" } else { "mcm" };
        let pts: Vec<String> = (0..=60)
            .map(|i| {
                let x = x_lo.max(1e-9 * x_hi) + (x_hi - x_lo.max(1e-9 * x_hi)) * i as f64 / 60.0;
                format!("{:.1},{:.1}", x_of(x), y_of(f.sd_at(x)))
            })
            .collect();
        let dash = match r.relationship {
            RelationshipSpec::I => "",
            RelationshipSpec::II => r#" stroke-dasharray="6 3""#,
            RelationshipSpec::III => r#" stroke-dasharray="2 3""#,
            RelationshipSpec::IV => r#" stroke-dasharray="8 3 2 3""#,
        };
        let _ = write!(s, r#"<polyline points="{}" class="{class}" fill="none" opacity="0.7"{dash}/>"#, pts.join(" "));
    }
    for l in &a.levels {
        let x = x_of(l.estimates.mean);
        let _ = write!(s, r#"<circle cx="{x:.1}" cy="{:.1}" r="4" class="bar"/>"#, y_of(l.estimates.s_r));
        let _ = write!(
            s,
            r#"<rect x="{:.1}" y="{:.1}" width="8" height="8" class="hist"/>"#,
            x - 4.0,
            y_of(l.estimates.s_rr) - 4.0
        );
    }
    s.push_str("</svg>");
    Some(s)
}
