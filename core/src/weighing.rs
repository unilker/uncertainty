//! Approximation of the errors of indication of a non-automatic weighing
//! instrument by a function of the indication — EURAMET Calibration Guide
//! No. 18 v4.0 (2015), Appendix C2.2.
//!
//! The errors E_j found at the test points I_j are fitted by a polynomial
//! E(I) = a₀ + a₁I + … + a_nₐ I^nₐ, or one without a₀ (through zero, C2.2-16),
//! by generalized least squares with the covariance matrix
//! U(e) = U(m_ref) + U(I_Cal) + U(mod) (C2.2-3a): fully correlated reference
//! masses U(m_ref) = s sᵀ (C2.2-3b), the variances of the indications and a
//! model term s_m²·I (C2.2-3c). The fit gives the coefficients and their
//! covariance matrix (C2.2-6, -9), the residuals and χ²_obs (C2.2-7, -8), the
//! goodness-of-fit criteria χ²_obs ≤ ν (C2.2-2a) and |f(I_j) − E_j| < U(f(I_j))
//! (C2.2-2b), and the approximated error with its uncertainty at any reading
//! (C2.2-11 to -14).

use crate::error::UncError;

fn invalid(msg: impl Into<String>) -> UncError {
    UncError::InvalidOption(msg.into())
}

/// One test point of the calibration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TestPoint {
    /// Indication I_j.
    pub indication: f64,
    /// Error of indication E_j = I_j − m_ref,j (in the unit of the fit).
    pub error: f64,
    /// u(m_ref,j), in the unit of the errors.
    pub u_reference: f64,
    /// u(I_j), in the unit of the errors.
    pub u_indication: f64,
}

/// Correlation of the reference masses between test points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceCorrelation {
    /// U(m_ref) = s sᵀ: "reasonably high correlation" (C2.2-3b).
    Full,
    /// Uncorrelated reference masses: diagonal U(m_ref).
    None,
}

/// The approximation function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Polynomial {
    /// Degree nₐ ≥ 1.
    pub degree: usize,
    /// Without the constant term a₀ (e.g. E = a₁I, C2.2-16).
    pub through_zero: bool,
}

impl Polynomial {
    /// Powers of I in the basis.
    pub fn powers(&self) -> Vec<i32> {
        let first = if self.through_zero { 1 } else { 0 };
        (first..=self.degree as i32).collect()
    }

    pub fn row(&self, x: f64) -> Vec<f64> {
        self.powers().iter().map(|&p| x.powi(p)).collect()
    }

