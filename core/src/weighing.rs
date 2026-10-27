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

// ---------------------------------------------------------------------------
// Uncertainty of a weighing result in use (7.4, 7.5) and minimum weight (G)
// ---------------------------------------------------------------------------

/// Relative standard uncertainty of an off-centre load in use, the effect
/// found at calibration taken in full: ΔI_ecc,max/(L_ecc√3) (7.4.4-10).
pub fn relative_eccentricity(deviation: f64, load: f64) -> f64 {
    deviation.abs() / (load * 3f64.sqrt())
}

/// Temperature drift of the sensitivity, K_T·ΔT/√12 (7.4.3-1).
pub fn relative_temperature(coefficient: f64, delta_t: f64) -> f64 {
    (coefficient * delta_t).abs() / 12f64.sqrt()
}

/// Change of the adjustment due to the air density, instrument not adjusted
/// before use, from the temperature variation ΔT at the site (7.4.3-4):
/// √(1.07·10⁻⁴ + 1.33·10⁻⁶ K⁻²·ΔT²)·ρ₀/ρ_c.
pub fn relative_buoyancy(delta_t: f64, rho_0: f64, rho_c: f64) -> f64 {
    (1.07e-4 + 1.33e-6 * delta_t * delta_t).sqrt() * rho_0 / rho_c
}

/// The most conservative buoyancy term, 0.1ρ₀/(ρ_c√3) (7.4.3-5).
pub fn relative_buoyancy_conservative(rho_0: f64, rho_c: f64) -> f64 {
    0.1 * rho_0 / (rho_c * 3f64.sqrt())
}

/// Drift of the adjustment between calibrations, ΔE(Max)/(Max√3) (7.4.3-6).
pub fn relative_adjustment(delta_e_max: f64, max: f64) -> f64 {
    delta_e_max.abs() / (max * 3f64.sqrt())
}

/// Tare balancing: the non-linearity of the error curve from the slopes
/// q_E = ΔE/ΔI between consecutive test points, (q_E,max − q_E,min)/√12
/// (7.4.4-4, -5). `points` are (indication, error) in the same unit.
pub fn relative_tare(points: &[(f64, f64)]) -> Result<f64, UncError> {
    let mut p = points.to_vec();
    p.sort_by(|a, b| a.0.total_cmp(&b.0));
    let q: Vec<f64> = p.windows(2).filter(|w| w[1].0 > w[0].0).map(|w| (w[1].1 - w[0].1) / (w[1].0 - w[0].0)).collect();
    if q.is_empty() {
        return Err(invalid("the tare term needs at least two test points with different indications"));
    }
    let max = q.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min = q.iter().copied().fold(f64::INFINITY, f64::min);
    Ok((max - min) / 12f64.sqrt())
}

/// Creep and hysteresis from the error on return to zero after loading
/// upwards, |E₀|/(Max√3) (7.4.4-7), or from m·mpe(Max)/(Max√3) (7.4.4-9a, -9b).
pub fn relative_time(limit: f64, max: f64) -> f64 {
    limit.abs() / (max * 3f64.sqrt())
}

/// One weighing interval (or range) in use: upper limit Max_i, scale
/// interval d_i and the standard deviation s of a single reading.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UseInterval {
    pub max: f64,
    pub d: f64,
    pub s: f64,
}

/// The uncertainty of a weighing result in use, u²(W) = α_W² + β_W²R²
/// (7.4.5-1b, -2), with the error approximated by E(R) = a₁R (C2.2-16).
#[derive(Clone, Debug, PartialEq)]
pub struct WeighingInUse {
    /// Intervals in ascending order of Max.
    pub intervals: Vec<UseInterval>,
    /// Scale interval at zero, d₀.
    pub d0: f64,
    /// Relative standard uncertainties, u(a₁) of the error function included.
    pub relative: Vec<f64>,
    /// Slope a₁ of the approximated error.
    pub a1: f64,
    /// Coverage factor (7.5.1-2b; normally 2).
    pub k: f64,
}

