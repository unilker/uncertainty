# Research scripts

Scripts used to recompute published examples or to investigate findings. They are not part of the program;
they are kept as the evidence behind entries of `docs/research/research-log.md`.

| Folder | Content | Log entries |
|---|---|---|
| `mupr-spreadsheet/` | Reading the workbooks of Bettencourt da Silva (2026), Talanta 130826, Supplementary Materials 03–08, with openpyxl: values (`sheets.py`, `sheets2.py`, `rows.py`, `labels.py`) and formulas (`cells.py`, `find.py`). The workbooks themselves are not included; they are downloadable from the publisher and Zenodo record 23043687. | D53, F71–F73 |
| `ufs-ranova/` | Attempts to reproduce the robust ANOVA (RANOVA) of Eurachem UfS:2019 Example A1 (`probe*.py`, `ranova_try.py`, `ref.py`, `t.json`) | F63 |
| `cg18/` | Check of EURAMET cg-18 Example H4 (`h4.py`) | D52, F70 |
| `gum5/` | JCGM GUM-5:2026: composition of Example 4 (`ex4_composition.py`), Monte Carlo of Example 2 (`ex2_mcm.py`), risks of Example 10 in the printed and the conditional form (`ex10_risks.py`), posterior of Example 12 (`ex12_posterior.py`) | F79–F82 |
| `findings_index.py` | writes `docs/research/findings-index.md` from the research log | — |

Usage of the spreadsheet scripts: `python3 -I sheets.py <workbook.xlsx> "<sheet>" <first row> <last row>
<last column>`. The supplements of 2026-10-27 were plain `.xlsx` files: the password `mupr2026` given in the paper
protects the sheets, not the files.