    /// Derivative of the basis with respect to I.
    pub fn derivative_row(&self, x: f64) -> Vec<f64> {
        self.powers().iter().map(|&p| if p == 0 { 0.0 } else { p as f64 * x.powi(p - 1) }).collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Approximation {
    pub function: Polynomial,
    /// Coefficients in the order of [`Polynomial::powers`].
    pub coefficients: Vec<f64>,
    /// U(â) = (XᵀPX)⁻¹ (C2.2-9).
    pub covariance: Vec<Vec<f64>>,
    pub s_m: f64,
    /// χ²_obs = vᵀPv (C2.2-8) and ν = n − n_par.
    pub chi_squared: f64,
    pub dof: usize,
    /// E_appr,j = f(I_j) (C2.2-10).
    pub approximated: Vec<f64>,
    /// v_j = f(I_j) − E_j (C2.2-7).
    pub residuals: Vec<f64>,
    /// u(E_appr,j) = √diag(X U(â) Xᵀ) (C2.2-11).
    pub u_approximated: Vec<f64>,
    /// √U(e)_jj: the standard uncertainty of each error used in the fit.
    pub u_error: Vec<f64>,
}

impl Approximation {
    /// Criterion C2.2-2a: χ²_obs ≤ ν.
    pub fn chi_squared_passed(&self) -> bool {
        self.chi_squared <= self.dof as f64
    }

    /// The points (indices) failing C2.2-2b, |v_j| < k·u(E_appr,j). A point
    /// with zero residual passes even where u(E_appr,j) = 0 (zero load of a
    /// function through zero).
    pub fn residual_failures(&self, k: f64) -> Vec<usize> {
        (0..self.residuals.len())
            .filter(|&j| {
                let v = self.residuals[j].abs();
                let u = k * self.u_approximated[j];
                !(v < u || (v == 0.0 && u == 0.0))
            })
            .collect()
    }

    /// Approximated error at a reading R and its standard uncertainty
    /// (C2.2-12 to -14): u² = (f′(R))²u²(R) + rᵀU(â)r.
    pub fn at(&self, reading: f64, u_reading: f64) -> (f64, f64) {
        let r = self.function.row(reading);
        let dr = self.function.derivative_row(reading);
        let e = dot(&r, &self.coefficients);
        let slope = dot(&dr, &self.coefficients);
        let var = slope * slope * u_reading * u_reading + quad(&self.covariance, &r);
        (e, var.max(0.0).sqrt())
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn quad(m: &[Vec<f64>], x: &[f64]) -> f64 {
    (0..x.len()).map(|i| x[i] * dot(&m[i], x)).sum()
}

/// Solves A Z = B (columns of B) by Gaussian elimination with partial
/// pivoting.
fn solve(a: &[Vec<f64>], b: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    let m = b.first().map_or(0, Vec::len);
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    for c in 0..n {
        let p = (c..n).max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))?;
        if !(a[p][c].abs() > 0.0) || !a[p][c].is_finite() {
            return None;
        }
        a.swap(c, p);
        b.swap(c, p);
        for r in c + 1..n {
            let f = a[r][c] / a[c][c];
            if f != 0.0 {
                for k in c..n {
                    a[r][k] -= f * a[c][k];
                }
                for k in 0..m {
                    b[r][k] -= f * b[c][k];
                }
            }
        }
    }
    let mut x = vec![vec![0.0; m]; n];
    for k in 0..m {
        for r in (0..n).rev() {
            let s: f64 = (r + 1..n).map(|i| a[r][i] * x[i][k]).sum();
            x[r][k] = (b[r][k] - s) / a[r][r];
        }
    }
    Some(x)
}

/// The covariance matrix U(e) of C2.2-3a.
pub fn error_covariance(points: &[TestPoint], correlation: ReferenceCorrelation, s_m: f64) -> Vec<Vec<f64>> {
    let n = points.len();
    let mut u = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let r = match correlation {
                ReferenceCorrelation::Full => points[i].u_reference * points[j].u_reference,
                ReferenceCorrelation::None if i == j => points[i].u_reference.powi(2),
                ReferenceCorrelation::None => 0.0,
            };
            u[i][j] = r;
        }
        u[i][i] += points[i].u_indication.powi(2) + s_m * s_m;
    }
    u
}

/// Generalized least-squares approximation (C2.2-3 to -11).
pub fn approximate(
    points: &[TestPoint],
    function: Polynomial,
    correlation: ReferenceCorrelation,
    s_m: f64,
) -> Result<Approximation, UncError> {
    if function.degree == 0 {
        return Err(invalid("the degree of the approximation must be at least 1"));
    }
    let n = points.len();
    let npar = function.powers().len();
    if n <= npar {
        return Err(invalid(format!("{npar} coefficients need more than {npar} test points")));
    }
    let finite = points
        .iter()
        .all(|p| p.indication.is_finite() && p.error.is_finite() && p.u_reference >= 0.0 && p.u_indication >= 0.0);
    if !finite || !(s_m >= 0.0 && s_m.is_finite()) {
        return Err(invalid("test points need finite values and non-negative uncertainties"));
    }
    let u = error_covariance(points, correlation, s_m);
    let x: Vec<Vec<f64>> = points.iter().map(|p| function.row(p.indication)).collect();
    let e: Vec<f64> = points.iter().map(|p| p.error).collect();
    // W = U⁻¹[X | e].
    let rhs: Vec<Vec<f64>> = (0..n).map(|i| x[i].iter().cloned().chain([e[i]]).collect()).collect();
    let w = solve(&u, &rhs).ok_or_else(|| invalid("the covariance matrix U(e) is singular"))?;
    let mut a = vec![vec![0.0; npar]; npar];
    let mut b = vec![vec![0.0]; npar];
    for j in 0..npar {
        for k in 0..npar {
            a[j][k] = (0..n).map(|i| x[i][j] * w[i][k]).sum();
        }
        b[j][0] = (0..n).map(|i| x[i][j] * w[i][npar]).sum();
    }
    let ident: Vec<Vec<f64>> = (0..npar).map(|j| (0..npar).map(|k| if j == k { 1.0 } else { 0.0 }).collect()).collect();
    let covariance = solve(&a, &ident).ok_or_else(|| invalid("the normal equations are singular"))?;
    let coefficients: Vec<f64> =
        (0..npar).map(|j| dot(&covariance[j], &b.iter().map(|r| r[0]).collect::<Vec<_>>())).collect();
    let approximated: Vec<f64> = x.iter().map(|r| dot(r, &coefficients)).collect();
    let residuals: Vec<f64> = (0..n).map(|i| approximated[i] - e[i]).collect();
    let z = solve(&u, &residuals.iter().map(|&v| vec![v]).collect::<Vec<_>>())
        .ok_or_else(|| invalid("the covariance matrix U(e) is singular"))?;
    let chi_squared = (0..n).map(|i| residuals[i] * z[i][0]).sum();
    let u_approximated = x.iter().map(|r| quad(&covariance, r).max(0.0).sqrt()).collect();
    let u_error = (0..n).map(|i| u[i][i].sqrt()).collect();
    Ok(Approximation {
        function,
        coefficients,
        covariance,
        s_m,
        chi_squared,
        dof: n - npar,
        approximated,
        residuals,
        u_approximated,
        u_error,
    })
}

