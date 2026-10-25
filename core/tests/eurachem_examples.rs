//! Validation against EURACHEM/CITAC Guide CG4, *Quantifying Uncertainty in
//! Analytical Measurement*, 3rd edition (QUAM:2012.P1).
//! https://www.eurachem.org/images/stories/Guides/pdf/QUAM2012_P1.pdf

use uncertainty_core::gum::{self, CoverageSpec, GumOptions};
use uncertainty_core::mcm::{self, IntervalKind, McmOptions, Trials};
use uncertainty_core::{Distribution, InputQuantity, Model};

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol, "{what}: got {actual}, expected {expected} ± {tol}");
}

/// Example A1, Tables A1.2 and A1.3: preparation of a 1000 mg/L Cd
/// calibration standard, c = 1000 m P / V, with u(V) = 0.07 mL as rounded in
/// the guide. The guide's spreadsheet (Kragten) value is u = 0.8633 mg/L
/// (√0.74529); the exact first-order value is 0.8638 mg/L.
#[test]
fn a1_calibration_standard_summary_budget() {
    let model = Model::builder()
        .quantity("P", Distribution::rectangular_half_width(0.9999, 0.0001))
        .quantity("m", Distribution::Normal { mean: 100.28, sd: 0.05 })
        .quantity("V", Distribution::Normal { mean: 100.0, sd: 0.07 })
        .output("c_Cd", "1000 * m * P / V")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions { coverage: CoverageSpec::Factor(2.0), ..Default::default() }).unwrap();
    close(g.estimate, 1002.69972, 1e-5, "c(Cd), Table A1.3 B9");
    close(g.combined_standard_uncertainty, 0.8633, 0.001, "u_c (0.9 mg/L)");
    // Table A1.3 row 10 contributions u(y, x_i): 0.05816, 0.49995, −0.70140.
    // The guide rounds u(P) to 0.000058; 0.0001/√3 = 0.0000577 gives 0.0579.
    close(g.budget[0].contribution, 0.05816, 0.0003, "u(y,P)");
    close(g.budget[1].contribution, 0.49995, 0.0002, "u(y,m)");
    close(g.budget[2].contribution, -0.70140, 0.0007, "u(y,V)");
    close(g.expanded_uncertainty, 1.73, 0.01, "U (1.8 mg/L after rounding u to 0.9)");
}

/// Example A1 with the volume sub-model of A1.4: calibration (±0.1 mL,
/// triangular), repeatability (s = 0.02 mL) and temperature (±0.084 mL,
/// rectangular) give u(V) = 0.066 mL (0.07 mL in the guide).
#[test]
fn a1_calibration_standard_with_volume_sub_model() {
    let model = Model::builder()
        .quantity("P", Distribution::rectangular_half_width(0.9999, 0.0001))
        .quantity("m", Distribution::Normal { mean: 100.28, sd: 0.05 })
        .quantity("V_nom", Distribution::Constant { value: 100.0 })
        .quantity("dV_cal", Distribution::triangular_half_width(0.0, 0.1))
        .quantity("dV_rep", Distribution::Normal { mean: 0.0, sd: 0.02 })
        .quantity("dV_temp", Distribution::rectangular_half_width(0.0, 100.0 * 4.0 * 2.1e-4))
        .intermediate("V", "V_nom + dV_cal + dV_rep + dV_temp")
        .output("c_Cd", "1000 * m * P / V")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g.budget[3].standard_uncertainty, 0.04, 0.001, "u(V_cal) = 0.1/√6");
    close(g.budget[5].standard_uncertainty, 0.05, 0.002, "u(V_temp) = 0.084/√3");
    let u_v = (g.budget[3].standard_uncertainty.powi(2)
        + g.budget[4].standard_uncertainty.powi(2)
        + g.budget[5].standard_uncertainty.powi(2))
    .sqrt();
    close(u_v, 0.066, 0.0005, "u(V)");
    close(g.combined_standard_uncertainty, 0.835, 0.002, "u_c");

    // MCM check (JCGM 101 8.1). The volume is dominated by rectangular and
    // triangular components, so the output PDF has lighter tails than a
    // Gaussian and the GUF 95 % interval is slightly conservative: endpoint
    // differences ≈ 0.015 mg/L. GUF is validated for one significant digit
    // in u (δ = 0.05 mg/L) but not for two (δ = 0.005 mg/L).
    let m =
        mcm::evaluate(&model, &McmOptions { trials: Trials::Fixed(2_000_000), seed: 9, ..Default::default() }).unwrap();
    close(m.standard_uncertainty, g.combined_standard_uncertainty, 0.002, "MCM u");
    let v2 = mcm::validate_gum(&g, &m, 2, IntervalKind::Symmetric);
    let v1 = mcm::validate_gum(&g, &m, 1, IntervalKind::Symmetric);
    assert!(!v2.validated && v1.validated, "{v2:?} {v1:?}");
    close(v2.d_low, 0.015, 0.006, "d_low");
    close(v2.d_high, 0.015, 0.006, "d_high");
    assert!(m.symmetric_interval.1 - m.symmetric_interval.0 < 2.0 * g.expanded_uncertainty);
}

