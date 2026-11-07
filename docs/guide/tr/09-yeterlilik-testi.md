# 9. Yeterlilik testi (ISO 13528)

Bir `proficiency` projesi, bir ölçülen büyüklük için bir yeterlilik testi programının bir turunu içerir:
katılımcıların sonuçları (bildirilmişse belirsizlikleriyle), atanan değerin ve yeterlilik değerlendirmesi için
standart sapmanın nasıl elde edildiği ve isteğe bağlı olarak yeterlilik testi numunelerinin homojenlik ve
kararlılık kontrolleri. Program ISO 13528:2022'nin istatistiksel yöntemlerini uygular ve standardın E.2, E.3 ve
E.4 çözümlü örnekleriyle doğrulanmıştır (`iso13528-*` kütüphane projeleri).

## 9.1 Özet istatistikler (Ek C)

p sayısal sonuç için ("< 0,015" gibi sansürlü sonuçlar listelenir ancak kullanılmaz) program medyanı,
ölçeklenmiş medyan mutlak sapmayı MADe = 1,483 med|xᵢ − med(x)|, normalleştirilmiş çeyrekler açıklığını
nIQR = 0,7413 (Q₃ − Q₁), aritmetik ortalamayı ve standart sapmayı ve **Algoritma A**'nın (C.3.1) robust
ortalaması x* ile robust standart sapması s*'ı verir.

Algoritma A, x* = medyan ve s* = MADe ile başlar; x* ± 1,5s* dışındaki her sonuç yakın sınırla değiştirilir ve
x* ile s*, değiştirilmiş değerlerin ortalaması ve standart sapmalarının 1,134 katı olarak yeniden hesaplanır.
İterasyon, x* ve s*'ın üçüncü anlamlı basamakları değişmeyince durur (C.3.1); `"algorithm_a": "converged"`
ise tam yakınsamaya kadar sürdürür. MADe sıfırsa başlangıçtaki s* örneklem standart sapmasıdır (C.3.1, Not 2).

Çeyrekler sıra istatistikleri arasında doğrusal enterpolasyonla hesaplanır; bu, ISO 13528 Tablo E.5'teki
nIQR'yi yeniden üretir. Başka yazılımlar başka tanımlar kullanabilir (Tablo E.5, not). Standart sapmaların
robust birleştirilmesi olan Algoritma S (C.4) motorda mevcuttur (`pt::algorithm_s`).

## 9.2 Atanan değer ve belirsizliği (Madde 7)

| `assigned_value.method` | x_pt | u(x_pt) |
|---|---|---|
| `algorithm_a` | x* | 1,25 s*/√p, Formül (6) |
| `median` | medyan | 1,25 nIQR/√p (veya MADe) |
| `mean` | aritmetik ortalama | s/√p |
| `reference` | formülasyon, CRM veya referans laboratuvardan değer (7.3–7.6) | verildiği gibi |

Homojensizlik, taşıma ve kararsızlık için belirsizlik bileşenleri eklenebilir (Formül 3). Uzlaşma değerinde
bunlar genellikle sonuçların dağılımına zaten yansımıştır (7.2.2, Not 2). 1,25 çarpanı muhafazakârdır;
`u_factor` ile değiştirilebilir.

Program u(x_pt) < 0,3 σ_pt olup olmadığını belirtir (Formül 10). Değilse atanan değerin belirsizliği ihmal
edilemez ve z′, ζ veya E_n skorları kullanılmalıdır (9.2.2).

## 9.3 Yeterlilik değerlendirmesi için standart sapma (Madde 8)

σ_pt ya verilir (amaca uygunluk, önceki turların deneyimi veya Horwitz eşitliği gibi genel bir model) ya da
turun sonuçlarından alınır (`robust`, `made`, `niqr`, `sd`). σ_pt yoksa z ve z′ skorları hesaplanmaz.

## 9.4 Performans istatistikleri (Madde 9)

