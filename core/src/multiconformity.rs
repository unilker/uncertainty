//! Conformity assessment of a multicomponent item: total risks of false
//! decisions when several correlated properties are assessed together
//! (IUPAC project 2016-007-1-500; Kuselman, Pennecchi et al.; EMPIR EMUE
//! compendium 2021, Example E2.1).
//!
//! The prior PDF g₀(c) of the true values and the measuring-system PDF
//! h(c_m | c) are multivariate normal. The posterior for a vector of test
//! results is then normal as well (EMUE (E2.1.1)–(E2.1.2)), and the total
//! specific risks are box probabilities of that posterior. They are computed
//! by the separation-of-variables method of Genz (1992) with antithetic
//! uniform draws. The total global risks are Monte Carlo frequencies.

use crate::error::UncError;
use crate::mcm::cholesky;
use crate::rng::Rng;
use crate::special::{normal_cdf, normal_quantile};

fn invalid(msg: &str) -> UncError {
    UncError::InvalidOption(format!("multicomponent conformity: {msg}"))
}

/// Probability that X ~ N(mean, cov) lies in the box [lower, upper]
/// (Genz 1992, separation of variables), with its standard error. Infinite
/// limits are allowed.
pub fn mvn_box_probability(
    mean: &[f64],
    cov: &[Vec<f64>],
    lower: &[f64],
    upper: &[f64],
    samples: usize,
    seed: u64,
) -> Result<(f64, f64), UncError> {
    let d = mean.len();
    if d == 0 || cov.len() != d || lower.len() != d || upper.len() != d || cov.iter().any(|r| r.len() != d) {
        return Err(invalid("dimensions of mean, covariance and limits differ"));
    }
    if lower.iter().zip(upper).any(|(a, b)| !(a < b)) {
        return Err(invalid("every lower limit must be below its upper limit"));
    }
    let l = cholesky(cov).ok_or_else(|| invalid("the covariance matrix is not positive definite"))?;
    if (0..d).any(|i| !(l[i][i] > 0.0)) {
        return Err(invalid("the covariance matrix is singular"));
    }
    let mut rng = Rng::new(seed);
    let mut y = vec![0.0; d];
    let mut one = |w: &[f64]| -> f64 {
        let mut f = 1.0;
        for i in 0..d {
            let s: f64 = (0..i).map(|j| l[i][j] * y[j]).sum();
            let phi = |x: f64| {
                if x == f64::INFINITY {
                    1.0
                } else if x == f64::NEG_INFINITY {
                    0.0
                } else {
                    normal_cdf((x - mean[i] - s) / l[i][i])
                }
            };
            let (a, b) = (phi(lower[i]), phi(upper[i]));
            let width = (b - a).max(0.0);
            f *= width;
            if f == 0.0 {
                return 0.0;
            }
            if i + 1 < d {
                y[i] = normal_quantile((a + w[i] * width).clamp(1e-300, 1.0 - 1e-16));
            }
        }
        f
    };
    let n = samples.max(2);
    let (mut sum, mut sum2) = (0.0, 0.0);
    let mut w = vec![0.0; d];
    let mut wa = vec![0.0; d];
    for _ in 0..n {
        for i in 0..d {
            w[i] = rng.uniform();
            wa[i] = 1.0 - w[i];
        }
        let v = 0.5 * (one(&w) + one(&wa));
        sum += v;
        sum2 += v * v;
    }
    let p = sum / n as f64;
    let var = ((sum2 / n as f64 - p * p) / (n as f64 - 1.0)).max(0.0);
    Ok((p.clamp(0.0, 1.0), var.sqrt()))
}

/// A multicomponent item: multivariate normal prior of the true values and
/// a measuring system with relative standard uncertainty `u_rel` and the same
/// correlation matrix (EMUE E2.1.6.1).
#[derive(Clone, Debug, PartialEq)]
pub struct Multicomponent {
    pub prior_mean: Vec<f64>,
    pub prior_sd: Vec<f64>,
    pub correlation: Vec<Vec<f64>>,
    pub u_rel: f64,
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
}

/// Total specific risk for one vector of test results.
#[derive(Clone, Debug, PartialEq)]
pub struct SpecificTotal {
    pub posterior_mean: Vec<f64>,
    /// Probability that all true values lie within their tolerance intervals.
    pub conformance_probability: f64,
    pub standard_error: f64,
    /// All test results lie within their acceptance (= tolerance) intervals.
    pub accepted: bool,
    /// Total specific consumer's risk 1 − p_c if accepted, producer's risk
    /// p_c if rejected.
    pub risk: f64,
}

