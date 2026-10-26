//! Repeatability and reproducibility of a standard measurement method from an
//! interlaboratory experiment — the basic method of ISO 5725-2:2019.
//!
//! * scrutiny of the results for consistency and outliers (8.3): Mandel's h
//!   and k statistics with their indicators (8.3.2, D.3), Cochran's test of
//!   the cell standard deviations (8.3.4, D.1) and Grubbs' tests of the cell
//!   means (8.3.5, D.2);
//! * the general mean and the repeatability, between-laboratory and
//!   reproducibility variances (8.4), or their restricted maximum likelihood
//!   estimates (Annex B.2);
//! * functional relationships between the standard deviations and the mean
//!   (8.5, relationships I–IV);
//! * the uncertainty of the estimated standard deviations (Annex A).
//!
//! The formulae of Annex D for the critical values are those of Wilrich (2013).
//!
//! Each level of the experiment is analysed separately. A *cell* holds the
//! results of one laboratory at one level.

use crate::error::UncError;
use crate::special::{f_quantile, student_t_quantile};

fn invalid(msg: impl Into<String>) -> UncError {
    UncError::InvalidOption(msg.into())
}

// ---------------------------------------------------------------------------
// Critical values and indicators (Clause 9, Annex D)
// ---------------------------------------------------------------------------

/// Critical value of Cochran's test for p standard deviations of n results
/// each at significance level α (Formula D.1; Table 5 for α = 0.05, 0.01).
pub fn cochran_critical(p: usize, n: usize, alpha: f64) -> f64 {
    assert!(p >= 2 && n >= 2, "Cochran's test needs p ≥ 2 and n ≥ 2");
    let (pf, nf) = (p as f64, n as f64);
    let f = f_quantile(alpha / pf, (pf - 1.0) * (nf - 1.0), nf - 1.0);
    1.0 / (1.0 + (pf - 1.0) * f)
}

/// Two-sided critical value of Grubbs' test for one outlying observation
/// among p values at significance level α (Formula D.2; Table 6, columns
/// "one largest or one smallest").
///
/// Formula D.2 prints the degrees of freedom of t as p − 2 in the formula and
/// as n − 2 in its key; both denote the number of values less two.
pub fn grubbs_single_critical(p: usize, alpha: f64) -> f64 {
    assert!(p >= 3, "Grubbs' test needs at least three values");
    let n = p as f64;
    let t = student_t_quantile(1.0 - alpha / (2.0 * n), n - 2.0);
    (n - 1.0) * t / (n * (n - 2.0 + t * t)).sqrt()
}

/// Table 6, columns "two largest or two smallest", lower 1 % and lower 5 %,
/// for p = 4 … 40 [ISO 5725-2:2019, from Grubbs (1969) and Grubbs and Beck (1972)].
// 0.3927 (p = 22) is a tabulated value, not π/8.
#[allow(clippy::approx_constant)]
const GRUBBS_DOUBLE: [(f64, f64); 37] = [
    (0.0000, 0.0002),
    (0.0018, 0.0090),
    (0.0116, 0.0349),
    (0.0308, 0.0708),
    (0.0563, 0.1101),
    (0.0851, 0.1492),
    (0.1150, 0.1864),
    (0.1448, 0.2213),
    (0.1738, 0.2537),
    (0.2016, 0.2836),
    (0.2280, 0.3112),
    (0.2530, 0.3367),
    (0.2767, 0.3603),
    (0.2990, 0.3822),
    (0.3200, 0.4025),
    (0.3398, 0.4214),
    (0.3585, 0.4391),
    (0.3761, 0.4556),
    (0.3927, 0.4711),
    (0.4085, 0.4857),
    (0.4234, 0.4994),
    (0.4376, 0.5123),
    (0.4510, 0.5245),
    (0.4638, 0.5360),
    (0.4759, 0.5470),
    (0.4875, 0.5574),
    (0.4985, 0.5672),
    (0.5091, 0.5766),
    (0.5192, 0.5856),
    (0.5288, 0.5941),
    (0.5381, 0.6023),
    (0.5469, 0.6101),
    (0.5554, 0.6175),
    (0.5636, 0.6247),
    (0.5714, 0.6316),
    (0.5789, 0.6382),
    (0.5862, 0.6445),
];

/// Coefficients g₀, g₁, g₂ of f(n) in Formula D.4 (Table D.1), by one-tailed
/// significance level.
const GRUBBS_D1: [(f64, [f64; 3]); 6] = [
    (0.001, [-4.2493, 1.0012, 0.0443]),
    (0.005, [-3.6613, 0.9558, 0.0388]),
    (0.010, [-3.3101, 0.9250, 0.0362]),
    (0.025, [-2.8580, 0.8833, 0.0322]),
    (0.050, [-2.5075, 0.8501, 0.0289]),
    (0.100, [-2.1615, 0.8169, 0.0251]),
];

