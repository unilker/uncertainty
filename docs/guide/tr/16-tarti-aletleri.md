# 16. Tartı aletlerinin kalibrasyonu (EURAMET cg-18)

EURAMET Kalibrasyon Rehberi No. 18 v4.0 (2015), analitik terazilerden köprü kantarlara kadar otomatik olmayan
tartı aletlerinin kalibrasyonu için başvuru belgesidir. Program, kalibrasyon sertifikasının iki bölümünü kapsar:

- her deney yükündeki **gösterge hatası** ve belirsizliği, bir ölçüm modeli olarak (16.1–16.3);
- tüm deney noktalarına uydurulan **hata yaklaşım fonksiyonu** E(I); bu fonksiyon herhangi bir okumadaki hatayı
  ve belirsizliğini verir (Ek C; `error_function` bölümü, 16.4).

Ek H'nin dört örneğinin tamamı kütüphane projesidir: `cg18-h1-*` (220 g / 0,1 mg), `cg18-h2-*` (60 kg çok
aralıklı), `cg18-h3-*` (ikame yükleriyle 30 t köprü kantar) ve `cg18-h4-*` (400 g'lık terazinin hata fonksiyonu).

## 16.1 Model olarak gösterge hatası (7.1)

E = I − m_ref (7.1-1). Kılavuz bütçesinin her terimi modelin bir girdi büyüklüğüdür:

| Terim | Girdi | Formül |
|---|---|---|
| sıfırda ve yükte yuvarlama | dikdörtgen, yarı genişlik d₀/2 ve d_L/2 | 7.1.1-2a, -3a |
| tekrarlanabilirlik | tekrarlanabilirlik deneyinden s, ν = n − 1 ile A tipi (tek okuma) | 7.1.1-5 |
| eksantriklik | dikdörtgen, yarı genişlik ΔI_ecc,maks·L/(2L_ecc) | 7.1.1-10 |
| sürünme ve histerezis | dikdörtgen, yarı genişlik E₀·L/Max | 7.4.4-7 |
| referans kütle | nominal değerindeki ağırlıklar: dikdörtgen, yarı genişlik mpe; veya sertifika değeri ve U/k | 7.1.2-1 … -3 |
| ağırlıkların sürüklenmesi | dikdörtgen, yarı genişlik D = k_D·U veya D = mpe | 7.1.2-10, -11 |
| hava kaldırma kuvveti | modelde −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) düzeltmesi (seçenek 2) veya 7.1.2-5c, -5d, -5e sınırları | 7.1.2-4 … -5e |

Çok aralıklı bir alette (Örnek H2) d_L, yükü içeren aralığın ölçek aralığıdır ve tekrarlanabilirlik aynı
aralıktaki deneyinkidir. Kapsam faktörü ν_eff'ten (Welch–Satterthwaite, B3-1) ve %95,45 için t dağılımından
gelir. Kılavuz k'yı GUM Tablo G.2'den bir alt tablo satırında okur; bu, tam kantilden biraz büyük k verebilir
(örneğin ν = 90'da 2,03 yerine 2,05). Projeler bu farkı not eder.

## 16.2 Kaldırma kuvveti: seçenek 1 ve seçenek 2

Hava yoğunluğu hakkında bilgi yoksa (seçenek 1) kaldırma kuvveti bir sınırdır. Alet kalibrasyondan hemen önce
ayarlanmışsa 7.1.2-5c, ayarlanmamışsa 7.1.2-5d, sahadaki sıcaklık değişimi biliniyorsa 7.1.2-5e uygulanır.
Hava yoğunluğu kalibrasyon sırasında ölçülmüşse (seçenek 2) düzeltme modelin parçasıdır ve belirsizliği
(7.1.2-5a) yayılımdan çıkar. Örnek H1 ve H2 her ikisini de gösterir. H2'de (d = 10 g) düzeltme ihmal
edilebilir, H1'de (d = 0,1 mg) edilemez.

## 16.3 İkame yükleri (4.3.3, 7.1.2-15, Örnek H3)

Büyük aletler yükün bir kısmı için standart ağırlıklarla, geri kalanı için ikame yükleriyle denenir. İkame yükü,
alet ağırlıklarla olan göstergenin aynısını gösterene kadar ayarlanır: L_ikame1 = m_ref + I(L_ikame1) − I(m_ref).
Bu şekilde kullanılan her gösterge kendi yuvarlama, tekrarlanabilirlik, eksantriklik ve sürünme terimlerini
getirir. Modelde bunlar ayrı girdilerdir ve aynı standart ağırlıklar her adımda yer alır (tam korelasyonlu).
Bu, u(L_ikame) için 7.1.2-15 formülünü yeniden üretir. Kılavuzun ν_eff'i yalnızca yükteki göstergenin
tekrarlanabilirliğini sayar. Model her tekrarlanabilirlik terimine ν = n − 1 verir ve bu nedenle daha küçük bir
ν_eff bulur (araştırma günlüğü F69).

## 16.4 Hata yaklaşım fonksiyonu (`error_function`, Ek C2.2)

Deney noktaları I_j'deki hatalar E_j bir polinomla uydurulur. Varsayılan, sıfırdan geçen doğru E = a₁I'dır
(C2.2-16); sabit terimli ve daha yüksek dereceli polinomlar da kullanılabilir (C2.2-3). Hataların kovaryans
matrisi

U(e) = U(m_ref) + U(I_Cal) + U(mod)  (C2.2-3a).

Burada U(m_ref) = s sᵀ tam korelasyonlu referans kütleler içindir (C2.2-3b; korelasyonsuz olanlar için
`"reference_correlation": "none"`), U(I_Cal) u²(I_j)'lerin köşegenidir ve U(mod) = s_m²·I bir model terimidir
(C2.2-3c). Genelleştirilmiş en küçük kareler çözümü, P = U(e)⁻¹ ile â = (XᵀPX)⁻¹XᵀPe, U(â) = (XᵀPX)⁻¹,
artıklar v = Xâ − e ve ν = n − n_par ile χ²_göz = vᵀPv verir (C2.2-6 … -9).

İki uyum iyiliği ölçütü raporlanır:

- **C2.2-2a**: χ²_göz ≤ ν: fonksiyon verilerle tutarlıdır;
- **C2.2-2b**: her artık kendi genişletilmiş belirsizliği içindedir, |v_j| < k·u(E_yakl,j); bu çok daha
  kısıtlayıcıdır.

Bir ölçüt sağlanmazsa kılavuz s_m'yi sağlanana kadar büyütür. Rapor her ölçüt için en küçük s_m'yi verir
(ikiye bölme yöntemiyle bulunur); böylece seçim belgelenebilir. Örnek H4'te veriler C2.2-2a için s_m = 0,044 mg,
C2.2-2b için 0,22 mg gerektirir; kılavuz 0,05 ve 0,25 mg kullanır. Daha büyük s_m, yaklaşık hatanın daha büyük
belirsizliği demektir: 400 g'da U(E_yakl), s_m = 0,05 mg ile 0,190 mg, 0,25 mg ile 0,334 mg'dır.

Herhangi bir okuma R'deki (`readings`) yaklaşık hata E_yakl(R) = rᵀâ'dır;
u²(E_yakl) = (f′(R))²u²(R) + rᵀU(â)r (C2.2-12, -13). Noktalar ya hatalarıyla (`error`) ya da referans kütleyle
(`reference`) verilir; `error_factor`, I − m_ref'i hataların birimine çevirir (g'dan mg'a 1000).
