//! Validation against EA-4/02 M:2022, *Evaluation of the Uncertainty of
//! Measurement in Calibration*, Supplements 1 and 2 (examples S2–S13).
//! https://european-accreditation.org/wp-content/uploads/2018/10/EA-4-02.pdf
//!
//! Units follow the examples (mg, mΩ, nm, °C, µV, dB, µm, mK, l).

use uncertainty_core::distribution::TypeA;
use uncertainty_core::gum::{self, CoverageSpec, GumOptions};
use uncertainty_core::special::trapezoidal_coverage_factor;
use uncertainty_core::{Distribution, InputQuantity, Model};

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol, "{what}: got {actual}, expected {expected} ± {tol}");
}

fn k2() -> GumOptions {
    GumOptions { coverage: CoverageSpec::Factor(2.0), ..Default::default() }
}

fn normal(x: f64, u: f64) -> Distribution {
    Distribution::Normal { mean: x, sd: u }
}

fn rect(x: f64, a: f64) -> Distribution {
    Distribution::rectangular_half_width(x, a)
}

fn contributions(g: &gum::GumResult) -> Vec<f64> {
    g.budget.iter().map(|r| r.contribution.abs()).collect()
}

/// S2: weight of nominal value 10 kg (mg relative to 10 kg).
#[test]
fn s2_weight_10_kg() {
    // S2.8: ABBA differences +10, +30, +20 mg; u(δm) = s_p/√3 = 25/√3.
    let model = Model::builder()
        .quantity("m_S", normal(5.0, 45.0 / 2.0))
        .quantity("dm_D", rect(0.0, 15.0))
        .quantity("dm", normal((10.0 + 30.0 + 20.0) / 3.0, 25.0 / 3f64.sqrt()))
        .quantity("dm_C", rect(0.0, 10.0))
        .quantity("dB", rect(0.0, 1e-6 * 10_000_000.0))
        .output("m_X", "m_S + dm_D + dm + dm_C + dB")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &k2()).unwrap();
    close(g.estimate, 25.0, 1e-12, "m_X − 10 kg");
    for (c, e) in contributions(&g).iter().zip([22.5, 8.66, 14.4, 5.77, 5.77]) {
        // Printed to three significant figures (14.43 → 14.4).
        close(*c, e, 0.05, "contribution");
    }
    // The guide combines the rounded contributions (14.4 for 14.43 mg):
    // 29.24 mg → 29.2 mg, U = 58 mg; unrounded 29.26 mg, U = 58.5 mg.
    close(g.combined_standard_uncertainty, 29.26, 0.005, "u(m_X)");
    let rounded: f64 = [22.5f64, 8.66, 14.4, 5.77, 5.77].iter().map(|v| v * v).sum::<f64>().sqrt();
    close(rounded, 29.2, 0.05, "u(m_X) from printed contributions");
    close(2.0 * rounded, 58.0, 0.5, "U (58 mg)");
}

/// S3: nominal 10 kΩ standard resistor (mΩ relative to 10 kΩ, r as ratio).
#[test]
fn s3_resistor_10_kohm() {
    // S3.8: five observed ratios → r̄ = 1.000 010 5, s = 0.158e-6.
    let obs = [1.000_010_4, 1.000_010_7, 1.000_010_6, 1.000_010_3, 1.000_010_5];
    let ta = TypeA::from_observations(&obs).unwrap();
    close(ta.mean, 1.000_010_5, 1e-12, "r̄");
    close(ta.std_dev, 0.158e-6, 0.0005e-6, "s(r)");
    close(ta.std_uncertainty, 0.0707e-6, 0.0005e-6, "u(r)");
    let model = Model::builder()
        .quantity("R_S", normal(10_000_053.0, 5.0 / 2.0))
        .quantity("dR_D", rect(20.0, 10.0))
        .quantity("dR_TS", rect(0.0, 2.75))
        .quantity("r_C", Distribution::triangular_half_width(1.0, 1.0e-6))
        .quantity("r", normal(ta.mean, ta.std_uncertainty))
        .quantity("dR_TX", rect(0.0, 5.5))
        .output("R_X", "(R_S + dR_D + dR_TS) * r_C * r - dR_TX")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &k2()).unwrap();
    close(g.estimate / 1000.0, 10_000.178, 0.0005, "R_X / Ω");
    // Inputs in model order: R_S, δR_D, δR_TS, r_C, r, δR_TX.
    for (c, e) in contributions(&g).iter().zip([2.5, 5.8, 1.6, 4.1, 0.7, 3.2]) {
        close(*c, e, 0.05, "contribution");
    }
    close(g.combined_standard_uncertainty, 8.33, 0.005, "u(R_X)");
    close(g.expanded_uncertainty, 17.0, 0.5, "U (17 mΩ)");
}

