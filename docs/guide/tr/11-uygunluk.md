# 11. Uygunluk değerlendirmesi (JCGM 106)

ISO/IEC 17025:2017 7.8.6, bir şartnameye uygunluk beyan eden laboratuvarın ölçüm belirsizliğini hesaba katan
bir *karar kuralı* belgelemesini ve uygulamasını ister. `conformity` bölümü bu amaçla JCGM 106:2012'yi
uygular. Bu kılavuzun 7–9. maddelerindeki tüm sayısal örneklerle doğrulanmıştır (`jcgm106-*` kütüphane
projeleri).

## 11.1 Uygunluk olasılığı (Madde 7)

Şartname bir tolerans aralığı olarak verilir: bir alt sınır T_L, bir üst sınır T_U veya ikisi birden. Standart
belirsizliği u olan bir y sonucu ve ölçülen büyüklük için normal bir olasılık yoğunluğu ile uygunluk olasılığı
şudur:

p_c = Φ((T_U − y)/u) − Φ((T_L − y)/u)

(Formül 11; tek taraflı sınırlar için Formül 8 ve 9). `dof` (kendi başına projeler) veya `"distribution": "t"`
(etkin serbestlik dereceli model projeleri) ile olasılık yoğunluğu ölçeklenmiş ve kaydırılmış t dağılımı olur
(JCGM 101 6.4.9). Monte Carlo yöntemi çalıştırılan bir model projesinde p_c, Monte Carlo değerlerinin tolerans
aralığında kalan oranı olarak da verilir. İki taraflı toleranslarda rapor ölçüm yeterlilik indeksi
C_m = (T_U − T_L)/(4u) değerini de verir (Formül 12).

## 11.2 Karar kuralları ve koruma bantları (Madde 8)

| `rule` | Kabul sınırları | Kullanım |
|---|---|---|
| `simple_acceptance` (varsayılan) | = tolerans sınırları | paylaşılan risk; kabul edilebilir bulunmuş bir belirsizlik gerektirir (8.2) |
| `guarded_acceptance` | tolerans sınırlarının w kadar içinde | uygun olmayan bir ürünü kabul etme riski daha düşük (8.3.2) |
| `guarded_rejection` | tolerans sınırlarının w kadar dışında | olumsuz bir işlemden önce açık kanıt (8.3.3) |

Koruma bandı şu yollardan biriyle belirlenir:
- `r`: w = r·U, U = 2u, varsayılan r = 1 (ISO 14253-1);
- `guard_band`: w doğrudan verilir;
- `probability`: kabul sınırı, uygunluk olasılığının (korumalı kabul) veya uygunsuzluk olasılığının
  (korumalı ret) verilen değere ulaştığı yere konur. `u_relative` ile belirsizlik ölçülen değere bağlı
  olabilir; hız sınırı örneğinde (8.3.3, Örnek 1) olduğu gibi.

Bağıl belirsizliği büyük (yaklaşık %20'nin üstünde) pozitif bir ölçülen büyüklük için `"distribution":
"lognormal"`, medyanı ölçülen değere eşit ve s_G = u/|y| olan lognormal dağılımı kullanır (Eurachem/CITAC uygunluk
rehberi 2021, Ek A, Durum 4, Eşitlik 2); `"exact_sdlog": true` ile s_G = √ln(1 + (u/y)²) alınır. Sabit bağıl
belirsizlikte kabul sınırları tam olarak L·exp(±k·s_G) olur. Rehberin normal sınırları L(1 ± k·u_bağıl), u'yu
sınırda alır; yani sabit `u` = u_bağıl·L. `u_relative` ve normal dağılımla program bağıl belirsizliği ölçülen
değerde uygular; bu L/(1 ∓ k·u_bağıl) verir.

Kabul sınırları `acceptance_lower` ve `acceptance_upper` ile doğrudan da verilebilir; kural bu durumda
sınırlardan çıkarılır.

Rapor her sonuç için p_c değerini, kararı ve özgül riski verir. Ürün kabul edildiyse bu tüketici riski
1 − p_c, reddedildiyse üretici riski p_c'dir (9.3.2).

## 11.3 Bir üretim sürecinin küresel riskleri (Madde 9)

Bir `process` (`mean` ve `sd` ile `normal` veya `gamma`, ya da ln Y'nin `meanlog` ve `sdlog` değerleriyle
`lognormal` olasılık yoğunluğu) ve ölçüm sisteminin `u` değeri verildiğinde program Formül (17)–(20)'yi sayısal
olarak integre eder. `u_relative` ile ölçüm sistemi dağılımının standart sapması gerçek değerle orantılı
u_rel·η olur. Şunları raporlar:
- uygun ürünlerin oranı;
- kabul edilen ürünlerin oranı;
- küresel tüketici riski R_C (uygun olmadığı hâlde kabul edilen);
- küresel üretici riski R_P (uygun olduğu hâlde reddedilen);
- aynı sonuçlar 100 ürün başına.

`target_consumer_risk` ile istenen R_C'yi veren koruma bandı çarpanı r bulunur (9.5.4). Gama olasılık
yoğunluğu, bilyalı rulman örneğindeki radyal hata hareketi gibi negatif olamayan özelliklere uygundur (Ek B).
Lognormal olasılık yoğunluğu çevresel derişimlere uygundur (JCGM GUM-5 Örnek 10). İstenen bir R_C sabit bir `u`
gerektirir.

**Önselle özgül riskler.** `process` içeren kendi başına bir projede `"posterior": true`, her sonucun uygunluk
olasılığını sonsal dağılımdan, g(η | η_m) ∝ g₀(η)·h(η_m | η), hesaplar. Önsel g₀ süreç dağılımıdır; olabilirlik,
sonucun u değeriyle normaldir (`u_relative` ile u_rel·η_m; JCGM 106 6.2, Ek A). Sınır önselin kuyruğundaysa bu
fark yaratır: GUM-5 Örnek 10'da 0,225 mg/m³ için özgül üretici riski lognormal önselle %11,3, önselsiz %5,6'dır.

## 11.4 İki kullanım biçimi

- **Bir model projesiyle**: `model` bölümünün yanına `"conformity": { "upper": …, "rule": … }` eklenir. GUM
  değerlendirmesinin sonucu ve çalıştırılmışsa Monte Carlo yönteminin sonucu değerlendirilir. İkinci bir
  raporlanan ifade p_c'yi ve kararı verir. Uygulamada *Ayarlar → Uygunluk değerlendirmesi* toleransı ve kuralı
  belirler.
- **Kendi başına**: yalnızca `conformity` içeren bir proje; ölçülen değerleri (`results`), ölçüm sistemini
  (`u`, `u_relative`, `dof`) ve isteğe bağlı olarak süreci içerir. Denetim planları, nandrolon örneğindeki
  tarama sınırı gibi karar sınırları ve küresel risk çalışmaları için uygundur.

JCGM 106'daki bilyalı rulman örneği tutarsızdır. Metin kabul aralığını 0 ≤ Y_m ≤ A olarak verir ve R_C
formülü bu aralığı kullanır; ancak basılı R_P (%7,5) yalnızca alt sınır dışarıda bırakılırsa çıkar; alt
sınırla R_P %8,9'dur (araştırma günlüğü F45).
