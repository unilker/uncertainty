//! Pseudo-random number generation following JCGM 101:2008 Annex C.
//!
//! * Rectangular R(0, 1): enhanced Wichmann–Hill generator (Table C.2, 64-bit
//!   form of C.3.3.3), period ≈ 2^121.
//! * Gaussian N(0, 1): Box–Muller transform (Table C.3).
//! * Student t: rejection procedure of Table C.5.
//! * Gamma: Marsaglia–Tsang method (used for gamma distributions with
//!   non-integer shape; JCGM 101 6.4.11 covers the integer case).

/// Multipliers a_j of Table C.2.
const WH_A: [i64; 4] = [11_600, 47_003, 23_000, 33_000];
/// Moduli d_j = 2 147 483 123 + (456, 420, 300, 0) of Table C.2.
const WH_D: [i64; 4] = [2_147_483_579, 2_147_483_543, 2_147_483_423, 2_147_483_123];

/// Enhanced Wichmann–Hill generator [Wichmann & Hill 2006; JCGM 101 C.3.3].
#[derive(Clone, Debug)]
pub struct WichmannHill {
    state: [i64; 4],
}

impl WichmannHill {
    /// Creates a generator from four explicit seeds, each 1 ≤ i_j < d_j.
    pub fn from_state(state: [i64; 4]) -> Option<Self> {
        if state.iter().zip(WH_D.iter()).all(|(&s, &d)| s >= 1 && s < d) {
            Some(Self { state })
        } else {
            None
        }
    }

    /// Creates a generator from a single 64-bit seed. The four component seeds
    /// are derived with SplitMix64 so that nearby seeds give unrelated streams.
    pub fn from_seed(seed: u64) -> Self {
        let mut z = seed;
        let mut state = [0i64; 4];
        for (s, &d) in state.iter_mut().zip(WH_D.iter()) {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            x ^= x >> 31;
            *s = 1 + (x % (d as u64 - 1)) as i64;
        }
        Self { state }
    }

    /// Current internal state (for reproducibility records).
    pub fn state(&self) -> [i64; 4] {
        self.state
    }

    /// Next draw from R(0, 1); the value 0 is never returned.
    pub fn next_f64(&mut self) -> f64 {
        loop {
            let mut w = 0.0;
            for j in 0..4 {
                self.state[j] = (WH_A[j] * self.state[j]) % WH_D[j];
                w += self.state[j] as f64 / WH_D[j] as f64;
            }
            let r = w - w.floor();
            if r > 0.0 {
                return r;
            }
        }
    }

    /// The 32-bit integer form of Table C.2 step a) (Schrage decomposition).
    /// Kept to demonstrate equivalence with the 64-bit form in tests.
    #[cfg(test)]
    fn next_f64_32bit(&mut self) -> f64 {
        const B: [i64; 4] = [185_127, 45_688, 93_368, 65_075];
        const C: [i64; 4] = [10_379, 10_479, 19_423, 8_123];
        let mut w = 0.0;
        for j in 0..4 {
            let i = self.state[j];
            let mut next = WH_A[j] * (i % B[j]) - C[j] * (i / B[j]);
            if next < 0 {
                next += WH_D[j];
            }
            self.state[j] = next;
            w += next as f64 / WH_D[j] as f64;
        }
        w - w.floor()
    }
}

