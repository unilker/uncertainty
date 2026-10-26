//! Top-down projects: measurement uncertainty from validation and
//! quality-control data entered as raw data (Nordtest TR 537; Eurachem/CITAC
//! MUPR 2026).
//!
//! A top-down project lists the components of the budget:
//!
//! * `precision` — within-laboratory reproducibility from a control-chart
//!   series, groups of replicates (pooled), duplicate pairs, control limits or
//!   a stated value;
//! * `bias` — from certified reference materials, proficiency tests or
//!   recovery experiments (Nordtest §6);
//! * `recovery` — mean recovery with its uncertainty and the significance
//!   test that decides whether results are corrected (MUPR §8.2);
//! * `components` — further standard uncertainties (e.g. stock solution,
//!   dilution).
//!
//! Every component may be restricted to a concentration range, so precision
//! models with an absolute part at low and a relative part at high
//! concentrations are written as two components. The budget is evaluated at
//! each of the `levels` (measured values); without levels all components
//! must be relative and the result is a relative uncertainty.
//!
//! **Convention:** relative quantities are always entered and reported in
//! per cent, in fields whose names end in `_percent`; all other fields are in
//! the unit of the measured quantity.

use crate::schema::{Check, Text};
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uncertainty_core::special::coverage_factor;
use uncertainty_core::topdown::{self as td, Component, RecoveryHomogeneity, RecoveryTest, TopDownBudget};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: what.to_string(), message: message.into() }
}

fn default_true() -> bool {
    true
}

fn default_two() -> f64 {
    2.0
}

fn default_one() -> u32 {
    1
}

fn default_confidence() -> f64 {
    0.95
}

// ---------------------------------------------------------------------------
// Specification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopDownSpec {
    /// Name of the measurand, e.g. "c(NH₄-N)".
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Measured values at which the uncertainty is evaluated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub levels: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub precision: Vec<PrecisionSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bias: Vec<BiasSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<RecoverySpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ComponentSpec>,
    /// Fixed coverage factor (default 2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_factor: Option<f64>,
    /// Coverage probability for k from the t-distribution with `coverage_dof`
    /// degrees of freedom (default ∞).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_probability: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_dof: Option<f64>,
}

/// Concentration range [min, max) in which a component applies.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct LevelRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

impl LevelRange {
    pub fn contains(&self, x: f64) -> bool {
        self.min.map_or(true, |m| x >= m) && self.max.map_or(true, |m| x < m)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrecisionSpec {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    pub data: PrecisionData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<LevelRange>,
    /// Precision of the mean of `replicates` results on each of `days` days
    /// [MUPR Eq. (5)].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_of: Option<MeanOf>,
    /// Express an absolute standard deviation relative to this value, or to
    /// the mean of the data (`"mean"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_relative_at: Option<At>,
    /// Express a relative standard deviation as an absolute one at this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_absolute_at: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum At {
    Value(f64),
    Keyword(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeanOf {
    pub replicates: usize,
    pub days: usize,
    /// Repeatability standard deviation s_r (same kind as the precision).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeatability: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeatability_percent: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum PrecisionData {
    /// Results of one control sample (control chart): s with ν = n − 1.
    Series {
        values: Vec<f64>,
        #[serde(default)]
        relative: bool,
    },
    /// Pooled standard deviation of several groups [MUPR Eq. (1)–(2)].
    Groups {
        groups: Vec<GroupSpec>,
        #[serde(default)]
        relative: bool,
    },
    /// Duplicate pairs, optionally only those whose mean lies in
    /// [min_mean, max_mean).
    Duplicates {
        pairs: Vec<[f64; 2]>,
        #[serde(default)]
        relative: bool,
        #[serde(default)]
        method: DuplicateMethod,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_mean: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_mean: Option<f64>,
    },
    /// Control-chart limits ±L set at k standard deviations [Nordtest §5.1].
    ControlLimits {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        limit: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        limit_percent: Option<f64>,
        #[serde(default = "default_two")]
        k: f64,
    },
    /// A standard deviation stated elsewhere (e.g. a validation report).
    Given {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_percent: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        dof: Option<f64>,
    },
    /// Standard deviation as a straight-line function of the level,
    /// s(c) = a + b·c, stated or fitted by least squares to (c, s) points
    /// [MUPR Example B3]. With a dilution factor F the line describes the
    /// diluted test solution: s(c) = F·(a + b·c/F).
    Linear {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        points: Option<Vec<[f64; 2]>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        intercept: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        slope: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        dilution_factor: Option<f64>,
    },
    /// s(c) = √(s₀² + (c·s′₁)²) [MUPR Eq. (A.1)–(A.4)].
    Continuous { sd0: f64, rsd1_percent: f64 },
    /// Reproducibility and repeatability of a collaborative study
    /// [ISO 21748:2017 Formulae (3), (14), (15)]: u² = s_L² + s_r²/n_r
    /// (+ u²(δ̂)), with the laboratory's own repeatability in place of s_r if
    /// given (7.3.2). All fields absolute, or all `_percent` (relative).
    Reproducibility {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_reproducibility: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_reproducibility_percent: Option<f64>,
        /// Between-laboratory s_L, instead of s_R.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_between_labs: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_between_labs_percent: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_repeatability: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_repeatability_percent: Option<f64>,
        /// The laboratory's repeatability s_l (7.3.2).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lab_repeatability: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lab_repeatability_percent: Option<f64>,
        /// Number n_r of complete replicates averaged for a result (Table 1).
        #[serde(default = "default_one_usize")]
        replicates: usize,
        /// Uncertainty of the method bias estimated in the collaborative
        /// study, Formula (15).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        method_bias: Option<MethodBiasSpec>,
        /// Check of the laboratory component of bias, Formulae (4)–(5).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bias_check: Option<BiasCheckSpec>,
    },
    /// Two-factor staggered-nested design [ISO 5725-3:2023 7.2, Annex C.1]:
    /// rows of (two results on day 1, one result on day 2), one row per
    /// sample, group or laboratory.
    StaggeredNested {
        results: Vec<[f64; 3]>,
        /// Which standard deviation is used: `repeatability`,
        /// `intermediate` (time-different, default) or `reproducibility`.
        #[serde(default)]
        estimate: NestedEstimate,
        /// Relative to the grand mean.
        #[serde(default)]
        relative: bool,
    },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NestedEstimate {
    Repeatability,
    #[default]
    Intermediate,
    Reproducibility,
}

fn default_one_usize() -> usize {
    1
}

/// Formula (15): u²(δ̂) = (s_R² − (1 − 1/n) s_r²)/p + u²(μ̂).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MethodBiasSpec {
    /// Number of laboratories p and replicates n of the trueness study.
    pub labs: usize,
    pub replicates: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_reference: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_reference_percent: Option<f64>,
}

/// Formulae (4) and (5): |Δ_l| < 2σ_D with s_D² = s_L² + s_w²/n_l.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BiasCheckSpec {
    /// Laboratory bias Δ_l on a reference material (mean − certified value).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bias: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bias_percent: Option<f64>,
    /// Within-laboratory standard deviation s_w of the n_l replicates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_within: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_within_percent: Option<f64>,
    pub n: usize,
}

/// A precision standard deviation that depends on the level (absolute).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LevelModel {
    Linear { intercept: f64, slope: f64, dilution_factor: f64, correlation: Option<f64> },
    Continuous { sd0: f64, rsd1: f64 },
}

impl LevelModel {
    /// Absolute standard deviation at level c.
    pub fn sd_at(&self, c: f64) -> f64 {
        match *self {
            LevelModel::Linear { intercept, slope, dilution_factor, .. } => {
                dilution_factor * (intercept + slope * c / dilution_factor)
            }
            LevelModel::Continuous { sd0, rsd1 } => td::PrecisionModel::Continuous { sd0, rsd1 }.sd_at(c),
        }
    }
}

/// One group of replicate results, as raw values or as (s, n).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DuplicateMethod {
    /// s = √(Σd²/2N), or pooled relative s of the pairs [Nordtest App. 5–6].
    #[default]
    Pooled,
    /// s of the normalised differences / √2, mean difference removed
    /// [EURACHEM CG4 A4]; relative only.
    NormalisedDifferences,
    /// Mean range / 1.128 [MUPR §8.1.3].
    MeanRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BiasKind {
    Crm,
    ProficiencyTests,
    Recovery,
}

/// Bias component u(bias) = √(RMS_bias² + ū(C_ref)²), or for one CRM
/// measured n times √(b² + (s/√n)² + u(C_ref)²) [Nordtest TR 537 §6].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BiasSpec {
    #[serde(default = "default_bias_name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    pub kind: BiasKind,
    /// Evaluate in relative terms (per cent; default) or in the unit of the
    /// measured quantity.
    #[serde(default = "default_true")]
    pub relative: bool,
    pub references: Vec<ReferenceSpec>,
    /// Reference uncertainty shared by all references (e.g. of a spike).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_reference: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_reference_percent: Option<f64>,
    /// Components of a shared reference uncertainty, combined in quadrature.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub u_reference_components_percent: Vec<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<LevelRange>,
}

fn default_bias_name() -> String {
    "u(bias)".into()
}

/// A CRM, proficiency-test round or recovery experiment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ReferenceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Certified or assigned value C.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_reference: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_reference_percent: Option<f64>,
    /// Expanded uncertainty of C with its coverage factor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k: Option<f64>,
    /// Reproducibility s_R and number of laboratories of a proficiency test:
    /// u(C) = s_R/√n_Lab.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_reproducibility: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_reproducibility_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labs: Option<usize>,
    /// The laboratory's results, or their mean, s and n.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,
    /// Bias stated directly, or a recovery (bias = R − 100 %).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bias: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bias_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_percent: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CorrectionRule {
    /// Correct (c/R̄) if R̄ differs significantly from 1 [MUPR Eq. (13)].
    #[default]
    IfSignificant,
    Always,
    Never,
}

