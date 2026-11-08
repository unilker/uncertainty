# Article outline (manuscript skeleton)

Status: draft skeleton, 2026-11-07. It supersedes the outline in `docs/research/publication-plan.md` (kept for the
target-journal table and the checklist). Every number below comes from `docs/article/numbers.md`
(`tools/research/article_stats.py`), the validation matrix or the research log; nothing is to be typed in by hand.

**Türkçe özet.** Tek bir ana Q1 makale önerilir: yazılım, doğrulama ve yayımlanmış örneklerin denetimi bir arada.
Ana iddia: 38 kılavuz ve makaleden 284 proje (çözümlü örnekler ve varyantları), girdilerinden program tarafından
yeniden hesaplanır ve her çalıştırmada test edilir; bu süreçte 98 bulgu belgelenmiştir; bunların 12'si nihai basılı
sonucu değiştiren doğrulanmış tutarsızlıktır (c1; D84). İkinci, kısa bir yöntem notu (uzlaşı değerinde yöntem seçiminin etkisi ve MCMC'siz kesin ızgara)
isteğe bağlıdır. Gönderimden önce: NUM/Suncal karşılaştırması, bağımsız inceleme, bulgu sınıflarının denetimi.

## 1. Paper and journal

**One main paper**, software + validation + audit, in a Q1 journal:

| Option | Why | Risk |
|---|---|---|
| *Measurement* (Elsevier, IMEKO) | software, validation and case studies across fields are in scope | long review queue |
| *Metrologia* (IOP/BIPM) | the audit of guidance examples and the consensus results speak to its readers | software papers are rarer; needs a methodological core |
| *Talanta* / *Analytica Chimica Acta* | chemistry is the largest field of the library (qNMR, top-down, sampling) | must be framed as analytical chemistry |

Recommendation: *Measurement*, with the audit as the second main result. Verify the current JCR and Scimago
quartiles at submission. Optional spin-off (only if it has its own contribution, no salami): a short *Metrologia*
note on consensus procedures (Section 6.3 below).

Working title options:

1. *Recomputing the worked examples of measurement-uncertainty guidance: an open, validated tool and what 277
   examples reveal*
2. *An open, cross-platform implementation of the GUM suite validated on 277 published examples*

## 2. Abstract (draft, ~230 words)

> Laboratories accredited to ISO/IEC 17025 evaluate measurement uncertainty with spreadsheets or with tools that
> cover one part of the GUM suite. We present an open-source program (Rust engine; desktop and mobile
> application; Turkish and English reports) that implements the GUM uncertainty framework with the 2026
> amendment, the Monte Carlo methods of JCGM 101 and 102, conformity assessment (JCGM 106), top-down evaluation
> from validation and quality-control data, proficiency-testing statistics (ISO 13528), calibration with
> uncertainty in both variables (ISO 6143), interlaboratory precision (ISO 5725-2), control charts (Nordtest TR
> 569), uncertainty from sampling, weighing instruments (EURAMET cg-18) and consensus values with degrees of
> equivalence. Each algorithm cites the clause it implements. The program is validated on [277] worked examples
> from [36] guidance documents and papers, stored as project files that the test suite recomputes from their
> inputs at every change. Reproducing the examples produced [98] findings: intermediate rounding, misprints,
> and [12] confirmed inconsistencies that change a final printed result (c1), besides inconsistencies in
> intermediate values, differences of method and published errata, which are counted separately. Bayesian random-effects consensus values that the literature obtains by MCMC are
> reproduced exactly on a deterministic grid. We argue that worked examples should be published with machine-
> readable inputs and checked by independent recomputation, and we make the library available for that purpose.

(Numbers in brackets are generated in `numbers.md`; classes from `docs/research/finding-classes.tsv`, D73.)

## 3. Introduction

Messages and sources:

1. ISO/IEC 17025 7.6 requires uncertainty evaluation; laboratories rely on worked examples in guidance documents
   to learn and to validate their own budgets (`iso17025-mapping.md`).
2. The GUM suite is growing (GUM-1:2023, GUM-6:2020, GUM-5:2026, JCGM 100 Amd.1:2026, revised definitions,
   D56, D61); chemistry uses Eurachem/CITAC guides, calibration EA-4/02 and M3003, PT ISO 13528.
3. Existing tools cover parts (NUM, Suncal, GUM Workbench, Metrodata, uncertainty toolboxes; NICOB for consensus):
   `literature-review.md` §3. Gap: no open tool that covers bottom-up, top-down, PT, calibration, conformity and
   consensus with traceable validation.
4. Worked examples are rarely recomputed independently; the few audits (cite from the literature review) are
   limited to single documents.
5. Contributions (each supported in §5–§7).

## 4. Methods (software)

- 4.1 Measurement models: expression parser with symbolic differentiation; first- and second-order LPU, the
  Amd.1:2026 second-order term (D56, D57); Welch–Satterthwaite with ensembles (D58, Willink 2007).
- 4.2 Monte Carlo: Wichmann–Hill generator checked against its primary source (task 21), adaptive procedure,
  symmetric/shortest intervals, the half-width about the estimate (D61), validation of the GUF (JCGM 101 8).
- 4.3 Modules: top-down (Nordtest, MUPR), ISO 13528 incl. the Bayesian between-unit model (D60), ISO 6143 GLS,
  ISO 5725-2, JCGM 102, TR 569, UfS, cg-18, JCGM 106 incl. lognormal processes and multicomponent risks (D59,
  D63), consensus values (D66–D68).
- 4.4 Deterministic Bayesian evaluations on grids instead of MCMC: half-Cauchy priors via τ = s·tan(πθ/2), μ
  integrated analytically, Laplace effects by the closed-form normal–Laplace convolution (D60, D67, D68).
- 4.5 Architecture and reproducibility: project files (JSON), reports TR/EN (Markdown, HTML with SVG, JSON),
  self-checking expected values, generators that rebuild every project from cited data, CI; every report carries
  the commit and the SHA-256 of the project (D79).
- 4.6 Use of AI tools (required by most journals; state it per the journal's policy): the code, the generators,
  the recomputations and a pre-review were produced with an AI coding assistant under the author's direction; the
  expected values come from the published documents, and the independent checks are Suncal, the NUM manual and
  the independent recomputation by a metrologist (§9). Common-mode errors (a standard read the same wrong way by
  the engine and by the Python checks written in the same environment) are a stated limitation (review item 14).

Figure 1: architecture (engine, project format, CLI, app, library, generators, tests).
Table 1: features × standard clauses (generate from `iso17025-mapping.md` and the module docs).

## 5. Validation

- 5.1 The library: 277 projects, by section and field (Table 2 from `numbers.md`); 36 source documents (Table 3).
- 5.2 How expected values are set: printed values with a tolerance of half a unit in the last digit; where the
  printed value is a Monte Carlo/MCMC estimate, its sampling error is added (D66–D68, F95, F96); independent
  recomputation (numpy/scipy scripts in `tools/research/`) where nothing is printed.
- 5.3 Monte Carlo dispersion study (σ over 20 seeds; validation-matrix "Measured MCM dispersion").
- 5.4 Benchmarks: Suncal 1.7.1 on 107 model projects (D74, `docs/research/benchmark-suncal.md`): first-order u
  agrees to 6 × 10⁻¹²; Monte Carlo s.d. within sampling error (48 projects, max |z| 3.0). NUM: the manual's
  examples (task 22; F29, F33, F35).
- 5.5 Validation matrix summary: 222 rows ✅, 23 of them with a finding.

## 6. Results

### 6.1 What recomputation reveals (the audit)

Classification (D12, D73): (a) intermediate rounding; (b) misprint or transcription without effect; (c)
inconsistency; (n) no discrepancy (methodological note, summary, full reproduction). The class-c findings are
split by effect (D84, review 2026-11-08 item 7; `class-c-impact.tsv`): c1 a final printed y, U or outcome is not
reproduced at its printed digits (12 confirmed); c2 only intermediate values (7); m a difference of method or
interpretation, not an inconsistency (5); e corrected by a published erratum (1, QUAM A5); 4 open findings are
not counted. Each row gives printed → recomputed and |Δy|/U, |ΔU|/U. Figure 2: findings per document by class
(table "Findings by document and class" in `numbers.md`). Table "From sources to findings" in `numbers.md` is
the counting chain (sources → projects → matrix rows → compared values → findings), with each step defined.

Selected c1 findings for Table 4 (check each against the current edition and errata before submission; F14 is a
known erratum and F79 a method difference, D84):

| F | Document | Finding |
|---|---|---|
| F21 | EA-4/02 S13 | printed diameter not reproducible from the data |
| F28 | NIST TN 1900 E4 | results inconsistent with the stated equation by √2 |
| F39 | ISO 21748 C.3 | flour example |
| F40 | ISO 5725-3:2023 J.2 | Table J.5 / transcription errors in Table J.2 (D41) |
| F41, F42 | ISO 6143:2025 | critical value of the drift test; exponential example |
| F56–F58 | UKAS M3003 K.3, K.5, K.6 | mismatch term, sign, temperature terms |
| F63 | Eurachem UfS:2019 | robust ANOVA not reproducible from the Guide |
| F82, F85 | GUM-5 Ex. 4, 11 | u(y_CO₂) of Ex. 4; reported result of Ex. 11 (F85 open: data read from the figure) |
| F90 | EMUE E3.3 | covariance of the reference values |
| F92 | IUPAC 2024 4I | sign of the endpoint correction |

Patterns worth a paragraph each:

- Intermediate rounding is ubiquitous and harmless (F1, F9, F11, F26, F54, F93, F94) but makes examples
  non-reproducible to the printed digits; examples should state unrounded inputs.
- Spreadsheet budgets contradict themselves (F97, BIPM ISRD qNMR: components ≠ c·u, ν fixed at 4, a printed
  purity not reproduced by its own inputs); recomputation from inputs catches this immediately.
- Monte Carlo and MCMC results in documents are estimates with sampling error (F2, F77, F95, F96); a deterministic
  evaluation removes the ambiguity (D67, D68).
- Methodological points: two conventions for t-distributed inputs (F31, F35, D85); W–S with correlated inputs
  (F79, D58: both readings cover 93–94 % when ν is recomputed per data set; coverage note in the F79 entry); shortest
  intervals by MCM (F3); second-order terms (F19, D56). F81 (getmode returns the first MCMC draw) and F82 (Table
  4.2 vs 4.1) go to a short appendix with the code and the computation (review item 17).
- Tolerances: the basis of every tolerance (half a unit, one unit, Monte Carlo 5σ, wider with a reason) and the
  distribution of |Δ|/tolerance from `numbers.md`, instead of pass/fail alone (review item 13).

### 6.2 Case studies (short)

- Chemistry: QUAM A1–A7, qNMR purity (BIPM ISRD, UME procedure 19.2), top-down (Nordtest, MUPR), sampling (UfS).
- Calibration: EA-4/02, M3003, cg-18 weighing instruments; DMM (UME procedure 19.1).
- Conformity: JCGM 106, GUM-5 Ex. 10 lognormal process, EMUE E2.1 multicomponent total risks.

### 6.3 Consensus values: the procedure is part of the model

On the 16 values of G (Merkatas et al. 2019) the program reproduces DL with Knapp–Hartung (6.673 99, u 0.000 25),
ML (6.673 90, 0.000 25), the NICOB hierarchical Bayes (6.673 89, 0.000 27; exact grid instead of MCMC) and the
Linear Pool (6.673 77, 0.001 17); the published fourteen procedures span 6.673 77–6.674 29 with u from 0.000 11
to 0.001 17. Laplace effects reproduce the Stan result for H₀ (Possolo & Meija 2022). Figure 3: G by procedure
with intervals. CCQM-K145 as a metrology example, participants anonymised and the official KCRV/DoE of the final report cited
(illustrative only, D66; review item 19).

## 7. Discussion

- When the GUM framework suffices (GUF validation results), interval choice (D61), asymmetric/bounded results
  (F97 item 6; outlook).
- Machine-readable worked examples as a recommendation to guideline authors: the project format as a proposal.
- Bounded outputs (D87): truncated GUM interval (Westwood et al. 2023) against the Monte Carlo result restricted to
  the bound (rectangular prior, Toman et al. 2016), ISRD-06 T6 as the case (estimate 0.999 77 vs 0.999 32 g/g).
- Limitations: methods not implemented, in the order of their importance for an NMI (review item 20): NICOB σ_j priors with finite ν (D68) and the Bayesian and Linear Pool degrees of equivalence (the
  DerSimonian–Laird leave-one-out and bilateral ones are implemented, D86), PDF
  output and signed packages (7.11.3); then ARMA, counting data, Easterling's methods and model averaging (D70);
  licensed standards (data only, `sources-provenance.md`).

## 8. Availability

Repository, licence (GPL-3.0), tagged release with Zenodo DOI, `CITATION.cff`, the library and generators,
`cargo test` reproducing every number; app builds for Windows, Linux, macOS, Android, iOS. State the commit that
was reviewed and the commit of the release (the pre-review used 809ebcf; review item 22).

## 9. Before submission (open)

- [x] D12 audit of the finding classes (D73): every finding has a class in `finding-classes.tsv`.
- [ ] Recheck the class-c findings against current editions and errata (F40, resolved in D41, is the model).
- [x] Benchmark against Suncal (D74). NUM: manual examples only (web service).
- [ ] Independent review by a metrologist outside the project (NMI or accredited laboratory). The pre-review of
      2026-11-08 (`docs/research/review-2026-11-08.md`) was made with an AI assistant and does not count; at least
      ten c1 findings to be recomputed by a person who has not seen the code (review item 14).
- [ ] Systematic literature search finished (task 41; `literature-search-2026-10.md`) and audits of worked
      examples by others cited.
- [ ] Permissions: data-only use of licensed documents confirmed (`sources-provenance.md`); figures redrawn.
- [ ] F79 against Willink 2007; F92 against Meija et al.; F29 still "to be confirmed"; F28 against Kline &
      McClintock (1953); Dobbert 2008 and the Z540.3 handbook for the TUR guard bands (D80).
- [ ] Figures 1–3 and Tables 1–4 generated by scripts in `tools/research/`.
- [ ] Decide whether findings are reported to the issuing bodies (current rule: recorded for the article only). The
      pre-review recommends sending the c1 findings to JCGM-WG1, NIST, UKAS, Eurachem and the ISO committees as
      technical notes before or with submission, and the F97 question to the BIPM qNMR team, and citing their
      replies (review items 8, 18). Decision of the author.
- [ ] Errata of M3003 ed. 6, ISO 5725-3:2023, ISO 6143:2025, EA-4/02 M:2022, EMUE, TN 1900 and GUM-5:2026 checked
      for every c1 finding, with the edition and date in the project note (review item 6; QUAM and UfS done).
