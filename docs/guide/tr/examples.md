# Çözümlü örnekler

Bu bölüm örnek kütüphanesinden `uncertainty catalogue library --lang tr` komutuyla üretilmiştir; her sonuç, proje dosyasında saklanan girdilerden program tarafından yeniden hesaplanır ve bu dosya güncel değilse test takımı başarısız olur. Monte Carlo sonuçları burada listelenmemiştir: tam bütçe, grafikler ve yayımlanmış referans değerlerle karşılaştırma ile birlikte almak için projeyi çalıştırın.

- Kimya (105)
- Elektrik ve RF (34)
- Akış ve hacim (7)
- Genel (yöntem doğrulama) (19)
- Uzunluk (11)
- Kütle (28)
- Fiziksel özellikler (10)
- Basınç (1)
- Sıcaklık (10)

## Kimya

### Potansiyel ve sıcaklıkta üç doğrusal interpolasyonla 23,7 °C'de pH, bağımsız sertifikalı değerler (JCGM GUM-5 Örnek 1)

`gum5-e1-ph-temperature-interpolation` — JCGM GUM-5:2026, Example 1, 1.6, 1.9 and 1.10.3

Deney çözeltisi 23,7 °C'dedir, sertifika sıcaklıkları 20 °C ile 25 °C arasında: her sıcaklıkta potansiyelde, sonra sıcaklıkta interpolasyon; tüm girdiler bağımsız (GUM-5 1.7.1).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. GUM-5 1.6'nın algoritması ölçüm modelidir: pH_X,T1 ve pH_X,T2, T₁ = 25 °C ve T₂ = 20 °C'deki sertifikalı değerlerle potansiyelde interpolasyonla bulunur ve T_X = 23,7 °C'deki pH_X bunların arasında interpolasyonla elde edilir; T₁ ve T₂ kesin sertifika sıcaklıklarıdır.
2. pH_X,T1 = 7,0002, pH_X,T2 = 7,0161, pH_X = 0,74·pH_X,T1 + 0,26·pH_X,T2 = 7,0043; sıcaklıkta kübik interpolasyon S1'in pH değerini yalnızca 0,0004 değiştirir (GUM-5 1.10.1.1).
3. Sertifikalı değerler bağımsızken sıcaklıktaki interpolasyon 20 °C ve 25 °C'deki değerlerin belirsizliklerinin ortalamasını alır: u(pH_X) = 0,0032.

**Sonuç:**

- pH_X = (7,0043 ± 0,0063); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- GUM-5 1.9.1, üçüncü interpolasyonla pH_X = 7,0043 bulur (burada yeniden hesaplanan: 7,0043), ardından yöntemin pH_X = 7,0109 verdiğini söyler; 7,0109 verilerden çıkmaz. 1.7.1 tüm girdileri bağımsız varsaydığı hâlde u = 0,0041 basar; bağımsızlıkla u = 0,0032 çıkar. 0,0041, her standardın 20 °C ve 25 °C'deki sertifikalı değerleri tam ilişkili alındığında elde edilir (0,0041) (araştırma günlüğü F78).

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e1-ph-temperature-interpolation.json --lang tr`

### Üç doğrusal interpolasyonla 23,7 °C'de pH, her standardın sertifikalı değerleri sıcaklıklar arasında ilişkili (JCGM GUM-5 Örnek 1)

`gum5-e1-ph-temperature-interpolation-correlated` — JCGM GUM-5:2026, Example 1, 1.6, 1.9 and 1.10.3

Bağımsız durum gibi, ancak her standardın 20 °C ve 25 °C'deki pH değerleri, ortak Bates–Guggenheim kuralının gösterdiği gibi tam ilişkilidir (GUM-5 1.10.3.3).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. GUM-5 1.6'nın algoritması ölçüm modelidir: pH_X,T1 ve pH_X,T2, T₁ = 25 °C ve T₂ = 20 °C'deki sertifikalı değerlerle potansiyelde interpolasyonla bulunur ve T_X = 23,7 °C'deki pH_X bunların arasında interpolasyonla elde edilir; T₁ ve T₂ kesin sertifika sıcaklıklarıdır.
2. pH_X,T1 = 7,0002, pH_X,T2 = 7,0161, pH_X = 0,74·pH_X,T1 + 0,26·pH_X,T2 = 7,0043; sıcaklıkta kübik interpolasyon S1'in pH değerini yalnızca 0,0004 değiştirir (GUM-5 1.10.1.1).
3. Aynı standardın değerleri arasında r = 1 iken sıcaklıktaki interpolasyon artık bunların ortalamasını almaz: u(pH_X) = 0,0041; GUM-5'in bastığı değer budur.

**Sonuç:**

- pH_X = (7,0043 ± 0,0080); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- GUM-5 1.9.1, üçüncü interpolasyonla pH_X = 7,0043 bulur (burada yeniden hesaplanan: 7,0043), ardından yöntemin pH_X = 7,0109 verdiğini söyler; 7,0109 verilerden çıkmaz. 1.7.1 tüm girdileri bağımsız varsaydığı hâlde u = 0,0041 basar; bağımsızlıkla u = 0,0032 çıkar. 0,0041, her standardın 20 °C ve 25 °C'deki sertifikalı değerleri tam ilişkili alındığında elde edilir (0,0041) (araştırma günlüğü F78).

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e1-ph-temperature-interpolation-correlated.json --lang tr`

### 25 °C'de iki noktalı interpolasyonla bir deney çözeltisinin pH değeri (JCGM GUM-5 Örnek 1)

`gum5-e1-ph-two-point-25c` — JCGM GUM-5:2026, Example 1, 1.8

Aynı hücreyle ölçülen iki NIST fosfat pH standardı arasında parantez yöntemi; IUPAC uygulamasındaki gibi girdiler bağımsız.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Parantez yöntemiyle iki noktalı kalibrasyon (GUM-5 1.4.2, 1.5): deney çözeltisinin pH değeri, iki NIST fosfat standardının (E_S1, pH_S1) ve (E_S2, pH_S2) noktaları arasında doğrusal interpolasyonla bulunur: pH_X = pH_S1 + (pH_S2 − pH_S1)(E_X − E_S1)/(E_S2 − E_S1).
2. q = (E_X − E_S1)/(E_S2 − E_S1) = 0,2469 ve eğim b = -0,016975 mV⁻¹ ile u²(pH_X) = b²(1 − q)²u²(E_S1) + (1 − q)²u²(pH_S1) + b²q²u²(E_S2) + q²u²(pH_S2) + b²u²(E_X) (Formül 1.17); katkılar 0,0038 (pH_S1), 0,0013 (pH_S2), 0,0004, 0,0004 ve 0,0001'dir.
3. pH_X = 7,0002, u(pH_X) = 0,0041: sertifikalı pH değerleri baskındır. Üç potansiyelin ortak kalibrasyon hatası δE_cal birbirini götürür (GUM-5 1.8.2).

**Sonuç:**

- pH_X = (7,0002 ± 0,0080); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e1-ph-two-point-25c.json --lang tr`

### Tam ilişkili pH standartlarıyla iki noktalı interpolasyon (JCGM GUM-5 Örnek 1)

`gum5-e1-ph-two-point-25c-correlated-standards` — JCGM GUM-5:2026, Example 1, 1.10.3.4 and 1.10.3.5

25 °C durumu gibi, ancak Bates–Guggenheim kuralını paylaşan sertifikalı pH değerleri arasında korelasyon 1.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Parantez yöntemiyle iki noktalı kalibrasyon (GUM-5 1.4.2, 1.5): deney çözeltisinin pH değeri, iki NIST fosfat standardının (E_S1, pH_S1) ve (E_S2, pH_S2) noktaları arasında doğrusal interpolasyonla bulunur: pH_X = pH_S1 + (pH_S2 − pH_S1)(E_X − E_S1)/(E_S2 − E_S1).
2. Sertifikalar SI'ya izlenebilir bir belirsizlik ve Bates–Guggenheim kuralı olmadan çok daha küçük bir belirsizlik verir; kural terimi iki standart için ortaktır, bu nedenle iki pH değerinin korelasyonu 1'e yakındır (GUM-5 1.10.3.3). Burada r = 1.
3. r = 1 iken (1 − q)²u²(pH_S1) + 2q(1 − q)u(pH_S1, pH_S2) + q²u²(pH_S2) terimleri u²(pH_S1)'e indirgenir (Formül 1.24): u(pH_X) = 0,0051; korelasyon yok sayılırsa 0,0041. Tam ilişkili katkılar doğrusal toplanır: 0,0038 + 0,0013 = 0,0051.

**Sonuç:**

- pH_X = (7,000 ± 0,010); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- 1.10.3.5, u = 0,0051 değerini pH_X = 7,0109'a ve '1.7 alt maddesindeki verilere' bağlar; hesap 1.8'in 25 °C verilerini kullanır ve bu veriler için pH_X = 7,0002'dir (araştırma günlüğü F78).

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e1-ph-two-point-25c-correlated-standards.json --lang tr`

### Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,17 mg/m³ (JCGM GUM-5 Örnek 10)

`gum5-e10-tspm-global-risks-acceptance-0.17` — JCGM GUM-5:2026, Example 10, 10.6.3

T_U = 0,2 mg/m³, lognormal popülasyon dağılımı, bağıl belirsizlik %7; kabul sınırı 0,17 mg/m³ değerine kaydırıldı.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Üst tolerans sınırı T_U = 0,2 mg/m³ (toplam askıda partikül madde için İsrail 24 saatlik sınırı). Popülasyonun önsel dağılımı, bir taş ocağı yakınındaki 220 sonuca uydurulmuş lognormaldir: ln ρ için µ = −2,325 ve σ = 0,434 (Formül 10.1); popülasyonun %95,0'ı uygundur.
2. Her sonucun bağıl standart belirsizliği %7'dir; baskın kaynak örneklenen hava hacmidir. Ölçüm sistemi dağılımı h(ρ̂ | ρ), gerçek değerle orantılı 0,07·ρ standart sapmalı normaldir; böylece her ρ için ρ̂'nin bir dağılımıdır (JCGM 106 9.5.1); küresel riskler JCGM 106 Formül (19) ve (20)'den bulunur.
3. R_C = %0,0103 ve R_P = %5,370. GUM-5 0,002 % ve 5,8 % basar; ortak dağılımı %0,0020 ve %5,781 verir.

**Notlar:**

- GUM-5 10.6.3.1 ortak dağılımda 0,07·ρ̂ standart sapmasını kullanır ve onu C = 1/1,005 ile normalleştirir; bu basılı riskleri yeniden üretir (araştırma günlüğü F80, tools/research/gum5/ex10_risks.py). Tüketici (yöre halkı) riski sınırı aşan havanın kabulü, üretici (ocak sahipleri) riski uygun havanın reddidir.

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e10-tspm-global-risks-acceptance-0.17.json --lang tr`

### Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,22 mg/m³ (JCGM GUM-5 Örnek 10)

`gum5-e10-tspm-global-risks-acceptance-0.22` — JCGM GUM-5:2026, Example 10, 10.6.3

T_U = 0,2 mg/m³, lognormal popülasyon dağılımı, bağıl belirsizlik %7; kabul sınırı 0,22 mg/m³ değerine kaydırıldı.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Üst tolerans sınırı T_U = 0,2 mg/m³ (toplam askıda partikül madde için İsrail 24 saatlik sınırı). Popülasyonun önsel dağılımı, bir taş ocağı yakınındaki 220 sonuca uydurulmuş lognormaldir: ln ρ için µ = −2,325 ve σ = 0,434 (Formül 10.1); popülasyonun %95,0'ı uygundur.
2. Her sonucun bağıl standart belirsizliği %7'dir; baskın kaynak örneklenen hava hacmidir. Ölçüm sistemi dağılımı h(ρ̂ | ρ), gerçek değerle orantılı 0,07·ρ standart sapmalı normaldir; böylece her ρ için ρ̂'nin bir dağılımıdır (JCGM 106 9.5.1); küresel riskler JCGM 106 Formül (19) ve (20)'den bulunur.
3. R_C = %1,8010 ve R_P = %0,055. GUM-5 1,7 % ve < 0,1 % basar; ortak dağılımı %1,6862 ve %0,115 verir.

**Notlar:**

- GUM-5 10.6.3.1 ortak dağılımda 0,07·ρ̂ standart sapmasını kullanır ve onu C = 1/1,005 ile normalleştirir; bu basılı riskleri yeniden üretir (araştırma günlüğü F80, tools/research/gum5/ex10_risks.py). Tüketici (yöre halkı) riski sınırı aşan havanın kabulü, üretici (ocak sahipleri) riski uygun havanın reddidir.

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e10-tspm-global-risks-acceptance-0.22.json --lang tr`

### Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,2 mg/m³ (JCGM GUM-5 Örnek 10)

`gum5-e10-tspm-global-risks-shared-risk` — JCGM GUM-5:2026, Example 10, 10.6.3

T_U = 0,2 mg/m³, lognormal popülasyon dağılımı, bağıl belirsizlik %7; paylaşılan risk kuralı A_U = T_U.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Üst tolerans sınırı T_U = 0,2 mg/m³ (toplam askıda partikül madde için İsrail 24 saatlik sınırı). Popülasyonun önsel dağılımı, bir taş ocağı yakınındaki 220 sonuca uydurulmuş lognormaldir: ln ρ için µ = −2,325 ve σ = 0,434 (Formül 10.1); popülasyonun %95,0'ı uygundur.
2. Her sonucun bağıl standart belirsizliği %7'dir; baskın kaynak örneklenen hava hacmidir. Ölçüm sistemi dağılımı h(ρ̂ | ρ), gerçek değerle orantılı 0,07·ρ standart sapmalı normaldir; böylece her ρ için ρ̂'nin bir dağılımıdır (JCGM 106 9.5.1); küresel riskler JCGM 106 Formül (19) ve (20)'den bulunur.
3. R_C = %0,5790 ve R_P = %0,739. GUM-5 0,5 % ve 0,9 % basar; ortak dağılımı %0,4984 ve %0,909 verir.

**Notlar:**

- GUM-5 10.6.3.1 ortak dağılımda 0,07·ρ̂ standart sapmasını kullanır ve onu C = 1/1,005 ile normalleştirir; bu basılı riskleri yeniden üretir (araştırma günlüğü F80, tools/research/gum5/ex10_risks.py). Tüketici (yöre halkı) riski sınırı aşan havanın kabulü, üretici (ocak sahipleri) riski uygun havanın reddidir.

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e10-tspm-global-risks-shared-risk.json --lang tr`

### Havada askıda partikül madde: lognormal önselle sonsal dağılımdan özgül riskler (JCGM GUM-5 Örnek 10)

`gum5-e10-tspm-specific-risks` — JCGM GUM-5:2026, Example 10, 10.6.4 and 10.7.2

Tek sonuçların uygunluk olasılıkları, sonsal g(ρ | ρ̂) ∝ g₀(ρ)·h(ρ̂ | ρ), lognormal önsel, u = 0,07·ρ̂, A_U = T_U = 0,2 mg/m³.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Üst tolerans sınırı T_U = 0,2 mg/m³ (toplam askıda partikül madde için İsrail 24 saatlik sınırı). Popülasyonun önsel dağılımı, bir taş ocağı yakınındaki 220 sonuca uydurulmuş lognormaldir: ln ρ için µ = −2,325 ve σ = 0,434 (Formül 10.1); popülasyonun %95,0'ı uygundur.
2. Verilen bir sonuç için olabilirlik, u = 0,07·ρ̂ standart sapmalı normaldir (10.6.4.3). Sonsal dağılımın kapalı biçimi yoktur ve sayısal olarak integre edilir; p_c = Pr(ρ ≤ T_U | ρ̂).
3. ρ̂ = 0,190 mg/m³ %14,3 özgül tüketici riskiyle (1 − p_c) kabul edilir; ρ̂ = 0,225 mg/m³ %11,3 özgül üretici riskiyle (p_c) reddedilir: gerçek derişimin uygun olma olasılığı %11'dir (10.7.2). Önsel olmadan riskler %22,6 ve %5,6'dir; önsel geniştir, ancak üst kuyruğunda kalan sınırın yakınında ihmal edilemez.

**Sonuç:**

- ρ̂ = 0.190 için ρ ≤ 0,2 mg/m³ şartına uygunluk olasılığı: p_c = %86; basit kabul kuralına göre kabul.
- ρ̂ = 0.225 için ρ ≤ 0,2 mg/m³ şartına uygunluk olasılığı: p_c = %11; basit kabul kuralına göre ret.

**Notlar:**

- 10.6.4 sonsal dağılımın esasen olabilirlikle belirlendiğini, bu nedenle frekansçı özgül risklerin çok yakın olacağını belirtir. T_U yakınında bu geçerli değildir: yalnız normal dağılım 0,190'da 1 − p_c = %22,6, 0,225 mg/m³'te p_c = %5,6 verir (araştırma günlüğü F80).

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e10-tspm-specific-risks.json --lang tr`

### İç standartla GC-MS'te benzo[a]piren, ilişkili girdiler (JCGM GUM-5 Örnek 2)

`gum5-e2-benzo-a-pyrene` — JCGM GUM-5:2026, Example 2, Tables 2.1–2.3

İki ilişkili girdi çiftli iç standart modeli; kovaryanslı LPU ve basıldığı gibi Welch–Satterthwaite ν_eff.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100)

**Adımlar:**

1. İç standart modeli (EN 15549): m_E = f·A_E/A_ISE·m_ISE; NIST SRM 2260a ile zenginleştirilmiş cam elyaf filtrenin ikinci ekstraktının 1 µL'sindeki benzo[a]piren kütlesi.
2. Kalibrasyonda ve analizde aynı iç standart kullanıldığından r(f, m_ISE) = −3,3·10⁻⁵/(0,017 × 0,0036) = -0,54; aynı enjeksiyonların pik alanları r(A_E, A_ISE) = −2,03·10⁸/(9564 × 44 492) = -0,48 verir.
3. Kovaryans terimleriyle u²(m_E) = 4,077e-06 ng², onlarsız 3,140e-06 ng²; korelasyon negatif, duyarlılıklar zıt işaretli olduğundan A_E–A_ISE terimi 1,02e-06 ng² ekler.
4. Welch–Satterthwaite (G.2b), payda korelasyonsuz: ν_eff = 5,20, k(%95) = 2,5; program G.2b'nin ilişkili girdiler için geçerli olmadığını uyarır. GUM-5 denklem (2.3) 4,81 ve k = 2,6, U = 0,0052 ng verir; eşlik eden projedeki topluluk yaklaşımı 2,11 verir.

**Sonuç:**

- mE = (0,0140 ± 0,0051) ng; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,54 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- GUM-5 (2.4.2) MCM'yi de uygular: (f, m_ISE) için iki değişkenli normal, (A_E, A_ISE) için ν = n − N = 1 olan iki değişkenli t ve negatif çekilişlerin reddi; medyan 0,014 ng, en kısa %95 aralık [≈ 0; 0,032] ng. Proje biçiminde skaler model için ortak t-dağılımı olmadığından MCM burada çalıştırılmaz; tools/research/gum5/ex2_mcm.py basılı aralığı yeniden üretir (10⁷ çekilişin 9,3·10⁶'sı tutuldu, [1·10⁻⁷; 0,0321] ng).
- Belirsizlik yalnız temiz, zenginleştirilmiş filtre için geçerlidir; matris etkileri ve geri kazanım bilerek dışarıda bırakılmıştır (GUM-5 2.2.2, not).

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e2-benzo-a-pyrene.json --lang tr`

### Benzo[a]piren: aynı enjeksiyonların pik alanları ν_eff'te topluluk olarak (JCGM GUM-5 Örnek 2, Willink 2007)

`gum5-e2-benzo-a-pyrene-ensemble` — JCGM GUM-5:2026, Example 2, Tables 2.1–2.3; Willink, Metrologia 44 (2007) 340, §4.1.1

Aynı bütçe, A_E ve A_ISE topluluk olarak tanımlanmış: ortak varyansları ν = 2 olan tek Welch–Satterthwaite bileşenidir; ν_eff 5,2 veya 4,8 yerine ≈ 2,1 olur.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100)

**Adımlar:**

1. İç standart modeli (EN 15549): m_E = f·A_E/A_ISE·m_ISE; NIST SRM 2260a ile zenginleştirilmiş cam elyaf filtrenin ikinci ekstraktının 1 µL'sindeki benzo[a]piren kütlesi.
2. Kalibrasyonda ve analizde aynı iç standart kullanıldığından r(f, m_ISE) = −3,3·10⁻⁵/(0,017 × 0,0036) = -0,54; aynı enjeksiyonların pik alanları r(A_E, A_ISE) = −2,03·10⁸/(9564 × 44 492) = -0,48 verir.
3. Kovaryans terimleriyle u²(m_E) = 4,077e-06 ng², onlarsız 3,140e-06 ng²; korelasyon negatif, duyarlılıklar zıt işaretli olduğundan A_E–A_ISE terimi 1,02e-06 ng² ekler.
4. A_E ve A_ISE aynı üç eşli enjeksiyonun ortalamalarıdır; tahmin edilen kovaryans matrisleri ν = 2 olan Wishart dağılımına uyar; bu nedenle her c_E·A_E + c_I·A_ISE bileşiminin varyans tahmini tam ν = 2'ye sahiptir (Willink 2007, §4.1.1; GTC real_ensemble). Bileşenleri (c_E u_E)² + (c_I u_I)² + 2c_E c_I u(A_E, A_ISE) = 3,971e-06 ng², u²'nin %97'idir.
5. ν_eff = u⁴/[(c_f u_f)⁴/9 + 3,971e-06²/2] = 2,11; k(%95) = 4,1 ve U = 0,0083 ng; basılı U = 0,0052 ng'den çok MCM aralığının [≈ 0; 0,032] ng genişliğine yakındır. GUM-5 denklem (2.3) iki alanı ayrı bileşenler olarak ele alır ve baskın bileşenin ν'si 2 olduğu hâlde 2'nin üstünde ν_eff verir (araştırma günlüğü F79). f–m_ISE korelasyonu (ν = 9 ve ∞) topluluk değildir; bu nedenle uyarı kalır.

**Sonuç:**

- mE = (0,0140 ± 0,0083) ng; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 4,10 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- GUM-5 (2.4.2) MCM'yi de uygular: (f, m_ISE) için iki değişkenli normal, (A_E, A_ISE) için ν = n − N = 1 olan iki değişkenli t ve negatif çekilişlerin reddi; medyan 0,014 ng, en kısa %95 aralık [≈ 0; 0,032] ng. Proje biçiminde skaler model için ortak t-dağılımı olmadığından MCM burada çalıştırılmaz; tools/research/gum5/ex2_mcm.py basılı aralığı yeniden üretir (10⁷ çekilişin 9,3·10⁶'sı tutuldu, [1·10⁻⁷; 0,0321] ng).
- Belirsizlik yalnız temiz, zenginleştirilmiş filtre için geçerlidir; matris etkileri ve geri kazanım bilerek dışarıda bırakılmıştır (GUM-5 2.2.2, not).

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e2-benzo-a-pyrene-ensemble.json --lang tr`

### Standart atom ağırlığı aralıklarından glikozun bağıl molekül kütlesi (JCGM GUM-5 Örnek 3)

`gum5-e3-molar-mass-glucose` — JCGM GUM-5:2026, Example 3, Tables 3.1 and 3.2

IUPAC 2021 standart atom ağırlığı aralıkları üzerinde dikdörtgen dağılımlarla M_r(C₆H₁₂O₆) = 6A_C + 12A_H + 6A_O.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Glikozun izotop bileşimi bilinmemektedir; normal bir malzeme için standart atom ağırlıkları [1,007 84; 1,008 11] (H), [12,0096; 12,0116] (C) ve [15,999 03; 15,999 77] (O) aralıklarındadır; bunlara dikdörtgen dağılım atanır (GUM 4.3.7).
2. Model kesin katsayılarla doğrusaldır: u(M_r) = √Σν_i²u²(A_i); katkılar 0,00094 (H), 0,00346 (C) ve 0,00128 (O) olup karbon baskındır.
3. M_r = 180,1557, u(M_r) = 0,0038. Dikdörtgen dağılımlar gerçek bileşimi değil, izotop bileşimi hakkındaki bilgi eksikliğini ifade eder (GUM-5 3.4.2).

**Sonuç:**

- M_r = (180,1557 ± 0,0075); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- CO₂ için aynı hesap kütüphanedeki NIST TN 1900 Örnek E8'dir.

**Çalıştırma:** `uncertainty run library/chemistry/gum5-e3-molar-mass-glucose.json --lang tr`

### Bir YT turunda sansürlü sonuçlar: sansürlü sonuçlar çıkarılarak (ISO 13528 E.1)

`iso13528-e1-censored-deleted` — ISO 13528:2022, Annex E.1 (Table E.1)

ISO 13528:2022 E.1: beşi ‘küçüktür’ değeri olan 23 sonuç; Algoritma A'nın robust ortalaması ve standart sapması sansürlü sonuçların nasıl işlendiğine bağlıdır.

**Yaklaşım:** yeterlilik testi istatistikleri (ISO 13528)

**Adımlar:**

1. 23 katılımcı; beşi ‘<10’, ‘<10’, ‘<20’, ‘<30’ ve ‘<50’ bildirmiştir (Tablo E.1).
2. Beş sansürlü sonuç ‘küçüktür’ olarak bildirilir ve istatistiklerin dışında kalır (5.5.3); p = 18 kalır.
3. p = 18 değer üzerinde Algoritma A (C,3,1): x* = 26,81, s* = 5,29. Üç yaklaşım x* için 23,95–26,81 ve s* için 5,29–8,60 verir: yeterlilik testi sağlayıcısı yaklaşımı önceden belirlemelidir (5.5.3.3).

**Sonuç:**

- x_pt = 26,8, U(x_pt) = 3,1 (k = 2,00)

**Notlar:**

- Tablo E.1, x* ± 3s* dışındaki sonuçları ‘#’ ile işaretler; buradaki robust değerlerle bu sonuçların z skorlarının mutlak değeri 3'ü aşar (‘<’ yok sayıldığında Z = 50, sansürlü sonuçlar çıkarıldığında Y = 45).

**Çalıştırma:** `uncertainty run library/chemistry/iso13528-e1-censored-deleted.json --lang tr`

### Bir YT turunda sansürlü sonuçlar: sınırın 0,5 katı (ISO 13528 E.1)

`iso13528-e1-censored-half` — ISO 13528:2022, Annex E.1 (Table E.1)

ISO 13528:2022 E.1: beşi ‘küçüktür’ değeri olan 23 sonuç; Algoritma A'nın robust ortalaması ve standart sapması sansürlü sonuçların nasıl işlendiğine bağlıdır.

**Yaklaşım:** yeterlilik testi istatistikleri (ISO 13528)

**Adımlar:**

1. 23 katılımcı; beşi ‘<10’, ‘<10’, ‘<20’, ‘<30’ ve ‘<50’ bildirmiştir (Tablo E.1).
2. Her sansürlü sonuç sınırının yarısıyla değiştirilir (5, 5, 10, 15, 25).
3. p = 23 değer üzerinde Algoritma A (C,3,1): x* = 23,95, s* = 8,60. Üç yaklaşım x* için 23,95–26,81 ve s* için 5,29–8,60 verir: yeterlilik testi sağlayıcısı yaklaşımı önceden belirlemelidir (5.5.3.3).

**Sonuç:**

- x_pt = 24,0, U(x_pt) = 4,5 (k = 2,00)

**Notlar:**

- Tablo E.1, x* ± 3s* dışındaki sonuçları ‘#’ ile işaretler; buradaki robust değerlerle bu sonuçların z skorlarının mutlak değeri 3'ü aşar (‘<’ yok sayıldığında Z = 50, sansürlü sonuçlar çıkarıldığında Y = 45).
- C.3.1'in durma kuralıyla (üçüncü anlamlı basamak) Algoritma A 11 iterasyondan sonra x* = 23,96, s* = 8,59'da durur; yakınsamaya kadar sürdürüldüğünde x* = 23,959, s* = 8,600 verir. Tablo E.1 23,95 ve 8,60 basar; yani yakınsamış değerler, x* yuvarlanmak yerine kesilmiş (araştırma günlüğü F44). Proje yakınsamaya kadar iterasyon yapar ve x*'ı 0,01 içinde kontrol eder.

**Çalıştırma:** `uncertainty run library/chemistry/iso13528-e1-censored-half.json --lang tr`

### Bir YT turunda sansürlü sonuçlar: ‘<’ işareti yok sayılarak (ISO 13528 E.1)

`iso13528-e1-censored-ignored` — ISO 13528:2022, Annex E.1 (Table E.1)

ISO 13528:2022 E.1: beşi ‘küçüktür’ değeri olan 23 sonuç; Algoritma A'nın robust ortalaması ve standart sapması sansürlü sonuçların nasıl işlendiğine bağlıdır.

**Yaklaşım:** yeterlilik testi istatistikleri (ISO 13528)

**Adımlar:**

1. 23 katılımcı; beşi ‘<10’, ‘<10’, ‘<20’, ‘<30’ ve ‘<50’ bildirmiştir (Tablo E.1).
2. ‘<’ işaretleri atılır ve beş sınır nicel sonuç olarak işlenir (10, 10, 20, 30, 50).
3. p = 23 değer üzerinde Algoritma A (C,3,1): x* = 26,01, s* = 7,23. Üç yaklaşım x* için 23,95–26,81 ve s* için 5,29–8,60 verir: yeterlilik testi sağlayıcısı yaklaşımı önceden belirlemelidir (5.5.3.3).

**Sonuç:**

- x_pt = 26,0, U(x_pt) = 3,8 (k = 2,00)

**Notlar:**

- Tablo E.1, x* ± 3s* dışındaki sonuçları ‘#’ ile işaretler; buradaki robust değerlerle bu sonuçların z skorlarının mutlak değeri 3'ü aşar (‘<’ yok sayıldığında Z = 50, sansürlü sonuçlar çıkarıldığında Y = 45).

**Çalıştırma:** `uncertainty run library/chemistry/iso13528-e1-censored-ignored.json --lang tr`

### Çikolatada arsenik: YT numunelerinin homojenliği ve kararlılığı (ISO 13528 E.2)

`iso13528-e2-arsenic-homogeneity` — ISO 13528:2022, E.2, Tables E.2 and E.3

ISO 13528:2022 E.2: ikişer kez ölçülen 10 numuneden numuneler arası standart sapma, 0,3σ_pt ile karşılaştırma ve 60 °C'de altı hafta sonra kararlılık kontrolü.

**Yaklaşım:** yeterlilik testi istatistikleri (ISO 13528)

**Adımlar:**

1. 1 000 şişeden tabakalı rastgele örneklemeyle seçilen on şişe, tekrarlanabilirlik koşullarında ikişer kez test edilmiştir (Tablo E.2).
2. Varyans analizi (B.3): genel ortalama 0,18715 mg/kg, numune ortalamalarının standart sapması s_x = 0,00398, numune içi s_w = 0,00556 ve numuneler arası s_s = √(s_x² − s_w²/2) = 0,00060 mg/kg.
3. Ortalamanın %15'i olan geçici σ_pt = 0,02807 mg/kg ile s_s, 0,3σ_pt = 0,00842 mg/kg'nin çok altındadır: numuneler yeterince homojendir. B.2.3'ün genişletilmiş ölçütü (g = 10 için F₁ = 1,88, F₂ = 1,01) √c = 0,0133 mg/kg verir.
4. Kararlılık: 60 °C'de altı hafta saklanan iki numune 0,19375 mg/kg ortalama verir; homojenlik ortalamasından 0,00660 mg/kg'lik fark 0,3σ_pt'nin altındadır, dolayısıyla kararlılık yeterlidir (B.5.1).

**Çalıştırma:** `uncertainty run library/chemistry/iso13528-e2-arsenic-homogeneity.json --lang tr`

### İçme suyunda atrazin: Algoritma A ile robust ortalama (ISO 13528 E.3)

`iso13528-e3-atrazine-algorithm-a` — ISO 13528:2022, E.3, Tables E.4 and E.5

ISO 13528:2022 E.3: 34 katılımcı sonucundan Algoritma A ile atanan değer ve yeterlilik değerlendirmesi için standart sapma; z skorları.

**Yaklaşım:** yeterlilik testi istatistikleri (ISO 13528)

**Adımlar:**

1. 34 katılımcı içme suyunda atrazin sonucu bildirmiştir (Tablo E.4, değere göre sıralı). İki çok düşük (0,040 ve 0,055 mg/L) ve bir yüksek (0,4246 mg/L) sonuç çoğunluktan çok uzaktır.
2. Algoritma A (C.3.1) x* = medyan = 0,2620 ve s* = MADe = 0,0386 mg/L değerleriyle başlar, x* ± 1,5s* dışındaki her sonucu yakın sınırla değiştirir ve x*'ı değiştirilmiş değerlerin ortalaması, s*'ı standart sapmalarının 1,134 katı olarak yeniden hesaplar.
3. Altı iterasyondan sonra üçüncü anlamlı basamaklar değişmez: x* = 0,2570 mg/L, s* = 0,0395 mg/L. Aritmetik ortalama 0,2512 ve standart sapma 0,0672 mg/L, üç uç sonuç tarafından çekilir.
4. Atanan değer x_pt = x*, u(x_pt) = 1,25 s*/√p = 1,25 × 0,0395/√34 = 0,0085 mg/L (Formül 6) ve σ_pt = s*'tır. u(x_pt) < 0,3σ_pt = 0,0119 mg/L olduğundan atanan değerin belirsizliği ihmal edilebilir (9.2.1) ve z skorları kullanılır; 1, 2 ve 34 numaralı laboratuvarlar eylem sinyali alır.

**Sonuç:**

- x_pt = 0,257 mg/L, U(x_pt) = 0,017 mg/L (k = 2,00)

**Notlar:**

- C.3.1'in durma kuralı (üçüncü anlamlı basamaklarda değişiklik olmaması) kullanılmıştır; tam yakınsamaya kadar iterasyon x* = 0,257011 ve s* = 0,039520 mg/L verir, basılı basamaklara kadar aynıdır. Tablo E.5'teki nIQR (0,0402), sıra istatistikleri arasında doğrusal enterpolasyonla bulunan çeyreklere karşılık gelir; diğer çeyrek tanımları 0,0379 ile 0,0423 arası değerler verir (Tablo E.5, not).

**Çalıştırma:** `uncertainty run library/chemistry/iso13528-e3-atrazine-algorithm-a.json --lang tr`

### İçme suyunda atrazin: medyan ve nIQR (ISO 13528 E.3)

`iso13528-e3-atrazine-median` — ISO 13528:2022, E.3, Table E.5

ISO 13528:2022 E.3, Tablo E.5'in ilk satırı: atanan değer olarak medyan, u(x_pt) = 1,25 nIQR/√p ve σ_pt olarak nIQR.

**Yaklaşım:** yeterlilik testi istatistikleri (ISO 13528)

**Adımlar:**

1. 34 katılımcı içme suyunda atrazin sonucu bildirmiştir (Tablo E.4, değere göre sıralı). İki çok düşük (0,040 ve 0,055 mg/L) ve bir yüksek (0,4246 mg/L) sonuç çoğunluktan çok uzaktır.
2. C.2'nin basit robust tahmin edicileri: medyan 0,2620 mg/L, MADe = 1,483 × medyan mutlak sapma = 0,0386 mg/L ve nIQR = 0,7413 (Q₃ − Q₁) = 0,0402 mg/L.
3. Atanan değer medyan olduğunda u(x_pt) = 1,25 × 0,0402/√34 = 0,0086 mg/L olur; bu, Algoritma A ile bulunan değerle (0,0085 mg/L) pratik olarak aynıdır; nIQR'nin kırılma noktası %25, MADe'ninki %50'dir (C.2.3, Not 1).

**Sonuç:**

- x_pt = 0,262 mg/L, U(x_pt) = 0,017 mg/L (k = 2,00)

**Çalıştırma:** `uncertainty run library/chemistry/iso13528-e3-atrazine-median.json --lang tr`

### Hayvan yeminde cıva: z, z′, ζ ve E_n skorları (ISO 13528 E.4)

`iso13528-e4-mercury-feed` — ISO 13528:2022, E.4, Tables E.6 and E.7

ISO 13528:2022 E.4 (IMEP 111): bir referans değere ve amaca uygun σ_pt'ye karşı 21 laboratuvarın, bildirdikleri belirsizlikler kullanılarak hesaplanan skorları.

**Yaklaşım:** yeterlilik testi istatistikleri (ISO 13528)

**Adımlar:**

1. x_pt = 0,044 mg/kg, U(x_pt) = 0,0082 mg/kg (k = 2, karakterizasyon ve homojensizlik) olan bir referans değerdir; σ_pt = 0,0066 mg/kg (%15) amaca uygunluk değeridir ve PA skorları için δ_E = 3σ_pt'dir. 'Küçüktür' olarak bildirilen üç sonuç skorlanmaz.
2. Laboratuvarların standart belirsizlikleri, genişletilmiş belirsizliklerinin kapsam faktörlerine bölünmesiyle bulunur (L23: k = 1,732); yuvarlanmamış değerler kullanılır.
3. z = D/σ_pt, z′ = D/√(σ_pt² + u²(x_pt)), ζ = D/√(u²(x) + u²(x_pt)) ve E_n = D/√(U²(x) + U²(x_pt)). Burada u(x_pt) = 0,0041 > 0,3σ_pt = 0,0020 olduğundan atanan değerin belirsizliği ihmal edilemez ve z′, z'den yaklaşık %15 küçüktür.
4. Sonuçlar yönteme göre iki tepelidir: doğrudan cıva analizörü (AMA) kullanan sekiz laboratuvar ile AAS kullanan bir laboratuvar referans değerin %30 ile %54'ünü bulur ve eylem sinyali alır (|z| ≥ 3); CV-AAS, HG-AAS veya ICP-MS kullanan laboratuvarların hepsi kabul edilebilir sonuç verir.

**Sonuç:**

- x_pt = 0,0440 mg/kg, U(x_pt) = 0,0082 mg/kg (k = 2,00)

**Notlar:**

- Tablo E.6, bildirilen belirsizlikleri '9.8'de tartışılan ölçütlere göre' a/b/c olarak işaretler; ancak bayraklar u_min = u(x_pt) = 0,0041 mg/kg (9.8.3) ile yeniden üretilemez: L05 (u = 0,0035) ve L09 (u = 0,0040) 'a' olarak işaretlenmiştir. Bayraklar 0,0018 ile 0,0035 mg/kg arasında bir alt sınıra (belki homojensizlik içermeyen karakterizasyon belirsizliği) ve 0,005 ile 0,015 mg/kg arasında bir üst sınıra (1,5σ_pt = 0,0099 ile tutarlı) işaret eder. Bu nedenle bayraklar burada kontrol edilmemiştir.
- ISO 13528 E.7 bu turu referans değeri uzlaşma değeriyle karşılaştırmak için kullanır: 21 sayısal sonuca Algoritma A x* = 0,03161 ve s* = 0,0164 mg/kg verir (yeniden üretildi), u(x*) = 1,25 s*/√21 = 0,0045, u_fark = √(0,0041² + 0,0045²) = 0,0061 mg/kg ve x_ref − x* = 0,012 mg/kg, u_fark'ın yaklaşık iki katıdır (7.8, Formül 7).

**Çalıştırma:** `uncertainty run library/chemistry/iso13528-e4-mercury-feed.json --lang tr`

### CRM ile karşılaştırmadan atanan değer: agregaların Los Angeles değeri (ISO 13528 E.5)

`iso13528-e5-los-angeles-reference-value` — ISO 13528:2022, Annex E.5 (Table E.8)

ISO 13528:2022 E.5 (7.5): yeterlilik testi numunesi ve benzer bir CRM 20 numunede ikişer kez test edilir; x_pt sertifikalı değer artı ortalama farktır, u(x_pt) CRM belirsizliğini ve farkın standart belirsizliğini birleştirir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. CRM'nin sertifikalı değeri 21,62 LA birimi, standart belirsizliği 0,26 LA birimi; bu belirsizlik homojensizlik, taşıma ve uzun süreli kararlılığı zaten içerir.
2. 20 numune için ikili ortalamaların farkları, YT numunesi − CRM (Tablo E.8): ortalama d = 1,727, s = 1,071, u(d) = s/√20 = 0,239 LA birimi,
3. x_pt = 21,62 + 1,73 = 23,35 LA birimi ve u(x_pt) = √(0,26² + 0,239²) = 0,35 LA birimi,

**Sonuç:**

- x_pt = (23,35 ± 0,71) LA units; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Farklar Tablo E.8'deki ikili sonuçlardan yeniden hesaplanır ve tablonun son sütunuyla uyumludur. Monte Carlo yöntemi d için 19 serbestlik dereceli ölçeklenmiş ve kaydırılmış t dağılımını kullanır (JCGM 101 6.4.9).

**Çalıştırma:** `uncertainty run library/chemistry/iso13528-e5-los-angeles-reference-value.json --lang tr`

### Azot ve yağdan et içeriği (ISO 21748 C.2)

`iso21748-c2-meat-content` — ISO 21748:2017, Annex C.2, Table C.1

ISO 21748:2017 C.2: girdilerinin her biri tekrarüretilebilirlik tabanlı belirsizlik taşıyan birleşik bir sonuç, w_meat = 100 w_N/f_N + w_fat; bağıl hata terimleri olan bir ölçüm modeli olarak yazılmıştır.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. w_meat = w_pro + w_fat, w_pro = 100 w_N/f_N (Formül C.1, C.2); w_N = %3,29 (Kjeldahl, ikili ölçüm ortalaması), f_N = 3,65 ± 0,052, w_fat = %5,50.
2. Azot sonucu laboratuvarlar arası s_L = %1,1 terimini ve ikili ölçüm için √2'ye bölünmüş tek sonuç tekrarlanabilirliğini s_r = %1,8 taşır (ISO 21748 Tablo 1): e_L ~ N(0; 0,011²), e_r ~ N(0; (0,018/√2)²); dolayısıyla u(w_N)/w_N = %1,7 (Formül C.3). Yağ s_R = %2,0 taşır.
3. Yayılım yasası w_meat = %95,6, u = %1,99 ve U = %4,0 (k = 2) verir; varyansın %99,7'si w_pro'dan gelir.

**Sonuç:**

- w_meat = (95,6 ± 4,0) %; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Tablo C.1 bağıl belirsizlikleri (0,017; 0,014; 0,022) birleştirmeden önce yuvarlar ve u = %1,98 basar; yuvarlanmamış terimlerle u = %1,99'dur. İkisi de U = %4,0 verir.

**Çalıştırma:** `uncertainty run library/chemistry/iso21748-c2-meat-content.json --lang tr`

### Kömürün kükürt içeriği: tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2 Örnek C.1)

`iso5725-2-c1-sulfur-in-coal` — ISO 5725-2:2019, Annex C.1 (Tables C.1–C.5, Formulae C.1–C.24)

ISO 5725-2:2019 C.1: dört düzeyde (%0,69 – %3,25) sekiz laboratuvarlı, eksik veya aykırı veri içermeyen, hücre başına sonuç sayıları eşit olmayan tek düzeyli bir deney.

**Yaklaşım:** laboratuvarlar arası deneyden tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2)

**Adımlar:**

1. Form A (Tablo C.1): sekiz laboratuvar, dört düzey; 1. ve 5. laboratuvarlar üç yerine dört veya beş sonuç bildirmiştir. Eşit olmayan sonuç sayılarıyla hesaplamayı göstermek için tüm sonuçlar tutulur (C.1.2, Not).
2. Form B ve C (Tablo C.2, C.3): hücre ortalamaları ve standart sapmaları, sonuçlardan bir fazla olmak üzere üç ondalıkla kaydedilir (8.2.10, 8.2.11); standart bu yuvarlanmış değerlerle çalışır (C.1.6).
3. Cochran testi (Formül C.1–C.4), hücrelerin çoğunluğundaki sonuç sayısı n = 3 ile: C = 0,341, 0,289, 0,580 ve 0,311; kritik değerler 0,516 (%5) ve 0,615 (%1); 3. düzeyde 5. laboratuvarın hücresi şüphelidir (straggler) ve tutulur.
4. Hücre ortalamalarına Grubbs testleri (Tablo C.4): tekli şüpheli veya aykırı değer yok; 2. düzeyde 3. ve 6. laboratuvarlar için ikili-yüksek istatistik 0,108, %5 kritik değeri 0,1101'in altındadır: şüpheli, tutulur.
5. Eşit olmayan n_ij ile Formül (23)–(31) (C.14–C.24): 1. düzeyde m̂ = %0,6904, s_r = %0,0152, s_L² = 0,000 460 5 ve s_R = %0,0263.

**Sonuç:**

- Düzey 1: m̂ = 0,69044 %, s_r = 0,01524 %, s_R = 0,02632 % (p = 8)
- Düzey 2: m̂ = 1,2523 %, s_r = 0,02893 %, s_R = 0,06060 % (p = 8)
- Düzey 3: m̂ = 1,6674 %, s_r = 0,01729 %, s_R = 0,03498 % (p = 8)
- Düzey 4: m̂ = 3,2497 %, s_r = 0,02613 %, s_R = 0,05827 % (p = 8)

**Notlar:**

- C.1.5 metni 2. ve 4. düzeylerdeki ikili-yüksek sonuçların şüpheli olduğunu söyler; ancak 4. düzey için Tablo C.4 %5 kritik değeri 0,1101'in üstünde 0,132 verir; yalnızca 2. düzeyde şüpheli bir çift vardır (araştırma günlüğü F47). Aynı metin ISO 5725-2:1994 B.1.5'te de bulunur.
- Formül (C.2)–(C.4) her satırda iki farklı kareler toplamı basar (ör. 0,006 36 ve 0,006 390); test değerleri ikincisinden çıkar.
- C.1.8 tüm aralık için s_r = %0,022 ve s_R = %0,045 verir: dört düzeyin aritmetik ortalamaları.

**Çalıştırma:** `uncertainty run library/chemistry/iso5725-2-c1-sulfur-in-coal.json --lang tr`

### Kömürün kükürt içeriği: REML tahminleri (ISO 5725-2 Örnek C.1)

`iso5725-2-c1-sulfur-in-coal-reml` — ISO 5725-2:2019, Annex C.1.9 (Table C.6), Annex B.2

ISO 5725-2:2019 C.1.9: Örnek C.1 verilerinin Formül (23)–(31) yerine kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi.

**Yaklaşım:** laboratuvarlar arası deneyden tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2)

**Adımlar:**

1. Form A (Tablo C.1): sekiz laboratuvar, dört düzey; 1. ve 5. laboratuvarlar üç yerine dört veya beş sonuç bildirmiştir. Eşit olmayan sonuç sayılarıyla hesaplamayı göstermek için tüm sonuçlar tutulur (C.1.2, Not).
2. REML (B.2), genel ortalaması çıkarılmış verinin olabilirliğini en büyükler; σ_r² ve σ_L² tek yönlü rastgele etkiler modelinin varyans bileşenleridir. Eşit olmayan n_ij ile genel ortalama w_i = 1/(s_L² + s_r²/n_i) ağırlıklarıyla alınır (Formül B.5, B.6).
3. Tahminler (Tablo C.6) Tablo C.5'ten üçüncü ondalıkta en çok bir birim farklıdır: önemli bir fark yoktur.

**Sonuç:**

- Düzey 1: m̂ = 0,68976 %, s_r = 0,01514 %, s_R = 0,02707 % (p = 8)
- Düzey 2: m̂ = 1,2543 %, s_r = 0,02880 %, s_R = 0,06155 % (p = 8)
- Düzey 3: m̂ = 1,6680 %, s_r = 0,01709 %, s_R = 0,03559 % (p = 8)
- Düzey 4: m̂ = 3,2527 %, s_r = 0,02610 %, s_R = 0,05983 % (p = 8)

**Çalıştırma:** `uncertainty run library/chemistry/iso5725-2-c1-sulfur-in-coal-reml.json --lang tr`

### Kreozot yağının termometrik titrasyonu: aykırı laboratuvar ve kesinlik ilişkileri (ISO 5725-2 Örnek C.3)

`iso5725-2-c3-creosote-oil` — ISO 5725-2:2019, Annex C.3 (Tables C.14–C.18) and 8.5 (Tables 1–4)

ISO 5725-2:2019 C.3 ve 8.5: beş düzeyde (%4 – %20) dokuz laboratuvar, aykırı bir laboratuvar, reddedilmiş bir hücre ve kesinlik ile düzey arasındaki I–IV fonksiyonel ilişkileri (Tablo 1–4).

**Yaklaşım:** laboratuvarlar arası deneyden tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2)

**Adımlar:**

1. Form A (Tablo C.14): dokuz laboratuvar, beş düzey, hücre başına iki sonuç.
2. Mandel h (Şekil C.7) 1. laboratuvarın tüm düzeylerde diğerlerinden çok yüksek olduğunu gösterir; Mandel k (Şekil C.8) 6. ve 7. laboratuvarlarda oldukça büyük hücre içi farklar gösterir.
3. Cochran testi: 4. düzeyde C = 1,10²/1,8149 = 0,667 (şüpheli, p = 9 için 0,638 < C ≤ 0,754); 5. düzeyde C = 1,98²/6,1663 = 0,636, 0,638'in hemen altında.
4. Grubbs testleri (Tablo C.17): 1. laboratuvarın 3. ve 4. düzeylerdeki hücre ortalamaları aykırıdır (2,50 ve 2,47 > 2,387); bu yüzden orada ikili testler uygulanmaz (8.3.5.3).
5. Panel 1. laboratuvarı aykırı laboratuvar olarak ve 6. laboratuvarın 5. düzeydeki hücresini reddeder; bunlar olmadan 4. düzeydeki Cochran istatistiği 0,680 (p = 8) ile karşılaştırılır ve artık şüpheli değildir.
6. Tablo C.18: p = 8 (5. düzeyde p = 7) ile kesinlik değerleri düzeyle artar: s_r %0,092'den %0,393'e, s_R %0,171'den %0,637'ye.
7. s_r için ilişkiler (Tablo 1–4): I, b = Σ(s/m)/q = 0,019; II, iki ağırlıklı uydurmadan sonra s = 0,030 + 0,0154m; III, s² = 0,061² + (0,0178m)²; IV, lg s = −1,5065 + 0,77 lg m. Tekrarlanabilirlik için orijinden geçen bir doğru yeterli görünmektedir (C.3.7).

**Sonuç:**

- Düzey 1: m̂ = 3,9406 %, s_r = 0,09216 %, s_R = 0,1708 % (p = 8)
- Düzey 2: m̂ = 8,2819 %, s_r = 0,1789 %, s_R = 0,4977 % (p = 8)
- Düzey 3: m̂ = 14,178 %, s_r = 0,1269 %, s_R = 0,4004 % (p = 8)
- Düzey 4: m̂ = 15,588 %, s_r = 0,3368 %, s_R = 0,5786 % (p = 8)
- Düzey 5: m̂ = 20,412 %, s_r = 0,3935 %, s_R = 0,6370 % (p = 7)

**Notlar:**

- Tablo 4'te basılı IV. ilişki eğimi d = 0,772, aynı tablodaki uydurulmuş değerlerle (0,089; 0,158; 0,239; 0,257; 0,316) tutarlı değildir; bu değerler tablodaki logaritmaların regresyonundan, d = 0,769'dan çıkar (araştırma günlüğü F49). Aynı değerler ISO 5725-2:1994'te de vardır.
- Tablo 2 ikinci uydurmanın eğimini 0,0154'e yuvarlar; tablodaki m ve s değerlerinden 0,01554 çıkar. Program Tablo C.18'in yuvarlanmamış s_r değerlerine uydurur.

**Çalıştırma:** `uncertainty run library/chemistry/iso5725-2-c3-creosote-oil.json --lang tr`

### Kreozot yağının termometrik titrasyonu: REML tahminleri (ISO 5725-2 Örnek C.3)

`iso5725-2-c3-creosote-oil-reml` — ISO 5725-2:2019, Annex C.3.8 (Table C.19), Annex B.2

ISO 5725-2:2019 C.3.8: Örnek C.3'ün tutulan verilerinin kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi.

**Yaklaşım:** laboratuvarlar arası deneyden tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2)

**Adımlar:**

1. Form A (Tablo C.14): dokuz laboratuvar, beş düzey, hücre başına iki sonuç.
2. Mandel h (Şekil C.7) 1. laboratuvarın tüm düzeylerde diğerlerinden çok yüksek olduğunu gösterir; Mandel k (Şekil C.8) 6. ve 7. laboratuvarlarda oldukça büyük hücre içi farklar gösterir.
3. Cochran testi: 4. düzeyde C = 1,10²/1,8149 = 0,667 (şüpheli, p = 9 için 0,638 < C ≤ 0,754); 5. düzeyde C = 1,98²/6,1663 = 0,636, 0,638'in hemen altında.
4. Grubbs testleri (Tablo C.17): 1. laboratuvarın 3. ve 4. düzeylerdeki hücre ortalamaları aykırıdır (2,50 ve 2,47 > 2,387); bu yüzden orada ikili testler uygulanmaz (8.3.5.3).
5. Panel 1. laboratuvarı aykırı laboratuvar olarak ve 6. laboratuvarın 5. düzeydeki hücresini reddeder; bunlar olmadan 4. düzeydeki Cochran istatistiği 0,680 (p = 8) ile karşılaştırılır ve artık şüpheli değildir.
6. Tablo C.19'un REML tahminleri basılı basamaklarda Tablo C.18 ile aynıdır.

**Sonuç:**

- Düzey 1: m̂ = 3,9406 %, s_r = 0,09216 %, s_R = 0,1708 % (p = 8)
- Düzey 2: m̂ = 8,2819 %, s_r = 0,1789 %, s_R = 0,4977 % (p = 8)
- Düzey 3: m̂ = 14,178 %, s_r = 0,1269 %, s_R = 0,4004 % (p = 8)
- Düzey 4: m̂ = 15,588 %, s_r = 0,3368 %, s_R = 0,5786 % (p = 8)
- Düzey 5: m̂ = 20,412 %, s_r = 0,3935 %, s_R = 0,6370 % (p = 7)

**Çalıştırma:** `uncertainty run library/chemistry/iso5725-2-c3-creosote-oil-reml.json --lang tr`

### N₂ içinde NO, UV absorpsiyonu: doğrusal analiz fonksiyonu (ISO 6143 Örnek 1)

`iso6143-b1-no-uv-linear` — ISO 6143:2025, Annex B.2.2 (Tables B.1–B.6)

ISO 6143:2025 B.2.2: hem içeriklerinde hem de UV yanıtlarında belirsizlik bulunan üç referans gaz karışımı (5 µmol/mol – 50 µmol/mol), genelleştirilmiş en küçük karelerle doğrusal analiz fonksiyonu ve üç karışımın içerikleri ile kovaryansları.

**Yaklaşım:** her iki değişkende belirsizlikli kalibrasyon (ISO 6143)

**Adımlar:**

1. ISO 6143 6.1 adım A–G: yanıtlar on ölçümün ortalamaları, u(y) ortalamanın standart sapmalarıdır; x'in bağıl belirsizliği %1, y'ninki %2'dir.
2. Adım H: x = b₀ + b₁y, S = Σ[(x̂ᵢ − xᵢ)²/u²(xᵢ) + (ŷᵢ − yᵢ)²/u²(yᵢ)] toplamı b ve düzeltilmiş yanıtlar ŷᵢ üzerinden en küçüklenerek uydurulur (Ek A): b₀ = −0,3575 µmol/mol, b₁ = 24,612 µmol/mol.
3. Doğrulama (6.2.2): S_res = 0,674, kabul sınırı 2(n − N − 1) = 2 ile karşılaştırılır; en büyük ağırlıklı sapma Γ = 0,568 ≤ 2: doğru, verilerle uyumludur.
4. Adım I–K (6.3): her karışım için x = G(y) ve u²(x) = (∂G/∂y)²u²(y) + Σ (∂G/∂bⱼ)(∂G/∂bₗ)u(bⱼ, bₗ); üç sonuç ortak kalibrasyon nedeniyle koreledir.

**Sonuç:**

- x(S1) = (5,99 ± 0,33) µmol/mol (k = 2,00)
- x(S2) = (14,41 ± 0,71) µmol/mol (k = 2,00)
- x(S3) = (43,9 ± 2,3) µmol/mol (k = 2,00)

**Notlar:**

- B.2.2'de basılan üç sonucun kovaryansları (1,16 × 10⁻², 1,48 × 10⁻², 1,37 × 10⁻¹ (µmol/mol)²) yeniden üretilir; rapor bunları korelasyon katsayıları olarak verir.

**Çalıştırma:** `uncertainty run library/chemistry/iso6143-b1-no-uv-linear.json --lang tr`

### Doğal gazda N₂, GC-TCD: korelasyonlu referans karışımlar (ISO 6143 Örnek 2)

`iso6143-b2-n2-natural-gas-covariances` — ISO 6143:2025, Annex B.2.3 (Table B.10)

ISO 6143:2025 B.2.3: yine doğru; bu kez geçmişi ortak olan referans karışımların (bir seyreltme serisi ve ortak bir birincil standart seti) kovaryanslarıyla.

**Yaklaşım:** her iki değişkende belirsizlikli kalibrasyon (ISO 6143)

**Adımlar:**

1. Sekiz kalibrasyon noktası: bir kör (M1, x = %0,0015) ve yedi referans gaz karışımı; sentetik doğal gazda N₂, ısıl iletkenlik dedektörlü gaz kromatografisiyle (pik alanları y).
2. M7, M4'ten seyreltilmiştir (yaklaşık 3 kat): u(x₄, x₇) = 0,00016 %²; M5 ve M8 ortak bir kalibrasyonla sertifikalandırılmıştır: u(x₅, x₈) = 0,0001 %².
3. Kovaryanslar yalnızca yayılıma girer (Formül A.7): parametreler değişmez, u(b₁) 2,416 × 10⁻⁸'den 2,550 × 10⁻⁸'e, iki karışımın u(x) değerleri %3–4 artar.

**Sonuç:**

- x(S1) = (1,7004 ± 0,0042) % (k = 2,00)
- x(S2) = (8,986 ± 0,021) % (k = 2,00)

**Çalıştırma:** `uncertainty run library/chemistry/iso6143-b2-n2-natural-gas-covariances.json --lang tr`

### Doğal gazda N₂, GC-TCD: doğrusal fonksiyon (ISO 6143 Örnek 2)

`iso6143-b2-n2-natural-gas-linear` — ISO 6143:2025, Annex B.2.3 (Tables B.7, B.8)

ISO 6143:2025 B.2.3: her iki değişkende belirsizlikli sekiz kalibrasyon noktasından genelleştirilmiş en küçük kareler doğrusu, diğer analiz fonksiyonlarıyla karşılaştırma ve iki karışımın içerikleri.

**Yaklaşım:** her iki değişkende belirsizlikli kalibrasyon (ISO 6143)

**Adımlar:**

1. Sekiz kalibrasyon noktası: bir kör (M1, x = %0,0015) ve yedi referans gaz karışımı; sentetik doğal gazda N₂, ısıl iletkenlik dedektörlü gaz kromatografisiyle (pik alanları y).
2. Doğru: b₀ = 3,981 × 10⁻⁴ %, b₁ = 2,4285 × 10⁻⁵ %/alan; S_res = 6,05 (sınır 2(n − N − 1) = 12) ve Γ = 1,63 ≤ 2: kabul edilebilir.
3. Karşılaştırma tablosu ikinci derece polinomun daha iyi uyduğunu gösterir (Γ = 0,87); ISO 6143 6.2.2 b), fiziksel bir model daha fazlasını gerektirmiyorsa kabul edilebilir en basit fonksiyonu önerir.

**Sonuç:**

- x(S1) = (1,7004 ± 0,0040) % (k = 2,00)
- x(S2) = (8,986 ± 0,020) % (k = 2,00)

**Çalıştırma:** `uncertainty run library/chemistry/iso6143-b2-n2-natural-gas-linear.json --lang tr`

### Doğal gazda N₂, GC-TCD: ikinci derece polinom (ISO 6143 Örnek 2)

`iso6143-b2-n2-natural-gas-quadratic` — ISO 6143:2025, Annex B.2.3 (Table B.9)

ISO 6143:2025 B.2.3: aynı kalibrasyon verileri, ikinci derece polinom analiz fonksiyonuyla.

**Yaklaşım:** her iki değişkende belirsizlikli kalibrasyon (ISO 6143)

**Adımlar:**

1. Sekiz kalibrasyon noktası: bir kör (M1, x = %0,0015) ve yedi referans gaz karışımı; sentetik doğal gazda N₂, ısıl iletkenlik dedektörlü gaz kromatografisiyle (pik alanları y).
2. x = b₀ + b₁y + b₂y²: b₂ = −4,09 × 10⁻¹³, u(b₂) = 1,9 × 10⁻¹³; S_res = 1,40 (sınır 10) ve Γ = 0,87.
3. Birinci derece katsayısı %0,5'ten az değişir ve iki karışımın içerikleri belirsizlikleri içinde doğrusal sonuçlarla uyumludur; standart seçimi 6.2.2'ye bırakır (kabul edilebilir en basit fonksiyon).

**Sonuç:**

- x(S1) = (1,7059 ± 0,0066) % (k = 2,00)
- x(S2) = (8,972 ± 0,024) % (k = 2,00)

**Çalıştırma:** `uncertainty run library/chemistry/iso6143-b2-n2-natural-gas-quadratic.json --lang tr`

### N₂ içinde NO, NDIR: üstel fonksiyon (ISO 6143 Örnek 3)

`iso6143-b3-no-ndir-exponential` — ISO 6143:2025, Annex B.1 and B.2.4 (Tables B.11, B.12)

ISO 6143:2025 B.2.4: aynı NDIR kalibrasyonu, standardın tercih ettiği x = b₀ + b₁e^(b₂y) üstel analiz fonksiyonuyla; orada basılan parametre belirsizliklerinin yanlış olduğu bilinmektedir.

**Yaklaşım:** her iki değişkende belirsizlikli kalibrasyon (ISO 6143)

**Adımlar:**

1. N₂ içinde 1 µmol/mol – 10 µmol/mol NO içeren on iki referans gaz karışımı, dağılmasız kızılötesi absorpsiyonla ölçülmüştür; yanıt belirgin biçimde doğrusal değildir: doğru S_res = 273 (sınır 20) ve Γ = 6,84 > 2 verir.
2. Üstel fonksiyon: b₀ = −47,96, b₁ = 47,97, b₂ = 2,128 × 10⁻⁵ (b₀ ile b₁ neredeyse birbirini götürür: korelasyonları −1,000'dir); S_res = 0,657 (sınır 18) ve Γ = 0,353; standardın denediği üç parametreli fonksiyonların en iyi uyumu (karşılaştırma tablosu standardın denemediği ikinci derece polinomu da ekler, Γ = 0,44).
3. y = 4950,6 ± 11 olan karışım: x = 5,3357 µmol/mol, u(x) = 0,0142 µmol/mol; üslü fonksiyon 5,3456 µmol/mol verir, bir standart belirsizlik içinde.

**Sonuç:**

- x(S1) = (5,336 ± 0,028) µmol/mol (k = 2,00)

**Notlar:**

- ISO 6143 B.1 ve B.2.4, B_LEAST'in üstel fonksiyon için yanlış parametre belirsizlikleri verdiğini belirtir (METAS'ta bulunmuştur). Basılan u(b) = 8,14 × 10⁻², 7,40 × 10⁻², 3,71 × 10⁻¹¹ değerleri yaklaşık 39 kat küçüktür; bu program 3,18, 3,17 ve 1,27 × 10⁻⁶ verir; ODRPACK (scipy.odr) ve kalibrasyonun Monte Carlo benzetimiyle (3,13, 3,12, 1,24 × 10⁻⁶) uyumludur.
- Tablo B.12 üstel fonksiyon için u(x) = 0,01328 µmol/mol verir; doğru kovaryanslarla birinci mertebe yayılım 0,01425, Monte Carlo benzetimi 0,01406 µmol/mol verir: basılı değer %6–7 küçüktür; bu, standardın iterasyon ayarları için öngördüğü 'yaklaşık %3'ten fazladır (araştırma günlüğü F42). Proje parametreleri ve x'i standarda, u(x)'i bağımsız hesaplara göre kontrol eder.

**Çalıştırma:** `uncertainty run library/chemistry/iso6143-b3-no-ndir-exponential.json --lang tr`

### N₂ içinde NO, NDIR: üslü fonksiyon (ISO 6143 Örnek 3)

`iso6143-b3-no-ndir-power` — ISO 6143:2025, Annex B.2.4 (Tables B.11, B.12)

ISO 6143:2025 B.2.4: on iki noktalı doğrusal olmayan bir NDIR kalibrasyonu; x = b₀ + b₁y^(1+b₂) üslü analiz fonksiyonu genelleştirilmiş en küçük karelerle uydurulur.

**Yaklaşım:** her iki değişkende belirsizlikli kalibrasyon (ISO 6143)

**Adımlar:**

1. N₂ içinde 1 µmol/mol – 10 µmol/mol NO içeren on iki referans gaz karışımı, dağılmasız kızılötesi absorpsiyonla ölçülmüştür; yanıt belirgin biçimde doğrusal değildir: doğru S_res = 273 (sınır 20) ve Γ = 6,84 > 2 verir.
2. Üslü fonksiyon: b₂ = 0,0850 ± 0,0051; S_res = 8,38 (sınır 18) ve Γ = 1,16 ≤ 2: kabul edilebilir.
3. y = 4950,6 ± 11 olan karışım: x = 5,3456 µmol/mol, u(x) = 0,0142 µmol/mol.

**Sonuç:**

- x(S1) = (5,346 ± 0,028) µmol/mol (k = 2,00)

**Notlar:**

- Bu doğrusal olmayan fonksiyon için parametre kovaryansları, en küçük koşulunun kapalı türevlenmesiyle, yani A.2'deki sayısal türevlemenin tam biçimiyle hesaplanır; B_LEAST ile %0,3 içinde uyumludur. Tüm kalibrasyonun Monte Carlo benzetimi (4000 yeniden uydurma, araştırma günlüğü F42) de u(x) = 0,0142 µmol/mol verir.

**Çalıştırma:** `uncertainty run library/chemistry/iso6143-b3-no-ndir-power.json --lang tr`

### Karbondioksitin molekül ağırlığı (NIST TN 1900 E8)

`nist-tn1900-e8-molecular-weight-co2` — NIST Technical Note 1900, Example E8, Exhibit 12

TN 1900 E8: IUPAC aralık atom ağırlıklarından M_r(CO₂) = A_r(C) + 2A_r(O); çıktı dağılımı yamuktur ve GUM aralığı gereğinden geniştir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Standart atom ağırlıkları aralıktır: A_r(C) ∈ [12,0096; 12,0116] ve A_r(O) ∈ [15,99903; 15,99977] (Wieser ve ark. 2013). Her biri kendi aralığı üzerinde dikdörtgen dağılımla modellenir.
2. Tahmin 12,0106 + 2 × 15,9994 = 44,0094'tür ve u² = u(A_r(C))² + 4u(A_r(O))², u = 0,00072 olur.
3. İki dikdörtgen değişkenin toplamı, yarı genişlikleri 0,00174 (taban) ve 0,00026 (üst) olan simetrik yamuk dağılıma sahiptir. En kısa %95 aralığı 44,0094 ± 0,00136 = [44,0080; 44,0108] olup GUM aralığı ± 1,96u = ± 0,00141'den dardır; Monte Carlo yöntemi tam aralığı yeniden üretir.

**Sonuç:**

- M_r = (44,0094 ± 0,0014); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- TN 1900, yuvarlanmış 0,0006 ve 0,0002 bileşenlerinden u = 0,000721 basar; yuvarlanmamış 0,000577 ve 0,000214 bileşenleri 0,000718 verir. Tam aralığın uç noktaları yamuk dağılımdan çıkar: 44,0094 ± (0,00174 − √(0,2 × 0,001 × 0,00074)) = [44,008045; 44,010755].

**Çalıştırma:** `uncertainty run library/chemistry/nist-tn1900-e8-molecular-weight-co2.json --lang tr`

### Kadmiyum kalibrasyon standardı, Monte Carlo (NIST TN 1900 E9)

`nist-tn1900-e9-cadmium-standard` — NIST Technical Note 1900, Example E9

TN 1900 E9: QUAM A1 kadmiyum standardı, c = 1000 m P / V, dikdörtgen saflık ve üçgen hacim dağılımıyla Monte Carlo yöntemiyle yeniden değerlendirilir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. m = 100,28 mg (u = 0,05 mg, normal), P = 0,9999 ([0,9998; 1] üzerinde dikdörtgen, u = 0,000058) ve V = 100,0 mL (üçgen, u = 0,07 mL); c = 1002,7 mg/L.
2. Çarpım ve bölüm için bağıl belirsizlikler kareler toplamı olarak birleşir (GUM 5.1.6): u(c)/c = √((0,05/100,28)² + (0,000058/0,9999)² + (0,07/100)²), u = 0,86 mg/L.
3. Monte Carlo yöntemi aynı standart belirsizliği ve TN 1900'de basılı basamaklara kadar, tüm girdiler normal dağılımlı alındığındakiyle aynı %95 aralığını, [1001,0; 1004,4] mg/L, verir. Üçgen ve dikdörtgen girdiler çıktının kuyruklarını biraz daraltır: Monte Carlo uç noktaları GUM aralığının yaklaşık 0,02–0,03 mg/L içindedir; bu nedenle GUM sonucu u'nun iki anlamlı basamağı için (δ = 0,005 mg/L) doğrulanmaz, ancak farkın pratik bir önemi yoktur.

**Sonuç:**

- c_Cd = (1002,7 ± 1,7) mg/L; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- TN 1900, V'yi 'ortalaması 100 mL ve standart sapması 0,7 mL olan' simetrik üçgen dağılım olarak tanımlar; basılı aralıkla yalnızca aynı örnekte ve QUAM A1'de kullanılan 0,07 mL tutarlıdır. Üçgen yarı genişliği 0,07 × √6 = 0,1715 mL'dir.
- 'quam-a1-cadmium-standard' projesi m, P ve V'nin belirsizliklerini QUAM A1'deki gibi bileşenlerinden oluşturur.

**Çalıştırma:** `uncertainty run library/chemistry/nist-tn1900-e9-cadmium-standard.json --lang tr`

### Kadmiyum kalibrasyon standardının hazırlanması

`quam-a1-cadmium-standard` — EURACHEM/CITAC Guide CG 4, Example A1, Tables A1.2 and A1.3

EURACHEM/CITAC QUAM Örnek A1: yüksek saflıkta metalden 1000 mg/L Cd standardı (özet bütçe).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. c_Cd = 1000 · m · P / V: saflığı 0,9999 olan 100,28 mg Cd çözülüp 100 mL'ye tamamlanır.
2. Saflık güven düzeyi belirtilmeden sınırlar olarak verilmiştir: dikdörtgen dağılım, u(P) = 0,0001/√3.
3. 100 mL'lik balonun hacmi kalibrasyon, tekrarlanabilirlik ve sıcaklık etkilerini birleştirerek u(V) = 0,07 mL verir (hacim alt modeli içeren projeye bakınız).
4. Baskın katkı hacimdir; u_c = 0,86 mg/L ve U = 2 × 0,86 ≈ 1,7 mg/L. Rehber u_c'yi 0,9'a yuvarlar ve (1002,7 ± 1,8) mg/L olarak raporlar.

**Sonuç:**

- c_Cd = (1002,7 ± 1,7) mg/L; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Rehberin tablolama (Kragten) değeri 0,8633 mg/L ile kesin birinci mertebe değeri 0,8638 mg/L arasındaki fark yalnızca sonlu fark yaklaşımından kaynaklanır; raporlanan sonuç değişmez.
- Yuvarlanmış u_c'nin iki katını almak (0,9 → U = 1,8 mg/L) 2 × 0,86 = 1,7 mg/L'den daha temkinlidir; program U'nun kendisini iki anlamlı basamağa yuvarlar.
- Bu projede etkin olan Kragten tablolama görünümü (QUAM Ek E.2), rehberin basılı tablolama değerini (0,8633 mg/L) basılı tüm basamaklarda yeniden üretir; kesin birinci mertebe değer yalnızca sonlu fark yaklaşımı nedeniyle farklıdır.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a1-cadmium-standard.json --lang tr`

### Hacim alt modelli kadmiyum standardı

`quam-a1-cadmium-standard-volume` — EURACHEM/CITAC Guide CG 4, Example A1, A1.4

Balon hacmi üç bileşeninden modellenmiş QUAM Örnek A1.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. c_Cd = 1000 · m · P / V: saflığı 0,9999 olan 100,28 mg Cd çözülüp 100 mL'ye tamamlanır.
2. Saflık güven düzeyi belirtilmeden sınırlar olarak verilmiştir: dikdörtgen dağılım, u(P) = 0,0001/√3.
3. V = V_nom + δV_cal + δV_rep + δV_temp: kalibrasyon ±0,1 mL (üçgen, 0,04 mL), tekrarlanabilirlik s = 0,02 mL, sıcaklık ±4 °C × 2,1 × 10⁻⁴ °C⁻¹ × 100 mL = ±0,084 mL (dikdörtgen, 0,05 mL) → u(V) = 0,066 mL.
4. Dikdörtgen ve üçgen bileşenler baskın olduğundan çıktı dağılımının kuyrukları normal dağılımdan daha incedir: Monte Carlo aralığı her uçta yaklaşık 0,015 mg/L daha kısadır ve GUM aralığı u'nun yalnızca bir anlamlı basamağı için doğrulanır.

**Sonuç:**

- c_Cd = (1002,7 ± 1,7) mg/L; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Rehber k ile çarpmadan önce u(V) = 0,066 mL'yi 0,07 mL'ye ve u_c'yi 0,9 mg/L'ye yuvarlar; yuvarlanmamış değerlerle u_c = 0,835 mg/L ve U = 1,7 mg/L bulunur, basılı sonuç ise (1002,7 ± 1,8) mg/L'dir.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a1-cadmium-standard-volume.json --lang tr`

### Sodyum hidroksit çözeltisinin ayarlanması

`quam-a2-naoh-standardisation` — EURACHEM/CITAC Guide CG 4, Example A2, Tables A2.2 and A2.3

QUAM Örnek A2: NaOH çözeltisinin potasyum hidrojen ftalata (KHP) karşı ayarlanması; rehberin standart belirsizlikleriyle özet bütçe.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. c_NaOH = 1000 · m_KHP · P_KHP / (M_KHP · V_T) · rep (mol/L); burada rep, bağıl standart belirsizliği %0,05 olan tekrarlanabilirlik çarpanıdır.
2. Çarpım/bölüm modelinde bağıl belirsizlikler kareler toplamının karekökü ile birleşir: u_c/c = √[(0,0005)² + (0,00013/0,3888)² + (0,00029)² + (0,0038/204,2212)² + (0,013/18,64)²].
3. Titrasyon hacmi ve tekrarlanabilirlik baskındır; k = 2 ile c = (0,10214 ± 0,00020) mol/L.

**Sonuç:**

- c_NaOH = (0,10214 ± 0,00020) mol/L; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bu projede etkin olan Kragten tablolama görünümü (QUAM Ek E.2), rehberin basılı tablolama değerini (0,000099 mol/L) basılı tüm basamaklarda yeniden üretir; kesin birinci mertebe değer yalnızca sonlu fark yaklaşımı nedeniyle farklıdır.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a2-naoh-standardisation.json --lang tr`

### Alt modellerle NaOH ayarlanması

`quam-a2-naoh-standardisation-submodels` — EURACHEM/CITAC Guide CG 4, Example A2, A2.4

Kütle, molar kütle ve hacmin bileşenlerinden oluşturulduğu QUAM Örnek A2.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. m_KHP = m_brüt − m_dara; her tartım ±0,15 mg'lık terazi doğrusallık terimi (dikdörtgen) içerir.
2. M_KHP = 8A_C + 5A_H + 4A_O + A_K, IUPAC atom ağırlıklarından; bir elementin atomları tam korelasyonludur ve model bunu tek atom ağırlığını atom sayısıyla çarparak ifade eder.
3. V_T = 18,64 mL + büret kalibrasyonu (±0,03 mL, üçgen) + sıcaklık etkisi (19 mL × 2,1 × 10⁻⁴ °C⁻¹ × 3 °C / 1,96, normal).
4. Sonuç: rehberdeki gibi k = 2 ile (0,1021 ± 0,0002) mol/L.

**Sonuç:**

- c_NaOH = (0,1021 ± 0,0002) mol/L; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu: rehber 0,15/√3 = 0,087 mg değerini 0,09 mg'ye yuvarlayarak u(m) = 0,13 mg bulur; yuvarlanmamış u(m) = 0,12 mg'dir. Benzer şekilde u(V_T) = 0,0137 mL'dir (rehber 0,013). Yuvarlanmamış birleşik belirsizlik 0,000101 mol/L'dir (rehber 0,000099); raporlanan sonuç değişmez.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a2-naoh-standardisation-submodels.json --lang tr`

### Hidroklorik asidin asit–baz titrasyonu

`quam-a3-hcl-titration` — EURACHEM/CITAC Guide CG 4, Example A3, Tables A3.2 and A3.3

QUAM Örnek A3: KHP'ye karşı ayarlanmış NaOH ile HCl titrasyonu.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. c_HCl = 1000 · m_KHP · P_KHP · V_T2 / (V_T1 · M_KHP · V_HCl) · rep: V_T1 ayarlamadaki, V_T2 ise 15 mL HCl'nin titrasyonundaki NaOH hacmidir.
2. İki adımlı işlemin tamamının tekrarlanabilirliği %0,1'dir (rep); en büyük tekil katkı budur.
3. k = 2 ile c = (0,1014 ± 0,0004) mol/L.

**Sonuç:**

- c_HCl = (0,1014 ± 0,0004) mol/L; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu: Özet Tablo A3.1 u_c = 0,00016 mol/L verir, ancak kendi girdileri 0,00019 mol/L sonucunu verir; 0,00016 üçlü tayinin sonucudur (Tablo A3.4).
- Bu projede etkin olan Kragten tablolama görünümü (QUAM Ek E.2), rehberin basılı tablolama değerini (√3,34 × 10⁻⁸ mol/L) basılı tüm basamaklarda yeniden üretir; kesin birinci mertebe değer yalnızca sonlu fark yaklaşımı nedeniyle farklıdır.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a3-hcl-titration.json --lang tr`

### Ekmekte organofosforlu pestisitler: birleştirme

`quam-a4-pesticides-combination` — EURACHEM/CITAC Guide CG 4, Example A4, Tables A4.4 and A4.5

QUAM Örnek A4: laboratuvar içi validasyon verilerinden (kesinlik, geri kazanım, homojenlik) elde edilen bağıl belirsizliğin P = 1'de çarpımsal modelde birleştirilmesi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. P_op = F_I · F_hom / Rec, anma değerlerinde değerlendirilir: kesinlik 0,27 (15 mükerrer ölçümden), geri kazanım 0,90 ve u = 0,043 (t = 2,31 > t_kritik olduğundan düzeltme uygulanır), homojenlik 0,20.
2. Bağıl belirsizlik: √(0,27² + 0,048² + 0,20²) = 0,34; P = 1,1111'de rehberin tablolaması u = 0,377 verir.
3. %20–27 bağıl belirsizliklerle model belirgin biçimde doğrusal değildir: Monte Carlo dağılımı çarpıktır ve standart sapması GUM değerinden farklıdır.

**Sonuç:**

- P_op = (1,11 ± 0,74); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu: Tablo A4.2'nin türetilmiş sütunlarında yazım hataları vardır (0,075 yerine ortalama 0,75, −0,01 yerine fark −0,10, 0,01 yerine ortalama 0,10); rehber 41 serbestlik derecesi için t_kritik = 2,021, yani t(40) kullanır (t(41) = 2,0195). Hiçbiri sonucu değiştirmez.
- Bu projede etkin olan Kragten tablolama görünümü (QUAM Ek E.2), rehberin basılı tablolama değerini (0,377) basılı tüm basamaklarda yeniden üretir; kesin birinci mertebe değer yalnızca sonlu fark yaklaşımı nedeniyle farklıdır.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a4-pesticides-combination.json --lang tr`

### Seramik kaplardan salınan kadmiyum

`quam-a5-cadmium-release` — EURACHEM/CITAC Guide CG 4, Example A5, Tables A5.3 and A5.4

QUAM Örnek A5: ampirik yöntem (BS 6748); birim alan başına çözünen Cd miktarı.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. r = c₀ · V_L / a_V · f_asit · f_süre · f_sıcaklık (mg/dm²): c₀ doğrusal AAS kalibrasyonundan (u(c₀) = 0,018 mg/L, kalibrasyon modülüne bakınız), özütleme hacmi 0,332 L, yüzey alanı 5,73 dm².
2. f çarpanları ampirik yöntemin koşullarının (asit derişimi, süre, sıcaklık) etkilerini temsil eder; ampirik yöntemde tanım gereği 1'dir ancak belirsizlik taşırlar.
3. Kalibrasyon (c₀) ve sıcaklık çarpanı baskındır; k = 2 ile r = (0,015 ± 0,003) mg/dm².

**Sonuç:**

- r = (0,015 ± 0,003) mg/dm²; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu: a_V = πd²/4, d = 2,70 dm (rehberin formülünde 2,77 yazım hatası); bileşenleri 0,042 ve 0,146 dm² olup 0,152 dm² verir, burada da kullanılan basılı 0,19 dm² değil. 0,152 dm² ile u_c = 0,00147 yerine 0,00144 mg/dm² bulunur; raporlanan sonuç değişmez.
- Kesin birinci mertebe u_c 0,001470 mg/dm²'dir; tablolama değeri 0,001465, sonlu farklardaki 1/a_V doğrusalsızlığı nedeniyle farklıdır.
- Bu projede etkin olan Kragten tablolama görünümü (QUAM Ek E.2), rehberin basılı tablolama değerini (0,001465 mg/dm²) basılı tüm basamaklarda yeniden üretir; kesin birinci mertebe değer yalnızca sonlu fark yaklaşımı nedeniyle farklıdır.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a5-cadmium-release.json --lang tr`

### IDMS için kurşun tayin standardı

`quam-a7-assay-standard` — EURACHEM/CITAC Guide CG 4, Example A7, Eq. (8), Table A7.3

QUAM Örnek A7, Eş. (8): gravimetrik olarak hazırlanmış birincil kurşun tayin standardının madde miktarı içeriği.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. c_z = 10⁶ · (m₂/d₂) · m₁ · w / (d₁ · M): saflığı w olan m₁ kütlesindeki kurşun çözülür, kütlece seyreltilir (d₁, d₂) ve m₂ kadar alınır; M tayin malzemesindeki kurşunun molar kütlesidir.
2. Tüm girdiler Tablo A7.3'teki standart belirsizliklerle normal dağılımlıdır; c_z = 0,092605 µmol/g, u = 0,000028 µmol/g.

**Sonuç:**

- c_z = (0,092605 ± 0,000056) µmol/g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a7-assay-standard.json --lang tr`

### Suda kurşunun çift izotop seyreltmesiyle tayini

`quam-a7-lead-double-idms` — EURACHEM/CITAC Guide CG 4, Example A7, Eq. (5), Tables A7.4–A7.8

QUAM Örnek A7, Eş. (5): kütle ayrımı düzeltilmiş izotop oranlarıyla çift IDMS (1. tekrar).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. c_x = c_z · (m_y/m_x) · (m_z/m_y') · (K_y1R_y1 − K_bR_b)/(K_bR_b − K_x1R_x1) · (K_b'R_b' − K_z1R_z1)/(K_y1R_y1 − K_b'R_b') · ΣK_xiR_xi/ΣK_ziR_zi − c_kör; her K = K₀ + K_sapma.
2. Oranların ve K₀'ın A tipi belirsizlikleri √8'e bölünür (sekiz ölçümün ortalaması); K_sapma terimleri (her biri u = 0,001) varyansın yaklaşık %38'ini oluşturur.
3. c_x = 0,053738 µmol/g, u_c = 0,00018 µmol/g.

**Sonuç:**

- c_x = (0,05374 ± 0,00036) µmol/g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Raporlanan (0,05370 ± 0,00036) µmol/g dört tekrarın ortalamasını kullanır; √8'e bölme yapılmasaydı deneysel u_c 0,00041 µmol/g olurdu.

**Çalıştırma:** `uncertainty run library/chemistry/quam-a7-lead-double-idms.json --lang tr`

### X-grafiğinin sınırlarının belirlenmesi, durum 1 (Nordtest TR 569 Bölüm 7)

`tr569-ch7-case1-statistical` — Nordtest TR 569 ed. 6 (2026), Chapter 7, Setting the control limits and the central line in X-charts, case 1

Durum 1: istatistiksel sınırlar ve ortalama merkez çizgisi, 59,2 µg/L'nin %4'ü s_Rw.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. X-grafiği için istatistiksel kontrol sınırları: uyarı sınırları CL ± 2s, eylem sınırları CL ± 3s (Bölüm 7).
2. Göreli standart sapma merkez çizgisine uygulanır: s = p·CL/100.

**Sonuç:**

- X-grafiği: merkez çizgisi 59,20 µg/L; uyarı sınırları 54,46 63,94 µg/L; eylem sınırları 52,10 66,30 µg/L (s = 2,37 µg/L, istatistiksel kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ch7-case1-statistical.json --lang tr`

### X-grafiğinin sınırlarının belirlenmesi, durum 2 (Nordtest TR 569 Bölüm 7)

`tr569-ch7-case2-target-mean` — Nordtest TR 569 ed. 6 (2026), Chapter 7, Setting the control limits and the central line in X-charts, case 2

Durum 2: gerekli %5 s_Rw'den hedef sınırlar, ortalama merkez çizgisi 59,2 µg/L.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Hedef kontrol sınırları: yöntemin s'si yerine gereklilikten s_hedef kullanılır; uyarı sınırları CL ± 2s_hedef, eylem sınırları CL ± 3s_hedef (Bölüm 7).
2. Göreli standart sapma merkez çizgisine uygulanır: s = p·CL/100.

**Sonuç:**

- X-grafiği: merkez çizgisi 59,20 µg/L; uyarı sınırları 53,28 65,12 µg/L; eylem sınırları 50,32 68,08 µg/L (s = 2,96 µg/L, hedef kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ch7-case2-target-mean.json --lang tr`

### X-grafiğinin sınırlarının belirlenmesi, durum 3 (Nordtest TR 569 Bölüm 7)

`tr569-ch7-case3-target-reference` — Nordtest TR 569 ed. 6 (2026), Chapter 7, Setting the control limits and the central line in X-charts, case 3

Durum 3: 60,0 µg/L referans değeri çevresinde %5 hedef sınırlar.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Hedef kontrol sınırları: yöntemin s'si yerine gereklilikten s_hedef kullanılır; uyarı sınırları CL ± 2s_hedef, eylem sınırları CL ± 3s_hedef (Bölüm 7).
2. Göreli standart sapma merkez çizgisine uygulanır: s = p·CL/100.

**Sonuç:**

- X-grafiği: merkez çizgisi 60,00 µg/L; uyarı sınırları 54,00 66,00 µg/L; eylem sınırları 51,00 69,00 µg/L (s = 3,00 µg/L, hedef kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ch7-case3-target-reference.json --lang tr`

### Paralel tayinler için r%-grafiğinin istatistiksel sınırları (Nordtest TR 569 Bölüm 7)

`tr569-ch7-range-case1-statistical` — Nordtest TR 569 ed. 6 (2026), Chapter 7, Setting the control limit in R or r%-charts, case 1

Açıklık grafikleri için durum 1: paralel tayinlerden %0,356 birleştirilmiş göreli standart sapma.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Paralel tayinlerin açıklık grafiği: merkez çizgisi ortalama açıklık, s = ortalama açıklık/d₂, d₂ = 1,128 (Eşitlik 4); üst uyarı sınırı D_WL·s = 2,833 s ve üst eylem sınırı D_AL·s = 3,686 s (Tablo 4, ISO 8258'den).
2. Üst uyarı sınırı 2,833·%0,356 = %1,0, üst eylem sınırı 3,686·%0,356 = %1,3.

**Sonuç:**

- r%-grafiği (göreli açıklık): merkez çizgisi 0,402 %; uyarı sınırları 1,01 %; eylem sınırları 1,31 % (s = 0,356 %, istatistiksel kontrol sınırları)

**Notlar:**

- Araştırma günlüğü F52: Bölüm 7'de açıklık grafikleri için durum 1'de uyarı sınırı 3·0,356 = %1,0 olarak basılmıştır; 3·0,356 = 1,07'dir, Tablo 4'ün katsayısı 2,83'tür ve 2,83·0,356 = 1,01'dir.

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ch7-range-case1-statistical.json --lang tr`

### Tekrarlanabilirlik sınırından r%-grafiğinin hedef sınırları (Nordtest TR 569 Bölüm 7)

`tr569-ch7-range-case2-target` — Nordtest TR 569 ed. 6 (2026), Chapter 7, Setting the control limit in R or r%-charts, case 2

Açıklık grafikleri için durum 2: standart yöntem r = %1 tekrarlanabilirlik sınırı verir.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. s_r = r/2,8 = %0,357 (2,8 ≈ 2√2 katsayısı iki sonucun farkından gelir, dipnot 12).
2. Paralel tayinlerin açıklık grafiği: merkez çizgisi ortalama açıklık, s = ortalama açıklık/d₂, d₂ = 1,128 (Eşitlik 4); üst uyarı sınırı D_WL·s = 2,833 s ve üst eylem sınırı D_AL·s = 3,686 s (Tablo 4, ISO 8258'den).
3. Üst uyarı sınırı 2,833·%0,357 = %1,0, üst eylem sınırı 3,686·%0,357 = %1,3; veri yokken merkez çizgisi beklenen açıklık d₂·s = %0,40'tır.

**Sonuç:**

- r%-grafiği (göreli açıklık): merkez çizgisi 0,403 %; uyarı sınırları 1,01 %; eylem sınırları 1,32 % (s = 0,357 %, hedef kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ch7-range-case2-target.json --lang tr`

### XRF ile düşük alaşımlı çelikte nikel: gerekli belirsizlikten hedef sınırlar (Nordtest TR 569 Örnek 1)

`tr569-ex1-nickel-steel-xrf` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 1

Bir çelik numunenin X-grafiği; bir yıllık ortalama ağırlıkça %4,58, s = ağırlıkça %0,026; gerekli genişletilmiş belirsizlik U = %4 (göreli).

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. u_c = U/2 = %2; s_Rw için gereklilik u_c'nin yarısıdır: s_Rw = U/4 = %1 (göreli) = ağırlıkça %0,0458.
2. Hedef kontrol sınırları: yöntemin s'si yerine gereklilikten s_hedef kullanılır; uyarı sınırları CL ± 2s_hedef, eylem sınırları CL ± 3s_hedef (Bölüm 7).
3. Yöntem hedefin altında s = ağırlıkça %0,026 ile çalışır: hedef sınırlar istatistiksel sınırlardan geniştir ve sonuçlar amaca uygun kalırken daha az yanlış alarm verir.

**Sonuç:**

- X-grafiği: merkez çizgisi 4,5800 weight-%; uyarı sınırları 4,4884 4,6716 weight-%; eylem sınırları 4,4426 4,7174 weight-% (s = 0,0458 weight-%, hedef kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex1-nickel-steel-xrf.json --lang tr`

### ICP-OES ile oyuncaklardan bor göçü: EN 71-3 tekrarüretilebilirliğinden hedef sınırlar (Nordtest TR 569 Örnek 10)

`tr569-ex10-boron-toys-target` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 10

Tüm işlem için malzeme yoktur: 5,0 µg/L laboratuvar içi sentetik standart kaymayı izler; s_hedef = %10, EN 71-3'ün %15 tekrarüretilebilirliğinin üçte ikisi.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Hedef kontrol sınırları: yöntemin s'si yerine gereklilikten s_hedef kullanılır; uyarı sınırları CL ± 2s_hedef, eylem sınırları CL ± 3s_hedef (Bölüm 7).
2. Değerlerin neredeyse tamamı merkez çizgisinin üzerindedir; yaklaşık +%6 sapma (ortalama 5,3 µg/L), yaklaşık %30 gerekli belirsizlik için kabul edilebilir bulunmuştur.

**Sonuç:**

- X-grafiği: merkez çizgisi 5,000 µg/L; uyarı sınırları 4,000 6,000 µg/L; eylem sınırları 3,500 6,500 µg/L (s = 0,500 µg/L, hedef kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex10-boron-toys-target.json --lang tr`

### Tekrarlı kontrol sonuçlarından tekrarlanabilirlik ve laboratuvar içi tekrarüretilebilirlik (Nordtest TR 569 Örnek 12)

`tr569-ex12-pooling-sr-srw` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 12 and footnote 24

Sekiz günün her birinde bir kalite kontrol numunesinin üç tekrarı. 24 sonucun birlikte standart sapması s_Rw için fazla düşük bir tahmindir ve çok dar kontrol sınırları verir.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. s_r: sekiz günün standart sapmaları (0,058 – 0,252) Eşitlik 10 ile birleştirilir: 0,15.
2. Basitleştirilmiş s_Rw: günler boyunca birinci, ikinci ve üçüncü sonuçların standart sapmaları (0,226, 0,342, 0,226) Eşitlik 10 ile birleştirilir: 0,270.
3. Tek yönlü varyans analizi: s_r = 0,1541, günler arası s_I = 0,2246, s_Rw = √(s_r² + s_I²) = 0,272 (dipnot 24).
4. Günlük ortalamaların X-grafiğinin s'si √(s_I² + s_r²/3)'tür; sınırları sekiz ortalamadan çıkar.

**Sonuç:**

- X-grafiği: merkez çizgisi 6,888; uyarı sınırları 6,404 7,371; eylem sınırları 6,163 7,612 (s = 0,242, istatistiksel kontrol sınırları)
- 8 kontrol değeri: 0 kontrol dışı, 0 uyarı ile eylem sınırları arasında
- s_r = 0,154, s_I = 0,225, s_Rw = 0,272 (tek yönlü varyans analizi)

**Notlar:**

- Araştırma günlüğü F53: dipnot 24 s_Rw = √(0,01541² + 0,02246²) = 0,0272 verir; değerler metindeki gibi s_r = 0,1541, s_I = 0,2246 ve s_Rw = 0,272'dir.

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex12-pooling-sr-srw.json --lang tr`

### XRF ile düşük alaşımlı çelikte kobalt: tayin sınırından hedef sınırlar (Nordtest TR 569 Örnek 2)

`tr569-ex2-cobalt-steel-xrf` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 2

Düşük derişimde X-grafiği, ortalama ağırlıkça %0,0768, s = ağırlıkça %0,00063; gerekli 0,01 ağırlıkça % LOQ, s'nin 6–10 katıdır, bu nedenle s_hedef = ağırlıkça %0,001.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Hedef kontrol sınırları: yöntemin s'si yerine gereklilikten s_hedef kullanılır; uyarı sınırları CL ± 2s_hedef, eylem sınırları CL ± 3s_hedef (Bölüm 7).
2. Kontrol numunesi ilgilenilen derişim olan LOQ'nun sekiz katındadır.

**Sonuç:**

- X-grafiği: merkez çizgisi 0,07680 weight-%; uyarı sınırları 0,07480 0,07880 weight-%; eylem sınırları 0,07380 0,07980 weight-% (s = 0,00100 weight-%, hedef kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex2-cobalt-steel-xrf.json --lang tr`

### Suda amonyum azotu: paralel numunelerin R-grafiği (Nordtest TR 569 Örnek 3)

`tr569-ex3-ammonium-r` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 3

Her seride 100 µg/L'nin altındaki bir numune paralel analiz edilir; paralel tayinlerin açıklığı çizilir.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Paralel tayinlerin açıklık grafiği: merkez çizgisi ortalama açıklık, s = ortalama açıklık/d₂, d₂ = 1,128 (Eşitlik 4); üst uyarı sınırı D_WL·s = 2,833 s ve üst eylem sınırı D_AL·s = 3,686 s (Tablo 4, ISO 8258'den).
2. Ortalama açıklık 0,559 µg/L, s = 0,559/1,128 = 0,496 µg/L; uyarı sınırı 2,83·0,496 = 1,40 µg/L, eylem sınırı 3,69·0,496 = 1,83 µg/L.

**Sonuç:**

- R-grafiği (açıklık): merkez çizgisi 0,559 µg/L; uyarı sınırları 1,40 µg/L; eylem sınırları 1,83 µg/L (s = 0,495 µg/L, istatistiksel kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex3-ammonium-r.json --lang tr`

### Suda amonyum azotu: standart çözeltinin X-grafiği (Nordtest TR 569 Örnek 3)

`tr569-ex3-ammonium-x` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 3

İndofenol mavisi yöntemi; NH₄Cl kalibrasyon standartlarından bağımsız olarak (NH₄)₂SO₄'ten hazırlanmış 20 µg/L kontrol çözeltisi; paralel tayinlerin ortalaması çizilir.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Merkez çizgisi ortalama x̄ = 19,99 µg/L; ortalamaların s'si 0,521 µg/L.
2. X-grafiği için istatistiksel kontrol sınırları: uyarı sınırları CL ± 2s, eylem sınırları CL ± 3s (Bölüm 7).

**Sonuç:**

- X-grafiği: merkez çizgisi 19,99 µg/L; uyarı sınırları 18,95 21,03 µg/L; eylem sınırları 18,43 21,55 µg/L (s = 0,521 µg/L, istatistiksel kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex3-ammonium-x.json --lang tr`

### ICP-MS ile suda kurşun: laboratuvar içi göl suyunun X-grafiği (Nordtest TR 569 Örnek 4)

`tr569-ex4-lead-water-icpms` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 4

Korunmuş göl suyunda düşük kurşun derişimi (0,29 µg/L), her seride bir kez analiz edilir; istatistiksel sınırlar.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. X-grafiği için istatistiksel kontrol sınırları: uyarı sınırları CL ± 2s, eylem sınırları CL ± 3s (Bölüm 7).
2. Merkez çizgisinin üstünde art arda on iki değer: istatistiksel kontrol dışı, ancak kontrol dışı değil (Bölüm 9); sonuçlar raporlanabilir.

**Sonuç:**

- X-grafiği: merkez çizgisi 0,2940 µg/L; uyarı sınırları 0,2780 0,3100 µg/L; eylem sınırları 0,2700 0,3180 µg/L (s = 0,00800 µg/L, istatistiksel kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex4-lead-water-icpms.json --lang tr`

### ICP-MS ile balık CRM'sinde arsenik: yalnızca ± %15 eylem sınırlı grafik (Nordtest TR 569 Örnek 5)

`tr569-ex5-arsenic-crm-tolerance` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 5

CRM NRC DORM-2 (köpek balığı kası), 18,0 µg/g arsenik, her seride bir kez analiz edilir; laboratuvar referans değerden %15'e kadar sapmayı kabul eder.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Merkez çizgisi referans değer; eylem sınırları CL ± tolerans = 18,0 ± 2,7 µg/g; uyarı sınırı yok (Bölüm 5 ve 7).
2. Uyarı sınırları olmadan üçte iki kuralı uygulanamaz; bu nedenle grafik ortalamadaki kaymaya daha az duyarlıdır.

**Sonuç:**

- X-grafiği: merkez çizgisi 18,00 µg/g; eylem sınırları 15,30 20,70 µg/g (toleransta eylem sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex5-arsenic-crm-tolerance.json --lang tr`

### Suda toplam fosfor: paralel tayinlerin r%-grafiği (Nordtest TR 569 Örnek 6)

`tr569-ex6-phosphorus-relative-range` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 6

10 µg/L'nin üzerindeki numuneler paralel analiz edilir; LOQ'nun çok üzerinde göreli standart sapma yaklaşık sabit olduğundan farkın ortalamaya yüzde oranı çizilir.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Paralel tayinlerin açıklık grafiği: merkez çizgisi ortalama açıklık, s = ortalama açıklık/d₂, d₂ = 1,128 (Eşitlik 4); üst uyarı sınırı D_WL·s = 2,833 s ve üst eylem sınırı D_AL·s = 3,686 s (Tablo 4, ISO 8258'den).
2. Ortalama r% %1,88, CV = 1,88/1,128 = %1,67; uyarı sınırı 2,83·1,67 = %4,73, eylem sınırı 3,69·1,67 = %6,16.

**Sonuç:**

- r%-grafiği (göreli açıklık): merkez çizgisi 1,88 %; uyarı sınırları 4,73 %; eylem sınırları 6,16 % (s = 1,67 %, istatistiksel kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex6-phosphorus-relative-range.json --lang tr`

### GC-ECD ile morina karaciğeri yağında β-HCH: referans değer çevresinde hedef sınırlar (Nordtest TR 569 Örnek 7)

`tr569-ex7-beta-hch-crm-target` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 7

CRM BCR 598, 16 µg/kg β-heksaklorosikloheksan, her seride bir kez analiz edilir; %15 hedef s.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Hedef kontrol sınırları: yöntemin s'si yerine gereklilikten s_hedef kullanılır; uyarı sınırları CL ± 2s_hedef, eylem sınırları CL ± 3s_hedef (Bölüm 7).
2. s_hedef = 0,15·16,0 = 2,4 µg/kg; merkez çizgisinin üzerindeki bir eğilim üç değerden ikisinin uyarı sınırının üstünde olmasıyla, yani kontrol dışı bir durumla sonuçlanmıştır (Örnek 7, yorum).

**Sonuç:**

- X-grafiği: merkez çizgisi 16,0 µg/kg; uyarı sınırları 11,2 20,8 µg/kg; eylem sınırları 8,80 23,2 µg/kg (s = 2,40 µg/kg, hedef kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex7-beta-hch-crm-target.json --lang tr`

### ICP-OES ile suda bakır: paralel tayinlerin R-grafiği ve gözden geçirilmesi (Nordtest TR 569 Örnek 8)

`tr569-ex8-copper-r-review` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 8

Numunelerin paralel tayinleri; ön ortalama açıklık 0,11 mg/L, s_r = 0,0975 mg/L; son 60 serinin s_r'si 0,0957 mg/L.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Paralel tayinlerin açıklık grafiği: merkez çizgisi ortalama açıklık, s = ortalama açıklık/d₂, d₂ = 1,128 (Eşitlik 4); üst uyarı sınırı D_WL·s = 2,833 s ve üst eylem sınırı D_AL·s = 3,686 s (Tablo 4, ISO 8258'den).
2. Uyarı sınırı 2,833·0,0975 = 0,28 mg/L, eylem sınırı 3,686·0,0975 = 0,36 mg/L.
3. F = 0,0975²/0,0957² = 1,04, kritik değer 1,67'nin altında (ν = 60, 60): tekrarlanabilirlik değişmemiştir. X-grafiği iki sonucun ortalamalarını çizdiğinden s_r, X-grafiğinin s_Rw'sinden büyüktür.

**Sonuç:**

- R-grafiği (açıklık): merkez çizgisi 0,110 mg/L; uyarı sınırları 0,276 mg/L; eylem sınırları 0,359 mg/L (s = 0,0975 mg/L, istatistiksel kontrol sınırları)
- Uzun dönem değerlendirmesi: F = 1,038 (kritik 1,67: anlamlı değil)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex8-copper-r-review.json --lang tr`

### ICP-OES ile suda bakır: tüm verilerden yeni X-grafiği sınırları (Nordtest TR 569 Örnek 8)

`tr569-ex8-copper-x-new-limits` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 8

Gözden geçirmeden sonra sınırlar daha uzun dönemden yeniden hesaplanır: x̄ = 1,048 mg/L ve s = 0,0822 mg/L.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. X-grafiği için istatistiksel kontrol sınırları: uyarı sınırları CL ± 2s, eylem sınırları CL ± 3s (Bölüm 7).
2. Nominal 1,00 mg/L'ye göre +%5 sapma, sonuçların kullanımı için önemliyse araştırılmalıdır.

**Sonuç:**

- X-grafiği: merkez çizgisi 1,048 mg/L; uyarı sınırları 0,8836 1,212 mg/L; eylem sınırları 0,8014 1,295 mg/L (s = 0,0822 mg/L, istatistiksel kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex8-copper-x-new-limits.json --lang tr`

### ICP-OES ile suda bakır: ön X-grafiği sınırlarının gözden geçirilmesi (Nordtest TR 569 Örnek 8)

`tr569-ex8-copper-x-review` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 8

Paralel ölçülen sentetik standart (1,00 ± 0,02 mg/L); ön sınırlar ilk 60 seriden, x̄ = 1,055 mg/L, s = 0,0667 mg/L. Son 60 değer gözden geçirilir: 11'i uyarı sınırları dışında; merkez çizgisinden 4 s'den uzak bir değer dışarıda bırakılır, x̄ = 1,041 mg/L ve s_Rw = 0,0834 mg/L olan 59 değer kalır.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. X-grafiği için istatistiksel kontrol sınırları: uyarı sınırları CL ± 2s, eylem sınırları CL ± 3s (Bölüm 7).
2. Yaklaşık 60 değerden 6'dan fazlası uyarı sınırları dışında: standart sapma muhtemelen artmıştır, bu nedenle F-testi uygulanır (Bölüm 10).
3. F = 0,0834²/0,0667² = 1,563, ν = 58 ve 59: kritik değerin altında (Tablo 3'te 60/60 için 1,67; tam değeri 1,68), anlamlı artış yok.
4. Birleştirilmiş s_p = √((59·0,0667² + 58·0,0834²)/117) = 0,07545 mg/L ile ortalamalara t-testi (Eşitlik 9): t = |1,055 − 1,041|/0,07545·√(60·59/119) = 1,012 < 1,98 (ν = 117): merkez çizgisinde anlamlı değişiklik yok.

**Sonuç:**

- X-grafiği: merkez çizgisi 1,055 mg/L; uyarı sınırları 0,9216 1,188 mg/L; eylem sınırları 0,8549 1,255 mg/L (s = 0,0667 mg/L, istatistiksel kontrol sınırları)
- Uzun dönem değerlendirmesi: F = 1,563 (kritik 1,68: anlamlı değil); t = 1,012 (kritik 1,98: anlamlı değil)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex8-copper-x-review.json --lang tr`

### Hidrojen peroksitte çinko: yöntem körünün X-grafiği (Nordtest TR 569 Örnek 9)

`tr569-ex9-zinc-procedural-blank` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 9

Kirlenmeyi saptamak için tüm işlemden geçirilen ultra saf su (50 mL buharlaştırma, HCl, 5 mL'ye seyreltme, ICP-OES); negatif değerler de çizilir.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. X-grafiği için istatistiksel kontrol sınırları: uyarı sınırları CL ± 2s, eylem sınırları CL ± 3s (Bölüm 7).
2. Alt sınırlar negatiftir: −0,051 mg/L ve −0,096 mg/L.

**Sonuç:**

- X-grafiği: merkez çizgisi 0,0390 mg/L; uyarı sınırları -0,0510 0,129 mg/L; eylem sınırları -0,0960 0,174 mg/L (s = 0,0450 mg/L, istatistiksel kontrol sınırları)

**Çalıştırma:** `uncertainty run library/chemistry/tr569-ex9-zinc-procedural-blank.json --lang tr`

### Sudaki çinko: 60 kontrol değerinden X-grafiği (Nordtest TR 569 Tablo 1)

`tr569-table1-zinc` — Nordtest TR 569 ed. 6 (2026), Chapter 2, Table 1 and Figure 1; Chapters 7 and 9

60,0 µg/L çinko çözeltisinin altmış kontrol değeri (Tablo 1), Şekil 1'de çizilmiştir. Sınırlar değerlerin kendisinden istatistiksel olarak belirlenir; her seri günlük olarak değerlendirilir.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. 60 değerin ortalaması ve standart sapması (Eşitlik 1, 2): x̄ = 60,278 µg/L, s = 2,598 µg/L; merkez çizgisi ortalamadır.
2. X-grafiği için istatistiksel kontrol sınırları: uyarı sınırları CL ± 2s, eylem sınırları CL ± 3s (Bölüm 7).
3. Günlük değerlendirme (Bölüm 9): üç değer uyarı ile eylem sınırları arasındadır (66,3, 54,5 ve 54,4 µg/L; 65,4 µg/L, 65,47 µg/L üst uyarı sınırının hemen içindedir) ve hiçbirinden önceki iki seride aynı tarafta başka bir değer yoktur; Şekil 1'deki gibi hiçbir değer kontrol dışı değildir.
4. Ortalama nominal 60,0 µg/L'den anlamlı olarak farklı değildir: t = |60,278 − 60,0|·√60/2,598 = 0,83 < 2,00 (Eşitlik 6).

**Sonuç:**

- X-grafiği: merkez çizgisi 60,28 µg/L; uyarı sınırları 55,08 65,47 µg/L; eylem sınırları 52,48 68,07 µg/L (s = 2,60 µg/L, istatistiksel kontrol sınırları)
- 60 kontrol değeri: 0 kontrol dışı, 3 uyarı ile eylem sınırları arasında

**Çalıştırma:** `uncertainty run library/chemistry/tr569-table1-zinc.json --lang tr`

### Serada yetişen marulda nitrat: çift numune yöntemi, klasik ANOVA (Eurachem UfS Örnek A1)

`ufs-a1-nitrate-lettuce` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A1, Table A1.1 and Figure A1.2

Sekiz marul bölmesi; her birinden iki numune (W düzeninde 10 baştan kompozit numune) ve HPLC ile çift analiz (Tablo A1.1).

**Yaklaşım:** numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)

**Adımlar:**

1. Çift numune yöntemi (Bölüm 9.4.2): her hedeften protokolün bağımsız uygulamalarıyla iki numune alınır ve her biri iki kez analiz edilir; s_ölçüm² = s_numune² + s_analiz² (Eşitlik 1).
2. Klasik iç içe ANOVA: 2n analiz çifti için KO_analiz = Σ(d²/2)/(2n); s_numune² = (KO_numune − KO_analiz)/2; s_T² = (KO_hedef − KO_numune)/4; negatif varyans tahminleri sıfır alınır.
3. Klasik sonuçlar (Şekil A1.2): x̄ = 4345,5625 mg/kg; s_T = 556,2804, s_numune = 518,16089, s_analiz = 148,18063 mg/kg; toplam varyansın %51,58, %44,76 ve %3,66'sı (s_toplam = 774,5296 mg/kg).
4. Bağıl genişletilmiş belirsizlik U′ = 2·100·s/x̄; x̄ tüm sonuçların ortalamasıdır (Eşitlik 4, k = 2).
5. C hedefinin numune çiftleri çok farklıdır (ortalama 5805,5 ve 3921,5 mg/kg): klasik s_numune'yi büyütür. Kılavuzun robust tahminleri U′ = %14,5 (numune alma), %7,6 (analiz) ve %16,4 (ölçüm); klasik tahminler %23,8, %6,8 ve %24,8 verir.

**Sonuç:**

- s_numune = 518,2, s_analiz = 148,2, s_ölçüm = 538,9 mg/kg, ortalama 4346 mg/kg: U′ = %23,8 (numune alma), %6,82 (analiz), %24,8 (ölçüm), k = 2; hedefler arası s_T = 556,3 mg/kg

**Notlar:**

- Kılavuz ana sonuç olarak robust ANOVA tahminlerini (RANOVA ve RANOVA2 programları) verir. Bunların algoritması kılavuzda yoktur ve robust değerler yeniden üretilememiştir (araştırma günlüğü F63); bu nedenle proje, kılavuzun yanında yazdırdığı ve birebir yeniden üretilen klasik ANOVA'yı değerlendirir.
- Robust tahminler (Şekil A1.2): ortalama 4408,3237, s_T 565,39868, s_numune 319,04834, s_analiz 167,94308 mg/kg. Sekiz hedef ortalamasına Huber H15 uygulanması robust ortalamayı (4408,319) verir, ancak yayımlanmış hiçbir değişken robust standart sapmaları yeniden üretmez.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a1-nitrate-lettuce.json --lang tr`

### Kirlenmiş üst toprakta kurşun: klasik ANOVA ve belirsizlik faktörü (Eurachem UfS Örnek A2)

`ufs-a2-lead-soil-uncertainty-factor` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A2, Tables A2.2–A2.4 and Figure A2.2

100 grid hedefinden onu (30 m × 30 m, 0–150 mm) 3 m arayla çift numune alınarak ICP-AES ile iki kez analiz edilmiştir (Tablo A2.2). Değerler çarpıktır: belirsizlik ln x'in ANOVA'sından bir faktör olarak da ifade edilir.

**Yaklaşım:** numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)

**Adımlar:**

1. Çift numune yöntemi (Bölüm 9.4.2): her hedeften protokolün bağımsız uygulamalarıyla iki numune alınır ve her biri iki kez analiz edilir; s_ölçüm² = s_numune² + s_analiz² (Eşitlik 1).
2. Klasik iç içe ANOVA: 2n analiz çifti için KO_analiz = Σ(d²/2)/(2n); s_numune² = (KO_numune − KO_analiz)/2; s_T² = (KO_hedef − KO_numune)/4; negatif varyans tahminleri sıfır alınır.
3. Klasik sonuçlar (Şekil A2.2): x̄ = 317,8 mg/kg; s_T = 197,55, s_numune = 135,43, s_analiz = 17,99, s_ölçüm = 136,62 mg/kg; U′ = %85,23 (numune alma), %11,32 (analiz), %85,98 (ölçüm).
4. ln x'in ANOVA'sı (Tablo A2.3): ortalama 5,478 (geometrik ortalama 239,4 mg/kg), s_G = 0,66775 (hedefler arası), 0,4784 (numune alma), 0,0567 (analiz), 0,4817 (ölçüm).
5. Genişletilmiş belirsizlik faktörü F_U = exp(2 s_G) (Eşitlik 5): ölçüm için 2,6207, numune alma için 2,6032; 300 mg/kg'lık bir sonuç 300/2,62 = 115 ile 300 × 2,62 = 784 mg/kg arasındadır (Tablo A2.4).
6. u′ = √(exp(s_G²) − 1) = 0,5111 (Bölüm 6.3); u′ > 0,2 olduğundan 2 ile çarpılmaz.

**Sonuç:**

- s_numune = 135,4, s_analiz = 17,99, s_ölçüm = 136,6 mg/kg, ortalama 317,8 mg/kg: U′ = %85,2 (numune alma), %11,3 (analiz), %86,0 (ölçüm), k = 2; hedefler arası s_T = 197,6 mg/kg
- ln x: s_G,numune = 0,4784, s_G,analiz = 0,05668, s_G,ölçüm = 0,4817; belirsizlik faktörleri F_U = 2,603 (numune alma), 1,120 (analiz), 2,621 (ölçüm)

**Notlar:**

- Kılavuz ana sonuç olarak robust ANOVA tahminlerini (RANOVA ve RANOVA2 programları) verir. Bunların algoritması kılavuzda yoktur ve robust değerler yeniden üretilememiştir (araştırma günlüğü F63); bu nedenle proje, kılavuzun yanında yazdırdığı ve birebir yeniden üretilen klasik ANOVA'yı değerlendirir.
- Şekil A2.2 klasik ANOVA satırında numune alma için F_U = 2,6032 ve ölçüm için 2,6207 verir; metin yuvarlanmış s_G'den 2,6034 = exp(2·0,4784) verir.
- Bölüm 6.4 analitik sapmayı (−%3,41 ± %1,34) rastgele analitik bileşene ekler: standart %5,24, robust numune alma terimiyle U′_ölçüm = %83,95; bu tür terimler bir model projesine eklenebilir.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a2-lead-soil-uncertainty-factor.json --lang tr`

### Yeraltı suyunda çözünmüş demir: klasik ANOVA (Eurachem UfS Örnek A3)

`ufs-a3-iron-groundwater-anova` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A3, Tables A3.5 and A3.7; Eurachem UfS:2019 Errata, Corrections to Table A3.7, Table A3.4 and the summary on p. 60

Bir yeraltı suyu kütlesinde altı izleme kuyusu; her birinden iki numune (hat üzerinde süzülmüş ve korunmuş) ve ICP-AES ile çift laboratuvar analizi (Tablo A3.7).

**Yaklaşım:** numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)

**Adımlar:**

1. Çift numune yöntemi (Bölüm 9.4.2): her hedeften protokolün bağımsız uygulamalarıyla iki numune alınır ve her biri iki kez analiz edilir; s_ölçüm² = s_numune² + s_analiz² (Eşitlik 1).
2. Klasik iç içe ANOVA: 2n analiz çifti için KO_analiz = Σ(d²/2)/(2n); s_numune² = (KO_numune − KO_analiz)/2; s_T² = (KO_hedef − KO_numune)/4; negatif varyans tahminleri sıfır alınır.
3. x̄ = 1,719 mg/L'de s_analiz = 0,0136, s_numune = 0,0827, s_T = 0,601 mg/L: U′ = %1,6 (analiz), %9,6 (numune alma), %70 (kuyular arası), Tablo A3.5'teki gibi; aralık yöntemi %1,8, %10,5 ve %70 verir.

**Sonuç:**

- s_numune = 0,08267, s_analiz = 0,01357, s_ölçüm = 0,08377 mg/L, ortalama 1,719 mg/L: U′ = %9,62 (numune alma), %1,58 (analiz), %9,74 (ölçüm), k = 2; hedefler arası s_T = 0,6013 mg/L

**Notlar:**

- Tablo A3.5 RANOVA tahminlerini de verir (ROBAN 1.0.1): %1,8, %9,9 ve %72; robust algoritma uygulanmamıştır (F63).

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a3-iron-groundwater-anova.json --lang tr`

### Yeraltı suyunda çözünmüş demir: aralık yöntemi (Eurachem UfS Örnek A3, düzeltilmiş)

`ufs-a3-iron-groundwater-range` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A3, Tables A3.4 and A3.7; Eurachem UfS:2019 Errata, Corrections to Table A3.7, Table A3.4 and the summary on p. 60

Bir yeraltı suyu kütlesinde altı izleme kuyusu; her birinden iki numune (hat üzerinde süzülmüş ve korunmuş) ve ICP-AES ile çift laboratuvar analizi (Tablo A3.7).

**Yaklaşım:** numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)

**Adımlar:**

1. Çift numune yöntemi (Bölüm 9.4.2): her hedeften protokolün bağımsız uygulamalarıyla iki numune alınır ve her biri iki kez analiz edilir; s_ölçüm² = s_numune² + s_analiz² (Eşitlik 1).
2. Analiz: R̄_A = |S1A1 − S1A2| ve |S2A1 − S2A2| ortalaması = 0,0173 mg/L; s_A = R̄_A/d₂ = 0,0154 mg/L (d₂ = 1,128, iki değerin aralığı).
3. Numune alma: R̄_S = |x̄_S1 − x̄_S2| ortalaması = 0,102 mg/L; s_S+A = R̄_S/d₂ = 0,091 mg/L; s_S = √(s_S+A² − s_A²/2) = 0,090 mg/L (CV %5,23).
4. Kuyular arası: kuyu ortalamalarının standart sapması s_T+S+A = 0,604 mg/L; s_T = √(0,604² − 0,091²/2) = 0,601 mg/L (CV %35).
5. U′ (k = 2): analiz %1,8, numune alma %10,5, ölçüm %11, kuyular arası %70 (düzeltme listesiyle düzeltilmiş Tablo A3.4 ve özet).

**Sonuç:**

- s_numune = 0,08989, s_analiz = 0,01536, s_ölçüm = 0,09119 mg/L, ortalama 1,719 mg/L: U′ = %10,5 (numune alma), %1,79 (analiz), %10,6 (ölçüm), k = 2; hedefler arası s_T = 0,6008 mg/L

**Notlar:**

- Basılı Tablo A3.7 s_T+S+A = 0,55, s_T = 0,548 mg/L ve CV_T = %32 verir (Tablo A3.4'te hedefler arası %64); düzeltme listesi (sürüm 1.1, 2020) bunları 0,604, 0,601 mg/L, %35 ve %70 olarak düzeltir; program bunları yeniden üretir.
- Özet tablo validasyonun analitik belirsizliği için %1,9, Tablo A3.4 %1,8 verir; aralık hesabı 2·%0,89 = %1,8 verir.
- Kılavuz d₂ = 1,128 kullanır; program 2/√π = 1,12838 (ISO 8258) kullanır; fark basılı basamakların altındadır.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a3-iron-groundwater-range.json --lang tr`

### Bebek lapasında A vitamini, 40 g deney numunesi: çift numune yöntemi (Eurachem UfS Örnek A4)

`ufs-a4-vitamin-a-porridge-40g` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A4, Tables A4.3, A4.5–A4.7, A4.14, A4.15 and summary; Eurachem UfS:2019 Errata, Correction to the RSD in the summary table of Example A4, p. 75

Muzlu ve kayısılı yulaf lapasının on üretim partisinin her birinden iki numune (paket); her biri HPLC için çift deney numunesine bölünmüştür (aynı deney numunesinin tekrar analizi yoktur).

**Yaklaşım:** numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)

**Adımlar:**

1. Çift numune yöntemi (Bölüm 9.4.2): her hedeften protokolün bağımsız uygulamalarıyla iki numune alınır ve her biri iki kez analiz edilir; s_ölçüm² = s_numune² + s_analiz² (Eşitlik 1).
2. KT_analiz = 16595 (µg/100 g)², ν = 20: s_analiz = 28,805 µg/100 g, RSD %8,28 (Tablo A4.5, A4.14).
3. KT_numune = 14231, ν = 10: V_numune = (14231/10 − 16595/20)/2 = 296,675, s_numune = 17,224 µg/100 g, RSD %4,95 (Tablo A4.6, A4.15).
4. u_ölçüm = √(4,95² + 8,28²) = %9,65; U′ = %9,9 (numune alma), %16,6 (analiz), %19,4 (ölçüm); partiler arası RSD_B = %6,1 (düzeltme listesi 1.4a).

**Sonuç:**

- s_numune = 17,22, s_analiz = 28,81, s_ölçüm = 33,56 µg/100 g, ortalama 347,9 µg/100 g: U′ = %9,90 (numune alma), %16,6 (analiz), %19,3 (ölçüm), k = 2; hedefler arası s_T = 21,27 µg/100 g

**Notlar:**

- Basılı özet partiler arası değişkenlik için RSD_B = %8,2 verir; düzeltme listesi 1.4a (2023) bunu %6,1'e düzeltir (Tablo A4.3'ün klasik ANOVA'sı); program bunu yeniden üretir.
- Tablo A4.7 U′'yu %17 (analiz) ve %20 (ölçüm) olarak yuvarlar; özet %16,6 ve %19,4, yani yuvarlanmış u'nun (%8,3 ve %9,7) iki katını verir. Yuvarlamadan U′_ölçüm = 2·%9,648 = %19,3.
- Laboratuvarın kendi analitik belirsizliği (%7) burada bulunan %8,28'den küçüktür; kılavuz büyük değeri kullanır.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a4-vitamin-a-porridge-40g.json --lang tr`

### Bebek lapasında A vitamini, 4 g deney numunesi: negatif numune alma varyansı (Eurachem UfS Örnek A4)

`ufs-a4-vitamin-a-porridge-4g` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A4, Tables A4.4, A4.8–A4.10

Muzlu ve kayısılı yulaf lapasının on üretim partisinin her birinden iki numune (paket); her biri HPLC için çift deney numunesine bölünmüştür (aynı deney numunesinin tekrar analizi yoktur).

**Yaklaşım:** numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)

**Adımlar:**

1. Çift numune yöntemi (Bölüm 9.4.2): her hedeften protokolün bağımsız uygulamalarıyla iki numune alınır ve her biri iki kez analiz edilir; s_ölçüm² = s_numune² + s_analiz² (Eşitlik 1).
2. KT_analiz = 312206,5, ν = 20: s_analiz = 124,94 µg/100 g, RSD %36,68 (Tablo A4.8).
3. KT_numune = 102860,25, ν = 10: V_numune = (102860,25/10 − 15610,325)/2 = −2662,15 < 0; geleneksel olarak sıfır alınır (Tablo A4.9).
4. Küçük deney porsiyonu baskındır: U′_analiz = %73,4. Kılavuz numune alma RSD'sini 40 g çalışmasından (%5) alır ve U′_ölçüm = %74 bildirir (Tablo A4.10): 4 g deney numunesi amaca uygun değildir.

**Sonuç:**

- s_numune = 0, s_analiz = 124,9, s_ölçüm = 124,9 µg/100 g, ortalama 340,6 µg/100 g: U′ = %0 (numune alma), %73,4 (analiz), %73,4 (ölçüm), k = 2; hedefler arası s_T = 0 µg/100 g

**Notlar:**

- Kılavuz robust ANOVA ile u_numune = %6,9 ve u_analiz = %30 bulur; uygulanmamıştır (F63).
- Bu projenin ölçüm belirsizliği (%73,4) yalnızca analitik terimi içerir, çünkü bu veri setinin numune alma varyansı sıfırdır; Tablo A4.10 bunu 40 g çalışmasındaki %5 ile birleştirir.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a4-vitamin-a-porridge-4g.json --lang tr`

### Bebek lapasında A vitamini: çift numunelerin kontrol grafiği (Eurachem UfS Örnek A4)

`ufs-a4-vitamin-a-qc-chart` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A4, Section 7.3, Table A4.13 and Figure A4.4; Section 13.2

Numune almanın kalite kontrolü: sekiz lapanın her birinin bir partisinden iki numune, her biri iki kez analiz (40 g); iki numunenin bağıl farkı D %, valide edilmiş belirsizlikten gelen sınırlara göre grafiğe işlenir (Tablo A4.13, Şekil A4.4).

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Valide edilmiş u_ölçüm = √(u_numune² + u_analiz²) = √(4,95² + 8,28²) = %9,65 (Tablo A4.7).
2. İki değerin aralık grafiği (Bölüm 13.2.2): CL = 1,128·u = %10,9, WL = 2,83·u = %27,3, AL = 3,69·u = %35,6 (kılavuzda %11, %27 ve %36).
3. Her ürün ve analiz için D % = |x_S1 − x_S2|/x̄·100: 16 değerin tümü uyarı sınırının altındadır, Şekil A4.4'teki gibi.

**Sonuç:**

- r%-grafiği (göreli açıklık): merkez çizgisi 10,9 %; uyarı sınırları 27,3 %; eylem sınırları 35,6 % (s = 9,65 %, hedef kontrol sınırları)
- 16 kontrol değeri: 0 kontrol dışı, 0 uyarı ile eylem sınırları arasında

**Notlar:**

- Bölüm 7.3 uyarı sınırı faktörünü formülde 2,38 yazar ancak 2,83 ile hesaplar (= çiftler için D_WL, Nordtest TR 569 Tablo 4).
- Merkez çizgisi, valide edilmiş belirsizlik için beklenen aralık d₂·u'dur; 16 değerin ortalaması (%12,1) değildir.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a4-vitamin-a-qc-chart.json --lang tr`

### Tavuk yeminde enzim: Gy kuramıyla temel numune alma hatası (Eurachem UfS Örnek A5)

`ufs-a5-enzyme-feed-gy` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A5, Table A5.1; Section 10.2.7–10.2.8, Equations 6–8

25 kg'lık torbadan 500 g birincil numune (%0,05 m/m enzim), < 0,5 mm'ye öğütülür ve ekstraksiyon ile LC için 2 g deney porsiyonu alınır (%5 analitik RSD). Tablo A5.1'deki malzeme özellikleriyle modelleme yaklaşımı.

**Yaklaşım:** numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)

**Adımlar:**

1. Bileşim faktörü (Eşitlik 8): c = (1 − a_L/α)²/(a_L/α)·ρ_c + (1 − a_L/α)·ρ_m = 2158,5 g cm⁻³ (kılavuzda 2160); a_L/α = %0,05/%100, ρ_c = 1,08 ve ρ_m = 0,67 g cm⁻³.
2. Numune alma sabitleri C = f·g·β·c, f = 0,5, β = 1: C₁ = 540 (g₁ = 0,5) ve C₂ = 270 g cm⁻³ (g₂ = 0,25).
3. s_r² = C·d³·(1/M_S − 1/M_L) (Eşitlik 6): birincil numune s_r1 = 0,033 (d = 0,1 cm, 25 000 g'dan 500 g); deney porsiyonu s_r2 = 0,130 (d = 0,05 cm, 500 g'dan 2 g).
4. s_t = √(s_r1² + s_r2² + 0,05²) = 0,143; U′ = 2·%14,3 = %28,5 (kılavuzda yuvarlanmış bileşenlerden %28,6); en zayıf halka deney porsiyonudur.

**Sonuç:**

- temel numune alma hatası: Primary sample s_r = 0,03252, Test portion s_r = 0,1296; numune alma s_r = 0,1336, toplam s_r = 0,1427; U′ = %26,7 (numune alma), %28,5 (toplam), k = 2

**Notlar:**

- Örnek A5 metni bileşim faktörüne 'Eşitlik 7' der; Bölüm 10.2.8'de bu Eşitlik 8'dir (ve 6 numarası iki kez kullanılmıştır).
- Yalnızca temel numune alma hatası: hatalı numune alma hataları ve analitik sapma ihmal edilebilir kabul edilmiştir.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a5-enzyme-feed-gy.json --lang tr`

### Tarım toprağında kadmiyum: modelleme yaklaşımı (Eurachem UfS Örnek A6)

`ufs-a6-cadmium-soil` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A6, Tables A6.1–A6.6

0,32 ha'lık bir tarlanın (143 m × 22 m, 0–30 cm) kompozit numunelerden ortalama derişimi; numune alma etkilerinin (konumlar arası, strateji, derinlik, bölme, kurutma) ve analizin belirsizlik bütçesi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Model x_site = x̄_sqr·f_strat·f_depth·f_split·f_dry·f_analiz; düzeltme faktörlerinin nominal değeri 1'dir (Bölüm 4.3).
2. Konumlar arası: beş karenin ortalaması, u = s_kare/√5 (A tipi).
3. Derinlik: x₋ = (x̄ − 0,1 c₋)/0,9 ve x₊ = (x̄ + 0,1 c₊)/1,1; c₋ = 0,14, c₊ = 0,1 mg/kg; yarı genişlik (x₋ − x₊)/2, dikdörtgen.
4. Kurutma: %2'lik nem aralığı, u = %1/√3 = %0,6.
5. Bölme u = Tablo A6.3'teki RSD'lerin ortalaması = %3,7; strateji %1 (hipoteze dayalı düzen); analiz √(3,6² + 2,7² + 2,7²) = %5,2 (Tablo A6.4).

**Sonuç:**

- x_site = (0,319 ± 0,059) mg/kg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Tablo A6.2'den derinlik terimi %3,6'dır (x₋ = 0,339, x₊ = 0,299 mg/kg); Tablo A6.6 %3,5 verir. Bununla birleşik belirsizlik %9,1 yerine %9,2, U ise %18,2 yerine %18,4 olur; yuvarlanmış sonuç 0,32 ± 0,06 mg/kg aynıdır.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a6-cadmium-soil.json --lang tr`

### Tarım toprağında fosfor: modelleme yaklaşımı (Eurachem UfS Örnek A6)

`ufs-a6-phosphorus-soil` — Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Appendix A, Example A6, Tables A6.1–A6.6

0,32 ha'lık bir tarlanın (143 m × 22 m, 0–30 cm) kompozit numunelerden ortalama derişimi; numune alma etkilerinin (konumlar arası, strateji, derinlik, bölme, kurutma) ve analizin belirsizlik bütçesi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Model x_site = x̄_sqr·f_strat·f_depth·f_split·f_dry·f_analiz; düzeltme faktörlerinin nominal değeri 1'dir (Bölüm 4.3).
2. Konumlar arası: beş karenin ortalaması, u = s_kare/√5 (A tipi).
3. Derinlik: x₋ = (x̄ − 0,1 c₋)/0,9 ve x₊ = (x̄ + 0,1 c₊)/1,1; c₋ = 47, c₊ = 35 mg/kg; yarı genişlik (x₋ − x₊)/2, dikdörtgen.
4. Kurutma: %2'lik nem aralığı, u = %1/√3 = %0,6.
5. Bölme u = %3,16 (Tablo A6.3 ortalaması); strateji %0,5; analiz √(1,7² + 9,5²) = %9,7 (Tablo A6.5).

**Sonuç:**

- x_site = (116 ± 26) mg/kg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Tablo A6.1 s_kare = 8,0 mg/kg yazar; beş değer 7,43 mg/kg (%6,4) verir ve basılı u_b-loc = %2,9 değeri 7,43/√5'ten çıkar.
- Tablo A6.3'teki RSD'lerin ortalaması basılı %3,3 değil %3,16'dır; proje %3,16 kullanır. Birleşik belirsizlik (%11,3) ve U = 26 mg/kg değişmez.

**Çalıştırma:** `uncertainty run library/chemistry/ufs-a6-phosphorus-soil.json --lang tr`

### Nehir sedimentlerinde fluoranten, aritmetik ortalama geri kazanım (MUPR Eş. 8–12) (Bettencourt da Silva 2026)

`bettencourt2026-fluoranthene-arithmetic` — R. J. N. Bettencourt da Silva, Section 3.2; Supplementary Material 06, sheets Trueness, Performance Model and Analysis sequence

MUPR elektronik tablosunun laboratuvar içi validasyon verilerinden nehir sedimentlerinde fluorantenin top-down belirsizliği: iki aralıklı kesinlik modeli ve uyumlu olmayan 20 ortalama geri kazanım; bu nedenle geri kazanımların dağılımı eklenir ve sonuçlar geri kazanım için düzeltilir.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Nehir sedimentlerinde fluoranten, hızlandırılmış çözücü ekstraksiyonu ve GC-MS/MS ile; sekiz günde valide edilmiştir. Makalenin elektronik tablosu Eurachem/CITAC MUPR rehberini uygular; Ek Materyal 06, doğal kütle kesrinin %10'undan küçük katkıları dışarıda bırakır.
2. Kesinlik modeli: I. aralık (−0,1 ile 1,5 mg/kg arası) sabit mutlak s_I = 0,075185 mg/kg (= 1,5 mg/kg × s′_I); II. aralık (1,5 ile 10,01 mg/kg arası) sabit bağıl s′_I = %5,0123. Tek sonuç raporlandığından kesinlik bileşeni s_I'dır.
3. Geri kazanım: SRM'den, doğal analit içermeyen bir sedimentin üç katkısından ve doğal analit içeren sedimentlerin 16 katkısından 20 ortalama geri kazanım; her birinin u(R_k) değeri elektronik tablodandır (kesinlik modeli ve katkı belirsizliği). Değerler %53,2 ile %89,8 arasındadır.
4. χ² testi (Eş. 16.2): χ² = 40,97 > χ²(0,95; 19) = 30,14; geri kazanımlar uyumlu değildir, bu nedenle s(R) = %8,755 ortalama geri kazanımın belirsizliğine eklenir (MUPR 8.2.7).
5. Aritmetik ortalama geri kazanım R̄ = ΣR_k/N = %76,307, u(R̄) = √Σu²(R_k)/N = %4,008 (Eş. 12); s(R) ile: u = %9,629. u(R̄)'nin büyük kısmı SS95(SP1) testinden (u = %75) gelir.
6. Anlamlılık (Eş. 13): |1 − R|/u = 2,461 > t(0,975; 19) = 2,093; sonuçlar düzeltilir, c = x/R. Kapsam faktörü k = 2,093, elektronik tablodaki gibi bileşenlerin en küçük serbestlik derecesinden (ν = 19, geri kazanımlar).
7. Sonuçlar: x = 0,1 mg/kg → (0,1310 ± 0,2091) mg/kg; x = 10 mg/kg → (13,105 ± 3,724) mg/kg (%28,4).

**Sonuç:**

- w(fluoranthene) = (0,13 ± 0,21) mg/kg (k = 2,09)
- w(fluoranthene) = (13,1 ± 3,7) mg/kg (k = 2,09)

**Notlar:**

- Elektronik tablo 10 mg/kg'da (13,35 ± 3,86) mg/kg ve 0,1 mg/kg'da (0,134 ± 0,161) mg/kg verir. Ağırlıklı ortalama R̿ = %74,887 ile aritmetik ortalamanın belirsizliğini, √Σu²/N = %4,008, birlikte kullanır (araştırma günlüğü F73) ve I. aralığın mutlak kesinliğini R̿'ya bölmez (F72). Makale metni k = 2,01 yazar; tablo 2,093 kullanır (F71).
- Program, geri kazanımla düzeltilmiş sonucun mutlak bileşenlerini geri kazanıma böler (y = x/R). Bu örneğin diğer türü MUPR Eş. (16.1)'in ağırlıklı ortalamasını kendi belirsizliğiyle kullanır.

**Çalıştırma:** `uncertainty run library/chemistry/bettencourt2026-fluoranthene-arithmetic.json --lang tr`

### Nehir sedimentlerinde fluoranten, ağırlıklı ortalama geri kazanım (MUPR Eş. 16.1) (Bettencourt da Silva 2026)

`bettencourt2026-fluoranthene-weighted` — R. J. N. Bettencourt da Silva, Section 3.2; Supplementary Material 06, sheets Trueness, Performance Model and Analysis sequence

MUPR elektronik tablosunun laboratuvar içi validasyon verilerinden nehir sedimentlerinde fluorantenin top-down belirsizliği: iki aralıklı kesinlik modeli ve uyumlu olmayan 20 ortalama geri kazanım; bu nedenle geri kazanımların dağılımı eklenir ve sonuçlar geri kazanım için düzeltilir.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Nehir sedimentlerinde fluoranten, hızlandırılmış çözücü ekstraksiyonu ve GC-MS/MS ile; sekiz günde valide edilmiştir. Makalenin elektronik tablosu Eurachem/CITAC MUPR rehberini uygular; Ek Materyal 06, doğal kütle kesrinin %10'undan küçük katkıları dışarıda bırakır.
2. Kesinlik modeli: I. aralık (−0,1 ile 1,5 mg/kg arası) sabit mutlak s_I = 0,075185 mg/kg (= 1,5 mg/kg × s′_I); II. aralık (1,5 ile 10,01 mg/kg arası) sabit bağıl s′_I = %5,0123. Tek sonuç raporlandığından kesinlik bileşeni s_I'dır.
3. Geri kazanım: SRM'den, doğal analit içermeyen bir sedimentin üç katkısından ve doğal analit içeren sedimentlerin 16 katkısından 20 ortalama geri kazanım; her birinin u(R_k) değeri elektronik tablodandır (kesinlik modeli ve katkı belirsizliği). Değerler %53,2 ile %89,8 arasındadır.
4. χ² testi (Eş. 16.2): χ² = 40,97 > χ²(0,95; 19) = 30,14; geri kazanımlar uyumlu değildir, bu nedenle s(R) = %8,755 ortalama geri kazanımın belirsizliğine eklenir (MUPR 8.2.7).
5. Ağırlıklı ortalama geri kazanım R̿ = Σ(R_k/u²)/Σ(1/u²) = %74,887, u(R̿) = (Σ1/u²)^−½ = %0,996; s(R) ile: u = %8,812.
6. Anlamlılık (Eş. 13): |1 − R|/u = 2,850 > t(0,975; 19) = 2,093; sonuçlar düzeltilir, c = x/R. Kapsam faktörü k = 2,093, elektronik tablodaki gibi bileşenlerin en küçük serbestlik derecesinden (ν = 19, geri kazanımlar).
7. Sonuçlar: x = 0,1 mg/kg → (0,1335 ± 0,2127) mg/kg; x = 10 mg/kg → (13,353 ± 3,575) mg/kg (%26,8).

**Sonuç:**

- w(fluoranthene) = (0,13 ± 0,21) mg/kg (k = 2,09)
- w(fluoranthene) = (13,4 ± 3,6) mg/kg (k = 2,09)

**Notlar:**

- Elektronik tablo 10 mg/kg'da (13,35 ± 3,86) mg/kg ve 0,1 mg/kg'da (0,134 ± 0,161) mg/kg verir. Ağırlıklı ortalama R̿ = %74,887 ile aritmetik ortalamanın belirsizliğini, √Σu²/N = %4,008, birlikte kullanır (araştırma günlüğü F73) ve I. aralığın mutlak kesinliğini R̿'ya bölmez (F72). Makale metni k = 2,01 yazar; tablo 2,093 kullanır (F71).
- Program, geri kazanımla düzeltilmiş sonucun mutlak bileşenlerini geri kazanıma böler (y = x/R). Bu örneğin diğer türü MUPR Eş. (8)–(12)'nin aritmetik ortalamasını kullanır.

**Çalıştırma:** `uncertainty run library/chemistry/bettencourt2026-fluoranthene-weighted.json --lang tr`

### Hayvan yeminde ham lif, tekrarüretilebilirlik verilerinden (ISO 21748 C.4)

`iso21748-c4-crude-fibre` — ISO 21748:2017, Annex C.4, Tables C.6–C.9

ISO 21748:2017 C.4: bir ortak çalışmadan düzeye bağlı tekrarüretilebilirlik ve sabit kütleye kurutmanın belirsizliği; QUAM A6 ile aynı örnek, kurutma terimi farklı ele alınmıştır.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Ortak çalışma (Tablo C.6): %2,3 lifte s_R = %0,293, %5,4'te %0,390 ve %10,1'de %0,575. Sapma bir CRM üzerinde kontrol edilmiştir (%9,3; laboratuvar %9,16; Δ_l = −%0,14) ve laboratuvarın tekrarlanabilirliği çalışmayla uyumludur (Tablo C.7).
2. Sabit kütleye kurutma çalışmanın kapsamında değildir: sabit süreli kurutma, uzatılmış kurutmanın 0,002 g yakınında kalır; bu dikdörtgen yarı genişlik 0,002/√3 = 0,00115 g, yani 1 g numunede %0,115 lif verir.
3. Üç aralıkta u = √(s_R² + 0,115²) = %0,31; 0,41 ve 0,59 (Tablo C.9), U = %0,6; 0,8 ve 1,2 (k = 2).

**Sonuç:**

- w(crude fibre) = (2,30 ± 0,63) % m/m (k = 2,00)
- w(crude fibre) = (4,00 ± 0,81) % m/m (k = 2,00)
- w(crude fibre) = (8,0 ± 1,2) % m/m (k = 2,00)

**Notlar:**

- ISO 21748 C.4.7, %0,115 belirsizliğinin 'bu en büyük tahmini sapmanın 3'e bölünmesiyle' bulunduğunu söyler; 0,002 g/3 %0,067 verirdi. Basılı değer √3'e bölmeye (dikdörtgen dağılım) karşılık gelir; proje bunu kullanır.
- QUAM A6 ('quam-a6-crude-fibre' projesi) yuvarlanmış s_R değerlerini (0,29; 0,40; 0,60) kullanır ve kurutma terimini yalnızca en düşük düzeyde dahil eder.

**Çalıştırma:** `uncertainty run library/chemistry/iso21748-c4-crude-fibre.json --lang tr`

### Çelikte karbon: [zaman + operatör] farklı ara kesinlik (ISO 5725-3 J.1)

`iso5725-3-j1-carbon-steel` — ISO 5725-3:2023, Annex J.1, Table J.1

ISO 5725-3:2023 J.1: her gün bir rutin numune ertesi gün başka bir analist tarafından yeniden analiz edilir, bir ayda 29 çift; çiftlerden s_I(TO), Formül (5).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Her çift (y_j1, y_j2) aynı numunenin iki günde iki analist tarafından ölçülmesidir; bu nedenle farklar tekrarlanabilirliğin yanında zaman ve operatör etkilerini de içerir (6.2.2).
2. 20 (%0,042/0,146) ve 24 (%0,100/0,161) numaralı çiftler çıkarılır: muhtemelen kayıt hatası olan farkları Cochran testine göre aykırıdır (29 çift arasında 20 için C = 0,72; 20 çıkarıldıktan sonra 24 için C = 0,89).
3. s_I(TO) = √(Σw_j²/(2 × 27)) = 2,87 × 10⁻³ % (kütle kesri), Formül (5), 27 serbestlik derecesiyle.

**Sonuç:**

- w(C) = (0,10000 ± 0,00574) % m/m (k = 2,00)

**Notlar:**

- Korunan 27 çift saklanmıştır; çıkarılan iki çift açıklamada verilmiştir. Belirtilen Cochran istatistikleri Tablo J.1'den hesaplanmıştır (ISO 5725-3 yalnızca sonucu verir).

**Çalıştırma:** `uncertainty run library/chemistry/iso5725-3-j1-carbon-steel.json --lang tr`

### Çelikte vanadyum: kademeli iç içe tasarım, düzey 1 (ISO 5725-3 J.2)

`iso5725-3-j2-vanadium-steel` — ISO 5725-3:2023, Annex J.2, Tables J.2–J.5

ISO 5725-3:2023 J.2: 20 laboratuvar, bir günde iki ve ertesi gün bir sonuç; varyans analizi s_r'yi, zaman farklı ara kesinlik s_I(T)'yi ve s_R'yi verir.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Her laboratuvar y₁, y₂ (tekrarlanabilirlik koşulları, 1. gün) ve y₃ (2. gün) bildirir. 20 numaralı laboratuvar düzey 1'de aykırıdır (Cochran) ve çıkarılır.
2. Ek C.1: w(1) = |y₁ − y₂|, w(2) = |(y₁ + y₂)/2 − y₃|; 18, 19 ve 19 serbestlik dereceli SS₀ = 3Σ(ȳᵢ − ȳ)², SS₁ = (2/3)Σw(2)², SS_e = (1/2)Σw(1)² (Tablo J.4).
3. Beklenen kareler ortalamalarından s_r² = MS_e = 0,145, s₍₁₎² = (3/4)(MS₁ − MS_e) = 0,218 ve s₀² = (MS₀ − MS_e − (5/3)s₍₁₎²)/3 = 0,278 (× 10⁻⁶): s_r = 0,381, s_I(T) = 0,603 ve s_R = 0,801 (× 10⁻³ %).

**Sonuç:**

- w(V) = (0,00980 ± 0,00121) % m/m (k = 2,00)

**Notlar:**

- ISO 5725-3:2023 Tablo J.2, ISO 5725-3:1994 Tablo D.2'den 15 girişte farklıdır (14'ü 3. düzeyde: 7–20. laboratuvarların 1. gündeki ikinci sonucu birincisinin tekrarıdır; birer tanesi 4. ve 5. düzeylerde); burada kullanılan 1. düzey her iki baskıda aynıdır. 1994 verileri Tablo J.5'i 1–5. düzeylerde tam olarak yeniden üretir (çekirdek testi staggered_nested_iso_5725_3_1994_levels_3_to_6). 6. düzeyde s₍₁₎² negatiftir: s_I(T) = s_r basılmıştır; ancak basılı s_R = 15,962 × 10⁻³ % negatif s₍₁₎²'yi tutar; her yerde s₍₁₎² = 0 alınırsa s_R = 16,331 × 10⁻³ % olur (araştırma günlüğü F40). Tablo J.5'teki 'Average (%)' sütunu fazladan bir 10⁻³ çarpanı taşır.

**Çalıştırma:** `uncertainty run library/chemistry/iso5725-3-j2-vanadium-steel.json --lang tr`

### İçme suyunda nitrat: iki kesinlik aralığı

`mupr-b1-nitrate` — Eurachem/CITAC Guide, Example B1

MUPR Örnek B1: 0,4 mg/L altında mutlak, üstünde bağıl kesinlik; 20 katkılı sudan geri kazanım; stok çözelti ve seyreltme bileşenleri.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Aralık I (c < 0,4 mg/L): s_I = 0,0328 mg/L; aralık II: s′_I = %8,43 (F = 100 seyreltme, u′_F = %2,2).
2. R̄ = 0,389/0,400 = %97,25, u(R̄) = %2,85; |1 − R̄|/u = 0,96 ≤ t₁₉ = 2,09: düzeltme yok, u′ = u(R̄).
3. U = 2√(s² + c²(u′²_R + 2u′²_DSS [+ u′²_F])): (0,261 ± 0,068) mg/L ve (50,0 ± 9,3) mg/L.

**Sonuç:**

- c(NO₃⁻) = (0,261 ± 0,068) mg/L (k = 2,00)
- c(NO₃⁻) = (50,0 ± 9,3) mg/L (k = 2,00)

**Notlar:**

- Bulgu: rehber R̄'yi 0,972'ye yuvarlayarak |1 − R̄|/u = 0,982 yazar; R̄ = 0,9725 ile istatistik 0,963'tür. Sonuç aynıdır.

**Çalıştırma:** `uncertainty run library/chemistry/mupr-b1-nitrate.json --lang tr`

### Nitrat: birleştirilmiş kesinlik ve iki geri kazanım düzeyi

`mupr-b2-pooled-precision` — Eurachem/CITAC Guide, Example B2, Tables B2.1–B2.3

MUPR Örnek B2: birden fazla malzemeden birleştirilmiş kesinlik (ikisi için ham veri), geçiş derişimi ve genel geri kazanım.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Birleştirilmiş s_I⟨I⟩ = 0,0285 mg/L (Eş. B2.1) ve s′_I⟨II⟩ = %7,73 (Eş. B2.2); iki model c_T = 0,0285/0,0773 = 0,37 mg/L'de buluşur.
2. %97,2 ve %102,0 geri kazanımları uyumludur; R̿ = %99,6, u = %2,24, anlamlı değil (38 serbestlik derecesi).
3. 50 mg/L'de: U = 0,169 c = 8,45 mg/L.

**Sonuç:**

- c(NO₃⁻) = (50,0 ± 8,4) mg/L (k = 2,00)

**Notlar:**

- Rehber yuvarlanmış s_I(2) = 0,0250'yi 0,3033'e böler (%8,24); yuvarlanmamış %8,25. Grup 2 mutlak birleştirmeye girdiğinden bütçe etkilenmez.

**Çalıştırma:** `uncertainty run library/chemistry/mupr-b2-pooled-precision.json --lang tr`

### Nitrat: 2 günde 3 tekrarın ortalaması olarak sonuç

`mupr-b2-replicates-on-two-days` — Eurachem/CITAC Guide, Example B2, Eq. (B2.8)–(B2.9)

MUPR Örnek B2, Eş. (B2.9): p günün her birinde n sonucun ortalamasının kesinliği (Eş. 5).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. s′_I(x̿) = √(s′_I²/p + s′_r²(1 − n)/(n p)); s′_I = %7,73, s′_r = %3,4, n = 3, p = 2.
2. 57,5 mg/L'de: U = 7,06 mg/L (iki günün her birinde tek sonuç için 7,41 mg/L).

**Sonuç:**

- c(NO₃⁻) = (57,5 ± 7,0) mg/L (k = 2,00)

**Çalıştırma:** `uncertainty run library/chemistry/mupr-b2-replicates-on-two-days.json --lang tr`

### Nitrat: derişimin doğrusal fonksiyonu olarak kesinlik

`mupr-b3-linear-precision` — Eurachem/CITAC Guide, Example B3, Table B3.1, Eq. (B3.1)

MUPR Örnek B3: dört malzemenin ara kesinliği en küçük kareler ile s_I = a + b·c olarak modellenir ve seyreltilmiş bir numuneye uygulanır.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Tablo B3.1: (c, s_I) = (0,389; 0,0328), (0,303; 0,0250), (0,204; 0,0245), (0,785; 0,0322) mg/L, s_I = 0,0232 + 0,0128 c (r = 0,727) verir.
2. Numune 100 kat seyreltildiğinden model test çözeltisine uygulanır: s(c_s) = 100·(a + b·c_s/100); c_s = 57,5 mg/L'de s = 3,06 mg/L.
3. Geri kazanım (%2,24), stok çözelti (2 × %0,95) ve seyreltme (%2,2) bileşenleriyle: U = 7,28 mg/L.

**Sonuç:**

- c(NO₃⁻) = (57,5 ± 7,3) mg/L (k = 2,00)

**Notlar:**

- Bulgu: en küçük kareler kesim noktasını 0,02324 verir (basılı 0,0233); eğim 0,0128 ve r = 0,7266 uyumludur.

**Çalıştırma:** `uncertainty run library/chemistry/mupr-b3-linear-precision.json --lang tr`

### Sedimentte arsenik: tek CRM

`mupr-b4-arsenic-one-crm` — Eurachem/CITAC Guide, Example B4

MUPR Örnek B4: birleştirilmiş bağıl kesinlik ve tek CRM'nin geri kazanımı.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Üç malzemeden birleştirilmiş s′_I(P) = %4,52 (Eş. B4.1); 6,46 mg/kg altında mutlak 4,52 % × 6,46 = 0,292 mg/kg olarak kullanılır.
2. CRM WQB-1 (23,00 ± 1,84) mg/kg, ortalama 21,040 mg/kg: R̄ = %91,5, u = %3,81; 2,24 ≤ t₉ = 2,26: düzeltilmez.
3. U = 2w√(0,0452² + 0,0381²) = 0,118 w: (16,0 ± 1,9) mg/kg.

**Sonuç:**

- w(As) = (16,0 ± 1,9) mg/kg (k = 2,00)

**Çalıştırma:** `uncertainty run library/chemistry/mupr-b4-arsenic-one-crm.json --lang tr`

### Arsenik: matrise bağlı geri kazanımlı CRM ve katkılı numuneler

`mupr-b5-crm-and-spiked-samples` — Eurachem/CITAC Guide, Example B5

MUPR Örnek B5: geri kazanımlar matrisle değiştiğinden standart sapmaları dahil edilir ve k = t(2).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. CRM ve iki katkılı sedimentten R̿ = %94,98; s(R) = %7,32, s(R) dahil u(R̿) = %8,00 (Eş. B5.1–B5.3).
2. |1 − R̿|/u = 0,63 ≤ t₂ = 4,30: düzeltilmez; ν = 2 ile kapsam faktörü t₀,₉₇₅(2) = 4,30 ve U = 0,395 w.

**Sonuç:**

- w(As) = (16,0 ± 6,3) mg/kg (k = 4,30)

**Çalıştırma:** `uncertainty run library/chemistry/mupr-b5-crm-and-spiked-samples.json --lang tr`

### Sedimentte iz element: heterojenlik ve 22 yeterlilik testi

`mupr-b6-sediment-22-pt` — Eurachem/CITAC Guide, Example B6, Tables B6.1–B6.3

MUPR Örnek B6: homojen bir malzemeden kesinlik ve gerçek numunelerin heterojenliği (mükerrer açıklıklar), 22 YT sonucundan geri kazanım (ham veri).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. 30 kez ölçülen homojen bir malzeme s′_I(DS) = %7,34 verir; 30 mükerrer gerçek numunenin açıklıkları 7,5 mg/kg altında s_r(h) = Ā/1,128 = 0,191 mg/kg ve üstünde s′_r(h) = %2,66 verir.
2. Aralık I: s_I = √((7,5 × %7,34)² + 0,191²) = 0,583 mg/kg; aralık II: s′_I = √(7,34² + 2,66²) = %7,81.
3. 22 YT geri kazanımı eşdeğerdir (χ² = 8,70 ≤ 32,7), bu nedenle s(R) eklenmez; R̿ = %91,7, u = %1,59, |1 − R̿|/u = 5,2 > t₂₁: sonuçlar düzeltilir (w/R̿) ve u′ = u/R̿.
4. Aralık II: U = 2(w/R̿)√(0,0781² + 0,0173²) = 0,160 w/R̿.

**Sonuç:**

- w = (5,4 ± 1,3) mg/kg (k = 2,00)
- w = (21,8 ± 3,5) mg/kg (k = 2,00)

**Notlar:**

- Bulgu: Eş. (B6.7) χ²'yi ağırlıksız ortalamaya göre hesaplar (8,80); Eş. (16) ağırlıklı ortalamayı öngörür ve 8,70 verir. Sonuç aynıdır.
- Bulgu: Eş. (B6.8) basılı haliyle düzeltilmiş sonucun kesinlik teriminde 1/R̿'yi atlar; program mutlak bileşenler için u(w/R̿)² = (s_I/R̿)² + (w u_R̿/R̿²)² kullanır. Aralık II'de (tümü bağıl) ikisi aynıdır.
- Her tekil YT sonucunun standart sapması, Tablo B6.3'teki gibi homojen malzeme modelinden alınmıştır (7,5 mg/kg üzerinde %7,34, altında 7,5 × %7,34).

**Çalıştırma:** `uncertainty run library/chemistry/mupr-b6-sediment-22-pt.json --lang tr`

### İki CRM ve bir yeterlilik testi numunesinden geri kazanım

`mupr-ex2-reference-materials` — Eurachem/CITAC Guide, Example 2

MUPR Örnek 2: orto-fosfat; üç referans malzemeden ortalama geri kazanım ve belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Rᵢ = c̄ᵢ/Cᵢ, u²(Rᵢ) = Rᵢ²[(sᵢ/(c̄ᵢ√nᵢ))² + (u(Cᵢ)/Cᵢ)²] (Eş. 9); terimler 0,000731, 0,001476, 0,01015.
2. R̄ = 1,008, u(R̄) = 0,0371: 1'den anlamlı olarak farklı değildir.
3. Ortalama geri kazanım R̄ = ΣRᵢ/N'nin belirsizliği u(R̄) = √Σu²(Rᵢ)/N'dir (MUPR Eş. 8–12); yalnızca |1 − R̄|/u(R̄) iki taraflı t değerini aşarsa düzeltme yapılır (Eş. 13).

**Sonuç:**

- U = %7,4 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/chemistry/mupr-ex2-reference-materials.json --lang tr`

### Katkılı nehir sularından geri kazanım

`mupr-ex3-spiked-samples` — Eurachem/CITAC Guide, Example 3

MUPR Örnek 3: doğal analit içeren iki katkılı numune (Eş. 10–11).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Rᵢ = (c̄ − c̄₀)/c₊: %103,6 ve %88,8; B'nin belirsizliği (katkıdan önce tek sonuç, küçük katkı) baskındır.
2. R̄ = %96,2, u(R̄) = %10,0 (yalnız A numunesiyle %1,5).
3. Ortalama geri kazanım R̄ = ΣRᵢ/N'nin belirsizliği u(R̄) = √Σu²(Rᵢ)/N'dir (MUPR Eş. 8–12); yalnızca |1 − R̄|/u(R̄) iki taraflı t değerini aşarsa düzeltme yapılır (Eş. 13).

**Sonuç:**

- U = %20 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/chemistry/mupr-ex3-spiked-samples.json --lang tr`

### Kontrol sınırları ve yeterlilik testlerinden sudaki amonyum

`nordtest-3-ammonium-pt` — Nordtest TR 537 ed. 4 (2017), §3.2 and Appendix 4

Nordtest TR 537 §3.2 ve Ek 4: NH₄-N; u(Rw) kontrol grafiğinden, u(bias) altı yeterlilik testinden (ham YT verisi).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Kontrol grafiği uyarı sınırları ±%3,34'tür (2s); bu nedenle u(Rw) = 3,34/2 = %1,67.
2. Her YT turu için sapma (x_lab − x_atanan)/x_atanan, atanan değerin belirsizliği u(C_ref,i) = s_R/√n_Lab'dır; RMS_bias = %2,26, ū(C_ref) = %1,52.
3. u(bias) = √(2,26² + 1,52²) = %2,73; u_c = √(1,67² + 2,73²) = %3,20; U = %6,4 → %7.
4. u_c = √(u(Rw)² + u(bias)²) ve U = 2u_c; tam yüzdeye yukarı yuvarlanır (Nordtest §3).

**Sonuç:**

- U = %7 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-3-ammonium-pt.json --lang tr`

### Amonyum: kontrol numuneleri ve mükerrer ölçümlerden laboratuvar içi tekrarüretilebilirlik

`nordtest-5.3-ammonium-duplicates` — Nordtest TR 537 ed. 4 (2017), §5.3 and Appendix 5

Nordtest TR 537 §5.3 ve Ek 5: doğal numunelerin iki aralıktaki NH₄-N mükerrer ölçümleri (ham veri), kontrol numunesi tekrarüretilebilirliğiyle birleştirilmiştir.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Kontrol numunesi numuneden numuneye değişimi kapsamadığından gerçek numunelerin mükerrer ölçümlerinden gelen tekrarlanabilirlik eklenir: u(Rw) = √(u(Rw, kontrol)² + s_r(mükerrer)²).
2. 30 µg/L'nin altında standart sapma sabittir: 46 çiftten s_r = √(Σd²/2N) = 0,44 µg/L; kontrol numunesiyle (0,5 µg/L) u(Rw) = 0,67 µg/L.
3. 30 µg/L ve üzerinde derişimle orantılıdır: 26 çiftten birleştirilmiş bağıl s_r = %3,8; kontrol numunesiyle (%1,5) u(Rw) = %4,1.
4. Her bileşen kendi aralığıyla sınırlandırılmıştır; bütçe 10 ve 100 µg/L'de gösterilmiştir.

**Sonuç:**

- c(NH₄-N) = (10,0 ± 1,3) µg/L (k = 2,00)
- c(NH₄-N) = (100,0 ± 8,2) µg/L (k = 2,00)

**Notlar:**

- Ek 5, 30 µg/L'nin altında 46 çift listeler; §5.3 n = 47 yazar. İstatistikler listelenen 46 çiftten yeniden hesaplanmıştır.

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-5.3-ammonium-duplicates.json --lang tr`

### Çözünmüş oksijen: mükerrer ölçümlerden tekrarüretilebilirlik

`nordtest-5.3-oxygen-duplicates` — Nordtest TR 537 ed. 4 (2017), §5.3 and Appendix 6

Nordtest TR 537 §5.3 ve Ek 6: deniz suyunda çözünmüş oksijen, 51 mükerrer çift (ham veri) ve kontrol numunesi bileşeni.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. s_r = √(Σd²/2N) = 0,0252 mg/L, ortalama 7,50 mg/L'de, yani %0,34.
2. Kontrol numunesi tekrarüretilebilirliği (%0,5) ile u(Rw) = √(0,34² + 0,5²) = %0,60.

**Sonuç:**

- U = %1,2 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-5.3-oxygen-duplicates.json --lang tr`

### Tek sertifikalı referans malzemeden sapma

`nordtest-6.1-one-crm` — Nordtest TR 537 ed. 4 (2017), §6.1

Nordtest TR 537 §6.1: 12 kez ölçülmüş bir CRM'den (11,5 ± 0,5) u(bias).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Sapma b = (11,9 − 11,5)/11,5 = %3,48; s_bias = %2,2 (n = 12); u(C_ref) = 0,25/11,5 = %2,17.
2. Tek referans malzeme için u(bias) = √(b² + (s_bias/√n)² + u(C_ref)²) = %4,15.

**Sonuç:**

- U = %8,3 (k = 2,00), ölçülen değere göre bağıl

**Notlar:**

- Bulgu (F5): el kitabı u(C_ref) = 0,25/11,5 = %2,16 yazar, oysa 0,25/11,5 = %2,17'dir; u(bias) = %4,1 sonucu %2,16'dan elde edilir, doğru değer %4,15'tir.

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-6.1-one-crm.json --lang tr`

### Birden fazla sertifikalı referans malzemeden sapma

`nordtest-6.1-three-crms` — Nordtest TR 537 ed. 4 (2017), §6.1

Nordtest TR 537 §6.1: üç CRM'den sapmaların karesel ortalamasının karekökü (RMS) ile u(bias).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Sapmalar %3,48, −0,9 ve 2,5: RMS_bias = √(Σbᵢ²/N) = %2,53; ū(C_ref) = (2,16 + 1,8 + 1,8)/3 = %1,92.
2. u(bias) = √(RMS_bias² + ū(C_ref)²) = %3,2.

**Sonuç:**

- U = %6,3 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-6.1-three-crms.json --lang tr`

### Yeterlilik testlerinden sapma

`nordtest-6.2-proficiency-tests` — Nordtest TR 537 ed. 4 (2017), §6.2

Nordtest TR 537 §6.2: altı yeterlilik testinden u(bias).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. u(C_ref,1) = s_R/√n_Lab = 8,7/√23 = %1,8; diğer beş değer %2,9, 1,7, 4,1, 3,0 ve 2,1; ū(C_ref) = %2,6.
2. Sapmalar +2, +7, −2, +3, +6, +5 %: RMS_bias = %4,6; u(bias) = √(4,6² + 2,6²) = %5,3.

**Sonuç:**

- U = %10 (k = 2,00), ölçülen değere göre bağıl

**Notlar:**

- Bulgu: altı u(C_ref,i) değerinin ortalaması %2,60'tır; el kitabı adım tablosunda %2,7 yazar, ardından %2,6 kullanır.

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-6.2-proficiency-tests.json --lang tr`

### Geri kazanım deneylerinden sapma

`nordtest-6.3-recovery` — Nordtest TR 537 ed. 4 (2017), §6.3

Nordtest TR 537 §6.3: altı katkı geri kazanımından u(bias).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Geri kazanımlar %95, 98, 97, 96, 99, 96: sapmalar R − %100, RMS_bias = %3,44.
2. Eklenen miktarın belirsizliği: katkı çözeltisi derişimi 1,2/2 = %0,6, hacim (±%1 dikdörtgen → 1/√3 = %0,58 ve %0,5) → u(C_geri kazanım) = √(0,6² + 0,58² + 0,5²) = %0,97 ≈ %1,0.
3. u(bias) = √(3,44² + 0,97²) = %3,6.

**Sonuç:**

- U = %7,1 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-6.3-recovery.json --lang tr`

### Atık suda BOİ: kontrol grafiği ve tek CRM

`nordtest-8-bod-crm` — Nordtest TR 537 ed. 4 (2017), §8.2 Example A, Appendix 7

Nordtest TR 537 §8.2 Örnek A, Ek 7'deki ham kontrol verileriyle.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. CRM'nin 18 mükerrer kontrol analizinin ortalamaları s = 5,6 mg/L, yani 214,8 mg/L'de u(Rw) = %2,6 verir.
2. Aynı sonuçlar sertifikalı değere (206 ± 5) mg/L karşı sapmayı verir: b = %4,3, u(C_ref) = 2,5/206 = %1,2, u(bias) = √(b² + s²/n + u(C_ref)²) = %4,5.
3. u_c = √(2,6² + 4,5²) = %5,2; U = %10,4 → %11.

**Sonuç:**

- U = %11 (k = 2,00), ölçülen değere göre bağıl

**Notlar:**

- El kitabı s_bias için n = 19 yazar; Ek 7 burada kullanılan 18 çifti listeler. Ortalamalar çiftlerden hesaplanmıştır (ek bunları 1 mg/L'ye yuvarlanmış verir).

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-8-bod-crm.json --lang tr`

### Atık suda BOİ: kontrol grafiği ve yeterlilik testleri

`nordtest-8-bod-pt` — Nordtest TR 537 ed. 4 (2017), §8.2 Example B, Table 6

Nordtest TR 537 §8.2 Örnek B: üç yeterlilik testinden u(bias) (Tablo 6, ham veri).

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. u(Rw) = %2,6, Örnek A'daki gibi.
2. u(C_ref,i) = s_R/√n: %1,50, 1,32, 2,25 (ortalama %1,69); sapmaların RMS'i %3,77 (Tablo 6'daki yuvarlanmış sapmalardan %3,76).
3. u(bias) = %4,13, u_c = %4,88, U = %9,8 → %10.

**Sonuç:**

- U = %10 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-8-bod-pt.json --lang tr`

### Sedimentte PCB: kontrol numunesi ve tek CRM

`nordtest-8-pcb-crm` — Nordtest TR 537 ed. 4 (2017), §8.3 Example C

Nordtest TR 537 §8.3 Örnek C.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Kontrol grafiğinden u(Rw) = %8; CRM (152 ± 14) µg/kg, ortalama sonuç 144 µg/kg: b = −%5,3, s_bias = %8 (n = 22), u(C_ref) = 7/152 = %4,6.
2. u(bias) = %7,2, u_c = √(8² + 7,2²) = %10,8, U = %21,5 → %22.

**Sonuç:**

- U = %22 (k = 2,00), ölçülen değere göre bağıl

**Notlar:**

- Verilen değerlerden b = %5,26 ve u(C_ref) = %4,61 ile u(bias) = %7,20 bulunur; el kitabı yuvarlanmış %5,3 ve %4,6 değerlerini kullanır (%7,22). Özet tablosu u(C_ref) = %4,7 ve u(bias) = %7,29 yazar.

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-8-pcb-crm.json --lang tr`

### Sedimentte PCB: kontrol numunesi ve yeterlilik testleri

`nordtest-8-pcb-pt` — Nordtest TR 537 ed. 4 (2017), §8.3 Example D

Nordtest TR 537 §8.3 Örnek D.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Sapmalar −2, −12, −5 %: RMS_bias = %7,6; u(C_ref,i) %2,7, 2,5, 3,5 → %2,9; u(bias) = %8,1.
2. u_c = √(8² + 8,1²) = %11,4, U = %22,8 → %23.

**Sonuç:**

- U = %23 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/chemistry/nordtest-8-pcb-pt.json --lang tr`

### Deniz sedimentlerinde krom, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-cr-marine-sediment` — C. Palma, Table 9

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Cr belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. İki belirsizlik modeli: tespit sınırından 2w_LOQ = 10 mg/kg'a kadar Aralık I (mutlak bileşenler), 2w_LOQ üzerinde Aralık II (bağıl bileşenler); OSPAR yöntemiyle toplam Cr, alevli AAS.
2. Kesinlik: gerçek sedimentlerin ikili analizlerinin alt örneklemeyi de içeren tekrarlanabilirliği, s_r(h) = 0,285 mg/kg (I) ve %2,85 (II) (ortalama aralık/1,128), ve homojen bir numunenin ara kesinliği, s_I = 0,565 mg/kg (I) ve %5,65 (II).
3. Matris etkileri: yeterlilik testlerinin sedimentlerinde Cr geri kazanımları ara kesinliğin açıklayabileceğinden fazla değişir; s_ME = √(s′(R)² − s_I²) = 0,818 mg/kg (I) ve %8,18 (II) (Formül 16, 17).
4. Doğruluk: ağırlıklı ortalama geri kazanım R̄_w = %108, u(R̄) = %1,89; yani sonucun u′_T = %1,75'si; Aralık I'de 2w_LOQ'da alınır: 0,175 mg/kg.
5. Aralık I'de u_c = 1,05 mg/kg ve Aralık II'de u′_c = %10,49 (Formül 20, 21); U = 2u_c.
6. Matris etkileri varyansın %60,8 (I) ve %60,8 (II) kadarını oluşturur (raporun varyans payları; Tablo 9'daki χ_ME: %60,8 ve %60,9).

**Sonuç:**

- w(Cr) = (5,0 ± 2,1) mg/kg (k = 2,00)
- w(Cr) = (50 ± 10) mg/kg (k = 2,00)

**Notlar:**

- Proje makalenin Tablo 9'da özetlenen bileşenlerini birleştirir; ham veriler (tüm ikili çiftler ve yeterlilik testi sonuçları) makalenin elektronik ekindedir. Geri kazanım %100den anlamlı olarak farklıdır (Formül 10), bu nedenle sonuçlar 1/R̄_w ile düzeltilir.

**Çalıştırma:** `uncertainty run library/chemistry/palma2019-cr-marine-sediment.json --lang tr`

### Deniz sedimentlerinde bakır, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-cu-marine-sediment` — C. Palma, Table 9

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Cu belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. İki belirsizlik modeli: tespit sınırından 2w_LOQ = 10 mg/kg'a kadar Aralık I (mutlak bileşenler), 2w_LOQ üzerinde Aralık II (bağıl bileşenler); OSPAR yöntemiyle toplam Cu, alevli AAS.
2. Kesinlik: gerçek sedimentlerin ikili analizlerinin alt örneklemeyi de içeren tekrarlanabilirliği, s_r(h) = 0,242 mg/kg (I) ve %2,43 (II) (ortalama aralık/1,128), ve homojen bir numunenin ara kesinliği, s_I = 0,248 mg/kg (I) ve %2,48 (II).
3. Matris etkileri: yeterlilik testlerinin sedimentlerinde Cu geri kazanımları ara kesinliğin açıklayabileceğinden fazla değişir; s_ME = √(s′(R)² − s_I²) = 0,493 mg/kg (I) ve %4,92 (II) (Formül 16, 17).
4. Doğruluk: ağırlıklı ortalama geri kazanım R̄_w = %99,9, u(R̄) = %1,11; yani sonucun u′_T = %1,11'si; Aralık I'de 2w_LOQ'da alınır: 0,111 mg/kg.
5. Aralık I'de u_c = 0,613 mg/kg ve Aralık II'de u′_c = %6,12 (Formül 20, 21); U = 2u_c.
6. Matris etkileri varyansın %64,7 (I) ve %64,6 (II) kadarını oluşturur (raporun varyans payları; Tablo 9'daki χ_ME: %64,7 ve %64,5).

**Sonuç:**

- w(Cu) = (5,0 ± 1,2) mg/kg (k = 2,00)
- w(Cu) = (50,0 ± 6,1) mg/kg (k = 2,00)

**Notlar:**

- Proje makalenin Tablo 9'da özetlenen bileşenlerini birleştirir; ham veriler (tüm ikili çiftler ve yeterlilik testi sonuçları) makalenin elektronik ekindedir. Geri kazanım %100 ile uyumludur, bu nedenle sonuçlar düzeltilmez.

**Çalıştırma:** `uncertainty run library/chemistry/palma2019-cu-marine-sediment.json --lang tr`

### Deniz sedimentlerinde lityum, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-li-marine-sediment` — C. Palma, Table 9

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Li belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. İki belirsizlik modeli: tespit sınırından 2w_LOQ = 1 mg/kg'a kadar Aralık I (mutlak bileşenler), 2w_LOQ üzerinde Aralık II (bağıl bileşenler); OSPAR yöntemiyle toplam Li, alevli AAS.
2. Kesinlik: gerçek sedimentlerin ikili analizlerinin alt örneklemeyi de içeren tekrarlanabilirliği, s_r(h) = 0,0345 mg/kg (I) ve %3,45 (II) (ortalama aralık/1,128), ve homojen bir numunenin ara kesinliği, s_I = 0,058 mg/kg (I) ve %5,8 (II).
3. Matris etkileri: yeterlilik testlerinin sedimentlerinde Li geri kazanımları ara kesinliğin açıklayabileceğinden fazla değişir; s_ME = √(s′(R)² − s_I²) = 0,1 mg/kg (I) ve %10 (II) (Formül 16, 17).
4. Doğruluk: ağırlıklı ortalama geri kazanım R̄_w = %102, u(R̄) = %3,95; yani sonucun u′_T = %3,87'si; Aralık I'de 2w_LOQ'da alınır: 0,0387 mg/kg.
5. Aralık I'de u_c = 0,127 mg/kg ve Aralık II'de u′_c = %12,67 (Formül 20, 21); U = 2u_c.
6. Matris etkileri varyansın %62,3 (I) ve %62,3 (II) kadarını oluşturur (raporun varyans payları; Tablo 9'daki χ_ME: %62,3 ve %62,4).

**Sonuç:**

- w(Li) = (0,50 ± 0,25) mg/kg (k = 2,00)
- w(Li) = (5,0 ± 1,3) mg/kg (k = 2,00)

**Notlar:**

- Proje makalenin Tablo 9'da özetlenen bileşenlerini birleştirir; ham veriler (tüm ikili çiftler ve yeterlilik testi sonuçları) makalenin elektronik ekindedir. Geri kazanım %100 ile uyumludur, bu nedenle sonuçlar düzeltilmez.

**Çalıştırma:** `uncertainty run library/chemistry/palma2019-li-marine-sediment.json --lang tr`

### Deniz sedimentlerinde mangan, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-mn-marine-sediment` — C. Palma, Table 9

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Mn belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. İki belirsizlik modeli: tespit sınırından 2w_LOQ = 10 mg/kg'a kadar Aralık I (mutlak bileşenler), 2w_LOQ üzerinde Aralık II (bağıl bileşenler); OSPAR yöntemiyle toplam Mn, alevli AAS.
2. Kesinlik: gerçek sedimentlerin ikili analizlerinin alt örneklemeyi de içeren tekrarlanabilirliği, s_r(h) = 0,179 mg/kg (I) ve %1,79 (II) (ortalama aralık/1,128), ve homojen bir numunenin ara kesinliği, s_I = 0,656 mg/kg (I) ve %6,56 (II).
3. Matris etkileri: yeterlilik testlerinin sedimentlerinde Mn geri kazanımları ara kesinliğin açıklayabileceğinden fazla değişir; s_ME = √(s′(R)² − s_I²) = 0,6 mg/kg (I) ve %6 (II) (Formül 16, 17).
4. Doğruluk: ağırlıklı ortalama geri kazanım R̄_w = %97,6, u(R̄) = %1,5; yani sonucun u′_T = %1,54'si; Aralık I'de 2w_LOQ'da alınır: 0,154 mg/kg.
5. Aralık I'de u_c = 0,92 mg/kg ve Aralık II'de u′_c = %9,20 (Formül 20, 21); U = 2u_c.
6. Matris etkileri varyansın %42,6 (I) ve %42,6 (II) kadarını oluşturur (raporun varyans payları; Tablo 9'daki χ_ME: %42,6 ve %42,6).

**Sonuç:**

- w(Mn) = (5,0 ± 1,8) mg/kg (k = 2,00)
- w(Mn) = (50,0 ± 9,2) mg/kg (k = 2,00)

**Notlar:**

- Proje makalenin Tablo 9'da özetlenen bileşenlerini birleştirir; ham veriler (tüm ikili çiftler ve yeterlilik testi sonuçları) makalenin elektronik ekindedir. Geri kazanım %100 ile uyumludur, bu nedenle sonuçlar düzeltilmez.

**Çalıştırma:** `uncertainty run library/chemistry/palma2019-mn-marine-sediment.json --lang tr`

### Deniz sedimentlerinde nikel, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-ni-marine-sediment` — C. Palma, Table 9

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde asitte ekstrakte edilebilir Ni belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. İki belirsizlik modeli: tespit sınırından 2w_LOQ = 15 mg/kg'a kadar Aralık I (mutlak bileşenler), 2w_LOQ üzerinde Aralık II (bağıl bileşenler); EPA 3051A yöntemiyle asitte ekstrakte edilebilir Ni, alevli AAS.
2. Kesinlik: gerçek sedimentlerin ikili analizlerinin alt örneklemeyi de içeren tekrarlanabilirliği, s_r(h) = 0,346 mg/kg (I) ve %2,58 (II) (ortalama aralık/1,128), ve homojen bir numunenin ara kesinliği, s_I = 1,13 mg/kg (I) ve %7,54 (II).
3. Matris etkileri: yeterlilik testlerinin sedimentlerinde Ni geri kazanımları ara kesinliğin açıklayabileceğinden fazla değişir; s_ME = √(s′(R)² − s_I²) = 0,352 mg/kg (I) ve %2,35 (II) (Formül 16, 17).
4. Doğruluk: ağırlıklı ortalama geri kazanım R̄_w = %87,7, u(R̄) = %1,31; yani sonucun u′_T = %1,49'si; Aralık I'de 2w_LOQ'da alınır: 0,224 mg/kg.
5. Aralık I'de u_c = 1,25 mg/kg ve Aralık II'de u′_c = %8,44 (Formül 20, 21); U = 2u_c.
6. Matris etkileri varyansın %7,9 (I) ve %7,7 (II) kadarını oluşturur (raporun varyans payları; Tablo 9'daki χ_ME: %7,9 ve %7,7).

**Sonuç:**

- w(Ni) = (7,5 ± 2,5) mg/kg (k = 2,00)
- w(Ni) = (75 ± 13) mg/kg (k = 2,00)

**Notlar:**

- Proje makalenin Tablo 9'da özetlenen bileşenlerini birleştirir; ham veriler (tüm ikili çiftler ve yeterlilik testi sonuçları) makalenin elektronik ekindedir. Geri kazanım %100den anlamlı olarak farklıdır (Formül 10), bu nedenle sonuçlar 1/R̄_w ile düzeltilir.

**Çalıştırma:** `uncertainty run library/chemistry/palma2019-ni-marine-sediment.json --lang tr`

### Deniz sedimentlerinde kurşun, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-pb-marine-sediment` — C. Palma, Table 9, Section 4.1

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde asitte ekstrakte edilebilir Pb belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. İki belirsizlik modeli: tespit sınırından 2w_LOQ = 20 mg/kg'a kadar Aralık I (mutlak bileşenler), 2w_LOQ üzerinde Aralık II (bağıl bileşenler); EPA 3051A yöntemiyle asitte ekstrakte edilebilir Pb, alevli AAS.
2. Kesinlik: gerçek sedimentlerin ikili analizlerinin alt örneklemeyi de içeren tekrarlanabilirliği, s_r(h) = 0,714 mg/kg (I) ve %4,95 (II) (ortalama aralık/1,128), ve homojen bir numunenin ara kesinliği, s_I = 0,804 mg/kg (I) ve %4,02 (II).
3. Matris etkileri: yeterlilik testlerinin sedimentlerinde Pb geri kazanımları ara kesinliğin açıklayabileceğinden fazla değişir; s_ME = √(s′(R)² − s_I²) = 1,53 mg/kg (I) ve %7,65 (II) (Formül 16, 17).
4. Doğruluk: ağırlıklı ortalama geri kazanım R̄_w = %91,9, u(R̄) = %1,41; yani sonucun u′_T = %1,53'si; Aralık I'de 2w_LOQ'da alınır: 0,307 mg/kg.
5. Aralık I'de u_c = 1,9 mg/kg ve Aralık II'de u′_c = %10,08 (Formül 20, 21); U = 2u_c.
6. Matris etkileri varyansın %65,2 (I) ve %57,6 (II) kadarını oluşturur (raporun varyans payları; Tablo 9'daki χ_ME: %65,2 ve %57,6).

**Sonuç:**

- w(Pb) = (10,0 ± 3,8) mg/kg (k = 2,00)
- w(Pb) = (100 ± 20) mg/kg (k = 2,00)

**Notlar:**

- Proje makalenin Tablo 9'da özetlenen bileşenlerini birleştirir; ham veriler (tüm ikili çiftler ve yeterlilik testi sonuçları) makalenin elektronik ekindedir. Geri kazanım %100den anlamlı olarak farklıdır (Formül 10), bu nedenle sonuçlar 1/R̄_w ile düzeltilir.
- Kurşun için metin u′_P = %6,38, Tablo 9 ise %6,37 verir; √((5,583/1,128)² + 4,02²) = %6,38'dir. Metin 60 mg/kg'da u_c = 6,04 mg/kg verir; oysa %10,08 × 60 mg/kg = 6,05 mg/kg'dır.

**Çalıştırma:** `uncertainty run library/chemistry/palma2019-pb-marine-sediment.json --lang tr`

### Deniz sedimentlerinde çinko, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-zn-marine-sediment` — C. Palma, Table 9

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Zn belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. İki belirsizlik modeli: tespit sınırından 2w_LOQ = 4 mg/kg'a kadar Aralık I (mutlak bileşenler), 2w_LOQ üzerinde Aralık II (bağıl bileşenler); OSPAR yöntemiyle toplam Zn, alevli AAS.
2. Kesinlik: gerçek sedimentlerin ikili analizlerinin alt örneklemeyi de içeren tekrarlanabilirliği, s_r(h) = 0,108 mg/kg (I) ve %2,69 (II) (ortalama aralık/1,128), ve homojen bir numunenin ara kesinliği, s_I = 0,117 mg/kg (I) ve %2,92 (II).
3. Matris etkileri: yeterlilik testlerinin sedimentlerinde Zn geri kazanımları ara kesinliğin açıklayabileceğinden fazla değişir; s_ME = √(s′(R)² − s_I²) = 0,324 mg/kg (I) ve %8,1 (II) (Formül 16, 17).
4. Doğruluk: ağırlıklı ortalama geri kazanım R̄_w = %102, u(R̄) = %1,45; yani sonucun u′_T = %1,42'si; Aralık I'de 2w_LOQ'da alınır: 0,0569 mg/kg.
5. Aralık I'de u_c = 0,365 mg/kg ve Aralık II'de u′_c = %9,13 (Formül 20, 21); U = 2u_c.
6. Matris etkileri varyansın %78,6 (I) ve %78,7 (II) kadarını oluşturur (raporun varyans payları; Tablo 9'daki χ_ME: %78,7 ve %78,6).

**Sonuç:**

- w(Zn) = (2,00 ± 0,73) mg/kg (k = 2,00)
- w(Zn) = (20,0 ± 3,6) mg/kg (k = 2,00)

**Notlar:**

- Proje makalenin Tablo 9'da özetlenen bileşenlerini birleştirir; ham veriler (tüm ikili çiftler ve yeterlilik testi sonuçları) makalenin elektronik ekindedir. Geri kazanım %100 ile uyumludur, bu nedenle sonuçlar düzeltilmez.

**Çalıştırma:** `uncertainty run library/chemistry/palma2019-zn-marine-sediment.json --lang tr`

### Validasyon verilerinden ekmekte organofosforlu pestisitler

`quam-a4-pesticides-topdown` — EURACHEM/CITAC Guide CG 4, Example A4, Tables A4.2–A4.5

EURACHEM QUAM Örnek A4, top-down değerlendirme olarak: 15 mükerrer ölçümden kesinlik (ham veri), 42 katkıdan geri kazanım, homojenlik.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Farklı serilerde ölçülen mükerrerler: normalize farkların (x₁ − x₂)/x̄ standart sapması 0,382'dir; bağıl ara kesinlik 0,382/√2 = %27'dir.
2. Geri kazanım: 42 katkıdan ortalama %90, s = %28, u = 28/√42 = %4,3; t = 2,31 > t₄₁ = 2,02: sonuç düzeltilir ve u′ = 4,3/90 = %4,8.
3. Somunun homojensizliği (binom model, senaryo b): %20.
4. Ölçülen düzey 1'de: P = 1/0,90 = 1,111, u = 1,111 × √(0,27² + 0,048² + 0,20²) = 0,377.

**Sonuç:**

- P_op = (1,11 ± 0,75) (k = 2,00)

**Notlar:**

- Bulgu: Tablo A4.2'nin türetilmiş sütunlarında yazım hataları vardır (0,075 yerine ortalama 0,75, −0,01 yerine fark −0,10, 0,01 yerine ortalama 0,10); istatistikler D1 ve D2'den yeniden hesaplanmıştır. Rehber 41 serbestlik derecesi için t_kritik = 2,021, yani t(40) kullanır (t(41) = 2,0195).

**Çalıştırma:** `uncertainty run library/chemistry/quam-a4-pesticides-topdown.json --lang tr`

### Hayvan yeminde ham lif (ampirik yöntem)

`quam-a6-crude-fibre` — EURACHEM/CITAC Guide CG 4, Example A6, Tables A6.4–A6.5

EURACHEM QUAM Örnek A6: üç düzeyde ortak çalışmadan tekrarüretilebilirlik s_R ve kurutma payı.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Ampirik yöntemde ortak çalışma tekrarüretilebilirliği s_R (Tablo A6.4) ana bileşendir; değerler % m/m (mutlak) biriminde olduğundan alanlarda _percent soneki yoktur.
2. Sabit tartıma kurutma: 1 g numunede ±2 mg = ±0,2 % m/m, dikdörtgen, u = 0,115 % m/m; yalnızca s_R/3'ü aştığı yerde, yani %2,5 düzeyinde dahil edilir.
3. U = 2u_c: 2,5, 5 ve 10 % m/m'de 0,62, 0,8 ve 1,2 % m/m.

**Sonuç:**

- w(crude fibre) = (2,50 ± 0,62) % m/m (k = 2,00)
- w(crude fibre) = (5,00 ± 0,80) % m/m (k = 2,00)
- w(crude fibre) = (10,0 ± 1,2) % m/m (k = 2,00)

**Notlar:**

- Bulgu: Tablo A6.5, %10 lifte U = 0,12 % m/m yazar; 2 × 0,6 = 1,2 % m/m'dir (yüzde sütunu, %12, doğrudur).

**Çalıştırma:** `uncertainty run library/chemistry/quam-a6-crude-fibre.json --lang tr`

## Elektrik ve RF

### 10 kΩ'luk standart direncin kalibrasyonu

`ea402-s3-resistor-10kohm` — EA-4/02 M:2022, S3

EA-4/02 S3: dört uçlu direncin, oran cihazı olarak sayısal multimetre kullanılarak standartla yer değiştirme yöntemiyle kalibrasyonu.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. R_X = (R_S + δR_D + δR_TS) · r_C · r − δR_TX (değerler 10 kΩ'a göre mΩ).
2. r oranı beş gözlemin ortalamasıdır: A tipi değerlendirme (s/√5 = 0,071 × 10⁻⁶, 4 serbestlik derecesi).
3. Standardın sürüklenmesi (+20 ± 10 mΩ) ve sıcaklık etkileri dikdörtgen; oran düzeltmesi r_C üçgen (±1 × 10⁻⁶) dağılımlıdır.
4. R_X = (10 000,178 ± 0,017) Ω (k = 2).

**Sonuç:**

- R_X = (10000178 ± 17) mΩ; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/ea402-s3-resistor-10kohm.json --lang tr`

### Bir güç sensörünün 18 GHz'de kalibrasyonu

`ea402-s6-power-sensor-18ghz` — EA-4/02 M:2022, S6

EA-4/02 S6: bir termistör güç sensörünün kalibrasyon faktörünün standart sensöre karşı belirlenmesi; uyumsuzluk terimleri U biçimlidir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. K_X = (K_S + δK_D) · (M_Sr·M_Xc)/(M_Sc·M_Xr) · p_Cr · p_Cc · p; p gözlenen güç oranıdır.
2. Uyumsuzluk çarpanlarının sınırları 1 ± 2|Γ_G||Γ|'dır; faz bilinmediği için ark sinüs (U biçimli) dağılım kullanılır (u = a/√2).
3. p, üç tekrarlı ölçümden A tipi tahmindir (2 serbestlik derecesi); bu ν_eff ≈ 310 verir.
4. K_X = 0,933 ± 0,032 (k = 2).

**Sonuç:**

- K_X = (0,933 ± 0,032); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Örneğin başlığında 19 GHz yazmaktadır; metin ve sonuç 18 GHz'e aittir.
- Üç gözlemle p için t dağılımı ν = 2'dir ve sonlu varyansı yoktur; bu nedenle Monte Carlo standart sapması kararsızdır, kapsam aralığı ise iyi tanımlıdır (JCGM 101 6.4.9).

**Çalıştırma:** `uncertainty run library/electrical/ea402-s6-power-sensor-18ghz.json --lang tr`

### Koaksiyel kademeli zayıflatıcının kalibrasyonu (30 dB)

`ea402-s7-step-attenuator-30db` — EA-4/02 M:2022, S7

EA-4/02 S7: koaksiyel zayıflatıcının 10 GHz'de 30 dB kademesi; uyumsuzluk S parametrelerinden.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. L_X = L_S + δL_S + δL_D + δL_M + δL_K + δL_ib − δL_ia + δL_0b − δL_0a (dB).
2. L_S dört gözlemin ortalamasıdır (A tipi, 3 serbestlik derecesi); uyumsuzluk terimi kaynak, yük ve zayıflatıcının yansıma katsayılarından hesaplanır (Eş. S7.2): u(δL_M) = 0,0198 dB.
3. L_X = (30,043 ± 0,045) dB (k = 2).

**Sonuç:**

- L_X = (30,043 ± 0,044) dB; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu: bütçe δL_ib'nin katkısını 0,0019 dB olarak verir, oysa standart belirsizliği 0,0003 dB'dir; basılı toplam 0,0223 dB bu 0,0019 değerini içerir (onsuz 0,0222 dB). Raporlanan sonuç değişmez.

**Çalıştırma:** `uncertainty run library/electrical/ea402-s7-step-attenuator-30db.json --lang tr`

### El tipi sayısal multimetrenin 100 V'ta kalibrasyonu

`ea402-s9-dmm-100v` — EA-4/02 M:2022, S9

EA-4/02 S9: 3½ basamaklı multimetrenin gösterge hatası; çözünürlük terimi baskındır ve çıktı neredeyse dikdörtgen dağılımlıdır.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. E_X = V_iX − V_S + δV_iX − δV_S (V); 0,1 V çözünürlük ±0,05 V (dikdörtgen), u = 0,029 V verir.
2. Tek bir dikdörtgen terim baskın olduğundan (diğerlerinin oranı 0,22 ≤ 0,3) EA-4/02 %95 için k = 2 yerine k = 0,95·√3 = 1,65 kullanır: U = 0,05 V (S9.14).
3. Monte Carlo %95 aralığı ±2u = ±0,06 V'un kapsamı fazla gösterdiğini doğrudan ortaya koyar; pratik bir kurala gerek kalmaz.

**Sonuç:**

- E_X = (0,10 ± 0,06) V; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/ea402-s9-dmm-100v.json --lang tr`

### Korelasyonlu V ve I'dan empedans büyüklüğü (GUM H.2)

`gum-h2-impedance` — JCGM 100:2008, H.2, Tables H.2 and H.3

GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden Z = V/I empedans büyüklüğü.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Bir devre elemanının uçlarındaki V gerilimi, içinden geçen I akımı ve φ faz açısının beş eşzamanlı okuma takımı, GUM Tablo H.2'deki ortalamaları s(x̄) = 0,0032 V, 0,0095 mA ve 0,00075 rad ile ve r(V, I) = −0,36, r(V, φ) = 0,86, r(I, φ) = −0,65 korelasyon katsayılarıyla verir (GUM 5.2.3). Proje bu değerleri Tablo H.2'deki beş takımdan yeniden hesaplanmış, yuvarlanmamış biçimde kullanır (s(x̄) = 0,003209 V, 0,009471 mA, 0,000752 rad; r = −0,3553; 0,8576; −0,6451).
2. GUM H.2.3'ün 1. yaklaşımı modeli ortalamalarda hesaplar ve kovaryans terimlerini, 2c_ic_ju(x_i)u(x_j)r(x_i, x_j), içeren Eşitlik (16) ile yayar.
3. Z = V/I için GUM Eşitlik (H.8c) u_r²(Z) = u_r²(V) + u_r²(I) − 2u_r(V)u_r(I)r(V, I) verir; r(V, I) = −0,36 olduğundan son terim varyansı artırır ve u_c(Z) = 0,236 Ω olur.

**Sonuç:**

- Z = (254,26 ± 0,46) Ω; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/gum-h2-impedance.json --lang tr`

### Korelasyonlu V, I ve φ'den reaktans (GUM H.2)

`gum-h2-reactance` — JCGM 100:2008, H.2, Tables H.2 and H.3

GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden X = (V/I) sin φ reaktansı.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Bir devre elemanının uçlarındaki V gerilimi, içinden geçen I akımı ve φ faz açısının beş eşzamanlı okuma takımı, GUM Tablo H.2'deki ortalamaları s(x̄) = 0,0032 V, 0,0095 mA ve 0,00075 rad ile ve r(V, I) = −0,36, r(V, φ) = 0,86, r(I, φ) = −0,65 korelasyon katsayılarıyla verir (GUM 5.2.3). Proje bu değerleri Tablo H.2'deki beş takımdan yeniden hesaplanmış, yuvarlanmamış biçimde kullanır (s(x̄) = 0,003209 V, 0,009471 mA, 0,000752 rad; r = −0,3553; 0,8576; −0,6451).
2. GUM H.2.3'ün 1. yaklaşımı modeli ortalamalarda hesaplar ve kovaryans terimlerini, 2c_ic_ju(x_i)u(x_j)r(x_i, x_j), içeren Eşitlik (16) ile yayar.
3. X = (V/I) sin φ için V ile φ arasındaki korelasyon varyansı artırır; u_c(X) = 0,295 Ω (bağıl %0,13).

**Sonuç:**

- X = (219,85 ± 0,58) Ω; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- Yuvarlanmamış verilerle 1. yaklaşım u_c(X) = 0,29558 Ω, 2. yaklaşım 0,29549 Ω verir; GUM Tablo H.3 0,295 bastığından tolerans 0,0006 Ω'dur.

**Çalıştırma:** `uncertainty run library/electrical/gum-h2-reactance.json --lang tr`

### Korelasyonlu V, I ve φ'den direnç (GUM H.2)

`gum-h2-resistance` — JCGM 100:2008, H.2, Tables H.2 and H.3

GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden R = (V/I) cos φ direnci.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Bir devre elemanının uçlarındaki V gerilimi, içinden geçen I akımı ve φ faz açısının beş eşzamanlı okuma takımı, GUM Tablo H.2'deki ortalamaları s(x̄) = 0,0032 V, 0,0095 mA ve 0,00075 rad ile ve r(V, I) = −0,36, r(V, φ) = 0,86, r(I, φ) = −0,65 korelasyon katsayılarıyla verir (GUM 5.2.3). Proje bu değerleri Tablo H.2'deki beş takımdan yeniden hesaplanmış, yuvarlanmamış biçimde kullanır (s(x̄) = 0,003209 V, 0,009471 mA, 0,000752 rad; r = −0,3553; 0,8576; −0,6451).
2. GUM H.2.3'ün 1. yaklaşımı modeli ortalamalarda hesaplar ve kovaryans terimlerini, 2c_ic_ju(x_i)u(x_j)r(x_i, x_j), içeren Eşitlik (16) ile yayar.
3. R = (V/I) cos φ için kovaryans terimleri varyansı azaltır: katkıların karelerinin toplamı tek başına u_c²(R)'nin yaklaşık 3,6 katıdır ve u_c(R) = 0,071 Ω olur. 2. yaklaşım (R'nin her takımdan hesaplanması, GUM Tablo H.4) aynı değeri verir.

**Sonuç:**

- R = (127,73 ± 0,14) Ω; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- Kovaryans terimleri R'nin varyansının büyük kısmını götürdüğünden u_c(R), korelasyon katsayılarının yuvarlanmasına duyarlıdır: Tablo H.2'de basılı iki basamaklı değerlerle yayılım yasası 0,0711 Ω yerine 0,0700 Ω verir (NIST Uncertainty Machine'i yeniden üreten 'num-10-resistance' projesi). X ve Z %0,1'den az değişir.

**Çalıştırma:** `uncertainty run library/electrical/gum-h2-resistance.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,0

`jcgm101-9.4-comparison-loss-x000` — JCGM 101:2008, 9.4, Table 8; JCGM GUM-5:2026, Example 16, Table 16.1

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. Buradaki GUM sonucu ikinci mertebe terimleri içerir; bu kuadratik model için varyansı tam olarak verir, ancak simetrik dağılım varsayımına dayalı %95 aralığı yine de uygun değildir.
5. Kestirimin ikinci mertebe terimiyle, ½ΣΣ ∂²f/∂xᵢ∂xⱼ u(xᵢ, xⱼ) = u²(x₁) + u²(x₂) = 50·10⁻⁶ (GUM H.10; JCGM 100:2008/Amd.1:2026 ile 4.1.4'e eklenen not 1), GUM kestirimi analitik değer olan 50·10⁻⁶ olur. JCGM 101:2008 Tablo 8 GUM çerçevesi için 0·10⁻⁶ basar; 9.4.2.2.8 bunu geçersiz sayar.

**Sonuç:**

- dY = (50 ± 640) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 12,7 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x000.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,0, r = 0,9

`jcgm101-9.4-comparison-loss-x000-correlated` — JCGM 101:2008, 9.4, Table 9; JCGM GUM-5:2026, Example 16, Table 16.2

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. r(X₁, X₂) = 0,9 iken girdiler iki değişkenli normal dağılımdan (Cholesky çarpanı) üretilir. c₂ = 0 olduğundan birinci mertebe GUM sonucu korelasyondan etkilenmez; GUM 5.1.2 notu ikinci mertebe terimleri yalnızca bağımsız girdiler için verir. GUM çerçevesi sonuçlarından etkiyi yalnızca ilişkili girdiler için yüksek mertebe terimleri yakalar (…-correlated-higher-order projesi, D57); MCM de yakalar.

**Sonuç:**

- dY = (0,0 ± 0,0) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 12,7 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x000-correlated.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri

`jcgm101-9.4-comparison-loss-x000-correlated-higher-order` — JCGM 101:2008, 9.4, Table 9; JCGM GUM-5:2026, Example 16, Table 16.2; G. Mana, higher-order terms for correlated inputs

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. GUM-5 Örnek 16 burada yalnızca birinci mertebe terimleri uygular, çünkü JCGM 100:2008 yüksek mertebe terimleri yalnızca bağımsız girdiler için verir. İki değişkenli normal girdiler için Taylor açılımı u²(δy) = cᵀΣc + ½tr((HΣ)²) verir; H = diag(2, 2): 4x₁²u² + 4u⁴(1 + r²), yani u(δy) = 67,3·10⁻⁶ (Mana ve Pennecchi 2007).
5. JCGM 100:2008/Amd.1:2026'nın ikinci mertebe terimiyle kestirim x₁² + 2u² = 50·10⁻⁶ olur. İkisi de GUM-5 Tablo 16.2'nin analitik A sütunuyla ve Monte Carlo sonucuyla uyuşur.

**Sonuç:**

- dY = (50 ± 850) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 12,7 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x000-correlated-higher-order.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,01

`jcgm101-9.4-comparison-loss-x010` — JCGM 101:2008, 9.4, Table 8; JCGM GUM-5:2026, Example 16, Table 16.1

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. Buradaki GUM sonucu ikinci mertebe terimleri içerir; bu kuadratik model için varyansı tam olarak verir, ancak simetrik dağılım varsayımına dayalı %95 aralığı yine de uygun değildir.
5. Kestirimin ikinci mertebe terimiyle, ½ΣΣ ∂²f/∂xᵢ∂xⱼ u(xᵢ, xⱼ) = u²(x₁) + u²(x₂) = 50·10⁻⁶ (GUM H.10; JCGM 100:2008/Amd.1:2026 ile 4.1.4'e eklenen not 1), GUM kestirimi analitik değer olan 150·10⁻⁶ olur. JCGM 101:2008 Tablo 8 GUM çerçevesi için 100·10⁻⁶ basar; 9.4.2.2.8 bunu geçersiz sayar.

**Sonuç:**

- dY = (150 ± 220) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x010.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,01, r = 0,9

`jcgm101-9.4-comparison-loss-x010-correlated` — JCGM 101:2008, 9.4, Table 9; JCGM GUM-5:2026, Example 16, Table 16.2

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. r(X₁, X₂) = 0,9 iken girdiler iki değişkenli normal dağılımdan (Cholesky çarpanı) üretilir. c₂ = 0 olduğundan birinci mertebe GUM sonucu korelasyondan etkilenmez; GUM 5.1.2 notu ikinci mertebe terimleri yalnızca bağımsız girdiler için verir. GUM çerçevesi sonuçlarından etkiyi yalnızca ilişkili girdiler için yüksek mertebe terimleri yakalar (…-correlated-higher-order projesi, D57); MCM de yakalar.

**Sonuç:**

- dY = (100 ± 200) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x010-correlated.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,01, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri

`jcgm101-9.4-comparison-loss-x010-correlated-higher-order` — JCGM 101:2008, 9.4, Table 9; JCGM GUM-5:2026, Example 16, Table 16.2; G. Mana, higher-order terms for correlated inputs

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. GUM-5 Örnek 16 burada yalnızca birinci mertebe terimleri uygular, çünkü JCGM 100:2008 yüksek mertebe terimleri yalnızca bağımsız girdiler için verir. İki değişkenli normal girdiler için Taylor açılımı u²(δy) = cᵀΣc + ½tr((HΣ)²) verir; H = diag(2, 2): 4x₁²u² + 4u⁴(1 + r²), yani u(δy) = 120,5·10⁻⁶ (Mana ve Pennecchi 2007).
5. JCGM 100:2008/Amd.1:2026'nın ikinci mertebe terimiyle kestirim x₁² + 2u² = 150·10⁻⁶ olur. İkisi de GUM-5 Tablo 16.2'nin analitik A sütunuyla ve Monte Carlo sonucuyla uyuşur.

**Sonuç:**

- dY = (150 ± 240) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x010-correlated-higher-order.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,05

`jcgm101-9.4-comparison-loss-x050` — JCGM 101:2008, 9.4, Table 8; JCGM GUM-5:2026, Example 16, Table 16.1

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. Buradaki GUM sonucu ikinci mertebe terimleri içerir; bu kuadratik model için varyansı tam olarak verir, ancak simetrik dağılım varsayımına dayalı %95 aralığı yine de uygun değildir.
5. Kestirimin ikinci mertebe terimiyle, ½ΣΣ ∂²f/∂xᵢ∂xⱼ u(xᵢ, xⱼ) = u²(x₁) + u²(x₂) = 50·10⁻⁶ (GUM H.10; JCGM 100:2008/Amd.1:2026 ile 4.1.4'e eklenen not 1), GUM kestirimi analitik değer olan 2550·10⁻⁶ olur. JCGM 101:2008 Tablo 8 GUM çerçevesi için 2500·10⁻⁶ basar; 9.4.2.2.8 bunu geçersiz sayar.

**Sonuç:**

- dY = (2550 ± 980) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x050.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,05, r = 0,9

`jcgm101-9.4-comparison-loss-x050-correlated` — JCGM 101:2008, 9.4, Table 9; JCGM GUM-5:2026, Example 16, Table 16.2

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. r(X₁, X₂) = 0,9 iken girdiler iki değişkenli normal dağılımdan (Cholesky çarpanı) üretilir. c₂ = 0 olduğundan birinci mertebe GUM sonucu korelasyondan etkilenmez; GUM 5.1.2 notu ikinci mertebe terimleri yalnızca bağımsız girdiler için verir. GUM çerçevesi sonuçlarından etkiyi yalnızca ilişkili girdiler için yüksek mertebe terimleri yakalar (…-correlated-higher-order projesi, D57); MCM de yakalar.

**Sonuç:**

- dY = (2500 ± 980) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x050-correlated.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,05, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri

`jcgm101-9.4-comparison-loss-x050-correlated-higher-order` — JCGM 101:2008, 9.4, Table 9; JCGM GUM-5:2026, Example 16, Table 16.2; G. Mana, higher-order terms for correlated inputs

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. GUM-5 Örnek 16 burada yalnızca birinci mertebe terimleri uygular, çünkü JCGM 100:2008 yüksek mertebe terimleri yalnızca bağımsız girdiler için verir. İki değişkenli normal girdiler için Taylor açılımı u²(δy) = cᵀΣc + ½tr((HΣ)²) verir; H = diag(2, 2): 4x₁²u² + 4u⁴(1 + r²), yani u(δy) = 504,5·10⁻⁶ (Mana ve Pennecchi 2007).
5. JCGM 100:2008/Amd.1:2026'nın ikinci mertebe terimiyle kestirim x₁² + 2u² = 2550·10⁻⁶ olur. İkisi de GUM-5 Tablo 16.2'nin analitik A sütunuyla ve Monte Carlo sonucuyla uyuşur.

**Sonuç:**

- dY = (2550 ± 990) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x050-correlated-higher-order.json --lang tr`

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,001 (JCGM 102 9.3)

`jcgm102-9.3-polar-x001` — JCGM 102:2011, 9.3 (Table 6)

JCGM 102:2011 9.3: x₁ = 0,001, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 6).

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Karmaşık bir Z = X₁ + iX₂ büyüklüğünün Kartezyen koordinatlardan kutupsal koordinatlara dönüşümü: R = √(X₁² + X₂²), Θ = atan2(X₂, X₁) (ifade 25); x₁ = 0,001, x₂ = 0, u(x₁) = u(x₂) = 0,010 ve bağımsız girdiler.
2. GUM yaklaşımı: (x₁, 0) noktasında duyarlılık matrisi [[1, 0], [0, 1/x₁]] olduğundan u(θ) = u(x₂)/x₁'dir; x₁, u(x₁) ile karşılaştırılabilir olduğunda bu doğrusallaştırma geçersizdir.
3. 10⁶ denemeli Monte Carlo yöntemi (Ek 10⁷ kullanır): Θ'nın dağılımı (−π, π] aralığına yayılır ve başlangıç noktasının yakınında korelasyonlu girdilerle iki tepeli olur (9.3.3.3); bu durumda elips ve dikdörtgen uygun kapsam bölgeleri değildir (9.3.4.4).

**Sonuç:**

- R = 0,0010, u(R) = 0,010 (GUM)
- Theta = 0,0 rad, u(Theta) = 10 rad (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/electrical/jcgm102-9.3-polar-x001.json --lang tr`

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,001, r = 0,9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x001-correlated` — JCGM 102:2011, 9.3 (Table 7)

JCGM 102:2011 9.3: x₁ = 0,001, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 7).

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Karmaşık bir Z = X₁ + iX₂ büyüklüğünün Kartezyen koordinatlardan kutupsal koordinatlara dönüşümü: R = √(X₁² + X₂²), Θ = atan2(X₂, X₁) (ifade 25); x₁ = 0,001, x₂ = 0, u(x₁) = u(x₂) = 0,010 ve r(x₁, x₂) = 0,9.
2. GUM yaklaşımı: (x₁, 0) noktasında duyarlılık matrisi [[1, 0], [0, 1/x₁]] olduğundan u(θ) = u(x₂)/x₁'dir; x₁, u(x₁) ile karşılaştırılabilir olduğunda bu doğrusallaştırma geçersizdir.
3. 10⁶ denemeli Monte Carlo yöntemi (Ek 10⁷ kullanır): Θ'nın dağılımı (−π, π] aralığına yayılır ve başlangıç noktasının yakınında korelasyonlu girdilerle iki tepeli olur (9.3.3.3); bu durumda elips ve dikdörtgen uygun kapsam bölgeleri değildir (9.3.4.4).

**Sonuç:**

- R = 0,0010, u(R) = 0,010 (GUM)
- Theta = 0,0 rad, u(Theta) = 10 rad (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/electrical/jcgm102-9.3-polar-x001-correlated.json --lang tr`

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,01 (JCGM 102 9.3)

`jcgm102-9.3-polar-x010` — JCGM 102:2011, 9.3 (Table 6)

JCGM 102:2011 9.3: x₁ = 0,01, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 6).

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Karmaşık bir Z = X₁ + iX₂ büyüklüğünün Kartezyen koordinatlardan kutupsal koordinatlara dönüşümü: R = √(X₁² + X₂²), Θ = atan2(X₂, X₁) (ifade 25); x₁ = 0,01, x₂ = 0, u(x₁) = u(x₂) = 0,010 ve bağımsız girdiler.
2. GUM yaklaşımı: (x₁, 0) noktasında duyarlılık matrisi [[1, 0], [0, 1/x₁]] olduğundan u(θ) = u(x₂)/x₁'dir; x₁, u(x₁) ile karşılaştırılabilir olduğunda bu doğrusallaştırma geçersizdir.
3. 10⁶ denemeli Monte Carlo yöntemi (Ek 10⁷ kullanır): Θ'nın dağılımı (−π, π] aralığına yayılır ve başlangıç noktasının yakınında korelasyonlu girdilerle iki tepeli olur (9.3.3.3); bu durumda elips ve dikdörtgen uygun kapsam bölgeleri değildir (9.3.4.4).

**Sonuç:**

- R = 0,0100, u(R) = 0,010 (GUM)
- Theta = 0,00 rad, u(Theta) = 1,0 rad (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/electrical/jcgm102-9.3-polar-x010.json --lang tr`

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,01, r = 0,9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x010-correlated` — JCGM 102:2011, 9.3 (Table 7)

JCGM 102:2011 9.3: x₁ = 0,01, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 7).

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Karmaşık bir Z = X₁ + iX₂ büyüklüğünün Kartezyen koordinatlardan kutupsal koordinatlara dönüşümü: R = √(X₁² + X₂²), Θ = atan2(X₂, X₁) (ifade 25); x₁ = 0,01, x₂ = 0, u(x₁) = u(x₂) = 0,010 ve r(x₁, x₂) = 0,9.
2. GUM yaklaşımı: (x₁, 0) noktasında duyarlılık matrisi [[1, 0], [0, 1/x₁]] olduğundan u(θ) = u(x₂)/x₁'dir; x₁, u(x₁) ile karşılaştırılabilir olduğunda bu doğrusallaştırma geçersizdir.
3. 10⁶ denemeli Monte Carlo yöntemi (Ek 10⁷ kullanır): Θ'nın dağılımı (−π, π] aralığına yayılır ve başlangıç noktasının yakınında korelasyonlu girdilerle iki tepeli olur (9.3.3.3); bu durumda elips ve dikdörtgen uygun kapsam bölgeleri değildir (9.3.4.4).

**Sonuç:**

- R = 0,0100, u(R) = 0,010 (GUM)
- Theta = 0,00 rad, u(Theta) = 1,0 rad (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/electrical/jcgm102-9.3-polar-x010-correlated.json --lang tr`

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,1 (JCGM 102 9.3)

`jcgm102-9.3-polar-x100` — JCGM 102:2011, 9.3 (Table 6)

JCGM 102:2011 9.3: x₁ = 0,1, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 6).

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Karmaşık bir Z = X₁ + iX₂ büyüklüğünün Kartezyen koordinatlardan kutupsal koordinatlara dönüşümü: R = √(X₁² + X₂²), Θ = atan2(X₂, X₁) (ifade 25); x₁ = 0,1, x₂ = 0, u(x₁) = u(x₂) = 0,010 ve bağımsız girdiler.
2. GUM yaklaşımı: (x₁, 0) noktasında duyarlılık matrisi [[1, 0], [0, 1/x₁]] olduğundan u(θ) = u(x₂)/x₁'dir; x₁, u(x₁) ile karşılaştırılabilir olduğunda bu doğrusallaştırma geçersizdir.
3. 10⁶ denemeli Monte Carlo yöntemi (Ek 10⁷ kullanır): Θ'nın dağılımı (−π, π] aralığına yayılır ve başlangıç noktasının yakınında korelasyonlu girdilerle iki tepeli olur (9.3.3.3); bu durumda elips ve dikdörtgen uygun kapsam bölgeleri değildir (9.3.4.4).

**Sonuç:**

- R = 0,1000, u(R) = 0,010 (GUM)
- Theta = 0,000 rad, u(Theta) = 0,10 rad (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/electrical/jcgm102-9.3-polar-x100.json --lang tr`

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,1, r = 0,9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x100-correlated` — JCGM 102:2011, 9.3 (Table 7)

JCGM 102:2011 9.3: x₁ = 0,1, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 7).

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Karmaşık bir Z = X₁ + iX₂ büyüklüğünün Kartezyen koordinatlardan kutupsal koordinatlara dönüşümü: R = √(X₁² + X₂²), Θ = atan2(X₂, X₁) (ifade 25); x₁ = 0,1, x₂ = 0, u(x₁) = u(x₂) = 0,010 ve r(x₁, x₂) = 0,9.
2. GUM yaklaşımı: (x₁, 0) noktasında duyarlılık matrisi [[1, 0], [0, 1/x₁]] olduğundan u(θ) = u(x₂)/x₁'dir; x₁, u(x₁) ile karşılaştırılabilir olduğunda bu doğrusallaştırma geçersizdir.
3. 10⁶ denemeli Monte Carlo yöntemi (Ek 10⁷ kullanır): Θ'nın dağılımı (−π, π] aralığına yayılır ve başlangıç noktasının yakınında korelasyonlu girdilerle iki tepeli olur (9.3.3.3); bu durumda elips ve dikdörtgen uygun kapsam bölgeleri değildir (9.3.4.4).

**Sonuç:**

- R = 0,1000, u(R) = 0,010 (GUM)
- Theta = 0,000 rad, u(Theta) = 0,10 rad (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/electrical/jcgm102-9.3-polar-x100-correlated.json --lang tr`

### Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans (JCGM 102 9.4)

`jcgm102-9.4-resistance-reactance` — JCGM 102:2011, 9.4 (Tables 8–11)

JCGM 102:2011 9.4 (GUM H.2'nin genişletilmesi): korelasyonlu ortalamalardan üç çıktı büyüklüğü; U_x = M/(n(n − 1)) ile GUM yaklaşımı ve çok değişkenli t dağılımıyla Monte Carlo yöntemi.

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Alternatif akım için Ohm yasası: R = (V/I) cos φ, X = (V/I) sin φ, Z = V/I (model 26); V, I ve φ'nin n = 6 eşzamanlı gösterge değeri takımıyla (Tablo 8; 6. takım 1–5. takımların ortalamasıdır).
2. Tahminler ortalamalardır; kareler ve çarpımlar toplamı matrisi M, ortalamaların kovaryans matrisini ve Tablo 10'daki korelasyon katsayılarını verir.
3. Monte Carlo yöntemi: X, çok değişkenli t dağılımından t_ν(x̄, S/n), ν = n − N = 3, S = M/ν olarak çekilir (5.3.2.4). Z² = R² + X² olduğundan U_y teoride tekildir (9.4.2.3 not 2).

**Sonuç:**

- R = 127,7322 Ω, u(R) = 0,058 Ω (GUM)
- X = 219,847 Ω, u(X) = 0,24 Ω (GUM)
- Z = 254,260 Ω, u(Z) = 0,19 Ω (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,80; dikdörtgen k_q = 2,39

**Notlar:**

- ν = 3 olduğunda çok değişkenli t dağılımının dördüncü momenti yoktur; bu yüzden Monte Carlo standart sapmaları yavaş yakınsar: 10⁶ denemeyle 1–6 tohumları u(R) için 0,128 Ω ile 0,136 Ω, r(R, X) için −0,580 ile −0,600 arasında değerler verir. Monte Carlo değerlerinin toleransları bu yayılımı kapsar; basılı MCM satırı bu tür bir gerçekleşmedir.
- Tablo 8, 6. takımı φ = 1,0445 rad olarak, yani 1–5. takımların yuvarlanmış ortalaması (1,04446) olarak basar. Yuvarlanmış değerle R'nin tahmini basılı 127,732 Ω yerine 127,7307 Ω olur; proje Tablo 9'daki değer olan 1,04446 rad'ı kullanır (araştırma günlüğü F51).

**Çalıştırma:** `uncertainty run library/electrical/jcgm102-9.4-resistance-reactance.json --lang tr`

### Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans, t kovaryansı (JCGM 102 9.4)

`jcgm102-9.4-resistance-reactance-alternative` — JCGM 102:2011, 9.4 (Tables 8–11)

JCGM 102:2011 9.4 (GUM H.2'nin genişletilmesi): korelasyonlu ortalamalardan üç çıktı büyüklüğü; çok değişkenli t dağılımının kovaryans matrisi ile GUM yaklaşımı ve çok değişkenli t dağılımıyla Monte Carlo yöntemi.

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Alternatif akım için Ohm yasası: R = (V/I) cos φ, X = (V/I) sin φ, Z = V/I (model 26); V, I ve φ'nin n = 6 eşzamanlı gösterge değeri takımıyla (Tablo 8; 6. takım 1–5. takımların ortalamasıdır).
2. Tahminler ortalamalardır; kareler ve çarpımlar toplamı matrisi M, ortalamaların kovaryans matrisini ve Tablo 10'daki korelasyon katsayılarını verir.
3. Monte Carlo yöntemi: X, çok değişkenli t dağılımından t_ν(x̄, S/n), ν = n − N = 3, S = M/ν olarak çekilir (5.3.2.4). Z² = R² + X² olduğundan U_y teoride tekildir (9.4.2.3 not 2).
4. Alternatif GUM değerlendirmesi V(X) = M/((ν − 2)n) kullanır; bu Monte Carlo sonuçlarıyla uyuşur (9.4.2.5–9.4.2.7).

**Sonuç:**

- R = 127,732 Ω, u(R) = 0,13 Ω (GUM)
- X = 219,847 Ω, u(X) = 0,54 Ω (GUM)
- Z = 254,260 Ω, u(Z) = 0,43 Ω (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,80; dikdörtgen k_q = 2,39

**Notlar:**

- ν = 3 olduğunda çok değişkenli t dağılımının dördüncü momenti yoktur; bu yüzden Monte Carlo standart sapmaları yavaş yakınsar: 10⁶ denemeyle 1–6 tohumları u(R) için 0,128 Ω ile 0,136 Ω, r(R, X) için −0,580 ile −0,600 arasında değerler verir. Monte Carlo değerlerinin toleransları bu yayılımı kapsar; basılı MCM satırı bu tür bir gerçekleşmedir.
- Tablo 8, 6. takımı φ = 1,0445 rad olarak, yani 1–5. takımların yuvarlanmış ortalaması (1,04446) olarak basar. Yuvarlanmış değerle R'nin tahmini basılı 127,732 Ω yerine 127,7307 Ω olur; proje Tablo 9'daki değer olan 1,04446 rad'ı kullanır (araştırma günlüğü F51).

**Çalıştırma:** `uncertainty run library/electrical/jcgm102-9.4-resistance-reactance-alternative.json --lang tr`

### Baskın çözünürlük terimli dijital voltmetre (UKAS M3003 Ek C)

`m3003-c5-dvm-dominant-rectangular` — UKAS M3003 ed. 6 (2024), Appendix C.5–C.10

Uygulanan 1,000 00 V için 1,001 V okuyan bir DVM; 1 mV basamağının yuvarlanması baskındır; bu nedenle %95,45 için k, dikdörtgen ve normal dağılımın konvolüsyonundan gelir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. ΔV = V_ind − V_ref + δI_T (C.10): referans U = 0,19 mV (k = 2), sürüklenme ve sıcaklık ±0,05 mV, yuvarlama ±0,5 mV.
2. u_R = 0,289 mV, u_N = 0,099 mV, u_N/u_R = 0,344 < 0,70: dikdörtgen terim baskındır (C.4, C.5).
3. Program konvolüsyon için P(|R + N| ≤ k·u_c) = 0,9545 eşitliğini çözer; bu C.7 tablosunu yeniden üretir: k = 1,77, U = 1,77 × 0,305 mV = 0,54 mV.

**Sonuç:**

- dV = (1,00 ± 0,54) mV; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,77 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F61: C.5 sürüklenme ve sıcaklık sınırını ±50 V olarak yazar; hesap 0,05 mV, yani ±50 µV kullanır.

**Çalıştırma:** `uncertainty run library/electrical/m3003-c5-dvm-dominant-rectangular.json --lang tr`

### Gerilim karşılaştırmasıyla 10 kΩ direnç (UKAS M3003 K.1)

`m3003-k1-resistor-10kohm` — UKAS M3003 ed. 6 (2024), Appendix K.1

Seri bağlı standart ve bilinmeyen 10 kΩ dirençler üzerindeki gerilimler tek bir DVM ile karşılaştırılır; her iki direnç 20,0 °C ± 0,1 °C yağ banyosundadır.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. R_X = (R_S + δR_D + R_TC·Δt)·V_X/V_S (K.1.1); DVM doğrusallığı ve çözünürlüğü her okumanın göreli hatası olarak, ±0,2 ppm, dikdörtgen (K.1.6).
2. R_S = 10 000,000 Ω, 10 000,1050 Ω = R_S·(1 + 10,50 ppm) sonucundan çıkar; sertifika ±0,5 ppm (k = 2), düzeltilmemiş sürüklenme ±0,5 ppm, ±0,1 °C üzerinde 2,5 ppm/°C sıcaklık katsayısı.
3. Oran sapmasının beş okuması (+10,4 … +10,7 ppm): ortalama +10,50 ppm, s = 0,158 ppm, u = s/√5 = 0,071 ppm, 4 serbestlik derecesi.
4. u_c = 0,445 ppm = 0,004 45 Ω; R_X = 10 000,1050 Ω ± 0,0089 Ω (k = 2).
5. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- R_X = (10000,1050 ± 0,0089) Ω; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/m3003-k1-resistor-10kohm.json --lang tr`

### 18 GHz'de koaksiyel güç sensörü (UKAS M3003 K.2)

`m3003-k2-power-sensor-18ghz` — UKAS M3003 ed. 6 (2024), Appendix K.2

Bilinmeyen bir güç sensörünün kalibrasyon faktörü, izlenen bir kaynakta standart sensörle yer değiştirme yöntemiyle; dört bağlantı.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. K_X = (K_S + δD_S)·f_DC·f_M·f_REF (K.2.1). M3003 tüm terimleri kalibrasyon faktörünün yüzde puanı olarak birleştirir; model bu nedenle onları gözlenen K_X'e ekler.
2. Uyumsuzluk sınırları 200·|Γ_G|·|Γ| %: her iki sensör için 50 MHz'de %0,08, 18 GHz'de %1,40 (standart) ve %1,68 (bilinmeyen), U biçimli (ark sinüs, u = a/√2).
3. Dört değer 93,45, 92,20, 93,95, 93,02 %: ortalama %93,16, s = %0,7415, u = s/√4 = %0,37 (3 serbestlik derecesi).
4. u_c = %1,69, 18 GHz uyumsuzluk terimleri baskındır; K_X = %93,2 ± %3,4 (k = 2).
5. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- K_X = (93,2 ± 3,4) %; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/m3003-k2-power-sensor-18ghz.json --lang tr`

### 10 GHz'de 30 dB koaksiyel zayıflatıcı (UKAS M3003 K.3)

`m3003-k3-attenuator-30db` — UKAS M3003 ed. 6 (2024), Appendix K.3

Koaksiyel kademeli zayıflatıcının IF yer değiştirme ile kalibrasyonu; uyumsuzluk terimi baskındır.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. A_X = A_b − A_a + δA_IF + δD_IF + δL_M + δR_D + δM + δA_L (K.3.1), 10 GHz'de çift kanallı 30 MHz IF yer değiştirme.
2. Dört okuma 30,04, 30,07, 30,03, 30,06 dB: ortalama 30,050 dB, s = 0,018 dB, u = 0,009 dB (3 serbestlik derecesi).
3. Bütçedeki gibi uyumsuzluk standart belirsizliği 0,022 dB (K.3.9); u_c = 0,0245 dB; A_X = 30,050 dB ± 0,049 dB (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- A_X = (30,050 ± 0,049) dB; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F57: K.3.6'daki yansıma ve saçılma katsayılarıyla E5 formülü 0,022 dB değil 0,0170 dB uyumsuzluk standart sapması verir; m3003-k3-attenuator-30db-mismatch-e5 projesine bakınız.

**Çalıştırma:** `uncertainty run library/electrical/m3003-k3-attenuator-30db.json --lang tr`

### E5 formülünden uyumsuzluk terimiyle 30 dB zayıflatıcı (UKAS M3003 K.3, E.9.5)

`m3003-k3-attenuator-30db-mismatch-e5` — UKAS M3003 ed. 6 (2024), Appendix K.3 and E.9.5

Örneğin verdiği katsayılardan yeniden hesaplanan uyumsuzluk standart sapmasıyla K.3 bütçesi (bulgu F57).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. A_X = A_b − A_a + δA_IF + δD_IF + δL_M + δR_D + δM + δA_L (K.3.1), 10 GHz'de çift kanallı 30 MHz IF yer değiştirme.
2. Dört okuma 30,04, 30,07, 30,03, 30,06 dB: ortalama 30,050 dB, s = 0,018 dB, u = 0,009 dB (3 serbestlik derecesi).
3. E5 formülü (Harris ve Warner): M = 8,686/√2 · [M_G + M_L + M_GL]^½ dB; M_G = |Γ_G|²(|s11a|² + |s11b|²), M_L = |Γ_L|²(|s22a|² + |s22b|²), M_GL = |Γ_G|²|Γ_L|²(|s21a|⁴ + |s21b|⁴).
4. Γ_G = Γ_L = 0,03, s11a = s11b = 0,05, s22a = 0,05, s22b = 0,01, s21a = 1, s21b = 0,31 ile (K.3.6): M = 0,0170 dB ve u_c = 0,0245 dB yerine 0,0202 dB.
5. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- A_X = (30,050 ± 0,040) dB; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/m3003-k3-attenuator-30db-mismatch-e5.json --lang tr`

### 1 V kademesinde DMM: 0,95 V'taki bütçe (UKAS M3003 L.3)

`m3003-l3-dmm-1v-range-095v` — UKAS M3003 ed. 6 (2024), Appendix L.3

Çok fonksiyonlu kalibratörle kalibre edilen 6½ basamaklı bir DMM'nin 1 V kademesinde 0,95 V'taki hatası; göreli (ppm) ve mutlak (µV) katkılar bir okumada birleştirilir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. ΔI_DMM = I_DMM − V_REF + δI_RES + δΔ_r, V_REF = V_CAL + δV_D + δV_UE + δV_TC + δV_LIN + δV_T + δV_CM (L.3.4); belirsizlik için okuma gerekmez; tahmin sıfırdır.
2. Göreli terimler (× I_DMM): kalibrasyon 2,8 ppm (k = 2), şartname ±8 ppm, tekrarlanabilirlik 2,5 ppm (ν = 9). Mutlak terimler: kalibrasyon 0,5 µV (k = 2), şartname ±2 µV, termoelektrik ±1 µV, ortak mod ±1 µV, çözünürlük ±0,5 µV.
3. I_DMM = 0,95 V'ta: u_c = 5,37 µV, U = 10,73 µV (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- dI = (0 ± 11) µV; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F61: bütçe başlığı 'ΔI_DMM = 0,95 V'ta değerlendirilmiş' der, kastedilen I_DMM = 0,95 V'tur; L.3.15, [a² + b²]^½ yerine Q[a, b] = [a² + ab]^½ tanımlar; Ek Q'nun notu u(R) = w(R) × R yerine u(R) = u(R) × R yazar.

**Çalıştırma:** `uncertainty run library/electrical/m3003-l3-dmm-1v-range-095v.json --lang tr`

### Tam ölçekte 1 V kademesinde DMM: baskın şartname terimi için kapsam faktörü (UKAS M3003 L.3.20)

`m3003-l3-dmm-1v-range-1v-dominant` — UKAS M3003 ed. 6 (2024), Appendix L.3

Çok fonksiyonlu kalibratörle kalibre edilen 6½ basamaklı bir DMM'nin 1 V kademesinde 1,0 V'taki hatası; göreli (ppm) ve mutlak (µV) katkılar bir okumada birleştirilir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. ΔI_DMM = I_DMM − V_REF + δI_RES + δΔ_r, V_REF = V_CAL + δV_D + δV_UE + δV_TC + δV_LIN + δV_T + δV_CM (L.3.4); belirsizlik için okuma gerekmez; tahmin sıfırdır.
2. Göreli terimler (× I_DMM): kalibrasyon 2,8 ppm (k = 2), şartname ±8 ppm, tekrarlanabilirlik 2,5 ppm (ν = 9). Mutlak terimler: kalibrasyon 0,5 µV (k = 2), şartname ±2 µV, termoelektrik ±1 µV, ortak mod ±1 µV, çözünürlük ±0,5 µV.
3. |I_DMM| = 1 V'ta şartname terimi sınırda baskındır: u_R/u_N = 1,435; konvolüsyon %95,45 için k = 1,898 verir (L.3.20); bu, 1,42 kritik oranına karşılık gelen k = 1,900'den anlamlı olarak farklı değildir.

**Sonuç:**

- dI = (0 ± 11) µV; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,90 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/m3003-l3-dmm-1v-range-1v-dominant.json --lang tr`

### 1 V kademesinde DMM: kademe formülünden 500 mV'taki belirsizlik (UKAS M3003 L.3)

`m3003-l3-dmm-1v-range-500mv` — UKAS M3003 ed. 6 (2024), Appendix L.3

Çok fonksiyonlu kalibratörle kalibre edilen 6½ basamaklı bir DMM'nin 1 V kademesinde 0,5 V'taki hatası; göreli (ppm) ve mutlak (µV) katkılar bir okumada birleştirilir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. ΔI_DMM = I_DMM − V_REF + δI_RES + δΔ_r, V_REF = V_CAL + δV_D + δV_UE + δV_TC + δV_LIN + δV_T + δV_CM (L.3.4); belirsizlik için okuma gerekmez; tahmin sıfırdır.
2. Göreli terimler (× I_DMM): kalibrasyon 2,8 ppm (k = 2), şartname ±8 ppm, tekrarlanabilirlik 2,5 ppm (ν = 9). Mutlak terimler: kalibrasyon 0,5 µV (k = 2), şartname ±2 µV, termoelektrik ±1 µV, ortak mod ±1 µV, çözünürlük ±0,5 µV.
3. I_DMM = 500 mV'ta: U = √((500 mV × 11 ppm)² + (2,9 µV)²) = 6,2 µV (L.3.14).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- dI = (0,0 ± 6,2) µV; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/m3003-l3-dmm-1v-range-500mv.json --lang tr`

### Mikrodalga kademeli zayıflatıcı, U biçimli girdiler (NIST TN 1900 E11)

`nist-tn1900-e11-step-attenuator` — NIST Technical Note 1900, Example E11, Exhibits 16 and 17

TN 1900 E11: EA-4/02 S7 Monte Carlo yöntemiyle yeniden değerlendirilir; baskın arksinüs uyumsuzluk terimi doğrusal bir toplamın dağılımını iki tepeli yapar.

**Yaklaşım:** Yalnızca Monte Carlo yöntemi (JCGM 101); sonuçlar için projeyi çalıştırın

**Adımlar:**

1. L_X = L_S + δL_S + δL_D + δL_M + δL_K + δL_ib − δL_ia + δL_0b − δL_0a; zayıflatıcının 30 dB kademesi kalibre edilmiş bir referans zayıflatıcıya karşı ölçülür (EA-4/02 S7).
2. L_S dört okumanın ortalamasıdır, 30,0402 dB, u = 0,0091 dB; TN 1900 bunu standart sapması 0,0091 dB olacak şekilde ölçeklenmiş bir t₃ değişkeni olarak modeller. Uyumsuzluk (u = 0,02 dB), kayma ve sızıntı (±0,003 dB) arksinüs (U biçimli); δL_S ve referans dedektör çözünürlükleri dikdörtgen; sıfır dedektörü terimleri normal dağılımlıdır (TN 1900 Şekil 16).
3. Monte Carlo yöntemi 30,043 dB, u = 0,0224 dB ve %95 aralığı [30,006; 30,081] dB verir. u²(δL_M), u²(L_X)'in %79'udur; bu nedenle çıktı yoğunluğu iki tepelidir ve klasik ± 2u aralığı muhafazakârdır (etkin kapsam %99).

**Notlar:**

- TN 1900, L_S'nin t₃ dağılımını standart sapması u = 0,0091 dB olacak şekilde ölçekler (ölçek 0,0091/√3 dB). JCGM 101 6.4.9 ise ölçek olarak s/√n = 0,0091 dB alır, yani standart sapma 0,0158 dB olur. Bir t girdisinin GUM belirsizliği ölçeği olduğundan GUM değerlendirmesi kapatılmıştır; EA-4/02 GUM sonucu 'ea402-s7-step-attenuator-30db' projesindedir.
- TN 1900, sızıntı terimi δL_K'yı ±0,003 dB üzerinde arksinüs olarak modeller (u = 0,0021 dB) ve EA-4/02 S7.12'deki 0,0017 dB'in dikdörtgen dağılıma karşılık geldiğini, oysa EA-4/02'nin terimi U biçimli olarak nitelediğini belirtir.

**Çalıştırma:** `uncertainty run library/electrical/nist-tn1900-e11-step-attenuator.json --lang tr`

### GUM Tablo H.2'nin yuvarlanmış değerleriyle direnç (NIST Uncertainty Machine §10)

`num-10-resistance` — JCGM 100:2008, H.2, Tables H.2 and H.3; T. Lafarge, §10, Figure 6

GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden R = (V/I) cos φ direnci.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. V, I ve φ'nin beş eşzamanlı okuma takımı GUM Tablo H.2'deki ortalamaları s(x̄) = 0,0032 V, 0,0095 mA ve 0,00075 rad ile ve r(V, I) = −0,36, r(V, φ) = 0,86, r(I, φ) = −0,65 ile verir; değerler burada basıldığı gibi girilmiştir.
2. Bu proje, NIST Uncertainty Machine kılavuzunun yaptığı gibi GUM Tablo H.2'de basılı yuvarlanmış değerleri girer. Kovaryans terimlerinin güçlü sadeleşmesi u_c(R) = 0,0700 Ω verir; yuvarlanmamış verilerden ise 0,071 Ω çıkar ('gum-h2-resistance' projesi). Üç değişkenli normal dağılımla Monte Carlo yöntemi yayılım yasasıyla uyuşur.

**Sonuç:**

- R = (127,73 ± 0,14) Ω; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/num-10-resistance.json --lang tr`

## Akış ve hacim

### 1000 mL'lik ölçülü balonun gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 1)

`cg19-ex1-flask-1000ml` — EURAMET Calibration Guide No. 19 v4.1 (2025), 7.1 (Tables 1 and 2)

İşaretine kadar doldurulup on kez tartılan 1000 mL'lik cam balon; su 19,11 °C, hava 0,2 °C daha sıcak.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. V₂₀ = m/(ρ_W − ρ_A)·(1 − ρ_A/ρ_B)·[1 − γ(t − 20 °C)] + δV_men + δV_rep (eşitlik 5); alet sıcaklığı t = t_W + δt_s.
2. Su yoğunluğu t_W'de Tanaka formülünden, ρ_W = a₅[1 − (t + a₁)²(t + a₂)/(a₃(t + a₄))]; formülün kendi belirsizliği 4,5·10⁻⁷ g/mL ve suyun saflığı için 5 ppm (dikdörtgen) (6.3.3). Suyun sıcaklığı böylece hem yoğunluğa hem de aletin genleşmesine, korelasyonlarıyla birlikte girer.
3. Termometre U = 0,009 °C (k = 2) ve çözünürlük 0,001 °C; su ile alet arasındaki fark |t_A − t_W|/(8√3) ile (6.3.2); hava yoğunluğu, ağırlıkların yoğunluğu ve genleşme katsayısı örnekteki gibi.
4. m = 996,9505 g, u(m) = 0,0013 g; her iki tartım için U(terazi) = 0,0018 g (k = 2) ve 0,1 mg çözünürlükten; menisküs ±0,015 mL (dikdörtgen, optik yardımcılar); n = 10 için s(V₀) = 0,044 mL.
5. Tekrarlanabilirlik (0,014 mL) ve menisküs (0,0087 mL) baskındır: u(V₂₀) = 0,017 mL, ν_eff = 19, k = 2,14; V₂₀ = 999,628 mL ± 0,036 mL.
6. Örnek k'yı 'yaklaşık %95' için verir, ancak k, EA-4/02'deki gibi %95,45 için t(ν_eff)'dir; proje p = %95,45 kullanır.

**Sonuç:**

- V_20 = (999,622 ± 0,036) mL; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,14 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F62: 7.1.1.4'te belirtilen girdilerle (0,16 °C, 6 Pa — formülde 60 —, %0,6 bağıl nem ve 2,4·10⁻⁴ formül belirsizliği) hava yoğunluğu formülü 1,76·10⁻⁶ g/mL değil u(ρ_A) = 6,0·10⁻⁷ g/mL verir; 7.1.1.5 u(ρ_B) = 0,06/2 = 0,03 g/mL verirken Tablo 2 0,0346 g/mL (= 0,06/√3) kullanır; 7.1.1.3 u(ρ_W,t)'yi metinde 1,024·10⁻⁶ g/mL, formülde 1,24·10⁻⁶ g/mL yazar; β·ρ_W·u(t_W) = 8,9·10⁻⁷ g/mL'dir; Tablo 2 kütle için gerekçesiz ν = 50 verir. Hiçbiri u(V₂₀)'yi basılı basamaklarda değiştirmez.

**Çalıştırma:** `uncertainty run library/flow/cg19-ex1-flask-1000ml.json --lang tr`

### 100 mL'lik cam piknometrenin gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 2)

`cg19-ex2-pycnometer-100ml` — EURAMET Calibration Guide No. 19 v4.1 (2025), 7.2 (Tables 3 and 4)

On kez tartılan 100 mL'lik piknometre; su 18,94 °C, hava 0,23 °C daha sıcak; menisküs katkısı ihmal edilebilir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. V₂₀ = m/(ρ_W − ρ_A)·(1 − ρ_A/ρ_B)·[1 − γ(t − 20 °C)] + δV_men + δV_rep (eşitlik 5); alet sıcaklığı t = t_W + δt_s.
2. Su yoğunluğu t_W'de Tanaka formülünden, ρ_W = a₅[1 − (t + a₁)²(t + a₂)/(a₃(t + a₄))]; formülün kendi belirsizliği 4,5·10⁻⁷ g/mL ve suyun saflığı için 5 ppm (dikdörtgen) (6.3.3). Suyun sıcaklığı böylece hem yoğunluğa hem de aletin genleşmesine, korelasyonlarıyla birlikte girer.
3. Termometre U = 0,009 °C (k = 2) ve çözünürlük 0,001 °C; su ile alet arasındaki fark |t_A − t_W|/(8√3) ile (6.3.2); hava yoğunluğu, ağırlıkların yoğunluğu ve genleşme katsayısı örnekteki gibi.
4. m = 99,7500 g, u(m) = 0,000 11 g; her iki tartım için U(terazi) = 0,000 15 g (k = 2) ve 0,1 mg çözünürlükten; n = 10 için s(V₀) = 0,0026 mL.
5. Tekrarlanabilirlik baskındır: u(V₂₀) = 0,89 µL, ν_eff = 13, k = 2,21; V₂₀ = 100,014 mL ± 0,0020 mL.
6. Örnek k'yı 'yaklaşık %95' için verir, ancak k, EA-4/02'deki gibi %95,45 için t(ν_eff)'dir; proje p = %95,45 kullanır.

**Sonuç:**

- V_20 = (100,0142 ± 0,0020) mL; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,23 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F62: Tablo 4 ν_eff(V₂₀) = 1727 verir; 7.2.5 ise Welch–Satterthwaite formülünün yeniden ürettiği ve k = 2,21'in karşılık geldiği 13 değerini verir. Örnek 1'deki gibi u(ρ_A) = 1,74·10⁻⁶ g/mL belirtilen girdilerden çıkmaz (4,8·10⁻⁷ g/mL).

**Çalıştırma:** `uncertainty run library/flow/cg19-ex2-pycnometer-100ml.json --lang tr`

### Su sayacı kalibrasyonu: tek çalışmanın bağıl hatası

`ea402-s12-water-meter-error` — EA-4/02 M:2022, S12

EA-4/02 S12 (ikinci bütçe): tek bir çalışma için sayacın bağıl gösterge hatası.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. e_X = (V_iX + δV_iX2 − δV_iX1)/V_X − 1: sayacın başlangıç ve bitiş okumalarının her biri ±0,1 L çözünürlük terimi taşır.
2. V_X birinci bütçeden gelir (199,93 L, u = 0,109 L).

**Sonuç:**

- e_X = (0,0003 ± 0,0014); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/flow/ea402-s12-water-meter-error.json --lang tr`

### Su sayacı kalibrasyonu: üç çalışmanın ortalaması

`ea402-s12-water-meter-mean-error` — EA-4/02 M:2022, S12

EA-4/02 S12 (üçüncü bütçe): üç çalışmanın ortalama bağıl hatası; Welch–Satterthwaite formülü ve t dağılımından k.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. e_X,ort = ē_X + δe_X: üç çalışmanın saçılımı (s = 0,001, u = s/√3, 2 serbestlik derecesi) tek çalışmanın belirsizliğine (0,68 × 10⁻³) eklenir.
2. s = 0,00104 ve u = s/√3 = 0,60 × 10⁻³ (EA-4/02 s = 0,001 yazar); birleşik u = 0,91 × 10⁻³ ve ν_eff = 10 olduğundan k = t₀,₉₅₄₅(10) = 2,28 (EA-4/02 Tablo E.1) ve U = 0,002.

**Sonuç:**

- e_Xav = (0,001 ± 0,002); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,28 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/flow/ea402-s12-water-meter-mean-error.json --lang tr`

### Su sayacı kalibrasyonu: referans hacim

`ea402-s12-water-meter-volume` — EA-4/02 M:2022, S12

EA-4/02 S12 (birinci bütçe): 2500 L/h'te bir konut tipi su sayacından geçen hacmin hacimsel standartta ölçülmesi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. V_X = V_S[1 + β_S(t_S − 20)][1 + γ_W(t_X − t_S)][1 − κ_W p_X]; standardın okuması sayaçtaki suyun sıcaklık ve basıncına düzeltilir.
2. Sayaçtaki su sıcaklığı (±2 °C) en büyük düzeltme belirsizliğini verir: c(t_X) = 0,030 L/K.
3. V_X = 199,93 L, u = 0,109 L.

**Sonuç:**

- V_X = (199,93 ± 0,22) L; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/flow/ea402-s12-water-meter-volume.json --lang tr`

### Standart basınç ve sıcaklığa indirgenmiş hacimsel debi (UKAS M3003 K.9)

`m3003-k9-flowrate-standard-conditions` — UKAS M3003 ed. 6 (2024), Appendix K.9

1018 hPa ve 295,1 K'de 4–20 mA'lik bir debimetreden debi, 1013,25 hPa ve 293,15 K'de ifade edilir; üç kısmi bütçe tek modelde.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Q_s = Q_m·(P_m/P_s)·(T_s/T_m) + δQ_s (K.9.2); Q_m = A·i + B, A = 6,25 L s⁻¹ mA⁻¹, B = −24,95 L s⁻¹ (K.9.4): i = 6,0 mA, Q_m = 12,55 L s⁻¹ verir.
2. Kısmi bütçeler: u(Q_m) = 0,207 L s⁻¹ (kalibrasyon, sürüklenme, ortam, akım); u(P_m) = 1,322 hPa; u(T_m) = 0,144 K; Q_s tekrarlanabilirliği s = 0,18 L s⁻¹ (ν = 24, tek okuma).
3. Q_s = 12,53 L s⁻¹, u_c = 0,275 L s⁻¹, ν_eff = 130; Q_s = (12,53 ± 0,55) L s⁻¹ (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- Q_s = (12,52 ± 0,55) L/s; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- ν_eff = 130 ile %95,45 için kapsam faktörü 2,03'tür (Ek B tablosu: 100'de 2,025); bu U = 0,56 L/s verirdi; örnek k = 2 kullanır.

**Çalıştırma:** `uncertainty run library/flow/m3003-k9-flowrate-standard-conditions.json --lang tr`

### Pitot tüpüyle hava hızı (NIST TN 1900 E4)

`nist-tn1900-e4-pitot-tube` — NIST Technical Note 1900, Example E4

TN 1900 E4 (Kline ve McClintock, 1953'ten): fark basıncı, hava basıncı ve sıcaklıktan hava hızı.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. v = √(2ΔR_sT/p); burada hava yoğunluğu ρ = p/(R_sT) ideal gaz yasasından gelir; R_s = 287,058 J kg⁻¹ K⁻¹ kuru havanın özgül gaz sabitidir.
2. Δ = 1,993 kPa (U-tüp manometre), p = 101,4 kPa (Bourdon manometre) ve T = 292,8 K (cıvalı cam termometre); %95 genişletilmiş belirsizlikleri 0,025 kPa, 2,1 kPa ve 0,11 K'dir, standart belirsizlikler bunların yarısıdır.
3. Üslü bir bağıntıda bağıl belirsizlikler ½ ağırlıkla kareler toplamı olarak birleşir: u(v)/v = ½ √((u_Δ/Δ)² + (u_p/p)² + (u_T/T)²) = %0,61; baskın katkı hava basıncınındır. Monte Carlo aralığı biraz daha düşük değerlere kaymıştır (v ∝ p^−½) ve GUM aralığı u'nun iki anlamlı basamağı için doğrulanmaz (uç nokta farkları 0,007 ve 0,009 m/s, δ = 0,005 m/s); TN 1900'de basılı iki aralık arasındaki ikinci basamak farkları da bunu gösterir.

**Sonuç:**

- v = (57,48 ± 0,68) m/s; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- Verilen denklem ve girdilerle v = 57,48 m/s ve u(v) = 0,348 m/s bulunur. TN 1900 ise v = 40,64 m/s ve u(v) = 0,25 m/s basar; bunlar aynı değerlerin √2'ye bölünmüşüdür, yani 2 çarpanı olmadan v = √(ΔR_sT/p) bağıntısının sonucudur. Bağıl belirsizlik (%0,61) her ikisinde de aynıdır.

**Çalıştırma:** `uncertainty run library/flow/nist-tn1900-e4-pitot-tube.json --lang tr`

## Genel (yöntem doğrulama)

### Normal dağılımlı girdilerle toplamsal model

`jcgm101-9.2.2-additive-gaussian` — JCGM 101:2008, 9.2.2; JCGM GUM-5:2026, Example 6, Table 6.1

JCGM 101 9.2.2: her biri N(0, 1) olan dört bağımsız girdi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Y = X1 + X2 + X3 + X4 modeli doğrusaldır ve tüm duyarlılık katsayıları 1'dir; bu nedenle GUM belirsizlik yayılım yasası u(y)² = Σ u(xᵢ)² sonucunu tam olarak verir.
2. Çıktı tam olarak normal dağılımlıdır, N(0, 2²); GUM %95 aralığı ±1,96 × 2 = ±3,92 tamdır, Monte Carlo yöntemi bunu yeniden üretir ve GUM sonucu doğrulanır.

**Sonuç:**

- Y = (0,0 ± 3,9); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/general/jcgm101-9.2.2-additive-gaussian.json --lang tr`

### Dikdörtgen dağılımlı girdilerle toplamsal model

`jcgm101-9.2.3-additive-rectangular` — JCGM 101:2008, 9.2.3; JCGM GUM-5:2026, Example 6, Table 6.2

JCGM 101 9.2.3: her biri standart sapması 1 olan dikdörtgen dağılımlı dört bağımsız girdi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Y = X1 + X2 + X3 + X4 modeli doğrusaldır ve tüm duyarlılık katsayıları 1'dir; bu nedenle GUM belirsizlik yayılım yasası u(y)² = Σ u(xᵢ)² sonucunu tam olarak verir.
2. Çıktı olasılık yoğunluk fonksiyonu dört dikdörtgen dağılımın konvolüsyonudur: %95 uç noktaları ±2√3[2 − (3/5)^{1/4}] = ±3,88'dir (JCGM 101 Ek E), GUM aralığı ±3,92'nin biraz içindedir.
3. u(y) = 2,0 için iki anlamlı basamakta sayısal tolerans δ = 0,05'tir; uç noktalar arasındaki kesin fark 3,920 − 3,881 = 0,039, δ'dan küçük olduğundan GUM aralığı doğrulanır, ancak sınıra çok yakındır.

**Sonuç:**

- Y = (0,0 ± 3,9); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- JCGM 101 Tablo 3, iki uyarlamalı çalışmasından birinde GUM aralığını doğrulanmış, diğerinde doğrulanmamış olarak verir (9.2.3.4): δ = 0,05'e karşı d ≈ 0,039 olduğunda sonucu uç noktalardaki Monte Carlo gürültüsü belirleyebilir.

**Çalıştırma:** `uncertainty run library/general/jcgm101-9.2.3-additive-rectangular.json --lang tr`

### Baskın dikdörtgen girdili toplamsal model

`jcgm101-9.2.4-additive-dominant-rectangular` — JCGM 101:2008, 9.2.4; JCGM GUM-5:2026, Example 6, Table 6.3

JCGM 101 9.2.4: 9.2.3 ile aynı, ancak X4'ün standart sapması 10'dur.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Y = X1 + X2 + X3 + X4 modeli doğrusaldır ve tüm duyarlılık katsayıları 1'dir; bu nedenle GUM belirsizlik yayılım yasası u(y)² = Σ u(xᵢ)² sonucunu tam olarak verir.
2. Çıktı neredeyse dikdörtgen dağılımlıdır: %95 aralığı yaklaşık ±17,0 iken GUM aralığı (normal varsayımı) ±19,9'dur; bir girdi baskın olduğunda merkezi limit teoremi geçerli olmaz.
3. GUM sonucu n_dig = 2 için (δ = 0,5) doğrulanmaz, n_dig = 1 için (δ = 5) doğrulanırdı.

**Sonuç:**

- Y = (0 ± 20); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/general/jcgm101-9.2.4-additive-dominant-rectangular.json --lang tr`

### Toplamalı iki değişkenli model, Gauss girdiler (JCGM 102 9.2.2)

`jcgm102-9.2.2-additive-gaussian` — JCGM 102:2011, 9.2.2 (Table 3)

JCGM 102:2011 9.2.2: ortak bir Gauss etkisini paylaşan iki çıktı; GUM sonuçları kesindir ve Monte Carlo yöntemiyle doğrulanır.

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. İki değişkenli toplamalı model Y₁ = X₁ + X₃, Y₂ = X₂ + X₃, bağımsız girdilerle; X₃ her iki çıktıda ortak olan bir etkidir (9.2.1).
2. GUM yaklaşımı: C_x = [[1, 0, 1], [0, 1, 1]] ile U_y = C_x U_x C_xᵀ (formül 3); N(y, U_y) için %95 bölgeler: k_p = 2,45 (χ²₂) ile elips ve k_q = 2,24 (q = 0,975) ile dikdörtgen (Tablo 1 ve 2).
3. 10⁶ denemeli Monte Carlo yöntemi: k_p, (y_r − y)ᵀ U_y⁻¹ (y_r − y) uzaklığına göre sıralanan çekilişlerden (7.7.2), k_q ise max_j |y_j,r − y_j|/u(y_j) ile (7.7.3); doğrulama Madde 8'deki gibi iki anlamlı basamakla.

**Sonuç:**

- Y1 = 0,00, u(Y1) = 1,4 (GUM)
- Y2 = 0,00, u(Y2) = 1,4 (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/general/jcgm102-9.2.2-additive-gaussian.json --lang tr`

### Toplamalı iki değişkenli model, dikdörtgen ortak etki (JCGM 102 9.2.3)

`jcgm102-9.2.3-additive-rectangular` — JCGM 102:2011, 9.2.3 (Table 4)

JCGM 102:2011 9.2.3: ortak etki X₃ dikdörtgen (standart sapma 1) olmak üzere 9.2.2 ile aynı; GUM kapsam faktörleri doğrulanmaz.

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. İki değişkenli toplamalı model Y₁ = X₁ + X₃, Y₂ = X₂ + X₃, bağımsız girdilerle; X₃ her iki çıktıda ortak olan bir etkidir (9.2.1).
2. GUM yaklaşımı: C_x = [[1, 0, 1], [0, 1, 1]] ile U_y = C_x U_x C_xᵀ (formül 3); N(y, U_y) için %95 bölgeler: k_p = 2,45 (χ²₂) ile elips ve k_q = 2,24 (q = 0,975) ile dikdörtgen (Tablo 1 ve 2).
3. 10⁶ denemeli Monte Carlo yöntemi: k_p, (y_r − y)ᵀ U_y⁻¹ (y_r − y) uzaklığına göre sıralanan çekilişlerden (7.7.2), k_q ise max_j |y_j,r − y_j|/u(y_j) ile (7.7.3); doğrulama Madde 8'deki gibi iki anlamlı basamakla.
4. Tahminler, standart belirsizlikler ve korelasyon katsayısı uyuşur; ancak k_p (2,45'e karşı 2,38) ve k_q (2,24'e karşı 2,15) 0,05 toleransından fazla farklıdır (9.2.3.4).

**Sonuç:**

- Y1 = 0,00, u(Y1) = 1,4 (GUM)
- Y2 = 0,00, u(Y2) = 1,4 (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/general/jcgm102-9.2.3-additive-rectangular.json --lang tr`

### Toplamalı iki değişkenli model, baskın dikdörtgen ortak etki (JCGM 102 9.2.4)

`jcgm102-9.2.4-additive-dominant-rectangular` — JCGM 102:2011, 9.2.4 (Table 5)

JCGM 102:2011 9.2.4: ortak dikdörtgen etkinin standart sapması 3'tür; r(y₁, y₂) = 0,9 ve GUM k_p ile k_q değerleri yaklaşık %7 ve %20 büyüktür.

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. İki değişkenli toplamalı model Y₁ = X₁ + X₃, Y₂ = X₂ + X₃, bağımsız girdilerle; X₃ her iki çıktıda ortak olan bir etkidir (9.2.1).
2. GUM yaklaşımı: C_x = [[1, 0, 1], [0, 1, 1]] ile U_y = C_x U_x C_xᵀ (formül 3); N(y, U_y) için %95 bölgeler: k_p = 2,45 (χ²₂) ile elips ve k_q = 2,24 (q = 0,975) ile dikdörtgen (Tablo 1 ve 2).
3. 10⁶ denemeli Monte Carlo yöntemi: k_p, (y_r − y)ᵀ U_y⁻¹ (y_r − y) uzaklığına göre sıralanan çekilişlerden (7.7.2), k_q ise max_j |y_j,r − y_j|/u(y_j) ile (7.7.3); doğrulama Madde 8'deki gibi iki anlamlı basamakla.

**Sonuç:**

- Y1 = 0,00, u(Y1) = 3,2 (GUM)
- Y2 = 0,00, u(Y2) = 3,2 (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 2,45; dikdörtgen k_q = 2,24

**Çalıştırma:** `uncertainty run library/general/jcgm102-9.2.4-additive-dominant-rectangular.json --lang tr`

### Bir kabın patlama dayanımı: uygunluk olasılığı (JCGM 106 7.3)

`jcgm106-7.3-bursting-strength` — JCGM 106:2012, 7.3.3, Example 2

JCGM 106:2012 7.3 Örnek 2: tek alt tolerans sınırı B ≥ 490 kPa, tahribatlı deney b = 509,7 kPa, u = 8,6 kPa.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Alt sınır için z = (y − T_L)/u = (509,7 − 490)/8,6 = 2,3 ve p_c = Φ(2,3) = 0,99 (Formül 8, 10).
2. Olasılık, deneyin yok ettiği kabın deneyden önceki durumuna aittir: uygun olmuş olma olasılığı %99'dur.

**Sonuç:**

- container için B ≥ 490 kPa şartına uygunluk olasılığı: p_c = %99; basit kabul kuralına göre kabul.

**Çalıştırma:** `uncertainty run library/general/jcgm106-7.3-bursting-strength.json --lang tr`

### Zener diyotun kırılma gerilimi: uygunluk olasılığı (JCGM 106 7.3)

`jcgm106-7.3-zener-diode` — JCGM 106:2012, 7.3.3, Example 1

JCGM 106:2012 7.3 Örnek 1: tek üst tolerans sınırı V_b ≤ −5,40 V, ölçülen −5,47 V, u = 0,05 V.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Ölçümden sonra V_b hakkındaki bilgi, beklenen değeri −5,47 V ve standart sapması 0,05 V olan normal bir olasılık yoğunluğudur.
2. Üst sınır için z = (T_U − y)/u = (−5,40 + 5,47)/0,05 = 1,40 ve p_c = Φ(1,40) = 0,92 (Formül 9, 10): diyotun uygun olma olasılığı %92'dir.

**Sonuç:**

- diode için V_b ≤ -5,4 V şartına uygunluk olasılığı: p_c = %92; basit kabul kuralına göre kabul.

**Çalıştırma:** `uncertainty run library/general/jcgm106-7.3-zener-diode.json --lang tr`

### Motor yağının kinematik viskozitesi: iki taraflı tolerans (JCGM 106 7.4)

`jcgm106-7.4-motor-oil-viscosity` — JCGM 106:2012, 7.4, Example

JCGM 106:2012 7.4 Örnek: 100 °C'de SAE 40 yağ, 12,5 mm²/s ≤ Y ≤ 16,3 mm²/s, ölçülen 13,6 mm²/s, u = 1,8 mm²/s.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. p_c = Φ((T_U − y)/u) − Φ((T_L − y)/u) = Φ(1,5) − Φ(−0,61) = 0,93 − 0,27 = 0,66 (Formül 11).
2. Ölçülen değer tolerans aralığındadır ve basit kabulle numune kabul edilir; ancak uygunluk olasılığı yalnızca %66'dır: belirsizlik toleransa göre büyüktür (C_m = 3,8/(4 × 1,8) = 0,53).

**Sonuç:**

- oil için 12,5 mm²/s ≤ Y ≤ 16,3 mm²/s şartına uygunluk olasılığı: p_c = %66; basit kabul kuralına göre kabul.

**Çalıştırma:** `uncertainty run library/general/jcgm106-7.4-motor-oil-viscosity.json --lang tr`

### Hayvansal ürünlerde nandrolon: tarama testi için karar sınırı (JCGM 106 8.3.3)

`jcgm106-8.3-nandrolone` — JCGM 106:2012, 8.3.3, Example 2

JCGM 106:2012 8.3.3 Örnek 2: eşik 2,00 µg/L; eşiğin aşılmış olma olasılığı en az %95 ise sonuç şüphelidir; on katkılı kör numuneden tekrarüretilebilirlik s = 0,20 µg/L, ν = 9 ile t dağılımı.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Derişimin olasılık yoğunluğu, ölçeği s = 0,20 µg/L ve serbestlik derecesi ν = 9 olan ölçeklenmiş ve kaydırılmış t dağılımıdır (JCGM 101 6.4.9).
2. Koruma bandı g = t₀,₉₅;₉ × s = 1,83 × 0,20 = 0,37 µg/L ve karar sınırı A = T + g = 2,37 µg/L (korumalı ret).

**Çalıştırma:** `uncertainty run library/general/jcgm106-8.3-nandrolone.json --lang tr`

### Hız sınırı denetimi: korumalı ret (JCGM 106 8.3.3)

`jcgm106-8.3-speed-limit` — JCGM 106:2012, 8.3.3, Example 1

JCGM 106:2012 8.3.3 Örnek 1: hız sınırı 100 km/h, bağıl standart belirsizliği %2 olan radar; yalnızca sınırın aşılmış olma olasılığı en az %99,9 ise ceza.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Ölçülen v için hız hakkındaki bilgi, standart sapması 0,02v olan normal bir olasılık yoğunluğudur.
2. Kabul sınırı v_max, (v_max − v₀)/(0,02 v_max) = z = 3,09 eşitliğini sağlar; dolayısıyla v_max = v₀/(1 − 0,02z) = 100/(1 − 0,0618) ≈ 107 km/h.
3. 100 km/h – 107 km/h aralığı bir koruma bandıdır: 107 km/h veya daha yüksek ölçülen hız, sınırın aşılmış olma olasılığını en az %99,9 yapar.

**Sonuç:**

- car A için v ≤ 100 km/h şartına uygunluk olasılığı: p_c = %0,86; korumalı ret kuralına göre kabul.
- car B için v ≤ 100 km/h şartına uygunluk olasılığı: p_c = %0,011; korumalı ret kuralına göre ret.

**Notlar:**

- Program koşulu z = Φ⁻¹(0,999) = 3,0902 ile tam çözer ve 106,59 km/h verir; standart 107 km/h'ye yuvarlar. 105 ve 108 km/h ölçülen hızlar kabul sınırının iki yanını gösterir.

**Çalıştırma:** `uncertainty run library/general/jcgm106-8.3-speed-limit.json --lang tr`

### Bilyalı rulmanlar: istenen tüketici riski için kabul sınırı (JCGM 106 9.5.4)

`jcgm106-9.5-ball-bearings` — JCGM 106:2012, 9.5.4, Example

JCGM 106:2012 9.5.4 Örnek: radyal hata hareketi ≤ 2 µm, ortalaması 1 µm ve standart sapması 0,5 µm olan gama süreç dağılımı, ölçüm sistemi u_m = 0,25 µm; küresel tüketici riski en çok %0,1.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Radyal hata hareketi pozitiftir; bu nedenle süreç dağılımı α = (1/0,5)² = 4 ve λ = 1/0,5² = 4 µm⁻¹ olan bir gama yoğunluğudur (Ek B, Formül 22); rulmanların %4,2'si 2 µm'yi aşar.
2. A = T − 2r u_m ile R_C(r) ve R_P(r), Formül (23) ve (24)'ten bulunur; R_C = %0,1, r ≈ 0,65 için sağlanır; dolayısıyla A ≈ 1,7 µm ve R_P ≈ %7,5'tir.

**Notlar:**

- Metin kabul aralığını 0 ≤ Y_m ≤ A olarak verir ve R_C için Formül (23) alt sınır 0'ı içerir; ancak R_P için Formül (24) bunu dışarıda bırakır. Basılı R_P ≈ %7,5 değeri (24)'ten çıkar; alt kabul sınırı 0 ile R_P %8,9 olurdu (araştırma günlüğü F45). Proje Y_m ≤ A kabul aralığını kullanır; bu hem R_C'yi hem R_P'yi yeniden üretir (alt sınırın R_C'ye etkisi yoktur).

**Çalıştırma:** `uncertainty run library/general/jcgm106-9.5-ball-bearings.json --lang tr`

### Hassas dirençler: küresel tüketici ve üretici riskleri (JCGM 106 9.5.3)

`jcgm106-9.5-precision-resistors` — JCGM 106:2012, 9.5.3, Example

JCGM 106:2012 9.5.3 Örnek: σ = 0,12 Ω olan bir süreçten 1500 Ω ± 0,2 Ω dirençler, u_m = 0,04 Ω ile ve 1499,82 Ω ile 1500,18 Ω kabul sınırlarıyla denetlenir.

**Yaklaşım:** uygunluk değerlendirmesi (JCGM 106)

**Adımlar:**

1. Denetim olmadan p_c = tolerans aralığı üzerinde ∫ φ(η; 1500, 0,12²) dη = 0,90: dirençlerin yaklaşık %10'u uygun olmazdı (Formül 21).
2. w = 0,02 Ω = 0,25U simetrik koruma bantlarıyla korumalı kabul.
3. Formül (19) ve (20) sayısal integrasyonla: R_C = %1 ve R_P = %7. 100 dirençten 83'ü doğru olarak kabul, 7'si yanlışlıkla ret, 9'u doğru olarak ret, 1'i yanlışlıkla kabul edilir; kabul edilenlerin %99'u uygundur.

**Çalıştırma:** `uncertainty run library/general/jcgm106-9.5-precision-resistors.json --lang tr`

### Geniş üçgen bölenli çarpım ve oran y = ab/c (NIST Uncertainty Machine §3)

`num-3-ratio-abc` — T. Lafarge, §3, Figure 1

NUM kılavuzu §3: normal a, dikdörtgen b ve bağıl belirsizliği %30 olan üçgen bölen c; birinci mertebe GUM sonucu Monte Carlo sonucundan çok uzaktır.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. a ~ N(32, 0,5²); b ortalaması 0,9 ve standart sapması 0,025 olan dikdörtgen (yarı genişlik 0,025√3); c ortalaması 1 ve standart sapması 0,3 olan simetrik üçgen (yarı genişlik 0,3√6 = 0,735).
2. Tahminlerde Gauss formülü: y = 32 × 0,9 / 1 = 28,8 ve u(y)/y = √((0,5/32)² + (0,025/0,9)² + 0,3²), u(y) = 8,7; varyansın %99'u c'den gelir.
3. 1/c, [0,27; 1,73] aralığında belirgin biçimde dışbükey olduğundan Monte Carlo ortalaması 32,2, standart sapması 13 ve %95 aralığı (18,2; 67) olur: y'nin dağılımı belirgin biçimde sağa çarpıktır ve GUM aralığı 28,8 ± 17,1 doğrulanmaz.

**Sonuç:**

- y = (29 ± 17); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- §3'ün metni GUM yönteminin y = 32,2 ve u(y) = 12,5 verdiğini söyler; ancak Şekil 1'de basılı NIST Uncertainty Machine çıktısı Gauss formülü için y = 28,8 ve u(y) = 8,7 verir (32,2 ve 13 Monte Carlo ortalaması ve standart sapmasıdır). Program Şekil 1'deki değerleri yeniden üretir.

**Çalıştırma:** `uncertainty run library/general/num-3-ratio-abc.json --lang tr`

### 1 mL mikropipetin sapması: toleransta eylem sınırlı grafik (Nordtest TR 569 Örnek 11)

`tr569-ex11-micropipette-tolerance` — Nordtest TR 569 ed. 6 (2026), Chapter 14, Example 11

Sabit hacimli 1 mL mikropipet on kez test edilir (Nordtest TR 626); ortalama hacim, laboratuvarın ± %1 toleransındaki eylem sınırlarıyla nominal değere karşı çizilir.

**Yaklaşım:** iç kalite kontrolü için kontrol grafiği (Nordtest TR 569)

**Adımlar:**

1. Merkez çizgisi nominal değer 1,00 mL; eylem sınırları 1,00 ± 0,01 mL; uyarı sınırı yok (cihaz yeterliliği, Bölüm 11).

**Sonuç:**

- X-grafiği: merkez çizgisi 1,0000 mL; eylem sınırları 0,99000 1,0100 mL (toleransta eylem sınırları)

**Çalıştırma:** `uncertainty run library/general/tr569-ex11-micropipette-tolerance.json --lang tr`

### Taşıt emisyonlarında karbon monoksit, tekrarüretilebilirlik verilerinden (ISO 21748 C.1)

`iso21748-c1-co-emissions` — ISO 21748:2017, Annex C.1

ISO 21748:2017 C.1: CO emisyonunun tip onay testi; sapma ve kesinlik kontrol altındayken ortak çalışmanın tekrarüretilebilirlik standart sapması standart belirsizliktir.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. ISO 5725-2'ye göre yapılan bir laboratuvarlar arası çalışma s_r = 0,22 g/km ve s_R = 0,28 g/km vermiştir; dolayısıyla s_L = √(s_R² − s_r²) = 0,17 g/km'dir.
2. Doğruluk, gaz analizörünün ve silindir tezgâhının kalibrasyonuyla kontrol edilir; bunların belirsizlikleri s_R'ye göre ihmal edilebilir; laboratuvarın tekrarlanabilirliği (0,20 g/km) çalışmanınkiyle uyumludur (ISO 21748 7.2, 7.3).
3. Ortak çalışmanın dışında kalan bir işlem yoktur; bu nedenle u = s_R = 0,28 g/km (Formül 3) ve U = 0,56 g/km'dir (k = 2); değerlendirme 2,2 g/km sınırı yakınında yapılmıştır.

**Sonuç:**

- CO = (2,20 ± 0,56) g/km (k = 2,00)

**Notlar:**

- Burada laboratuvarın tekrarlanabilirliği s_r yerine konmamıştır (ISO 21748 C.1'deki gibi); lab_repeatability = 0,20 g/km ile 7.3.2, s′_R = √(s_L² + s_l²) = 0,26 g/km verirdi.

**Çalıştırma:** `uncertainty run library/general/iso21748-c1-co-emissions.json --lang tr`

### Aerobik koloni sayımı, un: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-flour` — ISO 21748:2017, Annex C.3, Tables C.2–C.4

ISO 21748:2017 C.3 (AOAC 990.12, un): ortak çalışmanın s_R ve s_r değerlerinden, laboratuvarın daha iyi tekrarlanabilirliğinden (7.3.2) ve bir numune hazırlama teriminden log10(sayım)'ın bağıl belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Ortak çalışma (12 laboratuvar, ISO 5725-2): log10(sayım)'a göre s_R = %5,8, s_r = %5,3; dolayısıyla s_L = √(s_R² − s_r²) = %2,4.
2. Laboratuvarın 10 tekrardan bulunan tekrarlanabilirliği s_l = %5,0'dır; ISO 21748 7.3.2 bunun s_r yerine kullanılmasına izin verir: s′_R = √(s_L² + s_l²) = %5,5.
3. Çalışmanın kapsamadığı alt örnekleme ve tartım %3,0 ekler (uzman tahmini); u = √(s′_R² + 3,0²) = %6,3 ve U = %12,6 (k = 2).

**Sonuç:**

- U = %12 (k = 2,00), ölçülen değere göre bağıl

**Notlar:**

- ISO 21748 Tablo C.4 un için u = %6,4 ve C.3.7 U = %12,8 basar; ancak kendi girdileri √(2,4² + 5,0² + 3,0²) = %6,30 (yuvarlanmamış s_L ile %6,29) ve U = %12,6 verir. Tablo C.5'teki un aralıkları basılı %12,8'den çıkar. Proje yeniden hesaplanan değerleri kontrol eder.

**Çalıştırma:** `uncertainty run library/general/iso21748-c3-aerobic-plate-count-flour.json --lang tr`

### Aerobik koloni sayımı, karides: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-shrimp` — ISO 21748:2017, Annex C.3, Tables C.2–C.4

ISO 21748:2017 C.3 (AOAC 990.12, karides): ortak çalışmanın s_R ve s_r değerlerinden, laboratuvarın daha iyi tekrarlanabilirliğinden (7.3.2) ve bir numune hazırlama teriminden log10(sayım)'ın bağıl belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Ortak çalışma (12 laboratuvar, ISO 5725-2): log10(sayım)'a göre s_R = %11,1, s_r = %9,8; dolayısıyla s_L = √(s_R² − s_r²) = %5,2.
2. Laboratuvarın 10 tekrardan bulunan tekrarlanabilirliği s_l = %5,0'dır; ISO 21748 7.3.2 bunun s_r yerine kullanılmasına izin verir: s′_R = √(s_L² + s_l²) = %7,2.
3. Çalışmanın kapsamadığı alt örnekleme ve tartım %3,0 ekler (uzman tahmini); u = √(s′_R² + 3,0²) = %7,8 ve U = %15,6 (k = 2).

**Sonuç:**

- U = %16 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/general/iso21748-c3-aerobic-plate-count-shrimp.json --lang tr`

### Aerobik koloni sayımı, sebze: düzeltilmiş tekrarüretilebilirlik (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-vegetables` — ISO 21748:2017, Annex C.3, Tables C.2–C.4

ISO 21748:2017 C.3 (AOAC 990.12, sebze): ortak çalışmanın s_R ve s_r değerlerinden, laboratuvarın daha iyi tekrarlanabilirliğinden (7.3.2) ve bir numune hazırlama teriminden log10(sayım)'ın bağıl belirsizliği.

**Yaklaşım:** top-down (validasyon ve kalite kontrol verileri)

**Adımlar:**

1. Ortak çalışma (12 laboratuvar, ISO 5725-2): log10(sayım)'a göre s_R = %9,2, s_r = %6,3; dolayısıyla s_L = √(s_R² − s_r²) = %6,7.
2. Laboratuvarın 10 tekrardan bulunan tekrarlanabilirliği s_l = %5,0'dır; ISO 21748 7.3.2 bunun s_r yerine kullanılmasına izin verir: s′_R = √(s_L² + s_l²) = %8,4.
3. Çalışmanın kapsamadığı alt örnekleme ve tartım %3,0 ekler (uzman tahmini); u = √(s′_R² + 3,0²) = %8,9 ve U = %17,8 (k = 2).

**Sonuç:**

- U = %18 (k = 2,00), ölçülen değere göre bağıl

**Çalıştırma:** `uncertainty run library/general/iso21748-c3-aerobic-plate-count-vegetables.json --lang tr`

## Uzunluk

### Kumpasın 150 mm'de kalibrasyonu

`ea402-s10-vernier-calliper` — EA-4/02 M:2022, S10

EA-4/02 S10: kumpas hatasının mastar bloğuyla ölçülmesi; iki dikdörtgen terim baskındır (yamuk çıktı).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. E_X = l_iX − l_S + L·ᾱ·Δt + δl_iX + δl_M (µm).
2. Çözünürlük (±25 µm) ve mekanik etkiler (±50 µm) dikdörtgen dağılımlıdır; konvolüsyonları β = (50 − 25)/(50 + 25) = 0,33 olan yamuk dağılımdır ve %95 için k = 1,83 (Eş. S10.9), U ≈ 60 µm verir.

**Sonuç:**

- E_X = (100 ± 65) µm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/length/ea402-s10-vernier-calliper.json --lang tr`

### 90 mm'lik halka mastarın kalibrasyonu

`ea402-s13-ring-gauge-90mm` — EA-4/02 M:2022, S13

EA-4/02 S13: halka mastar çapının tek koordinatlı ölçüm makinesinde 40 mm'lik ayar halkasıyla karşılaştırılması (90 mm'ye göre µm).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. d_X = d_S + Δl + δl_0 + δl_i + δl_T + δl_P + δl_E + δl_A: ölçülen çap farkı Δl (5 gözlemden A tipi, 0,15 µm) ve komparatörün sıfırlanması (0,25 µm) ile alt bütçedeki sıcaklık terimi (0,15 µm) baskındır.
2. Eş eksenlilik hatası sıfırdan farklı ortalamalı ikinci mertebe (kosinüs) etkidir: δl_P = 2(1/D_X − 1/D_S)u²(Δc) = −0,004 µm, u = 0,0065 µm.
3. u(d_X) = 0,41 µm; U = 0,82 µm, EA-4/02 bunu yukarı yuvarlayarak 0,9 µm verir: d_X = (90,000 3 ± 0,000 9) mm. Proje bunu yeniden üretmek için tek anlamlı basamak ve yukarı yuvarlama kullanır.

**Sonuç:**

- d_X = (0,2 ± 0,9) µm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu (raporlanan değeri etkiler): bütçe, gözlemlerin ortalaması olan 49,999 536 mm yerine Δl = 49,999 55 mm ve −0,0037 µm yerine δl_P = +0,004 µm kullanarak d_X = 90,000 25 mm bulur ve 90,000 3 mm olarak raporlar. Verilerden d_X = 90,000 232 mm, yani (90,000 2 ± 0,000 9) mm elde edilir. 0,1 µm'lik fark U'nun çok içindedir.
- Bulgular: Δl ortalaması 49,999 54 mm iken bütçe 49,999 55 kullanır; metin tek gözlemin standart sapması için s = 0,18 µm verir (veriden 0,33 µm, ortalama için 0,15 µm); √(0,15² + 0,25²) = 0,29 µm'dir (basılı 0,30); eş eksenlilik düzeltmesi −0,004 µm'dir (bütçede +0,000 004 mm).

**Çalıştırma:** `uncertainty run library/length/ea402-s13-ring-gauge-90mm.json --lang tr`

### Halka mastar kalibrasyonu: sıcaklık düzeltmesi

`ea402-s13-ring-gauge-temperature` — EA-4/02 M:2022, S13

EA-4/02 S13: halka mastar karşılaştırmasının sıcaklık alt bütçesi; sıfır ortalamalı sıcaklık sapmalarının genleşme katsayılarıyla çarpımları.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. δl_T = [D_S(α_S − α_R) − D_X(α_X − α_R)]Δt_A + D_S α_S δt_S − D_X α_X δt_X + (D_S − D_X) α_R δt_R (µm); D_S = 40 mm ve D_X = 90 mm.
2. Tüm sıcaklık sapmalarının beklenen değeri sıfırdır; bu nedenle birinci terimin birinci mertebe duyarlılığı sıfırdır; ikinci mertebe terimler çarpımların varyansını tam verir: 0,018, 0,053, 0,12, 0,066 µm → u(δl_T) = 0,15 µm.

**Sonuç:**

- dl_T = (0,00 ± 0,30) µm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/length/ea402-s13-ring-gauge-temperature.json --lang tr`

### 50 mm'lik mastar bloğunun kalibrasyonu

`ea402-s4-gauge-block-50mm` — EA-4/02 M:2022, S4

EA-4/02 S4: 0 sınıfı çelik mastar bloğunun kalibre edilmiş standartla karşılaştırılması; iki sıfır ortalamalı büyüklüğün çarpımı ikinci mertebe terim gerektirir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. l_X = l_S + δl_D + δl + δl_C − L(ᾱ·δt + δα·θ) − δl_V, 50 mm'ye göre nm cinsinden (L = 50 mm).
2. δα (genleşme katsayıları farkı) ve θ (20 °C'den sapma) beklenen değerleri sıfırdır; çarpımlarının birinci mertebe duyarlılığı sıfırdır: katkısı L·u(δα)·u(θ) = 11,8 nm yalnızca ikinci mertebe terimlerle ortaya çıkar (S4.13); proje bunları etkinleştirir.
3. l_X = 49,999 926 mm ± 69 nm (k = 2).

**Sonuç:**

- l_X = (49999926 ± 68) nm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/length/ea402-s4-gauge-block-50mm.json --lang tr`

### Uç mastarı kalibrasyonu (GUM H.1)

`gum-h1-end-gauge` — JCGM 100:2008, H.1, Table H.1

JCGM 100 Ek H.1: 50 mm'lik bir uç mastarının kalibrasyonu; Welch–Satterthwaite etkin serbestlik derecesi ve %99 için k.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Bir uç mastarının boyu, aynı anma boyundaki (50 mm) bir standartla karşılaştırılarak bulunur: l = l_s + d − l_s(δα θ + α_s δθ); burada θ = θ̄ + Δ, 20 °C'den sıcaklık sapmasıdır.
2. Tahmin değerlerinde (δα = 0, δθ = 0, θ = −0,1 °C) α_s, θ̄ ve Δ'nın duyarlılık katsayıları sıfırdır; katkıları yalnızca ikinci mertebede ortaya çıkar (GUM H.1.7).
3. Serbestlik dereceleri: l_s 18, d 25,6, δα 50 (%10 güvenilir), δθ 2 (%50 güvenilir, GUM G.4.2). Welch–Satterthwaite formülü ν_eff = 16,7 verir; 16'ya kesilir ve k₉₉ = t₀,₉₉(16) = 2,92 olur.
4. u_c = 32 nm ve U₉₉ = 2,92 × 32 nm = 93 nm; l = (50,000 838 ± 0,000 093) mm.

**Sonuç:**

- l = (50000838 ± 92) nm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,92 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %99 kapsam olasılığına karşılık gelir.

**Notlar:**

- GUM yuvarlanmış u_c = 32 nm değerini çarpar (2,92 × 32 = 93 nm); yuvarlanmamış 31,7 nm ile U = 92,5 nm bulunur. İkinci mertebe 11,7 ve 1,7 nm terimleri (H.1.7) eklendiğinde u_c 34 nm'ye çıkar.
- Monte Carlo sonucu d'yi normal dağılımlı kabul eder; JCGM 101 9.5 (jcgm101-9.5-* projeleri) bunun yerine girdiler için t dağılımları ve eğrisel yamuk dağılımlar kullanır.

**Çalıştırma:** `uncertainty run library/length/gum-h1-end-gauge.json --lang tr`

### Mastar kalibrasyonu, yaklaşık model

`jcgm101-9.5-gauge-block-approximate` — JCGM 101:2008, 9.5, Tables 10 and 11; JCGM 100:2008, H.1; JCGM GUM-5:2026, Example 8, Table 8.2

JCGM 101 9.5: girdilere JCGM 101'in atadığı dağılımlarla (t, ark sinüs, eğrisel yamuk) GUM H.1 mastarı; %99 kapsam.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Bir uç mastarının boyu, aynı anma boyundaki (50 mm) bir standartla karşılaştırılarak bulunur: l = l_s + d − l_s(δα θ + α_s δθ); burada θ = θ̄ + Δ, 20 °C'den sıcaklık sapmasıdır.
2. Tahmin değerlerinde (δα = 0, δθ = 0, θ = −0,1 °C) α_s, θ̄ ve Δ'nın duyarlılık katsayıları sıfırdır; katkıları yalnızca ikinci mertebede ortaya çıkar (GUM H.1.7).
3. A tipi girdiler ölçekli ve kaydırılmış t dağılımlarına dönüşür (JCGM 101 6.4.9); kendisi tam bilinmeyen sınırlar eğrisel yamuk dağılımlara dönüşür (6.4.3) ve GUM serbestlik dereceleri bu sınırların güvenilirliğini ifade eder.
4. MCM u = 36 nm verir (GUM: 32 nm), çünkü δα·θ ve α_s·δθ çarpım terimleri birinci mertebede ihmal edilir; %99 en kısa aralık 50 mm'ye göre [745, 932] nm'dir.

**Sonuç:**

- dL = (838 ± 94) nm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,92 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %99 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/length/jcgm101-9.5-gauge-block-approximate.json --lang tr`

### Mastar kalibrasyonu, doğrusal olmayan model

`jcgm101-9.5-gauge-block-nonlinear` — JCGM 101:2008, 9.5, Tables 10 and 11; JCGM 100:2008, H.1; JCGM GUM-5:2026, Example 8

JCGM 101 9.5: girdilere JCGM 101'in atadığı dağılımlarla (t, ark sinüs, eğrisel yamuk) GUM H.1 mastarı; %99 kapsam.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Bir uç mastarının boyu, aynı anma boyundaki (50 mm) bir standartla karşılaştırılarak bulunur: l = l_s + d − l_s(δα θ + α_s δθ); burada θ = θ̄ + Δ, 20 °C'den sıcaklık sapmasıdır.
2. Tahmin değerlerinde (δα = 0, δθ = 0, θ = −0,1 °C) α_s, θ̄ ve Δ'nın duyarlılık katsayıları sıfırdır; katkıları yalnızca ikinci mertebede ortaya çıkar (GUM H.1.7).
3. A tipi girdiler ölçekli ve kaydırılmış t dağılımlarına dönüşür (JCGM 101 6.4.9); kendisi tam bilinmeyen sınırlar eğrisel yamuk dağılımlara dönüşür (6.4.3) ve GUM serbestlik dereceleri bu sınırların güvenilirliğini ifade eder.
4. MCM u = 36 nm verir (GUM: 32 nm), çünkü δα·θ ve α_s·δθ çarpım terimleri birinci mertebede ihmal edilir; %99 en kısa aralık 50 mm'ye göre [745, 932] nm'dir.

**Sonuç:**

- dL = (838 ± 94) nm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,92 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %99 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/length/jcgm101-9.5-gauge-block-nonlinear.json --lang tr`

### Alüminyum deney numunesinin kalınlığı (UKAS M3003 K.10)

`m3003-k10-sheet-thickness` — UKAS M3003 ed. 6 (2024), Appendix K.10

Bir numunenin merkezinde beş kalınlık okumasının ortalaması, kontrolsüz bir ortamda; kalibrasyon düzeltmesi uygulanmaz.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. w = w_ave + Δw_cal + δw_T (K.10.1): 1,51, 1,51, 1,52, 1,50, 1,53 mm okumaları 1,514 mm, s = 0,0114 mm, u = 0,0051 mm (ν = 4) verir.
2. Çözünürlük ±0,005 mm; kalibrasyon U = 0,015 mm (k = 2); sürüklenme sınırı max(0,002, ort(0,002, 0,004, 0,001)) = 0,002 33 mm; düzeltilmemiş hata ±0,001 mm; sıcaklık ±10 K, 11,9·10⁻⁶ K⁻¹ × 3 mm = 3,57·10⁻⁵ mm/K (K.10.7).
3. u_c = 0,009 63 mm, ν_eff = 50; w = 1,514 mm ± 0,019 mm (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- w = (1,514 ± 0,019) mm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- ν_eff = 50 ile %95,45 için kapsam faktörü 2,05'tir (Ek B); bu U = 0,020 mm verirdi; örnek k = 2 kullanır.

**Çalıştırma:** `uncertainty run library/length/m3003-k10-sheet-thickness.json --lang tr`

### 10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6)

`m3003-k6-gauge-block-10mm` — UKAS M3003 ed. 6 (2024), Appendix K.6

Standart mastar blokla karşılaştırma; iki sıcaklık etkisinin doğrusal toplanıp tek üçgen terim yapıldığı basılı bütçe.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. L_X = L_S + δL_D + ΔL − L(α·Δt + Δα·ΔT) + ΔD_C + δC + L_V(X) + δL_r (K.6.1); 10 mm'lik 2. sınıf blok, K sınıfı standarda karşı.
2. K.6.4'teki gibi sıcaklık etkileri: Δt'den ±1,6 nm ve ΔT'den ±4 nm, ikisi de Δα = ±2 µm m⁻¹ °C⁻¹ üzerinden, korelasyonlu kabul edilip toplanır: ±5,6 nm, üçgen.
3. u_c = 40,7 nm; L_X = 9,999 940 mm ± 0,081 µm (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- L_X = (9,999940 ± 0,000081) mm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F58: K.6.1 modeline göre Δt sıcaklık farkı Δα üzerinden değil ortalama genleşme katsayısı α üzerinden etki eder (L·α = 115 nm/°C, ±9,2 nm); Δα·ΔT çarpımının birinci mertebe duyarlılığı sıfırdır. m3003-k6-gauge-block-10mm-temperature-model projesine bakınız.

**Çalıştırma:** `uncertainty run library/length/m3003-k6-gauge-block-10mm.json --lang tr`

### Modelinin sıcaklık terimleriyle 10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6, EA-4/02 S4)

`m3003-k6-gauge-block-10mm-temperature-model` — UKAS M3003 ed. 6 (2024), Appendix K.6

−L(α·Δt + Δα·ΔT) teriminin modelin belirttiği gibi değerlendirildiği K.6 bütçesi; EA-4/02 S4'ün 50 mm'lik blok için yaptığı gibi (bulgu F58).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. L_X = L_S + δL_D + ΔL − L(α·Δt + Δα·ΔT) + ΔD_C + δC + L_V(X) + δL_r (K.6.1); 10 mm'lik 2. sınıf blok, K sınıfı standarda karşı.
2. Δt (±0,08 °C, dikdörtgen) −L·α = −115 nm/°C duyarlılıkla etki eder: u = 5,3 nm. Δα (±2 µm m⁻¹ °C⁻¹, üçgen) ve ΔT (±0,2 °C, dikdörtgen) beklenen değerleri sıfırdır; çarpımları yalnızca ikinci mertebe terim L·u(Δα)·u(ΔT) = 0,94 nm ile katkı verir (GUM 5.1.2 notu).
3. u_c = 40,7 nm yerine 41,0 nm; U = 82 nm (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- L_X = (9,999940 ± 0,000082) mm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/length/m3003-k6-gauge-block-10mm-temperature-model.json --lang tr`

### Normal girdilerle uç mastarı kalibrasyonu (NIST Uncertainty Machine §8)

`num-8-end-gauge-gaussian` — T. Lafarge, §8, Table 3 and Figure 3; JCGM 100:2008, H.1, Table H.1

NUM kılavuzu §8: GUM H.1'in birinci mertebe bütçesindeki altı girdiyle, hepsi normal dağılımlı; Monte Carlo yöntemi Gauss formülünün kaçırdığı ikinci mertebe katkıları bulur.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. l = l_S + d − l_S(δα·θ + α_S·δθ); l_S = 50 000 623(25) nm, d = 215(9,7) nm, δα = 0(0,58 × 10⁻⁶) °C⁻¹, θ = −0,1(0,41) °C, α_S = 11,5(1,2) × 10⁻⁶ °C⁻¹ ve δθ = 0(0,029) °C (GUM Tablo H.1), hepsi normal dağılımlı.
2. Tahminlerde δα = δθ = 0 olduğundan θ ve α_S'nin duyarlılık katsayıları sıfırdır ve Gauss formülü u(l) = 31,7 nm verir; program δα·θ ve α_S·δθ ikinci mertebe çarpımlarının kaçırıldığı uyarısını verir.
3. Monte Carlo yöntemi 33,9 nm standart sapma ve %99 olasılıksal simetrik aralığı (50 000 749; 50 000 927) nm verir; bu, GUM'un kendi aralığına, (50 000 745; 50 000 931) nm (ν_eff = 16,7'den k = 2,92), yakındır. Bu projenin GUM değerlendirmesinin aralığı, 50 000 838 ± 82 nm (k = 2,58), her uçta 7 nm kısadır ve doğrulanmaz.

**Sonuç:**

- l = (50000838 ± 82) nm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,58 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %99 kapsam olasılığına karşılık gelir.

**Notlar:**

- 'gum-h1-end-gauge' projesi GUM'un kendi değerlendirmesini yeniden üretir (θ ortalama ve çevrimsel kısmına ayrılır, Welch–Satterthwaite, k = 2,92).

**Çalıştırma:** `uncertainty run library/length/num-8-end-gauge-gaussian.json --lang tr`

## Kütle

### Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle 100 g'da hata (EURAMET cg-18 H1, seçenek 2)

`cg18-h1-100g-option2` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H1

Kalibrasyon sırasında hava yoğunluğu bilinir (990 hPa, %50 bağıl nem, 21 °C: 1,173 kg/m³); bu nedenle kaldırma kuvveti düzeltmesi uygulanır; deney ağırlıkları oda sıcaklığından 2 K farklıdır (konveksiyon).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. E = I − m_ref (7.1-1): sıfırda ve yükte yuvarlamayla (her biri d/2√3, d = 0,1 mg), tekrarlanabilirlik ve eksantriklikle gösterge I; E2 ağırlıklarının konvansiyonel kütlesi, sürüklenmesi ve hava kaldırma kuvvetiyle referans kütle m_ref (7.1.2-14).
2. 100 g'ın beş okumasından tekrarlanabilirlik (100,0006, 100,0003, 100,0005, 100,0004, 100,0005 g): s = 0,000 114 g, ν = 4; her yükte tek okumaya uygulanır (7.1.1-5).
3. Eksantriklik: ±|ΔI_ecc,maks|·L/(2L_ecc), L_ecc = 100 g'da ΔI_ecc,maks = 0,2 mg, dikdörtgen (7.1.1-10). Sürüklenme: k_D = 1,25 ile D = k_D·U(m_c), dikdörtgen (7.1.2-10, -11).
4. Welch–Satterthwaite ile ν_eff (B3-1) ve %95,45 için t dağılımından k.
5. δm_B = −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) (7.1.2-4), ρ_a = (1,173 ± 0,014) kg/m³ ve ρ = (7950 ± 70) kg/m³; belirsizliği (7.1.2-5a) modelden çıkar: göreli yaklaşık 3·10⁻⁸.
6. u(E) = 0,000149 g, ν_eff = 11, k = 2,25, U = 0,00033 g,

**Sonuç:**

- E = (0,00070 ± 0,00033) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,25 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h1-100g-option2.json --lang tr`

### Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesi olmadan 220 g'da hata (EURAMET cg-18 H1, seçenek 1)

`cg18-h1-220g-max-option1` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H1

Kalibrasyondan önce ayarlanmamış ve hava yoğunluğu bilinmiyor: kaldırma kuvveti terimi 7.1.2-5d formülünün temkinli sınırıdır ve baskındır.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. E = I − m_ref (7.1-1): sıfırda ve yükte yuvarlamayla (her biri d/2√3, d = 0,1 mg), tekrarlanabilirlik ve eksantriklikle gösterge I; E2 ağırlıklarının konvansiyonel kütlesi, sürüklenmesi ve hava kaldırma kuvvetiyle referans kütle m_ref (7.1.2-14).
2. 100 g'ın beş okumasından tekrarlanabilirlik (100,0006, 100,0003, 100,0005, 100,0004, 100,0005 g): s = 0,000 114 g, ν = 4; her yükte tek okumaya uygulanır (7.1.1-5).
3. Eksantriklik: ±|ΔI_ecc,maks|·L/(2L_ecc), L_ecc = 100 g'da ΔI_ecc,maks = 0,2 mg, dikdörtgen (7.1.1-10). Sürüklenme: k_D = 1,25 ile D = k_D·U(m_c), dikdörtgen (7.1.2-10, -11).
4. Welch–Satterthwaite ile ν_eff (B3-1) ve %95,45 için t dağılımından k.
5. Yük 200 g + 20 g, m_c = 220,0001 g, u(m_c) = 0,000 062 g; mpe(E2) = 0,30 + 0,08 mg. Kaldırma kuvveti u_rel = (0,1ρ₀/ρ_c + mpe/(4m_N))/√3 = 8,9·10⁻⁶: u = 0,001 96 g.
6. I = 220,0014 g, E = 0,0013 g, u(E) = 0,001 97 g, ν_eff ≈ 3,6·10⁵, k = 2,00, U = 0,003 94 g.

**Sonuç:**

- E = (0,0013 ± 0,0039) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h1-220g-max-option1.json --lang tr`

### Laboratuvar terazisi 220 g / 0,1 mg: ΔT = 5 K için kaldırma kuvveti terimiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 1, 7.1.2-5e formülü)

`cg18-h1-220g-max-option1-delta-t` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H1

Seçenek 1 gibi, ancak kaldırma kuvveti belirsizliği en kötü durum yerine odanın tahmini 5 K sıcaklık değişiminden çıkar.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. E = I − m_ref (7.1-1): sıfırda ve yükte yuvarlamayla (her biri d/2√3, d = 0,1 mg), tekrarlanabilirlik ve eksantriklikle gösterge I; E2 ağırlıklarının konvansiyonel kütlesi, sürüklenmesi ve hava kaldırma kuvvetiyle referans kütle m_ref (7.1.2-14).
2. 100 g'ın beş okumasından tekrarlanabilirlik (100,0006, 100,0003, 100,0005, 100,0004, 100,0005 g): s = 0,000 114 g, ν = 4; her yükte tek okumaya uygulanır (7.1.1-5).
3. Eksantriklik: ±|ΔI_ecc,maks|·L/(2L_ecc), L_ecc = 100 g'da ΔI_ecc,maks = 0,2 mg, dikdörtgen (7.1.1-10). Sürüklenme: k_D = 1,25 ile D = k_D·U(m_c), dikdörtgen (7.1.2-10, -11).
4. Welch–Satterthwaite ile ν_eff (B3-1) ve %95,45 için t dağılımından k.
5. u_rel = √(1,07·10⁻⁴ + 1,33·10⁻⁶ K⁻²·ΔT²)·ρ₀/ρ_c + mpe/(4m_N√3) = 2,03·10⁻⁶: u = 0,000 446 g.
6. u(E) = 0,000 491 g, ν_eff = 1377, k = 2,00, U = 0,000 98 g: seçenek 1'in temkinli kaldırma kuvveti sınırı U'yu dört kat büyütür.

**Sonuç:**

- E = (0,00130 ± 0,00098) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h1-220g-max-option1-delta-t.json --lang tr`

### Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 2)

`cg18-h1-220g-max-option2` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H1

Kalibrasyon sırasında hava yoğunluğu bilinir (990 hPa, %50 bağıl nem, 21 °C: 1,173 kg/m³); bu nedenle kaldırma kuvveti düzeltmesi uygulanır; deney ağırlıkları oda sıcaklığından 2 K farklıdır (konveksiyon).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. E = I − m_ref (7.1-1): sıfırda ve yükte yuvarlamayla (her biri d/2√3, d = 0,1 mg), tekrarlanabilirlik ve eksantriklikle gösterge I; E2 ağırlıklarının konvansiyonel kütlesi, sürüklenmesi ve hava kaldırma kuvvetiyle referans kütle m_ref (7.1.2-14).
2. 100 g'ın beş okumasından tekrarlanabilirlik (100,0006, 100,0003, 100,0005, 100,0004, 100,0005 g): s = 0,000 114 g, ν = 4; her yükte tek okumaya uygulanır (7.1.1-5).
3. Eksantriklik: ±|ΔI_ecc,maks|·L/(2L_ecc), L_ecc = 100 g'da ΔI_ecc,maks = 0,2 mg, dikdörtgen (7.1.1-10). Sürüklenme: k_D = 1,25 ile D = k_D·U(m_c), dikdörtgen (7.1.2-10, -11).
4. Welch–Satterthwaite ile ν_eff (B3-1) ve %95,45 için t dağılımından k.
5. δm_B = −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) (7.1.2-4), ρ_a = (1,173 ± 0,014) kg/m³ ve ρ = (7950 ± 70) kg/m³; belirsizliği (7.1.2-5a) modelden çıkar: göreli yaklaşık 3·10⁻⁸.
6. u(E) = 0,000226 g, ν_eff = 62, k = 2,05, U = 0,00046 g,

**Sonuç:**

- E = (0,00130 ± 0,00046) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,04 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h1-220g-max-option2.json --lang tr`

### Laboratuvar terazisi 220 g / 0,1 mg: sıfır yükte hata (EURAMET cg-18 H1)

`cg18-h1-220g-zero` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H1

Sıfır yükte yalnızca sıfırda yuvarlama ve tekrarlanabilirlik katkı verir; ν = 4 ile tekrarlanabilirlik kapsam faktörünü belirler.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. E = I − m_ref (7.1-1): sıfırda ve yükte yuvarlamayla (her biri d/2√3, d = 0,1 mg), tekrarlanabilirlik ve eksantriklikle gösterge I; E2 ağırlıklarının konvansiyonel kütlesi, sürüklenmesi ve hava kaldırma kuvvetiyle referans kütle m_ref (7.1.2-14).
2. 100 g'ın beş okumasından tekrarlanabilirlik (100,0006, 100,0003, 100,0005, 100,0004, 100,0005 g): s = 0,000 114 g, ν = 4; her yükte tek okumaya uygulanır (7.1.1-5).
3. Eksantriklik: ±|ΔI_ecc,maks|·L/(2L_ecc), L_ecc = 100 g'da ΔI_ecc,maks = 0,2 mg, dikdörtgen (7.1.1-10). Sürüklenme: k_D = 1,25 ile D = k_D·U(m_c), dikdörtgen (7.1.2-10, -11).
4. Welch–Satterthwaite ile ν_eff (B3-1) ve %95,45 için t dağılımından k.
5. u(E) = √(0,000 114² + 0,000 029²) g = 0,000 118 g, ν_eff = 4,5, 4'e kesilir: k = 2,87, U = 0,000 34 g.

**Sonuç:**

- E = (0,00000 ± 0,00034) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,87 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h1-220g-zero.json --lang tr`

### Terazi 220 g / 0,1 mg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H1.4/A)

`cg18-h1-a-weighing-result` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H1.1/A to H1.4/A; 7.4, 7.5; Appendix G

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.

**Yaklaşım:** tartı aletinin hatalarına yaklaşım fonksiyonu (EURAMET cg-18, Ek C)

**Adımlar:**

1. Electronic balance, Max 220 g, d = 0.1 mg. Sıfırdan geçen hata fonksiyonu E_yakl = a₁R (C2.2-16), 7.1.2-5e (room temperature variation 5 K) bütçesinin p_j = 1/u²(E_j) ağırlıklarıyla uydurulmuştur (C2.2-18b, korelasyonsuz referans kütleler): a₁ = 6,71e-06, u(a₁) = 1,242e-06.
2. Aralık başına mutlak terimler: α_W² = d₀²/12 + d²/12 + s² = 1,467e-08 g² (7.4.1-6); s tekrarlanabilirlik deneyinden (0,000114 g), d₀ = d₁.
3. Bağıl terimler (kullanımda): uyumdan u(a₁) = 1,242e-06 (C2.2-16d), sıcaklık K_T·ΔT/√12 = 1,299e-06 (K_T = 1,5e-06/K, ΔT = 3 K: yerleşik ayar 3 K'de devreye girer, 7.4.3-1), kaldırma kuvveti √(1,07·10⁻⁴ + 1,33·10⁻⁶·ΔT²)·ρ₀/ρ_c = 1,636e-06 (7.4.3-4), dara deney noktalarından (q_E,maks − q_E,min)/√12 = 1,072e-06 (7.4.4-5), eksantriklik ΔI_ecc,maks/(L_ecc√3) = 1,155e-06 (7.4.4-10); β_W² = 8,389e-12.
4. U(W) = 2√(α_W² + β_W²R²) (7.4.5-2, 7.5.1-2b), aralık sınırlarındaki U(W) değerlerinden geçen doğrularla değiştirilir (7.5.2-3d); okumalar düzeltilmeden genel belirsizlik |a₁|R'yi ekler: ilk aralıkta U_gl(W) = 0,0002422 g + 1,151e-05·R (7.5.2-3e).
5. Req = %1 ve güvenlik faktörü SF = 3 için minimum tartım: R_min,SF = α_gl·SF/(Req − β_gl·SF) = 0,07292 g (G-9).

**Sonuç:**

- E_yakl(I) = 6,710 × 10⁻⁶·I; χ²_göz = 0,298, ν = 4 (C2.2-2a ✓); s_m = 0 g
- u(a1) = 1,24 × 10⁻⁶
- U_gl(W) = 0,0002422 g + 1,151 × 10⁻⁵·R
- R_min,SF = 0,0729 g (Req = %1, SF = 3)

**Notlar:**

- Tartım sonucunun belirsizliği, varsayılan kullanım koşullarında kullanıcıya yönelik öneridir; kalibrasyon sonuçlarının parçası değildir (7.4).
- H1/A hataları 7.1.2-5e bütçesiyle (oda sıcaklığı değişimi), H2/A ise en tutucu 7.1.2-5d bütçesiyle uydurur; basılı a₁ ve u(a₁) yalnızca bu seçimlerle yeniden elde edilir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h1-a-weighing-result.json --lang tr`

### Terazi 220 g / 0,1 mg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H1.4/B)

`cg18-h1-b-weighing-result` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H1.1/B to H1.4/B; 7.4, 7.5; Appendix G

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.

**Yaklaşım:** tartı aletinin hatalarına yaklaşım fonksiyonu (EURAMET cg-18, Ek C)

**Adımlar:**

1. Electronic balance, Max 220 g, d = 0.1 mg. Sıfırdan geçen hata fonksiyonu E_yakl = a₁R (C2.2-16), 7.1.2-5c (adjusted immediately before calibration) bütçesinin p_j = 1/u²(E_j) ağırlıklarıyla uydurulmuştur (C2.2-18b, korelasyonsuz referans kütleler): a₁ = -3,898e-07, u(a₁) = 6,332e-07.
2. Aralık başına mutlak terimler: α_W² = d₀²/12 + d²/12 + s² = 1,467e-08 g² (7.4.1-6); s tekrarlanabilirlik deneyinden (0,000114 g), d₀ = d₁.
3. Bağıl terimler (kullanımda): uyumdan u(a₁) = 6,332e-07 (C2.2-16d), sıcaklık K_T·ΔT/√12 = 1,299e-06 (K_T = 1,5e-06/K, ΔT = 3 K: yerleşik ayar 3 K'de devreye girer, 7.4.3-1), kaldırma kuvveti √(1,07·10⁻⁴ + 1,33·10⁻⁶·ΔT²)·ρ₀/ρ_c = 1,636e-06 (7.4.3-4), dara deney noktalarından (q_E,maks − q_E,min)/√12 = 1,155e-06 (7.4.4-5), eksantriklik ΔI_ecc,maks/(L_ecc√3) = 1,155e-06 (7.4.4-10); β_W² = 7,432e-12.
4. U(W) = 2√(α_W² + β_W²R²) (7.4.5-2, 7.5.1-2b), aralık sınırlarındaki U(W) değerlerinden geçen doğrularla değiştirilir (7.5.2-3d); okumalar düzeltilmeden genel belirsizlik |a₁|R'yi ekler: ilk aralıkta U_gl(W) = 0,0002422 g + 4,851e-06·R (7.5.2-3e).
5. Req = %1 ve güvenlik faktörü SF = 3 için minimum tartım: R_min,SF = α_gl·SF/(Req − β_gl·SF) = 0,07277 g (G-9).

**Sonuç:**

- E_yakl(I) = -3,898 × 10⁻⁷·I; χ²_göz = 0,331, ν = 4 (C2.2-2a ✓); s_m = 0 g
- u(a1) = 6,33 × 10⁻⁷
- U_gl(W) = 0,0002422 g + 4,851 × 10⁻⁶·R
- R_min,SF = 0,0728 g (Req = %1, SF = 3)

**Notlar:**

- Tartım sonucunun belirsizliği, varsayılan kullanım koşullarında kullanıcıya yönelik öneridir; kalibrasyon sonuçlarının parçası değildir (7.4).
- Kılavuz u_bağıl(δR_Dara) = 5,774·10⁻⁷ basar. Deney noktaları arasındaki eğimler 0, −2·10⁻⁶, +2·10⁻⁶ ve −1,43·10⁻⁶'dır; dolayısıyla (q_E,maks − q_E,min)/√12 = 4·10⁻⁶/√12 = 1,155e-06 (7.4.4-5); basılı değer 2·10⁻⁶ genişliğe karşılık gelir. Proje formülü kullanır: β_W² = 6,433·10⁻¹² yerine 7,432e-12, U_gl(W) eğimi 4,479·10⁻⁶ yerine 4,851e-06, minimum tartım 0,0727 g yerine 0,07277 g (araştırma günlüğü F76).
- Kılavuz a₁ = 3,895·10⁻⁷ değerini işaretsiz basar; hatalar (100 g ve 220 g'da −0,1 mg) a₁ = −3,895·10⁻⁷ verir. Genel belirsizliğe yalnızca |a₁| girer.

**Çalıştırma:** `uncertainty run library/mass/cg18-h1-b-weighing-result.json --lang tr`

### Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesi olmadan 10 kg'da hata (EURAMET cg-18 H2, seçenek 1)

`cg18-h2-60kg-10kg-option1` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H2, H2.3/A

2 g'lık aralıktaki ilk yük noktası; hava yoğunluğu bilinmiyor ve kalibrasyondan önce ayarlanmamış: temkinli kaldırma kuvveti terimi 7.1.2-5d.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Çok aralıklı terazi 12 kg / 2 g, 30 kg / 5 g, 60 kg / 10 g: sıfırda (d₀ = 2 g) ve yükte (aralığının d'si) yuvarlama, tekrarlanabilirlik, eksantriklik ve referans kütle ile E = I − m_ref (7.1-1).
2. 10 kg'da (1. aralık) ve 25 kg'da (2. ve 3. aralık) beş okumadan tekrarlanabilirlik: s = 1,095 g ve 2,739 g, ν = 4 (7.1.1-5).
3. Eksantriklik L_ecc = 20 kg'da ΔI_ecc,maks = 5 g: yarı genişlik ΔI·L/(2L_ecc), dikdörtgen (7.1.1-10).
4. Nominal değerinde kullanılan F2 ağırlıkları: u(m_c) = mpe/√3 (7.1.2-3), ağırlık kombinasyonu için mpe'ler toplanır; sürüklenme D = mpe/2, u = mpe/(2√3) (7.1.2-11, k_D = 1,5).
5. 1. aralık (d = 2 g): u(I) = √(1,095² + 0,577² + 0,577² + 0,722²) = 1,545 g; bir adet 10 kg F2 ağırlık (mpe 0,16 g): u(m_c) = 0,092, u(m_D) = 0,046, u(m_B) = 0,110 g (7.1.2-5d); u(m_ref) = 0,151 g.
6. u(E) = 1,552 g, ν_eff = 16, k = 2,17, U = 3,37 g.

**Sonuç:**

- E = (0,0 ± 3,4) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,17 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h2-60kg-10kg-option1.json --lang tr`

### Çok aralıklı terazi 60 kg, kalibrasyondan önce ayarlanmış: Max'ta hata (EURAMET cg-18 H2, ikinci durum)

`cg18-h2-60kg-max-adjusted` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H2, H2.3/B

İkinci durum: terazi kalibrasyondan hemen önce dahili ağırlıklarıyla ayarlanır; tekrarlanabilirlik daha iyidir (s = 2,236 g).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Çok aralıklı terazi 12 kg / 2 g, 30 kg / 5 g, 60 kg / 10 g: sıfırda (d₀ = 2 g) ve yükte (aralığının d'si) yuvarlama, tekrarlanabilirlik, eksantriklik ve referans kütle ile E = I − m_ref (7.1-1).
2. 10 kg'da (1. aralık) ve 25 kg'da (2. ve 3. aralık) beş okumadan tekrarlanabilirlik: s = 0,894 g ve 2,236 g, ν = 4 (7.1.1-5).
3. Eksantriklik L_ecc = 20 kg'da ΔI_ecc,maks = 5 g: yarı genişlik ΔI·L/(2L_ecc), dikdörtgen (7.1.1-10).
4. Nominal değerinde kullanılan F2 ağırlıkları: u(m_c) = mpe/√3 (7.1.2-3), ağırlık kombinasyonu için mpe'ler toplanır; sürüklenme D = mpe/2, u = mpe/(2√3) (7.1.2-11, k_D = 1,5).
5. Kalibrasyondan hemen önce dahili ağırlıklarla ayarlanmış: kaldırma kuvveti yalnızca ağırlıkların yoğunluk toleransından, u_rel = mpe/(4m_N√3) (7.1.2-5c).
6. 25 kg'da s = 2,236 g: u(I) = 5,694 g, u(m_ref) = 0,595 g, u(E) = 5,725 g, ν_eff = 172, k = 2,01, U = 11,53 g.

**Sonuç:**

- E = (0 ± 12) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,01 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bütçedeki gibi mpe = 0,96 g ile u(m_ref) = 0,635 g ve u(E) = 5,729 g olur; tabloda 5,739 g basılıdır ve kendi girdilerinden √(5,694² + 0,635²) çıkmaz (F68).
- Kılavuz k'yı GUM Tablo G.2'den bir alt tablo satırında okur (ν = 90'da ν = 50 satırından 2,05; ν = 172'de 2,025); program %95,45 için tam t-kantilini kullanır (2,03 ve 2,01).

**Çalıştırma:** `uncertainty run library/mass/cg18-h2-60kg-max-adjusted.json --lang tr`

### Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesi olmadan Max'ta hata (EURAMET cg-18 H2, seçenek 1)

`cg18-h2-60kg-max-option1` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H2, H2.3/A

10 g'lık aralıkta 60 kg'da hata; sertifikadaki en büyük U.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Çok aralıklı terazi 12 kg / 2 g, 30 kg / 5 g, 60 kg / 10 g: sıfırda (d₀ = 2 g) ve yükte (aralığının d'si) yuvarlama, tekrarlanabilirlik, eksantriklik ve referans kütle ile E = I − m_ref (7.1-1).
2. 10 kg'da (1. aralık) ve 25 kg'da (2. ve 3. aralık) beş okumadan tekrarlanabilirlik: s = 1,095 g ve 2,739 g, ν = 4 (7.1.1-5).
3. Eksantriklik L_ecc = 20 kg'da ΔI_ecc,maks = 5 g: yarı genişlik ΔI·L/(2L_ecc), dikdörtgen (7.1.1-10).
4. Nominal değerinde kullanılan F2 ağırlıkları: u(m_c) = mpe/√3 (7.1.2-3), ağırlık kombinasyonu için mpe'ler toplanır; sürüklenme D = mpe/2, u = mpe/(2√3) (7.1.2-11, k_D = 1,5).
5. 3. aralık (d = 10 g): u(I) = √(2,739² + 0,577² + 2,887² + 4,330²) = 5,909 g; deney yükü 3 × 20 kg F2 (mpe 0,90 g): u(m_ref) = 0,871 g.
6. E = 59 990 − 60 000 = −10 g; u(E) = 5,973 g, ν_eff = 90, k = 2,03, U = 12,11 g.

**Sonuç:**

- E = (-10 ± 12) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,03 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bütçe 60 kg'lık deney yükü için mpe = 0,96 g kullanır (u(m_c) = 0,554 g, u(m_D) = 0,277 g, u(m_B) = 0,658 g); oysa üç adet 20 kg F2 ağırlığın mpe'si 3 × 0,30 g = 0,90 g'dır (OIML R111). Proje 0,90 g kullanır: u(m_ref) 0,904 g yerine 0,871 g, u(E) 5,978 g yerine 5,973 g olur (araştırma günlüğü F68).
- Kılavuz k'yı GUM Tablo G.2'den bir alt tablo satırında okur (ν = 90'da ν = 50 satırından 2,05; ν = 172'de 2,025); program %95,45 için tam t-kantilini kullanır (2,03 ve 2,01).

**Çalıştırma:** `uncertainty run library/mass/cg18-h2-60kg-max-option1.json --lang tr`

### Çok aralıklı terazi 60 kg: ΔT = 10 K için kaldırma kuvveti terimiyle Max'ta hata (EURAMET cg-18 H2, seçenek 1, 7.1.2-5e formülü)

`cg18-h2-60kg-max-option1-delta-t` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H2, H2.3/A

Seçenek 1 gibi; kaldırma kuvveti belirsizliği odanın beklenen sıcaklık değişiminden (10 K, pencereli oda).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Çok aralıklı terazi 12 kg / 2 g, 30 kg / 5 g, 60 kg / 10 g: sıfırda (d₀ = 2 g) ve yükte (aralığının d'si) yuvarlama, tekrarlanabilirlik, eksantriklik ve referans kütle ile E = I − m_ref (7.1-1).
2. 10 kg'da (1. aralık) ve 25 kg'da (2. ve 3. aralık) beş okumadan tekrarlanabilirlik: s = 1,095 g ve 2,739 g, ν = 4 (7.1.1-5).
3. Eksantriklik L_ecc = 20 kg'da ΔI_ecc,maks = 5 g: yarı genişlik ΔI·L/(2L_ecc), dikdörtgen (7.1.1-10).
4. Nominal değerinde kullanılan F2 ağırlıkları: u(m_c) = mpe/√3 (7.1.2-3), ağırlık kombinasyonu için mpe'ler toplanır; sürüklenme D = mpe/2, u = mpe/(2√3) (7.1.2-11, k_D = 1,5).
5. 10 K'lik oda sıcaklığı değişiminden kaldırma kuvveti: u_rel = √(1,07·10⁻⁴ + 1,33·10⁻⁶·100)·ρ₀/ρ_c + mpe/(4m_N√3) = 4,49·10⁻⁶, u = 0,269 g; u(m_ref) = 0,640 g.
6. u(E) = 5,944 g, k = 2,03, U = 12,06 g; kaldırma kuvveti terimi göstergenin yanında neredeyse önemsizdir.

**Sonuç:**

- E = (-10 ± 12) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,03 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bütçe 60 kg'lık deney yükü için mpe = 0,96 g kullanır (u(m_c) = 0,554 g, u(m_D) = 0,277 g, u(m_B) = 0,658 g); oysa üç adet 20 kg F2 ağırlığın mpe'si 3 × 0,30 g = 0,90 g'dır (OIML R111). Proje 0,90 g kullanır: u(m_ref) 0,904 g yerine 0,871 g, u(E) 5,978 g yerine 5,973 g olur (araştırma günlüğü F68).
- Kılavuz k'yı GUM Tablo G.2'den bir alt tablo satırında okur (ν = 90'da ν = 50 satırından 2,05; ν = 172'de 2,025); program %95,45 için tam t-kantilini kullanır (2,03 ve 2,01).
- mpe = 0,96 g ile 7.1.2-5e formülü 60 kg'da 0,278 g ve 40 kg'da 0,180 g verir; tabloda 0,276 g ve 0,178 g basılıdır.

**Çalıştırma:** `uncertainty run library/mass/cg18-h2-60kg-max-option1-delta-t.json --lang tr`

### Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesiyle Max'ta hata (EURAMET cg-18 H2, seçenek 2)

`cg18-h2-60kg-max-option2` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H2, H2.3/A

Kalibrasyon sırasındaki hava yoğunluğu CIPM formülünden hesaplanır ve kaldırma kuvveti düzeltmesi uygulanır; ayar sırasındaki hava yoğunluğu bilinmez.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Çok aralıklı terazi 12 kg / 2 g, 30 kg / 5 g, 60 kg / 10 g: sıfırda (d₀ = 2 g) ve yükte (aralığının d'si) yuvarlama, tekrarlanabilirlik, eksantriklik ve referans kütle ile E = I − m_ref (7.1-1).
2. 10 kg'da (1. aralık) ve 25 kg'da (2. ve 3. aralık) beş okumadan tekrarlanabilirlik: s = 1,095 g ve 2,739 g, ν = 4 (7.1.1-5).
3. Eksantriklik L_ecc = 20 kg'da ΔI_ecc,maks = 5 g: yarı genişlik ΔI·L/(2L_ecc), dikdörtgen (7.1.1-10).
4. Nominal değerinde kullanılan F2 ağırlıkları: u(m_c) = mpe/√3 (7.1.2-3), ağırlık kombinasyonu için mpe'ler toplanır; sürüklenme D = mpe/2, u = mpe/(2√3) (7.1.2-11, k_D = 1,5).
5. Hava yoğunluğu 1,173 kg/m³ (990 hPa, %50 bağıl nem, 21 °C), 10 K'lik sıcaklık değişimi için u = 0,018 kg/m³ (A3-2); ağırlıklar ρ = (7950 ± 70) kg/m³.
6. Düzeltme −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) = +1,3 mg ve belirsizliği (yaklaşık 2 mg), d = 10 g'ın yanında ihmal edilebilir: u(E) = 5,938 g, U = 12,05 g.

**Sonuç:**

- E = (-10 ± 12) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,03 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bütçe 60 kg'lık deney yükü için mpe = 0,96 g kullanır (u(m_c) = 0,554 g, u(m_D) = 0,277 g, u(m_B) = 0,658 g); oysa üç adet 20 kg F2 ağırlığın mpe'si 3 × 0,30 g = 0,90 g'dır (OIML R111). Proje 0,90 g kullanır: u(m_ref) 0,904 g yerine 0,871 g, u(E) 5,978 g yerine 5,973 g olur (araştırma günlüğü F68).
- Kılavuz k'yı GUM Tablo G.2'den bir alt tablo satırında okur (ν = 90'da ν = 50 satırından 2,05; ν = 172'de 2,025); program %95,45 için tam t-kantilini kullanır (2,03 ve 2,01).

**Çalıştırma:** `uncertainty run library/mass/cg18-h2-60kg-max-option2.json --lang tr`

### Çok aralıklı terazi 60 kg: sıfır yükte hata (EURAMET cg-18 H2)

`cg18-h2-60kg-zero` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H2, H2.3/A

Bulunduğu hâliyle, kalibrasyondan önce ayarlanmadan kalibre edilen 60 kg'lık çok aralıklı terazi (2 g / 5 g / 10 g).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Çok aralıklı terazi 12 kg / 2 g, 30 kg / 5 g, 60 kg / 10 g: sıfırda (d₀ = 2 g) ve yükte (aralığının d'si) yuvarlama, tekrarlanabilirlik, eksantriklik ve referans kütle ile E = I − m_ref (7.1-1).
2. 10 kg'da (1. aralık) ve 25 kg'da (2. ve 3. aralık) beş okumadan tekrarlanabilirlik: s = 1,095 g ve 2,739 g, ν = 4 (7.1.1-5).
3. Eksantriklik L_ecc = 20 kg'da ΔI_ecc,maks = 5 g: yarı genişlik ΔI·L/(2L_ecc), dikdörtgen (7.1.1-10).
4. Nominal değerinde kullanılan F2 ağırlıkları: u(m_c) = mpe/√3 (7.1.2-3), ağırlık kombinasyonu için mpe'ler toplanır; sürüklenme D = mpe/2, u = mpe/(2√3) (7.1.2-11, k_D = 1,5).
5. Sıfırda yalnızca d₀ ve 1. aralığın tekrarlanabilirliği katkı verir: u(E) = √(1,095² + 0,577²) = 1,238 g, ν_eff = 6,5, 6'ya kesilir: k = 2,52, U = 3,12 g.

**Sonuç:**

- E = (0,0 ± 3,1) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,52 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h2-60kg-zero.json --lang tr`

### Çok aralıklı tartı aleti 12/30/60 kg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H2.4/A)

`cg18-h2-a-weighing-result` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H2.1/A to H2.4/A; 7.4, 7.5; Appendix G

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.

**Yaklaşım:** tartı aletinin hatalarına yaklaşım fonksiyonu (EURAMET cg-18, Ek C)

**Adımlar:**

1. Multi-interval scale, Max 12/30/60 kg, d = 2/5/10 g. Sıfırdan geçen hata fonksiyonu E_yakl = a₁R (C2.2-16), 7.1.2-5d (no information on the air density) bütçesinin p_j = 1/u²(E_j) ağırlıklarıyla uydurulmuştur (C2.2-18b, korelasyonsuz referans kütleler): a₁ = -0,0001717, u(a₁) = 6,459e-05.
2. Aralık başına mutlak terimler: α_W² = d₀²/12 + d²/12 + s² = 1,867 g², 9,917 g², 16,17 g² (7.4.1-6); s tekrarlanabilirlik deneylerinden (1,095, 2,739, 2,739 g), d₀ = d₁.
3. Bağıl terimler (kullanımda): uyumdan u(a₁) = 6,459e-05 (C2.2-16d), sıcaklık K_T·ΔT/√12 = 1,732e-06 (K_T = 2e-06/K, ΔT = 3 K: yerleşik ayar 3 K'de devreye girer, 7.4.3-1), kaldırma kuvveti ilgili değil, dara deney noktalarından (q_E,maks − q_E,min)/√12 = 0,0001444 (7.4.4-5), eksantriklik ΔI_ecc,maks/(L_ecc√3) = 0,0001443 (7.4.4-10); β_W² = 4,586e-08.
4. U(W) = 2√(α_W² + β_W²R²) (7.4.5-2, 7.5.1-2b), aralık sınırlarındaki U(W) değerlerinden geçen doğrularla değiştirilir (7.5.2-3d, aralık başına -3f); okumalar düzeltilmeden genel belirsizlik |a₁|R'yi ekler: ilk aralıkta U_gl(W) = 2,733 g + 0,0004291·R (7.5.2-3e).
5. Req = %1 ve güvenlik faktörü SF = 2 için minimum tartım: R_min,SF = α_gl·SF/(Req − β_gl·SF) = 597,8 g (G-9).

**Sonuç:**

- E_yakl(I) = -0,0001717·I; χ²_göz = 1,82, ν = 4 (C2.2-2a ✓); s_m = 0 g
- u(a1) = 6,46 × 10⁻⁵
- U_gl(W) = 2,733 g + 0,0004291·R
- R_min,SF = 598 g (Req = %1, SF = 2)

**Notlar:**

- Tartım sonucunun belirsizliği, varsayılan kullanım koşullarında kullanıcıya yönelik öneridir; kalibrasyon sonuçlarının parçası değildir (7.4).
- Kılavuz ikinci ve üçüncü aralığın birinci mertebe U(W)'sini 10,190 g + 3,434·10⁻⁴(R − 12 000 g) ve 20,311 g + 3,923·10⁻⁴(R − 30 000 g) olarak basar. 10,190 g ve 20,311 g, 12 000 g ve 30 000 g'daki genel belirsizliklerdir, U(W) + |a₁|R; U(W)'nin kendisi orada 8,129 g ve 15,16 g'dır. Eğimler ve genel belirsizlik doğruları doğrudur (araştırma günlüğü F75).
- Kılavuz a₁ = 1,717·10⁻⁴ değerini işaretsiz basar; hatalar negatiftir (20 kg'da −5 g, 40 ve 60 kg'da −10 g).
- H1/A hataları 7.1.2-5e bütçesiyle (oda sıcaklığı değişimi), H2/A ise en tutucu 7.1.2-5d bütçesiyle uydurur; basılı a₁ ve u(a₁) yalnızca bu seçimlerle yeniden elde edilir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h2-a-weighing-result.json --lang tr`

### Çok aralıklı tartı aleti 12/30/60 kg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H2.4/B)

`cg18-h2-b-weighing-result` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H2.1/B to H2.4/B; 7.4, 7.5; Appendix G

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.

**Yaklaşım:** tartı aletinin hatalarına yaklaşım fonksiyonu (EURAMET cg-18, Ek C)

**Adımlar:**

1. Multi-interval scale, Max 12/30/60 kg, d = 2/5/10 g. Sıfırdan geçen hata fonksiyonu E_yakl = a₁R (C2.2-16), 7.1.2-5c (adjusted immediately before calibration) bütçesinin p_j = 1/u²(E_j) ağırlıklarıyla uydurulmuştur (C2.2-18b, korelasyonsuz referans kütleler): a₁ = 0, u(a₁) = 6,043e-05.
2. Aralık başına mutlak terimler: α_W² = d₀²/12 + d²/12 + s² = 1,467 g², 7,417 g², 13,67 g² (7.4.1-6); s tekrarlanabilirlik deneylerinden (0,8944, 2,236, 2,236 g), d₀ = d₁.
3. Bağıl terimler (kullanımda): uyumdan u(a₁) = 6,043e-05 (C2.2-16d), sıcaklık K_T·ΔT/√12 = 1,732e-06 (K_T = 2e-06/K, ΔT = 3 K: yerleşik ayar 3 K'de devreye girer, 7.4.3-1), kaldırma kuvveti ilgili değil, dara deney noktalarından (q_E,maks − q_E,min)/√12 = 0 (7.4.4-5), eksantriklik ΔI_ecc,maks/(L_ecc√3) = 0,0001443 (7.4.4-10); β_W² = 2,449e-08.
4. U(W) = 2√(α_W² + β_W²R²) (7.4.5-2, 7.5.1-2b), aralık sınırlarındaki U(W) değerlerinden geçen doğrularla değiştirilir (7.5.2-3d, aralık başına -3f); okumalar düzeltilmeden genel belirsizlik |a₁|R'yi ekler: ilk aralıkta U_gl(W) = 2,422 g + 0,0001706·R (7.5.2-3e).
5. Req = %1 ve güvenlik faktörü SF = 2 için minimum tartım: R_min,SF = α_gl·SF/(Req − β_gl·SF) = 501,5 g (G-9).

**Sonuç:**

- E_yakl(I) = 0·I; χ²_göz = 0, ν = 4 (C2.2-2a ✓); s_m = 0 g
- u(a1) = 6,04 × 10⁻⁵
- U_gl(W) = 2,422 g + 0,0001706·R
- R_min,SF = 502 g (Req = %1, SF = 2)

**Notlar:**

- Tartım sonucunun belirsizliği, varsayılan kullanım koşullarında kullanıcıya yönelik öneridir; kalibrasyon sonuçlarının parçası değildir (7.4).

**Çalıştırma:** `uncertainty run library/mass/cg18-h2-b-weighing-result.json --lang tr`

### Köprü kantar 30 t: 10 t standart ağırlıkta hata (EURAMET cg-18 H3)

`cg18-h3-30t-10t-weights` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H3, H3.2/A and H3.3/A

Yük alıcısında 10 × 1000 kg standart ağırlık; bu yük m_ref, ikame yükleri için de referanstır.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Köprü kantar 30 000 kg / 10 kg, servis modunda d_T = 1 kg ile denenmiştir: E = I − L_T (7.1-1). Her gösterge sıfırda ve yükte yuvarlama (d_T/√12), tekrarlanabilirlik, eksantriklik ve sürünme/histerezis taşır.
2. Yaklaşık 10 420 kg'lık altı yüklemeden tekrarlanabilirlik (10 405 … 10 425 kg): s = 6,74 kg, ν = 5 (7.1.1-5). Eksantriklik 10 420 kg'da 15 kg, iki denemenin bağıl olarak büyüğü: yarı genişlik ΔI·L/(2L_ecc) (7.1.1-10). Sıfıra dönüşteki E₀ = 4 kg'dan sürünme: yarı genişlik E₀·L/Max (7.4.4-7).
3. Standart ağırlıklar 10 × 1000 kg, M1 (her biri mpe 50 g), nominal değerinde: u(m_c) = mpe/√3 ve sürüklenme D = mpe (7.1.2-3, -11), kullanılan ağırlıklar için doğrusal toplanır; kaldırma kuvveti 7.1.2-5d, u_rel = 1,6·10⁻⁵, ikame yükleri için de (ρ ≥ 4400 kg/m³).
4. u(I) = √(6,74² + 0,29² + 0,29² + 4,16² + 0,77²) = 7,97 kg; u(m_ref) = √(0,29² + 0,29² + 0,16²) = 0,44 kg; u(E) = 7,98 kg, ν_eff = 9,8 → 9, k = 2,32, U = 18,5 kg.

**Sonuç:**

- E = (10 ± 18) kg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,32 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h3-30t-10t-weights.json --lang tr`

### Köprü kantar 30 t: ikame yüküyle 15 t'da hata (EURAMET cg-18 H3)

`cg18-h3-30t-15t-substitution` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H3, H3.2/A and H3.3/A

İkame yükü L_ikame1 (10 t standart ağırlıkla aynı göstergeyi verecek şekilde ayarlanmış römork ve konteynerler) üzerinde 5 t standart ağırlık.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Köprü kantar 30 000 kg / 10 kg, servis modunda d_T = 1 kg ile denenmiştir: E = I − L_T (7.1-1). Her gösterge sıfırda ve yükte yuvarlama (d_T/√12), tekrarlanabilirlik, eksantriklik ve sürünme/histerezis taşır.
2. Yaklaşık 10 420 kg'lık altı yüklemeden tekrarlanabilirlik (10 405 … 10 425 kg): s = 6,74 kg, ν = 5 (7.1.1-5). Eksantriklik 10 420 kg'da 15 kg, iki denemenin bağıl olarak büyüğü: yarı genişlik ΔI·L/(2L_ecc) (7.1.1-10). Sıfıra dönüşteki E₀ = 4 kg'dan sürünme: yarı genişlik E₀·L/Max (7.4.4-7).
3. Standart ağırlıklar 10 × 1000 kg, M1 (her biri mpe 50 g), nominal değerinde: u(m_c) = mpe/√3 ve sürüklenme D = mpe (7.1.2-3, -11), kullanılan ağırlıklar için doğrusal toplanır; kaldırma kuvveti 7.1.2-5d, u_rel = 1,6·10⁻⁵, ikame yükleri için de (ρ ≥ 4400 kg/m³).
4. İkame (4.3.3, 7.1.2-15): 10 t'da L_ikame1 = m_ref + I(L_ikame1) − I(m_ref) ve 20 t'da I₂ = I(L_ikame1 + L_ikame2) − I(m_ref + L_ikame1) ile L_ikame1 + L_ikame2 = 2m_ref + I₁ + I₂. Ayar için kullanılan her gösterge kendi yuvarlamasını, tekrarlanabilirliğini, eksantrikliğini ve sürünmesini getirir; standart ağırlıklar her adımda aynıdır.
5. u(L_ikame1)² = u²(m_ref) + 2u²(10 t'da I) = 0,44² + 2·7,97²: u(L_ikame1) = 11,28 kg. 15 t'da u(I) = 9,27 kg ve 5 t standart ağırlıkla: u(E) = 14,60 kg.
6. Üç tekrarlanabilirlik teriminin her biri için ν = 5 ile ν_eff = 37 ve k = 2,07: U = 29 kg yerine 30,3 kg.

**Sonuç:**

- E = (15 ± 30) kg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,07 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Kılavuzun ν_eff'i yalnızca yükteki göstergenin tekrarlanabilirliğini sayar; ikame yüklerini ayarlamada kullanılan göstergelerin tekrarlanabilirlik terimlerine (7.1.2-15) sonsuz serbestlik derecesi verilir. Program her birine ν = 5 verir (araştırma günlüğü F69).
- Üç tekrarlanabilirlik terimini aynı s'nin tek bir tahmini (ν = 5) olarak saymak, Welch–Satterthwaite'in aynı s üç kez kullanıldığında gerektirdiği gibi, ν_eff = 12 ve k = 2,23 verir (U = 32,6 kg).

**Çalıştırma:** `uncertainty run library/mass/cg18-h3-30t-15t-substitution.json --lang tr`

### Köprü kantar 30 t: iki ikame yüküyle Max'ta hata (EURAMET cg-18 H3)

`cg18-h3-30t-max-substitution` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H3, H3.2/A and H3.3/A

İkame yükleri L_ikame1 + L_ikame2 üzerinde 10 t standart ağırlık; sertifikadaki en büyük U.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Köprü kantar 30 000 kg / 10 kg, servis modunda d_T = 1 kg ile denenmiştir: E = I − L_T (7.1-1). Her gösterge sıfırda ve yükte yuvarlama (d_T/√12), tekrarlanabilirlik, eksantriklik ve sürünme/histerezis taşır.
2. Yaklaşık 10 420 kg'lık altı yüklemeden tekrarlanabilirlik (10 405 … 10 425 kg): s = 6,74 kg, ν = 5 (7.1.1-5). Eksantriklik 10 420 kg'da 15 kg, iki denemenin bağıl olarak büyüğü: yarı genişlik ΔI·L/(2L_ecc) (7.1.1-10). Sıfıra dönüşteki E₀ = 4 kg'dan sürünme: yarı genişlik E₀·L/Max (7.4.4-7).
3. Standart ağırlıklar 10 × 1000 kg, M1 (her biri mpe 50 g), nominal değerinde: u(m_c) = mpe/√3 ve sürüklenme D = mpe (7.1.2-3, -11), kullanılan ağırlıklar için doğrusal toplanır; kaldırma kuvveti 7.1.2-5d, u_rel = 1,6·10⁻⁵, ikame yükleri için de (ρ ≥ 4400 kg/m³).
4. İkame (4.3.3, 7.1.2-15): 10 t'da L_ikame1 = m_ref + I(L_ikame1) − I(m_ref) ve 20 t'da I₂ = I(L_ikame1 + L_ikame2) − I(m_ref + L_ikame1) ile L_ikame1 + L_ikame2 = 2m_ref + I₁ + I₂. Ayar için kullanılan her gösterge kendi yuvarlamasını, tekrarlanabilirliğini, eksantrikliğini ve sürünmesini getirir; standart ağırlıklar her adımda aynıdır.
5. u(L_ikame1 + L_ikame2)² = 4u²(m_ref) + 2u²(10 t'da I) + 2u²(20 t'da I) = 4·0,44² + 2·7,97² + 2·10,82²: 19,02 kg; 30 t'da u(I) = 14,38 kg ile: u(E) = 23,85 kg.
6. Aynı standart ağırlıklar üç kez girer (m_ref ve ikame yüklerinin içinde iki kez); model hatalarını doğrusal, kılavuz karesel olarak toplar (7.1.3-1c): 0,98 kg yerine 1,32 kg; u(E)'ye etkisi yoktur.
7. E = 30 040 − 30 010 = 30 kg; u(E) = 23,86 kg, ν_eff = 157, k = 2,02, U = 48,1 kg.

**Sonuç:**

- E = (30 ± 48) kg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,02 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Kılavuzun ν_eff'i yalnızca yükteki göstergenin tekrarlanabilirliğini sayar; ikame yüklerini ayarlamada kullanılan göstergelerin tekrarlanabilirlik terimlerine (7.1.2-15) sonsuz serbestlik derecesi verilir. Program her birine ν = 5 verir (araştırma günlüğü F69).
- Bütçe her yükte s = 6,74 kg kullanır; oysa 24 160 kg'daki tekrarlanabilirlik deneyi s = 9,03 kg vermiştir. 20 t ve 30 t'daki üç gösterge için 9,03 kg ile 30 t'da u(E) 23,9 kg yerine 26,0 kg olurdu (F69).

**Çalıştırma:** `uncertainty run library/mass/cg18-h3-30t-max-substitution.json --lang tr`

### Köprü kantar 30 t: sıfır yükte hata (EURAMET cg-18 H3)

`cg18-h3-30t-zero` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H3, H3.2/A and H3.3/A

Açık havada, bulunduğu hâliyle kalibre edilen 30 000 kg / 10 kg'lık karayolu taşıtı köprü kantarı.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Köprü kantar 30 000 kg / 10 kg, servis modunda d_T = 1 kg ile denenmiştir: E = I − L_T (7.1-1). Her gösterge sıfırda ve yükte yuvarlama (d_T/√12), tekrarlanabilirlik, eksantriklik ve sürünme/histerezis taşır.
2. Yaklaşık 10 420 kg'lık altı yüklemeden tekrarlanabilirlik (10 405 … 10 425 kg): s = 6,74 kg, ν = 5 (7.1.1-5). Eksantriklik 10 420 kg'da 15 kg, iki denemenin bağıl olarak büyüğü: yarı genişlik ΔI·L/(2L_ecc) (7.1.1-10). Sıfıra dönüşteki E₀ = 4 kg'dan sürünme: yarı genişlik E₀·L/Max (7.4.4-7).
3. Standart ağırlıklar 10 × 1000 kg, M1 (her biri mpe 50 g), nominal değerinde: u(m_c) = mpe/√3 ve sürüklenme D = mpe (7.1.2-3, -11), kullanılan ağırlıklar için doğrusal toplanır; kaldırma kuvveti 7.1.2-5d, u_rel = 1,6·10⁻⁵, ikame yükleri için de (ρ ≥ 4400 kg/m³).
4. Sıfırda: u(E) = √(6,74² + 0,29²) = 6,75 kg, ν_eff = 5, k = 2,65, U = 17,9 kg.

**Sonuç:**

- E = (0 ± 18) kg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,65 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/cg18-h3-30t-zero.json --lang tr`

### Terazi 400 g / 0,1 mg: hata yaklaşım fonksiyonu E = a₁I, ilk uyum (EURAMET cg-18 H4)

`cg18-h4-400g-error-function` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H4; Appendix C2.2

Korelasyonlu referans kütlelerle dokuz deney noktasından 400 g'lık bir terazinin hata yaklaşım fonksiyonu E = a₁I.

**Yaklaşım:** tartı aletinin hatalarına yaklaşım fonksiyonu (EURAMET cg-18, Ek C)

**Adımlar:**

1. Terazi 400 g / 0,1 mg, kalibrasyondan hemen önce ayarlanmış; E2 ağırlıklarla (50 g, 100 g, 2 × 200 g) 0 … 400 g arasında dokuz deney noktası, N = 3 çevrim: E_j = I_j − m_c,j.
2. u(I_j), tekrarlanabilirlik s = 0,052 mg/√3 = 0,030 mg (7.1.1-6), sıfırda ve yükte yuvarlama d/√12 ve 200 g'da 0,10 mg eksantriklikten (7.1.1-10): 0,042 … 0,077 mg.
3. u(m_ref,j), U(m_c)/2 ve ağırlıkların sürüklenmesinden (kullanılan ağırlıklar üzerinden toplanarak) ve burada negatif olan 7.1.2-5b kaldırma kuvveti teriminden (ρ_a = 1,090, ρ_a1 = 1,045 kg/m³, ρ = (8000 ± 60) kg/m³): 0,014 … 0,077 mg.
4. U(e) = s sᵀ + diag(u²(I_j)) + s_m²·I (C2.2-3a, -3b, -3c); genelleştirilmiş en küçük karelerle E(I) = a₁I: â₁ = (XᵀPX)⁻¹XᵀPe, U(â) = (XᵀPX)⁻¹, χ²_obs = vᵀPv, ν = 9 − 1 = 8 (C2.2-6 … -9).
5. Model terimi olmadan: a₁ = 0,000 83 mg/g, U(â) = 5,109·10⁻⁸ (mg/g)², u(a₁) = 0,000 23 mg/g; χ²_obs = 12,5 > ν = 8: χ² testi C2.2-2a başarısız.
6. Program en küçük model terimlerini de bulur: C2.2-2a için s_m = 0,044 mg ve C2.2-2b için 0,22 mg; kılavuz 0,05 ve 0,25 mg yuvarlak değerlerini kullanır.

**Sonuç:**

- E_yakl(I) = 0,0008340·I; χ²_göz = 12,5, ν = 8 (C2.2-2a ✗); s_m = 0 mg
- u(a1) = 0,000226

**Notlar:**

- Tablo H4.1 100, 150 ve 250 g'da e = 0,213, 0,274 ve 0,181 mg, Tablo H4.6/H4.8 ise 100,000 200, 150,000 267 ve 250,000 200 g göstergelerini verir; bütçe 0,113, 0,240 ve 0,081 mg (100,000 100, 150,000 233, 250,000 100 g) verir. Basılı a₁, U(â), χ²_obs ve artıklar bütçe değerlerinden çıkar; proje bunları kullanır (araştırma günlüğü F70).
- 7.1.2-5b'nin kaldırma kuvveti 'varyansı' negatiftir: ρ_a1'de kalibre edilmiş ağırlıkların sertifika değerinde zaten bulunan u(ρ) payını çıkarır. Bu bir varyans değil kovaryans düzeltmesidir; u²(m_ref) pozitif kalır.

**Çalıştırma:** `uncertainty run library/mass/cg18-h4-400g-error-function.json --lang tr`

### Terazi 400 g / 0,1 mg: s_m = 0,05 mg ile hata yaklaşım fonksiyonu, χ² testi geçer (EURAMET cg-18 H4)

`cg18-h4-400g-error-function-sm-0.05` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H4; Appendix C2.2

Korelasyonlu referans kütlelerle dokuz deney noktasından 400 g'lık bir terazinin hata yaklaşım fonksiyonu E = a₁I.

**Yaklaşım:** tartı aletinin hatalarına yaklaşım fonksiyonu (EURAMET cg-18, Ek C)

**Adımlar:**

1. Terazi 400 g / 0,1 mg, kalibrasyondan hemen önce ayarlanmış; E2 ağırlıklarla (50 g, 100 g, 2 × 200 g) 0 … 400 g arasında dokuz deney noktası, N = 3 çevrim: E_j = I_j − m_c,j.
2. u(I_j), tekrarlanabilirlik s = 0,052 mg/√3 = 0,030 mg (7.1.1-6), sıfırda ve yükte yuvarlama d/√12 ve 200 g'da 0,10 mg eksantriklikten (7.1.1-10): 0,042 … 0,077 mg.
3. u(m_ref,j), U(m_c)/2 ve ağırlıkların sürüklenmesinden (kullanılan ağırlıklar üzerinden toplanarak) ve burada negatif olan 7.1.2-5b kaldırma kuvveti teriminden (ρ_a = 1,090, ρ_a1 = 1,045 kg/m³, ρ = (8000 ± 60) kg/m³): 0,014 … 0,077 mg.
4. U(e) = s sᵀ + diag(u²(I_j)) + s_m²·I (C2.2-3a, -3b, -3c); genelleştirilmiş en küçük karelerle E(I) = a₁I: â₁ = (XᵀPX)⁻¹XᵀPe, U(â) = (XᵀPX)⁻¹, χ²_obs = vᵀPv, ν = 9 − 1 = 8 (C2.2-6 … -9).
5. U(e)'nin köşegeninde s_m = 0,05 mg ile: a₁ = 0,000 84 mg/g, U(â) = 5,637·10⁻⁸ (mg/g)², u(a₁) = 0,000 24 mg/g; χ²_obs = 7,3 ≤ 8 (C2.2-2a geçer).
6. u(E_yakl,j) = I_j·u(a₁) (C2.2-11): 50 g'da 0,012 mg … 400 g'da 0,095 mg. 150 g'daki (−0,114 mg) ve 250 g'daki (+0,129 mg) artıklar U(E_yakl) = 2u'yu aşar: daha katı C2.2-2b testi iki noktada başarısızdır (Tablo H4.6).

**Sonuç:**

- E_yakl(I) = 0,0008384·I; χ²_göz = 7,32, ν = 8 (C2.2-2a ✓); s_m = 0,05 mg
- u(a1) = 0,000237

**Notlar:**

- Tablo H4.1 100, 150 ve 250 g'da e = 0,213, 0,274 ve 0,181 mg, Tablo H4.6/H4.8 ise 100,000 200, 150,000 267 ve 250,000 200 g göstergelerini verir; bütçe 0,113, 0,240 ve 0,081 mg (100,000 100, 150,000 233, 250,000 100 g) verir. Basılı a₁, U(â), χ²_obs ve artıklar bütçe değerlerinden çıkar; proje bunları kullanır (araştırma günlüğü F70).
- 7.1.2-5b'nin kaldırma kuvveti 'varyansı' negatiftir: ρ_a1'de kalibre edilmiş ağırlıkların sertifika değerinde zaten bulunan u(ρ) payını çıkarır. Bu bir varyans değil kovaryans düzeltmesidir; u²(m_ref) pozitif kalır.

**Çalıştırma:** `uncertainty run library/mass/cg18-h4-400g-error-function-sm-0.05.json --lang tr`

### Terazi 400 g / 0,1 mg: s_m = 0,25 mg ile hata yaklaşım fonksiyonu, artık testi geçer (EURAMET cg-18 H4)

`cg18-h4-400g-error-function-sm-0.25` — EURAMET Calibration Guide No. 18 v4.0 (2015), Appendix H4; Appendix C2.2

Korelasyonlu referans kütlelerle dokuz deney noktasından 400 g'lık bir terazinin hata yaklaşım fonksiyonu E = a₁I.

**Yaklaşım:** tartı aletinin hatalarına yaklaşım fonksiyonu (EURAMET cg-18, Ek C)

**Adımlar:**

1. Terazi 400 g / 0,1 mg, kalibrasyondan hemen önce ayarlanmış; E2 ağırlıklarla (50 g, 100 g, 2 × 200 g) 0 … 400 g arasında dokuz deney noktası, N = 3 çevrim: E_j = I_j − m_c,j.
2. u(I_j), tekrarlanabilirlik s = 0,052 mg/√3 = 0,030 mg (7.1.1-6), sıfırda ve yükte yuvarlama d/√12 ve 200 g'da 0,10 mg eksantriklikten (7.1.1-10): 0,042 … 0,077 mg.
3. u(m_ref,j), U(m_c)/2 ve ağırlıkların sürüklenmesinden (kullanılan ağırlıklar üzerinden toplanarak) ve burada negatif olan 7.1.2-5b kaldırma kuvveti teriminden (ρ_a = 1,090, ρ_a1 = 1,045 kg/m³, ρ = (8000 ± 60) kg/m³): 0,014 … 0,077 mg.
4. U(e) = s sᵀ + diag(u²(I_j)) + s_m²·I (C2.2-3a, -3b, -3c); genelleştirilmiş en küçük karelerle E(I) = a₁I: â₁ = (XᵀPX)⁻¹XᵀPe, U(â) = (XᵀPX)⁻¹, χ²_obs = vᵀPv, ν = 9 − 1 = 8 (C2.2-6 … -9).
5. C2.2-2b'yi her noktada sağlamak için s_m 0,25 mg'a çıkarılır: a₁ = 0,000 84 mg/g, U(â) = 1,745·10⁻⁷ (mg/g)², u(a₁) = 0,000 42 mg/g; tüm artıklar U(E_yakl) içindedir (Tablo H4.8).
6. Bedeli, yaklaşık hatanın çok daha büyük belirsizliğidir: 400 g'da U(E_yakl) 0,190 mg yerine 0,334 mg.

**Sonuç:**

- E_yakl(I) = 0,0008407·I; χ²_göz = 0,679, ν = 8 (C2.2-2a ✓); s_m = 0,25 mg
- u(a1) = 0,000418

**Notlar:**

- Tablo H4.1 100, 150 ve 250 g'da e = 0,213, 0,274 ve 0,181 mg, Tablo H4.6/H4.8 ise 100,000 200, 150,000 267 ve 250,000 200 g göstergelerini verir; bütçe 0,113, 0,240 ve 0,081 mg (100,000 100, 150,000 233, 250,000 100 g) verir. Basılı a₁, U(â), χ²_obs ve artıklar bütçe değerlerinden çıkar; proje bunları kullanır (araştırma günlüğü F70).
- 7.1.2-5b'nin kaldırma kuvveti 'varyansı' negatiftir: ρ_a1'de kalibre edilmiş ağırlıkların sertifika değerinde zaten bulunan u(ρ) payını çıkarır. Bu bir varyans değil kovaryans düzeltmesidir; u²(m_ref) pozitif kalır.
- Tablo H4.8, bütçe değerlerinden hesaplanmış artıkların yanında Tablo H4.1'in e değerlerini (0,213, 0,274, 0,181 mg) basar.

**Çalıştırma:** `uncertainty run library/mass/cg18-h4-400g-error-function-sm-0.25.json --lang tr`

### 10 kg'lık bir ağırlığın kalibrasyonu

`ea402-s2-weight-10kg` — EA-4/02 M:2022, S2

EA-4/02 S2: OIML M1 sınıfı ağırlığın F2 standarda karşı kütle komparatörüyle (ABBA) kalibrasyonu.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. m_X = m_S + δm_D + δm + δm_C + δB (10 kg'dan sapmalar, mg).
2. Gözlenen fark üç ABBA karşılaştırmasının ortalamasıdır (+10, +30, +20 mg); standart belirsizliği daha önceki bir değerlendirmeden birleştirilmiş standart sapma s_p = 25 mg ile hesaplanır: u(δm) = 25/√3 = 14,4 mg.
3. Sürüklenme ±15 mg, komparatör ±10 mg ve hava kaldırma kuvveti ±10 mg (anma değerinin 1 × 10⁻⁶'sı) dikdörtgen dağılımlıdır.
4. m_X = 10 000,025 kg ± 58 mg (k = 2).

**Sonuç:**

- m_X = (25 ± 58) mg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu: rehber üç anlamlı basamağa yuvarlanmış katkıları (14,43 yerine 14,4 mg) birleştirerek 29,2 mg ve U = 58 mg bulur; yuvarlanmamış değerlerle u = 29,26 mg ve U = 58,5 mg'dir (iki basamakta 59 mg).

**Çalıştırma:** `uncertainty run library/mass/ea402-s2-weight-10kg.json --lang tr`

### Kütle kalibrasyonu (birinci mertebe GUM)

`jcgm101-9.3-mass-calibration` — JCGM 101:2008, 9.3, Tables 6 and 7; JCGM GUM-5:2026, Example 7, Table 7.2

JCGM 101 9.3: 100 g'lık bir ağırlığın referans ağırlıkla karşılaştırılarak kalibrasyonu; birinci mertebe GUM ile MCM karşılaştırması.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. W ağırlığının konvansiyonel kütlesi, havada R ile karşılaştırmalı tartımdan elde edilir: m_W = (m_R,c + δm_R,c)[1 + (ρ_a − ρ_a0)(1/ρ_W − 1/ρ_R)]; sonuç δm = m_W − 100 g sapması olarak (mg) verilir.
2. Tahmin değerlerinde ρ_a = ρ_a0 = 1,2 kg/m³ olduğundan üç yoğunluğun duyarlılık katsayıları sıfırdır: birinci mertebe değerlendirme kaldırma kuvveti düzeltmesini tamamen yok sayar (u = 0,0539 mg).
3. Monte Carlo yöntemi dağılımların tamamını yayar ve u = 0,0755 mg verir: birinci mertebe GUM sonucu tek anlamlı basamak için bile (n_dig = 1, δ = 0,005 mg) doğrulanmaz.
4. 'jcgm101-9.3-mass-calibration-higher-order' projesi, değerlendirmeyi GUM 5.1.2'nin ikinci mertebe terimleriyle tekrarlar; bu sonuç doğrulanır.

**Sonuç:**

- dm = (1,23 ± 0,10) mg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- JCGM 101 Tablo 6, M = 0,72 × 10⁶ ile en kısa aralığı [1,0834; 1,3825] olarak verir; çıktı dağılımı 1,2340 etrafında simetrik olduğundan burada çok daha az saçılımlı olan olasılıksal simetrik aralık kullanılmıştır.

**Çalıştırma:** `uncertainty run library/mass/jcgm101-9.3-mass-calibration.json --lang tr`

### Kütle kalibrasyonu (ikinci mertebe terimli GUM)

`jcgm101-9.3-mass-calibration-higher-order` — JCGM 101:2008, 9.3, Table 6; JCGM 100:2008, 5.1.2; JCGM GUM-5:2026, Example 7, Table 7.2

JCGM 101 9.3: aynı model, GUM 5.1.2'nin yüksek mertebe terimleriyle değerlendirilmiştir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. W ağırlığının konvansiyonel kütlesi, havada R ile karşılaştırmalı tartımdan elde edilir: m_W = (m_R,c + δm_R,c)[1 + (ρ_a − ρ_a0)(1/ρ_W − 1/ρ_R)]; sonuç δm = m_W − 100 g sapması olarak (mg) verilir.
2. Tahmin değerlerinde ρ_a = ρ_a0 = 1,2 kg/m³ olduğundan üç yoğunluğun duyarlılık katsayıları sıfırdır: birinci mertebe değerlendirme kaldırma kuvveti düzeltmesini tamamen yok sayar (u = 0,0539 mg).
3. İkinci mertebe terimler (ρ_a − 1,2)(1/ρ_W − 1/ρ_R) çarpımlarını yakalar ve u = 0,0750 mg verir; bu değer Monte Carlo sonucuyla uyumludur ve n_dig = 1 için doğrulanır.
4. GUM 5.1.2 notu bu terimleri normal dağılımlı girdiler için türetir; yoğunluklar dikdörtgen dağılımlı olduğundan program uyarı verir. Monte Carlo yönteminde böyle bir kısıt yoktur.

**Sonuç:**

- dm = (1,23 ± 0,15) mg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/jcgm101-9.3-mass-calibration-higher-order.json --lang tr`

### OIML M1 sınıfı 10 kg ağırlık (UKAS M3003 K.4)

`m3003-k4-weight-10kg` — UKAS M3003 ed. 6 (2024), Appendix K.4

Kütle karşılaştırıcısında F2 sınıfı bir standartla üç ara-okumalı karşılaştırma.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. W_X = W_S + ΔW + δD_S + δI_d + ΔC + ΔA_b + δW_r (K.4.1): sertifika 30 mg (k = 2), sürüklenme ±30 mg, yuvarlama ±10 mg (üçgen, iki okuma), doğrusal olmama ±3 mg, kaldırma kuvveti ±10 mg.
2. Üç karşılaştırmadan ortalama fark ΔW = +0,020 g; tekrarlanabilirlik önceki bir değerlendirmeden, s = 8,7 mg, 9 serbestlik derecesi; u = 8,7/√3 = 5,0 mg.
3. u_c = 24,56 mg; W_X = 10 000,025 g ± 0,049 g (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- W_X = (10000,025 ± 0,049) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F55: K.4.7, m = 10 karşılaştırma için ν = (m − 1) = 10 serbestlik derecesi yazar; bütçe m − 1 olan 9'u kullanır. Bütçenin altındaki not 10 okumadan söz eder.

**Çalıştırma:** `uncertainty run library/mass/m3003-k4-weight-10kg.json --lang tr`

### 200 g'de 205 g kapasiteli tartı aleti (UKAS M3003 K.5)

`m3003-k5-weighing-machine-200g` — UKAS M3003 ed. 6 (2024), Appendix K.5

0,1 mg basamaklı elektronik terazinin E2 sınıfı 200 g ağırlıkla gösterge hatası; tek okuma.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. ΔI_X = I_X − W_S + δD_S + δI_d0 + δI_d + ΔA_b + δI_r (K.5.1): sertifika 0,1 mg (k = 2), sürüklenme ±0,1 mg, sıfırda ve yükte ±0,05 mg yuvarlama, kaldırma kuvveti ±0,2 mg, tekrarlanabilirlik s = 0,05 mg (ν = 9, tek okuma).
2. 200 g standart için I_X = 199,9999 g (kütlesi nominal değer alınmıştır); model ile ΔI_X = −0,10 mg.
3. Bütçe katkılarından u_c = 0,153 mg; ΔI_X = −0,10 mg ± 0,31 mg (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- dI_X = (-0,10 ± 0,30) mg; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F56: K.5.9'un u_i sütunundaki katkılar (0,05, 0,058, 0,029, 0,029, 0,115, 0,05 mg) 0,150 mg değil 0,153 mg verir; basılı değer, sürüklenme sınırı √3 yerine 2'ye bölünseydi elde edilirdi. K.5.1 modeline göre ΔI_X = I_X − W_S = 199,9999 g − 200 g = −0,10 mg iken raporlanan gösterge hatası +0,10 mg'dir.

**Çalıştırma:** `uncertainty run library/mass/m3003-k5-weighing-machine-200g.json --lang tr`

### Tartımla toz kütlesi (NIST TN 1900 E1)

`nist-tn1900-e1-weighing` — NIST Technical Note 1900, Example E1, Exhibit 1

TN 1900 E1: tek kefeli terazide üç tartımdan toz kütlesi; doğrusal model, GUM ve MCM aynı sonucu verir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Tozun kütlesi m_P = c_P − c_E − (c_R2 − c_R1) bağıntısıyla bulunur: toz dolu kabın okuması, boş kabın okuması ve referans ağırlığın iki okuması arasındaki terazi kaymasının farkı.
2. c_P ve c_E'nin standart belirsizlikleri (0,005015 g), kabın belirsizliğini (0,005 g) terazinin kalibrasyon, tekrarlanabilirlik, doğrusallık ve sıcaklık bileşenleriyle (0,1; 0,1; 0,2 ve 0,3 mg) birleştirir; kayma teriminin belirsizliği 0,0005477 g'dır (TN 1900 Şekil 1).
3. Tüm duyarlılık katsayıları ±1 olduğundan u(m_P)² = u(c_P)² + u(c_E)² + u(c_R2 − c_R1)², u = 0,0071 g olur; çıktı normal dağılımlıdır ve Monte Carlo yöntemi aynı %95 aralığını, [50,270; 50,298] g, verir.

**Sonuç:**

- m_P = (50,284 ± 0,014) g; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/mass/nist-tn1900-e1-weighing.json --lang tr`

## Fiziksel özellikler

### Bozunma için düzeltilmiş ²⁰⁷Bi kaynağının aktivitesi (JCGM GUM-5 Örnek 14)

`gum5-e14-radioactive-decay-bi207` — JCGM GUM-5:2026, Example 14, 14.3 and Table 14.1

Sertifikalı aktivite, literatürden yarı ömür ve Δt = 2,73 a ile A₁ = A₀ exp(−Δt ln2/T½); doğrusallaştırmanın denetimi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. A₀ = 11,450 kBq, u = 0,050 kBq (sertifika); T½ = 32,9 a, u = 1,4 a (literatür, %4); Δt = t₁ − t₀ = 2,73 a, belirsizliksiz.
2. E = exp(−Δt ln2/T½) = 0,94411, A₁ = 10,810 kBq. c(A₀) = E, c(T½) = A₁Δt ln2/T½² = 0,018898 kBq/a: katkılar 0,0472 kBq ve 0,0265 kBq, u(A₁) = 0,054 kBq.
3. Model T½'ye göre doğrusal değildir. T½ ± 3u(T½) arasında doğrusallaştırılmış ve kesin modeller en çok 0,011 kBq farklıdır; bu u(A₁)'den çok küçüktür, dolayısıyla yayılım yasası yeterlidir (Tablo 14.1). Monte Carlo sonucu bunu doğrular.
4. Doğrusal olmamanın pratik ölçüsü u(E)/E = ln2·(u(T½)/T½)·(Δt/T½)'dir; yaklaşık %1'e ulaştığında doğrusallaştırma denetlenmelidir (GUM-5 14.4.2). Burada %0,25'tir.

**Sonuç:**

- A_1 = (10,81 ± 0,11) kBq; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/physical/gum5-e14-radioactive-decay-bi207.json --lang tr`

### Altı tahribatlı deneyden çelik halatın kopma kuvveti (JCGM GUM-5 Örnek 15)

`gum5-e15-wire-rope-breaking-force` — JCGM GUM-5:2026, Example 15, 15.3, 15.4 and 15.5

F₀ = F̄ − B − B_ref − e_res − e_T: bir makaradan kesilen altı parça üzerindeki deneylerin ortalaması; yük hücreleri, çözünürlük ve sıcaklık için dikdörtgen düzeltmeler.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Kopma deneyi aynı numunede tekrarlanamaz; aynı makaradan altı parça denenir ve bunların değişkenliği (halatın homojen olmaması) sonuçlara gömülür (GUM-6 gözlem denklemleri).
2. F̄ = 10005,7 kN, s = 1,75 kN, u(F̄) = s/√6 = 0,71 kN, ν = 5; B, B_ref, e_res, e_T düzeltmeleri yarı genişlikleri 6,8, 8,2, 8,7 ve 10,2 kN olan dikdörtgen dağılımlar.
3. u(F₀) = 9,9 kN, ν_eff = 5·(u(F₀)/u(F̄))⁴ ≈ 1,8e+05, k = 1,96, U = 19,4 kN. Deney sonuçları u(F₀)'a yalnızca %0,2 katkı yapar; düzeltmeler baskındır.

**Sonuç:**

- F_0 = (10006 ± 19) kN; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- GUM-5 15.5.2, 10,1 kN MCM standart sapması ve [9986,0; 10 025,3] kN aralığı basar. F için belirtilen t dağılımıyla (ölçek u(F̄) = 0,7 kN, ν = 5) MCM standart sapması √(u_c² − u²(F̄) + u²(F̄)·5/3) = 9,93 kN'dur; pratikte GUF değeri 9,9 kN'dur. 10,1 kN, ölçek olarak tek deneyin standart sapması s = 1,8 kN alınırsa çıkar: 10,14 kN (araştırma günlüğü F77).

**Çalıştırma:** `uncertainty run library/physical/gum5-e15-wire-rope-breaking-force.json --lang tr`

### Ziftin yumuşama noktası: eksik verilerle tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2 Örnek C.2)

`iso5725-2-c2-softening-point-pitch` — ISO 5725-2:2019, Annex C.2 (Tables C.7–C.12, Formulae C.25–C.33)

ISO 5725-2:2019 C.2: halka-bilye yöntemiyle ziftin yumuşama noktası; dört düzeyde (88 °C – 102 °C), ikili sonuçlu ve iki eksik hücreli on altı laboratuvar.

**Yaklaşım:** laboratuvarlar arası deneyden tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2)

**Adımlar:**

1. Form A (Tablo C.7): on altı laboratuvar, dört düzey, hücre başına iki sonuç. 8. laboratuvarın 1. düzey numunesi yoktur; 5. laboratuvarın 2. düzeyde tek sonucu vardır ve atılır (8.4.3 a); bu nedenle 1. ve 2. düzeylerde p = 15.
2. Hücre başına iki sonuçla Cochran istatistiği en büyük mutlak fark karesinin fark kareleri toplamına oranıdır (8.2.11, Not 1): C = 0,391, 0,424, 0,434 ve 0,380 (Tablo C.10); %5 kritik değerleri 0,471 (p = 15) ve 0,452 (p = 16)'nın altındadır.
3. Hücre ortalamalarına Grubbs testleri (Tablo C.11): tekli veya ikili şüpheli ya da aykırı değer yoktur.
4. n = 2 için varyanslar Formül (29) ve (30)'dan çıkar: 1. düzeyde s_r² = Σ(y_i1 − y_i2)²/(2p) = 1,2303 ve s_L² = 1,5575; böylece m̂ = 88,40 °C, s_r = 1,109 °C ve s_R = 1,670 °C (C.25–C.33).

**Sonuç:**

- Düzey 1: m̂ = 88,397 °C, s_r = 1,109 °C, s_R = 1,670 °C (p = 15)
- Düzey 2: m̂ = 96,267 °C, s_r = 0,9252 °C, s_R = 1,597 °C (p = 15)
- Düzey 3: m̂ = 97,069 °C, s_r = 0,9934 °C, s_R = 2,010 °C (p = 16)
- Düzey 4: m̂ = 101,96 °C, s_r = 1,004 °C, s_R = 1,918 °C (p = 16)

**Notlar:**

- Tablo C.12 4. düzeyde s_R = 1,915 °C verir; Formül (29)–(31) 1,9175 °C (1,918 °C) verir; bu REML Tablo C.13'ün değeridir (araştırma günlüğü F48). ISO 5725-2:1994 de aynı 1,915 °C değerini basar.

**Çalıştırma:** `uncertainty run library/physical/iso5725-2-c2-softening-point-pitch.json --lang tr`

### Ziftin yumuşama noktası: REML tahminleri (ISO 5725-2 Örnek C.2)

`iso5725-2-c2-softening-point-pitch-reml` — ISO 5725-2:2019, Annex C.2.9 (Table C.13), Annex B.2

ISO 5725-2:2019 C.2.9: Örnek C.2 verilerinin kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi.

**Yaklaşım:** laboratuvarlar arası deneyden tekrarlanabilirlik ve tekrarüretilebilirlik (ISO 5725-2)

**Adımlar:**

1. Form A (Tablo C.7): on altı laboratuvar, dört düzey, hücre başına iki sonuç. 8. laboratuvarın 1. düzey numunesi yoktur; 5. laboratuvarın 2. düzeyde tek sonucu vardır ve atılır (8.4.3 a); bu nedenle 1. ve 2. düzeylerde p = 15.
2. Her hücrede iki sonuç varken σ_L² tahmini negatif olmadıkça REML tahminleri ANOVA tahminlerine eşittir; Tablo C.13, 4. düzeydeki s_R dışında Tablo C.12 ile aynıdır (notlara bakınız).

**Sonuç:**

- Düzey 1: m̂ = 88,397 °C, s_r = 1,109 °C, s_R = 1,670 °C (p = 15)
- Düzey 2: m̂ = 96,267 °C, s_r = 0,9252 °C, s_R = 1,597 °C (p = 15)
- Düzey 3: m̂ = 97,069 °C, s_r = 0,9934 °C, s_R = 2,010 °C (p = 16)
- Düzey 4: m̂ = 101,96 °C, s_r = 1,004 °C, s_R = 1,918 °C (p = 16)

**Notlar:**

- Tablo C.13 4. düzeyde s_R = 1,918 °C verirken Tablo C.12 1,915 °C verir; dengeli hücrelerde her iki yöntem de 1,9175 °C verir (araştırma günlüğü F48).

**Çalıştırma:** `uncertainty run library/physical/iso5725-2-c2-softening-point-pitch-reml.json --lang tr`

### Isıl genleşme katsayısı, normal girdiler, %95 (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-gaussian-95` — NIST Technical Note 1900, Example E13, Exhibit 20

TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; normal girdiler, %95 kapsam.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. α = (L₁ − L₀)/(L₀(T₁ − T₀)); T₀ = 288,15 K'de L₀ = 1,4999 m ve T₁ = 373,10 K'de L₁ = 1,5021 m; u(L₀) = 0,0001 m, u(L₁) = 0,0002 m, u(T₀) = 0,02 K, u(T₁) = 0,05 K. Sonuç 10⁻⁶ K⁻¹ biriminde verilir.
2. Belirsizliğin neredeyse tamamı 2,2 mm'lik uzunluk farkından gelir: u(α) = 1,8 × 10⁻⁶ K⁻¹, yani α = 17,3 × 10⁻⁶ K⁻¹'in yaklaşık %10'u.
3. Normal girdilerle GUM ve Monte Carlo sonuçları uyuşur; Monte Carlo örnekleminin yüzdeliklerinden %95 aralığı [13,8; 20,7] × 10⁻⁶ K⁻¹ olur.

**Sonuç:**

- alpha = (17,3 ± 3,4) 10⁻⁶ K⁻¹; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- Eşlik eden nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} projeleri iki girdi modelini %95 ve %99'da karşılaştırır: t₃ girdilerle %95 aralığı normal girdilere göre daha kısa, %99 aralığı ise daha uzundur.

**Çalıştırma:** `uncertainty run library/physical/nist-tn1900-e13-thermal-expansion-gaussian-95.json --lang tr`

### Isıl genleşme katsayısı, normal girdiler, %99 (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-gaussian-99` — NIST Technical Note 1900, Example E13, Exhibit 20

TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; normal girdiler, %99 kapsam.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. α = (L₁ − L₀)/(L₀(T₁ − T₀)); T₀ = 288,15 K'de L₀ = 1,4999 m ve T₁ = 373,10 K'de L₁ = 1,5021 m; u(L₀) = 0,0001 m, u(L₁) = 0,0002 m, u(T₀) = 0,02 K, u(T₁) = 0,05 K. Sonuç 10⁻⁶ K⁻¹ biriminde verilir.
2. Belirsizliğin neredeyse tamamı 2,2 mm'lik uzunluk farkından gelir: u(α) = 1,8 × 10⁻⁶ K⁻¹, yani α = 17,3 × 10⁻⁶ K⁻¹'in yaklaşık %10'u.
3. Normal girdilerle GUM ve Monte Carlo sonuçları uyuşur; Monte Carlo örnekleminin yüzdeliklerinden %99 aralığı [12,7; 21,8] × 10⁻⁶ K⁻¹ olur.

**Sonuç:**

- alpha = (17,3 ± 4,5) 10⁻⁶ K⁻¹; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,58 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %99 kapsam olasılığına karşılık gelir.

**Notlar:**

- Eşlik eden nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} projeleri iki girdi modelini %95 ve %99'da karşılaştırır: t₃ girdilerle %95 aralığı normal girdilere göre daha kısa, %99 aralığı ise daha uzundur.

**Çalıştırma:** `uncertainty run library/physical/nist-tn1900-e13-thermal-expansion-gaussian-99.json --lang tr`

### Isıl genleşme katsayısı, Student t₃ girdiler, %95 (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-student-95` — NIST Technical Note 1900, Example E13, Exhibit 20

TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; Student t₃ girdiler, %95 kapsam.

**Yaklaşım:** Yalnızca Monte Carlo yöntemi (JCGM 101); sonuçlar için projeyi çalıştırın

**Adımlar:**

1. α = (L₁ − L₀)/(L₀(T₁ − T₀)); T₀ = 288,15 K'de L₀ = 1,4999 m ve T₁ = 373,10 K'de L₁ = 1,5021 m; u(L₀) = 0,0001 m, u(L₁) = 0,0002 m, u(T₀) = 0,02 K, u(T₁) = 0,05 K. Sonuç 10⁻⁶ K⁻¹ biriminde verilir.
2. Belirsizliğin neredeyse tamamı 2,2 mm'lik uzunluk farkından gelir: u(α) = 1,8 × 10⁻⁶ K⁻¹, yani α = 17,3 × 10⁻⁶ K⁻¹'in yaklaşık %10'u.
3. Her tahmin dört tekrar gözlemin ortalaması kabul edildiğinden TN 1900, standart sapmaları standart belirsizliklere eşit olacak şekilde ölçeklenmiş t₃ dağılımları atar. α ve u(α) değişmez, ancak çıktı daha sivri ve kuyrukları daha ağırdır: %95 aralığı [14,0; 20,5] × 10⁻⁶ K⁻¹ olur.

**Notlar:**

- Eşlik eden nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} projeleri iki girdi modelini %95 ve %99'da karşılaştırır: t₃ girdilerle %95 aralığı normal girdilere göre daha kısa, %99 aralığı ise daha uzundur.
- TN 1900 her t₃ dağılımını standart sapması u olacak şekilde ölçekler (ölçek u/√3); JCGM 101 6.4.9 ise ölçek olarak s/√n = u alır. Bir t girdisinin GUM belirsizliği ölçeği olduğundan burada GUM değerlendirmesi kapatılmıştır; GUM sonucu normal girdili projelerdedir.

**Çalıştırma:** `uncertainty run library/physical/nist-tn1900-e13-thermal-expansion-student-95.json --lang tr`

### Isıl genleşme katsayısı, Student t₃ girdiler, %99 (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-student-99` — NIST Technical Note 1900, Example E13, Exhibit 20

TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; Student t₃ girdiler, %99 kapsam.

**Yaklaşım:** Yalnızca Monte Carlo yöntemi (JCGM 101); sonuçlar için projeyi çalıştırın

**Adımlar:**

1. α = (L₁ − L₀)/(L₀(T₁ − T₀)); T₀ = 288,15 K'de L₀ = 1,4999 m ve T₁ = 373,10 K'de L₁ = 1,5021 m; u(L₀) = 0,0001 m, u(L₁) = 0,0002 m, u(T₀) = 0,02 K, u(T₁) = 0,05 K. Sonuç 10⁻⁶ K⁻¹ biriminde verilir.
2. Belirsizliğin neredeyse tamamı 2,2 mm'lik uzunluk farkından gelir: u(α) = 1,8 × 10⁻⁶ K⁻¹, yani α = 17,3 × 10⁻⁶ K⁻¹'in yaklaşık %10'u.
3. Her tahmin dört tekrar gözlemin ortalaması kabul edildiğinden TN 1900, standart sapmaları standart belirsizliklere eşit olacak şekilde ölçeklenmiş t₃ dağılımları atar. α ve u(α) değişmez, ancak çıktı daha sivri ve kuyrukları daha ağırdır: %99 aralığı [11,5; 23,0] × 10⁻⁶ K⁻¹ olur.

**Notlar:**

- Eşlik eden nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} projeleri iki girdi modelini %95 ve %99'da karşılaştırır: t₃ girdilerle %95 aralığı normal girdilere göre daha kısa, %99 aralığı ise daha uzundur.
- TN 1900 her t₃ dağılımını standart sapması u olacak şekilde ölçekler (ölçek u/√3); JCGM 101 6.4.9 ise ölçek olarak s/√n = u alır. Bir t girdisinin GUM belirsizliği ölçeği olduğundan burada GUM değerlendirmesi kapatılmıştır; GUM sonucu normal girdili projelerdedir.

**Çalıştırma:** `uncertainty run library/physical/nist-tn1900-e13-thermal-expansion-student-99.json --lang tr`

### Düşen bilyalı viskozimetre (NIST TN 1900 E3)

`nist-tn1900-e3-falling-ball-viscometer` — NIST Technical Note 1900, Example E3, Exhibit 3

TN 1900 E3: bir NaOH çözeltisinin dinamik viskozitesi kalibrasyon sıvısıyla karşılaştırılarak; düşme sürelerinin büyük bağıl belirsizlikleri çıktıyı çarpık yapar.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. μ_M = μ_C (ρ_B − ρ_M)/(ρ_B − ρ_C) · t_M/t_C; μ_C = 4,63 mPa s, ρ_B = 2217, ρ_C = 810, ρ_M = 1180 kg/m³, t_C = 36,6 s ve t_M = 61 s; tüm girdiler normal dağılımlı.
2. Bağıl standart belirsizlikler μ_C için %1, t_C için %15 ve t_M için %10'dur; yoğunlukların u'su 0,5 kg/m³'tür. TN 1900 bu değerlerin bilerek büyük seçildiğini belirtir.
3. Tahminlerin yerine konmasıyla 5,69 mPa s ve birinci mertebe yayılım yasasıyla u = 1,03 mPa s bulunur. 1/t_C, ±%15 aralığında belirgin biçimde doğrusal olmadığından Monte Carlo ortalaması 5,82 mPa s, u = 1,11 mPa s olur ve %95 aralığı, [4,05; 8,39] mPa s, tahmine göre asimetriktir.
4. Bu nedenle GUM aralığı, 5,69 ± 2,01 mPa s, doğrulanmaz: uç noktaları Monte Carlo aralığınınkilerin 0,38 mPa s ve 0,69 mPa s altındadır. En kısa Monte Carlo aralığı, [3,84; 8,05] mPa s, TN 1900'ün kullandığı olasılıksal simetrik aralıktan da farklıdır.

**Sonuç:**

- mu_M = (5,7 ± 2,0) mPa s; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- TN 1900, NIST Uncertainty Machine'de uygulanan klasik yayılım formülünün 1,11 mPa s verdiğini ve bunun Monte Carlo değeriyle aynı olduğunu belirtir. Birinci mertebe yayılım yasası (GUM eş. 10) verilen model ve girdilere uygulandığında 1,027 mPa s; GUM 5.1.2'nin ikinci mertebe terimleriyle yaklaşık 1,10 mPa s verir. NIST Uncertainty Machine kılavuzu (sürüm 1.4, §9) aynı ifadeyi tekrarlar; ancak Şekil 5'te bastığı çıktı Gauss formülü için u(y) = 1 mPa s verir ve bu 1,027 mPa s ile tutarlıdır: metindeki 1,11 mPa s Monte Carlo değeridir.

**Çalıştırma:** `uncertainty run library/physical/nist-tn1900-e3-falling-ball-viscometer.json --lang tr`

### CODATA 2010 değerlerinden Stefan–Boltzmann sabiti (NIST Uncertainty Machine §11)

`num-11-stefan-boltzmann` — T. Lafarge, §11, Table 5 and Figure 7

NUM kılavuzu §11: 2010 CODATA değerlerinden σ = 32π⁵hR⁴R∞⁴/(15A_r(e)⁴M_u⁴c⁶α⁸); belirsizliğin neredeyse tamamı molar gaz sabiti R'den gelir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Girdiler: h = 6,626 069 57(29) × 10⁻³⁴ J s, R = 8,314 462 1(75) J mol⁻¹ K⁻¹, R∞ = 10 973 731,568 539(55) m⁻¹, A_r(e) = 5,485 799 094 6(22) × 10⁻⁴, α = 7,297 352 569 8(24) × 10⁻³; M_u = 10⁻³ kg/mol ve c = 299 792 458 m/s kesindir. Sonuç 10⁻⁸ W m⁻² K⁻⁴ biriminde verilir.
2. Üslerin çarpımında bağıl varyans Σ(p_i u_r(x_i))²'dir: R 4. kuvvetle girer ve u_r(R) = 9,0 × 10⁻⁷ olduğundan u_r(σ) = 3,6 × 10⁻⁶ ve u(σ) = 2,05 × 10⁻¹³ W m⁻² K⁻⁴ olur; diğer girdilerin katkısı %0,02'nin altındadır.

**Sonuç:**

- sigma = (5,6703725 ± 0,0000401) 10⁻⁸ W m⁻² K⁻⁴; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- CODATA ayarlaması bu sabitleri korelasyonlu kılar; kılavuzda belirtildiği gibi korelasyonların hesaba katılması u(σ)'yı raporlanan basamakta değiştirmez. SI'nın 2019'daki yeniden tanımından beri h kesindir ve σ = 5,670 374 419… × 10⁻⁸ W m⁻² K⁻⁴ belirsizlik taşımaz.

**Çalıştırma:** `uncertainty run library/physical/num-11-stefan-boltzmann.json --lang tr`

## Basınç

### Ölü ağırlık test cihazına karşı 2 MPa'da dijital basınç göstergesi (UKAS M3003 K.8)

`m3003-k8-pressure-indicator-2mpa` — UKAS M3003 ed. 6 (2024), Appendix K.8

2 MPa'da bir dijital basınç göstergesinin hatası; bütçe, basınçta alışıldığı gibi göreli (ppm) olarak verilir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Kütlelerden, etkin alan A₀ = 80,6516 mm², yerçekimi, kaldırma kuvveti ve sıvı yükünden üretilen basınç P_X = 2,000 806 MPa (K.8.15); gösterge I_X = 2,000 84 MPa.
2. P_X'e göreli katkılar: alan 32,2 ppm (k = 2) ve sürüklenme ±30 ppm; deformasyon 1,0 ppm (k = 2); kütleler 7,3 ppm (k = 2, bütçedeki gibi) ve sürüklenme ±7,6 ppm; piston sıcaklığı ±0,5 °C × 23 ppm/°C; yerçekimi 3 ppm (k = 2); kaldırma kuvveti ±13 ppm; sıvı yükü ±13,5 ppm; diğer sıvı etkileri ±2 ppm; düşeylik ±3,4 ppm.
3. Gösterge: yuvarlama ±50 ppm (200 Pa adım) ve tekrarlanabilirlik 16 ppm (ν = 9, tek okuma). u_c = 43,0 ppm; ΔI_X = 17 ppm ± 86 ppm (k = 2), yani 34 Pa ± 170 Pa.
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- e = (17 ± 86) ppm; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F60: K.8.5.1, 7,3 ppm'lik göreli kütle belirsizliğinin dikdörtgen dağılımın yarı genişliği olarak alınacağını söyler; bütçe ise onu k = 2 ile genişletilmiş belirsizlik olarak alır (u = 4,21 ppm yerine 3,65 ppm). u_c'ye etkisi son basılı basamağın altındadır (her iki durumda 43,0 ppm).

**Çalıştırma:** `uncertainty run library/pressure/m3003-k8-pressure-indicator-2mpa.json --lang tr`

## Sıcaklık

### Sıcaklık blok kalibratörünün 180 °C'de kalibrasyonu

`ea402-s11-block-calibrator-180c` — EA-4/02 M:2022, S11

EA-4/02 S11: bloğun sıcaklığının kalibre edilmiş platin direnç termometresiyle ölçülmesi (değerler mK).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. t_X = t_S + δt_S + δt_D − δt_iX + δt_R + δt_A + δt_H + δt_V (mK).
2. Eksenel homojensizlik (±250 mK) ve radyal farklar (±100 mK) baskındır: β = 0,43 olan yamuk; EA-4/02 k = 1,81 verir (Eş. S10.9 1,80 verir); t_X = (180,1 ± 0,3) °C.

**Sonuç:**

- t_X = (180100 ± 330) mK; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu: rehber β = 0,43 için k = 1,81 verir; Eş. (S10.9) 1,80 verir. Raporlanan (180,1 ± 0,3) °C değişmez.

**Çalıştırma:** `uncertainty run library/temperature/ea402-s11-block-calibrator-180c.json --lang tr`

### Termokupl kalibrasyonu: bilinmeyenin emk'sı

`ea402-s5-thermocouple-emf` — EA-4/02 M:2022, S5

EA-4/02 S5 (ikinci bütçe): kalibre edilen N tipi termokuplun 1000 °C'deki emk'sı.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. V_X = V_iX + δV_iX1 + δV_iX2 + δV_R + δV_LX + Δt/C_X − δt_0X/C_X0; burada Δt = t − t_X, kalibrasyon noktası (1000 °C) ile birinci bütçedeki fırın sıcaklığı arasındaki farktır ve u = 0,641 °C'dir.
2. Fırın sıcaklığı baskındır (25,0 µV'nin 24,65 µV'u); V_X = (36 229 ± 50) µV (k = 2).

**Sonuç:**

- V_X = (36229 ± 50) µV; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Bulgu: δt'nin basılı katkısı (24,5 µV) 0,641 °C × 38,46 µV/°C = 24,65 µV ile tutarsızdır; basılı toplam 25,0 µV, 24,65 değerinden elde edilir. Δt'nin tahmini −0,5 °C'dir (basılı 0,5 °C); bu, belirtilen V_X = 36 248 − 19,2 = 36 229 µV sonucunu verir.

**Çalıştırma:** `uncertainty run library/temperature/ea402-s5-thermocouple-emf.json --lang tr`

### Termokupl kalibrasyonu: fırın sıcaklığı

`ea402-s5-thermocouple-furnace` — EA-4/02 M:2022, S5

EA-4/02 S5 (birinci bütçe): 1000 °C'deki fırının sıcaklığının N tipi referans termokupl ile ölçülmesi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. t_X = t_S(V_iS + δV_iS1 + δV_iS2 + δV_R − δt_0S/C_S0) + δt_S + δt_D + δt_F; referans termokuplun duyarlılığı C_S = 0,077 °C/µV ve referans eklemde C_S0 = 0,189 °C/µV ile doğrusallaştırılmıştır.
2. Fırın sıcaklığının homojen olmaması (±1 °C, dikdörtgen) baskındır: u(t_X) = 0,641 °C.

**Sonuç:**

- t_X = (1000,5 ± 1,3) °C; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/temperature/ea402-s5-thermocouple-furnace.json --lang tr`

### Direnç termometresiyle Celsius sıcaklığı, örtük model (JCGM 102 9.5.2)

`jcgm102-9.5.2-resistance-thermometer` — JCGM 102:2011, 9.5.2 (Tables 12, 13); JCGM GUM-5:2026, Example 13, 13.2

JCGM 102:2011 9.5.2: termometrenin korelasyonlu kalibrasyon sabitleriyle ölçülen bir direnç oranından tek bir sıcaklık.

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102)

**Adımlar:**

1. Örtük model h(θ, X) = R₀(1 + Aθ + Bθ²) − r R_S = 0 (31): termometre direnci R(θ), r = R(θ)/R_S oranını ölçen bir köprüyle standart direnç R_S ile karşılaştırılır.
2. Kalibrasyon sabitleri R₀, A ve B koreledir (Tablo 13); θ Newton yöntemiyle, u(θ) ise C_y = (A + 2Bθ)R₀ ile C_y U_y C_yᵀ = C_x U_x C_xᵀ bağıntısından bulunur (6.3.1.3, 9.5.2.6).

**Sonuç:**

- theta = 20,02317 °C, u(theta) = 0,0045 °C (GUM)

**Çalıştırma:** `uncertainty run library/temperature/jcgm102-9.5.2-resistance-thermometer.json --lang tr`

### Tek direnç termometresiyle on Celsius sıcaklığı, örtük model (JCGM 102 9.5.3)

`jcgm102-9.5.3-ten-temperatures` — JCGM 102:2011, 9.5.3 (Tables 14–16); JCGM GUM-5:2026, Example 13, 13.3, Tables 13.4 and 13.5

JCGM 102:2011 9.5.3: aynı kalibre termometreyle ölçülen on sıcaklık, kalibrasyon sabitleri üzerinden koreledir (Tablo 15 ve 16).

**Yaklaşım:** birden çok çıktı büyüklüğü için GUM belirsizlik yaklaşımı (JCGM 102)

**Adımlar:**

1. Örtük model h(θ, X) = R₀(1 + Aθ + Bθ²) − r R_S = 0 (31): termometre direnci R(θ), r = R(θ)/R_S oranını ölçen bir köprüyle standart direnç R_S ile karşılaştırılır.
2. Kalibrasyon sabitleri R₀, A ve B koreledir (Tablo 13); θ Newton yöntemiyle, u(θ) ise C_y = (A + 2Bθ)R₀ ile C_y U_y C_yᵀ = C_x U_x C_xᵀ bağıntısından bulunur (6.3.1.3, 9.5.2.6).
3. On denklem (32) R₀, A, B ve R_S'yi paylaşır; C_y köşegendir ve C_x = [C_x⁽¹⁾ C_x⁽²⁾] (9.5.3.5). Belirsizlik 0 °C yakınında en küçüktür ve kalibrasyon noktalarının dışında, 25 °C'nin üzerinde artar (9.5.3.6).

**Sonuç:**

- theta1 = 0,00998 °C, u(theta1) = 0,0018 °C (GUM)
- theta2 = 3,84911 °C, u(theta2) = 0,0027 °C (GUM)
- theta3 = 7,69279 °C, u(theta3) = 0,0040 °C (GUM)
- theta4 = 11,54101 °C, u(theta4) = 0,0046 °C (GUM)
- theta5 = 15,39378 °C, u(theta5) = 0,0047 °C (GUM)
- theta6 = 20,02317 °C, u(theta6) = 0,0045 °C (GUM)
- theta7 = 23,11312 °C, u(theta7) = 0,0046 °C (GUM)
- theta8 = 26,97971 °C, u(theta8) = 0,0060 °C (GUM)
- theta9 = 30,85093 °C, u(theta9) = 0,0089 °C (GUM)
- theta10 = 20,02317 °C, u(theta10) = 0,0045 °C (GUM)
- %95 kapsam bölgesi (GUM): hiper-elipsoit (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p², k_p = 4,28; dikdörtgen k_q = 2,81

**Notlar:**

- Tablo 15 yedinci sütunu θ7 yerine θ6 olarak adlandırır.

**Çalıştırma:** `uncertainty run library/temperature/jcgm102-9.5.3-ten-temperatures.json --lang tr`

### Kalibre edilmiş PT100 probdan kaptaki gazın sıcaklığı (UKAS M3003 K.11, ikinci bütçe)

`m3003-k11-gas-temperature` — UKAS M3003 ed. 6 (2024), Appendix K.11 (K.11.17–K.11.18)

L probu kabın dışına bağlanır; düzeltmesi ΔT_u = 0,000 55·I_u + 0,419 °C kalibrasyon fonksiyonundan gelir.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. T_g = (I_u − ΔT_u) − ΔT_ug − δT_h − δT_r (K.11.11): I_u = 23,4 °C (dördün ortalaması), ΔT_u = 0,431 °C.
2. L'nin kalibrasyonu U = 0,084 49 °C (k = 2, birinci bütçe), uydurma ±0,012 °C, sürüklenme ±0,004 67 °C, prob–gaz farkı ±0,1 °C, kendi kendine ısınma ±4 mK, dört okumanın ortalaması için tekrarlanabilirlik s = 0,076 °C (ν = 49).
3. u_c = 0,0864 °C; T_g = 22,97 °C ± 0,17 °C (k = 2).
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- T_g = (22,97 ± 0,17) °C; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/temperature/m3003-k11-gas-temperature.json --lang tr`

### Kuru blokta referans proba karşı PT100 prob kalibrasyonu (UKAS M3003 K.11, birinci bütçe)

`m3003-k11-probe-calibration` — UKAS M3003 ed. 6 (2024), Appendix K.11 (K.11.9)

Laboratuvar probu L'nin 20,4 °C'de, kalibrasyon fonksiyonu ΔT_S = 0,000 01·I_S + 0,0108 °C olan referans prob S'ye karşı hatası.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. ΔT_L = I_L − (I_S − ΔT_S − ΔT_SL) − δΔT_L (K.11.2): çözünürlük 0,1 °C (L) ve 0,01 °C (S), S'nin kalibrasyonu U = 0,024 °C (k = 2), uydurma ±0,005 °C, sürüklenme ±0,008 °C, blok gradyanları ±0,048 °C, tekrarlanabilirlik 0,001 23 °C (ν = 24).
2. I_S = 19,979 °C, K.11.10 tablosundaki gibi I_L = 20,4 °C'de ΔT_L = 0,432 °C olacak biçimde türetilmiştir (S'nin göstergesi basılmamıştır).
3. u_c = 0,042 25 °C, U = 0,084 °C (k = 2); bu U ikinci bütçeye L'nin kalibrasyon belirsizliği olarak girer.
4. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- dT_L = (0,432 ± 0,084) °C; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/temperature/m3003-k11-probe-calibration.json --lang tr`

### 1000 °C'de N tipi termoçift: tam bütçesiyle EMK (UKAS M3003 K.7)

`m3003-k7-thermocouple-emf` — UKAS M3003 ed. 6 (2024), Appendix K.7

K.7'nin iki bütçesi tek modelde: fırın sıcaklığı bilinmeyen termoçiftin EMK'sına 1/c_X = 38,5 µV/°C duyarlılığıyla girer.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. N tipi bir termoçift 1000 °C'de iki R tipi referans termoçiftle karşılaştırılır; ters çevrilmiş kutuplamayla dörder okuma standartlar için 10 502,5 µV ve 10 504,0 µV (1000,4 °C ve 1000,6 °C), bilinmeyen için 36 248 µV verir.
2. Fırın sıcaklığı t_X = t_S + c_S(δV_iS1 + δV_iS2 + δV_R) − (c_S/c_S0)·δt_0S + δt_D + δt_F + δt_r (K.7.2), ortalama 1000,5 °C; u_c(t_X) = 0,641 °C (K.7.19); ±1 °C fırın homojensizliği baskındır.
3. V_X(1000 °C) = V_iX + (1000 °C − t_X)/c_X + δV_iX1 + δV_iX2 + δV_LX + δt_TH/c_X + δV_R − δt_0X/c_X0 + δV_r (K.7.2, K.7.20); c_X = 0,026 °C/µV ve c_X0 = 0,038 °C/µV.
4. u_c = 25,8 µV; referans bağlantı 0 °C'de iken 1000 °C'de V_X = 36 230 µV ± 52 µV (k = 2).
5. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- V_X = (36229 ± 52) µV; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Notlar:**

- Araştırma günlüğü F59: raporlanan 36 230 µV EMK ölçeklemeyle, 36 248·1000/1000,5, elde edilir; belirsizlik bütçesi ise termoçiftin 38,5 µV/°C duyarlılığını kullanır, bu da 36 248 − 0,5·38,46 = 36 228,8 µV verir. Basılı u_c = 25,9 µV ve U = 51,9 µV yuvarlanmış duyarlılıklardan (38,5 ve 26,3) çıkar; tam değerler 25,84 µV ve 51,7 µV verir.

**Çalıştırma:** `uncertainty run library/temperature/m3003-k7-thermocouple-emf.json --lang tr`

### 1000 °C'de termoçift kalibrasyonunda fırın sıcaklığı (UKAS M3003 K.7, birinci bütçe)

`m3003-k7-thermocouple-furnace` — UKAS M3003 ed. 6 (2024), Appendix K.7 (K.7.19)

K.7'nin birinci kısmı: iki referans termoçiftten sıcak bağlantılardaki sıcaklık.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. N tipi bir termoçift 1000 °C'de iki R tipi referans termoçiftle karşılaştırılır; ters çevrilmiş kutuplamayla dörder okuma standartlar için 10 502,5 µV ve 10 504,0 µV (1000,4 °C ve 1000,6 °C), bilinmeyen için 36 248 µV verir.
2. Fırın sıcaklığı t_X = t_S + c_S(δV_iS1 + δV_iS2 + δV_R) − (c_S/c_S0)·δt_0S + δt_D + δt_F + δt_r (K.7.2), ortalama 1000,5 °C; u_c(t_X) = 0,641 °C (K.7.19); ±1 °C fırın homojensizliği baskındır.
3. M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.

**Sonuç:**

- t_X = (1000,5 ± 1,3) °C; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95,45 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/temperature/m3003-k7-thermocouple-furnace.json --lang tr`

### Günlük en yüksek sıcaklığın aylık ortalaması (NIST TN 1900 E2)

`nist-tn1900-e2-surface-temperature` — NIST Technical Note 1900, Example E2, Exhibit 2

TN 1900 E2: 22 günlük okuma için t_i = τ + E_i gözlem denklemi; 21 serbestlik dereceli A tipi değerlendirme.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Ölçülen büyüklük τ, Mayıs 2012'deki gerçek günlük en yüksek sıcaklıkların ortalamasıdır; her okuma ondan, ortalaması 0 olan bağımsız normal dağılımlı bir E_i hatası kadar sapar (TN 1900'ün gözlem denklemi).
2. Tahmin, m = 22 okumanın ortalaması olan 25,59 °C'dir; s = 4,09 °C olduğundan u(τ) = s/√22 = 0,872 °C ve ν = 21'dir (GUM 4.2.3).
3. %95 için kapsam faktörü t₀,₉₇₅(21) = 2,08 t dağılımı yüzdeliğidir ve aralık [23,8; 27,4] °C olur. Monte Carlo yöntemi τ'yu JCGM 101 6.4.9'un ölçeklenmiş ve kaydırılmış t₂₁ dağılımından çeker ve aynı aralığı verir.

**Sonuç:**

- tau = (25,6 ± 1,8) °C; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 2,08 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Notlar:**

- Monte Carlo standart sapması s/√m · √(ν/(ν − 2)) = 0,917 °C'dir ve u = 0,872 °C'den büyüktür, çünkü JCGM 101 6.4.9'un t dağılımının ölçeği s/√m'dir; kapsam aralıkları ise çakışır.
- TN 1900 ayrıca dağılımdan bağımsız Wilcoxon aralığını, [23,6; 27,6] °C, verir; bu bir GUM ya da Monte Carlo sonucu olmadığından burada yeniden üretilmez.

**Çalıştırma:** `uncertainty run library/temperature/nist-tn1900-e2-surface-temperature.json --lang tr`

