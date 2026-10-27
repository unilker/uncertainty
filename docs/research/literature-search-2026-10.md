# Literature search, October 2026 (open bibliographic databases)

Scope: protocol of `literature-review.md` §4, run on 2026-10-27 from the
development environment. Scopus, Web of Science and publisher full texts were
**not** accessible from there (no institutional network); the search used the
Crossref metadata API (OpenAlex was rate-limited). The search must be repeated
in Scopus/WoS from the institutional network before submission; this file
records what was found so far.

## Protocol

- **Queries:** 18 topic queries (software and its validation; GUM vs Monte
  Carlo; adaptive MCM; multivariate models; Bayesian evaluation; top-down
  uncertainty from validation and QC data; bias and recovery; uncertainty from
  sampling; conformity assessment and decision rules; weighing instruments;
  internal QC; proficiency testing; calibration with errors in both
  variables; interlaboratory precision; teaching; machine learning; digital
  calibration certificates), plus 20 targeted queries (named tools: Suncal,
  METAS UncLib, NIST Uncertainty Machine, Python and R packages; software
  validation; GUM-6; uncorrected bias; robust ANOVA; EURAMET cg-18; decision
  rules; DCC).
- **Sources:** journal articles from 2016 onwards in 24 metrology and
  analytical-chemistry journals (Metrologia, Measurement, Meas. Sci. Technol.,
  Accred. Qual. Assur., IEEE TIM, Talanta, Anal. Chim. Acta, TrAC, Anal.
  Bioanal. Chem., Analyst, Anal. Chem., J. Open Source Softw., SoftwareX,
  Acta IMEKO, Metrology, Int. J. Metrol. Qual. Eng., Chemometr. Intell. Lab.
  Syst., Clin. Chem. Lab. Med., Microchem. J., tm – Tech. Messen, MAPAN,
  Precis. Eng.), plus any venue for the targeted queries.
- **Counts:** 1080 records from the topic queries, 653 after a title keyword
  screen and deduplication, 507 from the targeted queries; 74 included after
  reading titles and the available abstracts.
- **Inclusion:** methods or tools of general use for uncertainty evaluation,
  conformity, sampling, PT/QC or calibration; preference for recent (2020 →)
  and highly cited work in the leading journals. Excluded: single
  applications without methodological content, machine-learning
  "uncertainty quantification" outside metrology.
- ★ marks the full texts to read first; OA marks a Creative Commons licence
  or an open-access publisher in the Crossref record.

## Main implications

- The closest work to this program is Bettencourt da Silva (2026), a
  spreadsheet that implements the Eurachem/CITAC MUPR guide for chemistry.
  Its full text and supplement have been read. Recomputing the fluoranthene
  example and the pH data found a k printed as 2.01 instead of 2.09, s_r
  printed as 0.238 instead of 0.0238, absolute precision not
  divided by R̄, and a weighted mean recovery combined with the uncertainty of
  the arithmetic mean (research log F71–F73).
- Tool landscape: GUM Tree Calculator, METAS UncLib, Suncal, NIST Uncertainty
  Machine, VNA Tools, and the Mathmet survey of MU software against
  JCGM 100/101/102. The four full texts read so far (GTC, UncLib, Mathmet,
  MUPR spreadsheet) confirm the working hypothesis: none combines bottom-up
  and top-down methods with offline mobile use and a library of hundreds of
  self-checking published examples (`literature-review.md` §3). Suncal and
  the NIST Uncertainty Machine paper are still to be read.
- Methods: the proposed new VIM definition of measurement uncertainty
  (Possolo 2025) and the IUPAC brief guide (2024) should be reflected in the
  guide's terminology. Huang (2019, title and venue only so far) argues against the scaled and shifted
  t-distribution for Type A inputs in the MCM, which the program uses as
  JCGM 101 6.4.9 prescribes; the article should address it.
