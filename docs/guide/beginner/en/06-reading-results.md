# 6. Reading the results

After **Calculate**, the **Results** tab shows everything the program calculated. This chapter goes through it from top to bottom for a measurement model. The other project types have their own results, explained in chapters [9](09-laboratory-data.md) and [10](10-special-evaluations.md), but the ideas are the same.

<a id="reported"></a>

## Reported result

The blue box at the top contains the sentence to copy into a report or certificate, for example:

```
m_X = (25 ± 59) mg; the expanded uncertainty is the combined standard uncertainty
multiplied by the coverage factor k = 2.00 and corresponds to a coverage probability
of approximately 95 %.
```

- The uncertainty is rounded to the number of significant digits chosen in **Settings** (2 by default), and the value is given with the same number of decimals.
- The sentence names the rounding rule. The default is the rule of EA-4/02: round normally, unless this lowers U by more than 5 %, in which case round up.
- "Approximately 95 %": with k = 2 the exact probability is 95.45 % for a normal distribution, but the probability is never known that precisely.

If you see the red line *Unsaved changes* above the results, you changed the project after the last calculation and the results belong to the earlier version: press **Calculate** again.

<a id="gum"></a>

## The GUM result

| Line | Meaning |
|---|---|
| **Estimate** | the value of the result, from the estimates of the inputs |
| **Combined standard uncertainty** | u_c, all inputs together |
| **Effective degrees of freedom** | ν_eff; ∞ when all inputs are Type B with no degrees of freedom |
| **Coverage factor** | k, fixed in **Settings** or chosen from ν_eff for a coverage probability |
| **Coverage probability** | the probability that belongs to k |
| **Expanded uncertainty** | U = k · u_c, before rounding |

If the model is clearly nonlinear and you enabled **Second-order terms** in **Settings**, the estimate and u_c include second-order terms, as the 2026 amendment of the GUM requires.

<a id="budget"></a>

## The uncertainty budget

One line for each input:

| Column | Meaning | What to look at |
|---|---|---|
| Estimate | the value of the input | is it the value you meant? |
| Standard uncertainty | u of the input, in its own unit | was U divided by k? |
| Degrees of freedom | ν of the input | small values make k larger |
| Sensitivity coefficient | c, how the result reacts to the input | the sign and size should make sense |
| Contribution | c · u, in the unit of the result | compare the inputs |
| Share of variance | (c · u)² / u_c², in percent | **the most useful column** |

The share of variance tells you where the uncertainty comes from. If one input has 80 %, that input is where to improve; making a 2 % input ten times better changes almost nothing.

> **Warning:** A sensitivity coefficient of exactly 0 means that, at the estimates you entered, the result does not change with that input to first order. The program then warns you. It can be correct (a correction at a turning point) or a sign of a wrong model; in such cases use the Monte Carlo result or the second-order terms.

The chart *Contributions to the combined variance* shows the same shares as bars.

The **Kragten spreadsheet view** (option in **Settings**) shows the same budget the way it is calculated in a spreadsheet: each input is shifted by its uncertainty and the change of the result is recorded. It is useful when you compare with an existing spreadsheet.

<a id="mcm"></a>

## The Monte Carlo result

If the Monte Carlo method is switched on (**Settings**), a second block follows:

- **Number of trials**: how many random draws were made. With *adaptive* trials the program continues until the results are stable to the number of digits requested.
- **Estimate (mean)** and **Standard uncertainty (standard deviation)** of the simulated results.
- **Coverage interval**: the interval that contains 95 % (or the probability you chose) of the simulated results. *Shortest* is the narrowest such interval, *probabilistically symmetric* leaves 2.5 % on each side. For an asymmetric result the two differ, and the estimate is not in the middle.
- A **histogram** of the simulated results, with the GUM interval for comparison.

**Validation of the GUM result.** The program compares the ends of the GUM interval with the Monte Carlo interval. *Validated* means that the simpler GUM result is good enough at the requested number of digits. *Not validated* means that the two differ: then prefer the Monte Carlo result, or find out why (chapter [7](07-monte-carlo.md#disagree)).

<a id="checks"></a>

## Comparison with reference values

For library examples (and for your own projects if you entered expected values), a table compares each computed value with the value published in the source, with its tolerance. *passed* means the program reproduces the source within the rounding of the printed numbers. When a source contains a mistake, the reference value is an independent recomputation and the **Notes** on the **Overview** tab explain the difference.

<a id="warnings"></a>

## Warnings

Warnings point to things you should check: a zero sensitivity coefficient, a result outside a physical limit, a validation that failed, too few Monte Carlo trials. A warning is not an error, but never ignore one without understanding it.

<a id="questions"></a>

## Common questions about results

**The estimate is not what I expected.** Check the units of every input and the model equation. A forgotten conversion (mL and L, % and fraction) is the most frequent cause.

**U is much larger than I expected.** Look at the share of variance: usually one input is entered too large, for example an expanded uncertainty entered as a standard uncertainty, or a value in the wrong unit.

**The GUM and Monte Carlo results differ.** See chapter [7](07-monte-carlo.md#disagree).

> **Try:** Open [Mass calibration (first-order GUM)](examples.md#jcgm101-9.3-mass-calibration), press **Calculate**, and compare the GUM and Monte Carlo standard uncertainties. Then open the same example [with second-order terms](examples.md#jcgm101-9.3-mass-calibration-higher-order) and see how the GUM result moves towards Monte Carlo.
