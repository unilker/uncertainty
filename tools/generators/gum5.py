"""Writes the JCGM GUM-5:2026 library projects (run from the repository root).

JCGM GUM-5:2026, Guide to the expression of uncertainty in measurement — Part 5: Examples,
doi:10.59161/YNLY8209. Examples 6, 7, 8, 13 and 16 are the JCGM 101/102 examples already in the library
(GUM-5 is added there as a source); this script writes the other examples. Expected values are the printed
ones; every one is recomputed here and the agreement asserted.
"""
import json
import math
import statistics as st
from pathlib import Path

DOC = "JCGM GUM-5:2026, Guide to the expression of uncertainty in measurement — Part 5: Examples"
URL = "https://doi.org/10.59161/YNLY8209"
R3 = math.sqrt(3)


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


def write(folder, pid, title, desc, expl, section, tags, model, expected, ref, notes=(), analysis=None, reporting=None, field=None):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": field or folder,
                      "sources": [{"document": DOC, "section": section, "url": URL}], "tags": ["gum-5"] + tags,
                      "notes": list(notes)},
         "model": model,
         "analysis": analysis or {"gum": {}, "mcm": {"trials": 1000000, "seed": 5, "interval": "symmetric"}},
         "expected": {**expected, "reference": ref}}
    if reporting:
        p["reporting"] = reporting
    Path(f"library/{folder}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


def inp(name, dist, unit=None, en=None, tr=None):
    q = {"name": name}
    if unit:
        q["unit"] = unit
    if en:
        q["description"] = T(en, tr)
    q["distribution"] = dist
    return q


# --- Example 3: relative molecular mass of glucose -------------------------------------------------------
W = {"H": (1.00784, 1.00811, 12), "C": (12.0096, 12.0116, 6), "O": (15.99903, 15.99977, 6)}
est = sum(n * (a + b) / 2 for a, b, n in W.values())
u = math.sqrt(sum((n * (b - a) / (2 * R3)) ** 2 for a, b, n in W.values()))
assert abs(est - 180.1557) < 5e-5 and abs(u - 0.0038) < 5e-5
contrib = {k: n * (b - a) / (2 * R3) for k, (a, b, n) in W.items()}
write("chemistry", "gum5-e3-molar-mass-glucose",
      T("Relative molecular mass of glucose from standard atomic weight intervals (JCGM GUM-5 Example 3)",
        "Standart atom ağırlığı aralıklarından glikozun bağıl molekül kütlesi (JCGM GUM-5 Örnek 3)"),
      T("M_r(C₆H₁₂O₆) = 6A_C + 12A_H + 6A_O with rectangular distributions over the IUPAC 2021 intervals of the standard atomic weights.",
        "IUPAC 2021 standart atom ağırlığı aralıkları üzerinde dikdörtgen dağılımlarla M_r(C₆H₁₂O₆) = 6A_C + 12A_H + 6A_O."),
      [T("The isotopic composition of the glucose is unknown; for a normal material the standard atomic weights lie in the intervals [1.007 84, 1.008 11] (H), [12.0096, 12.0116] (C) and [15.999 03, 15.999 77] (O), to which rectangular distributions are assigned (GUM 4.3.7).",
         "Glikozun izotop bileşimi bilinmemektedir; normal bir malzeme için standart atom ağırlıkları [1,007 84; 1,008 11] (H), [12,0096; 12,0116] (C) ve [15,999 03; 15,999 77] (O) aralıklarındadır; bunlara dikdörtgen dağılım atanır (GUM 4.3.7)."),
       T(f"The model is linear with exact coefficients: u(M_r) = √Σν_i²u²(A_i); the contributions are {contrib['H']:.5f} (H), {contrib['C']:.5f} (C) and {contrib['O']:.5f} (O), so carbon dominates.",
         f"Model kesin katsayılarla doğrusaldır: u(M_r) = √Σν_i²u²(A_i); katkılar {c(f'{contrib[chr(72)]:.5f}')} (H), {c(f'{contrib[chr(67)]:.5f}')} (C) ve {c(f'{contrib[chr(79)]:.5f}')} (O) olup karbon baskındır."),
       T(f"M_r = {est:.4f}, u(M_r) = {u:.4f}. The rectangular distributions express the lack of knowledge of the isotopic composition, not the actual composition (GUM-5 3.4.2).",
         f"M_r = {c(f'{est:.4f}')}, u(M_r) = {c(f'{u:.4f}')}. Dikdörtgen dağılımlar gerçek bileşimi değil, izotop bileşimi hakkındaki bilgi eksikliğini ifade eder (GUM-5 3.4.2).")],
      "Example 3, Tables 3.1 and 3.2", ["molar-mass", "atomic-weights", "rectangular", "linear"],
      {"output": {"name": "M_r", "expression": "6*A_C + 12*A_H + 6*A_O", "description": T("Relative molecular mass of glucose", "Glikozun bağıl molekül kütlesi")},
       "inputs": [inp("A_H", {"type": "rectangular", "lower": 1.00784, "upper": 1.00811}, None, "Standard atomic weight of hydrogen", "Hidrojenin standart atom ağırlığı"),
                  inp("A_C", {"type": "rectangular", "lower": 12.0096, "upper": 12.0116}, None, "Standard atomic weight of carbon", "Karbonun standart atom ağırlığı"),
                  inp("A_O", {"type": "rectangular", "lower": 15.99903, "upper": 15.99977}, None, "Standard atomic weight of oxygen", "Oksijenin standart atom ağırlığı")]},
      {"gum": {"estimate": chk(180.1557, 5e-5), "standard_uncertainty": chk(0.0038, 5e-5)}},
      T("JCGM GUM-5:2026, 3.3.3 and 3.3.4: M_r = 180.1557, u = 0.0038.", "JCGM GUM-5:2026, 3.3.3 ve 3.3.4: M_r = 180,1557, u = 0,0038."),
      notes=[T("The same calculation for CO₂ is NIST TN 1900 Example E8 in the library.", "CO₂ için aynı hesap kütüphanedeki NIST TN 1900 Örnek E8'dir.")])

# --- Example 14: activity of a radioactive source corrected for decay ------------------------------------
A0, uA0, Th, uTh, dt = 11.450, 0.050, 32.9, 1.4, 2.73
E = math.exp(-dt * math.log(2) / Th)
A1 = A0 * E
cT = A1 * dt * math.log(2) / Th ** 2
uA1 = math.hypot(E * uA0, cT * uTh)
assert abs(E - 0.94411) < 5e-6 and abs(A1 - 10.810) < 5e-4 and abs(cT - 0.018898) < 5e-7 and abs(uA1 - 0.054) < 5e-4
lin = []
for m in (-3, -2, -1, 0, 1, 2, 3):
    t = Th + m * uTh
    lin.append((t, A1 + cT * (t - Th), A0 * math.exp(-dt * math.log(2) / t)))
printed = [(28.7, 10.731, 10.719), (30.1, 10.757, 10.752), (31.5, 10.784, 10.782), (32.9, 10.810, 10.810),
           (34.3, 10.836, 10.835), (35.7, 10.863, 10.859), (37.1, 10.889, 10.881)]
for (t, l, e), (tp, lp, ep) in zip(lin, printed):
    assert abs(t - tp) < 1e-9 and abs(l - lp) < 6e-4 and abs(e - ep) < 6e-4, (t, l, e)
write("physical", "gum5-e14-radioactive-decay-bi207",
      T("Activity of a ²⁰⁷Bi source corrected for decay (JCGM GUM-5 Example 14)", "Bozunma için düzeltilmiş ²⁰⁷Bi kaynağının aktivitesi (JCGM GUM-5 Örnek 14)"),
      T("A₁ = A₀ exp(−Δt ln2/T½) with the certified activity, the half-life from the literature and Δt = 2.73 a; check of the linearization.",
        "Sertifikalı aktivite, literatürden yarı ömür ve Δt = 2,73 a ile A₁ = A₀ exp(−Δt ln2/T½); doğrusallaştırmanın denetimi."),
      [T("A₀ = 11.450 kBq, u = 0.050 kBq (certificate); T½ = 32.9 a, u = 1.4 a (literature, 4 %); Δt = t₁ − t₀ = 2.73 a without uncertainty.",
         "A₀ = 11,450 kBq, u = 0,050 kBq (sertifika); T½ = 32,9 a, u = 1,4 a (literatür, %4); Δt = t₁ − t₀ = 2,73 a, belirsizliksiz."),
       T(f"E = exp(−Δt ln2/T½) = {E:.5f}, A₁ = {A1:.3f} kBq. c(A₀) = E, c(T½) = A₁Δt ln2/T½² = {cT:.6f} kBq/a: the contributions are {E * uA0:.4f} kBq and {cT * uTh:.4f} kBq, u(A₁) = {uA1:.3f} kBq.",
         f"E = exp(−Δt ln2/T½) = {c(f'{E:.5f}')}, A₁ = {c(f'{A1:.3f}')} kBq. c(A₀) = E, c(T½) = A₁Δt ln2/T½² = {c(f'{cT:.6f}')} kBq/a: katkılar {c(f'{E * uA0:.4f}')} kBq ve {c(f'{cT * uTh:.4f}')} kBq, u(A₁) = {c(f'{uA1:.3f}')} kBq."),
       T("The model is nonlinear in T½. Between T½ ± 3u(T½) the linearized and exact models differ by at most 0.011 kBq, much less than u(A₁), so the law of propagation is adequate (Table 14.1). The Monte Carlo result confirms it.",
         "Model T½'ye göre doğrusal değildir. T½ ± 3u(T½) arasında doğrusallaştırılmış ve kesin modeller en çok 0,011 kBq farklıdır; bu u(A₁)'den çok küçüktür, dolayısıyla yayılım yasası yeterlidir (Tablo 14.1). Monte Carlo sonucu bunu doğrular."),
       T("A practical measure of the nonlinearity is u(E)/E = ln2·(u(T½)/T½)·(Δt/T½); when it reaches about 1 % the linearization should be checked (GUM-5 14.4.2). Here it is 0.25 %.",
         "Doğrusal olmamanın pratik ölçüsü u(E)/E = ln2·(u(T½)/T½)·(Δt/T½)'dir; yaklaşık %1'e ulaştığında doğrusallaştırma denetlenmelidir (GUM-5 14.4.2). Burada %0,25'tir.")],
      "Example 14, 14.3 and Table 14.1", ["radioactivity", "decay", "non-linear", "linearity-check"],
      {"output": {"name": "A_1", "unit": "kBq", "expression": "A_0*exp(-dt*ln(2)/T_half)", "description": T("Activity at the time of use", "Kullanım anındaki aktivite")},
       "inputs": [inp("A_0", {"type": "normal", "mean": A0, "sd": uA0}, "kBq", "Activity at the reference date (certificate)", "Referans tarihindeki aktivite (sertifika)"),
                  inp("T_half", {"type": "normal", "mean": Th, "sd": uTh}, "a", "Half-life of ²⁰⁷Bi", "²⁰⁷Bi'nin yarı ömrü"),
                  inp("dt", {"type": "constant", "value": dt}, "a", "Decay time t₁ − t₀", "Bozunma süresi t₁ − t₀")]},
      {"gum": {"estimate": chk(10.810, 5e-4), "standard_uncertainty": chk(0.054, 5e-4)}, "mcm": {"estimate": chk(10.810, 1e-3), "standard_uncertainty": chk(0.054, 1e-3)}},
      T("JCGM GUM-5:2026, 14.3.2: A₁ = 10.810 kBq, u(A₁) = 0.054 kBq (contributions 0.0472 and 0.0265 kBq).",
        "JCGM GUM-5:2026, 14.3.2: A₁ = 10,810 kBq, u(A₁) = 0,054 kBq (katkılar 0,0472 ve 0,0265 kBq)."))

# --- Example 15: breaking force of steel wire rope -------------------------------------------------------
F = [10006, 10007, 10005, 10008, 10003, 10005]
mF, sF = st.mean(F), st.stdev(F)
uF = sF / math.sqrt(len(F))
half = {"B": 6.8, "B_ref": 8.2, "e_res": 8.7, "e_T": 10.2}
uc = math.sqrt(uF ** 2 + sum((a / R3) ** 2 for a in half.values()))
assert abs(mF - 10005.7) < 0.05 and abs(sF - 1.8) < 0.06 and abs(uF - 0.7) < 0.05 and abs(uc - 9.9) < 0.05 and abs(1.96 * uc - 19.4) < 0.05
nu = (len(F) - 1) * (uc / uF) ** 4
assert nu > 1e5
u_mcm_stated = math.sqrt(uc ** 2 - uF ** 2 + uF ** 2 * 5 / 3)       # t-distribution, scale s/√n, ν = 5
u_mcm_sample_sd = math.sqrt(uc ** 2 - uF ** 2 + sF ** 2 * 5 / 3)    # scale s instead of s/√n
assert abs(u_mcm_stated - 9.93) < 0.01 and abs(u_mcm_sample_sd - 10.14) < 0.01
write("physical", "gum5-e15-wire-rope-breaking-force",
      T("Breaking force of steel wire rope from six destructive tests (JCGM GUM-5 Example 15)", "Altı tahribatlı deneyden çelik halatın kopma kuvveti (JCGM GUM-5 Örnek 15)"),
      T("F₀ = F̄ − B − B_ref − e_res − e_T: mean of six tests on pieces of one coil, with rectangular corrections for the load cells, resolution and temperature.",
        "F₀ = F̄ − B − B_ref − e_res − e_T: bir makaradan kesilen altı parça üzerindeki deneylerin ortalaması; yük hücreleri, çözünürlük ve sıcaklık için dikdörtgen düzeltmeler."),
      [T("A breaking test cannot be repeated on the same sample; six pieces of the same coil are tested and their variability (rope inhomogeneity) is embedded in the results (observation equations of GUM-6).",
         "Kopma deneyi aynı numunede tekrarlanamaz; aynı makaradan altı parça denenir ve bunların değişkenliği (halatın homojen olmaması) sonuçlara gömülür (GUM-6 gözlem denklemleri)."),
       T(f"F̄ = {mF:.1f} kN, s = {sF:.2f} kN, u(F̄) = s/√6 = {uF:.2f} kN with ν = 5; corrections B, B_ref, e_res, e_T rectangular with half-widths 6.8, 8.2, 8.7 and 10.2 kN.",
         f"F̄ = {c(f'{mF:.1f}')} kN, s = {c(f'{sF:.2f}')} kN, u(F̄) = s/√6 = {c(f'{uF:.2f}')} kN, ν = 5; B, B_ref, e_res, e_T düzeltmeleri yarı genişlikleri 6,8, 8,2, 8,7 ve 10,2 kN olan dikdörtgen dağılımlar."),
       T(f"u(F₀) = {uc:.1f} kN, ν_eff = 5·(u(F₀)/u(F̄))⁴ ≈ {nu:.1e}, k = 1.96, U = {1.96 * uc:.1f} kN. The test results contribute only 0.2 % to u(F₀); the corrections dominate.",
         f"u(F₀) = {c(f'{uc:.1f}')} kN, ν_eff = 5·(u(F₀)/u(F̄))⁴ ≈ {c(f'{nu:.1e}')}, k = 1,96, U = {c(f'{1.96 * uc:.1f}')} kN. Deney sonuçları u(F₀)'a yalnızca %0,2 katkı yapar; düzeltmeler baskındır.")],
      "Example 15, 15.3, 15.4 and 15.5", ["destructive-testing", "observation-equations", "rectangular", "type-a"],
      {"output": {"name": "F_0", "unit": "kN", "expression": "F - B - B_ref - e_res - e_T", "description": T("Breaking force of the rope", "Halatın kopma kuvveti")},
       "inputs": [inp("F", {"type": "type_a", "observations": F}, "kN", "Breaking forces of six pieces", "Altı parçanın kopma kuvvetleri"),
                  inp("B", {"type": "rectangular", "value": 0.0, "half_width": 6.8}, "kN", "Error of the test machine load cell", "Deney makinesi yük hücresinin hatası"),
                  inp("B_ref", {"type": "rectangular", "value": 0.0, "half_width": 8.2}, "kN", "Error of the reference load cell", "Referans yük hücresinin hatası"),
                  inp("e_res", {"type": "rectangular", "value": 0.0, "half_width": 8.7}, "kN", "Resolution of the force measuring system", "Kuvvet ölçme sisteminin çözünürlüğü"),
                  inp("e_T", {"type": "rectangular", "value": 0.0, "half_width": 10.2}, "kN", "Temperature effect", "Sıcaklık etkisi")]},
      {"gum": {"estimate": chk(10005.7, 0.05), "standard_uncertainty": chk(9.9, 0.05), "expanded_uncertainty": chk(19.4, 0.05)},
       "mcm": {"estimate": chk(10005.7, 0.05), "standard_uncertainty": chk(round(u_mcm_stated, 3), 0.02)}},
      T(f"JCGM GUM-5:2026, 15.4.1: F₀ = 10 005.7 kN, u = 9.9 kN, U = 19.4 kN (k = 1.96). The MCM standard uncertainty is recomputed: {u_mcm_stated:.2f} kN for the stated inputs; the guide prints 10.1 kN (see note).",
        f"JCGM GUM-5:2026, 15.4.1: F₀ = 10 005,7 kN, u = 9,9 kN, U = 19,4 kN (k = 1,96). MCM standart belirsizliği yeniden hesaplanmıştır: belirtilen girdilerle {c(f'{u_mcm_stated:.2f}')} kN; kılavuz 10,1 kN basar (nota bakınız)."),
      notes=[T(f"GUM-5 15.5.2 prints an MCM standard deviation of 10.1 kN and the interval [9986.0, 10 025.3] kN. With the stated t-distribution for F (scale u(F̄) = 0.7 kN, ν = 5) the MCM standard deviation is √(u_c² − u²(F̄) + u²(F̄)·5/3) = {u_mcm_stated:.2f} kN, practically the GUF value 9.9 kN. 10.1 kN follows if the scale is the standard deviation of a single test, s = 1.8 kN: {u_mcm_sample_sd:.2f} kN (research log F77).",
               f"GUM-5 15.5.2, 10,1 kN MCM standart sapması ve [9986,0; 10 025,3] kN aralığı basar. F için belirtilen t dağılımıyla (ölçek u(F̄) = 0,7 kN, ν = 5) MCM standart sapması √(u_c² − u²(F̄) + u²(F̄)·5/3) = {c(f'{u_mcm_stated:.2f}')} kN'dur; pratikte GUF değeri 9,9 kN'dur. 10,1 kN, ölçek olarak tek deneyin standart sapması s = 1,8 kN alınırsa çıkar: {c(f'{u_mcm_sample_sd:.2f}')} kN (araştırma günlüğü F77).")],
      )


# --- Example 1: pH by two-point and three-stage linear interpolation -----------------------------------
def lpu(f, x, u, corr=None):
    """First-order LPU with central differences; corr = {(i, j): r}."""
    n = len(x)
    cs = []
    for i in range(n):
        h = max(abs(x[i]), 1.0) * 1e-6
        xp, xm = list(x), list(x)
        xp[i] += h
        xm[i] -= h
        cs.append((f(xp) - f(xm)) / (2 * h))
    var = sum((cs[i] * u[i]) ** 2 for i in range(n))
    for (i, j), r in (corr or {}).items():
        var += 2 * cs[i] * cs[j] * r * u[i] * u[j]
    return f(x), math.sqrt(var), cs


def interp(x0, x1, y1, x2, y2):
    return y1 + (y2 - y1) * (x0 - x1) / (x2 - x1)


f25 = lambda v: interp(v[0], v[1], v[3], v[2], v[4])   # E_X, E_S1, E_S2, pH_S1, pH_S2
x25 = [-1.875, 6.15, -26.35, 6.8640, 7.4157]
u25 = [0.0250, 0.0289, 0.0289, 0.0051, 0.0051]
y25, uy25, c25 = lpu(f25, x25, u25)
assert abs(y25 - 7.0002) < 5e-5 and abs(uy25 - 0.0041) < 5e-5
_, uy25r, _ = lpu(f25, x25, u25, {(3, 4): 1.0})
assert abs(uy25r - 0.0051) < 5e-5
parts = sorted((abs(ci * ui) for ci, ui in zip(c25, u25)), reverse=True)
assert [round(p, 4) for p in parts] == [0.0038, 0.0013, 0.0004, 0.0004, 0.0001], parts


def fT(v):   # T_X, E_X, E_S1, E_S2, pH_S1,T1, pH_S2,T1, pH_S1,T2, pH_S2,T2 with T1 = 25 °C, T2 = 20 °C
    p1 = interp(v[1], v[2], v[4], v[3], v[5])
    p2 = interp(v[1], v[2], v[6], v[3], v[7])
    return interp(v[0], 25.0, p1, 20.0, p2)


xT = [23.7, -1.875, 6.15, -26.35, 6.8640, 7.4157, 6.8796, 7.4323]
uT = [0.0058, 0.025, 0.029, 0.029, 0.0051, 0.0051, 0.0051, 0.0051]
yT, uyT, _ = lpu(fT, xT, uT)
p1 = interp(-1.875, 6.15, 6.8640, -26.35, 7.4157)
p2 = interp(-1.875, 6.15, 6.8796, -26.35, 7.4323)
assert abs(p1 - 7.0002) < 5e-5 and abs(p2 - 7.0161) < 5e-5 and abs(yT - 7.0043) < 5e-5
# GUM-5 prints u = 0.0041. With the four certified values independent (1.7.1) u = 0.0032; with the values of each
# standard at 20 °C and 25 °C fully correlated u = 0.0041 (research log F78).
assert abs(uyT - 0.0032) < 5e-5
_, uyTc, _ = lpu(fT, xT, uT, {(4, 6): 1.0, (5, 7): 1.0})
assert abs(uyTc - 0.0041) < 5e-5
_, u_noph, _ = lpu(f25, x25, [0.025, 0.0289, 0.0289, 0, 0])
_, u_noE, _ = lpu(f25, x25, [0, 0, 0, 0.0051, 0.0051])
assert abs(u_noph - 0.0006) < 5e-5 and abs(u_noE - 0.0040) < 5e-5

ph_inputs = [inp("E_X", {"type": "normal", "mean": -1.875, "sd": 0.0250}, "mV", "Potential in the test solution (mean of 4)", "Deney çözeltisindeki potansiyel (4 ölçümün ortalaması)"),
             inp("E_S1", {"type": "normal", "mean": 6.15, "sd": 0.0289}, "mV", "Potential in standard S1 (SRM 186-I-g)", "S1 standardındaki potansiyel (SRM 186-I-g)"),
             inp("E_S2", {"type": "normal", "mean": -26.35, "sd": 0.0289}, "mV", "Potential in standard S2 (SRM 186-II-g)", "S2 standardındaki potansiyel (SRM 186-II-g)"),
             inp("pH_S1", {"type": "normal", "mean": 6.8640, "sd": 0.0051}, None, "Certified pH of S1 at 25 °C", "S1'in 25 °C'deki sertifikalı pH değeri"),
             inp("pH_S2", {"type": "normal", "mean": 7.4157, "sd": 0.0051}, None, "Certified pH of S2 at 25 °C", "S2'nin 25 °C'deki sertifikalı pH değeri")]
ph_expl = [T("Two-point calibration by bracketing (GUM-5 1.4.2, 1.5): the pH of the test solution is interpolated linearly between the points (E_S1, pH_S1) and (E_S2, pH_S2) of two NIST phosphate standards: pH_X = pH_S1 + (pH_S2 − pH_S1)(E_X − E_S1)/(E_S2 − E_S1).",
             "Parantez yöntemiyle iki noktalı kalibrasyon (GUM-5 1.4.2, 1.5): deney çözeltisinin pH değeri, iki NIST fosfat standardının (E_S1, pH_S1) ve (E_S2, pH_S2) noktaları arasında doğrusal interpolasyonla bulunur: pH_X = pH_S1 + (pH_S2 − pH_S1)(E_X − E_S1)/(E_S2 − E_S1)."),
           T(f"With q = (E_X − E_S1)/(E_S2 − E_S1) = {(x25[0] - x25[1]) / (x25[2] - x25[1]):.4f} and slope b = {(x25[4] - x25[3]) / (x25[2] - x25[1]):.6f} mV⁻¹, u²(pH_X) = b²(1 − q)²u²(E_S1) + (1 − q)²u²(pH_S1) + b²q²u²(E_S2) + q²u²(pH_S2) + b²u²(E_X) (Formula 1.17); the contributions are 0.0038 (pH_S1), 0.0013 (pH_S2), 0.0004, 0.0004 and 0.0001.",
             f"q = (E_X − E_S1)/(E_S2 − E_S1) = {c(f'{(x25[0] - x25[1]) / (x25[2] - x25[1]):.4f}')} ve eğim b = {c(f'{(x25[4] - x25[3]) / (x25[2] - x25[1]):.6f}')} mV⁻¹ ile u²(pH_X) = b²(1 − q)²u²(E_S1) + (1 − q)²u²(pH_S1) + b²q²u²(E_S2) + q²u²(pH_S2) + b²u²(E_X) (Formül 1.17); katkılar 0,0038 (pH_S1), 0,0013 (pH_S2), 0,0004, 0,0004 ve 0,0001'dir.")]
write("chemistry", "gum5-e1-ph-two-point-25c",
      T("pH of a test solution by two-point interpolation at 25 °C (JCGM GUM-5 Example 1)", "25 °C'de iki noktalı interpolasyonla bir deney çözeltisinin pH değeri (JCGM GUM-5 Örnek 1)"),
      T("Bracketing between two NIST phosphate pH standards measured with the same cell; inputs independent, as in IUPAC practice.",
        "Aynı hücreyle ölçülen iki NIST fosfat pH standardı arasında parantez yöntemi; IUPAC uygulamasındaki gibi girdiler bağımsız."),
      ph_expl + [T(f"pH_X = {y25:.4f}, u(pH_X) = {uy25:.4f}: the certified pH values dominate. A common calibration error δE_cal of the three potentials cancels (GUM-5 1.8.2).",
                   f"pH_X = {c(f'{y25:.4f}')}, u(pH_X) = {c(f'{uy25:.4f}')}: sertifikalı pH değerleri baskındır. Üç potansiyelin ortak kalibrasyon hatası δE_cal birbirini götürür (GUM-5 1.8.2).")],
      "Example 1, 1.8", ["ph", "interpolation", "calibration", "electrochemistry"],
      {"output": {"name": "pH_X", "expression": "pH_S1 + (pH_S2 - pH_S1)*(E_X - E_S1)/(E_S2 - E_S1)", "description": T("pH of the test solution", "Deney çözeltisinin pH değeri")},
       "inputs": ph_inputs},
      {"gum": {"estimate": chk(7.0002, 5e-5), "standard_uncertainty": chk(0.0041, 5e-5)}, "mcm": {"estimate": chk(7.0002, 1e-4), "standard_uncertainty": chk(0.0041, 1e-4)}},
      T("JCGM GUM-5:2026, 1.8.1: pH_X = 7.0002, u = 0.0041.", "JCGM GUM-5:2026, 1.8.1: pH_X = 7,0002, u = 0,0041."))
write("chemistry", "gum5-e1-ph-two-point-25c-correlated-standards",
      T("pH by two-point interpolation with fully correlated pH standards (JCGM GUM-5 Example 1)", "Tam ilişkili pH standartlarıyla iki noktalı interpolasyon (JCGM GUM-5 Örnek 1)"),
      T("As the 25 °C case, but with correlation 1 between the certified pH values, which share the Bates–Guggenheim convention.",
        "25 °C durumu gibi, ancak Bates–Guggenheim kuralını paylaşan sertifikalı pH değerleri arasında korelasyon 1."),
      ph_expl[:1] + [T("The certificates give an SI-traceable uncertainty and a much smaller one without the Bates–Guggenheim convention; the convention term is common to both standards, so the correlation of the two pH values is close to 1 (GUM-5 1.10.3.3). Here r = 1.",
                       "Sertifikalar SI'ya izlenebilir bir belirsizlik ve Bates–Guggenheim kuralı olmadan çok daha küçük bir belirsizlik verir; kural terimi iki standart için ortaktır, bu nedenle iki pH değerinin korelasyonu 1'e yakındır (GUM-5 1.10.3.3). Burada r = 1."),
                     T(f"With r = 1 the terms (1 − q)²u²(pH_S1) + 2q(1 − q)u(pH_S1, pH_S2) + q²u²(pH_S2) combine to u²(pH_S1) (Formula 1.24): u(pH_X) = {uy25r:.4f}, against {uy25:.4f} when the correlation is ignored. Perfectly correlated contributions add linearly: 0.0038 + 0.0013 = 0.0051.",
                       f"r = 1 iken (1 − q)²u²(pH_S1) + 2q(1 − q)u(pH_S1, pH_S2) + q²u²(pH_S2) terimleri u²(pH_S1)'e indirgenir (Formül 1.24): u(pH_X) = {c(f'{uy25r:.4f}')}; korelasyon yok sayılırsa {c(f'{uy25:.4f}')}. Tam ilişkili katkılar doğrusal toplanır: 0,0038 + 0,0013 = 0,0051.")],
      "Example 1, 1.10.3.4 and 1.10.3.5", ["ph", "interpolation", "correlation", "electrochemistry"],
      {"output": {"name": "pH_X", "expression": "pH_S1 + (pH_S2 - pH_S1)*(E_X - E_S1)/(E_S2 - E_S1)", "description": T("pH of the test solution", "Deney çözeltisinin pH değeri")},
       "inputs": ph_inputs, "correlations": [{"a": "pH_S1", "b": "pH_S2", "r": 1.0}]},
      {"gum": {"estimate": chk(7.0002, 5e-5), "standard_uncertainty": chk(0.0051, 5e-5)}},
      T("JCGM GUM-5:2026, 1.10.3.5: u(pH_X) = 0.0051 with perfectly correlated standards.", "JCGM GUM-5:2026, 1.10.3.5: tam ilişkili standartlarla u(pH_X) = 0,0051."),
      notes=[T("1.10.3.5 attributes u = 0.0051 to pH_X = 7.0109 and 'the data in subclause 1.7'; the calculation uses the 25 °C data of 1.8, for which pH_X = 7.0002 (research log F78).",
               "1.10.3.5, u = 0,0051 değerini pH_X = 7,0109'a ve '1.7 alt maddesindeki verilere' bağlar; hesap 1.8'in 25 °C verilerini kullanır ve bu veriler için pH_X = 7,0002'dir (araştırma günlüğü F78).")])
PH_T_INPUTS = [inp("T_X", {"type": "normal", "mean": 23.7, "sd": 0.0058}, "°C", "Temperature of the test solution", "Deney çözeltisinin sıcaklığı"),
               inp("E_X", {"type": "normal", "mean": -1.875, "sd": 0.025}, "mV", "Potential in the test solution", "Deney çözeltisindeki potansiyel"),
               inp("E_S1", {"type": "normal", "mean": 6.15, "sd": 0.029}, "mV", "Potential in standard S1", "S1 standardındaki potansiyel"),
               inp("E_S2", {"type": "normal", "mean": -26.35, "sd": 0.029}, "mV", "Potential in standard S2", "S2 standardındaki potansiyel"),
               inp("pH_S1_T1", {"type": "normal", "mean": 6.8640, "sd": 0.0051}, None, "pH of S1 at 25 °C", "S1'in 25 °C'deki pH değeri"),
               inp("pH_S2_T1", {"type": "normal", "mean": 7.4157, "sd": 0.0051}, None, "pH of S2 at 25 °C", "S2'nin 25 °C'deki pH değeri"),
               inp("pH_S1_T2", {"type": "normal", "mean": 6.8796, "sd": 0.0051}, None, "pH of S1 at 20 °C", "S1'in 20 °C'deki pH değeri"),
               inp("pH_S2_T2", {"type": "normal", "mean": 7.4323, "sd": 0.0051}, None, "pH of S2 at 20 °C", "S2'nin 20 °C'deki pH değeri"),
               inp("T1", {"type": "constant", "value": 25.0}, "°C", "Certificate temperature T₁", "Sertifika sıcaklığı T₁"),
               inp("T2", {"type": "constant", "value": 20.0}, "°C", "Certificate temperature T₂", "Sertifika sıcaklığı T₂")]
PH_T_MODEL = {"output": {"name": "pH_X", "expression": "pH_T1 + (pH_T2 - pH_T1)*(T_X - T1)/(T2 - T1)", "description": T("pH of the test solution at T_X", "Deney çözeltisinin T_X'teki pH değeri")},
              "intermediates": [{"name": "pH_T1", "expression": "pH_S1_T1 + (pH_S2_T1 - pH_S1_T1)*(E_X - E_S1)/(E_S2 - E_S1)", "description": T("pH of X interpolated at 25 °C", "X'in 25 °C'de interpolasyonla pH değeri")},
                                {"name": "pH_T2", "expression": "pH_S1_T2 + (pH_S2_T2 - pH_S1_T2)*(E_X - E_S1)/(E_S2 - E_S1)", "description": T("pH of X interpolated at 20 °C", "X'in 20 °C'de interpolasyonla pH değeri")}],
              "inputs": PH_T_INPUTS}
PH_T_EXPL = [T("The algorithm of GUM-5 1.6 is the measurement model: pH_X,T1 and pH_X,T2 are interpolated in potential with the certified values at T₁ = 25 °C and T₂ = 20 °C, and pH_X at T_X = 23.7 °C is interpolated between them; T₁ and T₂ are exact certificate temperatures.",
               "GUM-5 1.6'nın algoritması ölçüm modelidir: pH_X,T1 ve pH_X,T2, T₁ = 25 °C ve T₂ = 20 °C'deki sertifikalı değerlerle potansiyelde interpolasyonla bulunur ve T_X = 23,7 °C'deki pH_X bunların arasında interpolasyonla elde edilir; T₁ ve T₂ kesin sertifika sıcaklıklarıdır."),
             T(f"pH_X,T1 = {p1:.4f}, pH_X,T2 = {p2:.4f}, pH_X = 0.74·pH_X,T1 + 0.26·pH_X,T2 = {yT:.4f}; cubic interpolation in temperature changes the pH of S1 by only 0.0004 (GUM-5 1.10.1.1).",
               f"pH_X,T1 = {c(f'{p1:.4f}')}, pH_X,T2 = {c(f'{p2:.4f}')}, pH_X = 0,74·pH_X,T1 + 0,26·pH_X,T2 = {c(f'{yT:.4f}')}; sıcaklıkta kübik interpolasyon S1'in pH değerini yalnızca 0,0004 değiştirir (GUM-5 1.10.1.1).")]
F78_NOTE = T(f"GUM-5 1.9.1 obtains pH_X = 7.0043 by the third interpolation, as recomputed here ({yT:.4f}), and then states that the method gives pH_X = 7.0109; 7.0109 does not follow from the data. It prints u = 0.0041 although 1.7.1 assumes all inputs independent, which gives {uyT:.4f}; 0.0041 results when the certified values of each standard at 20 °C and 25 °C are fully correlated ({uyTc:.4f}) (research log F78).",
             f"GUM-5 1.9.1, üçüncü interpolasyonla pH_X = 7,0043 bulur (burada yeniden hesaplanan: {c(f'{yT:.4f}')}), ardından yöntemin pH_X = 7,0109 verdiğini söyler; 7,0109 verilerden çıkmaz. 1.7.1 tüm girdileri bağımsız varsaydığı hâlde u = 0,0041 basar; bağımsızlıkla u = {c(f'{uyT:.4f}')} çıkar. 0,0041, her standardın 20 °C ve 25 °C'deki sertifikalı değerleri tam ilişkili alındığında elde edilir ({c(f'{uyTc:.4f}')}) (araştırma günlüğü F78).")


def ph_temperature(pid, title, desc, extra, corr, u_exp, ref):
    model = dict(PH_T_MODEL)
    if corr:
        model["correlations"] = corr
    write("chemistry", pid, title, desc, PH_T_EXPL + extra, "Example 1, 1.6, 1.9 and 1.10.3", ["ph", "interpolation", "temperature", "electrochemistry", "intermediate-quantities"] + (["correlation"] if corr else []),
          model, {"gum": {"estimate": chk(round(yT, 4), 5e-5), "standard_uncertainty": chk(u_exp, 5e-5)}, "mcm": {"estimate": chk(round(yT, 4), 1e-4), "standard_uncertainty": chk(u_exp, 1e-4)}},
          ref, notes=[F78_NOTE])


ph_temperature("gum5-e1-ph-temperature-interpolation",
               T("pH at 23.7 °C by three linear interpolations, independent certified values (JCGM GUM-5 Example 1)", "Potansiyel ve sıcaklıkta üç doğrusal interpolasyonla 23,7 °C'de pH, bağımsız sertifikalı değerler (JCGM GUM-5 Örnek 1)"),
               T("The test solution is at 23.7 °C, between the certificate temperatures 20 °C and 25 °C: interpolation in potential at each temperature, then in temperature; all inputs independent (GUM-5 1.7.1).",
                 "Deney çözeltisi 23,7 °C'dedir, sertifika sıcaklıkları 20 °C ile 25 °C arasında: her sıcaklıkta potansiyelde, sonra sıcaklıkta interpolasyon; tüm girdiler bağımsız (GUM-5 1.7.1)."),
               [T(f"With independent certified values the interpolation in temperature averages the uncertainties of the values at 20 °C and 25 °C: u(pH_X) = {uyT:.4f}.",
                  f"Sertifikalı değerler bağımsızken sıcaklıktaki interpolasyon 20 °C ve 25 °C'deki değerlerin belirsizliklerinin ortalamasını alır: u(pH_X) = {c(f'{uyT:.4f}')}.")],
               None, round(uyT, 4),
               T(f"JCGM GUM-5:2026, 1.9.1: pH_X,T1 = 7.0002, pH_X,T2 = 7.0161, pH_X = 7.0043; u recomputed for independent inputs ({uyT:.4f}); the guide prints 0.0041 (see note).",
                 f"JCGM GUM-5:2026, 1.9.1: pH_X,T1 = 7,0002, pH_X,T2 = 7,0161, pH_X = 7,0043; u bağımsız girdiler için yeniden hesaplanmıştır ({c(f'{uyT:.4f}')}); kılavuz 0,0041 basar (nota bakınız)."))
ph_temperature("gum5-e1-ph-temperature-interpolation-correlated",
               T("pH at 23.7 °C by three linear interpolations, certified values of each standard correlated across temperatures (JCGM GUM-5 Example 1)", "Üç doğrusal interpolasyonla 23,7 °C'de pH, her standardın sertifikalı değerleri sıcaklıklar arasında ilişkili (JCGM GUM-5 Örnek 1)"),
               T("As the independent case, but the pH values of each standard at 20 °C and 25 °C are fully correlated, as the shared Bates–Guggenheim convention suggests (GUM-5 1.10.3.3).",
                 "Bağımsız durum gibi, ancak her standardın 20 °C ve 25 °C'deki pH değerleri, ortak Bates–Guggenheim kuralının gösterdiği gibi tam ilişkilidir (GUM-5 1.10.3.3)."),
               [T(f"With r = 1 between the values of the same standard the interpolation in temperature no longer averages them: u(pH_X) = {uyTc:.4f}, the value GUM-5 prints.",
                  f"Aynı standardın değerleri arasında r = 1 iken sıcaklıktaki interpolasyon artık bunların ortalamasını almaz: u(pH_X) = {c(f'{uyTc:.4f}')}; GUM-5'in bastığı değer budur.")],
               [{"a": "pH_S1_T1", "b": "pH_S1_T2", "r": 1.0}, {"a": "pH_S2_T1", "b": "pH_S2_T2", "r": 1.0}], 0.0041,
               T("JCGM GUM-5:2026, 1.9.1: pH_X = 7.0043 (see note), u = 0.0041.", "JCGM GUM-5:2026, 1.9.1: pH_X = 7,0043 (nota bakınız), u = 0,0041."))

# --- Example 2: benzo[a]pyrene by internal standard (GC-MS) -----------------------------------------------
# Table 2.1. A_E and A_ISE are means of three paired injections (ν = 2); f has ν = 9, m_ISE ν = ∞.
BAP = {"f": (0.616, 0.017, 9), "mISE": (0.2455, 0.0036, None), "AE": (85114, 9564, 2), "AISE": (917546, 44492, 2)}
r_fm = -3.3e-5 / (0.017 * 0.0036)
r_A = -2.03e8 / (9564 * 44492)
xb = {k: v[0] for k, v in BAP.items()}
mE = xb["f"] * xb["AE"] / xb["AISE"] * xb["mISE"]
cb = {"f": mE / xb["f"], "mISE": mE / xb["mISE"], "AE": mE / xb["AE"], "AISE": -mE / xb["AISE"]}
ub = {k: cb[k] * BAP[k][1] for k in BAP}          # signed contributions c_i u_i
v_fm = 2 * ub["f"] * ub["mISE"] * r_fm
v_A = 2 * ub["AE"] * ub["AISE"] * r_A
u2 = sum(v * v for v in ub.values()) + v_fm + v_A
u2_ind = sum(v * v for v in ub.values())
nu_ws = u2 ** 2 / (ub["f"] ** 4 / 9 + ub["AE"] ** 4 / 2 + ub["AISE"] ** 4 / 2)
# GUM-5 eq. (2.3): Σ_i Σ_j r_ij² c_i² c_j² u_i² u_j² / ν_i.
R = {("f", "mISE"): r_fm, ("AE", "AISE"): r_A}
r = lambda a, b: 1.0 if a == b else R.get((a, b), R.get((b, a), 0.0))
nu_23 = u2 ** 2 / sum(r(a, b) ** 2 * ub[a] ** 2 * ub[b] ** 2 / BAP[a][2] for a in BAP for b in BAP if BAP[a][2])
# Willink (2007) §4.1.1: A_E and A_ISE form an ensemble, one component with ν = 2.
v_ens = ub["AE"] ** 2 + ub["AISE"] ** 2 + v_A
nu_ens = u2 ** 2 / (ub["f"] ** 4 / 9 + v_ens ** 2 / 2)
assert abs(mE - 0.014) < 5e-4 and abs(u2 - 4.1e-6) < 0.05e-6 and abs(u2_ind - 3.1e-6) < 0.05e-6
assert abs(r_fm + 0.54) < 5e-3 and abs(r_A + 0.48) < 5e-3
assert abs(nu_ws - 5.2) < 0.05 and abs(nu_23 - 4.8) < 0.05 and abs(nu_ens - 2.10) < 0.01
ub_ = math.sqrt(u2)
BAP_MODEL_INPUTS = [
    inp("f", {"type": "normal", "mean": 0.616, "sd": 0.017}, None, "GC-MS calibration factor (mean of three factors, ν = 9)", "GC-MS kalibrasyon faktörü (üç faktörün ortalaması, ν = 9)"),
    inp("mISE", {"type": "normal", "mean": 0.2455, "sd": 0.0036}, "ng", "Mass of internal standard BaP-d12 in 1 µL of extract (certificate)", "1 µL ekstrakttaki iç standart BaP-d12 kütlesi (sertifika)"),
    inp("AE", {"type": "normal", "mean": 85114, "sd": 9564}, "a.u.", "Mean BaP peak area of three injections", "Üç enjeksiyonun ortalama BaP pik alanı"),
    inp("AISE", {"type": "normal", "mean": 917546, "sd": 44492}, "a.u.", "Mean BaP-d12 peak area of the same three injections", "Aynı üç enjeksiyonun ortalama BaP-d12 pik alanı"),
]
BAP_MODEL_INPUTS[0]["dof"] = 9
BAP_MODEL_INPUTS[2]["dof"] = 2
BAP_MODEL_INPUTS[3]["dof"] = 2
BAP_CORR = [{"a": "f", "b": "mISE", "r": round(r_fm, 4)}, {"a": "AE", "b": "AISE", "r": round(r_A, 4)}]
BAP_EXPL = [
    T("Internal-standard model (EN 15549): m_E = f·A_E/A_ISE·m_ISE, the mass of benzo[a]pyrene in 1 µL of the second extract of a glass-fibre filter spiked with NIST SRM 2260a.",
      "İç standart modeli (EN 15549): m_E = f·A_E/A_ISE·m_ISE; NIST SRM 2260a ile zenginleştirilmiş cam elyaf filtrenin ikinci ekstraktının 1 µL'sindeki benzo[a]piren kütlesi."),
    T(f"The same internal standard is used in calibration and analysis, so r(f, m_ISE) = −3.3·10⁻⁵/(0.017 × 0.0036) = {r_fm:.2f}; the peak areas of the same injections give r(A_E, A_ISE) = −2.03·10⁸/(9564 × 44 492) = {r_A:.2f}.",
      f"Kalibrasyonda ve analizde aynı iç standart kullanıldığından r(f, m_ISE) = −3,3·10⁻⁵/(0,017 × 0,0036) = {c(f'{r_fm:.2f}')}; aynı enjeksiyonların pik alanları r(A_E, A_ISE) = −2,03·10⁸/(9564 × 44 492) = {c(f'{r_A:.2f}')} verir."),
    T(f"u²(m_E) = {u2:.3e} ng² with the covariance terms and {u2_ind:.3e} ng² without them; the A_E–A_ISE term adds {v_A:.2e} ng² because the correlation is negative while the sensitivities have opposite signs.",
      f"Kovaryans terimleriyle u²(m_E) = {c(f'{u2:.3e}')} ng², onlarsız {c(f'{u2_ind:.3e}')} ng²; korelasyon negatif, duyarlılıklar zıt işaretli olduğundan A_E–A_ISE terimi {c(f'{v_A:.2e}')} ng² ekler."),
]
BAP_NOTES = [
    T("GUM-5 (2.4.2) also applies the MCM with a bivariate normal for (f, m_ISE), a bivariate t with ν = n − N = 1 for (A_E, A_ISE) and rejection of negative draws: median 0.014 ng, shortest 95 % interval [≈ 0, 0.032] ng. The project format has no joint t-distribution for a scalar model, so the MCM is not run here; tools/research/gum5/ex2_mcm.py reproduces the printed interval (9.3·10⁶ of 10⁷ draws kept, [1·10⁻⁷, 0.0321] ng).",
      "GUM-5 (2.4.2) MCM'yi de uygular: (f, m_ISE) için iki değişkenli normal, (A_E, A_ISE) için ν = n − N = 1 olan iki değişkenli t ve negatif çekilişlerin reddi; medyan 0,014 ng, en kısa %95 aralık [≈ 0; 0,032] ng. Proje biçiminde skaler model için ortak t-dağılımı olmadığından MCM burada çalıştırılmaz; tools/research/gum5/ex2_mcm.py basılı aralığı yeniden üretir (10⁷ çekilişin 9,3·10⁶'sı tutuldu, [1·10⁻⁷; 0,0321] ng)."),
    T("The uncertainty applies to a clean spiked filter only; matrix effects and recovery are deliberately excluded (GUM-5 2.2.2, note).",
      "Belirsizlik yalnız temiz, zenginleştirilmiş filtre için geçerlidir; matris etkileri ve geri kazanım bilerek dışarıda bırakılmıştır (GUM-5 2.2.2, not)."),
]
BAP_TAGS = ["internal-standard", "gc-ms", "pah", "correlation", "welch-satterthwaite", "ratio"]
BAP_OUT = {"name": "mE", "unit": "ng", "expression": "f*AE/AISE*mISE", "description": T("Mass of benzo[a]pyrene in 1 µL of extract", "1 µL ekstrakttaki benzo[a]piren kütlesi")}
write("chemistry", "gum5-e2-benzo-a-pyrene",
      T("Benzo[a]pyrene by GC-MS with an internal standard, correlated inputs (JCGM GUM-5 Example 2)",
        "İç standartla GC-MS'te benzo[a]piren, ilişkili girdiler (JCGM GUM-5 Örnek 2)"),
      T("Internal-standard model with two correlated pairs of inputs; LPU with covariances and the Welch–Satterthwaite ν_eff as printed.",
        "İki ilişkili girdi çiftli iç standart modeli; kovaryanslı LPU ve basıldığı gibi Welch–Satterthwaite ν_eff."),
      BAP_EXPL + [
          T(f"Welch–Satterthwaite (G.2b), correlations ignored in the denominator: ν_eff = {nu_ws:.2f}, k(95 %) = 2.5; the program warns that G.2b is not valid for correlated inputs. GUM-5 eq. (2.3) gives {nu_23:.2f} and k = 2.6, U = 0.0052 ng; the ensemble treatment of the companion project gives {nu_ens:.2f}.",
            f"Welch–Satterthwaite (G.2b), payda korelasyonsuz: ν_eff = {c(f'{nu_ws:.2f}')}, k(%95) = 2,5; program G.2b'nin ilişkili girdiler için geçerli olmadığını uyarır. GUM-5 denklem (2.3) {c(f'{nu_23:.2f}')} ve k = 2,6, U = 0,0052 ng verir; eşlik eden projedeki topluluk yaklaşımı {c(f'{nu_ens:.2f}')} verir.")],
      "Example 2, Tables 2.1–2.3", BAP_TAGS,
      {"output": BAP_OUT, "inputs": BAP_MODEL_INPUTS, "correlations": BAP_CORR},
      {"gum": {"estimate": chk(0.0140, 5e-5), "standard_uncertainty": chk(0.00202, 1.5e-5), "effective_dof": chk(5.2, 0.05), "coverage_factor": chk(2.5, 0.05)}},
      T("JCGM GUM-5:2026, 2.4.1.2–2.4.1.4 and Table 2.3: m_E = 0.014 ng, u = 0.002 ng (u² = 4.1·10⁻⁶ ng²), ν_eff = 5.2, k = 2.5.",
        "JCGM GUM-5:2026, 2.4.1.2–2.4.1.4 ve Tablo 2.3: m_E = 0,014 ng, u = 0,002 ng (u² = 4,1·10⁻⁶ ng²), ν_eff = 5,2, k = 2,5."),
      notes=BAP_NOTES, analysis={"gum": {"dof_rounding": "exact"}, "mcm": None})
write("chemistry", "gum5-e2-benzo-a-pyrene-ensemble",
      T("Benzo[a]pyrene: peak areas of the same injections as an ensemble in ν_eff (JCGM GUM-5 Example 2, Willink 2007)",
        "Benzo[a]piren: aynı enjeksiyonların pik alanları ν_eff'te topluluk olarak (JCGM GUM-5 Örnek 2, Willink 2007)"),
      T("The same budget with A_E and A_ISE declared an ensemble: their joint variance is one Welch–Satterthwaite component with ν = 2, giving ν_eff ≈ 2.1 instead of 5.2 or 4.8.",
        "Aynı bütçe, A_E ve A_ISE topluluk olarak tanımlanmış: ortak varyansları ν = 2 olan tek Welch–Satterthwaite bileşenidir; ν_eff 5,2 veya 4,8 yerine ≈ 2,1 olur."),
      BAP_EXPL + [
          T(f"A_E and A_ISE are means of the same three paired injections; their estimated covariance matrix has a Wishart distribution with ν = 2, so the estimate of the variance of any combination c_E·A_E + c_I·A_ISE has exactly ν = 2 (Willink 2007, §4.1.1; GTC real_ensemble). Their component is (c_E u_E)² + (c_I u_I)² + 2c_E c_I u(A_E, A_ISE) = {v_ens:.3e} ng², {100 * v_ens / u2:.0f} % of u².",
            f"A_E ve A_ISE aynı üç eşli enjeksiyonun ortalamalarıdır; tahmin edilen kovaryans matrisleri ν = 2 olan Wishart dağılımına uyar; bu nedenle her c_E·A_E + c_I·A_ISE bileşiminin varyans tahmini tam ν = 2'ye sahiptir (Willink 2007, §4.1.1; GTC real_ensemble). Bileşenleri (c_E u_E)² + (c_I u_I)² + 2c_E c_I u(A_E, A_ISE) = {c(f'{v_ens:.3e}')} ng², u²'nin %{100 * v_ens / u2:.0f}'idir."),
          T(f"ν_eff = u⁴/[(c_f u_f)⁴/9 + {v_ens:.3e}²/2] = {nu_ens:.2f}; k(95 %) = 4.1 and U = 0.0083 ng, closer to the width of the MCM interval [≈ 0, 0.032] ng than the U = 0.0052 ng printed. GUM-5 eq. (2.3) treats the two areas as separate components and gives a ν_eff above 2 although the dominant component has ν = 2 (research log F79). The f–m_ISE correlation (ν = 9 and ∞) is not an ensemble, so the warning remains.",
            f"ν_eff = u⁴/[(c_f u_f)⁴/9 + {c(f'{v_ens:.3e}')}²/2] = {c(f'{nu_ens:.2f}')}; k(%95) = 4,1 ve U = 0,0083 ng; basılı U = 0,0052 ng'den çok MCM aralığının [≈ 0; 0,032] ng genişliğine yakındır. GUM-5 denklem (2.3) iki alanı ayrı bileşenler olarak ele alır ve baskın bileşenin ν'si 2 olduğu hâlde 2'nin üstünde ν_eff verir (araştırma günlüğü F79). f–m_ISE korelasyonu (ν = 9 ve ∞) topluluk değildir; bu nedenle uyarı kalır.")],
      "Example 2, Tables 2.1–2.3; Willink, Metrologia 44 (2007) 340, §4.1.1", BAP_TAGS + ["ensemble"],
      {"output": BAP_OUT, "inputs": BAP_MODEL_INPUTS, "correlations": BAP_CORR, "ensembles": [["AE", "AISE"]]},
      {"gum": {"estimate": chk(0.0140, 5e-5), "standard_uncertainty": chk(0.00202, 1.5e-5), "effective_dof": chk(round(nu_ens, 2), 0.01)}},
      T(f"Budget of JCGM GUM-5:2026 Example 2 (u = 0.002 ng); ν_eff = {nu_ens:.2f} recomputed with the ensemble rule of Willink (2007), not printed in GUM-5.",
        f"JCGM GUM-5:2026 Örnek 2 bütçesi (u = 0,002 ng); ν_eff = {c(f'{nu_ens:.2f}')} Willink (2007) topluluk kuralıyla yeniden hesaplandı, GUM-5'te basılı değildir."),
      notes=BAP_NOTES, analysis={"gum": {"dof_rounding": "exact"}, "mcm": None})

# --- Example 10: conformity of total suspended particulate matter (lognormal prior) ------------------------
# Printed values: global risks at A_U = T_U 0.5 % (consumer) and 0.9 % (producer); A_U = 0.17: 0.002 %, 5.8 %;
# A_U = 0.22: 1.7 %, < 0.1 %; specific producer's risk at ρ̂ = 0.225: about 11 %. They follow from a joint PDF
# whose normal part has the standard deviation 0.07·ρ̂ (10.6.3.1), normalized by C = 1/1.005. With the
# conditional PDF of ρ̂ given ρ (standard deviation 0.07·ρ) the global risks are those below (F80).
# tools/research/gum5/ex10_risks.py computes both by quadrature (scipy).
MU, SG = -2.325, 0.434
p_conf = 0.5 * (1 + math.erf((math.log(0.2) - MU) / SG / math.sqrt(2)))
assert abs(p_conf - 0.9504) < 1e-4
TSPM_GLOBAL = {0.17: (0.000103, 0.053702), 0.2: (0.005790, 0.007392), 0.22: (0.018010, 0.000554)}
TSPM_PRINTED = {0.17: ("0.002 %", "5.8 %"), 0.2: ("0.5 %", "0.9 %"), 0.22: ("1.7 %", "< 0.1 %")}
TSPM_HAT = {0.17: (0.000020, 0.057811), 0.2: (0.004984, 0.009093), 0.22: (0.016862, 0.001148)}
TSPM_EXPL = [
    T("Upper tolerance limit T_U = 0.2 mg/m³ (Israeli 24 h limit for total suspended particulate matter). The prior PDF of the population is lognormal, fitted to 220 results near a stone quarry: µ = −2.325 and σ = 0.434 for ln ρ (Formula 10.1); 95.0 % of the population conforms.",
      "Üst tolerans sınırı T_U = 0,2 mg/m³ (toplam askıda partikül madde için İsrail 24 saatlik sınırı). Popülasyonun önsel dağılımı, bir taş ocağı yakınındaki 220 sonuca uydurulmuş lognormaldir: ln ρ için µ = −2,325 ve σ = 0,434 (Formül 10.1); popülasyonun %95,0'ı uygundur."),
    T("The relative standard uncertainty of every result is 7 %, dominated by the sampled air volume. The measuring-system PDF h(ρ̂ | ρ) is normal with standard deviation 0.07·ρ, proportional to the true value, so that it is a PDF of ρ̂ for every ρ (JCGM 106 9.5.1); global risks follow from JCGM 106 Formulae (19) and (20).",
      "Her sonucun bağıl standart belirsizliği %7'dir; baskın kaynak örneklenen hava hacmidir. Ölçüm sistemi dağılımı h(ρ̂ | ρ), gerçek değerle orantılı 0,07·ρ standart sapmalı normaldir; böylece her ρ için ρ̂'nin bir dağılımıdır (JCGM 106 9.5.1); küresel riskler JCGM 106 Formül (19) ve (20)'den bulunur."),
]
TSPM_NOTE = T("GUM-5 10.6.3.1 uses the standard deviation 0.07·ρ̂ in the joint PDF and normalizes it by C = 1/1.005; that reproduces the printed risks (research log F80, tools/research/gum5/ex10_risks.py). The consumer's (inhabitants') risk is that of accepting air that exceeds the limit, the producer's (quarry owners') risk that of rejecting air that conforms.",
              "GUM-5 10.6.3.1 ortak dağılımda 0,07·ρ̂ standart sapmasını kullanır ve onu C = 1/1,005 ile normalleştirir; bu basılı riskleri yeniden üretir (araştırma günlüğü F80, tools/research/gum5/ex10_risks.py). Tüketici (yöre halkı) riski sınırı aşan havanın kabulü, üretici (ocak sahipleri) riski uygun havanın reddidir.")
TSPM_PROCESS = {"distribution": "lognormal", "meanlog": MU, "sdlog": SG}


def write_cf(pid, title, desc, expl, section, tags, conformity, expected, ref, notes=()):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "chemistry",
                      "sources": [{"document": DOC, "section": section, "url": URL}], "tags": ["gum-5"] + tags,
                      "notes": list(notes)},
         "conformity": conformity,
         "expected": {"conformity": expected, "reference": ref}}
    Path(f"library/chemistry/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


for au, slug in ((0.2, "shared-risk"), (0.17, "acceptance-0.17"), (0.22, "acceptance-0.22")):
    rc, rp = TSPM_GLOBAL[au]
    hc, hp = TSPM_HAT[au]
    pc_, pp_ = TSPM_PRINTED[au]
    shared = au == 0.2
    write_cf(f"gum5-e10-tspm-global-risks-{slug}",
             T(f"Suspended particulate matter in air: global risks with a lognormal prior, A_U = {au} mg/m³ (JCGM GUM-5 Example 10)",
               f"Havada askıda partikül madde: lognormal önselle küresel riskler, A_U = {c(au)} mg/m³ (JCGM GUM-5 Örnek 10)"),
             T(f"T_U = 0.2 mg/m³, lognormal population PDF, relative uncertainty 7 %; {'shared-risk rule A_U = T_U' if shared else 'acceptance limit moved to ' + str(au) + ' mg/m³'}.",
               f"T_U = 0,2 mg/m³, lognormal popülasyon dağılımı, bağıl belirsizlik %7; {'paylaşılan risk kuralı A_U = T_U' if shared else 'kabul sınırı ' + c(au) + ' mg/m³ değerine kaydırıldı'}."),
             TSPM_EXPL + [
                 T(f"R_C = {100 * rc:.4f} % and R_P = {100 * rp:.3f} %. GUM-5 prints {pc_} and {pp_}; its joint PDF gives {100 * hc:.4f} % and {100 * hp:.3f} %.",
                   f"R_C = %{c(f'{100 * rc:.4f}')} ve R_P = %{c(f'{100 * rp:.3f}')}. GUM-5 {pc_.replace('.', ',')} ve {pp_.replace('.', ',')} basar; ortak dağılımı %{c(f'{100 * hc:.4f}')} ve %{c(f'{100 * hp:.3f}')} verir.")],
             "Example 10, 10.6.3", ["conformity-assessment", "global-risk", "lognormal-prior", "air-quality", "jcgm106"],
             {"quantity": "ρ", "unit": "mg/m³", "upper": 0.2, "acceptance_upper": au, "u_relative": 0.07, "process": TSPM_PROCESS},
             {"conforming": chk(0.9504, 1e-4), "consumer_risk": chk(rc, 2e-6), "producer_risk": chk(rp, 2e-6)},
             T(f"JCGM GUM-5:2026, 10.6.3.3–10.6.3.4 print R_C = {pc_}, R_P = {pp_}; the expected values are for the conditional measuring-system PDF (F80).",
               f"JCGM GUM-5:2026, 10.6.3.3–10.6.3.4 R_C = {pc_.replace('.', ',')}, R_P = {pp_.replace('.', ',')} basar; beklenen değerler koşullu ölçüm sistemi dağılımı içindir (F80)."),
             notes=[TSPM_NOTE])

write_cf("gum5-e10-tspm-specific-risks",
         T("Suspended particulate matter in air: specific risks from the posterior with a lognormal prior (JCGM GUM-5 Example 10)",
           "Havada askıda partikül madde: lognormal önselle sonsal dağılımdan özgül riskler (JCGM GUM-5 Örnek 10)"),
         T("Conformance probabilities of single results from the posterior g(ρ | ρ̂) ∝ g₀(ρ)·h(ρ̂ | ρ), lognormal prior, u = 0.07·ρ̂, A_U = T_U = 0.2 mg/m³.",
           "Tek sonuçların uygunluk olasılıkları, sonsal g(ρ | ρ̂) ∝ g₀(ρ)·h(ρ̂ | ρ), lognormal önsel, u = 0,07·ρ̂, A_U = T_U = 0,2 mg/m³."),
         TSPM_EXPL[:1] + [
             T("For a given result the likelihood is normal with standard deviation u = 0.07·ρ̂ (10.6.4.3). The posterior has no closed form and is integrated numerically; p_c = Pr(ρ ≤ T_U | ρ̂).",
               "Verilen bir sonuç için olabilirlik, u = 0,07·ρ̂ standart sapmalı normaldir (10.6.4.3). Sonsal dağılımın kapalı biçimi yoktur ve sayısal olarak integre edilir; p_c = Pr(ρ ≤ T_U | ρ̂)."),
             T("ρ̂ = 0.190 mg/m³ is accepted with a specific consumer's risk 1 − p_c = 14.3 %; ρ̂ = 0.225 mg/m³ is rejected with a specific producer's risk p_c = 11.3 %: an 11 % probability that the true concentration conforms (10.7.2). Without the prior the risks are 22.6 % and 5.6 %; the prior is wide but not negligible near the limit, which lies in its upper tail.",
               "ρ̂ = 0,190 mg/m³ %14,3 özgül tüketici riskiyle (1 − p_c) kabul edilir; ρ̂ = 0,225 mg/m³ %11,3 özgül üretici riskiyle (p_c) reddedilir: gerçek derişimin uygun olma olasılığı %11'dir (10.7.2). Önsel olmadan riskler %22,6 ve %5,6'dir; önsel geniştir, ancak üst kuyruğunda kalan sınırın yakınında ihmal edilemez.")],
         "Example 10, 10.6.4 and 10.7.2", ["conformity-assessment", "specific-risk", "posterior", "lognormal-prior", "air-quality", "jcgm106"],
         {"quantity": "ρ", "unit": "mg/m³", "upper": 0.2, "u_relative": 0.07, "process": TSPM_PROCESS, "posterior": True,
          "results": [{"name": "ρ̂ = 0.190", "value": 0.19}, {"name": "ρ̂ = 0.225", "value": 0.225}]},
         {"results": {"ρ̂ = 0.190": {"conformance_probability": chk(0.857384, 2e-6), "accepted": True},
                      "ρ̂ = 0.225": {"conformance_probability": chk(0.113340, 2e-6), "accepted": False}}},
         T("JCGM GUM-5:2026, 10.7.2: specific producer's risk about 11 % for ρ̂ = 0.225 mg/m³ (Figure 10.3).",
           "JCGM GUM-5:2026, 10.7.2: ρ̂ = 0,225 mg/m³ için özgül üretici riski yaklaşık %11 (Şekil 10.3)."),
         notes=[T("10.6.4 states that the posterior is essentially determined by the likelihood, so frequentist specific risks would be very close. Near T_U this does not hold: the normal PDF alone gives 1 − p_c = 22.6 % at 0.190 and p_c = 5.6 % at 0.225 mg/m³ (research log F80).",
                  "10.6.4 sonsal dağılımın esasen olabilirlikle belirlendiğini, bu nedenle frekansçı özgül risklerin çok yakın olacağını belirtir. T_U yakınında bu geçerli değildir: yalnız normal dağılım 0,190'da 1 − p_c = %22,6, 0,225 mg/m³'te p_c = %5,6 verir (araştırma günlüğü F80).")])

# --- Example 12: between-bottle homogeneity of N₂ in natural gas (Bayesian hierarchical model) -------------
N2 = [[0.424577, 0.425167, 0.425379, 0.424522, 0.424805], [0.425572, 0.425411, 0.423638, 0.425301, 0.424527],
      [0.424152, 0.425517, 0.425638, 0.424207, 0.425135], [0.426320, 0.424672, 0.425211, 0.425533, 0.425864],
      [0.424855, 0.425079, 0.425413, 0.424729, 0.424725], [0.425104, 0.424773, 0.426424, 0.424266, 0.424632],
      [0.425750, 0.424917, 0.424779, 0.425086, 0.425318], [0.425547, 0.426483, 0.424631, 0.425968, 0.424620],
      [0.426326, 0.424646, 0.425205, 0.426302, 0.425020], [0.425968, 0.424069, 0.425988, 0.425489, 0.423936]]
gN, mN = len(N2), len(N2[0])
mi = [sum(r) / mN for r in N2]
gmN = sum(mi) / gN
ssb = mN * sum((v - gmN) ** 2 for v in mi)
ssw = sum((x - mi[i]) ** 2 for i, r in enumerate(N2) for x in r)
msb, msw = ssb / (gN - 1), ssw / (gN * (mN - 1))
ubb = math.sqrt(msw / mN) * (2 / (gN * (mN - 1))) ** 0.25
assert abs(ssb - 2.92e-6) < 0.005e-6 and abs(ssw - 1.96e-5) < 0.005e-5 and abs(msb - 3.25e-7) < 0.005e-7 and abs(msw - 4.90e-7) < 0.005e-7
assert msb < msw and abs(ubb - 1.481e-4) < 1e-7
MU0 = 0.4250
write_pt = lambda pid, title, desc, expl, section, tags, prof, expected, ref, notes=(): Path(f"library/chemistry/{pid}.json").write_text(json.dumps({
    "format": "uncertainty-calculations/project", "version": 1,
    "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "chemistry",
                 "sources": [{"document": DOC, "section": section, "url": URL}], "tags": ["gum-5"] + tags, "notes": list(notes)},
    "proficiency": prof, "expected": {"proficiency": expected, "reference": ref}}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8") or print("wrote", pid)
write_pt("gum5-e12-between-bottle-homogeneity-bayes",
         T("Between-bottle homogeneity of nitrogen in natural gas mixtures: ANOVA, u*_bb and a Bayesian hierarchical model (JCGM GUM-5 Example 12)",
           "Doğal gaz karışımlarında azotun şişeler arası homojenliği: ANOVA, u*_bb ve Bayesçi hiyerarşik model (JCGM GUM-5 Örnek 12)"),
         T("Ten gas mixtures measured five times each; MS_between < MS_within, so the ANOVA gives no between-bottle standard deviation. The Bayesian model with half-Cauchy priors gives a posterior for τ.",
           "Her biri beş kez ölçülen on gaz karışımı; MS_arası < MS_içi olduğundan ANOVA şişeler arası standart sapma vermez. Yarı-Cauchy önselli Bayesçi model τ için bir sonsal dağılım verir."),
         [T(f"One-way ANOVA (Table 12.2): SS_between = {ssb:.3e}, ν = 9, MS_between = {msb:.3e}; SS_within = {ssw:.3e}, ν = 40, MS_within = {msw:.3e} (cmol/mol)². s_bb = √((MS_between − MS_within)/n) does not exist, and ISO 13528 sets s_s = 0.",
            f"Tek yönlü ANOVA (Tablo 12.2): SS_arası = {c(f'{ssb:.3e}')}, ν = 9, MS_arası = {c(f'{msb:.3e}')}; SS_içi = {c(f'{ssw:.3e}')}, ν = 40, MS_içi = {c(f'{msw:.3e}')} (cmol/mol)². s_bb = √((MS_arası − MS_içi)/n) mevcut değildir; ISO 13528 s_s = 0 alır."),
          T(f"The classical answer for this case is the between-unit standard deviation that the repeatability can hide, u*_bb = √(MS_within/n)·(2/ν_within)^¼ = {ubb:.3e} cmol/mol, {100 * ubb / gmN:.3f} % (Linsinger et al. 2001; ISO Guide 35). GUM-5 does not give it.",
            f"Bu durumda klasik yanıt, tekrarlanabilirliğin gizleyebileceği birimler arası standart sapmadır: u*_bb = √(MS_içi/n)·(2/ν_içi)^¼ = {c(f'{ubb:.3e}')} cmol/mol, %{c(f'{100 * ubb / gmN:.3f}')} (Linsinger vd. 2001; ISO Guide 35). GUM-5 bunu vermez."),
          T("Bayesian model y_ij = µ + B_i + ε_ij with µ ~ N(0.4250, (0.025 × 0.4250)²), τ ~ half-Cauchy(0, 0.0015 µ₀) and σ ~ half-Cauchy(0, 0.0020 µ₀), from the specification of the preparation (0.30 % at k = 2) and the known repeatability (0.20 %). The program integrates B_i and µ analytically and evaluates the posterior of (τ, σ) on a grid, where GUM-5 uses Stan (MCMC, 4 × 20 000 draws).",
            "Bayesçi model y_ij = µ + B_i + ε_ij; µ ~ N(0,4250; (0,025 × 0,4250)²), τ ~ yarı-Cauchy(0; 0,0015 µ₀) ve σ ~ yarı-Cauchy(0; 0,0020 µ₀); ölçekler hazırlama şartnamesinden (k = 2'de %0,30) ve bilinen tekrarlanabilirlikten (%0,20) gelir. Program B_i ve µ'yü analitik olarak integre eder ve (τ, σ) sonsal dağılımını bir ızgarada hesaplar; GUM-5 ise Stan (MCMC, 4 × 20 000 çekiliş) kullanır."),
          T("Posterior of τ: mean 0.000136, s.d. 0.000107, median 0.000113, 97.5 % quantile 0.000397 cmol/mol (Stan: 0.00014, 0.00011, —, 0.00040); shortest 95 % interval of τ/µ [0, 0.0008] and of σ/µ [0.0013, 0.0020] as in Table 12.3. The posterior mean of τ (1.36 µmol/mol) is close to u*_bb (1.48 µmol/mol).",
            "τ'nun sonsal dağılımı: ortalama 0,000136, s.s. 0,000107, medyan 0,000113, %97,5 kantili 0,000397 cmol/mol (Stan: 0,00014; 0,00011; —; 0,00040); τ/µ için en kısa %95 aralık [0; 0,0008], σ/µ için [0,0013; 0,0020], Tablo 12.3'teki gibi. τ'nun sonsal ortalaması (1,36 µmol/mol) u*_bb'ye (1,48 µmol/mol) yakındır.")],
         "Example 12, Tables 12.1–12.3, 12.4.5 and 12.4.8", ["homogeneity", "between-unit", "reference-material", "bayesian", "anova", "gas-analysis"],
         {"quantity": "x(N₂)", "unit": "cmol/mol",
          "homogeneity": {"items": N2, "bayes": {"tau_scale": round(0.0015 * MU0, 10), "sigma_scale": round(0.0020 * MU0, 10), "mu_mean": MU0, "mu_sd": round(0.025 * MU0, 10)}}},
         {"homogeneity": {"ms_between": chk(3.25e-7, 0.005e-7), "ms_within": chk(4.90e-7, 0.005e-7), "s_s": chk(0.0, 1e-12), "u_bb_star": chk(round(ubb, 9), 1e-9),
                          "tau_mean": chk(0.00014, 0.000005), "tau_sd": chk(0.00011, 0.000005), "tau_median": chk(0.000113, 1e-6), "tau_q975": chk(0.00040, 0.000005),
                          "tau_mode": chk(0.0, 1e-12), "tau_hpd_upper": chk(round(0.0008 * 0.42514, 7), 0.00005 * 0.42514),
                          "sigma_mean": chk(0.00069, 0.000005), "sigma_hpd_lower": chk(round(0.0013 * 0.42514, 7), 0.00005 * 0.42514),
                          "sigma_hpd_upper": chk(round(0.0020 * 0.42514, 7), 0.00005 * 0.42514), "mu_mean": chk(0.42514, 0.000005)}},
         T("JCGM GUM-5:2026, Table 12.2 (MS 3.25·10⁻⁷ and 4.90·10⁻⁷), 12.4.5 (τ 0.00014 ± 0.00011, [0.00001, 0.00040]; σ 0.00069; µ 0.42514), Table 12.3 (HPD) and 12.4.8 (median 1.1 µmol/mol); u*_bb, the median to 10⁻⁶ and the mode 0 are recomputed here.",
           "JCGM GUM-5:2026, Tablo 12.2 (MS 3,25·10⁻⁷ ve 4,90·10⁻⁷), 12.4.5 (τ 0,00014 ± 0,00011, [0,00001; 0,00040]; σ 0,00069; µ 0,42514), Tablo 12.3 (HPD) ve 12.4.8 (medyan 1,1 µmol/mol); u*_bb, 10⁻⁶'ya kadar medyan ve mod 0 burada yeniden hesaplanmıştır."),
         notes=[T("12.4.8 gives the mode of the posterior of τ as 0.3 µmol/mol. The marginal posterior density of τ is largest at τ = 0 here, because MS_between < MS_within and the half-Cauchy prior is largest at 0. The R function getmode of 12.5.2 tabulates exact repeats of the draws, which for continuous MCMC output are all distinct, so it returns the first draw rather than a mode (research log F81).",
                  "12.4.8, τ'nun sonsal dağılımının modunu 0,3 µmol/mol olarak verir. Burada τ'nun marjinal sonsal yoğunluğu τ = 0'da en büyüktür, çünkü MS_arası < MS_içi'dir ve yarı-Cauchy önsel 0'da en büyüktür. 12.5.2'deki R işlevi getmode çekilişlerin tam tekrarlarını sayar; sürekli MCMC çıktısında hepsi farklı olduğundan mod yerine ilk çekilişi döndürür (araştırma günlüğü F81)."),
                T("Equation (12.5) writes the likelihood as y_ij | µ, τ, σ ~ N(µ, τ² + σ²), as if the observations were independent; the replicates of one bottle share B_i and are correlated. The Stan code of 12.4.2 uses the hierarchical model, which the program also uses.",
                  "Denklem (12.5) olabilirliği gözlemler bağımsızmış gibi y_ij | µ, τ, σ ~ N(µ, τ² + σ²) olarak yazar; bir şişenin tekrarları B_i'yi paylaşır ve ilişkilidir. 12.4.2'deki Stan kodu hiyerarşik modeli kullanır; program da onu kullanır.")])
