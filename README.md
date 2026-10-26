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
> Status / Durum: **v0.1.0 — computation core, project files, reports and command-line tool** (graphical interface in development).
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
- [Example library](library/README.md): 60 published examples (38 model-based:
  chemistry, mass, length, temperature, electrical, flow; 22 top-down:
  Nordtest, MUPR, QUAM A4/A6) as self-checking projects.

## Usage / Kullanım

```sh
cargo build --release
B=target/release/uncertainty
$B template > my.json                                   # start a project / proje başlat
$B template --topdown > td.json                         # top-down project from raw data
$B run library/chemistry/quam-a1-cadmium-standard.json  # Markdown report (English)
$B run my.json --lang tr -o rapor.html                  # Türkçe HTML rapor
$B run my.json --format json --no-mcm                   # machine-readable results
$B check library                                        # re-verify all examples / tüm örnekleri doğrula
```

## Scientific validation / Bilimsel doğrulama

Every published reference example implemented so far is reproduced by an
automated test (104 tests; one of them reruns all 60 library projects): GUM H.1, all four examples of JCGM 101 clause 9,
EURACHEM CG4 A1–A7, Nordtest TR 537 and Eurachem MUPR 2026 (recomputed from
their raw data), and all twelve calibration examples of EA-4/02 M:2022. Details, tolerances and findings:
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
