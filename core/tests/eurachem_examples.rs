//! Validation against EURACHEM/CITAC Guide CG4, *Quantifying Uncertainty in
//! Analytical Measurement*, 3rd edition (QUAM:2012.P1).
//! https://www.eurachem.org/images/stories/Guides/pdf/QUAM2012_P1.pdf

use uncertainty_core::gum::{self, CoverageSpec, GumOptions};
use uncertainty_core::mcm::{self, IntervalKind, McmOptions, Trials};
use uncertainty_core::{Distribution, Model};

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
