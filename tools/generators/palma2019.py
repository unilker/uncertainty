"""Writes the Palma et al. (2019) marine-sediment library projects (run from the repository root)."""
import json, math
from pathlib import Path

DOC = "C. Palma, V. Morgado, R. J. N. Bettencourt da Silva, Top-down evaluation of matrix effects uncertainty, Talanta 192 (2019) 278–287"
URL = "https://doi.org/10.1016/j.talanta.2018.09.039"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def c(x): return str(x).replace('.', ',')

# Table 9: w_LOQ, s_r(h)<I>, s'_r(h)<II> %, s_I<I>, s'_I<II> %, u_ME<I>, u'_ME<II> %, u_T %, R_mean %, R_w %, procedure, names
E = [
 ("cr", "Cr", "krom", "chromium", 5, 0.285, 2.85, 0.565, 5.65, 0.818, 8.18, 1.89, 110, 108, "OSPAR", 1.05, 10.5),
 ("cu", "Cu", "bakır", "copper", 5, 0.242, 2.43, 0.248, 2.48, 0.493, 4.92, 1.11, 100, 99.9, "OSPAR", 0.612, 6.13),
 ("li", "Li", "lityum", "lithium", 0.5, 0.0345, 3.45, 0.058, 5.80, 0.100, 10.0, 3.95, 104, 102, "OSPAR", 0.127, 12.7),
 ("mn", "Mn", "mangan", "manganese", 5, 0.179, 1.79, 0.656, 6.56, 0.600, 6.00, 1.50, 99.3, 97.6, "OSPAR", 0.920, 9.20),
 ("ni", "Ni", "nikel", "nickel", 7.5, 0.346, 2.58, 1.13, 7.54, 0.352, 2.35, 1.31, 88.9, 87.7, "EPA 3051A", 1.25, 8.44),
 ("pb", "Pb", "kurşun", "lead", 10, 0.714, 4.95, 0.804, 4.02, 1.53, 7.65, 1.41, 93.1, 91.9, "EPA 3051A", 1.90, 10.1),
 ("zn", "Zn", "çinko", "zinc", 2, 0.108, 2.69, 0.117, 2.92, 0.324, 8.10, 1.45, 103, 102, "OSPAR", 0.365, 9.13),
]
CHI = {"cr": (60.8, 60.9), "cu": (64.7, 64.5), "li": (62.3, 62.4), "mn": (42.6, 42.6), "ni": (7.9, 7.7), "pb": (65.2, 57.6), "zn": (78.7, 78.6)}
for key, sym, tr_name, en_name, wloq, srh, srh2, si, si2, ume, ume2, ut, rmean, rw, proc, uci, ucii in E:
    lim = 2 * wloq
    utr = ut / rw * 100            # u'_T in per cent of the result
    level1, level2 = wloq, 10 * wloq
    total = "total" if proc == "OSPAR" else "acid-extractable"
    total_tr = "toplam" if proc == "OSPAR" else "asitte ekstrakte edilebilir"
    corrected = abs(1 - rw / 100) / (ut / rw) > 2.03
    ui = math.sqrt(srh**2 + si**2 + ume**2 + (utr / 100 * lim)**2)
    uii = math.sqrt(srh2**2 + si2**2 + ume2**2 + utr**2)
    expl = [
      T(f"Two models of the uncertainty: Interval I from the limit of detection to 2w_LOQ = {lim:g} mg/kg (absolute components), Interval II above 2w_LOQ (relative components); {total} {sym} by the {proc} procedure, flame AAS.",
        f"İki belirsizlik modeli: tespit sınırından 2w_LOQ = {c(f'{lim:g}')} mg/kg'a kadar Aralık I (mutlak bileşenler), 2w_LOQ üzerinde Aralık II (bağıl bileşenler); {proc} yöntemiyle {total_tr} {sym}, alevli AAS."),
      T(f"Precision: repeatability of duplicate analyses of real sediments, which includes subsampling, s_r(h) = {srh:g} mg/kg (I) and {srh2:g} % (II) (mean range/1.128), and the intermediate precision of a homogeneous sample, s_I = {si:g} mg/kg (I) and {si2:g} % (II).",
        f"Kesinlik: gerçek sedimentlerin ikili analizlerinin alt örneklemeyi de içeren tekrarlanabilirliği, s_r(h) = {c(f'{srh:g}')} mg/kg (I) ve %{c(f'{srh2:g}')} (II) (ortalama aralık/1,128), ve homojen bir numunenin ara kesinliği, s_I = {c(f'{si:g}')} mg/kg (I) ve %{c(f'{si2:g}')} (II)."),
      T(f"Matrix effects: the recoveries of {sym} in the sediments of the proficiency tests vary more than the intermediate precision explains; s_ME = √(s′(R)² − s_I²) = {ume:g} mg/kg (I) and {ume2:g} % (II) (Formulae 16, 17).",
        f"Matris etkileri: yeterlilik testlerinin sedimentlerinde {sym} geri kazanımları ara kesinliğin açıklayabileceğinden fazla değişir; s_ME = √(s′(R)² − s_I²) = {c(f'{ume:g}')} mg/kg (I) ve %{c(f'{ume2:g}')} (II) (Formül 16, 17)."),
      T(f"Trueness: weighted mean recovery R̄_w = {rw:g} % with u(R̄) = {ut:g} %, i.e. u′_T = {utr:.2f} % of the result; in Interval I it is taken at 2w_LOQ: {utr / 100 * lim:.3g} mg/kg.",
        f"Doğruluk: ağırlıklı ortalama geri kazanım R̄_w = %{c(f'{rw:g}')}, u(R̄) = %{c(f'{ut:g}')}; yani sonucun u′_T = %{c(f'{utr:.2f}')}'si; Aralık I'de 2w_LOQ'da alınır: {c(f'{utr / 100 * lim:.3g}')} mg/kg."),
      T(f"u_c = {ui:.3g} mg/kg in Interval I and u′_c = {uii:.2f} % in Interval II (Formulae 20, 21); U = 2u_c.",
        f"Aralık I'de u_c = {c(f'{ui:.3g}')} mg/kg ve Aralık II'de u′_c = %{c(f'{uii:.2f}')} (Formül 20, 21); U = 2u_c."),
      T(f"Matrix effects contribute {100 * ume**2 / ui**2:.1f} % (I) and {100 * ume2**2 / uii**2:.1f} % (II) of the variance (the variance shares of the report; χ_ME of Table 9: {CHI[key][0]} % and {CHI[key][1]} %).",
        f"Matris etkileri varyansın %{c(f'{100 * ume**2 / ui**2:.1f}')} (I) ve %{c(f'{100 * ume2**2 / uii**2:.1f}')} (II) kadarını oluşturur (raporun varyans payları; Tablo 9'daki χ_ME: %{c(CHI[key][0])} ve %{c(CHI[key][1])})."),
    ]
    assert abs(100 * ume**2 / ui**2 - CHI[key][0]) <= 0.15 and abs(100 * ume2**2 / uii**2 - CHI[key][1]) <= 0.15, (key, 100 * ume**2 / ui**2, 100 * ume2**2 / uii**2)
    notes = [T(f"The project combines the components summarised in Table 9 of the paper; the raw data (all duplicate pairs and proficiency-test results) are in its electronic supplement. The recovery is {'significantly different from 100 % (Formula 10), so results are corrected by 1/R̄_w' if corrected else 'compatible with 100 %, so results are not corrected'}.",
               f"Proje makalenin Tablo 9'da özetlenen bileşenlerini birleştirir; ham veriler (tüm ikili çiftler ve yeterlilik testi sonuçları) makalenin elektronik ekindedir. Geri kazanım {'%100den anlamlı olarak farklıdır (Formül 10), bu nedenle sonuçlar 1/R̄_w ile düzeltilir' if corrected else '%100 ile uyumludur, bu nedenle sonuçlar düzeltilmez'}.")]
    if key == "pb":
        notes.append(T("For lead the text gives u′_P = 6.38 % and Table 9 6.37 %; √((5.583/1.128)² + 4.02²) = 6.38 %. The text gives u_c = 6.04 mg/kg at 60 mg/kg, whereas 10.08 % × 60 mg/kg = 6.05 mg/kg.",
                       "Kurşun için metin u′_P = %6,38, Tablo 9 ise %6,37 verir; √((5,583/1,128)² + 4,02²) = %6,38'dir. Metin 60 mg/kg'da u_c = 6,04 mg/kg verir; oysa %10,08 × 60 mg/kg = 6,05 mg/kg'dır."))
    pid = f"palma2019-{key}-marine-sediment"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid,
                      "title": T(f"{en_name.capitalize()} in marine sediments, matrix effects in a two-interval top-down model (Palma et al. 2019)", f"Deniz sedimentlerinde {tr_name}, iki aralıklı top-down modelde matris etkileri (Palma ve ark. 2019)"),
                      "description": T(f"Palma, Morgado and Bettencourt da Silva (2019), Table 9: uncertainty of {total} {sym} in marine sediments from duplicate analyses, the intermediate precision of a homogeneous sample, the variability of recoveries in 36 proficiency tests (matrix effects) and the weighted mean recovery.",
                                       f"Palma, Morgado ve Bettencourt da Silva (2019), Tablo 9: ikili analizlerden, homojen bir numunenin ara kesinliğinden, 36 yeterlilik testindeki geri kazanım değişkenliğinden (matris etkileri) ve ağırlıklı ortalama geri kazanımdan deniz sedimentlerinde {total_tr} {sym} belirsizliği."),
                      "explanation": expl, "field": "chemistry",
                      "sources": [{"document": DOC, "section": "Table 9" + (", Section 4.1" if key == "pb" else ""), "url": URL}],
                      "tags": ["top-down", "matrix-effects", "two-intervals", "environmental", "proficiency-tests"],
                      "notes": notes},
         "topdown": {"quantity": f"w({sym})", "unit": "mg/kg", "levels": [level1, level2],
                     "precision": [
                        {"name": "s_r(h)⟨I⟩", "data": {"type": "given", "sd": srh}, "range": {"max": lim}},
                        {"name": "s_I⟨I⟩", "data": {"type": "given", "sd": si}, "range": {"max": lim}},
                        {"name": "s′_r(h)⟨II⟩", "data": {"type": "given", "sd_percent": srh2}, "range": {"min": lim}},
                        {"name": "s′_I⟨II⟩", "data": {"type": "given", "sd_percent": si2}, "range": {"min": lim}}],
                     "components": [
                        {"name": "u_ME⟨I⟩", "description": T("matrix effects", "matris etkileri"), "u": ume, "range": {"max": lim}},
                        {"name": "u′_ME⟨II⟩", "description": T("matrix effects", "matris etkileri"), "u_percent": ume2, "range": {"min": lim}},
                        {"name": "u_T⟨I⟩", "description": T("trueness at 2w_LOQ", "2w_LOQ'da doğruluk"), "u": round(utr / 100 * lim, 6), "range": {"max": lim}},
                        {"name": "u′_T⟨II⟩", "description": T("trueness, u(R̄)/R̄_w", "doğruluk, u(R̄)/R̄_w"), "u_percent": round(utr, 6), "range": {"min": lim}}]},
         "reporting": {"significant_digits": 2},
         "expected": {"topdown": {"levels": [
             {"level": level1, "standard_uncertainty": chk(uci, 0.01 if uci >= 1 else 0.001 if uci >= 0.1 else 0.0001)},
             {"level": level2, "relative_standard_percent": chk(ucii, 0.1 if ucii >= 10 else 0.01)}]},
             "reference": T(f"Palma et al. (2019) Table 9: u_c⟨I⟩ = {uci:g} mg/kg, u′_c⟨II⟩ = {ucii:g} %; tolerance one unit of the last printed digit, since the components are themselves rounded.",
                            f"Palma ve ark. (2019) Tablo 9: u_c⟨I⟩ = {c(f'{uci:g}')} mg/kg, u′_c⟨II⟩ = %{c(f'{ucii:g}')}; tolerans basılı son basamağın bir birimidir, çünkü bileşenlerin kendileri de yuvarlanmıştır.")}}
    Path(f"library/chemistry/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")
    print(key, round(ui, 4), uci, round(uii, 3), ucii, corrected)
