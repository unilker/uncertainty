# 2. Getting started

## 2.1 Installation

The computation core and the command-line tool are written in Rust. Until binary releases are available, build
from source (Rust 1.75 or later):

```sh
git clone https://github.com/unilker/uncertainty
cd uncertainty
cargo build --release
# the tool is target/release/kapsam
```

`cargo test` runs the validation suite, including every example of the library.

## 2.2 The command-line tool

| Command | Purpose |
|---|---|
| `kapsam run <project.json>` | evaluate a project and write a report |
| `kapsam check <files or directories>` | evaluate projects and compare them with their stored reference values (exit status 1 on failure) |
| `kapsam template [--topdown \| --proficiency \| --calibration \| --conformity \| --precision \| --multivariate \| --control-chart]` | print a minimal project file to start from |
| `kapsam catalogue <files or directories>` | write the worked-examples chapter of this guide |
| `kapsam --version`, `--help` | version, citation and help |

Options of `run`:

| Option | Meaning |
|---|---|
| `--lang en` / `--lang tr` | report language (English is the default) |
| `--format md`, `html`, `json`, `pdf` | Markdown, self-contained HTML with charts, machine-readable results, or a PDF document (A4, without the charts); by default taken from the extension of `--output`, otherwise Markdown |
| `--output FILE` (`-o`) | write the report to a file |
| `--no-mcm` | skip the Monte Carlo evaluation |
| `--trials N` | fixed number of Monte Carlo trials (overrides the project) |
| `--seed S` | random-number seed (overrides the project) |

Options of `check`:

| Option | Meaning |
|---|---|
| `--no-mcm` | skip the Monte Carlo evaluations and their reference values |
| `--lab PATH` | the laboratory's own projects (file or directory), reported in a section of their own |
| `--report FILE` | write a software validation report: PDF if FILE ends in `.pdf`, HTML for `.html`, otherwise Markdown |
| `--lang en` / `--lang tr` | language of the validation report |

The validation report is the record that ISO/IEC 17025:2017 7.11.2 asks for when software is validated before
use and after changes. It identifies the software by version, git commit and commit date ("-modified" if the build
had uncommitted changes), compiler and platform, gives the date and options and the SHA-256 digest of the whole set of
checked files; summarizes the outcome by method; lists every project with its source, the number of values compared,
the largest ratio of difference to tolerance and the SHA-256 of the file (which holds its inputs, reference values and
settings such as seeds, numbers of trials and rounding); and lists any value outside its tolerance. The laboratory's
own projects, given with `--lab`, are listed in a section of their own. It ends with a line for the signature of
the person who checked it. A laboratory would typically run

```sh
kapsam check library --lab my-projects --report validation-2026-10.html
```

after installing a new version, and file the report. Every calculation report also ends with the software identity
and the SHA-256 of the project as evaluated, so that the technical record (7.5) ties a result to its inputs and to the
validated build. The library examples verify the implementation against published values; the laboratory's own
projects, with reference values it has set, validate the software for its scope.

## 2.3 A first evaluation

The library example `library/chemistry/quam-a1-cadmium-standard.json` is EURACHEM/CITAC QUAM Example A1, the
preparation of a cadmium calibration standard:

```sh
kapsam run library/chemistry/quam-a1-cadmium-standard.json -o a1.html
kapsam run library/chemistry/quam-a1-cadmium-standard.json --lang tr -o a1-tr.html
```

The report contains, in this order:

1. **Measurement model** — the formula, c_Cd = 1000 · m · P / V, with any intermediate quantities.
2. **Input quantities** — for each input its unit, the probability distribution assigned to it and the
   information it was derived from (e.g. "0.9999 ± 0.0001", rectangular).
3. **Uncertainty budget** — estimate, standard uncertainty, degrees of freedom, sensitivity coefficient,
   contribution |cᵢ|u(xᵢ) and share of the combined variance of every input.
4. **Result** — estimate, combined standard uncertainty, effective degrees of freedom, coverage factor,
   coverage probability, expanded uncertainty and the reported result with its statement.
5. **Charts** (HTML) — contributions to the combined variance; the Monte Carlo histogram with the GUM
   probability density and both coverage intervals.
6. **Monte Carlo method** — number of trials, seed, estimate, standard deviation, coverage intervals and the
   validation of the GUM result (JCGM 101 clause 8).
7. **Explanation and notes** — the step-by-step explanation stored in the project, and notes on the source.
8. **Comparison with reference values** — for library examples, each stored reference value with its tolerance.
9. **Sources, methods and citation.**

## 2.4 Your own project

```sh
kapsam template > my-project.json            # measurement model
kapsam template --topdown > my-topdown.json  # top-down from validation data
```

Edit the file (any text editor; the format is described in [chapter 7](07-project-files.md)) and run it. A
misspelt field name or an inconsistent specification (for example both `sd` and `expanded` for one input) is
reported as an error rather than silently ignored.

To keep reference values with a project — for example the results of a validation study — add an `expected`
section; `kapsam check` then verifies the project whenever the program or the file changes.
