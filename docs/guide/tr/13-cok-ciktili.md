# 13. Birden çok çıktı büyüklüğü (JCGM 102)

Birçok ölçüm aynı girdi büyüklüklerinden birden fazla sonuç verir. Örnekler: bir empedansın direnci ve
reaktansı, karmaşık bir büyüklüğün genliği ve fazı, tek bir kalibre termometreyle ölçülen birkaç sıcaklık. Bu
sonuçların tahminleri koreledir; onları kullanan sonraki bir hesap standart belirsizliklerin yanında
kovaryanslara da ihtiyaç duyar. `multivariate` bölümü bu amaçla JCGM 102:2011'i (GUM Ek 2) uygular. Bu Ekin
6., 7. ve 9. maddelerindeki örneklerle doğrulanmıştır (`jcgm102-*` kütüphane projeleri).

## 13.1 Modeller

- **Açık model** Y = f(X): `model` bölümünde olduğu gibi her çıktının kendi ölçüm fonksiyonu vardır.
- **Örtük model** h(Y, X) = 0 (`"implicit": true`): her çıktı için girdilerde ve çıktılarda sıfıra eşit bir
  denklem ve bir başlangıç değeri verilir. Program denklemleri kesin türevlerle Newton yöntemiyle çözer.
  Örneğin direnç termometresinin R₀(1 + Aθ + Bθ²) − r R_S = 0 denklemi θ için bir formüle dönüştürülmek
  zorunda değildir (9.5.2.4 not).

Girdi büyüklükleri bir model projesindeki gibi belirtilir. Bunlara ek olarak girdiler n takım eşzamanlı
gösterge değerinden oluşan bir seriden gelebilir (`indications`, 5.3.2). Tahminleri ortalamalardır.
Kareler ve çarpımlar toplamı matrisi M kovaryans matrisini verir:

- GUM yaklaşımı için U_x = M/(n(n − 1)) (`"covariance": "mean"`) veya çok değişkenli t dağılımının kovaryans
  matrisi, ν = n − N ile M/((ν − 2)n) (`"covariance": "t"`, 9.4.2.5);
- Monte Carlo yöntemi için S = M/ν ile çok değişkenli t dağılımından, t_ν(x̄, S/n), çekilişler (5.3.2.4).

İfadelerde dört bölgeli ters tanjant `atan2(y, x)` kullanılabilir. Bu fonksiyon karmaşık bir büyüklüğün
fazını (−π, π] aralığında verir (9.3.1.1 not).

## 13.2 GUM belirsizlik yaklaşımı (Madde 6)

U_y = C_x U_x C_xᵀ; burada C_x, ∂f_j/∂X_i duyarlılık katsayılarının m × N matrisidir (formül 3). Örtük
modelde C_y = ∂h/∂Y ve C_x = ∂h/∂X ile C_y U_y C_yᵀ = C_x U_x C_xᵀ'dir (formül 8). Program
∂Y/∂X = −C_y⁻¹ C_x duyarlılıklarını C_y'nin tersini alarak değil, doğrusal sistemi çözerek bulur (Ek B).

Rapor şunları verir:

- tahminler ve standart belirsizlikleri;
- korelasyon matrisi;
- duyarlılık matrisi;
- N(y, U_y) Gauss dağılımı için iki %95 kapsam bölgesi (6.5):
  - **hiper-elipsoit** (η − y)ᵀ U_y⁻¹ (η − y) ≤ k_p²; k_p², χ²_m dağılımının p-kantilidir (Tablo 1: m = 2
    için 2,45);
  - **hiper-dikdörtgen** y_j ± k_q u(y_j); m aralığın her birinin kapsam olasılığı q = 1 − (1 − p)/m'dir
    (Tablo 2: m = 2 için 2,24). Bu bölgenin kapsam olasılığı en az p'dir.

İki çıktı varsa her iki bölgenin alanı da verilir.

