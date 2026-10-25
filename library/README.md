# Example library / Örnek kütüphanesi

Every file is a complete project (`uncertainty run <file>`) and contains the reference values of the
published example with documented tolerances. `uncertainty check library` and `cargo test` rerun all of
them; a failing comparison fails the build. Notes in each file record where the published example is
internally inconsistent (findings of `docs/research/research-log.md`).

Her dosya eksiksiz bir projedir (`uncertainty run <dosya>`) ve yayımlanmış örneğin referans değerlerini
belgelenmiş toleranslarla içerir. `uncertainty check library` ve `cargo test` hepsini yeniden hesaplar;
başarısız bir karşılaştırma derlemeyi başarısız kılar. Dosyalardaki notlar, yayımlanmış örneğin kendi içinde
tutarsız olduğu yerleri kaydeder (`docs/research/research-log.md` bulguları).

Reference values: published values where the document reproduces from its own inputs; otherwise analytic
solutions or independent high-precision Monte Carlo references (`validation/`). Monte Carlo tolerances are
about five times the measured Monte Carlo dispersion at the stated number of trials.

Referans değerler: belge kendi girdilerinden yeniden üretilebiliyorsa yayımlanmış değerler; aksi halde analitik
çözümler veya bağımsız yüksek hassasiyetli Monte Carlo referansları (`validation/`). Monte Carlo toleransları,
belirtilen deneme sayısında ölçülen Monte Carlo saçılımının yaklaşık beş katıdır.

**38 examples / örnek.**

## Chemistry / Kimya

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`quam-a1-cadmium-standard-volume`](chemistry/quam-a1-cadmium-standard-volume.json) | Cadmium standard with the volume sub-model | Hacim alt modelli kadmiyum standardı | EURACHEM/CITAC Guide CG 4 Example A1, A1.4 |
| [`quam-a1-cadmium-standard`](chemistry/quam-a1-cadmium-standard.json) | Preparation of a cadmium calibration standard | Kadmiyum kalibrasyon standardının hazırlanması | EURACHEM/CITAC Guide CG 4 Example A1, Tables A1.2 and A1.3 |
| [`quam-a2-naoh-standardisation-submodels`](chemistry/quam-a2-naoh-standardisation-submodels.json) | NaOH standardisation with sub-models | Alt modellerle NaOH ayarlanması | EURACHEM/CITAC Guide CG 4 Example A2, A2.4 |
| [`quam-a2-naoh-standardisation`](chemistry/quam-a2-naoh-standardisation.json) | Standardising a sodium hydroxide solution | Sodyum hidroksit çözeltisinin ayarlanması | EURACHEM/CITAC Guide CG 4 Example A2, Tables A2.2 and A2.3 |
| [`quam-a3-hcl-titration`](chemistry/quam-a3-hcl-titration.json) | Acid–base titration of hydrochloric acid | Hidroklorik asidin asit–baz titrasyonu | EURACHEM/CITAC Guide CG 4 Example A3, Tables A3.2 and A3.3 |
| [`quam-a4-pesticides-combination`](chemistry/quam-a4-pesticides-combination.json) | Organophosphorus pesticides in bread: combination | Ekmekte organofosforlu pestisitler: birleştirme | EURACHEM/CITAC Guide CG 4 Example A4, Tables A4.4 and A4.5 |
| [`quam-a5-cadmium-release`](chemistry/quam-a5-cadmium-release.json) | Cadmium released from ceramic ware | Seramik kaplardan salınan kadmiyum | EURACHEM/CITAC Guide CG 4 Example A5, Tables A5.3 and A5.4 |
| [`quam-a7-assay-standard`](chemistry/quam-a7-assay-standard.json) | Lead assay standard for IDMS | IDMS için kurşun tayin standardı | EURACHEM/CITAC Guide CG 4 Example A7, Eq. (8), Table A7.3 |
| [`quam-a7-lead-double-idms`](chemistry/quam-a7-lead-double-idms.json) | Lead in water by double isotope dilution | Suda kurşunun çift izotop seyreltmesiyle tayini | EURACHEM/CITAC Guide CG 4 Example A7, Eq. (5), Tables A7.4–A7.8 |

