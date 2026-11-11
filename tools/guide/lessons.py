"""Lessons of the library examples for the beginner's guide.

    python3 tools/guide/lessons.py      (from the repository root)

writes app/assets/guide/lessons.json. Each lesson belongs to the examples whose id starts with its prefix (the
longest prefix wins) and says, in plain words, what the group of examples teaches and what to look at; level 1 is
for beginners, 2 intermediate, 3 advanced. The test `every library example has a lesson` (app/test) checks that
each example is covered. Edit the texts here, not the JSON.
"""
import json
from pathlib import Path

L = []


def lesson(prefix, level, title_en, title_tr, en, tr):
    L.append({"prefix": prefix, "level": level, "title": {"en": title_en, "tr": title_tr}, "en": en, "tr": tr})


# --- Level 1: first steps -----------------------------------------------------------------------------------------

lesson("nist-tn1900-e1", 1, "Weighing a powder", "Bir tozu tartmak",
       "A very simple budget: the mass of a powder is the reading of the filled container minus the reading of the "
       "empty one, with a small correction for the drift of the balance. The two readings have the same uncertainty; "
       "together they give √2 times one of them, not twice. GUM and Monte Carlo give the same result, because the "
       "model is linear.",
       "Çok basit bir bütçe: tozun kütlesi, dolu kabın okumasından boş kabın okumasının çıkarılmasıyla ve terazinin "
       "kayması için küçük bir düzeltmeyle bulunur. İki okumanın belirsizliği aynıdır; birlikte birinin iki katını "
       "değil, √2 katını verirler. Model doğrusal olduğu için GUM ve Monte Carlo aynı sonucu verir.")
lesson("jcgm101-9.2", 1, "Adding inputs: normal and rectangular", "Girdileri toplamak: normal ve dikdörtgen",
       "Four inputs are simply added. The three variants differ only in the shape of the input distributions. "
       "Compare the GUM result with Monte Carlo: when one rectangular input dominates, the result is not normal any "
       "more and the 95 % interval is narrower than 2u. This is the classic case where k = 2 is not right.",
       "Dört girdi basitçe toplanır. Üç türev yalnızca girdi dağılımlarının biçiminde ayrılır. GUM sonucunu Monte "
       "Carlo ile karşılaştırın: tek bir dikdörtgen girdi baskın olduğunda sonuç artık normal değildir ve %95 aralığı "
       "2u'dan dardır. Bu, k = 2'nin doğru olmadığı klasik durumdur.")
lesson("ea402-s2", 1, "Calibrating a weight", "Bir ağırlığın kalibrasyonu",
       "A 10 kg weight is compared with a reference weight. It shows the typical calibration budget: the certificate "
       "of the standard (U and k), drift since its last calibration, the comparator, air buoyancy and the repeated "
       "comparisons. The certificate of the standard is the largest contribution.",
       "Bir 10 kg ağırlık bir referans ağırlıkla karşılaştırılır. Tipik bir kalibrasyon bütçesini gösterir: "
       "standardın sertifikası (U ve k), son kalibrasyondan beri sürüklenme, komparatör, hava kaldırma kuvveti ve "
       "tekrarlı karşılaştırmalar. En büyük katkı standardın sertifikasıdır.")
lesson("m3003-k4", 1, "A 10 kg weight in UKAS M3003", "UKAS M3003'te bir 10 kg ağırlık",
       "The same kind of weight calibration as in EA-4/02, presented in the British guide. Comparing the two shows "
       "that different laboratories make slightly different choices of components and distributions, and that the "
       "result is still similar.",
       "EA-4/02'dekiyle aynı türden bir ağırlık kalibrasyonu, İngiliz kılavuzundaki biçimiyle. İkisini karşılaştırmak, "
       "farklı laboratuvarların bileşenler ve dağılımlar için biraz farklı seçimler yaptığını ve sonucun yine de "
       "benzer çıktığını gösterir.")
lesson("ea402-s10", 1, "Calibrating a vernier calliper", "Bir kumpasın kalibrasyonu",
       "A gauge block is measured with a calliper. Most components are rectangular: the resolution of the scale, "
       "the temperature difference, the flatness of the jaws. A good example of how a reading on a scale is turned "
       "into an uncertainty.",
       "Bir mastar blok bir kumpasla ölçülür. Bileşenlerin çoğu dikdörtgendir: skalanın çözünürlüğü, sıcaklık farkı, "
       "çenelerin düzlüğü. Bir skaladaki okumanın bir belirsizliğe nasıl dönüştüğünün iyi bir örneği.")
lesson("ea402-s3", 1, "Calibrating a standard resistor", "Bir standart direncin kalibrasyonu",
       "A 10 kΩ resistor is compared with a reference resistor. The model is a product, so the sensitivity "
       "coefficients are not 1; look at how the budget converts each relative effect into ohms.",
       "Bir 10 kΩ direnç bir referans dirençle karşılaştırılır. Model bir çarpımdır, bu yüzden duyarlılık katsayıları "
       "1 değildir; bütçenin her bağıl etkiyi nasıl ohm'a çevirdiğine bakın.")
lesson("ea402-s9", 1, "Calibrating a digital multimeter", "Bir dijital multimetrenin kalibrasyonu",
       "A hand-held multimeter is checked at 100 V against a calibrator. The resolution of the display is often the "
       "largest term: a value shown with one decimal can be anywhere within half a digit.",
       "Bir el multimetresi 100 V'ta bir kalibratöre karşı denetlenir. Göstergenin çözünürlüğü çoğu zaman en büyük "
       "terimdir: tek ondalıkla gösterilen bir değer yarım basamak içinde herhangi bir yerde olabilir.")
lesson("ea402-s4", 1, "Calibrating a gauge block", "Bir mastar bloğun kalibrasyonu",
       "A 50 mm gauge block is compared with a reference block. Temperature matters: a small difference in "
       "temperature or in expansion coefficient changes the length, which is why the budget has thermal terms.",
       "Bir 50 mm mastar blok bir referans blokla karşılaştırılır. Sıcaklık önemlidir: sıcaklıktaki ya da genleşme "
       "katsayısındaki küçük bir fark boyu değiştirir; bütçede ısıl terimlerin olmasının nedeni budur.")
lesson("ea402-s11", 1, "A temperature block calibrator", "Bir sıcaklık blok kalibratörü",
       "The temperature of a dry block at 180 °C is measured with a reference thermometer. The budget lists the "
       "effects of the block itself: axial and radial non-uniformity, stability, loading.",
       "180 °C'deki bir kuru bloğun sıcaklığı bir referans termometreyle ölçülür. Bütçe bloğun kendi etkilerini "
       "listeler: eksenel ve radyal homojensizlik, kararlılık, yükleme.")
lesson("ea402-s12", 1, "Calibrating a water meter", "Bir su sayacının kalibrasyonu",
       "Three related projects: the reference volume, the relative error of one run and the mean error of three "
       "runs. They show how a long calculation can be split into steps, each with its own budget.",
       "Üç ilişkili proje: referans hacim, bir koşunun bağıl hatası ve üç koşunun ortalama hatası. Uzun bir "
       "hesabın her biri kendi bütçesine sahip adımlara nasıl bölünebileceğini gösterirler.")
