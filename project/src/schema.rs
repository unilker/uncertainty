//! Project file format (JSON), version 1.
//!
//! A project file fully describes one uncertainty evaluation — either a
//! measurement model with the information about each input quantity
//! (`model`), or a top-down evaluation from validation data (`topdown`, see
//! [`crate::topdown`]) — together with the analysis options and,
//! optionally, the results expected from a published reference — so that every
//! example in the library checks itself.
//!
//! ```json
//! {
//!   "format": "uncertainty-calculations/project",
//!   "version": 1,
//!   "metadata": { "title": { "en": "…", "tr": "…" } },
//!   "model": {
//!     "output": { "name": "c", "unit": "mg/L", "expression": "1000 * m * P / V" },
//!     "inputs": [
//!       { "name": "m", "unit": "mg", "distribution": { "type": "normal", "mean": 100.28, "sd": 0.05 } }
//!     ]
//!   }
//! }
//! ```

use crate::calibration::{CalibrationSpec, ExpectedCalibration};
use crate::proficiency::{ExpectedProficiency, ProficiencySpec};
use crate::topdown::{ExpectedTopDown, TopDownSpec};
use serde::{Deserialize, Serialize};

pub const FORMAT_ID: &str = "uncertainty-calculations/project";
pub const FORMAT_VERSION: u32 = 1;

