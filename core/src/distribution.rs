//! Probability distributions assigned to input quantities (JCGM 101 6.4,
//! GUM 4.3) together with their summaries for the GUM uncertainty framework.
//!
//! Two views of every distribution are provided:
//!
//! * the *GUM view* — best estimate, standard uncertainty and degrees of
//!   freedom used by the law of propagation of uncertainty (GUM 5.1, G.4);
//! * the *MCM view* — draws from the full distribution (JCGM 101 7.3).
//!
//! The two views differ for the scaled and shifted t-distribution: following
//! GUM 4.2.3 the standard uncertainty of a mean of n indications is s/√n with
//! ν = n − 1, whereas JCGM 101 6.4.9.4 gives the standard deviation of the
//! assigned t-distribution as √((n − 1)/(n − 3))·s/√n. `gum_standard_uncertainty`
//! returns the former and `standard_deviation` the latter.

use crate::error::UncError;
use crate::rng::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum Distribution {
    /// A quantity without uncertainty (e.g. a defined constant).
    Constant { value: f64 },
    /// Gaussian N(mean, sd²) [JCGM 101 6.4.7].
    Normal { mean: f64, sd: f64 },
    /// Scaled and shifted t-distribution t_ν(mean, scale²) [JCGM 101 6.4.9].
    StudentT { mean: f64, scale: f64, dof: f64 },
    /// Rectangular R(lower, upper) [JCGM 101 6.4.2; GUM 4.3.7].
    Rectangular { lower: f64, upper: f64 },
    /// Symmetric triangular T(lower, upper) [JCGM 101 6.4.5; GUM 4.3.9].
    Triangular { lower: f64, upper: f64 },
    /// Symmetric trapezoidal Trap(lower, upper, β), β = top/base semi-width
    /// ratio [JCGM 101 6.4.4; GUM 4.3.9].
    Trapezoidal { lower: f64, upper: f64, beta: f64 },
    /// Rectangular distribution with inexactly prescribed limits
    /// ("curvilinear trapezoid") CTrap(lower, upper, d) [JCGM 101 6.4.3].
    CurvilinearTrapezoid { lower: f64, upper: f64, d: f64 },
    /// Arc sine (U-shaped) U(lower, upper) [JCGM 101 6.4.6].
    Arcsine { lower: f64, upper: f64 },
    /// Exponential distribution with the given expectation [JCGM 101 6.4.10].
    Exponential { mean: f64 },
    /// Gamma distribution with shape α and scale θ (mean αθ, variance αθ²)
    /// [JCGM 101 6.4.11].
    Gamma { shape: f64, scale: f64 },
}

impl Distribution {
    /// Gaussian distribution from an expanded uncertainty U and coverage
    /// factor k (e.g. a calibration certificate) [GUM 4.3.3]. If an effective
    /// degrees of freedom is stated, a t-distribution is assigned
    /// [JCGM 101 6.4.9.7]; otherwise a Gaussian [JCGM 101 6.4.9.8].
    pub fn from_expanded_uncertainty(estimate: f64, expanded: f64, k: f64, dof: Option<f64>) -> Self {
        let u = expanded / k;
        match dof {
            Some(nu) if nu.is_finite() => Distribution::StudentT { mean: estimate, scale: u, dof: nu },
            _ => Distribution::Normal { mean: estimate, sd: u },
        }
    }

    /// Rectangular distribution centred on `estimate` with semi-width `a`.
    pub fn rectangular_half_width(estimate: f64, a: f64) -> Self {
        Distribution::Rectangular { lower: estimate - a, upper: estimate + a }
    }

    /// Triangular distribution centred on `estimate` with semi-width `a`.
    pub fn triangular_half_width(estimate: f64, a: f64) -> Self {
        Distribution::Triangular { lower: estimate - a, upper: estimate + a }
    }

