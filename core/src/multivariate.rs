//! Models with any number of output quantities — JCGM 102:2011 (GUM
//! Supplement 2).
//!
//! * explicit models Y = f(X) and implicit models h(Y, X) = 0 (6.2, 6.3);
//! * the generalized law of propagation of uncertainty U_y = C_x U_x C_xᵀ, for
//!   implicit models C_y U_y C_yᵀ = C_x U_x C_xᵀ (formulae 3 and 8);
//! * hyper-ellipsoidal and hyper-rectangular coverage regions for the GUM
//!   framework (6.5) and from Monte Carlo draws (7.7.2, 7.7.3), and the
//!   approximation to the smallest coverage region for two outputs (7.7.4);
//! * the adaptive Monte Carlo procedure (7.8.3) and the validation of the
//!   GUM framework (clause 8);
//! * input quantities characterized jointly by a multivariate t-distribution
//!   from a series of vector indication values (5.3.2).

use crate::error::UncError;
use crate::expr::{self, Expr, Program};
use crate::mcm::{cholesky, coverage_count, numerical_tolerance, Sampler, StudentGroup};
use crate::model::{Correlation, InputQuantity, Model};
use crate::rng::Rng;
use crate::special::{chi_squared_quantile, normal_quantile};
use crate::Distribution;
use std::collections::HashMap;

fn invalid(msg: impl Into<String>) -> UncError {
    UncError::InvalidOption(msg.into())
}

// ---------------------------------------------------------------------------
// Linear algebra for small matrices
// ---------------------------------------------------------------------------

type Matrix = Vec<Vec<f64>>;

fn mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
    let (n, k, m) = (a.len(), b.len(), b.first().map_or(0, Vec::len));
    let mut c = vec![vec![0.0; m]; n];
    for i in 0..n {
        for l in 0..k {
            let ail = a[i][l];
            if ail != 0.0 {
                for j in 0..m {
                    c[i][j] += ail * b[l][j];
                }
            }
        }
    }
    c
}

fn transpose(a: &Matrix) -> Matrix {
    let m = a.first().map_or(0, Vec::len);
    (0..m).map(|j| a.iter().map(|row| row[j]).collect()).collect()
}

/// Solves A X = B by Gaussian elimination with partial pivoting.
fn solve(a: &Matrix, b: &Matrix) -> Option<Matrix> {
    let n = a.len();
    let m = b.first().map_or(0, Vec::len);
    let mut a = a.clone();
    let mut b = b.clone();
    for c in 0..n {
        let piv = (c..n).max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))?;
        if !(a[piv][c].abs() > 0.0) || !a[piv][c].is_finite() {
            return None;
        }
        a.swap(c, piv);
        b.swap(c, piv);
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
    for r in (0..n).rev() {
        for k in 0..m {
            let mut s = b[r][k];
            for j in r + 1..n {
                s -= a[r][j] * x[j][k];
            }
            x[r][k] = s / a[r][r];
        }
    }
    Some(x)
}

/// Eigenvalues of a symmetric matrix by cyclic Jacobi rotations.
pub fn symmetric_eigenvalues(a: &Matrix) -> Vec<f64> {
    let n = a.len();
    let mut a = a.clone();
    for _ in 0..100 {
        let off: f64 = (0..n)
            .flat_map(|i| (0..n).filter(move |&j| j != i).map(move |j| (i, j)))
            .map(|(i, j)| a[i][j] * a[i][j])
            .sum();
        if off < 1e-30 {
            break;
        }
        for p in 0..n {
            for q in p + 1..n {
                if a[p][q].abs() < 1e-300 {
                    continue;
                }
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let t = if theta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for k in 0..n {
                    let (akp, akq) = (a[k][p], a[k][q]);
                    a[k][p] = c * akp - s * akq;
                    a[k][q] = s * akp + c * akq;
                }
                for k in 0..n {
                    let (apk, aqk) = (a[p][k], a[q][k]);
                    a[p][k] = c * apk - s * aqk;
                    a[q][k] = s * apk + c * aqk;
                }
            }
        }
    }
    let mut ev: Vec<f64> = (0..n).map(|i| a[i][i]).collect();
    ev.sort_by(f64::total_cmp);
    ev
}

/// Standard uncertainties and correlation matrix of a covariance matrix.
fn correlations_of(cov: &Matrix) -> (Vec<f64>, Matrix) {
    let u: Vec<f64> = (0..cov.len()).map(|i| cov[i][i].max(0.0).sqrt()).collect();
    let r = (0..cov.len())
        .map(|i| {
            (0..cov.len())
                .map(|j| {
                    if i == j {
                        1.0
                    } else if u[i] > 0.0 && u[j] > 0.0 {
                        cov[i][j] / (u[i] * u[j])
                    } else {
                        0.0
                    }
                })
                .collect()
        })
        .collect();
    (u, r)
}

// ---------------------------------------------------------------------------
// Coverage factors for Gaussian output quantities (6.5.3, Tables 1 and 2)
// ---------------------------------------------------------------------------

/// k_p of the hyper-ellipsoidal coverage region for m jointly Gaussian
/// quantities: k_p² is the p-quantile of χ²_m [6.5.3 a), Table 1].
pub fn ellipsoid_coverage_factor(m: usize, p: f64) -> f64 {
    chi_squared_quantile(p, m as f64).sqrt()
}

/// k_q of the hyper-rectangular coverage region: each interval has coverage
/// probability q = 1 − (1 − p)/m [6.5.3 b), Table 2].
pub fn rectangle_coverage_factor(m: usize, p: f64) -> f64 {
    normal_quantile(1.0 - 0.5 * (1.0 - p) / m as f64)
}

/// Size of the coverage region for the expectation of a multivariate Gaussian
/// distribution from n vector indication values (6.5.4): the region is
/// (A − a)ᵀ v⁻¹ (A − a) ≤ k², v the covariance matrix with divisor n, and
/// k² = F_{m, n−m}(p) · m/(n − m).
pub fn expectation_region_factor(n: usize, m: usize, p: f64) -> Result<f64, UncError> {
    if n <= m {
        return Err(invalid("n must exceed the dimension m"));
    }
    let f = crate::special::f_quantile(p, m as f64, (n - m) as f64);
    Ok(f * m as f64 / (n - m) as f64)
}

// ---------------------------------------------------------------------------
// Input quantities characterized by vector indication values (5.3.2, 9.4)
// ---------------------------------------------------------------------------

/// Covariance matrix used by the GUM framework for a group of input
/// quantities obtained as averages of n vector indication values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationCovariance {
    /// U_x = M/(n(n − 1)) [GUM 5.2.3; JCGM 102 9.4.2.2].
    Mean,
    /// V(X) = M/((ν − 2)n), ν = n − N, the covariance matrix of the
    /// multivariate t-distribution [JCGM 102 5.3.2.3, 9.4.2.5].
    StudentT,
}

