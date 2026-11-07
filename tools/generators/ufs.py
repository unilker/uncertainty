"""Writes the Eurachem UfS:2019 library projects (run from the repository root)."""
import json
import math
from pathlib import Path

DOC = ("Eurachem/EUROLAB/CITAC/Nordtest/AMC Guide UfS:2019, Measurement uncertainty arising from sampling: "
       "a guide to methods and approaches, 2nd ed., M. H. Ramsey, S. L. R. Ellison and P. Rostron (eds.)")
URL = "https://eurachem.org/index.php/publications/guides/musamp"
ERRATA = "Eurachem UfS:2019 Errata, version 1.4a (2023)"


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


ROBUST = T("The Guide reports robust ANOVA estimates (programs RANOVA and RANOVA2) as its main result. Their algorithm is not "
           "given in the Guide and the robust values could not be reproduced (research log F63), so this project evaluates "
           "the classical ANOVA, whose results the Guide prints alongside and which are reproduced exactly.",
           "Kılavuz ana sonuç olarak robust ANOVA tahminlerini (RANOVA ve RANOVA2 programları) verir. Bunların algoritması "
           "kılavuzda yoktur ve robust değerler yeniden üretilememiştir (araştırma günlüğü F63); bu nedenle proje, kılavuzun "
           "yanında yazdırdığı ve birebir yeniden üretilen klasik ANOVA'yı değerlendirir.")

DUP = T("Duplicate method (Section 9.4.2): two samples are taken from each target by independent applications of the "
        "protocol and each is analysed twice; s_meas² = s_samp² + s_anal² (Equation 1).",
        "Çift numune yöntemi (Bölüm 9.4.2): her hedeften protokolün bağımsız uygulamalarıyla iki numune alınır ve her biri "
        "iki kez analiz edilir; s_ölçüm² = s_numune² + s_analiz² (Eşitlik 1).")

ANOVA = T("Classical nested ANOVA: MS_anal = Σ(d²/2)/(2n) for the 2n analytical pairs; s_samp² = (MS_samp − MS_anal)/2; "
          "s_T² = (MS_target − MS_samp)/4; negative variance estimates are set to zero.",
          "Klasik iç içe ANOVA: 2n analiz çifti için KO_analiz = Σ(d²/2)/(2n); s_numune² = (KO_numune − KO_analiz)/2; "
          "s_T² = (KO_hedef − KO_numune)/4; negatif varyans tahminleri sıfır alınır.")

REL = T("Relative expanded uncertainty U′ = 2·100·s/x̄ with the mean x̄ of all results (Equation 4, k = 2).",
        "Bağıl genişletilmiş belirsizlik U′ = 2·100·s/x̄; x̄ tüm sonuçların ortalamasıdır (Eşitlik 4, k = 2).")


def write(pid, title, desc, expl, sec, tags, body, expected, ref, notes=(), field="chemistry", extra_sources=()):
    meta = {"id": pid, "title": title, "description": desc, "explanation": list(expl), "field": field,
            "sources": [{"document": DOC, "section": sec, "url": URL}] + list(extra_sources),
            "tags": ["eurachem-ufs", "sampling"] + tags}
    if notes:
        meta["notes"] = list(notes)
    p = {"format": "uncertainty-calculations/project", "version": 1, "metadata": meta}
    p.update(body)
    p["expected"] = dict(expected, reference=ref)
    Path(f"library/{field}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")


# ---------------------------------------------------------------------------
# A1 nitrate in lettuce
A1 = [[3898, 4139, 4466, 4693], [3910, 3993, 4201, 4126], [5708, 5903, 4061, 3782], [5028, 4754, 5450, 5416],
      [4640, 4401, 4248, 4191], [5182, 5023, 4662, 4839], [3028, 3224, 3023, 2901], [3966, 4283, 4131, 3788]]
write(
    "ufs-a1-nitrate-lettuce",
    T("Nitrate in glasshouse lettuce: duplicate method, classical ANOVA (Eurachem UfS Example A1)",
      "Serada yetişen marulda nitrat: çift numune yöntemi, klasik ANOVA (Eurachem UfS Örnek A1)"),
    T("Eight bays of lettuce, each sampled twice (10-head composite samples, W pattern) with duplicate analyses by HPLC (Table A1.1).",
      "Sekiz marul bölmesi; her birinden iki numune (W düzeninde 10 baştan kompozit numune) ve HPLC ile çift analiz (Tablo A1.1)."),
    [DUP, ANOVA,
     T("Classical results (Figure A1.2): x̄ = 4345.5625 mg/kg; s_T = 556.2804, s_samp = 518.16089, s_anal = 148.18063 mg/kg; "
       "51.58 %, 44.76 % and 3.66 % of the total variance (s_total = 774.5296 mg/kg).",
       "Klasik sonuçlar (Şekil A1.2): x̄ = 4345,5625 mg/kg; s_T = 556,2804, s_numune = 518,16089, s_analiz = 148,18063 mg/kg; "
       "toplam varyansın %51,58, %44,76 ve %3,66'sı (s_toplam = 774,5296 mg/kg)."),
     REL,
     T("Target C has very different sample duplicates (5805.5 and 3921.5 mg/kg on average): it inflates the classical s_samp. "
       "The robust estimates of the Guide give U′ = 14.5 % (sampling), 7.6 % (analysis) and 16.4 % (measurement); "
       "the classical ones 23.8 %, 6.8 % and 24.8 %.",
       "C hedefinin numune çiftleri çok farklıdır (ortalama 5805,5 ve 3921,5 mg/kg): klasik s_numune'yi büyütür. "
       "Kılavuzun robust tahminleri U′ = %14,5 (numune alma), %7,6 (analiz) ve %16,4 (ölçüm); klasik tahminler %23,8, %6,8 ve %24,8 verir.")],
    "Appendix A, Example A1, Table A1.1 and Figure A1.2",
    ["duplicate-method", "anova", "nitrate", "lettuce", "food", "raw-data"],
    {"sampling": {"quantity": "Nitrate in lettuce, one bay", "unit": "mg/kg", "duplicates": A1,
                  "labels": list("ABCDEFGH")}},
    {"sampling": {"mean": chk(4345.5625, 5e-5), "s_between_target": chk(556.2804, 5e-5),
                  "s_sampling": chk(518.16089, 5e-5), "s_analysis": chk(148.18063, 5e-6), "s_total": chk(774.5296, 5e-5),
                  "percent_between_target": chk(51.583582, 5e-5), "percent_sampling": chk(44.756204, 5e-5),
                  "percent_analysis": chk(3.6602174, 5e-6)}},
    T("Classical ANOVA results printed in Figure A1.2 (the robust results are not compared).",
      "Şekil A1.2'de yazdırılan klasik ANOVA sonuçları (robust sonuçlar karşılaştırılmamıştır)."),
    notes=[ROBUST,
           T("The robust estimates (Figure A1.2): mean 4408.3237, s_T 565.39868, s_samp 319.04834, s_anal 167.94308 mg/kg. "
             "Huber's H15 applied to the eight target means reproduces the robust mean (4408.319), but no published variant "
             "reproduces the robust standard deviations.",
             "Robust tahminler (Şekil A1.2): ortalama 4408,3237, s_T 565,39868, s_numune 319,04834, s_analiz 167,94308 mg/kg. "
             "Sekiz hedef ortalamasına Huber H15 uygulanması robust ortalamayı (4408,319) verir, ancak yayımlanmış hiçbir "
             "değişken robust standart sapmaları yeniden üretmez.")])