/// Total global risks of the simple-acceptance rule.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalTotal {
    /// Accepted although at least one true value is out of tolerance.
    pub consumer: f64,
    /// Rejected although all true values are within tolerance.
    pub producer: f64,
    pub trials: usize,
}

fn matmul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    (0..n).map(|i| (0..n).map(|j| (0..n).map(|k| a[i][k] * b[k][j]).sum()).collect()).collect()
}

/// Inverse of a symmetric positive definite matrix via its Cholesky factor.
fn spd_inverse(m: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = m.len();
    let l = cholesky(m)?;
    if (0..n).any(|i| !(l[i][i] > 0.0)) {
        return None;
    }
    // L⁻¹ by forward substitution, then M⁻¹ = L⁻ᵀ L⁻¹.
    let mut li = vec![vec![0.0; n]; n];
    for c in 0..n {
        for i in 0..n {
            let s: f64 = (0..i).map(|k| l[i][k] * li[k][c]).sum();
            li[i][c] = ((if i == c { 1.0 } else { 0.0 }) - s) / l[i][i];
        }
    }
    let lt: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| li[j][i]).collect()).collect();
    Some(matmul(&lt, &li))
}

impl Multicomponent {
    fn validate(&self) -> Result<usize, UncError> {
        let d = self.prior_mean.len();
        let ok_dims = d > 0
            && self.prior_sd.len() == d
            && self.lower.len() == d
            && self.upper.len() == d
            && self.correlation.len() == d
            && self.correlation.iter().all(|r| r.len() == d);
        if !ok_dims {
            return Err(invalid("give one prior mean, prior s.d., tolerance and correlation row per component"));
        }
        if self.prior_sd.iter().any(|s| !(*s > 0.0)) || !(self.u_rel > 0.0) {
            return Err(invalid("prior standard deviations and u_rel must be positive"));
        }
        for i in 0..d {
            if self.correlation[i][i] != 1.0 {
                return Err(invalid("the correlation matrix must have a unit diagonal"));
            }
            for j in 0..d {
                let r = self.correlation[i][j];
                if r != self.correlation[j][i] || !(r.abs() <= 1.0) {
                    return Err(invalid("the correlation matrix must be symmetric with |r| ≤ 1"));
                }
            }
        }
        Ok(d)
    }

    fn cov(&self, sd: &[f64]) -> Vec<Vec<f64>> {
        let d = sd.len();
        (0..d).map(|i| (0..d).map(|j| self.correlation[i][j] * sd[i] * sd[j]).collect()).collect()
    }

    /// Total specific risk for test results `cm` (EMUE (E2.1.1)–(E2.1.2),
    /// n_rep = 1); the likelihood has u_i = u_rel·c_mi.
    pub fn specific(&self, cm: &[f64], samples: usize, seed: u64) -> Result<SpecificTotal, UncError> {
        let d = self.validate()?;
        if cm.len() != d || cm.iter().any(|v| !(v.is_finite() && *v != 0.0)) {
            return Err(invalid("give one finite, non-zero test result per component"));
        }
        let sc_inv =
            spd_inverse(&self.cov(&self.prior_sd)).ok_or_else(|| invalid("prior covariance not positive definite"))?;
        let um: Vec<f64> = cm.iter().map(|v| self.u_rel * v.abs()).collect();
        let sm_inv =
            spd_inverse(&self.cov(&um)).ok_or_else(|| invalid("measurement covariance not positive definite"))?;
        let prec: Vec<Vec<f64>> = (0..d).map(|i| (0..d).map(|j| sc_inv[i][j] + sm_inv[i][j]).collect()).collect();
        let sp = spd_inverse(&prec).ok_or_else(|| invalid("posterior covariance not positive definite"))?;
        let rhs: Vec<f64> =
            (0..d).map(|i| (0..d).map(|j| sc_inv[i][j] * self.prior_mean[j] + sm_inv[i][j] * cm[j]).sum()).collect();
        let cp: Vec<f64> = (0..d).map(|i| (0..d).map(|j| sp[i][j] * rhs[j]).sum()).collect();
        let (pc, se) = mvn_box_probability(&cp, &sp, &self.lower, &self.upper, samples, seed)?;
        let accepted = (0..d).all(|i| cm[i] >= self.lower[i] && cm[i] <= self.upper[i]);
        Ok(SpecificTotal {
            posterior_mean: cp,
            conformance_probability: pc,
            standard_error: se,
            accepted,
            risk: if accepted { 1.0 - pc } else { pc },
        })
    }