lesson("m3003-k1", 1, "A resistor in UKAS M3003", "UKAS M3003'te bir direnç",
       "A 10 kΩ resistor calibrated by comparing voltages. A clear example of a budget table with the sources, "
       "their values, the divisors (√3, 2, 1) and the resulting standard uncertainties.",
       "Gerilimler karşılaştırılarak kalibre edilen bir 10 kΩ direnç. Kaynakları, değerlerini, bölenleri (√3, 2, 1) "
       "ve sonuçtaki standart belirsizlikleri içeren bir bütçe tablosunun açık bir örneği.")
lesson("m3003-k5", 1, "A weighing machine at 200 g", "200 g'da bir tartı aleti",
       "A weighing machine is checked with a 200 g weight. Repeatability, resolution and the weight's certificate "
       "are the main terms.",
       "Bir tartı aleti 200 g'lık bir ağırlıkla denetlenir. Başlıca terimler tekrarlanabilirlik, çözünürlük ve "
       "ağırlığın sertifikasıdır.")
lesson("m3003-k8", 1, "A pressure indicator at 2 MPa", "2 MPa'da bir basınç göstergesi",
       "A digital pressure indicator is compared with a dead-weight tester. A typical pressure calibration budget.",
       "Bir dijital basınç göstergesi bir ölü ağırlık test cihazıyla karşılaştırılır. Tipik bir basınç kalibrasyonu "
       "bütçesi.")
lesson("m3003-k10", 1, "Thickness of a sheet", "Bir levhanın kalınlığı",
       "The thickness of an aluminium sample is the mean of five readings. It combines a Type A evaluation (the "
       "scatter of the readings) with Type B terms: calibration, resolution, drift and temperature.",
       "Bir alüminyum numunenin kalınlığı beş okumanın ortalamasıdır. A tipi bir değerlendirmeyi (okumaların "
       "dağılımı) B tipi terimlerle birleştirir: kalibrasyon, çözünürlük, kayma ve sıcaklık.")
lesson("cg19", 1, "Calibrating volumetric glassware", "Hacimsel cam malzemenin kalibrasyonu",
       "The volume of a flask or pycnometer is found by weighing the water it holds. You see how the density of "
       "water and air, the temperature and the balance enter the result.",
       "Bir balonjojenin ya da piknometrenin hacmi, aldığı suyun tartılmasıyla bulunur. Suyun ve havanın "
       "yoğunluğunun, sıcaklığın ve terazinin sonuca nasıl girdiğini görürsünüz.")
lesson("quam-a1", 1, "Preparing a calibration standard", "Bir kalibrasyon standardı hazırlamak",
       "A cadmium solution is made by dissolving a weighed amount of pure metal in a known volume. The classic first "
       "example of the Eurachem guide: purity, mass and volume combine as relative uncertainties because the model is "
       "a product and a ratio.",
       "Tartılmış bir miktar saf metal bilinen bir hacimde çözülerek bir kadmiyum çözeltisi hazırlanır. Eurachem "
       "kılavuzunun klasik ilk örneği: model bir çarpım ve oran olduğu için saflık, kütle ve hacim bağıl "
       "belirsizlikler olarak birleşir.")
lesson("nordtest-6", 1, "Bias from a reference material or proficiency tests", "Referans malzemeden ya da yeterlilik testlerinden sapma",
       "The top-down way of finding the systematic part of the uncertainty: measure a certified reference material, "
       "or look at your results in proficiency tests, and turn the differences into u(bias). Compare the variants "
       "with one CRM, several CRMs, PT results and recovery.",
       "Belirsizliğin sistematik kısmını bulmanın yukarıdan aşağı yolu: sertifikalı bir referans malzemeyi ölçün ya da "
       "yeterlilik testlerindeki sonuçlarınıza bakın ve farkları u(sapma)'ya dönüştürün. Tek CRM, birkaç CRM, "
       "yeterlilik testi ve geri kazanım türevlerini karşılaştırın.")
lesson("nordtest-5", 1, "Precision from duplicates", "Çift ölçümlerden kesinlik",
       "Analysing samples in duplicate gives the repeatability; a control sample measured over months gives the "
       "within-laboratory reproducibility. These are the random part of a top-down uncertainty.",
       "Numuneleri çift analiz etmek tekrarlanabilirliği verir; aylar boyunca ölçülen bir kontrol numunesi laboratuvar "
       "içi tekrar üretilebilirliği verir. Bunlar yukarıdan aşağı belirsizliğin rastgele kısmıdır.")
lesson("nordtest-3", 1, "The whole top-down budget in one example", "Tek örnekte bütün yukarıdan aşağı bütçe",
       "Ammonium in water: precision from the control chart and bias from proficiency tests are combined into the "
       "expanded uncertainty. This is the recipe that most testing laboratories use.",
       "Sudaki amonyum: kontrol grafiğinden kesinlik ve yeterlilik testlerinden sapma, genişletilmiş belirsizlikte "
       "birleştirilir. Çoğu deney laboratuvarının kullandığı tarif budur.")
lesson("tr569-table1", 1, "A first control chart", "İlk kontrol grafiği",
       "Sixty control values of zinc in water are plotted with their warning and action limits. Learn to read a "
       "chart: which points are fine, which ones are warnings and which ones demand action.",
       "Sudaki çinkonun altmış kontrol değeri uyarı ve eylem sınırlarıyla çizilir. Bir grafiği okumayı öğrenin: "
       "hangi noktalar sorunsuz, hangileri uyarı, hangileri eylem gerektiriyor.")
lesson("tr569-ch7", 1, "Setting the limits of a control chart", "Kontrol grafiği sınırlarını belirlemek",
       "The three ways of setting limits: from the data (statistical), from a target, or around a reference value. "
       "Compare the variants to see why target limits are often wider and more useful.",
       "Sınırları belirlemenin üç yolu: verilerden (istatistiksel), bir hedeften ya da bir referans değer etrafında. "
       "Hedef sınırların neden çoğu zaman daha geniş ve daha kullanışlı olduğunu görmek için türevleri karşılaştırın.")
lesson("jcgm106-7", 1, "Probability that an item conforms", "Bir ürünün uygun olma olasılığı",
       "A measured value lies inside the specification, but how sure can we be? These examples calculate the "
       "probability of conformity from the measured value and its uncertainty, for one-sided and two-sided limits.",
       "Ölçülen değer spesifikasyonun içinde, ama ne kadar emin olabiliriz? Bu örnekler, ölçülen değer ve "
       "belirsizliğinden uygunluk olasılığını tek ve iki taraflı sınırlar için hesaplar.")
lesson("iso13528-e3", 1, "The assigned value of a proficiency test", "Yeterlilik testinin atanan değeri",
       "The same results evaluated with two robust estimators: Algorithm A and the median. Robust statistics keep a "
       "few outlying laboratories from moving the assigned value.",
       "Aynı sonuçlar iki dayanıklı kestiriciyle değerlendirilir: Algoritma A ve medyan. Dayanıklı istatistikler, "
       "birkaç aykırı laboratuvarın atanan değeri kaydırmasını önler.")
lesson("iso13528-e4", 1, "Scoring laboratories: z, ζ and E_n", "Laboratuvarları puanlamak: z, ζ ve E_n",
       "Every participant gets a z-score (is the result close enough to the assigned value?) and ζ and E_n scores "
       "(is the result consistent with the uncertainty the laboratory claimed?). The table of the standard is "
       "reproduced for all 21 numerical results.",
       "Her katılımcı bir z puanı (sonuç atanan değere yeterince yakın mı?) ve ζ ile E_n puanları (sonuç "
       "laboratuvarın beyan ettiği belirsizlikle tutarlı mı?) alır. Standardın tablosu 21 sayısal sonucun hepsi "
       "için yeniden üretilir.")
