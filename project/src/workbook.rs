//! Spreadsheet verification workbooks: an .xlsx file that recomputes a
//! result of KAPSAM with ordinary spreadsheet formulas, so that the user (or
//! an assessor) can check it in Excel or LibreOffice Calc, independently of
//! this software (ISO/IEC 17025:2017 7.11.2, validation of software).
//!
//! KAPSAM writes the data of the project and the formulas; the spreadsheet
//! program computes every number. Its own results are written beside them as
//! fixed values, with the difference and a tolerance (sheet *Check*). Nothing
//! in the workbook is evaluated by KAPSAM: a formula that KAPSAM got wrong
//! would show up as a difference.
//!
//! Covered: measurement models with one output (GUM law of propagation of
//! uncertainty, with correlations and intermediate quantities). The
//! sensitivity coefficients are central differences with a step of
//! 0.001 u(xᵢ) (the approach of Kragten's spreadsheet, Eurachem/CITAC QUAM:2012
//! Appendix E, with a smaller step), so the check is also independent of
//! KAPSAM's symbolic differentiation. Monte Carlo results are not covered.

use crate::schema::{DistributionSpec, InputSpec, Project};
use crate::xlsx::{abs_ref, cell_ref, number, sheet_ref, Sheet, Style, Workbook};
use crate::{Analysis, Evaluation, Lang, ProjectError};
use uncertainty_core::expr::{self, Expr, Func};
use uncertainty_core::gum::{CoverageSpec, DofRounding};

/// Relative step of the central differences, h = STEP·u(xᵢ).
const STEP: f64 = 1e-3;

/// True if a verification workbook can be written for the project.
pub fn supported(p: &Project, e: &Evaluation) -> bool {
    match e {
        Evaluation::Model(a) => p.model.as_ref().is_some_and(|m| m.ensembles.is_empty()) && a.gum.is_some(),
        _ => false,
    }
}

/// The verification workbook of an evaluated project (.xlsx bytes).
pub fn verification_workbook(p: &Project, e: &Evaluation, lang: Lang) -> Result<Vec<u8>, ProjectError> {
    if !supported(p, e) {
        return Err(ProjectError::Format(
            t(
                lang,
                "a spreadsheet verification is available for measurement models with a GUM evaluation",
                "elektronik tablo doğrulaması GUM değerlendirmesi olan ölçüm modelleri için yapılabilir",
            )
            .into(),
        ));
    }
    let Evaluation::Model(a) = e else { unreachable!() };
    model_workbook(p, a, lang).map(|wb| wb.to_bytes())
}

fn t<'a>(lang: Lang, en: &'a str, tr: &'a str) -> &'a str {
    match lang {
        Lang::En => en,
        Lang::Tr => tr,
    }
}

/// Spreadsheet formula of an expression; `refs[i]` is the cell of variable i.
pub fn excel(e: &Expr, refs: &[String]) -> String {
    match e {
        Expr::Num(v) => number(*v),
        Expr::Var(i) => refs[*i].clone(),
        Expr::Neg(a) => format!("(-{})", excel(a, refs)),
        Expr::Add(a, b) => format!("({}+{})", excel(a, refs), excel(b, refs)),
        Expr::Sub(a, b) => format!("({}-{})", excel(a, refs), excel(b, refs)),
        Expr::Mul(a, b) => format!("({}*{})", excel(a, refs), excel(b, refs)),
        Expr::Div(a, b) => format!("({}/{})", excel(a, refs), excel(b, refs)),
        Expr::Pow(a, b) => format!("POWER({},{})", excel(a, refs), excel(b, refs)),
        Expr::Call(f, a) => {
            let name = match f {
                Func::Sqrt => "SQRT",
                Func::Exp => "EXP",
                Func::Ln => "LN",
                Func::Log10 => "LOG10",
                Func::Sin => "SIN",
                Func::Cos => "COS",
                Func::Tan => "TAN",
                Func::Asin => "ASIN",
                Func::Acos => "ACOS",
                Func::Atan => "ATAN",
                Func::Sinh => "SINH",
                Func::Cosh => "COSH",
                Func::Tanh => "TANH",
                Func::Abs => "ABS",
                Func::Sign => "SIGN",
            };
            format!("{name}({})", excel(a, refs))
        }
        // The spreadsheet's ATAN2 takes (x, y).
        Expr::Atan2(y, x) => format!("ATAN2({},{})", excel(x, refs), excel(y, refs)),
    }
}

/// Rows of the sheet *Check*: a quantity, KAPSAM's value, the spreadsheet's
/// formula, the kind of difference and the tolerance.
struct Checks {
    rows: Vec<(String, f64, String, Diff, f64)>,
}

enum Diff {
    /// |S/K − 1| (|S| if K = 0).
    Relative,
    /// |S − K| / scale, with the scale a cell reference.
    Scaled(String),
}

impl Checks {
    fn add(&mut self, what: impl Into<String>, kapsam: f64, formula: impl Into<String>, diff: Diff, tol: f64) {
        if kapsam.is_finite() {
            self.rows.push((what.into(), kapsam, formula.into(), diff, tol));
        }
    }