/// Example A2, Tables A2.2 and A2.3: standardising NaOH against KHP,
/// c = 1000 m P / (M V) × rep, with the rounded standard uncertainties of the
/// guide. Spreadsheet results: c = 0.102136 mol/L, u_c = 0.000099 mol/L,
/// contributions 0.000051, 0.000034, 0.000030, −0.000002, −0.000071.
#[test]
fn a2_naoh_standardisation_table_values() {
    let model = Model::builder()
        .quantity("rep", Distribution::Normal { mean: 1.0, sd: 0.0005 })
        .quantity("m_KHP", Distribution::Normal { mean: 0.3888, sd: 0.00013 })
        .quantity("P_KHP", Distribution::Normal { mean: 1.0, sd: 0.00029 })
        .quantity("M_KHP", Distribution::Normal { mean: 204.2212, sd: 0.0038 })
        .quantity("V_T", Distribution::Normal { mean: 18.64, sd: 0.013 })
        .output("c_NaOH", "1000 * m_KHP * P_KHP / (M_KHP * V_T) * rep")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions { coverage: CoverageSpec::Factor(2.0), ..Default::default() }).unwrap();
    close(g.estimate, 0.102136, 5e-7, "c(NaOH), Table A2.3 B11");
    close(g.combined_standard_uncertainty, 0.000099, 5e-7, "u_c");
    for (row, expected) in g.budget.iter().zip([0.000051, 0.000034, 0.000030, -0.000002, -0.000071]) {
        close(row.contribution, expected, 1e-6, &format!("u(y, {})", row.name));
    }
}

/// Example A2 with the sub-models of A2.4: m = m_gross − m_tare with balance
/// linearity ±0.15 mg (rectangular) on each weighing; M(KHP) from IUPAC
/// atomic weights (rectangular, the 8, 5 and 4 atoms of one element fully
/// correlated); V_T with burette calibration ±0.03 mL (triangular) and a
/// temperature effect 19 × 2.1e-4 × 3/1.96 mL (normal).
///
/// The guide rounds 0.15/√3 = 0.087 to 0.09 mg, giving u(m) = 0.13 mg; the
/// unrounded value is 0.12 mg. Likewise u(V) = 0.0137 (0.013) mL. The
/// combined uncertainty becomes 0.000101 mol/L (guide: 0.000099); both report
/// as (0.1021 ± 0.0002) mol/L with k = 2.
#[test]
fn a2_naoh_standardisation_with_sub_models() {
    let lin = 0.15e-3;
    let model = Model::builder()
        .quantity("rep", Distribution::Normal { mean: 1.0, sd: 0.0005 })
        .quantity("m_gross", Distribution::rectangular_half_width(60.5450, lin))
        .quantity("m_tare", Distribution::rectangular_half_width(60.1562, lin))
        .quantity("P_KHP", Distribution::rectangular_half_width(1.0, 0.0005))
        .quantity("A_C", Distribution::rectangular_half_width(12.0107, 0.0008))
        .quantity("A_H", Distribution::rectangular_half_width(1.00794, 0.00007))
        .quantity("A_O", Distribution::rectangular_half_width(15.9994, 0.0003))
        .quantity("A_K", Distribution::rectangular_half_width(39.0983, 0.0001))
        .quantity("dV_cal", Distribution::triangular_half_width(0.0, 0.03))
        .quantity("dV_temp", Distribution::Normal { mean: 0.0, sd: 19.0 * 2.1e-4 * 3.0 / 1.96 })
        .intermediate("m_KHP", "m_gross - m_tare")
        .intermediate("M_KHP", "8*A_C + 5*A_H + 4*A_O + A_K")
        .intermediate("V_T", "18.64 + dV_cal + dV_temp")
        .output("c_NaOH", "1000 * m_KHP * P_KHP / (M_KHP * V_T) * rep")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions { coverage: CoverageSpec::Factor(2.0), ..Default::default() }).unwrap();
    close(g.estimate, 0.102136, 5e-7, "c(NaOH)");
    let u = |name: &str| g.budget.iter().find(|r| r.name == name).unwrap().standard_uncertainty;
    close((u("m_gross").powi(2) + u("m_tare").powi(2)).sqrt() * 1e3, 0.122, 0.0005, "u(m) / mg");
    let u_m =
        ((8.0 * u("A_C")).powi(2) + (5.0 * u("A_H")).powi(2) + (4.0 * u("A_O")).powi(2) + u("A_K").powi(2)).sqrt();
    close(u_m, 0.0038, 0.00005, "u(M_KHP)");
    close((u("dV_cal").powi(2) + u("dV_temp").powi(2)).sqrt(), 0.0137, 0.00005, "u(V_T)");
    close(g.combined_standard_uncertainty, 0.000101, 5e-7, "u_c");
    let rep = uncertainty_core::report::report_result(
        g.estimate,
        g.expanded_uncertainty,
        1,
        uncertainty_core::report::Rounding::HalfEvenFirstDiscarded,
        uncertainty_core::report::Rounding::HalfEvenFirstDiscarded,
    );
    assert_eq!((rep.value.as_str(), rep.uncertainty.as_str()), ("0.1021", "0.0002"));

    // The volume calibration term dominates the remaining budget; MCM agrees
    // with the GUM framework to two significant digits of u.
    let m = mcm::evaluate(&model, &McmOptions { trials: Trials::Fixed(1_000_000), seed: 21, ..Default::default() })
        .unwrap();
    close(m.standard_uncertainty, g.combined_standard_uncertainty, 1e-6, "MCM u");
}

