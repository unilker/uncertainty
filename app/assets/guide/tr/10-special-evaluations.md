# 10. Özel değerlendirmeler

Bu bölüm kalibrasyon eğrilerini, uygunluk kararlarını, birden çok sonuçlu modelleri, tartı aletlerini ve uzlaşı değerlerini ele alır. Her kısım tek başına okunabilir.

## İki eksende de belirsizlikli kalibrasyon eğrileri {#calibration}

**Ne zaman.** Bir analizör (gaz analizi, spektrometri), içerikleri kendileri de belirsiz olan referans karışımlar kullanılarak bir içeriğe dönüştürülmesi gereken bir yanıt verir. ISO 6143 bunu anlatır.

**Fikir.** Sıradan en küçük kareler doğrusu yalnızca yanıtların belirsiz olduğunu varsayar. Burada hem içeriklerin (x) hem yanıtların (y) belirsizliği vardır; bu yüzden program *genelleştirilmiş en küçük kareler* kullanır: her noktanın belirsizliğine göre iki eksen boyunca da hareket etmesine izin vererek noktalara en iyi uyan eğriyi bulur.

**Neye ihtiyacınız var.** Her referans karışım için: u(x) ile içeriği x ve u(y) ile ölçülen yanıtı y. Her bilinmeyen numune için: yanıtı ve u(y).

**Nasıl girilir.** **Veriler** sekmesinde noktaları (her satıra bir) girin, analiz fonksiyonunu (doğru, ikinci ya da üçüncü dereceden polinom, kuvvet ya da üstel) seçin ve numuneleri girin. Referans karışımlar korelasyonluysa (örneğin aynı ana gazdan yapılmışsa) kovaryanslarını girin.

**Ne elde edersiniz.**

- fonksiyonun parametreleri, belirsizlikleri ve kovaryansları;
- **Γ** (en büyük ağırlıklı artık) ve artık kareler toplamı: Γ ≤ 2 ise fonksiyon uyar; değilse başka bir fonksiyon deneyin;
- eğrinin belirsizliğini de içeren belirsizliğiyle her numunenin içeriği.

> **Örnek:** [Bir doğru](example:iso6143-b1-no-uv-linear), [doğru ya da ikinci dereceden polinom](example:iso6143-b2-n2-natural-gas-quadratic), [korelasyonlu referans karışımlar](example:iso6143-b2-n2-natural-gas-covariances).

## Uygunluk değerlendirmesi {#conformity}

**Ne zaman.** Bir ürünün, bir numunenin ya da bir cihazın bir spesifikasyona uyup uymadığına karar vermelisiniz: yasal bir sınırın altında, bir tolerans içinde, izin verilen en büyük hata içinde. JCGM 106 ve Eurachem/CITAC uygunluk kılavuzu bunun nasıl yapılacağını anlatır.

**Sorun.** Bir sınıra yakın ölçülmüş bir değer, belirsizlik yüzünden sınırın her iki tarafında da olabilir. Yalnızca ölçülen değere göre karar vermek, bazı kötü ürünleri kabul etmek ve bazı iyileri reddetmek demektir.

**Araçlar.**

- **Uygunluk olasılığı**: ölçülen değer ve belirsizliğinden, gerçek değerin sınırlar içinde olma olasılığı. Örneğin %97.
- **Karar kuralı**: ölçmeden önce üzerinde anlaşılır. *Basit kabul*: ölçülen değer içerideyse kabul. *Korumalı kabul*: yalnızca bir **koruma bandı** kadar daraltılmış daha küçük bir *kabul bölgesi* içinde kabul; bu müşteriyi korur. *Korumalı ret*: yalnızca açıkça dışarıdaysa ret; bu üreticiyi ya da yasal denetim ve doping kontrolünde suçlananı korur.
- **Özgül risk**: bu tek ürün için yanlış bir karar olasılığı.
- **Genel riskler**: üretim süreci bilindiğinde, bütün üretim üzerinden, kabul edilen bir ürünün gerçekte kötü (müşteri riski) ya da reddedilen bir ürünün gerçekte iyi (üretici riski) olma olasılığı.

**Nasıl girilir.** Ya ayrı bir proje olarak (**Yeni uygunluk değerlendirmesi**: sınırları, ölçülen değerleri ve belirsizliklerini girin) ya da bir ölçüm modelinin üstüne (**Ayarlar → Bir toleransa uygunluğu değerlendir**). Karar kuralını ve koruma bantları için gerekli olasılığı ya da riski seçin.