    /// Arc sine distribution centred on `estimate` with semi-width `a`.
    pub fn arcsine_half_width(estimate: f64, a: f64) -> Self {
        Distribution::Arcsine { lower: estimate - a, upper: estimate + a }
    }

    /// Checks that the parameters define a proper distribution.
    pub fn validate(&self) -> Result<(), UncError> {
        let bad = |msg: &str| Err(UncError::InvalidDistribution(msg.to_string()));
        let finite = |v: &[f64]| v.iter().all(|x| x.is_finite());
        match *self {
            Distribution::Constant { value } => {
                if !value.is_finite() {
                    return bad("constant value must be finite");
                }
            }
            Distribution::Normal { mean, sd } => {
                if !finite(&[mean, sd]) || sd < 0.0 {
                    return bad("normal: mean must be finite and sd ≥ 0");
                }
            }
            Distribution::StudentT { mean, scale, dof } => {
                if !finite(&[mean, scale]) || scale < 0.0 || !(dof > 0.0) {
                    return bad("t: mean finite, scale ≥ 0 and dof > 0 required");
                }
            }
            Distribution::Rectangular { lower, upper }
            | Distribution::Triangular { lower, upper }
            | Distribution::Arcsine { lower, upper } => {
                if !finite(&[lower, upper]) || lower > upper {
                    return bad("limits must be finite with lower ≤ upper");
                }
            }
            Distribution::Trapezoidal { lower, upper, beta } => {
                if !finite(&[lower, upper, beta]) || lower > upper || !(0.0..=1.0).contains(&beta) {
                    return bad("trapezoidal: lower ≤ upper and 0 ≤ β ≤ 1 required");
                }
            }
            Distribution::CurvilinearTrapezoid { lower, upper, d } => {
                // JCGM 101 6.4.3.1: d > 0 and a + d < b − d.
                if !finite(&[lower, upper, d]) || d < 0.0 || lower + d >= upper - d {
                    return bad("curvilinear trapezoid: d ≥ 0 and lower + d < upper − d required");
                }
            }
            Distribution::Exponential { mean } => {
                if !(mean.is_finite() && mean > 0.0) {
                    return bad("exponential: mean > 0 required");
                }
            }
            Distribution::Gamma { shape, scale } => {
                if !(shape.is_finite() && scale.is_finite() && shape > 0.0 && scale > 0.0) {
                    return bad("gamma: shape > 0 and scale > 0 required");
                }
            }
        }
        Ok(())
    }

    /// Expectation E(X), used as the best estimate x.
    pub fn expectation(&self) -> f64 {
        match *self {
            Distribution::Constant { value } => value,
            Distribution::Normal { mean, .. } | Distribution::StudentT { mean, .. } => mean,
            Distribution::Rectangular { lower, upper }
            | Distribution::Triangular { lower, upper }
            | Distribution::Trapezoidal { lower, upper, .. }
            | Distribution::CurvilinearTrapezoid { lower, upper, .. }
            | Distribution::Arcsine { lower, upper } => 0.5 * (lower + upper),
            Distribution::Exponential { mean } => mean,
            Distribution::Gamma { shape, scale } => shape * scale,
        }
    }

    /// Variance V(X) of the distribution, if it exists.
    pub fn variance(&self) -> Option<f64> {
        let w = |l: f64, u: f64| u - l;
        Some(match *self {
            Distribution::Constant { .. } => 0.0,
            Distribution::Normal { sd, .. } => sd * sd,
            Distribution::StudentT { scale, dof, .. } => {
                if dof.is_infinite() {
                    scale * scale
                } else if dof > 2.0 {
                    scale * scale * dof / (dof - 2.0)
                } else {
                    return None;
                }
            }
            Distribution::Rectangular { lower, upper } => w(lower, upper).powi(2) / 12.0,
            Distribution::Triangular { lower, upper } => w(lower, upper).powi(2) / 24.0,
            Distribution::Trapezoidal { lower, upper, beta } => w(lower, upper).powi(2) * (1.0 + beta * beta) / 24.0,
            Distribution::CurvilinearTrapezoid { lower, upper, d } => w(lower, upper).powi(2) / 12.0 + d * d / 9.0,
            Distribution::Arcsine { lower, upper } => w(lower, upper).powi(2) / 8.0,
            Distribution::Exponential { mean } => mean * mean,
            Distribution::Gamma { shape, scale } => shape * scale * scale,
        })
    }

