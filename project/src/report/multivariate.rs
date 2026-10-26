//! Reports of models with several output quantities (JCGM 102:2011).

use super::*;
use crate::multivariate::MultivariateAnalysis;

pub(super) struct MvLabels {
    pub implicit: &'static str,
    pub indications: &'static str,
    pub indications_mean: &'static str,
    pub indications_t: &'static str,
    pub gum: &'static str,
    pub gum_note: &'static str,
    pub mcm: &'static str,
    pub mcm_note: &'static str,
    pub output: &'static str,
    pub correlation: &'static str,
    pub sensitivity: &'static str,
    pub regions: &'static str,
    pub ellipsoid: &'static str,
    pub rectangle: &'static str,
    pub area: &'static str,
    pub smallest: &'static str,
    pub gum_coverage: &'static str,
    pub trials: &'static str,
    pub adaptive: &'static str,
    pub not_converged: &'static str,
    pub validation: &'static str,
    pub validation_note: &'static str,
    pub quantity: &'static str,
    pub difference: &'static str,
    pub tolerance: &'static str,
    pub validated: &'static str,
    pub not_validated: &'static str,
    pub region_statement: &'static str,
    pub seed: &'static str,
    pub chart: &'static str,
    pub u_chart: &'static str,
}

const MV_EN: MvLabels = MvLabels {
    implicit: "Implicit model: the output quantities are the solution of h(Y, X) = 0 (JCGM 102 6.3); the uncertainties follow from C_y U_y C_yᵀ = C_x U_x C_xᵀ (formula 8).",
    indications: "Input quantities from series of simultaneous indication values (JCGM 102 5.3.2)",
    indications_mean: "GUM framework: U_x = M/(n(n − 1)) (GUM 5.2.3).",
    indications_t: "GUM framework: U_x = M/((ν − 2)n), the covariance matrix of the multivariate t-distribution (JCGM 102 9.4.2.5).",
    gum: "GUM uncertainty framework (JCGM 102 clause 6)",
    gum_note: "U_y = C_x U_x C_xᵀ (formula 3); coverage regions for a Gaussian distribution N(y, U_y) (6.5).",
    mcm: "Monte Carlo method (JCGM 102 clause 7)",
    mcm_note: "Estimate and covariance matrix from the draws (7.6); k_p from the draws sorted by (y_r − y)ᵀ U_y⁻¹ (y_r − y) (7.7.2), k_q by max_j |y_j,r − y_j|/u(y_j) (7.7.3).",
    output: "Output quantity",
    correlation: "Correlation coefficients",
    sensitivity: "Sensitivity matrix ∂Y/∂X",
    regions: "Coverage regions",
    ellipsoid: "hyper-ellipsoid (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p²",
    rectangle: "hyper-rectangle y_j ± k_q u(y_j)",
    area: "area",
    smallest: "Approximation to the smallest coverage region (100 × 100 mesh, 7.7.4): area",
    gum_coverage: "Fraction of the Monte Carlo draws inside the GUM regions: ellipsoid {e}, rectangle {r}.",
    trials: "trials",
    adaptive: "adaptive procedure (7.8.3), {h} applications",
    not_converged: "the adaptive procedure stopped at the maximum number of trials before the results stabilized",
    validation: "Validation of the GUM framework (JCGM 102 clause 8)",
    validation_note: "Absolute differences between the GUM and Monte Carlo results compared with the numerical tolerances for {n} significant digits (7.8.2).",
    quantity: "Quantity",
    difference: "Difference",
    tolerance: "Tolerance",
    validated: "validated",
    not_validated: "not validated",
    region_statement: "{p} coverage region ({m}): {region}, k_p = {kp}; rectangle k_q = {kq}",
    seed: "seed",
    chart: "Monte Carlo draws with the coverage ellipses (GUM dotted, Monte Carlo solid) and the GUM rectangle (dashed)",
    u_chart: "Standard uncertainties of the output quantities (dark: GUM framework, light: Monte Carlo method)",
};

