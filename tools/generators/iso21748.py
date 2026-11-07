"""Writes the ISO 21748:2017 Annex C library projects (run from the repository root)."""
import json, math
from pathlib import Path

DOC = "ISO 21748:2017, Guidance for the use of repeatability, reproducibility and trueness estimates in measurement uncertainty evaluation"
URL = "https://www.iso.org/standard/71615.html"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def c(x): return str(x).replace('.', ',')

def project(path, pid, title, desc, expl, field, sec, tags, notes, body, expected, reporting=None, kind="topdown"):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": field,
                      "sources": [{"document": DOC, "section": sec, "url": URL}], "tags": ["iso21748", "reproducibility"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p[kind] = body
    if kind == "model":
        p["analysis"] = {"gum": {"coverage_factor": 2}, "mcm": {"trials": 1000000, "seed": 21}}
    if reporting: p["reporting"] = reporting
    p["expected"] = expected
    Path(path).write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

# ------------------------------------------------------------------ C.1
project("library/general/iso21748-c1-co-emissions.json", "iso21748-c1-co-emissions",
  T("Carbon monoxide in vehicle emissions from reproducibility data (ISO 21748 C.1)", "Taşıt emisyonlarında karbon monoksit, tekrar üretilebilirlik verilerinden (ISO 21748 C.1)"),
  T("ISO 21748:2017 C.1: type-approval test of CO emissions; with bias and precision under control the reproducibility standard deviation of the collaborative study is the standard uncertainty.",
    "ISO 21748:2017 C.1: CO emisyonunun tip onay testi; sapma ve kesinlik kontrol altındayken ortak çalışmanın tekrar üretilebilirlik standart sapması standart belirsizliktir."),
  [T("An interlaboratory study according to ISO 5725-2 gave s_r = 0.22 g/km and s_R = 0.28 g/km, so s_L = √(s_R² − s_r²) = 0.17 g/km.",
     "ISO 5725-2'ye göre yapılan bir laboratuvarlar arası çalışma s_r = 0,22 g/km ve s_R = 0,28 g/km vermiştir; dolayısıyla s_L = √(s_R² − s_r²) = 0,17 g/km'dir."),
   T("Trueness is controlled by calibration of the gas analyser and the roller bench, whose uncertainties are negligible against s_R; the laboratory's repeatability (0.20 g/km) is within that of the study (ISO 21748 7.2, 7.3).",
     "Doğruluk, gaz analizörünün ve silindir tezgâhının kalibrasyonuyla kontrol edilir; bunların belirsizlikleri s_R'ye göre ihmal edilebilir; laboratuvarın tekrarlanabilirliği (0,20 g/km) çalışmanınkiyle uyumludur (ISO 21748 7.2, 7.3)."),
   T("No operation is outside the collaborative study, so u = s_R = 0.28 g/km (Formula 3) and U = 0.56 g/km (k = 2), evaluated near the limit 2.2 g/km.",
     "Ortak çalışmanın dışında kalan bir işlem yoktur; bu nedenle u = s_R = 0,28 g/km (Formül 3) ve U = 0,56 g/km'dir (k = 2); değerlendirme 2,2 g/km sınırı yakınında yapılmıştır.")],
  "general", "Annex C.1", ["collaborative-study"],
  [T("The laboratory's repeatability is not substituted for s_r here (as in ISO 21748 C.1); with lab_repeatability = 0.20 g/km, 7.3.2 would give s′_R = √(s_L² + s_l²) = 0.26 g/km.",
     "Burada laboratuvarın tekrarlanabilirliği s_r yerine konmamıştır (ISO 21748 C.1'deki gibi); lab_repeatability = 0,20 g/km ile 7.3.2, s′_R = √(s_L² + s_l²) = 0,26 g/km verirdi.")],
  {"quantity": "CO", "unit": "g/km", "levels": [2.2],
   "precision": [{"name": "s_R", "data": {"type": "reproducibility", "sd_reproducibility": 0.28, "sd_repeatability": 0.22}}]},
  {"topdown": {"components": {"s_R": chk(0.28, 0.0005)},
               "levels": [{"level": 2.2, "standard_uncertainty": chk(0.28, 0.0005), "expanded_uncertainty": chk(0.56, 0.0005)}]},
   "reference": T("ISO 21748:2017 C.1.2 and C.1.6 (s_r = 0.22, s_R = 0.28 g/km, U = 0.56 g/km).", "ISO 21748:2017 C.1.2 ve C.1.6 (s_r = 0,22, s_R = 0,28 g/km, U = 0,56 g/km).")},
  reporting={"significant_digits": 2})

# ------------------------------------------------------------------ C.3
foods = [("shrimp", "karides", 11.1, 9.8, 7.2, 7.8, 15.6), ("vegetables", "sebze", 9.2, 6.3, 8.4, 8.9, 17.8), ("flour", "un", 5.8, 5.3, 5.5, 6.4, 12.8)]
for en, tr, sR, sr, adj, uc, U in foods:
    sL = math.sqrt(sR**2 - sr**2)
    sadj = math.sqrt(sL**2 + 25)
    u = math.sqrt(sadj**2 + 9)
    flour = en == "flour"
    notes = []
    exp_u = (uc, 0.05)
    exp_U = (U, 0.05)
    if flour:
        notes.append(T(f"ISO 21748 Table C.4 prints u = 6.4 % and C.3.7 U = 12.8 % for flour, but its own inputs give √(2.4² + 5.0² + 3.0²) = 6.30 % (unrounded s_L: {u:.2f} %) and U = 12.6 %. The intervals of Table C.5 for flour follow from the printed 12.8 %. The project checks the recomputed values.",
                       f"ISO 21748 Tablo C.4 un için u = %6,4 ve C.3.7 U = %12,8 basar; ancak kendi girdileri √(2,4² + 5,0² + 3,0²) = %6,30 (yuvarlanmamış s_L ile %{u:.2f}) ve U = %12,6 verir. Tablo C.5'teki un aralıkları basılı %12,8'den çıkar. Proje yeniden hesaplanan değerleri kontrol eder.".replace(f"{u:.2f}", f"{u:.2f}".replace('.', ','))))
        exp_u = (round(u, 3), 0.005); exp_U = (round(2 * u, 2), 0.01)
    project(f"library/general/iso21748-c3-aerobic-plate-count-{en}.json", f"iso21748-c3-aerobic-plate-count-{en}",
      T(f"Aerobic plate count, {en}: adjusted reproducibility (ISO 21748 C.3)", f"Aerobik koloni sayımı, {tr}: düzeltilmiş tekrar üretilebilirlik (ISO 21748 C.3)"),
      T(f"ISO 21748:2017 C.3 (AOAC 990.12, {en}): relative uncertainty of log10(count) from collaborative-study s_R and s_r, the laboratory's better repeatability (7.3.2) and a sample-preparation term.",
        f"ISO 21748:2017 C.3 (AOAC 990.12, {tr}): ortak çalışmanın s_R ve s_r değerlerinden, laboratuvarın daha iyi tekrarlanabilirliğinden (7.3.2) ve bir numune hazırlama teriminden log10(sayım)'ın bağıl belirsizliği."),
      [T(f"Collaborative study (12 laboratories, ISO 5725-2): s_R = {sR} %, s_r = {sr} % relative to log10(count), so s_L = √(s_R² − s_r²) = {sL:.1f} %.",
         f"Ortak çalışma (12 laboratuvar, ISO 5725-2): log10(sayım)'a göre s_R = %{c(sR)}, s_r = %{c(sr)}; dolayısıyla s_L = √(s_R² − s_r²) = %{c(f'{sL:.1f}')}."),
       T("The laboratory's repeatability from 10 replicates is s_l = 5.0 %; ISO 21748 7.3.2 allows it in place of s_r: s′_R = √(s_L² + s_l²) = " + f"{sadj:.1f} %.",
         "Laboratuvarın 10 tekrardan bulunan tekrarlanabilirliği s_l = %5,0'dır; ISO 21748 7.3.2 bunun s_r yerine kullanılmasına izin verir: s′_R = √(s_L² + s_l²) = %" + f"{sadj:.1f}.".replace('.', ',', 1)),
       T(f"Sub-sampling and weighing, not covered by the study, add 3.0 % (expert estimate); u = √(s′_R² + 3.0²) = {u:.1f} % and U = {2*u:.1f} % (k = 2).",
         f"Çalışmanın kapsamadığı alt örnekleme ve tartım %3,0 ekler (uzman tahmini); u = √(s′_R² + 3,0²) = %{u:.1f} ve U = %{2*u:.1f} (k = 2).".replace(f"{u:.1f}", f"{u:.1f}".replace('.', ',')).replace(f"{2*u:.1f}", f"{2*u:.1f}".replace('.', ',')))],
      "general", "Annex C.3, Tables C.2–C.4", ["microbiology", "relative"], notes,
      {"quantity": f"log10(CFU), {en}",
       "precision": [{"name": "s′_R", "data": {"type": "reproducibility", "sd_reproducibility_percent": sR, "sd_repeatability_percent": sr, "lab_repeatability_percent": 5.0}}],
       "components": [{"name": "sample preparation", "u_percent": 3.0}]},
      {"topdown": {"components": {"s′_R": chk(adj, 0.05)},
                   "levels": [{"relative_standard_percent": chk(*exp_u), "relative_expanded_percent": chk(*exp_U)}]},
       "reference": T(f"ISO 21748:2017 Tables C.3 and C.4 and C.3.7 ({en}: s′_R = {adj} %, u = {uc} %, U = {U} %)" + ("; flour: recomputed, see note." if flour else "."),
                      f"ISO 21748:2017 Tablo C.3 ve C.4 ile C.3.7 ({tr}: s′_R = %{c(adj)}, u = %{c(uc)}, U = %{c(U)})" + ("; un: yeniden hesaplanmış, nota bakınız." if flour else "."))},
      reporting={"significant_digits": 2})

# ------------------------------------------------------------------ C.4
project("library/chemistry/iso21748-c4-crude-fibre.json", "iso21748-c4-crude-fibre",
  T("Crude fibre in animal feed from reproducibility data (ISO 21748 C.4)", "Hayvan yeminde ham lif, tekrar üretilebilirlik verilerinden (ISO 21748 C.4)"),
  T("ISO 21748:2017 C.4: level-dependent reproducibility from a collaborative study plus the uncertainty of drying to constant mass; the same example as QUAM A6 with a different treatment of the drying term.",
    "ISO 21748:2017 C.4: bir ortak çalışmadan düzeye bağlı tekrar üretilebilirlik ve sabit kütleye kurutmanın belirsizliği; QUAM A6 ile aynı örnek, kurutma terimi farklı ele alınmıştır."),
  [T("Collaborative study (Table C.6): s_R = 0.293 % at 2.3 %, 0.390 % at 5.4 % and 0.575 % at 10.1 % fibre. Bias was checked on a CRM (9.3 %, laboratory 9.16 %, Δ_l = −0.14 %) and the laboratory's repeatability agrees with the study (Table C.7).",
     "Ortak çalışma (Tablo C.6): %2,3 lifte s_R = %0,293, %5,4'te %0,390 ve %10,1'de %0,575. Sapma bir CRM üzerinde kontrol edilmiştir (%9,3; laboratuvar %9,16; Δ_l = −%0,14) ve laboratuvarın tekrarlanabilirliği çalışmayla uyumludur (Tablo C.7)."),
   T("Drying to constant mass is not covered by the study: the fixed-time drying stays within 0.002 g of extended drying, a rectangular half-width giving 0.002/√3 = 0.00115 g, i.e. 0.115 % fibre for a 1 g sample.",
     "Sabit kütleye kurutma çalışmanın kapsamında değildir: sabit süreli kurutma, uzatılmış kurutmanın 0,002 g yakınında kalır; bu dikdörtgen yarı genişlik 0,002/√3 = 0,00115 g, yani 1 g numunede %0,115 lif verir."),
   T("u = √(s_R² + 0.115²) = 0.31, 0.41 and 0.59 % in the three ranges (Table C.9), U = 0.6, 0.8 and 1.2 % (k = 2).",
     "Üç aralıkta u = √(s_R² + 0,115²) = %0,31; 0,41 ve 0,59 (Tablo C.9), U = %0,6; 0,8 ve 1,2 (k = 2).")],
  "chemistry", "Annex C.4, Tables C.6–C.9", ["crude-fibre", "quam-a6", "ranges"],
  [T("ISO 21748 C.4.7 says that the uncertainty of 0.115 % was obtained by 'dividing this maximum estimated deviation by 3'; 0.002 g/3 would give 0.067 %. The printed value corresponds to division by √3 (rectangular distribution), which the project uses.",
     "ISO 21748 C.4.7, %0,115 belirsizliğinin 'bu en büyük tahmini sapmanın 3'e bölünmesiyle' bulunduğunu söyler; 0,002 g/3 %0,067 verirdi. Basılı değer √3'e bölmeye (dikdörtgen dağılım) karşılık gelir; proje bunu kullanır."),
   T("QUAM A6 (project 'quam-a6-crude-fibre') uses the rounded s_R 0.29, 0.40, 0.60 and includes the drying term only at the lowest level.",
     "QUAM A6 ('quam-a6-crude-fibre' projesi) yuvarlanmış s_R değerlerini (0,29; 0,40; 0,60) kullanır ve kurutma terimini yalnızca en düşük düzeyde dahil eder.")],
  {"quantity": "w(crude fibre)", "unit": "% m/m", "levels": [2.3, 4.0, 8.0],
   "precision": [{"name": "s_R (≤ 2.5)", "data": {"type": "reproducibility", "sd_reproducibility": 0.293, "sd_repeatability": 0.198}, "range": {"max": 2.5}},
                 {"name": "s_R (2.5–5)", "data": {"type": "reproducibility", "sd_reproducibility": 0.390, "sd_repeatability": 0.264}, "range": {"min": 2.5, "max": 5.0}},
                 {"name": "s_R (5–10)", "data": {"type": "reproducibility", "sd_reproducibility": 0.575, "sd_repeatability": 0.391}, "range": {"min": 5.0, "max": 10.0}}],
   "components": [{"name": "drying to constant mass", "half_width": 0.2}]},
  {"topdown": {"levels": [{"level": 2.3, "standard_uncertainty": chk(0.31, 0.005), "expanded_uncertainty": chk(0.6, 0.05)},
                          {"level": 4.0, "standard_uncertainty": chk(0.41, 0.005), "expanded_uncertainty": chk(0.8, 0.05)},
                          {"level": 8.0, "standard_uncertainty": chk(0.59, 0.005), "expanded_uncertainty": chk(1.2, 0.05)}]},
   "reference": T("ISO 21748:2017 Table C.9 and C.4.9 (u = 0.31, 0.41, 0.59 %; U = 0.6, 0.8, 1.2 %).", "ISO 21748:2017 Tablo C.9 ve C.4.9 (u = %0,31; 0,41; 0,59; U = %0,6; 0,8; 1,2).")},
  reporting={"significant_digits": 2})

# ------------------------------------------------------------------ C.2 (model)
project("library/chemistry/iso21748-c2-meat-content.json", "iso21748-c2-meat-content",
  T("Meat content from nitrogen and fat (ISO 21748 C.2)", "Azot ve yağdan et içeriği (ISO 21748 C.2)"),
  T("ISO 21748:2017 C.2: a combined result w_meat = 100 w_N/f_N + w_fat whose inputs each carry reproducibility-based uncertainties (Clause 12), written as a measurement model with relative error terms.",
    "ISO 21748:2017 C.2: girdilerinin her biri tekrar üretilebilirlik tabanlı belirsizlik taşıyan birleşik bir sonuç, w_meat = 100 w_N/f_N + w_fat; bağıl hata terimleri olan bir ölçüm modeli olarak yazılmıştır."),
  [T("w_meat = w_pro + w_fat with w_pro = 100 w_N/f_N (Formulae C.1, C.2); w_N = 3.29 % (Kjeldahl, mean of duplicates), f_N = 3.65 ± 0.052, w_fat = 5.50 %.",
     "w_meat = w_pro + w_fat, w_pro = 100 w_N/f_N (Formül C.1, C.2); w_N = %3,29 (Kjeldahl, ikili ölçüm ortalaması), f_N = 3,65 ± 0,052, w_fat = %5,50."),
   T("The nitrogen result carries the between-laboratory term s_L = 1.1 % and the repeatability s_r = 1.8 % of a single result divided by √2 for the duplicate (ISO 21748 Table 1): e_L ~ N(0, 0.011²), e_r ~ N(0, (0.018/√2)²), so u(w_N)/w_N = 1.7 % (Formula C.3). Fat carries s_R = 2.0 %.",
     "Azot sonucu laboratuvarlar arası s_L = %1,1 terimini ve ikili ölçüm için √2'ye bölünmüş tek sonuç tekrarlanabilirliğini s_r = %1,8 taşır (ISO 21748 Tablo 1): e_L ~ N(0; 0,011²), e_r ~ N(0; (0,018/√2)²); dolayısıyla u(w_N)/w_N = %1,7 (Formül C.3). Yağ s_R = %2,0 taşır."),
   T("The law of propagation gives w_meat = 95.6 % with u = 1.99 % and U = 4.0 % (k = 2); w_pro contributes 99.7 % of the variance.",
     "Yayılım yasası w_meat = %95,6, u = %1,99 ve U = %4,0 (k = 2) verir; varyansın %99,7'si w_pro'dan gelir.")],
  "chemistry", "Annex C.2, Table C.1", ["combined-result", "food"],
  [T("Table C.1 rounds the relative uncertainties (0.017, 0.014, 0.022) before combining and prints u = 1.98 %; with the unrounded terms u = 1.99 %. Both give U = 4.0 %.",
     "Tablo C.1 bağıl belirsizlikleri (0,017; 0,014; 0,022) birleştirmeden önce yuvarlar ve u = %1,98 basar; yuvarlanmamış terimlerle u = %1,99'dur. İkisi de U = %4,0 verir.")],
  {"output": {"name": "w_meat", "unit": "%", "expression": "100 * w_N * (1 + e_L + e_r) / f_N + w_fat * (1 + e_fat)",
              "description": T("Total meat content", "Toplam et içeriği")},
   "inputs": [{"name": "w_N", "unit": "%", "description": T("Total nitrogen content (Kjeldahl)", "Toplam azot içeriği (Kjeldahl)"), "distribution": {"type": "constant", "value": 3.29}},
              {"name": "e_L", "description": T("Relative laboratory effect, s_L = 0.011", "Bağıl laboratuvar etkisi, s_L = 0,011"), "distribution": {"type": "normal", "mean": 0, "sd": 0.011}},
              {"name": "e_r", "description": T("Relative repeatability of the mean of duplicates, 0.018/√2", "İkili ölçüm ortalamasının bağıl tekrarlanabilirliği, 0,018/√2"), "distribution": {"type": "normal", "mean": 0, "sd": round(0.018 / math.sqrt(2), 7)}},
              {"name": "f_N", "description": T("Nitrogen factor of the meat", "Etin azot faktörü"), "distribution": {"type": "normal", "mean": 3.65, "sd": 0.052}},
              {"name": "w_fat", "unit": "%", "description": T("Fat content", "Yağ içeriği"), "distribution": {"type": "constant", "value": 5.50}},
              {"name": "e_fat", "description": T("Relative reproducibility of the fat result, s_R = 0.02", "Yağ sonucunun bağıl tekrar üretilebilirliği, s_R = 0,02"), "distribution": {"type": "normal", "mean": 0, "sd": 0.02}}]},
  {"gum": {"estimate": chk(95.6, 0.05), "standard_uncertainty": chk(1.98, 0.015), "expanded_uncertainty": chk(4.0, 0.05)},
   "reference": T("ISO 21748:2017 Table C.1 and C.2.6 (w_meat = 95.6 %, u = 1.98 %, U = 4.0 %).", "ISO 21748:2017 Tablo C.1 ve C.2.6 (w_meat = %95,6, u = %1,98, U = %4,0).")},
  reporting={"significant_digits": 2}, kind="model")