/// Two-sided critical value of Grubbs' test for two outlying observations
/// among p values at significance level α; the statistic is significant when
/// it is *below* this value. Table 6 is used for α = 0.05 and 0.01 with
/// 4 ≤ p ≤ 40; otherwise the approximation of Formula D.3 with the one-tailed
/// level α/2, which reproduces exact values to within ±0.003 (D.2.2, Note 2).
/// `None` for p < 4 or a level α/2 not listed in Table D.1.
pub fn grubbs_double_critical(p: usize, alpha: f64) -> Option<f64> {
    if p < 4 {
        return None;
    }
    if p <= 40 {
        let (c1, c5) = GRUBBS_DOUBLE[p - 4];
        if (alpha - 0.05).abs() < 1e-12 {
            return Some(c5);
        }
        if (alpha - 0.01).abs() < 1e-12 {
            return Some(c1);
        }
    }
    let a = 0.5 * alpha;
    let g = GRUBBS_D1.iter().find(|(x, _)| (x - a).abs() < 1e-12)?.1;
    let n = p as f64;
    let f = g[0] + g[1] * n + g[2] * n * n;
    let q = f_quantile((1.0 - a).powf(1.0 / f), 2.0, n - 3.0);
    Some(1.0 / (1.0 + 2.0 / (n - 3.0) * q))
}

/// Two-sided indicator of Mandel's h for p laboratories at significance
/// level α (Formula D.5; Tables 7 and 8).
pub fn mandel_h_indicator(p: usize, alpha: f64) -> f64 {
    assert!(p >= 3, "Mandel's h indicator needs p ≥ 3");
    let pf = p as f64;
    let t = student_t_quantile(1.0 - 0.5 * alpha, pf - 2.0);
    (pf - 1.0) * t / (pf * (pf - 2.0 + t * t)).sqrt()
}

/// One-sided upper indicator of Mandel's k for p laboratories with n results
/// each at significance level α (Formula D.6; Tables 7 and 8).
pub fn mandel_k_indicator(p: usize, n: usize, alpha: f64) -> f64 {
    assert!(p >= 2 && n >= 2, "Mandel's k indicator needs p ≥ 2 and n ≥ 2");
    let (pf, nf) = (p as f64, n as f64);
    let f = f_quantile(alpha, (pf - 1.0) * (nf - 1.0), nf - 1.0);
    (pf / (1.0 + (pf - 1.0) * f)).sqrt()
}

// ---------------------------------------------------------------------------
// Classification (8.3.3.1)
// ---------------------------------------------------------------------------

/// Outcome of an outlier test at the 5 % and 1 % levels (8.3.3.1 a).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Not beyond the 5 % critical value: accepted as correct.
    Correct,
    /// Beyond the 5 % but not the 1 % critical value (marked *).
    Straggler,
    /// Beyond the 1 % critical value (marked **).
    Outlier,
}

impl Outcome {
    /// For statistics that are significant when large.
    pub fn upper(statistic: f64, critical_5: f64, critical_1: f64) -> Self {
        if statistic > critical_1 {
            Outcome::Outlier
        } else if statistic > critical_5 {
            Outcome::Straggler
        } else {
            Outcome::Correct
        }
    }

    /// For statistics that are significant when small (Grubbs' double test).
    pub fn lower(statistic: f64, critical_5: f64, critical_1: f64) -> Self {
        if statistic < critical_1 {
            Outcome::Outlier
        } else if statistic < critical_5 {
            Outcome::Straggler
        } else {
            Outcome::Correct
        }
    }

    pub fn mark(self) -> &'static str {
        match self {
            Outcome::Correct => "",
            Outcome::Straggler => "*",
            Outcome::Outlier => "**",
        }
    }
}

// ---------------------------------------------------------------------------
// Cells (8.2.10, 8.2.11)
// ---------------------------------------------------------------------------

/// Entries of Forms B and C for one cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    /// Number of results n_ij.
    pub n: usize,
    /// Cell mean ȳ_ij (Formula 2).
    pub mean: f64,
    /// Intracell standard deviation s_ij (Formula 3); `None` for n_ij < 2
    /// (a dash in Form C).
    pub sd: Option<f64>,
}