fn a3_model(rep: f64, u_m: f64, v_t2: (f64, f64), v_t1: (f64, f64), u_vhcl: f64) -> Model {
    Model::builder()
        .quantity("rep", Distribution::Normal { mean: 1.0, sd: rep })
        .quantity("m_KHP", Distribution::Normal { mean: 0.3888, sd: u_m })
        .quantity("P_KHP", Distribution::Normal { mean: 1.0, sd: 0.00029 })
        .quantity("V_T2", Distribution::Normal { mean: v_t2.0, sd: v_t2.1 })
        .quantity("V_T1", Distribution::Normal { mean: v_t1.0, sd: v_t1.1 })
        .quantity("M_KHP", Distribution::Normal { mean: 204.2212, sd: 0.0038 })
        .quantity("V_HCl", Distribution::Normal { mean: 15.0, sd: u_vhcl })
        .output("c_HCl", "1000 * m_KHP * P_KHP * V_T2 / (V_T1 * M_KHP * V_HCl) * rep")
        .build()
        .unwrap()
}

/// Example A3, Tables A3.2 and A3.3: HCl titrated with NaOH standardised
/// against KHP (two-step procedure). Spreadsheet: c = 0.101387 mol/L,
/// u_c = 0.00018 mol/L (√3.34e-8), contributions 0.000101, 0.000031,
/// 0.000029, 0.000095, −0.000082, −0.0000019, −0.000074; reported
/// (0.1014 ± 0.0004) mol/L.
///
/// The summary Table A3.1 states u_c = 0.00016 mol/L, but its own inputs give
/// 0.00019; 0.00016 is the result of the triple determination (Table A3.4).
#[test]
fn a3_acid_base_titration() {
    let model = a3_model(0.001, 0.00012, (14.89, 0.014), (18.64, 0.015), 0.011);
    let opts = GumOptions { coverage: CoverageSpec::Factor(2.0), ..Default::default() };
    let g = gum::evaluate(&model, &opts).unwrap();
    close(g.estimate, 0.101387, 5e-7, "c(HCl)");
    close(g.combined_standard_uncertainty, 3.34e-8f64.sqrt(), 5e-7, "u_c (0.00018)");
    let expected = [0.000101, 0.000031, 0.000029, 0.000095, -0.000082, -0.0000019, -0.000074];
    for (row, e) in g.budget.iter().zip(expected) {
        close(row.contribution, e, 1e-6, &format!("u(y, {})", row.name));
    }
    let rep = uncertainty_core::report::report_result(
        g.estimate,
        g.expanded_uncertainty,
        1,
        uncertainty_core::report::Rounding::HalfEvenFirstDiscarded,
        uncertainty_core::report::Rounding::HalfEvenFirstDiscarded,
    );
    assert_eq!((rep.value.as_str(), rep.uncertainty.as_str()), ("0.1014", "0.0004"));

    // Table A3.1 inputs (summary page) give 0.00019, not the printed 0.00016.
    let summary = a3_model(0.001, 0.00013, (14.89, 0.015), (18.64, 0.016), 0.011);
    let g = gum::evaluate(&summary, &opts).unwrap();
    close(g.combined_standard_uncertainty, 0.000189, 1e-6, "Table A3.1 inputs");

    // Table A3.4 (triple determination): rep = 0.001/√3, V_T2 = 14.90 mL,
    // V_T1 = 18.65 mL, u(V_HCl) = 0.01 mL, u(m) = 0.00013 g → 0.00016 mol/L.
    let triple = a3_model(0.001 / 3f64.sqrt(), 0.00013, (14.90, 0.014), (18.65, 0.015), 0.01);
    let g = gum::evaluate(&triple, &opts).unwrap();
    close(g.combined_standard_uncertainty, 0.00016, 5e-6, "triple determination");
}

