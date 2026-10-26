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

## 6.3 Raporun içeriği

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
