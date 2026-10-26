//! Special functions needed for uncertainty evaluation.
//!
//! All functions are implemented here (no external numerical library) so that
//! their accuracy can be documented and tested directly. Reference values used
//! in the unit tests were produced independently with SciPy 1.x
//! (`scipy.stats.t.ppf`, `scipy.special.erf`, ...); see
//! `docs/research/validation-matrix.md`.

use std::f64::consts::PI;

const LANCZOS_G: f64 = 7.0;
const LANCZOS_COEF: [f64; 9] = [
    0.999_999_999_999_809_9,
    676.520_368_121_885_1,
    -1_259.139_216_722_402_8,
    771.323_428_777_653_1,
    -176.615_029_162_140_6,
    12.507_343_278_686_905,
    -0.138_571_095_265_720_12,
    9.984_369_578_019_572e-6,
    1.505_632_735_149_311_6e-7,
];

/// Natural logarithm of the gamma function, ln Γ(x), for x > 0
/// (Lanczos approximation, g = 7, n = 9; relative accuracy about 1e-15).
pub fn ln_gamma(x: f64) -> f64 {
    if x < 0.5 {
        // Reflection formula: Γ(x) Γ(1 − x) = π / sin(πx).
        PI.ln() - (PI * x).sin().abs().ln() - ln_gamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let t = x + LANCZOS_G + 0.5;
        let mut a = LANCZOS_COEF[0];
        for (i, c) in LANCZOS_COEF.iter().enumerate().skip(1) {
            a += c / (x + i as f64);
        }
        0.5 * (2.0 * PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}

const EPS: f64 = 1e-16;
const FPMIN: f64 = 1e-300;
const MAX_ITER: usize = 10_000;

/// Regularized lower incomplete gamma function P(a, x).
pub fn gamma_p(a: f64, x: f64) -> f64 {
    assert!(a > 0.0, "gamma_p requires a > 0");
    if x <= 0.0 {
        0.0
    } else if x < a + 1.0 {
        gamma_series(a, x)
    } else {
        1.0 - gamma_continued_fraction(a, x)
    }
}

/// Regularized upper incomplete gamma function Q(a, x) = 1 − P(a, x).
pub fn gamma_q(a: f64, x: f64) -> f64 {
    assert!(a > 0.0, "gamma_q requires a > 0");
    if x <= 0.0 {
        1.0
    } else if x < a + 1.0 {
        1.0 - gamma_series(a, x)
    } else {
        gamma_continued_fraction(a, x)
    }
}

fn gamma_series(a: f64, x: f64) -> f64 {
    let mut ap = a;
    let mut sum = 1.0 / a;
    let mut del = sum;
    for _ in 0..MAX_ITER {
        ap += 1.0;
        del *= x / ap;
        sum += del;
        if del.abs() < sum.abs() * EPS {
            break;
        }
    }
    sum * (-x + a * x.ln() - ln_gamma(a)).exp()
}

/// Modified Lentz evaluation of the continued fraction for Q(a, x).
fn gamma_continued_fraction(a: f64, x: f64) -> f64 {
    let mut b = x + 1.0 - a;
    let mut c = 1.0 / FPMIN;
    let mut d = 1.0 / b;
    let mut h = d;
    for i in 1..MAX_ITER {
        let an = -(i as f64) * (i as f64 - a);
        b += 2.0;
        d = an * d + b;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = b + an / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < EPS {
            break;
        }
    }
    (-x + a * x.ln() - ln_gamma(a)).exp() * h
}

/// Error function erf(x).
pub fn erf(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x > 0.0 {
        gamma_p(0.5, x * x)
    } else {
        -gamma_p(0.5, x * x)
    }
}

/// Complementary error function erfc(x) = 1 − erf(x), accurate in the tails.
pub fn erfc(x: f64) -> f64 {
    if x >= 0.0 {
        gamma_q(0.5, x * x)
    } else {
        1.0 + gamma_p(0.5, x * x)
    }
}

/// Standard normal distribution function Φ(z).
pub fn normal_cdf(z: f64) -> f64 {
    0.5 * erfc(-z / std::f64::consts::SQRT_2)
}

/// Standard normal probability density φ(z).
pub fn normal_pdf(z: f64) -> f64 {
    (-0.5 * z * z).exp() / (2.0 * PI).sqrt()
}

/// Quantile Φ⁻¹(p) of the standard normal distribution, 0 < p < 1.
///
/// Acklam's rational approximation followed by one Halley refinement step
/// using the accurate `erfc`, giving close to full double precision.
pub fn normal_quantile(p: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "normal_quantile requires 0 < p < 1");
    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];
    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];
    const D: [f64; 4] =
        [7.784_695_709_041_462e-3, 3.224_671_290_700_398e-1, 2.445_134_137_142_996, 3.754_408_661_907_416];
    const P_LOW: f64 = 0.024_25;
    let x = if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= 1.0 - P_LOW {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    // Halley refinement.
    let e = normal_cdf(x) - p;
    let u = e * (2.0 * PI).sqrt() * (0.5 * x * x).exp();
    x - u / (1.0 + 0.5 * x * u)
}

