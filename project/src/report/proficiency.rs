//! Reports of proficiency-testing projects (ISO 13528:2022).

use super::*;
use crate::proficiency::{AssignedMethod, ProficiencyAnalysis, SdPtMethod, UncertaintyFlag};
use uncertainty_core::pt::{en_satisfactory, z_signal, Signal};

pub(super) struct PtLabels {
    pub title: &'static str,
    pub measurand: &'static str,
    pub summary: &'static str,
    pub statistic: &'static str,
    pub value: &'static str,
    pub participants_n: &'static str,
    pub median: &'static str,
    pub made: &'static str,
    pub niqr: &'static str,
    pub robust_mean: &'static str,
    pub robust_sd: &'static str,
    pub iterations: &'static str,
    pub mean: &'static str,
    pub sd: &'static str,
    pub assigned: &'static str,
    pub assigned_value: &'static str,
    pub u_assigned: &'static str,
    pub expanded_assigned: &'static str,
    pub sd_pt: &'static str,
    pub delta_e: &'static str,
    pub m_algorithm_a: &'static str,
    pub m_median: &'static str,
    pub m_mean: &'static str,
    pub m_reference: &'static str,
    pub s_robust: &'static str,
    pub s_made: &'static str,
    pub s_niqr: &'static str,
    pub s_sd: &'static str,
    pub s_given: &'static str,
    pub negligible: &'static str,
    pub not_negligible: &'static str,
    pub scores: &'static str,
    pub lab: &'static str,
    pub result: &'static str,
    pub u_lab: &'static str,
    pub signal: &'static str,
    pub flag: &'static str,
    pub acceptable: &'static str,
    pub warning: &'static str,
    pub action: &'static str,
    pub censored: &'static str,
    pub flags_note: &'static str,
    pub homogeneity: &'static str,
    pub items: &'static str,
    pub general_mean: &'static str,
    pub criterion: &'static str,
    pub expanded_criterion: &'static str,
    pub sufficient: &'static str,
    pub not_sufficient: &'static str,
    pub u_bb_star: &'static str,
    pub no_sigma_pt: &'static str,
    pub bayes_title: &'static str,
    pub mean_col: &'static str,
    pub sd_col: &'static str,
    pub median_col: &'static str,
    pub mode_col: &'static str,
    pub q_col: &'static str,
    pub hpd_col: &'static str,
    pub bayes_note: &'static str,
    pub stability: &'static str,
    pub difference: &'static str,
    pub chart: &'static str,
    pub homogeneity_chart: &'static str,
    pub statement: &'static str,
}