/// S4: gauge block of nominal length 50 mm (nm). The product δα·δt of two
/// zero-expectation quantities needs the second-order term (S4.13):
/// u = u(δα)·u(δt) = (2e-6/√6)(0.5/√3) = 0.236e-6.
#[test]
fn s4_gauge_block_50_mm() {
    let model = Model::builder()
        .quantity("l_S", normal(50_000_020.0, 15.0))
        .quantity("dl_D", Distribution::triangular_half_width(0.0, 30.0))
        .quantity("dl", normal(-94.0, 12.0 / 5f64.sqrt()))
        .quantity("dl_C", rect(0.0, 32.0))
        .quantity("alpha", Distribution::Constant { value: 11.5e-6 })
        .quantity("dt", rect(0.0, 0.05))
        .quantity("d_alpha", Distribution::triangular_half_width(0.0, 2e-6))
        .quantity("theta", rect(0.0, 0.5))
        .quantity("dl_V", rect(0.0, 6.7))
        .output("l_X", "l_S + dl_D + dl + dl_C - 50000000*(alpha*dt + d_alpha*theta) - dl_V")
        .build()
        .unwrap();
    // First-order terms only miss the δα·θ product entirely.
    let g1 = gum::evaluate(&model, &k2()).unwrap();
    close(g1.budget[6].contribution, 0.0, 1e-12, "first order misses δα·θ");
    let g = gum::evaluate(&model, &GumOptions { higher_order: true, ..k2() }).unwrap();
    close(g.estimate, 49_999_926.0, 1e-6, "l_X / nm");
    let u = |n: &str| g.budget.iter().find(|r| r.name == n).unwrap();
    close(u("l_S").contribution, 15.0, 1e-9, "l_S");
    close(u("dl_D").contribution, 12.2, 0.05, "δl_D");
    close(u("dl").contribution, 5.37, 0.005, "δl");
    close(u("dl_C").contribution, 18.5, 0.05, "δl_C");
    close(u("dt").contribution.abs(), 16.6, 0.05, "δt");
    close(u("d_alpha").standard_uncertainty * u("theta").standard_uncertainty, 0.236e-6, 0.0005e-6, "u(δα·θ)");
    close(g.higher_order_variance.sqrt(), 11.8, 0.05, "δα·θ contribution");
    close(u("dl_V").contribution.abs(), 3.87, 0.005, "δl_V");
    close(g.combined_standard_uncertainty, 34.3, 0.05, "u(l_X)");
    close(g.expanded_uncertainty, 69.0, 0.5, "U (69 nm)");
}

