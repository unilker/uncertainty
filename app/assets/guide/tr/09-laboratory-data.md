# 9. Laboratuvar verisinden değerlendirmeler

Bu bölüm, bir laboratuvarın zaten topladığı verilerle çalışan proje türlerini açıklar: validasyon verileri, kontrol numuneleri, çift ölçümler, yeterlilik testleri ve laboratuvarlar arası çalışmalar. Her kısım türün ne zaman kullanılacağını, neye ihtiyacınız olduğunu, nasıl girileceğini ve ne elde edeceğinizi söyler.

## Yukarıdan aşağı (top-down) değerlendirme {#topdown}

**Ne zaman.** Rutin bir deney yöntemi (kimyasal, mikrobiyolojik, çevresel) uyguluyorsunuz ve belirsizliğini her adımın bir modelinden değil, laboratuvarın kendi verisinden bulmak istiyorsunuz. Bu, Nordtest TR 537, Eurachem/CITAC MUPR kılavuzu (2026) ve ISO 21748'i izler.

**Fikir.** İki sayı neredeyse her şeyi kapsar:

```
u = √( u(Rw)² + u(sapma)² )
```

- **u(Rw)**, *laboratuvar içi tekrar üretilebilirlik*: aynı numunenin sonuçlarının uzun bir süre boyunca (farklı günler, analistler, kalibrasyonlar) ne kadar dağıldığı. Genellikle bir kontrol grafiğinden gelir.
- **u(sapma)**: yöntemin gerçek değeri ne kadar iyi tutturduğu; sertifikalı referans malzemelerden (CRM), yeterlilik testlerinden ya da geri kazanım deneylerinden, bu referans değerlerin belirsizliği de dahil.

**Neye ihtiyacınız var.**

- Birkaç biçimden birinde kesinlik verisi: bir kontrol grafiğinden standart sapma, çift ölçüm çiftleri, farklı günlerde tekrar grupları ya da derişime bağlı bir kesinlik.
- Sapma verisi: sertifikalı değerleri ve belirsizlikleriyle bir ya da daha fazla CRM üzerindeki sonuçlar ya da atanan değerleriyle yeterlilik testi sonuçları ya da katkılanmış numuneler üzerinde geri kazanım testleri.
- İsteğe bağlı olarak belirsizliği istediğiniz derişim düzeyleri.

**Nasıl girilir.** **Veriler** sekmesinde:

1. **Ölçülen büyüklük**, **Birim** ve her satıra bir tane olmak üzere **Ölçülen değerler (düzeyler)** girin.
2. **Kesinlik** altında **+**'ya basın ve veri türünü seçin. Kontrol grafiği verisi için standart sapmayı (mutlak ya da % olarak) girin; çift ölçümler ya da gruplar için sonuçları yapıştırın.
3. **Sapma** ya da **Geri kazanım** altında referans malzemelerinizi, yeterlilik testlerinizi ya da katkılanmış numunelerinizi ekleyin.
4. Kılavuz istiyorsa başka bileşenler ekleyin (örneğin alt numune alma).

**Ne elde edersiniz.** Her düzey için: u(Rw), u(sapma), birleşik u, genişletilmiş U (k = 2) ve bağıl değerler. Bir geri kazanım %100'den anlamlı biçimde farklıysa program bir düzeltme gerekip gerekmediğini söyler.

