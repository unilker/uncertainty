# 12. Laboratuvarlar arası kesinlik deneyleri (ISO 5725-2)

`precision_experiment` bölümü ISO 5725-2:2019'un temel yöntemini uygular. Yöntem, bir standart ölçüm
yönteminin tekrarlanabilirlik ve tekrarüretilebilirlik standart sapmalarını bir deneyden tahmin eder. Bu
deneyde p laboratuvarın her biri q düzeyi (malzemeyi) n kez ölçer. Uygulama, Ek C'deki üç örneğin tamamıyla
(REML seçenekleri dahil; `iso5725-2-*` kütüphane projeleri) ve Tablo 1–8 ile A.1 ile doğrulanmıştır.

ISO/IEC 17025:2017 7.6.3 Not 3, deney sonuçlarının belirsizliğini tahmin etmek için ISO 5725 serisini bir
temel olarak gösterir. Burada bulunan s_R, ISO 21748 yaklaşımının (bölüm 5) çıkış noktasıdır.

## 12.1 Veriler: Form A, B ve C

Sonuçlar düzey düzey girilir. Her düzeyde her laboratuvarın bir sonuç listesi vardır: bu, o laboratuvar ve
düzeyin *hücresidir* (Form A, 8.2.9). Eksik bir hücre boş bir listedir. Program hücrelerden şunları hesaplar:

- hücre ortalamaları (Form B, 8.2.10);
- hücre standart sapmaları (Form C, 8.2.11); tek sonuçlu bir hücrenin ortalaması vardır, standart sapması
  yoktur (8.4.3 b).

Rapordaki formlarda sonuçlardan bir fazla ondalık bulunur. `form_decimals` verilirse ortalamalar ve standart
sapmalar kaydedildikleri gibi yuvarlanır; testler ve varyanslar yuvarlanmış değerlerden hesaplanır.
Standardın örnekleri bu şekilde çözülmüştür (C.1.6): test istatistikleri ancak aynı yuvarlama kullanılırsa
basılı basamaklara kadar tutar.

## 12.2 Tutarlılık ve aykırı değer incelemesi (8.3)

Program tüm testleri tüm sonuçlara uygular ve raporlar:

| Test | İstatistik | Kritik değerler |
|---|---|---|
| Mandel h (8.3.2) | (ȳ_i − m̂)/s(ȳ), laboratuvar ve düzeye göre | Formül D.5 göstergeleri (Tablo 7, 8) |
| Mandel k (8.3.2) | s_i √p / √Σs² | Formül D.6 göstergeleri (Tablo 7, 8) |
| Cochran (8.3.4) | C = s²maks / Σs² | Formül D.1 (Tablo 5); n, hücrelerin çoğunluğundaki sonuç sayısıdır (8.3.4.3) |
| Grubbs, tek değer (8.3.5.1) | hücre ortalamalarının G = (x_maks − x̄)/s ve (x̄ − x_min)/s değerleri | Formül D.2 (Tablo 6) |
| Grubbs, iki değer (8.3.5.2) | G = s²(en yüksek veya en düşük iki değer olmadan)/s₀² | p ≤ 40 için Tablo 6; ötesinde Formül D.3 |

%5 kritik değerini aşan bir istatistik *şüpheli değer* (straggler, \*), %1 kritik değerini aşan bir istatistik
*istatistiksel aykırı değer* (\*\*) olarak işaretlenir (8.3.3.1). Cochran testi, en yüksek standart sapma
aykırı olduğu sürece o değer çıkarılarak tekrarlanır (8.3.4.6). 8.3.5.3'teki Grubbs yöntemi izlenir. Tekli
test aykırı bir değer bulursa diğer uç bu değer çıkarılarak test edilir ve ikili testler uygulanmaz. Aksi
hâlde ikili testler uygulanır.

Program kendiliğinden hiçbir şeyi çıkarmaz. Çıkarma, teknik bir açıklama arandıktan sonra panelin vereceği
bir karardır (8.3.3.1 b, 8.6.6–8.6.10). Her çıkarmayı nedeniyle birlikte `exclude` içine kaydedin:

- bütün bir laboratuvar (`"laboratory": "1"`);
- bir hücre (ayrıca `"level"`);
- tek bir sonuç (ayrıca 1'den başlayarak numaralanan `"result"`).

Rapor bu durumda formlarda çıkarılan değerleri köşeli parantez içinde gösterir ve incelemeyi kalan sonuçlar
üzerinde tekrarlar. İstatistiksel aykırı değerler hâlâ tutuluyorsa rapor bunu belirtir: 8.3.3.1 c) geçerli
bir neden olmadıkça bunların atılmasını öngörür.

