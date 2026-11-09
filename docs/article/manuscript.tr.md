# Ölçüm belirsizliği kılavuzlarının çözümlü örneklerini yeniden hesaplamak: açık, doğrulanmış bir program ve 286 örneğin ortaya koydukları

*İlk taslağın Türkçe çevirisi, 2026-11-09. Asıl metin İngilizce olan `docs/article/manuscript.md` dosyasıdır; bu
çeviri gözden geçirme içindir ve her değişiklikte İngilizce metinle birlikte güncellenir. Sayılar
`docs/article/numbers.md` (`tools/research/article_stats.py` ile üretilir), `docs/research/validation-matrix.md`
ve araştırma günlüğünden (kararlar D…, bulgular F…) gelir; [doğrula] işaretli kaynakların künyeleri yayınla
karşılaştırılacaktır. Şekil 1–3 ve Tablo 2–3, `tools/research/article_figures.py` ile programın çıktısından ve
`numbers.md` dosyasından üretilir.*

İlker Ün — TÜBİTAK Ulusal Metroloji Enstitüsü (UME), Gebze, Kocaeli, Türkiye

## Özet

ISO/IEC 17025'e göre akredite laboratuvarlar ölçüm belirsizliğini hesap tablolarıyla ya da GUM ailesinin yalnızca
bir bölümünü kapsayan programlarla değerlendirir; yöntemlerini kılavuz belgelerin çözümlü örnekleri üzerinde öğrenir
ve doğrular. Bu çalışmada açık kaynak kodlu bir program sunuyoruz: komut satırı aracı ile masaüstü ve mobil
uygulamasıyla birlikte bir Rust motoru; raporları İngilizce ve Türkçe. Program, GUM'un 2026 değişikliği dahil
belirsizliğin yayılımı yasasını, JCGM 101 ve 102'nin Monte Carlo yöntemlerini, uygunluk değerlendirmesini (JCGM
106), geçerli kılma ve kalite kontrol verilerinden yukarıdan aşağı değerlendirmeyi, yeterlilik testi istatistiklerini
(ISO 13528), iki değişkende de belirsizlik bulunan kalibrasyonu (ISO 6143), laboratuvarlar arası kesinliği (ISO
5725-2), kontrol grafiklerini (Nordtest TR 569), numune almadan kaynaklanan belirsizliği, tartı aletlerini (EURAMET
cg-18), eşdeğerlik dereceleriyle uzlaşı değerlerini ve fiziksel bir sınıra yakın sonuçları kapsar. Program, 38
kılavuz belge ve makalenin çözümlü örneklerinden oluşturulan 286 proje üzerinde doğrulanmıştır; test takımı her
değişiklikte 1865 referans değerinin tamamını girdilerinden yeniden hesaplar ve bağımsız bir programla (Sandia'nın
Suncal programı) 107 proje üzerinde yapılan karşılaştırmada birinci mertebe standart belirsizlikler 6 × 10⁻¹²
düzeyinde uyuşur. Örneklerin yeniden hesaplanması 100 bulgu ortaya çıkardı: belgelerin çoğunda ara yuvarlamalar ve
baskı hataları; ayrıca basılı bir nihai sonucu değiştiren 13 doğrulanmış tutarsızlık. Bu tutarsızlıklar yöntem
farklarından ve yayımlanmış düzeltmelerle giderilmiş hatalardan ayrı sayılmıştır. Literatürün MCMC ile elde ettiği
Bayesçi rastgele etkiler uzlaşı değerleri deterministik bir ızgara üzerinde yeniden üretilmiştir; yöntem seçimi bir
anahtar karşılaştırmanın eşdeğerlik derecelerini, referans değerini değiştirdiği ölçüde değiştirir. Çözümlü
örneklerin makinece okunabilir girdilerle yayımlanması ve bağımsız yeniden hesapla denetlenmesi gerektiğini
savunuyor, örnek kütüphanesini bu amaçla kullanıma sunuyoruz.

**Anahtar sözcükler:** ölçüm belirsizliği; GUM; Monte Carlo yöntemi; yazılım validasyonu; çözümlü örnekler; uzlaşı
değeri; eşdeğerlik dereceleri; ISO/IEC 17025

## 1. Giriş

ISO/IEC 17025, deney ve kalibrasyon laboratuvarlarının sonuçlarının belirsizliğini değerlendirmesini (7.6) ve bunun
için kullandıkları yazılımı doğrulamasını (7.11.2) şart koşar [1]. Değerlendirmenin kendisi *Ölçümde belirsizliğin
ifade edilmesi kılavuzu* (GUM) ve eklerine dayanır: belirsizliğin yayılımı yasası (LPU) [2], tek çıktı (JCGM 101)
[3] ve birden çok çıktı (JCGM 102) [4] için Monte Carlo yöntemi, belirsizliğin uygunluk değerlendirmesinde
kullanımı (JCGM 106) [5] ve 2026'dan beri doğrusal olmayan modeller üzerine bir GUM değişikliği [6] ile çözümlü
örneklerden oluşan yeni bir bölüm (GUM-5) [7]. Sektör kılavuzları bu çerçeveyi uygulamaya aktarır: analitik kimya
için Eurachem/CITAC kılavuzları [8, 9], çevre laboratuvarları için Nordtest el kitapları [10, 11], kalibrasyon için
EA-4/02 [12], UKAS M3003 [13] ve EURAMET kalibrasyon kılavuzları [14], yeterlilik testi [15], kesinlik deneyleri
[16, 17] ve gaz analizi [18] için ISO standartları.

Bu belgelerin her biri örnekle öğretir. Yeni bir yöntem için belirsizlik bütçesi kuran bir laboratuvar, sayılarını
bir çözümlü örnekle karşılaştırır; yazılımını doğrulayan bir laboratuvar, sonuçları basılı çözümlü örnekleri
yeniden çalıştırır. Her iki kullanım da basılı sonuçların basılı girdilerden çıktığını varsayar. Bizim
deneyimimizde ve başka programların geliştiricilerinin deneyiminde (Bölüm 5.4), bu çoğu zaman tam olarak doğru
değildir; zaman zaman da hiç doğru değildir.

