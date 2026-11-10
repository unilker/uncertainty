# 1. Buradan başlayın

Hoş geldiniz. Bu kılavuz ölçüm belirsizliğini en baştan, sade bir dille anlatır ve her türlü değerlendirmeyi bu programla, **KAPSAM** ile nasıl yapacağınızı gösterir. (Programın adı, [3. bölümde](03-key-words.md#expanded) tanışacağınız *kapsam faktörü* *k* ve *kapsam aralığından* gelir.) Başlamak için istatistik bilmeniz gerekmez. Belirsizlik bütçeleriyle zaten çalışıyorsanız ilginizi çeken bölümlere doğrudan geçebilirsiniz.

> **İpucu:** Her bölüm her ekrandan açılabilir: ana ekrandaki kitap simgesine ya da bir projedeki **?** simgesine basın; ikincisi o proje türünü anlatan bölümü açar.

<a id="paths"></a>

## Yolunuzu seçin

| Siz… | Okuyun | Süre |
|---|---|---|
| **Belirsizliğe yenisiniz** | 2'den 7'ye kadar bölümleri sırayla, sonra değerlendirme türünüzü bulmak için 8. bölümü | yaklaşık 1 saat |
| **Bir laboratuvarda çalışıyorsunuz** ve temelleri biliyorsunuz | [3](03-key-words.md), [5](05-where-numbers-come-from.md), [6](06-reading-results.md), sonra [8](08-choosing-a-project-type.md) ve yönteminizle ilgili kısım | 30 dakika |
| **Deneyimlisiniz**, programı denetlemek istiyorsunuz | [8](08-choosing-a-project-type.md), [10](10-special-evaluations.md), [11](11-reporting.md) ve [düzeylere göre örnekler](13-learning-with-examples.md) | istediğiniz kadar |

<a id="program"></a>

## Program nasıl düzenlenmiş?

**Ana ekranın** iki bölümü var:

- **Yeni bir değerlendirme başlatın**: her değerlendirme türü için bir kart. Bir kart, küçük bir örnekle doldurulmuş yeni bir proje açar; böylece hiçbir zaman boş bir sayfadan başlamazsınız. Doğru kartı seçmek için [8. bölüme](08-choosing-a-project-type.md) bakın.
- **Örnek kütüphanesi**: kılavuzlardan, standartlardan ve makalelerden alınmış 286 çözümlü örnek. Her biri yeniden hesaplanmış ve denetlenmiştir. Onları düzeye göre süzebilirsiniz: *Başlangıç*, *Orta* ve *İleri*.

**Bir proje** sekmeli bir pencerede açılır:

| Sekme | Orada ne yaparsınız |
|---|---|
| **Genel** | Başlık, tanım; kütüphane örnekleri için *Bu örnek ne öğretir?* ve *Adım adım, sade bir dille* |
| **Model** ya da **Veriler** | Projenin kalbi: ölçüm modeli ve girdileri ya da bir laboratuvar yönteminin verileri |
| **Ayarlar** | Kapsam faktörü, Monte Carlo deneme sayısı, sonucun yuvarlanması |
| **JSON** | Proje dosyasının metni, uzmanlar için |
| **Sonuçlar** | **Hesapla**'ya bastıktan sonra programın hesapladığı her şey |

Kırmızı **Hesapla** düğmesi değerlendirmeyi çalıştırır. **Kaydet** projeyi bir `.json` dosyası olarak saklar; menü (⋮) raporu Türkçe ya da İngilizce olarak PDF, HTML veya Markdown biçiminde dışa aktarır.

<a id="examples"></a>

## Örneklerle öğrenin

Kütüphanedeki her örnek aynı zamanda bir derstir:

1. Kütüphaneden bir örnek açın (*Başlangıç* düzeyinden başlayın).
2. **Genel** sekmesinde **Bu örnek ne öğretir?** ve **Adım adım** bölümlerini okuyun.
3. **Hesapla**'ya basın ve **Sonuçlar**'ı açın. Sayıları açıklamanın söyledikleriyle karşılaştırın.
4. Bir şeyi değiştirin (bir belirsizliği, bir okumayı, kapsam faktörünü) ve yeniden hesaplayın. Hiçbir şey bozulmaz: kütüphanedeki asıl örnek değişmez; değişiklikleriniz yalnızca **Kaydet**'e basarsanız saklanır.

[13. bölüm](13-learning-with-examples.md) bütün örnekleri düzeylerine göre, her grubun ne öğrettiğiyle birlikte listeler.

> **Deneyin:** Kütüphanenin en basit örneği olan [Bir tozun tartımla kütlesi](examples.md#nist-tn1900-e1-weighing) örneğini açın ve yukarıdaki dört adımı izleyin.

<a id="symbols"></a>

## Bu kılavuzdaki sözcükler ve simgeler

- **y** istediğiniz sonuç, **x₁, x₂, …** girdi büyüklükleridir.
- **u** bir standart belirsizlik, **u_c** birleşik standart belirsizlik, **U** genişletilmiş belirsizlik, **k** kapsam faktörüdür.
- Sayılar programdaki gibi yazılır: Türkçede ondalık ayırıcı virgüldür, İngilizcede noktadır.
- **Kalın** yazılan sözcükler programın düğmeleri, sekmeleri ya da alanlarıdır; *eğik* yazılan sözcükler hemen yakında açıklanan yeni terimlerdir.

[3. bölüm](03-key-words.md) her terimi sade bir dille ve her biri için küçük bir örnekle açıklar.
