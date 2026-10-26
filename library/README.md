# Example library / Örnek kütüphanesi

Every file is a complete project — a measurement model (GUM and Monte Carlo) or a
top-down evaluation from raw validation data — (`uncertainty run <file>`) and contains the reference values of the
published example with documented tolerances. `uncertainty check library` and `cargo test` rerun all of
them; a failing comparison fails the build. Notes in each file record where the published example is
internally inconsistent (findings of `docs/research/research-log.md`).

Her dosya eksiksiz bir projedir — bir ölçüm modeli (GUM ve Monte Carlo) ya da ham
validasyon verilerinden top-down değerlendirme — (`uncertainty run <dosya>`) ve yayımlanmış örneğin referans değerlerini
belgelenmiş toleranslarla içerir. `uncertainty check library` ve `cargo test` hepsini yeniden hesaplar;
başarısız bir karşılaştırma derlemeyi başarısız kılar. Dosyalardaki notlar, yayımlanmış örneğin kendi içinde
tutarsız olduğu yerleri kaydeder (`docs/research/research-log.md` bulguları).

Reference values: published values where the document reproduces from its own inputs; otherwise analytic
solutions or independent high-precision Monte Carlo references (`validation/`). Monte Carlo tolerances are
about five times the measured Monte Carlo dispersion at the stated number of trials.

Referans değerler: belge kendi girdilerinden yeniden üretilebiliyorsa yayımlanmış değerler; aksi halde analitik
çözümler veya bağımsız yüksek hassasiyetli Monte Carlo referansları (`validation/`). Monte Carlo toleransları,
belirtilen deneme sayısında ölçülen Monte Carlo saçılımının yaklaşık beş katıdır.

**100 examples / örnek** (58 model, 29 top-down, 7 proficiency testing / yeterlilik testi, 6 calibration in both variables / iki değişkenli kalibrasyon).

## Chemistry / Kimya

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`iso13528-e5-los-angeles-reference-value`](chemistry/iso13528-e5-los-angeles-reference-value.json) | Assigned value from a comparison with a CRM: Los Angeles value of aggregates (ISO 13528 E.5) | CRM ile karşılaştırmadan atanan değer: agregaların Los Angeles değeri (ISO 13528 E.5) | ISO 13528:2022 Annex E.5 (Table E.8) |
| [`iso21748-c2-meat-content`](chemistry/iso21748-c2-meat-content.json) | Meat content from nitrogen and fat (ISO 21748 C.2) | Azot ve yağdan et içeriği (ISO 21748 C.2) | ISO 21748:2017 Annex C.2, Table C.1 |
| [`nist-tn1900-e8-molecular-weight-co2`](chemistry/nist-tn1900-e8-molecular-weight-co2.json) | Molecular weight of carbon dioxide (NIST TN 1900 E8) | Karbondioksitin molekül ağırlığı (NIST TN 1900 E8) | NIST Technical Note 1900 Example E8, Exhibit 12 |
| [`nist-tn1900-e9-cadmium-standard`](chemistry/nist-tn1900-e9-cadmium-standard.json) | Cadmium calibration standard, Monte Carlo (NIST TN 1900 E9) | Kadmiyum kalibrasyon standardı, Monte Carlo (NIST TN 1900 E9) | NIST Technical Note 1900 Example E9 |
| [`quam-a1-cadmium-standard-volume`](chemistry/quam-a1-cadmium-standard-volume.json) | Cadmium standard with the volume sub-model | Hacim alt modelli kadmiyum standardı | EURACHEM/CITAC Guide CG 4 Example A1, A1.4 |
| [`quam-a1-cadmium-standard`](chemistry/quam-a1-cadmium-standard.json) | Preparation of a cadmium calibration standard | Kadmiyum kalibrasyon standardının hazırlanması | EURACHEM/CITAC Guide CG 4 Example A1, Tables A1.2 and A1.3 |
| [`quam-a2-naoh-standardisation-submodels`](chemistry/quam-a2-naoh-standardisation-submodels.json) | NaOH standardisation with sub-models | Alt modellerle NaOH ayarlanması | EURACHEM/CITAC Guide CG 4 Example A2, A2.4 |
| [`quam-a2-naoh-standardisation`](chemistry/quam-a2-naoh-standardisation.json) | Standardising a sodium hydroxide solution | Sodyum hidroksit çözeltisinin ayarlanması | EURACHEM/CITAC Guide CG 4 Example A2, Tables A2.2 and A2.3 |
| [`quam-a3-hcl-titration`](chemistry/quam-a3-hcl-titration.json) | Acid–base titration of hydrochloric acid | Hidroklorik asidin asit–baz titrasyonu | EURACHEM/CITAC Guide CG 4 Example A3, Tables A3.2 and A3.3 |
| [`quam-a4-pesticides-combination`](chemistry/quam-a4-pesticides-combination.json) | Organophosphorus pesticides in bread: combination | Ekmekte organofosforlu pestisitler: birleştirme | EURACHEM/CITAC Guide CG 4 Example A4, Tables A4.4 and A4.5 |
| [`quam-a5-cadmium-release`](chemistry/quam-a5-cadmium-release.json) | Cadmium released from ceramic ware | Seramik kaplardan salınan kadmiyum | EURACHEM/CITAC Guide CG 4 Example A5, Tables A5.3 and A5.4 |
| [`quam-a7-assay-standard`](chemistry/quam-a7-assay-standard.json) | Lead assay standard for IDMS | IDMS için kurşun tayin standardı | EURACHEM/CITAC Guide CG 4 Example A7, Eq. (8), Table A7.3 |
| [`quam-a7-lead-double-idms`](chemistry/quam-a7-lead-double-idms.json) | Lead in water by double isotope dilution | Suda kurşunun çift izotop seyreltmesiyle tayini | EURACHEM/CITAC Guide CG 4 Example A7, Eq. (5), Tables A7.4–A7.8 |

