//! Calibration with uncertainties in both the analyte contents and the
//! responses: generalized (weighted total) least squares [ISO 6143:2025,
//! 6.1–6.3 and Annex A].
//!
//! The analysis function x = G(y; b) is fitted to calibration points
//! (xᵢ, yᵢ) with standard uncertainties u(xᵢ), u(yᵢ) by minimising
//! S = Σ [(G(ŷᵢ) − xᵢ)²/u²(xᵢ) + (ŷᵢ − yᵢ)²/u²(yᵢ)] over the parameters b
//! and the adjusted responses ŷᵢ (Formula A.3), with a Levenberg–Marquardt
//! iteration. The covariance matrix of b is obtained by propagating the
//! input uncertainties (and covariances of the contents) through the fit
//! (Formulae A.6, A.7), with the derivatives ∂b/∂xᵢ, ∂b/∂yᵢ from the
//! implicit-function theorem at the minimum.

use crate::error::UncError;

/// Type of analysis function, ISO 6143 Formulae (1)–(5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalysisFunction {
    /// x = b₀ + b₁y + … + b_d y^d, d = 1, 2 or 3.
    Polynomial(usize),
    /// x = b₀ + b₁ y^(1 + b₂).
    Power,
    /// x = b₀ + b₁ e^(b₂ y).
    Exponential,
}

impl AnalysisFunction {
    pub fn parameters(&self) -> usize {
        match self {
            AnalysisFunction::Polynomial(d) => d + 1,
            AnalysisFunction::Power | AnalysisFunction::Exponential => 3,
        }
    }

    /// G(y; b).
    pub fn eval(&self, y: f64, b: &[f64]) -> f64 {
        match self {
            AnalysisFunction::Polynomial(_) => b.iter().rev().fold(0.0, |acc, c| acc * y + c),
            AnalysisFunction::Power => b[0] + b[1] * y.powf(1.0 + b[2]),
            AnalysisFunction::Exponential => b[0] + b[1] * (b[2] * y).exp(),
        }
    }

    /// ∂G/∂y.
    pub fn d_dy(&self, y: f64, b: &[f64]) -> f64 {
        match self {
            AnalysisFunction::Polynomial(_) => {
                b.iter().enumerate().skip(1).map(|(j, c)| j as f64 * c * y.powi(j as i32 - 1)).sum()
            }
            AnalysisFunction::Power => b[1] * (1.0 + b[2]) * y.powf(b[2]),
            AnalysisFunction::Exponential => b[1] * b[2] * (b[2] * y).exp(),
        }
    }

    /// Second derivatives of G: (∂²G/∂b_j∂b_l, ∂²G/∂b_j∂y, ∂²G/∂y²).
    pub fn second_derivatives(&self, y: f64, b: &[f64]) -> (Vec<Vec<f64>>, Vec<f64>, f64) {
        let m = self.parameters();
        let mut bb = vec![vec![0.0; m]; m];
        match self {
            AnalysisFunction::Polynomial(_) => {
                let by = (0..m).map(|j| if j == 0 { 0.0 } else { j as f64 * y.powi(j as i32 - 1) }).collect();
                let yy =
                    b.iter().enumerate().skip(2).map(|(j, c)| (j * (j - 1)) as f64 * c * y.powi(j as i32 - 2)).sum();
                (bb, by, yy)
            }
            AnalysisFunction::Power => {
                let (p, l, q) = (y.powf(1.0 + b[2]), y.ln(), y.powf(b[2]));
                bb[1][2] = p * l;
                bb[2][1] = p * l;
                bb[2][2] = b[1] * p * l * l;
                let by = vec![0.0, (1.0 + b[2]) * q, b[1] * q * (1.0 + (1.0 + b[2]) * l)];
                (bb, by, b[1] * (1.0 + b[2]) * b[2] * y.powf(b[2] - 1.0))
            }
            AnalysisFunction::Exponential => {
                let e = (b[2] * y).exp();
                bb[1][2] = y * e;
                bb[2][1] = y * e;
                bb[2][2] = b[1] * y * y * e;
                let by = vec![0.0, b[2] * e, b[1] * e * (1.0 + b[2] * y)];
                (bb, by, b[1] * b[2] * b[2] * e)
            }
        }
    }

