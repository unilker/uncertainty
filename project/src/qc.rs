//! Control charts for internal quality control after Nordtest TR 569 ed. 6
//! (2026): the control values of a series of runs, the central line and
//! control limits, the daily interpretation, the long-term review and the
//! precision obtained from replicate control results.

use crate::schema::Check;
use crate::ProjectError;
use serde::{Deserialize, Serialize};
use uncertainty_core::qc::{
    self, ChartKind, ControlLimits, RangeFactors, Review, RunAnova, StatisticalSignals, Status, Summary,
};

fn err(what: &str, message: impl Into<String>) -> ProjectError {
    ProjectError::Input { name: format!("control_chart.{what}"), message: message.into() }
}

// ---------------------------------------------------------------------------
// Specification
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlChartSpec {
    /// The control sample and analyte, e.g. "Zn, standard solution 60 µg/L".
    pub quantity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "is_x")]
    pub chart: ChartSpec,
    /// Control values in time order: single values or means (X), ranges (R)
    /// or relative ranges in per cent (r%).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<f64>,
    /// Alternatively, the replicate results of each run; the control value is
    /// their mean, range or relative range.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runs: Vec<Vec<f64>>,
    /// Labels of the runs (dates), for the report.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    /// Number of replicates of a range: needed for R- and r%-charts given as
    /// `values`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicates: Option<usize>,
    /// A reference or nominal central line, or a mean established earlier;
    /// by default the mean of the control values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub central_line: Option<CentralLineSpec>,
    /// Reference value of the control sample, for the t-test of the mean
    /// (Equation 6) when the central line is the mean.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<f64>,
    #[serde(default)]
    pub limits: LimitsSpec,
    /// The first N control values set the central line and statistical
    /// limits (preliminary limits); all values by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub establish: Option<usize>,
    /// Runs (1-based) left out of the statistics, e.g. out-of-control values
    /// with an assignable cause (Chapter 10).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<usize>,
    /// Long-term review of the control values after the establishing period
    /// (Chapter 10).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<ReviewSpec>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChartSpec {
    #[default]
    X,
    Range,
    RelativeRange,
}

fn is_x(c: &ChartSpec) -> bool {
    *c == ChartSpec::X
}