impl Cell {
    /// Summarizes the results of one laboratory at one level; `None` for an
    /// empty cell.
    pub fn from_results(results: &[f64]) -> Option<Cell> {
        let n = results.len();
        if n == 0 {
            return None;
        }
        let mean = results.iter().sum::<f64>() / n as f64;
        let sd = (n >= 2).then(|| (results.iter().map(|y| (y - mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt());
        Some(Cell { n, mean, sd })
    }

    /// The cell with its mean and standard deviation rounded to `decimals`
    /// places, as recorded in Forms B and C.
    pub fn rounded(self, decimals: u32) -> Cell {
        let f = 10f64.powi(decimals as i32);
        let r = |x: f64| (x * f).round() / f;
        Cell { n: self.n, mean: r(self.mean), sd: self.sd.map(r) }
    }
}

/// The number of results in the majority of cells (8.3.4.3); on a tie the
/// larger number.
pub fn majority_n(cells: &[Cell]) -> usize {
    let mut best = (0, 0);
    for c in cells {
        let count = cells.iter().filter(|d| d.n == c.n).count();
        if count > best.1 || (count == best.1 && c.n > best.0) {
            best = (c.n, count);
        }
    }
    best.0
}

// ---------------------------------------------------------------------------
// General mean and variances (8.4)
// ---------------------------------------------------------------------------

/// Estimates of the general mean and the precision at one level.
#[derive(Clone, Debug, PartialEq)]
pub struct LevelEstimates {
    /// Number of laboratories p_j (non-empty cells).
    pub p: usize,
    /// General mean m̂_j (Formula 23, or Formula B.5 for REML).
    pub mean: f64,
    /// Repeatability standard deviation s_r.
    pub s_r: f64,
    /// Between-laboratory standard deviation s_L (zero when the estimate of
    /// its variance is negative, 8.4.5.4).
    pub s_l: f64,
    /// Reproducibility standard deviation s_R (Formula 31).
    pub s_rr: f64,
    /// Formula 27 and Formula 28 (ANOVA only).
    pub s_d: Option<f64>,
    pub n_bar: Option<f64>,
    /// Standard error of the REML mean (Formula B.7).
    pub u_mean: Option<f64>,
}

/// General mean and variances from Forms B and C (8.4.4, 8.4.5). Cells with a
/// single result count in the mean and in s_d but not in s_r (8.4.3 b).
pub fn level_estimates(cells: &[Cell]) -> Result<LevelEstimates, UncError> {
    let p = cells.len();
    if p < 2 {
        return Err(invalid("at least two laboratories are needed at each level"));
    }
    let n_tot: f64 = cells.iter().map(|c| c.n as f64).sum();
    let mean = cells.iter().map(|c| c.n as f64 * c.mean).sum::<f64>() / n_tot;
    let dof: f64 = cells.iter().filter(|c| c.sd.is_some()).map(|c| (c.n - 1) as f64).sum();
    if dof == 0.0 {
        return Err(invalid("no cell with two or more results: the repeatability cannot be estimated"));
    }
    let sr2 = cells.iter().filter_map(|c| c.sd.map(|s| (c.n - 1) as f64 * s * s)).sum::<f64>() / dof;
    let pf = p as f64;
    let sd2 = cells.iter().map(|c| c.n as f64 * (c.mean - mean).powi(2)).sum::<f64>() / (pf - 1.0);
    let n_bar = (n_tot - cells.iter().map(|c| (c.n * c.n) as f64).sum::<f64>() / n_tot) / (pf - 1.0);
    let sl2 = ((sd2 - sr2) / n_bar).max(0.0);
    Ok(LevelEstimates {
        p,
        mean,
        s_r: sr2.sqrt(),
        s_l: sl2.sqrt(),
        s_rr: (sr2 + sl2).sqrt(),
        s_d: Some(sd2.sqrt()),
        n_bar: Some(n_bar),
        u_mean: None,
    })
}

/// Restricted maximum likelihood estimates for the one-way random-effects
/// model of a level (Annex B.2), from the results of each laboratory.
///
/// With γ = σ_L²/σ_r², the restricted log-likelihood (B.4) is maximized over
/// σ_r² in closed form, σ_r² = Q(γ)/(N − 1), and over γ ≥ 0 numerically;
/// Q(γ) = Σ(y − ȳ_i)² + Σ w_i(ȳ_i − x̂)² with w_i = n_i/(1 + n_iγ).
pub fn reml_estimates(cells: &[&[f64]]) -> Result<LevelEstimates, UncError> {
    let cells: Vec<&[f64]> = cells.iter().copied().filter(|c| !c.is_empty()).collect();
    let p = cells.len();
    if p < 2 {
        return Err(invalid("at least two laboratories are needed at each level"));
    }
    let n: Vec<f64> = cells.iter().map(|c| c.len() as f64).collect();
    let mean_i: Vec<f64> = cells.iter().map(|c| c.iter().sum::<f64>() / c.len() as f64).collect();
    let ssw: f64 = cells.iter().zip(&mean_i).map(|(c, m)| c.iter().map(|y| (y - m).powi(2)).sum::<f64>()).sum();
    let n_tot: f64 = n.iter().sum();
    if n_tot - p as f64 <= 0.0 {
        return Err(invalid("no cell with two or more results: the repeatability cannot be estimated"));
    }
    // −2 × profile restricted log-likelihood, without constants.
    let weighted = |g: f64| {
        let w: Vec<f64> = n.iter().map(|ni| ni / (1.0 + ni * g)).collect();
        let sw: f64 = w.iter().sum();
        let mu = w.iter().zip(&mean_i).map(|(w, m)| w * m).sum::<f64>() / sw;
        let q = ssw + w.iter().zip(&mean_i).map(|(w, m)| w * (m - mu).powi(2)).sum::<f64>();
        (q, sw, mu)
    };
    let objective = |g: f64| {
        let (q, sw, _) = weighted(g);
        (n_tot - 1.0) * q.ln() + n.iter().map(|ni| (1.0 + ni * g).ln()).sum::<f64>() + sw.ln()
    };
    // Grid over ln γ, then golden-section refinement around the best point.
    let (lo_t, hi_t, steps) = (-30.0_f64, 30.0_f64, 600);
    let h = (hi_t - lo_t) / steps as f64;
    let mut best = (0usize, f64::INFINITY);
    for i in 0..=steps {
        let v = objective((lo_t + h * i as f64).exp());
        if v < best.1 {
            best = (i, v);
        }
    }
    let at_zero = objective(0.0);
    let gamma = if at_zero <= best.1 {
        0.0
    } else {
        let (mut a, mut b) = (lo_t + h * (best.0.max(1) - 1) as f64, lo_t + h * (best.0 + 1).min(steps) as f64);
        let r = 0.5 * (5f64.sqrt() - 1.0);
        let (mut c, mut d) = (b - r * (b - a), a + r * (b - a));
        let (mut fc, mut fd) = (objective(c.exp()), objective(d.exp()));
        for _ in 0..200 {
            if fc < fd {
                b = d;
                d = c;
                fd = fc;
                c = b - r * (b - a);
                fc = objective(c.exp());
            } else {
                a = c;
                c = d;
                fc = fd;
                d = a + r * (b - a);
                fd = objective(d.exp());
            }
            if b - a < 1e-12 {
                break;
            }
        }
        (0.5 * (a + b)).exp()
    };
    let (q, sw, mu) = weighted(gamma);
    let sr2 = q / (n_tot - 1.0);
    let sl2 = gamma * sr2;
    Ok(LevelEstimates {
        p,
        mean: mu,
        s_r: sr2.sqrt(),
        s_l: sl2.sqrt(),
        s_rr: (sr2 + sl2).sqrt(),
        s_d: None,
        n_bar: None,
        // Σw_i of Formula B.6 is Σ n_i/(σ_r²(1 + n_iγ)).
        u_mean: Some((sr2 / sw).sqrt()),
    })
}

// ---------------------------------------------------------------------------
// Scrutiny (8.3)
// ---------------------------------------------------------------------------

/// Mandel's between-laboratory consistency statistic h for each cell
/// (Formula 6), with deviations from the general mean m̂ (8.4.4).
pub fn mandel_h(cells: &[Cell]) -> Vec<f64> {
    let p = cells.len();
    if p < 2 {
        return vec![f64::NAN; p];
    }
    let n_tot: f64 = cells.iter().map(|c| c.n as f64).sum();
    let m = cells.iter().map(|c| c.n as f64 * c.mean).sum::<f64>() / n_tot;
    let s = (cells.iter().map(|c| (c.mean - m).powi(2)).sum::<f64>() / (p - 1) as f64).sqrt();
    cells.iter().map(|c| if s > 0.0 { (c.mean - m) / s } else { 0.0 }).collect()
}

/// Mandel's within-laboratory consistency statistic k for each cell
/// (Formulae 7 and 8); `None` where the cell has no standard deviation.
pub fn mandel_k(cells: &[Cell]) -> Vec<Option<f64>> {
    let sds: Vec<f64> = cells.iter().filter_map(|c| c.sd).collect();
    let sum: f64 = sds.iter().map(|s| s * s).sum();
    let p = sds.len() as f64;
    cells.iter().map(|c| c.sd.map(|s| if sum > 0.0 { s * p.sqrt() / sum.sqrt() } else { 0.0 })).collect()
}

/// One application of Cochran's test (8.3.4).
#[derive(Clone, Debug, PartialEq)]
pub struct CochranTest {
    /// Index (into the cells given) of the highest standard deviation.
    pub cell: usize,
    pub statistic: f64,
    /// Number of standard deviations tested and the number of results used
    /// for the critical values (majority, 8.3.4.3).
    pub p: usize,
    pub n: usize,
    pub critical_5: f64,
    pub critical_1: f64,
    pub outcome: Outcome,
}

/// Cochran's test of the cell standard deviations (Formula 9), repeated
/// without the highest value as long as it is an outlier (8.3.4.6). Empty
/// when fewer than two standard deviations are available or all are zero.
pub fn cochran_tests(cells: &[Cell]) -> Vec<CochranTest> {
    let mut active: Vec<usize> = (0..cells.len()).filter(|&i| cells[i].sd.is_some()).collect();
    let mut out = Vec::new();
    while active.len() >= 2 {
        let tested: Vec<Cell> = active.iter().map(|&i| cells[i]).collect();
        let n = majority_n(&tested);
        let sum: f64 = tested.iter().map(|c| c.sd.unwrap_or(0.0).powi(2)).sum();
        if n < 2 || !(sum > 0.0) {
            break;
        }
        let (k, smax) = active
            .iter()
            .map(|&i| (i, cells[i].sd.unwrap_or(0.0)))
            .fold((usize::MAX, f64::NEG_INFINITY), |b, x| if x.1 > b.1 { x } else { b });
        let statistic = smax * smax / sum;
        let p = active.len();
        let (critical_5, critical_1) = (cochran_critical(p, n, 0.05), cochran_critical(p, n, 0.01));
        let outcome = Outcome::upper(statistic, critical_5, critical_1);
        out.push(CochranTest { cell: k, statistic, p, n, critical_5, critical_1, outcome });
        if outcome != Outcome::Outlier {
            break;
        }
        active.retain(|&i| i != k);
    }
    out
}

/// Grubbs' test for one outlying observation (Formulae 10–13).
#[derive(Clone, Debug, PartialEq)]
pub struct GrubbsSingle {
    /// Index of the tested value.
    pub index: usize,
    /// Number of values in the tested set.
    pub p: usize,
    pub statistic: f64,
    pub critical_5: f64,
    pub critical_1: f64,
    pub outcome: Outcome,
}

/// Grubbs' test for two outlying observations (Formulae 14–20).
#[derive(Clone, Debug, PartialEq)]
pub struct GrubbsDouble {
    pub indices: [usize; 2],
    pub statistic: f64,
    pub critical_5: f64,
    pub critical_1: f64,
    pub outcome: Outcome,
}

/// Grubbs' tests applied to the cell means of one level (8.3.5.3 a).
#[derive(Clone, Debug, PartialEq)]
pub struct GrubbsTests {
    pub low: GrubbsSingle,
    pub high: GrubbsSingle,
    /// When a single test finds an outlier, the single test of the other
    /// extreme with that value excluded; the double tests are then not
    /// applied.
    pub repeated: Option<GrubbsSingle>,
    pub double_low: Option<GrubbsDouble>,
    pub double_high: Option<GrubbsDouble>,
}

fn grubbs_single(values: &[(usize, f64)], high: bool) -> Option<GrubbsSingle> {
    let p = values.len();
    if p < 3 {
        return None;
    }
    let mean = values.iter().map(|v| v.1).sum::<f64>() / p as f64;
    let s = (values.iter().map(|v| (v.1 - mean).powi(2)).sum::<f64>() / (p - 1) as f64).sqrt();
    let pick = |a: &&(usize, f64), b: &&(usize, f64)| a.1.total_cmp(&b.1);
    let &(index, x) = if high { values.iter().max_by(pick)? } else { values.iter().min_by(pick)? };
    let statistic = if s > 0.0 { (if high { x - mean } else { mean - x }) / s } else { 0.0 };
    let (critical_5, critical_1) = (grubbs_single_critical(p, 0.05), grubbs_single_critical(p, 0.01));
    Some(GrubbsSingle {
        index,
        p,
        statistic,
        critical_5,
        critical_1,
        outcome: Outcome::upper(statistic, critical_5, critical_1),
    })
}

fn grubbs_double(values: &[(usize, f64)], high: bool) -> Option<GrubbsDouble> {
    let p = values.len();
    let (critical_5, critical_1) = (grubbs_double_critical(p, 0.05)?, grubbs_double_critical(p, 0.01)?);
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.1.total_cmp(&b.1));
    let ss = |v: &[(usize, f64)]| {
        let m = v.iter().map(|x| x.1).sum::<f64>() / v.len() as f64;
        v.iter().map(|x| (x.1 - m).powi(2)).sum::<f64>()
    };
    let s0 = ss(&sorted);
    let (rest, indices) = if high {
        (&sorted[..p - 2], [sorted[p - 1].0, sorted[p - 2].0])
    } else {
        (&sorted[2..], [sorted[0].0, sorted[1].0])
    };
    let statistic = if s0 > 0.0 { ss(rest) / s0 } else { 1.0 };
    Some(GrubbsDouble {
        indices,
        statistic,
        critical_5,
        critical_1,
        outcome: Outcome::lower(statistic, critical_5, critical_1),
    })
}

/// Grubbs' tests of a set of cell means following 8.3.5.3 a): the single
/// tests of the lowest and the highest value; if one of them is an outlier,
/// the single test of the other extreme without it, otherwise the double
/// tests. `None` for fewer than three values.
pub fn grubbs_tests(values: &[f64]) -> Option<GrubbsTests> {
    let indexed: Vec<(usize, f64)> = values.iter().copied().enumerate().collect();
    let low = grubbs_single(&indexed, false)?;
    let high = grubbs_single(&indexed, true)?;
    let outlier = |t: &GrubbsSingle| t.outcome == Outcome::Outlier;
    let first = match (outlier(&low), outlier(&high)) {
        (false, false) => None,
        (true, false) => Some((low.index, true)),
        (false, true) => Some((high.index, false)),
        // Both extremes: the more extreme one is excluded first.
        (true, true) => Some(if high.statistic >= low.statistic { (high.index, false) } else { (low.index, true) }),
    };
    let (repeated, double_low, double_high) = match first {
        Some((excluded, test_high)) => {
            let rest: Vec<(usize, f64)> = indexed.iter().copied().filter(|v| v.0 != excluded).collect();
            (grubbs_single(&rest, test_high), None, None)
        }
        None => (None, grubbs_double(&indexed, false), grubbs_double(&indexed, true)),
    };
    Some(GrubbsTests { low, high, repeated, double_low, double_high })
}

// ---------------------------------------------------------------------------
// Functional relationships (8.5)
// ---------------------------------------------------------------------------

/// The four relationships between a standard deviation s and the mean m
/// (8.5.1.3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Relationship {
    /// I: s = b·m.
    Proportional { b: f64 },
    /// II: s = a + b·m.
    Linear { a: f64, b: f64 },
    /// III: s² = a_v² + (b_v·m)², stored as the fitted a_v² and b_v²; a_v²
    /// may come out negative.
    Quadratic { a2: f64, b2: f64 },
    /// IV: lg s = c + d·lg m, i.e. s = 10^c·m^d.
    Power { c: f64, d: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelationshipKind {
    Proportional,
    Linear,
    Quadratic,
    Power,
}

impl Relationship {
    /// The fitted standard deviation at mean m.
    pub fn sd_at(&self, m: f64) -> f64 {
        match *self {
            Relationship::Proportional { b } => b * m,
            Relationship::Linear { a, b } => a + b * m,
            Relationship::Quadratic { a2, b2 } => (a2 + b2 * m * m).max(0.0).sqrt(),
            Relationship::Power { c, d } => 10f64.powf(c) * m.powf(d),
        }
    }

    pub fn kind(&self) -> RelationshipKind {
        match self {
            Relationship::Proportional { .. } => RelationshipKind::Proportional,
            Relationship::Linear { .. } => RelationshipKind::Linear,
            Relationship::Quadratic { .. } => RelationshipKind::Quadratic,
            Relationship::Power { .. } => RelationshipKind::Power,
        }
    }

    /// The parameters as printed: b (I); a, b (II); a_v, b_v (III, a_v NaN
    /// when a_v² < 0); c, d (IV).
    pub fn parameters(&self) -> Vec<f64> {
        match *self {
            Relationship::Proportional { b } => vec![b],
            Relationship::Linear { a, b } => vec![a, b],
            Relationship::Quadratic { a2, b2 } => vec![a2.sqrt(), b2.sqrt()],
            Relationship::Power { c, d } => vec![c, d],
        }
    }
}

/// Weighted straight line y = a + b·x (Formulae 32–38 and 43–49).
fn weighted_line(x: &[f64], y: &[f64], w: &[f64]) -> Result<(f64, f64), UncError> {
    let t1: f64 = w.iter().sum();
    let t2: f64 = w.iter().zip(x).map(|(w, x)| w * x).sum();
    let t3: f64 = w.iter().zip(x).map(|(w, x)| w * x * x).sum();
    let t4: f64 = w.iter().zip(y).map(|(w, y)| w * y).sum();
    let t5: f64 = w.iter().zip(x).zip(y).map(|((w, x), y)| w * x * y).sum();
    let det = t1 * t3 - t2 * t2;
    if !(det.abs() > 1e-300) || !det.is_finite() {
        return Err(invalid("the levels do not determine a straight line"));
    }
    Ok(((t3 * t4 - t2 * t5) / det, (t1 * t5 - t2 * t4) / det))
}

/// Fits a relationship to the standard deviations s at the means m of q
/// levels by the procedures of 8.5.2 to 8.5.4: relationship I in closed form
/// (Formula 39); II and III by weighted regression with weights from the
/// observed values and then once more from the fitted values (the second
/// fit is the final result); IV by unweighted regression of lg s on lg m.
pub fn fit_relationship(kind: RelationshipKind, m: &[f64], s: &[f64]) -> Result<Relationship, UncError> {
    let q = m.len();
    if q != s.len() {
        return Err(invalid("means and standard deviations differ in number"));
    }
    let needed = if kind == RelationshipKind::Proportional { 1 } else { 3 };
    if q < needed {
        return Err(invalid(format!("at least {needed} levels are needed for this relationship")));
    }
    if s.iter().any(|v| !(*v > 0.0)) {
        return Err(invalid("all standard deviations must be positive"));
    }
    let positive_m = m.iter().all(|v| *v > 0.0);
    match kind {
        RelationshipKind::Proportional => {
            if !positive_m {
                return Err(invalid("relationship I needs positive means"));
            }
            Ok(Relationship::Proportional { b: s.iter().zip(m).map(|(s, m)| s / m).sum::<f64>() / q as f64 })
        }
        RelationshipKind::Linear => {
            let w0: Vec<f64> = s.iter().map(|s| 1.0 / (s * s)).collect();
            let (a1, b1) = weighted_line(m, s, &w0)?;
            let fitted: Vec<f64> = m.iter().map(|m| a1 + b1 * m).collect();
            if fitted.iter().any(|v| !(*v > 0.0)) {
                return Err(invalid("relationship II gives a non-positive standard deviation at a level"));
            }
            let w1: Vec<f64> = fitted.iter().map(|s| 1.0 / (s * s)).collect();
            let (a, b) = weighted_line(m, s, &w1)?;
            Ok(Relationship::Linear { a, b })
        }
        RelationshipKind::Quadratic => {
            let x: Vec<f64> = m.iter().map(|m| m * m).collect();
            let y: Vec<f64> = s.iter().map(|s| s * s).collect();
            let w0: Vec<f64> = y.iter().map(|v| 1.0 / (v * v)).collect();
            let (a1, b1) = weighted_line(&x, &y, &w0)?;
            let fitted: Vec<f64> = x.iter().map(|x| a1 + b1 * x).collect();
            if fitted.iter().any(|v| !(*v > 0.0)) {
                return Err(invalid("relationship III gives a non-positive variance at a level"));
            }
            let w1: Vec<f64> = fitted.iter().map(|v| 1.0 / (v * v)).collect();
            let (a2, b2) = weighted_line(&x, &y, &w1)?;
            Ok(Relationship::Quadratic { a2, b2 })
        }
        RelationshipKind::Power => {
            if !positive_m {
                return Err(invalid("relationship IV needs positive means"));
            }
            let x: Vec<f64> = m.iter().map(|m| m.log10()).collect();
            let y: Vec<f64> = s.iter().map(|s| s.log10()).collect();
            let (c, d) = weighted_line(&x, &y, &vec![1.0; q])?;
            Ok(Relationship::Power { c, d })
        }
    }
}

// ---------------------------------------------------------------------------
// Annex A
// ---------------------------------------------------------------------------

/// A_r: the relative amount by which s_r can be expected to differ from σ_r
/// with a probability of about 95 %, for p laboratories with n results each
/// (Formula A.3).
pub fn uncertainty_factor_repeatability(p: usize, n: usize) -> f64 {
    1.96 * (1.0 / (2.0 * p as f64 * (n as f64 - 1.0))).sqrt()
}

/// A_R for s_R, with γ = σ_R/σ_r (Formula A.4).
pub fn uncertainty_factor_reproducibility(p: usize, n: usize, gamma: f64) -> f64 {
    let (p, n) = (p as f64, n as f64);
    let g2 = gamma * gamma;
    let num = p * (1.0 + n * (g2 - 1.0)).powi(2) + (n - 1.0) * (p - 1.0);
    1.96 * (num / (2.0 * g2 * g2 * n * n * (p - 1.0) * p)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn critical_values_reproduce_tables_5_to_8() {
        // Table 5 (Cochran).
        assert!(close(cochran_critical(8, 3, 0.05), 0.516, 5e-4));
        assert!(close(cochran_critical(8, 3, 0.01), 0.615, 5e-4));
        assert!(close(cochran_critical(9, 2, 0.05), 0.638, 5e-4));
        assert!(close(cochran_critical(9, 2, 0.01), 0.754, 5e-4));
        assert!(close(cochran_critical(8, 2, 0.05), 0.680, 5e-4));
        // Table 6, single test, by Formula D.2 (the table differs by up to 0.001 in the last digit).
        for (p, c1, c5) in
            [(8, 2.274, 2.126), (9, 2.387, 2.215), (15, 2.806, 2.549), (16, 2.852, 2.585), (40, 3.381, 3.036)]
        {
            assert!(close(grubbs_single_critical(p, 0.01), c1, 1e-3), "p = {p}");
            assert!(close(grubbs_single_critical(p, 0.05), c5, 1e-3), "p = {p}");
        }
        // Table 6, double test, and the approximation D.3 within ±0.003.
        assert_eq!(grubbs_double_critical(9, 0.05), Some(0.1492));
        assert_eq!(grubbs_double_critical(3, 0.05), None);
        for p in 6..=40 {
            let (c1, c5) = GRUBBS_DOUBLE[p - 4];
            let a = |alpha: f64| {
                let a = 0.5 * alpha;
                let g = GRUBBS_D1.iter().find(|(x, _)| (x - a).abs() < 1e-12).unwrap().1;
                let n = p as f64;
                let f = g[0] + g[1] * n + g[2] * n * n;
                1.0 / (1.0 + 2.0 / (n - 3.0) * f_quantile((1.0 - a).powf(1.0 / f), 2.0, n - 3.0))
            };
            assert!(close(a(0.01), c1, 3e-3), "p = {p}: {} vs {c1}", a(0.01));
            assert!(close(a(0.05), c5, 3e-3), "p = {p}: {} vs {c5}", a(0.05));
        }
        assert!(grubbs_double_critical(50, 0.05).unwrap() > 0.6445);
        // Tables 7 and 8 (Mandel's indicators), p = 9.
        assert!(close(mandel_h_indicator(9, 0.01), 2.13, 5e-3));
        assert!(close(mandel_h_indicator(9, 0.05), 1.78, 5e-3));
        assert!(close(mandel_k_indicator(9, 2, 0.01), 2.29, 5e-3));
        assert!(close(mandel_k_indicator(9, 2, 0.05), 1.90, 5e-3));
    }

    /// ISO 5725-2:2019 C.3, levels 3 and 4 (thermometric titration of creosote
    /// oil): the cell means with laboratory 1 outlying.
    #[test]
    fn grubbs_procedure_c3() {
        let level3 = [17.15, 14.46, 13.60, 14.40, 13.825, 13.98, 14.15, 14.84, 14.17];
        let g = grubbs_tests(&level3).unwrap();
        assert!(close(g.low.statistic, 0.86, 0.005) && close(g.high.statistic, 2.50, 0.005));
        assert_eq!(g.high.outcome, Outcome::Outlier);
        assert_eq!(g.high.index, 0);
        assert!(g.double_low.is_none() && g.double_high.is_none());
        let r = g.repeated.unwrap();
        assert_eq!((r.p, r.index), (8, 2));
        let level1 = [4.415, 4.13, 3.70, 4.10, 4.005, 3.89, 3.75, 3.905, 4.045];
        let g = grubbs_tests(&level1).unwrap();
        assert!(close(g.low.statistic, 1.356, 0.002) && close(g.high.statistic, 1.949, 0.002));
        assert!(close(g.double_low.unwrap().statistic, 0.502, 0.002));
        assert!(close(g.double_high.unwrap().statistic, 0.356, 0.002));
        assert!(g.repeated.is_none());
    }

    #[test]
    fn variances_and_reml_c3_level_4() {
        // Laboratories 2–9 (laboratory 1 excluded), Table C.14.
        let cells: [&[f64]; 8] = [
            &[16.06, 16.22],
            &[14.50, 15.10],
            &[15.60, 15.50],
            &[15.54, 15.78],
            &[16.42, 16.58],
            &[14.90, 16.00],
            &[15.41, 15.22],
            &[15.14, 15.44],
        ];
        let forms: Vec<Cell> = cells.iter().map(|c| Cell::from_results(c).unwrap()).collect();
        let e = level_estimates(&forms).unwrap();
        assert!(close(e.mean, 15.588, 5e-4) && close(e.s_r, 0.3368, 5e-5) && close(e.s_rr, 0.5786, 5e-5));
        let r = reml_estimates(&cells).unwrap();
        assert!(close(r.mean, 15.59, 0.005) && close(r.s_r, 0.337, 5e-4) && close(r.s_rr, 0.579, 5e-4));
        let c = cochran_tests(&forms);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].outcome, Outcome::Correct);
    }

    #[test]
    fn reml_unbalanced_and_boundary() {
        // ISO 5725-2:2019 C.1, level 2 (Table C.6: 1.254, 0.029, 0.062).
        let cells: [&[f64]; 8] = [
            &[1.20, 1.18, 1.23, 1.21],
            &[1.22, 1.21, 1.22],
            &[1.28, 1.31, 1.30],
            &[1.23, 1.18, 1.20],
            &[1.31, 1.22, 1.22, 1.24],
            &[1.39, 1.36, 1.37],
            &[1.20, 1.26, 1.26],
            &[1.24, 1.22, 1.30],
        ];
        let r = reml_estimates(&cells).unwrap();
        assert!(close(r.mean, 1.254, 5e-4) && close(r.s_r, 0.029, 5e-4) && close(r.s_rr, 0.062, 5e-4));
        // Identical cell means: the between-laboratory variance is zero.
        let flat: [&[f64]; 3] = [&[1.0, 2.0], &[1.0, 2.0], &[1.5, 1.5]];
        let r = reml_estimates(&flat).unwrap();
        assert_eq!(r.s_l, 0.0);
        assert!(close(r.mean, 1.5, 1e-12));
    }

    #[test]
    fn relationships_tables_1_to_4() {
        let m = [3.94, 8.28, 14.18, 15.59, 20.41];
        let s = [0.092, 0.179, 0.127, 0.337, 0.393];
        let p = |k| fit_relationship(k, &m, &s).unwrap().parameters();
        assert!(close(p(RelationshipKind::Proportional)[0], 0.019, 5e-4));
        let ii = p(RelationshipKind::Linear);
        assert!(close(ii[0], 0.030, 5e-4) && close(ii[1], 0.0155, 1e-4));
        let iii = p(RelationshipKind::Quadratic);
        assert!(close(iii[0], 0.061, 5e-4) && close(iii[1], 0.0178, 5e-5));
        let iv = p(RelationshipKind::Power);
        assert!(close(iv[0], -1.5075, 5e-4) && close(iv[1], 0.770, 5e-4));
    }

    #[test]
    fn annex_a_table_a1() {
        assert!(close(uncertainty_factor_repeatability(5, 2), 0.62, 5e-3));
        assert!(close(uncertainty_factor_repeatability(40, 4), 0.13, 5e-3));
        assert!(close(uncertainty_factor_reproducibility(5, 2, 1.0), 0.46, 5e-3));
        assert!(close(uncertainty_factor_reproducibility(10, 3, 2.0), 0.39, 5e-3));
        assert!(close(uncertainty_factor_reproducibility(40, 4, 5.0), 0.22, 5e-3));
    }

    #[test]
    fn majority_and_mandel() {
        let cells: Vec<Cell> = [vec![1.0, 2.0, 3.0], vec![2.0, 2.5, 3.0], vec![1.0, 1.5, 2.0, 2.5]]
            .iter()
            .map(|r| Cell::from_results(r).unwrap())
            .collect();
        assert_eq!(majority_n(&cells), 3);
        let h = mandel_h(&cells);
        assert!(h[1] > 0.0 && h[2] < 0.0);
        let k = mandel_k(&cells);
        assert!(k.iter().all(|x| x.is_some()));
        let sum: f64 = k.iter().map(|x| x.unwrap().powi(2)).sum();
        assert!(close(sum, 3.0, 1e-12));
    }
}