    /// ∂G/∂b_j.
    pub fn d_db(&self, y: f64, b: &[f64]) -> Vec<f64> {
        match self {
            AnalysisFunction::Polynomial(d) => (0..=*d).map(|j| y.powi(j as i32)).collect(),
            AnalysisFunction::Power => {
                let p = y.powf(1.0 + b[2]);
                vec![1.0, p, b[1] * p * y.ln()]
            }
            AnalysisFunction::Exponential => {
                let e = (b[2] * y).exp();
                vec![1.0, e, b[1] * y * e]
            }
        }
    }
}

/// Result of a generalized least-squares calibration.
#[derive(Clone, Debug)]
pub struct GlsFit {
    pub function: AnalysisFunction,
    pub params: Vec<f64>,
    /// Covariance matrix of the parameters.
    pub covariance: Vec<Vec<f64>>,
    /// Adjusted calibration points (x̂ᵢ, ŷᵢ).
    pub adjusted: Vec<(f64, f64)>,
    /// Weighted deviations ((xᵢ − x̂ᵢ)/u(xᵢ), (yᵢ − ŷᵢ)/u(yᵢ)), the sign
    /// convention of ISO 6143 Annex B.
    pub weighted_deviations: Vec<(f64, f64)>,
    /// Residual sum of squared weighted deviations S_res (A.1).
    pub s_res: f64,
    /// Goodness of fit Γ: largest absolute weighted deviation (6.2.2).
    pub gamma: f64,
    /// n − (N + 1).
    pub dof: usize,
}

/// Calibration data: contents, responses and their uncertainties; optional
/// covariances u(xᵢ, xₕ) of the contents (dilution series, common standards).
#[derive(Clone, Debug)]
pub struct CalibrationData {
    pub x: Vec<f64>,
    pub ux: Vec<f64>,
    pub y: Vec<f64>,
    pub uy: Vec<f64>,
    pub cov_x: Vec<(usize, usize, f64)>,
}

fn solve_linear(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for c in 0..n {
        let p = (c..n).max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))?;
        if a[p][c] == 0.0 || !a[p][c].is_finite() {
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
                b[r] -= f * b[c];
            }
        }
    }
    let mut x = vec![0.0; n];
    for r in (0..n).rev() {
        let s: f64 = (r + 1..n).map(|k| a[r][k] * x[k]).sum();
        x[r] = (b[r] - s) / a[r][r];
    }
    Some(x)
}

/// Weighted least squares of x on polynomial terms of y (starting values).
fn polynomial_start(d: usize, x: &[f64], ux: &[f64], y: &[f64]) -> Option<Vec<f64>> {
    let m = d + 1;
    let mut a = vec![vec![0.0; m]; m];
    let mut r = vec![0.0; m];
    for i in 0..x.len() {
        let w = 1.0 / (ux[i] * ux[i]);
        let t: Vec<f64> = (0..m).map(|j| y[i].powi(j as i32)).collect();
        for j in 0..m {
            r[j] += w * t[j] * x[i];
            for k in 0..m {
                a[j][k] += w * t[j] * t[k];
            }
        }
    }
    solve_linear(a, r)
}

fn objective(f: AnalysisFunction, d: &CalibrationData, b: &[f64], yh: &[f64]) -> f64 {
    (0..d.x.len()).map(|i| ((f.eval(yh[i], b) - d.x[i]) / d.ux[i]).powi(2) + ((yh[i] - d.y[i]) / d.uy[i]).powi(2)).sum()
}

