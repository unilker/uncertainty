# Temin edilmesi gereken belgeler / Documents to obtain

Bu liste, programın ve makalenin eksiksiz olması için gerekli olup **ücretli
olduğu veya erişimimiz olmadığı için** henüz okunamayan belgeleri içerir.
Ücretsiz belgeler (JCGM 100/101/102/104/106/200, GUM-6, GUM-1, EA-4/02 M:2022,
NIST TN 1900, Eurachem QUAM 2012, Eurachem MUPR 2026, Nordtest TR 537,
Suncal makalesi) indirildi; okunanlar `literature-review.md`'de işaretli.

Öneri: TÜBİTAK UME kütüphanesi / TSE aboneliği (TS EN ISO karşılıkları da
kabul edilir) ve kurumsal ScienceDirect, Springer, RSC, IOP erişimi.
Belgeyi bana verdiğinizde ilgili modülü tamamlayıp doğrulama testlerine
eklerim. Durum sütununu belge geldikçe güncelleyeceğim.

Lisanslı standartlar (ISO, BS, TSE kopyaları) yalnızca çalışma ortamında
kullanılır, depoya konmaz; kütüphaneye yalnızca standartlardaki örneklerin
sayısal verileri ve sonuçları kaynak gösterilerek alınır.

**2026-10-26 güncellemesi (2):** ISO 5725-2 (2019 ve 1994) ve ISO 5725-3 (2023 ve 1994, düzeltmesiyle) alındı ve kullanıldı (D40, D41).

**2026-10-26 güncellemesi:** 13 belge alındı (A2, A4'ün 3. bölümü, A5, A6,
A7; B2, B4, B6, B8, B9, B10, B7 yerine NUM kılavuzu ve ek olarak Kallner).
Eksik kalanların özeti en sondaki "Hâlâ eksik olanlar" bölümündedir.

## A. Standartlar (öncelik sırasıyla)

