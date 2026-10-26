//! Reports of conformity assessments (JCGM 106:2012).

use super::*;
use crate::conformity::{ConformityAnalysis, ProcessKind, RuleSpec};

pub(super) struct CfLabels {
    pub title: &'static str,
    pub tolerance: &'static str,
    pub rule: &'static str,
    pub rules: [&'static str; 3],
    pub guard_band: &'static str,
    pub acceptance: &'static str,
    pub probability_rule: [&'static str; 2],
    pub capability: &'static str,
    pub results: &'static str,
    pub name: &'static str,
    pub value: &'static str,
    pub pc: &'static str,
    pub pc_mcm: &'static str,
    pub decision: &'static str,
    pub accepted: &'static str,
    pub rejected: &'static str,
    pub specific_risk: &'static str,
    pub consumer: &'static str,
    pub producer: &'static str,
    pub global: &'static str,
    pub process: &'static str,
    pub normal: &'static str,
    pub gamma: &'static str,
    pub conforming: &'static str,
    pub accepted_share: &'static str,
    pub consumer_risk: &'static str,
    pub producer_risk: &'static str,
    pub per_hundred: &'static str,
    pub statement: &'static str,
    pub chart: &'static str,
    pub limits_chart: &'static str,
    pub t_note: &'static str,
    pub target_note: &'static str,
}

const CF_EN: CfLabels = CfLabels {
    title: "Conformity assessment (JCGM 106)",
    tolerance: "Tolerance interval",
    rule: "Decision rule",
    rules: ["simple acceptance (acceptance interval = tolerance interval, JCGM 106 8.2)", "guarded acceptance (acceptance limits inside the tolerance limits, 8.3.2)", "guarded rejection (acceptance limits outside the tolerance limits, 8.3.3)"],
    guard_band: "Guard band w",
    acceptance: "Acceptance interval",
    probability_rule: ["acceptance limits set so that the probability of conformity at a limit is", "acceptance limits set so that the probability of non-conformity at a limit is"],
    capability: "Measurement capability index C_m = T/(4u)",
    results: "Assessed results",
    name: "Result",
    value: "Measured value",
    pc: "p_c",
    pc_mcm: "p_c (Monte Carlo)",
    decision: "Decision",
    accepted: "accepted",
    rejected: "rejected",
    specific_risk: "Specific risk",
    consumer: "consumer's",
    producer: "producer's",
    global: "Global risks of the inspected process (JCGM 106 9.5)",
    process: "Process PDF",
    normal: "normal",
    gamma: "gamma",
    conforming: "Items that conform",
    accepted_share: "Items accepted",
    consumer_risk: "Global consumer's risk R_C (accepted, not conforming)",
    producer_risk: "Global producer's risk R_P (rejected, conforming)",
    per_hundred: "Of every 100 items: {c} conform, of which {ca} are accepted and {cr} falsely rejected; {n} do not conform, of which {na} are falsely accepted; of the accepted items {pa} % conform.",
    statement: "Probability that {q} conforms to {tol}: p_c = {pc}; {dec} under {rule}.",
    chart: "Measured values with ± U (k = 2), tolerance limits (solid) and acceptance limits (dashed)",
    limits_chart: "PDF for a measured value at the acceptance limit, with the tolerance limit (solid) and the acceptance limit (dashed)",
    t_note: "The PDF for the measurand is a scaled and shifted t distribution with ν = {nu} (JCGM 101 6.4.9).",
    target_note: "The guard-band multiplier r was found for the required global consumer's risk {t}.",
};

const CF_TR: CfLabels = CfLabels {
    title: "Uygunluk değerlendirmesi (JCGM 106)",
    tolerance: "Tolerans aralığı",
    rule: "Karar kuralı",
    rules: ["basit kabul (kabul aralığı = tolerans aralığı, JCGM 106 8.2)", "korumalı kabul (kabul sınırları tolerans sınırlarının içinde, 8.3.2)", "korumalı ret (kabul sınırları tolerans sınırlarının dışında, 8.3.3)"],
    guard_band: "Koruma bandı w",
    acceptance: "Kabul aralığı",
    probability_rule: ["kabul sınırları, bir sınırdaki uygunluk olasılığı şu değer olacak şekilde belirlendi:", "kabul sınırları, bir sınırdaki uygunsuzluk olasılığı şu değer olacak şekilde belirlendi:"],
    capability: "Ölçüm yeterlilik indeksi C_m = T/(4u)",
    results: "Değerlendirilen sonuçlar",
    name: "Sonuç",
    value: "Ölçülen değer",
    pc: "p_c",
    pc_mcm: "p_c (Monte Carlo)",
    decision: "Karar",
    accepted: "kabul",
    rejected: "ret",
    specific_risk: "Özgül risk",
    consumer: "tüketici",
    producer: "üretici",
    global: "Denetlenen sürecin küresel riskleri (JCGM 106 9.5)",
    process: "Süreç olasılık yoğunluğu",
    normal: "normal",
    gamma: "gama",
    conforming: "Uygun ürünler",
    accepted_share: "Kabul edilen ürünler",
    consumer_risk: "Küresel tüketici riski R_C (kabul edilen, uygun olmayan)",
    producer_risk: "Küresel üretici riski R_P (reddedilen, uygun olan)",
    per_hundred: "Her 100 üründen {c} tanesi uygundur; bunların {ca} tanesi kabul, {cr} tanesi yanlışlıkla reddedilir; {n} tanesi uygun değildir, bunların {na} tanesi yanlışlıkla kabul edilir; kabul edilenlerin %{pa}'i uygundur.",
    statement: "{q} için {tol} şartına uygunluk olasılığı: p_c = {pc}; {rule} kuralına göre {dec}.",
    chart: "± U (k = 2) ile ölçülen değerler, tolerans sınırları (düz) ve kabul sınırları (kesikli)",
    limits_chart: "Kabul sınırındaki bir ölçülen değer için olasılık yoğunluğu, tolerans sınırı (düz) ve kabul sınırı (kesikli)",
    t_note: "Ölçülen büyüklüğün olasılık yoğunluğu ν = {nu} serbestlik dereceli ölçeklenmiş ve kaydırılmış t dağılımıdır (JCGM 101 6.4.9).",
    target_note: "Koruma bandı çarpanı r, istenen küresel tüketici riski {t} için bulunmuştur.",
};

pub(super) fn cf_labels(lang: Lang) -> &'static CfLabels {
    match lang {
        Lang::En => &CF_EN,
        Lang::Tr => &CF_TR,
    }
}

fn rule_index(r: RuleSpec) -> usize {
    match r {
        RuleSpec::SimpleAcceptance => 0,
        RuleSpec::GuardedAcceptance => 1,
        RuleSpec::GuardedRejection => 2,
    }
}

fn rule_short(r: RuleSpec, lang: Lang) -> &'static str {
    match (r, lang) {
        (RuleSpec::SimpleAcceptance, Lang::En) => "simple acceptance",
        (RuleSpec::GuardedAcceptance, Lang::En) => "guarded acceptance",
        (RuleSpec::GuardedRejection, Lang::En) => "guarded rejection",
        (RuleSpec::SimpleAcceptance, Lang::Tr) => "basit kabul",
        (RuleSpec::GuardedAcceptance, Lang::Tr) => "korumalı kabul",
        (RuleSpec::GuardedRejection, Lang::Tr) => "korumalı ret",
    }
}

