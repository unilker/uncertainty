Dear Editor,

Please consider our manuscript **"KAPSAM: open-source software for measurement uncertainty, validated against the
worked examples of the guidance it implements"** for publication in *Measurement* as a full-length research
article.

Accredited testing and calibration laboratories learn and validate their uncertainty evaluations on the worked
examples of the GUM suite and of the sector guides (Eurachem/CITAC, Nordtest, EA, UKAS, EURAMET, ISO). We
recomputed these examples systematically: 286 projects built from the worked examples of 38 documents, with 1865
printed values that the software reproduces within documented tolerances at every change of its code. The work
contributes to measurement science in three ways:

1. **The worked examples themselves.** The recomputation produced 100 findings, among them 13 confirmed
   inconsistencies that change a final printed result (an estimate, its uncertainty or a stated decision) in
   documents that laboratories use every day. We propose that issuing bodies publish the inputs of their examples
   in machine-readable form and recompute them before publication, and we offer the library for that purpose.
2. **Choices that change a result.** For a key comparison, the choice of the consensus procedure changes the
   degrees of equivalence as much as the reference value; results near a physical bound and guard bands from the
   test uncertainty ratio are treated in the same quantitative way.
3. **Validation of measurement software.** We describe how uncertainty software can be validated for ISO/IEC 17025
   (7.11.2): reference values with tolerances derived from the printed digits, a classification of discrepancies
   fixed before the audit, an independent program (Suncal, 171 projects), a recalculation of 189 projects with
   ordinary spreadsheet formulas, and a validation report and spreadsheet workbooks that a laboratory can apply to
   its own projects.

The software is the instrument of this work, not its sole subject: the conclusions concern the correctness of
uncertainty evaluations and of the documents they rely on, which is why we submit to *Measurement* rather than to a
software journal.

The manuscript meets the requirements that the journal sets for its scope:

1. **A contribution to measurement science.** It contributes methods and evidence on the evaluation of measurement
   uncertainty and on the validation of uncertainty software (reference values with tolerances derived from the
   printed digits, a classification of discrepancies fixed before the audit, independent recomputations), not
   measurement results of another field.
2. **Metrological terms.** Terms are used as defined in the VIM and the GUM, which are cited where the terms are
   introduced (Section 2.1); the evaluation methods are those of the GUM suite and of the cited standards.
3. **The state of the art.** Section 1 reviews the guidance documents and the existing software (the NIST
   Uncertainty Machine, Suncal, the GUM Tree Calculator, METAS UncLib, the NIST Consensus Builder, the Mathmet survey
   of 35 programs and the PTB case study of the validation of GUM software) and states how the work advances it: one
   validated program for the evaluation types the laboratories need, validated on the worked examples of 38
   documents rather than of the GUM alone, with the consequences for the examples themselves (Sections 6 and 7).

The research paper has 26 pages with its figures, tables and references. As the journal uses double anonymized
review, the manuscript, the figures and the supplementary material contain no information that identifies the
author; the title page, which is a separate file, gives the author's details and the links to the repository and
to the archived release.

The work does not use machine learning or neural networks as a research method; all evaluations are the
deterministic and Monte Carlo methods of the GUM suite and of the cited standards. An AI coding assistant was used
to write code and to draft the text. This is stated in Section 3.4 and in the declaration on generative AI, and
Table 6 states which checks are independent of the program and of the assistant. Every number in the paper can be
reproduced from the public repository and its archived release v0.1.0 (https://doi.org/10.5281/zenodo.24118312); the supplementary material
contains the library, the findings, the validation matrix, the comparison with Suncal, a software validation
report, the guide of the software and the spreadsheet recalculation.

The findings concern published guidance documents. They will be sent to the issuing bodies as technical notes after
acceptance, so that the notes can refer to a peer-reviewed record.

The manuscript has not been published and is not under consideration elsewhere. The author declares no competing
interests. [Suggested reviewers, if the journal asks for them: name, affiliation, e-mail, reason — to be added by the
author.]

Yours sincerely,

İlker Ün
TÜBİTAK National Metrology Institute (TÜBİTAK UME), 41470 Gebze, Kocaeli, Türkiye
ORCID 0000-0001-5089-4602