/// The linear forms of 7.5.2-3d/-3f and 7.5.2-3e for each interval:
/// U(W) ≈ U(L_i) + b_i(R − L_i) and U_gl(W) ≈ c_i + (b_i + |a₁|)(R − L_i),
/// where L_i = Max_{i−1} (0 for the first interval).
#[derive(Clone, Debug, PartialEq)]
pub struct InUseResult {
    pub alpha_squared: Vec<f64>,
    pub beta_squared: f64,
    pub lower: Vec<f64>,
    pub expanded_at_lower: Vec<f64>,
    pub slope: Vec<f64>,
    pub global_at_lower: Vec<f64>,
    pub global_slope: Vec<f64>,
}

impl WeighingInUse {
    pub fn evaluate(&self) -> Result<InUseResult, UncError> {
        if self.intervals.is_empty() {
            return Err(invalid("at least one weighing interval is needed"));
        }
        let mut prev = 0.0;
        for i in &self.intervals {
            if !(i.max > prev && i.d > 0.0 && i.s >= 0.0 && i.max.is_finite() && i.s.is_finite()) {
                return Err(invalid("intervals need increasing Max > 0, d > 0 and s ≥ 0"));
            }
            prev = i.max;
        }
        if !(self.d0 > 0.0 && self.k > 0.0 && self.a1.is_finite())
            || self.relative.iter().any(|u| !(u.is_finite() && *u >= 0.0))
        {
            return Err(invalid("d₀ and k must be positive and the relative terms finite and non-negative"));
        }
        let beta_squared: f64 = self.relative.iter().map(|u| u * u).sum();
        let alpha_squared: Vec<f64> =
            self.intervals.iter().map(|i| self.d0 * self.d0 / 12.0 + i.d * i.d / 12.0 + i.s * i.s).collect();
        let n = self.intervals.len();
        let (mut lower, mut at_lower, mut slope, mut g_lower, mut g_slope) = (vec![], vec![], vec![], vec![], vec![]);
        for i in 0..n {
            let lo = if i == 0 { 0.0 } else { self.intervals[i - 1].max };
            let hi = self.intervals[i].max;
            let u = |r: f64| self.k * (alpha_squared[i] + beta_squared * r * r).sqrt();
            let b = (u(hi) - u(lo)) / (hi - lo);
            lower.push(lo);
            at_lower.push(u(lo));
            slope.push(b);
            g_lower.push(u(lo) + self.a1.abs() * lo);
            g_slope.push(b + self.a1.abs());
        }
        Ok(InUseResult {
            alpha_squared,
            beta_squared,
            lower,
            expanded_at_lower: at_lower,
            slope,
            global_at_lower: g_lower,
            global_slope: g_slope,
        })
    }
}

impl InUseResult {
    /// The interval that contains the reading, Max_{i−1} < R ≤ Max_i (the
    /// last one beyond Max).
    pub fn interval(&self, reading: f64) -> usize {
        (0..self.lower.len()).rev().find(|&i| reading > self.lower[i]).unwrap_or(0)
    }

    /// U(W) = k√(α_i² + β²R²) (7.5.1-2b) for coverage factor `k`.
    pub fn expanded(&self, reading: f64, k: f64) -> f64 {
        let i = self.interval(reading);
        k * (self.alpha_squared[i] + self.beta_squared * reading * reading).sqrt()
    }

    /// The linear form of U(W) (7.5.2-3d, -3f).
    pub fn expanded_linear(&self, reading: f64) -> f64 {
        let i = self.interval(reading);
        self.expanded_at_lower[i] + self.slope[i] * (reading - self.lower[i])
    }

    /// The global uncertainty in its linear form (7.5.2-3e, G-3).
    pub fn global(&self, reading: f64) -> f64 {
        let i = self.interval(reading);
        self.global_at_lower[i] + self.global_slope[i] * (reading - self.lower[i])
    }