/// A probability in per cent; close to 1 with enough digits to show 1 − x.
fn pct(x: f64, lang: Lang) -> String {
    if x > 0.9999 {
        return format!("> {}", fmt_pct(0.9999, 4, lang, true));
    }
    if x < 0.0001 {
        return format!("< {}", fmt_pct(0.0001, 1, lang, true));
    }
    let digits = if x > 0.99 { 2 + (-(1.0 - x).log10()).floor() as usize } else { 2 };
    fmt_pct(x, digits, lang, true)
}

/// A limit with two digits beyond its integer part (at least four).
fn limit(x: f64, lang: Lang) -> String {
    let int_digits = if x.abs() >= 1.0 { x.abs().log10().floor() as usize + 1 } else { 1 };
    fmt_num(x, (int_digits + 2).max(4), lang)
}

fn interval(lo: Option<f64>, hi: Option<f64>, q: &str, unit: &str, lang: Lang) -> String {
    let n = |x: f64| format!("{}{unit}", fmt_given(x, lang));
    match (lo, hi) {
        (Some(l), Some(h)) => format!("{} ≤ {q} ≤ {}", n(l), n(h)),
        (Some(l), None) => format!("{q} ≥ {}", n(l)),
        (None, Some(h)) => format!("{q} ≤ {}", n(h)),
        (None, None) => "—".into(),
    }
}

