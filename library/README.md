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

**180 examples / örnek** (84 model, 36 top-down, 7 proficiency testing / yeterlilik testi, 6 calibration in both variables / iki değişkenli kalibrasyon, 6 interlaboratory precision / laboratuvarlar arası kesinlik, 13 multivariate / çok çıktılı, 21 control charts / kontrol grafiği, 7 conformity assessment / uygunluk değerlendirmesi).

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
| [`palma2019-cr-marine-sediment`](chemistry/palma2019-cr-marine-sediment.json) | Chromium in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019) | Deniz sedimentlerinde krom, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019) | C. Palma Table 9 |
| [`palma2019-cu-marine-sediment`](chemistry/palma2019-cu-marine-sediment.json) | Copper in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019) | Deniz sedimentlerinde bakır, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019) | C. Palma Table 9 |
| [`palma2019-li-marine-sediment`](chemistry/palma2019-li-marine-sediment.json) | Lithium in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019) | Deniz sedimentlerinde lityum, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019) | C. Palma Table 9 |
| [`palma2019-mn-marine-sediment`](chemistry/palma2019-mn-marine-sediment.json) | Manganese in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019) | Deniz sedimentlerinde mangan, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019) | C. Palma Table 9 |
| [`palma2019-ni-marine-sediment`](chemistry/palma2019-ni-marine-sediment.json) | Nickel in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019) | Deniz sedimentlerinde nikel, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019) | C. Palma Table 9 |
| [`palma2019-pb-marine-sediment`](chemistry/palma2019-pb-marine-sediment.json) | Lead in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019) | Deniz sedimentlerinde kurşun, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019) | C. Palma Table 9, Section 4.1 |
| [`palma2019-zn-marine-sediment`](chemistry/palma2019-zn-marine-sediment.json) | Zinc in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019) | Deniz sedimentlerinde çinko, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019) | C. Palma Table 9 |
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

## Chemistry, interlaboratory precision (ISO 5725-2) / Kimya, laboratuvarlar arası kesinlik (ISO 5725-2)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`iso5725-2-c1-sulfur-in-coal-reml`](chemistry/iso5725-2-c1-sulfur-in-coal-reml.json) | Sulfur content of coal: REML estimates (ISO 5725-2 Example C.1) | Kömürün kükürt içeriği: REML tahminleri (ISO 5725-2 Örnek C.1) | ISO 5725-2:2019 Annex C.1.9 (Table C.6), Annex B.2 |
| [`iso5725-2-c1-sulfur-in-coal`](chemistry/iso5725-2-c1-sulfur-in-coal.json) | Sulfur content of coal: repeatability and reproducibility (ISO 5725-2 Example C.1) | Kömürün kükürt içeriği: tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2 Örnek C.1) | ISO 5725-2:2019 Annex C.1 (Tables C.1–C.5, Formulae C.1–C.24) |
| [`iso5725-2-c3-creosote-oil-reml`](chemistry/iso5725-2-c3-creosote-oil-reml.json) | Thermometric titration of creosote oil: REML estimates (ISO 5725-2 Example C.3) | Kreozot yağının termometrik titrasyonu: REML tahminleri (ISO 5725-2 Örnek C.3) | ISO 5725-2:2019 Annex C.3.8 (Table C.19), Annex B.2 |
| [`iso5725-2-c3-creosote-oil`](chemistry/iso5725-2-c3-creosote-oil.json) | Thermometric titration of creosote oil: outlying laboratory and precision relationships (ISO 5725-2 Example C.3) | Kreozot yağının termometrik titrasyonu: aykırı laboratuvar ve kesinlik ilişkileri (ISO 5725-2 Örnek C.3) | ISO 5725-2:2019 Annex C.3 (Tables C.14–C.18) and 8.5 (Tables 1–4) |