> **Örnek:** [Bir CRM'den sapma](example:nordtest-6.1-one-crm), [yeterlilik testlerinden](example:nordtest-6.2-proficiency-tests), [amonyum için bütün bütçe](example:nordtest-3-ammonium-pt) ve [içme suyunda nitrat](example:mupr-b1-nitrate) gibi MUPR örnekleri.

> **Dikkat:** Kontrol numunesi, numune hazırlama dahil yöntemin *tamamından* geçmelidir; yoksa u(Rw) fazla küçük çıkar. Yalnızca birkaç kez ölçülmüş bir CRM belirsiz bir u(sapma) verir; birkaç yıla yayılan yeterlilik testleri çoğu zaman daha temsil edicidir.

## Yeterlilik testi {#proficiency}

**Ne zaman.** Bir yeterlilik testi düzenliyorsunuz (ISO 13528) ya da bir turun puanlarını denetlemek istiyorsunuz.

**Fikir.** Her katılımcı aynı malzemeyi ölçer. Bir **atanan değer** x_pt belirlenir (bir referans laboratuvardan, bir CRM'den ya da dayanıklı istatistiklerle katılımcı sonuçlarından) ve bir **yeterlilik değerlendirmesi için standart sapma** σ_pt, bir sonucun ne kadar yakın olması gerektiğini söyler. Sonra:

| Puan | Formül | Anlamı |
|---|---|---|
| z | (x − x_pt)/σ_pt | |z| ≤ 2 başarılı, 2 < |z| < 3 şüpheli, |z| ≥ 3 başarısız |
| z′ | x_pt'nin belirsizliği σ_pt'ye eklenmiş z | x_pt belirsiz olduğunda kullanılır |
| ζ | (x − x_pt)/√(u(x)² + u(x_pt)²) | sonuç katılımcının kendi belirsizliği içinde uyuşuyor mu? |
| E_n | (x − x_pt)/√(U(x)² + U(x_pt)²) | aynısı genişletilmiş belirsizliklerle; |E_n| ≤ 1 başarılıdır |

**Neye ihtiyacınız var.** Katılımcıların sonuçları (kod, değer ve isteğe bağlı U ve k), atanan değerin yöntemi ve σ_pt. Bir homojenlik çalışması için her birimin tekrarlı sonuçları.

**Nasıl girilir.** **Veriler** sekmesinde sonuçları yapıştırın (her satıra bir laboratuvar: kod, sonuç, U, k). `<0,015` gibi bir sonuç sansürlü kabul edilir. Atanan değeri (Algoritma A ile dayanıklı ortalama, medyan, ortalama ya da bir referans değer) ve σ_pt'yi seçin.

**Ne elde edersiniz.** Belirsizliğiyle atanan değer, her puan, z puanlarının bir grafiği ve homojenlik için σ_pt ile karşılaştırılan birimler arası standart sapma.

> **Örnek:** [Atrazin sonuçlarının dayanıklı ortalaması](example:iso13528-e3-atrazine-algorithm-a), [yemde cıva için tüm puanlar](example:iso13528-e4-mercury-feed), [homojenlik ve kararlılık](example:iso13528-e2-arsenic-homogeneity).

## Laboratuvarlar arası kesinlik çalışması {#precision}

**Ne zaman.** Bir yöntem birkaç laboratuvarca geçerli kılınıyor (ISO 5725-2) ve onun tekrarlanabilirliği s_r ile tekrar üretilebilirliği s_R gerekiyor; örneğin bir standarda yazmak için.

**Neye ihtiyacınız var.** Her düzey için her laboratuvarın tekrarlı sonuçları (tipik olarak 8 ile 15 laboratuvar, her biri 2 tekrar).

**Nasıl girilir.** Laboratuvarları listeleyin; sonra her düzey için her laboratuvara bir satır, her tekrara bir sütun olan bir tablo yapıştırın. Klasik varyans analizini ya da REML'i (eksik verilerle daha iyi) seçin.

**Ne elde edersiniz.** Cochran testi (dağılımı fazla büyük bir laboratuvar), Grubbs testleri (aykırı laboratuvar ortalamaları), grafik olarak Mandel h ve k istatistikleri; sonra her düzeyde s_r ve s_R ile tekrarlanabilirlik ve tekrar üretilebilirlik sınırları r = 2,8 s_r ve R = 2,8 s_R. Aykırı değerler kendiliğinden çıkarılmaz: siz karar verirsiniz ve karar belgelenir.

> **Örnek:** [Kömürde kükürt](example:iso5725-2-c1-sulfur-in-coal), [eksik verilerle ziftin yumuşama noktası](example:iso5725-2-c2-softening-point-pitch).

## Kontrol grafikleri {#control-chart}

**Ne zaman.** Her seriyle bir kontrol numunesi ölçüyorsunuz ve yöntemin istatistiksel kontrol altında kalıp kalmadığını görmek istiyorsunuz (Nordtest TR 569).

**Fikir.** Sonuçlar zaman sırasıyla çizilir. Bir **merkez çizgisinin** (ortalama ya da bir hedef değer) çevresinde **uyarı sınırları** (± 2 s) ve **eylem sınırları** (± 3 s) vardır. Kurallar ne zaman harekete geçileceğini söyler; örneğin eylem sınırlarının dışında bir nokta ya da art arda üç noktadan ikisinin aynı tarafta uyarı sınırlarının dışında olması. Program Nordtest TR 569'un kurallarını uygular ve birini bozan her noktayı işaretler.

**Neye ihtiyacınız var.** Zaman sırasıyla kontrol değerleri (ya da çift ölçümlerin R grafikleri için tekrarlı sonuçlar) ve sınırlar hakkında bir karar:

- *istatistiksel sınırlar*: verilerin kendi dağılımından;
- *hedef sınırlar*: yöntemin ulaşması gereken kesinlikten (örneğin gerekli belirsizlikten).

**Ne elde edersiniz.** Sınırlarıyla grafik, bir kuralı bozan her nokta ve uzun dönem bir gözden geçirme: dağılım değişti mi (F testi)? ortalama kaydı mı (t testi)? yeni sınırlar belirlenmeli mi?

> **Örnek:** [Suda çinko, 60 değer](example:tr569-table1-zinc), [sınırları belirlemenin üç yolu](example:tr569-ch7-case1-statistical), [çift ölçümler için bir R grafiği](example:tr569-ex3-ammonium-r).

> **İpucu:** Bir kontrol grafiğinin uzun dönem standart sapması tam olarak yukarıdan aşağı bir değerlendirmenin u(Rw)'sidir. İki proje türü birbirini tamamlar.

## Numune almadan kaynaklanan belirsizlik {#sampling}

**Ne zaman.** Sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca bir numune analiz edersiniz. Numunenin nereden ve nasıl alındığından gelen değişim analitik belirsizlikten büyük olabilir (Eurachem/EUROLAB/CITAC/Nordtest/AMC kılavuzu UfS).

**Çift numune yöntemi.** En az sekiz hedeften (parti, tarla) ikişer bağımsız numune alın ve her numuneyi iki kez analiz edin:

```
hedef 1: numune A (analiz 1, 2), numune B (analiz 1, 2)
hedef 2: …
```

Varyans analizi üç kısmı ayırır: hedefler arası değişim (gerçektir, belirsizlik değildir), **numune alma** değişimi ve **analitik** değişim. Ölçüm belirsizliği

```
u = √( u(numune alma)² + u(analiz)² )
```

**Nasıl girilir.** Her hedef için dört sonuçla bir satır yapıştırın. Büyük bağıl belirsizlikler için logaritmik dönüşümlü değerlendirmeyi seçin: sonuç o zaman bir **belirsizlik çarpanı** olarak verilir (F_U ile çarp ya da böl).

**Ne elde edersiniz.** Üç varyans bileşeni, u(numune alma), u(analiz), genişletilmiş ölçüm belirsizliği ve numune alma ile analizin payları.

> **Örnek:** [Marulda nitrat](example:ufs-a1-nitrate-lettuce), [belirsizlik çarpanıyla toprakta kurşun](example:ufs-a2-lead-soil-uncertainty-factor), [Gy'nin numune alma teorisi](example:ufs-a5-enzyme-feed-gy).