/// One statement per assessed result.
pub(super) fn statements(a: &ConformityAnalysis, lang: Lang) -> Vec<String> {
    let d = cf_labels(lang);
    let unit = a.unit.clone().map(|u| format!(" {u}")).unwrap_or_default();
    a.results
        .iter()
        .map(|r| {
            d.statement
                .replace("{q}", &r.name)
                .replace("{tol}", &interval(a.tolerance.lower, a.tolerance.upper, &a.quantity, &unit, lang))
                .replace("{pc}", &pct(r.conformance_probability, lang))
                .replace("{dec}", if r.accepted { d.accepted } else { d.rejected })
                .replace("{rule}", rule_short(a.rule, lang))
        })
        .collect()
}

/// The conformity section (also used inside model reports).
pub(super) fn section(s: &mut String, a: &ConformityAnalysis, lang: Lang) {
    let d = cf_labels(lang);
    let unit = a.unit.clone().map(|u| format!(" {u}")).unwrap_or_default();
    let _ = writeln!(s, "## {}\n", d.title);
    let _ =
        writeln!(s, "- {}: {}", d.tolerance, interval(a.tolerance.lower, a.tolerance.upper, &a.quantity, &unit, lang));
    let _ = writeln!(s, "- {}: {}", d.rule, d.rules[rule_index(a.rule)]);
    if let Some(p) = a.probability {
        let which = if a.rule == RuleSpec::GuardedRejection { 1 } else { 0 };
        let _ = writeln!(s, "- {} {}", d.probability_rule[which], pct(p, lang));
    }
    if let Some(w) = a.guard_band {
        let r = a.r.map(|r| format!(" (r = {})", fmt_num(r, 3, lang))).unwrap_or_default();
        let _ = writeln!(s, "- {}: {}{unit}{r}", d.guard_band, fmt_num(w, 3, lang));
    }
    let acc = |x: Option<f64>| x.map(|v| format!("{}{unit}", limit(v, lang)));
    let acc_text = match (acc(a.acceptance.lower), acc(a.acceptance.upper)) {
        (Some(l), Some(h)) => format!("{l} … {h}"),
        (Some(l), None) => format!("≥ {l}"),
        (None, Some(h)) => format!("≤ {h}"),
        (None, None) => "—".into(),
    };
    let _ = writeln!(s, "- {}: {acc_text}", d.acceptance);
    if let Some(c) = a.capability_index {
        let _ = writeln!(s, "- {}: {}", d.capability, fmt_num(c, 3, lang));
    }
    if let Some(nu) = a.results.first().and_then(|r| r.dof).filter(|nu| nu.is_finite()) {
        let _ = writeln!(s, "\n{}", d.t_note.replace("{nu}", &fmt_num(nu, 3, lang)));
    }
    let _ = writeln!(s);
    if !a.results.is_empty() {
        let mcm = a.results.iter().any(|r| r.conformance_probability_mcm.is_some());
        let _ = writeln!(s, "### {}\n", d.results);
        let mut head = format!("| {} | {} | u | {} |", d.name, d.value, d.pc);
        let mut rule = "|---|---:|---:|---:|".to_string();
        if mcm {
            head += &format!(" {} |", d.pc_mcm);
            rule += "---:|";
        }
        head += &format!(" {} | {} |", d.decision, d.specific_risk);
        rule += "---|---|";
        let _ = writeln!(s, "{head}\n{rule}");
        for r in &a.results {
            let mut row = format!(
                "| {} | {}{unit} | {}{unit} | {} |",
                r.name,
                fmt_given(r.value, lang),
                fmt_num(r.u, 3, lang),
                pct(r.conformance_probability, lang)
            );
            if mcm {
                row += &format!(
                    " {} |",
                    r.conformance_probability_mcm.map(|p| pct(p, lang)).unwrap_or_else(|| "—".into())
                );
            }
            let (dec, who) = if r.accepted { (d.accepted, d.consumer) } else { (d.rejected, d.producer) };
            row += &format!(" {dec} | {who}: {} |", pct(r.specific_risk, lang));
            let _ = writeln!(s, "{row}");
        }
        let _ = writeln!(s);
    }
    if let (Some(g), Some(p)) = (&a.global, &a.process) {
        let _ = writeln!(s, "### {}\n", d.global);
        let kind = match p.distribution {
            ProcessKind::Normal => d.normal,
            ProcessKind::Gamma => d.gamma,
        };
        let _ = writeln!(
            s,
            "- {}: {kind}, {} {}{unit}, s {}{unit}",
            d.process,
            if lang == Lang::Tr { "ortalama" } else { "mean" },
            fmt_given(p.mean, lang),
            fmt_given(p.sd, lang)
        );
        let _ = writeln!(s, "- {}: {}", d.conforming, pct(g.conforming, lang));
        let _ = writeln!(s, "- {}: {}", d.accepted_share, pct(g.accepted, lang));
        let _ = writeln!(s, "- {}: {}", d.consumer_risk, pct(g.consumer, lang));
        let _ = writeln!(s, "- {}: {}\n", d.producer_risk, pct(g.producer, lang));
        let h = |x: f64| fmt_num(100.0 * x, 3, lang);
        let _ = writeln!(
            s,
            "{}\n",
            d.per_hundred
                .replace("{c}", &h(g.conforming))
                .replace("{ca}", &h(g.conforming - g.producer))
                .replace("{cr}", &h(g.producer))
                .replace("{n}", &h(1.0 - g.conforming))
                .replace("{na}", &h(g.consumer))
                .replace("{pa}", &fmt_num(100.0 * (g.accepted - g.consumer) / g.accepted, 3, lang))
        );
    }
}

