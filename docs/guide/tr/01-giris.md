# 1. Giriş

*Belirsizlik Hesaplamaları* (İngilizce: *Uncertainty Calculations*) ölçüm belirsizliğini üç yolla değerlendirir:

| Yaklaşım | Dayanak belgeler | Tipik kullanım |
|---|---|---|
| **GUM belirsizlik yaklaşımı** (aşağıdan yukarı, belirsizliğin yayılımı yasası) | JCGM 100:2008 (GUM); EA-4/02 M:2022; EURACHEM/CITAC QUAM:2012 | kalibrasyon, referans çözelti hazırlama, açık bir modeli olan her ölçüm |
| **Monte Carlo yöntemi** (dağılımların yayılımı) | JCGM 101:2008 (GUM Ek 1) | doğrusal olmayan modeller, baskın normal olmayan girdiler, asimetrik çıktı dağılımları; GUM sonucunun doğrulanması |
| **Top-down** (validasyon ve kalite kontrol verileri) | Nordtest TR 537 4. baskı (2017); Eurachem/CITAC MUPR (2026); QUAM:2012 | deney laboratuvarlarında rutin kimyasal analiz |

Program öncelikle analitik kimya, ardından kalibrasyon ve akredite laboratuvarlar için yazılmıştır; ancak
hesaplama motoru belirli bir alana özgü değildir: örnek kütüphanesi kimya, kütle, uzunluk, sıcaklık,
elektrik/RF ve akış metrolojisinden örnekler içerir.

## Rehberin yapısı

1. Giriş (bu bölüm)
2. [Başlarken](02-baslarken.md): kurulum, komut satırı aracı, ilk hesaplama
3. [GUM belirsizlik yaklaşımı](03-gum.md): modeller, standart belirsizlikler, duyarlılık katsayıları,
   birleştirme, serbestlik dereceleri, kapsam faktörleri
4. [Monte Carlo yöntemi](04-monte-carlo.md): dağılımların yayılımı, kapsam aralıkları, GUM sonucunun
   doğrulanması
5. [Top-down değerlendirme](05-top-down.md): laboratuvar içi tekrarüretilebilirlik, sapma, geri kazanım
6. [Raporlama](06-raporlama.md): yuvarlama, sonucun ifadesi, raporun içeriği
7. [Proje dosyası başvurusu](07-proje-dosyasi.md): JSON proje biçiminin tüm alanları
8. [Sözlük (İngilizce–Türkçe)](08-sozluk.md)
9. [Yeterlilik testi (ISO 13528)](09-yeterlilik-testi.md): atanan değerler, robust istatistikler, skorlar
10. [Her iki değişkende belirsizlikli kalibrasyon (ISO 6143)](10-kalibrasyon.md)
11. [Uygunluk değerlendirmesi (JCGM 106)](11-uygunluk.md): karar kuralları, koruma bantları, riskler
12. [Laboratuvarlar arası kesinlik deneyleri (ISO 5725-2)](12-kesinlik-deneyleri.md): aykırı değer testleri,
    tekrarlanabilirlik ve tekrarüretilebilirlik
13. [Birden çok çıktı büyüklüğü (JCGM 102)](13-cok-ciktili.md): kovaryans matrisleri, kapsam bölgeleri
14. [Kontrol grafikleri (Nordtest TR 569)](14-kontrol-grafikleri.md): sınırlar, günlük kurallar, uzun dönem
    değerlendirmesi
15. [Numune alma belirsizliği (Eurachem UfS)](15-numune-alma.md): çift numune yöntemi, aralık yöntemi, belirsizlik
    faktörü, Gy numune alma kuramı
16. [Tartı aletlerinin kalibrasyonu (EURAMET cg-18)](16-tarti-aletleri.md): gösterge hatası, ikame yükleri, hata
    yaklaşım fonksiyonu
17. [Çözümlü örnekler](examples.md): kütüphanedeki 221 örneğin tamamı, sonuçları program tarafından yeniden
    hesaplanmış olarak

3.–5. bölümler yalnızca programı değil yöntemlerin kendisini de anlatır: dayanak belgelerle birlikte
okunabilecek kısa bir belirsizlik değerlendirme rehberi olarak tasarlanmıştır.

## Bilimsel doğrulama

Her hesaplama yöntemi yayımlanmış çözümlü örneklerle doğrulanmıştır. Örnekler basılı sonuçlarla eşleştirilmez,
*girdilerinden* (belgeler veriyorsa ham verilerden) yeniden hesaplanır; her karşılaştırmanın belgelenmiş bir
toleransı vardır: belgenin basılı yuvarlaması ya da ölçülen Monte Carlo saçılımının yaklaşık beş katı.
Kütüphanedeki 221 örnek test takımının parçasıdır: programdaki bir değişiklik herhangi bir sonucu toleransının
ötesinde değiştirirse derleme başarısız olur.

Örneklerin girdilerinden yeniden hesaplanması, yayımlanmış örneklerin kendi içinde tutarsız olduğu yerleri de
ortaya çıkarır (yuvarlanmış ara değerler, aktarım hataları, verilen girdilerden çıkmayan değerler). Bu durumlar
her örneğin *notlarında* ve projenin araştırma günlüğünde (`docs/research/research-log.md`) kayıtlıdır;
hiçbiri yöntemi değiştirmez, yalnızca birkaçı basılı bir sonucu değiştirir.

## Lisans ve atıf

Program, ek atıf koşullarıyla (GPL §7) özgür yazılımdır (GPL-3.0-or-later). **Programın sonuçlarını kullanan
her yayın, rapor veya yazılım programa atıf yapmalıdır.** Programın ürettiği her rapor atıf metniyle biter;
`ADDITIONAL-TERMS.md` ve `CITATION.cff` dosyalarına bakınız.

> Ün, İ. *Uncertainty Calculations (Belirsizlik Hesaplamaları): measurement uncertainty evaluation software.*
> TÜBİTAK UME. https://github.com/unilker/uncertainty
