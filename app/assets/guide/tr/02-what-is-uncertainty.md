# 2. Ölçüm belirsizliği nedir?

## Hiçbir ölçüm kesin değildir {#idea}

Banyo tartısına art arda üç kez çıkın. 72,4 kg, 72,6 kg ve 72,5 kg okuyabilirsiniz. "Gerçek" ağırlığınız hangisi? Hiçbiri tam olarak değil. Her okuma biraz farklıdır, tartının kendisi biraz fazla ya da biraz az gösteriyor olabilir ve son basamak yuvarlanmıştır.

Her laboratuvarda da durum aynıdır, yalnızca sayılar daha küçüktür: bir terazi, bir termometre, bir pipet ya da bir kimyasal analiz cihazı hiçbir zaman tam gerçek değeri vermez. Bir büyüklüğün **gerçek değeri** hiçbir zaman mükemmel biçimde bilinemez. Yapabileceğimiz, bir **en iyi tahmin** vermek ve gerçek değerin ondan **ne kadar uzakta olabileceğini** söylemektir. Bu ikinci kısım *ölçüm belirsizliğidir*.

Dolayısıyla eksiksiz bir ölçüm sonucunun her zaman iki parçası vardır:

```
m = (72,5 ± 0,3) kg   (kapsam olasılığı yaklaşık %95)
```

Şöyle okunur: "En iyi tahminimiz 72,5 kg'dır ve gerçek değerin 72,2 kg ile 72,8 kg arasında olduğundan yaklaşık %95 eminiz."

> **Not:** Belirsizliği olmayan bir sonuç bir sınırla, başka bir laboratuvarla ya da daha önceki bir sonuçla karşılaştırılamaz. Tek başına "72,5 kg", 72,8 kg'ın farklı bir sonuç mu yoksa aynı sonuç mu olduğunu söylemez.

## Hata ve belirsizlik farklı şeylerdir {#error}

- **Hata**, ölçülen değer ile gerçek değer arasındaki farktır. Gerçek değer bilinmediği için hata da bilinmez.
- Hatanın bir kısmı biliniyorsa (örneğin bir kalibrasyon sertifikası termometrenizin 0,3 °C fazla gösterdiğini söylüyorsa) onu **düzeltirsiniz**: 0,3 °C çıkarırsınız.
- **Belirsizlik**, tüm düzeltmelerden sonra bilinmeyen olarak kalanı anlatır: sonuç hakkındaki şüpheyi.

Yani belirsizlik bir yanlışlık değildir ve "ne kadar yanıldığınız" da değildir. Sonucu ne kadar iyi bildiğinizin dürüst bir ifadesidir.

## Neden önemli? {#why}

- **Kararlar.** Sudaki kurşun yasal sınırın altında mı? Parça toleransına uyuyor mu? Bir sınıra yakın bir sonuç ancak belirsizliğiyle değerlendirilebilir ([10. bölüm](10-special-evaluations#conformity)).
- **Karşılaştırmalar.** İki laboratuvar aynı numuneyi ölçüyor ve farklı sayılar buluyor. Uyuşuyorlar mı? Bunu yalnızca belirsizlikleri söyleyebilir.
- **Akreditasyon.** ISO/IEC 17025, deney ve kalibrasyon laboratuvarlarının sonuçlarının belirsizliğini değerlendirmesini (madde 7.6) ve bunu kalibrasyon sertifikalarında, gerektiğinde de deney raporlarında bildirmesini şart koşar.
- **İyileştirme.** Bir belirsizlik bütçesi hangi etkinin en çok önemli olduğunu gösterir; böylece daha iyi ölçmek için nereye yatırım yapacağınızı bilirsiniz.

## Beş adımda tarif {#recipe}

Uluslararası başvuru belgesi *Ölçümde belirsizliğin ifade edilmesi kılavuzu*dur; kısaca **GUM** (JCGM 100) denir. Yöntemi beş adımda özetlenebilir:

1. **Tam olarak neyi ölçtüğünüzü söyleyin** (*ölçülen büyüklük*) ve sonucun başka büyüklüklerden nasıl hesaplandığını yazın: *ölçüm modeli*. Örnek: bir çözeltinin derişimi = madde kütlesi × saflık ÷ hacim.
2. **Sonucu etkileyen her şeyi listeleyin**: okumalar, cihazların kalibrasyonu, sıcaklık, bir reaktifin saflığı, bir göstergenin çözünürlüğü…
3. **Her etkiye bir standart belirsizlik verin**; ya tekrarlı ölçümlerin istatistiğinden (*A tipi*) ya da sertifika, spesifikasyon gibi başka bilgilerden (*B tipi*). [5. bölüm](05-where-numbers-come-from) nasıl yapılacağını gösterir.
4. Her birinin sonucu ne kadar güçlü etkilediğini hesaba katarak onları **birleştirin**.
5. Birleşik belirsizliği daha geniş bir aralığa **genişletin** (genellikle yaklaşık %95) ve sonucu **raporlayın**.

1–3. adımları siz yaparsınız, çünkü ölçümünüz hakkında bilgi gerektirirler. **4. ve 5. adımları program sizin için yapar**: tam olarak ve her ara değeri göstererek; üstelik sonucu ikinci bir yöntemle, Monte Carlo yöntemiyle denetleyebilir ([7. bölüm](07-monte-carlo)).

## Aynı hedefe iki yol {#approaches}

- **Aşağıdan yukarı (modelleme).** 1. adımdaki modeli kurun ve her girdiyi sayısallaştırın. Kalibrasyon laboratuvarları ve iyi anlaşılmış yöntemler için tipiktir. Programda: **Yeni ölçüm modeli**.
- **Yukarıdan aşağı (laboratuvar verisinden).** Bir deney laboratuvarının zaten sahip olduğu verileri kullanın: sonuçların aylar boyunca ne kadar dağıldığı (kontrol grafiklerinden) ve referans değerlerden ne kadar uzak oldukları (referans malzemelerden ya da yeterlilik testlerinden). Kimyasal ve çevresel deneyler için tipiktir. Programda: **Yeni top-down değerlendirme**.

Her ikisi de ISO/IEC 17025'e göre kabul edilir ve her ikisi de aynı biçimde genişletilip raporlanan bir standart belirsizlik verir. Seçim için [8. bölüme](08-choosing-a-project-type) bakın.

> **Deneyin:** [10 kg'lık bir ağırlığın kalibrasyonu](example:ea402-s2-weight-10kg) örneğini açın. **Genel** sekmesinde tarifin beş adımının gerçek bir kalibrasyona uygulanışını bulacaksınız.
