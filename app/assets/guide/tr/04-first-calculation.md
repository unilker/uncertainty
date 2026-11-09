# 4. İlk hesabınız, adım adım

Şimdi bir masanın boyunu çelik şerit metreyle ölçüp belirsizliğini programla, tıklama tıklama hesaplıyoruz. Yaklaşık on dakika sürer.

## Ölçüm {#measurement}

Masanın uzun kenarını beş kez ölçüyor ve en yakın milimetreye okuyorsunuz:

```
1502, 1503, 1501, 1502, 1503 mm
```

Şeridin bir kalibrasyon sertifikası var: *düzeltme 0 mm, genişletilmiş belirsizlik U = 0,6 mm, k = 2*. Şerit milimetre bölmeli olduğu için her okuma en yakın milimetreye yuvarlanmıştır.

## 1. adım: model {#step-model}

Uzunluk; okuma artı en iyi değeri sıfır olan, ama belirsizliği sıfır olmayan iki düzeltmedir:

```
L = L_r + d_cal + d_res
```

| Girdi | Anlamı | Bilgi | Tür |
|---|---|---|---|
| L_r | şeridin okuması | beş okuma | A |
| d_cal | kalibrasyon sertifikasından düzeltme | 0 mm, U = 0,6 mm, k = 2 | B |
| d_res | en yakın milimetreye yuvarlama | −0,5 ile +0,5 mm arası | B |

## 2. adım: projeyi oluşturun {#step-create}

1. Ana ekranda **Yeni ölçüm modeli**'ne basın. Küçük bir örnekle (bir kadmiyum çözeltisi) bir proje açılır; onu değiştireceğiz.
2. **Genel** sekmesinde bir başlık yazın, örneğin *Bir masanın boyu*.
3. **Model** sekmesini açın.

## 3. adım: çıktıyı ve denklemi girin {#step-output}

1. **Çıktı büyüklüğü** altında **Ad** alanına `L`, **Birim** alanına `mm` yazın.
2. **Model denklemi** alanında eski denklemi silip `L_r + d_cal + d_res` yazın.

> **İpucu:** Adlarda harf, rakam ve alt çizgi kullanılabilir. Çarpma `*`, bölme `/`, üs `^` ile yazılır; `sqrt()`, `exp()`, `ln()`, `sin()` gibi fonksiyonlar kullanılabilir.

## 4. adım: üç girdiyi girin {#step-inputs}

Örnekte zaten üç girdi kartı var. Onları tek tek değiştirin.

**Birinci girdi: okumalar (A tipi).**

1. **Ad**: `L_r`, **Birim**: `mm`.
2. **Dağılım**: *A tipi (gözlemler)* seçin.
3. **Gözlemler (her satıra bir)** alanına beş okumayı, her satıra bir tane yazın.

**İkinci girdi: şeridin kalibrasyonu.**

1. **Ad**: `d_cal`, **Birim**: `mm`.
2. **Dağılım**: *normal*.
3. **ortalama**: `0`. **standart belirsizlik** alanını boş bırakın; **genişletilmiş belirsizlik U** = `0,6` ve **kapsam faktörü k** = `2` girin. Program U'yu k'ye kendisi böler.

**Üçüncü girdi: çözünürlük.**

1. **Ad**: `d_res`, **Birim**: `mm`.
2. **Dağılım**: *dikdörtgen*.
3. **değer**: `0`, **yarı genişlik a**: `0,5`.

> **Dikkat:** Bir girdinin dağılımını değiştirdiğinizde her alanı yeniden denetleyin. Eski örnekten kalan bir değer (0,9999 ya da 100 gibi) tuhaf sonuçların sık bir nedenidir.

## 5. adım: hesaplayın {#step-calculate}

Kırmızı **Hesapla** düğmesine basın. **Sonuçlar** sekmesi açılır.

## 6. adım: sonuçları okuyun {#step-results}

**Raporlanan sonuç.** En üstte bir rapora kopyalanacak cümle var:

```
L = (1502,2 ± 1,1) mm
```

Ardından genişletilmiş belirsizliğin k = 2 ile bulunduğu ve yaklaşık %95'e karşılık geldiği açıklanır.

**GUM tablosu.**

| | Değer | Anlamı |
|---|---|---|
| Tahmin | 1502,20 mm | beş okumanın ortalaması, artı iki sıfır düzeltme |
| Birleşik standart belirsizlik | 0,560 mm | üç girdi birlikte |
| Etkin serbestlik derecesi | 20,0 | u_c'nin ne kadar güvenilir olduğu; 20, k = 2 için yeterlidir |
| Kapsam faktörü | 2,00 | **Ayarlar**'da belirlendiği gibi |
| Genişletilmiş belirsizlik | 1,12 mm | 2 × 0,560, 1,1 mm olarak raporlanır |

**Belirsizlik bütçesi.**

| Girdi | Standart belirsizlik | Varyans payı | Nereden geliyor |
|---|---|---|---|
| L_r | 0,374 mm | %44,7 | okumaların s = 0,837 mm'si, √5'e bölünmüş |
| d_cal | 0,300 mm | %28,7 | 0,6 / 2 |
| d_res | 0,289 mm | %26,6 | 0,5 / √3 |

Model yalnızca topladığı için tüm duyarlılık katsayıları 1'dir. Üç katkının nasıl birleştiğine dikkat edin: 0,374 + 0,300 + 0,289 = 0,963, ama √(0,374² + 0,300² + 0,289²) = 0,560. Belirsizlikler doğrudan toplanmaz; kareleri toplanır.

**Monte Carlo bölümü** biraz daha büyük bir standart belirsizlik verir (yaklaşık 0,67 mm). Şimdilik bunu dert etmeyin: [7. bölüm](07-monte-carlo#table) nedenini tam da bu örnekle açıklar.

## 7. adım: ölçümü iyileştirin {#step-improve}

Bütçe ne yapacağınızı söyler. En büyük katkı okumalardan (L_r) gelir. Beş yerine on kez ölçmek onların standart belirsizliğini √2'ye böler. Daha iyi sertifikalı bir şerit d_cal'ı küçültür. Deneyin: **Model** sekmesinde beş okuma daha ekleyin ve yeniden **Hesapla**'ya basın.

> **Deneyin:** Sabit bir kapsam faktörü yerine bir kapsam olasılığı isteyin. **Ayarlar**'da **Kapsam olasılığı (k, t dağılımından)** seçin ve `0,95` girin. Program k'yi etkin serbestlik derecesinden seçer (ν_eff = 20 için k = 2,09) ve U biraz büyür.

## Neleri dışarıda bıraktık? {#left-out}

Gerçek bir uzunluk ölçümü sıcaklığı da (çelik bir şerit metre başına ve derece başına yaklaşık 0,012 mm uzar), şeridin hizalanmasını ve kenarların düz olup olmadığını da hesaba katar. Her biri modelin bir girdisi daha olurdu. Kütüphanedeki [kumpas](example:ea402-s10-vernier-calliper) ve [mastar blok](example:ea402-s4-gauge-block-50mm) örnekleri bu tür etkileri gösterir.

## Kaydedin ve raporlayın {#save}

- **Kaydet** (disk simgesi) projeyi daha sonra **Proje dosyası aç** ile açabileceğiniz bir `.json` dosyası olarak saklar.
- Menü ⋮ raporu sağ üstte gösterilen dilde (EN/TR) **PDF**, **HTML** ya da **Markdown** olarak dışa aktarır.

[5. bölüm](05-where-numbers-come-from) şimdi her türlü bilginin bir standart belirsizliğe nasıl dönüştürüleceğini gösteriyor.
