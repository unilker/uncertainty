//! Report of the conformity assessment of a multicomponent item (total
//! risks, EMUE E2.1).

use super::*;
use crate::multiconformity::MulticomponentAnalysis;

pub(super) struct McLabels {
    pub title: &'static str,
    pub component: &'static str,
    pub prior: &'static str,
    pub tolerance: &'static str,
    pub correlation: &'static str,
    pub u_rel: &'static str,
    pub results: &'static str,
    pub cols: [&'static str; 4],
    pub accepted: &'static str,
    pub rejected: &'static str,
    pub consumer: &'static str,
    pub producer: &'static str,
    pub global: &'static str,
    pub global_consumer: &'static str,
    pub global_producer: &'static str,
    pub method: &'static str,
    pub statement_specific: &'static str,
    pub statement_global: &'static str,
}

const MC_EN: McLabels = McLabels {
    title: "Conformity assessment of a multicomponent item (total risks)",
    component: "Component",
    prior: "Prior (production) mean and s.d.",
    tolerance: "Tolerance = acceptance interval",
    correlation: "Correlation matrix (true values and test results)",
    u_rel: "Relative standard uncertainty of the test results",
    results: "Total specific risks",
    cols: ["Item", "Test results", "Posterior means", "Probability that all components conform"],
    accepted: "accepted",
    rejected: "rejected",
    consumer: "total specific consumer's risk",
    producer: "total specific producer's risk",
    global: "Total global risks",
    global_consumer: "Total global consumer's risk (all test results accepted, at least one true value out of tolerance)",
    global_producer: "Total global producer's risk (at least one test result rejected, all true values within tolerance)",
    method: "Multivariate normal prior and likelihood; the posterior is normal (EMUE (E2.1.1)–(E2.1.2)); its probability over the tolerance box by the separation-of-variables method of Genz with {n} antithetic draws; global risks by Monte Carlo with {m} trials.",
    statement_specific: "{name}: {decision}, {who} {r}",
    statement_global: "Total global consumer's risk {c}, producer's risk {p} ({m} Monte Carlo trials)",
};

const MC_TR: McLabels = McLabels {
    title: "Çok bileşenli bir ürünün uygunluk değerlendirmesi (toplam riskler)",
    component: "Bileşen",
    prior: "Önsel (üretim) ortalaması ve s.s.",
    tolerance: "Tolerans = kabul aralığı",
    correlation: "Korelasyon matrisi (gerçek değerler ve deney sonuçları)",
    u_rel: "Deney sonuçlarının bağıl standart belirsizliği",
    results: "Toplam özgül riskler",
    cols: ["Ürün", "Deney sonuçları", "Sonsal ortalamalar", "Tüm bileşenlerin uygun olma olasılığı"],
    accepted: "kabul",
    rejected: "ret",
    consumer: "toplam özgül tüketici riski",
    producer: "toplam özgül üretici riski",
    global: "Toplam küresel riskler",
    global_consumer: "Toplam küresel tüketici riski (tüm deney sonuçları kabul, en az bir gerçek değer tolerans dışı)",
    global_producer: "Toplam küresel üretici riski (en az bir deney sonucu ret, tüm gerçek değerler tolerans içinde)",
    method: "Çok değişkenli normal önsel ve olabilirlik; sonsal dağılım normaldir (EMUE (E2.1.1)–(E2.1.2)); tolerans kutusu üzerindeki olasılığı Genz'in değişkenlere ayırma yöntemiyle, {n} karşıt çekilişle; küresel riskler {m} denemeli Monte Carlo ile.",
    statement_specific: "{name}: {decision}, {who} {r}",
    statement_global: "Toplam küresel tüketici riski {c}, üretici riski {p} ({m} Monte Carlo denemesi)",
};

pub(super) fn mc_labels(lang: Lang) -> &'static McLabels {
    match lang {
        Lang::En => &MC_EN,
        Lang::Tr => &MC_TR,
    }
}