## Chemistry, control charts (Nordtest TR 569) / Kimya, kontrol grafikleri (Nordtest TR 569)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`tr569-ch7-case1-statistical`](chemistry/tr569-ch7-case1-statistical.json) | Setting the limits of an X-chart, case 1 (Nordtest TR 569 Chapter 7) | X-grafiğinin sınırlarının belirlenmesi, durum 1 (Nordtest TR 569 Bölüm 7) | Nordtest TR 569 ed. 6 (2026) Chapter 7, Setting the control limits and the central line in X-charts, case 1 |
| [`tr569-ch7-case2-target-mean`](chemistry/tr569-ch7-case2-target-mean.json) | Setting the limits of an X-chart, case 2 (Nordtest TR 569 Chapter 7) | X-grafiğinin sınırlarının belirlenmesi, durum 2 (Nordtest TR 569 Bölüm 7) | Nordtest TR 569 ed. 6 (2026) Chapter 7, Setting the control limits and the central line in X-charts, case 2 |
| [`tr569-ch7-case3-target-reference`](chemistry/tr569-ch7-case3-target-reference.json) | Setting the limits of an X-chart, case 3 (Nordtest TR 569 Chapter 7) | X-grafiğinin sınırlarının belirlenmesi, durum 3 (Nordtest TR 569 Bölüm 7) | Nordtest TR 569 ed. 6 (2026) Chapter 7, Setting the control limits and the central line in X-charts, case 3 |
| [`tr569-ch7-range-case1-statistical`](chemistry/tr569-ch7-range-case1-statistical.json) | Statistical limits of an r%-chart for duplicates (Nordtest TR 569 Chapter 7) | Paralel tayinler için r%-grafiğinin istatistiksel sınırları (Nordtest TR 569 Bölüm 7) | Nordtest TR 569 ed. 6 (2026) Chapter 7, Setting the control limit in R or r%-charts, case 1 |
| [`tr569-ch7-range-case2-target`](chemistry/tr569-ch7-range-case2-target.json) | Target limits of an r%-chart from a repeatability limit (Nordtest TR 569 Chapter 7) | Tekrarlanabilirlik sınırından r%-grafiğinin hedef sınırları (Nordtest TR 569 Bölüm 7) | Nordtest TR 569 ed. 6 (2026) Chapter 7, Setting the control limit in R or r%-charts, case 2 |
| [`tr569-ex1-nickel-steel-xrf`](chemistry/tr569-ex1-nickel-steel-xrf.json) | Nickel in low-alloy steel by XRF: target limits from the required uncertainty (Nordtest TR 569 Example 1) | XRF ile düşük alaşımlı çelikte nikel: gerekli belirsizlikten hedef sınırlar (Nordtest TR 569 Örnek 1) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 1 |
| [`tr569-ex10-boron-toys-target`](chemistry/tr569-ex10-boron-toys-target.json) | Boron migrating from toys by ICP-OES: target limits from the reproducibility of EN 71-3 (Nordtest TR 569 Example 10) | ICP-OES ile oyuncaklardan bor göçü: EN 71-3 tekrarüretilebilirliğinden hedef sınırlar (Nordtest TR 569 Örnek 10) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 10 |
| [`tr569-ex12-pooling-sr-srw`](chemistry/tr569-ex12-pooling-sr-srw.json) | Repeatability and within-laboratory reproducibility from replicate control results (Nordtest TR 569 Example 12) | Tekrarlı kontrol sonuçlarından tekrarlanabilirlik ve laboratuvar içi tekrarüretilebilirlik (Nordtest TR 569 Örnek 12) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 12 and footnote 24 |
| [`tr569-ex2-cobalt-steel-xrf`](chemistry/tr569-ex2-cobalt-steel-xrf.json) | Cobalt in low-alloy steel by XRF: target limits from the limit of quantification (Nordtest TR 569 Example 2) | XRF ile düşük alaşımlı çelikte kobalt: tayin sınırından hedef sınırlar (Nordtest TR 569 Örnek 2) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 2 |
| [`tr569-ex3-ammonium-r`](chemistry/tr569-ex3-ammonium-r.json) | Ammonium nitrogen in water: R-chart of duplicate test samples (Nordtest TR 569 Example 3) | Suda amonyum azotu: paralel numunelerin R-grafiği (Nordtest TR 569 Örnek 3) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 3 |
| [`tr569-ex3-ammonium-x`](chemistry/tr569-ex3-ammonium-x.json) | Ammonium nitrogen in water: X-chart of a standard solution (Nordtest TR 569 Example 3) | Suda amonyum azotu: standart çözeltinin X-grafiği (Nordtest TR 569 Örnek 3) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 3 |
| [`tr569-ex4-lead-water-icpms`](chemistry/tr569-ex4-lead-water-icpms.json) | Lead in water by ICP-MS: X-chart of an in-house lake water (Nordtest TR 569 Example 4) | ICP-MS ile suda kurşun: laboratuvar içi göl suyunun X-grafiği (Nordtest TR 569 Örnek 4) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 4 |
| [`tr569-ex5-arsenic-crm-tolerance`](chemistry/tr569-ex5-arsenic-crm-tolerance.json) | Arsenic in a fish CRM by ICP-MS: chart with action limits only at ± 15 % (Nordtest TR 569 Example 5) | ICP-MS ile balık CRM'sinde arsenik: yalnızca ± %15 eylem sınırlı grafik (Nordtest TR 569 Örnek 5) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 5 |
| [`tr569-ex6-phosphorus-relative-range`](chemistry/tr569-ex6-phosphorus-relative-range.json) | Total phosphorus in water: r%-chart of duplicates (Nordtest TR 569 Example 6) | Suda toplam fosfor: paralel tayinlerin r%-grafiği (Nordtest TR 569 Örnek 6) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 6 |
| [`tr569-ex7-beta-hch-crm-target`](chemistry/tr569-ex7-beta-hch-crm-target.json) | β-HCH in cod liver oil by GC-ECD: target limits around the reference value (Nordtest TR 569 Example 7) | GC-ECD ile morina karaciğeri yağında β-HCH: referans değer çevresinde hedef sınırlar (Nordtest TR 569 Örnek 7) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 7 |
| [`tr569-ex8-copper-r-review`](chemistry/tr569-ex8-copper-r-review.json) | Copper in water by ICP-OES: R-chart of duplicates and its review (Nordtest TR 569 Example 8) | ICP-OES ile suda bakır: paralel tayinlerin R-grafiği ve gözden geçirilmesi (Nordtest TR 569 Örnek 8) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 8 |
| [`tr569-ex8-copper-x-new-limits`](chemistry/tr569-ex8-copper-x-new-limits.json) | Copper in water by ICP-OES: new X-chart limits from all data (Nordtest TR 569 Example 8) | ICP-OES ile suda bakır: tüm verilerden yeni X-grafiği sınırları (Nordtest TR 569 Örnek 8) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 8 |
| [`tr569-ex8-copper-x-review`](chemistry/tr569-ex8-copper-x-review.json) | Copper in water by ICP-OES: review of preliminary X-chart limits (Nordtest TR 569 Example 8) | ICP-OES ile suda bakır: ön X-grafiği sınırlarının gözden geçirilmesi (Nordtest TR 569 Örnek 8) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 8 |
| [`tr569-ex9-zinc-procedural-blank`](chemistry/tr569-ex9-zinc-procedural-blank.json) | Zinc in hydrogen peroxide: X-chart of a procedural blank (Nordtest TR 569 Example 9) | Hidrojen peroksitte çinko: yöntem körünün X-grafiği (Nordtest TR 569 Örnek 9) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 9 |
| [`tr569-table1-zinc`](chemistry/tr569-table1-zinc.json) | Zinc in water: X-chart from 60 control values (Nordtest TR 569 Table 1) | Sudaki çinko: 60 kontrol değerinden X-grafiği (Nordtest TR 569 Tablo 1) | Nordtest TR 569 ed. 6 (2026) Chapter 2, Table 1 and Figure 1; Chapters 7 and 9 |

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
| [`m3003-c5-dvm-dominant-rectangular`](electrical/m3003-c5-dvm-dominant-rectangular.json) | Digital voltmeter with a dominant resolution term (UKAS M3003 Appendix C) | Baskın çözünürlük terimli dijital voltmetre (UKAS M3003 Ek C) | UKAS M3003 ed. 6 (2024) Appendix C.5–C.10 |
| [`m3003-k1-resistor-10kohm`](electrical/m3003-k1-resistor-10kohm.json) | 10 kΩ resistor by voltage intercomparison (UKAS M3003 K.1) | Gerilim karşılaştırmasıyla 10 kΩ direnç (UKAS M3003 K.1) | UKAS M3003 ed. 6 (2024) Appendix K.1 |
| [`m3003-k2-power-sensor-18ghz`](electrical/m3003-k2-power-sensor-18ghz.json) | Coaxial power sensor at 18 GHz (UKAS M3003 K.2) | 18 GHz'de koaksiyel güç sensörü (UKAS M3003 K.2) | UKAS M3003 ed. 6 (2024) Appendix K.2 |
| [`m3003-k3-attenuator-30db-mismatch-e5`](electrical/m3003-k3-attenuator-30db-mismatch-e5.json) | 30 dB attenuator with the mismatch term from formula E5 (UKAS M3003 K.3, E.9.5) | E5 formülünden uyumsuzluk terimiyle 30 dB zayıflatıcı (UKAS M3003 K.3, E.9.5) | UKAS M3003 ed. 6 (2024) Appendix K.3 and E.9.5 |
| [`m3003-k3-attenuator-30db`](electrical/m3003-k3-attenuator-30db.json) | 30 dB coaxial attenuator at 10 GHz (UKAS M3003 K.3) | 10 GHz'de 30 dB koaksiyel zayıflatıcı (UKAS M3003 K.3) | UKAS M3003 ed. 6 (2024) Appendix K.3 |
| [`m3003-l3-dmm-1v-range-095v`](electrical/m3003-l3-dmm-1v-range-095v.json) | DMM on the 1 V range: budget at 0.95 V (UKAS M3003 L.3) | 1 V kademesinde DMM: 0,95 V'taki bütçe (UKAS M3003 L.3) | UKAS M3003 ed. 6 (2024) Appendix L.3 |
| [`m3003-l3-dmm-1v-range-1v-dominant`](electrical/m3003-l3-dmm-1v-range-1v-dominant.json) | DMM on the 1 V range at full scale: coverage factor for a dominant specification term (UKAS M3003 L.3.20) | Tam ölçekte 1 V kademesinde DMM: baskın şartname terimi için kapsam faktörü (UKAS M3003 L.3.20) | UKAS M3003 ed. 6 (2024) Appendix L.3 |
| [`m3003-l3-dmm-1v-range-500mv`](electrical/m3003-l3-dmm-1v-range-500mv.json) | DMM on the 1 V range: uncertainty at 500 mV from the range formula (UKAS M3003 L.3) | 1 V kademesinde DMM: kademe formülünden 500 mV'taki belirsizlik (UKAS M3003 L.3) | UKAS M3003 ed. 6 (2024) Appendix L.3 |
| [`nist-tn1900-e11-step-attenuator`](electrical/nist-tn1900-e11-step-attenuator.json) | Microwave step attenuator, U-shaped inputs (NIST TN 1900 E11) | Mikrodalga kademeli zayıflatıcı, U biçimli girdiler (NIST TN 1900 E11) | NIST Technical Note 1900 Example E11, Exhibits 16 and 17 |
| [`num-10-resistance`](electrical/num-10-resistance.json) | Resistance with the rounded values of GUM Table H.2 (NIST Uncertainty Machine §10) | GUM Tablo H.2'nin yuvarlanmış değerleriyle direnç (NIST Uncertainty Machine §10) | JCGM 100:2008 H.2, Tables H.2 and H.3 |

