# 6. Raporlama

## 6.1 Yuvarlama

GUM (7.2.6), genişletilmiş belirsizliğin en fazla iki anlamlı basamakla, sonucun da aynı ondalık basamağa
kadar verilmesini önerir. Proje dosyasının `"reporting"` bölümü bunu denetler:

| Alan | Anlamı |
|---|---|
| `"significant_digits"` | U'nun anlamlı basamak sayısı (varsayılan 2) |
| `"decimal_places"` | bunun yerine U'yu sabit sayıda ondalık basamağa yuvarlar (örneğin tam yüzde için 0) |
| `"rounding"` | `half_even` (varsayılan): atılan 5 çifte yuvarlanır, sonraki basamaklar dikkate alınmaz (MUPR §12); `half_up`; `up`: U her zaman yukarı yuvarlanır (temkinli, GUM 7.2.6) |

Ölçülen değer, yuvarlanmış U'nun ondalık basamağına yuvarlanır. Tam hassasiyetli değerler bütçede ve JSON
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
