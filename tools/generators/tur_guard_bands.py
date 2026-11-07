"""Writes library projects for TUR-based guard bands (RSS, Dobbert's managed guard band, NCSLI RP-10) with a
normal process of 95 % in-tolerance probability. The expected guard-band factors and global risks are those of
Sandia's Suncal 1.7.1 (suncal.risk.guardband_tur and risk.PFA_norm/PFR_norm), an independent implementation;
there is no printed worked example in the documents at hand. Run from the repository root."""
import json
import math
from pathlib import Path

SDP = 0.5102134569246539  # process s.d. for ITP = 95 % with T = ±1: 1/z(0.975)
CASES = [  # method, TUR, Suncal GBF, PFA, PFR, PFA without guard band
    ("dobbert", 2.5, 0.9141856345435068, 0.0068831838402677575, 0.05216244734420719, 0.011767981624822774),
    ("rss", 1.5, 0.7453559924999299, 0.005678650991293455, 0.17700892756509434, 0.015411644495887495),
    ("rp10", 2.0, 0.75, 0.0032131275699658632, 0.14003940375582652, 0.013373408282072386),
]
NAMES = {"dobbert": ("Dobbert's managed guard band (method 6)", "Dobbert'in yönetilen koruma bandı (yöntem 6)"),
         "rss": ("root-sum-square guard band", "karelerin toplamının kökü koruma bandı"),
         "rp10": ("NCSLI RP-10 guard band", "NCSLI RP-10 koruma bandı")}
FORMULAS = {"dobbert": "GBF = 1 − M/TUR, M = 1.04 − exp(0.38 ln TUR − 0.54)", "rss": "GBF = √(1 − 1/TUR²)",
            "rp10": "GBF = 1.25 − 1/TUR (TUR ≤ 4)"}


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


for m, tur, gbf, pfa, pfr, pfa0 in CASES:
    u = 1 / (2 * tur)  # Suncal: test s.d. = SL/(2 TUR)
    assert abs(1 / (2 * u) - tur) < 1e-12
    en, tr = NAMES[m]
    pid = f"tur-guard-band-{m}-tur{str(tur).replace('.', '-')}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "field": "general",
                      "title": T(f"Acceptance limits from the test uncertainty ratio: {en}, TUR = {tur}, global risks for a 95 % in-tolerance process",
                                 f"Test belirsizlik oranından kabul sınırları: {tr}, TUR = {c(tur)}, %95 tolerans içi süreç için küresel riskler"),
                      "description": T(f"Tolerance ±1 (normalised), U₉₅ = 2u = {2 * u:.4g}, so TUR = 1/U₉₅ = {tur}. {FORMULAS[m]}. The process of the items is normal about the centre with 95 % of it within tolerance (s.d. {SDP:.6f}).",
                                       f"Tolerans ±1 (normalize), U₉₅ = 2u = {c(f'{2 * u:.4g}')}, dolayısıyla TUR = 1/U₉₅ = {c(tur)}. {FORMULAS[m].replace('.', ',')}. Ürünlerin süreci merkez etrafında normaldir ve %95'i tolerans içindedir (s.s. {c(f'{SDP:.6f}')})."),
                      "explanation": [T(f"Guard-band factor GBF = {gbf:.6f}: the acceptance limits are ±{gbf:.6f}, a guard band of {1 - gbf:.6f} inside each tolerance limit.",
                                        f"Koruma bandı çarpanı GBF = {c(f'{gbf:.6f}')}: kabul sınırları ±{c(f'{gbf:.6f}')}; her tolerans sınırının {c(f'{1 - gbf:.6f}')} içinde bir koruma bandı."),
                                      T(f"Global consumer's risk (probability of false acceptance) {100 * pfa:.3f} % instead of {100 * pfa0:.3f} % with simple acceptance; producer's risk {100 * pfr:.2f} %.",
                                        f"Küresel tüketici riski (yanlış kabul olasılığı) basit kabuldeki %{c(f'{100 * pfa0:.3f}')} yerine %{c(f'{100 * pfa:.3f}')}; üretici riski %{c(f'{100 * pfr:.2f}')}.")]
                                     + ([T("Dobbert's method keeps the global PFA at or below about 2 % whatever the in-tolerance probability: for TUR = 2.5 its maximum over ITP is 1.93 % (Suncal).",
                                           "Dobbert yöntemi, tolerans içi olasılık ne olursa olsun küresel PFA'yı yaklaşık %2'nin altında tutar: TUR = 2,5 için ITP üzerindeki en büyük değeri %1,93'tür (Suncal).")] if m == "dobbert" else []),
                      "tags": ["guard-band", "tur", "pfa", "calibration", "z540.3", m], "notes": [
                          T("TUR is defined with the 95 % expanded uncertainty U₉₅ = 2u, as in ANSI/NCSL Z540.3 practice and Suncal; JCGM 106 uses the capability index C_m = T/(4u), equal to TUR here.",
                            "TUR, ANSI/NCSL Z540.3 uygulamasında ve Suncal'da olduğu gibi %95 genişletilmiş belirsizlik U₉₅ = 2u ile tanımlanır; JCGM 106 ölçüm yeterlilik indeksini C_m = T/(4u) kullanır, burada TUR'a eşittir.")],
                      "sources": [{"document": "Sandia National Laboratories, Suncal 1.7.1 (suncal.risk.guardband_tur, risk.PFA_norm and PFR_norm)", "url": "https://pypi.org/project/suncal/"},
                                  {"document": "Delker & Peguero, Calculating guardbands for calibration and test (Sandia report, 2022)", "url": "https://www.osti.gov/biblio/2004392"}]},
         "conformity": {"quantity": "Normalised deviation from nominal", "lower": -1, "upper": 1, "u": u, "tur_method": m,
                        "process": {"distribution": "normal", "mean": 0, "sd": SDP},
                        "results": [{"name": "reading 0.70", "value": 0.70}, {"name": "reading 0.95", "value": 0.95}]},
         "expected": {"conformity": {"tur": chk(tur, 1e-12), "guard_band_factor": chk(gbf, 1e-9),
                                     "acceptance_lower": chk(-gbf, 1e-9), "acceptance_upper": chk(gbf, 1e-9),
                                     "consumer_risk": chk(pfa, 2e-6), "producer_risk": chk(pfr, 2e-6)},
                      "reference": T("Suncal 1.7.1: guard-band factor and PFA_norm/PFR_norm(itp = 0.95, TUR, GB) for the global risks.",
                                     "Suncal 1.7.1: koruma bandı çarpanı ve küresel riskler için PFA_norm/PFR_norm(itp = 0,95, TUR, GB).")}}
    Path(f"library/general/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)
