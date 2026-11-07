# Library generators

Python scripts that wrote the self-checking projects in `library/` from the
numerical data of the published examples, and a few cross-check scripts.
They are kept so that every number in the library can be traced and
recomputed.

- The JSON files in `library/` are the maintained source. Some projects were
  corrected by hand after generation, and those corrections are recorded in
  `docs/research/research-log.md`. Before regenerating a file, compare the
  output with the committed one.
- Run a script from the repository root, e.g.
  `python3 tools/generators/cg18inuse.py`. `gen_library.py` and
  `gen_topdown.py` take the library directory as their argument.
- Then run `python3 tools/library_index.py`, the `catalogue` commands for
  `docs/guide/{en,tr}/examples.md` and `cd app && dart run tool/bundle_library.dart`.
- The scripts need numpy and, for some, scipy.
- They contain only numerical data and results from the sources, with each
  source cited and linked. No licensed text or document is included.

| Script | Projects |
|---|---|
| `gen_library.py` | GUM, JCGM 101, QUAM A1–A7, EA-4/02 (model projects) |
| `gen_topdown.py` | Nordtest TR 537, MUPR, QUAM A4/A6 top-down projects |
| `tn1900.py`, `num.py` | NIST TN 1900 and NIST Uncertainty Machine manual |
| `iso13528.py`, `iso13528b.py` | ISO 13528:2022 proficiency testing |
| `iso21748.py`, `iso5725.py`, `iso5725_2.py`, `iso5725_2_lib.py` | ISO 21748, ISO 5725-3, ISO 5725-2 |
| `iso6143.py`, `mc6143.py`, `odr_check.py` | ISO 6143:2025 (projects and cross-checks) |
| `jcgm106.py`, `j106.py` | JCGM 106 conformity (projects and cross-check) |
| `jcgm102.py` | JCGM 102 multivariate |
| `palma2019.py`, `bettencourt2026.py` | Palma et al. 2019; Bettencourt da Silva 2026 |
| `gum5.py` | JCGM GUM-5:2026, Examples 1–5, 9, 10, 12, 14, 15 |
| `emue.py` | EMUE compendium 2021: E3.4, E2.1, E2.2, E3.8, E3.3, E6.5 |
| `iupac2024.py` | IUPAC brief guide 2024, Examples 3A, 3E/4M/4N, 4A, 4B, 4F, 4I |
| `possolo_meija2022.py` | Possolo & Meija 2022: Hubble–Lemaître constant, anion gap, storage tank, Wheatstone bridge, atomic weight of lead |
| `bipm_isrd.py` | BIPM ISRD-01, -02, -06, -07, -08: qNMR purity budgets (inputs from `tools/research/qnmr/isrd_budgets.py`) |
| `possolo_bodnar2018.py` | Possolo & Bodnar 2018 §4.4: chromium in soil, homogeneity with a hierarchical Bayesian model |
| `merkatas2019.py` | Merkatas et al. 2019: G from 16 results, four procedures of Table 4 |
| `koepke2017.py` | Koepke et al. 2017 (NIST Consensus Builder): G, half-lives of ¹³⁷Cs and ⁹⁰Sr |
| `eurachem_compliance.py` | Eurachem/CITAC compliance assessment guide 2021, Annex B Examples 1–3 |
| `tr569.py` | Nordtest TR 569 control charts |
| `m3003.py` | UKAS M3003 ed. 6 |
| `ufs.py` | Eurachem UfS:2019 |
| `cg18.py`, `cg18b.py`, `cg18h4.py`, `cg18inuse.py` | EURAMET cg-18 H1–H4, Appendix C, 7.4–7.5 and Appendix G |
| `cg19.py` | EURAMET cg-19 |
