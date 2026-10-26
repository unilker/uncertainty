# 9. Yeterlilik testi (ISO 13528)

Bir `proficiency` projesi, bir ölçülen büyüklük için bir yeterlilik testi programının bir turunu içerir:
katılımcıların sonuçları (bildirilmişse belirsizlikleriyle), atanan değerin ve yeterlilik değerlendirmesi için
standart sapmanın nasıl elde edildiği ve isteğe bağlı olarak yeterlilik testi numunelerinin homojenlik ve
kararlılık kontrolleri. Program ISO 13528:2022'nin istatistiksel yöntemlerini uygular ve standardın E.2, E.3 ve
E.4 çözümlü örnekleriyle doğrulanmıştır (`iso13528-*` kütüphane projeleri).

## 9.1 Özet istatistikler (Ek C)

p sayısal sonuç için ("< 0,015" gibi sansürlü sonuçlar listelenir ancak kullanılmaz) program medyanı,
ölçeklenmiş medyan mutlak sapmayı MADe = 1,483 med|xᵢ − med(x)|, normalleştirilmiş çeyrekler açıklığını
nIQR = 0,7413 (Q₃ − Q₁), aritmetik ortalamayı ve standart sapmayı ve **Algoritma A**'nın (C.3.1) robust
ortalaması x* ile robust standart sapması s*'ı verir.

Algoritma A, x* = medyan ve s* = MADe ile başlar; x* ± 1,5s* dışındaki her sonuç yakın sınırla değiştirilir ve
x* ile s*, değiştirilmiş değerlerin ortalaması ve standart sapmalarının 1,134 katı olarak yeniden hesaplanır.
İterasyon, x* ve s*'ın üçüncü anlamlı basamakları değişmeyince durur (C.3.1); `"algorithm_a": "converged"`
ise tam yakınsamaya kadar sürdürür. MADe sıfırsa başlangıçtaki s* örneklem standart sapmasıdır (C.3.1, Not 2).

Çeyrekler sıra istatistikleri arasında doğrusal enterpolasyonla hesaplanır; bu, ISO 13528 Tablo E.5'teki
nIQR'yi yeniden üretir. Başka yazılımlar başka tanımlar kullanabilir (Tablo E.5, not). Standart sapmaların
robust birleştirilmesi olan Algoritma S (C.4) motorda mevcuttur (`pt::algorithm_s`).

## 9.2 Atanan değer ve belirsizliği (Madde 7)

| `assigned_value.method` | x_pt | u(x_pt) |
|---|---|---|
| `algorithm_a` | x* | 1,25 s*/√p, Formül (6) |
| `median` | medyan | 1,25 nIQR/√p (veya MADe) |
| `mean` | aritmetik ortalama | s/√p |
| `reference` | formülasyon, CRM veya referans laboratuvardan değer (7.3–7.6) | verildiği gibi |

Homojensizlik, taşıma ve kararsızlık için belirsizlik bileşenleri eklenebilir (Formül 3). Uzlaşma değerinde
bunlar genellikle sonuçların dağılımına zaten yansımıştır (7.2.2, Not 2). 1,25 çarpanı muhafazakârdır;
`u_factor` ile değiştirilebilir.

Program u(x_pt) < 0,3 σ_pt olup olmadığını belirtir (Formül 10). Değilse atanan değerin belirsizliği ihmal
edilemez ve z′, ζ veya E_n skorları kullanılmalıdır (9.2.2).

## 9.3 Yeterlilik değerlendirmesi için standart sapma (Madde 8)

σ_pt ya verilir (amaca uygunluk, önceki turların deneyimi veya Horwitz eşitliği gibi genel bir model) ya da
turun sonuçlarından alınır (`robust`, `made`, `niqr`, `sd`). σ_pt yoksa z ve z′ skorları hesaplanmaz.

## 9.4 Performans istatistikleri (Madde 9)

| Skor | Formül | Yorum |
|---|---|---|
| D, D % | x − x_pt, 100 (x − x_pt)/x_pt | δ_E ile karşılaştırılır |
| PA | 100 D/δ_E (varsayılan δ_E = 3σ_pt) | ≥ %100 veya ≤ −%100: eylem |
| z | (x − x_pt)/σ_pt | \|z\| ≤ 2 kabul edilebilir, 2 < \|z\| < 3 uyarı, \|z\| ≥ 3 eylem |
| z′ | (x − x_pt)/√(σ_pt² + u²(x_pt)) | z gibi |
| ζ | (x − x_pt)/√(u²(x) + u²(x_pt)) | z gibi; katılımcının u(x) değerini gerektirir |
| E_n | (x − x_pt)/√(U²(x) + U²(x_pt)) | \|E_n\| < 1 yeterli |

Katılımcının standart belirsizliği, genişletilmiş belirsizliğinin kapsam faktörüne bölünmesiyle bulunur.
`uncertainty_screening` ile bildirilen belirsizlikler a (makul), b (u_min'in altında) veya c (u_max'ın
üstünde) olarak işaretlenir; ISO 13528 9.8, u_min = u(x_pt) ve u_max = 1,5 s* önerir.

ζ ve E_n skorları ile uzlaşma değerli z′ skoru, bir sonuç ile ondan hesaplanan atanan değer arasındaki
korelasyonu göz ardı eder; ISO 13528 (9.5.1, 9.6.1) bu yaklaşımı kabul eder.

## 9.5 Homojenlik ve kararlılık (Ek B)

Her biri m kez ölçülen g numune için B.3'ün varyans analizi, numune ortalamalarının standart sapması s_x'i,
numune içi standart sapma s_w'yi ve numuneler arası standart sapma s_s = √(s_x² − s_w²/m)'yi (negatifse sıfır)
verir. s_s ≤ 0,3 σ_pt ise numuneler yeterince homojendir (B.1). B.2.3'ün genişletilmiş ölçütü
√(F₁(0,3σ_pt)² + F₂s_w²) de verilir; F₁ = χ²₀,₉₅(g − 1)/(g − 1) ve F₂ = (F₀,₉₅(g − 1, g(m − 1)) − 1)/m tam olarak
hesaplanır (m = 2 için Tablo B.1).

Kararlılık kontrolü öncesi ve sonrası ortalamaları karşılaştırır: |ȳ₁ − ȳ₂| ≤ 0,3 σ_pt (B.17); ortalamaların
belirsizlikleri verilmişse ölçüt 2√(u²(ȳ₁) + u²(ȳ₂)) kadar genişletilir (B.18).

## 9.6 Uygulamada

*Yeni yeterlilik testi turu* bir şablonla başlar. *Veriler* sekmesinde sonuçlar bir elektronik tablodan
yapıştırılır, her satırda bir laboratuvar: kod, sonuç, genişletilmiş belirsizlik, kapsam faktörü ve isteğe
bağlı olarak yöntem (sekme, noktalı virgül veya boşlukla ayrılmış; ondalık virgül kabul edilir; sansürlü
sonuç için "<0,015"). *Sonuçlar* sekmesi özet istatistikleri, atanan değeri, sinyalleriyle skor tablosunu ve z
skorlarının grafiğini gösterir; HTML raporu homojenlik grafiğini de ekler.
