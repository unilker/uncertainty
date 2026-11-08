# Durum ve sonraki adımlar / Status and next steps

Son güncelleme: **2026-11-07**. Bu dosya çalışmaya ara verildiğinde nerede
kalındığını ve sırada ne olduğunu kaydeder.

- Ayrıntılı gerekçeler araştırma günlüğündedir: kararlar D…, bulgular F….
- Dal: `claude/sleepy-hypatia-1522g6`, `main`'e PR ile birleştirilir.

## 1. Mevcut durum (sayılar)

| | |
|---|---|
| Kütüphane | **284** kendi kendini doğrulayan proje (`library/`); her biri kaynakla karşılaştırılır |
| Testler | **203** Rust testi (bunlardan biri 284 projenin tamamını çalıştırır), **22** Flutter testi |
| Rehber | TR/EN 16 bölüm + otomatik üretilen örnekler bölümü (`docs/guide/`) |
| Proje bölümleri | `model`, `topdown`, `proficiency`, `calibration`, `precision_experiment`, `multivariate`, `control_chart`, `sampling`, `error_function`, `multicomponent_conformity`, `consensus` (+ `conformity`) |
| Araştırma günlüğü | oturum 1–21, kararlar D1–D77, bulgular F1–F98 (dizin: `findings-index.md`) |

## 2a. Oturum 21 (2026-11-07): yeni Drive klasörü, JCGM 100 Amd.1:2026, GUM-5:2026

1. **JCGM 100:2008/Amd.1:2026.** Tahminin ikinci mertebe terimi δ = ½ΣΣ ∂²f/∂xᵢ∂xⱼ u(xᵢ,xⱼ) her zaman
   hesaplanır. `higher_order` ile tahmine eklenir, aksi hâlde |δ| > 0,1 u_c olduğunda uyarı verilir (D56).
   İlişkili normal girdiler için yüksek mertebe varyansı Mana & Pennecchi 2007'ye göre hesaplanır (D57).
2. **GUM-5:2026 örnekleri kütüphanede** (`tools/generators/gum5.py`):
   - Örnek 1 (pH), 3 (glikoz), 14 (²⁰⁷Bi), 15 (çelik halat): F77, F78.
   - Örnek 2 (benzo[a]piren): **topluluk (ensemble) Welch–Satterthwaite** (Willink 2007, D58). GUM-5
     ν_eff = 4,8 veriyor; aynı enjeksiyonlardan gelen alanlar bir topluluk olunca ν_eff = 2,1 (F79).
   - Örnek 10 (TSPM uygunluk): **lognormal süreç dağılımı, bağıl u ile küresel risk, önselli sonsal p_c**
     (D59). GUM-5'in ortak dağılımı sd = 0,07·ρ̂ kullanıyor (F80).
   - Örnek 12 (şişeler arası homojenlik): **u*_bb ve Bayesçi hiyerarşik τ**, ızgarada, MCMC'siz (D60).
     τ'nun modu 0; basılı 0,3 µmol/mol `getmode` kodundan (F81).
   - Örnek 4 (gravimetrik gaz karışımı, 8 çıktı, `multivariate`): Tablo 4.2 Tablo 4.1'den çıkmıyor; u(CO₂)
     3,96 (basılı 5), H₂O 0,389 (basılı 0,379) (F82).
   - Örnek 9 (tork, EKK/AEKK ve Bayesçi eğim): model projeleri; Bayesçi sonuçlar betikle (F83 küçük).
   - Örnek 5 (sera gazı envanteri): bağımsız ve paylaşılan faaliyet verisi; u = 835,16 kt (F84 küçük).
   - Örnek 11 (tümör kesit alanı, Fourier): veriler şekilden okundu, yalnız betik; bilgi ölçütleri ve
     bildirilen sonuç tutarsız (F85).
   - Örnek 6, 7, 8, 13, 16 zaten kütüphanedeydi (JCGM 101/102); GUM-5 kaynak olarak eklendi.
3. **Kanıt betikleri:** `tools/research/gum5/` (ex2_mcm.py, ex10_risks.py, ex12_posterior.py).

## 2. Bu dönemde yapılanlar (oturum 16–20, 2026-10-26/27)