const PT_EN: PtLabels = PtLabels {
    title: "Proficiency testing — statistics of the round",
    measurand: "Measurand",
    summary: "Summary statistics of the participant results",
    statistic: "Statistic",
    value: "Value",
    participants_n: "Number of numerical results p",
    median: "Median",
    made: "MADe (scaled median absolute deviation)",
    niqr: "nIQR (normalised interquartile range)",
    robust_mean: "Robust average x* (Algorithm A)",
    robust_sd: "Robust standard deviation s* (Algorithm A)",
    iterations: "iterations",
    mean: "Arithmetic mean",
    sd: "Standard deviation",
    assigned: "Assigned value and performance criteria",
    assigned_value: "Assigned value x_pt",
    u_assigned: "Standard uncertainty u(x_pt)",
    expanded_assigned: "Expanded uncertainty U(x_pt)",
    sd_pt: "Standard deviation for proficiency assessment σ_pt",
    delta_e: "Allowance for measurement error δ_E",
    m_algorithm_a: "robust average of the participant results (Algorithm A, ISO 13528 C.3), u(x_pt) = 1.25 s*/√p",
    m_median: "median of the participant results, u(x_pt) = 1.25 × scale/√p",
    m_mean: "arithmetic mean of the participant results, u(x_pt) = s/√p",
    m_reference: "reference value (formulation, CRM or reference laboratory)",
    s_robust: "robust standard deviation of the participant results",
    s_made: "MADe of the participant results",
    s_niqr: "nIQR of the participant results",
    s_sd: "standard deviation of the participant results",
    s_given: "fitness for purpose / previous rounds / general model",
    negligible: "u(x_pt) < 0.3 σ_pt: the uncertainty of the assigned value is negligible (ISO 13528 9.2.1).",
    not_negligible: "u(x_pt) ≥ 0.3 σ_pt: the uncertainty of the assigned value is not negligible; use z′, ζ or E_n scores (ISO 13528 9.2.2).",
    scores: "Performance statistics",
    lab: "Laboratory",
    result: "Result",
    u_lab: "u(x)",
    signal: "z signal",
    flag: "u flag",
    acceptable: "acceptable",
    warning: "warning",
    action: "action",
    censored: "censored result, not scored",
    flags_note: "Flags of the reported uncertainties (ISO 13528 9.8): a — plausible, b — below u_min, c — above u_max.",
    homogeneity: "Homogeneity check (ISO 13528 B.3)",
    items: "items × replicates",
    general_mean: "General average",
    criterion: "Criterion 0.3 σ_pt",
    expanded_criterion: "Expanded criterion √c (B.2.3)",
    sufficient: "sufficient",
    not_sufficient: "not sufficient",
    u_bb_star: "u*_bb = √(MS_within/m)·(2/ν_within)^¼ (between-unit s.d. that the repeatability can hide; Linsinger et al. 2001)",
    no_sigma_pt: "No σ_pt is given, so the ISO 13528 criteria 0.3 σ_pt are not evaluated.",
    bayes_title: "Bayesian between-unit standard deviation (JCGM GUM-5 Example 12)",
    mean_col: "mean",
    sd_col: "s.d.",
    median_col: "median",
    mode_col: "mode",
    q_col: "95 % (2.5 %, 97.5 %)",
    hpd_col: "95 % shortest",
    bayes_note: "Model y_ij = µ + B_i + ε_ij, B_i ~ N(0, τ²), ε_ij ~ N(0, σ²), half-Cauchy priors for τ and σ. The unit effects and µ are integrated analytically and the posterior of (τ, σ) is evaluated on a grid, without MCMC. Posterior of µ: {mu}, standard deviation {u}.",
    stability: "Stability check (ISO 13528 B.5)",
    difference: "Difference of the averages",
    chart: "z scores (±2 warning, ±3 action limits)",
    homogeneity_chart: "Homogeneity check: replicate results of each item, general average and ± 0.3 σ_pt",
    statement: "x_pt = {v} {unit}, U(x_pt) = {U} {unit} (k = {k})",
};

