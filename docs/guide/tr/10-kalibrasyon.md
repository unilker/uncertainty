# 10. Her iki değişkende belirsizlikli kalibrasyon (ISO 6143)

Bir `calibration` projesi, içerikleri x **ve** yanıtları y belirsiz olan referans malzemelerden bir analiz
fonksiyonu x = G(y) belirler. Bu fonksiyonla aynı cihazda ölçülen numunelere belirsizlikleri ve
kovaryanslarıyla içerik atanır. Program ISO 6143:2025'in genelleştirilmiş en küçük kareler yöntemini uygular
(standart gaz analizi içindir, yöntem geneldir) ve standardın 1–3 numaralı çözümlü örnekleriyle
doğrulanmıştır (`iso6143-*` kütüphane projeleri). Yalnızca yanıtlar belirsizse içeriklere çok küçük
belirsizlikler verin: eğri bu durumda xᵢ'lerden geçmeye zorlanır ve uydurma y'de ağırlıklı en küçük
karelere dönüşür.

## 10.1 Analiz fonksiyonları (6.2.1)

| `function` | G(y; b) | parametre |
|---|---|---:|
| `linear` | b₀ + b₁y | 2 |
| `quadratic` | b₀ + b₁y + b₂y² | 3 |
| `cubic` | b₀ + b₁y + b₂y² + b₃y³ | 4 |
| `power` | b₀ + b₁y^(1 + b₂) (y > 0) | 3 |
| `exponential` | b₀ + b₁e^(b₂y) | 3 |

Analiz fonksiyonu içeriği yanıttan verir; kalibrasyon fonksiyonunun tersidir.

## 10.2 Uydurma (Ek A)

b parametreleri ve düzeltilmiş yanıtlar ŷᵢ şu toplamı en küçükler:

S = Σᵢ [(G(ŷᵢ; b) − xᵢ)²/u²(xᵢ) + (ŷᵢ − yᵢ)²/u²(yᵢ)].

Böylece düzeltilmiş noktalar (x̂ᵢ, ŷᵢ) = (G(ŷᵢ), ŷᵢ) eğrinin üzerindedir. En küçükleme, ağırlıklı en küçük
kareler başlangıcından bir Levenberg–Marquardt iterasyonuyla yapılır. b'nin kovaryans matrisi tüm
kalibrasyon verilerinin belirsizliklerini yayar. Buna geçmişi ortak olan içeriklerin kovaryansları
u(xᵢ, xₕ) da dahildir (bir seyreltme serisi, ortak bir birincil standart seti; Formül A.7). ISO 6143 A.2,
∂b/∂xᵢ ve ∂b/∂yᵢ türevlerini uydurmanın sayısal türevlenmesiyle elde eder. Program bu türevleri en küçük
noktasında kapalı fonksiyon teoremiyle tam olarak hesaplar. Böylece standardın B.1'de uyardığı adım
büyüklüğü ve durdurma kuralı bağımlılığı ortadan kalkar (araştırma günlüğü D33).

## 10.3 Fonksiyonun doğrulanması (6.2.2, 6.2.3)

- Her nokta için **ağırlıklı sapmalar** (xᵢ − x̂ᵢ)/u(xᵢ) ve (yᵢ − ŷᵢ)/u(yᵢ) hesaplanır; **uyum iyiliği** Γ
  bunların mutlak değerce en büyüğüdür. Γ ≤ 2 ise fonksiyon kabul edilebilir: eğri her kalibrasyon
  dikdörtgeninden (xᵢ ± 2u(xᵢ), yᵢ ± 2u(yᵢ)) geçer (Formül 9).
- Artık toplam S_res, kabul sınırı 2(n − N − 1) ile birlikte verilir; N + 1 parametre sayısıdır.
- `compare` ile (varsayılan) parametre sayısı nokta sayısından az olan diğer analiz fonksiyonları da
  uydurulur. Γ ve S_res değerleri listelenir; böylece 6.2.2'deki seçim yapılabilir: önce fiziksel model;
  yoksa benzer uyum verenlerin en basiti; o da yoksa en iyi uyum. Eğrinin gözle incelenmesi yine gereklidir.