Yazılım ortamı kılavuzların yapısını yansıtır. NIST Uncertainty Machine [19] açık modelleri GUM ve Monte Carlo ile
değerlendirir; Sandia'nın Suncal programı [20] kalibrasyon için sembolik LPU, eğri uydurma ve karar riski ekler; GUM
Tree Calculator [21] ve METAS UncLib [22] "belirsiz sayıları" programlar boyunca yayar; NIST Consensus Builder
(NICOB) [23] laboratuvarlar arası sonuçları birleştirir; MUPR hesap tablosu [24] kimya için yukarıdan aşağı bir
değerlendirme uygular. Avrupa Metroloji Ağı Mathmet'in 35 program üzerinde yaptığı bir taramada, programların
yalnızca %54'ünün bir tür validasyon kanıtı sunduğu ve JCGM 102 desteğinin en büyük eksik olduğu bulunmuştur [25].
Bu araçların hiçbiri aşağıdan yukarı ve yukarıdan aşağı değerlendirmeyi, yeterlilik testini, uygunluk
değerlendirmesini ve uzlaşı değerlerini birlikte kapsamaz; hiçbiri uyguladığı kılavuzun çözümlü örneklerini yeniden
hesaplayan bir validasyon yayımlamaz.

Bu makalenin üç katkısı vardır. (i) GUM ailesini ve başlıca sektör kılavuzlarını kapsayan, her algoritmanın
dayandığı maddeyi belirten ve iki dilde rapor üreten açık bir program tanımlıyoruz (Bölüm 2). (ii) Programı, 38
belgenin çözümlü örneklerinden oluşturulmuş, kendini denetleyen 286 projelik bir kütüphane üzerinde doğruluyoruz;
1865 referans değerinin her biri için belgelenmiş bir tolerans, bağımsız bir karşılaştırma ve önceden tanımlanmış
bir tutarsızlık sınıflandırması kullanıyoruz (Bölüm 3 ve 4). (iii) Yeniden hesabın örneklerin kendisi hakkında
ortaya koyduklarını raporluyor; uzlaşı değerleri ve fiziksel bir sınıra yakın sonuçlar için kılavuzların açık
bıraktığı seçimlerin raporlanan sonucu değiştirdiğini gösteriyoruz (Bölüm 5 ve 6).

## 2. Program

### 2.1 Mimari

Motor, bir Rust kütüphanesi (`core`) ile proje dosyalarını okuyan, değerlendirmeleri çalıştıran, sonuçları
referans değerlerle karşılaştıran ve rapor yazan bir proje katmanından (`project`) oluşur. Motora bir komut satırı
aracı (`cli`) ve bir C arayüzü (`ffi`) üzerinden erişilir; uygulama (Flutter) aynı motorla Windows, Linux, macOS,
Android ve iOS üzerinde çalışır. Bir proje; iki dilli üst verileri (başlık, açıklama, yorum, notlar, kaynaklar), on
bir değerlendirme türünden birinin girdi verilerini (Tablo 1) ve isteğe bağlı olarak toleranslarıyla beklenen
değerleri içeren bir JSON dosyasıdır. Raporlar İngilizce veya Türkçe olarak Markdown, SVG grafikli bağımsız HTML,
PDF (A4, yazı tipleri gömülü, aynı proje için bayt bayt aynı) ve tüm değerleri tam hassasiyetle içeren JSON
biçiminde yazılır. Her rapor yazılım sürümünü, derlendiği git commit'ini ve değerlendirilen projenin SHA-256
özetini belirtir; böylece bir teknik kayıt, sonucu girdilerine ve doğrulanmış bir derlemeye bağlar (ISO/IEC 17025
7.5, 7.11).

<!-- fig1 -->

**Tablo 1.** Değerlendirme türleri ve uyguladıkları yöntemler (modül belgeleri ve
`docs/research/iso17025-mapping.md` ile karşılaştırılmıştır).

| Proje türü | Yöntemler | Başlıca kaynaklar |
|---|---|---|
| model | korelasyonlu ve topluluklu birinci ve ikinci mertebe LPU; Welch–Satterthwaite; Kragten hesap tablosu; Monte Carlo (uyarlamalı, en kısa ve simetrik aralıklar, kestirim çevresinde yarı genişlik); GUM sonucunun Monte Carlo ile geçerli kılınması; sınırlı çıktılar | [2, 3, 6, 26] |
| multivariate | birden çok çıktı için LPU ve Monte Carlo, açık ve kapalı modeller, kapsama bölgeleri | [4] |
| conformity | özgül ve genel riskler, koruma bantları (JCGM 106; test belirsizlik oranı), lognormal süreçler, çok bileşenli ürünler | [5, 27] |
| topdown | geçerli kılma ve kalite kontrol verilerinden kesinlik ve geri kazanım (Nordtest, MUPR, ISO 21748) | [9, 10, 28] |
| proficiency | dayanıklı istatistikler, puanlar, homojenlik ve kararlılık (birimler arası Bayesçi model dahil) | [15, 29] |
| calibration | iki değişkende de belirsizlik bulunan genelleştirilmiş en küçük kareler | [18] |
| precision experiment | aykırı değer testleriyle tekrarlanabilirlik ve tekrar üretilebilirlik | [16, 17] |
| control chart | X ve R grafikleri, sınırlar, kurallar, uzun dönem gözden geçirme | [11] |
| sampling | çift numune yöntemi (klasik ANOVA; aralık ve logaritmik dönüşümlü türevleri), temel numune alma hatası; kılavuzun dayanıklı ANOVA'sı metninden yeniden üretilemiyor (F63) | [30] |
| error function | tartı aletleri, hata yaklaşım fonksiyonu ve kullanımdaki belirsizlik | [14] |
| consensus | parametrik bootstrap ile DerSimonian–Laird, Linear Pool, en çok olabilirlik, Bayesçi rastgele etkiler (Gauss veya Laplace); eşdeğerlik dereceleri (MRA, birini dışarıda bırakma, ikili) | [23, 31, 32] |