- Sampling: the new Eurachem guide VaMPIS (2025) and nonparametric
  estimation of UfS (2024) are the next extensions; the modified RANOVA paper
  (2016) may document the robust algorithm missing for F63.
- Bias: an asymmetric coverage interval for uncorrected bias (Anal. Chim.
  Acta 2026) is directly comparable with the program's RSSu/SUMU/Ue
  intervals.
- Palma et al. (2019) has an erratum (Talanta 2020); the library projects
  built from that paper have to be checked against it.

## Included references

### A. Software tools and their validation

| | Reference | Relevance | Link |
|---|---|---|---|
| ★ OA | Bettencourt da Silva (2026). User-friendly spreadsheet for implementing international guidance on the evaluation of measurement uncertainty. *Talanta* 130826. Cited 0. | Closest competitor in chemistry: a spreadsheet implementing international guidance (Eurachem/CITAC); compare scope, validation and usability with this program. **Read** with the supplement (2026-10-27): literature-review.md §3, research log D53, F71–F73. | https://doi.org/10.1016/j.talanta.2026.130826 |
| ★ OA | Hall (2022). The GUM Tree Calculator: A Python Package for Measurement Modelling and Data Processing with Automatic Evaluation of Uncertainty. *Metrology* 2, 128-149. Cited 13. | GUM Tree Calculator (Python): automatic propagation, complex quantities; open-source benchmark for the GUM framework. **Read** (2026-10-27): Willink ν for correlated inputs and JSON archives of uncertain numbers are features to adopt. | https://doi.org/10.3390/metrology2010009 |
| ★ OA | Lobsiger et al. (2022). METAS UncLib – A Measurement Uncertainty Calculator in Chemical Analysis. *CHIMIA* 76, 596. Cited 0. | METAS UncLib applied in chemical analysis: linear propagation library used by an NMI. **Read** (2026-10-27). | https://doi.org/10.2533/chimia.2022.596 |
| ★ | Sandia National Laboratories & Delker (2021). Sandia's Uncertainty Calculator: A Case Study in the Graded Approach. *NCSLI Measure* 13, 34-39. Cited 0. | Suncal (Sandia): graded software quality assurance of an uncertainty calculator — model for the validation report (ISO/IEC 17025 7.11). **Read** (2026-10-27, OSTI 1884074) with the Suncal paper (Delker, NCSLI Measure 13(3), 2021, OSTI 1830511): literature-review.md §3, research log F74. | https://doi.org/10.51843/measure.13.4.6 |
| ★ | Lafarge & Possolo (2015). The NIST Uncertainty Machine. *NCSLI Measure* 10, 20-27. Cited 65. | NIST Uncertainty Machine (listed as missing; DOI now known). | https://doi.org/10.1080/19315775.2015.11721732 |
|  | Albert (2020). Monte Carlo Uncertainty Propagation with the NIST Uncertainty Machine. *Journal of Chemical Education* 97, 1491-1494. Cited 33. | Teaching Monte Carlo propagation with the NIST Uncertainty Machine. | https://doi.org/10.1021/acs.jchemed.0c00096 |
| OA | Zeier et al. (2025). VNA Tools—A Metrology Software Supporting the Digital Traceability Chain. *Metrology* 5, 72. Cited 2. | VNA Tools: NMI metrology software supporting the digital traceability chain. | https://doi.org/10.3390/metrology5040072 |
| ★ OA | Pennecchi & Harris (2023). Mathmet Measurement Uncertainty Training activity – Overview of courses, software, and classroom examples. *Acta IMEKO* 12, 1-6. Cited 2. | Mathmet survey of courses, software and classroom examples, checked against JCGM 100/101/102 — the reference landscape for the gap analysis. **Read** (2026-10-27): 35 tools analysed, 54 % with validation evidence, JCGM 102 support 20 %/6 %/31 %. | https://doi.org/10.21014/actaimeko.v12i2.1310 |
|  | Zangl & Hoermaier (2017). Educational aspects of uncertainty calculation with software tools. *Measurement* 101, 257-264. Cited 12. | Educational aspects of uncertainty calculation with software tools. | https://doi.org/10.1016/j.measurement.2015.11.005 |
| OA | Veen & Cox (2021). Getting started with uncertainty evaluation using the Monte Carlo method in R. *Accreditation and Quality Assurance* 26, 129-141. Cited 24. | Getting started with the Monte Carlo method in R (AQA): accessible MCM implementation. | https://doi.org/10.1007/s00769-021-01469-5 |
|  | Black & Flater (2025). Integrating software metrology with the international metrology system. *150th anniversary of the Metre Convention — From Units to the Universe*. Cited 0. | Software metrology and the international system (NIST, 2025): context for validating measurement software. | https://doi.org/10.59161/wrxd1297 |
|  | Eichstädt & Wilkens (2016). GUM2DFT—a software tool for uncertainty evaluation of transient signals in the frequency domain. *Measurement Science and Technology* 27, 055001. Cited 43. | GUM2DFT: software for uncertainty of transient signals (PTB) — example of validated special-purpose tool. | https://doi.org/10.1088/0957-0233/27/5/055001 |