lesson("possolo-meija-2022-plasma-anion-gap", 1, "A sum of four concentrations", "Dört derişimin toplamı",
       "The plasma anion gap is a linear combination of four ion concentrations. With a linear model GUM and Monte "
       "Carlo agree perfectly: a good check that you understand the basics.",
       "Plazma anyon açığı dört iyon derişiminin doğrusal bir birleşimidir. Doğrusal bir modelde GUM ve Monte Carlo "
       "tam uyuşur: temelleri anladığınızı denetlemek için iyi bir örnek.")
lesson("iupac2024-4a", 1, "Hardness of water", "Suyun sertliği",
       "Total hardness from the calcium and magnesium concentrations: a short, conventional measurement model from "
       "the IUPAC guide, ideal for practising the steps of the GUM.",
       "Kalsiyum ve magnezyum derişimlerinden toplam sertlik: IUPAC kılavuzundan kısa, alışılmış bir ölçüm modeli; "
       "GUM adımlarını alıştırma yapmak için ideal.")
lesson("iupac2024-4i", 1, "A titration", "Bir titrasyon",
       "The concentration of sodium chloride by titration with silver nitrate. Volumes, the concentration of the "
       "titrant and the endpoint are the inputs.",
       "Sodyum klorür derişimi gümüş nitratla titrasyonla bulunur. Hacimler, titrantın derişimi ve dönüm noktası "
       "girdilerdir.")
lesson("gum5-e3", 1, "Molar mass from atomic weights", "Atom ağırlıklarından molar kütle",
       "The relative molecular mass of glucose from the standard atomic weights, which are given as intervals. An "
       "interval with no other information becomes a rectangular distribution.",
       "Glukozun bağıl molekül kütlesi, aralık olarak verilen standart atom ağırlıklarından bulunur. Başka bilgi "
       "olmayan bir aralık dikdörtgen dağılıma dönüşür.")
lesson("gum5-e14", 1, "Radioactive decay", "Radyoaktif bozunma",
       "The activity of a source is corrected for the time since its reference date. The half-life and the time "
       "enter through an exponential function.",
       "Bir kaynağın aktivitesi referans tarihinden bu yana geçen süre için düzeltilir. Yarı ömür ve süre üstel bir "
       "fonksiyon üzerinden girer.")
lesson("nist-tn1900-e2", 1, "Mean of daily temperatures", "Günlük sıcaklıkların ortalaması",
       "The monthly mean of daily maximum temperatures: a pure Type A evaluation. With few days, the t distribution "
       "makes the coverage factor clearly larger than 2.",
       "Günlük en yüksek sıcaklıkların aylık ortalaması: saf bir A tipi değerlendirme. Gün sayısı azken t dağılımı "
       "kapsam faktörünü belirgin biçimde 2'den büyük yapar.")

# --- Level 2: everyday practice ----------------------------------------------------------------------------------

lesson("ilac-p14", 2, "Reporting with the CMC", "CMC ile raporlama",
       "ILAC P14 says that a calibration certificate must not state an uncertainty smaller than the laboratory's "
       "calibration and measurement capability. The program applies this floor when it reports.",
       "ILAC P14'e göre bir kalibrasyon sertifikası laboratuvarın kalibrasyon ve ölçüm yeteneğinden (CMC) küçük bir "
       "belirsizlik belirtemez. Program raporlarken bu alt sınırı uygular.")
lesson("m3003-c5", 2, "When one rectangular term dominates", "Bir dikdörtgen terim baskın olduğunda",
       "A voltmeter whose resolution dominates. UKAS M3003 gives a special coverage factor for this case, because the "
       "result is closer to rectangular than to normal.",
       "Çözünürlüğü baskın olan bir voltmetre. Sonuç normalden çok dikdörtgene yakın olduğu için UKAS M3003 bu durum "
       "için özel bir kapsam faktörü verir.")
lesson("m3003-l3", 2, "A multimeter specification across a range", "Bir aralık boyunca multimetre spesifikasyonu",
       "A 6½-digit multimeter on its 1 V range: some contributions are relative to the reading (ppm), others are "
       "absolute (µV). The examples show the uncertainty at several points of the range and the coverage factor when "
       "the specification dominates.",
       "1 V aralığında 6½ basamaklı bir multimetre: katkıların bir kısmı okumaya göre bağıldır (ppm), bir kısmı "
       "mutlaktır (µV). Örnekler aralığın birkaç noktasındaki belirsizliği ve spesifikasyon baskın olduğundaki kapsam "
       "faktörünü gösterir.")
lesson("quam-a2", 2, "Standardising a sodium hydroxide solution", "Bir sodyum hidroksit çözeltisinin ayarlanması",
       "Titration against a weighed primary standard. The example uses sub-models (the mass, the volume of the "
       "burette) and shows that repeatability can be put into one combined term.",
       "Tartılmış bir birincil standarda karşı titrasyon. Örnek alt modeller kullanır (kütle, büret hacmi) ve "
       "tekrarlanabilirliğin tek bir birleşik terime konabileceğini gösterir.")
lesson("quam-a3", 2, "Titration of hydrochloric acid", "Hidroklorik asit titrasyonu",
       "The concentration of HCl from a titration with standardised NaOH. The uncertainty of the NaOH from the "
       "previous example is carried over: results of one evaluation become inputs of the next.",
       "Ayarlı NaOH ile titrasyondan HCl derişimi. Önceki örnekteki NaOH belirsizliği buraya taşınır: bir "
       "değerlendirmenin sonuçları bir sonrakinin girdileri olur.")
lesson("quam-a4", 2, "Pesticides in bread", "Ekmekte pestisitler",
       "Two ways to the same answer: a budget built from the steps of the method, and a top-down estimate from "
       "validation data (precision and recovery). Compare which components each approach captures.",
       "Aynı cevaba iki yol: yöntemin adımlarından kurulan bir bütçe ve validasyon verilerinden (kesinlik ve geri "
       "kazanım) yukarıdan aşağı bir tahmin. Her yaklaşımın hangi bileşenleri yakaladığını karşılaştırın.")
lesson("quam-a5", 2, "Cadmium released from ceramics", "Seramikten salınan kadmiyum",
       "An empirical method: the result is defined by the procedure (time, temperature, acid). Effects of these "
       "conditions are included as uncertainty terms.",
       "Ampirik bir yöntem: sonuç prosedürle tanımlanır (süre, sıcaklık, asit). Bu koşulların etkileri belirsizlik "
       "terimleri olarak eklenir.")
lesson("quam-a6", 2, "Crude fibre in animal feed", "Hayvan yeminde ham lif",
       "Another empirical method, evaluated from the precision of an interlaboratory study: the reproducibility of "
       "the method is the main part of the uncertainty.",
       "Bir başka ampirik yöntem; bir laboratuvarlar arası çalışmanın kesinliğiyle değerlendirilir: yöntemin tekrar "
       "üretilebilirliği belirsizliğin ana kısmıdır.")
lesson("nordtest-8", 2, "Top-down for environmental analyses", "Çevre analizleri için yukarıdan aşağı",
       "BOD in wastewater and PCB in sediment, each evaluated twice: once with a CRM and once with proficiency "
       "tests for the bias. See how the choice of bias data changes the result.",
       "Atık sudaki BOİ ve sedimentteki PCB, her biri iki kez değerlendirilir: sapma için bir kez CRM ile, bir kez "
       "yeterlilik testleriyle. Sapma verisinin seçiminin sonucu nasıl değiştirdiğini görün.")
