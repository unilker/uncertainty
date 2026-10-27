"""Writes the EURAMET cg-19 v4.1 library projects (run from the repository root)."""
import json
from pathlib import Path

DOC = "EURAMET Calibration Guide No. 19 v4.1 (2025), Guidelines on the Determination of Uncertainty in Gravimetric Volume Calibration"
URL = "https://www.euramet.org/publications-media-centre/calibration-guidelines"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def inp(name, dist, en, tr, unit=None):
    d = {"name": name, "description": T(en, tr), "distribution": dist}
    if unit: d["unit"] = unit
    return d

TANAKA = "0.99997495*(1 - (t_W - 3.983035)^2*(t_W + 301.797)/(522528.9*(t_W + 69.34881))) + d_form + d_pur"
MODEL = "m/(rho_W - rho_A)*(1 - rho_A/rho_B)*(1 - gamma*(t_W + d_ts - 20)) + dV_men + dV_rep"
COMMON = [
    T("V₂₀ = m/(ρ_W − ρ_A)·(1 − ρ_A/ρ_B)·[1 − γ(t − 20 °C)] + δV_men + δV_rep (equation 5), with the instrument temperature t = t_W + δt_s.",
      "V₂₀ = m/(ρ_W − ρ_A)·(1 − ρ_A/ρ_B)·[1 − γ(t − 20 °C)] + δV_men + δV_rep (eşitlik 5); alet sıcaklığı t = t_W + δt_s."),
    T("Water density from Tanaka's formula at t_W, ρ_W = a₅[1 − (t + a₁)²(t + a₂)/(a₃(t + a₄))], with its own uncertainty 4.5·10⁻⁷ g/mL and 5 ppm (rectangular) for the purity of the water (6.3.3). The temperature of the water thus enters both the density and the expansion of the instrument, with their correlation.",
      "Su yoğunluğu t_W'de Tanaka formülünden, ρ_W = a₅[1 − (t + a₁)²(t + a₂)/(a₃(t + a₄))]; formülün kendi belirsizliği 4,5·10⁻⁷ g/mL ve suyun saflığı için 5 ppm (dikdörtgen) (6.3.3). Suyun sıcaklığı böylece hem yoğunluğa hem de aletin genleşmesine, korelasyonlarıyla birlikte girer."),
    T("Thermometer U = 0.009 °C (k = 2) and resolution 0.001 °C; difference between water and instrument from |t_A − t_W|/(8√3) (6.3.2); air density, density of the weights and expansion coefficient as in the example.",
      "Termometre U = 0,009 °C (k = 2) ve çözünürlük 0,001 °C; su ile alet arasındaki fark |t_A − t_W|/(8√3) ile (6.3.2); hava yoğunluğu, ağırlıkların yoğunluğu ve genleşme katsayısı örnekteki gibi."),
]

