"""Writes the UKAS M3003 ed. 6 library projects (run from the repository root)."""
import json, math
from pathlib import Path

DOC = "UKAS M3003 ed. 6 (2024), The Expression of Uncertainty and Confidence in Measurement"
URL = "https://www.ukas.com/wp-content/uploads/2023/05/M3003-The-expression-of-uncertainty-and-confidence-in-measurement.pdf"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def normal(m, U=None, sd=None): return {"type": "normal", "mean": m, **({"expanded": U, "k": 2} if U is not None else {"sd": sd})}
def rect(hw, v=0.0): return {"type": "rectangular", "value": v, "half_width": hw}
def tri(hw, v=0.0): return {"type": "triangular", "value": v, "half_width": hw}
def arc(hw, v=0.0): return {"type": "arcsine", "value": v, "half_width": hw}
def const(v): return {"type": "constant", "value": v}
def typea(obs): return {"type": "type_a", "observations": obs}
def pooled(mean, sd, dof, n): return {"type": "type_a_pooled", "mean": mean, "pooled_sd": sd, "pooled_dof": dof, "n": n}
def inp(name, dist, en, tr, unit=None):
    d = {"name": name, "description": T(en, tr), "distribution": dist}
    if unit: d["unit"] = unit
    return d

K2 = {"gum": {"coverage_factor": 2}, "mcm": {"trials": 200000, "seed": 1}}
STATEMENT = T("Reported as in M3003: expanded uncertainty with k = 2 for a coverage probability of approximately 95 %.",
              "M3003'teki gibi raporlanır: yaklaşık %95 kapsam olasılığı için k = 2 ile genişletilmiş belirsizlik.")