/// Summary of n vector indication values of N quantities: the average x̄, the
/// matrix M of sums of squares and products, n and ν = n − N.
#[derive(Clone, Debug, PartialEq)]
pub struct Indications {
    pub mean: Vec<f64>,
    pub sums_of_squares: Matrix,
    pub n: usize,
    pub dof: f64,
}

impl Indications {
    pub fn from_rows(rows: &[Vec<f64>]) -> Result<Self, UncError> {
        let n = rows.len();
        let dim = rows.first().map_or(0, Vec::len);
        if dim == 0 || rows.iter().any(|r| r.len() != dim) {
            return Err(invalid("indication values: every row needs one value per quantity"));
        }
        if n <= dim {
            return Err(invalid(format!(
                "indication values: n = {n} sets of {dim} quantities; n > N is needed (5.3.2.1)"
            )));
        }
        if rows.iter().flatten().any(|v| !v.is_finite()) {
            return Err(invalid("indication values must be finite"));
        }
        let mean: Vec<f64> = (0..dim).map(|j| rows.iter().map(|r| r[j]).sum::<f64>() / n as f64).collect();
        let mut m = vec![vec![0.0; dim]; dim];
        for r in rows {
            for a in 0..dim {
                for b in 0..dim {
                    m[a][b] += (r[a] - mean[a]) * (r[b] - mean[b]);
                }
            }
        }
        Ok(Indications { mean, sums_of_squares: m, n, dof: (n - dim) as f64 })
    }

    /// Covariance matrix for the GUM framework.
    pub fn covariance(&self, kind: ObservationCovariance) -> Result<Matrix, UncError> {
        let n = self.n as f64;
        let d = match kind {
            ObservationCovariance::Mean => n * (n - 1.0),
            ObservationCovariance::StudentT => {
                if !(self.dof > 2.0) {
                    return Err(invalid(
                        "the multivariate t-distribution has a covariance matrix only for ν = n − N > 2",
                    ));
                }
                (self.dof - 2.0) * n
            }
        };
        Ok(self.sums_of_squares.iter().map(|r| r.iter().map(|v| v / d).collect()).collect())
    }

    /// Scale matrix S/n of t_ν(x̄, S/n), S = M/ν (5.3.2.1).
    pub fn scale(&self) -> Matrix {
        let d = self.dof * self.n as f64;
        self.sums_of_squares.iter().map(|r| r.iter().map(|v| v / d).collect()).collect()
    }
}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct OutputQuantity {
    pub name: String,
    pub unit: Option<String>,
    /// f_j(X) for an explicit model, h_j(Y, X) for an implicit one.
    pub source: String,
    pub expr: Expr,
    /// Starting value for solving an implicit model.
    pub start: Option<f64>,
}

/// A measurement model with m output quantities. Variables 0 … N−1 of the
/// expressions are the inputs; in an implicit model N … N+m−1 are the
/// outputs.
#[derive(Clone, Debug)]
pub struct MultivariateModel {
    pub inputs: Vec<InputQuantity>,
    pub correlation: Matrix,
    pub outputs: Vec<OutputQuantity>,
    pub implicit: bool,
    pub intermediates: Vec<(String, String)>,
    /// Groups drawn from multivariate t-distributions in the Monte Carlo
    /// method.
    students: Vec<StudentGroup>,
    /// Covariances fixed by indication values (replace u_i u_j r_ij).
    fixed_cov: Vec<(usize, usize, f64)>,
}

/// Builder for [`MultivariateModel`].
#[derive(Clone, Debug, Default)]
pub struct MultivariateBuilder {
    inputs: Vec<InputQuantity>,
    intermediates: Vec<(String, String)>,
    outputs: Vec<(String, Option<String>, String, Option<f64>)>,
    correlations: Vec<Correlation>,
    indications: Vec<(Vec<String>, Vec<Vec<f64>>, ObservationCovariance)>,
    implicit: bool,
}

impl MultivariateBuilder {
    pub fn input(mut self, q: InputQuantity) -> Self {
        self.inputs.push(q);
        self
    }

    pub fn quantity(self, name: &str, d: Distribution) -> Self {
        self.input(InputQuantity::new(name, d))
    }

    pub fn intermediate(mut self, name: &str, expression: &str) -> Self {
        self.intermediates.push((name.into(), expression.into()));
        self
    }

    /// Adds an output quantity: its measurement function (explicit model) or
    /// the equation h_j = 0 (implicit model, with a starting value).
    pub fn output(mut self, name: &str, unit: Option<&str>, expression: &str, start: Option<f64>) -> Self {
        self.outputs.push((name.into(), unit.map(str::to_string), expression.into(), start));
        self
    }

    pub fn correlation(mut self, a: &str, b: &str, r: f64) -> Self {
        self.correlations.push(Correlation { a: a.into(), b: b.into(), r });
        self
    }

    /// Input quantities whose estimates are the averages of vector indication
    /// values (one row per set, one column per quantity).
    pub fn indications(mut self, names: &[&str], rows: Vec<Vec<f64>>, gum: ObservationCovariance) -> Self {
        self.indications.push((names.iter().map(|s| s.to_string()).collect(), rows, gum));
        self
    }

    pub fn implicit(mut self, implicit: bool) -> Self {
        self.implicit = implicit;
        self
    }

