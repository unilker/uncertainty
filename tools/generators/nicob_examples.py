"""Writes two library projects from the examples of the NIST Consensus Builder user manual (2020 Nov 12):
§4.4 activity of 60Co (19 laboratories, BIPM.RI(II)-K1.Co-60) with leave-one-out and bilateral degrees of
equivalence, and §4.2 PCB 28 in sediment (6 laboratories with degrees of freedom) with the Linear Pool.
Run from the repository root.

The printed values (Exhibits 7 and 21) are the DerSimonian-Laird bootstrap and the Linear Pool. The degrees of
equivalence are not printed in numbers; their reference values come from an independent numpy port of the
NICOB R source with 10^6 draws (tools/research/consensus/nicob_doe.py, nicob_doe.json; research log D86).
Participants are coded Lab 1, Lab 2, ... in the order of the manual (D81): no degree of equivalence is
attributed to a named institute.
"""
import json
from pathlib import Path

MANUAL = "NIST Consensus Builder user manual (Koepke, Lafarge, Possolo & Toman, 2020 Nov 12)"
URL = "https://consensus.nist.gov"
REF = json.loads(Path("tools/research/consensus/nicob_doe.json").read_text(encoding="utf-8"))


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


def doe(ver, data, names, codes, rel=0.03, tol_d=None):
    """Expected (d, U) from the numpy reference: d exact (or within tol_d when d itself is a Monte Carlo mean, as
    for the Linear Pool), U within `rel` (Monte Carlo, 10^5 draws in the program)."""
    out = {}
    for n in names:
        d, u = data[n] if "|" not in n else data[n]
        key = codes[n] if "|" not in n else "|".join(codes[a] for a in n.split("|"))
        td = tol_d if tol_d is not None else 1e-6 * max(1.0, abs(d))
        out[key] = {"d": chk(round(d, 9), td), "expanded": chk(round(u, 6), round(rel * u, 6))}
    return out


