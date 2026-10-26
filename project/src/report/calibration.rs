//! Reports of calibration projects with uncertainties in both variables
//! (ISO 6143:2025).

use super::*;
use crate::calibration::{CalibrationAnalysis, FunctionSpec};

pub(super) struct CalLabels {
    pub title: &'static str,
    pub analyte: &'static str,
    pub data: &'static str,
    pub point: &'static str,
    pub function: &'static str,
    pub parameter: &'static str,
    pub estimate: &'static str,
    pub correlation: &'static str,
    pub s_res: &'static str,
    pub s_res_limit: &'static str,
    pub gamma: &'static str,
    pub admissible: &'static str,
    pub not_admissible: &'static str,
    pub compare: &'static str,
    pub parameters_n: &'static str,
    pub not_fitted: &'static str,
    pub chosen: &'static str,
    pub samples: &'static str,
    pub sample: &'static str,
    pub out_of_range: &'static str,
    pub sample_correlations: &'static str,
    pub bound: &'static str,
    pub bound_text: &'static str,
    pub target_met: &'static str,
    pub target_not_met: &'static str,
    pub drift: &'static str,
    pub drift_text: &'static str,
    pub drift_rows: [&'static str; 3],
    pub difference: &'static str,
    pub critical: &'static str,
    pub passed: &'static str,
    pub failed: &'static str,
    pub validation: &'static str,
    pub x_obs: &'static str,
    pub x_ref: &'static str,
    pub limit: &'static str,
    pub chart: &'static str,
    pub method_note: &'static str,
    pub names: [&'static str; 5],
}

const CAL_EN: CalLabels = CalLabels {
    title: "Calibration with uncertainties in both variables (ISO 6143)",
    analyte: "Analyte",
    data: "Calibration data and weighted deviations",
    point: "Reference mixture",
    function: "Analysis function",
    parameter: "Parameter",
    estimate: "Estimate",
    correlation: "Correlation coefficients of the parameters",
    s_res: "Residual sum of squared weighted deviations S_res",
    s_res_limit: "acceptance limit",
    gamma: "Goodness of fit Γ (largest weighted deviation)",
    admissible: "Γ ≤ 2: the analysis function is compatible with the calibration data (ISO 6143 6.2.2).",
    not_admissible: "Γ > 2: the analysis function is not compatible with the calibration data; try another function or revise the data (ISO 6143 6.2.2).",
    compare: "Comparison of analysis functions",
    parameters_n: "parameters",
    not_fitted: "not fitted",
    chosen: "selected",
    samples: "Contents of the analysed mixtures",
    sample: "Mixture",
    out_of_range: "outside the calibration range",
    sample_correlations: "Correlation coefficients of the results (through the common calibration)",
    bound: "Upper bound of the uncertainty (ISO 6143 6.2.3)",
    bound_text: "u(x) at the lowest and highest calibration responses: {lo} and {hi}; upper bound {max}.",
    target_met: "The upper bound does not exceed the acceptable standard uncertainty {t}.",
    target_not_met: "The upper bound exceeds the acceptable standard uncertainty {t}.",
    drift: "Drift control (ISO 6143 6.2.4)",
    drift_text: "Reference mixture {r}, y(calib) = {y}, n = {n} measurements before and after.",
    drift_rows: ["|ȳ(before) − y(calib)|", "|y(calib) − ȳ(after)|", "|ȳ(before) − ȳ(after)|"],
    difference: "Difference",
    critical: "Critical value",
    passed: "passed",
    failed: "failed",
    validation: "Validation with mismatching reference mixtures (ISO 6143 6.2.5, Formula 13)",
    x_obs: "x_obs",
    x_ref: "x_ref",
    limit: "2√(u²(x_obs) + u²(x_ref))",
    chart: "Weighted deviations of the calibration points (● content, ■ response) with the limits ±2",
    method_note: "Generalized least squares with uncertainties in both variables (ISO 6143 Annex A); covariance matrix of the parameters by propagation of the uncertainties of all calibration data through the fit.",
    names: ["linear", "quadratic", "cubic", "power", "exponential"],
};

const CAL_TR: CalLabels = CalLabels {
    title: "Her iki değişkende belirsizlikli kalibrasyon (ISO 6143)",
    analyte: "Analit",
    data: "Kalibrasyon verileri ve ağırlıklı sapmalar",
    point: "Referans karışım",
    function: "Analiz fonksiyonu",
    parameter: "Parametre",
    estimate: "Tahmin",
    correlation: "Parametrelerin korelasyon katsayıları",
    s_res: "Ağırlıklı sapma karelerinin artık toplamı S_res",
    s_res_limit: "kabul sınırı",
    gamma: "Uyum iyiliği Γ (en büyük ağırlıklı sapma)",
    admissible: "Γ ≤ 2: analiz fonksiyonu kalibrasyon verileriyle uyumludur (ISO 6143 6.2.2).",
    not_admissible: "Γ > 2: analiz fonksiyonu kalibrasyon verileriyle uyumlu değildir; başka bir fonksiyon deneyin veya verileri gözden geçirin (ISO 6143 6.2.2).",
    compare: "Analiz fonksiyonlarının karşılaştırılması",
    parameters_n: "parametre",
    not_fitted: "uydurulamadı",
    chosen: "seçilen",
    samples: "Analiz edilen karışımların içerikleri",
    sample: "Karışım",
    out_of_range: "kalibrasyon aralığının dışında",
    sample_correlations: "Sonuçların korelasyon katsayıları (ortak kalibrasyon nedeniyle)",
    bound: "Belirsizliğin üst sınırı (ISO 6143 6.2.3)",
    bound_text: "En düşük ve en yüksek kalibrasyon yanıtlarında u(x): {lo} ve {hi}; üst sınır {max}.",
    target_met: "Üst sınır kabul edilebilir standart belirsizliği ({t}) aşmıyor.",
    target_not_met: "Üst sınır kabul edilebilir standart belirsizliği ({t}) aşıyor.",
    drift: "Kayma kontrolü (ISO 6143 6.2.4)",
    drift_text: "Referans karışım {r}, y(kalib) = {y}, öncesinde ve sonrasında n = {n} ölçüm.",
    drift_rows: ["|ȳ(önce) − y(kalib)|", "|y(kalib) − ȳ(sonra)|", "|ȳ(önce) − ȳ(sonra)|"],
    difference: "Fark",
    critical: "Kritik değer",
    passed: "geçti",
    failed: "kaldı",
    validation: "Uyumsuz bileşimli referans karışımlarla doğrulama (ISO 6143 6.2.5, Formül 13)",
    x_obs: "x_göz",
    x_ref: "x_ref",
    limit: "2√(u²(x_göz) + u²(x_ref))",
    chart: "Kalibrasyon noktalarının ağırlıklı sapmaları (● içerik, ■ yanıt) ve ±2 sınırları",
    method_note: "Her iki değişkende belirsizlikli genelleştirilmiş en küçük kareler (ISO 6143 Ek A); parametrelerin kovaryans matrisi, tüm kalibrasyon verilerinin belirsizliklerinin uydurma boyunca yayılmasıyla.",
    names: ["doğrusal", "ikinci derece", "üçüncü derece", "üslü", "üstel"],
};

pub(super) fn cal_labels(lang: Lang) -> &'static CalLabels {
    match lang {
        Lang::En => &CAL_EN,
        Lang::Tr => &CAL_TR,
    }
}

fn function_name(f: FunctionSpec, l: &CalLabels) -> &'static str {
    l.names[FunctionSpec::ALL.iter().position(|&g| g == f).unwrap_or(0)]
}

