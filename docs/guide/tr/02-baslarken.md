# 2. Başlarken

## 2.1 Kurulum

Hesaplama çekirdeği ve komut satırı aracı Rust ile yazılmıştır. Hazır derlenmiş sürümler yayımlanana kadar
kaynak koddan derleyiniz (Rust 1.75 veya üzeri):

```sh
git clone https://github.com/unilker/uncertainty
cd uncertainty
cargo build --release
# araç: target/release/kapsam
```

`cargo test`, kütüphanedeki her örnek dahil doğrulama takımını çalıştırır.

## 2.2 Komut satırı aracı

| Komut | Amaç |
|---|---|
| `kapsam run <proje.json>` | bir projeyi hesaplar ve rapor yazar |
| `kapsam check <dosyalar veya klasörler>` | projeleri hesaplar ve kayıtlı referans değerleriyle karşılaştırır (başarısızlıkta çıkış kodu 1) |
| `kapsam template [--topdown \| --proficiency \| --calibration \| --conformity \| --precision \| --multivariate \| --control-chart]` | başlangıç için en küçük proje dosyasını yazdırır |
| `kapsam catalogue <dosyalar veya klasörler>` | bu rehberin çözümlü örnekler bölümünü yazar |
| `kapsam --version`, `--help` | sürüm, atıf ve yardım |

`run` seçenekleri:

| Seçenek | Anlamı |
|---|---|
| `--lang en` / `--lang tr` | rapor dili (varsayılan İngilizce) |
| `--format md`, `html`, `json`, `pdf` | Markdown, grafikli bağımsız HTML, makinece okunabilir sonuçlar veya PDF belgesi (A4, grafiksiz); varsayılan olarak `--output` dosyasının uzantısından, aksi halde Markdown |
| `--output DOSYA` (`-o`) | raporu dosyaya yazar |
| `--no-mcm` | Monte Carlo hesabını atlar |
| `--trials N` | sabit Monte Carlo deneme sayısı (projedekini geçersiz kılar) |
| `--seed S` | rastgele sayı tohumu (projedekini geçersiz kılar) |

`check` seçenekleri:

| Seçenek | Anlamı |
|---|---|
| `--no-mcm` | Monte Carlo hesaplarını ve bunların referans değerlerini atlar |
| `--lab YOL` | laboratuvarın kendi projeleri (dosya veya klasör); raporda ayrı bir bölümde verilir |
| `--report DOSYA` | yazılım validasyon raporu yazar: DOSYA `.pdf` ile bitiyorsa PDF, `.html` ile bitiyorsa HTML, aksi halde Markdown |
| `--lang en` / `--lang tr` | validasyon raporunun dili |

Validasyon raporu, ISO/IEC 17025:2017 7.11.2'nin yazılımın kullanımdan önce ve değişikliklerden sonra
validasyonu için istediği kayıttır. Rapor yazılımı sürüm, git commit'i ve tarihi (derleme kaydedilmemiş değişiklik
içeriyorsa "-modified"), derleyici ve platformla tanımlar; tarih, seçenekler ve denetlenen dosya kümesinin SHA-256
özeti de yer alır. Sonuçlar yönteme göre özetlenir. Her proje kaynağı, karşılaştırılan değer sayısı, farkın
toleransa en büyük oranı ve dosyanın SHA-256 özetiyle listelenir (dosya girdileri, referans değerleri ve tohum,
deneme sayısı, yuvarlama gibi ayarları içerir). Laboratuvarın `--lab` ile verilen kendi projeleri ayrı bir bölümdedir. Toleransı dışında kalan değerler ayrıca sıralanır. Rapor, kontrol eden kişinin imzası için bir
satırla biter. Bir laboratuvar genellikle yeni bir sürümü kurduktan sonra

```sh
kapsam check library --lab projelerim --report validasyon-2026-10.html --lang tr
```

komutunu çalıştırır ve raporu dosyalar. Her hesap raporu da yazılım kimliği ve değerlendirilen projenin SHA-256
özetiyle biter; böylece teknik kayıt (7.5) bir sonucu girdilerine ve validasyonu yapılmış derlemeye bağlar.
Kütüphane örnekleri uygulamayı yayımlanmış değerlere karşı doğrular (verification); laboratuvarın kendi belirlediği
referans değerli projeleri ise yazılımın kendi kapsamı için validasyonunu sağlar (validation; VIM 2.44–2.45).

## 2.3 İlk hesaplama

Kütüphanedeki `library/chemistry/quam-a1-cadmium-standard.json` örneği, EURACHEM/CITAC QUAM Örnek A1'dir:
kadmiyum kalibrasyon standardının hazırlanması.

```sh
kapsam run library/chemistry/quam-a1-cadmium-standard.json --lang tr -o a1.html
```

Rapor sırasıyla şunları içerir:

1. **Ölçüm modeli** — formül, c_Cd = 1000 · m · P / V, varsa ara büyüklüklerle.
2. **Girdi büyüklükleri** — her girdinin birimi, ona atanan olasılık dağılımı ve dağılımın dayandığı bilgi
   (örneğin "0,9999 ± 0,0001", dikdörtgen).
3. **Belirsizlik bütçesi** — her girdinin tahmini, standart belirsizliği, serbestlik derecesi, duyarlılık
   katsayısı, katkısı |cᵢ|u(xᵢ) ve birleşik varyanstaki payı.
4. **Sonuç** — tahmin, birleşik standart belirsizlik, etkin serbestlik derecesi, kapsam faktörü, kapsam
   olasılığı, genişletilmiş belirsizlik ve ifadesiyle birlikte raporlanan sonuç.
5. **Grafikler** (HTML) — birleşik varyansa katkılar; GUM olasılık yoğunluğu ve iki kapsam aralığıyla birlikte
   Monte Carlo histogramı.
6. **Monte Carlo yöntemi** — deneme sayısı, tohum, tahmin, standart sapma, kapsam aralıkları ve GUM sonucunun
   doğrulanması (JCGM 101 Madde 8).
7. **Açıklama ve notlar** — projede saklanan adım adım açıklama ve kaynağa ilişkin notlar.
8. **Referans değerlerle karşılaştırma** — kütüphane örneklerinde, kayıtlı her referans değer toleransıyla.
9. **Kaynaklar, yöntemler ve atıf.**

## 2.4 Kendi projeniz

```sh
kapsam template > projem.json            # ölçüm modeli
kapsam template --topdown > topdown.json  # validasyon verilerinden top-down
```

Dosyayı herhangi bir metin düzenleyiciyle düzenleyip (biçim [7. bölümde](07-proje-dosyasi.md) açıklanmıştır)
çalıştırınız. Yanlış yazılmış bir alan adı veya tutarsız bir tanım (örneğin bir girdi için hem `sd` hem
`expanded`) sessizce geçilmez, hata olarak bildirilir.

Bir projeyle birlikte referans değerler saklamak için (örneğin bir validasyon çalışmasının sonuçları) `expected`
bölümü ekleyiniz; `kapsam check` program veya dosya her değiştiğinde projeyi doğrular.
