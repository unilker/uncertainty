# Uncertainty *(working title)*

**EN** — Open software for evaluating measurement uncertainty with the GUM
uncertainty framework (JCGM 100:2008) and the Monte Carlo method of GUM
Supplement 1 (JCGM 101:2008), for analytical chemistry, calibration and
accredited laboratories. Desktop and mobile, Turkish and English.

**TR** — Ölçüm belirsizliğini GUM yaklaşımı (JCGM 100:2008) ve GUM Ek 1
Monte Carlo yöntemi (JCGM 101:2008) ile hesaplayan açık yazılım; analitik
kimya, kalibrasyon ve akredite laboratuvarlar için. Masaüstü ve mobil,
Türkçe ve İngilizce.

> Status / Durum: **v0.1.0 — computation core only** (no user interface yet).
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

## Scientific validation / Bilimsel doğrulama

Every published reference example implemented so far is reproduced by an
automated test: GUM H.1, all four examples of JCGM 101 clause 9, EURACHEM
CG4 A1. Details, tolerances and findings:
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