## Electrical and RF / Elektrik ve RF

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s3-resistor-10kohm`](electrical/ea402-s3-resistor-10kohm.json) | Calibration of a 10 kΩ standard resistor | 10 kΩ'luk standart direncin kalibrasyonu | EA-4/02 M:2022 S3 |
| [`ea402-s6-power-sensor-18ghz`](electrical/ea402-s6-power-sensor-18ghz.json) | Calibration of a power sensor at 18 GHz | Bir güç sensörünün 18 GHz'de kalibrasyonu | EA-4/02 M:2022 S6 |
| [`ea402-s7-step-attenuator-30db`](electrical/ea402-s7-step-attenuator-30db.json) | Calibration of a coaxial step attenuator (30 dB) | Koaksiyel kademeli zayıflatıcının kalibrasyonu (30 dB) | EA-4/02 M:2022 S7 |
| [`ea402-s9-dmm-100v`](electrical/ea402-s9-dmm-100v.json) | Calibration of a hand-held digital multimeter at 100 V | El tipi sayısal multimetrenin 100 V'ta kalibrasyonu | EA-4/02 M:2022 S9 |
| [`jcgm101-9.4-comparison-loss-x000-correlated`](electrical/jcgm101-9.4-comparison-loss-x000-correlated.json) | Comparison loss, x₁ = 0.0, r = 0.9 | Karşılaştırma kaybı, x₁ = 0,0, r = 0,9 | JCGM 101:2008 9.4, Table 9 |
| [`jcgm101-9.4-comparison-loss-x000`](electrical/jcgm101-9.4-comparison-loss-x000.json) | Comparison loss, x₁ = 0.0 | Karşılaştırma kaybı, x₁ = 0,0 | JCGM 101:2008 9.4, Table 8 |
| [`jcgm101-9.4-comparison-loss-x010-correlated`](electrical/jcgm101-9.4-comparison-loss-x010-correlated.json) | Comparison loss, x₁ = 0.01, r = 0.9 | Karşılaştırma kaybı, x₁ = 0,01, r = 0,9 | JCGM 101:2008 9.4, Table 9 |
| [`jcgm101-9.4-comparison-loss-x010`](electrical/jcgm101-9.4-comparison-loss-x010.json) | Comparison loss, x₁ = 0.01 | Karşılaştırma kaybı, x₁ = 0,01 | JCGM 101:2008 9.4, Table 8 |
| [`jcgm101-9.4-comparison-loss-x050-correlated`](electrical/jcgm101-9.4-comparison-loss-x050-correlated.json) | Comparison loss, x₁ = 0.05, r = 0.9 | Karşılaştırma kaybı, x₁ = 0,05, r = 0,9 | JCGM 101:2008 9.4, Table 9 |
| [`jcgm101-9.4-comparison-loss-x050`](electrical/jcgm101-9.4-comparison-loss-x050.json) | Comparison loss, x₁ = 0.05 | Karşılaştırma kaybı, x₁ = 0,05 | JCGM 101:2008 9.4, Table 8 |

## Flow and volume / Akış ve hacim

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s12-water-meter-error`](flow/ea402-s12-water-meter-error.json) | Water meter calibration: relative error of one run | Su sayacı kalibrasyonu: tek çalışmanın bağıl hatası | EA-4/02 M:2022 S12 |
| [`ea402-s12-water-meter-mean-error`](flow/ea402-s12-water-meter-mean-error.json) | Water meter calibration: mean of three runs | Su sayacı kalibrasyonu: üç çalışmanın ortalaması | EA-4/02 M:2022 S12 |
| [`ea402-s12-water-meter-volume`](flow/ea402-s12-water-meter-volume.json) | Water meter calibration: reference volume | Su sayacı kalibrasyonu: referans hacim | EA-4/02 M:2022 S12 |

## General (method validation) / Genel (yöntem doğrulama)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`jcgm101-9.2.2-additive-gaussian`](general/jcgm101-9.2.2-additive-gaussian.json) | Additive model with Gaussian inputs | Normal dağılımlı girdilerle toplamsal model | JCGM 101:2008 9.2.2 |
| [`jcgm101-9.2.3-additive-rectangular`](general/jcgm101-9.2.3-additive-rectangular.json) | Additive model with rectangular inputs | Dikdörtgen dağılımlı girdilerle toplamsal model | JCGM 101:2008 9.2.3 |
| [`jcgm101-9.2.4-additive-dominant-rectangular`](general/jcgm101-9.2.4-additive-dominant-rectangular.json) | Additive model with a dominant rectangular input | Baskın dikdörtgen girdili toplamsal model | JCGM 101:2008 9.2.4 |