/// Example A3 volume sub-models (A3.4): burette calibration ±0.03 mL
/// (triangular), temperature ±4 °C (rectangular) acting on 15 and 19 mL,
/// pipette calibration ±0.02 mL (triangular). The guide's u(V_HCl) also
/// contains an unexplained 0.0037 mL term (presumably pipette repeatability).
#[test]
fn a3_volume_sub_models_and_variants() {
    let t = |v: f64| Distribution::rectangular_half_width(0.0, v * 2.1e-4 * 4.0).gum_standard_uncertainty();
    let cal = Distribution::triangular_half_width(0.0, 0.03).gum_standard_uncertainty();
    let pip = Distribution::triangular_half_width(0.0, 0.02).gum_standard_uncertainty();
    close((cal.powi(2) + t(15.0).powi(2)).sqrt(), 0.014, 0.0005, "u(V_T2)");
    close((cal.powi(2) + t(19.0).powi(2)).sqrt(), 0.015, 0.0005, "u(V_T1)");
    // With the rounded 0.008 and 0.007 mL the guide obtains 0.011 mL; the
    // unrounded components (0.0082, 0.0073) give 0.0115 mL.
    close((0.0037f64.powi(2) + 0.008f64.powi(2) + 0.007f64.powi(2)).sqrt(), 0.011, 0.0005, "u(V_HCl), rounded");
    close((0.0037f64.powi(2) + pip.powi(2) + t(15.0).powi(2)).sqrt(), 0.0115, 0.00005, "u(V_HCl)");
    let u_m = 2f64.sqrt() * Distribution::rectangular_half_width(0.0, 0.15e-3).gum_standard_uncertainty();
    close(u_m, 0.00012, 0.000005, "u(m_KHP)");

    // Mean room temperature 25 °C: c = c₂₀ / [1 − α(T − 20)] = 0.10149 mol/L.
    // (The guide prints M = 204.2236 in this formula, a typo for 204.2212.)
    let model = Model::builder()
        .quantity("m_KHP", Distribution::Constant { value: 0.3888 })
        .quantity("V_T2", Distribution::Constant { value: 14.89 })
        .quantity("V_T1", Distribution::Constant { value: 18.64 })
        .quantity("M_KHP", Distribution::Constant { value: 204.2212 })
        .quantity("T", Distribution::Constant { value: 25.0 })
        .output("c", "1000 * m_KHP * V_T2 / (M_KHP * V_T1 * 15 * (1 - 2.1e-4 * (T - 20)))")
        .build()
        .unwrap();
    close(model.evaluate(&model.estimates()), 0.10149, 5e-6, "c at 25 °C");

    // Visual end-point (phenolphthalein): end-point repeatability 0.03 mL in
    // each titration volume → u(V_T1) = 0.034, u(V_T2) = 0.033 mL,
    // u_c ≈ 0.0003 mol/L.
    let u_t1 = (cal.powi(2) + t(19.0).powi(2) + 0.03f64.powi(2)).sqrt();
    let u_t2 = (cal.powi(2) + t(15.0).powi(2) + 0.03f64.powi(2)).sqrt();
    close(u_t1, 0.034, 0.0005, "u(V_T1) visual");
    close(u_t2, 0.033, 0.0005, "u(V_T2) visual");
    let visual = a3_model(0.001, 0.00012, (14.89, u_t2), (18.64, u_t1), 0.011);
    let g = gum::evaluate(&visual, &GumOptions::default()).unwrap();
    close(g.combined_standard_uncertainty, 0.0003, 0.00005, "u_c visual");
}