| Skor | Formül | Yorum |
|---|---|---|
| D, D % | x − x_pt, 100 (x − x_pt)/x_pt | δ_E ile karşılaştırılır |
| PA | 100 D/δ_E (varsayılan δ_E = 3σ_pt) | ≥ %100 veya ≤ −%100: eylem |
| z | (x − x_pt)/σ_pt | \|z\| ≤ 2 kabul edilebilir, 2 < \|z\| < 3 uyarı, \|z\| ≥ 3 eylem |
| z′ | (x − x_pt)/√(σ_pt² + u²(x_pt)) | z gibi |
| ζ | (x − x_pt)/√(u²(x) + u²(x_pt)) | z gibi; katılımcının u(x) değerini gerektirir |
| E_n | (x − x_pt)/√(U²(x) + U²(x_pt)) | \|E_n\| < 1 yeterli |

Katılımcının standart belirsizliği, genişletilmiş belirsizliğinin kapsam faktörüne bölünmesiyle bulunur.
`uncertainty_screening` ile bildirilen belirsizlikler a (makul), b (u_min'in altında) veya c (u_max'ın
üstünde) olarak işaretlenir; ISO 13528 9.8, u_min = u(x_pt) ve u_max = 1,5 s* önerir.

ζ ve E_n skorları ile uzlaşma değerli z′ skoru, bir sonuç ile ondan hesaplanan atanan değer arasındaki
korelasyonu göz ardı eder; ISO 13528 (9.5.1, 9.6.1) bu yaklaşımı kabul eder.

## 9.5 Homojenlik ve kararlılık (Ek B)

Her biri m kez ölçülen g numune için B.3'ün varyans analizi, numune ortalamalarının standart sapması s_x'i,
numune içi standart sapma s_w'yi ve numuneler arası standart sapma s_s = √(s_x² − s_w²/m)'yi (negatifse sıfır)
verir. s_s ≤ 0,3 σ_pt ise numuneler yeterince homojendir (B.1). B.2.3'ün genişletilmiş ölçütü
√(F₁(0,3σ_pt)² + F₂s_w²) de verilir; F₁ = χ²₀,₉₅(g − 1)/(g − 1) ve F₂ = (F₀,₉₅(g − 1, g(m − 1)) − 1)/m tam olarak
hesaplanır (m = 2 için Tablo B.1).

Kareler ortalamaları MS_arası = m·s_x² ve MS_içi = s_w² listelenir. u*_bb = √(MS_içi/m)·(2/ν_içi)^¼ de verilir:
yöntemin tekrarlanabilirliğinin gizleyebileceği birimler arası standart sapma (Linsinger vd. 2001; ISO Guide 35).
MS_arası < MS_içi olup s_s sıfır çıktığında, referans malzemenin belirsizliği için olağan değer budur. Bir referans
malzemenin homojenlik çalışmasında olduğu gibi σ_pt verilmezse ISO 13528 ölçütleri değerlendirilmez.

**Bayesçi birimler arası standart sapma.** `"bayes": { "tau_scale", "sigma_scale", "mu_mean", "mu_sd" }` ile
program hiyerarşik modeli de değerlendirir: y_ij = µ + B_i + ε_ij, B_i ~ N(0, τ²), ε_ij ~ N(0, σ²). τ ve σ verilen
ölçeklerle yarı-Cauchy önsellere sahiptir; bu ölçekler genellikle hazırlama şartnamesi ve bilinen
tekrarlanabilirliktir. µ normal bir önsele, `mu_mean` ve `mu_sd` verilmezse düz bir önsele sahiptir. Birim
etkileri ve µ tam olarak integre edilir, (τ, σ) sonsal dağılımı bir ızgarada hesaplanır. Bu nedenle sonuç MCMC
olmadan her çalıştırmada aynıdır. Rapor τ ve σ'nın ortalamasını, standart sapmasını, medyanını ve modunu, simetrik
ve en kısa %95 aralıklarını verir (JCGM GUM-5 Örnek 12). τ'nun sonsal dağılımı önsel ölçeğine duyarlıdır (GUM-5
12.4.6); bu nedenle ölçek gerekçelendirilmelidir.