## Electrical and RF, several output quantities (JCGM 102) / Elektrik ve RF, birden çok çıktı büyüklüğü (JCGM 102)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`jcgm102-9.3-polar-x001-correlated`](electrical/jcgm102-9.3-polar-x001-correlated.json) | Cartesian to polar co-ordinates, x₁ = 0.001, r = 0.9 (JCGM 102 9.3) | Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,001, r = 0,9 (JCGM 102 9.3) | JCGM 102:2011 9.3 (Table 7) |
| [`jcgm102-9.3-polar-x001`](electrical/jcgm102-9.3-polar-x001.json) | Cartesian to polar co-ordinates, x₁ = 0.001 (JCGM 102 9.3) | Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,001 (JCGM 102 9.3) | JCGM 102:2011 9.3 (Table 6) |
| [`jcgm102-9.3-polar-x010-correlated`](electrical/jcgm102-9.3-polar-x010-correlated.json) | Cartesian to polar co-ordinates, x₁ = 0.01, r = 0.9 (JCGM 102 9.3) | Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,01, r = 0,9 (JCGM 102 9.3) | JCGM 102:2011 9.3 (Table 7) |
| [`jcgm102-9.3-polar-x010`](electrical/jcgm102-9.3-polar-x010.json) | Cartesian to polar co-ordinates, x₁ = 0.01 (JCGM 102 9.3) | Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,01 (JCGM 102 9.3) | JCGM 102:2011 9.3 (Table 6) |
| [`jcgm102-9.3-polar-x100-correlated`](electrical/jcgm102-9.3-polar-x100-correlated.json) | Cartesian to polar co-ordinates, x₁ = 0.1, r = 0.9 (JCGM 102 9.3) | Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,1, r = 0,9 (JCGM 102 9.3) | JCGM 102:2011 9.3 (Table 7) |
| [`jcgm102-9.3-polar-x100`](electrical/jcgm102-9.3-polar-x100.json) | Cartesian to polar co-ordinates, x₁ = 0.1 (JCGM 102 9.3) | Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,1 (JCGM 102 9.3) | JCGM 102:2011 9.3 (Table 6) |
| [`jcgm102-9.4-resistance-reactance-alternative`](electrical/jcgm102-9.4-resistance-reactance-alternative.json) | Resistance, reactance and impedance from simultaneous indication values, t covariance (JCGM 102 9.4) | Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans, t kovaryansı (JCGM 102 9.4) | JCGM 102:2011 9.4 (Tables 8–11) |
| [`jcgm102-9.4-resistance-reactance`](electrical/jcgm102-9.4-resistance-reactance.json) | Resistance, reactance and impedance from simultaneous indication values (JCGM 102 9.4) | Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans (JCGM 102 9.4) | JCGM 102:2011 9.4 (Tables 8–11) |

