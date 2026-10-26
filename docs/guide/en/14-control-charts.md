# 14. Control charts for internal quality control (Nordtest TR 569)

ISO/IEC 17025:2017 7.7.1 requires a laboratory to monitor the validity of its results so that trends can be
detected, using statistical techniques where practicable. The usual tool is the control chart: a control
sample is analysed in every run and the control value is plotted against limits. The `control_chart` section
follows the Nordtest *Handbook of Internal Quality Control*, TR 569 ed. 6 (2026), the "Trollbook". It is
validated against Table 1, the cases of Chapter 7 and Examples 1 to 12 of the handbook (library projects
`tr569-*`).

The data of a control chart also give the within-laboratory reproducibility u(R_w) of a top-down evaluation
(chapter 5). In that case, enter them in a `topdown` project.

## 14.1 Types of chart (Chapter 5)

- **X-chart** (`"chart": "x"`, the default): single control values or the mean of the replicates of each
  run, around a central line. Special cases are the blank chart (negative values are plotted too) and the
  recovery chart.
- **R-chart** (`"range"`): the range (largest − smallest result) of n replicates of a test sample. It
  monitors repeatability and has upper limits only.
- **r%-chart** (`"relative_range"`): the range in per cent of the mean of the replicates. Use it well above
  the reporting limit, where the relative standard deviation is about constant.

Give either the control values (`values`) or the replicate results of each run (`runs`). With `runs`, the
program forms the control value itself: the mean, the range or the relative range.

## 14.2 Central line and control limits (Chapter 7)

The **central line** is the mean of the control values. Alternatively, it can be a mean established earlier
(`"central_line": {"mean": …}`) or the reference or nominal value of a reference material
(`{"reference": …}`). When the central line is the mean and a reference value is known (`reference`), the
report tests the mean against it: t = |x̄ − T|·√n/s (Equation 6).

| Basis | X-chart | R- and r%-chart (n replicates) |
|---|---|---|
| statistical | s of the control values; WL = CL ± 2s, AL = CL ± 3s | s = mean range/d₂; WL = D_WL·s, AL = D_AL·s |
| target | s_target from the requirement; same factors | s_target, or s_r = r/2.8 from a repeatability limit r |
| tolerance | AL = CL ± tolerance, no warning limits | AL = tolerance |

The range factors come from ISO 8258. d₂ and d₃ are the mean and the standard deviation of the range of n
standard normal values, and the upper action limit is D_AL = d₂ + 3d₃. TR 569 adds a warning limit
D_WL = d₂ + (2/3)(D_AL − d₂) (Table 4):

| n | d₂ | D_WL | D_AL |
|---|---|---|---|
| 2 | 1.128 | 2.833 | 3.686 |
| 3 | 1.693 | 3.470 | 4.358 |
| 4 | 2.059 | 3.818 | 4.698 |

The program computes d₂ and d₃ for 2 to 10 replicates. The test suite recomputes them by numerical
integration.

Target limits are the choice when the requirement on the results is known and the method performs better.
TR 569 recommends them for multi-element methods, where statistical limits for 20 analytes would give about
10 % false alarms per run. Two shortcuts:

- from a required relative expanded uncertainty U (k = 2): s_target = U/4, half of u_c (`expanded_percent`,
  Example 1);
- from a repeatability limit r of a standard method: s_r = r/2.8 (`repeatability_limit`).

A chart with **action limits only** (`"action_only": true`) has a default factor of 3 for X-charts and D_AL
for range charts. TR 569 suggests 2.5 s and 3.5 s instead (`action_factor`), which give about the false-alarm
rates of charts with both limits. Such a chart is less sensitive to a drift of the mean because the rule two
out of three is not available.

With `establish`, only the first N control values set the central line and the statistical limits (the
preliminary limits from about 25 values); the later values are judged against them. Runs with an assignable
cause can be left out of the statistics with `exclude`. TR 569 recommends fixed limits based on at least
60 values over about a year.

## 14.3 Daily interpretation (Chapter 9)

After each run:

| Situation | Interpretation |
|---|---|
| within the warning limits | in control |
| between warning and action limits, neither of the two previous values outside the warning limit on the same side | in control (warning zone) |
| outside the action limits | out of control |
| between warning and action limits, one of the two previous values outside the warning limit on the same side ("two out of three") | out of control |

With statistical limits and a mean central line, these two rules give about 0.54 % false out-of-control
situations for an X-chart. For a range chart the figure is about 1.3 %. A value on a limit counts as inside
it. For two out of three, a previous value beyond the action limit counts as outside the warning limit, as in
ISO 7870-2 (research log D43).

The report also lists the values at which the method is **out of statistical control**. These are seven
consecutive values steadily increasing or decreasing, or ten of eleven consecutive values on the same side of
the central line. The results can still be reported, but the cause should be investigated in the long-term
review. With a reference central line, a small bias alone can put many values on one side.

## 14.4 Long-term review (Chapter 10)

At least once a year, the values of the period are compared with those the limits were based on (`review`):

- **standard deviation**: F = s₁²/s₂² with s₁ > s₂ (Equation 5), against the two-sided critical value
  F₁₋α/₂(ν₁, ν₂). The program computes this value exactly; Table 3 of TR 569 rounds the degrees of freedom.
  For about 60 values, a quick check is the number of values outside the warning limits: more than 6 suggest
  that s has increased.
- **mean**: t = |x̄₁ − x̄₂|/s_p·√(n₁n₂/(n₁ + n₂)) (Equation 7), with the pooled s_p of Equation 9 and
  ν = n₁ + n₂ − 2. For about 60 values, a shift below 0.37 s needs no action.

By default the previous series is the establishing period and the current series is the later values.
Values more than 4 s from the central line are excluded from the current series (`exclude_beyond`). Either
series can instead be given as summary statistics (`previous`, `current`: `mean`, `sd`, `n`, `dof`), as in
Example 8. TR 569 changes limits only after a significant change, and moves a mean central line only when the
shift has an explanation.

## 14.5 Precision from replicate control results (Example 12)

When each run contains replicates of the control sample, the standard deviation of all results together
underestimates s_Rw. Control limits based on it would be too narrow. The report then gives:

- s_r pooled over the runs (Equations 9 and 10);
- the one-way analysis of variance: s_r, the between-run s_I and s_Rw = √(s_r² + s_I²);
- for equal numbers of replicates, the simplified s_Rw of Example 12: the standard deviations of the first,
  second, … results over the runs, pooled.

In Example 12 these are s_r = 0.15, s_Rw = 0.270 (simplified) and 0.272 (analysis of variance).

## 14.6 Points to note

- TR 569 rounds intermediate values: Example 6 multiplies the rounded CV of 1.67 % by the factors. The
  library projects give the published values, and their notes record the difference.
- In the examples called "pooled repeatability standard deviation" (Examples 3 and 8), s is the mean range
  divided by 1.128 (Equation 4). `"sd_from": "pooled"` uses the pooled within-run standard deviation of the
  replicates instead (Equation 9).
- Findings in the handbook are recorded in the research log (F52, F53):
  - the warning limit of case 1 of the range charts (Chapter 7) is printed as 3·0.356; the factor is 2.83;
  - footnote 24 of Example 12 prints s_r, s_I and s_Rw ten times too small.
