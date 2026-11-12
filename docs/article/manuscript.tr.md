# KAPSAM: uyguladığı kılavuzların çözümlü örnekleriyle doğrulanmış açık kaynak kodlu ölçüm belirsizliği yazılımı

*Depo kopyasının Türkçe çevirisi, 2026-11-12; bir yapay zekâ asistanıyla üç uzmanlık profilinde yazılan, üç turlu bir
gönderim öncesi incelemeden sonra gözden geçirilmiştir (araştırma günlüğü D110, D112, D113). Asıl metin İngilizce olan `docs/article/manuscript.md` dosyasıdır; bu çeviri gözden geçirme içindir ve
her değişiklikte İngilizce metinle birlikte güncellenir. Sayılar `docs/article/numbers.md`
(`tools/research/article_stats.py` ile üretilir), `docs/research/validation-matrix.md`, üretilen ek tablolar ve
araştırma günlüğünden (kararlar D…, bulgular F…) gelir; kaynakların künyeleri Crossref ile karşılaştırılmıştır.
Şekil 1 ve 3 ile Tablo 2–3 `tools/research/article_figures.py` ile üretilir; Şekil 2 uygulamanın bir ekran
görüntüsüdür.*

İlker Ün — TÜBİTAK National Metrology Institute (TÜBİTAK UME), 41470 Gebze, Kocaeli, Türkiye

## Özet

Akredite laboratuvarlar ölçüm belirsizliğini hesap tablolarıyla ya da Ölçümde Belirsizliğin İfadesi Kılavuzu (GUM)
ve eklerinin yalnızca bir bölümünü kapsayan programlarla değerlendirir; yöntemlerini kılavuz belgelerin çözümlü
örnekleri üzerinde öğrenir ve denetler. KAPSAM, Rust motoru, komut satırı aracı ve masaüstü ile mobil uygulaması olan
açık kaynak kodlu bir programdır. 2026 değişikliğiyle birlikte GUM'u, eklerinin Monte Carlo yöntemlerini, uygunluk
değerlendirmesini, yukarıdan aşağı değerlendirmeyi, yeterlilik testini, iki değişkende de belirsizlikli kalibrasyonu,
kesinlik deneylerini ve sektör kılavuzlarının diğer yöntemlerini uygular. Program 284 proje üzerinde doğrulanmıştır;
bunların 281'i, 25'i kılavuz belge ve standart olan 36 kaynağın çözümlü örneklerinden oluşturulmuştur. Test takımı
kodun her değişikliğinde 1841 referans değerinin tamamını yeniden hesaplar; Suncal 164 projenin, LibreOffice'te
yeniden hesaplanan elektronik tablo formülleri ise 187 projenin sonuçlarını yeniden üretir. Örneklerin yeniden
hesaplanması 102 bulgu ortaya çıkardı. Bunların on biri basılı bir nihai sonucu değiştiren doğrulanmış tutarsızlıklardır ve beşi sonucu
genişletilmiş belirsizliğinin onda birinden fazla değiştirir; kodu görmemiş bir metrolog, bir yapay zekâ asistanıyla
çalışarak bunları belgelerden yeniden üretmiştir. Çözümlü örneklerin makinece okunabilir girdilerle yayımlanması ve bağımsız yeniden hesaplamayla
denetlenmesi gerektiğini savunuyoruz.

**Anahtar sözcükler:** ölçüm belirsizliği; GUM; Monte Carlo yöntemi; yazılım doğrulaması; çözümlü örnekler; referans
verileri; ISO/IEC 17025

## 1. Giriş

ISO/IEC 17025, deney ve kalibrasyon laboratuvarlarından sonuçlarının belirsizliğini değerlendirmelerini (7.6),
yazılımlarını kullanmadan önce validasyondan geçirmelerini (7.11.2) ve hesaplamaları ile veri aktarımlarını sistematik
biçimde kontrol etmelerini (7.11.6) ister [1]. Değerlendirmenin kendisi *Ölçümde belirsizliğin ifadesi kılavuzu*
(GUM) ve eklerine dayanır: belirsizliğin yayılması yasası (LPU) [2], tek çıktı (JCGM 101) [3] ve birden çok çıktı
(JCGM 102) [4] için Monte Carlo yöntemi, uygunluk değerlendirmesinde belirsizliğin kullanımı (JCGM 106) [5] ve 2026'dan
bu yana doğrusal olmayan modeller üzerine bir GUM değişikliği [6] ile çözümlü örneklerden oluşan yeni bir bölüm
(GUM-5) [7]. Sektör kılavuzları bu çerçeveyi uygulamaya aktarır: analitik kimya için Eurachem/CITAC kılavuzları
[8, 9], çevre laboratuvarları için Nordtest el kitapları [10, 11], kalibrasyon için EA-4/02 [12], UKAS M3003 [13] ve
EURAMET kalibrasyon kılavuzları [14, 15], deneyler için ILAC G17 [16] ve yeterlilik testi [17], kesinlik
deneyleri [18, 19] ve gaz analizi [20] için ISO standartları.

Bu belgelerin her biri örnekle öğretir. Yeni bir yöntem için bütçe kuran bir laboratuvar sayılarını bir çözümlü
örnekle karşılaştırır; yazılımını doğrulayan bir laboratuvar da sonuçları basılı olan çözümlü örnekleri yeniden
çalıştırır. Her iki kullanım da basılı sonuçların basılı girdilerden çıktığını varsayar. Bizim deneyimimizde ve başka
programların geliştiricilerinin deneyiminde [23] bu çoğu zaman tam olarak böyle değildir, bazen de hiç değildir (Bölüm 6).

**Yazılım ve doğrulanması.** NIST Uncertainty Machine [21, 22] açık modelleri GUM ile ve Monte Carlo ile
değerlendirir; Sandia'nın bir Python paketi olan Suncal'ı [23, 24] buna sembolik LPU, eğri uydurma ve kalibrasyon için
karar riskini ekler; GUM Tree Calculator [25] ve METAS UncLib [26] "belirsiz sayıları" programlar boyunca yayar; GUM
Workbench [27] model tabanlı bütçeler için yaygın kullanılan ticari bir programdır; NIST Consensus Builder (NICOB)
[28] laboratuvarlar arası sonuçları birleştirir; MUkit [29] Nordtest'in yukarıdan aşağı değerlendirme yaklaşımını
uygular, bu yaklaşımın ISO karşılığı ISO 11352'dir [30]; MUPR hesap tablosu [31] kimya için yukarıdan aşağı
değerlendirmeyi uygular; UncertCalc [32] ise klinik laboratuvarlar için kalite kontrol verilerinin yukarıdan aşağı
değerlendirmesini aşağıdan yukarı bir bütçeyle birleştirir. Avrupa Metroloji Ağı Mathmet'in 35 programı kapsayan
taraması, programların yalnızca %54'ünün validasyona ilişkin bir kanıt sunduğunu ve JCGM 102 desteğinin en büyük
eksiklik olduğunu bulmuştur [33]. Bu tür programların doğrulanması daha önce de incelenmiştir. PTB'de GUM'u destekleyen
bir programın validasyonu GUM eklerindeki örneklerin yeniden üretilmesini de içermiş, birkaç yüz kara kutu test
durumu her çıktıyı değeri, anlamlı basamak sayısı ve yuvarlaması bakımından denetlemiştir [34]; Greif ve Schrepf,
yazılımların GUM'a uygunluğu için yeniden kullanılabilir test durumları türetmiş ve GUM'un sınanamayan veya belirsiz
ifadelerini listelemiştir [35]; TraCIM hizmeti de metroloji yazılımlarını, örneğin koordinat ölçme makinelerinin
değerlendirme yazılımlarını, sonuçları bilinen referans veri kümelerine karşı doğrular [36]. Çözümlü örneklerin
kendisindeki hatalar ise tek tek, yayımlanmış düzeltme listelerinde [37] ve başka programların yazarlarınca
[23] bildirilmiştir.

Bu çalışma bu yaklaşımı ölçek ve nitelik bakımından genişletir: 25 kılavuz belge ve standardın ve 11 başka kaynağın
çözümlü örnekleri, her referans değeri için belgelenmiş bir toleransla makinece okunabilir projelere dönüştürülür, kodun
her değişikliğinde yeniden hesaplanır, bağımsız uygulamalarla karşılaştırılır ve her tutarsızlık sınıflandırılır.
Katkıları, aynı zamanda makalenin yapısını da veren şu üç başlıktır:

1. **Program, KAPSAM** (Bölüm 2 ve 3): GUM ailesini ve başlıca sektör kılavuzlarını kapsar, her algoritmanın
   dayandığı maddeyi belirtir ve her sonucu girdilerine, yöntemine ve yazılımın tam derlemesine kadar izler.
2. **Doğrulanması** (Bölüm 4 ve 5): 1841 referans değerli 284 kendi kendini denetleyen proje, referans değerlerin ve
   toleransların kendisinin denetimi, bağımsız bir programla, elektronik tablo formülleriyle ve Monte Carlo yayılımıyla
   karşılaştırmalar.
3. **Yeniden hesabın çözümlü örnekler hakkında ortaya koydukları** (Bölüm 6): türüne ve basılı sonuca etkisine göre
   sınıflandırılmış 102 bulgu; doğrulanmış tutarsızlıklar kodu görmemiş bir kişi tarafından, bir yapay zekâ
   asistanıyla çalışılarak yeniden hesaplanmıştır.

Bölüm 7 programın ISO/IEC 17025 gerekliliklerine nasıl hizmet ettiğini özetler, Bölüm 8 doğrulamanın neyi gösterip
neyi göstermediğini tartışır, Bölüm 9 sonuçları verir.

## 2. Değerlendirme yöntemleri

### 2.1 Genel bakış

Terimler Uluslararası Metroloji Sözlüğü (VIM) [38] ve GUM [2] ile uyumludur. Bir proje, on bir değerlendirme türünden
birinin girdi verilerini içerir (Tablo 1). Her tür, kılavuz belgesinin belirtilen maddelerini uygular; belgenin açık
bıraktığı bir seçim projenin bir seçeneğidir ve raporda adıyla belirtilir. Türler, Désenfant ve Priel'in ayırdığı
yolları kapsar: başvuru yöntemi saydıkları GUM'un modelleme yaklaşımı ile tek laboratuvar validasyon verilerinden,
laboratuvarlar arası kesinlik çalışmalarından ve yeterlilik testinden giden yaklaşımlar [39]; bunlar, bazı girdi
büyüklüklerinin birleştirildiği, ölçüm modelinin bir sadeleştirmesi olan istatistiksel bir modele dayanır [40]. Bu
nedenle bir laboratuvar bir yöntemi aynı programla birden çok yoldan değerlendirebilir; modelden gelen belirsizliğin
laboratuvarlar arası karşılaştırmalarda görülen saçılımdan çok küçük olması eksik bir bileşene işaret eder [39].
Kılavuzların vermediği ya da farklı verdiği denklemler Ek A'dadır.

**Tablo 1.** Değerlendirme türleri ve uyguladıkları yöntemler.

