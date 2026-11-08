# 6. Raporlama

## 6.1 Yuvarlama

GUM (7.2.6), genişletilmiş belirsizliğin en fazla iki anlamlı basamakla, sonucun da aynı ondalık basamağa
kadar verilmesini önerir. Proje dosyasının `"reporting"` bölümü bunu denetler:

| Alan | Anlamı |
|---|---|
| `"significant_digits"` | U'nun anlamlı basamak sayısı (varsayılan 2) |
| `"decimal_places"` | bunun yerine U'yu sabit sayıda ondalık basamağa yuvarlar (örneğin tam yüzde için 0) |
| `"rounding"` | `ea4_02` (varsayılan): U olağan biçimde yuvarlanır, ancak olağan yuvarlama U'yu %5'ten fazla düşürecekse yukarı yuvarlanır (EA-4/02 M:2022, 6.3); `half_even`: atılan 5 çifte yuvarlanır, sonraki basamaklar dikkate alınmaz (MUPR §12; U'yu yaklaşık %5,7'ye kadar düşürebilir, ör. 0,10599 → 0,10); `half_up`; `up`: U her zaman yukarı yuvarlanır (temkinli, GUM 7.2.6) |

Ölçülen değer, yuvarlanmış U'nun ondalık basamağına yuvarlanır. Sonuç ifadesi kullanılan kuralı adıyla yazar ve
kapsam olasılığını EA-4/02 ile ILAC P14'teki biçimde "yaklaşık %95" olarak verir (tam yüzde; %99,5'in üstünde %0,1
adımla). k = 2 ve ν_eff = ∞ (%95,45) için de bu biçim kullanılır. Varsayılan kural, 8 Kasım 2026 tarihli bağımsız ön
incelemeden sonra `half_even`'dan `ea4_02`'ye değiştirilmiştir; Eurachem/CITAC MUPR kuralını izleyen projeler
`"half_even"` yazar. Tam hassasiyetli değerler bütçede ve JSON
sonuçlarında kalır; ara değerleri hesabın sonundan önce asla yuvarlamayınız — yayımlanmış birçok örnek bu
nedenle son basamakta değişir.

## 6.2 Sonucun ifadesi

Program sonucu EA-4/02'nin (Madde 6) önerdiği biçimde yazar:

> c_Cd = (1002,7 ± 1,7) mg/L; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam
> faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

Monte Carlo yöntemi kullanıldığında sonuç kapsam aralığının kendisidir; kapsam olasılığı ve aralığın türüyle
(en kısa veya olasılıksal simetrik) birlikte verilir.

**Kalibrasyon sertifikaları (ILAC P14:09/2020).**

- Sonuç, yukarıdaki gibi k ve kapsam olasılığıyla y ± U olarak verilir (5.2).
- U en çok iki anlamlı basamak taşır (5.3). `reporting.significant_digits` daha büyükse program sonuç ifadesinde
  uyarır.