def project(folder, name, title, desc, expl, sec, tags, model, expected, ref, notes=None, analysis=None):
    pid = f"m3003-{name}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl + [STATEMENT] if (analysis or K2).get("gum", {}).get("coverage_factor") == 2 else expl,
                      "field": folder, "sources": [{"document": DOC, "section": sec, "url": URL}],
                      "tags": ["ukas-m3003", "calibration", "validation"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p["model"] = model
    p["analysis"] = analysis or K2
    p["expected"] = {"gum": expected, "reference": ref}
    Path(f"library/{folder}").mkdir(exist_ok=True)
    Path(f"library/{folder}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

# ------------------------------------------------------------------ K.1
project("electrical", "k1-resistor-10kohm",
    T("10 kΩ resistor by voltage intercomparison (UKAS M3003 K.1)", "Gerilim karşılaştırmasıyla 10 kΩ direnç (UKAS M3003 K.1)"),
    T("The voltages across a standard and an unknown 10 kΩ resistor in series are compared with one DVM; both resistors are in an oil bath at 20.0 °C ± 0.1 °C.",
      "Seri bağlı standart ve bilinmeyen 10 kΩ dirençler üzerindeki gerilimler tek bir DVM ile karşılaştırılır; her iki direnç 20,0 °C ± 0,1 °C yağ banyosundadır."),
    [T("R_X = (R_S + δR_D + R_TC·Δt)·V_X/V_S (K.1.1), with the DVM linearity and resolution as relative errors of each reading, ±0.2 ppm, rectangular (K.1.6).",
       "R_X = (R_S + δR_D + R_TC·Δt)·V_X/V_S (K.1.1); DVM doğrusallığı ve çözünürlüğü her okumanın göreli hatası olarak, ±0,2 ppm, dikdörtgen (K.1.6)."),
     T("R_S = 10 000.000 Ω is implied by the result 10 000.1050 Ω = R_S·(1 + 10.50 ppm); certificate ±0.5 ppm (k = 2), uncorrected drift ±0.5 ppm, temperature coefficient 2.5 ppm/°C over ±0.1 °C.",
       "R_S = 10 000,000 Ω, 10 000,1050 Ω = R_S·(1 + 10,50 ppm) sonucundan çıkar; sertifika ±0,5 ppm (k = 2), düzeltilmemiş sürüklenme ±0,5 ppm, ±0,1 °C üzerinde 2,5 ppm/°C sıcaklık katsayısı."),
     T("Five readings of the ratio departure (+10.4 … +10.7 ppm): mean +10.50 ppm, s = 0.158 ppm, u = s/√5 = 0.071 ppm with 4 degrees of freedom.",
       "Oran sapmasının beş okuması (+10,4 … +10,7 ppm): ortalama +10,50 ppm, s = 0,158 ppm, u = s/√5 = 0,071 ppm, 4 serbestlik derecesi."),
     T("u_c = 0.445 ppm = 0.004 45 Ω; R_X = 10 000.1050 Ω ± 0.0089 Ω (k = 2).", "u_c = 0,445 ppm = 0,004 45 Ω; R_X = 10 000,1050 Ω ± 0,0089 Ω (k = 2).")],
    "Appendix K.1", ["resistance", "dc", "type-a"],
    {"output": {"name": "R_X", "unit": "Ω", "expression": "(R_S + dR_D + R_TC*dt) * r * (1 + d_VX) / (1 + d_VS)", "description": T("Resistance of the unknown resistor", "Bilinmeyen direncin değeri")},
     "inputs": [inp("R_S", normal(10000.0, U=0.005), "Standard resistor (certificate, ±0.5 ppm, k = 2)", "Standart direnç (sertifika, ±0,5 ppm, k = 2)", "Ω"),
                inp("dR_D", rect(0.005), "Drift of the standard since calibration (±0.5 ppm)", "Kalibrasyondan beri standardın sürüklenmesi (±0,5 ppm)", "Ω"),
                inp("R_TC", const(0.025), "Temperature coefficient of R_S (2.5 ppm/°C)", "R_S'nin sıcaklık katsayısı (2,5 ppm/°C)", "Ω/°C"),
                inp("dt", rect(0.1), "Deviation of the oil bath from 20.0 °C", "Yağ banyosunun 20,0 °C'den sapması", "°C"),
                inp("d_VS", rect(2e-7), "DVM linearity and resolution, reading across R_S (relative)", "DVM doğrusallığı ve çözünürlüğü, R_S üzerindeki okuma (göreli)"),
                inp("d_VX", rect(2e-7), "DVM linearity and resolution, reading across R_X (relative)", "DVM doğrusallığı ve çözünürlüğü, R_X üzerindeki okuma (göreli)"),
                inp("r", typea([1.0000104, 1.0000107, 1.0000106, 1.0000103, 1.0000105]), "Voltage ratio V_X/V_S, five readings", "V_X/V_S gerilim oranı, beş okuma")]},
    {"estimate": chk(10000.105, 5e-5), "standard_uncertainty": chk(0.00445, 5e-6), "expanded_uncertainty": chk(0.0089, 5e-5)},
    T("UKAS M3003 ed. 6 K.1.8–K.1.9: u_c = 0.445 ppm, U = 0.891 ppm; 10 000.1050 Ω ± 0.0089 Ω.", "UKAS M3003 6. baskı K.1.8–K.1.9: u_c = 0,445 ppm, U = 0,891 ppm; 10 000,1050 Ω ± 0,0089 Ω."))

# ------------------------------------------------------------------ K.2
project("electrical", "k2-power-sensor-18ghz",
    T("Coaxial power sensor at 18 GHz (UKAS M3003 K.2)", "18 GHz'de koaksiyel güç sensörü (UKAS M3003 K.2)"),
    T("Calibration factor of an unknown power sensor by substitution against a standard sensor on a monitored source; four connections.",
      "Bilinmeyen bir güç sensörünün kalibrasyon faktörü, izlenen bir kaynakta standart sensörle yer değiştirme yöntemiyle; dört bağlantı."),
    [T("K_X = (K_S + δD_S)·f_DC·f_M·f_REF (K.2.1). M3003 combines all terms in percentage points of the calibration factor, so the model adds them to the observed K_X.",
       "K_X = (K_S + δD_S)·f_DC·f_M·f_REF (K.2.1). M3003 tüm terimleri kalibrasyon faktörünün yüzde puanı olarak birleştirir; model bu nedenle onları gözlenen K_X'e ekler."),
     T("Mismatch limits 200·|Γ_G|·|Γ| %: 0.08 % at 50 MHz for both sensors, 1.40 % (standard) and 1.68 % (unknown) at 18 GHz, U-shaped (arc sine, u = a/√2).",
       "Uyumsuzluk sınırları 200·|Γ_G|·|Γ| %: her iki sensör için 50 MHz'de %0,08, 18 GHz'de %1,40 (standart) ve %1,68 (bilinmeyen), U biçimli (ark sinüs, u = a/√2)."),
     T("Four values 93.45, 92.20, 93.95, 93.02 %: mean 93.16 %, s = 0.7415 %, u = s/√4 = 0.37 % (3 degrees of freedom).", "Dört değer 93,45, 92,20, 93,95, 93,02 %: ortalama %93,16, s = %0,7415, u = s/√4 = %0,37 (3 serbestlik derecesi)."),
     T("u_c = 1.69 %, dominated by the 18 GHz mismatch terms; K_X = 93.2 % ± 3.4 % (k = 2).", "u_c = %1,69, 18 GHz uyumsuzluk terimleri baskındır; K_X = %93,2 ± %3,4 (k = 2).")],
    "Appendix K.2", ["rf", "power", "mismatch", "u-shaped", "type-a"],
    {"output": {"name": "K_X", "unit": "%", "expression": "K_obs + dK_S + dD_S + d_DC + d_REF + M_1 + M_2 + M_3 + M_4", "description": T("Calibration factor of the unknown sensor", "Bilinmeyen sensörün kalibrasyon faktörü")},
     "inputs": [inp("K_obs", typea([93.45, 92.20, 93.95, 93.02]), "Observed calibration factor, four connections", "Gözlenen kalibrasyon faktörü, dört bağlantı", "%"),
                inp("dK_S", normal(0.0, U=1.1), "Calibration factor of the standard (U = 1.1 %, k = 2)", "Standardın kalibrasyon faktörü (U = %1,1, k = 2)", "%"),
                inp("dD_S", rect(0.2), "Drift of the standard since calibration", "Kalibrasyondan beri standardın sürüklenmesi", "%"),
                inp("d_DC", normal(0.0, U=0.1), "Instrumentation linearity (U = 0.1 %, k = 2)", "Ölçüm düzeni doğrusallığı (U = %0,1, k = 2)", "%"),
                inp("d_REF", rect(0.2), "Stability of the 50 MHz reference", "50 MHz referansın kararlılığı", "%"),
                inp("M_1", arc(0.08), "Mismatch, standard sensor at 50 MHz", "Uyumsuzluk, standart sensör, 50 MHz", "%"),
                inp("M_2", arc(0.08), "Mismatch, unknown sensor at 50 MHz", "Uyumsuzluk, bilinmeyen sensör, 50 MHz", "%"),
                inp("M_3", arc(1.40), "Mismatch, standard sensor at 18 GHz", "Uyumsuzluk, standart sensör, 18 GHz", "%"),
                inp("M_4", arc(1.68), "Mismatch, unknown sensor at 18 GHz", "Uyumsuzluk, bilinmeyen sensör, 18 GHz", "%")]},
    {"estimate": chk(93.16, 0.005), "standard_uncertainty": chk(1.69, 0.005), "expanded_uncertainty": chk(3.39, 0.005)},
    T("UKAS M3003 ed. 6 K.2.9–K.2.10: u_c = 1.69 %, U = 3.39 %; 93.2 % ± 3.4 %.", "UKAS M3003 6. baskı K.2.9–K.2.10: u_c = %1,69, U = %3,39; %93,2 ± %3,4."))

# ------------------------------------------------------------------ K.3
att_inputs = lambda dm: [
    inp("A_obs", typea([30.04, 30.07, 30.03, 30.06]), "Indicated attenuation A_b − A_a, four measurements", "Gösterilen zayıflama A_b − A_a, dört ölçüm", "dB"),
    inp("dA_IF", normal(0.0, U=0.005), "Calibration of the reference IF attenuator (U = 0.005 dB, k = 2)", "Referans IF zayıflatıcının kalibrasyonu (U = 0,005 dB, k = 2)", "dB"),
    inp("dD_IF", rect(0.002), "Drift of the IF attenuator", "IF zayıflatıcının sürüklenmesi", "dB"),
    inp("dL_M", rect(0.006), "Mixer non-linearity (±0.002 dB per 10 dB)", "Karıştırıcı doğrusal olmama (10 dB başına ±0,002 dB)", "dB"),
    inp("dR_D", tri(0.010), "Resolution at the 0 dB and 30 dB readings, two rectangles combined", "0 dB ve 30 dB okumalarında çözünürlük, iki dikdörtgenin birleşimi", "dB"),
    inp("dM", normal(0.0, sd=dm), "Mismatch (Harris and Warner, E.9.5)", "Uyumsuzluk (Harris ve Warner, E.9.5)", "dB"),
    inp("dA_L", rect(0.001), "Signal leakage", "Sinyal kaçağı", "dB")]
att_model = lambda dm: {"output": {"name": "A_X", "unit": "dB", "expression": "A_obs + dA_IF + dD_IF + dL_M + dR_D + dM + dA_L", "description": T("Attenuation of the 30 dB step at 10 GHz", "10 GHz'de 30 dB kademenin zayıflaması")}, "inputs": att_inputs(dm)}
att_expl = [T("A_X = A_b − A_a + δA_IF + δD_IF + δL_M + δR_D + δM + δA_L (K.3.1), dual-channel 30 MHz IF substitution at 10 GHz.", "A_X = A_b − A_a + δA_IF + δD_IF + δL_M + δR_D + δM + δA_L (K.3.1), 10 GHz'de çift kanallı 30 MHz IF yer değiştirme."),
            T("Four readings 30.04, 30.07, 30.03, 30.06 dB: mean 30.050 dB, s = 0.018 dB, u = 0.009 dB (3 degrees of freedom).", "Dört okuma 30,04, 30,07, 30,03, 30,06 dB: ortalama 30,050 dB, s = 0,018 dB, u = 0,009 dB (3 serbestlik derecesi).")]
project("electrical", "k3-attenuator-30db",
    T("30 dB coaxial attenuator at 10 GHz (UKAS M3003 K.3)", "10 GHz'de 30 dB koaksiyel zayıflatıcı (UKAS M3003 K.3)"),
    T("Calibration of a coaxial step attenuator by IF substitution; the mismatch term dominates.", "Koaksiyel kademeli zayıflatıcının IF yer değiştirme ile kalibrasyonu; uyumsuzluk terimi baskındır."),
    att_expl + [T("Mismatch standard uncertainty 0.022 dB as printed in the budget (K.3.9); u_c = 0.0245 dB; A_X = 30.050 dB ± 0.049 dB (k = 2).", "Bütçedeki gibi uyumsuzluk standart belirsizliği 0,022 dB (K.3.9); u_c = 0,0245 dB; A_X = 30,050 dB ± 0,049 dB (k = 2).")],
    "Appendix K.3", ["rf", "attenuation", "mismatch", "type-a"],
    att_model(0.022),
    {"estimate": chk(30.05, 5e-4), "standard_uncertainty": chk(0.0245, 1e-4), "expanded_uncertainty": chk(0.0491, 5e-5)},
    T("UKAS M3003 ed. 6 K.3.9–K.3.10: u_c = 0.0245 dB (0.024 57 unrounded), U = 0.0491 dB; 30.050 dB ± 0.049 dB.", "UKAS M3003 6. baskı K.3.9–K.3.10: u_c = 0,0245 dB (yuvarlanmamış 0,024 57), U = 0,0491 dB; 30,050 dB ± 0,049 dB."),
    notes=[T("Research log F57: Formula E5 with the reflection and scattering coefficients of K.3.6 gives a mismatch standard deviation of 0.0170 dB, not the 0.022 dB of the budget; see the project m3003-k3-attenuator-30db-mismatch-e5.",
             "Araştırma günlüğü F57: K.3.6'daki yansıma ve saçılma katsayılarıyla E5 formülü 0,022 dB değil 0,0170 dB uyumsuzluk standart sapması verir; m3003-k3-attenuator-30db-mismatch-e5 projesine bakınız.")])
MG = 0.03**2 * (0.05**2 + 0.05**2); ML = 0.03**2 * (0.05**2 + 0.01**2); MGL = 0.03**2 * 0.03**2 * (1 + 0.31**4)
dm_e5 = 8.686 / math.sqrt(2) * math.sqrt(MG + ML + MGL)
u_e5 = math.sqrt(0.0025**2 + (0.002/math.sqrt(3))**2 + (0.006/math.sqrt(3))**2 + (0.01/math.sqrt(6))**2 + dm_e5**2 + (0.001/math.sqrt(3))**2 + (0.018257418583505537/2)**2)
project("electrical", "k3-attenuator-30db-mismatch-e5",
    T("30 dB attenuator with the mismatch term from formula E5 (UKAS M3003 K.3, E.9.5)", "E5 formülünden uyumsuzluk terimiyle 30 dB zayıflatıcı (UKAS M3003 K.3, E.9.5)"),
    T("The budget of K.3 with the mismatch standard deviation recomputed from the coefficients the example gives (finding F57).", "Örneğin verdiği katsayılardan yeniden hesaplanan uyumsuzluk standart sapmasıyla K.3 bütçesi (bulgu F57)."),
    att_expl + [T("Formula E5 (Harris and Warner): M = 8.686/√2 · [M_G + M_L + M_GL]^½ dB with M_G = |Γ_G|²(|s11a|² + |s11b|²), M_L = |Γ_L|²(|s22a|² + |s22b|²), M_GL = |Γ_G|²|Γ_L|²(|s21a|⁴ + |s21b|⁴).",
                   "E5 formülü (Harris ve Warner): M = 8,686/√2 · [M_G + M_L + M_GL]^½ dB; M_G = |Γ_G|²(|s11a|² + |s11b|²), M_L = |Γ_L|²(|s22a|² + |s22b|²), M_GL = |Γ_G|²|Γ_L|²(|s21a|⁴ + |s21b|⁴)."),
                 T(f"With Γ_G = Γ_L = 0.03, s11a = s11b = 0.05, s22a = 0.05, s22b = 0.01, s21a = 1, s21b = 0.31 (K.3.6): M = {dm_e5:.4f} dB, and u_c = {u_e5:.4f} dB instead of 0.0245 dB.",
                   f"Γ_G = Γ_L = 0,03, s11a = s11b = 0,05, s22a = 0,05, s22b = 0,01, s21a = 1, s21b = 0,31 ile (K.3.6): M = {dm_e5:.4f} dB ve u_c = 0,0245 dB yerine {u_e5:.4f} dB.".replace(f"{dm_e5:.4f}", f"{dm_e5:.4f}".replace(".", ",")).replace(f"{u_e5:.4f}", f"{u_e5:.4f}".replace(".", ",")))],
    "Appendix K.3 and E.9.5", ["rf", "attenuation", "mismatch", "type-a", "finding"],
    att_model(dm_e5),
    {"estimate": chk(30.05, 5e-4), "standard_uncertainty": chk(u_e5, 5e-6), "expanded_uncertainty": chk(2 * u_e5, 1e-5)},
    T("Recomputed from formula E5 and the data of K.3.6 (research log F57); not a printed result.", "E5 formülü ve K.3.6 verilerinden yeniden hesaplanmıştır (araştırma günlüğü F57); basılı bir sonuç değildir."))

# ------------------------------------------------------------------ K.4
project("mass", "k4-weight-10kg",
    T("10 kg weight of OIML class M1 (UKAS M3003 K.4)", "OIML M1 sınıfı 10 kg ağırlık (UKAS M3003 K.4)"),
    T("Comparison with a class F2 standard on a mass comparator, three bracketed comparisons.", "Kütle karşılaştırıcısında F2 sınıfı bir standartla üç ara-okumalı karşılaştırma."),
    [T("W_X = W_S + ΔW + δD_S + δI_d + ΔC + ΔA_b + δW_r (K.4.1): certificate 30 mg (k = 2), drift ±30 mg, rounding ±10 mg (triangular, two readings), non-linearity ±3 mg, buoyancy ±10 mg.",
       "W_X = W_S + ΔW + δD_S + δI_d + ΔC + ΔA_b + δW_r (K.4.1): sertifika 30 mg (k = 2), sürüklenme ±30 mg, yuvarlama ±10 mg (üçgen, iki okuma), doğrusal olmama ±3 mg, kaldırma kuvveti ±10 mg."),
     T("Mean difference ΔW = +0.020 g from three comparisons; repeatability from an earlier evaluation, s = 8.7 mg with 9 degrees of freedom, so u = 8.7/√3 = 5.0 mg.",
       "Üç karşılaştırmadan ortalama fark ΔW = +0,020 g; tekrarlanabilirlik önceki bir değerlendirmeden, s = 8,7 mg, 9 serbestlik derecesi; u = 8,7/√3 = 5,0 mg."),
     T("u_c = 24.56 mg; W_X = 10 000.025 g ± 0.049 g (k = 2).", "u_c = 24,56 mg; W_X = 10 000,025 g ± 0,049 g (k = 2).")],
    "Appendix K.4", ["mass", "weight", "comparator", "pooled-repeatability"],
    {"output": {"name": "W_X", "unit": "g", "expression": "W_S + dW + dD_S + dI_d + dC + dA_b", "description": T("Mass of the unknown weight", "Bilinmeyen ağırlığın kütlesi")},
     "inputs": [inp("W_S", normal(10000.005, U=0.030), "Standard weight (certificate, U = 30 mg, k = 2)", "Standart ağırlık (sertifika, U = 30 mg, k = 2)", "g"),
                inp("dW", pooled(0.020, 0.0087, 9, 3), "Mean difference of three comparisons (repeatability s = 8.7 mg, ν = 9)", "Üç karşılaştırmanın ortalama farkı (tekrarlanabilirlik s = 8,7 mg, ν = 9)", "g"),
                inp("dD_S", rect(0.030), "Drift of the standard", "Standardın sürüklenmesi", "g"),
                inp("dI_d", tri(0.010), "Rounding of the two indications (10 mg digit)", "İki göstergenin yuvarlanması (10 mg basamak)", "g"),
                inp("dC", rect(0.003), "Comparator non-linearity", "Karşılaştırıcı doğrusal olmama", "g"),
                inp("dA_b", rect(0.010), "Air buoyancy (±1 ppm of nominal value)", "Hava kaldırma kuvveti (nominal değerin ±1 ppm'i)", "g")]},
    {"estimate": chk(10000.025, 5e-4), "standard_uncertainty": chk(0.02456, 5e-6), "expanded_uncertainty": chk(0.04912, 1e-5)},
    T("UKAS M3003 ed. 6 K.4.10–K.4.11: u_c = 24.56 mg, U = 49.12 mg; 10 000.025 g ± 0.049 g.", "UKAS M3003 6. baskı K.4.10–K.4.11: u_c = 24,56 mg, U = 49,12 mg; 10 000,025 g ± 0,049 g."),
    notes=[T("Research log F55: K.4.7 states ν = (m − 1) = 10 degrees of freedom for m = 10 comparisons; the budget uses 9, which is m − 1. The note below the budget refers to 10 readings.",
             "Araştırma günlüğü F55: K.4.7, m = 10 karşılaştırma için ν = (m − 1) = 10 serbestlik derecesi yazar; bütçe m − 1 olan 9'u kullanır. Bütçenin altındaki not 10 okumadan söz eder.")])

# ------------------------------------------------------------------ K.5
project("mass", "k5-weighing-machine-200g",
    T("Weighing machine of 205 g capacity at 200 g (UKAS M3003 K.5)", "200 g'de 205 g kapasiteli tartı aleti (UKAS M3003 K.5)"),
    T("Error of indication of an electronic balance with a 0.1 mg digit, loaded with a class E2 200 g weight; one reading.", "0,1 mg basamaklı elektronik terazinin E2 sınıfı 200 g ağırlıkla gösterge hatası; tek okuma."),
    [T("ΔI_X = I_X − W_S + δD_S + δI_d0 + δI_d + ΔA_b + δI_r (K.5.1): certificate 0.1 mg (k = 2), drift ±0.1 mg, rounding ±0.05 mg at zero and at load, buoyancy ±0.2 mg, repeatability s = 0.05 mg (ν = 9, one reading).",
       "ΔI_X = I_X − W_S + δD_S + δI_d0 + δI_d + ΔA_b + δI_r (K.5.1): sertifika 0,1 mg (k = 2), sürüklenme ±0,1 mg, sıfırda ve yükte ±0,05 mg yuvarlama, kaldırma kuvveti ±0,2 mg, tekrarlanabilirlik s = 0,05 mg (ν = 9, tek okuma)."),
     T("I_X = 199.9999 g for the 200 g standard (nominal value taken as its mass), so ΔI_X = −0.10 mg by the model.", "200 g standart için I_X = 199,9999 g (kütlesi nominal değer alınmıştır); model ile ΔI_X = −0,10 mg."),
     T("u_c = 0.153 mg from the contributions of the budget; ΔI_X = −0.10 mg ± 0.31 mg (k = 2).", "Bütçe katkılarından u_c = 0,153 mg; ΔI_X = −0,10 mg ± 0,31 mg (k = 2).")],
    "Appendix K.5", ["mass", "balance", "error-of-indication", "finding"],
    {"output": {"name": "dI_X", "unit": "mg", "expression": "I_X - W_S + dD_S + dI_d0 + dI_d + dA_b + dI_r", "description": T("Error of indication at 200 g", "200 g'de gösterge hatası")},
     "inputs": [inp("I_X", const(199999.9), "Indication (199.9999 g)", "Gösterge (199,9999 g)", "mg"),
                inp("W_S", normal(200000.0, U=0.1), "Standard weight (U = 0.1 mg, k = 2)", "Standart ağırlık (U = 0,1 mg, k = 2)", "mg"),
                inp("dD_S", rect(0.1), "Drift of the standard", "Standardın sürüklenmesi", "mg"),
                inp("dI_d0", rect(0.05), "Rounding at zero", "Sıfırda yuvarlama", "mg"),
                inp("dI_d", rect(0.05), "Rounding of the indication", "Göstergenin yuvarlanması", "mg"),
                inp("dA_b", rect(0.2), "Air buoyancy (±1 ppm)", "Hava kaldırma kuvveti (±1 ppm)", "mg"),
                inp("dI_r", pooled(0.0, 0.05, 9, 1), "Repeatability (s = 0.05 mg, ν = 9, one reading)", "Tekrarlanabilirlik (s = 0,05 mg, ν = 9, tek okuma)", "mg")]},
    {"estimate": chk(-0.1, 1e-6), "standard_uncertainty": chk(0.15275, 5e-6), "expanded_uncertainty": chk(0.3055, 1e-4)},
    T("Recomputed from the contributions of K.5.9 (research log F56). M3003 prints u_c = 0.150 mg, U = 0.300 mg and the result 0.10 mg ± 0.30 mg.",
      "K.5.9'daki katkılardan yeniden hesaplanmıştır (araştırma günlüğü F56). M3003 u_c = 0,150 mg, U = 0,300 mg ve 0,10 mg ± 0,30 mg sonucunu verir."),
    notes=[T("Research log F56: the contributions printed in the u_i column of K.5.9 (0.05, 0.058, 0.029, 0.029, 0.115, 0.05 mg) combine to 0.153 mg, not 0.150 mg; the printed value would follow if the drift limit were divided by 2 instead of √3. By the model of K.5.1, ΔI_X = I_X − W_S = 199.9999 g − 200 g = −0.10 mg, while the reported error of indication is +0.10 mg.",
             "Araştırma günlüğü F56: K.5.9'un u_i sütunundaki katkılar (0,05, 0,058, 0,029, 0,029, 0,115, 0,05 mg) 0,150 mg değil 0,153 mg verir; basılı değer, sürüklenme sınırı √3 yerine 2'ye bölünseydi elde edilirdi. K.5.1 modeline göre ΔI_X = I_X − W_S = 199,9999 g − 200 g = −0,10 mg iken raporlanan gösterge hatası +0,10 mg'dir.")])

# ------------------------------------------------------------------ K.6
gb_common = [inp("L_SdL", normal(9.99994, U=3e-5), "L_S + ΔL: certified length of the standard plus the measured difference (only the sum is given, K.6.9); certificate U = 30 nm (k = 2)",
                 "L_S + ΔL: standardın sertifikalı uzunluğu artı ölçülen fark (yalnızca toplam verilmiştir, K.6.9); sertifika U = 30 nm (k = 2)", "mm"),
             inp("dL_D", tri(1.5e-5), "Drift of the standard (±15 nm, triangular)", "Standardın sürüklenmesi (±15 nm, üçgen)", "mm"),
             inp("dD_C", rect(5e-5), "Comparator discrimination and linearity (±0.05 µm)", "Karşılaştırıcı ayırma gücü ve doğrusallığı (±0,05 µm)", "mm"),
             inp("dC", rect(5e-6), "Difference in elastic compression (±0.005 µm)", "Elastik sıkışma farkı (±0,005 µm)", "mm"),
             inp("L_V", tri(4.2e-5), "Length variation of the unknown block (±42 nm, triangular)", "Bilinmeyen bloğun uzunluk değişimi (±42 nm, üçgen)", "mm"),
             inp("dL_r", pooled(0.0, 1.6e-5, 10, 1), "Repeatability (s = 16 nm from 11 earlier measurements, one reading)", "Tekrarlanabilirlik (11 önceki ölçümden s = 16 nm, tek okuma)", "mm")]
gb_expl = [T("L_X = L_S + δL_D + ΔL − L(α·Δt + Δα·ΔT) + ΔD_C + δC + L_V(X) + δL_r (K.6.1); grade 2 block of 10 mm against a grade K standard.", "L_X = L_S + δL_D + ΔL − L(α·Δt + Δα·ΔT) + ΔD_C + δC + L_V(X) + δL_r (K.6.1); 10 mm'lik 2. sınıf blok, K sınıfı standarda karşı.")]
project("length", "k6-gauge-block-10mm",
    T("Grade 2 gauge block of 10 mm (UKAS M3003 K.6)", "10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6)"),
    T("Comparison with a standard gauge block; the budget as printed, with the two temperature effects added linearly into one triangular term.", "Standart mastar blokla karşılaştırma; iki sıcaklık etkisinin doğrusal toplanıp tek üçgen terim yapıldığı basılı bütçe."),
    gb_expl + [T("Temperature effects as in K.6.4: ±1.6 nm from Δt and ±4 nm from ΔT, both through Δα = ±2 µm m⁻¹ °C⁻¹, treated as correlated and added: ±5.6 nm, triangular.",
                 "K.6.4'teki gibi sıcaklık etkileri: Δt'den ±1,6 nm ve ΔT'den ±4 nm, ikisi de Δα = ±2 µm m⁻¹ °C⁻¹ üzerinden, korelasyonlu kabul edilip toplanır: ±5,6 nm, üçgen."),
               T("u_c = 40.7 nm; L_X = 9.999 940 mm ± 0.081 µm (k = 2).", "u_c = 40,7 nm; L_X = 9,999 940 mm ± 0,081 µm (k = 2).")],
    "Appendix K.6", ["length", "gauge-block", "comparator"],
    {"output": {"name": "L_X", "unit": "mm", "expression": "L_SdL + dL_D + dD_C + dC + dT_SX + L_V + dL_r", "description": T("Length of the unknown gauge block", "Bilinmeyen mastar bloğun uzunluğu")},
     "inputs": gb_common[:4] + [inp("dT_SX", tri(5.6e-6), "Temperature effects, added linearly (±5.6 nm, triangular)", "Sıcaklık etkileri, doğrusal toplanmış (±5,6 nm, üçgen)", "mm")] + gb_common[4:]},
    {"estimate": chk(9.99994, 5e-9), "standard_uncertainty": chk(4.07e-5, 5e-8), "expanded_uncertainty": chk(8.15e-5, 5e-8)},
    T("UKAS M3003 ed. 6 K.6.10–K.6.11: u_c = 40.7 nm, U = 81.5 nm; 9.999 940 mm ± 0.081 µm.", "UKAS M3003 6. baskı K.6.10–K.6.11: u_c = 40,7 nm, U = 81,5 nm; 9,999 940 mm ± 0,081 µm."),
    notes=[T("Research log F58: by the model of K.6.1 the temperature difference Δt acts through the mean expansion coefficient α (L·α = 115 nm/°C, ±9.2 nm), not through Δα; the product Δα·ΔT has zero first-order sensitivity. See m3003-k6-gauge-block-10mm-temperature-model.",
             "Araştırma günlüğü F58: K.6.1 modeline göre Δt sıcaklık farkı Δα üzerinden değil ortalama genleşme katsayısı α üzerinden etki eder (L·α = 115 nm/°C, ±9,2 nm); Δα·ΔT çarpımının birinci mertebe duyarlılığı sıfırdır. m3003-k6-gauge-block-10mm-temperature-model projesine bakınız.")])
project("length", "k6-gauge-block-10mm-temperature-model",
    T("Grade 2 gauge block of 10 mm with the temperature terms of its model (UKAS M3003 K.6, EA-4/02 S4)", "Modelinin sıcaklık terimleriyle 10 mm'lik 2. sınıf mastar blok (UKAS M3003 K.6, EA-4/02 S4)"),
    T("The budget of K.6 with −L(α·Δt + Δα·ΔT) evaluated as the model states, as EA-4/02 S4 does for a 50 mm block (finding F58).", "−L(α·Δt + Δα·ΔT) teriminin modelin belirttiği gibi değerlendirildiği K.6 bütçesi; EA-4/02 S4'ün 50 mm'lik blok için yaptığı gibi (bulgu F58)."),
    gb_expl + [T("Δt (±0.08 °C, rectangular) acts with sensitivity −L·α = −115 nm/°C: u = 5.3 nm. Δα (±2 µm m⁻¹ °C⁻¹, triangular) and ΔT (±0.2 °C, rectangular) both have zero expectation, so their product contributes only through the second-order term L·u(Δα)·u(ΔT) = 0.94 nm (GUM 5.1.2 note).",
                 "Δt (±0,08 °C, dikdörtgen) −L·α = −115 nm/°C duyarlılıkla etki eder: u = 5,3 nm. Δα (±2 µm m⁻¹ °C⁻¹, üçgen) ve ΔT (±0,2 °C, dikdörtgen) beklenen değerleri sıfırdır; çarpımları yalnızca ikinci mertebe terim L·u(Δα)·u(ΔT) = 0,94 nm ile katkı verir (GUM 5.1.2 notu)."),
               T("u_c = 41.0 nm instead of 40.7 nm; U = 82 nm (k = 2).", "u_c = 40,7 nm yerine 41,0 nm; U = 82 nm (k = 2).")],
    "Appendix K.6", ["length", "gauge-block", "comparator", "higher-order", "finding"],
    {"output": {"name": "L_X", "unit": "mm", "expression": "L_SdL + dL_D + dD_C + dC - L*(alpha*dt + d_alpha*dT) + L_V + dL_r", "description": T("Length of the unknown gauge block", "Bilinmeyen mastar bloğun uzunluğu")},
     "inputs": gb_common[:4] + [inp("L", const(10.0), "Nominal length", "Nominal uzunluk", "mm"),
                                inp("alpha", const(11.5e-6), "Mean expansion coefficient", "Ortalama genleşme katsayısı", "1/°C"),
                                inp("dt", rect(0.08), "Temperature difference of the two blocks", "İki blok arasındaki sıcaklık farkı", "°C"),
                                inp("d_alpha", tri(2e-6), "Difference of the expansion coefficients", "Genleşme katsayıları farkı", "1/°C"),
                                inp("dT", rect(0.2), "Deviation of the mean temperature from 20 °C", "Ortalama sıcaklığın 20 °C'den sapması", "°C")] + gb_common[4:]},
    {"estimate": chk(9.99994, 5e-9), "standard_uncertainty": chk(4.1028e-5, 5e-9), "expanded_uncertainty": chk(8.2055e-5, 1e-8)},
    T("Recomputed from the data of K.6 with the model of K.6.1 (research log F58); not a printed result.", "K.6 verilerinden K.6.1 modeliyle yeniden hesaplanmıştır (araştırma günlüğü F58); basılı bir sonuç değildir."),
    analysis={"gum": {"coverage_factor": 2, "higher_order": True}, "mcm": {"trials": 200000, "seed": 1}})

# ------------------------------------------------------------------ K.7
furnace_inputs = [
    inp("t_S", normal(1000.5, U=0.3), "Reference thermocouples, mean of the two (certificate U = 0.3 °C, k = 2)", "Referans termoçiftler, ikisinin ortalaması (sertifika U = 0,3 °C, k = 2)", "°C"),
    inp("c_S", const(0.077), "Sensitivity of the R-type thermocouple at 1000 °C", "R tipi termoçiftin 1000 °C'deki duyarlılığı", "°C/µV"),
    inp("c_S0", const(0.189), "Sensitivity of the R-type thermocouple at 0 °C", "R tipi termoçiftin 0 °C'deki duyarlılığı", "°C/µV"),
    inp("dV_iS1", normal(0.0, U=2.0), "Microvoltmeter calibration (U = 2.0 µV, k = 2)", "Mikrovoltmetre kalibrasyonu (U = 2,0 µV, k = 2)", "µV"),
    inp("dV_iS2", rect(0.5), "Microvoltmeter resolution (1 µV digit)", "Mikrovoltmetre çözünürlüğü (1 µV basamak)", "µV"),
    inp("dV_R", rect(2.0), "Parasitic voltages of the switch contacts", "Anahtar kontaklarının parazit gerilimleri", "µV"),
    inp("dt_0S", rect(0.1), "Reference junction temperature of the standards", "Standartların referans bağlantı sıcaklığı", "°C"),
    inp("dt_D", rect(0.3), "Drift of the reference thermocouples", "Referans termoçiftlerin sürüklenmesi", "°C"),
    inp("dt_F", rect(1.0), "Non-uniformity of the furnace", "Fırının homojen olmaması", "°C"),
    inp("dt_r", pooled(0.0, 0.10, 9, 1), "Repeatability of the furnace temperature (s = 0.10 °C, ν = 9)", "Fırın sıcaklığının tekrarlanabilirliği (s = 0,10 °C, ν = 9)", "°C")]
furnace_expr = "t_S + c_S*dV_iS1 + c_S*dV_iS2 + c_S*dV_R - c_S/c_S0*dt_0S + dt_D + dt_F + dt_r"
tc_expl = [T("An N-type thermocouple is compared with two R-type reference thermocouples at 1000 °C; four readings each with reversed polarity give 10 502.5 µV and 10 504.0 µV (1000.4 °C and 1000.6 °C) for the standards and 36 248 µV for the unknown.",
             "N tipi bir termoçift 1000 °C'de iki R tipi referans termoçiftle karşılaştırılır; ters çevrilmiş kutuplamayla dörder okuma standartlar için 10 502,5 µV ve 10 504,0 µV (1000,4 °C ve 1000,6 °C), bilinmeyen için 36 248 µV verir."),
           T("Furnace temperature t_X = t_S + c_S(δV_iS1 + δV_iS2 + δV_R) − (c_S/c_S0)·δt_0S + δt_D + δt_F + δt_r (K.7.2), mean 1000.5 °C; u_c(t_X) = 0.641 °C (K.7.19), dominated by the furnace non-uniformity ±1 °C.",
             "Fırın sıcaklığı t_X = t_S + c_S(δV_iS1 + δV_iS2 + δV_R) − (c_S/c_S0)·δt_0S + δt_D + δt_F + δt_r (K.7.2), ortalama 1000,5 °C; u_c(t_X) = 0,641 °C (K.7.19); ±1 °C fırın homojensizliği baskındır.")]
project("temperature", "k7-thermocouple-furnace",
    T("Temperature of the furnace in a thermocouple calibration at 1000 °C (UKAS M3003 K.7, first budget)", "1000 °C'de termoçift kalibrasyonunda fırın sıcaklığı (UKAS M3003 K.7, birinci bütçe)"),
    T("First part of K.7: the temperature at the hot junctions from the two reference thermocouples.", "K.7'nin birinci kısmı: iki referans termoçiftten sıcak bağlantılardaki sıcaklık."),
    tc_expl, "Appendix K.7 (K.7.19)", ["temperature", "thermocouple", "furnace"],
    {"output": {"name": "t_X", "unit": "°C", "expression": furnace_expr, "description": T("Temperature of the furnace at the thermocouples", "Termoçiftlerin bulunduğu yerde fırın sıcaklığı")}, "inputs": furnace_inputs},
    {"estimate": chk(1000.5, 1e-9), "standard_uncertainty": chk(0.641, 5e-4)},
    T("UKAS M3003 ed. 6 K.7.19: u_c(t_X) = 0.641 °C.", "UKAS M3003 6. baskı K.7.19: u_c(t_X) = 0,641 °C."))
project("temperature", "k7-thermocouple-emf",
    T("N-type thermocouple at 1000 °C: EMF with its full budget (UKAS M3003 K.7)", "1000 °C'de N tipi termoçift: tam bütçesiyle EMK (UKAS M3003 K.7)"),
    T("Both budgets of K.7 in one model: the furnace temperature enters the EMF of the unknown thermocouple through its sensitivity 1/c_X = 38.5 µV/°C.", "K.7'nin iki bütçesi tek modelde: fırın sıcaklığı bilinmeyen termoçiftin EMK'sına 1/c_X = 38,5 µV/°C duyarlılığıyla girer."),
    tc_expl + [T("V_X(1000 °C) = V_iX + (1000 °C − t_X)/c_X + δV_iX1 + δV_iX2 + δV_LX + δt_TH/c_X + δV_R − δt_0X/c_X0 + δV_r (K.7.2, K.7.20) with c_X = 0.026 °C/µV and c_X0 = 0.038 °C/µV.",
                 "V_X(1000 °C) = V_iX + (1000 °C − t_X)/c_X + δV_iX1 + δV_iX2 + δV_LX + δt_TH/c_X + δV_R − δt_0X/c_X0 + δV_r (K.7.2, K.7.20); c_X = 0,026 °C/µV ve c_X0 = 0,038 °C/µV."),
               T("u_c = 25.8 µV; V_X = 36 230 µV ± 52 µV (k = 2) at 1000 °C with the reference junction at 0 °C.", "u_c = 25,8 µV; referans bağlantı 0 °C'de iken 1000 °C'de V_X = 36 230 µV ± 52 µV (k = 2).")],
    "Appendix K.7", ["temperature", "thermocouple", "emf", "intermediate"],
    {"output": {"name": "V_X", "unit": "µV", "expression": "V_iX + (1000 - t_X)/c_X + dV_iX1 + dV_iX2 + dV_LX + dt_TH/c_X + dV_R2 - dt_0X/c_X0 + dV_r", "description": T("EMF of the N-type thermocouple at 1000 °C", "N tipi termoçiftin 1000 °C'deki EMK'sı")},
     "intermediates": [{"name": "t_X", "unit": "°C", "expression": furnace_expr, "description": T("Furnace temperature (first budget)", "Fırın sıcaklığı (birinci bütçe)")}],
     "inputs": furnace_inputs + [
         inp("V_iX", const(36248.0), "Mean indication for the unknown thermocouple", "Bilinmeyen termoçift için ortalama gösterge", "µV"),
         inp("c_X", const(0.026), "Sensitivity of the N-type thermocouple at 1000 °C", "N tipi termoçiftin 1000 °C'deki duyarlılığı", "°C/µV"),
         inp("c_X0", const(0.038), "Sensitivity of the N-type thermocouple at 0 °C", "N tipi termoçiftin 0 °C'deki duyarlılığı", "°C/µV"),
         inp("dV_iX1", normal(0.0, U=2.0), "Microvoltmeter calibration (U = 2.0 µV, k = 2)", "Mikrovoltmetre kalibrasyonu (U = 2,0 µV, k = 2)", "µV"),
         inp("dV_iX2", rect(0.5), "Microvoltmeter resolution", "Mikrovoltmetre çözünürlüğü", "µV"),
         inp("dV_LX", rect(5.0), "Compensating leads", "Kompanzasyon kabloları", "µV"),
         inp("dt_TH", rect(0.3), "Inhomogeneity of the unknown thermocouple", "Bilinmeyen termoçiftin homojen olmaması", "°C"),
         inp("dV_R2", rect(2.0), "Switch contacts (reading of the unknown)", "Anahtar kontakları (bilinmeyenin okuması)", "µV"),
         inp("dt_0X", rect(0.1), "Reference junction temperature of the unknown", "Bilinmeyenin referans bağlantı sıcaklığı", "°C"),
         inp("dV_r", pooled(0.0, 1.6, 9, 1), "Repeatability of the voltage (s = 1.6 µV, ν = 9)", "Gerilimin tekrarlanabilirliği (s = 1,6 µV, ν = 9)", "µV")]},
    {"estimate": chk(36230.0, 5.0), "standard_uncertainty": chk(25.9, 0.1), "expanded_uncertainty": chk(51.9, 0.3)},
    T("UKAS M3003 ed. 6 K.7.20–K.7.21: u_c = 25.9 µV, U = 51.9 µV; 36 230 µV ± 52 µV (the estimate to the printed 10 µV).", "UKAS M3003 6. baskı K.7.20–K.7.21: u_c = 25,9 µV, U = 51,9 µV; 36 230 µV ± 52 µV (tahmin basılı 10 µV çözünürlükte)."),
    notes=[T("Research log F59: the reported EMF 36 230 µV is obtained by scaling, 36 248·1000/1000.5, whereas the uncertainty budget uses the thermocouple sensitivity 38.5 µV/°C, which gives 36 248 − 0.5·38.46 = 36 228.8 µV. The printed u_c = 25.9 µV and U = 51.9 µV follow from rounded sensitivities (38.5 and 26.3); the exact ones give 25.84 µV and 51.7 µV.",
             "Araştırma günlüğü F59: raporlanan 36 230 µV EMK ölçeklemeyle, 36 248·1000/1000,5, elde edilir; belirsizlik bütçesi ise termoçiftin 38,5 µV/°C duyarlılığını kullanır, bu da 36 248 − 0,5·38,46 = 36 228,8 µV verir. Basılı u_c = 25,9 µV ve U = 51,9 µV yuvarlanmış duyarlılıklardan (38,5 ve 26,3) çıkar; tam değerler 25,84 µV ve 51,7 µV verir.")])

# ------------------------------------------------------------------ K.8
project("pressure", "k8-pressure-indicator-2mpa",
    T("Digital pressure indicator at 2 MPa against a dead weight tester (UKAS M3003 K.8)", "Ölü ağırlık test cihazına karşı 2 MPa'da dijital basınç göstergesi (UKAS M3003 K.8)"),
    T("Error of a DPI at 2 MPa; the budget is in relative terms (ppm), as is the convention for pressure.", "2 MPa'da bir dijital basınç göstergesinin hatası; bütçe, basınçta alışıldığı gibi göreli (ppm) olarak verilir."),
    [T("Generated pressure P_X = 2.000 806 MPa from the masses, the effective area A₀ = 80.6516 mm², gravity, buoyancy and fluid head (K.8.15); indication I_X = 2.000 84 MPa.", "Kütlelerden, etkin alan A₀ = 80,6516 mm², yerçekimi, kaldırma kuvveti ve sıvı yükünden üretilen basınç P_X = 2,000 806 MPa (K.8.15); gösterge I_X = 2,000 84 MPa."),
     T("Relative contributions to P_X: area 32.2 ppm (k = 2) and drift ±30 ppm; distortion 1.0 ppm (k = 2); masses 7.3 ppm (k = 2, as in the budget) and drift ±7.6 ppm; piston temperature ±0.5 °C × 23 ppm/°C; gravity 3 ppm (k = 2); buoyancy ±13 ppm; fluid head ±13.5 ppm; other fluid effects ±2 ppm; verticality ±3.4 ppm.",
       "P_X'e göreli katkılar: alan 32,2 ppm (k = 2) ve sürüklenme ±30 ppm; deformasyon 1,0 ppm (k = 2); kütleler 7,3 ppm (k = 2, bütçedeki gibi) ve sürüklenme ±7,6 ppm; piston sıcaklığı ±0,5 °C × 23 ppm/°C; yerçekimi 3 ppm (k = 2); kaldırma kuvveti ±13 ppm; sıvı yükü ±13,5 ppm; diğer sıvı etkileri ±2 ppm; düşeylik ±3,4 ppm."),
     T("Indication: rounding ±50 ppm (200 Pa steps) and repeatability 16 ppm (ν = 9, one reading). u_c = 43.0 ppm; ΔI_X = 17 ppm ± 86 ppm (k = 2), i.e. 34 Pa ± 170 Pa.",
       "Gösterge: yuvarlama ±50 ppm (200 Pa adım) ve tekrarlanabilirlik 16 ppm (ν = 9, tek okuma). u_c = 43,0 ppm; ΔI_X = 17 ppm ± 86 ppm (k = 2), yani 34 Pa ± 170 Pa.")],
    "Appendix K.8", ["pressure", "dead-weight-tester", "relative"],
    {"output": {"name": "e", "unit": "ppm", "expression": "1e6*(I_X - P_X)/P_X - (A_cal + a_p + A_drift + m_cal + m_drift + lambda*dt + g + air + head + fluid + level) + d_res + d_rep",
                "description": T("Relative error of indication of the DPI", "Dijital basınç göstergesinin göreli gösterge hatası")},
     "inputs": [inp("I_X", const(2.00084), "Indication", "Gösterge", "MPa"), inp("P_X", const(2.000806), "Generated pressure", "Üretilen basınç", "MPa"),
                inp("A_cal", normal(0.0, U=32.2), "Effective area, calibration (U = 32.2 ppm, k = 2)", "Etkin alan, kalibrasyon (U = 32,2 ppm, k = 2)", "ppm"),
                inp("a_p", normal(0.0, U=1.0), "Distortion coefficient at 2 MPa (U = 1.0 ppm, k = 2)", "2 MPa'da deformasyon katsayısı (U = 1,0 ppm, k = 2)", "ppm"),
                inp("A_drift", rect(30.0), "Drift of the effective area", "Etkin alanın sürüklenmesi", "ppm"),
                inp("m_cal", normal(0.0, U=7.3), "Calibration of the total load (U = 7.3 ppm, k = 2)", "Toplam yükün kalibrasyonu (U = 7,3 ppm, k = 2)", "ppm"),
                inp("m_drift", rect(7.6), "Drift of the total load", "Toplam yükün sürüklenmesi", "ppm"),
                inp("lambda", const(23.0), "Temperature coefficient of piston and cylinder", "Piston ve silindirin sıcaklık katsayısı", "ppm/°C"),
                inp("dt", rect(0.5), "Temperature of the piston", "Piston sıcaklığı", "°C"),
                inp("g", normal(0.0, U=3.0), "Local gravity (U = 3 ppm, k = 2)", "Yerel yerçekimi (U = 3 ppm, k = 2)", "ppm"),
                inp("air", rect(13.0), "Air buoyancy approximation", "Hava kaldırma kuvveti yaklaşımı", "ppm"),
                inp("head", rect(13.5), "Fluid head (3 mm of oil)", "Sıvı yükü (3 mm yağ)", "ppm"),
                inp("fluid", rect(2.0), "Buoyancy volume, surface tension, fluid density", "Kaldırma hacmi, yüzey gerilimi, sıvı yoğunluğu", "ppm"),
                inp("level", rect(3.4), "Verticality of the piston (0.15°)", "Pistonun düşeyliği (0,15°)", "ppm"),
                inp("d_res", rect(50.0), "Rounding of the indication (200 Pa steps)", "Göstergenin yuvarlanması (200 Pa adım)", "ppm"),
                inp("d_rep", pooled(0.0, 16.0, 9, 1), "Repeatability (s = 16 ppm, ν = 9, one reading)", "Tekrarlanabilirlik (s = 16 ppm, ν = 9, tek okuma)", "ppm")]},
    {"estimate": chk(17.0, 0.5), "standard_uncertainty": chk(43.0, 0.05), "expanded_uncertainty": chk(86.0, 0.05)},
    T("UKAS M3003 ed. 6 K.8.16–K.8.17: u_c = 43.0 ppm, U = 86.0 ppm; 17 ppm ± 86 ppm.", "UKAS M3003 6. baskı K.8.16–K.8.17: u_c = 43,0 ppm, U = 86,0 ppm; 17 ppm ± 86 ppm."),
    notes=[T("Research log F60: K.8.5.1 says the relative mass uncertainty of 7.3 ppm is to be treated as the semi-range of a rectangular distribution, but the budget treats it as an expanded uncertainty with k = 2 (u = 3.65 ppm instead of 4.21 ppm). The effect on u_c is below its last printed digit (43.0 ppm either way).",
             "Araştırma günlüğü F60: K.8.5.1, 7,3 ppm'lik göreli kütle belirsizliğinin dikdörtgen dağılımın yarı genişliği olarak alınacağını söyler; bütçe ise onu k = 2 ile genişletilmiş belirsizlik olarak alır (u = 4,21 ppm yerine 3,65 ppm). u_c'ye etkisi son basılı basamağın altındadır (her iki durumda 43,0 ppm).")])

# ------------------------------------------------------------------ K.9
project("flow", "k9-flowrate-standard-conditions",
    T("Volume flowrate referred to standard pressure and temperature (UKAS M3003 K.9)", "Standart basınç ve sıcaklığa indirgenmiş hacimsel debi (UKAS M3003 K.9)"),
    T("Flowrate from a 4–20 mA flowmeter at 1018 hPa and 295.1 K, expressed at 1013.25 hPa and 293.15 K; three partial budgets in one model.",
      "1018 hPa ve 295,1 K'de 4–20 mA'lik bir debimetreden debi, 1013,25 hPa ve 293,15 K'de ifade edilir; üç kısmi bütçe tek modelde."),
    [T("Q_s = Q_m·(P_m/P_s)·(T_s/T_m) + δQ_s (K.9.2), with Q_m = A·i + B and A = 6.25 L s⁻¹ mA⁻¹, B = −24.95 L s⁻¹ (K.9.4): i = 6.0 mA gives Q_m = 12.55 L s⁻¹.",
       "Q_s = Q_m·(P_m/P_s)·(T_s/T_m) + δQ_s (K.9.2); Q_m = A·i + B, A = 6,25 L s⁻¹ mA⁻¹, B = −24,95 L s⁻¹ (K.9.4): i = 6,0 mA, Q_m = 12,55 L s⁻¹ verir."),
     T("Partial budgets: u(Q_m) = 0.207 L s⁻¹ (calibration, drift, environment, current); u(P_m) = 1.322 hPa; u(T_m) = 0.144 K; repeatability of Q_s s = 0.18 L s⁻¹ (ν = 24, one reading).",
       "Kısmi bütçeler: u(Q_m) = 0,207 L s⁻¹ (kalibrasyon, sürüklenme, ortam, akım); u(P_m) = 1,322 hPa; u(T_m) = 0,144 K; Q_s tekrarlanabilirliği s = 0,18 L s⁻¹ (ν = 24, tek okuma)."),
     T("Q_s = 12.53 L s⁻¹, u_c = 0.275 L s⁻¹, ν_eff = 130; Q_s = (12.53 ± 0.55) L s⁻¹ (k = 2).", "Q_s = 12,53 L s⁻¹, u_c = 0,275 L s⁻¹, ν_eff = 130; Q_s = (12,53 ± 0,55) L s⁻¹ (k = 2).")],
    "Appendix K.9", ["flow", "standard-conditions", "intermediate"],
    {"output": {"name": "Q_s", "unit": "L/s", "expression": "Q_m*(P_m/P_s)*(T_s/T_m) + dQ_s", "description": T("Flowrate at standard conditions", "Standart koşullarda debi")},
     "intermediates": [{"name": "Q_m", "unit": "L/s", "expression": "A*i + B + dq_cal - dq_d - dq_T", "description": T("Flowrate at the meter", "Debimetrede debi")},
                       {"name": "P_m", "unit": "hPa", "expression": "p_m - dp_m - dp_f - dp_d - dp_T", "description": T("Fluid pressure at the meter", "Debimetrede akışkan basıncı")},
                       {"name": "T_m", "unit": "K", "expression": "t_m - dt_m - dt_f - dt_d - dt_T", "description": T("Fluid temperature at the meter", "Debimetrede akışkan sıcaklığı")}],
     "inputs": [inp("A", const(6.25), "Slope of the calibration function", "Kalibrasyon fonksiyonunun eğimi", "L s⁻¹/mA"), inp("B", const(-24.95), "Intercept of the calibration function", "Kalibrasyon fonksiyonunun kesişimi", "L/s"),
                inp("i", normal(6.0, U=0.021), "Measured current (U = 0.021 mA, k = 2)", "Ölçülen akım (U = 0,021 mA, k = 2)", "mA"),
                inp("dq_cal", normal(0.0, U=0.26), "Flowmeter calibration (U = 0.26 L/s, k = 2)", "Debimetre kalibrasyonu (U = 0,26 L/s, k = 2)", "L/s"),
                inp("dq_d", rect(0.23), "Calibration drift", "Kalibrasyon sürüklenmesi", "L/s"), inp("dq_T", rect(0.11), "Environmental effects", "Ortam etkileri", "L/s"),
                inp("p_m", rect(0.5, 1018.0), "Pressure indication (1 hPa resolution)", "Basınç göstergesi (1 hPa çözünürlük)", "hPa"),
                inp("dp_m", normal(0.0, U=1.8), "Gauge calibration (U = 1.8 hPa, k = 2)", "Manometre kalibrasyonu (U = 1,8 hPa, k = 2)", "hPa"),
                inp("dp_f", rect(0.056), "Uncorrected calibration error", "Düzeltilmemiş kalibrasyon hatası", "hPa"), inp("dp_d", rect(1.25), "Gauge drift", "Manometre sürüklenmesi", "hPa"),
                inp("dp_T", rect(1.0), "Environmental effects on the gauge", "Manometreye ortam etkileri", "hPa"),
                inp("t_m", rect(0.05, 295.1), "Corrected temperature indication (0.1 K resolution)", "Düzeltilmiş sıcaklık göstergesi (0,1 K çözünürlük)", "K"),
                inp("dt_m", normal(0.0, U=0.055), "Thermometer calibration (U = 0.055 K, k = 2)", "Termometre kalibrasyonu (U = 0,055 K, k = 2)", "K"),
                inp("dt_f", rect(0.02), "Residual error after the mean correction", "Ortalama düzeltmeden sonra kalan hata", "K"), inp("dt_d", rect(0.13), "Thermometer drift", "Termometre sürüklenmesi", "K"),
                inp("dt_T", rect(0.2), "Installation effects", "Montaj etkileri", "K"),
                inp("P_s", const(1013.25), "Standard pressure", "Standart basınç", "hPa"), inp("T_s", const(293.15), "Standard temperature", "Standart sıcaklık", "K"),
                inp("dQ_s", pooled(0.0, 0.18, 24, 1), "Repeatability of Q_s (s = 0.18 L/s, ν = 24, one reading)", "Q_s'nin tekrarlanabilirliği (s = 0,18 L/s, ν = 24, tek okuma)", "L/s")]},
    {"estimate": chk(12.53, 0.005), "standard_uncertainty": chk(0.275, 5e-4), "effective_dof": chk(130, 1), "expanded_uncertainty": chk(0.55, 0.005)},
    T("UKAS M3003 ed. 6 K.9.10–K.9.12: u_c = 0.275 L/s, ν_eff = 130, U = 0.55 L/s; (12.53 ± 0.55) L/s.", "UKAS M3003 6. baskı K.9.10–K.9.12: u_c = 0,275 L/s, ν_eff = 130, U = 0,55 L/s; (12,53 ± 0,55) L/s."),
    notes=[T("With ν_eff = 130 the coverage factor for 95.45 % is 2.03 (Appendix B table: 2.025 at 100), which would give U = 0.56 L/s; the example uses k = 2.", "ν_eff = 130 ile %95,45 için kapsam faktörü 2,03'tür (Ek B tablosu: 100'de 2,025); bu U = 0,56 L/s verirdi; örnek k = 2 kullanır.")])

# ------------------------------------------------------------------ K.10
project("length", "k10-sheet-thickness",
    T("Thickness of an aluminium test sample (UKAS M3003 K.10)", "Alüminyum deney numunesinin kalınlığı (UKAS M3003 K.10)"),
    T("Mean of five thickness readings at the centre of a sample, measured in an uncontrolled environment; the calibration correction is not applied.", "Bir numunenin merkezinde beş kalınlık okumasının ortalaması, kontrolsüz bir ortamda; kalibrasyon düzeltmesi uygulanmaz."),
    [T("w = w_ave + Δw_cal + δw_T (K.10.1): readings 1.51, 1.51, 1.52, 1.50, 1.53 mm give 1.514 mm, s = 0.0114 mm, u = 0.0051 mm (ν = 4).", "w = w_ave + Δw_cal + δw_T (K.10.1): 1,51, 1,51, 1,52, 1,50, 1,53 mm okumaları 1,514 mm, s = 0,0114 mm, u = 0,0051 mm (ν = 4) verir."),
     T("Resolution ±0.005 mm; calibration U = 0.015 mm (k = 2); drift limit max(0.002, mean(0.002, 0.004, 0.001)) = 0.002 33 mm; uncorrected error ±0.001 mm; temperature ±10 K with 11.9·10⁻⁶ K⁻¹ × 3 mm = 3.57·10⁻⁵ mm/K (K.10.7).",
       "Çözünürlük ±0,005 mm; kalibrasyon U = 0,015 mm (k = 2); sürüklenme sınırı max(0,002, ort(0,002, 0,004, 0,001)) = 0,002 33 mm; düzeltilmemiş hata ±0,001 mm; sıcaklık ±10 K, 11,9·10⁻⁶ K⁻¹ × 3 mm = 3,57·10⁻⁵ mm/K (K.10.7)."),
     T("u_c = 0.009 63 mm, ν_eff = 50; w = 1.514 mm ± 0.019 mm (k = 2).", "u_c = 0,009 63 mm, ν_eff = 50; w = 1,514 mm ± 0,019 mm (k = 2).")],
    "Appendix K.10", ["length", "thickness", "testing", "drift"],
    {"output": {"name": "w", "unit": "mm", "expression": "w_ave + d_res + d_cal + d_drift + d_unc + c_T*d_T", "description": T("Thickness at the centre of the sample", "Numunenin merkezindeki kalınlık")},
     "inputs": [inp("w_ave", typea([1.51, 1.51, 1.52, 1.50, 1.53]), "Indicated thickness, five readings", "Gösterilen kalınlık, beş okuma", "mm"),
                inp("d_res", rect(0.005), "Resolution", "Çözünürlük", "mm"), inp("d_cal", normal(0.0, U=0.015), "Calibration (U = 0.015 mm, k = 2)", "Kalibrasyon (U = 0,015 mm, k = 2)", "mm"),
                inp("d_drift", rect(0.00233), "Calibration drift", "Kalibrasyon sürüklenmesi", "mm"), inp("d_unc", rect(0.001), "Uncorrected calibration error", "Düzeltilmemiş kalibrasyon hatası", "mm"),
                inp("c_T", const(3.57e-5), "Temperature sensitivity", "Sıcaklık duyarlılığı", "mm/K"), inp("d_T", rect(10.0), "Deviation from the calibration temperature", "Kalibrasyon sıcaklığından sapma", "K")]},
    {"estimate": chk(1.514, 5e-7), "standard_uncertainty": chk(0.00963, 5e-6), "effective_dof": chk(50, 1), "expanded_uncertainty": chk(0.019, 5e-4)},
    T("UKAS M3003 ed. 6 K.10.8–K.10.9: u_c = 0.009 63 mm, ν_eff = 50, U = 0.019 mm; 1.514 mm ± 0.019 mm.", "UKAS M3003 6. baskı K.10.8–K.10.9: u_c = 0,009 63 mm, ν_eff = 50, U = 0,019 mm; 1,514 mm ± 0,019 mm."),
    notes=[T("With ν_eff = 50 the coverage factor for 95.45 % is 2.05 (Appendix B), which would give U = 0.020 mm; the example uses k = 2.", "ν_eff = 50 ile %95,45 için kapsam faktörü 2,05'tir (Ek B); bu U = 0,020 mm verirdi; örnek k = 2 kullanır.")])

# ------------------------------------------------------------------ K.11
project("temperature", "k11-probe-calibration",
    T("Calibration of a PT100 probe in a dry block against a reference probe (UKAS M3003 K.11, first budget)", "Kuru blokta referans proba karşı PT100 prob kalibrasyonu (UKAS M3003 K.11, birinci bütçe)"),
    T("Error of the laboratory probe L at 20.4 °C against the reference probe S, whose calibration function is ΔT_S = 0.000 01·I_S + 0.0108 °C.", "Laboratuvar probu L'nin 20,4 °C'de, kalibrasyon fonksiyonu ΔT_S = 0,000 01·I_S + 0,0108 °C olan referans prob S'ye karşı hatası."),
    [T("ΔT_L = I_L − (I_S − ΔT_S − ΔT_SL) − δΔT_L (K.11.2): resolution 0.1 °C (L) and 0.01 °C (S), calibration of S U = 0.024 °C (k = 2), fit ±0.005 °C, drift ±0.008 °C, block gradients ±0.048 °C, repeatability 0.001 23 °C (ν = 24).",
       "ΔT_L = I_L − (I_S − ΔT_S − ΔT_SL) − δΔT_L (K.11.2): çözünürlük 0,1 °C (L) ve 0,01 °C (S), S'nin kalibrasyonu U = 0,024 °C (k = 2), uydurma ±0,005 °C, sürüklenme ±0,008 °C, blok gradyanları ±0,048 °C, tekrarlanabilirlik 0,001 23 °C (ν = 24)."),
     T("I_S = 19.979 °C is derived so that ΔT_L = 0.432 °C at I_L = 20.4 °C, as in the table of K.11.10 (the indication of S is not printed).", "I_S = 19,979 °C, K.11.10 tablosundaki gibi I_L = 20,4 °C'de ΔT_L = 0,432 °C olacak biçimde türetilmiştir (S'nin göstergesi basılmamıştır)."),
     T("u_c = 0.042 25 °C, U = 0.084 °C (k = 2); this U enters the second budget as the calibration uncertainty of L.", "u_c = 0,042 25 °C, U = 0,084 °C (k = 2); bu U ikinci bütçeye L'nin kalibrasyon belirsizliği olarak girer.")],
    "Appendix K.11 (K.11.9)", ["temperature", "pt100", "dry-block"],
    {"output": {"name": "dT_L", "unit": "°C", "expression": "I_L - (I_S - (0.00001*I_S + 0.0108 + dT_cal + dT_fit + dT_drift) - dT_SL) - d_rep", "description": T("Error of probe L", "L probunun hatası")},
     "inputs": [inp("I_L", rect(0.05, 20.4), "Indication of probe L (0.1 °C resolution)", "L probunun göstergesi (0,1 °C çözünürlük)", "°C"),
                inp("I_S", rect(0.005, 19.979), "Indication of reference probe S (0.01 °C resolution; derived value)", "Referans prob S'nin göstergesi (0,01 °C çözünürlük; türetilmiş değer)", "°C"),
                inp("dT_cal", normal(0.0, U=0.024), "Calibration of S (U = 0.024 °C, k = 2)", "S'nin kalibrasyonu (U = 0,024 °C, k = 2)", "°C"),
                inp("dT_fit", rect(0.005), "Linear fit of the calibration data of S", "S'nin kalibrasyon verilerine doğrusal uydurma", "°C"),
                inp("dT_drift", rect(0.008), "Drift of S", "S'nin sürüklenmesi", "°C"),
                inp("dT_SL", rect(0.048), "Gradients and installation in the block", "Blokta gradyanlar ve montaj", "°C"),
                inp("d_rep", pooled(0.0, 0.00123, 24, 1), "Repeatability (s = 0.001 23 °C, ν = 24)", "Tekrarlanabilirlik (s = 0,001 23 °C, ν = 24)", "°C")]},
    {"standard_uncertainty": chk(0.04225, 5e-6), "expanded_uncertainty": chk(0.08449, 1e-5)},
    T("UKAS M3003 ed. 6 K.11.9: u_c = 0.042 25 °C, U = 0.084 49 °C.", "UKAS M3003 6. baskı K.11.9: u_c = 0,042 25 °C, U = 0,084 49 °C."))
project("temperature", "k11-gas-temperature",
    T("Temperature of gas in a container from a calibrated PT100 probe (UKAS M3003 K.11, second budget)", "Kalibre edilmiş PT100 probdan kaptaki gazın sıcaklığı (UKAS M3003 K.11, ikinci bütçe)"),
    T("The probe L is attached to the outside of the vessel; its correction comes from the calibration function ΔT_u = 0.000 55·I_u + 0.419 °C.", "L probu kabın dışına bağlanır; düzeltmesi ΔT_u = 0,000 55·I_u + 0,419 °C kalibrasyon fonksiyonundan gelir."),
    [T("T_g = (I_u − ΔT_u) − ΔT_ug − δT_h − δT_r (K.11.11): I_u = 23.4 °C (mean of four), ΔT_u = 0.431 °C.", "T_g = (I_u − ΔT_u) − ΔT_ug − δT_h − δT_r (K.11.11): I_u = 23,4 °C (dördün ortalaması), ΔT_u = 0,431 °C."),
     T("Calibration of L U = 0.084 49 °C (k = 2, first budget), fit ±0.012 °C, drift ±0.004 67 °C, probe–gas difference ±0.1 °C, self-heating ±4 mK, repeatability s = 0.076 °C (ν = 49) for the mean of four.",
       "L'nin kalibrasyonu U = 0,084 49 °C (k = 2, birinci bütçe), uydurma ±0,012 °C, sürüklenme ±0,004 67 °C, prob–gaz farkı ±0,1 °C, kendi kendine ısınma ±4 mK, dört okumanın ortalaması için tekrarlanabilirlik s = 0,076 °C (ν = 49)."),
     T("u_c = 0.0864 °C; T_g = 22.97 °C ± 0.17 °C (k = 2).", "u_c = 0,0864 °C; T_g = 22,97 °C ± 0,17 °C (k = 2).")],
    "Appendix K.11 (K.11.17–K.11.18)", ["temperature", "pt100", "gas"],
    {"output": {"name": "T_g", "unit": "°C", "expression": "I_u - (0.00055*I_u + 0.419 + dT_cal + dT_fit + dT_drift) - dT_ug - dT_h - dT_r", "description": T("Temperature of the gas", "Gazın sıcaklığı")},
     "inputs": [inp("I_u", rect(0.05, 23.4), "Indication of probe L (0.1 °C resolution)", "L probunun göstergesi (0,1 °C çözünürlük)", "°C"),
                inp("dT_cal", normal(0.0, U=0.08449), "Calibration of L (U = 0.084 49 °C, k = 2)", "L'nin kalibrasyonu (U = 0,084 49 °C, k = 2)", "°C"),
                inp("dT_fit", rect(0.012), "Linear fit of the calibration data of L", "L'nin kalibrasyon verilerine doğrusal uydurma", "°C"),
                inp("dT_drift", rect(0.00467), "Drift of L", "L'nin sürüklenmesi", "°C"),
                inp("dT_ug", rect(0.1), "Difference between probe and gas", "Prob ile gaz arasındaki fark", "°C"),
                inp("dT_h", rect(0.004), "Self-heating of L on the vessel", "L'nin kap üzerinde kendi kendine ısınması", "°C"),
                inp("dT_r", pooled(0.0, 0.076, 49, 4), "Repeatability of the vessel temperature (s = 0.076 °C, ν = 49, mean of four)", "Kap sıcaklığının tekrarlanabilirliği (s = 0,076 °C, ν = 49, dördün ortalaması)", "°C")]},
    {"estimate": chk(22.97, 0.005), "standard_uncertainty": chk(0.08638, 5e-5), "expanded_uncertainty": chk(0.17276, 1e-4)},
    T("UKAS M3003 ed. 6 K.11.17–K.11.18: u_c = 0.086 38 °C, U = 0.172 76 °C; 22.97 °C ± 0.17 °C.", "UKAS M3003 6. baskı K.11.17–K.11.18: u_c = 0,086 38 °C, U = 0,172 76 °C; 22,97 °C ± 0,17 °C."))

# ------------------------------------------------------------------ Appendix C
project("electrical", "c5-dvm-dominant-rectangular",
    T("Digital voltmeter with a dominant resolution term (UKAS M3003 Appendix C)", "Baskın çözünürlük terimli dijital voltmetre (UKAS M3003 Ek C)"),
    T("A DVM reading 1.001 V for an applied 1.000 00 V; the rounding of the 1 mV digit dominates, so k for 95.45 % comes from the convolution of a rectangular and a normal distribution.",
      "Uygulanan 1,000 00 V için 1,001 V okuyan bir DVM; 1 mV basamağının yuvarlanması baskındır; bu nedenle %95,45 için k, dikdörtgen ve normal dağılımın konvolüsyonundan gelir."),
    [T("ΔV = V_ind − V_ref + δI_T (C.10): reference U = 0.19 mV (k = 2), drift and temperature ±0.05 mV, rounding ±0.5 mV.", "ΔV = V_ind − V_ref + δI_T (C.10): referans U = 0,19 mV (k = 2), sürüklenme ve sıcaklık ±0,05 mV, yuvarlama ±0,5 mV."),
     T("u_R = 0.289 mV, u_N = 0.099 mV, u_N/u_R = 0.344 < 0.70: the rectangular term dominates (C.4, C.5).", "u_R = 0,289 mV, u_N = 0,099 mV, u_N/u_R = 0,344 < 0,70: dikdörtgen terim baskındır (C.4, C.5)."),
     T("The program solves P(|R + N| ≤ k·u_c) = 0.9545 for the convolution, which reproduces the table of C.7: k = 1.77, U = 1.77 × 0.305 mV = 0.54 mV.", "Program konvolüsyon için P(|R + N| ≤ k·u_c) = 0,9545 eşitliğini çözer; bu C.7 tablosunu yeniden üretir: k = 1,77, U = 1,77 × 0,305 mV = 0,54 mV.")],
    "Appendix C.5–C.10", ["voltage", "dvm", "dominant-rectangular", "coverage-factor"],
    {"output": {"name": "dV", "unit": "mV", "expression": "V_ind - V_ref + d_IT", "description": T("Error of the voltmeter at 1 V", "Voltmetrenin 1 V'taki hatası")},
     "inputs": [inp("V_ind", rect(0.5, 1001.0), "Indication (1 mV digit)", "Gösterge (1 mV basamak)", "mV"),
                inp("V_ref", normal(1000.0, U=0.19), "Applied reference voltage (U = 0.19 mV, k = 2)", "Uygulanan referans gerilim (U = 0,19 mV, k = 2)", "mV"),
                inp("d_IT", rect(0.05), "Drift and temperature effects", "Sürüklenme ve sıcaklık etkileri", "mV")]},
    {"estimate": chk(1.0, 1e-9), "standard_uncertainty": chk(0.305, 5e-4), "coverage_factor": chk(1.77, 0.005), "expanded_uncertainty": chk(0.54, 0.005)},
    T("UKAS M3003 ed. 6 C.8–C.10: u_c = 0.305 mV, k = 1.77, U = 0.54 mV.", "UKAS M3003 6. baskı C.8–C.10: u_c = 0,305 mV, k = 1,77, U = 0,54 mV."),
    notes=[T("Research log F61: C.5 states the drift and temperature limit as ±50 V; the calculation uses 0.05 mV, i.e. ±50 µV.", "Araştırma günlüğü F61: C.5 sürüklenme ve sıcaklık sınırını ±50 V olarak yazar; hesap 0,05 mV, yani ±50 µV kullanır.")],
    analysis={"gum": {"coverage_probability": 0.9545, "dominant_rectangular": True}, "mcm": {"trials": 200000, "seed": 1}})

# ------------------------------------------------------------------ Appendix L
def dmm(volts, extra_expl, expected, ref, analysis=None, name=None, title=None, notes=None):
    I = volts * 1e6
    project("electrical", name, title,
        T(f"Error of a 6½-digit DMM on its 1 V range at {volts} V, calibrated with a multifunction calibrator; relative (ppm) and absolute (µV) contributions combined at one reading.",
          f"Çok fonksiyonlu kalibratörle kalibre edilen 6½ basamaklı bir DMM'nin 1 V kademesinde {str(volts).replace('.', ',')} V'taki hatası; göreli (ppm) ve mutlak (µV) katkılar bir okumada birleştirilir."),
        [T("ΔI_DMM = I_DMM − V_REF + δI_RES + δΔ_r with V_REF = V_CAL + δV_D + δV_UE + δV_TC + δV_LIN + δV_T + δV_CM (L.3.4); no readings are needed for the uncertainty, so the estimate is zero.",
           "ΔI_DMM = I_DMM − V_REF + δI_RES + δΔ_r, V_REF = V_CAL + δV_D + δV_UE + δV_TC + δV_LIN + δV_T + δV_CM (L.3.4); belirsizlik için okuma gerekmez; tahmin sıfırdır."),
         T("Relative terms (× I_DMM): calibration 2.8 ppm (k = 2), specification ±8 ppm, repeatability 2.5 ppm (ν = 9). Absolute terms: calibration 0.5 µV (k = 2), specification ±2 µV, thermoelectric ±1 µV, common mode ±1 µV, resolution ±0.5 µV.",
           "Göreli terimler (× I_DMM): kalibrasyon 2,8 ppm (k = 2), şartname ±8 ppm, tekrarlanabilirlik 2,5 ppm (ν = 9). Mutlak terimler: kalibrasyon 0,5 µV (k = 2), şartname ±2 µV, termoelektrik ±1 µV, ortak mod ±1 µV, çözünürlük ±0,5 µV.")] + extra_expl,
        "Appendix L.3", ["voltage", "dmm", "range-of-values", "relative"],
        {"output": {"name": "dI", "unit": "µV", "expression": "I_DMM*1e-6*(r_cal + r_spec + r_rep) + a_cal + a_spec + a_T + a_CM + a_res", "description": T("Error of the DMM indication", "DMM göstergesinin hatası")},
         "inputs": [inp("I_DMM", const(I), "Indication", "Gösterge", "µV"),
                    inp("r_cal", normal(0.0, U=2.8), "Calibration of the calibrator, relative (U = 2.8 ppm, k = 2)", "Kalibratörün kalibrasyonu, göreli (U = 2,8 ppm, k = 2)", "ppm"),
                    inp("r_spec", rect(8.0), "Specification of the calibrator, relative (±8 ppm)", "Kalibratör şartnamesi, göreli (±8 ppm)", "ppm"),
                    inp("r_rep", pooled(0.0, 2.5, 9, 1), "Repeatability (2.5 ppm, ν = 9, one reading)", "Tekrarlanabilirlik (2,5 ppm, ν = 9, tek okuma)", "ppm"),
                    inp("a_cal", normal(0.0, U=0.5), "Calibration of the calibrator, absolute (U = 0.5 µV, k = 2)", "Kalibratörün kalibrasyonu, mutlak (U = 0,5 µV, k = 2)", "µV"),
                    inp("a_spec", rect(2.0), "Specification of the calibrator, absolute (±2 µV)", "Kalibratör şartnamesi, mutlak (±2 µV)", "µV"),
                    inp("a_T", rect(1.0), "Thermoelectric voltages", "Termoelektrik gerilimler", "µV"), inp("a_CM", rect(1.0), "Common-mode effects", "Ortak mod etkileri", "µV"),
                    inp("a_res", rect(0.5), "Resolution of the DMM", "DMM çözünürlüğü", "µV")]},
        expected, ref, notes=notes, analysis=analysis)
dmm(0.95, [T("At I_DMM = 0.95 V: u_c = 5.37 µV, U = 10.73 µV (k = 2).", "I_DMM = 0,95 V'ta: u_c = 5,37 µV, U = 10,73 µV (k = 2).")],
    {"estimate": chk(0.0, 1e-12), "standard_uncertainty": chk(5.37, 0.005), "expanded_uncertainty": chk(10.73, 0.01)},
    T("UKAS M3003 ed. 6 L.3.19 (budget at 0.95 V): u_c = 5.37 µV, U = 10.73 µV.", "UKAS M3003 6. baskı L.3.19 (0,95 V'taki bütçe): u_c = 5,37 µV, U = 10,73 µV."),
    name="l3-dmm-1v-range-095v", title=T("DMM on the 1 V range: budget at 0.95 V (UKAS M3003 L.3)", "1 V kademesinde DMM: 0,95 V'taki bütçe (UKAS M3003 L.3)"),
    notes=[T("Research log F61: the budget is headed 'evaluated at ΔI_DMM = 0.95 V', meaning I_DMM = 0.95 V; L.3.15 defines Q[a, b] = [a² + ab]^½ for [a² + b²]^½; Appendix Q's note writes u(R) = u(R) × R for u(R) = w(R) × R.",
             "Araştırma günlüğü F61: bütçe başlığı 'ΔI_DMM = 0,95 V'ta değerlendirilmiş' der, kastedilen I_DMM = 0,95 V'tur; L.3.15, [a² + b²]^½ yerine Q[a, b] = [a² + ab]^½ tanımlar; Ek Q'nun notu u(R) = w(R) × R yerine u(R) = u(R) × R yazar.")])
dmm(0.5, [T("At I_DMM = 500 mV: U = √((500 mV × 11 ppm)² + (2.9 µV)²) = 6.2 µV (L.3.14).", "I_DMM = 500 mV'ta: U = √((500 mV × 11 ppm)² + (2,9 µV)²) = 6,2 µV (L.3.14).")],
    {"estimate": chk(0.0, 1e-12), "expanded_uncertainty": chk(6.2, 0.05)},
    T("UKAS M3003 ed. 6 L.3.14: U = 6.2 µV at 500 mV.", "UKAS M3003 6. baskı L.3.14: 500 mV'ta U = 6,2 µV."),
    name="l3-dmm-1v-range-500mv", title=T("DMM on the 1 V range: uncertainty at 500 mV from the range formula (UKAS M3003 L.3)", "1 V kademesinde DMM: kademe formülünden 500 mV'taki belirsizlik (UKAS M3003 L.3)"))
dmm(1.0, [T("At |I_DMM| = 1 V the specification term is marginally dominant: u_R/u_N = 1.435; the convolution gives k = 1.898 for 95.45 % (L.3.20), not significantly different from the k = 1.900 of the critical ratio 1.42.",
            "|I_DMM| = 1 V'ta şartname terimi sınırda baskındır: u_R/u_N = 1,435; konvolüsyon %95,45 için k = 1,898 verir (L.3.20); bu, 1,42 kritik oranına karşılık gelen k = 1,900'den anlamlı olarak farklı değildir.")],
    {"estimate": chk(0.0, 1e-12), "coverage_factor": chk(1.898, 5e-4)},
    T("UKAS M3003 ed. 6 L.3.20: k = 1.898 for u_R/u_N = 1.435.", "UKAS M3003 6. baskı L.3.20: u_R/u_N = 1,435 için k = 1,898."),
    analysis={"gum": {"coverage_probability": 0.9545, "dominant_rectangular": True}, "mcm": {"trials": 200000, "seed": 1}},
    name="l3-dmm-1v-range-1v-dominant", title=T("DMM on the 1 V range at full scale: coverage factor for a dominant specification term (UKAS M3003 L.3.20)", "Tam ölçekte 1 V kademesinde DMM: baskın şartname terimi için kapsam faktörü (UKAS M3003 L.3.20)"))
print("written")