/// S5: type N thermocouple at 1000 °C, two budgets.
///
/// The printed contribution of δt (24.5 µV) is inconsistent with
/// 0.641 °C × 38.46 µV/°C = 24.65 µV; the printed total 25.0 µV follows from
/// 24.65. The estimate of δt = t − t_X is −0.5 °C (printed 0.5 °C), giving
/// V_X = 36 248 − 19.2 = 36 229 µV.
#[test]
fn s5_thermocouple_1000_c() {
    let (cs, cs0, cx, cx0) = (0.077, 0.189, 0.026, 0.039);
    let furnace = Model::builder()
        .quantity("t_S", normal(1000.5, 0.10))
        .quantity("V_iS1", normal(0.0, 2.0 / 2.0))
        .quantity("V_iS2", rect(0.0, 0.5))
        .quantity("V_R", rect(0.0, 2.0))
        .quantity("t_0S", rect(0.0, 0.1))
        .quantity("dt_S", normal(0.0, 0.3 / 2.0))
        .quantity("dt_D", rect(0.0, 0.3))
        .quantity("dt_F", rect(0.0, 1.0))
        .output("t_X", &format!("t_S + {cs}*(V_iS1 + V_iS2 + V_R) - {cs}/{cs0}*t_0S + dt_S + dt_D + dt_F"))
        .build()
        .unwrap();
    let g = gum::evaluate(&furnace, &k2()).unwrap();
    close(g.budget[4].sensitivity, -0.407, 0.0005, "c(δt_0S)");
    for (c, e) in contributions(&g).iter().zip([0.10, 0.077, 0.022, 0.089, 0.023, 0.15, 0.173, 0.577]) {
        close(*c, e, 0.0006, "furnace contribution");
    }
    close(g.combined_standard_uncertainty, 0.641, 0.0005, "u(t_X)");
    close(g.expanded_uncertainty, 1.3, 0.05, "U(t_X)");
    let u_tx = g.combined_standard_uncertainty;

    let emf = Model::builder()
        .quantity("V_iX", normal(36_248.0, 1.6))
        .quantity("V_iX1", normal(0.0, 1.0))
        .quantity("V_iX2", rect(0.0, 0.5))
        .quantity("V_R", rect(0.0, 2.0))
        .quantity("V_LX", rect(0.0, 5.0))
        .quantity("dt", normal(1000.0 - 1000.5, u_tx))
        .quantity("t_0X", rect(0.0, 0.1))
        .output("V_X", &format!("V_iX + V_iX1 + V_iX2 + V_R + V_LX + dt/{cx} - t_0X/{cx0}"))
        .build()
        .unwrap();
    let g = gum::evaluate(&emf, &k2()).unwrap();
    close(g.estimate, 36_229.0, 0.5, "V_X");
    close(g.budget[5].sensitivity, 38.5, 0.05, "c(δt)");
    close(g.budget[5].contribution, 24.65, 0.01, "u(y, δt) (printed 24.5)");
    close(g.budget[6].contribution.abs(), 1.48, 0.005, "u(y, δt_0X)");
    close(g.combined_standard_uncertainty, 25.0, 0.05, "u(V_X)");
    close(g.expanded_uncertainty, 50.0, 0.5, "U(V_X)");
}

/// S6: power sensor (calibration factor) at 18 GHz. (The heading says
/// 19 GHz; the text and result say 18 GHz.) Mismatch limits 2|Γ_G||Γ|,
/// U-shaped; ν_eff ≈ 310 from the three readings of p.
#[test]
fn s6_power_sensor() {
    let obs = [(1.0001, 0.9924, 1.0001, 0.9698), (1.0000, 0.9942, 1.0000, 0.9615), (0.9999, 0.9953, 1.0001, 0.9792)];
    let p: Vec<f64> = obs.iter().map(|(sr, sc, xr, xc)| sr * xc / (sc * xr)).collect();
    for (pi, e) in p.iter().zip([0.9772, 0.9671, 0.9836]) {
        close(*pi, e, 0.00005, "p_i");
    }
    let ta = TypeA::from_observations(&p).unwrap();
    close(ta.mean, 0.9760, 0.00005, "p̄");
    close(ta.std_dev, 0.0083, 0.00005, "s(p)");
    let mismatch = |g: f64, s: f64| Distribution::arcsine_half_width(1.0, 2.0 * g * s);
    let model = Model::builder()
        .quantity("K_S", normal(0.957, 0.011 / 2.0))
        .quantity("dK_D", rect(-0.001, 0.002))
        .quantity("M_Sr", mismatch(0.02, 0.02))
        .quantity("M_Sc", mismatch(0.07, 0.10))
        .quantity("M_Xr", mismatch(0.02, 0.02))
        .quantity("M_Xc", mismatch(0.07, 0.12))
        .quantity("p_Cr", normal(1.0, 0.00142))
        .quantity("p_Cc", normal(1.0, 0.000142))
        .input(InputQuantity::new("p", normal(ta.mean, ta.std_uncertainty)).with_dof(ta.dof))
        .output("K_X", "(K_S + dK_D) * M_Sr * M_Xc / (M_Sc * M_Xr) * p_Cr * p_Cc * p")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &k2()).unwrap();
    close(g.estimate, 0.933, 0.0005, "K_X");
    let u = |n: &str| g.budget.iter().find(|r| r.name == n).unwrap().standard_uncertainty;
    close(u("M_Sr"), 0.0006, 0.00005, "u(M_Sr)");
    close(u("M_Sc"), 0.0099, 0.00005, "u(M_Sc)");
    close(u("M_Xc"), 0.0119, 0.00005, "u(M_Xc)");
    close(g.combined_standard_uncertainty, 0.016_23, 0.0001, "u(K_X)");
    close(g.expanded_uncertainty, 0.032, 0.0005, "U");
    close(g.effective_dof, 310.0, 15.0, "ν_eff ≈ 310");
}