/// Levenberg–Marquardt minimisation of S over (b, ŷ).
fn minimise(
    f: AnalysisFunction,
    d: &CalibrationData,
    b0: &[f64],
    y0: &[f64],
) -> Result<(Vec<f64>, Vec<f64>, f64), UncError> {
    let n = d.x.len();
    let m = f.parameters();
    let mut b = b0.to_vec();
    let mut yh = y0.to_vec();
    let mut s = objective(f, d, &b, &yh);
    if !s.is_finite() {
        return Err(UncError::InvalidOption(
            "calibration: the starting values give a non-finite sum of squares".into(),
        ));
    }
    let mut lambda = 1e-3;
    let k = m + n;
    for _ in 0..2000 {
        // Residuals r and Jacobian J (2n × k).
        let mut jtj = vec![vec![0.0; k]; k];
        let mut jtr = vec![0.0; k];
        for i in 0..n {
            let g = f.d_db(yh[i], &b);
            let gy = f.d_dy(yh[i], &b);
            let r1 = (f.eval(yh[i], &b) - d.x[i]) / d.ux[i];
            let r2 = (yh[i] - d.y[i]) / d.uy[i];
            let mut row1 = vec![0.0; k];
            for j in 0..m {
                row1[j] = g[j] / d.ux[i];
            }
            row1[m + i] = gy / d.ux[i];
            for a in 0..k {
                if row1[a] != 0.0 {
                    jtr[a] += row1[a] * r1;
                    for c in 0..k {
                        jtj[a][c] += row1[a] * row1[c];
                    }
                }
            }
            jtj[m + i][m + i] += 1.0 / (d.uy[i] * d.uy[i]);
            jtr[m + i] += r2 / d.uy[i];
        }
        let mut improved = false;
        for _ in 0..60 {
            let mut a = jtj.clone();
            for (j, row) in a.iter_mut().enumerate() {
                row[j] += lambda * jtj[j][j].max(1e-300);
            }
            let Some(step) = solve_linear(a, jtr.iter().map(|v| -v).collect()) else {
                lambda *= 10.0;
                continue;
            };
            let bn: Vec<f64> = (0..m).map(|j| b[j] + step[j]).collect();
            let yn: Vec<f64> = (0..n).map(|i| yh[i] + step[m + i]).collect();
            let sn = objective(f, d, &bn, &yn);
            if sn.is_finite() && sn <= s {
                let small = (0..m).all(|j| step[j].abs() <= 1e-14 * bn[j].abs().max(1e-300))
                    && (0..n).all(|i| step[m + i].abs() <= 1e-14 * d.uy[i]);
                let ds = s - sn;
                b = bn;
                yh = yn;
                s = sn;
                lambda = (lambda * 0.3).max(1e-15);
                improved = true;
                if small || ds <= 1e-16 * s.max(1e-300) {
                    return Ok((b, yh, s));
                }
                break;
            }
            lambda *= 10.0;
        }
        if !improved {
            return Ok((b, yh, s));
        }
    }
    Ok((b, yh, s))
}

fn starting_values(f: AnalysisFunction, d: &CalibrationData) -> Result<Vec<f64>, UncError> {
    let bad = || UncError::InvalidOption("calibration: cannot find starting values for the analysis function".into());
    Ok(match f {
        AnalysisFunction::Polynomial(deg) => polynomial_start(deg, &d.x, &d.ux, &d.y).ok_or_else(bad)?,
        AnalysisFunction::Power => {
            let l = polynomial_start(1, &d.x, &d.ux, &d.y).ok_or_else(bad)?;
            vec![l[0], l[1], 0.0]
        }
        AnalysisFunction::Exponential => {
            // x ≈ (b₀ + b₁) + b₁b₂y + b₁b₂²y²/2 for small b₂y.
            let q = polynomial_start(2, &d.x, &d.ux, &d.y).ok_or_else(bad)?;
            if q[2] == 0.0 || q[1] == 0.0 {
                return Err(bad());
            }
            let b2 = 2.0 * q[2] / q[1];
            let b1 = q[1] / b2;
            vec![q[0] - b1, b1, b2]
        }
    })
}