# ---------------------------------------------------------------------------
# A2 lead in soil
A2 = [[787, 769, 811, 780], [338, 327, 651, 563], [289, 297, 211, 204], [662, 702, 238, 246], [229, 215, 208, 218],
      [346, 374, 525, 520], [324, 321, 77, 73], [56, 61, 116, 120], [189, 189, 176, 168], [61, 61, 91, 119]]
write(
    "ufs-a2-lead-soil-uncertainty-factor",
    T("Lead in contaminated top soil: classical ANOVA and uncertainty factor (Eurachem UfS Example A2)",
      "Kirlenmiş üst toprakta kurşun: klasik ANOVA ve belirsizlik faktörü (Eurachem UfS Örnek A2)"),
    T("Ten of 100 grid targets (30 m × 30 m, 0–150 mm) sampled in duplicate 3 m apart and analysed twice by ICP-AES (Table A2.2). "
      "The values are skewed: the uncertainty is also expressed as a factor from the ANOVA of ln x.",
      "100 grid hedefinden onu (30 m × 30 m, 0–150 mm) 3 m arayla çift numune alınarak ICP-AES ile iki kez analiz edilmiştir (Tablo A2.2). "
      "Değerler çarpıktır: belirsizlik ln x'in ANOVA'sından bir faktör olarak da ifade edilir."),
    [DUP, ANOVA,
     T("Classical results (Figure A2.2): x̄ = 317.8 mg/kg; s_T = 197.55, s_samp = 135.43, s_anal = 17.99, s_meas = 136.62 mg/kg; "
       "U′ = 85.23 % (sampling), 11.32 % (analysis), 85.98 % (measurement).",
       "Klasik sonuçlar (Şekil A2.2): x̄ = 317,8 mg/kg; s_T = 197,55, s_numune = 135,43, s_analiz = 17,99, s_ölçüm = 136,62 mg/kg; "
       "U′ = %85,23 (numune alma), %11,32 (analiz), %85,98 (ölçüm)."),
     T("ANOVA of ln x (Table A2.3): mean 5.478 (geometric mean 239.4 mg/kg), s_G = 0.66775 (between targets), 0.4784 (sampling), "
       "0.0567 (analysis), 0.4817 (measurement).",
       "ln x'in ANOVA'sı (Tablo A2.3): ortalama 5,478 (geometrik ortalama 239,4 mg/kg), s_G = 0,66775 (hedefler arası), 0,4784 "
       "(numune alma), 0,0567 (analiz), 0,4817 (ölçüm)."),
     T("Expanded uncertainty factor F_U = exp(2 s_G) (Equation 5): 2.6207 for the measurement, 2.6032 for sampling; a result of "
       "300 mg/kg lies between 300/2.62 = 115 and 300 × 2.62 = 784 mg/kg (Table A2.4).",
       "Genişletilmiş belirsizlik faktörü F_U = exp(2 s_G) (Eşitlik 5): ölçüm için 2,6207, numune alma için 2,6032; 300 mg/kg'lık bir "
       "sonuç 300/2,62 = 115 ile 300 × 2,62 = 784 mg/kg arasındadır (Tablo A2.4)."),
     T("u′ = √(exp(s_G²) − 1) = 0.5111 (Section 6.3); u′ > 0.2, so it is not multiplied by 2.",
       "u′ = √(exp(s_G²) − 1) = 0,5111 (Bölüm 6.3); u′ > 0,2 olduğundan 2 ile çarpılmaz.")],
    "Appendix A, Example A2, Tables A2.2–A2.4 and Figure A2.2",
    ["duplicate-method", "anova", "log-normal", "uncertainty-factor", "lead", "soil", "environment", "raw-data"],
    {"sampling": {"quantity": "Lead in top soil, one 30 m × 30 m target", "unit": "mg/kg", "duplicates": A2,
                  "labels": ["A4", "B7", "C1", "D9", "E8", "F7", "G7", "H5", "I9", "J5"], "log": True}},
    {"sampling": {"mean": chk(317.8, 5e-5), "s_between_target": chk(197.55, 0.005), "s_sampling": chk(135.43, 0.005),
                  "s_analysis": chk(17.99, 0.005), "s_measurement": chk(136.62, 0.005), "s_total": chk(240.19, 0.005),
                  "expanded_relative_sampling": chk(85.23, 0.005), "expanded_relative_analysis": chk(11.32, 0.005),
                  "expanded_relative_measurement": chk(85.98, 0.005),
                  "percent_between_target": chk(67.65, 0.005), "percent_sampling": chk(31.79, 0.005),
                  "percent_analysis": chk(0.56, 0.005),
                  "log": {"mean": chk(5.478, 5e-4), "s_total": chk(0.82337, 5e-6), "s_between_target": chk(0.66775, 5e-6),
                          "s_sampling": chk(0.4784, 5e-5), "s_analysis": chk(0.0567, 5e-5), "s_measurement": chk(0.4817, 5e-5),
                          "factor_sampling": chk(2.6032, 5e-5), "factor_analysis": chk(1.12, 5e-3),
                          "factor_measurement": chk(2.6207, 5e-5), "relative_measurement": chk(0.5111, 5e-5)}}},
    T("Classical ANOVA of Figure A2.2 and of the logarithms in Table A2.3; F_U and u′ of Sections 6.2 and 6.3.",
      "Şekil A2.2'nin klasik ANOVA'sı ve Tablo A2.3'teki logaritmalarınki; Bölüm 6.2 ve 6.3'teki F_U ve u′."),
    notes=[ROBUST,
           T("Figure A2.2 prints F_U = 2.6032 for sampling and 2.6207 for the measurement on the line of the classical ANOVA; "
             "the text gives 2.6034 = exp(2·0.4784) from the rounded s_G.",
             "Şekil A2.2 klasik ANOVA satırında numune alma için F_U = 2,6032 ve ölçüm için 2,6207 verir; metin yuvarlanmış "
             "s_G'den 2,6034 = exp(2·0,4784) verir."),
           T("Section 6.4 adds the analytical bias (−3.41 % ± 1.34 %) to the random analytical component: 5.24 % standard, "
             "U′_meas = 83.95 % with the robust sampling term; a model project can include such terms.",
             "Bölüm 6.4 analitik sapmayı (−%3,41 ± %1,34) rastgele analitik bileşene ekler: standart %5,24, robust numune alma "
             "terimiyle U′_ölçüm = %83,95; bu tür terimler bir model projesine eklenebilir.")])