## 13.3 Monte Carlo yöntemi (Madde 7)

Girdilerin aynı çekilişleri tüm çıktılar için hesaplanır; böylece çıktılar arasındaki korelasyonlar doğrudan
çekilişlerden çıkar. Tahmin ve kovaryans matrisi, çekilişlerin ortalaması ve örnek kovaryans matrisidir
(7.6). Kapsam faktörleri şöyle bulunur:

- **k_p** (7.7.2): her çekiliş U_y = LLᵀ ile ẙ_r = L⁻¹(y_r − y) biçimine dönüştürülür. k_p, çekilişlerin p
  oranının aşmadığı |ẙ_r| uzaklığıdır.
- **k_q** (7.7.3): aynı yöntem, max_j |y_j,r − y_j|/u(y_j) uzaklığıyla.
- **en küçük kapsam bölgesi**, iki çıktı için (7.7.4): alanı 100 × 100'lük bir ağ üzerinde yaklaşık olarak
  hesaplanır.
- **GUM bölgelerinin kapsaması**: Monte Carlo çekilişlerinin bu bölgelerin içine düşen oranı.

Uyarlamalı yöntem (7.8.3) her biri max(10⁴, 100/(1 − p)) denemelik en az on bir grup çalıştırır. Grup
ortalamalarının standart sapmasının iki katı her sonuç için sayısal tolerans içinde kaldığında durur:
tahminler, standart belirsizlikler, korelasyon matrisinin en büyük özdeğeri, k_p ve k_q.

## 13.4 Doğrulama (Madde 8)

Her iki yöntem aynı kapsam olasılığıyla çalıştırıldığında GUM sonuçları Monte Carlo sonuçlarıyla
karşılaştırılır. Karşılaştırılan büyüklükler y_j, u(y_j), korelasyon matrisinin λ_maks değeri, k_p ve
k_q'dur. Toleranslar Monte Carlo değerlerinden `validation_ndig` anlamlı basamakla (varsayılan 2) oluşturulur.
Rapor hiper-elipsoidin ve hiper-dikdörtgenin doğrulanıp doğrulanmadığını ayrı ayrı belirtir. Örneğin
dikdörtgen ortak etkili toplamalı modelde (9.2.3) tahminler, belirsizlikler ve korelasyon uyuşur, ancak
kapsam bölgelerinin hiçbiri uyuşmaz: GUM'un k_p değeri 2,45, Monte Carlo yöntemininki 2,38'dir.

GUM hiper-elipsoidi doğrulanmadıkça raporlanan sonuç Monte Carlo sonucudur.

## 13.5 Dikkat edilecek noktalar

- Bir çıktı diğerlerine tam olarak bağlıysa U_y tekildir ve hiper-elipsoit tanımlı değildir. Örneğin
  empedans için Z² = R² + X²'dir (9.4.2.3 not 2). Program bu durumda bir kapsam faktörü yerine tekilliği
  bildirir.
- Gösterge değerleri azsa çok değişkenli t dağılımının kuyrukları kalındır. ν ≤ 4 için Monte Carlo standart
  sapmaları yavaş yakınsar: 9.4'te (ν = 3) 10⁶ denemeyle farklı tohumlar u(R) için 0,128 Ω ile 0,136 Ω
  arasında değerler verir.
- Modelin bir tekilliği yakınında GUM sonuçları büyüklük mertebeleri kadar yanlış olabilir; sıfıra yakın
  karmaşık bir büyüklüğün fazında olduğu gibi (9.3). Bu durumda elips ve dikdörtgen uygun kapsam bölgeleri
  değildir (9.3.4.4).
- Ekte bulunan noktalar araştırma günlüğüne kaydedilmiştir (F50, F51):
  - 7.7.2 örneğinde basılı matrisler Cholesky çarpanının ve tersinin devrikleridir;
  - Tablo 8'deki 6. takım, 1–5. takımların yuvarlanmış ortalamasıdır.
