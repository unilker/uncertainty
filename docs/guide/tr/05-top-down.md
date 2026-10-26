# 5. Top-down değerlendirme

Rutin deneylerde eksiksiz bir ölçüm modeli kurmak çoğu zaman pratik değildir: numune alma, ekstraksiyon, matris
etkileri ve günden güne değişim girdi girdi modellenmesi zor etkilerdir. **Top-down** (yukarıdan aşağı)
yaklaşım belirsizliği laboratuvarın zaten sahip olduğu verilerden tahmin eder: yöntem validasyonu, iç kalite
kontrol, referans malzemeler, yeterlilik testleri ve geri kazanım deneyleri. İki ana bileşeni, zaman içindeki
rastgele etkileri kapsayan **laboratuvar içi tekrarüretilebilirlik** u(Rw) ile sistematik etkileri kapsayan
**sapmanın** (gerçekliğin) belirsizliğidir.

Program Nordtest TR 537'yi (4. baskı, 2017) ve Eurachem/CITAC'ın *Evaluation of measurement uncertainty from
in-house precision and recovery data* (MUPR, 2026) rehberini izler. Her ikisi de y = x · (1 + δ_sapma) ·
(1 + δ_kesinlik) ⋯ biçiminde bir model için belirsizliğin yayılımı yasasına indirgenir; böylece mutlak ve bağıl
bileşenler şöyle birleşir:

$$u_c^2(y) = \sum_k m_k\,u_{\text{mutlak},k}^2 + y^2 \sum_k m_k\,u'^2_{\text{bağıl},k},$$

burada m_k bir bileşenin bütçeye kaç kez girdiğidir (örneğin hem kalibrantlar hem katkı için kullanılan bir
stok çözelti için 2, MUPR B1).

**Proje dosyalarında kural:** bağıl büyüklükler her zaman yüzde olarak, adı `_percent` ile biten alanlarda
girilir ve raporlanır; diğer tüm değerler ölçülen büyüklüğün birimindedir.

## 5.1 Laboratuvar içi tekrarüretilebilirlik u(Rw)

Kesinlik bileşeni rutin çalışmanın tüm adımlarını ve uzun bir dönemdeki tüm değişimlerini (farklı günler,
analistler, kalibrasyonlar, reaktif partileri) içermelidir. Kaynaklar, `"precision"` listesinde:

| Veri | `type` | Tahmin |
|---|---|---|
| bir kontrol numunesinin zaman içindeki sonuçları (kontrol grafiği) | `series` | s, ν = n − 1; `"relative": true` ile s/x̄ |
| birden fazla malzeme veya dönem, ham değerler veya (s, n) | `groups` | birleştirilmiş s_p = √(Σ(nᵢ − 1)sᵢ²/Σ(nᵢ − 1)) (MUPR Eş. 1–2) |
| gerçek numunelerin mükerrer ölçümleri | `duplicates` | `pooled`: s = √(Σd²/2N) (Nordtest Ek 5–6); bağıl: çiftlerin birleştirilmiş bağıl s'si; `normalised_differences`: (x₁ − x₂)/x̄'nin s'si bölü √2 (QUAM A4); `mean_range`: ortalama açıklık / 1,128 (MUPR §8.1.3) |
| k standart sapmada ±L kontrol grafiği sınırları | `control_limits` | L/k (Nordtest §5.1) |
| başka yerde verilmiş bir değer | `given` | s (isteğe bağlı ν) |
| s düzeye doğrusal bağlı | `linear` | s(c) = a + b·c, verilmiş ya da (c, s) noktalarına uydurulmuş (MUPR B3) |
| s(c) = √(s₀² + (c s′₁)²) | `continuous` | MUPR Eş. (A.1) |

**Kontrol numuneleri ve gerçek numuneler.** Kararlı bir kontrol numunesi farklı gerçek numuneler arasındaki
değişimi (homojensizlik, matris) göstermez. Bu nedenle Nordtest, gerçek numunelerin mükerrer ölçümlerinden
gelen tekrarlanabilirliği ekler: u(Rw) = √(u(Rw, kontrol)² + s_r(mükerrer)²); MUPR B6 aynı şeyi gerçek
numunelerin mükerrer açıklıklarıyla yapar.

