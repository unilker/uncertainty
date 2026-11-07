//! Validation against the worked examples of JCGM 101:2008 clause 9.
//!
//! Reference values are transcribed from the tables of JCGM 101:2008
//! (BIPM, https://www.bipm.org/documents/20126/2071204/JCGM_101_2008_E.pdf).
//! MCM reference values are themselves Monte Carlo estimates, so tolerances
//! allow for the stated number of digits plus the sampling dispersion at the
//! number of trials used here. See docs/research/validation-matrix.md.

use uncertainty_core::gum::{self, CoverageSpec, GumOptions};
use uncertainty_core::mcm::{self, IntervalKind, McmOptions, Trials};
use uncertainty_core::{Distribution, InputQuantity, Model};

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol, "{what}: got {actual}, expected {expected} ± {tol}");
}

fn mcm_fixed(model: &Model, m: usize, p: f64, seed: u64) -> mcm::McmResult {
    let opts = McmOptions { trials: Trials::Fixed(m), coverage_probability: p, seed, ..Default::default() };
    mcm::evaluate(model, &opts).unwrap()
}

fn additive(dists: [Distribution; 4]) -> Model {
    let [a, b, c, d] = dists;
    Model::builder()
        .quantity("X1", a)
        .quantity("X2", b)
        .quantity("X3", c)
        .quantity("X4", d)
        .output("Y", "X1 + X2 + X3 + X4")
        .build()
        .unwrap()
}

/// JCGM 101 9.2.2, Table 2: Gaussian inputs.
#[test]
fn additive_model_gaussian_inputs() {
    let n = || Distribution::Normal { mean: 0.0, sd: 1.0 };
    let model = additive([n(), n(), n(), n()]);
    let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g.estimate, 0.0, 1e-15, "GUF y");
    close(g.combined_standard_uncertainty, 2.0, 1e-15, "GUF u");
    close(g.interval.0, -3.92, 0.005, "GUF low");
    close(g.interval.1, 3.92, 0.005, "GUF high");

    let m = mcm_fixed(&model, 1_000_000, 0.95, 1);
    close(m.estimate, 0.0, 0.01, "MCM y");
    close(m.standard_uncertainty, 2.00, 0.005, "MCM u");
    close(m.symmetric_interval.0, -3.92, 0.02, "MCM low");
    close(m.symmetric_interval.1, 3.92, 0.02, "MCM high");
    let v = mcm::validate_gum(&g, &m, 2, IntervalKind::Symmetric);
    assert!(v.validated, "{v:?}");
}

/// JCGM 101 9.2.3, Table 3: rectangular inputs of equal width. The analytic
/// endpoints are ±2√3 [2 − (3/5)^{1/4}] ≈ ±3.88 (JCGM 101 Annex E).
#[test]
fn additive_model_rectangular_inputs() {
    let r = || Distribution::rectangular_half_width(0.0, 3f64.sqrt());
    let model = additive([r(), r(), r(), r()]);
    let analytic = 2.0 * 3f64.sqrt() * (2.0 - 0.6f64.powf(0.25));
    close(analytic, 3.88, 0.005, "analytic endpoint");

    let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g.interval.1, 3.92, 0.005, "GUF high");

    let m = mcm_fixed(&model, 2_000_000, 0.95, 2);
    close(m.standard_uncertainty, 2.00, 0.005, "MCM u");
    close(m.symmetric_interval.0, -analytic, 0.01, "MCM low");
    close(m.symmetric_interval.1, analytic, 0.01, "MCM high");
}

