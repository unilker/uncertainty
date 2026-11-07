//! Validation against JCGM 100:2008 (GUM) Annex H.1 — end-gauge calibration.
//!
//! Reference values from JCGM 100:2008 H.1.3–H.1.7 and Table H.1
//! (https://www.bipm.org/documents/20126/2071204/JCGM_100_2008_E.pdf).
//! Lengths are in nm, temperatures in °C.

use uncertainty_core::gum::{self, CoverageSpec, GumOptions};
use uncertainty_core::{Distribution, InputQuantity, Model};

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol, "{what}: got {actual}, expected {expected} ± {tol}");
}

/// Input quantity with a Normal distribution of standard uncertainty u and
/// the degrees of freedom stated in H.1.6.
fn q(name: &str, x: f64, u: f64, dof: Option<f64>) -> InputQuantity {
    let q = InputQuantity::new(name, Distribution::Normal { mean: x, sd: u });
    match dof {
        Some(nu) => q.with_dof(nu),
        None => q,
    }
}

/// Sub-model d = D̄ + d1 + d2 (H.1.3.2, H.1.6 item 2): u(d) = 9.7 nm,
/// ν_eff(d) = 25.6.
#[test]
fn h1_difference_sub_model() {
    let model = Model::builder()
        .input(q("Dbar", 215.0, 5.8, Some(24.0)))
        .input(q("d1", 0.0, 3.9, Some(5.0)))
        .input(q("d2", 0.0, 6.7, Some(8.0)))
        .output("d", "Dbar + d1 + d2")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g.combined_standard_uncertainty, 9.7, 0.05, "u(d)");
    // The GUM obtains 25.6 after rounding u(d) to 9.7 nm; unrounded, 25.4.
    close(g.effective_dof, 25.45, 0.05, "ν_eff(d)");
}