const MV_TR: MvLabels = MvLabels {
    implicit: "Örtük model: çıktı büyüklükleri h(Y, X) = 0 denkleminin çözümüdür (JCGM 102 6.3); belirsizlikler C_y U_y C_yᵀ = C_x U_x C_xᵀ bağıntısından çıkar (formül 8).",
    indications: "Eşzamanlı gösterge değeri serilerinden girdi büyüklükleri (JCGM 102 5.3.2)",
    indications_mean: "GUM yaklaşımı: U_x = M/(n(n − 1)) (GUM 5.2.3).",
    indications_t: "GUM yaklaşımı: U_x = M/((ν − 2)n), çok değişkenli t dağılımının kovaryans matrisi (JCGM 102 9.4.2.5).",
    gum: "GUM belirsizlik yaklaşımı (JCGM 102 Madde 6)",
    gum_note: "U_y = C_x U_x C_xᵀ (formül 3); N(y, U_y) Gauss dağılımı için kapsam bölgeleri (6.5).",
    mcm: "Monte Carlo yöntemi (JCGM 102 Madde 7)",
    mcm_note: "Tahmin ve kovaryans matrisi çekilişlerden (7.6); k_p, (y_r − y)ᵀ U_y⁻¹ (y_r − y) değerine göre sıralanan çekilişlerden (7.7.2), k_q ise max_j |y_j,r − y_j|/u(y_j) ile (7.7.3).",
    output: "Çıktı büyüklüğü",
    correlation: "Korelasyon katsayıları",
    sensitivity: "Duyarlılık matrisi ∂Y/∂X",
    regions: "Kapsam bölgeleri",
    ellipsoid: "hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p²",
    rectangle: "hiper-dikdörtgen y_j ± k_q u(y_j)",
    area: "alan",
    smallest: "En küçük kapsam bölgesinin yaklaşığı (100 × 100 ağ, 7.7.4): alan",
    gum_coverage: "GUM bölgelerinin içindeki Monte Carlo çekilişlerinin oranı: elipsoit {e}, dikdörtgen {r}.",
    trials: "deneme",
    adaptive: "uyarlamalı yöntem (7.8.3), {h} uygulama",
    not_converged: "uyarlamalı yöntem sonuçlar kararlı hâle gelmeden en büyük deneme sayısında durdu",
    validation: "GUM yaklaşımının doğrulanması (JCGM 102 Madde 8)",
    validation_note: "GUM ve Monte Carlo sonuçları arasındaki mutlak farklar {n} anlamlı basamak için sayısal toleranslarla karşılaştırılmıştır (7.8.2).",
    quantity: "Büyüklük",
    difference: "Fark",
    tolerance: "Tolerans",
    validated: "doğrulandı",
    not_validated: "doğrulanmadı",
    region_statement: "{p} kapsam bölgesi ({m}): {region}, k_p = {kp}; dikdörtgen k_q = {kq}",
    seed: "tohum",
    chart: "Kapsam elipsleri (GUM noktalı, Monte Carlo düz) ve GUM dikdörtgeni (kesikli) ile Monte Carlo çekilişleri",
    u_chart: "Çıktı büyüklüklerinin standart belirsizlikleri (koyu: GUM yaklaşımı, açık: Monte Carlo yöntemi)",
};

pub(super) fn mv_labels(lang: Lang) -> &'static MvLabels {
    match lang {
        Lang::En => &MV_EN,
        Lang::Tr => &MV_TR,
    }
}

/// x and u with the decimals that show three significant digits of u.
fn at_u(x: f64, u: f64, lang: Lang) -> String {
    if u.is_nan() || u <= 0.0 || !u.is_finite() || !x.is_finite() {
        return fmt_num(x, 6, lang);
    }
    let k = (2 - u.log10().floor() as i32).clamp(0, 12) as usize;
    fixed(x, k, lang)
}

fn fixed(x: f64, k: usize, lang: Lang) -> String {
    let t = format!("{x:.k$}");
    if lang == Lang::Tr {
        t.replace('.', ",").replace('-', "−")
    } else {
        t.replace('-', "−")
    }
}

