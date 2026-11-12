# 13. Örneklerle öğrenmek

Kütüphanedeki 284 örneğin her biri yayımlanmış bir kılavuzdan, standarttan ya da makaleden alınmıştır ve program her değişiklikte onları yeniden hesaplar. Aşağıda örnekler zorluk düzeyine göre gruplanmıştır. Her grubun başında o grubun ne öğrettiği yazar; bir örneğin adına tıklayınca örnek açılır. Açılan örneğin **Genel** sekmesinde, örneğin adım adım açıklaması vardır.

> **İpucu:** Yeni başlıyorsanız Başlangıç düzeyindeki ilk gruplarla başlayın, her örnekte önce **Hesapla** düğmesine basın, sonra sonuçları açıklamayla karşılaştırın.

<a id="level-1"></a>

## Düzey 1: Başlangıç

### Bir tozu tartmak

Çok basit bir bütçe: tozun kütlesi, dolu kabın okumasından boş kabın okumasının çıkarılmasıyla ve terazinin kayması için küçük bir düzeltmeyle bulunur. İki okumanın belirsizliği aynıdır; birlikte birinin iki katını değil, √2 katını verirler. Model doğrusal olduğu için GUM ve Monte Carlo aynı sonucu verir.

- [Tartımla toz kütlesi (NIST TN 1900 E1)](examples.md#nist-tn1900-e1-weighing)

### Girdileri toplamak: normal ve dikdörtgen

Dört girdi basitçe toplanır. Üç türev yalnızca girdi dağılımlarının biçiminde ayrılır. GUM sonucunu Monte Carlo ile karşılaştırın: tek bir dikdörtgen girdi baskın olduğunda sonuç artık normal değildir ve %95 aralığı 2u'dan dardır. Bu, k = 2'nin doğru olmadığı klasik durumdur.

- [Normal dağılımlı girdilerle toplamsal model](examples.md#jcgm101-9.2.2-additive-gaussian)
- [Dikdörtgen dağılımlı girdilerle toplamsal model](examples.md#jcgm101-9.2.3-additive-rectangular)
- [Baskın dikdörtgen girdili toplamsal model](examples.md#jcgm101-9.2.4-additive-dominant-rectangular)

### Bir ağırlığın kalibrasyonu

Bir 10 kg ağırlık bir referans ağırlıkla karşılaştırılır. Tipik bir kalibrasyon bütçesini gösterir: standardın sertifikası (U ve k), son kalibrasyondan beri sürüklenme, komparatör, hava kaldırma kuvveti ve tekrarlı karşılaştırmalar. En büyük katkı standardın sertifikasıdır.

- [10 kg'lık bir ağırlığın kalibrasyonu](examples.md#ea402-s2-weight-10kg)

### UKAS M3003'te bir 10 kg ağırlık

EA-4/02'dekiyle aynı türden bir ağırlık kalibrasyonu, İngiliz kılavuzundaki biçimiyle. İkisini karşılaştırmak, farklı laboratuvarların bileşenler ve dağılımlar için biraz farklı seçimler yaptığını ve sonucun yine de benzer çıktığını gösterir.

- [OIML M1 sınıfı 10 kg ağırlık (UKAS M3003 K.4)](examples.md#m3003-k4-weight-10kg)

### Bir kumpasın kalibrasyonu

Bir mastar blok bir kumpasla ölçülür. Bileşenlerin çoğu dikdörtgendir: skalanın çözünürlüğü, sıcaklık farkı, çenelerin düzlüğü. Bir skaladaki okumanın bir belirsizliğe nasıl dönüştüğünün iyi bir örneği.

- [Kumpasın 150 mm'de kalibrasyonu](examples.md#ea402-s10-vernier-calliper)

### Bir standart direncin kalibrasyonu

Bir 10 kΩ direnç bir referans dirençle karşılaştırılır. Model bir çarpımdır, bu yüzden duyarlılık katsayıları 1 değildir; bütçenin her bağıl etkiyi nasıl ohm'a çevirdiğine bakın.

- [10 kΩ'luk standart direncin kalibrasyonu](examples.md#ea402-s3-resistor-10kohm)

### Bir dijital multimetrenin kalibrasyonu

Bir el multimetresi 100 V'ta bir kalibratöre karşı denetlenir. Göstergenin çözünürlüğü çoğu zaman en büyük terimdir: tek ondalıkla gösterilen bir değer yarım basamak içinde herhangi bir yerde olabilir.

- [El tipi sayısal multimetrenin 100 V'ta kalibrasyonu](examples.md#ea402-s9-dmm-100v)

### Bir mastar bloğun kalibrasyonu

Bir 50 mm mastar blok bir referans blokla karşılaştırılır. Sıcaklık önemlidir: sıcaklıktaki ya da genleşme katsayısındaki küçük bir fark boyu değiştirir; bütçede ısıl terimlerin olmasının nedeni budur.

- [50 mm'lik mastar bloğunun kalibrasyonu](examples.md#ea402-s4-gauge-block-50mm)

### Bir sıcaklık blok kalibratörü

180 °C'deki bir kuru bloğun sıcaklığı bir referans termometreyle ölçülür. Bütçe bloğun kendi etkilerini listeler: eksenel ve radyal homojensizlik, kararlılık, yükleme.

- [Sıcaklık blok kalibratörünün 180 °C'de kalibrasyonu](examples.md#ea402-s11-block-calibrator-180c)

### Bir su sayacının kalibrasyonu

Üç ilişkili proje: referans hacim, bir koşunun bağıl hatası ve üç koşunun ortalama hatası. Uzun bir hesabın her biri kendi bütçesine sahip adımlara nasıl bölünebileceğini gösterirler.

- [Su sayacı kalibrasyonu: tek çalışmanın bağıl hatası](examples.md#ea402-s12-water-meter-error)
- [Su sayacı kalibrasyonu: üç çalışmanın ortalaması](examples.md#ea402-s12-water-meter-mean-error)
- [Su sayacı kalibrasyonu: referans hacim](examples.md#ea402-s12-water-meter-volume)

### UKAS M3003'te bir direnç

Gerilimler karşılaştırılarak kalibre edilen bir 10 kΩ direnç. Kaynakları, değerlerini, bölenleri (√3, 2, 1) ve sonuçtaki standart belirsizlikleri içeren bir bütçe tablosunun açık bir örneği.

- [Gerilim karşılaştırmasıyla 10 kΩ direnç (UKAS M3003 K.1)](examples.md#m3003-k1-resistor-10kohm)

### 200 g'da bir tartı aleti

Bir tartı aleti 200 g'lık bir ağırlıkla denetlenir. Başlıca terimler tekrarlanabilirlik, çözünürlük ve ağırlığın sertifikasıdır.

- [200 g'de 205 g kapasiteli tartı aleti (UKAS M3003 K.5)](examples.md#m3003-k5-weighing-machine-200g)

### 2 MPa'da bir basınç göstergesi

Bir dijital basınç göstergesi bir ölü ağırlık test cihazıyla karşılaştırılır. Tipik bir basınç kalibrasyonu bütçesi.

- [Ölü ağırlık test cihazına karşı 2 MPa'da dijital basınç göstergesi (UKAS M3003 K.8)](examples.md#m3003-k8-pressure-indicator-2mpa)

### Bir levhanın kalınlığı

Bir alüminyum numunenin kalınlığı beş okumanın ortalamasıdır. A tipi bir değerlendirmeyi (okumaların dağılımı) B tipi terimlerle birleştirir: kalibrasyon, çözünürlük, kayma ve sıcaklık.

- [Alüminyum deney numunesinin kalınlığı (UKAS M3003 K.10)](examples.md#m3003-k10-sheet-thickness)

### Hacimsel cam malzemenin kalibrasyonu

Bir balonjojenin ya da piknometrenin hacmi, aldığı suyun tartılmasıyla bulunur. Suyun ve havanın yoğunluğunun, sıcaklığın ve terazinin sonuca nasıl girdiğini görürsünüz.

- [1000 mL'lik ölçülü balonun gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 1)](examples.md#cg19-ex1-flask-1000ml)
- [100 mL'lik cam piknometrenin gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 2)](examples.md#cg19-ex2-pycnometer-100ml)

### Bir kalibrasyon standardı hazırlamak

Tartılmış bir miktar saf metal bilinen bir hacimde çözülerek bir kadmiyum çözeltisi hazırlanır. Eurachem kılavuzunun klasik ilk örneği: model bir çarpım ve oran olduğu için saflık, kütle ve hacim bağıl belirsizlikler olarak birleşir.

- [Hacim alt modelli kadmiyum standardı](examples.md#quam-a1-cadmium-standard-volume)
- [Kadmiyum kalibrasyon standardının hazırlanması](examples.md#quam-a1-cadmium-standard)

### Referans malzemeden ya da yeterlilik testlerinden sapma

Belirsizliğin sistematik kısmını bulmanın yukarıdan aşağı yolu: sertifikalı bir referans malzemeyi ölçün ya da yeterlilik testlerindeki sonuçlarınıza bakın ve farkları u(sapma)'ya dönüştürün. Tek CRM, birkaç CRM, yeterlilik testi ve geri kazanım türevlerini karşılaştırın.

- [Tek sertifikalı referans malzemeden sapma](examples.md#nordtest-6.1-one-crm)
- [Birden fazla sertifikalı referans malzemeden sapma](examples.md#nordtest-6.1-three-crms)
- [Yeterlilik testlerinden sapma](examples.md#nordtest-6.2-proficiency-tests)
- [Geri kazanım deneylerinden sapma](examples.md#nordtest-6.3-recovery)

### Çift ölçümlerden kesinlik

Numuneleri çift analiz etmek tekrarlanabilirliği verir; aylar boyunca ölçülen bir kontrol numunesi laboratuvar içi tekrar üretilebilirliği verir. Bunlar yukarıdan aşağı belirsizliğin rastgele kısmıdır.

- [Amonyum: kontrol numuneleri ve mükerrer ölçümlerden laboratuvar içi tekrar üretilebilirlik](examples.md#nordtest-5.3-ammonium-duplicates)
- [Çözünmüş oksijen: mükerrer ölçümlerden tekrar üretilebilirlik](examples.md#nordtest-5.3-oxygen-duplicates)

### Tek örnekte bütün yukarıdan aşağı bütçe

Sudaki amonyum: kontrol grafiğinden kesinlik ve yeterlilik testlerinden sapma, genişletilmiş belirsizlikte birleştirilir. Çoğu deney laboratuvarının kullandığı tarif budur.

- [Kontrol sınırları ve yeterlilik testlerinden sudaki amonyum](examples.md#nordtest-3-ammonium-pt)

### İlk kontrol grafiği

Sudaki çinkonun altmış kontrol değeri uyarı ve eylem sınırlarıyla çizilir. Bir grafiği okumayı öğrenin: hangi noktalar sorunsuz, hangileri uyarı, hangileri eylem gerektiriyor.

- [Sudaki çinko: 60 kontrol değerinden X-grafiği (Nordtest TR 569 Tablo 1)](examples.md#tr569-table1-zinc)

### Kontrol grafiği sınırlarını belirlemek

Sınırları belirlemenin üç yolu: verilerden (istatistiksel), bir hedeften ya da bir referans değer etrafında. Hedef sınırların neden çoğu zaman daha geniş ve daha kullanışlı olduğunu görmek için türevleri karşılaştırın.

- [X-grafiğinin sınırlarının belirlenmesi, durum 1 (Nordtest TR 569 Bölüm 7)](examples.md#tr569-ch7-case1-statistical)
- [X-grafiğinin sınırlarının belirlenmesi, durum 2 (Nordtest TR 569 Bölüm 7)](examples.md#tr569-ch7-case2-target-mean)
- [X-grafiğinin sınırlarının belirlenmesi, durum 3 (Nordtest TR 569 Bölüm 7)](examples.md#tr569-ch7-case3-target-reference)
- [Paralel tayinler için r%-grafiğinin istatistiksel sınırları (Nordtest TR 569 Bölüm 7)](examples.md#tr569-ch7-range-case1-statistical)
- [Tekrarlanabilirlik sınırından r%-grafiğinin hedef sınırları (Nordtest TR 569 Bölüm 7)](examples.md#tr569-ch7-range-case2-target)

### Bir ürünün uygun olma olasılığı

Ölçülen değer spesifikasyonun içinde, ama ne kadar emin olabiliriz? Bu örnekler, ölçülen değer ve belirsizliğinden uygunluk olasılığını tek ve iki taraflı sınırlar için hesaplar.

- [Bir kabın patlama dayanımı: uygunluk olasılığı (JCGM 106 7.3)](examples.md#jcgm106-7.3-bursting-strength)
- [Zener diyotun kırılma gerilimi: uygunluk olasılığı (JCGM 106 7.3)](examples.md#jcgm106-7.3-zener-diode)
- [Motor yağının kinematik viskozitesi: iki taraflı tolerans (JCGM 106 7.4)](examples.md#jcgm106-7.4-motor-oil-viscosity)

### Yeterlilik testinin atanan değeri

Aynı sonuçlar iki dayanıklı kestiriciyle değerlendirilir: Algoritma A ve medyan. Dayanıklı istatistikler, birkaç aykırı laboratuvarın atanan değeri kaydırmasını önler.

- [İçme suyunda atrazin: Algoritma A ile robust ortalama (ISO 13528 E.3)](examples.md#iso13528-e3-atrazine-algorithm-a)
- [İçme suyunda atrazin: medyan ve nIQR (ISO 13528 E.3)](examples.md#iso13528-e3-atrazine-median)

### Laboratuvarları puanlamak: z, ζ ve E_n

Her katılımcı bir z puanı (sonuç atanan değere yeterince yakın mı?) ve ζ ile E_n puanları (sonuç laboratuvarın beyan ettiği belirsizlikle tutarlı mı?) alır. Standardın tablosu 21 sayısal sonucun hepsi için yeniden üretilir.

- [Hayvan yeminde cıva: z, z′, ζ ve E_n skorları (ISO 13528 E.4)](examples.md#iso13528-e4-mercury-feed)

### Dört derişimin toplamı

Plazma anyon açığı dört iyon derişiminin doğrusal bir birleşimidir. Doğrusal bir modelde GUM ve Monte Carlo tam uyuşur: temelleri anladığınızı denetlemek için iyi bir örnek.

- [Plazma anyon açığı: dört iyon derişiminin doğrusal birleşimi (Possolo & Meija 2022)](examples.md#possolo-meija-2022-plasma-anion-gap)

### Suyun sertliği

Kalsiyum ve magnezyum derişimlerinden toplam sertlik: IUPAC kılavuzundan kısa, alışılmış bir ölçüm modeli; GUM adımlarını alıştırma yapmak için ideal.

- [Ca²⁺ ve Mg²⁺'dan suyun toplam sertliği: geleneksel bir ölçüm modeli (IUPAC 2024, Örnek 3C ve 4A)](examples.md#iupac2024-4a-water-hardness)

### Bir titrasyon

Sodyum klorür derişimi gümüş nitratla titrasyonla bulunur. Hacimler, titrantın derişimi ve dönüm noktası girdilerdir.

- [Arjantometrik titrasyonla (Mohr) NaCl miktar derişimi: klasik model (IUPAC 2024, Örnek 4I)](examples.md#iupac2024-4i-argentometric-titration)

### Atom ağırlıklarından molar kütle

Glukozun bağıl molekül kütlesi, aralık olarak verilen standart atom ağırlıklarından bulunur. Başka bilgi olmayan bir aralık dikdörtgen dağılıma dönüşür.

- [Standart atom ağırlığı aralıklarından glikozun bağıl molekül kütlesi (JCGM GUM-5 Örnek 3)](examples.md#gum5-e3-molar-mass-glucose)

### Radyoaktif bozunma

Bir kaynağın aktivitesi referans tarihinden bu yana geçen süre için düzeltilir. Yarı ömür ve süre üstel bir fonksiyon üzerinden girer.

- [Bozunma için düzeltilmiş ²⁰⁷Bi kaynağının aktivitesi (JCGM GUM-5 Örnek 14)](examples.md#gum5-e14-radioactive-decay-bi207)

### Günlük sıcaklıkların ortalaması

Günlük en yüksek sıcaklıkların aylık ortalaması: saf bir A tipi değerlendirme. Gün sayısı azken t dağılımı kapsam faktörünü belirgin biçimde 2'den büyük yapar.

- [Günlük en yüksek sıcaklığın aylık ortalaması (NIST TN 1900 E2)](examples.md#nist-tn1900-e2-surface-temperature)

<a id="level-2"></a>

## Düzey 2: Orta

### CMC ile raporlama

ILAC P14'e göre bir kalibrasyon sertifikası laboratuvarın kalibrasyon ve ölçüm yeteneğinden (CMC) küçük bir belirsizlik belirtemez. Program raporlarken bu alt sınırı uygular.

- [ILAC P14 5.5 raporlama kuralı: raporlanabilecek en küçük belirsizlik olarak CMC (EA-4/02 S2 kütle, örnek CMC)](examples.md#ilac-p14-cmc-floor-weight-10kg)

### Bir dikdörtgen terim baskın olduğunda

Çözünürlüğü baskın olan bir voltmetre. Sonuç normalden çok dikdörtgene yakın olduğu için UKAS M3003 bu durum için özel bir kapsam faktörü verir.

- [Baskın çözünürlük terimli dijital voltmetre (UKAS M3003 Ek C)](examples.md#m3003-c5-dvm-dominant-rectangular)

### Bir aralık boyunca multimetre spesifikasyonu

1 V aralığında 6½ basamaklı bir multimetre: katkıların bir kısmı okumaya göre bağıldır (ppm), bir kısmı mutlaktır (µV). Örnekler aralığın birkaç noktasındaki belirsizliği ve spesifikasyon baskın olduğundaki kapsam faktörünü gösterir.

- [1 V kademesinde DMM: 0,95 V'taki bütçe (UKAS M3003 L.3)](examples.md#m3003-l3-dmm-1v-range-095v)
- [Tam ölçekte 1 V kademesinde DMM: baskın şartname terimi için kapsam faktörü (UKAS M3003 L.3.20)](examples.md#m3003-l3-dmm-1v-range-1v-dominant)
- [1 V kademesinde DMM: kademe formülünden 500 mV'taki belirsizlik (UKAS M3003 L.3)](examples.md#m3003-l3-dmm-1v-range-500mv)

### Bir sodyum hidroksit çözeltisinin ayarlanması

Tartılmış bir birincil standarda karşı titrasyon. Örnek alt modeller kullanır (kütle, büret hacmi) ve tekrarlanabilirliğin tek bir birleşik terime konabileceğini gösterir.

- [Alt modellerle NaOH ayarlanması](examples.md#quam-a2-naoh-standardisation-submodels)
- [Sodyum hidroksit çözeltisinin ayarlanması](examples.md#quam-a2-naoh-standardisation)

### Hidroklorik asit titrasyonu

Ayarlı NaOH ile titrasyondan HCl derişimi. Önceki örnekteki NaOH belirsizliği buraya taşınır: bir değerlendirmenin sonuçları bir sonrakinin girdileri olur.

- [Hidroklorik asidin asit–baz titrasyonu](examples.md#quam-a3-hcl-titration)

### Ekmekte pestisitler

Aynı cevaba iki yol: yöntemin adımlarından kurulan bir bütçe ve validasyon verilerinden (kesinlik ve geri kazanım) yukarıdan aşağı bir tahmin. Her yaklaşımın hangi bileşenleri yakaladığını karşılaştırın.

- [Ekmekte organofosforlu pestisitler: birleştirme](examples.md#quam-a4-pesticides-combination)
- [Validasyon verilerinden ekmekte organofosforlu pestisitler](examples.md#quam-a4-pesticides-topdown)

### Seramikten salınan kadmiyum

Ampirik bir yöntem: sonuç prosedürle tanımlanır (süre, sıcaklık, asit). Bu koşulların etkileri belirsizlik terimleri olarak eklenir.

- [Seramik kaplardan salınan kadmiyum](examples.md#quam-a5-cadmium-release)

### Hayvan yeminde ham lif

Bir başka ampirik yöntem; bir laboratuvarlar arası çalışmanın kesinliğiyle değerlendirilir: yöntemin tekrar üretilebilirliği belirsizliğin ana kısmıdır.

- [Hayvan yeminde ham lif (ampirik yöntem)](examples.md#quam-a6-crude-fibre)

### Çevre analizleri için yukarıdan aşağı

Atık sudaki BOİ ve sedimentteki PCB, her biri iki kez değerlendirilir: sapma için bir kez CRM ile, bir kez yeterlilik testleriyle. Sapma verisinin seçiminin sonucu nasıl değiştirdiğini görün.

- [Atık suda BOİ: kontrol grafiği ve tek CRM](examples.md#nordtest-8-bod-crm)
- [Atık suda BOİ: kontrol grafiği ve yeterlilik testleri](examples.md#nordtest-8-bod-pt)
- [Sedimentte PCB: kontrol numunesi ve tek CRM](examples.md#nordtest-8-pcb-crm)
- [Sedimentte PCB: kontrol numunesi ve yeterlilik testleri](examples.md#nordtest-8-pcb-pt)

### Laboratuvar içi kesinlik ve geri kazanımdan belirsizlik

2026 Eurachem/CITAC kılavuzu (MUPR) belirsizliği her laboratuvarın sahip olduğu verilerden değerlendirir: birkaç düzeyde kesinlik ve referans malzemelerden ya da katkılanmış numunelerden geri kazanım. Örnekler birleştirilmiş ve derişime bağlı kesinliği ve farklı geri kazanım tasarımlarını kapsar.

- [İçme suyunda nitrat: iki kesinlik aralığı](examples.md#mupr-b1-nitrate)
- [Nitrat: birleştirilmiş kesinlik ve iki geri kazanım düzeyi](examples.md#mupr-b2-pooled-precision)
- [Nitrat: 2 günde 3 tekrarın ortalaması olarak sonuç](examples.md#mupr-b2-replicates-on-two-days)
- [Nitrat: derişimin doğrusal fonksiyonu olarak kesinlik](examples.md#mupr-b3-linear-precision)
- [Sedimentte arsenik: tek CRM](examples.md#mupr-b4-arsenic-one-crm)
- [Arsenik: matrise bağlı geri kazanımlı CRM ve katkılı numuneler](examples.md#mupr-b5-crm-and-spiked-samples)
- [Sedimentte iz element: heterojenlik ve 22 yeterlilik testi](examples.md#mupr-b6-sediment-22-pt)
- [İki CRM ve bir yeterlilik testi numunesinden geri kazanım](examples.md#mupr-ex2-reference-materials)
- [Katkılı nehir sularından geri kazanım](examples.md#mupr-ex3-spiked-samples)

### Uygulamada kontrol grafikleri

Nordtest el kitabının çözümlü örnekleri: X grafikleri, çift ölçümler için R ve r% grafikleri, gerekli belirsizlikten hedef sınırlar, körler, CRM'ler ve sınırların yıllık gözden geçirilmesi.

- [XRF ile düşük alaşımlı çelikte nikel: gerekli belirsizlikten hedef sınırlar (Nordtest TR 569 Örnek 1)](examples.md#tr569-ex1-nickel-steel-xrf)
- [ICP-OES ile oyuncaklardan bor göçü: EN 71-3 tekrar üretilebilirliğinden hedef sınırlar (Nordtest TR 569 Örnek 10)](examples.md#tr569-ex10-boron-toys-target)
- [Tekrarlı kontrol sonuçlarından tekrarlanabilirlik ve laboratuvar içi tekrar üretilebilirlik (Nordtest TR 569 Örnek 12)](examples.md#tr569-ex12-pooling-sr-srw)
- [XRF ile düşük alaşımlı çelikte kobalt: tayin sınırından hedef sınırlar (Nordtest TR 569 Örnek 2)](examples.md#tr569-ex2-cobalt-steel-xrf)
- [Suda amonyum azotu: paralel numunelerin R-grafiği (Nordtest TR 569 Örnek 3)](examples.md#tr569-ex3-ammonium-r)
- [Suda amonyum azotu: standart çözeltinin X-grafiği (Nordtest TR 569 Örnek 3)](examples.md#tr569-ex3-ammonium-x)
- [ICP-MS ile suda kurşun: laboratuvar içi göl suyunun X-grafiği (Nordtest TR 569 Örnek 4)](examples.md#tr569-ex4-lead-water-icpms)
- [ICP-MS ile balık CRM'sinde arsenik: yalnızca ± %15 eylem sınırlı grafik (Nordtest TR 569 Örnek 5)](examples.md#tr569-ex5-arsenic-crm-tolerance)
- [Suda toplam fosfor: paralel tayinlerin r%-grafiği (Nordtest TR 569 Örnek 6)](examples.md#tr569-ex6-phosphorus-relative-range)
- [GC-ECD ile morina karaciğeri yağında β-HCH: referans değer çevresinde hedef sınırlar (Nordtest TR 569 Örnek 7)](examples.md#tr569-ex7-beta-hch-crm-target)
- [ICP-OES ile suda bakır: paralel tayinlerin R-grafiği ve gözden geçirilmesi (Nordtest TR 569 Örnek 8)](examples.md#tr569-ex8-copper-r-review)
- [ICP-OES ile suda bakır: tüm verilerden yeni X-grafiği sınırları (Nordtest TR 569 Örnek 8)](examples.md#tr569-ex8-copper-x-new-limits)
- [ICP-OES ile suda bakır: ön X-grafiği sınırlarının gözden geçirilmesi (Nordtest TR 569 Örnek 8)](examples.md#tr569-ex8-copper-x-review)
- [Hidrojen peroksitte çinko: yöntem körünün X-grafiği (Nordtest TR 569 Örnek 9)](examples.md#tr569-ex9-zinc-procedural-blank)
- [1 mL mikropipetin sapması: toleransta eylem sınırlı grafik (Nordtest TR 569 Örnek 11)](examples.md#tr569-ex11-micropipette-tolerance)

### Karar sınırları

Ne zaman hız cezası kesilmeli ya da bir gıda numunesi uygunsuz ilan edilmeli? Karar sınırı yasal sınırın ötesine konur; böylece haksız bir suçlama çok olasılık dışı olur (korumalı ret).

- [Hayvansal ürünlerde nandrolon: tarama testi için karar sınırı (JCGM 106 8.3.3)](examples.md#jcgm106-8.3-nandrolone)
- [Hız sınırı denetimi: korumalı ret (JCGM 106 8.3.3)](examples.md#jcgm106-8.3-speed-limit)

### Bir spesifikasyona uygunluk

Eurachem/CITAC uygunluk kılavuzu: korumalı kabul ve ret, az serbestlik dereceli bir t dağılımı ve bir sınıra yakın büyük bir bağıl belirsizlik için lognormal dağılım.

- [Paslanmaz çelikte nikel: iki taraflı şartnamede korumalı kabul (Eurachem/CITAC uygunluk rehberi, Örnek B1)](examples.md#eurachem-compliance-b1-nickel-steel)
- [Üst sınırın altındaki analit, ν = 8 t dağılımı: korumalı ret (Eurachem/CITAC uygunluk rehberi, Örnek B2)](examples.md#eurachem-compliance-b2-upper-limit-t)
- [2 ng/g sınırında yasaklı madde, u_bağıl = %35: lognormal dağılım, kesin s_G (Eurachem/CITAC uygunluk rehberi, Örnek B3)](examples.md#eurachem-compliance-b3-banned-substance-lognormal-exact)
- [2 ng/g sınırında yasaklı madde, u_bağıl = %35: lognormal dağılım, s_G ≈ u_bağıl (Eurachem/CITAC uygunluk rehberi, Örnek B3)](examples.md#eurachem-compliance-b3-banned-substance-lognormal)
- [2 ng/g sınırında yasaklı madde, u_bağıl = %35: u = u_bağıl·L = 0,7 ng/g ile normal dağılım (Eurachem/CITAC uygunluk rehberi, Örnek B3)](examples.md#eurachem-compliance-b3-banned-substance-normal)

### Test belirsizlik oranından koruma bantları

Kalibrasyonda kabul sınırları çoğu zaman toleransın ölçüm belirsizliğine oranından (TUR) türetilir. Üç yaygın koruma bandı yöntemi yol açtıkları risklerle birlikte karşılaştırılır.

- [Test belirsizlik oranından kabul sınırları: Dobbert'in yönetilen koruma bandı (yöntem 6), TUR = 2,5, %95 tolerans içi süreç için küresel riskler](examples.md#tur-guard-band-dobbert-tur2-5)
- [Test belirsizlik oranından kabul sınırları: NCSLI RP-10 koruma bandı, TUR = 2,0, %95 tolerans içi süreç için küresel riskler](examples.md#tur-guard-band-rp10-tur2-0)
- [Test belirsizlik oranından kabul sınırları: karelerin toplamının kökü koruma bandı, TUR = 1,5, %95 tolerans içi süreç için küresel riskler](examples.md#tur-guard-band-rss-tur1-5)

### Sansürlü sonuçlar ("küçüktür")

Bazı laboratuvarlar bir sayı yerine "< 0,015" bildirir. Bunları işlemenin üç yolu karşılaştırılır: silmek, işareti yok saymak ya da sınırın yarısını kullanmak.

- [Bir YT turunda sansürlü sonuçlar: sansürlü sonuçlar çıkarılarak (ISO 13528 E.1)](examples.md#iso13528-e1-censored-deleted)
- [Bir YT turunda sansürlü sonuçlar: sınırın 0,5 katı (ISO 13528 E.1)](examples.md#iso13528-e1-censored-half)
- [Bir YT turunda sansürlü sonuçlar: ‘<’ işareti yok sayılarak (ISO 13528 E.1)](examples.md#iso13528-e1-censored-ignored)

### Homojenlik ve kararlılık

Bir yeterlilik testinden önce sağlayıcı, malzemenin tüm birimlerinin aynı olduğunu ve öyle kaldığını göstermelidir. Tekrarlı ölçümler üzerinde bir varyans analizi bunu cevaplar.

- [Çikolatada arsenik: YT numunelerinin homojenliği ve kararlılığı (ISO 13528 E.2)](examples.md#iso13528-e2-arsenic-homogeneity)

### Bir CRM'den atanan değer

Atanan değer sertifikalı bir referans malzemeyle karşılaştırmadan gelir ve onun belirsizliği de değerlendirmenin parçasıdır.

- [CRM ile karşılaştırmadan atanan değer: agregaların Los Angeles değeri (ISO 13528 E.5)](examples.md#iso13528-e5-los-angeles-reference-value)

### Bir depolama tankının hacmi

Kuvvetlerin çarpımı (π r² h). Bağıl belirsizlikler basitçe birleşir ve Monte Carlo yöntemi GUM sonucunu doğrular.

- [Silindirik depolama tankının hacmi: kuvvetler çarpımı, Gauss formülü ve Monte Carlo (Possolo & Meija 2022)](examples.md#possolo-meija-2022-storage-tank-volume)

### Bir Wheatstone köprüsü

Basit bir çarpım olmayan bir model: duyarlılık katsayılarının doğru türetilmesi gerekir. Program bunu sembolik olarak yapar.

- [Wheatstone köprüsü: kuvvetler çarpımı olmayan bir ölçüm modeli (Possolo & Meija 2022)](examples.md#possolo-meija-2022-wheatstone-bridge)

### İzotop bollukları

İki ölçülen izotop oranından silisyum izotoplarının bollukları. Sonuçlar korelasyonludur, çünkü toplamları 1 olmalıdır.

- [Ölçülen iki izotop miktar oranından silisyumun izotop bollukları (IUPAC 2024, Örnek 4B)](examples.md#iupac2024-4b-silicon-isotopic-abundances)

### Beta dağılımıyla anlatılan bir saflık

Yine kadmiyum standardı; bu kez saflık %100'ü aşamayan bir beta dağılımıyla. Monte Carlo böyle bir dağılımı doğal olarak işler.

- [Saflık için beta dağılımıyla Monte Carlo ile kadmiyum kalibrasyon standardı (IUPAC 2024, Örnek 4H; QUAM A1)](examples.md#iupac2024-4h-cadmium-standard-beta-mcm)

### 1'in altında asimetrik bir aralık

Benzoik asidin saflığı 1'e yakındır ve onu aşamaz; bu yüzden aralığı simetrik değildir. Onu bir beta dağılımı anlatır.

- [Beta dağılımı olarak ifade edilen benzoik asit saflığı (NIST PS1): 1 g/g'nin altında asimetrik bir aralık (IUPAC 2024, Örnek 5C)](examples.md#iupac2024-5c-benzoic-acid-purity-beta)

### Uç mastar kalibrasyonu (GUM H.1)

GUM'un kendisinin ünlü örneği. B tipi terimler için serbestlik derecelerini ve kapsam faktörünü seçmek için Welch–Satterthwaite formülünü kullanır.

- [Uç mastarı kalibrasyonu (GUM H.1)](examples.md#gum-h1-end-gauge)

### Korelasyonlu girdiler (GUM H.2)

Gerilim, akım ve faz eşzamanlı ölçülür, bu yüzden ortalamaları korelasyonludur. Direnç, reaktans ve empedans korelasyon terimleriyle hesaplanır.

- [Korelasyonlu V ve I'dan empedans büyüklüğü (GUM H.2)](examples.md#gum-h2-impedance)
- [Korelasyonlu V, I ve φ'den reaktans (GUM H.2)](examples.md#gum-h2-reactance)
- [Korelasyonlu V, I ve φ'den direnç (GUM H.2)](examples.md#gum-h2-resistance)

### NIST Uncertainty Machine örnekleri

NIST Uncertainty Machine kılavuzundan örnekler: geniş üçgen bölenli bir oran, bir uç mastar, bir direnç ve Stefan–Boltzmann sabiti. GUM ve Monte Carlo sonuçlarını karşılaştırırlar.

- [GUM Tablo H.2'nin yuvarlanmış değerleriyle direnç (NIST Uncertainty Machine §10)](examples.md#num-10-resistance)
- [Geniş üçgen bölenli çarpım ve oran y = ab/c (NIST Uncertainty Machine §3)](examples.md#num-3-ratio-abc)
- [Normal girdilerle uç mastarı kalibrasyonu (NIST Uncertainty Machine §8)](examples.md#num-8-end-gauge-gaussian)
- [CODATA 2010 değerlerinden Stefan–Boltzmann sabiti (NIST Uncertainty Machine §11)](examples.md#num-11-stefan-boltzmann)

### Bir ısıl çiftin kalibrasyonu

Bir ısıl çift bir fırında bir referans ısıl çiftle karşılaştırılır. İki bütçe: fırının sıcaklığı ve bilinmeyen ısıl çiftin emk'si.

- [Termokupl kalibrasyonu: bilinmeyenin emk'sı](examples.md#ea402-s5-thermocouple-emf)
- [Termokupl kalibrasyonu: fırın sıcaklığı](examples.md#ea402-s5-thermocouple-furnace)

### 18 GHz'de bir güç sensörü

Bir mikrodalga güç sensörü: sensör ile kaynak arasındaki uyumsuzluk U biçimli (arksinüs) bir terimdir.

- [Bir güç sensörünün 18 GHz'de kalibrasyonu](examples.md#ea402-s6-power-sensor-18ghz)

### Bir adımlı zayıflatıcı

Bir 30 dB zayıflatıcı. dB cinsinden belirsizlikler, uyumsuzluk terimleri ve bağlantıların tekrarlanabilirliği.

- [Koaksiyel kademeli zayıflatıcının kalibrasyonu (30 dB)](examples.md#ea402-s7-step-attenuator-30db)

### Bir halka mastar

Bir 90 mm halka mastarın çapı ve sıcaklık düzeltmesi, iki bütçede.

- [90 mm'lik halka mastarın kalibrasyonu](examples.md#ea402-s13-ring-gauge-90mm)
- [Halka mastar kalibrasyonu: sıcaklık düzeltmesi](examples.md#ea402-s13-ring-gauge-temperature)

### Bir güç sensörü (UKAS M3003)

Güç sensörü bütçesinin İngiliz sürümü, uyumsuzluk terimiyle.

- [18 GHz'de koaksiyel güç sensörü (UKAS M3003 K.2)](examples.md#m3003-k2-power-sensor-18ghz)

### Bir zayıflatıcı (UKAS M3003)

10 GHz'de bir 30 dB zayıflatıcı; uyumsuzluk terimini hesaplamanın iki yoluyla.

- [E5 formülünden uyumsuzluk terimiyle 30 dB zayıflatıcı (UKAS M3003 K.3, E.9.5)](examples.md#m3003-k3-attenuator-30db-mismatch-e5)
- [10 GHz'de 30 dB koaksiyel zayıflatıcı (UKAS M3003 K.3)](examples.md#m3003-k3-attenuator-30db)

### Bir mastar blok (UKAS M3003)

Bir 10 mm mastar blok. Bir türev sıcaklık etkilerini modelin kendisine yazar; bu, bir bütçe tablosu ile bir ölçüm modeli arasındaki farkı gösterir.

- [Modelinin sıcaklık terimleriyle 10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6, EA-4/02 S4)](examples.md#m3003-k6-gauge-block-10mm-temperature-model)
- [10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6)](examples.md#m3003-k6-gauge-block-10mm)

### 1000 °C'de bir ısıl çift

N tipi bir ısıl çift: çok sayıda küçük katkıyla fırın sıcaklığı ve emk.

- [1000 °C'de N tipi termoçift: tam bütçesiyle EMK (UKAS M3003 K.7)](examples.md#m3003-k7-thermocouple-emf)
- [1000 °C'de termoçift kalibrasyonunda fırın sıcaklığı (UKAS M3003 K.7, birinci bütçe)](examples.md#m3003-k7-thermocouple-furnace)

### Standart koşullarda debi

Standart basınç ve sıcaklığa dönüştürülen bir hacimsel debi: çarpım ve oran modeli.

- [Standart basınç ve sıcaklığa indirgenmiş hacimsel debi (UKAS M3003 K.9)](examples.md#m3003-k9-flowrate-standard-conditions)

### Bir PT100 sondası

Önce sonda bir kuru blokta kalibre edilir; sonra bir gazın sıcaklığını ölçer. Kalibrasyonun belirsizliği ölçümün bir girdisi olur.

- [Kalibre edilmiş PT100 probdan kaptaki gazın sıcaklığı (UKAS M3003 K.11, ikinci bütçe)](examples.md#m3003-k11-gas-temperature)
- [Kuru blokta referans proba karşı PT100 prob kalibrasyonu (UKAS M3003 K.11, birinci bütçe)](examples.md#m3003-k11-probe-calibration)

### Bir laboratuvar terazisinin kalibrasyonu

EURAMET cg-18'e göre 220 g'lık bir terazi: hava kaldırma kuvveti düzeltmesiyle ve düzeltmesiz birkaç yükteki hata ve günlük kullanımda bir tartım sonucunun belirsizliği.

- [Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle 100 g'da hata (EURAMET cg-18 H1, seçenek 2)](examples.md#cg18-h1-100g-option2)
- [Laboratuvar terazisi 220 g / 0,1 mg: ΔT = 5 K için kaldırma kuvveti terimiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 1, 7.1.2-5e formülü)](examples.md#cg18-h1-220g-max-option1-delta-t)
- [Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesi olmadan 220 g'da hata (EURAMET cg-18 H1, seçenek 1)](examples.md#cg18-h1-220g-max-option1)
- [Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 2)](examples.md#cg18-h1-220g-max-option2)
- [Laboratuvar terazisi 220 g / 0,1 mg: sıfır yükte hata (EURAMET cg-18 H1)](examples.md#cg18-h1-220g-zero)
- [Terazi 220 g / 0,1 mg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H1.4/A)](examples.md#cg18-h1-a-weighing-result)
- [Terazi 220 g / 0,1 mg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H1.4/B)](examples.md#cg18-h1-b-weighing-result)

### Çok aralıklı bir terazi

Üç tartım aralıklı 60 kg'lık bir terazi. Her aralığın kendi çözünürlüğü vardır; bu da bütçeyi yük boyunca değiştirir.

- [Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesi olmadan 10 kg'da hata (EURAMET cg-18 H2, seçenek 1)](examples.md#cg18-h2-60kg-10kg-option1)
- [Çok aralıklı terazi 60 kg, kalibrasyondan önce ayarlanmış: Max'ta hata (EURAMET cg-18 H2, ikinci durum)](examples.md#cg18-h2-60kg-max-adjusted)
- [Çok aralıklı terazi 60 kg: ΔT = 10 K için kaldırma kuvveti terimiyle Max'ta hata (EURAMET cg-18 H2, seçenek 1, 7.1.2-5e formülü)](examples.md#cg18-h2-60kg-max-option1-delta-t)
- [Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesi olmadan Max'ta hata (EURAMET cg-18 H2, seçenek 1)](examples.md#cg18-h2-60kg-max-option1)
- [Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesiyle Max'ta hata (EURAMET cg-18 H2, seçenek 2)](examples.md#cg18-h2-60kg-max-option2)
- [Çok aralıklı terazi 60 kg: sıfır yükte hata (EURAMET cg-18 H2)](examples.md#cg18-h2-60kg-zero)
- [Çok aralıklı tartı aleti 12/30/60 kg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H2.4/A)](examples.md#cg18-h2-a-weighing-result)
- [Çok aralıklı tartı aleti 12/30/60 kg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H2.4/B)](examples.md#cg18-h2-b-weighing-result)

### Bir kantar

Büyük terazilerin gerçek kalibrasyonlarındaki gibi standart ağırlıklar ve ikame yükleriyle denenen 30 t'lik bir kantar.

- [Köprü kantar 30 t: 10 t standart ağırlıkta hata (EURAMET cg-18 H3)](examples.md#cg18-h3-30t-10t-weights)
- [Köprü kantar 30 t: ikame yüküyle 15 t'da hata (EURAMET cg-18 H3)](examples.md#cg18-h3-30t-15t-substitution)
- [Köprü kantar 30 t: iki ikame yüküyle Max'ta hata (EURAMET cg-18 H3)](examples.md#cg18-h3-30t-max-substitution)
- [Köprü kantar 30 t: sıfır yükte hata (EURAMET cg-18 H3)](examples.md#cg18-h3-30t-zero)
- [Köprü kantar 30 t / 10 kg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H3.4/A)](examples.md#cg18-h3-a-weighing-result)
- [Köprü kantar 30 t / 10 kg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H3.4/B)](examples.md#cg18-h3-b-weighing-result)

### Düşen bilyeli viskozimetre

Bir bilyenin düşme süresinden viskozite. Yoğunluk ve süre oranları içeren bir model.

- [Düşen bilyalı viskozimetre (NIST TN 1900 E3)](examples.md#nist-tn1900-e3-falling-ball-viscometer)

### Pitot tüpüyle hava hızı

Hız, bir basınç farkından karekök üzerinden bulunur. Bu örnekte denklem ile basılı sonuç arasında yayımlanmış bir tutarsızlık da var (notlara bakın).

- [Pitot tüpüyle hava hızı (NIST TN 1900 E4)](examples.md#nist-tn1900-e4-pitot-tube)

### CO₂'nin molekül ağırlığı

Aralık olarak verilen karbon ve oksijen atom ağırlıklarından.

- [Karbondioksitin molekül ağırlığı (NIST TN 1900 E8)](examples.md#nist-tn1900-e8-molecular-weight-co2)

### Monte Carlo ile bir kadmiyum standardı

Eurachem kadmiyum örneğinin NIST sürümü, Monte Carlo yöntemiyle değerlendirilmiş.

- [Kadmiyum kalibrasyon standardı, Monte Carlo (NIST TN 1900 E9)](examples.md#nist-tn1900-e9-cadmium-standard)

### U biçimli girdiler

Uyumsuzluk terimleri U biçimli olan bir mikrodalga adımlı zayıflatıcı; GUM ve Monte Carlo karşılaştırılır.

- [Mikrodalga kademeli zayıflatıcı, U biçimli girdiler (NIST TN 1900 E11)](examples.md#nist-tn1900-e11-step-attenuator)

### Normal ya da t girdiler, %95 ya da %99

Isıl genleşme katsayısı dört kez değerlendirilir: normal ya da Student t girdilerle ve %95 ya da %99 kapsam için. Her iki seçimin aralığı nasıl değiştirdiğini görün.

- [Isıl genleşme katsayısı, normal girdiler, %95 (NIST TN 1900 E13)](examples.md#nist-tn1900-e13-thermal-expansion-gaussian-95)
- [Isıl genleşme katsayısı, normal girdiler, %99 (NIST TN 1900 E13)](examples.md#nist-tn1900-e13-thermal-expansion-gaussian-99)
- [Isıl genleşme katsayısı, Student t₃ girdiler, %95 (NIST TN 1900 E13)](examples.md#nist-tn1900-e13-thermal-expansion-student-95)
- [Isıl genleşme katsayısı, Student t₃ girdiler, %99 (NIST TN 1900 E13)](examples.md#nist-tn1900-e13-thermal-expansion-student-99)

### Kütle kalibrasyonu: birinci mertebe yetmediğinde

JCGM 101'in kütle kalibrasyonu birinci mertebe GUM, ikinci mertebe terimler ve Monte Carlo ile. Küçük bir doğrusal olmama birinci mertebe belirsizliği fazla küçük yapar.

- [Kütle kalibrasyonu (ikinci mertebe terimli GUM)](examples.md#jcgm101-9.3-mass-calibration-higher-order)
- [Kütle kalibrasyonu (birinci mertebe GUM)](examples.md#jcgm101-9.3-mass-calibration)

### Aynı girdilerden iki çıktı

Ortak girdilerden hesaplanan iki sonuç korelasyonludur. JCGM 102 iki ayrı aralık yerine kovaryans matrislerini ve bir kapsam bölgesini (bir elips) verir.

- [Toplamalı iki değişkenli model, Gauss girdiler (JCGM 102 9.2.2)](examples.md#jcgm102-9.2.2-additive-gaussian)
- [Toplamalı iki değişkenli model, dikdörtgen ortak etki (JCGM 102 9.2.3)](examples.md#jcgm102-9.2.3-additive-rectangular)
- [Toplamalı iki değişkenli model, baskın dikdörtgen ortak etki (JCGM 102 9.2.4)](examples.md#jcgm102-9.2.4-additive-dominant-rectangular)

### İnterpolasyonla pH

Bir çözeltinin pH'ı, 25 °C'de ve başka bir sıcaklıkta iki tampon standarttan. Türevler, standartların sertifikalı değerleri korelasyonlu olduğunda ne olduğunu gösterir.

- [Üç doğrusal interpolasyonla 23,7 °C'de pH, her standardın sertifikalı değerleri sıcaklıklar arasında ilişkili (JCGM GUM-5 Örnek 1)](examples.md#gum5-e1-ph-temperature-interpolation-correlated)
- [Potansiyel ve sıcaklıkta üç doğrusal interpolasyonla 23,7 °C'de pH, bağımsız sertifikalı değerler (JCGM GUM-5 Örnek 1)](examples.md#gum5-e1-ph-temperature-interpolation)
- [Tam ilişkili pH standartlarıyla iki noktalı interpolasyon (JCGM GUM-5 Örnek 1)](examples.md#gum5-e1-ph-two-point-25c-correlated-standards)
- [25 °C'de iki noktalı interpolasyonla bir deney çözeltisinin pH değeri (JCGM GUM-5 Örnek 1)](examples.md#gum5-e1-ph-two-point-25c)

### Bir sera gazı envanteri

Tarım makinelerinden emisyonlar. Aynı faaliyet verisi birkaç gaz için kullanıldığında sonuçlar korelasyonlu olur ve toplam belirsizlik değişir.

- [Sera gazı envanteri, tarımda hareketli makineler (BK 2018): CH₄ ve N₂O'nun paylaştığı faaliyet verisi (JCGM GUM-5 Örnek 5)](examples.md#gum5-e5-ghg-inventory-agriculture-shared-activity)
- [Sera gazı envanteri, tarımda hareketli makineler (BK 2018): bağımsız girdiler (JCGM GUM-5 Örnek 5)](examples.md#gum5-e5-ghg-inventory-agriculture)

### Tahribatlı testlerden kopma kuvveti

Altı tel halat kopana kadar çekilir. Yalnızca altı veri: bir t dağılımı ve Monte Carlo ölçeğine dikkatli bir bakış.

- [Altı tahribatlı deneyden çelik halatın kopma kuvveti (JCGM GUM-5 Örnek 15)](examples.md#gum5-e15-wire-rope-breaking-force)

### Vakum ölçer: bilinen bir düzeltmeyle ne yapmalı

Bir düzeltme faktörü uygulanabilir, yok sayılabilir ya da belirsizliğe katılabilir. Üç senaryo ve ortak bir sıcaklık hatasının bir basınç farkına etkisi karşılaştırılır.

- [Vakum ölçer kalibrasyonu, düzeltme çarpanı ve uygunluk olasılığı, senaryo 1 (EMUE E2.2)](examples.md#emue-e2.2-vacuum-gauge-correction-applied)
- [Vakum ölçer kalibrasyonu, düzeltme çarpanı ve uygunluk olasılığı, senaryo 3 (EMUE E2.2)](examples.md#emue-e2.2-vacuum-gauge-correction-ignored)
- [Vakum ölçer kalibrasyonu, düzeltme çarpanı ve uygunluk olasılığı, senaryo 2 (EMUE E2.2)](examples.md#emue-e2.2-vacuum-gauge-correction-in-uncertainty)
- [Vakum ölçer, ortak sıcaklık hatalı basınç farkı: korelasyon dahil (EMUE E2.2.7)](examples.md#emue-e2.2-vacuum-gauge-sum-model-correlated)
- [Vakum ölçer, ortak sıcaklık hatalı basınç farkı: korelasyon yok sayılmış (EMUE E2.2.7)](examples.md#emue-e2.2-vacuum-gauge-sum-model-correlation-ignored)

### Validasyon verilerinden topraktaki kurşun

Bir ICP-OES yönteminin validasyonundan kurulan bir ISO 21748 bütçesi: kesinlik, gerçeklik ve diğer terimler.

- [Topraktaki kral suyunda çözünür kurşun, ICP-OES: doğrulama verilerinden ISO 21748 bütçesi, kareler ortalamasının kökü sapma belirsizliği (EMUE E3.4)](examples.md#emue-e3.4-soil-lead-iso21748-rms-bias)
- [Topraktaki kral suyunda çözünür kurşun, ICP-OES: doğrulama verilerinden ISO 21748 bütçesi (EMUE E3.4)](examples.md#emue-e3.4-soil-lead-iso21748)

### Geçirgenlikle bir gaz karışımı

Bir geçirgenlik tüpünden amonyağın miktar kesri: kütle kayıp hızı, akış ve molar kütleler.

- [Permeasyonla N₂ içinde NH₃ kalibrasyon gazı karışımı: amonyağın madde miktarı kesri (EMUE E3.8)](examples.md#emue-e3.8-ammonia-permeation)

### Doping kontrolü: idrarda efedrin

Ara kesinlik ve sapmadan yukarıdan aşağı bir belirsizlik ve WADA'nın olumsuz bulgu ilan etmek için kullandığı karar sınırı.

- [İdrarda efedrin: korumalı ret kuralı olarak WADA karar sınırı (EMUE E6.5, JCGM 106)](examples.md#emue-e6.5-ephedrine-wada-decision-limit)
- [Doping kontrolünde idrarda efedrin: ara kesinlik ve bir SRM'den top-down belirsizlik (EMUE E6.5)](examples.md#emue-e6.5-ephedrine-wada-topdown)

### Tekrar üretilebilirlik verisinden belirsizlik

Standart bir yöntem bir laboratuvarlar arası çalışmada denenmişse, tekrar üretilebilirlik standart sapması belirsizliğin iyi bir ilk tahminidir (ISO 21748). Örnekler gerektiğinde başka terimler ekler.

- [Azot ve yağdan et içeriği (ISO 21748 C.2)](examples.md#iso21748-c2-meat-content)
- [Hayvan yeminde ham lif, tekrar üretilebilirlik verilerinden (ISO 21748 C.4)](examples.md#iso21748-c4-crude-fibre)
- [Taşıt emisyonlarında karbon monoksit, tekrar üretilebilirlik verilerinden (ISO 21748 C.1)](examples.md#iso21748-c1-co-emissions)
- [Aerobik koloni sayımı, un: düzeltilmiş tekrar üretilebilirlik (ISO 21748 C.3)](examples.md#iso21748-c3-aerobic-plate-count-flour)
- [Aerobik koloni sayımı, karides: düzeltilmiş tekrar üretilebilirlik (ISO 21748 C.3)](examples.md#iso21748-c3-aerobic-plate-count-shrimp)
- [Aerobik koloni sayımı, sebze: düzeltilmiş tekrar üretilebilirlik (ISO 21748 C.3)](examples.md#iso21748-c3-aerobic-plate-count-vegetables)

### Laboratuvarlar arası kesinlik çalışması

Sekiz ya da daha fazla laboratuvar, birkaç düzey, tekrarlı sonuçlar: Cochran ve Grubbs testlerinden sonra bir yöntemin tekrarlanabilirliği ve tekrar üretilebilirliği. Her örnek klasik yöntemle ve REML ile değerlendirilir.

- [Kömürün kükürt içeriği: REML tahminleri (ISO 5725-2 Örnek C.1)](examples.md#iso5725-2-c1-sulfur-in-coal-reml)
- [Kömürün kükürt içeriği: tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725-2 Örnek C.1)](examples.md#iso5725-2-c1-sulfur-in-coal)
- [Kreozot yağının termometrik titrasyonu: REML tahminleri (ISO 5725-2 Örnek C.3)](examples.md#iso5725-2-c3-creosote-oil-reml)
- [Kreozot yağının termometrik titrasyonu: aykırı laboratuvar ve kesinlik ilişkileri (ISO 5725-2 Örnek C.3)](examples.md#iso5725-2-c3-creosote-oil)
- [Ziftin yumuşama noktası: REML tahminleri (ISO 5725-2 Örnek C.2)](examples.md#iso5725-2-c2-softening-point-pitch-reml)
- [Ziftin yumuşama noktası: eksik verilerle tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725-2 Örnek C.2)](examples.md#iso5725-2-c2-softening-point-pitch)

### Doğrusal bir kalibrasyon

Referans gaz karışımlarından geçen bir doğruyla kalibre edilen UV soğurmalı azot monoksit; iki eksende de belirsizlik.

- [N₂ içinde NO, UV absorpsiyonu: doğrusal analiz fonksiyonu (ISO 6143 Örnek 1)](examples.md#iso6143-b1-no-uv-linear)

### Numune alma: marulda nitrat

Çift numune yöntemi: sekiz partinin her birinden iki numune, her biri iki kez analiz edilir. Belirsizliğin numune alma payı analiz payından büyüktür.

- [Serada yetişen marulda nitrat: çift numune yöntemi, klasik ANOVA (Eurachem UfS Örnek A1)](examples.md#ufs-a1-nitrate-lettuce)

### Numune alma: toprakta kurşun

Büyük bir bağıl belirsizlik logaritmik ölçekte bir çarpan (çarp ya da böl) olarak daha iyi ifade edilir.

- [Kirlenmiş üst toprakta kurşun: klasik ANOVA ve belirsizlik faktörü (Eurachem UfS Örnek A2)](examples.md#ufs-a2-lead-soil-uncertainty-factor)

### Numune alma: yeraltı suyunda demir

Aynı veriler varyans analiziyle ve daha basit aralık yöntemiyle değerlendirilir.

- [Yeraltı suyunda çözünmüş demir: klasik ANOVA (Eurachem UfS Örnek A3)](examples.md#ufs-a3-iron-groundwater-anova)
- [Yeraltı suyunda çözünmüş demir: aralık yöntemi (Eurachem UfS Örnek A3, düzeltilmiş)](examples.md#ufs-a3-iron-groundwater-range)

### Numune alma: lapada A vitamini

Küçük test porsiyonları numune alma varyansını büyütür; 4 g'lık porsiyonlarda kestirilen analitik varyans negatif çıkar, bu da yöntemin sınırlarını gösterir. Çift numunelerin bir kontrol grafiği de vardır.

- [Bebek lapasında A vitamini, 40 g deney numunesi: çift numune yöntemi (Eurachem UfS Örnek A4)](examples.md#ufs-a4-vitamin-a-porridge-40g)
- [Bebek lapasında A vitamini, 4 g deney numunesi: negatif numune alma varyansı (Eurachem UfS Örnek A4)](examples.md#ufs-a4-vitamin-a-porridge-4g)
- [Bebek lapasında A vitamini: çift numunelerin kontrol grafiği (Eurachem UfS Örnek A4)](examples.md#ufs-a4-vitamin-a-qc-chart)

### Modelleme ile numune alma belirsizliği

Çift numuneler yerine numune alma adımları bir GUM bütçesindeki gibi tek tek modellenir.

- [Tarım toprağında kadmiyum: modelleme yaklaşımı (Eurachem UfS Örnek A6)](examples.md#ufs-a6-cadmium-soil)
- [Tarım toprağında fosfor: modelleme yaklaşımı (Eurachem UfS Örnek A6)](examples.md#ufs-a6-phosphorus-soil)

<a id="level-3"></a>

## Düzey 3: İleri

### Yanlış kararların genel riskleri

Üretim süreci bilindiğinde, kabul edilen bir ürünün gerçekte kötü olma olasılığı (müşteri riski) bütün üretim için hesaplanabilir ve gerekli bir risk için bir kabul sınırı seçilebilir.

- [Bilyalı rulmanlar: istenen tüketici riski için kabul sınırı (JCGM 106 9.5.4)](examples.md#jcgm106-9.5-ball-bearings)
- [Hassas dirençler: küresel tüketici ve üretici riskleri (JCGM 106 9.5.3)](examples.md#jcgm106-9.5-precision-resistors)

### Birkaç bileşen birlikte uymalı

Dört korelasyonlu etken maddeli bir ilaç: yanlış karar riskleri Monte Carlo ile birlikte hesaplanır.

- [Dört ilişkili etken maddeli bir ilaç için toplam yanlış karar riskleri (EMUE E2.1)](examples.md#emue-e2.1-multicomponent-medication)

### Önsel bilgiyle genel ve özgül riskler

Havada partikül madde: tipik derişimler hakkındaki önsel bilgi (lognormal dağılım) bir kararın risklerini hesaplamak için ölçümle birleştirilir.

- [Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,17 mg/m³ (JCGM GUM-5 Örnek 10)](examples.md#gum5-e10-tspm-global-risks-acceptance-0.17)
- [Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,22 mg/m³ (JCGM GUM-5 Örnek 10)](examples.md#gum5-e10-tspm-global-risks-acceptance-0.22)
- [Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,2 mg/m³ (JCGM GUM-5 Örnek 10)](examples.md#gum5-e10-tspm-global-risks-shared-risk)
- [Havada askıda partikül madde: lognormal önselle sonsal dağılımdan özgül riskler (JCGM GUM-5 Örnek 10)](examples.md#gum5-e10-tspm-specific-risks)

### Belirgin biçimde doğrusal olmayan bir model

Mikrodalga gücünde karşılaştırma kaybı: sıfıra yakın model o kadar doğrusal değildir ki birinci mertebe GUM tamamen başarısız olur, Monte Carlo ise doğru cevabı verir. Türevler korelasyon ve yüksek mertebe terimler ekler.

- [Karşılaştırma kaybı, x₁ = 0, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri](examples.md#jcgm101-9.4-comparison-loss-x000-correlated-higher-order)
- [Karşılaştırma kaybı, x₁ = 0,0, r = 0,9](examples.md#jcgm101-9.4-comparison-loss-x000-correlated)
- [Karşılaştırma kaybı, x₁ = 0,0](examples.md#jcgm101-9.4-comparison-loss-x000)
- [Karşılaştırma kaybı, x₁ = 0,01, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri](examples.md#jcgm101-9.4-comparison-loss-x010-correlated-higher-order)
- [Karşılaştırma kaybı, x₁ = 0,01, r = 0,9](examples.md#jcgm101-9.4-comparison-loss-x010-correlated)
- [Karşılaştırma kaybı, x₁ = 0,01](examples.md#jcgm101-9.4-comparison-loss-x010)
- [Karşılaştırma kaybı, x₁ = 0,05, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri](examples.md#jcgm101-9.4-comparison-loss-x050-correlated-higher-order)
- [Karşılaştırma kaybı, x₁ = 0,05, r = 0,9](examples.md#jcgm101-9.4-comparison-loss-x050-correlated)
- [Karşılaştırma kaybı, x₁ = 0,05](examples.md#jcgm101-9.4-comparison-loss-x050)

### Mastar blok: tam ve yaklaşık modeller

JCGM 101'in tam doğrusal olmayan mastar blok modeli, doğrusal yaklaşımıyla GUM ve Monte Carlo üzerinden karşılaştırılır.

- [Mastar kalibrasyonu, yaklaşık model](examples.md#jcgm101-9.5-gauge-block-approximate)
- [Mastar kalibrasyonu, doğrusal olmayan model](examples.md#jcgm101-9.5-gauge-block-nonlinear)

### Kartezyen koordinattan kutupsal koordinata

İki koordinattan bir büyüklük ve bir faz. Merkeze yakın dönüşüm belirgin biçimde doğrusal değildir ve kapsam bölgesi bir elips değildir.

- [Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,001, r = 0,9 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x001-correlated)
- [Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,001 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x001)
- [Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,01, r = 0,9 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x010-correlated)
- [Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,01 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x010)
- [Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,1, r = 0,9 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x100-correlated)
- [Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,1 (JCGM 102 9.3)](examples.md#jcgm102-9.3-polar-x100)

### Eşzamanlı verilerden direnç ve reaktans

Korelasyonlu gösterge değerlerinden üç çıktı; alternatif sürüm az veri için çok değişkenli bir t dağılımı kullanır.

- [Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans, t kovaryansı (JCGM 102 9.4)](examples.md#jcgm102-9.4-resistance-reactance-alternative)
- [Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans (JCGM 102 9.4)](examples.md#jcgm102-9.4-resistance-reactance)

### Kapalı modeller: bir direnç termometresi

Sıcaklık direncin bir formülü olarak yazılamaz, bu yüzden denklem sayısal olarak çözülür. Bir termometreden on sıcaklıkla sonuçlar güçlü biçimde korelasyonludur.

- [Direnç termometresiyle Celsius sıcaklığı, örtük model (JCGM 102 9.5.2)](examples.md#jcgm102-9.5.2-resistance-thermometer)
- [Tek direnç termometresiyle on Celsius sıcaklığı, örtük model (JCGM 102 9.5.3)](examples.md#jcgm102-9.5.3-ten-temperatures)

### Aynı enjeksiyonlardan pik alanları

İç standartlı GC-MS ile benzo[a]piren. Aynı enjeksiyonlarda ölçülen iki alan korelasyonludur; türev, serbestlik derecesini sayarken onları bir topluluk olarak ele alır.

- [Benzo[a]piren: aynı enjeksiyonların pik alanları ν_eff'te topluluk olarak (JCGM GUM-5 Örnek 2, Willink 2007)](examples.md#gum5-e2-benzo-a-pyrene-ensemble)
- [İç standartla GC-MS'te benzo[a]piren, ilişkili girdiler (JCGM GUM-5 Örnek 2)](examples.md#gum5-e2-benzo-a-pyrene)

### Gravimetrik bir gaz karışımı

Tartımla hazırlanmış bir kalibrasyon gaz karışımının sekiz miktar kesri: çok sayıda girdi, ara büyüklükler ve korelasyonlu çıktılar.

- [N₂ içinde CO₂ kalibrasyon gazı karışımının gravimetrik hazırlanması: sekiz bileşenin madde miktarı kesirleri ve korelasyonları (JCGM GUM-5 Örnek 4)](examples.md#gum5-e4-gravimetric-gas-mixture)

### Bir tork sensörünün kalibrasyonu

Sıradan ve ağırlıklı en küçük karelerle uydurulan, orijinden geçen bir doğru ve eğimin GUM belirsizliği.

- [Tork sensörü kalibrasyonu, orijinden geçen doğru, GUF ile sıradan en küçük kareler (JCGM GUM-5 Örnek 9)](examples.md#gum5-e9-torque-slope-ols)
- [Tork sensörü kalibrasyonu, orijinden geçen doğru, GUF ile ağırlıklı en küçük kareler (JCGM GUM-5 Örnek 9)](examples.md#gum5-e9-torque-slope-wls)

### Bayes ile şişeler arası homojenlik

Şişeler arası varyans küçük olduğunda klasik tahmin sıfır ya da negatif çıkabilir; hiyerarşik bir Bayesçi model anlamlı bir değer verir.

- [Doğal gaz karışımlarında azotun şişeler arası homojenliği: ANOVA, u*_bb ve Bayesçi hiyerarşik model (JCGM GUM-5 Örnek 12)](examples.md#gum5-e12-between-bottle-homogeneity-bayes)

### Bir toprak referans malzemesinin homojenliği

Üçer kez ölçülen yirmi şişe; varyans analiziyle ve hiyerarşik bir Bayesçi modelle değerlendirilir.

- [Bir toprak referans malzemesinde krom, üçer tekrarlı 20 şişe: ANOVA ve hiyerarşik Bayes modeli (Possolo & Bodnar 2018; ISO Guide 35 C.1 verileri)](examples.md#possolo-bodnar-2018-chromium-soil-homogeneity-bayes)

### İzotop oranlarından atom ağırlığı

Üç ölçülen izotop oranından kurşunun atom ağırlığı ve izotop bileşimi: doğrusal olmayan bir modelden korelasyonlu çıktılar.

- [Üç izotop oranından NRC HIPB-1'deki kurşunun atom ağırlığı ve izotop bileşimi (Possolo & Meija 2022)](examples.md#possolo-meija-2022-atomic-weight-lead-hipb1)

### Kalın kuyruklu bir uzlaşı değeri

Hubble–Lemaître sabitinin yirmi iki tutarsız tahmini, laboratuvar etkileri Laplace dağılımına uyan ve aykırı değerlerden daha az etkilenen bir Bayesçi modelle birleştirilir.

- [Birbiriyle tutarsız 22 kestirimden Hubble–Lemaître sabiti: Laplace etkili Bayesçi uzlaşı (Possolo & Meija 2022)](examples.md#possolo-meija-2022-hubble-lemaitre-constant-laplace)

### Kapalı bir kimyasal model

Dönüm noktası kimyasıyla Mohr titrasyonu: çözünürlük dengesi programın çözdüğü kapalı bir denklem verir.

- [Dönüm noktası kimyasıyla arjantometrik titrasyon (Mohr): AgCl ve Ag₂CrO₄ çözünürlük çarpımlı örtük model (IUPAC 2024, Örnek 3A ve 4I)](examples.md#iupac2024-3a-argentometric-titration-ksp)

### Üç tutarsız belirleme

Birbiriyle uyuşmayan üç belirlemeden iridyumun molar kütlesi. Laboratuvar etkileri hiyerarşik bir Bayesçi modelle modellenir.

- [Birbiriyle tutarsız üç tayinden iridyumun mol kütlesi: laboratuvar etkileri ve Bayesçi rastgele etkiler modeli (IUPAC 2024, Örnek 3E, 4M ve 4N)](examples.md#iupac2024-3e-4m-iridium-molar-mass-bayes)

### Bir anahtar karşılaştırma

CCQM-K145, sığır karaciğerinde nikel: en çok olabilirlikle karanlık belirsizlikli bir uzlaşı değeri. Katılımcılar kodlanmıştır.

- [CCQM-K145 anahtar karşılaştırması, sığır karaciğerinde nikel: en çok olabilirlikle karanlık belirsizlikli uzlaşı değeri (IUPAC 2024, Örnek 4F)](examples.md#iupac2024-4f-ccqm-k145-nickel-bovine-liver)

### Tutarsız verilerin uzlaşı değerleri

Yarı ömürler ve kütle çekim sabiti: belirsizliklerinden çok daha fazla birbirinden ayrılan ölçümler. DerSimonian–Laird, Linear Pool ve hiyerarşik Bayes farklı cevaplar verir; seçim değerlendirmenin bir parçasıdır.

- [¹³⁷Cs yarı ömrü: birbiriyle tutarsız 19 ölçümün uzlaşı değeri (DerSimonian–Laird ve doğrusal havuz; Koepke vd. 2017)](examples.md#koepke2017-half-life-cs137)
- [⁹⁰Sr yarı ömrü: birbiriyle tutarsız 11 ölçümün uzlaşı değeri (DerSimonian–Laird ve doğrusal havuz; Koepke vd. 2017)](examples.md#koepke2017-half-life-sr90)
- [Newton çekim sabiti: 14 ölçümün uzlaşı değeri (DerSimonian–Laird ve doğrusal havuz; Koepke vd. 2017)](examples.md#koepke2017-newtonian-constant-g)

### Karanlık belirsizliğin tonları

Newton kütle çekim sabitinin on altı ölçümü birkaç yöntemle değerlendirilir; uzlaşı değeri ve belirsizliği yönteme güçlü biçimde bağlıdır.

- [16 ölçümden Newton çekim sabiti: Knapp–Hartung düzeltmeli DerSimonian–Laird, en çok olabilirlik, hiyerarşik Bayes ve doğrusal havuz (Merkatas vd. 2019)](examples.md#merkatas2019-newtonian-constant-g-shades)

### Bir karşılaştırmada eşdeğerlik dereceleri

NIST Consensus Builder örnekleri: serbestlik dereceli laboratuvarlardan bir uzlaşı değeri ve 19 laboratuvarın birini dışarıda bırakma ve ikili eşdeğerlik dereceleri.

- [Serbestlik dereceli altı laboratuvardan sedimentte PCB 28: DerSimonian–Laird, Linear Pool ve birini dışarıda bırakan eşdeğerlik dereceleri (NIST Consensus Builder §4.2)](examples.md#nicob-pcb28-sediment-degrees-of-freedom)
- [19 laboratuvarda ⁶⁰Co aktivitesi: birini dışarıda bırakan ve iki taraflı eşdeğerlik dereceleriyle uzlaşı değeri (NIST Consensus Builder §4.4)](examples.md#nicob-co60-activity-leave-one-out-bilateral)

### Ara kesinlik ve iç içe tasarımlar

Zaman ve operatör değiştiğinde kesinlik ve birkaç faktörü az ölçümle ayıran kademeli iç içe tasarım.

- [Çelikte karbon: [zaman + operatör] farklı ara kesinlik (ISO 5725-3 J.1)](examples.md#iso5725-3-j1-carbon-steel)
- [Çelikte vanadyum: kademeli iç içe tasarım, düzey 1 (ISO 5725-3 J.2)](examples.md#iso5725-3-j2-vanadium-steel)

### İki eksende belirsizlikli kalibrasyon fonksiyonları

Doğrusal, ikinci dereceden, kuvvet ve üstel fonksiyonlarla gaz analizi ve içerikleri korelasyonlu referans karışımlar. Program hangi fonksiyonun uyduğunu sınar.

- [Doğal gazda N₂, GC-TCD: korelasyonlu referans karışımlar (ISO 6143 Örnek 2)](examples.md#iso6143-b2-n2-natural-gas-covariances)
- [Doğal gazda N₂, GC-TCD: doğrusal fonksiyon (ISO 6143 Örnek 2)](examples.md#iso6143-b2-n2-natural-gas-linear)
- [Doğal gazda N₂, GC-TCD: ikinci derece polinom (ISO 6143 Örnek 2)](examples.md#iso6143-b2-n2-natural-gas-quadratic)
- [N₂ içinde NO, NDIR: üstel fonksiyon (ISO 6143 Örnek 3)](examples.md#iso6143-b3-no-ndir-exponential)
- [N₂ içinde NO, NDIR: üslü fonksiyon (ISO 6143 Örnek 3)](examples.md#iso6143-b3-no-ndir-power)

### Dinamik seyreltmeyle kalibre edilen bir analizör

Referans karışımlar aynı ana gazdan yapılır, bu yüzden içerikleri korelasyonludur. Kovaryansları yok saymak sonucu değiştirir.

- [Dinamik seyreltmeyle hazırlanan karışımlarla kalibre edilen NO₂ analizörü: ilişkili referans değerlerle analiz fonksiyonu, kovaryanslar yok sayılmış (EMUE E3.3)](examples.md#emue-e3.3-no2-analyser-dynamic-dilution-covariances-ignored)
- [Dinamik seyreltmeyle hazırlanan karışımlarla kalibre edilen NO₂ analizörü: ilişkili referans değerlerle analiz fonksiyonu (EMUE E3.3)](examples.md#emue-e3.3-no2-analyser-dynamic-dilution)

### Bir hata yaklaşım fonksiyonu

Bir terazinin birçok yükteki hatalarından geçirilen bir doğru; ek terim s_m'nin ne kadar büyük olması gerektiğine bir χ² testi ve bir artık testi karar verir.

- [Terazi 400 g / 0,1 mg: s_m = 0,05 mg ile hata yaklaşım fonksiyonu, χ² testi geçer (EURAMET cg-18 H4)](examples.md#cg18-h4-400g-error-function-sm-0.05)
- [Terazi 400 g / 0,1 mg: s_m = 0,25 mg ile hata yaklaşım fonksiyonu, artık testi geçer (EURAMET cg-18 H4)](examples.md#cg18-h4-400g-error-function-sm-0.25)
- [Terazi 400 g / 0,1 mg: hata yaklaşım fonksiyonu E = a₁I, ilk uyum (EURAMET cg-18 H4)](examples.md#cg18-h4-400g-error-function)

### Yukarıdan aşağı bir modelde matris etkileri

Deniz sedimentlerinde yedi element: farklı matrisli referans malzemelerden geri kazanımlar ve iki derişim aralıklı bir model.

- [Deniz sedimentlerinde krom, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)](examples.md#palma2019-cr-marine-sediment)
- [Deniz sedimentlerinde bakır, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)](examples.md#palma2019-cu-marine-sediment)
- [Deniz sedimentlerinde lityum, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)](examples.md#palma2019-li-marine-sediment)
- [Deniz sedimentlerinde mangan, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)](examples.md#palma2019-mn-marine-sediment)
- [Deniz sedimentlerinde nikel, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)](examples.md#palma2019-ni-marine-sediment)
- [Deniz sedimentlerinde kurşun, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)](examples.md#palma2019-pb-marine-sediment)
- [Deniz sedimentlerinde çinko, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)](examples.md#palma2019-zn-marine-sediment)

### Aritmetik ya da ağırlıklı ortalama geri kazanım

Nehir sedimentlerinde floranten: ortalama geri kazanım, MUPR kılavuzu ve hesap tablosundaki gibi iki yolla hesaplanır.

- [Nehir sedimentlerinde fluoranten, aritmetik ortalama geri kazanım (MUPR Eş. 8–12) (Bettencourt da Silva 2026)](examples.md#bettencourt2026-fluoranthene-arithmetic)
- [Nehir sedimentlerinde fluoranten, ağırlıklı ortalama geri kazanım (MUPR Eş. 16.1) (Bettencourt da Silva 2026)](examples.md#bettencourt2026-fluoranthene-weighted)

### Nicel NMR ile saflık (BIPM)

BIPM'in qNMR iç standartları için referans verileri. Tam saflık bütçeleri; bir sonuç fiziksel sınır olan 1'e yakındır ve asimetrik bir aralıkla raporlanır.

- [İç standart olarak maleik asit ile D2O içinde ¹H qNMR ile dimetil sülfon saflığı (BIPM ISRD-01, Tablo 4)](examples.md#bipm-isrd-01-qnmr-dmso2-ma-d2o)
- [İç standart olarak maleik asit ile DMSO-d6 içinde ¹H qNMR ile dimetil tereftalat saflığı (BIPM ISRD-01, Tablo 6)](examples.md#bipm-isrd-01-qnmr-dmtp-ma-dmso-d6)
- [İç standart olarak dimetil sülfon ile CDCl3 içinde ¹H qNMR ile dimetil tereftalat saflığı (BIPM ISRD-02, Tablo 6)](examples.md#bipm-isrd-02-qnmr-dmtp-dmso2-cdcl3)
- [İç standart olarak dimetil sülfon ile D2O içinde ¹H qNMR ile maleik asit saflığı (BIPM ISRD-02, Tablo 4)](examples.md#bipm-isrd-02-qnmr-ma-dmso2-d2o)
- [İç standart olarak BTMSB-d4 ile CD3OD içinde ¹H qNMR ile benzoik asit saflığı (BIPM ISRD-06, Tablo 6)](examples.md#bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od)
- [İç standart olarak BTMSB-d4 ile DMSO-d6 içinde ¹H qNMR ile dimetil sülfon saflığı (BIPM ISRD-06, Tablo 4)](examples.md#bipm-isrd-06-qnmr-dmso2-btmsb-d4-dmso-d6)
- [İç standart olarak DSS-d6 ile D2O içinde ¹H qNMR ile dimetil sülfon saflığı (BIPM ISRD-07, Tablo 4)](examples.md#bipm-isrd-07-qnmr-dmso2-dss-d6-d2o)
- [İç standart olarak DSS-d6 ile D2O içinde ¹H qNMR ile maleik asit saflığı (BIPM ISRD-07, Tablo 6)](examples.md#bipm-isrd-07-qnmr-ma-dss-d6-d2o)
- [İç standart olarak benzoik asit ile CD3OD içinde ¹H qNMR ile BTMSB-d4 saflığı (BIPM ISRD-08, Tablo 2)](examples.md#bipm-isrd-08-qnmr-btmsb-d4-ba-cd3od)

### İzotop seyreltme (IDMS)

Çift izotop seyreltmeyle suda kurşun: izotop oranları, kütleler ve bir kalibrasyon standardı içeren uzun bir model. Küçük belirsizlikli bir referans yöntem.

- [IDMS için kurşun tayin standardı](examples.md#quam-a7-assay-standard)
- [Suda kurşunun çift izotop seyreltmesiyle tayini](examples.md#quam-a7-lead-double-idms)

### Gy'nin numune alma teorisi

Tavuk yemindeki bir enzimin temel numune alma hatası, parçacık boyutu ve özelliklerinden kestirilir.

- [Tavuk yeminde enzim: Gy kuramıyla temel numune alma hatası (Eurachem UfS Örnek A5)](examples.md#ufs-a5-enzyme-feed-gy)