### B. Methods: GUM, Monte Carlo, Bayesian, JCGM

| | Reference | Relevance | Link |
|---|---|---|---|
| ★ OA | Possolo et al. (2024). A brief guide to measurement uncertainty (IUPAC Technical Report). *Pure and Applied Chemistry* 96, 113-134. Cited 27. | IUPAC Technical Report (2024): brief guide to measurement uncertainty, classical and Bayesian, with code; chemistry examples for the library. | https://doi.org/10.1515/pac-2022-1203 |
| ★ OA | Possolo (2025). Measurement uncertainty redefined. *Metrologia* 62, 042101. Cited 12. | New definition of measurement uncertainty proposed to the JCGM (2025): affects terminology of the guide and the article. **Read** (2026-10-27): noted in the glossary. | https://doi.org/10.1088/1681-7575/adfb80 |
| ★ | van der Veen et al. (2022). GUM guidance on developing and using measurement models. *Accreditation and Quality Assurance* 27, 295-297. Cited 4. | GUM-6 guidance on developing and using measurement models. | https://doi.org/10.1007/s00769-022-01509-8 |
| ★ | Demeyer et al. (2020). Guidance on Bayesian uncertainty evaluation for a class of GUM measurement models. *Metrologia* 58, 014001. Cited 19. | Guidance on Bayesian uncertainty evaluation for GUM models (Metrologia 2020). | https://doi.org/10.1088/1681-7575/abb065 |
|  | Possolo & Bodnar (2018). Approximate Bayesian evaluations of measurement uncertainty. *Metrologia* 55, 147-157. Cited 12. | Approximate Bayesian evaluations of measurement uncertainty (Possolo, Metrologia 2018). | https://doi.org/10.1088/1681-7575/aaa5be |
| OA | Marschall et al. (2021). Rejection sampling for Bayesian uncertainty evaluation using the Monte Carlo techniques of GUM-S1. *Metrologia* 59, 015004. Cited 9. | Rejection sampling for Bayesian evaluation with the MCM of GUM-S1. | https://doi.org/10.1088/1681-7575/ac3920 |
|  | Jamroz & Williams (2020). Consistency in Monte Carlo Uncertainty Analyses. *Metrologia* 57, 065008. Cited 29. | Consistency in Monte Carlo uncertainty analyses (Metrologia 2020). | https://doi.org/10.1088/1681-7575/aba5aa |
| OA | Degenhardt et al. (2025). A practically oriented, efficient alternative to the Monte Carlo method for measurement uncertainty estimation. *Metrologia* 62, 025004. Cited 3. | Efficient alternative to MCM for expensive models (Metrologia 2025). | https://doi.org/10.1088/1681-7575/adb3ab |
| ★ | Huang (2019). Why the scaled and shifted t-distribution should not be used in the Monte Carlo method for estimating measurement uncertainty?. *Measurement* 136, 282-288. Cited 13. | Argues against the scaled-shifted t-distribution for Type A inputs in MCM — relevant to the program's JCLM 101 6.4.9 choice (discuss). | https://doi.org/10.1016/j.measurement.2018.12.089 |
| OA | Stant et al. (2016). Comparing methods for evaluating measurement uncertainty given in the JCGM ‘Evaluation of Measurement Data’ documents. *Measurement* 94, 847-851. Cited 24. | Comparison of methods given in the JCGM documents. | https://doi.org/10.1016/j.measurement.2016.08.015 |
|  | Kacker (2018). Measurement uncertainty and its connection with true value in the GUM versus JCGM documents. *Measurement* 127, 525-532. Cited 21. | Kacker: uncertainty and true value in GUM vs JCGM documents. | https://doi.org/10.1016/j.measurement.2018.05.105 |
| ★ | Petitjean & Musset (2026). Finite-sample evaluation of dispersion estimators for measurement uncertainty: effects of distribution, replication structure, contamination, and uncertainty propagation. *Measurement Science and Technology* 37, 405002. Cited 0. | Finite-sample evaluation of dispersion estimators (MST 2026): Type A with few replicates, relevant to ν_eff findings (F69). | https://doi.org/10.1088/1361-6501/aeabcc |
|  | Xia et al. (2026). Measurement uncertainty evaluation with small samples: A review and prospect. *Measurement* 258, 119031. Cited 12. | Measurement uncertainty with small samples: review (2026). | https://doi.org/10.1016/j.measurement.2025.119031 |
|  | Ramnath (2019). Analysis and Comparison of Hyper-Ellipsoidal and Smallest Coverage Regions for Multivariate Monte Carlo Measurement Uncertainty Analysis Simulation Datasets. *MAPAN* 34, 387-402. Cited 5. | Hyper-ellipsoidal vs smallest coverage regions for multivariate MCM (JCGM 102 module). | https://doi.org/10.1007/s12647-019-00324-w |
| OA | Wübbeler et al. (2022). GUM-Compliant Uncertainty Evaluation Using Virtual Experiments. *Metrology* 2, 114-127. Cited 29. | GUM-compliant evaluation using virtual experiments (PTB, 2022). | https://doi.org/10.3390/metrology2010008 |