/// The results to report: Monte Carlo unless the GUM ellipsoid is validated.
fn preferred(a: &MultivariateAnalysis) -> (&'static str, Vec<f64>, Vec<f64>, f64, f64, f64) {
    let gum_ok = a.validation.as_ref().map_or(a.mcm.is_none(), |v| v.ellipsoid_validated);
    match (&a.gum, &a.mcm) {
        (Some(g), _) if gum_ok => (
            "GUM",
            g.estimate.clone(),
            g.standard_uncertainties.clone(),
            g.k_ellipsoid,
            g.k_rectangle,
            g.coverage_probability,
        ),
        (_, Some(m)) => (
            "MCM",
            m.estimate.clone(),
            m.standard_uncertainties.clone(),
            m.k_ellipsoid,
            m.k_rectangle,
            m.coverage_probability,
        ),
        (Some(g), None) => (
            "GUM",
            g.estimate.clone(),
            g.standard_uncertainties.clone(),
            g.k_ellipsoid,
            g.k_rectangle,
            g.coverage_probability,
        ),
        (None, None) => ("", Vec::new(), Vec::new(), f64::NAN, f64::NAN, f64::NAN),
    }
}

/// One statement per output (estimate and standard uncertainty) and one for
/// the coverage region.
pub(super) fn statements(a: &MultivariateAnalysis, lang: Lang) -> Vec<String> {
    let d = mv_labels(lang);
    let (method, y, u, kp, kq, p) = preferred(a);
    if method.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<String> = a
        .model
        .outputs
        .iter()
        .enumerate()
        .map(|(j, o)| {
            let unit = o.unit.clone().map(|x| format!(" {x}")).unwrap_or_default();
            format!(
                "{} = {}{unit}, u({}) = {}{unit} ({method})",
                o.name,
                at_u(y[j], u[j], lang),
                o.name,
                fmt_num(u[j], 2, lang)
            )
        })
        .collect();
    if a.model.n_outputs() > 1 {
        out.push(
            d.region_statement
                .replace("{p}", &fmt_pct(p, 3, lang, true))
                .replace("{m}", method)
                .replace("{region}", d.ellipsoid)
                .replace("{kp}", &fmt_num(kp, 3, lang))
                .replace("{kq}", &fmt_num(kq, 3, lang)),
        );
    }
    out
}

fn matrix_table(s: &mut String, names: &[String], m: &[Vec<f64>], lang: Lang) {
    let mut head = "| |".to_string();
    let mut rule = "|---|".to_string();
    for n in names {
        head += &format!(" {n} |");
        rule += "---:|";
    }
    let _ = writeln!(s, "{head}\n{rule}");
    for (i, n) in names.iter().enumerate() {
        let mut row = format!("| {n} |");
        for v in &m[i] {
            row += &format!(" {} |", fixed(*v, 3, lang));
        }
        let _ = writeln!(s, "{row}");
    }
    let _ = writeln!(s);
}

#[allow(clippy::too_many_arguments)]
fn results_block(
    s: &mut String,
    a: &MultivariateAnalysis,
    y: &[f64],
    u: &[f64],
    r: &[Vec<f64>],
    cov: &[Vec<f64>],
    kp: f64,
    kq: f64,
    p: f64,
    lang: Lang,
) {
    let l = labels(lang);
    let d = mv_labels(lang);
    let _ = writeln!(s, "| {} | {} | {} |\n|---|---:|---:|", d.output, l.estimate, l.std_unc);
    for (j, o) in a.model.outputs.iter().enumerate() {
        let unit = o.unit.clone().map(|x| format!(" {x}")).unwrap_or_default();
        let _ = writeln!(s, "| {} | {}{unit} | {}{unit} |", o.name, at_u(y[j], u[j], lang), at_u(u[j], u[j], lang));
    }
    let _ = writeln!(s);
    let names = a.output_names();
    if names.len() > 1 {
        let _ = writeln!(s, "{}:\n", d.correlation);
        matrix_table(s, &names, r, lang);
        let _ = writeln!(s, "{} ({}):\n", d.regions, fmt_pct(p, 3, lang, true));
        let mut e = format!("- {}: k_p = {}", d.ellipsoid, fmt_num(kp, 3, lang));
        let mut q = format!("- {}: k_q = {}", d.rectangle, fmt_num(kq, 3, lang));
        if names.len() == 2 {
            e += &format!(
                " ({} {})",
                d.area,
                fmt_num(uncertainty_core::multivariate::ellipse_area(&cov.to_vec(), kp), 3, lang)
            );
            q += &format!(" ({} {})", d.area, fmt_num(uncertainty_core::multivariate::rectangle_area(u, kq), 3, lang));
        }
        let _ = writeln!(s, "{e}\n{q}\n");
    }
}