const PT_TR: PtLabels = PtLabels {
    title: "Yeterlilik testi — turun istatistikleri",
    measurand: "Ölçülen büyüklük",
    summary: "Katılımcı sonuçlarının özet istatistikleri",
    statistic: "İstatistik",
    value: "Değer",
    participants_n: "Sayısal sonuç sayısı p",
    median: "Medyan",
    made: "MADe (ölçeklenmiş medyan mutlak sapma)",
    niqr: "nIQR (normalleştirilmiş çeyrekler açıklığı)",
    robust_mean: "Robust ortalama x* (Algoritma A)",
    robust_sd: "Robust standart sapma s* (Algoritma A)",
    iterations: "iterasyon",
    mean: "Aritmetik ortalama",
    sd: "Standart sapma",
    assigned: "Atanan değer ve performans ölçütleri",
    assigned_value: "Atanan değer x_pt",
    u_assigned: "Standart belirsizlik u(x_pt)",
    expanded_assigned: "Genişletilmiş belirsizlik U(x_pt)",
    sd_pt: "Yeterlilik değerlendirmesi için standart sapma σ_pt",
    delta_e: "Ölçüm hatası için izin δ_E",
    m_algorithm_a: "katılımcı sonuçlarının robust ortalaması (Algoritma A, ISO 13528 C.3), u(x_pt) = 1,25 s*/√p",
    m_median: "katılımcı sonuçlarının medyanı, u(x_pt) = 1,25 × ölçek/√p",
    m_mean: "katılımcı sonuçlarının aritmetik ortalaması, u(x_pt) = s/√p",
    m_reference: "referans değer (formülasyon, CRM veya referans laboratuvar)",
    s_robust: "katılımcı sonuçlarının robust standart sapması",
    s_made: "katılımcı sonuçlarının MADe değeri",
    s_niqr: "katılımcı sonuçlarının nIQR değeri",
    s_sd: "katılımcı sonuçlarının standart sapması",
    s_given: "amaca uygunluk / önceki turlar / genel model",
    negligible: "u(x_pt) < 0,3 σ_pt: atanan değerin belirsizliği ihmal edilebilir (ISO 13528 9.2.1).",
    not_negligible: "u(x_pt) ≥ 0,3 σ_pt: atanan değerin belirsizliği ihmal edilemez; z′, ζ veya E_n skorlarını kullanın (ISO 13528 9.2.2).",
    scores: "Performans istatistikleri",
    lab: "Laboratuvar",
    result: "Sonuç",
    u_lab: "u(x)",
    signal: "z sinyali",
    flag: "u bayrağı",
    acceptable: "kabul edilebilir",
    warning: "uyarı",
    action: "eylem",
    censored: "sansürlü sonuç, skorlanmadı",
    flags_note: "Raporlanan belirsizliklerin bayrakları (ISO 13528 9.8): a — makul, b — u_min'in altında, c — u_max'ın üstünde.",
    homogeneity: "Homojenlik kontrolü (ISO 13528 B.3)",
    items: "numune × tekrar",
    general_mean: "Genel ortalama",
    criterion: "Ölçüt 0,3 σ_pt",
    expanded_criterion: "Genişletilmiş ölçüt √c (B.2.3)",
    sufficient: "yeterli",
    not_sufficient: "yeterli değil",
    u_bb_star: "u*_bb = √(MS_içinde/m)·(2/ν_içinde)^¼ (tekrarlanabilirliğin gizleyebileceği birimler arası s.s.; Linsinger vd. 2001)",
    no_sigma_pt: "σ_pt verilmediğinden ISO 13528'in 0,3 σ_pt ölçütleri değerlendirilmedi.",
    bayes_title: "Bayesçi birimler arası standart sapma (JCGM GUM-5 Örnek 12)",
    mean_col: "ortalama",
    sd_col: "s.s.",
    median_col: "medyan",
    mode_col: "mod",
    q_col: "%95 (%2,5, %97,5)",
    hpd_col: "%95 en kısa",
    bayes_note: "Model y_ij = µ + B_i + ε_ij, B_i ~ N(0, τ²), ε_ij ~ N(0, σ²); τ ve σ için yarı-Cauchy önseller. Birim etkileri ve µ analitik olarak integre edilir; (τ, σ) sonsal dağılımı MCMC kullanılmadan bir ızgarada hesaplanır. µ'nün sonsal dağılımı: {mu}, standart sapma {u}.",
    stability: "Kararlılık kontrolü (ISO 13528 B.5)",
    difference: "Ortalamalar arasındaki fark",
    chart: "z skorları (±2 uyarı, ±3 eylem sınırları)",
    homogeneity_chart: "Homojenlik kontrolü: her numunenin tekrar sonuçları, genel ortalama ve ± 0,3 σ_pt",
    statement: "x_pt = {v} {unit}, U(x_pt) = {U} {unit} (k = {k})",
};

pub(super) fn pt_labels(lang: Lang) -> &'static PtLabels {
    match lang {
        Lang::En => &PT_EN,
        Lang::Tr => &PT_TR,
    }
}

fn signal_text(s: Signal, l: &PtLabels) -> &'static str {
    match s {
        Signal::Acceptable => l.acceptable,
        Signal::Warning => l.warning,
        Signal::Action => l.action,
    }
}

