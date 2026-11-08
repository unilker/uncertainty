"""Writes the top-down projects of library/chemistry/ (Nordtest TR 537,
Eurachem/CITAC MUPR, EURACHEM QUAM A4/A6) with raw data.

One-off authoring helper; the JSON files are the maintained source.
Raw data are read from core/tests/topdown_examples.rs, where they were
transcribed from the documents and validated.
"""
import json, math, os, re, sys

ROOT = sys.argv[1]
TESTS = open(os.path.join(ROOT, "..", "core", "tests", "topdown_examples.rs"), encoding="utf-8").read()


def pairs(name):
    m = re.search(r"const " + name + r": \[\(f64, f64\); \d+\] = \[(.*?)\];", TESTS, re.S)
    return [[float(a), float(b)] for a, b in re.findall(r"\(([-\d.]+), ([-\d.]+)\)", m.group(1))]


NH4_LOW, NH4_HIGH, OXYGEN, BOD = pairs("NH4_LOW"), pairs("NH4_HIGH"), pairs("OXYGEN"), pairs("BOD")
NH4_PT = [(float(a), float(b), float(c), int(d)) for a, b, c, d in
          re.findall(r"\(([\d.]+), ([\d.]+), ([\d.]+), (\d+)\)", re.search(r"const NH4_PT.*?\];", TESTS, re.S).group(0))]
B6_BLOCK = TESTS[TESTS.index("fn mupr_example_b6"):]
B6_DS = [float(x) for x in re.search(r"let ds = \[(.*?)\];", B6_BLOCK, re.S).group(1).replace("\n", " ").split(",") if x.strip()]
B6_DUP = [[float(a), float(b)] for a, b in re.findall(r"\(([\d.]+), ([\d.]+)\)", re.search(r"let dup = \[(.*?)\];", B6_BLOCK, re.S).group(1))]
B6_PT = [tuple(float(v) for v in t) for t in re.findall(r"\(([\d.]+), ([\d.]+), ([\d.]+), ([\d.]+), ([\d.]+)\)",
                                                          re.search(r"let pt = \[(.*?)\];", B6_BLOCK, re.S).group(1))]
A4_BLOCK = TESTS  # A4 duplicates live in eurachem_examples.rs
EUR = open(os.path.join(ROOT, "..", "core", "tests", "eurachem_examples.rs"), encoding="utf-8").read()
A4_DUP = [[float(a), float(b)] for a, b in re.findall(r"\(([\d.]+), ([\d.]+)\)",
                                                       re.search(r"let duplicates = \[(.*?)\];", EUR, re.S).group(1))]
assert (len(NH4_LOW), len(NH4_HIGH), len(OXYGEN), len(BOD), len(NH4_PT)) == (46, 26, 51, 18, 6)
assert (len(B6_DS), len(B6_DUP), len(B6_PT), len(A4_DUP)) == (30, 30, 22, 15)

NT = {"document": "Nordtest TR 537 ed. 4 (2017), Handbook for calculation of measurement uncertainty in environmental laboratories",
      "url": "https://nordtest.info/images/documents/nt-technical-reports/NT_TR_537_edition4_English_Handbook_for_calculation_of_measurement_uncertainty_in_environmental_laboratories.pdf"}
MUPR = {"document": "Eurachem/CITAC Guide, Evaluation of measurement uncertainty from in-house precision and recovery data (MUPR), 1st ed. (2026)",
        "url": "https://eurachem.org/images/stories/Guides/pdf/MUPR_2026_EN_v01.pdf"}
QUAM = {"document": "EURACHEM/CITAC Guide CG 4, Quantifying Uncertainty in Analytical Measurement, 3rd ed. (QUAM:2012.P1)",
        "url": "https://www.eurachem.org/images/stories/Guides/pdf/QUAM2012_P1.pdf"}


def src(base, section):
    return {"document": base["document"], "section": section, "url": base["url"]}


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, tol):
    return {"value": v, "tolerance": tol}


def prec(name, data, desc=None, **kw):
    d = {"name": name}
    if desc:
        d["description"] = T(*desc)
    d["data"] = data
    d.update(kw)
    return d