def write(folder, pid, meta, consensus, expected, ref):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, **meta},
         "consensus": consensus,
         "expected": {"consensus": expected, "reference": ref}}
    Path(f"library/{folder}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


# --- 60Co (§4.4, Exhibits 19, 21, 22) ---------------------------------------------------------------------
CO_LABS = "LNMRI ENEA ANSTO KRISS MKEH LNE-LNHB CIEMAT NPL IRA BARC PTB NMISA CNEA RC NMIJ IRMM IFIN-HH NIST BEV".split()
CO_X = [7077, 7065, 7056, 7047, 7051, 7060, 7090, 7053, 7037, 7099, 7057, 7098, 7050, 7040, 7050, 7039, 7101, 7083, 7057]
CO_U = [8, 26, 10, 22, 18, 4, 11, 21, 8, 46, 16, 16, 15, 40, 8, 17, 24, 14, 17]
co = {n: f"Lab {i}" for i, n in enumerate(CO_LABS, 1)}
r = REF["co60"]
sig_loo = [k for k, (b, u) in r["loo"][1].items() if abs(b) > u]
pick_loo = ["LNMRI", "LNE-LNHB", "CIEMAT", "IRA", "NMISA", "IFIN-HH"]
pick_bil = sig_loo[:4] + ["LNMRI|ENEA"]
expected = {"mean": chk(7062, 0.5), "bootstrap_u": chk(4, 0.6), "interval_lower": chk(7053, 0.6), "interval_upper": chk(7071, 0.6),
            "linear_pool_mean": chk(7064, 1.0), "linear_pool_u": chk(29, 0.7),
            "linear_pool_lower": chk(7012, 1.5), "linear_pool_upper": chk(7127, 1.5),
            "leave_one_out": doe("loo", r["loo"][0], pick_loo, co),
            "bilateral": doe("loo", r["loo"][1], pick_bil, co),
            # Exhibit 21, hierarchical Bayesian (σ_j = u_j): 7062, 5, (7053, 7072); MCMC, so the printed digit plus
            # its sampling error.
            "bayes_mean": chk(7062, 0.5), "bayes_u": chk(5, 0.5), "bayes_lower": chk(7053, 0.8), "bayes_upper": chk(7072, 0.8),
            # Linear Pool and Bayes degrees of equivalence, leave-one-out (numpy port; D86b). The Linear Pool D is a
            # difference to a Monte Carlo mean: tolerance 5σ of that mean with 10^5 draws (29/√10^5 kBq).
            "doe": {"linear_pool": {"unilateral": doe("lp", r["lp_loo"][0], pick_loo, co, tol_d=0.5),
                                    "bilateral": doe("lp", r["lp_loo"][1], ["LNMRI|IRA", "CIEMAT|IRA"], co, tol_d=0.7)},
                    "bayes": {"unilateral": doe("bayes", r["bayes_loo"][0], pick_loo, co),
                              "bilateral": doe("bayes", r["bayes_loo"][1], [k for k, (b, u) in r["bayes_loo"][1].items() if abs(b) > 1.05 * u][:4], co)}}}
write("physical", "nicob-co60-activity-leave-one-out-bilateral",
      {"field": "physical",
       "title": T("Activity of ⁶⁰Co in 19 laboratories: consensus value with leave-one-out and bilateral degrees of equivalence (NIST Consensus Builder §4.4)",
                  "19 laboratuvarda ⁶⁰Co aktivitesi: birini dışarıda bırakan ve iki taraflı eşdeğerlik dereceleriyle uzlaşı değeri (NIST Consensus Builder §4.4)"),
       "description": T("SIR equivalent activities of ⁶⁰Co from the ongoing key comparison BIPM.RI(II)-K1.Co-60, as listed in the NICOB manual (Exhibit 19). Cochran's Q = 36.9 with 18 degrees of freedom (p = 0.005): the results are more dispersed than their uncertainties, and the dark uncertainty is τ̂ = 11.9 kBq.",
                        "Süren anahtar karşılaştırma BIPM.RI(II)-K1.Co-60'tan, NICOB kılavuzunda (Örnek 19) listelenen ⁶⁰Co SIR eşdeğer aktiviteleri. Cochran Q = 36,9, 18 serbestlik derecesi (p = 0,005): sonuçlar belirsizliklerinden daha dağınıktır; karanlık belirsizlik τ̂ = 11,9 kBq'dir."),
       "explanation": [T("The degrees of equivalence are computed in the leave-one-out version D*_j = x_j − μ̂₋ⱼ (NICOB §3.2.8): μ̂₋ⱼ is the DerSimonian–Laird value of the other 18 results, so a result does not pull the reference value towards itself. U95 %(D*) comes from draws x_j + e − μ₋ⱼ with μ₋ⱼ from the modified Knapp–Hartung t approximation and e ~ N(0, τ²₋ⱼ + u_j²).",
                         "Eşdeğerlik dereceleri birini dışarıda bırakan biçimde hesaplanır, D*_j = x_j − μ̂₋ⱼ (NICOB §3.2.8): μ̂₋ⱼ diğer 18 sonucun DerSimonian–Laird değeridir; böylece bir sonuç referans değeri kendine doğru çekmez. U95 %(D*), μ₋ⱼ değiştirilmiş Knapp–Hartung t yaklaşımından ve e ~ N(0, τ²₋ⱼ + u_j²) olmak üzere x_j + e − μ₋ⱼ çekilişlerinden gelir."),
                       T(f"Bilateral degrees of equivalence B_ij = D*_i − D*_j use the same draws; {len(sig_loo)} of the 171 pairs differ significantly from zero (|B| > U95 %), against 4 in the MRA version D_j = x_j − μ̂.",
                         f"İki taraflı eşdeğerlik dereceleri B_ij = D*_i − D*_j aynı çekilişleri kullanır; 171 çiftten {len(sig_loo)}'u sıfırdan anlamlı biçimde farklıdır (|B| > U95 %); MRA biçiminde (D_j = x_j − μ̂) bu sayı 4'tür.")],
       "tags": ["nicob", "consensus", "key-comparison", "degrees-of-equivalence", "leave-one-out", "bilateral", "radionuclide"],
       "notes": [T("Participants are coded Lab 1 to Lab 19 in the order of Exhibit 19 of the manual. The official reference value and degrees of equivalence of BIPM.RI(II)-K1.Co-60 are those of the KCDB; the degrees of equivalence here illustrate the procedures on the same data.",
                   "Katılımcılar kılavuzun 19. örneğindeki sırayla Lab 1 – Lab 19 olarak kodlanmıştır. BIPM.RI(II)-K1.Co-60'ın resmî referans değeri ve eşdeğerlik dereceleri KCDB'dekilerdir; buradaki eşdeğerlik dereceleri yöntemleri aynı veri üzerinde gösterir."),
                 T("The degrees of equivalence are shown only in a figure in the manual (Exhibit 22); their reference values here come from a numpy port of the NICOB R source with 10⁶ draws (research log D86). The program, with 10⁵ draws, agrees within 1 % in U.",
                   "Kılavuzda eşdeğerlik dereceleri yalnızca bir şekilde (Örnek 22) gösterilir; buradaki referans değerleri NICOB R kaynağının 10⁶ çekilişli numpy aktarımından gelir (araştırma günlüğü D86). 10⁵ çekilişli program U'da %1 içinde uyuşur.")],
       "sources": [{"document": MANUAL, "section": "§4.4, Exhibits 19, 21 and 22; §A.5.1", "url": URL}]},
      {"measurand": "A (SIR equivalent activity)", "unit": "kBq",
       "results": [{"name": co[n], "value": x, "u": u} for n, x, u in zip(CO_LABS, CO_X, CO_U)],
       "linear_pool": True, "bayes": {}, "leave_one_out": True, "bilateral": True, "trials": 100000, "seed": 1},
      expected,
      T("NICOB manual Exhibit 21: DerSimonian–Laird 7062 kBq, u 4 kBq, (7053, 7071) kBq; Linear Pool 7064, 29, (7012, 7127) kBq (tolerances: the printed digit and the Monte Carlo dispersion). Degrees of equivalence: numpy port of the NICOB source, 10⁶ draws; tolerance 3 % of U95 % (5σ of the program's 10⁵ draws).",
        "NICOB kılavuzu Örnek 21: DerSimonian–Laird 7062 kBq, u 4 kBq, (7053, 7071) kBq; Linear Pool 7064, 29, (7012, 7127) kBq (toleranslar: basılı basamak ve Monte Carlo saçılımı). Eşdeğerlik dereceleri: NICOB kaynağının numpy aktarımı, 10⁶ çekiliş; tolerans U95 %'in %3'ü (programın 10⁵ çekilişinin 5σ'sı)."))

# --- PCB 28 (§4.2, Exhibits 6 and 7) ----------------------------------------------------------------------
PCB_LABS = "IRMM KRISS NARL NIST NMIJ NRC".split()
PCB_X = [34.30, 32.90, 34.53, 32.42, 31.90, 35.80]
PCB_U = [1.03, 0.69, 0.83, 0.29, 0.40, 0.38]
PCB_NU = [60, 4, 18, 2, 13, 60]
pc = {n: f"Lab {i}" for i, n in enumerate(PCB_LABS, 1)}
r = REF["pcb28"]
sig = [k for k, (b, u) in r["loo"][1].items() if abs(b) > u]
expected = {"mean": chk(33.6, 0.05), "bootstrap_u": chk(0.77, 0.03), "interval_lower": chk(32.0, 0.1), "interval_upper": chk(35.2, 0.1),
            "linear_pool_mean": chk(33.6, 0.1), "linear_pool_u": chk(1.55, 0.1),
            "linear_pool_lower": chk(31.4, 0.1), "linear_pool_upper": chk(36.3, 0.1),
            "leave_one_out": doe("loo", r["loo"][0], PCB_LABS, pc),
            "bilateral": doe("loo", r["loo"][1], sig + ["IRMM|KRISS"], pc),
            "doe": {"linear_pool": {"unilateral": doe("lp", r["lp_loo"][0], PCB_LABS, pc, tol_d=0.03)}}}
write("chemistry", "nicob-pcb28-sediment-degrees-of-freedom",
      {"field": "chemistry",
       "title": T("PCB 28 in sediment from six laboratories with degrees of freedom: DerSimonian–Laird, Linear Pool and leave-one-out degrees of equivalence (NIST Consensus Builder §4.2)",
                  "Serbestlik dereceli altı laboratuvardan sedimentte PCB 28: DerSimonian–Laird, Linear Pool ve birini dışarıda bırakan eşdeğerlik dereceleri (NIST Consensus Builder §4.2)"),
       "description": T("Mass fraction of PCB 28 (2,4,4'-trichlorobiphenyl) in a sediment, measured by six laboratories that state the degrees of freedom of their standard uncertainties (Exhibit 6). One ν is 2: its Student t distribution has no finite variance, and NICOB rescales it with an approximation (DoEUnilateralDerSimonianLaird.R, linearOP.R), which the program follows.",
                        "Bir sedimentte PCB 28 (2,4,4'-triklorobifenil) kütle kesri; altı laboratuvar standart belirsizliklerinin serbestlik derecelerini bildirir (Örnek 6). Bir ν değeri 2'dir: Student t dağılımının sonlu varyansı yoktur ve NICOB onu bir yaklaşımla yeniden ölçekler (DoEUnilateralDerSimonianLaird.R, linearOP.R); program bunu izler."),
       "explanation": [T("The bootstrap draws u_j,k = u_j √(ν_j/χ²_νj) for each result (NICOB §A.2 step c); the Linear Pool mixes rescaled t distributions with means x_j and standard deviations u_j.",
                         "Bootstrap her sonuç için u_j,k = u_j √(ν_j/χ²_νj) çeker (NICOB §A.2 adım c); Linear Pool ortalamaları x_j, standart sapmaları u_j olan yeniden ölçeklenmiş t dağılımlarını karıştırır."),
                       T(f"Leave-one-out degrees of equivalence with t-distributed deviations e for the results with finite ν; {len(sig)} of the 15 bilateral pairs differ significantly from zero.",
                         f"Sonlu ν'li sonuçlar için t dağılımlı sapmalarla birini dışarıda bırakan eşdeğerlik dereceleri; 15 iki taraflı çiftten {len(sig)}'i sıfırdan anlamlı biçimde farklıdır.")],
       "tags": ["nicob", "consensus", "degrees-of-equivalence", "leave-one-out", "bilateral", "degrees-of-freedom", "linear-pool"],
       "notes": [T("Participants are coded Lab 1 to Lab 6 in the order of Exhibit 6.", "Katılımcılar 6. örnekteki sırayla Lab 1 – Lab 6 olarak kodlanmıştır."),
                 T("The printed bootstrap u, 0.77 ng/g, is a Monte Carlo estimate: an independent numpy evaluation with 10⁶ replicates gives 0.752 ng/g (the program 0.751), and subsamples of 10⁴ replicates spread from 0.739 to 0.767 ng/g; the tolerance includes this dispersion. With ν = 2 the Linear Pool has heavy tails and its sample standard deviation is unstable (printed 1.55, program 1.51 ng/g).",
                   "Basılı bootstrap u, 0,77 ng/g, bir Monte Carlo tahminidir: 10⁶ tekrarlı bağımsız numpy hesabı 0,752 ng/g (program 0,751) verir ve 10⁴ tekrarlı alt örnekler 0,739 ile 0,767 ng/g arasında dağılır; tolerans bu saçılımı içerir. ν = 2 ile Linear Pool'un kuyrukları kalındır ve örnek standart sapması kararsızdır (basılı 1,55, program 1,51 ng/g)."),
                 T("The hierarchical Bayesian row of Exhibit 7 (33.6, 0.79 ng/g) uses priors for the σ_j with the stated ν_j, which the program does not implement (research log D68).",
                   "Örnek 7'deki hiyerarşik Bayes satırı (33,6, 0,79 ng/g) belirtilen ν_j ile σ_j için önseller kullanır; program bunu uygulamaz (araştırma günlüğü D68).")],
       "sources": [{"document": MANUAL, "section": "§4.2, Exhibits 6 and 7; §A.2, §A.4, §A.5.1", "url": URL}]},
      {"measurand": "w(PCB 28)", "unit": "ng/g",
       "results": [{"name": pc[n], "value": x, "u": u, "dof": nu} for n, x, u, nu in zip(PCB_LABS, PCB_X, PCB_U, PCB_NU)],
       "linear_pool": True, "leave_one_out": True, "bilateral": True, "trials": 100000, "seed": 1},
      expected,
      T("NICOB manual Exhibit 7: DerSimonian–Laird 33.6 ng/g, u 0.77, (32.0, 35.2); Linear Pool 33.6, 1.55, (31.4, 36.3) ng/g (tolerances: printed digit and Monte Carlo dispersion, see the notes). Degrees of equivalence: numpy port of the NICOB source, 10⁶ draws; tolerance 3 % of U95 %.",
        "NICOB kılavuzu Örnek 7: DerSimonian–Laird 33,6 ng/g, u 0,77, (32,0, 35,2); Linear Pool 33,6, 1,55, (31,4, 36,3) ng/g (toleranslar: basılı basamak ve Monte Carlo saçılımı, notlara bakınız). Eşdeğerlik dereceleri: NICOB kaynağının numpy aktarımı, 10⁶ çekiliş; tolerans U95 %'in %3'ü."))