lesson("mupr", 2, "Uncertainty from in-house precision and recovery", "Laboratuvar içi kesinlik ve geri kazanımdan belirsizlik",
       "The 2026 Eurachem/CITAC guide (MUPR) evaluates uncertainty from data every laboratory has: precision at "
       "several levels and recovery from reference materials or spiked samples. The examples cover pooled and "
       "concentration-dependent precision and different recovery designs.",
       "2026 Eurachem/CITAC kılavuzu (MUPR) belirsizliği her laboratuvarın sahip olduğu verilerden değerlendirir: "
       "birkaç düzeyde kesinlik ve referans malzemelerden ya da katkılanmış numunelerden geri kazanım. Örnekler "
       "birleştirilmiş ve derişime bağlı kesinliği ve farklı geri kazanım tasarımlarını kapsar.")
lesson("tr569-ex", 2, "Control charts in practice", "Uygulamada kontrol grafikleri",
       "The worked examples of the Nordtest handbook: X-charts, R-charts and r%-charts for duplicates, target limits "
       "from a required uncertainty, blanks, CRMs and the yearly review of the limits.",
       "Nordtest el kitabının çözümlü örnekleri: X grafikleri, çift ölçümler için R ve r% grafikleri, gerekli "
       "belirsizlikten hedef sınırlar, körler, CRM'ler ve sınırların yıllık gözden geçirilmesi.")
lesson("jcgm106-8", 2, "Decision limits", "Karar sınırları",
       "When must a speeding ticket be issued, or a food sample be declared non-compliant? A decision limit is "
       "placed beyond the legal limit, so that a false accusation is very unlikely (guarded rejection).",
       "Ne zaman hız cezası kesilmeli ya da bir gıda numunesi uygunsuz ilan edilmeli? Karar sınırı yasal sınırın "
       "ötesine konur; böylece haksız bir suçlama çok olasılık dışı olur (korumalı ret).")
lesson("eurachem-compliance", 2, "Compliance with a specification", "Bir spesifikasyona uygunluk",
       "The Eurachem/CITAC guide on compliance: guarded acceptance and rejection, a t distribution with few degrees "
       "of freedom, and a lognormal distribution for a large relative uncertainty near a limit.",
       "Eurachem/CITAC uygunluk kılavuzu: korumalı kabul ve ret, az serbestlik dereceli bir t dağılımı ve bir sınıra "
       "yakın büyük bir bağıl belirsizlik için lognormal dağılım.")
lesson("tur-guard-band", 2, "Guard bands from the test uncertainty ratio", "Test belirsizlik oranından koruma bantları",
       "In calibration, acceptance limits are often derived from the ratio of the tolerance to the measurement "
       "uncertainty (TUR). Three common guard-band methods are compared together with the risks they lead to.",
       "Kalibrasyonda kabul sınırları çoğu zaman toleransın ölçüm belirsizliğine oranından (TUR) türetilir. Üç yaygın "
       "koruma bandı yöntemi yol açtıkları risklerle birlikte karşılaştırılır.")
lesson("iso13528-e1", 2, "Censored results (\"less than\")", "Sansürlü sonuçlar (\"küçüktür\")",
       "Some laboratories report \"< 0.015\" instead of a number. Three ways of handling them are compared: delete "
       "them, ignore the sign, or use half the limit.",
       "Bazı laboratuvarlar bir sayı yerine \"< 0,015\" bildirir. Bunları işlemenin üç yolu karşılaştırılır: silmek, "
       "işareti yok saymak ya da sınırın yarısını kullanmak.")
lesson("iso13528-e2", 2, "Homogeneity and stability", "Homojenlik ve kararlılık",
       "Before a proficiency test, the provider must show that all units of the material are the same and stay the "
       "same. An analysis of variance on replicate measurements answers this.",
       "Bir yeterlilik testinden önce sağlayıcı, malzemenin tüm birimlerinin aynı olduğunu ve öyle kaldığını "
       "göstermelidir. Tekrarlı ölçümler üzerinde bir varyans analizi bunu cevaplar.")
lesson("iso13528-e5", 2, "An assigned value from a CRM", "Bir CRM'den atanan değer",
       "The assigned value comes from a comparison with a certified reference material, and its uncertainty is "
       "part of the evaluation.",
       "Atanan değer sertifikalı bir referans malzemeyle karşılaştırmadan gelir ve onun belirsizliği de "
       "değerlendirmenin parçasıdır.")
lesson("possolo-meija-2022-storage-tank", 2, "Volume of a storage tank", "Bir depolama tankının hacmi",
       "A product of powers (π r² h). The relative uncertainties combine simply, and the Monte Carlo method confirms "
       "the GUM result.",
       "Kuvvetlerin çarpımı (π r² h). Bağıl belirsizlikler basitçe birleşir ve Monte Carlo yöntemi GUM sonucunu "
       "doğrular.")
lesson("possolo-meija-2022-wheatstone", 2, "A Wheatstone bridge", "Bir Wheatstone köprüsü",
       "A model that is not a simple product: the sensitivity coefficients must be derived properly. The program "
       "does this symbolically.",
       "Basit bir çarpım olmayan bir model: duyarlılık katsayılarının doğru türetilmesi gerekir. Program bunu "
       "sembolik olarak yapar.")
lesson("iupac2024-4b", 2, "Isotopic abundances", "İzotop bollukları",
       "The abundances of the silicon isotopes from two measured isotope ratios. The results are correlated because "
       "they must add up to 1.",
       "İki ölçülen izotop oranından silisyum izotoplarının bollukları. Sonuçlar korelasyonludur, çünkü toplamları "
       "1 olmalıdır.")
lesson("iupac2024-4h", 2, "A purity described by a beta distribution", "Beta dağılımıyla anlatılan bir saflık",
       "The cadmium standard again, now with the purity as a beta distribution that cannot exceed 100 %. Monte "
       "Carlo handles such a distribution naturally.",
       "Yine kadmiyum standardı; bu kez saflık %100'ü aşamayan bir beta dağılımıyla. Monte Carlo böyle bir dağılımı "
       "doğal olarak işler.")
lesson("iupac2024-5c", 2, "An asymmetric interval below 1", "1'in altında asimetrik bir aralık",
       "The purity of benzoic acid is close to 1 and cannot exceed it, so its interval is not symmetric. A beta "
       "distribution describes it.",
       "Benzoik asidin saflığı 1'e yakındır ve onu aşamaz; bu yüzden aralığı simetrik değildir. Onu bir beta "
       "dağılımı anlatır.")
lesson("gum-h1", 2, "End-gauge calibration (GUM H.1)", "Uç mastar kalibrasyonu (GUM H.1)",
       "The famous example of the GUM itself. It uses degrees of freedom for Type B terms and the Welch–Satterthwaite "
       "formula to choose the coverage factor.",
       "GUM'un kendisinin ünlü örneği. B tipi terimler için serbestlik derecelerini ve kapsam faktörünü seçmek için "
       "Welch–Satterthwaite formülünü kullanır.")
lesson("gum-h2", 2, "Correlated inputs (GUM H.2)", "Korelasyonlu girdiler (GUM H.2)",
       "Voltage, current and phase are measured simultaneously, so their means are correlated. Resistance, reactance "
       "and impedance are calculated with and without the correlation terms.",
       "Gerilim, akım ve faz eşzamanlı ölçülür, bu yüzden ortalamaları korelasyonludur. Direnç, reaktans ve empedans "
       "korelasyon terimleriyle hesaplanır.")
