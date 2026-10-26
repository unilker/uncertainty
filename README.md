# Uncertainty Calculations · Belirsizlik Hesaplamaları

**EN** — Open software for evaluating measurement uncertainty with the GUM
uncertainty framework (JCGM 100:2008) and the Monte Carlo method of GUM
Supplement 1 (JCGM 101:2008), for analytical chemistry, calibration and
accredited laboratories. Desktop and mobile, Turkish and English.

**TR** — Ölçüm belirsizliğini GUM yaklaşımı (JCGM 100:2008) ve GUM Ek 1
Monte Carlo yöntemi (JCGM 101:2008) ile hesaplayan açık yazılım; analitik
kimya, kalibrasyon ve akredite laboratuvarlar için. Masaüstü ve mobil,
Türkçe ve İngilizce.

> Author / Yazar: İlker Ün, TÜBİTAK UME ([ORCID 0000-0001-5089-4602](https://orcid.org/0000-0001-5089-4602))
>
> Status / Durum: **v0.1.0 — computation core, project files, reports, command-line tool, user guide; graphical interface (Linux) in development.**
> See [docs/ROADMAP.md](docs/ROADMAP.md).

## What works now / Şu an çalışanlar

- GUM: first-order law of propagation with correlations, higher-order terms
  (GUM 5.1.2 note), Welch–Satterthwaite ν_eff, coverage factor from the
  t-distribution or fixed k.
- Monte Carlo: JCGM 101 random number generator (enhanced Wichmann–Hill),
  ten distribution types, correlated Gaussian inputs, fixed or **adaptive**
  number of trials (7.9), symmetric and shortest coverage intervals,
  automatic **validation of GUM against MCM** (clause 8).
- Measurement models typed as formulas (`1000 * m * P / V`), with
  intermediate quantities and exact symbolic sensitivity coefficients.
- Top-down (chemistry): within-laboratory reproducibility, bias from CRMs,
  proficiency tests and recovery (Nordtest TR 537); mean recovery, its
  uncertainty and significance, precision models over the working range and
  combination with contributions (Eurachem/CITAC MUPR 2026).
- Calibration curves: straight-line least squares and uncertainty of
  predicted concentrations (EURACHEM CG4 Appendix E.4).
- Coverage factors for dominant rectangular/trapezoidal contributions
  (EA-4/02).
- Reporting: rounding of results by the GUM, MUPR or round-up rule.
- Project files (JSON) with bilingual metadata; reports in English or Turkish
  as Markdown or self-contained HTML (budget, contribution chart, MCM
  histogram with the GUM PDF, validation statement, citation).
- Top-down project files with **raw validation data**: control-chart series,
  replicate groups, duplicate pairs (pooled, normalised differences, mean
  range), CRMs, proficiency-test rounds, spike recoveries; concentration
  ranges, recovery significance test and correction, χ² equivalence of
  recoveries (Nordtest TR 537, Eurachem/CITAC MUPR).
- Proficiency testing (ISO 13528:2022): median, MADe, nIQR, Algorithm A and S,
  assigned value and its uncertainty, D, D %, PA, z, z′, ζ and E_n scores,
  screening of reported uncertainties, homogeneity and stability checks.
- Precision from collaborative studies: s_R-based budgets with the
  laboratory's own repeatability, method-bias uncertainty and bias-control
  check (ISO 21748:2017); staggered-nested designs (ISO 5725-3:2023).
- Calibration with uncertainties in both the contents and the responses
  (ISO 6143:2025): generalized least squares for linear, polynomial, power and
  exponential analysis functions, covariances of correlated standards,
  goodness of fit Γ and comparison of functions, contents of samples with
  their covariances, upper bound of the uncertainty, drift control.
- Conformity assessment (JCGM 106:2012, ISO/IEC 17025 7.8.6): probability of
  conformity (normal, t or Monte Carlo), measurement capability index, simple
  acceptance and guarded acceptance/rejection with guard bands from r, from a
  length or from a required probability, global consumer's and producer's
  risks for normal and gamma process distributions.
- Interlaboratory precision experiments (ISO 5725-2:2019): Mandel's h and k,
  Cochran's and Grubbs' tests with the critical values of Annex D, repeatability,
  between-laboratory and reproducibility standard deviations by Formulae
  (23)–(31) or REML (Annex B.2), documented exclusions, relationships I–IV
  between precision and level, Annex A uncertainty factors.
- Several output quantities (JCGM 102:2011): explicit and implicit models,
  covariance matrices, hyper-ellipsoidal and hyper-rectangular coverage regions
  (GUM and Monte Carlo), the smallest coverage region of two outputs, the adaptive
  procedure and the validation of clause 8; inputs from vector indication values
  (multivariate t-distribution).
- Results not corrected for a known bias: RSSu, RSSU, SUMU, SUMU_MAX and Ue
  intervals (Magnusson & Ellison 2008), with their coverage checked by simulation.
- [Example library](library/README.md): 154 published examples (58 model-based, 13 with several outputs:
  chemistry, mass, length, temperature, electrical, flow, physical properties;
  36 top-down: Nordtest, MUPR, QUAM A4/A6, ISO 21748, ISO 5725-3, Palma et al.
  2019; 7 proficiency
  testing: ISO 13528; 6 calibrations: ISO 6143; 6 interlaboratory precision
  experiments: ISO 5725-2; 7 conformity assessments: JCGM 106; 21 control
  charts: Nordtest TR 569) as self-checking projects.

## Usage / Kullanım

```sh
cargo build --release
B=target/release/uncertainty
$B template > my.json                                   # start a project / proje başlat
$B template --topdown > td.json                         # top-down project from raw data
$B template --proficiency > pt.json                     # proficiency test round (ISO 13528)
$B template --calibration > cal.json                    # calibration in both variables (ISO 6143)
$B template --conformity > cf.json                      # conformity assessment (JCGM 106)
$B template --precision > pe.json                       # interlaboratory precision experiment (ISO 5725-2)
$B template --multivariate > mv.json                    # several output quantities (JCGM 102)
$B run library/chemistry/quam-a1-cadmium-standard.json  # Markdown report (English)
$B run my.json --lang tr -o rapor.html                  # Türkçe HTML rapor
$B run my.json --format json --no-mcm                   # machine-readable results
$B check library                                        # re-verify all examples / tüm örnekleri doğrula
```

## Graphical interface / Grafik arayüz

[app/](app/README.md) — Flutter application (Linux, Windows, Android and macOS packages and an unsigned
iOS build, all produced by the Package workflow) using the same Rust engine through a C interface
([ffi/](ffi/src/lib.rs)): library browser, model, top-down and proficiency-test editors with direct entry of
raw data, a calibration editor (ISO 6143), results with budget, charts and validation, HTML/Markdown report export, English and Turkish.

Flutter uygulaması (Linux, Windows, Android ve macOS paketleri ile imzasız iOS derlemesi Package iş
akışıyla üretilir); aynı Rust motorunu kullanır: örnek kütüphanesi, ham veri girişli model, top-down,
yeterlilik testi ve kalibrasyon (ISO 6143) düzenleyicileri, bütçe, grafikler ve doğrulamayla sonuçlar, HTML/Markdown rapor, Türkçe ve
İngilizce.

## User guide / Kullanım rehberi

[docs/guide](docs/guide/README.md) — in English and Turkish: getting started, the GUM uncertainty framework,
the Monte Carlo method, top-down evaluation, reporting, the project file reference, an English–Turkish
glossary, proficiency testing, calibration in both variables, conformity assessment, interlaboratory precision
experiments, several output quantities and all library examples with their results recomputed by the program.

İngilizce ve Türkçe: başlarken, GUM belirsizlik yaklaşımı, Monte Carlo yöntemi, top-down değerlendirme,
raporlama, proje dosyası başvurusu, İngilizce–Türkçe sözlük, yeterlilik testi, iki değişkenli kalibrasyon, uygunluk değerlendirmesi, laboratuvarlar arası kesinlik deneyleri, birden çok çıktı büyüklüğü, kontrol grafikleri ve sonuçları program tarafından yeniden
hesaplanan tüm kütüphane örnekleri.

## Scientific validation / Bilimsel doğrulama

Every published reference example implemented so far is reproduced by an
automated test (163 tests; one of them reruns all 154 library projects): GUM H.1 and H.2, all four examples of JCGM 101 clause 9, the examples of JCGM 102 clauses 6, 7 and 9,
EURACHEM CG4 A1–A7, Nordtest TR 537 and Eurachem MUPR 2026 (recomputed from
their raw data), all twelve calibration examples of EA-4/02 M:2022, eight examples of NIST TN 1900 and the scalar examples of the NIST Uncertainty Machine manual (§3, §7–§11), the worked examples E.1–E.5 of ISO 13528:2022, Annex C of ISO 21748:2017, Annex J of ISO 5725-3:2023 (with the data of the 1994 edition), Annex C of ISO 5725-2:2019 (including REML) and its Tables 1–8, Annex B of ISO 6143:2025, the examples of JCGM 106:2012 clauses 7–9 and Table 1, Chapter 7 and Examples 1–12 of Nordtest TR 569 ed. 6. Details, tolerances and findings:
[docs/research/validation-matrix.md](docs/research/validation-matrix.md).

```sh
cargo test                     # unit + validation tests
cargo run --release --example mcm_dispersion   # Monte Carlo dispersion study
```

## Research notes / Araştırma notları

The project is developed as a scientific publication; decisions, findings
and sources are recorded in [docs/research/](docs/research/).

## Licence and citation / Lisans ve atıf

GPL-3.0-or-later with additional attribution terms (GPL §7) — see
[LICENSE](LICENSE), [ADDITIONAL-TERMS.md](ADDITIONAL-TERMS.md), [NOTICE](NOTICE).
If you use this software or its results, **please cite it** as given in
[CITATION.cff](CITATION.cff).

Bu yazılımı veya sonuçlarını kullanıyorsanız lütfen
[CITATION.cff](CITATION.cff) dosyasındaki şekilde **atıf yapınız**.
