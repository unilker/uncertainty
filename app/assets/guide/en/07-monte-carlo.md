# 7. The Monte Carlo method, simply explained

## The idea {#idea}

Imagine you could repeat your whole measurement a million times, each time with a different, but plausible, value of every input: a reading a little higher, a temperature a little lower, a standard slightly heavier. You would get a million results, and their spread would show directly how uncertain the result is.

That is exactly what the **Monte Carlo method** (JCGM 101) does, with a computer:

1. For each input, draw a random value from its distribution (normal, rectangular, …).
2. Calculate the result with the model.
3. Repeat this many times (typically 10⁵ to 10⁶ times).
4. The mean of all results is the estimate, their standard deviation is the standard uncertainty, and the interval that contains 95 % of them is the coverage interval.

No derivatives, no sensitivity coefficients and no assumption that the result is normally distributed are needed.

## Why have two methods? {#why}

The GUM method (chapter [6](06-reading-results)) is fast and gives a budget that shows where the uncertainty comes from. But it relies on two simplifications:

- the model is treated as **linear** around the estimates;
- the result is treated as **normal** (or t), so that U = k · u_c covers 95 %.

When these hold, both methods agree. When they do not, the Monte Carlo result is the more reliable one. The program runs both and tells you whether they agree (*validation*). In practice: use the GUM budget to understand and improve the measurement, and the Monte Carlo result to check it.

## When do the two methods disagree? {#disagree}

| Situation | What happens | Example to open |
|---|---|---|
| **One rectangular input dominates** | the result is nearly rectangular; the 95 % interval is narrower than 2u | [Additive model with a dominant rectangular input](example:jcgm101-9.2.4-additive-dominant-rectangular) |
| **Strongly nonlinear model** | the first-order GUM misses part of the uncertainty, or the estimate is shifted | [Comparison loss, x₁ = 0](example:jcgm101-9.4-comparison-loss-x000) |
| **Few degrees of freedom** | the two methods use different input variances (see below) | the table of chapter 4 |
| **Result near a physical limit** | the distribution is cut off and asymmetric | [Benzoic acid near 1 g/g](example:bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od) |
| **Asymmetric input** (beta, lognormal) | the interval is not symmetric about the estimate | [Purity as a beta distribution](example:iupac2024-5c-benzoic-acid-purity-beta) |

## The table of chapter 4 {#table}

In the table example, the GUM standard uncertainty was 0.560 mm but Monte Carlo gave about 0.67 mm, and the validation failed. Nothing is wrong. The five readings are a Type A input with only ν = 4 degrees of freedom:

- the GUM uses u = s/√n = 0.374 mm for the readings, and accounts for the few degrees of freedom later, through the coverage factor;
- the Monte Carlo method of JCGM 101 draws the readings' input from a t distribution with 4 degrees of freedom, whose standard deviation is larger: √(4/2) × 0.374 = 0.529 mm.

So the two methods describe the same knowledge in different places. With 20 readings instead of 5, the difference almost disappears. The report explains this in a note whenever an input has few degrees of freedom.

## Settings {#settings}

In **Settings**:

- **Run the Monte Carlo method**: switch it on or off.
- **Adaptive number of trials (JCGM 101 7.9)**: the program keeps drawing until the results are stable to the requested number of significant digits (n_dig). This is the recommended choice.
- **Number of trials**: a fixed number instead, for example 1 000 000.
- **Random-number seed**: the starting point of the random numbers. With the same seed you get exactly the same result again, which is important for reproducibility and for validation reports.
- **Coverage interval**: *shortest* or *probabilistically symmetric*.

> **Tip:** Monte Carlo results change slightly with the seed and the number of trials. This is normal sampling noise. A difference in the third significant digit between two runs is not a problem; a difference in the first digit is.

## Summary {#summary}

- The GUM method gives the budget; the Monte Carlo method checks it.
- Both are accepted by the JCGM documents and by ISO/IEC 17025.
- If they disagree, find the reason in the table above before you report.