/// Regularized incomplete beta function I_x(a, b).
pub fn beta_inc(a: f64, b: f64, x: f64) -> f64 {
    assert!(a > 0.0 && b > 0.0, "beta_inc requires a, b > 0");
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let ln_front = ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln();
    let front = ln_front.exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        front * beta_continued_fraction(a, b, x) / a
    } else {
        1.0 - front * beta_continued_fraction(b, a, 1.0 - x) / b
    }
}

/// Continued fraction for the incomplete beta function (modified Lentz).
fn beta_continued_fraction(a: f64, b: f64, x: f64) -> f64 {
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < FPMIN {
        d = FPMIN;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..MAX_ITER {
        let m = m as f64;
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < EPS {
            break;
        }
    }
    h
}

/// Upper tail probability Pr(T > t) of Student's t-distribution with ν degrees
/// of freedom (ν may be non-integer).
pub fn student_t_sf(t: f64, nu: f64) -> f64 {
    assert!(nu > 0.0, "degrees of freedom must be positive");
    if nu.is_infinite() {
        return normal_cdf(-t);
    }
    let x = nu / (nu + t * t);
    let tail = 0.5 * beta_inc(0.5 * nu, 0.5, x);
    if t >= 0.0 {
        tail
    } else {
        1.0 - tail
    }
}

/// Distribution function Pr(T ≤ t) of Student's t-distribution.
pub fn student_t_cdf(t: f64, nu: f64) -> f64 {
    if t > 0.0 {
        1.0 - student_t_sf(t, nu)
    } else {
        student_t_sf(-t, nu)
    }
}

/// Probability density of Student's t-distribution.
pub fn student_t_pdf(t: f64, nu: f64) -> f64 {
    if nu.is_infinite() {
        return normal_pdf(t);
    }
    (ln_gamma(0.5 * (nu + 1.0))
        - ln_gamma(0.5 * nu)
        - 0.5 * (nu * PI).ln()
        - 0.5 * (nu + 1.0) * (1.0 + t * t / nu).ln())
    .exp()
}

/// Quantile of Student's t-distribution: the value t with Pr(T ≤ t) = p.
///
/// Solved by safeguarded Newton iteration on the upper tail probability, which
/// keeps full relative accuracy for p close to 1.
pub fn student_t_quantile(p: f64, nu: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "student_t_quantile requires 0 < p < 1");
    assert!(nu > 0.0, "degrees of freedom must be positive");
    if nu.is_infinite() {
        return normal_quantile(p);
    }
    if p < 0.5 {
        return -student_t_quantile(1.0 - p, nu);
    }
    if p == 0.5 {
        return 0.0;
    }
    let target = 1.0 - p; // upper tail probability to match
                          // Bracket the root: sf is decreasing in t.
    let mut lo = 0.0_f64;
    let mut hi = normal_quantile(p).max(1.0);
    while student_t_sf(hi, nu) > target {
        lo = hi;
        hi *= 2.0;
        if hi > 1e300 {
            return f64::INFINITY;
        }
    }
    let mut t = 0.5 * (lo + hi);
    for _ in 0..200 {
        let f = student_t_sf(t, nu) - target;
        if f > 0.0 {
            lo = t;
        } else {
            hi = t;
        }
        // Newton step: d(sf)/dt = −pdf.
        let pdf = student_t_pdf(t, nu);
        let mut next = if pdf > 0.0 { t + f / pdf } else { f64::NAN };
        if !(next > lo && next < hi) {
            next = 0.5 * (lo + hi);
        }
        if (next - t).abs() <= 1e-15 * t.abs().max(1.0) {
            return next;
        }
        t = next;
    }
    t
}