## Flow and volume / Akış ve hacim

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`cg19-ex1-flask-1000ml`](flow/cg19-ex1-flask-1000ml.json) | Gravimetric calibration of a 1000 mL volumetric flask (EURAMET cg-19 Example 1) | 1000 mL'lik ölçülü balonun gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 1) | EURAMET Calibration Guide No. 19 v4.1 (2025) 7.1 (Tables 1 and 2) |
| [`cg19-ex2-pycnometer-100ml`](flow/cg19-ex2-pycnometer-100ml.json) | Gravimetric calibration of a 100 mL glass pycnometer (EURAMET cg-19 Example 2) | 100 mL'lik cam piknometrenin gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 2) | EURAMET Calibration Guide No. 19 v4.1 (2025) 7.2 (Tables 3 and 4) |
| [`ea402-s12-water-meter-error`](flow/ea402-s12-water-meter-error.json) | Water meter calibration: relative error of one run | Su sayacı kalibrasyonu: tek çalışmanın bağıl hatası | EA-4/02 M:2022 S12 |
| [`ea402-s12-water-meter-mean-error`](flow/ea402-s12-water-meter-mean-error.json) | Water meter calibration: mean of three runs | Su sayacı kalibrasyonu: üç çalışmanın ortalaması | EA-4/02 M:2022 S12 |
| [`ea402-s12-water-meter-volume`](flow/ea402-s12-water-meter-volume.json) | Water meter calibration: reference volume | Su sayacı kalibrasyonu: referans hacim | EA-4/02 M:2022 S12 |
| [`m3003-k9-flowrate-standard-conditions`](flow/m3003-k9-flowrate-standard-conditions.json) | Volume flowrate referred to standard pressure and temperature (UKAS M3003 K.9) | Standart basınç ve sıcaklığa indirgenmiş hacimsel debi (UKAS M3003 K.9) | UKAS M3003 ed. 6 (2024) Appendix K.9 |
| [`nist-tn1900-e4-pitot-tube`](flow/nist-tn1900-e4-pitot-tube.json) | Airspeed with a Pitot tube (NIST TN 1900 E4) | Pitot tüpüyle hava hızı (NIST TN 1900 E4) | NIST Technical Note 1900 Example E4 |