    /// Minimum weight R_min,SF: the smallest reading with
    /// U_gl(W)/R ≤ Req/SF (G-5 to G-9), searched interval by interval
    /// (the first interval gives G-9, α_gl·SF/(Req − β_gl·SF)). None if no
    /// reading up to the last Max qualifies.
    pub fn minimum_weight(&self, requirement: f64, safety_factor: f64, last_max: f64) -> Option<f64> {
        let target = requirement / safety_factor;
        for i in 0..self.lower.len() {
            let lo = self.lower[i];
            let hi = if i + 1 < self.lower.len() { self.lower[i + 1] } else { last_max };
            let intercept = self.global_at_lower[i] - self.global_slope[i] * lo;
            let denom = target - self.global_slope[i];
            if denom > 0.0 {
                let r = intercept / denom;
                if (lo..=hi).contains(&r) {
                    return Some(r);
                }
            }
        }
        None
    }
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

    #[test]
    fn weighing_result_of_example_h1() {
        // cg-18 H1.4/A: 220 g / 0.1 mg, s = 0.000114 g, K_T = 1.5·10⁻⁶/K, ΔT = 3 K.
        let tare =
            relative_tare(&[(0.0, 0.0), (50.0004, 0.0004), (100.0006, 0.0007), (150.0009, 0.0010), (220.0014, 0.0013)])
                .unwrap();
        close(tare, 1.072e-6, 5e-10);
        close(relative_temperature(1.5e-6, 3.0), 1.299e-6, 5e-10);
        close(relative_buoyancy(3.0, 1.2, 8000.0), 1.636e-6, 5e-10);
        close(relative_eccentricity(0.0002, 100.0), 1.155e-6, 5e-10);
        let w = WeighingInUse {
            intervals: vec![UseInterval { max: 220.0, d: 0.0001, s: 0.000114 }],
            d0: 0.0001,
            relative: vec![1.2416e-6, 1.299e-6, 1.636e-6, tare, 1.155e-6],
            a1: 6.709e-6,
            k: 2.0,
        };
        let r = w.evaluate().unwrap();
        close(r.alpha_squared[0], 1.467e-8, 1e-11);
        close(r.beta_squared, 8.390e-12, 5e-15);
        close(r.expanded_at_lower[0], 2.422e-4, 1e-7);
        close(r.slope[0], 4.796e-6, 2e-9);
        close(r.global_slope[0], 1.150e-5, 5e-9);
        close(r.minimum_weight(0.01, 3.0, 220.0).unwrap(), 0.0729, 5e-5);
        close(r.expanded_linear(220.0), r.expanded(220.0, 2.0), 1e-15);
    }

    #[test]
    fn multi_interval_linear_forms_are_continuous() {
        let w = WeighingInUse {
            intervals: vec![
                UseInterval { max: 12000.0, d: 2.0, s: 1.095 },
                UseInterval { max: 30000.0, d: 5.0, s: 2.739 },
                UseInterval { max: 60000.0, d: 10.0, s: 2.739 },
            ],
            d0: 2.0,
            relative: vec![(4.589e-8f64).sqrt()],
            a1: -1.717e-4,
            k: 2.0,
        };
        let r = w.evaluate().unwrap();
        // H2.4/A: α² = 1.867, 9.917, 16.167 g²; slopes 2.574, 3.434, 3.923 (×10⁻⁴).
        close(r.alpha_squared[1], 9.917, 2e-3);
        close(r.slope[1], 3.434e-4, 2e-7);
        close(r.slope[2], 3.923e-4, 3e-7);
        // The global form is U(W) + |a₁|R at the lower limit of each interval.
        let u2 = 2.0 * (r.alpha_squared[1] + r.beta_squared * 12000.0f64.powi(2)).sqrt();
        close(r.global_at_lower[1], u2 + 1.717e-4 * 12000.0, 1e-9);
        close(r.minimum_weight(0.01, 2.0, 60000.0).unwrap(), 598.0, 0.5);
        assert_eq!(r.interval(12000.0), 0);
        assert_eq!(r.interval(12000.5), 1);
        assert_eq!(r.interval(0.0), 0);
    }
}