| Proje türü | Yöntemler | Başlıca kaynaklar |
|---|---|---|
| ölçüm modeli | korelasyonlu ve topluluklu (ensemble) birinci ve ikinci mertebe LPU; Welch–Satterthwaite; Kragten tablosu; Monte Carlo (uyarlamalı, en kısa ve simetrik aralıklar, kestirim etrafında yarı genişlik); GUM sonucunun doğrulanması; sınırlı çıktılar | [2, 3, 6, 41] |
| birden çok çıktı | birden çok çıktı için LPU ve Monte Carlo, açık ve örtük modeller, kapsama bölgeleri | [4] |
| uygunluk değerlendirmesi | özgül ve küresel riskler, koruma bantları (JCGM 106; test belirsizlik oranı), lognormal süreçler, çok bileşenli ürünler | [5, 42, 43] |
| yukarıdan aşağı değerlendirme | validasyon ve kalite kontrol verilerinden kesinlik ve geri kazanım (Nordtest, MUPR, ISO 21748) | [9, 10, 44] |
| yeterlilik testi | sağlam istatistikler, puanlar, homojenlik ve kararlılık (birimler arası Bayesçi model dahil) | [17, 45, 46] |
| kalibrasyon fonksiyonu | iki değişkende de belirsizlikli genelleştirilmiş en küçük kareler | [20] |
| kesinlik deneyi | aykırı değer testleriyle tekrarlanabilirlik ve tekrarüretilebilirlik | [18, 19] |
| kontrol grafiği | X ve R grafikleri, sınırlar, kurallar, uzun dönem değerlendirmesi | [11] |
| numune alma | ikili numune yöntemi (klasik ANOVA; aralık ve logaritmik dönüşümlü türevler), temel numune alma hatası; kılavuzun sağlam ANOVA'sı metninden yeniden üretilemiyor (F63) | [47] |
| gösterge hatası | tartı aletlerinin hata yaklaşımı ve kullanımdaki belirsizliği | [14] |
| uzlaşı değeri | parametrik bootstrap ile DerSimonian–Laird, Linear Pool, en çok olabilirlik, Bayesçi rastgele etkiler (Gauss veya Laplace); eşdeğerlik dereceleri (MRA, birini dışarıda bırakma, ikili) | [28, 48, 49] |

### 2.2 Ölçüm modelleri ve GUM

Modeller ifade olarak yazılır; ayrıştırıcı bunların türevlerini sembolik olarak alır, böylece duyarlılık katsayıları
kesindir ve yazdırılabilir. Birleşik standart belirsizlik korelasyonları içerir. GUM'un 2026 değişikliğindeki [6]
kestirimin ikinci mertebe terimi her zaman hesaplanır ve istenirse eklenir; bu durumda varyans, Mana ve Pennecchi'nin
[41] korelasyonlu normal girdiler için ikinci ve üçüncü mertebe terimlerini de içerir (Ek A.1). Aynı verilerden
değerlendirilen girdiler bir *topluluk* (ensemble) olarak tanımlanabilir; topluluk Welch–Satterthwaite formülüne
ortak serbestlik derecesiyle tek bir bileşen olarak girer (Willink [50, Bölüm 4.1], Willink ve Hall'un özetlediği
biçimiyle [51, Bölüm 4]; GUM Tree Calculator aynı kuralı uygular [25]; Ek A.2). Genişletilmiş belirsizlik EA-4/02'nin
kuralıyla (U'yu %5'ten fazla düşürmedikçe olağan yuvarlama, aksi halde yukarı) [12], Eurachem/CITAC MUPR kılavuzunun
kuralıyla [9] ya da her zaman yukarı yuvarlanır; sonuç ifadesi kuralı adıyla belirtir. Kalibrasyon raporlarında kalibrasyon ve ölçüm yeteneği (CMC) alt sınır olarak uygulanabilir, çünkü
raporlanan belirsizlik CMC'den küçük olamaz (ILAC P14 [52]).

### 2.3 Monte Carlo yöntemi

JCGM 101 7.9'un uyarlamalı yöntemi, kestirim, standart belirsizlik ve kapsama aralığının iki ucu belirtilen sayısal
toleransta kararlı hale gelince durur; rapor bu toleransı verir. Deneme sayısı sabitse rapor sayıyı ve tohumu verir.
Program, olasılıkça simetrik ve en kısa kapsama aralığının yanı sıra kestirim etrafında simetrik aralığın yarı
genişliğini de, Pr(|Y − y| ≤ U) = p, raporlar. GUM sonucu Monte Carlo sonucuyla doğrulanır (JCGM 101 madde 8);
tekrarlanan gözlemlerden gelen girdiler varsa rapor, iki değerlendirmenin az serbestlik derecesinde farklı girdi
varyansları kullandığını (s/√n'ye karşı √(ν/(ν − 2))·s/√n) ve bu nedenle başarısız bir doğrulamanın hata anlamına
gelmeyebileceğini belirtir.

### 2.4 Uygunluk değerlendirmesi ve yeterlilik testi

Uygunluk değerlendirmesi her sonucun uygunluk olasılığını ve bir süreç dağılımı için JCGM 106'nın küresel tüketici ve
üretici risklerini verir [5]; karar kuralı ve varsa koruma bandı, ILAC G8'in önerdiği gibi raporda belirtilir
[43]. Koruma bantları test belirsizlik oranından, TUR = (T_U − T_L)/(2U₉₅), türetilebilir; buna Dobbert'in
yönetilen koruma bandı da [53] dahildir; bu bandın kabul sınırları, çarpan sınırlanmazsa TUR = 4,59'un üzerinde
toleransın dışına düşer. Program varsayılan olarak U₉₅ = k₉₅u kullanır (Suncal 2u kullanır), 2u'yu seçenek olarak
tutar ve kabul edilmiş bir kural daha fazlasına izin vermedikçe çarpanı 1 ile sınırlar. Bu tür kurallar
uzlaşımlardır ve Pendrill [54] bunları ekonomik açıdan inceler; program maliyetleri eniyilemez, ancak böyle bir
eniyilemenin gerektirdiği riskleri raporlar. Yeterlilik testi için program ISO 13528'in sağlam istatistiklerini,
puanlarını ve homojenlik denetimlerini hesaplar [17]; beyan edilen belirsizlikleri de kullanan Carobbi'nin
değiştirilmiş A Algoritması [55] uygulanmamıştır.

### 2.5 Uzlaşı değerleri ve fiziksel bir sınıra yakın sonuçlar

Uzlaşı modülü NIST Consensus Builder'ı [28, 49] izler ve açık olan R kaynak koduyla karşılaştırılmıştır. Shirono ve
arkadaşları birbiriyle tutarsız sonuçların iki nedenini ayırır [56]: laboratuvarların çoğunu etkileyen ve karanlık
belirsizlikli bir rastgele etkiler modeli gerektiren ortak bir etki [48] ve aykırı laboratuvarlar. Modül rastgele
etkiler modellerini sağlar; aykırı laboratuvarlar bu paragrafın sonunda anlatılan, birini dışarıda bırakan
eşdeğerlik dereceleriyle belirlenir ve hiçbir sonuç kendiliğinden dışlanmaz. Modül DerSimonian–Laird kestirimini [57]
Knapp–Hartung düzeltmesiyle [58] ve laboratuvarlar arası varyansı Biggerstaff ve Tweedie'nin Cochran Q'su için gama
yaklaşımından [59] çeken NICOB parametrik bootstrap'iyle verir. Kodun okunması, kılavuzunun belirtmediği bir ayrıntıyı,
yaklaşımın hangi noktada değerlendirildiğini ortaya koydu (Ek A.3); program kodu izler. Laboratuvarlar arası standart
sapma üzerinde yarı Cauchy önselli hiyerarşik Bayesçi model MCMC yerine deterministik ızgaralarda değerlendirilir
(Ek A.4). Tek taraflı ve ikili eşdeğerlik dereceleri, CIPM MRA sürümünde ve belirsizliği NICOB'daki gibi Röver ve
arkadaşlarının değiştirilmiş Knapp–Hartung düzeltmesini [60] kullanan birini dışarıda bırakma sürümünde verilir.

Bir kütle kesri 1 g/g'yi aşamaz. y ± U böyle bir sınırı aştığında program aralığı sınırda keser ve sonucu, saflık
atamasına ilişkin IUPAC teknik raporunun izin verdiği gibi [61], y +U_U/−U_L olarak ifade eder; Monte Carlo yöntemiyle
ölçülen büyüklük üzerinde dikdörtgen bir önsele karşılık gelen [62], sınırla kısıtlanmış dağılımı da ortalaması,
standart sapması ve kapsama aralığıyla raporlar. Cowen ve Ellison doğal sınırlara yakın sonuçlar için bu seçenekleri
karşılaştırır [63]; rapor hangisinin kullanıldığını belirtir.

## 3. Yazılım tasarımı ve gerçekleştirimi

KAPSAM (kapsama faktörü ve kapsama aralığındaki *kapsam*) tek bir motor etrafında düzenlenmiştir; böylece her arayüz
aynı sayıları verir (Şekil 1). Motor, proje dosyalarını okuyan, değerlendirmeleri çalıştıran, sonuçları referans
değerlerle denetleyen ve raporları yazan bir proje katmanı (`project`, yaklaşık 19 000 satır) ile bir Rust
kütüphanesidir (`core`, yaklaşık 13 000 satır); komut satırı aracı (`cli`) toplu işlere hizmet eder, bir C arayüzü
(`ffi`) de motoru Windows, Linux, macOS, Android ve iOS için Flutter ile yazılmış bir uygulamaya (yaklaşık 9 000
satır Dart) bağlar. Programın ağ kodu yoktur: tüm hesaplamalar kullanıcının bilgisayarında yapılır.

<!-- fig1 -->

Bir proje, bir değerlendirmeyi yinelemek için gereken her şeyi tutan bir JSON dosyasıdır: iki dilli üstveri (başlık,
açıklama, notlar ve bölüm ile bağlantı içeren kaynaklar), bir değerlendirme türünün girdi verileri ve isteğe bağlı
olarak toleranslı referans değerleri ile bunların nereden geldiğini belirten bir ifade. İlk Eurachem örneğinin dosyası,
kısaltılmış olarak:

```
{ "format": "uncertainty-calculations/project", "version": 1,
  "metadata": { "title": { "en": "Preparation of a cadmium calibration standard", "tr": "…" },
                "sources": [ { "document": "EURACHEM/CITAC Guide CG 4 (QUAM:2012.P1)",
                               "section": "Example A1, Tables A1.2 and A1.3" } ] },
  "model": { "output": { "name": "c_Cd", "unit": "mg/L", "expression": "1000 * m * P / V" },
             "inputs": [ { "name": "P", "distribution":
                             { "type": "rectangular", "value": 0.9999, "half_width": 0.0001 } },
                         { "name": "m", "unit": "mg", "distribution":
                             { "type": "normal", "mean": 100.28, "sd": 0.05 } }, … ] },
  "analysis": { "gum": { "coverage_factor": 2 }, "mcm": { "trials": 1000000, "seed": 1 } },
  "expected": { "gum": { "standard_uncertainty": { "value": 0.8633, "tolerance": 0.001 } },
                "reference": { "en": "QUAM:2012 Table A1.3 (spreadsheet value u = 0.8633 mg/L).", … } } }
```

Kılavuz hesap tablosu (Kragten) değeri 0,8633 mg/L'yi basar; analitik değer 0,8637 mg/L'dir; bu nedenle tolerans son
basamağın yarım biriminden geniştir ve nedeni dosyada belirtilmiştir (Bölüm 4.2). Şema bilinmeyen alanları reddeder;
böylece yanlış yazılmış bir seçenek sessizce yok sayılmak yerine hata verir. Sonuçlar dosyada saklanmaz; girdilerden
yeniden hesaplanır. Referans değerleriyle bir proje kendi test durumudur: `kapsam check` projeyi değerlendirir ve her
sonucu referansıyla karşılaştırır.

Birinci ve ikinci mertebe türevler sembolik olarak alınır ve değerlendirmeden önce sadeleştirilir. Rastgele sayı
üreteci, JCGM 101'in önerdiği ve birincil kaynağıyla karşılaştırılmış geliştirilmiş Wichmann–Hill üretecidir [3, 64];
normal sayılar kesin olan Box–Muller dönüşümünden, gama sayıları Marsaglia ve Tsang yönteminden [65] gelir. Birden çok
çıktılı bir modelde her deneme bütün çıktıları ortak girdi dağılımının tek bir çekilişinden hesaplar. Tüm hesaplar çift
duyarlıkta yapılır; değerler yalnızca raporlanan ifadede yuvarlanır.

Yazılımın kalite güvencesi: biri 284 kütüphane projesinin tamamını değerlendiren 224 motor testi ve uygulamanın 28
testi, kaynak kodun her değişikliğinde bir biçim denetimi, her uyarıyı hata sayan bir statik analiz ve elektronik
tablo doğrulama dosyalarının LibreOffice Calc'ta [66] yeniden hesaplanmasıyla birlikte çalışır (sürekli
tümleştirme sunucusunda 26.2 sürümü; ek malzeme 24.2 sürümüyle üretilmiştir ve iki sürüm de karşılaştırılan 4631
değerin tamamını tolerans içinde verir). Bölüm 2 ve 3 ile Ek A'da programın ne yaptığına ilişkin her ifade, deponun bir
tablosunda onu gerçekleştiren kod ve test ile eşlenmiştir; hiçbir değerlendirmenin kullanmadığı açık fonksiyonların da
birim testleri vardır. Beş platformun paketleri aynı commit'ten derlenir. Her rapor program sürümünü, derlendiği git
commit'ini ve tarihini, derleyiciyi ve değerlendirilen projenin SHA-256 özetini verir (ISO/IEC 17025 7.5, 7.11).
Kaynak kod, kütüphane projelerinin üreteçleri, yeniden hesap betikleri ve elektronik tablo dosyalarının formülleri,
yöntemleri ve belgeleri seçen, referans değerlerini ve toleransları belirleyen ve sonuçları gözden geçiren yazarın
yönlendirmesiyle bir yapay zekâ kodlama asistanıyla (Claude, Anthropic) yazılmıştır; doğruluk bu nedenle asistanın
üretmediği değerlerle karşılaştırmalara dayanır ve Tablo 8 her denetimin programla neyi paylaştığını gösterir. Depo,
her tasarım kararının ve bulgunun araştırma günlüğünü, her kütüphane projesini kaynağının sayısal verilerinden yazan
üreteçleri ve her yeniden hesabın betiklerini içerir.