/// The assigned value with its expanded uncertainty, rounded as reported.
pub(super) fn statements(p: &Project, a: &ProficiencyAnalysis, lang: Lang) -> Vec<String> {
    let (Some(spec), Some(x)) = (p.proficiency.as_ref(), a.assigned.as_ref()) else { return Vec::new() };
    let (v, u) = round_reported(x.value, x.expanded, &p.reporting, lang);
    let unit = spec.unit.clone().unwrap_or_default();
    vec![pt_labels(lang)
        .statement
        .replace("{v}", &v)
        .replace("{U}", &u)
        .replace("{unit}", &unit)
        .replace("{k}", &fmt_num(a.coverage_factor, 3, lang))
        .replace("  ", " ")
        .replace(" ,", ",")
        .trim_end()
        .to_string()]
}

pub(super) fn markdown(
    p: &Project,
    a: &ProficiencyAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = pt_labels(lang);
    let Some(spec) = p.proficiency.as_ref() else { return String::new() };
    let unit = spec.unit.clone().map(|u| format!(" {u}")).unwrap_or_default();
    let n = |x: f64| fmt_num(x, 4, lang);
    let sc = |x: Option<f64>| x.map(|v| fmt_num(v, 3, lang)).unwrap_or_else(|| "—".into());
    let fixed2 = |x: Option<f64>| {
        x.map(|v| {
            let t = format!("{v:.2}");
            if lang == Lang::Tr {
                t.replace('.', ",")
            } else {
                t
            }
        })
        .unwrap_or_else(|| "—".into())
    };
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", d.title);
    let _ = writeln!(
        s,
        "{}: {}{}\n",
        d.measurand,
        spec.quantity,
        spec.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default()
    );

    if let Some(sm) = &a.summary {
        let _ = writeln!(s, "## {}\n", d.summary);
        let _ = writeln!(s, "| {} | {} |\n|---|---:|", d.statistic, d.value);
        let _ = writeln!(s, "| {} | {} |", d.participants_n, sm.p);
        let _ = writeln!(s, "| {} | {}{unit} |", d.median, n(sm.median));
        let _ = writeln!(s, "| {} | {}{unit} |", d.made, n(sm.made));
        let _ = writeln!(s, "| {} | {}{unit} |", d.niqr, n(sm.niqr));
        if let Some(r) = &sm.robust {
            let _ = writeln!(s, "| {} | {}{unit} |", d.robust_mean, n(r.mean));
            let _ = writeln!(s, "| {} | {}{unit} ({} {}) |", d.robust_sd, n(r.sd), r.iterations, d.iterations);
        }
        let _ = writeln!(s, "| {} | {}{unit} |", d.mean, n(sm.mean));
        let _ = writeln!(s, "| {} | {}{unit} |\n", d.sd, n(sm.sd));
    }

    if a.assigned.is_some() || a.sigma_pt.is_some() {
        let _ = writeln!(s, "## {}\n", d.assigned);
    }
    if let Some(x) = &a.assigned {
        let method = match x.method {
            AssignedMethod::AlgorithmA => d.m_algorithm_a,
            AssignedMethod::Median => d.m_median,
            AssignedMethod::Mean => d.m_mean,
            AssignedMethod::Reference => d.m_reference,
        };
        let _ = writeln!(s, "- {}: {}{unit} — {method}", d.assigned_value, n(x.value));
        let _ = writeln!(s, "- {}: {}{unit}", d.u_assigned, fmt_num(x.u, 3, lang));
        let _ = writeln!(
            s,
            "- {}: {}{unit} (k = {})",
            d.expanded_assigned,
            fmt_num(x.expanded, 3, lang),
            fmt_num(a.coverage_factor, 3, lang)
        );
    }
    if let (Some(sig), Some(m)) = (a.sigma_pt, a.sd_pt_method) {
        let how = match m {
            SdPtMethod::Robust => d.s_robust,
            SdPtMethod::Made => d.s_made,
            SdPtMethod::Niqr => d.s_niqr,
            SdPtMethod::Sd => d.s_sd,
            SdPtMethod::Given => d.s_given,
        };
        let _ = writeln!(s, "- {}: {}{unit} — {how}", d.sd_pt, fmt_num(sig, 3, lang));
    }
    if let Some(de) = a.delta_e {
        let _ = writeln!(s, "- {}: {}{unit}", d.delta_e, fmt_num(de, 3, lang));
    }
    if let Some(neg) = a.u_negligible {
        let _ = writeln!(s, "\n{}", if neg { d.negligible } else { d.not_negligible });
    }
    let _ = writeln!(s);
    for st in statements(p, a, lang) {
        let _ = writeln!(s, "**{}:** {st}\n", l.reported);
    }

    if !a.participants.is_empty() {
        let _ = writeln!(s, "## {}\n", d.scores);
        let has_u = a.participants.iter().any(|p| p.u.is_some());
        let has_flags = a.participants.iter().any(|p| p.flag.is_some());
        let mut head = format!("| {} | {} |", d.lab, d.result);
        let mut rule = "|---|---:|".to_string();
        if has_u {
            head += &format!(" {} |", d.u_lab);
            rule += "---:|";
        }
        head += " D | D % | PA % | z | z′ |";
        rule += "---:|---:|---:|---:|---:|";
        if has_u {
            head += " ζ | E_n |";
            rule += "---:|---:|";
        }
        head += &format!(" {} |", d.signal);
        rule += "---|";
        if has_flags {
            head += &format!(" {} |", d.flag);
            rule += "---|";
        }
        let _ = writeln!(s, "{head}\n{rule}");
        for part in &a.participants {
            let result = match (part.value, part.censored) {
                (Some(v), _) => fmt_given(v, lang),
                (None, Some((c, v))) => format!("{c} {}", fmt_given(v, lang)),
                _ => "—".into(),
            };
            let mut row = format!("| {} | {result} |", part.lab);
            if has_u {
                row += &format!(" {} |", part.u.map(|u| fmt_num(u, 2, lang)).unwrap_or_else(|| "—".into()));
            }
            match &part.scores {
                Some(x) => {
                    let _ = write!(
                        row,
                        " {} | {} | {} | {} | {} |",
                        sc(Some(x.d)),
                        x.d_percent
                            .map(|v| format!("{v:.1}"))
                            .map(|t| if lang == Lang::Tr { t.replace('.', ",") } else { t })
                            .unwrap_or_else(|| "—".into()),
                        x.pa.map(|v| format!("{v:.1}"))
                            .map(|t| if lang == Lang::Tr { t.replace('.', ",") } else { t })
                            .unwrap_or_else(|| "—".into()),
                        fixed2(x.z),
                        fixed2(x.z_prime)
                    );
                    if has_u {
                        let en =
                            x.en.map(|v| format!("{}{}", fixed2(Some(v)), if en_satisfactory(v) { "" } else { " ⚠" }));
                        let _ = write!(row, " {} | {} |", fixed2(x.zeta), en.unwrap_or_else(|| "—".into()));
                    }
                    let sig = x.z.map(|z| signal_text(z_signal(z), d)).unwrap_or("—");
                    let _ = write!(row, " {sig} |");
                }
                None => {
                    let cols = 5 + if has_u { 2 } else { 0 };
                    row += &" — |".repeat(cols);
                    let _ = write!(row, " {} |", d.censored);
                }
            }
            if has_flags {
                let f = part.flag.map(|f| match f {
                    UncertaintyFlag::Plausible => "a",
                    UncertaintyFlag::BelowMin => "b",
                    UncertaintyFlag::AboveMax => "c",
                });
                let _ = write!(row, " {} |", f.unwrap_or("—"));
            }
            let _ = writeln!(s, "{row}");
        }
        let _ = writeln!(s);
        if has_flags {
            let lim =
                |x: Option<f64>| x.map(|v| format!("{}{unit}", fmt_num(v, 3, lang))).unwrap_or_else(|| "—".into());
            let _ = writeln!(s, "{} u_min = {}, u_max = {}.\n", d.flags_note, lim(a.u_min), lim(a.u_max));
        }
    }

    if let Some(h) = &a.homogeneity {
        let _ = writeln!(s, "## {}\n", d.homogeneity);
        let _ = writeln!(s, "| {} | {} |\n|---|---:|", d.statistic, d.value);
        let _ = writeln!(s, "| {} | {} × {} |", d.items, h.items, h.replicates);
        let _ = writeln!(s, "| {} | {}{unit} |", d.general_mean, fmt_num(h.mean, 5, lang));
        let _ = writeln!(s, "| s_x | {}{unit} |", fmt_num(h.s_x, 3, lang));
        let _ = writeln!(s, "| s_w | {}{unit} |", fmt_num(h.s_w, 3, lang));
        let _ = writeln!(s, "| s_s | {}{unit} |", fmt_num(h.s_s, 3, lang));
        let ok = |b: bool| if b { d.sufficient } else { d.not_sufficient };
        let _ = writeln!(s, "| MS_between | {} |", fmt_num(h.ms_between, 3, lang));
        let _ = writeln!(s, "| MS_within | {} |", fmt_num(h.ms_within, 3, lang));
        let _ = writeln!(s, "| {} | {}{unit} |", d.u_bb_star, fmt_num(h.u_bb_star, 3, lang));
        if let (Some(c), Some(ok1), Some(ce), Some(ok2)) =
            (h.criterion, h.sufficient, h.expanded_criterion, h.sufficient_expanded)
        {
            let _ = writeln!(s, "| {} | {}{unit} — {} |", d.criterion, fmt_num(c, 3, lang), ok(ok1));
            let _ = writeln!(
                s,
                "| {} | {}{unit} — {} (F₁ = {}, F₂ = {}) |",
                d.expanded_criterion,
                fmt_num(ce, 3, lang),
                ok(ok2),
                fmt_num(h.f1, 3, lang),
                fmt_num(h.fm, 3, lang)
            );
        }
        let _ = writeln!(s);
        if h.criterion.is_none() {
            let _ = writeln!(s, "{}\n", d.no_sigma_pt);
        }
        if let Some(b) = &a.between_unit {
            let _ = writeln!(s, "### {}\n", d.bayes_title);
            let _ = writeln!(
                s,
                "| | {} | {} | {} | {} | {} | {} |\n|---|---:|---:|---:|---:|---:|---:|",
                d.mean_col, d.sd_col, d.median_col, d.mode_col, d.q_col, d.hpd_col
            );
            for (name, p) in [("τ", &b.tau), ("σ", &b.sigma)] {
                let n = |x: f64| fmt_num(x, 3, lang);
                let _ = writeln!(
                    s,
                    "| {name} | {}{unit} | {}{unit} | {}{unit} | {}{unit} | [{}, {}]{unit} | [{}, {}]{unit} |",
                    n(p.mean),
                    n(p.sd),
                    n(p.median),
                    n(p.mode),
                    n(p.q025),
                    n(p.q975),
                    n(p.hpd.0),
                    n(p.hpd.1)
                );
            }
            let _ = writeln!(
                s,
                "\n{}\n",
                d.bayes_note.replace("{mu}", &fmt_num(b.mu_mean, 6, lang)).replace("{u}", &fmt_num(b.mu_sd, 3, lang))
            );
        }
    }
    if let Some(st) = &a.stability {
        let _ = writeln!(s, "## {}\n", d.stability);
        let _ = writeln!(
            s,
            "{}: {}{unit}; {}: {}{unit} — {}.\n",
            d.difference,
            fmt_num(st.difference, 3, lang),
            d.criterion,
            fmt_num(st.criterion, 3, lang),
            if st.sufficient { d.sufficient } else { d.not_sufficient }
        );
    }
    tail(&mut s, p, checks, lang, "ISO 13528:2022", generated);
    s
}

