"""Writes the library projects of the EMPIR EMUE compendium, "Good practice in evaluating measurement
uncertainty — Compendium of examples" (van der Veen & Cox, eds., 2021), for examples not already in the
library through JCGM GUM-5 (run from the repository root). Printed values are recomputed and asserted.
"""
import json
import math
import statistics as st
from pathlib import Path

DOC = "EMPIR 17NRM05 EMUE, Good practice in evaluating measurement uncertainty — Compendium of examples (2021)"
URL = "http://empir.npl.co.uk/emue/wp-content/uploads/sites/49/2021/07/Compendium_M36.pdf"
R3 = math.sqrt(3)


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


def inp(name, dist, unit=None, en=None, tr=None):
    q = {"name": name}
    if unit:
        q["unit"] = unit
    if en:
        q["description"] = T(en, tr)
    q["distribution"] = dist
    return q


def write(field, pid, title, desc, expl, section, tags, model, expected, ref, notes=(), analysis=None):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": field,
                      "sources": [{"document": DOC, "section": section, "url": URL}], "tags": ["emue"] + tags, "notes": list(notes)},
         "model": model,
         "analysis": analysis or {"gum": {"coverage_factor": 2}, "mcm": {"trials": 1000000, "seed": 5, "interval": "symmetric"}},
         "expected": {**expected, "reference": ref}}
    Path(f"library/{field}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


# --- E3.4: aqua regia extractable metals in soil (ISO 21748 budget) -----------------------------------
# Table E3.4.2, column u(δ̂)/x_ref (%): the text calls 3.3 % the root mean square; it is the mean (F87).
RELU = [1.7, 2.4, 2.8, 2.8, 3.7, 4.4, 3.4, 5.7, 6.2, 2.5, 2.0, 3.0, 3.3, 2.6, 2.4, 5.7, 2.6, 2.5, 2.9, 3.0]
rms_u = math.sqrt(sum(v * v for v in RELU) / len(RELU))
assert round(st.mean(RELU), 1) == 3.3 and abs(st.median(RELU) - 2.9) < 0.06 and round(rms_u, 1) == 3.5  # median 2.885 from unrounded data
# Table E3.4.1 RSD (%) excluding Zn in C279a: median 3.2 %, root mean square 3.6 %.
RSD = [3.8, 4.6, 2.1, 1.7, 3.2, 3.6, 3.1, 5.8, 3.1, 2.9, 4.1, 7.1, 2.3, 3.7, 3.9, 3.3, 1.9, 1.5, 2.1]
assert round(st.median(RSD), 1) == 3.2 and round(math.sqrt(sum(v * v for v in RSD) / len(RSD)), 1) == 3.6
W = 27.8
for variant, ud in (("printed", 0.033), ("rms-bias-uncertainty", round(rms_u / 100, 4))):
    u2 = ud ** 2 + 0.036 ** 2 + (0.004 * 5 / R3) ** 2 + (0.0009 * 5 / R3) ** 2 + (0.985 / 984.74) ** 2
    urel = math.sqrt(u2)
    if variant == "printed":
        assert abs(u2 - 0.002526) < 5e-7 and round(2 * urel * W, 1) == 2.8
    U = 2 * urel * W
    pid = "emue-e3.4-soil-lead-iso21748" + ("" if variant == "printed" else "-rms-bias")
    write("chemistry", pid,
          T("Aqua regia extractable lead in soil by ICP-OES: ISO 21748 budget from validation data" + ("" if variant == "printed" else ", root-mean-square bias uncertainty") + " (EMUE E3.4)",
            "Topraktaki kral suyunda çözünür kurşun, ICP-OES: doğrulama verilerinden ISO 21748 bütçesi" + ("" if variant == "printed" else ", kareler ortalamasının kökü sapma belirsizliği") + " (EMUE E3.4)"),
          T(f"u²(w)/w² = u²(δ̂)/w² + s_I²/w² + (c_T u_T)² + (c_t u_t)² + (u(γ₀)/γ₀)² with relative summary values from precision, trueness and ruggedness studies; u(δ̂)/w = {ud}.",
            f"u²(w)/w² = u²(δ̂)/w² + s_I²/w² + (c_T u_T)² + (c_t u_t)² + (u(γ₀)/γ₀)²; kesinlik, gerçeklik ve sağlamlık çalışmalarından bağıl özet değerlerle; u(δ̂)/w = {c(ud)}."),
          [T("ISO 21748 model y = µ + δ + B + Σc_i x′_i + e (E3.4.2): an 11 × 2 nested design on four materials gives the intermediate precision; its median RSD is 3.2 % and the more conservative root-mean-square RSD 3.6 % (zinc in C279a excluded).",
             "ISO 21748 modeli y = µ + δ + B + Σc_i x′_i + e (E3.4.2): dört malzemede 11 × 2 iç içe tasarım ara kesinliği verir; ortanca RSD %3,2, daha temkinli kareler ortalamasının kökü RSD %3,6'dır (C279a'daki çinko hariç)."),
           T("Bias against two CRMs and two PT materials, u²(δ̂) = s²_δ̂ + u²_ref with s²_δ̂ = (s_I² − (1 − 1/n)s_r²)/p (E3.4.4–5). The procedure shows a consistent low bias near 10 %, within the permitted limits, so it is not corrected.",
             "İki SRM ve iki YT malzemesine karşı sapma, u²(δ̂) = s²_δ̂ + u²_ref, s²_δ̂ = (s_I² − (1 − 1/n)s_r²)/p (E3.4.4–5). Yöntem izin verilen sınırlar içinde yaklaşık %10'luk tutarlı bir düşük sapma gösterir; bu nedenle düzeltilmez."),
           T("Extraction temperature and time are not represented in the precision study; their relative sensitivity coefficients 0.004 °C⁻¹ and 9·10⁻⁴ min⁻¹ come from a 2 × 2 factorial experiment (Table E3.4.3), with rectangular uncertainties ±5 °C and ±5 min. The stock solution of Pb (984.74 ± 0.985 mg/kg) enters as a proportional effect.",
             "Ekstraksiyon sıcaklığı ve süresi kesinlik çalışmasında temsil edilmez; bağıl duyarlılık katsayıları 0,004 °C⁻¹ ve 9·10⁻⁴ min⁻¹, 2 × 2 faktöriyel deneyden gelir (Tablo E3.4.3); belirsizlikler ±5 °C ve ±5 min dikdörtgen. Pb stok çözeltisi (984,74 ± 0,985 mg/kg) orantılı etki olarak girer."),
           T(f"u_rel = {urel:.4f}; for a result of {W} mg/kg, U = {U:.2f} mg/kg (k = 2)." + (" The guide's u(δ̂)/w = 0.033 is called the root mean square of Table E3.4.2 but is its mean; the root mean square is 0.035, which gives U = 2.9 mg/kg." if variant != "printed" else ""),
             f"u_bağıl = {c(f'{urel:.4f}')}; {c(W)} mg/kg'lık bir sonuç için U = {c(f'{U:.2f}')} mg/kg (k = 2)." + (" Rehberin u(δ̂)/w = 0,033 değeri Tablo E3.4.2'nin kareler ortalamasının kökü olarak verilir, ancak ortalamasıdır; kareler ortalamasının kökü 0,035'tir ve U = 2,9 mg/kg verir." if variant != "printed" else ""))],
          "E3.4.6–E3.4.7, Tables E3.4.1–E3.4.7, equation (E3.4.18)", ["iso-21748", "top-down", "soil", "metals", "icp-oes", "ruggedness", "uncorrected-bias"],
          {"output": {"name": "w", "unit": "mg/kg", "expression": "w_obs*(1 + d_bias)*(1 + e_I)*(1 + 0.004*dT)*(1 + 0.0009*dt)*(g0/984.74)",
                      "description": T("Aqua regia extractable Pb in soil", "Toprakta kral suyunda çözünür Pb")},
           "inputs": [inp("w_obs", {"type": "constant", "value": W}, "mg/kg", "Measured mass fraction (result for Pb in C279a, E3.4.7)", "Ölçülen kütle kesri (C279a'da Pb sonucu, E3.4.7)"),
                      inp("d_bias", {"type": "normal", "mean": 0.0, "sd": ud}, None, "Relative bias term; its estimate (≈ −10 %) is not applied, its uncertainty is", "Bağıl sapma terimi; tahmini (≈ −%10) uygulanmaz, belirsizliği uygulanır"),
                      inp("e_I", {"type": "normal", "mean": 0.0, "sd": 0.036}, None, "Relative intermediate precision (root-mean-square RSD)", "Bağıl ara kesinlik (kareler ortalamasının kökü RSD)"),
                      inp("dT", {"type": "rectangular", "value": 0.0, "half_width": 5.0}, "°C", "Deviation of the extraction temperature from 65 °C", "Ekstraksiyon sıcaklığının 65 °C'den sapması"),
                      inp("dt", {"type": "rectangular", "value": 0.0, "half_width": 5.0}, "min", "Deviation of the extraction time from 120 min", "Ekstraksiyon süresinin 120 min'den sapması"),
                      inp("g0", {"type": "normal", "mean": 984.74, "sd": 0.985}, "mg/kg", "Pb stock solution (certificate)", "Pb stok çözeltisi (sertifika)")]},
          {"gum": {"estimate": chk(W, 1e-9), "standard_uncertainty": chk(round(urel * W, 4), 2e-4), "expanded_uncertainty": chk(round(U, 3), 0.001)}},
          T("EMUE E3.4, (E3.4.18) and E3.4.7: u′(Pb)² = 0.002 526, u′ ≈ 5 %, result (27.8 ± 2.8) mg/kg (k = 2)" + ("" if variant == "printed" else "; this variant recomputes u(δ̂)/w as the root mean square (F87)") + ".",
            "EMUE E3.4, (E3.4.18) ve E3.4.7: u′(Pb)² = 0,002 526, u′ ≈ %5, sonuç (27,8 ± 2,8) mg/kg (k = 2)" + ("" if variant == "printed" else "; bu varyant u(δ̂)/w'yi kareler ortalamasının kökü olarak yeniden hesaplar (F87)") + "."),
          notes=[T("The bias of about −10 % is reported, not corrected (Eurachem): 'The procedure in use shows apparent recovery of approximately 90 %. Since the recovery is within permitted limits, no correction for recovery has been applied.' The program's reporting.uncorrected_bias offers intervals that include it (Magnusson & Ellison 2008).",
                   "Yaklaşık −%10'luk sapma düzeltilmez, raporlanır (Eurachem): 'Kullanılan yöntem yaklaşık %90 görünür geri kazanım gösterir. Geri kazanım izin verilen sınırlar içinde olduğundan düzeltme uygulanmamıştır.' Programın reporting.uncorrected_bias seçeneği onu içeren aralıklar sunar (Magnusson & Ellison 2008)."),
                 T("The result 27.8 mg/kg of E3.4.7 is not the mean 27.29 mg/kg of Table E3.4.1 for Pb in C279a; with 27.29 mg/kg U = 2.7 mg/kg.",
                   "E3.4.7'deki 27,8 mg/kg sonucu, Tablo E3.4.1'deki C279a Pb ortalaması 27,29 mg/kg değildir; 27,29 mg/kg ile U = 2,7 mg/kg olur.")])

# --- E2.1: conformity of an influenza medication with four correlated active components -------------
R21 = [[1, 0.107, 0.125, 0.177], [0.107, 1, 0.311, 0.404], [0.125, 0.311, 1, 0.539], [0.177, 0.404, 0.539, 1]]
COMP21 = [("APAP", 99.18, 1.37), ("DEX", 97.70, 1.02), ("DOX", 99.33, 1.05), ("PE", 98.94, 1.22)]
# Recomputed (tools/research/emue/e21_total_risks.py, scipy and numpy): 0.002 881, 0.011 716, 0.000 212; global 0.001 82.
p21 = {"format": "uncertainty-calculations/project", "version": 1,
       "metadata": {"id": "emue-e2.1-multicomponent-medication", "field": "chemistry",
                    "title": T("Total risks of false decisions for a medication with four correlated active components (EMUE E2.1)",
                               "Dört ilişkili etken maddeli bir ilaç için toplam yanlış karar riskleri (EMUE E2.1)"),
                    "description": T("NyQuil tablets: APAP, DEX, DOX and PE, tolerance 95–105 % of the labelled amount, u_rel = 2.8 %; total specific consumer's risks for three sets of test results and the total global consumer's risk.",
                                     "NyQuil tabletleri: APAP, DEX, DOX ve PE; tolerans etiket miktarının %95–105'i, u_bağıl = %2,8; üç deney sonucu takımı için toplam özgül tüketici riskleri ve toplam küresel tüketici riski."),
                    "explanation": [
                        T("Each component may conform while the item as a whole does not: the total specific consumer's risk is the probability that at least one true content lies outside 95–105 % although all test results lie inside (IUPAC project; EMUE E2.1.6.2).",
                          "Her bileşen tek başına uygun olabilir, ürün bütünüyle uygun olmayabilir: toplam özgül tüketici riski, tüm deney sonuçları içerideyken en az bir gerçek içeriğin %95–105 dışında olma olasılığıdır (IUPAC projesi; EMUE E2.1.6.2)."),
                        T("Prior: multivariate normal from 105 production lots (means 99.18, 97.70, 99.33, 98.94 %; s.d. 1.37, 1.02, 1.05, 1.22 %) with the Pearson correlations of Table E2.1.1. Likelihood: multivariate normal with u_i = 0.028·c_mi and the same correlations. The posterior is normal, S_post = (S_c⁻¹ + S_cm⁻¹)⁻¹ (E2.1.1–E2.1.2).",
                          "Önsel: 105 üretim partisinden çok değişkenli normal (ortalamalar %99,18, 97,70, 99,33, 98,94; s.s. %1,37, 1,02, 1,05, 1,22), Tablo E2.1.1'in Pearson korelasyonlarıyla. Olabilirlik: u_i = 0,028·c_mi ve aynı korelasyonlarla çok değişkenli normal. Sonsal dağılım normaldir, S_post = (S_c⁻¹ + S_cm⁻¹)⁻¹ (E2.1.1–E2.1.2)."),
                        T("Total specific consumer's risks: 0.29 % when the results equal the prior means, 1.17 % at the lower limits and 0.02 % at the upper limits; the prior shifts the posterior towards its means, which lie below 100 %. Total global consumer's risk 0.18 %: 18 of 10 000 tablets accepted without conforming.",
                          "Toplam özgül tüketici riskleri: sonuçlar önsel ortalamalara eşitken %0,29, alt sınırlarda %1,17, üst sınırlarda %0,02; önsel, sonsal dağılımı %100'ün altındaki ortalamalarına doğru kaydırır. Toplam küresel tüketici riski %0,18: 10 000 tabletten 18'i uygun olmadan kabul edilir.")],
                    "sources": [{"document": DOC, "section": "Example E2.1, Tables E2.1.1–E2.1.2, E2.1.6", "url": URL}],
                    "tags": ["emue", "conformity-assessment", "multicomponent", "total-risk", "pharmaceutical", "correlation", "bayesian"],
                    "notes": [T("The program integrates the 4-dimensional posterior over the tolerance box by the method of Genz (separation of variables, 200 000 antithetic draws, standard error about 10⁻⁶) instead of R's pmvnorm, and the global risk by Monte Carlo with u_i proportional to the true value.",
                                "Program 4 boyutlu sonsal dağılımı tolerans kutusu üzerinde R'nin pmvnorm işlevi yerine Genz yöntemiyle (değişkenlere ayırma, 200 000 karşıt çekiliş, standart hata yaklaşık 10⁻⁶) integre eder; küresel riski u_i gerçek değerle orantılı alınarak Monte Carlo ile hesaplar.")]},
       "multicomponent_conformity": {"unit": "% of label",
                                     "components": [{"name": n_, "prior_mean": m_, "prior_sd": s_, "lower": 95.0, "upper": 105.0} for n_, m_, s_ in COMP21],
                                     "correlation": R21, "u_relative": 0.028,
                                     "results": [{"name": "c_m = m", "values": [m_ for _, m_, _ in COMP21]},
                                                 {"name": "c_m = T_L", "values": [95.0] * 4}, {"name": "c_m = T_U", "values": [105.0] * 4}],
                                     "global_trials": 2000000, "samples": 200000, "seed": 1},
       "expected": {"multicomponent_conformity": {
           "results": {"c_m = m": {"risk": chk(0.0029, 5e-5), "accepted": True}, "c_m = T_L": {"risk": chk(0.0117, 5e-5), "accepted": True},
                       "c_m = T_U": {"risk": chk(0.0002, 5e-5), "accepted": True}},
           "global_consumer_risk": chk(0.0018, 1e-4)},
           "reference": T("EMUE E2.1.6.2–E2.1.6.3: R*_tot = 0.0029, 0.0117, 0.0002; R_tot = 0.0018 (10⁷ trials; 0.0019 in the original paper).",
                          "EMUE E2.1.6.2–E2.1.6.3: R*_tot = 0,0029, 0,0117, 0,0002; R_tot = 0,0018 (10⁷ deneme; özgün makalede 0,0019).")}}
Path("library/chemistry/emue-e2.1-multicomponent-medication.json").write_text(json.dumps(p21, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("wrote emue-e2.1-multicomponent-medication")
