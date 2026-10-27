"""Writes the EURAMET cg-18 v4.0 Examples H2 and H3 library projects (run from the repository root)."""
import json
import math
import statistics
from pathlib import Path

DOC = "EURAMET Calibration Guide No. 18 v4.0 (2015), Guidelines on the Calibration of Non-Automatic Weighing Instruments"
URL = "https://www.euramet.org/publications-media-centre/calibration-guidelines"
R3 = math.sqrt(3)
RHO0, RHOC = 1.2, 8000.0


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def tr_num(s):
    return s.replace(".", ",")


def inp(name, dist, en, tr, unit):
    return {"name": name, "unit": unit, "description": T(en, tr), "distribution": dist}


def rect(hw):
    return {"type": "rectangular", "value": 0.0, "half_width": hw}


def rep(s, dof):
    return {"type": "type_a_pooled", "mean": 0.0, "pooled_sd": s, "pooled_dof": dof, "n": 1}


class Indication:
    """Inputs of one indication I: rounding at zero and at the load, repeatability, eccentricity, creep."""

    def __init__(self, tag, label_en, label_tr, unit, d0, dL, s, dof, ecc_hw, creep_hw=0.0, zero=False):
        self.tag, self.unit = tag, unit
        sfx = f"_{tag}" if tag else ""
        self.inputs = [inp(f"dI_dig0{sfx}", rect(d0 / 2), f"Rounding at zero{label_en}", f"Sıfırda yuvarlama{label_tr}", unit),
                       inp(f"dI_rep{sfx}", rep(s, dof), f"Repeatability{label_en} (s = {s:.4g} {unit}, ν = {dof})",
                           f"Tekrarlanabilirlik{label_tr} (s = {tr_num(f'{s:.4g}')} {unit}, ν = {dof})", unit)]
        terms = [f"dI_dig0{sfx}", f"dI_rep{sfx}"]
        if not zero:
            self.inputs += [inp(f"dI_digL{sfx}", rect(dL / 2), f"Rounding at the load{label_en}", f"Yükte yuvarlama{label_tr}", unit),
                            inp(f"dI_ecc{sfx}", rect(ecc_hw), f"Eccentricity{label_en} (7.1.1-10)", f"Eksantriklik{label_tr} (7.1.1-10)", unit)]
            terms += [f"dI_digL{sfx}", f"dI_ecc{sfx}"]
            if creep_hw:
                self.inputs.append(inp(f"dI_time{sfx}", rect(creep_hw), f"Creep and hysteresis{label_en} (7.4.4-7)",
                                       f"Sürünme ve histerezis{label_tr} (7.4.4-7)", unit))
                terms.append(f"dI_time{sfx}")
        self.expr = " + ".join(terms)


def write(pid, title, desc, expl, sec, tags, inputs, expr, expected, ref, notes, unit, dof_note=None):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "mass",
                      "sources": [{"document": DOC, "section": sec, "url": URL}],
                      "tags": ["euramet-cg18", "balance", "weighing-instrument", "calibration", "validation"] + tags},
         "model": {"output": {"name": "E", "unit": unit, "expression": expr, "description": T("Error of indication", "Gösterge hatası")},
                   "inputs": inputs},
         "analysis": {"gum": {"coverage_probability": 0.9545}, "mcm": {"trials": 200000, "seed": 1}},
         "expected": {"gum": expected, "reference": ref}}
    if notes:
        p["metadata"]["notes"] = notes
    Path(f"library/mass/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")


# ---------------------------------------------------------------------------
# H2: 60 kg multi-interval balance, 2 g / 5 g / 10 g
H2_REP = {"A": ([9998, 10000, 9998, 10000, 10000], [24995, 25000, 24995, 24995, 25000]),
          "B": ([10000, 10000, 9998, 10000, 10000], [25000, 25000, 25000, 24995, 25000])}
F2_MPE = {10000: 0.16, 20000: 0.30}   # OIML R111 class F2, g
LOADS = {10000: [10000], 20000: [20000], 40000: [20000, 20000], 60000: [20000, 20000, 20000]}


def h2_d(L):
    return 2 if L <= 12000 else 5 if L <= 30000 else 10


def h2_common(sit):
    r1, r2 = H2_REP[sit]
    s1, s2 = statistics.stdev(r1), statistics.stdev(r2)
    return s1, s2, [
        T("Multi-interval instrument 12 kg / 2 g, 30 kg / 5 g, 60 kg / 10 g: E = I − m_ref (7.1-1) with the rounding at zero (d₀ = 2 g) and at the load (d of its interval), repeatability, eccentricity and the reference mass.",
          "Çok aralıklı terazi 12 kg / 2 g, 30 kg / 5 g, 60 kg / 10 g: sıfırda (d₀ = 2 g) ve yükte (aralığının d'si) yuvarlama, tekrarlanabilirlik, eksantriklik ve referans kütle ile E = I − m_ref (7.1-1)."),
        T(f"Repeatability from five readings at 10 kg (interval 1) and 25 kg (intervals 2 and 3): s = {s1:.3f} g and {s2:.3f} g, ν = 4 (7.1.1-5).",
          f"10 kg'da (1. aralık) ve 25 kg'da (2. ve 3. aralık) beş okumadan tekrarlanabilirlik: s = {tr_num(f'{s1:.3f}')} g ve {tr_num(f'{s2:.3f}')} g, ν = 4 (7.1.1-5)."),
        T("Eccentricity ΔI_ecc,max = 5 g at L_ecc = 20 kg: half-width ΔI·L/(2L_ecc), rectangular (7.1.1-10).",
          "Eksantriklik L_ecc = 20 kg'da ΔI_ecc,maks = 5 g: yarı genişlik ΔI·L/(2L_ecc), dikdörtgen (7.1.1-10)."),
        T("F2 weights used at their nominal value: u(m_c) = mpe/√3 (7.1.2-3), with the mpe summed for a combination of weights; drift D = mpe/2, u = mpe/(2√3) (7.1.2-11, k_D = 1.5).",
          "Nominal değerinde kullanılan F2 ağırlıkları: u(m_c) = mpe/√3 (7.1.2-3), ağırlık kombinasyonu için mpe'ler toplanır; sürüklenme D = mpe/2, u = mpe/(2√3) (7.1.2-11, k_D = 1,5)."),
    ]


