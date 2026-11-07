//! Monte Carlo method for the propagation of distributions
//! (JCGM 101:2008, clause 7) and validation of the GUM uncertainty framework
//! (clause 8).

use crate::distribution::Distribution;
use crate::error::UncError;
use crate::gum::GumResult;
use crate::model::Model;
use crate::rng::Rng;
use crate::special::accurate_sum;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Trials {
    /// A pre-assigned number of trials M [JCGM 101 7.2].
    Fixed(usize),
    /// Adaptive procedure of JCGM 101 7.9.4.
    Adaptive {
        /// Number of significant decimal digits regarded as meaningful in u(y)
        /// (normally 1 or 2).
        ndig: u32,
        /// The tolerance δ of 7.9.2 is divided by this factor; use 5 to obtain
        /// results for validating the GUM framework [JCGM 101 8.2].
        tolerance_divisor: f64,
        /// Safety limit on the total number of trials.
        max_trials: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntervalKind {
    /// Probabilistically symmetric coverage interval [JCGM 101 7.7.2].
    Symmetric,
    /// Shortest coverage interval [JCGM 101 7.7.2].
    Shortest,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct McmOptions {
    pub trials: Trials,
    pub coverage_probability: f64,
    pub seed: u64,
    /// Interval whose endpoints must stabilize in the adaptive procedure.
    pub interval: IntervalKind,
    pub histogram_bins: usize,
    /// Keep all model values in the result (needed for plots or further use
    /// as an input distribution, JCGM 101 6.5).
    pub keep_values: bool,
}

impl Default for McmOptions {
    fn default() -> Self {
        Self {
            trials: Trials::Fixed(1_000_000),
            coverage_probability: 0.95,
            seed: 1,
            interval: IntervalKind::Shortest,
            histogram_bins: 100,
            keep_values: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Histogram {
    pub lower: f64,
    pub bin_width: f64,
    pub counts: Vec<u64>,
    /// Model values below `lower` / above the last bin.
    pub underflow: u64,
    pub overflow: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct McmResult {
    /// Average of the model values ỹ [JCGM 101 eq. (16)].
    pub estimate: f64,
    /// Standard deviation u(ỹ) [JCGM 101 eq. (17)].
    pub standard_uncertainty: f64,
    pub coverage_probability: f64,
    pub symmetric_interval: (f64, f64),
    pub shortest_interval: (f64, f64),
    /// Half-width U of the coverage interval symmetric about the estimate,
    /// Pr(|Y − ỹ| ≤ U) = p: the expanded uncertainty in the definition
    /// proposed by JCGM-WG1/WG2 (JCGM-WG1 News, December 2025).
    pub expanded_about_estimate: f64,
    pub median: f64,
    pub skewness: f64,
    pub excess_kurtosis: f64,
    pub trials: usize,
    /// Number of sequential runs (adaptive procedure); 1 for fixed M.
    pub runs: usize,
    /// False if the adaptive procedure hit `max_trials` before stabilizing.
    pub converged: bool,
    /// Numerical tolerance reached by the adaptive procedure (None for fixed M).
    pub tolerance: Option<f64>,
    pub seed: u64,
    pub histogram: Histogram,
    pub values: Option<Vec<f64>>,
}

impl McmResult {
    pub fn interval(&self, kind: IntervalKind) -> (f64, f64) {
        match kind {
            IntervalKind::Symmetric => self.symmetric_interval,
            IntervalKind::Shortest => self.shortest_interval,
        }
    }
}

/// Correlated Gaussian group: input indices, means, lower Cholesky factor.
type GaussianGroup = (Vec<usize>, Vec<f64>, Vec<Vec<f64>>);

/// Multivariate t-distribution t_ν(x̄, S/n) for a group of inputs
/// [JCGM 102 5.3.2]: input indices, x̄, lower Cholesky factor of S/n, ν.
pub(crate) type StudentGroup = (Vec<usize>, Vec<f64>, Vec<Vec<f64>>, f64);

/// Draws input vectors from the assigned (joint) distribution.
pub(crate) struct Sampler {
    independent: Vec<(usize, Distribution)>,
    groups: Vec<GaussianGroup>,
    students: Vec<StudentGroup>,
    z: Vec<f64>,
}

impl Sampler {
    pub(crate) fn new(model: &Model) -> Result<Self, UncError> {
        Self::with_students(model, Vec::new())
    }

    /// As [`Sampler::new`], with groups of inputs drawn jointly from
    /// multivariate t-distributions; their correlations in `model` are used
    /// by the GUM framework only.
    pub(crate) fn with_students(model: &Model, students: Vec<StudentGroup>) -> Result<Self, UncError> {
        let n = model.n_inputs();
        let in_student = |i: usize| students.iter().any(|g| g.0.contains(&i));
        // Connected components of the correlation graph.
        let mut group_of = vec![usize::MAX; n];
        for i in 0..n {
            if in_student(i) {
                group_of[i] = usize::MAX - 1;
            }
        }
        let mut groups_idx: Vec<Vec<usize>> = Vec::new();
        for start in 0..n {
            if group_of[start] != usize::MAX {
                continue;
            }
            let g = groups_idx.len();
            let mut stack = vec![start];
            let mut members = Vec::new();
            group_of[start] = g;
            while let Some(i) = stack.pop() {
                members.push(i);
                for j in 0..n {
                    if i != j && model.correlation[i][j] != 0.0 && group_of[j] == usize::MAX {
                        group_of[j] = g;
                        stack.push(j);
                    }
                }
            }
            members.sort_unstable();
            groups_idx.push(members);
        }

        let mut independent = Vec::new();
        let mut groups = Vec::new();
        let mut max_group = students.iter().map(|g| g.0.len()).max().unwrap_or(0);
        for members in groups_idx {
            if members.len() == 1 {
                let i = members[0];
                independent.push((i, model.inputs[i].distribution.clone()));
                continue;
            }
            let mut means = Vec::with_capacity(members.len());
            let mut sds = Vec::with_capacity(members.len());
            for &i in &members {
                match model.inputs[i].distribution {
                    Distribution::Normal { mean, sd } => {
                        means.push(mean);
                        sds.push(sd);
                    }
                    ref other => {
                        return Err(UncError::Unsupported(format!(
                            "Monte Carlo sampling of correlated inputs requires a joint (multivariate \
                             Gaussian) distribution [JCGM 101 6.4.8]; '{}' is {}",
                            model.inputs[i].name,
                            other.kind()
                        )))
                    }
                }
            }
            let m = members.len();
            let mut cov = vec![vec![0.0; m]; m];
            for a in 0..m {
                for b in 0..m {
                    cov[a][b] = model.correlation[members[a]][members[b]] * sds[a] * sds[b];
                }
            }
            let names: Vec<&str> = members.iter().map(|&i| model.inputs[i].name.as_str()).collect();
            let l = cholesky(&cov).ok_or_else(|| {
                UncError::InvalidCorrelation(format!(
                    "the covariance matrix of {} is not positive definite [JCGM 101 C.5.2 note 2]",
                    names.join(", ")
                ))
            })?;
            max_group = max_group.max(m);
            groups.push((members, means, l));
        }
        Ok(Self { independent, groups, students, z: vec![0.0; max_group] })
    }

    #[inline]
    pub(crate) fn draw(&mut self, rng: &mut Rng, x: &mut [f64]) {
        for (i, d) in &self.independent {
            x[*i] = d.sample(rng);
        }
        for (members, means, l) in &self.groups {
            let m = members.len();
            for k in 0..m {
                self.z[k] = rng.normal();
            }
            // X = µ + L z  [JCGM 101 Table C.4, with L = Rᵀ]
            for a in 0..m {
                let mut v = means[a];
                for b in 0..=a {
                    v += l[a][b] * self.z[b];
                }
                x[members[a]] = v;
            }
        }
        // ξ = x̄ + L z √(ν/w), w ~ χ²_ν  [JCGM 102 5.3.2.4]
        for (members, means, l, nu) in &self.students {
            let m = members.len();
            for k in 0..m {
                self.z[k] = rng.normal();
            }
            let f = (nu / (2.0 * rng.gamma(0.5 * nu))).sqrt();
            for a in 0..m {
                let mut v = 0.0;
                for b in 0..=a {
                    v += l[a][b] * self.z[b];
                }
                x[members[a]] = means[a] + f * v;
            }
        }
    }
}

/// Lower-triangular Cholesky factor L with V = L Lᵀ, or None if V is not
/// positive semi-definite (within rounding). A zero pivot is accepted when the
/// corresponding row is fully determined by earlier rows (|r| = 1).
pub(crate) fn cholesky(v: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = v.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut s = v[i][j];
            for k in 0..j {
                s -= l[i][k] * l[j][k];
            }
            let tol = 1e-12 * (v[i][i] * v[j][j]).sqrt().max(f64::MIN_POSITIVE);
            if i == j {
                if s < -tol {
                    return None;
                }
                l[i][i] = if s > tol { s.sqrt() } else { 0.0 };
            } else if l[j][j] > 0.0 {
                l[i][j] = s / l[j][j];
            } else if s.abs() > tol {
                return None;
            }
        }
    }
    Some(l)
}

struct Runner {
    program: crate::expr::Program,
    sampler: Sampler,
    rng: Rng,
    x: Vec<f64>,
    stack: Vec<f64>,
}

impl Runner {
    fn run(&mut self, m: usize, out: &mut Vec<f64>) -> Result<(), UncError> {
        let mut bad = 0usize;
        out.reserve(m);
        for _ in 0..m {
            self.sampler.draw(&mut self.rng, &mut self.x);
            let y = self.program.eval_with(&self.x, &mut self.stack);
            if y.is_finite() {
                out.push(y);
            } else {
                bad += 1;
            }
        }
        if bad > 0 {
            return Err(UncError::NonFiniteModelValues { count: bad, trials: m });
        }
        Ok(())
    }
}

/// Number of model values q covering probability p among M sorted values
/// [JCGM 101 7.7.2].
pub(crate) fn coverage_count(m: usize, p: f64) -> usize {
    let pm = p * m as f64;
    if (pm - pm.round()).abs() < 1e-9 * pm.max(1.0) {
        pm.round() as usize
    } else {
        (pm + 0.5).floor() as usize
    }
}

/// Probabilistically symmetric and shortest coverage intervals from sorted
/// model values [JCGM 101 7.7.2]. Indices in the standard are 1-based.
pub fn coverage_intervals(sorted: &[f64], p: f64) -> ((f64, f64), (f64, f64)) {
    let m = sorted.len();
    let q = coverage_count(m, p).clamp(1, m - 1);
    let r_sym = (m - q).div_ceil(2);
    let r_sym = r_sym.max(1);
    let symmetric = (sorted[r_sym - 1], sorted[r_sym + q - 1]);
    let mut best_r = 1;
    let mut best_len = f64::INFINITY;
    for r in 1..=(m - q) {
        let len = sorted[r + q - 1] - sorted[r - 1];
        if len < best_len {
            best_len = len;
            best_r = r;
        }
    }
    let shortest = (sorted[best_r - 1], sorted[best_r + q - 1]);
    (symmetric, shortest)
}

/// Smallest half-width U with at least the fraction p of the values in
/// [c − U, c + U] (the q-th smallest |y_r − c|, q as for the coverage
/// intervals [JCGM 101 7.7.1]).
pub fn half_width_about(values: &[f64], c: f64, p: f64) -> f64 {
    let m = values.len();
    let q = coverage_count(m, p).clamp(1, m);
    let mut d: Vec<f64> = values.iter().map(|v| (v - c).abs()).collect();
    let (_, kth, _) = d.select_nth_unstable_by(q - 1, f64::total_cmp);
    *kth
}

/// Numerical tolerance δ associated with a value z expressed to `ndig`
/// significant decimal digits [JCGM 101 7.9.2].
pub fn numerical_tolerance(z: f64, ndig: u32) -> f64 {
    assert!(ndig >= 1);
    let z = z.abs();
    if z == 0.0 || !z.is_finite() {
        return 0.0;
    }
    let mut l = z.log10().floor() as i32 - ndig as i32 + 1;
    // Rounding to ndig digits may carry into an extra digit (e.g. 0.0996 → 0.1).
    let c = (z / 10f64.powi(l)).round();
    if c >= 10f64.powi(ndig as i32) {
        l += 1;
    }
    0.5 * 10f64.powi(l)
}

fn mean_and_sd(values: &[f64]) -> (f64, f64) {
    let n = values.len() as f64;
    let mean = accurate_sum(values) / n;
    // Two-pass formula (JCGM 101 7.6 note 1), compensated.
    let mut sum = 0.0;
    let mut comp = 0.0;
    for &v in values {
        let d = (v - mean) * (v - mean);
        let t = sum + d;
        comp += if sum >= d { (sum - t) + d } else { (d - t) + sum };
        sum = t;
    }
    let var = if values.len() > 1 { (sum + comp) / (n - 1.0) } else { 0.0 };
    (mean, var.sqrt())
}

struct RunSummary {
    mean: f64,
    sd: f64,
    low: f64,
    high: f64,
}

fn summarize(values: &mut [f64], p: f64, kind: IntervalKind) -> RunSummary {
    let (mean, sd) = mean_and_sd(values);
    values.sort_unstable_by(f64::total_cmp);
    let (sym, short) = coverage_intervals(values, p);
    let (low, high) = match kind {
        IntervalKind::Symmetric => sym,
        IntervalKind::Shortest => short,
    };
    RunSummary { mean, sd, low, high }
}

/// Standard deviation of the average of h values [JCGM 101 7.9.4 g)].
fn sd_of_average(v: &[f64]) -> f64 {
    let h = v.len() as f64;
    let mean = v.iter().sum::<f64>() / h;
    (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (h * (h - 1.0))).sqrt()
}

/// Applies the Monte Carlo method to a model.
pub fn evaluate(model: &Model, options: &McmOptions) -> Result<McmResult, UncError> {
    let p = options.coverage_probability;
    if !(p > 0.0 && p < 1.0) {
        return Err(UncError::InvalidOption("coverage probability must lie in (0, 1)".into()));
    }
    let program = model.expr.compile();
    let mut runner = Runner {
        stack: Vec::with_capacity(program.stack_size()),
        program,
        sampler: Sampler::new(model)?,
        rng: Rng::new(options.seed),
        x: vec![0.0; model.n_inputs()],
    };

    // Smallest M for which a 100p % interval is meaningful [JCGM 101 7.2.2].
    let j_min = (100.0 / (1.0 - p)).ceil() as usize;

    let mut values: Vec<f64> = Vec::new();
    let (runs, converged, tolerance) = match options.trials {
        Trials::Fixed(m) => {
            if m < 2 || coverage_count(m, p) >= m {
                return Err(UncError::InvalidOption(format!(
                    "M = {m} trials is too small for a {p} coverage interval (need M ≫ 1/(1 − p))"
                )));
            }
            runner.run(m, &mut values)?;
            (1, true, None)
        }
        Trials::Adaptive { ndig, tolerance_divisor, max_trials } => {
            if ndig == 0 || !(tolerance_divisor >= 1.0) {
                return Err(UncError::InvalidOption("adaptive MCM needs ndig ≥ 1 and divisor ≥ 1".into()));
            }
            let m = j_min.max(10_000);
            let mut stats: Vec<RunSummary> = Vec::new();
            let mut chunk = Vec::with_capacity(m);
            let mut converged = false;
            let mut delta = 0.0;
            loop {
                chunk.clear();
                runner.run(m, &mut chunk)?;
                values.extend_from_slice(&chunk);
                stats.push(summarize(&mut chunk, p, options.interval));
                if stats.len() >= 2 {
                    let s_y = sd_of_average(&stats.iter().map(|s| s.mean).collect::<Vec<_>>());
                    let s_u = sd_of_average(&stats.iter().map(|s| s.sd).collect::<Vec<_>>());
                    let s_lo = sd_of_average(&stats.iter().map(|s| s.low).collect::<Vec<_>>());
                    let s_hi = sd_of_average(&stats.iter().map(|s| s.high).collect::<Vec<_>>());
                    let (_, u_all) = mean_and_sd(&values);
                    delta = numerical_tolerance(u_all, ndig) / tolerance_divisor;
                    if [s_y, s_u, s_lo, s_hi].iter().all(|s| 2.0 * s <= delta) {
                        converged = true;
                        break;
                    }
                }
                if values.len() + m > max_trials {
                    break;
                }
            }
            (stats.len(), converged, Some(delta))
        }
    };

    let trials = values.len();
    let (mean, sd) = mean_and_sd(&values);
    let n = trials as f64;
    let (mut m3, mut m4) = (0.0, 0.0);
    for &v in &values {
        let d = v - mean;
        let d2 = d * d;
        m3 += d2 * d;
        m4 += d2 * d2;
    }
    let pop_var = sd * sd * (n - 1.0) / n;
    let (skewness, excess_kurtosis) =
        if pop_var > 0.0 { (m3 / n / pop_var.powf(1.5), m4 / n / (pop_var * pop_var) - 3.0) } else { (0.0, 0.0) };
    values.sort_unstable_by(f64::total_cmp);
    let (symmetric_interval, shortest_interval) = coverage_intervals(&values, p);
    let expanded_about_estimate = half_width_about(&values, mean, p);
    let median = if trials % 2 == 1 { values[trials / 2] } else { 0.5 * (values[trials / 2 - 1] + values[trials / 2]) };
    let histogram = histogram(&values, options.histogram_bins.max(1));

    Ok(McmResult {
        estimate: mean,
        standard_uncertainty: sd,
        coverage_probability: p,
        symmetric_interval,
        shortest_interval,
        expanded_about_estimate,
        median,
        skewness,
        excess_kurtosis,
        trials,
        runs,
        converged,
        tolerance,
        seed: options.seed,
        histogram,
        values: if options.keep_values { Some(values) } else { None },
    })
}

/// Histogram over the central 99.9 % of the sorted model values.
fn histogram(sorted: &[f64], bins: usize) -> Histogram {
    let m = sorted.len();
    let lo = sorted[((m as f64) * 0.0005) as usize];
    let hi = sorted[(((m as f64) * 0.9995) as usize).min(m - 1)];
    let width = if hi > lo { (hi - lo) / bins as f64 } else { 1.0 };
    let mut counts = vec![0u64; bins];
    let (mut under, mut over) = (0u64, 0u64);
    for &v in sorted {
        if v < lo {
            under += 1;
        } else {
            let b = ((v - lo) / width) as usize;
            if b < bins {
                counts[b] += 1;
            } else if v <= hi {
                counts[bins - 1] += 1;
            } else {
                over += 1;
            }
        }
    }
    Histogram { lower: lo, bin_width: width, counts, underflow: under, overflow: over }
}

/// Outcome of comparing the GUM framework with MCM [JCGM 101 8.1].
#[derive(Clone, Debug, PartialEq)]
pub struct Validation {
    pub ndig: u32,
    pub tolerance: f64,
    pub d_low: f64,
    pub d_high: f64,
    pub validated: bool,
}

/// Validates the GUM coverage interval y ± U_p against the MCM interval of
/// the given kind [JCGM 101 8.1.3]. δ is formed from the MCM u(y).
pub fn validate_gum(gum: &GumResult, mcm: &McmResult, ndig: u32, kind: IntervalKind) -> Validation {
    let delta = numerical_tolerance(mcm.standard_uncertainty, ndig);
    let (lo, hi) = mcm.interval(kind);
    let d_low = (gum.estimate - gum.expanded_uncertainty - lo).abs();
    let d_high = (gum.estimate + gum.expanded_uncertainty - hi).abs();
    Validation { ndig, tolerance: delta, d_low, d_high, validated: d_low <= delta && d_high <= delta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tolerance_examples_of_jcgm101_7_9_2() {
        // Example 1: u = 0.000 35 g, ndig = 2 → δ = 0.000 005 g.
        assert!((numerical_tolerance(0.000_35, 2) - 0.000_005).abs() < 1e-18);
        // Example 2: ndig = 1 → δ = 0.000 05 g.
        assert!((numerical_tolerance(0.000_35, 1) - 0.000_05).abs() < 1e-18);
        // Example 3: u = 2 K, ndig = 1 → δ = 0.5 K.
        assert_eq!(numerical_tolerance(2.0, 1), 0.5);
        // 9.2.2.7: u = 2.0, ndig = 2 → δ = 0.05.
        assert!((numerical_tolerance(2.0, 2) - 0.05).abs() < 1e-15);
        // 9.3.2.6: u = 0.0754 → 0.08, ndig = 1 → δ = 0.005.
        assert!((numerical_tolerance(0.0754, 1) - 0.005).abs() < 1e-15);
        // Carry: 0.0996 with ndig = 1 is 1 × 10⁻¹ → δ = 0.05.
        assert!((numerical_tolerance(0.0996, 1) - 0.05).abs() < 1e-15);
    }

    #[test]
    fn coverage_interval_indices() {
        // M = 100, p = 0.95 → q = 95, r = 3 (integer part of 6/2 = 3 when M − q = 5 is odd).
        let v: Vec<f64> = (1..=100).map(|i| i as f64).collect();
        let (sym, short) = coverage_intervals(&v, 0.95);
        assert_eq!(sym, (3.0, 98.0));
        assert_eq!(short.1 - short.0, 95.0);
        // M = 1000, p = 0.95 → q = 950, (M − q)/2 = 25 → [y(25), y(975)].
        let v: Vec<f64> = (1..=1000).map(|i| i as f64).collect();
        let (sym, _) = coverage_intervals(&v, 0.95);
        assert_eq!(sym, (25.0, 975.0));
    }

    #[test]
    fn cholesky_factor() {
        let v = vec![vec![4.0, 2.0], vec![2.0, 3.0]];
        let l = cholesky(&v).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                let s: f64 = (0..2).map(|k| l[i][k] * l[j][k]).sum();
                assert!((s - v[i][j]).abs() < 1e-14);
            }
        }
        assert!(cholesky(&[vec![1.0, 2.0], vec![2.0, 1.0]]).is_none());
        // Perfect correlation is positive semi-definite and accepted.
        assert!(cholesky(&[vec![1.0, 1.0], vec![1.0, 1.0]]).is_some());
    }
}