/// JCGM 101 9.2.4, Table 4: X4 has standard deviation 10. GUF is not
/// validated for ndig = 2 (δ = 0.5) but is for ndig = 1 (δ = 5).
#[test]
fn additive_model_dominant_rectangular_input() {
    let r1 = || Distribution::rectangular_half_width(0.0, 3f64.sqrt());
    let model = additive([r1(), r1(), r1(), Distribution::rectangular_half_width(0.0, 10.0 * 3f64.sqrt())]);
    let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g.combined_standard_uncertainty, 10.1, 0.05, "GUF u");
    close(g.interval.1, 19.9, 0.05, "GUF high");

    let m = mcm_fixed(&model, 1_000_000, 0.95, 3);
    close(m.standard_uncertainty, 10.15, 0.06, "MCM u");
    close(m.symmetric_interval.0, -17.0, 0.06, "MCM low");
    close(m.symmetric_interval.1, 17.0, 0.06, "MCM high");
    // Symmetric distribution centred on the estimate: the half-width of the
    // interval symmetric about ỹ equals that of the probabilistically
    // symmetric interval (proposed definition of expanded uncertainty).
    close(m.expanded_about_estimate, 17.0, 0.06, "MCM U about ỹ");
    assert!(!mcm::validate_gum(&g, &m, 2, IntervalKind::Symmetric).validated);
    assert!(mcm::validate_gum(&g, &m, 1, IntervalKind::Symmetric).validated);
}

fn mass_model() -> Model {
    Model::builder()
        .quantity("m_Rc", Distribution::Normal { mean: 100_000.000, sd: 0.050 })
        .quantity("dm_Rc", Distribution::Normal { mean: 1.234, sd: 0.020 })
        .quantity("rho_a", Distribution::rectangular_half_width(1.20, 0.10))
        .quantity("rho_W", Distribution::rectangular_half_width(8e3, 1e3))
        .quantity("rho_R", Distribution::rectangular_half_width(8.00e3, 0.05e3))
        .output("dm", "(m_Rc + dm_Rc) * (1 + (rho_a - 1.2) * (1/rho_W - 1/rho_R)) - 100000")
        .build()
        .unwrap()
}

