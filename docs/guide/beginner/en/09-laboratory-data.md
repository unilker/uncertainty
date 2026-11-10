# 9. Evaluations from laboratory data

This chapter explains the project types that work with data a laboratory collects anyway: validation data, control samples, duplicates, proficiency tests and interlaboratory studies. Each section says when to use the type, what you need, how to enter it and what you get.

<a id="topdown"></a>

## Top-down evaluation

**When.** You run a routine test method (chemical, microbiological, environmental) and want its uncertainty from the laboratory's own data rather than from a model of every step. This follows Nordtest TR 537, the Eurachem/CITAC MUPR guide (2026) and ISO 21748.

**The idea.** Two numbers cover almost everything:

```
u = √( u(Rw)² + u(bias)² )
```

- **u(Rw)**, the *within-laboratory reproducibility*: how much results of the same sample scatter over a long time (different days, analysts, calibrations). It usually comes from a control chart.
- **u(bias)**: how well the method hits the true value, from certified reference materials (CRMs), proficiency tests or recovery experiments, including the uncertainty of those reference values.

**What you need.**

- Precision data in one of several forms: a standard deviation from a control chart, duplicate pairs, groups of replicates on different days, or a precision that depends on the concentration.
- Bias data: results on one or more CRMs with their certified values and uncertainties, or proficiency-test results with assigned values, or recovery tests on spiked samples.
- Optionally the concentration levels at which you want the uncertainty.

**How to enter it.** In the **Data** tab:

1. Give the **Measurand** and the **Unit**, and the **Measured values (levels)**, one per line.
2. Under **Precision**, press **+** and choose the kind of data. For control-chart data enter the standard deviation (absolute or in %); for duplicates or groups, paste the results.
3. Under **Bias** or **Recovery**, add your reference materials, proficiency tests or spiked samples.
4. Add further components if the guide asks for them (for example sub-sampling).

**What you get.** For each level: u(Rw), u(bias), the combined u, the expanded U (k = 2) and the relative values. If a recovery differs significantly from 100 %, the program says whether a correction is needed.