def project(name, title, desc, expl, sec, tags, values, expected, ref, notes):
    pid = f"cg19-{name}"
    m, um, tw, dts_hw, rho_a_u, men_hw, s_rep, n_rep = values
    inputs = [
        inp("m", {"type": "normal", "mean": m, "sd": um}, "Mass of water, I_L − I_E (balance U and resolution for both weighings)", "Su kütlesi, I_L − I_E (her iki tartım için terazi U ve çözünürlüğü)", "g"),
        inp("t_W0", {"type": "normal", "mean": tw, "expanded": 0.009, "k": 2}, "Measured water temperature (thermometer U = 0.009 °C, k = 2)", "Ölçülen su sıcaklığı (termometre U = 0,009 °C, k = 2)", "°C"),
        inp("d_res", {"type": "rectangular", "value": 0.0, "half_width": 0.0005}, "Resolution of the thermometer (0.001 °C)", "Termometrenin çözünürlüğü (0,001 °C)", "°C"),
        inp("d_ts", {"type": "rectangular", "value": 0.0, "half_width": dts_hw}, "Difference between water and instrument temperature", "Su ile alet sıcaklığı arasındaki fark", "°C"),
        inp("d_form", {"type": "normal", "mean": 0.0, "sd": 4.5e-7}, "Uncertainty of Tanaka's formula", "Tanaka formülünün belirsizliği", "g/mL"),
        inp("d_pur", {"type": "rectangular", "value": 0.0, "half_width": 4.992e-6}, "Purity of the water (5 ppm)", "Suyun saflığı (5 ppm)", "g/mL"),
        inp("rho_A", {"type": "normal", "mean": 0.0012, "sd": rho_a_u}, "Air density (u as printed in the example)", "Hava yoğunluğu (u örnekte basıldığı gibi)", "g/mL"),
        inp("rho_B", {"type": "rectangular", "value": 8.0, "half_width": 0.06}, "Density of the reference weights (±0.06 g/mL, u = 0.0346 g/mL as in the summary table)", "Referans ağırlıkların yoğunluğu (±0,06 g/mL, özet tablodaki gibi u = 0,0346 g/mL)", "g/mL"),
        inp("gamma", {"type": "rectangular", "value": 1e-5, "half_width": 5e-7}, "Cubic expansion coefficient of the glass (±5 %)", "Camın hacimsel genleşme katsayısı (±%5)", "1/°C"),
        inp("dV_men", {"type": "rectangular", "value": 0.0, "half_width": men_hw} if men_hw else {"type": "constant", "value": 0.0}, "Meniscus reading" if men_hw else "Meniscus reading (negligible)", "Menisküs okuma" if men_hw else "Menisküs okuma (ihmal edilebilir)", "mL"),
        inp("dV_rep", {"type": "type_a_pooled", "mean": 0.0, "pooled_sd": s_rep, "pooled_dof": n_rep - 1, "n": n_rep}, f"Repeatability (s(V₀) = {s_rep} mL, mean of {n_rep})", f"Tekrarlanabilirlik (s(V₀) = {str(s_rep).replace('.', ',')} mL, {n_rep} ölçümün ortalaması)", "mL"),
    ]
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": COMMON + expl, "field": "flow",
                      "sources": [{"document": DOC, "section": sec, "url": URL}],
                      "tags": ["euramet-cg19", "volume", "gravimetric", "calibration", "validation"] + tags, "notes": notes},
         "model": {"output": {"name": "V_20", "unit": "mL", "expression": MODEL, "description": T("Volume at 20 °C", "20 °C'deki hacim")},
                   "intermediates": [{"name": "t_W", "unit": "°C", "expression": "t_W0 + d_res", "description": T("Water temperature", "Su sıcaklığı")},
                                     {"name": "rho_W", "unit": "g/mL", "expression": TANAKA, "description": T("Water density (Tanaka 2001)", "Su yoğunluğu (Tanaka 2001)")}],
                   "inputs": inputs},
         "analysis": {"gum": {"coverage_probability": 0.9545}, "mcm": {"trials": 200000, "seed": 1}},
         "expected": {"gum": expected, "reference": ref}}
    Path(f"library/flow/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

COV = T("The example states k for 'approximately 95 %', but k = t(ν_eff) for 95.45 %, as in EA-4/02; the project uses p = 95.45 %.",
        "Örnek k'yı 'yaklaşık %95' için verir, ancak k, EA-4/02'deki gibi %95,45 için t(ν_eff)'dir; proje p = %95,45 kullanır.")

project("ex1-flask-1000ml",
    T("Gravimetric calibration of a 1000 mL volumetric flask (EURAMET cg-19 Example 1)", "1000 mL'lik ölçülü balonun gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 1)"),
    T("A 1000 mL glass flask filled to the mark and weighed ten times; water at 19.11 °C, air 0.2 °C warmer.", "İşaretine kadar doldurulup on kez tartılan 1000 mL'lik cam balon; su 19,11 °C, hava 0,2 °C daha sıcak."),
    [T("m = 996.9505 g with u(m) = 0.0013 g from U(balance) = 0.0018 g (k = 2) and the 0.1 mg resolution for both weighings; meniscus ±0.015 mL (rectangular, optical aids); s(V₀) = 0.044 mL for n = 10.",
       "m = 996,9505 g, u(m) = 0,0013 g; her iki tartım için U(terazi) = 0,0018 g (k = 2) ve 0,1 mg çözünürlükten; menisküs ±0,015 mL (dikdörtgen, optik yardımcılar); n = 10 için s(V₀) = 0,044 mL."),
     T("Repeatability (0.014 mL) and meniscus (0.0087 mL) dominate: u(V₂₀) = 0.017 mL, ν_eff = 19, k = 2.14; V₂₀ = 999.628 mL ± 0.036 mL.", "Tekrarlanabilirlik (0,014 mL) ve menisküs (0,0087 mL) baskındır: u(V₂₀) = 0,017 mL, ν_eff = 19, k = 2,14; V₂₀ = 999,628 mL ± 0,036 mL."), COV],
    "7.1 (Tables 1 and 2)", ["flask", "glassware"],
    (996.9505, 0.00127, 19.11, 0.025, 1.76e-6, 0.015, 0.044, 10),
    {"estimate": chk(999.628, 0.01), "standard_uncertainty": chk(0.017, 0.0005), "effective_dof": chk(19, 1), "coverage_factor": chk(2.15, 0.015), "expanded_uncertainty": chk(0.036, 0.001)},
    T("EURAMET cg-19 v4.1 7.1, Table 2: V₂₀ = 999.628 mL, u = 0.017 mL, ν_eff = 19, k = 2.15, U = 0.036 mL. The estimate is compared within 0.01 mL because Table 1 gives the densities rounded (0.9984 and 0.0012 g/mL).",
      "EURAMET cg-19 v4.1 7.1, Tablo 2: V₂₀ = 999,628 mL, u = 0,017 mL, ν_eff = 19, k = 2,15, U = 0,036 mL. Tablo 1 yoğunlukları yuvarlanmış verdiğinden (0,9984 ve 0,0012 g/mL) tahmin 0,01 mL içinde karşılaştırılır."),
    [T("Research log F62: with the inputs stated in 7.1.1.4 (0.16 °C, 6 Pa — 60 in the formula —, 0.6 %rh and the 2.4·10⁻⁴ formula uncertainty), the air-density formula gives u(ρ_A) = 6.0·10⁻⁷ g/mL, not 1.76·10⁻⁶ g/mL; 7.1.1.5 gives u(ρ_B) = 0.06/2 = 0.03 g/mL while Table 2 uses 0.0346 g/mL (= 0.06/√3); 7.1.1.3 writes u(ρ_W,t) as 1.024·10⁻⁶ g/mL in the text and 1.24·10⁻⁶ g/mL in the formula, and β·ρ_W·u(t_W) = 8.9·10⁻⁷ g/mL; Table 2 gives ν = 50 for the mass without a stated basis. None changes u(V₂₀) at its printed digits.",
       "Araştırma günlüğü F62: 7.1.1.4'te belirtilen girdilerle (0,16 °C, 6 Pa — formülde 60 —, %0,6 bağıl nem ve 2,4·10⁻⁴ formül belirsizliği) hava yoğunluğu formülü 1,76·10⁻⁶ g/mL değil u(ρ_A) = 6,0·10⁻⁷ g/mL verir; 7.1.1.5 u(ρ_B) = 0,06/2 = 0,03 g/mL verirken Tablo 2 0,0346 g/mL (= 0,06/√3) kullanır; 7.1.1.3 u(ρ_W,t)'yi metinde 1,024·10⁻⁶ g/mL, formülde 1,24·10⁻⁶ g/mL yazar; β·ρ_W·u(t_W) = 8,9·10⁻⁷ g/mL'dir; Tablo 2 kütle için gerekçesiz ν = 50 verir. Hiçbiri u(V₂₀)'yi basılı basamaklarda değiştirmez.")])

project("ex2-pycnometer-100ml",
    T("Gravimetric calibration of a 100 mL glass pycnometer (EURAMET cg-19 Example 2)", "100 mL'lik cam piknometrenin gravimetrik kalibrasyonu (EURAMET cg-19 Örnek 2)"),
    T("A 100 mL pycnometer weighed ten times; water at 18.94 °C, air 0.23 °C warmer; the meniscus contribution is negligible.", "On kez tartılan 100 mL'lik piknometre; su 18,94 °C, hava 0,23 °C daha sıcak; menisküs katkısı ihmal edilebilir."),
    [T("m = 99.7500 g with u(m) = 0.000 11 g from U(balance) = 0.000 15 g (k = 2) and the 0.1 mg resolution for both weighings; s(V₀) = 0.0026 mL for n = 10.",
       "m = 99,7500 g, u(m) = 0,000 11 g; her iki tartım için U(terazi) = 0,000 15 g (k = 2) ve 0,1 mg çözünürlükten; n = 10 için s(V₀) = 0,0026 mL."),
     T("Repeatability dominates: u(V₂₀) = 0.89 µL, ν_eff = 13, k = 2.21; V₂₀ = 100.014 mL ± 0.0020 mL.", "Tekrarlanabilirlik baskındır: u(V₂₀) = 0,89 µL, ν_eff = 13, k = 2,21; V₂₀ = 100,014 mL ± 0,0020 mL."), COV],
    "7.2 (Tables 3 and 4)", ["pycnometer", "glassware"],
    (99.75, 0.000114, 18.94, 0.02875, 1.74e-6, None, 0.0026, 10),
    {"estimate": chk(100.014, 0.0005), "standard_uncertainty": chk(0.00089, 0.00002), "effective_dof": chk(13, 1), "coverage_factor": chk(2.21, 0.025), "expanded_uncertainty": chk(0.0020, 0.0001)},
    T("EURAMET cg-19 v4.1 7.2, Table 4: V₂₀ = 100.014 mL, u = 0.89 µL, ν_eff = 13 (7.2.5), k = 2.21, U = 2.0 µL. ν_eff = 12.9 is rounded to 13 in the example; GUM G.6.4 truncates it to 12, which gives k = 2.23 and the same U = 2.0 µL.", "EURAMET cg-19 v4.1 7.2, Tablo 4: V₂₀ = 100,014 mL, u = 0,89 µL, ν_eff = 13 (7.2.5), k = 2,21, U = 2,0 µL. Örnek ν_eff = 12,9 değerini 13'e yuvarlar; GUM G.6.4 onu 12'ye keser, bu da k = 2,23 ve aynı U = 2,0 µL verir."),
    [T("Research log F62: Table 4 gives ν_eff(V₂₀) = 1727, whereas 7.2.5 gives 13, which the Welch–Satterthwaite formula reproduces and which k = 2.21 corresponds to. As in Example 1, u(ρ_A) = 1.74·10⁻⁶ g/mL does not follow from the stated inputs (4.8·10⁻⁷ g/mL).",
       "Araştırma günlüğü F62: Tablo 4 ν_eff(V₂₀) = 1727 verir; 7.2.5 ise Welch–Satterthwaite formülünün yeniden ürettiği ve k = 2,21'in karşılık geldiği 13 değerini verir. Örnek 1'deki gibi u(ρ_A) = 1,74·10⁻⁶ g/mL belirtilen girdilerden çıkmaz (4,8·10⁻⁷ g/mL).")])
print("written")
