# User guide / Kullanım rehberi

**English**

1. [Introduction](en/01-introduction.md)
2. [Getting started](en/02-getting-started.md)
3. [The GUM uncertainty framework](en/03-gum.md)
4. [The Monte Carlo method](en/04-monte-carlo.md)
5. [Top-down evaluation](en/05-top-down.md)
6. [Reporting](en/06-reporting.md)
7. [Project file reference](en/07-project-files.md)
8. [Glossary (English–Turkish)](en/08-glossary.md)
9. [Proficiency testing (ISO 13528)](en/09-proficiency-testing.md)
10. [Calibration with uncertainties in both variables (ISO 6143)](en/10-calibration.md)
11. [Conformity assessment (JCGM 106)](en/11-conformity.md)
12. [Interlaboratory precision experiments (ISO 5725-2)](en/12-precision-experiments.md)
13. [Several output quantities (JCGM 102)](en/13-multivariate.md)
14. [Worked examples](en/examples.md) (generated)

**Türkçe**

1. [Giriş](tr/01-giris.md)
2. [Başlarken](tr/02-baslarken.md)
3. [GUM belirsizlik yaklaşımı](tr/03-gum.md)
4. [Monte Carlo yöntemi](tr/04-monte-carlo.md)
5. [Top-down değerlendirme](tr/05-top-down.md)
6. [Raporlama](tr/06-raporlama.md)
7. [Proje dosyası başvurusu](tr/07-proje-dosyasi.md)
8. [Sözlük (İngilizce–Türkçe)](tr/08-sozluk.md)
9. [Yeterlilik testi (ISO 13528)](tr/09-yeterlilik-testi.md)
10. [Her iki değişkende belirsizlikli kalibrasyon (ISO 6143)](tr/10-kalibrasyon.md)
11. [Uygunluk değerlendirmesi (JCGM 106)](tr/11-uygunluk.md)
12. [Laboratuvarlar arası kesinlik deneyleri (ISO 5725-2)](tr/12-kesinlik-deneyleri.md)
13. [Birden çok çıktı büyüklüğü (JCGM 102)](tr/13-cok-ciktili.md)
14. [Çözümlü örnekler](tr/examples.md) (otomatik üretilir)

The worked-examples chapters are generated from the example library and checked by the test suite
(`project/tests/library.rs`). After changing the library, regenerate them with

```sh
uncertainty catalogue library --lang en -o docs/guide/en/examples.md
uncertainty catalogue library --lang tr -o docs/guide/tr/examples.md
```

Çözümlü örnekler bölümleri örnek kütüphanesinden üretilir ve test takımı tarafından denetlenir. Kütüphane
değiştiğinde yukarıdaki komutlarla yeniden üretiniz.