    /// Standard deviation of the distribution (MCM view), if it exists.
    pub fn standard_deviation(&self) -> Option<f64> {
        self.variance().map(f64::sqrt)
    }

    /// Standard uncertainty used in the GUM uncertainty framework.
    ///
    /// Equal to the standard deviation except for the t-distribution, for
    /// which the scale parameter (s/√n, or U/k from a certificate) is returned
    /// in accordance with GUM 4.2.3 and G.4 (see the module documentation).
    pub fn gum_standard_uncertainty(&self) -> f64 {
        match *self {
            Distribution::StudentT { scale, .. } => scale,
            _ => self.standard_deviation().expect("only the t-distribution can lack a variance"),
        }
    }

    /// Degrees of freedom attached to the standard uncertainty in the GUM
    /// framework: ν for a t-distribution, otherwise infinite (a finite value
    /// for a Type B evaluation can be set on the input quantity, GUM G.4.2).
    pub fn gum_degrees_of_freedom(&self) -> f64 {
        match *self {
            Distribution::StudentT { dof, .. } => dof,
            _ => f64::INFINITY,
        }
    }

    pub fn is_gaussian(&self) -> bool {
        matches!(self, Distribution::Normal { .. })
    }

    /// Short name for reports.
    pub fn kind(&self) -> &'static str {
        match self {
            Distribution::Constant { .. } => "constant",
            Distribution::Normal { .. } => "normal",
            Distribution::StudentT { .. } => "t",
            Distribution::Rectangular { .. } => "rectangular",
            Distribution::Triangular { .. } => "triangular",
            Distribution::Trapezoidal { .. } => "trapezoidal",
            Distribution::CurvilinearTrapezoid { .. } => "curvilinear-trapezoid",
            Distribution::Arcsine { .. } => "arcsine",
            Distribution::Exponential { .. } => "exponential",
            Distribution::Gamma { .. } => "gamma",
        }
    }

    /// One draw from the distribution using the sampling procedures of
    /// JCGM 101 6.4.
    pub fn sample(&self, rng: &mut Rng) -> f64 {
        match *self {
            Distribution::Constant { value } => value,
            Distribution::Normal { mean, sd } => mean + sd * rng.normal(),
            Distribution::StudentT { mean, scale, dof } => mean + scale * rng.student_t(dof),
            Distribution::Rectangular { lower, upper } => lower + (upper - lower) * rng.uniform(),
            Distribution::Triangular { lower, upper } => {
                // 6.4.5.4: ξ = a + (b − a)(r1 + r2)/2
                lower + 0.5 * (upper - lower) * (rng.uniform() + rng.uniform())
            }
            Distribution::Trapezoidal { lower, upper, beta } => {
                // 6.4.4.4: ξ = a + (b − a)/2 [(1 + β) r1 + (1 − β) r2]
                let r1 = rng.uniform();
                let r2 = rng.uniform();
                lower + 0.5 * (upper - lower) * ((1.0 + beta) * r1 + (1.0 - beta) * r2)
            }
            Distribution::CurvilinearTrapezoid { lower, upper, d } => {
                // 6.4.3.4: a_s = (a − d) + 2 d r1, b_s = (a + b) − a_s, ξ = a_s + (b_s − a_s) r2
                let a_s = (lower - d) + 2.0 * d * rng.uniform();
                let b_s = (lower + upper) - a_s;
                a_s + (b_s - a_s) * rng.uniform()
            }
            Distribution::Arcsine { lower, upper } => {
                // 6.4.6.4: ξ = (a + b)/2 + (b − a)/2 sin 2πr
                0.5 * (lower + upper) + 0.5 * (upper - lower) * (2.0 * std::f64::consts::PI * rng.uniform()).sin()
            }
            Distribution::Exponential { mean } => -mean * rng.uniform().ln(),
            Distribution::Gamma { shape, scale } => scale * rng.gamma(shape),
        }
    }
}