lesson("num", 2, "Examples of the NIST Uncertainty Machine", "NIST Uncertainty Machine örnekleri",
       "Examples from the manual of the NIST Uncertainty Machine: a ratio with a wide triangular divisor, an end "
       "gauge, a resistance and the Stefan–Boltzmann constant. They compare GUM and Monte Carlo results.",
       "NIST Uncertainty Machine kılavuzundan örnekler: geniş üçgen bölenli bir oran, bir uç mastar, bir direnç ve "
       "Stefan–Boltzmann sabiti. GUM ve Monte Carlo sonuçlarını karşılaştırırlar.")
lesson("ea402-s5", 2, "Calibrating a thermocouple", "Bir ısıl çiftin kalibrasyonu",
       "A thermocouple is compared with a reference thermocouple in a furnace. Two budgets: the temperature of the "
       "furnace and the emf of the unknown thermocouple.",
       "Bir ısıl çift bir fırında bir referans ısıl çiftle karşılaştırılır. İki bütçe: fırının sıcaklığı ve "
       "bilinmeyen ısıl çiftin emk'si.")
lesson("ea402-s6", 2, "A power sensor at 18 GHz", "18 GHz'de bir güç sensörü",
       "A microwave power sensor: mismatch between sensor and source is a U-shaped (arcsine) term.",
       "Bir mikrodalga güç sensörü: sensör ile kaynak arasındaki uyumsuzluk U biçimli (arksinüs) bir terimdir.")
lesson("ea402-s7", 2, "A step attenuator", "Bir adımlı zayıflatıcı",
       "A 30 dB attenuator. Uncertainties in dB, mismatch terms and the repeatability of connections.",
       "Bir 30 dB zayıflatıcı. dB cinsinden belirsizlikler, uyumsuzluk terimleri ve bağlantıların "
       "tekrarlanabilirliği.")
lesson("ea402-s13", 2, "A ring gauge", "Bir halka mastar",
       "The diameter of a 90 mm ring gauge with its temperature correction, in two budgets.",
       "Bir 90 mm halka mastarın çapı ve sıcaklık düzeltmesi, iki bütçede.")
lesson("m3003-k2", 2, "A power sensor (UKAS M3003)", "Bir güç sensörü (UKAS M3003)",
       "The British version of the power-sensor budget, with its mismatch term.",
       "Güç sensörü bütçesinin İngiliz sürümü, uyumsuzluk terimiyle.")
lesson("m3003-k3", 2, "An attenuator (UKAS M3003)", "Bir zayıflatıcı (UKAS M3003)",
       "A 30 dB attenuator at 10 GHz, with two ways of calculating the mismatch term.",
       "10 GHz'de bir 30 dB zayıflatıcı; uyumsuzluk terimini hesaplamanın iki yoluyla.")
lesson("m3003-k6", 2, "A gauge block (UKAS M3003)", "Bir mastar blok (UKAS M3003)",
       "A 10 mm gauge block. One variant writes the temperature effects into the model itself, which shows the "
       "difference between a budget table and a measurement model.",
       "Bir 10 mm mastar blok. Bir türev sıcaklık etkilerini modelin kendisine yazar; bu, bir bütçe tablosu ile bir "
       "ölçüm modeli arasındaki farkı gösterir.")
lesson("m3003-k7", 2, "A thermocouple at 1000 °C", "1000 °C'de bir ısıl çift",
       "An N-type thermocouple: furnace temperature and emf, with many small contributions.",
       "N tipi bir ısıl çift: çok sayıda küçük katkıyla fırın sıcaklığı ve emk.")
lesson("m3003-k9", 2, "Flow rate at standard conditions", "Standart koşullarda debi",
       "A volume flow rate converted to standard pressure and temperature: a product and ratio model.",
       "Standart basınç ve sıcaklığa dönüştürülen bir hacimsel debi: çarpım ve oran modeli.")
lesson("m3003-k11", 2, "A PT100 probe", "Bir PT100 sondası",
       "First the probe is calibrated in a dry block; then it measures the temperature of a gas. The uncertainty of "
       "the calibration becomes an input of the measurement.",
       "Önce sonda bir kuru blokta kalibre edilir; sonra bir gazın sıcaklığını ölçer. Kalibrasyonun belirsizliği "
       "ölçümün bir girdisi olur.")
lesson("cg18-h1", 2, "Calibrating a laboratory balance", "Bir laboratuvar terazisinin kalibrasyonu",
       "A 220 g balance according to EURAMET cg-18: the error at several loads, with and without the air buoyancy "
       "correction, and the uncertainty of a weighing result in daily use.",
       "EURAMET cg-18'e göre 220 g'lık bir terazi: hava kaldırma kuvveti düzeltmesiyle ve düzeltmesiz birkaç yükteki "
       "hata ve günlük kullanımda bir tartım sonucunun belirsizliği.")
lesson("cg18-h2", 2, "A multi-interval balance", "Çok aralıklı bir terazi",
       "A 60 kg scale with three weighing intervals. Each interval has its own resolution, which changes the budget "
       "along the load.",
       "Üç tartım aralıklı 60 kg'lık bir terazi. Her aralığın kendi çözünürlüğü vardır; bu da bütçeyi yük boyunca "
       "değiştirir.")
lesson("cg18-h3", 2, "A weighbridge", "Bir kantar",
       "A 30 t weighbridge tested with standard weights and substitution loads, as in real calibrations of large "
       "scales.",
       "Büyük terazilerin gerçek kalibrasyonlarındaki gibi standart ağırlıklar ve ikame yükleriyle denenen 30 t'lik "
       "bir kantar.")
lesson("nist-tn1900-e3", 2, "A falling-ball viscometer", "Düşen bilyeli viskozimetre",
       "Viscosity from the fall time of a ball. A model with ratios of densities and times.",
       "Bir bilyenin düşme süresinden viskozite. Yoğunluk ve süre oranları içeren bir model.")
lesson("nist-tn1900-e4", 2, "Airspeed with a Pitot tube", "Pitot tüpüyle hava hızı",
       "The speed follows from a pressure difference through a square root. This example also contains a published "
       "inconsistency between the equation and the printed result (see the notes).",
       "Hız, bir basınç farkından karekök üzerinden bulunur. Bu örnekte denklem ile basılı sonuç arasında yayımlanmış "
       "bir tutarsızlık da var (notlara bakın).")
lesson("nist-tn1900-e8", 2, "Molecular weight of CO₂", "CO₂'nin molekül ağırlığı",
       "From the atomic weights of carbon and oxygen, which are given as intervals.",
       "Aralık olarak verilen karbon ve oksijen atom ağırlıklarından.")
lesson("nist-tn1900-e9", 2, "A cadmium standard by Monte Carlo", "Monte Carlo ile bir kadmiyum standardı",
       "NIST's version of the Eurachem cadmium example, evaluated with the Monte Carlo method.",
       "Eurachem kadmiyum örneğinin NIST sürümü, Monte Carlo yöntemiyle değerlendirilmiş.")
lesson("nist-tn1900-e11", 2, "U-shaped inputs", "U biçimli girdiler",
       "A microwave step attenuator whose mismatch terms are U-shaped; GUM and Monte Carlo are compared.",
       "Uyumsuzluk terimleri U biçimli olan bir mikrodalga adımlı zayıflatıcı; GUM ve Monte Carlo karşılaştırılır.")