/// Generalized least-squares calibration [ISO 6143:2025 A.1, A.2].
pub fn gls_fit(f: AnalysisFunction, d: &CalibrationData) -> Result<GlsFit, UncError> {
    let n = d.x.len();
    let m = f.parameters();
    if let AnalysisFunction::Polynomial(deg) = f {
        if !(1..=3).contains(&deg) {
            return Err(UncError::InvalidOption("calibration: polynomial degree 1, 2 or 3".into()));
        }
    }
    if d.ux.len() != n || d.y.len() != n || d.uy.len() != n {
        return Err(UncError::InvalidOption("calibration: x, u(x), y and u(y) need the same length".into()));
    }
    if n < m {
        return Err(UncError::InvalidOption(format!("calibration: at least {m} calibration points are needed")));
    }
    if d.x.iter().chain(&d.y).any(|v| !v.is_finite()) || d.ux.iter().chain(&d.uy).any(|u| !(*u > 0.0 && u.is_finite()))
    {
        return Err(UncError::InvalidOption("calibration: finite data and positive uncertainties required".into()));
    }
    if d.cov_x.iter().any(|&(i, h, _)| i >= n || h >= n || i == h) {
        return Err(UncError::InvalidOption("calibration: covariance indices out of range".into()));
    }
    let start = starting_values(f, d)?;
    let (b, yh, s) = minimise(f, d, &start, &d.y)?;
    // ∂b/∂z for z = x₁…xₙ, y₁…yₙ (A.2). At the minimum ∇S(θ; z) = 0 for
    // θ = (b, ŷ), so dθ/dz = −H⁻¹ ∂(∇S)/∂z with H the full Hessian of S: the
    // exact derivative that the numerical differentiation of the refit in A.2
    // approximates, without its dependence on the step and the stopping rule.
    let k = m + n;
    let mut hess = vec![vec![0.0; k]; k];
    for i in 0..n {
        let g = f.d_db(yh[i], &b);
        let mut row = g.clone();
        row.push(f.d_dy(yh[i], &b));
        let (gbb, gby, gyy) = f.second_derivatives(yh[i], &b);
        let r = (f.eval(yh[i], &b) - d.x[i]) / d.ux[i];
        let w = 1.0 / (d.ux[i] * d.ux[i]);
        let idx = |a: usize| if a < m { a } else { m + i };
        for a in 0..=m {
            for c in 0..=m {
                let second = match (a < m, c < m) {
                    (true, true) => gbb[a][c],
                    (true, false) => gby[a],
                    (false, true) => gby[c],
                    (false, false) => gyy,
                };
                hess[idx(a)][idx(c)] += 2.0 * (w * row[a] * row[c] + r / d.ux[i] * second);
            }
        }
        hess[m + i][m + i] += 2.0 / (d.uy[i] * d.uy[i]);
    }
    let mut grads: Vec<Vec<f64>> = Vec::with_capacity(2 * n);
    for z in 0..2 * n {
        let mut rhs = vec![0.0; k];
        if z < n {
            // ∂(∇S)/∂xᵢ = −(2/u²(xᵢ)) ∇θ G(ŷᵢ).
            let i = z;
            let w = 2.0 / (d.ux[i] * d.ux[i]);
            for (j, gj) in f.d_db(yh[i], &b).iter().enumerate() {
                rhs[j] = w * gj;
            }
            rhs[m + i] = w * f.d_dy(yh[i], &b);
        } else {
            // ∂(∇S)/∂yᵢ = −(2/u²(yᵢ)) e(ŷᵢ).
            let i = z - n;
            rhs[m + i] = 2.0 / (d.uy[i] * d.uy[i]);
        }
        let dtheta = solve_linear(hess.clone(), rhs).ok_or_else(|| {
            UncError::InvalidOption("calibration: singular normal matrix; the parameters are not identifiable".into())
        })?;
        grads.push(dtheta[..m].to_vec());
    }
    let mut cov = vec![vec![0.0; m]; m];
    for k in 0..2 * n {
        let var = if k < n { d.ux[k] * d.ux[k] } else { d.uy[k - n] * d.uy[k - n] };
        for j in 0..m {
            for l in 0..m {
                cov[j][l] += grads[k][j] * grads[k][l] * var;
            }
        }
    }
    for &(i, h, c) in &d.cov_x {
        for j in 0..m {
            for l in 0..m {
                cov[j][l] += (grads[i][j] * grads[h][l] + grads[h][j] * grads[i][l]) * c;
            }
        }
    }
    let adjusted: Vec<(f64, f64)> = (0..n).map(|i| (f.eval(yh[i], &b), yh[i])).collect();
    let weighted: Vec<(f64, f64)> =
        (0..n).map(|i| ((d.x[i] - adjusted[i].0) / d.ux[i], (d.y[i] - adjusted[i].1) / d.uy[i])).collect();
    let gamma = weighted.iter().map(|w| w.0.abs().max(w.1.abs())).fold(0.0, f64::max);
    Ok(GlsFit {
        function: f,
        params: b,
        covariance: cov,
        adjusted,
        weighted_deviations: weighted,
        s_res: s,
        gamma,
        dof: n - m,
    })
}