## Chemistry, top-down / Kimya, top-down

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`iso21748-c4-crude-fibre`](chemistry/iso21748-c4-crude-fibre.json) | Crude fibre in animal feed from reproducibility data (ISO 21748 C.4) | Hayvan yeminde ham lif, tekrarüretilebilirlik verilerinden (ISO 21748 C.4) | ISO 21748:2017 Annex C.4, Tables C.6–C.9 |
| [`iso5725-3-j1-carbon-steel`](chemistry/iso5725-3-j1-carbon-steel.json) | Carbon in steel: [time + operator]-different intermediate precision (ISO 5725-3 J.1) | Çelikte karbon: [zaman + operatör] farklı ara kesinlik (ISO 5725-3 J.1) | ISO 5725-3:2023 Annex J.1, Table J.1 |
| [`iso5725-3-j2-vanadium-steel`](chemistry/iso5725-3-j2-vanadium-steel.json) | Vanadium in steel: staggered-nested design, level 1 (ISO 5725-3 J.2) | Çelikte vanadyum: kademeli iç içe tasarım, düzey 1 (ISO 5725-3 J.2) | ISO 5725-3:2023 Annex J.2, Tables J.2–J.5 |
| [`mupr-b1-nitrate`](chemistry/mupr-b1-nitrate.json) | Nitrate in drinking water: two precision intervals | İçme suyunda nitrat: iki kesinlik aralığı | Eurachem/CITAC Guide Example B1 |
| [`mupr-b2-pooled-precision`](chemistry/mupr-b2-pooled-precision.json) | Nitrate: pooled precision and two recovery levels | Nitrat: birleştirilmiş kesinlik ve iki geri kazanım düzeyi | Eurachem/CITAC Guide Example B2, Tables B2.1–B2.3 |
| [`mupr-b2-replicates-on-two-days`](chemistry/mupr-b2-replicates-on-two-days.json) | Nitrate: result as the mean of 3 replicates on 2 days | Nitrat: 2 günde 3 tekrarın ortalaması olarak sonuç | Eurachem/CITAC Guide Example B2, Eq. (B2.8)–(B2.9) |
| [`mupr-b3-linear-precision`](chemistry/mupr-b3-linear-precision.json) | Nitrate: precision as a linear function of concentration | Nitrat: derişimin doğrusal fonksiyonu olarak kesinlik | Eurachem/CITAC Guide Example B3, Table B3.1, Eq. (B3.1) |
| [`mupr-b4-arsenic-one-crm`](chemistry/mupr-b4-arsenic-one-crm.json) | Arsenic in sediment: one CRM | Sedimentte arsenik: tek CRM | Eurachem/CITAC Guide Example B4 |
| [`mupr-b5-crm-and-spiked-samples`](chemistry/mupr-b5-crm-and-spiked-samples.json) | Arsenic: CRM and spiked samples with matrix-dependent recovery | Arsenik: matrise bağlı geri kazanımlı CRM ve katkılı numuneler | Eurachem/CITAC Guide Example B5 |
| [`mupr-b6-sediment-22-pt`](chemistry/mupr-b6-sediment-22-pt.json) | Trace element in sediment: heterogeneity and 22 proficiency tests | Sedimentte iz element: heterojenlik ve 22 yeterlilik testi | Eurachem/CITAC Guide Example B6, Tables B6.1–B6.3 |
| [`mupr-ex2-reference-materials`](chemistry/mupr-ex2-reference-materials.json) | Recovery from two CRMs and a proficiency-test sample | İki CRM ve bir yeterlilik testi numunesinden geri kazanım | Eurachem/CITAC Guide Example 2 |
| [`mupr-ex3-spiked-samples`](chemistry/mupr-ex3-spiked-samples.json) | Recovery from spiked river waters | Katkılı nehir sularından geri kazanım | Eurachem/CITAC Guide Example 3 |
| [`nordtest-3-ammonium-pt`](chemistry/nordtest-3-ammonium-pt.json) | Ammonium in water from control limits and proficiency tests | Kontrol sınırları ve yeterlilik testlerinden sudaki amonyum | Nordtest TR 537 ed. 4 (2017) §3.2 and Appendix 4 |
| [`nordtest-5.3-ammonium-duplicates`](chemistry/nordtest-5.3-ammonium-duplicates.json) | Ammonium: within-laboratory reproducibility from control samples and duplicates | Amonyum: kontrol numuneleri ve mükerrer ölçümlerden laboratuvar içi tekrarüretilebilirlik | Nordtest TR 537 ed. 4 (2017) §5.3 and Appendix 5 |
| [`nordtest-5.3-oxygen-duplicates`](chemistry/nordtest-5.3-oxygen-duplicates.json) | Dissolved oxygen: reproducibility from duplicates | Çözünmüş oksijen: mükerrer ölçümlerden tekrarüretilebilirlik | Nordtest TR 537 ed. 4 (2017) §5.3 and Appendix 6 |
| [`nordtest-6.1-one-crm`](chemistry/nordtest-6.1-one-crm.json) | Bias from one certified reference material | Tek sertifikalı referans malzemeden sapma | Nordtest TR 537 ed. 4 (2017) §6.1 |
| [`nordtest-6.1-three-crms`](chemistry/nordtest-6.1-three-crms.json) | Bias from several certified reference materials | Birden fazla sertifikalı referans malzemeden sapma | Nordtest TR 537 ed. 4 (2017) §6.1 |
| [`nordtest-6.2-proficiency-tests`](chemistry/nordtest-6.2-proficiency-tests.json) | Bias from proficiency tests | Yeterlilik testlerinden sapma | Nordtest TR 537 ed. 4 (2017) §6.2 |
| [`nordtest-6.3-recovery`](chemistry/nordtest-6.3-recovery.json) | Bias from recovery experiments | Geri kazanım deneylerinden sapma | Nordtest TR 537 ed. 4 (2017) §6.3 |
| [`nordtest-8-bod-crm`](chemistry/nordtest-8-bod-crm.json) | BOD in wastewater: control chart and one CRM | Atık suda BOİ: kontrol grafiği ve tek CRM | Nordtest TR 537 ed. 4 (2017) §8.2 Example A, Appendix 7 |
| [`nordtest-8-bod-pt`](chemistry/nordtest-8-bod-pt.json) | BOD in wastewater: control chart and proficiency tests | Atık suda BOİ: kontrol grafiği ve yeterlilik testleri | Nordtest TR 537 ed. 4 (2017) §8.2 Example B, Table 6 |
| [`nordtest-8-pcb-crm`](chemistry/nordtest-8-pcb-crm.json) | PCB in sediment: control sample and one CRM | Sedimentte PCB: kontrol numunesi ve tek CRM | Nordtest TR 537 ed. 4 (2017) §8.3 Example C |
| [`nordtest-8-pcb-pt`](chemistry/nordtest-8-pcb-pt.json) | PCB in sediment: control sample and proficiency tests | Sedimentte PCB: kontrol numunesi ve yeterlilik testleri | Nordtest TR 537 ed. 4 (2017) §8.3 Example D |
| [`quam-a4-pesticides-topdown`](chemistry/quam-a4-pesticides-topdown.json) | Organophosphorus pesticides in bread from validation data | Validasyon verilerinden ekmekte organofosforlu pestisitler | EURACHEM/CITAC Guide CG 4 Example A4, Tables A4.2–A4.5 |
| [`quam-a6-crude-fibre`](chemistry/quam-a6-crude-fibre.json) | Crude fibre in animal feed (empirical method) | Hayvan yeminde ham lif (ampirik yöntem) | EURACHEM/CITAC Guide CG 4 Example A6, Tables A6.4–A6.5 |

