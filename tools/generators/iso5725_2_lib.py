"""Writes the ISO 5725-2:2019 Annex C library projects (run from the repository root)."""
import json
from pathlib import Path

DOC = ("ISO 5725-2:2019, Accuracy (trueness and precision) of measurement methods and results — Part 2: "
       "Basic method for the determination of repeatability and reproducibility of a standard measurement method")
URL = "https://www.iso.org/standard/69419.html"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}

def project(folder, name, title, desc, expl, sec, tags, notes, body, expected, ref):
    pid = f"iso5725-2-{name}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": folder,
                      "sources": [{"document": DOC, "section": sec, "url": URL}],
                      "tags": ["iso5725-2", "interlaboratory-study", "precision"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p["precision_experiment"] = body
    p["expected"] = {"precision_experiment": expected, "reference": ref}
    Path(f"library/{folder}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

# ---------------------------------------------------------------- C.1 sulfur in coal
C1 = [
 [[0.71,0.71,0.70,0.71],[1.20,1.18,1.23,1.21],[1.68,1.70,1.68,1.69],[3.26,3.26,3.20,3.24]],
 [[0.69,0.67,0.68],[1.22,1.21,1.22],[1.64,1.64,1.65],[3.20,3.20,3.20]],
 [[0.66,0.65,0.69],[1.28,1.31,1.30],[1.61,1.61,1.62],[3.37,3.36,3.38]],
 [[0.67,0.65,0.66],[1.23,1.18,1.20],[1.68,1.66,1.66],[3.16,3.22,3.23]],
 [[0.70,0.69,0.66,0.71,0.69],[1.31,1.22,1.22,1.24],[1.64,1.67,1.60,1.66,1.68],[3.20,3.19,3.18,3.27,3.24]],
 [[0.73,0.74,0.73],[1.39,1.36,1.37],[1.70,1.73,1.73],[3.27,3.31,3.29]],
 [[0.71,0.71,0.69],[1.20,1.26,1.26],[1.69,1.70,1.68],[3.27,3.24,3.23]],
 [[0.70,0.65,0.68],[1.24,1.22,1.30],[1.67,1.68,1.67],[3.25,3.26,3.26]]]
labs8 = [str(i) for i in range(1, 9)]
c1_body = {"quantity": "Sulfur content of coal", "unit": "%", "laboratories": labs8,
           "levels": [{"name": str(j + 1), "results": [C1[i][j] for i in range(8)]} for j in range(4)]}
c1_common_expl = [
    T("Form A (Table C.1): eight laboratories, four levels; laboratories 1 and 5 reported four or five results instead of three. All results are retained to show the calculation for unequal numbers of results (C.1.2, Note).",
      "Form A (Tablo C.1): sekiz laboratuvar, dört düzey; 1. ve 5. laboratuvarlar üç yerine dört veya beş sonuç bildirmiştir. Eşit olmayan sonuç sayılarıyla hesaplamayı göstermek için tüm sonuçlar tutulur (C.1.2, Not)."),
]
project("chemistry", "c1-sulfur-in-coal",
  T("Sulfur content of coal: repeatability and reproducibility (ISO 5725-2 Example C.1)",
    "Kömürün kükürt içeriği: tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725-2 Örnek C.1)"),
  T("ISO 5725-2:2019 C.1: a uniform-level experiment with eight laboratories at four levels (0.69 % to 3.25 %), no missing or outlying data, unequal numbers of results per cell.",
    "ISO 5725-2:2019 C.1: dört düzeyde (%0,69 – %3,25) sekiz laboratuvarlı, eksik veya aykırı veri içermeyen, hücre başına sonuç sayıları eşit olmayan tek düzeyli bir deney."),
  c1_common_expl + [
    T("Forms B and C (Tables C.2, C.3): cell means and standard deviations recorded to three decimals, one more than the results (8.2.10, 8.2.11); the standard works from these rounded values (C.1.6).",
      "Form B ve C (Tablo C.2, C.3): hücre ortalamaları ve standart sapmaları, sonuçlardan bir fazla olmak üzere üç ondalıkla kaydedilir (8.2.10, 8.2.11); standart bu yuvarlanmış değerlerle çalışır (C.1.6)."),
    T("Cochran's test (Formulae C.1–C.4), with n = 3, the number of results in the majority of cells: C = 0.341, 0.289, 0.580 and 0.311 against 0.516 (5 %) and 0.615 (1 %); the cell of laboratory 5 at level 3 is a straggler and is retained.",
      "Cochran testi (Formül C.1–C.4), hücrelerin çoğunluğundaki sonuç sayısı n = 3 ile: C = 0,341, 0,289, 0,580 ve 0,311; kritik değerler 0,516 (%5) ve 0,615 (%1); 3. düzeyde 5. laboratuvarın hücresi şüphelidir (straggler) ve tutulur."),
    T("Grubbs' tests of the cell means (Table C.4): no single stragglers or outliers; at level 2 the double-high statistic 0.108 for laboratories 3 and 6 is below the 5 % critical value 0.1101: stragglers, retained.",
      "Hücre ortalamalarına Grubbs testleri (Tablo C.4): tekli şüpheli veya aykırı değer yok; 2. düzeyde 3. ve 6. laboratuvarlar için ikili-yüksek istatistik 0,108, %5 kritik değeri 0,1101'in altındadır: şüpheli, tutulur."),
    T("Formulae (23)–(31) with unequal n_ij (C.14–C.24): at level 1, m̂ = 0.6904 %, s_r = 0.0152 %, s_L² = 0.000 460 5 and s_R = 0.0263 %.",
      "Eşit olmayan n_ij ile Formül (23)–(31) (C.14–C.24): 1. düzeyde m̂ = %0,6904, s_r = %0,0152, s_L² = 0,000 460 5 ve s_R = %0,0263."),
  ],
  "Annex C.1 (Tables C.1–C.5, Formulae C.1–C.24)", ["coal", "unequal-replicates"],
  [T("The text of C.1.5 says that the double-high results at levels 2 and 4 are stragglers, but at level 4 Table C.4 gives 0.132, above the 5 % critical value 0.1101; only level 2 has a straggler pair (research log F47). The same text is in ISO 5725-2:1994, B.1.5.",
     "C.1.5 metni 2. ve 4. düzeylerdeki ikili-yüksek sonuçların şüpheli olduğunu söyler; ancak 4. düzey için Tablo C.4 %5 kritik değeri 0,1101'in üstünde 0,132 verir; yalnızca 2. düzeyde şüpheli bir çift vardır (araştırma günlüğü F47). Aynı metin ISO 5725-2:1994 B.1.5'te de bulunur."),
   T("Formulae (C.2)–(C.4) print two different sums of squares in each line (e.g. 0.006 36 and 0.006 390); the test values follow from the second.",
     "Formül (C.2)–(C.4) her satırda iki farklı kareler toplamı basar (ör. 0,006 36 ve 0,006 390); test değerleri ikincisinden çıkar."),
   T("C.1.8 quotes s_r = 0.022 % and s_R = 0.045 % for the whole range: the arithmetic means of the four levels.",
     "C.1.8 tüm aralık için s_r = %0,022 ve s_R = %0,045 verir: dört düzeyin aritmetik ortalamaları.")],
  dict(c1_body, form_decimals=3, relationships=[]),
  {"levels": {
     "1": {"p": 8, "m": chk(0.690, 0.0005), "s_r": chk(0.015, 0.0005), "s_R": chk(0.026, 0.0005), "cochran": chk(0.341, 0.0005),
           "grubbs_low": chk(1.24, 0.005), "grubbs_high": chk(1.80, 0.005), "grubbs_double_low": chk(0.539, 0.0005), "grubbs_double_high": chk(0.298, 0.0005)},
     "2": {"p": 8, "m": chk(1.252, 0.0005), "s_r": chk(0.029, 0.0005), "s_R": chk(0.061, 0.0005), "cochran": chk(0.289, 0.0005),
           "grubbs_low": chk(0.91, 0.005), "grubbs_high": chk(2.08, 0.005), "grubbs_double_low": chk(0.699, 0.0005), "grubbs_double_high": chk(0.108, 0.0005)},
     "3": {"p": 8, "m": chk(1.667, 0.0005), "s_r": chk(0.017, 0.0005), "s_R": chk(0.035, 0.0005), "cochran": chk(0.580, 0.0005),
           "grubbs_low": chk(1.67, 0.005), "grubbs_high": chk(1.58, 0.005), "grubbs_double_low": chk(0.378, 0.0005), "grubbs_double_high": chk(0.460, 0.0005)},
     "4": {"p": 8, "m": chk(3.250, 0.0005), "s_r": chk(0.026, 0.0005), "s_R": chk(0.058, 0.0005), "cochran": chk(0.311, 0.0005),
           "grubbs_low": chk(0.94, 0.005), "grubbs_high": chk(2.09, 0.005), "grubbs_double_low": chk(0.679, 0.0005), "grubbs_double_high": chk(0.132, 0.0005)}}},
  T("ISO 5725-2:2019, Formulae (C.1)–(C.4), Tables C.4 and C.5.", "ISO 5725-2:2019, Formül (C.1)–(C.4), Tablo C.4 ve C.5."))

project("chemistry", "c1-sulfur-in-coal-reml",
  T("Sulfur content of coal: REML estimates (ISO 5725-2 Example C.1)",
    "Kömürün kükürt içeriği: REML tahminleri (ISO 5725-2 Örnek C.1)"),
  T("ISO 5725-2:2019 C.1.9: the data of Example C.1 evaluated by restricted maximum likelihood (Annex B.2) instead of Formulae (23)–(31).",
    "ISO 5725-2:2019 C.1.9: Örnek C.1 verilerinin Formül (23)–(31) yerine kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi."),
  c1_common_expl + [
    T("REML (B.2) maximizes the likelihood of the data with the general mean removed; σ_r² and σ_L² are the variance components of the one-way random-effects model. With unequal n_ij the general mean is weighted by w_i = 1/(s_L² + s_r²/n_i) (Formulae B.5, B.6).",
      "REML (B.2), genel ortalaması çıkarılmış verinin olabilirliğini en büyükler; σ_r² ve σ_L² tek yönlü rastgele etkiler modelinin varyans bileşenleridir. Eşit olmayan n_ij ile genel ortalama w_i = 1/(s_L² + s_r²/n_i) ağırlıklarıyla alınır (Formül B.5, B.6)."),
    T("The estimates (Table C.6) differ from Table C.5 by at most one unit in the third decimal: no material difference.",
      "Tahminler (Tablo C.6) Tablo C.5'ten üçüncü ondalıkta en çok bir birim farklıdır: önemli bir fark yoktur."),
  ],
  "Annex C.1.9 (Table C.6), Annex B.2", ["coal", "reml"], [],
  dict(c1_body, method="reml", relationships=[]),
  {"levels": {
     "1": {"p": 8, "m": chk(0.690, 0.0005), "s_r": chk(0.015, 0.0005), "s_R": chk(0.027, 0.0005)},
     "2": {"p": 8, "m": chk(1.254, 0.0005), "s_r": chk(0.029, 0.0005), "s_R": chk(0.062, 0.0005)},
     "3": {"p": 8, "m": chk(1.668, 0.0005), "s_r": chk(0.017, 0.0005), "s_R": chk(0.036, 0.0005)},
     "4": {"p": 8, "m": chk(3.253, 0.0005), "s_r": chk(0.026, 0.0005), "s_R": chk(0.060, 0.0005)}}},
  T("ISO 5725-2:2019, Table C.6.", "ISO 5725-2:2019, Tablo C.6."))

# ---------------------------------------------------------------- C.2 softening point of pitch
C2txt = """91.0 97.0 96.5 104.0|89.6 97.2 97.0 104.0
89.7 98.5 97.2 102.6|89.8 97.2 97.0 103.6
88.0 97.8 94.2 103.0|87.5 94.5 95.8 99.5
89.2 96.8 96.0 102.5|88.5 97.5 98.0 103.5
89.0 97.2 98.2 101.0|90.0 - 98.5 100.2
88.5 97.8 99.5 102.2|90.5 97.2 103.2 102.0
88.9 96.6 98.2 102.8|88.2 97.5 99.0 102.2
- 96.0 98.4 102.6|- 97.5 97.4 103.9
90.1 95.5 98.2 102.8|88.4 96.8 96.7 102.0
86.0 95.2 94.8 99.8|85.8 95.0 93.0 100.8
87.6 93.2 93.6 98.2|84.4 93.4 93.9 97.8
88.2 95.8 95.8 101.7|87.4 95.4 95.4 101.2
91.0 98.2 98.0 104.5|90.4 99.5 97.0 105.6
87.5 97.0 97.1 105.2|87.8 95.5 96.6 101.8
87.5 95.0 97.8 101.5|87.6 95.2 99.2 100.9
88.8 95.0 97.2 99.5|85.0 93.2 97.8 99.8"""
C2 = []
for line in C2txt.split("\n"):
    a, b = line.split("|"); a = a.split(); b = b.split()
    C2.append([[float(x) for x in (a[j], b[j]) if x != "-"] for j in range(4)])
labs16 = [str(i) for i in range(1, 17)]
c2_body = {"quantity": "Softening point of pitch", "unit": "°C", "laboratories": labs16,
           "levels": [{"name": str(j + 1), "results": [C2[i][j] for i in range(16)]} for j in range(4)],
           "exclude": [{"laboratory": "5", "level": "2",
                        "reason": T("single result: the method was first applied incorrectly and too little material remained (C.2.1.4); the solitary result is discarded (8.4.3 a)",
                                    "tek sonuç: yöntem önce yanlış uygulanmış ve yeterli malzeme kalmamıştır (C.2.1.4); tek sonuç atılır (8.4.3 a)")}]}
c2_expl = [
    T("Form A (Table C.7): sixteen laboratories, four levels, two results per cell. Laboratory 8 had no sample for level 1; laboratory 5 has a single result at level 2, which is discarded (8.4.3 a), so p = 15 at levels 1 and 2.",
      "Form A (Tablo C.7): on altı laboratuvar, dört düzey, hücre başına iki sonuç. 8. laboratuvarın 1. düzey numunesi yoktur; 5. laboratuvarın 2. düzeyde tek sonucu vardır ve atılır (8.4.3 a); bu nedenle 1. ve 2. düzeylerde p = 15."),
]
project("physical", "c2-softening-point-pitch",
  T("Softening point of pitch: repeatability and reproducibility with missing data (ISO 5725-2 Example C.2)",
    "Ziftin yumuşama noktası: eksik verilerle tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725-2 Örnek C.2)"),
  T("ISO 5725-2:2019 C.2: ring-and-ball softening point of pitch, sixteen laboratories at four levels (88 °C to 102 °C) with duplicate results and two missing cells.",
    "ISO 5725-2:2019 C.2: halka-bilye yöntemiyle ziftin yumuşama noktası; dört düzeyde (88 °C – 102 °C), ikili sonuçlu ve iki eksik hücreli on altı laboratuvar."),
  c2_expl + [
    T("With two results per cell, Cochran's statistic is the largest squared absolute difference over the sum of the squared differences (8.2.11, Note 1): C = 0.391, 0.424, 0.434 and 0.380 (Table C.10), below the 5 % critical values 0.471 (p = 15) and 0.452 (p = 16).",
      "Hücre başına iki sonuçla Cochran istatistiği en büyük mutlak fark karesinin fark kareleri toplamına oranıdır (8.2.11, Not 1): C = 0,391, 0,424, 0,434 ve 0,380 (Tablo C.10); %5 kritik değerleri 0,471 (p = 15) ve 0,452 (p = 16)'nın altındadır."),
    T("Grubbs' tests of the cell means (Table C.11): no single or double stragglers or outliers.",
      "Hücre ortalamalarına Grubbs testleri (Tablo C.11): tekli veya ikili şüpheli ya da aykırı değer yoktur."),
    T("For n = 2 the variances follow from Formulae (29) and (30): at level 1, s_r² = Σ(y_i1 − y_i2)²/(2p) = 1.2303 and s_L² = 1.5575, so m̂ = 88.40 °C, s_r = 1.109 °C and s_R = 1.670 °C (C.25–C.33).",
      "n = 2 için varyanslar Formül (29) ve (30)'dan çıkar: 1. düzeyde s_r² = Σ(y_i1 − y_i2)²/(2p) = 1,2303 ve s_L² = 1,5575; böylece m̂ = 88,40 °C, s_r = 1,109 °C ve s_R = 1,670 °C (C.25–C.33)."),
  ],
  "Annex C.2 (Tables C.7–C.12, Formulae C.25–C.33)", ["pitch", "missing-data", "duplicates"],
  [T("Table C.12 gives s_R = 1.915 °C at level 4; Formulae (29)–(31) give 1.9175 °C (1.918 °C), the value of the REML Table C.13 (research log F48). ISO 5725-2:1994 prints the same 1.915 °C.",
     "Tablo C.12 4. düzeyde s_R = 1,915 °C verir; Formül (29)–(31) 1,9175 °C (1,918 °C) verir; bu REML Tablo C.13'ün değeridir (araştırma günlüğü F48). ISO 5725-2:1994 de aynı 1,915 °C değerini basar.")],
  dict(c2_body, relationships=[]),
  {"levels": {
     "1": {"p": 15, "m": chk(88.40, 0.005), "s_r": chk(1.109, 0.0005), "s_R": chk(1.670, 0.0005), "cochran": chk(0.391, 0.0005),
           "grubbs_low": chk(1.69, 0.005), "grubbs_high": chk(1.56, 0.005), "grubbs_double_low": chk(0.546, 0.0005), "grubbs_double_high": chk(0.662, 0.0005)},
     "2": {"p": 15, "m": chk(96.27, 0.005), "s_r": chk(0.925, 0.0005), "s_R": chk(1.597, 0.0005), "final_cochran": chk(0.424, 0.0005)},
     "3": {"p": 16, "m": chk(97.07, 0.005), "s_r": chk(0.993, 0.0005), "s_R": chk(2.010, 0.0005), "cochran": chk(0.434, 0.0005),
           "grubbs_low": chk(1.76, 0.005), "grubbs_high": chk(2.27, 0.005), "grubbs_double_low": chk(0.548, 0.0005), "grubbs_double_high": chk(0.566, 0.0005)},
     "4": {"p": 16, "m": chk(101.96, 0.005), "s_r": chk(1.004, 0.0005), "cochran": chk(0.380, 0.0005),
           "grubbs_low": chk(2.22, 0.005), "grubbs_high": chk(1.74, 0.005), "grubbs_double_low": chk(0.500, 0.0005), "grubbs_double_high": chk(0.672, 0.0005)}}},
  T("ISO 5725-2:2019, Formulae (C.25)–(C.33), Tables C.10–C.12.", "ISO 5725-2:2019, Formül (C.25)–(C.33), Tablo C.10–C.12."))

project("physical", "c2-softening-point-pitch-reml",
  T("Softening point of pitch: REML estimates (ISO 5725-2 Example C.2)",
    "Ziftin yumuşama noktası: REML tahminleri (ISO 5725-2 Örnek C.2)"),
  T("ISO 5725-2:2019 C.2.9: the data of Example C.2 evaluated by restricted maximum likelihood (Annex B.2).",
    "ISO 5725-2:2019 C.2.9: Örnek C.2 verilerinin kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi."),
  c2_expl + [
    T("With two results in every cell the REML estimates equal the ANOVA estimates unless the estimate of σ_L² is negative; Table C.13 agrees with Table C.12 except s_R at level 4 (see the notes).",
      "Her hücrede iki sonuç varken σ_L² tahmini negatif olmadıkça REML tahminleri ANOVA tahminlerine eşittir; Tablo C.13, 4. düzeydeki s_R dışında Tablo C.12 ile aynıdır (notlara bakınız)."),
  ],
  "Annex C.2.9 (Table C.13), Annex B.2", ["pitch", "missing-data", "reml"],
  [T("Table C.13 gives s_R = 1.918 °C at level 4 where Table C.12 gives 1.915 °C; for balanced cells both methods give 1.9175 °C (research log F48).",
     "Tablo C.13 4. düzeyde s_R = 1,918 °C verirken Tablo C.12 1,915 °C verir; dengeli hücrelerde her iki yöntem de 1,9175 °C verir (araştırma günlüğü F48).")],
  dict(c2_body, method="reml", relationships=[]),
  {"levels": {
     "1": {"p": 15, "m": chk(88.40, 0.005), "s_r": chk(1.109, 0.0005), "s_R": chk(1.670, 0.0005)},
     "2": {"p": 15, "m": chk(96.27, 0.005), "s_r": chk(0.925, 0.0005), "s_R": chk(1.597, 0.0005)},
     "3": {"p": 16, "m": chk(97.07, 0.005), "s_r": chk(0.993, 0.0005), "s_R": chk(2.010, 0.0005)},
     "4": {"p": 16, "m": chk(101.96, 0.005), "s_r": chk(1.004, 0.0005), "s_R": chk(1.918, 0.0005)}}},
  T("ISO 5725-2:2019, Table C.13.", "ISO 5725-2:2019, Tablo C.13."))

# ---------------------------------------------------------------- C.3 creosote oil
C3 = [[4.44,4.39,9.34,9.34,17.40,16.90,19.23,19.23,24.28,24.00],[4.03,4.23,8.42,8.33,14.42,14.50,16.06,16.22,20.40,19.91],
      [3.70,3.70,7.60,7.40,13.60,13.60,14.50,15.10,19.30,19.70],[4.10,4.10,8.93,8.80,14.60,14.20,15.60,15.50,20.30,20.30],
      [3.97,4.04,7.89,8.12,13.73,13.92,15.54,15.78,20.53,20.88],[3.75,4.03,8.76,9.24,13.90,14.06,16.42,16.58,18.56,16.58],
      [3.70,3.80,8.00,8.30,14.10,14.20,14.90,16.00,19.70,20.50],[3.91,3.90,8.04,8.07,14.84,14.84,15.41,15.22,21.10,20.78],
      [4.02,4.07,8.44,8.17,14.24,14.10,15.14,15.44,20.71,21.66]]
labs9 = [str(i) for i in range(1, 10)]
c3_body = {"quantity": "Creosote oil, thermometric titration", "unit": "%", "laboratories": labs9,
           "levels": [{"name": str(j + 1), "results": [C3[i][2 * j:2 * j + 2] for i in range(9)]} for j in range(5)],
           "exclude": [{"laboratory": "1", "reason": T("outlying laboratory: cell means much higher than all others at every level (Mandel's h, Figure C.7); outliers by Grubbs' test at levels 3 and 4 (C.3.5)",
                                                        "aykırı laboratuvar: hücre ortalamaları her düzeyde diğerlerinin hepsinden çok yüksek (Mandel h, Şekil C.7); 3. ve 4. düzeylerde Grubbs testine göre aykırı (C.3.5)")},
                       {"laboratory": "6", "level": "5", "reason": T("at least one sample may have come from level 4 by mistake; the absolute difference was also suspect (C.3.5)",
                                                                      "en az bir numune yanlışlıkla 4. düzeyden gelmiş olabilir; mutlak fark da şüphelidir (C.3.5)")}]}
c3_expl = [
    T("Form A (Table C.14): nine laboratories, five levels, two results per cell.",
      "Form A (Tablo C.14): dokuz laboratuvar, beş düzey, hücre başına iki sonuç."),
    T("Mandel's h (Figure C.7) shows laboratory 1 much higher than all others at all levels; Mandel's k (Figure C.8) shows rather large within-cell differences for laboratories 6 and 7.",
      "Mandel h (Şekil C.7) 1. laboratuvarın tüm düzeylerde diğerlerinden çok yüksek olduğunu gösterir; Mandel k (Şekil C.8) 6. ve 7. laboratuvarlarda oldukça büyük hücre içi farklar gösterir."),
    T("Cochran's test: at level 4, C = 1.10²/1.8149 = 0.667 (straggler, 0.638 < C ≤ 0.754 for p = 9); at level 5, C = 1.98²/6.1663 = 0.636, just below 0.638.",
      "Cochran testi: 4. düzeyde C = 1,10²/1,8149 = 0,667 (şüpheli, p = 9 için 0,638 < C ≤ 0,754); 5. düzeyde C = 1,98²/6,1663 = 0,636, 0,638'in hemen altında."),
    T("Grubbs' tests (Table C.17): the cell means of laboratory 1 at levels 3 and 4 are outliers (2.50 and 2.47 > 2.387), so the double tests are not applied there (8.3.5.3).",
      "Grubbs testleri (Tablo C.17): 1. laboratuvarın 3. ve 4. düzeylerdeki hücre ortalamaları aykırıdır (2,50 ve 2,47 > 2,387); bu yüzden orada ikili testler uygulanmaz (8.3.5.3)."),
    T("The panel rejects laboratory 1 as an outlying laboratory and the cell of laboratory 6 at level 5; without them Cochran's statistic at level 4 is compared with 0.680 (p = 8) and is no longer a straggler.",
      "Panel 1. laboratuvarı aykırı laboratuvar olarak ve 6. laboratuvarın 5. düzeydeki hücresini reddeder; bunlar olmadan 4. düzeydeki Cochran istatistiği 0,680 (p = 8) ile karşılaştırılır ve artık şüpheli değildir."),
]
c3_levels = {
    "1": {"p": 8, "m": chk(3.94, 0.005), "s_r": chk(0.092, 0.0005), "s_R": chk(0.171, 0.0005),
          "grubbs_low": chk(1.36, 0.005), "grubbs_high": chk(1.95, 0.005), "grubbs_double_low": chk(0.502, 0.0005), "grubbs_double_high": chk(0.356, 0.0005)},
    "2": {"p": 8, "m": chk(8.28, 0.005), "s_r": chk(0.179, 0.0005), "s_R": chk(0.498, 0.0005),
          "grubbs_low": chk(1.57, 0.005), "grubbs_high": chk(1.64, 0.005), "grubbs_double_low": chk(0.540, 0.0005), "grubbs_double_high": chk(0.395, 0.0005)},
    "3": {"p": 8, "m": chk(14.18, 0.005), "s_r": chk(0.127, 0.0005), "s_R": chk(0.400, 0.0005),
          "grubbs_low": chk(0.86, 0.005), "grubbs_high": chk(2.50, 0.005)},
    "4": {"p": 8, "m": chk(15.59, 0.005), "s_r": chk(0.337, 0.0005), "s_R": chk(0.579, 0.0005), "cochran": chk(0.667, 0.0005),
          "grubbs_low": chk(0.91, 0.005), "grubbs_high": chk(2.47, 0.005)},
    "5": {"p": 7, "m": chk(20.41, 0.005), "s_r": chk(0.393, 0.0005), "s_R": chk(0.637, 0.0005), "cochran": chk(0.636, 0.0005),
          "grubbs_low": chk(1.70, 0.005), "grubbs_high": chk(2.10, 0.005), "grubbs_double_low": chk(0.501, 0.0005), "grubbs_double_high": chk(0.318, 0.0005)}}
project("chemistry", "c3-creosote-oil",
  T("Thermometric titration of creosote oil: outlying laboratory and precision relationships (ISO 5725-2 Example C.3)",
    "Kreozot yağının termometrik titrasyonu: aykırı laboratuvar ve kesinlik ilişkileri (ISO 5725-2 Örnek C.3)"),
  T("ISO 5725-2:2019 C.3 with 8.5: nine laboratories at five levels (4 % to 20 %), an outlying laboratory, a rejected cell, and the functional relationships I–IV between the precision and the level (Tables 1–4).",
    "ISO 5725-2:2019 C.3 ve 8.5: beş düzeyde (%4 – %20) dokuz laboratuvar, aykırı bir laboratuvar, reddedilmiş bir hücre ve kesinlik ile düzey arasındaki I–IV fonksiyonel ilişkileri (Tablo 1–4)."),
  c3_expl + [
    T("Table C.18: with p = 8 (p = 7 at level 5) the precision increases with the level: s_r from 0.092 % to 0.393 %, s_R from 0.171 % to 0.637 %.",
      "Tablo C.18: p = 8 (5. düzeyde p = 7) ile kesinlik değerleri düzeyle artar: s_r %0,092'den %0,393'e, s_R %0,171'den %0,637'ye."),
    T("Relationships for s_r (Tables 1–4): I, b = Σ(s/m)/q = 0.019; II, s = 0.030 + 0.0154m after two weighted fits; III, s² = 0.061² + (0.0178m)²; IV, lg s = −1.5065 + 0.77 lg m. For repeatability a straight line through the origin seems adequate (C.3.7).",
      "s_r için ilişkiler (Tablo 1–4): I, b = Σ(s/m)/q = 0,019; II, iki ağırlıklı uydurmadan sonra s = 0,030 + 0,0154m; III, s² = 0,061² + (0,0178m)²; IV, lg s = −1,5065 + 0,77 lg m. Tekrarlanabilirlik için orijinden geçen bir doğru yeterli görünmektedir (C.3.7)."),
  ],
  "Annex C.3 (Tables C.14–C.18) and 8.5 (Tables 1–4)", ["creosote-oil", "outliers", "precision-relationships"],
  [T("The printed slope of relationship IV in Table 4, d = 0.772, is not consistent with the fitted values in the same table (0.089, 0.158, 0.239, 0.257, 0.316), which follow from the regression of the tabulated logarithms, d = 0.769 (research log F49). The same values are in ISO 5725-2:1994.",
     "Tablo 4'te basılı IV. ilişki eğimi d = 0,772, aynı tablodaki uydurulmuş değerlerle (0,089; 0,158; 0,239; 0,257; 0,316) tutarlı değildir; bu değerler tablodaki logaritmaların regresyonundan, d = 0,769'dan çıkar (araştırma günlüğü F49). Aynı değerler ISO 5725-2:1994'te de vardır."),
   T("Table 2 rounds the slope of the second fit to 0.0154; from the tabulated m and s it is 0.01554. The program fits the unrounded s_r of Table C.18.",
     "Tablo 2 ikinci uydurmanın eğimini 0,0154'e yuvarlar; tablodaki m ve s değerlerinden 0,01554 çıkar. Program Tablo C.18'in yuvarlanmamış s_r değerlerine uydurur.")],
  c3_body,
  {"levels": c3_levels,
   "relationships": [
     {"of": "s_r", "relationship": "I", "params": [chk(0.019, 0.0005)]},
     {"of": "s_r", "relationship": "II", "params": [chk(0.030, 0.001), chk(0.0154, 0.0002)]},
     {"of": "s_r", "relationship": "III", "params": [chk(0.0612, 0.001), chk(0.0178, 0.0001)]},
     {"of": "s_r", "relationship": "IV", "params": [chk(-1.5065, 0.001)]}]},
  T("ISO 5725-2:2019, Tables C.17 and C.18, Tables 1–4, and C.3.5.", "ISO 5725-2:2019, Tablo C.17 ve C.18, Tablo 1–4 ve C.3.5."))

project("chemistry", "c3-creosote-oil-reml",
  T("Thermometric titration of creosote oil: REML estimates (ISO 5725-2 Example C.3)",
    "Kreozot yağının termometrik titrasyonu: REML tahminleri (ISO 5725-2 Örnek C.3)"),
  T("ISO 5725-2:2019 C.3.8: the retained data of Example C.3 evaluated by restricted maximum likelihood (Annex B.2).",
    "ISO 5725-2:2019 C.3.8: Örnek C.3'ün tutulan verilerinin kısıtlı en çok olabilirlikle (Ek B.2) değerlendirilmesi."),
  c3_expl + [
    T("The REML estimates of Table C.19 are the same as Table C.18 to the printed digits.",
      "Tablo C.19'un REML tahminleri basılı basamaklarda Tablo C.18 ile aynıdır."),
  ],
  "Annex C.3.8 (Table C.19), Annex B.2", ["creosote-oil", "outliers", "reml"], [],
  dict(c3_body, method="reml", relationships=[]),
  {"levels": {k: {"p": v["p"], "m": v["m"], "s_r": v["s_r"], "s_R": v["s_R"]} for k, v in c3_levels.items()}},
  T("ISO 5725-2:2019, Table C.19.", "ISO 5725-2:2019, Tablo C.19."))
