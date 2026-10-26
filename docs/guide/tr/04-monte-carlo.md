# 4. Monte Carlo yöntemi

JCGM 101:2008 (GUM Ek 1), *belirsizliklerin* yayılımı yerine *dağılımların* yayılımını kullanır: her girdinin
olasılık dağılımından rastgele değerler çekilir, model her çekim için hesaplanır ve çıktının elde edilen
değerleri onun olasılık dağılımını tanımlar. Doğrusallaştırma, normal çıktı varsayımı veya etkin serbestlik
derecesi gerekmez.

## 4.1 Adımlar (JCGM 101 Madde 7)

1. Her girdi büyüklüğüne bir olasılık dağılımı atanır (3. bölümdeki bilgilerle; kurallar JCGM 101 Madde 6'da).
2. Deneme sayısı M seçilir.
3. M adet girdi vektörü çekilir; korelasyonlu normal girdiler birlikte çekilir (kovaryans matrisinin Cholesky
   çarpanı).
4. Model her vektör için hesaplanır.
5. M çıktı değerinden tahmin (ortalama), standart belirsizlik (standart sapma) ve istenen kapsam olasılığı için
   bir kapsam aralığı hesaplanır.

Program, JCGM 101'in önerdiği rastgele sayı üretecini (geliştirilmiş Wichmann–Hill, Ek C.3), normal değerler
için Box–Muller dönüşümünü ve diğer dağılımlar için Ek C'deki üreteçleri kullanır. Her çalıştırma
**yeniden üretilebilirdir**: tohum (`"seed"`, varsayılan 1) ve deneme sayısı raporda yazılıdır.

### Monte Carlo yönteminde A tipi girdiler

n gözlem için JCGM 101 (6.4.9), n − 1 serbestlik dereceli, ortalaması x̄ ve ölçeği s/√n olan ölçekli ve
kaydırılmış bir t dağılımı atar. Bu dağılımın standart sapması (s/√n)·√((n − 1)/(n − 3)) olup s/√n'den
büyüktür; n ≤ 3 için varyansı sonsuzdur: Monte Carlo standart sapması bu durumda kararsızdır, kapsam aralığı
ise iyi tanımlıdır (kütüphanedeki EA-4/02 S6 örneğine bakınız).

### Korelasyonlu girdiler

Korelasyonlar normal dağılımlı girdiler için desteklenir. Program, normal olmayan girdiler arasındaki
korelasyonları sessizce yok saymak yerine yaymayı reddeder (JCGM 101 genel bir yöntem vermez; bunun yerine ortak
etkiyi açıkça modelleyiniz).

## 4.2 Deneme sayısı

* **Sabit:** `"trials": 1000000` (varsayılan). JCGM 101 7.2.2, M = 10⁶'nın %95'lik bir kapsam aralığını
  çoğunlukla bir veya iki anlamlı ondalık basamağa kadar doğru vermesinin beklenebileceğini belirtir.
* **Uyarlamalı** (JCGM 101 7.9): `"adaptive": { "ndig": 2 }`. Tahminin, standart belirsizliğin ve iki aralık
  uç noktasının parti ortalamalarının standart sapmaları δ/5'in altına inene kadar (`"tolerance_divisor": 5`
  ile; varsayılan 1, 7.9.4'ün basılı haline karşılık gelir) denemeler partiler halinde eklenir; δ, u(y)'nin
  `ndig` anlamlı basamaktaki sayısal toleransıdır. Rapor, yöntemin `max_trials` sınırından önce yakınsayıp
  yakınsamadığını belirtir.

Monte Carlo sonuçları örnekleme gürültüsü taşır. Programın doğrulamasında Monte Carlo değerlerinin
toleransları, tekrarlı çalıştırmalarla ölçülen bu gürültünün yaklaşık beş standart sapmasıdır
(`core/examples/mcm_dispersion.rs`).

## 4.3 Kapsam aralıkları

* **Olasılıksal simetrik** aralık: p = %95 için %2,5 ve %97,5 kantilleri (JCGM 101 7.7.2).
* **En kısa** aralık: değerlerin p kesrini içeren tüm aralıkların en kısası. Asimetrik bir dağılımda simetrik
  aralıktan farklıdır — alttan sıfırla sınırlı bir büyüklükte (JCGM 101 9.4, karşılaştırma kaybı) alt ucu sıfır
  olabilir.

`"interval": "shortest"` (varsayılan) veya `"symmetric"` ile seçiniz. Simetrik bir çıktı dağılımında ikisi de
aynı aralığı tahmin eder, ancak simetrik aralığın örnekleme gürültüsü çok daha azdır; kütüphane dağılımın
simetrik olduğunun bilindiği yerlerde (JCGM 101 9.2, 9.3) onu kullanır.

Monte Carlo yönteminin kapsam olasılığı varsayılan olarak GUM sonucununkine eşittir (t dağılımıyla %95, k = 2
ile %95,45); böylece ikisi karşılaştırılabilir.

## 4.4 GUM sonucunun doğrulanması (JCGM 101 Madde 8)

y ± U aralığının uç noktaları Monte Carlo aralığının uç noktalarından en fazla δ sayısal toleransı kadar
farklıysa GUM sonucu **doğrulanmış** olur:

* u(y) n_dig anlamlı basamakla c × 10ˡ olarak yazılır; δ = ½ × 10ˡ;
* d_low = |y − U − y_low|, d_high = |y + U − y_high|;
* d_low ≤ δ ve d_high ≤ δ ise doğrulanmıştır.

n_dig, `"validation_ndig"` ile belirlenir (varsayılan 2). Rapor δ, d_low, d_high değerlerini ve kararı yazar.
Eşiğe yakın durumlarda kararın kendisi de Monte Carlo gürültüsüne bağlıdır — JCGM 101 9.2.3 aynı model için bir
çalıştırmada "doğrulandı", diğerinde "doğrulanmadı" sonucunu alır.

## 4.5 GUM yaklaşımının yetersiz kaldığı durumlar

Kütüphane klasik durumları içerir:

| Durum | Örnek | Ne olur |
|---|---|---|
| tek baskın dikdörtgen girdi | JCGM 101 9.2.4; EA-4/02 S9, S10 | çıktı neredeyse dikdörtgendir; %95 için ±2u çok geniştir |
| tahmin değerinde sıfır duyarlılık katsayıları | JCGM 101 9.3 (kütle), 9.4 (karşılaştırma kaybı) | birinci mertebe GUM belirsizliğin çoğunu kaçırır |
| belirgin asimetrik çıktı | JCGM 101 9.4 | simetrik ±U anlamsızdır; en kısa aralık sıfırdan başlar |
| bir bölümde büyük bağıl belirsizlikler | QUAM A4 | çıktı çarpıktır; ortalama ve medyan farklıdır |

Bu durumların hepsinde raporlanması gereken Monte Carlo sonucudur; GUM sonucu modelin denetimi ve bütçenin
okunması için yararlıdır.