### 2.2 Ölçüm modelleri ve GUM

Modeller ifade olarak yazılır; ayrıştırıcı bunların türevini sembolik olarak alır, böylece duyarlılık katsayıları
kesindir ve yazdırılabilir. Birleşik standart belirsizlik korelasyonları içerir; GUM'un 2026 değişikliğiyle [6]
getirilen kestirimin ikinci mertebe terimi her zaman hesaplanır ve istenirse kestirime eklenir; korelasyonlu normal
girdiler için ikinci mertebe varyans terimleri Mana ve Pennecchi'yi [26] izler. Aynı verilerden değerlendirilen
girdiler bir *topluluk* olarak tanımlanabilir; topluluk, Welch–Satterthwaite formülüne ortak serbestlik derecesiyle
tek bir bileşen olarak girer (Willink [33]; GUM Tree Calculator aynı kuralı uygular [21]). Genişletilmiş
belirsizlikler EA-4/02 kuralına göre (olağan yuvarlama U'yu %5'ten fazla küçültmüyorsa olağan yuvarlama, küçültüyorsa
yukarı yuvarlama) [12], Eurachem/CITAC MUPR kılavuzunun kuralına göre [9] ya da her zaman yukarı yuvarlanır; sonuç
ifadesi kullanılan kuralı belirtir. Kalibrasyon raporları, kalibrasyon ve ölçüm yeteneğini raporlanabilecek en
küçük belirsizlik olarak uygulayabilir (ILAC P14 [34]).

### 2.3 Monte Carlo yöntemi

Üreteç, JCGM 101'in önerdiği geliştirilmiş Wichmann–Hill üretecidir [3, 35] ve birincil kaynağıyla
karşılaştırılmıştır; normal sapmalar Box–Muller dönüşümünden, gama sapmaları Marsaglia ve Tsang yönteminden [36]
elde edilir. JCGM 101 7.9'un uyarlamalı yöntemi belirtilen sayısal toleransta durur. Program, olasılık bakımından
simetrik ve en kısa kapsama aralığının yanında, kestirim çevresinde simetrik aralığın yarı genişliğini de raporlar;
bu, JCGM çalışma gruplarının önerdiği genişletilmiş belirsizlik tanımına karşılık gelir (D61). GUM sonucu Monte
Carlo sonucuyla geçerli kılınır (JCGM 101 madde 8); tekrarlı gözlemlerden gelen girdiler bulunduğunda rapor, az
serbestlik derecesi için iki değerlendirmenin farklı girdi varyansları kullandığını (s/√n'ye karşı
√(ν/(ν − 2))·s/√n) belirtir; dolayısıyla başarısız bir geçerli kılma bir hata anlamına gelmeyebilir (D85).

### 2.4 Uzlaşı değerleri ve eşdeğerlik dereceleri

Uzlaşı modülü NIST Consensus Builder'ı [23, 32] izler ve onun herkese açık R kaynak koduyla karşılaştırılmıştır.
DerSimonian–Laird kestirimi [37], Knapp–Hartung düzeltmesiyle [38] ve NICOB'un parametrik bootstrap'ıyla birlikte
verilir; bu bootstrap, laboratuvarlar arası varyansı Biggerstaff ve Tweedie'nin [39] Cochran Q istatistiği için
verdiği gama yaklaşımından çeker. Kod okunduğunda kılavuzun belirtmediği iki ayrıntı görüldü: gama dağılımı τ²'nin
kesilmemiş moment kestirimine göre kurulur (bu kestirim Q < n − 1 olduğunda negatiftir) ve varyansı doğrusal terimde
yayımlanmış formülden farklıdır; τ̂ > 0 olduğunda fark u'da %0,1, Q < n − 1 olduğunda %13'tür (D82). Program kodu
izler ve yayımlanmış biçimi karşılaştırma için saklar.

u_j değerleri bilinen ve τ üzerinde yarım Cauchy önselli hiyerarşik Bayesçi model MCMC kullanılmadan
değerlendirilir: μ analitik olarak integre edilir ve τ'nun sonsal dağılımı, önsel olasılıkta düzgün bir ızgara
üzerinde, τ = s·tan(πθ/2), hesaplanır; bu ızgarada yarım Cauchy önseli sabittir. Laplace dağılımlı laboratuvar
etkileri için normal–Laplace evrişimi kapalı biçimde yazılır ve sonsal dağılım bir (θ, μ) ızgarası üzerinde
hesaplanır. Sonuçlar bir tohum değerine bağlı değildir; ızgarayı iki katına çıkarmak ortalamaları, standart
sapmaları ve yüzdelikleri sonsal standart sapmanın 10⁻³'ünden daha az değiştirir (D83). Eşdeğerlik dereceleri
DerSimonian–Laird, Linear Pool ve Bayesçi yöntemler için, CIPM MRA sürümünde (D_j = x_j − μ̂) ve birini dışarıda
bırakma sürümünde (D*_j = x_j − μ̂₋ⱼ), tekli ve ikili olarak hesaplanabilir (D86, D88).

### 2.5 Fiziksel bir sınıra yakın sonuçlar

Bir kütle kesri 1 g/g'ı aşamaz. y ± U böyle bir sınırı aştığında program aralığı sınırda keser ve sonucu, IUPAC'ın
saflık belirleme üzerine teknik raporunun izin verdiği gibi [40], y +U_U/−U_L olarak ifade eder; Monte Carlo
yönteminde ayrıca sınırla kısıtlanmış sonucu (sınır dışındaki çekilişler atılır; bu, ölçülen büyüklük üzerinde
dikdörtgen bir önsele karşılık gelir [41]) kestirimi, standart sapması ve kapsama aralığıyla raporlar (D87).

### 2.6 Programın ve kütüphanesinin yeniden üretilebilirliği