## General (method validation) / Genel (yöntem doğrulama)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`jcgm101-9.2.2-additive-gaussian`](general/jcgm101-9.2.2-additive-gaussian.json) | Additive model with Gaussian inputs | Normal dağılımlı girdilerle toplamsal model | JCGM 101:2008 9.2.2 |
| [`jcgm101-9.2.3-additive-rectangular`](general/jcgm101-9.2.3-additive-rectangular.json) | Additive model with rectangular inputs | Dikdörtgen dağılımlı girdilerle toplamsal model | JCGM 101:2008 9.2.3 |
| [`jcgm101-9.2.4-additive-dominant-rectangular`](general/jcgm101-9.2.4-additive-dominant-rectangular.json) | Additive model with a dominant rectangular input | Baskın dikdörtgen girdili toplamsal model | JCGM 101:2008 9.2.4 |
| [`num-3-ratio-abc`](general/num-3-ratio-abc.json) | Product and ratio y = ab/c with a wide triangular divisor (NIST Uncertainty Machine §3) | Geniş üçgen bölenli çarpım ve oran y = ab/c (NIST Uncertainty Machine §3) | T. Lafarge §3, Figure 1 |

## General (method validation), several output quantities (JCGM 102) / Genel (yöntem doğrulama), birden çok çıktı büyüklüğü (JCGM 102)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`jcgm102-9.2.2-additive-gaussian`](general/jcgm102-9.2.2-additive-gaussian.json) | Additive bivariate model, Gaussian inputs (JCGM 102 9.2.2) | Toplamalı iki değişkenli model, Gauss girdiler (JCGM 102 9.2.2) | JCGM 102:2011 9.2.2 (Table 3) |
| [`jcgm102-9.2.3-additive-rectangular`](general/jcgm102-9.2.3-additive-rectangular.json) | Additive bivariate model, rectangular common effect (JCGM 102 9.2.3) | Toplamalı iki değişkenli model, dikdörtgen ortak etki (JCGM 102 9.2.3) | JCGM 102:2011 9.2.3 (Table 4) |
| [`jcgm102-9.2.4-additive-dominant-rectangular`](general/jcgm102-9.2.4-additive-dominant-rectangular.json) | Additive bivariate model, dominant rectangular common effect (JCGM 102 9.2.4) | Toplamalı iki değişkenli model, baskın dikdörtgen ortak etki (JCGM 102 9.2.4) | JCGM 102:2011 9.2.4 (Table 5) |

## General (method validation), top-down / Genel (yöntem doğrulama), top-down

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`iso21748-c1-co-emissions`](general/iso21748-c1-co-emissions.json) | Carbon monoxide in vehicle emissions from reproducibility data (ISO 21748 C.1) | Taşıt emisyonlarında karbon monoksit, tekrarüretilebilirlik verilerinden (ISO 21748 C.1) | ISO 21748:2017 Annex C.1 |
| [`iso21748-c3-aerobic-plate-count-flour`](general/iso21748-c3-aerobic-plate-count-flour.json) | Aerobic plate count, flour: adjusted reproducibility (ISO 21748 C.3) | Aerobik koloni sayımı, un: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3) | ISO 21748:2017 Annex C.3, Tables C.2–C.4 |
| [`iso21748-c3-aerobic-plate-count-shrimp`](general/iso21748-c3-aerobic-plate-count-shrimp.json) | Aerobic plate count, shrimp: adjusted reproducibility (ISO 21748 C.3) | Aerobik koloni sayımı, karides: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3) | ISO 21748:2017 Annex C.3, Tables C.2–C.4 |
| [`iso21748-c3-aerobic-plate-count-vegetables`](general/iso21748-c3-aerobic-plate-count-vegetables.json) | Aerobic plate count, vegetables: adjusted reproducibility (ISO 21748 C.3) | Aerobik koloni sayımı, sebze: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3) | ISO 21748:2017 Annex C.3, Tables C.2–C.4 |