impl GlsFit {
    /// Content x = G(y) of a mixture with response y and its standard
    /// uncertainty, ISO 6143 6.3 step K (Formula 15).
    pub fn predict(&self, y: f64, uy: f64) -> (f64, f64) {
        let b = &self.params;
        let g = self.function.d_db(y, b);
        let gy = self.function.d_dy(y, b);
        let mut v = gy * gy * uy * uy;
        for j in 0..g.len() {
            for l in 0..g.len() {
                v += g[j] * g[l] * self.covariance[j][l];
            }
        }
        (self.function.eval(y, b), v.max(0.0).sqrt())
    }

    /// Covariance of the contents assigned to two mixtures with responses y₁
    /// and y₂ from the same calibration (through the parameters only).
    pub fn prediction_covariance(&self, y1: f64, y2: f64) -> f64 {
        let g1 = self.function.d_db(y1, &self.params);
        let g2 = self.function.d_db(y2, &self.params);
        let mut v = 0.0;
        for j in 0..g1.len() {
            for l in 0..g2.len() {
                v += g1[j] * g2[l] * self.covariance[j][l];
            }
        }
        v
    }
}

/// Drift control with one bracketing reference gas mixture [ISO 6143:2025
/// 6.2.4].
#[derive(Clone, Debug, PartialEq)]
pub struct DriftTest {
    /// |ȳ(before) − ȳ(calib)|, |ȳ(calib) − ȳ(after)|, |ȳ(before) − ȳ(after)|.
    pub differences: [f64; 3],
    pub critical: [f64; 3],
    pub passed: bool,
}

/// Drift test of 6.2.4: the means of n measurements of the reference mixture
/// before and after the sample are compared with the mean y(calib) of the
/// n_calib calibration measurements (standard deviation of the mean u) and
/// with each other. With equal single-measurement standard deviations the
/// differences have standard deviations u√(1 + n_calib/n), u√(1 + n_calib/n)
/// and u√(2n_calib/n); the critical values are twice these, 2√2 u for
/// n = n_calib = 10 as stated in the Note to 6.2.4. Formula (12) of the
/// standard reads 2√(1 + 20/n) u, which gives 2√3 u for n = 10 and
/// contradicts that Note; the derivation above is used (research log F41).
pub fn drift_test(
    y_calib: f64,
    u_calib: f64,
    before: f64,
    after: f64,
    n: usize,
    n_calib: usize,
) -> Result<DriftTest, UncError> {
    if n == 0 || n_calib == 0 || u_calib.is_nan() || u_calib <= 0.0 {
        return Err(UncError::InvalidOption("drift test: n, n_calib and u(y_calib) must be positive".into()));
    }
    let r = n_calib as f64 / n as f64;
    let differences = [(before - y_calib).abs(), (y_calib - after).abs(), (before - after).abs()];
    let critical =
        [2.0 * (1.0 + r).sqrt() * u_calib, 2.0 * (1.0 + r).sqrt() * u_calib, 2.0 * (2.0 * r).sqrt() * u_calib];
    let passed = differences.iter().zip(&critical).all(|(d, c)| d <= c);
    Ok(DriftTest { differences, critical, passed })
}

