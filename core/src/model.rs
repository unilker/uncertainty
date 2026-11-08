//! Measurement model: input quantities, intermediate quantities, the
//! measurement function Y = f(X₁, …, X_N) and correlations between inputs.

use crate::distribution::Distribution;
use crate::error::UncError;
use crate::expr::{self, Expr};
use std::collections::HashMap;

/// An input quantity X_i with its assigned distribution.
#[derive(Clone, Debug, PartialEq)]
pub struct InputQuantity {
    pub name: String,
    pub distribution: Distribution,
    /// Degrees of freedom for the GUM framework overriding the distribution's
    /// default, e.g. a Type B evaluation judged reliable to a stated relative
    /// uncertainty (GUM G.4.2, ν ≈ ½ [Δu/u]⁻²).
    pub dof: Option<f64>,
    pub unit: Option<String>,
    pub description: Option<String>,
}

impl InputQuantity {
    pub fn new(name: impl Into<String>, distribution: Distribution) -> Self {
        Self { name: name.into(), distribution, dof: None, unit: None, description: None }
    }

    pub fn with_dof(mut self, dof: f64) -> Self {
        self.dof = Some(dof);
        self
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn estimate(&self) -> f64 {
        self.distribution.expectation()
    }

    pub fn gum_standard_uncertainty(&self) -> f64 {
        self.distribution.gum_standard_uncertainty()
    }

    pub fn gum_dof(&self) -> f64 {
        self.dof.unwrap_or_else(|| self.distribution.gum_degrees_of_freedom())
    }
}

/// Correlation coefficient r(x_i, x_j) between two input estimates [GUM 5.2.2].
#[derive(Clone, Debug, PartialEq)]
pub struct Correlation {
    pub a: String,
    pub b: String,
    pub r: f64,
}

/// A complete, validated measurement model.
#[derive(Clone, Debug)]
pub struct Model {
    pub output_name: String,
    pub output_unit: Option<String>,
    pub inputs: Vec<InputQuantity>,
    /// Output expression with intermediate quantities inlined.
    pub expr: Expr,
    /// Source text of the output expression and of intermediates, for reports.
    pub output_source: String,
    pub intermediates: Vec<(String, String)>,
    /// Correlation matrix r(x_i, x_j) (identity if no correlations given).
    pub correlation: Vec<Vec<f64>>,
    /// Ensembles: groups of inputs evaluated from the same set of data, with a
    /// common number of degrees of freedom (e.g. means of paired indications).
    /// Each ensemble counts as one component in the Welch–Satterthwaite
    /// formula, so correlations inside it do not invalidate ν_eff
    /// [Willink, Metrologia 44 (2007) 340, §4.1.1].
    pub ensembles: Vec<Vec<usize>>,
}

impl Model {
    pub fn builder() -> ModelBuilder {
        ModelBuilder::default()
    }

    pub fn n_inputs(&self) -> usize {
        self.inputs.len()
    }

    pub fn input_names(&self) -> Vec<String> {
        self.inputs.iter().map(|q| q.name.clone()).collect()
    }

    pub fn estimates(&self) -> Vec<f64> {
        self.inputs.iter().map(InputQuantity::estimate).collect()
    }

    pub fn has_correlations(&self) -> bool {
        let n = self.n_inputs();
        (0..n).any(|i| (0..n).any(|j| i != j && self.correlation[i][j] != 0.0))
    }

    /// Evaluates the model function at the given input values.
    pub fn evaluate(&self, x: &[f64]) -> f64 {
        self.expr.eval(x)
    }
}

/// Builder for [`Model`].
#[derive(Clone, Debug, Default)]
pub struct ModelBuilder {
    inputs: Vec<InputQuantity>,
    intermediates: Vec<(String, String)>,
    output: Option<(String, String)>,
    output_unit: Option<String>,
    correlations: Vec<Correlation>,
    ensembles: Vec<Vec<String>>,
}

impl ModelBuilder {
    /// Adds an input quantity.
    pub fn input(mut self, q: InputQuantity) -> Self {
        self.inputs.push(q);
        self
    }

    /// Shorthand for an input quantity with only a name and distribution.
    pub fn quantity(self, name: &str, distribution: Distribution) -> Self {
        self.input(InputQuantity::new(name, distribution))
    }

    /// Defines an intermediate quantity by an expression of inputs and of
    /// previously defined intermediates (a sub-model, JCGM GUM-6 §7). It is
    /// substituted into the output expression, so uncertainties propagate
    /// exactly as for a single explicit model.
    pub fn intermediate(mut self, name: &str, expression: &str) -> Self {
        self.intermediates.push((name.to_string(), expression.to_string()));
        self
    }

    /// Sets the output quantity and its measurement function.
    pub fn output(mut self, name: &str, expression: &str) -> Self {
        self.output = Some((name.to_string(), expression.to_string()));
        self
    }

    pub fn output_unit(mut self, unit: &str) -> Self {
        self.output_unit = Some(unit.to_string());
        self
    }

    /// Declares a correlation coefficient between two input quantities.
    pub fn correlation(mut self, a: &str, b: &str, r: f64) -> Self {
        self.correlations.push(Correlation { a: a.to_string(), b: b.to_string(), r });
        self
    }