Kararlılık kontrolü öncesi ve sonrası ortalamaları karşılaştırır: |ȳ₁ − ȳ₂| ≤ 0,3 σ_pt (B.17); ortalamaların
belirsizlikleri verilmişse ölçüt 2√(u²(ȳ₁) + u²(ȳ₂)) kadar genişletilir (B.18).

## 9.6 Uygulamada

*Yeni yeterlilik testi turu* bir şablonla başlar. *Veriler* sekmesinde sonuçlar bir elektronik tablodan
yapıştırılır, her satırda bir laboratuvar: kod, sonuç, genişletilmiş belirsizlik, kapsam faktörü ve isteğe
bağlı olarak yöntem (sekme, noktalı virgül veya boşlukla ayrılmış; ondalık virgül kabul edilir; sansürlü
sonuç için "<0,015"). *Sonuçlar* sekmesi özet istatistikleri, atanan değeri, sinyalleriyle skor tablosunu ve z
skorlarının grafiğini gösterir; HTML raporu homojenlik grafiğini de ekler.

## 9.7 Laboratuvarlar arası karşılaştırmanın uzlaşı değeri

Bir anahtar karşılaştırma, bir referans malzemenin karakterizasyonu için laboratuvarlar arası çalışma ya da
yayımlanmış sonuçların meta-analizi, standart belirsizlikleri u_j olan n değer x_j verir. Değerler u_j'lerin
izin verdiğinden daha fazla dağıldığında 1/u_j² ağırlıklı ortalama ve belirsizliği 1/√Σu_j⁻² fazla iyimserdir.
`consensus` bölümü (7.14b) NIST Consensus Builder'ı (NICOB, kullanım kılavuzu 2020, Ek A; Koepke vd.,
Metrologia 54 (2017) S34) izler ve x_j = μ + λ_j + ε_j rastgele etkiler modelini kullanır: laboratuvar etkileri
λ_j'nin ortalaması 0, standart sapması τ'dur (*karanlık belirsizlik*).

**DerSimonian–Laird.** Cochran Q = Σu_j⁻²(x_j − x̄_w)² ile τ̂² = maks{0, (Q − n + 1)/(Σu_j⁻² − Σu_j⁻⁴/Σu_j⁻²)};
uzlaşı değeri w_j = 1/(τ̂² + u_j²) ağırlıklı ortalamadır, saf belirsizliği u_DL = 1/√Σw_j. Rapor Q'nun p
değerini (n − 1 serbestlik dereceli χ²) ve t_{n−1} ile genişletilen Knapp–Hartung standart belirsizliğini
u_DL·√(Σw_j(x_j − μ̂)²/(n − 1)) verir. τ̂ = 0 olduğunda yöntem ağırlıklı ortalamaya indirgenir (Cox 2002,
Prosedür A).

**Parametrik bootstrap (NICOB §A.2).** u_DL, az serbestlik derecesine dayanan τ̂'nun belirsizliğini yok sayar.
K tekrarın her birinde τ_k, Q'nun dağılımının gama yaklaşımından (Biggerstaff ve Tweedie 1997: Q'nun kesin
ortalama ve varyansına sahip gama), x_jk N(μ̂, τ_k² + u_j²) dağılımından ve ν_j verilmişse
u_jk = u_j√(ν_j/χ²_ν_j) olarak çekilir; DerSimonian–Laird μ_k yeniden hesaplanır. μ_k'lerin standart sapması
ile %2,5 ve %97,5 yüzdelikleri u'yu ve kapsam aralığını verir.

