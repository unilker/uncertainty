//! Validation of the top-down module against
//!
//! * Nordtest TR 537 ed. 4 (2017), *Handbook for calculation of measurement
//!   uncertainty in environmental laboratories*
//!   (https://nordtest.info/images/documents/nt-technical-reports/NT_TR_537_edition4_English_Handbook_for_calculation_of_measurement_uncertainty_in_environmental_laboratories.pdf);
//! * Eurachem/CITAC MUPR, 1st ed. (2026), *Evaluation of measurement
//!   uncertainty from in-house precision and recovery data*
//!   (https://eurachem.org/images/stories/Guides/pdf/MUPR_2026_EN_v01.pdf).
//!
//! Where the documents give raw data, the statistics are recomputed from the
//! raw data. Relative quantities are in per cent unless stated otherwise.

use uncertainty_core::report::{report_result, round_to_place, Rounding};
use uncertainty_core::topdown::*;

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol, "{what}: got {actual}, expected {expected} ± {tol}");
}

// --- Nordtest raw data ------------------------------------------------------

/// Appendix 5, NH4-N duplicates below 30 µg/L (µg/L). The appendix lists 46
/// pairs; §5.3 states n = 47.
const NH4_LOW: [(f64, f64); 46] = [
    (7.46, 7.25),
    (9.01, 9.17),
    (3.60, 3.10),
    (6.48, 6.48),
    (14.49, 14.12),
    (10.84, 10.89),
    (4.61, 5.00),
    (2.60, 2.42),
    (2.80, 2.62),
    (5.84, 6.19),
    (2.12, 2.50),
    (2.30, 2.11),
    (2.52, 2.89),
    (3.71, 3.71),
    (7.43, 7.43),
    (8.83, 8.51),
    (9.12, 8.79),
    (8.24, 7.90),
    (2.62, 2.78),
    (3.33, 3.33),
    (2.69, 2.69),
    (12.09, 12.09),
    (4.24, 4.24),
    (10.49, 10.64),
    (3.68, 3.52),
    (2.22, 2.06),
    (6.10, 6.10),
    (2.96, 2.86),
    (14.02, 13.70),
    (4.24, 3.62),
    (5.10, 4.61),
    (2.78, 2.62),
    (8.52, 6.81),
    (12.82, 14.05),
    (3.17, 2.40),
    (11.28, 11.43),
    (14.31, 13.82),
    (4.01, 4.48),
    (3.27, 3.58),
    (9.98, 10.29),
    (12.56, 13.66),
    (16.2, 16.6),
    (28.8, 28.7),
    (15.8, 18.5),
    (17.7, 16.7),
    (3.35, 2.88),
];

/// Appendix 5, NH4-N duplicates above 30 µg/L (µg/L), 26 pairs.
const NH4_HIGH: [(f64, f64); 26] = [
    (37.6, 36.9),
    (4490.0, 4413.0),
    (136.0, 125.0),
    (62.6, 62.3),
    (159.0, 159.0),
    (16540.0, 16080.0),
    (31.3, 30.1),
    (58.5, 60.1),
    (741.0, 796.0),
    (130.0, 127.0),
    (29.4, 29.2),
    (1372.0, 1388.0),
    (36.6, 44.7),
    (22.6, 23.4),
    (34.8, 33.2),
    (92.9, 94.0),
    (40.6, 42.2),
    (80.4, 86.4),
    (78.2, 73.8),
    (48.9, 50.9),
    (36.6, 35.3),
    (51.9, 52.2),
    (198.0, 207.0),
    (70.3, 69.2),
    (29.9, 30.6),
    (31.9, 32.4),
];