/// Markdown report of a conformity project.
pub(super) fn markdown(
    p: &Project,
    a: &ConformityAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    section(&mut s, a, lang);
    if let Some(t) = p.conformity.as_ref().and_then(|c| c.target_consumer_risk) {
        let _ = writeln!(s, "{}\n", cf_labels(lang).target_note.replace("{t}", &pct(t, lang)));
    }
    for st in statements(a, lang) {
        let _ = writeln!(s, "**{}:** {st}\n", l.reported);
    }
    tail(&mut s, p, checks, lang, "JCGM 106:2012", generated);
    s
}

/// Measured values with ± 2u against tolerance and acceptance limits (SVG),
/// after JCGM 106 Figure 8.
pub(super) fn chart_svg(a: &ConformityAnalysis, lang: Lang) -> Option<String> {
    if a.results.is_empty() {
        return limits_svg(a, lang);
    }
    let (w, row_h, left, right) = (640.0, 26.0, 110.0, 20.0);
    let h = row_h * a.results.len() as f64 + 34.0;
    let mut xs: Vec<f64> = a.results.iter().flat_map(|r| [r.value - 2.0 * r.u, r.value + 2.0 * r.u]).collect();
    xs.extend([a.tolerance.lower, a.tolerance.upper, a.acceptance.lower, a.acceptance.upper].into_iter().flatten());
    let (lo, hi) = xs.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)));
    let pad = 0.08 * (hi - lo).max(1e-300);
    let (lo, hi) = (lo - pad, hi + pad);
    let plot = w - left - right;
    let x_of = |v: f64| left + plot * (v - lo) / (hi - lo);
    let bottom = h - 22.0;
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(cf_labels(lang).chart)
    );
    for (v, class) in [
        (a.tolerance.lower, "axis"),
        (a.tolerance.upper, "axis"),
        (a.acceptance.lower, "mcm"),
        (a.acceptance.upper, "mcm"),
    ] {
        if let Some(v) = v {
            let x = x_of(v);
            let _ = write!(
                s,
                r#"<line x1="{x:.1}" x2="{x:.1}" y1="4" y2="{bottom:.1}" class="{class}"/><text x="{x:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#,
                h - 6.0,
                fmt_num(v, 4, lang)
            );
        }
    }
    for (i, r) in a.results.iter().enumerate() {
        let y = 14.0 + i as f64 * row_h;
        let (x0, x1, xm) = (x_of(r.value - 2.0 * r.u), x_of(r.value + 2.0 * r.u), x_of(r.value));
        let _ = write!(
            s,
            r#"<text x="{}" y="{:.1}" text-anchor="end" class="lbl">{}</text><line x1="{x0:.1}" x2="{x1:.1}" y1="{y:.1}" y2="{y:.1}" class="pdf"/><circle cx="{xm:.1}" cy="{y:.1}" r="4" class="bar"/>"#,
            left - 8.0,
            y + 4.0,
            esc(&r.name)
        );
    }
    s.push_str("</svg>");
    Some(s)
}

