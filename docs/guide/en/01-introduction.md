# 1. Introduction

*Uncertainty Calculations* (Turkish: *Belirsizlik Hesaplamaları*) evaluates measurement uncertainty in three ways:

| Approach | Reference documents | Typical use |
|---|---|---|
| **GUM uncertainty framework** (bottom-up, law of propagation of uncertainty) | JCGM 100:2008 (GUM); EA-4/02 M:2022; EURACHEM/CITAC QUAM:2012 | calibration, preparation of reference solutions, any measurement with an explicit model |
| **Monte Carlo method** (propagation of distributions) | JCGM 101:2008 (GUM Supplement 1) | non-linear models, dominant non-Gaussian inputs, asymmetric output distributions; validating a GUM result |
| **Top-down** (validation and quality-control data) | Nordtest TR 537 ed. 4 (2017); Eurachem/CITAC MUPR (2026); QUAM:2012 | routine chemical analysis in testing laboratories |

The program is written for analytical chemistry first, then calibration and accredited laboratories, but the
engine is not specific to any field: the example library contains cases from chemistry, mass, length,
temperature, electrical/RF and flow metrology.

## How this guide is organised

1. Introduction (this chapter)
2. [Getting started](02-getting-started.md): installation, the command-line tool, a first evaluation
3. [The GUM uncertainty framework](03-gum.md): models, standard uncertainties, sensitivity coefficients,
   combination, degrees of freedom, coverage factors
4. [The Monte Carlo method](04-monte-carlo.md): propagation of distributions, coverage intervals, validation
   of the GUM result
5. [Top-down evaluation](05-top-down.md): within-laboratory reproducibility, bias, recovery
6. [Reporting](06-reporting.md): rounding, the statement of the result, what a report contains
7. [Project file reference](07-project-files.md): every field of the JSON project format
8. [Glossary (English–Turkish)](08-glossary.md)
9. [Proficiency testing (ISO 13528)](09-proficiency-testing.md): assigned values, robust statistics, scores; consensus values of interlaboratory comparisons (NIST Consensus Builder)
10. [Calibration with uncertainties in both variables (ISO 6143)](10-calibration.md)
11. [Conformity assessment (JCGM 106)](11-conformity.md): decision rules, guard bands, risks
12. [Interlaboratory precision experiments (ISO 5725-2)](12-precision-experiments.md): outlier tests,
    repeatability and reproducibility
13. [Several output quantities (JCGM 102)](13-multivariate.md): covariance matrices, coverage regions
14. [Control charts (Nordtest TR 569)](14-control-charts.md): limits, daily rules, long-term review
15. [Uncertainty from sampling (Eurachem UfS)](15-sampling.md): duplicate method, range method, uncertainty factor,
    Gy's sampling theory
16. [Calibration of weighing instruments (EURAMET cg-18)](16-weighing.md): error of indication, substitution
    loads, error approximation function
17. [Worked examples](examples.md): all 282 library examples, with their results recomputed by the program

Chapters 3 to 5 explain the methods themselves, not only the program: they are intended as a compact guide to
uncertainty evaluation that can be read with the reference documents at hand.

## Scientific validation

Every calculation method is validated against published worked examples. The examples are recomputed from their
*inputs* (raw data wherever the documents give them), not matched to the printed results, and each comparison has
a documented tolerance — the printed rounding of the document, or about five times the measured Monte Carlo
dispersion. The 282 examples of the library are part of the test suite: if a change to the program alters any
result beyond its tolerance, the build fails.

Recomputing examples from their inputs also reveals where published examples are internally inconsistent
(rounded intermediate values, transcription errors, values that do not follow from the stated inputs). Such
cases are recorded in the *notes* of each example and in the research log of the project
(`docs/research/research-log.md`); none of them changes the method, and only a few change a printed result.

## Licence and citation

The program is free software (GPL-3.0-or-later) with additional attribution terms (GPL §7). **Any publication,
report or software that uses results of the program must cite it.** Every report the program produces ends
with the citation; see `ADDITIONAL-TERMS.md` and `CITATION.cff`.

> Ün, İ. *Uncertainty Calculations (Belirsizlik Hesaplamaları): measurement uncertainty evaluation software.*
> TÜBİTAK UME. https://github.com/unilker/uncertainty