    /// Writes the sheet and returns the cell with the number of failed rows.
    fn write(&self, sh: &mut Sheet, lang: Lang) -> String {
        sh.text(
            0,
            0,
            t(
                lang,
                "Comparison of KAPSAM's results with the spreadsheet",
                "KAPSAM sonuçlarının elektronik tabloyla karşılaştırılması",
            ),
            Style::Title,
        );
        sh.text(
            1,
            0,
            t(
                lang,
                "Column B was written by KAPSAM; column C is calculated by this spreadsheet. A row passes when the difference (D) is at most the tolerance (E).",
                "B sütununu KAPSAM yazdı; C sütununu bu elektronik tablo hesaplar. Fark (D) toleransı (E) aşmıyorsa satır uygundur.",
            ),
            Style::Note,
        );
        let heads = [
            t(lang, "Quantity", "Büyüklük"),
            "KAPSAM",
            t(lang, "Spreadsheet", "Elektronik tablo"),
            t(lang, "Difference", "Fark"),
            t(lang, "Tolerance", "Tolerans"),
            t(lang, "Result", "Sonuç"),
            t(lang, "Difference is", "Fark türü"),
        ];
        for (c, h) in heads.iter().enumerate() {
            sh.text(3, c as u32, *h, Style::Heading);
        }
        let (ok, fail) = (t(lang, "OK", "UYGUN"), t(lang, "FAIL", "UYGUN DEĞİL"));
        let first = 4u32;
        for (k, (what, kap, f, diff, tol)) in self.rows.iter().enumerate() {
            let r = first + k as u32;
            let (b, c) = (cell_ref(r, 1), cell_ref(r, 2));
            sh.text(r, 0, what.clone(), Style::Normal);
            sh.number(r, 1, *kap, Style::Number);
            sh.formula(r, 2, f.clone(), Style::Number);
            match diff {
                Diff::Relative => {
                    sh.formula(r, 3, format!("IF({b}=0,ABS({c}),ABS({c}/{b}-1))"), Style::Number);
                    sh.text(r, 6, t(lang, "relative", "bağıl"), Style::Note);
                }
                Diff::Scaled(s) => {
                    sh.formula(r, 3, format!("ABS({c}-{b})/({s})"), Style::Number);
                    sh.text(r, 6, t(lang, "relative to u(y)", "u(y)'ye göre"), Style::Note);
                }
            }
            sh.number(r, 4, *tol, Style::Number);
            sh.formula(r, 5, format!("IF({}<={},\"{ok}\",\"{fail}\")", cell_ref(r, 3), cell_ref(r, 4)), Style::Bold);
        }
        let last = first + self.rows.len().max(1) as u32 - 1;
        let n = last + 2;
        sh.text(n, 0, t(lang, "Rows outside the tolerance", "Toleransı aşan satırlar"), Style::Bold);
        sh.formula(
            n,
            2,
            format!(
                "SUMPRODUCT(--({}:{}>{}:{}))",
                cell_ref(first, 3),
                cell_ref(last, 3),
                cell_ref(first, 4),
                cell_ref(last, 4)
            ),
            Style::Bold,
        );
        for (c, w) in [(0, 42.0), (1, 20.0), (2, 20.0), (3, 18.0), (4, 12.0), (5, 14.0), (6, 16.0)] {
            sh.width(c, w);
        }
        format!("'{}'!{}", sh.name, abs_ref(n, 2))
    }
}