def h2(name, sit, L, I, buoyancy, expected, ref, extra_expl, notes, title, desc):
    s1, s2, common = h2_common(sit)
    s = s1 if L <= 12000 else s2
    unit = "g"
    ind = Indication("", "", "", unit, 2.0, h2_d(L), s, 4, 5.0 * L / (2 * 20000.0), zero=(L == 0))
    inputs = [inp("I_L", {"type": "constant", "value": float(I)}, "Indication at the load", "Yükteki gösterge", unit)] + ind.inputs
    expr = f"I_L + {ind.expr}"
    if L:
        mpe = sum(F2_MPE[w] for w in LOADS[L])
        inputs += [inp("m_N", {"type": "constant", "value": float(L)}, "Nominal value of the test load", "Deney yükünün nominal değeri", unit),
                   inp("dm_c", rect(mpe), f"Weights at nominal value, F2 mpe = {mpe:.2f} g (7.1.2-3)", f"Nominal değerindeki ağırlıklar, F2 mpe = {tr_num(f'{mpe:.2f}')} g (7.1.2-3)", unit),
                   inp("dm_D", rect(mpe / 2), "Drift of the weights, D = mpe/2 (7.1.2-11)", "Ağırlıkların sürüklenmesi, D = mpe/2 (7.1.2-11)", unit)]
        m = "m_N + dm_c + dm_D"
        if buoyancy == "5d":
            inputs.append(inp("dm_B", rect(L * (0.1 * RHO0 / RHOC + mpe / (4 * L))), "Air buoyancy, not adjusted before calibration (7.1.2-5d)", "Hava kaldırma kuvveti, kalibrasyondan önce ayarlanmamış (7.1.2-5d)", unit))
            m += " + dm_B"
        elif buoyancy == "5e":
            u = L * (math.sqrt(1.07e-4 + 1.33e-6 * 100) * RHO0 / RHOC + mpe / (4 * L * R3))
            inputs.append(inp("dm_B", {"type": "normal", "mean": 0.0, "sd": u}, "Air buoyancy for ΔT = 10 K (7.1.2-5e)", "ΔT = 10 K için hava kaldırma kuvveti (7.1.2-5e)", unit))
            m += " + dm_B"
        elif buoyancy == "5c":
            inputs.append(inp("dm_B", rect(mpe / 4), "Air buoyancy, adjusted immediately before calibration (7.1.2-5c)", "Hava kaldırma kuvveti, kalibrasyondan hemen önce ayarlanmış (7.1.2-5c)", unit))
            m += " + dm_B"
        else:  # option 2: correction applied
            inputs += [inp("rho_a", {"type": "normal", "mean": 1.173, "sd": 0.018}, "Air density during calibration (CIPM formula; ΔT = 10 K by A3-2)", "Kalibrasyon sırasında hava yoğunluğu (CIPM formülü; A3-2 ile ΔT = 10 K)", "kg/m³"),
                       inp("rho", {"type": "normal", "mean": 7950.0, "sd": 70.0}, "Density of the weights", "Ağırlıkların yoğunluğu", "kg/m³"),
                       inp("rho_0", {"type": "constant", "value": RHO0}, "Reference air density", "Referans hava yoğunluğu", "kg/m³"),
                       inp("rho_c", {"type": "constant", "value": RHOC}, "Reference density of the weights", "Ağırlıkların referans yoğunluğu", "kg/m³")]
            m += " - m_N*(rho_a - rho_0)*(1/rho - 1/rho_c)"
        expr += f" - ({m})"
    write(f"cg18-{name}", title, desc, common + extra_expl, f"Appendix H2, H2.3/{sit}", ["multi-interval"], inputs, expr, expected, ref, notes, unit)


MPE60 = T("The budget uses mpe = 0.96 g for the 60 kg test load (u(m_c) = 0.554 g, u(m_D) = 0.277 g, u(m_B) = 0.658 g), while three 20 kg F2 weights have mpe = 3 × 0.30 g = 0.90 g (OIML R111). The project uses 0.90 g: u(m_ref) = 0.871 g instead of 0.904 g, and u(E) = 5.973 g instead of 5.978 g (research log F68).",
          "Bütçe 60 kg'lık deney yükü için mpe = 0,96 g kullanır (u(m_c) = 0,554 g, u(m_D) = 0,277 g, u(m_B) = 0,658 g); oysa üç adet 20 kg F2 ağırlığın mpe'si 3 × 0,30 g = 0,90 g'dır (OIML R111). Proje 0,90 g kullanır: u(m_ref) 0,904 g yerine 0,871 g, u(E) 5,978 g yerine 5,973 g olur (araştırma günlüğü F68).")