pub(super) fn markdown(
    p: &Project,
    a: &MultivariateAnalysis,
    checks: &[CheckOutcome],
    lang: Lang,
    generated: Option<&str>,
) -> String {
    let l = labels(lang);
    let d = mv_labels(lang);
    let Some(spec) = p.multivariate.as_ref() else { return String::new() };
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", p.metadata.title.get(lang));
    let _ = writeln!(s, "*{}*\n", l.report);
    if let Some(x) = &p.metadata.description {
        let _ = writeln!(s, "{}\n", x.get(lang));
    }
    let _ = writeln!(s, "## {}\n", l.model);
    let _ = writeln!(s, "```");
    for o in &spec.outputs {
        let unit = o.unit.clone().map(|u| format!("  [{u}]")).unwrap_or_default();
        if spec.implicit {
            let _ = writeln!(s, "0 = {}  ({}{unit})", o.expression, o.name);
        } else {
            let _ = writeln!(s, "{} = {}{unit}", o.name, o.expression);
        }
    }
    let _ = writeln!(s, "```\n");
    if spec.implicit {
        let _ = writeln!(s, "{}\n", d.implicit);
    }
    if !spec.intermediates.is_empty() {
        let _ = writeln!(s, "{}:\n\n```", l.intermediates);
        for i in &spec.intermediates {
            let _ = writeln!(s, "{} = {}", i.name, i.expression);
        }
        let _ = writeln!(s, "```\n");
    }
    if !spec.inputs.is_empty() {
        let _ = writeln!(s, "## {}\n", l.inputs);
        let _ =
            writeln!(s, "| {} | {} | {} | {} |\n|---|---|---|---|", l.quantity, l.unit, l.distribution, l.information);
        for q in &spec.inputs {
            let desc = q.description.as_ref().map(|x| format!(" — {}", x.get(lang))).unwrap_or_default();
            let _ = writeln!(
                s,
                "| {}{desc} | {} | {} | {} |",
                q.name,
                q.unit.clone().unwrap_or_default(),
                dist_name(&q.distribution, lang),
                dist_info(&q.distribution, lang)
            );
        }
        let _ = writeln!(s);
        if !spec.correlations.is_empty() {
            for c in &spec.correlations {
                let _ = writeln!(s, "- r({}, {}) = {}", c.a, c.b, fmt_given(c.r, lang));
            }
            let _ = writeln!(s);
        }
    }
    for g in &spec.indications {
        let _ = writeln!(s, "## {}\n", d.indications);
        let units = |j: usize| g.units.get(j).map(|u| format!(" [{u}]")).unwrap_or_default();
        let mut head = "| |".to_string();
        let mut rule = "|---|".to_string();
        for (j, n) in g.inputs.iter().enumerate() {
            head += &format!(" {n}{} |", units(j));
            rule += "---:|";
        }
        let _ = writeln!(s, "{head}\n{rule}");
        for (i, row) in g.values.iter().enumerate() {
            let cells: String = row.iter().map(|v| format!(" {} |", fmt_given(*v, lang))).collect();
            let _ = writeln!(s, "| {} |{cells}", i + 1);
        }
        let idx: Vec<usize> =
            g.inputs.iter().filter_map(|n| a.model.inputs.iter().position(|q| &q.name == n)).collect();
        let ux = a.model.input_covariance();
        let est: String =
            idx.iter().map(|&i| format!(" {} |", fmt_num(a.model.inputs[i].estimate(), 6, lang))).collect();
        let unc: String = idx.iter().map(|&i| format!(" {} |", fmt_num(ux[i][i].sqrt(), 3, lang))).collect();
        let _ = writeln!(s, "| {} |{est}\n| {} |{unc}\n", l.estimate, l.std_unc);
        let names: Vec<String> = g.inputs.clone();
        let r: Vec<Vec<f64>> = idx.iter().map(|&i| idx.iter().map(|&j| a.model.correlation[i][j]).collect()).collect();
        let _ = writeln!(s, "{}:\n", d.correlation);
        matrix_table(&mut s, &names, &r, lang);
        let _ = writeln!(
            s,
            "{}\n",
            match g.covariance {
                crate::multivariate::CovarianceSpec::Mean => d.indications_mean,
                crate::multivariate::CovarianceSpec::T => d.indications_t,
            }
        );
    }
    if let Some(g) = &a.gum {
        let _ = writeln!(s, "## {}\n", d.gum);
        let _ = writeln!(s, "{}\n", d.gum_note);
        results_block(
            &mut s,
            a,
            &g.estimate,
            &g.standard_uncertainties,
            &g.correlation,
            &g.covariance,
            g.k_ellipsoid,
            g.k_rectangle,
            g.coverage_probability,
            lang,
        );
        let _ = writeln!(s, "{}:\n", d.sensitivity);
        let inputs: Vec<String> = a.model.inputs.iter().map(|q| q.name.clone()).collect();
        let mut head = "| |".to_string();
        let mut rule = "|---|".to_string();
        for n in &inputs {
            head += &format!(" {n} |");
            rule += "---:|";
        }
        let _ = writeln!(s, "{head}\n{rule}");
        for (j, o) in a.model.outputs.iter().enumerate() {
            let cells: String = g.sensitivity[j].iter().map(|v| format!(" {} |", fmt_num(*v, 4, lang))).collect();
            let _ = writeln!(s, "| {} |{cells}", o.name);
        }
        let _ = writeln!(s);
    }
    if let Some(m) = &a.mcm {
        let _ = writeln!(s, "## {}\n", d.mcm);
        let how = if m.runs > 1 { d.adaptive.replace("{h}", &m.runs.to_string()) } else { String::new() };
        let _ = writeln!(
            s,
            "M = {} {}{}; {} {}.\n",
            m.trials,
            d.trials,
            if how.is_empty() { String::new() } else { format!(", {how}") },
            d.seed,
            m.seed
        );
        if !m.converged {
            let _ = writeln!(s, "**{}**\n", d.not_converged);
        }
        let _ = writeln!(s, "{}\n", d.mcm_note);
        results_block(
            &mut s,
            a,
            &m.estimate,
            &m.standard_uncertainties,
            &m.correlation,
            &m.covariance,
            m.k_ellipsoid,
            m.k_rectangle,
            m.coverage_probability,
            lang,
        );
        if let Some(area) = m.smallest_region_area {
            let _ = writeln!(s, "{} {}\n", d.smallest, fmt_num(area, 3, lang));
        }
        if let (Some(e), Some(r)) = (m.gum_ellipsoid_coverage, m.gum_rectangle_coverage) {
            let _ = writeln!(
                s,
                "{}\n",
                d.gum_coverage.replace("{e}", &fmt_pct(e, 3, lang, false)).replace("{r}", &fmt_pct(r, 3, lang, false))
            );
        }
    }
    if let Some(v) = &a.validation {
        let _ = writeln!(s, "## {}\n", d.validation);
        let _ = writeln!(s, "{}\n", d.validation_note.replace("{n}", &v.ndig.to_string()));
        let _ = writeln!(s, "| {} | {} | {} | |\n|---|---:|---:|---|", d.quantity, d.difference, d.tolerance);
        let ok = |x: bool| if x { "✓" } else { "✗" };
        for (j, o) in a.model.outputs.iter().enumerate() {
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} |",
                o.name,
                fmt_num(v.d_estimate[j], 2, lang),
                fmt_num(v.delta[j], 1, lang),
                ok(v.d_estimate[j] <= v.delta[j])
            );
            let _ = writeln!(
                s,
                "| u({}) | {} | {} | {} |",
                o.name,
                fmt_num(v.d_uncertainty[j], 2, lang),
                fmt_num(v.delta[j], 1, lang),
                ok(v.d_uncertainty[j] <= v.delta[j])
            );
        }
        if a.model.n_outputs() > 1 {
            let _ = writeln!(
                s,
                "| λ_max | {} | {} | {} |",
                fmt_num(v.d_lambda, 2, lang),
                fmt_num(v.rho, 1, lang),
                ok(v.d_lambda <= v.rho)
            );
            let _ = writeln!(
                s,
                "| k_p | {} | {} | {} |",
                fmt_num(v.d_kp, 2, lang),
                fmt_num(v.kappa_p, 1, lang),
                ok(v.d_kp <= v.kappa_p)
            );
            let _ = writeln!(
                s,
                "| k_q | {} | {} | {} |",
                fmt_num(v.d_kq, 2, lang),
                fmt_num(v.kappa_q, 1, lang),
                ok(v.d_kq <= v.kappa_q)
            );
        }
        let _ = writeln!(s);
        let word = |x: bool| if x { d.validated } else { d.not_validated };
        let _ = writeln!(s, "- {}: {}", d.ellipsoid, word(v.ellipsoid_validated));
        let _ = writeln!(s, "- {}: {}\n", d.rectangle, word(v.rectangle_validated));
    }
    for st in statements(a, lang) {
        let _ = writeln!(s, "**{}:** {st}\n", l.reported);
    }
    tail(&mut s, p, checks, lang, "JCGM 102:2011", generated);
    s
}