    /// Declares inputs evaluated from the same set of data (an ensemble);
    /// they must have the same finite degrees of freedom.
    pub fn ensemble(mut self, names: &[&str]) -> Self {
        self.ensembles.push(names.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn build(self) -> Result<Model, UncError> {
        let (output_name, output_source) =
            self.output.ok_or_else(|| UncError::InvalidOption("the model has no output expression".into()))?;

        let mut index: HashMap<String, usize> = HashMap::new();
        for (i, q) in self.inputs.iter().enumerate() {
            if q.name.is_empty() {
                return Err(UncError::InvalidOption("input quantity with empty name".into()));
            }
            if index.insert(q.name.clone(), i).is_some() {
                return Err(UncError::DuplicateName(q.name.clone()));
            }
            q.distribution.validate().map_err(|e| match e {
                UncError::InvalidDistribution(m) => UncError::InvalidDistribution(format!("{}: {m}", q.name)),
                other => other,
            })?;
            if let Some(nu) = q.dof {
                if !(nu > 0.0) {
                    return Err(UncError::InvalidOption(format!("{}: degrees of freedom must be > 0", q.name)));
                }
            }
        }

        let mut defined: HashMap<String, Expr> = HashMap::new();
        for (name, src) in &self.intermediates {
            if index.contains_key(name) || defined.contains_key(name) || *name == output_name {
                return Err(UncError::DuplicateName(name.clone()));
            }
            let e = {
                let lookup = |n: &str| -> Option<Expr> {
                    index.get(n).map(|&i| Expr::Var(i)).or_else(|| defined.get(n).cloned())
                };
                expr::parse(src, &lookup)?
            };
            defined.insert(name.clone(), e);
        }
        if index.contains_key(&output_name) {
            return Err(UncError::DuplicateName(output_name));
        }
        let lookup =
            |n: &str| -> Option<Expr> { index.get(n).map(|&i| Expr::Var(i)).or_else(|| defined.get(n).cloned()) };
        let expr = expr::parse(&output_source, &lookup)?;

        let n = self.inputs.len();
        let mut correlation = vec![vec![0.0; n]; n];
        for (i, row) in correlation.iter_mut().enumerate() {
            row[i] = 1.0;
        }
        for c in &self.correlations {
            let i = *index.get(&c.a).ok_or_else(|| UncError::UnknownSymbol(c.a.clone()))?;
            let j = *index.get(&c.b).ok_or_else(|| UncError::UnknownSymbol(c.b.clone()))?;
            if i == j {
                return Err(UncError::InvalidCorrelation(format!("{} is correlated with itself", c.a)));
            }
            if !(c.r >= -1.0 && c.r <= 1.0) {
                return Err(UncError::InvalidCorrelation(format!("r({}, {}) must lie in [−1, 1]", c.a, c.b)));
            }
            correlation[i][j] = c.r;
            correlation[j][i] = c.r;
        }

        let mut ensembles: Vec<Vec<usize>> = Vec::new();
        for names in &self.ensembles {
            if names.len() < 2 {
                return Err(UncError::InvalidOption("an ensemble needs at least two inputs".into()));
            }
            let mut members = Vec::new();
            for name in names {
                let i = *index.get(name).ok_or_else(|| UncError::UnknownSymbol(name.clone()))?;
                if members.contains(&i) || ensembles.iter().any(|e| e.contains(&i)) {
                    return Err(UncError::InvalidOption(format!("{name} appears in more than one ensemble")));
                }
                members.push(i);
            }
            let nu = self.inputs[members[0]].gum_dof();
            if !nu.is_finite() || members.iter().any(|&i| self.inputs[i].gum_dof() != nu) {
                return Err(UncError::InvalidOption(format!(
                    "the inputs of an ensemble ({}) must have the same finite degrees of freedom",
                    names.join(", ")
                )));
            }
            ensembles.push(members);
        }

        let model = Model {
            output_name,
            output_unit: self.output_unit,
            inputs: self.inputs,
            expr,
            output_source,
            intermediates: self.intermediates,
            correlation,
            ensembles,
        };
        let y = model.evaluate(&model.estimates());
        if !y.is_finite() {
            return Err(UncError::ModelNotDefined(format!("f(x) = {y} at the input estimates")));
        }
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intermediates_are_inlined() {
        let m = Model::builder()
            .quantity("a", Distribution::Normal { mean: 1.0, sd: 0.1 })
            .quantity("b", Distribution::Normal { mean: 2.0, sd: 0.1 })
            .intermediate("s", "a + b")
            .intermediate("t", "2 * s")
            .output("y", "t * a")
            .build()
            .unwrap();
        assert_eq!(m.evaluate(&m.estimates()), 6.0);
    }

    #[test]
    fn build_errors() {
        let base = || Model::builder().quantity("a", Distribution::Normal { mean: 1.0, sd: 0.1 });
        assert!(matches!(base().build(), Err(UncError::InvalidOption(_))));
        assert!(matches!(base().output("y", "a + b").build(), Err(UncError::UnknownSymbol(_))));
        assert!(matches!(
            base().quantity("a", Distribution::Constant { value: 1.0 }).output("y", "a").build(),
            Err(UncError::DuplicateName(_))
        ));
        assert!(matches!(
            base().output("y", "a").correlation("a", "a", 0.5).build(),
            Err(UncError::InvalidCorrelation(_))
        ));
        assert!(matches!(base().output("y", "1 / (a - 1)").build(), Err(UncError::ModelNotDefined(_))));
        assert!(matches!(
            base().quantity("b", Distribution::Normal { mean: 0.0, sd: -1.0 }).output("y", "a").build(),
            Err(UncError::InvalidDistribution(_))
        ));
    }
}