KG2 = T("The guide reads k from GUM Table G.2 at the next lower tabulated ν (2.05 at ν = 90 from the row for ν = 50; 2.025 at ν = 172); the program uses the exact t-quantile for 95.45 % (2.03 and 2.01).",
        "Kılavuz k'yı GUM Tablo G.2'den bir alt tablo satırında okur (ν = 90'da ν = 50 satırından 2,05; ν = 172'de 2,025); program %95,45 için tam t-kantilini kullanır (2,03 ve 2,01).")

h2("h2-60kg-zero", "A", 0, 0, None,
   {"estimate": chk(0.0, 1e-12), "standard_uncertainty": chk(1.238, 5e-4), "coverage_factor": chk(2.52, 0.005), "expanded_uncertainty": chk(3.120, 5e-3)},
   T("EURAMET cg-18 v4.0 H2.3/A, load 0: u(E) = 1.238 g, ν_eff = 6, k = 2.52, U = 3.120 g.", "EURAMET cg-18 v4.0 H2.3/A, yük 0: u(E) = 1,238 g, ν_eff = 6, k = 2,52, U = 3,120 g."),
   [T("At zero only d₀ and the repeatability of interval 1 contribute: u(E) = √(1.095² + 0.577²) = 1.238 g, ν_eff = 6.5, truncated to 6: k = 2.52, U = 3.12 g.",
      "Sıfırda yalnızca d₀ ve 1. aralığın tekrarlanabilirliği katkı verir: u(E) = √(1,095² + 0,577²) = 1,238 g, ν_eff = 6,5, 6'ya kesilir: k = 2,52, U = 3,12 g.")],
   None,
   T("Multi-interval balance 60 kg: error at zero load (EURAMET cg-18 H2)", "Çok aralıklı terazi 60 kg: sıfır yükte hata (EURAMET cg-18 H2)"),
   T("A 60 kg multi-interval balance (2 g / 5 g / 10 g) calibrated as found, not adjusted before calibration.", "Bulunduğu hâliyle, kalibrasyondan önce ayarlanmadan kalibre edilen 60 kg'lık çok aralıklı terazi (2 g / 5 g / 10 g)."))
h2("h2-60kg-10kg-option1", "A", 10000, 10000, "5d",
   {"estimate": chk(0.0, 1e-9), "standard_uncertainty": chk(1.552, 5e-4), "effective_dof": chk(16, 0.6), "coverage_factor": chk(2.17, 0.005), "expanded_uncertainty": chk(3.369, 5e-3)},
   T("EURAMET cg-18 v4.0 H2.3/A, option 1 (7.1.2-5d), 10 kg: u(E) = 1.552 g, ν_eff = 16, k = 2.17, U = 3.369 g.", "EURAMET cg-18 v4.0 H2.3/A, seçenek 1 (7.1.2-5d), 10 kg: u(E) = 1,552 g, ν_eff = 16, k = 2,17, U = 3,369 g."),
   [T("Interval 1 (d = 2 g): u(I) = √(1.095² + 0.577² + 0.577² + 0.722²) = 1.545 g; one 10 kg F2 weight (mpe 0.16 g): u(m_c) = 0.092, u(m_D) = 0.046, u(m_B) = 0.110 g (7.1.2-5d); u(m_ref) = 0.151 g.",
      "1. aralık (d = 2 g): u(I) = √(1,095² + 0,577² + 0,577² + 0,722²) = 1,545 g; bir adet 10 kg F2 ağırlık (mpe 0,16 g): u(m_c) = 0,092, u(m_D) = 0,046, u(m_B) = 0,110 g (7.1.2-5d); u(m_ref) = 0,151 g."),
    T("u(E) = 1.552 g, ν_eff = 16, k = 2.17, U = 3.37 g.", "u(E) = 1,552 g, ν_eff = 16, k = 2,17, U = 3,37 g.")],
   None,
   T("Multi-interval balance 60 kg: error at 10 kg without buoyancy correction (EURAMET cg-18 H2, option 1)", "Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesi olmadan 10 kg'da hata (EURAMET cg-18 H2, seçenek 1)"),
   T("First load point, in the 2 g interval; no air density known and not adjusted before calibration: conservative buoyancy term 7.1.2-5d.", "2 g'lık aralıktaki ilk yük noktası; hava yoğunluğu bilinmiyor ve kalibrasyondan önce ayarlanmamış: temkinli kaldırma kuvveti terimi 7.1.2-5d."))