Uygulama, değerlendirme türlerinin kartları ve aranabilen, düzeye göre süzülebilen kütüphaneyle açılır. Bir projenin
beş sekmesi vardır: genel bakış, model veya veri düzenleyicisi, ayarlar, proje dosyasının kendisi ve sonuçlar
(Şekil 2). Arayüz ve raporlar İngilizce ve Türkçedir; raporlar Markdown, kendi içinde bütün HTML, PDF ve tüm değerleri
tam duyarlıkta veren JSON olarak yazılır.

<!-- fig2 -->

## 4. Doğrulama tasarımı

### 4.1 Kütüphane

Kütüphanede 284 proje vardır (Tablo 3). Birincil kaynakları, 253 projeyle 25 kılavuz belge ve standart — GUM, ekleri ve
GUM-5, Eurachem/CITAC, Nordtest, EA, UKAS ve EURAMET kılavuzları, beş ISO standardı, NIST TN 1900 [67] ile NIST
Uncertainty Machine ve Consensus Builder kılavuzları, EMPIR EMUE derlemesi [68] ve IUPAC kısa kılavuzu [69] —, 9
projeyle qNMR üzerine beş BIPM referans veri raporu [70] (dizinin altıncı raporu yalnızca F97 için yeniden
hesaplanmıştır), 19 projeyle beş makale ve bir kitap (Palma ve ark. [71],
Bettencourt da Silva [31], Koepke ve ark. [49], Merkatas ve ark. [72], Possolo ve Bodnar [45], Possolo ve Meija
[73]); test belirsizlik oranından koruma bantları üzerine üç proje, yayımlanmış koruma bandı formüllerinden bu çalışma
için kurulmuş sınama durumlarıdır (Bölüm 4.4). Bu makalenin doğrulama savları 25 kılavuz belge ve standarda (253 proje,
1670 referans değeri) dayanır; diğer kaynaklar test durumlarını genişletir. Alana göre 143 proje kimya, 35 elektrik, 31 kütle, 24 genel, 16 fiziksel büyüklükler, 12 uzunluk, 10
sıcaklık, 7 akış ve 6 basınç alanındadır; değerlendirme türüne göre 137'si bir ölçüm modeli içerir, diğerleri kalan on
türü kapsar (Tablo 2; her projenin kaynağıyla tam listesi ek malzeme S1'dedir). Bir çözümlü örnek türevler olarak
birden çok proje verebilir; 284 proje 227 çözümlü örneği veya türevi yeniden üretir.

<!-- table2 -->

<!-- table3 -->

### 4.2 Referans değerleri ve toleranslar

Referans değeri, kaynak kendi girdilerinden yuvarlaması içinde yeniden üretiliyorsa kaynakta basılı olan değerdir ve
toleransı son basamağının yarım birimidir. Basılı değer kendisi bir Monte Carlo veya MCMC kestirimiyse örnekleme hatası
toleransa eklenir. Kaynak hiçbir şey basmamışsa ya da girdilerinden çıkmayan bir değer basmışsa referans değeri,
depodaki bir betiğin NumPy [74] ve SciPy [75] ile yaptığı bağımsız bir yeniden hesaptır ve tutarsızlık bir bulgu olarak
kaydedilir; bir motorun kendi çıktısı hiçbir zaman kendi referansı olarak kullanılmaz. Her proje referans değerlerinin
nereden geldiğini belirtir. Bu ifadelerden sınıflandırıldığında (S1) 205 projenin (1217 değer) referans değerleri
yalnızca basılı değerlerdir, 21 projeninkiler (147 değer) basılı Monte Carlo veya MCMC kestirimleri içerir, 58
projeninkiler (477 değer) ise kısmen ya da tamamen bağımsız bir yeniden hesaptan gelir. Bu ifadeler değer başına değil
proje başına yazıldığından 477, bir yeniden hesaba dayanan değerlerin sayısı için bir üst sınırdır.

1841 referans değerinden 932'sinin toleransı son basamağın yarım birimi, 242'sinin en çok bir birimi kadardır; 265'i
Monte Carlo değeridir ve 402'sinin, nedeni projede belirtilen daha geniş bir toleransı vardır (örneğin iki anlamlı
basamakla basılmış bir girdiden türetilen bir değer). Tüm değerlerde |Δ|/tolerans oranının ortancası 0,22, 99.
yüzdeliği 0,98'dir. |Δ|'sı toleransının 0,9'undan büyük olan 73 değerin 51'inin toleransı yarım birimdir: kesin
değerleri basılı değerin yuvarlama sınırına yakındır (örneğin 93,155'in 93,16 olarak basılması). 73 değerin tamamı
toleranslarının dayanağıyla ek malzeme S3'te listelenmiştir.

Bir Monte Carlo sonucunun toleransı, programın sonucunun referans değerden ne kadar uzak olduğuna bağlı değildir.
Tolerans, referans değerin belgelenmiş toleransı (son basamağın yarım birimi ve referans kendisi bir Monte Carlo veya
MCMC kestirimiyse onun örnekleme hatası) ile son basamağın yarım birimi artı programın sonucunun on bir tohumdaki
(projenin kendi tohumu ve 101–110) standart sapmasının beş katından büyük olanıdır. Bu tohumlar toleransları
belirlediği için kural yirmi başka tohumla (201–210 ve 301–310) sınanmıştır: 265 değerin 5300 karşılaştırmasının tamamı tolerans içindedir ve en büyük |Δ|/tolerans 0,97'dir (S3). On altı değer referans değeri
olarak kullanılmaz: örneklem standart sapmalarının varyansı sonlu olmayan, t₃ girdili iki örneğin (JCGM 102 Örnek 9.4
ve TN 1900 E13) Monte Carlo standart sapmaları ve korelasyonları (F102); iki projede tohuma bağlı olan GUM doğrulaması
sonucu; ve örneğin basmadığı ve farklı bir büyüklük olan GUM kestirimiyle karşılaştırılmış olan GUM-5 Örnek 14'ün Monte
Carlo sonuçları. Kuralın geçmişi S3'te verilmiştir.

### 4.3 Tutarsızlıkların sınıflandırılması

Bulguların denetiminden önce dört sınıf tanımlandı: (a) basılı bir basamağı değiştiren ara yuvarlama; (b) sonuca
etkisi olmayan bir baskı veya aktarma hatası; (c) bir tutarsızlık; (n) tutarsızlık yok (yöntemsel bir not veya tam
yeniden üretim). Bulguların ilk iç incelemesinden sonra c sınıfı bulgular etkilerine göre ayrıca bölündü: c1, basılı
bir nihai sonuç — kestirim, belirsizliği veya belirtilen sonuç — ya tutarsızlık giderildiğinde, belgenin kendi ara değer
yuvarlamasıyla bile basılı basamaklarında değişir ya da belgenin kendi basılı verilerinden çıkmaz; c2, tutarsızlık
yalnızca bir ara değeri değiştirir; m, bir tutarsızlık değil, yöntem veya yorum farkı; e, yayımlanmış bir düzeltmeyle
zaten giderilmiş bir hata. Bu alt sınıflar bulgular toplandıktan sonra getirildi; beş bulgu daha sonra bağımsız
yeniden hesap ve inceleme ışığında yeniden sınıflandırıldı, bölündü veya genişletildi (Bölüm 6.1). Belgenin kendi verilerinden yeniden hesap açık bir soru bırakmıyorsa bulgu doğrulanmış
sayılır; basılı sonucuna basılı girdilerin makul bir yuvarlanışıyla ulaşılabilen bir c1 bulgusu açıktır. Başka bir
kaynakla denetlenmeyi bekleyen bulgular açıktır ve sayılmaz.

### 4.4 Bağımsız denetimler

Suncal 1.7.1'in [24] ifade edebildiği her proje iki programla da değerlendirildi (ek malzeme S4): 133 ölçüm modeli,
birden çok çıktılı 12 model, düz doğrular için Suncal'ın York uydurmasıyla, diğer fonksiyonlar için dik uzaklık
regresyonuyla karşılaştırılan iki değişkende de belirsizlikli 6 kalibrasyon ve her sonucun uygunluk olasılığı ile
küresel risklerle karşılaştırılan 18 uygunluk değerlendirmesi. Beş proje hem model hem uygunluk değerlendirmesi olarak
karşılaştırıldığından bunlar 164 projenin 169 karşılaştırmasıdır. Diğer proje türlerinin Suncal'da karşılığı yoktur.
Test belirsizlik oranından koruma bantları üzerine üç projenin referans değerleri Suncal'dan alınmamış, yayımlanmış
koruma bandı çarpanlarından (Dobbert'in yönetilen koruma bandı [53], 1,25 − 1/TUR ve √(1 − 1/TUR²)) ve JCGM 106'nın
risk integrallerinden SciPy ile hesaplanmıştır; böylece bunların Suncal ile karşılaştırılması döngüsel değildir. Bu üç
proje bir kılavuzun çözümlü örneği değil, bu çalışma için kurulmuş sınama durumlarıdır.

Ölçüm modelleri, yeterlilik testleri, kesinlik deneyleri ve yukarıdan aşağı değerlendirmeler için KAPSAM ayrıca bir
elektronik tablo doğrulama dosyası (.xlsx) yazar. Dosya projenin verilerini ve sıradan elektronik tablo formüllerini
içerir: duyarlılık katsayıları Kragten tablosu anlayışında merkezi farklar olsun diye kestirimlerde ve xᵢ ± 0,001 u(xᵢ)
noktalarında değerlendirilen bir formül olarak ölçüm modeli [8, Appendix E]; her yinelemesi bir sütunda ISO 13528'in A
Algoritması; ISO 5725-2'nin varyans analizi; Nordtest TR 537'nin bileşenleri. KAPSAM'ın kendi sonuçları yalnızca
elektronik tablonunkilerin yanında sabit sayılar olarak, fark ve toleransla birlikte yer alır. Böylece aritmetiği
elektronik tablo programı yapar ve her formül incelenebilir; formüllerin kendisi ise standartların programla aynı
okunuşunu izler (Bölüm 8). Hesap tabloları ve bir R betiği UncertCalc'ı rastgele üretilmiş veriler üzerinde doğrulamak
için aynı biçimde kullanılmıştır [32]; kütüphaneden üretilen dosyalar bu denetimi sonuçları kılavuzlarda basılı
veriler üzerinde yapar.

## 5. Doğrulama sonuçları

284 projenin 1841 referans değerinin tamamı toleransları içinde yeniden üretilir. Bunların en az 1364'ü basılı
değerdir, en çok 477'si bağımsız bir yeniden hesaba dayanır (Bölüm 4.2) ve 265 Monte Carlo değeri, programın tohumlar
arası saçılımını içeren toleranslarla karşılaştırılır. Validasyon matrisi (ek malzeme S3)
227 çözümlü örneği veya türevi yeniden üretilmiş olarak kaydeder; bunların 25'inde bulgu vardır. Tablo 4 bağımsız
uygulamalarla karşılaştırmaları özetler.

**Tablo 4.** Bağımsız uygulamalarla karşılaştırmalar.

| Karşılaştırma | Kapsam | Ölçüt | En büyük sapma | Not |
|---|---|---|---|---|
| Suncal, birinci mertebe LPU | 133 model | u'nun bağıl farkı | 6,4 × 10⁻¹² | ikinci mertebe kestirim terimi istenen 6 proje dışında kestirimler aynı (Suncal birinci mertebedir): Δy = 0,5u ve 0,1u, birinci mertebe u'nun sıfır olduğu yerde ise terimin kendisi |
| Suncal, birden çok çıktı | 12 model | u ve korelasyon katsayıları | 1,6 × 10⁻¹⁴; 3,1 × 10⁻¹⁴ | |
| Suncal, Monte Carlo | standart sapmaların 79 karşılaştırması | farkın standart hatasına oranı, z | %0,29; en büyük &#124;z&#124; = 2,38 | örnekleme hatasıyla uyumlu: 79 bağımsız karşılaştırmada en büyük &#124;z&#124;'nin 2,38'i aşma olasılığı 0,75'tir (bazıları aynı modelin türevleri olduğundan bu bir yol göstericidir) |
| Suncal, kalibrasyon | 6 kalibrasyon | parametreler, u'ları, numune içerikleri | 2 × 10⁻⁴ u(b); u(b)'de %0,75 | u(b)'deki fark uydurmadan değil kovaryansın tanımından gelir (tam Hessian'a karşı Gauss–Newton) |
| Kalibrasyon uydurması boyunca Monte Carlo | 8 proje, 10⁶ çekiliş | parametrelerin ve numune içeriklerinin s.s.'si | KAPSAM'dan %0,15; Gauss–Newton'dan %0,8 | tam Hessian değerlerine daha yakın (fark %0,12–0,15, yaklaşık üç standart hata) |
| Suncal, uygunluk | 18 değerlendirme, 32 değer | uygunluk olasılıkları, küresel riskler | 1,4 × 10⁻¹⁰ | |
| LibreOffice'te elektronik tablo dosyaları | 187 proje, 4631 değer (3649'u formüllerle hesaplanır) | tolerans içinde fark | toleransın 0,44'ü | M3003 K1'in etkin serbestlik derecesi (dördüncü kuvvette merkezi farklar) |
| Yayımlanmış uzlaşı değerleri | 9 proje | basılı değerler | tolerans içinde | Koepke ve ark., IUPAC 2024, Possolo ve Meija, NICOB kılavuzu |
| NICOB kodunun NumPy aktarımı | iki NICOB örneğinin eşdeğerlik dereceleri | D ve U₉₅ | D aynı; DerSimonian–Laird U₉₅'i %1,7 (PCB 28) ve %0,7 (⁶⁰Co), Linear Pool ve Bayes U₉₅'i %1,0 içinde | program 10⁵, aktarım 10⁶ çekiliş; programın U₉₅'i tohumlar arasında %0,8'e kadar (bağıl s.s.) değişir, yani farklar Monte Carlo saçılımı büyüklüğündedir |

