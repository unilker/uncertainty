//! Verification workbook of a proficiency test round [ISO 13528:2022]:
//! robust statistics (C.2), Algorithm A with the stopping rule of C.3.1, the
//! assigned value and its uncertainty (Clause 7), σ_pt (Clause 8), the
//! performance statistics (Clause 9), and the homogeneity and stability
//! checks (B.2, B.3, B.5). The Bayesian between-unit model is not included.

use super::{info_sheet, t, Checks, Diff};
use crate::proficiency::{AssignedMethodSpec, ProficiencyAnalysis, ScaleEstimator, SdPtMethodSpec, StopSpec};
use crate::schema::Project;
use crate::xlsx::{abs_ref, cell_ref, number, sheet_ref, Sheet, Style, Workbook};
use crate::{Lang, ProjectError};

/// Columns of the sheet Results.
const C_LAB: u32 = 0;
const C_X: u32 = 1;
const C_CENS: u32 = 2;
const C_LIMIT: u32 = 3;
const C_U_REP: u32 = 4;
const C_UU_REP: u32 = 5;
const C_K_REP: u32 = 6;
const C_U: u32 = 7;
const C_UU: u32 = 8;
const C_ABS: u32 = 9;
const C_D: u32 = 10;
const C_DPC: u32 = 11;
const C_PA: u32 = 12;
const C_Z: u32 = 13;
const C_ZP: u32 = 14;
const C_ZETA: u32 = 15;
const C_EN: u32 = 16;

/// A value rounded to three significant figures, as a formula (C.3.1).
fn round3(c: &str) -> String {
    format!("IF({c}=0,0,ROUND({c},2-INT(LOG10(ABS({c})))))")
}

