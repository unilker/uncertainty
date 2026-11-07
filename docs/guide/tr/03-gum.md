# 3. GUM belirsizlik yaklaşımı

Bu bölüm, ölçüm belirsizliğinin JCGM 100:2008'e (GUM) göre değerlendirilmesini programda uygulandığı biçimiyle
özetler. Aksi belirtilmedikçe madde numaraları GUM'a aittir.

## 3.1 Ölçülen büyüklük ve ölçüm modeli

İlk adım **ölçülen büyüklüğü** — ölçülmesi amaçlanan büyüklüğü — tanımlamak ve onu bir **ölçüm modeliyle**
ifade etmektir:

$$Y = f(X_1, X_2, \dots, X_N).$$

Xᵢ girdi büyüklükleri sonucu etkileyen her şeyi içerir: okumalar, kalibrasyon değerleri, sistematik etkiler
için düzeltmeler, referans değerler, fiziksel sabitler (GUM 4.1). En iyi tahmini sıfır olan bir düzeltme de
girdi büyüklüğüdür, çünkü belirsizliği Y'nin belirsizliğine katkıda bulunur.

Proje dosyasında model formül olarak yazılır:

```json
"output": { "name": "c_Cd", "unit": "mg/L", "expression": "1000 * m * P / V" }
```

İfadelerde `+ - * /`, üs alma (`^` veya `**` ya da `pow(x, y)`), parantezler, `pi` sabiti ve `sqrt`, `exp`,
`ln`, `log10`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `sinh`, `cosh`, `tanh`, `abs` fonksiyonları
ve dört bölgeli ters tanjant `atan2(y, x)` kullanılabilir. Adlar her alfabeden harf, rakam ve `_` içerebilir (örneğin `ρ_a`, `dV_cal`). Uzun modeller
**ara büyüklüklerle** daha okunaklı olur; bunlar modele sembolik olarak açılır (hiçbir yaklaşıklık yoktur):

```json
"intermediates": [ { "name": "V", "expression": "V_nom + dV_cal + dV_rep + dV_temp" } ]
```

## 3.2 Girdilerin standart belirsizlikleri

Her girdi büyüklüğü, hakkındaki bilgiyi ifade eden bir olasılık dağılımıyla tanımlanır; tahmini xᵢ bu
dağılımın beklenen değeri, **standart belirsizliği** u(xᵢ) ise standart sapmasıdır (GUM 4.1.6, 3.3.5).

### A tipi değerlendirme

Aynı koşullarda bir büyüklüğün n bağımsız gözlemi q₁ … qₙ için (GUM 4.2):

$$\bar q = \frac{1}{n}\sum q_k,\qquad s(q) = \sqrt{\frac{\sum (q_k-\bar q)^2}{n-1}},\qquad u(\bar q) = \frac{s(q)}{\sqrt n},\qquad \nu = n-1.$$

Standart sapma daha önceki, daha büyük bir çalışmadan biliniyorsa (ν_p serbestlik dereceli birleştirilmiş
standart sapma s_p), mevcut n gözlemin ortalamasının belirsizliği ν_p serbestlik derecesiyle s_p/√n'dir
(GUM 4.2.4). Programda: `"type": "type_a", "observations": [...]` veya `"type": "type_a_pooled", "mean": ...,
"pooled_sd": ..., "pooled_dof": ..., "n": ...`.

### B tipi değerlendirme

Diğer bilgiler — sertifikalar, şartnameler, el kitapları, deneyim — varsayılan bir dağılım aracılığıyla
standart belirsizliğe dönüştürülür (GUM 4.3). a sınırların yarı genişliği olmak üzere:

