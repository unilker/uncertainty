# Temin edilmesi gereken belgeler / Documents to obtain

Bu liste, programın ve makalenin eksiksiz olması için gerekli olup **ücretli
olduğu veya erişimimiz olmadığı için** henüz okunamayan belgeleri içerir.
Ücretsiz belgeler (JCGM 100/101/102/104/106/200, GUM-6, GUM-1, EA-4/02 M:2022,
NIST TN 1900, Eurachem QUAM 2012, Eurachem MUPR 2026, Nordtest TR 537)
zaten indirilmiş ve okunmaktadır.

Öneri: TÜBİTAK UME kütüphanesi / TSE aboneliği (TS EN ISO karşılıkları da
kabul edilir) ve kurumsal ScienceDirect, Springer, RSC, IOP erişimi.
Belgeyi bana verdiğinizde ilgili modülü tamamlayıp doğrulama testlerine
eklerim. Durum sütununu belge geldikçe güncelleyeceğim.

## A. Standartlar (öncelik sırasıyla)

| # | Belge | Ne için gerekli | Öncelik | Durum |
|---|---|---|---|---|
| 1 | **ISO 11352:2012** Water quality — Estimation of measurement uncertainty based on validation and quality control data | Top-down modülünün dayandığı standart; Nordtest bunu uygular, kendi örnekleri doğrulama vakası olur | Yüksek | bekleniyor |
| 2 | **ISO 21748:2017** Guidance for the use of repeatability, reproducibility and trueness estimates in measurement uncertainty evaluation | Tekrarüretilebilirlik (s_R) tabanlı belirsizlik; top-down modülü | Yüksek | bekleniyor |
| 3 | **ISO/TS 28037:2010** Determination and use of straight-line calibration functions | Kalibrasyon eğrisi belirsizliği (kimyada temel); standarttaki örnek veri setleri doğrulama için şart | Yüksek | bekleniyor |
| 4 | **ISO 5725-2** (2019) ve mümkünse **ISO 5725-1, -3, -4, -6** (güncel baskılar) Accuracy (trueness and precision) of measurement methods and results | Tekrarlanabilirlik/tekrarüretilebilirlik, ara kesinlik, ANOVA, Cochran/Grubbs testleri, kritik aralık faktörleri (bulgu F9: 3.69 vs 3.64) | Yüksek | bekleniyor |
| 5 | **ISO 13528:2022** Statistical methods for use in proficiency testing by interlaboratory comparison | Yeterlilik testi verisinden u(C_ref), robust istatistikler (Nordtest'teki 1.25 faktörü), z/ζ/En skorları | Orta | bekleniyor |
| 6 | **ISO/IEC 17025:2017** (TS EN ISO/IEC 17025) | 7.6 belirsizlik, 7.8.6 uygunluk beyanı, 7.11 yazılım validasyonu — rapor şablonu ve validasyon raporu | Orta | bekleniyor |
| 7 | **ISO 6143** (güncel baskı) Gas analysis — Comparison methods for determining and checking the composition of calibration gas mixtures | Her iki eksende belirsizlik olan kalibrasyon (gaz analizi) | Düşük | bekleniyor |
| 8 | **ISO 11843-1/-2** Capability of detection | Tespit ve tayin sınırı modülü | Düşük | bekleniyor |
| 9 | **ISO/TS 20914:2019** Medical laboratories — Practical guidance for the estimation of measurement uncertainty | Klinik/tıbbi laboratuvar örnekleri (ileride) | Düşük | bekleniyor |

## B. Makaleler

| # | Kaynak | Ne için gerekli | Öncelik |
|---|---|---|---|
| 1 | Cordeiro R., Rosa C., Bettencourt da Silva R., "Measurements recovery evaluation from the analysis of independent reference materials…", *Accred. Qual. Assur.* 23 (2018) 57–71 | MUPR Eq. (9)–(12)'nin türetildiği kaynak | Yüksek |
| 2 | Magnusson B., Ellison S.L.R., "Treatment of uncorrected measurement bias in uncertainty estimation for chemical measurements", *Anal. Bioanal. Chem.* 390 (2008) 201–213 | Düzeltilmeyen sapma yaklaşımları (Q6) | Yüksek |
| 3 | O'Donnell G.E., Hibbert D.B., "Treatment of bias in estimating measurement uncertainty", *Analyst* 130 (2005) 721–729 | Aynı konu, alternatif yöntemler | Orta |
| 4 | Palma C., Morgado V., Bettencourt da Silva R., "Top-down evaluation of matrix effects uncertainty", *Talanta* 192 (2019) 278–287 | Matris etkisi belirsizliği (MUPR B5–B6) | Orta |
| 5 | Kragten J., "Calculating standard deviations and confidence intervals with a universally applicable spreadsheet technique", *Analyst* 119 (1994) 2161–2166 | EURACHEM'in kullandığı sayısal yöntem; makalede karşılaştırma | Orta |
| 6 | Wichmann B.A., Hill I.D., "Generating good pseudo-random numbers", *Comput. Stat. Data Anal.* 51 (2006) 1614–1622 | Kullandığımız rastgele sayı üreticisinin birincil kaynağı | Orta |
| 7 | Lafarge T., Possolo A., "The NIST Uncertainty Machine", *NCSLI Measure* 10(3) (2015) 20–27 | Karşılaştırmalı değerlendirme (benchmark) | Orta |
| 8 | Zeier M., Hoffmann J., Wollensack M., "Metas.UncLib — a measurement uncertainty calculator for advanced problems", *Metrologia* 49 (2012) | Rakip yazılım analizi | Düşük |
| 9 | Marsaglia G., Tsang W.W., "A simple method for generating gamma variables", *ACM TOMS* 26 (2000) 363–372 | Gamma dağılımı üreticisinin kaynağı | Düşük |
| 10 | Mateo R. ve ark., "Impact of recovery correction or subjecting calibrators to sample preparation on measurement uncertainty: PAH determinations in waters", *Talanta* 207 (2020) 120274 | Ağırlıklı geri kazanım (MUPR dipnot) | Düşük |

## C. Literatür taraması için erişim

| # | İhtiyaç | Neden |
|---|---|---|
| 1 | Scopus veya Web of Science erişimi (kurumsal) | Makale öncesi sistematik literatür taraması (literature-review.md §4) ve yenilik iddialarının kontrolü |
| 2 | Monte Carlo "en kısa kapsam aralığı" dağılımı üzerine önceki çalışmalar | Bulgu F3'ün yeni olup olmadığını doğrulamak (Q3) |
| 3 | Hedef dergilerin güncel JCR/SJR çeyrek (Q) bilgisi | Dergi seçimi (publication-plan.md) |

## D. Kontrol edilmesi gereken basılı baskılar / errata

| # | Belge | Kontrol edilecek |
|---|---|---|
| 1 | EURACHEM/CITAC QUAM:2012, Tablo A2.4 | Basılı baskı veya errata: sütun kayması şüphesi (bulgu F10) |
| 2 | Nordtest TR 537 ed. 4, §6.1 | 2.16 % / 2.17 % hesap hatası (F5) için güncel düzeltme var mı |

## E. Yazar tarafından sağlanabilecek veriler (isteğe bağlı, çok değerli)

- UME veya iş birliği yapılan laboratuvarlardan **anonimleştirilmiş gerçek
  validasyon/kalite kontrol verisi** (kontrol kartı, mükerrer ölçümler,
  CRM ve yeterlilik testi sonuçları): makalede "vaka çalışması" olarak
  kullanılabilir, özgün katkıyı güçlendirir.
- Kullanılması planlanan **Türkçe terim karşılıkları** (UME/TSE kullanımı):
  rehberin Türkçe sözlüğü için.