## Chemistry, proficiency testing / Kimya, yeterlilik testi

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`iso13528-e1-censored-deleted`](chemistry/iso13528-e1-censored-deleted.json) | Censored results in a PT round: censored results deleted (ISO 13528 E.1) | Bir YT turunda sansürlü sonuçlar: sansürlü sonuçlar çıkarılarak (ISO 13528 E.1) | ISO 13528:2022 Annex E.1 (Table E.1) |
| [`iso13528-e1-censored-half`](chemistry/iso13528-e1-censored-half.json) | Censored results in a PT round: 0.5 × the limit (ISO 13528 E.1) | Bir YT turunda sansürlü sonuçlar: sınırın 0,5 katı (ISO 13528 E.1) | ISO 13528:2022 Annex E.1 (Table E.1) |
| [`iso13528-e1-censored-ignored`](chemistry/iso13528-e1-censored-ignored.json) | Censored results in a PT round: ‘<’ ignored (ISO 13528 E.1) | Bir YT turunda sansürlü sonuçlar: ‘<’ işareti yok sayılarak (ISO 13528 E.1) | ISO 13528:2022 Annex E.1 (Table E.1) |
| [`iso13528-e2-arsenic-homogeneity`](chemistry/iso13528-e2-arsenic-homogeneity.json) | Arsenic in chocolate: homogeneity and stability of PT items (ISO 13528 E.2) | Çikolatada arsenik: YT numunelerinin homojenliği ve kararlılığı (ISO 13528 E.2) | ISO 13528:2022 E.2, Tables E.2 and E.3 |
| [`iso13528-e3-atrazine-algorithm-a`](chemistry/iso13528-e3-atrazine-algorithm-a.json) | Atrazine in drinking water: robust average by Algorithm A (ISO 13528 E.3) | İçme suyunda atrazin: Algoritma A ile robust ortalama (ISO 13528 E.3) | ISO 13528:2022 E.3, Tables E.4 and E.5 |
| [`iso13528-e3-atrazine-median`](chemistry/iso13528-e3-atrazine-median.json) | Atrazine in drinking water: median and nIQR (ISO 13528 E.3) | İçme suyunda atrazin: medyan ve nIQR (ISO 13528 E.3) | ISO 13528:2022 E.3, Table E.5 |
| [`iso13528-e4-mercury-feed`](chemistry/iso13528-e4-mercury-feed.json) | Mercury in animal feed: z, z′, ζ and E_n scores (ISO 13528 E.4) | Hayvan yeminde cıva: z, z′, ζ ve E_n skorları (ISO 13528 E.4) | ISO 13528:2022 E.4, Tables E.6 and E.7 |