/// Bar chart of the z scores with the warning and action limits (SVG).
pub(super) fn z_scores_svg(a: &ProficiencyAnalysis, lang: Lang) -> Option<String> {
    let rows: Vec<(&str, f64)> = a
        .participants
        .iter()
        .filter_map(|p| p.scores.as_ref().and_then(|s| s.z).map(|z| (p.lab.as_str(), z)))
        .collect();
    if rows.is_empty() {
        return None;
    }
    let (w, row_h, left) = (640.0, 20.0, 110.0);
    let h = row_h * rows.len() as f64 + 30.0;
    let zmax = rows.iter().map(|r| r.1.abs()).fold(3.5_f64, f64::max).min(10.0);
    let plot_w = w - left - 20.0;
    let x_of = |z: f64| left + plot_w * (z.clamp(-zmax, zmax) + zmax) / (2.0 * zmax);
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(pt_labels(lang).chart)
    );
    let bottom = h - 20.0;
    for z in [-3.0, -2.0, 0.0, 2.0, 3.0] {
        let x = x_of(z);
        let class = if z == 0.0 {
            "axis"
        } else if z.abs() == 2.0 {
            "gumline"
        } else {
            "mcm"
        };
        let _ = write!(
            s,
            r#"<line x1="{x:.1}" x2="{x:.1}" y1="6" y2="{bottom:.1}" class="{class}"/><text x="{x:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#,
            h - 6.0,
            fmt_num(z, 1, lang)
        );
    }
    for (i, (lab, z)) in rows.iter().enumerate() {
        let y = 8.0 + i as f64 * row_h;
        let (x0, x1) = (x_of(0.0), x_of(*z));
        let _ = write!(
            s,
            r#"<text x="{}" y="{:.1}" text-anchor="end" class="lbl">{}</text><rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" class="bar"/>"#,
            left - 8.0,
            y + row_h * 0.7,
            esc(lab),
            x0.min(x1),
            y + 3.0,
            (x1 - x0).abs().max(0.5),
            row_h - 6.0
        );
    }
    s.push_str("</svg>");
    Some(s)
}