/// S7: coaxial step attenuator, 30 dB at 10 GHz. Mismatch from Eq. (S7.2).
///
/// The budget prints the contribution of δL_ib as 0.0019 dB although its
/// standard uncertainty is 0.0003 dB; the printed total 0.0223 dB includes
/// the 0.0019 value (0.0222 dB without it). Reported (30.043 ± 0.045) dB.
#[test]
fn s7_step_attenuator() {
    let (gs, gl) = (0.03f64, 0.03f64);
    let (s11a, s11b, s22a, s22b, s21a, s21b) = (0.05f64, 0.09f64, 0.01f64, 0.01f64, 0.95f64, 0.031f64);
    let u_lm = 8.686 / 2f64.sqrt()
        * (gs * gs * (s11a * s11a + s11b * s11b)
            + gl * gl * (s22a * s22a + s22b * s22b)
            + gs * gs * gl * gl * (s21a.powi(4) + s21b.powi(4)))
        .sqrt();
    close(u_lm, 0.0198, 0.00005, "u(δL_M)");
    let obs = [30.033, 30.058, 30.018, 30.052];
    let ta = TypeA::from_observations(&obs).unwrap();
    close(ta.mean, 30.040, 0.0005, "L̄_S");
    close(ta.std_dev, 0.018, 0.0005, "s(L_S)");
    let model = Model::builder()
        .input(InputQuantity::new("L_S", normal(ta.mean, ta.std_uncertainty)).with_dof(3.0))
        .quantity("dL_S", normal(0.003, 0.005 / 2.0))
        .quantity("dL_D", rect(0.0, 0.002))
        .quantity("dL_M", normal(0.0, u_lm))
        .quantity("dL_K", rect(0.0, 0.003))
        .quantity("dL_ia", rect(0.0, 0.0005))
        .quantity("dL_ib", rect(0.0, 0.0005))
        .quantity("dL_0a", normal(0.0, 0.002))
        .quantity("dL_0b", normal(0.0, 0.002))
        .output("L_X", "L_S + dL_S + dL_D + dL_M + dL_K + dL_ib - dL_ia + dL_0b - dL_0a")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &k2()).unwrap();
    close(g.estimate, 30.043, 0.0005, "L_X");
    close(g.combined_standard_uncertainty, 0.0222, 0.00005, "u(L_X) (printed 0.0223)");
    let printed: f64 = g.combined_standard_uncertainty.powi(2) - 0.0003f64.powi(2) + 0.0019f64.powi(2);
    close(printed.sqrt(), 0.0223, 0.00005, "printed total reproduces with 0.0019");
    close(g.expanded_uncertainty, 0.045, 0.0006, "U");
    // S7.13: ν_eff ≈ 3 × 2.5⁴ ≈ 100.
    close(g.effective_dof, 100.0, 20.0, "ν_eff");
}

/// S9: hand-held DMM at 100 V; the resolution term dominates, the output is
/// essentially rectangular and k = 0.95·√3 = 1.65 (S9.14).
#[test]
fn s9_dmm_100_v() {
    let model = Model::builder()
        .quantity("V_iX", Distribution::Constant { value: 100.1 })
        .quantity("V_S", normal(100.0, 0.002 / 2.0))
        .quantity("dV_iX", rect(0.0, 0.05))
        .quantity("dV_S", rect(0.0, 0.011))
        .output("E_X", "V_iX - V_S + dV_iX - dV_S")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &k2()).unwrap();
    close(g.estimate, 0.1, 1e-12, "E_X");
    close(g.budget[2].contribution, 0.029, 0.0005, "resolution");
    close(g.budget[3].contribution.abs(), 0.0064, 0.00005, "calibrator");
    close(g.combined_standard_uncertainty, 0.030, 0.0005, "u(E_X)");
    // Ratio of the non-dominant terms to the dominant one: 0.22 (≤ 0.3).
    let ur = (g.budget[1].contribution.powi(2) + g.budget[3].contribution.powi(2)).sqrt();
    close(ur / g.budget[2].contribution, 0.22, 0.005, "u_R/u_1");
    let k = trapezoidal_coverage_factor(0.95, 1.0);
    close(k, 1.65, 0.005, "k (rectangular)");
    close(k * g.combined_standard_uncertainty, 0.0487, 0.0005, "U");
    let rep = uncertainty_core::report::report_result(
        g.estimate,
        k * g.combined_standard_uncertainty,
        1,
        uncertainty_core::report::Rounding::HalfUp,
        uncertainty_core::report::Rounding::HalfUp,
    );
    assert_eq!((rep.value.as_str(), rep.uncertainty.as_str()), ("0.10", "0.05"));
}