/// Appendix 6, dissolved oxygen in sea water, duplicate results (mg/L).
const OXYGEN: [(f64, f64); 51] = [
    (8.90, 8.91),
    (8.99, 9.01),
    (8.90, 8.90),
    (9.11, 9.12),
    (8.68, 8.64),
    (8.60, 8.51),
    (8.61, 8.61),
    (8.02, 8.00),
    (7.05, 7.08),
    (6.98, 7.01),
    (7.13, 7.10),
    (6.79, 6.78),
    (6.55, 6.55),
    (6.53, 6.53),
    (4.68, 4.68),
    (5.28, 5.33),
    (7.42, 7.40),
    (7.62, 7.63),
    (5.88, 5.88),
    (6.03, 6.06),
    (6.33, 6.33),
    (5.90, 5.90),
    (6.24, 6.27),
    (6.02, 6.02),
    (9.13, 9.11),
    (9.10, 9.14),
    (8.50, 8.44),
    (8.73, 8.71),
    (8.09, 8.09),
    (7.56, 7.58),
    (6.30, 6.32),
    (6.43, 6.44),
    (7.25, 7.34),
    (7.25, 7.31),
    (8.00, 8.03),
    (8.38, 8.29),
    (9.23, 9.29),
    (9.09, 9.08),
    (9.37, 9.36),
    (9.38, 9.37),
    (9.32, 9.25),
    (8.47, 8.49),
    (8.27, 8.28),
    (8.37, 8.31),
    (8.09, 8.15),
    (8.05, 8.03),
    (7.38, 7.40),
    (7.49, 7.49),
    (4.52, 4.49),
    (4.45, 4.44),
    (4.29, 4.27),
];

/// Appendix 7, BOD control results on a CRM (206 ± 5) mg/L, duplicates.
const BOD: [(f64, f64); 18] = [
    (219.0, 215.0),
    (206.0, 221.0),
    (221.0, 210.0),
    (215.0, 207.0),
    (199.0, 218.0),
    (224.0, 212.0),
    (216.0, 213.0),
    (196.0, 215.0),
    (210.0, 207.0),
    (228.0, 223.0),
    (207.0, 229.0),
    (207.0, 208.0),
    (224.0, 214.0),
    (201.0, 214.0),
    (219.0, 223.0),
    (217.0, 218.0),
    (206.0, 228.0),
    (215.0, 226.0),
];

/// Appendix 4, NH4-N proficiency tests: (assigned, laboratory result, s_R %, n_labs).
const NH4_PT: [(f64, f64, f64, usize); 6] = [
    (81.0, 83.0, 10.0, 31),
    (73.0, 75.0, 7.0, 36),
    (264.0, 269.0, 8.0, 32),
    (210.0, 213.0, 10.0, 35),
    (110.0, 112.0, 7.0, 36),
    (140.0, 144.0, 11.0, 34),
];

#[test]
fn nordtest_duplicates_appendices_5_and_6() {
    let low = sd_from_duplicates(&NH4_LOW).unwrap();
    close(low.sd, 0.44, 0.005, "NH4-N s_r below 30 µg/L (Appendix 5: 0.44 µg/L)");
    let high = relative_sd_from_duplicates(&NH4_HIGH).unwrap();
    close(high.sd * 100.0, 3.8, 0.05, "NH4-N relative s_r above 30 µg/L (3.8 %)");
    assert_eq!(high.dof, 26.0);

    let ox = sd_from_duplicates(&OXYGEN).unwrap();
    close(ox.sd, 0.0252, 0.00005, "O2 pooled s (Appendix 6: 0.0252 mg/L)");
    let mean = OXYGEN.iter().map(|(a, b)| a + b).sum::<f64>() / (2.0 * OXYGEN.len() as f64);
    close(mean, 7.50, 0.005, "O2 mean (§5.3 table: 7.50 mg/L)");
    close(ox.sd / mean * 100.0, 0.34, 0.005, "O2 relative s_r (0.34 %)");
    // §5.3: u(Rw) = √(0.34² + 0.5²) = 0.60 %.
    close(root_sum_of_squares(&[0.34, 0.5]), 0.60, 0.005, "O2 u(Rw)");
    // §5.3: low level √(0.5² + 0.44²) = 0.7 µg/L; high level √(1.5² + 3.8²) = 4.1 %.
    close(root_sum_of_squares(&[0.5, 0.44]), 0.67, 0.005, "u(Rw) low (0.7 µg/L)");
    close(root_sum_of_squares(&[1.5, 3.8]), 4.1, 0.05, "u(Rw) high (4.1 %)");
}

