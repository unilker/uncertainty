//! Report of a consensus value with degrees of equivalence (NIST Consensus
//! Builder; Koepke et al. 2017).

use super::*;
use crate::consensus::ConsensusAnalysis;

pub(super) struct CsLabels {
    pub inputs: &'static str,
    pub cols: [&'static str; 7],
    pub dl: &'static str,
    pub consensus: &'static str,
    pub u_dl: &'static str,
    pub tau: &'static str,
    pub q: &'static str,
    pub kh: &'static str,
    pub boot_u: &'static str,
    pub boot_int: &'static str,
    pub lp: &'static str,
    pub lp_u: &'static str,
    pub lp_int: &'static str,
    pub ml: &'static str,
    pub bayes: &'static str,
    pub bayes_int: &'static str,
    pub bayes_tau: &'static str,
    pub method_bayes: &'static str,
    pub prior_flat: &'static str,
    pub laplace: &'static str,
    pub prior_normal: &'static str,
    pub statement_bayes: &'static str,
    pub ml_tau: &'static str,
    pub method_ml: &'static str,
    pub statement_ml: &'static str,
    pub doe_note: &'static str,
    pub method_dl: &'static str,
    pub method_lp: &'static str,
    pub chart: &'static str,
    pub statement_dl: &'static str,
    pub statement_lp: &'static str,
    pub statement_tau: &'static str,
    pub heterogeneous: &'static str,
    pub homogeneous: &'static str,
}

const CS_EN: CsLabels = CsLabels {
    inputs: "Measurement results",
    cols: ["Participant", "Value", "u", "ν", "DL weight", "D", "U95 %(D)"],
    dl: "DerSimonian–Laird procedure",
    consensus: "Consensus value",
    u_dl: "Standard uncertainty u_DL = 1/√Σw (τ treated as known)",
    tau: "Dark uncertainty τ (between-participant standard deviation)",
    q: "Cochran's Q (n − 1 = {df}), p = {p}: {verdict}",
    kh: "Knapp–Hartung standard uncertainty (expanded with t_{df}: {U})",
    boot_u: "Standard uncertainty by the parametric bootstrap",
    boot_int: "{p} coverage interval by the parametric bootstrap",
    lp: "Linear Pool",
    lp_u: "Standard uncertainty",
    lp_int: "{p} coverage interval",
    ml: "Maximum likelihood",
    bayes: "Bayesian random-effects model",
    bayes_int: "{p} credible interval (quantiles of the posterior)",
    bayes_tau: "Dark uncertainty τ: posterior median and {p} interval",
    method_bayes: "Gaussian likelihood with known u_j; prior: τ half-Cauchy with median {t}{unit}; μ {mu}. μ is integrated out analytically and the posterior of τ is evaluated on a grid of 10⁵ points uniform in the prior probability, so the result does not depend on a random seed (IUPAC 2024, Examples 4M and 4N, which use MCMC).",
    prior_flat: "flat",
    laplace: "Laplace laboratory effects with standard deviation τ (NIST Decision Tree); μ and τ on a two-dimensional grid. ",
    prior_normal: "Gaussian with mean {m}{unit} and standard deviation {s}{unit}",
    statement_bayes: "{m}: {x}{unit}, standard uncertainty {u}{unit}, {p} credible interval [{lo}, {hi}]{unit}; posterior median of τ {t}{unit} (Bayes)",
    ml_tau: "Dark uncertainty τ",
    method_ml: "Gaussian random-effects model with known u_j; the likelihood is maximised over τ with μ̂(τ) the weighted mean; the standard uncertainties come from the inverse Hessian of the negative log-likelihood (IUPAC 2024, Example 4F). With few participants they tend to be too small; the bootstrap and Bayesian procedures are preferable.",
    statement_ml: "{m}: {x}{unit}, standard uncertainty {u}{unit}; τ = {t}{unit} (maximum likelihood)",
    doe_note: "D = x − μ̂ is the unilateral degree of equivalence of the DerSimonian–Laird procedure; U95 %(D) is the half-length of the shortest interval centred at the mean of the bootstrap values of D that contains 95 % of them (NICOB §A.5.1).",
    method_dl: "Random-effects model x_j = μ + λ_j + ε_j; τ by the method of moments; the bootstrap draws τ from the gamma approximation of the distribution of Q (Biggerstaff and Tweedie 1997), x_j from N(μ̂, τ² + u_j²) and, when ν_j is given, u_j √(ν_j/χ²); {n} replicates (NICOB §A.2).",
    method_lp: "Linear Pool: {n} draws from the mixture of Gaussian (or rescaled Student t) distributions with means x_j and standard deviations u_j (NICOB §A.4).",
    chart: "Unilateral degrees of equivalence D with U95 %(D)",
    statement_dl: "{m}: consensus value {x}{unit}, standard uncertainty {u}{unit}, {p} coverage interval [{lo}, {hi}]{unit} (DerSimonian–Laird with parametric bootstrap)",
    statement_lp: "{m}: {x}{unit}, standard uncertainty {u}{unit}, {p} coverage interval [{lo}, {hi}]{unit} (Linear Pool)",
    statement_tau: "Dark uncertainty τ = {t}{unit}; Cochran's Q = {q} with {df} degrees of freedom",
    heterogeneous: "the results are mutually inconsistent at the 5 % level",
    homogeneous: "no inconsistency is detected at the 5 % level",
};

const CS_TR: CsLabels = CsLabels {
    inputs: "Ölçüm sonuçları",
    cols: ["Katılımcı", "Değer", "u", "ν", "DL ağırlığı", "D", "U95 %(D)"],
    dl: "DerSimonian–Laird yöntemi",
    consensus: "Uzlaşı değeri",
    u_dl: "Standart belirsizlik u_DL = 1/√Σw (τ biliniyor sayılarak)",
    tau: "Karanlık belirsizlik τ (katılımcılar arası standart sapma)",
    q: "Cochran Q (n − 1 = {df}), p = {p}: {verdict}",
    kh: "Knapp–Hartung standart belirsizliği (t_{df} ile genişletilmiş: {U})",
    boot_u: "Parametrik bootstrap ile standart belirsizlik",
    boot_int: "Parametrik bootstrap ile {p} kapsam aralığı",
    lp: "Doğrusal havuz (Linear Pool)",
    lp_u: "Standart belirsizlik",
    lp_int: "{p} kapsam aralığı",
    ml: "En çok olabilirlik",
    bayes: "Bayesçi rastgele etkiler modeli",
    bayes_int: "{p} inanılırlık aralığı (sonsal dağılımın kantilleri)",
    bayes_tau: "Karanlık belirsizlik τ: sonsal ortanca ve {p} aralık",
    method_bayes: "u_j'leri bilinen normal olabilirlik; önsel: τ için ortancası {t}{unit} olan yarım Cauchy; μ için {mu}. μ analitik olarak integre edilir, τ'nun sonsalı önsel olasılıkta düzgün 10⁵ noktalı bir ızgarada hesaplanır; sonuç rastgele tohuma bağlı değildir (IUPAC 2024, Örnek 4M ve 4N MCMC kullanır).",
    prior_flat: "düz",
    laplace: "Standart sapması τ olan Laplace laboratuvar etkileri (NIST Decision Tree); μ ve τ iki boyutlu ızgarada. ",
    prior_normal: "ortalaması {m}{unit}, standart sapması {s}{unit} olan normal",
    statement_bayes: "{m}: {x}{unit}, standart belirsizlik {u}{unit}, {p} inanılırlık aralığı [{lo}, {hi}]{unit}; τ'nun sonsal ortancası {t}{unit} (Bayes)",
    ml_tau: "Karanlık belirsizlik τ",
    method_ml: "u_j'leri bilinen normal rastgele etkiler modeli; olabilirlik τ üzerinden, μ̂(τ) ağırlıklı ortalama olmak üzere en büyüklenir; standart belirsizlikler negatif log-olabilirliğin Hessian matrisinin tersinden gelir (IUPAC 2024, Örnek 4F). Katılımcı sayısı az olduğunda bunlar küçük kalma eğilimindedir; bootstrap ve Bayesçi yöntemler tercih edilir.",
    statement_ml: "{m}: {x}{unit}, standart belirsizlik {u}{unit}; τ = {t}{unit} (en çok olabilirlik)",
    doe_note: "D = x − μ̂, DerSimonian–Laird yönteminin tek taraflı eşdeğerlik derecesidir; U95 %(D), D'nin bootstrap değerlerinin ortalamasında ortalanan ve bunların %95'ini içeren en kısa aralığın yarı uzunluğudur (NICOB §A.5.1).",
    method_dl: "Rastgele etkiler modeli x_j = μ + λ_j + ε_j; τ momentler yöntemiyle; bootstrap τ'yu Q'nun dağılımının gama yaklaşımından (Biggerstaff ve Tweedie 1997), x_j'yi N(μ̂, τ² + u_j²) dağılımından ve ν_j verilmişse u_j'yi u_j √(ν_j/χ²) olarak çeker; {n} tekrar (NICOB §A.2).",
    method_lp: "Doğrusal havuz: ortalamaları x_j, standart sapmaları u_j olan normal (ya da ölçeklenmiş Student t) dağılımların karışımından {n} çekiliş (NICOB §A.4).",
    chart: "Tek taraflı eşdeğerlik dereceleri D ve U95 %(D)",
    statement_dl: "{m}: uzlaşı değeri {x}{unit}, standart belirsizlik {u}{unit}, {p} kapsam aralığı [{lo}, {hi}]{unit} (DerSimonian–Laird, parametrik bootstrap)",
    statement_lp: "{m}: {x}{unit}, standart belirsizlik {u}{unit}, {p} kapsam aralığı [{lo}, {hi}]{unit} (doğrusal havuz)",
    statement_tau: "Karanlık belirsizlik τ = {t}{unit}; Cochran Q = {q}, serbestlik derecesi {df}",
    heterogeneous: "sonuçlar %5 düzeyinde birbiriyle tutarsızdır",
    homogeneous: "%5 düzeyinde tutarsızlık saptanmadı",
};

pub(super) fn cs_labels(lang: Lang) -> &'static CsLabels {
    match lang {
        Lang::En => &CS_EN,
        Lang::Tr => &CS_TR,
    }
}