Kütüphanedeki her proje, kaynağının sayısal verilerinden bir betik (`tools/generators/`) tarafından yazılır; kaynak
bölümü ve bağlantısıyla belirtilir. Lisanslı belgeler yalnızca sayısal verileri için kullanılır ve dağıtılmaz. Depo,
her kararın ve bulgunun araştırma günlüğünü, her bağımsız yeniden hesabın betiklerini (`tools/research/`) ve bu
makalenin üretilmiş tablolarını içerir.

## 3. Validasyon tasarımı

### 3.1 Kütüphane

Kütüphane 38 belgeden 286 proje içerir: GUM, ekleri ve GUM-5; Eurachem/CITAC, Nordtest, EA, UKAS ve EURAMET
kılavuzları; beş ISO standardı; NIST TN 1900 [42] ve Uncertainty Machine kılavuzu; EMPIR EMUE örnek derlemesi [43];
IUPAC kısa kılavuzu [44]; BIPM qNMR referans veri raporları [45]; Koepke ve ark. [32] ile Merkatas ve ark. [46]
uzlaşı makaleleri; Possolo ve ortak yazarlarının kitap ve makaleleri [47, 48]. Alanlara göre 144 proje kimya, 36
elektrik, 31 kütle, 24 genel, 16 fiziksel büyüklükler, 12 uzunluk, 10 sıcaklık, 7 akış ve 6 basınç alanındadır;
değerlendirme türüne göre 134'ü ölçüm modelidir (118'i Monte Carlo değerlendirmeli), 152'si diğer on türü kapsar
(Tablo 2).

<!-- table2 -->

### 3.2 Referans değerleri ve toleranslar

Referans değeri, kaynağın kendi girdilerinden kendi yuvarlaması içinde yeniden ürettiği her durumda, kaynakta basılı
olan değerdir ve toleransı son basamağının yarım birimidir (D15). Basılı değerin kendisi bir Monte Carlo veya MCMC
kestirimi olduğunda, onun örnekleme hatası toleransa eklenir. Kaynak hiçbir şey basmadığında ya da girdilerinden
çıkmayan bir değer bastığında, referans değeri bağımsız bir yeniden hesaptır (`tools/research/` içindeki
numpy/scipy betikleri) ve tutarsızlık bir bulgu olarak kaydedilir; bir motorun kendi çıktısı hiçbir zaman kendi
referansı olarak kullanılmaz. Programın Monte Carlo sonuçları, ölçülen dağılımlarının yaklaşık beş katı bir
toleransla karşılaştırılır.

1865 referans değerinden 938'inin toleransı son basamağın yarım birimi, 244'ünün en çok bir birimdir; 281'i Monte
Carlo değeridir ve 402'sinin toleransı daha geniştir, nedeni projede belirtilmiştir (örneğin iki anlamlı basamakla
basılmış bir girdiden türetilen değer). Tüm değerler üzerinden |Δ|/tolerans oranının medyanı 0,23, 99. yüzdeliği
0,98'dir (`numbers.md`). Test takımı her değişiklikte tüm değerleri yeniden hesaplar; programın bir laboratuvar için
yazdığı yazılım validasyon raporu, her proje dosyasının özetini, derlemenin commit'ini ve derleyicisini listeler ve
laboratuvarın kendi projelerini ayrı bir bölümde verir.

### 3.3 Tutarsızlıkların sınıflandırılması

Bulguların denetiminden önce sınıflar sabitlendi (D12, D73): (a) basılı bir basamağı değiştiren ara yuvarlama; (b)
sonuca etkisi olmayan baskı veya aktarma hatası; (c) tutarsızlık; (n) tutarsızlık yok (yöntemsel bir not veya tam
yeniden üretim). İç ön incelemeden sonra (Bölüm 7) c sınıfı bulgular etkilerine göre ayrıca bölündü (D84): c1, basılı
bir nihai sonuç (kestirim, belirsizliği veya ifade edilen hüküm) basılı basamaklarında yeniden üretilemiyor; c2,
yalnızca bir ara değer yeniden üretilemiyor; m, tutarsızlık değil yöntem veya yorum farkı; e, yayımlanmış bir
düzeltmeyle giderilmiş hata. Bir bulgu, belgenin kendi verilerinden yapılan yeniden hesap açık soru bırakmadığında ya
da bulgu bağımsız olarak yeniden üretildiğinde doğrulanmış sayılır; başka bir kaynakla karşılaştırılmayı bekleyen
bulgular sayılmaz.

### 3.4 Bağımsız karşılaştırma

Suncal 1.7.1'in ifade edebildiği her model projesi (skaler çıktılı, korelasyonsuz ve ara büyüklüksüz 107 proje) bir
Suncal modeline çevrildi ve iki programla değerlendirildi (D74).

## 4. Validasyon sonuçları

286 projenin 1865 referans değerinin tamamı toleransları içinde yeniden üretilmektedir. Validasyon matrisi 228
çözümlü örneği veya türevi yeniden üretilmiş olarak kaydeder; bunların 25'inde bir bulgu vardır. Suncal ile
birinci mertebe standart belirsizlikler bağıl 6,4 × 10⁻¹² düzeyinde uyuşur; kestirimler, 2026 değişikliğinin
ikinci mertebe kestirim terimini isteyen iki proje dışında aynıdır (tasarım gereği Δy = 0,5u ve 0,1u; Suncal birinci
mertebedir). Girdilerini her iki programın da örnekleyebildiği 48 projede Monte Carlo standart sapmaları en çok
%0,3 farklıdır; fark, her iki programın örnekleme hatalarını ve çıktının basıklığı için bir düzeltmeyi birleştiren
farkın standart hatasına bölündüğünde en büyük |z| 3,0'dır; 48 bağımsız karşılaştırma için bunun olasılığı
0,12'dir, yani dağılım yalnızca örnekleme hatasıyla tutarlıdır.

<!-- table3 -->

