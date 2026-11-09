# 8. Hangi tür projeye ihtiyacım var?

Ana ekranda her değerlendirme türü için bir kart var. Sorunuzu tabloda bulun, kartı açın ve kılavuzun gösterdiği kısmı okuyun. Her satır, kütüphaneden açıp çalıştırabileceğiniz bir örneği de verir.

## Sorunuzu bulun {#table}

| Sorunuz | Kart | Okuyun | Örnek |
|---|---|---|---|
| Sonucum için bir denklem yazabiliyorum ve girdileri biliyorum | **Yeni ölçüm modeli** | [4. bölüm](04-first-calculation) | [10 kg'lık ağırlığın kalibrasyonu](example:ea402-s2-weight-10kg) |
| Bir deney laboratuvarıyım; validasyon ve kalite kontrol verilerim var (kontrol grafikleri, CRM'ler, yeterlilik testleri) | **Yeni top-down değerlendirme** | [yukarıdan aşağı](09-laboratory-data#topdown) | [Suda amonyum](example:nordtest-3-ammonium-pt) |
| Bir yeterlilik testi düzenliyorum ve katılımcıları puanlamalıyım | **Yeni yeterlilik testi turu** | [yeterlilik testi](09-laboratory-data#proficiency) | [Hayvan yeminde cıva](example:iso13528-e4-mercury-feed) |
| Bir analizörü referans karışımlarla kalibre ediyorum ve iki eksenin de belirsizliği var | **Yeni kalibrasyon (iki eksen)** | [kalibrasyon eğrileri](10-special-evaluations#calibration) | [UV ile N₂ içinde NO](example:iso6143-b1-no-uv-linear) |
| Sonucum (ya da ürünüm) bir spesifikasyona ya da yasal sınıra uyuyor mu? | **Yeni uygunluk değerlendirmesi** | [uygunluk](10-special-evaluations#conformity) | [Hız sınırı denetimi](example:jcgm106-8.3-speed-limit) |
| Birkaç laboratuvar aynı numuneleri ölçtü; yöntemin tekrarlanabilirliği ve tekrar üretilebilirliği gerekiyor | **Yeni laboratuvarlar arası kesinlik çalışması** | [kesinlik çalışmaları](09-laboratory-data#precision) | [Kömürde kükürt](example:iso5725-2-c1-sulfur-in-coal) |
| Modelim aynı anda birkaç sonuç veriyor ve bunlar korelasyonlu | **Yeni çok çıktılı model** | [birden çok çıktı](10-special-evaluations#multivariate) | [İki çıktı, Gauss girdiler](example:jcgm102-9.2.2-additive-gaussian) |
| Bir yöntemi kontrol numunesiyle izlemek istiyorum | **Yeni kontrol grafiği** | [kontrol grafikleri](09-laboratory-data#control-chart) | [Suda çinko](example:tr569-table1-zinc) |
| Belirsizliğimin ne kadarı numune almadan geliyor? | **Yeni numune alma belirsizliği** | [numune alma](09-laboratory-data#sampling) | [Marulda nitrat](example:ufs-a1-nitrate-lettuce) |
| Otomatik olmayan bir tartı aletini kalibre ediyorum | **Yeni hata fonksiyonu** | [tartı aletleri](10-special-evaluations#weighing) | [220 g'lık laboratuvar terazisi](example:cg18-h1-a-weighing-result) |
| Birkaç laboratuvarın sonuçlarını tek bir değerde birleştirmeliyim (anahtar karşılaştırma, referans değer) | **Yeni uzlaşı değeri** | [uzlaşı değerleri](10-special-evaluations#consensus) | [¹³⁷Cs yarı ömrü](example:koepke2017-half-life-cs137) |

## Aşağıdan yukarı mı, yukarıdan aşağı mı? {#bottom-up-top-down}

Bir deney laboratuvarındaki en yaygın tereddüt budur.

- Yöntem iyi anlaşılmışsa, bir kalibrasyonsa ya da her adımın katkısını bilmeniz gerekiyorsa (örneğin yöntemi iyileştirmek için) bir **ölçüm modeli** (aşağıdan yukarı) seçin.
- Yöntem karmaşıksa (numune hazırlama, matris etkileri, çok adım) ve iyi uzun dönem verileriniz varsa, yani aylara yayılan bir kontrol grafiği ve sapmayı denetlemenin bir yolu, **yukarıdan aşağı** bir değerlendirme seçin. Yukarıdan aşağı değerlendirmeler, bir modelin unutabileceği etkileri kendiliğinden içerir.

İkisi birleştirilebilir: analitik yöntem için yukarıdan aşağı bir tahmin artı bir alt numune alma adımı için bir model ya da yukarıdan aşağı tahminle denetlenen bir model.

> **İpucu:** Eurachem'in [Ekmekte pestisitler](example:quam-a4-pesticides-topdown) örneği kütüphanede iki yolla da değerlendirilmiştir ([adımların birleşimi olarak](example:quam-a4-pesticides-combination)). İkisini karşılaştırmak çok öğreticidir.

## Bir modelin üstüne uygunluk sorusu {#conformity-on-model}

Bir sonucu bir sınıra göre değerlendirmek için ayrı bir projeye gerek yoktur. Bir ölçüm modelinde **Ayarlar**'ı açın ve **Bir toleransa uygunluğu değerlendir** seçeneğini açın: program sonucu ve hemen ardından spesifikasyona uyma olasılığını hesaplar.

## Var olan bir dosyadan ya da örnekten başlamak {#start}

- **Proje dosyası aç**, daha önce kaydettiğiniz ya da bir meslektaşınızın gönderdiği bir `.json` projesini açar.
- Her kütüphane örneği değiştirilip yeni bir adla kaydedilebilir: benzer bir örneği açın, sayıları kendinizinkilerle değiştirin ve **Kaydet**'e basın. Başlamanın çoğu zaman en hızlı yolu budur.