fn unit_of(a: &ConsensusAnalysis) -> String {
    a.unit.clone().map(|u| format!(" {u}")).unwrap_or_default()
}

/// Significant digits that show two digits of the smallest uncertainty.
fn fx(a: &ConsensusAnalysis, x: f64, lang: Lang) -> String {
    let umin = a.u.iter().copied().fold(f64::INFINITY, f64::min).min(a.dl.u);
    fmt_estimate(x, umin, lang)
}

pub(super) fn statements(a: &ConsensusAnalysis, lang: Lang) -> Vec<String> {
    let d = cs_labels(lang);
    let unit = unit_of(a);
    let m = a.measurand.clone().unwrap_or_else(|| d.consensus.to_string());
    let p = fmt_pct(a.coverage, 3, lang, true);
    let mut out = Vec::new();
    if let Some(b) = &a.bootstrap {
        out.push(
            d.statement_dl
                .replace("{m}", &m)
                .replace("{x}", &fx(a, b.mean, lang))
                .replace("{u}", &fmt_num(b.u, 2, lang))
                .replace("{p}", &p)
                .replace("{lo}", &fx(a, b.interval.0, lang))
                .replace("{hi}", &fx(a, b.interval.1, lang))
                .replace("{unit}", &unit),
        );
        out.push(
            d.statement_tau
                .replace("{t}", &fmt_num(a.dl.tau, 2, lang))
                .replace("{q}", &fmt_num(a.dl.q, 4, lang))
                .replace("{df}", &(a.x.len() - 1).to_string())
                .replace("{unit}", &unit),
        );
    }
    if let Some(b) = &a.linear_pool {
        out.push(
            d.statement_lp
                .replace("{m}", &m)
                .replace("{x}", &fx(a, b.mean, lang))
                .replace("{u}", &fmt_num(b.u, 2, lang))
                .replace("{p}", &p)
                .replace("{lo}", &fx(a, b.interval.0, lang))
                .replace("{hi}", &fx(a, b.interval.1, lang))
                .replace("{unit}", &unit),
        );
    }
    if let Some(b) = &a.bayes {
        out.push(
            d.statement_bayes
                .replace("{m}", &m)
                .replace("{x}", &fx(a, b.mean, lang))
                .replace("{u}", &fmt_num(b.sd, 2, lang))
                .replace("{p}", &p)
                .replace("{lo}", &fx(a, b.interval.0, lang))
                .replace("{hi}", &fx(a, b.interval.1, lang))
                .replace("{t}", &fmt_num(b.tau_median, 2, lang))
                .replace("{unit}", &unit),
        );
    }
    if let Some(ml) = &a.maximum_likelihood {
        out.push(
            d.statement_ml
                .replace("{m}", &m)
                .replace("{x}", &fx(a, ml.mean, lang))
                .replace("{u}", &fmt_num(ml.u_mean, 2, lang))
                .replace("{t}", &format!("{} ± {}", fmt_num(ml.tau, 2, lang), fmt_num(ml.u_tau, 2, lang)))
                .replace("{unit}", &unit),
        );
    }
    out
}