## Chemistry, calibration (ISO 6143) / Kimya, kalibrasyon (ISO 6143)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`iso6143-b1-no-uv-linear`](chemistry/iso6143-b1-no-uv-linear.json) | NO in N₂ by UV absorption: straight-line analysis function (ISO 6143 Example 1) | N₂ içinde NO, UV absorpsiyonu: doğrusal analiz fonksiyonu (ISO 6143 Örnek 1) | ISO 6143:2025 Annex B.2.2 (Tables B.1–B.6) |
| [`iso6143-b2-n2-natural-gas-covariances`](chemistry/iso6143-b2-n2-natural-gas-covariances.json) | N₂ in natural gas by GC-TCD: correlated reference mixtures (ISO 6143 Example 2) | Doğal gazda N₂, GC-TCD: korelasyonlu referans karışımlar (ISO 6143 Örnek 2) | ISO 6143:2025 Annex B.2.3 (Table B.10) |
| [`iso6143-b2-n2-natural-gas-linear`](chemistry/iso6143-b2-n2-natural-gas-linear.json) | N₂ in natural gas by GC-TCD: straight line (ISO 6143 Example 2) | Doğal gazda N₂, GC-TCD: doğrusal fonksiyon (ISO 6143 Örnek 2) | ISO 6143:2025 Annex B.2.3 (Tables B.7, B.8) |
| [`iso6143-b2-n2-natural-gas-quadratic`](chemistry/iso6143-b2-n2-natural-gas-quadratic.json) | N₂ in natural gas by GC-TCD: second-order polynomial (ISO 6143 Example 2) | Doğal gazda N₂, GC-TCD: ikinci derece polinom (ISO 6143 Örnek 2) | ISO 6143:2025 Annex B.2.3 (Table B.9) |
| [`iso6143-b3-no-ndir-exponential`](chemistry/iso6143-b3-no-ndir-exponential.json) | NO in N₂ by NDIR: exponential function (ISO 6143 Example 3) | N₂ içinde NO, NDIR: üstel fonksiyon (ISO 6143 Örnek 3) | ISO 6143:2025 Annex B.1 and B.2.4 (Tables B.11, B.12) |
| [`iso6143-b3-no-ndir-power`](chemistry/iso6143-b3-no-ndir-power.json) | NO in N₂ by NDIR: power function (ISO 6143 Example 3) | N₂ içinde NO, NDIR: üslü fonksiyon (ISO 6143 Örnek 3) | ISO 6143:2025 Annex B.2.4 (Tables B.11, B.12) |