/// Random source used by the Monte Carlo method.
#[derive(Clone, Debug)]
pub struct Rng {
    uniform: WichmannHill,
    spare_normal: Option<f64>,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { uniform: WichmannHill::from_seed(seed), spare_normal: None }
    }

    /// Draw from R(0, 1), never exactly 0.
    #[inline]
    pub fn uniform(&mut self) -> f64 {
        self.uniform.next_f64()
    }

    /// Draw from N(0, 1) by the Box–Muller transform (JCGM 101 Table C.3).
    pub fn normal(&mut self) -> f64 {
        if let Some(z) = self.spare_normal.take() {
            return z;
        }
        let r1 = self.uniform();
        let r2 = self.uniform();
        let rad = (-2.0 * r1.ln()).sqrt();
        let (s, c) = (2.0 * std::f64::consts::PI * r2).sin_cos();
        self.spare_normal = Some(rad * s);
        rad * c
    }

    /// Draw from the central t-distribution with ν degrees of freedom
    /// (JCGM 101 Table C.5). ν may be non-integer.
    pub fn student_t(&mut self, nu: f64) -> f64 {
        if nu.is_infinite() {
            return self.normal();
        }
        loop {
            let r1 = self.uniform();
            let r2 = self.uniform();
            let (t, v) = if r1 < 0.5 {
                let t = 1.0 / (4.0 * r1 - 1.0);
                (t, r2 / (t * t))
            } else {
                (4.0 * r1 - 3.0, r2)
            };
            if !t.is_finite() {
                continue;
            }
            if v < 1.0 - 0.5 * t.abs() || v < (1.0 + t * t / nu).powf(-0.5 * (nu + 1.0)) {
                return t;
            }
        }
    }

    /// Draw from the gamma distribution with the given shape and unit scale
    /// (Marsaglia & Tsang 2000).
    pub fn gamma(&mut self, shape: f64) -> f64 {
        if shape < 1.0 {
            let u = self.uniform();
            return self.gamma(shape + 1.0) * u.powf(1.0 / shape);
        }
        let d = shape - 1.0 / 3.0;
        let c = 1.0 / (9.0 * d).sqrt();
        loop {
            let x = self.normal();
            let v = 1.0 + c * x;
            if v <= 0.0 {
                continue;
            }
            let v = v * v * v;
            let u = self.uniform();
            if u < 1.0 - 0.0331 * x.powi(4) || u.ln() < 0.5 * x * x + d * (1.0 - v + v.ln()) {
                return d * v;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixty_four_bit_form_equals_thirty_two_bit_form() {
        // JCGM 101 C.3.3.3 states the two forms are equivalent.
        let seeds = [1, 2_147_483_000, 123_456_789, 42];
        let mut a = WichmannHill::from_state(seeds).unwrap();
        let mut b = a.clone();
        for _ in 0..100_000 {
            let x = a.next_f64();
            let y = b.next_f64_32bit();
            assert_eq!(x, y);
        }
        assert_eq!(a.state(), b.state());
    }

    #[test]
    fn invalid_state_rejected() {
        assert!(WichmannHill::from_state([0, 1, 1, 1]).is_none());
        assert!(WichmannHill::from_state([1, 1, 1, 2_147_483_123]).is_none());
    }

    #[test]
    fn uniform_moments() {
        let mut rng = Rng::new(2026);
        let n = 1_000_000;
        let mut s1 = 0.0;
        let mut s2 = 0.0;
        for _ in 0..n {
            let r = rng.uniform();
            assert!(r > 0.0 && r < 1.0);
            s1 += r;
            s2 += r * r;
        }
        let mean = s1 / n as f64;
        let var = s2 / n as f64 - mean * mean;
        // Standard error of the mean is 0.289/1000.
        assert!((mean - 0.5).abs() < 0.0015);
        assert!((var - 1.0 / 12.0).abs() < 0.0005);
    }

    #[test]
    fn normal_moments() {
        let mut rng = Rng::new(7);
        let n = 1_000_000;
        let (mut s1, mut s2, mut s4) = (0.0, 0.0, 0.0);
        for _ in 0..n {
            let z = rng.normal();
            s1 += z;
            s2 += z * z;
            s4 += z.powi(4);
        }
        let n = n as f64;
        assert!((s1 / n).abs() < 0.005);
        assert!((s2 / n - 1.0).abs() < 0.005);
        assert!((s4 / n - 3.0).abs() < 0.05);
    }

    #[test]
    fn student_t_matches_distribution_function() {
        // Empirical CDF at several points versus the exact t distribution function.
        use crate::special::student_t_cdf;
        for &nu in &[3.0, 7.5, 30.0] {
            let mut rng = Rng::new(99);
            let n = 400_000;
            let draws: Vec<f64> = (0..n).map(|_| rng.student_t(nu)).collect();
            for &t in &[-2.5, -1.0, 0.0, 0.7, 2.0] {
                let emp = draws.iter().filter(|&&x| x <= t).count() as f64 / n as f64;
                let exact = student_t_cdf(t, nu);
                // 4.5 standard errors of a proportion at n = 4e5.
                assert!(
                    (emp - exact).abs() < 4.5 * (exact * (1.0 - exact) / n as f64).sqrt() + 1e-4,
                    "nu={nu} t={t} emp={emp} exact={exact}"
                );
            }
        }
    }

    #[test]
    fn gamma_moments() {
        for &shape in &[0.5, 1.0, 3.7, 20.0] {
            let mut rng = Rng::new(11);
            let n = 500_000;
            let (mut s1, mut s2) = (0.0, 0.0);
            for _ in 0..n {
                let g = rng.gamma(shape);
                s1 += g;
                s2 += g * g;
            }
            let mean = s1 / n as f64;
            let var = s2 / n as f64 - mean * mean;
            assert!(((mean - shape) / shape).abs() < 0.01, "shape={shape}");
            assert!(((var - shape) / shape).abs() < 0.03, "shape={shape}");
        }
    }
}