pub(super) fn markdown(
    p: &Project,
    a: &ConsensusAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = cs_labels(lang);
    let unit = unit_of(a);
    let pc = fmt_pct(a.coverage, 3, lang, true);
    let n = a.x.len();
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", d.inputs);
    let doe = a.bootstrap.as_ref().map(|b| &b.degrees_of_equivalence);
    let _ = writeln!(
        s,
        "| {} | {}{unit} | {}{unit} | {} | {} | {}{unit} | {}{unit} |\n|---|---:|---:|---:|---:|---:|---:|",
        d.cols[0], d.cols[1], d.cols[2], d.cols[3], d.cols[4], d.cols[5], d.cols[6]
    );
    for j in 0..n {
        let nu = a.dof.get(j).filter(|v| v.is_finite()).map_or("∞".to_string(), |v| fmt_num(*v, 3, lang));
        let (dd, uu) =
            doe.map_or(("—".into(), "—".into()), |v| (fmt_num(v[j].d, 3, lang), fmt_num(v[j].expanded, 3, lang)));
        let _ = writeln!(
            s,
            "| {} | {} | {} | {nu} | {} | {dd} | {uu} |",
            a.names[j],
            fmt_given(a.x[j], lang),
            fmt_given(a.u[j], lang),
            fmt_pct(a.dl.weights[j], 3, lang, false)
        );
    }
    let _ = writeln!(s);
    if doe.is_some() {
        let _ = writeln!(s, "{}\n", d.doe_note);
    }
    let _ = writeln!(s, "## {}\n", d.dl);
    let df = (n - 1).to_string();
    let verdict = if a.dl.q_p_value < 0.05 { d.heterogeneous } else { d.homogeneous };
    let _ = writeln!(s, "- {}: {}{unit}", d.consensus, fx(a, a.dl.mean, lang));
    let _ = writeln!(s, "- {}: {}{unit}", d.u_dl, fmt_num(a.dl.u, 3, lang));
    let _ = writeln!(s, "- {}: {}{unit}", d.tau, fmt_num(a.dl.tau, 3, lang));
    let _ = writeln!(
        s,
        "- {}",
        d.q.replace("{df}", &df).replace("{p}", &fmt_num(a.dl.q_p_value, 2, lang)).replace("{verdict}", verdict)
    );
    let _ = writeln!(s, "  - Q = {}", fmt_num(a.dl.q, 4, lang));
    let _ = writeln!(
        s,
        "- {}: {}{unit}",
        d.kh.replace("{df}", &df).replace("{U}", &format!("{}{unit}", fmt_num(a.knapp_hartung_expanded, 3, lang))),
        fmt_num(a.dl.u_knapp_hartung, 3, lang)
    );
    if let Some(b) = &a.bootstrap {
        let _ = writeln!(s, "- {}: {}{unit}", d.boot_u, fmt_num(b.u, 3, lang));
        let _ = writeln!(
            s,
            "- {}: [{}, {}]{unit}",
            d.boot_int.replace("{p}", &pc),
            fx(a, b.interval.0, lang),
            fx(a, b.interval.1, lang)
        );
        let _ = writeln!(s, "\n{}\n", d.method_dl.replace("{n}", &b.trials.to_string()));
    } else {
        let _ = writeln!(s);
    }
    if let Some(b) = &a.linear_pool {
        let _ = writeln!(s, "## {}\n", d.lp);
        let _ = writeln!(s, "- {}: {}{unit}", d.consensus, fx(a, b.mean, lang));
        let _ = writeln!(s, "- {}: {}{unit}", d.lp_u, fmt_num(b.u, 3, lang));
        let _ = writeln!(
            s,
            "- {}: [{}, {}]{unit}",
            d.lp_int.replace("{p}", &pc),
            fx(a, b.interval.0, lang),
            fx(a, b.interval.1, lang)
        );
        let _ = writeln!(s, "\n{}\n", d.method_lp.replace("{n}", &b.trials.to_string()));
    }
    if let (Some(b), Some(pr)) = (&a.bayes, &a.bayes_prior) {
        let _ = writeln!(s, "## {}\n", d.bayes);
        let _ = writeln!(s, "- {}: {}{unit}, u = {}{unit}", d.consensus, fx(a, b.mean, lang), fmt_num(b.sd, 3, lang));
        let _ = writeln!(
            s,
            "- {}: [{}, {}]{unit}",
            d.bayes_int.replace("{p}", &pc),
            fx(a, b.interval.0, lang),
            fx(a, b.interval.1, lang)
        );
        let _ = writeln!(
            s,
            "- {}: {}{unit} [{}, {}]{unit}",
            d.bayes_tau.replace("{p}", &pc),
            fmt_num(b.tau_median, 3, lang),
            fmt_num(b.tau_interval.0, 3, lang),
            fmt_num(b.tau_interval.1, 3, lang)
        );
        let mu = match (pr.mu_mean, pr.mu_sd) {
            (Some(m0), Some(s0)) => d
                .prior_normal
                .replace("{m}", &fmt_given(m0, lang))
                .replace("{s}", &fmt_given(s0, lang))
                .replace("{unit}", &unit),
            _ => d.prior_flat.to_string(),
        };
        let lap = if pr.effects == crate::consensus::Effects::Laplace { d.laplace } else { "" };
        let _ = writeln!(
            s,
            "\n{lap}{}\n",
            d.method_bayes
                .replace("{t}", &fmt_num(pr.tau_median.unwrap_or(f64::NAN), 4, lang))
                .replace("{mu}", &mu)
                .replace("{unit}", &unit)
        );
    }
    if let Some(ml) = &a.maximum_likelihood {
        let _ = writeln!(s, "## {}\n", d.ml);
        let _ =
            writeln!(s, "- {}: {}{unit}, u = {}{unit}", d.consensus, fx(a, ml.mean, lang), fmt_num(ml.u_mean, 3, lang));
        let _ =
            writeln!(s, "- {}: {}{unit}, u = {}{unit}", d.ml_tau, fmt_num(ml.tau, 3, lang), fmt_num(ml.u_tau, 3, lang));
        let _ = writeln!(s, "\n{}\n", d.method_ml);
    }
    tail(
        &mut s,
        p,
        checks,
        lang,
        "NIST Consensus Builder (2020) App. A; Koepke et al., Metrologia 54 (2017) S34",
        generated,
    );
    s
}

