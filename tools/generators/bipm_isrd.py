"""Writes library projects for the qNMR purity budgets of the BIPM Internal Standard Reference Data documents
(ISRD-01 maleic acid, ISRD-02 dimethyl sulfone, ISRD-06 BTMSB-d4, ISRD-07 DSS-d6, ISRD-08 benzoic acid). The
inputs are the values and standard uncertainties as printed; the checks are the printed purity and combined
standard uncertainty. Run from the repository root. Evidence: tools/research/qnmr/isrd_budgets.py (F97).
"""
import json
import math
import sys
from pathlib import Path

sys.path.insert(0, "tools/research/qnmr")
from isrd_budgets import T as TABLES  # noqa: E402

DOCS = {
    "ISRD-01": ("BIPM, Internal Standard Reference Data for qNMR: Maleic Acid, ISRD-01, Rapport BIPM-2018/01", "https://www.bipm.org/en/publications/rapports-bipm"),
    "ISRD-02": ("BIPM, Internal Standard Reference Data for qNMR: Dimethyl Sulfone, ISRD-02, Rapport BIPM-2018/04", "https://www.bipm.org/en/publications/rapports-bipm"),
    "ISRD-06": ("BIPM, Internal Standard Reference Data for qNMR: 1,4-Bis(trimethylsilyl)benzene-d4, ISRD-06, Rapport BIPM-2019/02", "https://www.bipm.org/en/publications/rapports-bipm"),
    "ISRD-07": ("BIPM, Internal Standard Reference Data for qNMR: Sodium 4,4-dimethyl-4-silapentane-1-sulfonate-d6, ISRD-07, Rapport BIPM-2019/03", "https://www.bipm.org/en/publications/rapports-bipm"),
    "ISRD-08": ("BIPM, Internal Standard Reference Data for qNMR: Benzoic Acid, ISRD-08, Rapport BIPM-2023/02", "https://www.bipm.org/en/publications/rapports-bipm"),
}
NAMES = {"MA": ("maleic acid", "maleik asit"), "DMSO2": ("dimethyl sulfone", "dimetil sülfon"), "DMTP": ("dimethyl terephthalate", "dimetil tereftalat"),
         "BA": ("benzoic acid", "benzoik asit"), "BTMSB-d4": ("BTMSB-d4", "BTMSB-d4"), "DSS-d6": ("DSS-d6", "DSS-d6"), "KHP": ("KHP", "KHP")}
SKIP = {"ISRD-03 T4 MA/KHP D2O": "normalised ratio; the product of the printed values gives 0.99966, printed 0.9995",
        "ISRD-08 T4 MA/BA acetone-d6": "the product of the printed values gives 1.1078, printed 0.9974"}


def T(en, tr):
    return {"en": en, "tr": tr}


def c(x):
    return str(x).replace(".", ",")


def decimals(x):
    s = repr(x)
    return len(s.split(".")[1]) if "." in s else 0