## Electrical and RF / Elektrik ve RF

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s3-resistor-10kohm`](electrical/ea402-s3-resistor-10kohm.json) | Calibration of a 10 kΩ standard resistor | 10 kΩ'luk standart direncin kalibrasyonu | EA-4/02 M:2022 S3 |
| [`ea402-s6-power-sensor-18ghz`](electrical/ea402-s6-power-sensor-18ghz.json) | Calibration of a power sensor at 18 GHz | Bir güç sensörünün 18 GHz'de kalibrasyonu | EA-4/02 M:2022 S6 |
| [`ea402-s7-step-attenuator-30db`](electrical/ea402-s7-step-attenuator-30db.json) | Calibration of a coaxial step attenuator (30 dB) | Koaksiyel kademeli zayıflatıcının kalibrasyonu (30 dB) | EA-4/02 M:2022 S7 |
| [`ea402-s9-dmm-100v`](electrical/ea402-s9-dmm-100v.json) | Calibration of a hand-held digital multimeter at 100 V | El tipi sayısal multimetrenin 100 V'ta kalibrasyonu | EA-4/02 M:2022 S9 |
| [`gum-h2-impedance`](electrical/gum-h2-impedance.json) | Impedance magnitude from correlated V and I (GUM H.2) | Korelasyonlu V ve I'dan empedans büyüklüğü (GUM H.2) | JCGM 100:2008 H.2, Tables H.2 and H.3 |
| [`gum-h2-reactance`](electrical/gum-h2-reactance.json) | Reactance from correlated V, I and φ (GUM H.2) | Korelasyonlu V, I ve φ'den reaktans (GUM H.2) | JCGM 100:2008 H.2, Tables H.2 and H.3 |
| [`gum-h2-resistance`](electrical/gum-h2-resistance.json) | Resistance from correlated V, I and φ (GUM H.2) | Korelasyonlu V, I ve φ'den direnç (GUM H.2) | JCGM 100:2008 H.2, Tables H.2 and H.3 |
| [`jcgm101-9.4-comparison-loss-x000-correlated`](electrical/jcgm101-9.4-comparison-loss-x000-correlated.json) | Comparison loss, x₁ = 0.0, r = 0.9 | Karşılaştırma kaybı, x₁ = 0,0, r = 0,9 | JCGM 101:2008 9.4, Table 9 |
| [`jcgm101-9.4-comparison-loss-x000`](electrical/jcgm101-9.4-comparison-loss-x000.json) | Comparison loss, x₁ = 0.0 | Karşılaştırma kaybı, x₁ = 0,0 | JCGM 101:2008 9.4, Table 8 |
| [`jcgm101-9.4-comparison-loss-x010-correlated`](electrical/jcgm101-9.4-comparison-loss-x010-correlated.json) | Comparison loss, x₁ = 0.01, r = 0.9 | Karşılaştırma kaybı, x₁ = 0,01, r = 0,9 | JCGM 101:2008 9.4, Table 9 |
| [`jcgm101-9.4-comparison-loss-x010`](electrical/jcgm101-9.4-comparison-loss-x010.json) | Comparison loss, x₁ = 0.01 | Karşılaştırma kaybı, x₁ = 0,01 | JCGM 101:2008 9.4, Table 8 |
| [`jcgm101-9.4-comparison-loss-x050-correlated`](electrical/jcgm101-9.4-comparison-loss-x050-correlated.json) | Comparison loss, x₁ = 0.05, r = 0.9 | Karşılaştırma kaybı, x₁ = 0,05, r = 0,9 | JCGM 101:2008 9.4, Table 9 |
| [`jcgm101-9.4-comparison-loss-x050`](electrical/jcgm101-9.4-comparison-loss-x050.json) | Comparison loss, x₁ = 0.05 | Karşılaştırma kaybı, x₁ = 0,05 | JCGM 101:2008 9.4, Table 8 |
| [`nist-tn1900-e11-step-attenuator`](electrical/nist-tn1900-e11-step-attenuator.json) | Microwave step attenuator, U-shaped inputs (NIST TN 1900 E11) | Mikrodalga kademeli zayıflatıcı, U biçimli girdiler (NIST TN 1900 E11) | NIST Technical Note 1900 Example E11, Exhibits 16 and 17 |
| [`num-10-resistance`](electrical/num-10-resistance.json) | Resistance with the rounded values of GUM Table H.2 (NIST Uncertainty Machine §10) | GUM Tablo H.2'nin yuvarlanmış değerleriyle direnç (NIST Uncertainty Machine §10) | JCGM 100:2008 H.2, Tables H.2 and H.3 |

## Flow and volume / Akış ve hacim

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s12-water-meter-error`](flow/ea402-s12-water-meter-error.json) | Water meter calibration: relative error of one run | Su sayacı kalibrasyonu: tek çalışmanın bağıl hatası | EA-4/02 M:2022 S12 |
| [`ea402-s12-water-meter-mean-error`](flow/ea402-s12-water-meter-mean-error.json) | Water meter calibration: mean of three runs | Su sayacı kalibrasyonu: üç çalışmanın ortalaması | EA-4/02 M:2022 S12 |
| [`ea402-s12-water-meter-volume`](flow/ea402-s12-water-meter-volume.json) | Water meter calibration: reference volume | Su sayacı kalibrasyonu: referans hacim | EA-4/02 M:2022 S12 |
| [`nist-tn1900-e4-pitot-tube`](flow/nist-tn1900-e4-pitot-tube.json) | Airspeed with a Pitot tube (NIST TN 1900 E4) | Pitot tüpüyle hava hızı (NIST TN 1900 E4) | NIST Technical Note 1900 Example E4 |