## General (method validation), control charts (Nordtest TR 569) / Genel (yöntem doğrulama), kontrol grafikleri (Nordtest TR 569)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`tr569-ex11-micropipette-tolerance`](general/tr569-ex11-micropipette-tolerance.json) | Bias of a 1 mL micropipette: chart with action limits at the tolerance (Nordtest TR 569 Example 11) | 1 mL mikropipetin sapması: toleransta eylem sınırlı grafik (Nordtest TR 569 Örnek 11) | Nordtest TR 569 ed. 6 (2026) Chapter 14, Example 11 |

## General (method validation), conformity assessment (JCGM 106) / Genel (yöntem doğrulama), uygunluk değerlendirmesi (JCGM 106)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`jcgm106-7.3-bursting-strength`](general/jcgm106-7.3-bursting-strength.json) | Bursting strength of a container: probability of conformity (JCGM 106 7.3) | Bir kabın patlama dayanımı: uygunluk olasılığı (JCGM 106 7.3) | JCGM 106:2012 7.3.3, Example 2 |
| [`jcgm106-7.3-zener-diode`](general/jcgm106-7.3-zener-diode.json) | Breakdown voltage of a Zener diode: probability of conformity (JCGM 106 7.3) | Zener diyotun kırılma gerilimi: uygunluk olasılığı (JCGM 106 7.3) | JCGM 106:2012 7.3.3, Example 1 |
| [`jcgm106-7.4-motor-oil-viscosity`](general/jcgm106-7.4-motor-oil-viscosity.json) | Kinematic viscosity of a motor oil: two-sided tolerance (JCGM 106 7.4) | Motor yağının kinematik viskozitesi: iki taraflı tolerans (JCGM 106 7.4) | JCGM 106:2012 7.4, Example |
| [`jcgm106-8.3-nandrolone`](general/jcgm106-8.3-nandrolone.json) | Nandrolone in animal products: decision limit for a screening test (JCGM 106 8.3.3) | Hayvansal ürünlerde nandrolon: tarama testi için karar sınırı (JCGM 106 8.3.3) | JCGM 106:2012 8.3.3, Example 2 |
| [`jcgm106-8.3-speed-limit`](general/jcgm106-8.3-speed-limit.json) | Speed limit enforcement: guarded rejection (JCGM 106 8.3.3) | Hız sınırı denetimi: korumalı ret (JCGM 106 8.3.3) | JCGM 106:2012 8.3.3, Example 1 |
| [`jcgm106-9.5-ball-bearings`](general/jcgm106-9.5-ball-bearings.json) | Ball bearings: acceptance limit for a required consumer's risk (JCGM 106 9.5.4) | Bilyalı rulmanlar: istenen tüketici riski için kabul sınırı (JCGM 106 9.5.4) | JCGM 106:2012 9.5.4, Example |
| [`jcgm106-9.5-precision-resistors`](general/jcgm106-9.5-precision-resistors.json) | Precision resistors: global consumer's and producer's risks (JCGM 106 9.5.3) | Hassas dirençler: küresel tüketici ve üretici riskleri (JCGM 106 9.5.3) | JCGM 106:2012 9.5.3, Example |

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
| [`m3003-k10-sheet-thickness`](length/m3003-k10-sheet-thickness.json) | Thickness of an aluminium test sample (UKAS M3003 K.10) | Alüminyum deney numunesinin kalınlığı (UKAS M3003 K.10) | UKAS M3003 ed. 6 (2024) Appendix K.10 |
| [`m3003-k6-gauge-block-10mm-temperature-model`](length/m3003-k6-gauge-block-10mm-temperature-model.json) | Grade 2 gauge block of 10 mm with the temperature terms of its model (UKAS M3003 K.6, EA-4/02 S4) | Modelinin sıcaklık terimleriyle 10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6, EA-4/02 S4) | UKAS M3003 ed. 6 (2024) Appendix K.6 |
| [`m3003-k6-gauge-block-10mm`](length/m3003-k6-gauge-block-10mm.json) | Grade 2 gauge block of 10 mm (UKAS M3003 K.6) | 10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6) | UKAS M3003 ed. 6 (2024) Appendix K.6 |
| [`num-8-end-gauge-gaussian`](length/num-8-end-gauge-gaussian.json) | End-gauge calibration with Gaussian inputs (NIST Uncertainty Machine §8) | Normal girdilerle uç mastarı kalibrasyonu (NIST Uncertainty Machine §8) | T. Lafarge §8, Table 3 and Figure 3 |

