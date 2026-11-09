# 11. Sonucu raporlamak

## Eksiksiz ifade {#statement}

Bir kalibrasyon sertifikasında ya da deney raporunda bir sonuç şunları içerir:

1. değer ve birimi,
2. aynı birimde (ya da bağıl) genişletilmiş belirsizlik U,
3. kapsam faktörü k ve kapsam olasılığı,
4. gerekirse belirsizliğin nasıl değerlendirildiği (GUM, Monte Carlo, yukarıdan aşağı).

Örneğin:

```
m = (100,0021 ± 0,0012) g
Beyan edilen genişletilmiş belirsizlik, standart belirsizliğin k = 2 kapsam
faktörüyle çarpılmasıyla elde edilmiştir ve normal dağılım için yaklaşık %95
kapsam olasılığına karşılık gelir.
```

Program bu cümleyi sizin için Türkçe ya da İngilizce olarak **Raporlanan sonuç** altında yazar.

## Kaç basamak? {#digits}

- U'yu **iki anlamlı basamakla** (bazen bir) verin: 0,001 234 5 g değil, 0,0012 g. Daha fazla basamak, belirsizliğin kendisinde olmayan bir kesinlik varmış gibi gösterir.
- Değeri U ile **aynı sayıda ondalıkla** verin: 100,002 ya da 100,002 13 değil, 100,0021 g.
- U'yu asla fazla aşağı yuvarlamayın: varsayılan kural (EA-4/02 M:2022, 6.3) olağan yuvarlama yapar, ama bu U'yu %5'ten fazla küçültüyorsa yukarı yuvarlar. **Ayarlar → Yuvarlama**'da başka kurallar da var: her zaman yukarı, yarım yukarı, yarım çifte.
- Hesap boyunca tüm basamakları koruyun; yalnızca raporlanan sonucu yuvarlayın. Program bunu yapar.

## Bağıl belirsizlik ve yüzdeler {#relative}

Kimyasal analizlerde bağıl genişletilmiş belirsizlik yaygındır: "c = 12,4 mg/L ± %8". Bir yüzdenin bağıl mı (değerin %'si) yoksa mutlak mı (örneğin belirsizliği %0,1 olan % cinsinden bir kütle kesri) olduğunu belirtmeye dikkat edin.

## Bir sınıra ya da fiziksel sınıra yakın sonuç {#bounds}

- Bir kütle kesri 0,9998 g/g ve U = 0,0005 g/g ise aralık 1 g/g'ı aşar; bu imkânsızdır. Program o zaman asimetrik bir aralık verir, örneğin 0,9998 +0,0002/−0,0005 g/g, ve Monte Carlo sonucunu fiziksel sınırla kısıtlayabilir.
- Uygunlukla ilgili bir karar raporlanıyorsa karar kuralı da belirtilmelidir ([10. bölüm](10-special-evaluations#conformity)).

## Bir kalibrasyon laboratuvarının CMC'si {#cmc}

Akredite bir kalibrasyon laboratuvarı CMC'sinden küçük bir belirsizlik raporlayamaz (ILAC P14). Hesap daha küçük bir değer verirse CMC raporlanır. CMC'nizi projede bir genişletilmiş belirsizlik olarak verin (**JSON** sekmesinde: `"reporting": { "cmc": { "expanded": 80 } }`); program bu kuralı uygular ve raporda belirtir. [10 kg'lık ağırlıkla ILAC P14](example:ilac-p14-cmc-floor-weight-10kg) örneği bunu gösterir.

## Raporu dışa aktarmak {#export}

Bir projenin ⋮ menüsü şunları sunar:

- **PDF**: tüm tabloları içeren, yazı tipleri gömülü, dosyalamaya ya da eklemeye hazır bir A4 rapor;
- **HTML**: aynı rapor grafiklerle, bir tarayıcı için;
- **Markdown**: düz metin, başka belgelere eklemesi kolay.

Rapor sağ üstte seçili dilde (EN/TR) yazılır. Modeli, girdileri ve kaynaklarını, bütçeyi, GUM ve Monte Carlo sonuçlarını, raporlanan sonucu ve sonunda program sürümünü ve projenin bir dijital parmak izini (SHA-256) içerir. Parmak iziyle herkes daha sonra bir raporun belirli bir proje dosyasına ait olduğunu denetleyebilir.

## ISO/IEC 17025 için yazılım validasyonu {#validation}

ISO/IEC 17025 (7.11.2), hesaplar için kullanılan yazılımın doğrulanmasını ister. Program üç yolla yardım eder:

- yeniden hesaplanıp yayımlanmış değerlerle karşılaştırılan 286 örnekten oluşan kütüphanesi, yöntemlerin validasyonudur;
- komut satırı aracı bir **yazılım validasyon raporu** yazar (`kapsam check library --lab projelerim --report validasyon.pdf`); bu rapor her denetimi, dosyaların parmak izlerini, program sürümünü ve derleyiciyi listeler ve laboratuvarınızın kendi projelerini ayrı bir bölüme koyar;
- ölçüm modelleri, yeterlilik testleri, kesinlik deneyleri ve top-down değerlendirmeler için bir **elektronik tablo doğrulama dosyası** yazar (⋮ menüsü → **Excel'de doğrula (.xlsx)**; birçok proje için birden `kapsam check projelerim --workbooks klasor --report validasyon.pdf`). Dosyayı Excel'de veya LibreOffice Calc'ta açın: elektronik tablo her sonucu projenin verilerinden, hücre hücre inceleyebileceğiniz sıradan formüllerle kendisi yeniden hesaplar; **Bilgi** sayfası bütün sonuçların toleransları içinde KAPSAM'ınkilerle uyuşup uyuşmadığını söyler. Hesabı elektronik tablo yaptığı için bu kontrol programa bağlı değildir.

Kendi tipik projelerinizi beklenen değerleriyle saklayın: programın her güncellemesinden sonra onları yeniden çalıştırmak basit ve belgelenmiş bir validasyondur.

> **İpucu:** Bir sertifika için kullandığınız her projeyi kaydedin. `.json` dosyası, rapor ve parmak izi birlikte değerlendirmenin teknik kaydıdır.