/// JCGM 101 9.3, Tables 6 and 7: mass calibration (values in mg).
#[test]
fn mass_calibration() {
    let model = mass_model();
    let g1 = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g1.estimate, 1.2340, 0.00005, "GUF1 estimate");
    close(g1.combined_standard_uncertainty, 0.0539, 0.00005, "GUF1 u");
    close(g1.interval.0, 1.1285, 0.00005, "GUF1 low");
    close(g1.interval.1, 1.3395, 0.00005, "GUF1 high");
    // Table 7: sensitivity coefficients 1, 1, 0, 0, 0.
    let c: Vec<f64> = g1.budget.iter().map(|r| r.sensitivity).collect();
    close(c[0], 1.0, 1e-12, "c(m_Rc)");
    close(c[1], 1.0, 1e-12, "c(dm_Rc)");
    for (i, ci) in c.iter().enumerate().skip(2) {
        close(*ci, 0.0, 1e-12, &format!("c[{i}]"));
    }

    let g2 = gum::evaluate(&model, &GumOptions { higher_order: true, ..Default::default() }).unwrap();
    close(g2.combined_standard_uncertainty, 0.0750, 0.00005, "GUF2 u");
    close(g2.interval.0, 1.0870, 0.0001, "GUF2 low");
    close(g2.interval.1, 1.3810, 0.0001, "GUF2 high");

    // MCM. Primary reference: independent 2 × 10⁸-trial PCG64 run
    // (validation/reference_values.json): y = 1.233 998, u = 0.075 484,
    // symmetric 95 % interval [1.084 43, 1.383 59], shortest [1.084 37, 1.383 53].
    // The output PDF is exactly symmetric about 1.2340 (the non-linear term
    // is a product with a zero-mean symmetric factor), so both intervals
    // coincide in the limit. Tolerances are ≈5 standard deviations of the
    // engine's Monte Carlo dispersion at M = 2 × 10⁶ (examples/mcm_dispersion.rs):
    // 4e-5 (y), 4e-5 (u), 1.1e-4 (symmetric endpoints), 7.4e-4 (shortest
    // endpoints — poorly located because the length function is flat).
    let m = mcm_fixed(&model, 2_000_000, 0.95, 4);
    close(m.estimate, 1.233_998, 0.0002, "MCM estimate");
    close(m.standard_uncertainty, 0.075_484, 0.0002, "MCM u");
    close(m.symmetric_interval.0, 1.084_43, 0.0006, "MCM symmetric low");
    close(m.symmetric_interval.1, 1.383_59, 0.0006, "MCM symmetric high");
    close(m.shortest_interval.0, 1.084_37, 0.0037, "MCM shortest low");
    close(m.shortest_interval.1, 1.383_53, 0.0037, "MCM shortest high");
    // JCGM 101 Table 6 (M = 0.72 × 10⁶; its endpoints carry dispersion of
    // the same order, e.g. a midpoint of 1.2330 for a PDF symmetric about 1.2340).
    close(m.estimate, 1.2341, 0.0002, "Table 6 estimate");
    close(m.standard_uncertainty, 0.0754, 0.0003, "Table 6 u");
    close(m.shortest_interval.0, 1.0834, 0.006, "Table 6 low");
    close(m.shortest_interval.1, 1.3825, 0.006, "Table 6 high");

    // 9.3.2.6: ndig = 1 → δ = 0.005; GUF1 not validated, GUF2 validated.
    // The comparison uses the symmetric interval, which equals the shortest
    // one for this symmetric PDF but is far less dispersed. Reference
    // d_low = |1.128 45 − 1.084 43| = 0.0440, d_high = 0.0440 (Table 6:
    // 0.0451, 0.0430 from its noisier shortest interval).
    let v1 = mcm::validate_gum(&g1, &m, 1, IntervalKind::Symmetric);
    let v2 = mcm::validate_gum(&g2, &m, 1, IntervalKind::Symmetric);
    close(v1.tolerance, 0.005, 1e-15, "δ");
    assert!(!v1.validated && v2.validated, "{v1:?} {v2:?}");
    close(v1.d_low, 0.0440, 0.0007, "GUF1 d_low");
    close(v1.d_high, 0.0440, 0.0007, "GUF1 d_high");
    close(v1.d_low, 0.0451, 0.0015, "Table 6 GUF1 d_low");
    close(v1.d_high, 0.0430, 0.0015, "Table 6 GUF1 d_high");
}

/// JCGM 101 9.3 with the adaptive procedure (δ/5, ndig = 1).
#[test]
fn mass_calibration_adaptive() {
    let model = mass_model();
    let opts = McmOptions {
        trials: Trials::Adaptive { ndig: 1, tolerance_divisor: 5.0, max_trials: 20_000_000 },
        seed: 5,
        ..Default::default()
    };
    let m = mcm::evaluate(&model, &opts).unwrap();
    assert!(m.converged);
    // JCGM 101 reports 0.72 × 10⁶ trials for one run of the procedure.
    assert!(m.trials > 100_000 && m.trials < 5_000_000, "M = {}", m.trials);
    close(m.standard_uncertainty, 0.075_484, 0.0005, "MCM u");
    // Stabilized to 2s ≤ δ/5 = 0.001 for the endpoints (JCGM 101 7.9.4 k).
    close(m.shortest_interval.0, 1.084_37, 0.002, "MCM low");
    close(m.shortest_interval.1, 1.383_53, 0.002, "MCM high");
}

fn comparison_loss(x1: f64, r: f64) -> Model {
    let mut b = Model::builder()
        .quantity("X1", Distribution::Normal { mean: x1, sd: 0.005 })
        .quantity("X2", Distribution::Normal { mean: 0.0, sd: 0.005 })
        .output("dY", "X1^2 + X2^2");
    if r != 0.0 {
        b = b.correlation("X1", "X2", r);
    }
    b.build().unwrap()
}

