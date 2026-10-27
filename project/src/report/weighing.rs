//! Reports of the error approximation function of a weighing instrument
//! (EURAMET cg-18, Appendix C2.2).

use super::*;
use crate::weighing::ErrorFunctionAnalysis;
use uncertainty_core::weighing::ReferenceCorrelation;

pub(super) struct EfLabels {
    pub title: &'static str,
    pub instrument: &'static str,
    pub function: &'static str,
    pub through_zero: &'static str,
    pub with_constant: &'static str,
    pub covariance: &'static str,
    pub correlation: [&'static str; 2],
    pub s_m: &'static str,
    pub coverage: &'static str,
    pub points: &'static str,
    pub cols: [&'static str; 8],
    pub yes: &'static str,
    pub no: &'static str,
    pub coefficients: &'static str,
    pub coefficient: &'static str,
    pub covariance_matrix: &'static str,
    pub fit: &'static str,
    pub chi_ok: &'static str,
    pub chi_fail: &'static str,
    pub resid_ok: &'static str,
    pub resid_fail: &'static str,
    pub min_s_m: &'static str,
    pub min_none: &'static str,
    pub readings: &'static str,
    pub reading_cols: [&'static str; 4],
    pub chart: &'static str,
    pub statement: &'static str,
}

const EF_EN: EfLabels = EfLabels {
    title: "Error approximation function (EURAMET cg-18, Appendix C)",
    instrument: "Instrument",
    function: "Approximation function",
    through_zero: "polynomial through zero",
    with_constant: "polynomial",
    covariance: "Covariance matrix U(e)",
    correlation: [
        "U(m_ref) = s sᵀ (fully correlated reference masses, C2.2-3b) + U(I_Cal) + s_m²·I",
        "uncorrelated reference masses + U(I_Cal) + s_m²·I",
    ],
    s_m: "Model term s_m",
    coverage: "Coverage factor",
    points: "Test points",
    cols: ["I", "E", "u(e)", "E_appr", "Residual v", "u(E_appr)", "U(E_appr)", "C2.2-2b"],
    yes: "yes",
    no: "no",
    coefficients: "Coefficients (C2.2-6, C2.2-9)",
    coefficient: "Coefficient",
    covariance_matrix: "Covariance matrix U(â)",
    fit: "Goodness of fit",
    chi_ok: "χ²_obs = {chi} ≤ ν = {nu}: the function is consistent with the data (C2.2-2a).",
    chi_fail:
        "χ²_obs = {chi} > ν = {nu}: the function is not consistent with the data (C2.2-2a); increase s_m or the degree.",
    resid_ok: "Every residual lies within its expanded uncertainty U(E_appr) (C2.2-2b).",
    resid_fail: "{n} residual(s) exceed U(E_appr) (C2.2-2b, the more restrictive criterion): {list}.",
    min_s_m: "Smallest model term meeting C2.2-2a: s_m = {a}; meeting C2.2-2b: s_m = {b}.",
    min_none: "not found",
    readings: "Approximated errors at readings (C2.2-12, C2.2-13)",
    reading_cols: ["Reading R", "u(R)", "E_appr(R)", "U(E_appr)"],
    chart: "Errors of indication with U(e) and the approximation function with U(E_appr)",
    statement: "E_appr(I) = {f}; χ²_obs = {chi}, ν = {nu} ({verdict}); s_m = {sm}",
};

const EF_TR: EfLabels = EfLabels {
    title: "Hata yaklaşım fonksiyonu (EURAMET cg-18, Ek C)",
    instrument: "Tartı aleti",
    function: "Yaklaşım fonksiyonu",
    through_zero: "sıfırdan geçen polinom",
    with_constant: "polinom",
    covariance: "Kovaryans matrisi U(e)",
    correlation: [
        "U(m_ref) = s sᵀ (tam korelasyonlu referans kütleler, C2.2-3b) + U(I_Cal) + s_m²·I",
        "korelasyonsuz referans kütleler + U(I_Cal) + s_m²·I",
    ],
    s_m: "Model terimi s_m",
    coverage: "Kapsam faktörü",
    points: "Deney noktaları",
    cols: ["I", "E", "u(e)", "E_yakl", "Artık v", "u(E_yakl)", "U(E_yakl)", "C2.2-2b"],
    yes: "evet",
    no: "hayır",
    coefficients: "Katsayılar (C2.2-6, C2.2-9)",
    coefficient: "Katsayı",
    covariance_matrix: "Kovaryans matrisi U(â)",
    fit: "Uyum iyiliği",
    chi_ok: "χ²_göz = {chi} ≤ ν = {nu}: fonksiyon verilerle tutarlıdır (C2.2-2a).",
    chi_fail:
        "χ²_göz = {chi} > ν = {nu}: fonksiyon verilerle tutarlı değildir (C2.2-2a); s_m'yi veya dereceyi artırın.",
    resid_ok: "Her artık kendi genişletilmiş belirsizliği U(E_yakl) içindedir (C2.2-2b).",
    resid_fail: "{n} artık U(E_yakl)'yi aşıyor (C2.2-2b, daha kısıtlayıcı ölçüt): {list}.",
    min_s_m: "C2.2-2a'yı sağlayan en küçük model terimi: s_m = {a}; C2.2-2b'yi sağlayan: s_m = {b}.",
    min_none: "bulunamadı",
    readings: "Okumalarda yaklaşık hatalar (C2.2-12, C2.2-13)",
    reading_cols: ["Okuma R", "u(R)", "E_yakl(R)", "U(E_yakl)"],
    chart: "U(e) ile gösterge hataları ve U(E_yakl) ile yaklaşım fonksiyonu",
    statement: "E_yakl(I) = {f}; χ²_göz = {chi}, ν = {nu} ({verdict}); s_m = {sm}",
};

pub(super) fn ef_labels(lang: Lang) -> &'static EfLabels {
    match lang {
        Lang::En => &EF_EN,
        Lang::Tr => &EF_TR,
    }
}

fn eunit(a: &ErrorFunctionAnalysis) -> String {
    a.error_unit.clone().map(|u| format!(" {u}")).unwrap_or_default()
}

fn iunit(a: &ErrorFunctionAnalysis) -> String {
    a.unit.clone().map(|u| format!(" {u}")).unwrap_or_default()
}

/// The fitted function as text, e.g. "8.384·10⁻⁴ mg/g · I".
fn function_text(a: &ErrorFunctionAnalysis, lang: Lang) -> String {
    let f = &a.fit;
    let parts: Vec<String> = f
        .function
        .powers()
        .iter()
        .zip(&f.coefficients)
        .map(|(&p, &c)| match p {
            0 => fmt_num(c, 4, lang),
            1 => format!("{}·I", fmt_num(c, 4, lang)),
            _ => format!("{}·I{}", fmt_num(c, 4, lang), superscript(p)),
        })
        .collect();
    parts.join(" + ").replace("+ -", "− ")
}

pub(super) fn statements(a: &ErrorFunctionAnalysis, lang: Lang) -> Vec<String> {
    let d = ef_labels(lang);
    let f = &a.fit;
    let mut out = vec![d
        .statement
        .replace("{f}", &function_text(a, lang))
        .replace("{chi}", &fmt_num(f.chi_squared, 3, lang))
        .replace("{nu}", &f.dof.to_string())
        .replace("{verdict}", if f.chi_squared_passed() { "C2.2-2a ✓" } else { "C2.2-2a ✗" })
        .replace("{sm}", &format!("{}{}", fmt_given(f.s_m, lang), eunit(a)))];
    let k = a.coverage_factor;
    let u: Vec<String> = f
        .function
        .powers()
        .iter()
        .enumerate()
        .map(|(i, p)| format!("u(a{p}) = {}", fmt_num(f.covariance[i][i].sqrt(), 3, lang)))
        .collect();
    out.push(u.join(", "));
    for r in &a.readings {
        out.push(format!(
            "E_appr({}{}) = {}{}, U = {}{} (k = {})",
            fmt_given(r.reading, lang),
            iunit(a),
            fmt_num(r.error, 4, lang),
            eunit(a),
            fmt_num(k * r.u, 3, lang),
            eunit(a),
            fmt_given(k, lang)
        ));
    }
    out
}

pub(super) fn markdown(
    p: &Project,
    a: &ErrorFunctionAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = ef_labels(lang);
    let f = &a.fit;
    let k = a.coverage_factor;
    let eu = eunit(a);
    let eb = a.error_unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default();
    let n = |x: f64| fmt_num(x, 3, lang);
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", d.title);
    let _ = writeln!(s, "| | |\n|---|---|");
    let _ = writeln!(s, "| {} | {} |", d.instrument, a.instrument);
    let kind = if f.function.through_zero { d.through_zero } else { d.with_constant };
    let _ = writeln!(s, "| {} | {kind}, nₐ = {}: E(I) = {} |", d.function, f.function.degree, function_text(a, lang));
    let corr = match a.correlation {
        ReferenceCorrelation::Full => d.correlation[0],
        ReferenceCorrelation::None => d.correlation[1],
    };
    let _ = writeln!(s, "| {} | {corr} |", d.covariance);
    let _ = writeln!(s, "| {} | {}{eu} |", d.s_m, fmt_given(f.s_m, lang));
    let _ = writeln!(s, "| {} | {} |\n", d.coverage, fmt_given(k, lang));

    let _ = writeln!(s, "### {}\n", d.points);
    let c = &d.cols;
    let _ = writeln!(
        s,
        "| {}{} | {}{eb} | {}{eb} | {}{eb} | {}{eb} | {}{eb} | {}{eb} | {} |\n|---:|---:|---:|---:|---:|---:|---:|---|",
        c[0],
        a.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default(),
        c[1],
        c[2],
        c[3],
        c[4],
        c[5],
        c[6],
        c[7]
    );
    for (j, pt) in a.points.iter().enumerate() {
        let ok = !a.residual_failures.contains(&j);
        let _ = writeln!(
            s,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            fmt_given(pt.indication, lang),
            n(pt.error),
            fmt_num(f.u_error[j], 3, lang),
            n(f.approximated[j]),
            n(f.residuals[j]),
            fmt_num(f.u_approximated[j], 3, lang),
            fmt_num(k * f.u_approximated[j], 3, lang),
            if ok { d.yes } else { d.no }
        );
    }
    let _ = writeln!(s);

    let _ = writeln!(s, "### {}\n", d.coefficients);
    let _ = writeln!(s, "| {} | â | u(â) |\n|---|---:|---:|", d.coefficient);
    for (i, p) in f.function.powers().iter().enumerate() {
        let _ = writeln!(
            s,
            "| a{p} | {} | {} |",
            fmt_num(f.coefficients[i], 5, lang),
            fmt_num(f.covariance[i][i].sqrt(), 3, lang)
        );
    }
    let _ = writeln!(s);
    if f.coefficients.len() > 1 {
        let _ = writeln!(s, "{}:\n", d.covariance_matrix);
        let names: Vec<String> = f.function.powers().iter().map(|p| format!("a{p}")).collect();
        let _ = writeln!(s, "| | {} |\n|---|{}", names.join(" | "), "---:|".repeat(names.len()));
        for (i, row) in f.covariance.iter().enumerate() {
            let v: Vec<String> = row.iter().map(|&x| fmt_num(x, 4, lang)).collect();
            let _ = writeln!(s, "| {} | {} |", names[i], v.join(" | "));
        }
        let _ = writeln!(s);
    } else {
        let _ = writeln!(s, "{}: {}\n", d.covariance_matrix, fmt_num(f.covariance[0][0], 4, lang));
    }

    let _ = writeln!(s, "### {}\n", d.fit);
    let chi = if f.chi_squared_passed() { d.chi_ok } else { d.chi_fail };
    let _ =
        writeln!(s, "{}\n", chi.replace("{chi}", &fmt_num(f.chi_squared, 3, lang)).replace("{nu}", &f.dof.to_string()));
    if a.residual_failures.is_empty() {
        let _ = writeln!(s, "{}\n", d.resid_ok);
    } else {
        let list: Vec<String> = a
            .residual_failures
            .iter()
            .map(|&j| format!("I = {}{}", fmt_given(a.points[j].indication, lang), iunit(a)))
            .collect();
        let _ = writeln!(
            s,
            "{}\n",
            d.resid_fail.replace("{n}", &a.residual_failures.len().to_string()).replace("{list}", &list.join(", "))
        );
    }
    let m =
        |x: Option<f64>| x.map(|v| format!("{}{eu}", fmt_num(v, 3, lang))).unwrap_or_else(|| d.min_none.to_string());
    let _ = writeln!(
        s,
        "{}\n",
        d.min_s_m.replace("{a}", &m(a.min_s_m_chi_squared)).replace("{b}", &m(a.min_s_m_residuals))
    );

    if !a.readings.is_empty() {
        let _ = writeln!(s, "### {}\n", d.readings);
        let rc = &d.reading_cols;
        let _ = writeln!(
            s,
            "| {}{} | {}{} | {}{eb} | {} (k = {}){eb} |\n|---:|---:|---:|---:|",
            rc[0],
            a.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default(),
            rc[1],
            a.unit.clone().map(|u| format!(" [{u}]")).unwrap_or_default(),
            rc[2],
            rc[3],
            fmt_given(k, lang)
        );
        for r in &a.readings {
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} |",
                fmt_given(r.reading, lang),
                fmt_given(r.u_reading, lang),
                n(r.error),
                fmt_num(k * r.u, 3, lang)
            );
        }
        let _ = writeln!(s);
    }
    tail(&mut s, p, checks, lang, "EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix C2.2", generated);
    s
}