1. **Numune almadan gelen belirsizlik** (Eurachem UfS:2019). İç içe ANOVA,
   aralık yöntemi, ln-ANOVA ve Gy FSE uygulandı; A1–A6 kütüphanede (D50,
   F63–F67). Robust RANOVA, algoritması yayımlanmadığı için
   yeniden üretilemedi (F63).
2. **EURAMET cg-18.**
   - H2–H4 kütüphanede; Ek C hata yaklaşım fonksiyonu GLS ile (D51, D52,
     F68–F70).
   - PR #1 ile `main`'e birleştirildi.
3. **Literatür taraması** (Crossref, açık veritabanları): 74 kaynak, 26'sı
   öncelikli (`literature-search-2026-10.md`).
4. **Yazılım araçları, tam metinler** (kullanıcının Drive'ından): GTC, METAS
   UncLib, Mathmet taraması ve Bettencourt da Silva (2026) MUPR Excel aracı
   ile ekleri.
   - Boşluk analizi `literature-review.md` §3'te.
   - Excel aracının fluoranten örneği yeniden hesaplandı; F71–F73 bulundu
     (k yanlış basılmış, mutlak kesinlikte 1/R̄ eksik, ağırlıklı/aritmetik
     ortalama karışımı).
5. **Ağırlıklı ortalama geri kazanım.** `recovery.mean = "weighted"`,
   MUPR Eş. 16.1; yeni `given` test türü; iki Bettencourt projesi (D54).
6. **Açık erişimli makaleler.**
   - Suncal (Delker 2021, OSTI) okundu: TN 1900 E4'teki eksik 2 çarpanını
     bağımsız olarak doğruluyor (F28); kendi örneğinde girdiler yer
     değiştirmiş (F74).
   - Possolo 2025: belirsizliğin yeni tanımı sözlüğe eklendi.
   - Fritsch 2019, Shirono 2022 (iki bölüm) ve Sandia koruma bandı sunumu
     okundu.
7. **cg-18 7.4–7.5 ve Ek G.**
   - `error_function.in_use`: kullanımda tartım sonucunun belirsizliği, genel
     belirsizlik, minimum tartım.
   - H1.4/A, H1.4/B, H2.4/A ve H2.4/B kütüphanede (D55, F75, F76).
8. **Üreteçler depoya alındı.** Kütüphane üreteçleri `tools/generators/`
   altına taşındı; daha önce yalnızca geçici çalışma alanındaydı.

## 3. Nerede kaldık / bekleyenler

**Belge klasörü (2026-11-07'den beri):**
https://drive.google.com/drive/folders/1mgbbHJlYKsij_6twWugM8m19LktEnjk7

Klasörde 92 dosya var; envanter ve kullanım sırası `drive-inventory-2026-11.md`'de. Eski klasör paylaşımdan
kaldırıldı.

**Kullanıcıdan beklenen makaleler.** Aşağıdakilerden yalnızca IUPAC 2024 geldi; diğerleri hâlâ eksik. Bunlar açık erişimli olsa da buradan
indirilemedi; Drive klasörüne konacak:

1. Possolo et al. 2024, IUPAC kısa rehberi — 10.1515/pac-2022-1203 (kütüphaneye kimya örnekleri)
2. Palma et al. 2020 erratum — 10.1016/j.talanta.2019.120476 (Palma 2019 projelerini kontrol)
3. Dołęgowska et al. 2016, modified RANOVA — 10.1007/s10661-016-5584-9 (F63)
4. Ramsey & Rostron 2024 — 10.1007/s00769-024-01575-0
5. do Nascimento Maranho & Lourenço 2026 — 10.1016/j.aca.2026.346327 (asimetrik aralık, `reporting.uncorrected_bias` ile karşılaştırma)
6. Synek 2017 — 10.1007/s00769-017-1268-6
7. Lourenço & Oliveira 2024 — 10.1016/j.aca.2024.342597
8. Lafarge & Possolo 2015, NIST UM — 10.1080/19315775.2015.11721732
9. Huang 2019 — 10.1016/j.measurement.2018.12.089 (MCM'de ölçekli t dağılımı tartışması)
10. Ilinca 2026 — 10.3390/metrology6040076; Zeier 2025 VNA Tools — 10.3390/metrology5040072
11. Volinsky 2026 — 10.1088/1681-7575/aea165; Petitjean & Musset 2026 — 10.1088/1361-6501/aeabcc; Ramsey 2025 VaMPIS — 10.21014/tc24-2025.030

**Diğer bekleyenler.**

- Bettencourt da Silva (2026) Ek 04 (pH, kontrol standartları hariç) iki
  kopyada da kesik geldi.
  - Ek 03'ten s_r = 0,0238 doğrulandı (F71).
  - 0,0229 değeri doğrulanamadı.
- Ücretli standartlar için `documents-needed.md`'ye bakılacak (ISO 11352,
  ISO/TS 28037, ISO 11843 …).

## 4. Sıradaki işler (öncelik sırasıyla)

0. **Drive belgeleri, `drive-inventory-2026-11.md` sırası.**
   - Tamamlananlar:
     - GUM-5 (16 örnek; F77–F85);
     - WG1 tanımları (D61), Eurachem/CITAC uygunluk 2021 (D62, F86);
     - EMUE E1.1, E2.1 (D63), E2.2, E3.3, E3.4, E3.8, E5.1, E6.5 (F78 eki, F85 eki, F87–F91);
     - IUPAC 2024 3A/4A/4B/4I (F92), ILAC P14 (D64), Nordtest TR 537 TR (D65);
     - Mari 2025 ve Pradella 2026 (sözlükte "Tartışma");
     - uzlaşı değeri modülü (D66, F93): DerSimonian–Laird, Knapp–Hartung, NICOB parametrik bootstrap,
       eşdeğerlik dereceleri, doğrusal havuz, en çok olabilirlik; Koepke 2017 (G, ¹³⁷Cs, ⁹⁰Sr) ve IUPAC 4F
       (CCQM-K145); Bayesçi rastgele etkiler modeli MCMC'siz ızgarada (D67; IUPAC 3E/4M/4N iridyum, F94, F95).
   - Possolo & Meija 2022: Hubble (Laplace etkili Bayes, D68), anyon açığı, tank, Wheatstone, kurşun (F96).
   - BIPM ISRD qNMR saflık bütçeleri: 9 proje (D69), tablolar arası tutarsızlıklar (F97).
   - Bayesçi öğreticiler, Toman 2016, Degenhardt 2025 okundu (D70); Possolo & Bodnar 2018 krom örneği kütüphanede.
   - UME prosedür taslağı: iki örnek kütüphanede, yazara notlar (D71).
   - Makale iskeleti yazıldı: `docs/article/outline.md` (D72).
   - D12 sınıf denetimi tamam (D73): `finding-classes.tsv`; 29 bulguda c bileşeni.
   - Suncal karşılaştırması (D74): 107 projede birinci mertebe u 6×10⁻¹² içinde aynı; 48 projede MCM örnekleme hatası içinde.
   - Sıradakiler: makale iskeletindeki §9 kontrol listesi (c sınıfı bulguları güncel baskılarla kontrol, bağımsız inceleme);
     QAC 2026, A2LA G104, eğitim örnekleri (Kirkup, White 2008, Leito, Kessel).
   - Açık kalan küçük işler:
     - skaler modelde ortak t dağılımı (GUM-5 Örnek 2 MCM'si);
     - tekil çıktı kovaryanslı çok değişkenli MCM (bolluklar);
     - F79'u Willink 2007 metniyle, F92'yi Meija vd. ile doğrulamak;
     - uzlaşıda sonlu ν_j için σ_j önselleri (NICOB §A.3), birini dışarıda bırakan ve iki taraflı eşdeğerlik dereceleri.
     - (tamam) uygulamada `consensus` formu ve şablonu (D76); homojenlikte Bayes model anahtarı ve önsel ölçekleri.
1. **TUR'a dayalı koruma bantları — tamam (D77).** `tur_method`: RSS, Dobbert (M = 1,04 − exp(0,38 ln TUR − 0,54),
   GBF = 1 − M/TUR), RP-10, U₉₅; Suncal ile doğrulandı. Kalan: Dobbert 2008 ve Z540.3 el kitabını birincil kaynak
   olarak okumak; en az maliyet/minimaks yöntemleri.
2. **cg-18 H3.4/A ve H3.4/B — tamam** (iki proje; F98: u(a₁) basılı değerden %0,3–1,6 farklı).
3. **Uygulamada `in_use` formu.** Şu anda yalnızca JSON sekmesinden
   düzenlenebiliyor.
4. Makaleler geldikçe:
   - IUPAC 2024 örneklerini kütüphaneye eklemek;
   - Palma 2020 erratumunu 7 projeyle karşılaştırmak;
   - Dołęgowska ile robust ANOVA'yı yeniden denemek (F63);
   - Maranho 2026 ve Synek 2017 ile düzeltilmemiş sapma aralıklarını
     karşılaştırmak.
5. **EMUE derlemesi** (41 ücretsiz örnek, van der Veen & Cox 2021,
   http://empir.npl.co.uk/emue/) kütüphane kaynağı olarak.
6. **Diğer uzantılar.**
   - VaMPIS (Eurachem 2025);
   - parametrik olmayan UfS (Lourenço & Oliveira 2024);
   - Mateos 2020'deki ν'ye bağlı ağırlıklar (F46);
   - Shirono 2022 ekonomik kabul aralığı;
   - ~~GTC'deki ilişkili girdiler için Willink ν'si~~ (D58 ile yapıldı).
7. **Faz 7.**
   - NIST UM ve Suncal ile karşılaştırmalı ölçümler;
   - Scopus/WoS taraması kurumsal ağdan; bunun için Claude Desktop'ta yerel
     oturum gerekir, bulut oturumu kurumsal ağa erişemez;
   - makale taslağı (`publication-plan.md`);
   - DOI'li sürüm.

## 5. Makale için kanıt dosyaları

| Dosya | İçerik |
|---|---|
| `research-log.md` | tüm kararlar ve bulgular, tam metin (değiştirilmez, yalnızca eklenir) |
| `findings-index.md` | 97 bulgu ve 71 kararın dizini (`tools/research/findings_index.py` ile üretilir) |
| `validation-matrix.md` | her örnek için basılı ve hesaplanan değerler |
| `literature-review.md`, `literature-search-2026-10.md`, `search-2026-10/` | literatür incelemesi, tarama raporu ve ham tarama verileri |
| `sources-provenance.md` | okunan her belgenin kaynağı ve nasıl edinildiği |
| `publication-plan.md` | hedef dergiler, katkı iddiaları, ilk taslak |
| `../article/outline.md`, `../article/numbers.md` | makale iskeleti (bölümler, kanıtlar, şekiller, kontrol listesi) ve üretilen sayılar (`tools/research/article_stats.py`) |
| `tools/generators/`, `tools/research/` | kütüphaneyi üreten betikler; bulguların arkasındaki analiz betikleri |

Gönderimden önce yapılacaklar:

- Bulgu sınıfları denetlendi (D73, `finding-classes.tsv`). F72, F75 ve F76
  c olarak düzeltildi.
- Güncel baskıları ve errataları kontrol etmek.

## 6. Çalışma notları

- **Lisanslı belgeler.** Lisanslı standartlar ve ücretli makale PDF'leri
  depoya konmaz. Kütüphaneye yalnızca sayısal veriler ve sonuçlar, kaynak
  gösterilerek alınır. Kılavuzlardaki tutarsızlıklar kurumlara bildirilmez,
  makale için günlüğe yazılır.
- **Kütüphane iş akışı.**
  1. `tools/generators/<betik>.py`
  2. `python3 tools/library_index.py`
  3. `./target/release/uncertainty catalogue library --lang en|tr -o docs/guide/{en,tr}/examples.md`
  4. `cd app && dart run tool/bundle_library.dart`
- **Kontroller.**
  - Rust: `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`,
    `cargo test --workspace --release`.
  - Uygulama: `dart format -l 160`, `flutter analyze`, `flutter test`.
- **Kaynak dosyalarının çalışma kopyaları** geçicidir ve kapsayıcıyla
  birlikte silinir: cg-18 metni, MUPR, Drive'dan gelen PDF'ler. Yeni oturumda
  kullanıcıdan yeniden istenmeli veya açık kaynaklardan yeniden indirilmeli.
