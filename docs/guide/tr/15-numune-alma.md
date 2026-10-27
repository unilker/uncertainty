# 15. Numune almadan kaynaklanan belirsizlik (Eurachem UfS:2019)

Ölçülen büyüklük laboratuvar numunesindeki değil de bir *numune alma hedefindeki* (bir parti, bir tarla, bir
kuyu) derişim olduğunda, numune alma sonucun belirsizliğine katkıda bulunur; çoğu zaman analizden daha fazla.
ISO/IEC 17025:2017 7.6.1, "numune almadan kaynaklananlar dahil" tüm önemli katkıların hesaba katılmasını ister.
`sampling` bölümü, Eurachem/EUROLAB/CITAC/Nordtest/AMC kılavuzu *Measurement uncertainty arising from sampling*,
2. baskı (2019) ve düzeltme listesine dayanır. Kılavuzun A1–A6 örnekleriyle doğrulanmıştır (kütüphane
projeleri `ufs-*`).

Kılavuz iki yaklaşım tanımlar:

- **ampirik** ("top-down") yaklaşım: hedeflerden tekrarlı numune alma ve analiz; numune alma ve analiz
  varyanslarını ayıran bir varyans analiziyle (15.1–15.3);
- **modelleme** ("bottom-up") yaklaşımı: tek tek etkilerin bütçesi. Bu etkiler ya Gy numune alma kuramının temel
  numune alma hatasıdır (15.4) ya da her numune alma etkisi için düzeltme faktörleri içeren bir ölçüm modelidir
  (15.5).

## 15.1 Çift numune yöntemi (Bölüm 9.4.2)

En az sekiz numune alma hedefinden, numune alma protokolünün bağımsız uygulamalarıyla iki numune alınır ve her
numune iki kez analiz edilir. Bu dengeli tasarım her hedef için dört sonuç verir: S1A1, S1A2, S2A1 ve S2A2
(`duplicates`). Kılavuzun modeli (Eşitlik 1 ve 2):

x = X_gerçek + ε_hedef + ε_numune + ε_analiz,  s_ölçüm² = s_numune² + s_analiz².

**Klasik iç içe varyans analizi** (`"method": "anova"`, varsayılan) üç varyansı düzeylerin kareler
ortalamalarından tahmin eder:

| Düzey | Serbestlik derecesi (n hedef) | Tahmin |
|---|---|---|
| analiz | 2n | s_analiz² = KO_analiz |
| numune alma | n | s_numune² = (KO_numune − KO_analiz)/2 |
| hedefler arası | n − 1 | s_T² = (KO_hedef − KO_numune)/4 |