**Test belirsizlik oranı.** Kalibrasyonda kabul sınırları çoğu zaman toleransın belirsizliğe oranından (TUR) türetilir. Program bilinen koruma bandı yöntemlerini (kareler toplamının karekökü, Dobbert'in yönetilen koruma bandı, NCSLI RP-10) sunar ve yol açtıkları riskleri gösterir.

> **Örnek:** [Bir Zener diyotun uygunluk olasılığı](example:jcgm106-7.3-zener-diode), [korumalı retle hız sınırı](example:jcgm106-8.3-speed-limit), [çelikte nikel için korumalı kabul](example:eurachem-compliance-b1-nickel-steel), [TUR'dan koruma bandı](example:tur-guard-band-dobbert-tur2-5).

> **Dikkat:** Karar kuralı müşteriyle üzerinde anlaşılmalı ve raporda belirtilmelidir (ISO/IEC 17025 7.1.3 ve 7.8.6). Sonucu gördükten sonra seçmek kabul edilemez.

## Birden çok sonuçlu modeller {#multivariate}

**Ne zaman.** Bir model aynı anda birkaç sonuç verir: bir gaz karışımının tüm bileşenlerinin miktar kesirleri, bir empedansın direnci ve reaktansı, bir eğrinin parametreleri. Girdileri ortak olduğu için sonuçlar **korelasyonludur** ve bu, daha sonra birlikte kullanıldıklarında önem taşır.

**Ne elde edersiniz** (JCGM 102):

- standart belirsizliğiyle her sonuç;
- **kovaryans matrisi** ve sonuçlar arasındaki korelasyon katsayıları;
- iki ayrı aralık yerine bir **kapsam bölgesi**: iki sonuç için bir elips (GUM) ya da Monte Carlo ile bulunan en küçük bölge.

**Nasıl girilir.** Bir ölçüm modeli gibi, ama her birinin kendi denklemi olan bir çıktı listesiyle. Denklemler sonuçlara göre çözülemiyorsa (*örtük* bir model), **Örtük model h(Y, X) = 0** seçeneğini açın ve her çıktıya bir başlangıç değeri verin: program denklemleri sayısal olarak çözer.

> **Örnek:** [Ortak girdilerden iki çıktı](example:jcgm102-9.2.2-additive-gaussian), [Kartezyen koordinattan kutupsal koordinata](example:jcgm102-9.3-polar-x010), [sekiz bileşenli gravimetrik bir gaz karışımı](example:gum5-e4-gravimetric-gas-mixture).

## Tartı aletleri {#weighing}

**Ne zaman.** Otomatik olmayan bir tartı aletini (terazi, baskül, kantar) EURAMET cg-18'e göre kalibre ediyorsunuz ve her yükteki hatayı, belirsizliğini ve kullanımdaki bir tartım sonucunun belirsizliğini istiyorsunuz.

**Fikir.** Terazi standart ağırlıklarla birkaç yükte denenir. Her yükte **hata** E = gösterge − referans kütle; tekrarlanabilirlikten, göstergenin yuvarlamasından (sıfırda ve yükte), eksantriklikten, ağırlıklardan ve hava kaldırma kuvvetinden gelen bir belirsizlikle bulunur. Hatalardan geçen bir doğru (**hata yaklaşım fonksiyonu**) sonra her yükte hatayı ve belirsizliğini verir.

**Nasıl girilir.** **Veriler** sekmesinde her deney yükü için göstergeyi, hatayı ve standart belirsizlikleri girin; doğrunun sıfırdan geçip geçmediğini seçin. Terazinin kullanımı için aralıkları, eksantrikliği, sıcaklık etkisini ve minimum tartım için gerekliliği ekleyin.

**Ne elde edersiniz.** Doğrunun katsayıları ve belirsizlikleri, bir χ² testi ve bir artık testi (doğru uyuyor mu?), her okumada hata ve belirsizlik, kullanımdaki bir tartım sonucunun belirsizliği ve **minimum tartım**.

> **Örnek:** [220 g'lık bir laboratuvar terazisi](example:cg18-h1-a-weighing-result), [bir kantar](example:cg18-h3-a-weighing-result), [bir hata yaklaşım fonksiyonu](example:cg18-h4-400g-error-function).

## Uzlaşı değerleri ve eşdeğerlik dereceleri {#consensus}

**Ne zaman.** Birkaç laboratuvar (ya da deney) aynı büyüklüğü her biri kendi belirsizliğiyle ölçtü ve birleşik tek bir değer gerekiyor: bir anahtar karşılaştırmanın referans değeri, bir laboratuvarlar arası karakterizasyondan bir sertifikalı değer, bir fiziksel sabit.

**Sorun.** Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır. Laboratuvarlar arasındaki bu fazladan yayılmaya *karanlık belirsizlik* denir. Basit bir ağırlıklı ortalama o zaman çok fazla küçük belirsizlikli bir değer verir.

**Yöntemler** (NIST Consensus Builder'daki gibi):

| Yöntem | Fikir |
|---|---|
| **DerSimonian–Laird** | rastgele etkiler modeli: fazladan yayılma τ'yu tahmin eder ve her laboratuvarın belirsizliğine ekler; belirsizlik için Knapp–Hartung düzeltmesi ya da parametrik bootstrap ile |
| **Linear Pool** | tüm laboratuvarların dağılımlarını karıştırır; çok dayanıklı, belirsizliği büyük |
| **Hiyerarşik Bayes** | önsel dağılımlarla rastgele etkiler, bir ızgara üzerinde tam olarak hesaplanır (rastgele örnekleme yok) |
| **En çok olabilirlik** | en çok olabilirlikle tahmin edilen rastgele etkiler modeli |

**Eşdeğerlik dereceleri.** Her laboratuvar için genişletilmiş belirsizliğiyle D_j = x_j − x_ref: uzlaşı değerinden ne kadar ve ne kadar anlamlı biçimde uzak olduğu. *Birini dışarıda bırakma* her laboratuvarı diğerlerinin uzlaşısıyla karşılaştırır; *ikili* eşdeğerlik dereceleri laboratuvar çiftlerini karşılaştırır.

> **Örnek:** [¹³⁷Cs yarı ömrü](example:koepke2017-half-life-cs137), [birkaç yöntemle kütle çekim sabiti](example:merkatas2019-newtonian-constant-g-shades), [19 laboratuvarın eşdeğerlik dereceleri](example:nicob-co60-activity-leave-one-out-bilateral).

> **Not:** Yöntemler aynı veri için oldukça farklı değerler ve belirsizlikler verebilir. Seçim değerlendirmenin bir parçasıdır, veriler görülmeden önce yapılmalı ve raporda belirtilmelidir.