/// S10: vernier calliper at 150 mm (µm); two rectangular terms dominate
/// (±50 and ±25 µm) → trapezoid β = 0.33, k = 1.83, U = 60 µm.
#[test]
fn s10_vernier_calliper() {
    let model = Model::builder()
        .quantity("l_iX", Distribution::Constant { value: 150_100.0 })
        .quantity("l_S", rect(150_000.0, 0.8))
        .quantity("dt", rect(0.0, 2.0))
        .quantity("dl_iX", rect(0.0, 25.0))
        .quantity("dl_M", rect(0.0, 50.0))
        .output("E_X", "l_iX - l_S + 150000*11.5e-6*dt + dl_iX + dl_M")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &k2()).unwrap();
    close(g.estimate, 100.0, 1e-9, "E_X / µm");
    for (c, e) in contributions(&g).iter().skip(1).zip([0.46, 2.0, 14.0, 29.0]) {
        close(*c, e, 0.5, "contribution");
    }
    close(g.combined_standard_uncertainty, 32.0, 0.5, "u(E_X)");
    let beta = (50.0 - 25.0) / (50.0 + 25.0);
    let k = trapezoidal_coverage_factor(0.95, beta);
    close(k, 1.83, 0.005, "k (β = 0.33)");
    close(k * g.combined_standard_uncertainty, 60.0, 1.0, "U ≈ 60 µm");
}

/// S11: temperature block calibrator at 180 °C (mK). Dominant rectangular
/// terms ±250 and ±100 mK → β = 0.43. The guide quotes k = 1.81; Eq. (S10.9)
/// gives 1.80. Reported (180.1 ± 0.3) °C either way.
#[test]
fn s11_temperature_block_calibrator() {
    let model = Model::builder()
        .quantity("t_S", normal(180_100.0, 30.0 / 2.0))
        .quantity("dt_S", normal(0.0, 10.0))
        .quantity("dt_D", rect(0.0, 40.0))
        .quantity("dt_iX", rect(0.0, 50.0))
        .quantity("dt_R", rect(0.0, 100.0))
        .quantity("dt_A", rect(0.0, 250.0))
        .quantity("dt_H", rect(0.0, 50.0))
        .quantity("dt_V", rect(0.0, 30.0))
        .output("t_X", "t_S + dt_S + dt_D - dt_iX + dt_R + dt_A + dt_H + dt_V")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &k2()).unwrap();
    for (c, e) in contributions(&g).iter().zip([15.0, 10.0, 23.0, 29.0, 58.0, 144.0, 29.0, 17.0]) {
        close(*c, e, 0.5, "contribution");
    }
    close(g.combined_standard_uncertainty, 164.0, 0.5, "u(t_X)");
    let beta = (250.0 - 100.0) / (250.0 + 100.0);
    close(beta, 0.43, 0.005, "β");
    let k = trapezoidal_coverage_factor(0.95, beta);
    close(k, 1.80, 0.005, "k (printed 1.81)");
    close(k * g.combined_standard_uncertainty / 1000.0, 0.3, 0.05, "U ≈ 0.3 °C");
}