## 12.3 Genel ortalama ve kesinlik (8.4)

`"method": "anova"` (varsayılan) ile program Formül (23)–(31)'i kullanır:

- m̂ = Σn_i ȳ_i / Σn_i;
- s_r² = Σ(n_i − 1)s_i² / Σ(n_i − 1);
- s_d² = Σn_i(ȳ_i − m̂)²/(p − 1), n̄ = (Σn_i − Σn_i²/Σn_i)/(p − 1) ile;
- s_L² = (s_d² − s_r²)/n̄, negatifse sıfır alınır (8.4.5.4);
- s_R² = s_L² + s_r².

Bu formüller eşit olmayan sonuç sayılarında da geçerlidir. Her hücrede iki sonuç varken Formül (29) ve (30)'a
indirgenirler.

`"method": "reml"` ile varyans bileşenleri kısıtlı en çok olabilirlikle tahmin edilir (8.4.6.2, Ek B.2).
Olabilirlik (B.4), σ_r² üzerinden kapalı biçimde, γ = σ_L²/σ_r² ≥ 0 üzerinden sayısal olarak en büyüklenir.
Bu durumda genel ortalama Formül B.5'in ağırlıklı ortalamasıdır; standart hatası Formül B.7 ile verilir.
Dengeli verilerde REML ve ANOVA, σ_L²'nin ANOVA tahmini negatif olmadıkça aynı tahminleri verir. Eşit olmayan
n_ij'de iki yöntem biraz farklıdır (Tablo C.5 ve C.6'da olduğu gibi). Program, 8.4.6.2'nin REML yazılımından
istediği gibi, standardın REML tablolarını basılı basamaklara kadar yeniden üretir.

Rapor her düzey için Ek A'nın A_r ve A_R değerlerini de verir (Formül A.3 ve A.4, γ = s_R/s_r). Bunlar s_r ve
s_R'nin σ_r ve σ_R'den yaklaşık %95 olasılıkla ne kadar farklı olabileceğini gösteren göreli miktarlardır.
Planlama içindir (kaç laboratuvar gerekir?), güven sınırı olarak kullanılmamalıdır.

## 12.4 Düzeye bağlılık (8.5)

Üç veya daha fazla düzey varken program 8.5.1'in dört ilişkisini s_r ve s_R'ye uydurur:

| | İlişki | Uydurma |
|---|---|---|
| I | s = b·m | b = Σ(s_j/m_j)/q (Formül 39) |
| II | s = a + b·m | 1/s² ağırlıklı regresyon, ardından uydurulan değerlerle bir kez daha (8.5.2.5) |
| III | s² = a_v² + (b_v·m)² | s²'nin m²'ye 1/s⁴ ağırlıklı regresyonu, ardından bir kez daha (8.5.3.2) |
| IV | lg s = c + d·lg m | ağırlıksız regresyon (8.5.4) |

Rapor her düzeydeki uydurulmuş değerleri listeler; s_r ve s_R'yi m̂'ye karşı uydurulan eğrilerle çizer.
İlişkiyi grafiğe ve yöntem hakkında bilinenlere göre seçin. Standart burada teknik bir değerlendirme bekler
(8.5.1, C.3.7). Belirgin bir bağlılık yoksa tüm aralık için tek bir değer verin (C.1.7, C.2.7).

## 12.5 Standartta bulunan noktalar

ISO 5725-2:2019'un her sayısını yeniden üretmek şu noktaları ortaya çıkardı. Hepsi araştırma günlüğüne
kaydedilmiştir ve her biri ISO 5725-2:1994'te de bulunur.

- **F47:** C.1.5, 2. ve 4. düzeylerdeki ikili-yüksek sonuçların şüpheli olduğunu söyler. Oysa 4. düzey için
  Tablo C.4, %5 kritik değeri 0,1101'in üstünde 0,132 verir. C.1.8 dört şüpheli değer bildirir. Tablo C.4 ve
  Formül (C.3) ise üç tane verir: 2. düzeydeki çift ve 3. düzeyde 5. laboratuvarın hücresi.
- **F48:** Tablo C.12 4. düzeyde s_R = 1,915 °C verir. Formül (29)–(31) ise 1,9175 °C verir; bu, REML Tablo
  C.13'ün değeridir.
- **F49:** Tablo 4'te basılı eğim d = 0,772, aynı tabloda basılı uydurulmuş değerlere uymaz. Bu değerler
  d ≈ 0,769'dan çıkar.