- Akredite bir laboratuvar CMC'sinden küçük bir belirsizlik raporlayamaz (5.5). `"reporting": { "cmc": {
  "expanded": … } }` (veya `"relative_percent"`) ile program max(U, CMC) değerini raporlar ve bunu ifadede belirtir.
  Kütüphanedeki `ilac-p14-cmc-floor-weight-10kg` projesi bunu gösterir.

## 6.3 Bilinen bir sapma için düzeltilmemiş sonuçlar

GUM, sonuçların tanınmış bütün önemli sistematik etkiler için düzeltildiğini varsayar (GUM 3.2.4). Rutin
deneylerde ise bir sapma çoğu zaman bilinir veya beklenir ama düzeltilmez; örneğin yöntem düzeltme
öngörmez veya sapma matristen matrise değişir. Düzeltilmemiş sonucu olağan belirsizliğiyle raporlamak bu
durumda yanıltıcı olur. `reporting.uncorrected_bias` ile (sapma `bias` b, standart belirsizliği `u_bias` ve
bir `strategy`) model raporları, aralığı sapmayı da içeren ikinci bir ifade ekler. Yaklaşımlar Magnusson ve
Ellison'un (2008, Tablo 3) yaklaşımlarıdır. Formüllerde Δ = −b ve u_c = √(u²(y) + u²(b)):

| `strategy` | Aralık | Not |
|---|---|---|
| `ue` (varsayılan) | y ± U; U, N(y + Δ, u_c²) dağılımının 2Φ(k) − 1 kadarını içeren yarı genişlik | Synek; doğru kapsamlı en dar simetrik aralık; \|Δ\| > 0,8u_c için ≈ 1,65u_c + \|Δ\| |
| `rssu` | y ± k√(u²(y) + Δ² + u²(b)) | Nordtest, NIST; büyük sapmada temkinli |
| `rssu_k` | y ± k√(u²(y) + (Δ/k)² + u²(b)) | IUPAC; büyük sapmada kapsam düşer |
| `sumu` | [y − max(0, ku_c − Δ), y + max(0, ku_c + Δ)] | Phillips ve ark. (NIST, 1997); asimetrik |
| `sumu_max` | y ± (ku_c + \|Δ\|) | GUM F.2.4.5; simetrik, geniş |

Örnek: kadmiyum standardına `"reporting": { "uncorrected_bias": { "bias": 1.5, "u_bias": 0.3, "strategy": "sumu" } }`
eklenirse:

> c_Cd = 1002,7 mg/L (−3,3, +0,3); tahmin edilen sapma b = 1,50 mg/L (u(b) = 0,30 mg/L) için düzeltme yapılmamıştır …

Test takımı Magnusson ve Ellison'un kapsam benzetimini (Şekil 1) yeniden üretir. SUMU sıfır sapmada yaklaşık
%95, 3u_c'de %97,5 kapsar. Ue sıfır sapmada yaklaşık %98, daha büyük sapmalarda %95 kapsar. RSSu ve SUMU_MAX
her zaman %95'ten fazla kapsar. Şekil 2'deki aralık genişlikleri de yeniden üretilir. Gerekçelendirilebiliyorsa
belirsizliğiyle birlikte bir düzeltme yine tercih edilen yoldur (GUM 6.3.1; araştırma günlüğü F43, Tablo 3'teki
bir işaret hatasını kaydeder).

## 6.3b Fiziksel bir sınıra yakın sonuçlar

Bir kütle kesri 1 g/g'yi aşamaz, bir derişim negatif olamaz. y ± U böyle bir sınırı aşıyorsa modelin `"output"`
bölümünde `"lower_bound"` ve/veya `"upper_bound"` ile belirtilir. Program bu durumda

- GUM aralığını sınırda keser ve sonucu asimetrik olarak yazar, örneğin w = (0,9998 +0,0002/−0,0015) g/g: simetrik
  w ± U95 % aralığı, w − U95 % ≤ w ≤ 1 g/g asimetrik aralığı belirtiyor kabul edilebilir (Westwood ve ark. 2023,
  SI izlenebilir saflık üzerine IUPAC teknik raporu, 4.1.1; BIPM ISRD raporları böyle yapar);
- Monte Carlo yöntemi çalışıyorsa sınırlarla kısıtlanmış sonucu da verir: dışarıda kalan çekilişler atılır; bu,
  ölçülen büyüklük için sınırlar üzerinde dikdörtgen bir önsele karşılık gelir (Toman ve ark. 2016; Westwood ve ark.
  2023, 4.2.5). İçerideki çekiliş oranı, ortalama, standart sapma ve kapsam aralığı verilir; tahmin sınırdan
  uzaklaşır ve aralık genellikle kesilmiş GUM aralığından dardır.

Tahminin kendisi sınırların dışındaysa sonuç ifadesi bunu belirtir; bu durumda kısıtlanmış Monte Carlo sonucu
raporlanmalıdır. Kütüphane projesi: `bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od` (basılı gibi 999,8 +0,2/−1,5 mg/g).

## 6.4 Raporun içeriği

Rapor, değerlendirmenin yeniden üretilebilir ve gözden geçirilebilir olmasını amaçlar (ISO/IEC 17025:2017,
7.6 ve 7.8; GUM 7.2):

* ölçülen büyüklük ve model; top-down değerlendirmelerde kullanılan veriler ve formüller;
* her girdinin değeri, dağılımı ve bilginin kaynağı;
* duyarlılık katsayıları, katkılar ve varyans paylarıyla belirsizlik bütçesi;
* kapsam faktörü ve nasıl elde edildiği (serbestlik dereceleri, kapsam olasılığı);
* Monte Carlo yönteminde deneme sayısı, tohum ve GUM sonucunun doğrulanması;
* açıklamalar, kaynaklara ilişkin notlar ve kaynakça;
* yazılım sürümü ve atıf.

Raporlar Markdown, bağımsız HTML (SVG grafikler; açık ve koyu tema; yazdırılabilir) ve JSON (ileri işleme için
tüm değerler tam hassasiyetle) olarak alınabilir. Türkçe raporlar ondalık virgül ve öne yazılan yüzde işareti
(%95) kullanır.