/// Dot-and-bar chart of the unilateral degrees of equivalence (SVG).
pub(super) fn chart_svg(a: &ConsensusAnalysis, lang: Lang) -> Option<String> {
    let doe = &a.bootstrap.as_ref()?.degrees_of_equivalence;
    let (w, row_h, left) = (640.0, 20.0, 110.0);
    let h = row_h * doe.len() as f64 + 30.0;
    let lim = doe.iter().map(|d| d.d.abs() + d.expanded).fold(0.0_f64, f64::max) * 1.05;
    if lim.is_nan() || lim <= 0.0 {
        return None;
    }
    let plot_w = w - left - 20.0;
    let x_of = |v: f64| left + plot_w * (v + lim) / (2.0 * lim);
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(cs_labels(lang).chart)
    );
    let bottom = h - 20.0;
    for v in [-lim / 1.05, 0.0, lim / 1.05] {
        let x = x_of(v);
        let class = if v == 0.0 { "axis" } else { "mcm" };
        let _ = write!(
            s,
            r#"<line x1="{x:.1}" x2="{x:.1}" y1="6" y2="{bottom:.1}" class="{class}"/><text x="{x:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#,
            h - 6.0,
            fmt_num(v, 2, lang)
        );
    }
    for (j, dj) in doe.iter().enumerate() {
        let y = 8.0 + j as f64 * row_h + row_h * 0.5;
        let _ = write!(
            s,
            r#"<text x="{}" y="{:.1}" text-anchor="end" class="lbl">{}</text><line x1="{:.1}" x2="{:.1}" y1="{y:.1}" y2="{y:.1}" class="gumline"/><circle cx="{:.1}" cy="{y:.1}" r="3.5" class="bar"/>"#,
            left - 8.0,
            y + 4.0,
            esc(&a.names[j]),
            x_of(dj.d - dj.expanded),
            x_of(dj.d + dj.expanded),
            x_of(dj.d)
        );
    }
    s.push_str("</svg>");
    Some(s)
}