/// Example A4, Tables A4.2–A4.5: organophosphorus pesticides in bread from
/// in-house validation data. Precision from 15 duplicates in different runs
/// (s of normalised differences 0.382 → 0.382/√2 = 0.27); recovery 0.90 with
/// u = 0.28/√42 = 0.0432, significantly different from 1 (t = 2.31) so the
/// result is corrected; homogeneity 0.20 (binomial scenario (b));
/// u_c/P = √(0.27² + 0.048² + 0.2²) = 0.34; Kragten u = 0.377 at P = 1.1111.
///
/// Table A4.2 contains typographical errors in derived columns (mean 0.75
/// for 0.075, difference −0.10 for −0.01, mean 0.10 for 0.01); the
/// statistics are recomputed here from D1 and D2.
#[test]
fn a4_pesticides_in_bread() {
    use uncertainty_core::topdown::*;
    let duplicates = [
        (1.30, 1.30),
        (1.30, 0.90),
        (0.57, 0.53),
        (0.16, 0.26),
        (0.65, 0.58),
        (0.04, 0.04),
        (0.08, 0.09),
        (0.02, 0.02),
        (0.01, 0.02),
        (0.02, 0.01),
        (0.03, 0.02),
        (0.04, 0.06),
        (0.07, 0.08),
        (0.01, 0.01),
        (0.06, 0.03),
    ];
    let prec = relative_sd_from_normalised_differences(&duplicates).unwrap();
    close(prec.sd * 2f64.sqrt(), 0.382, 0.0005, "s of normalised differences");
    close(prec.sd, 0.27, 0.0005, "relative intermediate precision");

    // Recovery (Table A4.3, bread): mean 0.90, s 0.28, n 42.
    let r = MeanRecovery { mean: 0.90, standard_uncertainty: 0.28 / 42f64.sqrt(), tests: 42 };
    close(r.standard_uncertainty, 0.0432, 0.00005, "u(Rec)");
    let sig = recovery_significance(&r, 41.0, 0.95);
    close(sig.statistic, 2.31, 0.005, "t");
    // The guide quotes t_crit = 2.021 (that is t(40)); t(41) = 2.0195.
    close(sig.critical, 2.0195, 0.0005, "t_crit(41)");
    assert!(sig.significant);
    close(sig.relative_uncertainty, 0.048, 0.0005, "u(Rec)/Rec");

    // A4.6 binomial inhomogeneity scenarios for a 12 × 12 × 24 cm loaf, 2 cm
    // portions, 15 portions homogenised: (a) 0.58, (b) 0.20, (c) 0.33, and
    // 0.09 when half of the outer material is lost.
    close(binomial_inhomogeneity_rsd(15.0, 1.0 / 6.0, 1.0, 0.0), 0.58, 0.005, "scenario (a)");
    let p_outer = (12.0 * 12.0 * 24.0 - 8.0 * 8.0 * 20.0) / (12.0 * 12.0 * 24.0);
    close(p_outer, 0.63, 0.005, "p₁ (outer 2 cm)");
    close(binomial_inhomogeneity_rsd(15.0, p_outer, 1.0, 0.0), 0.20, 0.005, "scenario (b)");
    close(binomial_inhomogeneity_rsd(15.0, 1.0 - p_outer, 1.0, 0.0), 0.33, 0.01, "scenario (c)");
    close(binomial_inhomogeneity_rsd(15.0, 1.0 - p_outer, 2.0, 1.0), 0.09, 0.005, "scenario (c), 50 % loss");

    // Combination (Eq. A4.1, multiplicative): GUM and the Kragten table.
    let model = Model::builder()
        .quantity("F_I", Distribution::Normal { mean: 1.0, sd: 0.27 })
        .quantity("Rec", Distribution::Normal { mean: 0.9, sd: 0.043 })
        .quantity("F_hom", Distribution::Normal { mean: 1.0, sd: 0.2 })
        .output("P_op", "F_I * F_hom / Rec")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g.estimate, 1.1111, 0.00005, "P (nominal)");
    close(g.combined_standard_uncertainty / g.estimate, 0.34, 0.005, "u_c/P");
    close(g.combined_standard_uncertainty, 0.377, 0.0005, "u_c (Table A4.5: 0.377)");
    // With relative uncertainties of 27 % the model is visibly non-linear in
    // Rec: Kragten's one-sided difference gives −0.0507, the derivative −0.0531.
    close(g.budget[1].contribution, -0.0531, 0.0005, "u(y, Rec)");
}

/// Example A5, Tables A5.2–A5.4: cadmium released from ceramic ware
/// (empirical method BS 6748). The calibration (Table A5.2, 15 points) gives
/// b₁ = 0.2410 (0.0050), b₀ = 0.0087 (0.0029), r = 0.997, S = 0.005486,
/// S_xx = 1.2, and u(c₀) = 0.018 mg/L for c₀ = 0.26 mg/L from p = 2 readings.
#[test]
fn a5_cadmium_release_calibration() {
    use uncertainty_core::calibration::StraightLine;
    let conc = [0.1, 0.3, 0.5, 0.7, 0.9];
    let absorb = [
        [0.028, 0.029, 0.029],
        [0.084, 0.083, 0.081],
        [0.135, 0.131, 0.133],
        [0.180, 0.181, 0.183],
        [0.215, 0.230, 0.216],
    ];
    let mut x = Vec::new();
    let mut y = Vec::new();
    for (c, reps) in conc.iter().zip(absorb) {
        for a in reps {
            x.push(*c);
            y.push(a);
        }
    }
    let f = StraightLine::fit(&x, &y).unwrap();
    close(f.slope, 0.2410, 0.00005, "b₁");
    close(f.var_slope.sqrt(), 0.0050, 0.00005, "s(b₁)");
    close(f.intercept, 0.0087, 0.00005, "b₀");
    close(f.var_intercept.sqrt(), 0.0029, 0.00005, "s(b₀)");
    close(f.correlation, 0.997, 0.0005, "r");
    close(f.residual_sd, 0.005486, 0.0000005, "S");
    close(f.sxx, 1.2, 1e-12, "S_xx");
    close(f.u_predicted_x(0.26, 2, 1.0), 0.018, 0.0005, "u(c₀)");
}