h2("h2-60kg-max-option1", "A", 60000, 59990, "5d",
   {"estimate": chk(-10.0, 1e-9), "standard_uncertainty": chk(5.978, 0.006), "effective_dof": chk(90, 1.5), "coverage_factor": chk(2.05, 0.025), "expanded_uncertainty": chk(12.254, 0.15)},
   T("EURAMET cg-18 v4.0 H2.3/A, option 1 (7.1.2-5d), 60 kg: E = −10 g, u(E) = 5.978 g, ν_eff = 90, k = 2.05, U = 12.254 g; the tolerances cover the mpe of the test load and the k of Table G.2 (notes).",
     "EURAMET cg-18 v4.0 H2.3/A, seçenek 1 (7.1.2-5d), 60 kg: E = −10 g, u(E) = 5,978 g, ν_eff = 90, k = 2,05, U = 12,254 g; toleranslar deney yükünün mpe'sini ve Tablo G.2'nin k'sını kapsar (notlar)."),
   [T("Interval 3 (d = 10 g): u(I) = √(2.739² + 0.577² + 2.887² + 4.330²) = 5.909 g; test load 3 × 20 kg F2 (mpe 0.90 g): u(m_ref) = 0.871 g.",
      "3. aralık (d = 10 g): u(I) = √(2,739² + 0,577² + 2,887² + 4,330²) = 5,909 g; deney yükü 3 × 20 kg F2 (mpe 0,90 g): u(m_ref) = 0,871 g."),
    T("E = 59 990 − 60 000 = −10 g; u(E) = 5.973 g, ν_eff = 90, k = 2.03, U = 12.11 g.", "E = 59 990 − 60 000 = −10 g; u(E) = 5,973 g, ν_eff = 90, k = 2,03, U = 12,11 g.")],
   [MPE60, KG2],
   T("Multi-interval balance 60 kg: error at Max without buoyancy correction (EURAMET cg-18 H2, option 1)", "Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesi olmadan Max'ta hata (EURAMET cg-18 H2, seçenek 1)"),
   T("Error at 60 kg in the 10 g interval, the largest U of the certificate.", "10 g'lık aralıkta 60 kg'da hata; sertifikadaki en büyük U."))
h2("h2-60kg-max-option1-delta-t", "A", 60000, 59990, "5e",
   {"estimate": chk(-10.0, 1e-9), "standard_uncertainty": chk(5.948, 0.006), "coverage_factor": chk(2.05, 0.025), "expanded_uncertainty": chk(12.193, 0.15)},
   T("EURAMET cg-18 v4.0 H2.3/A, option 1 with 7.1.2-5e (ΔT = 10 K), 60 kg: u(E) = 5.948 g, ν_eff = 88, k = 2.05, U = 12.193 g.", "EURAMET cg-18 v4.0 H2.3/A, 7.1.2-5e ile seçenek 1 (ΔT = 10 K), 60 kg: u(E) = 5,948 g, ν_eff = 88, k = 2,05, U = 12,193 g."),
   [T("Buoyancy from the room temperature variation of 10 K: u_rel = √(1.07·10⁻⁴ + 1.33·10⁻⁶·100)·ρ₀/ρ_c + mpe/(4m_N√3) = 4.49·10⁻⁶, u = 0.269 g; u(m_ref) = 0.640 g.",
      "10 K'lik oda sıcaklığı değişiminden kaldırma kuvveti: u_rel = √(1,07·10⁻⁴ + 1,33·10⁻⁶·100)·ρ₀/ρ_c + mpe/(4m_N√3) = 4,49·10⁻⁶, u = 0,269 g; u(m_ref) = 0,640 g."),
    T("u(E) = 5.944 g, k = 2.03, U = 12.06 g; the buoyancy term hardly matters next to the indication.", "u(E) = 5,944 g, k = 2,03, U = 12,06 g; kaldırma kuvveti terimi göstergenin yanında neredeyse önemsizdir.")],
   [MPE60, KG2,
    T("With mpe = 0.96 g, formula 7.1.2-5e gives 0.278 g at 60 kg and 0.180 g at 40 kg; the table prints 0.276 g and 0.178 g.",
      "mpe = 0,96 g ile 7.1.2-5e formülü 60 kg'da 0,278 g ve 40 kg'da 0,180 g verir; tabloda 0,276 g ve 0,178 g basılıdır.")],
   T("Multi-interval balance 60 kg: error at Max with the buoyancy term for ΔT = 10 K (EURAMET cg-18 H2, option 1, formula 7.1.2-5e)", "Çok aralıklı terazi 60 kg: ΔT = 10 K için kaldırma kuvveti terimiyle Max'ta hata (EURAMET cg-18 H2, seçenek 1, 7.1.2-5e formülü)"),
   T("As option 1, with the buoyancy uncertainty from the expected temperature variation of the room (10 K, a room with windows).", "Seçenek 1 gibi; kaldırma kuvveti belirsizliği odanın beklenen sıcaklık değişiminden (10 K, pencereli oda)."))
h2("h2-60kg-max-option2", "A", 60000, 59990, "correction",
   {"estimate": chk(-10.0, 0.005), "standard_uncertainty": chk(5.941, 0.006), "coverage_factor": chk(2.05, 0.025), "expanded_uncertainty": chk(12.180, 0.15)},
   T("EURAMET cg-18 v4.0 H2.3/A, option 2, 60 kg: δm_B = 2.138·10⁻⁸·m_N, u(E) = 5.941 g, ν_eff = 88, k = 2.05, U = 12.180 g.", "EURAMET cg-18 v4.0 H2.3/A, seçenek 2, 60 kg: δm_B = 2,138·10⁻⁸·m_N, u(E) = 5,941 g, ν_eff = 88, k = 2,05, U = 12,180 g."),
   [T("Air density 1.173 kg/m³ (990 hPa, 50 %RH, 21 °C) with u = 0.018 kg/m³ for a temperature variation of 10 K (A3-2); weights ρ = (7950 ± 70) kg/m³.",
      "Hava yoğunluğu 1,173 kg/m³ (990 hPa, %50 bağıl nem, 21 °C), 10 K'lik sıcaklık değişimi için u = 0,018 kg/m³ (A3-2); ağırlıklar ρ = (7950 ± 70) kg/m³."),
    T("The correction −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) = +1.3 mg and its uncertainty (about 2 mg) are negligible next to d = 10 g: u(E) = 5.938 g, U = 12.05 g.",
      "Düzeltme −m_N(ρ_a − ρ₀)(1/ρ − 1/ρ_c) = +1,3 mg ve belirsizliği (yaklaşık 2 mg), d = 10 g'ın yanında ihmal edilebilir: u(E) = 5,938 g, U = 12,05 g.")],
   [MPE60, KG2],
   T("Multi-interval balance 60 kg: error at Max with the buoyancy correction (EURAMET cg-18 H2, option 2)", "Çok aralıklı terazi 60 kg: kaldırma kuvveti düzeltmesiyle Max'ta hata (EURAMET cg-18 H2, seçenek 2)"),
   T("The air density during calibration is computed from the CIPM formula and the buoyancy correction applied; the adjustment air density is unknown.", "Kalibrasyon sırasındaki hava yoğunluğu CIPM formülünden hesaplanır ve kaldırma kuvveti düzeltmesi uygulanır; ayar sırasındaki hava yoğunluğu bilinmez."))