/// Summary of a Type A evaluation from a series of indications [GUM 4.2].
#[derive(Clone, Debug, PartialEq)]
pub struct TypeA {
    pub n: usize,
    pub mean: f64,
    /// Experimental standard deviation s(q_k) [GUM 4.2.2].
    pub std_dev: f64,
    /// Experimental standard deviation of the mean s(q̄) = s/√n [GUM 4.2.3].
    pub std_uncertainty: f64,
    pub dof: f64,
}

impl TypeA {
    /// Type A evaluation of a series of n ≥ 2 independent indications.
    pub fn from_observations(values: &[f64]) -> Result<Self, UncError> {
        let n = values.len();
        if n < 2 {
            return Err(UncError::InvalidDistribution("a Type A evaluation needs at least two indications".into()));
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(UncError::InvalidDistribution("indications must be finite".into()));
        }
        let mean = crate::special::accurate_sum(values) / n as f64;
        let dev: Vec<f64> = values.iter().map(|v| (v - mean).powi(2)).collect();
        let s = (crate::special::accurate_sum(&dev) / (n as f64 - 1.0)).sqrt();
        Ok(Self { n, mean, std_dev: s, std_uncertainty: s / (n as f64).sqrt(), dof: n as f64 - 1.0 })
    }

    /// Type A evaluation using a pooled standard deviation s_p with ν_p degrees
    /// of freedom for the mean of n indications [GUM 4.2.4, H.3.6 note;
    /// JCGM 101 6.4.9.6].
    pub fn pooled(mean: f64, pooled_sd: f64, pooled_dof: f64, n: usize) -> Self {
        Self { n, mean, std_dev: pooled_sd, std_uncertainty: pooled_sd / (n as f64).sqrt(), dof: pooled_dof }
    }

    /// The distribution JCGM 101 6.4.9 assigns: t_ν(x̄, s²/n).
    pub fn distribution(&self) -> Distribution {
        Distribution::StudentT { mean: self.mean, scale: self.std_uncertainty, dof: self.dof }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_moments(d: &Distribution, n: usize, seed: u64) -> (f64, f64) {
        let mut rng = Rng::new(seed);
        let xs: Vec<f64> = (0..n).map(|_| d.sample(&mut rng)).collect();
        let m = xs.iter().sum::<f64>() / n as f64;
        let v = xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n as f64 - 1.0);
        (m, v)
    }

    #[test]
    fn analytic_variances() {
        // JCGM 101 6.4.3 example: 10.0 V ± 0.1 V, d = 0.05 V → u = 0.060 V.
        let ct = Distribution::CurvilinearTrapezoid { lower: 9.9, upper: 10.1, d: 0.05 };
        assert!((ct.gum_standard_uncertainty() - 0.060).abs() < 0.0005);
        // GUM 4.3.7: rectangular semi-width a → a/√3.
        let r = Distribution::rectangular_half_width(0.0, 3f64.sqrt());
        assert!((r.gum_standard_uncertainty() - 1.0).abs() < 1e-15);
        // GUM 4.3.9: triangular semi-width a → a/√6.
        let t = Distribution::triangular_half_width(0.0, 6f64.sqrt());
        assert!((t.gum_standard_uncertainty() - 1.0).abs() < 1e-15);
        // Arc sine semi-width a → a/√2.
        let u = Distribution::arcsine_half_width(0.0, 2f64.sqrt());
        assert!((u.gum_standard_uncertainty() - 1.0).abs() < 1e-15);
        // t-distribution: GUM view uses the scale, MCM view the true sd.
        let st = Distribution::StudentT { mean: 0.0, scale: 1.0, dof: 4.0 };
        assert_eq!(st.gum_standard_uncertainty(), 1.0);
        assert!((st.standard_deviation().unwrap() - 2f64.sqrt()).abs() < 1e-15);
        assert!(Distribution::StudentT { mean: 0.0, scale: 1.0, dof: 2.0 }.variance().is_none());
    }