    pub fn build(self) -> Result<MultivariateModel, UncError> {
        let mut inputs = self.inputs;
        let mut students: Vec<StudentGroup> = Vec::new();
        let mut fixed_cov = Vec::new();
        let mut extra_corr = Vec::new();
        for (names, rows, kind) in &self.indications {
            let ind = Indications::from_rows(rows)?;
            if names.len() != ind.mean.len() {
                return Err(invalid("indication values: one name per column is needed"));
            }
            let cov = ind.covariance(*kind)?;
            let first = inputs.len();
            for (j, name) in names.iter().enumerate() {
                inputs.push(InputQuantity::new(name, Distribution::Normal { mean: ind.mean[j], sd: cov[j][j].sqrt() }));
            }
            for a in 0..names.len() {
                for b in a + 1..names.len() {
                    fixed_cov.push((first + a, first + b, cov[a][b]));
                    let den = (cov[a][a] * cov[b][b]).sqrt();
                    extra_corr.push((first + a, first + b, if den > 0.0 { cov[a][b] / den } else { 0.0 }));
                }
            }
            let l = cholesky(&ind.scale())
                .ok_or_else(|| invalid("indication values: the scale matrix is not positive semi-definite"))?;
            students.push(((first..first + names.len()).collect(), ind.mean.clone(), l, ind.dof));
        }
        let n = inputs.len();
        let mut index: HashMap<String, usize> = HashMap::new();
        for (i, q) in inputs.iter().enumerate() {
            if q.name.is_empty() {
                return Err(invalid("input quantity with empty name"));
            }
            if index.insert(q.name.clone(), i).is_some() {
                return Err(UncError::DuplicateName(q.name.clone()));
            }
            q.distribution.validate()?;
        }
        if self.outputs.is_empty() {
            return Err(invalid("the model has no output quantity"));
        }
        let mut out_index: HashMap<String, usize> = HashMap::new();
        for (j, o) in self.outputs.iter().enumerate() {
            if index.contains_key(&o.0) || out_index.insert(o.0.clone(), n + j).is_some() {
                return Err(UncError::DuplicateName(o.0.clone()));
            }
        }
        let mut defined: HashMap<String, Expr> = HashMap::new();
        for (name, src) in &self.intermediates {
            if index.contains_key(name) || out_index.contains_key(name) || defined.contains_key(name) {
                return Err(UncError::DuplicateName(name.clone()));
            }
            let e = {
                let lookup = |s: &str| -> Option<Expr> {
                    index.get(s).map(|&i| Expr::Var(i)).or_else(|| defined.get(s).cloned())
                };
                expr::parse(src, &lookup)?
            };
            defined.insert(name.clone(), e);
        }
        let implicit = self.implicit;
        let lookup = |s: &str| -> Option<Expr> {
            index.get(s).map(|&i| Expr::Var(i)).or_else(|| defined.get(s).cloned()).or_else(|| {
                if implicit {
                    out_index.get(s).map(|&k| Expr::Var(k))
                } else {
                    None
                }
            })
        };
        let mut outputs = Vec::new();
        for (name, unit, src, start) in self.outputs {
            let e = expr::parse(&src, &lookup)?;
            if implicit && start.is_none() {
                return Err(invalid(format!("{name}: an implicit model needs a starting value")));
            }
            outputs.push(OutputQuantity { name, unit, source: src, expr: e, start });
        }
        let mut correlation = vec![vec![0.0; n]; n];
        for (i, row) in correlation.iter_mut().enumerate() {
            row[i] = 1.0;
        }
        for (a, b, r) in extra_corr {
            correlation[a][b] = r;
            correlation[b][a] = r;
        }
        for c in &self.correlations {
            let i = *index.get(&c.a).ok_or_else(|| UncError::UnknownSymbol(c.a.clone()))?;
            let j = *index.get(&c.b).ok_or_else(|| UncError::UnknownSymbol(c.b.clone()))?;
            if i == j || !(c.r >= -1.0 && c.r <= 1.0) {
                return Err(UncError::InvalidCorrelation(format!("r({}, {}) = {}", c.a, c.b, c.r)));
            }
            if students.iter().any(|g| g.0.contains(&i) || g.0.contains(&j)) {
                return Err(UncError::InvalidCorrelation(format!(
                    "{} or {} is characterized by indication values; its correlations follow from them",
                    c.a, c.b
                )));
            }
            correlation[i][j] = c.r;
            correlation[j][i] = c.r;
        }
        let model = MultivariateModel {
            inputs,
            correlation,
            outputs,
            implicit,
            intermediates: self.intermediates,
            students,
            fixed_cov,
        };
        let y = model.solve(&model.estimates(), None)?;
        if y.iter().any(|v| !v.is_finite()) {
            return Err(UncError::ModelNotDefined("the model is not defined at the input estimates".into()));
        }
        Ok(model)
    }
}

impl MultivariateModel {
    pub fn builder() -> MultivariateBuilder {
        MultivariateBuilder::default()
    }

    pub fn n_inputs(&self) -> usize {
        self.inputs.len()
    }

    pub fn n_outputs(&self) -> usize {
        self.outputs.len()
    }

    pub fn estimates(&self) -> Vec<f64> {
        self.inputs.iter().map(InputQuantity::estimate).collect()
    }

    pub fn output_names(&self) -> Vec<String> {
        self.outputs.iter().map(|o| o.name.clone()).collect()
    }

    /// Covariance matrix U_x of the input estimates.
    pub fn input_covariance(&self) -> Matrix {
        let u: Vec<f64> = self.inputs.iter().map(InputQuantity::gum_standard_uncertainty).collect();
        let n = self.n_inputs();
        let mut c: Matrix = (0..n).map(|i| (0..n).map(|j| u[i] * u[j] * self.correlation[i][j]).collect()).collect();
        for &(a, b, v) in &self.fixed_cov {
            c[a][b] = v;
            c[b][a] = v;
        }
        c
    }

    /// The output values for input values x: f(x), or the solution of
    /// h(y, x) = 0 by Newton's method from `start` (default the starting
    /// values of the outputs).
    pub fn solve(&self, x: &[f64], start: Option<&[f64]>) -> Result<Vec<f64>, UncError> {
        if !self.implicit {
            return Ok(self.outputs.iter().map(|o| o.expr.eval(x)).collect());
        }
        let jac: Vec<Vec<Expr>> = self
            .outputs
            .iter()
            .map(|o| (0..self.n_outputs()).map(|k| o.expr.diff(self.n_inputs() + k)).collect())
            .collect();
        let y0: Vec<f64> = match start {
            Some(s) => s.to_vec(),
            None => self.outputs.iter().map(|o| o.start.unwrap_or(0.0)).collect(),
        };
        let mut z: Vec<f64> = x.iter().copied().chain(y0).collect();
        newton(
            &mut z,
            self.n_inputs(),
            &|z: &[f64]| self.outputs.iter().map(|o| o.expr.eval(z)).collect(),
            &|z: &[f64]| jac.iter().map(|row| row.iter().map(|e| e.eval(z)).collect()).collect(),
        )?;
        Ok(z[self.n_inputs()..].to_vec())
    }
}

/// Newton iteration on the trailing variables of z. It stops when every
/// step is below 10⁻¹² of the size of the variable (or of its starting
/// value), or when the steps stop decreasing at the level of rounding.
fn newton(
    z: &mut [f64],
    n: usize,
    h: &dyn Fn(&[f64]) -> Vec<f64>,
    jac: &dyn Fn(&[f64]) -> Matrix,
) -> Result<(), UncError> {
    let scale: Vec<f64> = z[n..].iter().map(|v| v.abs()).collect();
    let mut previous = f64::INFINITY;
    for _ in 0..60 {
        let r = h(z);
        let j = jac(z);
        let step = solve(&j, &r.iter().map(|v| vec![*v]).collect())
            .ok_or_else(|| UncError::ModelNotDefined("the implicit model has a singular Jacobian ∂h/∂Y".into()))?;
        let mut converged = true;
        let mut stalled = true;
        let mut largest: f64 = 0.0;
        for (k, s) in step.iter().enumerate() {
            let y = &mut z[n + k];
            *y -= s[0];
            let size = y.abs().max(scale[k]);
            let rel = s[0].abs() / size.max(f64::MIN_POSITIVE);
            converged &= s[0].abs() <= 1e-12 * size;
            stalled &= s[0].abs() <= 1e-8 * size;
            largest = largest.max(rel);
        }
        if !largest.is_finite() {
            break;
        }
        if converged || (stalled && largest >= 0.5 * previous) {
            return Ok(());
        }
        previous = largest;
    }
    Err(UncError::ModelNotDefined("the implicit model did not converge (Newton's method)".into()))
}