## Mass / Kütle

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`cg18-h1-100g-option2`](mass/cg18-h1-100g-option2.json) | Laboratory balance 220 g / 0.1 mg: error at 100 g with the buoyancy correction (EURAMET cg-18 H1, option 2) | Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle 100 g'da hata (EURAMET cg-18 H1, seçenek 2) | EURAMET Calibration Guide No. 18 v4.0 (2015) Appendix H1 |
| [`cg18-h1-220g-max-option1-delta-t`](mass/cg18-h1-220g-max-option1-delta-t.json) | Laboratory balance 220 g / 0.1 mg: error at 220 g with the buoyancy term for ΔT = 5 K (EURAMET cg-18 H1, option 1, formula 7.1.2-5e) | Laboratuvar terazisi 220 g / 0,1 mg: ΔT = 5 K için kaldırma kuvveti terimiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 1, 7.1.2-5e formülü) | EURAMET Calibration Guide No. 18 v4.0 (2015) Appendix H1 |
| [`cg18-h1-220g-max-option1`](mass/cg18-h1-220g-max-option1.json) | Laboratory balance 220 g / 0.1 mg: error at 220 g without buoyancy correction (EURAMET cg-18 H1, option 1) | Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesi olmadan 220 g'da hata (EURAMET cg-18 H1, seçenek 1) | EURAMET Calibration Guide No. 18 v4.0 (2015) Appendix H1 |
| [`cg18-h1-220g-max-option2`](mass/cg18-h1-220g-max-option2.json) | Laboratory balance 220 g / 0.1 mg: error at 220 g with the buoyancy correction (EURAMET cg-18 H1, option 2) | Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 2) | EURAMET Calibration Guide No. 18 v4.0 (2015) Appendix H1 |
| [`cg18-h1-220g-zero`](mass/cg18-h1-220g-zero.json) | Laboratory balance 220 g / 0.1 mg: error at zero load (EURAMET cg-18 H1) | Laboratuvar terazisi 220 g / 0,1 mg: sıfır yükte hata (EURAMET cg-18 H1) | EURAMET Calibration Guide No. 18 v4.0 (2015) Appendix H1 |
| [`ea402-s2-weight-10kg`](mass/ea402-s2-weight-10kg.json) | Calibration of a 10 kg weight | 10 kg'lık bir ağırlığın kalibrasyonu | EA-4/02 M:2022 S2 |
| [`jcgm101-9.3-mass-calibration-higher-order`](mass/jcgm101-9.3-mass-calibration-higher-order.json) | Mass calibration (GUM with second-order terms) | Kütle kalibrasyonu (ikinci mertebe terimli GUM) | JCGM 101:2008 9.3, Table 6 |
| [`jcgm101-9.3-mass-calibration`](mass/jcgm101-9.3-mass-calibration.json) | Mass calibration (first-order GUM) | Kütle kalibrasyonu (birinci mertebe GUM) | JCGM 101:2008 9.3, Tables 6 and 7 |
| [`m3003-k4-weight-10kg`](mass/m3003-k4-weight-10kg.json) | 10 kg weight of OIML class M1 (UKAS M3003 K.4) | OIML M1 sınıfı 10 kg ağırlık (UKAS M3003 K.4) | UKAS M3003 ed. 6 (2024) Appendix K.4 |
| [`m3003-k5-weighing-machine-200g`](mass/m3003-k5-weighing-machine-200g.json) | Weighing machine of 205 g capacity at 200 g (UKAS M3003 K.5) | 200 g'de 205 g kapasiteli tartı aleti (UKAS M3003 K.5) | UKAS M3003 ed. 6 (2024) Appendix K.5 |
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

## Physical properties, interlaboratory precision (ISO 5725-2) / Fiziksel özellikler, laboratuvarlar arası kesinlik (ISO 5725-2)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`iso5725-2-c2-softening-point-pitch-reml`](physical/iso5725-2-c2-softening-point-pitch-reml.json) | Softening point of pitch: REML estimates (ISO 5725-2 Example C.2) | Ziftin yumuşama noktası: REML tahminleri (ISO 5725-2 Örnek C.2) | ISO 5725-2:2019 Annex C.2.9 (Table C.13), Annex B.2 |
| [`iso5725-2-c2-softening-point-pitch`](physical/iso5725-2-c2-softening-point-pitch.json) | Softening point of pitch: repeatability and reproducibility with missing data (ISO 5725-2 Example C.2) | Ziftin yumuşama noktası: eksik verilerle tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2 Örnek C.2) | ISO 5725-2:2019 Annex C.2 (Tables C.7–C.12, Formulae C.25–C.33) |