- **Belirsizliğin üst sınırı** (6.2.3): en düşük ve en yüksek standartların yanıtlarında u(x). `target_uncertainty`
  verilmişse ikisinin büyüğüyle karşılaştırılır.

## 10.4 Numunelerin içerikleri (6.3)

Yanıtı y ve belirsizliği u(y) olan her numune için:

u²(x) = (∂G/∂y)² u²(y) + Σⱼ Σₗ (∂G/∂bⱼ)(∂G/∂bₗ) u(bⱼ, bₗ).

Genişletilmiş belirsizlik `coverage_factor` ile hesaplanır (varsayılan 2). Aynı kalibrasyondan bulunan
sonuçlar b üzerinden koreledir. Rapor bunların korelasyon katsayılarını verir; karışımlar sonradan birlikte
bir standart seti olarak kullanılacaksa bu katsayılar gereklidir. Yanıtı kalibrasyon aralığının dışında
kalan numuneler işaretlenir.

## 10.5 Kayma kontrolü ve uyumsuz bileşimli karışımlar (6.2.4, 6.2.5)

`drift`, kalibrasyon aralığında numuneyi kuşatan bir referans karışımın numuneden önceki ve sonraki n
ölçümünün ortalamalarını kalibrasyondaki yanıtı y(kalib) ile ve birbirleriyle karşılaştırır. Kritik
değerler, y(kalib)'den olan iki fark için 2u√(1 + n_kalib/n), aralarındaki fark için 2u√(2n_kalib/n)'dir;
u = u(y(kalib)), varsayılan değerler n = n_kalib = 10'dur. Bunlar n = 10 için 6.2.4'ün Notunda belirtilen
2√2 u'yu verir. Standardın Formül (12)'si 2u√(1 + 20/n) yazar; bu n = 10 için 2√3 u verir ve o Notla
çelişir. Program türetilmiş ifadeyi kullanır (araştırma günlüğü F41).

`validation`, matrisi standartlarınkinden farklı olan referans karışımları listeler. Her biri
|x_göz − x_ref| ≤ 2√(u²(x_göz) + u²(x_ref)) ise geçer (Formül 13).

## 10.6 Üstel fonksiyon üzerine bir not

ISO 6143 B.1, başvuru yazılımı B_LEAST'in üstel fonksiyon için yanlış parametre belirsizlikleri verdiğini
belirtir. Örnek 3'te basılan değerler yaklaşık 39 kat küçüktür. Bu program, ODRPACK ve tüm kalibrasyonun
Monte Carlo benzetimi birbiriyle uyumludur. Tablo B.12'de üstel fonksiyonla atanan içeriğin basılı u(x)
değeri %6–7 küçüktür (araştırma günlüğü F42). Güçlü biçimde doğrusal olmayan fonksiyonlarda birinci
mertebe yayılımın kendisi de bir yaklaşıktır (B.2.4, son not). `iso6143-b3-no-ndir-exponential` kütüphane
projesi Monte Carlo kontrolünü belgeler.

## 10.7 Uygulamada

*Yeni kalibrasyon (iki eksen)* bir şablonla başlar. *Veriler* sekmesine kalibrasyon noktaları her satırda
bir tane olacak şekilde yapıştırılır (ad, x, u(x), y, u(y)). Ardından analiz fonksiyonu, numuneler
(ad, y, u(y)) ve isteğe bağlı kovaryanslar (nokta, nokta, değer) girilir. *Sonuçlar* sekmesi parametreleri,
Γ ve S_res'i, fonksiyonların karşılaştırmasını, numunelerin içeriklerini, üst sınırı ve ağırlıklı
sapmaların ±2 sınırlarıyla grafiğini gösterir.