# ---------------------------------------------------------------------------
# A3 dissolved iron in groundwater
A3 = [[0.815, 0.834, 0.912, 0.893], [1.8, 1.83, 1.94, 1.93], [1.69, 1.68, 1.79, 1.77], [2.62, 2.61, 2.83, 2.84],
      [1.66, 1.63, 1.58, 1.59], [1.52, 1.53, 1.47, 1.50]]
WELLS = ["99.474", "99.468", "99.469", "99.916", "99.327", "99.371"]
A3_DESC = T("Six monitoring wells of a groundwater body, each sampled twice (on-line filtered and preserved) with duplicate "
            "laboratory analyses by ICP-AES (Table A3.7).",
            "Bir yeraltı suyu kütlesinde altı izleme kuyusu; her birinden iki numune (hat üzerinde süzülmüş ve korunmuş) ve "
            "ICP-AES ile çift laboratuvar analizi (Tablo A3.7).")
ERRATA_SRC = {"document": ERRATA, "section": "Corrections to Table A3.7, Table A3.4 and the summary on p. 60", "url": URL}
write(
    "ufs-a3-iron-groundwater-range",
    T("Dissolved iron in groundwater: range method (Eurachem UfS Example A3, corrected)",
      "Yeraltı suyunda çözünmüş demir: aralık yöntemi (Eurachem UfS Örnek A3, düzeltilmiş)"),
    A3_DESC,
    [DUP,
     T("Analysis: R̄_A = mean of |S1A1 − S1A2| and |S2A1 − S2A2| = 0.0173 mg/L; s_A = R̄_A/d₂ = 0.0154 mg/L (d₂ = 1.128, ranges of two).",
       "Analiz: R̄_A = |S1A1 − S1A2| ve |S2A1 − S2A2| ortalaması = 0,0173 mg/L; s_A = R̄_A/d₂ = 0,0154 mg/L (d₂ = 1,128, iki değerin aralığı)."),
     T("Sampling: R̄_S = mean |x̄_S1 − x̄_S2| = 0.102 mg/L; s_S+A = R̄_S/d₂ = 0.091 mg/L; s_S = √(s_S+A² − s_A²/2) = 0.090 mg/L (CV 5.23 %).",
       "Numune alma: R̄_S = |x̄_S1 − x̄_S2| ortalaması = 0,102 mg/L; s_S+A = R̄_S/d₂ = 0,091 mg/L; s_S = √(s_S+A² − s_A²/2) = 0,090 mg/L (CV %5,23)."),
     T("Between wells: standard deviation of the well means s_T+S+A = 0.604 mg/L; s_T = √(0.604² − 0.091²/2) = 0.601 mg/L (CV 35 %).",
       "Kuyular arası: kuyu ortalamalarının standart sapması s_T+S+A = 0,604 mg/L; s_T = √(0,604² − 0,091²/2) = 0,601 mg/L (CV %35)."),
     T("U′ (k = 2): 1.8 % analysis, 10.5 % sampling, 11 % measurement, 70 % between wells (Table A3.4 and summary as corrected by the errata).",
       "U′ (k = 2): analiz %1,8, numune alma %10,5, ölçüm %11, kuyular arası %70 (düzeltme listesiyle düzeltilmiş Tablo A3.4 ve özet).")],
    "Appendix A, Example A3, Tables A3.4 and A3.7",
    ["duplicate-method", "range-method", "iron", "groundwater", "environment", "errata", "raw-data"],
    {"sampling": {"quantity": "Dissolved iron in groundwater, one well", "unit": "mg/L", "duplicates": A3, "labels": WELLS,
                  "method": "range"}},
    {"sampling": {"mean": chk(1.719, 5e-4), "s_analysis": chk(0.015, 5e-4), "s_sampling": chk(0.090, 5e-4),
                  "s_between_target": chk(0.601, 5e-4), "expanded_relative_analysis": chk(1.8, 0.05),
                  "expanded_relative_sampling": chk(10.5, 0.05), "expanded_relative_measurement": chk(11.0, 0.5),
                  "expanded_relative_between_target": chk(70.0, 0.5)}},
    T("Range calculations of Table A3.7 as corrected by errata 1.1 (s_T+S+A = 0.604, s_T = 0.601 mg/L, CV_T = 35 %); Table A3.4 and summary of the errata.",
      "Düzeltme listesi 1.1 ile düzeltilmiş Tablo A3.7 aralık hesapları (s_T+S+A = 0,604, s_T = 0,601 mg/L, CV_T = %35); düzeltme listesindeki Tablo A3.4 ve özet."),
    notes=[T("The printed Table A3.7 gives s_T+S+A = 0.55, s_T = 0.548 mg/L and CV_T = 32 % (between-target 64 % in Table A3.4); "
             "the errata (version 1.1, 2020) correct them to 0.604, 0.601 mg/L, 35 % and 70 %, which the program reproduces.",
             "Basılı Tablo A3.7 s_T+S+A = 0,55, s_T = 0,548 mg/L ve CV_T = %32 verir (Tablo A3.4'te hedefler arası %64); düzeltme "
             "listesi (sürüm 1.1, 2020) bunları 0,604, 0,601 mg/L, %35 ve %70 olarak düzeltir; program bunları yeniden üretir."),
           T("The summary table gives 1.9 % for the analytical uncertainty of the validation, Table A3.4 1.8 %; the range "
             "calculation gives 2·0.89 % = 1.8 %.",
             "Özet tablo validasyonun analitik belirsizliği için %1,9, Tablo A3.4 %1,8 verir; aralık hesabı 2·%0,89 = %1,8 verir."),
           T("The Guide uses d₂ = 1.128; the program uses 2/√π = 1.12838 (ISO 8258), a difference below the printed digits.",
             "Kılavuz d₂ = 1,128 kullanır; program 2/√π = 1,12838 (ISO 8258) kullanır; fark basılı basamakların altındadır.")],
    extra_sources=[ERRATA_SRC])