### C. Top-down, bias and validation data (chemistry)

| | Reference | Relevance | Link |
|---|---|---|---|
| ★ OA | do Nascimento Maranho & Lourenço (2026). Measurement uncertainty evaluation from analytical validation data: Asymmetric coverage interval due to uncorrected significant bias. *Analytica Chimica Acta* 1425, 346327. Cited 0. | Asymmetric coverage interval for an uncorrected significant bias from validation data (ACA 2026) — compare with the program's RSSu/SUMU/Ue intervals. | https://doi.org/10.1016/j.aca.2026.346327 |
| ★ | Synek (2017). How to encompass an uncorrected bias into the expanded uncertainty with a fixed coverage probability: calculation procedures. *Accreditation and Quality Assurance* 22, 179-186. Cited 7. | Synek (2017): uncorrected bias in the expanded uncertainty with fixed coverage probability. | https://doi.org/10.1007/s00769-017-1268-6 |
|  | Frenkel et al. (2019). Bias in analytical chemistry: A review of selected procedures for incorporating uncorrected bias into the expanded uncertainty of analytical measurements and a graphical method for evaluating the concordance of reference and test procedures. *Clinica Chimica Acta* 495, 129-138. Cited 19. | Review of procedures for incorporating uncorrected bias (Clin. Chim. Acta 2019). | https://doi.org/10.1016/j.cca.2019.03.1633 |
| ★ | Palma et al. (2020). Erratum to “Top-down evaluation of matrix effects uncertainty” [Talanta 192 (2018) 278–287]. *Talanta* 208, 120476. Cited 1. | Erratum to Palma et al. (2019) — must be checked against the library projects built from that paper. | https://doi.org/10.1016/j.talanta.2019.120476 |
|  | Milde et al. (2020). Measurement uncertainty evaluation from correlated validation data: Determination of elemental impurities in pharmaceutical products by ICP-MS. *Talanta* 220, 121386. Cited 22. | Uncertainty from correlated validation data (ICP-MS, Talanta 2020). | https://doi.org/10.1016/j.talanta.2020.121386 |
|  | Mateos et al. (2020). Impact of recovery correction or subjecting calibrators to sample preparation on measurement uncertainty: PAH determinations in waters. *Talanta* 207, 120274. Cited 12. | Recovery correction vs calibrators through sample preparation (Talanta 2020; Mateos et al., already used). | https://doi.org/10.1016/j.talanta.2019.120274 |
|  | Padoan et al. (2017). An approach for estimating measurement uncertainty in medical laboratories using data from long-term quality control and external quality assessment schemes. *Clinical Chemistry and Laboratory Medicine (CCLM)* 55. Cited 37. | Uncertainty in medical laboratories from long-term QC and EQA data. | https://doi.org/10.1515/cclm-2016-0896 |
|  | Şeneş et al. (2026). Measurement uncertainty in urine drug analysis: A comparison of ISO/TS 20914 and GUM/Eurachem approaches. *Clinica Chimica Acta* 590, 121067. Cited 0. | ISO/TS 20914 vs GUM/Eurachem approaches compared (2026). | https://doi.org/10.1016/j.cca.2026.121067 |
| OA | Synek & Kříženecká (2023). Estimation of uncertainty from duplicate measurements: new quantification procedure in the case of concentration-dependent precision. *Accreditation and Quality Assurance* 28, 279-298. Cited 5. | Uncertainty from duplicate measurements with concentration-dependent precision (AQA 2023). | https://doi.org/10.1007/s00769-023-01556-9 |
|  | Renato Couto & Rebello Lourenço (2023). Bottom-up and top-down measurement uncertainty evaluation for multivariate spectrophotometric procedures. *Microchemical Journal* 193, 109194. Cited 3. | Bottom-up and top-down evaluation for multivariate spectrophotometric procedures. | https://doi.org/10.1016/j.microc.2023.109194 |

