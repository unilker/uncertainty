# 8. Which kind of project do I need?

The home screen has a card for each kind of evaluation. Find your question in the table, open the card, and read the section of the guide it points to. Each row also names an example of the library that you can open and run.

<a id="table"></a>

## Find your question

| Your question | Card | Read | Example |
|---|---|---|---|
| I can write an equation for my result and I know the inputs | **New measurement model** | [chapter 4](04-first-calculation.md) | [Calibration of a 10 kg weight](examples.md#ea402-s2-weight-10kg) |
| I am a testing laboratory and I have validation and quality-control data (control charts, CRMs, proficiency tests) | **New top-down evaluation** | [top-down](09-laboratory-data.md#topdown) | [Ammonium in water](examples.md#nordtest-3-ammonium-pt) |
| I organise a proficiency test and must score the participants | **New proficiency test round** | [proficiency testing](09-laboratory-data.md#proficiency) | [Mercury in animal feed](examples.md#iso13528-e4-mercury-feed) |
| I calibrate an analyser with reference mixtures, and both axes have uncertainties | **New calibration (both axes)** | [calibration curves](10-special-evaluations.md#calibration) | [NO in N₂ by UV](examples.md#iso6143-b1-no-uv-linear) |
| Does my result (or product) meet a specification or a legal limit? | **New conformity assessment** | [conformity](10-special-evaluations.md#conformity) | [Speed limit enforcement](examples.md#jcgm106-8.3-speed-limit) |
| Several laboratories measured the same samples; I need repeatability and reproducibility of the method | **New interlaboratory precision study** | [precision studies](09-laboratory-data.md#precision) | [Sulfur in coal](examples.md#iso5725-2-c1-sulfur-in-coal) |
| My model gives several results at once, which are correlated | **New model with several outputs** | [several outputs](10-special-evaluations.md#multivariate) | [Two outputs, Gaussian inputs](examples.md#jcgm102-9.2.2-additive-gaussian) |
| I want to monitor a method with a control sample | **New control chart** | [control charts](09-laboratory-data.md#control-chart) | [Zinc in water](examples.md#tr569-table1-zinc) |
| How much of my uncertainty comes from taking the sample? | **New sampling uncertainty** | [sampling](09-laboratory-data.md#sampling) | [Nitrate in lettuce](examples.md#ufs-a1-nitrate-lettuce) |
| I calibrate a non-automatic weighing instrument | **New error function** | [weighing instruments](10-special-evaluations.md#weighing) | [Laboratory balance 220 g](examples.md#cg18-h1-a-weighing-result) |
| I must combine results of several laboratories into one value (key comparison, reference value) | **New consensus value** | [consensus values](10-special-evaluations.md#consensus) | [Half-life of ¹³⁷Cs](examples.md#koepke2017-half-life-cs137) |

<a id="bottom-up-top-down"></a>

## Bottom-up or top-down?

This is the most common doubt in a testing laboratory.

- Choose a **measurement model** (bottom-up) when the method is well understood, when it is a calibration, or when you need to know the contribution of each step (for example to improve the method).
- Choose a **top-down** evaluation when the method is complex (sample preparation, matrix effects, many steps) and you have good long-term data: a control chart covering months and a way to check bias. Top-down evaluations automatically include effects that a model may forget.

Both can be combined: a top-down estimate for the analytical method plus a model for a sub-sampling step, or a model checked against the top-down estimate.

> **Tip:** The Eurachem example [Pesticides in bread](examples.md#quam-a4-pesticides-topdown) is evaluated in both ways in the library ([as a combination of steps](examples.md#quam-a4-pesticides-combination)). Comparing the two is very instructive.

<a id="conformity-on-model"></a>

## A conformity question on top of a model

You do not need a separate project to judge a result against a limit. In a measurement model, open **Settings** and switch on **Assess conformity with a tolerance**: the program then calculates the result and, right after it, the probability that it meets the specification.

<a id="start"></a>

## Starting from an existing file or example

- **Open project file** opens a `.json` project you saved earlier, or one a colleague sent you.
- Every library example can be modified and saved under a new name: open a similar example, change the numbers to yours, and press **Save**. This is often the fastest way to start.