write(
    "ufs-a3-iron-groundwater-anova",
    T("Dissolved iron in groundwater: classical ANOVA (Eurachem UfS Example A3)",
      "Yeraltı suyunda çözünmüş demir: klasik ANOVA (Eurachem UfS Örnek A3)"),
    A3_DESC,
    [DUP, ANOVA,
     T("s_anal = 0.0136, s_samp = 0.0827, s_T = 0.601 mg/L at x̄ = 1.719 mg/L: U′ = 1.6 % (analysis), 9.6 % (sampling), 70 % "
       "(between wells), as in Table A3.5; the range method gives 1.8 %, 10.5 % and 70 %.",
       "x̄ = 1,719 mg/L'de s_analiz = 0,0136, s_numune = 0,0827, s_T = 0,601 mg/L: U′ = %1,6 (analiz), %9,6 (numune alma), %70 "
       "(kuyular arası), Tablo A3.5'teki gibi; aralık yöntemi %1,8, %10,5 ve %70 verir.")],
    "Appendix A, Example A3, Tables A3.5 and A3.7",
    ["duplicate-method", "anova", "iron", "groundwater", "environment", "raw-data"],
    {"sampling": {"quantity": "Dissolved iron in groundwater, one well", "unit": "mg/L", "duplicates": A3, "labels": WELLS}},
    {"sampling": {"expanded_relative_analysis": chk(1.6, 0.05), "expanded_relative_sampling": chk(9.6, 0.05),
                  "expanded_relative_between_target": chk(70.0, 0.5)}},
    T("ANOVA row of Table A3.5 (the RANOVA row, 1.8 %, 9.9 % and 72 %, is not compared).",
      "Tablo A3.5'in ANOVA satırı (RANOVA satırı, %1,8, %9,9 ve %72, karşılaştırılmamıştır)."),
    notes=[T("Table A3.5 also gives RANOVA estimates (ROBAN 1.0.1): 1.8 %, 9.9 % and 72 %; the robust algorithm is not implemented (F63).",
             "Tablo A3.5 RANOVA tahminlerini de verir (ROBAN 1.0.1): %1,8, %9,9 ve %72; robust algoritma uygulanmamıştır (F63).")],
    extra_sources=[ERRATA_SRC])

# ---------------------------------------------------------------------------
# A4 vitamin A in baby porridge
A4_40 = [[402, 325, 361, 351], [382, 319, 349, 362], [332, 291, 397, 348], [280, 278, 358, 321], [370, 409, 378, 460],
         [344, 318, 381, 392], [297, 333, 341, 315], [336, 320, 292, 306], [372, 353, 332, 337], [407, 361, 322, 382]]
A4_4 = [[400, 491, 323, 355], [413, 159, 392, 434], [315, 391, 252, 454], [223, 220, 357, 469], [462, 343, 262, 293],
        [353, 265, 305, 456], [298, 234, 152, 323], [425, 263, 417, 353], [622, 189, 291, 272], [292, 397, 142, 568]]
BATCHES = [f"B{i}" for i in range(1, 11)]
A4_DESC = T("Two samples (packages) from each of ten production batches of an oat porridge with bananas and apricots, each "
            "split into duplicate test samples for HPLC (no analytical duplicates of one test sample).",
            "Muzlu ve kayısılı yulaf lapasının on üretim partisinin her birinden iki numune (paket); her biri HPLC için çift deney "
            "numunesine bölünmüştür (aynı deney numunesinin tekrar analizi yoktur).")
