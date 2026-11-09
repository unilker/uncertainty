# 10. Special evaluations

This chapter covers calibration curves, conformity decisions, models with several results, weighing instruments and consensus values. Each section can be read on its own.

<a id="calibration"></a>

## Calibration curves with uncertainty on both axes

**When.** An analyser (gas analysis, spectrometry) gives a response that must be converted into a content, using reference mixtures whose contents are themselves uncertain. ISO 6143 describes this.

**The idea.** An ordinary least-squares line assumes that only the responses are uncertain. Here both the contents (x) and the responses (y) have uncertainties, so the program uses *generalised least squares*: it finds the curve that best fits the points while allowing each point to move along both axes according to its uncertainty.

**What you need.** For each reference mixture: its content x with u(x), and the measured response y with u(y). For each unknown sample: its response and u(y).

**How to enter it.** In the **Data** tab, enter the points (one per line), choose the analysis function (straight line, second or third degree polynomial, power or exponential) and enter the samples. If the reference mixtures are correlated (for example made from the same parent gas), enter their covariances.

**What you get.**

- the parameters of the function and their uncertainties and covariances;
- **Γ** (the largest weighted residual) and the residual sum of squares: if Γ ≤ 2 the function fits; otherwise try another function;
- the content of every sample with its uncertainty, which includes the uncertainty of the curve.