/// Example A5 budget. Table A5.3/A5.4 values: r = 0.015065 mg/dm², Kragten
/// u = 0.001465; contributions 0.001043, 0.000082, −0.000483, 0.000012,
/// 0.000015, 0.000904; result (0.015 ± 0.003) mg/dm².
///
/// Sub-models: u(V_L) = 1.83 mL reproduced. The area a_V = πd²/4 with
/// d = 2.70 dm (the guide's formula shows 2.77, a typo) gives 5.73 dm²; its
/// components 0.042 and 0.146 dm² combine to 0.152 dm², not the 0.19 dm²
/// printed and used; with 0.152 dm² u_c is 0.00144 instead of 0.00147, and
/// the reported result is unchanged.
#[test]
fn a5_cadmium_release_budget() {
    let build = |u_area: f64| {
        Model::builder()
            .quantity("c0", Distribution::Normal { mean: 0.26, sd: 0.018 })
            .quantity("V_L", Distribution::Normal { mean: 0.332, sd: 0.0018 })
            .quantity("a_V", Distribution::Normal { mean: 5.73, sd: u_area })
            .quantity("f_acid", Distribution::Normal { mean: 1.0, sd: 0.0008 })
            .quantity("f_time", Distribution::Normal { mean: 1.0, sd: 0.001 })
            .quantity("f_temp", Distribution::Normal { mean: 1.0, sd: 0.06 })
            .output("r", "c0 * V_L / a_V * f_acid * f_time * f_temp")
            .build()
            .unwrap()
    };
    let g =
        gum::evaluate(&build(0.19), &GumOptions { coverage: CoverageSpec::Factor(2.0), ..Default::default() }).unwrap();
    close(g.estimate, 0.015065, 5e-7, "r");
    // Kragten's one-sided differences give 0.001465; the exact first-order
    // value is 0.001470 (difference from the 1/a_V non-linearity).
    close(g.combined_standard_uncertainty, 0.001470, 5e-7, "u_c (first order)");
    close(g.combined_standard_uncertainty, 0.001465, 6e-6, "u_c (Kragten 0.001465)");
    for (row, e) in g.budget.iter().zip([0.001043, 0.000082, -0.000483, 0.000012, 0.000015, 0.000904]) {
        // Kragten's finite difference for a_V (−0.000483) differs from the
        // derivative (−0.000500) because of the 1/a_V non-linearity.
        let tol = if row.name == "a_V" { 0.00002 } else { 1e-6 };
        close(row.contribution, e, tol, &format!("u(y, {})", row.name));
    }
    let rep = uncertainty_core::report::report_result(
        g.estimate,
        g.expanded_uncertainty,
        1,
        uncertainty_core::report::Rounding::HalfEvenFirstDiscarded,
        uncertainty_core::report::Rounding::HalfEvenFirstDiscarded,
    );
    assert_eq!((rep.value.as_str(), rep.uncertainty.as_str()), ("0.015", "0.003"));

    // Volume sub-model: filling 0.995 ± 0.005 (triangular), temperature
    // ±2 °C (rectangular), reading 1 % (triangular), calibration ±2.5 mL
    // (triangular) → 1.83 mL.
    let tri = |a: f64| Distribution::triangular_half_width(0.0, a).gum_standard_uncertainty();
    let rect = |a: f64| Distribution::rectangular_half_width(0.0, a).gum_standard_uncertainty();
    let u_v = (tri(0.005 * 332.0).powi(2)
        + rect(2.1e-4 * 332.0 * 2.0).powi(2)
        + tri(0.01 * 332.0).powi(2)
        + tri(2.5).powi(2))
    .sqrt();
    close(u_v, 1.83, 0.005, "u(V_L) / mL");

    // Area sub-model.
    let area = Model::builder()
        .quantity("d", Distribution::Normal { mean: 2.70, sd: 0.01 })
        .quantity("f_shape", Distribution::Normal { mean: 1.0, sd: 0.05 / 1.96 })
        .output("a_V", "pi * d^2 / 4 * f_shape")
        .build()
        .unwrap();
    let ga = gum::evaluate(&area, &GumOptions::default()).unwrap();
    close(ga.estimate, 5.73, 0.005, "a_V");
    close(ga.budget[0].contribution, 0.042, 0.0005, "u(a_V) from d");
    close(ga.budget[1].contribution, 0.146, 0.0005, "u(a_V) from shape");
    close(ga.combined_standard_uncertainty, 0.152, 0.0005, "u(a_V) (printed 0.19)");
    let g2 = gum::evaluate(&build(ga.combined_standard_uncertainty), &GumOptions::default()).unwrap();
    close(g2.combined_standard_uncertainty, 0.00144, 0.000005, "u_c with u(a_V) = 0.152");
}