> **Example:** [Bias from one CRM](examples.md#nordtest-6.1-one-crm), [from proficiency tests](examples.md#nordtest-6.2-proficiency-tests), [the whole budget for ammonium](examples.md#nordtest-3-ammonium-pt), and the MUPR examples such as [nitrate in drinking water](examples.md#mupr-b1-nitrate).

> **Warning:** The control sample must go through the *whole* method, including sample preparation, otherwise u(Rw) is too small. A CRM measured only a few times gives an uncertain u(bias); proficiency tests over several years are often more representative.

<a id="proficiency"></a>

## Proficiency testing

**When.** You organise a proficiency test (ISO 13528) or want to check the scores of a round.

**The idea.** Every participant measures the same material. An **assigned value** x_pt is fixed (from a reference laboratory, a CRM or the participants' results with robust statistics), and a **standard deviation for proficiency assessment** σ_pt says how close a result must be. Then:

| Score | Formula | Meaning |
|---|---|---|
| z | (x − x_pt)/σ_pt | |z| ≤ 2 satisfactory, 2 < |z| < 3 questionable, |z| ≥ 3 unsatisfactory |
| z′ | z with the uncertainty of x_pt added to σ_pt | used when x_pt is uncertain |
| ζ | (x − x_pt)/√(u(x)² + u(x_pt)²) | does the result agree within the participant's own uncertainty? |
| E_n | (x − x_pt)/√(U(x)² + U(x_pt)²) | the same with expanded uncertainties; |E_n| ≤ 1 is satisfactory |

**What you need.** The participants' results (code, value, and optionally U and k), the method for the assigned value, and σ_pt. For a homogeneity study, the replicate results of each unit.

**How to enter it.** In the **Data** tab, paste the results (one laboratory per line: code, result, U, k). A result such as `<0.015` is treated as censored. Choose the assigned value (robust average by Algorithm A, median, mean or a reference value) and σ_pt.

**What you get.** The assigned value with its uncertainty, every score, a chart of the z-scores and, for homogeneity, the between-unit standard deviation compared with σ_pt.

> **Example:** [Robust average of atrazine results](examples.md#iso13528-e3-atrazine-algorithm-a), [all scores for mercury in feed](examples.md#iso13528-e4-mercury-feed), [homogeneity and stability](examples.md#iso13528-e2-arsenic-homogeneity).

<a id="precision"></a>

## Interlaboratory precision study

**When.** A method is validated by several laboratories (ISO 5725-2) and you need its repeatability s_r and reproducibility s_R, for example to write them in a standard.

**What you need.** For each level, the replicate results of every laboratory (typically 8 to 15 laboratories, 2 replicates each).

**How to enter it.** List the laboratories, then for each level paste a table with one row per laboratory and one column per replicate. Choose the classical analysis of variance or REML (better with missing data).

**What you get.** Cochran's test (one laboratory with too large a scatter), Grubbs's tests (laboratory means that are outliers), Mandel's h and k statistics as charts, then s_r and s_R at each level, and the repeatability and reproducibility limits r = 2.8 s_r and R = 2.8 s_R. Outliers are not removed automatically: you decide, and the decision is documented.

> **Example:** [Sulfur in coal](examples.md#iso5725-2-c1-sulfur-in-coal), [softening point of pitch with missing data](examples.md#iso5725-2-c2-softening-point-pitch).

<a id="control-chart"></a>

## Control charts

**When.** You measure a control sample with every series and want to see whether the method stays in statistical control (Nordtest TR 569).

**The idea.** The results are plotted in time order. A **central line** (the mean or a target value) is surrounded by **warning limits** (± 2 s) and **action limits** (± 3 s). Rules tell you when to act, for example one point outside the action limits, or two of three successive points outside the warning limits on the same side. The program applies the rules of Nordtest TR 569 and marks every point that breaks one.

**What you need.** The control values in time order (or replicate results for R-charts of duplicates), and a decision about the limits:

- *statistical limits*: from the scatter of the data themselves;
- *target limits*: from the precision the method must achieve (for example from the required uncertainty).

**What you get.** The chart with its limits, every point that breaks a rule, and a long-term review: has the scatter changed (F-test)? has the mean shifted (t-test)? Should new limits be set?

> **Example:** [Zinc in water, 60 values](examples.md#tr569-table1-zinc), [the three ways of setting limits](examples.md#tr569-ch7-case1-statistical), [a duplicate R-chart](examples.md#tr569-ex3-ammonium-r).

> **Tip:** The standard deviation of a control chart over a long time is exactly the u(Rw) of a top-down evaluation. The two project types belong together.

<a id="sampling"></a>

## Uncertainty from sampling

**When.** The result describes a whole lot, field or batch, but you analyse only a sample. The variation from where and how the sample is taken can be larger than the analytical uncertainty (Eurachem/EUROLAB/CITAC/Nordtest/AMC guide UfS).

**The duplicate method.** From at least eight targets (lots, fields), take two independent samples each, and analyse each sample twice:

```
target 1: sample A (analysis 1, 2), sample B (analysis 1, 2)
target 2: …
```

An analysis of variance separates three parts: the variation between targets (which is real and not uncertainty), the **sampling** variation and the **analytical** variation. The measurement uncertainty is

```
u = √( u(sampling)² + u(analysis)² )
```

**How to enter it.** Paste one row per target with the four results. For large relative uncertainties, choose the log-transformed evaluation: the result is then given as an **uncertainty factor** (multiply or divide by F_U).

**What you get.** The three variance components, u(sampling), u(analysis), the expanded measurement uncertainty and the shares of sampling and analysis.

> **Example:** [Nitrate in lettuce](examples.md#ufs-a1-nitrate-lettuce), [lead in soil with an uncertainty factor](examples.md#ufs-a2-lead-soil-uncertainty-factor), [Gy's sampling theory](examples.md#ufs-a5-enzyme-feed-gy).