| Bilgi | Dağılım | Standart belirsizlik | `type` |
|---|---|---|---|
| k kapsam faktörüyle genişletilmiş belirsizlik U (sertifika) | normal | U/k | `expanded` ve `k` ile `normal` |
| standart belirsizlik verilmiş | normal | u | `sd` ile `normal` |
| ±a sınırları, her değer eşit olasılıklı | dikdörtgen | a/√3 | `rectangular` |
| ±a sınırları, merkeze yakın değerler daha olası | üçgen | a/√6 | `triangular` |
| ±a sınırları, yarı genişliği βa olan düz tepe | yamuk | a√((1 + β²)/6) | `trapezoidal` (`beta`) |
| ±a sınırları yalnızca ±d ile biliniyor | eğrisel yamuk (JCGM 101 6.4.3) | √(a²/3 + d²/9) | `curvilinear_trapezoid` (`d`) |
| ±a arasında sinüzoidal değişim (örn. periyodik sıcaklık, RF uyumsuzluğu) | ark sinüs (U biçimli) | a/√2 | `arcsine` |
| t dağılımlı bilgi (ölçek s, ν serbestlik derecesi) | ölçekli ve kaydırılmış t | s (GUM yaklaşımında) | `t` |
| yalnızca beklenen değeri bilinen negatif olmayan büyüklük | üstel | beklenen değere eşit | `exponential` |
| negatif olmayan büyüklük, biçim α ve ölçek θ | gama | √α · θ | `gamma` |
| tam bilinen (tanımlı sabit) | — | 0 | `constant` |

Sınırlar `"lower"`/`"upper"` ya da `"value"`/`"half_width"` olarak girilebilir.

**B tipi belirsizliğin güvenilirliği.** Standart belirsizliğin kendisi yalnızca Δu/u bağıl kesinliğiyle
biliniyorsa ν ≈ ½ (Δu/u)⁻² serbestlik derecesi atanır (GUM G.4.2): %10 → ν = 50, %25 → ν = 8, %50 → ν = 2.
Proje dosyasında bu, girdinin `"dof"` alanıdır.

## 3.3 Duyarlılık katsayıları

Tahmin değerlerinde hesaplanan cᵢ = ∂f/∂Xᵢ duyarlılık katsayısı, bir girdinin belirsizliğini çıktının
belirsizliğine katkıya dönüştürür (GUM 5.1.3). Program modelin türevini **sembolik** olarak alır; bu nedenle
katsayılar kesindir (Kragten gibi tablolama yöntemleri sonlu farklar kullanır ve doğrusal olmayan modellerde
biraz farklı sonuç verir; QUAM A4 ve A5 notlarına bakınız). Her katsayının sembolik ifadesi JSON sonuçlarında
yer alır.

**Kragten görünümü.** Yayımlanmış birçok örnek (özellikle EURACHEM/CITAC QUAM) Kragten'in tablolama yöntemiyle
(QUAM Ek E.2) hesaplanmıştır; bu yöntem türevin yerine Δᵢ = f(…, xᵢ + u(xᵢ), …) − f(x) sonlu farkını kullanır.
`"gum": { "kragten": true }` ile program bu sütunu bütçeye ekler ve Kragten u_c değerini kesin birinci mertebe
değerin yanında verir. QUAM tablolama örneklerinde (A1–A5) basılı değerleri basılı tüm basamaklarda yeniden
üretir; bu, o değerlerle kesin yayılım yasası arasındaki küçük farkların yalnızca sonlu farklardan kaynaklandığını
gösterir.

Saf bir çarpım veya bölüm Y = c · X₁^{p₁} X₂^{p₂} ⋯ için bağıl belirsizlikler doğrudan birleşir:
(u(y)/y)² = Σ pᵢ² (u(xᵢ)/xᵢ)² (GUM 5.1.6).

## 3.4 Belirsizliğin yayılımı yasası

Girdi belirsizlikleri aralığında yeterince doğrusal bir model için (GUM 5.1.2, 5.2.2):

$$u_c^2(y) = \sum_{i=1}^{N} c_i^2\,u^2(x_i) + 2\sum_{i=1}^{N-1}\sum_{j=i+1}^{N} c_i c_j\, u(x_i)\,u(x_j)\, r(x_i,x_j).$$

**Belirsizlik bütçesi** her girdi için uᵢ(y) = cᵢ u(xᵢ) katkısını ve bunun birleşik varyanstaki payını,
uᵢ²(y)/u_c²(y), listeler. En büyük paylar, ölçümde iyileştirmenin nerede emeğe değeceğini gösterir.