for key, (R, NA, NS, MA, MS, mA, mS, wS, contrib, w_p, uc_p, _nu) in TABLES.items():
    if key in SKIP:
        continue
    doc, table, rest = key.split(" ", 2)
    pair, solvent = rest.split(" ", 1)
    analyte, standard = pair.split("/")
    w = R[0] * NS[0] / NA[0] * MA[0] / MS[0] * mS[0] / mA[0] * wS[0]
    cs = [w / R[0], w / NA[0], w / NS[0], w / MA[0], w / MS[0], w / mA[0], w / mS[0], w / wS[0]]
    uc = math.sqrt(sum((a * b[1]) ** 2 for a, b in zip(cs, (R, NA, NS, MA, MS, mA, mS, wS))))
    # Tolerance of the estimate: rounding of the printed purity and of the printed ratio.
    tol_w = 0.5 * 10 ** -decimals(w_p) + w * 0.5 * 10 ** -decimals(R[0]) / R[0]
    assert abs(w - w_p) <= tol_w, (key, w, w_p)
    # Tolerance of u_c: 1 % or the difference between the program and the printed value (F97), rounded up.
    d = abs(uc - uc_p) / uc_p
    tol_u = max(0.01, math.ceil(100 * d + 0.5) / 100) * uc_p
    an_en, an_tr = NAMES[analyte]
    st_en, st_tr = NAMES[standard]
    pid = f"bipm-{doc.lower()}-qnmr-{analyte.lower()}-{standard.lower()}-{solvent.lower()}".replace("_", "-")
    inp = lambda n, v, unit, en, tr: {"name": n, "unit": unit, "description": T(en, tr), "distribution": {"type": "normal", "mean": v[0], "sd": v[1]}}
    contrib_en = ", ".join(f"{x:.2e}" for x in contrib)
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "field": "chemistry",
                      "title": T(f"Purity of {an_en} by ¹H qNMR with {st_en} as internal standard in {solvent} (BIPM {doc}, {table.replace('T', 'Table ')})",
                                 f"İç standart olarak {st_tr} ile {solvent} içinde ¹H qNMR ile {an_tr} saflığı (BIPM {doc}, Tablo {table[1:]})"),
                      "description": T("Mass fraction of the analyte A by quantitative NMR with an internal standard S (measurement equation 1 of the ISRD): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. The integral ratio is a Type A evaluation (standard deviation of the mean); the other components are Type B.",
                                       "İç standart S ile kantitatif NMR'de analit A'nın kütle kesri (ISRD ölçüm eşitliği 1): w_A = (I_A/I_S)(N_S/N_A)(M_A/M_S)(m_S/m_A) w_S. İntegral oranı A tipi değerlendirmedir (ortalamanın standart sapması); diğer bileşenler B tipidir."),
                      "explanation": [T(f"Purity from the printed values: w_A = {w:.5f} g/g (printed {w_p}); combined standard uncertainty {uc:.3e} g/g (printed {uc_p:.3e}).",
                                        f"Basılı değerlerden saflık: w_A = {c(f'{w:.5f}')} g/g (basılı {c(w_p)}); birleşik standart belirsizlik {c(f'{uc:.3e}')} g/g (basılı {c(f'{uc_p:.3e}')})."),
                                      T("The masses and the purity of the internal standard dominate; the number of ¹H nuclei is given to four decimals (it is corrected for the natural abundance of deuterium and ¹³C).",
                                        "Kütleler ve iç standardın saflığı baskındır; ¹H çekirdek sayısı dört ondalıkla verilmiştir (döteryum ve ¹³C doğal bolluğu için düzeltilmiştir).")],
                      "tags": ["bipm", "isrd", "qnmr", "purity", "organic-reference-material", "internal-standard"],
                      "notes": [T(f"Printed uncertainty components c·u: {contrib_en}. In several tables of the ISRD series the component of the integral ratio exceeds the product of the printed sensitivity coefficient and standard uncertainty; the program uses the printed standard uncertainties (research log F97).",
                                  f"Basılı belirsizlik bileşenleri c·u: {contrib_en}. ISRD serisinin birkaç tablosunda integral oranının bileşeni, basılı duyarlılık katsayısı ile standart belirsizliğin çarpımından büyüktür; program basılı standart belirsizlikleri kullanır (araştırma günlüğü F97).")],
                      "sources": [{"document": DOCS[doc][0], "section": table.replace("T", "Table "), "url": DOCS[doc][1]}]},
         "model": {"output": {"name": "w_A", "unit": "g/g", "expression": "R * (N_S/N_A) * (M_A/M_S) * (m_S/m_A) * w_S",
                              "description": T(f"Mass fraction (purity) of {an_en}", f"{an_tr} kütle kesri (saflık)")},
                   "inputs": [inp("R", R, "1", "Integral ratio I_A/I_S (mean of replicates)", "İntegral oranı I_A/I_S (tekrarların ortalaması)"),
                              inp("N_A", NA, "1", f"Number of ¹H nuclei of the quantified signal of {an_en}", f"{an_tr} nicel sinyalinin ¹H çekirdek sayısı"),
                              inp("N_S", NS, "1", f"Number of ¹H nuclei of the quantified signal of {st_en}", f"{st_tr} nicel sinyalinin ¹H çekirdek sayısı"),
                              inp("M_A", MA, "g/mol", f"Molar mass of {an_en}", f"{an_tr} mol kütlesi"),
                              inp("M_S", MS, "g/mol", f"Molar mass of {st_en}", f"{st_tr} mol kütlesi"),
                              inp("m_A", mA, "mg", f"Mass of {an_en} weighed", f"Tartılan {an_tr} kütlesi"),
                              inp("m_S", mS, "mg", f"Mass of {st_en} weighed", f"Tartılan {st_tr} kütlesi"),
                              inp("w_S", wS, "g/g", f"Purity of the internal standard {st_en}", f"İç standart {st_tr} saflığı")]},
         "analysis": {"gum": {"coverage_factor": 2}, "mcm": None},
         "expected": {"gum": {"estimate": {"value": w_p, "tolerance": round(tol_w, 7)}, "standard_uncertainty": {"value": uc_p, "tolerance": round(tol_u, 7)}},
                      "reference": T(f"BIPM {doc}, {table.replace('T', 'Table ')}: assigned value {w_p} g/g, combined standard uncertainty {uc_p:.3e} g/g (k = 2 for the expanded uncertainty). Tolerance of u: 1 % or the difference to the value recomputed from the printed inputs, {100 * d:.1f} % (F97).",
                                     f"BIPM {doc}, Tablo {table[1:]}: atanan değer {c(w_p)} g/g, birleşik standart belirsizlik {c(f'{uc_p:.3e}')} g/g (genişletilmiş belirsizlik için k = 2). u toleransı: %1 ya da basılı girdilerden yeniden hesaplanan değerle fark, %{c(f'{100 * d:.1f}')} (F97).")}}
    if key == "ISRD-06 T6 BA/BTMSB-d4 CD3OD":
        # R is printed with two significant digits (1.5): the printed purity allows ±0.033 g/g and the check would
        # say nothing; check the recomputation from the printed inputs instead (review 2026-11-08, item 8).
        p["expected"]["gum"]["estimate"] = {"value": round(w, 9), "tolerance": 5e-9}
        p["expected"]["reference"] = T(p["expected"]["reference"]["en"].replace(f"Table 6: assigned value {w_p} g/g,", f"Table 6: assigned value {w_p} g/g; the integral ratio is printed as 1.5 (two significant digits), which allows ±0.033 g/g, so the estimate is checked against the value recomputed from the printed inputs (independent Python, {w:.9f} g/g) instead;"),
                                       p["expected"]["reference"]["tr"].replace(f"Tablo 6: atanan değer {c(w_p)} g/g,", f"Tablo 6: atanan değer {c(w_p)} g/g; integral oranı 1,5 olarak (iki anlamlı basamak) basıldığından ±0,033 g/g'lık bir tolerans gerekir, bu nedenle tahmin basılı girdilerden yeniden hesaplanan değerle (bağımsız Python, {c(f'{w:.9f}')} g/g) karşılaştırılır;"))
        p["metadata"]["notes"].append(T("The ISRD reports 999.8 +0.2/−1.5 mg/g: the interval is truncated at the physical limit of 1000 mg/g. The program reports the symmetric GUM interval; the bound is not part of the model.",
                                        "ISRD 999,8 +0,2/−1,5 mg/g bildirir: aralık 1000 mg/g fiziksel sınırında kesilmiştir. Program simetrik GUM aralığını verir; sınır modelin parçası değildir."))
    Path(f"library/chemistry/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid, f"w {w:.5f}/{w_p}  u {uc:.3e}/{uc_p:.3e} ({100 * d:.1f} %)")