pub(super) fn workbook(p: &Project, a: &ProficiencyAnalysis, lang: Lang) -> Result<Workbook, ProjectError> {
    let spec = p.proficiency.as_ref().ok_or_else(|| ProjectError::Format("no proficiency section".into()))?;
    let s_res = t(lang, "Results", "Sonuçlar");
    let s_stat = t(lang, "Statistics", "İstatistikler");
    let s_alg = t(lang, "Algorithm A", "Algoritma A");
    let s_hom = t(lang, "Homogeneity", "Homojenlik");
    let s_check = t(lang, "Check", "Karşılaştırma");
    let mut sheets = Vec::new();
    let mut ch = Checks::default();
    let n = spec.results.len();
    let r0 = 3u32;
    let last = r0 + n.max(1) as u32 - 1;
    let range = |col: u32| format!("'{s_res}'!{}:{}", abs_ref(r0, col), abs_ref(last, col));
    let stat = |row: u32| sheet_ref(s_stat, row, 1);

    // Rows of the sheet Statistics.
    const R_P: u32 = 3;
    const R_MED: u32 = 4;
    const R_MADE: u32 = 5;
    const R_NIQR: u32 = 6;
    const R_MEAN: u32 = 7;
    const R_SD: u32 = 8;
    const R_XS: u32 = 9;
    const R_SS: u32 = 10;
    const R_IT: u32 = 11;
    const R_XPT: u32 = 13;
    const R_UPT: u32 = 14;
    const R_UUPT: u32 = 15;
    const R_SIG: u32 = 17;
    const R_DE: u32 = 18;
    const R_EXTRA: u32 = 20;

    if n > 0 {
        // --- Results ----------------------------------------------------------
        let mut sr = Sheet::new(s_res);
        sr.text(
            0,
            0,
            t(
                lang,
                "Participant results (data of the project file) and performance statistics",
                "Katılımcı sonuçları (proje dosyasının verileri) ve performans istatistikleri",
            ),
            Style::Title,
        );
        let heads = [
            (C_LAB, t(lang, "Laboratory", "Laboratuvar")),
            (C_X, "x"),
            (C_CENS, t(lang, "Censored", "Sansürlü")),
            (C_LIMIT, t(lang, "Limit", "Sınır")),
            (C_U_REP, t(lang, "u reported", "Bildirilen u")),
            (C_UU_REP, t(lang, "U reported", "Bildirilen U")),
            (C_K_REP, "k"),
            (C_U, "u(x)"),
            (C_UU, "U(x)"),
            (C_ABS, "|x − med|"),
            (C_D, "D (11)"),
            (C_DPC, "D % (12)"),
            (C_PA, "PA (13)"),
            (C_Z, "z (14)"),
            (C_ZP, "z′ (15)"),
            (C_ZETA, "ζ (19)"),
            (C_EN, "Eₙ (20)"),
        ];
        for (c, h) in heads {
            sr.text(2, c, h, Style::Heading);
        }
        let assigned = a.assigned.is_some();
        for (i, r) in spec.results.iter().enumerate() {
            let row = r0 + i as u32;
            let at = |c: u32| cell_ref(row, c);
            sr.text(row, C_LAB, r.lab.clone(), Style::Bold);
            if let Some(v) = r.value {
                sr.number(row, C_X, v, Style::Number);
            }
            if let Some(l) = r.less_than {
                sr.text(row, C_CENS, "<", Style::Normal);
                sr.number(row, C_LIMIT, l, Style::Number);
            }
            if let Some(g) = r.greater_than {
                sr.text(row, C_CENS, ">", Style::Normal);
                sr.number(row, C_LIMIT, g, Style::Number);
            }
            if let Some(u) = r.u {
                sr.number(row, C_U_REP, u, Style::Number);
            }
            if let Some(e) = r.expanded {
                sr.number(row, C_UU_REP, e, Style::Number);
            }
            if let Some(k) = r.k {
                sr.number(row, C_K_REP, k, Style::Number);
            }
            sr.formula(
                row,
                C_U,
                format!(
                    "IF(ISNUMBER({u}),{u},IF(ISNUMBER({e}),{e}/IF(ISNUMBER({k}),{k},2),\"\"))",
                    u = at(C_U_REP),
                    e = at(C_UU_REP),
                    k = at(C_K_REP)
                ),
                Style::Number,
            );
            sr.formula(
                row,
                C_UU,
                format!("IF(ISNUMBER({u}),2*{u},IF(ISNUMBER({e}),{e},\"\"))", u = at(C_U_REP), e = at(C_UU_REP)),
                Style::Number,
            );
            sr.formula(
                row,
                C_ABS,
                format!("IF(ISNUMBER({x}),ABS({x}-{}),\"\")", stat(R_MED), x = at(C_X)),
                Style::Number,
            );
            if r.value.is_some() && assigned {
                let (x, d) = (at(C_X), at(C_D));
                sr.formula(row, C_D, format!("{x}-{}", stat(R_XPT)), Style::Number);
                sr.formula(row, C_DPC, format!("IF({xpt}=0,\"\",100*{d}/{xpt})", xpt = stat(R_XPT)), Style::Number);
                if a.delta_e.is_some() {
                    sr.formula(row, C_PA, format!("100*{d}/{}", stat(R_DE)), Style::Number);
                }
                if a.sigma_pt.is_some() {
                    sr.formula(row, C_Z, format!("{d}/{}", stat(R_SIG)), Style::Number);
                    sr.formula(
                        row,
                        C_ZP,
                        format!("{d}/SQRT({s}^2+{u}^2)", s = stat(R_SIG), u = stat(R_UPT)),
                        Style::Number,
                    );
                }
                sr.formula(
                    row,
                    C_ZETA,
                    format!("IF(ISNUMBER({u}),{d}/SQRT({u}^2+{upt}^2),\"\")", u = at(C_U), upt = stat(R_UPT)),
                    Style::Number,
                );
                sr.formula(
                    row,
                    C_EN,
                    format!("IF(ISNUMBER({e}),{d}/SQRT({e}^2+{ue}^2),\"\")", e = at(C_UU), ue = stat(R_UUPT)),
                    Style::Number,
                );
            }
        }
        sr.width(0, 16.0);
        for c in 1..=C_EN {
            sr.width(c, 15.0);
        }

        // --- Statistics -------------------------------------------------------
        let mut ss = Sheet::new(s_stat);
        ss.text(
            0,
            0,
            t(
                lang,
                "Robust statistics, assigned value and σ_pt (ISO 13528 C.2, Clauses 7 and 8)",
                "Dayanıklı istatistikler, atanan değer ve σ_pt (ISO 13528 C.2, Madde 7 ve 8)",
            ),
            Style::Title,
        );
        let x = range(C_X);
        let rows: [(u32, String, String); 6] = [
            (R_P, "p".into(), format!("COUNT({x})")),
            (R_MED, t(lang, "median (C.1)", "medyan (C.1)").into(), format!("MEDIAN({x})")),
            (R_MADE, "MADe = 1.483 med|xᵢ − med| (C.2, C.3)".into(), format!("1.483*MEDIAN({})", range(C_ABS))),
            (R_NIQR, "nIQR = 0.7413 (Q₃ − Q₁) (C.4)".into(), format!("0.7413*(QUARTILE({x},3)-QUARTILE({x},1))")),
            (R_MEAN, t(lang, "mean", "ortalama").into(), format!("AVERAGE({x})")),
            (R_SD, t(lang, "standard deviation", "standart sapma").into(), format!("STDEV({x})")),
        ];
        for (r, label, f) in rows {
            ss.text(r, 0, label, Style::Bold);
            ss.formula(r, 1, f, Style::Number);
        }
        if let Some(sm) = &a.summary {
            ch.add(t(lang, "median", "medyan"), sm.median, stat(R_MED), Diff::Relative, 1e-12);
            ch.add("MADe", sm.made, stat(R_MADE), Diff::Relative, 1e-12);
            ch.add("nIQR", sm.niqr, stat(R_NIQR), Diff::Relative, 1e-12);
            ch.add(t(lang, "mean", "ortalama"), sm.mean, stat(R_MEAN), Diff::Relative, 1e-12);
            ch.add(t(lang, "standard deviation", "standart sapma"), sm.sd, stat(R_SD), Diff::Relative, 1e-12);
        }

        // --- Algorithm A ------------------------------------------------------
        if let Some(rob) = a.summary.as_ref().and_then(|s| s.robust.as_ref()) {
            let mut sa = Sheet::new(s_alg);
            sa.text(
                0,
                0,
                t(
                    lang,
                    "Algorithm A with iterated scale (ISO 13528 C.3.1), one column per iteration",
                    "Yinelemeli ölçekli Algoritma A (ISO 13528 C.3.1), her yineleme bir sütunda",
                ),
                Style::Title,
            );
            let iters = (rob.iterations + 10).max(25) as u32;
            let (r_it, r_delta, p0) = (2u32, 3u32, 4u32);
            let r_x = p0 + n as u32;
            let (r_s, r_flag) = (r_x + 1, r_x + 2);
            sa.text(r_it, 0, t(lang, "Iteration", "Yineleme"), Style::Heading);
            sa.text(r_delta, 0, "δ = 1.5 s*", Style::Bold);
            for (i, r) in spec.results.iter().enumerate() {
                sa.text(p0 + i as u32, 0, r.lab.clone(), Style::Normal);
            }
            sa.text(r_x, 0, "x*", Style::Bold);
            sa.text(r_s, 0, "s* = 1.134 s", Style::Bold);
            let stop = match spec.algorithm_a {
                StopSpec::ThirdSignificantFigure => {
                    t(lang, "no change in the 3rd significant figure", "3. anlamlı basamakta değişiklik yok")
                }
                StopSpec::Converged => t(lang, "converged (10⁻¹²)", "yakınsadı (10⁻¹²)"),
            };
            sa.text(r_flag, 0, stop, Style::Bold);
            // Column 1: starting values x* = median, s* = MADe (or s if MADe = 0, C.3.1 Note 2).
            sa.number(r_it, 1, 0.0, Style::Heading);
            for i in 0..n {
                sa.formula(
                    p0 + i as u32,
                    1,
                    format!("IF(ISNUMBER({v}),{v},\"\")", v = sheet_ref(s_res, r0 + i as u32, C_X)),
                    Style::Number,
                );
            }
            sa.formula(r_x, 1, stat(R_MED), Style::BoldNumber);
            sa.formula(r_s, 1, format!("IF({m}=0,{s},{m})", m = stat(R_MADE), s = stat(R_SD)), Style::BoldNumber);
            for k in 1..=iters {
                let c = 1 + k;
                let (xp, sp) = (cell_ref(r_x, c - 1), cell_ref(r_s, c - 1));
                sa.number(r_it, c, k as f64, Style::Heading);
                sa.formula(r_delta, c, format!("1.5*{sp}"), Style::Number);
                let d = cell_ref(r_delta, c);
                for i in 0..n {
                    let v = sheet_ref(s_res, r0 + i as u32, C_X);
                    sa.formula(
                        p0 + i as u32,
                        c,
                        format!("IF(ISNUMBER({v}),MAX(MIN({v},{xp}+{d}),{xp}-{d}),\"\")"),
                        Style::Number,
                    );
                }
                let col = format!("{}:{}", cell_ref(p0, c), cell_ref(p0 + n as u32 - 1, c));
                sa.formula(r_x, c, format!("AVERAGE({col})"), Style::BoldNumber);
                sa.formula(r_s, c, format!("1.134*STDEV({col})"), Style::BoldNumber);
                let (xn, sn) = (cell_ref(r_x, c), cell_ref(r_s, c));
                let flag = match spec.algorithm_a {
                    StopSpec::ThirdSignificantFigure => {
                        format!("AND({}={},{}={})", round3(&xn), round3(&xp), round3(&sn), round3(&sp))
                    }
                    StopSpec::Converged => {
                        format!("AND(ABS({xn}-{xp})<=1E-12*MAX(ABS({xn}),{sn}),ABS({sn}-{sp})<=1E-12*{sn})")
                    }
                };
                sa.formula(r_flag, c, flag, Style::Normal);
            }
            let (c1, cn) = (2u32, 1 + iters);
            let flags = format!("{}:{}", abs_ref(r_flag, c1), abs_ref(r_flag, cn));
            let pick = |row: u32| {
                format!(
                    "IFERROR(INDEX({}:{},MATCH(TRUE,{flags},0)),{})",
                    abs_ref(row, c1),
                    abs_ref(row, cn),
                    abs_ref(row, cn)
                )
            };
            let rr = r_flag + 2;
            sa.text(
                rr,
                0,
                t(
                    lang,
                    "Result: first iteration that meets the stopping rule",
                    "Sonuç: durdurma kuralını sağlayan ilk yineleme",
                ),
                Style::Bold,
            );
            sa.text(rr + 1, 0, "x*", Style::Bold);
            sa.formula(rr + 1, 1, pick(r_x), Style::BoldNumber);
            sa.text(rr + 2, 0, "s*", Style::Bold);
            sa.formula(rr + 2, 1, pick(r_s), Style::BoldNumber);
            sa.text(rr + 3, 0, t(lang, "iterations", "yineleme sayısı"), Style::Bold);
            sa.formula(rr + 3, 1, format!("IFERROR(MATCH(TRUE,{flags},0),\"> {iters}\")"), Style::Normal);
            sa.width(0, 34.0);
            for c in 1..=cn {
                sa.width(c, 15.0);
            }
            ss.text(R_XS, 0, t(lang, "x* (Algorithm A)", "x* (Algoritma A)"), Style::Bold);
            ss.formula(R_XS, 1, sheet_ref(s_alg, rr + 1, 1), Style::Number);
            ss.text(R_SS, 0, t(lang, "s* (Algorithm A)", "s* (Algoritma A)"), Style::Bold);
            ss.formula(R_SS, 1, sheet_ref(s_alg, rr + 2, 1), Style::Number);
            ss.text(R_IT, 0, t(lang, "iterations", "yineleme sayısı"), Style::Bold);
            ss.formula(R_IT, 1, sheet_ref(s_alg, rr + 3, 1), Style::Normal);
            ch.add("x*", rob.mean, stat(R_XS), Diff::Relative, 1e-10);
            ch.add("s*", rob.sd, stat(R_SS), Diff::Relative, 1e-10);
            ch.add(
                t(lang, "iterations of Algorithm A", "Algoritma A yineleme sayısı"),
                rob.iterations as f64,
                stat(R_IT),
                Diff::Absolute,
                0.0,
            );
            sheets.push(sa);
        }

        // --- Assigned value, its uncertainty, σ_pt and δ_E ---------------------
        if let (Some(av), Some(asg)) = (&spec.assigned_value, &a.assigned) {
            ss.text(R_EXTRA, 0, t(lang, "Added components (Formula 3)", "Eklenen bileşenler (Formül 3)"), Style::Bold);
            let extras = [("u_hom", av.u_hom), ("u_trans", av.u_trans), ("u_stab", av.u_stab)];
            for (k, (name, v)) in extras.iter().enumerate() {
                ss.text(R_EXTRA + 1 + k as u32, 0, *name, Style::Normal);
                ss.number(R_EXTRA + 1 + k as u32, 1, v.unwrap_or(0.0), Style::Number);
            }
            let ex2 = format!("{}^2+{}^2+{}^2", stat(R_EXTRA + 1), stat(R_EXTRA + 2), stat(R_EXTRA + 3));
            let factor = number(av.u_factor);
            let (xf, uf, method) = match av.method {
                AssignedMethodSpec::AlgorithmA => {
                    (stat(R_XS), format!("{factor}*{}/SQRT({})", stat(R_SS), stat(R_P)), "x* (7.7)")
                }
                AssignedMethodSpec::Median => {
                    let s = match av.scale {
                        ScaleEstimator::Niqr => stat(R_NIQR),
                        ScaleEstimator::Made => stat(R_MADE),
                    };
                    (stat(R_MED), format!("{factor}*{s}/SQRT({})", stat(R_P)), t(lang, "median", "medyan"))
                }
                AssignedMethodSpec::Mean => {
                    (stat(R_MEAN), format!("{}/SQRT({})", stat(R_SD), stat(R_P)), t(lang, "mean", "ortalama"))
                }
                AssignedMethodSpec::Reference => {
                    let v = av.value.unwrap_or(f64::NAN);
                    let u = match (av.u, av.expanded) {
                        (Some(u), _) => number(u),
                        (None, Some(e)) => format!("{}/{}", number(e), number(av.k.unwrap_or(2.0))),
                        _ => "NA()".into(),
                    };
                    (number(v), u, t(lang, "reference value (7.3–7.6)", "referans değer (7.3–7.6)"))
                }
            };
            ss.text(R_XPT - 1, 0, t(lang, "Assigned value", "Atanan değer"), Style::Heading);
            ss.text(R_XPT - 1, 1, method, Style::Heading);
            ss.text(R_XPT, 0, "x_pt", Style::Bold);
            ss.formula(R_XPT, 1, xf, Style::BoldNumber);
            ss.text(R_UPT, 0, "u(x_pt) (6)", Style::Bold);
            ss.formula(R_UPT, 1, format!("SQRT(({uf})^2+{ex2})"), Style::BoldNumber);
            ss.text(R_UUPT, 0, "U(x_pt)", Style::Bold);
            let stated = av.method == AssignedMethodSpec::Reference
                && av.expanded.is_some()
                && av.u_hom.is_none()
                && av.u_trans.is_none()
                && av.u_stab.is_none();
            let uu = if stated {
                number(av.expanded.unwrap_or(f64::NAN))
            } else {
                format!("{}*{}", number(a.coverage_factor), stat(R_UPT))
            };
            ss.formula(R_UUPT, 1, uu, Style::BoldNumber);
            ch.add("x_pt", asg.value, stat(R_XPT), Diff::Relative, 1e-10);
            ch.add("u(x_pt)", asg.u, stat(R_UPT), Diff::Relative, 1e-10);
            ch.add("U(x_pt)", asg.expanded, stat(R_UUPT), Diff::Relative, 1e-10);
        }
        if let (Some(sp), Some(sig)) = (&spec.sd_pt, a.sigma_pt) {
            let f = match sp.method {
                SdPtMethodSpec::Robust => stat(R_SS),
                SdPtMethodSpec::Made => stat(R_MADE),
                SdPtMethodSpec::Niqr => stat(R_NIQR),
                SdPtMethodSpec::Sd => stat(R_SD),
                SdPtMethodSpec::Given => match (sp.value, sp.percent) {
                    (Some(v), _) => number(v),
                    (None, Some(pc)) => format!("{}/100*ABS({})", number(pc), stat(R_XPT)),
                    _ => "NA()".into(),
                },
            };
            ss.text(R_SIG, 0, "σ_pt (8)", Style::Bold);
            ss.formula(R_SIG, 1, f, Style::BoldNumber);
            ch.add("σ_pt", sig, stat(R_SIG), Diff::Relative, 1e-10);
        }
        if let Some(de) = a.delta_e {
            let f = match (spec.delta_e, spec.delta_e_percent) {
                (Some(d), _) => number(d),
                (None, Some(pc)) => format!("{}/100*ABS({})", number(pc), stat(R_XPT)),
                _ => format!("3*{}", stat(R_SIG)),
            };
            ss.text(R_DE, 0, "δ_E (9.3)", Style::Bold);
            ss.formula(R_DE, 1, f, Style::Number);
            ch.add("δ_E", de, stat(R_DE), Diff::Relative, 1e-10);
        }
        ss.width(0, 44.0);
        ss.width(1, 22.0);

        // Scores of every participant.
        for (i, part) in a.participants.iter().enumerate() {
            let Some(sc) = &part.scores else { continue };
            let row = r0 + i as u32;
            let at = |c: u32| sheet_ref(s_res, row, c);
            let lab = &part.lab;
            if let Some(z) = sc.z {
                ch.add(format!("z ({lab})"), z, at(C_Z), Diff::Absolute, 1e-9);
            }
            if let Some(z) = sc.z_prime {
                ch.add(format!("z′ ({lab})"), z, at(C_ZP), Diff::Absolute, 1e-9);
            }
            if let Some(z) = sc.zeta {
                ch.add(format!("ζ ({lab})"), z, at(C_ZETA), Diff::Absolute, 1e-9);
            }
            if let Some(e) = sc.en {
                ch.add(format!("Eₙ ({lab})"), e, at(C_EN), Diff::Absolute, 1e-9);
            }
        }
        sheets.insert(0, ss);
        sheets.insert(0, sr);
    }

    // --- Homogeneity and stability (B.2, B.3, B.5) ------------------------------
    if let (Some(hs), Some(h)) = (&spec.homogeneity, &a.homogeneity) {
        let mut sh = Sheet::new(s_hom);
        sh.text(
            0,
            0,
            t(
                lang,
                "Homogeneity check (ISO 13528 B.2, B.3) and stability check (B.5)",
                "Homojenlik kontrolü (ISO 13528 B.2, B.3) ve kararlılık kontrolü (B.5)",
            ),
            Style::Title,
        );
        let g = hs.items.len();
        let m = hs.items.first().map_or(0, Vec::len);
        let (h0, cm) = (3u32, 1 + m as u32);
        sh.text(2, 0, t(lang, "Item", "Numune"), Style::Heading);
        for j in 0..m {
            sh.text(2, 1 + j as u32, format!("{} {}", t(lang, "replicate", "tekrar"), j + 1), Style::Heading);
        }
        sh.text(2, cm, t(lang, "average x̄ₜ", "ortalama x̄ₜ"), Style::Heading);
        sh.text(2, cm + 1, t(lang, "variance sₜ²", "varyans sₜ²"), Style::Heading);
        for (i, item) in hs.items.iter().enumerate() {
            let r = h0 + i as u32;
            sh.number(r, 0, (i + 1) as f64, Style::Normal);
            for (j, v) in item.iter().enumerate() {
                sh.number(r, 1 + j as u32, *v, Style::Number);
            }
            let row = format!("{}:{}", cell_ref(r, 1), cell_ref(r, m as u32));
            sh.formula(r, cm, format!("AVERAGE({row})"), Style::Number);
            sh.formula(r, cm + 1, format!("VAR({row})"), Style::Number);
        }
        let hl = h0 + g as u32 - 1;
        let means = format!("{}:{}", cell_ref(h0, cm), cell_ref(hl, cm));
        let vars = format!("{}:{}", cell_ref(h0, cm + 1), cell_ref(hl, cm + 1));
        let r = hl + 2;
        let hcell = |row: u32| sheet_ref(s_hom, row, 1);
        let sigma = match (hs.sd_pt, hs.sd_pt_percent_of_mean) {
            (Some(v), _) => Some(number(v)),
            (None, Some(pc)) => Some(format!("{}/100*ABS({})", number(pc), cell_ref(r, 1))),
            (None, None) if a.sigma_pt.is_some() && n > 0 => Some(stat(R_SIG)),
            _ => None,
        };
        let lines: Vec<(String, String)> = vec![
            ("x̄ (B.6)".into(), format!("AVERAGE({means})")),
            ("sₓ (B.7)".into(), format!("STDEV({means})")),
            ("s_w (B.8)".into(), format!("SQRT(AVERAGE({vars}))")),
            (
                "s_s (B.10)".into(),
                format!("SQRT(MAX(0,{sx}^2-{sw}^2/{m}))", sx = cell_ref(r + 1, 1), sw = cell_ref(r + 2, 1)),
            ),
            (
                "u*_bb = √(s_w²/m) (2/ν_w)^¼".into(),
                format!("SQRT({sw}^2/{m})*(2/({g}*({m}-1)))^0.25", sw = cell_ref(r + 2, 1)),
            ),
            ("F₁ (B.2.3)".into(), format!("CHIINV(0.05,{g}-1)/({g}-1)")),
            ("F_m (B.2.3)".into(), format!("(FINV(0.05,{g}-1,{g}*({m}-1))-1)/{m}")),
        ];
        for (k, (label, f)) in lines.iter().enumerate() {
            sh.text(r + k as u32, 0, label.clone(), Style::Bold);
            sh.formula(r + k as u32, 1, f.clone(), Style::Number);
        }
        let mut next = r + lines.len() as u32;
        ch.add("x̄ (B.6)", h.mean, hcell(r), Diff::Relative, 1e-12);
        ch.add("sₓ (B.7)", h.s_x, hcell(r + 1), Diff::Relative, 1e-9);
        ch.add("s_w (B.8)", h.s_w, hcell(r + 2), Diff::Relative, 1e-9);
        ch.add("s_s (B.10)", h.s_s, hcell(r + 3), if h.s_s == 0.0 { Diff::Absolute } else { Diff::Relative }, 1e-9);
        ch.add("u*_bb", h.u_bb_star, hcell(r + 4), Diff::Relative, 1e-9);
        ch.add("F₁", h.f1, hcell(r + 5), Diff::Relative, 1e-6);
        ch.add("F_m", h.fm, hcell(r + 6), Diff::Relative, 1e-6);
        if let (Some(sig), Some(crit), Some(ec)) = (sigma, h.criterion, h.expanded_criterion) {
            sh.text(next, 0, "σ_pt", Style::Bold);
            sh.formula(next, 1, sig, Style::Number);
            sh.text(next + 1, 0, "0.3 σ_pt (B.1)", Style::Bold);
            sh.formula(next + 1, 1, format!("0.3*{}", cell_ref(next, 1)), Style::Number);
            sh.text(next + 2, 0, "√c = √(F₁ (0.3σ_pt)² + F_m s_w²) (B.2.3)", Style::Bold);
            sh.formula(
                next + 2,
                1,
                format!(
                    "SQRT({}*{}^2+{}*{}^2)",
                    cell_ref(r + 5, 1),
                    cell_ref(next + 1, 1),
                    cell_ref(r + 6, 1),
                    cell_ref(r + 2, 1)
                ),
                Style::Number,
            );
            ch.add("0.3 σ_pt", crit, hcell(next + 1), Diff::Relative, 1e-10);
            ch.add("√c", ec, hcell(next + 2), Diff::Relative, 1e-6);
            next += 4;
        }
        if let (Some(st), Some(stab)) = (&spec.stability, &a.stability) {
            sh.text(next, 0, t(lang, "Stability check (B.5)", "Kararlılık kontrolü (B.5)"), Style::Heading);
            for (j, v) in st.values.iter().enumerate() {
                sh.number(next + 1, 1 + j as u32, *v, Style::Number);
            }
            sh.text(
                next + 1,
                0,
                t(lang, "results after the study period", "çalışma süresi sonundaki sonuçlar"),
                Style::Normal,
            );
            let vals = format!("{}:{}", cell_ref(next + 1, 1), cell_ref(next + 1, st.values.len() as u32));
            let y1 = st.reference.map_or_else(|| cell_ref(r, 1), number);
            let s = if h.criterion.is_some() { cell_ref(next - 4, 1) } else { stat(R_SIG) };
            let unc = match (st.u_reference, st.u_values) {
                (Some(u1), Some(u2)) => format!("+2*SQRT({}^2+{}^2)", number(u1), number(u2)),
                _ => String::new(),
            };
            sh.text(next + 2, 0, "|ȳ₁ − ȳ₂|", Style::Bold);
            sh.formula(next + 2, 1, format!("ABS({y1}-AVERAGE({vals}))"), Style::Number);
            sh.text(next + 3, 0, t(lang, "criterion 0.3 σ_pt (B.18)", "ölçüt 0,3 σ_pt (B.18)"), Style::Bold);
            sh.formula(next + 3, 1, format!("0.3*{s}{unc}"), Style::Number);
            ch.add(
                "|ȳ₁ − ȳ₂|",
                stab.difference,
                hcell(next + 2),
                Diff::Absolute,
                1e-12_f64.max(1e-12 * stab.difference.abs()),
            );
            ch.add(
                t(lang, "stability criterion", "kararlılık ölçütü"),
                stab.criterion,
                hcell(next + 3),
                Diff::Relative,
                1e-10,
            );
        }
        sh.width(0, 40.0);
        for c in 1..=cm + 1 {
            sh.width(c, 14.0);
        }
        sheets.push(sh);
    }

    let mut sc = Sheet::new(s_check);
    let fails = ch.write(&mut sc, lang);
    let info = info_sheet(p, lang, &fails, &[
        t(lang,
          "Results: the participant results of the project file; censored results (< or >) are excluded from the statistics and are not scored (5.5.3). The reported uncertainties give u(x) and U(x) (k = 2 if not stated).",
          "Sonuçlar: proje dosyasındaki katılımcı sonuçları; sansürlü sonuçlar (< veya >) istatistiklere girmez ve skorlanmaz (5.5.3). Bildirilen belirsizliklerden u(x) ve U(x) bulunur (belirtilmemişse k = 2)."),
        t(lang,
          "Statistics: median, MADe and nIQR (C.2; quartiles as QUARTILE, linear interpolation), assigned value and u(x_pt) by Formula (6) with the components of Formula (3), σ_pt (Clause 8) and δ_E (9.3).",
          "İstatistikler: medyan, MADe ve nIQR (C.2; çeyrekler QUARTILE ile, doğrusal ara değer), atanan değer ve Formül (6) ile Formül (3)'ün bileşenleriyle u(x_pt), σ_pt (Madde 8) ve δ_E (9.3)."),
        t(lang,
          "Algorithm A: every iteration in its own column; the result is the first iteration that meets the stopping rule of C.3.1.",
          "Algoritma A: her yineleme kendi sütununda; sonuç, C.3.1'in durdurma kuralını sağlayan ilk yinelemedir."),
        t(lang,
          "Homogeneity: one-way analysis of the replicate results (B.3) and the criteria of B.2. The Bayesian between-unit estimate is not recomputed here.",
          "Homojenlik: tekrar sonuçlarının tek yönlü analizi (B.3) ve B.2'nin ölçütleri. Bayesçi numuneler arası kestirim burada yeniden hesaplanmaz."),
    ]);
    let mut all = vec![info, sc];
    all.extend(sheets);
    Ok(Workbook { sheets: all })
}
