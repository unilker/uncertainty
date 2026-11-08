//! GUM uncertainty framework (JCGM 100:2008, clauses 5 and 6, Annex G).
//!
//! * Law of propagation of uncertainty, first order, with correlations
//!   [GUM 5.1.2, 5.2.2, equations (10) and (13)].
//! * Optional higher-order terms of the GUM 5.1.2 note for normally
//!   distributed inputs, extended to correlated inputs [Mana & Pennecchi,
//!   Metrologia 44 (2007) 246], and the second-order term of the estimate,
//!   ½ΣΣ ∂²f/∂xᵢ∂xⱼ u(xᵢ, xⱼ) [GUM H.10; JCGM 100:2008/Amd.1:2026, note 1 to
//!   4.1.4].
//! * Effective degrees of freedom by the Welch–Satterthwaite formula
//!   [GUM G.4.1, equation (G.2b)] and coverage factor from the
//!   t-distribution [GUM G.3, G.6.4].

use crate::error::UncError;
use crate::expr::{Expr, Program};
use crate::model::Model;
use crate::special::coverage_factor;

/// How a non-integer effective degrees of freedom is turned into a
/// coverage factor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DofRounding {
    /// Truncate ν_eff to the next lower integer [GUM G.4.1 note 1, G.6.4].
    Truncate,
    /// Use the t-distribution with non-integer ν_eff (equivalent to exact
    /// interpolation, also allowed by GUM G.4.1 note 1).
    Exact,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoverageSpec {
    /// Coverage factor from the t-distribution with ν_eff for the stated
    /// coverage probability [GUM G.6.4].
    Probability(f64),
    /// Fixed coverage factor (e.g. k = 2) [GUM 6.3.3]; the reported coverage
    /// probability is then the one implied for a normal distribution.
    Factor(f64),
    /// Coverage factor for the stated coverage probability from the
    /// convolution of the largest rectangular contribution with a normal
    /// distribution for the others [UKAS M3003 ed. 6, Appendix C]; without a
    /// rectangular input, as `Probability`.
    DominantRectangular(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GumOptions {
    pub coverage: CoverageSpec,
    /// Include the second-order terms of the GUM 5.1.2 note in u_c(y) and the
    /// second-order term of note 1 to 4.1.4 (Amd.1:2026) in y.
    pub higher_order: bool,
    pub dof_rounding: DofRounding,
}

impl Default for GumOptions {
    fn default() -> Self {
        Self { coverage: CoverageSpec::Probability(0.95), higher_order: false, dof_rounding: DofRounding::Truncate }
    }
}

/// One row of the uncertainty budget.
#[derive(Clone, Debug, PartialEq)]
pub struct BudgetRow {
    pub name: String,
    pub estimate: f64,
    pub standard_uncertainty: f64,
    pub distribution: &'static str,
    pub dof: f64,
    /// Sensitivity coefficient c_i = ∂f/∂x_i at the estimates [GUM 5.1.3].
    pub sensitivity: f64,
    /// Uncertainty contribution u_i(y) = c_i u(x_i) (signed).
    pub contribution: f64,
    /// Share of the combined variance, (c_i u(x_i))² / u_c²(y). Shares do not
    /// sum to one when correlations or higher-order terms are present.
    pub variance_share: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GumResult {
    /// Output estimate y = f(x₁, …, x_N) [GUM 4.1.4], plus the second-order
    /// term when higher-order terms are requested.
    pub estimate: f64,
    /// Second-order term of the estimate, δ = ½ΣΣ ∂²f/∂xᵢ∂xⱼ u(xᵢ, xⱼ)
    /// [GUM H.10; Amd.1:2026, note 1 to 4.1.4]; always evaluated, added to
    /// `estimate` only with higher-order terms.
    pub second_order_estimate_term: f64,
    pub combined_standard_uncertainty: f64,
    /// u_c(y) from first-order terms only.
    pub first_order_uncertainty: f64,
    /// Variance from correlation terms 2 Σ_{i<j} c_i c_j u(x_i, x_j).
    pub correlation_variance: f64,
    /// Variance from the higher-order terms (0 if not requested).
    pub higher_order_variance: f64,
    pub effective_dof: f64,
    pub coverage_factor: f64,
    pub coverage_probability: f64,
    pub expanded_uncertainty: f64,
    pub interval: (f64, f64),
    pub budget: Vec<BudgetRow>,
    pub warnings: Vec<GumWarning>,
    /// With [`CoverageSpec::DominantRectangular`]: the rectangular input whose
    /// contribution was convolved, and u_N/u_R.
    pub dominant_rectangular: Option<(String, f64)>,
}

/// A condition the user should know about when reading a GUM result.
#[derive(Clone, Debug, PartialEq)]
pub enum GumWarning {
    /// Higher-order terms were requested but some inputs are not Gaussian.
    HigherOrderNonGaussian,
    /// The Welch–Satterthwaite formula was applied with correlated inputs.
    WelchSatterthwaiteCorrelated,
    /// Inputs with non-zero uncertainty have a zero sensitivity coefficient
    /// at the estimates but contribute through second-order terms, which a
    /// first-order evaluation misses; `missing_share` is the estimated share
    /// of the variance missed (GUM 5.1.2 note).
    ZeroSensitivity { inputs: Vec<String>, missing_share: f64 },
    /// The second-order term of the estimate (Amd.1:2026, note 1 to 4.1.4)
    /// exceeds a tenth of u_c(y) and higher-order terms were not requested.
    NonlinearEstimate { term: f64, ratio: f64 },
}

impl std::fmt::Display for GumWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GumWarning::HigherOrderNonGaussian => write!(
                f,
                "higher-order terms (GUM 5.1.2 note) are derived for normally distributed inputs; \
                 non-Gaussian inputs are present — validate with the Monte Carlo method (JCGM 101 8)"
            ),
            GumWarning::WelchSatterthwaiteCorrelated => write!(
                f,
                "the Welch–Satterthwaite formula is not valid for correlated inputs (GUM G.4.1); \
                 the effective degrees of freedom is approximate"
            ),
            GumWarning::ZeroSensitivity { inputs, missing_share } => write!(
                f,
                "the sensitivity coefficient of {} is zero at the estimates although the uncertainty is not: \
                 the first-order evaluation misses about {:.0} % of the variance (second-order terms, GUM 5.1.2 \
                 note) — enable the higher-order terms or use the Monte Carlo method",
                inputs.join(", "),
                100.0 * missing_share
            ),
            GumWarning::NonlinearEstimate { term, ratio } => write!(
                f,
                "the model is nonlinear: the second-order term of the estimate, ½ΣΣ ∂²f/∂xᵢ∂xⱼ u(xᵢ, xⱼ) = {term:.3e}, \
                 is {:.2} u_c(y) (JCGM 100:2008/Amd.1:2026, note 1 to 4.1.4) — enable the higher-order terms or use \
                 the Monte Carlo method",
                ratio
            ),
        }
    }
}