/// Design study without measured values: the PDF for a measured value at each
/// acceptance limit, with the tolerance limits (SVG).
fn limits_svg(a: &ConformityAnalysis, lang: Lang) -> Option<String> {
    use uncertainty_core::special::{normal_pdf, student_t_pdf};
    if a.limit_knowledge.is_empty() {
        return None;
    }
    let (w, h, left, right, top, bottom) = (640.0, 220.0, 20.0, 20.0, 10.0, 26.0);
    let mut xs: Vec<f64> = a.limit_knowledge.iter().flat_map(|&(m, u, _)| [m - 4.5 * u, m + 4.5 * u]).collect();
    xs.extend([a.tolerance.lower, a.tolerance.upper].into_iter().flatten());
    let (lo, hi) = xs.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| (a.min(x), b.max(x)));
    let x_of = |v: f64| left + (w - left - right) * (v - lo) / (hi - lo);
    let dens = |x: f64, m: f64, u: f64, nu: Option<f64>| match nu {
        Some(nu) if nu.is_finite() => student_t_pdf((x - m) / u, nu) / u,
        _ => normal_pdf((x - m) / u) / u,
    };
    let peak = a.limit_knowledge.iter().map(|&(m, u, nu)| dens(m, m, u, nu)).fold(0.0, f64::max);
    let y_of = |d: f64| h - bottom - (h - top - bottom) * d / peak;
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(cf_labels(lang).limits_chart)
    );
    for &(m, u, nu) in &a.limit_knowledge {
        let pts: Vec<String> = (0..=160)
            .map(|i| {
                let x = m - 4.5 * u + 9.0 * u * i as f64 / 160.0;
                format!("{:.1},{:.1}", x_of(x), y_of(dens(x, m, u, nu)))
            })
            .collect();
        let _ = write!(s, r#"<polyline points="{}" class="pdf"/>"#, pts.join(" "));
    }
    for (v, class) in [
        (a.tolerance.lower, "axis"),
        (a.tolerance.upper, "axis"),
        (a.acceptance.lower, "mcm"),
        (a.acceptance.upper, "mcm"),
    ] {
        if let Some(v) = v {
            let x = x_of(v);
            let _ = write!(
                s,
                r#"<line x1="{x:.1}" x2="{x:.1}" y1="{top}" y2="{:.1}" class="{class}"/><text x="{x:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#,
                h - bottom,
                h - 8.0,
                fmt_num(v, 4, lang)
            );
        }
    }
    s.push_str("</svg>");
    Some(s)
}
