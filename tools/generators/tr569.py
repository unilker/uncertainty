"""Writes the Nordtest TR 569 ed. 6 library projects (run from the repository root)."""
import json, statistics
from pathlib import Path

DOC = "Nordtest TR 569 ed. 6 (2026), Handbook of Internal Quality Control, by B. Magnusson, H. Hovind, M. Krysell, U. Lund, I. Mäkinen and E. Sahlin"
URL = "https://www.nordtest.info/wp/wp-content/uploads/2026/03/NT_TR_569_ed6_Eng.pdf"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}

def project(folder, name, title, desc, expl, sec, tags, chart, expected, ref, notes=None):
    pid = f"tr569-{name}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": folder,
                      "sources": [{"document": DOC, "section": sec, "url": URL}],
                      "tags": ["nordtest-tr569", "internal-quality-control", "control-chart"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p["control_chart"] = chart
    p["expected"] = {"control_chart": expected, "reference": ref}
    Path(f"library/{folder}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

RULES_X = T("Statistical control limits for an X-chart: warning limits CL ± 2s and action limits CL ± 3s (Chapter 7).",
            "X-grafiği için istatistiksel kontrol sınırları: uyarı sınırları CL ± 2s, eylem sınırları CL ± 3s (Bölüm 7).")
TARGET_X = T("Target control limits: s_target from the requirement replaces the s of the method; warning limits CL ± 2s_target, action limits CL ± 3s_target (Chapter 7).",
             "Hedef kontrol sınırları: yöntemin s'si yerine gereklilikten s_hedef kullanılır; uyarı sınırları CL ± 2s_hedef, eylem sınırları CL ± 3s_hedef (Bölüm 7).")
RANGE = T("Range chart of duplicates: central line the mean range, s = mean range/d₂ with d₂ = 1.128 (Equation 4), upper warning limit D_WL·s = 2.833 s and upper action limit D_AL·s = 3.686 s (Table 4, from ISO 8258).",
          "Paralel tayinlerin açıklık grafiği: merkez çizgisi ortalama açıklık, s = ortalama açıklık/d₂, d₂ = 1,128 (Eşitlik 4); üst uyarı sınırı D_WL·s = 2,833 s ve üst eylem sınırı D_AL·s = 3,686 s (Tablo 4, ISO 8258'den).")

# ------------------------------------------------------------------ Table 1 / Figure 1: zinc
zn = [64.5, 66.3, 61.1, 59.7, 57.4, 56.2, 58.4, 58.2, 63.0, 59.5,
      56.0, 59.4, 60.2, 62.9, 60.5, 60.8, 61.5, 58.5, 58.9, 60.5,
      61.2, 57.8, 63.4, 60.2, 61.5, 62.3, 60.5, 61.7, 64.0, 62.7,
      61.0, 65.4, 60.0, 59.2, 57.0, 62.5, 57.7, 56.2, 62.9, 62.5,
      56.5, 60.2, 58.2, 56.5, 64.7, 54.5, 60.5, 59.5, 61.6, 60.8,
      58.7, 54.4, 62.2, 59.0, 60.3, 60.8, 59.5, 60.0, 61.8, 63.8]
m, s = statistics.mean(zn), statistics.stdev(zn)
project("chemistry", "table1-zinc",
    T("Zinc in water: X-chart from 60 control values (Nordtest TR 569 Table 1)", "Sudaki çinko: 60 kontrol değerinden X-grafiği (Nordtest TR 569 Tablo 1)"),
    T("Sixty control values of a 60.0 µg/L zinc solution (Table 1), plotted in Figure 1. Statistical limits from the values themselves; daily interpretation of every run.",
      "60,0 µg/L çinko çözeltisinin altmış kontrol değeri (Tablo 1), Şekil 1'de çizilmiştir. Sınırlar değerlerin kendisinden istatistiksel olarak belirlenir; her seri günlük olarak değerlendirilir."),
    [T(f"Mean and standard deviation of the 60 values (Equations 1, 2): x̄ = {m:.3f} µg/L, s = {s:.3f} µg/L; the central line is the mean.",
       "60 değerin ortalaması ve standart sapması (Eşitlik 1, 2): x̄ = 60,278 µg/L, s = 2,598 µg/L; merkez çizgisi ortalamadır."),
     RULES_X,
     T("Daily interpretation (Chapter 9): three values lie between the warning and action limits (66.3, 54.5 and 54.4 µg/L; 65.4 µg/L is just inside the upper warning limit of 65.47 µg/L), none of them preceded by another on the same side within two runs; no value is out of control, as in Figure 1.",
       "Günlük değerlendirme (Bölüm 9): üç değer uyarı ile eylem sınırları arasındadır (66,3, 54,5 ve 54,4 µg/L; 65,4 µg/L, 65,47 µg/L üst uyarı sınırının hemen içindedir) ve hiçbirinden önceki iki seride aynı tarafta başka bir değer yoktur; Şekil 1'deki gibi hiçbir değer kontrol dışı değildir."),
     T("The mean does not differ significantly from the nominal 60.0 µg/L: t = |60.278 − 60.0|·√60/2.598 = 0.83 < 2.00 (Equation 6).",
       "Ortalama nominal 60,0 µg/L'den anlamlı olarak farklı değildir: t = |60,278 − 60,0|·√60/2,598 = 0,83 < 2,00 (Eşitlik 6).")],
    "Chapter 2, Table 1 and Figure 1; Chapters 7 and 9", ["zinc", "water", "x-chart", "raw-data"],
    {"quantity": "Zn, control solution 60.0 µg/L", "unit": "µg/L", "values": zn, "reference": 60.0},
    {"mean": chk(60.278, 5e-4), "sd": chk(2.598, 5e-4), "center": chk(60.278, 5e-4),
     "lower_warning": chk(55.08, 0.005), "upper_warning": chk(65.47, 0.005),
     "lower_action": chk(52.48, 0.005), "upper_action": chk(68.07, 0.005),
     "outside_warning": 3, "out_of_control": 0},
    T("Values of Table 1; mean, s and limits computed from them (TR 569 prints the chart, Figure 1, without the numbers); the zones of Figure 1 agree.",
      "Tablo 1 değerleri; ortalama, s ve sınırlar bunlardan hesaplanmıştır (TR 569 grafiği, Şekil 1, sayılar olmadan verir); Şekil 1'deki bölgeler uyumludur."))

# ------------------------------------------------------------------ Chapter 7, X-chart cases
cases = [
    ("1", "statistical", {"basis": "statistical", "sd_percent": 4.0}, {"mean": 59.2}, (54.5, 63.9, 52.1, 66.3),
     T("Case 1: statistical limits and a mean central line, s_Rw = 4 % of 59.2 µg/L.", "Durum 1: istatistiksel sınırlar ve ortalama merkez çizgisi, 59,2 µg/L'nin %4'ü s_Rw.")),
    ("2", "target-mean", {"basis": "target", "sd_percent": 5.0}, {"mean": 59.2}, (53.3, 65.1, 50.3, 68.1),
     T("Case 2: target limits from a required s_Rw of 5 %, mean central line 59.2 µg/L.", "Durum 2: gerekli %5 s_Rw'den hedef sınırlar, ortalama merkez çizgisi 59,2 µg/L.")),
    ("3", "target-reference", {"basis": "target", "sd_percent": 5.0}, {"reference": 60.0}, (54.0, 66.0, 51.0, 69.0),
     T("Case 3: target limits of 5 % around the reference value 60.0 µg/L.", "Durum 3: 60,0 µg/L referans değeri çevresinde %5 hedef sınırlar.")),
]
for no, slug, lim, cl, (lw, uw, la, ua), desc in cases:
    project("chemistry", f"ch7-case{no}-{slug}",
        T(f"Setting the limits of an X-chart, case {no} (Nordtest TR 569 Chapter 7)", f"X-grafiğinin sınırlarının belirlenmesi, durum {no} (Nordtest TR 569 Bölüm 7)"),
        desc,
        [RULES_X if lim["basis"] == "statistical" else TARGET_X,
         T("The relative standard deviation applies to the central line: s = p·CL/100.", "Göreli standart sapma merkez çizgisine uygulanır: s = p·CL/100.")],
        "Chapter 7, Setting the control limits and the central line in X-charts, case " + no, ["x-chart", "limits"],
        {"quantity": "control sample", "unit": "µg/L", "central_line": cl, "limits": lim},
        {"lower_warning": chk(lw, 0.05), "upper_warning": chk(uw, 0.05), "lower_action": chk(la, 0.05), "upper_action": chk(ua, 0.05)},
        T("TR 569 Chapter 7, values to 0.1 µg/L.", "TR 569 Bölüm 7, 0,1 µg/L'ye yuvarlanmış değerler."))

# ------------------------------------------------------------------ Chapter 7, range-chart cases
project("chemistry", "ch7-range-case1-statistical",
    T("Statistical limits of an r%-chart for duplicates (Nordtest TR 569 Chapter 7)", "Paralel tayinler için r%-grafiğinin istatistiksel sınırları (Nordtest TR 569 Bölüm 7)"),
    T("Case 1 for range charts: pooled relative standard deviation 0.356 % from duplicates.", "Açıklık grafikleri için durum 1: paralel tayinlerden %0,356 birleştirilmiş göreli standart sapma."),
    [RANGE, T("Upper warning limit 2.833·0.356 % = 1.0 %, upper action limit 3.686·0.356 % = 1.3 %.", "Üst uyarı sınırı 2,833·%0,356 = %1,0, üst eylem sınırı 3,686·%0,356 = %1,3.")],
    "Chapter 7, Setting the control limit in R or r%-charts, case 1", ["r-chart", "relative-range", "limits"],
    {"quantity": "duplicates of test samples", "chart": "relative_range", "replicates": 2,
     "limits": {"basis": "statistical", "sd": 0.356}},
    {"upper_warning": chk(1.0, 0.05), "upper_action": chk(1.3, 0.05)},
    T("TR 569 Chapter 7, case 1, values to 0.1 %. The text prints the warning limit as 3·0.356 = 1.0 %; the factor of Table 4 is 2.83 (research log F52). The central line is not given; the program uses the expected range d₂·s = 0.40 %.",
      "TR 569 Bölüm 7, durum 1, %0,1'e yuvarlanmış değerler. Metin uyarı sınırını 3·0,356 = %1,0 olarak yazar; Tablo 4'ün katsayısı 2,83'tür (araştırma günlüğü F52). Merkez çizgisi verilmemiştir; program beklenen açıklık d₂·s = %0,40 değerini kullanır."),
    notes=[T("Research log F52: in Chapter 7, case 1 for range charts, the warning limit is printed as 3·0.356 = 1.0 %; 3·0.356 = 1.07, the factor of Table 4 is 2.83 and 2.83·0.356 = 1.01.",
             "Araştırma günlüğü F52: Bölüm 7'de açıklık grafikleri için durum 1'de uyarı sınırı 3·0,356 = %1,0 olarak basılmıştır; 3·0,356 = 1,07'dir, Tablo 4'ün katsayısı 2,83'tür ve 2,83·0,356 = 1,01'dir.")])
project("chemistry", "ch7-range-case2-target",
    T("Target limits of an r%-chart from a repeatability limit (Nordtest TR 569 Chapter 7)", "Tekrarlanabilirlik sınırından r%-grafiğinin hedef sınırları (Nordtest TR 569 Bölüm 7)"),
    T("Case 2 for range charts: the standard method gives a repeatability limit r = 1 %.", "Açıklık grafikleri için durum 2: standart yöntem r = %1 tekrarlanabilirlik sınırı verir."),
    [T("s_r = r/2.8 = 0.357 % (the factor 2.8 ≈ 2√2 comes from the difference of two results, footnote 12).", "s_r = r/2,8 = %0,357 (2,8 ≈ 2√2 katsayısı iki sonucun farkından gelir, dipnot 12)."),
     RANGE, T("Upper warning limit 2.833·0.357 % = 1.0 %, upper action limit 3.686·0.357 % = 1.3 %; without data the central line is the expected range d₂·s = 0.40 %.",
              "Üst uyarı sınırı 2,833·%0,357 = %1,0, üst eylem sınırı 3,686·%0,357 = %1,3; veri yokken merkez çizgisi beklenen açıklık d₂·s = %0,40'tır.")],
    "Chapter 7, Setting the control limit in R or r%-charts, case 2", ["r-chart", "relative-range", "limits", "repeatability-limit"],
    {"quantity": "duplicates of test samples", "chart": "relative_range", "replicates": 2,
     "limits": {"basis": "target", "repeatability_limit": 1.0}},
    {"sd": chk(0.357, 5e-4), "upper_warning": chk(1.0, 0.05), "upper_action": chk(1.3, 0.05)},
    T("TR 569 Chapter 7, case 2.", "TR 569 Bölüm 7, durum 2."))

# ------------------------------------------------------------------ Examples 1–2: XRF, target limits
project("chemistry", "ex1-nickel-steel-xrf",
    T("Nickel in low-alloy steel by XRF: target limits from the required uncertainty (Nordtest TR 569 Example 1)", "XRF ile düşük alaşımlı çelikte nikel: gerekli belirsizlikten hedef sınırlar (Nordtest TR 569 Örnek 1)"),
    T("X-chart of a steel test sample, mean over one year 4.58 weight-% with s = 0.026 weight-%; required expanded uncertainty U = 4 % (relative).",
      "Bir çelik numunenin X-grafiği; bir yıllık ortalama ağırlıkça %4,58, s = ağırlıkça %0,026; gerekli genişletilmiş belirsizlik U = %4 (göreli)."),
    [T("u_c = U/2 = 2 %; the requirement on s_Rw is half of u_c: s_Rw = U/4 = 1 % (relative) = 0.0458 weight-%.", "u_c = U/2 = %2; s_Rw için gereklilik u_c'nin yarısıdır: s_Rw = U/4 = %1 (göreli) = ağırlıkça %0,0458."),
     TARGET_X,
     T("The method performs with s = 0.026 weight-%, below the target: the target limits are wider than statistical limits and give fewer false alarms while the results remain fit for purpose.",
       "Yöntem hedefin altında s = ağırlıkça %0,026 ile çalışır: hedef sınırlar istatistiksel sınırlardan geniştir ve sonuçlar amaca uygun kalırken daha az yanlış alarm verir.")],
    "Chapter 14, Example 1", ["nickel", "steel", "xrf", "x-chart", "target-limits"],
    {"quantity": "Ni, steel control sample", "unit": "weight-%", "central_line": {"mean": 4.58},
     "limits": {"basis": "target", "expanded_percent": 4.0}},
    {"sd": chk(0.0458, 5e-5), "lower_warning": chk(4.49, 0.005), "upper_warning": chk(4.67, 0.005),
     "lower_action": chk(4.44, 0.005), "upper_action": chk(4.72, 0.005)},
    T("TR 569 Example 1.", "TR 569 Örnek 1."))
project("chemistry", "ex2-cobalt-steel-xrf",
    T("Cobalt in low-alloy steel by XRF: target limits from the limit of quantification (Nordtest TR 569 Example 2)", "XRF ile düşük alaşımlı çelikte kobalt: tayin sınırından hedef sınırlar (Nordtest TR 569 Örnek 2)"),
    T("X-chart at low concentration, mean 0.0768 weight-% with s = 0.00063 weight-%; the required LOQ of 0.01 weight-% is 6 to 10 times s, so s_target = 0.001 weight-%.",
      "Düşük derişimde X-grafiği, ortalama ağırlıkça %0,0768, s = ağırlıkça %0,00063; gerekli 0,01 ağırlıkça % LOQ, s'nin 6–10 katıdır, bu nedenle s_hedef = ağırlıkça %0,001."),
    [TARGET_X, T("The control sample is at eight times the LOQ, the concentration of interest.", "Kontrol numunesi ilgilenilen derişim olan LOQ'nun sekiz katındadır.")],
    "Chapter 14, Example 2", ["cobalt", "steel", "xrf", "x-chart", "target-limits", "loq"],
    {"quantity": "Co, steel control sample", "unit": "weight-%", "central_line": {"mean": 0.0768},
     "limits": {"basis": "target", "sd": 0.001}},
    {"lower_warning": chk(0.0748, 5e-6), "upper_warning": chk(0.0788, 5e-6), "lower_action": chk(0.0738, 5e-6), "upper_action": chk(0.0798, 5e-6)},
    T("TR 569 Example 2.", "TR 569 Örnek 2."))

# ------------------------------------------------------------------ Example 3: ammonium, X and R
project("chemistry", "ex3-ammonium-x",
    T("Ammonium nitrogen in water: X-chart of a standard solution (Nordtest TR 569 Example 3)", "Suda amonyum azotu: standart çözeltinin X-grafiği (Nordtest TR 569 Örnek 3)"),
    T("Indophenol blue method, 20 µg/L control solution prepared from (NH₄)₂SO₄, independent of the NH₄Cl calibration standards; the mean of duplicates is plotted.",
      "İndofenol mavisi yöntemi; NH₄Cl kalibrasyon standartlarından bağımsız olarak (NH₄)₂SO₄'ten hazırlanmış 20 µg/L kontrol çözeltisi; paralel tayinlerin ortalaması çizilir."),
    [T("Central line the mean x̄ = 19.99 µg/L; s = 0.521 µg/L of the means.", "Merkez çizgisi ortalama x̄ = 19,99 µg/L; ortalamaların s'si 0,521 µg/L."), RULES_X],
    "Chapter 14, Example 3", ["ammonium", "water", "x-chart"],
    {"quantity": "N-NH₄, standard solution 20 µg/L", "unit": "µg/L", "central_line": {"mean": 19.99}, "replicates": 2,
     "limits": {"basis": "statistical", "sd": 0.521}},
    {"lower_warning": chk(18.95, 0.005), "upper_warning": chk(21.03, 0.005), "lower_action": chk(18.43, 0.005), "upper_action": chk(21.55, 0.005)},
    T("TR 569 Example 3.", "TR 569 Örnek 3."))
project("chemistry", "ex3-ammonium-r",
    T("Ammonium nitrogen in water: R-chart of duplicate test samples (Nordtest TR 569 Example 3)", "Suda amonyum azotu: paralel numunelerin R-grafiği (Nordtest TR 569 Örnek 3)"),
    T("One test sample below 100 µg/L analysed in duplicate in each run; the range of the duplicates is plotted.", "Her seride 100 µg/L'nin altındaki bir numune paralel analiz edilir; paralel tayinlerin açıklığı çizilir."),
    [RANGE, T("Mean range 0.559 µg/L, s = 0.559/1.128 = 0.496 µg/L; warning limit 2.83·0.496 = 1.40 µg/L, action limit 3.69·0.496 = 1.83 µg/L.",
              "Ortalama açıklık 0,559 µg/L, s = 0,559/1,128 = 0,496 µg/L; uyarı sınırı 2,83·0,496 = 1,40 µg/L, eylem sınırı 3,69·0,496 = 1,83 µg/L.")],
    "Chapter 14, Example 3", ["ammonium", "water", "r-chart", "duplicates"],
    {"quantity": "N-NH₄, duplicates of test samples", "unit": "µg/L", "chart": "range", "replicates": 2,
     "central_line": {"mean": 0.559}},
    {"sd": chk(0.496, 1e-3), "center": chk(0.559, 1e-9), "upper_warning": chk(1.40, 0.005), "upper_action": chk(1.83, 0.005)},
    T("TR 569 Example 3. The example calls s the pooled repeatability standard deviation; its value is the mean range divided by 1.128.",
      "TR 569 Örnek 3. Örnek s'ye birleştirilmiş tekrarlanabilirlik standart sapması der; değeri ortalama açıklığın 1,128'e bölümüdür."))

# ------------------------------------------------------------------ Example 4: lead
project("chemistry", "ex4-lead-water-icpms",
    T("Lead in water by ICP-MS: X-chart of an in-house lake water (Nordtest TR 569 Example 4)", "ICP-MS ile suda kurşun: laboratuvar içi göl suyunun X-grafiği (Nordtest TR 569 Örnek 4)"),
    T("Low lead concentration (0.29 µg/L) in a preserved lake water, analysed once in each run; statistical limits.", "Korunmuş göl suyunda düşük kurşun derişimi (0,29 µg/L), her seride bir kez analiz edilir; istatistiksel sınırlar."),
    [RULES_X, T("Twelve consecutive values above the central line: out of statistical control, but not out of control (Chapter 9); the results can be reported.",
                "Merkez çizgisinin üstünde art arda on iki değer: istatistiksel kontrol dışı, ancak kontrol dışı değil (Bölüm 9); sonuçlar raporlanabilir.")],
    "Chapter 14, Example 4", ["lead", "water", "icp-ms", "x-chart"],
    {"quantity": "Pb, in-house lake water", "unit": "µg/L", "central_line": {"mean": 0.294}, "limits": {"basis": "statistical", "sd": 0.008}},
    {"lower_warning": chk(0.278, 5e-4), "upper_warning": chk(0.310, 5e-4), "lower_action": chk(0.270, 5e-4), "upper_action": chk(0.318, 5e-4)},
    T("TR 569 Example 4.", "TR 569 Örnek 4."))

# ------------------------------------------------------------------ Example 5: arsenic, action limits only
project("chemistry", "ex5-arsenic-crm-tolerance",
    T("Arsenic in a fish CRM by ICP-MS: chart with action limits only at ± 15 % (Nordtest TR 569 Example 5)", "ICP-MS ile balık CRM'sinde arsenik: yalnızca ± %15 eylem sınırlı grafik (Nordtest TR 569 Örnek 5)"),
    T("CRM NRC DORM-2 (dogfish muscle), 18.0 µg/g arsenic, analysed once in each run; the laboratory accepts deviations of up to 15 % from the reference value.",
      "CRM NRC DORM-2 (köpek balığı kası), 18,0 µg/g arsenik, her seride bir kez analiz edilir; laboratuvar referans değerden %15'e kadar sapmayı kabul eder."),
    [T("Central line the reference value; action limits CL ± tolerance = 18.0 ± 2.7 µg/g; no warning limits (Chapters 5 and 7).", "Merkez çizgisi referans değer; eylem sınırları CL ± tolerans = 18,0 ± 2,7 µg/g; uyarı sınırı yok (Bölüm 5 ve 7)."),
     T("Without warning limits the rule two out of three is not available, so the chart is less sensitive to a drift of the mean.", "Uyarı sınırları olmadan üçte iki kuralı uygulanamaz; bu nedenle grafik ortalamadaki kaymaya daha az duyarlıdır.")],
    "Chapter 14, Example 5", ["arsenic", "crm", "icp-ms", "action-limits-only", "tolerance"],
    {"quantity": "As, CRM DORM-2", "unit": "µg/g", "central_line": {"reference": 18.0}, "limits": {"basis": "tolerance", "tolerance_percent": 15.0}},
    {"lower_action": chk(15.3, 1e-9), "upper_action": chk(20.7, 1e-9)},
    T("TR 569 Example 5.", "TR 569 Örnek 5."))

# ------------------------------------------------------------------ Example 6: phosphorus, r%
project("chemistry", "ex6-phosphorus-relative-range",
    T("Total phosphorus in water: r%-chart of duplicates (Nordtest TR 569 Example 6)", "Suda toplam fosfor: paralel tayinlerin r%-grafiği (Nordtest TR 569 Örnek 6)"),
    T("Test samples above 10 µg/L analysed in duplicate; the absolute difference as per cent of the mean is plotted, since the relative standard deviation is about constant well above the LOQ.",
      "10 µg/L'nin üzerindeki numuneler paralel analiz edilir; LOQ'nun çok üzerinde göreli standart sapma yaklaşık sabit olduğundan farkın ortalamaya yüzde oranı çizilir."),
    [RANGE, T("Mean r% 1.88 %, CV = 1.88/1.128 = 1.67 %; warning limit 2.83·1.67 = 4.73 %, action limit 3.69·1.67 = 6.16 %.", "Ortalama r% %1,88, CV = 1,88/1,128 = %1,67; uyarı sınırı 2,83·1,67 = %4,73, eylem sınırı 3,69·1,67 = %6,16.")],
    "Chapter 14, Example 6", ["phosphorus", "water", "relative-range", "duplicates"],
    {"quantity": "total P, duplicates of test samples", "chart": "relative_range", "replicates": 2, "central_line": {"mean": 1.88},
     "limits": {"basis": "statistical", "sd": 1.67}},
    {"sd": chk(1.67, 1e-12), "upper_warning": chk(4.73, 0.005), "upper_action": chk(6.16, 0.005)},
    T("TR 569 Example 6. The example rounds CV = 1.88/1.128 = 1.667 % to 1.67 % before multiplying by the factors; with the unrounded value the limits would be 4.72 % and 6.14 %.",
      "TR 569 Örnek 6. Örnek CV = 1,88/1,128 = %1,667 değerini katsayılarla çarpmadan önce %1,67'ye yuvarlar; yuvarlanmamış değerle sınırlar %4,72 ve %6,14 olurdu."))

# ------------------------------------------------------------------ Example 7: beta-HCH
project("chemistry", "ex7-beta-hch-crm-target",
    T("β-HCH in cod liver oil by GC-ECD: target limits around the reference value (Nordtest TR 569 Example 7)", "GC-ECD ile morina karaciğeri yağında β-HCH: referans değer çevresinde hedef sınırlar (Nordtest TR 569 Örnek 7)"),
    T("CRM BCR 598, 16 µg/kg β-hexachlorocyclohexane, analysed once in each run; target s of 15 %.", "CRM BCR 598, 16 µg/kg β-heksaklorosikloheksan, her seride bir kez analiz edilir; %15 hedef s."),
    [TARGET_X, T("s_target = 0.15·16.0 = 2.4 µg/kg; a trend above the central line ended in two of three values above the warning limit, an out-of-control situation (Example 7, comment).",
                 "s_hedef = 0,15·16,0 = 2,4 µg/kg; merkez çizgisinin üzerindeki bir eğilim üç değerden ikisinin uyarı sınırının üstünde olmasıyla, yani kontrol dışı bir durumla sonuçlanmıştır (Örnek 7, yorum).")],
    "Chapter 14, Example 7", ["beta-hch", "pesticide", "crm", "gc", "target-limits"],
    {"quantity": "β-HCH, CRM BCR 598", "unit": "µg/kg", "central_line": {"reference": 16.0}, "limits": {"basis": "target", "sd_percent": 15.0}},
    {"sd": chk(2.4, 1e-9), "lower_warning": chk(11.2, 1e-9), "upper_warning": chk(20.8, 1e-9), "lower_action": chk(8.8, 1e-9), "upper_action": chk(23.2, 1e-9)},
    T("TR 569 Example 7.", "TR 569 Örnek 7."))

# ------------------------------------------------------------------ Example 8: copper, review
project("chemistry", "ex8-copper-x-review",
    T("Copper in water by ICP-OES: review of preliminary X-chart limits (Nordtest TR 569 Example 8)", "ICP-OES ile suda bakır: ön X-grafiği sınırlarının gözden geçirilmesi (Nordtest TR 569 Örnek 8)"),
    T("Synthetic standard (1.00 ± 0.02 mg/L) measured in duplicate; preliminary limits from the first 60 runs, x̄ = 1.055 mg/L, s = 0.0667 mg/L. The last 60 values are reviewed: 11 outside the warning limits; one value more than 4 s from the central line is excluded, leaving 59 values with x̄ = 1.041 mg/L and s_Rw = 0.0834 mg/L.",
      "Paralel ölçülen sentetik standart (1,00 ± 0,02 mg/L); ön sınırlar ilk 60 seriden, x̄ = 1,055 mg/L, s = 0,0667 mg/L. Son 60 değer gözden geçirilir: 11'i uyarı sınırları dışında; merkez çizgisinden 4 s'den uzak bir değer dışarıda bırakılır, x̄ = 1,041 mg/L ve s_Rw = 0,0834 mg/L olan 59 değer kalır."),
    [RULES_X,
     T("More than 6 of about 60 values outside the warning limits: the standard deviation has probably increased, so the F-test is applied (Chapter 10).", "Yaklaşık 60 değerden 6'dan fazlası uyarı sınırları dışında: standart sapma muhtemelen artmıştır, bu nedenle F-testi uygulanır (Bölüm 10)."),
     T("F = 0.0834²/0.0667² = 1.563 with ν = 58 and 59: below the critical value (1.67 in Table 3 at 60/60; 1.68 exactly), no significant increase.", "F = 0,0834²/0,0667² = 1,563, ν = 58 ve 59: kritik değerin altında (Tablo 3'te 60/60 için 1,67; tam değeri 1,68), anlamlı artış yok."),
     T("t-test of the means with the pooled s_p = √((59·0.0667² + 58·0.0834²)/117) = 0.07545 mg/L (Equation 9): t = |1.055 − 1.041|/0.07545·√(60·59/119) = 1.012 < 1.98 (ν = 117): no significant change of the central line.",
       "Birleştirilmiş s_p = √((59·0,0667² + 58·0,0834²)/117) = 0,07545 mg/L ile ortalamalara t-testi (Eşitlik 9): t = |1,055 − 1,041|/0,07545·√(60·59/119) = 1,012 < 1,98 (ν = 117): merkez çizgisinde anlamlı değişiklik yok.")],
    "Chapter 14, Example 8", ["copper", "water", "icp-oes", "x-chart", "review", "f-test", "t-test"],
    {"quantity": "Cu, synthetic standard 1.00 mg/L", "unit": "mg/L", "replicates": 2, "central_line": {"mean": 1.055},
     "limits": {"basis": "statistical", "sd": 0.0667},
     "review": {"previous": {"mean": 1.055, "sd": 0.0667, "n": 60}, "current": {"mean": 1.041, "sd": 0.0834, "n": 59}}},
    {"lower_warning": chk(0.92, 0.005), "upper_warning": chk(1.19, 0.005), "lower_action": chk(0.85, 0.005), "upper_action": chk(1.26, 0.005),
     "review": {"f": chk(1.563, 5e-4), "f_critical": chk(1.67, 0.015), "pooled_sd": chk(0.07545, 5e-5), "t": chk(1.012, 5e-4),
                "t_critical": chk(1.98, 0.005), "sd_significant": False, "mean_significant": False}},
    T("TR 569 Example 8. The critical F of 1.67 is read from Table 3 at 60 and 60 degrees of freedom; the program computes F₀.₉₇₅(58, 59) exactly.",
      "TR 569 Örnek 8. 1,67 kritik F değeri Tablo 3'ten 60 ve 60 serbestlik derecesinde okunmuştur; program F₀.₉₇₅(58, 59) değerini tam olarak hesaplar."))
project("chemistry", "ex8-copper-x-new-limits",
    T("Copper in water by ICP-OES: new X-chart limits from all data (Nordtest TR 569 Example 8)", "ICP-OES ile suda bakır: tüm verilerden yeni X-grafiği sınırları (Nordtest TR 569 Örnek 8)"),
    T("After the review the limits are recalculated from the longer period: x̄ = 1.048 mg/L and s = 0.0822 mg/L.", "Gözden geçirmeden sonra sınırlar daha uzun dönemden yeniden hesaplanır: x̄ = 1,048 mg/L ve s = 0,0822 mg/L."),
    [RULES_X, T("The +5 % bias against the nominal 1.00 mg/L should be investigated if it matters for the use of the results.", "Nominal 1,00 mg/L'ye göre +%5 sapma, sonuçların kullanımı için önemliyse araştırılmalıdır.")],
    "Chapter 14, Example 8", ["copper", "water", "icp-oes", "x-chart"],
    {"quantity": "Cu, synthetic standard 1.00 mg/L", "unit": "mg/L", "replicates": 2, "central_line": {"mean": 1.048},
     "limits": {"basis": "statistical", "sd": 0.0822}},
    {"lower_warning": chk(0.88, 0.005), "upper_warning": chk(1.21, 0.005), "lower_action": chk(0.80, 0.005), "upper_action": chk(1.29, 0.005)},
    T("TR 569 Example 8.", "TR 569 Örnek 8."))
project("chemistry", "ex8-copper-r-review",
    T("Copper in water by ICP-OES: R-chart of duplicates and its review (Nordtest TR 569 Example 8)", "ICP-OES ile suda bakır: paralel tayinlerin R-grafiği ve gözden geçirilmesi (Nordtest TR 569 Örnek 8)"),
    T("Duplicates of test samples; preliminary mean range 0.11 mg/L, s_r = 0.0975 mg/L; s_r of the last 60 runs 0.0957 mg/L.", "Numunelerin paralel tayinleri; ön ortalama açıklık 0,11 mg/L, s_r = 0,0975 mg/L; son 60 serinin s_r'si 0,0957 mg/L."),
    [RANGE, T("Warning limit 2.833·0.0975 = 0.28 mg/L, action limit 3.686·0.0975 = 0.36 mg/L.", "Uyarı sınırı 2,833·0,0975 = 0,28 mg/L, eylem sınırı 3,686·0,0975 = 0,36 mg/L."),
     T("F = 0.0975²/0.0957² = 1.04 below the critical value 1.67 (ν = 60, 60): the repeatability has not changed. s_r exceeds s_Rw of the X-chart because the X-chart plots means of two results.",
       "F = 0,0975²/0,0957² = 1,04, kritik değer 1,67'nin altında (ν = 60, 60): tekrarlanabilirlik değişmemiştir. X-grafiği iki sonucun ortalamalarını çizdiğinden s_r, X-grafiğinin s_Rw'sinden büyüktür.")],
    "Chapter 14, Example 8", ["copper", "water", "icp-oes", "r-chart", "review", "f-test"],
    {"quantity": "Cu, duplicates of test samples", "unit": "mg/L", "chart": "range", "replicates": 2, "central_line": {"mean": 0.11},
     "review": {"previous": {"sd": 0.0975, "n": 60, "dof": 60}, "current": {"sd": 0.0957, "n": 60, "dof": 60}}},
    {"sd": chk(0.0975, 5e-5), "upper_warning": chk(0.28, 0.005), "upper_action": chk(0.36, 0.005),
     "review": {"f": chk(1.038, 5e-4), "f_critical": chk(1.67, 0.005), "sd_significant": False}},
    T("TR 569 Example 8. The example prints F = 1.037; 0.0975²/0.0957² = 1.0380.", "TR 569 Örnek 8. Örnek F = 1,037 yazar; 0,0975²/0,0957² = 1,0380'dir."))

# ------------------------------------------------------------------ Example 9: blank
project("chemistry", "ex9-zinc-procedural-blank",
    T("Zinc in hydrogen peroxide: X-chart of a procedural blank (Nordtest TR 569 Example 9)", "Hidrojen peroksitte çinko: yöntem körünün X-grafiği (Nordtest TR 569 Örnek 9)"),
    T("Ultrapure water taken through the whole procedure (evaporation of 50 mL, HCl, dilution to 5 mL, ICP-OES) to detect contamination; negative values are plotted as well.",
      "Kirlenmeyi saptamak için tüm işlemden geçirilen ultra saf su (50 mL buharlaştırma, HCl, 5 mL'ye seyreltme, ICP-OES); negatif değerler de çizilir."),
    [RULES_X, T("The lower limits are negative: −0.051 mg/L and −0.096 mg/L.", "Alt sınırlar negatiftir: −0,051 mg/L ve −0,096 mg/L.")],
    "Chapter 14, Example 9", ["zinc", "blank", "icp-oes", "x-chart"],
    {"quantity": "Zn, procedural blank", "unit": "mg/L", "central_line": {"mean": 0.039}, "limits": {"basis": "statistical", "sd": 0.045}},
    {"lower_warning": chk(-0.051, 1e-9), "upper_warning": chk(0.129, 1e-9), "lower_action": chk(-0.096, 1e-9), "upper_action": chk(0.174, 1e-9)},
    T("TR 569 Example 9.", "TR 569 Örnek 9."))

# ------------------------------------------------------------------ Example 10: boron
project("chemistry", "ex10-boron-toys-target",
    T("Boron migrating from toys by ICP-OES: target limits from the reproducibility of EN 71-3 (Nordtest TR 569 Example 10)", "ICP-OES ile oyuncaklardan bor göçü: EN 71-3 tekrar üretilebilirliğinden hedef sınırlar (Nordtest TR 569 Örnek 10)"),
    T("No material for the whole procedure: an in-house synthetic standard of 5.0 µg/L monitors drift; s_target = 10 %, two thirds of the 15 % reproducibility of EN 71-3.",
      "Tüm işlem için malzeme yoktur: 5,0 µg/L laboratuvar içi sentetik standart kaymayı izler; s_hedef = %10, EN 71-3'ün %15 tekrar üretilebilirliğinin üçte ikisi."),
    [TARGET_X, T("Nearly all values lie above the central line, a bias of about +6 % (mean 5.3 µg/L), judged acceptable for a required uncertainty of about 30 %.",
                 "Değerlerin neredeyse tamamı merkez çizgisinin üzerindedir; yaklaşık +%6 sapma (ortalama 5,3 µg/L), yaklaşık %30 gerekli belirsizlik için kabul edilebilir bulunmuştur.")],
    "Chapter 14, Example 10", ["boron", "toys", "icp-oes", "target-limits"],
    {"quantity": "B, synthetic standard", "unit": "µg/L", "central_line": {"reference": 5.0}, "limits": {"basis": "target", "sd_percent": 10.0}},
    {"lower_warning": chk(4.0, 1e-9), "upper_warning": chk(6.0, 1e-9), "lower_action": chk(3.5, 1e-9), "upper_action": chk(6.5, 1e-9)},
    T("TR 569 Example 10.", "TR 569 Örnek 10."))

# ------------------------------------------------------------------ Example 11: micropipette
project("general", "ex11-micropipette-tolerance",
    T("Bias of a 1 mL micropipette: chart with action limits at the tolerance (Nordtest TR 569 Example 11)", "1 mL mikropipetin sapması: toleransta eylem sınırlı grafik (Nordtest TR 569 Örnek 11)"),
    T("A fixed 1 mL micropipette tested ten times (Nordtest TR 626); the mean volume is plotted against the nominal value with action limits at the laboratory's tolerance of ± 1 %.",
      "Sabit hacimli 1 mL mikropipet on kez test edilir (Nordtest TR 626); ortalama hacim, laboratuvarın ± %1 toleransındaki eylem sınırlarıyla nominal değere karşı çizilir."),
    [T("Central line the nominal value 1.00 mL; action limits 1.00 ± 0.01 mL; no warning limits (instrument qualification, Chapter 11).", "Merkez çizgisi nominal değer 1,00 mL; eylem sınırları 1,00 ± 0,01 mL; uyarı sınırı yok (cihaz yeterliliği, Bölüm 11).")],
    "Chapter 14, Example 11", ["pipette", "volume", "action-limits-only", "tolerance", "instrument-qualification"],
    {"quantity": "mean volume of a 1 mL micropipette", "unit": "mL", "central_line": {"reference": 1.0}, "limits": {"basis": "tolerance", "tolerance": 0.01}},
    {"lower_action": chk(0.99, 1e-12), "upper_action": chk(1.01, 1e-12)},
    T("TR 569 Example 11.", "TR 569 Örnek 11."))

# ------------------------------------------------------------------ Example 12: pooling
days = [[7.1, 7.1, 7.0], [6.9, 6.7, 6.8], [6.6, 6.5, 6.9], [6.7, 6.5, 6.6],
        [7.0, 6.9, 6.6], [7.3, 7.4, 7.3], [7.1, 7.1, 6.9], [7.0, 6.5, 6.8]]
project("chemistry", "ex12-pooling-sr-srw",
    T("Repeatability and within-laboratory reproducibility from replicate control results (Nordtest TR 569 Example 12)", "Tekrarlı kontrol sonuçlarından tekrarlanabilirlik ve laboratuvar içi tekrar üretilebilirlik (Nordtest TR 569 Örnek 12)"),
    T("Three replicates of a QC sample on each of eight days. The standard deviation of all 24 results together is too low an estimate of s_Rw and would give control limits that are too narrow.",
      "Sekiz günün her birinde bir kalite kontrol numunesinin üç tekrarı. 24 sonucun birlikte standart sapması s_Rw için fazla düşük bir tahmindir ve çok dar kontrol sınırları verir."),
    [T("s_r: the standard deviations of the eight days (0.058 to 0.252) pooled by Equation 10: 0.15.", "s_r: sekiz günün standart sapmaları (0,058 – 0,252) Eşitlik 10 ile birleştirilir: 0,15."),
     T("Simplified s_Rw: the standard deviations of the first, second and third results over the days (0.226, 0.342, 0.226) pooled by Equation 10: 0.270.",
       "Basitleştirilmiş s_Rw: günler boyunca birinci, ikinci ve üçüncü sonuçların standart sapmaları (0,226, 0,342, 0,226) Eşitlik 10 ile birleştirilir: 0,270."),
     T("One-way analysis of variance: s_r = 0.1541, between-day s_I = 0.2246, s_Rw = √(s_r² + s_I²) = 0.272 (footnote 24).",
       "Tek yönlü varyans analizi: s_r = 0,1541, günler arası s_I = 0,2246, s_Rw = √(s_r² + s_I²) = 0,272 (dipnot 24)."),
     T("The X-chart of the daily means has s = √(s_I² + s_r²/3); its limits follow from the eight means.", "Günlük ortalamaların X-grafiğinin s'si √(s_I² + s_r²/3)'tür; sınırları sekiz ortalamadan çıkar.")],
    "Chapter 14, Example 12 and footnote 24", ["pooling", "anova", "repeatability", "intermediate-precision", "raw-data"],
    {"quantity": "QC sample", "runs": days, "labels": [f"day {i}" for i in range(1, 9)]},
    {"precision": {"s_r_pooled": chk(0.15, 0.005), "s_rw_simplified": chk(0.270, 5e-4), "s_r": chk(0.1541, 5e-5),
                   "s_between": chk(0.2246, 1e-4), "s_rw": chk(0.272, 5e-4)}},
    T("TR 569 Example 12; footnote 24 prints √(0.01541² + 0.02246²) = 0.0272, ten times too small (research log F53).",
      "TR 569 Örnek 12; dipnot 24 √(0,01541² + 0,02246²) = 0,0272 yazar, on kat küçük (araştırma günlüğü F53)."),
    notes=[T("Research log F53: footnote 24 gives s_Rw = √(0.01541² + 0.02246²) = 0.0272; the values are s_r = 0.1541, s_I = 0.2246 and s_Rw = 0.272 as in the text.",
             "Araştırma günlüğü F53: dipnot 24 s_Rw = √(0,01541² + 0,02246²) = 0,0272 verir; değerler metindeki gibi s_r = 0,1541, s_I = 0,2246 ve s_Rw = 0,272'dir.")])
print("written")