impl ChartSpec {
    pub fn kind(self) -> ChartKind {
        match self {
            ChartSpec::X => ChartKind::X,
            ChartSpec::Range => ChartKind::Range,
            ChartSpec::RelativeRange => ChartKind::RelativeRange,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum CentralLineSpec {
    /// Reference value of a reference material or nominal value.
    Reference(f64),
    /// Mean value established earlier (summary statistics).
    Mean(f64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BasisSpec {
    /// From the performance of the method (Chapter 7).
    #[default]
    Statistical,
    /// From the requirement on s_Rw or s_r.
    Target,
    /// Action limits at ± a tolerance (charts with action limits only).
    Tolerance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SdFromSpec {
    /// s = mean range/d₂ (Equation 4, Table 4).
    #[default]
    MeanRange,
    /// Pooled within-run standard deviation of the replicates (Equation 9).
    Pooled,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct LimitsSpec {
    #[serde(default)]
    pub basis: BasisSpec,
    /// Statistical: the standard deviation established earlier (instead of
    /// the one computed from the values). Target: s_target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd: Option<f64>,
    /// Target: s_target in per cent of the central line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_percent: Option<f64>,
    /// Target: s_target = U/4 from a required relative expanded uncertainty
    /// U (k = 2) in per cent (Example 1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_percent: Option<f64>,
    /// Target for range charts: s_r = r/2.8 from a repeatability limit r.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeatability_limit: Option<f64>,
    /// Tolerance: half-width of the action limits (X) or upper action limit
    /// (R, r%), absolute or in per cent of the central line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance_percent: Option<f64>,
    /// Statistical range charts: how s is obtained from the replicates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_from: Option<SdFromSpec>,
    /// A chart with action limits only, at `action_factor`·s (default 3 for
    /// X-charts and D_AL for range charts; 2.5 and 3.5 give about the false
    /// alarm rates of charts with both limits, footnotes 9 and 11).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub action_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_factor: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ReviewSpec {
    /// The statistics the limits were based on; by default those of the
    /// establishing period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<SummarySpec>,
    /// The statistics of the period reviewed; by default those of the
    /// control values after the establishing period, without values more
    /// than `exclude_beyond` standard deviations from the central line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<SummarySpec>,
    /// Default 4 (Chapter 10).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_beyond: Option<f64>,
    /// Confidence level of the F- and t-tests, default 0.95.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummarySpec {
    /// Not needed for range charts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<f64>,
    pub sd: f64,
    pub n: usize,
    /// Degrees of freedom of sd; n − 1 by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dof: Option<f64>,
}

// ---------------------------------------------------------------------------
// Expected values
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedControlChart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_warning: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_warning: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_action: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_action: Option<Check>,
    /// Numbers of control values: outside the warning limits; out of control
    /// (action limit or two out of three); out of statistical control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outside_warning: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_of_control: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistical_signals: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<ExpectedReview>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<ExpectedPrecision>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedReview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f_critical: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pooled_sd: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t_critical: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sd_significant: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_significant: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ExpectedPrecision {
    /// Equal-weight pooling of the within-run standard deviations (Eq. 10).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_r_pooled: Option<Check>,
    /// Pooling of the standard deviations of the k-th results of the runs
    /// (the simplified s_Rw of Example 12).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_rw_simplified: Option<Check>,
    /// One-way analysis of variance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_r: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_between: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s_rw: Option<Check>,
}

// ---------------------------------------------------------------------------
// Results
// ---------------------------------------------------------------------------

/// Where the central line comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterSource {
    /// Mean of the control values of the establishing period.
    Computed,
    /// Mean established earlier.
    GivenMean,
    /// Reference or nominal value.
    Reference,
    /// Expected range d₂·s of a range chart without data.
    ExpectedRange,
}

/// Where the standard deviation used for the limits comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdSource {
    /// Standard deviation of the control values (X).
    Computed,
    /// Mean range/d₂ (R, r%).
    MeanRange,
    /// Pooled within-run standard deviation (R, r%).
    Pooled,
    /// Established earlier.
    Given,
    /// Target from the requirement.
    Target,
    /// Not used: limits at the tolerance.
    None,
}

#[derive(Clone, Debug)]
pub struct ReviewResult {
    pub previous: Summary,
    pub current: Summary,
    /// Whether the summaries were computed from the control values.
    pub previous_computed: bool,
    pub current_computed: bool,
    /// Run indices (0-based) of the review period excluded as more than k s
    /// from the central line.
    pub excluded: Vec<usize>,
    pub exclude_beyond: f64,
    /// Control values of the review period outside the warning limits.
    pub outside_warning: Option<usize>,
    pub period: Option<usize>,
    pub confidence: f64,
    /// F- and t-tests; the t-test only when both means are known.
    pub review: Review,
    pub with_means: bool,
}

#[derive(Clone, Debug)]
pub struct PrecisionResult {
    pub anova: RunAnova,
    /// Equation 9 and Equation 10 pooling of the within-run standard
    /// deviations.
    pub s_r_pooled: f64,
    pub s_r_equal: f64,
    /// Standard deviations of the first, second, … results of the runs and
    /// their pooled value (Example 12, simplified s_Rw); balanced runs only.
    pub by_position: Vec<f64>,
    pub s_rw_simplified: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct ControlChartAnalysis {
    pub quantity: String,
    pub unit: Option<String>,
    pub kind: ChartKind,
    pub replicates: Option<usize>,
    pub factors: Option<RangeFactors>,
    pub values: Vec<f64>,
    pub labels: Vec<String>,
    /// Number of control values that establish the limits.
    pub establish: usize,
    pub excluded: Vec<usize>,
    /// Statistics of the establishing period (without exclusions).
    pub stats: Option<Summary>,
    pub center: f64,
    pub center_source: CenterSource,
    pub sd: Option<f64>,
    pub sd_source: SdSource,
    pub basis: BasisSpec,
    /// Factor of s for the action limits of a chart with action limits only.
    pub action_factor: Option<f64>,
    pub limits: ControlLimits,
    pub daily: Vec<Status>,
    pub signals: Vec<StatisticalSignals>,
    /// Mean against the reference value (Equation 6): T, t and critical t.
    pub bias_test: Option<(f64, f64, f64)>,
    pub review: Option<ReviewResult>,
    pub precision: Option<PrecisionResult>,
}

impl ControlChartAnalysis {
    pub fn outside_warning(&self) -> usize {
        self.values.iter().filter(|&&x| self.limits.zone(x) != 0).count()
    }
    pub fn out_of_control(&self) -> usize {
        self.daily.iter().filter(|s| s.out_of_control()).count()
    }
    pub fn statistical_signals(&self) -> usize {
        self.signals.iter().filter(|s| s.trend || s.run).count()
    }
}

fn positive(x: Option<f64>, what: &str) -> Result<Option<f64>, ProjectError> {
    match x {
        Some(v) if !(v.is_finite() && v > 0.0) => Err(err(what, "must be positive")),
        _ => Ok(x),
    }
}

pub fn evaluate(spec: &ControlChartSpec) -> Result<ControlChartAnalysis, ProjectError> {
    let kind = spec.chart.kind();
    let qe = |e: uncertainty_core::UncError| err("values", e.to_string());
    if !spec.values.is_empty() && !spec.runs.is_empty() {
        return Err(err("values", "give either values or runs"));
    }
    let values: Vec<f64> = if spec.runs.is_empty() {
        if spec.values.iter().any(|v| !v.is_finite()) {
            return Err(err("values", "values must be finite"));
        }
        spec.values.clone()
    } else {
        spec.runs.iter().map(|r| qc::control_value(kind, r)).collect::<Result<_, _>>().map_err(qe)?
    };
    if !spec.labels.is_empty() && spec.labels.len() != values.len() {
        return Err(err("labels", format!("{} labels for {} control values", spec.labels.len(), values.len())));
    }
    if kind != ChartKind::X && values.iter().any(|&v| v < 0.0) {
        return Err(err("values", "ranges cannot be negative"));
    }

    // Replicates of a range chart.
    let run_sizes: Vec<usize> = spec.runs.iter().map(Vec::len).collect();
    let replicates = match (spec.replicates, run_sizes.first()) {
        (Some(n), _) if !spec.runs.is_empty() && run_sizes.iter().any(|&m| m != n) => {
            return Err(err("replicates", "differs from the number of results in the runs"));
        }
        (Some(n), _) => Some(n),
        (None, Some(&n)) if run_sizes.iter().all(|&m| m == n) => Some(n),
        (None, Some(_)) if kind != ChartKind::X => {
            return Err(err("runs", "a range chart needs the same number of replicates in every run"))
        }
        _ => None,
    };
    let factors = if kind == ChartKind::X {
        None
    } else {
        let n = replicates.ok_or_else(|| err("replicates", "the number of replicates of a range is needed"))?;
        Some(qc::range_factors(n).map_err(|e| err("replicates", e.to_string()))?)
    };

    // Establishing period.
    let establish = spec.establish.unwrap_or(values.len());
    if establish > values.len() {
        return Err(err("establish", format!("{establish} exceeds the {} control values", values.len())));
    }
    for &i in &spec.exclude {
        if i == 0 || i > values.len() {
            return Err(err("exclude", format!("run {i} does not exist")));
        }
    }
    let excluded: Vec<usize> = spec.exclude.iter().map(|i| i - 1).collect();
    let kept: Vec<usize> = (0..establish).filter(|i| !excluded.contains(i)).collect();
    let est_values: Vec<f64> = kept.iter().map(|&i| values[i]).collect();
    let stats = if est_values.len() >= 2 { Some(qc::summary(&est_values).map_err(qe)?) } else { None };

    // Central line.
    let given_center = match spec.central_line {
        Some(CentralLineSpec::Reference(v)) => Some((v, CenterSource::Reference)),
        Some(CentralLineSpec::Mean(v)) => Some((v, CenterSource::GivenMean)),
        None => None,
    };
    if given_center.is_some_and(|(v, _)| !v.is_finite()) {
        return Err(err("central_line", "must be finite"));
    }

    // Standard deviation for the limits.
    let l = &spec.limits;
    for (v, w) in [
        (l.sd, "limits.sd"),
        (l.sd_percent, "limits.sd_percent"),
        (l.expanded_percent, "limits.expanded_percent"),
        (l.repeatability_limit, "limits.repeatability_limit"),
        (l.tolerance, "limits.tolerance"),
        (l.tolerance_percent, "limits.tolerance_percent"),
        (l.action_factor, "limits.action_factor"),
    ] {
        positive(v, w)?;
    }
    let provisional_center = given_center.map(|c| c.0).or(stats.map(|s| s.mean));
    let relative = |pct: f64, what: &str| -> Result<f64, ProjectError> {
        let c = provisional_center.ok_or_else(|| err(what, "a relative value needs a central line"))?;
        Ok(pct / 100.0 * c.abs())
    };
    let target_options =
        [l.sd.is_some(), l.sd_percent.is_some(), l.expanded_percent.is_some(), l.repeatability_limit.is_some()];
    let tolerance_options = [l.tolerance.is_some(), l.tolerance_percent.is_some()];
    let (sd, sd_source) = match l.basis {
        BasisSpec::Statistical => {
            if target_options[2..].iter().any(|&b| b)
                || tolerance_options.iter().any(|&b| b)
                || (l.sd.is_some() && l.sd_percent.is_some())
            {
                return Err(err(
                    "limits",
                    "statistical limits take only sd or sd_percent (established earlier) and sd_from",
                ));
            }
            if let Some(s) = l.sd {
                (Some(s), SdSource::Given)
            } else if let Some(p) = l.sd_percent {
                (Some(relative(p, "limits.sd_percent")?), SdSource::Given)
            } else if kind == ChartKind::X {
                let s = stats
                    .ok_or_else(|| err("values", "statistical limits need at least two control values or limits.sd"))?;
                (Some(s.sd), SdSource::Computed)
            } else {
                let f = factors.expect("range factors");
                match l.sd_from.unwrap_or_default() {
                    SdFromSpec::MeanRange => {
                        let mean_range = match (stats, given_center) {
                            (Some(s), _) => s.mean,
                            (None, Some((m, CenterSource::GivenMean))) => m,
                            _ => {
                                return Err(err(
                                    "values",
                                    "statistical limits need at least two ranges, a mean range or limits.sd",
                                ))
                            }
                        };
                        (Some(mean_range / f.d2), SdSource::MeanRange)
                    }
                    SdFromSpec::Pooled => {
                        if spec.runs.is_empty() {
                            return Err(err("limits.sd_from", "pooling needs the replicate results (runs)"));
                        }
                        let groups: Vec<Summary> = kept
                            .iter()
                            .map(|&i| qc::summary(&relative_run(kind, &spec.runs[i])))
                            .collect::<Result<_, _>>()
                            .map_err(qe)?;
                        let (s, _) = qc::pooled_sd(&groups).map_err(qe)?;
                        (Some(s), SdSource::Pooled)
                    }
                }
            }
        }
        BasisSpec::Target => {
            if target_options.iter().filter(|&&b| b).count() != 1 || tolerance_options.iter().any(|&b| b) {
                return Err(err(
                    "limits",
                    "target limits take one of sd, sd_percent, expanded_percent and repeatability_limit",
                ));
            }
            let s = if let Some(s) = l.sd {
                s
            } else if let Some(p) = l.sd_percent {
                relative(p, "limits.sd_percent")?
            } else if let Some(u) = l.expanded_percent {
                relative(qc::target_sd_from_expanded(u), "limits.expanded_percent")?
            } else {
                if kind == ChartKind::X {
                    return Err(err("limits.repeatability_limit", "applies to range charts"));
                }
                qc::sd_from_repeatability_limit(l.repeatability_limit.unwrap_or_default())
            };
            (Some(s), SdSource::Target)
        }
        BasisSpec::Tolerance => {
            if tolerance_options.iter().filter(|&&b| b).count() != 1 || target_options.iter().any(|&b| b) {
                return Err(err("limits", "tolerance limits take one of tolerance and tolerance_percent"));
            }
            (None, SdSource::None)
        }
    };

    let (center, center_source) = match (given_center, stats, kind) {
        (Some(c), _, _) => c,
        (None, Some(s), _) => (s.mean, CenterSource::Computed),
        (None, None, ChartKind::X) => return Err(err("central_line", "needed when there are no control values")),
        (None, None, _) => match (sd, factors) {
            (Some(s), Some(f)) => (f.d2 * s, CenterSource::ExpectedRange),
            _ => return Err(err("central_line", "needed when there are no control values")),
        },
    };

    // Limits.
    let le = |e: uncertainty_core::UncError| err("limits", e.to_string());
    let action_only = l.action_only || l.basis == BasisSpec::Tolerance;
    if l.action_factor.is_some() && !(l.action_only && sd.is_some()) {
        return Err(err("limits.action_factor", "applies to charts with action limits only at a multiple of s"));
    }
    let (limits, action_factor) = if l.basis == BasisSpec::Tolerance {
        let t = match (l.tolerance, l.tolerance_percent) {
            (Some(t), _) => t,
            (_, Some(p)) => p / 100.0 * center.abs(),
            _ => unreachable!(),
        };
        let lim = if kind == ChartKind::X {
            ControlLimits::x_action_only(center, t)
        } else {
            ControlLimits::range_action_only(center, t)
        };
        (lim.map_err(le)?, None)
    } else {
        let s = sd.expect("standard deviation");
        if action_only {
            let k = l.action_factor.unwrap_or(match factors {
                None => 3.0,
                Some(f) => f.action,
            });
            let lim = if kind == ChartKind::X {
                ControlLimits::x_action_only(center, k * s)
            } else {
                ControlLimits::range_action_only(center, k * s)
            };
            (lim.map_err(le)?, Some(k))
        } else if kind == ChartKind::X {
            (ControlLimits::x(center, s).map_err(le)?, None)
        } else {
            (ControlLimits::range(factors.expect("range factors").n, s, center).map_err(le)?, None)
        }
    };

    let daily = qc::daily(&values, &limits);
    let signals = qc::statistical_signals(&values, center);
    if spec.reference.is_some_and(|r| !r.is_finite())
        || (spec.reference.is_some() && center_source == CenterSource::Reference)
    {
        return Err(err("reference", "a finite value, when the central line is not already the reference value"));
    }
    let reference = if center_source == CenterSource::Reference { Some(center) } else { spec.reference };
    let bias_test = match (reference, stats, kind) {
        (Some(r), Some(s), ChartKind::X) if s.sd > 0.0 => {
            let (t, tc) = qc::t_against_reference(&s, r, 0.95);
            Some((r, t, tc))
        }
        _ => None,
    };

    // Review.
    let review = match &spec.review {
        None => None,
        Some(r) => Some(review(r, kind, &values, establish, &excluded, stats, sd, factors, &limits, center)?),
    };

    // Precision from replicate control results.
    let precision = if spec.runs.len() >= 2 && spec.runs.iter().any(|r| r.len() >= 2) && kind == ChartKind::X {
        let runs: Vec<Vec<f64>> = kept.iter().map(|&i| spec.runs[i].clone()).collect();
        Some(precision(&runs).map_err(|e| err("runs", e.to_string()))?)
    } else {
        None
    };

    Ok(ControlChartAnalysis {
        quantity: spec.quantity.clone(),
        unit: spec.unit.clone(),
        kind,
        replicates,
        factors,
        values,
        labels: spec.labels.clone(),
        establish,
        excluded,
        stats,
        center,
        center_source,
        sd,
        sd_source,
        basis: l.basis,
        action_factor,
        limits,
        daily,
        signals,
        bias_test,
        review,
        precision,
    })
}

/// The replicates of a run scaled for pooling: relative to their mean in
/// per cent for an r%-chart.
fn relative_run(kind: ChartKind, run: &[f64]) -> Vec<f64> {
    if kind == ChartKind::RelativeRange {
        let m = run.iter().sum::<f64>() / run.len() as f64;
        run.iter().map(|x| 100.0 * x / m.abs()).collect()
    } else {
        run.to_vec()
    }
}

fn summary_of(s: &SummarySpec, what: &str) -> Result<Summary, ProjectError> {
    if !(s.sd.is_finite() && s.sd > 0.0) || s.n < 2 || s.mean.is_some_and(|m| !m.is_finite()) {
        return Err(err(what, "needs sd > 0, n ≥ 2 and a finite mean"));
    }
    let dof = s.dof.unwrap_or(s.n as f64 - 1.0);
    if dof.is_nan() || dof <= 0.0 {
        return Err(err(what, "dof must be positive"));
    }
    Ok(Summary { mean: s.mean.unwrap_or(f64::NAN), sd: s.sd, n: s.n, dof })
}

#[allow(clippy::too_many_arguments)]
fn review(
    r: &ReviewSpec,
    kind: ChartKind,
    values: &[f64],
    establish: usize,
    excluded: &[usize],
    stats: Option<Summary>,
    sd: Option<f64>,
    factors: Option<RangeFactors>,
    limits: &ControlLimits,
    center: f64,
) -> Result<ReviewResult, ProjectError> {
    let confidence = r.confidence.unwrap_or(0.95);
    if !(confidence > 0.0 && confidence < 1.0) {
        return Err(err("review.confidence", "must be between 0 and 1"));
    }
    let k = r.exclude_beyond.unwrap_or(4.0);
    if k.is_nan() || k <= 0.0 {
        return Err(err("review.exclude_beyond", "must be positive"));
    }
    let (previous, previous_computed) = match &r.previous {
        Some(p) => (summary_of(p, "review.previous")?, false),
        None => {
            let mut s =
                stats.ok_or_else(|| err("review.previous", "needed when no control values establish the limits"))?;
            if let Some(f) = factors {
                // The standard deviation of the ranges is not that of the
                // results: use the s of the limits with ν = N(n − 1).
                s.sd = sd.ok_or_else(|| err("review.previous", "needed for limits at a tolerance"))?;
                s.dof = (s.n * (f.n - 1)) as f64;
            }
            (s, true)
        }
    };
    let period: Vec<usize> = (establish..values.len()).filter(|i| !excluded.contains(i)).collect();
    let (current, current_computed, beyond, outside_warning) = match &r.current {
        Some(c) => (summary_of(c, "review.current")?, false, Vec::new(), None),
        None => {
            if period.len() < 2 {
                return Err(err(
                    "review.current",
                    "needed when fewer than two control values follow the establishing period",
                ));
            }
            let pv: Vec<f64> = period.iter().map(|&i| values[i]).collect();
            let ow = pv.iter().filter(|&&x| limits.zone(x) != 0).count();
            match factors {
                None => {
                    let beyond: Vec<usize> =
                        qc::beyond(&pv, center, previous.sd, k).into_iter().map(|j| period[j]).collect();
                    let kept: Vec<f64> = period.iter().filter(|i| !beyond.contains(i)).map(|&i| values[i]).collect();
                    let s = qc::summary(&kept).map_err(|e| err("review", e.to_string()))?;
                    (s, true, beyond, Some(ow))
                }
                Some(f) => {
                    // s = mean range/d₂ of the period, ν = N(n − 1).
                    let mean = pv.iter().sum::<f64>() / pv.len() as f64;
                    let s = Summary { mean, sd: mean / f.d2, n: pv.len(), dof: (pv.len() * (f.n - 1)) as f64 };
                    (s, true, Vec::new(), Some(ow))
                }
            }
        }
    };
    // The t-test compares means of control values (X-charts).
    let with_means = kind == ChartKind::X && previous.mean.is_finite() && current.mean.is_finite();
    let mut p = previous;
    let mut c = current;
    if !with_means {
        p.mean = 0.0;
        c.mean = 0.0;
    }
    let review = qc::review(&p, &c, confidence).map_err(|e| err("review", e.to_string()))?;
    Ok(ReviewResult {
        previous,
        current,
        previous_computed,
        current_computed,
        excluded: beyond,
        exclude_beyond: k,
        outside_warning,
        period: if current_computed { Some(period.len()) } else { None },
        confidence,
        review,
        with_means,
    })
}

fn precision(runs: &[Vec<f64>]) -> Result<PrecisionResult, uncertainty_core::UncError> {
    let anova = qc::run_anova(runs)?;
    let groups: Vec<Summary> =
        runs.iter().filter(|r| r.len() >= 2).map(|r| qc::summary(r)).collect::<Result<_, _>>()?;
    let (s_r_pooled, _) = qc::pooled_sd(&groups)?;
    let s_r_equal = qc::pooled_sd_equal(&groups.iter().map(|g| g.sd).collect::<Vec<_>>());
    let n = runs[0].len();
    let (by_position, s_rw_simplified) = if n >= 2 && runs.iter().all(|r| r.len() == n) {
        let sds: Vec<f64> = (0..n)
            .map(|j| qc::summary(&runs.iter().map(|r| r[j]).collect::<Vec<_>>()).map(|s| s.sd))
            .collect::<Result<_, _>>()?;
        let p = qc::pooled_sd_equal(&sds);
        (sds, Some(p))
    } else {
        (Vec::new(), None)
    };
    Ok(PrecisionResult { anova, s_r_pooled, s_r_equal, by_position, s_rw_simplified })
}

// ---------------------------------------------------------------------------
// Checks and JSON
// ---------------------------------------------------------------------------

pub fn check(e: &ExpectedControlChart, a: &ControlChartAnalysis, out: &mut Vec<crate::CheckOutcome>) {
    let mut push = |what: &str, computed: Option<f64>, c: &Option<Check>| {
        if let Some(c) = c {
            let computed = computed.unwrap_or(f64::NAN);
            out.push(crate::CheckOutcome {
                what: format!("control_chart.{what}"),
                computed,
                expected: c.value,
                tolerance: c.tolerance,
                passed: (computed - c.value).abs() <= c.tolerance,
            });
        }
    };
    let exact = |v: usize| Some(Check { value: v as f64, tolerance: 0.0 });
    let b = |x: bool| if x { 1.0 } else { 0.0 };
    push("mean", a.stats.map(|s| s.mean), &e.mean);
    push("sd", a.sd, &e.sd);
    push("center", Some(a.center), &e.center);
    push("lower_warning", a.limits.lower_warning, &e.lower_warning);
    push("upper_warning", a.limits.upper_warning, &e.upper_warning);
    push("lower_action", a.limits.lower_action, &e.lower_action);
    push("upper_action", Some(a.limits.upper_action), &e.upper_action);
    if let Some(v) = e.outside_warning {
        push("outside_warning", Some(a.outside_warning() as f64), &exact(v));
    }
    if let Some(v) = e.out_of_control {
        push("out_of_control", Some(a.out_of_control() as f64), &exact(v));
    }
    if let Some(v) = e.statistical_signals {
        push("statistical_signals", Some(a.statistical_signals() as f64), &exact(v));
    }
    if let Some(x) = &e.review {
        let r = a.review.as_ref();
        push("review.mean", r.filter(|r| r.with_means).map(|r| r.current.mean), &x.mean);
        push("review.sd", r.map(|r| r.current.sd), &x.sd);
        push("review.f", r.map(|r| r.review.f), &x.f);
        push("review.f_critical", r.map(|r| r.review.f_critical), &x.f_critical);
        push("review.pooled_sd", r.filter(|r| r.with_means).map(|r| r.review.pooled_sd), &x.pooled_sd);
        push("review.t", r.filter(|r| r.with_means).map(|r| r.review.t), &x.t);
        push("review.t_critical", r.filter(|r| r.with_means).map(|r| r.review.t_critical), &x.t_critical);
        if let Some(v) = x.sd_significant {
            push(
                "review.sd_significant",
                r.map(|r| b(r.review.sd_significant())),
                &Some(Check { value: b(v), tolerance: 0.0 }),
            );
        }
        if let Some(v) = x.mean_significant {
            push(
                "review.mean_significant",
                r.filter(|r| r.with_means).map(|r| b(r.review.mean_significant())),
                &Some(Check { value: b(v), tolerance: 0.0 }),
            );
        }
    }
    if let Some(x) = &e.precision {
        let p = a.precision.as_ref();
        push("precision.s_r_pooled", p.map(|p| p.s_r_equal), &x.s_r_pooled);
        push("precision.s_rw_simplified", p.and_then(|p| p.s_rw_simplified), &x.s_rw_simplified);
        push("precision.s_r", p.map(|p| p.anova.s_r), &x.s_r);
        push("precision.s_between", p.map(|p| p.anova.s_between), &x.s_between);
        push("precision.s_rw", p.map(|p| p.anova.s_rw), &x.s_rw);
    }
}

pub fn kind_code(k: ChartKind) -> &'static str {
    match k {
        ChartKind::X => "x",
        ChartKind::Range => "range",
        ChartKind::RelativeRange => "relative_range",
    }
}

pub fn status_code(s: Status) -> &'static str {
    match s {
        Status::InControl => "in_control",
        Status::Warning => "warning",
        Status::OutsideAction => "outside_action",
        Status::TwoOfThree => "two_of_three",
    }
}

pub fn results_json(a: &ControlChartAnalysis) -> serde_json::Value {
    use serde_json::json;
    let summary = |s: &Summary| json!({"mean": if s.mean.is_finite() { json!(s.mean) } else { json!(null) }, "sd": s.sd, "n": s.n, "dof": s.dof});
    json!({
        "chart": kind_code(a.kind),
        "replicates": a.replicates,
        "factors": a.factors.map(|f| json!({"d2": f.d2, "d3": f.d3, "D_WL": f.warning, "D_AL": f.action})),
        "values": a.values,
        "labels": a.labels,
        "establish": a.establish,
        "excluded": a.excluded.iter().map(|i| i + 1).collect::<Vec<_>>(),
        "statistics": a.stats.as_ref().map(summary),
        "center": a.center,
        "sd": a.sd,
        "limits": {
            "lower_warning": a.limits.lower_warning, "upper_warning": a.limits.upper_warning,
            "lower_action": a.limits.lower_action, "upper_action": a.limits.upper_action,
        },
        "daily": a.daily.iter().map(|&s| status_code(s)).collect::<Vec<_>>(),
        "statistical_signals": a.signals.iter().map(|s| json!({"trend": s.trend, "run": s.run})).collect::<Vec<_>>(),
        "outside_warning": a.outside_warning(),
        "out_of_control": a.out_of_control(),
        "bias_test": a.bias_test.map(|(r, t, tc)| json!({"reference": r, "t": t, "t_critical": tc})),
        "review": a.review.as_ref().map(|r| json!({
            "previous": summary(&r.previous), "current": summary(&r.current),
            "excluded": r.excluded.iter().map(|i| i + 1).collect::<Vec<_>>(),
            "outside_warning": r.outside_warning,
            "F": r.review.f, "F_dof": [r.review.f_dof.0, r.review.f_dof.1], "F_critical": r.review.f_critical,
            "sd_significant": r.review.sd_significant(),
            "pooled_sd": if r.with_means { json!(r.review.pooled_sd) } else { json!(null) },
            "t": if r.with_means { json!(r.review.t) } else { json!(null) },
            "t_critical": if r.with_means { json!(r.review.t_critical) } else { json!(null) },
            "mean_significant": if r.with_means { json!(r.review.mean_significant()) } else { json!(null) },
            "mean_shift_in_sd": if r.with_means { json!(r.review.mean_shift) } else { json!(null) },
        })),
        "precision": a.precision.as_ref().map(|p| json!({
            "s_r": p.anova.s_r, "s_between": p.anova.s_between, "s_rw": p.anova.s_rw,
            "s_r_pooled_eq9": p.s_r_pooled, "s_r_pooled_eq10": p.s_r_equal,
            "by_position": p.by_position, "s_rw_simplified": p.s_rw_simplified,
        })),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(json: &str) -> ControlChartSpec {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn range_chart_from_runs_mean_range_and_pooled() {
        let runs = r#"[[1.0, 1.2], [2.0, 2.1], [1.5, 1.9], [1.1, 1.0], [0.9, 1.3]]"#;
        let a = evaluate(&spec(&format!(r#"{{"quantity": "x", "chart": "range", "runs": {runs}}}"#))).unwrap();
        let ranges = [0.2, 0.1, 0.4, 0.1, 0.4];
        let mean_range = ranges.iter().sum::<f64>() / 5.0;
        assert!((a.center - mean_range).abs() < 1e-12);
        assert_eq!(a.sd_source, SdSource::MeanRange);
        assert!((a.sd.unwrap() - mean_range / std::f64::consts::FRAC_2_SQRT_PI).abs() < 1e-12);
        assert!((a.limits.upper_action - 3.686 * a.sd.unwrap()).abs() < 1e-3 * a.sd.unwrap());
        // Pooled s of duplicates: √(Σd²/2N).
        let a = evaluate(&spec(&format!(
            r#"{{"quantity": "x", "chart": "range", "runs": {runs}, "limits": {{"sd_from": "pooled"}}}}"#
        )))
        .unwrap();
        let pooled = (ranges.iter().map(|d| d * d).sum::<f64>() / 10.0).sqrt();
        assert!((a.sd.unwrap() - pooled).abs() < 1e-12);
        assert_eq!(a.replicates, Some(2));
        // r%: relative ranges, pooled relative s.
        let a = evaluate(&spec(&format!(r#"{{"quantity": "x", "chart": "relative_range", "runs": {runs}}}"#))).unwrap();
        assert!((a.values[0] - 100.0 * 0.2 / 1.1).abs() < 1e-12);
    }

    #[test]
    fn establishing_period_exclusions_and_review() {
        let mut v: Vec<f64> = (0..40).map(|i| 10.0 + if i % 2 == 0 { 0.5 } else { -0.5 }).collect();
        v[35] = 20.0;
        let s = format!(r#"{{"quantity": "x", "values": {v:?}, "establish": 20, "exclude": [3], "review": {{}}}}"#);
        let a = evaluate(&spec(&s)).unwrap();
        assert_eq!(a.stats.unwrap().n, 19);
        assert_eq!(a.establish, 20);
        let r = a.review.as_ref().unwrap();
        assert!(r.previous_computed && r.current_computed);
        assert_eq!(r.excluded, vec![35]);
        assert_eq!(r.current.n, 19);
        assert_eq!(r.outside_warning, Some(1));
        assert!(a.daily[35] == Status::OutsideAction);
        // Alternating values: no trend or run signals.
        assert_eq!(a.statistical_signals(), 0);
    }

    #[test]
    fn invalid_specifications() {
        let bad = [
            r#"{"quantity": "x"}"#,
            r#"{"quantity": "x", "values": [1, 2], "runs": [[1, 2]]}"#,
            r#"{"quantity": "x", "chart": "range", "values": [0.1, 0.2]}"#,
            r#"{"quantity": "x", "chart": "range", "replicates": 2, "values": [0.1, -0.2]}"#,
            r#"{"quantity": "x", "values": [1, 2], "establish": 3}"#,
            r#"{"quantity": "x", "values": [1, 2], "exclude": [0]}"#,
            r#"{"quantity": "x", "values": [1, 2], "limits": {"basis": "target"}}"#,
            r#"{"quantity": "x", "values": [1, 2], "limits": {"basis": "target", "sd": 1, "sd_percent": 2}}"#,
            r#"{"quantity": "x", "values": [1, 2], "limits": {"basis": "tolerance", "sd": 1}}"#,
            r#"{"quantity": "x", "values": [1, 2], "limits": {"sd": -1}}"#,
            r#"{"quantity": "x", "values": [1, 2], "limits": {"action_factor": 2.5}}"#,
            r#"{"quantity": "x", "values": [1, 2], "labels": ["a"]}"#,
            r#"{"quantity": "x", "central_line": {"reference": 5}, "reference": 5, "limits": {"sd": 1}}"#,
            r#"{"quantity": "x", "values": [1, 2], "limits": {"basis": "target", "repeatability_limit": 1}}"#,
            r#"{"quantity": "x", "values": [1, 2, 3], "establish": 2, "review": {}}"#,
        ];
        for b in bad {
            assert!(evaluate(&spec(b)).is_err(), "{b}");
        }
        assert!(serde_json::from_str::<ControlChartSpec>(r#"{"quantity": "x", "limit": {}}"#).is_err());
    }

    #[test]
    fn action_only_and_target_ranges() {
        let a = evaluate(&spec(r#"{"quantity": "x", "central_line": {"mean": 10}, "limits": {"sd": 1, "action_only": true, "action_factor": 2.5}}"#)).unwrap();
        assert!(!a.limits.has_warning_limits());
        assert_eq!((a.limits.lower_action, a.limits.upper_action), (Some(7.5), 12.5));
        let a = evaluate(&spec(r#"{"quantity": "x", "chart": "range", "replicates": 3, "limits": {"basis": "target", "sd": 1, "action_only": true}}"#)).unwrap();
        assert_eq!(a.center_source, CenterSource::ExpectedRange);
        assert!((a.center - 1.692569).abs() < 1e-9 && (a.limits.upper_action - 4.357673).abs() < 1e-5);
    }
}
