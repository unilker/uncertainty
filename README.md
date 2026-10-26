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
- [Example library](library/README.md): 96 published examples (57 model-based:
  chemistry, mass, length, temperature, electrical, flow, physical properties;
  29 top-down: Nordtest, MUPR, QUAM A4/A6, ISO 21748, ISO 5725-3; 4 proficiency
  testing: ISO 13528; 6 calibrations: ISO 6143) as self-checking projects.

## Usage / Kullanım

```sh
cargo build --release
B=target/release/uncertainty
$B template > my.json                                   # start a project / proje başlat
$B template --topdown > td.json                         # top-down project from raw data
$B template --proficiency > pt.json                     # proficiency test round (ISO 13528)
$B template --calibration > cal.json                    # calibration in both variables (ISO 6143)
$B run library/chemistry/quam-a1-cadmium-standard.json  # Markdown report (English)
$B run my.json --lang tr -o rapor.html                  # Türkçe HTML rapor
$B run my.json --format json --no-mcm                   # machine-readable results
$B check library                                        # re-verify all examples / tüm örnekleri doğrula
```

## Graphical interface / Grafik arayüz

[app/](app/README.md) — Flutter application (Linux, Windows and Android packages built and tested by the
Package workflow; macOS and iOS next) using the same Rust engine through a C interface
([ffi/](ffi/src/lib.rs)): library browser, model, top-down and proficiency-test editors with direct entry of
raw data, a calibration editor (ISO 6143), results with budget, charts and validation, HTML/Markdown report export, English and Turkish.

Flutter uygulaması (Linux, Windows ve Android paketleri Package iş akışıyla derlenir ve test edilir; macOS
ve iOS sırada); aynı Rust motorunu kullanır: örnek kütüphanesi, ham veri girişli model, top-down,
yeterlilik testi ve kalibrasyon (ISO 6143) düzenleyicileri, bütçe, grafikler ve doğrulamayla sonuçlar, HTML/Markdown rapor, Türkçe ve
İngilizce.

## User guide / Kullanım rehberi

[docs/guide](docs/guide/README.md) — in English and Turkish: getting started, the GUM uncertainty framework,
the Monte Carlo method, top-down evaluation, reporting, the project file reference, an English–Turkish
glossary, proficiency testing, calibration in both variables and all library examples with their results recomputed by the program.

İngilizce ve Türkçe: başlarken, GUM belirsizlik yaklaşımı, Monte Carlo yöntemi, top-down değerlendirme,
raporlama, proje dosyası başvurusu, İngilizce–Türkçe sözlük, yeterlilik testi, iki değişkenli kalibrasyon ve sonuçları program tarafından yeniden
hesaplanan tüm kütüphane örnekleri.

## Scientific validation / Bilimsel doğrulama

Every published reference example implemented so far is reproduced by an
automated test (123 tests; one of them reruns all 96 library projects): GUM H.1 and H.2, all four examples of JCGM 101 clause 9,
EURACHEM CG4 A1–A7, Nordtest TR 537 and Eurachem MUPR 2026 (recomputed from
their raw data), all twelve calibration examples of EA-4/02 M:2022, eight examples of NIST TN 1900 and the scalar examples of the NIST Uncertainty Machine manual (§3, §7–§11), the worked examples E.2–E.4 of ISO 13528:2022, Annex C of ISO 21748:2017, Annex J of ISO 5725-3:2023 and Annex B of ISO 6143:2025. Details, tolerances and findings:
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