/// Applicability to a mismatching mixture, Formula (13):
/// |x_obs − x_ref| ≤ 2√(u²(x_obs) + u²(x_ref)). Returns the difference, the
/// limit and whether the condition holds.
pub fn mismatch_check(x_obs: f64, u_obs: f64, x_ref: f64, u_ref: f64) -> (f64, f64, bool) {
    let d = (x_obs - x_ref).abs();
    let limit = 2.0 * (u_obs * u_obs + u_ref * u_ref).sqrt();
    (d, limit, d <= limit)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(a: f64, b: f64, t: f64) -> bool {
        ((a - b) / b).abs() < t
    }

    #[test]
    fn iso_6143_example_1_linear() {
        let d = CalibrationData {
            x: vec![4.5, 18.75, 50.0],
            ux: vec![0.045, 0.1875, 0.5],
            y: vec![0.1969, 0.7874, 2.0228],
            uy: vec![0.003938, 0.015748, 0.040456],
            cov_x: vec![],
        };
        let r = gls_fit(AnalysisFunction::Polynomial(1), &d).unwrap();
        assert!(rel(r.params[0], -3.5747e-1, 5e-5) && rel(r.params[1], 2.4612e1, 5e-5), "{:?}", r.params);
        assert!(rel(r.covariance[0][0].sqrt(), 1.5716e-1, 5e-4) && rel(r.covariance[1][1].sqrt(), 4.8048e-1, 5e-4));
        assert!(rel(r.covariance[0][1], -5.6921e-2, 5e-4));
        assert!(rel(r.s_res, 0.6743, 5e-4) && rel(r.gamma, 0.568, 1e-3));
        for (e, w) in [(4.52e-2, -9.73e-2), (-2.75e-1, 5.68e-1), (2.31e-1, -4.60e-1)].iter().zip(&r.weighted_deviations)
        {
            assert!(rel(w.0, e.0, 5e-3) && rel(w.1, e.1, 5e-3), "{w:?}");
        }
        let expect = [
            (0.2580, 0.005160, 5.9923, 1.6377e-1),
            (0.6000, 0.012, 1.4409e1, 3.5599e-1),
            (1.8, 0.036, 4.3943e1, 1.1631),
        ];
        for &(y, uy, x, ux) in &expect {
            let (px, pu) = r.predict(y, uy);
            assert!(rel(px, x, 5e-5) && rel(pu, ux, 5e-4), "y = {y}: {px} {pu}");
        }
        assert!(rel(r.prediction_covariance(0.258, 0.6), 1.16e-2, 5e-3));
        assert!(rel(r.prediction_covariance(0.6, 1.8), 1.37e-1, 5e-3));
    }
    fn example_2() -> CalibrationData {
        CalibrationData {
            x: vec![1.5e-3, 0.1888, 1.990, 3.796, 5.677, 7.118, 9.210, 10.90],
            ux: vec![9e-4, 4.5e-4, 4e-3, 3.9e-2, 1.25e-2, 1.25e-2, 2.0e-2, 2.5e-2],
            y: vec![60.0, 7786.0, 81700.0, 156200.0, 233300.0, 293000.0, 380600.0, 449700.0],
            uy: vec![35.0, 135.7, 36.7, 223.2, 137.2, 245.5, 125.1, 321.8],
            cov_x: vec![],
        }
    }

    #[test]
    fn iso_6143_example_2_linear_quadratic_covariance() {
        let d = example_2();
        let r = gls_fit(AnalysisFunction::Polynomial(1), &d).unwrap();
        assert!(rel(r.params[0], 3.9810e-4, 1e-3) && rel(r.params[1], 2.4285e-5, 5e-5), "{:?}", r.params);
        assert!(rel(r.covariance[0][0].sqrt(), 1.1458e-3, 1e-3) && rel(r.covariance[1][1].sqrt(), 2.4159e-8, 1e-3));
        assert!(rel(r.covariance[0][1], -7.2739e-12, 1e-3));
        assert!(rel(r.s_res, 6.0455, 1e-3) && rel(r.gamma, 1.6266, 1e-3), "{} {}", r.s_res, r.gamma);
        let (x, u) = r.predict(70000.0, 40.0);
        assert!(rel(x, 1.7004, 1e-4) && rel(u, 2.0244e-3, 1e-3), "{x} {u}");
        let (x, u) = r.predict(370000.0, 200.0);
        assert!(rel(x, 8.9859, 1e-4) && rel(u, 9.9712e-3, 1e-3), "{x} {u}");

        let q = gls_fit(AnalysisFunction::Polynomial(2), &d).unwrap();
        for (a, e) in q.params.iter().zip([-1.3111e-4, 2.4401e-5, -4.0865e-13]) {
            assert!(rel(*a, e, 2e-3), "{:?}", q.params);
        }
        for (j, e) in [1.175e-3, 5.901e-8, 1.895e-13].iter().enumerate() {
            assert!(rel(q.covariance[j][j].sqrt(), *e, 2e-3), "{:?}", q.covariance);
        }
        assert!(rel(q.covariance[0][1], -2.057e-11, 2e-3) && rel(q.covariance[0][2], 4.667e-17, 2e-3));
        assert!(rel(q.covariance[1][2], -1.020e-20, 2e-3));
        assert!(rel(q.s_res, 1.3964, 1e-3) && rel(q.gamma, 0.8664, 1e-3), "{} {}", q.s_res, q.gamma);
        let (x, u) = q.predict(70000.0, 40.0);
        assert!(rel(x, 1.7059, 1e-4) && rel(u, 3.2911e-3, 1e-3), "{x} {u}");
        let (x, u) = q.predict(370000.0, 200.0);
        assert!(rel(x, 8.9723, 1e-4) && rel(u, 1.1761e-2, 1e-3), "{x} {u}");

        let mut c = example_2();
        c.cov_x = vec![(3, 6, 0.00016), (4, 7, 0.0001)];
        let r = gls_fit(AnalysisFunction::Polynomial(1), &c).unwrap();
        assert!(rel(r.covariance[0][0].sqrt(), 1.1463e-3, 1e-3) && rel(r.covariance[1][1].sqrt(), 2.5497e-8, 1e-3));
        assert!(rel(r.covariance[0][1], -7.6010e-12, 1e-3), "{:?}", r.covariance);
        let (_, u) = r.predict(70000.0, 40.0);
        assert!(rel(u, 2.0926e-3, 1e-3), "{u}");
        let (_, u) = r.predict(370000.0, 200.0);
        assert!(rel(u, 1.0406e-2, 1e-3), "{u}");
    }

    fn example_3() -> CalibrationData {
        CalibrationData {
            x: vec![1.0006, 1.0010, 1.9995, 3.0018, 3.9982, 4.0043, 4.9981, 5.9961, 7.9974, 7.9995, 9.9980, 10.0060],
            ux: vec![0.00134, 0.0011, 0.0032, 0.0034, 0.0058, 0.0058, 0.0058, 0.0078, 0.0084, 0.010, 0.012, 0.0104],
            y: vec![
                963.7988, 966.2585, 1912.5692, 2846.9306, 3754.9386, 3764.5905, 4647.1511, 5529.9991, 7246.0082,
                7240.4767, 8884.6955, 8902.6916,
            ],
            uy: vec![14.0, 12.5, 16.8, 11.6, 10.8, 12.1, 14.4, 12.1, 11.9, 10.7, 10.1, 14.1],
            cov_x: vec![],
        }
    }

    #[test]
    fn iso_6143_example_3_power_exponential() {
        let d = example_3();
        let l = gls_fit(AnalysisFunction::Polynomial(1), &d).unwrap();
        assert!(rel(l.s_res, 272.6392, 1e-3) && rel(l.gamma, 6.8352, 1e-3), "{} {}", l.s_res, l.gamma);

        let p = gls_fit(AnalysisFunction::Power, &d).unwrap();
        for (a, e) in p.params.iter().zip([0.12130, 5.1211e-4, 8.4990e-2]) {
            assert!(rel(*a, e, 2e-3), "{:?}", p.params);
        }
        for (j, e) in [1.8095e-2, 2.4143e-5, 5.0692e-3].iter().enumerate() {
            assert!(rel(p.covariance[j][j].sqrt(), *e, 3e-3), "{:?}", p.covariance);
        }
        assert!(rel(p.covariance[0][1], -3.9719e-7, 5e-3) && rel(p.covariance[0][2], 8.2385e-5, 5e-3));
        assert!(rel(p.covariance[1][2], -1.2233e-7, 5e-3));
        assert!(rel(p.s_res, 8.3804, 1e-3) && rel(p.gamma, 1.1594, 1e-3), "{} {}", p.s_res, p.gamma);
        let (x, u) = p.predict(4950.6, 11.0);
        assert!(rel(x, 5.3456, 1e-4) && rel(u, 1.4169e-2, 2e-3), "{x} {u}");

        let e = gls_fit(AnalysisFunction::Exponential, &d).unwrap();
        for (a, v) in e.params.iter().zip([-47.963, 47.969, 2.1283e-5]) {
            assert!(rel(*a, v, 2e-3), "{:?}", e.params);
        }
        assert!(rel(e.s_res, 0.6572, 2e-3) && rel(e.gamma, 0.3529, 2e-3), "{} {}", e.s_res, e.gamma);
        // The standard's u(b) for this fit (8.1404e-2, 7.3965e-2, 3.7068e-11)
        // come from B_LEAST, which it notes is wrong for the exponential
        // function. Reference: ODRPACK (scipy.odr) cov_beta for the same data,
        // 3.1816, 3.1705, 1.2739e-6 and u(x) = 1.4246e-2.
        for (j, v) in [3.1816, 3.1705, 1.2739e-6].iter().enumerate() {
            assert!(rel(e.covariance[j][j].sqrt(), *v, 2e-3), "{:?}", e.covariance);
        }
        let (x, u) = e.predict(4950.6, 11.0);
        assert!(rel(x, 5.3357, 1e-4) && rel(u, 1.4246e-2, 2e-3), "{x} {u}");
    }

    #[test]
    fn drift_critical_values() {
        let t = drift_test(100.0, 1.0, 101.0, 102.9, 10, 10).unwrap();
        assert!((t.critical[0] - 2.0 * 2f64.sqrt()).abs() < 1e-12 && (t.critical[2] - 2.0 * 2f64.sqrt()).abs() < 1e-12);
        assert!(!t.passed && t.differences[1] > t.critical[1]);
        let t = drift_test(100.0, 1.0, 101.0, 101.5, 5, 10).unwrap();
        assert!((t.critical[0] - 2.0 * 3f64.sqrt()).abs() < 1e-12 && (t.critical[2] - 4.0).abs() < 1e-12 && t.passed);
        assert!(mismatch_check(5.0, 0.03, 5.05, 0.02).2 && !mismatch_check(5.0, 0.01, 5.05, 0.01).2);
    }
}
