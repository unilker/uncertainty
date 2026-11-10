# 12. Common mistakes and questions

## The ten most common mistakes {#mistakes}

1. **Entering U as a standard uncertainty.** A certificate says "U = 0.6 mm (k = 2)". Entering 0.6 as the standard uncertainty doubles that contribution. Enter U and k in their own fields, or divide by k yourself.
2. **Using the standard deviation of single readings for a mean.** The standard uncertainty of the mean of n readings is s/√n. Conversely, if the reported result is a single reading, s (not s/√n) applies.
3. **Counting an effect twice.** The scatter of a control chart already contains repeatability, the effect of different operators and of recalibrations. Adding them again as separate terms counts them twice.
4. **Forgetting an effect because its correction is zero.** A temperature correction of 0 °C still has an uncertainty. Every effect stays in the model, even with a zero best value.
5. **Mixing units.** mL and L, mg and g, % and fraction, °C and K in a temperature *difference* (the same) and in an absolute temperature (not the same). Check the estimate of the result first: if it is wrong, a unit is usually the cause.
6. **Ignoring correlation.** Two quantities that come from the same standard, the same balance or the same stock solution are correlated. Either enter the correlation or write the shared quantity into the model.
7. **Choosing a normal distribution for a tolerance.** A tolerance "± 0.1 mL" with no other information is rectangular (u = 0.058 mL), not normal with u = 0.1 mL.
8. **Rounding too early.** Rounded intermediate values can change the last digit of the result. Enter values with all their digits and let the program round only the reported result.
9. **Using k = 2 with very few degrees of freedom.** With only 3 or 4 readings dominating the budget, k = 2 covers clearly less than 95 %. Choose **Coverage probability (k from t)** in **Settings**.
10. **Reporting too many digits.** "U = 0.123 456 mg" is not more precise than "U = 0.12 mg"; it only looks so.

## Questions {#questions}

**Do I need the Monte Carlo method?**
Not always. Keep it switched on: if the program says the GUM result is *validated*, the GUM result is fine. If not, look at chapter [7](07-monte-carlo#disagree).

**What coverage probability should I use?**
About 95 % (k = 2) is the international convention for calibration and testing. Some fields use 99 % (k ≈ 2.6) or require a stated probability; the program accepts any.

**My Type B input has no degrees of freedom. Is that a problem?**
No. A Type B value is normally considered reliable (ν = ∞). If you doubt the value itself, for example "u is known to about 25 %", you may give ν ≈ 8 in the **Degrees of freedom** field of the input (GUM G.4.2: ν ≈ ½ (Δu/u)⁻²).

**Two laboratories got different uncertainties for the same method. Who is right?**
Possibly both. Uncertainty depends on the instruments, the standards, the environment and the number of readings of each laboratory. A proficiency test (chapter [9](09-laboratory-data#proficiency)) with ζ or E_n scores shows whether the uncertainties are realistic.

**Can the uncertainty be smaller than the scatter of my readings?**
The uncertainty of a *mean* can be smaller than the scatter of single readings (s/√n). The uncertainty of a *single* reading cannot be smaller than the scatter of single readings.

**How do I enter a quantity that is the result of another evaluation?**
Use the reported value and its standard uncertainty (or U and k) as a normal input. If several results of the same earlier evaluation enter the model, they are correlated: enter that, or combine both evaluations into one model.

**What is "dark uncertainty"?**
The part of the disagreement between laboratories that their stated uncertainties do not explain. Consensus procedures (chapter [10](10-special-evaluations#consensus)) estimate it.

**Can I use the program for my accreditation?**
The program implements the methods of the cited guides and standards and is validated against 286 published examples. Your laboratory still has to validate its use for its own methods (ISO/IEC 17025 7.11): keep your own typical projects with expected values and rerun them after each update (chapter [11](11-reporting#validation)).

**Where are my projects stored?**
Wherever you save them: **Save** writes a `.json` file. On mobile devices, projects are kept in the documents folder of the app and can be shared.

**I found a strange result in a library example.**
Read the **Notes** on the **Overview** tab first: several published examples contain rounding or printing errors, which are documented there. If it is not explained, please report it to the author of the program.

## Where to go from here {#next}

- Chapter [13](13-learning-with-examples): all examples of the library by level, with what each group teaches.
- The reference guide in the `docs/guide` folder of the source code: every method in full detail, with the clauses of the standards.
