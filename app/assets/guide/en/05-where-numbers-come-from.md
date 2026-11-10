# 5. Where do the numbers come from?

The hardest part of an uncertainty evaluation is not the calculation, which the program does. It is turning the information you have about each input into a **standard uncertainty**. This chapter is a cookbook: find the kind of information you have, and it tells you which distribution to choose and what to enter.

## The quick table {#table}

| What you have | Distribution in the program | What to enter | Standard uncertainty |
|---|---|---|---|
| A certificate: "U = 0.6 mm, k = 2" | normal | mean, **expanded uncertainty U**, **coverage factor k** | U/k = 0.3 mm |
| A certificate: "u = 0.3 mm" | normal | mean, **standard uncertainty** | 0.3 mm |
| A tolerance or specification "± a" | rectangular | value, **half-width a** | a/√3 |
| The resolution of a digital display d | rectangular | value, half-width d/2 | d/(2√3) |
| Limits, but the centre is more likely | triangular | value, half-width a | a/√6 |
| A value that cycles between limits | arc sine (U-shaped) | value, half-width a | a/√2 |
| Repeated readings of this measurement | Type A (observations) | the readings, one per line | s/√n |
| The mean of n readings, with a standard deviation known from earlier | Type A, pooled | mean, pooled s, its degrees of freedom, n | s/√n |
| A certificate value with few degrees of freedom | t (scaled, shifted) | mean, U and k (or scale) and the degrees of freedom | U/k |
| An exact value (a defined constant, a nominal value) | constant | value | 0 |

The sections below explain each case with an example.

## Calibration certificates {#certificate}

A certificate normally states an expanded uncertainty U and its coverage factor k: "(10.000 05 ± 0.000 20) g, k = 2". The standard uncertainty is U/k = 0.000 10 g.

- If the certificate says "95 %" without k, use k = 2 (k = 1.96 if it says the distribution is normal and the degrees of freedom are large).
- If it gives the effective degrees of freedom (for example ν = 9), choose the **t** distribution and enter them: few degrees of freedom make the coverage factor of your own result larger.
- If you use the *nominal* value of a standard (10 g) rather than the certified value (10.000 05 g), the deviation 0.000 05 g is a systematic error that must either be corrected or added to the uncertainty.

> **Example:** In [Calibration of a 10 kg weight](example:ea402-s2-weight-10kg) the reference weight has a certificate with U = 45 mg and k = 2: its standard uncertainty is 22.5 mg, the largest in the budget.

## Tolerances and specifications {#tolerance}

A manufacturer or a standard says that a value lies within limits, for example "a 100 mL volumetric flask class A: ± 0.10 mL". You know nothing else, so every value between the limits is equally likely: a **rectangular** distribution with half-width a = 0.10 mL and

```
u = a / √3 = 0.10 / 1.732 = 0.058 mL
```

If you have reasons to think that values near the nominal value are more likely (good production control, as for volumetric glassware), the **triangular** distribution is used instead: u = a/√6 = 0.041 mL. The Eurachem guide uses triangular for glassware; when in doubt, rectangular is the safer choice because it gives the larger u.

Instrument specifications are often written as "± (0.02 % of reading + 2 digits)". At a reading of 10.000 V with a last digit of 0.001 V: a = 0.002 + 0.002 = 0.004 V, and u = 0.004/√3 = 0.0023 V.

## Resolution of a display or a scale {#resolution}

A digital balance that shows 0.1 g rounds every value to the nearest 0.1 g. The true value can be anywhere within ± 0.05 g of the shown value: rectangular, half-width 0.05 g, u = 0.029 g. For an analogue scale, the half-width is what you can honestly read, often half a division or less.

> **Warning:** Resolution is counted once for each reading that enters the result. A mass obtained as the difference of two balance readings (gross − tare) has two resolution terms.

## Repeated readings: Type A {#type-a}

If you repeat the measurement n times, the readings scatter. Their standard deviation s describes the scatter of a single reading; the standard uncertainty of their **mean** is

```
u = s / √n
```

with ν = n − 1 degrees of freedom. In the program, choose **Type A (observations)** and type the readings: it calculates the mean, s, u and ν.

- If your reported result is **one single reading** in routine work, the scatter of one reading counts, so use s (not s/√n), preferably determined earlier from many readings.
- With very few readings (2 or 3), s itself is very uncertain. A better choice is a standard deviation determined earlier from more data: **Type A, pooled**. For example, the mean of 3 comparisons with a pooled standard deviation of 25 mg from an earlier study: u = 25/√3 = 14.4 mg.

## Values in percent, relative values {#relative}

"The relative standard uncertainty of the purity is 0.05 %" means u = 0.0005 × (the value). If the purity is 0.9999, u = 0.000 50. Always convert to the unit of the input before entering it. For models made of products and ratios, the program shows the relative contributions anyway.

## Effects that depend on something else {#effects}

Some influences are better written into the model than estimated directly. A volume changes with temperature:

```
V = V_20 · (1 + γ · (t − 20))
```

The laboratory temperature t is known to ± 2 °C (rectangular), the expansion coefficient of water γ = 2.1 × 10⁻⁴ /°C. The program then calculates the effect itself: for 100 mL, about 0.024 mL. Writing such effects into the model makes the evaluation clearer and avoids mistakes with signs and units.

## Correlated inputs {#correlation}

Inputs are correlated when a common cause acts on both. Typical cases:

- two weighings on the same balance with the same calibration,
- several solutions diluted from the same stock solution,
- certified values of several standards calibrated together,
- the same temperature effect acting on two quantities.

Enter them in the **Model** tab under **Correlations**, with the correlation coefficient r (from −1 to +1). If the correlation is caused by a shared quantity, it is often simpler and safer to write that quantity into the model once (for example the same stock concentration in both expressions): the program then handles the correlation automatically.

## When there is no information {#no-information}

Sometimes nothing is documented, for example the effect of an operator on a reading. Then use your experience: "I am sure the effect is not larger than ± 0.2 mm" is information, and a rectangular distribution with a = 0.2 mm is a reasonable, documented choice. Never leave out an effect only because it is difficult to estimate: a rough estimate is better than zero.

> **Tip:** Write down where each number comes from. The **description** of every input appears in the report, so that an assessor (and you, a year later) can follow the budget.

## Checklist before you calculate {#checklist}

- Every input has a unit, and the units fit the model.
- No expanded uncertainty was entered as a standard uncertainty (divide U by k).
- Each effect appears once, and only once.
- Corrections with a zero best value are still in the model, with their uncertainty.
- Correlations are entered, or the shared quantity is in the model.