#[test]
fn nordtest_section_3_ammonium_from_raw_pt_data() {
    let biases: Vec<f64> = NH4_PT.iter().map(|&(x_ref, x, _, _)| 100.0 * (x - x_ref) / x_ref).collect();
    let u_crefs: Vec<f64> = NH4_PT.iter().map(|&(_, _, s_r, n)| u_cref_from_proficiency_test(s_r, n)).collect();
    // Appendix 4: mean bias +2.20 %, RMS 2.26 %, u(C_ref,i) 1.80 … 1.89, mean 1.52 %.
    close(biases.iter().sum::<f64>() / 6.0, 2.20, 0.005, "mean bias");
    close(root_mean_square(&biases), 2.26, 0.005, "RMS_bias");
    for (u, expected) in u_crefs.iter().zip([1.80, 1.17, 1.41, 1.69, 1.17, 1.89]) {
        close(*u, expected, 0.005, "u(C_ref,i)");
    }
    close(u_crefs.iter().sum::<f64>() / 6.0, 1.52, 0.005, "mean u(C_ref)");

    // §3.2 steps 4–6: u(Rw) = 3.34/2 = 1.67 %, u(bias) = 2.73 %, u_c = 3.20 %,
    // U = 6.40 → 7 %.
    let u_rw = sd_from_control_limits(3.34, 2.0);
    close(u_rw, 1.67, 1e-12, "u(Rw)");
    let u_bias = u_bias_from_references(&biases, &u_crefs).unwrap();
    close(u_bias, 2.73, 0.005, "u(bias)");
    let budget = TopDownBudget::new(
        vec![Component::relative("u(Rw)", u_rw / 100.0), Component::relative("u(bias)", u_bias / 100.0)],
        2.0,
    );
    let r = budget.evaluate(1.0);
    close(r.relative_combined_uncertainty * 100.0, 3.20, 0.005, "u_c");
    close(r.expanded_uncertainty * 100.0, 6.40, 0.01, "U");
    assert_eq!(round_to_place(r.expanded_uncertainty * 100.0, 0, Rounding::Up), 7.0);
}

#[test]
fn nordtest_section_6_bias_examples() {
    // §6.1 one CRM: 11.5 ± 0.5 (95 %), mean 11.9, s_bias 2.2 % (n = 12).
    // The handbook prints u(C_ref) = 0.25/11.5 = 2.16 %, but 0.25/11.5 =
    // 2.17 %; its result u(bias) = 4.1 % follows from 2.16 %, the correct
    // value is 4.15 % (research log F5).
    let bias = 100.0 * (11.9 - 11.5) / 11.5;
    let u_cref = 100.0 * (0.5 / 2.0) / 11.5;
    close(bias, 3.48, 0.005, "bias");
    close(u_cref, 2.174, 0.0005, "u(C_ref) = 0.25/11.5");
    close(u_bias_single_reference(3.48, 2.2, 12, 2.16), 4.1, 0.05, "u(bias) with the printed 2.16 %");
    close(u_bias_single_reference(bias, 2.2, 12, u_cref), 4.15, 0.005, "u(bias) recomputed");

    // §6.1 several CRMs: biases 3.48, −0.9, 2.5 %; u(C_ref) 2.16, 1.8, 1.8 % →
    // RMS 2.53 %, mean u(C_ref) 1.92 %, u(bias) 3.2 %.
    let b = [3.48, -0.9, 2.5];
    close(root_mean_square(&b), 2.53, 0.005, "RMS_bias");
    close(u_bias_from_references(&b, &[2.16, 1.8, 1.8]).unwrap(), 3.2, 0.05, "u(bias), three CRMs");

    // §6.2 proficiency tests: s_R = 8.7 %, n = 23 → 1.8 %; the six u(C_ref,i)
    // average 2.60 % (the handbook prints 2.7 % and then uses 2.6 %); biases
    // +2, +7, −2, +3, +6, +5 % → RMS 4.6 %, u(bias) 5.3 %.
    close(u_cref_from_proficiency_test(8.7, 23), 1.8, 0.05, "u(C_ref,1)");
    let u = [1.8, 2.9, 1.7, 4.1, 3.0, 2.1];
    close(u.iter().sum::<f64>() / 6.0, 2.60, 1e-12, "mean u(C_ref)");
    let b = [2.0, 7.0, -2.0, 3.0, 6.0, 5.0];
    close(root_mean_square(&b), 4.6, 0.01, "RMS_bias");
    close(u_bias_from_references(&b, &u).unwrap(), 5.3, 0.02, "u(bias), PT");

    // §6.3 recovery: recoveries 95, 98, 97, 96, 99, 96 % → RMS_bias 3.44 %;
    // u(conc) = 1.2/2 = 0.6 %, u(vol) = √((1/√3)² + 0.5²) = 0.76 %,
    // u(C_recovery) = 1.0 %, u(bias) = 3.6 %.
    let b: Vec<f64> = [95.0, 98.0, 97.0, 96.0, 99.0, 96.0].iter().map(|r| r - 100.0).collect();
    close(root_mean_square(&b), 3.44, 0.005, "RMS_bias (recovery)");
    let u_vol = root_sum_of_squares(&[1.0 / 3f64.sqrt(), 0.5]);
    close(u_vol, 0.76, 0.005, "u(vol)");
    let u_rec = root_sum_of_squares(&[0.6, u_vol]);
    close(u_rec, 1.0, 0.05, "u(C_recovery)");
    close(u_bias_from_references(&b, &[u_rec]).unwrap(), 3.6, 0.05, "u(bias), recovery");
}

