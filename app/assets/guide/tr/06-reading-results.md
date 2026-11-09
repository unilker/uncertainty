# 6. Sonuçları okumak

**Hesapla**'dan sonra **Sonuçlar** sekmesi programın hesapladığı her şeyi gösterir. Bu bölüm bir ölçüm modeli için sonuçları yukarıdan aşağıya inceler. Diğer proje türlerinin kendi sonuçları vardır ve [9.](09-laboratory-data) ile [10. bölümlerde](10-special-evaluations) açıklanır, ama fikirler aynıdır.

## Raporlanan sonuç {#reported}

En üstteki mavi kutu, bir rapora ya da sertifikaya kopyalanacak cümleyi içerir; örneğin:

```
m_X = (25 ± 59) mg; genişletilmiş belirsizlik, birleşik standart belirsizliğin
k = 2,00 kapsam faktörüyle çarpılmasıyla elde edilmiştir ve yaklaşık %95 kapsam
olasılığına karşılık gelir.
```

- Belirsizlik **Ayarlar**'da seçilen anlamlı basamak sayısına (varsayılan 2) yuvarlanır ve değer aynı sayıda ondalıkla verilir.
- Cümle yuvarlama kuralını belirtir. Varsayılan, EA-4/02 kuralıdır: olağan yuvarlama, U'yu %5'ten fazla küçültüyorsa yukarı yuvarlama.
- "Yaklaşık %95": k = 2 ile normal dağılım için olasılık tam olarak %95,45'tir, ama olasılık hiçbir zaman bu kadar kesin bilinmez.

Sonuçların üstünde kırmızı *Kaydedilmemiş değişiklikler* satırını görüyorsanız, son hesaplamadan sonra projeyi değiştirmişsinizdir ve sonuçlar önceki sürüme aittir: yeniden **Hesapla**'ya basın.

## GUM sonucu {#gum}

| Satır | Anlamı |
|---|---|
| **Tahmin** | girdilerin tahminlerinden bulunan sonuç değeri |
| **Birleşik standart belirsizlik** | u_c, tüm girdiler birlikte |
| **Etkin serbestlik derecesi** | ν_eff; tüm girdiler serbestlik derecesi verilmemiş B tipiyse ∞ |
| **Kapsam faktörü** | k; **Ayarlar**'da sabitlenmiş ya da bir kapsam olasılığı için ν_eff'ten seçilmiş |
| **Kapsam olasılığı** | k'ye ait olasılık |
| **Genişletilmiş belirsizlik** | U = k · u_c, yuvarlanmadan önce |

Model belirgin biçimde doğrusal değilse ve **Ayarlar**'da **İkinci mertebe terimler** seçeneğini açtıysanız, tahmin ve u_c, GUM'un 2026 değişikliğinin istediği gibi ikinci mertebe terimleri içerir.

## Belirsizlik bütçesi {#budget}

Her girdi için bir satır:

| Sütun | Anlamı | Neye bakılır |
|---|---|---|
| Tahmin | girdinin değeri | kastettiğiniz değer mi? |
| Standart belirsizlik | girdinin kendi birimindeki u'su | U, k'ye bölündü mü? |
| Serbestlik derecesi | girdinin ν'sü | küçük değerler k'yi büyütür |
| Duyarlılık katsayısı | c, sonucun girdiye tepkisi | işareti ve büyüklüğü mantıklı olmalı |
| Katkı | c · u, sonucun biriminde | girdileri karşılaştırın |
| Varyans payı | (c · u)² / u_c², yüzde olarak | **en yararlı sütun** |

Varyans payı belirsizliğin nereden geldiğini söyler. Bir girdinin payı %80 ise iyileştirilecek yer orasıdır; %2'lik bir girdiyi on kat iyileştirmek neredeyse hiçbir şeyi değiştirmez.

> **Dikkat:** Tam olarak 0 olan bir duyarlılık katsayısı, girdiğiniz tahminlerde sonucun birinci mertebede o girdiyle değişmediği anlamına gelir. Program o zaman sizi uyarır. Bu doğru olabilir (bir dönüm noktasındaki düzeltme) ya da yanlış bir modelin işareti olabilir; bu durumlarda Monte Carlo sonucunu ya da ikinci mertebe terimleri kullanın.

