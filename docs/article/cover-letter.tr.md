*Editöre mektubun Türkçe çevirisi (yazarın okuması için; dergiye İngilizcesi gönderilir: `cover-letter.md`).*

Sayın Editör,

**"KAPSAM: open-source software for measurement uncertainty, validated against the worked examples of the guidance
it implements"** başlıklı makalemizi *Measurement* dergisinde tam uzunlukta araştırma makalesi olarak yayımlanmak
üzere değerlendirmenizi rica ederiz.

Akredite deney ve kalibrasyon laboratuvarları belirsizlik değerlendirmelerini GUM ailesinin ve sektör
kılavuzlarının (Eurachem/CITAC, Nordtest, EA, UKAS, EURAMET, ISO) çözümlü örnekleri üzerinde öğrenir ve doğrular. Bu
örnekleri sistematik olarak yeniden hesapladık: 38 belgenin çözümlü örneklerinden oluşturulan 286 proje ve yazılımın,
kodunun her değişikliğinde belgelenmiş toleranslar içinde yeniden ürettiği 1865 basılı değer. Çalışma ölçüm bilimine
üç yolla katkı sağlar:

1. **Çözümlü örneklerin kendisi.** Yeniden hesaplama 100 bulgu ortaya çıkardı; bunların 13'ü, laboratuvarların her
   gün kullandığı belgelerde basılı bir nihai sonucu (bir kestirimi, belirsizliğini ya da belirtilen bir kararı)
   değiştiren doğrulanmış tutarsızlıklardır. Yayımcı kuruluşların örneklerinin girdilerini makinece okunabilir
   biçimde yayımlamalarını ve yayından önce yeniden hesaplamalarını öneriyor, kütüphaneyi bu amaçla sunuyoruz.
2. **Sonucu değiştiren seçimler.** Bir anahtar karşılaştırmada uzlaşı yönteminin seçimi, eşdeğerlik derecelerini
   referans değer kadar değiştirmektedir; fiziksel bir sınıra yakın sonuçlar ve test belirsizlik oranından koruma
   bantları da aynı nicel biçimde ele alınmıştır.
3. **Ölçüm yazılımının validasyonu.** Belirsizlik yazılımının ISO/IEC 17025 (7.11.2) için nasıl doğrulanabileceğini
   anlatıyoruz: basılı basamaklardan türetilen toleranslarla referans değerler, denetimden önce sabitlenmiş bir
   uyuşmazlık sınıflandırması, bağımsız bir program (Suncal, 171 proje), 189 projenin sıradan elektronik tablo
   formülleriyle yeniden hesaplanması ve bir laboratuvarın kendi projelerine uygulayabileceği bir validasyon raporu
   ile elektronik tablo dosyaları.

Yazılım bu çalışmanın aracıdır, tek konusu değildir: sonuçlar, belirsizlik değerlendirmelerinin ve dayandıkları
belgelerin doğruluğuyla ilgilidir; bu nedenle makaleyi bir yazılım dergisine değil *Measurement*'a gönderiyoruz.

Makale, derginin kapsamı için koyduğu koşulları karşılar:

1. **Ölçüm bilimine katkı.** Makale, ölçüm belirsizliğinin değerlendirilmesine ve belirsizlik yazılımının
   validasyonuna ilişkin yöntemler ve kanıtlar sunar (basılı basamaklardan türetilen toleranslarla referans değerler,
   denetimden önce sabitlenmiş bir uyuşmazlık sınıflandırması, bağımsız yeniden hesaplamalar); başka bir alanın
   ölçüm sonuçlarını değil.
2. **Metroloji terimleri.** Terimler VIM'de ve GUM'da tanımlandığı biçimde kullanılır; ikisine de terimlerin
   tanıtıldığı yerde atıf yapılır (Bölüm 2.1). Değerlendirme yöntemleri GUM ailesinin ve atıf yapılan standartların
   yöntemleridir.
3. **Mevcut durum.** Bölüm 1 kılavuz belgeleri ve mevcut yazılımları gözden geçirir (NIST Uncertainty Machine,
   Suncal, GUM Tree Calculator, METAS UncLib, NIST Consensus Builder, Mathmet'in 35 programlık taraması ve PTB'nin
   GUM yazılımı validasyonu vaka çalışması) ve çalışmanın bunu nasıl ilerlettiğini belirtir: laboratuvarların
   ihtiyaç duyduğu değerlendirme türleri için tek bir doğrulanmış program; yalnızca GUM'un değil 38 belgenin çözümlü
   örnekleri üzerinde validasyon ve bunun örneklerin kendisi için sonuçları (Bölüm 6 ve 7).

Araştırma makalesi şekil, tablo ve kaynaklarıyla 26 sayfadır. Dergi çift kör hakemlik uyguladığından makale, şekiller
ve ek malzeme yazarı tanıtan hiçbir bilgi içermez; yazarın bilgileri ile depo ve arşivlenmiş sürüm bağlantıları
ayrı bir dosya olan kapak sayfasındadır.

Çalışma, araştırma yöntemi olarak makine öğrenmesi ya da sinir ağları kullanmaz; bütün değerlendirmeler GUM
ailesinin ve atıf yapılan standartların deterministik ve Monte Carlo yöntemleridir. Kod yazmak ve metni taslak
olarak hazırlamak için bir yapay zekâ kodlama asistanı kullanılmıştır. Bu, Bölüm 3.4'te ve üretken yapay zekâ
beyanında belirtilmiştir; Tablo 6 hangi denetimlerin programdan ve asistandan bağımsız olduğunu gösterir. Makaledeki
her sayı herkese açık depodan ve arşivlenmiş v0.1.0 sürümünden (https://doi.org/10.5281/zenodo.24118312) yeniden üretilebilir; ek malzeme
kütüphaneyi, bulguları, validasyon matrisini, Suncal karşılaştırmasını, bir yazılım validasyon raporunu, yazılımın
kılavuzunu ve elektronik tablo yeniden hesaplamasını içerir.

Bulgular yayımlanmış kılavuz belgelerle ilgilidir. Hakemli bir kayda atıf yapabilmeleri için makale kabul edildikten
sonra yayımcı kuruluşlara teknik not olarak iletilecektir.

Makale daha önce yayımlanmamıştır ve başka bir yerde değerlendirmede değildir. Yazarın çıkar çatışması yoktur.
[Dergi isterse önerilen hakemler: ad, kurum, e-posta, gerekçe — yazar tarafından eklenecek.]

Saygılarımla,

İlker Ün
TÜBİTAK Ulusal Metroloji Enstitüsü (UME), Gebze, Kocaeli, Türkiye
ORCID 0000-0001-5089-4602