h2("h2-60kg-max-adjusted", "B", 60000, 60000, "5c",
   {"estimate": chk(0.0, 1e-9), "standard_uncertainty": chk(5.739, 0.015), "effective_dof": chk(172, 2), "coverage_factor": chk(2.025, 0.015), "expanded_uncertainty": chk(11.601, 0.08)},
   T("EURAMET cg-18 v4.0 H2.3/B, option 1 (7.1.2-5c), 60 kg: u(E) = 5.739 g, ν_eff = 172, k = 2.025, U = 11.601 g.", "EURAMET cg-18 v4.0 H2.3/B, seçenek 1 (7.1.2-5c), 60 kg: u(E) = 5,739 g, ν_eff = 172, k = 2,025, U = 11,601 g."),
   [T("Adjusted with the built-in weights immediately before calibration: buoyancy only from the density tolerance of the weights, u_rel = mpe/(4m_N√3) (7.1.2-5c).",
      "Kalibrasyondan hemen önce dahili ağırlıklarla ayarlanmış: kaldırma kuvveti yalnızca ağırlıkların yoğunluk toleransından, u_rel = mpe/(4m_N√3) (7.1.2-5c)."),
    T("s = 2.236 g at 25 kg: u(I) = 5.694 g, u(m_ref) = 0.595 g, u(E) = 5.725 g, ν_eff = 172, k = 2.01, U = 11.53 g.",
      "25 kg'da s = 2,236 g: u(I) = 5,694 g, u(m_ref) = 0,595 g, u(E) = 5,725 g, ν_eff = 172, k = 2,01, U = 11,53 g.")],
   [T("With mpe = 0.96 g as in the budget, u(m_ref) = 0.635 g and u(E) = 5.729 g; the table prints 5.739 g, which does not follow from its own entries √(5.694² + 0.635²) (F68).",
      "Bütçedeki gibi mpe = 0,96 g ile u(m_ref) = 0,635 g ve u(E) = 5,729 g olur; tabloda 5,739 g basılıdır ve kendi girdilerinden √(5,694² + 0,635²) çıkmaz (F68)."),
    KG2],
   T("Multi-interval balance 60 kg, adjusted before calibration: error at Max (EURAMET cg-18 H2, second situation)", "Çok aralıklı terazi 60 kg, kalibrasyondan önce ayarlanmış: Max'ta hata (EURAMET cg-18 H2, ikinci durum)"),
   T("Second situation: the instrument is adjusted with its built-in weights immediately before calibration; the repeatability is better (s = 2.236 g).", "İkinci durum: terazi kalibrasyondan hemen önce dahili ağırlıklarıyla ayarlanır; tekrarlanabilirlik daha iyidir (s = 2,236 g)."))

# ---------------------------------------------------------------------------
# H3: weighbridge 30 000 kg / 10 kg, tested with d_T = 1 kg, substitution loads
REP3 = [10405, 10414, 10418, 10412, 10418, 10425]
S3 = statistics.stdev(REP3)
ECC3 = 15.0 / 10420.0            # relative eccentricity of the 10 420 kg test (larger than 24/24 160)
E0, MAX3 = 4.0, 30000.0
MPE_T = 0.050                   # mpe of one 1000 kg M1 weight, kg
BUOY_REL = 0.1 * RHO0 / RHOC + MPE_T / (4 * 1000.0)   # half-width of 7.1.2-5d per kg of load


def ind3(tag, L, en, tr, zero=False):
    return Indication(tag, en, tr, "kg", 1.0, 1.0, S3, 5, ECC3 * L / 2, creep_hw=E0 * L / MAX3, zero=zero)