lesson("nist-tn1900-e13", 2, "Normal or t inputs, 95 % or 99 %", "Normal ya da t girdiler, %95 ya da %99",
       "The thermal expansion coefficient evaluated four times: with normal or Student t inputs, and for 95 % or "
       "99 % coverage. See how both choices change the interval.",
       "Isıl genleşme katsayısı dört kez değerlendirilir: normal ya da Student t girdilerle ve %95 ya da %99 kapsam "
       "için. Her iki seçimin aralığı nasıl değiştirdiğini görün.")
lesson("jcgm101-9.3", 2, "Mass calibration: when the first order is not enough", "Kütle kalibrasyonu: birinci mertebe yetmediğinde",
       "The mass calibration of JCGM 101 with the first-order GUM, with second-order terms and with Monte Carlo. A "
       "small nonlinearity makes the first-order uncertainty too small.",
       "JCGM 101'in kütle kalibrasyonu birinci mertebe GUM, ikinci mertebe terimler ve Monte Carlo ile. Küçük bir "
       "doğrusal olmama birinci mertebe belirsizliği fazla küçük yapar.")
lesson("jcgm102-9.2", 2, "Two outputs from the same inputs", "Aynı girdilerden iki çıktı",
       "Two results calculated from shared inputs are correlated. JCGM 102 gives their covariance matrix and a "
       "coverage region (an ellipse) instead of two separate intervals.",
       "Ortak girdilerden hesaplanan iki sonuç korelasyonludur. JCGM 102 iki ayrı aralık yerine kovaryans matrislerini "
       "ve bir kapsam bölgesini (bir elips) verir.")
lesson("gum5-e1", 2, "pH by interpolation", "İnterpolasyonla pH",
       "The pH of a solution from two buffer standards, at 25 °C and at another temperature. The variants show what "
       "happens when the certified values of the standards are correlated.",
       "Bir çözeltinin pH'ı, 25 °C'de ve başka bir sıcaklıkta iki tampon standarttan. Türevler, standartların "
       "sertifikalı değerleri korelasyonlu olduğunda ne olduğunu gösterir.")
lesson("gum5-e5", 2, "A greenhouse gas inventory", "Bir sera gazı envanteri",
       "Emissions from agricultural machinery. When the same activity data are used for several gases, the results "
       "are correlated and the total uncertainty changes.",
       "Tarım makinelerinden emisyonlar. Aynı faaliyet verisi birkaç gaz için kullanıldığında sonuçlar korelasyonlu "
       "olur ve toplam belirsizlik değişir.")
lesson("gum5-e15", 2, "Breaking force from destructive tests", "Tahribatlı testlerden kopma kuvveti",
       "Six wire ropes are pulled until they break. Only six data: a t distribution and a careful look at the Monte "
       "Carlo scale.",
       "Altı tel halat kopana kadar çekilir. Yalnızca altı veri: bir t dağılımı ve Monte Carlo ölçeğine dikkatli bir "
       "bakış.")
lesson("emue-e2.2", 2, "Vacuum gauge: what to do with a known correction", "Vakum ölçer: bilinen bir düzeltmeyle ne yapmalı",
       "A correction factor can be applied, ignored, or put into the uncertainty. The three scenarios and the effect "
       "of a common temperature error on a pressure difference are compared.",
       "Bir düzeltme faktörü uygulanabilir, yok sayılabilir ya da belirsizliğe katılabilir. Üç senaryo ve ortak bir "
       "sıcaklık hatasının bir basınç farkına etkisi karşılaştırılır.")
lesson("emue-e3.4", 2, "Lead in soil from validation data", "Validasyon verilerinden topraktaki kurşun",
       "An ISO 21748 budget built from the validation of an ICP-OES method: precision, trueness and further terms.",
       "Bir ICP-OES yönteminin validasyonundan kurulan bir ISO 21748 bütçesi: kesinlik, gerçeklik ve diğer terimler.")
lesson("emue-e3.8", 2, "A gas mixture by permeation", "Geçirgenlikle bir gaz karışımı",
       "The amount fraction of ammonia from a permeation tube: mass loss rate, flow and molar masses.",
       "Bir geçirgenlik tüpünden amonyağın miktar kesri: kütle kayıp hızı, akış ve molar kütleler.")
lesson("emue-e6.5", 2, "Anti-doping: ephedrine in urine", "Doping kontrolü: idrarda efedrin",
       "A top-down uncertainty from intermediate precision and bias, and the decision limit used by WADA to declare "
       "an adverse finding.",
       "Ara kesinlik ve sapmadan yukarıdan aşağı bir belirsizlik ve WADA'nın olumsuz bulgu ilan etmek için kullandığı "
       "karar sınırı.")
lesson("iso21748", 2, "Uncertainty from reproducibility data", "Tekrar üretilebilirlik verisinden belirsizlik",
       "When a standard method has been tested in an interlaboratory study, its reproducibility standard deviation "
       "is a good first estimate of the uncertainty (ISO 21748). The examples add further terms where needed.",
       "Standart bir yöntem bir laboratuvarlar arası çalışmada denenmişse, tekrar üretilebilirlik standart sapması "
       "belirsizliğin iyi bir ilk tahminidir (ISO 21748). Örnekler gerektiğinde başka terimler ekler.")
lesson("iso5725-2", 2, "An interlaboratory precision study", "Laboratuvarlar arası kesinlik çalışması",
       "Eight or more laboratories, several levels, replicate results: repeatability and reproducibility of a "
       "method, after Cochran's and Grubbs's tests. Each example is evaluated classically and with REML.",
       "Sekiz ya da daha fazla laboratuvar, birkaç düzey, tekrarlı sonuçlar: Cochran ve Grubbs testlerinden sonra bir "
       "yöntemin tekrarlanabilirliği ve tekrar üretilebilirliği. Her örnek klasik yöntemle ve REML ile "
       "değerlendirilir.")
lesson("iso6143-b1", 2, "A straight-line calibration", "Doğrusal bir kalibrasyon",
       "Nitrogen monoxide by UV absorption calibrated with a straight line through reference gas mixtures, with "
       "uncertainties on both axes.",
       "Referans gaz karışımlarından geçen bir doğruyla kalibre edilen UV soğurmalı azot monoksit; iki eksende de "
       "belirsizlik.")
lesson("ufs-a1", 2, "Sampling: nitrate in lettuce", "Numune alma: marulda nitrat",
       "The duplicate method: two samples from each of eight lots, each analysed twice. The sampling part of the "
       "uncertainty is larger than the analytical part.",
       "Çift numune yöntemi: sekiz partinin her birinden iki numune, her biri iki kez analiz edilir. Belirsizliğin "
       "numune alma payı analiz payından büyüktür.")
lesson("ufs-a2", 2, "Sampling: lead in soil", "Numune alma: toprakta kurşun",
       "A large relative uncertainty is better expressed as a factor (multiply or divide) on a log scale.",
       "Büyük bir bağıl belirsizlik logaritmik ölçekte bir çarpan (çarp ya da böl) olarak daha iyi ifade edilir.")
lesson("ufs-a3", 2, "Sampling: iron in groundwater", "Numune alma: yeraltı suyunda demir",
       "The same data evaluated with the analysis of variance and with the simpler range method.",
       "Aynı veriler varyans analiziyle ve daha basit aralık yöntemiyle değerlendirilir.")