/// Replicate results of the homogeneity check by item, with the general
/// average and the band ± 0.3 σ_pt (SVG).
pub(super) fn homogeneity_svg(
    spec: &crate::proficiency::ProficiencySpec,
    a: &ProficiencyAnalysis,
    lang: Lang,
) -> Option<String> {
    let h = a.homogeneity.as_ref()?;
    let items = &spec.homogeneity.as_ref()?.items;
    let (w, ht, left, top, bottom) = (640.0, 260.0, 70.0, 12.0, 30.0);
    let vals = items.iter().flatten().copied();
    let crit = h.criterion.unwrap_or(0.0);
    let lo = vals.clone().fold(h.mean - crit, f64::min);
    let hi = vals.fold(h.mean + crit, f64::max);
    let pad = 0.08 * (hi - lo).max(1e-300);
    let (lo, hi) = (lo - pad, hi + pad);
    let plot_w = w - left - 16.0;
    let y_of = |v: f64| top + (ht - top - bottom) * (hi - v) / (hi - lo);
    let x_of = |i: usize| left + plot_w * (i as f64 + 0.5) / items.len() as f64;
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {ht}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(pt_labels(lang).homogeneity_chart)
    );
    let (y1, y2) = (y_of(h.mean + crit), y_of(h.mean - crit));
    let _ = write!(
        s,
        r#"<rect x="{left}" y="{y1:.1}" width="{plot_w:.1}" height="{:.1}" class="hist" opacity="0.35"/>"#,
        y2 - y1
    );
    let ym = y_of(h.mean);
    let _ = write!(s, r#"<line x1="{left}" x2="{:.1}" y1="{ym:.1}" y2="{ym:.1}" class="mcm"/>"#, left + plot_w);
    for v in [lo + pad, h.mean, hi - pad] {
        let _ = write!(
            s,
            r#"<text x="{}" y="{:.1}" text-anchor="end" class="val">{}</text>"#,
            left - 6.0,
            y_of(v) + 4.0,
            fmt_num(v, 4, lang)
        );
    }
    for (i, t) in items.iter().enumerate() {
        let x = x_of(i);
        for &v in t {
            let _ = write!(s, r#"<circle cx="{x:.1}" cy="{:.1}" r="3.5" class="bar"/>"#, y_of(v));
        }
        let _ = write!(s, r#"<text x="{x:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#, ht - 10.0, i + 1);
    }
    s.push_str("</svg>");
    Some(s)
}