/// Mean recovery from reference materials and spiked samples [MUPR §8.2].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoverySpec {
    #[serde(default = "default_recovery_name")]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    pub tests: Vec<RecoveryTestSpec>,
    /// Add the standard deviation of the individual recoveries when they
    /// vary with the matrix [MUPR §8.2.7].
    #[serde(default)]
    pub include_sd_of_recoveries: bool,
    /// Degrees of freedom of the significance test (default: Σ(nᵢ − 1) over
    /// the tests, or N − 1 if that is zero or s(R̄) is included).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dof: Option<f64>,
    #[serde(default = "default_confidence")]
    pub confidence: f64,
    #[serde(default)]
    pub correction: CorrectionRule,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<LevelRange>,
}

fn default_recovery_name() -> String {
    "u(R̄)".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecoveryTestSpec {
    /// Reference material or spiked blank: R = c̄/C [MUPR Eq. (8)–(9)].
    Reference {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        measured: Option<Vec<f64>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mean: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        n: Option<usize>,
        reference: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        u_reference: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        expanded: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        k: Option<f64>,
    },
    /// Spiked sample with native analyte: R = (c̄ − c̄₀)/c₊ [MUPR Eq. (10)–(11)].
    Spiked {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        after: Option<Vec<f64>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mean_after: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_after: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        n_after: Option<usize>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        before: Option<Vec<f64>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mean_before: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_before: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        n_before: Option<usize>,
        spike: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        u_spike: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        u_spike_percent: Option<f64>,
    },
    /// Individual recoveries in per cent, or their mean, s and n:
    /// R̄ᵢ = mean, u = s/√n.
    Recoveries {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        values_percent: Option<Vec<f64>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mean_percent: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd_percent: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        n: Option<usize>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LimitDistribution {
    #[default]
    Rectangular,
    Triangular,
}

/// A further component: a standard uncertainty, or limits ±a with a
/// rectangular or triangular distribution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentSpec {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_width_percent: Option<f64>,
    #[serde(default)]
    pub distribution: LimitDistribution,
    /// Number of times the component enters the budget.
    #[serde(default = "default_one")]
    pub multiplicity: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<LevelRange>,
}

// ---------------------------------------------------------------------------
// Expected values
// ---------------------------------------------------------------------------

/// Reference values of a top-down project. Relative quantities in per cent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedTopDown {
    /// Standard uncertainty of each component by name (per cent if relative).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub components: BTreeMap<String, Check>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub bias: BTreeMap<String, ExpectedBias>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<ExpectedRecovery>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub levels: Vec<ExpectedLevel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedBias {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rms: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_u_reference: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedRecovery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_percent: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_uncertainty_percent: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_percent: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistic: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chi_squared: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub significant: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedLevel {
    /// The level as listed in `topdown.levels`; omitted for a relative-only
    /// project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_uncertainty: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_uncertainty: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_standard_percent: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_expanded_percent: Option<Check>,
}

// ---------------------------------------------------------------------------
// Results
// ---------------------------------------------------------------------------

/// How a precision estimate was obtained (for reports).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrecisionMethod {
    Series,
    Groups,
    DuplicatesPooled,
    DuplicatesNormalised,
    DuplicatesRange,
    ControlLimits,
    Given,
    Linear,
    Continuous,
    Reproducibility,
    StaggeredNested,
}