H3_COMMON = [
    T("Weighbridge 30 000 kg / 10 kg, tested in the service mode d_T = 1 kg: E = I − L_T (7.1-1). Each indication carries the rounding at zero and at the load (d_T/√12), repeatability, eccentricity and creep/hysteresis.",
      "Köprü kantar 30 000 kg / 10 kg, servis modunda d_T = 1 kg ile denenmiştir: E = I − L_T (7.1-1). Her gösterge sıfırda ve yükte yuvarlama (d_T/√12), tekrarlanabilirlik, eksantriklik ve sürünme/histerezis taşır."),
    T(f"Repeatability from six loadings of about 10 420 kg (10 405 … 10 425 kg): s = {S3:.2f} kg, ν = 5 (7.1.1-5). Eccentricity 15 kg at 10 420 kg, the larger relative value of the two tests: half-width ΔI·L/(2L_ecc) (7.1.1-10). Creep from the return to zero E₀ = 4 kg: half-width E₀·L/Max (7.4.4-7).",
      f"Yaklaşık 10 420 kg'lık altı yüklemeden tekrarlanabilirlik (10 405 … 10 425 kg): s = {tr_num(f'{S3:.2f}')} kg, ν = 5 (7.1.1-5). Eksantriklik 10 420 kg'da 15 kg, iki denemenin bağıl olarak büyüğü: yarı genişlik ΔI·L/(2L_ecc) (7.1.1-10). Sıfıra dönüşteki E₀ = 4 kg'dan sürünme: yarı genişlik E₀·L/Max (7.4.4-7)."),
    T("Standard weights 10 × 1000 kg, M1 (mpe 50 g each), at nominal value: u(m_c) = mpe/√3 and drift D = mpe (7.1.2-3, -11), summed linearly for the weights used; buoyancy 7.1.2-5d with u_rel = 1.6·10⁻⁵, for the substitution loads too (ρ ≥ 4400 kg/m³).",
      "Standart ağırlıklar 10 × 1000 kg, M1 (her biri mpe 50 g), nominal değerinde: u(m_c) = mpe/√3 ve sürüklenme D = mpe (7.1.2-3, -11), kullanılan ağırlıklar için doğrusal toplanır; kaldırma kuvveti 7.1.2-5d, u_rel = 1,6·10⁻⁵, ikame yükleri için de (ρ ≥ 4400 kg/m³)."),
]


def weights(prefix, n_t, en, tr):
    """Inputs for n_t tonnes of the standard weights; the same weights are used in every step (fully correlated)."""
    return [inp(f"dm_c{prefix}", rect(MPE_T), f"Error of one 1000 kg weight{en} (M1, at nominal value)", f"Bir adet 1000 kg ağırlığın hatası{tr} (M1, nominal değerinde)", "kg"),
            inp(f"dm_D{prefix}", rect(MPE_T), f"Drift of one 1000 kg weight{en}, D = mpe", f"Bir adet 1000 kg ağırlığın sürüklenmesi{tr}, D = mpe", "kg"),
            inp(f"dm_B{prefix}", rect(BUOY_REL * 1000.0), f"Air buoyancy per 1000 kg{en} (7.1.2-5d)", f"1000 kg başına hava kaldırma kuvveti{tr} (7.1.2-5d)", "kg")]


SUBST_EXPL = T("Substitution (4.3.3, 7.1.2-15): L_sub1 = m_ref + I(L_sub1) − I(m_ref) at 10 t, and L_sub1 + L_sub2 = 2m_ref + I₁ + I₂ with I₂ = I(L_sub1 + L_sub2) − I(m_ref + L_sub1) at 20 t. Every indication used for the adjustment brings its own rounding, repeatability, eccentricity and creep; the standard weights are the same in every step.",
               "İkame (4.3.3, 7.1.2-15): 10 t'da L_ikame1 = m_ref + I(L_ikame1) − I(m_ref) ve 20 t'da I₂ = I(L_ikame1 + L_ikame2) − I(m_ref + L_ikame1) ile L_ikame1 + L_ikame2 = 2m_ref + I₁ + I₂. Ayar için kullanılan her gösterge kendi yuvarlamasını, tekrarlanabilirliğini, eksantrikliğini ve sürünmesini getirir; standart ağırlıklar her adımda aynıdır.")
DOF3 = T("The guide's ν_eff counts only the repeatability of the indication at the load; the repeatability terms of the indications used to adjust the substitution loads (7.1.2-15) are given infinite degrees of freedom. The program gives each of them ν = 5 (research log F69).",
         "Kılavuzun ν_eff'i yalnızca yükteki göstergenin tekrarlanabilirliğini sayar; ikame yüklerini ayarlamada kullanılan göstergelerin tekrarlanabilirlik terimlerine (7.1.2-15) sonsuz serbestlik derecesi verilir. Program her birine ν = 5 verir (araştırma günlüğü F69).")
REP24 = T("The budget uses s = 6.74 kg at every load, although the repeatability test at 24 160 kg gave s = 9.03 kg. With 9.03 kg for the three indications at 20 t and 30 t, u(E) at 30 t would be 26.0 kg instead of 23.9 kg (F69).",
          "Bütçe her yükte s = 6,74 kg kullanır; oysa 24 160 kg'daki tekrarlanabilirlik deneyi s = 9,03 kg vermiştir. 20 t ve 30 t'daki üç gösterge için 9,03 kg ile 30 t'da u(E) 23,9 kg yerine 26,0 kg olurdu (F69).")