**Korelasyonlar.** Ortak etkilere bağlı girdiler (aynı terazi, aynı referans standardı, ortak sıcaklık)
korelasyonludur. Korelasyon katsayılarını `"correlations"` içine giriniz. Çoğu zaman ortak etkiyi kendi başına
bir girdi büyüklüğü olarak açıkça modellemek korelasyonu ortadan kaldırmanın daha iyi yoludur — IDMS örneği
(QUAM A7) kütle ayrımı çarpanlarıyla bunu yapar.

## 3.5 Doğrusal olmayan modeller ve ikinci mertebe terimler

Model belirgin biçimde doğrusal değilse birinci mertebe yayılım yasası yetersiz kalabilir. GUM (5.1.2 notu)
bağımsız, normal dağılımlı girdiler için en önemli ikinci mertebe terimleri verir:

$$u_c^2(y) = \sum_i c_i^2 u^2(x_i) + \sum_i\sum_j \left[\tfrac12\left(\frac{\partial^2 f}{\partial x_i\,\partial x_j}\right)^2 + \frac{\partial f}{\partial x_i}\frac{\partial^3 f}{\partial x_i\,\partial x_j^2}\right] u^2(x_i)\,u^2(x_j).$$

Bu terimler özellikle, girdinin belirsizliği olduğu halde **duyarlılık katsayısı tahmin değerlerinde sıfırsa**
önemlidir — tipik olarak beklenen değerleri sıfır olan iki büyüklüğün çarpımı, örneğin bir sıcaklık sapması ile
genleşme katsayıları farkı (GUM H.1, EA-4/02 S4 ve S13, JCGM 101 9.3). Birinci mertebe sonuç bu katkıyı tamamen
yok sayar. **Program bu durumu algılar:** belirsiz bir girdinin duyarlılık katsayısı sıfır olduğu halde ikinci
mertebe terimlerle katkıda bulunuyorsa rapor uyarır ve birinci mertebe değerlendirmenin kaçırdığı varyans payını
belirtir (GUM H.1'de %12). Terimleri `"gum": { "higher_order": true }` ile etkinleştiriniz; program bu durumda
notun normal dağılımlı girdiler varsaydığı konusunda uyarır. Bu durumlarda güvenilir denetim Monte Carlo yöntemidir (4. bölüm).

**Kestirim.** GUM'un 1. düzeltmesi (JCGM 100:2008/Amd.1:2026) 4.1.4'e not 1'i ekler: doğrusal olmama belirginse
Taylor serisinin ikinci mertebe terimi kestirime de eklenmelidir:

$$y = f(x_1, \dots, x_N) + \tfrac12\sum_i\sum_j \frac{\partial^2 f}{\partial x_i\,\partial x_j}\,u(x_i, x_j).$$

Bu, GUM Denklem (H.10)'dur; bağımsız girdilerde yalnızca ∂²f/∂xᵢ² u²(xᵢ) terimleri kalır. Program bu terimi her zaman
hesaplar (JSON sonuçlarında `second_order_estimate_term`). `"higher_order": true` ile terimi y'ye ekler ve aralığı
düzeltilmiş kestirime ortalar. Seçenek kapalıyken terim u_c(y)'nin onda birini aşarsa rapor uyarır. JCGM 101 9.4'ün
karşılaştırma kaybı δY = X₁² + X₂² için düzeltilmiş kestirim analitik ortalamaya eşittir (x₁ = 0'da 50·10⁻⁶); birinci
mertebe kestirim 0 ise geçersizdir (JCGM 101 9.4.2.2.8). GUM H.1'de terim, Amd.1'in belirttiği gibi sıfırdır.

## 3.6 Serbestlik dereceleri ve Welch–Satterthwaite formülü

Birleşik standart belirsizliğin **etkin serbestlik derecesi** (GUM G.4.1, Eş. G.2b):

$$\nu_\text{eff} = \frac{u_c^4(y)}{\sum_i u_i^4(y)/\nu_i}.$$

ν = ∞ olan girdiler (B tipi değerlendirmelerin çoğu) paydaya katkıda bulunmaz. Program, GUM'un önerdiği gibi
(G.4.1 notu) t dağılımına bakmadan önce varsayılan olarak ν_eff'i bir alt tam sayıya keser;
`"dof_rounding": "exact"` kesirli değeri kullanır.

## 3.7 Kapsam faktörü ve genişletilmiş belirsizlik

**Genişletilmiş belirsizlik** U = k u_c(y), ölçülen büyüklüğe makul olarak atfedilebilecek değerlerin büyük
bir p kesrini içermesi beklenen y ± U aralığını tanımlar (GUM 6.2, 6.3).

* `"coverage_probability": 0.95` (varsayılan) ile k = t_p(ν_eff), iki taraflı Student t kantilidir (GUM G.3,
  G.4). ν_eff = ∞ için k = 1,96.
* EA-4/02, t(∞) = 2'nin tam olarak sağlandığı p = %95,45'i kullanır; Tablo E.1
  `"coverage_probability": 0.9545` ile yeniden üretilir.
* `"coverage_factor": 2` ile faktör sabittir ve program normal dağılım varsayımı altında karşılık gelen kapsam
  olasılığını (%95,45) belirtir.

**Normal varsayımı geçerli olmadığında.** Normal olmayan tek bir bileşen baskınsa çıktı dağılımı normalden çok
uzaktır ve k = 2 ciddi biçimde yanlış olabilir. EA-4/02 (S9, S10) bir veya iki baskın dikdörtgen katkı için
kural verir (yamuk çıktı; tek dikdörtgen terim için k = 1,65, β = 0,33 için 1,83); ancak genel ve daha
güvenilir çözüm, kapsam aralığını doğrudan veren Monte Carlo yöntemidir.

UKAS M3003 (Ek C) yaygın bir durumu, örneğin dijital bir göstergenin çözünürlüğü gibi tek bir baskın
dikdörtgen terimi, diğer terimleri normal kabul ederek ele alır. `"dominant_rectangular": true` ile program en
büyük dikdörtgen katkı u_R'yi bulur, u_N = √(u_c² − u_R²) alır ve iki dağılımın konvolüsyonu için
P(|R + N| ≤ k u_c) = p eşitliğini çözer. Bu, p = %95,45 için M3003 C.7 tablosunu yeniden üretir: u_N = 0 için
k = 1,65, u_N/u_R = 0,35 için 1,77, 0,70 için 1,90 ve dikdörtgen terim artık baskın değilken 2,00. Rapor girdiyi
ve oranı belirtir. M3003 C.5 örneğinde (`m3003-c5-dvm-dominant-rectangular` kütüphane projesi) Monte Carlo
aralığı bulunan y ± U ile uyuşur.

## 3.8 Bütçenin okunması

* Tek bir girdinin **varyans payı** %100'e yakınsa sonuç o girdi kadar iyidir; diğerleri önemsizdir. %2'lik bir
  katkıyı iyileştirmek hiçbir şeyi değiştirmez.
* Belirsiz bir girdinin **sıfır duyarlılık katsayısı** bir uyarı işaretidir: ikinci mertebe terimlerin veya
  Monte Carlo yönteminin gerekip gerekmediğini kontrol ediniz (3.5).
* **İşaretli katkılar** (cᵢu(xᵢ) < 0) yalnızca korelasyonlar aracılığıyla önem taşır; kareleri toplanır.
* Yaklaşık 10'un altındaki **etkin serbestlik dereceleri** kapsam faktörünü belirgin biçimde artırır; baskın A
  tipi girdinin daha fazla gözlemi bunu azaltır.

İncelenecek örnekler: QUAM A1–A3 (çarpımlar ve bölümler), A7 (33 girdili büyük bir model), GUM H.1 (serbestlik
dereceleri ve %99 kapsam), EA-4/02 S4 ve S13 (ikinci mertebe terimler), S12 (Welch–Satterthwaite).
