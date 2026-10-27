# Raw data of the October 2026 literature search

This folder holds the evidence for `../literature-search-2026-10.md`: the queries, the records retrieved and
the screening.

- All records are Crossref metadata, which is public (CC0). No full text is included.
- The scripts are those that were run on 2026-10-27, from this folder.

| File | Content |
|---|---|
| `cr.py`, `cr2.py`, `cr3.py` | Crossref queries: `python3 cr.py <tag> "<query>" <from-date>` writes `cr_<tag>.jsonl` (topic queries, 100 rows, journal articles); `cr2.py` and `cr3.py` write the keyword (`k_<tag>.jsonl`) and targeted queries |
| `cr_*.jsonl` | topic queries (18 themes: software, GUM vs MCM, adaptive MCM, multivariate, Bayesian, top-down, bias, sampling, conformity, weighing, QC, PT, calibration, …) |
| `k_*.jsonl` | keyword queries per theme |
| `q1.jsonl`, `all.jsonl` | first trial query; all records merged and deduplicated |
| `screen.txt` | title screen: `theme/year/citations/venue/title/DOI` of the 653 records kept after the keyword screen and deduplication |
| `targeted.txt` | targeted queries (named tools, Suncal, METAS UncLib, NIST UM, GUM-6, robust ANOVA, …) |
| `sel.tsv` | the 74 included references: theme (A–I), DOI, relevance |
| `sel_meta.json`, `sel_meta2.json`, `sel_meta.txt` | Crossref metadata of the included references (authors, venue, year, citations, licence) |
| `abs.py`, `links.py` | abstracts and full-text links of selected DOIs |
| `oa.py`, `oa1.json` | OpenAlex attempt (rate-limited, HTTP 429) |
| `table.md`, `chat.json` | the reference table of the search report and the link list given to the author |

Counts reported in the search report:

- 1080 records from the topic queries;
- 653 after the keyword screen and deduplication;
- 507 from the targeted queries;
- 74 included.

Scopus and Web of Science were not reachable from the cloud environment. The search has to be repeated there
from the institutional network before submission.
