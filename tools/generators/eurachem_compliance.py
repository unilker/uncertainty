"""Writes the library projects of Eurachem/CITAC Guide "Use of uncertainty information in compliance
assessment" (2nd ed., 2021), Annex B Examples 1–3 (run from the repository root). Printed values are
recomputed here and the agreement asserted; the guide rounds k to 1.64 and the limits to the decimals shown.
"""
import json
import math
from pathlib import Path
from statistics import NormalDist

DOC = "Eurachem/CITAC Guide: Use of uncertainty information in compliance assessment, 2nd ed. (2021)"
URL = "https://www.eurachem.org/index.php/publications/guides/uncertcompliance"
K95 = NormalDist().inv_cdf(0.95)


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


def write(pid, title, desc, expl, section, tags, cf, expected, ref, notes=()):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "chemistry",
                      "sources": [{"document": DOC, "section": section, "url": URL}], "tags": ["eurachem-compliance", "conformity-assessment"] + tags,
                      "notes": list(notes)},
         "conformity": cf, "expected": {"conformity": expected, "reference": ref}}
    Path(f"library/chemistry/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


# Example 1: nickel in stainless steel, 16.0–18.0 %, U = 0.2 % (k = 2), high confidence of correct acceptance.
lo1, hi1 = 16.0 + K95 * 0.1, 18.0 - K95 * 0.1
assert round(lo1, 1) == 16.2 and round(hi1, 1) == 17.8 and round(1.64 * 0.1, 3) == 0.164
write("eurachem-compliance-b1-nickel-steel",
      T("Nickel in stainless steel: guarded acceptance with a two-sided specification (Eurachem/CITAC compliance guide, Example B1)",
        "Paslanmaz çelikte nikel: iki taraflı şartnamede korumalı kabul (Eurachem/CITAC uygunluk rehberi, Örnek B1)"),
      T("Specification 16.0–18.0 % Ni, U = 0.2 % (k = 2); acceptance zone where conformity is at least 95 % probable; measured 16.1 % Ni.",
        "Şartname %16,0–18,0 Ni, U = %0,2 (k = 2); uygunluğun en az %95 olası olduğu kabul bölgesi; ölçülen %16,1 Ni."),
      [T("Case 1b of Annex A: the expanded uncertainty is divided by its k = 2, u = 0.1 % Ni, and each guard band is k·u with the one-sided 95 % normal quantile k = 1.645.",
         "Ek A Durum 1b: genişletilmiş belirsizlik k = 2'ye bölünür, u = %0,1 Ni; her koruma bandı, tek taraflı %95 normal kantili k = 1,645 ile k·u'dur."),
       T(f"Acceptance zone {lo1:.3f}–{hi1:.3f} % Ni (printed 16.2–17.8 after rounding). 16.1 % Ni lies in the rejection zone: p_c = 84 %, the batch is non-compliant; with simple acceptance it would be compliant.",
         f"Kabul bölgesi %{c(f'{lo1:.3f}')}–{c(f'{hi1:.3f}')} Ni (yuvarlamadan sonra basılı 16,2–17,8). %16,1 Ni ret bölgesindedir: p_c = %84, parti uygun değildir; basit kabulde uygun olurdu.")],
      "Annex B, Example 1; Annex A, Case 1b", ["guarded-acceptance", "two-sided", "metals"],
      {"quantity": "w(Ni)", "unit": "%", "lower": 16.0, "upper": 18.0, "rule": "guarded_acceptance", "probability": 0.95, "u": 0.1,
       "results": [{"name": "batch", "value": 16.1}]},
      {"acceptance_lower": chk(round(lo1, 4), 1e-4), "acceptance_upper": chk(round(hi1, 4), 1e-4),
       "results": {"batch": {"accepted": False, "conformance_probability": chk(round(NormalDist(16.1, 0.1).cdf(18) - NormalDist(16.1, 0.1).cdf(16), 4), 1e-4)}}},
      T("Eurachem/CITAC 2021, Example B1: acceptance zone 16.2–18.0 → 16.2 % to 17.8 % Ni after rounding; 16.1 % Ni non-compliant.",
        "Eurachem/CITAC 2021, Örnek B1: yuvarlamadan sonra kabul bölgesi %16,2–17,8 Ni; %16,1 Ni uygun değil."),
      notes=[T("The guide gives the guard band as 1.64u ≈ 0.17 %; 1.64 × 0.1 = 0.164 rounds to 0.16 % (research log F86). The rounded acceptance limits 16.2 and 17.8 % correspond to a probability of 97.7 %, not 95 %.",
               "Rehber koruma bandını 1,64u ≈ %0,17 olarak verir; 1,64 × 0,1 = 0,164, %0,16'ya yuvarlanır (araştırma günlüğü F86). Yuvarlanmış 16,2 ve %17,8 kabul sınırları %95'e değil %97,7 olasılığa karşılık gelir.")])

# Example 2: analyte below 200 ng/g, u = 2.2 ng/g with ν = 8, high confidence of correct rejection.
k8 = 1.8595480375228421  # t_0.95(8)
assert round(k8, 2) == 1.86 and round(k8 * 2.2, 1) == 4.1
write("eurachem-compliance-b2-upper-limit-t",
      T("Analyte below an upper limit, t distribution with ν = 8: guarded rejection (Eurachem/CITAC compliance guide, Example B2)",
        "Üst sınırın altındaki analit, ν = 8 t dağılımı: korumalı ret (Eurachem/CITAC uygunluk rehberi, Örnek B2)"),
      T("Upper limit 200 ng/g, u = 2.2 ng/g with 8 degrees of freedom; non-compliant only if the probability above the limit exceeds 95 %; measured 203.7 ng/g.",
        "Üst sınır 200 ng/g, u = 8 serbestlik dereceli 2,2 ng/g; yalnızca sınırın üstünde olma olasılığı %95'i aşarsa uygun değil; ölçülen 203,7 ng/g."),
      [T("Case 2 of Annex A: the guard band is t₀.₉₅(8)·u = 1.860 × 2.2 = 4.09 ng/g, so the acceptance limit is 204.09 ng/g.",
         "Ek A Durum 2: koruma bandı t₀,₉₅(8)·u = 1,860 × 2,2 = 4,09 ng/g; kabul sınırı 204,09 ng/g'dır."),
       T("203.7 ng/g lies below the acceptance limit: the probability of exceeding 200 ng/g is 93.4 %, below 95 %, and the batch is compliant. Simple acceptance would declare it non-compliant.",
         "203,7 ng/g kabul sınırının altındadır: 200 ng/g'ı aşma olasılığı %93,4, %95'in altındadır; parti uygundur. Basit kabul onu uygun değil sayardı.")],
      "Annex B, Example 2; Annex A, Case 2", ["guarded-rejection", "t-distribution", "upper-limit"],
      {"quantity": "w", "unit": "ng/g", "upper": 200.0, "rule": "guarded_rejection", "probability": 0.95, "u": 2.2, "dof": 8,
       "results": [{"name": "batch", "value": 203.7}]},
      {"acceptance_upper": chk(204.09, 0.005), "results": {"batch": {"accepted": True, "conformance_probability": chk(0.06555, 0.00005)}}},
      T("Eurachem/CITAC 2021, Example B2: k = 1.86, g = 4.1 ng/g, acceptance limit 204.1 ng/g; 203.7 ng/g compliant.",
        "Eurachem/CITAC 2021, Örnek B2: k = 1,86, g = 4,1 ng/g, kabul sınırı 204,1 ng/g; 203,7 ng/g uygun."),
      notes=[T("The table of the example states 'the distribution … is assumed to be Normal' but the guard band uses the t distribution with 8 degrees of freedom (research log F86).",
               "Örneğin tablosu 'dağılımın normal olduğu varsayılır' der, ancak koruma bandı 8 serbestlik dereceli t dağılımını kullanır (araştırma günlüğü F86).")])

# Example 3: banned substance, limit 2 ng/g, u_rel = 35 %, lognormal, correct rejection.
UF = math.exp(1.64 * 0.35)
assert round(UF, 2) == 1.78 and round(2 * UF - 2, 1) == 1.6
r_ln = 2 * math.exp(K95 * 0.35)
r_ex = 2 * math.exp(K95 * math.sqrt(math.log1p(0.35 ** 2)))
r_n = 2 * (1 + K95 * 0.35)
assert abs(r_ln - 3.557) < 5e-4 and abs(r_ex - 3.498) < 5e-4 and abs(r_n - 3.151) < 5e-4
for variant, dist, extra, lim, acc in (("lognormal", "lognormal", {}, r_ln, True), ("lognormal-exact", "lognormal", {"exact_sdlog": True}, r_ex, True),
                                       ("normal", "normal", {}, r_n, False)):
    pc = {"lognormal": 1 - NormalDist().cdf(math.log(3.3 / 2) / 0.35),
          "lognormal-exact": 1 - NormalDist().cdf(math.log(3.3 / 2) / math.sqrt(math.log1p(0.35 ** 2))),
          "normal": NormalDist(3.3, 0.7).cdf(2)}[variant]
    label = {"lognormal": ("lognormal PDF, s_G ≈ u_rel", "lognormal dağılım, s_G ≈ u_bağıl"), "lognormal-exact": ("lognormal PDF, exact s_G", "lognormal dağılım, kesin s_G"),
             "normal": ("normal PDF with u = u_rel·L = 0.7 ng/g", "u = u_bağıl·L = 0,7 ng/g ile normal dağılım")}[variant]
    write(f"eurachem-compliance-b3-banned-substance-{variant}",
          T(f"Banned substance at a 2 ng/g limit with u_rel = 35 %: {label[0]} (Eurachem/CITAC compliance guide, Example B3)",
            f"2 ng/g sınırında yasaklı madde, u_bağıl = %35: {label[1]} (Eurachem/CITAC uygunluk rehberi, Örnek B3)"),
          T("Upper limit 2 ng/g, relative standard uncertainty 35 %; non-compliant if the probability above the limit is at least 95 %; measured 3.3 ng/g.",
            "Üst sınır 2 ng/g, bağıl standart belirsizlik %35; sınırın üstünde olma olasılığı en az %95 ise uygun değil; ölçülen 3,3 ng/g."),
          [T("Case 4 of Annex A: for positive quantities from products and ratios a lognormal PDF is appropriate. The acceptance limit for correct rejection is L·U_F with the uncertainty factor U_F = exp(k·s_G), k = 1.645 (Equations 1–3).",
             "Ek A Durum 4: çarpım ve oranlardan gelen pozitif büyüklükler için lognormal dağılım uygundur. Doğru ret için kabul sınırı L·U_F'dir; belirsizlik çarpanı U_F = exp(k·s_G), k = 1,645 (Eşitlik 1–3)."),
           T(f"Acceptance limit {lim:.3f} ng/g; at 3.3 ng/g the probability of exceeding 2 ng/g is {100 * (1 - pc):.1f} % and the sample is {'compliant' if acc else 'non-compliant'}. Normal PDF: 3.151 ng/g; lognormal with s_G = u_rel: 3.557; with exact s_G = √ln(1 + u_rel²) = 0.3398: 3.498 ng/g.",
             f"Kabul sınırı {c(f'{lim:.3f}')} ng/g; 3,3 ng/g'da 2 ng/g'ı aşma olasılığı %{c(f'{100 * (1 - pc):.1f}')} olup numune {'uygundur' if acc else 'uygun değildir'}. Normal dağılım: 3,151 ng/g; s_G = u_bağıl ile lognormal: 3,557; kesin s_G = √ln(1 + u_bağıl²) = 0,3398 ile: 3,498 ng/g.")],
          "Annex B, Example 3; Annex A, Case 4 and Table 1", ["guarded-rejection", "lognormal", "upper-limit", "residues"],
          {"quantity": "w", "unit": "ng/g", "upper": 2.0, "rule": "guarded_rejection", "probability": 0.95, **({"u": 0.7} if dist == "normal" else {"u_relative": 0.35}), "distribution": dist, **extra,
           "results": [{"name": "sample", "value": 3.3}]},
          {"acceptance_upper": chk(round(lim, 4), 1e-4), "results": {"sample": {"accepted": acc, "conformance_probability": chk(round(pc, 4), 1e-4)}}},
          T("Eurachem/CITAC 2021, Example B3: U_F = 1.78, g = 1.6 ng/g, acceptance limit 3.6 ng/g (lognormal), 3.2 ng/g (normal); 3.3 ng/g compliant with the lognormal PDF only.",
            "Eurachem/CITAC 2021, Örnek B3: U_F = 1,78, g = 1,6 ng/g, kabul sınırı 3,6 ng/g (lognormal), 3,2 ng/g (normal); 3,3 ng/g yalnız lognormal dağılımla uygun."),
          notes=([T("L(1 + k·u_rel) takes u at the limit, u = 0.35 × 2 = 0.7 ng/g, which is what this project enters. With the relative uncertainty applied at the measured value, as for the lognormal PDF, a normal PDF needs the acceptance limit L/(1 − k·u_rel) = 4.714 ng/g: at 3.151 ng/g, u = 1.10 ng/g and the probability above 2 ng/g is only 85 %. The lognormal limit L·exp(k·s_G) is exact for a constant s_G (research log F86).",
                    "L(1 + k·u_bağıl), u'yu sınırda alır: u = 0,35 × 2 = 0,7 ng/g; bu proje bunu girer. Bağıl belirsizlik lognormal dağılımdaki gibi ölçülen değerde uygulanırsa normal dağılım L/(1 − k·u_bağıl) = 4,714 ng/g kabul sınırını gerektirir: 3,151 ng/g'da u = 1,10 ng/g ve 2 ng/g'ın üstünde olma olasılığı yalnızca %85'tir. Lognormal sınır L·exp(k·s_G), sabit s_G için kesindir (araştırma günlüğü F86).")] if dist == "normal" else []) + [T("Printed limits are rounded up: 2 × 1.775 = 3.55 → 3.6 and 2(1 + 1.64 × 0.35) = 3.148 → 3.2, whereas 3.148 rounds to 3.1 (research log F86). The decisions are unaffected.",
                   "Basılı sınırlar yukarı yuvarlanmıştır: 2 × 1,775 = 3,55 → 3,6 ve 2(1 + 1,64 × 0,35) = 3,148 → 3,2; oysa 3,148, 3,1'e yuvarlanır (araştırma günlüğü F86). Kararlar etkilenmez.")])
