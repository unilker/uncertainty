# 12. Sık yapılan hatalar ve sorular

<a id="mistakes"></a>

## En sık yapılan on hata

1. **U'yu standart belirsizlik olarak girmek.** Bir sertifika "U = 0,6 mm (k = 2)" diyor. 0,6'yı standart belirsizlik olarak girmek o katkıyı iki katına çıkarır. U ve k'yi kendi alanlarına girin ya da k'ye kendiniz bölün.
2. **Bir ortalama için tek okumaların standart sapmasını kullanmak.** n okumanın ortalamasının standart belirsizliği s/√n'dir. Tersine, raporlanan sonuç tek bir okumaysa s'yi (s/√n'yi değil) kullanın.
3. **Bir etkiyi iki kez saymak.** Bir kontrol grafiğinin dağılımı tekrarlanabilirliği, farklı operatörlerin ve yeniden kalibrasyonların etkisini zaten içerir. Onları ayrı terimler olarak yeniden eklemek iki kez saymak demektir.
4. **Düzeltmesi sıfır diye bir etkiyi unutmak.** 0 °C'lik bir sıcaklık düzeltmesinin de bir belirsizliği vardır. Her etki, en iyi değeri sıfır olsa bile modelde kalır.
5. **Birimleri karıştırmak.** mL ve L, mg ve g, % ve kesir; bir sıcaklık *farkında* °C ve K (aynı), mutlak bir sıcaklıkta (aynı değil). Önce sonucun tahminini denetleyin: yanlışsa neden çoğu zaman bir birimdir.
6. **Korelasyonu yok saymak.** Aynı standarttan, aynı teraziden ya da aynı stok çözeltiden gelen iki büyüklük korelasyonludur. Ya korelasyonu girin ya da ortak büyüklüğü modele yazın.
7. **Bir tolerans için normal dağılım seçmek.** Başka bilgi olmayan "± 0,1 mL" bir tolerans dikdörtgendir (u = 0,058 mL); u = 0,1 mL olan bir normal değildir.
8. **Erken yuvarlamak.** Yuvarlanmış ara değerler sonucun son basamağını değiştirebilir. Değerleri tüm basamaklarıyla girin ve programın yalnızca raporlanan sonucu yuvarlamasına izin verin.
9. **Çok az serbestlik derecesiyle k = 2 kullanmak.** Bütçeye yalnızca 3 ya da 4 okuma hâkimse k = 2, %95'ten belirgin biçimde azını kapsar. **Ayarlar**'da **Kapsam olasılığı (k, t dağılımından)** seçin.
10. **Fazla basamak raporlamak.** "U = 0,123 456 mg", "U = 0,12 mg"dan daha kesin değildir; yalnızca öyle görünür.

<a id="questions"></a>

## Sorular

**Monte Carlo yöntemine ihtiyacım var mı?**
Her zaman değil. Açık tutun: program GUM sonucunun *doğrulandığını* söylüyorsa GUM sonucu yeterlidir. Söylemiyorsa [7. bölüme](07-monte-carlo.md#disagree) bakın.

**Hangi kapsam olasılığını kullanmalıyım?**
Yaklaşık %95 (k = 2) kalibrasyon ve deney için uluslararası alışkanlıktır. Bazı alanlar %99 (k ≈ 2,6) kullanır ya da belirli bir olasılık ister; program her birini kabul eder.

**B tipi girdimin serbestlik derecesi yok. Bu bir sorun mu?**
Hayır. Bir B tipi değer normalde güvenilir kabul edilir (ν = ∞). Değerin kendisinden şüpheniz varsa, örneğin "u yaklaşık %25 içinde biliniyor", girdinin **serbestlik derecesi** alanına ν ≈ 8 yazabilirsiniz (GUM G.4.2: ν ≈ ½ (Δu/u)⁻²).

**İki laboratuvar aynı yöntem için farklı belirsizlikler buldu. Hangisi haklı?**
Belki ikisi de. Belirsizlik her laboratuvarın cihazlarına, standartlarına, ortamına ve okuma sayısına bağlıdır. ζ ya da E_n puanlarıyla bir yeterlilik testi ([9. bölüm](09-laboratory-data.md#proficiency)) belirsizliklerin gerçekçi olup olmadığını gösterir.

**Belirsizlik okumalarımın dağılımından küçük olabilir mi?**
Bir *ortalamanın* belirsizliği tek okumaların dağılımından küçük olabilir (s/√n). *Tek* bir okumanın belirsizliği tek okumaların dağılımından küçük olamaz.

**Başka bir değerlendirmenin sonucu olan bir büyüklüğü nasıl girerim?**
Raporlanan değeri ve standart belirsizliğini (ya da U ve k'yi) normal bir girdi olarak kullanın. Aynı önceki değerlendirmenin birkaç sonucu modele giriyorsa bunlar korelasyonludur: bunu girin ya da iki değerlendirmeyi tek bir modelde birleştirin.

**"Karanlık belirsizlik" nedir?**
Laboratuvarlar arasındaki uyuşmazlığın, beyan ettikleri belirsizliklerin açıklamadığı kısmı. Uzlaşı yöntemleri ([10. bölüm](10-special-evaluations.md#consensus)) onu tahmin eder.

**Programı akreditasyonum için kullanabilir miyim?**
Program, kaynak gösterilen kılavuz ve standartların yöntemlerini uygular ve yayımlanmış 286 örnekle doğrulanmıştır. Laboratuvarınızın yine de kendi yöntemleri için kullanımını doğrulaması gerekir (ISO/IEC 17025 7.11): beklenen değerleriyle kendi tipik projelerinizi saklayın ve her güncellemeden sonra yeniden çalıştırın ([11. bölüm](11-reporting.md#validation)).

**Projelerim nerede saklanıyor?**
Onları nereye kaydederseniz: **Kaydet** bir `.json` dosyası yazar. Mobil cihazlarda projeler uygulamanın belgeler klasöründe tutulur ve paylaşılabilir.

**Bir kütüphane örneğinde tuhaf bir sonuç buldum.**
Önce **Genel** sekmesindeki **Notlar**'ı okuyun: yayımlanmış birkaç örnek yuvarlama ya da baskı hataları içerir ve bunlar orada belgelenmiştir. Açıklanmamışsa lütfen programın yazarına bildirin.

<a id="next"></a>

## Buradan nereye?

- [13. bölüm](13-learning-with-examples.md): kütüphanenin bütün örnekleri, düzeylerine göre ve her grubun ne öğrettiğiyle.
- Kaynak kodun `docs/guide` klasöründeki başvuru kılavuzu: her yöntem, standartların maddeleriyle birlikte tüm ayrıntısıyla.