*Birleşik varyansa katkılar* grafiği aynı payları çubuklar olarak gösterir.

**Kragten tablolama görünümü** (**Ayarlar**'da bir seçenek) aynı bütçeyi bir hesap tablosunda hesaplandığı gibi gösterir: her girdi belirsizliği kadar kaydırılır ve sonuçtaki değişim kaydedilir. Var olan bir hesap tablosuyla karşılaştırırken işe yarar.

## Monte Carlo sonucu {#mcm}

Monte Carlo yöntemi açıksa (**Ayarlar**), ikinci bir blok gelir:

- **Deneme sayısı**: kaç rastgele çekiliş yapıldığı. *Uyarlamalı* deneme sayısıyla program, sonuçlar istenen basamak sayısında kararlı olana kadar devam eder.
- Benzetilmiş sonuçların **Tahmin (ortalama)** ve **Standart belirsizlik (standart sapma)** değerleri.
- **Kapsam aralığı**: benzetilmiş sonuçların %95'ini (ya da seçtiğiniz olasılığı) içeren aralık. *En kısa*, böyle aralıkların en darıdır; *olasılıksal simetrik*, her iki tarafta %2,5 bırakır. Asimetrik bir sonuçta ikisi farklıdır ve tahmin ortada değildir.
- Karşılaştırma için GUM aralığıyla birlikte benzetilmiş sonuçların bir **histogramı**.

**GUM sonucunun doğrulanması.** Program GUM aralığının uçlarını Monte Carlo aralığıyla karşılaştırır. *Doğrulandı*, daha basit olan GUM sonucunun istenen basamak sayısında yeterince iyi olduğu anlamına gelir. *Doğrulanmadı*, ikisinin farklı olduğu anlamına gelir: o zaman Monte Carlo sonucunu tercih edin ya da nedenini bulun ([7. bölüm](07-monte-carlo#disagree)).

## Referans değerlerle karşılaştırma {#checks}

Kütüphane örnekleri için (ve beklenen değerleri girdiyseniz kendi projeleriniz için) bir tablo, hesaplanan her değeri kaynakta yayımlanmış değerle toleransıyla birlikte karşılaştırır. *başarılı*, programın kaynağı basılı sayıların yuvarlaması içinde yeniden ürettiği anlamına gelir. Bir kaynak hata içeriyorsa referans değer bağımsız bir yeniden hesaptır ve **Genel** sekmesindeki **Notlar** farkı açıklar.

## Uyarılar {#warnings}

Uyarılar denetlemeniz gereken şeyleri gösterir: sıfır bir duyarlılık katsayısı, fiziksel bir sınırın dışında bir sonuç, başarısız bir doğrulama, çok az Monte Carlo denemesi. Bir uyarı hata değildir, ama hiçbirini anlamadan geçmeyin.

## Sonuçlarla ilgili sık sorular {#questions}

**Tahmin beklediğim gibi değil.** Her girdinin birimini ve model denklemini denetleyin. Unutulmuş bir dönüşüm (mL ve L, % ve kesir) en sık nedendir.

**U beklediğimden çok daha büyük.** Varyans payına bakın: genellikle bir girdi fazla büyük girilmiştir; örneğin standart belirsizlik yerine genişletilmiş belirsizlik ya da yanlış birimde bir değer.

**GUM ve Monte Carlo sonuçları farklı.** [7. bölüme](07-monte-carlo#disagree) bakın.

> **Deneyin:** [Kütle kalibrasyonu (birinci mertebe GUM)](example:jcgm101-9.3-mass-calibration) örneğini açın, **Hesapla**'ya basın ve GUM ile Monte Carlo standart belirsizliklerini karşılaştırın. Sonra aynı örneği [ikinci mertebe terimlerle](example:jcgm101-9.3-mass-calibration-higher-order) açın ve GUM sonucunun Monte Carlo'ya nasıl yaklaştığını görün.