#[test]
fn nordtest_section_8_bod_examples() {
    // Appendix 7: averages of the duplicate pairs give mean 214.8, s 5.6, 2.6 %.
    let av: Vec<f64> = BOD.iter().map(|(a, b)| 0.5 * (a + b)).collect();
    let mean = av.iter().sum::<f64>() / av.len() as f64;
    let s = (av.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (av.len() as f64 - 1.0)).sqrt();
    close(mean, 214.75, 1e-9, "BOD mean (printed as 214.8)");
    close(s, 5.6, 0.05, "BOD s");
    close(100.0 * s / mean, 2.6, 0.005, "BOD s %");

    // Example A (CRM): bias 4.3 %, s_bias 2.6 % (n = 19), u(C_ref) = 1.2 % →
    // u(bias) 4.5 %, u_c 5.2 %, U 10.4 → 11 %.
    close(100.0 * (214.8 - 206.0) / 206.0, 4.3, 0.05, "bias");
    let u_bias = u_bias_single_reference(4.3, 2.6, 19, 1.2);
    close(u_bias, 4.5, 0.05, "u(bias) A");
    let uc = root_sum_of_squares(&[2.6, u_bias]);
    close(uc, 5.2, 0.05, "u_c A");
    assert_eq!(round_to_place(2.0 * uc, 0, Rounding::Up), 11.0);

    // Example B (PT, Table 6): u(C_ref,i) = s_R/√n = 1.50, 1.32, 2.25 %, mean
    // 1.69 %; RMS of the tabulated biases (+4.5, −4.1, +2.3 %) 3.76 %
    // (3.77 % from the unrounded biases); u(bias) 4.12 %, u_c 4.87 %, U → 10 %.
    let pts = [(154.0, 161.0, 7.2, 23), (219.0, 210.0, 6.6, 25), (176.0, 180.0, 9.8, 19)];
    let u_crefs: Vec<f64> = pts.iter().map(|&(_, _, s, n)| u_cref_from_proficiency_test(s, n)).collect();
    for (u, e) in u_crefs.iter().zip([1.50, 1.32, 2.25]) {
        close(*u, e, 0.005, "u(C_ref,i)");
    }
    close(u_crefs.iter().sum::<f64>() / 3.0, 1.69, 0.005, "mean u(C_ref)");
    close(root_mean_square(&[4.5, -4.1, 2.3]), 3.76, 0.005, "RMS_bias (rounded biases)");
    let unrounded: Vec<f64> = pts.iter().map(|&(a, x, _, _)| 100.0 * (x - a) / a).collect();
    close(root_mean_square(&unrounded), 3.77, 0.005, "RMS_bias (unrounded)");
    let u_bias = u_bias_from_references(&[4.5, -4.1, 2.3], &u_crefs).unwrap();
    close(u_bias, 4.12, 0.005, "u(bias) B");
    let uc = root_sum_of_squares(&[2.6, u_bias]);
    close(uc, 4.87, 0.005, "u_c B");
    assert_eq!(round_to_place(2.0 * uc, 0, Rounding::Up), 10.0);
}