Kalibrasyonlarda parametrelerin standart belirsizlikleri iki projede (EMUE E3.3 ve ISO 6143 B.3'ün kuvvet fonksiyonu)
%0,75'e kadar, diğerlerinde 1,1 × 10⁻⁴ veya daha az farklıdır. Fark en büyük ağırlıklı sapma Γ'yı izlemez (ISO 6143
B.2'de 1,63, farkı 1,1 × 10⁻⁴): Gauss–Newton yaklaşımının atladığı, artıklar ve modelin eğriliğiyle büyüyen tam
Hessian terimlerinden gelir; KAPSAM'ın çözümünde hesaplanan Gauss–Newton değerleri Suncal'ınkilerle 6 × 10⁻⁵ içinde
uyuşur. Kalibrasyon verilerinin en küçük kareler çözümü boyunca Monte Carlo ile yayılması [76] uydurmayı her çekilişte
yineler ve bu nedenle iki doğrusallaştırmadan da bağımsızdır. İki doğrusallaştırmanın ayrıştığı yerde parametrelerin
Monte Carlo standart sapmaları tam Hessian değerlerine (%0,15 içinde, standart hata %0,05–0,06) Gauss–Newton
değerlerinden (%0,8'e kadar) daha yakındır ve 13 numune içeriğinin tamamının standart belirsizlikleri KAPSAM'ınkilerle
%0,15 içinde uyuşur. Parametreleri b₀ ve b₁ arasında −0,999 997 korelasyon olan ISO 6143 B.3'ün üstel fonksiyonunda
b₀ ve b₁'in Monte Carlo standart sapmaları iki doğrusal değerden de %1,4 büyüktür; bunların birleşimine bağlı olan
numune içeriği ise %0,04 içinde uyuşur.

Elektronik tablo dosyalarında karşılaştırılan 4631 değerin 3649'u elektronik tablo formülleriyle hesaplanır; diğer 982
değer bir girdiyi veya sabit bir kapsama faktörünü yineler ve aritmetiği değil projenin dosyaya aktarılmasını denetler.
Dosyalar LibreOffice Calc'ta yeniden hesaplanır; sıradan .xlsx dosyalarıdır, ancak Excel'de yeniden hesaplanmaları
sistematik olarak denetlenmemiştir.

## 6. Yeniden hesabın ortaya koydukları

### 6.1 Genel bakış

Örneklerin yeniden hesaplanması 102 bulgu ortaya çıkardı (F1–F102; tamamı ek malzeme S2'de, c1 bulgularının
türetilişleri S8'dedir). Sınıflarına göre 27'si yalnızca ara yuvarlama, 24'ü etkisiz baskı hatası, 24'ü tutarsızlık ve
14'ü tutarsızlık yok sınıfındadır; 13'ü iki sınıfı birleştirir (6'sı a ve b, 1'i a ve c, 6'sı b ve c). Otuz bir bulgunun c
sınıfı bir bileşeni vardır; Tablo 5 bunları etkiye ve duruma göre ayırır. Sonuçlanan 25 bulgudan 11'i basılı bir nihai
sonucu değiştirir (c1), 8'i yalnızca bir ara değeri değiştirir (c2; ikisi düzeltmelerle kısmen giderilmiş), 5'i yöntem
farkıdır (m) ve 1'i bilinen bir düzeltmedir (e); 6'sı açıktır ve sayılmaz. Her biri bir kez ve en ağır sınıfıyla
sayıldığında bulgular belgelerin çoğunu ilgilendirir (Şekil 3a): GUM-5 ve Eurachem QUAM'da dokuzar, UKAS M3003'te yedi, EMUE derlemesinde
beş bulgu vardır. Üç bulgu, proje hazırlanırken yeniden hesaplanan ve kütüphanenin 36 kaynağının dışında kalan
belgelere aittir (Magnusson ve Ellison [77], Mateos ve ark. [78], Delker [23]).

**Tablo 5.** c sınıfı bileşeni olan 31 bulgu, etkiye ve duruma göre.

| Sınıf | Doğrulanmış | Bağımsız denetlenmiş | Düzeltme | Açık | Üyeler |
|---|---:|---:|---:|---:|---|
| c1 | 0 | 11 | 0 | 5 | Tablo 6; açık: F29, F85, F90, F92, F97 |
| c2 | 4 | 2 | 2 | 0 | F87 bunlar arasında |
| m | 3 | 2 | 0 | 1 | F40, F63, F69, F73, F79, F80 |
| e | 0 | 0 | 1 | 0 | F14 |

*Bağımsız denetlenmiş: doğrulanmış ve ayrıca kodu görmemiş bir kişi tarafından, bir yapay zekâ asistanıyla çalışılarak
yeniden üretilmiş (S8) ya da başka bir yazar tarafından bildirilmiş.*

c1 bulguları basılı sonucu, genişletilmiş belirsizliğinin %0,6'sı (F58, U'da bir değişiklik) ile 34 katı (F28,
kestirimde bir değişiklik) arasında değiştirir; beşi U'nun onda birinden fazla değiştirir (Şekil 3b, Tablo 6). İlk
aşamada c1 sınıfına konan on üç bulgunun tamamı kodu görmemiş bir metrolog tarafından, bir yapay zekâ asistanıyla
çalışılarak belgelerden yeniden hesaplandı (Bölüm 8). Ek malzeme S8 her biri için basılı değeri, programın değerini ve
yeniden hesaplanan değeri verir. Yeniden hesaplanan değerler, üç durum dışında basılı basamaklarda, F58'de ise
0,02 nm içinde (82,04'e karşı 82,06 nm) programınkilerle uyuşur: F40'ta yeniden hesaplayan kişi aşağıda anlatılan üç
ele alınışı verdi, F82'de ikisi belgenin farklı tablolarından başlar ve yuvarlamadan sonra uyuşur, F90'da ise ikisi de
aşağıda anlatılan aralıktadır. Ardından beş bulgu yeniden sınıflandırıldı, bölündü veya genişletildi. F40'ta (ISO 5725-3:2023, Tablo J.5, düzey 6) basılı tekrarüretilebilirlik standart sapması
negatif bir varyans kestiriminin savunulabilir üç ele alınışından biridir; bu nedenle bulgu satırın sonucunu değil iç
tutarlılığını ilgilendirir ve m sınıfına alındı. F87'de (EMUE E3.4) bağıl sapma belirsizliklerinin karesel ortalaması
basılı %3,3 değil %3,5'tir; ancak örneğin yaptığı gibi birleşik bağıl belirsizlik %5'e yuvarlanınca basılı
U = 2,8 mg/kg her iki durumda da çıkar; bulgu c2 sınıfına alındı. F90'da (EMUE E3.3) basılı, yuvarlanmış girdiler
4,14 ile 4,78 nmol/mol arasında bir validasyon belirsizliğine izin verir; bu aralık basılı 4,7'yi içerdiğinden c1
ölçütü sağlanmaz ve bulgu açıktır. F101, F40'tan ayrıldı: ISO 5725-3:2023'ün Tablo J.2'si 1994 baskısının
verilerinden 3–5. düzeylerde 15 hücrede farklıdır ve basılı Tablo J.5 1994 verilerinden çıkar, 2023 tablosundan
çıkmaz; yani basılı sonuç belgenin kendi verilerinden çıkmaz. ⟦CORTR⟧ BIPM qNMR bütçelerine ilişkin F97'ye
(Bölüm 6.3) açık bir c1 bölümü eklendi.

<!-- fig3 -->

**Tablo 6.** Basılı bir nihai sonucu değiştiren veya onu belgenin kendi verilerinden dayanaksız bırakan doğrulanmış
tutarsızlıklar (c1 sınıfı). Etki: kestirimdeki (y) veya genişletilmiş belirsizlikteki (U) değişiklik, basılı U
biriminde.

| F | Belge | Büyüklük | Basılı | Yeniden hesaplanan | Etki | Neden | Önceden biliniyor mu? |
|---|---|---|---|---|---|---|---|
| F28 | NIST TN 1900 E4 | v / (m/s) | 40,64 | 57,48 | 34 U (y) | basılı değer = verilen denklem ÷ √2 | Delker [23] bildirmiş; düzeltme yok |
| F78 | GUM-5 Örn. 1 | pH_X | 7,0109 | 7,0043 | 0,80 U (y) | örneğin kendi enterpolasyonları 7,0043 verir | düzeltme yok |
| F82 | GUM-5 Örn. 4 | u(y_CO₂) / (µmol/mol) | 5 | 4 | 0,20 U (U) | Tablo 4.1 ve 4.2'den birleşik u 3,9–4,0 µmol/mol'dür | düzeltme yok |
| F57 | UKAS M3003 K.3 | U / dB | 0,049 | 0,040 | 0,18 U (U) | uyumsuzluk terimi 0,022 dB; verilen katsayılarla E5 formülü 0,017 dB verir | düzeltme yok |
| F21 | EA-4/02 S13 | d_X / mm | 90,0003 | 90,0002 | 0,11 U (y) | bütçedeki Δl ve eş eksenlilik düzeltmesi verilerden farklı | düzeltme yok |
| F42 | ISO 6143:2025 Örn. 3 | u(x) / (µmol/mol) | 0,013280 | 0,014246 | 0,07 U (U) | standardın kendisinin (B.2.4) yanlış dediği parametre belirsizliklerinden | kısmen (B.2.4); düzeltme yok; değişiklik DIS aşamasında |
| F56 | UKAS M3003 K.5 | U / mg | 0,30 | 0,31 | 0,03 U (U) | basılı bileşenler 0,150 mg değil 0,153 mg verir | düzeltme yok |
| F77 | GUM-5 Örn. 15 | MCM s.s. / kN | 10,1 | 9,93 | 0,02 U (U) | t girdisinin ölçeği | düzeltme yok |
| F39 | ISO 21748 C.3 | U(un) / % | 12,8 | 12,6 | 0,02 U (U) | tablonun kendi bileşenleri %6,31 verir | düzeltme yok |
| F58 | UKAS M3003 K.6 | U / nm | 81,5 | 82,0 | 0,006 U (U) | bütçe sıcaklık farkına L·Δα katsayısını uygular, verilen model ise L·α verir | düzeltme yok |
| F101 | ISO 5725-3:2023 J.2, J.5 | s_r, düzey 5 / (% × 10⁻³) | 6,237 (Tablo J.5; 1994 verilerinden çıkar) | 2023 Tablo J.2'den 14,715 | — | veri tablosu J.2 1994 verilerinden 15 hücrede farklı; sonuç tablosu J.5 ondan çıkmaz | 2023 baskısında düzeltme yok ⟦CORTTR⟧ |

*Her satır kodu görmemiş bir metrolog tarafından, bir yapay zekâ asistanıyla çalışılarak belgenin güncel baskısından
yeniden hesaplanmıştır (S8, her satırın türetilişiyle). F101 dışındaki her satırda düzeltilmiş sonuç, belgenin kendi
yuvarlamasıyla bile basılı basamaklarında basılı sonuçtan farklıdır; F101'de basılı sonuç onunla birlikte basılan veri
tablosundan çıkmaz ve satır iki tablodan hangisinin yanlış olduğunu söylemez. F42'de u(x)'teki bağıl değişiklik U'dakine
eşittir. Düzeltme listeleri, düzeltmeler ve değişiklikler 11 Kasım 2026'da yayımcı kuruluşların sayfalarında
denetlenmiştir (S8).*

On bir bulgudan biri biliniyordu (F28, Delker [23] tarafından bildirilmiş), biri kısmen biliniyordur (F42: ISO 6143
programının üstel fonksiyon için yanlış parametre belirsizlikleri verdiğini kendisi belirtir, ancak bunları u(x) için
kullanır); ISO 6143'ün programının doğrusal olmayan fonksiyonlardaki başarımına ilişkin bir değişikliği DIS
aşamasındadır ve ⟦DAMDTR⟧ Diğer dokuzu için hiçbir
düzeltme listesi, düzeltme veya değişiklik yayımlanmamıştır. Dördü basılı sonucu U'nun yirmide birinden az değiştirir,
F101 ise belirtilmiş bir U'su olmayan bir kesinlik kestirimini ilgilendirir; bunlar ölçüm için değil, örneğin basılı
basamaklarına göre referans olarak kullanılması için önemlidir.

### 6.2 Ara yuvarlama yaygındır

Otuz dört bulgu (27'si yalnızca a sınıfında, 7'si başka bir sınıfla birlikte) ara yuvarlamayla ilgilidir: bir belge
bir duyarlılık katsayısını üç basamağa ya da bir bileşeni iki basamağa yuvarlar ve sonra yuvarlanmış sayıları
birleştirir; böylece basılı sonuç kesin bir yeniden hesaptan son basamağında ayrılır. Bu sonuç için zararsızdır, ancak
örneği basılı basamaklara göre referans olarak kullanılamaz kılar ve gerçek yanlışları yuvarlama farkları arasında
gizler. Çözümlü örnekler girdilerini yuvarlanmamış olarak vermeli ve yalnızca raporlanan sonucu yuvarlamalıdır.

### 6.3 Kendi bileşenlerini yeniden üretmeyen bütçe tabloları

BIPM qNMR referans veri raporları eksiksiz saflık bütçeleri basar [70] (F97). Altı raporun 11 ¹H bütçe tablosunun
7'sinde (tabloların dokuzu kütüphanededir) integral oranının basılı bileşeni, basılı duyarlılık katsayısı ile
standart belirsizliğin çarpımını %3–17 aşar; iki anlamlı basamakla basılmış iki çarpan bunun yaklaşık %10'a kadarını
açıklar, tamamını değil. Bir tabloda (ISRD-01 Tablo 6) basılı birleşik belirsizlik, 3,96 × 10⁻⁴, basılı girdilerden
çıkan 4,10 × 10⁻⁴'ün %3,5 altındadır; bunu yuvarlanmamış hesap tablosu değerlerinin açıklayıp açıklamadığı açıktır ve
bilinene kadar bulgu açık sayılır. Basılı etkin serbestlik dereceleri, bunları veren beş tablonun hepsinde integral
oranı için ν = 4 ile yeniden üretilir; oysa raporlar oranı dört ila altı tekrarın ya da on altı değerin ortalaması
olarak tanımlar. Sertifikalı değerler tartışma konusu değildir; ancak bileşenleri kendi sütunlarından yeniden
hesaplanamayan bir bütçe referans olamaz ve girdilerden yeniden hesap bunu hemen gösterir.

### 6.4 Monte Carlo ve MCMC sonuçları birer kestirimdir

Basılı Monte Carlo ve MCMC değerleri örnekleme hatası taşır ve birçok tutarsızlık bu büyüklüktedir: kesin değer
0,185 a (yıl) iken 0,19 a olan bir Bayesçi standart sapma (F95), 5 × 10⁴ çekilişten korelasyonlar (F96), 10⁶ tekrardan
0,752 ng/g iken 0,77 ng/g olan bir bootstrap standart belirsizliği (F99). Üç serbestlik dereceli t dağılımına uyan
girdilerde çıktının örneklem standart sapmasının varyansı sonlu değildir; bu yüzden JCGM 102 Örnek 9.4'ün ve TN 1900
E13'ün basılı Monte Carlo standart sapmaları ve korelasyonları hiçbir programla kararlı biçimde yeniden üretilemez
(F102); JCGM 102 Örnek 9.4'te GUM belirsizlikleri ve korelasyonları ile Monte Carlo kestirimleri referans değeri olarak
kalır. Deterministik bir değerlendirme varsa belirsizliği ortadan kaldırır; yoksa çekiliş sayısı sonuçla birlikte
basılmalıdır.

### 6.5 Yöntem farkları

Sonuçlanmış beş c sınıfı bulgu yöntem veya yorum farkıdır. GUM-5 Örnek 2'de analit ve iç standardın alanları aynı üç
enjeksiyondan gelir. Willink, aynı eşzamanlı gözlemlerden kestirilen girdileri n − 1 serbestlik dereceli tek bir
bileşende birleştirir [50, Bölüm 4.1]; bu burada ν_eff = 2,11 ve k = 4,10 verir. Örnek ise kendi (2.3) denklemini,
yani Willink'in varyansları bağımsız kestirilen korelasyonlu kestirimler için verdiği genellemeyi [50, Bölüm 4.2]
uygular ve ν_eff = 4,8 ile k = 2,6 bulur (F79; iki formül Ek A.2'de; Willink ve Hall [51, Bölüm 4] iki durumu aynı
biçimde ayırır). Basılı ortalamalar, standart sapmalar ve korelasyonla üçer çift
enjeksiyonluk 4 × 10⁵ veri kümesinin benzetiminde, ν 4,8'de ve 2,11'de sabit tutulduğunda GUM aralığı %89 ve %96
kapsar; ν bir laboratuvarın yapacağı gibi her veri kümesinden yeniden hesaplandığında her iki okuma da %93–94 kapsar
(S2). Örnek, bağımsız varyans kestirimleri için olan kuralı aynı verilerden gelen kestirimlere uygular; seçim tek bir
sonuç için önemlidir ve bulgu bir tutarsızlık değil, yöntem farkıdır (m).

t dağılımlı girdiler için iki uzlaşım — JCGM 101'in ölçeklenmiş ve kaydırılmış t'si ile TN 1900'ün ve NIST Uncertainty
Machine'in yeniden ölçeklenmiş t'si — aynı veri için de farklı Monte Carlo sonuçları verir (F31, F35). Huang [79],
standart sapması √((n − 1)/(n − 3))·s/√n s/√n'den büyük olduğu ve dörtten az gözlemde tanımlı olmadığı için ölçeklenmiş
ve kaydırılmış t'nin kullanılmaması gerektiğini savunur ve standart sapması s/(c₄√n) olan bir normal dağılım önerir;
n = 4 olan örneğinde t uzlaşımı %50,9 daha büyük bir genişletilmiş belirsizlik verir. Program JCGM 101'i izler ve
farkı raporda belirtir (Bölüm 2.3); A tipi girdiler için programların karşılaştırılmasında her birinin uzlaşımı
belirtilmelidir.

## 7. Laboratuvarlarda kullanım

Tablo 7 programın başlıca kullanımlarını proje türleri, çıktılar ve ilgili ISO/IEC 17025 maddeleriyle listeler.

**Tablo 7.** Programın laboratuvarlarda tipik kullanımları.

| Laboratuvar | Görev | Proje türü | Çıktı | Dayanak |
|---|---|---|---|---|
| Kalibrasyon | bir kalibrasyonun belirsizliği, raporlanan belirsizliğin alt sınırı olarak CMC | model, birden çok çıktı, gösterge hatası | sertifika ifadesi, bütçe, kapsama bölgesi | ISO/IEC 17025 7.6.2, 7.8.4; ILAC P14 |
| Deney | rutin bir yöntemin validasyon ve KK verilerinden belirsizliği | yukarıdan aşağı, numune alma | her düzeyde belirsizlik, numune almanın payı | ISO/IEC 17025 7.2.2, 7.6.3; ILAC G17 |
| Deney ve kalibrasyon | sonuçların geçerliliğinin izlenmesi | kontrol grafiği, yukarıdan aşağı | sınırlar, kural ihlalleri, uzun dönem değerlendirmesi | ISO/IEC 17025 7.7.1 |
| Spesifikasyonu olan her laboratuvar | karar kuralıyla uygunluk beyanı | uygunluk | uygunluk olasılığı, koruma bandı, riskler | ISO/IEC 17025 7.1.3, 7.8.6; ILAC G8 |
| Bir YT programının katılımcısı | kendi sonuçlarının değerlendirilmesi | yeterlilik | z, ζ, E_n puanları | ISO/IEC 17025 7.7.2 |
| YT sağlayıcısı | atanmış değer, puanlar, homojenlik ve kararlılık | yeterlilik | atanmış değer, puanlar, homojenlik denetimleri | ISO/IEC 17043 [46] |
| Yöntem geliştiren | bir yöntemin tekrarlanabilirliği ve tekrarüretilebilirliği | kesinlik deneyi | aykırı değer testleriyle s_r, s_R | ISO 5725-2 |
| Gaz analizi | bir kalibrasyon fonksiyonundan numune içerikleri | kalibrasyon fonksiyonu | fonksiyon, Γ, numune içerikleri | ISO 6143 |
| Referans malzeme üreticisi | homojenlik, karakterizasyon | yeterlilik (homojenlik), uzlaşı | birimler arası değişim, uzlaşı değeri | ISO 17034 [80], ISO 33405 [81] |

**Yazılım validasyonu ve kayıtlar.** ISO/IEC 17025 7.11.2 laboratuvarın yazılımını kullanmadan önce işlevsellik
açısından validasyondan geçirmesini, 7.11.6 da hesaplamaların uygun ve sistematik biçimde kontrol edilmesini ister.
`kapsam check library --lab my-projects --report validation.pdf` komutu program sürümünü, commit'i, derleyiciyi ve
platformu, her proje dosyasının ve kümenin SHA-256 özetini ve her karşılaştırmayı referans değeri ve toleransıyla
listeleyen bir yazılım validasyon raporu yazar; `--lab` ile verilen projeler ayrı bir bölümde yer alır, çünkü
yayımlanmış örneklerin yeniden üretilmesi uygulamayı doğrular ama laboratuvarın prosedürünü doğrulamaz. Ek malzeme S5
yalnızca kütüphanenin Linux'ta yazılmış raporudur; validasyon laboratuvarın kullandığı yapılandırmada, örneğin Windows
paketiyle, yinelenmelidir. Laboratuvar tipik projelerini beklenen değerleriyle — test edilebilir biçimdeki
gereksinimleri [34] — saklar ve programın her güncellemesinden sonra yeniden çalıştırır. Teknik kayıtlar (7.5) için
proje dosyası, rapor ve raporda basılı özet birlikte bir değerlendirmeyi belgeler: özet, dosyalanmış bir raporun belirli
bir proje dosyasına ait olduğunu gösterir, ancak onu kimin hazırladığını göstermez. Aynı komut, doğrulama dosyası olan
her projenin elektronik tablo dosyasını da yazar; böylece laboratuvar kendi projelerini kendi elektronik tablo
programıyla denetleyebilir.

**Öğrenme.** Belirsizlik de çözümlü örneklerle öğretilir [33]. Uygulama, belirsizlik kavramından raporlamaya uzanan
İngilizce ve Türkçe bir rehber içerir; kullanıcınınkiler dahil her projenin program tarafından proje verilerinden
yazılan sade bir açıklaması vardır; 284 kütüphane örneği üç düzeyde 121 derse ayrılmıştır (ek malzeme S6).

## 8. Tartışma

**Referans verisi olarak çözümlü örnekler.** Kılavuz belgelerin örnekleri fiilen belirsizlik yazılımlarının referans
verileridir, ancak dizilmiş metin olarak yayımlanır. Bunları makinece okunabilir biçime dönüştürüp yeniden hesaplamak,
dikkatli okumanın tek başına yakalayamadığı Bölüm 6'daki sorunları ortaya çıkardı. Yayımcı kuruluşların örneklerinin
girdilerini makinece okunabilir biçimde — Bölüm 3'teki proje biçimi bir adaydır — yayımlamalarını ve yayından önce
TraCIM'in referans veri kümeleri için yaptığı gibi [36] yeniden hesaplamalarını öneriyoruz. Bu çalışmanın
kütüphanesi bu amaçla sunulmaktadır.

**Bu doğrulamanın gösterdikleri ve göstermedikleri.** Yayımlanmış değerlerin yeniden üretilmesi belgelenmiş
yöntemlerin uygulanışını doğrular; bir laboratuvarın kendi ölçüm prosedürünü validasyondan geçirmez. Program ve bir
yeniden hesap bir standardı aynı biçimde okuduğunda yayımlanmış değerlerle uyum ortak kaynaklı bir hatayı da
gizleyebilir. Bu yüzden Tablo 8 *bağımsız* sözcüğünün iki anlamını ayırır: farklı bir uygulama ve farklı bir kişi.
Belgelerde basılı değerler, Suncal ve NIST kılavuzlarının örnekleri başkalarınca üretilmiştir. NICOB kodunun NumPy
aktarımı, yeniden hesap betikleri ve elektronik tablo formülleri farklı uygulamalardır, ancak aynı projede ve programla
aynı yapay zekâ kodlama asistanıyla yazılmıştır (Bölüm 3); bu nedenle aritmetik ve kodlama hatalarına karşı korurlar,
bir standardın yanlış okunmasına karşı korumazlar. c1 bulguları, aynı enstitüden, makalenin yazarı olmayan ve kodu
görmemiş bir metrolog tarafından yeniden hesaplandı. Bir form, her bulgu için belgeyi, örneği ve başlanacak noktayı —
her zaman örneğin verilerini, hiçbir zaman basılı bir toplamı — verdi; programın değerleri ancak form doldurulduktan
sonra açıldı. Yeniden hesabı yapan kişi belgelerden el hesabı, Python ve Monte Carlo benzetimiyle, programı yazmakta
kullanılanla aynı aileden bir yapay zekâ asistanının (Claude, Anthropic) yardımıyla çalıştı ve her ara adımı forma
yazdı; yazar, yazılı her adımı programla karşılaştırmadan önce belgeyle denetledi. Form, asistanın hangi adımları
yaptığını kaydetmez. Bu nedenle yeniden hesap programın kodundan ve yazarından bağımsızdır, ancak asistandan bağımsız
değildir: asistanın yapmaya yatkın olduğu bir belge yanlış okuması ikisinde de ortak olabilir. Buna karşılık her c1
bulgusu, belgenin kendisinde basılı sayılarla yapılan ve S8'in verdiği, okurun elle yineleyebileceği birkaç satırlık
aritmetiğe dayanır; F78'de doğru ara değeri belgenin kendisi basar, F28 ise Delker [23] tarafından bağımsız olarak
bildirilmiştir. Programın enstitü dışındaki metrologlarca incelenmesi
yapılmamıştır.

**Tablo 8.** Programın denetimleri ve her birini programdan bağımsız kılan nedir.

| Denetim | Kapsam | Üreten | Programla paylaşılan | Programdan bağımsız olduğu nokta |
|---|---|---|---|---|
| Kılavuzların çözümlü örnekleri (Bölüm 5) | 284 proje, 1843 değer | kaynak belgelerin yazarları | proje dosyalarına aktarılan girdiler; toleranslar | yöntem, uygulama ve kişi |
| Suncal 1.7.1 (Bölüm 4.4, S4) | 164 proje | Sandia National Laboratories | projelerin Suncal modellerine çevrilmesi (bu projenin bir betiği) | uygulama: sembolik LPU, kendi Monte Carlo'su, York ve ODR uydurmaları, risk integralleri |
| NIST Uncertainty Machine ve NICOB kılavuzları | kılavuzların örnekleri | NIST | girdiler | yöntem, uygulama ve kişi |
| NICOB R kodunun NumPy aktarımı | NICOB örneklerinin eşdeğerlik dereceleri | bu proje | aktarım bu projede yazıldı | programın uygulaması; algoritma NICOB'unkidir |
| Kalibrasyon uydurması boyunca Monte Carlo yayılımı (S4) | 8 proje, her biri 10⁶ çekiliş | bu proje | betik ve ISO 6143 A.3 modeli | doğrusallaştırma: uydurma her çekilişte yinelenir |
| Elektronik tablo dosyaları (Bölüm 4.4, S7) | 187 proje, 3649 hesaplanan değer | formüller: bu proje; aritmetik: LibreOffice Calc | standartların okunuşu | aritmetik; sembolik yerine sayısal türevler |
| c1 bulgularının yeniden hesabı (Bölüm 6.1, S8) | 13 bulgu | aynı enstitüden, kodu görmemiş bir metrolog, bir yapay zekâ asistanıyla | belgeler; aynı aileden bir yapay zekâ asistanı (Claude) | uygulama ve kişi, asistan değil |

**Bulguların bildirilmesi.** Bulgular göndermeden önce yayımcı kuruluşlara iletilmedi. c1 bulguları, notların hakem
denetiminden geçmiş bir kayda atıf yapabilmesi için kabulden sonra teknik notlar olarak kuruluşlara gönderilecek ve
yanıtları programın açık araştırma günlüğüne eklenecektir. Göndermeden önce c1 bulgusu olan her belgenin düzeltme
listeleri, düzeltmeleri ve değişiklikleri denetlendi (S8). ISO 6143 için programının doğrusal olmayan fonksiyonlardaki
başarımına ilişkin bir değişiklik DIS aşamasındadır; metni ulusal ayna komite aracılığıyla incelenebilir ve F42'yi ele
alıp almadığı denetlenmemiştir.

**Sınırlılıklar.** Uygulanmamış olanlar: sonlu serbestlik dereceli sonuçlar için NICOB önselleri, zaman serileri ve
sayım istatistikleri, model ortalaması, kalibrasyon eğrilerinin Monte Carlo ile değerlendirilmesi [76] ve imzalı sonuç
paketleri; PDF raporlar ve SHA-256 özetleri bütünlük denetimi sağlar, ancak imza sağlamaz. Windows paketleri henüz
kod imzalı değildir. Proje dosyaları çözümlü örneklerin sayısal verilerini kaynak atfıyla içerir; belgelerin kendisi
dağıtılmaz ve kodun lisansı atıf yapılan verilere ilişkin haklara uzanmaz. t₃ girdili modellerin Monte Carlo standart
sapmaları, örnekleme dağılımlarının varyansı sonlu olmadığından referans değeri olarak kullanılmaz (Bölüm 4.2,
F102). Programın laboratuvarlarda kullanımı henüz incelenmemiştir.

## 9. Sonuçlar

KAPSAM GUM ailesini ve başlıca sektör kılavuzlarını tek bir motorda uygular. Program, 281'i 36 kaynağın çözümlü
örneklerinden oluşturulan 284 proje üzerinde; 1841 referans değerinin her biri için belgelenmiş bir toleransla, Monte
Carlo toleranslarının belirlenmesinde kullanılmamış tohumlarla sınanmasıyla ve bağımsız bir programla, elektronik
tablo formülleriyle ve Monte Carlo yayılımıyla karşılaştırmalarla doğrulanmıştır. Çözümlü örneklerin yeniden
hesaplanması 102 bulgu ortaya çıkardı; on biri basılı bir nihai sonucu değiştirir veya onu belgenin kendi verilerinden
dayanaksız bırakır, beşi sonucu genişletilmiş belirsizliğinin onda birinden fazla değiştirir ve kodu görmemiş bir kişi,
bir yapay zekâ asistanıyla çalışarak bunları yeniden üretmiştir. Bulguların çoğu ölçümü etkilemeyen, ancak örnekleri
basılı basamaklarına göre güvenilmez referanslar yapan ara yuvarlama veya baskı hatalarıdır. Bu nedenle çözümlü örnekler
yuvarlanmamış, makinece okunabilir girdilerle yayımlanmalı ve yayından önce bağımsız yazılımla yeniden
hesaplanmalıdır; bu çalışmanın kütüphanesi bir başlangıç noktasıdır.

## Yazılım ve veri erişilebilirliği

KAPSAM, GPL-3.0 lisansıyla https://github.com/unilker/uncertainty adresinde bulunur. Bu makalede anlatılan sürüm,
Zenodo'da arşivlenen v0.1.1'dir [82] (tüm sürümler: https://doi.org/10.5281/zenodo.24118311); ek malzemenin her dosyası
üretildiği derlemeyi (0.1.1 sürümü ve commit'i) belirtir. 284 projelik kütüphane, bağımsız yeniden hesaplar, araştırma günlüğü ve tabloları ile şekilleri üreten betikler deponun
parçasıdır; `cargo test` Bölüm 4 ve 5'teki her sayıyı yeniden üretir.

## Ek malzeme

- **S1.** 284 kütüphane projesi: kaynak belge ve bölüm, kaynağın türü, referans değerlerinin dayanağı, alan,
  değerlendirme türü ve karşılaştırılan değer sayısı.
- **S2.** Sınıfları, etkileri ve durumlarıyla 102 bulgu.
- **S3.** Validasyon matrisi: her çözümlü örnek veya türev ve sonucu; toleransına yakın değerler; başka tohumlarla
  Monte Carlo sonuçları ve tolerans kuralının geçmişi.
- **S4.** Suncal 1.7.1 ile karşılaştırma (164 projenin 169 karşılaştırması) ve kalibrasyon uydurması boyunca Monte
  Carlo yayılımı.
- **S5.** Programın kütüphanenin tamamı için yazdığı yazılım validasyon raporu.
- **S6.** *Sıfırdan belirsizlik* rehberi ve her örneğin adım adım açıklaması (İngilizce; Türkçe sürümü depoda ve
  uygulamadadır).
- **S7.** Kütüphanenin 187 elektronik tablo doğrulama dosyasının LibreOffice Calc'ta yeniden hesaplanması ve örnek
  olarak QUAM Örnek A1'in dosyası.
- **S8.** c1 bulgularının yeniden hesap kaydı: her birinin basılı, program ve yeniden hesaplanan değeri, sonuçlanmış her
  c1 bulgusunun belgenin sayılarından türetilişi ve düzeltme listelerinin ve değişikliklerin denetimi.

## Ek A. Kılavuzlarda verilmeyen denklemler

**A.1 İkinci mertebe terimler.** Girdilerin kestirimlerinde gradyan c, Hessian H = (∂²f/∂xᵢ∂xⱼ) ve kovaryans matrisi Σ
ile GUM'un 2026 değişikliğinin [6] kestirimi y = f(x) + ½ tr(HΣ)'dır. Ortak normal girdiler için varyans
u²(y) = cᵀΣc + ½ tr(HΣHΣ) + Σᵢⱼₖₗ cₗ f₍ᵢⱼₖ₎ Σₗᵢ Σⱼₖ'dir; f₍ᵢⱼₖ₎ üçüncü türevlerdir (Mana ve Pennecchi [41]);
bağımsız girdiler için bu, GUM 5.1.2'nin notundaki ifadedir. Yüksek mertebe kısma sonsuz serbestlik derecesi verilir.

**A.2 Topluluklu etkin serbestlik derecesi.** Bir toplulukta olmayan girdilerin uᵢ(y) = cᵢu(xᵢ) bileşenleri
Welch–Satterthwaite formülüne tek tek girer. ν_E serbestlik dereceli aynı verilerden kestirilen bir E topluluğunun
girdileri tek bir bileşen olarak girer, u_E²(y) = Σ_{i,j∈E} cᵢcⱼu(xᵢ, xⱼ):
ν_eff = u⁴(y) / (Σ_{i∉E} uᵢ⁴(y)/νᵢ + u_E⁴(y)/ν_E); topluluk bir kez sayılarak en çok Σνᵢ (Willink [50,
Bölüm 4.1], GUM Tree Calculator [25]). GUM-5 Örnek 2'de analit ve iç standardın alanları ν_E = 2 olan bir topluluk
oluşturur; bileşeni u_E² = 3,97 × 10⁻⁶ ng², toplam u² = 4,08 × 10⁻⁶ ng²'dir; diğer bileşenlerin serbestlik dereceleri
büyük olduğundan ν_eff ≈ (4,08 × 10⁻⁶)²/((3,97 × 10⁻⁶)²/2) = 2,11'dir. Örnekte kullanılan GUM-5 (2.3) denklemi
ν_eff = u⁴(y) / Σᵢ Σⱼ r²(xᵢ, xⱼ) cᵢ²cⱼ² u²(xᵢ)u²(xⱼ)/νᵢ'dir; Willink'in varyansları bağımsız kestirilen korelasyonlu
kestirimler için genellemesidir [50, Bölüm 4.2]. Eşit paylı, cᵢu(xᵢ) = a, r = 0,5 ve νᵢ = ν olan iki bileşen için
u⁴(y) = 9a⁴ ve payda 2,5a⁴/ν'dir; böylece (2.3) denklemi 3,6ν verir; aynı ν + 1 gözlemden gelen kestirimler için
topluluk kuralı ise ν verir.

**A.3 NIST Consensus Builder'ın bootstrap'i.** wᵢ = 1/uᵢ² ağırlıklı, W = diag(wᵢ) ve Sₘ = Σwᵢᵐ olan K sonuç için
Cochran istatistiği A = W − wwᵀ/S₁ ile Q = xᵀAx'tir ve laboratuvarlar arası varyansın moment kestirimi
τ²_M = (Q − (K − 1))/(S₁ − S₂/S₁)'dir; Q < K − 1 iken negatiftir; DerSimonian–Laird kestirimi max(0, τ²_M)'dir.
Rastgele etkiler modelinde, Σ = W⁻¹ + τ²I ile Q'nun kesin ortalaması E Q = (K − 1) + (S₁ − S₂/S₁)τ², varyansı
Var Q = 2 tr[(AΣ)²] = 2(K − 1) + 4(S₁ − S₂/S₁)τ² + 2(S₂ − 2S₃/S₁ + S₂²/S₁²)τ⁴'tür; bunlar normal değişkenlerdeki
karesel bir biçimin ortalama ve varyansından doğrudan çıkar (A ortak ortalamayı yok eder, A1 = 0). Bu iki momentli gama
dağılımı Biggerstaff ve Tweedie'nin [59] yaklaşımıdır. NICOB'un kaynak kodu momentleri kesilmemiş τ²_M'de değerlendirir; böylece ortalama
gözlenen Q'ya eşit olur; kılavuzu bunu belirtmez. Homojen bir kümede (Q < K − 1) bu, dört sonuçlu bir örnekte
τ̂²_DL = 0'da değerlendirmeye göre u'yu %13 düşürür. Program kodu izler; bir birim testi programın varyansını doğrudan
hesaplanan 2 tr[(AΣ)²] ile karşılaştırır.

**A.4 Bayesçi ızgaralar.** Bilinen uᵢ, μ üzerinde düz önsel ve τ üzerinde s ölçekli (NICOB'daki gibi belirlenen önsel
ortancası) yarı Cauchy önselle μ analitik olarak integre edilir ve τ'nun sonsalı önsel olasılıkta düzgün bir ızgarada,
τ = s·tan(πθ/2), θ ∈ (0, 1), değerlendirilir; bu ızgarada yarı Cauchy önsel sabittir. Izgara tüm τ ∈ [0, ∞) aralığını
kapsar; μ üzerinde düz önselle olabilirlik τ^−(K−1) gibi düşer, bu nedenle K ≥ 2 için son hücre ihmal edilebilir bir
kütle taşır. Izgaranın iki katına çıkarılması (10⁵'ten 2 × 10⁵ noktaya; Laplace etkileri için (θ, μ) üzerinde 300²'den
600² noktaya) ortalamaları, standart sapmaları ve yüzdelikleri sonsal standart sapmanın 10⁻³'ünden (Gauss) ve
10⁻²'sinden (Laplace) az değiştirir.

## CRediT yazar katkı beyanı

İlker Ün: Kavramsallaştırma, Yöntem, Yazılım, Validasyon, Araştırma, Yazım – ilk taslak, Yazım – gözden geçirme ve
düzenleme.

## Çıkar çatışması beyanı

Yazar, bu makalede bildirilen çalışmayı etkilediği düşünülebilecek bilinen hiçbir mali çıkar çatışması veya kişisel
ilişki bulunmadığını beyan eder.

## Finansman

Bu araştırma kamu, ticari veya kâr amacı gütmeyen kesimlerdeki finansman kuruluşlarından özel bir destek almamıştır.

## Makalenin hazırlanmasında üretken yapay zekâ ve yapay zekâ destekli teknolojilerin kullanımına ilişkin beyan

Bu çalışmanın hazırlanması sırasında yazar, makale metnini taslak olarak yazmak ve düzenlemek, İngilizce ile Türkçe
arasında çevirmek, tablolarını ve şekillerini üreten betikleri yazmak ve gönderim öncesinde makaleyi eleştirel olarak
incelemek için Claude'u (Anthropic) kullanmıştır;
yazılımın geliştirilmesindeki kullanımı Bölüm 3'te anlatılmıştır. c1 bulgularını yeniden hesaplayan meslektaş da bir
yapay zekâ asistanı (Claude, Anthropic; Bölüm 8 ve S8) kullanmıştır. Hiçbir görsel üretken bir yapay zekâ aracıyla
üretilmemiştir: Şekil 1 ve 3 deponun bir betiğiyle programın çıktısından çizilmiştir, Şekil 2 uygulamanın bir ekran
görüntüsüdür. Bu aracı kullandıktan sonra yazar içeriği gerektiği gibi gözden geçirmiş ve düzenlemiştir; yayının
içeriğinin tüm sorumluluğunu üstlenir.

## Kaynaklar

[1] ISO/IEC 17025:2017, General requirements for the competence of testing and calibration laboratories. ISO, Geneva.

[2] JCGM 100:2008, Evaluation of measurement data — Guide to the expression of uncertainty in measurement. BIPM.

[3] JCGM 101:2008, Evaluation of measurement data — Supplement 1 to the GUM — Propagation of distributions using a Monte Carlo method. BIPM.

[4] JCGM 102:2011, Evaluation of measurement data — Supplement 2 to the GUM — Extension to any number of output quantities. BIPM.

[5] JCGM 106:2012, Evaluation of measurement data — The role of measurement uncertainty in conformity assessment. BIPM.

[6] JCGM 100:2008/Amd.1:2026, Evaluation of measurement data — Guide to the expression of uncertainty in measurement. Amendment 1: Nonlinearity in measurement models. BIPM, 2026 (also ISO/IEC Guide 98-3:2008/Amd 1:2026). https://doi.org/10.59161/PPDI3267

[7] JCGM GUM-5:2026, Guide to the expression of uncertainty in measurement — Part 5: Examples. BIPM, 2026 (also ISO/IEC Guide 98-5:2026). https://doi.org/10.59161/YNLY8209

[8] S. L. R. Ellison, A. Williams (eds.), Eurachem/CITAC Guide CG 4, Quantifying Uncertainty in Analytical Measurement, 3rd ed., 2012 (QUAM:2012.P1).

[9] R. Bettencourt da Silva (ed.), Eurachem/CITAC Guide: Evaluation of measurement uncertainty from in-house precision and recovery data (MUPR), 1st ed., Eurachem, 2026. ISBN 978-972-9348-30-3.

[10] Nordtest TR 537, Handbook for calculation of measurement uncertainty in environmental laboratories, ed. 4, 2017.

[11] B. Magnusson, H. Hovind, M. Krysell, U. Lund, I. Mäkinen, E. Sahlin, Handbook of Internal Quality Control, Nordtest Report TR 569, ed. 6, 2026.

[12] EA-4/02 M:2022, Evaluation of the Uncertainty of Measurement in Calibration. European co-operation for Accreditation.

[13] UKAS M3003, The Expression of Uncertainty and Confidence in Measurement, ed. 6, 2024.

[14] EURAMET Calibration Guide No. 18, Guidelines on the Calibration of Non-Automatic Weighing Instruments, v4.0, 2015.

[15] EURAMET Calibration Guide No. 19, Guidelines on the Determination of Uncertainty in Gravimetric Volume Calibration, v4.1, 2025.

[16] ILAC G17:01/2021, ILAC Guidelines for Measurement Uncertainty in Testing.

[17] ISO 13528:2022, Statistical methods for use in proficiency testing by interlaboratory comparison.

[18] ISO 5725-2:2019, Accuracy (trueness and precision) of measurement methods and results — Part 2.

[19] ISO 5725-3:2023, Accuracy (trueness and precision) of measurement methods and results — Part 3.

[20] ISO 6143:2025, Gas analysis — Comparison methods for determining and checking the composition of calibration gas mixtures.

[21] T. Lafarge, A. Possolo, The NIST Uncertainty Machine, NCSLI Measure 10(3) (2015) 20–27. https://doi.org/10.1080/19315775.2015.11721732

[22] T. Lafarge, A. Possolo, NIST Uncertainty Machine — User's Manual, version 1.4, NIST, 2020.

[23] C. J. Delker, A comprehensive open-source R software for statistical metrology calculations: from uncertainty evaluation to risk analysis, NCSLI Measure 13(3) (2021) 45–60. https://doi.org/10.51843/measure.13.3.7

[24] C. J. Delker, Suncal: Sandia PSL Uncertainty Calculator, version 1.7.1 [software], Sandia National Laboratories, July 11, 2026. https://github.com/sandialabs/suncal (accessed 10 November 2026)

[25] B. D. Hall, The GUM Tree Calculator: a Python package for measurement modelling and data processing with automatic evaluation of uncertainty, Metrology 2 (2022) 128–149. https://doi.org/10.3390/metrology2010009

[26] M. Zeier, J. Hoffmann, M. Wollensack, Metas.UncLib — a measurement uncertainty calculator for advanced problems, Metrologia 49 (2012) 809–815. https://doi.org/10.1088/0026-1394/49/6/809

[27] Metrodata GmbH, GUM Workbench Professional, version 2.4 [software], Metrodata GmbH, Braunschweig. https://www.metrodata.de (accessed 11 November 2026)

[28] A. Koepke, T. Lafarge, A. Possolo, B. Toman, NIST Consensus Builder — User's Manual, NIST, 2020. https://consensus.nist.gov (accessed 10 November 2026)

[29] Finnish Environment Institute (Syke), MUkit — measurement uncertainty kit, version 3.0.12 [software], 2025. https://www.syke.fi/envical (accessed 11 November 2026)

[30] ISO 11352:2012, Water quality — Estimation of measurement uncertainty based on validation and quality control data.

[31] R. J. N. Bettencourt da Silva, User-friendly spreadsheet for implementing international guidance on the evaluation of measurement uncertainty, Talanta 314 (2027) 130826. https://doi.org/10.1016/j.talanta.2026.130826

[32] B. Yalçınkaya, UncertCalc: a web-based tool for measurement uncertainty calculation, J. Chem. Metrol. 19(2) (2025) 128–135. https://doi.org/10.25135/jcm.127.2511.3738

[33] F. R. Pennecchi, P. M. Harris, Mathmet Measurement Uncertainty Training activity — overview of courses, software, and classroom examples, Acta IMEKO 12(2) (2023) 1–6. https://doi.org/10.21014/actaimeko.v12i2.1310

[34] N. Greif, H. Schrepf, D. Richter, Software validation in metrology: a case study for a GUM-supporting software, Measurement 39 (2006) 849–855. https://doi.org/10.1016/j.measurement.2006.04.005

[35] N. Greif, H. Schrepf, GUM conformity of software products — a discussion from a software tester's perspective, Acta IMEKO 2(2) (2013) 34–40. https://doi.org/10.21014/acta_imeko.v2i2.69

[36] K. Wendt, M. Franke, F. Härtig, Validation of CMM evaluation software using TraCIM, in: Advanced Mathematical and Computational Tools in Metrology and Testing X, World Scientific, Singapore, 2015, pp. 392–399. https://doi.org/10.1142/9789814678629_0047

[37] Eurachem, Errata for QUAM:2012.P1 (QUAM_ERRATA_v2), 2019. https://www.eurachem.org

[38] JCGM 200:2012, International vocabulary of metrology — Basic and general concepts and associated terms (VIM), 3rd ed., BIPM, 2012. https://doi.org/10.59161/JCGM200-2012

[39] M. Désenfant, M. Priel, Road map for measurement uncertainty evaluation, Measurement 39 (2006) 841–848. https://doi.org/10.1016/j.measurement.2006.04.008

[40] M. Désenfant, M. Priel, Reference and additional methods for measurement uncertainty evaluation, Measurement 95 (2017) 339–344. https://doi.org/10.1016/j.measurement.2016.10.022

[41] G. Mana, F. Pennecchi, Uncertainty propagation in non-linear measurement equations, Metrologia 44 (2007) 246–251. https://doi.org/10.1088/0026-1394/44/3/012

[42] Eurachem/CITAC Guide, Use of uncertainty information in compliance assessment, 2nd ed., 2021.

[43] ILAC G8:09/2019, Guidelines on Decision Rules and Statements of Conformity.

[44] ISO 21748:2017, Guidance for the use of repeatability, reproducibility and trueness estimates in measurement uncertainty evaluation.

[45] A. Possolo, C. Bodnar, Approximate Bayesian evaluations of measurement uncertainty, Metrologia 55 (2018) 147–157. https://doi.org/10.1088/1681-7575/aaa5be

[46] ISO/IEC 17043:2023, Conformity assessment — General requirements for the competence of proficiency testing providers.

[47] M. H. Ramsey, S. L. R. Ellison, P. Rostron (eds.), Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide: Measurement uncertainty arising from sampling, 2nd ed., 2019.

[48] M. Thompson, S. L. R. Ellison, Dark uncertainty, Accred. Qual. Assur. 16 (2011) 483–487. https://doi.org/10.1007/s00769-011-0803-0

[49] A. Koepke, T. Lafarge, A. Possolo, B. Toman, Consensus building for interlaboratory studies, key comparisons, and meta-analysis, Metrologia 54 (2017) S34–S62. https://doi.org/10.1088/1681-7575/aa6c0e

[50] R. Willink, A generalization of the Welch–Satterthwaite formula for use with correlated uncertainty components, Metrologia 44 (2007) 340–349. https://doi.org/10.1088/0026-1394/44/5/010

[51] R. Willink, B. D. Hall, An extension to GUM methodology: degrees-of-freedom calculations for correlated multidimensional estimates, arXiv:1311.0343 [physics.data-an], 2013. https://doi.org/10.48550/arXiv.1311.0343

[52] ILAC P14:09/2020, ILAC Policy for Measurement Uncertainty in Calibration.

[53] M. Dobbert, A guard-band strategy for managing false-accept risk, NCSLI Measure 3(4) (2008) 44–48. https://doi.org/10.1080/19315775.2008.11721446

[54] L. R. Pendrill, Optimised measurement uncertainty and decision-making when sampling by variables or by attribute, Measurement 39 (2006) 829–840. https://doi.org/10.1016/j.measurement.2006.04.014

[55] C. F. M. Carobbi, A modified ISO 13528 robust analysis (Algorithm A) that takes measurement uncertainty into account, Measurement 110 (2017) 296–306. https://doi.org/10.1016/j.measurement.2017.07.006

[56] K. Shirono, M. Shiro, H. Tanaka, K. Ehara, Proficiency tests with uncertainty information: detection of an unknown random effect, Measurement 83 (2016) 144–152. https://doi.org/10.1016/j.measurement.2016.01.002

[57] R. DerSimonian, N. Laird, Meta-analysis in clinical trials, Control. Clin. Trials 7 (1986) 177–188. https://doi.org/10.1016/0197-2456(86)90046-2

[58] G. Knapp, J. Hartung, Improved tests for a random effects meta-regression with a single covariate, Stat. Med. 22 (2003) 2693–2710. https://doi.org/10.1002/sim.1482

[59] B. J. Biggerstaff, R. L. Tweedie, Incorporating variability in estimates of heterogeneity in the random effects model in meta-analysis, Stat. Med. 16 (1997) 753–768. https://doi.org/10.1002/(SICI)1097-0258(19970415)16:7<753::AID-SIM494>3.0.CO;2-G

[60] C. Röver, G. Knapp, T. Friede, Hartung–Knapp–Sidik–Jonkman approach and its modification for random-effects meta-analysis with few studies, BMC Med. Res. Methodol. 15 (2015) 99. https://doi.org/10.1186/s12874-015-0091-1

[61] S. Westwood et al., Methods for the SI-traceable value assignment of the purity of organic compounds (IUPAC Technical Report), Pure Appl. Chem. 95 (2023) 1–77. https://doi.org/10.1515/pac-2020-0804

[62] B. Toman, M. A. Nelson, K. A. Lippa, Chemical purity using quantitative ¹H-nuclear magnetic resonance: a hierarchical Bayesian approach for traceable calibrations, Metrologia 53 (2016) 1193–1203. https://doi.org/10.1088/0026-1394/53/5/1193

[63] S. Cowen, S. L. R. Ellison, Reporting measurement uncertainty and coverage intervals near natural limits, Analyst 131 (2006) 710–717. https://doi.org/10.1039/b518084h

[64] B. A. Wichmann, I. D. Hill, Generating good pseudo-random numbers, Comput. Stat. Data Anal. 51 (2006) 1614–1622. https://doi.org/10.1016/j.csda.2006.05.019

[65] G. Marsaglia, W. W. Tsang, A simple method for generating gamma variables, ACM Trans. Math. Softw. 26 (2000) 363–372. https://doi.org/10.1145/358407.358414

[66] The Document Foundation, LibreOffice Calc, version 24.2.7.2 [software], 2024. https://www.libreoffice.org (accessed 10 November 2026)

[67] A. Possolo, Simple Guide for Evaluating and Expressing the Uncertainty of NIST Measurement Results, NIST Technical Note 1900, 2015. https://doi.org/10.6028/NIST.TN.1900

[68] A. M. H. van der Veen, M. G. Cox (eds.), Good practice in evaluating measurement uncertainty — Compendium of examples, EMPIR 17NRM05 EMUE, 2021.

[69] A. Possolo et al., A brief guide to measurement uncertainty (IUPAC Technical Report), Pure Appl. Chem. 96 (2024) 113–134. https://doi.org/10.1515/pac-2022-1203

[70] BIPM, Internal Standard Reference Data for qNMR: Maleic Acid (ISRD-01, Rapport BIPM-2018/01); Dimethyl Sulfone (ISRD-02, Rapport BIPM-2018/04); Potassium Hydrogen Phthalate (ISRD-03, Rapport BIPM-2018/05); 1,4-Bis(trimethylsilyl)benzene-d4 (ISRD-06, Rapport BIPM-2019/03); Sodium 4,4-dimethyl-4-silapentane-1-sulfonate-d6 (ISRD-07, Rapport BIPM-2019/04); Benzoic Acid (ISRD-08, Rapport BIPM-2023/02). BIPM, Sèvres.

[71] C. Palma, V. Morgado, R. J. N. Bettencourt da Silva, Top-down evaluation of matrix effects uncertainty, Talanta 192 (2019) 278–287, https://doi.org/10.1016/j.talanta.2018.09.039; erratum Talanta 208 (2020) 120476, https://doi.org/10.1016/j.talanta.2019.120476

[72] C. Merkatas, B. Toman, A. Possolo, S. Schlamminger, Shades of dark uncertainty and consensus value for the Newtonian constant of gravitation, Metrologia 56 (2019) 054001. https://doi.org/10.1088/1681-7575/ab3365

[73] A. Possolo, J. Meija, Measurement Uncertainty: A Reintroduction, 2nd ed., SIM, Montevideo, 2022. https://doi.org/10.4224/1tqz-b038

[74] C. R. Harris et al., Array programming with NumPy, Nature 585 (2020) 357–362. https://doi.org/10.1038/s41586-020-2649-2

[75] P. Virtanen et al., SciPy 1.0: fundamental algorithms for scientific computing in Python, Nat. Methods 17 (2020) 261–272. https://doi.org/10.1038/s41592-019-0686-2

[76] I. Lira, Monte Carlo evaluation of the uncertainty associated with the construction and use of a fitted curve, Measurement 44 (2011) 2156–2164. https://doi.org/10.1016/j.measurement.2011.07.021

[77] B. Magnusson, S. L. R. Ellison, Treatment of uncorrected measurement bias in uncertainty estimation for chemical measurements, Anal. Bioanal. Chem. 390 (2008) 201–213. https://doi.org/10.1007/s00216-007-1693-1

[78] R. Mateos et al., Impact of recovery correction or subjecting calibrators to sample preparation on measurement uncertainty: PAH determinations in waters, Talanta 207 (2020) 120274. https://doi.org/10.1016/j.talanta.2019.120274

[79] H. Huang, Why the scaled and shifted t-distribution should not be used in the Monte Carlo method for estimating measurement uncertainty?, Measurement 136 (2019) 282–288. https://doi.org/10.1016/j.measurement.2018.12.089

[80] ISO 17034:2016, General requirements for the competence of reference material producers.

[81] ISO 33405:2024, Reference materials — Approaches for characterization and assessment of homogeneity and stability.

[82] İ. Ün, KAPSAM: open-source software for the evaluation of measurement uncertainty (GUM, Monte Carlo and top-down), v0.1.1 [software], Zenodo, 2026. ⟦DOI⟧