lesson("ufs-a4", 2, "Sampling: vitamin A in porridge", "Numune alma: lapada A vitamini",
       "Small test portions make the sampling variance large; with 4 g portions the estimated analytical variance "
       "becomes negative, which shows the limits of the method. A control chart of duplicates is included.",
       "Küçük test porsiyonları numune alma varyansını büyütür; 4 g'lık porsiyonlarda kestirilen analitik varyans "
       "negatif çıkar, bu da yöntemin sınırlarını gösterir. Çift numunelerin bir kontrol grafiği de vardır.")
lesson("ufs-a6", 2, "Sampling uncertainty by modelling", "Modelleme ile numune alma belirsizliği",
       "Instead of duplicates, the sampling steps are modelled one by one, as in a GUM budget.",
       "Çift numuneler yerine numune alma adımları bir GUM bütçesindeki gibi tek tek modellenir.")

# --- Level 3: advanced topics ------------------------------------------------------------------------------------

lesson("jcgm106-9", 3, "Global risks of false decisions", "Yanlış kararların genel riskleri",
       "With knowledge of the production process, the probability that an accepted item is actually bad "
       "(consumer's risk) can be calculated for the whole production, and an acceptance limit chosen for a "
       "required risk.",
       "Üretim süreci bilindiğinde, kabul edilen bir ürünün gerçekte kötü olma olasılığı (müşteri riski) bütün üretim "
       "için hesaplanabilir ve gerekli bir risk için bir kabul sınırı seçilebilir.")
lesson("emue-e2.1", 3, "Several components must conform", "Birkaç bileşen birlikte uymalı",
       "A medicine with four correlated active components: the risks of a false decision are calculated jointly by "
       "Monte Carlo.",
       "Dört korelasyonlu etken maddeli bir ilaç: yanlış karar riskleri Monte Carlo ile birlikte hesaplanır.")
lesson("gum5-e10", 3, "Global and specific risks with a prior", "Önsel bilgiyle genel ve özgül riskler",
       "Particulate matter in air: prior knowledge of typical concentrations (a lognormal distribution) is combined "
       "with the measurement to calculate the risks of a decision.",
       "Havada partikül madde: tipik derişimler hakkındaki önsel bilgi (lognormal dağılım) bir kararın risklerini "
       "hesaplamak için ölçümle birleştirilir.")
lesson("jcgm101-9.4", 3, "A strongly nonlinear model", "Belirgin biçimde doğrusal olmayan bir model",
       "The comparison loss in microwave power: near zero the model is so nonlinear that the first-order GUM fails "
       "completely, while Monte Carlo gives the right answer. The variants add correlation and higher-order terms.",
       "Mikrodalga gücünde karşılaştırma kaybı: sıfıra yakın model o kadar doğrusal değildir ki birinci mertebe GUM "
       "tamamen başarısız olur, Monte Carlo ise doğru cevabı verir. Türevler korelasyon ve yüksek mertebe terimler "
       "ekler.")
lesson("jcgm101-9.5", 3, "Gauge block: exact and approximate models", "Mastar blok: tam ve yaklaşık modeller",
       "The full nonlinear gauge-block model of JCGM 101 compared with its linear approximation, by GUM and Monte "
       "Carlo.",
       "JCGM 101'in tam doğrusal olmayan mastar blok modeli, doğrusal yaklaşımıyla GUM ve Monte Carlo üzerinden "
       "karşılaştırılır.")
lesson("jcgm102-9.3", 3, "Cartesian to polar coordinates", "Kartezyen koordinattan kutupsal koordinata",
       "A magnitude and a phase from two coordinates. Near the origin the transformation is strongly nonlinear and "
       "the coverage region is not an ellipse.",
       "İki koordinattan bir büyüklük ve bir faz. Merkeze yakın dönüşüm belirgin biçimde doğrusal değildir ve kapsam "
       "bölgesi bir elips değildir.")
lesson("jcgm102-9.4", 3, "Resistance and reactance from simultaneous data", "Eşzamanlı verilerden direnç ve reaktans",
       "Three outputs from correlated indication values; the alternative uses a multivariate t distribution for the "
       "few data.",
       "Korelasyonlu gösterge değerlerinden üç çıktı; alternatif sürüm az veri için çok değişkenli bir t dağılımı "
       "kullanır.")
lesson("jcgm102-9.5", 3, "Implicit models: a resistance thermometer", "Kapalı modeller: bir direnç termometresi",
       "The temperature cannot be written as a formula of the resistance, so the equation is solved numerically. "
       "With ten temperatures from one thermometer, the results are strongly correlated.",
       "Sıcaklık direncin bir formülü olarak yazılamaz, bu yüzden denklem sayısal olarak çözülür. Bir termometreden on "
       "sıcaklıkla sonuçlar güçlü biçimde korelasyonludur.")
lesson("gum5-e2", 3, "Peak areas from the same injections", "Aynı enjeksiyonlardan pik alanları",
       "Benzo[a]pyrene by GC-MS with an internal standard. Two areas measured in the same injections are correlated; "
       "the variant treats them as an ensemble when counting degrees of freedom.",
       "İç standartlı GC-MS ile benzo[a]piren. Aynı enjeksiyonlarda ölçülen iki alan korelasyonludur; türev, "
       "serbestlik derecesini sayarken onları bir topluluk olarak ele alır.")
lesson("gum5-e4", 3, "A gravimetric gas mixture", "Gravimetrik bir gaz karışımı",
       "Eight amount fractions of a calibration gas mixture prepared by weighing: many inputs, intermediate "
       "quantities and correlated outputs.",
       "Tartımla hazırlanmış bir kalibrasyon gaz karışımının sekiz miktar kesri: çok sayıda girdi, ara büyüklükler ve "
       "korelasyonlu çıktılar.")
lesson("gum5-e9", 3, "Calibrating a torque sensor", "Bir tork sensörünün kalibrasyonu",
       "A straight line through the origin fitted by ordinary and weighted least squares, with the GUM uncertainty "
       "of the slope.",
       "Sıradan ve ağırlıklı en küçük karelerle uydurulan, orijinden geçen bir doğru ve eğimin GUM belirsizliği.")
lesson("gum5-e12", 3, "Between-bottle homogeneity with Bayes", "Bayes ile şişeler arası homojenlik",
       "When the between-bottle variance is small, the classical estimate can be zero or negative; a hierarchical "
       "Bayesian model gives a sensible value.",
       "Şişeler arası varyans küçük olduğunda klasik tahmin sıfır ya da negatif çıkabilir; hiyerarşik bir Bayesçi "
       "model anlamlı bir değer verir.")
lesson("possolo-bodnar-2018", 3, "Homogeneity of a soil reference material", "Bir toprak referans malzemesinin homojenliği",
       "Twenty bottles measured in triplicate, evaluated by the analysis of variance and by a hierarchical Bayesian "
       "model.",
       "Üçer kez ölçülen yirmi şişe; varyans analiziyle ve hiyerarşik bir Bayesçi modelle değerlendirilir.")
lesson("possolo-meija-2022-atomic-weight", 3, "Atomic weight from isotope ratios", "İzotop oranlarından atom ağırlığı",
       "The atomic weight and isotopic composition of lead from three measured isotope ratios: correlated outputs "
       "from a nonlinear model.",
       "Üç ölçülen izotop oranından kurşunun atom ağırlığı ve izotop bileşimi: doğrusal olmayan bir modelden "
       "korelasyonlu çıktılar.")