### D. Uncertainty from sampling

| | Reference | Relevance | Link |
|---|---|---|---|
| ★ | Ramsey (2025). Validation of Measurement Procedures that Include Sampling (VaMPIS) – a new Eurachem Guide. *Proceedings of the IMEKO TC8, TC11 and TC24 Conference on Chemical Measurements* 155-157. Cited 0. | New Eurachem guide VaMPIS (2025): validation of procedures that include sampling — next extension of the sampling module. | https://doi.org/10.21014/tc24-2025.030 |
| ★ OA | Ramsey & Rostron (2024). Measurement uncertainty from sampling and its role in validation of measurement procedures. *Accreditation and Quality Assurance* 29, 153-162. Cited 9. | Ramsey & Rostron (2024): UfS and its role in validation. | https://doi.org/10.1007/s00769-024-01575-0 |
| ★ | Lourenço & Oliveira (2024). Nonparametric estimation of measurement uncertainty arising from sampling. *Analytica Chimica Acta* 1305, 342597. Cited 6. | Nonparametric estimation of uncertainty from sampling (ACA 2024) — alternative to robust ANOVA (F63). | https://doi.org/10.1016/j.aca.2024.342597 |
| ★ OA | Dołęgowska et al. (2016). Heterogeneous areas—identification of outliers and calculation of soil sampling uncertainty using the modified RANOVA method. *Environmental Monitoring and Assessment* 188, 581. Cited 4. | Modified RANOVA for heterogeneous areas — may document the robust ANOVA algorithm (F63). | https://doi.org/10.1007/s10661-016-5584-9 |
| OA | Ramsey & Ellison (2017). Combined uncertainty factor for sampling and analysis. *Accreditation and Quality Assurance* 22, 187-189. Cited 10. | Combined uncertainty factor for sampling and analysis. | https://doi.org/10.1007/s00769-017-1271-y |
|  | Kostakis et al. (2020). A novel nested design approach for estimation of uncertainty arising from sampling in feed. *Accreditation and Quality Assurance* 25, 329-333. Cited 1. | Nested design for uncertainty from sampling in feed. | https://doi.org/10.1007/s00769-020-01443-7 |
|  | Sona & Dubé (2021). Sampling particulate matter for analysis – Controlling uncertainty and bias using the theory of sampling. *Analytica Chimica Acta* 1185, 338982. Cited 2. | Theory of Sampling for particulate matter: controlling uncertainty and bias. | https://doi.org/10.1016/j.aca.2021.338982 |
| OA | Svensmark (2021). Extensions to the Theory of Sampling 1. The extended Gy's formula, the segregation paradox and the fundamental sampling uncertainty (FSU). *Analytica Chimica Acta* 1187, 339127. Cited 6. | Extended Gy's formula and fundamental sampling uncertainty (ACA 2021). | https://doi.org/10.1016/j.aca.2021.339127 |
|  | Martins & Lourenço (2024). Measurement uncertainty for <711> Dissolution tests using Monte Carlo methods – Uncertainties arising from sampling and analytical steps. *Analytica Chimica Acta* 1330, 343285. Cited 11. | Uncertainty from sampling and analytical steps in dissolution tests by MCM. | https://doi.org/10.1016/j.aca.2024.343285 |