    #[test]
    fn sampling_reproduces_moments() {
        let cases = [
            Distribution::Normal { mean: 1.0, sd: 2.0 },
            Distribution::StudentT { mean: -1.0, scale: 0.5, dof: 6.0 },
            Distribution::Rectangular { lower: -1.0, upper: 3.0 },
            Distribution::Triangular { lower: 0.0, upper: 2.0 },
            Distribution::Trapezoidal { lower: -2.0, upper: 2.0, beta: 0.4 },
            Distribution::CurvilinearTrapezoid { lower: -1.0, upper: 1.0, d: 0.3 },
            Distribution::Arcsine { lower: 4.0, upper: 6.0 },
            Distribution::Exponential { mean: 2.5 },
            Distribution::Gamma { shape: 4.0, scale: 0.5 },
        ];
        for (i, d) in cases.iter().enumerate() {
            d.validate().unwrap();
            let (m, v) = sample_moments(d, 1_000_000, 100 + i as u64);
            let ev = d.variance().unwrap();
            let sd = ev.sqrt();
            assert!((m - d.expectation()).abs() < 5.0 * sd / 1000.0, "{d:?}: mean {m}");
            assert!(((v - ev) / ev).abs() < 0.01, "{d:?}: var {v} vs {ev}");
        }
    }

    #[test]
    fn sampling_respects_support() {
        let mut rng = Rng::new(5);
        let d = Distribution::CurvilinearTrapezoid { lower: 1.0, upper: 2.0, d: 0.1 };
        for _ in 0..100_000 {
            let x = d.sample(&mut rng);
            assert!((0.9..=2.1).contains(&x));
        }
        let d = Distribution::Trapezoidal { lower: 1.0, upper: 2.0, beta: 0.5 };
        for _ in 0..100_000 {
            let x = d.sample(&mut rng);
            assert!((1.0..=2.0).contains(&x));
        }
    }

    #[test]
    fn invalid_parameters_rejected() {
        assert!(Distribution::Normal { mean: 0.0, sd: -1.0 }.validate().is_err());
        assert!(Distribution::Rectangular { lower: 2.0, upper: 1.0 }.validate().is_err());
        assert!(Distribution::Trapezoidal { lower: 0.0, upper: 1.0, beta: 1.5 }.validate().is_err());
        assert!(Distribution::CurvilinearTrapezoid { lower: 0.0, upper: 1.0, d: 0.6 }.validate().is_err());
        assert!(Distribution::StudentT { mean: 0.0, scale: 1.0, dof: 0.0 }.validate().is_err());
    }

    #[test]
    fn type_a_evaluation() {
        // Mean 10.0, s = sqrt(2.5) for 1..5 shifted.
        let ta = TypeA::from_observations(&[8.0, 9.0, 10.0, 11.0, 12.0]).unwrap();
        assert_eq!(ta.n, 5);
        assert!((ta.mean - 10.0).abs() < 1e-15);
        assert!((ta.std_dev - 2.5f64.sqrt()).abs() < 1e-15);
        assert!((ta.std_uncertainty - (2.5f64 / 5.0).sqrt()).abs() < 1e-15);
        assert_eq!(ta.dof, 4.0);
        assert!(TypeA::from_observations(&[1.0]).is_err());
    }
}