#[test]
fn nordtest_section_8_pcb_examples() {
    // Example C (CRM 152 ± 14 µg/kg, mean 144): bias 5.3 %, s_bias 8 %
    // (n = 22), u(C_ref) = 7/152 = 4.6 % → u(bias) 7.22 %, u_c 10.8 %, U 22 %.
    let bias: f64 = 100.0 * (144.0 - 152.0) / 152.0;
    close(bias.abs(), 5.3, 0.05, "bias");
    let u_cref = 100.0 * 7.0 / 152.0;
    close(u_cref, 4.6, 0.01, "u(C_ref)");
    let u_bias = u_bias_single_reference(5.3, 8.0, 22, 4.6);
    close(u_bias, 7.22, 0.005, "u(bias) C");
    let uc = root_sum_of_squares(&[8.0, u_bias]);
    close(uc, 10.8, 0.05, "u_c C");
    assert_eq!(round_to_place(2.0 * uc, 0, Rounding::Up), 22.0);
    // The summary table prints u(C_ref) = 4.7 % and u(bias) = 7.29 %, which
    // are mutually consistent but differ from 7/152 = 4.6 % used in the example.
    close(u_bias_single_reference(5.3, 8.0, 22, 4.7), 7.29, 0.005, "summary-table variant");

    // Example D (3 PT): biases −2, −12, −5 % → RMS 7.6 %; u(C_ref) mean 2.9 %;
    // u(bias) 8.1 %, u_c 11.4 %, U 22.8 → 23 %.
    let b = [-2.0, -12.0, -5.0];
    close(root_mean_square(&b), 7.6, 0.01, "RMS_bias D");
    let u_bias = u_bias_from_references(&b, &[2.7, 2.5, 3.5]).unwrap();
    close(u_bias, 8.1, 0.05, "u(bias) D");
    let uc = root_sum_of_squares(&[8.0, u_bias]);
    close(uc, 11.4, 0.01, "u_c D");
    assert_eq!(round_to_place(2.0 * uc, 0, Rounding::Up), 23.0);
}

#[test]
fn nordtest_section_7_reproducibility() {
    // s_R from a reproducibility limit R: s_R = R/2.8; U = 2 s_R
    // (conductivity 3.2 % → 6.4 ≈ 7 %; NH4-N 8.8 % → 17.6 ≈ 18 %).
    assert_eq!(round_to_place(2.0 * 3.2, 0, Rounding::Up), 7.0);
    assert_eq!(round_to_place(2.0 * 8.8, 0, Rounding::Up), 18.0);
}

// --- Eurachem/CITAC MUPR (2026) -----------------------------------------

#[test]
fn mupr_example_2_reference_materials() {
    // Two CRMs and one PT sample, ortho-phosphate (mg/L): R̄ = 1.008,
    // u_R̄ = 0.03705 (terms 0.0007308, 0.001476, 0.01015).
    let tests = [
        RecoveryTest::Reference { mean: 2.31, sd: 0.11, n: 12, reference: 2.24, u_reference: 0.10 / 2.0 },
        RecoveryTest::Reference { mean: 1.032, sd: 0.090, n: 10, reference: 1.000, u_reference: 0.050 / 2.0 },
        RecoveryTest::Reference { mean: 1.70, sd: 0.14, n: 1, reference: 1.77, u_reference: 0.23 / 2.0 },
    ];
    for (t, term) in tests.iter().zip([0.000_730_8, 0.001_476, 0.010_15]) {
        close(t.standard_uncertainty().powi(2), term, 0.000_000_5 + 0.0005 * term, "Eq. (9) term");
    }
    let r = mean_recovery(&tests).unwrap();
    close(r.mean, 1.008, 0.0005, "R̄");
    close(r.standard_uncertainty, 0.037_05, 0.000_01, "u_R̄");
}

