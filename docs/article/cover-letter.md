Dear Editor,

Please consider my manuscript **"KAPSAM: open-source software for measurement uncertainty, verified against the
worked examples of the guidance it implements"** for publication in *Measurement* as a full-length research article.

Accredited testing and calibration laboratories learn and check their uncertainty evaluations on the worked examples
of the GUM suite and of the sector guides (Eurachem/CITAC, Nordtest, EA, UKAS, EURAMET, ISO). I recomputed these
examples systematically: 284 projects, 281 of them built from the worked examples of 36 sources, 25 of which are
guidance documents and standards, with 1841 reference values that the software reproduces within documented tolerances at every change of
its code. The work contributes to measurement science in two ways:

(a) **The worked examples themselves.** The recomputation produced 102 findings. Eleven are confirmed inconsistencies
that change a final printed result (an estimate or its uncertainty) in documents that laboratories use every day,
five of them by more than a tenth of the expanded uncertainty; a metrologist who had not seen the code reproduced them
from the documents, with the help of an AI assistant, as the manuscript states (Section 8), and the derivation of each
is given in the supplementary material (S8) so that a reader can repeat it by hand. Before submission I checked the
errata, corrigenda and amendments of every document concerned (S8). I propose that issuing bodies publish the inputs of their examples in machine-readable form and
recompute them before publication, and I offer the library for that purpose.

(b) **The verification of measurement software.** The manuscript shows how uncertainty software can be verified for
ISO/IEC 17025 (7.11.2, 7.11.6): reference values with documented tolerances and a stated origin, a test of the
tolerances with random seeds not used to set them, a classification of discrepancies, an independent program (Suncal, 164
projects), a recalculation of 187 projects with ordinary spreadsheet formulas, and a validation report that a
laboratory can apply to its own projects.

The software is the instrument of this work, not its sole subject: the conclusions concern the correctness of
uncertainty evaluations and of the documents they rely on, which is why I submit to *Measurement* rather than to a
software journal.

The manuscript meets the requirements that the journal sets for its scope:

- *A contribution to measurement science:* methods and evidence on the evaluation of measurement uncertainty and on
  the verification of uncertainty software, not measurement results of another field.
- *Metrological terms:* terms are used as defined in the VIM and the GUM, which are cited where the terms are
  introduced (Section 2.1); the evaluation methods are those of the GUM suite and of the cited standards.
- *The state of the art:* Section 1 reviews the existing software and earlier work on its verification (the PTB case
  study, the GUM test cases of Greif and Schrepf, TraCIM, the Mathmet survey) and states how the work extends it: in
  scale (25 guidance documents and standards rather than the GUM alone), as a regression test at every change of the
  code, and with a classification of what the recomputation found in the examples.

The research paper has 27 pages with its figures, tables and references (the limit is 30). As the journal uses double
anonymized review, the manuscript, the figures and the supplementary material contain no name, affiliation or link
of the author; the title page, which is a separate file, gives them. Complete anonymity is not possible for this
work: the program has a Turkish name and a Turkish interface, and its archived release is public, so that a search
for its name finds the author.

The work does not use machine learning or neural networks as a research method; all evaluations are the
deterministic and Monte Carlo methods of the GUM suite and of the cited standards. An AI coding assistant was used to
write code, to draft and review the text, and the colleague who recomputed the c1 findings used one as well. This is stated in Section 3 and in the declaration on generative AI, and Table 8
states which checks are independent of the program and of the assistant. Every number in the paper can be reproduced
from the public repository and its archived release v0.1.1 (https://doi.org/10.5281/zenodo.24161568),
with which every supplementary file was generated.

The findings concern published guidance documents. They will be sent to the issuing bodies as technical notes after
acceptance, so that the notes can refer to a peer-reviewed record.

The manuscript has not been published and is not under consideration elsewhere. The author declares no competing
interests.

Suggested reviewers (their e-mail addresses are entered in the submission system). Four of them are authors of papers
cited in the manuscript, and three work at institutes involved in documents with findings (the EMUE compendium was
edited at VSL and NPL; INRiM takes part in the working groups of the JCGM); I mention this so that the editor can
weigh it.

- Blair D. Hall, Measurement Standards Laboratory of New Zealand, Lower Hutt, New Zealand: developer of the GUM Tree
  Calculator; digital representation of measurement data.
- Francesca R. Pennecchi, Istituto Nazionale di Ricerca Metrologica (INRiM), Torino, Italy: overview of uncertainty
  software and training in Mathmet; uncertainty propagation in nonlinear models; risks of conformity decisions.
- Peter M. Harris, National Physical Laboratory (NPL), Teddington, United Kingdom: validation of metrology software;
  Mathmet training on measurement uncertainty.
- Katsuhiro Shirono, National Metrology Institute of Japan (NMIJ), AIST, Tsukuba, Japan: proficiency tests with
  uncertainty information; acceptance intervals in conformity assessment.
- Gertjan Kok, VSL, Delft, The Netherlands: measurement models and software for uncertainty evaluation; Monte Carlo
  methods.

Yours sincerely,

İlker Ün
TÜBİTAK National Metrology Institute (TÜBİTAK UME), 41470 Gebze, Kocaeli, Türkiye
ORCID 0000-0001-5089-4602
