# 11. Reporting the result

## The complete statement {#statement}

A result in a calibration certificate or test report contains:

1. the value and its unit,
2. the expanded uncertainty U, with the same unit (or relative),
3. the coverage factor k and the coverage probability,
4. if needed, how the uncertainty was evaluated (GUM, Monte Carlo, top-down).

For example:

```
m = (100.0021 ± 0.0012) g
The reported expanded uncertainty is the standard uncertainty multiplied by the
coverage factor k = 2, which for a normal distribution corresponds to a coverage
probability of approximately 95 %.
```

The program writes this sentence for you, in English or Turkish, under **Reported result**.

## How many digits? {#digits}

- Give U with **two significant digits** (sometimes one): 0.0012 g, not 0.001 234 5 g. More digits pretend a precision that the uncertainty itself does not have.
- Give the value with **the same number of decimals** as U: 100.0021 g, not 100.002 or 100.002 13.
- Never round U down too much: the default rule (EA-4/02 M:2022, 6.3) rounds normally, unless this lowers U by more than 5 %, and then rounds up. Other rules are available in **Settings → Rounding**: always up, half up, half even.
- Keep all digits during the calculation; round only the reported result. The program does this.

## Relative uncertainty and percentages {#relative}

For chemical analyses the relative expanded uncertainty is common: "c = 12.4 mg/L ± 8 %". Be careful to state whether a percentage is relative (% of the value) or absolute (for example a mass fraction in % with an uncertainty of 0.1 %).

## A result near a limit or a bound {#bounds}

- If a mass fraction is 0.9998 g/g with U = 0.0005 g/g, the interval would go beyond 1 g/g, which is impossible. The program then states an asymmetric interval, for example 0.9998 +0.0002/−0.0005 g/g, and can restrict the Monte Carlo result to the physical bound.
- If a decision on conformity is reported, the decision rule must be stated too (chapter [10](10-special-evaluations#conformity)).

## The CMC of a calibration laboratory {#cmc}

An accredited calibration laboratory may not report an uncertainty smaller than its CMC (ILAC P14). If the calculation gives less, the CMC is reported. Give your CMC as an expanded uncertainty in the project (on the **JSON** tab: `"reporting": { "cmc": { "expanded": 80 } }`) and the program applies this rule, noting it in the report. The example [ILAC P14 with a 10 kg weight](example:ilac-p14-cmc-floor-weight-10kg) shows it.

## Exporting the report {#export}

The menu ⋮ of a project offers:

- **PDF**: an A4 report with all tables, with the fonts embedded, ready to file or attach;
- **HTML**: the same report with charts, for a browser;
- **Markdown**: plain text, easy to include in other documents.

The report is written in the language selected at the top right (EN/TR). It contains the model, the inputs and their sources, the budget, the GUM and Monte Carlo results, the reported result, and at the end the program version and a digital fingerprint (SHA-256) of the project. With the fingerprint anyone can check later that a report belongs to a given project file.

## Software validation for ISO/IEC 17025 {#validation}

ISO/IEC 17025 (7.11.2) requires that software used for calculations is validated. The program helps in two ways:

- its library of 286 examples, recomputed and compared with the published values, is the validation of the methods;
- the command-line tool writes a **software validation report** (`uncertainty check library --lab my-projects --report validation.pdf`) that lists every check, the fingerprints of the files, the program version and the compiler, and puts your laboratory's own projects in a separate section.

Keep your own typical projects with their expected values: rerunning them after every update of the program is a simple, documented validation.

> **Tip:** Save each project you use for a certificate. The `.json` file, the report and its fingerprint together are the technical record of the evaluation.