### E. Conformity assessment and decision risks

| | Reference | Relevance | Link |
|---|---|---|---|
| ★ OA | Shirono et al. (2022). Economic optimization of acceptance interval in conformity assessment: 1. Process with no systematic effect. *Metrologia* 59, 045005. Cited 4. | Economic optimisation of acceptance intervals, process without systematic effect (Metrologia 2022, part 1). **Read** (2026-10-27). | https://doi.org/10.1088/1681-7575/ac6fa1 |
| ★ OA | Shirono et al. (2022). Economic optimization of acceptance interval in conformity assessment: 2. Process with unknown systematic effect. *Metrologia* 59, 045006. Cited 3. | Same, process with unknown systematic effect (part 2). **Read** (2026-10-27). | https://doi.org/10.1088/1681-7575/ac6fa2 |
| OA | Pennecchi & Kuselman (2024). Probabilities of true and false decisions in conformity assessment of a finite sample of items. *Measurement Science and Technology* 35, 056003. Cited 3. | True and false decision probabilities for a finite sample of items (MST 2024). | https://doi.org/10.1088/1361-6501/ad2439 |
|  | Bettencourt da Silva et al. (2019). Spreadsheet for evaluation of global risks in conformity assessment of a multicomponent material or object. *Chemometrics and Intelligent Laboratory Systems* 188, 1-5. Cited 30. | Spreadsheet for global risks of multicomponent materials (IUPAC project). | https://doi.org/10.1016/j.chemolab.2019.02.010 |
|  | Lourenço & Bettencourt da Silva (2019). Risk of false conformity decisions of multicomponent items controlled by correlated measurement results due to the sharing of analytical steps. *Talanta* 196, 174-181. Cited 34. | Risks for correlated multicomponent results sharing analytical steps. | https://doi.org/10.1016/j.talanta.2018.12.021 |
|  | Maranho & Lourenço (2026). Development of a Python-based tool for evaluation of multivariate decision risk considering measurement uncertainty and correlation effects. *Microchemical Journal* 224, 117824. Cited 1. | Python tool for multivariate decision risk with correlation (2026). | https://doi.org/10.1016/j.microc.2026.117824 |
| OA | Shirono et al. (2026). Economic optimisation of acceptance interval in conformity assessment for processes with known and unknown biases. *Measurement* 261, 119847. Cited 1. | Economic optimisation of acceptance interval with known and unknown biases (2026). | https://doi.org/10.1016/j.measurement.2025.119847 |
|  | Carobbi & Pennecchi (2016). Bayesian conformity assessment in presence of systematic measurement errors. *Metrologia* 53, S74-S80. Cited 20. | Bayesian conformity assessment with systematic errors (Metrologia 2016). | https://doi.org/10.1088/0026-1394/53/2/s74 |
| OA | Purata-Sifuentes et al. (2023). Overestimation of conformity assessment risks in legal requirements of weighing instruments. *Acta IMEKO* 12, 1-7. Cited 4. | Overestimated conformity risks in legal requirements for weighing instruments. | https://doi.org/10.21014/actaimeko.v12i3.1471 |