write(
    "ufs-a4-vitamin-a-porridge-40g",
    T("Vitamin A in baby porridge, 40 g test samples: duplicate method (Eurachem UfS Example A4)",
      "Bebek lapasında A vitamini, 40 g deney numunesi: çift numune yöntemi (Eurachem UfS Örnek A4)"),
    A4_DESC,
    [DUP,
     T("SS_anal = 16595 (µg/100 g)², ν = 20: s_anal = 28.805 µg/100 g, RSD 8.28 % (Tables A4.5, A4.14).",
       "KT_analiz = 16595 (µg/100 g)², ν = 20: s_analiz = 28,805 µg/100 g, RSD %8,28 (Tablo A4.5, A4.14)."),
     T("SS_samp = 14231, ν = 10: V_samp = (14231/10 − 16595/20)/2 = 296.675, s_samp = 17.224 µg/100 g, RSD 4.95 % (Tables A4.6, A4.15).",
       "KT_numune = 14231, ν = 10: V_numune = (14231/10 − 16595/20)/2 = 296,675, s_numune = 17,224 µg/100 g, RSD %4,95 (Tablo A4.6, A4.15)."),
     T("u_meas = √(4.95² + 8.28²) = 9.65 %; U′ = 9.9 % (sampling), 16.6 % (analysis), 19.4 % (measurement); between batches RSD_B = 6.1 % (errata 1.4a).",
       "u_ölçüm = √(4,95² + 8,28²) = %9,65; U′ = %9,9 (numune alma), %16,6 (analiz), %19,4 (ölçüm); partiler arası RSD_B = %6,1 (düzeltme listesi 1.4a).")],
    "Appendix A, Example A4, Tables A4.3, A4.5–A4.7, A4.14, A4.15 and summary",
    ["duplicate-method", "anova", "vitamin-a", "food", "errata", "raw-data"],
    {"sampling": {"quantity": "Vitamin A (retinol) in baby porridge powder, 40 g test samples", "unit": "µg/100 g",
                  "duplicates": A4_40, "labels": BATCHES}},
    {"sampling": {"mean": chk(347.85, 5e-3), "s_analysis": chk(28.80538, 5e-5), "s_sampling": chk(17.224, 5e-4),
                  "expanded_relative_sampling": chk(9.9, 0.05), "expanded_relative_analysis": chk(16.6, 0.05),
                  "expanded_relative_measurement": chk(19.4, 0.15), "expanded_relative_between_target": chk(12.2, 0.05)}},
    T("Tables A4.5, A4.6 and A4.14–A4.15; U′ of the summary table and RSD_B = 6.1 % as corrected by errata 1.4a.",
      "Tablo A4.5, A4.6 ve A4.14–A4.15; özet tablonun U′ değerleri ve düzeltme listesi 1.4a ile düzeltilmiş RSD_B = %6,1."),
    notes=[T("The printed summary gives RSD_B = 8.2 % for the between-batch variation; errata 1.4a (2023) corrects it to 6.1 % "
             "(classical ANOVA of Table A4.3), which the program reproduces.",
             "Basılı özet partiler arası değişkenlik için RSD_B = %8,2 verir; düzeltme listesi 1.4a (2023) bunu %6,1'e düzeltir "
             "(Tablo A4.3'ün klasik ANOVA'sı); program bunu yeniden üretir."),
           T("Table A4.7 rounds U′ to 17 % (analysis) and 20 % (measurement); the summary gives 16.6 % and 19.4 %, i.e. twice the "
             "rounded u of 8.3 % and 9.7 %. Unrounded, U′_meas = 2·9.648 % = 19.3 %.",
             "Tablo A4.7 U′'yu %17 (analiz) ve %20 (ölçüm) olarak yuvarlar; özet %16,6 ve %19,4, yani yuvarlanmış u'nun (%8,3 ve %9,7) "
             "iki katını verir. Yuvarlamadan U′_ölçüm = 2·%9,648 = %19,3."),
           T("The laboratory's own analytical uncertainty (7 %) is lower than the 8.28 % found here; the Guide uses the larger value.",
             "Laboratuvarın kendi analitik belirsizliği (%7) burada bulunan %8,28'den küçüktür; kılavuz büyük değeri kullanır.")],
    extra_sources=[{"document": ERRATA, "section": "Correction to the RSD in the summary table of Example A4, p. 75", "url": URL}])
write(
    "ufs-a4-vitamin-a-porridge-4g",
    T("Vitamin A in baby porridge, 4 g test samples: negative sampling variance (Eurachem UfS Example A4)",
      "Bebek lapasında A vitamini, 4 g deney numunesi: negatif numune alma varyansı (Eurachem UfS Örnek A4)"),
    A4_DESC,
    [DUP,
     T("SS_anal = 312206.5, ν = 20: s_anal = 124.94 µg/100 g, RSD 36.68 % (Table A4.8).",
       "KT_analiz = 312206,5, ν = 20: s_analiz = 124,94 µg/100 g, RSD %36,68 (Tablo A4.8)."),
     T("SS_samp = 102860.25, ν = 10: V_samp = (102860.25/10 − 15610.325)/2 = −2662.15 < 0, conventionally set to zero (Table A4.9).",
       "KT_numune = 102860,25, ν = 10: V_numune = (102860,25/10 − 15610,325)/2 = −2662,15 < 0; geleneksel olarak sıfır alınır (Tablo A4.9)."),
     T("The small test portion dominates: U′_anal = 73.4 %. The Guide takes the sampling RSD of the 40 g study (5 %) and reports "
       "U′_meas = 74 % (Table A4.10): a 4 g test sample is not fit for purpose.",
       "Küçük deney porsiyonu baskındır: U′_analiz = %73,4. Kılavuz numune alma RSD'sini 40 g çalışmasından (%5) alır ve "
       "U′_ölçüm = %74 bildirir (Tablo A4.10): 4 g deney numunesi amaca uygun değildir.")],
    "Appendix A, Example A4, Tables A4.4, A4.8–A4.10",
    ["duplicate-method", "anova", "negative-variance", "vitamin-a", "food", "raw-data"],
    {"sampling": {"quantity": "Vitamin A (retinol) in baby porridge powder, 4 g test samples", "unit": "µg/100 g",
                  "duplicates": A4_4, "labels": BATCHES}},
    {"sampling": {"mean": chk(340.625, 5e-4), "s_analysis": chk(124.9413, 5e-4), "s_sampling": chk(0.0, 0.0),
                  "sampling_variance_negative": True, "expanded_relative_analysis": chk(73.4, 0.05)}},
    T("Tables A4.8 and A4.9 (V_samp = −2662.15 set to zero) and the analytical U′ of Table A4.10.",
      "Tablo A4.8 ve A4.9 (V_numune = −2662,15 sıfır alınır) ve Tablo A4.10'un analitik U′ değeri."),
    notes=[T("With a robust ANOVA the Guide finds u_samp = 6.9 % and u_anal = 30 %; it is not implemented (F63).",
             "Kılavuz robust ANOVA ile u_numune = %6,9 ve u_analiz = %30 bulur; uygulanmamıştır (F63)."),
           T("The measurement uncertainty of this project (73.4 %) contains the analytical term only, since the sampling variance "
             "of this data set is zero; Table A4.10 combines it with 5 % from the 40 g study.",
             "Bu projenin ölçüm belirsizliği (%73,4) yalnızca analitik terimi içerir, çünkü bu veri setinin numune alma varyansı "
             "sıfırdır; Tablo A4.10 bunu 40 g çalışmasındaki %5 ile birleştirir.")])

# A4 quality control chart
QC = [("P1", 322, 319, 350, 375), ("P2", 332, 317, 358, 393), ("P3", 443, 430, 461, 388), ("P4", 318, 383, 390, 334),
      ("P5", 252, 219, 265, 227), ("P6", 274, 239, 233, 217), ("P7", 206, 225, 198, 195), ("P8", 392, 335, 375, 416)]
