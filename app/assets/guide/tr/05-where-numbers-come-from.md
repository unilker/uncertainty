# 5. Sayılar nereden gelir?

Bir belirsizlik değerlendirmesinin en zor kısmı, programın yaptığı hesap değildir. Her girdi hakkında elinizdeki bilgiyi bir **standart belirsizliğe** dönüştürmektir. Bu bölüm bir yemek kitabı gibidir: elinizdeki bilgi türünü bulun; hangi dağılımı seçeceğinizi ve ne gireceğinizi söyler.

## Hızlı tablo {#table}

| Elinizdeki | Programdaki dağılım | Ne girilir | Standart belirsizlik |
|---|---|---|---|
| Bir sertifika: "U = 0,6 mm, k = 2" | normal | ortalama, **genişletilmiş belirsizlik U**, **kapsam faktörü k** | U/k = 0,3 mm |
| Bir sertifika: "u = 0,3 mm" | normal | ortalama, **standart belirsizlik** | 0,3 mm |
| Bir tolerans ya da spesifikasyon "± a" | dikdörtgen | değer, **yarı genişlik a** | a/√3 |
| Dijital bir göstergenin çözünürlüğü d | dikdörtgen | değer, yarı genişlik d/2 | d/(2√3) |
| Sınırlar biliniyor, ama merkez daha olası | üçgen | değer, yarı genişlik a | a/√6 |
| Sınırlar arasında salınan bir değer | ark sinüs (U biçimli) | değer, yarı genişlik a | a/√2 |
| Bu ölçümün tekrarlı okumaları | A tipi (gözlemler) | okumalar, her satıra bir | s/√n |
| n okumanın ortalaması, standart sapması daha önceden biliniyor | A tipi, birleştirilmiş | ortalama, birleştirilmiş s, serbestlik derecesi, n | s/√n |
| Az serbestlik dereceli bir sertifika değeri | t (ölçekli, kaydırılmış) | ortalama, U ve k (ya da ölçek) ve serbestlik derecesi | U/k |
| Tam bilinen bir değer (tanımlı bir sabit, bir anma değeri) | sabit | değer | 0 |

Aşağıdaki kısımlar her durumu bir örnekle açıklar.

## Kalibrasyon sertifikaları {#certificate}

Bir sertifika normalde bir genişletilmiş belirsizlik U ve onun kapsam faktörü k'yi verir: "(10,000 05 ± 0,000 20) g, k = 2". Standart belirsizlik U/k = 0,000 10 g'dır.

- Sertifika k vermeden "%95" diyorsa k = 2 kullanın (dağılımın normal ve serbestlik derecesinin büyük olduğu belirtilmişse k = 1,96).
- Etkin serbestlik derecesini veriyorsa (örneğin ν = 9) **t** dağılımını seçin ve onu girin: az serbestlik derecesi kendi sonucunuzun kapsam faktörünü büyütür.
- Bir standardın sertifikalı değeri (10,000 05 g) yerine *anma* değerini (10 g) kullanırsanız, aradaki 0,000 05 g fark sistematik bir hatadır; ya düzeltilmeli ya da belirsizliğe eklenmelidir.

> **Örnek:** [10 kg'lık bir ağırlığın kalibrasyonu](example:ea402-s2-weight-10kg) örneğinde referans ağırlığın sertifikası U = 45 mg ve k = 2 verir: standart belirsizliği 22,5 mg'dır ve bütçedeki en büyük değerdir.

## Toleranslar ve spesifikasyonlar {#tolerance}

Bir üretici ya da bir standart, bir değerin sınırlar içinde olduğunu söyler; örneğin "A sınıfı 100 mL balonjoje: ± 0,10 mL". Başka bir şey bilmiyorsunuz, bu yüzden sınırlar arasındaki her değer eşit olasılıklıdır: yarı genişliği a = 0,10 mL olan bir **dikdörtgen** dağılım ve

```
u = a / √3 = 0,10 / 1,732 = 0,058 mL
```

Anma değerine yakın değerlerin daha olası olduğunu düşünmek için nedenleriniz varsa (hacimsel cam malzemedeki gibi iyi bir üretim kontrolü), bunun yerine **üçgen** dağılım kullanılır: u = a/√6 = 0,041 mL. Eurachem kılavuzu cam malzeme için üçgeni kullanır; kararsız kaldığınızda dikdörtgen daha güvenli seçimdir, çünkü daha büyük u verir.

Cihaz spesifikasyonları çoğu zaman "± (okumanın %0,02'si + 2 basamak)" diye yazılır. 10,000 V okumada, son basamak 0,001 V iken: a = 0,002 + 0,002 = 0,004 V ve u = 0,004/√3 = 0,0023 V.

## Bir gösterge ya da skalanın çözünürlüğü {#resolution}

0,1 g gösteren dijital bir terazi her değeri en yakın 0,1 g'a yuvarlar. Gerçek değer gösterilen değerin ± 0,05 g'ı içinde herhangi bir yerde olabilir: dikdörtgen, yarı genişlik 0,05 g, u = 0,029 g. Analog bir skala için yarı genişlik dürüstçe okuyabildiğiniz kadardır; çoğu zaman yarım bölme ya da daha azı.

> **Dikkat:** Çözünürlük, sonuca giren her okuma için bir kez sayılır. İki terazi okumasının farkı olarak bulunan bir kütlenin (brüt − dara) iki çözünürlük terimi vardır.

## Tekrarlı okumalar: A tipi {#type-a}

Ölçümü n kez tekrarlarsanız okumalar dağılır. Standart sapmaları s, tek bir okumanın dağılımını anlatır; **ortalamalarının** standart belirsizliği

```
u = s / √n
```

ve serbestlik derecesi ν = n − 1'dir. Programda **A tipi (gözlemler)** seçin ve okumaları yazın: ortalamayı, s'yi, u'yu ve ν'yü hesaplar.

- Rutin işte raporladığınız sonuç **tek bir okumaysa**, tek okumanın dağılımı geçerlidir; s'yi kullanın (s/√n'yi değil), tercihen daha önce çok sayıda okumadan belirlenmiş olanı.
- Çok az okumayla (2 ya da 3) s'nin kendisi de çok belirsizdir. Daha iyi bir seçim, daha önce daha çok veriden belirlenmiş bir standart sapmadır: **A tipi, birleştirilmiş**. Örneğin daha önceki bir çalışmadan gelen 25 mg birleştirilmiş standart sapmayla 3 karşılaştırmanın ortalaması: u = 25/√3 = 14,4 mg.