#[test]
fn mupr_example_3_spiked_samples() {
    // Two spiked river waters (µg/L): R̄ = 96.2 %, u_R̄ = 10.02 %; 1.5 % with
    // sample A alone.
    let a = RecoveryTest::Spiked {
        mean_after: 66.30,
        sd_after: 1.405,
        n_after: 16,
        mean_before: 35.21,
        sd_before: 1.054,
        n_before: 16,
        spike: 30.00,
        u_spike: 0.10,
    };
    let b = RecoveryTest::Spiked {
        mean_after: 64.02,
        sd_after: 1.934,
        n_after: 5,
        mean_before: 55.14,
        sd_before: 1.802,
        n_before: 1,
        spike: 10.000,
        u_spike: 0.040,
    };
    close(a.recovery(), 1.036, 0.0005, "R_A");
    close(b.recovery(), 0.888, 0.0005, "R_B");
    close(a.standard_uncertainty().powi(2), 0.000_226_2, 0.000_000_5, "term A");
    close(b.standard_uncertainty().powi(2), 0.039_97, 0.000_05, "term B");
    let r = mean_recovery(&[a, b]).unwrap();
    close(r.mean * 100.0, 96.2, 0.05, "R̄");
    close(r.standard_uncertainty * 100.0, 10.02, 0.01, "u_R̄");
    close(mean_recovery(&[a]).unwrap().standard_uncertainty * 100.0, 1.5, 0.05, "u_R̄ (A only)");
    // Eq. (15): the two recoveries are compatible at k = 2.
    assert!(recoveries_equivalent(a.recovery(), a.standard_uncertainty(), b.recovery(), b.standard_uncertainty(), 2.0));
}