/// Reference and tolerance data for one comparison-loss case (units 10⁻⁶).
struct LossCase {
    x1: f64,
    r: f64,
    /// Reference [mean, sd, shortest lo, shortest hi, symmetric lo, symmetric hi].
    reference: [f64; 6],
    /// Tolerances ≈ 5 × dispersion at M = 2 × 10⁶ (examples/mcm_dispersion.rs).
    tol: [f64; 6],
    /// JCGM 101 table: [estimate, u, shortest lo, shortest hi].
    table: [f64; 4],
}

fn check_loss_case(c: &LossCase, seed: u64) {
    let model = comparison_loss(c.x1, c.r);
    let m = mcm_fixed(&model, 2_000_000, 0.95, seed);
    let got = [
        m.estimate,
        m.standard_uncertainty,
        m.shortest_interval.0,
        m.shortest_interval.1,
        m.symmetric_interval.0,
        m.symmetric_interval.1,
    ]
    .map(|v| v * 1e6);
    let names = ["estimate", "u", "shortest low", "shortest high", "symmetric low", "symmetric high"];
    for k in 0..6 {
        close(got[k], c.reference[k], c.tol[k], &format!("x1={} r={}: {}", c.x1, c.r, names[k]));
    }
    // Table values are rounded and come from M = 10⁶ (√2 more dispersion).
    for k in 0..4 {
        close(got[k], c.table[k], 2.0 * c.tol[k] + 0.5, &format!("x1={} r={}: table {}", c.x1, c.r, names[k]));
    }
}

/// JCGM 101 9.4.2, Table 8: comparison loss δY = X₁² + X₂², uncorrelated.
/// Primary reference: the analytic solution δY/u² ~ noncentral χ²(2, x₁²/u²)
/// (JCGM 101 Annex F; validation/reference_values.py).
#[test]
fn comparison_loss_uncorrelated() {
    // GUF first-order and higher-order standard uncertainties of Table 8.
    for (x1, g1u, g2u) in [(0.000, 0.0, 50.0), (0.010, 100.0, 112.0), (0.050, 500.0, 502.0)] {
        let model = comparison_loss(x1, 0.0);
        let g1 = gum::evaluate(&model, &GumOptions::default()).unwrap();
        let g2 = gum::evaluate(&model, &GumOptions { higher_order: true, ..Default::default() }).unwrap();
        close(g1.combined_standard_uncertainty * 1e6, g1u, 0.5, "G1 u");
        close(g2.combined_standard_uncertainty * 1e6, g2u, 0.5, "G2 u");
    }
    let cases = [
        LossCase {
            x1: 0.000,
            r: 0.0,
            reference: [50.0, 50.0, 0.0, 149.787, 1.265_89, 184.444],
            tol: [0.2, 0.25, 0.001, 0.9, 0.03, 1.2],
            table: [50.0, 50.0, 0.0, 150.0],
        },
        LossCase {
            x1: 0.010,
            r: 0.0,
            reference: [150.0, 111.803, 0.0, 366.005, 8.546_84, 427.123],
            tol: [0.5, 0.4, 0.003, 1.5, 0.15, 1.5],
            table: [150.0, 112.0, 0.0, 367.0],
        },
        LossCase {
            x1: 0.050,
            r: 0.0,
            reference: [2550.0, 502.494, 1593.570, 3548.570, 1638.477, 3603.358],
            tol: [2.5, 1.0, 16.0, 17.0, 4.0, 4.3],
            table: [2551.0, 502.0, 1590.0, 3543.0],
        },
    ];
    for (i, c) in cases.iter().enumerate() {
        check_loss_case(c, 10 + i as u64);
    }
}

