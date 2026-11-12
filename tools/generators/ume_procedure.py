"""Writes library projects for the two worked examples of the TÜBİTAK UME draft procedure "Belirsizlik
Hesapları Prosedürü" (İ. Ün, November 2026), Section 19. Only the numerical example data are used; Example 2 is
illustrative (the procedure says its values are not from a real measurement). Run from the repository root."""
import json
import math
from pathlib import Path

DOC = "TÜBİTAK UME, Belirsizlik Hesapları Prosedürü (draft, November 2026), author's own document"


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def write(field, pid, body, meta, expected, ref, analysis):
    section = meta.pop("section")
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "field": field, **meta, "sources": [{"document": DOC, "section": section}]},
         **body, "analysis": analysis, "expected": {**expected, "reference": ref}}
    # Kept outside library/: the procedure is an unpublished draft, so these projects are not verification
    # cases of the library (research log D112); they serve as laboratory templates and --lab examples.
    Path(f"laboratory/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


# --- Example 19.1: 10 V DC calibration of a digital multimeter ---------------------------------------------
uA = 27.9 / math.sqrt(12)
comps = [uA, 25.0, 40 / math.sqrt(3), 5 / math.sqrt(3)]
uc = math.sqrt(sum(v * v for v in comps))
nu = uc ** 4 / (uA ** 4 / 11)
assert round(uc, 1) == 35.1 and round(2 * uc, 1) == 70.2 and abs(nu - 4000) < 100
uN = math.sqrt(comps[0] ** 2 + comps[1] ** 2 + comps[3] ** 2)
assert round(uN, 1) == 26.4 and round(uN / comps[2], 2) == 1.14
write("electrical", "ume-procedure-19-1-dmm-10v-calibration",
      {"model": {"output": {"name": "E", "unit": "µV", "expression": "x_DMM + d_res - d_cal - d_drift",
                            "description": T("Error of indication of the multimeter at 10 V DC, (23 ± 1) °C", "Multimetrenin 10 V DC noktasındaki gösterge hatası, (23 ± 1) °C")},
                 "inputs": [{"name": "x_DMM", "unit": "µV", "description": T("Mean of 12 readings minus 10 V (10.000 212 V; s = 27.9 µV)", "12 okumanın ortalaması eksi 10 V (10,000 212 V; s = 27,9 µV)"),
                             "distribution": {"type": "t", "mean": 212, "scale": round(uA, 6), "dof": 11}},
                            {"name": "d_cal", "unit": "µV", "description": T("Calibrator certificate, U = 5 µV/V at 10 V (k = 2)", "Kalibratör sertifikası, 10 V'ta U = 5 µV/V (k = 2)"),
                             "distribution": {"type": "normal", "mean": 0, "expanded": 50, "k": 2}},
                            {"name": "d_drift", "unit": "µV", "description": T("Annual drift of the calibrator, ±40 µV", "Kalibratörün yıllık kayması, ±40 µV"),
                             "distribution": {"type": "rectangular", "value": 0, "half_width": 40}},
                            {"name": "d_res", "unit": "µV", "description": T("Resolution of the multimeter, 10 µV", "Multimetre çözünürlüğü, 10 µV"),
                             "distribution": {"type": "rectangular", "value": 0, "half_width": 5}}]}},
      {"title": T("10 V DC calibration of a digital multimeter (TÜBİTAK UME uncertainty procedure, Example 19.1)",
                  "Dijital multimetrenin 10 V DC kalibrasyonu (TÜBİTAK UME belirsizlik prosedürü, Örnek 19.1)"),
       "description": T("Additive model with absolute uncertainties: the error of indication E = V_DMM − V_ref with corrections for the calibrator certificate, its drift and the resolution.",
                        "Mutlak belirsizlikli toplamsal model: kalibratör sertifikası, kayması ve çözünürlük düzeltmeleriyle gösterge hatası E = V_DMM − V_ref."),
       "explanation": [T(f"u_c = √(8.1² + 25.0² + 23.1² + 2.9²) = {uc:.1f} µV; ν_eff = {nu:.0f} (Welch–Satterthwaite with ν = 11 for the readings); U = 2u_c = {2 * uc:.1f} µV.",
                         f"u_c = √(8,1² + 25,0² + 23,1² + 2,9²) = {uc:.1f} µV; ν_eff = {nu:.0f} (okumalar için ν = 11 ile Welch–Satterthwaite); U = 2u_c = {2 * uc:.1f} µV.".replace("µV.", "µV§").replace(".", ",").replace("§", ".")),
                       T(f"Dominant-component check of the procedure: the largest rectangular contribution u_R = 23.1 µV against the others u_N = {uN:.1f} µV; u_N/u_R = 1.14 > 0.70, so k = 2 holds.",
                         f"Prosedürün baskın bileşen kontrolü: en büyük dikdörtgen katkı u_R = 23,1 µV, diğerleri u_N = {str(round(uN, 1)).replace('.', ',')} µV; u_N/u_R = 1,14 > 0,70 olduğundan k = 2 uygundur."),
                       T("Result: E = (0.21 ± 0.07) mV, or U = 7 µV/V relative to 10 V (k = 2, p ≈ 95 %).", "Sonuç: E = (0,21 ± 0,07) mV; 10 V'a göre bağıl U = 7 µV/V (k = 2, p ≈ %95).")],
       "section": "Section 19.1", "tags": ["ume", "procedure", "dmm", "voltage", "calibration", "welch-satterthwaite"], "notes": []},
      {"gum": {"estimate": chk(212, 0.5), "standard_uncertainty": chk(35.1, 0.05), "expanded_uncertainty": chk(70.2, 0.05),
               "effective_dof": chk(4000, 100)}},
      T("UME procedure, Section 19.1: u_c = 35.1 µV, ν_eff ≈ 4000, U = 70.2 µV (k = 2); E = 0.21 mV.",
        "UME prosedürü, Bölüm 19.1: u_c = 35,1 µV, ν_eff ≈ 4000, U = 70,2 µV (k = 2); E = 0,21 mV."),
      {"gum": {"coverage_factor": 2}, "mcm": {"trials": 1000000, "seed": 1, "interval": "symmetric"}})

# --- Example 19.2: purity by ¹H qNMR with an internal standard (illustrative values) ----------------------
R, Pstd, mstd, ma, Ma, Mstd = 1.40023, 999.4, 6.512, 10.215, 194.19, 116.072
Na, Nstd = 3, 2  # not printed (the model equation is an embedded object); only 2/3 gives the printed 995.0
Pa = R * (Nstd / Na) * (Ma / Mstd) * (mstd / ma) * Pstd
rel = [0.0015 / math.sqrt(5), 0.002 / math.sqrt(3), 0.75 / Pstd, 0.002 / mstd, 0.002 / ma, 21e-6, 17e-6]
ur = math.sqrt(sum(v * v for v in rel))
nu2 = ur ** 4 / (rel[0] ** 4 / 4)
assert round(Pa, 1) == 995.0 and round(1e6 * ur) == 1575 and round(nu2) == 121 and round(2 * Pa * ur, 1) == 3.1
write("chemistry", "ume-procedure-19-2-qnmr-purity-internal-standard",
      {"model": {"output": {"name": "P_a", "unit": "mg/g", "expression": "R * f_int * (N_std/N_a) * (M_a/M_std) * (m_std/m_a) * P_std",
                            "description": T("Purity (mass fraction) of the analyte by ¹H qNMR", "¹H qNMR ile analit saflığı (kütle kesri)")},
                 "inputs": [{"name": "R", "unit": "1", "description": T("Integral ratio I_a/I_std, 5 independent samples, RSD 0.15 % (Type A)", "İntegral oranı I_a/I_std, 5 bağımsız numune, %RSD 0,15 (A tipi)"),
                             "distribution": {"type": "t", "mean": R, "scale": round(R * rel[0], 9), "dof": 4}},
                            {"name": "f_int", "unit": "1", "description": T("Integration (phase, baseline, limits), ±0.2 %", "Entegrasyon (faz, taban çizgisi, sınırlar), ±%0,2"),
                             "distribution": {"type": "rectangular", "value": 1, "half_width": 0.002}},
                            {"name": "N_std", "unit": "1", "description": T("¹H nuclei of the internal-standard signal (2, e.g. maleic acid)", "İç standart sinyalinin ¹H çekirdek sayısı (2, ör. maleik asit)"),
                             "distribution": {"type": "constant", "value": Nstd}},
                            {"name": "N_a", "unit": "1", "description": T("¹H nuclei of the analyte signal (3, a methyl singlet)", "Analit sinyalinin ¹H çekirdek sayısı (3, bir metil singleti)"),
                             "distribution": {"type": "constant", "value": Na}},
                            {"name": "M_a", "unit": "g/mol", "description": T("Molar mass of the analyte (IUPAC atomic weights)", "Analitin mol kütlesi (IUPAC atom ağırlıkları)"),
                             "distribution": {"type": "rectangular", "value": Ma, "half_width": round(21e-6 * Ma * math.sqrt(3), 7)}},
                            {"name": "M_std", "unit": "g/mol", "description": T("Molar mass of the internal standard", "İç standardın mol kütlesi"),
                             "distribution": {"type": "rectangular", "value": Mstd, "half_width": round(17e-6 * Mstd * math.sqrt(3), 7)}},
                            {"name": "m_std", "unit": "mg", "description": T("Mass of the internal standard, microbalance u = 2.0 µg", "İç standart kütlesi, mikroterazi u = 2,0 µg"),
                             "distribution": {"type": "normal", "mean": mstd, "sd": 0.002}},
                            {"name": "m_a", "unit": "mg", "description": T("Mass of the analyte, microbalance u = 2.0 µg", "Analit kütlesi, mikroterazi u = 2,0 µg"),
                             "distribution": {"type": "normal", "mean": ma, "sd": 0.002}},
                            {"name": "P_std", "unit": "mg/g", "description": T("Purity of the internal-standard CRM, U = 1.5 mg/g (k = 2)", "İç standart CRM saflığı, U = 1,5 mg/g (k = 2)"),
                             "distribution": {"type": "normal", "mean": Pstd, "expanded": 1.5, "k": 2}}]}},
      {"title": T("Purity by ¹H qNMR with an internal standard: an illustrative budget (TÜBİTAK UME uncertainty procedure, Example 19.2)",
                  "İç standartla ¹H qNMR saflık tayini: örnek bir bütçe (TÜBİTAK UME belirsizlik prosedürü, Örnek 19.2)"),
       "description": T("Multiplicative model with relative uncertainties; the values are illustrative, not from a real measurement. The integration is a separate Type B component (±0.2 %, rectangular) beside the repeatability of the ratio.",
                        "Bağıl belirsizlikli çarpımsal model; değerler örnektir, gerçek bir ölçüme ait değildir. Entegrasyon, oranın tekrarlanabilirliğinin yanında ayrı bir B tipi bileşendir (±%0,2, dikdörtgen)."),
       "explanation": [T(f"Relative standard uncertainties (10⁻⁶): repeatability 671, integration 1155, P_std 750, m_std 307, m_a 196, M_a 21, M_std 17; combined {1e6 * ur:.0f}; ν_eff = {nu2:.0f}.",
                         f"Bağıl standart belirsizlikler (10⁻⁶): tekrarlanabilirlik 671, entegrasyon 1155, P_std 750, m_std 307, m_a 196, M_a 21, M_std 17; birleşik {1e6 * ur:.0f}; ν_eff = {nu2:.0f}."),
                       T(f"P_a = {Pa:.1f} mg/g, u_c = {Pa * ur:.3f} mg/g, U = 2u_c = {2 * Pa * ur:.2f} → 3.1 mg/g: P_a = (995.0 ± 3.1) mg/g (k = 2).",
                         f"P_a = {Pa:.1f} mg/g, u_c = {Pa * ur:.3f} mg/g, U = 2u_c = {2 * Pa * ur:.2f} → 3,1 mg/g: P_a = (995,0 ± 3,1) mg/g (k = 2).".replace("995.0 mg", "995,0 mg").replace(f"{Pa * ur:.3f}", f"{Pa * ur:.3f}".replace(".", ",")).replace(f"{2 * Pa * ur:.2f}", f"{2 * Pa * ur:.2f}".replace(".", ","))),
                       T("More than half of the variance comes from the integration and about a quarter from the purity of the internal standard (54 % and 23 %).",
                         "Varyansın yarısından fazlası entegrasyondan, yaklaşık dörtte biri iç standardın saflığından gelir (%54 ve %23).")],
       "section": "Section 19.2", "tags": ["ume", "procedure", "qnmr", "purity", "relative-uncertainty"],
       "notes": [T("The numbers of ¹H nuclei are not legible in the procedure (the model is an embedded equation object); N_std/N_a = 2/3 is the only ratio that reproduces the printed 995.0 mg/g, consistent with maleic acid (M = 116.072 g/mol, 2 H) as internal standard and an analyte of M = 194.19 g/mol with a methyl singlet (e.g. caffeine).",
                   "¹H çekirdek sayıları prosedürde okunamıyor (model gömülü bir denklem nesnesidir); basılı 995,0 mg/g değerini veren tek oran N_std/N_a = 2/3'tür; iç standart maleik asit (M = 116,072 g/mol, 2 H) ve bir metil singleti olan M = 194,19 g/mol'lük bir analit (ör. kafein) ile uyumludur.")]},
      {"gum": {"estimate": chk(995.0, 0.05), "standard_uncertainty": chk(1.567, 0.001), "expanded_uncertainty": chk(3.13, 0.005),
               "effective_dof": chk(121, 1)}},
      T("UME procedure, Section 19.2: P_a = 995.0 mg/g, relative u_c = 0.001 575, u_c = 1.567 mg/g, ν_eff = 121, U = 3.13 mg/g → (995.0 ± 3.1) mg/g (k = 2).",
        "UME prosedürü, Bölüm 19.2: P_a = 995,0 mg/g, bağıl u_c = 0,001 575, u_c = 1,567 mg/g, ν_eff = 121, U = 3,13 mg/g → (995,0 ± 3,1) mg/g (k = 2)."),
      {"gum": {"coverage_factor": 2}, "mcm": None})