runs = [[r[1], r[3]] for r in QC] + [[r[2], r[4]] for r in QC]
labels = [f"{r[0]}-A1" for r in QC] + [f"{r[0]}-A2" for r in QC]
u = math.hypot(4.95, 8.28)
write(
    "ufs-a4-vitamin-a-qc-chart",
    T("Vitamin A in baby porridge: control chart of duplicate samples (Eurachem UfS Example A4)",
      "Bebek lapasında A vitamini: çift numunelerin kontrol grafiği (Eurachem UfS Örnek A4)"),
    T("Quality control of sampling: two samples from one batch of each of eight porridges, each analysed twice (40 g); the "
      "relative difference D % of the two samples is charted against limits from the validated uncertainty (Table A4.13, Figure A4.4).",
      "Numune almanın kalite kontrolü: sekiz lapanın her birinin bir partisinden iki numune, her biri iki kez analiz (40 g); iki "
      "numunenin bağıl farkı D %, valide edilmiş belirsizlikten gelen sınırlara göre grafiğe işlenir (Tablo A4.13, Şekil A4.4)."),
    [T("Validated u_meas = √(u_samp² + u_anal²) = √(4.95² + 8.28²) = 9.65 % (Table A4.7).",
       "Valide edilmiş u_ölçüm = √(u_numune² + u_analiz²) = √(4,95² + 8,28²) = %9,65 (Tablo A4.7)."),
     T("Range chart of two values (Section 13.2.2): CL = 1.128·u = 10.9 %, WL = 2.83·u = 27.3 %, AL = 3.69·u = 35.6 % (11 %, 27 % and 36 % in the Guide).",
       "İki değerin aralık grafiği (Bölüm 13.2.2): CL = 1,128·u = %10,9, WL = 2,83·u = %27,3, AL = 3,69·u = %35,6 (kılavuzda %11, %27 ve %36)."),
     T("D % = |x_S1 − x_S2|/x̄·100 for each product and analysis: all 16 values are below the warning limit, as in Figure A4.4.",
       "Her ürün ve analiz için D % = |x_S1 − x_S2|/x̄·100: 16 değerin tümü uyarı sınırının altındadır, Şekil A4.4'teki gibi.")],
    "Appendix A, Example A4, Section 7.3, Table A4.13 and Figure A4.4; Section 13.2",
    ["quality-control", "control-chart", "range-chart", "vitamin-a", "food", "raw-data"],
    {"control_chart": {"quantity": "Vitamin A, duplicate samples (40 g test samples)", "chart": "relative_range",
                       "runs": runs, "labels": labels,
                       "central_line": {"mean": round(1.128 * u, 2)},
                       "limits": {"basis": "target", "sd": round(u, 2)}}},
    {"control_chart": {"center": chk(11.0, 0.15), "upper_warning": chk(27.0, 0.5), "upper_action": chk(36.0, 0.5),
                       "outside_warning": 0, "out_of_control": 0}},
    T("Limits of Section 7.3 (CL 11 %, WL 27 %, AL 36 %) and Figure A4.4: no value outside the warning limit.",
      "Bölüm 7.3'ün sınırları (CL %11, WL %27, AL %36) ve Şekil A4.4: uyarı sınırı dışında değer yok."),
    notes=[T("Section 7.3 writes the warning-limit factor as 2.38 in the formula but computes with 2.83 (= D_WL for duplicates, Nordtest TR 569 Table 4).",
             "Bölüm 7.3 uyarı sınırı faktörünü formülde 2,38 yazar ancak 2,83 ile hesaplar (= çiftler için D_WL, Nordtest TR 569 Tablo 4)."),
           T("The central line is the expected range d₂·u for the validated uncertainty, not the mean of the 16 values (12.1 %).",
             "Merkez çizgisi, valide edilmiş belirsizlik için beklenen aralık d₂·u'dur; 16 değerin ortalaması (%12,1) değildir.")])