/// S12: household water meter at 2500 l/h.
#[test]
fn s12_water_meter() {
    let volume = Model::builder()
        .quantity("V_iS", normal(200.0, 0.2 / 2.0))
        .quantity("dV_iS", rect(0.0, 0.02))
        .quantity("beta_S", rect(51e-6, 0.5e-6))
        .quantity("t_S", rect(15.0, 2.0))
        .quantity("gamma_W", rect(0.15e-3, 0.005e-3))
        .quantity("t_X", rect(16.0, 2.0))
        .quantity("kappa_W", rect(0.46e-6, 0.005e-6))
        .quantity("p_X", rect(500.0, 50.0))
        .output("V_X", "(V_iS + dV_iS)*(1 + beta_S*(t_S - 20))*(1 + gamma_W*(t_X - t_S))*(1 - kappa_W*p_X)")
        .build()
        .unwrap();
    let g = gum::evaluate(&volume, &k2()).unwrap();
    close(g.estimate, 199.93, 0.005, "V_X");
    let c = |n: &str| g.budget.iter().find(|r| r.name == n).unwrap();
    close(c("beta_S").sensitivity, -1000.0, 1.0, "c(β_S)");
    close(c("t_S").sensitivity, -0.0198, 0.0001, "c(t_S)");
    close(c("t_X").sensitivity, 0.0300, 0.0001, "c(t_X)");
    close(c("t_X").contribution, 0.0346, 0.0001, "u(y, t_X)");
    close(c("p_X").contribution.abs(), 0.0027, 0.0001, "u(y, p_X)");
    close(g.combined_standard_uncertainty, 0.109, 0.0005, "u(V_X)");
    let u_vx = g.combined_standard_uncertainty;

    // Relative error of a single run.
    let single = Model::builder()
        .quantity("V_iX", Distribution::Constant { value: 200.0 })
        .quantity("dV_iX1", rect(0.0, 0.1))
        .quantity("dV_iX2", rect(0.0, 0.1))
        .quantity("V_X", normal(g.estimate, u_vx))
        .output("e_X", "(V_iX + dV_iX2 - dV_iX1)/V_X - 1")
        .build()
        .unwrap();
    let ge = gum::evaluate(&single, &k2()).unwrap();
    close(ge.estimate, 0.0003, 0.00006, "e_X (0.0003)");
    close(ge.combined_standard_uncertainty, 0.68e-3, 0.005e-3, "u(e_X)");

    // Three runs: mean 0.001, s = 0.001; u = 0.6e-3 with ν = 2; combined
    // 0.91e-3 with ν_eff = 10 → k = 2.28 (Table E.1) → U = 0.002.
    let runs = TypeA::from_observations(&[0.0003, 0.0005, 0.0022]).unwrap();
    close(runs.mean, 0.001, 1e-12, "e_X,av");
    close(runs.std_dev, 0.001, 0.00005, "s");
    let avg = Model::builder()
        .input(InputQuantity::new("de_X", normal(0.0, runs.std_uncertainty)).with_dof(runs.dof))
        .quantity("e_X", normal(runs.mean, ge.combined_standard_uncertainty))
        .output("e_Xav", "e_X + de_X")
        .build()
        .unwrap();
    let ga =
        gum::evaluate(&avg, &GumOptions { coverage: CoverageSpec::Probability(0.9545), ..Default::default() }).unwrap();
    close(ga.combined_standard_uncertainty, 0.91e-3, 0.01e-3, "u(e_X,av)");
    close(ga.effective_dof.floor(), 10.0, 0.0, "ν_eff");
    close(ga.coverage_factor, 2.28, 0.005, "k");
    close(ga.expanded_uncertainty, 0.002, 0.0002, "U ≈ 0.002");
}