fn formula(f: FunctionSpec) -> &'static str {
    match f {
        FunctionSpec::Linear => "x = b₀ + b₁y",
        FunctionSpec::Quadratic => "x = b₀ + b₁y + b₂y²",
        FunctionSpec::Cubic => "x = b₀ + b₁y + b₂y² + b₃y³",
        FunctionSpec::Power => "x = b₀ + b₁y^(1 + b₂)",
        FunctionSpec::Exponential => "x = b₀ + b₁e^(b₂y)",
    }
}

/// The content of each analysed mixture with its expanded uncertainty.
pub(super) fn statements(p: &Project, a: &CalibrationAnalysis, lang: Lang) -> Vec<String> {
    let Some(spec) = p.calibration.as_ref() else { return Vec::new() };
    let unit = spec.unit.clone().map(|u| format!(" {u}")).unwrap_or_default();
    a.samples
        .iter()
        .map(|s| {
            let (v, u) = round_reported(s.x, s.expanded, &p.reporting, lang);
            format!("x({}) = ({v} ± {u}){unit} (k = {})", s.name, fmt_num(a.coverage_factor, 3, lang))
        })
        .collect()
}

pub(super) fn markdown(
    p: &Project,
    a: &CalibrationAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = cal_labels(lang);
    let Some(spec) = p.calibration.as_ref() else { return String::new() };
    let unit = spec.unit.clone().map(|u| format!(" {u}")).unwrap_or_default();
    let n = |x: f64| fmt_num(x, 5, lang);
    let n3 = |x: f64| fmt_num(x, 3, lang);
    let fixed = |x: f64, k: usize| {
        let t = format!("{x:.k$}");
        if lang == Lang::Tr {
            t.replace('.', ",")
        } else {
            t
        }
    };
    let f = &a.fit;
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", d.title);
    let ru = spec.response_unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default();
    let _ = writeln!(
        s,
        "{}: {}{}; y{ru}\n",
        d.analyte,
        spec.analyte,
        spec.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default()
    );
    let _ = writeln!(s, "{}\n", d.method_note);

    let _ = writeln!(s, "## {}\n", d.data);
    let _ = writeln!(
        s,
        "| {} | x | u(x) | y | u(y) | (x − x̂)/u(x) | (y − ŷ)/u(y) |\n|---|---:|---:|---:|---:|---:|---:|",
        d.point
    );
    for (pt, w) in spec.points.iter().zip(&f.weighted_deviations) {
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {} |",
            pt.name,
            fmt_given(pt.x, lang),
            fmt_given(pt.u_x, lang),
            fmt_given(pt.y, lang),
            fmt_given(pt.u_y, lang),
            fixed(w.0, 3),
            fixed(w.1, 3)
        );
    }
    let _ = writeln!(s);
    for (i, c) in spec.covariances.iter().enumerate() {
        let sep = if i + 1 == spec.covariances.len() { "\n" } else { "" };
        let _ = writeln!(s, "- u(x({}), x({})) = {}{sep}", c.between[0], c.between[1], fmt_given(c.value, lang));
    }

    let _ = writeln!(s, "## {}: {} — {}\n", d.function, function_name(a.function, d), formula(a.function));
    let _ = writeln!(s, "| {} | {} | u |\n|---|---:|---:|", d.parameter, d.estimate);
    for (j, b) in f.params.iter().enumerate() {
        let _ = writeln!(s, "| b{} | {} | {} |", sub(j), n(*b), n(f.covariance[j][j].sqrt()));
    }
    let _ = writeln!(s);
    let m = f.params.len();
    if m > 1 {
        let _ = writeln!(s, "{}:\n", d.correlation);
        let mut head = "| |".to_string();
        let mut rule = "|---|".to_string();
        for j in 0..m {
            head += &format!(" b{} |", sub(j));
            rule += "---:|";
        }
        let _ = writeln!(s, "{head}\n{rule}");
        for j in 0..m {
            let mut row = format!("| b{} |", sub(j));
            for k in 0..m {
                let r = f.covariance[j][k] / (f.covariance[j][j] * f.covariance[k][k]).sqrt();
                row += &format!(" {} |", fixed(r, 4));
            }
            let _ = writeln!(s, "{row}");
        }
        let _ = writeln!(s);
    }
    let _ = writeln!(s, "- {}: {} ({} 2(n − N − 1) = {})", d.s_res, n(f.s_res), d.s_res_limit, 2 * f.dof);
    let _ = writeln!(s, "- {}: {}\n", d.gamma, n3(f.gamma));
    let _ = writeln!(s, "{}\n", if a.admissible { d.admissible } else { d.not_admissible });

    if a.candidates.len() > 1 {
        let _ = writeln!(s, "## {}\n", d.compare);
        let _ = writeln!(s, "| {} | {} | Γ | S_res | |\n|---|---:|---:|---:|---|", d.function, d.parameters_n);
        for c in &a.candidates {
            let np = c.function.function().parameters();
            let mark = if c.function == a.function { d.chosen } else { "" };
            match &c.fit {
                Ok((g, sr)) => {
                    let _ =
                        writeln!(s, "| {} | {np} | {} | {} | {mark} |", function_name(c.function, d), n3(*g), n(*sr));
                }
                Err(_) => {
                    let _ = writeln!(s, "| {} | {np} | — | — | {} |", function_name(c.function, d), d.not_fitted);
                }
            }
        }
        let _ = writeln!(s);
    }

    if !a.samples.is_empty() {
        let _ = writeln!(s, "## {}\n", d.samples);
        let _ = writeln!(
            s,
            "| {} | y | u(y) | x | u(x) | U(x) (k = {}) | |\n|---|---:|---:|---:|---:|---:|---|",
            d.sample,
            n3(a.coverage_factor)
        );
        for x in &a.samples {
            let _ = writeln!(
                s,
                "| {} | {} | {} | {}{unit} | {}{unit} | {}{unit} | {} |",
                x.name,
                fmt_given(x.y, lang),
                fmt_given(x.u_y, lang),
                n(x.x),
                n(x.u),
                n3(x.expanded),
                if x.in_range { "" } else { d.out_of_range }
            );
        }
        let _ = writeln!(s);
        if a.samples.len() > 1 {
            let _ = writeln!(s, "{}:\n", d.sample_correlations);
            let mut head = "| |".to_string();
            let mut rule = "|---|".to_string();
            for x in &a.samples {
                head += &format!(" {} |", x.name);
                rule += "---:|";
            }
            let _ = writeln!(s, "{head}\n{rule}");
            for (x, row) in a.samples.iter().zip(&a.sample_correlations) {
                let cells: String = row.iter().map(|r| format!(" {} |", fixed(*r, 3))).collect();
                let _ = writeln!(s, "| {} |{cells}", x.name);
            }
            let _ = writeln!(s);
        }
        for st in statements(p, a, lang) {
            let _ = writeln!(s, "**{}:** {st}\n", l.reported);
        }
    }

    let _ = writeln!(s, "## {}\n", d.bound);
    let _ = writeln!(
        s,
        "{}",
        d.bound_text
            .replace("{lo}", &format!("{}{unit}", n3(a.u_lo)))
            .replace("{hi}", &format!("{}{unit}", n3(a.u_hi)))
            .replace("{max}", &format!("{}{unit}", n3(a.upper_bound())))
    );
    if let (Some(t), Some(ok)) = (spec.target_uncertainty, a.target_met) {
        let _ = writeln!(
            s,
            "{}",
            (if ok { d.target_met } else { d.target_not_met }).replace("{t}", &format!("{}{unit}", fmt_given(t, lang)))
        );
    }
    let _ = writeln!(s);

    if let (Some((name, y, t)), Some(ds)) = (&a.drift, &spec.drift) {
        let _ = writeln!(s, "## {}\n", d.drift);
        let _ = writeln!(
            s,
            "{}\n",
            d.drift_text.replace("{r}", name).replace("{y}", &fmt_given(*y, lang)).replace("{n}", &ds.n.to_string())
        );
        let _ = writeln!(s, "| | {} | {} | |\n|---|---:|---:|---|", d.difference, d.critical);
        for i in 0..3 {
            let ok = t.differences[i] <= t.critical[i];
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} |",
                d.drift_rows[i],
                n3(t.differences[i]),
                n3(t.critical[i]),
                if ok { d.passed } else { d.failed }
            );
        }
        let _ = writeln!(s);
    }

    if !a.validation.is_empty() {
        let _ = writeln!(s, "## {}\n", d.validation);
        let _ = writeln!(
            s,
            "| {} | {} | u | {} | u | {} | {} | |\n|---|---:|---:|---:|---:|---:|---:|---|",
            d.sample, d.x_obs, d.x_ref, d.difference, d.limit
        );
        for v in &a.validation {
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                v.name,
                n(v.x_obs),
                n3(v.u_obs),
                fmt_given(v.x_ref, lang),
                fmt_given(v.u_ref, lang),
                n3(v.difference),
                n3(v.limit),
                if v.passed { d.passed } else { d.failed }
            );
        }
        let _ = writeln!(s);
    }
    tail(&mut s, p, checks, lang, "ISO 6143:2025", generated);
    s
}

