//! Verification workbook of a top-down evaluation [Nordtest TR 537;
//! Eurachem/CITAC MUPR; ISO 21748]: the precision components from their data,
//! the bias components u(bias) = √(RMS_bias² + ū(C_ref)²), the further
//! components, and the combination at every level with absolute and relative
//! components (Nordtest §3, MUPR Eq. 19–20). The recovery component, the
//! staggered-nested design and the ISO 21748 reproducibility component are
//! entered with KAPSAM's values (marked) and are not recomputed.

use super::{info_sheet, t, Checks, Diff};
use crate::schema::Project;
use crate::topdown::{
    At, BiasKind, ComponentSource, DuplicateMethod, LevelRange, LimitDistribution, PrecisionData, TopDownAnalysis,
};
use crate::xlsx::{cell_ref, number, sheet_ref, Sheet, Style, Workbook};
use crate::{Lang, ProjectError};

/// A budget item as the sheet Levels needs it.
struct Item {
    name: String,
    /// Cell of the standard uncertainty (fraction if relative); None for a
    /// level-dependent model.
    u: Option<String>,
    relative: bool,
    multiplicity: u32,
    range: LevelRange,
    /// Formula of the absolute standard deviation at the level in cell `{x}`.
    model: Option<String>,
    recovery: bool,
}

fn in_range(r: &LevelRange, x: &str) -> String {
    let mut c = Vec::new();
    if let Some(m) = r.min {
        c.push(format!("{x}>={}", number(m)));
    }
    if let Some(m) = r.max {
        c.push(format!("{x}<{}", number(m)));
    }
    if c.is_empty() {
        "TRUE".into()
    } else {
        format!("AND({})", c.join(","))
    }
}

