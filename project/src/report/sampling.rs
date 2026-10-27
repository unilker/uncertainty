//! Reports of the uncertainty arising from sampling (Eurachem UfS:2019).

use super::*;
use crate::sampling::{DuplicateAnalysis, Estimate, GyAnalysis, MethodSpec, SamplingAnalysis, SamplingResult};
use uncertainty_core::sampling::{relative_from_log, uncertainty_factor, NestedAnova};

pub(super) struct SpLabels {
    pub title: &'static str,
    pub quantity: &'static str,
    pub design: &'static str,
    pub balanced: &'static str,
    pub unbalanced: &'static str,
    pub calculation: &'static str,
    pub methods: [&'static str; 2],
    pub mean: &'static str,
    pub coverage: &'static str,
    pub data: &'static str,
    pub target: &'static str,
    pub anova: &'static str,
    pub source: &'static str,
    pub levels: [&'static str; 3],
    pub residual_ms: &'static str,
    pub ems: &'static str,
    pub range: &'static str,
    pub range_rows: [&'static str; 7],
    pub results: &'static str,
    pub level_rows: [&'static str; 5],
    pub rsd: &'static str,
    pub expanded: &'static str,
    pub variance: &'static str,
    pub negative: &'static str,
    pub robust: &'static str,
    pub log: &'static str,
    pub log_text: &'static str,
    pub factor: &'static str,
    pub geometric: &'static str,
    pub u_from_log: &'static str,
    pub u_large: &'static str,
    pub gy: &'static str,
    pub gy_text: &'static str,
    pub material: &'static str,
    pub material_rows: [&'static str; 5],
    pub step: &'static str,
    pub step_cols: [&'static str; 8],
    pub analytical: &'static str,
    pub sampling_total: &'static str,
    pub total: &'static str,
    pub chart: &'static str,
    pub statement_dup: &'static str,
    pub statement_log: &'static str,
    pub statement_gy: &'static str,
}

const SP_EN: SpLabels = SpLabels {
    title: "Uncertainty arising from sampling (Eurachem UfS:2019)",
    quantity: "Analyte and sampling target",
    design: "Design",
    balanced: "duplicate method, balanced design: {t} sampling targets, two samples each with two analyses",
    unbalanced: "duplicate method: {t} sampling targets, {s} samples, {n} analyses (unbalanced design)",
    calculation: "Calculation",
    methods: ["classical nested analysis of variance", "range method (Example A3)"],
    mean: "Mean of all results",
    coverage: "Coverage factor",
    data: "Measured values",
    target: "Target",
    anova: "Analysis of variance",
    source: "Source of variation",
    levels: ["Between targets", "Sampling (between samples)", "Analysis (between analyses)"],
    residual_ms: "Mean square",
    ems: "Unbalanced design: E(MS_samp) = σ_A² + {k1}·σ_S², E(MS_target) = σ_A² + {k2}·σ_S² + {k3}·σ_T² (method of moments).",
    range: "Range calculations",
    range_rows: [
        "Mean analytical range R̄_A = mean of |S1A1 − S1A2| and |S2A1 − S2A2|",
        "s_A = R̄_A / d₂",
        "Mean range of the sample means R̄_S = mean |x̄_S1 − x̄_S2|",
        "s_S+A = R̄_S / d₂",
        "s_S = √(s_S+A² − s_A²/2)",
        "Standard deviation of the target means s_T+S+A",
        "s_T = √(s_T+S+A² − s_S+A²/2)",
    ],
    results: "Standard deviations and relative uncertainties",
    level_rows: ["Between targets s_T", "Sampling s_samp", "Analysis s_anal", "Measurement s_meas = √(s_samp² + s_anal²)", "Total"],
    rsd: "Relative s",
    expanded: "U′ (k = {k})",
    variance: "Share of total variance",
    negative: "The estimate of the {level} variance is negative ({v}) and is set to zero (the variance is small compared with the variances below it).",
    robust: "The robust ANOVA of the programs RANOVA/RANOVA2 used in the Guide is not implemented: its algorithm is not given in the Guide. Outlying values affect the classical estimates.",
    log: "Log-transformed values (uncertainty factor)",
    log_text: "Classical ANOVA of the natural logarithms. The expanded uncertainty factor F_U = exp(k·s_G) gives the interval x/F_U to x·F_U (Equation 5, Section 9.5.3).",
    factor: "F_U",
    geometric: "Mean of ln x {m}; geometric mean {g}{unit}.",
    u_from_log: "Relative standard uncertainty of the measurement u′ = √(exp(s_G²) − 1) = {u}.",
    u_large: "u′ exceeds 0.2: the expanded uncertainty should be expressed by the uncertainty factor rather than by k·u′ (Section 9.5.3).",
    gy: "Fundamental sampling error (Gy's sampling theory)",
    gy_text: "Relative variance of each step s_r² = C·d³·(1/M_S − 1/M_L) with C = f·g·β·c (Section 10.2.8, Equations 6–8); the components are combined by the sum of variances.",
    material: "Material",
    material_rows: [
        "Mean concentration in the lot a_L",
        "Concentration in the critical particles α",
        "Density of the critical particles ρ_c",
        "Density of the matrix particles ρ_m",
        "Constitution factor c = (1 − a_L/α)²/(a_L/α)·ρ_c + (1 − a_L/α)·ρ_m",
    ],
    step: "Sampling steps",
    step_cols: ["Step", "M_S", "M_L", "d", "g", "f", "β", "C"],
    analytical: "Analytical determination",
    sampling_total: "Sampling (combined)",
    total: "Total",
    chart: "Shares of the total variance",
    statement_dup: "s_samp = {s}, s_anal = {a}, s_meas = {m}{unit} at the mean {x}{unit}: U′ = {us} (sampling), {ua} (analysis), {um} (measurement), k = {k}; between-target s_T = {t}{unit}",
    statement_log: "ln x: s_G,samp = {s}, s_G,anal = {a}, s_G,meas = {m}; uncertainty factors F_U = {fs} (sampling), {fa} (analysis), {fm} (measurement)",
    statement_gy: "fundamental sampling error: {steps}; sampling s_r = {s}, total s_r = {t}; U′ = {us} (sampling), {ut} (total), k = {k}",
};

const SP_TR: SpLabels = SpLabels {
    title: "Numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)",
    quantity: "Analit ve numune alma hedefi",
    design: "Tasarım",
    balanced: "çift numune yöntemi, dengeli tasarım: {t} numune alma hedefi, her birinden iki analizli iki numune",
    unbalanced: "çift numune yöntemi: {t} numune alma hedefi, {s} numune, {n} analiz (dengesiz tasarım)",
    calculation: "Hesaplama",
    methods: ["klasik iç içe varyans analizi", "aralık yöntemi (Örnek A3)"],
    mean: "Tüm sonuçların ortalaması",
    coverage: "Kapsam faktörü",
    data: "Ölçülen değerler",
    target: "Hedef",
    anova: "Varyans analizi",
    source: "Değişkenlik kaynağı",
    levels: ["Hedefler arası", "Numune alma (numuneler arası)", "Analiz (analizler arası)"],
    residual_ms: "Kareler ortalaması",
    ems: "Dengesiz tasarım: E(KO_numune) = σ_A² + {k1}·σ_S², E(KO_hedef) = σ_A² + {k2}·σ_S² + {k3}·σ_T² (momentler yöntemi).",
    range: "Aralık hesapları",
    range_rows: [
        "Ortalama analiz aralığı R̄_A = |S1A1 − S1A2| ve |S2A1 − S2A2| ortalaması",
        "s_A = R̄_A / d₂",
        "Numune ortalamalarının ortalama aralığı R̄_S = |x̄_S1 − x̄_S2| ortalaması",
        "s_S+A = R̄_S / d₂",
        "s_S = √(s_S+A² − s_A²/2)",
        "Hedef ortalamalarının standart sapması s_T+S+A",
        "s_T = √(s_T+S+A² − s_S+A²/2)",
    ],
    results: "Standart sapmalar ve bağıl belirsizlikler",
    level_rows: [
        "Hedefler arası s_T",
        "Numune alma s_numune",
        "Analiz s_analiz",
        "Ölçüm s_ölçüm = √(s_numune² + s_analiz²)",
        "Toplam",
    ],
    rsd: "Bağıl s",
    expanded: "U′ (k = {k})",
    variance: "Toplam varyanstaki pay",
    negative: "{level} varyansı tahmini negatiftir ({v}) ve sıfır alınmıştır (varyans, alt düzeylerin varyansına göre küçüktür).",
    robust: "Kılavuzda kullanılan RANOVA/RANOVA2 programlarının robust ANOVA'sı uygulanmamıştır: algoritması kılavuzda verilmemiştir. Aykırı değerler klasik tahminleri etkiler.",
    log: "Logaritması alınmış değerler (belirsizlik faktörü)",
    log_text: "Doğal logaritmaların klasik ANOVA'sı. Genişletilmiş belirsizlik faktörü F_U = exp(k·s_G), x/F_U ile x·F_U aralığını verir (Eşitlik 5, Bölüm 9.5.3).",
    factor: "F_U",
    geometric: "ln x ortalaması {m}; geometrik ortalama {g}{unit}.",
    u_from_log: "Ölçümün bağıl standart belirsizliği u′ = √(exp(s_G²) − 1) = {u}.",
    u_large: "u′ 0,2'yi aşıyor: genişletilmiş belirsizlik k·u′ yerine belirsizlik faktörüyle ifade edilmelidir (Bölüm 9.5.3).",
    gy: "Temel numune alma hatası (Gy numune alma kuramı)",
    gy_text: "Her adımın bağıl varyansı s_r² = C·d³·(1/M_S − 1/M_L), C = f·g·β·c (Bölüm 10.2.8, Eşitlik 6–8); bileşenler varyansların toplamıyla birleştirilir.",
    material: "Malzeme",
    material_rows: [
        "Partideki ortalama derişim a_L",
        "Kritik taneciklerdeki derişim α",
        "Kritik taneciklerin yoğunluğu ρ_c",
        "Matris taneciklerinin yoğunluğu ρ_m",
        "Bileşim faktörü c = (1 − a_L/α)²/(a_L/α)·ρ_c + (1 − a_L/α)·ρ_m",
    ],
    step: "Numune alma adımları",
    step_cols: ["Adım", "M_S", "M_L", "d", "g", "f", "β", "C"],
    analytical: "Analitik tayin",
    sampling_total: "Numune alma (birleşik)",
    total: "Toplam",
    chart: "Toplam varyanstaki paylar",
    statement_dup: "s_numune = {s}, s_analiz = {a}, s_ölçüm = {m}{unit}, ortalama {x}{unit}: U′ = {us} (numune alma), {ua} (analiz), {um} (ölçüm), k = {k}; hedefler arası s_T = {t}{unit}",
    statement_log: "ln x: s_G,numune = {s}, s_G,analiz = {a}, s_G,ölçüm = {m}; belirsizlik faktörleri F_U = {fs} (numune alma), {fa} (analiz), {fm} (ölçüm)",
    statement_gy: "temel numune alma hatası: {steps}; numune alma s_r = {s}, toplam s_r = {t}; U′ = {us} (numune alma), {ut} (toplam), k = {k}",
};

pub(super) fn sp_labels(lang: Lang) -> &'static SpLabels {
    match lang {
        Lang::En => &SP_EN,
        Lang::Tr => &SP_TR,
    }
}

fn unit_of(a: &SamplingAnalysis) -> String {
    a.unit.clone().map(|u| format!(" {u}")).unwrap_or_default()
}

fn pct(x: f64, lang: Lang) -> String {
    fmt_pct(x / 100.0, 3, lang, false)
}

fn label(a: &SamplingAnalysis, i: usize) -> String {
    a.labels.get(i).cloned().unwrap_or_else(|| (i + 1).to_string())
}

pub(super) fn statements(a: &SamplingAnalysis, lang: Lang) -> Vec<String> {
    let d = sp_labels(lang);
    let unit = unit_of(a);
    let k = a.coverage_factor;
    let n4 = |x: f64| fmt_num(x, 4, lang);
    match &a.result {
        SamplingResult::Duplicate(dup) => {
            let e = &dup.estimate;
            let (m, sd) = (e.mean(), e.sd());
            let rel = |s: f64| pct(100.0 * k * s / m, lang);
            let mut out = vec![d
                .statement_dup
                .replace("{s}", &n4(sd[1]))
                .replace("{a}", &n4(sd[2]))
                .replace("{m}", &n4(e.measurement_sd()))
                .replace("{x}", &n4(m))
                .replace("{us}", &rel(sd[1]))
                .replace("{ua}", &rel(sd[2]))
                .replace("{um}", &rel(e.measurement_sd()))
                .replace("{t}", &n4(sd[0]))
                .replace("{k}", &fmt_given(k, lang))
                .replace("{unit}", &unit)];
            if let Some(g) = &dup.log {
                out.push(
                    d.statement_log
                        .replace("{s}", &n4(g.sd[1]))
                        .replace("{a}", &n4(g.sd[2]))
                        .replace("{m}", &n4(g.measurement_sd()))
                        .replace("{fs}", &n4(uncertainty_factor(g.sd[1], k)))
                        .replace("{fa}", &n4(uncertainty_factor(g.sd[2], k)))
                        .replace("{fm}", &n4(uncertainty_factor(g.measurement_sd(), k))),
                );
            }
            out
        }
        SamplingResult::Gy(g) => {
            let steps: Vec<String> = g.steps.iter().map(|s| format!("{} s_r = {}", s.name, n4(s.rsd))).collect();
            vec![d
                .statement_gy
                .replace("{steps}", &steps.join(", "))
                .replace("{s}", &n4(g.rsd_sampling()))
                .replace("{t}", &n4(g.rsd_total()))
                .replace("{us}", &pct(100.0 * k * g.rsd_sampling(), lang))
                .replace("{ut}", &pct(100.0 * k * g.rsd_total(), lang))
                .replace("{k}", &fmt_given(k, lang))]
        }
    }
}

pub(super) fn markdown(
    p: &Project,
    a: &SamplingAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = sp_labels(lang);
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", d.title);
    match &a.result {
        SamplingResult::Duplicate(dup) => duplicate_markdown(&mut s, a, dup, lang),
        SamplingResult::Gy(g) => gy_markdown(&mut s, a, g, lang),
    }
    tail(&mut s, p, checks, lang, "Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019", generated);
    s
}

fn duplicate_markdown(s: &mut String, a: &SamplingAnalysis, dup: &DuplicateAnalysis, lang: Lang) {
    let d = sp_labels(lang);
    let unit = unit_of(a);
    let unit_b = a.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default();
    let k = a.coverage_factor;
    let e = &dup.estimate;
    let mean = e.mean();
    let n = |x: f64| fmt_num(x, 5, lang);
    let samples: usize = dup.targets.iter().map(|t| t.len()).sum();
    let analyses: usize = dup.targets.iter().flatten().map(|x| x.len()).sum();
    let design = if dup.balanced { d.balanced } else { d.unbalanced }
        .replace("{t}", &dup.targets.len().to_string())
        .replace("{s}", &samples.to_string())
        .replace("{n}", &analyses.to_string());
    let _ = writeln!(s, "| | |\n|---|---|");
    let _ = writeln!(s, "| {} | {}{unit_b} |", d.quantity, a.quantity);
    let _ = writeln!(s, "| {} | {design} |", d.design);
    let method = match dup.method {
        MethodSpec::Anova => d.methods[0],
        MethodSpec::Range => d.methods[1],
    };
    let _ = writeln!(s, "| {} | {method} |", d.calculation);
    let _ = writeln!(s, "| {} | {}{unit} |", d.mean, n(mean));
    let _ = writeln!(s, "| {} | {} |\n", d.coverage, fmt_given(k, lang));

    let _ = writeln!(s, "### {}\n", d.data);
    if dup.balanced {
        let _ = writeln!(s, "| {} | S1A1 | S1A2 | S2A1 | S2A2 |\n|---|---:|---:|---:|---:|", d.target);
        for (i, t) in dup.targets.iter().enumerate() {
            let v: Vec<String> = t.iter().flatten().map(|&x| fmt_given(x, lang)).collect();
            let _ = writeln!(s, "| {} | {} |", label(a, i), v.join(" | "));
        }
    } else {
        let _ = writeln!(s, "| {} | S1 | S2 | … |\n|---|---|---|---|", d.target);
        for (i, t) in dup.targets.iter().enumerate() {
            let v: Vec<String> =
                t.iter().map(|x| x.iter().map(|&y| fmt_given(y, lang)).collect::<Vec<_>>().join("; ")).collect();
            let _ = writeln!(s, "| {} | {} |", label(a, i), v.join(" | "));
        }
    }
    let _ = writeln!(s);

    match e {
        Estimate::Anova(x) => anova_table(s, x, &unit, lang),
        Estimate::Range(r) => {
            let _ = writeln!(s, "### {}\n", d.range);
            let _ = writeln!(s, "| | |\n|---|---:|");
            let vals = [
                r.mean_range_analysis,
                r.s_analysis,
                r.mean_range_samples,
                r.s_samples,
                r.sd[1],
                r.sd_target_means,
                r.sd[0],
            ];
            for (row, v) in d.range_rows.iter().zip(vals) {
                let _ = writeln!(s, "| {row} | {}{unit} |", n(v));
            }
            let _ = writeln!(s, "\nd₂ = {}.\n", fmt_num(r.d2, 5, lang));
        }
    }

    let _ = writeln!(s, "### {}\n", d.results);
    let exp = d.expanded.replace("{k}", &fmt_given(k, lang));
    let _ = writeln!(s, "| | s | {} | {exp} | {} |\n|---|---:|---:|---:|---:|", d.rsd, d.variance);
    let sd = e.sd();
    let pc = e.percent();
    let rows = [
        (sd[0], Some(pc[0])),
        (sd[1], Some(pc[1])),
        (sd[2], Some(pc[2])),
        (e.measurement_sd(), Some(pc[1] + pc[2])),
        (e.total_sd(), None),
    ];
    for (name, (v, share)) in d.level_rows.iter().zip(rows) {
        // The total includes the between-target variance: no U′ for it.
        let expanded = if share.is_some() { pct(100.0 * k * v / mean, lang) } else { String::new() };
        let _ = writeln!(
            s,
            "| {name} | {}{unit} | {} | {expanded} | {} |",
            n(v),
            pct(100.0 * v / mean, lang),
            share.map(|x| pct(x, lang)).unwrap_or_default()
        );
    }
    let _ = writeln!(s);
    let raw = e.variance_raw();
    for (lv, name) in [(0, d.levels[0]), (1, d.levels[1])] {
        if raw[lv] < 0.0 {
            let _ = writeln!(s, "{}\n", d.negative.replace("{level}", name).replace("{v}", &fmt_num(raw[lv], 5, lang)));
        }
    }
    let _ = writeln!(s, "{}\n", d.robust);

    if let Some(g) = &dup.log {
        let _ = writeln!(s, "### {}\n", d.log);
        let _ = writeln!(s, "{}\n", d.log_text);
        anova_table(s, g, "", lang);
        let _ = writeln!(s, "| | s_G | {} |\n|---|---:|---:|", d.factor);
        let rows = [(0usize, g.sd[0]), (1, g.sd[1]), (2, g.sd[2]), (3, g.measurement_sd()), (4, g.total_sd())];
        for (i, v) in rows {
            let f = if (1..=3).contains(&i) { fmt_num(uncertainty_factor(v, k), 5, lang) } else { String::new() };
            let _ = writeln!(s, "| {} | {} | {f} |", d.level_rows[i], fmt_num(v, 5, lang));
        }
        let _ = writeln!(s);
        let _ = writeln!(
            s,
            "{}",
            d.geometric
                .replace("{m}", &fmt_num(g.mean, 5, lang))
                .replace("{g}", &fmt_num(g.mean.exp(), 5, lang))
                .replace("{unit}", &unit)
        );
        let u = relative_from_log(g.measurement_sd());
        let _ = writeln!(s, "{}\n", d.u_from_log.replace("{u}", &fmt_num(u, 4, lang)));
        if u > 0.2 {
            let _ = writeln!(s, "{}\n", d.u_large);
        }
    }
}

fn anova_table(s: &mut String, x: &NestedAnova, unit: &str, lang: Lang) {
    let d = sp_labels(lang);
    let l = labels(lang);
    let _ = writeln!(s, "### {}{}\n", d.anova, if unit.is_empty() { " (ln x)" } else { "" });
    let _ = writeln!(s, "| {} | SS | ν | {} |\n|---|---:|---:|---:|", d.source, d.residual_ms);
    for i in 0..3 {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} |",
            d.levels[i],
            fmt_num(x.ss[i], 6, lang),
            fmt_given(x.dof[i], lang),
            fmt_num(x.ms[i], 6, lang)
        );
    }
    let _ = writeln!(s);
    let balanced = (x.k1 - 2.0).abs() < 1e-12 && (x.k2 - 2.0).abs() < 1e-12 && (x.k3 - 4.0).abs() < 1e-12;
    if !balanced {
        let _ = writeln!(
            s,
            "{}\n",
            d.ems
                .replace("{k1}", &fmt_num(x.k1, 4, lang))
                .replace("{k2}", &fmt_num(x.k2, 4, lang))
                .replace("{k3}", &fmt_num(x.k3, 4, lang))
        );
    }
    let _ = l;
}

fn gy_markdown(s: &mut String, a: &SamplingAnalysis, g: &GyAnalysis, lang: Lang) {
    let d = sp_labels(lang);
    let k = a.coverage_factor;
    let n = |x: f64| fmt_num(x, 4, lang);
    let _ = writeln!(s, "| | |\n|---|---|");
    let unit_b = a.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default();
    let _ = writeln!(s, "| {} | {}{unit_b} |", d.quantity, a.quantity);
    let _ = writeln!(s, "| {} | {} |\n", d.coverage, fmt_given(k, lang));
    let _ = writeln!(s, "### {}\n", d.gy);
    let _ = writeln!(s, "{}\n", d.gy_text);
    let _ = writeln!(s, "#### {}\n", d.material);
    let _ = writeln!(s, "| | |\n|---|---:|");
    let m = &g.material;
    let vals = [
        fmt_given(m.lot_concentration, lang),
        fmt_given(m.particle_concentration, lang),
        format!("{} g cm⁻³", fmt_given(m.critical_density, lang)),
        format!("{} g cm⁻³", fmt_given(m.matrix_density, lang)),
        format!("{} g cm⁻³", n(g.constitution_factor)),
    ];
    for (row, v) in d.material_rows.iter().zip(vals) {
        let _ = writeln!(s, "| {row} | {v} |");
    }
    let _ = writeln!(s, "\n#### {}\n", d.step);
    let c = &d.step_cols;
    let _ = writeln!(
        s,
        "| {} | {} [g] | {} [g] | {} [cm] | {} | {} | {} | {} [g cm⁻³] | s_r |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|",
        c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]
    );
    for st in &g.steps {
        let x = &st.step;
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            st.name,
            fmt_given(x.sample_mass, lang),
            fmt_given(x.lot_mass, lang),
            fmt_given(x.particle_size, lang),
            fmt_given(x.size_factor, lang),
            fmt_given(x.shape_factor, lang),
            fmt_given(x.liberation, lang),
            n(st.sampling_constant),
            n(st.rsd)
        );
    }
    let _ = writeln!(s);
    let exp = d.expanded.replace("{k}", &fmt_given(k, lang));
    let _ = writeln!(s, "| | s_r | {exp} | {} |\n|---|---:|---:|---:|", d.variance);
    let total = g.rsd_total().powi(2);
    let share = |v: f64| if total > 0.0 { pct(100.0 * v * v / total, lang) } else { String::new() };
    for st in &g.steps {
        let _ = writeln!(s, "| {} | {} | {} | {} |", st.name, n(st.rsd), pct(100.0 * k * st.rsd, lang), share(st.rsd));
    }
    let _ = writeln!(
        s,
        "| {} | {} | {} | {} |",
        d.sampling_total,
        n(g.rsd_sampling()),
        pct(100.0 * k * g.rsd_sampling(), lang),
        share(g.rsd_sampling())
    );
    if let Some(an) = g.analytical_rsd {
        let _ = writeln!(s, "| {} | {} | {} | {} |", d.analytical, n(an), pct(100.0 * k * an, lang), share(an));
    }
    let _ = writeln!(s, "| {} | {} | {} | |\n", d.total, n(g.rsd_total()), pct(100.0 * k * g.rsd_total(), lang));
}

/// Bar chart of the shares of the total variance.
pub(super) fn chart_svg(a: &SamplingAnalysis, lang: Lang) -> Option<String> {
    let d = sp_labels(lang);
    match &a.result {
        SamplingResult::Duplicate(dup) => {
            let p = dup.estimate.percent();
            let rows: Vec<(&str, f64)> = d.levels.iter().zip(p).map(|(&n, v)| (n, v / 100.0)).collect();
            contributions_svg(&rows, lang)
        }
        SamplingResult::Gy(g) => {
            let total = g.rsd_total().powi(2);
            if total <= 0.0 {
                return None;
            }
            let mut rows: Vec<(&str, f64)> = g.steps.iter().map(|s| (s.name.as_str(), s.rsd * s.rsd / total)).collect();
            if let Some(an) = g.analytical_rsd {
                rows.push((d.analytical, an * an / total));
            }
            contributions_svg(&rows, lang)
        }
    }
}