def h3(name, L_T, I, n_std, steps, expected, ref, expl, notes, title, desc):
    """steps: 0 (no substitution), 1 (L_sub1) or 2 (L_sub1 + L_sub2)."""
    inputs = [inp("I_L", {"type": "constant", "value": float(I)}, "Indication at the load", "Yükteki gösterge", "kg")]
    cur = ind3("", L_T, "", "", zero=(L_T == 0))
    inputs += cur.inputs
    expr = f"I_L + {cur.expr}"
    if L_T:
        inputs += weights("", 0, "", "")
        unit_w = "(1000 + dm_c + dm_D + dm_B)"
        n_w = n_std + 10 * steps
        inputs.append(inp("n_std", {"type": "constant", "value": float(n_std)}, "Standard weights on the load receptor, t", "Yük alıcısındaki standart ağırlıklar, t", "1"))
        L = f"{n_std} * {unit_w}" if n_std else ""
        if steps:
            i_s1 = ind3("s1", 10000, ", I(L_sub1)", ", I(L_ikame1)")
            i_r1 = ind3("r1", 10000, ", I(m_ref) at 10 t", ", 10 t'da I(m_ref)")
            inputs += i_s1.inputs + i_r1.inputs
            sub = f"10 * {unit_w} + ({i_s1.expr}) - ({i_r1.expr})"
            sub_mass = 10000.0
            if steps == 2:
                i_s2 = ind3("s2", 20000, ", I(L_sub1 + L_sub2)", ", I(L_ikame1 + L_ikame2)")
                i_r2 = ind3("r2", 20000, ", I(m_ref + L_sub1) at 20 t", ", 20 t'da I(m_ref + L_ikame1)")
                inputs += [inp("I_2", {"type": "constant", "value": 10.0}, "I₂ = I(L_sub1 + L_sub2) − I(m_ref + L_sub1) = 20 028 − 20 018 kg", "I₂ = I(L_ikame1 + L_ikame2) − I(m_ref + L_ikame1) = 20 028 − 20 018 kg", "kg")]
                inputs += i_s2.inputs + i_r2.inputs
                sub = f"20 * {unit_w} + I_2 + ({i_s1.expr}) - ({i_r1.expr}) + ({i_s2.expr}) - ({i_r2.expr})"
                sub_mass = 20010.0
            inputs.append(inp("dm_Bsub", rect(BUOY_REL * sub_mass), "Air buoyancy of the substitution loads (7.1.2-5d, ρ ≥ 4400 kg/m³)", "İkame yüklerinin hava kaldırma kuvveti (7.1.2-5d, ρ ≥ 4400 kg/m³)", "kg"))
            L = (L + " + " if L else "") + f"{sub} + dm_Bsub"
        expr += f" - ({L})"
        del n_w
        inputs = [x for x in inputs if x["name"] != "n_std"]
    write(f"cg18-{name}", title, desc, H3_COMMON + expl, "Appendix H3, H3.2/A and H3.3/A", ["weighbridge", "substitution"], inputs, expr, expected, ref, notes, "kg")


h3("h3-30t-zero", 0, 0, 0, 0,
   {"estimate": chk(0.0, 1e-12), "standard_uncertainty": chk(6.75, 0.005), "coverage_factor": chk(2.65, 0.005), "expanded_uncertainty": chk(18, 0.5)},
   T("EURAMET cg-18 v4.0 H3.3/A, load 0: u(E) = 6.75 kg, ν_eff = 5, k = 2.65, U = 18 kg.", "EURAMET cg-18 v4.0 H3.3/A, yük 0: u(E) = 6,75 kg, ν_eff = 5, k = 2,65, U = 18 kg."),
   [T("At zero: u(E) = √(6.74² + 0.29²) = 6.75 kg, ν_eff = 5, k = 2.65, U = 17.9 kg.", "Sıfırda: u(E) = √(6,74² + 0,29²) = 6,75 kg, ν_eff = 5, k = 2,65, U = 17,9 kg.")],
   None,
   T("Weighbridge 30 t: error at zero load (EURAMET cg-18 H3)", "Köprü kantar 30 t: sıfır yükte hata (EURAMET cg-18 H3)"),
   T("A road-vehicle weighbridge 30 000 kg / 10 kg, outdoors, calibrated as found.", "Açık havada, bulunduğu hâliyle kalibre edilen 30 000 kg / 10 kg'lık karayolu taşıtı köprü kantarı."))
h3("h3-30t-10t-weights", 10000, 10010, 10, 0,
   {"estimate": chk(10.0, 1e-9), "standard_uncertainty": chk(7.98, 0.01), "effective_dof": chk(9, 0.85), "coverage_factor": chk(2.32, 0.005), "expanded_uncertainty": chk(19, 0.5)},
   T("EURAMET cg-18 v4.0 H3.3/A, 10 000 kg of standard weights: E = 10 kg, u(E) = 7.98 kg, ν_eff = 9 (9.8 truncated), k = 2.32, U = 19 kg.", "EURAMET cg-18 v4.0 H3.3/A, 10 000 kg standart ağırlık: E = 10 kg, u(E) = 7,98 kg, ν_eff = 9, k = 2,32, U = 19 kg."),
   [T("u(I) = √(6.74² + 0.29² + 0.29² + 4.16² + 0.77²) = 7.97 kg; u(m_ref) = √(0.29² + 0.29² + 0.16²) = 0.44 kg; u(E) = 7.98 kg, ν_eff = 9.8 → 9, k = 2.32, U = 18.5 kg.",
      "u(I) = √(6,74² + 0,29² + 0,29² + 4,16² + 0,77²) = 7,97 kg; u(m_ref) = √(0,29² + 0,29² + 0,16²) = 0,44 kg; u(E) = 7,98 kg, ν_eff = 9,8 → 9, k = 2,32, U = 18,5 kg.")],
   None,
   T("Weighbridge 30 t: error at 10 t of standard weights (EURAMET cg-18 H3)", "Köprü kantar 30 t: 10 t standart ağırlıkta hata (EURAMET cg-18 H3)"),
   T("The 10 × 1000 kg standard weights on the load receptor; this load m_ref is also the reference for the substitution loads.", "Yük alıcısında 10 × 1000 kg standart ağırlık; bu yük m_ref, ikame yükleri için de referanstır."))