fn pct(x: f64, lang: Lang) -> String {
    fmt_pct(x, 3, lang, false)
}

pub(super) fn statements(a: &MulticomponentAnalysis, lang: Lang) -> Vec<String> {
    let d = mc_labels(lang);
    let mut out: Vec<String> = a
        .results
        .iter()
        .map(|(name, _, s)| {
            d.statement_specific
                .replace("{name}", name)
                .replace("{decision}", if s.accepted { d.accepted } else { d.rejected })
                .replace("{who}", if s.accepted { d.consumer } else { d.producer })
                .replace("{r}", &pct(s.risk, lang))
        })
        .collect();
    if let Some(g) = &a.global {
        out.push(
            d.statement_global
                .replace("{c}", &pct(g.consumer, lang))
                .replace("{p}", &pct(g.producer, lang))
                .replace("{m}", &g.trials.to_string()),
        );
    }
    out
}

pub(super) fn markdown(
    p: &Project,
    a: &MulticomponentAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = mc_labels(lang);
    let unit = a.unit.clone().map(|u| format!(" {u}")).unwrap_or_default();
    let n = |x: f64| fmt_num(x, 4, lang);
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", d.title);
    let _ = writeln!(s, "| {} | {} | {} |\n|---|---|---|", d.component, d.prior, d.tolerance);
    let m = &a.model;
    for (i, name) in a.names.iter().enumerate() {
        let _ = writeln!(
            s,
            "| {name} | {}{unit}, {}{unit} | {} … {}{unit} |",
            n(m.prior_mean[i]),
            n(m.prior_sd[i]),
            n(m.lower[i]),
            n(m.upper[i])
        );
    }
    let _ = writeln!(s, "\n- {}: {}", d.u_rel, pct(m.u_rel, lang));
    let _ = writeln!(s, "- {}:\n", d.correlation);
    let _ = writeln!(s, "| | {} |\n|---|{}", a.names.join(" | "), "---:|".repeat(a.names.len()));
    for (i, name) in a.names.iter().enumerate() {
        let row: Vec<String> = m.correlation[i].iter().map(|r| fmt_num(*r, 3, lang)).collect();
        let _ = writeln!(s, "| {name} | {} |", row.join(" | "));
    }
    let _ = writeln!(s);
    if !a.results.is_empty() {
        let _ = writeln!(s, "### {}\n", d.results);
        let _ =
            writeln!(s, "| {} | {} | {} | {} | |\n|---|---|---|---:|---|", d.cols[0], d.cols[1], d.cols[2], d.cols[3]);
        for (name, v, r) in &a.results {
            let vals: Vec<String> = v.iter().map(|x| n(*x)).collect();
            let post: Vec<String> = r.posterior_mean.iter().map(|x| n(*x)).collect();
            let who = if r.accepted { d.consumer } else { d.producer };
            let dec = if r.accepted { d.accepted } else { d.rejected };
            let _ = writeln!(
                s,
                "| {name} | {} | {} | {} | {dec}; {who} {} |",
                vals.join("; "),
                post.join("; "),
                fmt_pct(r.conformance_probability, 5, lang, false),
                pct(r.risk, lang)
            );
        }
        let _ = writeln!(s);
    }
    if let Some(g) = &a.global {
        let _ = writeln!(s, "### {}\n", d.global);
        let _ = writeln!(s, "- {}: {}", d.global_consumer, pct(g.consumer, lang));
        let _ = writeln!(s, "- {}: {}\n", d.global_producer, pct(g.producer, lang));
    }
    let trials = a.global.map(|g| g.trials).unwrap_or(0);
    let _ = writeln!(s, "{}\n", d.method.replace("{n}", &a.samples.to_string()).replace("{m}", &trials.to_string()));
    tail(&mut s, p, checks, lang, "JCGM 106:2012; EMUE E2.1 (2021)", generated);
    s
}