#[test]
fn mupr_example_b1_nitrate() {
    // B1.7.ii: 20 spiked drinking waters, c̄ = 0.3890, s_I = 0.0328 mg/L,
    // C = 0.400 ± 0.018 (k = 2) mg/L → R̄ = 97.2 %, u_R̄ = 0.0285.
    let rm = RecoveryTest::Reference { mean: 0.3890, sd: 0.0328, n: 20, reference: 0.400, u_reference: 0.009 };
    let r = mean_recovery(&[rm]).unwrap();
    close(r.mean, 0.9725, 1e-12, "R̄₁ (printed as 97.2 %)");
    close(r.standard_uncertainty, 0.0285, 0.00005, "u_R̄₁");
    // Eq. (B1.3): 0.982 ≤ t₁₉ = 2.093 → no correction, u′ = u_R̄/1.
    let sig = recovery_significance(&r, 19.0, 0.95);
    // The guide prints 0.982 = |1 − 0.972|/0.0285 using the rounded R̄; with
    // R̄ = 0.9725 the statistic is 0.963. Same conclusion either way.
    close(sig.statistic, 0.963, 0.0005, "|1 − R̄|/u");
    close((1.0 - 0.972) / 0.0285, 0.982, 0.0005, "printed statistic");
    close(sig.critical, 2.093, 0.0005, "t₁₉");
    assert!(!sig.significant);
    assert_eq!(sig.relative_uncertainty, r.standard_uncertainty);

    // Eq. (B1.4)–(B1.5): single-recovery limits with u from Eq. (B1.2), n = 1.
    let single = RecoveryTest::Reference { mean: 0.3890, sd: 0.0328, n: 1, reference: 0.400, u_reference: 0.009 };
    close(single.standard_uncertainty(), 0.084_87, 0.000_005, "u(R), n = 1 (0.0848 with R̄ rounded to 0.972)");
    let (lo, hi) = recovery_acceptance_limits(0.972, 0.0848, 19.0, 0.95);
    close(lo, 0.794, 0.0006, "L_R 95 % (0.7945 printed as 0.794)");
    close(hi, 1.15, 0.005, "H_R 95 %");
    let (lo, hi) = recovery_acceptance_limits(0.972, 0.0848, 19.0, 0.99);
    close(lo, 0.729, 0.0005, "L_R 99 %");
    close(hi, 1.21, 0.005, "H_R 99 %");

    // B1.7.iv: Interval I (c ≤ 0.4 mg/L, absolute s_I = 0.0328 mg/L) and
    // Interval II (relative s′_I = 8.43 %), with u′_R̄ = 0.0285, stock
    // solution u′_DSS = 0.0095 counted twice and dilution u′_F = 0.022.
    let u_r = 0.0285;
    let interval_i = TopDownBudget::new(
        vec![
            Component::absolute("precision", 0.0328),
            Component::relative("recovery", u_r),
            Component::relative("DSS", 0.0095).times(2),
        ],
        2.0,
    );
    let res = interval_i.evaluate(0.261);
    close(res.expanded_uncertainty, 0.0676, 0.00005, "U⟨I⟩ at 0.261 mg/L");
    let shares: Vec<f64> = res.contributions.iter().map(|c| c.1 * 100.0).collect();
    close(shares[0], 94.1, 0.1, "χ_I");
    close(shares[1], 4.8, 0.1, "χ_R");
    close(shares[2], 1.1, 0.1, "χ_DSS");
    let rep = report_result(
        0.261,
        res.expanded_uncertainty,
        2,
        Rounding::HalfEvenFirstDiscarded,
        Rounding::HalfEvenFirstDiscarded,
    );
    assert_eq!((rep.value.as_str(), rep.uncertainty.as_str()), ("0.261", "0.068"));

    let interval_ii = TopDownBudget::new(
        vec![
            Component::relative("precision", 0.0843),
            Component::relative("recovery", u_r),
            Component::relative("DSS", 0.0095).times(2),
            Component::relative("dilution", 0.022),
        ],
        2.0,
    );
    let res = interval_ii.evaluate(50.0);
    close(res.expanded_uncertainty / 50.0, 0.185, 0.0005, "U⟨II⟩/c");
    close(res.expanded_uncertainty, 9.26, 0.005, "U⟨II⟩ at 50 mg/L");
    let shares: Vec<f64> = res.contributions.iter().map(|c| c.1 * 100.0).collect();
    close(shares[0], 82.9, 0.15, "χ_I⟨II⟩");
    close(shares[1], 9.5, 0.1, "χ_R⟨II⟩");
    close(shares[2], 2.1, 0.1, "χ_DSS⟨II⟩");
    close(shares[3], 5.6, 0.1, "χ_F⟨II⟩");
    let rep = report_result(
        50.0,
        res.expanded_uncertainty,
        2,
        Rounding::HalfEvenFirstDiscarded,
        Rounding::HalfEvenFirstDiscarded,
    );
    assert_eq!((rep.value.as_str(), rep.uncertainty.as_str()), ("50.0", "9.3"));

    // B1.8 Eq. (B1.16): three results on three days at 57.5 mg/L → 7.13 mg/L.
    let s_mean = intermediate_precision_of_mean(0.0, 0.0843, 1, 3).unwrap();
    let reduced = TopDownBudget::new(
        vec![
            Component::relative("precision", s_mean),
            Component::relative("recovery", u_r),
            Component::relative("DSS", 0.0095).times(2),
            Component::relative("dilution", 0.022),
        ],
        2.0,
    );
    close(reduced.evaluate(57.5).expanded_uncertainty, 7.13, 0.005, "U with 3 days");

    // B1.9 Eq. (B1.18): PT (4.43 ± 0.82) vs (5.00 ± 0.20) mg/L, k = 2.
    let (d, lim, ok) = metrological_compatibility(4.43, 0.82 / 2.0, 5.00, 0.20 / 2.0, 3.0);
    close(d, 0.57, 1e-9, "|c − C|");
    close(lim, 1.27, 0.005, "3 u_d");
    assert!(ok);

    // B1.10: c_LOQ = 10 × 0.033 mg/L.
    close(10.0 * 0.033, 0.33, 1e-12, "LOQ");
}

#[test]
fn mupr_uncorrected_recovery_allowance() {
    // Eq. (17) with one reference reduces to R̄ √((R̄ − 1)² + precision² + ref²).
    let rm = RecoveryTest::Reference { mean: 0.3890, sd: 0.0328, n: 20, reference: 0.400, u_reference: 0.009 };
    let u = recovery_uncertainty_uncorrected(&[rm]).unwrap();
    let r: f64 = 0.389 / 0.4;
    let expected =
        r * ((r - 1.0).powi(2) + (0.0328 / (0.389 * 20f64.sqrt())).powi(2) + (0.009 / 0.4f64).powi(2)).sqrt();
    close(u, expected, 1e-15, "Eq. (17)");
    assert!(u > 0.0285);
}
