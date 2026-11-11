*Editöre mektubun Türkçe çevirisi (yazarın okuması için; dergiye İngilizcesi gönderilir: `cover-letter.md`).*

Sayın Editör,

**"KAPSAM: open-source software for measurement uncertainty, verified against the worked examples of the guidance it
implements"** başlıklı makalemi *Measurement* dergisinde tam uzunlukta araştırma makalesi olarak yayımlanmak üzere
değerlendirmenizi rica ederim.

Akredite deney ve kalibrasyon laboratuvarları belirsizlik değerlendirmelerini GUM ailesinin ve sektör kılavuzlarının
(Eurachem/CITAC, Nordtest, EA, UKAS, EURAMET, ISO) çözümlü örnekleri üzerinde öğrenir ve denetler. Bu örnekleri
sistematik olarak yeniden hesapladım: 25'i kılavuz belge ve standart olan 37 kaynağın çözümlü örneklerinden
oluşturulan 286 proje ve yazılımın, kodunun her değişikliğinde belgelenmiş toleranslar içinde yeniden ürettiği 1865
referans değeri. Çalışma ölçüm bilimine iki yolla katkı sağlar:

(a) **Çözümlü örneklerin kendisi.** Yeniden hesaplama 100 bulgu ortaya çıkardı. Bunların on biri, laboratuvarların her
gün kullandığı belgelerde basılı bir nihai sonucu (bir kestirimi veya belirsizliğini) değiştiren doğrulanmış
tutarsızlıklardır; beşi sonucu genişletilmiş belirsizliğin onda birinden fazla değiştirir; kodu görmemiş bir metrolog
bunları, makalede belirtildiği gibi bir yapay zekâ asistanının yardımıyla, belgelerden yeniden üretmiştir (Bölüm 8). Yayımcı kuruluşların örneklerinin girdilerini makinece okunabilir biçimde
yayımlamalarını ve yayından önce yeniden hesaplamalarını öneriyor, kütüphaneyi bu amaçla sunuyorum.

(b) **Ölçüm yazılımının doğrulanması.** Makale belirsizlik yazılımının ISO/IEC 17025 (7.11.2, 7.11.6) için nasıl
doğrulanabileceğini gösterir: belgelenmiş toleranslı ve kökeni belirtilmiş referans değerleri, toleransların başka
rastgele tohumlarla sınanması, bir tutarsızlık sınıflandırması, bağımsız bir program (Suncal, 166 proje), 189 projenin
sıradan elektronik tablo formülleriyle yeniden hesaplanması ve bir laboratuvarın kendi projelerine uygulayabileceği bir
validasyon raporu.

Yazılım bu çalışmanın aracıdır, tek konusu değildir: sonuçlar belirsizlik değerlendirmelerinin ve dayandıkları
belgelerin doğruluğuyla ilgilidir; bu nedenle makaleyi bir yazılım dergisine değil *Measurement*'a gönderiyorum.

Makale, derginin kapsam için koyduğu koşulları karşılar:

- *Ölçüm bilimine katkı:* başka bir alanın ölçüm sonuçları değil, ölçüm belirsizliğinin değerlendirilmesi ve
  belirsizlik yazılımının doğrulanması üzerine yöntemler ve kanıtlar.
- *Metrolojik terimler:* terimler VIM ve GUM'daki tanımlarıyla kullanılır; bunlara terimlerin tanıtıldığı yerde atıf
  yapılır (Bölüm 2.1); değerlendirme yöntemleri GUM ailesinin ve atıf yapılan standartlarındır.
- *Alanın durumu:* Bölüm 1 mevcut yazılımları ve bunların doğrulanmasına ilişkin önceki çalışmaları (PTB vaka
  çalışması, Greif ve Schrepf'in GUM test durumları, TraCIM, Mathmet taraması) gözden geçirir ve çalışmanın bunları
  nasıl genişlettiğini belirtir: ölçekte (yalnızca GUM değil, 25 kılavuz belge ve standart), kodun her değişikliğinde
  çalışan bir regresyon testi olarak ve yeniden hesabın örneklerde bulduklarının sınıflandırılmasıyla.

Araştırma makalesi şekil, tablo ve kaynaklarıyla 27 sayfadır (sınır 30 sayfa). Dergi çift kör hakemlik uyguladığından makale,
şekiller ve ek malzeme yazarın adını, kurumunu veya bağlantısını içermez; bunlar ayrı bir dosya olan başlık
sayfasındadır. Bu çalışmada tam anonimlik mümkün değildir: programın adı Türkçedir, Türkçe bir arayüzü vardır ve
arşivlenmiş sürümü açıktır; adıyla yapılacak bir arama yazarı bulur.

Çalışma araştırma yöntemi olarak makine öğrenmesi veya sinir ağları kullanmaz; tüm değerlendirmeler GUM ailesinin ve
atıf yapılan standartların deterministik ve Monte Carlo yöntemleridir. Kod yazmak ve metnin taslağını hazırlamak için
bir yapay zekâ kodlama asistanı kullanılmıştır; c1 bulgularını yeniden hesaplayan meslektaş da bir asistan kullanmıştır. Bu, Bölüm 3'te ve üretken yapay zekâ beyanında belirtilmiştir; Tablo 8
hangi denetimlerin programdan ve asistandan bağımsız olduğunu gösterir. Makaledeki her sayı açık depodan ve arşivlenmiş
v0.1.0 sürümünden (https://doi.org/10.5281/zenodo.24118312) yeniden üretilebilir.

Bulgular yayımlanmış kılavuz belgelerle ilgilidir. Hakemli bir kayda atıf yapabilmeleri için makale kabul edildikten
sonra yayımcı kuruluşlara teknik not olarak iletilecektir.

Makale daha önce yayımlanmamıştır ve başka bir yerde değerlendirmede değildir. Yazarın çıkar çatışması yoktur.

Önerilen hakemler (e-posta adresleri gönderim sistemine girilmiştir). Dördü makalede atıf yapılan çalışmaların
yazarlarıdır; üçü bulgu içeren belgelerle ilişkili kurumlarda çalışır (EMUE derlemesi VSL ve NPL'de hazırlanmıştır;
INRiM JCGM çalışma gruplarında yer alır); editörün değerlendirebilmesi için bunu belirtiyorum.

- Blair D. Hall, Measurement Standards Laboratory of New Zealand, Lower Hutt, Yeni Zelanda: GUM Tree Calculator'ın
  geliştiricisi; ölçüm verisinin dijital temsili.
- Francesca R. Pennecchi, Istituto Nazionale di Ricerca Metrologica (INRiM), Torino, İtalya: Mathmet'te belirsizlik
  yazılımı ve eğitimi taraması; doğrusal olmayan modellerde belirsizlik yayılımı; uygunluk kararlarının riskleri.
- Peter M. Harris, National Physical Laboratory (NPL), Teddington, Birleşik Krallık: metroloji yazılımının
  validasyonu; Mathmet ölçüm belirsizliği eğitimi.
- Katsuhiro Shirono, National Metrology Institute of Japan (NMIJ), AIST, Tsukuba, Japonya: belirsizlik bilgisiyle
  yeterlilik testleri; uygunluk değerlendirmesinde kabul aralıkları.
- Gertjan Kok, VSL, Delft, Hollanda: ölçüm modelleri ve belirsizlik değerlendirme yazılımı; Monte Carlo yöntemleri.

Saygılarımla,

İlker Ün
TÜBİTAK National Metrology Institute (TÜBİTAK UME), 41470 Gebze, Kocaeli, Türkiye
ORCID 0000-0001-5089-4602