/// JCGM 101 9.4.3, Table 9: comparison loss with r(x1, x2) = 0.9.
/// Primary reference: independent 2 × 10⁸-trial PCG64 run.
#[test]
fn comparison_loss_correlated() {
    let cases = [
        LossCase {
            x1: 0.000,
            r: 0.9,
            reference: [50.006, 67.278, 0.0, 185.095, 0.560_93, 241.246],
            tol: [0.2, 0.45, 0.001, 1.1, 0.015, 1.6],
            table: [50.0, 67.0, 0.0, 185.0],
        },
        LossCase {
            x1: 0.010,
            r: 0.9,
            reference: [150.021, 120.531, 12.630, 397.486, 29.0807, 478.366],
            tol: [0.5, 0.6, 1.7, 2.5, 0.15, 2.0],
            table: [150.0, 121.0, 13.0, 398.0],
        },
        LossCase {
            x1: 0.050,
            r: 0.9,
            reference: [2550.08, 504.542, 1624.033, 3554.431, 1696.837, 3659.923],
            tol: [2.5, 1.2, 25.0, 25.0, 4.0, 5.0],
            table: [2551.0, 504.0, 1628.0, 3555.0],
        },
    ];
    for (i, c) in cases.iter().enumerate() {
        let model = comparison_loss(c.x1, c.r);
        let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
        // First-order GUF is unaffected by the correlation because c2 = 0.
        close(g.combined_standard_uncertainty * 1e6, [0.0, 100.0, 500.0][i], 0.5, "GUF u");
        // Higher-order GUF with correlated normal inputs [Mana & Pennecchi 2007]
        // gives the analytic expectation and standard deviation of Table 16.2 of
        // JCGM GUM-5:2026: 50/150/2550 and 2σ²√(1 + r²) = 67.3, 120.5, 504.5 (×10⁻⁶).
        let g2 = gum::evaluate(&model, &GumOptions { higher_order: true, ..Default::default() }).unwrap();
        let s2 = 0.005f64.powi(2);
        let analytic_u = (4.0 * c.x1 * c.x1 * s2 + 4.0 * s2 * s2 * (1.0 + c.r * c.r)).sqrt();
        close(g2.estimate * 1e6, c.x1 * c.x1 * 1e6 + 50.0, 1e-9, "GUF2 estimate (Amd.1)");
        close(g2.combined_standard_uncertainty, analytic_u, 1e-15, "GUF2 u, correlated");
        close(g2.combined_standard_uncertainty * 1e6, [67.0, 121.0, 505.0][i], 0.6, "GUM-5 Table 16.2, column A");
        check_loss_case(c, 30 + i as u64);
    }
}