/// S13: ring gauge of nominal diameter 90 mm (µm relative to the nominal
/// diameters). The temperature correction is a sum of products of quantities
/// whose expectations are zero (temperature deviations) or not (expansion
/// coefficients); the second-order terms give the exact variance of the
/// products (S4.13, S13.5): 0.018, 0.053, 0.12, 0.066 → 0.15 µm.
///
/// Discrepancies in the printed example: the mean Δl is 49.999 54 mm but the
/// budget uses 49.999 55; s of a single observation is 0.33 µm but the text
/// later says 0.18 µm; √(0.15² + 0.25²) = 0.29 µm (printed 0.30); the
/// coaxiality correction is −0.004 µm (budget +0.000 004 mm); and
/// U = 2 × 0.41 = 0.82 µm is reported as 0.9 µm, i.e. rounded up.
#[test]
fn s13_ring_gauge_90_mm() {
    let obs = [49.99935, 49.99911, 49.99972, 49.99954, 49.99996];
    let ta = TypeA::from_observations(&obs).unwrap();
    close(ta.mean, 49.99954, 0.000005, "Δl̄ / mm");
    close(ta.std_dev * 1000.0, 0.33, 0.005, "s(Δl) / µm");
    close(ta.std_uncertainty * 1000.0, 0.15, 0.005, "s(Δl̄) / µm");
    let u_dl = ((ta.std_uncertainty * 1000.0).powi(2) + 0.25f64.powi(2)).sqrt();
    close(u_dl, 0.29, 0.005, "u(Δl) (printed 0.30)");

    // Temperature sub-budget (µm, K; D in µm).
    let (ds, dx) = (40_000.0, 90_000.0);
    let a = || rect(11.5e-6, 1.0e-6);
    let lt = Model::builder()
        .quantity("a_S", a())
        .quantity("a_X", a())
        .quantity("a_R", a())
        .quantity("dt_A", rect(0.0, 0.5))
        .quantity("dt_S", rect(0.0, 0.2))
        .quantity("dt_X", rect(0.0, 0.2))
        .quantity("dt_R", rect(0.0, 0.2))
        .intermediate("l_TA", &format!("({ds}*(a_S - a_R) - {dx}*(a_X - a_R))*dt_A"))
        .intermediate("l_TS", &format!("{ds}*a_S*dt_S"))
        .intermediate("l_TX", &format!("{dx}*a_X*dt_X"))
        .intermediate("l_TR", &format!("({ds} - {dx})*a_R*dt_R"))
        .output("dl_T", "l_TA + l_TS - l_TX + l_TR")
        .build()
        .unwrap();
    let g = gum::evaluate(&lt, &GumOptions { higher_order: true, ..k2() }).unwrap();
    close(g.combined_standard_uncertainty, 0.148, 0.0005, "u(δl_T) (0.15 µm)");
    // The first-order evaluation misses the l_TA product completely.
    let g1 = gum::evaluate(&lt, &k2()).unwrap();
    assert!(g1.combined_standard_uncertainty < g.combined_standard_uncertainty);

    // Coaxiality (S13.4–S13.5): ±20 µm rectangular Δc.
    let uc = Distribution::rectangular_half_width(0.0, 20.0).gum_standard_uncertainty();
    let dlp = 2.0 * (1.0 / dx - 1.0 / ds) * uc * uc;
    let u_dlp = (16.0 / 5.0 * (1.0 / (dx * dx) + 1.0 / (ds * ds))).sqrt() * uc * uc;
    close(dlp, -0.0037, 0.00005, "δl_P / µm (≈ −0.004)");
    close(u_dlp, 0.0065, 0.00005, "u(δl_P) / µm");

    let model = Model::builder()
        .quantity("d_S", normal(0.7, 0.2 / 2.0))
        .quantity("dl", normal(-0.46, u_dl))
        .quantity("dl_i", rect(0.0, 0.375))
        .quantity("dl_T", normal(0.0, g.combined_standard_uncertainty))
        .quantity("dl_P", normal(dlp, u_dlp))
        .quantity("dl_E", rect(0.0, 0.03))
        .quantity("dl_A", rect(0.0, 0.02))
        .output("d_X", "d_S + dl + dl_i + dl_T + dl_P + dl_E + dl_A")
        .build()
        .unwrap();
    let gx = gum::evaluate(&model, &k2()).unwrap();
    close(gx.estimate, 0.24, 0.005, "d_X − 90 mm / µm (90.000 24 mm)");
    close(gx.combined_standard_uncertainty, 0.41, 0.01, "u(d_X)");
    use uncertainty_core::report::{report_result, Rounding};
    // Reported (90.000 3 ± 0.000 9) mm: consistent only with rounding U up.
    let up = report_result(gx.estimate, gx.expanded_uncertainty, 1, Rounding::Up, Rounding::HalfUp);
    assert_eq!(up.uncertainty, "0.9");
    let nearest = report_result(gx.estimate, gx.expanded_uncertainty, 1, Rounding::HalfUp, Rounding::HalfUp);
    assert_eq!(nearest.uncertainty, "0.8");
}