/// Coverage factor k_p for a coverage probability p (two-sided) and ν degrees
/// of freedom: k_p = t_{(1+p)/2}(ν) [GUM G.3]; ν = ∞ gives the normal value.
pub fn coverage_factor(p: f64, nu: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "coverage probability must lie in (0, 1)");
    student_t_quantile(0.5 * (1.0 + p), nu)
}

/// Quantile of the chi-squared distribution with ν degrees of freedom: the
/// value x with P(ν/2, x/2) = p (e.g. the critical value χ²₀.₉₅,ν).
pub fn chi_squared_quantile(p: f64, nu: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "chi_squared_quantile requires 0 < p < 1");
    assert!(nu > 0.0, "degrees of freedom must be positive");
    let a = 0.5 * nu;
    let cdf = |x: f64| gamma_p(a, 0.5 * x);
    let pdf = |x: f64| ((a - 1.0) * (0.5 * x).ln() - 0.5 * x - ln_gamma(a)).exp() * 0.5;
    let mut lo = 0.0_f64;
    let mut hi = nu.max(1.0);
    while cdf(hi) < p {
        lo = hi;
        hi *= 2.0;
    }
    let mut x = 0.5 * (lo + hi);
    for _ in 0..200 {
        let f = cdf(x) - p;
        if f < 0.0 {
            lo = x;
        } else {
            hi = x;
        }
        let d = pdf(x);
        let mut next = if d > 0.0 { x - f / d } else { f64::NAN };
        if !(next > lo && next < hi) {
            next = 0.5 * (lo + hi);
        }
        if (next - x).abs() <= 1e-15 * x.abs().max(1e-300) {
            return next;
        }
        x = next;
    }
    x
}

/// Distribution function of the F distribution with (ν₁, ν₂) degrees of
/// freedom: P(F ≤ x) = I_{ν₁x/(ν₁x + ν₂)}(ν₁/2, ν₂/2).
pub fn f_cdf(x: f64, nu1: f64, nu2: f64) -> f64 {
    if !(x > 0.0) {
        return 0.0;
    }
    beta_inc(0.5 * nu1, 0.5 * nu2, nu1 * x / (nu1 * x + nu2))
}

/// Quantile of the F distribution with (ν₁, ν₂) degrees of freedom, by
/// bisection on [`f_cdf`] (e.g. F₀.₉₅(ν₁, ν₂), the value exceeded with
/// probability 0.05).
pub fn f_quantile(p: f64, nu1: f64, nu2: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "f_quantile requires 0 < p < 1");
    assert!(nu1 > 0.0 && nu2 > 0.0, "degrees of freedom must be positive");
    let mut lo = 0.0_f64;
    let mut hi = 1.0_f64;
    while f_cdf(hi, nu1, nu2) < p {
        lo = hi;
        hi *= 2.0;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if f_cdf(mid, nu1, nu2) < p {
            lo = mid;
        } else {
            hi = mid;
        }
        if hi - lo <= 1e-14 * hi {
            break;
        }
    }
    0.5 * (lo + hi)
}

/// Coverage factor for an output whose distribution is a symmetric
/// trapezoid with edge parameter β = b/a (half-width of the top over
/// half-width of the base), as when one or two rectangular contributions
/// dominate [EA-4/02 M:2022 S9.14, Eq. (S10.9)]. β = 1 is the rectangular
/// case k = p√3 (k = 1.65 for p = 0.95); β = 0 the triangular case.
pub fn trapezoidal_coverage_factor(p: f64, beta: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "coverage probability must lie in (0, 1)");
    assert!((0.0..=1.0).contains(&beta), "edge parameter must lie in [0, 1]");
    let u_over_a = ((1.0 + beta * beta) / 6.0).sqrt();
    if beta >= p / (2.0 - p) {
        // Interval end inside the plateau.
        p * (1.0 + beta) / (2.0 * u_over_a)
    } else {
        (1.0 - ((1.0 - p) * (1.0 - beta * beta)).sqrt()) / u_over_a
    }
}

