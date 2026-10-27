"""Writes the EURAMET cg-18 v4.0 Example H1 library projects (run from the repository root)."""
import json, math, statistics
from pathlib import Path

DOC = "EURAMET Calibration Guide No. 18 v4.0 (2015), Guidelines on the Calibration of Non-Automatic Weighing Instruments"
URL = "https://www.euramet.org/publications-media-centre/calibration-guidelines"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def inp(name, dist, en, tr, unit="g"):
    return {"name": name, "unit": unit, "description": T(en, tr), "distribution": dist}
R3 = math.sqrt(3)

rep_readings = [100.0006, 100.0003, 100.0005, 100.0004, 100.0005]
s_rep = statistics.stdev(rep_readings)
d = 0.0001
ECC_MAX, L_ECC = 0.0002, 100.0
RHO0, RHOC = 1.2, 8000.0

COMMON = [
    T("E = I − m_ref (7.1-1): the indication I with the rounding at zero and at load (d/2√3 each, d = 0.1 mg), repeatability and eccentricity; the reference mass m_ref with the conventional mass of the E2 weights, their drift and air buoyancy (7.1.2-14).",
      "E = I − m_ref (7.1-1): sıfırda ve yükte yuvarlamayla (her biri d/2√3, d = 0,1 mg), tekrarlanabilirlik ve eksantriklikle gösterge I; E2 ağırlıklarının konvansiyonel kütlesi, sürüklenmesi ve hava kaldırma kuvvetiyle referans kütle m_ref (7.1.2-14)."),
    T("Repeatability from five readings of 100 g (100.0006, 100.0003, 100.0005, 100.0004, 100.0005 g): s = 0.000 114 g with ν = 4, applied to a single reading at each load (7.1.1-5).",
      "100 g'ın beş okumasından tekrarlanabilirlik (100,0006, 100,0003, 100,0005, 100,0004, 100,0005 g): s = 0,000 114 g, ν = 4; her yükte tek okumaya uygulanır (7.1.1-5)."),
    T("Eccentricity: ±|ΔI_ecc,max|·L/(2L_ecc) with ΔI_ecc,max = 0.2 mg at L_ecc = 100 g, rectangular (7.1.1-10). Drift: D = k_D·U(m_c) with k_D = 1.25, rectangular (7.1.2-10, -11).",
      "Eksantriklik: ±|ΔI_ecc,maks|·L/(2L_ecc), L_ecc = 100 g'da ΔI_ecc,maks = 0,2 mg, dikdörtgen (7.1.1-10). Sürüklenme: k_D = 1,25 ile D = k_D·U(m_c), dikdörtgen (7.1.2-10, -11)."),
    T("ν_eff by Welch–Satterthwaite (B3-1) and k for 95.45 % from the t-distribution.", "Welch–Satterthwaite ile ν_eff (B3-1) ve %95,45 için t dağılımından k."),
]