Negatif bir tahmin, varyansın altındaki düzeyin varyansına göre küçük olduğu anlamına gelir. Geleneksel olarak
sıfır alınır ve rapor bunu belirtir (Örnek A4, 4 g deney numuneleri). Bağıl belirsizlikler tüm sonuçların
ortalaması x̄'a göredir. Bağıl genişletilmiş belirsizlik U′ = k·100·s/x̄'dır, k = 2 (Eşitlik 4). Rapor ayrıca
her düzeyin toplam varyanstaki payını verir. Kılavuz bunu amaca uygunluğu değerlendirmede kullanır (Bölüm 16.2;
ideal olarak ölçümün payı en çok %20'dir).

Dengeli olmayan tasarımlar (Ek D2), örneğin ikinci numunenin yalnızca bir kez analiz edildiği tasarım, `targets`
olarak girilir: her hedef için numunelerin listesi, her numune için analizleri. Program bu durumda dengesiz iç
içe tasarım için momentler yöntemini kullanır. Beklenen kareler ortalamalarının katsayıları (dengeli durumda 2,
2 ve 4) raporda gösterilir.

**Robust ANOVA.** Kılavuzun ana sonuçları RANOVA ve RANOVA2 robust ANOVA programlarından gelir. Bunların
algoritması kılavuzda yayımlanmamıştır. Huber H15'e dayanan yeniden kurgular Örnek A1 ve A2'nin robust
*ortalamasını* verir, ancak robust standart sapmaları vermez. Bu nedenle robust yöntem uygulanmamıştır
(araştırma günlüğü, F63). Kılavuzun yanında yazdırdığı klasik sonuçlar birebir yeniden üretilir. Verileri aykırı
hedefler için inceleyin (Örnek A1, C hedefi): bunlar klasik tahminleri büyütür.

## 15.2 Aralık yöntemi (Örnek A3)

Kılavuz dengeli tasarım için basit aralık hesaplarını da kullanır (`"method": "range"`):

- s_A = R̄_A/d₂; R̄_A, |S1A1 − S1A2| ve |S2A1 − S2A2| analiz aralıklarının ortalamasıdır;
- s_S+A = R̄_S/d₂; R̄_S, numune ortalamalarının ortalama aralığı |x̄_S1 − x̄_S2|'dir; ardından
  s_S = √(s_S+A² − s_A²/2);
- s_T = √(s_T+S+A² − s_S+A²/2); s_T+S+A, hedef ortalamalarının standart sapmasıdır.

Burada iki değerin aralığı için d₂ = 2/√π = 1,128'dir. Basılı Tablo A3.7'de hedefler arası standart sapmada bir
hata vardır; düzeltme listesi (sürüm 1.1) bunu düzeltir. Program düzeltilmiş değerleri yeniden üretir.

## 15.3 Log-normal değerler ve belirsizlik faktörü (Bölüm 9.5.3, Örnek A2)

Değerler çarpık olduğunda, özellikle bağıl standart belirsizlik yaklaşık %20'yi aştığında, belirsizlik bir
faktör olarak daha iyi ifade edilir. `"log": true` ile program klasik ANOVA'yı ln x üzerinde de çalıştırır.
Doğal logaritmaların standart sapmalarını s_G ve numune alma, analiz ve ölçüm için genişletilmiş belirsizlik
faktörlerini verir:

F_U = exp(k·s_G)  (Eşitlik 5).

Aralık x/F_U ile x·F_U arasıdır ve sonuç etrafında simetrik değildir. Örnek A2'de 300 mg/kg'lık bir sonuç
F_U = 2,62 ile 115–784 mg/kg aralığını verir; U′ = %83,6 ile aralık 49–551 mg/kg olurdu. Rapor ayrıca bağıl
standart belirsizliği, u′ = √(exp(s_G²) − 1), verir. u′ 0,2'yi aşınca uyarır, çünkü u′ bu durumda doğrudan 2 ile
çarpılmamalıdır.

## 15.4 Temel numune alma hatası (Gy, Bölüm 10.2, Örnek A5)

Doğru alınan taneli malzemelerde her adımın en küçük numune alma hatası temel numune alma hatasıdır (`gy`).
Bağıl varyansı

s_r² = C·d³·(1/M_S − 1/M_L)  (Eşitlik 6), C = f·g·β·c.

M_S ve M_L numunenin ve partinin kütleleri, d tanecik boyutunun %95 üst sınırı, f şekil faktörü (küresel
tanecikler için 0,5), g boyut dağılımı faktörü (geniş dağılım 0,25, tek boyut 1) ve β serbestleşme faktörüdür
(serbest tanecikler için 1). Bileşim faktörü (Eşitlik 8):

c = (1 − a_L/α)²/(a_L/α)·ρ_c + (1 − a_L/α)·ρ_m.

a_L partideki, α kritik taneciklerdeki derişimdir; ρ_c ve ρ_m kritik ve matris taneciklerinin yoğunluklarıdır.
Adımlar (birincil numune, alt numune alma, deney porsiyonu) ve analitik bağıl standart sapma varyansların
toplamıyla birleştirilir. Rapor en zayıf halkanın hangi adım olduğunu gösterir. Örnek A5'te bu, 2 g'lık deney
porsiyonudur (toplam %14,3'ün içinde s_r = %13).

## 15.5 Ölçüm modeliyle modelleme (Örnek A6)

Tek tek numune alma etkileri keşif deneyleriyle değerlendirildiğinde, bunlar bir ölçüm modelinde toplanır:
x_saha = x̄·f_konum·f_strateji·f_derinlik·f_hazırlık·f_kuru·f_analiz; düzeltme faktörlerinin nominal değeri
1'dir. Bu, sıradan bir `model` projesidir (bölüm 3); kütüphane projeleri `ufs-a6-cadmium-soil` ve
`ufs-a6-phosphorus-soil` buna örnektir. Konumlar arası terim karelerin kompozit numunelerinden gelen A tipi bir
girdi, derinlik etkisi dikdörtgen bir terim, analiz ise kendi bileşenleridir.

## 15.6 Numune almanın kalite kontrolü (Bölüm 13.2, Örnek A4)

Valide edildikten sonra numune alma belirsizliği, hedeflerin bir kısmından çift numune alınarak izlenir.
Çiftlerin bağıl farkı D % = |x₁ − x₂|/x̄·100 bir aralık grafiğine işlenir (bölüm 14,
`"chart": "relative_range"`). Sınırlar valide edilmiş u = √(u_numune² + u_analiz²) değerinden gelir: merkez
çizgisi 1,128·u, uyarı sınırı 2,83·u, eylem sınırı 3,69·u (`ufs-a4-vitamin-a-qc-chart`).

## 15.7 Değerlendirmenin kapsamadıkları

Ne çift numune yöntemi ne de temel numune alma hatası **numune alma sapmasını** içerir. Kılavuz bunu birden çok
numune alıcı veya protokolle (Tablo 5), bir referans numune alma hedefiyle ya da uzman yargısıyla ele alır.
Analitik sapma Örnek A2'deki gibi (Bölüm 6.4) eklenebilir, örneğin bir model projesinde. Belirsizlik,
validasyondakilere benzer hedefler için geçerlidir. Ölçülen büyüklük bütün bir sahanın ortalaması olduğunda,
geçerli olan o ortalamanın belirsizliğidir (s_toplam/√n) (Örnek A2, Bölüm 7).