# ---------------------------------------------------------------------------
# A5 enzyme in chicken feed (Gy)
write(
    "ufs-a5-enzyme-feed-gy",
    T("Enzyme in chicken feed: fundamental sampling error by Gy's theory (Eurachem UfS Example A5)",
      "Tavuk yeminde enzim: Gy kuramıyla temel numune alma hatası (Eurachem UfS Örnek A5)"),
    T("A 500 g primary sample from a 25 kg bag (0.05 % m/m enzyme), ground to < 0.5 mm, and a 2 g test portion for extraction "
      "and LC (5 % analytical RSD). Modelling approach with the material properties of Table A5.1.",
      "25 kg'lık torbadan 500 g birincil numune (%0,05 m/m enzim), < 0,5 mm'ye öğütülür ve ekstraksiyon ile LC için 2 g deney "
      "porsiyonu alınır (%5 analitik RSD). Tablo A5.1'deki malzeme özellikleriyle modelleme yaklaşımı."),
    [T("Constitution factor (Equation 8): c = (1 − a_L/α)²/(a_L/α)·ρ_c + (1 − a_L/α)·ρ_m = 2158.5 g cm⁻³ (2160 in the Guide), "
       "with a_L/α = 0.05 %/100 %, ρ_c = 1.08 and ρ_m = 0.67 g cm⁻³.",
       "Bileşim faktörü (Eşitlik 8): c = (1 − a_L/α)²/(a_L/α)·ρ_c + (1 − a_L/α)·ρ_m = 2158,5 g cm⁻³ (kılavuzda 2160); "
       "a_L/α = %0,05/%100, ρ_c = 1,08 ve ρ_m = 0,67 g cm⁻³."),
     T("Sampling constants C = f·g·β·c with f = 0.5, β = 1: C₁ = 540 (g₁ = 0.5) and C₂ = 270 g cm⁻³ (g₂ = 0.25).",
       "Numune alma sabitleri C = f·g·β·c, f = 0,5, β = 1: C₁ = 540 (g₁ = 0,5) ve C₂ = 270 g cm⁻³ (g₂ = 0,25)."),
     T("s_r² = C·d³·(1/M_S − 1/M_L) (Equation 6): primary sample s_r1 = 0.033 (d = 0.1 cm, 500 g of 25 000 g); test portion "
       "s_r2 = 0.130 (d = 0.05 cm, 2 g of 500 g).",
       "s_r² = C·d³·(1/M_S − 1/M_L) (Eşitlik 6): birincil numune s_r1 = 0,033 (d = 0,1 cm, 25 000 g'dan 500 g); deney porsiyonu "
       "s_r2 = 0,130 (d = 0,05 cm, 500 g'dan 2 g)."),
     T("s_t = √(s_r1² + s_r2² + 0.05²) = 0.143; U′ = 2·14.3 % = 28.5 % (28.6 % in the Guide from the rounded components); "
       "the test portion is the weakest link.",
       "s_t = √(s_r1² + s_r2² + 0,05²) = 0,143; U′ = 2·%14,3 = %28,5 (kılavuzda yuvarlanmış bileşenlerden %28,6); en zayıf "
       "halka deney porsiyonudur.")],
    "Appendix A, Example A5, Table A5.1; Section 10.2.7–10.2.8, Equations 6–8",
    ["modelling", "sampling-theory", "gy", "fundamental-sampling-error", "enzyme", "feed"],
    {"sampling": {"quantity": "Enzyme in chicken feed, one 25 kg bag", "unit": "% m/m",
                  "gy": {"lot_concentration": 0.05, "particle_concentration": 100, "critical_density": 1.08,
                         "matrix_density": 0.67, "analytical_rsd": 0.05,
                         "steps": [{"name": "Primary sample", "sample_mass": 500, "lot_mass": 25000, "particle_size": 0.1,
                                    "size_factor": 0.5},
                                   {"name": "Test portion", "sample_mass": 2, "lot_mass": 500, "particle_size": 0.05,
                                    "size_factor": 0.25}]}}},
    {"sampling": {"gy": {"constitution_factor": chk(2160, 2), "sampling_constants": [chk(540, 0.5), chk(270, 0.3)],
                         "rsd": [chk(0.033, 5e-4), chk(0.13, 5e-3)], "rsd_total": chk(0.143, 5e-4),
                         "expanded_relative_sampling": chk(26.8, 0.1), "expanded_relative_total": chk(28.6, 0.1)}}},
    T("Values of Section 5 and the summary of Example A5 (c, C₁, C₂, s_r1, s_r2, s_t; U′ 26.8 % sampling, 28.6 % total).",
      "Örnek A5'in Bölüm 5 ve özet değerleri (c, C₁, C₂, s_r1, s_r2, s_t; U′ numune alma %26,8, toplam %28,6)."),
    notes=[T("The text of Example A5 calls the constitution factor 'Equation 7'; in Section 10.2.8 it is Equation 8 (and the "
             "number 6 is used twice).",
             "Örnek A5 metni bileşim faktörüne 'Eşitlik 7' der; Bölüm 10.2.8'de bu Eşitlik 8'dir (ve 6 numarası iki kez kullanılmıştır)."),
           T("Fundamental sampling error only: incorrect sampling errors and analytical bias are assumed negligible.",
             "Yalnızca temel numune alma hatası: hatalı numune alma hataları ve analitik sapma ihmal edilebilir kabul edilmiştir.")])

# ---------------------------------------------------------------------------
# A6 cadmium and phosphorus in agricultural soil (modelling, GUM)
def soil(pid, analyte, analyte_tr, unit, squares, cm, cp, split, strat, anal_terms, expected_u, title_suffix, notes, expl_extra):
    xbar = sum(squares) / len(squares)
    xm = (xbar - 0.1 * cm) / 0.9
    xp = (xbar + 0.1 * cp) / 1.1
    hw_depth = (xm - xp) / 2 / xbar
    inputs = [
        {"name": "x_sqr", "unit": unit, "description": T(f"{analyte} in the composite samples of five squares A1, A3, B2, C1, C3 (Table A6.1)",
                                                         f"Beş karenin (A1, A3, B2, C1, C3) kompozit numunelerinde {analyte_tr} (Tablo A6.1)"),
         "distribution": {"type": "type_a", "observations": squares}},
        {"name": "f_strat", "description": T("Sampling strategy (bias from the spatial pattern)", "Numune alma stratejisi (konumsal düzenden sapma)"),
         "distribution": {"type": "normal", "mean": 1.0, "sd": strat}},
        {"name": "f_depth", "description": T("'Depth effect': ±10 % in depth with the gradient of Table A6.2, rectangular",
                                             "'Derinlik etkisi': Tablo A6.2'deki gradyanla derinlikte ±%10, dikdörtgen"),
         "distribution": {"type": "rectangular", "value": 1.0, "half_width": round(hw_depth, 5)}},
        {"name": "f_split", "description": T("Splitting by coning and quartering: mean RSD of duplicate splits (Table A6.3)",
                                             "Konileme ve dörde bölme ile bölme: çift bölmelerin ortalama RSD'si (Tablo A6.3)"),
         "distribution": {"type": "normal", "mean": 1.0, "sd": split}},
        {"name": "f_dry", "description": T("Moisture of air-dried soil, a range of 2 %, rectangular", "Havada kurutulmuş toprağın nemi, %2'lik aralık, dikdörtgen"),
         "distribution": {"type": "rectangular", "value": 1.0, "half_width": 0.01}},
    ] + [{"name": n, "description": d, "distribution": {"type": "normal", "mean": 1.0, "sd": s}} for n, d, s in anal_terms]
    expr = "x_sqr * f_strat * f_depth * f_split * f_dry * " + " * ".join(n for n, _, _ in anal_terms)
    write(
        pid,
        T(f"{analyte} in agricultural top soil: modelling approach{title_suffix[0]} (Eurachem UfS Example A6)",
          f"Tarım toprağında {analyte_tr}: modelleme yaklaşımı{title_suffix[1]} (Eurachem UfS Örnek A6)"),
        T("Mean concentration of an arable field of 0.32 ha (143 m × 22 m, 0–30 cm) from composite samples; uncertainty budget "
          "of the sampling effects (between locations, strategy, depth, splitting, drying) and of the analysis.",
          "0,32 ha'lık bir tarlanın (143 m × 22 m, 0–30 cm) kompozit numunelerden ortalama derişimi; numune alma etkilerinin "
          "(konumlar arası, strateji, derinlik, bölme, kurutma) ve analizin belirsizlik bütçesi."),
        [T("Model x_site = x̄_sqr·f_strat·f_depth·f_split·f_dry·f_anal with nominal correction factors of 1 (Section 4.3).",
           "Model x_site = x̄_sqr·f_strat·f_depth·f_split·f_dry·f_analiz; düzeltme faktörlerinin nominal değeri 1'dir (Bölüm 4.3)."),
         T("Between locations: mean of the five squares with u = s_sqr/√5 (Type A).",
           "Konumlar arası: beş karenin ortalaması, u = s_kare/√5 (A tipi)."),
         T(f"Depth: x₋ = (x̄ − 0.1 c₋)/0.9 and x₊ = (x̄ + 0.1 c₊)/1.1 with c₋ = {cm}, c₊ = {cp} {unit}; half-width (x₋ − x₊)/2, rectangular.",
           f"Derinlik: x₋ = (x̄ − 0,1 c₋)/0,9 ve x₊ = (x̄ + 0,1 c₊)/1,1; c₋ = {str(cm).replace('.', ',')}, c₊ = {str(cp).replace('.', ',')} {unit}; yarı genişlik (x₋ − x₊)/2, dikdörtgen."),
         T("Drying: 2 % range of moisture, u = 1 %/√3 = 0.6 %.", "Kurutma: %2'lik nem aralığı, u = %1/√3 = %0,6.")] + expl_extra,
        "Appendix A, Example A6, Tables A6.1–A6.6",
        ["modelling", "gum", "soil", "environment", analyte.lower()],
        {"model": {"output": {"name": "x_site", "unit": unit, "expression": expr,
                              "description": T(f"Mean {analyte.lower()} content of the field", f"Tarlanın ortalama {analyte_tr} içeriği")},
                   "inputs": inputs},
         "analysis": {"gum": {"coverage_factor": 2}, "mcm": {"trials": 200000, "seed": 1}}},
        {"gum": expected_u},
        T("Measurement result and expanded uncertainty of Section 6.7 (k = 2).", "Bölüm 6.7'deki ölçüm sonucu ve genişletilmiş belirsizlik (k = 2)."),
        notes=notes, field="chemistry")