Uzlaşı modülü; Koepke ve ark. [32] için Newton kütle çekim sabitinin ve ¹³⁷Cs ile ⁹⁰Sr yarı ömürlerinin, IUPAC 2024
[44] için sığır karaciğerindeki nikelin ve iridyumun molar kütlesinin, Laplace etkileriyle Possolo ve Meija [47]
için Hubble–Lemaître sabitinin (Stan ile 71,69'a karşı 71,69 (km/s)/Mpc) yayımlanmış DerSimonian–Laird, bootstrap,
Linear Pool, en çok olabilirlik ve hiyerarşik Bayesçi değerlerini ve NICOB kılavuzunun ⁶⁰Co ve PCB 28 uzlaşı
değerlerini [23] yeniden üretir. Kılavuzun yalnızca şekillerde gösterdiği eşdeğerlik dereceleri için NICOB R
kodunun numpy'ye bağımsız bir aktarımı referans olarak kullanılır: D değerleri tam olarak, genişletilmiş
belirsizlikler 10⁵ çekilişin Monte Carlo dağılımı olan %1,7 içinde uyuşur.

## 5. Yeniden hesabın ortaya koydukları

### 5.1 Genel bakış

Örneklerin yeniden hesaplanması 100 bulgu (F1–F100) ortaya çıkardı. Sınıflarına göre 27'si ara yuvarlama, 25'i
etkisiz baskı hatası, 23'ü tutarsızlık, 13'ü tutarsızlık yok sınıfındadır ve 12'si iki sınıfı birleştirir; 29'unun
c sınıfı bir bileşeni vardır. Bunlardan 13'ü basılı bir nihai sonucu değiştiren doğrulanmış tutarsızlıklardır (c1),
7'si yalnızca ara değerleri etkiler (c2; ikisi düzeltmelerle kısmen giderilmiş), 5'i yöntem veya yorum farkıdır (m),
1'i bilinen bir düzeltmedir (e) ve 5'i açıktır ve sayılmaz (Tablo 4). Bulgular belgelerin çoğunu ilgilendirir
(Şekil 2): 2026 tarihli GUM-5 örneklerinde 13, Eurachem QUAM'da 11, UKAS M3003 ve EMUE derlemesinde 7'şer bulgu
vardır (iki sınıflı bir bulgu her ikisinde de sayılmıştır).

<!-- fig2 -->

**Tablo 4.** Basılı bir nihai sonucu değiştiren doğrulanmış tutarsızlıklar (c1 sınıfı; `class-c-impact.tsv`).

| F | Belge | Büyüklük | Basılı | Yeniden hesaplanan | Not |
|---|---|---|---|---|---|
| F21 | EA-4/02 S13 | d_X / mm | 90,0003 | 90,0002 | bütçedeki Δl ve eş eksenlilik düzeltmesi verilerden farklı |
| F28 | NIST TN 1900 E4 | v / (m/s) | 40,64 | 57,48 | basılı değer = verilen denklem ÷ √2; Delker [20] tarafından bağımsız olarak bildirilmiş |
| F39 | ISO 21748 C.3 | U(un) / % | 12,8 | 12,6 | |
| F40 | ISO 5725-3:2023 J.2 | s_R, düzey 6 | 15,962 | 16,33 | Tablo J.2'de aktarma hataları |
| F42 | ISO 6143:2025 Örn. 3 | u(x) / (µmol/mol) | 0,013280 | 0,014246 | parametre belirsizliklerinin yanlış olduğu standartta kabul edilmiş; u(x) için edilmemiş |
| F56 | UKAS M3003 K.5 | U / mg | 0,30 | 0,31 | basılı bileşenlerin birleşik u değeri |
| F57 | UKAS M3003 K.3 | U / dB | 0,049 | 0,040 | uyumsuzluk terimi |
| F58 | UKAS M3003 K.6 | U / nm | 81,5 | 82 | mastar blok modelinin sıcaklık terimleri |
| F77 | GUM-5 Örn. 15 | MCM s.s. / kN | 10,1 | 9,93 | t girdisinin ölçeği |
| F78 | GUM-5 Örn. 1 | pH_X | 7,0109 | 7,0043 | kestirim verilerden çıkmıyor |
| F82 | GUM-5 Örn. 4 | u(y_CO₂) / (µmol/mol) | 5 | 4 | |
| F87 | EMUE E3.4 | U / (mg/kg) | 2,8 | 2,9 | bağıl sapma belirsizliklerinin karesel ortalaması |
| F90 | EMUE E3.3 | u / (nmol/mol) | 4,7 | 4,47 | kovaryans matrisinin köşegeni |

*Göndermeden önce her satır güncel baskı ve yayımlanmış düzeltmelerle karşılaştırılacak ve kodu görmemiş bir kişi
tarafından yeniden hesaplanacaktır (Bölüm 7).*

### 5.2 Ara yuvarlama her yerde

Belgelerin çoğu ara değerleri yuvarlar (bir duyarlılık katsayısını üç basamağa, bir bileşeni iki basamağa) ve sonra
yuvarlanmış sayıları birleştirir; böylece basılı sonuç, kesin bir yeniden hesaptan son basamağında farklıdır. Bu,
sonuç için zararsızdır, ancak örneği basılı basamaklarıyla bir validasyon referansı olarak kullanılamaz hâle getirir
ve gerçek yanlışları yuvarlama farkları arasında gizler. Çözümlü örnekler girdilerini yuvarlamadan vermeli ve
yalnızca raporlanan sonucu yuvarlamalıdır.

### 5.3 Hesap tablosu bütçeleri kendisiyle çelişiyor

BIPM qNMR referans veri raporları, ortak bir hesap tablosundan alınmış tam saflık bütçeleri basar (F97). 11 tablonun
7'sinde integral oranının basılı belirsizlik bileşeni, basılı duyarlılık katsayısı ile standart belirsizliğin
çarpımını %3–17 aşar; beş tabloda oranın serbestlik derecesi 4'e sabitlenmiştir. Sertifikalı değerler sorgulanmıyor;
mesele, bileşenleri kendi sütunlarından yeniden hesaplanamayan bir bütçenin validasyon referansı olamayacağı ve
girdilerden yeniden hesabın bunu hemen ortaya çıkardığıdır.

### 5.4 Monte Carlo ve MCMC sonuçları birer kestirimdir

Basılı Monte Carlo ve MCMC değerleri örnekleme hatası taşır ve birkaç tutarsızlık bu büyüklüktedir: kesin değer
0,185 a iken basılı 0,19 a Bayesçi standart sapma (F95), 5 × 10⁴ çekilişten elde edilmiş korelasyonlar (F96), 10⁶
tekrarla elde edilen 0,752 ng/g yerine basılı 0,77 ng/g bootstrap standart belirsizliği (F99). Deterministik bir
değerlendirme varsa belirsizliği ortadan kaldırır; yoksa çekiliş sayısı sonuçla birlikte basılmalıdır.

### 5.5 Yöntem farkları hata değildir

Beş c sınıfı bulgunun yöntem veya yorum farkı olduğu anlaşıldı. GUM-5 Örnek 2'de analitin ve iç standardın iki alanı
aynı üç enjeksiyondan gelir; ortak varyans bileşenleri Willink'in topluluk kuralına [33] göre iki serbestlik
derecesine sahiptir, örnek ise genelleştirilmiş bir Welch–Satterthwaite formülü uygulayarak ν_eff = 4,8 bulur
(F79). ν bu iki değerde sabit tutulduğunda GUM aralığı, ölçümün bir benzetiminde %89 ve %96 kapsar; ν, bir
laboratuvarın yapacağı gibi her benzetilmiş veri kümesinden yeniden hesaplandığında her iki okuma da %93–94 kapsar.
Seçim tek bir sonucun kapsam faktörü için önemlidir (2,6 ile 4,1), ancak iki okumanın hiçbiri tutarsızlık
değildir. t dağılımlı girdiler için iki gelenek de (JCGM 101'in ölçeklenmiş ve kaydırılmış t dağılımı ile TN 1900 ve
NIST Uncertainty Machine'in yeniden ölçeklenmiş t dağılımı) aynı veriler için farklı Monte Carlo sonuçları verir
(F31, F35).

## 6. Sonucu değiştiren seçimler

### 6.1 Uzlaşı yöntemi modelin bir parçasıdır

Merkatas ve ark.'nın [46] incelediği Newton kütle çekim sabitinin 16 değeri için program; Knapp–Hartung
düzeltmeli DerSimonian–Laird (6,673 99 × 10⁻¹¹ m³ kg⁻¹ s⁻², u = 0,000 25), en çok olabilirlik (6,673 90; 0,000 25),
NICOB hiyerarşik Bayesçi model (6,673 89; 0,000 27; MCMC yerine kesin ızgara) ve Linear Pool (6,673 77; 0,001 17)
sonuçlarını yeniden üretir; makalenin on dört yöntemi 0,000 11 ile 0,001 17 arasında standart belirsizliklerle
6,673 77–6,674 29 aralığına yayılır.

Eşdeğerlik dereceleri de yönteme aynı ölçüde bağlıdır. NICOB kılavuzundaki ⁶⁰Co aktivite karşılaştırmasının 19
sonucu için (katılımcılar kodlanmıştır), ikili eşdeğerlik derecesi sıfırdan anlamlı olarak farklı olan çiftlerin
sayısı 171 çift içinde DerSimonian–Laird ile MRA sürümünde 4, birini dışarıda bırakma sürümünde 9; Bayesçi modelle
3 ve 7; MRA sürümünde Linear Pool ile 22'dir. Linear Pool'un ikili belirsizlikleri yalnızca iki katılımcının
belirttiği belirsizlikleri içerir, karanlık belirsizliği içermez (Şekil 3). NICOB kodu ile kılavuzu, Linear Pool
eşdeğerlik derecelerinin nasıl hesaplandığı konusunda farklıdır (F100); program kılavuzu uygular. Dolayısıyla bir
katılımcının "anlamlı" eşdeğerlik derecesi sonuç kadar yöntem hakkında da bir ifadedir; yöntem, veriler görülmeden
önce seçilmeli ve belirtilmelidir.

<!-- fig3 -->

### 6.2 Sınıra yakın sonuçlar

İç standartla belirlenen benzoik asit saflığı için (BIPM ISRD-06, Tablo 6) y = 0,999 77 g/g ve U = 0,0015 g/g'dır.
y ± U'yu 1 g/g'da kesmek basılı 999,8 +0,2/−1,5 mg/g sonucunu verir. w ≤ 1 g/g ile kısıtlanmış Monte Carlo sonucu
çekilişlerin %62'sini tutar ve 0,999 32 g/g, u = 0,000 49 g/g ve en kısa %95 aralığı [0,998 39; 1] g/g verir: farklı
bir kestirim ve daha dar bir aralık. İkisi de IUPAC kılavuzunun [40] savunulabilir okumalarıdır; rapor hangisinin
kullanıldığını belirtmelidir.

### 6.3 Test belirsizlik oranından koruma bantları

Kalibrasyonda kabul sınırları çoğu zaman test belirsizlik oranından, TUR = (T_U − T_L)/2/U₉₅, belirlenir. Suncal'ın
yaptığı gibi U₉₅ = 2u almak, büyük serbestlik derecelerinde TUR'u U₉₅ = k₉₅u'ya göre yaklaşık %2 düşük, küçük
serbestlik derecelerinde yüksek verir; Dobbert'in yönetilen koruma bandı, sınırlanmadığı sürece TUR = 4,59'un
üzerinde tolerans dışında kalan kabul sınırları verir (D80). Program varsayılan olarak k₉₅u kullanır, 2u'yu seçenek
olarak tutar ve üzerinde anlaşılmış bir karar kuralı daha fazlasına izin vermedikçe faktörü 1 ile sınırlar.

## 7. Tartışma

**Validasyon verisi olarak çözümlü örnekler.** Kılavuz belgelerin örnekleri fiilen belirsizlik yazılımlarının
referans verileridir, ancak dizilmiş metin olarak yayımlanırlar. Bunları makinece okunabilir biçime dönüştürmek ve
yeniden hesaplamak, dikkatli okumanın tek başına yakalayamadığı Bölüm 5'teki sorunları ortaya çıkardı. Yayımcı
kuruluşların örneklerinin girdilerini makinece okunabilir biçimde yayımlamasını (bu programın proje biçimi bir
adaydır) ve yayımdan önce yeniden hesaplamasını öneriyoruz. Bu çalışmanın kütüphanesi bu amaçla sunulmaktadır.

**Bu validasyonun gösterdiği ve göstermediği.** Yayımlanmış değerleri yeniden üretmek, belgelenmiş yöntemlerin
uygulanmasını doğrular; bir laboratuvarın kendi ölçüm yöntemini geçerli kılmaz. Programın validasyon raporunun
laboratuvarın projelerini kendi bölümünde tutmasının nedeni budur. Yayımlanmış değerlerle uyum, program ve bağımsız
betikler bir standardı aynı biçimde okuduğunda ortak kaynaklı bir hatayı da gizleyebilir; gerçekten bağımsız
denetimler Suncal karşılaştırması, NICOB kaynak kodu ve NIST Uncertainty Machine'dir. Bir yapay zekâ asistanıyla iç
ön inceleme yapılmıştır (Bölüm 8); göndermeden önce, proje dışındaki metrologlarca bağımsız bir inceleme, c1
bulgularından en az onunun kodu görmemiş bir kişi tarafından yeniden hesaplanması dahil, planlanmaktadır.

**Bulguların bildirilmesi.** Bulgular, gönderimden önce yayımcı kuruluşlara bildirilmemiştir. Tablo 4'teki c1
bulguları, hakemli bir kayda atıf yapabilmeleri için makale kabul edildikten sonra kuruluşlara teknik not olarak
iletilecek; yanıtları programın herkese açık araştırma günlüğüne eklenecektir.

**Sınırlamalar.** Sonlu serbestlik dereceli sonuçlar için σ_j üzerindeki NICOB önselleri, Laplace etkileriyle
eşdeğerlik dereceleri, zaman serileri (ARMA) ve sayım istatistikleri, model ortalaması ve imzalı sonuç paketleri
uygulanmamıştır; PDF raporları ve SHA-256 özetleri bütünlük denetimi sağlar ama imza sağlamaz. Lisanslı standartlar
yalnızca sayısal verileri için kullanılmıştır.

## 8. Yazılımın erişilebilirliği ve yapay zekâ araçlarının kullanımı

Program GPL-3.0 lisansıyla https://github.com/unilker/uncertainty adresinde erişilebilir durumdadır; bu makalede
kullanılan sürüm [sürüm etiketi ve Zenodo DOI eklenecek]; ön inceleme 809ebcf commit'ini kullanmıştır. `cargo test`
Bölüm 4–6'daki her sayıyı yeniden üretir; `tools/research/` içindeki betikler tabloları ve şekilleri yeniden
oluşturur.

Kod, kütüphane üreteçleri, bağımsız yeniden hesaplar ve bir iç ön inceleme, yazarın yönlendirmesiyle bir yapay
zekâ kodlama asistanı (Claude, Anthropic) kullanılarak üretilmiştir; referans değerleri yayımlanmış belgelerden
gelir ve bağımsız denetimler Bölüm 7'de adı geçenlerdir. [İfade derginin politikasına göre uyarlanacaktır.]

## Kaynaklar

[1] ISO/IEC 17025:2017, General requirements for the competence of testing and calibration laboratories. ISO, Geneva.

[2] JCGM 100:2008, Evaluation of measurement data — Guide to the expression of uncertainty in measurement. BIPM.

[3] JCGM 101:2008, Evaluation of measurement data — Supplement 1 to the GUM — Propagation of distributions using a Monte Carlo method. BIPM.

[4] JCGM 102:2011, Evaluation of measurement data — Supplement 2 to the GUM — Extension to any number of output quantities. BIPM.

[5] JCGM 106:2012, Evaluation of measurement data — The role of measurement uncertainty in conformity assessment. BIPM.

[6] JCGM 100:2008/Amd.1:2026, Evaluation of measurement data — Guide to the expression of uncertainty in measurement. Amendment 1: Nonlinearity in measurement models. BIPM, 2026.

[7] JCGM GUM-5:2026, Guide to the expression of uncertainty in measurement — Part 5: Examples. BIPM. https://doi.org/10.59161/YNLY8209

[8] Eurachem/CITAC Guide CG 4, Quantifying Uncertainty in Analytical Measurement, 3rd ed., 2012 (QUAM:2012.P1).

[9] Eurachem/CITAC Guide, Evaluation of measurement uncertainty from in-house precision and recovery data (MUPR), 1st ed., 2026.

[10] Nordtest TR 537, Handbook for calculation of measurement uncertainty in environmental laboratories, ed. 4, 2017.

[11] B. Magnusson, H. Hovind, M. Krysell, U. Lund, I. Mäkinen, E. Sahlin, Handbook of Internal Quality Control, Nordtest TR 569, ed. 6, 2026.

[12] EA-4/02 M:2022, Evaluation of the Uncertainty of Measurement in Calibration. European co-operation for Accreditation.

[13] UKAS M3003, The Expression of Uncertainty and Confidence in Measurement, ed. 6, 2024.

[14] EURAMET Calibration Guide No. 18, Guidelines on the Calibration of Non-Automatic Weighing Instruments, v4.0, 2015.

[15] ISO 13528:2022, Statistical methods for use in proficiency testing by interlaboratory comparison.

[16] ISO 5725-2:2019, Accuracy (trueness and precision) of measurement methods and results — Part 2.

[17] ISO 5725-3:2023, Accuracy (trueness and precision) of measurement methods and results — Part 3.

[18] ISO 6143:2025, Gas analysis — Comparison methods for determining and checking the composition of calibration gas mixtures.

[19] T. Lafarge, A. Possolo, The NIST Uncertainty Machine, NCSLI Measure 10(3) (2015) 20–27. https://doi.org/10.1080/19315775.2015.11721732

[20] C. J. Delker, A comprehensive open-source software for statistical metrology calculations: from uncertainty evaluation to risk analysis, NCSLI Measure 13(3) (2021); Sandia report SAND2021-8022J, OSTI 1830511. [doğrula: sayfalar]

[21] B. D. Hall, J. S. Borbely, The GUM Tree Calculator: a Python package for measurement modelling and data processing with automatic evaluation of uncertainty, Metrology 2 (2022) 128–149. https://doi.org/10.3390/metrology2010009

[22] M. Zeier, J. Hoffmann, M. Wollensack, Metas.UncLib — a measurement uncertainty calculator for advanced problems, Metrologia 49 (2012) 809–815. https://doi.org/10.1088/0026-1394/49/6/809

[23] A. Koepke, T. Lafarge, A. Possolo, B. Toman, NIST Consensus Builder — User's Manual, NIST, 2020. https://consensus.nist.gov

[24] R. J. N. Bettencourt da Silva, User-friendly spreadsheet for implementing international guidance on the evaluation of measurement uncertainty, Talanta (2026) 130826. https://doi.org/10.1016/j.talanta.2026.130826

[25] F. R. Pennecchi, P. M. Harris, Mathmet Measurement Uncertainty Training activity — overview of courses, software, and classroom examples, Acta IMEKO 12(2) (2023) 1–6.

[26] G. Mana, F. Pennecchi, Uncertainty propagation in non-linear measurement equations, Metrologia 44 (2007) 246–251. https://doi.org/10.1088/0026-1394/44/3/012

[27] Eurachem/CITAC Guide, Use of uncertainty information in compliance assessment, 2nd ed., 2021.

[28] ISO 21748:2017, Guidance for the use of repeatability, reproducibility and trueness estimates in measurement uncertainty evaluation.

[29] A. Possolo, C. Bodnar, Approximate Bayesian evaluations of measurement uncertainty, Metrologia 55 (2018) 147–157. https://doi.org/10.1088/1681-7575/aaa5be

[30] M. H. Ramsey, S. L. R. Ellison, P. Rostron (eds.), Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide: Measurement uncertainty arising from sampling, 2nd ed., 2019.

[31] M. Thompson, S. L. R. Ellison, Dark uncertainty, Accred. Qual. Assur. 16 (2011) 483–487. https://doi.org/10.1007/s00769-011-0803-0

[32] A. Koepke, T. Lafarge, A. Possolo, B. Toman, Consensus building for interlaboratory studies, key comparisons, and meta-analysis, Metrologia 54 (2017) S34–S62. https://doi.org/10.1088/1681-7575/aa6c0e

[33] R. Willink, A generalization of the Welch–Satterthwaite formula for use with correlated uncertainty components, Metrologia 44 (2007) 340–349. https://doi.org/10.1088/0026-1394/44/5/010

[34] ILAC P14:09/2020, ILAC Policy for Measurement Uncertainty in Calibration.

[35] B. A. Wichmann, I. D. Hill, Generating good pseudo-random numbers, Comput. Stat. Data Anal. 51 (2006) 1614–1622.

[36] G. Marsaglia, W. W. Tsang, A simple method for generating gamma variables, ACM Trans. Math. Softw. 26 (2000) 363–372.

[37] R. DerSimonian, N. Laird, Meta-analysis in clinical trials, Control. Clin. Trials 7 (1986) 177–188.

[38] G. Knapp, J. Hartung, Improved tests for a random effects meta-regression with a single covariate, Stat. Med. 22 (2003) 2693–2710.

[39] B. J. Biggerstaff, R. L. Tweedie, Incorporating variability in estimates of heterogeneity in the random effects model in meta-analysis, Stat. Med. 16 (1997) 753–768.

[40] S. Westwood et al., Methods for the SI-traceable value assignment of the purity of organic compounds (IUPAC Technical Report), Pure Appl. Chem. (published online 2022). https://doi.org/10.1515/pac-2020-0804 [doğrula: sayının cilt ve sayfaları]

[41] B. Toman, M. A. Nelson, K. A. Lippa, Chemical purity using quantitative ¹H-nuclear magnetic resonance: a hierarchical Bayesian approach for traceable calibrations, Metrologia 53 (2016) 1193–1203. https://doi.org/10.1088/0026-1394/53/5/1193

[42] A. Possolo, Simple Guide for Evaluating and Expressing the Uncertainty of NIST Measurement Results, NIST Technical Note 1900, 2015. https://doi.org/10.6028/NIST.TN.1900

[43] A. M. H. van der Veen, M. G. Cox (eds.), Good practice in evaluating measurement uncertainty — Compendium of examples, EMPIR 17NRM05 EMUE, 2021.

[44] A. Possolo et al., A brief guide to measurement uncertainty (IUPAC Technical Report), Pure Appl. Chem. 96 (2024) 113–134. https://doi.org/10.1515/pac-2022-1203

[45] BIPM, Internal Standard Reference Data for qNMR, ISRD-01 to ISRD-08, Rapports BIPM 2018–2023.

[46] C. Merkatas, B. Toman, A. Possolo, S. Schlamminger, Shades of dark uncertainty and consensus value for the Newtonian constant of gravitation, Metrologia 56 (2019) 054001. https://doi.org/10.1088/1681-7575/ab3365

[47] A. Possolo, J. Meija, Measurement Uncertainty: A Reintroduction, 2nd ed., SIM, Montevideo, 2022. https://doi.org/10.4224/1tqz-b038

[48] G. Röver, G. Knapp, T. Friede, Hartung–Knapp–Sidik–Jonkman approach and its modification for random-effects meta-analysis with few studies, BMC Med. Res. Methodol. 15 (2015) 99. https://doi.org/10.1186/s12874-015-0091-1