// ---------------------------------------------------------------------------
// GUM uncertainty framework (clause 6)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct MultivariateGum {
    pub estimate: Vec<f64>,
    pub covariance: Matrix,
    pub standard_uncertainties: Vec<f64>,
    pub correlation: Matrix,
    /// Largest eigenvalue of the correlation matrix (7.8.2.3).
    pub lambda_max: f64,
    /// Sensitivity matrix ∂Y/∂X (m × N); for an implicit model −C_y⁻¹ C_x.
    pub sensitivity: Matrix,
    pub coverage_probability: f64,
    pub k_ellipsoid: f64,
    pub k_rectangle: f64,
}

/// Applies the generalized law of propagation of uncertainty (6.2, 6.3).
pub fn gum(model: &MultivariateModel, p: f64) -> Result<MultivariateGum, UncError> {
    if !(p > 0.0 && p < 1.0) {
        return Err(invalid("coverage probability must lie in (0, 1)"));
    }
    let x = model.estimates();
    let y = model.solve(&x, None)?;
    let (n, m) = (model.n_inputs(), model.n_outputs());
    let z: Vec<f64> = x.iter().copied().chain(y.iter().copied()).collect();
    let cx: Matrix = model.outputs.iter().map(|o| (0..n).map(|i| o.expr.diff(i).eval(&z)).collect()).collect();
    let sensitivity = if model.implicit {
        let cy: Matrix = model.outputs.iter().map(|o| (0..m).map(|k| o.expr.diff(n + k).eval(&z)).collect()).collect();
        // Annex B: solve C_y A = C_x rather than invert C_y.
        let a =
            solve(&cy, &cx).ok_or_else(|| UncError::ModelNotDefined("∂h/∂Y is singular at the estimates".into()))?;
        a.into_iter().map(|r| r.into_iter().map(|v| -v).collect()).collect()
    } else {
        cx
    };
    if sensitivity.iter().flatten().any(|v| !v.is_finite()) {
        return Err(UncError::ModelNotDefined("a sensitivity coefficient is not finite at the estimates".into()));
    }
    let ux = model.input_covariance();
    let covariance = mat_mul(&mat_mul(&sensitivity, &ux), &transpose(&sensitivity));
    let (u, r) = correlations_of(&covariance);
    let lambda_max = symmetric_eigenvalues(&r).last().copied().unwrap_or(1.0);
    Ok(MultivariateGum {
        estimate: y,
        covariance,
        standard_uncertainties: u,
        correlation: r,
        lambda_max,
        sensitivity,
        coverage_probability: p,
        k_ellipsoid: ellipsoid_coverage_factor(m, p),
        k_rectangle: rectangle_coverage_factor(m, p),
    })
}

// ---------------------------------------------------------------------------
// Monte Carlo method (clause 7)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MultiTrials {
    Fixed(usize),
    /// Adaptive procedure of 7.8.3 with ndig significant digits; the
    /// tolerances are divided by `tolerance_divisor` (8.3 note 3).
    Adaptive {
        ndig: u32,
        tolerance_divisor: f64,
        max_trials: usize,
    },
}

#[derive(Clone, Debug)]
pub struct MultiMcmOptions {
    pub trials: MultiTrials,
    pub coverage_probability: f64,
    pub seed: u64,
    /// Mesh of the approximation to the smallest coverage region for two
    /// outputs (7.7.4); 0 to skip it.
    pub mesh: usize,
    pub keep_values: bool,
}

#[derive(Clone, Debug)]
pub struct MultivariateMcm {
    pub estimate: Vec<f64>,
    pub covariance: Matrix,
    pub standard_uncertainties: Vec<f64>,
    pub correlation: Matrix,
    pub lambda_max: f64,
    pub coverage_probability: f64,
    /// k_p of the hyper-ellipsoid (η − y)ᵀ U_y⁻¹ (η − y) = k_p² containing the
    /// fraction p of the draws (7.7.2).
    pub k_ellipsoid: f64,
    /// k_q of the hyper-rectangle y_j ± k_q u(y_j) (7.7.3).
    pub k_rectangle: f64,
    /// Area of the approximation to the smallest coverage region (m = 2,
    /// 7.7.4).
    pub smallest_region_area: Option<f64>,
    /// Fractions of the draws inside the GUM ellipsoid and rectangle.
    pub gum_ellipsoid_coverage: Option<f64>,
    pub gum_rectangle_coverage: Option<f64>,
    pub trials: usize,
    pub runs: usize,
    pub converged: bool,
    pub seed: u64,
    /// Output values, trial by trial (m values each), when kept.
    pub values: Option<Vec<f64>>,
}

struct Summary {
    y: Vec<f64>,
    cov: Matrix,
    u: Vec<f64>,
    r: Matrix,
    lambda: f64,
    kp: f64,
    kq: f64,
}

fn summarize(values: &[f64], m: usize, p: f64) -> Result<Summary, UncError> {
    let n = values.len() / m;
    let mut y = vec![0.0; m];
    for t in values.chunks_exact(m) {
        for j in 0..m {
            y[j] += t[j];
        }
    }
    for v in &mut y {
        *v /= n as f64;
    }
    let mut cov = vec![vec![0.0; m]; m];
    for t in values.chunks_exact(m) {
        for a in 0..m {
            let da = t[a] - y[a];
            for b in a..m {
                cov[a][b] += da * (t[b] - y[b]);
            }
        }
    }
    for a in 0..m {
        for b in a..m {
            cov[a][b] /= (n - 1) as f64;
            cov[b][a] = cov[a][b];
        }
    }
    let (u, r) = correlations_of(&cov);
    let lambda = symmetric_eigenvalues(&r).last().copied().unwrap_or(1.0);
    let l = cholesky(&cov).ok_or_else(|| invalid("the Monte Carlo covariance matrix is not positive semi-definite"))?;
    if (0..m).any(|j| !(l[j][j] > 0.0)) {
        return Err(UncError::Unsupported(
            "the covariance matrix of the outputs is singular (dependent outputs); a hyper-ellipsoidal coverage region is \
             not defined (JCGM 102 9.4.2.3 note 2)"
                .into(),
        ));
    }
    let q = coverage_count(n, p).clamp(1, n);
    let mut d2 = Vec::with_capacity(n);
    let mut dmax = Vec::with_capacity(n);
    let mut w = vec![0.0; m];
    for t in values.chunks_exact(m) {
        // ẙ = L⁻¹ (y_r − y) by forward substitution (formula 21).
        let mut s2 = 0.0;
        let mut mx: f64 = 0.0;
        for a in 0..m {
            let mut v = t[a] - y[a];
            for b in 0..a {
                v -= l[a][b] * w[b];
            }
            w[a] = v / l[a][a];
            s2 += w[a] * w[a];
            mx = mx.max(((t[a] - y[a]) / u[a]).abs());
        }
        d2.push(s2);
        dmax.push(mx);
    }
    d2.sort_unstable_by(f64::total_cmp);
    dmax.sort_unstable_by(f64::total_cmp);
    Ok(Summary { y, cov, u, r, lambda, kp: d2[q - 1].sqrt(), kq: dmax[q - 1] })
}

