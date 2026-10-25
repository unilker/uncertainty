//! GUM uncertainty framework (JCGM 100:2008, clauses 5 and 6, Annex G).
//!
//! * Law of propagation of uncertainty, first order, with correlations
//!   [GUM 5.1.2, 5.2.2, equations (10) and (13)].
//! * Optional higher-order terms of the GUM 5.1.2 note (independent,
//!   normally distributed inputs).
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GumOptions {
    pub coverage: CoverageSpec,
    /// Include the second-order terms of the GUM 5.1.2 note.
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
    /// Output estimate y = f(x₁, …, x_N) [GUM 4.1.4].
    pub estimate: f64,
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
    pub warnings: Vec<String>,
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

    let mut higher_order_var = 0.0;
    if options.higher_order {
        if model.has_correlations() {
            return Err(UncError::Unsupported(
                "the higher-order terms of GUM 5.1.2 note apply to independent inputs only; \
                 use the Monte Carlo method for correlated inputs"
                    .into(),
            ));
        }
        if model.inputs.iter().any(|q| !q.distribution.is_gaussian() && u_nonzero(q)) {
            warnings.push(
                "higher-order terms (GUM 5.1.2 note) are derived for normally distributed inputs; \
                 non-Gaussian inputs are present — validate with the Monte Carlo method (JCGM 101 8)"
                    .into(),
            );
        }
        // Σ_i Σ_j [ ½ (∂²f/∂x_i∂x_j)² + ∂f/∂x_i ∂³f/∂x_i∂x_j² ] u²(x_i) u²(x_j)
        for i in 0..n {
            if u[i] == 0.0 {
                continue;
            }
            for j in 0..n {
                if u[j] == 0.0 {
                    continue;
                }
                let dij = first[i].diff(j);
                let second = dij.eval(&x);
                let third = dij.diff(j).eval(&x);
                higher_order_var += (0.5 * second * second + c[i] * third) * u[i].powi(2) * u[j].powi(2);
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

    // Welch–Satterthwaite [GUM G.2b]. Higher-order variance is treated as
    // having infinite degrees of freedom (documented assumption).
    if model.has_correlations() && nu.iter().any(|v| v.is_finite()) {
        warnings.push(
            "the Welch–Satterthwaite formula is not valid for correlated inputs (GUM G.4.1); \
             the effective degrees of freedom is approximate"
                .into(),
        );
    }
    let mut denom = 0.0;
    for i in 0..n {
        if nu[i].is_finite() && ui_y[i] != 0.0 {
            denom += ui_y[i].powi(4) / nu[i];
        }
    }
    let nu_eff = if denom > 0.0 { (uc.powi(4) / denom).min(f64::MAX) } else { f64::INFINITY };
    // G.2c: ν_eff ≤ Σ ν_i.
    let nu_sum: f64 = (0..n).filter(|&i| ui_y[i] != 0.0).map(|i| nu[i]).sum();
    let nu_eff = nu_eff.min(nu_sum);

    let (k, p) = match options.coverage {
        CoverageSpec::Factor(k) => {
            if !(k > 0.0) {
                return Err(UncError::InvalidOption("coverage factor must be positive".into()));
            }
            (k, 2.0 * crate::special::normal_cdf(k) - 1.0)
        }
        CoverageSpec::Probability(p) => {
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
        estimate: y,
        combined_standard_uncertainty: uc,
        first_order_uncertainty: (first_order_var + correlation_var).max(0.0).sqrt(),
        correlation_variance: correlation_var,
        higher_order_variance: higher_order_var,
        effective_dof: nu_eff,
        coverage_factor: k,
        coverage_probability: p,
        expanded_uncertainty: expanded,
        interval: (y - expanded, y + expanded),
        budget,
        warnings,
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
