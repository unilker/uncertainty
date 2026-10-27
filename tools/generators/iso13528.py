"""Writes the ISO 13528:2022 Annex E library projects (run from the repository root)."""
import json
from pathlib import Path

DOC = "ISO 13528:2022, Statistical methods for use in proficiency testing by interlaboratory comparison"
URL = "https://www.iso.org/standard/78879.html"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}

def project(path, pid, title, desc, expl, sec, tags, notes, prof, expected, reporting=None):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "chemistry",
                      "sources": [{"document": DOC, "section": sec, "url": URL}], "tags": ["iso13528", "proficiency-testing"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p["proficiency"] = prof
    if reporting: p["reporting"] = reporting
    p["expected"] = expected
    Path(path).write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

ATRAZINE = [0.040, 0.055, 0.178, 0.202, 0.206, 0.227, 0.228, 0.230, 0.230, 0.235, 0.236, 0.237, 0.243, 0.244, 0.245, 0.2555,
            0.260, 0.264, 0.267, 0.270, 0.273, 0.274, 0.274, 0.278, 0.2811, 0.287, 0.287, 0.288, 0.289, 0.295, 0.296, 0.311,
            0.331, 0.4246]
results = [{"lab": str(i + 1), "value": v} for i, v in enumerate(ATRAZINE)]
common_expl = [
    T("34 participants reported atrazine in drinking water (Table E.4, ordered by value). Two very low results (0.040 and 0.055 mg/L) and one high result (0.4246 mg/L) lie far from the majority.",
      "34 katılımcı içme suyunda atrazin sonucu bildirmiştir (Tablo E.4, değere göre sıralı). İki çok düşük (0,040 ve 0,055 mg/L) ve bir yüksek (0,4246 mg/L) sonuç çoğunluktan çok uzaktır."),
]
stats_expected = {"median": chk(0.2620, 0.00005), "made": chk(0.0386, 0.00005), "niqr": chk(0.0402, 0.00005),
                  "robust_mean": chk(0.2570, 0.00005), "robust_sd": chk(0.0395, 0.00005), "iterations": chk(6, 0),
                  "mean": chk(0.2512, 0.00005), "sd": chk(0.0672, 0.00005)}
project("library/chemistry/iso13528-e3-atrazine-algorithm-a.json", "iso13528-e3-atrazine-algorithm-a",
  T("Atrazine in drinking water: robust average by Algorithm A (ISO 13528 E.3)", "İçme suyunda atrazin: Algoritma A ile robust ortalama (ISO 13528 E.3)"),
  T("ISO 13528:2022 E.3: assigned value and standard deviation for proficiency assessment from 34 participant results with Algorithm A; z scores.",
    "ISO 13528:2022 E.3: 34 katılımcı sonucundan Algoritma A ile atanan değer ve yeterlilik değerlendirmesi için standart sapma; z skorları."),
  common_expl + [
    T("Algorithm A (C.3.1) starts from x* = median = 0.2620 and s* = MADe = 0.0386 mg/L, replaces every result outside x* ± 1.5s* by the nearer limit and recomputes x* as the mean and s* as 1.134 × the standard deviation of the modified values.",
      "Algoritma A (C.3.1) x* = medyan = 0,2620 ve s* = MADe = 0,0386 mg/L değerleriyle başlar, x* ± 1,5s* dışındaki her sonucu yakın sınırla değiştirir ve x*'ı değiştirilmiş değerlerin ortalaması, s*'ı standart sapmalarının 1,134 katı olarak yeniden hesaplar."),
    T("After six iterations the third significant figures no longer change: x* = 0.2570 mg/L, s* = 0.0395 mg/L. The arithmetic mean 0.2512 and standard deviation 0.0672 mg/L are pulled by the three outlying results.",
      "Altı iterasyondan sonra üçüncü anlamlı basamaklar değişmez: x* = 0,2570 mg/L, s* = 0,0395 mg/L. Aritmetik ortalama 0,2512 ve standart sapma 0,0672 mg/L, üç uç sonuç tarafından çekilir."),
    T("The assigned value is x_pt = x* with u(x_pt) = 1.25 s*/√p = 1.25 × 0.0395/√34 = 0.0085 mg/L (Formula 6) and σ_pt = s*. Since u(x_pt) < 0.3σ_pt = 0.0119 mg/L the uncertainty of the assigned value is negligible (9.2.1) and z scores are used; laboratories 1, 2 and 34 receive action signals.",
      "Atanan değer x_pt = x*, u(x_pt) = 1,25 s*/√p = 1,25 × 0,0395/√34 = 0,0085 mg/L (Formül 6) ve σ_pt = s*'tır. u(x_pt) < 0,3σ_pt = 0,0119 mg/L olduğundan atanan değerin belirsizliği ihmal edilebilir (9.2.1) ve z skorları kullanılır; 1, 2 ve 34 numaralı laboratuvarlar eylem sinyali alır.")],
  "E.3, Tables E.4 and E.5", ["algorithm-a", "robust-statistics"],
  [T("The stopping rule of C.3.1 (no change in the third significant figures) is used; iterating to full convergence gives x* = 0.257011 and s* = 0.039520 mg/L, the same to the printed digits. The nIQR of Table E.5 (0.0402) corresponds to quartiles by linear interpolation between order statistics; other quartile definitions give 0.0379 to 0.0423 (Table E.5, note).",
     "C.3.1'in durma kuralı (üçüncü anlamlı basamaklarda değişiklik olmaması) kullanılmıştır; tam yakınsamaya kadar iterasyon x* = 0,257011 ve s* = 0,039520 mg/L verir, basılı basamaklara kadar aynıdır. Tablo E.5'teki nIQR (0,0402), sıra istatistikleri arasında doğrusal enterpolasyonla bulunan çeyreklere karşılık gelir; diğer çeyrek tanımları 0,0379 ile 0,0423 arası değerler verir (Tablo E.5, not).")],
  {"quantity": "atrazine", "unit": "mg/L", "results": results, "assigned_value": {"method": "algorithm_a"}, "sd_pt": {"method": "robust"}},
  {"proficiency": {**stats_expected, "assigned_value": chk(0.2570, 0.00005), "u_assigned": chk(0.0085, 0.00005), "sd_pt": chk(0.0395, 0.00005),
                   "scores": {"1": {"z": chk((0.040 - 0.257011) / 0.03952, 0.02)}, "34": {"z": chk((0.4246 - 0.257011) / 0.03952, 0.02)}}},
   "reference": T("ISO 13528:2022 Table E.4 (x* = 0.2570, s* = 0.0395 mg/L after 6 iterations) and Table E.5 (median 0.2620, nIQR 0.0402, MADe 0.0386, u(x_pt) = 0.0085, arithmetic 0.2512 and 0.0672 mg/L); z scores recomputed from these values.",
                  "ISO 13528:2022 Tablo E.4 (6 iterasyondan sonra x* = 0,2570, s* = 0,0395 mg/L) ve Tablo E.5 (medyan 0,2620, nIQR 0,0402, MADe 0,0386, u(x_pt) = 0,0085, aritmetik 0,2512 ve 0,0672 mg/L); z skorları bu değerlerden yeniden hesaplanmıştır.")},
  reporting={"significant_digits": 2})

project("library/chemistry/iso13528-e3-atrazine-median.json", "iso13528-e3-atrazine-median",
  T("Atrazine in drinking water: median and nIQR (ISO 13528 E.3)", "İçme suyunda atrazin: medyan ve nIQR (ISO 13528 E.3)"),
  T("ISO 13528:2022 E.3, first row of Table E.5: the median as assigned value with u(x_pt) = 1.25 nIQR/√p and the nIQR as σ_pt.",
    "ISO 13528:2022 E.3, Tablo E.5'in ilk satırı: atanan değer olarak medyan, u(x_pt) = 1,25 nIQR/√p ve σ_pt olarak nIQR."),
  common_expl + [
    T("The simple robust estimators of C.2: median 0.2620 mg/L, MADe = 1.483 × median absolute deviation = 0.0386 mg/L and nIQR = 0.7413 (Q₃ − Q₁) = 0.0402 mg/L.",
      "C.2'nin basit robust tahmin edicileri: medyan 0,2620 mg/L, MADe = 1,483 × medyan mutlak sapma = 0,0386 mg/L ve nIQR = 0,7413 (Q₃ − Q₁) = 0,0402 mg/L."),
    T("With the median as assigned value, u(x_pt) = 1.25 × 0.0402/√34 = 0.0086 mg/L, practically the same as with Algorithm A (0.0085 mg/L); the nIQR has a breakdown point of 25 %, the MADe of 50 % (C.2.3, Note 1).",
      "Atanan değer medyan olduğunda u(x_pt) = 1,25 × 0,0402/√34 = 0,0086 mg/L olur; bu, Algoritma A ile bulunan değerle (0,0085 mg/L) pratik olarak aynıdır; nIQR'nin kırılma noktası %25, MADe'ninki %50'dir (C.2.3, Not 1).")],
  "E.3, Table E.5", ["median", "niqr", "robust-statistics"], None,
  {"quantity": "atrazine", "unit": "mg/L", "results": results, "assigned_value": {"method": "median"}, "sd_pt": {"method": "niqr"}},
  {"proficiency": {"median": chk(0.2620, 0.00005), "made": chk(0.0386, 0.00005), "niqr": chk(0.0402, 0.00005),
                   "assigned_value": chk(0.2620, 0.00005), "u_assigned": chk(0.0086, 0.00005), "sd_pt": chk(0.0402, 0.00005)},
   "reference": T("ISO 13528:2022 Table E.5, row 'Robust: Median, nIQR (MADe)': 0.2620, 0.0402 (0.0386), u(x_pt) = 0.0086 mg/L.",
                  "ISO 13528:2022 Tablo E.5, 'Robust: Median, nIQR (MADe)' satırı: 0,2620; 0,0402 (0,0386); u(x_pt) = 0,0086 mg/L.")},
  reporting={"significant_digits": 2})

# --------------------------------------------------------------------- E.4 mercury
HG = [("L04", 0.013, 0.003, 2, "AMA"), ("L05", 0.013, 0.007, 2, "AMA"), ("L23", 0.0135, 0.00108, 1.732, "AMA"),
      ("L02", 0.014, 0.004, 2, "AMA"), ("L15", 0.014, 0.0005, 2, "AMA"), ("L17", "<0.015", None, None, "CV-ICP-AES"),
      ("L06", 0.016, 0.003, 2, "AMA"), ("L09", 0.017, 0.008, 2, "AMA"), ("L26", 0.019, 0.003, 2, "AAS"),
      ("L12", 0.0239, 0.0036, 2, "AMA"), ("L13", "<0.034", None, None, "TDA-AAS"), ("L03", 0.037, 0.013, 2, "CV-AAS"),
      ("L29", 0.039, 0.007, 2, "CV-AAS"), ("L07", 0.04, 0.008, 2, "ICP-MS"), ("L21", 0.04, 0.03, 2, "HG-AAS"),
      ("L25", 0.040, 0.010, 2, "CV-AAS"), ("L16", 0.0424, 0.008, 2, "CV-AAS"), ("L08", 0.044, 0.007, 2, "CV-AAS"),
      ("L10", 0.045, 0.007, 2, "ICP-MS"), ("L24", 0.045, 0.005, 2, "HG-AAS"), ("L18", 0.046, 0.007, 2, "CV-AAS"),
      ("L28", 0.049, 0.0072, 2, "CV-AAS"), ("L01", 0.053, 0.007, 2, "CV-AAS"), ("L14", "<0.1", None, None, "ICP-MS")]
E7 = {"L04": (-70.5, -156.6, -4.70, -3.99, -7.10, -3.55), "L05": (-70.5, -156.6, -4.70, -3.99, -5.75, -2.88),
      "L23": (-69.3, -154.0, -4.62, -3.93, -7.35, -3.69), "L02": (-68.2, -151.5, -4.55, -3.86, -6.58, -3.29),
      "L15": (-68.2, -151.5, -4.55, -3.86, -7.30, -3.65), "L06": (-63.6, -141.4, -4.24, -3.60, -6.41, -3.21),
      "L09": (-61.4, -136.4, -4.09, -3.47, -4.71, -2.36), "L26": (-56.8, -126.3, -3.79, -3.22, -5.73, -2.86),
      "L12": (-45.7, -101.5, -3.05, -2.59, -4.49, -2.24), "L03": (-15.9, -35.4, -1.06, -0.90, -0.91, -0.46),
      "L29": (-11.4, -25.3, -0.76, -0.64, -0.93, -0.46), "L07": (-9.1, -20.2, -0.61, -0.51, -0.70, -0.35),
      "L21": (-9.1, -20.2, -0.61, -0.51, -0.26, -0.13), "L25": (-9.1, -20.2, -0.61, -0.51, -0.62, -0.31),
      "L16": (-3.6, -8.1, -0.24, -0.21, -0.28, -0.14), "L08": (0.0, 0.0, 0.00, 0.00, 0.00, 0.00),
      "L10": (2.3, 5.1, 0.15, 0.13, 0.19, 0.09), "L24": (2.3, 5.1, 0.15, 0.13, 0.21, 0.10),
      "L18": (4.5, 10.1, 0.30, 0.26, 0.37, 0.19), "L28": (11.4, 25.3, 0.76, 0.64, 0.92, 0.46),
      "L01": (20.5, 45.5, 1.36, 1.16, 1.67, 0.83)}
res = []
for lab, v, U, k, m in HG:
    r = {"lab": lab}
    if isinstance(v, str):
        r["less_than"] = float(v[1:])
    else:
        r["value"] = v; r["expanded"] = U; r["k"] = k
    r["method"] = m
    res.append(r)
scores = {lab: {"d_percent": chk(a, 0.051), "pa": chk(b, 0.051), "z": chk(c, 0.0051), "z_prime": chk(d, 0.0051),
                "zeta": chk(e, 0.0051), "en": chk(f, 0.0051)} for lab, (a, b, c, d, e, f) in E7.items()}
project("library/chemistry/iso13528-e4-mercury-feed.json", "iso13528-e4-mercury-feed",
  T("Mercury in animal feed: z, z′, ζ and E_n scores (ISO 13528 E.4)", "Hayvan yeminde cıva: z, z′, ζ ve E_n skorları (ISO 13528 E.4)"),
  T("ISO 13528:2022 E.4 (IMEP 111): scores of 21 laboratories against a reference value with a fitness-for-purpose σ_pt, using the uncertainties the laboratories reported.",
    "ISO 13528:2022 E.4 (IMEP 111): bir referans değere ve amaca uygun σ_pt'ye karşı 21 laboratuvarın, bildirdikleri belirsizlikler kullanılarak hesaplanan skorları."),
  [T("x_pt = 0.044 mg/kg is a reference value with U(x_pt) = 0.0082 mg/kg (k = 2, characterisation and inhomogeneity); σ_pt = 0.0066 mg/kg (15 %) is a fitness-for-purpose value and δ_E = 3σ_pt for the PA scores. Three results reported as 'less than' are not scored.",
     "x_pt = 0,044 mg/kg, U(x_pt) = 0,0082 mg/kg (k = 2, karakterizasyon ve homojensizlik) olan bir referans değerdir; σ_pt = 0,0066 mg/kg (%15) amaca uygunluk değeridir ve PA skorları için δ_E = 3σ_pt'dir. 'Küçüktür' olarak bildirilen üç sonuç skorlanmaz."),
   T("The laboratories' standard uncertainties are their expanded uncertainties divided by their coverage factors (L23: k = 1.732); unrounded values are used.",
     "Laboratuvarların standart belirsizlikleri, genişletilmiş belirsizliklerinin kapsam faktörlerine bölünmesiyle bulunur (L23: k = 1,732); yuvarlanmamış değerler kullanılır."),
   T("z = D/σ_pt, z′ = D/√(σ_pt² + u²(x_pt)), ζ = D/√(u²(x) + u²(x_pt)) and E_n = D/√(U²(x) + U²(x_pt)). Here u(x_pt) = 0.0041 > 0.3σ_pt = 0.0020, so the uncertainty of the assigned value is not negligible and z′ is about 15 % smaller than z.",
     "z = D/σ_pt, z′ = D/√(σ_pt² + u²(x_pt)), ζ = D/√(u²(x) + u²(x_pt)) ve E_n = D/√(U²(x) + U²(x_pt)). Burada u(x_pt) = 0,0041 > 0,3σ_pt = 0,0020 olduğundan atanan değerin belirsizliği ihmal edilemez ve z′, z'den yaklaşık %15 küçüktür."),
   T("The results are bimodal by method: the eight laboratories using a direct mercury analyser (AMA) and one using AAS obtain 30 % to 54 % of the reference value and receive action signals (|z| ≥ 3), whereas all laboratories using CV-AAS, HG-AAS or ICP-MS are acceptable.",
     "Sonuçlar yönteme göre iki tepelidir: doğrudan cıva analizörü (AMA) kullanan sekiz laboratuvar ile AAS kullanan bir laboratuvar referans değerin %30 ile %54'ünü bulur ve eylem sinyali alır (|z| ≥ 3); CV-AAS, HG-AAS veya ICP-MS kullanan laboratuvarların hepsi kabul edilebilir sonuç verir.")],
  "E.4, Tables E.6 and E.7", ["z-score", "zeta-score", "en-score", "reference-value"],
  [T("Table E.6 flags the reported uncertainties a/b/c 'following criteria discussed in 9.8', but the flags are not reproduced with u_min = u(x_pt) = 0.0041 mg/kg (9.8.3): L05 (u = 0.0035) and L09 (u = 0.0040) are flagged 'a'. The flags imply a lower limit between 0.0018 and 0.0035 mg/kg (possibly the characterisation uncertainty without inhomogeneity) and an upper limit between 0.005 and 0.015 mg/kg (consistent with 1.5σ_pt = 0.0099). The flags are therefore not checked here.",
     "Tablo E.6, bildirilen belirsizlikleri '9.8'de tartışılan ölçütlere göre' a/b/c olarak işaretler; ancak bayraklar u_min = u(x_pt) = 0,0041 mg/kg (9.8.3) ile yeniden üretilemez: L05 (u = 0,0035) ve L09 (u = 0,0040) 'a' olarak işaretlenmiştir. Bayraklar 0,0018 ile 0,0035 mg/kg arasında bir alt sınıra (belki homojensizlik içermeyen karakterizasyon belirsizliği) ve 0,005 ile 0,015 mg/kg arasında bir üst sınıra (1,5σ_pt = 0,0099 ile tutarlı) işaret eder. Bu nedenle bayraklar burada kontrol edilmemiştir.")],
  {"quantity": "total Hg", "unit": "mg/kg", "results": res,
   "assigned_value": {"method": "reference", "value": 0.044, "expanded": 0.0082, "k": 2},
   "sd_pt": {"method": "given", "value": 0.0066},
   "uncertainty_screening": {"u_min": "u_assigned", "u_max": "1.5_sd_pt"}},
  {"proficiency": {"assigned_value": chk(0.044, 1e-9), "u_assigned": chk(0.0041, 1e-9), "sd_pt": chk(0.0066, 1e-9), "scores": scores},
   "reference": T("ISO 13528:2022 Table E.7 (D %, PA, z, z′, ζ and E_n of the 21 numerical results; tolerances half a unit of the printed digit).",
                  "ISO 13528:2022 Tablo E.7 (21 sayısal sonucun D %, PA, z, z′, ζ ve E_n değerleri; toleranslar basılı son basamağın yarım birimi).")},
  reporting={"significant_digits": 2})

# --------------------------------------------------------------------- E.2 arsenic
project("library/chemistry/iso13528-e2-arsenic-homogeneity.json", "iso13528-e2-arsenic-homogeneity",
  T("Arsenic in chocolate: homogeneity and stability of PT items (ISO 13528 E.2)", "Çikolatada arsenik: YT numunelerinin homojenliği ve kararlılığı (ISO 13528 E.2)"),
  T("ISO 13528:2022 E.2: between-sample standard deviation from 10 items measured in duplicate, compared with 0.3σ_pt, and a stability check after six weeks at 60 °C.",
    "ISO 13528:2022 E.2: ikişer kez ölçülen 10 numuneden numuneler arası standart sapma, 0,3σ_pt ile karşılaştırma ve 60 °C'de altı hafta sonra kararlılık kontrolü."),
  [T("Ten vials, selected by stratified random sampling from 1 000, are each tested twice under repeatability conditions (Table E.2).",
     "1 000 şişeden tabakalı rastgele örneklemeyle seçilen on şişe, tekrarlanabilirlik koşullarında ikişer kez test edilmiştir (Tablo E.2)."),
   T("Analysis of variance (B.3): general average 0.18715 mg/kg, standard deviation of the item averages s_x = 0.00398, within-sample s_w = 0.00556 and between-sample s_s = √(s_x² − s_w²/2) = 0.00060 mg/kg.",
     "Varyans analizi (B.3): genel ortalama 0,18715 mg/kg, numune ortalamalarının standart sapması s_x = 0,00398, numune içi s_w = 0,00556 ve numuneler arası s_s = √(s_x² − s_w²/2) = 0,00060 mg/kg."),
   T("With the provisional σ_pt = 15 % of the average = 0.02807 mg/kg, s_s is far below 0.3σ_pt = 0.00842 mg/kg: the items are sufficiently homogeneous. The expanded criterion of B.2.3 (F₁ = 1.88, F₂ = 1.01 for g = 10) gives √c = 0.0133 mg/kg.",
     "Ortalamanın %15'i olan geçici σ_pt = 0,02807 mg/kg ile s_s, 0,3σ_pt = 0,00842 mg/kg'nin çok altındadır: numuneler yeterince homojendir. B.2.3'ün genişletilmiş ölçütü (g = 10 için F₁ = 1,88, F₂ = 1,01) √c = 0,0133 mg/kg verir."),
   T("Stability: two items stored at 60 °C for six weeks give an average of 0.19375 mg/kg; the difference 0.00660 mg/kg from the homogeneity mean is below 0.3σ_pt, so stability is sufficient (B.5.1).",
     "Kararlılık: 60 °C'de altı hafta saklanan iki numune 0,19375 mg/kg ortalama verir; homojenlik ortalamasından 0,00660 mg/kg'lik fark 0,3σ_pt'nin altındadır, dolayısıyla kararlılık yeterlidir (B.5.1).")],
  "E.2, Tables E.2 and E.3", ["homogeneity", "stability"], None,
  {"quantity": "As", "unit": "mg/kg",
   "homogeneity": {"items": [[0.185, 0.194], [0.187, 0.189], [0.182, 0.186], [0.188, 0.196], [0.191, 0.181],
                             [0.188, 0.180], [0.187, 0.196], [0.177, 0.186], [0.179, 0.187], [0.188, 0.196]],
                   "sd_pt_percent_of_mean": 15},
   "stability": {"values": [0.191, 0.198, 0.190, 0.196]}},
  {"proficiency": {"homogeneity": {"mean": chk(0.18715, 0.000005), "s_x": chk(0.00398, 0.000005), "s_w": chk(0.00556, 0.000005),
                                   "s_s": chk(0.00060, 0.000005), "criterion": chk(0.00842, 0.000005), "sufficient": True},
                   "stability": {"difference": chk(0.00660, 0.000005), "sufficient": True}},
   "reference": T("ISO 13528:2022 E.2 (Tables E.2 and E.3 and the summary statistics printed with them).",
                  "ISO 13528:2022 E.2 (Tablo E.2 ve E.3 ile bunlarla birlikte basılı özet istatistikler).")})
