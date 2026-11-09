# 3. The key words in plain language

Each term has a short explanation and a small example. The example is always the same simple measurement: **the length of a table measured with a tape measure**, which chapter [4](04-first-calculation.md) then calculates step by step.

<a id="measurand"></a>

## Measurand

The quantity you intend to measure, described precisely enough. "The length of the table" is not quite enough: *which* edge, at *what* temperature? A good definition: "the length of the long edge of the table top at 20 °C".

<a id="model"></a>

## Measurement model

The equation that gives the result from the input quantities. For the table:

```
L = L_r + d_cal + d_res
```

L is the length, L_r the reading of the tape, d_cal the correction from the tape's calibration and d_res the effect of reading to the nearest millimetre. Corrections whose best value is zero still belong in the model, because their *uncertainty* is not zero.

<a id="input"></a>

## Input quantity and estimate

Every quantity on the right-hand side of the model is an *input quantity* (here L_r, d_cal, d_res). Its best value is its **estimate** (in the program: *Estimate* / *Tahmin*). The result L is the *output quantity*.

<a id="standard-uncertainty"></a>

## Standard uncertainty u

How far, typically, the true value may be from the estimate, expressed as a standard deviation. Every input gets a standard uncertainty in the unit of that input. It is the "common currency" of uncertainty: whatever the source of information, it is first converted into a standard uncertainty.

<a id="types"></a>

## Type A and Type B

Two ways to obtain a standard uncertainty. Both are equally valid.

- **Type A**: by statistics of repeated measurements. You measure the table five times; the scatter of the five readings gives u.
- **Type B**: by any other information: a calibration certificate, a manufacturer's specification, a handbook value, the resolution of a display, experience.

<a id="distribution"></a>

## Probability distribution

A description of which values of an input are likely. The program knows many; four cover most needs:

| Distribution | When | Standard uncertainty |
|---|---|---|
| **Normal** | values near the estimate are most likely; certificates, means of many readings | as given, or U/k |
| **Rectangular** | only limits ± a are known, any value inside equally likely; tolerances, resolution, specifications | a/√3 |
| **Triangular** | limits ± a are known, but values near the centre are more likely | a/√6 |
| **U-shaped (arc sine)** | the value spends most time near the limits; a temperature cycling around a set point | a/√2 |

Chapter [5](05-where-numbers-come-from.md) explains how to choose.

<a id="sensitivity"></a>

## Sensitivity coefficient c

How much the result changes when an input changes by one unit. For L = L_r + d_cal + d_res every coefficient is 1: one millimetre more on the tape gives one millimetre more length. For a product such as c = m × P / V, the coefficients are not 1 and they have units. The program calculates them exactly from the model.

<a id="budget"></a>

## Contribution and uncertainty budget

The **contribution** of an input is c × u: its standard uncertainty translated into the unit of the result. The **uncertainty budget** is the table of all inputs with their estimates, standard uncertainties, sensitivity coefficients and contributions. Its last column, the *share of variance*, tells you in percent which input matters most.

<a id="combined"></a>

## Combined standard uncertainty u_c

The standard uncertainty of the result. For independent inputs it is the square root of the sum of the squared contributions:

```
u_c = √( (c₁·u₁)² + (c₂·u₂)² + … )
```

Squares mean that a large contribution dominates: halving a small contribution hardly changes u_c, halving the largest one does.

<a id="dof"></a>

## Degrees of freedom ν

How reliable a standard uncertainty is. A standard deviation from 5 readings has ν = 4 degrees of freedom and is itself quite uncertain; one from 50 readings has ν = 49 and is reliable. A Type B value is usually considered exact (ν = ∞). The *effective degrees of freedom* ν_eff of the result follow from those of the inputs (Welch–Satterthwaite formula).

<a id="expanded"></a>

## Expanded uncertainty U, coverage factor k, coverage probability

The standard uncertainty covers only about 68 % of plausible values. To state an interval that contains the true value with high probability, u_c is multiplied by a **coverage factor** k:

```
U = k · u_c
```

With k = 2 the **coverage probability** is about 95 %. When ν_eff is small, k must be larger (from the t distribution), which the program does if you ask for a probability instead of a fixed k.

<a id="correlation"></a>

## Correlation

Two inputs are *correlated* when a common cause affects both, for example two masses weighed on the same balance, or two solutions prepared from the same stock. The correlation coefficient r goes from −1 to +1. Correlation can increase or decrease the combined uncertainty, so it must be entered (in the **Model** tab, *Correlations*).

<a id="relative"></a>

## Relative uncertainty

The uncertainty divided by the value, often in percent: u/|y|. For models made only of products and ratios, relative uncertainties simply combine like the contributions above.

<a id="bias"></a>

## Bias, recovery and correction

**Bias** is a systematic difference between the results of a method and the true value, found with reference materials or proficiency tests. **Recovery** is the fraction of the analyte a method finds (100 % is ideal). A known bias is corrected; the uncertainty of the correction stays in the budget.

<a id="precision"></a>

## Repeatability and reproducibility

**Repeatability** (s_r): scatter of results under the same conditions, same day, same person, same instrument. **Within-laboratory reproducibility** (s_Rw): scatter over a long time in one laboratory, with different days, people and calibrations. **Reproducibility** (s_R): scatter between different laboratories.

<a id="traceability"></a>

## Traceability and CMC

**Metrological traceability** means that a result is linked to the SI units through an unbroken chain of calibrations, each with its uncertainty. The **CMC** (calibration and measurement capability) is the smallest uncertainty an accredited calibration laboratory may state for a service.

<a id="terms"></a>

## English–Turkish terms

| English | Turkish |
|---|---|
| measurand | ölçülen büyüklük |
| measurement model | ölçüm modeli |
| input / output quantity | girdi / çıktı büyüklüğü |
| estimate | tahmin (kestirim) |
| standard uncertainty | standart belirsizlik |
| combined standard uncertainty | birleşik standart belirsizlik |
| expanded uncertainty | genişletilmiş belirsizlik |
| coverage factor / probability | kapsam faktörü / olasılığı |
| sensitivity coefficient | duyarlılık katsayısı |
| degrees of freedom | serbestlik derecesi |
| uncertainty budget | belirsizlik bütçesi |
| repeatability / reproducibility | tekrarlanabilirlik / tekrar üretilebilirlik |
| bias / recovery | sapma / geri kazanım |
| proficiency test | yeterlilik testi |
| consensus value | uzlaşı değeri |
| degree of equivalence | eşdeğerlik derecesi |
