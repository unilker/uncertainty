# 14. İç kalite kontrolü için kontrol grafikleri (Nordtest TR 569)

ISO/IEC 17025:2017 7.7.1, laboratuvarın sonuçlarının geçerliliğini izlemesini ister. Eğilimler
saptanabilmeli, uygulanabildiğinde istatistiksel teknikler kullanılmalıdır. Bunun olağan aracı kontrol
grafiğidir: her seride bir kontrol numunesi analiz edilir ve kontrol değeri sınırlara karşı çizilir.
`control_chart` bölümü Nordtest *İç Kalite Kontrolü El Kitabı*'nı, TR 569 6. baskı (2026), yani "Trollbook"u
izler. Bölüm, el kitabının Tablo 1'i, Bölüm 7'deki durumlar ve Örnek 1–12 ile doğrulanmıştır (`tr569-*`
kütüphane projeleri).

Kontrol grafiği verileri, top-down değerlendirmenin laboratuvar içi tekrar üretilebilirlik bileşeni u(R_w)'yi
de verir (5. bölüm). Bu durumda veriler bir `topdown` projesine girilir.

## 14.1 Grafik türleri (Bölüm 5)

- **X-grafiği** (`"chart": "x"`, varsayılan): tekil kontrol değerleri veya her serideki tekrarların
  ortalaması, bir merkez çizgisi çevresinde. Özel durumları kör grafiği (negatif değerler de çizilir) ve geri
  kazanım grafiğidir.
- **R-grafiği** (`"range"`): bir numunenin n tekrarının açıklığı (en büyük − en küçük sonuç). Tekrarlanabilirliği
  izler ve yalnızca üst sınırları vardır.
- **r%-grafiği** (`"relative_range"`): açıklığın tekrarların ortalamasına yüzde oranı. Göreli standart
  sapmanın yaklaşık sabit olduğu, raporlama sınırının çok üzerindeki derişimlerde kullanılır.

Kontrol değerleri (`values`) veya her serinin tekrar sonuçları (`runs`) verilir. `runs` verildiğinde kontrol
değerini program oluşturur: ortalama, açıklık veya göreli açıklık.

## 14.2 Merkez çizgisi ve kontrol sınırları (Bölüm 7)

**Merkez çizgisi** kontrol değerlerinin ortalamasıdır. Bunun yerine daha önce belirlenmiş bir ortalama
(`"central_line": {"mean": …}`) veya bir referans malzemenin referans ya da nominal değeri
(`{"reference": …}`) olabilir. Merkez çizgisi ortalama olduğunda ve bir referans değer biliniyorsa
(`reference`), rapor ortalamayı bu değere karşı test eder: t = |x̄ − T|·√n/s (Eşitlik 6).

| Dayanak | X-grafiği | R- ve r%-grafiği (n tekrar) |
|---|---|---|
| istatistiksel | kontrol değerlerinin s'si; WL = CL ± 2s, AL = CL ± 3s | s = ortalama açıklık/d₂; WL = D_WL·s, AL = D_AL·s |
| hedef | gereklilikten s_hedef; aynı katsayılar | s_hedef veya r tekrarlanabilirlik sınırından s_r = r/2,8 |
| tolerans | AL = CL ± tolerans, uyarı sınırı yok | AL = tolerans |

Açıklık katsayıları ISO 8258'den alınır. d₂ ve d₃, n standart normal değerin açıklığının ortalaması ve
standart sapmasıdır; üst eylem sınırı D_AL = d₂ + 3d₃'tür. TR 569 buna D_WL = d₂ + (2/3)(D_AL − d₂) uyarı
sınırını ekler (Tablo 4):

| n | d₂ | D_WL | D_AL |
|---|---|---|---|
| 2 | 1,128 | 2,833 | 3,686 |
| 3 | 1,693 | 3,470 | 4,358 |
| 4 | 2,059 | 3,818 | 4,698 |

Program d₂ ve d₃ değerlerini 2–10 tekrar için hesaplar. Test takımı bunları sayısal integrasyonla yeniden
hesaplar.

Sonuçlar için gereklilik biliniyor ve yöntem bundan daha iyi çalışıyorsa hedef sınırlar seçilir. TR 569 bunları
çok elementli yöntemler için önerir: 20 analit için istatistiksel sınırlar her seride yaklaşık %10 yanlış alarm
verir. İki kısa yol vardır:

- gerekli göreli genişletilmiş belirsizlik U'dan (k = 2): s_hedef = U/4, yani u_c'nin yarısı
  (`expanded_percent`, Örnek 1);
- standart bir yöntemin r tekrarlanabilirlik sınırından: s_r = r/2,8 (`repeatability_limit`).

**Yalnızca eylem sınırları** olan bir grafikte (`"action_only": true`) varsayılan katsayı X-grafikleri için 3,
açıklık grafikleri için D_AL'dir. TR 569 bunun yerine 2,5 s ve 3,5 s önerir (`action_factor`); bunlar her iki
sınırı olan grafiklerinkine yakın yanlış alarm oranları verir. Üçte iki kuralı uygulanamadığından böyle bir
grafik ortalamadaki kaymaya daha az duyarlıdır.

