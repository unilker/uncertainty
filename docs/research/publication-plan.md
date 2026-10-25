# Publication plan

## Target journals (shortlist)

Quartiles change yearly and differ between JCR and SJR categories:
**verify the current quartile (JCR and Scimago) at submission time**.

| Journal | Publisher | Fit | Article type |
|---|---|---|---|
| *Measurement* | Elsevier (IMEKO) | metrology methods and tools | full research paper (software + validation + case studies) |
| *Metrologia* | IOP / BIPM | uncertainty methodology | paper on a methodological finding (e.g. F3) |
| *Analytica Chimica Acta* / *Talanta* | Elsevier | analytical chemistry | application paper: GUM vs MCM across chemistry budgets (F4, Q4) |
| *Measurement Science and Technology* | IOP | metrology software | software / methods paper |
| *SoftwareX* | Elsevier | research software | short software paper (requires OSI licence — satisfied by GPL-3.0) |
| *Accreditation and Quality Assurance* | Springer | lab practice, ISO 17025 | practitioner-oriented paper |

Strategy: one main paper in a Q1 metrology/analytical journal (software +
validation + chemistry case studies), optionally a short methodological note
(F3) and a software-journal paper. Avoid salami publication; each paper must
have a distinct contribution.

## Working title

*An open, validated cross-platform tool for GUM and Monte Carlo evaluation of
measurement uncertainty in analytical chemistry and calibration, combining
bottom-up and top-down approaches.*

## Contribution claims (each must be supported before submission)

1. Implementation faithful to JCGM 100/101, every algorithm cited to the
   clause, no external numerical dependencies.
2. Public validation matrix: all GUM/JCGM 101/EURACHEM reference examples
   reproduced, with high-accuracy references and measured MC dispersion.
3. Bottom-up and top-down approaches in one workflow for chemistry.
4. Validated example library with explanations (TR/EN) — educational value.
5. Native desktop + mobile, offline, reproducible (seeded) results.
6. Methodological findings from the validation (F1–F4), if novel.

## Manuscript outline

1. Introduction — need for uncertainty evaluation (ISO/IEC 17025), GUM vs MCM,
   bottom-up vs top-down, gaps in existing tools (literature-review.md §3).
2. Methods — mathematical basis (LPU, higher order, W–S, MCM, adaptive
   procedure, validation of GUF), numerical implementation (RNG, special
   functions, symbolic differentiation), software architecture.
3. Validation — reference examples, reference-value methodology, dispersion
   study, comparison with NUM and Suncal.
4. Case studies — chemistry (EURACHEM examples, top-down data), calibration.
5. Discussion — when GUF suffices, interval choice, limitations.
6. Conclusions; availability (repository, DOI via Zenodo, licence).

## Reproducibility package

- Tagged release with Zenodo DOI; `CITATION.cff`.
- `cargo test` reproduces every number in the validation tables.
- `validation/reference_values.py` reproduces the reference values.
- `core/examples/mcm_dispersion.rs` reproduces the dispersion table.

## Before submission (checklist)

- [ ] Author metadata (name, ORCID, affiliation) — Q5
- [ ] Systematic literature search (literature-review.md §4)
- [ ] All "to verify" references checked
- [ ] Independent review by a metrologist (e.g. NMI / accredited laboratory)
- [ ] Benchmarks against NUM and Suncal
- [ ] Permissions for any reproduced example material
