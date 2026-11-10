# 4. Your first calculation, step by step

We now measure the length of a table with a steel tape measure and calculate its uncertainty with the program, click by click. It takes about ten minutes.

<a id="measurement"></a>

## The measurement

You measure the long edge of the table five times and read, to the nearest millimetre:

```
1502, 1503, 1501, 1502, 1503 mm
```

The tape has a calibration certificate: *correction 0 mm, expanded uncertainty U = 0.6 mm with k = 2*. The tape is graduated in millimetres, so each reading is rounded to the nearest millimetre.

<a id="step-model"></a>

## Step 1: the model

The length is the reading plus two corrections whose best value is zero, but whose uncertainty is not:

```
L = L_r + d_cal + d_res
```

| Input | Meaning | Information | Type |
|---|---|---|---|
| L_r | reading of the tape | five readings | A |
| d_cal | correction from the calibration certificate | 0 mm, U = 0.6 mm, k = 2 | B |
| d_res | rounding to the nearest millimetre | between −0.5 and +0.5 mm | B |

<a id="step-create"></a>

## Step 2: create the project

1. On the home screen, press **New measurement model**. A project opens with a small example (a cadmium solution) that we will replace.
2. On the **Overview** tab, type a title, for example *Length of a table*.
3. Open the **Model** tab.

<a id="step-output"></a>

## Step 3: enter the output and the equation

1. Under **Output quantity**, set **Name** to `L` and **Unit** to `mm`.
2. In **Model equation**, delete the old equation and type `L_r + d_cal + d_res`.

> **Tip:** Names may contain letters, digits and the underscore. Multiplication is `*`, division `/`, powers `^`, and functions such as `sqrt()`, `exp()`, `ln()`, `sin()` are available.

<a id="step-inputs"></a>

## Step 4: enter the three inputs

The example already has three input cards. Change them one by one.

**First input: the readings (Type A).**

1. **Name**: `L_r`, **Unit**: `mm`.
2. **Distribution**: choose *Type A (observations)*.
3. In **Observations (one per line)** type the five readings, one per line.

**Second input: the calibration of the tape.**

1. **Name**: `d_cal`, **Unit**: `mm`.
2. **Distribution**: *normal*.
3. **mean**: `0`. Leave **standard uncertainty** empty, and enter **expanded uncertainty U** = `0.6` and **coverage factor k** = `2`. The program divides U by k for you.

**Third input: the resolution.**

1. **Name**: `d_res`, **Unit**: `mm`.
2. **Distribution**: *rectangular*.
3. **value**: `0`, **half-width a**: `0.5`.

> **Warning:** When you change the distribution of an input, check every field again. A value kept from the old example (such as 0.9999 or 100) is a common source of strange results.

<a id="step-calculate"></a>

## Step 5: calculate

Press the red **Calculate** button. The **Results** tab opens.

<a id="step-results"></a>

## Step 6: read the results

**Reported result.** At the top you find the sentence to copy into a report:

```
L = (1502.2 ± 1.1) mm
```

with the explanation that the expanded uncertainty uses k = 2 and corresponds to about 95 %.

**The GUM table.**

| | Value | What it means |
|---|---|---|
| Estimate | 1502.20 mm | the mean of the five readings, plus the two zero corrections |
| Combined standard uncertainty | 0.560 mm | all three inputs together |
| Effective degrees of freedom | 20.0 | how reliable u_c is; 20 is good enough for k = 2 |
| Coverage factor | 2.00 | as set in **Settings** |
| Expanded uncertainty | 1.12 mm | 2 × 0.560, reported as 1.1 mm |

**The uncertainty budget.**

| Input | Standard uncertainty | Share of variance | Where it comes from |
|---|---|---|---|
| L_r | 0.374 mm | 44.7 % | s = 0.837 mm of the readings, divided by √5 |
| d_cal | 0.300 mm | 28.7 % | 0.6 / 2 |
| d_res | 0.289 mm | 26.6 % | 0.5 / √3 |

All sensitivity coefficients are 1, because the model only adds. Notice how the three contributions combine: 0.374 + 0.300 + 0.289 = 0.963, but √(0.374² + 0.300² + 0.289²) = 0.560. Uncertainties do not add up directly; their squares do.

**The Monte Carlo section** gives a slightly larger standard uncertainty (about 0.67 mm). Do not worry about it now: chapter [7](07-monte-carlo.md#table) explains why, using this very example.

<a id="step-improve"></a>

## Step 7: improve the measurement

The budget tells you what to do. The readings (L_r) contribute most. Measuring ten times instead of five would divide their standard uncertainty by √2. A tape with a better certificate would reduce d_cal. Try it: add five more readings in the **Model** tab and press **Calculate** again.

> **Try:** Ask for a coverage probability instead of a fixed coverage factor. In **Settings**, choose **Coverage probability (k from t)** and enter `0.95`. The program then chooses k from the effective degrees of freedom (k = 2.09 for ν_eff = 20) and U becomes a little larger.

<a id="left-out"></a>

## What we left out

A real length measurement also considers the temperature (a steel tape expands by about 0.012 mm per metre and per degree), the alignment of the tape and whether the edges are straight. Each would be another input of the model. The libraries' [vernier calliper](examples.md#ea402-s10-vernier-calliper) and [gauge block](examples.md#ea402-s4-gauge-block-50mm) examples show such effects.

<a id="save"></a>

## Save and report

- **Save** (disk icon) stores the project as a `.json` file that you can open later with **Open project file**.
- The menu ⋮ exports the report as **PDF**, **HTML** or **Markdown**, in the language shown at the top right (EN/TR).

Chapter [5](05-where-numbers-come-from.md) now shows how to turn any kind of information into a standard uncertainty.