/// Points of an ellipse (η − y)ᵀ U⁻¹ (η − y) = k² in the plane.
fn ellipse_points(y: &[f64], cov: &[Vec<f64>], k: f64) -> Vec<(f64, f64)> {
    let l11 = cov[0][0].max(0.0).sqrt();
    let l21 = if l11 > 0.0 { cov[1][0] / l11 } else { 0.0 };
    let l22 = (cov[1][1] - l21 * l21).max(0.0).sqrt();
    (0..=96)
        .map(|i| {
            let t = 2.0 * std::f64::consts::PI * i as f64 / 96.0;
            let (c, s) = (k * t.cos(), k * t.sin());
            (y[0] + l11 * c, y[1] + l21 * c + l22 * s)
        })
        .collect()
}

/// Scatter of the Monte Carlo draws of a bivariate model with the coverage
/// regions (SVG).
pub(super) fn regions_svg(a: &MultivariateAnalysis, lang: Lang) -> Option<String> {
    if a.model.n_outputs() != 2 {
        return None;
    }
    let mut curves: Vec<(Vec<(f64, f64)>, &str)> = Vec::new();
    if let Some(g) = &a.gum {
        curves.push((ellipse_points(&g.estimate, &g.covariance, g.k_ellipsoid), "gumline"));
        let (y, u, k) = (&g.estimate, &g.standard_uncertainties, g.k_rectangle);
        curves.push((
            vec![
                (y[0] - k * u[0], y[1] - k * u[1]),
                (y[0] + k * u[0], y[1] - k * u[1]),
                (y[0] + k * u[0], y[1] + k * u[1]),
                (y[0] - k * u[0], y[1] + k * u[1]),
                (y[0] - k * u[0], y[1] - k * u[1]),
            ],
            "mcm",
        ));
    }
    if let Some(m) = &a.mcm {
        curves.push((ellipse_points(&m.estimate, &m.covariance, m.k_ellipsoid), "pdf"));
    }
    let all: Vec<(f64, f64)> =
        curves.iter().flat_map(|c| c.0.iter().copied()).chain(a.points.iter().map(|p| (p[0], p[1]))).collect();
    if all.is_empty() {
        return None;
    }
    let (mut x0, mut x1, mut y0, mut y1) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    for (x, y) in &all {
        if x.is_finite() && y.is_finite() {
            x0 = x0.min(*x);
            x1 = x1.max(*x);
            y0 = y0.min(*y);
            y1 = y1.max(*y);
        }
    }
    if !(x1 > x0 && y1 > y0) {
        return None;
    }
    let (w, h, left, top, bottom) = (640.0, 420.0, 60.0, 10.0, 30.0);
    let plot_w = w - left - 12.0;
    let plot_h = h - top - bottom;
    let px = |x: f64| left + plot_w * (x - x0) / (x1 - x0);
    let py = |y: f64| top + plot_h * (y1 - y) / (y1 - y0);
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(mv_labels(lang).chart)
    );
    let _ = write!(
        s,
        r#"<line x1="{left}" x2="{:.1}" y1="{:.1}" y2="{:.1}" class="axis"/><line x1="{left}" x2="{left}" y1="{top}" y2="{:.1}" class="axis"/>"#,
        left + plot_w,
        top + plot_h,
        top + plot_h,
        top + plot_h
    );
    for (v, t) in [(x0, "start"), (x1, "end")] {
        let _ = write!(
            s,
            r#"<text x="{:.1}" y="{:.1}" text-anchor="{t}" class="val">{}</text>"#,
            px(v),
            h - 8.0,
            fmt_num(v, 3, lang)
        );
    }
    for v in [y0, y1] {
        let _ = write!(
            s,
            r#"<text x="{:.1}" y="{:.1}" text-anchor="end" class="val">{}</text>"#,
            left - 4.0,
            py(v) + 4.0,
            fmt_num(v, 3, lang)
        );
    }
    let names = a.output_names();
    let _ = write!(
        s,
        r#"<text x="{:.1}" y="{:.1}" text-anchor="middle" class="lbl">{}</text>"#,
        left + plot_w / 2.0,
        h - 8.0,
        esc(&names[0])
    );
    let _ = write!(s, r#"<text x="{:.1}" y="{:.1}" class="lbl">{}</text>"#, left + 4.0, top + 12.0, esc(&names[1]));
    for p in &a.points {
        let _ = write!(s, r#"<circle cx="{:.1}" cy="{:.1}" r="1.2" class="hist"/>"#, px(p[0]), py(p[1]));
    }
    for (pts, class) in &curves {
        let path: Vec<String> = pts.iter().map(|(x, y)| format!("{:.1},{:.1}", px(*x), py(*y))).collect();
        let _ = write!(s, r#"<polyline points="{}" class="{class}" fill="none"/>"#, path.join(" "));
    }
    s.push_str("</svg>");
    Some(s)
}

/// Standard uncertainties of the outputs, GUM and Monte Carlo side by side
/// (SVG); used when the model does not have exactly two outputs.
pub(super) fn uncertainties_svg(a: &MultivariateAnalysis, lang: Lang) -> Option<String> {
    let m = a.model.n_outputs();
    let g = a.gum.as_ref().map(|g| g.standard_uncertainties.clone());
    let mc = a.mcm.as_ref().map(|m| m.standard_uncertainties.clone());
    let top = g.iter().chain(mc.iter()).flatten().copied().fold(0.0_f64, f64::max);
    if m == 0 || top.is_nan() || top <= 0.0 {
        return None;
    }
    let (w, row, left) = (640.0, 22.0, 110.0);
    let h = row * m as f64 + 24.0;
    let plot_w = w - left - 70.0;
    let mut s = format!(
        r#"<svg viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{}" xmlns="http://www.w3.org/2000/svg">"#,
        esc(mv_labels(lang).u_chart)
    );
    for (j, o) in a.model.outputs.iter().enumerate() {
        let y = 6.0 + j as f64 * row;
        let _ = write!(
            s,
            r#"<text x="{}" y="{:.1}" text-anchor="end" class="lbl">{}</text>"#,
            left - 8.0,
            y + 13.0,
            esc(&o.name)
        );
        let mut dy = 0.0;
        for (vals, class) in [(&g, "bar"), (&mc, "hist")] {
            if let Some(v) = vals {
                let bw = plot_w * v[j] / top;
                let _ = write!(
                    s,
                    r#"<rect x="{left}" y="{:.1}" width="{:.1}" height="8" class="{class}"/>"#,
                    y + dy,
                    bw.max(0.5)
                );
                dy += 9.0;
            }
        }
        let shown = mc.as_ref().or(g.as_ref()).map(|v| v[j]).unwrap_or(f64::NAN);
        let _ = write!(
            s,
            r#"<text x="{:.1}" y="{:.1}" class="val">{}</text>"#,
            left + plot_w + 6.0,
            y + 13.0,
            fmt_num(shown, 2, lang)
        );
    }
    s.push_str("</svg>");
    Some(s)
}
