# 7. Monte Carlo yöntemi, basitçe

## Fikir {#idea}

Bütün ölçümünüzü bir milyon kez tekrarlayabildiğinizi düşünün; her seferinde her girdinin farklı, ama olası bir değeriyle: biraz daha yüksek bir okuma, biraz daha düşük bir sıcaklık, biraz daha ağır bir standart. Bir milyon sonuç bulurdunuz ve bunların yayılması, sonucun ne kadar belirsiz olduğunu doğrudan gösterirdi.

**Monte Carlo yöntemi** (JCGM 101) tam olarak bunu bir bilgisayarla yapar:

1. Her girdi için dağılımından (normal, dikdörtgen, …) rastgele bir değer çekilir.
2. Sonuç modelle hesaplanır.
3. Bu çok sayıda (tipik olarak 10⁵ ile 10⁶ kez) tekrarlanır.
4. Tüm sonuçların ortalaması tahmin, standart sapmaları standart belirsizlik, %95'ini içeren aralık da kapsam aralığıdır.

Türev, duyarlılık katsayısı ya da sonucun normal dağıldığı varsayımı gerekmez.

## Neden iki yöntem? {#why}

GUM yöntemi ([6. bölüm](06-reading-results)) hızlıdır ve belirsizliğin nereden geldiğini gösteren bir bütçe verir. Ama iki basitleştirmeye dayanır:

- model tahminlerin çevresinde **doğrusal** kabul edilir;
- sonuç **normal** (ya da t) kabul edilir, böylece U = k · u_c %95'i kapsar.

Bunlar geçerliyken iki yöntem uyuşur. Geçerli değilken Monte Carlo sonucu daha güvenilirdir. Program ikisini de çalıştırır ve uyuşup uyuşmadıklarını söyler (*doğrulama*). Uygulamada: ölçümü anlamak ve iyileştirmek için GUM bütçesini, onu denetlemek için Monte Carlo sonucunu kullanın.

## İki yöntem ne zaman ayrışır? {#disagree}

| Durum | Ne olur | Açılacak örnek |
|---|---|---|
| **Tek bir dikdörtgen girdi baskın** | sonuç neredeyse dikdörtgendir; %95 aralığı 2u'dan dardır | [Baskın dikdörtgen girdili toplamsal model](example:jcgm101-9.2.4-additive-dominant-rectangular) |
| **Belirgin biçimde doğrusal olmayan model** | birinci mertebe GUM belirsizliğin bir kısmını kaçırır ya da tahmin kayar | [Karşılaştırma kaybı, x₁ = 0](example:jcgm101-9.4-comparison-loss-x000) |
| **Az serbestlik derecesi** | iki yöntem farklı girdi varyansları kullanır (aşağıya bakın) | 4. bölümdeki masa |
| **Fiziksel bir sınıra yakın sonuç** | dağılım kesilir ve asimetrik olur | [1 g/g'a yakın benzoik asit](example:bipm-isrd-06-qnmr-ba-btmsb-d4-cd3od) |
| **Asimetrik girdi** (beta, lognormal) | aralık tahmin etrafında simetrik değildir | [Beta dağılımı olarak saflık](example:iupac2024-5c-benzoic-acid-purity-beta) |

## 4. bölümdeki masa {#table}

Masa örneğinde GUM standart belirsizliği 0,560 mm idi, ama Monte Carlo yaklaşık 0,67 mm verdi ve doğrulama başarısız oldu. Yanlış bir şey yok. Beş okuma, yalnızca ν = 4 serbestlik dereceli bir A tipi girdidir:

- GUM okumalar için u = s/√n = 0,374 mm kullanır ve az serbestlik derecesini daha sonra, kapsam faktörü üzerinden hesaba katar;
- JCGM 101'in Monte Carlo yöntemi okumalar girdisini 4 serbestlik dereceli bir t dağılımından çeker; bunun standart sapması daha büyüktür: √(4/2) × 0,374 = 0,529 mm.

Yani iki yöntem aynı bilgiyi farklı yerlerde anlatır. 5 yerine 20 okumayla fark neredeyse kaybolur. Bir girdinin serbestlik derecesi az olduğunda rapor bunu bir notla açıklar.

## Ayarlar {#settings}

**Ayarlar**'da:

- **Monte Carlo yöntemini çalıştır**: açın ya da kapatın.
- **Uyarlamalı deneme sayısı (JCGM 101 7.9)**: program sonuçlar istenen anlamlı basamak sayısında (n_dig) kararlı olana kadar çekmeye devam eder. Önerilen seçim budur.
- **Deneme sayısı**: bunun yerine sabit bir sayı, örneğin 1 000 000.
- **Rastgele sayı tohumu**: rastgele sayıların başlangıç noktası. Aynı tohumla tam olarak aynı sonucu yeniden alırsınız; bu, tekrarlanabilirlik ve validasyon raporları için önemlidir.
- **Kapsam aralığı**: *en kısa* ya da *olasılıksal simetrik*.

> **İpucu:** Monte Carlo sonuçları tohum ve deneme sayısıyla biraz değişir. Bu normal bir örnekleme gürültüsüdür. İki çalıştırma arasında üçüncü anlamlı basamaktaki bir fark sorun değildir; birinci basamaktaki bir fark sorundur.

## Özet {#summary}

- GUM yöntemi bütçeyi verir; Monte Carlo yöntemi onu denetler.
- İkisi de JCGM belgelerince ve ISO/IEC 17025'e göre kabul edilir.
- Ayrışıyorlarsa, raporlamadan önce nedenini yukarıdaki tabloda bulun.