/// Example A6, Tables A6.2–A6.5: crude fibre (empirical method) using the
/// collaborative-trial reproducibility s_R plus an allowance for drying to
/// constant weight (±2 mg, rectangular → 0.115 % m/m for a 1 g sample),
/// included only where it exceeds s_R/3.
///
/// Table A6.5 prints U = 0.12 % m/m at 10 % fibre; 2 × 0.6 = 1.2 % m/m
/// (the percentage column, 12 %, is correct).
#[test]
fn a6_crude_fibre() {
    use uncertainty_core::topdown::*;
    // Table A6.3: RSD contributions = sensitivity × parameter uncertainty
    // (digestion time ±5 min, rectangular: 5/√3 = 2.887, printed 2.89).
    close(5.0 / 3f64.sqrt(), 2.89, 0.005, "u(digestion time)");
    for (c, u, e) in [(0.23, 0.0013, 0.00030), (0.21, 0.0023, 0.00048), (0.0031, 2.89, 0.0090), (0.0025, 2.89, 0.0072)]
    {
        // Printed to two significant figures.
        close(c * u, e, 0.05 * e, "Table A6.3");
    }
    let drying = Distribution::rectangular_half_width(0.0, 0.002).gum_standard_uncertainty() / 1.0 * 100.0;
    close(drying, 0.115, 0.0005, "drying allowance, % m/m");
    // Table A6.4/A6.5.
    for (content, s_r, uc, u_exp, u_pct) in
        [(2.5, 0.29, 0.31, 0.62, 25.0), (5.0, 0.4, 0.4, 0.8, 16.0), (10.0, 0.6, 0.6, 1.2, 12.0)]
    {
        let mut comps = vec![Component::absolute("s_R", s_r)];
        if drying > s_r / 3.0 {
            comps.push(Component::absolute("drying", drying));
        }
        let r = TopDownBudget::new(comps, 2.0).evaluate(content);
        close(r.combined_standard_uncertainty, uc, 0.005, "u_c");
        close(r.expanded_uncertainty, u_exp, 0.01, "U");
        close(100.0 * r.expanded_uncertainty / content, u_pct, 0.5, "U %");
    }
}