fn sd_of_average(v: &[f64]) -> f64 {
    let h = v.len() as f64;
    let mean = v.iter().sum::<f64>() / h;
    (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (h * (h - 1.0))).sqrt()
}

/// Approximation to the smallest coverage region of two outputs on a
/// mesh × mesh grid over the range of the draws (7.7.4): its area.
fn smallest_region_area(values: &[f64], p: f64, mesh: usize) -> f64 {
    let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for t in values.chunks_exact(2) {
        for j in 0..2 {
            lo[j] = lo[j].min(t[j]);
            hi[j] = hi[j].max(t[j]);
        }
    }
    let w = [(hi[0] - lo[0]) / mesh as f64, (hi[1] - lo[1]) / mesh as f64];
    let mut counts = vec![0usize; mesh * mesh];
    for t in values.chunks_exact(2) {
        let i = (((t[0] - lo[0]) / w[0]) as usize).min(mesh - 1);
        let j = (((t[1] - lo[1]) / w[1]) as usize).min(mesh - 1);
        counts[i * mesh + j] += 1;
    }
    counts.sort_unstable_by(|a, b| b.cmp(a));
    let need = p * (values.len() / 2) as f64;
    let mut acc = 0usize;
    let mut cells = 0usize;
    for c in counts {
        if (acc as f64) >= need {
            break;
        }
        acc += c;
        cells += 1;
    }
    cells as f64 * w[0] * w[1]
}

/// Applies the Monte Carlo method (7.1–7.8). With a GUM result, the
/// fractions of the draws inside its coverage regions are also given.
pub fn mcm(
    model: &MultivariateModel,
    options: &MultiMcmOptions,
    gum: Option<&MultivariateGum>,
) -> Result<MultivariateMcm, UncError> {
    let p = options.coverage_probability;
    if !(p > 0.0 && p < 1.0) {
        return Err(invalid("coverage probability must lie in (0, 1)"));
    }
    let (n, m) = (model.n_inputs(), model.n_outputs());
    let base = Model {
        output_name: String::new(),
        output_unit: None,
        inputs: model.inputs.clone(),
        expr: Expr::Num(0.0),
        output_source: String::new(),
        intermediates: Vec::new(),
        correlation: model.correlation.clone(),
        ensembles: Vec::new(),
    };
    let mut sampler = Sampler::with_students(&base, model.students.clone())?;
    let mut rng = Rng::new(options.seed);
    let programs: Vec<Program> = model.outputs.iter().map(|o| o.expr.compile()).collect();
    let jac: Vec<Vec<Program>> = if model.implicit {
        model.outputs.iter().map(|o| (0..m).map(|k| o.expr.diff(n + k).compile()).collect()).collect()
    } else {
        Vec::new()
    };
    let start: Vec<f64> = model.solve(&model.estimates(), None)?;
    let mut z = vec![0.0; n + m];
    let mut stack = Vec::new();
    let mut run = |count: usize, out: &mut Vec<f64>| -> Result<(), UncError> {
        let mut bad = 0usize;
        for _ in 0..count {
            sampler.draw(&mut rng, &mut z[..n]);
            if model.implicit {
                z[n..].copy_from_slice(&start);
                let h = |z: &[f64]| -> Vec<f64> { programs.iter().map(|pr| pr.eval(z)).collect() };
                let j =
                    |z: &[f64]| -> Matrix { jac.iter().map(|row| row.iter().map(|pr| pr.eval(z)).collect()).collect() };
                if newton(&mut z, n, &h, &j).is_err() {
                    bad += 1;
                    continue;
                }
                out.extend_from_slice(&z[n..]);
            } else {
                let before = out.len();
                for pr in &programs {
                    out.push(pr.eval_with(&z, &mut stack));
                }
                if out[before..].iter().any(|v| !v.is_finite()) {
                    out.truncate(before);
                    bad += 1;
                }
            }
        }
        if bad > 0 {
            return Err(UncError::NonFiniteModelValues { count: bad, trials: count });
        }
        Ok(())
    };
    let j_min = (100.0 / (1.0 - p)).ceil() as usize;
    let mut values: Vec<f64> = Vec::new();
    let (runs, converged) = match options.trials {
        MultiTrials::Fixed(count) => {
            if count < 2 || coverage_count(count, p) >= count {
                return Err(invalid(format!("M = {count} trials is too small for a {p} coverage region")));
            }
            run(count, &mut values)?;
            (1, true)
        }
        MultiTrials::Adaptive { ndig, tolerance_divisor, max_trials } => {
            if ndig == 0 || !(tolerance_divisor >= 1.0) {
                return Err(invalid("adaptive MCM needs ndig ≥ 1 and divisor ≥ 1"));
            }
            let chunk_size = j_min.max(10_000);
            let mut stats: Vec<Summary> = Vec::new();
            let mut chunk = Vec::with_capacity(chunk_size * m);
            let mut converged = false;
            loop {
                chunk.clear();
                run(chunk_size, &mut chunk)?;
                values.extend_from_slice(&chunk);
                stats.push(summarize(&chunk, m, p)?);
                // Steps f) to m): at least eleven applications.
                if stats.len() > 10 {
                    let all = summarize(&values, m, p)?;
                    let tol = |v: f64| numerical_tolerance(v, ndig) / tolerance_divisor;
                    let sd = |f: &dyn Fn(&Summary) -> f64| sd_of_average(&stats.iter().map(f).collect::<Vec<_>>());
                    let mut ok = (0..m).all(|j| {
                        let d = tol(all.u[j]);
                        2.0 * sd(&|s| s.y[j]) <= d && 2.0 * sd(&|s| s.u[j]) <= d
                    });
                    ok &= 2.0 * sd(&|s| s.lambda) <= tol(all.lambda);
                    ok &= 2.0 * sd(&|s| s.kp) <= tol(all.kp);
                    ok &= 2.0 * sd(&|s| s.kq) <= tol(all.kq);
                    if ok {
                        converged = true;
                        break;
                    }
                }
                if (values.len() / m) + chunk_size > max_trials {
                    break;
                }
            }
            (stats.len(), converged)
        }
    };
    let s = summarize(&values, m, p)?;
    let trials = values.len() / m;
    let smallest_region_area = (m == 2 && options.mesh > 0).then(|| smallest_region_area(&values, p, options.mesh));
    let (mut gum_ellipsoid_coverage, mut gum_rectangle_coverage) = (None, None);
    if let Some(g) = gum {
        if let Some(l) = cholesky(&g.covariance).filter(|l| (0..m).all(|j| l[j][j] > 0.0)) {
            let (mut inside_e, mut inside_r) = (0usize, 0usize);
            let mut w = vec![0.0; m];
            let ke2 = g.k_ellipsoid * g.k_ellipsoid;
            for t in values.chunks_exact(m) {
                let mut s2 = 0.0;
                let mut in_r = true;
                for a in 0..m {
                    let mut v = t[a] - g.estimate[a];
                    for b in 0..a {
                        v -= l[a][b] * w[b];
                    }
                    w[a] = v / l[a][a];
                    s2 += w[a] * w[a];
                    in_r &= (t[a] - g.estimate[a]).abs() <= g.k_rectangle * g.standard_uncertainties[a];
                }
                inside_e += usize::from(s2 <= ke2);
                inside_r += usize::from(in_r);
            }
            gum_ellipsoid_coverage = Some(inside_e as f64 / trials as f64);
            gum_rectangle_coverage = Some(inside_r as f64 / trials as f64);
        }
    }
    Ok(MultivariateMcm {
        estimate: s.y,
        covariance: s.cov,
        standard_uncertainties: s.u,
        correlation: s.r,
        lambda_max: s.lambda,
        coverage_probability: p,
        k_ellipsoid: s.kp,
        k_rectangle: s.kq,
        smallest_region_area,
        gum_ellipsoid_coverage,
        gum_rectangle_coverage,
        trials,
        runs,
        converged,
        seed: options.seed,
        values: options.keep_values.then_some(values),
    })
}