## Length / Uzunluk

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s10-vernier-calliper`](length/ea402-s10-vernier-calliper.json) | Calibration of a vernier calliper at 150 mm | Kumpasın 150 mm'de kalibrasyonu | EA-4/02 M:2022 S10 |
| [`ea402-s13-ring-gauge-90mm`](length/ea402-s13-ring-gauge-90mm.json) | Calibration of a 90 mm ring gauge | 90 mm'lik halka mastarın kalibrasyonu | EA-4/02 M:2022 S13 |
| [`ea402-s13-ring-gauge-temperature`](length/ea402-s13-ring-gauge-temperature.json) | Ring gauge calibration: temperature correction | Halka mastar kalibrasyonu: sıcaklık düzeltmesi | EA-4/02 M:2022 S13 |
| [`ea402-s4-gauge-block-50mm`](length/ea402-s4-gauge-block-50mm.json) | Calibration of a 50 mm gauge block | 50 mm'lik mastar bloğunun kalibrasyonu | EA-4/02 M:2022 S4 |
| [`gum-h1-end-gauge`](length/gum-h1-end-gauge.json) | End-gauge calibration (GUM H.1) | Uç mastarı kalibrasyonu (GUM H.1) | JCGM 100:2008 H.1, Table H.1 |
| [`jcgm101-9.5-gauge-block-approximate`](length/jcgm101-9.5-gauge-block-approximate.json) | Gauge block calibration, approximate model | Mastar kalibrasyonu, yaklaşık model | JCGM 101:2008 9.5, Tables 10 and 11 |
| [`jcgm101-9.5-gauge-block-nonlinear`](length/jcgm101-9.5-gauge-block-nonlinear.json) | Gauge block calibration, non-linear model | Mastar kalibrasyonu, doğrusal olmayan model | JCGM 101:2008 9.5, Tables 10 and 11 |

## Mass / Kütle

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s2-weight-10kg`](mass/ea402-s2-weight-10kg.json) | Calibration of a 10 kg weight | 10 kg'lık bir ağırlığın kalibrasyonu | EA-4/02 M:2022 S2 |
| [`jcgm101-9.3-mass-calibration-higher-order`](mass/jcgm101-9.3-mass-calibration-higher-order.json) | Mass calibration (GUM with second-order terms) | Kütle kalibrasyonu (ikinci mertebe terimli GUM) | JCGM 101:2008 9.3, Table 6 |
| [`jcgm101-9.3-mass-calibration`](mass/jcgm101-9.3-mass-calibration.json) | Mass calibration (first-order GUM) | Kütle kalibrasyonu (birinci mertebe GUM) | JCGM 101:2008 9.3, Tables 6 and 7 |

## Temperature / Sıcaklık

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s11-block-calibrator-180c`](temperature/ea402-s11-block-calibrator-180c.json) | Calibration of a temperature block calibrator at 180 °C | Sıcaklık blok kalibratörünün 180 °C'de kalibrasyonu | EA-4/02 M:2022 S11 |
| [`ea402-s5-thermocouple-emf`](temperature/ea402-s5-thermocouple-emf.json) | Thermocouple calibration: emf of the unknown | Termokupl kalibrasyonu: bilinmeyenin emk'sı | EA-4/02 M:2022 S5 |
| [`ea402-s5-thermocouple-furnace`](temperature/ea402-s5-thermocouple-furnace.json) | Thermocouple calibration: furnace temperature | Termokupl kalibrasyonu: fırın sıcaklığı | EA-4/02 M:2022 S5 |

## Adding an example / Örnek ekleme

Start from `uncertainty template`, give the `metadata.id` the file name, cite the source, and put the
published values with their rounding tolerance under `expected`. Explain every deviation from the source in
`metadata.notes` (English and Turkish).

`uncertainty template` çıktısından başlayın, `metadata.id` alanına dosya adını verin, kaynağı belirtin ve
yayımlanmış değerleri yuvarlama toleranslarıyla `expected` altına yazın. Kaynaktan her sapmayı
`metadata.notes` içinde (İngilizce ve Türkçe) açıklayın.
