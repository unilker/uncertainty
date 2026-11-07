"""Writes the library projects of Possolo, Hibbert, Stohner, Bodnar & Meija, "A brief guide to measurement
uncertainty (IUPAC Technical Report)", Pure Appl. Chem. 96 (2024) 113–134 (run from the repository root).
Printed values are recomputed and asserted.
"""
import json
import math
from pathlib import Path

DOC = "Possolo et al., A brief guide to measurement uncertainty (IUPAC Technical Report), Pure Appl. Chem. 96 (2024) 113–134"
URL = "https://doi.org/10.1515/pac-2022-1203"
R6 = math.sqrt(6)


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


def write(field, pid, body, meta, expected, ref, analysis=None):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "field": field, **meta, "sources": [{"document": DOC, "section": meta.pop("section", ""), "url": URL}]},
         **body}
    if analysis is not None:
        p["analysis"] = analysis
    p["expected"] = {**expected, "reference": ref}
    p["metadata"].pop("section", None)
    Path(f"library/{field}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


# --- Example 3C/4A: total hardness of water ------------------------------------------------------------
M_CaCO3, M_Ca, M_Mg = 100.0869, 40.078, 24.305
c1, c2 = M_CaCO3 / M_Ca, M_CaCO3 / M_Mg
H = 23.2 * c1 + 9.4 * c2
uH = math.hypot(c1 * 0.8, c2 * 0.6)
assert round(c1, 1) == 2.5 and round(c2, 1) == 4.1 and round(uH) == 3
write("chemistry", "iupac2024-4a-water-hardness",
      {"model": {"output": {"name": "H", "unit": "mg/L", "expression": "g_Ca*M_CaCO3/M_Ca + g_Mg*M_CaCO3/M_Mg",
                            "description": T("Total hardness of water as mass concentration of CaCO₃", "CaCO₃ kütle derişimi olarak suyun toplam sertliği")},
                 "inputs": [inp("g_Ca", {"type": "normal", "mean": 23.2, "sd": 0.8}, "mg/L", "Mass concentration of Ca²⁺ (AAS)", "Ca²⁺ kütle derişimi (AAS)"),
                            inp("g_Mg", {"type": "normal", "mean": 9.4, "sd": 0.6}, "mg/L", "Mass concentration of Mg²⁺ (AAS, measured separately)", "Mg²⁺ kütle derişimi (AAS, ayrı ölçülmüş)"),
                            inp("M_CaCO3", {"type": "constant", "value": M_CaCO3}, "g/mol"), inp("M_Ca", {"type": "constant", "value": M_Ca}, "g/mol"),
                            inp("M_Mg", {"type": "constant", "value": M_Mg}, "g/mol")]}},
      {"title": T("Total hardness of water from Ca²⁺ and Mg²⁺: a conventional measurement model (IUPAC 2024, Examples 3C and 4A)",
                  "Ca²⁺ ve Mg²⁺'dan suyun toplam sertliği: geleneksel bir ölçüm modeli (IUPAC 2024, Örnek 3C ve 4A)"),
       "description": T("H = γ(Ca²⁺)·M(CaCO₃)/M(Ca) + γ(Mg²⁺)·M(CaCO₃)/M(Mg), defined by convention; Gauss's formula with independent inputs.",
                        "H = γ(Ca²⁺)·M(CaCO₃)/M(Ca) + γ(Mg²⁺)·M(CaCO₃)/M(Mg), uzlaşımla tanımlı; bağımsız girdilerle Gauss formülü."),
       "explanation": [T(f"Sensitivity coefficients M(CaCO₃)/M(Ca) = {c1:.4f} and M(CaCO₃)/M(Mg) = {c2:.4f} (the guide rounds to 2.5 and 4.1); H = {H:.2f} mg/L, u(H) = {uH:.2f} mg/L ≈ 3 mg/L. The molar masses are taken as exact.",
                         f"Duyarlılık katsayıları M(CaCO₃)/M(Ca) = {c(f'{c1:.4f}')} ve M(CaCO₃)/M(Mg) = {c(f'{c2:.4f}')} (rehber 2,5 ve 4,1'e yuvarlar); H = {c(f'{H:.2f}')} mg/L, u(H) = {c(f'{uH:.2f}')} mg/L ≈ 3 mg/L. Mol kütleleri kesin alınır."),
                       T("The definition of the measurand fixes the measurement function (a conventional model, 3C); Ca and Mg determined separately by AAS justify independence (4A).",
                         "Ölçülenin tanımı ölçüm fonksiyonunu belirler (geleneksel model, 3C); Ca ve Mg'nin AAS ile ayrı tayini bağımsızlığı gerekçelendirir (4A).")],
       "section": "Examples 3C and 4A", "tags": ["iupac-2024", "water", "hardness", "linear", "conventional-model"], "notes": []},
      {"gum": {"estimate": chk(round(H, 2), 0.01), "standard_uncertainty": chk(3.0, 0.2)}},
      T("IUPAC 2024, Example 4A: u²(H) = 2.5² × 0.8² + 4.1² × 0.6² ≈ (3 mg/L)².", "IUPAC 2024, Örnek 4A: u²(H) = 2,5² × 0,8² + 4,1² × 0,6² ≈ (3 mg/L)²."))

# --- Example 4B: isotopic abundances of silicon from two isotope ratios ----------------------------------
R29, u29, R30, u30 = 0.0500657, 0.0000025, 0.0329035, 0.0000034
S = 1 + R29 + R30
x28, x29, x30 = 1 / S, R29 / S, R30 / S
J = [[-x28 ** 2, -x28 ** 2], [x28 ** 2 * (1 + R30), -x28 ** 2 * R29], [-x28 ** 2 * R30, x28 ** 2 * (1 + R29)]]
ux = [math.hypot(J[i][0] * u29, J[i][1] * u30) for i in range(3)]
assert [round(v, 7) for v in (x28, x29, x30)] == [0.9233873, 0.0462300, 0.0303827]
assert [round(1e7 * v) for v in ux] == [36, 22, 30]
r_28_30 = (J[0][0] * J[2][0] * u29 ** 2 + J[0][1] * J[2][1] * u30 ** 2) / (ux[0] * ux[2])
r_28_29 = (J[0][0] * J[1][0] * u29 ** 2 + J[0][1] * J[1][1] * u30 ** 2) / (ux[0] * ux[1])
write("chemistry", "iupac2024-4b-silicon-isotopic-abundances",
      {"multivariate": {"outputs": [{"name": "x28", "unit": "mol/mol", "expression": "1/(1 + R29 + R30)"},
                                    {"name": "x29", "unit": "mol/mol", "expression": "R29/(1 + R29 + R30)"},
                                    {"name": "x30", "unit": "mol/mol", "expression": "R30/(1 + R29 + R30)"}],
                        "inputs": [inp("R29", {"type": "normal", "mean": R29, "sd": u29}, "mol/mol", "Amount ratio n(²⁹Si)/n(²⁸Si), Mixture III", "Miktar oranı n(²⁹Si)/n(²⁸Si), Karışım III"),
                                   inp("R30", {"type": "normal", "mean": R30, "sd": u30}, "mol/mol", "Amount ratio n(³⁰Si)/n(²⁸Si), Mixture III", "Miktar oranı n(³⁰Si)/n(²⁸Si), Karışım III")]},
       "analysis": {"gum": {}, "mcm": None}},
      {"title": T("Isotopic abundances of silicon from two measured isotope amount ratios (IUPAC 2024, Example 4B)",
                  "Ölçülen iki izotop miktar oranından silisyumun izotop bollukları (IUPAC 2024, Örnek 4B)"),
       "description": T("x₂₈ = 1/(1 + R₂₉/₂₈ + R₃₀/₂₈), x₂₉ = R₂₉/₂₈·x₂₈, x₃₀ = R₃₀/₂₈·x₂₈: three outputs from two inputs, with the covariance matrix Σ_y = JΣ_xJᵀ (Formula 3).",
                        "x₂₈ = 1/(1 + R₂₉/₂₈ + R₃₀/₂₈), x₂₉ = R₂₉/₂₈·x₂₈, x₃₀ = R₃₀/₂₈·x₂₈: iki girdiden üç çıktı; kovaryans matrisi Σ_y = JΣ_xJᵀ (Formül 3)."),
       "explanation": [T("Valkiers et al., Mixture III: R₂₉/₂₈ = 0.050 0657(25), R₃₀/₂₈ = 0.032 9035(34) mol/mol, taken as uncorrelated. The Jacobian is x₂₈²·[[−1, −1], [1 + R₃₀/₂₈, −R₂₉/₂₈], [−R₃₀/₂₈, 1 + R₂₉/₂₈]].",
                         "Valkiers vd., Karışım III: R₂₉/₂₈ = 0,050 0657(25), R₃₀/₂₈ = 0,032 9035(34) mol/mol, ilişkisiz alınmıştır. Jakobiyen x₂₈²·[[−1, −1], [1 + R₃₀/₂₈, −R₂₉/₂₈], [−R₃₀/₂₈, 1 + R₂₉/₂₈]]'dir."),
                       T(f"x₂₈ = {x28:.7f}(36), x₂₉ = {x29:.7f}(22), x₃₀ = {x30:.7f}(30) mol/mol. The abundances sum to 1, so their covariance matrix is singular; r(x₂₈, x₂₉) = {r_28_29:.2f}, r(x₂₈, x₃₀) = {r_28_30:.2f}.",
                         f"x₂₈ = {c(f'{x28:.7f}')}(36), x₂₉ = {c(f'{x29:.7f}')}(22), x₃₀ = {c(f'{x30:.7f}')}(30) mol/mol. Bolluklar toplamı 1 olduğundan kovaryans matrisi tekildir; r(x₂₈, x₂₉) = {c(f'{r_28_29:.2f}')}, r(x₂₈, x₃₀) = {c(f'{r_28_30:.2f}')}.")],
       "section": "Example 4B", "tags": ["iupac-2024", "isotope-ratio", "silicon", "multivariate", "jcgm102", "atomic-weight"], "notes": []},
      {"multivariate": {"gum": {"estimate": {"x28": chk(0.9233873, 5e-8), "x29": chk(0.0462300, 5e-8), "x30": chk(0.0303827, 5e-8)},
                                "standard_uncertainty": {"x28": chk(3.6e-6, 5e-8), "x29": chk(2.2e-6, 5e-8), "x30": chk(3.0e-6, 5e-8)},
                                "correlations": [{"between": ["x28", "x29"], "value": round(r_28_29, 3), "tolerance": 0.001},
                                                 {"between": ["x28", "x30"], "value": round(r_28_30, 3), "tolerance": 0.001}]}}},
      T("IUPAC 2024, Example 4B: x₂₈ = 0.923 3873(36), x₂₉ = 0.046 2300(22), x₃₀ = 0.030 3827(30) mol/mol.",
        "IUPAC 2024, Örnek 4B: x₂₈ = 0,923 3873(36), x₂₉ = 0,046 2300(22), x₃₀ = 0,030 3827(30) mol/mol."))

# --- Example 4I: argentometric titration (Mohr), conventional and solubility-product models ----------------
uVN, uVA, uVE = 0.08 / R6, 0.01 / R6, 0.025
uc = 10 * math.sqrt((uVN / 100) ** 2 + (uVA / 10) ** 2 + (uVE / 10) ** 2)
assert round(uc, 3) == 0.026
Ksp1, Ksp2 = 10 ** -9.75, 10 ** -11.9
crom = 1.75e-3 * 100 / 110
ag = math.sqrt(Ksp2 / crom)
excess = 1000 * (ag - Ksp1 / ag)            # mmol/L of Ag⁺ in excess at the endpoint
c_printed = (100 * 10 + excess * 110) / 100  # sign of the printed equation
c_balance = (100 * 10 - excess * 110) / 100  # mass balance: Ag added − Cl present = [Ag⁺] − [Cl⁻]
assert round(100 * (c_printed / 10 - 1), 2) == 0.24 and round(100 * (c_balance / 10 - 1), 2) == -0.24
ins4i = [inp("c_Ag", {"type": "constant", "value": 100.0}, "mmol/L", "Concentration of the AgNO₃ titrant (taken as exact)", "AgNO₃ titrantının derişimi (kesin alınmıştır)"),
         inp("V_Ag", {"type": "constant", "value": 10.0}, "mL", "Volume of titrant read at the endpoint", "Dönüm noktasında okunan titrant hacmi"),
         inp("dV_Ag", {"type": "triangular", "value": 0.0, "half_width": 0.01}, "mL", "Resolution of the electronic burette, ±0.01 mL", "Elektronik büretin çözünürlüğü, ±0,01 mL"),
         inp("dV_E", {"type": "normal", "mean": 0.0, "sd": 0.025}, "mL", "Visual endpoint detection, one droplet at 95 %", "Görsel dönüm noktası tespiti, %95'te bir damla"),
         inp("V_NaCl", {"type": "constant", "value": 100.0}, "mL", "Nominal volume of the sample", "Numunenin nominal hacmi"),
         inp("dV_NaCl", {"type": "triangular", "value": 0.0, "half_width": 0.08}, "mL", "Class A volumetric pipette, triangular with base 0.16 mL", "A sınıfı tam pipet, taban genişliği 0,16 mL üçgen")]
meta4i = {"section": "Examples 3A and 4I", "tags": ["iupac-2024", "titration", "argentometry", "mohr", "volumetric"]}
write("chemistry", "iupac2024-4i-argentometric-titration",
      {"model": {"output": {"name": "c_NaCl", "unit": "mmol/L", "expression": "c_Ag*(V_Ag + dV_Ag + dV_E)/(V_NaCl + dV_NaCl)",
                            "description": T("Amount concentration of NaCl", "NaCl miktar derişimi")}, "inputs": ins4i}},
      {**meta4i, "title": T("Amount concentration of NaCl by argentometric titration (Mohr): conventional model (IUPAC 2024, Example 4I)",
                            "Arjantometrik titrasyonla (Mohr) NaCl miktar derişimi: klasik model (IUPAC 2024, Örnek 4I)"),
       "description": T("c(NaCl) = c(AgNO₃)·(V(AgNO₃) + ΔV_V + ΔV_E)/(V(NaCl) + ΔV_V); only the volumetric and endpoint uncertainties are considered.",
                        "c(NaCl) = c(AgNO₃)·(V(AgNO₃) + ΔV_V + ΔV_E)/(V(NaCl) + ΔV_V); yalnızca hacim ve dönüm noktası belirsizlikleri dikkate alınır."),
       "explanation": [T(f"Pipette: triangular, s.d. 0.08/√6 = {uVN:.4f} mL; burette: triangular ±0.01 mL, s.d. {uVA:.5f} mL; endpoint: normal, s.d. 0.025 mL, which dominates. c(NaCl) = 10.000 mmol/L, u = {uc:.4f} mmol/L (0.26 %).",
                         f"Pipet: üçgen, s.s. 0,08/√6 = {c(f'{uVN:.4f}')} mL; büret: üçgen ±0,01 mL, s.s. {c(f'{uVA:.5f}')} mL; dönüm noktası: normal, s.s. 0,025 mL, baskın. c(NaCl) = 10,000 mmol/L, u = {c(f'{uc:.4f}')} mmol/L (%0,26).")],
       "notes": [T("The endpoint (first permanent Ag₂CrO₄) lies after the equivalence point; the companion project iupac2024-3a-argentometric-titration-ksp models this chemistry instead of treating it as an uncertainty.",
                   "Dönüm noktası (ilk kalıcı Ag₂CrO₄) eşdeğerlik noktasından sonradır; eşlik eden iupac2024-3a-argentometric-titration-ksp projesi bu kimyayı belirsizlik olarak ele almak yerine modeller.")]},
      {"gum": {"estimate": chk(10.0, 1e-9), "standard_uncertainty": chk(0.026, 0.0006)}, "mcm": {"estimate": chk(10.0, 0.0002), "standard_uncertainty": chk(0.0255, 0.0002)}},
      T("IUPAC 2024, Example 4I: c(NaCl) = 10.000 mmol/L, u = 0.026 mmol/L (0.26 %).", "IUPAC 2024, Örnek 4I: c(NaCl) = 10,000 mmol/L, u = 0,026 mmol/L (%0,26)."),
      analysis={"gum": {}, "mcm": {"trials": 1000000, "seed": 5, "interval": "symmetric"}})
VE = "(V_Ag + dV_Ag + dV_E)"
VN = "(V_NaCl + dV_NaCl)"
AGexpr = f"sqrt(Ksp2*({VN} + {VE})/(c_K*{VN}))"
eq = f"1000*({AGexpr} - Ksp1/{AGexpr}) - (c_Ag*{VE} - c_NaCl*{VN})/({VN} + {VE})"
ins3a = ins4i + [inp("c_K", {"type": "constant", "value": 1.75e-3}, "mol/L", "Concentration of the K₂CrO₄ indicator in the sample before titration", "Titrasyondan önce numunedeki K₂CrO₄ indikatörünün derişimi"),
                 inp("Ksp1", {"type": "constant", "value": Ksp1}, "(mol/L)²", "Solubility product of AgCl, lg K = −9.75", "AgCl çözünürlük çarpımı, lg K = −9,75"),
                 inp("Ksp2", {"type": "constant", "value": Ksp2}, "(mol/L)³", "Solubility product of Ag₂CrO₄, lg K = −11.9", "Ag₂CrO₄ çözünürlük çarpımı, lg K = −11,9")]
write("chemistry", "iupac2024-3a-argentometric-titration-ksp",
      {"multivariate": {"implicit": True, "outputs": [{"name": "c_NaCl", "unit": "mmol/L", "expression": eq, "start": 10.0}], "inputs": ins3a},
       "analysis": {"gum": {}, "mcm": None}},
      {**meta4i, "section": "Examples 3A and 4I; Meija et al., Eq. (19)",
       "title": T("Argentometric titration (Mohr) with the endpoint chemistry: implicit model with the solubility products of AgCl and Ag₂CrO₄ (IUPAC 2024, Examples 3A and 4I)",
                  "Dönüm noktası kimyasıyla arjantometrik titrasyon (Mohr): AgCl ve Ag₂CrO₄ çözünürlük çarpımlı örtük model (IUPAC 2024, Örnek 3A ve 4I)"),
       "description": T("At the endpoint [Ag⁺] = √(K_sp2/[CrO₄²⁻]) and [Cl⁻] = K_sp1/[Ag⁺]; the mass balance of silver and chloride gives an equation for c(NaCl) that is solved numerically.",
                        "Dönüm noktasında [Ag⁺] = √(K_sp2/[CrO₄²⁻]) ve [Cl⁻] = K_sp1/[Ag⁺]; gümüş ve klorürün kütle dengesi c(NaCl) için sayısal olarak çözülen bir denklem verir."),
       "explanation": [T(f"With c(K₂CrO₄) = 1.75 mmol/L diluted to 110 mL, [Ag⁺] = {1e6 * ag:.2f} µmol/L and [Cl⁻] = {1e6 * Ksp1 / ag:.2f} µmol/L at the endpoint: the excess silver {1000 * excess:.1f} µmol/L × 110 mL must be subtracted from the titrant.",
                         f"c(K₂CrO₄) = 1,75 mmol/L, 110 mL'ye seyreltilmiş; dönüm noktasında [Ag⁺] = {c(f'{1e6 * ag:.2f}')} µmol/L ve [Cl⁻] = {c(f'{1e6 * Ksp1 / ag:.2f}')} µmol/L: fazla gümüş, {c(f'{1000 * excess:.1f}')} µmol/L × 110 mL, titranttan düşülmelidir."),
                       T(f"(c(AgNO₃)·V_E − c(NaCl)·V)/(V + V_E) = [Ag⁺] − [Cl⁻] gives c(NaCl) = {c_balance:.4f} mmol/L, 0.24 % below the conventional 10.000 mmol/L; the uncertainty from the volumes is unchanged (0.026 mmol/L).",
                         f"(c(AgNO₃)·V_E − c(NaCl)·V)/(V + V_E) = [Ag⁺] − [Cl⁻], c(NaCl) = {c(f'{c_balance:.4f}')} mmol/L verir; klasik 10,000 mmol/L'nin %0,24 altında; hacimlerden gelen belirsizlik değişmez (0,026 mmol/L).")],
       "notes": [T("The equation printed in Example 4I has the right-hand side (c(NaCl)V(NaCl) − c(AgNO₃)V_E)/(V + V_E) with a positive left-hand side [Ag⁺] − [Cl⁻]; solved as printed it gives 10.024 mmol/L, 0.24 % above the conventional value. Since the indicator responds only after the equivalence point, the endpoint volume overestimates the chloride and the mass balance gives 9.976 mmol/L; the guide states the magnitude, 0.24 %, but not its sign (research log F92).",
                   "Örnek 4I'da basılan denklemin sağ tarafı (c(NaCl)V(NaCl) − c(AgNO₃)V_E)/(V + V_E), sol tarafı ise pozitif [Ag⁺] − [Cl⁻]'dir; basıldığı gibi çözülünce klasik değerin %0,24 üstünde 10,024 mmol/L verir. İndikatör ancak eşdeğerlik noktasından sonra tepki verdiğinden dönüm noktası hacmi klorürü fazla gösterir ve kütle dengesi 9,976 mmol/L verir; rehber büyüklüğü (%0,24) verir, işaretini vermez (araştırma günlüğü F92).")]},
      {"multivariate": {"gum": {"estimate": {"c_NaCl": chk(round(c_balance, 4), 2e-4)}, "standard_uncertainty": {"c_NaCl": chk(0.0255, 0.0003)}}}},
      T("IUPAC 2024, Example 4I (after Meija et al., Eq. 19): relative difference from the conventional estimate 0.24 %; lg K_sp1 = −9.75, lg K_sp2 = −11.9.",
        "IUPAC 2024, Örnek 4I (Meija vd., Eş. 19'dan): klasik tahminden bağıl fark %0,24; lg K_sp1 = −9,75, lg K_sp2 = −11,9."))

# --- ILAC P14:09/2020, 5.5: the CMC as a floor for the reported uncertainty (on EA-4/02 S2) ----------------
base = json.loads(Path("library/mass/ea402-s2-weight-10kg.json").read_text(encoding="utf-8"))
p = json.loads(json.dumps(base))
p["metadata"]["id"] = "ilac-p14-cmc-floor-weight-10kg"
p["metadata"]["title"] = T("Reporting rule of ILAC P14 5.5: the CMC as the smallest reportable uncertainty (EA-4/02 S2 weight, illustrative CMC)",
                           "ILAC P14 5.5 raporlama kuralı: raporlanabilecek en küçük belirsizlik olarak CMC (EA-4/02 S2 kütle, örnek CMC)")
p["metadata"]["description"] = T("The 10 kg weight of EA-4/02 S2 (U = 58 mg) reported by a laboratory whose accredited CMC is taken, for illustration, as 80 mg: the reported U is the CMC, not the evaluated 58 mg.",
                                 "EA-4/02 S2'deki 10 kg kütle (U = 58 mg); akredite CMC'si örnek olarak 80 mg alınan bir laboratuvar tarafından raporlanır: raporlanan U, değerlendirilen 58 mg değil CMC'dir.")
p["metadata"]["explanation"] = list(base["metadata"].get("explanation", [])) + [
    T("ILAC P14:09/2020, 5.5: an accredited calibration laboratory shall not report a smaller measurement uncertainty than the uncertainty described by its CMC. With reporting.cmc the program reports max(U, CMC) and says so in the result statement; 5.3 limits U to at most two significant digits.",
      "ILAC P14:09/2020, 5.5: akredite bir kalibrasyon laboratuvarı CMC'sinin tanımladığından küçük bir ölçüm belirsizliği raporlayamaz. reporting.cmc ile program max(U, CMC) raporlar ve bunu sonuç ifadesinde belirtir; 5.3, U'yu en çok iki anlamlı basamakla sınırlar.")]
p["metadata"]["sources"] = base["metadata"]["sources"] + [{"document": "ILAC P14:09/2020, ILAC Policy for Measurement Uncertainty in Calibration", "section": "5.3, 5.5", "url": "https://ilac.org/publications-and-resources/ilac-policy-series/"}]
p["metadata"]["tags"] = sorted(set(base["metadata"]["tags"] + ["ilac-p14", "cmc", "reporting"]))
p["metadata"]["notes"] = [T("The CMC of 80 mg is a hypothetical value chosen to exceed the evaluated U; it is not taken from a scope of accreditation.",
                            "80 mg'lık CMC, değerlendirilen U'yu aşacak biçimde seçilmiş varsayımsal bir değerdir; bir akreditasyon kapsamından alınmamıştır.")]
p["reporting"] = {**base.get("reporting", {}), "cmc": {"expanded": 80.0}}
p["expected"] = {"gum": {**base["expected"]["gum"], "reported_expanded_uncertainty": chk(80.0, 1e-9)},
                 "reference": T("EA-4/02 M:2022 S2 (U = 58 mg) and ILAC P14:09/2020 5.5; the CMC is illustrative.",
                                "EA-4/02 M:2022 S2 (U = 58 mg) ve ILAC P14:09/2020 5.5; CMC örnek amaçlıdır.")}
Path("library/mass/ilac-p14-cmc-floor-weight-10kg.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("wrote ilac-p14-cmc-floor-weight-10kg")