/// Breakdown of an ISO 21748 reproducibility-based precision component
/// (fractions if relative).
#[derive(Clone, Debug, PartialEq)]
pub struct ReproducibilityDetail {
    pub s_reproducibility: f64,
    pub s_between_labs: f64,
    pub s_repeatability: f64,
    /// Repeatability used: s_l if given, otherwise s_r.
    pub s_repeatability_used: f64,
    pub lab_repeatability_used: bool,
    pub replicates: usize,
    /// √(s_L² + s²/n_r), the adjusted reproducibility.
    pub s_adjusted: f64,
    pub u_method_bias: Option<f64>,
    /// (|Δ_l|, s_D, |Δ_l| < 2 s_D).
    pub bias_check: Option<(f64, f64, bool)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrecisionResult {
    pub name: String,
    pub method: PrecisionMethod,
    /// Number of results, pairs or groups.
    pub n: usize,
    /// Mean of the data, if any.
    pub mean: Option<f64>,
    /// Standard deviation of the data before `mean_of` and conversions
    /// (fraction if `raw_relative`).
    pub raw_sd: f64,
    pub raw_relative: bool,
    /// Final standard uncertainty (fraction if `relative`).
    pub sd: f64,
    pub relative: bool,
    pub dof: f64,
    pub range: LevelRange,
    /// Level-dependent model (then `sd` and `raw_sd` are not used).
    pub model: Option<LevelModel>,
    pub reproducibility: Option<ReproducibilityDetail>,
    /// (s_r, s_I(T), s_R) of a staggered-nested design (fractions if relative).
    pub nested: Option<(f64, f64, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BiasEntry {
    pub label: String,
    /// Bias and reference uncertainty (fractions if relative).
    pub bias: f64,
    pub u_reference: f64,
    pub sd: Option<f64>,
    pub n: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BiasResult {
    pub name: String,
    pub kind: BiasKind,
    pub relative: bool,
    pub entries: Vec<BiasEntry>,
    pub mean_bias: f64,
    pub rms_bias: f64,
    pub mean_u_reference: f64,
    /// True if the single-CRM formula √(b² + s²/n + u²(C)) was used.
    pub single_reference: bool,
    pub u: f64,
    pub range: LevelRange,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecoveryEntry {
    pub label: String,
    pub recovery: f64,
    pub u: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecoveryResult {
    pub name: String,
    pub entries: Vec<RecoveryEntry>,
    pub mean: f64,
    /// u_R̄ from MUPR Eq. (9)/(11)/(12).
    pub u_mean: f64,
    pub sd_of_recoveries: Option<f64>,
    /// Uncertainty used in the test and budget (with s(R̄) if requested).
    pub u_used: f64,
    pub dof: f64,
    pub confidence: f64,
    pub statistic: f64,
    pub critical: f64,
    pub significant: bool,
    pub corrected: bool,
    /// Relative standard uncertainty entering the budget.
    pub u_relative: f64,
    pub homogeneity: Option<RecoveryHomogeneity>,
    pub range: LevelRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentSource {
    Precision,
    Bias,
    Recovery,
    Additional,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BudgetItem {
    pub name: String,
    pub source: ComponentSource,
    /// Standard uncertainty (fraction if relative).
    pub u: f64,
    pub relative: bool,
    pub multiplicity: u32,
    pub range: LevelRange,
    /// Level-dependent absolute standard uncertainty (overrides `u`).
    pub model: Option<LevelModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LevelResult {
    /// Measured value, or None for a relative-only evaluation.
    pub level: Option<f64>,
    /// Reported value (corrected for recovery where applicable).
    pub value: Option<f64>,
    pub standard_uncertainty: Option<f64>,
    pub relative_standard: f64,
    pub expanded_uncertainty: Option<f64>,
    pub relative_expanded: f64,
    /// Variance share of each budget item (index into `budget`).
    pub shares: Vec<(usize, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopDownAnalysis {
    pub precision: Vec<PrecisionResult>,
    pub bias: Vec<BiasResult>,
    pub recovery: Option<RecoveryResult>,
    pub budget: Vec<BudgetItem>,
    pub coverage_factor: f64,
    pub levels: Vec<LevelResult>,
}

// ---------------------------------------------------------------------------
// Evaluation
// ---------------------------------------------------------------------------

fn mean_sd(v: &[f64], what: &str) -> Result<(f64, f64), ProjectError> {
    if v.len() < 2 {
        return Err(err(what, "at least two values are needed"));
    }
    if v.iter().any(|x| !x.is_finite()) {
        return Err(err(what, "values must be finite"));
    }
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    Ok((m, (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1.0)).sqrt()))
}

/// Exactly one of an absolute and a per-cent value, as (value, relative).
fn one_of(name: &str, abs: Option<f64>, pct: Option<f64>, what: &str) -> Result<(f64, bool), ProjectError> {
    match (abs, pct) {
        (Some(a), None) => Ok((a, false)),
        (None, Some(p)) => Ok((p / 100.0, true)),
        _ => Err(err(name, format!("give either {what} or {what}_percent"))),
    }
}

fn precision(p: &PrecisionSpec) -> Result<PrecisionResult, ProjectError> {
    let name = p.name.as_str();
    let model = match &p.data {
        PrecisionData::Linear { points, intercept, slope, dilution_factor } => {
            let f = dilution_factor.unwrap_or(1.0);
            if !(f.is_finite() && f > 0.0) {
                return Err(err(name, "dilution_factor must be positive"));
            }
            let (a, b, r, n) = match (points, intercept, slope) {
                (Some(pts), None, None) => {
                    let x: Vec<f64> = pts.iter().map(|q| q[0]).collect();
                    let y: Vec<f64> = pts.iter().map(|q| q[1]).collect();
                    let fit = uncertainty_core::calibration::StraightLine::fit(&x, &y)?;
                    (fit.intercept, fit.slope, Some(fit.correlation), pts.len())
                }
                (None, Some(a), Some(b)) => (*a, *b, None, 0),
                _ => return Err(err(name, "give points, or intercept and slope")),
            };
            Some((
                LevelModel::Linear { intercept: a, slope: b, dilution_factor: f, correlation: r },
                PrecisionMethod::Linear,
                n,
            ))
        }
        PrecisionData::Continuous { sd0, rsd1_percent } => {
            Some((LevelModel::Continuous { sd0: *sd0, rsd1: rsd1_percent / 100.0 }, PrecisionMethod::Continuous, 0))
        }
        _ => None,
    };
    if let Some((m, method, n)) = model {
        if p.mean_of.is_some() || p.to_relative_at.is_some() || p.to_absolute_at.is_some() {
            return Err(err(name, "mean_of and conversions are not available for level-dependent models"));
        }
        return Ok(PrecisionResult {
            name: p.name.clone(),
            method,
            n,
            mean: None,
            raw_sd: f64::NAN,
            raw_relative: false,
            sd: f64::NAN,
            relative: false,
            dof: if n > 2 { n as f64 - 2.0 } else { f64::INFINITY },
            range: p.range.unwrap_or_default(),
            model: Some(m),
            reproducibility: None,
            nested: None,
        });
    }
    let mut reproducibility_detail = None;
    let mut nested = None;
    let (method, n, mean, sd, relative, dof) = match &p.data {
        PrecisionData::Series { values, relative } => {
            let (m, s) = mean_sd(values, name)?;
            let sd = if *relative { s / m.abs() } else { s };
            (PrecisionMethod::Series, values.len(), Some(m), sd, *relative, values.len() as f64 - 1.0)
        }
        PrecisionData::Groups { groups, relative } => {
            let mut g = Vec::new();
            for (i, gr) in groups.iter().enumerate() {
                let what = format!("{name}, group {}", i + 1);
                let (s, n) = match (&gr.values, gr.sd, gr.sd_percent) {
                    (Some(v), None, None) => {
                        let (m, s) = mean_sd(v, &what)?;
                        (if *relative { s / m.abs() } else { s }, v.len())
                    }
                    (None, Some(s), None) if !relative => (s, gr.n.ok_or_else(|| err(&what, "n is required"))?),
                    (None, None, Some(s)) if *relative => (s / 100.0, gr.n.ok_or_else(|| err(&what, "n is required"))?),
                    _ => {
                        return Err(err(
                            &what,
                            "give values, or sd and n (absolute) / sd_percent and n (relative) matching `relative`",
                        ))
                    }
                };
                g.push((s, n));
            }
            let pooled = td::pool_standard_deviations(&g)?;
            (PrecisionMethod::Groups, groups.len(), None, pooled.sd, *relative, pooled.dof)
        }
        PrecisionData::Duplicates { pairs, relative, method, min_mean, max_mean } => {
            let range = LevelRange { min: *min_mean, max: *max_mean };
            let sel: Vec<(f64, f64)> =
                pairs.iter().map(|p| (p[0], p[1])).filter(|(a, b)| range.contains(0.5 * (a + b))).collect();
            if sel.is_empty() {
                return Err(err(name, "no duplicate pair in the selected range"));
            }
            let grand = sel.iter().map(|(a, b)| a + b).sum::<f64>() / (2.0 * sel.len() as f64);
            let (m, r) = match (method, relative) {
                (DuplicateMethod::Pooled, false) => (PrecisionMethod::DuplicatesPooled, td::sd_from_duplicates(&sel)?),
                (DuplicateMethod::Pooled, true) => {
                    (PrecisionMethod::DuplicatesPooled, td::relative_sd_from_duplicates(&sel)?)
                }
                (DuplicateMethod::NormalisedDifferences, true) => {
                    (PrecisionMethod::DuplicatesNormalised, td::relative_sd_from_normalised_differences(&sel)?)
                }
                (DuplicateMethod::NormalisedDifferences, false) => {
                    return Err(err(name, "normalised differences give a relative standard deviation"))
                }
                (DuplicateMethod::MeanRange, rel) => {
                    let ranges: Vec<f64> = sel
                        .iter()
                        .map(|(a, b)| if *rel { (a - b).abs() / (0.5 * (a + b)) } else { (a - b).abs() })
                        .collect();
                    let mr = ranges.iter().sum::<f64>() / ranges.len() as f64;
                    (
                        PrecisionMethod::DuplicatesRange,
                        td::PooledSd { sd: td::sd_from_mean_range(mr), dof: ranges.len() as f64 },
                    )
                }
            };
            (m, sel.len(), Some(grand), r.sd, *relative, r.dof)
        }
        PrecisionData::ControlLimits { limit, limit_percent, k } => {
            let (l, rel) = one_of(name, *limit, *limit_percent, "limit")?;
            (PrecisionMethod::ControlLimits, 0, None, td::sd_from_control_limits(l, *k), rel, f64::INFINITY)
        }
        PrecisionData::Given { sd, sd_percent, dof } => {
            let (s, rel) = one_of(name, *sd, *sd_percent, "sd")?;
            (PrecisionMethod::Given, 0, None, s, rel, dof.unwrap_or(f64::INFINITY))
        }
        PrecisionData::StaggeredNested { results, estimate, relative } => {
            let a = td::staggered_nested(results)?;
            let f = if *relative { 1.0 / a.mean.abs() } else { 1.0 };
            let (r, i, big_r) = (a.repeatability() * f, a.intermediate() * f, a.reproducibility() * f);
            nested = Some((r, i, big_r));
            let (sd, dof) = match estimate {
                NestedEstimate::Repeatability => (r, a.dof[2]),
                NestedEstimate::Intermediate => (i, f64::INFINITY),
                NestedEstimate::Reproducibility => (big_r, f64::INFINITY),
            };
            (PrecisionMethod::StaggeredNested, results.len(), Some(a.mean), sd, *relative, dof)
        }
        PrecisionData::Reproducibility { .. } => {
            let (u, rel, detail) = reproducibility(name, &p.data)?;
            reproducibility_detail = Some(detail);
            (PrecisionMethod::Reproducibility, 0, None, u, rel, f64::INFINITY)
        }
        PrecisionData::Linear { .. } | PrecisionData::Continuous { .. } => unreachable!("handled above"),
    };
    let reproducibility = reproducibility_detail;
    if !(sd.is_finite() && sd >= 0.0) {
        return Err(err(name, "the standard deviation is not a finite non-negative number"));
    }
    let (raw_sd, raw_relative) = (sd, relative);
    let mut sd = sd;
    let mut relative = relative;
    if let Some(mo) = &p.mean_of {
        let s_r = match (mo.repeatability, mo.repeatability_percent) {
            (None, None) => 0.0,
            (Some(s), None) if !relative => s,
            (None, Some(s)) if relative => s / 100.0,
            _ => return Err(err(name, "mean_of: repeatability must be of the same kind (absolute/relative)")),
        };
        sd = td::intermediate_precision_of_mean(s_r, sd, mo.replicates, mo.days)?;
    }
    match (&p.to_relative_at, p.to_absolute_at) {
        (None, None) => {}
        (Some(at), None) if !relative => {
            let v = match at {
                At::Value(v) => *v,
                At::Keyword(k) if k == "mean" => mean.ok_or_else(|| err(name, "the data have no mean"))?,
                At::Keyword(k) => {
                    return Err(err(name, format!("to_relative_at: unknown keyword '{k}' (use \"mean\")")))
                }
            };
            sd /= v.abs();
            relative = true;
        }
        (None, Some(v)) if relative => {
            sd *= v.abs();
            relative = false;
        }
        _ => return Err(err(name, "to_relative_at needs an absolute and to_absolute_at a relative precision")),
    }
    Ok(PrecisionResult {
        name: p.name.clone(),
        method,
        n,
        mean,
        raw_sd,
        raw_relative,
        sd,
        relative,
        dof,
        range: p.range.unwrap_or_default(),
        model: None,
        reproducibility,
        nested,
    })
}

/// Reads an absolute or a `_percent` value; `None` if neither is given.
fn kind_of(name: &str, what: &str, abs: Option<f64>, pct: Option<f64>) -> Result<Option<(f64, bool)>, ProjectError> {
    match (abs, pct) {
        (Some(_), Some(_)) => Err(err(name, format!("give either {what} or {what}_percent"))),
        (Some(v), None) => Ok(Some((v, false))),
        (None, Some(v)) => Ok(Some((v / 100.0, true))),
        (None, None) => Ok(None),
    }
}

/// ISO 21748:2017 reproducibility-based precision (see [`PrecisionData::Reproducibility`]).
fn reproducibility(name: &str, d: &PrecisionData) -> Result<(f64, bool, ReproducibilityDetail), ProjectError> {
    let PrecisionData::Reproducibility {
        sd_reproducibility,
        sd_reproducibility_percent,
        sd_between_labs,
        sd_between_labs_percent,
        sd_repeatability,
        sd_repeatability_percent,
        lab_repeatability,
        lab_repeatability_percent,
        replicates,
        method_bias,
        bias_check,
    } = d
    else {
        unreachable!()
    };
    let big_r = kind_of(name, "sd_reproducibility", *sd_reproducibility, *sd_reproducibility_percent)?;
    let big_l = kind_of(name, "sd_between_labs", *sd_between_labs, *sd_between_labs_percent)?;
    let r = kind_of(name, "sd_repeatability", *sd_repeatability, *sd_repeatability_percent)?
        .ok_or_else(|| err(name, "sd_repeatability (or _percent) is required"))?;
    let l = kind_of(name, "lab_repeatability", *lab_repeatability, *lab_repeatability_percent)?;
    let relative = r.1;
    let same = |x: Option<(f64, bool)>| x.map_or(true, |(_, rel)| rel == relative);
    if !(same(big_r) && same(big_l) && same(l)) {
        return Err(err(name, "all standard deviations must be absolute, or all given in per cent"));
    }
    let s_r = r.0;
    let (s_big_r, s_l_between) = match (big_r, big_l) {
        (Some((sr_big, _)), None) => {
            if sr_big < s_r {
                return Err(err(name, "s_R must not be smaller than s_r"));
            }
            (sr_big, (sr_big * sr_big - s_r * s_r).sqrt())
        }
        (None, Some((sl, _))) => ((sl * sl + s_r * s_r).sqrt(), sl),
        _ => return Err(err(name, "give either sd_reproducibility or sd_between_labs")),
    };
    if *replicates == 0 {
        return Err(err(name, "replicates must be at least 1"));
    }
    let s_rep = l.map_or(s_r, |x| x.0);
    let s_adjusted = (s_l_between * s_l_between + s_rep * s_rep / *replicates as f64).sqrt();
    let u_method_bias = match method_bias {
        None => None,
        Some(mb) => {
            if mb.labs == 0 || mb.replicates == 0 {
                return Err(err(name, "method_bias: labs and replicates must be at least 1"));
            }
            let u_ref = kind_of(name, "method_bias.u_reference", mb.u_reference, mb.u_reference_percent)?;
            if u_ref.is_some_and(|(_, rel)| rel != relative) {
                return Err(err(name, "method_bias.u_reference must be of the same kind as the standard deviations"));
            }
            let n = mb.replicates as f64;
            let v =
                (s_big_r * s_big_r - (1.0 - 1.0 / n) * s_r * s_r) / mb.labs as f64 + u_ref.map_or(0.0, |x| x.0 * x.0);
            Some(v.max(0.0).sqrt())
        }
    };
    let check = match bias_check {
        None => None,
        Some(bc) => {
            let delta = kind_of(name, "bias_check.bias", bc.bias, bc.bias_percent)?
                .ok_or_else(|| err(name, "bias_check needs bias or bias_percent"))?;
            let sw = kind_of(name, "bias_check.sd_within", bc.sd_within, bc.sd_within_percent)?
                .ok_or_else(|| err(name, "bias_check needs sd_within or sd_within_percent"))?;
            if delta.1 != relative || sw.1 != relative || bc.n == 0 {
                return Err(err(name, "bias_check: same kind as the standard deviations and n ≥ 1"));
            }
            let s_d = (s_l_between * s_l_between + sw.0 * sw.0 / bc.n as f64).sqrt();
            Some((delta.0.abs(), s_d, delta.0.abs() < 2.0 * s_d))
        }
    };
    let total = (s_adjusted * s_adjusted + u_method_bias.map_or(0.0, |u| u * u)).sqrt();
    Ok((
        total,
        relative,
        ReproducibilityDetail {
            s_reproducibility: s_big_r,
            s_between_labs: s_l_between,
            s_repeatability: s_r,
            s_repeatability_used: s_rep,
            lab_repeatability_used: l.is_some(),
            replicates: *replicates,
            s_adjusted,
            u_method_bias,
            bias_check: check,
        },
    ))
}

fn bias(b: &BiasSpec) -> Result<BiasResult, ProjectError> {
    let name = b.name.as_str();
    if b.references.is_empty() {
        return Err(err(name, "no references"));
    }
    let shared = if !b.u_reference_components_percent.is_empty() {
        if b.u_reference.is_some() || b.u_reference_percent.is_some() {
            return Err(err(name, "give one shared reference uncertainty"));
        }
        Some((td::root_sum_of_squares(&b.u_reference_components_percent) / 100.0, true))
    } else {
        match (b.u_reference, b.u_reference_percent) {
            (None, None) => None,
            (a, p) => Some(one_of(name, a, p, "u_reference")?),
        }
    };
    let mut entries = Vec::new();
    for (i, r) in b.references.iter().enumerate() {
        let label = r.label.clone().unwrap_or_else(|| format!("{}", i + 1));
        let what = format!("{name}, reference {label}");
        // Converts a value of the given kind to the evaluation's kind.
        let conv = |v: f64, rel: bool| -> Result<f64, ProjectError> {
            if rel == b.relative {
                Ok(v)
            } else {
                let c = r.reference.ok_or_else(|| err(&what, "the reference value is needed to convert"))?;
                Ok(if b.relative { v / c } else { v * c })
            }
        };
        let (mean, sd_abs, n) = match (&r.measured, r.mean) {
            (Some(v), None) => {
                let (m, s) = mean_sd(v, &what)?;
                (Some(m), Some(s), Some(v.len()))
            }
            (None, m) => (m, None, r.n),
            _ => return Err(err(&what, "give measured values or their mean, not both")),
        };
        let sd = match (sd_abs, r.sd, r.sd_percent) {
            (Some(s), None, None) | (None, Some(s), None) => {
                Some(if b.relative { s / mean.ok_or_else(|| err(&what, "mean needed"))?.abs() } else { s })
            }
            (None, None, Some(s)) => Some(if b.relative { s / 100.0 } else { s / 100.0 * mean.unwrap_or(f64::NAN) }),
            (None, None, None) => None,
            _ => return Err(err(&what, "give one standard deviation")),
        };
        let bias = match (r.bias, r.bias_percent, r.recovery_percent) {
            (Some(v), None, None) => conv(v, false)?,
            (None, Some(v), None) => conv(v / 100.0, true)?,
            (None, None, Some(rp)) => conv(rp / 100.0 - 1.0, true)?,
            (None, None, None) => {
                let c = r.reference.ok_or_else(|| err(&what, "give the reference value or the bias"))?;
                let m = mean.ok_or_else(|| err(&what, "give measured values, mean or the bias"))?;
                if b.relative {
                    (m - c) / c
                } else {
                    m - c
                }
            }
            _ => return Err(err(&what, "give one of bias, bias_percent, recovery_percent")),
        };
        let own = [
            r.u_reference.map(|u| (u, false)),
            r.u_reference_percent.map(|u| (u / 100.0, true)),
            match (r.expanded, r.k) {
                (Some(u), Some(k)) if k > 0.0 => Some((u / k, false)),
                (None, None) => None,
                _ => return Err(err(&what, "expanded needs k > 0")),
            },
            match (r.sd_reproducibility, r.sd_reproducibility_percent, r.labs) {
                (None, None, None) => None,
                (Some(s), None, Some(l)) => Some((td::u_cref_from_proficiency_test(s, l), false)),
                (None, Some(s), Some(l)) => Some((td::u_cref_from_proficiency_test(s, l) / 100.0, true)),
                _ => return Err(err(&what, "give sd_reproducibility (or _percent) with labs")),
            },
        ];
        let given: Vec<(f64, bool)> = own.into_iter().flatten().collect();
        let (u, rel) = match (given.as_slice(), shared) {
            ([one], _) => *one,
            ([], Some(s)) => s,
            ([], None) => return Err(err(&what, "no reference uncertainty")),
            _ => return Err(err(&what, "give one reference uncertainty")),
        };
        entries.push(BiasEntry { label, bias, u_reference: conv(u, rel)?, sd, n });
    }
    let biases: Vec<f64> = entries.iter().map(|e| e.bias).collect();
    let urefs: Vec<f64> = entries.iter().map(|e| e.u_reference).collect();
    let single = b.kind == BiasKind::Crm && entries.len() == 1 && entries[0].sd.is_some() && entries[0].n.is_some();
    let u = if single {
        let e = &entries[0];
        td::u_bias_single_reference(e.bias, e.sd.unwrap(), e.n.unwrap(), e.u_reference)
    } else {
        td::u_bias_from_references(&biases, &urefs)?
    };
    Ok(BiasResult {
        name: b.name.clone(),
        kind: b.kind,
        relative: b.relative,
        mean_bias: biases.iter().sum::<f64>() / biases.len() as f64,
        rms_bias: td::root_mean_square(&biases),
        mean_u_reference: urefs.iter().sum::<f64>() / urefs.len() as f64,
        entries,
        single_reference: single,
        u,
        range: b.range.unwrap_or_default(),
    })
}

/// Mean, s and n from raw values or from stated statistics.
fn stats(
    what: &str,
    values: &Option<Vec<f64>>,
    mean: Option<f64>,
    sd: Option<f64>,
    n: Option<usize>,
) -> Result<(f64, f64, usize), ProjectError> {
    match (values, mean, sd, n) {
        (Some(v), None, None, None) => {
            let (m, s) = mean_sd(v, what)?;
            Ok((m, s, v.len()))
        }
        (None, Some(m), Some(s), Some(n)) => Ok((m, s, n)),
        (None, Some(m), None, Some(1)) => Ok((m, 0.0, 1)),
        _ => Err(err(what, "give raw values, or mean, sd and n")),
    }
}

fn recovery(r: &RecoverySpec) -> Result<RecoveryResult, ProjectError> {
    let name = r.name.as_str();
    let mut tests = Vec::new();
    let mut labels = Vec::new();
    let mut dof_sum = 0.0;
    for (i, t) in r.tests.iter().enumerate() {
        let (label, test, nu) = match t {
            RecoveryTestSpec::Reference { label, measured, mean, sd, n, reference, u_reference, expanded, k } => {
                let what = format!("{name}, test {}", i + 1);
                let (m, s, n) = stats(&what, measured, *mean, *sd, *n)?;
                let u = match (u_reference, expanded, k) {
                    (Some(u), None, None) => *u,
                    (None, Some(u), Some(k)) if *k > 0.0 => u / k,
                    _ => return Err(err(&what, "give u_reference, or expanded and k")),
                };
                (label, RecoveryTest::Reference { mean: m, sd: s, n, reference: *reference, u_reference: u }, n)
            }
            RecoveryTestSpec::Spiked {
                label,
                after,
                mean_after,
                sd_after,
                n_after,
                before,
                mean_before,
                sd_before,
                n_before,
                spike,
                u_spike,
                u_spike_percent,
            } => {
                let what = format!("{name}, test {}", i + 1);
                let (ma, sa, na) = stats(&what, after, *mean_after, *sd_after, *n_after)?;
                let (mb, sb, nb) = stats(&what, before, *mean_before, *sd_before, *n_before)?;
                let (u, rel) = one_of(&what, *u_spike, *u_spike_percent, "u_spike")?;
                let u = if rel { u * spike } else { u };
                (
                    label,
                    RecoveryTest::Spiked {
                        mean_after: ma,
                        sd_after: sa,
                        n_after: na,
                        mean_before: mb,
                        sd_before: sb,
                        n_before: nb,
                        spike: *spike,
                        u_spike: u,
                    },
                    na,
                )
            }
            RecoveryTestSpec::Recoveries { label, values_percent, mean_percent, sd_percent, n } => {
                let what = format!("{name}, test {}", i + 1);
                let (m, s, n) = stats(&what, values_percent, *mean_percent, *sd_percent, *n)?;
                (
                    label,
                    RecoveryTest::Reference { mean: m / 100.0, sd: s / 100.0, n, reference: 1.0, u_reference: 0.0 },
                    n,
                )
            }
        };
        dof_sum += nu.saturating_sub(1) as f64;
        labels.push(label.clone().unwrap_or_else(|| format!("{}", i + 1)));
        tests.push(test);
    }
    let mr = td::mean_recovery(&tests)?;
    let sd_r = if tests.len() >= 2 { Some(td::recovery_standard_deviation(&tests)?) } else { None };
    let u_used = match (r.include_sd_of_recoveries, sd_r) {
        (true, Some(s)) => td::root_sum_of_squares(&[mr.standard_uncertainty, s]),
        (true, None) => return Err(err(name, "s(R̄) needs at least two recovery tests")),
        (false, _) => mr.standard_uncertainty,
    };
    let dof =
        r.dof.unwrap_or(if r.include_sd_of_recoveries || dof_sum == 0.0 { tests.len() as f64 - 1.0 } else { dof_sum });
    let sig = td::recovery_significance(&td::MeanRecovery { standard_uncertainty: u_used, ..mr }, dof, r.confidence);
    let corrected = match r.correction {
        CorrectionRule::IfSignificant => sig.significant,
        CorrectionRule::Always => true,
        CorrectionRule::Never => false,
    };
    let homogeneity = if tests.len() >= 2 {
        let v: Vec<(f64, f64)> = tests.iter().map(|t| (t.recovery(), t.standard_uncertainty())).collect();
        if v.iter().all(|&(_, u)| u > 0.0) {
            Some(td::recovery_homogeneity(&v, r.confidence)?)
        } else {
            None
        }
    } else {
        None
    };
    Ok(RecoveryResult {
        name: r.name.clone(),
        entries: labels
            .into_iter()
            .zip(&tests)
            .map(|(label, t)| RecoveryEntry { label, recovery: t.recovery(), u: t.standard_uncertainty() })
            .collect(),
        mean: mr.mean,
        u_mean: mr.standard_uncertainty,
        sd_of_recoveries: sd_r,
        u_used,
        dof,
        confidence: r.confidence,
        statistic: sig.statistic,
        critical: sig.critical,
        significant: sig.significant,
        corrected,
        u_relative: if corrected { u_used / mr.mean } else { u_used },
        homogeneity,
        range: r.range.unwrap_or_default(),
    })
}

fn component(c: &ComponentSpec) -> Result<BudgetItem, ProjectError> {
    let div = match c.distribution {
        LimitDistribution::Rectangular => 3f64.sqrt(),
        LimitDistribution::Triangular => 6f64.sqrt(),
    };
    let given: Vec<(f64, bool)> = [
        c.u.map(|u| (u, false)),
        c.u_percent.map(|u| (u / 100.0, true)),
        c.half_width.map(|a| (a / div, false)),
        c.half_width_percent.map(|a| (a / 100.0 / div, true)),
    ]
    .into_iter()
    .flatten()
    .collect();
    let [(u, relative)] = given.as_slice() else {
        return Err(err(&c.name, "give exactly one of u, u_percent, half_width, half_width_percent"));
    };
    if c.multiplicity == 0 {
        return Err(err(&c.name, "multiplicity must be at least 1"));
    }
    Ok(BudgetItem {
        name: c.name.clone(),
        source: ComponentSource::Additional,
        u: *u,
        relative: *relative,
        multiplicity: c.multiplicity,
        range: c.range.unwrap_or_default(),
        model: None,
    })
}

/// Evaluates a top-down project.
pub fn evaluate(spec: &TopDownSpec) -> Result<TopDownAnalysis, ProjectError> {
    let precision: Vec<PrecisionResult> = spec.precision.iter().map(precision).collect::<Result<_, _>>()?;
    let bias: Vec<BiasResult> = spec.bias.iter().map(bias).collect::<Result<_, _>>()?;
    let recovery = spec.recovery.as_ref().map(recovery).transpose()?;
    let mut budget: Vec<BudgetItem> = precision
        .iter()
        .map(|p| BudgetItem {
            name: p.name.clone(),
            source: ComponentSource::Precision,
            u: p.sd,
            relative: p.relative,
            multiplicity: 1,
            range: p.range,
            model: p.model,
        })
        .collect();
    budget.extend(bias.iter().map(|b| BudgetItem {
        name: b.name.clone(),
        source: ComponentSource::Bias,
        u: b.u,
        relative: b.relative,
        multiplicity: 1,
        range: b.range,
        model: None,
    }));
    if let Some(r) = &recovery {
        budget.push(BudgetItem {
            name: r.name.clone(),
            source: ComponentSource::Recovery,
            u: r.u_relative,
            relative: true,
            multiplicity: 1,
            range: r.range,
            model: None,
        });
    }
    for c in &spec.components {
        budget.push(component(c)?);
    }
    let mut names = std::collections::HashSet::new();
    for b in &budget {
        if !names.insert(b.name.as_str()) {
            return Err(err(&b.name, "component names must be unique"));
        }
    }
    if budget.is_empty() {
        return Err(err(&spec.quantity, "the budget has no components"));
    }
    let k = match (spec.coverage_factor, spec.coverage_probability) {
        (Some(k), None) if k > 0.0 => k,
        (None, Some(p)) if p > 0.0 && p < 1.0 => coverage_factor(p, spec.coverage_dof.unwrap_or(f64::INFINITY)),
        (None, None) => 2.0,
        _ => return Err(err(&spec.quantity, "give either coverage_factor > 0 or a coverage_probability in (0, 1)")),
    };
    let correction = recovery.as_ref().filter(|r| r.corrected).map(|r| r.mean);
    let mut levels = Vec::new();
    let level_list: Vec<Option<f64>> =
        if spec.levels.is_empty() { vec![None] } else { spec.levels.iter().copied().map(Some).collect() };
    for level in level_list {
        let selected: Vec<usize> =
            (0..budget.len()).filter(|&i| level.map_or(true, |x| budget[i].range.contains(x))).collect();
        if selected.is_empty() {
            return Err(err(&spec.quantity, format!("no component applies at level {}", level.unwrap_or(f64::NAN))));
        }
        if level.is_none() && selected.iter().any(|&i| !budget[i].relative) {
            return Err(err(&spec.quantity, "absolute components need `levels` at which to evaluate the budget"));
        }
        // A recovery-corrected result y = x/R̄ carries absolute components
        // divided by R̄.
        let r = correction.unwrap_or(1.0);
        let comps: Vec<Component> = selected
            .iter()
            .map(|&i| {
                let b = &budget[i];
                let c = if let (Some(m), Some(x)) = (b.model, level) {
                    Component::absolute(&b.name, m.sd_at(x) / r)
                } else if b.relative {
                    Component::relative(&b.name, b.u)
                } else {
                    let u = if b.source == ComponentSource::Recovery { b.u } else { b.u / r };
                    Component::absolute(&b.name, u)
                };
                c.times(b.multiplicity)
            })
            .collect();
        let value = level.map(|x| x / r).unwrap_or(1.0);
        let res = TopDownBudget::new(comps, k).evaluate(value);
        let abs = level.is_some();
        levels.push(LevelResult {
            level,
            value: level.map(|_| value),
            standard_uncertainty: abs.then_some(res.combined_standard_uncertainty),
            relative_standard: res.relative_combined_uncertainty,
            expanded_uncertainty: abs.then_some(res.expanded_uncertainty),
            relative_expanded: res.relative_combined_uncertainty * k,
            shares: selected.iter().zip(&res.contributions).map(|(&i, c)| (i, c.1)).collect(),
        });
    }
    Ok(TopDownAnalysis { precision, bias, recovery, budget, coverage_factor: k, levels })
}

/// Compares a top-down analysis with expected values.
pub fn check(e: &ExpectedTopDown, a: &TopDownAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut verdict = None;
    let mut push = |what: String, computed: Option<f64>, c: Option<&Check>| {
        if let Some(c) = c {
            let computed = computed.unwrap_or(f64::NAN);
            out.push(crate::CheckOutcome {
                what,
                computed,
                expected: c.value,
                tolerance: c.tolerance,
                passed: (computed - c.value).abs() <= c.tolerance,
            });
        }
    };
    let pct = |v: f64, rel: bool| if rel { 100.0 * v } else { v };
    for (name, c) in &e.components {
        let item = a.budget.iter().find(|b| &b.name == name);
        push(format!("topdown.component[{name}]"), item.map(|b| pct(b.u, b.relative)), Some(c));
    }
    for (name, eb) in &e.bias {
        let b = a.bias.iter().find(|b| &b.name == name);
        push(format!("topdown.bias[{name}].mean"), b.map(|b| pct(b.mean_bias, b.relative)), eb.mean.as_ref());
        push(format!("topdown.bias[{name}].rms"), b.map(|b| pct(b.rms_bias, b.relative)), eb.rms.as_ref());
        push(
            format!("topdown.bias[{name}].mean_u_reference"),
            b.map(|b| pct(b.mean_u_reference, b.relative)),
            eb.mean_u_reference.as_ref(),
        );
    }
    if let Some(er) = &e.recovery {
        let r = a.recovery.as_ref();
        push("topdown.recovery.mean_percent".into(), r.map(|r| 100.0 * r.mean), er.mean_percent.as_ref());
        push(
            "topdown.recovery.standard_uncertainty_percent".into(),
            r.map(|r| 100.0 * r.u_used),
            er.standard_uncertainty_percent.as_ref(),
        );
        push(
            "topdown.recovery.sd_percent".into(),
            r.and_then(|r| r.sd_of_recoveries).map(|s| 100.0 * s),
            er.sd_percent.as_ref(),
        );
        push("topdown.recovery.statistic".into(), r.map(|r| r.statistic), er.statistic.as_ref());
        push("topdown.recovery.critical".into(), r.map(|r| r.critical), er.critical.as_ref());
        push(
            "topdown.recovery.chi_squared".into(),
            r.and_then(|r| r.homogeneity.as_ref()).map(|h| h.chi_squared),
            er.chi_squared.as_ref(),
        );
        if let Some(sig) = er.significant {
            let got = r.map(|r| r.significant);
            verdict = Some(crate::CheckOutcome {
                what: "topdown.recovery.significant".into(),
                computed: got.map(|b| b as u8 as f64).unwrap_or(f64::NAN),
                expected: sig as u8 as f64,
                tolerance: 0.0,
                passed: got == Some(sig),
            });
        }
    }
    for el in &e.levels {
        let l = a.levels.iter().find(|l| match (l.level, el.level) {
            (Some(x), Some(y)) => (x - y).abs() <= 1e-12 * x.abs().max(1.0),
            (None, None) => true,
            _ => false,
        });
        let tag = el.level.map(|x| format!("{x}")).unwrap_or_else(|| "relative".into());
        push(format!("topdown.level[{tag}].value"), l.and_then(|l| l.value), el.value.as_ref());
        push(
            format!("topdown.level[{tag}].standard_uncertainty"),
            l.and_then(|l| l.standard_uncertainty),
            el.standard_uncertainty.as_ref(),
        );
        push(
            format!("topdown.level[{tag}].expanded_uncertainty"),
            l.and_then(|l| l.expanded_uncertainty),
            el.expanded_uncertainty.as_ref(),
        );
        push(
            format!("topdown.level[{tag}].relative_standard_percent"),
            l.map(|l| 100.0 * l.relative_standard),
            el.relative_standard_percent.as_ref(),
        );
        push(
            format!("topdown.level[{tag}].relative_expanded_percent"),
            l.map(|l| 100.0 * l.relative_expanded),
            el.relative_expanded_percent.as_ref(),
        );
    }
    out.extend(verdict);
}

/// Machine-readable results (relative quantities as fractions).
pub fn results_json(a: &TopDownAnalysis) -> serde_json::Value {
    use serde_json::json;
    let num = |x: f64| if x.is_finite() { json!(x) } else { json!("inf") };
    json!({
        "precision": a.precision.iter().map(|p| json!({
            "name": p.name, "n": p.n, "mean": p.mean, "sd": p.sd, "relative": p.relative, "dof": num(p.dof),
            "nested": p.nested.map(|(r, i, big)| json!({"repeatability": r, "intermediate": i, "reproducibility": big})),
            "reproducibility": p.reproducibility.as_ref().map(|r| json!({
                "s_reproducibility": r.s_reproducibility, "s_between_labs": r.s_between_labs,
                "s_repeatability": r.s_repeatability, "s_repeatability_used": r.s_repeatability_used,
                "replicates": r.replicates, "s_adjusted": r.s_adjusted, "u_method_bias": r.u_method_bias,
                "bias_check": r.bias_check.map(|(d, s, ok)| json!({"abs_bias": d, "s_d": s, "under_control": ok})),
            })),
        })).collect::<Vec<_>>(),
        "bias": a.bias.iter().map(|b| json!({
            "name": b.name, "relative": b.relative, "u": b.u, "mean_bias": b.mean_bias, "rms_bias": b.rms_bias,
            "mean_u_reference": b.mean_u_reference, "single_reference": b.single_reference,
            "entries": b.entries.iter().map(|e| json!({"label": e.label, "bias": e.bias, "u_reference": e.u_reference})).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "recovery": a.recovery.as_ref().map(|r| json!({
            "mean": r.mean, "u_mean": r.u_mean, "sd_of_recoveries": r.sd_of_recoveries, "u_used": r.u_used,
            "dof": num(r.dof), "statistic": r.statistic, "critical": r.critical, "significant": r.significant,
            "corrected": r.corrected, "u_relative": r.u_relative,
            "chi_squared": r.homogeneity.as_ref().map(|h| h.chi_squared),
            "tests": r.entries.iter().map(|e| json!({"label": e.label, "recovery": e.recovery, "u": e.u})).collect::<Vec<_>>(),
        })),
        "budget": a.budget.iter().map(|b| json!({
            "name": b.name, "u": b.u, "relative": b.relative, "multiplicity": b.multiplicity,
            "model": b.model.map(|m| match m {
                LevelModel::Linear { intercept, slope, dilution_factor, .. } =>
                    json!({"type": "linear", "intercept": intercept, "slope": slope, "dilution_factor": dilution_factor}),
                LevelModel::Continuous { sd0, rsd1 } => json!({"type": "continuous", "sd0": sd0, "rsd1": rsd1}),
            }),
            "min": b.range.min, "max": b.range.max,
        })).collect::<Vec<_>>(),
        "coverage_factor": a.coverage_factor,
        "levels": a.levels.iter().map(|l| json!({
            "level": l.level, "value": l.value, "standard_uncertainty": l.standard_uncertainty,
            "expanded_uncertainty": l.expanded_uncertainty, "relative_standard": l.relative_standard,
            "relative_expanded": l.relative_expanded,
            "shares": l.shares.iter().map(|(i, s)| json!({"name": a.budget[*i].name, "share": s})).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(json: &str) -> TopDownSpec {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn nordtest_section_3_relative_budget() {
        let s = spec(
            r#"{
            "quantity": "c(NH4-N)",
            "precision": [ { "name": "u(Rw)", "data": { "type": "control_limits", "limit_percent": 3.34 } } ],
            "bias": [ { "kind": "proficiency_tests", "references": [
                { "reference": 81.0, "mean": 83.0, "sd_reproducibility_percent": 10.0, "labs": 31 },
                { "reference": 73.0, "mean": 75.0, "sd_reproducibility_percent": 7.0, "labs": 36 },
                { "reference": 264.0, "mean": 269.0, "sd_reproducibility_percent": 8.0, "labs": 32 },
                { "reference": 210.0, "mean": 213.0, "sd_reproducibility_percent": 10.0, "labs": 35 },
                { "reference": 110.0, "mean": 112.0, "sd_reproducibility_percent": 7.0, "labs": 36 },
                { "reference": 140.0, "mean": 144.0, "sd_reproducibility_percent": 11.0, "labs": 34 } ] } ]
        }"#,
        );
        let a = evaluate(&s).unwrap();
        assert!((a.budget[0].u - 0.0167).abs() < 1e-12);
        assert!((a.bias[0].rms_bias * 100.0 - 2.26).abs() < 0.005);
        assert!((a.bias[0].u * 100.0 - 2.73).abs() < 0.005);
        let l = &a.levels[0];
        assert!(l.level.is_none() && l.standard_uncertainty.is_none());
        assert!((l.relative_standard * 100.0 - 3.20).abs() < 0.005);
    }

    #[test]
    fn ranges_recovery_correction_and_errors() {
        let s = spec(
            r#"{
            "quantity": "w", "unit": "mg/kg", "levels": [5.0, 20.0],
            "precision": [
                { "name": "low", "data": { "type": "given", "sd": 0.5 }, "range": { "max": 7.5 } },
                { "name": "high", "data": { "type": "given", "sd_percent": 8.0 }, "range": { "min": 7.5 } } ],
            "recovery": { "tests": [ { "type": "recoveries", "mean_percent": 90.0, "sd_percent": 10.0, "n": 25 } ] }
        }"#,
        );
        let a = evaluate(&s).unwrap();
        let r = a.recovery.as_ref().unwrap();
        assert!(r.significant && r.corrected, "{r:?}");
        assert!((r.u_used - 0.02).abs() < 1e-12);
        // At 5 mg/kg: y = 5/0.9, u = √((0.5/0.9)² + (y · 0.02/0.9)²).
        let y: f64 = 5.0 / 0.9;
        let u = ((0.5f64 / 0.9).powi(2) + (y * 0.02 / 0.9).powi(2)).sqrt();
        assert!((a.levels[0].value.unwrap() - y).abs() < 1e-12);
        assert!((a.levels[0].standard_uncertainty.unwrap() - u).abs() < 1e-12);
        assert_eq!(a.levels[1].shares.len(), 2);
        assert_eq!(a.budget[a.levels[1].shares[0].0].name, "high");

        let bad =
            spec(r#"{ "quantity": "x", "precision": [ { "name": "p", "data": { "type": "given", "sd": 1 } } ] }"#);
        assert!(evaluate(&bad).is_err(), "absolute component without levels");
        let dup = spec(
            r#"{ "quantity": "x", "precision": [
                { "name": "p", "data": { "type": "given", "sd_percent": 1 } },
                { "name": "p", "data": { "type": "given", "sd_percent": 2 } } ] }"#,
        );
        assert!(evaluate(&dup).is_err(), "duplicate names");
        assert!(serde_json::from_str::<TopDownSpec>(r#"{ "quantity": "x", "colour": 1 }"#).is_err());
    }

    #[test]
    fn level_dependent_precision_models() {
        let s = spec(
            r#"{
            "quantity": "c", "levels": [10.0, 40.0],
            "precision": [
                { "name": "lin", "data": { "type": "linear", "points": [[1, 1.1], [2, 1.2], [3, 1.3]], "dilution_factor": 10 },
                  "range": { "max": 20 } },
                { "name": "cont", "data": { "type": "continuous", "sd0": 3, "rsd1_percent": 10 }, "range": { "min": 20 } } ]
        }"#,
        );
        let a = evaluate(&s).unwrap();
        // s(c) = 10·(1 + 0.1·c/10) = 10 + 0.1 c → 11 at c = 10; √(3² + 4²) = 5 at 40.
        assert!((a.levels[0].standard_uncertainty.unwrap() - 11.0).abs() < 1e-12);
        assert!((a.levels[1].standard_uncertainty.unwrap() - 5.0).abs() < 1e-12);
        let rel = spec(
            r#"{ "quantity": "c", "precision": [ { "name": "m", "data": { "type": "continuous", "sd0": 1, "rsd1_percent": 1 } } ] }"#,
        );
        assert!(evaluate(&rel).is_err(), "a level-dependent model needs levels");
    }

    #[test]
    fn duplicates_selection_and_conversions() {
        let s = spec(
            r#"{
            "quantity": "x", "levels": [10.0],
            "precision": [
                { "name": "r", "data": { "type": "duplicates", "pairs": [[9, 11], [10, 10], [100, 104]], "max_mean": 50 },
                  "to_relative_at": "mean" },
                { "name": "m", "data": { "type": "series", "values": [1, 2, 3], "relative": true }, "to_absolute_at": 2,
                  "mean_of": { "replicates": 1, "days": 4 } } ]
        }"#,
        );
        let a = evaluate(&s).unwrap();
        // Two pairs below 50: s = √(4/4) = 1, mean 10 → 10 %.
        assert_eq!(a.precision[0].n, 2);
        assert!((a.precision[0].sd - 0.1).abs() < 1e-12 && a.precision[0].relative);
        // s/mean = 0.5, of the mean of 4 days 0.25, at 2 → 0.5 absolute.
        assert!((a.precision[1].sd - 0.5).abs() < 1e-12 && !a.precision[1].relative);
    }

    #[test]
    fn reproducibility_iso_21748() {
        // s_L² = 0.28² − 0.22² = 0.03; u(δ̂)² = (0.0784 − 0.5·0.0484)/10 + 0.05² = 0.00792 (Formula 15);
        // s_D² = 0.03 + 0.2²/4 = 0.04, |Δ| = 0.3 < 2s_D = 0.4 (Formulae 4–5).
        let s = spec(
            r#"{ "quantity": "y", "unit": "g", "levels": [1.0],
                "precision": [ { "name": "R", "data": { "type": "reproducibility", "sd_reproducibility": 0.28,
                    "sd_repeatability": 0.22, "replicates": 2,
                    "method_bias": { "labs": 10, "replicates": 2, "u_reference": 0.05 },
                    "bias_check": { "bias": -0.3, "sd_within": 0.2, "n": 4 } } } ] }"#,
        );
        let a = evaluate(&s).unwrap();
        let r = a.precision[0].reproducibility.as_ref().unwrap();
        assert!((r.s_between_labs - 0.03f64.sqrt()).abs() < 1e-12);
        assert!((r.s_adjusted - (0.03 + 0.0484 / 2.0f64).sqrt()).abs() < 1e-12);
        assert!((r.u_method_bias.unwrap() - 0.00792f64.sqrt()).abs() < 1e-12);
        let (d, sd, ok) = r.bias_check.unwrap();
        assert!((d - 0.3).abs() < 1e-12 && (sd - 0.2).abs() < 1e-12 && ok);
        assert!((a.precision[0].sd - (0.03 + 0.0242 + 0.00792f64).sqrt()).abs() < 1e-12);
        // Mixed kinds and s_R < s_r are refused.
        let bad = spec(
            r#"{ "quantity": "y", "precision": [ { "name": "R", "data": { "type": "reproducibility",
            "sd_reproducibility_percent": 2.0, "sd_repeatability": 0.1 } } ] }"#,
        );
        assert!(evaluate(&bad).is_err());
        let bad = spec(
            r#"{ "quantity": "y", "precision": [ { "name": "R", "data": { "type": "reproducibility",
            "sd_reproducibility_percent": 1.0, "sd_repeatability_percent": 2.0 } } ] }"#,
        );
        assert!(evaluate(&bad).is_err());
    }
}