/// JCGM 101 9.5, Tables 10 and 11: gauge block calibration, approximate model
/// (37) and non-linear model (36). Lengths in nm.
#[test]
fn gauge_block_calibration() {
    let build = |expr: &str| {
        Model::builder()
            .quantity("Ls", Distribution::StudentT { mean: 50_000_623.0, scale: 25.0, dof: 18.0 })
            .quantity("D", Distribution::StudentT { mean: 215.0, scale: 6.0, dof: 24.0 })
            .quantity("d1", Distribution::StudentT { mean: 0.0, scale: 4.0, dof: 5.0 })
            .quantity("d2", Distribution::StudentT { mean: 0.0, scale: 7.0, dof: 8.0 })
            .quantity("alpha_s", Distribution::Rectangular { lower: 9.5e-6, upper: 13.5e-6 })
            .quantity("theta0", Distribution::Normal { mean: -0.1, sd: 0.2 })
            .quantity("Delta", Distribution::Arcsine { lower: -0.5, upper: 0.5 })
            // GUM H.1.6: δα reliable to 10 % (ν = 50), δθ to 50 % (ν = 2).
            .input(
                InputQuantity::new(
                    "d_alpha",
                    Distribution::CurvilinearTrapezoid { lower: -1.0e-6, upper: 1.0e-6, d: 0.1e-6 },
                )
                .with_dof(50.0),
            )
            .input(
                InputQuantity::new(
                    "d_theta",
                    Distribution::CurvilinearTrapezoid { lower: -0.050, upper: 0.050, d: 0.025 },
                )
                .with_dof(2.0),
            )
            .output("dL", expr)
            .build()
            .unwrap()
    };
    let approx = build("Ls + D + d1 + d2 - Ls*(d_alpha*(theta0 + Delta) + alpha_s*d_theta) - 50000000");
    let exact = build(
        "(Ls*(1 + alpha_s*(theta0 + Delta - d_theta)) + D + d1 + d2) / (1 + (alpha_s + d_alpha)*(theta0 + Delta)) - 50000000",
    );
    let opts = GumOptions { coverage: CoverageSpec::Probability(0.99), ..Default::default() };
    for model in [&approx, &exact] {
        let g = gum::evaluate(model, &opts).unwrap();
        close(g.estimate, 838.0, 0.5, "GUF estimate");
        close(g.combined_standard_uncertainty, 32.0, 0.5, "GUF u");
        close(g.effective_dof.floor(), 16.0, 0.0, "ν_eff");
        close(g.coverage_factor, 2.92, 0.005, "k99");
        // Table 11: [745, 931]; the difference of ≤ 1 nm comes from the
        // table rounding u(y) to 32 nm.
        close(g.interval.0, 745.0, 1.5, "GUF low");
        close(g.interval.1, 931.0, 1.5, "GUF high");

        let m = mcm_fixed(model, 1_000_000, 0.99, 40);
        close(m.estimate, 838.0, 0.5, "MCM estimate");
        close(m.standard_uncertainty, 36.0, 0.5, "MCM u");
        close(m.shortest_interval.0, 745.0, 1.5, "MCM low");
        close(m.shortest_interval.1, 932.0, 1.5, "MCM high");
    }
}

/// MCM refuses correlated non-Gaussian inputs instead of silently ignoring
/// the correlation.
#[test]
fn correlated_non_gaussian_inputs_are_refused() {
    let model = Model::builder()
        .quantity("a", Distribution::Rectangular { lower: 0.0, upper: 1.0 })
        .quantity("b", Distribution::Normal { mean: 0.0, sd: 1.0 })
        .correlation("a", "b", 0.5)
        .output("y", "a + b")
        .build()
        .unwrap();
    assert!(mcm::evaluate(&model, &McmOptions::default()).is_err());
}

/// Same seed ⇒ identical results (reproducibility of published numbers).
#[test]
fn results_are_reproducible_for_a_seed() {
    let model = mass_model();
    let a = mcm_fixed(&model, 100_000, 0.95, 77);
    let b = mcm_fixed(&model, 100_000, 0.95, 77);
    assert_eq!(a, b);
    let c = mcm_fixed(&model, 100_000, 0.95, 78);
    assert_ne!(a.estimate, c.estimate);
}

/// Half-width of the coverage interval symmetric about the estimate
/// (expanded uncertainty as proposed by JCGM-WG1/WG2, December 2025): for a
/// skewed output, Y = X² with X ~ N(1, 0.5²), it differs from both the
/// probabilistically symmetric and the shortest interval. Exact values by
/// quadrature: E(Y) = 1.25, Pr(|Y − 1.25| ≤ U) = 0.95 at U = 2.0712 (scipy brentq).
#[test]
fn expanded_uncertainty_about_the_estimate() {
    let model =
        Model::builder().quantity("X", Distribution::Normal { mean: 1.0, sd: 0.5 }).output("Y", "X^2").build().unwrap();
    let m = mcm_fixed(&model, 2_000_000, 0.95, 7);
    close(m.estimate, 1.25, 0.002, "E(Y)");
    close(m.expanded_about_estimate, 2.0712, 0.004, "U about ỹ");
    let half_sym = 0.5 * (m.symmetric_interval.1 - m.symmetric_interval.0);
    assert!((half_sym - m.expanded_about_estimate).abs() > 0.05, "{half_sym}");
}