pub(super) fn workbook(p: &Project, a: &TopDownAnalysis, lang: Lang) -> Result<Workbook, ProjectError> {
    let spec = p.topdown.as_ref().ok_or_else(|| ProjectError::Format("no topdown section".into()))?;
    let s_comp = t(lang, "Components", "Bileşenler");
    let s_lev = t(lang, "Levels", "Düzeyler");
    let mut ch = Checks::default();
    let mut sc = Sheet::new(s_comp);
    let mut items: Vec<Item> = Vec::new();
    let fixed = t(lang, "KAPSAM's value (not recomputed)", "KAPSAM değeri (yeniden hesaplanmadı)");
    let at = |r: u32, c: u32| sheet_ref(s_comp, r, c);
    sc.text(0, 0, t(lang, "Components of the budget from their data", "Bütçe bileşenleri, verilerinden"), Style::Title);
    let mut r = 2u32;
    let label = |sc: &mut Sheet, r: u32, s: &str| sc.text(r, 0, s.to_string(), Style::Bold);

    // --- Precision --------------------------------------------------------------------
    for (ps, pr) in spec.precision.iter().zip(&a.precision) {
        sc.text(r, 0, format!("{} — {}", t(lang, "Precision", "Kesinlik"), ps.name), Style::Heading);
        sc.text(r, 1, format!("{:?}", pr.method), Style::Heading);
        r += 1;
        // (cell of the raw s, cell of the mean of the data, recomputed)
        let mut mean_cell: Option<String> = None;
        let mut model: Option<String> = None;
        let raw: Option<String> = match &ps.data {
            PrecisionData::Series { values, relative } => {
                label(&mut sc, r, t(lang, "results", "sonuçlar"));
                for (k, v) in values.iter().enumerate() {
                    sc.number(r, 1 + k as u32, *v, Style::Normal);
                }
                let rg = format!("{}:{}", cell_ref(r, 1), cell_ref(r, values.len() as u32));
                label(&mut sc, r + 1, t(lang, "mean", "ortalama"));
                sc.formula(r + 1, 1, format!("AVERAGE({rg})"), Style::Number);
                label(&mut sc, r + 2, if *relative { "s / |x̄|" } else { "s" });
                let s = format!("STDEV({rg})");
                sc.formula(
                    r + 2,
                    1,
                    if *relative { format!("{s}/ABS({})", cell_ref(r + 1, 1)) } else { s },
                    Style::Number,
                );
                mean_cell = Some(cell_ref(r + 1, 1));
                r += 3;
                Some(cell_ref(r - 1, 1))
            }
            PrecisionData::Groups { groups, relative } => {
                let heads = [t(lang, "group", "grup"), "s", "n", "(n − 1) s²", "n − 1", t(lang, "values", "değerler")];
                for (c, h) in heads.iter().enumerate() {
                    sc.text(r, c as u32, *h, Style::Heading);
                }
                let g0 = r + 1;
                for (k, g) in groups.iter().enumerate() {
                    let row = g0 + k as u32;
                    sc.text(row, 0, g.label.clone().unwrap_or_else(|| format!("{}", k + 1)), Style::Normal);
                    if let Some(v) = &g.values {
                        for (j, x) in v.iter().enumerate() {
                            sc.number(row, 5 + j as u32, *x, Style::Normal);
                        }
                        let rg = format!("{}:{}", cell_ref(row, 5), cell_ref(row, 4 + v.len() as u32));
                        let s = format!("STDEV({rg})");
                        sc.formula(
                            row,
                            1,
                            if *relative { format!("{s}/ABS(AVERAGE({rg}))") } else { s },
                            Style::Number,
                        );
                        sc.formula(row, 2, format!("COUNT({rg})"), Style::Normal);
                    } else {
                        let s = g.sd.or(g.sd_percent.map(|x| x / 100.0)).unwrap_or(f64::NAN);
                        sc.number(row, 1, s, Style::Number);
                        sc.number(row, 2, g.n.unwrap_or(0) as f64, Style::Normal);
                    }
                    sc.formula(
                        row,
                        3,
                        format!("({n}-1)*{s}^2", n = cell_ref(row, 2), s = cell_ref(row, 1)),
                        Style::Number,
                    );
                    sc.formula(row, 4, format!("{}-1", cell_ref(row, 2)), Style::Normal);
                }
                let g1 = g0 + groups.len() as u32 - 1;
                r = g1 + 1;
                label(&mut sc, r, t(lang, "pooled s (MUPR Eq. 1)", "birleştirilmiş s (MUPR Denk. 1)"));
                sc.formula(
                    r,
                    1,
                    format!(
                        "SQRT(SUM({}:{})/SUM({}:{}))",
                        cell_ref(g0, 3),
                        cell_ref(g1, 3),
                        cell_ref(g0, 4),
                        cell_ref(g1, 4)
                    ),
                    Style::Number,
                );
                r += 1;
                Some(cell_ref(r - 1, 1))
            }
            PrecisionData::Duplicates { pairs, relative, method, min_mean, max_mean } => {
                let range = LevelRange { min: *min_mean, max: *max_mean };
                let heads = [
                    "x₁",
                    "x₂",
                    t(lang, "mean", "ortalama"),
                    t(lang, "in range", "aralıkta"),
                    t(lang, "term", "terim"),
                ];
                for (c, h) in heads.iter().enumerate() {
                    sc.text(r, 1 + c as u32, *h, Style::Heading);
                }
                let d0 = r + 1;
                for (k, pr) in pairs.iter().enumerate() {
                    let row = d0 + k as u32;
                    sc.number(row, 1, pr[0], Style::Normal);
                    sc.number(row, 2, pr[1], Style::Normal);
                    let (x1, x2, m) = (cell_ref(row, 1), cell_ref(row, 2), cell_ref(row, 3));
                    sc.formula(row, 3, format!("({x1}+{x2})/2"), Style::Number);
                    sc.formula(row, 4, in_range(&range, &m), Style::Normal);
                    let term = match (method, relative) {
                        (DuplicateMethod::Pooled, false) => format!("({x1}-{x2})^2"),
                        (DuplicateMethod::Pooled, true) => format!("(({x1}-{x2})^2/2)/{m}^2"),
                        (DuplicateMethod::NormalisedDifferences, _) => format!("({x1}-{x2})/{m}"),
                        (DuplicateMethod::MeanRange, false) => format!("ABS({x1}-{x2})"),
                        (DuplicateMethod::MeanRange, true) => format!("ABS({x1}-{x2})/{m}"),
                    };
                    sc.formula(row, 5, format!("IF({},{term},\"\")", cell_ref(row, 4)), Style::Number);
                    sc.formula(row, 6, format!("IF({},{m},\"\")", cell_ref(row, 4)), Style::Number);
                }
                let d1 = d0 + pairs.len() as u32 - 1;
                let terms = format!("{}:{}", cell_ref(d0, 5), cell_ref(d1, 5));
                r = d1 + 1;
                label(&mut sc, r, t(lang, "mean of the selected pairs", "seçilen çiftlerin ortalaması"));
                sc.formula(r, 1, format!("AVERAGE({}:{})", cell_ref(d0, 6), cell_ref(d1, 6)), Style::Number);
                mean_cell = Some(cell_ref(r, 1));
                let f = match (method, relative) {
                    (DuplicateMethod::Pooled, false) => format!("SQRT(SUM({terms})/(2*COUNT({terms})))"),
                    (DuplicateMethod::Pooled, true) => format!("SQRT(AVERAGE({terms}))"),
                    (DuplicateMethod::NormalisedDifferences, _) => format!("STDEV({terms})/SQRT(2)"),
                    (DuplicateMethod::MeanRange, _) => format!("AVERAGE({terms})/1.128"),
                };
                label(&mut sc, r + 1, "s");
                sc.formula(r + 1, 1, f, Style::Number);
                r += 2;
                Some(cell_ref(r - 1, 1))
            }
            PrecisionData::ControlLimits { limit, limit_percent, k } => {
                let l = limit.or(limit_percent.map(|x| x / 100.0)).unwrap_or(f64::NAN);
                label(&mut sc, r, "L, k");
                sc.number(r, 1, l, Style::Number);
                sc.number(r, 2, *k, Style::Normal);
                label(&mut sc, r + 1, "s = L / k (Nordtest §5.1)");
                sc.formula(r + 1, 1, format!("{}/{}", cell_ref(r, 1), cell_ref(r, 2)), Style::Number);
                r += 2;
                Some(cell_ref(r - 1, 1))
            }
            PrecisionData::Given { sd, sd_percent, .. } => {
                label(&mut sc, r, "s");
                sc.number(r, 1, sd.or(sd_percent.map(|x| x / 100.0)).unwrap_or(f64::NAN), Style::Number);
                r += 1;
                Some(cell_ref(r - 1, 1))
            }
            PrecisionData::Linear { points, intercept, slope, dilution_factor } => {
                let f = dilution_factor.unwrap_or(1.0);
                let (ia, ib) = match (points, intercept, slope) {
                    (Some(pts), _, _) => {
                        sc.text(r, 1, "c", Style::Heading);
                        sc.text(r, 2, "s", Style::Heading);
                        for (k, q) in pts.iter().enumerate() {
                            sc.number(r + 1 + k as u32, 1, q[0], Style::Normal);
                            sc.number(r + 1 + k as u32, 2, q[1], Style::Normal);
                        }
                        let (c0, c1) = (r + 1, r + pts.len() as u32);
                        r = c1 + 1;
                        let (xs, ys) = (
                            format!("{}:{}", cell_ref(c0, 1), cell_ref(c1, 1)),
                            format!("{}:{}", cell_ref(c0, 2), cell_ref(c1, 2)),
                        );
                        (format!("INTERCEPT({ys},{xs})"), format!("SLOPE({ys},{xs})"))
                    }
                    _ => (number(intercept.unwrap_or(f64::NAN)), number(slope.unwrap_or(f64::NAN))),
                };
                label(&mut sc, r, "a, b, F");
                sc.formula(r, 1, ia, Style::Number);
                sc.formula(r, 2, ib, Style::Number);
                sc.number(r, 3, f, Style::Normal);
                model = Some(format!("{F}*({a}+{b}*{{x}}/{F})", F = at(r, 3), a = at(r, 1), b = at(r, 2)));
                r += 1;
                None
            }
            PrecisionData::Continuous { sd0, rsd1_percent } => {
                label(&mut sc, r, "s₀, s′₁");
                sc.number(r, 1, *sd0, Style::Number);
                sc.formula(r, 2, format!("{}/100", number(*rsd1_percent)), Style::Number);
                model = Some(format!("SQRT({s0}^2+({{x}}*{s1})^2)", s0 = at(r, 1), s1 = at(r, 2)));
                r += 1;
                None
            }
            PrecisionData::StaggeredNested { .. } | PrecisionData::Reproducibility { .. } => {
                label(&mut sc, r, "s");
                sc.number(r, 1, pr.sd, Style::Number);
                sc.text(r, 2, fixed, Style::Note);
                r += 1;
                items.push(Item {
                    name: pr.name.clone(),
                    u: Some(at(r - 1, 1)),
                    relative: pr.relative,
                    multiplicity: 1,
                    range: pr.range,
                    model: None,
                    recovery: false,
                });
                r += 1;
                continue;
            }
        };
        if let Some(m) = model {
            items.push(Item {
                name: pr.name.clone(),
                u: None,
                relative: false,
                multiplicity: 1,
                range: pr.range,
                model: Some(m),
                recovery: false,
            });
            r += 1;
            continue;
        }
        let mut u = raw.expect("raw standard deviation");
        if let Some(mo) = &ps.mean_of {
            let s_r = mo.repeatability.or(mo.repeatability_percent.map(|x| x / 100.0)).unwrap_or(0.0);
            label(
                &mut sc,
                r,
                t(
                    lang,
                    "mean of n results on p days (MUPR Eq. 5): s_r, n, p",
                    "p günde n sonucun ortalaması (MUPR Denk. 5): s_r, n, p",
                ),
            );
            sc.number(r, 1, s_r, Style::Number);
            sc.number(r, 2, mo.replicates as f64, Style::Normal);
            sc.number(r, 3, mo.days as f64, Style::Normal);
            sc.formula(
                r + 1,
                1,
                format!(
                    "SQRT({u}^2/{p}+{sr}^2*(1-{n})/({n}*{p}))",
                    sr = cell_ref(r, 1),
                    n = cell_ref(r, 2),
                    p = cell_ref(r, 3)
                ),
                Style::Number,
            );
            label(&mut sc, r + 1, "s_I(x̿)");
            u = cell_ref(r + 1, 1);
            r += 2;
        }
        match (&ps.to_relative_at, ps.to_absolute_at) {
            (Some(At::Value(v)), _) => {
                label(&mut sc, r, t(lang, "relative at", "şu değere göre bağıl"));
                sc.formula(r, 1, format!("{u}/ABS({})", number(*v)), Style::Number);
                u = cell_ref(r, 1);
                r += 1;
            }
            (Some(At::Keyword(_)), _) => {
                label(&mut sc, r, t(lang, "relative to the mean", "ortalamaya göre bağıl"));
                sc.formula(
                    r,
                    1,
                    format!("{u}/ABS({})", mean_cell.clone().unwrap_or_else(|| "NA()".into())),
                    Style::Number,
                );
                u = cell_ref(r, 1);
                r += 1;
            }
            (None, Some(v)) => {
                label(&mut sc, r, t(lang, "absolute at", "şu değerde mutlak"));
                sc.formula(r, 1, format!("{u}*ABS({})", number(v)), Style::Number);
                u = cell_ref(r, 1);
                r += 1;
            }
            _ => {}
        }
        let (row, col) = parse_ref(&u);
        ch.add(format!("u ({})", pr.name), pr.sd, at(row, col), Diff::Relative, 1e-10);
        items.push(Item {
            name: pr.name.clone(),
            u: Some(at(row, col)),
            relative: pr.relative,
            multiplicity: 1,
            range: pr.range,
            model: None,
            recovery: false,
        });
        r += 1;
    }

    // --- Bias -------------------------------------------------------------------------
    for (bs, br) in spec.bias.iter().zip(&a.bias) {
        sc.text(r, 0, format!("{} — {}", t(lang, "Bias", "Sapma"), bs.name), Style::Heading);
        sc.text(
            r,
            1,
            if bs.relative { t(lang, "relative", "bağıl") } else { t(lang, "absolute", "mutlak") },
            Style::Heading,
        );
        r += 1;
        let shared = if !bs.u_reference_components_percent.is_empty() {
            label(&mut sc, r, t(lang, "shared u(C_ref) components, %", "ortak u(C_ref) bileşenleri, %"));
            for (k, v) in bs.u_reference_components_percent.iter().enumerate() {
                sc.number(r, 2 + k as u32, *v, Style::Normal);
            }
            sc.formula(
                r,
                1,
                format!(
                    "SQRT(SUMSQ({}:{}))/100",
                    cell_ref(r, 2),
                    cell_ref(r, 1 + bs.u_reference_components_percent.len() as u32)
                ),
                Style::Number,
            );
            r += 1;
            Some((cell_ref(r - 1, 1), true))
        } else {
            match (bs.u_reference, bs.u_reference_percent) {
                (Some(v), _) => Some((number(v), false)),
                (None, Some(v)) => Some((format!("{}/100", number(v)), true)),
                _ => None,
            }
        };
        let heads = [
            t(lang, "reference", "referans"),
            "C",
            t(lang, "mean", "ortalama"),
            "s",
            "n",
            t(lang, "bias", "sapma"),
            "u(C_ref)",
            t(lang, "results", "sonuçlar"),
        ];
        for (c, h) in heads.iter().enumerate() {
            sc.text(r, c as u32, *h, Style::Heading);
        }
        let e0 = r + 1;
        for (k, rf) in bs.references.iter().enumerate() {
            let row = e0 + k as u32;
            let cc = |c: u32| cell_ref(row, c);
            sc.text(row, 0, rf.label.clone().unwrap_or_else(|| format!("{}", k + 1)), Style::Normal);
            if let Some(c) = rf.reference {
                sc.number(row, 1, c, Style::Number);
            }
            match (&rf.measured, rf.mean) {
                (Some(v), _) => {
                    for (j, x) in v.iter().enumerate() {
                        sc.number(row, 7 + j as u32, *x, Style::Normal);
                    }
                    let rg = format!("{}:{}", cc(7), cell_ref(row, 6 + v.len() as u32));
                    sc.formula(row, 2, format!("AVERAGE({rg})"), Style::Number);
                    sc.formula(row, 3, format!("STDEV({rg})"), Style::Number);
                    sc.formula(row, 4, format!("COUNT({rg})"), Style::Normal);
                }
                (None, m) => {
                    if let Some(m) = m {
                        sc.number(row, 2, m, Style::Number);
                    }
                    if let Some(s) = rf.sd {
                        sc.number(row, 3, s, Style::Number);
                    } else if let Some(s) = rf.sd_percent {
                        sc.formula(row, 3, format!("{}/100*{}", number(s), cc(2)), Style::Number);
                    }
                    if let Some(n) = rf.n {
                        sc.number(row, 4, n as f64, Style::Normal);
                    }
                }
            }
            // Converts a value of the given kind to the kind of the evaluation.
            let conv = |v: String, rel: bool| -> String {
                if rel == bs.relative {
                    v
                } else if bs.relative {
                    format!("({v})/{}", cc(1))
                } else {
                    format!("({v})*{}", cc(1))
                }
            };
            let bias = match (rf.bias, rf.bias_percent, rf.recovery_percent) {
                (Some(v), _, _) => conv(number(v), false),
                (None, Some(v), _) => conv(format!("{}/100", number(v)), true),
                (None, None, Some(rp)) => conv(format!("{}/100-1", number(rp)), true),
                _ => {
                    if bs.relative {
                        format!("({m}-{c})/{c}", m = cc(2), c = cc(1))
                    } else {
                        format!("{}-{}", cc(2), cc(1))
                    }
                }
            };
            sc.formula(row, 5, bias, Style::Number);
            let own = if let Some(u) = rf.u_reference {
                Some(conv(number(u), false))
            } else if let Some(u) = rf.u_reference_percent {
                Some(conv(format!("{}/100", number(u)), true))
            } else if let (Some(u), Some(k)) = (rf.expanded, rf.k) {
                Some(conv(format!("{}/{}", number(u), number(k)), false))
            } else if let (Some(s), Some(l)) = (rf.sd_reproducibility, rf.labs) {
                Some(conv(format!("{}/SQRT({l})", number(s)), false))
            } else if let (Some(s), Some(l)) = (rf.sd_reproducibility_percent, rf.labs) {
                Some(conv(format!("{}/SQRT({l})/100", number(s)), true))
            } else {
                shared.as_ref().map(|(f, rel)| conv(f.clone(), *rel))
            };
            sc.formula(row, 6, own.unwrap_or_else(|| "NA()".into()), Style::Number);
        }
        let e1 = e0 + bs.references.len() as u32 - 1;
        let (bcol, ucol) =
            (format!("{}:{}", cell_ref(e0, 5), cell_ref(e1, 5)), format!("{}:{}", cell_ref(e0, 6), cell_ref(e1, 6)));
        r = e1 + 1;
        label(&mut sc, r, "RMS_bias");
        sc.formula(r, 1, format!("SQRT(SUMSQ({bcol})/COUNT({bcol}))"), Style::Number);
        label(&mut sc, r + 1, "ū(C_ref)");
        sc.formula(r + 1, 1, format!("AVERAGE({ucol})"), Style::Number);
        let single = bs.kind == BiasKind::Crm && br.single_reference;
        let uf = if single {
            let s = if bs.relative { format!("{}/ABS({})", cell_ref(e0, 3), cell_ref(e0, 2)) } else { cell_ref(e0, 3) };
            label(&mut sc, r + 2, "u(bias) = √(b² + s²/n + u²(C_ref))");
            format!("SQRT({b}^2+({s})^2/{n}+{u}^2)", b = cell_ref(e0, 5), n = cell_ref(e0, 4), u = cell_ref(e0, 6))
        } else {
            label(&mut sc, r + 2, "u(bias) = √(RMS_bias² + ū(C_ref)²)");
            format!("SQRT({}^2+{}^2)", cell_ref(r, 1), cell_ref(r + 1, 1))
        };
        sc.formula(r + 2, 1, uf, Style::BoldNumber);
        ch.add(format!("RMS_bias ({})", br.name), br.rms_bias, at(r, 1), Diff::Relative, 1e-10);
        ch.add(format!("ū(C_ref) ({})", br.name), br.mean_u_reference, at(r + 1, 1), Diff::Relative, 1e-10);
        ch.add(format!("u ({})", br.name), br.u, at(r + 2, 1), Diff::Relative, 1e-10);
        items.push(Item {
            name: br.name.clone(),
            u: Some(at(r + 2, 1)),
            relative: br.relative,
            multiplicity: 1,
            range: br.range,
            model: None,
            recovery: false,
        });
        r += 4;
    }

    // --- Recovery (KAPSAM's value) ------------------------------------------------------
    let mut correction: Option<String> = None;
    if let Some(rec) = &a.recovery {
        sc.text(r, 0, format!("{} — {}", t(lang, "Recovery", "Geri kazanım"), rec.name), Style::Heading);
        r += 1;
        label(&mut sc, r, "R̄");
        sc.number(r, 1, rec.mean, Style::Number);
        sc.text(r, 2, fixed, Style::Note);
        label(&mut sc, r + 1, t(lang, "relative u entering the budget", "bütçeye giren bağıl u"));
        sc.number(r + 1, 1, rec.u_relative, Style::Number);
        sc.text(r + 1, 2, fixed, Style::Note);
        if rec.corrected {
            correction = Some(at(r, 1));
        }
        items.push(Item {
            name: rec.name.clone(),
            u: Some(at(r + 1, 1)),
            relative: true,
            multiplicity: 1,
            range: rec.range,
            model: None,
            recovery: true,
        });
        r += 3;
    }

    // --- Further components -------------------------------------------------------------
    for (cs, bi) in spec.components.iter().zip(a.budget.iter().filter(|b| b.source == ComponentSource::Additional)) {
        let div = match cs.distribution {
            LimitDistribution::Rectangular => "SQRT(3)",
            LimitDistribution::Triangular => "SQRT(6)",
        };
        let f = if let Some(u) = cs.u {
            number(u)
        } else if let Some(u) = cs.u_percent {
            format!("{}/100", number(u))
        } else if let Some(h) = cs.half_width {
            format!("{}/{div}", number(h))
        } else {
            format!("{}/100/{div}", number(cs.half_width_percent.unwrap_or(f64::NAN)))
        };
        label(&mut sc, r, &cs.name);
        sc.formula(r, 1, f, Style::Number);
        sc.text(
            r,
            2,
            if bi.relative { t(lang, "relative", "bağıl") } else { t(lang, "absolute", "mutlak") },
            Style::Note,
        );
        if cs.multiplicity > 1 {
            sc.text(r, 3, format!("× {}", cs.multiplicity), Style::Note);
        }
        ch.add(format!("u ({})", cs.name), bi.u, at(r, 1), Diff::Relative, 1e-12);
        items.push(Item {
            name: cs.name.clone(),
            u: Some(at(r, 1)),
            relative: bi.relative,
            multiplicity: cs.multiplicity,
            range: cs.range.unwrap_or_default(),
            model: None,
            recovery: false,
        });
        r += 1;
    }
    sc.width(0, 40.0);
    for c in 1..12 {
        sc.width(c, 15.0);
    }

    // --- Levels -----------------------------------------------------------------------
    let mut sl = Sheet::new(s_lev);
    sl.text(
        0,
        0,
        t(
            lang,
            "Combination at each level: u_c = √(Σ u_abs² + c² Σ u′_rel²) (Nordtest §3, MUPR Eq. 19–20)",
            "Her düzeyde birleştirme: u_c = √(Σ u_mutlak² + c² Σ u′_bağıl²) (Nordtest §3, MUPR Denk. 19–20)",
        ),
        Style::Title,
    );
    let kf = match (spec.coverage_factor, spec.coverage_probability) {
        (Some(k), _) => number(k),
        (None, Some(pr)) => match spec.coverage_dof {
            Some(nu) if nu.is_finite() => {
                format!("SQRT({nu}*(1/BETAINV(1-{p},{nu}/2,0.5)-1))", nu = number(nu), p = number(pr))
            }
            _ => format!("NORMSINV((1+{})/2)", number(pr)),
        },
        (None, None) => "2".into(),
    };
    sl.text(2, 0, "k", Style::Bold);
    sl.formula(2, 1, kf, Style::Number);
    ch.add("k", a.coverage_factor, sheet_ref(s_lev, 2, 1), Diff::Relative, 1e-9);
    sl.text(3, 0, t(lang, "recovery correction R̄ (1 if none)", "geri kazanım düzeltmesi R̄ (yoksa 1)"), Style::Bold);
    sl.formula(3, 1, correction.unwrap_or_else(|| "1".into()), Style::Number);
    let (k_ref, r_ref) = (cell_ref(2, 1), cell_ref(3, 1));
    let hr = 5u32;
    let (c_lv, c_val) = (0u32, 1u32);
    let c_item0 = 2u32;
    let ni = items.len() as u32;
    let (c_uc, c_rel, c_uu, c_relu) = (c_item0 + ni, c_item0 + ni + 1, c_item0 + ni + 2, c_item0 + ni + 3);
    sl.text(hr, c_lv, t(lang, "Level", "Düzey"), Style::Heading);
    sl.text(hr, c_val, t(lang, "value", "değer"), Style::Heading);
    for (k, it) in items.iter().enumerate() {
        sl.text(hr, c_item0 + k as u32, format!("{} (u²)", it.name), Style::Heading);
    }
    sl.text(hr, c_uc, "u_c", Style::Heading);
    sl.text(hr, c_rel, "u_c / |value|", Style::Heading);
    sl.text(hr, c_uu, "U", Style::Heading);
    sl.text(hr, c_relu, "U / |value|", Style::Heading);
    let levels: Vec<Option<f64>> =
        if spec.levels.is_empty() { vec![None] } else { spec.levels.iter().copied().map(Some).collect() };
    for (li, (lv, res)) in levels.iter().zip(&a.levels).enumerate() {
        let row = hr + 1 + li as u32;
        let x = cell_ref(row, c_lv);
        match lv {
            Some(v) => sl.number(row, c_lv, *v, Style::Number),
            None => sl.text(row, c_lv, t(lang, "relative only", "yalnızca bağıl"), Style::Normal),
        }
        sl.formula(row, c_val, if lv.is_some() { format!("{x}/{r_ref}") } else { "1".into() }, Style::Number);
        let val = cell_ref(row, c_val);
        for (k, it) in items.iter().enumerate() {
            let m = it.multiplicity;
            let var = if let Some(model) = &it.model {
                let sd = model.replace("{x}", &x);
                format!("{m}*(({sd})/{r_ref})^2")
            } else if it.relative {
                format!("{m}*({val}*{})^2", it.u.as_deref().unwrap_or("0"))
            } else if it.recovery {
                format!("{m}*{}^2", it.u.as_deref().unwrap_or("0"))
            } else {
                format!("{m}*({}/{r_ref})^2", it.u.as_deref().unwrap_or("0"))
            };
            let cond = if lv.is_some() { in_range(&it.range, &x) } else { "TRUE".into() };
            sl.formula(row, c_item0 + k as u32, format!("IF({cond},{var},0)"), Style::Number);
        }
        let terms = format!("{}:{}", cell_ref(row, c_item0), cell_ref(row, c_item0 + ni - 1));
        sl.formula(row, c_uc, format!("SQRT(SUM({terms}))"), Style::BoldNumber);
        sl.formula(row, c_rel, format!("{}/ABS({val})", cell_ref(row, c_uc)), Style::Number);
        sl.formula(row, c_uu, format!("{k_ref}*{}", cell_ref(row, c_uc)), Style::BoldNumber);
        sl.formula(row, c_relu, format!("{k_ref}*{}", cell_ref(row, c_rel)), Style::Number);
        let tag = lv.map_or_else(|| t(lang, "relative", "bağıl").to_string(), |v| format!("{v}"));
        if let (Some(u), Some(big_u)) = (res.standard_uncertainty, res.expanded_uncertainty) {
            ch.add(format!("u_c ({tag})"), u, sheet_ref(s_lev, row, c_uc), Diff::Relative, 1e-10);
            ch.add(format!("U ({tag})"), big_u, sheet_ref(s_lev, row, c_uu), Diff::Relative, 1e-9);
        } else {
            ch.add(
                format!("u_c / |value| ({tag})"),
                res.relative_standard,
                sheet_ref(s_lev, row, c_rel),
                Diff::Relative,
                1e-10,
            );
            ch.add(
                format!("U / |value| ({tag})"),
                res.relative_expanded,
                sheet_ref(s_lev, row, c_relu),
                Diff::Relative,
                1e-9,
            );
        }
    }
    sl.width(0, 34.0);
    for c in 1..=c_relu {
        sl.width(c, 16.0);
    }

    let mut s_check = Sheet::new(t(lang, "Check", "Karşılaştırma"));
    let fails = ch.write(&mut s_check, lang);
    let info = info_sheet(p, lang, &fails, &[
        t(lang,
          "Components: every precision component from its data (control-sample series, groups pooled by MUPR Eq. 1, duplicates by Nordtest Appendices 5–6 or the mean range, control limits, stated values, level-dependent models), the mean of n results on p days (MUPR Eq. 5) and the conversions between absolute and relative values; every bias component from its references, u(bias) = √(RMS_bias² + ū(C_ref)²) or, for one CRM measured n times, √(b² + s²/n + u²(C_ref)) (Nordtest §6); the further components (u, or a half-width divided by √3 or √6).",
          "Bileşenler: her kesinlik bileşeni verisinden (kontrol numunesi serisi, MUPR Denk. 1 ile birleştirilen gruplar, Nordtest Ek 5–6'ya göre ya da ortalama açıklıkla çiftler, kontrol sınırları, verilmiş değerler, düzeye bağlı modeller), p günde n sonucun ortalaması (MUPR Denk. 5) ve mutlak-bağıl dönüşümler; her sapma bileşeni referanslarından, u(sapma) = √(RMS_sapma² + ū(C_ref)²) ya da n kez ölçülen tek bir SRM için √(b² + s²/n + u²(C_ref)) (Nordtest §6); ek bileşenler (u ya da √3 veya √6'ya bölünen yarı genişlik)."),
        t(lang,
          "Levels: the components that apply at each level are combined, absolute ones as they are and relative ones multiplied by the value; U = k u_c.",
          "Düzeyler: her düzeyde geçerli bileşenler birleştirilir; mutlak bileşenler olduğu gibi, bağıl bileşenler değerle çarpılarak; U = k u_c."),
        t(lang,
          "Entered with KAPSAM's values and not recomputed: the recovery (significance test and correction, MUPR §8.2), the staggered-nested design (ISO 5725-3) and the ISO 21748 reproducibility component.",
          "KAPSAM değerleriyle girilen ve yeniden hesaplanmayanlar: geri kazanım (anlamlılık testi ve düzeltme, MUPR §8.2), kademeli iç içe tasarım (ISO 5725-3) ve ISO 21748 tekrar üretilebilirlik bileşeni."),
    ]);
    Ok(Workbook { sheets: vec![info, s_check, sc, sl] })
}

/// Row and column of an "A1" reference.
fn parse_ref(s: &str) -> (u32, u32) {
    let letters: String = s.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    let digits: String = s.chars().skip_while(|c| c.is_ascii_alphabetic()).collect();
    let col = letters.bytes().fold(0u32, |acc, b| acc * 26 + (b - b'A' + 1) as u32) - 1;
    (digits.parse::<u32>().unwrap_or(1) - 1, col)
}