/// Kragten's spreadsheet approximation of the law of propagation of
/// uncertainty: the contribution of each input is the change of the output
/// when the input is moved by its standard uncertainty,
/// Δᵢ = f(…, xᵢ + u(xᵢ), …) − f(x), and u_c² = ΣΔᵢ² + 2ΣΣ ΔᵢΔⱼ r(xᵢ, xⱼ)
/// [Kragten, Analyst 119 (1994) 2161; EURACHEM/CITAC QUAM:2012 Appendix E.2].
/// It differs from the exact first-order result when the model is not linear
/// over ±u(xᵢ); it is provided for comparison with spreadsheet evaluations.
#[derive(Clone, Debug, PartialEq)]
pub struct Kragten {
    pub contributions: Vec<f64>,
    pub combined_standard_uncertainty: f64,
}

/// Evaluates the Kragten approximation for a model.
pub fn kragten(model: &Model) -> Result<Kragten, UncError> {
    let x = model.estimates();
    let y = model.evaluate(&x);
    let n = model.n_inputs();
    let mut contributions = Vec::with_capacity(n);
    for i in 0..n {
        let mut xi = x.clone();
        xi[i] += model.inputs[i].gum_standard_uncertainty();
        let d = model.evaluate(&xi) - y;
        if !d.is_finite() {
            return Err(UncError::ModelNotDefined(format!(
                "the model is not defined at {} + u({})",
                model.inputs[i].name, model.inputs[i].name
            )));
        }
        contributions.push(d);
    }
    let mut var: f64 = contributions.iter().map(|d| d * d).sum();
    for i in 0..n {
        for j in (i + 1)..n {
            var += 2.0 * contributions[i] * contributions[j] * model.correlation[i][j];
        }
    }
    Ok(Kragten { contributions, combined_standard_uncertainty: var.max(0.0).sqrt() })
}