/// Example A7, Tables A7.3–A7.8: lead in water by double IDMS (Eq. 5, 8, 12).
/// Reference values: c_z = 0.092605 µmol/g with u = 0.000028; M(Pb, Assay 1)
/// = 207.2103 g/mol; c_x = 0.053738 µmol/g (replicate 1) with u_c = 0.00018
/// (type A uncertainties divided by √8) and 0.00041 without that division;
/// ΣK_bias contributes 37.6 % of the final variance.
#[test]
fn a7_lead_by_double_idms() {
    // Eq. (8): amount content of the primary assay standard.
    let assay = Model::builder()
        .quantity("m1", Distribution::Normal { mean: 0.36544, sd: 0.00005 })
        .quantity("d1", Distribution::Normal { mean: 196.14, sd: 0.03 })
        .quantity("m2", Distribution::Normal { mean: 1.0292, sd: 0.0002 })
        .quantity("d2", Distribution::Normal { mean: 99.931, sd: 0.01 })
        .quantity("w", Distribution::Normal { mean: 0.99999, sd: 0.000005 })
        .quantity("M", Distribution::Normal { mean: 207.2104, sd: 0.0010 })
        .output("c_z", "1e6 * m2 / d2 * m1 * w / d1 / M")
        .build()
        .unwrap();
    let gz = gum::evaluate(&assay, &GumOptions::default()).unwrap();
    close(gz.estimate, 0.092605, 1e-6, "c_z / µmol g⁻¹");
    close(gz.combined_standard_uncertainty, 0.000028, 5e-7, "u(c_z)");

    // Eq. (5) with K = K0 + K_bias for every ratio; final uncertainties of
    // Table A7.8 (type A divided by √8).
    let idms = |div: f64| {
        let a = |name: &str, x: f64, u: f64| InputQuantity::new(name, Distribution::Normal { mean: x, sd: u / div });
        let b = |name: &str, x: f64, u: f64| InputQuantity::new(name, Distribution::Normal { mean: x, sd: u });
        let mut m = Model::builder()
            .input(b("c_z", 0.092605, 0.000028))
            .input(b("m_x", 1.0440, 0.0002))
            .input(b("m_y", 1.1360, 0.0002))
            .input(b("m_y2", 1.0654, 0.0002))
            .input(b("m_z", 1.1029, 0.0002))
            .input(a("K0_b", 0.9987, 0.0025))
            .input(a("K0_b2", 0.9983, 0.0025))
            .input(a("K0_x1", 0.9992, 0.0025))
            .input(a("K0_x3", 1.0004, 0.0035))
            .input(a("K0_x4", 1.001, 0.006))
            .input(a("K0_y1", 0.9999, 0.0025))
            .input(a("K0_z1", 0.9989, 0.0025))
            .input(a("K0_z3", 0.9993, 0.0035))
            .input(a("K0_z4", 1.0002, 0.006))
            .input(a("R_b", 0.29360, 0.00073))
            .input(a("R_b2", 0.5050, 0.0013))
            .input(a("R_x1", 2.1402, 0.0054))
            .input(a("R_x3", 0.9142, 0.0032))
            .input(a("R_x4", 0.05901, 0.00035))
            .input(a("R_y1", 0.00064, 0.00004))
            .input(a("R_z1", 2.1429, 0.0054))
            .input(a("R_z3", 0.9147, 0.0032))
            .input(a("R_z4", 0.05870, 0.00035))
            .input(a("c_blank", 4.5e-7, 4.0e-7));
        for k in ["b", "b2", "x1", "x3", "x4", "y1", "z1", "z3", "z4"] {
            m = m.input(b(&format!("Kbias_{k}"), 0.0, 0.001));
        }
        for k in ["b", "b2", "x1", "x3", "x4", "y1", "z1", "z3", "z4"] {
            m = m.intermediate(&format!("K_{k}"), &format!("K0_{k} + Kbias_{k}"));
        }
        m.intermediate("sum_x", "K_x1*R_x1 + 1 + K_x3*R_x3 + K_x4*R_x4")
            .intermediate("sum_z", "K_z1*R_z1 + 1 + K_z3*R_z3 + K_z4*R_z4")
            .output(
                "c_x",
                "c_z * m_y/m_x * m_z/m_y2 * (K_y1*R_y1 - K_b*R_b)/(K_b*R_b - K_x1*R_x1) \
                 * (K_b2*R_b2 - K_z1*R_z1)/(K_y1*R_y1 - K_b2*R_b2) * sum_x/sum_z - c_blank",
            )
            .build()
            .unwrap()
    };
    let g =
        gum::evaluate(&idms(8f64.sqrt()), &GumOptions { coverage: CoverageSpec::Factor(2.0), ..Default::default() })
            .unwrap();
    close(g.estimate, 0.053738, 3e-6, "c_x / µmol g⁻¹");
    close(g.combined_standard_uncertainty, 0.00018, 5e-6, "u_c(c_x)");
    let bias_share: f64 = g.budget.iter().filter(|r| r.name.starts_with("Kbias")).map(|r| r.variance_share).sum();
    close(bias_share * 100.0, 37.6, 2.0, "ΣK_bias contribution %");
    let share = |n: &str| g.budget.iter().find(|r| r.name == n).unwrap().variance_share * 100.0;
    close(share("R_b2"), 12.7, 1.0, "R'_b contribution %");
    close(share("K0_b2"), 11.9, 1.0, "K0(b') contribution %");
    close(share("R_b"), 9.5, 1.0, "R_b contribution %");
    // Without dividing the type A uncertainties by √8.
    let g1 = gum::evaluate(&idms(1.0), &GumOptions::default()).unwrap();
    close(g1.combined_standard_uncertainty, 0.00041, 1e-5, "experimental u_c");
    // Reported (0.05370 ± 0.00036) µmol/g uses the mean of four replicates.
    close(2.0 * 0.00018, 0.00036, 1e-12, "U");
}

/// Example A7, Eq. (12): molar mass of Pb in the assay material from
/// mass-bias-corrected isotope ratios: 207.2103 g/mol.
#[test]
fn a7_molar_mass_of_lead() {
    let model = Model::builder()
        .quantity("K_z1", Distribution::Normal { mean: 0.9989, sd: 0.0025 })
        .quantity("K_z3", Distribution::Normal { mean: 0.9993, sd: 0.0035 })
        .quantity("K_z4", Distribution::Normal { mean: 1.0002, sd: 0.0060 })
        .quantity("R_z1", Distribution::Normal { mean: 2.1429, sd: 0.0054 })
        .quantity("R_z3", Distribution::Normal { mean: 0.9147, sd: 0.0032 })
        .quantity("R_z4", Distribution::Normal { mean: 0.05870, sd: 0.00035 })
        .quantity("M1", Distribution::Normal { mean: 207.976636, sd: 0.000003 })
        .quantity("M2", Distribution::Normal { mean: 205.974449, sd: 0.000003 })
        .quantity("M3", Distribution::Normal { mean: 206.975880, sd: 0.000003 })
        .quantity("M4", Distribution::Normal { mean: 203.973028, sd: 0.000003 })
        .output("M", "(K_z1*R_z1*M1 + M2 + K_z3*R_z3*M3 + K_z4*R_z4*M4) / (K_z1*R_z1 + 1 + K_z3*R_z3 + K_z4*R_z4)")
        .build()
        .unwrap();
    close(model.evaluate(&model.estimates()), 207.2103, 0.0002, "M(Pb, Assay 1)");
}