lesson("possolo-meija-2022-hubble", 3, "A consensus value with heavy tails", "Kalın kuyruklu bir uzlaşı değeri",
       "Twenty-two inconsistent estimates of the Hubble–Lemaître constant combined with a Bayesian model whose "
       "laboratory effects follow a Laplace distribution, which is less influenced by outliers.",
       "Hubble–Lemaître sabitinin yirmi iki tutarsız tahmini, laboratuvar etkileri Laplace dağılımına uyan ve "
       "aykırı değerlerden daha az etkilenen bir Bayesçi modelle birleştirilir.")
lesson("iupac2024-3a", 3, "An implicit chemical model", "Kapalı bir kimyasal model",
       "The Mohr titration with its endpoint chemistry: the solubility equilibrium gives an implicit equation that "
       "the program solves.",
       "Dönüm noktası kimyasıyla Mohr titrasyonu: çözünürlük dengesi programın çözdüğü kapalı bir denklem verir.")
lesson("iupac2024-3e", 3, "Three inconsistent determinations", "Üç tutarsız belirleme",
       "The molar mass of iridium from three determinations that disagree. Laboratory effects are modelled with a "
       "hierarchical Bayesian model.",
       "Birbiriyle uyuşmayan üç belirlemeden iridyumun molar kütlesi. Laboratuvar etkileri hiyerarşik bir Bayesçi "
       "modelle modellenir.")
lesson("iupac2024-4f", 3, "A key comparison", "Bir anahtar karşılaştırma",
       "CCQM-K145, nickel in bovine liver: a consensus value with dark uncertainty by maximum likelihood. "
       "Participants are coded.",
       "CCQM-K145, sığır karaciğerinde nikel: en çok olabilirlikle karanlık belirsizlikli bir uzlaşı değeri. "
       "Katılımcılar kodlanmıştır.")
lesson("koepke2017", 3, "Consensus values of discrepant data", "Tutarsız verilerin uzlaşı değerleri",
       "Half-lives and the gravitational constant: measurements that disagree far more than their uncertainties. "
       "DerSimonian–Laird, Linear Pool and hierarchical Bayes give different answers; the choice is part of the "
       "evaluation.",
       "Yarı ömürler ve kütle çekim sabiti: belirsizliklerinden çok daha fazla birbirinden ayrılan ölçümler. "
       "DerSimonian–Laird, Linear Pool ve hiyerarşik Bayes farklı cevaplar verir; seçim değerlendirmenin bir "
       "parçasıdır.")
lesson("merkatas2019", 3, "Shades of dark uncertainty", "Karanlık belirsizliğin tonları",
       "Sixteen measurements of the Newtonian constant of gravitation evaluated with several procedures; the "
       "consensus value and its uncertainty depend strongly on the procedure.",
       "Newton kütle çekim sabitinin on altı ölçümü birkaç yöntemle değerlendirilir; uzlaşı değeri ve belirsizliği "
       "yönteme güçlü biçimde bağlıdır.")
lesson("nicob", 3, "Degrees of equivalence in a comparison", "Bir karşılaştırmada eşdeğerlik dereceleri",
       "The examples of the NIST Consensus Builder: a consensus value from laboratories with degrees of freedom, and "
       "leave-one-out and bilateral degrees of equivalence of 19 laboratories.",
       "NIST Consensus Builder örnekleri: serbestlik dereceli laboratuvarlardan bir uzlaşı değeri ve 19 laboratuvarın "
       "birini dışarıda bırakma ve ikili eşdeğerlik dereceleri.")
lesson("iso5725-3", 3, "Intermediate precision and nested designs", "Ara kesinlik ve iç içe tasarımlar",
       "Precision when time and operator change, and the staggered-nested design that separates several factors "
       "with few measurements.",
       "Zaman ve operatör değiştiğinde kesinlik ve birkaç faktörü az ölçümle ayıran kademeli iç içe tasarım.")
lesson("iso6143", 3, "Calibration functions with uncertainty on both axes", "İki eksende belirsizlikli kalibrasyon fonksiyonları",
       "Gas analysis with straight-line, quadratic, power and exponential functions, and reference mixtures whose "
       "contents are correlated. The program tests which function fits.",
       "Doğrusal, ikinci dereceden, kuvvet ve üstel fonksiyonlarla gaz analizi ve içerikleri korelasyonlu referans "
       "karışımlar. Program hangi fonksiyonun uyduğunu sınar.")
lesson("emue-e3.3", 3, "An analyser calibrated by dynamic dilution", "Dinamik seyreltmeyle kalibre edilen bir analizör",
       "The reference mixtures are made from the same parent gas, so their contents are correlated. Ignoring the "
       "covariances changes the result.",
       "Referans karışımlar aynı ana gazdan yapılır, bu yüzden içerikleri korelasyonludur. Kovaryansları yok saymak "
       "sonucu değiştirir.")
lesson("cg18-h4", 3, "An error approximation function", "Bir hata yaklaşım fonksiyonu",
       "A straight line fitted through the errors of a balance at many loads, with a χ² test and a residual test "
       "deciding how large an extra term s_m must be.",
       "Bir terazinin birçok yükteki hatalarından geçirilen bir doğru; ek terim s_m'nin ne kadar büyük olması "
       "gerektiğine bir χ² testi ve bir artık testi karar verir.")
lesson("palma2019", 3, "Matrix effects in a top-down model", "Yukarıdan aşağı bir modelde matris etkileri",
       "Seven elements in marine sediments: recoveries from reference materials of different matrices, and a model "
       "with two concentration intervals.",
       "Deniz sedimentlerinde yedi element: farklı matrisli referans malzemelerden geri kazanımlar ve iki derişim "
       "aralıklı bir model.")
lesson("bettencourt2026", 3, "Arithmetic or weighted mean recovery", "Aritmetik ya da ağırlıklı ortalama geri kazanım",
       "Fluoranthene in river sediments: the mean recovery calculated in two ways, as in the MUPR guide and its "
       "spreadsheet.",
       "Nehir sedimentlerinde floranten: ortalama geri kazanım, MUPR kılavuzu ve hesap tablosundaki gibi iki yolla "
       "hesaplanır.")
lesson("bipm-isrd", 3, "Purity by quantitative NMR (BIPM)", "Nicel NMR ile saflık (BIPM)",
       "The reference data of the BIPM for internal standards in qNMR. Complete purity budgets; one result lies close "
       "to the physical limit of 1 and is reported with an asymmetric interval.",
       "BIPM'in qNMR iç standartları için referans verileri. Tam saflık bütçeleri; bir sonuç fiziksel sınır olan 1'e "
       "yakındır ve asimetrik bir aralıkla raporlanır.")
lesson("quam-a7", 3, "Isotope dilution (IDMS)", "İzotop seyreltme (IDMS)",
       "Lead in water by double isotope dilution: a long model with isotope ratios, masses and a calibration "
       "standard. A reference method with a small uncertainty.",
       "Çift izotop seyreltmeyle suda kurşun: izotop oranları, kütleler ve bir kalibrasyon standardı içeren uzun bir "
       "model. Küçük belirsizlikli bir referans yöntem.")
lesson("ufs-a5", 3, "Gy's sampling theory", "Gy'nin numune alma teorisi",
       "The fundamental sampling error of an enzyme in chicken feed, estimated from particle size and properties.",
       "Tavuk yemindeki bir enzimin temel numune alma hatası, parçacık boyutu ve özelliklerinden kestirilir.")

if __name__ == "__main__":
    out = Path("app/assets/guide/lessons.json")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(L, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print("wrote", out, len(L), "lessons")
