//! Straight-line calibration y = b₀ + b₁x by least squares and the
//! uncertainty of a value predicted from it [EURACHEM/CITAC CG4 (2012)
//! Appendix E.4, Eq. E3.1–E3.8].
//!
//! Only uncertainty in y is treated here (ordinary or weighted least squares).
//! Calibration with uncertainties in both variables and correlated contents
//! is in [`crate::gls`] (ISO 6143:2025).

use crate::error::UncError;

/// Result of a straight-line least-squares fit.
#[derive(Clone, Debug, PartialEq)]
pub struct StraightLine {
    pub intercept: f64,
    pub slope: f64,
    /// Residual standard deviation S = √(Σw(y − ŷ)²/(n − 2)) [Eq. E3.4].
    pub residual_sd: f64,
    pub var_intercept: f64,
    pub var_slope: f64,
    pub cov_intercept_slope: f64,
    /// Pearson correlation coefficient of the (x, y) data.
    pub correlation: f64,
    pub n: usize,
    /// Weighted mean of x.
    pub mean_x: f64,
    /// Σw(x − x̄)² (S_xx for unit weights).
    pub sxx: f64,
    /// Σw (n for unit weights).
    pub sum_weights: f64,
}

impl StraightLine {
    /// Ordinary least squares fit.
    pub fn fit(x: &[f64], y: &[f64]) -> Result<Self, UncError> {
        Self::fit_weighted(x, y, &vec![1.0; x.len()])
    }

    /// Weighted least squares fit with weights w_i (proportional to 1/var(y_i)).
    pub fn fit_weighted(x: &[f64], y: &[f64], w: &[f64]) -> Result<Self, UncError> {
        let n = x.len();
        if y.len() != n || w.len() != n {
            return Err(UncError::InvalidOption("x, y and weights must have the same length".into()));
        }
        if n < 3 {
            return Err(UncError::InvalidOption("a straight-line fit with uncertainty needs n ≥ 3".into()));
        }
        if x.iter().chain(y).chain(w).any(|v| !v.is_finite()) || w.iter().any(|&v| v <= 0.0) {
            return Err(UncError::InvalidOption("data must be finite and weights positive".into()));
        }
        let sw: f64 = w.iter().sum();
        let mx = x.iter().zip(w).map(|(a, b)| a * b).sum::<f64>() / sw;
        let my = y.iter().zip(w).map(|(a, b)| a * b).sum::<f64>() / sw;
        let mut sxx = 0.0;
        let mut syy = 0.0;
        let mut sxy = 0.0;
        for i in 0..n {
            let dx = x[i] - mx;
            let dy = y[i] - my;
            sxx += w[i] * dx * dx;
            syy += w[i] * dy * dy;
            sxy += w[i] * dx * dy;
        }
        if sxx == 0.0 {
            return Err(UncError::InvalidOption("all x values are equal".into()));
        }
        let slope = sxy / sxx;
        let intercept = my - slope * mx;
        let ss_res: f64 = (0..n).map(|i| w[i] * (y[i] - intercept - slope * x[i]).powi(2)).sum();
        let s2 = ss_res / (n as f64 - 2.0);
        Ok(Self {
            intercept,
            slope,
            residual_sd: s2.sqrt(),
            var_intercept: s2 * (1.0 / sw + mx * mx / sxx),
            var_slope: s2 / sxx,
            cov_intercept_slope: -s2 * mx / sxx,
            correlation: if syy > 0.0 { sxy / (sxx * syy).sqrt() } else { 0.0 },
            n,
            mean_x: mx,
            sxx,
            sum_weights: sw,
        })
    }

    /// Degrees of freedom of the residual standard deviation, n − 2.
    pub fn dof(&self) -> f64 {
        self.n as f64 - 2.0
    }

    /// x predicted from an observed response: (y_obs − b₀)/b₁ [Eq. E3.2].
    pub fn predict_x(&self, y_obs: f64) -> f64 {
        (y_obs - self.intercept) / self.slope
    }

    /// Standard uncertainty of a predicted x from the mean of p responses,
    /// due to random variation in y [Eq. E3.4/E3.5]:
    /// u = (S/|b₁|) √(1/(p w_obs) + 1/Σw + (x − x̄)²/S_xx).
    /// `weight_obs` is the weight of one observation of the unknown (1 for
    /// ordinary least squares).
    pub fn u_predicted_x(&self, x_pred: f64, p: usize, weight_obs: f64) -> f64 {
        let s = self.residual_sd / self.slope.abs();
        s * (1.0 / (p as f64 * weight_obs) + 1.0 / self.sum_weights + (x_pred - self.mean_x).powi(2) / self.sxx).sqrt()
    }

    /// The same uncertainty from the parameter covariance [Eq. E3.3], with
    /// var(y_obs) given explicitly.
    pub fn u_predicted_x_from_covariance(&self, x_pred: f64, var_y_obs: f64) -> f64 {
        ((var_y_obs + x_pred * x_pred * self.var_slope + 2.0 * x_pred * self.cov_intercept_slope + self.var_intercept)
            / (self.slope * self.slope))
            .sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_line_and_formula_equivalence() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [3.1, 4.9, 7.2, 8.8, 11.0];
        let f = StraightLine::fit(&x, &y).unwrap();
        // Hand values: b₁ = Sxy/Sxx = 19.7/10, b₀ = 7 − 1.97·3.
        assert!((f.slope - 1.97).abs() < 1e-12);
        assert!((f.intercept - (7.0 - 1.97 * 3.0)).abs() < 1e-12);
        // Eq. E3.3 and E3.5 agree for unweighted data.
        let p = 2;
        for xp in [0.0, 2.5, 6.0] {
            let a = f.u_predicted_x(xp, p, 1.0);
            let b = f.u_predicted_x_from_covariance(xp, f.residual_sd.powi(2) / p as f64);
            assert!((a - b).abs() < 1e-14, "{a} {b}");
        }
        // Uniform weights reproduce the unweighted fit.
        let g = StraightLine::fit_weighted(&x, &y, &[4.0; 5]).unwrap();
        assert!((g.slope - f.slope).abs() < 1e-12);
        assert!((g.u_predicted_x(2.5, p, 4.0) - f.u_predicted_x(2.5, p, 1.0)).abs() < 1e-14);
        assert!(StraightLine::fit(&[1.0, 1.0, 1.0], &[1.0, 2.0, 3.0]).is_err());
    }
}