/// Main model l = l_s + d − l_s(δα θ + α_s δθ) (H.3). The inputs are given
/// the distributions the GUM text describes, so that their standard
/// uncertainties are the unrounded values the GUM itself uses in H.6 and
/// H.1.7 (e.g. u(δθ) = 0.05/√3 = 0.028 87 °C, shown as 0.029 in Table H.1):
///
/// * α_s: 11.5 × 10⁻⁶ ± 2 × 10⁻⁶ °C⁻¹, rectangular (H.1.3.3);
/// * θ = θ̄ + Δ: θ̄ = −0.1 °C with u = 0.2 °C, Δ cyclic ±0.5 °C, arc sine
///   (H.1.3.4);
/// * δα: ±1 × 10⁻⁶ °C⁻¹ rectangular, ν = 50 (H.1.3.5, H.1.6);
/// * δθ: ±0.05 °C rectangular, ν = 2 (H.1.3.6, H.1.6).
///
/// Expected: contributions 25, 9.7, 0, 0, 2.9, 16.6 nm (Table H.1);
/// u_c = 32 nm (H.6c); ν_eff = 16.7 → 16, k₉₉ = 2.92, U₉₉ = 93 nm (H.1.6);
/// with second-order terms 11.7 nm and 1.7 nm, u_c = 34 nm (H.1.7).
#[test]
fn h1_end_gauge() {
    let model = Model::builder()
        .input(q("l_s", 50_000_623.0, 25.0, Some(18.0)))
        .input(q("d", 215.0, 9.7, Some(25.6)))
        .quantity("alpha_s", Distribution::rectangular_half_width(11.5e-6, 2e-6))
        .quantity("theta_bar", Distribution::Normal { mean: -0.1, sd: 0.2 })
        .quantity("Delta", Distribution::arcsine_half_width(0.0, 0.5))
        .input(InputQuantity::new("d_alpha", Distribution::rectangular_half_width(0.0, 1e-6)).with_dof(50.0))
        .input(InputQuantity::new("d_theta", Distribution::rectangular_half_width(0.0, 0.05)).with_dof(2.0))
        .intermediate("theta", "theta_bar + Delta")
        .output("l", "l_s + d - l_s*(d_alpha*theta + alpha_s*d_theta)")
        .build()
        .unwrap();
    let opts = GumOptions { coverage: CoverageSpec::Probability(0.99), ..Default::default() };
    let g = gum::evaluate(&model, &opts).unwrap();
    close(g.estimate, 50_000_838.0, 1e-6, "l");

    // Table H.1 standard uncertainties and contributions |c_i| u(x_i).
    let u: Vec<f64> = g.budget.iter().map(|r| r.standard_uncertainty).collect();
    close(u[2], 1.2e-6, 0.05e-6, "u(α_s)");
    close((u[3].powi(2) + u[4].powi(2)).sqrt(), 0.41, 0.005, "u(θ)");
    close(u[5], 0.58e-6, 0.005e-6, "u(δα)");
    close(u[6], 0.029, 0.0005, "u(δθ)");
    let contrib: Vec<f64> = g.budget.iter().map(|r| r.contribution.abs()).collect();
    close(contrib[0], 25.0, 1e-9, "l_s");
    close(contrib[1], 9.7, 1e-9, "d");
    for c in &contrib[2..=4] {
        close(*c, 0.0, 1e-9, "zero sensitivity (α_s, θ̄, Δ)");
    }
    close(contrib[5], 2.9, 0.05, "δα");
    close(contrib[6], 16.6, 0.05, "δθ");
    // Sensitivity coefficients: c(δα) = −l_s θ, c(δθ) = −l_s α_s.
    close(g.budget[5].sensitivity, 50_000_623.0 * 0.1, 1e-3, "c(δα)");
    close(g.budget[6].sensitivity, -50_000_623.0 * 11.5e-6, 1e-6, "c(δθ)");

    close(g.combined_standard_uncertainty, 31.7, 0.05, "u_c (32 nm)");
    close(g.effective_dof, 16.7, 0.1, "ν_eff");
    close(g.coverage_factor, 2.92, 0.005, "k99");
    // The GUM multiplies the rounded u_c = 32 nm: 2.92 × 32 = 93 nm.
    close(g.expanded_uncertainty, 2.92 * 31.67, 0.2, "U99 = k u_c");
    close(g.expanded_uncertainty, 93.0, 0.6, "U99 (93 nm)");
    close(g.expanded_uncertainty / g.estimate, 1.9e-6, 0.06e-6, "U/l (1.9 × 10⁻⁶)");

    let g2 = gum::evaluate(&model, &GumOptions { higher_order: true, ..opts }).unwrap();
    close(g2.combined_standard_uncertainty, 34.0, 0.5, "u_c with second-order terms");
    let t1 = 50_000_623.0 * u[5] * (u[3].powi(2) + u[4].powi(2)).sqrt();
    let t2 = 50_000_623.0 * u[2] * u[6];
    close(t1, 11.7, 0.05, "l_s u(δα) u(θ)");
    close(t2, 1.7, 0.05, "l_s u(α_s) u(δθ)");
    close(g2.higher_order_variance, t1 * t1 + t2 * t2, 1e-6, "second-order variance");
    // JCGM 100:2008/Amd.1:2026, H.1.7: "the first expression given in note 1 to
    // 4.1.4 is equal to zero, so that the estimate l is unaffected".
    close(g2.second_order_estimate_term, 0.0, 1e-12, "second-order term of the estimate (H.1)");
    close(g2.estimate, g.estimate, 1e-9, "estimate unchanged by Amd.1");
    // Non-Gaussian inputs: the engine warns that the 5.1.2 note assumes normality.
    assert_eq!(g2.warnings, vec![gum::GumWarning::HigherOrderNonGaussian]);

    // The first-order evaluation warns that α_s, θ̄ and Δ have zero sensitivity
    // but contribute in second order: (11.7² + 1.7²)/(31.7² + 11.7² + 1.7²) ≈ 12 %.
    let missed = t1 * t1 + t2 * t2;
    let expected_share = missed / (g.combined_standard_uncertainty.powi(2) + missed);
    match g.warnings.as_slice() {
        [gum::GumWarning::ZeroSensitivity { inputs, missing_share }] => {
            assert_eq!(inputs, &["alpha_s", "theta_bar", "Delta"]);
            close(*missing_share, expected_share, 1e-9, "missed variance share");
            close(*missing_share, 0.122, 0.001, "≈ 12 %");
        }
        other => panic!("expected one zero-sensitivity warning, got {other:?}"),
    }
}

/// GUM 4.3.7 / 4.3.9: Type B standard uncertainties from limits.
#[test]
fn type_b_from_limits() {
    let rect = Distribution::rectangular_half_width(0.0, 1.0);
    let tri = Distribution::triangular_half_width(0.0, 1.0);
    close(rect.gum_standard_uncertainty(), 1.0 / 3f64.sqrt(), 1e-15, "rectangular");
    close(tri.gum_standard_uncertainty(), 1.0 / 6f64.sqrt(), 1e-15, "triangular");
    // Trapezoid with β = 0 is triangular, β = 1 rectangular.
    let t0 = Distribution::Trapezoidal { lower: -1.0, upper: 1.0, beta: 0.0 };
    let t1 = Distribution::Trapezoidal { lower: -1.0, upper: 1.0, beta: 1.0 };
    close(t0.gum_standard_uncertainty(), tri.gum_standard_uncertainty(), 1e-15, "β = 0");
    close(t1.gum_standard_uncertainty(), rect.gum_standard_uncertainty(), 1e-15, "β = 1");
}