soil("ufs-a6-cadmium-soil", "Cadmium", "kadmiyum", "mg/kg", [0.270, 0.285, 0.343, 0.355, 0.343], 0.14, 0.10, 0.037, 0.01,
     [("f_Rw", T("Within-laboratory reproducibility (mean of 10 test samples, GF-ZAAS)", "Laboratuvar içi tekrar üretilebilirlik (10 deney numunesinin ortalaması, GF-ZAAS)"), 0.036),
      ("f_ref", T("Certified value of the CRM", "CRM'nin sertifikalı değeri"), 0.027),
      ("f_bias", T("Day-to-day bias correction with the CRM (mean of 3)", "CRM ile günden güne sapma düzeltmesi (3'ün ortalaması)"), 0.027)],
     {"estimate": chk(0.32, 0.005), "expanded_uncertainty": chk(0.06, 0.005)}, ("", ""),
     [T("The depth term from Table A6.2 is 3.6 % (x₋ = 0.339, x₊ = 0.299 mg/kg); Table A6.6 gives 3.5 %. With it the combined "
        "uncertainty is 9.2 % instead of 9.1 %, and U = 18.4 % instead of 18.2 %; the rounded result 0.32 ± 0.06 mg/kg is the same.",
        "Tablo A6.2'den derinlik terimi %3,6'dır (x₋ = 0,339, x₊ = 0,299 mg/kg); Tablo A6.6 %3,5 verir. Bununla birleşik belirsizlik "
        "%9,1 yerine %9,2, U ise %18,2 yerine %18,4 olur; yuvarlanmış sonuç 0,32 ± 0,06 mg/kg aynıdır.")],
     [T("Splitting u = mean of the RSDs of Table A6.3 = 3.7 %; strategy 1 % (hypothesis-based pattern); analysis √(3.6² + 2.7² + 2.7²) = 5.2 % (Table A6.4).",
        "Bölme u = Tablo A6.3'teki RSD'lerin ortalaması = %3,7; strateji %1 (hipoteze dayalı düzen); analiz √(3,6² + 2,7² + 2,7²) = %5,2 (Tablo A6.4).")])
soil("ufs-a6-phosphorus-soil", "Phosphorus", "fosfor", "mg/kg", [124, 112, 120, 118, 105], 47, 35, 0.0316, 0.005,
     [("f_Rw", T("Within-laboratory reproducibility (CAL method)", "Laboratuvar içi tekrar üretilebilirlik (CAL yöntemi)"), 0.017),
      ("f_bias", T("Trueness from the reproducibility s_R of an interlaboratory comparison", "Laboratuvarlar arası karşılaştırmanın tekrar üretilebilirliği s_R'den doğruluk"), 0.095)],
     {"estimate": chk(116, 0.5), "expanded_uncertainty": chk(26, 0.5)}, ("", ""),
     [T("Table A6.1 prints s_sqr = 8.0 mg/kg; the five values give 7.43 mg/kg (6.4 %), and u_b-loc = 2.9 % as printed follows from 7.43/√5.",
        "Tablo A6.1 s_kare = 8,0 mg/kg yazar; beş değer 7,43 mg/kg (%6,4) verir ve basılı u_b-loc = %2,9 değeri 7,43/√5'ten çıkar."),
      T("The mean of the RSDs of Table A6.3 is 3.16 %, not the printed 3.3 %; the project uses 3.16 %. The combined uncertainty "
        "(11.3 %) and U = 26 mg/kg are unchanged.",
        "Tablo A6.3'teki RSD'lerin ortalaması basılı %3,3 değil %3,16'dır; proje %3,16 kullanır. Birleşik belirsizlik (%11,3) ve "
        "U = 26 mg/kg değişmez.")],
     [T("Splitting u = 3.16 % (mean of Table A6.3); strategy 0.5 %; analysis √(1.7² + 9.5²) = 9.7 % (Table A6.5).",
        "Bölme u = %3,16 (Tablo A6.3 ortalaması); strateji %0,5; analiz √(1,7² + 9,5²) = %9,7 (Tablo A6.5).")])
print("written")