/// Text in English with an optional Turkish translation. In a file it is
/// written `{"en": "…", "tr": "…"}`, or as a plain string (English only).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(from = "TextRepr")]
pub struct Text {
    pub en: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tr: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TextRepr {
    Plain(String),
    Full(TextFull),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TextFull {
    en: String,
    #[serde(default)]
    tr: Option<String>,
}

impl From<TextRepr> for Text {
    fn from(t: TextRepr) -> Self {
        match t {
            TextRepr::Plain(en) => Text { en, tr: None },
            TextRepr::Full(TextFull { en, tr }) => Text { en, tr },
        }
    }
}

impl Text {
    pub fn get(&self, lang: crate::Lang) -> &str {
        match lang {
            crate::Lang::Tr => self.tr.as_deref().unwrap_or(&self.en),
            crate::Lang::En => &self.en,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub format: String,
    pub version: u32,
    pub metadata: Metadata,
    /// Measurement model (bottom-up GUM and Monte Carlo evaluation).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelSpec>,
    /// Top-down evaluation from validation and quality-control data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topdown: Option<TopDownSpec>,
    /// One round of a proficiency testing scheme (ISO 13528).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proficiency: Option<ProficiencySpec>,
    /// Calibration with uncertainties in both variables (ISO 6143).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calibration: Option<CalibrationSpec>,
    #[serde(default)]
    pub analysis: AnalysisSpec,
    #[serde(default)]
    pub reporting: ReportingSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<Expected>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    /// Step-by-step explanation of the evaluation (for the guide).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explanation: Vec<Text>,
    /// Field of metrology, e.g. "chemistry", "mass", "length".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<Source>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Notes on deviations from the published example (e.g. findings F1–F20).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<Text>,
}

/// A bibliographic source of the example or method.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub document: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSpec {
    pub output: OutputSpec,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intermediates: Vec<NamedExpression>,
    pub inputs: Vec<InputSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub correlations: Vec<CorrelationSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputSpec {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub expression: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedExpression {
    pub name: String,
    pub expression: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputSpec {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    pub distribution: DistributionSpec,
    /// Degrees of freedom for the GUM framework (Type B reliability, GUM G.4.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dof: Option<f64>,
}

/// Information about an input quantity. Symmetric distributions accept either
/// `lower`/`upper` or `value`/`half_width`; normal and t distributions accept
/// either a standard uncertainty or an expanded uncertainty with its coverage
/// factor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum DistributionSpec {
    Constant {
        value: f64,
    },
    Normal {
        mean: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sd: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        expanded: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        k: Option<f64>,
    },
    /// Scaled and shifted t-distribution (JCGM 101 6.4.9).
    T {
        mean: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scale: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        expanded: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        k: Option<f64>,
        dof: f64,
    },
    Rectangular {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lower: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        upper: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        half_width: Option<f64>,
    },
    Triangular {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lower: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        upper: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        half_width: Option<f64>,
    },
    Arcsine {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lower: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        upper: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        half_width: Option<f64>,
    },
    Trapezoidal {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lower: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        upper: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        half_width: Option<f64>,
        beta: f64,
    },
    CurvilinearTrapezoid {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lower: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        upper: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        half_width: Option<f64>,
        d: f64,
    },
    Exponential {
        mean: f64,
    },
    Gamma {
        shape: f64,
        scale: f64,
    },
    /// Type A evaluation of a series of indications (GUM 4.2): GUM uses
    /// x̄, s/√n, ν = n − 1; MCM samples t_{n−1}(x̄, s²/n) (JCGM 101 6.4.9).
    TypeA {
        observations: Vec<f64>,
    },
    /// Mean of n indications with a pooled standard deviation (GUM 4.2.4).
    TypeAPooled {
        mean: f64,
        pooled_sd: f64,
        pooled_dof: f64,
        n: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationSpec {
    pub a: String,
    pub b: String,
    pub r: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisSpec {
    #[serde(default = "some_default_gum", skip_serializing_if = "Option::is_none")]
    pub gum: Option<GumSpec>,
    #[serde(default = "some_default_mcm", skip_serializing_if = "Option::is_none")]
    pub mcm: Option<McmSpec>,
}

fn some_default_gum() -> Option<GumSpec> {
    Some(GumSpec::default())
}

fn some_default_mcm() -> Option<McmSpec> {
    Some(McmSpec::default())
}

impl Default for AnalysisSpec {
    fn default() -> Self {
        Self { gum: some_default_gum(), mcm: some_default_mcm() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GumSpec {
    /// Coverage probability for k from the t-distribution (default 0.95).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_probability: Option<f64>,
    /// Fixed coverage factor (e.g. 2); overrides `coverage_probability`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_factor: Option<f64>,
    #[serde(default)]
    pub higher_order: bool,
    #[serde(default)]
    pub dof_rounding: DofRoundingSpec,
    /// Also evaluate Kragten's spreadsheet approximation (QUAM Appendix E.2),
    /// for comparison with spreadsheet evaluations.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub kragten: bool,
}

impl Default for GumSpec {
    fn default() -> Self {
        Self {
            coverage_probability: None,
            coverage_factor: None,
            higher_order: false,
            dof_rounding: DofRoundingSpec::Truncate,
            kragten: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DofRoundingSpec {
    #[default]
    Truncate,
    Exact,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McmSpec {
    /// Fixed number of trials (default 10⁶) unless `adaptive` is given.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trials: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adaptive: Option<AdaptiveSpec>,
    /// Defaults to the GUM coverage probability (or 0.95).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_probability: Option<f64>,
    #[serde(default = "default_seed")]
    pub seed: u64,
    #[serde(default)]
    pub interval: IntervalSpec,
    /// Significant digits of u(y) for validating the GUM result (JCGM 101 8.1).
    #[serde(default = "default_ndig")]
    pub validation_ndig: u32,
}

fn default_seed() -> u64 {
    1
}

fn default_ndig() -> u32 {
    2
}

impl Default for McmSpec {
    fn default() -> Self {
        Self {
            trials: None,
            adaptive: None,
            coverage_probability: None,
            seed: default_seed(),
            interval: IntervalSpec::default(),
            validation_ndig: default_ndig(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveSpec {
    pub ndig: u32,
    #[serde(default = "default_divisor")]
    pub tolerance_divisor: f64,
    #[serde(default = "default_max_trials")]
    pub max_trials: usize,
}

fn default_divisor() -> f64 {
    1.0
}

fn default_max_trials() -> usize {
    10_000_000
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum IntervalSpec {
    #[default]
    Shortest,
    Symmetric,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportingSpec {
    #[serde(default = "default_sig")]
    pub significant_digits: u32,
    #[serde(default)]
    pub rounding: RoundingSpec,
    /// Round the expanded uncertainty to this number of decimal places
    /// instead of to significant digits (e.g. 0 for whole per cent,
    /// Nordtest TR 537).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
}

fn default_sig() -> u32 {
    2
}

impl Default for ReportingSpec {
    fn default() -> Self {
        Self { significant_digits: default_sig(), rounding: RoundingSpec::default(), decimal_places: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RoundingSpec {
    /// MUPR §12 rule (discarded 5 to even, later digits ignored).
    #[default]
    HalfEven,
    HalfUp,
    /// Round the uncertainty up (GUM 7.2.6).
    Up,
}

/// A value with its tolerance, for self-checking examples.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Check {
    pub value: f64,
    pub tolerance: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Expected {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gum: Option<ExpectedGum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcm: Option<ExpectedMcm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topdown: Option<ExpectedTopDown>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proficiency: Option<ExpectedProficiency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calibration: Option<ExpectedCalibration>,
    /// Where the expected values come from (document and table).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<Text>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedGum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimate: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_uncertainty: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_uncertainty: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_factor: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_dof: Option<Check>,
    /// Combined standard uncertainty by Kragten's spreadsheet method.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kragten_uncertainty: Option<Check>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedMcm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimate: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_uncertainty: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_low: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_high: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gum_validated: Option<bool>,
}