/// Fixed coverage factor k = 2 (GUM 6.3.3; EA-4/02 practice).
#[test]
fn fixed_coverage_factor() {
    let model = Model::builder()
        .quantity("a", Distribution::Normal { mean: 1.0, sd: 0.3 })
        .quantity("b", Distribution::Normal { mean: 2.0, sd: 0.4 })
        .output("y", "a + b")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions { coverage: CoverageSpec::Factor(2.0), ..Default::default() }).unwrap();
    close(g.combined_standard_uncertainty, 0.5, 1e-15, "u");
    close(g.expanded_uncertainty, 1.0, 1e-15, "U");
    close(g.coverage_probability, 0.9545, 0.0001, "p for k = 2");
}

/// Correlated inputs [GUM 5.2.2, equation (13)]: y = a + b with r = 1 gives
/// u = u_a + u_b; y = a − b with r = 1 gives |u_a − u_b|.
#[test]
fn correlation_terms() {
    let build = |expr: &str, r: f64| {
        Model::builder()
            .quantity("a", Distribution::Normal { mean: 1.0, sd: 0.3 })
            .quantity("b", Distribution::Normal { mean: 2.0, sd: 0.4 })
            .correlation("a", "b", r)
            .output("y", expr)
            .build()
            .unwrap()
    };
    let g = gum::evaluate(&build("a + b", 1.0), &GumOptions::default()).unwrap();
    close(g.combined_standard_uncertainty, 0.7, 1e-12, "sum, r = 1");
    let g = gum::evaluate(&build("a - b", 1.0), &GumOptions::default()).unwrap();
    close(g.combined_standard_uncertainty, 0.1, 1e-12, "difference, r = 1");
    let g = gum::evaluate(&build("a - b", -1.0), &GumOptions::default()).unwrap();
    close(g.combined_standard_uncertainty, 0.7, 1e-12, "difference, r = −1");
}

/// No warning for a linear model, nor for an input whose sensitivity is zero
/// because the model does not depend on it at all.
#[test]
fn zero_sensitivity_warning_only_when_second_order_matters() {
    let model = Model::builder()
        .input(q("a", 1.0, 0.1, None))
        .input(q("b", 0.0, 0.1, None))
        .output("y", "2*a + 0*b")
        .build()
        .unwrap();
    assert!(gum::evaluate(&model, &GumOptions::default()).unwrap().warnings.is_empty());
    // y = a·b at b = 0: c(a) = 0 but the product contributes u(a)u(b), here
    // as much as the first-order term (share 50 %); below 1 % no warning.
    let model = Model::builder()
        .input(q("a", 1.0, 1.0, None))
        .input(q("b", 0.0, 0.1, None))
        .output("y", "a*b")
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
    assert!(
        matches!(&g.warnings[..], [gum::GumWarning::ZeroSensitivity { inputs, missing_share }]
            if inputs == &["a"] && (missing_share - 0.5).abs() < 1e-12),
        "{:?}",
        g.warnings
    );
    let g2 = gum::evaluate(&model, &GumOptions { higher_order: true, ..Default::default() }).unwrap();
    assert!(g2.warnings.is_empty());
}

#[test]
fn second_order_term_of_the_estimate_amd1_2026() {
    // Note 1 to 4.1.4 (Amd.1:2026): y = f(x) + ½Σ ∂²f/∂xᵢ² u²(xᵢ) for
    // independent normal inputs. Comparison loss δY = X₁² + X₂² at x = 0,
    // u = 0.005 (JCGM 101 9.4): the term is u₁² + u₂² = 5·10⁻⁵, the analytic mean.
    let model = Model::builder()
        .input(q("x1", 0.0, 0.005, None))
        .input(q("x2", 0.0, 0.005, None))
        .output("dY", "x1^2 + x2^2")
        .build()
        .unwrap();
    let g1 = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g1.estimate, 0.0, 1e-15, "first order: f(x)");
    close(g1.second_order_estimate_term, 5e-5, 1e-15, "term evaluated without higher order");
    let g2 = gum::evaluate(&model, &GumOptions { higher_order: true, ..Default::default() }).unwrap();
    close(g2.estimate, 5e-5, 1e-15, "estimate with the second-order term");
    close(g2.combined_standard_uncertainty, 5e-5, 1e-15, "u_c with the 5.1.2 note");
    assert!((g2.interval.0 + g2.interval.1) / 2.0 - 5e-5 < 1e-15, "interval centred on the corrected estimate");

    // Correlated inputs (GUM H.10): y = a·b with r(a, b) = 0.5 gives
    // ½(∂²f/∂a∂b + ∂²f/∂b∂a) r u(a) u(b) = r u(a) u(b); the warning appears
    // when the term exceeds u_c/10 and higher-order terms are off.
    let model = Model::builder()
        .input(q("a", 0.0, 1.0, None))
        .input(q("b", 0.0, 1.0, None))
        .output("y", "a*b + 0.1*a")
        .correlation("a", "b", 0.5)
        .build()
        .unwrap();
    let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
    close(g.second_order_estimate_term, 0.5, 1e-12, "r u(a) u(b)");
    close(g.estimate, 0.0, 1e-15, "f(x) without higher-order terms");
    assert!(
        g.warnings
            .iter()
            .any(|w| matches!(w, gum::GumWarning::NonlinearEstimate { ratio, .. } if (ratio - 5.0).abs() < 1e-9)),
        "{:?}",
        g.warnings
    );
}