/// Which goodness-of-fit criterion the model term s_m has to satisfy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Criterion {
    /// χ²_obs ≤ ν (C2.2-2a).
    ChiSquared,
    /// Every residual within its expanded uncertainty, coverage factor k (C2.2-2b).
    Residuals,
}

/// The smallest s_m for which the approximation meets the criterion, found
/// by bisection to a relative precision of 10⁻⁶ (s_m is enlarged "in an
/// iterative way", C2.2-3c). Returns 0 when the fit meets it without s_m.
pub fn minimum_s_m(
    points: &[TestPoint],
    function: Polynomial,
    correlation: ReferenceCorrelation,
    criterion: Criterion,
    k: f64,
) -> Result<f64, UncError> {
    let ok = |s: f64| -> Result<bool, UncError> {
        let a = approximate(points, function, correlation, s)?;
        Ok(match criterion {
            Criterion::ChiSquared => a.chi_squared_passed(),
            Criterion::Residuals => a.residual_failures(k).is_empty(),
        })
    };
    if ok(0.0)? {
        return Ok(0.0);
    }
    let scale = points.iter().map(|p| p.error.abs().max(p.u_indication)).fold(0.0, f64::max).max(f64::MIN_POSITIVE);
    let (mut lo, mut hi) = (0.0, scale);
    let mut tries = 0;
    while !ok(hi)? {
        lo = hi;
        hi *= 2.0;
        tries += 1;
        if tries > 60 {
            return Err(invalid("no model term s_m satisfies the criterion"));
        }
    }
    while hi - lo > 1e-6 * hi {
        let mid = 0.5 * (lo + hi);
        if ok(mid)? {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Ok(hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} vs {b}");
    }

    /// EURAMET cg-18 Example H4: 400 g / 0.1 mg balance, E = a₁I.
    fn h4_points() -> Vec<TestPoint> {
        let mc = [0.0, 50.000006, 99.999987, 149.999993, 200.000013, 250.000019, 300.0, 350.000006, 400.00001];
        let ind = [0.0, 50.000067, 100.0001, 150.000233, 200.000267, 250.0001, 300.0002, 350.000267, 400.0004];
        // u(m_ref): U(m_c)/2 and the drift summed over the weights used, the
        // buoyancy term of 7.1.2-5b as a (negative) variance.
        let weights: [&[(f64, f64)]; 9] = [
            &[],
            &[(0.015, 0.005)],
            &[(0.025, 0.010)],
            &[(0.025, 0.010), (0.015, 0.005)],
            &[(0.045, 0.015)],
            &[(0.045, 0.015), (0.015, 0.005)],
            &[(0.045, 0.015), (0.025, 0.010)],
            &[(0.045, 0.015), (0.025, 0.010), (0.015, 0.005)],
            &[(0.045, 0.015), (0.045, 0.015)],
        ];
        let d = 0.1 / 12f64.sqrt();
        (0..9)
            .map(|j| {
                let n = j as f64 * 50.0;
                let uc: f64 = weights[j].iter().map(|w| w.0).sum();
                let ud: f64 = weights[j].iter().map(|w| w.1).sum();
                let b =
                    (n * 1000.0).powi(2) * (1.09 - 1.2) * (1.09 - 1.2 - 2.0 * (1.045 - 1.2)) * 3600.0 / 8000f64.powi(4);
                let ecc = 0.10 * n / (2.0 * 200.0 * 3f64.sqrt());
                let digl = if j == 0 { 0.0 } else { d };
                TestPoint {
                    indication: ind[j],
                    error: (ind[j] - mc[j]) * 1000.0,
                    u_reference: (uc * uc + ud * ud + b).max(0.0).sqrt(),
                    u_indication: (0.03f64.powi(2) + d * d + digl * digl + ecc * ecc).sqrt(),
                }
            })
            .collect()
    }

    const LINE: Polynomial = Polynomial { degree: 1, through_zero: true };

    #[test]
    fn example_h4_without_model_term() {
        let p = h4_points();
        // Budget: u(m_ref) 0.014 … 0.077 mg, u(I) 0.042 … 0.077 mg.
        close(p[1].u_reference, 0.0142, 1e-4);
        close(p[8].u_reference, 0.0769, 1e-4);
        close(p[8].u_indication, 0.0768, 1e-4);
        let a = approximate(&p, LINE, ReferenceCorrelation::Full, 0.0).unwrap();
        close(a.coefficients[0], 0.00083, 5e-6);
        close(a.covariance[0][0], 5.109e-8, 5e-11);
        close(a.chi_squared, 12.5, 0.05);
        assert_eq!(a.dof, 8);
        assert!(!a.chi_squared_passed());
    }

    #[test]
    fn example_h4_with_model_terms() {
        let p = h4_points();
        let a = approximate(&p, LINE, ReferenceCorrelation::Full, 0.05).unwrap();
        close(a.coefficients[0], 0.00084, 5e-6);
        close(a.covariance[0][0], 5.637e-8, 5e-11);
        close(a.chi_squared, 7.3, 0.05);
        assert!(a.chi_squared_passed());
        // Table H4.6: u(E_appr) and the two points failing C2.2-2b.
        for (j, u) in [(1, 0.012), (2, 0.024), (4, 0.047), (8, 0.095)] {
            close(a.u_approximated[j], u, 5e-4);
        }
        assert_eq!(a.residual_failures(2.0), vec![3, 5]);
        let b = approximate(&p, LINE, ReferenceCorrelation::Full, 0.25).unwrap();
        close(b.covariance[0][0], 1.745e-7, 5e-10);
        close(b.covariance[0][0].sqrt(), 0.00042, 5e-6);
        assert!(b.residual_failures(2.0).is_empty());
        // Table H4.8: U(E_appr) at 400 g.
        close(2.0 * b.u_approximated[8], 0.334, 5e-4);
        let (e, u) = b.at(400.0004, 0.0);
        close(e, b.approximated[8], 1e-12);
        close(u, b.u_approximated[8], 1e-12);
    }

    #[test]
    fn minimum_model_term() {
        let p = h4_points();
        let s2a = minimum_s_m(&p, LINE, ReferenceCorrelation::Full, Criterion::ChiSquared, 2.0).unwrap();
        assert!(s2a > 0.0 && s2a < 0.05);
        let a = approximate(&p, LINE, ReferenceCorrelation::Full, s2a).unwrap();
        close(a.chi_squared, 8.0, 1e-4);
        let s2b = minimum_s_m(&p, LINE, ReferenceCorrelation::Full, Criterion::Residuals, 2.0).unwrap();
        assert!(s2b > 0.05 && s2b < 0.25);
    }

    #[test]
    fn ordinary_regression_and_errors() {
        // Without covariances and model term: weighted least squares of a
        // straight line with intercept reproduces the exact line.
        let p: Vec<TestPoint> = (0..5)
            .map(|i| TestPoint {
                indication: i as f64,
                error: 1.0 + 2.0 * i as f64,
                u_reference: 0.0,
                u_indication: 0.1,
            })
            .collect();
        let a =
            approximate(&p, Polynomial { degree: 1, through_zero: false }, ReferenceCorrelation::None, 0.0).unwrap();
        close(a.coefficients[0], 1.0, 1e-12);
        close(a.coefficients[1], 2.0, 1e-12);
        close(a.chi_squared, 0.0, 1e-20);
        // Var(slope) of an unweighted fit: σ²/Σ(x − x̄)² = 0.01/10.
        close(a.covariance[1][1], 0.001, 1e-15);
        let (e, u) = a.at(2.0, 0.05);
        close(e, 5.0, 1e-12);
        close(u, (4.0f64 * 0.0025 + 0.01 / 5.0).sqrt(), 1e-12);
        assert!(approximate(&p[..2], Polynomial { degree: 1, through_zero: false }, ReferenceCorrelation::None, 0.0)
            .is_err());
        assert!(
            approximate(&p, Polynomial { degree: 0, through_zero: false }, ReferenceCorrelation::None, 0.0).is_err()
        );
    }
}