### F. Weighing instruments (EURAMET cg-18)

| | Reference | Relevance | Link |
|---|---|---|---|
| ★ OA | Fritsch (2019). EURAMET cg-18 – state-of-the-art calibration guideline for non-automatic weighing instruments. *ACTA IMEKO* 8, 10. Cited 1. | EURAMET cg-18 state of the art (Fritsch et al., 2019) — context for the cg-18 projects and findings F68–F70. **Read** (2026-10-27): uncertainty of a weighing result and minimum weight (cg-18 §8) are not yet in the program. | https://doi.org/10.21014/acta_imeko.v8i3.655 |
|  | Fritsch & Ciesniewski (2015). Calibration of Weighing Instruments: Measurement Uncertainty, Minimum Weight and the Safe Weighing Range. *NCSLI Measure* 10, 56-65. Cited 1. | Calibration of weighing instruments: uncertainty, minimum weight, safe weighing range. | https://doi.org/10.1080/19315775.2015.11721736 |
|  | Sheveleva & Foyer (2025). Measurement Data and Information of Non-Automatic Weighing Instruments as Structured Data. *Proceedings of the IMEKO TC6 Conference on Digitalization - M4DConf 2025* 292-296. Cited 0. | NAWI calibration data as structured (machine-readable) data (2025). | https://doi.org/10.21014/tc6-2025.054 |
|  | Purata-Sifuentes et al. (2026). Decentered process effect on conformity assessment risks for manufacturing of non-automatic weighing instruments. *Measurement* 270, 120882. Cited 0. | Decentred process effect on conformity risks in manufacturing of NAWIs (2026). | https://doi.org/10.1016/j.measurement.2026.120882 |

### G. Proficiency testing, interlaboratory studies, internal QC