> **Example:** [A straight line](examples.md#iso6143-b1-no-uv-linear), [straight line or second-degree polynomial](examples.md#iso6143-b2-n2-natural-gas-quadratic), [correlated reference mixtures](examples.md#iso6143-b2-n2-natural-gas-covariances).

<a id="conformity"></a>

## Conformity assessment

**When.** You must decide whether an item, a sample or an instrument meets a specification: below a legal limit, within a tolerance, inside the maximum permissible error. JCGM 106 and the Eurachem/CITAC guide on compliance describe how.

**The problem.** A measured value close to a limit could be on either side of it because of the uncertainty. Deciding only by the measured value means accepting some bad items and rejecting some good ones.

**The tools.**

- **Probability of conformity**: from the measured value and its uncertainty, the probability that the true value is inside the limits. For example 97 %.
- **Decision rule**: agreed before measuring. *Simple acceptance*: accept if the measured value is inside. *Guarded acceptance*: accept only inside a smaller *acceptance zone*, reduced by a **guard band**; this protects the customer. *Guarded rejection*: reject only when clearly outside; this protects the producer, or the accused in law enforcement and anti-doping.
- **Specific risk**: the probability of a wrong decision for this one item.
- **Global risks**: with knowledge of the production process, the probability that an accepted item is actually bad (consumer's risk) or a rejected one actually good (producer's risk), over the whole production.

**How to enter it.** Either as a separate project (**New conformity assessment**: enter the limits, the measured values and their uncertainty), or on top of a measurement model (**Settings → Assess conformity with a tolerance**). Choose the decision rule and, for guard bands, the required probability or risk.

**Test uncertainty ratio.** In calibration, acceptance limits are often derived from the ratio of the tolerance to the uncertainty (TUR). The program offers the usual guard-band methods (root-sum-square, Dobbert's managed guard band, NCSLI RP-10) and shows the risks they lead to.

> **Example:** [Probability of conformity of a Zener diode](examples.md#jcgm106-7.3-zener-diode), [speed limit with guarded rejection](examples.md#jcgm106-8.3-speed-limit), [guarded acceptance for nickel in steel](examples.md#eurachem-compliance-b1-nickel-steel), [guard band from the TUR](examples.md#tur-guard-band-dobbert-tur2-5).

> **Warning:** The decision rule must be agreed with the customer and stated in the report (ISO/IEC 17025 7.1.3 and 7.8.6). Choosing it after seeing the result is not acceptable.

<a id="multivariate"></a>

## Models with several results

**When.** One model gives several results at once: the amount fractions of all components of a gas mixture, the resistance and reactance of an impedance, the parameters of a curve. Because they share inputs, the results are **correlated**, and that matters when they are used together later.

**What you get** (JCGM 102):

- each result with its standard uncertainty;
- the **covariance matrix** and the correlation coefficients between the results;
- a **coverage region**: for two results an ellipse (GUM) or the smallest region found by Monte Carlo, instead of two separate intervals.

**How to enter it.** Like a measurement model, but with a list of outputs, each with its own equation. If the equations cannot be solved for the results (an *implicit* model), switch on **Implicit model h(Y, X) = 0** and give each output a starting value: the program solves the equations numerically.

> **Example:** [Two outputs from shared inputs](examples.md#jcgm102-9.2.2-additive-gaussian), [Cartesian to polar coordinates](examples.md#jcgm102-9.3-polar-x010), [a gravimetric gas mixture with eight components](examples.md#gum5-e4-gravimetric-gas-mixture).

<a id="weighing"></a>

## Weighing instruments

**When.** You calibrate a non-automatic weighing instrument (a balance, a scale, a weighbridge) according to EURAMET cg-18, and you want the error and its uncertainty at every load, and the uncertainty of a weighing result in use.

**The idea.** The balance is tested at several loads with standard weights. At each load the **error** E = indication − reference mass is found, with an uncertainty from repeatability, the rounding of the display (at zero and at load), eccentricity, the weights and air buoyancy. A straight line (the **error approximation function**) through the errors then gives the error and its uncertainty at any load.

**How to enter it.** In the **Data** tab, enter for each test load the indication, the error and the standard uncertainties; choose whether the line goes through zero. For the use of the balance, add the intervals, the eccentricity, the temperature effect and the requirement for the minimum weight.

**What you get.** The coefficients of the line and their uncertainties, a χ² test and a residual test (does the line fit?), the error and uncertainty at every reading, the uncertainty of a weighing result in use and the **minimum weight**.

> **Example:** [A 220 g laboratory balance](examples.md#cg18-h1-a-weighing-result), [a weighbridge](examples.md#cg18-h3-a-weighing-result), [an error approximation function](examples.md#cg18-h4-400g-error-function).

<a id="consensus"></a>

## Consensus values and degrees of equivalence

**When.** Several laboratories (or experiments) measured the same quantity, each with its own uncertainty, and one combined value is needed: the reference value of a key comparison, a certified value from an interlaboratory characterisation, a physical constant.

**The problem.** Results often disagree by more than their uncertainties allow. This extra spread between laboratories is called *dark uncertainty*. A simple weighted mean then gives a value with a much too small uncertainty.

**The procedures** (as in the NIST Consensus Builder):

| Procedure | Idea |
|---|---|
| **DerSimonian–Laird** | random-effects model: estimates the extra spread τ and adds it to every laboratory's uncertainty; with Knapp–Hartung adjustment or a parametric bootstrap for the uncertainty |
| **Linear Pool** | mixes the distributions of all laboratories; very robust, large uncertainty |
| **Hierarchical Bayes** | random effects with prior distributions, evaluated exactly on a grid (no random sampling) |
| **Maximum likelihood** | random-effects model estimated by maximum likelihood |

**Degrees of equivalence.** For each laboratory D_j = x_j − x_ref with its expanded uncertainty: how far, and how significantly, it is from the consensus value. *Leave-one-out* compares each laboratory with the consensus of the others, and *bilateral* degrees of equivalence compare pairs of laboratories.

> **Example:** [Half-life of ¹³⁷Cs](examples.md#koepke2017-half-life-cs137), [the gravitational constant by several procedures](examples.md#merkatas2019-newtonian-constant-g-shades), [degrees of equivalence of 19 laboratories](examples.md#nicob-co60-activity-leave-one-out-bilateral).

> **Note:** The procedures can give quite different values and uncertainties for the same data. The choice is part of the evaluation, should be made before the data are seen, and must be stated in the report.