h3("h3-30t-15t-substitution", 15000, 15015, 5, 1,
   {"estimate": chk(15.0, 1e-9), "standard_uncertainty": chk(14.60, 0.02)},
   T("EURAMET cg-18 v4.0 H3.3/A, 15 000 kg = ½m_ref + L_sub1: E = 15 kg, u(E) = 14.60 kg (ν_eff = 109, k = 2.02, U = 29 kg; the coverage factor is not compared, see the notes).",
     "EURAMET cg-18 v4.0 H3.3/A, 15 000 kg = ½m_ref + L_ikame1: E = 15 kg, u(E) = 14,60 kg (ν_eff = 109, k = 2,02, U = 29 kg; kapsam faktörü karşılaştırılmamıştır, notlara bakınız)."),
   [SUBST_EXPL,
    T("u(L_sub1)² = u²(m_ref) + 2u²(I at 10 t) = 0.44² + 2·7.97²: u(L_sub1) = 11.28 kg. With u(I at 15 t) = 9.27 kg and 5 t of standard weights: u(E) = 14.60 kg.",
      "u(L_ikame1)² = u²(m_ref) + 2u²(10 t'da I) = 0,44² + 2·7,97²: u(L_ikame1) = 11,28 kg. 15 t'da u(I) = 9,27 kg ve 5 t standart ağırlıkla: u(E) = 14,60 kg."),
    T("With ν = 5 for each of the three repeatability terms, ν_eff = 37 and k = 2.07: U = 30.3 kg instead of 29 kg.",
      "Üç tekrarlanabilirlik teriminin her biri için ν = 5 ile ν_eff = 37 ve k = 2,07: U = 29 kg yerine 30,3 kg.")],
   [DOF3,
    T("Counting the three repeatability terms as one estimate of the same s (ν = 5), as Welch–Satterthwaite requires when the same s is used three times, gives ν_eff = 12 and k = 2.23 (U = 32.6 kg).",
      "Üç tekrarlanabilirlik terimini aynı s'nin tek bir tahmini (ν = 5) olarak saymak, Welch–Satterthwaite'in aynı s üç kez kullanıldığında gerektirdiği gibi, ν_eff = 12 ve k = 2,23 verir (U = 32,6 kg).")],
   T("Weighbridge 30 t: error at 15 t with a substitution load (EURAMET cg-18 H3)", "Köprü kantar 30 t: ikame yüküyle 15 t'da hata (EURAMET cg-18 H3)"),
   T("5 t of standard weights on top of the substitution load L_sub1 (trailer and containers, adjusted to read like the 10 t of standard weights).", "İkame yükü L_ikame1 (10 t standart ağırlıkla aynı göstergeyi verecek şekilde ayarlanmış römork ve konteynerler) üzerinde 5 t standart ağırlık."))
h3("h3-30t-max-substitution", 30010, 30040, 10, 2,
   {"estimate": chk(30.0, 1e-9), "standard_uncertainty": chk(23.85, 0.03), "coverage_factor": chk(2.00, 0.02), "expanded_uncertainty": chk(48, 0.5)},
   T("EURAMET cg-18 v4.0 H3.3/A, 30 010 kg = m_ref + L_sub1 + L_sub2: E = 30 kg, u(E) = 23.85 kg, ν_eff = 783, k = 2.00, U = 48 kg.", "EURAMET cg-18 v4.0 H3.3/A, 30 010 kg = m_ref + L_ikame1 + L_ikame2: E = 30 kg, u(E) = 23,85 kg, ν_eff = 783, k = 2,00, U = 48 kg."),
   [SUBST_EXPL,
    T("u(L_sub1 + L_sub2)² = 4u²(m_ref) + 2u²(I at 10 t) + 2u²(I at 20 t) = 4·0.44² + 2·7.97² + 2·10.82²: 19.02 kg; with u(I at 30 t) = 14.38 kg: u(E) = 23.85 kg.",
      "u(L_ikame1 + L_ikame2)² = 4u²(m_ref) + 2u²(10 t'da I) + 2u²(20 t'da I) = 4·0,44² + 2·7,97² + 2·10,82²: 19,02 kg; 30 t'da u(I) = 14,38 kg ile: u(E) = 23,85 kg."),
    T("The same standard weights enter three times (m_ref and twice inside the substitution loads); the model adds their errors linearly, the guide in quadrature (7.1.3-1c): 1.32 kg instead of 0.98 kg, without effect on u(E).",
      "Aynı standart ağırlıklar üç kez girer (m_ref ve ikame yüklerinin içinde iki kez); model hatalarını doğrusal, kılavuz karesel olarak toplar (7.1.3-1c): 0,98 kg yerine 1,32 kg; u(E)'ye etkisi yoktur."),
    T("E = 30 040 − 30 010 = 30 kg; u(E) = 23.86 kg, ν_eff = 157, k = 2.02, U = 48.1 kg.", "E = 30 040 − 30 010 = 30 kg; u(E) = 23,86 kg, ν_eff = 157, k = 2,02, U = 48,1 kg.")],
   [DOF3, REP24],
   T("Weighbridge 30 t: error at Max with two substitution loads (EURAMET cg-18 H3)", "Köprü kantar 30 t: iki ikame yüküyle Max'ta hata (EURAMET cg-18 H3)"),
   T("10 t of standard weights on top of the substitution loads L_sub1 + L_sub2; the largest U of the certificate.", "İkame yükleri L_ikame1 + L_ikame2 üzerinde 10 t standart ağırlık; sertifikadaki en büyük U."))
print("written")