def project(id, title, desc, explanation, sources, topdown, expected, reference, *, notes=(), tags=(), reporting=None):
    meta = {"id": id, "title": T(*title), "description": T(*desc), "explanation": [T(*e) for e in explanation],
            "field": "chemistry", "sources": sources, "tags": ["top-down"] + list(tags)}
    if notes:
        meta["notes"] = [T(*n) for n in notes]
    p = {"format": "uncertainty-calculations/project", "version": 1, "metadata": meta, "topdown": topdown}
    if reporting:
        p["reporting"] = reporting
    p["expected"] = {"topdown": expected, "reference": T(*reference)}
    path = os.path.join(ROOT, "chemistry", id + ".json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(p, f, ensure_ascii=False, indent=2)
        f.write("\n")


WHOLE_PERCENT_UP = {"decimal_places": 0, "rounding": "up"}
NT_COMBINE = ("u_c = √(u(Rw)² + u(bias)²) and U = 2u_c, rounded up to a whole per cent (Nordtest §3).",
              "u_c = √(u(Rw)² + u(bias)²) ve U = 2u_c; tam yüzdeye yukarı yuvarlanır (Nordtest §3).")

# ------------------------------------------------------------------ Nordtest
project("nordtest-3-ammonium-pt",
        ("Ammonium in water from control limits and proficiency tests", "Kontrol sınırları ve yeterlilik testlerinden sudaki amonyum"),
        ("Nordtest TR 537 §3.2 and Appendix 4: NH₄-N, u(Rw) from the control chart and u(bias) from six proficiency tests (raw PT data).",
         "Nordtest TR 537 §3.2 ve Ek 4: NH₄-N; u(Rw) kontrol grafiğinden, u(bias) altı yeterlilik testinden (ham YT verisi)."),
        [("The control chart warning limits are ±3.34 % (2s), so u(Rw) = 3.34/2 = 1.67 %.",
          "Kontrol grafiği uyarı sınırları ±%3,34'tür (2s); bu nedenle u(Rw) = 3,34/2 = %1,67."),
         ("For each PT round the bias is (x_lab − x_assigned)/x_assigned and the uncertainty of the assigned value "
          "u(C_ref,i) = s_R/√n_Lab; RMS_bias = 2.26 %, ū(C_ref) = 1.52 %.",
          "Her YT turu için sapma (x_lab − x_atanan)/x_atanan, atanan değerin belirsizliği u(C_ref,i) = s_R/√n_Lab'dır; "
          "RMS_bias = %2,26, ū(C_ref) = %1,52."),
         ("u(bias) = √(2.26² + 1.52²) = 2.73 %; u_c = √(1.67² + 2.73²) = 3.20 %; U = 6.4 % → 7 %.",
          "u(bias) = √(2,26² + 1,52²) = %2,73; u_c = √(1,67² + 2,73²) = %3,20; U = %6,4 → %7."), NT_COMBINE],
        [src(NT, "§3.2 and Appendix 4")],
        {"quantity": "c(NH₄-N)", "unit": "µg/L",
         "precision": [prec("u(Rw)", {"type": "control_limits", "limit_percent": 3.34, "k": 2},
                            ("Control chart warning limits ±3.34 % (2s)", "Kontrol grafiği uyarı sınırları ±%3,34 (2s)"))],
         "bias": [{"name": "u(bias)", "kind": "proficiency_tests",
                   "references": [{"label": f"PT {i + 1}", "reference": a, "mean": x, "sd_reproducibility_percent": s, "labs": n}
                                  for i, (a, x, s, n) in enumerate(NH4_PT)]}]},
        {"components": {"u(Rw)": chk(1.67, 1e-9), "u(bias)": chk(2.73, 0.005)},
         "bias": {"u(bias)": {"mean": chk(2.20, 0.005), "rms": chk(2.26, 0.005), "mean_u_reference": chk(1.52, 0.005)}},
         "levels": [{"relative_standard_percent": chk(3.20, 0.005), "relative_expanded_percent": chk(6.40, 0.01)}]},
        ("Nordtest TR 537 §3.2 and Appendix 4 (RMS_bias 2.26 %, ū(C_ref) 1.52 %, u(bias) 2.73 %, u_c 3.20 %, U 6.4 → 7 %).",
         "Nordtest TR 537 §3.2 ve Ek 4 (RMS_bias %2,26, ū(C_ref) %1,52, u(bias) %2,73, u_c %3,20, U %6,4 → %7)."),
        reporting=WHOLE_PERCENT_UP, tags=["nordtest", "proficiency-tests"])

project("nordtest-5.3-ammonium-duplicates",
        ("Ammonium: within-laboratory reproducibility from control samples and duplicates",
         "Amonyum: kontrol numuneleri ve mükerrer ölçümlerden laboratuvar içi tekrar üretilebilirlik"),
        ("Nordtest TR 537 §5.3 and Appendix 5: NH₄-N duplicates of natural samples in two ranges (raw data), combined with the control-sample reproducibility.",
         "Nordtest TR 537 §5.3 ve Ek 5: doğal numunelerin iki aralıktaki NH₄-N mükerrer ölçümleri (ham veri), kontrol numunesi tekrar üretilebilirliğiyle birleştirilmiştir."),
        [("The control sample does not cover sample-to-sample variation, so the repeatability from duplicates of real "
          "samples is added: u(Rw) = √(u(Rw, control)² + s_r(duplicates)²).",
          "Kontrol numunesi numuneden numuneye değişimi kapsamadığından gerçek numunelerin mükerrer ölçümlerinden gelen "
          "tekrarlanabilirlik eklenir: u(Rw) = √(u(Rw, kontrol)² + s_r(mükerrer)²)."),
         ("Below 30 µg/L the standard deviation is constant: s_r = √(Σd²/2N) = 0.44 µg/L from 46 pairs; with the control "
          "sample (0.5 µg/L) u(Rw) = 0.67 µg/L.",
          "30 µg/L'nin altında standart sapma sabittir: 46 çiftten s_r = √(Σd²/2N) = 0,44 µg/L; kontrol numunesiyle "
          "(0,5 µg/L) u(Rw) = 0,67 µg/L."),
         ("At and above 30 µg/L it is proportional to the concentration: pooled relative s_r = 3.8 % from 26 pairs; with "
          "the control sample (1.5 %) u(Rw) = 4.1 %.",
          "30 µg/L ve üzerinde derişimle orantılıdır: 26 çiftten birleştirilmiş bağıl s_r = %3,8; kontrol numunesiyle "
          "(%1,5) u(Rw) = %4,1."),
         ("Each component is restricted to its range; the budget is shown at 10 and 100 µg/L.",
          "Her bileşen kendi aralığıyla sınırlandırılmıştır; bütçe 10 ve 100 µg/L'de gösterilmiştir.")],
        [src(NT, "§5.3 and Appendix 5")],
        {"quantity": "c(NH₄-N)", "unit": "µg/L", "levels": [10.0, 100.0],
         "precision": [
             prec("u(Rw), control < 30", {"type": "given", "sd": 0.5}, ("Control sample, low range", "Kontrol numunesi, düşük aralık"), range={"max": 30.0}),
             prec("s_r, duplicates < 30", {"type": "duplicates", "pairs": NH4_LOW}, ("Duplicates of natural samples below 30 µg/L", "30 µg/L altındaki doğal numunelerin mükerrerleri"), range={"max": 30.0}),
             prec("u(Rw), control ≥ 30", {"type": "given", "sd_percent": 1.5}, ("Control sample, high range", "Kontrol numunesi, yüksek aralık"), range={"min": 30.0}),
             prec("s_r, duplicates ≥ 30", {"type": "duplicates", "pairs": NH4_HIGH, "relative": True}, ("Duplicates of natural samples at or above 30 µg/L", "30 µg/L ve üzerindeki doğal numunelerin mükerrerleri"), range={"min": 30.0})]},
        {"components": {"s_r, duplicates < 30": chk(0.44, 0.005), "s_r, duplicates ≥ 30": chk(3.8, 0.05)},
         "levels": [{"level": 10.0, "standard_uncertainty": chk(0.67, 0.005)},
                    {"level": 100.0, "relative_standard_percent": chk(4.1, 0.05)}]},
        ("Nordtest TR 537 §5.3 and Appendix 5 (s_r 0.44 µg/L and 3.8 %; u(Rw) 0.7 µg/L and 4.1 %).",
         "Nordtest TR 537 §5.3 ve Ek 5 (s_r 0,44 µg/L ve %3,8; u(Rw) 0,7 µg/L ve %4,1)."),
        notes=[("Appendix 5 lists 46 pairs below 30 µg/L; §5.3 states n = 47. The statistics are recomputed from the 46 listed pairs.",
                "Ek 5, 30 µg/L'nin altında 46 çift listeler; §5.3 n = 47 yazar. İstatistikler listelenen 46 çiftten yeniden hesaplanmıştır.")],
        tags=["nordtest", "duplicates", "ranges"])

project("nordtest-5.3-oxygen-duplicates",
        ("Dissolved oxygen: reproducibility from duplicates", "Çözünmüş oksijen: mükerrer ölçümlerden tekrar üretilebilirlik"),
        ("Nordtest TR 537 §5.3 and Appendix 6: dissolved oxygen in sea water, 51 duplicate pairs (raw data) and a control-sample component.",
         "Nordtest TR 537 §5.3 ve Ek 6: deniz suyunda çözünmüş oksijen, 51 mükerrer çift (ham veri) ve kontrol numunesi bileşeni."),
        [("s_r = √(Σd²/2N) = 0.0252 mg/L at a mean of 7.50 mg/L, i.e. 0.34 %.",
          "s_r = √(Σd²/2N) = 0,0252 mg/L, ortalama 7,50 mg/L'de, yani %0,34."),
         ("With the control-sample reproducibility (0.5 %) u(Rw) = √(0.34² + 0.5²) = 0.60 %.",
          "Kontrol numunesi tekrar üretilebilirliği (%0,5) ile u(Rw) = √(0,34² + 0,5²) = %0,60.")],
        [src(NT, "§5.3 and Appendix 6")],
        {"quantity": "c(O₂)", "unit": "mg/L",
         "precision": [
             prec("s_r, duplicates", {"type": "duplicates", "pairs": OXYGEN}, ("Duplicate titrations of sea-water samples", "Deniz suyu numunelerinin mükerrer titrasyonları"), to_relative_at="mean"),
             prec("u(Rw), control", {"type": "given", "sd_percent": 0.5}, ("Control sample", "Kontrol numunesi"))]},
        {"components": {"s_r, duplicates": chk(0.34, 0.005)},
         "levels": [{"relative_standard_percent": chk(0.60, 0.005)}]},
        ("Nordtest TR 537 §5.3 and Appendix 6 (s = 0.0252 mg/L, 0.34 %; u(Rw) = 0.60 %).",
         "Nordtest TR 537 §5.3 ve Ek 6 (s = 0,0252 mg/L, %0,34; u(Rw) = %0,60)."),
        tags=["nordtest", "duplicates"])

project("nordtest-6.1-one-crm",
        ("Bias from one certified reference material", "Tek sertifikalı referans malzemeden sapma"),
        ("Nordtest TR 537 §6.1: u(bias) from a CRM (11.5 ± 0.5) measured 12 times.",
         "Nordtest TR 537 §6.1: 12 kez ölçülmüş bir CRM'den (11,5 ± 0,5) u(bias)."),
        [("Bias b = (11.9 − 11.5)/11.5 = 3.48 %; s_bias = 2.2 % (n = 12); u(C_ref) = 0.25/11.5 = 2.17 %.",
          "Sapma b = (11,9 − 11,5)/11,5 = %3,48; s_bias = %2,2 (n = 12); u(C_ref) = 0,25/11,5 = %2,17."),
         ("For one reference material u(bias) = √(b² + (s_bias/√n)² + u(C_ref)²) = 4.15 %.",
          "Tek referans malzeme için u(bias) = √(b² + (s_bias/√n)² + u(C_ref)²) = %4,15.")],
        [src(NT, "§6.1")],
        {"quantity": "c", "bias": [{"name": "u(bias)", "kind": "crm", "references": [
            {"label": "CRM", "reference": 11.5, "expanded": 0.5, "k": 2, "mean": 11.9, "sd_percent": 2.2, "n": 12}]}]},
        {"components": {"u(bias)": chk(4.15, 0.005)}, "bias": {"u(bias)": {"mean": chk(3.48, 0.005), "mean_u_reference": chk(2.174, 0.0005)}}},
        ("Nordtest TR 537 §6.1, recomputed (the handbook prints 4.1 %, see notes).",
         "Nordtest TR 537 §6.1, yeniden hesaplanmış (el kitabı %4,1 yazar, notlara bakınız)."),
        notes=[("Finding (F5): the handbook prints u(C_ref) = 0.25/11.5 = 2.16 % but 0.25/11.5 = 2.17 %, and its result "
                "u(bias) = 4.1 % follows from 2.16 %; the correct value is 4.15 %.",
                "Bulgu (F5): el kitabı u(C_ref) = 0,25/11,5 = %2,16 yazar, oysa 0,25/11,5 = %2,17'dir; u(bias) = %4,1 sonucu "
                "%2,16'dan elde edilir, doğru değer %4,15'tir.")],
        tags=["nordtest", "crm", "bias"])

project("nordtest-6.1-three-crms",
        ("Bias from several certified reference materials", "Birden fazla sertifikalı referans malzemeden sapma"),
        ("Nordtest TR 537 §6.1: u(bias) from three CRMs by the RMS of the biases.",
         "Nordtest TR 537 §6.1: üç CRM'den sapmaların karesel ortalamasının karekökü (RMS) ile u(bias)."),
        [("Biases 3.48, −0.9 and 2.5 %: RMS_bias = √(Σbᵢ²/N) = 2.53 %; ū(C_ref) = (2.16 + 1.8 + 1.8)/3 = 1.92 %.",
          "Sapmalar %3,48, −0,9 ve 2,5: RMS_bias = √(Σbᵢ²/N) = %2,53; ū(C_ref) = (2,16 + 1,8 + 1,8)/3 = %1,92."),
         ("u(bias) = √(RMS_bias² + ū(C_ref)²) = 3.2 %.", "u(bias) = √(RMS_bias² + ū(C_ref)²) = %3,2.")],
        [src(NT, "§6.1")],
        {"quantity": "c", "bias": [{"name": "u(bias)", "kind": "crm", "references": [
            {"label": "CRM 1", "bias_percent": 3.48, "u_reference_percent": 2.16},
            {"label": "CRM 2", "bias_percent": -0.9, "u_reference_percent": 1.8},
            {"label": "CRM 3", "bias_percent": 2.5, "u_reference_percent": 1.8}]}]},
        {"components": {"u(bias)": chk(3.2, 0.05)},
         "bias": {"u(bias)": {"rms": chk(2.53, 0.005), "mean_u_reference": chk(1.92, 0.005)}}},
        ("Nordtest TR 537 §6.1 (RMS_bias 2.53 %, ū(C_ref) 1.92 %, u(bias) 3.2 %).",
         "Nordtest TR 537 §6.1 (RMS_bias %2,53, ū(C_ref) %1,92, u(bias) %3,2)."),
        tags=["nordtest", "crm", "bias"])

pt62 = [(2.0, None), (7.0, 2.9), (-2.0, 1.7), (3.0, 4.1), (6.0, 3.0), (5.0, 2.1)]
refs62 = []
for i, (b, u) in enumerate(pt62):
    r = {"label": f"PT {i + 1}", "bias_percent": b}
    if u is None:
        r.update({"sd_reproducibility_percent": 8.7, "labs": 23})
    else:
        r["u_reference_percent"] = u
    refs62.append(r)
project("nordtest-6.2-proficiency-tests",
        ("Bias from proficiency tests", "Yeterlilik testlerinden sapma"),
        ("Nordtest TR 537 §6.2: u(bias) from six proficiency tests.", "Nordtest TR 537 §6.2: altı yeterlilik testinden u(bias)."),
        [("u(C_ref,1) = s_R/√n_Lab = 8.7/√23 = 1.8 %; the other five values are 2.9, 1.7, 4.1, 3.0 and 2.1 %; ū(C_ref) = 2.6 %.",
          "u(C_ref,1) = s_R/√n_Lab = 8,7/√23 = %1,8; diğer beş değer %2,9, 1,7, 4,1, 3,0 ve 2,1; ū(C_ref) = %2,6."),
         ("Biases +2, +7, −2, +3, +6, +5 %: RMS_bias = 4.6 %; u(bias) = √(4.6² + 2.6²) = 5.3 %.",
          "Sapmalar +2, +7, −2, +3, +6, +5 %: RMS_bias = %4,6; u(bias) = √(4,6² + 2,6²) = %5,3.")],
        [src(NT, "§6.2")],
        {"quantity": "c", "bias": [{"name": "u(bias)", "kind": "proficiency_tests", "references": refs62}]},
        {"components": {"u(bias)": chk(5.3, 0.02)},
         "bias": {"u(bias)": {"rms": chk(4.6, 0.01), "mean_u_reference": chk(2.60, 0.005)}}},
        ("Nordtest TR 537 §6.2 (RMS_bias 4.6 %, ū(C_ref) 2.6 %, u(bias) 5.3 %).",
         "Nordtest TR 537 §6.2 (RMS_bias %4,6, ū(C_ref) %2,6, u(bias) %5,3)."),
        notes=[("Finding: the mean of the six u(C_ref,i) is 2.60 %; the handbook prints 2.7 % in the step table and then "
                "uses 2.6 %.", "Bulgu: altı u(C_ref,i) değerinin ortalaması %2,60'tır; el kitabı adım tablosunda %2,7 yazar, "
                "ardından %2,6 kullanır.")],
        tags=["nordtest", "proficiency-tests", "bias"])

project("nordtest-6.3-recovery",
        ("Bias from recovery experiments", "Geri kazanım deneylerinden sapma"),
        ("Nordtest TR 537 §6.3: u(bias) from six spike recoveries.", "Nordtest TR 537 §6.3: altı katkı geri kazanımından u(bias)."),
        [("Recoveries 95, 98, 97, 96, 99, 96 %: biases R − 100 %, RMS_bias = 3.44 %.",
          "Geri kazanımlar %95, 98, 97, 96, 99, 96: sapmalar R − %100, RMS_bias = %3,44."),
         ("Uncertainty of the added amount: concentration of the spike solution 1.2/2 = 0.6 %, volume (±1 % rectangular → "
          "1/√3 = 0.58 %, and 0.5 %) → u(C_recovery) = √(0.6² + 0.58² + 0.5²) = 0.97 % ≈ 1.0 %.",
          "Eklenen miktarın belirsizliği: katkı çözeltisi derişimi 1,2/2 = %0,6, hacim (±%1 dikdörtgen → 1/√3 = %0,58 ve "
          "%0,5) → u(C_geri kazanım) = √(0,6² + 0,58² + 0,5²) = %0,97 ≈ %1,0."),
         ("u(bias) = √(3.44² + 0.97²) = 3.6 %.", "u(bias) = √(3,44² + 0,97²) = %3,6.")],
        [src(NT, "§6.3")],
        {"quantity": "c", "bias": [{"name": "u(bias)", "kind": "recovery",
                                    "references": [{"label": f"R{i + 1}", "recovery_percent": r} for i, r in enumerate([95.0, 98.0, 97.0, 96.0, 99.0, 96.0])],
                                    "u_reference_components_percent": [0.6, 1 / math.sqrt(3), 0.5]}]},
        {"components": {"u(bias)": chk(3.6, 0.05)}, "bias": {"u(bias)": {"rms": chk(3.44, 0.005), "mean_u_reference": chk(1.0, 0.05)}}},
        ("Nordtest TR 537 §6.3 (RMS_bias 3.44 %, u(C_recovery) 1.0 %, u(bias) 3.6 %).",
         "Nordtest TR 537 §6.3 (RMS_bias %3,44, u(C_geri kazanım) %1,0, u(bias) %3,6)."),
        tags=["nordtest", "recovery", "bias"])

BOD_AV = [round((a + b) / 2, 6) for a, b in BOD]
bod_prec = prec("u(Rw)", {"type": "series", "values": BOD_AV, "relative": True},
                ("Averages of the duplicate control analyses of the CRM (Appendix 7)", "CRM'nin mükerrer kontrol analizlerinin ortalamaları (Ek 7)"))
project("nordtest-8-bod-crm",
        ("BOD in wastewater: control chart and one CRM", "Atık suda BOİ: kontrol grafiği ve tek CRM"),
        ("Nordtest TR 537 §8.2 Example A with the raw control data of Appendix 7.",
         "Nordtest TR 537 §8.2 Örnek A, Ek 7'deki ham kontrol verileriyle."),
        [("The averages of the 18 duplicate control analyses of the CRM give s = 5.6 mg/L, i.e. u(Rw) = 2.6 % at 214.8 mg/L.",
          "CRM'nin 18 mükerrer kontrol analizinin ortalamaları s = 5,6 mg/L, yani 214,8 mg/L'de u(Rw) = %2,6 verir."),
         ("The same results give the bias against the certified value (206 ± 5) mg/L: b = 4.3 %, u(C_ref) = 2.5/206 = 1.2 %, "
          "u(bias) = √(b² + s²/n + u(C_ref)²) = 4.5 %.",
          "Aynı sonuçlar sertifikalı değere (206 ± 5) mg/L karşı sapmayı verir: b = %4,3, u(C_ref) = 2,5/206 = %1,2, "
          "u(bias) = √(b² + s²/n + u(C_ref)²) = %4,5."),
         ("u_c = √(2.6² + 4.5²) = 5.2 %; U = 10.4 % → 11 %.", "u_c = √(2,6² + 4,5²) = %5,2; U = %10,4 → %11.")],
        [src(NT, "§8.2 Example A, Appendix 7")],
        {"quantity": "BOD", "unit": "mg/L", "precision": [bod_prec],
         "bias": [{"name": "u(bias)", "kind": "crm", "references": [{"label": "CRM", "reference": 206.0, "expanded": 5.0, "k": 2, "measured": BOD_AV}]}]},
        {"components": {"u(Rw)": chk(2.6, 0.05), "u(bias)": chk(4.5, 0.05)},
         "levels": [{"relative_standard_percent": chk(5.2, 0.05), "relative_expanded_percent": chk(10.4, 0.1)}]},
        ("Nordtest TR 537 §8.2 Example A (u(Rw) 2.6 %, u(bias) 4.5 %, u_c 5.2 %, U 10.4 → 11 %).",
         "Nordtest TR 537 §8.2 Örnek A (u(Rw) %2,6, u(bias) %4,5, u_c %5,2, U %10,4 → %11)."),
        reporting=WHOLE_PERCENT_UP,
        notes=[("The handbook states n = 19 for s_bias; Appendix 7 lists 18 pairs, which are used here. The averages are "
                "computed from the pairs (the appendix prints them rounded to 1 mg/L).",
                "El kitabı s_bias için n = 19 yazar; Ek 7 burada kullanılan 18 çifti listeler. Ortalamalar çiftlerden "
                "hesaplanmıştır (ek bunları 1 mg/L'ye yuvarlanmış verir).")],
        tags=["nordtest", "crm", "control-chart"])

project("nordtest-8-bod-pt",
        ("BOD in wastewater: control chart and proficiency tests", "Atık suda BOİ: kontrol grafiği ve yeterlilik testleri"),
        ("Nordtest TR 537 §8.2 Example B: u(bias) from three proficiency tests (Table 6, raw data).",
         "Nordtest TR 537 §8.2 Örnek B: üç yeterlilik testinden u(bias) (Tablo 6, ham veri)."),
        [("u(Rw) = 2.6 % as in Example A.", "u(Rw) = %2,6, Örnek A'daki gibi."),
         ("u(C_ref,i) = s_R/√n: 1.50, 1.32, 2.25 % (mean 1.69 %); RMS of the biases 3.77 % (3.76 % from the rounded biases "
          "of Table 6).",
          "u(C_ref,i) = s_R/√n: %1,50, 1,32, 2,25 (ortalama %1,69); sapmaların RMS'i %3,77 (Tablo 6'daki yuvarlanmış "
          "sapmalardan %3,76)."),
         ("u(bias) = 4.13 %, u_c = 4.88 %, U = 9.8 % → 10 %.", "u(bias) = %4,13, u_c = %4,88, U = %9,8 → %10.")],
        [src(NT, "§8.2 Example B, Table 6")],
        {"quantity": "BOD", "unit": "mg/L", "precision": [bod_prec],
         "bias": [{"name": "u(bias)", "kind": "proficiency_tests", "references": [
             {"label": f"PT {i + 1}", "reference": a, "mean": x, "sd_reproducibility_percent": s, "labs": n}
             for i, (a, x, s, n) in enumerate([(154.0, 161.0, 7.2, 23), (219.0, 210.0, 6.6, 25), (176.0, 180.0, 9.8, 19)])]}]},
        {"components": {"u(bias)": chk(4.12, 0.015)},
         "bias": {"u(bias)": {"rms": chk(3.77, 0.005), "mean_u_reference": chk(1.69, 0.005)}},
         "levels": [{"relative_standard_percent": chk(4.87, 0.015), "relative_expanded_percent": chk(9.74, 0.03)}]},
        ("Nordtest TR 537 §8.2 Example B (u(bias) 4.12 %, u_c 4.87 %, U 9.7 → 10 %); tolerances cover the rounded biases of Table 6.",
         "Nordtest TR 537 §8.2 Örnek B (u(bias) %4,12, u_c %4,87, U %9,7 → %10); toleranslar Tablo 6'daki yuvarlanmış sapmaları kapsar."),
        reporting=WHOLE_PERCENT_UP, tags=["nordtest", "proficiency-tests"])

project("nordtest-8-pcb-crm",
        ("PCB in sediment: control sample and one CRM", "Sedimentte PCB: kontrol numunesi ve tek CRM"),
        ("Nordtest TR 537 §8.3 Example C.", "Nordtest TR 537 §8.3 Örnek C."),
        [("u(Rw) = 8 % from the control chart; CRM (152 ± 14) µg/kg, mean result 144 µg/kg: b = −5.3 %, s_bias = 8 % (n = 22), "
          "u(C_ref) = 7/152 = 4.6 %.",
          "Kontrol grafiğinden u(Rw) = %8; CRM (152 ± 14) µg/kg, ortalama sonuç 144 µg/kg: b = −%5,3, s_bias = %8 (n = 22), "
          "u(C_ref) = 7/152 = %4,6."),
         ("u(bias) = 7.2 %, u_c = √(8² + 7.2²) = 10.8 %, U = 21.5 % → 22 %.", "u(bias) = %7,2, u_c = √(8² + 7,2²) = %10,8, U = %21,5 → %22.")],
        [src(NT, "§8.3 Example C")],
        {"quantity": "w(ΣPCB₇)", "unit": "µg/kg",
         "precision": [prec("u(Rw)", {"type": "given", "sd_percent": 8.0}, ("Control chart of a CRM", "Bir CRM'nin kontrol grafiği"))],
         "bias": [{"name": "u(bias)", "kind": "crm", "references": [
             {"label": "CRM", "reference": 152.0, "expanded": 14.0, "k": 2, "mean": 144.0, "sd_percent": 8.0, "n": 22}]}]},
        {"components": {"u(bias)": chk(7.22, 0.03)},
         "levels": [{"relative_standard_percent": chk(10.8, 0.05), "relative_expanded_percent": chk(21.6, 0.15)}]},
        ("Nordtest TR 537 §8.3 Example C (u(bias) 7.22 %, u_c 10.8 %, U 21.6 → 22 %); computed from 144 and 152 rather than the rounded 5.3 % and 4.6 %.",
         "Nordtest TR 537 §8.3 Örnek C (u(bias) %7,22, u_c %10,8, U %21,6 → %22); yuvarlanmış %5,3 ve %4,6 yerine 144 ve 152'den hesaplanmıştır."),
        reporting=WHOLE_PERCENT_UP,
        notes=[("From the stated values b = 5.26 % and u(C_ref) = 4.61 % give u(bias) = 7.20 %; the handbook uses the rounded "
                "5.3 % and 4.6 % (7.22 %). Its summary table prints u(C_ref) = 4.7 % and u(bias) = 7.29 %.",
                "Verilen değerlerden b = %5,26 ve u(C_ref) = %4,61 ile u(bias) = %7,20 bulunur; el kitabı yuvarlanmış %5,3 ve "
                "%4,6 değerlerini kullanır (%7,22). Özet tablosu u(C_ref) = %4,7 ve u(bias) = %7,29 yazar.")],
        tags=["nordtest", "crm"])

project("nordtest-8-pcb-pt",
        ("PCB in sediment: control sample and proficiency tests", "Sedimentte PCB: kontrol numunesi ve yeterlilik testleri"),
        ("Nordtest TR 537 §8.3 Example D.", "Nordtest TR 537 §8.3 Örnek D."),
        [("Biases −2, −12, −5 %: RMS_bias = 7.6 %; u(C_ref,i) 2.7, 2.5, 3.5 % → 2.9 %; u(bias) = 8.1 %.",
          "Sapmalar −2, −12, −5 %: RMS_bias = %7,6; u(C_ref,i) %2,7, 2,5, 3,5 → %2,9; u(bias) = %8,1."),
         ("u_c = √(8² + 8.1²) = 11.4 %, U = 22.8 % → 23 %.", "u_c = √(8² + 8,1²) = %11,4, U = %22,8 → %23.")],
        [src(NT, "§8.3 Example D")],
        {"quantity": "w(ΣPCB₇)", "unit": "µg/kg",
         "precision": [prec("u(Rw)", {"type": "given", "sd_percent": 8.0}, ("Control chart of an in-house material", "Laboratuvar içi malzemenin kontrol grafiği"))],
         "bias": [{"name": "u(bias)", "kind": "proficiency_tests", "references": [
             {"label": f"PT {i + 1}", "bias_percent": b, "u_reference_percent": u} for i, (b, u) in enumerate([(-2.0, 2.7), (-12.0, 2.5), (-5.0, 3.5)])]}]},
        {"components": {"u(bias)": chk(8.1, 0.05)}, "bias": {"u(bias)": {"rms": chk(7.6, 0.01), "mean_u_reference": chk(2.9, 0.005)}},
         "levels": [{"relative_standard_percent": chk(11.4, 0.01), "relative_expanded_percent": chk(22.8, 0.03)}]},
        ("Nordtest TR 537 §8.3 Example D (RMS_bias 7.6 %, u(bias) 8.1 %, u_c 11.4 %, U 22.8 → 23 %).",
         "Nordtest TR 537 §8.3 Örnek D (RMS_bias %7,6, u(bias) %8,1, u_c %11,4, U %22,8 → %23)."),
        reporting=WHOLE_PERCENT_UP, tags=["nordtest", "proficiency-tests"])

# ------------------------------------------------------------------ MUPR
REC_EXPL = ("The mean recovery R̄ = ΣRᵢ/N has u(R̄) = √Σu²(Rᵢ)/N (MUPR Eq. 8–12); it is corrected for only if |1 − R̄|/u(R̄) "
            "exceeds the two-sided t value (Eq. 13).",
            "Ortalama geri kazanım R̄ = ΣRᵢ/N'nin belirsizliği u(R̄) = √Σu²(Rᵢ)/N'dir (MUPR Eş. 8–12); yalnızca |1 − R̄|/u(R̄) "
            "iki taraflı t değerini aşarsa düzeltme yapılır (Eş. 13).")
project("mupr-ex2-reference-materials",
        ("Recovery from two CRMs and a proficiency-test sample", "İki CRM ve bir yeterlilik testi numunesinden geri kazanım"),
        ("MUPR Example 2: ortho-phosphate; mean recovery and its uncertainty from three reference materials.",
         "MUPR Örnek 2: orto-fosfat; üç referans malzemeden ortalama geri kazanım ve belirsizliği."),
        [("Rᵢ = c̄ᵢ/Cᵢ with u²(Rᵢ) = Rᵢ²[(sᵢ/(c̄ᵢ√nᵢ))² + (u(Cᵢ)/Cᵢ)²] (Eq. 9); terms 0.000731, 0.001476, 0.01015.",
          "Rᵢ = c̄ᵢ/Cᵢ, u²(Rᵢ) = Rᵢ²[(sᵢ/(c̄ᵢ√nᵢ))² + (u(Cᵢ)/Cᵢ)²] (Eş. 9); terimler 0,000731, 0,001476, 0,01015."),
         ("R̄ = 1.008, u(R̄) = 0.0371: not significantly different from 1.", "R̄ = 1,008, u(R̄) = 0,0371: 1'den anlamlı olarak farklı değildir."), REC_EXPL],
        [src(MUPR, "Example 2")],
        {"quantity": "c(PO₄-P)", "unit": "mg/L", "recovery": {"name": "u(R̄)", "tests": [
            {"type": "reference", "label": "CRM 1", "mean": 2.31, "sd": 0.11, "n": 12, "reference": 2.24, "expanded": 0.10, "k": 2},
            {"type": "reference", "label": "CRM 2", "mean": 1.032, "sd": 0.090, "n": 10, "reference": 1.000, "expanded": 0.050, "k": 2},
            {"type": "reference", "label": "PT", "mean": 1.70, "sd": 0.14, "n": 1, "reference": 1.77, "expanded": 0.23, "k": 2}]}},
        {"recovery": {"mean_percent": chk(100.8, 0.05), "standard_uncertainty_percent": chk(3.705, 0.001), "significant": False}},
        ("MUPR Example 2 (R̄ = 1.008, u(R̄) = 0.03705).", "MUPR Örnek 2 (R̄ = 1,008, u(R̄) = 0,03705)."),
        tags=["mupr", "recovery"])

project("mupr-ex3-spiked-samples",
        ("Recovery from spiked river waters", "Katkılı nehir sularından geri kazanım"),
        ("MUPR Example 3: two spiked samples with native analyte (Eq. 10–11).",
         "MUPR Örnek 3: doğal analit içeren iki katkılı numune (Eş. 10–11)."),
        [("Rᵢ = (c̄ − c̄₀)/c₊: 103.6 % and 88.8 %; the uncertainty of B (one result before spiking, small spike) dominates.",
          "Rᵢ = (c̄ − c̄₀)/c₊: %103,6 ve %88,8; B'nin belirsizliği (katkıdan önce tek sonuç, küçük katkı) baskındır."),
         ("R̄ = 96.2 %, u(R̄) = 10.0 % (1.5 % with sample A alone).", "R̄ = %96,2, u(R̄) = %10,0 (yalnız A numunesiyle %1,5)."), REC_EXPL],
        [src(MUPR, "Example 3")],
        {"quantity": "c", "unit": "µg/L", "recovery": {"name": "u(R̄)", "tests": [
            {"type": "spiked", "label": "A", "mean_after": 66.30, "sd_after": 1.405, "n_after": 16, "mean_before": 35.21,
             "sd_before": 1.054, "n_before": 16, "spike": 30.00, "u_spike": 0.10},
            {"type": "spiked", "label": "B", "mean_after": 64.02, "sd_after": 1.934, "n_after": 5, "mean_before": 55.14,
             "sd_before": 1.802, "n_before": 1, "spike": 10.000, "u_spike": 0.040}]}},
        {"recovery": {"mean_percent": chk(96.2, 0.05), "standard_uncertainty_percent": chk(10.02, 0.01)}},
        ("MUPR Example 3 (R̄ = 96.2 %, u(R̄) = 10.02 %).", "MUPR Örnek 3 (R̄ = %96,2, u(R̄) = %10,02)."),
        tags=["mupr", "recovery", "spike"])

DSS = {"name": "u′(DSS)", "description": T("Stock solution, used for calibrators and spikes (counted twice)", "Stok çözelti; kalibrantlar ve katkılar için kullanılır (iki kez sayılır)"),
       "u_percent": 0.95, "multiplicity": 2}
B1_REC = {"name": "u(R̄)", "tests": [{"type": "reference", "label": "spiked waters", "mean": 0.3890, "sd": 0.0328, "n": 20,
                                     "reference": 0.400, "expanded": 0.018, "k": 2}]}
project("mupr-b1-nitrate",
        ("Nitrate in drinking water: two precision intervals", "İçme suyunda nitrat: iki kesinlik aralığı"),
        ("MUPR Example B1: absolute precision below 0.4 mg/L, relative above; recovery from 20 spiked waters; stock solution and dilution components.",
         "MUPR Örnek B1: 0,4 mg/L altında mutlak, üstünde bağıl kesinlik; 20 katkılı sudan geri kazanım; stok çözelti ve seyreltme bileşenleri."),
        [("Interval I (c < 0.4 mg/L): s_I = 0.0328 mg/L; interval II: s′_I = 8.43 % (with dilution F = 100, u′_F = 2.2 %).",
          "Aralık I (c < 0,4 mg/L): s_I = 0,0328 mg/L; aralık II: s′_I = %8,43 (F = 100 seyreltme, u′_F = %2,2)."),
         ("R̄ = 0.389/0.400 = 97.25 %, u(R̄) = 2.85 %; |1 − R̄|/u = 0.96 ≤ t₁₉ = 2.09: no correction, u′ = u(R̄).",
          "R̄ = 0,389/0,400 = %97,25, u(R̄) = %2,85; |1 − R̄|/u = 0,96 ≤ t₁₉ = 2,09: düzeltme yok, u′ = u(R̄)."),
         ("U = 2√(s² + c²(u′²_R + 2u′²_DSS [+ u′²_F])): (0.261 ± 0.068) mg/L and (50.0 ± 9.3) mg/L.",
          "U = 2√(s² + c²(u′²_R + 2u′²_DSS [+ u′²_F])): (0,261 ± 0,068) mg/L ve (50,0 ± 9,3) mg/L.")],
        [src(MUPR, "Example B1")],
        {"quantity": "c(NO₃⁻)", "unit": "mg/L", "levels": [0.261, 50.0],
         "precision": [prec("s_I⟨I⟩", {"type": "given", "sd": 0.0328, "dof": 19}, range={"max": 0.4}),
                       prec("s′_I⟨II⟩", {"type": "given", "sd_percent": 8.43, "dof": 19}, range={"min": 0.4})],
         "recovery": B1_REC,
         "components": [DSS, {"name": "u′(F)", "description": T("Dilution of samples above 0.4 mg/L", "0,4 mg/L üzerindeki numunelerin seyreltilmesi"),
                              "u_percent": 2.2, "range": {"min": 0.4}}]},
        {"recovery": {"mean_percent": chk(97.25, 0.005), "standard_uncertainty_percent": chk(2.85, 0.005),
                      "statistic": chk(0.963, 0.0005), "critical": chk(2.093, 0.0005), "significant": False},
         "levels": [{"level": 0.261, "expanded_uncertainty": chk(0.0676, 0.00005)}, {"level": 50.0, "expanded_uncertainty": chk(9.26, 0.01)}]},
        ("MUPR Example B1, Eq. (B1.3) and B1.7.iv (U = 0.0676 mg/L at 0.261 mg/L; 9.26 mg/L at 50 mg/L, computed by the guide with u′(R̄) rounded to 0.0285; unrounded 9.27 mg/L).",
         "MUPR Örnek B1, Eş. (B1.3) ve B1.7.iv (0,261 mg/L'de U = 0,0676 mg/L; 50 mg/L'de 9,26 mg/L, rehber u′(R̄)'yi 0,0285'e yuvarlayarak hesaplar; yuvarlanmamış 9,27 mg/L)."),
        notes=[("Finding: the guide prints |1 − R̄|/u = 0.982 using R̄ rounded to 0.972; with R̄ = 0.9725 the statistic is 0.963. "
                "The conclusion is the same.", "Bulgu: rehber R̄'yi 0,972'ye yuvarlayarak |1 − R̄|/u = 0,982 yazar; R̄ = 0,9725 ile "
                "istatistik 0,963'tür. Sonuç aynıdır.")],
        tags=["mupr", "recovery", "ranges"])

B2_G2 = [0.288, 0.345, 0.315, 0.272, 0.303, 0.297]
B2_G4 = [0.773, 0.732, 0.772, 0.817, 0.803, 0.812]
B2_PREC = [prec("s_I(P)⟨I⟩", {"type": "groups", "groups": [{"label": "1", "sd": 0.0328, "n": 20}, {"label": "2", "values": B2_G2}, {"label": "3", "sd": 0.0245, "n": 20}]},
                ("Pooled intermediate precision, three low-level materials", "Birleştirilmiş ara kesinlik, üç düşük düzeyli malzeme"), range={"max": 0.37}),
           prec("s′_I(P)⟨II⟩", {"type": "groups", "relative": True, "groups": [{"label": "1", "sd_percent": 8.43, "n": 20}, {"label": "4", "values": B2_G4}]},
                ("Pooled relative intermediate precision", "Birleştirilmiş bağıl ara kesinlik"), range={"min": 0.37})]
B2_REC = {"name": "u(R̿)", "tests": [
    {"type": "reference", "label": "0.4 mg/L", "mean": 0.389, "sd": 0.0328, "n": 20, "reference": 0.4, "u_reference": 0.009},
    {"type": "reference", "label": "0.2 mg/L", "mean": 0.204, "sd": 0.0245, "n": 20, "reference": 0.2, "u_reference": 0.0041}]}
B2_COMP = [DSS, {"name": "u′(F)", "u_percent": 2.2, "range": {"min": 0.37}}]
project("mupr-b2-pooled-precision",
        ("Nitrate: pooled precision and two recovery levels", "Nitrat: birleştirilmiş kesinlik ve iki geri kazanım düzeyi"),
        ("MUPR Example B2: precision pooled from several materials (raw data for two of them), transition concentration and overall recovery.",
         "MUPR Örnek B2: birden fazla malzemeden birleştirilmiş kesinlik (ikisi için ham veri), geçiş derişimi ve genel geri kazanım."),
        [("Pooled s_I⟨I⟩ = 0.0285 mg/L (Eq. B2.1) and s′_I⟨II⟩ = 7.73 % (Eq. B2.2); the two models meet at c_T = 0.0285/0.0773 = 0.37 mg/L.",
          "Birleştirilmiş s_I⟨I⟩ = 0,0285 mg/L (Eş. B2.1) ve s′_I⟨II⟩ = %7,73 (Eş. B2.2); iki model c_T = 0,0285/0,0773 = 0,37 mg/L'de buluşur."),
         ("Recoveries 97.2 % and 102.0 % are compatible; R̿ = 99.6 %, u = 2.24 %, not significant (38 degrees of freedom).",
          "%97,2 ve %102,0 geri kazanımları uyumludur; R̿ = %99,6, u = %2,24, anlamlı değil (38 serbestlik derecesi)."),
         ("At 50 mg/L: U = 0.169 c = 8.45 mg/L.", "50 mg/L'de: U = 0,169 c = 8,45 mg/L.")],
        [src(MUPR, "Example B2, Tables B2.1–B2.3")],
        {"quantity": "c(NO₃⁻)", "unit": "mg/L", "levels": [50.0], "precision": B2_PREC, "recovery": B2_REC, "components": B2_COMP},
        {"components": {"s_I(P)⟨I⟩": chk(0.0285, 0.00005), "s′_I(P)⟨II⟩": chk(7.73, 0.005)},
         "recovery": {"mean_percent": chk(99.6, 0.05), "standard_uncertainty_percent": chk(2.24, 0.005), "significant": False},
         "levels": [{"level": 50.0, "expanded_uncertainty": chk(8.45, 0.005)}]},
        ("MUPR Example B2, Eq. (B2.1)–(B2.7).", "MUPR Örnek B2, Eş. (B2.1)–(B2.7)."),
        notes=[("The guide divides the rounded s_I(2) = 0.0250 by 0.3033 (8.24 %); unrounded 8.25 %. Group 2 enters the absolute pool, "
                "so the budget is unaffected.", "Rehber yuvarlanmış s_I(2) = 0,0250'yi 0,3033'e böler (%8,24); yuvarlanmamış %8,25. "
                "Grup 2 mutlak birleştirmeye girdiğinden bütçe etkilenmez.")],
        tags=["mupr", "pooled-precision", "recovery"])

B2_PREC_DAYS = [dict(B2_PREC[1], mean_of={"replicates": 3, "days": 2, "repeatability_percent": 3.4})]
project("mupr-b2-replicates-on-two-days",
        ("Nitrate: result as the mean of 3 replicates on 2 days", "Nitrat: 2 günde 3 tekrarın ortalaması olarak sonuç"),
        ("MUPR Example B2, Eq. (B2.9): precision of a mean of n results on each of p days (Eq. 5).",
         "MUPR Örnek B2, Eş. (B2.9): p günün her birinde n sonucun ortalamasının kesinliği (Eş. 5)."),
        [("s′_I(x̿) = √(s′_I²/p + s′_r²(1 − n)/(n p)) with s′_I = 7.73 %, s′_r = 3.4 %, n = 3, p = 2.",
          "s′_I(x̿) = √(s′_I²/p + s′_r²(1 − n)/(n p)); s′_I = %7,73, s′_r = %3,4, n = 3, p = 2."),
         ("At 57.5 mg/L: U = 7.06 mg/L (7.41 mg/L for one result on each of two days).",
          "57,5 mg/L'de: U = 7,06 mg/L (iki günün her birinde tek sonuç için 7,41 mg/L).")],
        [src(MUPR, "Example B2, Eq. (B2.8)–(B2.9)")],
        {"quantity": "c(NO₃⁻)", "unit": "mg/L", "levels": [57.5], "precision": B2_PREC_DAYS, "recovery": B2_REC,
         "components": [DSS, {"name": "u′(F)", "u_percent": 2.2}]},
        {"levels": [{"level": 57.5, "expanded_uncertainty": chk(7.06, 0.005)}]},
        ("MUPR Example B2, Eq. (B2.9) (U = 7.06 mg/L).", "MUPR Örnek B2, Eş. (B2.9) (U = 7,06 mg/L)."),
        tags=["mupr", "replicates"])

project("mupr-b3-linear-precision",
        ("Nitrate: precision as a linear function of concentration", "Nitrat: derişimin doğrusal fonksiyonu olarak kesinlik"),
        ("MUPR Example B3: the intermediate precision of four materials is modelled as s_I = a + b·c by least squares and applied to a diluted sample.",
         "MUPR Örnek B3: dört malzemenin ara kesinliği en küçük kareler ile s_I = a + b·c olarak modellenir ve seyreltilmiş bir numuneye uygulanır."),
        [("Table B3.1: (c, s_I) = (0.389, 0.0328), (0.303, 0.0250), (0.204, 0.0245), (0.785, 0.0322) mg/L give "
          "s_I = 0.0232 + 0.0128 c (r = 0.727).",
          "Tablo B3.1: (c, s_I) = (0,389; 0,0328), (0,303; 0,0250), (0,204; 0,0245), (0,785; 0,0322) mg/L, "
          "s_I = 0,0232 + 0,0128 c (r = 0,727) verir."),
         ("The sample is diluted 100-fold, so the model is applied to the test solution: s(c_s) = 100·(a + b·c_s/100); at "
          "c_s = 57.5 mg/L, s = 3.06 mg/L.",
          "Numune 100 kat seyreltildiğinden model test çözeltisine uygulanır: s(c_s) = 100·(a + b·c_s/100); c_s = 57,5 mg/L'de "
          "s = 3,06 mg/L."),
         ("With the recovery (2.24 %), stock solution (2 × 0.95 %) and dilution (2.2 %) components: U = 7.28 mg/L.",
          "Geri kazanım (%2,24), stok çözelti (2 × %0,95) ve seyreltme (%2,2) bileşenleriyle: U = 7,28 mg/L.")],
        [src(MUPR, "Example B3, Table B3.1, Eq. (B3.1)")],
        {"quantity": "c(NO₃⁻)", "unit": "mg/L", "levels": [57.5],
         "precision": [prec("s_I(c)", {"type": "linear", "points": [[0.3890, 0.0328], [0.3033, 0.0250], [0.2040, 0.0245], [0.7848, 0.0322]],
                                       "dilution_factor": 100},
                            ("Linear precision model fitted to four materials (test-solution concentrations)", "Dört malzemeye uydurulmuş doğrusal kesinlik modeli (test çözeltisi derişimleri)"))],
         "recovery": B2_REC, "components": [DSS, {"name": "u′(F)", "u_percent": 2.2}]},
        {"levels": [{"level": 57.5, "expanded_uncertainty": chk(7.28, 0.01)}]},
        ("MUPR Example B3, Eq. (B3.1) (U = 7.28 mg/L with the printed intercept 0.0233; 7.27 mg/L with the least-squares 0.02324).",
         "MUPR Örnek B3, Eş. (B3.1) (basılı kesim noktası 0,0233 ile U = 7,28 mg/L; en küçük kareler değeri 0,02324 ile 7,27 mg/L)."),
        notes=[("Finding: least squares gives the intercept 0.02324 (printed 0.0233); slope 0.0128 and r = 0.7266 agree.",
                "Bulgu: en küçük kareler kesim noktasını 0,02324 verir (basılı 0,0233); eğim 0,0128 ve r = 0,7266 uyumludur.")],
        tags=["mupr", "precision-model", "linear"])

B4_PREC = {"type": "groups", "relative": True, "groups": [{"label": "A1", "sd_percent": 2.93, "n": 10}, {"label": "B", "sd_percent": 5.46, "n": 10}, {"label": "C", "sd_percent": 4.83, "n": 9}]}
B4_CRM = {"type": "reference", "label": "WQB-1", "mean": 21.040, "sd": 0.779, "n": 10, "reference": 23.00, "expanded": 1.84, "k": 2}
project("mupr-b4-arsenic-one-crm",
        ("Arsenic in sediment: one CRM", "Sedimentte arsenik: tek CRM"),
        ("MUPR Example B4: pooled relative precision and the recovery of one CRM.", "MUPR Örnek B4: birleştirilmiş bağıl kesinlik ve tek CRM'nin geri kazanımı."),
        [("s′_I(P) = 4.52 % pooled from three materials (Eq. B4.1); below 6.46 mg/kg it is used as an absolute 4.52 % × 6.46 = 0.292 mg/kg.",
          "Üç malzemeden birleştirilmiş s′_I(P) = %4,52 (Eş. B4.1); 6,46 mg/kg altında mutlak 4,52 % × 6,46 = 0,292 mg/kg olarak kullanılır."),
         ("CRM WQB-1 (23.00 ± 1.84) mg/kg, mean 21.040 mg/kg: R̄ = 91.5 %, u = 3.81 %; 2.24 ≤ t₉ = 2.26: not corrected.",
          "CRM WQB-1 (23,00 ± 1,84) mg/kg, ortalama 21,040 mg/kg: R̄ = %91,5, u = %3,81; 2,24 ≤ t₉ = 2,26: düzeltilmez."),
         ("U = 2w√(0.0452² + 0.0381²) = 0.118 w: (16.0 ± 1.9) mg/kg.", "U = 2w√(0,0452² + 0,0381²) = 0,118 w: (16,0 ± 1,9) mg/kg.")],
        [src(MUPR, "Example B4")],
        {"quantity": "w(As)", "unit": "mg/kg", "levels": [16.0],
         "precision": [prec("s_I⟨I⟩", B4_PREC, to_absolute_at=6.46, range={"max": 6.46}), prec("s′_I⟨II⟩", B4_PREC, range={"min": 6.46})],
         "recovery": {"name": "u(R̄)", "tests": [B4_CRM]}},
        {"components": {"s_I⟨I⟩": chk(0.292, 0.0005), "s′_I⟨II⟩": chk(4.52, 0.005)},
         "recovery": {"mean_percent": chk(91.48, 0.005), "standard_uncertainty_percent": chk(3.81, 0.005), "statistic": chk(2.24, 0.005),
                      "critical": chk(2.26, 0.005), "significant": False},
         "levels": [{"level": 16.0, "expanded_uncertainty": chk(1.89, 0.005), "relative_expanded_percent": chk(11.8, 0.05)}]},
        ("MUPR Example B4, Eq. (B4.1)–(B4.5).", "MUPR Örnek B4, Eş. (B4.1)–(B4.5)."),
        tags=["mupr", "crm", "recovery"])

project("mupr-b5-crm-and-spiked-samples",
        ("Arsenic: CRM and spiked samples with matrix-dependent recovery", "Arsenik: matrise bağlı geri kazanımlı CRM ve katkılı numuneler"),
        ("MUPR Example B5: recoveries vary with the matrix, so their standard deviation is included and k = t(2).",
         "MUPR Örnek B5: geri kazanımlar matrisle değiştiğinden standart sapmaları dahil edilir ve k = t(2)."),
        [("R̿ = 94.98 % from the CRM and two spiked sediments; s(R) = 7.32 %, u(R̿) with s(R) = 8.00 % (Eq. B5.1–B5.3).",
          "CRM ve iki katkılı sedimentten R̿ = %94,98; s(R) = %7,32, s(R) dahil u(R̿) = %8,00 (Eş. B5.1–B5.3)."),
         ("|1 − R̿|/u = 0.63 ≤ t₂ = 4.30: not corrected; with ν = 2 the coverage factor is t₀.₉₇₅(2) = 4.30 and U = 0.395 w.",
          "|1 − R̿|/u = 0,63 ≤ t₂ = 4,30: düzeltilmez; ν = 2 ile kapsam faktörü t₀,₉₇₅(2) = 4,30 ve U = 0,395 w.")],
        [src(MUPR, "Example B5")],
        {"quantity": "w(As)", "unit": "mg/kg", "levels": [16.0], "precision": [prec("s′_I⟨II⟩", B4_PREC)],
         "recovery": {"name": "u(R̿)", "include_sd_of_recoveries": True, "tests": [
             B4_CRM,
             {"type": "spiked", "label": "A", "mean_after": 22.546, "sd_after": 0.782, "n_after": 10, "mean_before": 15.79,
              "sd_before": 0.106, "n_before": 10, "spike": 7.501, "u_spike_percent": 0.93},
             {"type": "spiked", "label": "C", "mean_after": 14.31, "sd_after": 1.83, "n_after": 9, "mean_before": 6.461,
              "sd_before": 0.312, "n_before": 9, "spike": 7.591, "u_spike_percent": 0.93}]},
         "coverage_probability": 0.95, "coverage_dof": 2},
        {"recovery": {"mean_percent": chk(94.98, 0.005), "sd_percent": chk(7.32, 0.005), "standard_uncertainty_percent": chk(8.00, 0.005),
                      "statistic": chk(0.6275, 0.0005), "critical": chk(4.30, 0.005), "significant": False},
         "levels": [{"level": 16.0, "relative_expanded_percent": chk(39.5, 0.1)}]},
        ("MUPR Example B5, Eq. (B5.1)–(B5.6) (U = 0.395 w with t rounded to 4.30; with t₀.₉₇₅(2) = 4.303, 0.3955 w).",
         "MUPR Örnek B5, Eş. (B5.1)–(B5.6) (t 4,30'a yuvarlanarak U = 0,395 w; t₀,₉₇₅(2) = 4,303 ile 0,3955 w)."),
        tags=["mupr", "recovery", "spike", "matrix-effects"])

prec_b6 = lambda w: 0.0734 * w if w >= 7.5 else 7.5 * 0.0734
project("mupr-b6-sediment-22-pt",
        ("Trace element in sediment: heterogeneity and 22 proficiency tests", "Sedimentte iz element: heterojenlik ve 22 yeterlilik testi"),
        ("MUPR Example B6: precision from a homogeneous material plus the heterogeneity of real samples (duplicate ranges), recovery from 22 PT results (raw data).",
         "MUPR Örnek B6: homojen bir malzemeden kesinlik ve gerçek numunelerin heterojenliği (mükerrer açıklıklar), 22 YT sonucundan geri kazanım (ham veri)."),
        [("A homogeneous material measured 30 times gives s′_I(DS) = 7.34 %; the ranges of 30 duplicate real samples give "
          "s_r(h) = Ā/1.128 = 0.191 mg/kg below 7.5 mg/kg and s′_r(h) = 2.66 % above.",
          "30 kez ölçülen homojen bir malzeme s′_I(DS) = %7,34 verir; 30 mükerrer gerçek numunenin açıklıkları 7,5 mg/kg altında "
          "s_r(h) = Ā/1,128 = 0,191 mg/kg ve üstünde s′_r(h) = %2,66 verir."),
         ("Interval I: s_I = √((7.5 × 7.34 %)² + 0.191²) = 0.583 mg/kg; interval II: s′_I = √(7.34² + 2.66²) = 7.81 %.",
          "Aralık I: s_I = √((7,5 × %7,34)² + 0,191²) = 0,583 mg/kg; aralık II: s′_I = √(7,34² + 2,66²) = %7,81."),
         ("The 22 PT recoveries are equivalent (χ² = 8.70 ≤ 32.7), so s(R) is not added; R̿ = 91.7 %, u = 1.59 %, "
          "|1 − R̿|/u = 5.2 > t₂₁: results are corrected (w/R̿) and u′ = u/R̿.",
          "22 YT geri kazanımı eşdeğerdir (χ² = 8,70 ≤ 32,7), bu nedenle s(R) eklenmez; R̿ = %91,7, u = %1,59, "
          "|1 − R̿|/u = 5,2 > t₂₁: sonuçlar düzeltilir (w/R̿) ve u′ = u/R̿."),
         ("Interval II: U = 2(w/R̿)√(0.0781² + 0.0173²) = 0.160 w/R̿.", "Aralık II: U = 2(w/R̿)√(0,0781² + 0,0173²) = 0,160 w/R̿.")],
        [src(MUPR, "Example B6, Tables B6.1–B6.3")],
        {"quantity": "w", "unit": "mg/kg", "levels": [5.0, 20.0],
         "precision": [
             prec("s_I(DS)⟨I⟩", {"type": "series", "values": B6_DS, "relative": True}, ("Homogeneous material, 30 results, at 7.5 mg/kg", "Homojen malzeme, 30 sonuç, 7,5 mg/kg'da"), to_absolute_at=7.5, range={"max": 7.5}),
             prec("s_r(h)⟨I⟩", {"type": "duplicates", "pairs": B6_DUP, "method": "mean_range", "max_mean": 7.5}, ("Duplicate real samples below 7.5 mg/kg", "7,5 mg/kg altındaki mükerrer gerçek numuneler"), range={"max": 7.5}),
             prec("s′_I(DS)⟨II⟩", {"type": "series", "values": B6_DS, "relative": True}, ("Homogeneous material, 30 results", "Homojen malzeme, 30 sonuç"), range={"min": 7.5}),
             prec("s′_r(h)⟨II⟩", {"type": "duplicates", "pairs": B6_DUP, "method": "mean_range", "relative": True, "min_mean": 7.5}, ("Duplicate real samples at or above 7.5 mg/kg", "7,5 mg/kg ve üzerindeki mükerrer gerçek numuneler"), range={"min": 7.5})],
         "recovery": {"name": "u(R̿)", "tests": [
             {"type": "reference", "label": f"PT {i + 1}", "mean": w, "sd": round(prec_b6(w), 6), "n": 1, "reference": W, "u_reference": u}
             for i, (w, W, u, _, _) in enumerate(B6_PT)]}},
        {"components": {"s′_I(DS)⟨II⟩": chk(7.34, 0.005), "s′_r(h)⟨II⟩": chk(2.66, 0.005), "s_r(h)⟨I⟩": chk(0.1912, 0.00005)},
         "recovery": {"mean_percent": chk(91.735, 0.0005), "standard_uncertainty_percent": chk(1.585, 0.001), "sd_percent": chk(4.607, 0.0005),
                      "statistic": chk(5.21, 0.01), "chi_squared": chk(8.70, 0.005), "significant": True},
         "levels": [{"level": 20.0, "value": chk(20.0 / 0.91735, 0.001), "relative_expanded_percent": chk(16.0, 0.05)}]},
        ("MUPR Example B6, Tables B6.1–B6.3 and Eq. (B6.1)–(B6.9); χ² recomputed with the weighted mean of Eq. (16).",
         "MUPR Örnek B6, Tablo B6.1–B6.3 ve Eş. (B6.1)–(B6.9); χ², Eş. (16)'daki ağırlıklı ortalama ile yeniden hesaplanmıştır."),
        notes=[("Finding: Eq. (B6.7) computes χ² about the unweighted mean (8.80); Eq. (16) prescribes the weighted mean, giving 8.70. "
                "The conclusion is the same.", "Bulgu: Eş. (B6.7) χ²'yi ağırlıksız ortalamaya göre hesaplar (8,80); Eş. (16) ağırlıklı "
                "ortalamayı öngörür ve 8,70 verir. Sonuç aynıdır."),
               ("Finding: Eq. (B6.8) as printed omits 1/R̿ on the precision term of the corrected result; for absolute components "
                "the program uses u(w/R̿)² = (s_I/R̿)² + (w u_R̿/R̿²)². In interval II (all relative) both agree.",
                "Bulgu: Eş. (B6.8) basılı haliyle düzeltilmiş sonucun kesinlik teriminde 1/R̿'yi atlar; program mutlak bileşenler "
                "için u(w/R̿)² = (s_I/R̿)² + (w u_R̿/R̿²)² kullanır. Aralık II'de (tümü bağıl) ikisi aynıdır."),
               ("The standard deviation of each single PT result is taken from the homogeneous-material model (7.34 % above "
                "7.5 mg/kg, 7.5 × 7.34 % below), as in Table B6.3.",
                "Her tekil YT sonucunun standart sapması, Tablo B6.3'teki gibi homojen malzeme modelinden alınmıştır (7,5 mg/kg "
                "üzerinde %7,34, altında 7,5 × %7,34).")],
        tags=["mupr", "proficiency-tests", "heterogeneity", "recovery-correction"])

# ------------------------------------------------------------------ QUAM
project("quam-a4-pesticides-topdown",
        ("Organophosphorus pesticides in bread from validation data", "Validasyon verilerinden ekmekte organofosforlu pestisitler"),
        ("EURACHEM QUAM Example A4 as a top-down evaluation: precision from 15 duplicates (raw data), recovery from 42 spikes, homogeneity.",
         "EURACHEM QUAM Örnek A4, top-down değerlendirme olarak: 15 mükerrer ölçümden kesinlik (ham veri), 42 katkıdan geri kazanım, homojenlik."),
        [("Duplicates measured in different runs: the standard deviation of the normalised differences (x₁ − x₂)/x̄ is "
          "0.382, so the relative intermediate precision is 0.382/√2 = 27 %.",
          "Farklı serilerde ölçülen mükerrerler: normalize farkların (x₁ − x₂)/x̄ standart sapması 0,382'dir; bağıl ara "
          "kesinlik 0,382/√2 = %27'dir."),
         ("Recovery: mean 90 %, s = 28 % from 42 spikes, u = 28/√42 = 4.3 %; t = 2.31 > t₄₁ = 2.02: the result is corrected "
          "and u′ = 4.3/90 = 4.8 %.",
          "Geri kazanım: 42 katkıdan ortalama %90, s = %28, u = 28/√42 = %4,3; t = 2,31 > t₄₁ = 2,02: sonuç düzeltilir ve "
          "u′ = 4,3/90 = %4,8."),
         ("Inhomogeneity of the loaf (binomial model, scenario b): 20 %.", "Somunun homojensizliği (binom model, senaryo b): %20."),
         ("At a measured level of 1: P = 1/0.90 = 1.111 with u = 1.111 × √(0.27² + 0.048² + 0.20²) = 0.377.",
          "Ölçülen düzey 1'de: P = 1/0,90 = 1,111, u = 1,111 × √(0,27² + 0,048² + 0,20²) = 0,377.")],
        [src(QUAM, "Example A4, Tables A4.2–A4.5")],
        {"quantity": "P_op", "levels": [1.0],
         "precision": [prec("precision", {"type": "duplicates", "pairs": A4_DUP, "relative": True, "method": "normalised_differences"},
                            ("15 duplicate pairs in different runs (Table A4.2)", "Farklı serilerde 15 mükerrer çift (Tablo A4.2)"))],
         "recovery": {"name": "recovery", "tests": [{"type": "recoveries", "label": "bread, 42 spikes", "mean_percent": 90.0, "sd_percent": 28.0, "n": 42}]},
         "components": [{"name": "homogeneity", "description": T("Binomial sampling model, scenario (b)", "Binom örnekleme modeli, senaryo (b)"), "u_percent": 20.0}]},
        {"components": {"precision": chk(27.0, 0.05), "recovery": chk(4.80, 0.005)},
         "recovery": {"statistic": chk(2.31, 0.005), "significant": True},
         "levels": [{"level": 1.0, "value": chk(1.1111, 0.00005), "standard_uncertainty": chk(0.377, 0.001), "relative_standard_percent": chk(34.0, 0.5)}]},
        ("QUAM:2012 Tables A4.3–A4.5 (0.27, 0.048, 0.2 → 0.34; u = 0.377 at P = 1.1111).",
         "QUAM:2012 Tablo A4.3–A4.5 (0,27, 0,048, 0,2 → 0,34; P = 1,1111'de u = 0,377)."),
        notes=[("Finding: Table A4.2 contains typographical errors in derived columns (mean 0.75 for 0.075, difference −0.10 for "
                "−0.01, mean 0.10 for 0.01); the statistics are recomputed from D1 and D2. The guide quotes t_crit = 2.021, i.e. "
                "t(40), for 41 degrees of freedom (t(41) = 2.0195).",
                "Bulgu: Tablo A4.2'nin türetilmiş sütunlarında yazım hataları vardır (0,075 yerine ortalama 0,75, −0,01 yerine "
                "fark −0,10, 0,01 yerine ortalama 0,10); istatistikler D1 ve D2'den yeniden hesaplanmıştır. Rehber 41 serbestlik "
                "derecesi için t_kritik = 2,021, yani t(40) kullanır (t(41) = 2,0195).")],
        tags=["quam", "duplicates", "recovery-correction"])

project("quam-a6-crude-fibre",
        ("Crude fibre in animal feed (empirical method)", "Hayvan yeminde ham lif (ampirik yöntem)"),
        ("EURACHEM QUAM Example A6: reproducibility s_R from a collaborative trial at three levels plus a drying allowance.",
         "EURACHEM QUAM Örnek A6: üç düzeyde ortak çalışmadan tekrar üretilebilirlik s_R ve kurutma payı."),
        [("For an empirical method the collaborative-trial reproducibility s_R (Table A6.4) is the main component; values "
          "are in % m/m (absolute), so the fields have no _percent suffix.",
          "Ampirik yöntemde ortak çalışma tekrar üretilebilirliği s_R (Tablo A6.4) ana bileşendir; değerler % m/m (mutlak) "
          "biriminde olduğundan alanlarda _percent soneki yoktur."),
         ("Drying to constant weight: ±2 mg on a 1 g sample = ±0.2 % m/m, rectangular, u = 0.115 % m/m; it is included only "
          "where it exceeds s_R/3, i.e. at the 2.5 % level.",
          "Sabit tartıma kurutma: 1 g numunede ±2 mg = ±0,2 % m/m, dikdörtgen, u = 0,115 % m/m; yalnızca s_R/3'ü aştığı "
          "yerde, yani %2,5 düzeyinde dahil edilir."),
         ("U = 2u_c: 0.62, 0.8 and 1.2 % m/m at 2.5, 5 and 10 % m/m.", "U = 2u_c: 2,5, 5 ve 10 % m/m'de 0,62, 0,8 ve 1,2 % m/m.")],
        [src(QUAM, "Example A6, Tables A6.4–A6.5")],
        {"quantity": "w(crude fibre)", "unit": "% m/m", "levels": [2.5, 5.0, 10.0],
         "precision": [prec("s_R (2.5)", {"type": "given", "sd": 0.29}, range={"max": 3.75}),
                       prec("s_R (5)", {"type": "given", "sd": 0.4}, range={"min": 3.75, "max": 7.5}),
                       prec("s_R (10)", {"type": "given", "sd": 0.6}, range={"min": 7.5})],
         "components": [{"name": "drying", "description": T("Drying to constant weight, ±2 mg on 1 g", "Sabit tartıma kurutma, 1 g'da ±2 mg"),
                         "half_width": 0.2, "range": {"max": 3.75}}]},
        {"levels": [{"level": 2.5, "standard_uncertainty": chk(0.31, 0.005), "expanded_uncertainty": chk(0.62, 0.01)},
                    {"level": 5.0, "standard_uncertainty": chk(0.4, 0.005), "expanded_uncertainty": chk(0.8, 0.01)},
                    {"level": 10.0, "standard_uncertainty": chk(0.6, 0.005), "expanded_uncertainty": chk(1.2, 0.01)}]},
        ("QUAM:2012 Tables A6.4–A6.5.", "QUAM:2012 Tablo A6.4–A6.5."),
        notes=[("Finding: Table A6.5 prints U = 0.12 % m/m at 10 % fibre; 2 × 0.6 = 1.2 % m/m (the percentage column, 12 %, is correct).",
                "Bulgu: Tablo A6.5, %10 lifte U = 0,12 % m/m yazar; 2 × 0,6 = 1,2 % m/m'dir (yüzde sütunu, %12, doğrudur).")],
        tags=["quam", "reproducibility", "empirical-method"])
print("ok")
