# Bütün örnekler adım adım

Kütüphanedeki 284 örneğin her biri, uygulamanın **Genel** sekmesinde gösterilen açıklamayla. Örnekler derslerine ve düzeylerine göre gruplanmıştır.

## Bir tozu tartmak (Başlangıç)

Çok basit bir bütçe: tozun kütlesi, dolu kabın okumasından boş kabın okumasının çıkarılmasıyla ve terazinin kayması için küçük bir düzeltmeyle bulunur. İki okumanın belirsizliği aynıdır; birlikte birinin iki katını değil, √2 katını verirler. Model doğrusal olduğu için GUM ve Monte Carlo aynı sonucu verir.

<a id="nist-tn1900-e1-weighing"></a>

### Tartımla toz kütlesi (NIST TN 1900 E1)

`nist-tn1900-e1-weighing`

#### Bu proje ne hakkında?

TN 1900 E1: tek kefeli terazide üç tartımdan toz kütlesi; doğrusal model, GUM ve MCM aynı sonucu verir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **m_P** (g): Tozun kütlesi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
m_P = c_P - c_E - dc_R
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **c_P**: Toz dolu kabın okuması. En iyi değer 53,768 g, standart belirsizlik u = 0,005015 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **c_E**: Boş kabın okuması. En iyi değer 3,436 g, standart belirsizlik u = 0,005015 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **dc_R**: Referans ağırlık okumalarının kayması c_R2 − c_R1. En iyi değer 0,048 g, standart belirsizlik u = 0,0005477 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 50,284 g
- standart belirsizlik u = 0,0071 g

Kaynak: NIST TN 1900 E1: m_P = 50,284 g, u = 0,0071 g, %95 aralığı [50,270; 50,298] g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Girdileri toplamak: normal ve dikdörtgen (Başlangıç)

Dört girdi basitçe toplanır. Üç türev yalnızca girdi dağılımlarının biçiminde ayrılır. GUM sonucunu Monte Carlo ile karşılaştırın: tek bir dikdörtgen girdi baskın olduğunda sonuç artık normal değildir ve %95 aralığı 2u'dan dardır. Bu, k = 2'nin doğru olmadığı klasik durumdur.

<a id="jcgm101-9.2.2-additive-gaussian"></a>

### Normal dağılımlı girdilerle toplamsal model

`jcgm101-9.2.2-additive-gaussian`

#### Bu proje ne hakkında?

JCGM 101 9.2.2: her biri N(0, 1) olan dört bağımsız girdi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **Y**. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
Y = X1 + X2 + X3 + X4
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **X3**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **X4**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0
- standart belirsizlik u = 2

Kaynak: JCGM 101:2008 Tablo 2 (kesin: N(0, 4)); MCM toleransları M = 10⁶'da Monte Carlo saçılımının ≈ 5 katı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.2.3-additive-rectangular"></a>

### Dikdörtgen dağılımlı girdilerle toplamsal model

`jcgm101-9.2.3-additive-rectangular`

#### Bu proje ne hakkında?

JCGM 101 9.2.3: her biri standart sapması 1 olan dikdörtgen dağılımlı dört bağımsız girdi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **Y**. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
Y = X1 + X2 + X3 + X4
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. Yalnızca sınırlar biliniyor: değer -1,7321 ile 1,7321 arasında (0 ± 1,7321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,7321/√3 = 1,00.
2. **X2**. Yalnızca sınırlar biliniyor: değer -1,7321 ile 1,7321 arasında (0 ± 1,7321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,7321/√3 = 1,00.
3. **X3**. Yalnızca sınırlar biliniyor: değer -1,7321 ile 1,7321 arasında (0 ± 1,7321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,7321/√3 = 1,00.
4. **X4**. Yalnızca sınırlar biliniyor: değer -1,7321 ile 1,7321 arasında (0 ± 1,7321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,7321/√3 = 1,00.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0
- standart belirsizlik u = 2

Kaynak: JCGM 101:2008 Tablo 3 ve Ek E (analitik uç noktalar ±3,8812).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.2.4-additive-dominant-rectangular"></a>

### Baskın dikdörtgen girdili toplamsal model

`jcgm101-9.2.4-additive-dominant-rectangular`

#### Bu proje ne hakkında?

JCGM 101 9.2.4: 9.2.3 ile aynı, ancak X4'ün standart sapması 10'dur.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **Y**. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
Y = X1 + X2 + X3 + X4
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. Yalnızca sınırlar biliniyor: değer -1,7321 ile 1,7321 arasında (0 ± 1,7321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,7321/√3 = 1,00.
2. **X2**. Yalnızca sınırlar biliniyor: değer -1,7321 ile 1,7321 arasında (0 ± 1,7321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,7321/√3 = 1,00.
3. **X3**. Yalnızca sınırlar biliniyor: değer -1,7321 ile 1,7321 arasında (0 ± 1,7321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,7321/√3 = 1,00.
4. **X4**. Yalnızca sınırlar biliniyor: değer -17,321 ile 17,321 arasında (0 ± 17,321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 17,321/√3 = 10,0.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- standart belirsizlik u = 10,149

Kaynak: JCGM 101:2008 Tablo 4 (u(y) = √103 = 10,149).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir ağırlığın kalibrasyonu (Başlangıç)

Bir 10 kg ağırlık bir referans ağırlıkla karşılaştırılır. Tipik bir kalibrasyon bütçesini gösterir: standardın sertifikası (U ve k), son kalibrasyondan beri sürüklenme, komparatör, hava kaldırma kuvveti ve tekrarlı karşılaştırmalar. En büyük katkı standardın sertifikasıdır.

<a id="ea402-s2-weight-10kg"></a>

### 10 kg'lık bir ağırlığın kalibrasyonu

`ea402-s2-weight-10kg`

#### Bu proje ne hakkında?

EA-4/02 S2: OIML M1 sınıfı ağırlığın F2 standarda karşı kütle komparatörüyle (ABBA) kalibrasyonu.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **m_X** (mg): Ağırlığın 10 kg'dan sapması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
m_X = m_S + dm_D + dm + dm_C + dB
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m_S**: Standart (sertifika: +5 mg, U = 45 mg, k = 2). En iyi değer 5 mg. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 45 mg, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 45/2 = 22,5 mg olur (normal dağılım).
2. **dm_D**: Standardın son kalibrasyondan beri sürüklenmesi. Yalnızca sınırlar biliniyor: değer -15 ile 15 mg arasında (0 ± 15) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 15/√3 = 8,66 mg.
3. **dm**: Gözlenen fark (3 ölçüm ortalaması, s_p = 25 mg). En iyi değer 20 mg, standart belirsizlik u = 14,434 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **dm_C**: Komparatör eksantrikliği ve manyetik etkiler. Yalnızca sınırlar biliniyor: değer -10 ile 10 mg arasında (0 ± 10) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 10/√3 = 5,77 mg.
5. **dB**: Hava kaldırma kuvveti. Yalnızca sınırlar biliniyor: değer -10 ile 10 mg arasında (0 ± 10) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 10/√3 = 5,77 mg.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 25 mg
- standart belirsizlik u = 29,2 mg
- genişletilmiş belirsizlik U = 58 mg

Kaynak: EA-4/02 M:2022 S2.10–S2.11 (u = 29,2 mg, U = 58 mg).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## UKAS M3003'te bir 10 kg ağırlık (Başlangıç)

EA-4/02'dekiyle aynı türden bir ağırlık kalibrasyonu, İngiliz kılavuzundaki biçimiyle. İkisini karşılaştırmak, farklı laboratuvarların bileşenler ve dağılımlar için biraz farklı seçimler yaptığını ve sonucun yine de benzer çıktığını gösterir.

<a id="m3003-k4-weight-10kg"></a>

### OIML M1 sınıfı 10 kg ağırlık (UKAS M3003 K.4)

`m3003-k4-weight-10kg`

#### Bu proje ne hakkında?

Kütle karşılaştırıcısında F2 sınıfı bir standartla üç ara-okumalı karşılaştırma.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **W_X** (g): Bilinmeyen ağırlığın kütlesi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
W_X = W_S + dW + dD_S + dI_d + dC + dA_b
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **W_S**: Standart ağırlık (sertifika, U = 30 mg, k = 2). En iyi değer 10000,005 g. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,03 g, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,03/2 = 0,0150 g olur (normal dağılım).
2. **dW**: Üç karşılaştırmanın ortalama farkı (tekrarlanabilirlik s = 8,7 mg, ν = 9). 3 okumanın ortalaması, 0,02 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,0087 g (ν = 9) kullanılır; böylece u = s_p/√n = 0,00502 g.
3. **dD_S**: Standardın sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,03 ile 0,03 g arasında (0 ± 0,03) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,03/√3 = 0,0173 g.
4. **dI_d**: İki göstergenin yuvarlanması (10 mg basamak). Değer -0,01 ile 0,01 g arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,01/√6 = 0,00408 g.
5. **dC**: Karşılaştırıcı doğrusal olmama. Yalnızca sınırlar biliniyor: değer -0,003 ile 0,003 g arasında (0 ± 0,003) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,003/√3 = 0,00173 g.
6. **dA_b**: Hava kaldırma kuvveti (nominal değerin ±1 ppm'i). Yalnızca sınırlar biliniyor: değer -0,01 ile 0,01 g arasında (0 ± 0,01) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,01/√3 = 0,00577 g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 10000,025 g
- standart belirsizlik u = 0,02456 g
- genişletilmiş belirsizlik U = 0,04912 g

Kaynak: UKAS M3003 6. baskı K.4.10–K.4.11: u_c = 24,56 mg, U = 49,12 mg; 10 000,025 g ± 0,049 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir kumpasın kalibrasyonu (Başlangıç)

Bir mastar blok bir kumpasla ölçülür. Bileşenlerin çoğu dikdörtgendir: skalanın çözünürlüğü, sıcaklık farkı, çenelerin düzlüğü. Bir skaladaki okumanın bir belirsizliğe nasıl dönüştüğünün iyi bir örneği.

<a id="ea402-s10-vernier-calliper"></a>

### Kumpasın 150 mm'de kalibrasyonu

`ea402-s10-vernier-calliper`

#### Bu proje ne hakkında?

EA-4/02 S10: kumpas hatasının mastar bloğuyla ölçülmesi; iki dikdörtgen terim baskındır (yamuk çıktı).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E_X** (µm): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E_X = l_iX - l_S + 150000*11.5e-6*dt + dl_iX + dl_M
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **l_iX**: Gösterge. Tam bilinen bir değer, 150100 µm; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **l_S**: Mastar bloğunun boyu. Yalnızca sınırlar biliniyor: değer 149999,2 ile 150000,8 µm arasında (150000 ± 0,8) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,8/√3 = 0,462 µm.
3. **dt**: Kumpas–blok sıcaklık farkı. Yalnızca sınırlar biliniyor: değer -2 ile 2 K arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 K.
4. **dl_iX**: Skala çözünürlüğü. Yalnızca sınırlar biliniyor: değer -25 ile 25 µm arasında (0 ± 25) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 25/√3 = 14,4 µm.
5. **dl_M**: Mekanik etkiler (kuvvet, düzlemsellik, paralellik). Yalnızca sınırlar biliniyor: değer -50 ile 50 µm arasında (0 ± 50) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 50/√3 = 28,9 µm.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 100 µm
- standart belirsizlik u = 32 µm

Kaynak: EA-4/02 M:2022 S10.10–S10.12 (u = 32 µm).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir standart direncin kalibrasyonu (Başlangıç)

Bir 10 kΩ direnç bir referans dirençle karşılaştırılır. Model bir çarpımdır, bu yüzden duyarlılık katsayıları 1 değildir; bütçenin her bağıl etkiyi nasıl ohm'a çevirdiğine bakın.

<a id="ea402-s3-resistor-10kohm"></a>

### 10 kΩ'luk standart direncin kalibrasyonu

`ea402-s3-resistor-10kohm`

#### Bu proje ne hakkında?

EA-4/02 S3: dört uçlu direncin, oran cihazı olarak sayısal multimetre kullanılarak standartla yer değiştirme yöntemiyle kalibrasyonu.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **R_X** (mΩ): Bilinmeyen direnç (mΩ). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
R_X = (R_S + dR_D + dR_TS) * r_C * r - dR_TX
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R_S**: Standart direnç (sertifika). En iyi değer 10000053 mΩ. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 5 mΩ, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 5/2 = 2,50 mΩ olur (normal dağılım).
2. **dR_D**: Son kalibrasyondan beri sürüklenme. Yalnızca sınırlar biliniyor: değer 10 ile 30 mΩ arasında (20 ± 10) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 10/√3 = 5,77 mΩ.
3. **dR_TS**: Standarda sıcaklık etkisi. Yalnızca sınırlar biliniyor: değer -2,75 ile 2,75 mΩ arasında (0 ± 2,75) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2,75/√3 = 1,59 mΩ.
4. **r_C**: Oran düzeltmesi (multimetre). Değer 0,999999 ile 1,000001 arasında ve merkez 1'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,000001/√6 = 4,08 × 10⁻⁷.
5. **r**: Gözlenen oran R_iX/R_iS (5 okuma). 5 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 1,00001; okumalar s = 1,58 × 10⁻⁷ standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 1,58 × 10⁻⁷/√5 = 7,07 × 10⁻⁸, serbestlik derecesi ν = n − 1 = 4.
6. **dR_TX**: Bilinmeyene sıcaklık etkisi. Yalnızca sınırlar biliniyor: değer -5,5 ile 5,5 mΩ arasında (0 ± 5,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5,5/√3 = 3,18 mΩ.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 10000178 mΩ
- standart belirsizlik u = 8,33 mΩ
- genişletilmiş belirsizlik U = 17 mΩ

Kaynak: EA-4/02 M:2022 S3.10–S3.11 (u = 8,33 mΩ, U = 0,017 Ω).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir dijital multimetrenin kalibrasyonu (Başlangıç)

Bir el multimetresi 100 V'ta bir kalibratöre karşı denetlenir. Göstergenin çözünürlüğü çoğu zaman en büyük terimdir: tek ondalıkla gösterilen bir değer yarım basamak içinde herhangi bir yerde olabilir.

<a id="ea402-s9-dmm-100v"></a>

### El tipi sayısal multimetrenin 100 V'ta kalibrasyonu

`ea402-s9-dmm-100v`

#### Bu proje ne hakkında?

EA-4/02 S9: 3½ basamaklı multimetrenin gösterge hatası; çözünürlük terimi baskındır ve çıktı neredeyse dikdörtgen dağılımlıdır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E_X** (V): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E_X = V_iX - V_S + dV_iX - dV_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V_iX**: Multimetrenin göstergesi. Tam bilinen bir değer, 100,1 V; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **V_S**: Kalibratörün gerilimi. En iyi değer 100 V. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,002 V, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,002/2 = 0,00100 V olur (normal dağılım).
3. **dV_iX**: Multimetrenin çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 V arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 V.
4. **dV_S**: Kalibratör: şartname ve sürüklenme. Yalnızca sınırlar biliniyor: değer -0,011 ile 0,011 V arasında (0 ± 0,011) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,011/√3 = 0,00635 V.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,1 V
- standart belirsizlik u = 0,03 V

Kaynak: EA-4/02 M:2022 S9.12–S9.14 (u = 0,030 V).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir mastar bloğun kalibrasyonu (Başlangıç)

Bir 50 mm mastar blok bir referans blokla karşılaştırılır. Sıcaklık önemlidir: sıcaklıktaki ya da genleşme katsayısındaki küçük bir fark boyu değiştirir; bütçede ısıl terimlerin olmasının nedeni budur.

<a id="ea402-s4-gauge-block-50mm"></a>

### 50 mm'lik mastar bloğunun kalibrasyonu

`ea402-s4-gauge-block-50mm`

#### Bu proje ne hakkında?

EA-4/02 S4: 0 sınıfı çelik mastar bloğunun kalibre edilmiş standartla karşılaştırılması; iki sıfır ortalamalı büyüklüğün çarpımı ikinci mertebe terim gerektirir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **l_X** (nm): Mastar bloğunun boyu. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
l_X = l_S + dl_D + dl + dl_C - 50000000*(alpha*dt + d_alpha*theta) - dl_V
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **l_S**: Standart (sertifika: 50,000 02 mm, U = 30 nm). En iyi değer 50000020 nm, standart belirsizlik u = 15 nm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **dl_D**: Standardın sürüklenmesi. Değer -30 ile 30 nm arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 30/√6 = 12,2 nm.
3. **dl**: Gözlenen fark (5 ölçüm ortalaması, s_p = 12 nm). En iyi değer -94 nm, standart belirsizlik u = 5,3666 nm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **dl_C**: Komparatörün doğrusallık ve sıfır hatası. Yalnızca sınırlar biliniyor: değer -32 ile 32 nm arasında (0 ± 32) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 32/√3 = 18,5 nm.
5. **alpha**: Ortalama genleşme katsayısı. Tam bilinen bir değer, 0,0000115 1/K; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
6. **dt**: Bloklar arası sıcaklık farkı. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 K arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 K.
7. **d_alpha**: Genleşme katsayıları farkı. Değer -0,000002 ile 0,000002 1/K arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,000002/√6 = 8,16 × 10⁻⁷ 1/K.
8. **theta**: Ortalama sıcaklığın 20 °C'den sapması. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 K arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 K.
9. **dl_V**: Merkez dışı temas. Yalnızca sınırlar biliniyor: değer -6,7 ile 6,7 nm arasında (0 ± 6,7) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 6,7/√3 = 3,87 nm.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 49999926 nm
- standart belirsizlik u = 34,3 nm
- genişletilmiş belirsizlik U = 69 nm

Kaynak: EA-4/02 M:2022 S4.12–S4.14 (u = 34,3 nm, U = 69 nm).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir sıcaklık blok kalibratörü (Başlangıç)

180 °C'deki bir kuru bloğun sıcaklığı bir referans termometreyle ölçülür. Bütçe bloğun kendi etkilerini listeler: eksenel ve radyal homojensizlik, kararlılık, yükleme.

<a id="ea402-s11-block-calibrator-180c"></a>

### Sıcaklık blok kalibratörünün 180 °C'de kalibrasyonu

`ea402-s11-block-calibrator-180c`

#### Bu proje ne hakkında?

EA-4/02 S11: bloğun sıcaklığının kalibre edilmiş platin direnç termometresiyle ölçülmesi (değerler mK).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **t_X** (mK): Bloğun sıcaklığı (mK). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
t_X = t_S + dt_S + dt_D - dt_iX + dt_R + dt_A + dt_H + dt_V
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **t_S**: Referans termometre okuması. En iyi değer 180100 mK. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 30 mK, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 30/2 = 15,0 mK olur (normal dağılım).
2. **dt_S**: Referans termometrenin kalibrasyonu. En iyi değer 0 mK, standart belirsizlik u = 10 mK (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **dt_D**: Referans termometrenin sürüklenmesi. Yalnızca sınırlar biliniyor: değer -40 ile 40 mK arasında (0 ± 40) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 40/√3 = 23,1 mK.
4. **dt_iX**: Kalibratör ekranının çözünürlüğü. Yalnızca sınırlar biliniyor: değer -50 ile 50 mK arasında (0 ± 50) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 50/√3 = 28,9 mK.
5. **dt_R**: Radyal sıcaklık farkları. Yalnızca sınırlar biliniyor: değer -100 ile 100 mK arasında (0 ± 100) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 100/√3 = 57,7 mK.
6. **dt_A**: Eksenel homojensizlik. Yalnızca sınırlar biliniyor: değer -250 ile 250 mK arasında (0 ± 250) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 250/√3 = 144 mK.
7. **dt_H**: Histerezis. Yalnızca sınırlar biliniyor: değer -50 ile 50 mK arasında (0 ± 50) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 50/√3 = 28,9 mK.
8. **dt_V**: Ölçüm sırasında sıcaklık değişimi. Yalnızca sınırlar biliniyor: değer -30 ile 30 mK arasında (0 ± 30) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 30/√3 = 17,3 mK.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 180100 mK
- standart belirsizlik u = 164 mK

Kaynak: EA-4/02 M:2022 S11.9–S11.11 (u = 164 mK).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir su sayacının kalibrasyonu (Başlangıç)

Üç ilişkili proje: referans hacim, bir koşunun bağıl hatası ve üç koşunun ortalama hatası. Uzun bir hesabın her biri kendi bütçesine sahip adımlara nasıl bölünebileceğini gösterirler.

<a id="ea402-s12-water-meter-error"></a>

### Su sayacı kalibrasyonu: tek çalışmanın bağıl hatası

`ea402-s12-water-meter-error`

#### Bu proje ne hakkında?

EA-4/02 S12 (ikinci bütçe): tek bir çalışma için sayacın bağıl gösterge hatası.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **e_X**: Bağıl gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
e_X = (V_iX + dV_iX2 - dV_iX1)/V_X - 1
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V_iX**: Gösterilen hacim. Tam bilinen bir değer, 200 L; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dV_iX1**: Başlangıçta çözünürlük. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 L arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 L.
3. **dV_iX2**: Bitişte çözünürlük. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 L arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 L.
4. **V_X**: Sayaçtaki hacim (birinci bütçe). En iyi değer 199,93 L, standart belirsizlik u = 0,109 L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,0003
- standart belirsizlik u = 0,00068

Kaynak: EA-4/02 M:2022 S12.10 (e_X = 0,0003, u = 0,68 × 10⁻³).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="ea402-s12-water-meter-mean-error"></a>

### Su sayacı kalibrasyonu: üç çalışmanın ortalaması

`ea402-s12-water-meter-mean-error`

#### Bu proje ne hakkında?

EA-4/02 S12 (üçüncü bütçe): üç çalışmanın ortalama bağıl hatası; Welch–Satterthwaite formülü ve t dağılımından k.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **e_Xav**: Ortalama bağıl gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
e_Xav = e_X + de_X
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **e_X**: Tek çalışmanın hatası (belirsizlik ikinci bütçeden). En iyi değer 0, standart belirsizlik u = 0,00068132 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **de_X**: Üç çalışmanın bağıl hataları (A tipi). 3 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 0,00100000; okumalar s = 0,00104 standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,00104/√3 = 0,000603, serbestlik derecesi ν = n − 1 = 2.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- standart belirsizlik u = 0,00091
- kapsam faktörü k = 2,28
- genişletilmiş belirsizlik U = 0,002

Kaynak: EA-4/02 M:2022 S12.11–S12.13 (u = 0,91 × 10⁻³, ν_eff = 10, k = 2,28, U = 0,002).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="ea402-s12-water-meter-volume"></a>

### Su sayacı kalibrasyonu: referans hacim

`ea402-s12-water-meter-volume`

#### Bu proje ne hakkında?

EA-4/02 S12 (birinci bütçe): 2500 L/h'te bir konut tipi su sayacından geçen hacmin hacimsel standartta ölçülmesi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **V_X** (L): Sayaçtaki hacim. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
V_X = (V_iS + dV_iS)*(1 + beta_S*(t_S - 20))*(1 + gamma_W*(t_X - t_S))*(1 - kappa_W*p_X)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V_iS**: Hacimsel standardın göstergesi. En iyi değer 200 L. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,2 L, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,2/2 = 0,100 L olur (normal dağılım).
2. **dV_iS**: Standardın okunması. Yalnızca sınırlar biliniyor: değer -0,02 ile 0,02 L arasında (0 ± 0,02) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,02/√3 = 0,0115 L.
3. **beta_S**: Standardın hacimsel genleşme katsayısı. Yalnızca sınırlar biliniyor: değer 0,0000505 ile 0,0000515 1/K arasında (0,000051 ± 5e-7) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5e-7/√3 = 2,89 × 10⁻⁷ 1/K.
4. **t_S**: Standarttaki su sıcaklığı. Yalnızca sınırlar biliniyor: değer 13 ile 17 °C arasında (15 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 °C.
5. **gamma_W**: Suyun genleşme katsayısı. Yalnızca sınırlar biliniyor: değer 0,00014500 ile 0,000155 1/K arasında (0,00015 ± 0,000005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000005/√3 = 2,89 × 10⁻⁶ 1/K.
6. **t_X**: Sayaçtaki su sıcaklığı. Yalnızca sınırlar biliniyor: değer 14 ile 18 °C arasında (16 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 °C.
7. **kappa_W**: Suyun sıkıştırılabilirliği. Yalnızca sınırlar biliniyor: değer 4,55e-7 ile 4,65e-7 1/kPa arasında (4,6e-7 ± 5e-9) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5e-9/√3 = 2,89 × 10⁻⁹ 1/kPa.
8. **p_X**: Sayaçtaki gösterge basıncı. Yalnızca sınırlar biliniyor: değer 450 ile 550 kPa arasında (500 ± 50) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 50/√3 = 28,9 kPa.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 199,93 L
- standart belirsizlik u = 0,109 L

Kaynak: EA-4/02 M:2022 S12.9 (V_X = 199,93 L, u = 0,109 L).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## UKAS M3003'te bir direnç (Başlangıç)

Gerilimler karşılaştırılarak kalibre edilen bir 10 kΩ direnç. Kaynakları, değerlerini, bölenleri (√3, 2, 1) ve sonuçtaki standart belirsizlikleri içeren bir bütçe tablosunun açık bir örneği.

<a id="m3003-k1-resistor-10kohm"></a>

### Gerilim karşılaştırmasıyla 10 kΩ direnç (UKAS M3003 K.1)

`m3003-k1-resistor-10kohm`

#### Bu proje ne hakkında?

Seri bağlı standart ve bilinmeyen 10 kΩ dirençler üzerindeki gerilimler tek bir DVM ile karşılaştırılır; her iki direnç 20,0 °C ± 0,1 °C yağ banyosundadır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **R_X** (Ω): Bilinmeyen direncin değeri. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
R_X = (R_S + dR_D + R_TC*dt) * r * (1 + d_VX) / (1 + d_VS)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R_S**: Standart direnç (sertifika, ±0,5 ppm, k = 2). En iyi değer 10000 Ω. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,005 Ω, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,005/2 = 0,00250 Ω olur (normal dağılım).
2. **dR_D**: Kalibrasyondan beri standardın sürüklenmesi (±0,5 ppm). Yalnızca sınırlar biliniyor: değer -0,005 ile 0,005 Ω arasında (0 ± 0,005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,005/√3 = 0,00289 Ω.
3. **R_TC**: R_S'nin sıcaklık katsayısı (2,5 ppm/°C). Tam bilinen bir değer, 0,025 Ω/°C; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
4. **dt**: Yağ banyosunun 20,0 °C'den sapması. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 °C arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 °C.
5. **d_VS**: DVM doğrusallığı ve çözünürlüğü, R_S üzerindeki okuma (göreli). Yalnızca sınırlar biliniyor: değer -2e-7 ile 2e-7 arasında (0 ± 2e-7) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2e-7/√3 = 1,15 × 10⁻⁷.
6. **d_VX**: DVM doğrusallığı ve çözünürlüğü, R_X üzerindeki okuma (göreli). Yalnızca sınırlar biliniyor: değer -2e-7 ile 2e-7 arasında (0 ± 2e-7) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2e-7/√3 = 1,15 × 10⁻⁷.
7. **r**: V_X/V_S gerilim oranı, beş okuma. 5 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 1,00001; okumalar s = 1,58 × 10⁻⁷ standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 1,58 × 10⁻⁷/√5 = 7,07 × 10⁻⁸, serbestlik derecesi ν = n − 1 = 4.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 10000,105 Ω
- standart belirsizlik u = 0,00445 Ω
- genişletilmiş belirsizlik U = 0,0089 Ω

Kaynak: UKAS M3003 6. baskı K.1.8–K.1.9: u_c = 0,445 ppm, U = 0,891 ppm; 10 000,1050 Ω ± 0,0089 Ω.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## 200 g'da bir tartı aleti (Başlangıç)

Bir tartı aleti 200 g'lık bir ağırlıkla denetlenir. Başlıca terimler tekrarlanabilirlik, çözünürlük ve ağırlığın sertifikasıdır.

<a id="m3003-k5-weighing-machine-200g"></a>

### 200 g'de 205 g kapasiteli tartı aleti (UKAS M3003 K.5)

`m3003-k5-weighing-machine-200g`

#### Bu proje ne hakkında?

0,1 mg basamaklı elektronik terazinin E2 sınıfı 200 g ağırlıkla gösterge hatası; tek okuma.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dI_X** (mg): 200 g'de gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dI_X = I_X - W_S + dD_S + dI_d0 + dI_d + dA_b + dI_r
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_X**: Gösterge (199,9999 g). Tam bilinen bir değer, 199999,9 mg; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **W_S**: Standart ağırlık (U = 0,1 mg, k = 2). En iyi değer 200000 mg. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,1 mg, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,1/2 = 0,0500 mg olur (normal dağılım).
3. **dD_S**: Standardın sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 mg arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 mg.
4. **dI_d0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 mg arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 mg.
5. **dI_d**: Göstergenin yuvarlanması. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 mg arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 mg.
6. **dA_b**: Hava kaldırma kuvveti (±1 ppm). Yalnızca sınırlar biliniyor: değer -0,2 ile 0,2 mg arasında (0 ± 0,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,2/√3 = 0,115 mg.
7. **dI_r**: Tekrarlanabilirlik (s = 0,05 mg, ν = 9, tek okuma). 1 okumanın ortalaması, 0 mg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,05 mg (ν = 9) kullanılır; böylece u = s_p/√n = 0,0500 mg.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = -0,1 mg
- standart belirsizlik u = 0,15275 mg
- genişletilmiş belirsizlik U = 0,3055 mg

Kaynak: K.5.9'daki katkılardan yeniden hesaplanmıştır (araştırma günlüğü F56). M3003 u_c = 0,150 mg, U = 0,300 mg ve 0,10 mg ± 0,30 mg sonucunu verir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## 2 MPa'da bir basınç göstergesi (Başlangıç)

Bir dijital basınç göstergesi bir ölü ağırlık test cihazıyla karşılaştırılır. Tipik bir basınç kalibrasyonu bütçesi.

<a id="m3003-k8-pressure-indicator-2mpa"></a>

### Ölü ağırlık test cihazına karşı 2 MPa'da dijital basınç göstergesi (UKAS M3003 K.8)

`m3003-k8-pressure-indicator-2mpa`

#### Bu proje ne hakkında?

2 MPa'da bir dijital basınç göstergesinin hatası; bütçe, basınçta alışıldığı gibi göreli (ppm) olarak verilir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **e** (ppm): Dijital basınç göstergesinin göreli gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
e = 1e6*(I_X - P_X)/P_X - (A_cal + a_p + A_drift + m_cal + m_drift + lambda*dt + g + air + head + fluid + level) + d_res + d_rep
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 16 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_X**: Gösterge. Tam bilinen bir değer, 2,00084 MPa; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **P_X**: Üretilen basınç. Tam bilinen bir değer, 2,000806 MPa; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
3. **A_cal**: Etkin alan, kalibrasyon (U = 32,2 ppm, k = 2). En iyi değer 0 ppm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 32,2 ppm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 32,2/2 = 16,1 ppm olur (normal dağılım).
4. **a_p**: 2 MPa'da deformasyon katsayısı (U = 1,0 ppm, k = 2). En iyi değer 0 ppm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 1 ppm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 1/2 = 0,500 ppm olur (normal dağılım).
5. **A_drift**: Etkin alanın sürüklenmesi. Yalnızca sınırlar biliniyor: değer -30 ile 30 ppm arasında (0 ± 30) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 30/√3 = 17,3 ppm.
6. **m_cal**: Toplam yükün kalibrasyonu (U = 7,3 ppm, k = 2). En iyi değer 0 ppm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 7,3 ppm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 7,3/2 = 3,65 ppm olur (normal dağılım).
7. **m_drift**: Toplam yükün sürüklenmesi. Yalnızca sınırlar biliniyor: değer -7,6 ile 7,6 ppm arasında (0 ± 7,6) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,6/√3 = 4,39 ppm.
8. **lambda**: Piston ve silindirin sıcaklık katsayısı. Tam bilinen bir değer, 23 ppm/°C; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
9. **dt**: Piston sıcaklığı. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 °C arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 °C.
10. **g**: Yerel yerçekimi (U = 3 ppm, k = 2). En iyi değer 0 ppm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 3 ppm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 3/2 = 1,50 ppm olur (normal dağılım).
11. **air**: Hava kaldırma kuvveti yaklaşımı. Yalnızca sınırlar biliniyor: değer -13 ile 13 ppm arasında (0 ± 13) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 13/√3 = 7,51 ppm.
12. **head**: Sıvı yükü (3 mm yağ). Yalnızca sınırlar biliniyor: değer -13,5 ile 13,5 ppm arasında (0 ± 13,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 13,5/√3 = 7,79 ppm.
13. **fluid**: Kaldırma hacmi, yüzey gerilimi, sıvı yoğunluğu. Yalnızca sınırlar biliniyor: değer -2 ile 2 ppm arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 ppm.
14. **level**: Pistonun düşeyliği (0,15°). Yalnızca sınırlar biliniyor: değer -3,4 ile 3,4 ppm arasında (0 ± 3,4) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 3,4/√3 = 1,96 ppm.
15. **d_res**: Göstergenin yuvarlanması (200 Pa adım). Yalnızca sınırlar biliniyor: değer -50 ile 50 ppm arasında (0 ± 50) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 50/√3 = 28,9 ppm.
16. **d_rep**: Tekrarlanabilirlik (s = 16 ppm, ν = 9, tek okuma). 1 okumanın ortalaması, 0 ppm. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 16 ppm (ν = 9) kullanılır; böylece u = s_p/√n = 16,0 ppm.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 17 ppm
- standart belirsizlik u = 43 ppm
- genişletilmiş belirsizlik U = 86 ppm

Kaynak: UKAS M3003 6. baskı K.8.16–K.8.17: u_c = 43,0 ppm, U = 86,0 ppm; 17 ppm ± 86 ppm.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir levhanın kalınlığı (Başlangıç)

Bir alüminyum numunenin kalınlığı beş okumanın ortalamasıdır. A tipi bir değerlendirmeyi (okumaların dağılımı) B tipi terimlerle birleştirir: kalibrasyon, çözünürlük, kayma ve sıcaklık.

<a id="m3003-k10-sheet-thickness"></a>

### Alüminyum deney numunesinin kalınlığı (UKAS M3003 K.10)

`m3003-k10-sheet-thickness`

#### Bu proje ne hakkında?

Bir numunenin merkezinde beş kalınlık okumasının ortalaması, kontrolsüz bir ortamda; kalibrasyon düzeltmesi uygulanmaz.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w** (mm): Numunenin merkezindeki kalınlık. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w = w_ave + d_res + d_cal + d_drift + d_unc + c_T*d_T
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **w_ave**: Gösterilen kalınlık, beş okuma. 5 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 1,51400 mm; okumalar s = 0,0114 mm standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,0114/√5 = 0,00510 mm, serbestlik derecesi ν = n − 1 = 4.
2. **d_res**: Çözünürlük. Yalnızca sınırlar biliniyor: değer -0,005 ile 0,005 mm arasında (0 ± 0,005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,005/√3 = 0,00289 mm.
3. **d_cal**: Kalibrasyon (U = 0,015 mm, k = 2). En iyi değer 0 mm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,015 mm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,015/2 = 0,00750 mm olur (normal dağılım).
4. **d_drift**: Kalibrasyon sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,00233 ile 0,00233 mm arasında (0 ± 0,00233) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00233/√3 = 0,00135 mm.
5. **d_unc**: Düzeltilmemiş kalibrasyon hatası. Yalnızca sınırlar biliniyor: değer -0,001 ile 0,001 mm arasında (0 ± 0,001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,001/√3 = 0,000577 mm.
6. **c_T**: Sıcaklık duyarlılığı. Tam bilinen bir değer, 0,0000357 mm/K; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
7. **d_T**: Kalibrasyon sıcaklığından sapma. Yalnızca sınırlar biliniyor: değer -10 ile 10 K arasında (0 ± 10) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 10/√3 = 5,77 K.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,514 mm
- standart belirsizlik u = 0,00963 mm
- genişletilmiş belirsizlik U = 0,019 mm

Kaynak: UKAS M3003 6. baskı K.10.8–K.10.9: u_c = 0,009 63 mm, ν_eff = 50, U = 0,019 mm; 1,514 mm ± 0,019 mm.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Hacimsel cam malzemenin kalibrasyonu (Başlangıç)

Bir balonjojenin ya da piknometrenin hacmi, aldığı suyun tartılmasıyla bulunur. Suyun ve havanın yoğunluğunun, sıcaklığın ve terazinin sonuca nasıl girdiğini görürsünüz.

<a id="cg19-ex1-flask-1000ml"></a>

### 1000 mL'lik ölçülü balonun gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 1)

`cg19-ex1-flask-1000ml`

#### Bu proje ne hakkında?

İşaretine kadar doldurulup on kez tartılan 1000 mL'lik cam balon; su 19,11 °C, hava 0,2 °C daha sıcak.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **V_20** (mL): 20 °C'deki hacim. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
V_20 = m/(rho_W - rho_A)*(1 - rho_A/rho_B)*(1 - gamma*(t_W + d_ts - 20)) + dV_men + dV_rep
```

Denklemi okunur tutmak için önce 2 ara büyüklük hesaplanır:

- `t_W = t_W0 + d_res`
- `rho_W = 0.99997495*(1 - (t_W - 3.983035)^2*(t_W + 301.797)/(522528.9*(t_W + 69.34881))) + d_form + d_pur`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 11 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m**: Su kütlesi, I_L − I_E (her iki tartım için terazi U ve çözünürlüğü). En iyi değer 996,9505 g, standart belirsizlik u = 0,00127 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **t_W0**: Ölçülen su sıcaklığı (termometre U = 0,009 °C, k = 2). En iyi değer 19,11 °C. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,009 °C, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,009/2 = 0,00450 °C olur (normal dağılım).
3. **d_res**: Termometrenin çözünürlüğü (0,001 °C). Yalnızca sınırlar biliniyor: değer -0,0005 ile 0,0005 °C arasında (0 ± 0,0005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0005/√3 = 0,000289 °C.
4. **d_ts**: Su ile alet sıcaklığı arasındaki fark. Yalnızca sınırlar biliniyor: değer -0,025 ile 0,025 °C arasında (0 ± 0,025) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,025/√3 = 0,0144 °C.
5. **d_form**: Tanaka formülünün belirsizliği. En iyi değer 0 g/mL, standart belirsizlik u = 4,5e-7 g/mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **d_pur**: Suyun saflığı (5 ppm). Yalnızca sınırlar biliniyor: değer -4,9920 × 10⁻⁶ ile 4,9920 × 10⁻⁶ g/mL arasında (0 ± 4,9920 × 10⁻⁶) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 4,9920 × 10⁻⁶/√3 = 2,88 × 10⁻⁶ g/mL.
7. **rho_A**: Hava yoğunluğu (u örnekte basıldığı gibi). En iyi değer 0,0012 g/mL, standart belirsizlik u = 0,00000176 g/mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **rho_B**: Referans ağırlıkların yoğunluğu (±0,06 g/mL, özet tablodaki gibi u = 0,0346 g/mL). Yalnızca sınırlar biliniyor: değer 7,94 ile 8,06 g/mL arasında (8 ± 0,06) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,06/√3 = 0,0346 g/mL.
9. **gamma**: Camın hacimsel genleşme katsayısı (±%5). Yalnızca sınırlar biliniyor: değer 0,0000095 ile 1,0500 × 10⁻⁵ 1/°C arasında (0,00001 ± 5e-7) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5e-7/√3 = 2,89 × 10⁻⁷ 1/°C.
10. **dV_men**: Menisküs okuma. Yalnızca sınırlar biliniyor: değer -0,015 ile 0,015 mL arasında (0 ± 0,015) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,015/√3 = 0,00866 mL.
11. **dV_rep**: Tekrarlanabilirlik (s(V₀) = 0,044 mL, 10 ölçümün ortalaması). 10 okumanın ortalaması, 0 mL. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,044 mL (ν = 9) kullanılır; böylece u = s_p/√n = 0,0139 mL.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 999,628 mL
- standart belirsizlik u = 0,017 mL
- kapsam faktörü k = 2,15
- genişletilmiş belirsizlik U = 0,036 mL

Kaynak: EURAMET cg-19 v4.1 7.1, Tablo 2: V₂₀ = 999,628 mL, u = 0,017 mL, ν_eff = 19, k = 2,15, U = 0,036 mL. Tablo 1 yoğunlukları yuvarlanmış verdiğinden (0,9984 ve 0,0012 g/mL) tahmin 0,01 mL içinde karşılaştırılır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg19-ex2-pycnometer-100ml"></a>

### 100 mL'lik cam piknometrenin gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 2)

`cg19-ex2-pycnometer-100ml`

#### Bu proje ne hakkında?

On kez tartılan 100 mL'lik piknometre; su 18,94 °C, hava 0,23 °C daha sıcak; menisküs katkısı ihmal edilebilir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **V_20** (mL): 20 °C'deki hacim. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
V_20 = m/(rho_W - rho_A)*(1 - rho_A/rho_B)*(1 - gamma*(t_W + d_ts - 20)) + dV_men + dV_rep
```

Denklemi okunur tutmak için önce 2 ara büyüklük hesaplanır:

- `t_W = t_W0 + d_res`
- `rho_W = 0.99997495*(1 - (t_W - 3.983035)^2*(t_W + 301.797)/(522528.9*(t_W + 69.34881))) + d_form + d_pur`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 11 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m**: Su kütlesi, I_L − I_E (her iki tartım için terazi U ve çözünürlüğü). En iyi değer 99,75 g, standart belirsizlik u = 0,000114 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **t_W0**: Ölçülen su sıcaklığı (termometre U = 0,009 °C, k = 2). En iyi değer 18,94 °C. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,009 °C, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,009/2 = 0,00450 °C olur (normal dağılım).
3. **d_res**: Termometrenin çözünürlüğü (0,001 °C). Yalnızca sınırlar biliniyor: değer -0,0005 ile 0,0005 °C arasında (0 ± 0,0005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0005/√3 = 0,000289 °C.
4. **d_ts**: Su ile alet sıcaklığı arasındaki fark. Yalnızca sınırlar biliniyor: değer -0,02875 ile 0,02875 °C arasında (0 ± 0,02875) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,02875/√3 = 0,0166 °C.
5. **d_form**: Tanaka formülünün belirsizliği. En iyi değer 0 g/mL, standart belirsizlik u = 4,5e-7 g/mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **d_pur**: Suyun saflığı (5 ppm). Yalnızca sınırlar biliniyor: değer -4,9920 × 10⁻⁶ ile 4,9920 × 10⁻⁶ g/mL arasında (0 ± 4,9920 × 10⁻⁶) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 4,9920 × 10⁻⁶/√3 = 2,88 × 10⁻⁶ g/mL.
7. **rho_A**: Hava yoğunluğu (u örnekte basıldığı gibi). En iyi değer 0,0012 g/mL, standart belirsizlik u = 0,00000174 g/mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **rho_B**: Referans ağırlıkların yoğunluğu (±0,06 g/mL, özet tablodaki gibi u = 0,0346 g/mL). Yalnızca sınırlar biliniyor: değer 7,94 ile 8,06 g/mL arasında (8 ± 0,06) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,06/√3 = 0,0346 g/mL.
9. **gamma**: Camın hacimsel genleşme katsayısı (±%5). Yalnızca sınırlar biliniyor: değer 0,0000095 ile 1,0500 × 10⁻⁵ 1/°C arasında (0,00001 ± 5e-7) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5e-7/√3 = 2,89 × 10⁻⁷ 1/°C.
10. **dV_men**: Menisküs okuma (ihmal edilebilir). Tam bilinen bir değer, 0 mL; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
11. **dV_rep**: Tekrarlanabilirlik (s(V₀) = 0,0026 mL, 10 ölçümün ortalaması). 10 okumanın ortalaması, 0 mL. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,0026 mL (ν = 9) kullanılır; böylece u = s_p/√n = 0,000822 mL.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 100,014 mL
- standart belirsizlik u = 0,00089 mL
- kapsam faktörü k = 2,21
- genişletilmiş belirsizlik U = 0,002 mL

Kaynak: EURAMET cg-19 v4.1 7.2, Tablo 4: V₂₀ = 100,014 mL, u = 0,89 µL, ν_eff = 13 (7.2.5), k = 2,21, U = 2,0 µL. Örnek ν_eff = 12,9 değerini 13'e yuvarlar; GUM G.6.4 onu 12'ye keser, bu da k = 2,23 ve aynı U = 2,0 µL verir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir kalibrasyon standardı hazırlamak (Başlangıç)

Tartılmış bir miktar saf metal bilinen bir hacimde çözülerek bir kadmiyum çözeltisi hazırlanır. Eurachem kılavuzunun klasik ilk örneği: model bir çarpım ve oran olduğu için saflık, kütle ve hacim bağıl belirsizlikler olarak birleşir.

<a id="quam-a1-cadmium-standard-volume"></a>

### Hacim alt modelli kadmiyum standardı

`quam-a1-cadmium-standard-volume`

#### Bu proje ne hakkında?

Balon hacmi üç bileşeninden modellenmiş QUAM Örnek A1.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_Cd** (mg/L): Kalibrasyon standardının derişimi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_Cd = 1000 * m * P / V
```

Denklemi okunur tutmak için önce 1 ara büyüklük hesaplanır:

- `V = V_nom + dV_cal + dV_rep + dV_temp`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **P**: Kadmiyum metalinin saflığı (sertifika: %99,99 ± 0,01). Yalnızca sınırlar biliniyor: değer 0,9998 ile 1 arasında (0,9999 ± 0,0001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0001/√3 = 5,77 × 10⁻⁵.
2. **m**: Kadmiyumun kütlesi (fark tartımı). En iyi değer 100,28 mg, standart belirsizlik u = 0,05 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **V_nom**: Anma hacmi. Tam bilinen bir değer, 100 mL; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
4. **dV_cal**: Balon kalibrasyonu (±0,1 mL). Değer -0,1 ile 0,1 mL arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,1/√6 = 0,0408 mL.
5. **dV_rep**: Doldurma tekrarlanabilirliği. En iyi değer 0 mL, standart belirsizlik u = 0,02 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **dV_temp**: Sıcaklık etkisi (±4 °C). Yalnızca sınırlar biliniyor: değer -0,084 ile 0,084 mL arasında (0 ± 0,084) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,084/√3 = 0,0485 mL.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1002,7 mg/L
- genişletilmiş belirsizlik U = 1,7 mg/L

Kaynak: QUAM:2012 A1.4: u(V) = 0,066 mL; rehberin u_c'yi 0,9 mg/L'ye yuvarlamasından önce U = 2u_c = 1,7 mg/L.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="quam-a1-cadmium-standard"></a>

### Kadmiyum kalibrasyon standardının hazırlanması

`quam-a1-cadmium-standard`

#### Bu proje ne hakkında?

EURACHEM/CITAC QUAM Örnek A1: yüksek saflıkta metalden 1000 mg/L Cd standardı (özet bütçe).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_Cd** (mg/L): Kalibrasyon standardının derişimi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_Cd = 1000 * m * P / V
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **P**: Kadmiyum metalinin saflığı (sertifika: %99,99 ± 0,01). Yalnızca sınırlar biliniyor: değer 0,9998 ile 1 arasında (0,9999 ± 0,0001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0001/√3 = 5,77 × 10⁻⁵.
2. **m**: Kadmiyumun kütlesi (fark tartımı). En iyi değer 100,28 mg, standart belirsizlik u = 0,05 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **V**: Balonun hacmi. En iyi değer 100 mL, standart belirsizlik u = 0,07 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1002,7 mg/L
- standart belirsizlik u = 0,8633 mg/L

Kaynak: QUAM:2012 Tablo A1.3 (tablolama değeri u = 0,8633 mg/L).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Referans malzemeden ya da yeterlilik testlerinden sapma (Başlangıç)

Belirsizliğin sistematik kısmını bulmanın yukarıdan aşağı yolu: sertifikalı bir referans malzemeyi ölçün ya da yeterlilik testlerindeki sonuçlarınıza bakın ve farkları u(sapma)'ya dönüştürün. Tek CRM, birkaç CRM, yeterlilik testi ve geri kazanım türevlerini karşılaştırın.

<a id="nordtest-6.1-one-crm"></a>

### Tek sertifikalı referans malzemeden sapma

`nordtest-6.1-one-crm`

#### Bu proje ne hakkında?

Nordtest TR 537 §6.1: 12 kez ölçülmüş bir CRM'den (11,5 ± 0,5) u(bias).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c** için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §6.1, yeniden hesaplanmış (el kitabı %4,1 yazar, notlara bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nordtest-6.1-three-crms"></a>

### Birden fazla sertifikalı referans malzemeden sapma

`nordtest-6.1-three-crms`

#### Bu proje ne hakkında?

Nordtest TR 537 §6.1: üç CRM'den sapmaların karesel ortalamasının karekökü (RMS) ile u(bias).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c** için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §6.1 (RMS_bias %2,53, ū(C_ref) %1,92, u(bias) %3,2).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nordtest-6.2-proficiency-tests"></a>

### Yeterlilik testlerinden sapma

`nordtest-6.2-proficiency-tests`

#### Bu proje ne hakkında?

Nordtest TR 537 §6.2: altı yeterlilik testinden u(bias).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c** için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §6.2 (RMS_bias %4,6, ū(C_ref) %2,6, u(bias) %5,3).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nordtest-6.3-recovery"></a>

### Geri kazanım deneylerinden sapma

`nordtest-6.3-recovery`

#### Bu proje ne hakkında?

Nordtest TR 537 §6.3: altı katkı geri kazanımından u(bias).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c** için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §6.3 (RMS_bias %3,44, u(C_geri kazanım) %1,0, u(bias) %3,6).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Çift ölçümlerden kesinlik (Başlangıç)

Numuneleri çift analiz etmek tekrarlanabilirliği verir; aylar boyunca ölçülen bir kontrol numunesi laboratuvar içi tekrar üretilebilirliği verir. Bunlar yukarıdan aşağı belirsizliğin rastgele kısmıdır.

<a id="nordtest-5.3-ammonium-duplicates"></a>

### Amonyum: kontrol numuneleri ve mükerrer ölçümlerden laboratuvar içi tekrar üretilebilirlik

`nordtest-5.3-ammonium-duplicates`

#### Bu proje ne hakkında?

Nordtest TR 537 §5.3 ve Ek 5: doğal numunelerin iki aralıktaki NH₄-N mükerrer ölçümleri (ham veri), kontrol numunesi tekrar üretilebilirliğiyle birleştirilmiştir.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c(NH₄-N)** (µg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 10; 100 µg/L.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **u(Rw), control < 30**: standart sapma 0,5 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_r, duplicates < 30**: 46 çift ölçüm; standart sapma çiftlerin içindeki farklardan bulunur.
- **u(Rw), control ≥ 30**: standart sapma 1,5 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_r, duplicates ≥ 30**: 26 çift ölçüm; standart sapma çiftlerin içindeki farklardan bulunur.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §5.3 ve Ek 5 (s_r 0,44 µg/L ve %3,8; u(Rw) 0,7 µg/L ve %4,1).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nordtest-5.3-oxygen-duplicates"></a>

### Çözünmüş oksijen: mükerrer ölçümlerden tekrar üretilebilirlik

`nordtest-5.3-oxygen-duplicates`

#### Bu proje ne hakkında?

Nordtest TR 537 §5.3 ve Ek 6: deniz suyunda çözünmüş oksijen, 51 mükerrer çift (ham veri) ve kontrol numunesi bileşeni.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c(O₂)** (mg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 2 kayıt:

- **s_r, duplicates**: 51 çift ölçüm; standart sapma çiftlerin içindeki farklardan bulunur.
- **u(Rw), control**: standart sapma 0,5 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §5.3 ve Ek 6 (s = 0,0252 mg/L, %0,34; u(Rw) = %0,60).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Tek örnekte bütün yukarıdan aşağı bütçe (Başlangıç)

Sudaki amonyum: kontrol grafiğinden kesinlik ve yeterlilik testlerinden sapma, genişletilmiş belirsizlikte birleştirilir. Çoğu deney laboratuvarının kullandığı tarif budur.

<a id="nordtest-3-ammonium-pt"></a>

### Kontrol sınırları ve yeterlilik testlerinden sudaki amonyum

`nordtest-3-ammonium-pt`

#### Bu proje ne hakkında?

Nordtest TR 537 §3.2 ve Ek 4: NH₄-N; u(Rw) kontrol grafiğinden, u(bias) altı yeterlilik testinden (ham YT verisi).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c(NH₄-N)** (µg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **u(Rw)**: control_limits

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §3.2 ve Ek 4 (RMS_bias %2,26, ū(C_ref) %1,52, u(bias) %2,73, u_c %3,20, U %6,4 → %7).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## İlk kontrol grafiği (Başlangıç)

Sudaki çinkonun altmış kontrol değeri uyarı ve eylem sınırlarıyla çizilir. Bir grafiği okumayı öğrenin: hangi noktalar sorunsuz, hangileri uyarı, hangileri eylem gerektiriyor.

<a id="tr569-table1-zinc"></a>

### Sudaki çinko: 60 kontrol değerinden X-grafiği (Nordtest TR 569 Tablo 1)

`tr569-table1-zinc`

#### Bu proje ne hakkında?

60,0 µg/L çinko çözeltisinin altmış kontrol değeri (Tablo 1), Şekil 1'de çizilmiştir. Sınırlar değerlerin kendisinden istatistiksel olarak belirlenir; her seri günlük olarak değerlendirilir.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

60 kontrol değeri girilmiş.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Tablo 1 değerleri; ortalama, s ve sınırlar bunlardan hesaplanmıştır (TR 569 grafiği, Şekil 1, sayılar olmadan verir); Şekil 1'deki bölgeler uyumludur.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Kontrol grafiği sınırlarını belirlemek (Başlangıç)

Sınırları belirlemenin üç yolu: verilerden (istatistiksel), bir hedeften ya da bir referans değer etrafında. Hedef sınırların neden çoğu zaman daha geniş ve daha kullanışlı olduğunu görmek için türevleri karşılaştırın.

<a id="tr569-ch7-case1-statistical"></a>

### X-grafiğinin sınırlarının belirlenmesi, durum 1 (Nordtest TR 569 Bölüm 7)

`tr569-ch7-case1-statistical`

#### Bu proje ne hakkında?

Durum 1: istatistiksel sınırlar ve ortalama merkez çizgisi, 59,2 µg/L'nin %4'ü s_Rw.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: istatistiksel, kontrol değerlerinin kendi dağılımından.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Bölüm 7, 0,1 µg/L'ye yuvarlanmış değerler.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ch7-case2-target-mean"></a>

### X-grafiğinin sınırlarının belirlenmesi, durum 2 (Nordtest TR 569 Bölüm 7)

`tr569-ch7-case2-target-mean`

#### Bu proje ne hakkında?

Durum 2: gerekli %5 s_Rw'den hedef sınırlar, ortalama merkez çizgisi 59,2 µg/L.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: hedef sınırlar, yöntemin ulaşması gereken kesinlikten.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Bölüm 7, 0,1 µg/L'ye yuvarlanmış değerler.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ch7-case3-target-reference"></a>

### X-grafiğinin sınırlarının belirlenmesi, durum 3 (Nordtest TR 569 Bölüm 7)

`tr569-ch7-case3-target-reference`

#### Bu proje ne hakkında?

Durum 3: 60,0 µg/L referans değeri çevresinde %5 hedef sınırlar.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: hedef sınırlar, yöntemin ulaşması gereken kesinlikten.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Bölüm 7, 0,1 µg/L'ye yuvarlanmış değerler.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ch7-range-case1-statistical"></a>

### Paralel tayinler için r%-grafiğinin istatistiksel sınırları (Nordtest TR 569 Bölüm 7)

`tr569-ch7-range-case1-statistical`

#### Bu proje ne hakkında?

Açıklık grafikleri için durum 1: paralel tayinlerden %0,356 birleştirilmiş göreli standart sapma.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: istatistiksel, kontrol değerlerinin kendi dağılımından.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Bölüm 7, durum 1, %0,1'e yuvarlanmış değerler. Metin uyarı sınırını 3·0,356 = %1,0 olarak yazar; Tablo 4'ün katsayısı 2,83'tür (araştırma günlüğü F52). Merkez çizgisi verilmemiştir; program beklenen açıklık d₂·s = %0,40 değerini kullanır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ch7-range-case2-target"></a>

### Tekrarlanabilirlik sınırından r%-grafiğinin hedef sınırları (Nordtest TR 569 Bölüm 7)

`tr569-ch7-range-case2-target`

#### Bu proje ne hakkında?

Açıklık grafikleri için durum 2: standart yöntem r = %1 tekrarlanabilirlik sınırı verir.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: hedef sınırlar, yöntemin ulaşması gereken kesinlikten.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Bölüm 7, durum 2.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir ürünün uygun olma olasılığı (Başlangıç)

Ölçülen değer spesifikasyonun içinde, ama ne kadar emin olabiliriz? Bu örnekler, ölçülen değer ve belirsizliğinden uygunluk olasılığını tek ve iki taraflı sınırlar için hesaplar.

<a id="jcgm106-7.3-bursting-strength"></a>

### Bir kabın patlama dayanımı: uygunluk olasılığı (JCGM 106 7.3)

`jcgm106-7.3-bursting-strength`

#### Bu proje ne hakkında?

JCGM 106:2012 7.3 Örnek 2: tek alt tolerans sınırı B ≥ 490 kPa, tahribatlı deney b = 509,7 kPa, u = 8,6 kPa.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük B en az 490 kPa olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 106:2012 7.3, Örnek 2 (p_c = 0,99).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm106-7.3-zener-diode"></a>

### Zener diyotun kırılma gerilimi: uygunluk olasılığı (JCGM 106 7.3)

`jcgm106-7.3-zener-diode`

#### Bu proje ne hakkında?

JCGM 106:2012 7.3 Örnek 1: tek üst tolerans sınırı V_b ≤ −5,40 V, ölçülen −5,47 V, u = 0,05 V.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük V_b en çok -5,4 V olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 106:2012 7.3, Örnek 1 (p_c = 0,92).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm106-7.4-motor-oil-viscosity"></a>

### Motor yağının kinematik viskozitesi: iki taraflı tolerans (JCGM 106 7.4)

`jcgm106-7.4-motor-oil-viscosity`

#### Bu proje ne hakkında?

JCGM 106:2012 7.4 Örnek: 100 °C'de SAE 40 yağ, 12,5 mm²/s ≤ Y ≤ 16,3 mm²/s, ölçülen 13,6 mm²/s, u = 1,8 mm²/s.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük Y 12,5 ile 16,3 arasında mm²/s olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 106:2012 7.4, Örnek (p_c = 0,66); C_m Formül (12)'den.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Yeterlilik testinin atanan değeri (Başlangıç)

Aynı sonuçlar iki dayanıklı kestiriciyle değerlendirilir: Algoritma A ve medyan. Dayanıklı istatistikler, birkaç aykırı laboratuvarın atanan değeri kaydırmasını önler.

<a id="iso13528-e3-atrazine-algorithm-a"></a>

### İçme suyunda atrazin: Algoritma A ile robust ortalama (ISO 13528 E.3)

`iso13528-e3-atrazine-algorithm-a`

#### Bu proje ne hakkında?

ISO 13528:2022 E.3: 34 katılımcı sonucundan Algoritma A ile atanan değer ve yeterlilik değerlendirmesi için standart sapma; z skorları.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Yeterlilik testinde** aynı malzeme 34 laboratuvara gönderilir ve hepsi **atrazine** ölçer. Sonra her laboratuvar, sonucunun atanan değerle uyuşup uyuşmadığını söyleyen bir puan alır.

Atanan değer: katılımcı sonuçlarının dayanıklı ortalaması (aykırı değerlerin etkisini sınırlayan Algoritma A). Yeterlilik değerlendirmesi için standart sapma σ_pt, bir sonucun atanan değerden ne kadar uzak olabileceğine karar verir.


#### Adım 2. Puanlar

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 başarılı, 2 < |z| < 3 şüpheli, |z| ≥ 3 başarısız.
- **ζ ve E_n**, her laboratuvarın bildirdiği belirsizliği de kullanır; laboratuvarın kendi belirsizliğinin gerçekçi olup olmadığını denetler.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 13528:2022 Tablo E.4 (6 iterasyondan sonra x* = 0,2570, s* = 0,0395 mg/L) ve Tablo E.5 (medyan 0,2620, nIQR 0,0402, MADe 0,0386, u(x_pt) = 0,0085, aritmetik 0,2512 ve 0,0672 mg/L); z skorları bu değerlerden yeniden hesaplanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso13528-e3-atrazine-median"></a>

### İçme suyunda atrazin: medyan ve nIQR (ISO 13528 E.3)

`iso13528-e3-atrazine-median`

#### Bu proje ne hakkında?

ISO 13528:2022 E.3, Tablo E.5'in ilk satırı: atanan değer olarak medyan, u(x_pt) = 1,25 nIQR/√p ve σ_pt olarak nIQR.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Yeterlilik testinde** aynı malzeme 34 laboratuvara gönderilir ve hepsi **atrazine** ölçer. Sonra her laboratuvar, sonucunun atanan değerle uyuşup uyuşmadığını söyleyen bir puan alır.

Atanan değer: katılımcı sonuçlarının medyanı. Yeterlilik değerlendirmesi için standart sapma σ_pt, bir sonucun atanan değerden ne kadar uzak olabileceğine karar verir.


#### Adım 2. Puanlar

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 başarılı, 2 < |z| < 3 şüpheli, |z| ≥ 3 başarısız.
- **ζ ve E_n**, her laboratuvarın bildirdiği belirsizliği de kullanır; laboratuvarın kendi belirsizliğinin gerçekçi olup olmadığını denetler.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 13528:2022 Tablo E.5, 'Robust: Median, nIQR (MADe)' satırı: 0,2620; 0,0402 (0,0386); u(x_pt) = 0,0086 mg/L.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Laboratuvarları puanlamak: z, ζ ve E_n (Başlangıç)

Her katılımcı bir z puanı (sonuç atanan değere yeterince yakın mı?) ve ζ ile E_n puanları (sonuç laboratuvarın beyan ettiği belirsizlikle tutarlı mı?) alır. Standardın tablosu 21 sayısal sonucun hepsi için yeniden üretilir.

<a id="iso13528-e4-mercury-feed"></a>

### Hayvan yeminde cıva: z, z′, ζ ve E_n skorları (ISO 13528 E.4)

`iso13528-e4-mercury-feed`

#### Bu proje ne hakkında?

ISO 13528:2022 E.4 (IMEP 111): bir referans değere ve amaca uygun σ_pt'ye karşı 21 laboratuvarın, bildirdikleri belirsizlikler kullanılarak hesaplanan skorları.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Yeterlilik testinde** aynı malzeme 24 laboratuvara gönderilir ve hepsi **total Hg** ölçer. Sonra her laboratuvar, sonucunun atanan değerle uyuşup uyuşmadığını söyleyen bir puan alır.

Atanan değer: bağımsız olarak belirlenmiş bir referans değer (örneğin bir referans laboratuvarınca ya da sertifikalı referans malzemeden). Yeterlilik değerlendirmesi için standart sapma σ_pt, bir sonucun atanan değerden ne kadar uzak olabileceğine karar verir.


#### Adım 2. Puanlar

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 başarılı, 2 < |z| < 3 şüpheli, |z| ≥ 3 başarısız.
- **ζ ve E_n**, her laboratuvarın bildirdiği belirsizliği de kullanır; laboratuvarın kendi belirsizliğinin gerçekçi olup olmadığını denetler.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 13528:2022 Tablo E.7 (21 sayısal sonucun D %, PA, z, z′, ζ ve E_n değerleri; toleranslar basılı son basamağın yarım birimi). Robust x* ve s*, E.7'den.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Dört derişimin toplamı (Başlangıç)

Plazma anyon açığı dört iyon derişiminin doğrusal bir birleşimidir. Doğrusal bir modelde GUM ve Monte Carlo tam uyuşur: temelleri anladığınızı denetlemek için iyi bir örnek.

<a id="possolo-meija-2022-plasma-anion-gap"></a>

### Plazma anyon açığı: dört iyon derişiminin doğrusal birleşimi (Possolo & Meija 2022)

`possolo-meija-2022-plasma-anion-gap`

#### Bu proje ne hakkında?

Bir hasta için Δc_AG = c(Na⁺) + c(K⁺) − c(Cl⁻) − c(HCO₃⁻) (White 2008); belirsizlikler standart sapma olarak bildirilmiştir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **AG** (mmol/L): Plazma anyon açığı Δc_AG. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
AG = c_Na + c_K - c_Cl - c_HCO3
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **c_Na**: Na⁺ miktar derişimi. En iyi değer 137 mmol/L, standart belirsizlik u = 1,48 mmol/L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **c_K**: K⁺ miktar derişimi. En iyi değer 4 mmol/L, standart belirsizlik u = 0,04 mmol/L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **c_Cl**: Cl⁻ miktar derişimi. En iyi değer 106 mmol/L, standart belirsizlik u = 0,72 mmol/L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **c_HCO3**: HCO₃⁻ miktar derişimi. En iyi değer 10 mmol/L, standart belirsizlik u = 0,84 mmol/L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 25 mmol/L
- standart belirsizlik u = 1,85 mmol/L

Kaynak: Possolo & Meija (2022), s. 17: Δc_AG = 25 mmol/L, u = 1,85 mmol/L.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Suyun sertliği (Başlangıç)

Kalsiyum ve magnezyum derişimlerinden toplam sertlik: IUPAC kılavuzundan kısa, alışılmış bir ölçüm modeli; GUM adımlarını alıştırma yapmak için ideal.

<a id="iupac2024-4a-water-hardness"></a>

### Ca²⁺ ve Mg²⁺'dan suyun toplam sertliği: geleneksel bir ölçüm modeli (IUPAC 2024, Örnek 3C ve 4A)

`iupac2024-4a-water-hardness`

#### Bu proje ne hakkında?

H = γ(Ca²⁺)·M(CaCO₃)/M(Ca) + γ(Mg²⁺)·M(CaCO₃)/M(Mg), uzlaşımla tanımlı; bağımsız girdilerle Gauss formülü.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **H** (mg/L): CaCO₃ kütle derişimi olarak suyun toplam sertliği. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
H = g_Ca*M_CaCO3/M_Ca + g_Mg*M_CaCO3/M_Mg
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **g_Ca**: Ca²⁺ kütle derişimi (AAS). En iyi değer 23,2 mg/L, standart belirsizlik u = 0,8 mg/L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **g_Mg**: Mg²⁺ kütle derişimi (AAS, ayrı ölçülmüş). En iyi değer 9,4 mg/L, standart belirsizlik u = 0,6 mg/L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **M_CaCO3**. Tam bilinen bir değer, 100,0869 g/mol; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
4. **M_Ca**. Tam bilinen bir değer, 40,078 g/mol; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
5. **M_Mg**. Tam bilinen bir değer, 24,305 g/mol; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 96,65 mg/L
- standart belirsizlik u = 3 mg/L

Kaynak: IUPAC 2024, Örnek 4A: u²(H) = 2,5² × 0,8² + 4,1² × 0,6² ≈ (3 mg/L)².

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir titrasyon (Başlangıç)

Sodyum klorür derişimi gümüş nitratla titrasyonla bulunur. Hacimler, titrantın derişimi ve dönüm noktası girdilerdir.

<a id="iupac2024-4i-argentometric-titration"></a>

### Arjantometrik titrasyonla (Mohr) NaCl miktar derişimi: klasik model (IUPAC 2024, Örnek 4I)

`iupac2024-4i-argentometric-titration`

#### Bu proje ne hakkında?

c(NaCl) = c(AgNO₃)·(V(AgNO₃) + ΔV_V + ΔV_E)/(V(NaCl) + ΔV_V); yalnızca hacim ve dönüm noktası belirsizlikleri dikkate alınır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_NaCl** (mmol/L): NaCl miktar derişimi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_NaCl = c_Ag*(V_Ag + dV_Ag + dV_E)/(V_NaCl + dV_NaCl)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **c_Ag**: AgNO₃ titrantının derişimi (kesin alınmıştır). Tam bilinen bir değer, 100 mmol/L; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **V_Ag**: Dönüm noktasında okunan titrant hacmi. Tam bilinen bir değer, 10 mL; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
3. **dV_Ag**: Elektronik büretin çözünürlüğü, ±0,01 mL. Değer -0,01 ile 0,01 mL arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,01/√6 = 0,00408 mL.
4. **dV_E**: Görsel dönüm noktası tespiti, %95'te bir damla. En iyi değer 0 mL, standart belirsizlik u = 0,025 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **V_NaCl**: Numunenin nominal hacmi. Tam bilinen bir değer, 100 mL; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
6. **dV_NaCl**: A sınıfı tam pipet, taban genişliği 0,16 mL üçgen. Değer -0,08 ile 0,08 mL arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,08/√6 = 0,0327 mL.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 10 mmol/L
- standart belirsizlik u = 0,026 mmol/L

Kaynak: IUPAC 2024, Örnek 4I: c(NaCl) = 10,000 mmol/L, u = 0,026 mmol/L (%0,26).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Atom ağırlıklarından molar kütle (Başlangıç)

Glukozun bağıl molekül kütlesi, aralık olarak verilen standart atom ağırlıklarından bulunur. Başka bilgi olmayan bir aralık dikdörtgen dağılıma dönüşür.

<a id="gum5-e3-molar-mass-glucose"></a>

### Standart atom ağırlığı aralıklarından glikozun bağıl molekül kütlesi (JCGM GUM-5 Örnek 3)

`gum5-e3-molar-mass-glucose`

#### Bu proje ne hakkında?

IUPAC 2021 standart atom ağırlığı aralıkları üzerinde dikdörtgen dağılımlarla M_r(C₆H₁₂O₆) = 6A_C + 12A_H + 6A_O.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **M_r**: Glikozun bağıl molekül kütlesi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
M_r = 6*A_C + 12*A_H + 6*A_O
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **A_H**: Hidrojenin standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 1,00784 ile 1,00811 arasında (1,007975 ± 0,00013500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00013500/√3 = 7,79 × 10⁻⁵.
2. **A_C**: Karbonun standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 12,0096 ile 12,0116 arasında (12,0106 ± 0,0010000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0010000/√3 = 0,000577.
3. **A_O**: Oksijenin standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 15,99903 ile 15,99977 arasında (15,9994 ± 0,00037000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00037000/√3 = 0,000214.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 180,1557
- standart belirsizlik u = 0,0038

Kaynak: JCGM GUM-5:2026, 3.3.3 ve 3.3.4: M_r = 180,1557, u = 0,0038.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Radyoaktif bozunma (Başlangıç)

Bir kaynağın aktivitesi referans tarihinden bu yana geçen süre için düzeltilir. Yarı ömür ve süre üstel bir fonksiyon üzerinden girer.

<a id="gum5-e14-radioactive-decay-bi207"></a>

### Bozunma için düzeltilmiş ²⁰⁷Bi kaynağının aktivitesi (JCGM GUM-5 Örnek 14)

`gum5-e14-radioactive-decay-bi207`

#### Bu proje ne hakkında?

Sertifikalı aktivite, literatürden yarı ömür ve Δt = 2,73 a ile A₁ = A₀ exp(−Δt ln2/T½); doğrusallaştırmanın denetimi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **A_1** (kBq): Kullanım anındaki aktivite. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
A_1 = A_0*exp(-dt*ln(2)/T_half)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **A_0**: Referans tarihindeki aktivite (sertifika). En iyi değer 11,45 kBq, standart belirsizlik u = 0,05 kBq (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **T_half**: ²⁰⁷Bi'nin yarı ömrü. En iyi değer 32,9 a, standart belirsizlik u = 1,4 a (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **dt**: Bozunma süresi t₁ − t₀. Tam bilinen bir değer, 2,73 a; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 10,81 kBq
- standart belirsizlik u = 0,054 kBq

Kaynak: JCGM GUM-5:2026, 14.3.2: A₁ = 10,810 kBq, u(A₁) = 0,054 kBq (katkılar 0,0472 ve 0,0265 kBq).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Günlük sıcaklıkların ortalaması (Başlangıç)

Günlük en yüksek sıcaklıkların aylık ortalaması: saf bir A tipi değerlendirme. Gün sayısı azken t dağılımı kapsam faktörünü belirgin biçimde 2'den büyük yapar.

<a id="nist-tn1900-e2-surface-temperature"></a>

### Günlük en yüksek sıcaklığın aylık ortalaması (NIST TN 1900 E2)

`nist-tn1900-e2-surface-temperature`

#### Bu proje ne hakkında?

TN 1900 E2: 22 günlük okuma için t_i = τ + E_i gözlem denklemi; 21 serbestlik dereceli A tipi değerlendirme.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **tau** (°C): Mayıs 2012'deki ortalama günlük en yüksek sıcaklık. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
tau = t
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 1 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **t**: Termometrenin günlük en yüksek sıcaklık okumaları. 22 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 25,5909 °C; okumalar s = 4,09 °C standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 4,09/√22 = 0,872 °C, serbestlik derecesi ν = n − 1 = 21.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 25,59 °C
- standart belirsizlik u = 0,872 °C
- kapsam faktörü k = 2,08

Kaynak: NIST TN 1900 E2: t̄ = 25,6 °C, u = 0,872 °C, k = 2,08, %95 aralığı (23,8; 27,4) °C; 25,59 °C metinden.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## CMC ile raporlama (Orta)

ILAC P14'e göre bir kalibrasyon sertifikası laboratuvarın kalibrasyon ve ölçüm yeteneğinden (CMC) küçük bir belirsizlik belirtemez. Program raporlarken bu alt sınırı uygular.

<a id="ilac-p14-cmc-floor-weight-10kg"></a>

### ILAC P14 5.5 raporlama kuralı: raporlanabilecek en küçük belirsizlik olarak CMC (EA-4/02 S2 kütle, örnek CMC)

`ilac-p14-cmc-floor-weight-10kg`

#### Bu proje ne hakkında?

EA-4/02 S2'deki 10 kg kütle (U = 58 mg); akredite CMC'si örnek olarak 80 mg alınan bir laboratuvar tarafından raporlanır: raporlanan U, değerlendirilen 58 mg değil CMC'dir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **m_X** (mg): Ağırlığın 10 kg'dan sapması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
m_X = m_S + dm_D + dm + dm_C + dB
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m_S**: Standart (sertifika: +5 mg, U = 45 mg, k = 2). En iyi değer 5 mg. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 45 mg, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 45/2 = 22,5 mg olur (normal dağılım).
2. **dm_D**: Standardın son kalibrasyondan beri sürüklenmesi. Yalnızca sınırlar biliniyor: değer -15 ile 15 mg arasında (0 ± 15) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 15/√3 = 8,66 mg.
3. **dm**: Gözlenen fark (3 ölçüm ortalaması, s_p = 25 mg). En iyi değer 20 mg, standart belirsizlik u = 14,434 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **dm_C**: Komparatör eksantrikliği ve manyetik etkiler. Yalnızca sınırlar biliniyor: değer -10 ile 10 mg arasında (0 ± 10) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 10/√3 = 5,77 mg.
5. **dB**: Hava kaldırma kuvveti. Yalnızca sınırlar biliniyor: değer -10 ile 10 mg arasında (0 ± 10) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 10/√3 = 5,77 mg.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 25 mg
- standart belirsizlik u = 29,2 mg
- genişletilmiş belirsizlik U = 58 mg

Kaynak: EA-4/02 M:2022 S2 (U = 58 mg) ve ILAC P14:09/2020 5.5; CMC örnek amaçlıdır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir dikdörtgen terim baskın olduğunda (Orta)

Çözünürlüğü baskın olan bir voltmetre. Sonuç normalden çok dikdörtgene yakın olduğu için UKAS M3003 bu durum için özel bir kapsam faktörü verir.

<a id="m3003-c5-dvm-dominant-rectangular"></a>

### Baskın çözünürlük terimli dijital voltmetre (UKAS M3003 Ek C)

`m3003-c5-dvm-dominant-rectangular`

#### Bu proje ne hakkında?

Uygulanan 1,000 00 V için 1,001 V okuyan bir DVM; 1 mV basamağının yuvarlanması baskındır; bu nedenle %95,45 için k, dikdörtgen ve normal dağılımın konvolüsyonundan gelir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dV** (mV): Voltmetrenin 1 V'taki hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dV = V_ind - V_ref + d_IT
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V_ind**: Gösterge (1 mV basamak). Yalnızca sınırlar biliniyor: değer 1000,5 ile 1001,5 mV arasında (1001 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 mV.
2. **V_ref**: Uygulanan referans gerilim (U = 0,19 mV, k = 2). En iyi değer 1000 mV. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,19 mV, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,19/2 = 0,0950 mV olur (normal dağılım).
3. **d_IT**: Sürüklenme ve sıcaklık etkileri. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 mV arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 mV.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1 mV
- standart belirsizlik u = 0,305 mV
- kapsam faktörü k = 1,77
- genişletilmiş belirsizlik U = 0,54 mV

Kaynak: UKAS M3003 6. baskı C.8–C.10: u_c = 0,305 mV, k = 1,77, U = 0,54 mV.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir aralık boyunca multimetre spesifikasyonu (Orta)

1 V aralığında 6½ basamaklı bir multimetre: katkıların bir kısmı okumaya göre bağıldır (ppm), bir kısmı mutlaktır (µV). Örnekler aralığın birkaç noktasındaki belirsizliği ve spesifikasyon baskın olduğundaki kapsam faktörünü gösterir.

<a id="m3003-l3-dmm-1v-range-095v"></a>

### 1 V kademesinde DMM: 0,95 V'taki bütçe (UKAS M3003 L.3)

`m3003-l3-dmm-1v-range-095v`

#### Bu proje ne hakkında?

Çok fonksiyonlu kalibratörle kalibre edilen 6½ basamaklı bir DMM'nin 1 V kademesinde 0,95 V'taki hatası; göreli (ppm) ve mutlak (µV) katkılar bir okumada birleştirilir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dI** (µV): DMM göstergesinin hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dI = I_DMM*1e-6*(r_cal + r_spec + r_rep) + a_cal + a_spec + a_T + a_CM + a_res
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_DMM**: Gösterge. Tam bilinen bir değer, 950000 µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **r_cal**: Kalibratörün kalibrasyonu, göreli (U = 2,8 ppm, k = 2). En iyi değer 0 ppm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 2,8 ppm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 2,8/2 = 1,40 ppm olur (normal dağılım).
3. **r_spec**: Kalibratör şartnamesi, göreli (±8 ppm). Yalnızca sınırlar biliniyor: değer -8 ile 8 ppm arasında (0 ± 8) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 8/√3 = 4,62 ppm.
4. **r_rep**: Tekrarlanabilirlik (2,5 ppm, ν = 9, tek okuma). 1 okumanın ortalaması, 0 ppm. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 2,5 ppm (ν = 9) kullanılır; böylece u = s_p/√n = 2,50 ppm.
5. **a_cal**: Kalibratörün kalibrasyonu, mutlak (U = 0,5 µV, k = 2). En iyi değer 0 µV. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,5 µV, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,5/2 = 0,250 µV olur (normal dağılım).
6. **a_spec**: Kalibratör şartnamesi, mutlak (±2 µV). Yalnızca sınırlar biliniyor: değer -2 ile 2 µV arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 µV.
7. **a_T**: Termoelektrik gerilimler. Yalnızca sınırlar biliniyor: değer -1 ile 1 µV arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 µV.
8. **a_CM**: Ortak mod etkileri. Yalnızca sınırlar biliniyor: değer -1 ile 1 µV arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 µV.
9. **a_res**: DMM çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 µV arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 µV.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 µV
- standart belirsizlik u = 5,37 µV
- genişletilmiş belirsizlik U = 10,73 µV

Kaynak: UKAS M3003 6. baskı L.3.19 (0,95 V'taki bütçe): u_c = 5,37 µV, U = 10,73 µV.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="m3003-l3-dmm-1v-range-1v-dominant"></a>

### Tam ölçekte 1 V kademesinde DMM: baskın şartname terimi için kapsam faktörü (UKAS M3003 L.3.20)

`m3003-l3-dmm-1v-range-1v-dominant`

#### Bu proje ne hakkında?

Çok fonksiyonlu kalibratörle kalibre edilen 6½ basamaklı bir DMM'nin 1 V kademesinde 1,0 V'taki hatası; göreli (ppm) ve mutlak (µV) katkılar bir okumada birleştirilir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dI** (µV): DMM göstergesinin hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dI = I_DMM*1e-6*(r_cal + r_spec + r_rep) + a_cal + a_spec + a_T + a_CM + a_res
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_DMM**: Gösterge. Tam bilinen bir değer, 1000000 µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **r_cal**: Kalibratörün kalibrasyonu, göreli (U = 2,8 ppm, k = 2). En iyi değer 0 ppm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 2,8 ppm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 2,8/2 = 1,40 ppm olur (normal dağılım).
3. **r_spec**: Kalibratör şartnamesi, göreli (±8 ppm). Yalnızca sınırlar biliniyor: değer -8 ile 8 ppm arasında (0 ± 8) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 8/√3 = 4,62 ppm.
4. **r_rep**: Tekrarlanabilirlik (2,5 ppm, ν = 9, tek okuma). 1 okumanın ortalaması, 0 ppm. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 2,5 ppm (ν = 9) kullanılır; böylece u = s_p/√n = 2,50 ppm.
5. **a_cal**: Kalibratörün kalibrasyonu, mutlak (U = 0,5 µV, k = 2). En iyi değer 0 µV. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,5 µV, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,5/2 = 0,250 µV olur (normal dağılım).
6. **a_spec**: Kalibratör şartnamesi, mutlak (±2 µV). Yalnızca sınırlar biliniyor: değer -2 ile 2 µV arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 µV.
7. **a_T**: Termoelektrik gerilimler. Yalnızca sınırlar biliniyor: değer -1 ile 1 µV arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 µV.
8. **a_CM**: Ortak mod etkileri. Yalnızca sınırlar biliniyor: değer -1 ile 1 µV arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 µV.
9. **a_res**: DMM çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 µV arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 µV.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 µV
- kapsam faktörü k = 1,898

Kaynak: UKAS M3003 6. baskı L.3.20: u_R/u_N = 1,435 için k = 1,898.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="m3003-l3-dmm-1v-range-500mv"></a>

### 1 V kademesinde DMM: kademe formülünden 500 mV'taki belirsizlik (UKAS M3003 L.3)

`m3003-l3-dmm-1v-range-500mv`

#### Bu proje ne hakkında?

Çok fonksiyonlu kalibratörle kalibre edilen 6½ basamaklı bir DMM'nin 1 V kademesinde 0,5 V'taki hatası; göreli (ppm) ve mutlak (µV) katkılar bir okumada birleştirilir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dI** (µV): DMM göstergesinin hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dI = I_DMM*1e-6*(r_cal + r_spec + r_rep) + a_cal + a_spec + a_T + a_CM + a_res
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_DMM**: Gösterge. Tam bilinen bir değer, 500000 µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **r_cal**: Kalibratörün kalibrasyonu, göreli (U = 2,8 ppm, k = 2). En iyi değer 0 ppm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 2,8 ppm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 2,8/2 = 1,40 ppm olur (normal dağılım).
3. **r_spec**: Kalibratör şartnamesi, göreli (±8 ppm). Yalnızca sınırlar biliniyor: değer -8 ile 8 ppm arasında (0 ± 8) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 8/√3 = 4,62 ppm.
4. **r_rep**: Tekrarlanabilirlik (2,5 ppm, ν = 9, tek okuma). 1 okumanın ortalaması, 0 ppm. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 2,5 ppm (ν = 9) kullanılır; böylece u = s_p/√n = 2,50 ppm.
5. **a_cal**: Kalibratörün kalibrasyonu, mutlak (U = 0,5 µV, k = 2). En iyi değer 0 µV. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,5 µV, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,5/2 = 0,250 µV olur (normal dağılım).
6. **a_spec**: Kalibratör şartnamesi, mutlak (±2 µV). Yalnızca sınırlar biliniyor: değer -2 ile 2 µV arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 µV.
7. **a_T**: Termoelektrik gerilimler. Yalnızca sınırlar biliniyor: değer -1 ile 1 µV arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 µV.
8. **a_CM**: Ortak mod etkileri. Yalnızca sınırlar biliniyor: değer -1 ile 1 µV arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 µV.
9. **a_res**: DMM çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 µV arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 µV.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 µV
- genişletilmiş belirsizlik U = 6,2 µV

Kaynak: UKAS M3003 6. baskı L.3.14: 500 mV'ta U = 6,2 µV.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir sodyum hidroksit çözeltisinin ayarlanması (Orta)

Tartılmış bir birincil standarda karşı titrasyon. Örnek alt modeller kullanır (kütle, büret hacmi) ve tekrarlanabilirliğin tek bir birleşik terime konabileceğini gösterir.

<a id="quam-a2-naoh-standardisation-submodels"></a>

### Alt modellerle NaOH ayarlanması

`quam-a2-naoh-standardisation-submodels`

#### Bu proje ne hakkında?

Kütle, molar kütle ve hacmin bileşenlerinden oluşturulduğu QUAM Örnek A2.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_NaOH** (mol/L): NaOH çözeltisinin derişimi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_NaOH = 1000 * m_KHP * P_KHP / (M_KHP * V_T) * rep
```

Denklemi okunur tutmak için önce 3 ara büyüklük hesaplanır:

- `m_KHP = m_gross - m_tare`
- `M_KHP = 8*A_C + 5*A_H + 4*A_O + A_K`
- `V_T = 18.64 + dV_cal + dV_temp`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 10 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **rep**: Tekrarlanabilirlik çarpanı. En iyi değer 1, standart belirsizlik u = 0,0005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **m_gross**: Kap ve KHP. Yalnızca sınırlar biliniyor: değer 60,545 ile 60,54515 g arasında (60,545 ± 0,00015) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00015/√3 = 8,66 × 10⁻⁵ g.
3. **m_tare**: Boş kap. Yalnızca sınırlar biliniyor: değer 60,15605 ile 60,156 g arasında (60,1562 ± 0,00015) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00015/√3 = 8,66 × 10⁻⁵ g.
4. **P_KHP**: KHP saflığı (1,0000 ± 0,0005). Yalnızca sınırlar biliniyor: değer 0,9995 ile 1,0005 arasında (1 ± 0,0005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0005/√3 = 0,000289.
5. **A_C**: C atom ağırlığı. Yalnızca sınırlar biliniyor: değer 12,0099 ile 12,0115 g/mol arasında (12,0107 ± 0,0008) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0008/√3 = 0,000462 g/mol.
6. **A_H**: H atom ağırlığı. Yalnızca sınırlar biliniyor: değer 1,00787 ile 1,00801 g/mol arasında (1,00794 ± 0,00007) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00007/√3 = 4,04 × 10⁻⁵ g/mol.
7. **A_O**: O atom ağırlığı. Yalnızca sınırlar biliniyor: değer 15,9991 ile 16,000 g/mol arasında (15,9994 ± 0,0003) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0003/√3 = 0,000173 g/mol.
8. **A_K**: K atom ağırlığı. Yalnızca sınırlar biliniyor: değer 39,0982 ile 39,098 g/mol arasında (39,0983 ± 0,0001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0001/√3 = 5,77 × 10⁻⁵ g/mol.
9. **dV_cal**: Büret kalibrasyonu. Değer -0,03 ile 0,03 mL arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,03/√6 = 0,0122 mL.
10. **dV_temp**: Sıcaklık etkisi (%95'te ±3 °C). En iyi değer 0 mL, standart belirsizlik u = 0,0061071 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,102136 mol/L
- genişletilmiş belirsizlik U = 0,0002 mol/L

Kaynak: QUAM:2012 A2.4–A2.6, sonuç (0,1021 ± 0,0002) mol/L.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="quam-a2-naoh-standardisation"></a>

### Sodyum hidroksit çözeltisinin ayarlanması

`quam-a2-naoh-standardisation`

#### Bu proje ne hakkında?

QUAM Örnek A2: NaOH çözeltisinin potasyum hidrojen ftalata (KHP) karşı ayarlanması; rehberin standart belirsizlikleriyle özet bütçe.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_NaOH** (mol/L): NaOH çözeltisinin derişimi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_NaOH = 1000 * m_KHP * P_KHP / (M_KHP * V_T) * rep
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **rep**: Tekrarlanabilirlik çarpanı. En iyi değer 1, standart belirsizlik u = 0,0005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **m_KHP**: KHP kütlesi. En iyi değer 0,3888 g, standart belirsizlik u = 0,00013 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **P_KHP**: KHP saflığı. En iyi değer 1, standart belirsizlik u = 0,00029 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_KHP**: KHP'nin molar kütlesi. En iyi değer 204,2212 g/mol, standart belirsizlik u = 0,0038 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **V_T**: NaOH titrasyon hacmi. En iyi değer 18,64 mL, standart belirsizlik u = 0,013 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,102136 mol/L
- standart belirsizlik u = 0,000099 mol/L

Kaynak: QUAM:2012 Tablo A2.3 (c = 0,102136 mol/L, u_c = 0,000099 mol/L).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Hidroklorik asit titrasyonu (Orta)

Ayarlı NaOH ile titrasyondan HCl derişimi. Önceki örnekteki NaOH belirsizliği buraya taşınır: bir değerlendirmenin sonuçları bir sonrakinin girdileri olur.

<a id="quam-a3-hcl-titration"></a>

### Hidroklorik asidin asit–baz titrasyonu

`quam-a3-hcl-titration`

#### Bu proje ne hakkında?

QUAM Örnek A3: KHP'ye karşı ayarlanmış NaOH ile HCl titrasyonu.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_HCl** (mol/L): HCl derişimi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_HCl = 1000 * m_KHP * P_KHP * V_T2 / (V_T1 * M_KHP * V_HCl) * rep
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **rep**: Tekrarlanabilirlik çarpanı. En iyi değer 1, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **m_KHP**: KHP kütlesi. En iyi değer 0,3888 g, standart belirsizlik u = 0,00012 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **P_KHP**: KHP saflığı. En iyi değer 1, standart belirsizlik u = 0,00029 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **V_T2**: HCl titrasyonu için NaOH hacmi. En iyi değer 14,89 mL, standart belirsizlik u = 0,014 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **V_T1**: Ayarlama için NaOH hacmi. En iyi değer 18,64 mL, standart belirsizlik u = 0,015 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **M_KHP**: KHP'nin molar kütlesi. En iyi değer 204,2212 g/mol, standart belirsizlik u = 0,0038 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **V_HCl**: HCl hacmi. En iyi değer 15 mL, standart belirsizlik u = 0,011 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,101387 mol/L
- standart belirsizlik u = 0,00018276 mol/L
- genişletilmiş belirsizlik U = 0,0004 mol/L

Kaynak: QUAM:2012 Tablo A3.3 (u_c = √3,34 × 10⁻⁸ mol/L) ve sonuç (0,1014 ± 0,0004) mol/L.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Ekmekte pestisitler (Orta)

Aynı cevaba iki yol: yöntemin adımlarından kurulan bir bütçe ve validasyon verilerinden (kesinlik ve geri kazanım) yukarıdan aşağı bir tahmin. Her yaklaşımın hangi bileşenleri yakaladığını karşılaştırın.

<a id="quam-a4-pesticides-combination"></a>

### Ekmekte organofosforlu pestisitler: birleştirme

`quam-a4-pesticides-combination`

#### Bu proje ne hakkında?

QUAM Örnek A4: laboratuvar içi validasyon verilerinden (kesinlik, geri kazanım, homojenlik) elde edilen bağıl belirsizliğin P = 1'de çarpımsal modelde birleştirilmesi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **P_op**: Ölçülen değere göre pestisit düzeyi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
P_op = F_I * F_hom / Rec
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **F_I**: Ara kesinlik çarpanı. En iyi değer 1, standart belirsizlik u = 0,27 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **Rec**: Ortalama geri kazanım. En iyi değer 0,9, standart belirsizlik u = 0,043 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **F_hom**: Numune homojensizliği çarpanı. En iyi değer 1, standart belirsizlik u = 0,2 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,1111
- standart belirsizlik u = 0,377

Kaynak: QUAM:2012 Tablo A4.5 (P = 1,1111, u = 0,377).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="quam-a4-pesticides-topdown"></a>

### Validasyon verilerinden ekmekte organofosforlu pestisitler

`quam-a4-pesticides-topdown`

#### Bu proje ne hakkında?

EURACHEM QUAM Örnek A4, top-down değerlendirme olarak: 15 mükerrer ölçümden kesinlik (ham veri), 42 katkıdan geri kazanım, homojenlik.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **P_op** için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 1.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **precision**: 15 çift ölçüm; standart sapma çiftlerin içindeki farklardan bulunur.

**Geri kazanım**: 1 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.

**Diğer bileşenler**, verildiği gibi eklenir:

- homogeneity


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: QUAM:2012 Tablo A4.3–A4.5 (0,27, 0,048, 0,2 → 0,34; P = 1,1111'de u = 0,377).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Seramikten salınan kadmiyum (Orta)

Ampirik bir yöntem: sonuç prosedürle tanımlanır (süre, sıcaklık, asit). Bu koşulların etkileri belirsizlik terimleri olarak eklenir.

<a id="quam-a5-cadmium-release"></a>

### Seramik kaplardan salınan kadmiyum

`quam-a5-cadmium-release`

#### Bu proje ne hakkında?

QUAM Örnek A5: ampirik yöntem (BS 6748); birim alan başına çözünen Cd miktarı.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **r** (mg/dm²): Birim alan başına salınan Cd. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
r = c0 * V_L / a_V * f_acid * f_time * f_temp
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **c0**: Özütleme çözeltisindeki Cd derişimi. En iyi değer 0,26 mg/L, standart belirsizlik u = 0,018 mg/L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **V_L**: Özütleme çözeltisinin hacmi. En iyi değer 0,332 L, standart belirsizlik u = 0,0018 L (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **a_V**: Kabın yüzey alanı. En iyi değer 5,73 dm², standart belirsizlik u = 0,19 dm² (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **f_acid**: Asit derişimi çarpanı. En iyi değer 1, standart belirsizlik u = 0,0008 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **f_time**: Özütleme süresi çarpanı. En iyi değer 1, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **f_temp**: Sıcaklık çarpanı. En iyi değer 1, standart belirsizlik u = 0,06 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,015065 mg/dm²
- standart belirsizlik u = 0,001465 mg/dm²
- genişletilmiş belirsizlik U = 0,003 mg/dm²

Kaynak: QUAM:2012 Tablo A5.4 (r = 0,015065 mg/dm², tablolama değeri u = 0,001465) ve sonuç (0,015 ± 0,003) mg/dm².

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Hayvan yeminde ham lif (Orta)

Bir başka ampirik yöntem; bir laboratuvarlar arası çalışmanın kesinliğiyle değerlendirilir: yöntemin tekrar üretilebilirliği belirsizliğin ana kısmıdır.

<a id="quam-a6-crude-fibre"></a>

### Hayvan yeminde ham lif (ampirik yöntem)

`quam-a6-crude-fibre`

#### Bu proje ne hakkında?

EURACHEM QUAM Örnek A6: üç düzeyde ortak çalışmadan tekrar üretilebilirlik s_R ve kurutma payı.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(crude fibre)** (% m/m) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 3 derişim düzeyinde değerlendirilir: 2,5; 5; 10 % m/m.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 3 kayıt:

- **s_R (2.5)**: standart sapma 0,29 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_R (5)**: standart sapma 0,4 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_R (10)**: standart sapma 0,6 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Diğer bileşenler**, verildiği gibi eklenir:

- drying


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: QUAM:2012 Tablo A6.4–A6.5.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Çevre analizleri için yukarıdan aşağı (Orta)

Atık sudaki BOİ ve sedimentteki PCB, her biri iki kez değerlendirilir: sapma için bir kez CRM ile, bir kez yeterlilik testleriyle. Sapma verisinin seçiminin sonucu nasıl değiştirdiğini görün.

<a id="nordtest-8-bod-crm"></a>

### Atık suda BOİ: kontrol grafiği ve tek CRM

`nordtest-8-bod-crm`

#### Bu proje ne hakkında?

Nordtest TR 537 §8.2 Örnek A, Ek 7'deki ham kontrol verileriyle.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **BOD** (mg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **u(Rw)**: series

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §8.2 Örnek A (u(Rw) %2,6, u(bias) %4,5, u_c %5,2, U %10,4 → %11).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nordtest-8-bod-pt"></a>

### Atık suda BOİ: kontrol grafiği ve yeterlilik testleri

`nordtest-8-bod-pt`

#### Bu proje ne hakkında?

Nordtest TR 537 §8.2 Örnek B: üç yeterlilik testinden u(bias) (Tablo 6, ham veri).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **BOD** (mg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **u(Rw)**: series

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §8.2 Örnek B (u(bias) %4,12, u_c %4,87, U %9,7 → %10); toleranslar Tablo 6'daki yuvarlanmış sapmaları kapsar.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nordtest-8-pcb-crm"></a>

### Sedimentte PCB: kontrol numunesi ve tek CRM

`nordtest-8-pcb-crm`

#### Bu proje ne hakkında?

Nordtest TR 537 §8.3 Örnek C.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(ΣPCB₇)** (µg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **u(Rw)**: standart sapma 8 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §8.3 Örnek C (u(bias) %7,22, u_c %10,8, U %21,6 → %22); yuvarlanmış %5,3 ve %4,6 yerine 144 ve 152'den hesaplanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nordtest-8-pcb-pt"></a>

### Sedimentte PCB: kontrol numunesi ve yeterlilik testleri

`nordtest-8-pcb-pt`

#### Bu proje ne hakkında?

Nordtest TR 537 §8.3 Örnek D.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(ΣPCB₇)** (µg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **u(Rw)**: standart sapma 8 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Nordtest TR 537 §8.3 Örnek D (RMS_bias %7,6, u(bias) %8,1, u_c %11,4, U %22,8 → %23).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Laboratuvar içi kesinlik ve geri kazanımdan belirsizlik (Orta)

2026 Eurachem/CITAC kılavuzu (MUPR) belirsizliği her laboratuvarın sahip olduğu verilerden değerlendirir: birkaç düzeyde kesinlik ve referans malzemelerden ya da katkılanmış numunelerden geri kazanım. Örnekler birleştirilmiş ve derişime bağlı kesinliği ve farklı geri kazanım tasarımlarını kapsar.

<a id="mupr-b1-nitrate"></a>

### İçme suyunda nitrat: iki kesinlik aralığı

`mupr-b1-nitrate`

#### Bu proje ne hakkında?

MUPR Örnek B1: 0,4 mg/L altında mutlak, üstünde bağıl kesinlik; 20 katkılı sudan geri kazanım; stok çözelti ve seyreltme bileşenleri.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c(NO₃⁻)** (mg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 0,261; 50 mg/L.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 2 kayıt:

- **s_I⟨I⟩**: standart sapma 0,0328 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 8,43 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Geri kazanım**: 1 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.

**Diğer bileşenler**, verildiği gibi eklenir:

- u′(DSS)
- u′(F)


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek B1, Eş. (B1.3) ve B1.7.iv (0,261 mg/L'de U = 0,0676 mg/L; 50 mg/L'de 9,26 mg/L, rehber u′(R̄)'yi 0,0285'e yuvarlayarak hesaplar; yuvarlanmamış 9,27 mg/L).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="mupr-b2-pooled-precision"></a>

### Nitrat: birleştirilmiş kesinlik ve iki geri kazanım düzeyi

`mupr-b2-pooled-precision`

#### Bu proje ne hakkında?

MUPR Örnek B2: birden fazla malzemeden birleştirilmiş kesinlik (ikisi için ham veri), geçiş derişimi ve genel geri kazanım.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c(NO₃⁻)** (mg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 50 mg/L.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 2 kayıt:

- **s_I(P)⟨I⟩**: 3 grup tekrarlı sonuç (örneğin günler); varyans analizi tekrarlanabilirliği günler arası değişimden ayırır.
- **s′_I(P)⟨II⟩**: 2 grup tekrarlı sonuç (örneğin günler); varyans analizi tekrarlanabilirliği günler arası değişimden ayırır.

**Geri kazanım**: 2 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.

**Diğer bileşenler**, verildiği gibi eklenir:

- u′(DSS)
- u′(F)


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek B2, Eş. (B2.1)–(B2.7).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="mupr-b2-replicates-on-two-days"></a>

### Nitrat: 2 günde 3 tekrarın ortalaması olarak sonuç

`mupr-b2-replicates-on-two-days`

#### Bu proje ne hakkında?

MUPR Örnek B2, Eş. (B2.9): p günün her birinde n sonucun ortalamasının kesinliği (Eş. 5).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c(NO₃⁻)** (mg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 57,5 mg/L.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s′_I(P)⟨II⟩**: 2 grup tekrarlı sonuç (örneğin günler); varyans analizi tekrarlanabilirliği günler arası değişimden ayırır.

**Geri kazanım**: 2 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.

**Diğer bileşenler**, verildiği gibi eklenir:

- u′(DSS)
- u′(F)


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek B2, Eş. (B2.9) (U = 7,06 mg/L).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="mupr-b3-linear-precision"></a>

### Nitrat: derişimin doğrusal fonksiyonu olarak kesinlik

`mupr-b3-linear-precision`

#### Bu proje ne hakkında?

MUPR Örnek B3: dört malzemenin ara kesinliği en küçük kareler ile s_I = a + b·c olarak modellenir ve seyreltilmiş bir numuneye uygulanır.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c(NO₃⁻)** (mg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 57,5 mg/L.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s_I(c)**: linear

**Geri kazanım**: 2 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.

**Diğer bileşenler**, verildiği gibi eklenir:

- u′(DSS)
- u′(F)


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek B3, Eş. (B3.1) (basılı kesim noktası 0,0233 ile U = 7,28 mg/L; en küçük kareler değeri 0,02324 ile 7,27 mg/L).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="mupr-b4-arsenic-one-crm"></a>

### Sedimentte arsenik: tek CRM

`mupr-b4-arsenic-one-crm`

#### Bu proje ne hakkında?

MUPR Örnek B4: birleştirilmiş bağıl kesinlik ve tek CRM'nin geri kazanımı.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(As)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 16 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 2 kayıt:

- **s_I⟨I⟩**: 3 grup tekrarlı sonuç (örneğin günler); varyans analizi tekrarlanabilirliği günler arası değişimden ayırır.
- **s′_I⟨II⟩**: 3 grup tekrarlı sonuç (örneğin günler); varyans analizi tekrarlanabilirliği günler arası değişimden ayırır.

**Geri kazanım**: 1 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek B4, Eş. (B4.1)–(B4.5).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="mupr-b5-crm-and-spiked-samples"></a>

### Arsenik: matrise bağlı geri kazanımlı CRM ve katkılı numuneler

`mupr-b5-crm-and-spiked-samples`

#### Bu proje ne hakkında?

MUPR Örnek B5: geri kazanımlar matrisle değiştiğinden standart sapmaları dahil edilir ve k = t(2).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(As)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 16 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s′_I⟨II⟩**: 3 grup tekrarlı sonuç (örneğin günler); varyans analizi tekrarlanabilirliği günler arası değişimden ayırır.

**Geri kazanım**: 3 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek B5, Eş. (B5.1)–(B5.6) (t 4,30'a yuvarlanarak U = 0,395 w; t₀,₉₇₅(2) = 4,303 ile 0,3955 w).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="mupr-b6-sediment-22-pt"></a>

### Sedimentte iz element: heterojenlik ve 22 yeterlilik testi

`mupr-b6-sediment-22-pt`

#### Bu proje ne hakkında?

MUPR Örnek B6: homojen bir malzemeden kesinlik ve gerçek numunelerin heterojenliği (mükerrer açıklıklar), 22 YT sonucundan geri kazanım (ham veri).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 5; 20 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **s_I(DS)⟨I⟩**: series
- **s_r(h)⟨I⟩**: 30 çift ölçüm; standart sapma çiftlerin içindeki farklardan bulunur.
- **s′_I(DS)⟨II⟩**: series
- **s′_r(h)⟨II⟩**: 30 çift ölçüm; standart sapma çiftlerin içindeki farklardan bulunur.

**Geri kazanım**: 22 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek B6, Tablo B6.1–B6.3 ve Eş. (B6.1)–(B6.9); χ², Eş. (16)'daki ağırlıklı ortalama ile yeniden hesaplanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="mupr-ex2-reference-materials"></a>

### İki CRM ve bir yeterlilik testi numunesinden geri kazanım

`mupr-ex2-reference-materials`

#### Bu proje ne hakkında?

MUPR Örnek 2: orto-fosfat; üç referans malzemeden ortalama geri kazanım ve belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c(PO₄-P)** (mg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Geri kazanım**: 3 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek 2 (R̄ = 1,008, u(R̄) = 0,03705).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="mupr-ex3-spiked-samples"></a>

### Katkılı nehir sularından geri kazanım

`mupr-ex3-spiked-samples`

#### Bu proje ne hakkında?

MUPR Örnek 3: doğal analit içeren iki katkılı numune (Eş. 10–11).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **c** (µg/L) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Geri kazanım**: 2 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: MUPR Örnek 3 (R̄ = %96,2, u(R̄) = %10,02).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Uygulamada kontrol grafikleri (Orta)

Nordtest el kitabının çözümlü örnekleri: X grafikleri, çift ölçümler için R ve r% grafikleri, gerekli belirsizlikten hedef sınırlar, körler, CRM'ler ve sınırların yıllık gözden geçirilmesi.

<a id="tr569-ex1-nickel-steel-xrf"></a>

### XRF ile düşük alaşımlı çelikte nikel: gerekli belirsizlikten hedef sınırlar (Nordtest TR 569 Örnek 1)

`tr569-ex1-nickel-steel-xrf`

#### Bu proje ne hakkında?

Bir çelik numunenin X-grafiği; bir yıllık ortalama ağırlıkça %4,58, s = ağırlıkça %0,026; gerekli genişletilmiş belirsizlik U = %4 (göreli).


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: hedef sınırlar, yöntemin ulaşması gereken kesinlikten.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 1.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex10-boron-toys-target"></a>

### ICP-OES ile oyuncaklardan bor göçü: EN 71-3 tekrar üretilebilirliğinden hedef sınırlar (Nordtest TR 569 Örnek 10)

`tr569-ex10-boron-toys-target`

#### Bu proje ne hakkında?

Tüm işlem için malzeme yoktur: 5,0 µg/L laboratuvar içi sentetik standart kaymayı izler; s_hedef = %10, EN 71-3'ün %15 tekrar üretilebilirliğinin üçte ikisi.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: hedef sınırlar, yöntemin ulaşması gereken kesinlikten.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 10.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex12-pooling-sr-srw"></a>

### Tekrarlı kontrol sonuçlarından tekrarlanabilirlik ve laboratuvar içi tekrar üretilebilirlik (Nordtest TR 569 Örnek 12)

`tr569-ex12-pooling-sr-srw`

#### Bu proje ne hakkında?

Sekiz günün her birinde bir kalite kontrol numunesinin üç tekrarı. 24 sonucun birlikte standart sapması s_Rw için fazla düşük bir tahmindir ve çok dar kontrol sınırları verir.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

8 kontrol değeri girilmiş.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 12; dipnot 24 √(0,01541² + 0,02246²) = 0,0272 yazar, on kat küçük (araştırma günlüğü F53).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex2-cobalt-steel-xrf"></a>

### XRF ile düşük alaşımlı çelikte kobalt: tayin sınırından hedef sınırlar (Nordtest TR 569 Örnek 2)

`tr569-ex2-cobalt-steel-xrf`

#### Bu proje ne hakkında?

Düşük derişimde X-grafiği, ortalama ağırlıkça %0,0768, s = ağırlıkça %0,00063; gerekli 0,01 ağırlıkça % LOQ, s'nin 6–10 katıdır, bu nedenle s_hedef = ağırlıkça %0,001.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: hedef sınırlar, yöntemin ulaşması gereken kesinlikten.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 2.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex3-ammonium-r"></a>

### Suda amonyum azotu: paralel numunelerin R-grafiği (Nordtest TR 569 Örnek 3)

`tr569-ex3-ammonium-r`

#### Bu proje ne hakkında?

Her seride 100 µg/L'nin altındaki bir numune paralel analiz edilir; paralel tayinlerin açıklığı çizilir.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 3. Örnek s'ye birleştirilmiş tekrarlanabilirlik standart sapması der; değeri ortalama açıklığın 1,128'e bölümüdür.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex3-ammonium-x"></a>

### Suda amonyum azotu: standart çözeltinin X-grafiği (Nordtest TR 569 Örnek 3)

`tr569-ex3-ammonium-x`

#### Bu proje ne hakkında?

İndofenol mavisi yöntemi; NH₄Cl kalibrasyon standartlarından bağımsız olarak (NH₄)₂SO₄'ten hazırlanmış 20 µg/L kontrol çözeltisi; paralel tayinlerin ortalaması çizilir.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: istatistiksel, kontrol değerlerinin kendi dağılımından.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 3.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex4-lead-water-icpms"></a>

### ICP-MS ile suda kurşun: laboratuvar içi göl suyunun X-grafiği (Nordtest TR 569 Örnek 4)

`tr569-ex4-lead-water-icpms`

#### Bu proje ne hakkında?

Korunmuş göl suyunda düşük kurşun derişimi (0,29 µg/L), her seride bir kez analiz edilir; istatistiksel sınırlar.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: istatistiksel, kontrol değerlerinin kendi dağılımından.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 4.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex5-arsenic-crm-tolerance"></a>

### ICP-MS ile balık CRM'sinde arsenik: yalnızca ± %15 eylem sınırlı grafik (Nordtest TR 569 Örnek 5)

`tr569-ex5-arsenic-crm-tolerance`

#### Bu proje ne hakkında?

CRM NRC DORM-2 (köpek balığı kası), 18,0 µg/g arsenik, her seride bir kez analiz edilir; laboratuvar referans değerden %15'e kadar sapmayı kabul eder.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: tolerance.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 5.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex6-phosphorus-relative-range"></a>

### Suda toplam fosfor: paralel tayinlerin r%-grafiği (Nordtest TR 569 Örnek 6)

`tr569-ex6-phosphorus-relative-range`

#### Bu proje ne hakkında?

10 µg/L'nin üzerindeki numuneler paralel analiz edilir; LOQ'nun çok üzerinde göreli standart sapma yaklaşık sabit olduğundan farkın ortalamaya yüzde oranı çizilir.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: istatistiksel, kontrol değerlerinin kendi dağılımından.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 6. Örnek CV = 1,88/1,128 = %1,667 değerini katsayılarla çarpmadan önce %1,67'ye yuvarlar; yuvarlanmamış değerle sınırlar %4,72 ve %6,14 olurdu.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex7-beta-hch-crm-target"></a>

### GC-ECD ile morina karaciğeri yağında β-HCH: referans değer çevresinde hedef sınırlar (Nordtest TR 569 Örnek 7)

`tr569-ex7-beta-hch-crm-target`

#### Bu proje ne hakkında?

CRM BCR 598, 16 µg/kg β-heksaklorosikloheksan, her seride bir kez analiz edilir; %15 hedef s.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: hedef sınırlar, yöntemin ulaşması gereken kesinlikten.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 7.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex8-copper-r-review"></a>

### ICP-OES ile suda bakır: paralel tayinlerin R-grafiği ve gözden geçirilmesi (Nordtest TR 569 Örnek 8)

`tr569-ex8-copper-r-review`

#### Bu proje ne hakkında?

Numunelerin paralel tayinleri; ön ortalama açıklık 0,11 mg/L, s_r = 0,0975 mg/L; son 60 serinin s_r'si 0,0957 mg/L.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 8. Örnek F = 1,037 yazar; 0,0975²/0,0957² = 1,0380'dir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex8-copper-x-new-limits"></a>

### ICP-OES ile suda bakır: tüm verilerden yeni X-grafiği sınırları (Nordtest TR 569 Örnek 8)

`tr569-ex8-copper-x-new-limits`

#### Bu proje ne hakkında?

Gözden geçirmeden sonra sınırlar daha uzun dönemden yeniden hesaplanır: x̄ = 1,048 mg/L ve s = 0,0822 mg/L.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: istatistiksel, kontrol değerlerinin kendi dağılımından.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 8.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex8-copper-x-review"></a>

### ICP-OES ile suda bakır: ön X-grafiği sınırlarının gözden geçirilmesi (Nordtest TR 569 Örnek 8)

`tr569-ex8-copper-x-review`

#### Bu proje ne hakkında?

Paralel ölçülen sentetik standart (1,00 ± 0,02 mg/L); ön sınırlar ilk 60 seriden, x̄ = 1,055 mg/L, s = 0,0667 mg/L. Son 60 değer gözden geçirilir: 11'i uyarı sınırları dışında; merkez çizgisinden 4 s'den uzak bir değer dışarıda bırakılır, x̄ = 1,041 mg/L ve s_Rw = 0,0834 mg/L olan 59 değer kalır.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: istatistiksel, kontrol değerlerinin kendi dağılımından.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 8. 1,67 kritik F değeri Tablo 3'ten 60 ve 60 serbestlik derecesinde okunmuştur; program F₀.₉₇₅(58, 59) değerini tam olarak hesaplar.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex9-zinc-procedural-blank"></a>

### Hidrojen peroksitte çinko: yöntem körünün X-grafiği (Nordtest TR 569 Örnek 9)

`tr569-ex9-zinc-procedural-blank`

#### Bu proje ne hakkında?

Kirlenmeyi saptamak için tüm işlemden geçirilen ultra saf su (50 mL buharlaştırma, HCl, 5 mL'ye seyreltme, ICP-OES); negatif değerler de çizilir.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: istatistiksel, kontrol değerlerinin kendi dağılımından.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 9.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tr569-ex11-micropipette-tolerance"></a>

### 1 mL mikropipetin sapması: toleransta eylem sınırlı grafik (Nordtest TR 569 Örnek 11)

`tr569-ex11-micropipette-tolerance`

#### Bu proje ne hakkında?

Sabit hacimli 1 mL mikropipet on kez test edilir (Nordtest TR 626); ortalama hacim, laboratuvarın ± %1 toleransındaki eylem sınırlarıyla nominal değere karşı çizilir.


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: tolerance.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: TR 569 Örnek 11.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Karar sınırları (Orta)

Ne zaman hız cezası kesilmeli ya da bir gıda numunesi uygunsuz ilan edilmeli? Karar sınırı yasal sınırın ötesine konur; böylece haksız bir suçlama çok olasılık dışı olur (korumalı ret).

<a id="jcgm106-8.3-nandrolone"></a>

### Hayvansal ürünlerde nandrolon: tarama testi için karar sınırı (JCGM 106 8.3.3)

`jcgm106-8.3-nandrolone`

#### Bu proje ne hakkında?

JCGM 106:2012 8.3.3 Örnek 2: eşik 2,00 µg/L; eşiğin aşılmış olma olasılığı en az %95 ise sonuç şüphelidir; on katkılı kör numuneden tekrar üretilebilirlik s = 0,20 µg/L, ν = 9 ile t dağılımı.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük c en çok 2 µg/L olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı ret: bir ürün ancak sınırın açıkça dışındaysa reddedilir (örneğin haksız bir suçlamadan kaçınılması gereken yasal denetim ya da doping kontrolünde).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 106:2012 8.3.3, Örnek 2 (A = 2,37 µg/L).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm106-8.3-speed-limit"></a>

### Hız sınırı denetimi: korumalı ret (JCGM 106 8.3.3)

`jcgm106-8.3-speed-limit`

#### Bu proje ne hakkında?

JCGM 106:2012 8.3.3 Örnek 1: hız sınırı 100 km/h, bağıl standart belirsizliği %2 olan radar; yalnızca sınırın aşılmış olma olasılığı en az %99,9 ise ceza.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük v en çok 100 km/h olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı ret: bir ürün ancak sınırın açıkça dışındaysa reddedilir (örneğin haksız bir suçlamadan kaçınılması gereken yasal denetim ya da doping kontrolünde).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 106:2012 8.3.3, Örnek 1 (v_max ≈ 107 km/h).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir spesifikasyona uygunluk (Orta)

Eurachem/CITAC uygunluk kılavuzu: korumalı kabul ve ret, az serbestlik dereceli bir t dağılımı ve bir sınıra yakın büyük bir bağıl belirsizlik için lognormal dağılım.

<a id="eurachem-compliance-b1-nickel-steel"></a>

### Paslanmaz çelikte nikel: iki taraflı şartnamede korumalı kabul (Eurachem/CITAC uygunluk rehberi, Örnek B1)

`eurachem-compliance-b1-nickel-steel`

#### Bu proje ne hakkında?

Şartname %16,0–18,0 Ni, U = %0,2 (k = 2); uygunluğun en az %95 olası olduğu kabul bölgesi; ölçülen %16,1 Ni.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük w(Ni) 16 ile 18 arasında % olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı kabul: kabul bölgesi tolerans aralığından bir koruma bandı kadar daraltılır; böylece kötü bir ürünü kabul etmek olasılık dışı olur (müşteriyi korur).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Eurachem/CITAC 2021, Örnek B1: yuvarlamadan sonra kabul bölgesi %16,2–17,8 Ni; %16,1 Ni uygun değil.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="eurachem-compliance-b2-upper-limit-t"></a>

### Üst sınırın altındaki analit, ν = 8 t dağılımı: korumalı ret (Eurachem/CITAC uygunluk rehberi, Örnek B2)

`eurachem-compliance-b2-upper-limit-t`

#### Bu proje ne hakkında?

Üst sınır 200 ng/g, u = 8 serbestlik dereceli 2,2 ng/g; yalnızca sınırın üstünde olma olasılığı %95'i aşarsa uygun değil; ölçülen 203,7 ng/g.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük w en çok 200 ng/g olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı ret: bir ürün ancak sınırın açıkça dışındaysa reddedilir (örneğin haksız bir suçlamadan kaçınılması gereken yasal denetim ya da doping kontrolünde).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Eurachem/CITAC 2021, Örnek B2: k = 1,86, g = 4,1 ng/g, kabul sınırı 204,1 ng/g; 203,7 ng/g uygun.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="eurachem-compliance-b3-banned-substance-lognormal-exact"></a>

### 2 ng/g sınırında yasaklı madde, u_bağıl = %35: lognormal dağılım, kesin s_G (Eurachem/CITAC uygunluk rehberi, Örnek B3)

`eurachem-compliance-b3-banned-substance-lognormal-exact`

#### Bu proje ne hakkında?

Üst sınır 2 ng/g, bağıl standart belirsizlik %35; sınırın üstünde olma olasılığı en az %95 ise uygun değil; ölçülen 3,3 ng/g.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük w en çok 2 ng/g olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı ret: bir ürün ancak sınırın açıkça dışındaysa reddedilir (örneğin haksız bir suçlamadan kaçınılması gereken yasal denetim ya da doping kontrolünde).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Eurachem/CITAC 2021, Örnek B3: U_F = 1,78, g = 1,6 ng/g, kabul sınırı 3,6 ng/g (lognormal), 3,2 ng/g (normal); 3,3 ng/g yalnız lognormal dağılımla uygun.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="eurachem-compliance-b3-banned-substance-lognormal"></a>

### 2 ng/g sınırında yasaklı madde, u_bağıl = %35: lognormal dağılım, s_G ≈ u_bağıl (Eurachem/CITAC uygunluk rehberi, Örnek B3)

`eurachem-compliance-b3-banned-substance-lognormal`

#### Bu proje ne hakkında?

Üst sınır 2 ng/g, bağıl standart belirsizlik %35; sınırın üstünde olma olasılığı en az %95 ise uygun değil; ölçülen 3,3 ng/g.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük w en çok 2 ng/g olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı ret: bir ürün ancak sınırın açıkça dışındaysa reddedilir (örneğin haksız bir suçlamadan kaçınılması gereken yasal denetim ya da doping kontrolünde).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Eurachem/CITAC 2021, Örnek B3: U_F = 1,78, g = 1,6 ng/g, kabul sınırı 3,6 ng/g (lognormal), 3,2 ng/g (normal); 3,3 ng/g yalnız lognormal dağılımla uygun.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="eurachem-compliance-b3-banned-substance-normal"></a>

### 2 ng/g sınırında yasaklı madde, u_bağıl = %35: u = u_bağıl·L = 0,7 ng/g ile normal dağılım (Eurachem/CITAC uygunluk rehberi, Örnek B3)

`eurachem-compliance-b3-banned-substance-normal`

#### Bu proje ne hakkında?

Üst sınır 2 ng/g, bağıl standart belirsizlik %35; sınırın üstünde olma olasılığı en az %95 ise uygun değil; ölçülen 3,3 ng/g.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük w en çok 2 ng/g olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı ret: bir ürün ancak sınırın açıkça dışındaysa reddedilir (örneğin haksız bir suçlamadan kaçınılması gereken yasal denetim ya da doping kontrolünde).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Eurachem/CITAC 2021, Örnek B3: U_F = 1,78, g = 1,6 ng/g, kabul sınırı 3,6 ng/g (lognormal), 3,2 ng/g (normal); 3,3 ng/g yalnız lognormal dağılımla uygun.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Test belirsizlik oranından koruma bantları (Orta)

Kalibrasyonda kabul sınırları çoğu zaman toleransın ölçüm belirsizliğine oranından (TUR) türetilir. Üç yaygın koruma bandı yöntemi yol açtıkları risklerle birlikte karşılaştırılır.

<a id="tur-guard-band-dobbert-tur2-5"></a>

### Test belirsizlik oranından kabul sınırları: Dobbert'in yönetilen koruma bandı (yöntem 6), TUR = 2,5, %95 tolerans içi süreç için küresel riskler

`tur-guard-band-dobbert-tur2-5`

#### Bu proje ne hakkında?

Tolerans ±1 (normalize), U₉₅ = 2u = 0,4, dolayısıyla TUR = 1/U₉₅ = 2,5. GBF = 1 − M/TUR, M = 1,04 − exp(0,38 ln TUR − 0,54). Ürünlerin süreci merkez etrafında normaldir ve %95'i tolerans içindedir (s.s. 0,510213).


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük Normalised deviation from nominal -1 ile 1 arasında olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Koruma bandı çarpanı yayımlanmış formülden (Dobbert 2008; Delker & Peguero 2022); %95'i tolerans içinde olan normal bir süreç için JCGM 106:2012 9.5'in küresel tüketici ve üretici riskleri, SciPy ile sayısal integrasyonla (tools/research/conformity/tur_risks.py). Suncal 1.7.1 aynı değerleri verir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tur-guard-band-rp10-tur2-0"></a>

### Test belirsizlik oranından kabul sınırları: NCSLI RP-10 koruma bandı, TUR = 2,0, %95 tolerans içi süreç için küresel riskler

`tur-guard-band-rp10-tur2-0`

#### Bu proje ne hakkında?

Tolerans ±1 (normalize), U₉₅ = 2u = 0,5, dolayısıyla TUR = 1/U₉₅ = 2,0. GBF = 1,25 − 1/TUR (TUR ≤ 4). Ürünlerin süreci merkez etrafında normaldir ve %95'i tolerans içindedir (s.s. 0,510213).


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük Normalised deviation from nominal -1 ile 1 arasında olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Koruma bandı çarpanı yayımlanmış formülden (Dobbert 2008; Delker & Peguero 2022); %95'i tolerans içinde olan normal bir süreç için JCGM 106:2012 9.5'in küresel tüketici ve üretici riskleri, SciPy ile sayısal integrasyonla (tools/research/conformity/tur_risks.py). Suncal 1.7.1 aynı değerleri verir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="tur-guard-band-rss-tur1-5"></a>

### Test belirsizlik oranından kabul sınırları: karelerin toplamının kökü koruma bandı, TUR = 1,5, %95 tolerans içi süreç için küresel riskler

`tur-guard-band-rss-tur1-5`

#### Bu proje ne hakkında?

Tolerans ±1 (normalize), U₉₅ = 2u = 0,6667, dolayısıyla TUR = 1/U₉₅ = 1,5. GBF = √(1 − 1/TUR²). Ürünlerin süreci merkez etrafında normaldir ve %95'i tolerans içindedir (s.s. 0,510213).


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük Normalised deviation from nominal -1 ile 1 arasında olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Koruma bandı çarpanı yayımlanmış formülden (Dobbert 2008; Delker & Peguero 2022); %95'i tolerans içinde olan normal bir süreç için JCGM 106:2012 9.5'in küresel tüketici ve üretici riskleri, SciPy ile sayısal integrasyonla (tools/research/conformity/tur_risks.py). Suncal 1.7.1 aynı değerleri verir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Sansürlü sonuçlar ("küçüktür") (Orta)

Bazı laboratuvarlar bir sayı yerine "< 0,015" bildirir. Bunları işlemenin üç yolu karşılaştırılır: silmek, işareti yok saymak ya da sınırın yarısını kullanmak.

<a id="iso13528-e1-censored-deleted"></a>

### Bir YT turunda sansürlü sonuçlar: sansürlü sonuçlar çıkarılarak (ISO 13528 E.1)

`iso13528-e1-censored-deleted`

#### Bu proje ne hakkında?

ISO 13528:2022 E.1: beşi ‘küçüktür’ değeri olan 23 sonuç; Algoritma A'nın robust ortalaması ve standart sapması sansürlü sonuçların nasıl işlendiğine bağlıdır.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Yeterlilik testinde** aynı malzeme 23 laboratuvara gönderilir ve hepsi **result** ölçer. Sonra her laboratuvar, sonucunun atanan değerle uyuşup uyuşmadığını söyleyen bir puan alır.

Atanan değer: katılımcı sonuçlarının dayanıklı ortalaması (aykırı değerlerin etkisini sınırlayan Algoritma A). Yeterlilik değerlendirmesi için standart sapma σ_pt, bir sonucun atanan değerden ne kadar uzak olabileceğine karar verir.


#### Adım 2. Puanlar

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 başarılı, 2 < |z| < 3 şüpheli, |z| ≥ 3 başarısız.
- **ζ ve E_n**, her laboratuvarın bildirdiği belirsizliği de kullanır; laboratuvarın kendi belirsizliğinin gerçekçi olup olmadığını denetler.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 13528:2022 Tablo E.1 (x* = 26,81, s* = 5,29),

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso13528-e1-censored-half"></a>

### Bir YT turunda sansürlü sonuçlar: sınırın 0,5 katı (ISO 13528 E.1)

`iso13528-e1-censored-half`

#### Bu proje ne hakkında?

ISO 13528:2022 E.1: beşi ‘küçüktür’ değeri olan 23 sonuç; Algoritma A'nın robust ortalaması ve standart sapması sansürlü sonuçların nasıl işlendiğine bağlıdır.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Yeterlilik testinde** aynı malzeme 23 laboratuvara gönderilir ve hepsi **result** ölçer. Sonra her laboratuvar, sonucunun atanan değerle uyuşup uyuşmadığını söyleyen bir puan alır.

Atanan değer: katılımcı sonuçlarının dayanıklı ortalaması (aykırı değerlerin etkisini sınırlayan Algoritma A). Yeterlilik değerlendirmesi için standart sapma σ_pt, bir sonucun atanan değerden ne kadar uzak olabileceğine karar verir.


#### Adım 2. Puanlar

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 başarılı, 2 < |z| < 3 şüpheli, |z| ≥ 3 başarısız.
- **ζ ve E_n**, her laboratuvarın bildirdiği belirsizliği de kullanır; laboratuvarın kendi belirsizliğinin gerçekçi olup olmadığını denetler.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 13528:2022 Tablo E.1 (x* = 23,95, s* = 8,60),

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso13528-e1-censored-ignored"></a>

### Bir YT turunda sansürlü sonuçlar: ‘<’ işareti yok sayılarak (ISO 13528 E.1)

`iso13528-e1-censored-ignored`

#### Bu proje ne hakkında?

ISO 13528:2022 E.1: beşi ‘küçüktür’ değeri olan 23 sonuç; Algoritma A'nın robust ortalaması ve standart sapması sansürlü sonuçların nasıl işlendiğine bağlıdır.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Yeterlilik testinde** aynı malzeme 23 laboratuvara gönderilir ve hepsi **result** ölçer. Sonra her laboratuvar, sonucunun atanan değerle uyuşup uyuşmadığını söyleyen bir puan alır.

Atanan değer: katılımcı sonuçlarının dayanıklı ortalaması (aykırı değerlerin etkisini sınırlayan Algoritma A). Yeterlilik değerlendirmesi için standart sapma σ_pt, bir sonucun atanan değerden ne kadar uzak olabileceğine karar verir.


#### Adım 2. Puanlar

- **z = (x − x_pt)/σ_pt**: |z| ≤ 2 başarılı, 2 < |z| < 3 şüpheli, |z| ≥ 3 başarısız.
- **ζ ve E_n**, her laboratuvarın bildirdiği belirsizliği de kullanır; laboratuvarın kendi belirsizliğinin gerçekçi olup olmadığını denetler.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 13528:2022 Tablo E.1 (x* = 26,01, s* = 7,23),

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Homojenlik ve kararlılık (Orta)

Bir yeterlilik testinden önce sağlayıcı, malzemenin tüm birimlerinin aynı olduğunu ve öyle kaldığını göstermelidir. Tekrarlı ölçümler üzerinde bir varyans analizi bunu cevaplar.

<a id="iso13528-e2-arsenic-homogeneity"></a>

### Çikolatada arsenik: YT numunelerinin homojenliği ve kararlılığı (ISO 13528 E.2)

`iso13528-e2-arsenic-homogeneity`

#### Bu proje ne hakkında?

ISO 13528:2022 E.2: ikişer kez ölçülen 10 numuneden numuneler arası standart sapma, 0,3σ_pt ile karşılaştırma ve 60 °C'de altı hafta sonra kararlılık kontrolü.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Homojenlik**: malzemenin birkaç birimi tekrarlı ölçülür. Varyans analizi birimler arası değişimi ölçümün tekrarlanabilirliğinden ayırır; birimler arası standart sapma σ_pt'ye göre küçük olmalıdır.

**Kararlılık**: turun başında ve sonunda ölçülen birimler uyuşmalıdır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 13528:2022 E.2 (Tablo E.2 ve E.3 ile bunlarla birlikte basılı özet istatistikler).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir CRM'den atanan değer (Orta)

Atanan değer sertifikalı bir referans malzemeyle karşılaştırmadan gelir ve onun belirsizliği de değerlendirmenin parçasıdır.

<a id="iso13528-e5-los-angeles-reference-value"></a>

### CRM ile karşılaştırmadan atanan değer: agregaların Los Angeles değeri (ISO 13528 E.5)

`iso13528-e5-los-angeles-reference-value`

#### Bu proje ne hakkında?

ISO 13528:2022 E.5 (7.5): yeterlilik testi numunesi ve benzer bir CRM 20 numunede ikişer kez test edilir; x_pt sertifikalı değer artı ortalama farktır, u(x_pt) CRM belirsizliğini ve farkın standart belirsizliğini birleştirir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **x_pt** (LA units). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
x_pt = x_CRM + d
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **x_CRM**: CRM'nin sertifikalı değeri. En iyi değer 21,62 LA units, standart belirsizlik u = 0,26 LA units (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **d**: 20 numunede ortalama fark YT numunesi − CRM. 20 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 1,72750 LA units; okumalar s = 1,07 LA units standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 1,07/√20 = 0,239 LA units, serbestlik derecesi ν = n − 1 = 19.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 23,35 LA units
- standart belirsizlik u = 0,35 LA units

Kaynak: ISO 13528:2022 E.5 (x_pt = 23,35, u(x_pt) = 0,35 LA birimi; Tablo E.8: d = 1,73, s = 1,07, u(d) = 0,24).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir depolama tankının hacmi (Orta)

Kuvvetlerin çarpımı (π r² h). Bağıl belirsizlikler basitçe birleşir ve Monte Carlo yöntemi GUM sonucunu doğrular.

<a id="possolo-meija-2022-storage-tank-volume"></a>

### Silindirik depolama tankının hacmi: kuvvetler çarpımı, Gauss formülü ve Monte Carlo (Possolo & Meija 2022)

`possolo-meija-2022-storage-tank-volume`

#### Bu proje ne hakkında?

V = πR²H; R = 8,40(3) m ve H = 32,50(7) m.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **V** (m³): Silindirik depolama tankının iç hacmi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
V = pi * R^2 * H
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: Yarıçap, çatının şeritle ölçülen çapından. En iyi değer 8,4 m, standart belirsizlik u = 0,03 m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **H**: Yükseklik, çatı kenarından çekül ile. En iyi değer 32,5 m, standart belirsizlik u = 0,07 m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 7204 m³
- standart belirsizlik u = 54 m³

Kaynak: Possolo & Meija (2022), s. 21–24: V = 7204 m³ ± 54 m³ (Gauss formülü ve Monte Carlo, K = 10⁶).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir Wheatstone köprüsü (Orta)

Basit bir çarpım olmayan bir model: duyarlılık katsayılarının doğru türetilmesi gerekir. Program bunu sembolik olarak yapar.

<a id="possolo-meija-2022-wheatstone-bridge"></a>

### Wheatstone köprüsü: kuvvetler çarpımı olmayan bir ölçüm modeli (Possolo & Meija 2022)

`possolo-meija-2022-wheatstone-bridge`

#### Bu proje ne hakkında?

Dengede R_U/R_G = R_F/R_EH; R_EH = 1/(R_E⁻¹ + R_H⁻¹), paralel E ve H'nin ortak direnci; dolayısıyla R_U = R_G R_F (R_E⁻¹ + R_H⁻¹).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **R_U** (Ω): Dengelenmiş Wheatstone köprüsündeki bilinmeyen U direncinin değeri. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
R_U = R_G * R_F * (1/R_E + 1/R_H)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R_E**: Dekad direnç E (EICO, %0,5). En iyi değer 951 Ω, standart belirsizlik u = 5 Ω (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **R_F**: Sabit direnç F. En iyi değer 997 Ω, standart belirsizlik u = 5 Ω (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **R_G**: Dekad direnç G (General Radio, %0,05). En iyi değer 909 Ω, standart belirsizlik u = 0,5 Ω (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **R_H**: E'ye paralel direnç ikame kutusu H. En iyi değer 225200 Ω, standart belirsizlik u = 2300 Ω (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 957 Ω
- standart belirsizlik u = 7 Ω

Kaynak: Possolo & Meija (2022), s. 25: R_U = 957 Ω, u(R_U) ≈ 7 Ω; katkılar F %48, G %0,6, E %52, H %0,004.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## İzotop bollukları (Orta)

İki ölçülen izotop oranından silisyum izotoplarının bollukları. Sonuçlar korelasyonludur, çünkü toplamları 1 olmalıdır.

<a id="iupac2024-4b-silicon-isotopic-abundances"></a>

### Ölçülen iki izotop miktar oranından silisyumun izotop bollukları (IUPAC 2024, Örnek 4B)

`iupac2024-4b-silicon-isotopic-abundances`

#### Bu proje ne hakkında?

x₂₈ = 1/(1 + R₂₉/₂₈ + R₃₀/₂₈), x₂₉ = R₂₉/₂₈·x₂₈, x₃₀ = R₃₀/₂₈·x₂₈: iki girdiden üç çıktı; kovaryans matrisi Σ_y = JΣ_xJᵀ (Formül 3).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 3 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **x28** mol/mol: `x28 = 1/(1 + R29 + R30)`
- **x29** mol/mol: `x29 = R29/(1 + R29 + R30)`
- **x30** mol/mol: `x30 = R30/(1 + R29 + R30)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R29**: Miktar oranı n(²⁹Si)/n(²⁸Si), Karışım III. En iyi değer 0,0500657 mol/mol, standart belirsizlik u = 0,0000025 mol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **R30**: Miktar oranı n(³⁰Si)/n(²⁸Si), Karışım III. En iyi değer 0,0329035 mol/mol, standart belirsizlik u = 0,0000034 mol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: IUPAC 2024, Örnek 4B: x₂₈ = 0,923 3873(36), x₂₉ = 0,046 2300(22), x₃₀ = 0,030 3827(30) mol/mol.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Beta dağılımıyla anlatılan bir saflık (Orta)

Yine kadmiyum standardı; bu kez saflık %100'ü aşamayan bir beta dağılımıyla. Monte Carlo böyle bir dağılımı doğal olarak işler.

<a id="iupac2024-4h-cadmium-standard-beta-mcm"></a>

### Saflık için beta dağılımıyla Monte Carlo ile kadmiyum kalibrasyon standardı (IUPAC 2024, Örnek 4H; QUAM A1)

`iupac2024-4h-cadmium-standard-beta-mcm`

#### Bu proje ne hakkında?

γ_Cd = w_Cd m_Cd/V; m_Cd = 100,28(5) mg, w_Cd = 0,999 900(58) g/g ve V = 100,00(7) mL. 1 g/g ile sınırlı saflık beta dağılımıyla, hacim üçgen dağılımla modellenir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **gamma_Cd** (mg/mL): Kalibrasyon standardındaki kadmiyum kütle derişimi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
gamma_Cd = w_Cd * m_Cd / V
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m_Cd**: Kadmiyum kütlesi (normal; 0 mg'deki kesmenin etkisi yoktur). En iyi değer 100,28 mg, standart belirsizlik u = 0,05 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **w_Cd**: Kadmiyum saflığı, belirtilen ortalama ve s.s. ile [0, 1] üzerinde beta dağılımı. 0 ile 1 g/g arasında beta dağılımı; ortalama 0,9999, u = 0,000058 g/g (bir aralığa hapsolmuş büyüklük).
3. **V**: Balon hacmi, s.s. 0,07 mL olan simetrik üçgen. Değer 99,8285357 ile 100,17 mL arasında ve merkez 100'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,1714643/√6 = 0,0700 mL.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,0027 mg/mL
- standart belirsizlik u = 0,0009 mg/mL

Kaynak: IUPAC 2024, Örnek 4H: γ = 1,0027 mg/mL, u = 0,0009 mg/mL (Gauss ve Monte Carlo, K = 10⁷), U95 = 0,0017 mg/mL (%2,5–97,5 yüzdelik aralığının yarısı). numpy yüzdelikleri (10⁷ çekiliş): 1,001 03 ve 1,004 37 mg/mL.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## 1'in altında asimetrik bir aralık (Orta)

Benzoik asidin saflığı 1'e yakındır ve onu aşamaz; bu yüzden aralığı simetrik değildir. Onu bir beta dağılımı anlatır.

<a id="iupac2024-5c-benzoic-acid-purity-beta"></a>

### Beta dağılımı olarak ifade edilen benzoik asit saflığı (NIST PS1): 1 g/g'nin altında asimetrik bir aralık (IUPAC 2024, Örnek 5C)

`iupac2024-5c-benzoic-acid-purity-beta`

#### Bu proje ne hakkında?

Kütle kesri hakkındaki bilgi durumu α = 88 873,7 ve β = 7,11 olan bir beta dağılımıdır; standart sapması standart belirsizliktir, inanılırlık aralıkları kantillerinden çıkar.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_BA** (g/g): NIST PS1'de benzoik asidin kütle kesri. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_BA = w
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 1 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **w**: Saflığın Bayesçi sonsalına yaklaşan beta dağılımı. 0 ile 1 g/g arasında beta dağılımı (bir aralığa hapsolmuş, asimetrik olabilen bir büyüklük; örneğin 1'e yakın bir saflık): ortalama 0,999920 g/g, u = 3,00 × 10⁻⁵ g/g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,99992 g/g
- standart belirsizlik u = 0,00003 g/g

Kaynak: IUPAC 2024, Örnek 5C: α = 88 873,7 ve β = 7,11 olan beta dağılımı; ortalama ve s.s. 0,999 920 ve 0,000 030 g/g; kantiller 0,999 851 ve 0,999 968 g/g (scipy).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Uç mastar kalibrasyonu (GUM H.1) (Orta)

GUM'un kendisinin ünlü örneği. B tipi terimler için serbestlik derecelerini ve kapsam faktörünü seçmek için Welch–Satterthwaite formülünü kullanır.

<a id="gum-h1-end-gauge"></a>

### Uç mastarı kalibrasyonu (GUM H.1)

`gum-h1-end-gauge`

#### Bu proje ne hakkında?

JCGM 100 Ek H.1: 50 mm'lik bir uç mastarının kalibrasyonu; Welch–Satterthwaite etkin serbestlik derecesi ve %99 için k.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **l** (nm): Uç mastarının boyu. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
l = l_s + d - l_s*(d_alpha*theta + alpha_s*d_theta)
```

Denklemi okunur tutmak için önce 1 ara büyüklük hesaplanır:

- `theta = theta_bar + Delta`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **l_s**: Standardın boyu (kalibrasyon sertifikası). En iyi değer 50000623 nm, standart belirsizlik u = 25 nm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 18 serbestlik derecesi vardır.
2. **d**: Ölçülen boy farkı (5 okumanın ortalaması, komparatör). En iyi değer 215 nm, standart belirsizlik u = 9,7 nm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 25,6 serbestlik derecesi vardır.
3. **alpha_s**: Standardın ısıl genleşme katsayısı. Yalnızca sınırlar biliniyor: değer 0,0000095 ile 0,0000135 1/°C arasında (0,0000115 ± 0,000002) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000002/√3 = 1,15 × 10⁻⁶ 1/°C.
4. **theta_bar**: Tablanın ortalama sıcaklık sapması. En iyi değer -0,1 °C, standart belirsizlik u = 0,2 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **Delta**: Periyodik sıcaklık değişimi. Değer -0,5 ile 0,5 °C arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,5/√2 = 0,354 °C.
6. **d_alpha**: Genleşme katsayıları farkı. Yalnızca sınırlar biliniyor: değer -0,000001 ile 0,000001 1/°C arasında (0 ± 0,000001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000001/√3 = 5,77 × 10⁻⁷ 1/°C. Bu standart belirsizliğin kendisi de yaklaşık bilinir: 50 serbestlik derecesi vardır.
7. **d_theta**: Mastarlar arası sıcaklık farkı. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 °C arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 °C. Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %99,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 50000838 nm
- standart belirsizlik u = 32 nm
- kapsam faktörü k = 2,92
- genişletilmiş belirsizlik U = 93 nm

Kaynak: JCGM 100:2008 H.1.4–H.1.6 (u_c = 32 nm, ν_eff = 16,7, k = 2,92, U = 93 nm).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Korelasyonlu girdiler (GUM H.2) (Orta)

Gerilim, akım ve faz eşzamanlı ölçülür, bu yüzden ortalamaları korelasyonludur. Direnç, reaktans ve empedans korelasyon terimleriyle hesaplanır.

<a id="gum-h2-impedance"></a>

### Korelasyonlu V ve I'dan empedans büyüklüğü (GUM H.2)

`gum-h2-impedance`

#### Bu proje ne hakkında?

GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden Z = V/I empedans büyüklüğü.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **Z** (Ω): Z = v/i empedans büyüklüğü. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
Z = 1000 * V / I
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V**: Beş gerilim okumasının ortalaması. En iyi değer 4,999 V, standart belirsizlik u = 0,0032094 V (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **I**: Beş akım okumasının ortalaması. En iyi değer 19,661 mA, standart belirsizlik u = 0,0094710 mA (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **phi**: Beş faz açısı okumasının ortalaması. En iyi değer 1,04446 rad, standart belirsizlik u = 0,00075206 rad (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- V ↔ I: r = -0,355311
- V ↔ phi: r = 0,857624
- I ↔ phi: r = -0,645111


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 254,26 Ω
- standart belirsizlik u = 0,236 Ω

Kaynak: JCGM 100:2008 Tablo H.3: Z = 254,26 Ω, u_c = 0,236 Ω.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum-h2-reactance"></a>

### Korelasyonlu V, I ve φ'den reaktans (GUM H.2)

`gum-h2-reactance`

#### Bu proje ne hakkında?

GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden X = (V/I) sin φ reaktansı.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **X** (Ω): X = (v/i) sin φ reaktansı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
X = 1000 * V / I * sin(phi)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V**: Beş gerilim okumasının ortalaması. En iyi değer 4,999 V, standart belirsizlik u = 0,0032094 V (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **I**: Beş akım okumasının ortalaması. En iyi değer 19,661 mA, standart belirsizlik u = 0,0094710 mA (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **phi**: Beş faz açısı okumasının ortalaması. En iyi değer 1,04446 rad, standart belirsizlik u = 0,00075206 rad (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- V ↔ I: r = -0,355311
- V ↔ phi: r = 0,857624
- I ↔ phi: r = -0,645111


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 219,847 Ω
- standart belirsizlik u = 0,295 Ω

Kaynak: JCGM 100:2008 Tablo H.3: X = 219,847 Ω, u_c = 0,295 Ω.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum-h2-resistance"></a>

### Korelasyonlu V, I ve φ'den direnç (GUM H.2)

`gum-h2-resistance`

#### Bu proje ne hakkında?

GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden R = (V/I) cos φ direnci.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **R** (Ω): R = (v/i) cos φ direnci. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
R = 1000 * V / I * cos(phi)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V**: Beş gerilim okumasının ortalaması. En iyi değer 4,999 V, standart belirsizlik u = 0,0032094 V (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **I**: Beş akım okumasının ortalaması. En iyi değer 19,661 mA, standart belirsizlik u = 0,0094710 mA (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **phi**: Beş faz açısı okumasının ortalaması. En iyi değer 1,04446 rad, standart belirsizlik u = 0,00075206 rad (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- V ↔ I: r = -0,355311
- V ↔ phi: r = 0,857624
- I ↔ phi: r = -0,645111


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 127,732 Ω
- standart belirsizlik u = 0,071 Ω

Kaynak: JCGM 100:2008 Tablo H.3: R = 127,732 Ω, u_c = 0,071 Ω.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## NIST Uncertainty Machine örnekleri (Orta)

NIST Uncertainty Machine kılavuzundan örnekler: geniş üçgen bölenli bir oran, bir uç mastar, bir direnç ve Stefan–Boltzmann sabiti. GUM ve Monte Carlo sonuçlarını karşılaştırırlar.

<a id="num-10-resistance"></a>

### GUM Tablo H.2'nin yuvarlanmış değerleriyle direnç (NIST Uncertainty Machine §10)

`num-10-resistance`

#### Bu proje ne hakkında?

GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden R = (V/I) cos φ direnci.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **R** (Ω): R = (v/i) cos φ direnci. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
R = 1000 * V / I * cos(phi)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V**: Beş gerilim okumasının ortalaması. En iyi değer 4,999 V, standart belirsizlik u = 0,0032 V (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **I**: Beş akım okumasının ortalaması. En iyi değer 19,661 mA, standart belirsizlik u = 0,0095 mA (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **phi**: Beş faz açısı okumasının ortalaması. En iyi değer 1,04446 rad, standart belirsizlik u = 0,00075 rad (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- V ↔ I: r = -0,36
- V ↔ phi: r = 0,86
- I ↔ phi: r = -0,65


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 127,732 Ω
- standart belirsizlik u = 0,07 Ω

Kaynak: GUM: yuvarlanmış girdilerle yayılım yasası (0,0700 Ω); NIST Uncertainty Machine kılavuzu §10 (M = 10⁶): Gauss formülü u = 0,07 Ω; ortalama 127,732 Ω, standart sapma 0,0699 Ω, %95 aralığı (127,595; 127,869) Ω.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="num-3-ratio-abc"></a>

### Geniş üçgen bölenli çarpım ve oran y = ab/c (NIST Uncertainty Machine §3)

`num-3-ratio-abc`

#### Bu proje ne hakkında?

NUM kılavuzu §3: normal a, dikdörtgen b ve bağıl belirsizliği %30 olan üçgen bölen c; birinci mertebe GUM sonucu Monte Carlo sonucundan çok uzaktır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **y**: Çıktı büyüklüğü. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
y = a * b / c
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **a**: Normal çarpan. En iyi değer 32, standart belirsizlik u = 0,5 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **b**: Dikdörtgen çarpan. Yalnızca sınırlar biliniyor: değer 0,8566987 ile 0,9433013 arasında (0,9 ± 0,0433013) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0433013/√3 = 0,0250.
3. **c**: Üçgen bölen. Değer 0,26515 ile 1,734847 arasında ve merkez 1'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,734847/√6 = 0,300.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 28,8
- standart belirsizlik u = 8,7

Kaynak: NIST Uncertainty Machine kılavuzu §3, Şekil 1 (Gauss formülü y = 28,8, u = 8,7; Monte Carlo, M = 10⁶: ortalama 32,2, standart sapma 13, %95 aralığı (18,2; 67)).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="num-8-end-gauge-gaussian"></a>

### Normal girdilerle uç mastarı kalibrasyonu (NIST Uncertainty Machine §8)

`num-8-end-gauge-gaussian`

#### Bu proje ne hakkında?

NUM kılavuzu §8: GUM H.1'in birinci mertebe bütçesindeki altı girdiyle, hepsi normal dağılımlı; Monte Carlo yöntemi Gauss formülünün kaçırdığı ikinci mertebe katkıları bulur.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **l** (nm): Uç mastarının 20 °C'deki uzunluğu. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
l = l_S + d - l_S * (d_alpha * theta + alpha_S * d_theta)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **l_S**: Referans standardın uzunluğu. En iyi değer 50000623 nm, standart belirsizlik u = 25 nm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **d**: Ölçülen uzunluk farkı. En iyi değer 215 nm, standart belirsizlik u = 9,7 nm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **d_alpha**: Genleşme katsayılarının farkı. En iyi değer 0 1/°C, standart belirsizlik u = 5,8e-7 1/°C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **theta**: Sıcaklığın 20 °C'den sapması. En iyi değer -0,1 °C, standart belirsizlik u = 0,41 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **alpha_S**: Standardın genleşme katsayısı. En iyi değer 0,0000115 1/°C, standart belirsizlik u = 0,0000012 1/°C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **d_theta**: Mastarlar arasındaki sıcaklık farkı. En iyi değer 0 °C, standart belirsizlik u = 0,029 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %99,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 50000838 nm
- standart belirsizlik u = 31,7 nm

Kaynak: NIST Uncertainty Machine kılavuzu §8, Şekil 3 (Gauss formülü 50 000 838 nm, u = 31,7 nm; Monte Carlo, M = 10⁶: ortalama 50 000 837,9 nm, standart sapma 33,9 nm, %99 simetrik aralık (50 000 749; 50 000 927) nm).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="num-11-stefan-boltzmann"></a>

### CODATA 2010 değerlerinden Stefan–Boltzmann sabiti (NIST Uncertainty Machine §11)

`num-11-stefan-boltzmann`

#### Bu proje ne hakkında?

NUM kılavuzu §11: 2010 CODATA değerlerinden σ = 32π⁵hR⁴R∞⁴/(15A_r(e)⁴M_u⁴c⁶α⁸); belirsizliğin neredeyse tamamı molar gaz sabiti R'den gelir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **sigma** (10⁻⁸ W m⁻² K⁻⁴): Stefan–Boltzmann sabiti. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
sigma = 1e8 * 32 * pi^5 * h * R^4 * R_inf^4 / (15 * A_e^4 * (1e-3)^4 * 299792458^6 * alpha^8)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **h**: Planck sabiti. En iyi değer 6,6261 × 10⁻³⁴ J s, standart belirsizlik u = 2,9e-41 J s (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **R**: Molar gaz sabiti. En iyi değer 8,3144621 J/(mol K), standart belirsizlik u = 0,0000075 J/(mol K) (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **R_inf**: Rydberg sabiti. En iyi değer 10973732 1/m, standart belirsizlik u = 0,000055 1/m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **A_e**: Elektronun bağıl atom kütlesi. En iyi değer 0,00054858, standart belirsizlik u = 2,2e-13 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **alpha**: İnce yapı sabiti. En iyi değer 0,0072974, standart belirsizlik u = 2,4e-12 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 5,6703725 10⁻⁸ W m⁻² K⁻⁴
- standart belirsizlik u = 0,0000205 10⁻⁸ W m⁻² K⁻⁴

Kaynak: NIST Uncertainty Machine kılavuzu §11, Şekil 7: σ = 5,670 3725 × 10⁻⁸, u(σ) = 2,05 × 10⁻¹³ W m⁻² K⁻⁴ (Gauss formülü ve Monte Carlo), %95 aralığı (5,670 332; 5,670 412 6) × 10⁻⁸ W m⁻² K⁻⁴.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir ısıl çiftin kalibrasyonu (Orta)

Bir ısıl çift bir fırında bir referans ısıl çiftle karşılaştırılır. İki bütçe: fırının sıcaklığı ve bilinmeyen ısıl çiftin emk'si.

<a id="ea402-s5-thermocouple-emf"></a>

### Termokupl kalibrasyonu: bilinmeyenin emk'sı

`ea402-s5-thermocouple-emf`

#### Bu proje ne hakkında?

EA-4/02 S5 (ikinci bütçe): kalibre edilen N tipi termokuplun 1000 °C'deki emk'sı.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **V_X** (µV): Termokuplun 1000 °C'deki emk'sı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
V_X = V_iX + V_iX1 + V_iX2 + V_R + V_LX + dt/0.026 - t_0X/0.039
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **V_iX**: Ortalama voltmetre okuması. En iyi değer 36248 µV, standart belirsizlik u = 1,6 µV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **V_iX1**: Voltmetre kalibrasyonu. En iyi değer 0 µV, standart belirsizlik u = 1 µV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **V_iX2**: Voltmetre çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 µV arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 µV.
4. **V_R**: Kontak parazitik gerilimleri. Yalnızca sınırlar biliniyor: değer -2 ile 2 µV arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 µV.
5. **V_LX**: Uzatma kabloları. Yalnızca sınırlar biliniyor: değer -5 ile 5 µV arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 µV.
6. **dt**: Kalibrasyon noktası eksi fırın sıcaklığı. En iyi değer -0,5 °C, standart belirsizlik u = 0,641 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **t_0X**: Referans eklem sıcaklığı. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 °C arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 °C.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 36229 µV
- standart belirsizlik u = 25 µV
- genişletilmiş belirsizlik U = 50 µV

Kaynak: EA-4/02 M:2022 S5.10–S5.12 (u(V_X) = 25,0 µV, U = 50 µV).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="ea402-s5-thermocouple-furnace"></a>

### Termokupl kalibrasyonu: fırın sıcaklığı

`ea402-s5-thermocouple-furnace`

#### Bu proje ne hakkında?

EA-4/02 S5 (birinci bütçe): 1000 °C'deki fırının sıcaklığının N tipi referans termokupl ile ölçülmesi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **t_X** (°C): Fırının sıcaklığı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
t_X = t_S + 0.077*(V_iS1 + V_iS2 + V_R) - 0.077/0.189*t_0S + dt_S + dt_D + dt_F
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **t_S**: Referans termokupldan sıcaklık. En iyi değer 1000,5 °C, standart belirsizlik u = 0,1 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **V_iS1**: Voltmetre kalibrasyonu. En iyi değer 0 µV. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 2 µV, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 2/2 = 1,00 µV olur (normal dağılım).
3. **V_iS2**: Voltmetre çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 µV arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 µV.
4. **V_R**: Kontak parazitik gerilimleri. Yalnızca sınırlar biliniyor: değer -2 ile 2 µV arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 µV.
5. **t_0S**: Soğuk eklemin referans sıcaklığı. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 °C arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 °C.
6. **dt_S**: Referans termokupl kalibrasyonu. En iyi değer 0 °C. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,3 °C, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,3/2 = 0,150 °C olur (normal dağılım).
7. **dt_D**: Referans termokuplun sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,3 ile 0,3 °C arasında (0 ± 0,3) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,3/√3 = 0,173 °C.
8. **dt_F**: Fırın sıcaklığının homojen olmaması. Yalnızca sınırlar biliniyor: değer -1 ile 1 °C arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 °C.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1000,5 °C
- standart belirsizlik u = 0,641 °C
- genişletilmiş belirsizlik U = 1,3 °C

Kaynak: EA-4/02 M:2022 S5.9 (u(t_X) = 0,641 °C).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## 18 GHz'de bir güç sensörü (Orta)

Bir mikrodalga güç sensörü: sensör ile kaynak arasındaki uyumsuzluk U biçimli (arksinüs) bir terimdir.

<a id="ea402-s6-power-sensor-18ghz"></a>

### Bir güç sensörünün 18 GHz'de kalibrasyonu

`ea402-s6-power-sensor-18ghz`

#### Bu proje ne hakkında?

EA-4/02 S6: bir termistör güç sensörünün kalibrasyon faktörünün standart sensöre karşı belirlenmesi; uyumsuzluk terimleri U biçimlidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **K_X**: Sensörün kalibrasyon faktörü. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
K_X = (K_S + dK_D) * M_Sr * M_Xc / (M_Sc * M_Xr) * p_Cr * p_Cc * p
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **K_S**: Standardın kalibrasyon faktörü. En iyi değer 0,957. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,011, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,011/2 = 0,00550 olur (normal dağılım).
2. **dK_D**: Standardın sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,003 ile 0,001 arasında (-0,001 ± 0,002) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,002/√3 = 0,00115.
3. **M_Sr**: Uyumsuzluk, standart, referans kapısı. Değer 0,9992 ile 1,0008 arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,0008/√2 = 0,000566.
4. **M_Sc**: Uyumsuzluk, standart, 18 GHz. Değer 0,986 ile 1,014 arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,014000/√2 = 0,00990.
5. **M_Xr**: Uyumsuzluk, bilinmeyen, referans kapısı. Değer 0,9992 ile 1,0008 arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,0008/√2 = 0,000566.
6. **M_Xc**: Uyumsuzluk, bilinmeyen, 18 GHz. Değer 0,9832 ile 1,0168 arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,016800/√2 = 0,0119.
7. **p_Cr**: Güç ölçer, referans oranı. En iyi değer 1, standart belirsizlik u = 0,00142 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **p_Cc**: Güç ölçer, 18 GHz oranı. En iyi değer 1, standart belirsizlik u = 0,000142 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **p**: Gözlenen güç oranı (3 ölçüm). 3 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 0,975988; okumalar s = 0,00833 standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,00833/√3 = 0,00481, serbestlik derecesi ν = n − 1 = 2.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,933
- standart belirsizlik u = 0,01623
- genişletilmiş belirsizlik U = 0,032

Kaynak: EA-4/02 M:2022 S6.12–S6.13 (u = 0,0162, U = 0,032, ν_eff ≈ 310).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir adımlı zayıflatıcı (Orta)

Bir 30 dB zayıflatıcı. dB cinsinden belirsizlikler, uyumsuzluk terimleri ve bağlantıların tekrarlanabilirliği.

<a id="ea402-s7-step-attenuator-30db"></a>

### Koaksiyel kademeli zayıflatıcının kalibrasyonu (30 dB)

`ea402-s7-step-attenuator-30db`

#### Bu proje ne hakkında?

EA-4/02 S7: koaksiyel zayıflatıcının 10 GHz'de 30 dB kademesi; uyumsuzluk S parametrelerinden.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **L_X** (dB): 30 dB kademesinin zayıflatması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
L_X = L_S + dL_S + dL_D + dL_M + dL_K + dL_ib - dL_ia + dL_0b - dL_0a
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **L_S**: Gözlenen zayıflatma (4 okuma). 4 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 30,0403 dB; okumalar s = 0,0183 dB standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,0183/√4 = 0,00913 dB, serbestlik derecesi ν = n − 1 = 3.
2. **dL_S**: Standardın kalibrasyonu. En iyi değer 0,003 dB. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,005 dB, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,005/2 = 0,00250 dB olur (normal dağılım).
3. **dL_D**: Standardın sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,002 ile 0,002 dB arasında (0 ± 0,002) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,002/√3 = 0,00115 dB.
4. **dL_M**: Uyumsuzluk (Eş. S7.2). En iyi değer 0 dB, standart belirsizlik u = 0,019788 dB (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **dL_K**: Kaçak. Yalnızca sınırlar biliniyor: değer -0,003 ile 0,003 dB arasında (0 ± 0,003) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,003/√3 = 0,00173 dB.
6. **dL_ia**: Çözünürlük, referans ayarı. Yalnızca sınırlar biliniyor: değer -0,0005 ile 0,0005 dB arasında (0 ± 0,0005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0005/√3 = 0,000289 dB.
7. **dL_ib**: Çözünürlük, zayıflatma ayarı. Yalnızca sınırlar biliniyor: değer -0,0005 ile 0,0005 dB arasında (0 ± 0,0005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0005/√3 = 0,000289 dB.
8. **dL_0a**: Gürültü, referans ayarı. En iyi değer 0 dB, standart belirsizlik u = 0,002 dB (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **dL_0b**: Gürültü, zayıflatma ayarı. En iyi değer 0 dB, standart belirsizlik u = 0,002 dB (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 30,043 dB
- standart belirsizlik u = 0,0223 dB
- genişletilmiş belirsizlik U = 0,045 dB

Kaynak: EA-4/02 M:2022 S7.12–S7.13 (u = 0,0223 dB, U = 0,045 dB).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir halka mastar (Orta)

Bir 90 mm halka mastarın çapı ve sıcaklık düzeltmesi, iki bütçede.

<a id="ea402-s13-ring-gauge-90mm"></a>

### 90 mm'lik halka mastarın kalibrasyonu

`ea402-s13-ring-gauge-90mm`

#### Bu proje ne hakkında?

EA-4/02 S13: halka mastar çapının tek koordinatlı ölçüm makinesinde 40 mm'lik ayar halkasıyla karşılaştırılması (90 mm'ye göre µm).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **d_X** (µm): Çapın 90 mm'den sapması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
d_X = d_S + dl + dl_0 + dl_i + dl_T + dl_P + dl_E + dl_A
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **d_S**: Ayar halkası çap sapması (40 mm). En iyi değer 0,7 µm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,2 µm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,2/2 = 0,100 µm olur (normal dağılım).
2. **dl**: 50 mm'ye göre gözlenen çap farkı (5 okuma). 5 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama -0,464000 µm; okumalar s = 0,328 µm standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,328/√5 = 0,147 µm, serbestlik derecesi ν = n − 1 = 4.
3. **dl_0**: Komparatörün sıfırlanması (s_p(0) = 0,25 µm). En iyi değer 0 µm, standart belirsizlik u = 0,25 µm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **dl_i**: Makine skalasının kalibrasyonu ve doğrusallığı. Yalnızca sınırlar biliniyor: değer -0,375 ile 0,375 µm arasında (0 ± 0,375) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,375/√3 = 0,217 µm.
5. **dl_T**: Sıcaklık düzeltmesi (alt bütçe). En iyi değer 0 µm, standart belirsizlik u = 0,148 µm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **dl_P**: Eş eksenlilik (kosinüs) hatası. En iyi değer -0,003704 µm, standart belirsizlik u = 0,006525 µm (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **dl_E**: Elastik deformasyon. Yalnızca sınırlar biliniyor: değer -0,03 ile 0,03 µm arasında (0 ± 0,03) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,03/√3 = 0,0173 µm.
8. **dl_A**: Abbe hatası. Yalnızca sınırlar biliniyor: değer -0,02 ile 0,02 µm arasında (0 ± 0,02) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,02/√3 = 0,0115 µm.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,23230 µm
- standart belirsizlik u = 0,411 µm
- genişletilmiş belirsizlik U = 0,82 µm

Kaynak: EA-4/02 M:2022 S13.10–S13.12 (u = 0,411 µm, U = 0,82 µm, 0,9 µm olarak raporlanmış; tolerans, burada u(Δl) = 0,29 µm, basılı değer 0,30 µm olmasını kapsar); tahmin gözlemlerden yeniden hesaplanmıştır: 0,7 − 0,464 − 0,0037 µm (basılı 90,000 25 mm, notlara bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="ea402-s13-ring-gauge-temperature"></a>

### Halka mastar kalibrasyonu: sıcaklık düzeltmesi

`ea402-s13-ring-gauge-temperature`

#### Bu proje ne hakkında?

EA-4/02 S13: halka mastar karşılaştırmasının sıcaklık alt bütçesi; sıfır ortalamalı sıcaklık sapmalarının genleşme katsayılarıyla çarpımları.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dl_T** (µm): Sıcaklık düzeltmesi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dl_T = l_TA + l_TS - l_TX + l_TR
```

Denklemi okunur tutmak için önce 4 ara büyüklük hesaplanır:

- `l_TA = (40000*(a_S - a_R) - 90000*(a_X - a_R))*dt_A`
- `l_TS = 40000*a_S*dt_S`
- `l_TX = 90000*a_X*dt_X`
- `l_TR = (40000 - 90000)*a_R*dt_R`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **a_S**: Ayar halkasının genleşme katsayısı. Yalnızca sınırlar biliniyor: değer 0,0000105 ile 0,0000125 1/K arasında (0,0000115 ± 0,000001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000001/√3 = 5,77 × 10⁻⁷ 1/K.
2. **a_X**: Halka mastarın genleşme katsayısı. Yalnızca sınırlar biliniyor: değer 0,0000105 ile 0,0000125 1/K arasında (0,0000115 ± 0,000001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000001/√3 = 5,77 × 10⁻⁷ 1/K.
3. **a_R**: Makine skalasının genleşme katsayısı. Yalnızca sınırlar biliniyor: değer 0,0000105 ile 0,0000125 1/K arasında (0,0000115 ± 0,000001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000001/√3 = 5,77 × 10⁻⁷ 1/K.
4. **dt_A**: Ortalama sıcaklığın 20 °C'den sapması. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 K arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 K.
5. **dt_S**: Sıcaklık farkı, ayar halkası. Yalnızca sınırlar biliniyor: değer -0,2 ile 0,2 K arasında (0 ± 0,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,2/√3 = 0,115 K.
6. **dt_X**: Sıcaklık farkı, halka mastar. Yalnızca sınırlar biliniyor: değer -0,2 ile 0,2 K arasında (0 ± 0,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,2/√3 = 0,115 K.
7. **dt_R**: Sıcaklık farkı, skala. Yalnızca sınırlar biliniyor: değer -0,2 ile 0,2 K arasında (0 ± 0,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,2/√3 = 0,115 K.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 µm
- standart belirsizlik u = 0,15 µm

Kaynak: EA-4/02 M:2022 S13.5 (u(δl_T) = 0,15 µm).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir güç sensörü (UKAS M3003) (Orta)

Güç sensörü bütçesinin İngiliz sürümü, uyumsuzluk terimiyle.

<a id="m3003-k2-power-sensor-18ghz"></a>

### 18 GHz'de koaksiyel güç sensörü (UKAS M3003 K.2)

`m3003-k2-power-sensor-18ghz`

#### Bu proje ne hakkında?

Bilinmeyen bir güç sensörünün kalibrasyon faktörü, izlenen bir kaynakta standart sensörle yer değiştirme yöntemiyle; dört bağlantı.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **K_X** (%): Bilinmeyen sensörün kalibrasyon faktörü. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
K_X = K_obs + dK_S + dD_S + d_DC + d_REF + M_1 + M_2 + M_3 + M_4
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **K_obs**: Gözlenen kalibrasyon faktörü, dört bağlantı. 4 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 93,1550 %; okumalar s = 0,741 % standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,741/√4 = 0,371 %, serbestlik derecesi ν = n − 1 = 3.
2. **dK_S**: Standardın kalibrasyon faktörü (U = %1,1, k = 2). En iyi değer 0 %. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 1,1 %, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 1,1/2 = 0,550 % olur (normal dağılım).
3. **dD_S**: Kalibrasyondan beri standardın sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,2 ile 0,2 % arasında (0 ± 0,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,2/√3 = 0,115 %.
4. **d_DC**: Ölçüm düzeni doğrusallığı (U = %0,1, k = 2). En iyi değer 0 %. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,1 %, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,1/2 = 0,0500 % olur (normal dağılım).
5. **d_REF**: 50 MHz referansın kararlılığı. Yalnızca sınırlar biliniyor: değer -0,2 ile 0,2 % arasında (0 ± 0,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,2/√3 = 0,115 %.
6. **M_1**: Uyumsuzluk, standart sensör, 50 MHz. Değer -0,08 ile 0,08 % arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,08/√2 = 0,0566 %.
7. **M_2**: Uyumsuzluk, bilinmeyen sensör, 50 MHz. Değer -0,08 ile 0,08 % arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,08/√2 = 0,0566 %.
8. **M_3**: Uyumsuzluk, standart sensör, 18 GHz. Değer -1,4 ile 1,4 % arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 1,4/√2 = 0,990 %.
9. **M_4**: Uyumsuzluk, bilinmeyen sensör, 18 GHz. Değer -1,68 ile 1,68 % arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 1,68/√2 = 1,19 %.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 93,16 %
- standart belirsizlik u = 1,69 %
- genişletilmiş belirsizlik U = 3,39 %

Kaynak: UKAS M3003 6. baskı K.2.9–K.2.10: u_c = %1,69, U = %3,39; %93,2 ± %3,4.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir zayıflatıcı (UKAS M3003) (Orta)

10 GHz'de bir 30 dB zayıflatıcı; uyumsuzluk terimini hesaplamanın iki yoluyla.

<a id="m3003-k3-attenuator-30db-mismatch-e5"></a>

### E5 formülünden uyumsuzluk terimiyle 30 dB zayıflatıcı (UKAS M3003 K.3, E.9.5)

`m3003-k3-attenuator-30db-mismatch-e5`

#### Bu proje ne hakkında?

Örneğin verdiği katsayılardan yeniden hesaplanan uyumsuzluk standart sapmasıyla K.3 bütçesi (bulgu F57).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **A_X** (dB): 10 GHz'de 30 dB kademenin zayıflaması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
A_X = A_obs + dA_IF + dD_IF + dL_M + dR_D + dM + dA_L
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **A_obs**: Gösterilen zayıflama A_b − A_a, dört ölçüm. 4 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 30,0500 dB; okumalar s = 0,0183 dB standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,0183/√4 = 0,00913 dB, serbestlik derecesi ν = n − 1 = 3.
2. **dA_IF**: Referans IF zayıflatıcının kalibrasyonu (U = 0,005 dB, k = 2). En iyi değer 0 dB. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,005 dB, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,005/2 = 0,00250 dB olur (normal dağılım).
3. **dD_IF**: IF zayıflatıcının sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,002 ile 0,002 dB arasında (0 ± 0,002) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,002/√3 = 0,00115 dB.
4. **dL_M**: Karıştırıcı doğrusal olmama (10 dB başına ±0,002 dB). Yalnızca sınırlar biliniyor: değer -0,006 ile 0,006 dB arasında (0 ± 0,006) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,006/√3 = 0,00346 dB.
5. **dR_D**: 0 dB ve 30 dB okumalarında çözünürlük, iki dikdörtgenin birleşimi. Değer -0,01 ile 0,01 dB arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,01/√6 = 0,00408 dB.
6. **dM**: Uyumsuzluk (Harris ve Warner, E.9.5). En iyi değer 0 dB, standart belirsizlik u = 0,016996 dB (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **dA_L**: Sinyal kaçağı. Yalnızca sınırlar biliniyor: değer -0,001 ile 0,001 dB arasında (0 ± 0,001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,001/√3 = 0,000577 dB.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 30,05 dB
- standart belirsizlik u = 0,020218 dB
- genişletilmiş belirsizlik U = 0,040437 dB

Kaynak: E5 formülü ve K.3.6 verilerinden yeniden hesaplanmıştır (araştırma günlüğü F57); basılı bir sonuç değildir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="m3003-k3-attenuator-30db"></a>

### 10 GHz'de 30 dB koaksiyel zayıflatıcı (UKAS M3003 K.3)

`m3003-k3-attenuator-30db`

#### Bu proje ne hakkında?

Koaksiyel kademeli zayıflatıcının IF yer değiştirme ile kalibrasyonu; uyumsuzluk terimi baskındır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **A_X** (dB): 10 GHz'de 30 dB kademenin zayıflaması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
A_X = A_obs + dA_IF + dD_IF + dL_M + dR_D + dM + dA_L
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **A_obs**: Gösterilen zayıflama A_b − A_a, dört ölçüm. 4 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 30,0500 dB; okumalar s = 0,0183 dB standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,0183/√4 = 0,00913 dB, serbestlik derecesi ν = n − 1 = 3.
2. **dA_IF**: Referans IF zayıflatıcının kalibrasyonu (U = 0,005 dB, k = 2). En iyi değer 0 dB. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,005 dB, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,005/2 = 0,00250 dB olur (normal dağılım).
3. **dD_IF**: IF zayıflatıcının sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,002 ile 0,002 dB arasında (0 ± 0,002) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,002/√3 = 0,00115 dB.
4. **dL_M**: Karıştırıcı doğrusal olmama (10 dB başına ±0,002 dB). Yalnızca sınırlar biliniyor: değer -0,006 ile 0,006 dB arasında (0 ± 0,006) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,006/√3 = 0,00346 dB.
5. **dR_D**: 0 dB ve 30 dB okumalarında çözünürlük, iki dikdörtgenin birleşimi. Değer -0,01 ile 0,01 dB arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,01/√6 = 0,00408 dB.
6. **dM**: Uyumsuzluk (Harris ve Warner, E.9.5). En iyi değer 0 dB, standart belirsizlik u = 0,022 dB (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **dA_L**: Sinyal kaçağı. Yalnızca sınırlar biliniyor: değer -0,001 ile 0,001 dB arasında (0 ± 0,001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,001/√3 = 0,000577 dB.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 30,05 dB
- standart belirsizlik u = 0,0245 dB
- genişletilmiş belirsizlik U = 0,0491 dB

Kaynak: UKAS M3003 6. baskı K.3.9–K.3.10: u_c = 0,0245 dB (yuvarlanmamış 0,024 57), U = 0,0491 dB; 30,050 dB ± 0,049 dB.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir mastar blok (UKAS M3003) (Orta)

Bir 10 mm mastar blok. Bir türev sıcaklık etkilerini modelin kendisine yazar; bu, bir bütçe tablosu ile bir ölçüm modeli arasındaki farkı gösterir.

<a id="m3003-k6-gauge-block-10mm-temperature-model"></a>

### Modelinin sıcaklık terimleriyle 10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6, EA-4/02 S4)

`m3003-k6-gauge-block-10mm-temperature-model`

#### Bu proje ne hakkında?

−L(α·Δt + Δα·ΔT) teriminin modelin belirttiği gibi değerlendirildiği K.6 bütçesi; EA-4/02 S4'ün 50 mm'lik blok için yaptığı gibi (bulgu F58).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **L_X** (mm): Bilinmeyen mastar bloğun uzunluğu. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
L_X = L_SdL + dL_D + dD_C + dC - L*(alpha*dt + d_alpha*dT) + L_V + dL_r
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 11 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **L_SdL**: L_S + ΔL: standardın sertifikalı uzunluğu artı ölçülen fark (yalnızca toplam verilmiştir, K.6.9); sertifika U = 30 nm (k = 2). En iyi değer 9,99994 mm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,00003 mm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,00003/2 = 1,50 × 10⁻⁵ mm olur (normal dağılım).
2. **dL_D**: Standardın sürüklenmesi (±15 nm, üçgen). Değer -0,000015 ile 0,000015 mm arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,000015/√6 = 6,12 × 10⁻⁶ mm.
3. **dD_C**: Karşılaştırıcı ayırma gücü ve doğrusallığı (±0,05 µm). Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 mm arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ mm.
4. **dC**: Elastik sıkışma farkı (±0,005 µm). Yalnızca sınırlar biliniyor: değer -0,000005 ile 0,000005 mm arasında (0 ± 0,000005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000005/√3 = 2,89 × 10⁻⁶ mm.
5. **L**: Nominal uzunluk. Tam bilinen bir değer, 10 mm; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
6. **alpha**: Ortalama genleşme katsayısı. Tam bilinen bir değer, 0,0000115 1/°C; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
7. **dt**: İki blok arasındaki sıcaklık farkı. Yalnızca sınırlar biliniyor: değer -0,08 ile 0,08 °C arasında (0 ± 0,08) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,08/√3 = 0,0462 °C.
8. **d_alpha**: Genleşme katsayıları farkı. Değer -0,000002 ile 0,000002 1/°C arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,000002/√6 = 8,16 × 10⁻⁷ 1/°C.
9. **dT**: Ortalama sıcaklığın 20 °C'den sapması. Yalnızca sınırlar biliniyor: değer -0,2 ile 0,2 °C arasında (0 ± 0,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,2/√3 = 0,115 °C.
10. **L_V**: Bilinmeyen bloğun uzunluk değişimi (±42 nm, üçgen). Değer -0,000042 ile 0,000042 mm arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,000042/√6 = 1,71 × 10⁻⁵ mm.
11. **dL_r**: Tekrarlanabilirlik (11 önceki ölçümden s = 16 nm, tek okuma). 1 okumanın ortalaması, 0 mm. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,000016 mm (ν = 10) kullanılır; böylece u = s_p/√n = 1,60 × 10⁻⁵ mm.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 9,99994 mm
- standart belirsizlik u = 4,1028 × 10⁻⁵ mm
- genişletilmiş belirsizlik U = 8,2055 × 10⁻⁵ mm

Kaynak: K.6 verilerinden K.6.1 modeliyle yeniden hesaplanmıştır (araştırma günlüğü F58); basılı bir sonuç değildir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="m3003-k6-gauge-block-10mm"></a>

### 10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6)

`m3003-k6-gauge-block-10mm`

#### Bu proje ne hakkında?

Standart mastar blokla karşılaştırma; iki sıcaklık etkisinin doğrusal toplanıp tek üçgen terim yapıldığı basılı bütçe.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **L_X** (mm): Bilinmeyen mastar bloğun uzunluğu. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
L_X = L_SdL + dL_D + dD_C + dC + dT_SX + L_V + dL_r
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **L_SdL**: L_S + ΔL: standardın sertifikalı uzunluğu artı ölçülen fark (yalnızca toplam verilmiştir, K.6.9); sertifika U = 30 nm (k = 2). En iyi değer 9,99994 mm. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,00003 mm, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,00003/2 = 1,50 × 10⁻⁵ mm olur (normal dağılım).
2. **dL_D**: Standardın sürüklenmesi (±15 nm, üçgen). Değer -0,000015 ile 0,000015 mm arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,000015/√6 = 6,12 × 10⁻⁶ mm.
3. **dD_C**: Karşılaştırıcı ayırma gücü ve doğrusallığı (±0,05 µm). Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 mm arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ mm.
4. **dC**: Elastik sıkışma farkı (±0,005 µm). Yalnızca sınırlar biliniyor: değer -0,000005 ile 0,000005 mm arasında (0 ± 0,000005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000005/√3 = 2,89 × 10⁻⁶ mm.
5. **dT_SX**: Sıcaklık etkileri, doğrusal toplanmış (±5,6 nm, üçgen). Değer -0,0000056 ile 0,0000056 mm arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,0000056/√6 = 2,29 × 10⁻⁶ mm.
6. **L_V**: Bilinmeyen bloğun uzunluk değişimi (±42 nm, üçgen). Değer -0,000042 ile 0,000042 mm arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,000042/√6 = 1,71 × 10⁻⁵ mm.
7. **dL_r**: Tekrarlanabilirlik (11 önceki ölçümden s = 16 nm, tek okuma). 1 okumanın ortalaması, 0 mm. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,000016 mm (ν = 10) kullanılır; böylece u = s_p/√n = 1,60 × 10⁻⁵ mm.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 9,99994 mm
- standart belirsizlik u = 0,0000407 mm
- genişletilmiş belirsizlik U = 0,0000815 mm

Kaynak: UKAS M3003 6. baskı K.6.10–K.6.11: u_c = 40,7 nm, U = 81,5 nm; 9,999 940 mm ± 0,081 µm.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## 1000 °C'de bir ısıl çift (Orta)

N tipi bir ısıl çift: çok sayıda küçük katkıyla fırın sıcaklığı ve emk.

<a id="m3003-k7-thermocouple-emf"></a>

### 1000 °C'de N tipi termoçift: tam bütçesiyle EMK (UKAS M3003 K.7)

`m3003-k7-thermocouple-emf`

#### Bu proje ne hakkında?

K.7'nin iki bütçesi tek modelde: fırın sıcaklığı bilinmeyen termoçiftin EMK'sına 1/c_X = 38,5 µV/°C duyarlılığıyla girer.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **V_X** (µV): N tipi termoçiftin 1000 °C'deki EMK'sı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
V_X = V_iX + (1000 - t_X)/c_X + dV_iX1 + dV_iX2 + dV_LX + dt_TH/c_X + dV_R2 - dt_0X/c_X0 + dV_r
```

Denklemi okunur tutmak için önce 1 ara büyüklük hesaplanır:

- `t_X = t_S + c_S*dV_iS1 + c_S*dV_iS2 + c_S*dV_R - c_S/c_S0*dt_0S + dt_D + dt_F + dt_r`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 20 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **t_S**: Referans termoçiftler, ikisinin ortalaması (sertifika U = 0,3 °C, k = 2). En iyi değer 1000,5 °C. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,3 °C, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,3/2 = 0,150 °C olur (normal dağılım).
2. **c_S**: R tipi termoçiftin 1000 °C'deki duyarlılığı. Tam bilinen bir değer, 0,077 °C/µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
3. **c_S0**: R tipi termoçiftin 0 °C'deki duyarlılığı. Tam bilinen bir değer, 0,189 °C/µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
4. **dV_iS1**: Mikrovoltmetre kalibrasyonu (U = 2,0 µV, k = 2). En iyi değer 0 µV. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 2 µV, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 2/2 = 1,00 µV olur (normal dağılım).
5. **dV_iS2**: Mikrovoltmetre çözünürlüğü (1 µV basamak). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 µV arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 µV.
6. **dV_R**: Anahtar kontaklarının parazit gerilimleri. Yalnızca sınırlar biliniyor: değer -2 ile 2 µV arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 µV.
7. **dt_0S**: Standartların referans bağlantı sıcaklığı. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 °C arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 °C.
8. **dt_D**: Referans termoçiftlerin sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,3 ile 0,3 °C arasında (0 ± 0,3) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,3/√3 = 0,173 °C.
9. **dt_F**: Fırının homojen olmaması. Yalnızca sınırlar biliniyor: değer -1 ile 1 °C arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 °C.
10. **dt_r**: Fırın sıcaklığının tekrarlanabilirliği (s = 0,10 °C, ν = 9). 1 okumanın ortalaması, 0 °C. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,1 °C (ν = 9) kullanılır; böylece u = s_p/√n = 0,100 °C.
11. **V_iX**: Bilinmeyen termoçift için ortalama gösterge. Tam bilinen bir değer, 36248 µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
12. **c_X**: N tipi termoçiftin 1000 °C'deki duyarlılığı. Tam bilinen bir değer, 0,026 °C/µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
13. **c_X0**: N tipi termoçiftin 0 °C'deki duyarlılığı. Tam bilinen bir değer, 0,038 °C/µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
14. **dV_iX1**: Mikrovoltmetre kalibrasyonu (U = 2,0 µV, k = 2). En iyi değer 0 µV. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 2 µV, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 2/2 = 1,00 µV olur (normal dağılım).
15. **dV_iX2**: Mikrovoltmetre çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 µV arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 µV.
16. **dV_LX**: Kompanzasyon kabloları. Yalnızca sınırlar biliniyor: değer -5 ile 5 µV arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 µV.
17. **dt_TH**: Bilinmeyen termoçiftin homojen olmaması. Yalnızca sınırlar biliniyor: değer -0,3 ile 0,3 °C arasında (0 ± 0,3) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,3/√3 = 0,173 °C.
18. **dV_R2**: Anahtar kontakları (bilinmeyenin okuması). Yalnızca sınırlar biliniyor: değer -2 ile 2 µV arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 µV.
19. **dt_0X**: Bilinmeyenin referans bağlantı sıcaklığı. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 °C arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 °C.
20. **dV_r**: Gerilimin tekrarlanabilirliği (s = 1,6 µV, ν = 9). 1 okumanın ortalaması, 0 µV. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 1,6 µV (ν = 9) kullanılır; böylece u = s_p/√n = 1,60 µV.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 36230 µV
- standart belirsizlik u = 25,9 µV
- genişletilmiş belirsizlik U = 51,9 µV

Kaynak: UKAS M3003 6. baskı K.7.20–K.7.21: u_c = 25,9 µV, U = 51,9 µV; 36 230 µV ± 52 µV (tahmin basılı 10 µV çözünürlükte).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="m3003-k7-thermocouple-furnace"></a>

### 1000 °C'de termoçift kalibrasyonunda fırın sıcaklığı (UKAS M3003 K.7, birinci bütçe)

`m3003-k7-thermocouple-furnace`

#### Bu proje ne hakkında?

K.7'nin birinci kısmı: iki referans termoçiftten sıcak bağlantılardaki sıcaklık.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **t_X** (°C): Termoçiftlerin bulunduğu yerde fırın sıcaklığı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
t_X = t_S + c_S*dV_iS1 + c_S*dV_iS2 + c_S*dV_R - c_S/c_S0*dt_0S + dt_D + dt_F + dt_r
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 10 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **t_S**: Referans termoçiftler, ikisinin ortalaması (sertifika U = 0,3 °C, k = 2). En iyi değer 1000,5 °C. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,3 °C, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,3/2 = 0,150 °C olur (normal dağılım).
2. **c_S**: R tipi termoçiftin 1000 °C'deki duyarlılığı. Tam bilinen bir değer, 0,077 °C/µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
3. **c_S0**: R tipi termoçiftin 0 °C'deki duyarlılığı. Tam bilinen bir değer, 0,189 °C/µV; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
4. **dV_iS1**: Mikrovoltmetre kalibrasyonu (U = 2,0 µV, k = 2). En iyi değer 0 µV. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 2 µV, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 2/2 = 1,00 µV olur (normal dağılım).
5. **dV_iS2**: Mikrovoltmetre çözünürlüğü (1 µV basamak). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 µV arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 µV.
6. **dV_R**: Anahtar kontaklarının parazit gerilimleri. Yalnızca sınırlar biliniyor: değer -2 ile 2 µV arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 µV.
7. **dt_0S**: Standartların referans bağlantı sıcaklığı. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 °C arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 °C.
8. **dt_D**: Referans termoçiftlerin sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,3 ile 0,3 °C arasında (0 ± 0,3) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,3/√3 = 0,173 °C.
9. **dt_F**: Fırının homojen olmaması. Yalnızca sınırlar biliniyor: değer -1 ile 1 °C arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 °C.
10. **dt_r**: Fırın sıcaklığının tekrarlanabilirliği (s = 0,10 °C, ν = 9). 1 okumanın ortalaması, 0 °C. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,1 °C (ν = 9) kullanılır; böylece u = s_p/√n = 0,100 °C.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1000,5 °C
- standart belirsizlik u = 0,641 °C

Kaynak: UKAS M3003 6. baskı K.7.19: u_c(t_X) = 0,641 °C.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Standart koşullarda debi (Orta)

Standart basınç ve sıcaklığa dönüştürülen bir hacimsel debi: çarpım ve oran modeli.

<a id="m3003-k9-flowrate-standard-conditions"></a>

### Standart basınç ve sıcaklığa indirgenmiş hacimsel debi (UKAS M3003 K.9)

`m3003-k9-flowrate-standard-conditions`

#### Bu proje ne hakkında?

1018 hPa ve 295,1 K'de 4–20 mA'lik bir debimetreden debi, 1013,25 hPa ve 293,15 K'de ifade edilir; üç kısmi bütçe tek modelde.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **Q_s** (L/s): Standart koşullarda debi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
Q_s = Q_m*(P_m/P_s)*(T_s/T_m) + dQ_s
```

Denklemi okunur tutmak için önce 3 ara büyüklük hesaplanır:

- `Q_m = A*i + B + dq_cal - dq_d - dq_T`
- `P_m = p_m - dp_m - dp_f - dp_d - dp_T`
- `T_m = t_m - dt_m - dt_f - dt_d - dt_T`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 19 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **A**: Kalibrasyon fonksiyonunun eğimi. Tam bilinen bir değer, 6,25 L s⁻¹/mA; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **B**: Kalibrasyon fonksiyonunun kesişimi. Tam bilinen bir değer, -24,95 L/s; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
3. **i**: Ölçülen akım (U = 0,021 mA, k = 2). En iyi değer 6 mA. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,021 mA, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,021/2 = 0,0105 mA olur (normal dağılım).
4. **dq_cal**: Debimetre kalibrasyonu (U = 0,26 L/s, k = 2). En iyi değer 0 L/s. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,26 L/s, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,26/2 = 0,130 L/s olur (normal dağılım).
5. **dq_d**: Kalibrasyon sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,23 ile 0,23 L/s arasında (0 ± 0,23) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,23/√3 = 0,133 L/s.
6. **dq_T**: Ortam etkileri. Yalnızca sınırlar biliniyor: değer -0,11 ile 0,11 L/s arasında (0 ± 0,11) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,11/√3 = 0,0635 L/s.
7. **p_m**: Basınç göstergesi (1 hPa çözünürlük). Yalnızca sınırlar biliniyor: değer 1017,5 ile 1018,5 hPa arasında (1018 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 hPa.
8. **dp_m**: Manometre kalibrasyonu (U = 1,8 hPa, k = 2). En iyi değer 0 hPa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 1,8 hPa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 1,8/2 = 0,900 hPa olur (normal dağılım).
9. **dp_f**: Düzeltilmemiş kalibrasyon hatası. Yalnızca sınırlar biliniyor: değer -0,056 ile 0,056 hPa arasında (0 ± 0,056) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,056/√3 = 0,0323 hPa.
10. **dp_d**: Manometre sürüklenmesi. Yalnızca sınırlar biliniyor: değer -1,25 ile 1,25 hPa arasında (0 ± 1,25) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,25/√3 = 0,722 hPa.
11. **dp_T**: Manometreye ortam etkileri. Yalnızca sınırlar biliniyor: değer -1 ile 1 hPa arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 hPa.
12. **t_m**: Düzeltilmiş sıcaklık göstergesi (0,1 K çözünürlük). Yalnızca sınırlar biliniyor: değer 295,05 ile 295,15 K arasında (295,1 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 K.
13. **dt_m**: Termometre kalibrasyonu (U = 0,055 K, k = 2). En iyi değer 0 K. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,055 K, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,055/2 = 0,0275 K olur (normal dağılım).
14. **dt_f**: Ortalama düzeltmeden sonra kalan hata. Yalnızca sınırlar biliniyor: değer -0,02 ile 0,02 K arasında (0 ± 0,02) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,02/√3 = 0,0115 K.
15. **dt_d**: Termometre sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,13 ile 0,13 K arasında (0 ± 0,13) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,13/√3 = 0,0751 K.
16. **dt_T**: Montaj etkileri. Yalnızca sınırlar biliniyor: değer -0,2 ile 0,2 K arasında (0 ± 0,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,2/√3 = 0,115 K.
17. **P_s**: Standart basınç. Tam bilinen bir değer, 1013,25 hPa; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
18. **T_s**: Standart sıcaklık. Tam bilinen bir değer, 293,15 K; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
19. **dQ_s**: Q_s'nin tekrarlanabilirliği (s = 0,18 L/s, ν = 24, tek okuma). 1 okumanın ortalaması, 0 L/s. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,18 L/s (ν = 24) kullanılır; böylece u = s_p/√n = 0,180 L/s.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 12,53 L/s
- standart belirsizlik u = 0,275 L/s
- genişletilmiş belirsizlik U = 0,55 L/s

Kaynak: UKAS M3003 6. baskı K.9.10–K.9.12: u_c = 0,275 L/s, ν_eff = 130, U = 0,55 L/s; (12,53 ± 0,55) L/s.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir PT100 sondası (Orta)

Önce sonda bir kuru blokta kalibre edilir; sonra bir gazın sıcaklığını ölçer. Kalibrasyonun belirsizliği ölçümün bir girdisi olur.

<a id="m3003-k11-gas-temperature"></a>

### Kalibre edilmiş PT100 probdan kaptaki gazın sıcaklığı (UKAS M3003 K.11, ikinci bütçe)

`m3003-k11-gas-temperature`

#### Bu proje ne hakkında?

L probu kabın dışına bağlanır; düzeltmesi ΔT_u = 0,000 55·I_u + 0,419 °C kalibrasyon fonksiyonundan gelir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **T_g** (°C): Gazın sıcaklığı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
T_g = I_u - (0.00055*I_u + 0.419 + dT_cal + dT_fit + dT_drift) - dT_ug - dT_h - dT_r
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_u**: L probunun göstergesi (0,1 °C çözünürlük). Yalnızca sınırlar biliniyor: değer 23,350 ile 23,45 °C arasında (23,4 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 °C.
2. **dT_cal**: L'nin kalibrasyonu (U = 0,084 49 °C, k = 2). En iyi değer 0 °C. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,08449 °C, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,08449/2 = 0,0422 °C olur (normal dağılım).
3. **dT_fit**: L'nin kalibrasyon verilerine doğrusal uydurma. Yalnızca sınırlar biliniyor: değer -0,012 ile 0,012 °C arasında (0 ± 0,012) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,012/√3 = 0,00693 °C.
4. **dT_drift**: L'nin sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,00467 ile 0,00467 °C arasında (0 ± 0,00467) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00467/√3 = 0,00270 °C.
5. **dT_ug**: Prob ile gaz arasındaki fark. Yalnızca sınırlar biliniyor: değer -0,1 ile 0,1 °C arasında (0 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 °C.
6. **dT_h**: L'nin kap üzerinde kendi kendine ısınması. Yalnızca sınırlar biliniyor: değer -0,004 ile 0,004 °C arasında (0 ± 0,004) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,004/√3 = 0,00231 °C.
7. **dT_r**: Kap sıcaklığının tekrarlanabilirliği (s = 0,076 °C, ν = 49, dördün ortalaması). 4 okumanın ortalaması, 0 °C. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,076 °C (ν = 49) kullanılır; böylece u = s_p/√n = 0,0380 °C.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 22,97 °C
- standart belirsizlik u = 0,08638 °C
- genişletilmiş belirsizlik U = 0,17276 °C

Kaynak: UKAS M3003 6. baskı K.11.17–K.11.18: u_c = 0,086 38 °C, U = 0,172 76 °C; 22,97 °C ± 0,17 °C.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="m3003-k11-probe-calibration"></a>

### Kuru blokta referans proba karşı PT100 prob kalibrasyonu (UKAS M3003 K.11, birinci bütçe)

`m3003-k11-probe-calibration`

#### Bu proje ne hakkında?

Laboratuvar probu L'nin 20,4 °C'de, kalibrasyon fonksiyonu ΔT_S = 0,000 01·I_S + 0,0108 °C olan referans prob S'ye karşı hatası.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dT_L** (°C): L probunun hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dT_L = I_L - (I_S - (0.00001*I_S + 0.0108 + dT_cal + dT_fit + dT_drift) - dT_SL) - d_rep
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: L probunun göstergesi (0,1 °C çözünürlük). Yalnızca sınırlar biliniyor: değer 20,350 ile 20,45 °C arasında (20,4 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 °C.
2. **I_S**: Referans prob S'nin göstergesi (0,01 °C çözünürlük; türetilmiş değer). Yalnızca sınırlar biliniyor: değer 19,974 ile 19,984 °C arasında (19,979 ± 0,005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,005/√3 = 0,00289 °C.
3. **dT_cal**: S'nin kalibrasyonu (U = 0,024 °C, k = 2). En iyi değer 0 °C. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,024 °C, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,024/2 = 0,0120 °C olur (normal dağılım).
4. **dT_fit**: S'nin kalibrasyon verilerine doğrusal uydurma. Yalnızca sınırlar biliniyor: değer -0,005 ile 0,005 °C arasında (0 ± 0,005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,005/√3 = 0,00289 °C.
5. **dT_drift**: S'nin sürüklenmesi. Yalnızca sınırlar biliniyor: değer -0,008 ile 0,008 °C arasında (0 ± 0,008) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,008/√3 = 0,00462 °C.
6. **dT_SL**: Blokta gradyanlar ve montaj. Yalnızca sınırlar biliniyor: değer -0,048 ile 0,048 °C arasında (0 ± 0,048) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,048/√3 = 0,0277 °C.
7. **d_rep**: Tekrarlanabilirlik (s = 0,001 23 °C, ν = 24). 1 okumanın ortalaması, 0 °C. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,00123 °C (ν = 24) kullanılır; böylece u = s_p/√n = 0,00123 °C.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- standart belirsizlik u = 0,04225 °C
- genişletilmiş belirsizlik U = 0,08449 °C

Kaynak: UKAS M3003 6. baskı K.11.9: u_c = 0,042 25 °C, U = 0,084 49 °C.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir laboratuvar terazisinin kalibrasyonu (Orta)

EURAMET cg-18'e göre 220 g'lık bir terazi: hava kaldırma kuvveti düzeltmesiyle ve düzeltmesiz birkaç yükteki hata ve günlük kullanımda bir tartım sonucunun belirsizliği.

<a id="cg18-h1-100g-option2"></a>

### Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle 100 g'da hata (EURAMET cg-18 H1, seçenek 2)

`cg18-h1-100g-option2`

#### Bu proje ne hakkında?

Kalibrasyon sırasında hava yoğunluğu bilinir (990 hPa, %50 bağıl nem, 21 °C: 1,173 kg/m³); bu nedenle kaldırma kuvveti düzeltmesi uygulanır; deney ağırlıkları oda sıcaklığından 2 K farklıdır (konveksiyon).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_c + dm_D - m_N*(rho_a - rho_0)*(1/rho - 1/rho_c) + dm_conv)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 13 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 100,0006 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
3. **dI_rep**: Tekrarlanabilirlik (beş okumadan s, ν = 4, tek okuma). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,00011402 g (ν = 4) kullanılır; böylece u = s_p/√n = 0,000114 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
5. **dI_ecc**: Eksantriklik. Yalnızca sınırlar biliniyor: değer -0,0001 ile 0,0001 g arasında (0 ± 0,0001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0001/√3 = 5,77 × 10⁻⁵ g.
6. **m_c**: Ağırlıkların konvansiyonel kütlesi (sertifika U/k). En iyi değer 99,9999 g, standart belirsizlik u = 0,000025 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **dm_D**: Ağırlıkların sürüklenmesi (D = 1,25·U). Yalnızca sınırlar biliniyor: değer -0,0000625 ile 0,0000625 g arasında (0 ± 0,0000625) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0000625/√3 = 3,61 × 10⁻⁵ g.
8. **m_N**: Yükün nominal değeri. Tam bilinen bir değer, 100 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
9. **rho_a**: Kalibrasyon sırasında hava yoğunluğu (CIPM formülü; A3-2 ile ΔT = 5 K). En iyi değer 1,173 kg/m³, standart belirsizlik u = 0,014 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **rho**: Ağırlıkların yoğunluğu. En iyi değer 7950 kg/m³, standart belirsizlik u = 70 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
11. **rho_0**: Referans hava yoğunluğu. Tam bilinen bir değer, 1,2 kg/m³; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
12. **rho_c**: Ağırlıkların referans yoğunluğu. Tam bilinen bir değer, 8000 kg/m³; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
13. **dm_conv**: Konveksiyon, oda sıcaklığından 2 K farklı ağırlıklar (Tablo F2.1). Yalnızca sınırlar biliniyor: değer -7,9674 × 10⁻⁵ ile 7,9674 × 10⁻⁵ g arasında (0 ± 7,9674 × 10⁻⁵) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,9674 × 10⁻⁵/√3 = 4,60 × 10⁻⁵ g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- standart belirsizlik u = 0,000149 g
- kapsam faktörü k = 2,25
- genişletilmiş belirsizlik U = 0,00033 g

Kaynak: EURAMET cg-18 v4.0 H1.3/A, seçenek 2, 100 g: u(E) = 0,000149 g, ν_eff = 11, k = 2,25, U = 0,00033 g, Kılavuz k'yı GUM Tablo G.2'den alır (ν = 62'de 2,05, ν = 50 satırıdır); tam t değeri 2,04'tür.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h1-220g-max-option1-delta-t"></a>

### Laboratuvar terazisi 220 g / 0,1 mg: ΔT = 5 K için kaldırma kuvveti terimiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 1, 7.1.2-5e formülü)

`cg18-h1-220g-max-option1-delta-t`

#### Bu proje ne hakkında?

Seçenek 1 gibi, ancak kaldırma kuvveti belirsizliği en kötü durum yerine odanın tahmini 5 K sıcaklık değişiminden çıkar.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_c + dm_D + dm_B)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 220,0014 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
3. **dI_rep**: Tekrarlanabilirlik (beş okumadan s, ν = 4, tek okuma). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,00011402 g (ν = 4) kullanılır; böylece u = s_p/√n = 0,000114 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
5. **dI_ecc**: Eksantriklik. Yalnızca sınırlar biliniyor: değer -0,00022000 ile 0,00022000 g arasında (0 ± 0,00022000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00022000/√3 = 0,000127 g.
6. **m_c**: Ağırlıkların konvansiyonel kütlesi (sertifika U/k). En iyi değer 220,0001 g, standart belirsizlik u = 0,000062 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **dm_D**: Ağırlıkların sürüklenmesi (D = 1,25·U). Yalnızca sınırlar biliniyor: değer -0,000155 ile 0,000155 g arasında (0 ± 0,000155) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000155/√3 = 8,95 × 10⁻⁵ g.
8. **dm_B**: Hava kaldırma kuvveti, düzeltilmemiş, ΔT = 5 K için (7.1.2-5e). En iyi değer 0 g, standart belirsizlik u = 0,00044566 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,0013 g
- standart belirsizlik u = 0,000491 g
- kapsam faktörü k = 2
- genişletilmiş belirsizlik U = 0,00098 g

Kaynak: EURAMET cg-18 v4.0 H1.3/A, 7.1.2-5e ile seçenek 1, 220 g: u(E) = 0,000 491 g, ν_eff = 1377, k = 2,00, U = 0,000 98 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h1-220g-max-option1"></a>

### Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesi olmadan 220 g'da hata (EURAMET cg-18 H1, seçenek 1)

`cg18-h1-220g-max-option1`

#### Bu proje ne hakkında?

Kalibrasyondan önce ayarlanmamış ve hava yoğunluğu bilinmiyor: kaldırma kuvveti terimi 7.1.2-5d formülünün temkinli sınırıdır ve baskındır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_c + dm_D + dm_B)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 220,0014 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
3. **dI_rep**: Tekrarlanabilirlik (beş okumadan s, ν = 4, tek okuma). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,00011402 g (ν = 4) kullanılır; böylece u = s_p/√n = 0,000114 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
5. **dI_ecc**: Eksantriklik. Yalnızca sınırlar biliniyor: değer -0,00022000 ile 0,00022000 g arasında (0 ± 0,00022000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00022000/√3 = 0,000127 g.
6. **m_c**: Ağırlıkların konvansiyonel kütlesi (sertifika U/k). En iyi değer 220,0001 g, standart belirsizlik u = 0,000062 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **dm_D**: Ağırlıkların sürüklenmesi (D = 1,25·U). Yalnızca sınırlar biliniyor: değer -0,000155 ile 0,000155 g arasında (0 ± 0,000155) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000155/√3 = 8,95 × 10⁻⁵ g.
8. **dm_B**: Hava kaldırma kuvveti, düzeltilmemiş (7.1.2-5d). Yalnızca sınırlar biliniyor: değer -0,0033950 ile 0,0033950 g arasında (0 ± 0,0033950) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0033950/√3 = 0,00196 g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,0013 g
- standart belirsizlik u = 0,001971 g
- kapsam faktörü k = 2
- genişletilmiş belirsizlik U = 0,00394 g

Kaynak: EURAMET cg-18 v4.0 H1.3/A, seçenek 1 (7.1.2-5d), 220 g: u(E) = 0,001 971 g, ν_eff = 357 098, k = 2,00, U = 0,003 94 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h1-220g-max-option2"></a>

### Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 2)

`cg18-h1-220g-max-option2`

#### Bu proje ne hakkında?

Kalibrasyon sırasında hava yoğunluğu bilinir (990 hPa, %50 bağıl nem, 21 °C: 1,173 kg/m³); bu nedenle kaldırma kuvveti düzeltmesi uygulanır; deney ağırlıkları oda sıcaklığından 2 K farklıdır (konveksiyon).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_c + dm_D - m_N*(rho_a - rho_0)*(1/rho - 1/rho_c) + dm_conv)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 13 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 220,0014 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
3. **dI_rep**: Tekrarlanabilirlik (beş okumadan s, ν = 4, tek okuma). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,00011402 g (ν = 4) kullanılır; böylece u = s_p/√n = 0,000114 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
5. **dI_ecc**: Eksantriklik. Yalnızca sınırlar biliniyor: değer -0,00022000 ile 0,00022000 g arasında (0 ± 0,00022000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00022000/√3 = 0,000127 g.
6. **m_c**: Ağırlıkların konvansiyonel kütlesi (sertifika U/k). En iyi değer 220,0001 g, standart belirsizlik u = 0,000062 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **dm_D**: Ağırlıkların sürüklenmesi (D = 1,25·U). Yalnızca sınırlar biliniyor: değer -0,000155 ile 0,000155 g arasında (0 ± 0,000155) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,000155/√3 = 8,95 × 10⁻⁵ g.
8. **m_N**: Yükün nominal değeri. Tam bilinen bir değer, 220 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
9. **rho_a**: Kalibrasyon sırasında hava yoğunluğu (CIPM formülü; A3-2 ile ΔT = 5 K). En iyi değer 1,173 kg/m³, standart belirsizlik u = 0,014 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **rho**: Ağırlıkların yoğunluğu. En iyi değer 7950 kg/m³, standart belirsizlik u = 70 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
11. **rho_0**: Referans hava yoğunluğu. Tam bilinen bir değer, 1,2 kg/m³; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
12. **rho_c**: Ağırlıkların referans yoğunluğu. Tam bilinen bir değer, 8000 kg/m³; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
13. **dm_conv**: Konveksiyon, oda sıcaklığından 2 K farklı ağırlıklar (Tablo F2.1). Yalnızca sınırlar biliniyor: değer -0,00015935 ile 0,00015935 g arasında (0 ± 0,00015935) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00015935/√3 = 9,20 × 10⁻⁵ g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- standart belirsizlik u = 0,000226 g
- kapsam faktörü k = 2,05
- genişletilmiş belirsizlik U = 0,00046 g

Kaynak: EURAMET cg-18 v4.0 H1.3/A, seçenek 2, 220 g: u(E) = 0,000226 g, ν_eff = 62, k = 2,05, U = 0,00046 g, Kılavuz k'yı GUM Tablo G.2'den alır (ν = 62'de 2,05, ν = 50 satırıdır); tam t değeri 2,04'tür.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h1-220g-zero"></a>

### Laboratuvar terazisi 220 g / 0,1 mg: sıfır yükte hata (EURAMET cg-18 H1)

`cg18-h1-220g-zero`

#### Bu proje ne hakkında?

Sıfır yükte yalnızca sıfırda yuvarlama ve tekrarlanabilirlik katkı verir; ν = 4 ile tekrarlanabilirlik kapsam faktörünü belirler.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 0 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,00005 ile 0,00005 g arasında (0 ± 0,00005) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00005/√3 = 2,89 × 10⁻⁵ g.
3. **dI_rep**: Tekrarlanabilirlik (beş okumadan s, ν = 4, tek okuma). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 0,00011402 g (ν = 4) kullanılır; böylece u = s_p/√n = 0,000114 g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 g
- standart belirsizlik u = 0,000118 g
- kapsam faktörü k = 2,87
- genişletilmiş belirsizlik U = 0,00034 g

Kaynak: EURAMET cg-18 v4.0 H1.3/A, seçenek 1, yük 0: u(E) = 0,000 118 g, ν_eff = 4, k = 2,87, U = 0,000 34 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h1-a-weighing-result"></a>

### Terazi 220 g / 0,1 mg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H1.4/A)

`cg18-h1-a-weighing-result`

#### Bu proje ne hakkında?

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Electronic balance, Max 220 g, d = 0.1 mg, 5 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.

Sonra günlük kullanımda ortaya çıkan etkiler de (sıcaklık, eksantriklik, dara) katılarak **kullanımdaki bir tartım sonucunun belirsizliği** ve **minimum tartım**, yani bağıl belirsizliği hâlâ kabul edilebilir olan en küçük yük hesaplanır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 Ek H1.1/A to H1.4/A: tartım sonucunun belirsizliğinin ve minimum tartımın basılı değerleri.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h1-b-weighing-result"></a>

### Terazi 220 g / 0,1 mg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H1.4/B)

`cg18-h1-b-weighing-result`

#### Bu proje ne hakkında?

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Electronic balance, Max 220 g, d = 0.1 mg, 5 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.

Sonra günlük kullanımda ortaya çıkan etkiler de (sıcaklık, eksantriklik, dara) katılarak **kullanımdaki bir tartım sonucunun belirsizliği** ve **minimum tartım**, yani bağıl belirsizliği hâlâ kabul edilebilir olan en küçük yük hesaplanır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 Ek H1.1/B to H1.4/B: tartım sonucunun belirsizliğinin ve minimum tartımın basılı değerleri; F75/F76'dan etkilenen değerler yeniden hesaplanmış olanlardır (notlara bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Çok aralıklı bir terazi (Orta)

Üç tartım aralıklı 60 kg'lık bir terazi. Her aralığın kendi çözünürlüğü vardır; bu da bütçeyi yük boyunca değiştirir.

<a id="cg18-h2-60kg-10kg-option1"></a>

### Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesi olmadan 10 kg'da hata (EURAMET cg-18 H2, seçenek 1)

`cg18-h2-60kg-10kg-option1`

#### Bu proje ne hakkında?

2 g'lık aralıktaki ilk yük noktası; hava yoğunluğu bilinmiyor ve kalibrasyondan önce ayarlanmamış: temkinli kaldırma kuvveti terimi 7.1.2-5d.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D + dm_B)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 10000 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -1 ile 1 g arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 g.
3. **dI_rep**: Tekrarlanabilirlik (s = 1,095 g, ν = 4). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 1,0954 g (ν = 4) kullanılır; böylece u = s_p/√n = 1,10 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -1 ile 1 g arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 g.
5. **dI_ecc**: Eksantriklik (7.1.1-10). Yalnızca sınırlar biliniyor: değer -1,25 ile 1,25 g arasında (0 ± 1,25) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,25/√3 = 0,722 g.
6. **m_N**: Deney yükünün nominal değeri. Tam bilinen bir değer, 10000 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
7. **dm_c**: Nominal değerindeki ağırlıklar, F2 mpe = 0,16 g (7.1.2-3). Yalnızca sınırlar biliniyor: değer -0,16 ile 0,16 g arasında (0 ± 0,16) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,16/√3 = 0,0924 g.
8. **dm_D**: Ağırlıkların sürüklenmesi, D = mpe/2 (7.1.2-11). Yalnızca sınırlar biliniyor: değer -0,08 ile 0,08 g arasında (0 ± 0,08) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,08/√3 = 0,0462 g.
9. **dm_B**: Hava kaldırma kuvveti, kalibrasyondan önce ayarlanmamış (7.1.2-5d). Yalnızca sınırlar biliniyor: değer -0,19000 ile 0,19000 g arasında (0 ± 0,19000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,19000/√3 = 0,110 g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 g
- standart belirsizlik u = 1,552 g
- kapsam faktörü k = 2,17
- genişletilmiş belirsizlik U = 3,369 g

Kaynak: EURAMET cg-18 v4.0 H2.3/A, seçenek 1 (7.1.2-5d), 10 kg: u(E) = 1,552 g, ν_eff = 16, k = 2,17, U = 3,369 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h2-60kg-max-adjusted"></a>

### Çok aralıklı terazi 60 kg, kalibrasyondan önce ayarlanmış: Max'ta hata (EURAMET cg-18 H2, ikinci durum)

`cg18-h2-60kg-max-adjusted`

#### Bu proje ne hakkında?

İkinci durum: terazi kalibrasyondan hemen önce dahili ağırlıklarıyla ayarlanır; tekrarlanabilirlik daha iyidir (s = 2,236 g).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D + dm_B)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 60000 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -1 ile 1 g arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 g.
3. **dI_rep**: Tekrarlanabilirlik (s = 2,236 g, ν = 4). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 2,2361 g (ν = 4) kullanılır; böylece u = s_p/√n = 2,24 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -5 ile 5 g arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 g.
5. **dI_ecc**: Eksantriklik (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,5 ile 7,5 g arasında (0 ± 7,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,5/√3 = 4,33 g.
6. **m_N**: Deney yükünün nominal değeri. Tam bilinen bir değer, 60000 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
7. **dm_c**: Nominal değerindeki ağırlıklar, F2 mpe = 0,90 g (7.1.2-3). Yalnızca sınırlar biliniyor: değer -0,90000 ile 0,90000 g arasında (0 ± 0,90000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,90000/√3 = 0,520 g.
8. **dm_D**: Ağırlıkların sürüklenmesi, D = mpe/2 (7.1.2-11). Yalnızca sınırlar biliniyor: değer -0,45000 ile 0,45000 g arasında (0 ± 0,45000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,45000/√3 = 0,260 g.
9. **dm_B**: Hava kaldırma kuvveti, kalibrasyondan hemen önce ayarlanmış (7.1.2-5c). Yalnızca sınırlar biliniyor: değer -0,22500 ile 0,22500 g arasında (0 ± 0,22500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,22500/√3 = 0,130 g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 g
- standart belirsizlik u = 5,739 g
- kapsam faktörü k = 2,025
- genişletilmiş belirsizlik U = 11,601 g

Kaynak: EURAMET cg-18 v4.0 H2.3/B, seçenek 1 (7.1.2-5c), 60 kg: u(E) = 5,739 g, ν_eff = 172, k = 2,025, U = 11,601 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h2-60kg-max-option1-delta-t"></a>

### Çok aralıklı terazi 60 kg: ΔT = 10 K için kaldırma kuvveti terimiyle Max'ta hata (EURAMET cg-18 H2, seçenek 1, 7.1.2-5e formülü)

`cg18-h2-60kg-max-option1-delta-t`

#### Bu proje ne hakkında?

Seçenek 1 gibi; kaldırma kuvveti belirsizliği odanın beklenen sıcaklık değişiminden (10 K, pencereli oda).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D + dm_B)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 59990 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -1 ile 1 g arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 g.
3. **dI_rep**: Tekrarlanabilirlik (s = 2,739 g, ν = 4). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 2,7386 g (ν = 4) kullanılır; böylece u = s_p/√n = 2,74 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -5 ile 5 g arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 g.
5. **dI_ecc**: Eksantriklik (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,5 ile 7,5 g arasında (0 ± 7,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,5/√3 = 4,33 g.
6. **m_N**: Deney yükünün nominal değeri. Tam bilinen bir değer, 60000 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
7. **dm_c**: Nominal değerindeki ağırlıklar, F2 mpe = 0,90 g (7.1.2-3). Yalnızca sınırlar biliniyor: değer -0,90000 ile 0,90000 g arasında (0 ± 0,90000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,90000/√3 = 0,520 g.
8. **dm_D**: Ağırlıkların sürüklenmesi, D = mpe/2 (7.1.2-11). Yalnızca sınırlar biliniyor: değer -0,45000 ile 0,45000 g arasında (0 ± 0,45000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,45000/√3 = 0,260 g.
9. **dm_B**: ΔT = 10 K için hava kaldırma kuvveti (7.1.2-5e). En iyi değer 0 g, standart belirsizlik u = 0,26933 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = -10 g
- standart belirsizlik u = 5,948 g
- kapsam faktörü k = 2,05
- genişletilmiş belirsizlik U = 12,193 g

Kaynak: EURAMET cg-18 v4.0 H2.3/A, 7.1.2-5e ile seçenek 1 (ΔT = 10 K), 60 kg: u(E) = 5,948 g, ν_eff = 88, k = 2,05, U = 12,193 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h2-60kg-max-option1"></a>

### Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesi olmadan Max'ta hata (EURAMET cg-18 H2, seçenek 1)

`cg18-h2-60kg-max-option1`

#### Bu proje ne hakkında?

10 g'lık aralıkta 60 kg'da hata; sertifikadaki en büyük U.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D + dm_B)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 59990 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -1 ile 1 g arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 g.
3. **dI_rep**: Tekrarlanabilirlik (s = 2,739 g, ν = 4). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 2,7386 g (ν = 4) kullanılır; böylece u = s_p/√n = 2,74 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -5 ile 5 g arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 g.
5. **dI_ecc**: Eksantriklik (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,5 ile 7,5 g arasında (0 ± 7,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,5/√3 = 4,33 g.
6. **m_N**: Deney yükünün nominal değeri. Tam bilinen bir değer, 60000 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
7. **dm_c**: Nominal değerindeki ağırlıklar, F2 mpe = 0,90 g (7.1.2-3). Yalnızca sınırlar biliniyor: değer -0,90000 ile 0,90000 g arasında (0 ± 0,90000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,90000/√3 = 0,520 g.
8. **dm_D**: Ağırlıkların sürüklenmesi, D = mpe/2 (7.1.2-11). Yalnızca sınırlar biliniyor: değer -0,45000 ile 0,45000 g arasında (0 ± 0,45000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,45000/√3 = 0,260 g.
9. **dm_B**: Hava kaldırma kuvveti, kalibrasyondan önce ayarlanmamış (7.1.2-5d). Yalnızca sınırlar biliniyor: değer -1,125 ile 1,125 g arasında (0 ± 1,125) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,125/√3 = 0,650 g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = -10 g
- standart belirsizlik u = 5,978 g
- kapsam faktörü k = 2,05
- genişletilmiş belirsizlik U = 12,254 g

Kaynak: EURAMET cg-18 v4.0 H2.3/A, seçenek 1 (7.1.2-5d), 60 kg: E = −10 g, u(E) = 5,978 g, ν_eff = 90, k = 2,05, U = 12,254 g; toleranslar deney yükünün mpe'sini ve Tablo G.2'nin k'sını kapsar (notlar).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h2-60kg-max-option2"></a>

### Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesiyle Max'ta hata (EURAMET cg-18 H2, seçenek 2)

`cg18-h2-60kg-max-option2`

#### Bu proje ne hakkında?

Kalibrasyon sırasındaki hava yoğunluğu CIPM formülünden hesaplanır ve kaldırma kuvveti düzeltmesi uygulanır; ayar sırasındaki hava yoğunluğu bilinmez.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc - (m_N + dm_c + dm_D - m_N*(rho_a - rho_0)*(1/rho - 1/rho_c))
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 12 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 59990 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -1 ile 1 g arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 g.
3. **dI_rep**: Tekrarlanabilirlik (s = 2,739 g, ν = 4). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 2,7386 g (ν = 4) kullanılır; böylece u = s_p/√n = 2,74 g.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -5 ile 5 g arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 g.
5. **dI_ecc**: Eksantriklik (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,5 ile 7,5 g arasında (0 ± 7,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,5/√3 = 4,33 g.
6. **m_N**: Deney yükünün nominal değeri. Tam bilinen bir değer, 60000 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
7. **dm_c**: Nominal değerindeki ağırlıklar, F2 mpe = 0,90 g (7.1.2-3). Yalnızca sınırlar biliniyor: değer -0,90000 ile 0,90000 g arasında (0 ± 0,90000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,90000/√3 = 0,520 g.
8. **dm_D**: Ağırlıkların sürüklenmesi, D = mpe/2 (7.1.2-11). Yalnızca sınırlar biliniyor: değer -0,45000 ile 0,45000 g arasında (0 ± 0,45000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,45000/√3 = 0,260 g.
9. **rho_a**: Kalibrasyon sırasında hava yoğunluğu (CIPM formülü; A3-2 ile ΔT = 10 K). En iyi değer 1,173 kg/m³, standart belirsizlik u = 0,018 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **rho**: Ağırlıkların yoğunluğu. En iyi değer 7950 kg/m³, standart belirsizlik u = 70 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
11. **rho_0**: Referans hava yoğunluğu. Tam bilinen bir değer, 1,2 kg/m³; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
12. **rho_c**: Ağırlıkların referans yoğunluğu. Tam bilinen bir değer, 8000 kg/m³; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = -10 g
- standart belirsizlik u = 5,941 g
- kapsam faktörü k = 2,05
- genişletilmiş belirsizlik U = 12,18 g

Kaynak: EURAMET cg-18 v4.0 H2.3/A, seçenek 2, 60 kg: δm_B = 2,138·10⁻⁸·m_N, u(E) = 5,941 g, ν_eff = 88, k = 2,05, U = 12,180 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h2-60kg-zero"></a>

### Çok aralıklı terazi 60 kg: sıfır yükte hata (EURAMET cg-18 H2)

`cg18-h2-60kg-zero`

#### Bu proje ne hakkında?

Bulunduğu hâliyle, kalibrasyondan önce ayarlanmadan kalibre edilen 60 kg'lık çok aralıklı terazi (2 g / 5 g / 10 g).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (g): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 0 g; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -1 ile 1 g arasında (0 ± 1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1/√3 = 0,577 g.
3. **dI_rep**: Tekrarlanabilirlik (s = 1,095 g, ν = 4). 1 okumanın ortalaması, 0 g. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 1,0954 g (ν = 4) kullanılır; böylece u = s_p/√n = 1,10 g.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 g
- standart belirsizlik u = 1,238 g
- kapsam faktörü k = 2,52
- genişletilmiş belirsizlik U = 3,12 g

Kaynak: EURAMET cg-18 v4.0 H2.3/A, yük 0: u(E) = 1,238 g, ν_eff = 6, k = 2,52, U = 3,120 g.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h2-a-weighing-result"></a>

### Çok aralıklı tartı aleti 12/30/60 kg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H2.4/A)

`cg18-h2-a-weighing-result`

#### Bu proje ne hakkında?

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Multi-interval scale, Max 12/30/60 kg, d = 2/5/10 g, 5 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.

Sonra günlük kullanımda ortaya çıkan etkiler de (sıcaklık, eksantriklik, dara) katılarak **kullanımdaki bir tartım sonucunun belirsizliği** ve **minimum tartım**, yani bağıl belirsizliği hâlâ kabul edilebilir olan en küçük yük hesaplanır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 Ek H2.1/A to H2.4/A: tartım sonucunun belirsizliğinin ve minimum tartımın basılı değerleri; F75/F76'dan etkilenen değerler yeniden hesaplanmış olanlardır (notlara bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h2-b-weighing-result"></a>

### Çok aralıklı tartı aleti 12/30/60 kg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H2.4/B)

`cg18-h2-b-weighing-result`

#### Bu proje ne hakkında?

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Multi-interval scale, Max 12/30/60 kg, d = 2/5/10 g, 5 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.

Sonra günlük kullanımda ortaya çıkan etkiler de (sıcaklık, eksantriklik, dara) katılarak **kullanımdaki bir tartım sonucunun belirsizliği** ve **minimum tartım**, yani bağıl belirsizliği hâlâ kabul edilebilir olan en küçük yük hesaplanır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 Ek H2.1/B to H2.4/B: tartım sonucunun belirsizliğinin ve minimum tartımın basılı değerleri.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir kantar (Orta)

Büyük terazilerin gerçek kalibrasyonlarındaki gibi standart ağırlıklar ve ikame yükleriyle denenen 30 t'lik bir kantar.

<a id="cg18-h3-30t-10t-weights"></a>

### Köprü kantar 30 t: 10 t standart ağırlıkta hata (EURAMET cg-18 H3)

`cg18-h3-30t-10t-weights`

#### Bu proje ne hakkında?

Yük alıcısında 10 × 1000 kg standart ağırlık; bu yük m_ref, ikame yükleri için de referanstır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (kg): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc + dI_time - (10 * (1000 + dm_c + dm_D + dm_B))
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 10010 kg; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
3. **dI_rep**: Tekrarlanabilirlik (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
5. **dI_ecc**: Eksantriklik (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,1977 ile 7,1977 kg arasında (0 ± 7,1977) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,1977/√3 = 4,16 kg.
6. **dI_time**: Sürünme ve histerezis (7.4.4-7). Yalnızca sınırlar biliniyor: değer -1,3333 ile 1,3333 kg arasında (0 ± 1,3333) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,3333/√3 = 0,770 kg.
7. **dm_c**: Bir adet 1000 kg ağırlığın hatası (M1, nominal değerinde). Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 kg arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 kg.
8. **dm_D**: Bir adet 1000 kg ağırlığın sürüklenmesi, D = mpe. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 kg arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 kg.
9. **dm_B**: 1000 kg başına hava kaldırma kuvveti (7.1.2-5d). Yalnızca sınırlar biliniyor: değer -0,027500 ile 0,027500 kg arasında (0 ± 0,027500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,027500/√3 = 0,0159 kg.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 10 kg
- standart belirsizlik u = 7,98 kg
- kapsam faktörü k = 2,32
- genişletilmiş belirsizlik U = 19 kg

Kaynak: EURAMET cg-18 v4.0 H3.3/A, 10 000 kg standart ağırlık: E = 10 kg, u(E) = 7,98 kg, ν_eff = 9, k = 2,32, U = 19 kg.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h3-30t-15t-substitution"></a>

### Köprü kantar 30 t: ikame yüküyle 15 t'da hata (EURAMET cg-18 H3)

`cg18-h3-30t-15t-substitution`

#### Bu proje ne hakkında?

İkame yükü L_ikame1 (10 t standart ağırlıkla aynı göstergeyi verecek şekilde ayarlanmış römork ve konteynerler) üzerinde 5 t standart ağırlık.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (kg): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc + dI_time - (5 * (1000 + dm_c + dm_D + dm_B) + 10 * (1000 + dm_c + dm_D + dm_B) + (dI_dig0_s1 + dI_rep_s1 + dI_digL_s1 + dI_ecc_s1 + dI_time_s1) - (dI_dig0_r1 + dI_rep_r1 + dI_digL_r1 + dI_ecc_r1 + dI_time_r1) + dm_Bsub)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 20 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 15015 kg; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
3. **dI_rep**: Tekrarlanabilirlik (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
5. **dI_ecc**: Eksantriklik (7.1.1-10). Yalnızca sınırlar biliniyor: değer -10,797 ile 10,797 kg arasında (0 ± 10,797) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 10,797/√3 = 6,23 kg.
6. **dI_time**: Sürünme ve histerezis (7.4.4-7). Yalnızca sınırlar biliniyor: değer -2 ile 2 kg arasında (0 ± 2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2/√3 = 1,15 kg.
7. **dm_c**: Bir adet 1000 kg ağırlığın hatası (M1, nominal değerinde). Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 kg arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 kg.
8. **dm_D**: Bir adet 1000 kg ağırlığın sürüklenmesi, D = mpe. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 kg arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 kg.
9. **dm_B**: 1000 kg başına hava kaldırma kuvveti (7.1.2-5d). Yalnızca sınırlar biliniyor: değer -0,027500 ile 0,027500 kg arasında (0 ± 0,027500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,027500/√3 = 0,0159 kg.
10. **dI_dig0_s1**: Sıfırda yuvarlama, I(L_ikame1). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
11. **dI_rep_s1**: Tekrarlanabilirlik, I(L_ikame1) (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
12. **dI_digL_s1**: Yükte yuvarlama, I(L_ikame1). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
13. **dI_ecc_s1**: Eksantriklik, I(L_ikame1) (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,1977 ile 7,1977 kg arasında (0 ± 7,1977) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,1977/√3 = 4,16 kg.
14. **dI_time_s1**: Sürünme ve histerezis, I(L_ikame1) (7.4.4-7). Yalnızca sınırlar biliniyor: değer -1,3333 ile 1,3333 kg arasında (0 ± 1,3333) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,3333/√3 = 0,770 kg.
15. **dI_dig0_r1**: Sıfırda yuvarlama, 10 t'da I(m_ref). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
16. **dI_rep_r1**: Tekrarlanabilirlik, 10 t'da I(m_ref) (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
17. **dI_digL_r1**: Yükte yuvarlama, 10 t'da I(m_ref). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
18. **dI_ecc_r1**: Eksantriklik, 10 t'da I(m_ref) (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,1977 ile 7,1977 kg arasında (0 ± 7,1977) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,1977/√3 = 4,16 kg.
19. **dI_time_r1**: Sürünme ve histerezis, 10 t'da I(m_ref) (7.4.4-7). Yalnızca sınırlar biliniyor: değer -1,3333 ile 1,3333 kg arasında (0 ± 1,3333) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,3333/√3 = 0,770 kg.
20. **dm_Bsub**: İkame yüklerinin hava kaldırma kuvveti (7.1.2-5d, ρ ≥ 4400 kg/m³). Yalnızca sınırlar biliniyor: değer -0,27500 ile 0,27500 kg arasında (0 ± 0,27500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,27500/√3 = 0,159 kg.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 15 kg
- standart belirsizlik u = 14,6 kg

Kaynak: EURAMET cg-18 v4.0 H3.3/A, 15 000 kg = ½m_ref + L_ikame1: E = 15 kg, u(E) = 14,60 kg (ν_eff = 109, k = 2,02, U = 29 kg; kapsam faktörü karşılaştırılmamıştır, notlara bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h3-30t-max-substitution"></a>

### Köprü kantar 30 t: iki ikame yüküyle Max'ta hata (EURAMET cg-18 H3)

`cg18-h3-30t-max-substitution`

#### Bu proje ne hakkında?

İkame yükleri L_ikame1 + L_ikame2 üzerinde 10 t standart ağırlık; sertifikadaki en büyük U.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (kg): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep + dI_digL + dI_ecc + dI_time - (10 * (1000 + dm_c + dm_D + dm_B) + 20 * (1000 + dm_c + dm_D + dm_B) + I_2 + (dI_dig0_s1 + dI_rep_s1 + dI_digL_s1 + dI_ecc_s1 + dI_time_s1) - (dI_dig0_r1 + dI_rep_r1 + dI_digL_r1 + dI_ecc_r1 + dI_time_r1) + (dI_dig0_s2 + dI_rep_s2 + dI_digL_s2 + dI_ecc_s2 + dI_time_s2) - (dI_dig0_r2 + dI_rep_r2 + dI_digL_r2 + dI_ecc_r2 + dI_time_r2) + dm_Bsub)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 31 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 30040 kg; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
3. **dI_rep**: Tekrarlanabilirlik (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
4. **dI_digL**: Yükte yuvarlama. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
5. **dI_ecc**: Eksantriklik (7.1.1-10). Yalnızca sınırlar biliniyor: değer -21,600 ile 21,600 kg arasında (0 ± 21,600) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 21,600/√3 = 12,5 kg.
6. **dI_time**: Sürünme ve histerezis (7.4.4-7). Yalnızca sınırlar biliniyor: değer -4,0013 ile 4,0013 kg arasında (0 ± 4,0013) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 4,0013/√3 = 2,31 kg.
7. **dm_c**: Bir adet 1000 kg ağırlığın hatası (M1, nominal değerinde). Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 kg arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 kg.
8. **dm_D**: Bir adet 1000 kg ağırlığın sürüklenmesi, D = mpe. Yalnızca sınırlar biliniyor: değer -0,05 ile 0,05 kg arasında (0 ± 0,05) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,05/√3 = 0,0289 kg.
9. **dm_B**: 1000 kg başına hava kaldırma kuvveti (7.1.2-5d). Yalnızca sınırlar biliniyor: değer -0,027500 ile 0,027500 kg arasında (0 ± 0,027500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,027500/√3 = 0,0159 kg.
10. **dI_dig0_s1**: Sıfırda yuvarlama, I(L_ikame1). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
11. **dI_rep_s1**: Tekrarlanabilirlik, I(L_ikame1) (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
12. **dI_digL_s1**: Yükte yuvarlama, I(L_ikame1). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
13. **dI_ecc_s1**: Eksantriklik, I(L_ikame1) (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,1977 ile 7,1977 kg arasında (0 ± 7,1977) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,1977/√3 = 4,16 kg.
14. **dI_time_s1**: Sürünme ve histerezis, I(L_ikame1) (7.4.4-7). Yalnızca sınırlar biliniyor: değer -1,3333 ile 1,3333 kg arasında (0 ± 1,3333) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,3333/√3 = 0,770 kg.
15. **dI_dig0_r1**: Sıfırda yuvarlama, 10 t'da I(m_ref). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
16. **dI_rep_r1**: Tekrarlanabilirlik, 10 t'da I(m_ref) (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
17. **dI_digL_r1**: Yükte yuvarlama, 10 t'da I(m_ref). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
18. **dI_ecc_r1**: Eksantriklik, 10 t'da I(m_ref) (7.1.1-10). Yalnızca sınırlar biliniyor: değer -7,1977 ile 7,1977 kg arasında (0 ± 7,1977) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 7,1977/√3 = 4,16 kg.
19. **dI_time_r1**: Sürünme ve histerezis, 10 t'da I(m_ref) (7.4.4-7). Yalnızca sınırlar biliniyor: değer -1,3333 ile 1,3333 kg arasında (0 ± 1,3333) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,3333/√3 = 0,770 kg.
20. **I_2**: I₂ = I(L_ikame1 + L_ikame2) − I(m_ref + L_ikame1) = 20 028 − 20 018 kg. Tam bilinen bir değer, 10 kg; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
21. **dI_dig0_s2**: Sıfırda yuvarlama, I(L_ikame1 + L_ikame2). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
22. **dI_rep_s2**: Tekrarlanabilirlik, I(L_ikame1 + L_ikame2) (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
23. **dI_digL_s2**: Yükte yuvarlama, I(L_ikame1 + L_ikame2). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
24. **dI_ecc_s2**: Eksantriklik, I(L_ikame1 + L_ikame2) (7.1.1-10). Yalnızca sınırlar biliniyor: değer -14,395 ile 14,395 kg arasında (0 ± 14,395) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 14,395/√3 = 8,31 kg.
25. **dI_time_s2**: Sürünme ve histerezis, I(L_ikame1 + L_ikame2) (7.4.4-7). Yalnızca sınırlar biliniyor: değer -2,6667 ile 2,6667 kg arasında (0 ± 2,6667) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2,6667/√3 = 1,54 kg.
26. **dI_dig0_r2**: Sıfırda yuvarlama, 20 t'da I(m_ref + L_ikame1). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
27. **dI_rep_r2**: Tekrarlanabilirlik, 20 t'da I(m_ref + L_ikame1) (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.
28. **dI_digL_r2**: Yükte yuvarlama, 20 t'da I(m_ref + L_ikame1). Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
29. **dI_ecc_r2**: Eksantriklik, 20 t'da I(m_ref + L_ikame1) (7.1.1-10). Yalnızca sınırlar biliniyor: değer -14,395 ile 14,395 kg arasında (0 ± 14,395) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 14,395/√3 = 8,31 kg.
30. **dI_time_r2**: Sürünme ve histerezis, 20 t'da I(m_ref + L_ikame1) (7.4.4-7). Yalnızca sınırlar biliniyor: değer -2,6667 ile 2,6667 kg arasında (0 ± 2,6667) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2,6667/√3 = 1,54 kg.
31. **dm_Bsub**: İkame yüklerinin hava kaldırma kuvveti (7.1.2-5d, ρ ≥ 4400 kg/m³). Yalnızca sınırlar biliniyor: değer -0,550275 ile 0,550275 kg arasında (0 ± 0,550275) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,550275/√3 = 0,318 kg.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 30 kg
- standart belirsizlik u = 23,85 kg
- kapsam faktörü k = 2
- genişletilmiş belirsizlik U = 48 kg

Kaynak: EURAMET cg-18 v4.0 H3.3/A, 30 010 kg = m_ref + L_ikame1 + L_ikame2: E = 30 kg, u(E) = 23,85 kg, ν_eff = 783, k = 2,00, U = 48 kg.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h3-30t-zero"></a>

### Köprü kantar 30 t: sıfır yükte hata (EURAMET cg-18 H3)

`cg18-h3-30t-zero`

#### Bu proje ne hakkında?

Açık havada, bulunduğu hâliyle kalibre edilen 30 000 kg / 10 kg'lık karayolu taşıtı köprü kantarı.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (kg): Gösterge hatası. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = I_L + dI_dig0 + dI_rep
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **I_L**: Yükteki gösterge. Tam bilinen bir değer, 0 kg; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **dI_dig0**: Sıfırda yuvarlama. Yalnızca sınırlar biliniyor: değer -0,5 ile 0,5 kg arasında (0 ± 0,5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,5/√3 = 0,289 kg.
3. **dI_rep**: Tekrarlanabilirlik (s = 6,743 kg, ν = 5). 1 okumanın ortalaması, 0 kg. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = 6,7429 kg (ν = 5) kullanılır; böylece u = s_p/√n = 6,74 kg.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,5 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0 kg
- standart belirsizlik u = 6,75 kg
- kapsam faktörü k = 2,65
- genişletilmiş belirsizlik U = 18 kg

Kaynak: EURAMET cg-18 v4.0 H3.3/A, yük 0: u(E) = 6,75 kg, ν_eff = 5, k = 2,65, U = 18 kg.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h3-a-weighing-result"></a>

### Köprü kantar 30 t / 10 kg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H3.4/A)

`cg18-h3-a-weighing-result`

#### Bu proje ne hakkında?

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Electronic weighbridge, Max 30 000 kg, d = 10 kg (calibrated with d_T = 1 kg and substitution loads), 7 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.

Sonra günlük kullanımda ortaya çıkan etkiler de (sıcaklık, eksantriklik, dara) katılarak **kullanımdaki bir tartım sonucunun belirsizliği** ve **minimum tartım**, yani bağıl belirsizliği hâlâ kabul edilebilir olan en küçük yük hesaplanır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 Ek H3.1/A to H3.4/A: tartım sonucunun belirsizliğinin ve minimum tartımın basılı değerleri; F75/F76'dan etkilenen değerler yeniden hesaplanmış olanlardır (notlara bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h3-b-weighing-result"></a>

### Köprü kantar 30 t / 10 kg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H3.4/B)

`cg18-h3-b-weighing-result`

#### Bu proje ne hakkında?

Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Electronic weighbridge, Max 30 000 kg, d = 10 kg (calibrated with d_T = 1 kg and substitution loads), 7 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.

Sonra günlük kullanımda ortaya çıkan etkiler de (sıcaklık, eksantriklik, dara) katılarak **kullanımdaki bir tartım sonucunun belirsizliği** ve **minimum tartım**, yani bağıl belirsizliği hâlâ kabul edilebilir olan en küçük yük hesaplanır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 Ek H3.1/B to H3.4/B: tartım sonucunun belirsizliğinin ve minimum tartımın basılı değerleri; F75/F76'dan etkilenen değerler yeniden hesaplanmış olanlardır (notlara bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Düşen bilyeli viskozimetre (Orta)

Bir bilyenin düşme süresinden viskozite. Yoğunluk ve süre oranları içeren bir model.

<a id="nist-tn1900-e3-falling-ball-viscometer"></a>

### Düşen bilyalı viskozimetre (NIST TN 1900 E3)

`nist-tn1900-e3-falling-ball-viscometer`

#### Bu proje ne hakkında?

TN 1900 E3: bir NaOH çözeltisinin dinamik viskozitesi kalibrasyon sıvısıyla karşılaştırılarak; düşme sürelerinin büyük bağıl belirsizlikleri çıktıyı çarpık yapar.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **mu_M** (mPa s): NaOH çözeltisinin dinamik viskozitesi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
mu_M = mu_C * (rho_B - rho_M) / (rho_B - rho_C) * t_M / t_C
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **mu_C**: Kalibrasyon sıvısının viskozitesi. En iyi değer 4,63 mPa s, standart belirsizlik u = 0,0463 mPa s (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **rho_B**: Bilyanın yoğunluğu. En iyi değer 2217 kg/m³, standart belirsizlik u = 0,5 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **rho_C**: Kalibrasyon sıvısının yoğunluğu. En iyi değer 810 kg/m³, standart belirsizlik u = 0,5 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **rho_M**: NaOH çözeltisinin yoğunluğu. En iyi değer 1180 kg/m³, standart belirsizlik u = 0,5 kg/m³ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **t_C**: Bilyanın kalibrasyon sıvısındaki düşme süresi. En iyi değer 36,6 s, standart belirsizlik u = 5,49 s (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **t_M**: Bilyanın NaOH çözeltisindeki düşme süresi. En iyi değer 61 s, standart belirsizlik u = 6,1 s (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 5,69 mPa s
- standart belirsizlik u = 1,0269 mPa s

Kaynak: NIST TN 1900 E3: ölçüm denkleminden tahmin 5,69 mPa s; Monte Carlo 5,82 mPa s, u = 1,11 mPa s, %95 aralığı [4,05; 8,39] mPa s. GUM u = 1,027 mPa s: birinci mertebe yayılım yasasının analitik değeri (nota bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Pitot tüpüyle hava hızı (Orta)

Hız, bir basınç farkından karekök üzerinden bulunur. Bu örnekte denklem ile basılı sonuç arasında yayımlanmış bir tutarsızlık da var (notlara bakın).

<a id="nist-tn1900-e4-pitot-tube"></a>

### Pitot tüpüyle hava hızı (NIST TN 1900 E4)

`nist-tn1900-e4-pitot-tube`

#### Bu proje ne hakkında?

TN 1900 E4 (Kline ve McClintock, 1953'ten): fark basıncı, hava basıncı ve sıcaklıktan hava hızı.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **v** (m/s): Hava hızı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
v = sqrt(2 * 1000 * Delta * R_s * T / (1000 * p))
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **Delta**: Toplam ve statik basınç farkı. En iyi değer 1,993 kPa, standart belirsizlik u = 0,0125 kPa (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **R_s**: Kuru havanın özgül gaz sabiti. Tam bilinen bir değer, 287,058 J/(kg K); belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
3. **T**: Hava sıcaklığı. En iyi değer 292,8 K, standart belirsizlik u = 0,055 K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **p**: Hava basıncı. En iyi değer 101,4 kPa, standart belirsizlik u = 1,05 kPa (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 57,474 m/s
- standart belirsizlik u = 0,34790 m/s

Kaynak: NIST TN 1900 E4: v = 40,64 m/s, u(v) = 0,25 m/s (bağıl %0,61), Monte Carlo %95 aralığı [40,17; 41,13] m/s; eksik 2 çarpanı için her biri √2 ile çarpılmıştır (nota bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## CO₂'nin molekül ağırlığı (Orta)

Aralık olarak verilen karbon ve oksijen atom ağırlıklarından.

<a id="nist-tn1900-e8-molecular-weight-co2"></a>

### Karbondioksitin molekül ağırlığı (NIST TN 1900 E8)

`nist-tn1900-e8-molecular-weight-co2`

#### Bu proje ne hakkında?

TN 1900 E8: IUPAC aralık atom ağırlıklarından M_r(CO₂) = A_r(C) + 2A_r(O); çıktı dağılımı yamuktur ve GUM aralığı gereğinden geniştir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **M_r**: CO₂'nin bağıl molekül kütlesi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
M_r = A_C + 2 * A_O
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **A_C**: Karbonun standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 12,0096 ile 12,0116 arasında (12,0106 ± 0,0010000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0010000/√3 = 0,000577.
2. **A_O**: Oksijenin standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 15,99903 ile 15,99977 arasında (15,9994 ± 0,00037000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00037000/√3 = 0,000214.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 44,0094
- standart belirsizlik u = 0,0007

Kaynak: NIST TN 1900 E8: M_r(CO₂) = 44,0094, u = 0,0007, tam en kısa %95 aralığı [44,0080; 44,0108]; yuvarlanmamış uç noktalar 44,008045 ve 44,010755 yamuk dağılımdan çıkar (nota bakınız); tolerans: son basamağın yarım birimi artı tohumlar arası Monte Carlo saçılımının 5 katı (D110).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Monte Carlo ile bir kadmiyum standardı (Orta)

Eurachem kadmiyum örneğinin NIST sürümü, Monte Carlo yöntemiyle değerlendirilmiş.

<a id="nist-tn1900-e9-cadmium-standard"></a>

### Kadmiyum kalibrasyon standardı, Monte Carlo (NIST TN 1900 E9)

`nist-tn1900-e9-cadmium-standard`

#### Bu proje ne hakkında?

TN 1900 E9: QUAM A1 kadmiyum standardı, c = 1000 m P / V, dikdörtgen saflık ve üçgen hacim dağılımıyla Monte Carlo yöntemiyle yeniden değerlendirilir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_Cd** (mg/L): Standardın kadmiyum derişimi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_Cd = 1000 * m * P / V
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m**: Kadmiyum kütlesi. En iyi değer 100,28 mg, standart belirsizlik u = 0,05 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **P**: Metalin saflığı (kütle kesri). Yalnızca sınırlar biliniyor: değer 0,9998 ile 1 arasında (0,9999 ± 0,00010000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00010000/√3 = 5,77 × 10⁻⁵.
3. **V**: Çözeltinin hacmi. Değer 99,828536 ile 100,171464 mL arasında ve merkez 100'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,171464/√6 = 0,0700 mL.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1002,7 mg/L
- standart belirsizlik u = 0,9 mg/L

Kaynak: NIST TN 1900 E9: c = 1002,7 mg/L, u ≈ 0,9 mg/L, Monte Carlo %95 aralığı [1001,0; 1004,4] mg/L.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## U biçimli girdiler (Orta)

Uyumsuzluk terimleri U biçimli olan bir mikrodalga adımlı zayıflatıcı; GUM ve Monte Carlo karşılaştırılır.

<a id="nist-tn1900-e11-step-attenuator"></a>

### Mikrodalga kademeli zayıflatıcı, U biçimli girdiler (NIST TN 1900 E11)

`nist-tn1900-e11-step-attenuator`

#### Bu proje ne hakkında?

TN 1900 E11: EA-4/02 S7 Monte Carlo yöntemiyle yeniden değerlendirilir; baskın arksinüs uyumsuzluk terimi doğrusal bir toplamın dağılımını iki tepeli yapar.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **L_X** (dB): 30 dB kademesinin zayıflatması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
L_X = L_S + dL_S + dL_D + dL_M + dL_K + dL_ib - dL_ia + dL_0b - dL_0a
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **L_S**: Dört zayıflatma farkının ortalaması. En iyi değer 30,0402 dB, standart belirsizlik u = 0,00525 dB. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
2. **dL_S**: Referans zayıflatıcının kalibrasyonundan düzeltme. Yalnızca sınırlar biliniyor: değer -0,0013301 ile 0,0073301 dB arasında (0,003 ± 0,0043301) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0043301/√3 = 0,00250 dB.
3. **dL_D**: Referans zayıflatıcının kayması. Değer -0,0019799 ile 0,0019799 dB arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,0019799/√2 = 0,00140 dB.
4. **dL_M**: Uyumsuzluk kaybı düzeltmesi. Değer -0,0282843 ile 0,0282843 dB arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,0282843/√2 = 0,0200 dB.
5. **dL_K**: Sızıntı düzeltmesi. Değer -0,003 ile 0,003 dB arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,003/√2 = 0,00212 dB.
6. **dL_ia**: Referans dedektörün 0 dB'deki çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,0005196 ile 0,0005196 dB arasında (0 ± 0,0005196) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0005196/√3 = 0,000300 dB.
7. **dL_ib**: Referans dedektörün 30 dB'deki çözünürlüğü. Yalnızca sınırlar biliniyor: değer -0,0005196 ile 0,0005196 dB arasında (0 ± 0,0005196) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0005196/√3 = 0,000300 dB.
8. **dL_0a**: Sıfır dedektörünün 0 dB'deki çözünürlüğü. En iyi değer 0 dB, standart belirsizlik u = 0,002 dB (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **dL_0b**: Sıfır dedektörünün 30 dB'deki çözünürlüğü. En iyi değer 0 dB, standart belirsizlik u = 0,002 dB (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: NIST TN 1900 E11: L_X = 30,043 dB, u = 0,0224 dB, %95 aralığı [30,006; 30,081] dB (Monte Carlo, M = 10⁷).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Normal ya da t girdiler, %95 ya da %99 (Orta)

Isıl genleşme katsayısı dört kez değerlendirilir: normal ya da Student t girdilerle ve %95 ya da %99 kapsam için. Her iki seçimin aralığı nasıl değiştirdiğini görün.

<a id="nist-tn1900-e13-thermal-expansion-gaussian-95"></a>

### Isıl genleşme katsayısı, normal girdiler, %95 (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-gaussian-95`

#### Bu proje ne hakkında?

TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; normal girdiler, %95 kapsam.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **alpha** (10⁻⁶ K⁻¹): Bakır çubuğun doğrusal ısıl genleşme katsayısı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
alpha = 1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **L_0**: T₀'daki uzunluk. En iyi değer 1,4999 m, standart belirsizlik u = 0,0001 m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **L_1**: T₁'deki uzunluk. En iyi değer 1,5021 m, standart belirsizlik u = 0,0002 m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **T_0**: Başlangıç sıcaklığı. En iyi değer 288,15 K, standart belirsizlik u = 0,02 K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **T_1**: Son sıcaklık. En iyi değer 373,1 K, standart belirsizlik u = 0,05 K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 4 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 4 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 17,27 10⁻⁶ K⁻¹
- standart belirsizlik u = 1,8 10⁻⁶ K⁻¹

Kaynak: NIST TN 1900 E13 (normal girdiler): α = 1,73 × 10⁻⁵ K⁻¹, u = 0,18 × 10⁻⁵ K⁻¹, %95 aralığı (1,38; 2,07) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Toleranslar: basılı son basamağın yarım birimi artı Monte Carlo saçılımı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nist-tn1900-e13-thermal-expansion-gaussian-99"></a>

### Isıl genleşme katsayısı, normal girdiler, %99 (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-gaussian-99`

#### Bu proje ne hakkında?

TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; normal girdiler, %99 kapsam.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **alpha** (10⁻⁶ K⁻¹): Bakır çubuğun doğrusal ısıl genleşme katsayısı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
alpha = 1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **L_0**: T₀'daki uzunluk. En iyi değer 1,4999 m, standart belirsizlik u = 0,0001 m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **L_1**: T₁'deki uzunluk. En iyi değer 1,5021 m, standart belirsizlik u = 0,0002 m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **T_0**: Başlangıç sıcaklığı. En iyi değer 288,15 K, standart belirsizlik u = 0,02 K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **T_1**: Son sıcaklık. En iyi değer 373,1 K, standart belirsizlik u = 0,05 K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %99,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 4 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 4 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 17,27 10⁻⁶ K⁻¹
- standart belirsizlik u = 1,8 10⁻⁶ K⁻¹

Kaynak: NIST TN 1900 E13 (normal girdiler): α = 1,73 × 10⁻⁵ K⁻¹, u = 0,18 × 10⁻⁵ K⁻¹, %99 aralığı (1,27; 2,18) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Toleranslar: basılı son basamağın yarım birimi artı Monte Carlo saçılımı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nist-tn1900-e13-thermal-expansion-student-95"></a>

### Isıl genleşme katsayısı, Student t₃ girdiler, %95 (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-student-95`

#### Bu proje ne hakkında?

TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; Student t₃ girdiler, %95 kapsam.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **alpha** (10⁻⁶ K⁻¹): Bakır çubuğun doğrusal ısıl genleşme katsayısı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
alpha = 1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **L_0**: T₀'daki uzunluk. En iyi değer 1,4999 m, standart belirsizlik u = 5,77 × 10⁻⁵ m. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
2. **L_1**: T₁'deki uzunluk. En iyi değer 1,5021 m, standart belirsizlik u = 0,000115 m. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
3. **T_0**: Başlangıç sıcaklığı. En iyi değer 288,15 K, standart belirsizlik u = 0,0115 K. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
4. **T_1**: Son sıcaklık. En iyi değer 373,1 K, standart belirsizlik u = 0,0289 K. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 4 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 4 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: NIST TN 1900 E13 (Student girdiler): α = 1,73 × 10⁻⁵ K⁻¹, u = 0,18 × 10⁻⁵ K⁻¹, %95 aralığı (1,40; 2,05) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Toleranslar: basılı son basamağın yarım birimi artı Monte Carlo saçılımı; t₃ girdilerde örneklem standart sapması yavaş yakınsar (dördüncü moment sonsuzdur).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nist-tn1900-e13-thermal-expansion-student-99"></a>

### Isıl genleşme katsayısı, Student t₃ girdiler, %99 (NIST TN 1900 E13)

`nist-tn1900-e13-thermal-expansion-student-99`

#### Bu proje ne hakkında?

TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; Student t₃ girdiler, %99 kapsam.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **alpha** (10⁻⁶ K⁻¹): Bakır çubuğun doğrusal ısıl genleşme katsayısı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
alpha = 1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **L_0**: T₀'daki uzunluk. En iyi değer 1,4999 m, standart belirsizlik u = 5,77 × 10⁻⁵ m. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
2. **L_1**: T₁'deki uzunluk. En iyi değer 1,5021 m, standart belirsizlik u = 0,000115 m. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
3. **T_0**: Başlangıç sıcaklığı. En iyi değer 288,15 K, standart belirsizlik u = 0,0115 K. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
4. **T_1**: Son sıcaklık. En iyi değer 373,1 K, standart belirsizlik u = 0,0289 K. Az sayıda veriye dayandığı için (ν = 3 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 4 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 4 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: NIST TN 1900 E13 (Student girdiler): α = 1,73 × 10⁻⁵ K⁻¹, u = 0,18 × 10⁻⁵ K⁻¹, %99 aralığı (1,15; 2,30) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Toleranslar: basılı son basamağın yarım birimi artı Monte Carlo saçılımı; t₃ girdilerde örneklem standart sapması yavaş yakınsar (dördüncü moment sonsuzdur).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Kütle kalibrasyonu: birinci mertebe yetmediğinde (Orta)

JCGM 101'in kütle kalibrasyonu birinci mertebe GUM, ikinci mertebe terimler ve Monte Carlo ile. Küçük bir doğrusal olmama birinci mertebe belirsizliği fazla küçük yapar.

<a id="jcgm101-9.3-mass-calibration-higher-order"></a>

### Kütle kalibrasyonu (ikinci mertebe terimli GUM)

`jcgm101-9.3-mass-calibration-higher-order`

#### Bu proje ne hakkında?

JCGM 101 9.3: aynı model, GUM 5.1.2'nin yüksek mertebe terimleriyle değerlendirilmiştir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dm** (mg): W'nin konvansiyonel kütlesinin 100 g'dan sapması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dm = (m_Rc + dm_Rc) * (1 + (rho_a - 1.2) * (1/rho_W - 1/rho_R)) - 100000
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m_Rc**: Referans ağırlığın konvansiyonel kütlesi. En iyi değer 100000 mg, standart belirsizlik u = 0,05 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **dm_Rc**: Dengelemek için eklenen küçük ağırlığın kütlesi. En iyi değer 1,234 mg, standart belirsizlik u = 0,02 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **rho_a**: Hava yoğunluğu. Yalnızca sınırlar biliniyor: değer 1,1000 ile 1,3 kg/m³ arasında (1,2 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 kg/m³.
4. **rho_W**: W ağırlığının yoğunluğu. Yalnızca sınırlar biliniyor: değer 7000 ile 9000 kg/m³ arasında (8000 ± 1000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1000/√3 = 577 kg/m³.
5. **rho_R**: R referans ağırlığının yoğunluğu. Yalnızca sınırlar biliniyor: değer 7950 ile 8050 kg/m³ arasında (8000 ± 50) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 50/√3 = 28,9 kg/m³.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,234 mg
- standart belirsizlik u = 0,075 mg

Kaynak: JCGM 101:2008 Tablo 6 (GUF2) ve bağımsız MCM referansı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.3-mass-calibration"></a>

### Kütle kalibrasyonu (birinci mertebe GUM)

`jcgm101-9.3-mass-calibration`

#### Bu proje ne hakkında?

JCGM 101 9.3: 100 g'lık bir ağırlığın referans ağırlıkla karşılaştırılarak kalibrasyonu; birinci mertebe GUM ile MCM karşılaştırması.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dm** (mg): W'nin konvansiyonel kütlesinin 100 g'dan sapması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dm = (m_Rc + dm_Rc) * (1 + (rho_a - 1.2) * (1/rho_W - 1/rho_R)) - 100000
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m_Rc**: Referans ağırlığın konvansiyonel kütlesi. En iyi değer 100000 mg, standart belirsizlik u = 0,05 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **dm_Rc**: Dengelemek için eklenen küçük ağırlığın kütlesi. En iyi değer 1,234 mg, standart belirsizlik u = 0,02 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **rho_a**: Hava yoğunluğu. Yalnızca sınırlar biliniyor: değer 1,1000 ile 1,3 kg/m³ arasında (1,2 ± 0,1) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,1/√3 = 0,0577 kg/m³.
4. **rho_W**: W ağırlığının yoğunluğu. Yalnızca sınırlar biliniyor: değer 7000 ile 9000 kg/m³ arasında (8000 ± 1000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1000/√3 = 577 kg/m³.
5. **rho_R**: R referans ağırlığının yoğunluğu. Yalnızca sınırlar biliniyor: değer 7950 ile 8050 kg/m³ arasında (8000 ± 50) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 50/√3 = 28,9 kg/m³.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,234 mg
- standart belirsizlik u = 0,0539 mg

Kaynak: GUM: JCGM 101:2008 Tablo 6. MCM: bağımsız 2 × 10⁸ denemeli referans (validation/reference_values.json); toleranslar M = 2 × 10⁶'da motor saçılımının ≈ 5 katı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Aynı girdilerden iki çıktı (Orta)

Ortak girdilerden hesaplanan iki sonuç korelasyonludur. JCGM 102 iki ayrı aralık yerine kovaryans matrislerini ve bir kapsam bölgesini (bir elips) verir.

<a id="jcgm102-9.2.2-additive-gaussian"></a>

### Toplamalı iki değişkenli model, Gauss girdiler (JCGM 102 9.2.2)

`jcgm102-9.2.2-additive-gaussian`

#### Bu proje ne hakkında?

JCGM 102:2011 9.2.2: ortak bir Gauss etkisini paylaşan iki çıktı; GUM sonuçları kesindir ve Monte Carlo yöntemiyle doğrulanır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **Y1**: `Y1 = X1 + X3`
- **Y2**: `Y2 = X2 + X3`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **X3**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 3 (GUF ve M = 10⁶ ile MCM).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.2.3-additive-rectangular"></a>

### Toplamalı iki değişkenli model, dikdörtgen ortak etki (JCGM 102 9.2.3)

`jcgm102-9.2.3-additive-rectangular`

#### Bu proje ne hakkında?

JCGM 102:2011 9.2.3: ortak etki X₃ dikdörtgen (standart sapma 1) olmak üzere 9.2.2 ile aynı; GUM kapsam faktörleri doğrulanmaz.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **Y1**: `Y1 = X1 + X3`
- **Y2**: `Y2 = X2 + X3`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **X3**. Yalnızca sınırlar biliniyor: değer -1,7321 ile 1,7321 arasında (0 ± 1,7321) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 1,7321/√3 = 1,00.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 4 (GUF ve M = 10⁶ ile MCM).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.2.4-additive-dominant-rectangular"></a>

### Toplamalı iki değişkenli model, baskın dikdörtgen ortak etki (JCGM 102 9.2.4)

`jcgm102-9.2.4-additive-dominant-rectangular`

#### Bu proje ne hakkında?

JCGM 102:2011 9.2.4: ortak dikdörtgen etkinin standart sapması 3'tür; r(y₁, y₂) = 0,9 ve GUM k_p ile k_q değerleri yaklaşık %7 ve %20 büyüktür.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **Y1**: `Y1 = X1 + X3`
- **Y2**: `Y2 = X2 + X3`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **X3**. Yalnızca sınırlar biliniyor: değer -5,1962 ile 5,1962 arasında (0 ± 5,1962) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5,1962/√3 = 3,00.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 5 (GUF ve M = 10⁶ ile MCM).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## İnterpolasyonla pH (Orta)

Bir çözeltinin pH'ı, 25 °C'de ve başka bir sıcaklıkta iki tampon standarttan. Türevler, standartların sertifikalı değerleri korelasyonlu olduğunda ne olduğunu gösterir.

<a id="gum5-e1-ph-temperature-interpolation-correlated"></a>

### Üç doğrusal interpolasyonla 23,7 °C'de pH, her standardın sertifikalı değerleri sıcaklıklar arasında ilişkili (JCGM GUM-5 Örnek 1)

`gum5-e1-ph-temperature-interpolation-correlated`

#### Bu proje ne hakkında?

Bağımsız durum gibi, ancak her standardın 20 °C ve 25 °C'deki pH değerleri, ortak Bates–Guggenheim kuralının gösterdiği gibi tam ilişkilidir (GUM-5 1.10.3.3).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **pH_X**: Deney çözeltisinin T_X'teki pH değeri. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
pH_X = pH_T1 + (pH_T2 - pH_T1)*(T_X - T1)/(T2 - T1)
```

Denklemi okunur tutmak için önce 2 ara büyüklük hesaplanır:

- `pH_T1 = pH_S1_T1 + (pH_S2_T1 - pH_S1_T1)*(E_X - E_S1)/(E_S2 - E_S1)`
- `pH_T2 = pH_S1_T2 + (pH_S2_T2 - pH_S1_T2)*(E_X - E_S1)/(E_S2 - E_S1)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 10 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **T_X**: Deney çözeltisinin sıcaklığı. En iyi değer 23,7 °C, standart belirsizlik u = 0,0058 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **E_X**: Deney çözeltisindeki potansiyel. En iyi değer -1,875 mV, standart belirsizlik u = 0,025 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **E_S1**: S1 standardındaki potansiyel. En iyi değer 6,15 mV, standart belirsizlik u = 0,029 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **E_S2**: S2 standardındaki potansiyel. En iyi değer -26,35 mV, standart belirsizlik u = 0,029 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **pH_S1_T1**: S1'in 25 °C'deki pH değeri. En iyi değer 6,864, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **pH_S2_T1**: S2'nin 25 °C'deki pH değeri. En iyi değer 7,4157, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **pH_S1_T2**: S1'in 20 °C'deki pH değeri. En iyi değer 6,8796, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **pH_S2_T2**: S2'nin 20 °C'deki pH değeri. En iyi değer 7,4323, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **T1**: Sertifika sıcaklığı T₁. Tam bilinen bir değer, 25 °C; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
10. **T2**: Sertifika sıcaklığı T₂. Tam bilinen bir değer, 20 °C; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- pH_S1_T1 ↔ pH_S1_T2: r = 1
- pH_S2_T1 ↔ pH_S2_T2: r = 1


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 7,0043
- standart belirsizlik u = 0,0041

Kaynak: JCGM GUM-5:2026, 1.9.1: pH_X = 7,0043 (nota bakınız), u = 0,0041.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e1-ph-temperature-interpolation"></a>

### Potansiyel ve sıcaklıkta üç doğrusal interpolasyonla 23,7 °C'de pH, bağımsız sertifikalı değerler (JCGM GUM-5 Örnek 1)

`gum5-e1-ph-temperature-interpolation`

#### Bu proje ne hakkında?

Deney çözeltisi 23,7 °C'dedir, sertifika sıcaklıkları 20 °C ile 25 °C arasında: her sıcaklıkta potansiyelde, sonra sıcaklıkta interpolasyon; tüm girdiler bağımsız (GUM-5 1.7.1).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **pH_X**: Deney çözeltisinin T_X'teki pH değeri. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
pH_X = pH_T1 + (pH_T2 - pH_T1)*(T_X - T1)/(T2 - T1)
```

Denklemi okunur tutmak için önce 2 ara büyüklük hesaplanır:

- `pH_T1 = pH_S1_T1 + (pH_S2_T1 - pH_S1_T1)*(E_X - E_S1)/(E_S2 - E_S1)`
- `pH_T2 = pH_S1_T2 + (pH_S2_T2 - pH_S1_T2)*(E_X - E_S1)/(E_S2 - E_S1)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 10 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **T_X**: Deney çözeltisinin sıcaklığı. En iyi değer 23,7 °C, standart belirsizlik u = 0,0058 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **E_X**: Deney çözeltisindeki potansiyel. En iyi değer -1,875 mV, standart belirsizlik u = 0,025 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **E_S1**: S1 standardındaki potansiyel. En iyi değer 6,15 mV, standart belirsizlik u = 0,029 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **E_S2**: S2 standardındaki potansiyel. En iyi değer -26,35 mV, standart belirsizlik u = 0,029 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **pH_S1_T1**: S1'in 25 °C'deki pH değeri. En iyi değer 6,864, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **pH_S2_T1**: S2'nin 25 °C'deki pH değeri. En iyi değer 7,4157, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **pH_S1_T2**: S1'in 20 °C'deki pH değeri. En iyi değer 6,8796, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **pH_S2_T2**: S2'nin 20 °C'deki pH değeri. En iyi değer 7,4323, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **T1**: Sertifika sıcaklığı T₁. Tam bilinen bir değer, 25 °C; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
10. **T2**: Sertifika sıcaklığı T₂. Tam bilinen bir değer, 20 °C; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 7,0043
- standart belirsizlik u = 0,0032

Kaynak: JCGM GUM-5:2026, 1.9.1: pH_X,T1 = 7,0002, pH_X,T2 = 7,0161, pH_X = 7,0043; u bağımsız girdiler için yeniden hesaplanmıştır (0,0032); kılavuz 0,0041 basar (nota bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e1-ph-two-point-25c-correlated-standards"></a>

### Tam ilişkili pH standartlarıyla iki noktalı interpolasyon (JCGM GUM-5 Örnek 1)

`gum5-e1-ph-two-point-25c-correlated-standards`

#### Bu proje ne hakkında?

25 °C durumu gibi, ancak Bates–Guggenheim kuralını paylaşan sertifikalı pH değerleri arasında korelasyon 1.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **pH_X**: Deney çözeltisinin pH değeri. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
pH_X = pH_S1 + (pH_S2 - pH_S1)*(E_X - E_S1)/(E_S2 - E_S1)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **E_X**: Deney çözeltisindeki potansiyel (4 ölçümün ortalaması). En iyi değer -1,875 mV, standart belirsizlik u = 0,025 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **E_S1**: S1 standardındaki potansiyel (SRM 186-I-g). En iyi değer 6,15 mV, standart belirsizlik u = 0,0289 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **E_S2**: S2 standardındaki potansiyel (SRM 186-II-g). En iyi değer -26,35 mV, standart belirsizlik u = 0,0289 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **pH_S1**: S1'in 25 °C'deki sertifikalı pH değeri. En iyi değer 6,864, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **pH_S2**: S2'nin 25 °C'deki sertifikalı pH değeri. En iyi değer 7,4157, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- pH_S1 ↔ pH_S2: r = 1


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 7,0002
- standart belirsizlik u = 0,0051

Kaynak: JCGM GUM-5:2026, 1.10.3.5: tam ilişkili standartlarla u(pH_X) = 0,0051.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e1-ph-two-point-25c"></a>

### 25 °C'de iki noktalı interpolasyonla bir deney çözeltisinin pH değeri (JCGM GUM-5 Örnek 1)

`gum5-e1-ph-two-point-25c`

#### Bu proje ne hakkında?

Aynı hücreyle ölçülen iki NIST fosfat pH standardı arasında parantez yöntemi; IUPAC uygulamasındaki gibi girdiler bağımsız.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **pH_X**: Deney çözeltisinin pH değeri. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
pH_X = pH_S1 + (pH_S2 - pH_S1)*(E_X - E_S1)/(E_S2 - E_S1)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **E_X**: Deney çözeltisindeki potansiyel (4 ölçümün ortalaması). En iyi değer -1,875 mV, standart belirsizlik u = 0,025 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **E_S1**: S1 standardındaki potansiyel (SRM 186-I-g). En iyi değer 6,15 mV, standart belirsizlik u = 0,0289 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **E_S2**: S2 standardındaki potansiyel (SRM 186-II-g). En iyi değer -26,35 mV, standart belirsizlik u = 0,0289 mV (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **pH_S1**: S1'in 25 °C'deki sertifikalı pH değeri. En iyi değer 6,864, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **pH_S2**: S2'nin 25 °C'deki sertifikalı pH değeri. En iyi değer 7,4157, standart belirsizlik u = 0,0051 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 7,0002
- standart belirsizlik u = 0,0041

Kaynak: JCGM GUM-5:2026, 1.8.1: pH_X = 7,0002, u = 0,0041.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir sera gazı envanteri (Orta)

Tarım makinelerinden emisyonlar. Aynı faaliyet verisi birkaç gaz için kullanıldığında sonuçlar korelasyonlu olur ve toplam belirsizlik değişir.

<a id="gum5-e5-ghg-inventory-agriculture-shared-activity"></a>

### Sera gazı envanteri, tarımda hareketli makineler (BK 2018): CH₄ ve N₂O'nun paylaştığı faaliyet verisi (JCGM GUM-5 Örnek 5)

`gum5-e5-ghg-inventory-agriculture-shared-activity`

#### Bu proje ne hakkında?

CO₂ eşdeğeri emisyonlar E = Σ s_i A_i F_i (IPCC kategorisi A4ci) ve standart belirsizlikleri LPU ile, A₂ = A₃ ve A₅ = A₆ her biri tek büyüklük olarak (5.4.9).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (kt): Alt sektörün toplam CO₂ eşdeğeri emisyonları. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = 11/3*A1*F1 + 25*A2*F2 + 298*A2*F3 + 11/3*A4*F4 + 25*A5*F5 + 298*A5*F6
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 10 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **A1**: Faaliyet verisi, gas oil (CO2); U_rel = %38,6 (%95). En iyi değer 56317,092 TJ, standart belirsizlik u = 11091 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **F1**: Emisyon faktörü, CO2, gas oil; U_rel = %2,7 (%95). En iyi değer 0,020438 kt/TJ, standart belirsizlik u = 0,00028154 kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **A2**: Faaliyet verisi, gas oil (CH₄ ve N₂O); U_rel = %1,6 (%95). En iyi değer 56317,092 TJ, standart belirsizlik u = 459,73 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **F2**: Emisyon faktörü, CH4, gas oil; U_rel = %80,0 (%95). En iyi değer 3,5368 × 10⁻⁶ kt/TJ, standart belirsizlik u = 1,4436 × 10⁻⁶ kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **F3**: Emisyon faktörü, N2O, gas oil; U_rel = %216,3 (%95). En iyi değer 3,0984 × 10⁻⁶ kt/TJ, standart belirsizlik u = 3,4193 × 10⁻⁶ kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **A4**: Faaliyet verisi, petrol (CO2); U_rel = %50,7 (%95). En iyi değer 67,1856 TJ, standart belirsizlik u = 17,379 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **F4**: Emisyon faktörü, CO2, petrol; U_rel = %2,0 (%95). En iyi değer 0,019127 kt/TJ, standart belirsizlik u = 0,00019517 kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **A5**: Faaliyet verisi, petrol (CH₄ ve N₂O); U_rel = %1,6 (%95). En iyi değer 67,1856 TJ, standart belirsizlik u = 0,54845 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **F5**: Emisyon faktörü, CH4, petrol; U_rel = %80,0 (%95). En iyi değer 4,8654 × 10⁻⁵ kt/TJ, standart belirsizlik u = 1,9859 × 10⁻⁵ kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **F6**: Emisyon faktörü, N2O, petrol; U_rel = %216,3 (%95). En iyi değer 3,3578e-7 kt/TJ, standart belirsizlik u = 3,7056 × 10⁻⁷ kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 1,96 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 4282,04 kt
- standart belirsizlik u = 835,16 kt

Kaynak: JCGM GUM-5:2026, 5.4.6, (5.6), (5.8) ve Tablo 5.3: E(CO₂e) = 4282,04 kt, u = 835,16 kt.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e5-ghg-inventory-agriculture"></a>

### Sera gazı envanteri, tarımda hareketli makineler (BK 2018): bağımsız girdiler (JCGM GUM-5 Örnek 5)

`gum5-e5-ghg-inventory-agriculture`

#### Bu proje ne hakkında?

CO₂ eşdeğeri emisyonlar E = Σ s_i A_i F_i (IPCC kategorisi A4ci) ve standart belirsizlikleri LPU ile, bağımsız girdiler için (5.5).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **E** (kt): Alt sektörün toplam CO₂ eşdeğeri emisyonları. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
E = 11/3*A1*F1 + 25*A2*F2 + 298*A3*F3 + 11/3*A4*F4 + 25*A5*F5 + 298*A6*F6
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 12 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **A1**: Faaliyet verisi, gas oil (CO2); U_rel = %38,6 (%95). En iyi değer 56317,092 TJ, standart belirsizlik u = 11091 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **F1**: Emisyon faktörü, CO2, gas oil; U_rel = %2,7 (%95). En iyi değer 0,020438 kt/TJ, standart belirsizlik u = 0,00028154 kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **A2**: Faaliyet verisi, gas oil (CH4); U_rel = %1,6 (%95). En iyi değer 56317,092 TJ, standart belirsizlik u = 459,73 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **F2**: Emisyon faktörü, CH4, gas oil; U_rel = %80,0 (%95). En iyi değer 3,5368 × 10⁻⁶ kt/TJ, standart belirsizlik u = 1,4436 × 10⁻⁶ kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **A3**: Faaliyet verisi, gas oil (N2O); U_rel = %1,6 (%95). En iyi değer 56317,092 TJ, standart belirsizlik u = 459,73 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **F3**: Emisyon faktörü, N2O, gas oil; U_rel = %216,3 (%95). En iyi değer 3,0984 × 10⁻⁶ kt/TJ, standart belirsizlik u = 3,4193 × 10⁻⁶ kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **A4**: Faaliyet verisi, petrol (CO2); U_rel = %50,7 (%95). En iyi değer 67,1856 TJ, standart belirsizlik u = 17,379 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **F4**: Emisyon faktörü, CO2, petrol; U_rel = %2,0 (%95). En iyi değer 0,019127 kt/TJ, standart belirsizlik u = 0,00019517 kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **A5**: Faaliyet verisi, petrol (CH4); U_rel = %1,6 (%95). En iyi değer 67,1856 TJ, standart belirsizlik u = 0,54845 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **F5**: Emisyon faktörü, CH4, petrol; U_rel = %80,0 (%95). En iyi değer 4,8654 × 10⁻⁵ kt/TJ, standart belirsizlik u = 1,9859 × 10⁻⁵ kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
11. **A6**: Faaliyet verisi, petrol (N2O); U_rel = %1,6 (%95). En iyi değer 67,1856 TJ, standart belirsizlik u = 0,54845 TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
12. **F6**: Emisyon faktörü, N2O, petrol; U_rel = %216,3 (%95). En iyi değer 3,3578e-7 kt/TJ, standart belirsizlik u = 3,7056 × 10⁻⁷ kt/TJ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 1,96 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 4282,04 kt
- standart belirsizlik u = 835,16 kt

Kaynak: JCGM GUM-5:2026, 5.4.6, (5.6), (5.8) ve Tablo 5.3: E(CO₂e) = 4282,04 kt, u = 835,16 kt.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Tahribatlı testlerden kopma kuvveti (Orta)

Altı tel halat kopana kadar çekilir. Yalnızca altı veri: bir t dağılımı ve Monte Carlo ölçeğine dikkatli bir bakış.

<a id="gum5-e15-wire-rope-breaking-force"></a>

### Altı tahribatlı deneyden çelik halatın kopma kuvveti (JCGM GUM-5 Örnek 15)

`gum5-e15-wire-rope-breaking-force`

#### Bu proje ne hakkında?

F₀ = F̄ − B − B_ref − e_res − e_T: bir makaradan kesilen altı parça üzerindeki deneylerin ortalaması; yük hücreleri, çözünürlük ve sıcaklık için dikdörtgen düzeltmeler.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **F_0** (kN): Halatın kopma kuvveti. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
F_0 = F - B - B_ref - e_res - e_T
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **F**: Altı parçanın kopma kuvvetleri. 6 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 10005,7 kN; okumalar s = 1,75 kN standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 1,75/√6 = 0,715 kN, serbestlik derecesi ν = n − 1 = 5.
2. **B**: Deney makinesi yük hücresinin hatası. Yalnızca sınırlar biliniyor: değer -6,8 ile 6,8 kN arasında (0 ± 6,8) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 6,8/√3 = 3,93 kN.
3. **B_ref**: Referans yük hücresinin hatası. Yalnızca sınırlar biliniyor: değer -8,2 ile 8,2 kN arasında (0 ± 8,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 8,2/√3 = 4,73 kN.
4. **e_res**: Kuvvet ölçme sisteminin çözünürlüğü. Yalnızca sınırlar biliniyor: değer -8,7 ile 8,7 kN arasında (0 ± 8,7) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 8,7/√3 = 5,02 kN.
5. **e_T**: Sıcaklık etkisi. Yalnızca sınırlar biliniyor: değer -10,2 ile 10,2 kN arasında (0 ± 10,2) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 10,2/√3 = 5,89 kN.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 10005,7 kN
- standart belirsizlik u = 9,9 kN
- genişletilmiş belirsizlik U = 19,4 kN

Kaynak: JCGM GUM-5:2026, 15.4.1: F₀ = 10 005,7 kN, u = 9,9 kN, U = 19,4 kN (k = 1,96). MCM standart belirsizliği yeniden hesaplanmıştır: belirtilen girdilerle 9,93 kN; kılavuz 10,1 kN basar (nota bakınız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Vakum ölçer: bilinen bir düzeltmeyle ne yapmalı (Orta)

Bir düzeltme faktörü uygulanabilir, yok sayılabilir ya da belirsizliğe katılabilir. Üç senaryo ve ortak bir sıcaklık hatasının bir basınç farkına etkisi karşılaştırılır.

<a id="emue-e2.2-vacuum-gauge-correction-applied"></a>

### Vakum ölçer kalibrasyonu, düzeltme çarpanı ve uygunluk olasılığı, senaryo 1 (EMUE E2.2)

`emue-e2.2-vacuum-gauge-correction-applied`

#### Bu proje ne hakkında?

Senaryo 1: referans düzeltmesi Δp_m = %0,05 p_std uygulanır, u(Δp_m) = 0,5 Pa. Kalibrasyonun 8. noktası (256 Pa); şartname: okuma hatası ±%0,5 içinde, yani 0,995 ≤ f ≤ 1,005.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **f**: Ölçerin düzeltme çarpanı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
f = (p_std + dp_m)/p_UUC
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **p_std**: Referans basınç (referans ölçerin göstergesi). En iyi değer 256,4 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 1,5 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 1,5/2 = 0,750 Pa olur (normal dağılım).
2. **p_UUC**: Kalibre edilen ölçerin göstergesi. En iyi değer 255,9 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,23 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,23/2 = 0,115 Pa olur (normal dağılım).
3. **dp_m**: referans düzeltmesi Δp_m = %0,05 p_std uygulanır, u(Δp_m) = 0,5 Pa. En iyi değer 0,1282 Pa, standart belirsizlik u = 0,5 Pa (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Ardından: sonuç spesifikasyona uyuyor mu?

Ölçülen büyüklük 0,995 ile 1,005 arasında olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,0025
- genişletilmiş belirsizlik U = 0,0071

Kaynak: EMUE E2.2, 8. nokta: f = 1,0025, U(f) = 0,0071, p_c = %74,5 (Tablo E2.2.2 ve E2.2.3).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="emue-e2.2-vacuum-gauge-correction-ignored"></a>

### Vakum ölçer kalibrasyonu, düzeltme çarpanı ve uygunluk olasılığı, senaryo 3 (EMUE E2.2)

`emue-e2.2-vacuum-gauge-correction-ignored`

#### Bu proje ne hakkında?

Senaryo 3: düzeltme ve belirsizliği yok sayılır. Kalibrasyonun 8. noktası (256 Pa); şartname: okuma hatası ±%0,5 içinde, yani 0,995 ≤ f ≤ 1,005.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **f**: Ölçerin düzeltme çarpanı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
f = p_std/p_UUC
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **p_std**: Referans basınç (referans ölçerin göstergesi). En iyi değer 256,4 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 1,5 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 1,5/2 = 0,750 Pa olur (normal dağılım).
2. **p_UUC**: Kalibre edilen ölçerin göstergesi. En iyi değer 255,9 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,23 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,23/2 = 0,115 Pa olur (normal dağılım).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Ardından: sonuç spesifikasyona uyuyor mu?

Ölçülen büyüklük 0,995 ile 1,005 arasında olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,002
- genişletilmiş belirsizlik U = 0,0059

Kaynak: EMUE E2.2, 8. nokta: f = 1,0020, U(f) = 0,0059, p_c = %83,8 (Tablo E2.2.6 ve E2.2.7).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="emue-e2.2-vacuum-gauge-correction-in-uncertainty"></a>

### Vakum ölçer kalibrasyonu, düzeltme çarpanı ve uygunluk olasılığı, senaryo 2 (EMUE E2.2)

`emue-e2.2-vacuum-gauge-correction-in-uncertainty`

#### Bu proje ne hakkında?

Senaryo 2: düzeltme uygulanmaz, belirsizliğine katılır, u² = 0,5² + (%0,05 p_std)²/3. Kalibrasyonun 8. noktası (256 Pa); şartname: okuma hatası ±%0,5 içinde, yani 0,995 ≤ f ≤ 1,005.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **f**: Ölçerin düzeltme çarpanı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
f = (p_std + dp_m)/p_UUC
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 3 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **p_std**: Referans basınç (referans ölçerin göstergesi). En iyi değer 256,4 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 1,5 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 1,5/2 = 0,750 Pa olur (normal dağılım).
2. **p_UUC**: Kalibre edilen ölçerin göstergesi. En iyi değer 255,9 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,23 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,23/2 = 0,115 Pa olur (normal dağılım).
3. **dp_m**: düzeltme uygulanmaz, belirsizliğine katılır, u² = 0,5² + (%0,05 p_std)²/3. En iyi değer 0 Pa, standart belirsizlik u = 0,505449 Pa (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Ardından: sonuç spesifikasyona uyuyor mu?

Ölçülen büyüklük 0,995 ile 1,005 arasında olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,002
- genişletilmiş belirsizlik U = 0,0071

Kaynak: EMUE E2.2, 8. nokta: f = 1,0020, U(f) = 0,0071, p_c = %77,8 (Tablo E2.2.4 ve E2.2.5).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="emue-e2.2-vacuum-gauge-sum-model-correlated"></a>

### Vakum ölçer, ortak sıcaklık hatalı basınç farkı: korelasyon dahil (EMUE E2.2.7)

`emue-e2.2-vacuum-gauge-sum-model-correlated`

#### Bu proje ne hakkında?

Δp = p_UUC − (p_std + Δp_m), iki ölçer için p = p′(1 + αΔT) (E2.2.8); 8. nokta, şartname okumanın ±%0,5'i.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dp** (Pa): Basınç farkı (ölçerin hatası). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dp = p_UUC_obs*(1 + a_UUC*dT) - (p_std_obs*(1 + a_std*dT) + dp_m)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **p_std_obs**: Gözlenen referans basınç. En iyi değer 256,4 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 1,5 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 1,5/2 = 0,750 Pa olur (normal dağılım).
2. **p_UUC_obs**: Kalibre edilen ölçerin gözlenen göstergesi. En iyi değer 255,9 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,23 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,23/2 = 0,115 Pa olur (normal dağılım).
3. **a_std**: Sıcaklık katsayısı, referans. En iyi değer 0,0033333 1/K, standart belirsizlik u = 0,00033333 1/K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **a_UUC**: Sıcaklık katsayısı, kalibre edilen ölçer. En iyi değer 0,0033333 1/K, standart belirsizlik u = 0,00033333 1/K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **dT**: Gaz sıcaklığının sistematik hatası. En iyi değer 0 K, standart belirsizlik u = 0,57 K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **dp_m**: Referans basınç düzeltmesi %0,05 p_std. En iyi değer 0,1282 Pa, standart belirsizlik u = 0,5 Pa (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Ardından: sonuç spesifikasyona uyuyor mu?

Ölçülen büyüklük -1,2795 ile 1,2795 arasında olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = -0,6282 Pa
- standart belirsizlik u = 0,909 Pa

Kaynak: EMUE E2.2.7: u(p_std) = 0,90 Pa, u(p_UUC) = 0,51 Pa; u(Δp) korelasyonsuz 1,14 Pa, korelasyonla 0,91 Pa; p_c = 0,67 ve 0,75.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="emue-e2.2-vacuum-gauge-sum-model-correlation-ignored"></a>

### Vakum ölçer, ortak sıcaklık hatalı basınç farkı: korelasyon yok sayılmış (EMUE E2.2.7)

`emue-e2.2-vacuum-gauge-sum-model-correlation-ignored`

#### Bu proje ne hakkında?

Δp = p_UUC − (p_std + Δp_m), iki ölçer için p = p′(1 + αΔT) (E2.2.8); 8. nokta, şartname okumanın ±%0,5'i.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dp** (Pa): Basınç farkı (ölçerin hatası). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dp = p_UUC_obs*(1 + a_UUC*dT_UUC) - (p_std_obs*(1 + a_std*dT_std) + dp_m)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **p_std_obs**: Gözlenen referans basınç. En iyi değer 256,4 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 1,5 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 1,5/2 = 0,750 Pa olur (normal dağılım).
2. **p_UUC_obs**: Kalibre edilen ölçerin gözlenen göstergesi. En iyi değer 255,9 Pa. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = 0,23 Pa, k = 2 olarak verir; bu yüzden standart belirsizlik u = U/k = 0,23/2 = 0,115 Pa olur (normal dağılım).
3. **a_std**: Sıcaklık katsayısı, referans. En iyi değer 0,0033333 1/K, standart belirsizlik u = 0,00033333 1/K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **a_UUC**: Sıcaklık katsayısı, kalibre edilen ölçer. En iyi değer 0,0033333 1/K, standart belirsizlik u = 0,00033333 1/K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **dT_std**: Gaz sıcaklığının sistematik hatası (yanlış olarak her ölçer için ayrı alınmış). En iyi değer 0 K, standart belirsizlik u = 0,57 K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **dT_UUC**: Gaz sıcaklığının sistematik hatası (yanlış olarak her ölçer için ayrı alınmış). En iyi değer 0 K, standart belirsizlik u = 0,57 K (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **dp_m**: Referans basınç düzeltmesi %0,05 p_std. En iyi değer 0,1282 Pa, standart belirsizlik u = 0,5 Pa (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Ardından: sonuç spesifikasyona uyuyor mu?

Ölçülen büyüklük -1,2795 ile 1,2795 arasında olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = -0,6282 Pa
- standart belirsizlik u = 1,14 Pa

Kaynak: EMUE E2.2.7: u(p_std) = 0,90 Pa, u(p_UUC) = 0,51 Pa; u(Δp) korelasyonsuz 1,14 Pa, korelasyonla 0,91 Pa; p_c = 0,67 ve 0,75.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Validasyon verilerinden topraktaki kurşun (Orta)

Bir ICP-OES yönteminin validasyonundan kurulan bir ISO 21748 bütçesi: kesinlik, gerçeklik ve diğer terimler.

<a id="emue-e3.4-soil-lead-iso21748-rms-bias"></a>

### Topraktaki kral suyunda çözünür kurşun, ICP-OES: doğrulama verilerinden ISO 21748 bütçesi, kareler ortalamasının kökü sapma belirsizliği (EMUE E3.4)

`emue-e3.4-soil-lead-iso21748-rms-bias`

#### Bu proje ne hakkında?

u²(w)/w² = u²(δ̂)/w² + s_I²/w² + (c_T u_T)² + (c_t u_t)² + (u(γ₀)/γ₀)²; kesinlik, gerçeklik ve sağlamlık çalışmalarından bağıl özet değerlerle; u(δ̂)/w = 0,035.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w** (mg/kg): Toprakta kral suyunda çözünür Pb. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w = w_obs*(1 + d_bias)*(1 + e_I)*(1 + 0.004*dT)*(1 + 0.0009*dt)*(g0/984.74)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **w_obs**: Ölçülen kütle kesri (C279a'da Pb sonucu, E3.4.7). Tam bilinen bir değer, 27,8 mg/kg; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **d_bias**: Bağıl sapma terimi; tahmini (≈ −%10) uygulanmaz, belirsizliği uygulanır. En iyi değer 0, standart belirsizlik u = 0,035 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **e_I**: Bağıl ara kesinlik (kareler ortalamasının kökü RSD). En iyi değer 0, standart belirsizlik u = 0,036 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **dT**: Ekstraksiyon sıcaklığının 65 °C'den sapması. Yalnızca sınırlar biliniyor: değer -5 ile 5 °C arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 °C.
5. **dt**: Ekstraksiyon süresinin 120 min'den sapması. Yalnızca sınırlar biliniyor: değer -5 ile 5 min arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 min.
6. **g0**: Pb stok çözeltisi (sertifika). En iyi değer 984,74 mg/kg, standart belirsizlik u = 0,985 mg/kg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 27,8 mg/kg
- standart belirsizlik u = 1,4344 mg/kg
- genişletilmiş belirsizlik U = 2,869 mg/kg

Kaynak: EMUE E3.4, (E3.4.18) ve E3.4.7: u′(Pb)² = 0,002 526, u′ ≈ %5, sonuç (27,8 ± 2,8) mg/kg (k = 2); bu varyant u(δ̂)/w'yi kareler ortalamasının kökü olarak yeniden hesaplar (F87).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="emue-e3.4-soil-lead-iso21748"></a>

### Topraktaki kral suyunda çözünür kurşun, ICP-OES: doğrulama verilerinden ISO 21748 bütçesi (EMUE E3.4)

`emue-e3.4-soil-lead-iso21748`

#### Bu proje ne hakkında?

u²(w)/w² = u²(δ̂)/w² + s_I²/w² + (c_T u_T)² + (c_t u_t)² + (u(γ₀)/γ₀)²; kesinlik, gerçeklik ve sağlamlık çalışmalarından bağıl özet değerlerle; u(δ̂)/w = 0,033.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w** (mg/kg): Toprakta kral suyunda çözünür Pb. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w = w_obs*(1 + d_bias)*(1 + e_I)*(1 + 0.004*dT)*(1 + 0.0009*dt)*(g0/984.74)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **w_obs**: Ölçülen kütle kesri (C279a'da Pb sonucu, E3.4.7). Tam bilinen bir değer, 27,8 mg/kg; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **d_bias**: Bağıl sapma terimi; tahmini (≈ −%10) uygulanmaz, belirsizliği uygulanır. En iyi değer 0, standart belirsizlik u = 0,033 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **e_I**: Bağıl ara kesinlik (kareler ortalamasının kökü RSD). En iyi değer 0, standart belirsizlik u = 0,036 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **dT**: Ekstraksiyon sıcaklığının 65 °C'den sapması. Yalnızca sınırlar biliniyor: değer -5 ile 5 °C arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 °C.
5. **dt**: Ekstraksiyon süresinin 120 min'den sapması. Yalnızca sınırlar biliniyor: değer -5 ile 5 min arasında (0 ± 5) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 5/√3 = 2,89 min.
6. **g0**: Pb stok çözeltisi (sertifika). En iyi değer 984,74 mg/kg, standart belirsizlik u = 0,985 mg/kg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 27,8 mg/kg
- standart belirsizlik u = 1,3972 mg/kg
- genişletilmiş belirsizlik U = 2,794 mg/kg

Kaynak: EMUE E3.4, (E3.4.18) ve E3.4.7: u′(Pb)² = 0,002 526, u′ ≈ %5, sonuç (27,8 ± 2,8) mg/kg (k = 2).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Geçirgenlikle bir gaz karışımı (Orta)

Bir geçirgenlik tüpünden amonyağın miktar kesri: kütle kayıp hızı, akış ve molar kütleler.

<a id="emue-e3.8-ammonia-permeation"></a>

### Permeasyonla N₂ içinde NH₃ kalibrasyon gazı karışımı: amonyağın madde miktarı kesri (EMUE E3.8)

`emue-e3.8-ammonia-permeation`

#### Bu proje ne hakkında?

y = ṅ₁x₁/(ṅ₁ + ṅ₂), ṅ_j = q_mj/M̄_j (E3.8.1): permeasyon hızı manyetik askılı teraziden, taşıyıcı akış termal kütle akış kontrolöründen, iki ana gazın saflıkları ve standart atom ağırlıklarından mol kütleleri.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **y_NH3** (µmol/mol): Karışımdaki NH₃ madde miktarı kesri. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
y_NH3 = 1e6*n1*(1 - x_w1)/(n1 + n2)
```

Denklemi okunur tutmak için önce 6 ara büyüklük hesaplanır:

- `M_NH3 = A_N + 3*A_H`
- `M_H2O = A_H*2 + A_O`
- `M1 = (1 - x_w1)*M_NH3 + x_w1*M_H2O`
- `M2 = (1 - 1e-6*(x2_Ar + x2_CH4 + x2_CO + x2_CO2 + x2_H2 + x2_H2O + x2_O2))*(2*A_N) + 1e-6*(x2_Ar*(A_Ar) + x2_CH4*(A_C + 4*A_H) + x2_CO*(A_C + A_O) + x2_CO2*(A_C + 2*A_O) + x2_H2*(2*A_H) + x2_H2O*(A_H*2 + A_O) + x2_O2*(2*A_O))`
- `n1 = 1e-6*q_m1/M1`
- `n2 = q_m2/M2`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 15 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **q_m1**: 62 °C'de NH₃ tüpünün permeasyon hızı (kütle–zaman doğrusunun EKK eğimi). En iyi değer 13,599 µg/min, standart belirsizlik u = 0,108 µg/min (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **q_m2**: Azot taşıyıcı gazın kütle akışı (termal MFC, 250 mL/min). En iyi değer 0,3126 g/min, standart belirsizlik u = 0,0006 g/min (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **x_w1**: Amonyaktaki su (ana gaz 1); x(NH₃) = 1 − x_w1. En iyi değer 0,01 mol/mol, standart belirsizlik u = 0,002 mol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **A_H**: H standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 1,00784 ile 1,00811 arasında (1,007975 ± 0,00013500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00013500/√3 = 7,79 × 10⁻⁵.
5. **A_C**: C standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 12,0096 ile 12,0116 arasında (12,0106 ± 0,0010000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0010000/√3 = 0,000577.
6. **A_N**: N standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 14,00643 ile 14,00728 arasında (14,006855 ± 0,00042500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00042500/√3 = 0,000245.
7. **A_O**: O standart atom ağırlığı. Yalnızca sınırlar biliniyor: değer 15,99903 ile 15,99977 arasında (15,9994 ± 0,00037000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00037000/√3 = 0,000214.
8. **A_Ar**: Ar standart atom ağırlığı (IUPAC 2013, 39,948 ± 0,001, Tablo E3.8.4'teki gibi). Yalnızca sınırlar biliniyor: değer 39,947 ile 39,949 arasında (39,948 ± 0,001) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,001/√3 = 0,000577.
9. **x2_Ar**: Azottaki Ar (ana gaz 2). En iyi değer 5 µmol/mol, standart belirsizlik u = 3 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **x2_CH4**: Azottaki CH4 (ana gaz 2). En iyi değer 0,008 µmol/mol, standart belirsizlik u = 0,005 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
11. **x2_CO**: Azottaki CO (ana gaz 2). En iyi değer 0,015 µmol/mol, standart belirsizlik u = 0,009 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
12. **x2_CO2**: Azottaki CO2 (ana gaz 2). En iyi değer 0,01 µmol/mol, standart belirsizlik u = 0,006 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
13. **x2_H2**: Azottaki H2 (ana gaz 2). En iyi değer 0,025000 µmol/mol, standart belirsizlik u = 0,015 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
14. **x2_H2O**: Azottaki H2O (ana gaz 2). En iyi değer 0,01 µmol/mol, standart belirsizlik u = 0,006 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
15. **x2_O2**: Azottaki O2 (ana gaz 2). En iyi değer 0,005 µmol/mol, standart belirsizlik u = 0,003 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 70,796 µmol/mol
- standart belirsizlik u = 0,598 µmol/mol
- genişletilmiş belirsizlik U = 1,2 µmol/mol

Kaynak: EMUE E3.8.6 ve Tablo E3.8.5: y(NH₃) = 7,08·10⁻⁵ mol/mol, u = 5,99·10⁻⁷ mol/mol; (70,8 ± 1,2) µmol/mol (k = 2).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Doping kontrolü: idrarda efedrin (Orta)

Ara kesinlik ve sapmadan yukarıdan aşağı bir belirsizlik ve WADA'nın olumsuz bulgu ilan etmek için kullandığı karar sınırı.

<a id="emue-e6.5-ephedrine-wada-decision-limit"></a>

### İdrarda efedrin: korumalı ret kuralı olarak WADA karar sınırı (EMUE E6.5, JCGM 106)

`emue-e6.5-ephedrine-wada-decision-limit`

#### Bu proje ne hakkında?

Eşik T = 10 µg/mL; olumsuz analitik bulgu, derişimin T'yi en az %95 güvenle aşmasını gerektirir; hedef belirsizlik u_c,max = 0,5 µg/mL ile: DL = T + 1,645 u_c,max.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük γ(ephedrine) en çok 10 µg/mL olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı ret: bir ürün ancak sınırın açıkça dışındaysa reddedilir (örneğin haksız bir suçlamadan kaçınılması gereken yasal denetim ya da doping kontrolünde).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EMUE E6.5: DL = T + 1,645 u_c,max; T = 10 µg/mL ve u_c,max = 0,5 µg/mL için 11 µg/mL olarak belirlenmiştir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="emue-e6.5-ephedrine-wada-topdown"></a>

### Doping kontrolünde idrarda efedrin: ara kesinlik ve bir SRM'den top-down belirsizlik (EMUE E6.5)

`emue-e6.5-ephedrine-wada-topdown`

#### Bu proje ne hakkında?

Üçlü sonuçların ortalaması için u_c(x) = √(s_w²/n + u_bias²); u_bias² = s_ref²/n′ + Δ² + u²(x_ref), bir SRM'nin 30 analizinden (Tablo E6.5.1); WADA, 10 µg/mL eşiği yakınında u_c ≤ 0,5 µg/mL (%5) ister.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **γ(ephedrine)** (µg/mL) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 10 µg/mL.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **u(Rw), mean of 3**: standart sapma 2,424871 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Sapma** (sistematik etkiler): 1 kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EMUE E6.5, Tablo E6.5.1: u_prec = %2,4, u_bias = %3,0, u_c = %3,8 (yuvarlamasız yeniden hesaplanan %3,89, F91).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Tekrar üretilebilirlik verisinden belirsizlik (Orta)

Standart bir yöntem bir laboratuvarlar arası çalışmada denenmişse, tekrar üretilebilirlik standart sapması belirsizliğin iyi bir ilk tahminidir (ISO 21748). Örnekler gerektiğinde başka terimler ekler.

<a id="iso21748-c2-meat-content"></a>

### Azot ve yağdan et içeriği (ISO 21748 C.2)

`iso21748-c2-meat-content`

#### Bu proje ne hakkında?

ISO 21748:2017 C.2: girdilerinin her biri tekrar üretilebilirlik tabanlı belirsizlik taşıyan birleşik bir sonuç, w_meat = 100 w_N/f_N + w_fat; bağıl hata terimleri olan bir ölçüm modeli olarak yazılmıştır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_meat** (%): Toplam et içeriği. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_meat = 100 * w_N * (1 + e_L + e_r) / f_N + w_fat * (1 + e_fat)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **w_N**: Toplam azot içeriği (Kjeldahl). Tam bilinen bir değer, 3,29 %; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **e_L**: Bağıl laboratuvar etkisi, s_L = 0,011. En iyi değer 0, standart belirsizlik u = 0,011 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **e_r**: İkili ölçüm ortalamasının bağıl tekrarlanabilirliği, 0,018/√2. En iyi değer 0, standart belirsizlik u = 0,0127279 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **f_N**: Etin azot faktörü. En iyi değer 3,65, standart belirsizlik u = 0,052 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **w_fat**: Yağ içeriği. Tam bilinen bir değer, 5,5 %; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
6. **e_fat**: Yağ sonucunun bağıl tekrar üretilebilirliği, s_R = 0,02. En iyi değer 0, standart belirsizlik u = 0,02 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 95,6 %
- standart belirsizlik u = 1,98 %
- genişletilmiş belirsizlik U = 4 %

Kaynak: ISO 21748:2017 Tablo C.1 ve C.2.6 (w_meat = %95,6, u = %1,98, U = %4,0).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso21748-c4-crude-fibre"></a>

### Hayvan yeminde ham lif, tekrar üretilebilirlik verilerinden (ISO 21748 C.4)

`iso21748-c4-crude-fibre`

#### Bu proje ne hakkında?

ISO 21748:2017 C.4: bir ortak çalışmadan düzeye bağlı tekrar üretilebilirlik ve sabit kütleye kurutmanın belirsizliği; QUAM A6 ile aynı örnek, kurutma terimi farklı ele alınmıştır.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(crude fibre)** (% m/m) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 3 derişim düzeyinde değerlendirilir: 2,3; 4; 8 % m/m.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 3 kayıt:

- **s_R (≤ 2.5)**: bir laboratuvarlar arası çalışmadan tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725 / ISO 21748).
- **s_R (2.5–5)**: bir laboratuvarlar arası çalışmadan tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725 / ISO 21748).
- **s_R (5–10)**: bir laboratuvarlar arası çalışmadan tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725 / ISO 21748).

**Diğer bileşenler**, verildiği gibi eklenir:

- drying to constant mass


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 21748:2017 Tablo C.9 ve C.4.9 (u = %0,31; 0,41; 0,59; U = %0,6; 0,8; 1,2).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso21748-c1-co-emissions"></a>

### Taşıt emisyonlarında karbon monoksit, tekrar üretilebilirlik verilerinden (ISO 21748 C.1)

`iso21748-c1-co-emissions`

#### Bu proje ne hakkında?

ISO 21748:2017 C.1: CO emisyonunun tip onay testi; sapma ve kesinlik kontrol altındayken ortak çalışmanın tekrar üretilebilirlik standart sapması standart belirsizliktir.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **CO** (g/km) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 2,2 g/km.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s_R**: bir laboratuvarlar arası çalışmadan tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725 / ISO 21748).


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 21748:2017 C.1.2 ve C.1.6 (s_r = 0,22, s_R = 0,28 g/km, U = 0,56 g/km).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso21748-c3-aerobic-plate-count-flour"></a>

### Aerobik koloni sayımı, un: düzeltilmiş tekrar üretilebilirlik (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-flour`

#### Bu proje ne hakkında?

ISO 21748:2017 C.3 (AOAC 990.12, un): ortak çalışmanın s_R ve s_r değerlerinden, laboratuvarın daha iyi tekrarlanabilirliğinden (7.3.2) ve bir numune hazırlama teriminden log10(sayım)'ın bağıl belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **log10(CFU), flour** için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s′_R**: bir laboratuvarlar arası çalışmadan tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725 / ISO 21748).

**Diğer bileşenler**, verildiği gibi eklenir:

- sample preparation


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 21748:2017 Tablo C.3 ve C.4 ile C.3.7 (un: s′_R = %5,5, u = %6,4, U = %12,8); un: yeniden hesaplanmış, nota bakınız.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso21748-c3-aerobic-plate-count-shrimp"></a>

### Aerobik koloni sayımı, karides: düzeltilmiş tekrar üretilebilirlik (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-shrimp`

#### Bu proje ne hakkında?

ISO 21748:2017 C.3 (AOAC 990.12, karides): ortak çalışmanın s_R ve s_r değerlerinden, laboratuvarın daha iyi tekrarlanabilirliğinden (7.3.2) ve bir numune hazırlama teriminden log10(sayım)'ın bağıl belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **log10(CFU), shrimp** için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s′_R**: bir laboratuvarlar arası çalışmadan tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725 / ISO 21748).

**Diğer bileşenler**, verildiği gibi eklenir:

- sample preparation


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 21748:2017 Tablo C.3 ve C.4 ile C.3.7 (karides: s′_R = %7,2, u = %7,8, U = %15,6).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso21748-c3-aerobic-plate-count-vegetables"></a>

### Aerobik koloni sayımı, sebze: düzeltilmiş tekrar üretilebilirlik (ISO 21748 C.3)

`iso21748-c3-aerobic-plate-count-vegetables`

#### Bu proje ne hakkında?

ISO 21748:2017 C.3 (AOAC 990.12, sebze): ortak çalışmanın s_R ve s_r değerlerinden, laboratuvarın daha iyi tekrarlanabilirliğinden (7.3.2) ve bir numune hazırlama teriminden log10(sayım)'ın bağıl belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **log10(CFU), vegetables** için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s′_R**: bir laboratuvarlar arası çalışmadan tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725 / ISO 21748).

**Diğer bileşenler**, verildiği gibi eklenir:

- sample preparation


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 21748:2017 Tablo C.3 ve C.4 ile C.3.7 (sebze: s′_R = %8,4, u = %8,9, U = %17,8).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Laboratuvarlar arası kesinlik çalışması (Orta)

Sekiz ya da daha fazla laboratuvar, birkaç düzey, tekrarlı sonuçlar: Cochran ve Grubbs testlerinden sonra bir yöntemin tekrarlanabilirliği ve tekrar üretilebilirliği. Her örnek klasik yöntemle ve REML ile değerlendirilir.

<a id="iso5725-2-c1-sulfur-in-coal-reml"></a>

### Kömürün kükürt içeriği: REML tahminleri (ISO 5725-2 Örnek C.1)

`iso5725-2-c1-sulfur-in-coal-reml`

#### Bu proje ne hakkında?

ISO 5725-2:2019 C.1.9: Örnek C.1 verilerinin Formül (23)–(31) yerine kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi.


#### Adım 1. Fikir: laboratuvarlar arası kesinlik çalışması

8 laboratuvar, **Sulfur content of coal** büyüklüğünü 4 düzeyde tekrarlı ölçtü (ISO 5725-2). Program bundan yöntemin **tekrarlanabilirliğini** s_r (aynı laboratuvar, aynı gün) ve **tekrar üretilebilirliğini** s_R (farklı laboratuvarlar) hesaplar.

Bundan önce aykırı değer testleri (varyanslar için Cochran, ortalamalar için Grubbs) ve Mandel h ve k istatistikleri uymayan laboratuvarları gösterir. Yöntem: REML (kısıtlı en çok olabilirlik); dengesiz verilerle başa çıkar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 5725-2:2019, Tablo C.6.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso5725-2-c1-sulfur-in-coal"></a>

### Kömürün kükürt içeriği: tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725-2 Örnek C.1)

`iso5725-2-c1-sulfur-in-coal`

#### Bu proje ne hakkında?

ISO 5725-2:2019 C.1: dört düzeyde (%0,69 – %3,25) sekiz laboratuvarlı, eksik veya aykırı veri içermeyen, hücre başına sonuç sayıları eşit olmayan tek düzeyli bir deney.


#### Adım 1. Fikir: laboratuvarlar arası kesinlik çalışması

8 laboratuvar, **Sulfur content of coal** büyüklüğünü 4 düzeyde tekrarlı ölçtü (ISO 5725-2). Program bundan yöntemin **tekrarlanabilirliğini** s_r (aynı laboratuvar, aynı gün) ve **tekrar üretilebilirliğini** s_R (farklı laboratuvarlar) hesaplar.

Bundan önce aykırı değer testleri (varyanslar için Cochran, ortalamalar için Grubbs) ve Mandel h ve k istatistikleri uymayan laboratuvarları gösterir. Yöntem: ISO 5725-2'nin klasik varyans analizi.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 5725-2:2019, Formül (C.1)–(C.4), Tablo C.4 ve C.5.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso5725-2-c3-creosote-oil-reml"></a>

### Kreozot yağının termometrik titrasyonu: REML tahminleri (ISO 5725-2 Örnek C.3)

`iso5725-2-c3-creosote-oil-reml`

#### Bu proje ne hakkında?

ISO 5725-2:2019 C.3.8: Örnek C.3'ün tutulan verilerinin kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi.


#### Adım 1. Fikir: laboratuvarlar arası kesinlik çalışması

9 laboratuvar, **Creosote oil, thermometric titration** büyüklüğünü 5 düzeyde tekrarlı ölçtü (ISO 5725-2). Program bundan yöntemin **tekrarlanabilirliğini** s_r (aynı laboratuvar, aynı gün) ve **tekrar üretilebilirliğini** s_R (farklı laboratuvarlar) hesaplar.

Bundan önce aykırı değer testleri (varyanslar için Cochran, ortalamalar için Grubbs) ve Mandel h ve k istatistikleri uymayan laboratuvarları gösterir. Yöntem: REML (kısıtlı en çok olabilirlik); dengesiz verilerle başa çıkar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 5725-2:2019, Tablo C.19.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso5725-2-c3-creosote-oil"></a>

### Kreozot yağının termometrik titrasyonu: aykırı laboratuvar ve kesinlik ilişkileri (ISO 5725-2 Örnek C.3)

`iso5725-2-c3-creosote-oil`

#### Bu proje ne hakkında?

ISO 5725-2:2019 C.3 ve 8.5: beş düzeyde (%4 – %20) dokuz laboratuvar, aykırı bir laboratuvar, reddedilmiş bir hücre ve kesinlik ile düzey arasındaki I–IV fonksiyonel ilişkileri (Tablo 1–4).


#### Adım 1. Fikir: laboratuvarlar arası kesinlik çalışması

9 laboratuvar, **Creosote oil, thermometric titration** büyüklüğünü 5 düzeyde tekrarlı ölçtü (ISO 5725-2). Program bundan yöntemin **tekrarlanabilirliğini** s_r (aynı laboratuvar, aynı gün) ve **tekrar üretilebilirliğini** s_R (farklı laboratuvarlar) hesaplar.

Bundan önce aykırı değer testleri (varyanslar için Cochran, ortalamalar için Grubbs) ve Mandel h ve k istatistikleri uymayan laboratuvarları gösterir. Yöntem: ISO 5725-2'nin klasik varyans analizi.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 5725-2:2019, Tablo C.17 ve C.18, Tablo 1–4 ve C.3.5.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso5725-2-c2-softening-point-pitch-reml"></a>

### Ziftin yumuşama noktası: REML tahminleri (ISO 5725-2 Örnek C.2)

`iso5725-2-c2-softening-point-pitch-reml`

#### Bu proje ne hakkında?

ISO 5725-2:2019 C.2.9: Örnek C.2 verilerinin kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi.


#### Adım 1. Fikir: laboratuvarlar arası kesinlik çalışması

16 laboratuvar, **Softening point of pitch** büyüklüğünü 4 düzeyde tekrarlı ölçtü (ISO 5725-2). Program bundan yöntemin **tekrarlanabilirliğini** s_r (aynı laboratuvar, aynı gün) ve **tekrar üretilebilirliğini** s_R (farklı laboratuvarlar) hesaplar.

Bundan önce aykırı değer testleri (varyanslar için Cochran, ortalamalar için Grubbs) ve Mandel h ve k istatistikleri uymayan laboratuvarları gösterir. Yöntem: REML (kısıtlı en çok olabilirlik); dengesiz verilerle başa çıkar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 5725-2:2019, Tablo C.13.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso5725-2-c2-softening-point-pitch"></a>

### Ziftin yumuşama noktası: eksik verilerle tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725-2 Örnek C.2)

`iso5725-2-c2-softening-point-pitch`

#### Bu proje ne hakkında?

ISO 5725-2:2019 C.2: halka-bilye yöntemiyle ziftin yumuşama noktası; dört düzeyde (88 °C – 102 °C), ikili sonuçlu ve iki eksik hücreli on altı laboratuvar.


#### Adım 1. Fikir: laboratuvarlar arası kesinlik çalışması

16 laboratuvar, **Softening point of pitch** büyüklüğünü 4 düzeyde tekrarlı ölçtü (ISO 5725-2). Program bundan yöntemin **tekrarlanabilirliğini** s_r (aynı laboratuvar, aynı gün) ve **tekrar üretilebilirliğini** s_R (farklı laboratuvarlar) hesaplar.

Bundan önce aykırı değer testleri (varyanslar için Cochran, ortalamalar için Grubbs) ve Mandel h ve k istatistikleri uymayan laboratuvarları gösterir. Yöntem: ISO 5725-2'nin klasik varyans analizi.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 5725-2:2019, Formül (C.25)–(C.33), Tablo C.10–C.12.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Doğrusal bir kalibrasyon (Orta)

Referans gaz karışımlarından geçen bir doğruyla kalibre edilen UV soğurmalı azot monoksit; iki eksende de belirsizlik.

<a id="iso6143-b1-no-uv-linear"></a>

### N₂ içinde NO, UV absorpsiyonu: doğrusal analiz fonksiyonu (ISO 6143 Örnek 1)

`iso6143-b1-no-uv-linear`

#### Bu proje ne hakkında?

ISO 6143:2025 B.2.2: hem içeriklerinde hem de UV yanıtlarında belirsizlik bulunan üç referans gaz karışımı (5 µmol/mol – 50 µmol/mol), genelleştirilmiş en küçük karelerle doğrusal analiz fonksiyonu ve üç karışımın içerikleri ile kovaryansları.


#### Adım 1. Fikir: bir kalibrasyon eğrisi

Bir cihaz, doğrudan **NO** içeriği olmayan bir yanıt verir (burada absorbance). Cihaz, içeriği bilinen 3 referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.

Analiz fonksiyonu: **linear**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).


#### Adım 2. Bilinmeyen numuneler

3 numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 6143:2025 B.2.2, parametreler ile Tablo B.4 ve B.6 (B_LEAST).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Numune alma: marulda nitrat (Orta)

Çift numune yöntemi: sekiz partinin her birinden iki numune, her biri iki kez analiz edilir. Belirsizliğin numune alma payı analiz payından büyüktür.

<a id="ufs-a1-nitrate-lettuce"></a>

### Serada yetişen marulda nitrat: çift numune yöntemi, klasik ANOVA (Eurachem UfS Örnek A1)

`ufs-a1-nitrate-lettuce`

#### Bu proje ne hakkında?

Sekiz marul bölmesi; her birinden iki numune (W düzeninde 10 baştan kompozit numune) ve HPLC ile çift analiz (Tablo A1.1).


#### Adım 1. Fikir: belirsizlik numune almayla başlar

Bir sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca küçük bir numune analiz edilir. **Çift numune yöntemi** (Eurachem UfS), 8 hedeften ikişer bağımsız numune alır ve her birini iki kez analiz eder. Varyans analizi sonra **numune almadan** ve **analizden** gelen değişimi birbirinden ayırır.


> **Deneyin:** Sonuçlarda u(numune alma) ile u(analiz)'i karşılaştırın: çoğu zaman numune alma payı daha büyüktür ve daha iyi bir analiz işe yaramaz.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Şekil A1.2'de yazdırılan klasik ANOVA sonuçları (robust sonuçlar karşılaştırılmamıştır).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Numune alma: toprakta kurşun (Orta)

Büyük bir bağıl belirsizlik logaritmik ölçekte bir çarpan (çarp ya da böl) olarak daha iyi ifade edilir.

<a id="ufs-a2-lead-soil-uncertainty-factor"></a>

### Kirlenmiş üst toprakta kurşun: klasik ANOVA ve belirsizlik faktörü (Eurachem UfS Örnek A2)

`ufs-a2-lead-soil-uncertainty-factor`

#### Bu proje ne hakkında?

100 grid hedefinden onu (30 m × 30 m, 0–150 mm) 3 m arayla çift numune alınarak ICP-AES ile iki kez analiz edilmiştir (Tablo A2.2). Değerler çarpıktır: belirsizlik ln x'in ANOVA'sından bir faktör olarak da ifade edilir.


#### Adım 1. Fikir: belirsizlik numune almayla başlar

Bir sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca küçük bir numune analiz edilir. **Çift numune yöntemi** (Eurachem UfS), 10 hedeften ikişer bağımsız numune alır ve her birini iki kez analiz eder. Varyans analizi sonra **numune almadan** ve **analizden** gelen değişimi birbirinden ayırır.


> **Deneyin:** Sonuçlarda u(numune alma) ile u(analiz)'i karşılaştırın: çoğu zaman numune alma payı daha büyüktür ve daha iyi bir analiz işe yaramaz.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Şekil A2.2'nin klasik ANOVA'sı ve Tablo A2.3'teki logaritmalarınki; Bölüm 6.2 ve 6.3'teki F_U ve u′.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Numune alma: yeraltı suyunda demir (Orta)

Aynı veriler varyans analiziyle ve daha basit aralık yöntemiyle değerlendirilir.

<a id="ufs-a3-iron-groundwater-anova"></a>

### Yeraltı suyunda çözünmüş demir: klasik ANOVA (Eurachem UfS Örnek A3)

`ufs-a3-iron-groundwater-anova`

#### Bu proje ne hakkında?

Bir yeraltı suyu kütlesinde altı izleme kuyusu; her birinden iki numune (hat üzerinde süzülmüş ve korunmuş) ve ICP-AES ile çift laboratuvar analizi (Tablo A3.7).


#### Adım 1. Fikir: belirsizlik numune almayla başlar

Bir sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca küçük bir numune analiz edilir. **Çift numune yöntemi** (Eurachem UfS), 6 hedeften ikişer bağımsız numune alır ve her birini iki kez analiz eder. Varyans analizi sonra **numune almadan** ve **analizden** gelen değişimi birbirinden ayırır.


> **Deneyin:** Sonuçlarda u(numune alma) ile u(analiz)'i karşılaştırın: çoğu zaman numune alma payı daha büyüktür ve daha iyi bir analiz işe yaramaz.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Tablo A3.5'in ANOVA satırı (RANOVA satırı, %1,8, %9,9 ve %72, karşılaştırılmamıştır).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="ufs-a3-iron-groundwater-range"></a>

### Yeraltı suyunda çözünmüş demir: aralık yöntemi (Eurachem UfS Örnek A3, düzeltilmiş)

`ufs-a3-iron-groundwater-range`

#### Bu proje ne hakkında?

Bir yeraltı suyu kütlesinde altı izleme kuyusu; her birinden iki numune (hat üzerinde süzülmüş ve korunmuş) ve ICP-AES ile çift laboratuvar analizi (Tablo A3.7).


#### Adım 1. Fikir: belirsizlik numune almayla başlar

Bir sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca küçük bir numune analiz edilir. **Çift numune yöntemi** (Eurachem UfS), 6 hedeften ikişer bağımsız numune alır ve her birini iki kez analiz eder. Varyans analizi sonra **numune almadan** ve **analizden** gelen değişimi birbirinden ayırır.


> **Deneyin:** Sonuçlarda u(numune alma) ile u(analiz)'i karşılaştırın: çoğu zaman numune alma payı daha büyüktür ve daha iyi bir analiz işe yaramaz.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Düzeltme listesi 1.1 ile düzeltilmiş Tablo A3.7 aralık hesapları (s_T+S+A = 0,604, s_T = 0,601 mg/L, CV_T = %35); düzeltme listesindeki Tablo A3.4 ve özet.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Numune alma: lapada A vitamini (Orta)

Küçük test porsiyonları numune alma varyansını büyütür; 4 g'lık porsiyonlarda kestirilen analitik varyans negatif çıkar, bu da yöntemin sınırlarını gösterir. Çift numunelerin bir kontrol grafiği de vardır.

<a id="ufs-a4-vitamin-a-porridge-40g"></a>

### Bebek lapasında A vitamini, 40 g deney numunesi: çift numune yöntemi (Eurachem UfS Örnek A4)

`ufs-a4-vitamin-a-porridge-40g`

#### Bu proje ne hakkında?

Muzlu ve kayısılı yulaf lapasının on üretim partisinin her birinden iki numune (paket); her biri HPLC için çift deney numunesine bölünmüştür (aynı deney numunesinin tekrar analizi yoktur).


#### Adım 1. Fikir: belirsizlik numune almayla başlar

Bir sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca küçük bir numune analiz edilir. **Çift numune yöntemi** (Eurachem UfS), 10 hedeften ikişer bağımsız numune alır ve her birini iki kez analiz eder. Varyans analizi sonra **numune almadan** ve **analizden** gelen değişimi birbirinden ayırır.


> **Deneyin:** Sonuçlarda u(numune alma) ile u(analiz)'i karşılaştırın: çoğu zaman numune alma payı daha büyüktür ve daha iyi bir analiz işe yaramaz.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Tablo A4.5, A4.6 ve A4.14–A4.15; özet tablonun U′ değerleri ve düzeltme listesi 1.4a ile düzeltilmiş RSD_B = %6,1.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="ufs-a4-vitamin-a-porridge-4g"></a>

### Bebek lapasında A vitamini, 4 g deney numunesi: negatif numune alma varyansı (Eurachem UfS Örnek A4)

`ufs-a4-vitamin-a-porridge-4g`

#### Bu proje ne hakkında?

Muzlu ve kayısılı yulaf lapasının on üretim partisinin her birinden iki numune (paket); her biri HPLC için çift deney numunesine bölünmüştür (aynı deney numunesinin tekrar analizi yoktur).


#### Adım 1. Fikir: belirsizlik numune almayla başlar

Bir sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca küçük bir numune analiz edilir. **Çift numune yöntemi** (Eurachem UfS), 10 hedeften ikişer bağımsız numune alır ve her birini iki kez analiz eder. Varyans analizi sonra **numune almadan** ve **analizden** gelen değişimi birbirinden ayırır.


> **Deneyin:** Sonuçlarda u(numune alma) ile u(analiz)'i karşılaştırın: çoğu zaman numune alma payı daha büyüktür ve daha iyi bir analiz işe yaramaz.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Tablo A4.8 ve A4.9 (V_numune = −2662,15 sıfır alınır) ve Tablo A4.10'un analitik U′ değeri.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="ufs-a4-vitamin-a-qc-chart"></a>

### Bebek lapasında A vitamini: çift numunelerin kontrol grafiği (Eurachem UfS Örnek A4)

`ufs-a4-vitamin-a-qc-chart`

#### Bu proje ne hakkında?

Numune almanın kalite kontrolü: sekiz lapanın her birinin bir partisinden iki numune, her biri iki kez analiz (40 g); iki numunenin bağıl farkı D %, valide edilmiş belirsizlikten gelen sınırlara göre grafiğe işlenir (Tablo A4.13, Şekil A4.4).


#### Adım 1. Fikir: bir yöntemi zaman içinde izlemek

Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.

Sınırlar: hedef sınırlar, yöntemin ulaşması gereken kesinlikten.

16 kontrol değeri girilmiş.


> **Deneyin:** Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Bölüm 7.3'ün sınırları (CL %11, WL %27, AL %36) ve Şekil A4.4: uyarı sınırı dışında değer yok.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Modelleme ile numune alma belirsizliği (Orta)

Çift numuneler yerine numune alma adımları bir GUM bütçesindeki gibi tek tek modellenir.

<a id="ufs-a6-cadmium-soil"></a>

### Tarım toprağında kadmiyum: modelleme yaklaşımı (Eurachem UfS Örnek A6)

`ufs-a6-cadmium-soil`

#### Bu proje ne hakkında?

0,32 ha'lık bir tarlanın (143 m × 22 m, 0–30 cm) kompozit numunelerden ortalama derişimi; numune alma etkilerinin (konumlar arası, strateji, derinlik, bölme, kurutma) ve analizin belirsizlik bütçesi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **x_site** (mg/kg): Tarlanın ortalama kadmiyum içeriği. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
x_site = x_sqr * f_strat * f_depth * f_split * f_dry * f_Rw * f_ref * f_bias
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **x_sqr**: Beş karenin (A1, A3, B2, C1, C3) kompozit numunelerinde kadmiyum (Tablo A6.1). 5 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 0,319200 mg/kg; okumalar s = 0,0387 mg/kg standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 0,0387/√5 = 0,0173 mg/kg, serbestlik derecesi ν = n − 1 = 4.
2. **f_strat**: Numune alma stratejisi (konumsal düzenden sapma). En iyi değer 1, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **f_depth**: 'Derinlik etkisi': Tablo A6.2'deki gradyanla derinlikte ±%10, dikdörtgen. Yalnızca sınırlar biliniyor: değer 0,9376 ile 1,0624 arasında (1 ± 0,0624) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0624/√3 = 0,0360.
4. **f_split**: Konileme ve dörde bölme ile bölme: çift bölmelerin ortalama RSD'si (Tablo A6.3). En iyi değer 1, standart belirsizlik u = 0,037 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **f_dry**: Havada kurutulmuş toprağın nemi, %2'lik aralık, dikdörtgen. Yalnızca sınırlar biliniyor: değer 0,99 ile 1,01 arasında (1 ± 0,01) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,01/√3 = 0,00577.
6. **f_Rw**: Laboratuvar içi tekrar üretilebilirlik (10 deney numunesinin ortalaması, GF-ZAAS). En iyi değer 1, standart belirsizlik u = 0,036 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **f_ref**: CRM'nin sertifikalı değeri. En iyi değer 1, standart belirsizlik u = 0,027 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **f_bias**: CRM ile günden güne sapma düzeltmesi (3'ün ortalaması). En iyi değer 1, standart belirsizlik u = 0,027 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,32 mg/kg
- genişletilmiş belirsizlik U = 0,06 mg/kg

Kaynak: Bölüm 6.7'deki ölçüm sonucu ve genişletilmiş belirsizlik (k = 2).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="ufs-a6-phosphorus-soil"></a>

### Tarım toprağında fosfor: modelleme yaklaşımı (Eurachem UfS Örnek A6)

`ufs-a6-phosphorus-soil`

#### Bu proje ne hakkında?

0,32 ha'lık bir tarlanın (143 m × 22 m, 0–30 cm) kompozit numunelerden ortalama derişimi; numune alma etkilerinin (konumlar arası, strateji, derinlik, bölme, kurutma) ve analizin belirsizlik bütçesi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **x_site** (mg/kg): Tarlanın ortalama fosfor içeriği. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
x_site = x_sqr * f_strat * f_depth * f_split * f_dry * f_Rw * f_bias
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **x_sqr**: Beş karenin (A1, A3, B2, C1, C3) kompozit numunelerinde fosfor (Tablo A6.1). 5 kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama 115,800 mg/kg; okumalar s = 7,43 mg/kg standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = 7,43/√5 = 3,32 mg/kg, serbestlik derecesi ν = n − 1 = 4.
2. **f_strat**: Numune alma stratejisi (konumsal düzenden sapma). En iyi değer 1, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **f_depth**: 'Derinlik etkisi': Tablo A6.2'deki gradyanla derinlikte ±%10, dikdörtgen. Yalnızca sınırlar biliniyor: değer 0,93528 ile 1,0647 arasında (1 ± 0,06472) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,06472/√3 = 0,0374.
4. **f_split**: Konileme ve dörde bölme ile bölme: çift bölmelerin ortalama RSD'si (Tablo A6.3). En iyi değer 1, standart belirsizlik u = 0,0316 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **f_dry**: Havada kurutulmuş toprağın nemi, %2'lik aralık, dikdörtgen. Yalnızca sınırlar biliniyor: değer 0,99 ile 1,01 arasında (1 ± 0,01) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,01/√3 = 0,00577.
6. **f_Rw**: Laboratuvar içi tekrar üretilebilirlik (CAL yöntemi). En iyi değer 1, standart belirsizlik u = 0,017 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **f_bias**: Laboratuvarlar arası karşılaştırmanın tekrar üretilebilirliği s_R'den doğruluk. En iyi değer 1, standart belirsizlik u = 0,095 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 116 mg/kg
- genişletilmiş belirsizlik U = 26 mg/kg

Kaynak: Bölüm 6.7'deki ölçüm sonucu ve genişletilmiş belirsizlik (k = 2).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Yanlış kararların genel riskleri (İleri)

Üretim süreci bilindiğinde, kabul edilen bir ürünün gerçekte kötü olma olasılığı (müşteri riski) bütün üretim için hesaplanabilir ve gerekli bir risk için bir kabul sınırı seçilebilir.

<a id="jcgm106-9.5-ball-bearings"></a>

### Bilyalı rulmanlar: istenen tüketici riski için kabul sınırı (JCGM 106 9.5.4)

`jcgm106-9.5-ball-bearings`

#### Bu proje ne hakkında?

JCGM 106:2012 9.5.4 Örnek: radyal hata hareketi ≤ 2 µm, ortalaması 1 µm ve standart sapması 0,5 µm olan gama süreç dağılımı, ölçüm sistemi u_m = 0,25 µm; küresel tüketici riski en çok %0,1.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük radial error motion en çok 2 µm olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Karar kuralı: korumalı kabul: kabul bölgesi tolerans aralığından bir koruma bandı kadar daraltılır; böylece kötü bir ürünü kabul etmek olasılık dışı olur (müşteriyi korur).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 106:2012 9.5.4, Örnek (%4,2 uygunsuz, r ≈ 0,65, A ≈ 1,7 µm, R_P ≈ %7,5).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm106-9.5-precision-resistors"></a>

### Hassas dirençler: küresel tüketici ve üretici riskleri (JCGM 106 9.5.3)

`jcgm106-9.5-precision-resistors`

#### Bu proje ne hakkında?

JCGM 106:2012 9.5.3 Örnek: σ = 0,12 Ω olan bir süreçten 1500 Ω ± 0,2 Ω dirençler, u_m = 0,04 Ω ile ve 1499,82 Ω ile 1500,18 Ω kabul sınırlarıyla denetlenir.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük R 1499,8 ile 1500,2 arasında Ω olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 106:2012 9.5.3, Örnek (p_c ≈ %90, R_C ≈ %1, R_P ≈ %7).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Birkaç bileşen birlikte uymalı (İleri)

Dört korelasyonlu etken maddeli bir ilaç: yanlış karar riskleri Monte Carlo ile birlikte hesaplanır.

<a id="emue-e2.1-multicomponent-medication"></a>

### Dört ilişkili etken maddeli bir ilaç için toplam yanlış karar riskleri (EMUE E2.1)

`emue-e2.1-multicomponent-medication`

#### Bu proje ne hakkında?

NyQuil tabletleri: APAP, DEX, DOX ve PE; tolerans etiket miktarının %95–105'i, u_bağıl = %2,8; üç deney sonucu takımı için toplam özgül tüketici riskleri ve toplam küresel tüketici riski.


#### Adım 1. Fikir: birkaç özellik aynı anda uymalı

Bir ürün ancak 4 bileşenin hepsi sınırları içindeyse uygundur. Ölçülen değerler korelasyonlu olduğu için yanlış karar riskleri birlikte hesaplanır (Monte Carlo).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EMUE E2.1.6.2–E2.1.6.3: R*_tot = 0,0029, 0,0117, 0,0002; R_tot = 0,0018 (10⁷ deneme; özgün makalede 0,0019).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Önsel bilgiyle genel ve özgül riskler (İleri)

Havada partikül madde: tipik derişimler hakkındaki önsel bilgi (lognormal dağılım) bir kararın risklerini hesaplamak için ölçümle birleştirilir.

<a id="gum5-e10-tspm-global-risks-acceptance-0.17"></a>

### Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,17 mg/m³ (JCGM GUM-5 Örnek 10)

`gum5-e10-tspm-global-risks-acceptance-0.17`

#### Bu proje ne hakkında?

T_U = 0,2 mg/m³, lognormal popülasyon dağılımı, bağıl belirsizlik %7; kabul sınırı 0,17 mg/m³ değerine kaydırıldı.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük ρ en çok 0,2 mg/m³ olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM GUM-5:2026, 10.6.3.3–10.6.3.4 R_C = 0,002 %, R_P = 5,8 % basar; beklenen değerler koşullu ölçüm sistemi dağılımı içindir (F80).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e10-tspm-global-risks-acceptance-0.22"></a>

### Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,22 mg/m³ (JCGM GUM-5 Örnek 10)

`gum5-e10-tspm-global-risks-acceptance-0.22`

#### Bu proje ne hakkında?

T_U = 0,2 mg/m³, lognormal popülasyon dağılımı, bağıl belirsizlik %7; kabul sınırı 0,22 mg/m³ değerine kaydırıldı.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük ρ en çok 0,2 mg/m³ olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM GUM-5:2026, 10.6.3.3–10.6.3.4 R_C = 1,7 %, R_P = < 0,1 % basar; beklenen değerler koşullu ölçüm sistemi dağılımı içindir (F80).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e10-tspm-global-risks-shared-risk"></a>

### Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = 0,2 mg/m³ (JCGM GUM-5 Örnek 10)

`gum5-e10-tspm-global-risks-shared-risk`

#### Bu proje ne hakkında?

T_U = 0,2 mg/m³, lognormal popülasyon dağılımı, bağıl belirsizlik %7; paylaşılan risk kuralı A_U = T_U.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük ρ en çok 0,2 mg/m³ olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM GUM-5:2026, 10.6.3.3–10.6.3.4 R_C = 0,5 %, R_P = 0,9 % basar; beklenen değerler koşullu ölçüm sistemi dağılımı içindir (F80).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e10-tspm-specific-risks"></a>

### Havada askıda partikül madde: lognormal önselle sonsal dağılımdan özgül riskler (JCGM GUM-5 Örnek 10)

`gum5-e10-tspm-specific-risks`

#### Bu proje ne hakkında?

Tek sonuçların uygunluk olasılıkları, sonsal g(ρ | ρ̂) ∝ g₀(ρ)·h(ρ̂ | ρ), lognormal önsel, u = 0,07·ρ̂, A_U = T_U = 0,2 mg/m³.


#### Adım 1. Fikir: ürün spesifikasyona uyuyor mu?

Ölçülen büyüklük ρ en çok 0,2 mg/m³ olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).

Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM GUM-5:2026, 10.7.2: ρ̂ = 0,225 mg/m³ için özgül üretici riski yaklaşık %11 (Şekil 10.3).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Belirgin biçimde doğrusal olmayan bir model (İleri)

Mikrodalga gücünde karşılaştırma kaybı: sıfıra yakın model o kadar doğrusal değildir ki birinci mertebe GUM tamamen başarısız olur, Monte Carlo ise doğru cevabı verir. Türevler korelasyon ve yüksek mertebe terimler ekler.

<a id="jcgm101-9.4-comparison-loss-x000-correlated-higher-order"></a>

### Karşılaştırma kaybı, x₁ = 0, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri

`jcgm101-9.4-comparison-loss-x000-correlated-higher-order`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 50 10⁻⁶
- standart belirsizlik u = 67,2681 10⁻⁶

Kaynak: İki değişkenli normal girdiler için δY'nin analitik beklenen değeri ve standart sapması (GUM-5 Tablo 16.2, A sütunu: 50, 67·10⁻⁶); MCM birinci mertebe projedeki gibi.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.4-comparison-loss-x000-correlated"></a>

### Karşılaştırma kaybı, x₁ = 0,0, r = 0,9

`jcgm101-9.4-comparison-loss-x000-correlated`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- standart belirsizlik u = 0 10⁻⁶

Kaynak: JCGM 101:2008 Tablo 9 ve bağımsız 2 × 10⁸ denemeli referans; toleranslar M = 2 × 10⁶'da motor saçılımının ≈ 5 katı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.4-comparison-loss-x000"></a>

### Karşılaştırma kaybı, x₁ = 0,0

`jcgm101-9.4-comparison-loss-x000`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 50 10⁻⁶
- standart belirsizlik u = 50 10⁻⁶

Kaynak: JCGM 101:2008 Tablo 8 ve analitik merkezî olmayan χ² çözümü (Ek F); toleranslar M = 2 × 10⁶'da motor saçılımının ≈ 5 katı. GUM kestirimi analitik değerdir; JCGM 100:2008/Amd.1:2026'nın ikinci mertebe terimi bunu verir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.4-comparison-loss-x010-correlated-higher-order"></a>

### Karşılaştırma kaybı, x₁ = 0,01, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri

`jcgm101-9.4-comparison-loss-x010-correlated-higher-order`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0,01, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 150 10⁻⁶
- standart belirsizlik u = 120,5197 10⁻⁶

Kaynak: İki değişkenli normal girdiler için δY'nin analitik beklenen değeri ve standart sapması (GUM-5 Tablo 16.2, A sütunu: 150, 121·10⁻⁶); MCM birinci mertebe projedeki gibi.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.4-comparison-loss-x010-correlated"></a>

### Karşılaştırma kaybı, x₁ = 0,01, r = 0,9

`jcgm101-9.4-comparison-loss-x010-correlated`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0,01, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- standart belirsizlik u = 100 10⁻⁶

Kaynak: JCGM 101:2008 Tablo 9 ve bağımsız 2 × 10⁸ denemeli referans; toleranslar M = 2 × 10⁶'da motor saçılımının ≈ 5 katı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.4-comparison-loss-x010"></a>

### Karşılaştırma kaybı, x₁ = 0,01

`jcgm101-9.4-comparison-loss-x010`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0,01, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 150 10⁻⁶
- standart belirsizlik u = 112 10⁻⁶

Kaynak: JCGM 101:2008 Tablo 8 ve analitik merkezî olmayan χ² çözümü (Ek F); toleranslar M = 2 × 10⁶'da motor saçılımının ≈ 5 katı. GUM kestirimi analitik değerdir; JCGM 100:2008/Amd.1:2026'nın ikinci mertebe terimi bunu verir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.4-comparison-loss-x050-correlated-higher-order"></a>

### Karşılaştırma kaybı, x₁ = 0,05, r = 0,9, ilişkili girdiler için yüksek mertebe GUM terimleri

`jcgm101-9.4-comparison-loss-x050-correlated-higher-order`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0,05, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 2550,0 10⁻⁶
- standart belirsizlik u = 504,5047 10⁻⁶

Kaynak: İki değişkenli normal girdiler için δY'nin analitik beklenen değeri ve standart sapması (GUM-5 Tablo 16.2, A sütunu: 2550, 505·10⁻⁶); MCM birinci mertebe projedeki gibi.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.4-comparison-loss-x050-correlated"></a>

### Karşılaştırma kaybı, x₁ = 0,05, r = 0,9

`jcgm101-9.4-comparison-loss-x050-correlated`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0,05, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- standart belirsizlik u = 500 10⁻⁶

Kaynak: JCGM 101:2008 Tablo 9 ve bağımsız 2 × 10⁸ denemeli referans; toleranslar M = 2 × 10⁶'da motor saçılımının ≈ 5 katı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.4-comparison-loss-x050"></a>

### Karşılaştırma kaybı, x₁ = 0,05

`jcgm101-9.4-comparison-loss-x050`

#### Bu proje ne hakkında?

JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dY** (10⁻⁶): Karşılaştırma kaybı. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dY = 1e6 * (X1^2 + X2^2)
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**: Yansıma katsayısının gerçel kısmı. En iyi değer 0,05, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**: Yansıma katsayısının sanal kısmı. En iyi değer 0, standart belirsizlik u = 0,005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.
6. Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 2 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 2 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 2550 10⁻⁶
- standart belirsizlik u = 502 10⁻⁶

Kaynak: JCGM 101:2008 Tablo 8 ve analitik merkezî olmayan χ² çözümü (Ek F); toleranslar M = 2 × 10⁶'da motor saçılımının ≈ 5 katı. GUM kestirimi analitik değerdir; JCGM 100:2008/Amd.1:2026'nın ikinci mertebe terimi bunu verir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Mastar blok: tam ve yaklaşık modeller (İleri)

JCGM 101'in tam doğrusal olmayan mastar blok modeli, doğrusal yaklaşımıyla GUM ve Monte Carlo üzerinden karşılaştırılır.

<a id="jcgm101-9.5-gauge-block-approximate"></a>

### Mastar kalibrasyonu, yaklaşık model

`jcgm101-9.5-gauge-block-approximate`

#### Bu proje ne hakkında?

JCGM 101 9.5: girdilere JCGM 101'in atadığı dağılımlarla (t, ark sinüs, eğrisel yamuk) GUM H.1 mastarı; %99 kapsam.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dL** (nm): Mastar boyunun 50 mm'den sapması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dL = Ls + D + d1 + d2 - Ls*(d_alpha*(theta0 + Delta) + alpha_s*d_theta) - 50000000
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **Ls**: Standardın 20 °C'deki boyu. En iyi değer 50000623 nm, standart belirsizlik u = 25,0 nm. Az sayıda veriye dayandığı için (ν = 18 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
2. **D**: Beş göstergenin ortalama farkı. En iyi değer 215 nm, standart belirsizlik u = 6,00 nm. Az sayıda veriye dayandığı için (ν = 24 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
3. **d1**: Komparatörün rastgele etkileri. En iyi değer 0 nm, standart belirsizlik u = 4,00 nm. Az sayıda veriye dayandığı için (ν = 5 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
4. **d2**: Komparatörün sistematik etkileri. En iyi değer 0 nm, standart belirsizlik u = 7,00 nm. Az sayıda veriye dayandığı için (ν = 8 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
5. **alpha_s**: Standardın ısıl genleşme katsayısı. Yalnızca sınırlar biliniyor: değer 0,0000095 ile 0,0000135 1/°C arasında (0,0000115 ± 2,0000 × 10⁻⁶) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2,0000 × 10⁻⁶/√3 = 1,15 × 10⁻⁶ 1/°C.
6. **theta0**: Ortalama sıcaklık sapması. En iyi değer -0,1 °C, standart belirsizlik u = 0,2 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **Delta**: Periyodik sıcaklık değişimi. Değer -0,5 ile 0,5 °C arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,5/√2 = 0,354 °C.
8. **d_alpha**: Genleşme katsayıları farkı (sınırlar ±%10 belirsiz). Değer -0,000001 ile 0,000001 1/°C arasında, ancak sınırların kendisi de ± 1e-7 belirsiz (eğrisel yamuk, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = 5,78 × 10⁻⁷ 1/°C. Bu standart belirsizliğin kendisi de yaklaşık bilinir: 50 serbestlik derecesi vardır.
9. **d_theta**: Mastarlar arası sıcaklık farkı (sınırlar ±%50 belirsiz). Değer -0,05 ile 0,05 °C arasında, ancak sınırların kendisi de ± 0,025 belirsiz (eğrisel yamuk, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = 0,0300 °C. Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %99,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 838 nm
- standart belirsizlik u = 32 nm
- kapsam faktörü k = 2,92

Kaynak: JCGM 101:2008 Tablo 11 (değerler 1 nm'ye yuvarlanmış).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm101-9.5-gauge-block-nonlinear"></a>

### Mastar kalibrasyonu, doğrusal olmayan model

`jcgm101-9.5-gauge-block-nonlinear`

#### Bu proje ne hakkında?

JCGM 101 9.5: girdilere JCGM 101'in atadığı dağılımlarla (t, ark sinüs, eğrisel yamuk) GUM H.1 mastarı; %99 kapsam.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **dL** (nm): Mastar boyunun 50 mm'den sapması. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
dL = (Ls*(1 + alpha_s*(theta0 + Delta - d_theta)) + D + d1 + d2) / (1 + (alpha_s + d_alpha)*(theta0 + Delta)) - 50000000
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **Ls**: Standardın 20 °C'deki boyu. En iyi değer 50000623 nm, standart belirsizlik u = 25,0 nm. Az sayıda veriye dayandığı için (ν = 18 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
2. **D**: Beş göstergenin ortalama farkı. En iyi değer 215 nm, standart belirsizlik u = 6,00 nm. Az sayıda veriye dayandığı için (ν = 24 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
3. **d1**: Komparatörün rastgele etkileri. En iyi değer 0 nm, standart belirsizlik u = 4,00 nm. Az sayıda veriye dayandığı için (ν = 5 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
4. **d2**: Komparatörün sistematik etkileri. En iyi değer 0 nm, standart belirsizlik u = 7,00 nm. Az sayıda veriye dayandığı için (ν = 8 serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.
5. **alpha_s**: Standardın ısıl genleşme katsayısı. Yalnızca sınırlar biliniyor: değer 0,0000095 ile 0,0000135 1/°C arasında (0,0000115 ± 2,0000 × 10⁻⁶) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 2,0000 × 10⁻⁶/√3 = 1,15 × 10⁻⁶ 1/°C.
6. **theta0**: Ortalama sıcaklık sapması. En iyi değer -0,1 °C, standart belirsizlik u = 0,2 °C (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **Delta**: Periyodik sıcaklık değişimi. Değer -0,5 ile 0,5 °C arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = 0,5/√2 = 0,354 °C.
8. **d_alpha**: Genleşme katsayıları farkı (sınırlar ±%10 belirsiz). Değer -0,000001 ile 0,000001 1/°C arasında, ancak sınırların kendisi de ± 1e-7 belirsiz (eğrisel yamuk, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = 5,78 × 10⁻⁷ 1/°C. Bu standart belirsizliğin kendisi de yaklaşık bilinir: 50 serbestlik derecesi vardır.
9. **d_theta**: Mastarlar arası sıcaklık farkı (sınırlar ±%50 belirsiz). Değer -0,05 ile 0,05 °C arasında, ancak sınırların kendisi de ± 0,025 belirsiz (eğrisel yamuk, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = 0,0300 °C. Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %99,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 838 nm
- standart belirsizlik u = 32 nm
- kapsam faktörü k = 2,92

Kaynak: JCGM 101:2008 Tablo 11 (değerler 1 nm'ye yuvarlanmış).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Kartezyen koordinattan kutupsal koordinata (İleri)

İki koordinattan bir büyüklük ve bir faz. Merkeze yakın dönüşüm belirgin biçimde doğrusal değildir ve kapsam bölgesi bir elips değildir.

<a id="jcgm102-9.3-polar-x001-correlated"></a>

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,001, r = 0,9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x001-correlated`

#### Bu proje ne hakkında?

JCGM 102:2011 9.3: x₁ = 0,001, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 7).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0,001, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 7, x₁ = 0,001 (M = 10⁷ ile MCM; toleranslar 10⁶ deneme için Monte Carlo saçılımını içerir).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.3-polar-x001"></a>

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,001 (JCGM 102 9.3)

`jcgm102-9.3-polar-x001`

#### Bu proje ne hakkında?

JCGM 102:2011 9.3: x₁ = 0,001, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 6).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0,001, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 6, x₁ = 0,001 (M = 10⁷ ile MCM; toleranslar 10⁶ deneme için Monte Carlo saçılımını içerir).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.3-polar-x010-correlated"></a>

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,01, r = 0,9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x010-correlated`

#### Bu proje ne hakkında?

JCGM 102:2011 9.3: x₁ = 0,01, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 7).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0,01, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 7, x₁ = 0,01 (M = 10⁷ ile MCM; toleranslar 10⁶ deneme için Monte Carlo saçılımını içerir).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.3-polar-x010"></a>

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,01 (JCGM 102 9.3)

`jcgm102-9.3-polar-x010`

#### Bu proje ne hakkında?

JCGM 102:2011 9.3: x₁ = 0,01, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 6).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0,01, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 6, x₁ = 0,01 (M = 10⁷ ile MCM; toleranslar 10⁶ deneme için Monte Carlo saçılımını içerir).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.3-polar-x100-correlated"></a>

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,1, r = 0,9 (JCGM 102 9.3)

`jcgm102-9.3-polar-x100-correlated`

#### Bu proje ne hakkında?

JCGM 102:2011 9.3: x₁ = 0,1, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 7).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0,1, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- X1 ↔ X2: r = 0,9


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 7, x₁ = 0,1 (M = 10⁷ ile MCM; toleranslar 10⁶ deneme için Monte Carlo saçılımını içerir).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.3-polar-x100"></a>

### Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = 0,1 (JCGM 102 9.3)

`jcgm102-9.3-polar-x100`

#### Bu proje ne hakkında?

JCGM 102:2011 9.3: x₁ = 0,1, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması (Tablo 6).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 2 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **R**: `R = sqrt(X1^2 + X2^2)`
- **Theta** rad: `Theta = atan2(X2, X1)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 2 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **X1**. En iyi değer 0,1, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **X2**. En iyi değer 0, standart belirsizlik u = 0,01 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 6, x₁ = 0,1 (M = 10⁷ ile MCM; toleranslar 10⁶ deneme için Monte Carlo saçılımını içerir).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Eşzamanlı verilerden direnç ve reaktans (İleri)

Korelasyonlu gösterge değerlerinden üç çıktı; alternatif sürüm az veri için çok değişkenli bir t dağılımı kullanır.

<a id="jcgm102-9.4-resistance-reactance-alternative"></a>

### Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans, t kovaryansı (JCGM 102 9.4)

`jcgm102-9.4-resistance-reactance-alternative`

#### Bu proje ne hakkında?

JCGM 102:2011 9.4 (GUM H.2'nin genişletilmesi): korelasyonlu ortalamalardan üç çıktı büyüklüğü; çok değişkenli t dağılımının kovaryans matrisi ile GUM yaklaşımı ve çok değişkenli t dağılımıyla Monte Carlo yöntemi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 3 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **R** Ω: `R = V/(I*1e-3)*cos(phi)`
- **X** Ω: `X = V/(I*1e-3)*sin(phi)`
- **Z** Ω: `Z = V/(I*1e-3)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 0 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:



#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 11, Alt. GUF ve MCM satırları (M = 10⁶).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.4-resistance-reactance"></a>

### Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans (JCGM 102 9.4)

`jcgm102-9.4-resistance-reactance`

#### Bu proje ne hakkında?

JCGM 102:2011 9.4 (GUM H.2'nin genişletilmesi): korelasyonlu ortalamalardan üç çıktı büyüklüğü; U_x = M/(n(n − 1)) ile GUM yaklaşımı ve çok değişkenli t dağılımıyla Monte Carlo yöntemi.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 3 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **R** Ω: `R = V/(I*1e-3)*cos(phi)`
- **X** Ω: `X = V/(I*1e-3)*sin(phi)`
- **Z** Ω: `Z = V/(I*1e-3)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 0 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:



#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 11, GUF ve MCM satırları (M = 10⁶).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Kapalı modeller: bir direnç termometresi (İleri)

Sıcaklık direncin bir formülü olarak yazılamaz, bu yüzden denklem sayısal olarak çözülür. Bir termometreden on sıcaklıkla sonuçlar güçlü biçimde korelasyonludur.

<a id="jcgm102-9.5.2-resistance-thermometer"></a>

### Direnç termometresiyle Celsius sıcaklığı, örtük model (JCGM 102 9.5.2)

`jcgm102-9.5.2-resistance-thermometer`

#### Bu proje ne hakkında?

JCGM 102:2011 9.5.2: termometrenin korelasyonlu kalibrasyon sabitleriyle ölçülen bir direnç oranından tek bir sıcaklık.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 1 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **theta** °C: `theta = R0*(1 + A*theta + B*theta^2) - r*RS`

Model *kapalıdır*: denklemler sonuçlara göre çözülmüş değildir; program onları sayısal olarak çözer.


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 5 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R0**. En iyi değer 99,9961 Ω, standart belirsizlik u = 0,0005 Ω (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **A**. En iyi değer 0,0039096 °C⁻¹, standart belirsizlik u = 0,0000027 °C⁻¹ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **B**. En iyi değer -6e-7 °C⁻², standart belirsizlik u = 1,1e-7 °C⁻² (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **RS**. En iyi değer 99,99947 Ω, standart belirsizlik u = 0,0001 Ω (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **r**. En iyi değer 1,0780057, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- R0 ↔ A: r = -0,155
- R0 ↔ B: r = 0,092
- A ↔ B: r = -0,959


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 9.5.2.5 ve 9.5.2.6 (θ = 20,023 2 °C, u = 0,004 5 °C).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="jcgm102-9.5.3-ten-temperatures"></a>

### Tek direnç termometresiyle on Celsius sıcaklığı, örtük model (JCGM 102 9.5.3)

`jcgm102-9.5.3-ten-temperatures`

#### Bu proje ne hakkında?

JCGM 102:2011 9.5.3: aynı kalibre termometreyle ölçülen on sıcaklık, kalibrasyon sabitleri üzerinden koreledir (Tablo 15 ve 16).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 10 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **theta1** °C: `theta1 = R0*(1 + A*theta1 + B*theta1^2) - r1*RS`
- **theta2** °C: `theta2 = R0*(1 + A*theta2 + B*theta2^2) - r2*RS`
- **theta3** °C: `theta3 = R0*(1 + A*theta3 + B*theta3^2) - r3*RS`
- **theta4** °C: `theta4 = R0*(1 + A*theta4 + B*theta4^2) - r4*RS`
- **theta5** °C: `theta5 = R0*(1 + A*theta5 + B*theta5^2) - r5*RS`
- **theta6** °C: `theta6 = R0*(1 + A*theta6 + B*theta6^2) - r6*RS`
- **theta7** °C: `theta7 = R0*(1 + A*theta7 + B*theta7^2) - r7*RS`
- **theta8** °C: `theta8 = R0*(1 + A*theta8 + B*theta8^2) - r8*RS`
- **theta9** °C: `theta9 = R0*(1 + A*theta9 + B*theta9^2) - r9*RS`
- **theta10** °C: `theta10 = R0*(1 + A*theta10 + B*theta10^2) - r10*RS`

Model *kapalıdır*: denklemler sonuçlara göre çözülmüş değildir; program onları sayısal olarak çözer.


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 14 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R0**. En iyi değer 99,9961 Ω, standart belirsizlik u = 0,0005 Ω (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **A**. En iyi değer 0,0039096 °C⁻¹, standart belirsizlik u = 0,0000027 °C⁻¹ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **B**. En iyi değer -6e-7 °C⁻², standart belirsizlik u = 1,1e-7 °C⁻² (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **RS**. En iyi değer 99,99947 Ω, standart belirsizlik u = 0,0001 Ω (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **r1**. En iyi değer 1,0000053, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **r2**. En iyi değer 1,0150054, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **r3**. En iyi değer 1,0300055, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **r4**. En iyi değer 1,0450056, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **r5**. En iyi değer 1,0600056, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **r6**. En iyi değer 1,0780057, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
11. **r7**. En iyi değer 1,0900058, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
12. **r8**. En iyi değer 1,1050059, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
13. **r9**. En iyi değer 1,120006, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
14. **r10**. En iyi değer 1,0780057, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- R0 ↔ A: r = -0,155
- R0 ↔ B: r = 0,092
- A ↔ B: r = -0,959


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Kapsam faktörü k'yi %95,0 kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM 102:2011 Tablo 15 ve 16.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Aynı enjeksiyonlardan pik alanları (İleri)

İç standartlı GC-MS ile benzo[a]piren. Aynı enjeksiyonlarda ölçülen iki alan korelasyonludur; türev, serbestlik derecesini sayarken onları bir topluluk olarak ele alır.

<a id="gum5-e2-benzo-a-pyrene-ensemble"></a>

### Benzo[a]piren: aynı enjeksiyonların pik alanları ν_eff'te topluluk olarak (JCGM GUM-5 Örnek 2, Willink 2007)

`gum5-e2-benzo-a-pyrene-ensemble`

#### Bu proje ne hakkında?

Aynı bütçe, A_E ve A_ISE topluluk olarak tanımlanmış: ortak varyansları ν = 2 olan tek Welch–Satterthwaite bileşenidir; ν_eff 5,2 veya 4,8 yerine ≈ 2,1 olur.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **mE** (ng): 1 µL ekstrakttaki benzo[a]piren kütlesi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
mE = f*AE/AISE*mISE
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **f**: GC-MS kalibrasyon faktörü (üç faktörün ortalaması, ν = 9). En iyi değer 0,616, standart belirsizlik u = 0,017 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 9 serbestlik derecesi vardır.
2. **mISE**: 1 µL ekstrakttaki iç standart BaP-d12 kütlesi (sertifika). En iyi değer 0,2455 ng, standart belirsizlik u = 0,0036 ng (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **AE**: Üç enjeksiyonun ortalama BaP pik alanı. En iyi değer 85114 a.u., standart belirsizlik u = 9564 a.u. (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.
4. **AISE**: Aynı üç enjeksiyonun ortalama BaP-d12 pik alanı. En iyi değer 917546 a.u., standart belirsizlik u = 44492 a.u. (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- f ↔ mISE: r = -0,5392
- AE ↔ AISE: r = -0,4771


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,014 ng
- standart belirsizlik u = 0,00202 ng

Kaynak: JCGM GUM-5:2026 Örnek 2 bütçesi (u = 0,002 ng); ν_eff = 2,11 Willink (2007) topluluk kuralıyla yeniden hesaplandı, GUM-5'te basılı değildir.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e2-benzo-a-pyrene"></a>

### İç standartla GC-MS'te benzo[a]piren, ilişkili girdiler (JCGM GUM-5 Örnek 2)

`gum5-e2-benzo-a-pyrene`

#### Bu proje ne hakkında?

İki ilişkili girdi çiftli iç standart modeli; kovaryanslı LPU ve basıldığı gibi Welch–Satterthwaite ν_eff.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **mE** (ng): 1 µL ekstrakttaki benzo[a]piren kütlesi. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
mE = f*AE/AISE*mISE
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 4 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **f**: GC-MS kalibrasyon faktörü (üç faktörün ortalaması, ν = 9). En iyi değer 0,616, standart belirsizlik u = 0,017 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 9 serbestlik derecesi vardır.
2. **mISE**: 1 µL ekstrakttaki iç standart BaP-d12 kütlesi (sertifika). En iyi değer 0,2455 ng, standart belirsizlik u = 0,0036 ng (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **AE**: Üç enjeksiyonun ortalama BaP pik alanı. En iyi değer 85114 a.u., standart belirsizlik u = 9564 a.u. (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.
4. **AISE**: Aynı üç enjeksiyonun ortalama BaP-d12 pik alanı. En iyi değer 917546 a.u., standart belirsizlik u = 44492 a.u. (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.

**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:

- f ↔ mISE: r = -0,5392
- AE ↔ AISE: r = -0,4771


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,014 ng
- standart belirsizlik u = 0,00202 ng
- kapsam faktörü k = 2,5

Kaynak: JCGM GUM-5:2026, 2.4.1.2–2.4.1.4 ve Tablo 2.3: m_E = 0,014 ng, u = 0,002 ng (u² = 4,1·10⁻⁶ ng²), ν_eff = 5,2, k = 2,5.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Gravimetrik bir gaz karışımı (İleri)

Tartımla hazırlanmış bir kalibrasyon gaz karışımının sekiz miktar kesri: çok sayıda girdi, ara büyüklükler ve korelasyonlu çıktılar.

<a id="gum5-e4-gravimetric-gas-mixture"></a>

### N₂ içinde CO₂ kalibrasyon gazı karışımının gravimetrik hazırlanması: sekiz bileşenin madde miktarı kesirleri ve korelasyonları (JCGM GUM-5 Örnek 4)

`gum5-e4-gravimetric-gas-mixture`

#### Bu proje ne hakkında?

Üç tartım, iki ana gazın saflık tabloları ve IUPAC atom ağırlığı aralıklarından ISO 6142-1 modeli y_k = n_k/n_tot; çok değişkenli LPU (JCGM 102).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 8 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **y_Ar** µmol/mol: `y_Ar = 1e6*(m2*1e-6*x2_Ar/MP2)/ntot`
- **y_CH4** µmol/mol: `y_CH4 = 1e6*(m1*1e-6*x1_CH4/MP1 + m2*1e-6*x2_CH4/MP2)/ntot`
- **y_CO** µmol/mol: `y_CO = 1e6*(m1*1e-6*x1_CO/MP1 + m2*1e-6*x2_CO/MP2)/ntot`
- **y_CO2** µmol/mol: `y_CO2 = 1e6*(m1*x1_CO2/MP1 + m2*1e-6*x2_CO2/MP2)/ntot`
- **y_H2** µmol/mol: `y_H2 = 1e6*(m2*1e-6*x2_H2/MP2)/ntot`
- **y_H2O** µmol/mol: `y_H2O = 1e6*(m1*1e-6*x1_H2O/MP1 + m2*1e-6*x2_H2O/MP2)/ntot`
- **y_N2** µmol/mol: `y_N2 = 1e6*(m1*1e-6*x1_N2/MP1 + m2*x2_N2/MP2)/ntot`
- **y_O2** µmol/mol: `y_O2 = 1e6*(m1*1e-6*x1_O2/MP1 + m2*1e-6*x2_O2/MP2)/ntot`

Denklemi okunur tutmak için önce 15 ara büyüklük hesaplanır:

- `M_Ar = A_Ar`
- `M_CH4 = 4*A_H + A_C`
- `M_CO = A_C + A_O`
- `M_CO2 = A_C + 2*A_O`
- `M_H2 = 2*A_H`
- `M_H2O = 2*A_H + A_O`
- `M_N2 = 2*A_N`
- `M_O2 = 2*A_O`
- `x1_CO2 = 1 - 1e-6*(x1_CH4 + x1_CO + x1_H2O + x1_N2 + x1_O2)`
- `x2_N2 = 1 - 1e-6*(x2_Ar + x2_CH4 + x2_CO + x2_CO2 + x2_H2 + x2_H2O + x2_O2)`
- `MP1 = 1e-6*x1_CH4*M_CH4 + 1e-6*x1_CO*M_CO + x1_CO2*M_CO2 + 1e-6*x1_H2O*M_H2O + 1e-6*x1_N2*M_N2 + 1e-6*x1_O2*M_O2`
- `MP2 = 1e-6*x2_Ar*M_Ar + 1e-6*x2_CH4*M_CH4 + 1e-6*x2_CO*M_CO + 1e-6*x2_CO2*M_CO2 + 1e-6*x2_H2*M_H2 + 1e-6*x2_H2O*M_H2O + x2_N2*M_N2 + 1e-6*x2_O2*M_O2`
- `m1 = r1 - r0`
- `m2 = r2 - r1`
- `ntot = m1/MP1 + m2/MP2`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 20 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **r0**: Ortalama terazi farkı, numune − referans tüp, boşaltılmış. En iyi değer -152,0243 g, standart belirsizlik u = 0,0005 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **r1**: Ortalama terazi farkı, numune − referans tüp, 1. ana gazdan sonra. En iyi değer 93,4951 g, standart belirsizlik u = 0,0008 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **r2**: Ortalama terazi farkı, numune − referans tüp, 2. ana gazdan sonra. En iyi değer 717,274 g, standart belirsizlik u = 0,0007 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **A_H**: H standart atom ağırlığı (IUPAC aralığı). Yalnızca sınırlar biliniyor: değer 1,00784 ile 1,00811 arasında (1,007975 ± 0,00013500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00013500/√3 = 7,79 × 10⁻⁵.
5. **A_C**: C standart atom ağırlığı (IUPAC aralığı). Yalnızca sınırlar biliniyor: değer 12,0096 ile 12,0116 arasında (12,0106 ± 0,0010000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,0010000/√3 = 0,000577.
6. **A_N**: N standart atom ağırlığı (IUPAC aralığı). Yalnızca sınırlar biliniyor: değer 14,00643 ile 14,00728 arasında (14,006855 ± 0,00042500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00042500/√3 = 0,000245.
7. **A_O**: O standart atom ağırlığı (IUPAC aralığı). Yalnızca sınırlar biliniyor: değer 15,99903 ile 15,99977 arasında (15,9994 ± 0,00037000) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,00037000/√3 = 0,000214.
8. **A_Ar**: Ar standart atom ağırlığı (IUPAC aralığı). Yalnızca sınırlar biliniyor: değer 39,792 ile 39,963 arasında (39,8775 ± 0,085500) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = 0,085500/√3 = 0,0494.
9. **x1_CH4**: Karbon dioksit ana gazında CH4. En iyi değer 0,1 µmol/mol, standart belirsizlik u = 0,06 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **x1_CO**: Karbon dioksit ana gazında CO. En iyi değer 0,5 µmol/mol, standart belirsizlik u = 0,3 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
11. **x1_H2O**: Karbon dioksit ana gazında H2O. En iyi değer 1,9 µmol/mol, standart belirsizlik u = 0,1 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
12. **x1_N2**: Karbon dioksit ana gazında N2. En iyi değer 4,3 µmol/mol, standart belirsizlik u = 2,1 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
13. **x1_O2**: Karbon dioksit ana gazında O2. En iyi değer 5,3 µmol/mol, standart belirsizlik u = 2,6 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
14. **x2_Ar**: Azot ana gazında Ar. En iyi değer 5 µmol/mol, standart belirsizlik u = 3 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
15. **x2_CH4**: Azot ana gazında CH4. En iyi değer 0,008 µmol/mol, standart belirsizlik u = 0,005 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
16. **x2_CO**: Azot ana gazında CO. En iyi değer 0,015 µmol/mol, standart belirsizlik u = 0,009 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
17. **x2_CO2**: Azot ana gazında CO2. En iyi değer 0,01 µmol/mol, standart belirsizlik u = 0,006 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
18. **x2_H2**: Azot ana gazında H2. En iyi değer 0,025 µmol/mol, standart belirsizlik u = 0,015 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
19. **x2_H2O**: Azot ana gazında H2O. En iyi değer 0,01 µmol/mol, standart belirsizlik u = 0,006 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
20. **x2_O2**: Azot ana gazında O2. En iyi değer 0,005 µmol/mol, standart belirsizlik u = 0,003 µmol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM GUM-5:2026 Tablo 4.6 ve 4.3.6.9; beklenen değerler yeniden hesaplandı: Ar 4,0 ± 2,4, CH₄ 0,026 ± 0,013, CO 0,112 ± 0,061, H₂ 0,020 ± 0,012, O₂ 1,07 ± 0,52, N₂ 799 649 ± 5 µmol/mol basıldığı gibi; CO₂ 200 345 ± 4 (basılı ± 5), H₂O 0,389 (basılı 0,379); r(CO₂, N₂) −0,85 (−0,86), r(Ar, N₂) −0,55 (−0,50).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir tork sensörünün kalibrasyonu (İleri)

Sıradan ve ağırlıklı en küçük karelerle uydurulan, orijinden geçen bir doğru ve eğimin GUM belirsizliği.

<a id="gum5-e9-torque-slope-ols"></a>

### Tork sensörü kalibrasyonu, orijinden geçen doğru, GUF ile sıradan en küçük kareler (JCGM GUM-5 Örnek 9)

`gum5-e9-torque-slope-ols`

#### Bu proje ne hakkında?

Sekiz tork seviyesinden Y = βX'in eğimi β; SEKK kestiricisi (9.4) Y_i ortalamalarında doğrusaldır, bu nedenle LPU ve Welch–Satterthwaite doğrudan uygulanır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **beta**: Kalibrasyon doğrusunun eğimi Y = βX. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
beta = 0.002593756963*Y1 + 0.005161833164*Y2 + 0.007832632413*Y3 + 0.01286606177*Y4 + 0.02570644277*Y5 + 0.07704228603*Y6 + 0.1027487288*Y7 + 0.06429178769*Y8
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **Y1**: x = 0,101 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0055 N·m). En iyi değer 0,095 N·m, standart belirsizlik u = 0,0022454 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
2. **Y2**: x = 0,201 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0052 N·m). En iyi değer 0,1966 N·m, standart belirsizlik u = 0,0021229 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
3. **Y3**: x = 0,305 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0041 N·m). En iyi değer 0,3016 N·m, standart belirsizlik u = 0,0016738 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
4. **Y4**: x = 0,501 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0041 N·m). En iyi değer 0,4983 N·m, standart belirsizlik u = 0,0016738 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
5. **Y5**: x = 1,001 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0098 N·m). En iyi değer 1,0083 N·m, standart belirsizlik u = 0,0040008 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
6. **Y6**: x = 3,0 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0082 N·m). En iyi değer 3,0266 N·m, standart belirsizlik u = 0,0033476 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
7. **Y7**: x = 4,001 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0121 N·m). En iyi değer 4,0466 N·m, standart belirsizlik u = 0,0049398 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
8. **Y8**: x = 5,007 N·m'de ortalama sensör yanıtı (3 okuma, s = 0,0379 N·m). En iyi değer 5,0666 N·m, standart belirsizlik u = 0,021882 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,0107
- standart belirsizlik u = 0,0015
- genişletilmiş belirsizlik U = 0,00515

Kaynak: JCGM GUM-5:2026 Tablo 9.3, OLS: β = 1,0107, u = 0,0015, [1,0055; 1,0158]; 9.5.1.4'te ν_eff = 2,7.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="gum5-e9-torque-slope-wls"></a>

### Tork sensörü kalibrasyonu, orijinden geçen doğru, GUF ile ağırlıklı en küçük kareler (JCGM GUM-5 Örnek 9)

`gum5-e9-torque-slope-wls`

#### Bu proje ne hakkında?

Sekiz tork seviyesinden Y = βX'in eğimi β; AEKK kestiricisi (9.4) Y_i ortalamalarında doğrusaldır, bu nedenle LPU ve Welch–Satterthwaite doğrudan uygulanır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **beta**: Kalibrasyon doğrusunun eğimi Y = βX. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
beta = 0.01172995665*Y1 + 0.02611498511*Y2 + 0.06374300572*Y3 + 0.1047057242*Y4 + 0.03661696445*Y5 + 0.156745096*Y6 + 0.09600596629*Y7 + 0.006123075128*Y8
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **Y1**: x = 0,101 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0055 N·m). En iyi değer 0,095 N·m, standart belirsizlik u = 0,0022454 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
2. **Y2**: x = 0,201 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0052 N·m). En iyi değer 0,1966 N·m, standart belirsizlik u = 0,0021229 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
3. **Y3**: x = 0,305 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0041 N·m). En iyi değer 0,3016 N·m, standart belirsizlik u = 0,0016738 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
4. **Y4**: x = 0,501 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0041 N·m). En iyi değer 0,4983 N·m, standart belirsizlik u = 0,0016738 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
5. **Y5**: x = 1,001 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0098 N·m). En iyi değer 1,0083 N·m, standart belirsizlik u = 0,0040008 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
6. **Y6**: x = 3,0 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0082 N·m). En iyi değer 3,0266 N·m, standart belirsizlik u = 0,0033476 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
7. **Y7**: x = 4,001 N·m'de ortalama sensör yanıtı (6 okuma, s = 0,0121 N·m). En iyi değer 4,0466 N·m, standart belirsizlik u = 0,0049398 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 5 serbestlik derecesi vardır.
8. **Y8**: x = 5,007 N·m'de ortalama sensör yanıtı (3 okuma, s = 0,0379 N·m). En iyi değer 5,0666 N·m, standart belirsizlik u = 0,021882 N·m (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir). Bu standart belirsizliğin kendisi de yaklaşık bilinir: 2 serbestlik derecesi vardır.


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 1,0085
- standart belirsizlik u = 0,0008
- genişletilmiş belirsizlik U = 0,0016

Kaynak: JCGM GUM-5:2026 Tablo 9.3, WLS: β = 1,0085, u = 0,0008, [1,0069; 1,0101]; 9.5.1.4'te ν_eff = 14; yeniden hesaplanan 13,31 (F83).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bayes ile şişeler arası homojenlik (İleri)

Şişeler arası varyans küçük olduğunda klasik tahmin sıfır ya da negatif çıkabilir; hiyerarşik bir Bayesçi model anlamlı bir değer verir.

<a id="gum5-e12-between-bottle-homogeneity-bayes"></a>

### Doğal gaz karışımlarında azotun şişeler arası homojenliği: ANOVA, u*_bb ve Bayesçi hiyerarşik model (JCGM GUM-5 Örnek 12)

`gum5-e12-between-bottle-homogeneity-bayes`

#### Bu proje ne hakkında?

Her biri beş kez ölçülen on gaz karışımı; MS_arası < MS_içi olduğundan ANOVA şişeler arası standart sapma vermez. Yarı-Cauchy önselli Bayesçi model τ için bir sonsal dağılım verir.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Homojenlik**: malzemenin birkaç birimi tekrarlı ölçülür. Varyans analizi birimler arası değişimi ölçümün tekrarlanabilirliğinden ayırır; birimler arası standart sapma σ_pt'ye göre küçük olmalıdır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: JCGM GUM-5:2026, Tablo 12.2 (MS 3,25·10⁻⁷ ve 4,90·10⁻⁷), 12.4.5 (τ 0,00014 ± 0,00011, [0,00001; 0,00040]; σ 0,00069; µ 0,42514), Tablo 12.3 (HPD) ve 12.4.8 (medyan 1,1 µmol/mol); u*_bb, 10⁻⁶'ya kadar medyan ve mod 0 burada yeniden hesaplanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir toprak referans malzemesinin homojenliği (İleri)

Üçer kez ölçülen yirmi şişe; varyans analiziyle ve hiyerarşik bir Bayesçi modelle değerlendirilir.

<a id="possolo-bodnar-2018-chromium-soil-homogeneity-bayes"></a>

### Bir toprak referans malzemesinde krom, üçer tekrarlı 20 şişe: ANOVA ve hiyerarşik Bayes modeli (Possolo & Bodnar 2018; ISO Guide 35 C.1 verileri)

`possolo-bodnar-2018-chromium-soil-homogeneity-bayes`

#### Bu proje ne hakkında?

w_ij = ω + β_j + ε_ij; şişe etkileri β_j ~ N(0, τ²), hatalar ε_ij ~ N(0, σ²). Malzemeye atanan değer ω'nın kestirimidir; belirsizliği şişeler arası ve şişe içi değişkenliği içerir.


#### Adım 1. Fikir: laboratuvarları karşılaştırmak

**Homojenlik**: malzemenin birkaç birimi tekrarlı ölçülür. Varyans analizi birimler arası değişimi ölçümün tekrarlanabilirliğinden ayırır; birimler arası standart sapma σ_pt'ye göre küçük olmalıdır.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Possolo & Bodnar (2018), §4.4 ve Tablo 4: REML 121,62 (0,95) mg/kg, τ 3,93 ve σ 2,87 mg/kg (= ISO Guide 35 s_bb ve s_r); Bayes (MCMC) 121,62 (1) mg/kg, Laplace 121,62 (0,98) mg/kg.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## İzotop oranlarından atom ağırlığı (İleri)

Üç ölçülen izotop oranından kurşunun atom ağırlığı ve izotop bileşimi: doğrusal olmayan bir modelden korelasyonlu çıktılar.

<a id="possolo-meija-2022-atomic-weight-lead-hipb1"></a>

### Üç izotop oranından NRC HIPB-1'deki kurşunun atom ağırlığı ve izotop bileşimi (Possolo & Meija 2022)

`possolo-meija-2022-atomic-weight-lead-hipb1`

#### Bu proje ne hakkında?

Tong vd. (2019) yüksek saflıktaki kurşun HIPB-1'de ²⁰⁴Pb/²⁰⁷Pb, ²⁰⁶Pb/²⁰⁷Pb ve ²⁰⁸Pb/²⁰⁷Pb oranlarını ölçtü. Miktar kesirleri oranlardan ve Σx = 1 kapanışından çıkar; A_r(Pb) izotop kütlelerinin bu kesirlerle ağırlıklı ortalamasıdır.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 5 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **x204** mol/mol: `x204 = R4/(1 + R4 + R6 + R8)`
- **x206** mol/mol: `x206 = R6/(1 + R4 + R6 + R8)`
- **x207** mol/mol: `x207 = 1/(1 + R4 + R6 + R8)`
- **x208** mol/mol: `x208 = R8/(1 + R4 + R6 + R8)`
- **ArPb** 1: `ArPb = (A204*R4 + A206*R6 + A207 + A208*R8)/(1 + R4 + R6 + R8)`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 7 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R4**: İzotop oranı n(²⁰⁴Pb)/n(²⁰⁷Pb), MC-ICP-MS (U95 % = 0,000 06, k = 2). En iyi değer 0,063 mol/mol, standart belirsizlik u = 0,00003 mol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **R6**: İzotop oranı n(²⁰⁶Pb)/n(²⁰⁷Pb) (U95 % = 0,0004). En iyi değer 1,3314 mol/mol, standart belirsizlik u = 0,0002 mol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **R8**: İzotop oranı n(²⁰⁸Pb)/n(²⁰⁷Pb) (U95 % = 0,0008). En iyi değer 2,5193 mol/mol, standart belirsizlik u = 0,0004 mol/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **A204**: ²⁰⁴Pb'nin bağıl atom kütlesi (AME 2020). En iyi değer 203,97 1, standart belirsizlik u = 0,0000012 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **A206**: ²⁰⁶Pb'nin bağıl atom kütlesi (AME 2020). En iyi değer 205,97 1, standart belirsizlik u = 0,0000012 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **A207**: ²⁰⁷Pb'nin bağıl atom kütlesi (AME 2020). En iyi değer 206,98 1, standart belirsizlik u = 0,0000012 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **A208**: ²⁰⁸Pb'nin bağıl atom kütlesi (AME 2020). En iyi değer 207,976652 1, standart belirsizlik u = 0,0000012 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 200 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 200 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Possolo & Meija (2022), s. 29: u(x) = 0,000 006, 0,000 037, 0,000 019, 0,000 045 mol/mol; korelasyonlar −0,02, +0,13, +0,18, −0,17, −0,90, −0,58 (Monte Carlo, 5 × 10⁴); A_r(Pb) = 207,1791, U95 % = 0,0002 (u = U/2; korelasyon toleransı: Monte Carlo hatası).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Kalın kuyruklu bir uzlaşı değeri (İleri)

Hubble–Lemaître sabitinin yirmi iki tutarsız tahmini, laboratuvar etkileri Laplace dağılımına uyan ve aykırı değerlerden daha az etkilenen bir Bayesçi modelle birleştirilir.

<a id="possolo-meija-2022-hubble-lemaitre-constant-laplace"></a>

### Birbiriyle tutarsız 22 kestirimden Hubble–Lemaître sabiti: Laplace etkili Bayesçi uzlaşı (Possolo & Meija 2022)

`possolo-meija-2022-hubble-lemaitre-constant-laplace`

#### Bu proje ne hakkında?

H₀'ın son kestirimleri (Planck hariç) belirsizliklerinin öngördüğünden daha fazla dağılır. NIST Decision Tree'nin önerdiği gibi Laplace deney etkileri λ_j (standart sapma τ) ve bildirilen u_j'li normal hatalarla H₀,j = H₀ + λ_j + ε_j modeli bunları bir uzlaşı değerinde birleştirir.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

22 laboratuvar ya da deney **H₀** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Possolo & Meija (2022), s. 150: sonsal ortalama 71,69 (km/s)/Mpc, standart sapma 0,66 (km/s)/Mpc, τ ortancası 2,03 (km/s)/Mpc (Stan). numpy ızgara (2000 × 1500): 71,6915, 0,6603, 2,0336.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Kapalı bir kimyasal model (İleri)

Dönüm noktası kimyasıyla Mohr titrasyonu: çözünürlük dengesi programın çözdüğü kapalı bir denklem verir.

<a id="iupac2024-3a-argentometric-titration-ksp"></a>

### Dönüm noktası kimyasıyla arjantometrik titrasyon (Mohr): AgCl ve Ag₂CrO₄ çözünürlük çarpımlı örtük model (IUPAC 2024, Örnek 3A ve 4I)

`iupac2024-3a-argentometric-titration-ksp`

#### Bu proje ne hakkında?

Dönüm noktasında [Ag⁺] = √(K_sp2/[CrO₄²⁻]) ve [Cl⁻] = K_sp1/[Ag⁺]; gümüş ve klorürün kütle dengesi c(NaCl) için sayısal olarak çözülen bir denklem verir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Bu projede aynı girdilerden birlikte hesaplanan 1 sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:

- **c_NaCl** mmol/L: `c_NaCl = 1000*(sqrt(Ksp2*((V_NaCl + dV_NaCl) + (V_Ag + dV_Ag + dV_E))/(c_K*(V_NaCl + dV_NaCl))) - Ksp1/sqrt(Ksp2*((V_NaCl + dV_NaCl) + (V_Ag + dV_Ag + dV_E))/(c_K*(V_NaCl + dV_NaCl)))) - (c_Ag*(V_Ag + dV_Ag + dV_E) - c_NaCl*(V_NaCl + dV_NaCl))/((V_NaCl + dV_NaCl) + (V_Ag + dV_Ag + dV_E))`

Model *kapalıdır*: denklemler sonuçlara göre çözülmüş değildir; program onları sayısal olarak çözer.


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 9 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **c_Ag**: AgNO₃ titrantının derişimi (kesin alınmıştır). Tam bilinen bir değer, 100 mmol/L; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
2. **V_Ag**: Dönüm noktasında okunan titrant hacmi. Tam bilinen bir değer, 10 mL; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
3. **dV_Ag**: Elektronik büretin çözünürlüğü, ±0,01 mL. Değer -0,01 ile 0,01 mL arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,01/√6 = 0,00408 mL.
4. **dV_E**: Görsel dönüm noktası tespiti, %95'te bir damla. En iyi değer 0 mL, standart belirsizlik u = 0,025 mL (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **V_NaCl**: Numunenin nominal hacmi. Tam bilinen bir değer, 100 mL; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
6. **dV_NaCl**: A sınıfı tam pipet, taban genişliği 0,16 mL üçgen. Değer -0,08 ile 0,08 mL arasında ve merkez 0'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = 0,08/√6 = 0,0327 mL.
7. **c_K**: Titrasyondan önce numunedeki K₂CrO₄ indikatörünün derişimi. Tam bilinen bir değer, 0,00175 mol/L; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
8. **Ksp1**: AgCl çözünürlük çarpımı, lg K = −9,75. Tam bilinen bir değer, 1,7783 × 10⁻¹⁰ (mol/L)²; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).
9. **Ksp2**: Ag₂CrO₄ çözünürlük çarpımı, lg K = −11,9. Tam bilinen bir değer, 1,2589 × 10⁻¹² (mol/L)³; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: IUPAC 2024, Örnek 4I (Meija vd., Eş. 19'dan): klasik tahminden bağıl fark %0,24; lg K_sp1 = −9,75, lg K_sp2 = −11,9.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Üç tutarsız belirleme (İleri)

Birbiriyle uyuşmayan üç belirlemeden iridyumun molar kütlesi. Laboratuvar etkileri hiyerarşik bir Bayesçi modelle modellenir.

<a id="iupac2024-3e-4m-iridium-molar-mass-bayes"></a>

### Birbiriyle tutarsız üç tayinden iridyumun mol kütlesi: laboratuvar etkileri ve Bayesçi rastgele etkiler modeli (IUPAC 2024, Örnek 3E, 4M ve 4N)

`iupac2024-3e-4m-iridium-molar-mass-bayes`

#### Bu proje ne hakkında?

İridyumun 2021 standart atom ağırlığının (Prohaska vd.) dayandığı üç izotop oranı tayini M = 192,216 77(81), 192,216 62(29) ve 192,217 63(17) g/mol verir. Standart sapmaları belirsizliklerin geometrik ortalamasının 1,6 katıdır; Cochran Q testi tutarsızlığı doğrular. Standart sapması τ olan laboratuvar etkileri bunları uzlaştırır.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

3 laboratuvar ya da deney **M(Ir)** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: IUPAC 2024, Örnek 3E: p = 0,0082; Örnek 4N: M = 192,2171(4) g/mol, τ ortancası 0,0005 g/mol (MCMC). numpy ızgara: 192,217 119, 0,000 440, τ ortancası 0,000 506.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir anahtar karşılaştırma (İleri)

CCQM-K145, sığır karaciğerinde nikel: en çok olabilirlikle karanlık belirsizlikli bir uzlaşı değeri. Katılımcılar kodlanmıştır.

<a id="iupac2024-4f-ccqm-k145-nickel-bovine-liver"></a>

### CCQM-K145 anahtar karşılaştırması, sığır karaciğerinde nikel: en çok olabilirlikle karanlık belirsizlikli uzlaşı değeri (IUPAC 2024, Örnek 4F)

`iupac2024-4f-ccqm-k145-nickel-bovine-liver`

#### Bu proje ne hakkında?

17 ulusal metroloji enstitüsü sığır karaciğerindeki nikel kütle kesrini ölçtü. Sonuçlar beyan edilen belirsizliklerin öngördüğünden daha fazla dağılır; x_j = μ + λ_j + ε_j, λ_j ~ N(0, τ²) laboratuvar etkileri modeli karanlık belirsizliği (dark uncertainty: laboratuvarlar arasında beyan edilen belirsizliklerle açıklanamayan fazla değişkenlik, τ) tanır.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

17 laboratuvar ya da deney **w(Ni)** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: IUPAC 2024, Örnek 4F: μ̂ = 2,043(15) mg/kg ve τ̂ = 0,044(14) mg/kg (scipy: 2,042939, 0,015442; 0,043541, 0,014239).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Tutarsız verilerin uzlaşı değerleri (İleri)

Yarı ömürler ve kütle çekim sabiti: belirsizliklerinden çok daha fazla birbirinden ayrılan ölçümler. DerSimonian–Laird, Linear Pool ve hiyerarşik Bayes farklı cevaplar verir; seçim değerlendirmenin bir parçasıdır.

<a id="koepke2017-half-life-cs137"></a>

### ¹³⁷Cs yarı ömrü: birbiriyle tutarsız 19 ölçümün uzlaşı değeri (DerSimonian–Laird ve doğrusal havuz; Koepke vd. 2017)

`koepke2017-half-life-cs137`

#### Bu proje ne hakkında?

MacMahon vd. (2004) tarafından derlenen 19 yayımlanmış yarı ömür heterojendir (Q = 336, serbestlik derecesi 18). Uzlaşı değeri, veriler verildiği gibi gün biriminde hesaplanır; Koepke vd. bunu 365,25 günlük yıl cinsinden bildirir.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

19 laboratuvar ya da deney **T½(¹³⁷Cs)** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Koepke vd. (2017) Tablo 4: DerSimonian–Laird 30,04 a, u 0,04 a; doğrusal havuz 29,94 a, u 0,93 a; hiyerarşik Bayes 29,94 a, u 0,19 a (× 365,25 gün, tolerans son basamağın yarım birimi; doğrusal havuzda buna 10⁵ çekilişin Monte Carlo hatası eklenir). τ, Q, bootstrap u ve yüzdelikleri ile doğrusal havuz yüzdelikleri: bağımsız numpy hesabı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="koepke2017-half-life-sr90"></a>

### ⁹⁰Sr yarı ömrü: birbiriyle tutarsız 11 ölçümün uzlaşı değeri (DerSimonian–Laird ve doğrusal havuz; Koepke vd. 2017)

`koepke2017-half-life-sr90`

#### Bu proje ne hakkında?

MacMahon vd. (2004) tarafından derlenen 11 yayımlanmış yarı ömür heterojendir (Q = 400, serbestlik derecesi 10). Uzlaşı değeri, veriler verildiği gibi gün biriminde hesaplanır; Koepke vd. bunu 365,25 günlük yıl cinsinden bildirir.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

11 laboratuvar ya da deney **T½(⁹⁰Sr)** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Koepke vd. (2017) Tablo 4: DerSimonian–Laird 28,73 a, u 0,11 a; doğrusal havuz 28,68 a, u 0,77 a; hiyerarşik Bayes 28,71 a, u 0,16 a (× 365,25 gün, tolerans son basamağın yarım birimi; doğrusal havuzda buna 10⁵ çekilişin Monte Carlo hatası eklenir). τ, Q, bootstrap u ve yüzdelikleri ile doğrusal havuz yüzdelikleri: bağımsız numpy hesabı.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="koepke2017-newtonian-constant-g"></a>

### Newton çekim sabiti: 14 ölçümün uzlaşı değeri (DerSimonian–Laird ve doğrusal havuz; Koepke vd. 2017)

`koepke2017-newtonian-constant-g`

#### Bu proje ne hakkında?

CODATA 2014 düzeltmesinde kullanılan 14 G ölçümü birbiriyle tutarsızdır (Cochran Q = 319, serbestlik derecesi 13): ölçülen değerler tipik beyan edilen belirsizliğin yaklaşık dört katı dağılır. Rastgele etkiler modeli bu karanlık belirsizliği (τ) tanır.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

14 laboratuvar ya da deney **G** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Koepke vd. (2017) Tablo 2: DerSimonian–Laird 6,673 80, u 0,000 28, (6,673 20, 6,674 40); doğrusal havuz 6,673 67, 0,001 24, (6,671 18, 6,675 79); en çok olabilirlik 6,673 78, 0,000 30; hiyerarşik Bayes 6,673 77, 0,000 33, (6,673 10, 6,674 42). τ, Q, u_KH, bootstrap u ve yüzdelikleri ile D_j: bağımsız numpy hesabı (10⁵ tekrar; basılı aralık bunların 10⁻⁴'e yuvarlanmışıyla uyumludur).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Karanlık belirsizliğin tonları (İleri)

Newton kütle çekim sabitinin on altı ölçümü birkaç yöntemle değerlendirilir; uzlaşı değeri ve belirsizliği yönteme güçlü biçimde bağlıdır.

<a id="merkatas2019-newtonian-constant-g-shades"></a>

### 16 ölçümden Newton çekim sabiti: Knapp–Hartung düzeltmeli DerSimonian–Laird, en çok olabilirlik, hiyerarşik Bayes ve doğrusal havuz (Merkatas vd. 2019)

`merkatas2019-newtonian-constant-g-shades`

#### Bu proje ne hakkında?

Koepke vd. (2017) verileri; JILA-10 düzeltilmiş (6,672 60 ± 0,000 25) ve 2018'deki iki HUST sonucu (salınım süresi ve açısal ivme geri beslemesi) eklenmiştir. Merkatas vd. on dört yöntemi karşılaştırır; bunların dördü programda vardır.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

16 laboratuvar ya da deney **G** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Merkatas vd. (2019), Tablo 4, DL, ML, BG ve LOP satırları. BG ve LOP MCMC ve Monte Carlo'dan (toleranslar: yarım birim artı MCMC örnekleme hatası, ortalama için yaklaşık 3 × 10⁻⁶, sınırlar için 10⁻⁵; kesin ızgara 6,673 896, 0,000 271, (6,673 342, 6,674 415), τ ortancası 0,000 936).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir karşılaştırmada eşdeğerlik dereceleri (İleri)

NIST Consensus Builder örnekleri: serbestlik dereceli laboratuvarlardan bir uzlaşı değeri ve 19 laboratuvarın birini dışarıda bırakma ve ikili eşdeğerlik dereceleri.

<a id="nicob-pcb28-sediment-degrees-of-freedom"></a>

### Serbestlik dereceli altı laboratuvardan sedimentte PCB 28: DerSimonian–Laird, Linear Pool ve birini dışarıda bırakan eşdeğerlik dereceleri (NIST Consensus Builder §4.2)

`nicob-pcb28-sediment-degrees-of-freedom`

#### Bu proje ne hakkında?

Bir sedimentte PCB 28 (2,4,4'-triklorobifenil) kütle kesri; altı laboratuvar standart belirsizliklerinin serbestlik derecelerini bildirir (Örnek 6). Bir ν değeri 2'dir: Student t dağılımının sonlu varyansı yoktur ve NICOB onu bir yaklaşımla yeniden ölçekler (DoEUnilateralDerSimonianLaird.R, linearOP.R); program bunu izler.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

6 laboratuvar ya da deney **w(PCB 28)** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: NICOB kılavuzu Örnek 7: DerSimonian–Laird 33,6 ng/g, u 0,77, (32,0, 35,2); Linear Pool 33,6, 1,55, (31,4, 36,3) ng/g (toleranslar: basılı basamak ve Monte Carlo saçılımı, notlara bakınız). Eşdeğerlik dereceleri: NICOB kaynağının numpy aktarımı, 10⁶ çekiliş; tolerans U95 %'in %3'ü.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="nicob-co60-activity-leave-one-out-bilateral"></a>

### 19 laboratuvarda ⁶⁰Co aktivitesi: birini dışarıda bırakan ve iki taraflı eşdeğerlik dereceleriyle uzlaşı değeri (NIST Consensus Builder §4.4)

`nicob-co60-activity-leave-one-out-bilateral`

#### Bu proje ne hakkında?

Süren anahtar karşılaştırma BIPM.RI(II)-K1.Co-60'tan, NICOB kılavuzunda (Örnek 19) listelenen ⁶⁰Co SIR eşdeğer aktiviteleri. Cochran Q = 36,9, 18 serbestlik derecesi (p = 0,005): sonuçlar belirsizliklerinden daha dağınıktır; karanlık belirsizlik τ̂ = 11,9 kBq'dir.


#### Adım 1. Fikir: birkaç laboratuvardan tek bir değer

19 laboratuvar ya da deney **A (SIR equivalent activity)** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.

Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.


> **Deneyin:** **Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: NICOB kılavuzu Örnek 21: DerSimonian–Laird 7062 kBq, u 4 kBq, (7053, 7071) kBq; Linear Pool 7064, 29, (7012, 7127) kBq (toleranslar: basılı basamak ve Monte Carlo saçılımı). Eşdeğerlik dereceleri: NICOB kaynağının numpy aktarımı, 10⁶ çekiliş; tolerans U95 %'in %3'ü (programın 10⁵ çekilişinin 5σ'sı).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Ara kesinlik ve iç içe tasarımlar (İleri)

Zaman ve operatör değiştiğinde kesinlik ve birkaç faktörü az ölçümle ayıran kademeli iç içe tasarım.

<a id="iso5725-3-j1-carbon-steel"></a>

### Çelikte karbon: [zaman + operatör] farklı ara kesinlik (ISO 5725-3 J.1)

`iso5725-3-j1-carbon-steel`

#### Bu proje ne hakkında?

ISO 5725-3:2023 J.1: her gün bir rutin numune ertesi gün başka bir analist tarafından yeniden analiz edilir, bir ayda 29 çift; çiftlerden s_I(TO), Formül (5).


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(C)** (% m/m) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 0,1 % m/m.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s_I(TO)**: 27 çift ölçüm; standart sapma çiftlerin içindeki farklardan bulunur.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 5725-3:2023 J.1.2 (s_I(TO) = 2,87 × 10⁻³ %).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso5725-3-j2-vanadium-steel"></a>

### Çelikte vanadyum: kademeli iç içe tasarım, düzey 1 (ISO 5725-3 J.2)

`iso5725-3-j2-vanadium-steel`

#### Bu proje ne hakkında?

ISO 5725-3:2023 J.2: 20 laboratuvar, bir günde iki ve ertesi gün bir sonuç; varyans analizi s_r'yi, zaman farklı ara kesinlik s_I(T)'yi ve s_R'yi verir.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(V)** (% m/m) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 1 derişim düzeyinde değerlendirilir: 0,0098 % m/m.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 1 kayıt:

- **s_I(T)**: staggered_nested


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 5725-3:2023 Tablo J.4 ve J.5, düzey 1 (s_r = 0,381, s_I(T) = 0,603, s_R = 0,801 × 10⁻³ %).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## İki eksende belirsizlikli kalibrasyon fonksiyonları (İleri)

Doğrusal, ikinci dereceden, kuvvet ve üstel fonksiyonlarla gaz analizi ve içerikleri korelasyonlu referans karışımlar. Program hangi fonksiyonun uyduğunu sınar.

<a id="iso6143-b2-n2-natural-gas-covariances"></a>

### Doğal gazda N₂, GC-TCD: korelasyonlu referans karışımlar (ISO 6143 Örnek 2)

`iso6143-b2-n2-natural-gas-covariances`

#### Bu proje ne hakkında?

ISO 6143:2025 B.2.3: yine doğru; bu kez geçmişi ortak olan referans karışımların (bir seyreltme serisi ve ortak bir birincil standart seti) kovaryanslarıyla.


#### Adım 1. Fikir: bir kalibrasyon eğrisi

Bir cihaz, doğrudan **N₂** içeriği olmayan bir yanıt verir (burada peak area). Cihaz, içeriği bilinen 8 referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.

Analiz fonksiyonu: **linear**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).


#### Adım 2. Bilinmeyen numuneler

2 numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 6143:2025 B.2.3, kovaryanslı parametreler ve Tablo B.10 (B_LEAST).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso6143-b2-n2-natural-gas-linear"></a>

### Doğal gazda N₂, GC-TCD: doğrusal fonksiyon (ISO 6143 Örnek 2)

`iso6143-b2-n2-natural-gas-linear`

#### Bu proje ne hakkında?

ISO 6143:2025 B.2.3: her iki değişkende belirsizlikli sekiz kalibrasyon noktasından genelleştirilmiş en küçük kareler doğrusu, diğer analiz fonksiyonlarıyla karşılaştırma ve iki karışımın içerikleri.


#### Adım 1. Fikir: bir kalibrasyon eğrisi

Bir cihaz, doğrudan **N₂** içeriği olmayan bir yanıt verir (burada peak area). Cihaz, içeriği bilinen 8 referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.

Analiz fonksiyonu: **linear**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).


#### Adım 2. Bilinmeyen numuneler

2 numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 6143:2025 B.2.3, doğrusal parametreler ve Tablo B.8 (B_LEAST).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso6143-b2-n2-natural-gas-quadratic"></a>

### Doğal gazda N₂, GC-TCD: ikinci derece polinom (ISO 6143 Örnek 2)

`iso6143-b2-n2-natural-gas-quadratic`

#### Bu proje ne hakkında?

ISO 6143:2025 B.2.3: aynı kalibrasyon verileri, ikinci derece polinom analiz fonksiyonuyla.


#### Adım 1. Fikir: bir kalibrasyon eğrisi

Bir cihaz, doğrudan **N₂** içeriği olmayan bir yanıt verir (burada peak area). Cihaz, içeriği bilinen 8 referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.

Analiz fonksiyonu: **quadratic**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).


#### Adım 2. Bilinmeyen numuneler

2 numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 6143:2025 B.2.3, polinom katsayıları ve Tablo B.9 (B_LEAST).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso6143-b3-no-ndir-exponential"></a>

### N₂ içinde NO, NDIR: üstel fonksiyon (ISO 6143 Örnek 3)

`iso6143-b3-no-ndir-exponential`

#### Bu proje ne hakkında?

ISO 6143:2025 B.2.4: aynı NDIR kalibrasyonu, standardın tercih ettiği x = b₀ + b₁e^(b₂y) üstel analiz fonksiyonuyla; orada basılan parametre belirsizliklerinin yanlış olduğu bilinmektedir.


#### Adım 1. Fikir: bir kalibrasyon eğrisi

Bir cihaz, doğrudan **NO** içeriği olmayan bir yanıt verir (burada NDIR signal). Cihaz, içeriği bilinen 12 referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.

Analiz fonksiyonu: **exponential**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).


#### Adım 2. Bilinmeyen numuneler

1 numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 6143:2025 B.2.4, üstel parametreler, S_res, Γ ve Tablo B.12'deki x (B_LEAST); u(b) ve u(x) aynı veriler için ODRPACK'ten (scipy.odr, cov_beta).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="iso6143-b3-no-ndir-power"></a>

### N₂ içinde NO, NDIR: üslü fonksiyon (ISO 6143 Örnek 3)

`iso6143-b3-no-ndir-power`

#### Bu proje ne hakkında?

ISO 6143:2025 B.2.4: on iki noktalı doğrusal olmayan bir NDIR kalibrasyonu; x = b₀ + b₁y^(1+b₂) üslü analiz fonksiyonu genelleştirilmiş en küçük karelerle uydurulur.


#### Adım 1. Fikir: bir kalibrasyon eğrisi

Bir cihaz, doğrudan **NO** içeriği olmayan bir yanıt verir (burada NDIR signal). Cihaz, içeriği bilinen 12 referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.

Analiz fonksiyonu: **power**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).


#### Adım 2. Bilinmeyen numuneler

1 numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: ISO 6143:2025 B.2.4, üslü fonksiyon parametreleri ve Tablo B.12 (B_LEAST).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Dinamik seyreltmeyle kalibre edilen bir analizör (İleri)

Referans karışımlar aynı ana gazdan yapılır, bu yüzden içerikleri korelasyonludur. Kovaryansları yok saymak sonucu değiştirir.

<a id="emue-e3.3-no2-analyser-dynamic-dilution-covariances-ignored"></a>

### Dinamik seyreltmeyle hazırlanan karışımlarla kalibre edilen NO₂ analizörü: ilişkili referans değerlerle analiz fonksiyonu, kovaryanslar yok sayılmış (EMUE E3.3)

`emue-e3.3-no2-analyser-dynamic-dilution-covariances-ignored`

#### Bu proje ne hakkında?

Tek bir ana karışımın iki kalibre MFC ile seyreltilmesinden üç NO₂/hava karışımı (708, 974, 1290 nmol/mol); her iki değişkende belirsizlikle uydurulan doğrusal analiz fonksiyonu x = A + B·w; 975,5 nmol/mol'lük bağımsız bir karışımla doğrulama.


#### Adım 1. Fikir: bir kalibrasyon eğrisi

Bir cihaz, doğrudan **NO₂ in synthetic air** içeriği olmayan bir yanıt verir (burada a.u.). Cihaz, içeriği bilinen 3 referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.

Analiz fonksiyonu: **linear**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).


#### Adım 2. Bilinmeyen numuneler

1 numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EMUE E3.3.6: A = 88,94 nmol/mol, B = 4,19 nmol/(mol·a.u.), V_AB (E3.3.18); doğrulama y = 982,0 nmol/mol, u = 4,7 (kovaryanssız 4,5). Beklenen değerler yeniden hesaplandı (F90).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="emue-e3.3-no2-analyser-dynamic-dilution"></a>

### Dinamik seyreltmeyle hazırlanan karışımlarla kalibre edilen NO₂ analizörü: ilişkili referans değerlerle analiz fonksiyonu (EMUE E3.3)

`emue-e3.3-no2-analyser-dynamic-dilution`

#### Bu proje ne hakkında?

Tek bir ana karışımın iki kalibre MFC ile seyreltilmesinden üç NO₂/hava karışımı (708, 974, 1290 nmol/mol); her iki değişkende belirsizlikle uydurulan doğrusal analiz fonksiyonu x = A + B·w; 975,5 nmol/mol'lük bağımsız bir karışımla doğrulama.


#### Adım 1. Fikir: bir kalibrasyon eğrisi

Bir cihaz, doğrudan **NO₂ in synthetic air** içeriği olmayan bir yanıt verir (burada a.u.). Cihaz, içeriği bilinen 3 referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.

Analiz fonksiyonu: **linear**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).


#### Adım 2. Bilinmeyen numuneler

1 numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EMUE E3.3.6: A = 88,94 nmol/mol, B = 4,19 nmol/(mol·a.u.), V_AB (E3.3.18); doğrulama y = 982,0 nmol/mol, u = 4,7 (kovaryanssız 4,5). Beklenen değerler yeniden hesaplandı (F90).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Bir hata yaklaşım fonksiyonu (İleri)

Bir terazinin birçok yükteki hatalarından geçirilen bir doğru; ek terim s_m'nin ne kadar büyük olması gerektiğine bir χ² testi ve bir artık testi karar verir.

<a id="cg18-h4-400g-error-function-sm-0.05"></a>

### Terazi 400 g / 0,1 mg: s_m = 0,05 mg ile hata yaklaşım fonksiyonu, χ² testi geçer (EURAMET cg-18 H4)

`cg18-h4-400g-error-function-sm-0.05`

#### Bu proje ne hakkında?

Korelasyonlu referans kütlelerle dokuz deney noktasından 400 g'lık bir terazinin hata yaklaşım fonksiyonu E = a₁I.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Electronic balance, Max 400 g, d = 0.1 mg, 9 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 H4.4, s_m = 0,05 mg: a₁ = 0,000 84 mg/g, U(â) = 5,637·10⁻⁸ (mg/g)², χ²_obs = 7,3; Tablo H4.6'nın u(E_yakl) değerleri ve artık testi.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h4-400g-error-function-sm-0.25"></a>

### Terazi 400 g / 0,1 mg: s_m = 0,25 mg ile hata yaklaşım fonksiyonu, artık testi geçer (EURAMET cg-18 H4)

`cg18-h4-400g-error-function-sm-0.25`

#### Bu proje ne hakkında?

Korelasyonlu referans kütlelerle dokuz deney noktasından 400 g'lık bir terazinin hata yaklaşım fonksiyonu E = a₁I.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Electronic balance, Max 400 g, d = 0.1 mg, 9 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 H4.4, s_m = 0,25 mg: a₁ = 0,000 84 mg/g, U(â) = 1,745·10⁻⁷ (mg/g)², u(a₁) = 0,000 42 mg/g; Tablo H4.8'in u(E_yakl) değerleri.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="cg18-h4-400g-error-function"></a>

### Terazi 400 g / 0,1 mg: hata yaklaşım fonksiyonu E = a₁I, ilk uyum (EURAMET cg-18 H4)

`cg18-h4-400g-error-function`

#### Bu proje ne hakkında?

Korelasyonlu referans kütlelerle dokuz deney noktasından 400 g'lık bir terazinin hata yaklaşım fonksiyonu E = a₁I.


#### Adım 1. Fikir: bir terazinin kalibrasyonu

Electronic balance, Max 400 g, d = 0.1 mg, 9 yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).

Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: EURAMET cg-18 v4.0 H4.4, s_m = 0: a₁ = 0,000 83 mg/g, U(â) = 5,109·10⁻⁸ (mg/g)², u(a₁) = 0,000 23 mg/g, χ²_obs = 12,5 (test başarısız).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Yukarıdan aşağı bir modelde matris etkileri (İleri)

Deniz sedimentlerinde yedi element: farklı matrisli referans malzemelerden geri kazanımlar ve iki derişim aralıklı bir model.

<a id="palma2019-cr-marine-sediment"></a>

### Deniz sedimentlerinde krom, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-cr-marine-sediment`

#### Bu proje ne hakkında?

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Cr belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(Cr)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 5; 50 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **s_r(h)⟨I⟩**: standart sapma 0,285 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_I⟨I⟩**: standart sapma 0,565 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_r(h)⟨II⟩**: standart sapma 2,85 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 5,65 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Diğer bileşenler**, verildiği gibi eklenir:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Palma ve ark. (2019) Tablo 9: u_c⟨I⟩ = 1,05 mg/kg, u′_c⟨II⟩ = %10,5; tolerans basılı son basamağın bir birimidir, çünkü bileşenlerin kendileri de yuvarlanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="palma2019-cu-marine-sediment"></a>

### Deniz sedimentlerinde bakır, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-cu-marine-sediment`

#### Bu proje ne hakkında?

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Cu belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(Cu)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 5; 50 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **s_r(h)⟨I⟩**: standart sapma 0,242 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_I⟨I⟩**: standart sapma 0,248 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_r(h)⟨II⟩**: standart sapma 2,43 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 2,48 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Diğer bileşenler**, verildiği gibi eklenir:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Palma ve ark. (2019) Tablo 9: u_c⟨I⟩ = 0,612 mg/kg, u′_c⟨II⟩ = %6,13; tolerans basılı son basamağın bir birimidir, çünkü bileşenlerin kendileri de yuvarlanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="palma2019-li-marine-sediment"></a>

### Deniz sedimentlerinde lityum, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-li-marine-sediment`

#### Bu proje ne hakkında?

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Li belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(Li)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 0,5; 5 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **s_r(h)⟨I⟩**: standart sapma 0,0345 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_I⟨I⟩**: standart sapma 0,058 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_r(h)⟨II⟩**: standart sapma 3,45 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 5,8 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Diğer bileşenler**, verildiği gibi eklenir:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Palma ve ark. (2019) Tablo 9: u_c⟨I⟩ = 0,127 mg/kg, u′_c⟨II⟩ = %12,7; tolerans basılı son basamağın bir birimidir, çünkü bileşenlerin kendileri de yuvarlanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="palma2019-mn-marine-sediment"></a>

### Deniz sedimentlerinde mangan, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-mn-marine-sediment`

#### Bu proje ne hakkında?

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Mn belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(Mn)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 5; 50 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **s_r(h)⟨I⟩**: standart sapma 0,179 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_I⟨I⟩**: standart sapma 0,656 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_r(h)⟨II⟩**: standart sapma 1,79 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 6,56 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Diğer bileşenler**, verildiği gibi eklenir:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Palma ve ark. (2019) Tablo 9: u_c⟨I⟩ = 0,92 mg/kg, u′_c⟨II⟩ = %9,2; tolerans basılı son basamağın bir birimidir, çünkü bileşenlerin kendileri de yuvarlanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="palma2019-ni-marine-sediment"></a>

### Deniz sedimentlerinde nikel, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-ni-marine-sediment`

#### Bu proje ne hakkında?

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde asitte ekstrakte edilebilir Ni belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(Ni)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 7,5; 75 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **s_r(h)⟨I⟩**: standart sapma 0,346 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_I⟨I⟩**: standart sapma 1,13 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_r(h)⟨II⟩**: standart sapma 2,58 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 7,54 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Diğer bileşenler**, verildiği gibi eklenir:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Palma ve ark. (2019) Tablo 9: u_c⟨I⟩ = 1,25 mg/kg, u′_c⟨II⟩ = %8,44; tolerans basılı son basamağın bir birimidir, çünkü bileşenlerin kendileri de yuvarlanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="palma2019-pb-marine-sediment"></a>

### Deniz sedimentlerinde kurşun, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-pb-marine-sediment`

#### Bu proje ne hakkında?

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde asitte ekstrakte edilebilir Pb belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(Pb)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 10; 100 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **s_r(h)⟨I⟩**: standart sapma 0,714 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_I⟨I⟩**: standart sapma 0,804 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_r(h)⟨II⟩**: standart sapma 4,95 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 4,02 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Diğer bileşenler**, verildiği gibi eklenir:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Palma ve ark. (2019) Tablo 9: u_c⟨I⟩ = 1,9 mg/kg, u′_c⟨II⟩ = %10,1; tolerans basılı son basamağın bir birimidir, çünkü bileşenlerin kendileri de yuvarlanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="palma2019-zn-marine-sediment"></a>

### Deniz sedimentlerinde çinko, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)

`palma2019-zn-marine-sediment`

#### Bu proje ne hakkında?

Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde toplam Zn belirsizliği.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(Zn)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 2; 20 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 4 kayıt:

- **s_r(h)⟨I⟩**: standart sapma 0,108 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s_I⟨I⟩**: standart sapma 0,117 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_r(h)⟨II⟩**: standart sapma 2,69 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 2,92 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Diğer bileşenler**, verildiği gibi eklenir:

- u_ME⟨I⟩
- u′_ME⟨II⟩
- u_T⟨I⟩
- u′_T⟨II⟩


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Palma ve ark. (2019) Tablo 9: u_c⟨I⟩ = 0,365 mg/kg, u′_c⟨II⟩ = %9,13; tolerans basılı son basamağın bir birimidir, çünkü bileşenlerin kendileri de yuvarlanmıştır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Aritmetik ya da ağırlıklı ortalama geri kazanım (İleri)

Nehir sedimentlerinde floranten: ortalama geri kazanım, MUPR kılavuzu ve hesap tablosundaki gibi iki yolla hesaplanır.

<a id="bettencourt2026-fluoranthene-arithmetic"></a>

### Nehir sedimentlerinde fluoranten, aritmetik ortalama geri kazanım (MUPR Eş. 8–12) (Bettencourt da Silva 2026)

`bettencourt2026-fluoranthene-arithmetic`

#### Bu proje ne hakkında?

MUPR elektronik tablosunun laboratuvar içi validasyon verilerinden nehir sedimentlerinde fluorantenin top-down belirsizliği: iki aralıklı kesinlik modeli ve uyumlu olmayan 20 ortalama geri kazanım; bu nedenle geri kazanımların dağılımı eklenir ve sonuçlar geri kazanım için düzeltilir.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(fluoranthene)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 0,1; 10 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 2 kayıt:

- **s_I⟨I⟩**: standart sapma 0,075185 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 5,0123 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Geri kazanım**: 20 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Ek Materyal 06'dan yeniden hesaplanmıştır (bağımsız betik). χ² = 40,9729, s(R) = %8,7552 ve R̿ = %74,8871 Trueness sayfasıyla (CM16, CO16, CK16) uyumludur; belirsizlikler notlarda açıklandığı gibi elektronik tablodan farklıdır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bettencourt2026-fluoranthene-weighted"></a>

### Nehir sedimentlerinde fluoranten, ağırlıklı ortalama geri kazanım (MUPR Eş. 16.1) (Bettencourt da Silva 2026)

`bettencourt2026-fluoranthene-weighted`

#### Bu proje ne hakkında?

MUPR elektronik tablosunun laboratuvar içi validasyon verilerinden nehir sedimentlerinde fluorantenin top-down belirsizliği: iki aralıklı kesinlik modeli ve uyumlu olmayan 20 ortalama geri kazanım; bu nedenle geri kazanımların dağılımı eklenir ve sonuçlar geri kazanım için düzeltilir.


#### Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik

Bu, **w(fluoranthene)** (mg/kg) için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.

Belirsizlik 2 derişim düzeyinde değerlendirilir: 0,1; 10 mg/kg.


#### Adım 2. Veriler

**Kesinlik** (rastgele etkiler), 2 kayıt:

- **s_I⟨I⟩**: standart sapma 0,075185 (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).
- **s′_I⟨II⟩**: standart sapma 5,0123 % (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).

**Geri kazanım**: 20 geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100'den anlamlı biçimde farklıysa düzeltme düşünülür.


#### Adım 3. Program ne hesaplıyor?

Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.


> **Deneyin:** Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Ek Materyal 06'dan yeniden hesaplanmıştır (bağımsız betik). χ² = 40,9729, s(R) = %8,7552 ve R̿ = %74,8871 Trueness sayfasıyla (CM16, CO16, CK16) uyumludur; belirsizlikler notlarda açıklandığı gibi elektronik tablodan farklıdır.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Nicel NMR ile saflık (BIPM) (İleri)

BIPM'in qNMR iç standartları için referans verileri. Tam saflık bütçeleri; bir sonuç fiziksel sınır olan 1'e yakındır ve asimetrik bir aralıkla raporlanır.

<a id="bipm-isrd-01-qnmr-dmso2-ma-d2o"></a>

### İç standart olarak maleik asit ile D2O içinde ¹H qNMR ile dimetil sülfon saflığı (BIPM ISRD-01, Tablo 4)

`bipm-isrd-01-qnmr-dmso2-ma-d2o`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): dimetil sülfon kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 5,4919 1, standart belirsizlik u = 0,00096 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: dimetil sülfon nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 5,9934 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: maleik asit nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 1,9994 1, standart belirsizlik u = 0,0002 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: dimetil sülfon mol kütlesi. En iyi değer 94,136 g/mol, standart belirsizlik u = 0,005 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: maleik asit mol kütlesi. En iyi değer 116,0724 g/mol, standart belirsizlik u = 0,0025 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan dimetil sülfon kütlesi. En iyi değer 3,5063 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan maleik asit kütlesi. En iyi değer 2,3545 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart maleik asit saflığı. En iyi değer 0,9995 g/g, standart belirsizlik u = 0,0003 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,9973 g/g
- standart belirsizlik u = 0,00073 g/g

Kaynak: BIPM ISRD-01, Tablo 4: atanan değer 0,9973 g/g, birleşik standart belirsizlik 7,300e-04 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %0,3 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bipm-isrd-01-qnmr-dmtp-ma-dmso-d6"></a>

### İç standart olarak maleik asit ile DMSO-d6 içinde ¹H qNMR ile dimetil tereftalat saflığı (BIPM ISRD-01, Tablo 6)

`bipm-isrd-01-qnmr-dmtp-ma-dmso-d6`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): dimetil tereftalat kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 0,7501 1, standart belirsizlik u = 0,00012 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: dimetil tereftalat nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 3,9968 1, standart belirsizlik u = 0,0002 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: maleik asit nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 1,9994 1, standart belirsizlik u = 0,0002 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: dimetil tereftalat mol kütlesi. En iyi değer 194,184 g/mol, standart belirsizlik u = 0,006 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: maleik asit mol kütlesi. En iyi değer 116,072 g/mol, standart belirsizlik u = 0,0025 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan dimetil tereftalat kütlesi. En iyi değer 20,3108 mg, standart belirsizlik u = 0,00337 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan maleik asit kütlesi. En iyi değer 32,3521 mg, standart belirsizlik u = 0,00337 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart maleik asit saflığı. En iyi değer 0,9995 g/g, standart belirsizlik u = 0,0003 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,9995 g/g
- standart belirsizlik u = 0,000396 g/g

Kaynak: BIPM ISRD-01, Tablo 6: atanan değer 0,9995 g/g, birleşik standart belirsizlik 3,960e-04 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %3,4 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bipm-isrd-02-qnmr-dmtp-dmso2-cdcl3"></a>

### İç standart olarak dimetil sülfon ile CDCl3 içinde ¹H qNMR ile dimetil tereftalat saflığı (BIPM ISRD-02, Tablo 6)

`bipm-isrd-02-qnmr-dmtp-dmso2-cdcl3`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): dimetil tereftalat kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 0,7154 1, standart belirsizlik u = 0,00009 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: dimetil tereftalat nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 5,9988 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: dimetil sülfon nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 5,9988 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: dimetil tereftalat mol kütlesi. En iyi değer 194,194 g/mol, standart belirsizlik u = 0,008 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: dimetil sülfon mol kütlesi. En iyi değer 94,1284 g/mol, standart belirsizlik u = 0,0062 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan dimetil tereftalat kütlesi. En iyi değer 8,5689 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan dimetil sülfon kütlesi. En iyi değer 5,8213 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart dimetil sülfon saflığı. En iyi değer 0,9964 g/g, standart belirsizlik u = 0,0004 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,999 g/g
- standart belirsizlik u = 0,000505 g/g

Kaynak: BIPM ISRD-02, Tablo 6: atanan değer 0,999 g/g, birleşik standart belirsizlik 5,050e-04 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %0,2 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bipm-isrd-02-qnmr-ma-dmso2-d2o"></a>

### İç standart olarak dimetil sülfon ile D2O içinde ¹H qNMR ile maleik asit saflığı (BIPM ISRD-02, Tablo 4)

`bipm-isrd-02-qnmr-ma-dmso2-d2o`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): maleik asit kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 0,1821 1, standart belirsizlik u = 0,00003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: maleik asit nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 1,9996 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: dimetil sülfon nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 5,9988 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: maleik asit mol kütlesi. En iyi değer 116,08 g/mol, standart belirsizlik u = 0,004 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: dimetil sülfon mol kütlesi. En iyi değer 94,1284 g/mol, standart belirsizlik u = 0,0062 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan maleik asit kütlesi. En iyi değer 2,3545 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan dimetil sülfon kütlesi. En iyi değer 3,5063 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart dimetil sülfon saflığı. En iyi değer 0,9965 g/g, standart belirsizlik u = 0,0004 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,9997 g/g
- standart belirsizlik u = 0,000791 g/g

Kaynak: BIPM ISRD-02, Tablo 4: atanan değer 0,9997 g/g, birleşik standart belirsizlik 7,910e-04 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %0,4 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od"></a>

### İç standart olarak BTMSB-d4 ile CD3OD içinde ¹H qNMR ile benzoik asit saflığı (BIPM ISRD-06, Tablo 6)

`bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): benzoik asit kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```

Sonuç 1 değerinin üstünde olamaz (fiziksel bir sınır; örneğin saflık 1'i aşamaz). Program aralığı verirken bu sınırı dikkate alır.


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 1,5 1, standart belirsizlik u = 0,00026 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: benzoik asit nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 4,999 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: BTMSB-d4 nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 17,9964 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: benzoik asit mol kütlesi. En iyi değer 122,123 g/mol, standart belirsizlik u = 0,0054 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: BTMSB-d4 mol kütlesi. En iyi değer 226,502 g/mol, standart belirsizlik u = 0,0125 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan benzoik asit kütlesi. En iyi değer 6,4659 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan BTMSB-d4 kütlesi. En iyi değer 2,2223 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart BTMSB-d4 saflığı. En iyi değer 0,9991 g/g, standart belirsizlik u = 0,0004 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,99977 g/g
- standart belirsizlik u = 0,000744 g/g

Kaynak: BIPM ISRD-06, Tablo 6: atanan değer 0,9998 g/g; integral oranı 1,5 olarak (iki anlamlı basamak) basıldığından ±0,033 g/g'lık bir tolerans gerekir, bu nedenle tahmin basılı girdilerden yeniden hesaplanan değerle (bağımsız Python, 0,999774014 g/g) karşılaştırılır; birleşik standart belirsizlik 7,440e-04 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %0,6 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bipm-isrd-06-qnmr-dmso2-btmsb-d4-dmso-d6"></a>

### İç standart olarak BTMSB-d4 ile DMSO-d6 içinde ¹H qNMR ile dimetil sülfon saflığı (BIPM ISRD-06, Tablo 4)

`bipm-isrd-06-qnmr-dmso2-btmsb-d4-dmso-d6`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): dimetil sülfon kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 0,8173 1, standart belirsizlik u = 0,00011 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: dimetil sülfon nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 5,9988 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: BTMSB-d4 nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 17,9964 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: dimetil sülfon mol kütlesi. En iyi değer 94,128 g/mol, standart belirsizlik u = 0,007 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: BTMSB-d4 mol kütlesi. En iyi değer 226,5024 g/mol, standart belirsizlik u = 0,0125 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan dimetil sülfon kütlesi. En iyi değer 2,6524 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan BTMSB-d4 kütlesi. En iyi değer 2,5969 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart BTMSB-d4 saflığı. En iyi değer 0,9991 g/g, standart belirsizlik u = 0,0004 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,9967 g/g
- standart belirsizlik u = 0,000797 g/g

Kaynak: BIPM ISRD-06, Tablo 4: atanan değer 0,9967 g/g, birleşik standart belirsizlik 7,970e-04 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %0,2 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bipm-isrd-07-qnmr-dmso2-dss-d6-d2o"></a>

### İç standart olarak DSS-d6 ile D2O içinde ¹H qNMR ile dimetil sülfon saflığı (BIPM ISRD-07, Tablo 4)

`bipm-isrd-07-qnmr-dmso2-dss-d6-d2o`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): dimetil sülfon kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 4,5173 1, standart belirsizlik u = 0,00057 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: dimetil sülfon nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 5,9988 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: DSS-d6 nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 8,9982 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: dimetil sülfon mol kütlesi. En iyi değer 94,128 g/mol, standart belirsizlik u = 0,0069 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: DSS-d6 mol kütlesi. En iyi değer 224,3544 g/mol, standart belirsizlik u = 0,0081 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan dimetil sülfon kütlesi. En iyi değer 3,252 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan DSS-d6 kütlesi. En iyi değer 1,2351 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart DSS-d6 saflığı. En iyi değer 0,9227 g/g, standart belirsizlik u = 0,00045 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,9962 g/g
- standart belirsizlik u = 0,00119 g/g

Kaynak: BIPM ISRD-07, Tablo 4: atanan değer 0,9962 g/g, birleşik standart belirsizlik 1,190e-03 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %0,3 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bipm-isrd-07-qnmr-ma-dss-d6-d2o"></a>

### İç standart olarak DSS-d6 ile D2O içinde ¹H qNMR ile maleik asit saflığı (BIPM ISRD-07, Tablo 6)

`bipm-isrd-07-qnmr-ma-dss-d6-d2o`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): maleik asit kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 1,7512 1, standart belirsizlik u = 0,00031 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: maleik asit nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 1,9996 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: DSS-d6 nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 8,9982 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: maleik asit mol kütlesi. En iyi değer 116,072 g/mol, standart belirsizlik u = 0,004 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: DSS-d6 mol kütlesi. En iyi değer 224,354 g/mol, standart belirsizlik u = 0,0081 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan maleik asit kütlesi. En iyi değer 5,1383 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan DSS-d6 kütlesi. En iyi değer 1,3641 mg, standart belirsizlik u = 0,00124 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart DSS-d6 saflığı. En iyi değer 0,9227 g/g, standart belirsizlik u = 0,00045 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,9987 g/g
- standart belirsizlik u = 0,00108 g/g

Kaynak: BIPM ISRD-07, Tablo 6: atanan değer 0,9987 g/g, birleşik standart belirsizlik 1,080e-03 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %0,4 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="bipm-isrd-08-qnmr-btmsb-d4-ba-cd3od"></a>

### İç standart olarak benzoik asit ile CD3OD içinde ¹H qNMR ile BTMSB-d4 saflığı (BIPM ISRD-08, Tablo 2)

`bipm-isrd-08-qnmr-btmsb-d4-ba-cd3od`

#### Bu proje ne hakkında?

İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **w_A** (g/g): BTMSB-d4 kütle kesri (saflık). Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
w_A = R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 8 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **R**: İntegral oranı I_A/I_S (tekrarların ortalaması). En iyi değer 1,4313 1, standart belirsizlik u = 0,00032 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **N_A**: BTMSB-d4 nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 17,9964 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **N_S**: benzoik asit nicel sinyalinin ¹H çekirdek sayısı. En iyi değer 1,9996 1, standart belirsizlik u = 0,0003 1 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **M_A**: BTMSB-d4 mol kütlesi. En iyi değer 226,502 g/mol, standart belirsizlik u = 0,013 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **M_S**: benzoik asit mol kütlesi. En iyi değer 122,123 g/mol, standart belirsizlik u = 0,006 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **m_A**: Tartılan BTMSB-d4 kütlesi. En iyi değer 2,2271 mg, standart belirsizlik u = 0,00029 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **m_S**: Tartılan benzoik asit kütlesi. En iyi değer 7,5422 mg, standart belirsizlik u = 0,00029 mg (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **w_S**: İç standart benzoik asit saflığı. En iyi değer 0,99992 g/g, standart belirsizlik u = 0,000025 g/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,9988 g/g
- standart belirsizlik u = 0,00031 g/g

Kaynak: BIPM ISRD-08, Tablo 2: atanan değer 0,9988 g/g, birleşik standart belirsizlik 3,100e-04 g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %0,6 (F97).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## İzotop seyreltme (IDMS) (İleri)

Çift izotop seyreltmeyle suda kurşun: izotop oranları, kütleler ve bir kalibrasyon standardı içeren uzun bir model. Küçük belirsizlikli bir referans yöntem.

<a id="quam-a7-assay-standard"></a>

### IDMS için kurşun tayin standardı

`quam-a7-assay-standard`

#### Bu proje ne hakkında?

QUAM Örnek A7, Eş. (8): gravimetrik olarak hazırlanmış birincil kurşun tayin standardının madde miktarı içeriği.


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_z** (µmol/g): Tayin standardının madde miktarı içeriği. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_z = 1e6 * m2 / d2 * m1 * w / d1 / M
```


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 6 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **m1**: Kurşun kütlesi. En iyi değer 0,36544 g, standart belirsizlik u = 0,00005 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **d1**: Birinci seyreltme (toplam kütle). En iyi değer 196,14 g, standart belirsizlik u = 0,03 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **m2**: Birinci seyreltmeden alınan kütle. En iyi değer 1,0292 g, standart belirsizlik u = 0,0002 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **d2**: İkinci seyreltme (toplam kütle). En iyi değer 99,931 g, standart belirsizlik u = 0,01 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **w**: Kurşunun saflığı. En iyi değer 0,99999, standart belirsizlik u = 0,000005 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **M**: Tayin malzemesindeki kurşunun molar kütlesi. En iyi değer 207,2104 g/mol, standart belirsizlik u = 0,001 g/mol (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,092605 µmol/g
- standart belirsizlik u = 0,000028 µmol/g

Kaynak: QUAM:2012 Örnek A7, Tablo A7.3.

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

<a id="quam-a7-lead-double-idms"></a>

### Suda kurşunun çift izotop seyreltmesiyle tayini

`quam-a7-lead-double-idms`

#### Bu proje ne hakkında?

QUAM Örnek A7, Eş. (5): kütle ayrımı düzeltilmiş izotop oranlarıyla çift IDMS (1. tekrar).


#### Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?

Sonuç **c_x** (µmol/g): Su numunesindeki kurşunun madde miktarı içeriği. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır

```
c_x = c_z * m_y/m_x * m_z/m_y2 * (K_y1*R_y1 - K_b*R_b)/(K_b*R_b - K_x1*R_x1) * (K_b2*R_b2 - K_z1*R_z1)/(K_y1*R_y1 - K_b2*R_b2) * sum_x/sum_z - c_blank
```

Denklemi okunur tutmak için önce 11 ara büyüklük hesaplanır:

- `K_b = K0_b + Kbias_b`
- `K_b2 = K0_b2 + Kbias_b2`
- `K_x1 = K0_x1 + Kbias_x1`
- `K_x3 = K0_x3 + Kbias_x3`
- `K_x4 = K0_x4 + Kbias_x4`
- `K_y1 = K0_y1 + Kbias_y1`
- `K_z1 = K0_z1 + Kbias_z1`
- `K_z3 = K0_z3 + Kbias_z3`
- `K_z4 = K0_z4 + Kbias_z4`
- `sum_x = K_x1*R_x1 + 1 + K_x3*R_x3 + K_x4*R_x4`
- `sum_z = K_z1*R_z1 + 1 + K_z3*R_z3 + K_z4*R_z4`


#### Adım 2. Girdi büyüklükleri ve belirsizlikleri

Modelde 33 girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u'nun nasıl bulunacağını belirler:

1. **c_z**: Tayin standardının madde miktarı içeriği. En iyi değer 0,092605 µmol/g, standart belirsizlik u = 0,000028 µmol/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
2. **m_x**: b karışımındaki numune kütlesi. En iyi değer 1,044 g, standart belirsizlik u = 0,0002 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
3. **m_y**: b karışımındaki işaretleyici kütlesi. En iyi değer 1,136 g, standart belirsizlik u = 0,0002 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
4. **m_y2**: b' karışımındaki işaretleyici kütlesi. En iyi değer 1,0654 g, standart belirsizlik u = 0,0002 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
5. **m_z**: b' karışımındaki tayin standardı kütlesi. En iyi değer 1,1029 g, standart belirsizlik u = 0,0002 g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
6. **K0_b**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,0025/√8. En iyi değer 0,9987, standart belirsizlik u = 0,00088388 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
7. **K0_b2**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,0025/√8. En iyi değer 0,9983, standart belirsizlik u = 0,00088388 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
8. **K0_x1**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,0025/√8. En iyi değer 0,9992, standart belirsizlik u = 0,00088388 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
9. **K0_x3**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,0035/√8. En iyi değer 1,0004, standart belirsizlik u = 0,0012374 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
10. **K0_x4**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,006/√8. En iyi değer 1,001, standart belirsizlik u = 0,0021213 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
11. **K0_y1**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,0025/√8. En iyi değer 0,9999, standart belirsizlik u = 0,00088388 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
12. **K0_z1**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,0025/√8. En iyi değer 0,9989, standart belirsizlik u = 0,00088388 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
13. **K0_z3**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,0035/√8. En iyi değer 0,9993, standart belirsizlik u = 0,0012374 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
14. **K0_z4**: Kütle ayrımı düzeltme çarpanı (ölçülen); u = 0,006/√8. En iyi değer 1,0002, standart belirsizlik u = 0,0021213 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
15. **R_b**: Ölçülen izotop madde miktarı oranı; u = 0,00073/√8. En iyi değer 0,2936, standart belirsizlik u = 0,00025809 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
16. **R_b2**: Ölçülen izotop madde miktarı oranı; u = 0,0013/√8. En iyi değer 0,505, standart belirsizlik u = 0,00045962 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
17. **R_x1**: Ölçülen izotop madde miktarı oranı; u = 0,0054/√8. En iyi değer 2,1402, standart belirsizlik u = 0,0019092 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
18. **R_x3**: Ölçülen izotop madde miktarı oranı; u = 0,0032/√8. En iyi değer 0,9142, standart belirsizlik u = 0,0011314 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
19. **R_x4**: Ölçülen izotop madde miktarı oranı; u = 0,00035/√8. En iyi değer 0,05901, standart belirsizlik u = 0,00012374 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
20. **R_y1**: Ölçülen izotop madde miktarı oranı; u = 4e-05/√8. En iyi değer 0,00064, standart belirsizlik u = 1,4142 × 10⁻⁵ (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
21. **R_z1**: Ölçülen izotop madde miktarı oranı; u = 0,0054/√8. En iyi değer 2,1429, standart belirsizlik u = 0,0019092 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
22. **R_z3**: Ölçülen izotop madde miktarı oranı; u = 0,0032/√8. En iyi değer 0,9147, standart belirsizlik u = 0,0011314 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
23. **R_z4**: Ölçülen izotop madde miktarı oranı; u = 0,00035/√8. En iyi değer 0,0587, standart belirsizlik u = 0,00012374 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
24. **c_blank**: Yöntem körü; u = 4,0 × 10⁻⁷/√8. En iyi değer 4,5e-7 µmol/g, standart belirsizlik u = 1,4142 × 10⁻⁷ µmol/g (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
25. **Kbias_b**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
26. **Kbias_b2**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
27. **Kbias_x1**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
28. **Kbias_x3**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
29. **Kbias_x4**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
30. **Kbias_y1**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
31. **Kbias_z1**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
32. **Kbias_z3**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).
33. **Kbias_z4**: Kütle ayrımı düzeltmesinin sapması. En iyi değer 0, standart belirsizlik u = 0,001 (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).


#### Adım 3. Program ne hesaplıyor?

1. Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.
2. Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).
3. Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.
4. Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.
5. u_c'yi kapsam faktörü k = 2 ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).

Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından 1 000 000 rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu 1 000 000 sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.


> **Deneyin:** **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.


#### Son olarak: kaynak ne sonuç veriyor?

GUM yöntemiyle kaynak şunları verir:

- tahmin y = 0,053738 µmol/g
- standart belirsizlik u = 0,00018 µmol/g

Kaynak: QUAM:2012 Örnek A7, Tablo A7.8 (1. tekrar).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.

## Gy'nin numune alma teorisi (İleri)

Tavuk yemindeki bir enzimin temel numune alma hatası, parçacık boyutu ve özelliklerinden kestirilir.

<a id="ufs-a5-enzyme-feed-gy"></a>

### Tavuk yeminde enzim: Gy kuramıyla temel numune alma hatası (Eurachem UfS Örnek A5)

`ufs-a5-enzyme-feed-gy`

#### Bu proje ne hakkında?

25 kg'lık torbadan 500 g birincil numune (%0,05 m/m enzim), < 0,5 mm'ye öğütülür ve ekstraksiyon ile LC için 2 g deney porsiyonu alınır (%5 analitik RSD). Tablo A5.1'deki malzeme özellikleriyle modelleme yaklaşımı.


#### Adım 1. Fikir: belirsizlik numune almayla başlar

Bir sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca küçük bir numune analiz edilir. **Çift numune yöntemi** (Eurachem UfS), birkaç hedeften ikişer bağımsız numune alır ve her birini iki kez analiz eder. Varyans analizi sonra **numune almadan** ve **analizden** gelen değişimi birbirinden ayırır.

Burada temel numune alma hatası da parçacıkların özelliklerinden kestirilir (Gy teorisi).


> **Deneyin:** Sonuçlarda u(numune alma) ile u(analiz)'i karşılaştırın: çoğu zaman numune alma payı daha büyüktür ve daha iyi bir analiz işe yaramaz.


#### Son olarak: kaynak ne sonuç veriyor?

Kaynak: Örnek A5'in Bölüm 5 ve özet değerleri (c, C₁, C₂, s_r1, s_r2, s_t; U′ numune alma %26,8, toplam %28,6).

**Hesapla**'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.