fn sub(j: usize) -> &'static str {
    ["₀", "₁", "₂", "₃"][j.min(3)]
}

/// Weighted deviations of the calibration points with the limits ±2 (SVG).
pub(super) fn deviations_svg(p: &Project, a: &CalibrationAnalysis, lang: Lang) -> Option<String> {
    let spec = p.calibration.as_ref()?;
    let w = &a.fit.weighted_deviations;
    if w.is_empty() {
        return None;
    }
    let (wd, ht, left, top, bottom) = (640.0, 240.0, 50.0, 12.0, 30.0);
    let lim = w.iter().map(|d| d.0.abs().max(d.1.abs())).fold(2.5_f64, f64::max).min(10.0);
    let plot_w = wd - left - 16.0;
    let y_of = |v: f64| top + (ht - top - bottom) * (lim - v.clamp(-lim, lim)) / (2.0 * lim);
    let x_of = |i: usize| left + plot_w * (i as f64 + 0.5) / w.len() as f64;
    let mut s = format!(
        r#"<svg viewBox="0 0 {wd} {ht}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(cal_labels(lang).chart)
    );
    for v in [-2.0, 0.0, 2.0] {
        let y = y_of(v);
        let class = if v == 0.0 { "axis" } else { "mcm" };
        let _ = write!(
            s,
            r#"<line x1="{left}" x2="{:.1}" y1="{y:.1}" y2="{y:.1}" class="{class}"/><text x="{}" y="{:.1}" text-anchor="end" class="val">{}</text>"#,
            left + plot_w,
            left - 6.0,
            y + 4.0,
            fmt_num(v, 1, lang)
        );
    }
    for (i, (d, pt)) in w.iter().zip(&spec.points).enumerate() {
        let x = x_of(i);
        let _ = write!(s, r#"<circle cx="{:.1}" cy="{:.1}" r="4" class="bar"/>"#, x - 5.0, y_of(d.0));
        let _ = write!(s, r#"<rect x="{:.1}" y="{:.1}" width="7" height="7" class="hist"/>"#, x + 1.5, y_of(d.1) - 3.5);
        let _ = write!(
            s,
            r#"<text x="{x:.1}" y="{:.1}" text-anchor="middle" class="val">{}</text>"#,
            ht - 10.0,
            esc(&pt.name)
        );
    }
    s.push_str("</svg>");
    Some(s)
}