/// Compensated (Neumaier) summation of a slice.
pub fn accurate_sum(values: &[f64]) -> f64 {
    let mut sum = 0.0_f64;
    let mut comp = 0.0_f64;
    for &v in values {
        let t = sum + v;
        if sum.abs() >= v.abs() {
            comp += (sum - t) + v;
        } else {
            comp += (v - t) + sum;
        }
        sum = t;
    }
    sum + comp
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(a: f64, b: f64) -> f64 {
        ((a - b) / b).abs()
    }

    #[test]
    fn ln_gamma_known_values() {
        assert!((ln_gamma(1.0)).abs() < 1e-14);
        assert!((ln_gamma(2.0)).abs() < 1e-14);
        assert!(rel(ln_gamma(0.5), 0.5 * PI.ln()) < 1e-14);
        // ln(10!) = ln(3628800)
        assert!(rel(ln_gamma(11.0), 3_628_800f64.ln()) < 1e-14);
    }

    #[test]
    fn erf_reference_values() {
        // SciPy: erf(0.5), erf(1), erf(2), erfc(3), erfc(5)
        assert!(rel(erf(0.5), 0.520_499_877_813_046_5) < 1e-14);
        assert!(rel(erf(1.0), 0.842_700_792_949_714_9) < 1e-14);
        assert!(rel(erf(2.0), 0.995_322_265_018_952_7) < 1e-14);
        assert!(rel(erfc(3.0), 2.209_049_699_858_544e-5) < 1e-12);
        assert!(rel(erfc(5.0), 1.537_459_794_428_034_8e-12) < 1e-11);
    }

    #[test]
    fn normal_quantile_reference_values() {
        assert!(rel(normal_quantile(0.975), 1.959_963_984_540_054) < 1e-14);
        assert!(rel(normal_quantile(0.995), 2.575_829_303_548_900_4) < 1e-14);
        assert!(rel(normal_quantile(0.5 * (1.0 + 0.9545)), 2.000_002_443_899_602_7) < 1e-13);
        assert!(rel(normal_quantile(1e-10), -6.361_340_902_404_056) < 1e-12);
    }

    #[test]
    fn student_t_quantile_reference_values() {
        // (ν, t_0.975, t_0.995) from scipy.stats.t.ppf
        let table = [
            (1.0, 12.706_204_736_174_694, 63.656_741_162_871_526),
            (2.0, 4.302_652_729_749_462, 9.924_843_200_918_287),
            (3.0, 3.182_446_305_283_707_8, 5.840_909_309_733_355),
            (5.0, 2.570_581_835_636_314_6, 4.032_142_983_555_228),
            (10.0, 2.228_138_851_986_274, 3.169_272_672_616_95),
            (16.0, 2.119_905_299_221_254_6, 2.920_781_622_425_1),
            (18.0, 2.100_922_040_241_038_2, 2.878_440_472_738_607_7),
            (24.0, 2.063_898_561_628_024_5, 2.796_939_504_774_456),
            (50.0, 2.008_559_112_100_761, 2.677_793_270_940_844),
            (100.0, 1.983_971_518_523_551_8, 2.625_890_521_438_017_3),
        ];
        for (nu, t975, t995) in table {
            assert!(rel(student_t_quantile(0.975, nu), t975) < 1e-11, "nu={nu}");
            assert!(rel(student_t_quantile(0.995, nu), t995) < 1e-11, "nu={nu}");
        }
        // Non-integer degrees of freedom.
        assert!(rel(student_t_quantile(0.975, 2.5), 3.574_654_842_003_681_7) < 1e-11);
        assert!(rel(student_t_quantile(0.975, 16.7), 2.112_706_420_659_590_5) < 1e-11);
        assert_eq!(student_t_quantile(0.975, f64::INFINITY), normal_quantile(0.975));
    }

    #[test]
    fn coverage_factor_gum_table_g2() {
        // GUM Table G.2 (rounded to two decimals): t_99(16) = 2.92, t_95(∞) = 1.96,
        // t_95.45(∞) = 2.00.
        assert!((coverage_factor(0.99, 16.0) - 2.92).abs() < 0.005);
        assert!((coverage_factor(0.95, f64::INFINITY) - 1.96).abs() < 0.005);
        assert!((coverage_factor(0.9545, f64::INFINITY) - 2.00).abs() < 0.005);
    }

    #[test]
    fn f_quantile_reference_values() {
        // F₀.₉₅ critical values (e.g. NIST/SEMATECH e-Handbook 1.3.6.7.3).
        for &(n1, n2, f) in
            &[(1.0, 10.0, 4.965), (5.0, 10.0, 3.326), (9.0, 10.0, 3.020), (10.0, 20.0, 2.348), (3.0, 3.0, 9.277)]
        {
            assert!((f_quantile(0.95, n1, n2) - f).abs() < 0.0015, "F({n1}, {n2})");
        }
        assert!((f_cdf(f_quantile(0.3, 4.0, 7.0), 4.0, 7.0) - 0.3).abs() < 1e-12);
    }

    #[test]
    fn chi_squared_quantile_reference_values() {
        // scipy.stats.chi2.ppf(p, ν)
        let table = [
            (1.0, 3.841_458_820_694_124, 0.003_932_140_000_019_522),
            (2.0, 5.991_464_547_107_979, 0.102_586_588_775_101_06),
            (5.0, 11.070_497_693_516_351, 1.145_476_226_061_769_2),
            (10.0, 18.307_038_053_275_146, 3.940_299_136_119_060_5),
            (30.0, 43.772_971_825_742_19, 18.492_660_981_953_47),
        ];
        for (nu, q95, q05) in table {
            assert!(rel(chi_squared_quantile(0.95, nu), q95) < 1e-11, "nu={nu}");
            assert!(rel(chi_squared_quantile(0.05, nu), q05) < 1e-9, "nu={nu}");
        }
    }

    #[test]
    fn ea_4_02_table_e1_and_trapezoidal_factors() {
        // EA-4/02 Table E.1: k = t_95.45(ν) to two decimals.
        let table = [
            (1.0, 13.97),
            (2.0, 4.53),
            (3.0, 3.31),
            (4.0, 2.87),
            (5.0, 2.65),
            (6.0, 2.52),
            (7.0, 2.43),
            (8.0, 2.37),
            (9.0, 2.32),
            (10.0, 2.28),
            (11.0, 2.25),
            (12.0, 2.23),
            (13.0, 2.21),
            (14.0, 2.20),
            (15.0, 2.18),
            (16.0, 2.17),
            (17.0, 2.16),
            (18.0, 2.15),
            (19.0, 2.14),
            (20.0, 2.13),
            (25.0, 2.11),
            (30.0, 2.09),
            (35.0, 2.07),
            (40.0, 2.06),
            (45.0, 2.06),
            (50.0, 2.05),
        ];
        for (nu, k) in table {
            assert!((coverage_factor(0.9545, nu) - k).abs() < 0.005, "nu={nu}");
        }
        // S9.14: rectangular, k = 0.95·√3 = 1.65; S10.10: β = 0.33 → 1.83.
        assert!((trapezoidal_coverage_factor(0.95, 1.0) - 0.95 * 3f64.sqrt()).abs() < 1e-14);
        assert!((trapezoidal_coverage_factor(0.95, 1.0 / 3.0) - 1.83).abs() < 0.005);
        // Triangular limit β = 0: U = a(1 − √(1 − p)), u = a/√6.
        let tri = (1.0 - 0.05f64.sqrt()) * 6f64.sqrt();
        assert!((trapezoidal_coverage_factor(0.95, 0.0) - tri).abs() < 1e-14);
        // Continuity at the branch point β = p/(2 − p).
        let b = 0.95 / 1.05;
        let k1 = trapezoidal_coverage_factor(0.95, b);
        let k2 = trapezoidal_coverage_factor(0.95, b - 1e-12);
        assert!((k1 - k2).abs() < 1e-9);
    }

    #[test]
    fn t_cdf_inverts_quantile() {
        for &nu in &[1.0, 2.0, 4.5, 30.0] {
            for &p in &[0.6, 0.9, 0.975, 0.999] {
                let t = student_t_quantile(p, nu);
                assert!((student_t_cdf(t, nu) - p).abs() < 1e-13);
            }
        }
    }
}