/// Errors with ±U(e) bars and the fitted function with its ±U band (SVG).
pub(super) fn chart_svg(a: &ErrorFunctionAnalysis, lang: Lang) -> Option<String> {
    let d = ef_labels(lang);
    let f = &a.fit;
    let k = a.coverage_factor;
    if a.points.is_empty() {
        return None;
    }
    let (x0, x1) = a
        .points
        .iter()
        .map(|p| p.indication)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| (lo.min(x), hi.max(x)));
    if x1.is_nan() || x0.is_nan() || x1 <= x0 {
        return None;
    }
    let grid: Vec<f64> = (0..=60).map(|i| x0 + (x1 - x0) * i as f64 / 60.0).collect();
    let band: Vec<(f64, f64)> = grid.iter().map(|&x| a.fit_at(x)).collect();
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for (j, p) in a.points.iter().enumerate() {
        lo = lo.min(p.error - k * f.u_error[j]);
        hi = hi.max(p.error + k * f.u_error[j]);
    }
    for &(e, u) in &band {
        lo = lo.min(e - k * u);
        hi = hi.max(e + k * u);
    }
    if hi.is_nan() || lo.is_nan() || hi <= lo {
        return None;
    }
    let pad = 0.05 * (hi - lo);
    let (lo, hi) = (lo - pad, hi + pad);
    let (w, h, left, right, top, bottom) = (640.0, 280.0, 64.0, 12.0, 10.0, 30.0);
    let pw = w - left - right;
    let ph = h - top - bottom;
    let xs = |x: f64| left + pw * (x - x0) / (x1 - x0);
    let ys = |y: f64| top + ph * (hi - y) / (hi - lo);
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(d.chart)
    );
    // Band of the approximation function.
    let mut path = String::new();
    for (i, (&x, &(e, u))) in grid.iter().zip(&band).enumerate() {
        let _ = write!(path, "{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, xs(x), ys(e + k * u));
    }
    for (&x, &(e, u)) in grid.iter().zip(&band).rev() {
        let _ = write!(path, "L{:.1},{:.1} ", xs(x), ys(e - k * u));
    }
    let _ = write!(s, r#"<path d="{path}Z" class="hist"/>"#);
    let line: Vec<String> = grid.iter().zip(&band).map(|(&x, &(e, _))| format!("{:.1},{:.1}", xs(x), ys(e))).collect();
    let _ = write!(s, r#"<polyline points="{}" class="pdf"/>"#, line.join(" "));
    if lo < 0.0 && hi > 0.0 {
        let y = ys(0.0);
        let _ = write!(s, r#"<line x1="{left}" x2="{:.1}" y1="{y:.1}" y2="{y:.1}" class="axis"/>"#, left + pw);
    }
    for (j, p) in a.points.iter().enumerate() {
        let (x, y) = (xs(p.indication), ys(p.error));
        let u = k * f.u_error[j];
        let _ = write!(
            s,
            r#"<line x1="{x:.1}" x2="{x:.1}" y1="{:.1}" y2="{:.1}" class="axis"/><circle cx="{x:.1}" cy="{y:.1}" r="4" class="bar"/>"#,
            ys(p.error + u),
            ys(p.error - u)
        );
    }
    for v in [lo + pad, hi - pad] {
        let _ = write!(
            s,
            r#"<text x="{}" y="{:.1}" text-anchor="end" class="val">{}</text>"#,
            left - 6.0,
            ys(v) + 4.0,
            fmt_num(v, 3, lang)
        );
    }
    for x in [x0, x1] {
        let _ = write!(
            s,
            r#"<text x="{:.1}" y="{}" text-anchor="middle" class="val">{}</text>"#,
            xs(x),
            h - 10.0,
            fmt_given(x, lang)
        );
    }
    s.push_str("</svg>");
    Some(s)
}

impl ErrorFunctionAnalysis {
    /// Approximated error and its standard uncertainty at an indication,
    /// without the uncertainty of the reading.
    fn fit_at(&self, x: f64) -> (f64, f64) {
        self.fit.at(x, 0.0)
    }
}