/// JCGM GUM-5:2026 Example 2 (benzo[a]pyrene, internal standard):
/// m_E = f·A_E/A_ISE·m_ISE with r(f, m_ISE) = −0.54 and r(A_E, A_ISE) = −0.48
/// (Table 2.1). GUM-5 prints u = 0.002 ng, u² = 4.1·10⁻⁶ ng² and
/// ν_eff = 5.2 (G.2b, correlations ignored). A_E and A_ISE are means of the
/// same three paired injections, so they form an ensemble with ν = 2
/// [Willink 2007, §4.1.1]: their joint variance counts as one component and
/// ν_eff ≈ 2.1, not the 4.8 that GUM-5 obtains from its eq. (2.3).
#[test]
fn gum5_example2_benzo_a_pyrene_ensemble() {
    let build = |ensemble: bool| {
        let mut b = Model::builder()
            .input(q("f", 0.616, 0.017, Some(9.0)))
            .input(q("mISE", 0.2455, 0.0036, None))
            .input(q("AE", 85114.0, 9564.0, Some(2.0)))
            .input(q("AISE", 917546.0, 44492.0, Some(2.0)))
            .correlation("f", "mISE", -3.3e-5 / (0.017 * 0.0036))
            .correlation("AE", "AISE", -2.03e8 / (9564.0 * 44492.0))
            .output("mE", "f*AE/AISE*mISE");
        if ensemble {
            b = b.ensemble(&["AE", "AISE"]);
        }
        b.build().unwrap()
    };
    let opts = GumOptions { dof_rounding: gum::DofRounding::Exact, ..GumOptions::default() };
    let plain = gum::evaluate(&build(false), &opts).unwrap();
    close(plain.estimate, 0.014028, 5e-6, "m_E");
    close(plain.combined_standard_uncertainty.powi(2), 4.08e-6, 0.01e-6, "u²(m_E)");
    close(plain.effective_dof, 5.20, 0.01, "ν_eff (G.2b)");
    assert!(plain.warnings.iter().any(|w| matches!(w, gum::GumWarning::WelchSatterthwaiteCorrelated)));

    let ens = gum::evaluate(&build(true), &opts).unwrap();
    assert_eq!(ens.combined_standard_uncertainty, plain.combined_standard_uncertainty);
    // Ensemble component: (c_E u_E)² + (c_I u_I)² + 2 c_E c_I u(A_E, A_ISE) = 3.97e-6 ng², ν = 2.
    close(ens.effective_dof, 2.10, 0.01, "ν_eff (ensemble)");
    // f–m_ISE (ν = 9 and ∞) is still a correlation outside an ensemble.
    assert!(ens.warnings.iter().any(|w| matches!(w, gum::GumWarning::WelchSatterthwaiteCorrelated)));
}

/// A single ensemble gives exactly its own ν whatever the correlation, as
/// the Wishart distribution of the sample covariance matrix requires
/// (cᵀSc ~ cᵀΣc·χ²_ν/ν), and no warning is raised.
#[test]
fn ensemble_of_paired_means_keeps_its_dof() {
    for r in [-0.9, -0.3, 0.0, 0.5, 0.95] {
        let model = Model::builder()
            .input(q("a", 10.0, 0.4, Some(4.0)))
            .input(q("b", 5.0, 0.3, Some(4.0)))
            .correlation("a", "b", r)
            .ensemble(&["a", "b"])
            .output("y", "a/b")
            .build()
            .unwrap();
        let g = gum::evaluate(&model, &GumOptions::default()).unwrap();
        close(g.effective_dof, 4.0, 1e-9, "ν_eff");
        assert!(g.warnings.is_empty(), "r = {r}: {:?}", g.warnings);
    }
    let err = Model::builder()
        .input(q("a", 10.0, 0.4, Some(4.0)))
        .input(q("b", 5.0, 0.3, Some(5.0)))
        .ensemble(&["a", "b"])
        .output("y", "a/b")
        .build();
    assert!(err.is_err());
}