| # | Belge | Ne için gerekli | Öncelik | Durum |
|---|---|---|---|---|
| 1 | **ISO 11352:2012** Water quality — Estimation of measurement uncertainty based on validation and quality control data | Top-down modülünün dayandığı standart; Nordtest bunu uygular, kendi örnekleri doğrulama vakası olur | Yüksek | bekleniyor |
| 2 | **ISO 21748:2017** Guidance for the use of repeatability, reproducibility and trueness estimates in measurement uncertainty evaluation | Tekrarüretilebilirlik (s_R) tabanlı belirsizlik; top-down modülü | Yüksek | **alındı (2026-10-26)** |
| 3 | **ISO/TS 28037:2010** Determination and use of straight-line calibration functions | Kalibrasyon eğrisi belirsizliği (kimyada temel); standarttaki örnek veri setleri doğrulama için şart | Yüksek | bekleniyor |
| 4 | **ISO 5725-2** (2019) ve mümkünse **ISO 5725-1, -4, -6** (güncel baskılar) Accuracy (trueness and precision) of measurement methods and results | Tekrarlanabilirlik/tekrarüretilebilirlik, ANOVA, Cochran/Grubbs testleri, kritik aralık faktörleri (bulgu F9: 3.69 vs 3.64) | Yüksek | **ISO 5725-2:2019 ve :1994, ISO 5725-3:2023 ve :1994 alındı (2026-10-26)**; -1, -4, -6 bekleniyor |
| 5 | **ISO 13528:2022** Statistical methods for use in proficiency testing by interlaboratory comparison | Yeterlilik testi verisinden u(C_ref), robust istatistikler (Nordtest'teki 1.25 faktörü), z/ζ/En skorları | Orta | **alındı** |
| 6 | **ISO/IEC 17025:2017** (TS EN ISO/IEC 17025) | 7.6 belirsizlik, 7.8.6 uygunluk beyanı, 7.11 yazılım validasyonu — rapor şablonu ve validasyon raporu | Orta | **alındı (TS EN, Aralık 2017)** |
| 7 | **ISO 6143** (güncel baskı) Gas analysis — Comparison methods for determining and checking the composition of calibration gas mixtures | Her iki eksende belirsizlik olan kalibrasyon (gaz analizi) | Düşük | **alındı (EN ISO 6143:2025)** |
| 8 | **ISO 11843-1/-2** Capability of detection | Tespit ve tayin sınırı modülü | Düşük | bekleniyor |
| 9 | **ISO/TS 20914:2019** Medical laboratories — Practical guidance for the estimation of measurement uncertainty | Klinik/tıbbi laboratuvar örnekleri (ileride) | Düşük | bekleniyor |

## B. Makaleler

| # | Kaynak | Ne için gerekli | Öncelik | Durum |
|---|---|---|---|---|
| 1 | Cordeiro R., Rosa C., Bettencourt da Silva R., "Measurements recovery evaluation from the analysis of independent reference materials…", *Accred. Qual. Assur.* 23 (2018) 57–71 | MUPR Eq. (9)–(12)'nin türetildiği kaynak | Yüksek | bekleniyor |
| 2 | Magnusson B., Ellison S.L.R., *Anal. Bioanal. Chem.* 390 (2008) 201–213 | Düzeltilmeyen sapma yaklaşımları (Q6) | Yüksek | **alındı** |
| 3 | O'Donnell G.E., Hibbert D.B., "Treatment of bias in estimating measurement uncertainty", *Analyst* 130 (2005) 721–729 | Aynı konu, alternatif yöntemler | Orta | bekleniyor |
| 4 | Palma C., Morgado V., Bettencourt da Silva R., *Talanta* 192 (2019) 278–287 | Matris etkisi belirsizliği (MUPR B5–B6) | Orta | **alındı** |
| 5 | Kragten J., "Calculating standard deviations and confidence intervals with a universally applicable spreadsheet technique", *Analyst* 119 (1994) 2161–2166 | EURACHEM'in kullandığı sayısal yöntem; Kragten görünümünün birincil kaynağı | Orta | bekleniyor |
| 6 | Wichmann B.A., Hill I.D., *Comput. Stat. Data Anal.* 51 (2006) 1614–1622 | Rastgele sayı üreticisinin birincil kaynağı | Orta | **alındı, doğrulandı (D29)** |
| 7 | Lafarge T., Possolo A., "The NIST Uncertainty Machine", *NCSLI Measure* 10(3) (2015) 20–27 | Karşılaştırmalı değerlendirme; makalede atıf | Orta | kullanıcı kılavuzu (v1.4) alındı ve kullanıldı (D30); makalenin kendisi bekleniyor |
| 8 | Zeier M. ve ark., *Metrologia* 49 (2012) 809 | Rakip yazılım analizi | Düşük | **alındı** |
| 9 | Marsaglia G., Tsang W.W., *ACM TOMS* 26 (2000) 363–372 | Gamma üreticisinin kaynağı | Düşük | **alındı, doğrulandı (D29)** |
| 10 | Mateos R. ve ark., *Talanta* 207 (2020) 120274 | Ağırlıklı geri kazanım (MUPR dipnot) | Düşük | **alındı** |
| 11 | Kallner A., "Uncertainty in measurement — introduction and examples from laboratory medicine", *eJIFCC* 13(1) (2002) | Klinik laboratuvar örnekleri (ek, istenmemişti) | — | **alındı** |
| 12 | Wilrich P.-T., "Critical values of Mandel's h and k, the Grubbs and the Cochran test statistic", *AStA Adv. Stat. Anal.* 97 (2013) 1–10 | ISO 5725-2 Ek D formüllerinin kaynağı; D.3 yaklaşımının (±0,003) doğruluğunu ve Tablo D.1 katsayılarını kontrol etmek için | Düşük | bekleniyor |

## C. Literatür taraması için erişim

| # | İhtiyaç | Neden |
|---|---|---|
| 1 | Scopus veya Web of Science erişimi (kurumsal) | Makale öncesi sistematik literatür taraması (literature-review.md §4) ve yenilik iddialarının kontrolü |
| 2 | Monte Carlo "en kısa kapsam aralığı" dağılımı üzerine önceki çalışmalar | Bulgu F3'ün yeni olup olmadığını doğrulamak (Q3) |
| 3 | Hedef dergilerin güncel JCR/SJR çeyrek (Q) bilgisi | Dergi seçimi (publication-plan.md) |

## D. Kontrol edilmesi gereken basılı baskılar / errata

| # | Belge | Kontrol edilecek |
|---|---|---|
| 1 | EURACHEM/CITAC QUAM:2012, Tablo A2.4, A3.1, A5 (u(a_V)), A6.5 | Basılı baskı veya errata: F10, F12, F14, F15 |
| 2 | Nordtest TR 537 ed. 4, §6.1 | 2.16 % / 2.17 % hesap hatası (F5) için güncel düzeltme var mı |
| 3 | EA-4/02 M:2022 (S5, S7, S13) | güncel revizyon / errata (F18) |
| 4 | NIST TN 1900 (2015) E3, E4, E9 | errata / güncel sürüm (F28–F30) — **kontrol edildi (2026-10-26): yeni sürüm veya errata yok (D36)** |
| 5 | NIST Uncertainty Machine kılavuzu v1.4, §3 ve §9 | daha yeni sürüm (F29, F33) — **kontrol edildi: v1.4 güncel (D36)** |
| 6 | ISO 6143:2025, Formül (12) ve Tablo B.12 | düzeltme (corrigendum/amendment) yayımlanmış mı; B_LEAST'in güncel sürümü üstel fonksiyon belirsizliklerini düzeltmiş mi (F41, F42) |
| 7 | ISO 5725-3:2023, Tablo J.2 ve J.5 | **çözüldü (D41):** 1994 baskısının verileri J.5'i yeniden üretiyor; 2023 Tablo J.2'de 15 hatalı giriş var. Yalnızca bir düzeltme (corrigendum) yayımlanıp yayımlanmadığı kontrol edilebilir |
| 11 | ISO 5725-2:2019, C.1.5, Tablo C.12 ve Tablo 4 | düzeltme yayımlanmış mı (F47–F49; 1994 baskısında da aynı) |
| 8 | ISO 13528:2022, Tablo E.6 | belirsizlik bayraklarının ölçütü (F36) |
| 9 | ISO 21748:2017, C.3 ve C.4.7 | un için %6,4 / %12,8 ve "3'e bölme" (F38, F39) |
| 10 | Magnusson ve Ellison (2008), Tablo 3 | SUMU işaret hatası için düzeltme notu var mı (F43) |

## E. Yazar tarafından sağlanabilecek veriler (isteğe bağlı, çok değerli)

- UME veya iş birliği yapılan laboratuvarlardan **anonimleştirilmiş gerçek
  validasyon/kalite kontrol verisi** (kontrol kartı, mükerrer ölçümler,
  CRM ve yeterlilik testi sonuçları): makalede "vaka çalışması" olarak
  kullanılabilir, özgün katkıyı güçlendirir.
- Kullanılması planlanan **Türkçe terim karşılıkları** (UME/TSE kullanımı):
  rehberin Türkçe sözlüğü için.

## Hâlâ eksik olanlar (2026-10-26, güncellendi)

Öncelik sırasıyla:

1. **ISO 5725-6:1994** (ve mümkünse **ISO 5725-1, -4**) — kritik aralık faktörleri f(n) (F9: MUPR B1'deki 3,69), tekrarlanabilirlik ve tekrarüretilebilirlik sınırları (r = 2,8 s_r), laboratuvarlar arası karşılaştırmada kullanım; -4 doğruluk (trueness) tayini. ISO 5725-2 alındı ve uygulandı (D40).
2. **ISO 11352:2012** — top-down modülünün dayandığı su kalitesi standardı.
3. **ISO/TS 28037:2010** — doğru kalibrasyon fonksiyonları (her iki eksende belirsizlik); standarttaki örnek veri setleri doğrulama için gerekli. ISO 6143:2025 ile genelleştirilmiş en küçük kareler motoru yazıldı ve doğrulandı (D33); ISO/TS 28037'nin kendi veri setleri ek doğrulama için hâlâ gerekli.
4. **Cordeiro ve ark. (2018)**, *Accred. Qual. Assur.* 23:57–71 — MUPR geri kazanım eşitliklerinin kaynağı.
5. **Kragten (1994)**, *Analyst* 119:2161 — Kragten görünümünün birincil kaynağı.
6. **O'Donnell ve Hibbert (2005)**, *Analyst* 130:721 — sapma yaklaşımlarının karşılaştırması; Magnusson ve Ellison'un benzetimiyle çelişen kapsam sonuçları var (D34), karşılaştırmak için gerekli.
6a. **Synek (2005)**, *Talanta* 65:829–837 — U_e yaklaşımının birincil kaynağı; program U_e'yi tanımından hesaplıyor, Synek'in E çarpanlarıyla karşılaştırmak için gerekli.
6c. **Palma ve ark. (2019) ve Mateos ve ark. (2020) elektronik ekleri** (Excel dosyaları) — ham veriler; Palma'nın geri kazanım uyumluluk testlerini (D39) ve Mateos'un Aralık II tutarsızlığını (F46) kontrol etmek için. Talanta'nın sayfasından "Supplementary material" olarak indirilebilir.
6b. **Maroto ve ark. (2002)**, *Accred. Qual. Assur.* — düzeltilmemiş sapma için Monte Carlo kapsam çalışması (düşük öncelik).
7. **Lafarge ve Possolo (2015)**, *NCSLI Measure* 10(3):20 — NIST Uncertainty Machine makalesi (atıf için).
8. **ISO 11843-1/-2** — tespit sınırı modülü (düşük öncelik).
9. **ISO/TS 20914:2019** — tıbbi laboratuvarlar (düşük öncelik; Kallner makalesi başlangıç için yeterli).
10. **Wilrich (2013)**, *AStA Adv. Stat. Anal.* 97:1–10 — ISO 5725-2 Ek D kritik değer formüllerinin kaynağı (düşük öncelik; tablolar ve formüller standartta mevcut).

Erişim ihtiyaçları (C bölümü) değişmedi. D bölümüne NIST TN 1900 (F28–F30), NUM kılavuzu (F29, F33),
ISO 6143 (F41, F42), ISO 5725-3 (F40, çözüldü), ISO 5725-2 (F47–F49), ISO 13528 (F36), ISO 21748 (F38, F39) ve
Magnusson ve Ellison (F43) eklendi.