## Yüzde ve bağıl değerler {#relative}

"Saflığın bağıl standart belirsizliği %0,05'tir" demek u = 0,0005 × (değer) demektir. Saflık 0,9999 ise u = 0,000 50'dir. Girmeden önce her zaman girdinin birimine çevirin. Çarpım ve oranlardan oluşan modellerde program bağıl katkıları zaten gösterir.

## Başka bir şeye bağlı etkiler {#effects}

Bazı etkileri doğrudan tahmin etmek yerine modele yazmak daha iyidir. Bir hacim sıcaklıkla değişir:

```
V = V_20 · (1 + γ · (t − 20))
```

Laboratuvar sıcaklığı t ± 2 °C içinde biliniyor (dikdörtgen), suyun genleşme katsayısı γ = 2,1 × 10⁻⁴ /°C. Program etkiyi kendisi hesaplar: 100 mL için yaklaşık 0,024 mL. Bu tür etkileri modele yazmak değerlendirmeyi daha açık yapar ve işaret ve birim hatalarını önler.

## Korelasyonlu girdiler {#correlation}

Ortak bir neden iki girdiye birden etki ediyorsa bunlar korelasyonludur. Tipik durumlar:

- aynı kalibrasyonla aynı terazide yapılan iki tartım,
- aynı stok çözeltiden seyreltilmiş birkaç çözelti,
- birlikte kalibre edilmiş birkaç standardın sertifikalı değerleri,
- iki büyüklüğe birden etki eden aynı sıcaklık etkisi.

Onları **Model** sekmesinde **Korelasyonlar** altında, korelasyon katsayısı r ile (−1'den +1'e) girin. Korelasyon ortak bir büyüklükten kaynaklanıyorsa, o büyüklüğü modele bir kez yazmak (örneğin her iki ifadede aynı stok derişimi) çoğu zaman daha basit ve güvenlidir: program korelasyonu kendiliğinden hesaba katar.

## Hiç bilgi olmadığında {#no-information}

Bazen hiçbir şey belgelenmemiştir; örneğin operatörün bir okumaya etkisi. O zaman deneyiminizi kullanın: "Etkinin ± 0,2 mm'den büyük olmadığından eminim" bir bilgidir; a = 0,2 mm olan bir dikdörtgen dağılım makul ve belgelenmiş bir seçimdir. Bir etkiyi yalnızca tahmin etmesi zor diye dışarıda bırakmayın: kaba bir tahmin sıfırdan iyidir.

> **İpucu:** Her sayının nereden geldiğini yazın. Her girdinin **tanımı** raporda görünür; böylece bir denetçi (ve bir yıl sonra siz) bütçeyi izleyebilir.

## Hesaplamadan önce kontrol listesi {#checklist}

- Her girdinin bir birimi var ve birimler modele uyuyor.
- Hiçbir genişletilmiş belirsizlik standart belirsizlik olarak girilmedi (U'yu k'ye bölün).
- Her etki bir kez ve yalnızca bir kez yer alıyor.
- En iyi değeri sıfır olan düzeltmeler belirsizlikleriyle birlikte hâlâ modelde.
- Korelasyonlar girildi ya da ortak büyüklük modelde.