/// Parameters, formulas for x and u(x), and ν of one input.
struct InputCells {
    params: Vec<(&'static str, f64)>,
    x: String,
    u: String,
    nu: Option<String>,
    note: &'static str,
    observations: Vec<f64>,
}

const PCOL: [u32; 4] = [3, 5, 7, 9];

/// Labelled parameters of an input and the formula of its standard uncertainty.
type Params = (Vec<(&'static str, f64)>, String);
const OBS_COL: u32 = 15;

fn input_cells(q: &InputSpec, row: u32, lang: Lang) -> Result<InputCells, ProjectError> {
    let p = |k: usize| cell_ref(row, PCOL[k]);
    let lim = |lower: Option<f64>, upper: Option<f64>, value: Option<f64>, half: Option<f64>| match (
        lower, upper, value, half,
    ) {
        (Some(l), Some(u), None, None) => Ok((l, u)),
        (None, None, Some(v), Some(a)) => Ok((v - a, v + a)),
        _ => Err(ProjectError::Input { name: q.name.clone(), message: "limits".into() }),
    };
    let sd_or_u = |sd: Option<f64>, expanded: Option<f64>, k: Option<f64>| -> Result<Params, ProjectError> {
        match (sd, expanded, k) {
            (Some(s), None, None) => Ok((vec![("u", s)], p(1))),
            (None, Some(u), Some(k)) => Ok((vec![("U", u), ("k", k)], format!("{}/{}", p(1), p(2)))),
            _ => Err(ProjectError::Input { name: q.name.clone(), message: "uncertainty".into() }),
        }
    };
    let mut c = match &q.distribution {
        DistributionSpec::Constant { value } => InputCells {
            params: vec![(t(lang, "value", "değer"), *value)],
            x: p(0),
            u: "0".into(),
            nu: None,
            note: t(lang, "constant", "sabit"),
            observations: vec![],
        },
        DistributionSpec::Normal { mean, sd, expanded, k } => {
            let (ps, u) = sd_or_u(*sd, *expanded, *k)?;
            let mut params = vec![(t(lang, "mean", "ortalama"), *mean)];
            params.extend(ps);
            InputCells {
                params,
                x: p(0),
                u,
                nu: None,
                note: t(lang, "normal: u, or U/k (GUM 4.3.4)", "normal: u ya da U/k (GUM 4.3.4)"),
                observations: vec![],
            }
        }
        DistributionSpec::T { mean, scale, expanded, k, dof } => {
            let (ps, u) = sd_or_u(*scale, *expanded, *k)?;
            let mut params = vec![(t(lang, "mean", "ortalama"), *mean)];
            params.extend(ps);
            let nu_col = params.len();
            params.push(("ν", *dof));
            InputCells {
                params,
                x: p(0),
                u,
                nu: Some(p(nu_col)),
                note: t(lang, "t: scale (u or U/k) with ν (GUM G.4)", "t: ölçek (u ya da U/k) ve ν (GUM G.4)"),
                observations: vec![],
            }
        }
        DistributionSpec::Rectangular { lower, upper, value, half_width } => {
            let (l, u) = lim(*lower, *upper, *value, *half_width)?;
            InputCells {
                params: vec![(t(lang, "lower", "alt"), l), (t(lang, "upper", "üst"), u)],
                x: format!("({}+{})/2", p(0), p(1)),
                u: format!("({}-{})/SQRT(12)", p(1), p(0)),
                nu: None,
                note: t(lang, "rectangular: a/√3 (GUM 4.3.7)", "dikdörtgen: a/√3 (GUM 4.3.7)"),
                observations: vec![],
            }
        }
        DistributionSpec::Triangular { lower, upper, value, half_width } => {
            let (l, u) = lim(*lower, *upper, *value, *half_width)?;
            InputCells {
                params: vec![(t(lang, "lower", "alt"), l), (t(lang, "upper", "üst"), u)],
                x: format!("({}+{})/2", p(0), p(1)),
                u: format!("({}-{})/SQRT(24)", p(1), p(0)),
                nu: None,
                note: t(lang, "triangular: a/√6 (GUM 4.3.9)", "üçgen: a/√6 (GUM 4.3.9)"),
                observations: vec![],
            }
        }
        DistributionSpec::Arcsine { lower, upper, value, half_width } => {
            let (l, u) = lim(*lower, *upper, *value, *half_width)?;
            InputCells {
                params: vec![(t(lang, "lower", "alt"), l), (t(lang, "upper", "üst"), u)],
                x: format!("({}+{})/2", p(0), p(1)),
                u: format!("({}-{})/SQRT(8)", p(1), p(0)),
                nu: None,
                note: t(
                    lang,
                    "arcsine (U-shaped): a/√2 (JCGM 101 6.4.6)",
                    "arksinüs (U biçimli): a/√2 (JCGM 101 6.4.6)",
                ),
                observations: vec![],
            }
        }
        DistributionSpec::Trapezoidal { lower, upper, value, half_width, beta } => {
            let (l, u) = lim(*lower, *upper, *value, *half_width)?;
            InputCells {
                params: vec![(t(lang, "lower", "alt"), l), (t(lang, "upper", "üst"), u), ("β", *beta)],
                x: format!("({}+{})/2", p(0), p(1)),
                u: format!("({}-{})*SQRT((1+{}^2)/24)", p(1), p(0), p(2)),
                nu: None,
                note: t(lang, "trapezoidal (GUM 4.3.9, JCGM 101 6.4.4)", "yamuk (GUM 4.3.9, JCGM 101 6.4.4)"),
                observations: vec![],
            }
        }
        DistributionSpec::CurvilinearTrapezoid { lower, upper, value, half_width, d } => {
            let (l, u) = lim(*lower, *upper, *value, *half_width)?;
            InputCells {
                params: vec![(t(lang, "lower", "alt"), l), (t(lang, "upper", "üst"), u), ("d", *d)],
                x: format!("({}+{})/2", p(0), p(1)),
                u: format!("SQRT(({}-{})^2/12+{}^2/9)", p(1), p(0), p(2)),
                nu: None,
                note: t(lang, "curvilinear trapezoid (JCGM 101 6.4.3)", "eğrisel yamuk (JCGM 101 6.4.3)"),
                observations: vec![],
            }
        }
        DistributionSpec::Exponential { mean } => InputCells {
            params: vec![(t(lang, "mean", "ortalama"), *mean)],
            x: p(0),
            u: p(0),
            nu: None,
            note: t(lang, "exponential: u = mean (JCGM 101 6.4.10)", "üstel: u = ortalama (JCGM 101 6.4.10)"),
            observations: vec![],
        },
        DistributionSpec::Gamma { shape, scale } => InputCells {
            params: vec![(t(lang, "shape", "biçim"), *shape), (t(lang, "scale", "ölçek"), *scale)],
            x: format!("{}*{}", p(0), p(1)),
            u: format!("SQRT({})*{}", p(0), p(1)),
            nu: None,
            note: t(lang, "gamma: x = αθ, u = √α θ (JCGM 101 6.4.11)", "gama: x = αθ, u = √α θ (JCGM 101 6.4.11)"),
            observations: vec![],
        },
        DistributionSpec::Beta { alpha, beta, mean, sd, lower, upper } => {
            let (a, b) = (lower.unwrap_or(0.0), upper.unwrap_or(1.0));
            match (alpha, beta, mean, sd) {
                (Some(al), Some(be), None, None) => InputCells {
                    params: vec![("α", *al), ("β", *be), (t(lang, "lower", "alt"), a), (t(lang, "upper", "üst"), b)],
                    x: format!("{2}+({3}-{2})*{0}/({0}+{1})", p(0), p(1), p(2), p(3)),
                    u: format!("({3}-{2})*SQRT({0}*{1}/(({0}+{1})^2*({0}+{1}+1)))", p(0), p(1), p(2), p(3)),
                    nu: None,
                    note: t(lang, "beta (JCGM 101 6.4.8)", "beta (JCGM 101 6.4.8)"),
                    observations: vec![],
                },
                (None, None, Some(m), Some(s)) => InputCells {
                    params: vec![(t(lang, "mean", "ortalama"), *m), ("u", *s)],
                    x: p(0),
                    u: p(1),
                    nu: None,
                    note: t(lang, "beta from its mean and standard deviation", "ortalama ve standart sapmasından beta"),
                    observations: vec![],
                },
                _ => return Err(ProjectError::Input { name: q.name.clone(), message: "beta".into() }),
            }
        }
        DistributionSpec::TypeA { observations } => {
            let range =
                format!("{}:{}", cell_ref(row, OBS_COL), cell_ref(row, OBS_COL + observations.len() as u32 - 1));
            InputCells {
                params: vec![("n", observations.len() as f64)],
                x: format!("AVERAGE({range})"),
                u: format!("STDEV({range})/SQRT(COUNT({range}))"),
                nu: Some(format!("COUNT({range})-1")),
                note: t(
                    lang,
                    "Type A: mean, s/√n, ν = n − 1 (GUM 4.2); observations from column P",
                    "A tipi: ortalama, s/√n, ν = n − 1 (GUM 4.2); gözlemler P sütunundan itibaren",
                ),
                observations: observations.clone(),
            }
        }
        DistributionSpec::TypeAPooled { mean, pooled_sd, pooled_dof, n } => InputCells {
            params: vec![
                (t(lang, "mean", "ortalama"), *mean),
                ("s_p", *pooled_sd),
                ("ν_p", *pooled_dof),
                ("n", *n as f64),
            ],
            x: p(0),
            u: format!("{}/SQRT({})", p(1), p(3)),
            nu: Some(p(2)),
            note: t(lang, "pooled Type A: s_p/√n (GUM H.3.6)", "birleştirilmiş A tipi: s_p/√n (GUM H.3.6)"),
            observations: vec![],
        },
    };
    if let Some(nu) = q.dof {
        c.nu = Some(number(nu));
    }
    Ok(c)
}

fn model_workbook(p: &Project, a: &Analysis, lang: Lang) -> Result<Workbook, ProjectError> {
    let m = p.model.as_ref().expect("model project");
    let gum = a.gum.as_ref().expect("GUM result");
    let n = m.inputs.len();
    let ni = m.intermediates.len();
    let gum_spec = p.analysis.gum.clone().unwrap_or_default();

    // Parse the intermediates and the output with variables 0..n (inputs) and
    // n..n+ni (intermediates), so that each becomes a row of the sheet Model.
    let mut names: Vec<String> = m.inputs.iter().map(|q| q.name.clone()).collect();
    let mut exprs = Vec::new();
    for q in m
        .intermediates
        .iter()
        .map(|i| (&i.name, &i.expression))
        .chain(std::iter::once((&m.output.name, &m.output.expression)))
    {
        let known = names.clone();
        let e = expr::parse(q.1, &|s: &str| known.iter().position(|k| k == s).map(Expr::Var))
            .map_err(|e| ProjectError::Format(format!("{}: {e}", q.0)))?;
        exprs.push(e);
        names.push(q.0.clone());
    }

    let s_inputs = t(lang, "Inputs", "Girdiler");
    let s_model = "Model";
    let s_budget = t(lang, "Budget", "Bütçe");
    let s_check = t(lang, "Check", "Karşılaştırma");

    // --- Inputs ---------------------------------------------------------------
    let mut si = Sheet::new(s_inputs);
    si.text(
        0,
        0,
        t(lang, "Input quantities (data of the project file)", "Girdi büyüklükleri (proje dosyasının verileri)"),
        Style::Title,
    );
    let heads = [
        t(lang, "Quantity", "Büyüklük"),
        t(lang, "Distribution", "Dağılım"),
        "",
        t(lang, "Parameter 1", "Parametre 1"),
        "",
        t(lang, "Parameter 2", "Parametre 2"),
        "",
        t(lang, "Parameter 3", "Parametre 3"),
        "",
        t(lang, "Parameter 4", "Parametre 4"),
        "xᵢ",
        "u(xᵢ)",
        "νᵢ",
        t(lang, "How u(xᵢ) is obtained", "u(xᵢ) nasıl bulunur"),
    ];
    for (c, h) in heads.iter().enumerate() {
        si.text(2, c as u32, *h, Style::Heading);
    }
    si.text(2, OBS_COL, t(lang, "Observations (Type A)", "Gözlemler (A tipi)"), Style::Heading);
    let in_row = |i: usize| 3 + i as u32;
    for (i, q) in m.inputs.iter().enumerate() {
        let r = in_row(i);
        let c = input_cells(q, r, lang)?;
        si.text(r, 0, q.name.clone(), Style::Bold);
        si.text(r, 1, distribution_name(&q.distribution), Style::Normal);
        for (k, (label, v)) in c.params.iter().enumerate() {
            si.text(r, PCOL[k] - 1, *label, Style::Note);
            si.number(r, PCOL[k], *v, Style::Number);
        }
        si.formula(r, 10, c.x, Style::Number);
        si.formula(r, 11, c.u, Style::Number);
        if let Some(nu) = c.nu {
            si.formula(r, 12, nu, Style::Normal);
        } else {
            si.text(r, 12, "∞", Style::Normal);
        }
        si.text(r, 13, c.note, Style::Note);
        for (k, v) in c.observations.iter().enumerate() {
            si.number(r, OBS_COL + k as u32, *v, Style::Normal);
        }
    }
    // Correlation matrix (data), rows and columns in the order of the inputs.
    let corr_row0 = in_row(n) + 2;
    let has_corr = !m.correlations.is_empty();
    if has_corr {
        si.text(
            corr_row0 - 1,
            0,
            t(lang, "Correlation coefficients r(xᵢ, xⱼ)", "Korelasyon katsayıları r(xᵢ, xⱼ)"),
            Style::Bold,
        );
        for (j, q) in m.inputs.iter().enumerate() {
            si.text(corr_row0, 1 + j as u32, q.name.clone(), Style::Heading);
        }
        for (i, qi) in m.inputs.iter().enumerate() {
            let r = corr_row0 + 1 + i as u32;
            si.text(r, 0, qi.name.clone(), Style::Heading);
            for (j, qj) in m.inputs.iter().enumerate() {
                let v = if i == j {
                    1.0
                } else {
                    m.correlations
                        .iter()
                        .find(|c| (c.a == qi.name && c.b == qj.name) || (c.a == qj.name && c.b == qi.name))
                        .map_or(0.0, |c| c.r)
                };
                si.number(r, 1 + j as u32, v, Style::Normal);
            }
        }
    }
    si.width(0, 18.0);
    si.width(1, 14.0);
    for c in [10, 11] {
        si.width(c, 18.0);
    }
    si.width(13, 44.0);
    let x_of = |i: usize| sheet_ref(s_inputs, in_row(i), 10);
    let u_of = |i: usize| sheet_ref(s_inputs, in_row(i), 11);
    let nu_of = |i: usize| sheet_ref(s_inputs, in_row(i), 12);

    // --- Model: one column at the estimates, two per input (±h) -----------------
    let mut sm = Sheet::new(s_model);
    sm.text(
        0,
        0,
        t(
            lang,
            "Measurement model evaluated at the estimates and at xᵢ ± h, h = 0.001 u(xᵢ)",
            "Ölçüm modeli, kestirimlerde ve xᵢ ± h noktalarında (h = 0,001 u(xᵢ))",
        ),
        Style::Title,
    );
    sm.text(2, 0, t(lang, "Quantity", "Büyüklük"), Style::Heading);
    sm.text(2, 1, t(lang, "Estimates", "Kestirimler"), Style::Heading);
    sm.text(3, 0, "h", Style::Bold);
    let row_of = |k: usize| 4 + k as u32; // variable k (input, intermediate or output)
    let out_row = row_of(n + ni);
    let col_plus = |i: usize| 2 + 2 * i as u32;
    for (i, q) in m.inputs.iter().enumerate() {
        sm.text(2, col_plus(i), format!("{} + h", q.name), Style::Heading);
        sm.text(2, col_plus(i) + 1, format!("{} − h", q.name), Style::Heading);
        sm.formula(3, col_plus(i), format!("{}*{}", number(STEP), u_of(i)), Style::Number);
        sm.formula(3, col_plus(i) + 1, cell_ref(3, col_plus(i)), Style::Number);
    }
    for (k, name) in names.iter().enumerate() {
        sm.text(row_of(k), 0, name.clone(), if k == n + ni { Style::Bold } else { Style::Normal });
    }
    for col in 1..=(2 * n as u32 + 1) {
        let refs: Vec<String> = (0..n + ni).map(|k| cell_ref(row_of(k), col)).collect();
        for i in 0..n {
            let f = if col == 1 {
                x_of(i)
            } else {
                let j = ((col - 2) / 2) as usize;
                let h = abs_ref(3, col);
                match (i == j, (col - 2) % 2) {
                    (true, 0) => format!("{}+{h}", x_of(i)),
                    (true, _) => format!("{}-{h}", x_of(i)),
                    _ => x_of(i),
                }
            };
            sm.formula(row_of(i), col, f, Style::Number);
        }
        for (k, e) in exprs.iter().enumerate() {
            sm.formula(row_of(n + k), col, excel(e, &refs), if k == ni { Style::BoldNumber } else { Style::Number });
        }
    }
    let c_row = out_row + 2;
    sm.text(
        c_row,
        0,
        t(
            lang,
            "Sensitivity coefficient cᵢ = [y(xᵢ + h) − y(xᵢ − h)] / 2h",
            "Duyarlılık katsayısı cᵢ = [y(xᵢ + h) − y(xᵢ − h)] / 2h",
        ),
        Style::Bold,
    );
    for i in 0..n {
        let (cp, cm) = (col_plus(i), col_plus(i) + 1);
        sm.formula(
            c_row,
            cp,
            format!("IF({h}=0,0,({}-{})/(2*{h}))", cell_ref(out_row, cp), cell_ref(out_row, cm), h = cell_ref(3, cp)),
            Style::BoldNumber,
        );
    }
    sm.width(0, 22.0);
    for col in 1..=(2 * n as u32 + 1) {
        sm.width(col, 17.0);
    }
    let c_of = |i: usize| sheet_ref(s_model, c_row, col_plus(i));
    let y_cell = sheet_ref(s_model, out_row, 1);

    // --- Budget ---------------------------------------------------------------
    let mut sb = Sheet::new(s_budget);
    sb.text(
        0,
        0,
        t(
            lang,
            "Uncertainty budget: law of propagation of uncertainty (GUM 5.1.2, 5.2.2)",
            "Belirsizlik bütçesi: belirsizliğin yayılması yasası (GUM 5.1.2, 5.2.2)",
        ),
        Style::Title,
    );
    let bh = [
        t(lang, "Quantity", "Büyüklük"),
        "xᵢ",
        "u(xᵢ)",
        "cᵢ",
        "cᵢ u(xᵢ)",
        "νᵢ",
        "(cᵢ u(xᵢ))⁴ / νᵢ",
        t(lang, "Variance term", "Varyans terimi"),
    ];
    for (c, h) in bh.iter().enumerate() {
        sb.text(2, c as u32, *h, Style::Heading);
    }
    let b_row = |i: usize| 3 + i as u32;
    let cu_row = b_row(n) + 1; // c_i u(x_i) again as a row, for the correlation terms
    for (i, q) in m.inputs.iter().enumerate() {
        let r = b_row(i);
        sb.text(r, 0, q.name.clone(), Style::Bold);
        sb.formula(r, 1, x_of(i), Style::Number);
        sb.formula(r, 2, u_of(i), Style::Number);
        sb.formula(r, 3, c_of(i), Style::Number);
        sb.formula(r, 4, format!("{}*{}", cell_ref(r, 3), cell_ref(r, 2)), Style::Number);
        sb.formula(r, 5, nu_of(i), Style::Normal);
        sb.formula(
            r,
            6,
            format!("IF(ISNUMBER({nu}),{e}^4/{nu},0)", nu = cell_ref(r, 5), e = cell_ref(r, 4)),
            Style::Number,
        );
        let var = if has_corr {
            // cᵢuᵢ Σⱼ rᵢⱼ cⱼuⱼ (GUM 5.2.2)
            let corr = format!(
                "'{}'!{}:{}",
                s_inputs,
                abs_ref(corr_row0 + 1 + i as u32, 1),
                abs_ref(corr_row0 + 1 + i as u32, n as u32)
            );
            format!("{}*SUMPRODUCT({corr},{}:{})", cell_ref(r, 4), abs_ref(cu_row, 1), abs_ref(cu_row, n as u32))
        } else {
            format!("{}^2", cell_ref(r, 4))
        };
        sb.formula(r, 7, var, Style::Number);
        sb.formula(cu_row, 1 + i as u32, cell_ref(r, 4), Style::Number);
    }
    if has_corr {
        sb.text(cu_row, 0, "cᵢ u(xᵢ)", Style::Note);
    }
    let (first, last) = (b_row(0), b_row(n - 1));
    let rs = cu_row + 2;
    let label = |sb: &mut Sheet, r: u32, s: &str| sb.text(r, 0, s.to_string(), Style::Bold);
    label(&mut sb, rs, "y");
    sb.formula(rs, 1, y_cell.clone(), Style::BoldNumber);
    label(&mut sb, rs + 1, "u(y)");
    sb.formula(rs + 1, 1, format!("SQRT(SUM({}:{}))", cell_ref(first, 7), cell_ref(last, 7)), Style::BoldNumber);
    label(&mut sb, rs + 2, t(lang, "ν_eff (Welch–Satterthwaite, GUM G.4.1)", "ν_eff (Welch–Satterthwaite, GUM G.4.1)"));
    sb.formula(
        rs + 2,
        1,
        format!(
            "IF(SUM({g0}:{g1})=0,\"∞\",{u}^4/SUM({g0}:{g1}))",
            g0 = cell_ref(first, 6),
            g1 = cell_ref(last, 6),
            u = cell_ref(rs + 1, 1)
        ),
        Style::Number,
    );
    let u_ref = format!("'{}'!{}", s_budget, abs_ref(rs + 1, 1));
    let nu_ref = cell_ref(rs + 2, 1);
    // Coverage factor as KAPSAM is asked to obtain it.
    let opts = crate::gum_options(&gum_spec);
    let k_formula = match opts.coverage {
        CoverageSpec::Factor(k) => Some(number(k)),
        CoverageSpec::Probability(pr) => {
            let nu = match opts.dof_rounding {
                DofRounding::Truncate => format!("INT({nu_ref})"),
                DofRounding::Exact => nu_ref.clone(),
            };
            Some(format!(
                "IF(ISNUMBER({nu_ref}),SQRT({nu}*(1/BETAINV(1-{p},{nu}/2,0.5)-1)),NORMSINV((1+{p})/2))",
                p = number(pr)
            ))
        }
        CoverageSpec::DominantRectangular(_) => None,
    };
    if let Some(kf) = &k_formula {
        label(&mut sb, rs + 3, t(lang, "k (GUM G.6.4)", "k (GUM G.6.4)"));
        sb.formula(rs + 3, 1, kf.clone(), Style::Number);
        label(&mut sb, rs + 4, "U = k u(y)");
        sb.formula(rs + 4, 1, format!("{}*{}", cell_ref(rs + 3, 1), cell_ref(rs + 1, 1)), Style::BoldNumber);
    }
    sb.width(0, 34.0);
    for c in 1..8 {
        sb.width(c, 18.0);
    }

    // --- Check ----------------------------------------------------------------
    let mut ch = Checks { rows: Vec::new() };
    let row_by_name = |name: &str| gum.budget.iter().find(|b| b.name == name);
    for (i, q) in m.inputs.iter().enumerate() {
        if let Some(b) = row_by_name(&q.name) {
            ch.add(format!("x({})", q.name), b.estimate, x_of(i), Diff::Relative, 1e-10);
            ch.add(format!("u({})", q.name), b.standard_uncertainty, u_of(i), Diff::Relative, 1e-10);
        }
    }
    // Differences of c and y are taken relative to u(y) when u(y) > 0.
    let scaled = gum.first_order_uncertainty > 0.0;
    for (i, q) in m.inputs.iter().enumerate() {
        if let Some(b) = row_by_name(&q.name) {
            if b.standard_uncertainty > 0.0 {
                // The sensitivity coefficient through its contribution, relative to u(y).
                let d = if scaled { Diff::Scaled(format!("{u_ref}/{}", u_of(i))) } else { Diff::Relative };
                ch.add(format!("c({})", q.name), b.sensitivity, c_of(i), d, 1e-6);
            }
        }
    }
    let y0 = gum.estimate - if gum_spec.higher_order { gum.second_order_estimate_term } else { 0.0 };
    ch.add(
        "y",
        y0,
        sheet_ref(s_budget, rs, 1),
        if scaled { Diff::Scaled(u_ref.clone()) } else { Diff::Relative },
        1e-9,
    );
    ch.add("u(y)", gum.first_order_uncertainty, u_ref.clone(), Diff::Relative, 1e-6);
    // ν_eff, k and U of KAPSAM include the higher-order terms when they are requested.
    let wsat = !has_corr && !gum_spec.higher_order && gum.effective_dof.is_finite();
    if wsat {
        ch.add("ν_eff", gum.effective_dof, sheet_ref(s_budget, rs + 2, 1), Diff::Relative, 1e-5);
    }
    if k_formula.is_some() && !gum_spec.higher_order && (wsat || gum.effective_dof.is_infinite()) {
        ch.add("k", gum.coverage_factor, sheet_ref(s_budget, rs + 3, 1), Diff::Relative, 1e-6);
        ch.add("U", gum.expanded_uncertainty, sheet_ref(s_budget, rs + 4, 1), Diff::Relative, 1e-6);
    }
    let mut sc = Sheet::new(s_check);
    let fails = ch.write(&mut sc, lang);

    let info = info_sheet(p, lang, &fails, &[
        t(lang,
          "Inputs: the data of the project file. The estimates xᵢ and standard uncertainties u(xᵢ) are calculated by the spreadsheet from them (formulas in columns K and L).",
          "Girdiler: proje dosyasının verileri. Kestirimler xᵢ ve standart belirsizlikler u(xᵢ) bu verilerden elektronik tabloda hesaplanır (K ve L sütunlarındaki formüller)."),
        t(lang,
          "Model: the measurement model written as spreadsheet formulas and evaluated at the estimates and with each input shifted by ±h, h = 0.001 u(xᵢ); the sensitivity coefficients are the central differences (the idea of Kragten's spreadsheet, Eurachem/CITAC QUAM:2012 Appendix E).",
          "Model: ölçüm modeli elektronik tablo formülü olarak yazılmış ve kestirimlerde ve her girdi ±h kaydırılarak (h = 0,001 u(xᵢ)) hesaplanmıştır; duyarlılık katsayıları merkezi farklardır (Kragten tablosu yaklaşımı, Eurachem/CITAC QUAM:2012 Ek E)."),
        t(lang,
          "Budget: law of propagation of uncertainty with the correlation coefficients (GUM 5.1.2, 5.2.2), Welch–Satterthwaite formula (GUM G.4.1), coverage factor from the t-distribution (GUM G.6.4).",
          "Bütçe: korelasyon katsayılarıyla belirsizliğin yayılması yasası (GUM 5.1.2, 5.2.2), Welch–Satterthwaite formülü (GUM G.4.1), t dağılımından kapsam faktörü (GUM G.6.4)."),
        t(lang,
          "Check: KAPSAM's results (fixed numbers) beside the spreadsheet's. Monte Carlo results (JCGM 101) are not recomputed here.",
          "Karşılaştırma: KAPSAM sonuçları (sabit sayılar) ve elektronik tablonun sonuçları yan yana. Monte Carlo sonuçları (JCGM 101) burada yeniden hesaplanmaz."),
    ]);
    Ok(Workbook { sheets: vec![info, sc, si, sm, sb] })
}

fn distribution_name(d: &DistributionSpec) -> &'static str {
    match d {
        DistributionSpec::Constant { .. } => "constant",
        DistributionSpec::Normal { .. } => "normal",
        DistributionSpec::T { .. } => "t",
        DistributionSpec::Rectangular { .. } => "rectangular",
        DistributionSpec::Triangular { .. } => "triangular",
        DistributionSpec::Arcsine { .. } => "arcsine",
        DistributionSpec::Trapezoidal { .. } => "trapezoidal",
        DistributionSpec::CurvilinearTrapezoid { .. } => "curvilinear trapezoid",
        DistributionSpec::Exponential { .. } => "exponential",
        DistributionSpec::Gamma { .. } => "gamma",
        DistributionSpec::Beta { .. } => "beta",
        DistributionSpec::TypeA { .. } => "type A",
        DistributionSpec::TypeAPooled { .. } => "type A (pooled)",
    }
}

fn info_sheet(p: &Project, lang: Lang, fails: &str, how: &[&str]) -> Sheet {
    let mut s = Sheet::new(t(lang, "About", "Bilgi"));
    s.text(
        0,
        0,
        t(lang, "Spreadsheet verification of a KAPSAM result", "KAPSAM sonucunun elektronik tabloyla doğrulanması"),
        Style::Title,
    );
    s.text(2, 0, t(lang, "Project", "Proje"), Style::Bold);
    s.text(2, 1, p.metadata.title.get(lang).to_string(), Style::Normal);
    if let Some(id) = &p.metadata.id {
        s.text(3, 0, "ID", Style::Bold);
        s.text(3, 1, id.clone(), Style::Normal);
    }
    s.text(4, 0, "Software", Style::Bold);
    s.text(4, 1, format!("{} {}", crate::SOFTWARE_NAME, crate::software_identity()), Style::Normal);
    s.text(5, 0, t(lang, "Project digest (SHA-256)", "Proje özeti (SHA-256)"), Style::Bold);
    s.text(5, 1, crate::project_digest(p), Style::Normal);
    s.text(7, 0, t(lang, "Result of the check", "Karşılaştırmanın sonucu"), Style::Bold);
    s.formula(
        7,
        1,
        format!(
            "IF({fails}=0,\"{}\",{fails}&\"{}\")",
            t(lang, "All results agree within the tolerances", "Bütün sonuçlar toleranslar içinde uyumlu"),
            t(
                lang,
                " result(s) outside the tolerance: see the sheet Check",
                " sonuç toleransı aşıyor: Karşılaştırma sayfasına bakınız"
            )
        ),
        Style::Bold,
    );
    s.text(9, 0, t(lang, "How this workbook works", "Bu çalışma kitabı nasıl çalışır"), Style::Bold);
    s.text(
        10,
        0,
        t(
            lang,
            "KAPSAM wrote the data and the formulas; every number in columns marked as results is calculated by this spreadsheet program when the file is opened. KAPSAM's own results are written on the sheet Check as fixed numbers, so the comparison is independent of KAPSAM's calculations (ISO/IEC 17025:2017 7.11.2).",
            "Verileri ve formülleri KAPSAM yazdı; sonuç sütunlarındaki her sayıyı, dosya açıldığında bu elektronik tablo programı hesaplar. KAPSAM'ın kendi sonuçları Karşılaştırma sayfasına sabit sayılar olarak yazılmıştır; böylece karşılaştırma KAPSAM'ın hesaplarından bağımsızdır (ISO/IEC 17025:2017 7.11.2).",
        ),
        Style::Normal,
    );
    for (k, line) in how.iter().enumerate() {
        s.text(12 + k as u32, 0, format!("{}. {line}", k + 1), Style::Normal);
    }
    s.width(0, 30.0);
    s.width(1, 90.0);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formulas_keep_precedence_and_argument_order() {
        let names = ["a".to_string(), "b".to_string()];
        let e = expr::parse("-a^2 + atan2(b, a)/log10(b)", &|s: &str| names.iter().position(|k| k == s).map(Expr::Var))
            .unwrap();
        let refs = vec!["A1".to_string(), "B1".to_string()];
        let f = excel(&e, &refs);
        assert!(f.contains("POWER(A1,2E0)") && f.contains("ATAN2(A1,B1)") && f.contains("LOG10(B1)"), "{f}");
        assert!(f.starts_with("((-POWER"), "{f}");
    }

    #[test]
    fn model_workbook_has_the_checks() {
        let p = crate::parse(include_str!("../../library/chemistry/quam-a1-cadmium-standard.json")).unwrap();
        let e = crate::run(&p, &crate::Overrides { skip_mcm: true, ..Default::default() }).unwrap();
        assert!(supported(&p, &e));
        let Evaluation::Model(a) = &e else { panic!() };
        let wb = model_workbook(&p, a, Lang::Tr).unwrap();
        let names: Vec<&str> = wb.sheets.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["Bilgi", "Karşılaştırma", "Girdiler", "Model", "Bütçe"]);
        let check = &wb.sheets[1];
        let rows: Vec<String> = (4..40)
            .filter_map(|r| match check.get(r, 0) {
                Some(crate::xlsx::Cell::Text(s, _)) => Some(s.clone()),
                _ => None,
            })
            .collect();
        for w in ["x(P)", "u(V)", "c(m)", "y", "u(y)", "k", "U"] {
            assert!(rows.iter().any(|r| r == w), "{w} missing: {rows:?}");
        }
        // The model is written as a formula of the cells of its column.
        assert!(
            matches!(wb.sheets[3].get(7, 1), Some(crate::xlsx::Cell::Formula(f, _)) if f.contains("B5") && f.contains("B6") && f.contains("B7"))
        );
        assert!(verification_workbook(&p, &e, Lang::En).unwrap().starts_with(b"PK"));
    }
}