`establish` ile merkez çizgisini ve istatistiksel sınırları yalnızca ilk N kontrol değeri belirler (yaklaşık
25 değerden ön sınırlar); sonraki değerler bunlara göre değerlendirilir. Belirlenebilir bir nedeni olan
seriler `exclude` ile istatistiklerin dışında bırakılabilir. TR 569, yaklaşık bir yıllık en az 60 değere
dayanan sabit sınırlar önerir.

## 14.3 Günlük değerlendirme (Bölüm 9)

Her seriden sonra:

| Durum | Değerlendirme |
|---|---|
| uyarı sınırları içinde | kontrol altında |
| uyarı ile eylem sınırları arasında ve önceki iki değerin hiçbiri aynı tarafta uyarı sınırı dışında değil | kontrol altında (uyarı bölgesi) |
| eylem sınırları dışında | kontrol dışı |
| uyarı ile eylem sınırları arasında ve önceki iki değerden biri aynı tarafta uyarı sınırı dışında ("üçte iki") | kontrol dışı |

İstatistiksel sınırlar ve ortalama merkez çizgisiyle bu iki kural bir X-grafiğinde yaklaşık %0,54 yanlış
kontrol dışı durum verir. Açıklık grafiğinde bu oran yaklaşık %1,3'tür. Sınırın üzerindeki bir değer sınırın
içinde sayılır. Üçte iki kuralında eylem sınırının ötesindeki önceki bir değer, ISO 7870-2'deki gibi uyarı
sınırının dışında sayılır (araştırma günlüğü D43).

Rapor, yöntemin **istatistiksel kontrol dışı** olduğu değerleri de listeler: art arda sürekli artan veya
azalan yedi değer ya da art arda on bir değerden onunun merkez çizgisinin aynı tarafında olması. Sonuçlar
yine raporlanabilir, ancak neden uzun dönem değerlendirmesinde araştırılmalıdır. Referans merkez çizgisinde
küçük bir sapma tek başına birçok değeri bir tarafa toplayabilir.

## 14.4 Uzun dönem değerlendirmesi (Bölüm 10)

En az yılda bir kez dönemin değerleri, sınırların dayandığı değerlerle karşılaştırılır (`review`):

- **standart sapma**: s₁ > s₂ olmak üzere F = s₁²/s₂² (Eşitlik 5), iki taraflı F₁₋α/₂(ν₁, ν₂) kritik değerine
  karşı. Program bu değeri tam olarak hesaplar; TR 569 Tablo 3 serbestlik derecelerini yuvarlar. Yaklaşık
  60 değer için hızlı bir kontrol, uyarı sınırları dışındaki değerlerin sayısıdır: 6'dan fazlası s'nin
  arttığını düşündürür.
- **ortalama**: Eşitlik 9'un birleştirilmiş s_p değeri ve ν = n₁ + n₂ − 2 ile
  t = |x̄₁ − x̄₂|/s_p·√(n₁n₂/(n₁ + n₂)) (Eşitlik 7). Yaklaşık 60 değer için 0,37 s'den küçük bir kayma eylem
  gerektirmez.

Varsayılan olarak önceki seri kuruluş dönemi, güncel seri sonraki değerlerdir. Merkez çizgisinden 4 s'den uzak
değerler güncel seriden çıkarılır (`exclude_beyond`). Serilerden biri, Örnek 8'deki gibi özet istatistiklerle
de verilebilir (`previous`, `current`: `mean`, `sd`, `n`, `dof`). TR 569 sınırları yalnızca anlamlı bir
değişiklikten sonra değiştirir; ortalama merkez çizgisini ise yalnızca kaymanın bir açıklaması varsa kaydırır.

## 14.5 Tekrarlı kontrol sonuçlarından kesinlik (Örnek 12)

Her seride kontrol numunesinin tekrarları varsa tüm sonuçların birlikte standart sapması s_Rw'yi olduğundan
küçük verir. Buna dayanan kontrol sınırları çok dar olur. Bu durumda rapor şunları verir:

- serilerden birleştirilmiş s_r (Eşitlik 9 ve 10);
- tek yönlü varyans analizi: s_r, seriler arası s_I ve s_Rw = √(s_r² + s_I²);
- tekrar sayıları eşitse Örnek 12'nin basitleştirilmiş s_Rw'si: serilerdeki birinci, ikinci, … sonuçların
  standart sapmaları, birleştirilmiş.

Örnek 12'de bunlar s_r = 0,15, s_Rw = 0,270 (basitleştirilmiş) ve 0,272'dir (varyans analizi).

## 14.6 Dikkat edilecek noktalar

- TR 569 ara değerleri yuvarlar: Örnek 6 yuvarlanmış %1,67 CV değerini katsayılarla çarpar. Kütüphane
  projeleri yayımlanmış değerleri verir ve farkı notlarında kaydeder.
- "Birleştirilmiş tekrarlanabilirlik standart sapması" denen örneklerde (Örnek 3 ve 8) s, ortalama açıklığın
  1,128'e bölümüdür (Eşitlik 4). `"sd_from": "pooled"` bunun yerine tekrarların birleştirilmiş seri içi standart
  sapmasını kullanır (Eşitlik 9).
- El kitabındaki bulgular araştırma günlüğüne kaydedilmiştir (F52, F53):
  - açıklık grafiklerinin 1. durumundaki (Bölüm 7) uyarı sınırı 3·0,356 olarak basılmıştır; katsayı 2,83'tür;
  - Örnek 12'nin 24. dipnotu s_r, s_I ve s_Rw değerlerini on kat küçük verir.