**Çalışma aralığı boyunca kesinlik.** Kesinlik genellikle tayin sınırına yakın yerlerde mutlak olarak, daha
yüksek derişimlerde bağıl olarak sabittir. Bileşenleri `"range"` ile sınırlayınız (min dahil, max hariç);
örneğin 30 µg/L altında mutlak, üstünde bağıl s (Nordtest §5.3) veya MUPR'ın c_T = s⟨L⟩/s′⟨H⟩ geçiş derişimli
iki aralığı (MUPR Ek A). Mükerrer çiftler ortalamalarına göre `"min_mean"`/`"max_mean"` ile seçilebilir.

**Dönüşümler.** `"to_relative_at"` mutlak bir s'yi bir değere (veya verinin ortalaması olan `"mean"`'e) göre
bağıl olarak ifade eder; `"to_absolute_at"` bağıl bir s'yi bir değerde mutlak olarak ifade eder — örneğin MUPR
B4 düşük aralığın mutlak kesinliği olarak %4,52 × 6,46 mg/kg kullanır.

**Tekrarların ortalaması.** Raporlanan sonuç p günün her birinde n sonucun ortalamasıysa kesinliği
s_I(x̿) = √(s_I²/p + s_r²(1 − n)/(n p))'dir (MUPR Eş. 5): `"mean_of": { "replicates": n, "days": p,
"repeatability_percent": … }`.

## 5.2 Sapma u(bias) (Nordtest §6)

Sapmanın belirsizliği, gözlenen sapmaları referans değerlerin belirsizliğiyle birleştirir. `"kind"` değeri
`crm`, `proficiency_tests` veya `recovery` olan bir `"bias"` girdisinde her referans bir bᵢ sapması ve bir
u(C_ref,i) referans belirsizliği verir:

* **n kez ölçülmüş tek CRM** (ortalama x̄, standart sapma s): b = (x̄ − C)/C,

  $$u(\text{bias}) = \sqrt{b^2 + \left(\frac{s}{\sqrt n}\right)^2 + u^2(C_\text{ref})}.$$

* **Birden fazla CRM, yeterlilik testi veya geri kazanım deneyi:**

  $$u(\text{bias}) = \sqrt{\text{RMS}_\text{bias}^2 + \bar u^2(C_\text{ref})},\qquad \text{RMS}_\text{bias} = \sqrt{\frac{\sum b_i^2}{N}},$$

  ū(C_ref) referans belirsizliklerinin ortalamasıdır. Yeterlilik testinde u(C_ref,i) = s_R/√n_Lab'dır
  (`"sd_reproducibility_percent"` ve `"labs"`); sağlayıcının bildirdiği robust standart sapma önce 1,25 ile
  çarpılır (Nordtest §6.2 notu). Geri kazanım deneylerinde bᵢ = Rᵢ − %100 ve u(C_ref) eklenen miktarın
  belirsizliğidir.

Her referans ham sonuçlarla (`"measured"`), ortalama, s ve n ile veya doğrudan verilen bir sapmayla girilebilir.
Güvenilir bir tahmin için en az altı yeterlilik testi önerilir (Nordtest §6.2).

Birleşik belirsizlik u_c = √(u(Rw)² + u(bias)²) olur; Nordtest U = 2u_c değerini tam yüzdeye yukarı yuvarlar
(`"reporting": { "decimal_places": 0, "rounding": "up" }`).

## 5.3 Geri kazanım (MUPR §8.2)

MUPR gerçekliği referans malzemelerden, katkılı körlerden veya katkılı numunelerden elde edilen **ortalama geri
kazanım** R̄ ile ele alır (`"recovery"` bölümü):

* referans malzeme veya katkılı kör: Rᵢ = c̄ᵢ/Cᵢ, u²(Rᵢ) = Rᵢ²[(sᵢ/(c̄ᵢ√nᵢ))² + (u(Cᵢ)/Cᵢ)²] (Eş. 8–9);
* doğal analit içeren katkılı numune: Rᵢ = (c̄ − c̄₀)/c₊; her iki ortalamanın ve katkının belirsizlikleriyle
  (Eş. 10–11);
* R̄ = ΣRᵢ/N ve u(R̄) = √(Σu²(Rᵢ))/N (Eş. 12).

**Anlamlılık testi (Eş. 13).** |1 − R̄|/u(R̄), seçilen güven düzeyi (varsayılan %95) için iki taraflı t
değerinden büyükse geri kazanım 1'den anlamlı olarak farklıdır ve sonuçlar düzeltilir, c_düz = c/R̄; bağıl
bileşen u(R̄)/R̄ olur. Aksi halde sonuçlar düzeltilmez ve bileşen u(R̄)'dir. Kural `"correction": "always"` veya
`"never"` ile zorlanabilir. Düzeltilmiş bir sonuçta mutlak bileşenler de R̄'ye bölünür; y = x/R̄ için doğru
yayılım budur.

**Matrise bağlı geri kazanımlar.** Geri kazanımlar matrisler arasında değişiyorsa standart sapmaları s(R)
eklenir: u = √(u²(R̄) + s²(R)) (`"include_sd_of_recoveries": true`, MUPR B5); anlamlılık testi bu durumda
N − 1 serbestlik derecesine sahiptir. Eş. (16)'nın χ² testi tekil geri kazanımların eşdeğer olup olmadığını
gösterir (MUPR B6: χ² = 8,70 ≤ 32,7; dolayısıyla s(R) eklenmesi gerekmez).

### Ortak çalışmalardan kesinlik (ISO 21748, ISO 5725-3)

* `reproducibility` — ortak çalışma s_R ve s_r değerleri olan standart bir yöntem (ISO 21748:2017). Kesinlik
  bileşeni √(s_L² + s²/n_r)'dir; s_L² = s_R² − s_r² (veya `sd_between_labs`), s = s_r ya da doğrulanmışsa
  laboratuvarın kendi tekrarlanabilirliği s_l (7.3.2), n_r ortalaması alınan tam tekrar sayısıdır (Tablo 1).
  `method_bias`, çalışmada tahmin edilen yöntem sapmasının belirsizliğini ekler:
  u²(δ̂) = (s_R² − (1 − 1/n)s_r²)/p + u²(μ̂) (Formül 15). `bias_check`, bir referans malzemedeki laboratuvar
  sapmasının |Δ_l| < 2s_D, s_D² = s_L² + s_w²/n_l koşulunu sağlayıp sağlamadığını bildirir (Formül 4–5).
  Çalışmanın kapsamadığı etkiler (örnekleme, ön işlem, kurutma) ek bileşen olarak eklenir (Madde 8–9).
* `staggered_nested` — 1. gün iki ve 2. gün bir sonuçtan oluşan satırlar (ISO 5725-3:2023, 7.2 ve Ek C.1). Varyans
  analizi s_r'yi, zaman farklı ara kesinlik s_I(T)'yi ve gruplar arası s_R'yi verir; `estimate` hangisinin
  kullanılacağını seçer (varsayılan `intermediate`).

## 5.4 Ek bileşenler

Verilerin kapsamadığı bileşenler — stok çözelti, seyreltme, numunenin homojensizliği, kurutma —
`"components"` içine standart belirsizlik (`"u"`, `"u_percent"`) veya sınırlar (`"half_width"`,
`"half_width_percent"`, dikdörtgen veya üçgen) olarak, isteğe bağlı `"multiplicity"` ve `"range"` ile girilir.

## 5.5 Nerede değerlendirilir

Ölçülen değerleri `"levels"` içinde listeleyiniz; bütçe, sonuç (gerekiyorsa geri kazanım için düzeltilmiş) ve
varyans payları her biri için verilir. Düzey verilmezse tüm bileşenler bağıl olmalıdır ve sonuç bağıl
genişletilmiş belirsizliktir.

İncelenecek örnekler: Nordtest §3 (yöntemin tamamı), §5.3 (iki aralıkta mükerrerler), §6.1–6.3 (sapmanın üç
tahmin yolu), §8 (kontrol grafiği, CRM ve YT'nin dört birleşimi); MUPR B1–B6 (geri kazanım, kesinlik modelleri,
matris etkileri, 22 yeterlilik testi); QUAM A4 (mükerrerler ve geri kazanım) ve A6 (ampirik bir yöntemin
tekrarüretilebilirliği).
