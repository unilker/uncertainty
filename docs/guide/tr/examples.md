# Çözümlü örnekler

Bu bölüm örnek kütüphanesinden `uncertainty catalogue library --lang tr` komutuyla üretilmiştir; her sonuç, proje dosyasında saklanan girdilerden program tarafından yeniden hesaplanır ve bu dosya güncel değilse test takımı başarısız olur. Monte Carlo sonuçları burada listelenmemiştir: tam bütçe, grafikler ve yayımlanmış referans değerlerle karşılaştırma ile birlikte almak için projeyi çalıştırın.

- Kimya (31)
- Elektrik ve RF (10)
- Akış ve hacim (3)
- Genel (yöntem doğrulama) (3)
- Uzunluk (7)
- Kütle (3)
- Sıcaklık (3)

## Kimya

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

### Karşılaştırma kaybı, x₁ = 0,0

`jcgm101-9.4-comparison-loss-x000` — JCGM 101:2008, 9.4, Table 8

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. Buradaki GUM sonucu ikinci mertebe terimleri içerir; bu kuadratik model için varyansı tam olarak verir, ancak simetrik dağılım varsayımına dayalı %95 aralığı yine de uygun değildir.

**Sonuç:**

- dY = (0 ± 640) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 12,7 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x000.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,0, r = 0,9

`jcgm101-9.4-comparison-loss-x000-correlated` — JCGM 101:2008, 9.4, Table 9

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. r(X₁, X₂) = 0,9 iken girdiler iki değişkenli normal dağılımdan (Cholesky çarpanı) üretilir. c₂ = 0 olduğundan birinci mertebe GUM sonucu korelasyondan etkilenmez; korelasyonlu girdiler için ikinci mertebe terimler tanımlı değildir (GUM 5.1.2 notu), bu nedenle etkiyi yalnızca MCM yakalar.

**Sonuç:**

- dY = (0,0 ± 0,0) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 12,7 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x000-correlated.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,01

`jcgm101-9.4-comparison-loss-x010` — JCGM 101:2008, 9.4, Table 8

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. Buradaki GUM sonucu ikinci mertebe terimleri içerir; bu kuadratik model için varyansı tam olarak verir, ancak simetrik dağılım varsayımına dayalı %95 aralığı yine de uygun değildir.

**Sonuç:**

- dY = (100 ± 220) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x010.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,01, r = 0,9

`jcgm101-9.4-comparison-loss-x010-correlated` — JCGM 101:2008, 9.4, Table 9

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. r(X₁, X₂) = 0,9 iken girdiler iki değişkenli normal dağılımdan (Cholesky çarpanı) üretilir. c₂ = 0 olduğundan birinci mertebe GUM sonucu korelasyondan etkilenmez; korelasyonlu girdiler için ikinci mertebe terimler tanımlı değildir (GUM 5.1.2 notu), bu nedenle etkiyi yalnızca MCM yakalar.

**Sonuç:**

- dY = (100 ± 200) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x010-correlated.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,05

`jcgm101-9.4-comparison-loss-x050` — JCGM 101:2008, 9.4, Table 8

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. Buradaki GUM sonucu ikinci mertebe terimleri içerir; bu kuadratik model için varyansı tam olarak verir, ancak simetrik dağılım varsayımına dayalı %95 aralığı yine de uygun değildir.

**Sonuç:**

- dY = (2500 ± 980) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x050.json --lang tr`

### Karşılaştırma kaybı, x₁ = 0,05, r = 0,9

`jcgm101-9.4-comparison-loss-x050-correlated` — JCGM 101:2008, 9.4, Table 9

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir.
2. ∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve belirgin biçimde asimetriktir.
3. Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır.
4. r(X₁, X₂) = 0,9 iken girdiler iki değişkenli normal dağılımdan (Cholesky çarpanı) üretilir. c₂ = 0 olduğundan birinci mertebe GUM sonucu korelasyondan etkilenmez; korelasyonlu girdiler için ikinci mertebe terimler tanımlı değildir (GUM 5.1.2 notu), bu nedenle etkiyi yalnızca MCM yakalar.

**Sonuç:**

- dY = (2500 ± 980) 10⁻⁶; genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/electrical/jcgm101-9.4-comparison-loss-x050-correlated.json --lang tr`

## Akış ve hacim

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

## Genel (yöntem doğrulama)

### Normal dağılımlı girdilerle toplamsal model

`jcgm101-9.2.2-additive-gaussian` — JCGM 101:2008, 9.2.2

JCGM 101 9.2.2: her biri N(0, 1) olan dört bağımsız girdi.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Y = X1 + X2 + X3 + X4 modeli doğrusaldır ve tüm duyarlılık katsayıları 1'dir; bu nedenle GUM belirsizlik yayılım yasası u(y)² = Σ u(xᵢ)² sonucunu tam olarak verir.
2. Çıktı tam olarak normal dağılımlıdır, N(0, 2²); GUM %95 aralığı ±1,96 × 2 = ±3,92 tamdır, Monte Carlo yöntemi bunu yeniden üretir ve GUM sonucu doğrulanır.

**Sonuç:**

- Y = (0,0 ± 3,9); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/general/jcgm101-9.2.2-additive-gaussian.json --lang tr`

### Dikdörtgen dağılımlı girdilerle toplamsal model

`jcgm101-9.2.3-additive-rectangular` — JCGM 101:2008, 9.2.3

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

`jcgm101-9.2.4-additive-dominant-rectangular` — JCGM 101:2008, 9.2.4

JCGM 101 9.2.4: 9.2.3 ile aynı, ancak X4'ün standart sapması 10'dur.

**Yaklaşım:** GUM belirsizlik yaklaşımı (JCGM 100) ve Monte Carlo yöntemi (JCGM 101)

**Adımlar:**

1. Y = X1 + X2 + X3 + X4 modeli doğrusaldır ve tüm duyarlılık katsayıları 1'dir; bu nedenle GUM belirsizlik yayılım yasası u(y)² = Σ u(xᵢ)² sonucunu tam olarak verir.
2. Çıktı neredeyse dikdörtgen dağılımlıdır: %95 aralığı yaklaşık ±17,0 iken GUM aralığı (normal varsayımı) ±19,9'dur; bir girdi baskın olduğunda merkezi limit teoremi geçerli olmaz.
3. GUM sonucu n_dig = 2 için (δ = 0,5) doğrulanmaz, n_dig = 1 için (δ = 5) doğrulanırdı.

**Sonuç:**

- Y = (0 ± 20); genişletilmiş belirsizlik, birleşik standart belirsizliğin k = 1,96 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam olasılığına karşılık gelir.

**Çalıştırma:** `uncertainty run library/general/jcgm101-9.2.4-additive-dominant-rectangular.json --lang tr`

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

`jcgm101-9.5-gauge-block-approximate` — JCGM 101:2008, 9.5, Tables 10 and 11; JCGM 100:2008, H.1

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

`jcgm101-9.5-gauge-block-nonlinear` — JCGM 101:2008, 9.5, Tables 10 and 11; JCGM 100:2008, H.1

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

## Kütle

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

`jcgm101-9.3-mass-calibration` — JCGM 101:2008, 9.3, Tables 6 and 7

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

`jcgm101-9.3-mass-calibration-higher-order` — JCGM 101:2008, 9.3, Table 6; JCGM 100:2008, 5.1.2

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