| | Reference | Relevance | Link |
|---|---|---|---|
| ★ OA | Ilinca et al. (2026). Assigned-Value Uncertainty in Limited-Participant Proficiency Testing: Monte Carlo Evaluation of ISO 13528 Estimators in an Industrial Calibration Scenario. *Metrology* 6, 76. Cited 0. | Assigned-value uncertainty with few PT participants: Monte Carlo study of ISO 13528 estimators (2026). | https://doi.org/10.3390/metrology6040076 |
|  | Coucke & Soumali (2020). A critical view at the ISO 13528 and IUPAC's harmonized protocol approach for proficiency testing for homogeneity assessment for quantitative variables. *The Analyst* 145, 7630-7635. Cited 3. | Critical view of the ISO 13528 and IUPAC homogeneity assessment. | https://doi.org/10.1039/d0an01607a |
|  | Carobbi (2017). A modified ISO 13528 robust analysis (Algorithm A) that takes measurement uncertainty into account. *Measurement* 110, 296-306. Cited 6. | Algorithm A modified to take the reported uncertainties into account. | https://doi.org/10.1016/j.measurement.2017.07.006 |
|  | Morales et al. (2021). Boxplot fences in proficiency testing. *Accreditation and Quality Assurance* 26, 193-200. Cited 12. | Boxplot fences in proficiency testing. | https://doi.org/10.1007/s00769-021-01474-8 |
| ★ | Koepke et al. (2017). Consensus building for interlaboratory studies, key comparisons, and meta-analysis. *Metrologia* 54, S34-S62. Cited 83. | Consensus building for interlaboratory studies and key comparisons (NIST Consensus Builder). | https://doi.org/10.1088/1681-7575/aa6c0e |
| OA | Molenaar et al. (2018). Efficient and robust analysis of interlaboratory studies. *Chemometrics and Intelligent Laboratory Systems* 175, 65-73. Cited 15. | Efficient and robust analysis of interlaboratory studies. | https://doi.org/10.1016/j.chemolab.2018.01.003 |
|  | Thompson (2021). Statistical internal quality control (SIQC) in chemical measurement—do we really understand it?. *Accreditation and Quality Assurance* 26, 99-101. Cited 0. | Statistical internal quality control in chemical measurement — do we really understand it? | https://doi.org/10.1007/s00769-021-01467-7 |
|  | Braga et al. (2020). The internal quality control in the traceability era. *Clinical Chemistry and Laboratory Medicine (CCLM)* 59, 291-300. Cited 62. | Internal quality control in the traceability era. | https://doi.org/10.1515/cclm-2020-0371 |

### H. Calibration and regression

| | Reference | Relevance | Link |
|---|---|---|---|
| ★ | Tellinghuisen (2020). Least Squares Methods for Treating Problems with Uncertainty inxandy. *Analytical Chemistry* 92, 10863-10871. Cited 32. | Least squares with uncertainty in x and y (Anal. Chem. 2020). | https://doi.org/10.1021/acs.analchem.0c02178 |
|  | Cecelski et al. (2022). Errors-in-variables calibration with dark uncertainty. *Metrologia* 59, 045002. Cited 7. | Errors-in-variables calibration with dark uncertainty (Metrologia 2022). | https://doi.org/10.1088/1681-7575/ac711c |
|  | Tellinghuisen (2022). Goodness-of-Fit Tests in Calibration: Are They Any Good for Selecting Least-Squares Weighting Formulas?. *Analytical Chemistry* 94, 15997-16005. Cited 7. | Goodness-of-fit tests and weighting in calibration (Anal. Chem. 2022). | https://doi.org/10.1021/acs.analchem.2c02904 |
|  | Raposo (2016). Evaluation of analytical calibration based on least-squares linear regression for instrumental techniques: A tutorial review. *TrAC Trends in Analytical Chemistry* 77, 167-185. Cited 183. | Tutorial review of least-squares calibration for instrumental techniques (TrAC 2016). | https://doi.org/10.1016/j.trac.2015.12.006 |

### I. Digital calibration certificates and reporting

| | Reference | Relevance | Link |
|---|---|---|---|
|  | Gadelrab & Abouhogail (2021). Towards a new generation of digital calibration certificate: Analysis and survey. *Measurement* 181, 109611. Cited 26. | Survey of digital calibration certificates. | https://doi.org/10.1016/j.measurement.2021.109611 |
| OA | Smith et al. (2022). The Storage within Digital Calibration Certificates of Uncertainty Information Obtained Using a Monte Carlo Method. *Metrology* 2, 33-45. Cited 8. | Storing Monte Carlo uncertainty information in digital calibration certificates. | https://doi.org/10.3390/metrology2010003 |
| ★ OA | Volinsky (2026). Digital precision in metrology: significant figures and trailing zeros as machine readable metadata. *Metrologia* 63, 055006. Cited 0. | Significant figures and trailing zeros as machine-readable metadata (Metrologia 2026) — relevant to report rounding. | https://doi.org/10.1088/1681-7575/aea165 |