**Eşdeğerlik dereceleri (NICOB §A.5.1).** D_j = x_j − μ̂ ve U95 %(D_j): bootstrap değerleri D_jk = x_jk − μ_k'nin
ortalamasında ortalanan ve bunların %95'ini içeren en kısa aralığın yarı uzunluğu. |D_j| > U95 %(D_j) olan bir
sonuç modelin izin verdiği aralığın dışındadır (Jones ve Spiegelhalter 2011). HTML raporu bunları çizer.

**Doğrusal havuz (NICOB §A.4).** Ortalamaları x_j, standart sapmaları u_j olan normal (ν_j verilmişse ölçeklenmiş
Student t) dağılımların eşit ya da verilen ağırlıklarla karışımından bir örneklem. Ortalaması ve standart
sapması uzlaşı değeri ve belirsizliğidir; aralık yüzdeliklerden alınır. Karışım çok tepeli olabilir;
belirsizliği sonuçların tüm yayılımını kapsar.

**En çok olabilirlik (IUPAC 2024, Örnek 4F).** u_j'leri bilinen normal etkiler ve hatalar altında μ ve τ'nun
olabilirliği, μ̂(τ) ağırlıklı ortalama olmak üzere τ üzerinden en büyüklenir; standart belirsizlikler Hessian'ın
tersinden gelir. Az sonuçla bunlar küçük kalır; bootstrap tercih edilir.

**Bayesçi rastgele etkiler modeli (NICOB §A.3; IUPAC 2024 Örnek 4M ve 4N).** σ_j = u_j ile normal olabilirlik,
τ için yarım Cauchy, μ için normal ya da düz önsel. `"bayes": {}` ile NICOB varsayılanları kullanılır: τ
önselinin ortancası |x_j − ortanca(x)|'lerin ortancasıdır, μ düzdür (NICOB: N(0, 10⁵)). μ analitik olarak
integre edilir, τ'nun sonsalı önsel olasılıkta düzgün 10⁵ noktalı bir ızgarada (τ = s·tan(πθ/2)) hesaplanır.
Sonuç kesindir ve her çalıştırmada aynıdır; NICOB ve rehber MCMC kullanır. Rapor μ'nün sonsal ortalamasını ve
standart sapmasını, kantil aralığını, sağa çarpık τ'nun sonsal ortancasını ve aralığını verir. Sonlu ν_j için
NICOB'un σ_j önselleri uygulanmamıştır.

`"effects": "laplace"` ile laboratuvar etkileri standart sapması τ olan bir Laplace dağılımına sahiptir; NIST
Decision Tree bunu birkaç sonuç diğerlerinden çok uzakta olduğunda (normalden ağır kuyruklar) önerir. Her
sonucun olabilirliği bir Laplace ve bir normal yoğunluğun kapalı biçimli evrişimidir; μ ve τ iki boyutlu bir
ızgarada hesaplanır (20 sonuç için yaklaşık bir saniye). Possolo & Meija (2022) bu modeli Hubble–Lemaître
sabitinin 22 kestirimine Stan ile uydurur; program aynı 71,69(66) (km/s)/Mpc değerini verir.

Kütüphane: Koepke vd. 2017 (G, ¹³⁷Cs ve ⁹⁰Sr yarı ömürleri), IUPAC 2024 Örnek 3E/4M/4N (iridyum) ve 4F
(CCQM-K145), Possolo & Meija 2022 (Hubble–Lemaître sabiti). Henüz uygulanmayanlar: birini dışarıda bırakan ve iki taraflı eşdeğerlik dereceleri.

*Uygulamada* *Yeni uzlaşı değeri* bir şablonla başlar. *Veriler* sekmesinde katılımcı sonuçları her satıra bir
tane yapıştırılır (ad, değer, standart belirsizlik ve isteğe bağlı serbestlik derecesi) ve yöntemler açılıp
kapatılır; Bayes modeli normal ya da Laplace etkileri kullanabilir. *Sonuçlar* sekmesi uzlaşı değerlerini, τ'yu,
kapsam aralıklarını ve eşdeğerlik derecelerini gösterir; HTML raporu bunların grafiğini ekler.
