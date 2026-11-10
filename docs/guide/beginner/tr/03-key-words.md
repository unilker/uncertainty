# 3. Temel kavramlar, sade bir dille

Her terimin kısa bir açıklaması ve küçük bir örneği var. Örnek hep aynı basit ölçümdür: **bir masanın boyunun şerit metreyle ölçülmesi**. [4. bölüm](04-first-calculation.md) bu ölçümü adım adım hesaplar.

<a id="measurand"></a>

## Ölçülen büyüklük

Ölçmek istediğiniz büyüklük, yeterince kesin tanımlanmış haliyle. "Masanın boyu" tam yeterli değildir: *hangi* kenar, *hangi* sıcaklıkta? İyi bir tanım: "masa tablasının uzun kenarının 20 °C'deki uzunluğu".

<a id="model"></a>

## Ölçüm modeli

Sonucu girdi büyüklüklerinden veren denklem. Masa için:

```
L = L_r + d_cal + d_res
```

L uzunluk, L_r şeridin okuması, d_cal şeridin kalibrasyonundan gelen düzeltme ve d_res en yakın milimetreye okumanın etkisidir. En iyi değeri sıfır olan düzeltmeler de modelde yer alır, çünkü *belirsizlikleri* sıfır değildir.

<a id="input"></a>

## Girdi büyüklüğü ve tahmin

Modelin sağ tarafındaki her büyüklük bir *girdi büyüklüğüdür* (burada L_r, d_cal, d_res). En iyi değeri onun **tahminidir** (kestirim de denir; programda *Tahmin*). Sonuç L ise *çıktı büyüklüğüdür*.

<a id="standard-uncertainty"></a>

## Standart belirsizlik u

Gerçek değerin tahminden tipik olarak ne kadar uzakta olabileceği, bir standart sapma olarak ifade edilmiş haliyle. Her girdi, kendi biriminde bir standart belirsizlik alır. Belirsizliğin "ortak para birimidir": bilgi nereden gelirse gelsin, önce bir standart belirsizliğe dönüştürülür.

<a id="types"></a>

## A tipi ve B tipi

Bir standart belirsizlik elde etmenin iki yolu. İkisi de eşit derecede geçerlidir.

- **A tipi**: tekrarlı ölçümlerin istatistiğiyle. Masayı beş kez ölçersiniz; beş okumanın dağılımı u'yu verir.
- **B tipi**: başka herhangi bir bilgiyle: kalibrasyon sertifikası, üretici spesifikasyonu, el kitabı değeri, göstergenin çözünürlüğü, deneyim.

<a id="distribution"></a>

## Olasılık dağılımı

Bir girdinin hangi değerlerinin olası olduğunun tarifi. Program pek çoğunu bilir; dördü ihtiyaçların çoğunu karşılar:

| Dağılım | Ne zaman | Standart belirsizlik |
|---|---|---|
| **Normal** | tahmine yakın değerler en olası; sertifikalar, çok sayıda okumanın ortalaması | verildiği gibi ya da U/k |
| **Dikdörtgen** | yalnızca ± a sınırları biliniyor, içerideki her değer eşit olasılıklı; toleranslar, çözünürlük, spesifikasyonlar | a/√3 |
| **Üçgen** | ± a sınırları biliniyor, ama merkeze yakın değerler daha olası | a/√6 |
| **U biçimli (ark sinüs)** | değer zamanının çoğunu sınırlara yakın geçirir; bir ayar noktası etrafında salınan sıcaklık | a/√2 |

Nasıl seçileceğini [5. bölüm](05-where-numbers-come-from.md) anlatır.

<a id="sensitivity"></a>

## Duyarlılık katsayısı c

Bir girdi bir birim değiştiğinde sonucun ne kadar değiştiği. L = L_r + d_cal + d_res için her katsayı 1'dir: şeritte bir milimetre fazlası, uzunlukta bir milimetre fazlası demektir. c = m × P / V gibi bir çarpım için katsayılar 1 değildir ve birimleri vardır. Program onları modelden tam olarak hesaplar.

<a id="budget"></a>

## Katkı ve belirsizlik bütçesi

Bir girdinin **katkısı** c × u'dur: standart belirsizliğinin sonucun birimine çevrilmiş hali. **Belirsizlik bütçesi**, tüm girdilerin tahminleri, standart belirsizlikleri, duyarlılık katsayıları ve katkılarıyla birlikte tablosudur. Son sütunu olan *varyans payı*, hangi girdinin en çok önemli olduğunu yüzde olarak söyler.

<a id="combined"></a>

## Birleşik standart belirsizlik u_c

Sonucun standart belirsizliği. Bağımsız girdiler için katkıların karelerinin toplamının kareköküdür:

```
u_c = √( (c₁·u₁)² + (c₂·u₂)² + … )
```

Kareler, büyük bir katkının baskın olduğu anlamına gelir: küçük bir katkıyı yarıya indirmek u_c'yi neredeyse hiç değiştirmez, en büyüğünü yarıya indirmek ise değiştirir.

<a id="dof"></a>

## Serbestlik derecesi ν

Bir standart belirsizliğin ne kadar güvenilir olduğu. 5 okumadan bulunan bir standart sapmanın ν = 4 serbestlik derecesi vardır ve kendisi de oldukça belirsizdir; 50 okumadan bulunanın ν = 49'dur ve güvenilirdir. Bir B tipi değer genellikle kesin kabul edilir (ν = ∞). Sonucun *etkin serbestlik derecesi* ν_eff, girdilerinkinden bulunur (Welch–Satterthwaite formülü).

<a id="expanded"></a>

## Genişletilmiş belirsizlik U, kapsam faktörü k, kapsam olasılığı

Standart belirsizlik olası değerlerin yalnızca yaklaşık %68'ini kapsar. Gerçek değeri yüksek olasılıkla içeren bir aralık vermek için u_c bir **kapsam faktörü** k ile çarpılır:

```
U = k · u_c
```

k = 2 ile **kapsam olasılığı** yaklaşık %95'tir. ν_eff küçük olduğunda k daha büyük olmalıdır (t dağılımından); sabit bir k yerine bir olasılık isterseniz program bunu yapar.

<a id="correlation"></a>

## Korelasyon

İki girdiyi ortak bir neden etkiliyorsa *korelasyonludur*; örneğin aynı terazide tartılan iki kütle ya da aynı stok çözeltiden hazırlanan iki çözelti. Korelasyon katsayısı r, −1 ile +1 arasındadır. Korelasyon birleşik belirsizliği artırabilir ya da azaltabilir, bu yüzden girilmelidir (**Model** sekmesinde, *Korelasyonlar*).

<a id="relative"></a>

## Bağıl belirsizlik

Belirsizliğin değere bölümü, çoğu zaman yüzde olarak: u/|y|. Yalnızca çarpım ve oranlardan oluşan modellerde bağıl belirsizlikler yukarıdaki katkılar gibi basitçe birleşir.

<a id="bias"></a>

## Sapma, geri kazanım ve düzeltme

**Sapma** (bias), bir yöntemin sonuçları ile gerçek değer arasındaki sistematik farktır; referans malzemelerle ya da yeterlilik testleriyle bulunur. **Geri kazanım**, bir yöntemin analitin ne kadarını bulduğudur (ideal olan %100). Bilinen bir sapma düzeltilir; düzeltmenin belirsizliği bütçede kalır.

<a id="precision"></a>

## Tekrarlanabilirlik ve tekrar üretilebilirlik

**Tekrarlanabilirlik** (s_r): aynı koşullarda, aynı gün, aynı kişi, aynı cihazla sonuçların dağılımı. **Laboratuvar içi tekrar üretilebilirlik** (s_Rw): bir laboratuvarda uzun bir süre boyunca, farklı günler, kişiler ve kalibrasyonlarla dağılım. **Tekrar üretilebilirlik** (s_R): farklı laboratuvarlar arasındaki dağılım.

<a id="traceability"></a>

## İzlenebilirlik ve CMC

**Metrolojik izlenebilirlik**, bir sonucun her birinin belirsizliği bilinen kesintisiz bir kalibrasyon zinciriyle SI birimlerine bağlı olması demektir. **CMC** (kalibrasyon ve ölçüm yeteneği), akredite bir kalibrasyon laboratuvarının bir hizmet için beyan edebileceği en küçük belirsizliktir.

<a id="terms"></a>

## Türkçe–İngilizce terimler

| Türkçe | İngilizce |
|---|---|
| ölçülen büyüklük | measurand |
| ölçüm modeli | measurement model |
| girdi / çıktı büyüklüğü | input / output quantity |
| tahmin (kestirim) | estimate |
| standart belirsizlik | standard uncertainty |
| birleşik standart belirsizlik | combined standard uncertainty |
| genişletilmiş belirsizlik | expanded uncertainty |
| kapsam faktörü / olasılığı | coverage factor / probability |
| duyarlılık katsayısı | sensitivity coefficient |
| serbestlik derecesi | degrees of freedom |
| belirsizlik bütçesi | uncertainty budget |
| tekrarlanabilirlik / tekrar üretilebilirlik | repeatability / reproducibility |
| sapma / geri kazanım | bias / recovery |
| yeterlilik testi | proficiency test |
| uzlaşı değeri | consensus value |
| eşdeğerlik derecesi | degree of equivalence |
