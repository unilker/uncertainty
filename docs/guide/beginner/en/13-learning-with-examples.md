# 13. Learning with the examples

Each of the 286 examples of the library comes from a published guide, standard or paper, and the program recomputes them at every change. Below, the examples are grouped by level. Each group starts with what it teaches; click the name of an example to open it. The **Overview** tab of the opened example explains it step by step.

> **Tip:** If you are new, start with the first groups of the Beginner level; in each example press **Calculate** first, then compare the results with the explanation.

<a id="level-1"></a>

## Level 1: Beginner

### Weighing a powder

A very simple budget: the mass of a powder is the reading of the filled container minus the reading of the empty one, with a small correction for the drift of the balance. The two readings have the same uncertainty; together they give √2 times one of them, not twice. GUM and Monte Carlo give the same result, because the model is linear.

- [Mass of a powder by weighing (NIST TN 1900 E1)](examples.md#nist-tn1900-e1-weighing)

### Adding inputs: normal and rectangular

Four inputs are simply added. The three variants differ only in the shape of the input distributions. Compare the GUM result with Monte Carlo: when one rectangular input dominates, the result is not normal any more and the 95 % interval is narrower than 2u. This is the classic case where k = 2 is not right.

- [Additive model with Gaussian inputs](examples.md#jcgm101-9.2.2-additive-gaussian)
- [Additive model with rectangular inputs](examples.md#jcgm101-9.2.3-additive-rectangular)
- [Additive model with a dominant rectangular input](examples.md#jcgm101-9.2.4-additive-dominant-rectangular)

### Calibrating a weight

A 10 kg weight is compared with a reference weight. It shows the typical calibration budget: the certificate of the standard (U and k), drift since its last calibration, the comparator, air buoyancy and the repeated comparisons. The certificate of the standard is the largest contribution.

- [Calibration of a 10 kg weight](examples.md#ea402-s2-weight-10kg)

### A 10 kg weight in UKAS M3003

The same kind of weight calibration as in EA-4/02, presented in the British guide. Comparing the two shows that different laboratories make slightly different choices of components and distributions, and that the result is still similar.

- [10 kg weight of OIML class M1 (UKAS M3003 K.4)](examples.md#m3003-k4-weight-10kg)

### Calibrating a vernier calliper

A gauge block is measured with a calliper. Most components are rectangular: the resolution of the scale, the temperature difference, the flatness of the jaws. A good example of how a reading on a scale is turned into an uncertainty.

- [Calibration of a vernier calliper at 150 mm](examples.md#ea402-s10-vernier-calliper)

### Calibrating a standard resistor

A 10 kΩ resistor is compared with a reference resistor. The model is a product, so the sensitivity coefficients are not 1; look at how the budget converts each relative effect into ohms.

- [Calibration of a 10 kΩ standard resistor](examples.md#ea402-s3-resistor-10kohm)

### Calibrating a digital multimeter

A hand-held multimeter is checked at 100 V against a calibrator. The resolution of the display is often the largest term: a value shown with one decimal can be anywhere within half a digit.

- [Calibration of a hand-held digital multimeter at 100 V](examples.md#ea402-s9-dmm-100v)

### A multimeter at 10 V (TÜBİTAK UME)

An example from the uncertainty procedure of TÜBİTAK UME: calibration of a digital multimeter at 10 V DC. It follows the same steps as the guides and is a good template for an electrical calibration certificate.

- [10 V DC calibration of a digital multimeter (TÜBİTAK UME uncertainty procedure, Example 19.1)](examples.md#ume-procedure-19-1-dmm-10v-calibration)

### Calibrating a gauge block

A 50 mm gauge block is compared with a reference block. Temperature matters: a small difference in temperature or in expansion coefficient changes the length, which is why the budget has thermal terms.

- [Calibration of a 50 mm gauge block](examples.md#ea402-s4-gauge-block-50mm)

### A temperature block calibrator

The temperature of a dry block at 180 °C is measured with a reference thermometer. The budget lists the effects of the block itself: axial and radial non-uniformity, stability, loading.

- [Calibration of a temperature block calibrator at 180 °C](examples.md#ea402-s11-block-calibrator-180c)

### Calibrating a water meter

Three related projects: the reference volume, the relative error of one run and the mean error of three runs. They show how a long calculation can be split into steps, each with its own budget.

- [Water meter calibration: relative error of one run](examples.md#ea402-s12-water-meter-error)
- [Water meter calibration: mean of three runs](examples.md#ea402-s12-water-meter-mean-error)
- [Water meter calibration: reference volume](examples.md#ea402-s12-water-meter-volume)

### A resistor in UKAS M3003

A 10 kΩ resistor calibrated by comparing voltages. A clear example of a budget table with the sources, their values, the divisors (√3, 2, 1) and the resulting standard uncertainties.

- [10 kΩ resistor by voltage intercomparison (UKAS M3003 K.1)](examples.md#m3003-k1-resistor-10kohm)

### A weighing machine at 200 g

A weighing machine is checked with a 200 g weight. Repeatability, resolution and the weight's certificate are the main terms.

- [Weighing machine of 205 g capacity at 200 g (UKAS M3003 K.5)](examples.md#m3003-k5-weighing-machine-200g)

### A pressure indicator at 2 MPa

A digital pressure indicator is compared with a dead-weight tester. A typical pressure calibration budget.

- [Digital pressure indicator at 2 MPa against a dead weight tester (UKAS M3003 K.8)](examples.md#m3003-k8-pressure-indicator-2mpa)

### Thickness of a sheet

The thickness of an aluminium sample is the mean of five readings. It combines a Type A evaluation (the scatter of the readings) with Type B terms: calibration, resolution, drift and temperature.

- [Thickness of an aluminium test sample (UKAS M3003 K.10)](examples.md#m3003-k10-sheet-thickness)

### Calibrating volumetric glassware

The volume of a flask or pycnometer is found by weighing the water it holds. You see how the density of water and air, the temperature and the balance enter the result.

- [Gravimetric calibration of a 1000 mL volumetric flask (EURAMET cg-19 Example 1)](examples.md#cg19-ex1-flask-1000ml)
- [Gravimetric calibration of a 100 mL glass pycnometer (EURAMET cg-19 Example 2)](examples.md#cg19-ex2-pycnometer-100ml)

### Preparing a calibration standard

A cadmium solution is made by dissolving a weighed amount of pure metal in a known volume. The classic first example of the Eurachem guide: purity, mass and volume combine as relative uncertainties because the model is a product and a ratio.

- [Cadmium standard with the volume sub-model](examples.md#quam-a1-cadmium-standard-volume)
- [Preparation of a cadmium calibration standard](examples.md#quam-a1-cadmium-standard)

### Bias from a reference material or proficiency tests

The top-down way of finding the systematic part of the uncertainty: measure a certified reference material, or look at your results in proficiency tests, and turn the differences into u(bias). Compare the variants with one CRM, several CRMs, PT results and recovery.

- [Bias from one certified reference material](examples.md#nordtest-6.1-one-crm)
- [Bias from several certified reference materials](examples.md#nordtest-6.1-three-crms)
- [Bias from proficiency tests](examples.md#nordtest-6.2-proficiency-tests)
- [Bias from recovery experiments](examples.md#nordtest-6.3-recovery)

### Precision from duplicates

Analysing samples in duplicate gives the repeatability; a control sample measured over months gives the within-laboratory reproducibility. These are the random part of a top-down uncertainty.

- [Ammonium: within-laboratory reproducibility from control samples and duplicates](examples.md#nordtest-5.3-ammonium-duplicates)
- [Dissolved oxygen: reproducibility from duplicates](examples.md#nordtest-5.3-oxygen-duplicates)

### The whole top-down budget in one example

Ammonium in water: precision from the control chart and bias from proficiency tests are combined into the expanded uncertainty. This is the recipe that most testing laboratories use.

- [Ammonium in water from control limits and proficiency tests](examples.md#nordtest-3-ammonium-pt)

### A first control chart

Sixty control values of zinc in water are plotted with their warning and action limits. Learn to read a chart: which points are fine, which ones are warnings and which ones demand action.

- [Zinc in water: X-chart from 60 control values (Nordtest TR 569 Table 1)](examples.md#tr569-table1-zinc)

### Setting the limits of a control chart

The three ways of setting limits: from the data (statistical), from a target, or around a reference value. Compare the variants to see why target limits are often wider and more useful.

- [Setting the limits of an X-chart, case 1 (Nordtest TR 569 Chapter 7)](examples.md#tr569-ch7-case1-statistical)
- [Setting the limits of an X-chart, case 2 (Nordtest TR 569 Chapter 7)](examples.md#tr569-ch7-case2-target-mean)
- [Setting the limits of an X-chart, case 3 (Nordtest TR 569 Chapter 7)](examples.md#tr569-ch7-case3-target-reference)
- [Statistical limits of an r%-chart for duplicates (Nordtest TR 569 Chapter 7)](examples.md#tr569-ch7-range-case1-statistical)
- [Target limits of an r%-chart from a repeatability limit (Nordtest TR 569 Chapter 7)](examples.md#tr569-ch7-range-case2-target)

### Probability that an item conforms

A measured value lies inside the specification, but how sure can we be? These examples calculate the probability of conformity from the measured value and its uncertainty, for one-sided and two-sided limits.

- [Bursting strength of a container: probability of conformity (JCGM 106 7.3)](examples.md#jcgm106-7.3-bursting-strength)
- [Breakdown voltage of a Zener diode: probability of conformity (JCGM 106 7.3)](examples.md#jcgm106-7.3-zener-diode)
- [Kinematic viscosity of a motor oil: two-sided tolerance (JCGM 106 7.4)](examples.md#jcgm106-7.4-motor-oil-viscosity)

### The assigned value of a proficiency test

The same results evaluated with two robust estimators: Algorithm A and the median. Robust statistics keep a few outlying laboratories from moving the assigned value.

- [Atrazine in drinking water: robust average by Algorithm A (ISO 13528 E.3)](examples.md#iso13528-e3-atrazine-algorithm-a)
- [Atrazine in drinking water: median and nIQR (ISO 13528 E.3)](examples.md#iso13528-e3-atrazine-median)

### Scoring laboratories: z, ζ and E_n

Every participant gets a z-score (is the result close enough to the assigned value?) and ζ and E_n scores (is the result consistent with the uncertainty the laboratory claimed?). The table of the standard is reproduced for all 21 numerical results.

- [Mercury in animal feed: z, z′, ζ and E_n scores (ISO 13528 E.4)](examples.md#iso13528-e4-mercury-feed)

### A sum of four concentrations

The plasma anion gap is a linear combination of four ion concentrations. With a linear model GUM and Monte Carlo agree perfectly: a good check that you understand the basics.

- [Plasma anion gap: a linear combination of four ion concentrations (Possolo & Meija 2022)](examples.md#possolo-meija-2022-plasma-anion-gap)

### Hardness of water

Total hardness from the calcium and magnesium concentrations: a short, conventional measurement model from the IUPAC guide, ideal for practising the steps of the GUM.

- [Total hardness of water from Ca²⁺ and Mg²⁺: a conventional measurement model (IUPAC 2024, Examples 3C and 4A)](examples.md#iupac2024-4a-water-hardness)

### A titration

The concentration of sodium chloride by titration with silver nitrate. Volumes, the concentration of the titrant and the endpoint are the inputs.

- [Amount concentration of NaCl by argentometric titration (Mohr): conventional model (IUPAC 2024, Example 4I)](examples.md#iupac2024-4i-argentometric-titration)

### Molar mass from atomic weights

The relative molecular mass of glucose from the standard atomic weights, which are given as intervals. An interval with no other information becomes a rectangular distribution.

- [Relative molecular mass of glucose from standard atomic weight intervals (JCGM GUM-5 Example 3)](examples.md#gum5-e3-molar-mass-glucose)

### Radioactive decay

The activity of a source is corrected for the time since its reference date. The half-life and the time enter through an exponential function.

- [Activity of a ²⁰⁷Bi source corrected for decay (JCGM GUM-5 Example 14)](examples.md#gum5-e14-radioactive-decay-bi207)

### Mean of daily temperatures

The monthly mean of daily maximum temperatures: a pure Type A evaluation. With few days, the t distribution makes the coverage factor clearly larger than 2.

- [Monthly mean of daily maximum temperature (NIST TN 1900 E2)](examples.md#nist-tn1900-e2-surface-temperature)

<a id="level-2"></a>

## Level 2: Intermediate

### Reporting with the CMC

ILAC P14 says that a calibration certificate must not state an uncertainty smaller than the laboratory's calibration and measurement capability. The program applies this floor when it reports.

- [Reporting rule of ILAC P14 5.5: the CMC as the smallest reportable uncertainty (EA-4/02 S2 weight, illustrative CMC)](examples.md#ilac-p14-cmc-floor-weight-10kg)

### When one rectangular term dominates

A voltmeter whose resolution dominates. UKAS M3003 gives a special coverage factor for this case, because the result is closer to rectangular than to normal.

- [Digital voltmeter with a dominant resolution term (UKAS M3003 Appendix C)](examples.md#m3003-c5-dvm-dominant-rectangular)

### A multimeter specification across a range

A 6½-digit multimeter on its 1 V range: some contributions are relative to the reading (ppm), others are absolute (µV). The examples show the uncertainty at several points of the range and the coverage factor when the specification dominates.

- [DMM on the 1 V range: budget at 0.95 V (UKAS M3003 L.3)](examples.md#m3003-l3-dmm-1v-range-095v)
- [DMM on the 1 V range at full scale: coverage factor for a dominant specification term (UKAS M3003 L.3.20)](examples.md#m3003-l3-dmm-1v-range-1v-dominant)
- [DMM on the 1 V range: uncertainty at 500 mV from the range formula (UKAS M3003 L.3)](examples.md#m3003-l3-dmm-1v-range-500mv)

### Standardising a sodium hydroxide solution

Titration against a weighed primary standard. The example uses sub-models (the mass, the volume of the burette) and shows that repeatability can be put into one combined term.

- [NaOH standardisation with sub-models](examples.md#quam-a2-naoh-standardisation-submodels)
- [Standardising a sodium hydroxide solution](examples.md#quam-a2-naoh-standardisation)

### Titration of hydrochloric acid

The concentration of HCl from a titration with standardised NaOH. The uncertainty of the NaOH from the previous example is carried over: results of one evaluation become inputs of the next.

- [Acid–base titration of hydrochloric acid](examples.md#quam-a3-hcl-titration)

### Pesticides in bread

Two ways to the same answer: a budget built from the steps of the method, and a top-down estimate from validation data (precision and recovery). Compare which components each approach captures.

- [Organophosphorus pesticides in bread: combination](examples.md#quam-a4-pesticides-combination)
- [Organophosphorus pesticides in bread from validation data](examples.md#quam-a4-pesticides-topdown)

### Cadmium released from ceramics

An empirical method: the result is defined by the procedure (time, temperature, acid). Effects of these conditions are included as uncertainty terms.

- [Cadmium released from ceramic ware](examples.md#quam-a5-cadmium-release)

### Crude fibre in animal feed

Another empirical method, evaluated from the precision of an interlaboratory study: the reproducibility of the method is the main part of the uncertainty.

- [Crude fibre in animal feed (empirical method)](examples.md#quam-a6-crude-fibre)

### Top-down for environmental analyses

BOD in wastewater and PCB in sediment, each evaluated twice: once with a CRM and once with proficiency tests for the bias. See how the choice of bias data changes the result.

- [BOD in wastewater: control chart and one CRM](examples.md#nordtest-8-bod-crm)
- [BOD in wastewater: control chart and proficiency tests](examples.md#nordtest-8-bod-pt)
- [PCB in sediment: control sample and one CRM](examples.md#nordtest-8-pcb-crm)
- [PCB in sediment: control sample and proficiency tests](examples.md#nordtest-8-pcb-pt)

### Uncertainty from in-house precision and recovery

The 2026 Eurachem/CITAC guide (MUPR) evaluates uncertainty from data every laboratory has: precision at several levels and recovery from reference materials or spiked samples. The examples cover pooled and concentration-dependent precision and different recovery designs.

- [Nitrate in drinking water: two precision intervals](examples.md#mupr-b1-nitrate)
- [Nitrate: pooled precision and two recovery levels](examples.md#mupr-b2-pooled-precision)
- [Nitrate: result as the mean of 3 replicates on 2 days](examples.md#mupr-b2-replicates-on-two-days)
- [Nitrate: precision as a linear function of concentration](examples.md#mupr-b3-linear-precision)
- [Arsenic in sediment: one CRM](examples.md#mupr-b4-arsenic-one-crm)
- [Arsenic: CRM and spiked samples with matrix-dependent recovery](examples.md#mupr-b5-crm-and-spiked-samples)
- [Trace element in sediment: heterogeneity and 22 proficiency tests](examples.md#mupr-b6-sediment-22-pt)
- [Recovery from two CRMs and a proficiency-test sample](examples.md#mupr-ex2-reference-materials)
- [Recovery from spiked river waters](examples.md#mupr-ex3-spiked-samples)

### Control charts in practice

The worked examples of the Nordtest handbook: X-charts, R-charts and r%-charts for duplicates, target limits from a required uncertainty, blanks, CRMs and the yearly review of the limits.

- [Nickel in low-alloy steel by XRF: target limits from the required uncertainty (Nordtest TR 569 Example 1)](examples.md#tr569-ex1-nickel-steel-xrf)
- [Boron migrating from toys by ICP-OES: target limits from the reproducibility of EN 71-3 (Nordtest TR 569 Example 10)](examples.md#tr569-ex10-boron-toys-target)
- [Repeatability and within-laboratory reproducibility from replicate control results (Nordtest TR 569 Example 12)](examples.md#tr569-ex12-pooling-sr-srw)
- [Cobalt in low-alloy steel by XRF: target limits from the limit of quantification (Nordtest TR 569 Example 2)](examples.md#tr569-ex2-cobalt-steel-xrf)
- [Ammonium nitrogen in water: R-chart of duplicate test samples (Nordtest TR 569 Example 3)](examples.md#tr569-ex3-ammonium-r)
- [Ammonium nitrogen in water: X-chart of a standard solution (Nordtest TR 569 Example 3)](examples.md#tr569-ex3-ammonium-x)
- [Lead in water by ICP-MS: X-chart of an in-house lake water (Nordtest TR 569 Example 4)](examples.md#tr569-ex4-lead-water-icpms)
- [Arsenic in a fish CRM by ICP-MS: chart with action limits only at ± 15 % (Nordtest TR 569 Example 5)](examples.md#tr569-ex5-arsenic-crm-tolerance)
- [Total phosphorus in water: r%-chart of duplicates (Nordtest TR 569 Example 6)](examples.md#tr569-ex6-phosphorus-relative-range)
- [β-HCH in cod liver oil by GC-ECD: target limits around the reference value (Nordtest TR 569 Example 7)](examples.md#tr569-ex7-beta-hch-crm-target)
- [Copper in water by ICP-OES: R-chart of duplicates and its review (Nordtest TR 569 Example 8)](examples.md#tr569-ex8-copper-r-review)
- [Copper in water by ICP-OES: new X-chart limits from all data (Nordtest TR 569 Example 8)](examples.md#tr569-ex8-copper-x-new-limits)
- [Copper in water by ICP-OES: review of preliminary X-chart limits (Nordtest TR 569 Example 8)](examples.md#tr569-ex8-copper-x-review)
- [Zinc in hydrogen peroxide: X-chart of a procedural blank (Nordtest TR 569 Example 9)](examples.md#tr569-ex9-zinc-procedural-blank)
- [Bias of a 1 mL micropipette: chart with action limits at the tolerance (Nordtest TR 569 Example 11)](examples.md#tr569-ex11-micropipette-tolerance)

### Decision limits

When must a speeding ticket be issued, or a food sample be declared non-compliant? A decision limit is placed beyond the legal limit, so that a false accusation is very unlikely (guarded rejection).

- [Nandrolone in animal products: decision limit for a screening test (JCGM 106 8.3.3)](examples.md#jcgm106-8.3-nandrolone)
- [Speed limit enforcement: guarded rejection (JCGM 106 8.3.3)](examples.md#jcgm106-8.3-speed-limit)

### Compliance with a specification

The Eurachem/CITAC guide on compliance: guarded acceptance and rejection, a t distribution with few degrees of freedom, and a lognormal distribution for a large relative uncertainty near a limit.

- [Nickel in stainless steel: guarded acceptance with a two-sided specification (Eurachem/CITAC compliance guide, Example B1)](examples.md#eurachem-compliance-b1-nickel-steel)
- [Analyte below an upper limit, t distribution with ν = 8: guarded rejection (Eurachem/CITAC compliance guide, Example B2)](examples.md#eurachem-compliance-b2-upper-limit-t)
- [Banned substance at a 2 ng/g limit with u_rel = 35 %: lognormal PDF, exact s_G (Eurachem/CITAC compliance guide, Example B3)](examples.md#eurachem-compliance-b3-banned-substance-lognormal-exact)
- [Banned substance at a 2 ng/g limit with u_rel = 35 %: lognormal PDF, s_G ≈ u_rel (Eurachem/CITAC compliance guide, Example B3)](examples.md#eurachem-compliance-b3-banned-substance-lognormal)
- [Banned substance at a 2 ng/g limit with u_rel = 35 %: normal PDF with u = u_rel·L = 0.7 ng/g (Eurachem/CITAC compliance guide, Example B3)](examples.md#eurachem-compliance-b3-banned-substance-normal)

### Guard bands from the test uncertainty ratio

In calibration, acceptance limits are often derived from the ratio of the tolerance to the measurement uncertainty (TUR). Three common guard-band methods are compared together with the risks they lead to.

- [Acceptance limits from the test uncertainty ratio: Dobbert's managed guard band (method 6), TUR = 2.5, global risks for a 95 % in-tolerance process](examples.md#tur-guard-band-dobbert-tur2-5)
- [Acceptance limits from the test uncertainty ratio: NCSLI RP-10 guard band, TUR = 2.0, global risks for a 95 % in-tolerance process](examples.md#tur-guard-band-rp10-tur2-0)
- [Acceptance limits from the test uncertainty ratio: root-sum-square guard band, TUR = 1.5, global risks for a 95 % in-tolerance process](examples.md#tur-guard-band-rss-tur1-5)

### Censored results ("less than")

Some laboratories report "< 0.015" instead of a number. Three ways of handling them are compared: delete them, ignore the sign, or use half the limit.

- [Censored results in a PT round: censored results deleted (ISO 13528 E.1)](examples.md#iso13528-e1-censored-deleted)
- [Censored results in a PT round: 0.5 × the limit (ISO 13528 E.1)](examples.md#iso13528-e1-censored-half)
- [Censored results in a PT round: ‘<’ ignored (ISO 13528 E.1)](examples.md#iso13528-e1-censored-ignored)

### Homogeneity and stability

Before a proficiency test, the provider must show that all units of the material are the same and stay the same. An analysis of variance on replicate measurements answers this.

- [Arsenic in chocolate: homogeneity and stability of PT items (ISO 13528 E.2)](examples.md#iso13528-e2-arsenic-homogeneity)

### An assigned value from a CRM

The assigned value comes from a comparison with a certified reference material, and its uncertainty is part of the evaluation.

- [Assigned value from a comparison with a CRM: Los Angeles value of aggregates (ISO 13528 E.5)](examples.md#iso13528-e5-los-angeles-reference-value)

### Volume of a storage tank

A product of powers (π r² h). The relative uncertainties combine simply, and the Monte Carlo method confirms the GUM result.

- [Volume of a cylindrical storage tank: product of powers, Gauss's formula and Monte Carlo (Possolo & Meija 2022)](examples.md#possolo-meija-2022-storage-tank-volume)

### A Wheatstone bridge

A model that is not a simple product: the sensitivity coefficients must be derived properly. The program does this symbolically.

- [Wheatstone bridge: a measurement model that is not a product of powers (Possolo & Meija 2022)](examples.md#possolo-meija-2022-wheatstone-bridge)

### Isotopic abundances

The abundances of the silicon isotopes from two measured isotope ratios. The results are correlated because they must add up to 1.

- [Isotopic abundances of silicon from two measured isotope amount ratios (IUPAC 2024, Example 4B)](examples.md#iupac2024-4b-silicon-isotopic-abundances)

### A purity described by a beta distribution

The cadmium standard again, now with the purity as a beta distribution that cannot exceed 100 %. Monte Carlo handles such a distribution naturally.

- [Cadmium calibration standard by Monte Carlo with a beta distribution for the purity (IUPAC 2024, Example 4H; QUAM A1)](examples.md#iupac2024-4h-cadmium-standard-beta-mcm)

### An asymmetric interval below 1

The purity of benzoic acid is close to 1 and cannot exceed it, so its interval is not symmetric. A beta distribution describes it.

- [Purity of benzoic acid (NIST PS1) expressed as a beta distribution: an asymmetric interval below 1 g/g (IUPAC 2024, Example 5C)](examples.md#iupac2024-5c-benzoic-acid-purity-beta)

### End-gauge calibration (GUM H.1)

The famous example of the GUM itself. It uses degrees of freedom for Type B terms and the Welch–Satterthwaite formula to choose the coverage factor.

- [End-gauge calibration (GUM H.1)](examples.md#gum-h1-end-gauge)

### Correlated inputs (GUM H.2)

Voltage, current and phase are measured simultaneously, so their means are correlated. Resistance, reactance and impedance are calculated with and without the correlation terms.

- [Impedance magnitude from correlated V and I (GUM H.2)](examples.md#gum-h2-impedance)
- [Reactance from correlated V, I and φ (GUM H.2)](examples.md#gum-h2-reactance)
- [Resistance from correlated V, I and φ (GUM H.2)](examples.md#gum-h2-resistance)

### Examples of the NIST Uncertainty Machine

Examples from the manual of the NIST Uncertainty Machine: a ratio with a wide triangular divisor, an end gauge, a resistance and the Stefan–Boltzmann constant. They compare GUM and Monte Carlo results.

- [Resistance with the rounded values of GUM Table H.2 (NIST Uncertainty Machine §10)](examples.md#num-10-resistance)
- [Product and ratio y = ab/c with a wide triangular divisor (NIST Uncertainty Machine §3)](examples.md#num-3-ratio-abc)
- [End-gauge calibration with Gaussian inputs (NIST Uncertainty Machine §8)](examples.md#num-8-end-gauge-gaussian)
- [Stefan–Boltzmann constant from CODATA 2010 values (NIST Uncertainty Machine §11)](examples.md#num-11-stefan-boltzmann)

### Calibrating a thermocouple

A thermocouple is compared with a reference thermocouple in a furnace. Two budgets: the temperature of the furnace and the emf of the unknown thermocouple.

- [Thermocouple calibration: emf of the unknown](examples.md#ea402-s5-thermocouple-emf)
- [Thermocouple calibration: furnace temperature](examples.md#ea402-s5-thermocouple-furnace)

### A power sensor at 18 GHz

A microwave power sensor: mismatch between sensor and source is a U-shaped (arcsine) term.

- [Calibration of a power sensor at 18 GHz](examples.md#ea402-s6-power-sensor-18ghz)

### A step attenuator

A 30 dB attenuator. Uncertainties in dB, mismatch terms and the repeatability of connections.

- [Calibration of a coaxial step attenuator (30 dB)](examples.md#ea402-s7-step-attenuator-30db)

### A ring gauge

The diameter of a 90 mm ring gauge with its temperature correction, in two budgets.

- [Calibration of a 90 mm ring gauge](examples.md#ea402-s13-ring-gauge-90mm)
- [Ring gauge calibration: temperature correction](examples.md#ea402-s13-ring-gauge-temperature)

### A power sensor (UKAS M3003)

The British version of the power-sensor budget, with its mismatch term.

- [Coaxial power sensor at 18 GHz (UKAS M3003 K.2)](examples.md#m3003-k2-power-sensor-18ghz)

### An attenuator (UKAS M3003)

A 30 dB attenuator at 10 GHz, with two ways of calculating the mismatch term.

- [30 dB attenuator with the mismatch term from formula E5 (UKAS M3003 K.3, E.9.5)](examples.md#m3003-k3-attenuator-30db-mismatch-e5)
- [30 dB coaxial attenuator at 10 GHz (UKAS M3003 K.3)](examples.md#m3003-k3-attenuator-30db)

### A gauge block (UKAS M3003)

A 10 mm gauge block. One variant writes the temperature effects into the model itself, which shows the difference between a budget table and a measurement model.

- [Grade 2 gauge block of 10 mm with the temperature terms of its model (UKAS M3003 K.6, EA-4/02 S4)](examples.md#m3003-k6-gauge-block-10mm-temperature-model)
- [Grade 2 gauge block of 10 mm (UKAS M3003 K.6)](examples.md#m3003-k6-gauge-block-10mm)

### A thermocouple at 1000 °C

An N-type thermocouple: furnace temperature and emf, with many small contributions.

- [N-type thermocouple at 1000 °C: EMF with its full budget (UKAS M3003 K.7)](examples.md#m3003-k7-thermocouple-emf)
- [Temperature of the furnace in a thermocouple calibration at 1000 °C (UKAS M3003 K.7, first budget)](examples.md#m3003-k7-thermocouple-furnace)

### Flow rate at standard conditions

A volume flow rate converted to standard pressure and temperature: a product and ratio model.

- [Volume flowrate referred to standard pressure and temperature (UKAS M3003 K.9)](examples.md#m3003-k9-flowrate-standard-conditions)

### A PT100 probe

First the probe is calibrated in a dry block; then it measures the temperature of a gas. The uncertainty of the calibration becomes an input of the measurement.

- [Temperature of gas in a container from a calibrated PT100 probe (UKAS M3003 K.11, second budget)](examples.md#m3003-k11-gas-temperature)
- [Calibration of a PT100 probe in a dry block against a reference probe (UKAS M3003 K.11, first budget)](examples.md#m3003-k11-probe-calibration)

### Calibrating a laboratory balance

A 220 g balance according to EURAMET cg-18: the error at several loads, with and without the air buoyancy correction, and the uncertainty of a weighing result in daily use.

- [Laboratory balance 220 g / 0.1 mg: error at 100 g with the buoyancy correction (EURAMET cg-18 H1, option 2)](examples.md#cg18-h1-100g-option2)
- [Laboratory balance 220 g / 0.1 mg: error at 220 g with the buoyancy term for ΔT = 5 K (EURAMET cg-18 H1, option 1, formula 7.1.2-5e)](examples.md#cg18-h1-220g-max-option1-delta-t)
- [Laboratory balance 220 g / 0.1 mg: error at 220 g without buoyancy correction (EURAMET cg-18 H1, option 1)](examples.md#cg18-h1-220g-max-option1)
- [Laboratory balance 220 g / 0.1 mg: error at 220 g with the buoyancy correction (EURAMET cg-18 H1, option 2)](examples.md#cg18-h1-220g-max-option2)
- [Laboratory balance 220 g / 0.1 mg: error at zero load (EURAMET cg-18 H1)](examples.md#cg18-h1-220g-zero)
- [Balance 220 g / 0.1 mg, not adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H1.4/A)](examples.md#cg18-h1-a-weighing-result)
- [Balance 220 g / 0.1 mg, adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H1.4/B)](examples.md#cg18-h1-b-weighing-result)

### A multi-interval balance

A 60 kg scale with three weighing intervals. Each interval has its own resolution, which changes the budget along the load.

- [Multi-interval balance 60 kg: error at 10 kg without buoyancy correction (EURAMET cg-18 H2, option 1)](examples.md#cg18-h2-60kg-10kg-option1)
- [Multi-interval balance 60 kg, adjusted before calibration: error at Max (EURAMET cg-18 H2, second situation)](examples.md#cg18-h2-60kg-max-adjusted)
- [Multi-interval balance 60 kg: error at Max with the buoyancy term for ΔT = 10 K (EURAMET cg-18 H2, option 1, formula 7.1.2-5e)](examples.md#cg18-h2-60kg-max-option1-delta-t)
- [Multi-interval balance 60 kg: error at Max without buoyancy correction (EURAMET cg-18 H2, option 1)](examples.md#cg18-h2-60kg-max-option1)
- [Multi-interval balance 60 kg: error at Max with the buoyancy correction (EURAMET cg-18 H2, option 2)](examples.md#cg18-h2-60kg-max-option2)
- [Multi-interval balance 60 kg: error at zero load (EURAMET cg-18 H2)](examples.md#cg18-h2-60kg-zero)
- [Multi-interval scale 12/30/60 kg, not adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H2.4/A)](examples.md#cg18-h2-a-weighing-result)
- [Multi-interval scale 12/30/60 kg, adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H2.4/B)](examples.md#cg18-h2-b-weighing-result)

### A weighbridge

A 30 t weighbridge tested with standard weights and substitution loads, as in real calibrations of large scales.

- [Weighbridge 30 t: error at 10 t of standard weights (EURAMET cg-18 H3)](examples.md#cg18-h3-30t-10t-weights)
- [Weighbridge 30 t: error at 15 t with a substitution load (EURAMET cg-18 H3)](examples.md#cg18-h3-30t-15t-substitution)
- [Weighbridge 30 t: error at Max with two substitution loads (EURAMET cg-18 H3)](examples.md#cg18-h3-30t-max-substitution)
- [Weighbridge 30 t: error at zero load (EURAMET cg-18 H3)](examples.md#cg18-h3-30t-zero)
- [Weighbridge 30 t / 10 kg, not adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H3.4/A)](examples.md#cg18-h3-a-weighing-result)
- [Weighbridge 30 t / 10 kg, adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H3.4/B)](examples.md#cg18-h3-b-weighing-result)

### A falling-ball viscometer

Viscosity from the fall time of a ball. A model with ratios of densities and times.

- [Falling-ball viscometer (NIST TN 1900 E3)](examples.md#nist-tn1900-e3-falling-ball-viscometer)

### Airspeed with a Pitot tube

The speed follows from a pressure difference through a square root. This example also contains a published inconsistency between the equation and the printed result (see the notes).

- [Airspeed with a Pitot tube (NIST TN 1900 E4)](examples.md#nist-tn1900-e4-pitot-tube)

### Molecular weight of CO₂

From the atomic weights of carbon and oxygen, which are given as intervals.

- [Molecular weight of carbon dioxide (NIST TN 1900 E8)](examples.md#nist-tn1900-e8-molecular-weight-co2)

### A cadmium standard by Monte Carlo

NIST's version of the Eurachem cadmium example, evaluated with the Monte Carlo method.

- [Cadmium calibration standard, Monte Carlo (NIST TN 1900 E9)](examples.md#nist-tn1900-e9-cadmium-standard)

### U-shaped inputs

A microwave step attenuator whose mismatch terms are U-shaped; GUM and Monte Carlo are compared.

- [Microwave step attenuator, U-shaped inputs (NIST TN 1900 E11)](examples.md#nist-tn1900-e11-step-attenuator)

### Normal or t inputs, 95 % or 99 %

The thermal expansion coefficient evaluated four times: with normal or Student t inputs, and for 95 % or 99 % coverage. See how both choices change the interval.

- [Thermal expansion coefficient, Gaussian inputs, 95 % (NIST TN 1900 E13)](examples.md#nist-tn1900-e13-thermal-expansion-gaussian-95)
- [Thermal expansion coefficient, Gaussian inputs, 99 % (NIST TN 1900 E13)](examples.md#nist-tn1900-e13-thermal-expansion-gaussian-99)
- [Thermal expansion coefficient, Student t₃ inputs, 95 % (NIST TN 1900 E13)](examples.md#nist-tn1900-e13-thermal-expansion-student-95)
- [Thermal expansion coefficient, Student t₃ inputs, 99 % (NIST TN 1900 E13)](examples.md#nist-tn1900-e13-thermal-expansion-student-99)

### Mass calibration: when the first order is not enough

The mass calibration of JCGM 101 with the first-order GUM, with second-order terms and with Monte Carlo. A small nonlinearity makes the first-order uncertainty too small.

- [Mass calibration (GUM with second-order terms)](examples.md#jcgm101-9.3-mass-calibration-higher-order)
- [Mass calibration (first-order GUM)](examples.md#jcgm101-9.3-mass-calibration)

### Two outputs from the same inputs

Two results calculated from shared inputs are correlated. JCGM 102 gives their covariance matrix and a coverage region (an ellipse) instead of two separate intervals.

- [Additive bivariate model, Gaussian inputs (JCGM 102 9.2.2)](examples.md#jcgm102-9.2.2-additive-gaussian)
- [Additive bivariate model, rectangular common effect (JCGM 102 9.2.3)](examples.md#jcgm102-9.2.3-additive-rectangular)
- [Additive bivariate model, dominant rectangular common effect (JCGM 102 9.2.4)](examples.md#jcgm102-9.2.4-additive-dominant-rectangular)

### pH by interpolation

The pH of a solution from two buffer standards, at 25 °C and at another temperature. The variants show what happens when the certified values of the standards are correlated.

- [pH at 23.7 °C by three linear interpolations, certified values of each standard correlated across temperatures (JCGM GUM-5 Example 1)](examples.md#gum5-e1-ph-temperature-interpolation-correlated)
- [pH at 23.7 °C by three linear interpolations, independent certified values (JCGM GUM-5 Example 1)](examples.md#gum5-e1-ph-temperature-interpolation)
- [pH by two-point interpolation with fully correlated pH standards (JCGM GUM-5 Example 1)](examples.md#gum5-e1-ph-two-point-25c-correlated-standards)
- [pH of a test solution by two-point interpolation at 25 °C (JCGM GUM-5 Example 1)](examples.md#gum5-e1-ph-two-point-25c)

### A greenhouse gas inventory

Emissions from agricultural machinery. When the same activity data are used for several gases, the results are correlated and the total uncertainty changes.

- [Greenhouse gas inventory, mobile machinery in agriculture (UK 2018): activity data shared by CH₄ and N₂O (JCGM GUM-5 Example 5)](examples.md#gum5-e5-ghg-inventory-agriculture-shared-activity)
- [Greenhouse gas inventory, mobile machinery in agriculture (UK 2018): independent inputs (JCGM GUM-5 Example 5)](examples.md#gum5-e5-ghg-inventory-agriculture)

### Breaking force from destructive tests

Six wire ropes are pulled until they break. Only six data: a t distribution and a careful look at the Monte Carlo scale.

- [Breaking force of steel wire rope from six destructive tests (JCGM GUM-5 Example 15)](examples.md#gum5-e15-wire-rope-breaking-force)

### Vacuum gauge: what to do with a known correction

A correction factor can be applied, ignored, or put into the uncertainty. The three scenarios and the effect of a common temperature error on a pressure difference are compared.

- [Vacuum gauge calibration, correction factor and conformance probability, scenario 1 (EMUE E2.2)](examples.md#emue-e2.2-vacuum-gauge-correction-applied)
- [Vacuum gauge calibration, correction factor and conformance probability, scenario 3 (EMUE E2.2)](examples.md#emue-e2.2-vacuum-gauge-correction-ignored)
- [Vacuum gauge calibration, correction factor and conformance probability, scenario 2 (EMUE E2.2)](examples.md#emue-e2.2-vacuum-gauge-correction-in-uncertainty)
- [Vacuum gauge, pressure difference with a common temperature error: correlation included (EMUE E2.2.7)](examples.md#emue-e2.2-vacuum-gauge-sum-model-correlated)
- [Vacuum gauge, pressure difference with a common temperature error: correlation ignored (EMUE E2.2.7)](examples.md#emue-e2.2-vacuum-gauge-sum-model-correlation-ignored)

### Lead in soil from validation data

An ISO 21748 budget built from the validation of an ICP-OES method: precision, trueness and further terms.

- [Aqua regia extractable lead in soil by ICP-OES: ISO 21748 budget from validation data, root-mean-square bias uncertainty (EMUE E3.4)](examples.md#emue-e3.4-soil-lead-iso21748-rms-bias)
- [Aqua regia extractable lead in soil by ICP-OES: ISO 21748 budget from validation data (EMUE E3.4)](examples.md#emue-e3.4-soil-lead-iso21748)

### A gas mixture by permeation

The amount fraction of ammonia from a permeation tube: mass loss rate, flow and molar masses.

- [Calibration gas mixture of NH₃ in N₂ by permeation: amount fraction of ammonia (EMUE E3.8)](examples.md#emue-e3.8-ammonia-permeation)

### Anti-doping: ephedrine in urine

A top-down uncertainty from intermediate precision and bias, and the decision limit used by WADA to declare an adverse finding.

- [Ephedrine in urine: WADA decision limit as a guarded-rejection rule (EMUE E6.5, JCGM 106)](examples.md#emue-e6.5-ephedrine-wada-decision-limit)
- [Ephedrine in urine for anti-doping testing: top-down uncertainty from intermediate precision and a CRM (EMUE E6.5)](examples.md#emue-e6.5-ephedrine-wada-topdown)

### Uncertainty from reproducibility data

When a standard method has been tested in an interlaboratory study, its reproducibility standard deviation is a good first estimate of the uncertainty (ISO 21748). The examples add further terms where needed.

- [Meat content from nitrogen and fat (ISO 21748 C.2)](examples.md#iso21748-c2-meat-content)
- [Crude fibre in animal feed from reproducibility data (ISO 21748 C.4)](examples.md#iso21748-c4-crude-fibre)
- [Carbon monoxide in vehicle emissions from reproducibility data (ISO 21748 C.1)](examples.md#iso21748-c1-co-emissions)
- [Aerobic plate count, flour: adjusted reproducibility (ISO 21748 C.3)](examples.md#iso21748-c3-aerobic-plate-count-flour)
- [Aerobic plate count, shrimp: adjusted reproducibility (ISO 21748 C.3)](examples.md#iso21748-c3-aerobic-plate-count-shrimp)
- [Aerobic plate count, vegetables: adjusted reproducibility (ISO 21748 C.3)](examples.md#iso21748-c3-aerobic-plate-count-vegetables)

### An interlaboratory precision study

Eight or more laboratories, several levels, replicate results: repeatability and reproducibility of a method, after Cochran's and Grubbs's tests. Each example is evaluated classically and with REML.

- [Sulfur content of coal: REML estimates (ISO 5725-2 Example C.1)](examples.md#iso5725-2-c1-sulfur-in-coal-reml)
- [Sulfur content of coal: repeatability and reproducibility (ISO 5725-2 Example C.1)](examples.md#iso5725-2-c1-sulfur-in-coal)
- [Thermometric titration of creosote oil: REML estimates (ISO 5725-2 Example C.3)](examples.md#iso5725-2-c3-creosote-oil-reml)
- [Thermometric titration of creosote oil: outlying laboratory and precision relationships (ISO 5725-2 Example C.3)](examples.md#iso5725-2-c3-creosote-oil)
- [Softening point of pitch: REML estimates (ISO 5725-2 Example C.2)](examples.md#iso5725-2-c2-softening-point-pitch-reml)
- [Softening point of pitch: repeatability and reproducibility with missing data (ISO 5725-2 Example C.2)](examples.md#iso5725-2-c2-softening-point-pitch)

### A straight-line calibration

Nitrogen monoxide by UV absorption calibrated with a straight line through reference gas mixtures, with uncertainties on both axes.

- [NO in N₂ by UV absorption: straight-line analysis function (ISO 6143 Example 1)](examples.md#iso6143-b1-no-uv-linear)

### Purity by qNMR (TÜBİTAK UME)

An illustrative budget for quantitative NMR with an internal standard, from the TÜBİTAK UME uncertainty procedure.

- [Purity by ¹H qNMR with an internal standard: an illustrative budget (TÜBİTAK UME uncertainty procedure, Example 19.2)](examples.md#ume-procedure-19-2-qnmr-purity-internal-standard)

### Sampling: nitrate in lettuce

The duplicate method: two samples from each of eight lots, each analysed twice. The sampling part of the uncertainty is larger than the analytical part.

- [Nitrate in glasshouse lettuce: duplicate method, classical ANOVA (Eurachem UfS Example A1)](examples.md#ufs-a1-nitrate-lettuce)

### Sampling: lead in soil

A large relative uncertainty is better expressed as a factor (multiply or divide) on a log scale.

- [Lead in contaminated top soil: classical ANOVA and uncertainty factor (Eurachem UfS Example A2)](examples.md#ufs-a2-lead-soil-uncertainty-factor)

### Sampling: iron in groundwater

The same data evaluated with the analysis of variance and with the simpler range method.

- [Dissolved iron in groundwater: classical ANOVA (Eurachem UfS Example A3)](examples.md#ufs-a3-iron-groundwater-anova)
- [Dissolved iron in groundwater: range method (Eurachem UfS Example A3, corrected)](examples.md#ufs-a3-iron-groundwater-range)

### Sampling: vitamin A in porridge

Small test portions make the sampling variance large; with 4 g portions the estimated analytical variance becomes negative, which shows the limits of the method. A control chart of duplicates is included.

- [Vitamin A in baby porridge, 40 g test samples: duplicate method (Eurachem UfS Example A4)](examples.md#ufs-a4-vitamin-a-porridge-40g)
- [Vitamin A in baby porridge, 4 g test samples: negative sampling variance (Eurachem UfS Example A4)](examples.md#ufs-a4-vitamin-a-porridge-4g)
- [Vitamin A in baby porridge: control chart of duplicate samples (Eurachem UfS Example A4)](examples.md#ufs-a4-vitamin-a-qc-chart)

### Sampling uncertainty by modelling

Instead of duplicates, the sampling steps are modelled one by one, as in a GUM budget.

- [Cadmium in agricultural top soil: modelling approach (Eurachem UfS Example A6)](examples.md#ufs-a6-cadmium-soil)
- [Phosphorus in agricultural top soil: modelling approach (Eurachem UfS Example A6)](examples.md#ufs-a6-phosphorus-soil)

<a id="level-3"></a>

## Level 3: Advanced

### Global risks of false decisions

With knowledge of the production process, the probability that an accepted item is actually bad (consumer's risk) can be calculated for the whole production, and an acceptance limit chosen for a required risk.

- [Ball bearings: acceptance limit for a required consumer's risk (JCGM 106 9.5.4)](examples.md#jcgm106-9.5-ball-bearings)
- [Precision resistors: global consumer's and producer's risks (JCGM 106 9.5.3)](examples.md#jcgm106-9.5-precision-resistors)

### Several components must conform

A medicine with four correlated active components: the risks of a false decision are calculated jointly by Monte Carlo.

- [Total risks of false decisions for a medication with four correlated active components (EMUE E2.1)](examples.md#emue-e2.1-multicomponent-medication)

### Global and specific risks with a prior

Particulate matter in air: prior knowledge of typical concentrations (a lognormal distribution) is combined with the measurement to calculate the risks of a decision.

- [Suspended particulate matter in air: global risks with a lognormal prior, A_U = 0.17 mg/m³ (JCGM GUM-5 Example 10)](examples.md#gum5-e10-tspm-global-risks-acceptance-0.17)
- [Suspended particulate matter in air: global risks with a lognormal prior, A_U = 0.22 mg/m³ (JCGM GUM-5 Example 10)](examples.md#gum5-e10-tspm-global-risks-acceptance-0.22)
- [Suspended particulate matter in air: global risks with a lognormal prior, A_U = 0.2 mg/m³ (JCGM GUM-5 Example 10)](examples.md#gum5-e10-tspm-global-risks-shared-risk)
- [Suspended particulate matter in air: specific risks from the posterior with a lognormal prior (JCGM GUM-5 Example 10)](examples.md#gum5-e10-tspm-specific-risks)

### A strongly nonlinear model

The comparison loss in microwave power: near zero the model is so nonlinear that the first-order GUM fails completely, while Monte Carlo gives the right answer. The variants add correlation and higher-order terms.

- [Comparison loss, x₁ = 0, r = 0.9, higher-order GUM terms for correlated inputs](examples.md#jcgm101-9.4-comparison-loss-x000-correlated-higher-order)
- [Comparison loss, x₁ = 0.0, r = 0.9](examples.md#jcgm101-9.4-comparison-loss-x000-correlated)
- [Comparison loss, x₁ = 0.0](examples.md#jcgm101-9.4-comparison-loss-x000)
- [Comparison loss, x₁ = 0.01, r = 0.9, higher-order GUM terms for correlated inputs](examples.md#jcgm101-9.4-comparison-loss-x010-correlated-higher-order)
- [Comparison loss, x₁ = 0.01, r = 0.9](examples.md#jcgm101-9.4-comparison-loss-x010-correlated)
- [Comparison loss, x₁ = 0.01](examples.md#jcgm101-9.4-comparison-loss-x010)
- [Comparison loss, x₁ = 0.05, r = 0.9, higher-order GUM terms for correlated inputs](examples.md#jcgm101-9.4-comparison-loss-x050-correlated-higher-order)
- [Comparison loss, x₁ = 0.05, r = 0.9](examples.md#jcgm101-9.4-comparison-loss-x050-correlated)
- [Comparison loss, x₁ = 0.05](examples.md#jcgm101-9.4-comparison-loss-x050)

### Gauge block: exact and approximate models

The full nonlinear gauge-block model of JCGM 101 compared with its linear approximation, by GUM and Monte Carlo.

- [Gauge block calibration, approximate model](examples.md#jcgm101-9.5-gauge-block-approximate)
- [Gauge block calibration, non-linear model](examples.md#jcgm101-9.5-gauge-block-nonlinear)

### Cartesian to polar coordinates

A magnitude and a phase from two coordinates. Near the origin the transformation is strongly nonlinear and the coverage region is not an ellipse.

- [Cartesian to polar co-ordinates, x₁ = 0.001, r = 0.9 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x001-correlated)
- [Cartesian to polar co-ordinates, x₁ = 0.001 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x001)
- [Cartesian to polar co-ordinates, x₁ = 0.01, r = 0.9 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x010-correlated)
- [Cartesian to polar co-ordinates, x₁ = 0.01 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x010)
- [Cartesian to polar co-ordinates, x₁ = 0.1, r = 0.9 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x100-correlated)
- [Cartesian to polar co-ordinates, x₁ = 0.1 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x100)

### Resistance and reactance from simultaneous data

Three outputs from correlated indication values; the alternative uses a multivariate t distribution for the few data.

- [Resistance, reactance and impedance from simultaneous indication values, t covariance (JCGM 102 9.4)](examples.md#jcgm102-9.4-resistance-reactance-alternative)
- [Resistance, reactance and impedance from simultaneous indication values (JCGM 102 9.4)](examples.md#jcgm102-9.4-resistance-reactance)

### Implicit models: a resistance thermometer

The temperature cannot be written as a formula of the resistance, so the equation is solved numerically. With ten temperatures from one thermometer, the results are strongly correlated.

- [Celsius temperature from a resistance thermometer, implicit model (JCGM 102 9.5.2)](examples.md#jcgm102-9.5.2-resistance-thermometer)
- [Ten Celsius temperatures with one resistance thermometer, implicit model (JCGM 102 9.5.3)](examples.md#jcgm102-9.5.3-ten-temperatures)

### Peak areas from the same injections

Benzo[a]pyrene by GC-MS with an internal standard. Two areas measured in the same injections are correlated; the variant treats them as an ensemble when counting degrees of freedom.

- [Benzo[a]pyrene: peak areas of the same injections as an ensemble in ν_eff (JCGM GUM-5 Example 2, Willink 2007)](examples.md#gum5-e2-benzo-a-pyrene-ensemble)
- [Benzo[a]pyrene by GC-MS with an internal standard, correlated inputs (JCGM GUM-5 Example 2)](examples.md#gum5-e2-benzo-a-pyrene)

### A gravimetric gas mixture

Eight amount fractions of a calibration gas mixture prepared by weighing: many inputs, intermediate quantities and correlated outputs.

- [Gravimetric preparation of a CO₂ in N₂ calibration gas mixture: amount fractions of eight components and their correlations (JCGM GUM-5 Example 4)](examples.md#gum5-e4-gravimetric-gas-mixture)

### Calibrating a torque sensor

A straight line through the origin fitted by ordinary and weighted least squares, with the GUM uncertainty of the slope.

- [Torque sensor calibration, straight line through the origin, ordinary least squares with the GUF (JCGM GUM-5 Example 9)](examples.md#gum5-e9-torque-slope-ols)
- [Torque sensor calibration, straight line through the origin, weighted least squares with the GUF (JCGM GUM-5 Example 9)](examples.md#gum5-e9-torque-slope-wls)

### Between-bottle homogeneity with Bayes

When the between-bottle variance is small, the classical estimate can be zero or negative; a hierarchical Bayesian model gives a sensible value.

- [Between-bottle homogeneity of nitrogen in natural gas mixtures: ANOVA, u*_bb and a Bayesian hierarchical model (JCGM GUM-5 Example 12)](examples.md#gum5-e12-between-bottle-homogeneity-bayes)

### Homogeneity of a soil reference material

Twenty bottles measured in triplicate, evaluated by the analysis of variance and by a hierarchical Bayesian model.

- [Chromium in a soil reference material, 20 bottles in triplicate: ANOVA and a hierarchical Bayesian model (Possolo & Bodnar 2018; ISO Guide 35 C.1 data)](examples.md#possolo-bodnar-2018-chromium-soil-homogeneity-bayes)

### Atomic weight from isotope ratios

The atomic weight and isotopic composition of lead from three measured isotope ratios: correlated outputs from a nonlinear model.

- [Atomic weight and isotopic composition of lead in NRC HIPB-1 from three isotope ratios (Possolo & Meija 2022)](examples.md#possolo-meija-2022-atomic-weight-lead-hipb1)

### A consensus value with heavy tails

Twenty-two inconsistent estimates of the Hubble–Lemaître constant combined with a Bayesian model whose laboratory effects follow a Laplace distribution, which is less influenced by outliers.

- [Hubble–Lemaître constant from 22 mutually inconsistent estimates: Bayesian consensus with Laplace effects (Possolo & Meija 2022)](examples.md#possolo-meija-2022-hubble-lemaitre-constant-laplace)

### An implicit chemical model

The Mohr titration with its endpoint chemistry: the solubility equilibrium gives an implicit equation that the program solves.

- [Argentometric titration (Mohr) with the endpoint chemistry: implicit model with the solubility products of AgCl and Ag₂CrO₄ (IUPAC 2024, Examples 3A and 4I)](examples.md#iupac2024-3a-argentometric-titration-ksp)

### Three inconsistent determinations

The molar mass of iridium from three determinations that disagree. Laboratory effects are modelled with a hierarchical Bayesian model.

- [Molar mass of iridium from three mutually inconsistent determinations: laboratory effects and a Bayesian random-effects model (IUPAC 2024, Examples 3E, 4M and 4N)](examples.md#iupac2024-3e-4m-iridium-molar-mass-bayes)

### A key comparison

CCQM-K145, nickel in bovine liver: a consensus value with dark uncertainty by maximum likelihood. Participants are coded.

- [Key comparison CCQM-K145, nickel in bovine liver: consensus value with dark uncertainty by maximum likelihood (IUPAC 2024, Example 4F)](examples.md#iupac2024-4f-ccqm-k145-nickel-bovine-liver)

### Consensus values of discrepant data

Half-lives and the gravitational constant: measurements that disagree far more than their uncertainties. DerSimonian–Laird, Linear Pool and hierarchical Bayes give different answers; the choice is part of the evaluation.

- [Half-life of ¹³⁷Cs: consensus value of 19 discrepant measurements (DerSimonian–Laird and Linear Pool; Koepke et al. 2017)](examples.md#koepke2017-half-life-cs137)
- [Half-life of ⁹⁰Sr: consensus value of 11 discrepant measurements (DerSimonian–Laird and Linear Pool; Koepke et al. 2017)](examples.md#koepke2017-half-life-sr90)
- [Newtonian constant of gravitation: consensus value of 14 measurements (DerSimonian–Laird and Linear Pool; Koepke et al. 2017)](examples.md#koepke2017-newtonian-constant-g)

### Shades of dark uncertainty

Sixteen measurements of the Newtonian constant of gravitation evaluated with several procedures; the consensus value and its uncertainty depend strongly on the procedure.

- [Newtonian constant of gravitation from 16 measurements: DerSimonian–Laird with Knapp–Hartung, maximum likelihood, hierarchical Bayes and Linear Pool (Merkatas et al. 2019)](examples.md#merkatas2019-newtonian-constant-g-shades)

### Degrees of equivalence in a comparison

The examples of the NIST Consensus Builder: a consensus value from laboratories with degrees of freedom, and leave-one-out and bilateral degrees of equivalence of 19 laboratories.

- [PCB 28 in sediment from six laboratories with degrees of freedom: DerSimonian–Laird, Linear Pool and leave-one-out degrees of equivalence (NIST Consensus Builder §4.2)](examples.md#nicob-pcb28-sediment-degrees-of-freedom)
- [Activity of ⁶⁰Co in 19 laboratories: consensus value with leave-one-out and bilateral degrees of equivalence (NIST Consensus Builder §4.4)](examples.md#nicob-co60-activity-leave-one-out-bilateral)

### Intermediate precision and nested designs

Precision when time and operator change, and the staggered-nested design that separates several factors with few measurements.

- [Carbon in steel: [time + operator]-different intermediate precision (ISO 5725-3 J.1)](examples.md#iso5725-3-j1-carbon-steel)
- [Vanadium in steel: staggered-nested design, level 1 (ISO 5725-3 J.2)](examples.md#iso5725-3-j2-vanadium-steel)

### Calibration functions with uncertainty on both axes

Gas analysis with straight-line, quadratic, power and exponential functions, and reference mixtures whose contents are correlated. The program tests which function fits.

- [N₂ in natural gas by GC-TCD: correlated reference mixtures (ISO 6143 Example 2)](examples.md#iso6143-b2-n2-natural-gas-covariances)
- [N₂ in natural gas by GC-TCD: straight line (ISO 6143 Example 2)](examples.md#iso6143-b2-n2-natural-gas-linear)
- [N₂ in natural gas by GC-TCD: second-order polynomial (ISO 6143 Example 2)](examples.md#iso6143-b2-n2-natural-gas-quadratic)
- [NO in N₂ by NDIR: exponential function (ISO 6143 Example 3)](examples.md#iso6143-b3-no-ndir-exponential)
- [NO in N₂ by NDIR: power function (ISO 6143 Example 3)](examples.md#iso6143-b3-no-ndir-power)

### An analyser calibrated by dynamic dilution

The reference mixtures are made from the same parent gas, so their contents are correlated. Ignoring the covariances changes the result.

- [NO₂ analyser calibrated with mixtures from dynamic dilution: analysis function with correlated reference values, covariances ignored (EMUE E3.3)](examples.md#emue-e3.3-no2-analyser-dynamic-dilution-covariances-ignored)
- [NO₂ analyser calibrated with mixtures from dynamic dilution: analysis function with correlated reference values (EMUE E3.3)](examples.md#emue-e3.3-no2-analyser-dynamic-dilution)

### An error approximation function

A straight line fitted through the errors of a balance at many loads, with a χ² test and a residual test deciding how large an extra term s_m must be.

- [Balance 400 g / 0.1 mg: error approximation function with s_m = 0.05 mg, χ² test passed (EURAMET cg-18 H4)](examples.md#cg18-h4-400g-error-function-sm-0.05)
- [Balance 400 g / 0.1 mg: error approximation function with s_m = 0.25 mg, residual test passed (EURAMET cg-18 H4)](examples.md#cg18-h4-400g-error-function-sm-0.25)
- [Balance 400 g / 0.1 mg: error approximation function E = a₁I, first fit (EURAMET cg-18 H4)](examples.md#cg18-h4-400g-error-function)

### Matrix effects in a top-down model

Seven elements in marine sediments: recoveries from reference materials of different matrices, and a model with two concentration intervals.

- [Chromium in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)](examples.md#palma2019-cr-marine-sediment)
- [Copper in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)](examples.md#palma2019-cu-marine-sediment)
- [Lithium in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)](examples.md#palma2019-li-marine-sediment)
- [Manganese in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)](examples.md#palma2019-mn-marine-sediment)
- [Nickel in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)](examples.md#palma2019-ni-marine-sediment)
- [Lead in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)](examples.md#palma2019-pb-marine-sediment)
- [Zinc in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)](examples.md#palma2019-zn-marine-sediment)

### Arithmetic or weighted mean recovery

Fluoranthene in river sediments: the mean recovery calculated in two ways, as in the MUPR guide and its spreadsheet.

- [Fluoranthene in river sediments, arithmetic mean recovery (MUPR Eq. 8–12) (Bettencourt da Silva 2026)](examples.md#bettencourt2026-fluoranthene-arithmetic)
- [Fluoranthene in river sediments, weighted mean recovery (MUPR Eq. 16.1) (Bettencourt da Silva 2026)](examples.md#bettencourt2026-fluoranthene-weighted)

### Purity by quantitative NMR (BIPM)

The reference data of the BIPM for internal standards in qNMR. Complete purity budgets; one result lies close to the physical limit of 1 and is reported with an asymmetric interval.

- [Purity of dimethyl sulfone by ¹H qNMR with maleic acid as internal standard in D2O (BIPM ISRD-01, Table 4)](examples.md#bipm-isrd-01-qnmr-dmso2-ma-d2o)
- [Purity of dimethyl terephthalate by ¹H qNMR with maleic acid as internal standard in DMSO-d6 (BIPM ISRD-01, Table 6)](examples.md#bipm-isrd-01-qnmr-dmtp-ma-dmso-d6)
- [Purity of dimethyl terephthalate by ¹H qNMR with dimethyl sulfone as internal standard in CDCl3 (BIPM ISRD-02, Table 6)](examples.md#bipm-isrd-02-qnmr-dmtp-dmso2-cdcl3)
- [Purity of maleic acid by ¹H qNMR with dimethyl sulfone as internal standard in D2O (BIPM ISRD-02, Table 4)](examples.md#bipm-isrd-02-qnmr-ma-dmso2-d2o)
- [Purity of benzoic acid by ¹H qNMR with BTMSB-d4 as internal standard in CD3OD (BIPM ISRD-06, Table 6)](examples.md#bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od)
- [Purity of dimethyl sulfone by ¹H qNMR with BTMSB-d4 as internal standard in DMSO-d6 (BIPM ISRD-06, Table 4)](examples.md#bipm-isrd-06-qnmr-dmso2-btmsb-d4-dmso-d6)
- [Purity of dimethyl sulfone by ¹H qNMR with DSS-d6 as internal standard in D2O (BIPM ISRD-07, Table 4)](examples.md#bipm-isrd-07-qnmr-dmso2-dss-d6-d2o)
- [Purity of maleic acid by ¹H qNMR with DSS-d6 as internal standard in D2O (BIPM ISRD-07, Table 6)](examples.md#bipm-isrd-07-qnmr-ma-dss-d6-d2o)
- [Purity of BTMSB-d4 by ¹H qNMR with benzoic acid as internal standard in CD3OD (BIPM ISRD-08, Table 2)](examples.md#bipm-isrd-08-qnmr-btmsb-d4-ba-cd3od)

### Isotope dilution (IDMS)

Lead in water by double isotope dilution: a long model with isotope ratios, masses and a calibration standard. A reference method with a small uncertainty.

- [Lead assay standard for IDMS](examples.md#quam-a7-assay-standard)
- [Lead in water by double isotope dilution](examples.md#quam-a7-lead-double-idms)

### Gy's sampling theory

The fundamental sampling error of an enzyme in chicken feed, estimated from particle size and properties.

- [Enzyme in chicken feed: fundamental sampling error by Gy's theory (Eurachem UfS Example A5)](examples.md#ufs-a5-enzyme-feed-gy)
