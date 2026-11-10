# 2. What is measurement uncertainty?

## No measurement is exact {#idea}

Step on a bathroom scale three times in a row. You may read 72.4 kg, 72.6 kg and 72.5 kg. Which one is your "true" weight? None of them exactly. Every reading is a little different, the scale itself may read slightly too high or too low, and the last digit is rounded.

The same is true in every laboratory, only with smaller numbers: a balance, a thermometer, a pipette or an instrument for chemical analysis never gives the exact true value. The **true value** of a quantity can never be known perfectly. What we can do is give a **best estimate** and say **how far the true value could plausibly be** from it. That second part is the *measurement uncertainty*.

A complete measurement result therefore always has two parts:

```
m = (72.5 ± 0.3) kg   (coverage probability about 95 %)
```

Read it as: "Our best estimate is 72.5 kg, and we are about 95 % confident that the true value lies between 72.2 kg and 72.8 kg."

> **Note:** A result without its uncertainty cannot be compared with a limit, with another laboratory or with an earlier result. "72.5 kg" alone does not tell you whether 72.8 kg is a different result or the same one.

## Error and uncertainty are different things {#error}

- The **error** is the difference between a measured value and the true value. Because the true value is unknown, the error is unknown too.
- If part of the error is known (for example, a calibration certificate says that your thermometer reads 0.3 °C too high), you **correct** for it: you subtract 0.3 °C.
- The **uncertainty** describes what remains unknown after all corrections: the doubt about the result.

So uncertainty is not a mistake, and it is not "how wrong" you are. It is an honest statement of how well you know the result.

## Why it matters {#why}

- **Decisions.** Is the lead content of the water below the legal limit? Does the part meet its tolerance? A result close to a limit can only be judged with its uncertainty (chapter [10](10-special-evaluations#conformity)).
- **Comparisons.** Two laboratories measure the same sample and get different numbers. Do they agree? Only their uncertainties can tell.
- **Accreditation.** ISO/IEC 17025 requires testing and calibration laboratories to evaluate the uncertainty of their results (clause 7.6) and to report it on calibration certificates and, where relevant, in test reports.
- **Improvement.** An uncertainty budget shows which effect matters most, so you know where to invest to measure better.

## The recipe in five steps {#recipe}

The international reference is the *Guide to the expression of uncertainty in measurement*, called **the GUM** (JCGM 100). Its method can be summarised in five steps:

1. **Say exactly what you measure** (the *measurand*) and write how the result is calculated from other quantities: the *measurement model*. Example: the concentration of a solution = mass of substance × purity ÷ volume.
2. **List everything that influences the result**: the readings, the calibration of the instruments, the temperature, the purity of a reagent, the resolution of a display…
3. **Give each influence a standard uncertainty**, either from statistics of repeated measurements (*Type A*) or from other information such as a certificate or a specification (*Type B*). Chapter [5](05-where-numbers-come-from) shows how.
4. **Combine** them, taking into account how strongly each one affects the result.
5. **Expand** the combined uncertainty to a wider interval (usually about 95 %) and **report** the result.

You do steps 1 to 3, because they need knowledge of your measurement. **The program does steps 4 and 5** for you, exactly and with every intermediate value shown, and it can check the result with a second method, the Monte Carlo method (chapter [7](07-monte-carlo)).

## Two ways to the same goal {#approaches}

- **Bottom-up (modelling).** Build the model of step 1 and quantify each input. This is typical for calibration laboratories and for well understood methods. In the program: **New measurement model**.
- **Top-down (from laboratory data).** Use the data a testing laboratory already has: how much results scatter over months (from control charts) and how far they are from reference values (from reference materials or proficiency tests). This is typical for chemical and environmental testing. In the program: **New top-down evaluation**.

Both are accepted by ISO/IEC 17025 and both give a standard uncertainty that is expanded and reported in the same way. Chapter [8](08-choosing-a-project-type) helps you choose.

> **Try:** Open [Calibration of a 10 kg weight](example:ea402-s2-weight-10kg). In the **Overview** tab you will find the five steps of the recipe applied to a real calibration.