def project(name, title, desc, expl, L, I, m_c, u_mc, mpe, buoyancy, expected, ref, notes=None, conv=None):
    pid = f"cg18-{name}"
    inputs = [inp("I_L", {"type": "constant", "value": I}, "Indication at the load", "Yükteki gösterge"),
              inp("dI_dig0", {"type": "rectangular", "value": 0.0, "half_width": d / 2}, "Rounding at zero", "Sıfırda yuvarlama"),
              inp("dI_rep", {"type": "type_a_pooled", "mean": 0.0, "pooled_sd": s_rep, "pooled_dof": 4, "n": 1}, "Repeatability (s from five readings, ν = 4, one reading)", "Tekrarlanabilirlik (beş okumadan s, ν = 4, tek okuma)")]
    expr_I = "I_L + dI_dig0 + dI_rep"
    if L > 0:
        inputs += [inp("dI_digL", {"type": "rectangular", "value": 0.0, "half_width": d / 2}, "Rounding at the load", "Yükte yuvarlama"),
                   inp("dI_ecc", {"type": "rectangular", "value": 0.0, "half_width": ECC_MAX * L / (2 * L_ECC)}, "Eccentricity", "Eksantriklik"),
                   inp("m_c", {"type": "normal", "mean": m_c, "sd": u_mc}, "Conventional mass of the weights (certificate U/k)", "Ağırlıkların konvansiyonel kütlesi (sertifika U/k)"),
                   inp("dm_D", {"type": "rectangular", "value": 0.0, "half_width": 1.25 * 2 * u_mc}, "Drift of the weights (D = 1.25·U)", "Ağırlıkların sürüklenmesi (D = 1,25·U)")]
        expr_I += " + dI_digL + dI_ecc"
        expr_m = "m_c + dm_D"
        if buoyancy == "5d":
            inputs.append(inp("dm_B", {"type": "rectangular", "value": 0.0, "half_width": L * (0.1 * RHO0 / RHOC + mpe / (4 * L))}, "Air buoyancy, not corrected (7.1.2-5d)", "Hava kaldırma kuvveti, düzeltilmemiş (7.1.2-5d)"))
            expr_m += " + dm_B"
        elif buoyancy == "5e":
            u = L * (math.sqrt(1.07e-4 + 1.33e-6 * 25) * RHO0 / RHOC + mpe / (4 * L * R3))
            inputs.append(inp("dm_B", {"type": "normal", "mean": 0.0, "sd": u}, "Air buoyancy, not corrected, for ΔT = 5 K (7.1.2-5e)", "Hava kaldırma kuvveti, düzeltilmemiş, ΔT = 5 K için (7.1.2-5e)"))
            expr_m += " + dm_B"
        else:
            inputs += [inp("m_N", {"type": "constant", "value": L}, "Nominal value of the load", "Yükün nominal değeri"),
                       inp("rho_a", {"type": "normal", "mean": 1.173, "sd": 0.014}, "Air density during calibration (CIPM formula; ΔT = 5 K by A3-2)", "Kalibrasyon sırasında hava yoğunluğu (CIPM formülü; A3-2 ile ΔT = 5 K)", "kg/m³"),
                       inp("rho", {"type": "normal", "mean": 7950.0, "sd": 70.0}, "Density of the weights", "Ağırlıkların yoğunluğu", "kg/m³"),
                       inp("rho_0", {"type": "constant", "value": RHO0}, "Reference air density", "Referans hava yoğunluğu", "kg/m³"),
                       inp("rho_c", {"type": "constant", "value": RHOC}, "Reference density of the weights", "Ağırlıkların referans yoğunluğu", "kg/m³"),
                       inp("dm_conv", {"type": "rectangular", "value": 0.0, "half_width": conv * R3}, "Convection, weights 2 K from room temperature (Table F2.1)", "Konveksiyon, oda sıcaklığından 2 K farklı ağırlıklar (Tablo F2.1)")]
            expr_m += " - m_N*(rho_a - rho_0)*(1/rho - 1/rho_c) + dm_conv"
        expr = f"{expr_I} - ({expr_m})"
    else:
        expr = expr_I
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": COMMON + expl, "field": "mass",
                      "sources": [{"document": DOC, "section": "Appendix H1", "url": URL}],
                      "tags": ["euramet-cg18", "balance", "weighing-instrument", "calibration", "validation"]},
         "model": {"output": {"name": "E", "unit": "g", "expression": expr, "description": T("Error of indication", "Gösterge hatası")}, "inputs": inputs},
         "analysis": {"gum": {"coverage_probability": 0.9545}, "mcm": {"trials": 200000, "seed": 1}},
         "expected": {"gum": expected, "reference": ref}}
    if notes: p["metadata"]["notes"] = notes
    Path(f"library/mass/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

project("h1-220g-zero",
    T("Laboratory balance 220 g / 0.1 mg: error at zero load (EURAMET cg-18 H1)", "Laboratuvar terazisi 220 g / 0,1 mg: sıfır yükte hata (EURAMET cg-18 H1)"),
    T("At zero load only the rounding at zero and the repeatability contribute; the repeatability with ν = 4 sets the coverage factor.", "Sıfır yükte yalnızca sıfırda yuvarlama ve tekrarlanabilirlik katkı verir; ν = 4 ile tekrarlanabilirlik kapsam faktörünü belirler."),
    [T("u(E) = √(0.000 114² + 0.000 029²) g = 0.000 118 g, ν_eff = 4.5, truncated to 4: k = 2.87, U = 0.000 34 g.", "u(E) = √(0,000 114² + 0,000 029²) g = 0,000 118 g, ν_eff = 4,5, 4'e kesilir: k = 2,87, U = 0,000 34 g.")],
    0.0, 0.0, None, None, None, None,
    {"estimate": chk(0.0, 1e-12), "standard_uncertainty": chk(0.000118, 5e-7), "effective_dof": chk(4.5, 0.6), "coverage_factor": chk(2.87, 0.005), "expanded_uncertainty": chk(0.00034, 5e-6)},
    T("EURAMET cg-18 v4.0 H1.3/A, option 1, load 0: u(E) = 0.000 118 g, ν_eff = 4, k = 2.87, U = 0.000 34 g.", "EURAMET cg-18 v4.0 H1.3/A, seçenek 1, yük 0: u(E) = 0,000 118 g, ν_eff = 4, k = 2,87, U = 0,000 34 g."))
project("h1-220g-max-option1",
    T("Laboratory balance 220 g / 0.1 mg: error at 220 g without buoyancy correction (EURAMET cg-18 H1, option 1)", "Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesi olmadan 220 g'da hata (EURAMET cg-18 H1, seçenek 1)"),
    T("Not adjusted before calibration and no air density known: the buoyancy term is the conservative limit of formula 7.1.2-5d and dominates.", "Kalibrasyondan önce ayarlanmamış ve hava yoğunluğu bilinmiyor: kaldırma kuvveti terimi 7.1.2-5d formülünün temkinli sınırıdır ve baskındır."),
    [T("Load 200 g + 20 g, m_c = 220.0001 g, u(m_c) = 0.000 062 g; mpe(E2) = 0.30 + 0.08 mg. Buoyancy u_rel = (0.1ρ₀/ρ_c + mpe/(4m_N))/√3 = 8.9·10⁻⁶: u = 0.001 96 g.",
       "Yük 200 g + 20 g, m_c = 220,0001 g, u(m_c) = 0,000 062 g; mpe(E2) = 0,30 + 0,08 mg. Kaldırma kuvveti u_rel = (0,1ρ₀/ρ_c + mpe/(4m_N))/√3 = 8,9·10⁻⁶: u = 0,001 96 g."),
     T("I = 220.0014 g, E = 0.0013 g, u(E) = 0.001 97 g, ν_eff ≈ 3.6·10⁵, k = 2.00, U = 0.003 94 g.", "I = 220,0014 g, E = 0,0013 g, u(E) = 0,001 97 g, ν_eff ≈ 3,6·10⁵, k = 2,00, U = 0,003 94 g.")],
    220.0, 220.0014, 220.0001, 0.000062, 0.00038, "5d",
    {"estimate": chk(0.0013, 1e-9), "standard_uncertainty": chk(0.001971, 1e-6), "coverage_factor": chk(2.0, 0.005), "expanded_uncertainty": chk(0.00394, 5e-6)},
    T("EURAMET cg-18 v4.0 H1.3/A, option 1 (7.1.2-5d), 220 g: u(E) = 0.001 971 g, ν_eff = 357 098, k = 2.00, U = 0.003 94 g.", "EURAMET cg-18 v4.0 H1.3/A, seçenek 1 (7.1.2-5d), 220 g: u(E) = 0,001 971 g, ν_eff = 357 098, k = 2,00, U = 0,003 94 g."))
project("h1-220g-max-option1-delta-t",
    T("Laboratory balance 220 g / 0.1 mg: error at 220 g with the buoyancy term for ΔT = 5 K (EURAMET cg-18 H1, option 1, formula 7.1.2-5e)", "Laboratuvar terazisi 220 g / 0,1 mg: ΔT = 5 K için kaldırma kuvveti terimiyle 220 g'da hata (EURAMET cg-18 H1, seçenek 1, 7.1.2-5e formülü)"),
    T("As option 1, but the buoyancy uncertainty follows from the estimated temperature variation of the room, 5 K, instead of the worst case.", "Seçenek 1 gibi, ancak kaldırma kuvveti belirsizliği en kötü durum yerine odanın tahmini 5 K sıcaklık değişiminden çıkar."),
    [T("u_rel = √(1.07·10⁻⁴ + 1.33·10⁻⁶ K⁻²·ΔT²)·ρ₀/ρ_c + mpe/(4m_N√3) = 2.03·10⁻⁶: u = 0.000 446 g.", "u_rel = √(1,07·10⁻⁴ + 1,33·10⁻⁶ K⁻²·ΔT²)·ρ₀/ρ_c + mpe/(4m_N√3) = 2,03·10⁻⁶: u = 0,000 446 g."),
     T("u(E) = 0.000 491 g, ν_eff = 1377, k = 2.00, U = 0.000 98 g: the conservative buoyancy limit of option 1 inflates U fourfold.", "u(E) = 0,000 491 g, ν_eff = 1377, k = 2,00, U = 0,000 98 g: seçenek 1'in temkinli kaldırma kuvveti sınırı U'yu dört kat büyütür.")],
    220.0, 220.0014, 220.0001, 0.000062, 0.00038, "5e",
    {"estimate": chk(0.0013, 1e-9), "standard_uncertainty": chk(0.000491, 1e-6), "effective_dof": chk(1377, 10), "coverage_factor": chk(2.0, 0.005), "expanded_uncertainty": chk(0.00098, 5e-6)},
    T("EURAMET cg-18 v4.0 H1.3/A, option 1 with 7.1.2-5e, 220 g: u(E) = 0.000 491 g, ν_eff = 1377, k = 2.00, U = 0.000 98 g.", "EURAMET cg-18 v4.0 H1.3/A, 7.1.2-5e ile seçenek 1, 220 g: u(E) = 0,000 491 g, ν_eff = 1377, k = 2,00, U = 0,000 98 g."))
for L, I, m_c, u_mc, conv, uE, nu, k, U, lbl in [(220.0, 220.0014, 220.0001, 0.000062, 0.000092, 0.000226, 62, 2.05, 0.00046, "220g-max"),
                                                (100.0, 100.0006, 99.9999, 0.000025, 0.000046, 0.000149, 11, 2.25, 0.00033, "100g")]:
    Ls = f"{L:.0f}"
    project(f"h1-{lbl}-option2",
        T(f"Laboratory balance 220 g / 0.1 mg: error at {Ls} g with the buoyancy correction (EURAMET cg-18 H1, option 2)", f"Laboratuvar terazisi 220 g / 0,1 mg: kaldırma kuvveti düzeltmesiyle {Ls} g'da hata (EURAMET cg-18 H1, seçenek 2)"),
        T("The air density during calibration is known (990 hPa, 50 %RH, 21 °C: 1.173 kg/m³), so the buoyancy correction is applied; the test weights are 2 K from room temperature (convection).",
          "Kalibrasyon sırasında hava yoğunluğu bilinir (990 hPa, %50 bağıl nem, 21 °C: 1,173 kg/m³); bu nedenle kaldırma kuvveti düzeltmesi uygulanır; deney ağırlıkları oda sıcaklığından 2 K farklıdır (konveksiyon)."),
        [T("δm_B = −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) (7.1.2-4) with ρ_a = (1.173 ± 0.014) kg/m³ and ρ = (7950 ± 70) kg/m³; its uncertainty (7.1.2-5a) follows from the model: about 3·10⁻⁸ relative.",
           "δm_B = −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) (7.1.2-4), ρ_a = (1,173 ± 0,014) kg/m³ ve ρ = (7950 ± 70) kg/m³; belirsizliği (7.1.2-5a) modelden çıkar: göreli yaklaşık 3·10⁻⁸."),
         T(f"u(E) = {uE:.6f} g, ν_eff = {nu}, k = {k:.2f}, U = {U:.5f} g.", f"u(E) = {uE:.6f} g, ν_eff = {nu}, k = {k:.2f}, U = {U:.5f} g.".replace(".", ","))],
        L, I, m_c, u_mc, None, "correction",
        {"standard_uncertainty": chk(uE, 1e-6), "effective_dof": chk(nu, 1.5), "coverage_factor": chk(k, 0.01), "expanded_uncertainty": chk(U, 5e-6)},
        T(f"EURAMET cg-18 v4.0 H1.3/A, option 2, {Ls} g: u(E) = {uE:.6f} g, ν_eff = {nu}, k = {k:.2f}, U = {U:.5f} g. The guide takes k from Table G.2 of the GUM (2.05 at ν = 62 is the entry for ν = 50); the exact t-value is 2.04.", f"EURAMET cg-18 v4.0 H1.3/A, seçenek 2, {Ls} g: u(E) = {uE:.6f} g, ν_eff = {nu}, k = {k:.2f}, U = {U:.5f} g.".replace(".", ",").replace("EURAMET cg-18 v4,0 H1,3/A", "EURAMET cg-18 v4.0 H1.3/A") + " Kılavuz k'yı GUM Tablo G.2'den alır (ν = 62'de 2,05, ν = 50 satırıdır); tam t değeri 2,04'tür."),
        conv=conv)
print("written")