## General (method validation) / Genel (yöntem doğrulama)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`jcgm101-9.2.2-additive-gaussian`](general/jcgm101-9.2.2-additive-gaussian.json) | Additive model with Gaussian inputs | Normal dağılımlı girdilerle toplamsal model | JCGM 101:2008 9.2.2 |
| [`jcgm101-9.2.3-additive-rectangular`](general/jcgm101-9.2.3-additive-rectangular.json) | Additive model with rectangular inputs | Dikdörtgen dağılımlı girdilerle toplamsal model | JCGM 101:2008 9.2.3 |
| [`jcgm101-9.2.4-additive-dominant-rectangular`](general/jcgm101-9.2.4-additive-dominant-rectangular.json) | Additive model with a dominant rectangular input | Baskın dikdörtgen girdili toplamsal model | JCGM 101:2008 9.2.4 |
| [`num-3-ratio-abc`](general/num-3-ratio-abc.json) | Product and ratio y = ab/c with a wide triangular divisor (NIST Uncertainty Machine §3) | Geniş üçgen bölenli çarpım ve oran y = ab/c (NIST Uncertainty Machine §3) | T. Lafarge §3, Figure 1 |

## General (method validation), top-down / Genel (yöntem doğrulama), top-down

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`iso21748-c1-co-emissions`](general/iso21748-c1-co-emissions.json) | Carbon monoxide in vehicle emissions from reproducibility data (ISO 21748 C.1) | Taşıt emisyonlarında karbon monoksit, tekrarüretilebilirlik verilerinden (ISO 21748 C.1) | ISO 21748:2017 Annex C.1 |
| [`iso21748-c3-aerobic-plate-count-flour`](general/iso21748-c3-aerobic-plate-count-flour.json) | Aerobic plate count, flour: adjusted reproducibility (ISO 21748 C.3) | Aerobik koloni sayımı, un: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3) | ISO 21748:2017 Annex C.3, Tables C.2–C.4 |
| [`iso21748-c3-aerobic-plate-count-shrimp`](general/iso21748-c3-aerobic-plate-count-shrimp.json) | Aerobic plate count, shrimp: adjusted reproducibility (ISO 21748 C.3) | Aerobik koloni sayımı, karides: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3) | ISO 21748:2017 Annex C.3, Tables C.2–C.4 |
| [`iso21748-c3-aerobic-plate-count-vegetables`](general/iso21748-c3-aerobic-plate-count-vegetables.json) | Aerobic plate count, vegetables: adjusted reproducibility (ISO 21748 C.3) | Aerobik koloni sayımı, sebze: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3) | ISO 21748:2017 Annex C.3, Tables C.2–C.4 |

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
| [`num-8-end-gauge-gaussian`](length/num-8-end-gauge-gaussian.json) | End-gauge calibration with Gaussian inputs (NIST Uncertainty Machine §8) | Normal girdilerle uç mastarı kalibrasyonu (NIST Uncertainty Machine §8) | T. Lafarge §8, Table 3 and Figure 3 |

