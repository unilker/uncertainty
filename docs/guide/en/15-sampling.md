# 15. Uncertainty arising from sampling (Eurachem UfS:2019)

When the measurand is the concentration in a *sampling target* (a batch, a field, a well), not in the
laboratory sample, sampling contributes to the uncertainty of the result, often more than the analysis.
ISO/IEC 17025:2017 7.6.1 asks for every significant contribution, "including those arising from sampling", to
be taken into account. The `sampling` section follows the Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide
*Measurement uncertainty arising from sampling*, 2nd edition (2019), with its errata. It is validated against
Examples A1 to A6 of the Guide (library projects `ufs-*`).

The Guide describes two approaches:

- the **empirical** ("top-down") approach: replicate sampling and analysis of the targets, with an analysis of
  variance that separates the sampling and analytical variances (15.1–15.3);
- the **modelling** ("bottom-up") approach: a budget of the individual effects. These are either the
  fundamental sampling error of Gy's sampling theory (15.4) or a measurement model with correction factors for
  each sampling effect (15.5).

## 15.1 The duplicate method (Section 9.4.2)

From at least eight sampling targets, two samples are taken by independent applications of the sampling
protocol, and each sample is analysed twice. This balanced design gives, for each target, the four results
S1A1, S1A2, S2A1 and S2A2 (`duplicates`). The model of the Guide (Equations 1 and 2) is

x = X_true + ε_target + ε_sampling + ε_analysis,  s_meas² = s_samp² + s_anal².

The **classical nested analysis of variance** (`"method": "anova"`, the default) estimates the three
variances from the mean squares of the levels:

| Level | Degrees of freedom (n targets) | Estimate |
|---|---|---|
| analysis | 2n | s_anal² = MS_anal |
| sampling | n | s_samp² = (MS_samp − MS_anal)/2 |
| between targets | n − 1 | s_T² = (MS_target − MS_samp)/4 |

A negative estimate means that the variance is small compared with the variance below it. It is
conventionally set to zero, and the report says so (Example A4, 4 g test samples). Relative uncertainties
refer to the mean x̄ of all results. The relative expanded uncertainty is U′ = k·100·s/x̄ with k = 2
(Equation 4). The report also gives the share of each level in the total variance, which the Guide uses to
judge fitness for purpose (Section 16.2; ideally the measurement contributes at most 20 %).

Designs that are not balanced (Appendix D2), e.g. with only one analysis of the second sample, are entered as
`targets`: for each target a list of samples, for each sample its analyses. The program then uses the method of
moments for the unbalanced nested design. The coefficients of the expected mean squares, 2, 2 and 4 in the
balanced case, are shown in the report.

**Robust ANOVA.** The Guide's main results come from the robust ANOVA programs RANOVA and RANOVA2. Their
algorithm is not published in the Guide. Reconstructions based on Huber's H15 reproduce the robust *mean*
of Examples A1 and A2, but not the robust standard deviations, so the robust method is not implemented
(research log, F63). The classical results, which the Guide prints alongside, are reproduced exactly.
Inspect the data for outlying targets (Example A1, target C): they inflate the classical estimates.

## 15.2 The range method (Example A3)

For the balanced design, the Guide also uses simple range calculations (`"method": "range"`):

- s_A = R̄_A/d₂, where R̄_A is the mean of the analytical ranges |S1A1 − S1A2| and |S2A1 − S2A2|;
- s_S+A = R̄_S/d₂, where R̄_S is the mean range |x̄_S1 − x̄_S2| of the sample means; then
  s_S = √(s_S+A² − s_A²/2);
- s_T = √(s_T+S+A² − s_S+A²/2), where s_T+S+A is the standard deviation of the target means.

Here d₂ = 2/√π = 1.128 for ranges of two values. The printed Table A3.7 contains an error in the
between-target standard deviation, which the errata (version 1.1) correct. The program reproduces the
corrected values.

## 15.3 Log-normal values and the uncertainty factor (Section 9.5.3, Example A2)

When the values are skewed, and especially when the relative standard uncertainty exceeds about 20 %, the
uncertainty is better expressed as a factor. With `"log": true` the program also runs the classical ANOVA on
ln x. It reports the standard deviations s_G of the natural logarithms and the expanded uncertainty factors

F_U = exp(k·s_G)  (Equation 5),

for sampling, analysis and measurement. The interval is x/F_U to x·F_U, asymmetric around the result. For
Example A2, a result of 300 mg/kg with F_U = 2.62 gives 115 to 784 mg/kg instead of 49 to 551 mg/kg with
U′ = 83.6 %. The report also gives the relative standard uncertainty u′ = √(exp(s_G²) − 1). It warns when u′
exceeds 0.2, since u′ should then not simply be multiplied by 2.

## 15.4 Fundamental sampling error (Gy, Section 10.2, Example A5)

For particulate materials sampled correctly, the minimum sampling error of each step is the fundamental
sampling error (`gy`). Its relative variance is

s_r² = C·d³·(1/M_S − 1/M_L)  (Equation 6), with C = f·g·β·c,

where M_S and M_L are the masses of the sample and the lot, d is the 95 % upper particle size, f the shape
factor (0.5 for spheroidal particles), g the size-distribution factor (0.25 wide, 1 uniform) and β the
liberation factor (1 when liberated). The constitution factor (Equation 8) is

c = (1 − a_L/α)²/(a_L/α)·ρ_c + (1 − a_L/α)·ρ_m,

with the concentration a_L of the lot and α of the critical particles, and the densities ρ_c and ρ_m of the
critical and matrix particles. The steps (primary sample, sub-sampling, test portion) and the analytical
relative standard deviation are combined as a sum of variances. The report shows which step is the weakest
link. In Example A5 it is the 2 g test portion (s_r = 13 % of a total of 14.3 %).

## 15.5 Modelling with a measurement model (Example A6)

When the individual sampling effects are evaluated by exploratory experiments, a measurement model holds them:
x_site = x̄·f_b-loc·f_strat·f_depth·f_prep·f_dry·f_anal, with correction factors of nominal value 1. This is an
ordinary `model` project (chapter 3), as in the library projects `ufs-a6-cadmium-soil` and
`ufs-a6-phosphorus-soil`. The between-location term is a Type A input from the composite samples of the
squares, the depth effect a rectangular term, and the analysis its own components.

## 15.6 Quality control of sampling (Section 13.2, Example A4)

Once validated, the sampling uncertainty is monitored by duplicate sampling of a fraction of the targets. The
relative difference D % = |x₁ − x₂|/x̄·100 of the duplicates is plotted on a range chart (chapter 14,
`"chart": "relative_range"`). The limits come from the validated u = √(u_samp² + u_anal²): central line
1.128·u, warning limit 2.83·u, action limit 3.69·u (`ufs-a4-vitamin-a-qc-chart`).

## 15.7 What the evaluation does not include

Neither the duplicate method nor the fundamental sampling error includes **sampling bias**. The Guide
addresses it with several samplers or protocols (Table 5), with a reference sampling target, or by judgement.
Analytical bias can be added as in Example A2 (Section 6.4), e.g. in a model project. The uncertainty applies
to targets like those of the validation. When the measurand is the mean over a whole site, the uncertainty of
that mean (s_total/√n) is the relevant one (Example A2, Section 7).