/// Applies the GUM uncertainty framework to a model.
pub fn evaluate(model: &Model, options: &GumOptions) -> Result<GumResult, UncError> {
    let n = model.n_inputs();
    let x = model.estimates();
    let u: Vec<f64> = model.inputs.iter().map(|q| q.gum_standard_uncertainty()).collect();
    let nu: Vec<f64> = model.inputs.iter().map(|q| q.gum_dof()).collect();
    let mut warnings = Vec::new();

    let y = model.evaluate(&x);
    let first: Vec<Expr> = (0..n).map(|i| model.expr.diff(i)).collect();
    let c: Vec<f64> = first.iter().map(|d| d.eval(&x)).collect();
    if let Some(i) = c.iter().position(|v| !v.is_finite()) {
        return Err(UncError::ModelNotDefined(format!(
            "sensitivity coefficient for {} is not finite at the estimates",
            model.inputs[i].name
        )));
    }

    let ui_y: Vec<f64> = (0..n).map(|i| c[i] * u[i]).collect();
    let first_order_var: f64 = ui_y.iter().map(|v| v * v).sum();

    let mut correlation_var = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            let r = model.correlation[i][j];
            if r != 0.0 {
                correlation_var += 2.0 * ui_y[i] * ui_y[j] * r;
            }
        }
    }

    // Second-order term of the estimate [GUM H.10; Amd.1:2026, note 1 to
    // 4.1.4]: ½ Σ_i Σ_j ∂²f/∂x_i∂x_j u(x_i, x_j).
    let mut second_order_term = 0.0;
    for i in 0..n {
        if u[i] == 0.0 {
            continue;
        }
        for j in 0..n {
            let r = if i == j { 1.0 } else { model.correlation[i][j] };
            if u[j] == 0.0 || r == 0.0 {
                continue;
            }
            let fij = first[i].diff(j).eval(&x);
            if fij.is_finite() {
                second_order_term += 0.5 * fij * r * u[i] * u[j];
            }
        }
    }

    let mut higher_order_var = 0.0;
    if options.higher_order {
        if model.inputs.iter().any(|q| !q.distribution.is_gaussian() && u_nonzero(q)) {
            warnings.push(GumWarning::HigherOrderNonGaussian);
        }
        // Second- and third-order Taylor terms for (multivariate) normal inputs
        // with covariance matrix Σ [Mana & Pennecchi, Metrologia 44 (2007) 246]:
        //   ½ Σ_ijkl f_ij f_kl Σ_jk Σ_li  +  Σ_ijkl c_l f_ijk Σ_li Σ_jk,
        // which for independent inputs is the expression of the GUM 5.1.2 note,
        // Σ_i Σ_j [½ f_ij² + c_i f_ijj] u²(x_i) u²(x_j).
        let active: Vec<usize> = (0..n).filter(|&i| u[i] > 0.0).collect();
        let cov = |i: usize, j: usize| -> f64 {
            if i == j {
                u[i] * u[i]
            } else {
                model.correlation[i][j] * u[i] * u[j]
            }
        };
        let m = active.len();
        let sigma: Vec<Vec<f64>> = active.iter().map(|&i| active.iter().map(|&j| cov(i, j)).collect()).collect();
        let second_exprs: Vec<Vec<Expr>> =
            active.iter().map(|&i| active.iter().map(|&j| first[i].diff(j)).collect()).collect();
        let h: Vec<Vec<f64>> = second_exprs.iter().map(|row| row.iter().map(|e| e.eval(&x)).collect()).collect();
        // ½ tr((HΣ)²)
        let hs: Vec<Vec<f64>> =
            (0..m).map(|a| (0..m).map(|b| (0..m).map(|k| h[a][k] * sigma[k][b]).sum()).collect()).collect();
        let mut v2 = 0.0;
        for a in 0..m {
            for b in 0..m {
                v2 += 0.5 * hs[a][b] * hs[b][a];
            }
        }
        // Σ_l Σ_ijk c_l f_ijk Σ_li Σ_jk
        let mut v3 = 0.0;
        for i in 0..m {
            let csig: f64 = (0..m).map(|l| c[active[l]] * sigma[l][i]).sum();
            if csig == 0.0 {
                continue;
            }
            for j in 0..m {
                for k in 0..m {
                    if sigma[j][k] == 0.0 {
                        continue;
                    }
                    let fijk = second_exprs[i][j].diff(active[k]).eval(&x);
                    v3 += csig * fijk * sigma[j][k];
                }
            }
        }
        higher_order_var = v2 + v3;
        if !higher_order_var.is_finite() {
            return Err(UncError::ModelNotDefined("higher-order terms are not finite at the estimates".into()));
        }
    }

    // Inputs whose first-order contribution vanishes although they are
    // uncertain: estimate the second-order variance a first-order evaluation
    // misses (terms of the GUM 5.1.2 note involving these inputs).
    if !options.higher_order {
        let scale = first_order_var.sqrt();
        let zero: Vec<usize> =
            (0..n).filter(|&i| u[i] > 0.0 && (ui_y[i].abs() <= 1e-12 * scale || c[i] == 0.0)).collect();
        if !zero.is_empty() {
            let mut missed = 0.0;
            for i in 0..n {
                for j in 0..n {
                    if u[i] == 0.0 || u[j] == 0.0 || !(zero.contains(&i) || zero.contains(&j)) {
                        continue;
                    }
                    let dij = first[i].diff(j);
                    let second = dij.eval(&x);
                    let third = dij.diff(j).eval(&x);
                    if second.is_finite() && third.is_finite() {
                        missed += (0.5 * second * second + c[i] * third) * u[i].powi(2) * u[j].powi(2);
                    }
                }
            }
            let total = first_order_var + missed;
            if missed > 0.0 && total > 0.0 && missed / total > 0.01 {
                let mut inputs: Vec<String> = zero
                    .iter()
                    .filter(|&&i| (0..n).any(|j| u[j] > 0.0 && first[i].diff(j).eval(&x) != 0.0))
                    .map(|&i| model.inputs[i].name.clone())
                    .collect();
                if inputs.is_empty() {
                    inputs = zero.iter().map(|&i| model.inputs[i].name.clone()).collect();
                }
                warnings.push(GumWarning::ZeroSensitivity { inputs, missing_share: missed / total });
            }
        }
    }

    let combined_var = first_order_var + correlation_var + higher_order_var;
    if !(combined_var >= 0.0) {
        return Err(UncError::ModelNotDefined(
            "the combined variance is negative; check the correlation coefficients".into(),
        ));
    }
    let uc = combined_var.sqrt();
    let estimate = if options.higher_order { y + second_order_term } else { y };
    if !options.higher_order && uc > 0.0 && second_order_term.abs() > 0.1 * uc {
        warnings.push(GumWarning::NonlinearEstimate { term: second_order_term, ratio: second_order_term.abs() / uc });
    }

    // Welch–Satterthwaite [GUM G.2b]. Higher-order variance is treated as
    // having infinite degrees of freedom (documented assumption). Inputs of
    // an ensemble (same data, same ν) form one component whose variance
    // includes their covariances [Willink 2007, §4.1.1; GTC]. A correlation
    // between inputs with finite ν outside a common ensemble invalidates the
    // formula [GUM G.4.1]; it is applied anyway, with a warning.
    let ensemble_of = |i: usize| model.ensembles.iter().position(|e| e.contains(&i));
    let unsupported_correlation = (0..n).any(|i| {
        (i + 1..n).any(|j| {
            model.correlation[i][j] != 0.0
                && (nu[i].is_finite() || nu[j].is_finite())
                && !(ensemble_of(i).is_some() && ensemble_of(i) == ensemble_of(j))
        })
    });
    if unsupported_correlation {
        warnings.push(GumWarning::WelchSatterthwaiteCorrelated);
    }
    let mut components: Vec<(f64, f64)> =
        (0..n).filter(|&i| ensemble_of(i).is_none() && ui_y[i] != 0.0).map(|i| (ui_y[i] * ui_y[i], nu[i])).collect();
    for e in &model.ensembles {
        let v: f64 =
            e.iter().map(|&i| e.iter().map(|&j| ui_y[i] * ui_y[j] * model.correlation[i][j]).sum::<f64>()).sum();
        if v != 0.0 {
            components.push((v, nu[e[0]]));
        }
    }
    let denom: f64 = components.iter().filter(|c| c.1.is_finite()).map(|&(v, nu_i)| v * v / nu_i).sum();
    let nu_eff = if denom > 0.0 { (uc.powi(4) / denom).min(f64::MAX) } else { f64::INFINITY };
    // G.2c: ν_eff ≤ Σ ν_i (an ensemble counts once).
    let nu_sum: f64 = components.iter().map(|c| c.1).sum();
    let nu_eff = nu_eff.min(nu_sum);

    let (k, p) = match options.coverage {
        CoverageSpec::Factor(k) => {
            if !(k > 0.0) {
                return Err(UncError::InvalidOption("coverage factor must be positive".into()));
            }
            (k, 2.0 * crate::special::normal_cdf(k) - 1.0)
        }
        CoverageSpec::Probability(p) | CoverageSpec::DominantRectangular(p) => {
            if !(p > 0.0 && p < 1.0) {
                return Err(UncError::InvalidOption("coverage probability must lie in (0, 1)".into()));
            }
            let nu_k = match options.dof_rounding {
                _ if nu_eff.is_infinite() => f64::INFINITY,
                DofRounding::Truncate => nu_eff.floor().max(1.0),
                DofRounding::Exact => nu_eff,
            };
            (coverage_factor(p, nu_k), p)
        }
    };
    let mut dominant_rectangular = None;
    let mut k = k;
    if let CoverageSpec::DominantRectangular(p) = options.coverage {
        let largest = (0..n)
            .filter(|&i| model.inputs[i].distribution.kind() == "rectangular" && ui_y[i] != 0.0)
            .max_by(|&a, &b| ui_y[a].abs().total_cmp(&ui_y[b].abs()));
        if let Some(i) = largest {
            let ur = ui_y[i].abs();
            let ratio = (combined_var - ur * ur).max(0.0).sqrt() / ur;
            k = crate::special::rectangular_normal_coverage_factor(p, ratio);
            dominant_rectangular = Some((model.inputs[i].name.clone(), ratio));
        }
    }
    let expanded = k * uc;

    let budget = (0..n)
        .map(|i| BudgetRow {
            name: model.inputs[i].name.clone(),
            estimate: x[i],
            standard_uncertainty: u[i],
            distribution: model.inputs[i].distribution.kind(),
            dof: nu[i],
            sensitivity: c[i],
            contribution: ui_y[i],
            variance_share: if combined_var > 0.0 { ui_y[i].powi(2) / combined_var } else { 0.0 },
        })
        .collect();

    Ok(GumResult {
        estimate,
        second_order_estimate_term: second_order_term,
        combined_standard_uncertainty: uc,
        first_order_uncertainty: (first_order_var + correlation_var).max(0.0).sqrt(),
        correlation_variance: correlation_var,
        higher_order_variance: higher_order_var,
        effective_dof: nu_eff,
        coverage_factor: k,
        coverage_probability: p,
        expanded_uncertainty: expanded,
        interval: (estimate - expanded, estimate + expanded),
        budget,
        warnings,
        dominant_rectangular,
    })
}

fn u_nonzero(q: &crate::model::InputQuantity) -> bool {
    q.gum_standard_uncertainty() > 0.0
}

/// Sensitivity-coefficient programs, exposed for reports that show the
/// partial derivatives symbolically.
pub fn sensitivity_expressions(model: &Model) -> Vec<String> {
    let names = model.input_names();
    (0..model.n_inputs()).map(|i| model.expr.diff(i).to_string_with(&names)).collect()
}

/// Compiled sensitivity coefficients (useful for repeated evaluation).
pub fn sensitivity_programs(model: &Model) -> Vec<Program> {
    (0..model.n_inputs()).map(|i| model.expr.diff(i).compile()).collect()
}
