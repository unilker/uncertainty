# Research scripts

Scripts used to recompute published examples or to investigate findings. They are not part of the program;
they are kept as the evidence behind entries of `docs/research/research-log.md`.

| Folder | Content | Log entries |
|---|---|---|
| `mupr-spreadsheet/` | Reading the workbooks of Bettencourt da Silva (2026), Talanta 130826, Supplementary Materials 03–08, with openpyxl: values (`sheets.py`, `sheets2.py`, `rows.py`, `labels.py`) and formulas (`cells.py`, `find.py`). The workbooks themselves are not included; they are downloadable from the publisher and Zenodo record 23043687. | D53, F71–F73 |
| `ufs-ranova/` | Attempts to reproduce the robust ANOVA (RANOVA) of Eurachem UfS:2019 Example A1 (`probe*.py`, `ranova_try.py`, `ref.py`, `t.json`) | F63 |
| `cg18/` | Check of EURAMET cg-18 Example H4 (`h4.py`) | D52, F70 |
| `gum5/` | JCGM GUM-5:2026: composition of Example 4 (`ex4_composition.py`), Monte Carlo of Example 2 (`ex2_mcm.py`), risks of Example 10 in the printed and the conditional form (`ex10_risks.py`), posterior of Example 12 (`ex12_posterior.py`), Fourier area of Example 11 (`ex11_fourier_area.py`), Bayesian slope of Example 9 (`ex9_bayes.py`) | F79–F85 |
| `emue/` | EMUE compendium 2021: tables of E3.4 (`e34_soil_tables.py`), total risks of E2.1 (`e21_total_risks.py`, scipy), vacuum-gauge tables of E2.2 (`e22_vacuum_tables.py`), permeation of E3.8 (`e38_permeation.py`), dilution and WTLS of E3.3 (`e33_dilution.py`, `e33_wtls.py`) | F87–F90, D63 |
| `consensus/` | Koepke et al. 2017 Tables 1–4: DerSimonian–Laird, Knapp–Hartung, the NICOB parametric bootstrap and the Linear Pool, independent numpy implementation (`koepke2017.py`); grid posterior of the Bayesian random-effects model (`bayes_grid.py`); Merkatas et al. 2019 (`merkatas2019.py`); Laplace effects, Hubble (`hubble_laplace.py`); lead in HIPB-1 (`lead_hipb1.py`) | D66–D68, F93–F96 |
| `qnmr/` | BIPM ISRD qNMR budgets: recomputation of the eleven ¹H tables (`isrd_budgets.py`, also the data source of `tools/generators/bipm_isrd.py`) | D69, F97 |
| `homogeneity/` | ISO Guide 35 C.1 chromium data and ANOVA (`cr_soil.py`, also the data source of `possolo_bodnar2018.py`) | D70 |
| `joinwords.py` | rejoins glyph-by-glyph PDF text from `pdftotext -bbox` (IUPAC 2024 PDF) | — |
| `findings_index.py` | writes `docs/research/findings-index.md` from the research log | — |

Usage of the spreadsheet scripts: `python3 -I sheets.py <workbook.xlsx> "<sheet>" <first row> <last row>
<last column>`. The supplements of 2026-10-27 were plain `.xlsx` files: the password `mupr2026` given in the paper
protects the sheets, not the files.
