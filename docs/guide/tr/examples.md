# Çözümlü örnekler

Bu bölüm örnek kütüphanesinden `uncertainty catalogue library --lang tr` komutuyla üretilmiştir; her sonuç, proje dosyasında saklanan girdilerden program tarafından yeniden hesaplanır ve bu dosya güncel değilse test takımı başarısız olur. Monte Carlo sonuçları burada listelenmemiştir: tam bütçe, grafikler ve yayımlanmış referans değerlerle karşılaştırma ile birlikte almak için projeyi çalıştırın.

- Kimya (51)
- Elektrik ve RF (15)
- Akış ve hacim (4)
- Genel (yöntem doğrulama) (15)
- Uzunluk (8)
- Kütle (4)
- Fiziksel özellikler (6)
- Sıcaklık (4)

## Kimya

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

- Tablo J.5'in 1. ve 2. düzeyleri Tablo J.2'den tam olarak yeniden üretilir; 3., 4. ve 5. düzeyler ile 6. düzeydeki s_R, belirtilen aykırı değer çıkarmalarıyla basılı verilerden yeniden üretilemez (araştırma günlüğü F40). Tablo J.5'teki 'Average (%)' sütunu fazladan bir 10⁻³ çarpanı taşır.

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