    /// Total global risks by Monte Carlo: c ~ g₀, c_m ~ N(c, R∘(u uᵀ)) with
    /// u_i = u_rel·|c_i| (EMUE E2.1.6.3).
    pub fn global(&self, trials: usize, seed: u64) -> Result<GlobalTotal, UncError> {
        let d = self.validate()?;
        let lc =
            cholesky(&self.cov(&self.prior_sd)).ok_or_else(|| invalid("prior covariance not positive definite"))?;
        let lr = cholesky(&self.correlation).ok_or_else(|| invalid("correlation matrix not positive definite"))?;
        let mut rng = Rng::new(seed);
        let (mut cons, mut prod) = (0usize, 0usize);
        let (mut z, mut c, mut e) = (vec![0.0; d], vec![0.0; d], vec![0.0; d]);
        for _ in 0..trials {
            for v in z.iter_mut() {
                *v = rng.normal();
            }
            for i in 0..d {
                c[i] = self.prior_mean[i] + (0..=i).map(|j| lc[i][j] * z[j]).sum::<f64>();
            }
            for v in z.iter_mut() {
                *v = rng.normal();
            }
            for i in 0..d {
                e[i] = (0..=i).map(|j| lr[i][j] * z[j]).sum::<f64>();
            }
            let conforming = (0..d).all(|i| c[i] >= self.lower[i] && c[i] <= self.upper[i]);
            let accepted = (0..d).all(|i| {
                let m = c[i] + e[i] * self.u_rel * c[i].abs();
                m >= self.lower[i] && m <= self.upper[i]
            });
            if accepted && !conforming {
                cons += 1;
            } else if !accepted && conforming {
                prod += 1;
            }
        }
        let n = trials.max(1) as f64;
        Ok(GlobalTotal { consumer: cons as f64 / n, producer: prod as f64 / n, trials })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// EMUE compendium 2021, Example E2.1 (NyQuil tablets, four active
    /// components, tolerance 95–105 % of the labelled amount, u_rel = 2.8 %):
    /// R*_tot = 0.0029 (c_m = m), 0.0117 (c_m = T_L), 0.0002 (c_m = T_U);
    /// R_tot = 0.0018 (10⁷ trials). Values to more digits by scipy (pmvnorm
    /// equivalent) and numpy: 0.002 881, 0.011 716, 0.000 212; 0.001 82.
    #[test]
    fn emue_e21_influenza_medication() {
        let r = vec![
            vec![1.0, 0.107, 0.125, 0.177],
            vec![0.107, 1.0, 0.311, 0.404],
            vec![0.125, 0.311, 1.0, 0.539],
            vec![0.177, 0.404, 0.539, 1.0],
        ];
        let mc = Multicomponent {
            prior_mean: vec![99.18, 97.70, 99.33, 98.94],
            prior_sd: vec![1.37, 1.02, 1.05, 1.22],
            correlation: r,
            u_rel: 0.028,
            lower: vec![95.0; 4],
            upper: vec![105.0; 4],
        };
        let s = mc.specific(&mc.prior_mean.clone(), 200_000, 1).unwrap();
        assert!(s.accepted && (s.risk - 0.002881).abs() < 2e-5, "{}", s.risk);
        let s = mc.specific(&[95.0; 4], 200_000, 2).unwrap();
        assert!((s.risk - 0.011716).abs() < 5e-5, "{}", s.risk);
        let s = mc.specific(&[105.0; 4], 200_000, 3).unwrap();
        assert!((s.risk - 0.000212).abs() < 5e-6, "{}", s.risk);
        let g = mc.global(2_000_000, 4).unwrap();
        assert!((g.consumer - 0.00182).abs() < 1e-4, "{}", g.consumer);
    }

    #[test]
    fn box_probability_of_independent_normals_is_a_product() {
        let cov = vec![vec![1.0, 0.0], vec![0.0, 4.0]];
        let (p, _) = mvn_box_probability(&[0.0, 0.0], &cov, &[-1.0, -2.0], &[1.0, f64::INFINITY], 2000, 1).unwrap();
        let exact = (normal_cdf(1.0) - normal_cdf(-1.0)) * (1.0 - normal_cdf(-1.0));
        assert!((p - exact).abs() < 1e-12, "{p} {exact}");
    }
}