## Mass / Kütle

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s2-weight-10kg`](mass/ea402-s2-weight-10kg.json) | Calibration of a 10 kg weight | 10 kg'lık bir ağırlığın kalibrasyonu | EA-4/02 M:2022 S2 |
| [`jcgm101-9.3-mass-calibration-higher-order`](mass/jcgm101-9.3-mass-calibration-higher-order.json) | Mass calibration (GUM with second-order terms) | Kütle kalibrasyonu (ikinci mertebe terimli GUM) | JCGM 101:2008 9.3, Table 6 |
| [`jcgm101-9.3-mass-calibration`](mass/jcgm101-9.3-mass-calibration.json) | Mass calibration (first-order GUM) | Kütle kalibrasyonu (birinci mertebe GUM) | JCGM 101:2008 9.3, Tables 6 and 7 |
| [`nist-tn1900-e1-weighing`](mass/nist-tn1900-e1-weighing.json) | Mass of a powder by weighing (NIST TN 1900 E1) | Tartımla toz kütlesi (NIST TN 1900 E1) | NIST Technical Note 1900 Example E1, Exhibit 1 |

## Physical properties / Fiziksel özellikler

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`nist-tn1900-e13-thermal-expansion-gaussian-95`](physical/nist-tn1900-e13-thermal-expansion-gaussian-95.json) | Thermal expansion coefficient, Gaussian inputs, 95 % (NIST TN 1900 E13) | Isıl genleşme katsayısı, normal girdiler, %95 (NIST TN 1900 E13) | NIST Technical Note 1900 Example E13, Exhibit 20 |
| [`nist-tn1900-e13-thermal-expansion-gaussian-99`](physical/nist-tn1900-e13-thermal-expansion-gaussian-99.json) | Thermal expansion coefficient, Gaussian inputs, 99 % (NIST TN 1900 E13) | Isıl genleşme katsayısı, normal girdiler, %99 (NIST TN 1900 E13) | NIST Technical Note 1900 Example E13, Exhibit 20 |
| [`nist-tn1900-e13-thermal-expansion-student-95`](physical/nist-tn1900-e13-thermal-expansion-student-95.json) | Thermal expansion coefficient, Student t₃ inputs, 95 % (NIST TN 1900 E13) | Isıl genleşme katsayısı, Student t₃ girdiler, %95 (NIST TN 1900 E13) | NIST Technical Note 1900 Example E13, Exhibit 20 |
| [`nist-tn1900-e13-thermal-expansion-student-99`](physical/nist-tn1900-e13-thermal-expansion-student-99.json) | Thermal expansion coefficient, Student t₃ inputs, 99 % (NIST TN 1900 E13) | Isıl genleşme katsayısı, Student t₃ girdiler, %99 (NIST TN 1900 E13) | NIST Technical Note 1900 Example E13, Exhibit 20 |
| [`nist-tn1900-e3-falling-ball-viscometer`](physical/nist-tn1900-e3-falling-ball-viscometer.json) | Falling-ball viscometer (NIST TN 1900 E3) | Düşen bilyalı viskozimetre (NIST TN 1900 E3) | NIST Technical Note 1900 Example E3, Exhibit 3 |
| [`num-11-stefan-boltzmann`](physical/num-11-stefan-boltzmann.json) | Stefan–Boltzmann constant from CODATA 2010 values (NIST Uncertainty Machine §11) | CODATA 2010 değerlerinden Stefan–Boltzmann sabiti (NIST Uncertainty Machine §11) | T. Lafarge §11, Table 5 and Figure 7 |

## Temperature / Sıcaklık

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s11-block-calibrator-180c`](temperature/ea402-s11-block-calibrator-180c.json) | Calibration of a temperature block calibrator at 180 °C | Sıcaklık blok kalibratörünün 180 °C'de kalibrasyonu | EA-4/02 M:2022 S11 |
| [`ea402-s5-thermocouple-emf`](temperature/ea402-s5-thermocouple-emf.json) | Thermocouple calibration: emf of the unknown | Termokupl kalibrasyonu: bilinmeyenin emk'sı | EA-4/02 M:2022 S5 |
| [`ea402-s5-thermocouple-furnace`](temperature/ea402-s5-thermocouple-furnace.json) | Thermocouple calibration: furnace temperature | Termokupl kalibrasyonu: fırın sıcaklığı | EA-4/02 M:2022 S5 |
| [`nist-tn1900-e2-surface-temperature`](temperature/nist-tn1900-e2-surface-temperature.json) | Monthly mean of daily maximum temperature (NIST TN 1900 E2) | Günlük en yüksek sıcaklığın aylık ortalaması (NIST TN 1900 E2) | NIST Technical Note 1900 Example E2, Exhibit 2 |

## Adding an example / Örnek ekleme

Start from `uncertainty template`, give the `metadata.id` the file name, cite the source, and put the
published values with their rounding tolerance under `expected`. Explain every deviation from the source in
`metadata.notes` (English and Turkish).

`uncertainty template` çıktısından başlayın, `metadata.id` alanına dosya adını verin, kaynağı belirtin ve
yayımlanmış değerleri yuvarlama toleranslarıyla `expected` altına yazın. Kaynaktan her sapmayı
`metadata.notes` içinde (İngilizce ve Türkçe) açıklayın.