## Pressure / Basınç

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`m3003-k8-pressure-indicator-2mpa`](pressure/m3003-k8-pressure-indicator-2mpa.json) | Digital pressure indicator at 2 MPa against a dead weight tester (UKAS M3003 K.8) | Ölü ağırlık test cihazına karşı 2 MPa'da dijital basınç göstergesi (UKAS M3003 K.8) | UKAS M3003 ed. 6 (2024) Appendix K.8 |

## Temperature / Sıcaklık

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`ea402-s11-block-calibrator-180c`](temperature/ea402-s11-block-calibrator-180c.json) | Calibration of a temperature block calibrator at 180 °C | Sıcaklık blok kalibratörünün 180 °C'de kalibrasyonu | EA-4/02 M:2022 S11 |
| [`ea402-s5-thermocouple-emf`](temperature/ea402-s5-thermocouple-emf.json) | Thermocouple calibration: emf of the unknown | Termokupl kalibrasyonu: bilinmeyenin emk'sı | EA-4/02 M:2022 S5 |
| [`ea402-s5-thermocouple-furnace`](temperature/ea402-s5-thermocouple-furnace.json) | Thermocouple calibration: furnace temperature | Termokupl kalibrasyonu: fırın sıcaklığı | EA-4/02 M:2022 S5 |
| [`m3003-k11-gas-temperature`](temperature/m3003-k11-gas-temperature.json) | Temperature of gas in a container from a calibrated PT100 probe (UKAS M3003 K.11, second budget) | Kalibre edilmiş PT100 probdan kaptaki gazın sıcaklığı (UKAS M3003 K.11, ikinci bütçe) | UKAS M3003 ed. 6 (2024) Appendix K.11 (K.11.17–K.11.18) |
| [`m3003-k11-probe-calibration`](temperature/m3003-k11-probe-calibration.json) | Calibration of a PT100 probe in a dry block against a reference probe (UKAS M3003 K.11, first budget) | Kuru blokta referans proba karşı PT100 prob kalibrasyonu (UKAS M3003 K.11, birinci bütçe) | UKAS M3003 ed. 6 (2024) Appendix K.11 (K.11.9) |
| [`m3003-k7-thermocouple-emf`](temperature/m3003-k7-thermocouple-emf.json) | N-type thermocouple at 1000 °C: EMF with its full budget (UKAS M3003 K.7) | 1000 °C'de N tipi termoçift: tam bütçesiyle EMK (UKAS M3003 K.7) | UKAS M3003 ed. 6 (2024) Appendix K.7 |
| [`m3003-k7-thermocouple-furnace`](temperature/m3003-k7-thermocouple-furnace.json) | Temperature of the furnace in a thermocouple calibration at 1000 °C (UKAS M3003 K.7, first budget) | 1000 °C'de termoçift kalibrasyonunda fırın sıcaklığı (UKAS M3003 K.7, birinci bütçe) | UKAS M3003 ed. 6 (2024) Appendix K.7 (K.7.19) |
| [`nist-tn1900-e2-surface-temperature`](temperature/nist-tn1900-e2-surface-temperature.json) | Monthly mean of daily maximum temperature (NIST TN 1900 E2) | Günlük en yüksek sıcaklığın aylık ortalaması (NIST TN 1900 E2) | NIST Technical Note 1900 Example E2, Exhibit 2 |

## Temperature, several output quantities (JCGM 102) / Sıcaklık, birden çok çıktı büyüklüğü (JCGM 102)

| File / Dosya | Title | Başlık | Source / Kaynak |
|---|---|---|---|
| [`jcgm102-9.5.2-resistance-thermometer`](temperature/jcgm102-9.5.2-resistance-thermometer.json) | Celsius temperature from a resistance thermometer, implicit model (JCGM 102 9.5.2) | Direnç termometresiyle Celsius sıcaklığı, örtük model (JCGM 102 9.5.2) | JCGM 102:2011 9.5.2 (Tables 12, 13) |
| [`jcgm102-9.5.3-ten-temperatures`](temperature/jcgm102-9.5.3-ten-temperatures.json) | Ten Celsius temperatures with one resistance thermometer, implicit model (JCGM 102 9.5.3) | Tek direnç termometresiyle on Celsius sıcaklığı, örtük model (JCGM 102 9.5.3) | JCGM 102:2011 9.5.3 (Tables 14–16) |

## Adding an example / Örnek ekleme

Start from `uncertainty template`, give the `metadata.id` the file name, cite the source, and put the
published values with their rounding tolerance under `expected`. Explain every deviation from the source in
`metadata.notes` (English and Turkish).

`uncertainty template` çıktısından başlayın, `metadata.id` alanına dosya adını verin, kaynağı belirtin ve
yayımlanmış değerleri yuvarlama toleranslarıyla `expected` altına yazın. Kaynaktan her sapmayı
`metadata.notes` içinde (İngilizce ve Türkçe) açıklayın.