/// Area of the coverage ellipse of two quantities: π k² √det U.
pub fn ellipse_area(cov: &Matrix, k: f64) -> f64 {
    std::f64::consts::PI * k * k * (cov[0][0] * cov[1][1] - cov[0][1] * cov[1][0]).max(0.0).sqrt()
}

/// Area of the coverage rectangle y_j ± k u(y_j) of two quantities.
pub fn rectangle_area(u: &[f64], k: f64) -> f64 {
    4.0 * k * k * u[0] * u[1]
}

// ---------------------------------------------------------------------------
// Validation (clause 8)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct MultiValidation {
    pub ndig: u32,
    /// δ_j and the differences d_yj, d_u(yj).
    pub delta: Vec<f64>,
    pub d_estimate: Vec<f64>,
    pub d_uncertainty: Vec<f64>,
    pub rho: f64,
    pub d_lambda: f64,
    pub kappa_p: f64,
    pub d_kp: f64,
    pub kappa_q: f64,
    pub d_kq: f64,
    /// Estimates, uncertainties and λ_max agree (8.3 e).
    pub moments_validated: bool,
    pub ellipsoid_validated: bool,
    pub rectangle_validated: bool,
}

/// Compares the GUM results with Monte Carlo results (8.3); the tolerances
/// are formed from the Monte Carlo values.
pub fn validate(g: &MultivariateGum, mc: &MultivariateMcm, ndig: u32) -> MultiValidation {
    let m = g.estimate.len();
    let delta: Vec<f64> = (0..m).map(|j| numerical_tolerance(mc.standard_uncertainties[j], ndig)).collect();
    let d_estimate: Vec<f64> = (0..m).map(|j| (g.estimate[j] - mc.estimate[j]).abs()).collect();
    let d_uncertainty: Vec<f64> =
        (0..m).map(|j| (g.standard_uncertainties[j] - mc.standard_uncertainties[j]).abs()).collect();
    let rho = numerical_tolerance(mc.lambda_max, ndig);
    let d_lambda = (g.lambda_max - mc.lambda_max).abs();
    let kappa_p = numerical_tolerance(mc.k_ellipsoid, ndig);
    let d_kp = (g.k_ellipsoid - mc.k_ellipsoid).abs();
    let kappa_q = numerical_tolerance(mc.k_rectangle, ndig);
    let d_kq = (g.k_rectangle - mc.k_rectangle).abs();
    let moments_validated =
        (0..m).all(|j| d_estimate[j] <= delta[j] && d_uncertainty[j] <= delta[j]) && d_lambda <= rho;
    MultiValidation {
        ndig,
        delta,
        d_estimate,
        d_uncertainty,
        rho,
        d_lambda,
        kappa_p,
        d_kp,
        kappa_q,
        d_kq,
        moments_validated,
        ellipsoid_validated: moments_validated && d_kp <= kappa_p,
        rectangle_validated: moments_validated && d_kq <= kappa_q,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, t: f64) -> bool {
        (a - b).abs() <= t
    }

    #[test]
    fn tables_1_and_2_and_6_5_examples() {
        let kp = [(1, 1.96), (2, 2.45), (3, 2.80), (5, 3.33), (10, 4.28), (20, 5.60), (50, 8.22)];
        for (m, k) in kp {
            assert!(close(ellipsoid_coverage_factor(m, 0.95), k, 0.005), "m = {m}");
        }
        let kq = [(1, 1.96), (2, 2.24), (3, 2.39), (5, 2.58), (10, 2.81), (20, 3.02), (50, 3.29)];
        for (m, k) in kq {
            assert!(close(rectangle_coverage_factor(m, 0.95), k, 0.005), "m = {m}");
        }
        // 6.5.2.3 examples 1 and 2: areas of the ellipse and the rectangle.
        let (k, q) = (ellipsoid_coverage_factor(2, 0.95), rectangle_coverage_factor(2, 0.95));
        let u1 = vec![vec![2.0, 0.0], vec![0.0, 1.0]];
        assert!(close(ellipse_area(&u1, k), 26.6, 0.05) && close(rectangle_area(&[2f64.sqrt(), 1.0], q), 28.4, 0.05));
        let u2 = vec![vec![2.0, 1.9], vec![1.9, 2.0]];
        assert!(close(ellipse_area(&u2, k), 11.8, 0.05));
        // 40.1 in the text follows from k_q rounded to 2.24; unrounded 40.19.
        assert!(close(rectangle_area(&[2f64.sqrt(), 2f64.sqrt()], 2.24), 40.1, 0.05));
        // 6.5.4: F(2, 10; 0.95) = 4.10.
        assert!(close(expectation_region_factor(12, 2, 0.95).unwrap() * 10.0 / 2.0, 4.10, 0.005));
        // Eigenvalues of [[2, 1], [1, 2]].
        let ev = symmetric_eigenvalues(&vec![vec![2.0, 1.0], vec![1.0, 2.0]]);
        assert!(close(ev[0], 1.0, 1e-12) && close(ev[1], 3.0, 1e-12));
    }

    fn additive(x3: Distribution) -> MultivariateModel {
        MultivariateModel::builder()
            .quantity("X1", Distribution::Normal { mean: 0.0, sd: 1.0 })
            .quantity("X2", Distribution::Normal { mean: 0.0, sd: 1.0 })
            .quantity("X3", x3)
            .output("Y1", None, "X1 + X3", None)
            .output("Y2", None, "X2 + X3", None)
            .build()
            .unwrap()
    }

    #[test]
    fn additive_model_9_2() {
        let opts = |seed| MultiMcmOptions {
            trials: MultiTrials::Fixed(200_000),
            coverage_probability: 0.95,
            seed,
            mesh: 0,
            keep_values: false,
        };
        // Case 1 (Table 3).
        let m = additive(Distribution::Normal { mean: 0.0, sd: 1.0 });
        let g = gum(&m, 0.95).unwrap();
        assert_eq!(g.covariance, vec![vec![2.0, 1.0], vec![1.0, 2.0]]);
        assert!(
            close(g.correlation[0][1], 0.5, 1e-15)
                && close(g.k_ellipsoid, 2.45, 0.005)
                && close(g.k_rectangle, 2.24, 0.005)
        );
        let mc = mcm(&m, &opts(1), Some(&g)).unwrap();
        assert!(close(mc.k_ellipsoid, 2.45, 0.02) && close(mc.k_rectangle, 2.21, 0.02));
        let v = validate(&g, &mc, 2);
        assert!(v.ellipsoid_validated && v.rectangle_validated, "{v:?}");
        // Case 2: rectangular X3 with sd 1 (Table 4): k_p 2.38, k_q 2.15, not validated.
        let a = 3f64.sqrt();
        let m = additive(Distribution::Rectangular { lower: -a, upper: a });
        let g = gum(&m, 0.95).unwrap();
        let mc = mcm(&m, &opts(2), Some(&g)).unwrap();
        assert!(
            close(mc.k_ellipsoid, 2.38, 0.02) && close(mc.k_rectangle, 2.15, 0.02),
            "{} {}",
            mc.k_ellipsoid,
            mc.k_rectangle
        );
        let v = validate(&g, &mc, 2);
        assert!(v.moments_validated && !v.ellipsoid_validated && !v.rectangle_validated);
        // Case 3: sd 3 (Table 5): r 0.900, k_p 2.28, k_q 1.87.
        let a = 3.0 * 3f64.sqrt();
        let m = additive(Distribution::Rectangular { lower: -a, upper: a });
        let g = gum(&m, 0.95).unwrap();
        assert!(close(g.standard_uncertainties[0], 3.162, 5e-4) && close(g.correlation[0][1], 0.9, 1e-12));
        let mc = mcm(&m, &opts(3), Some(&g)).unwrap();
        assert!(close(mc.k_ellipsoid, 2.28, 0.02) && close(mc.k_rectangle, 1.87, 0.02));
        assert!(close(mc.correlation[0][1], 0.900, 0.003));
    }

    #[test]
    fn adaptive_procedure_stabilizes() {
        let m = additive(Distribution::Normal { mean: 0.0, sd: 1.0 });
        let o = MultiMcmOptions {
            trials: MultiTrials::Adaptive { ndig: 2, tolerance_divisor: 1.0, max_trials: 2_000_000 },
            coverage_probability: 0.95,
            seed: 7,
            mesh: 0,
            keep_values: false,
        };
        let mc = mcm(&m, &o, None).unwrap();
        assert!(mc.converged && mc.runs >= 11 && mc.trials == mc.runs * 10_000);
    }

    #[test]
    fn coordinate_transformation_9_3() {
        let model = |x1: f64, r: f64| {
            let mut b = MultivariateModel::builder()
                .quantity("X1", Distribution::Normal { mean: x1, sd: 0.01 })
                .quantity("X2", Distribution::Normal { mean: 0.0, sd: 0.01 })
                .output("R", None, "sqrt(X1^2 + X2^2)", None)
                .output("Theta", None, "atan2(X2, X1)", None);
            if r != 0.0 {
                b = b.correlation("X1", "X2", r);
            }
            b.build().unwrap()
        };
        let o = MultiMcmOptions {
            trials: MultiTrials::Fixed(400_000),
            coverage_probability: 0.95,
            seed: 11,
            mesh: 0,
            keep_values: false,
        };
        // Table 6, x1 = 0.001, r = 0: GUF u(y2) = 10, MCM 0.013, 0.007, 1.744.
        let m = model(0.001, 0.0);
        let g = gum(&m, 0.95).unwrap();
        assert!(close(g.standard_uncertainties[0], 0.010, 1e-12) && close(g.standard_uncertainties[1], 10.0, 1e-9));
        let mc = mcm(&m, &o, None).unwrap();
        assert!(close(mc.estimate[0], 0.013, 0.0005) && close(mc.standard_uncertainties[0], 0.007, 0.0005));
        assert!(close(mc.standard_uncertainties[1], 1.744, 0.004));
        // Table 7, x1 = 0.010, r = 0.9: GUF r = 0.900; MCM y2 −0.343, u(y2) 0.903, r 0.352.
        let m = model(0.010, 0.9);
        let g = gum(&m, 0.95).unwrap();
        assert!(close(g.correlation[0][1], 0.9, 1e-12));
        let mc = mcm(&m, &o, None).unwrap();
        assert!(close(mc.estimate[1], -0.343, 0.005) && close(mc.standard_uncertainties[1], 0.903, 0.004));
        assert!(close(mc.correlation[0][1], 0.352, 0.005));
    }

    fn impedance(kind: ObservationCovariance) -> MultivariateModel {
        let rows = vec![
            vec![5.007, 19.663, 1.0456],
            vec![4.994, 19.639, 1.0438],
            vec![5.005, 19.640, 1.0468],
            vec![4.990, 19.685, 1.0428],
            vec![4.999, 19.678, 1.0433],
            vec![4.999, 19.661, 1.04446],
        ];
        MultivariateModel::builder()
            .indications(&["V", "I", "phi"], rows, kind)
            .output("R", Some("Ω"), "V/(I*1e-3)*cos(phi)", None)
            .output("X", Some("Ω"), "V/(I*1e-3)*sin(phi)", None)
            .output("Z", Some("Ω"), "V/(I*1e-3)", None)
            .build()
            .unwrap()
    }

    #[test]
    fn resistance_and_reactance_9_4() {
        // Tables 9 and 10.
        let m = impedance(ObservationCovariance::Mean);
        let ux = m.input_covariance();
        assert!(close(ux[0][0].sqrt(), 0.0026, 0.00005) && close(ux[1][1].sqrt(), 0.0077, 0.00005));
        assert!(close(ux[2][2].sqrt(), 0.00061, 0.000005));
        assert!(close(m.correlation[0][1], -0.355, 0.0005) && close(m.correlation[0][2], 0.858, 0.0005));
        assert!(close(m.correlation[1][2], -0.645, 0.0005));
        // Table 11, GUF.
        let g = gum(&m, 0.95).unwrap();
        for (j, (y, u)) in [(127.732, 0.058), (219.847, 0.241), (254.260, 0.193)].iter().enumerate() {
            assert!(
                close(g.estimate[j], *y, 0.0005) && close(g.standard_uncertainties[j], *u, 0.0005),
                "{j}: {} {}",
                g.estimate[j],
                g.standard_uncertainties[j]
            );
        }
        assert!(close(g.correlation[0][1], -0.588, 0.0005) && close(g.correlation[0][2], -0.485, 0.0005));
        assert!(close(1.0 - g.correlation[1][2], 0.749e-2, 0.0005e-2));
        // Alt. GUF with V(X) = M/((ν − 2)n).
        let alt = gum(&impedance(ObservationCovariance::StudentT), 0.95).unwrap();
        for (j, u) in [0.130, 0.540, 0.431].iter().enumerate() {
            assert!(close(alt.standard_uncertainties[j], *u, 0.0005), "{j}: {}", alt.standard_uncertainties[j]);
        }
        // MCM with the multivariate t-distribution (ν = 3): u 0.130, 0.536, 0.429.
        // The outputs are dependent (Z² = R² + X²): no ellipsoid, but the moments.
        let o = MultiMcmOptions {
            trials: MultiTrials::Fixed(200_000),
            coverage_probability: 0.95,
            seed: 5,
            mesh: 0,
            keep_values: false,
        };
        match mcm(&m, &o, None) {
            Ok(mc) => {
                for (j, u) in [0.130, 0.536, 0.429].iter().enumerate() {
                    assert!(close(mc.standard_uncertainties[j], *u, 0.01), "{j}: {}", mc.standard_uncertainties[j]);
                }
                assert!(close(mc.correlation[0][1], -0.587, 0.01));
            }
            Err(e) => panic!("{e}"),
        }
    }

    fn thermometer(ratios: &[f64]) -> MultivariateModel {
        let mut b = MultivariateModel::builder()
            .quantity("R0", Distribution::Normal { mean: 99.99610, sd: 0.00050 })
            .quantity("A", Distribution::Normal { mean: 0.0039096, sd: 0.0000027 })
            .quantity("B", Distribution::Normal { mean: -6.0e-7, sd: 1.1e-7 })
            .quantity("RS", Distribution::Normal { mean: 99.99947, sd: 0.00010 })
            .correlation("R0", "A", -0.155)
            .correlation("R0", "B", 0.092)
            .correlation("A", "B", -0.959)
            .implicit(true);
        for (j, r) in ratios.iter().enumerate() {
            let name = format!("r{}", j + 1);
            b = b.quantity(&name, Distribution::Normal { mean: *r, sd: 50e-7 });
            b = b.output(
                &format!("t{}", j + 1),
                Some("°C"),
                &format!("R0*(1 + A*t{0} + B*t{0}^2) - {name}*RS", j + 1),
                Some(20.0),
            );
        }
        b.build().unwrap()
    }

    #[test]
    fn resistance_thermometer_9_5() {
        // 9.5.2: θ = 20.023 2 °C, u = 0.004 5 °C.
        let g = gum(&thermometer(&[1.0780057]), 0.95).unwrap();
        assert!(close(g.estimate[0], 20.0232, 0.00005) && close(g.standard_uncertainties[0], 0.0045, 0.00005));
        // 9.5.3, Tables 15 and 16.
        let r: Vec<f64> =
            [53.0, 150054.0, 300055.0, 450056.0, 600056.0, 780057.0, 900058.0, 1050059.0, 1200060.0, 780057.0]
                .iter()
                .map(|v| 1.0 + v * 1e-7)
                .collect();
        let g = gum(&thermometer(&r), 0.95).unwrap();
        let t = [0.0100, 3.8491, 7.6928, 11.5410, 15.3938, 20.0232, 23.1131, 26.9797, 30.8509, 20.0232];
        let u = [0.0018, 0.0027, 0.0040, 0.0046, 0.0047, 0.0045, 0.0046, 0.0060, 0.0089, 0.0045];
        for j in 0..10 {
            assert!(close(g.estimate[j], t[j], 0.00005), "θ{}: {}", j + 1, g.estimate[j]);
            assert!(close(g.standard_uncertainties[j], u[j], 0.00005), "u{}: {}", j + 1, g.standard_uncertainties[j]);
        }
        let r_last = [0.054, 0.580, 0.691, 0.766, 0.847, 0.918, 0.841, 0.549, 0.264];
        for j in 0..9 {
            assert!(close(g.correlation[j][9], r_last[j], 0.0005), "r(θ{}, θ10) = {}", j + 1, g.correlation[j][9]);
        }
        assert!(close(g.correlation[1][2], 0.815, 0.0005) && close(g.correlation[1][8], -0.358, 0.0005));
    }

    #[test]
    fn implicit_monte_carlo_matches_explicit() {
        // Y = X² as h = Y − X² = 0 and explicitly.
        let imp = MultivariateModel::builder()
            .quantity("X", Distribution::Normal { mean: 2.0, sd: 0.1 })
            .implicit(true)
            .output("Y", None, "Y - X^2", Some(1.0))
            .build()
            .unwrap();
        let exp = MultivariateModel::builder()
            .quantity("X", Distribution::Normal { mean: 2.0, sd: 0.1 })
            .output("Y", None, "X^2", None)
            .build()
            .unwrap();
        let o = MultiMcmOptions {
            trials: MultiTrials::Fixed(20_000),
            coverage_probability: 0.95,
            seed: 3,
            mesh: 0,
            keep_values: false,
        };
        let a = mcm(&imp, &o, None).unwrap();
        let b = mcm(&exp, &o, None).unwrap();
        assert!(
            close(a.estimate[0], b.estimate[0], 1e-10)
                && close(a.standard_uncertainties[0], b.standard_uncertainties[0], 1e-10)
        );
        assert!(close(gum(&imp, 0.95).unwrap().standard_uncertainties[0], 0.4, 1e-12));
    }

    #[test]
    fn smallest_region_7_7_4() {
        // 7.7.2 example: X1, X2 ~ N(0, 0.1), X3 rectangular with variance 1.9.
        let a = (3.0f64 * 1.9).sqrt();
        let m = MultivariateModel::builder()
            .quantity("X1", Distribution::Normal { mean: 0.0, sd: 0.1f64.sqrt() })
            .quantity("X2", Distribution::Normal { mean: 0.0, sd: 0.1f64.sqrt() })
            .quantity("X3", Distribution::Rectangular { lower: -a, upper: a })
            .output("Y1", None, "X1 + X3", None)
            .output("Y2", None, "X2 + X3", None)
            .build()
            .unwrap();
        let g = gum(&m, 0.95).unwrap();
        assert!(close(g.covariance[0][1], 1.9, 1e-12));
        let o = MultiMcmOptions {
            trials: MultiTrials::Fixed(1_000_000),
            coverage_probability: 0.95,
            seed: 2,
            mesh: 100,
            keep_values: false,
        };
        let mc = mcm(&m, &o, Some(&g)).unwrap();
        // Smallest region ≈ 9.4 unit² for 10⁶ draws on a 100 × 100 mesh (7.7.4 example).
        // With 1 000 draws the text finds k_p = 2.33 and an area of 10.6; with 10⁶ draws k_p = 2.27 and 10.1.
        assert!(close(mc.k_ellipsoid, 2.27, 0.01), "{}", mc.k_ellipsoid);
        assert!(close(ellipse_area(&mc.covariance, mc.k_ellipsoid), 10.1, 0.1));
        assert!(close(mc.smallest_region_area.unwrap(), 9.4, 0.3), "{:?}", mc.smallest_region_area);
        // The GUM ellipse (k_p = 2.45) contains more than 95 % of the draws (968 of 1 000 in the text).
        assert!(mc.gum_ellipsoid_coverage.unwrap() > 0.96);
    }
}
