"""Writes the ISO 5725-3:2023 Annex J library projects."""
import json
from pathlib import Path
DOC = "ISO 5725-3:2023, Accuracy (trueness and precision) of measurement methods and results — Part 3: Intermediate precision and alternative designs for collaborative studies"
URL = "https://www.iso.org/standard/77864.html"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def project(path, pid, title, desc, expl, sec, tags, notes, body, expected):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "chemistry",
                      "sources": [{"document": DOC, "section": sec, "url": URL}], "tags": ["iso5725-3", "intermediate-precision"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p["topdown"] = body; p["reporting"] = {"significant_digits": 3}; p["expected"] = expected
    Path(path).write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

carbon = [(0.130,0.127),(0.140,0.132),(0.078,0.080),(0.110,0.113),(0.126,0.128),(0.036,0.032),(0.050,0.047),(0.143,0.140),(0.091,0.089),(0.040,0.030),(0.110,0.113),(0.142,0.145),(0.143,0.150),(0.169,0.165),(0.169,0.173),(0.149,0.144),(0.044,0.044),(0.127,0.122),(0.050,0.048),(0.042,0.146),(0.150,0.145),(0.135,0.133),(0.044,0.045),(0.100,0.161),(0.132,0.131),(0.047,0.045),(0.168,0.165),(0.092,0.088),(0.041,0.043)]
kept = [list(x) for i, x in enumerate(carbon) if i not in (19, 23)]
project("library/chemistry/iso5725-3-j1-carbon-steel.json", "iso5725-3-j1-carbon-steel",
  T("Carbon in steel: [time + operator]-different intermediate precision (ISO 5725-3 J.1)", "Çelikte karbon: [zaman + operatör] farklı ara kesinlik (ISO 5725-3 J.1)"),
  T("ISO 5725-3:2023 J.1: one routine sample a day re-analysed the next day by another analyst, 29 pairs in a month; s_I(TO) from the pairs, Formula (5).",
    "ISO 5725-3:2023 J.1: her gün bir rutin numune ertesi gün başka bir analist tarafından yeniden analiz edilir, bir ayda 29 çift; çiftlerden s_I(TO), Formül (5)."),
  [T("Each pair (y_j1, y_j2) is the same sample measured on two days by two analysts, so the differences contain the time and operator effects as well as repeatability (6.2.2).",
     "Her çift (y_j1, y_j2) aynı numunenin iki günde iki analist tarafından ölçülmesidir; bu nedenle farklar tekrarlanabilirliğin yanında zaman ve operatör etkilerini de içerir (6.2.2)."),
   T("Pairs 20 (0.042/0.146 %) and 24 (0.100/0.161 %) are removed: their differences, probably recording errors, are outliers by Cochran's test (C = 0.72 for pair 20 among all 29, C = 0.89 for pair 24 after removing 20).",
     "20 (%0,042/0,146) ve 24 (%0,100/0,161) numaralı çiftler çıkarılır: muhtemelen kayıt hatası olan farkları Cochran testine göre aykırıdır (29 çift arasında 20 için C = 0,72; 20 çıkarıldıktan sonra 24 için C = 0,89)."),
   T("s_I(TO) = √(Σw_j²/(2 × 27)) = 2.87 × 10⁻³ % (mass fraction), Formula (5), with 27 degrees of freedom.",
     "s_I(TO) = √(Σw_j²/(2 × 27)) = 2,87 × 10⁻³ % (kütle kesri), Formül (5), 27 serbestlik derecesiyle.")],
  "Annex J.1, Table J.1", ["duplicates", "steel"],
  [T("The 27 retained pairs are stored; the two removed pairs are given in the explanation. The Cochran statistics quoted are computed by the program's authors from Table J.1 (ISO 5725-3 gives only the conclusion).",
     "Korunan 27 çift saklanmıştır; çıkarılan iki çift açıklamada verilmiştir. Belirtilen Cochran istatistikleri Tablo J.1'den hesaplanmıştır (ISO 5725-3 yalnızca sonucu verir).")],
  {"quantity": "w(C)", "unit": "% m/m", "levels": [0.1],
   "precision": [{"name": "s_I(TO)", "data": {"type": "duplicates", "pairs": kept}}]},
  {"topdown": {"components": {"s_I(TO)": chk(2.87e-3, 0.005e-3)}},
   "reference": T("ISO 5725-3:2023 J.1.2 (s_I(TO) = 2.87 × 10⁻³ %).", "ISO 5725-3:2023 J.1.2 (s_I(TO) = 2,87 × 10⁻³ %).")})

rows = {int(k): v for k, v in json.load(open("/tmp/claude-0/-home-user-uncertainty/ce684ba1-ace2-51bd-a3d6-42372cfaa0dd/scratchpad/vanadium.json")).items()}
lv1 = [rows[l][0:3] for l in sorted(rows) if l != 20]
project("library/chemistry/iso5725-3-j2-vanadium-steel.json", "iso5725-3-j2-vanadium-steel",
  T("Vanadium in steel: staggered-nested design, level 1 (ISO 5725-3 J.2)", "Çelikte vanadyum: kademeli iç içe tasarım, düzey 1 (ISO 5725-3 J.2)"),
  T("ISO 5725-3:2023 J.2: 20 laboratories, two results on one day and one on the next; analysis of variance gives s_r, the time-different intermediate precision s_I(T) and s_R.",
    "ISO 5725-3:2023 J.2: 20 laboratuvar, bir günde iki ve ertesi gün bir sonuç; varyans analizi s_r'yi, zaman farklı ara kesinlik s_I(T)'yi ve s_R'yi verir."),
  [T("Each laboratory reports y₁, y₂ (repeatability conditions, day 1) and y₃ (day 2). Laboratory 20 is an outlier at level 1 (Cochran) and is removed.",
     "Her laboratuvar y₁, y₂ (tekrarlanabilirlik koşulları, 1. gün) ve y₃ (2. gün) bildirir. 20 numaralı laboratuvar düzey 1'de aykırıdır (Cochran) ve çıkarılır."),
   T("Annex C.1: w(1) = |y₁ − y₂|, w(2) = |(y₁ + y₂)/2 − y₃|; SS₀ = 3Σ(ȳᵢ − ȳ)², SS₁ = (2/3)Σw(2)², SS_e = (1/2)Σw(1)² with 18, 19 and 19 degrees of freedom (Table J.4).",
     "Ek C.1: w(1) = |y₁ − y₂|, w(2) = |(y₁ + y₂)/2 − y₃|; 18, 19 ve 19 serbestlik dereceli SS₀ = 3Σ(ȳᵢ − ȳ)², SS₁ = (2/3)Σw(2)², SS_e = (1/2)Σw(1)² (Tablo J.4)."),
   T("From the expected mean squares, s_r² = MS_e = 0.145, s₍₁₎² = (3/4)(MS₁ − MS_e) = 0.218 and s₀² = (MS₀ − MS_e − (5/3)s₍₁₎²)/3 = 0.278 (× 10⁻⁶): s_r = 0.381, s_I(T) = 0.603 and s_R = 0.801 (× 10⁻³ %).",
     "Beklenen kareler ortalamalarından s_r² = MS_e = 0,145, s₍₁₎² = (3/4)(MS₁ − MS_e) = 0,218 ve s₀² = (MS₀ − MS_e − (5/3)s₍₁₎²)/3 = 0,278 (× 10⁻⁶): s_r = 0,381, s_I(T) = 0,603 ve s_R = 0,801 (× 10⁻³ %).")],
  "Annex J.2, Tables J.2–J.5", ["staggered-nested", "anova", "steel"],
  [T("Levels 1 and 2 of Table J.5 are reproduced exactly from Table J.2; levels 3, 4 and 5, and s_R at level 6, are not reproduced from the printed data with the stated outlier exclusions (research log F40). The column 'Average (%)' of Table J.5 carries a spurious factor 10⁻³.",
     "Tablo J.5'in 1. ve 2. düzeyleri Tablo J.2'den tam olarak yeniden üretilir; 3., 4. ve 5. düzeyler ile 6. düzeydeki s_R, belirtilen aykırı değer çıkarmalarıyla basılı verilerden yeniden üretilemez (araştırma günlüğü F40). Tablo J.5'teki 'Average (%)' sütunu fazladan bir 10⁻³ çarpanı taşır.")],
  {"quantity": "w(V)", "unit": "% m/m", "levels": [0.0098],
   "precision": [{"name": "s_I(T)", "data": {"type": "staggered_nested", "results": lv1, "estimate": "intermediate"}}]},
  {"topdown": {"components": {"s_I(T)": chk(0.603e-3, 0.0005e-3)}},
   "reference": T("ISO 5725-3:2023 Tables J.4 and J.5, level 1 (s_r = 0.381, s_I(T) = 0.603, s_R = 0.801 × 10⁻³ %).", "ISO 5725-3:2023 Tablo J.4 ve J.5, düzey 1 (s_r = 0,381, s_I(T) = 0,603, s_R = 0,801 × 10⁻³ %).")})
