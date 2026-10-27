"""Writes the Bettencourt da Silva (2026) fluoranthene library projects (run from the repository root).

Data: Supplementary Material 06 of the paper (workbook "excluding low spiked levels"), sheet "Trueness"
(CF16:CG35, mean recoveries and their standard uncertainties, labels from AG/K) and sheet
"Performance Model" (precision models). Expected values are recomputed here independently.
"""
import json, math, statistics as st
from pathlib import Path

DOC = ("R. J. N. Bettencourt da Silva, User-friendly spreadsheet for implementing international guidance on the "
       "evaluation of measurement uncertainty, Talanta (2026) 130826 (journal pre-proof)")
URL = "https://doi.org/10.1016/j.talanta.2026.130826"
SECTION = "Section 3.2; Supplementary Material 06, sheets Trueness, Performance Model and Analysis sequence"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def c(x): return str(x).replace('.', ',')

# (label, R, u(R)) in the order of the sheet "Trueness"
TESTS = [
    ("SS95(SP1)", 0.8983819416699959, 0.750619585260691),
    ("SS95(SP2)", 0.7819640431482221, 0.040965526347019485),
    ("SS95(SP3)", 0.6802716739912105, 0.035401382451313135),
    ("CRM1", 0.7948430493273542, 0.020736846237952872),
    ("SS68(SP2)", 0.8363339992009587, 0.070494293440818),
    ("SS68(SP3)", 0.7746184578505793, 0.053085239335122845),
    ("SS87(SP2)", 0.860450259688374, 0.06453888827046286),
    ("SS87(SP3)", 0.8547744706352377, 0.04759435077194429),
    ("SS91(SP2)", 0.6938613663603672, 0.05940711681079574),
    ("SS91(SP3)", 0.7067271274470633, 0.058358660496067985),
    ("SS114(SP2)", 0.5318937275269674, 0.08855175405256016),
    ("SS114(SP3)", 0.683726727926488, 0.06437258472643949),
    ("SS130(SP2)", 0.6867630842988413, 0.08080282510234493),
    ("SS130(SP3)", 0.798721534159009, 0.0629252779251703),
    ("SS132(SP2)", 0.7842728725529363, 0.10998085057955843),
    ("SS132(SP3)", 0.7978827407111466, 0.0746533360185201),
    ("SS150(SP2)", 0.8548030363563722, 0.09392187225699777),
    ("SS150(SP3)", 0.8213431881741907, 0.06644183442891737),
    ("SS50(SP2)", 0.7398333333333331, 0.02840439225512317),
    ("SS50(SP3)", 0.6799526068717537, 0.022755075271632588),
]
S_I_ABS = 0.0751847213412569      # interval I, complex model a = 1.5 mg/kg × s'_I (mg/kg)
S_I_REL = 0.050123147560838       # interval II, s'_I
SPLIT, TOP = 1.5, 10.01
NU, K = 19, 2.093024054408263     # minimum ν (recoveries, N − 1); t(0.975, 19)
# spreadsheet results (sheet "Analysis sequence", rows "Sample 0.1 [1]" and "Sample 10 [1]")
SHEET = {"R": 0.748871, "u": 0.096291, 0.1: (0.133534, 0.161415), 10: (13.353432, 3.857127)}

R = [t[1] for t in TESTS]; U = [t[2] for t in TESTS]; N = len(R)
W = sum(1 / u**2 for u in U)
R_W = sum(r / u**2 for r, u in zip(R, U)) / W
CHI2 = sum(((r - R_W) / u)**2 for r, u in zip(R, U))
CHI2_CRIT = 30.14352720564616     # χ²(0.95, 19)
S_R = st.stdev(R)
assert abs(R_W - SHEET["R"]) < 1e-6 and abs(CHI2 - 40.972872) < 1e-5 and abs(S_R - 0.087552) < 1e-6
assert abs(math.hypot(math.sqrt(sum(u * u for u in U)) / N, S_R) - SHEET["u"]) < 1e-6

def evaluate(weighted):
    if weighted:
        mean, u_mean = R_W, W ** -0.5
    else:
        mean, u_mean = st.mean(R), math.sqrt(sum(u * u for u in U)) / N
    u_used = math.hypot(u_mean, S_R)
    t = abs(1 - mean) / u_used
    levels = {}
    for x in (0.1, 10):
        y = x / mean
        up = S_I_ABS / mean if x < SPLIT else y * S_I_REL
        ut = y * u_used / mean
        uc = math.hypot(up, ut)
        levels[x] = (y, uc, K * uc)
    return mean, u_mean, u_used, t, levels

def project(weighted):
    mean, u_mean, u_used, t, lv = evaluate(weighted)
    assert t > K
    y1, u1, U1 = lv[0.1]; y2, u2, U2 = lv[10]
    kind = "weighted" if weighted else "arithmetic"
    name_en = "weighted mean recovery (MUPR Eq. 16.1)" if weighted else "arithmetic mean recovery (MUPR Eq. 8–12)"
    name_tr = "ağırlıklı ortalama geri kazanım (MUPR Eş. 16.1)" if weighted else "aritmetik ortalama geri kazanım (MUPR Eş. 8–12)"
    a_mean, a_u = st.mean(R), math.sqrt(sum(u * u for u in U)) / N
    expl = [
        T("Fluoranthene in river sediments by accelerated solvent extraction and GC-MS/MS, validated over eight days. The spreadsheet of the paper implements the Eurachem/CITAC MUPR guide; Supplementary Material 06 excludes the spikes smaller than 10 % of the native mass fraction.",
          "Nehir sedimentlerinde fluoranten, hızlandırılmış çözücü ekstraksiyonu ve GC-MS/MS ile; sekiz günde valide edilmiştir. Makalenin elektronik tablosu Eurachem/CITAC MUPR rehberini uygular; Ek Materyal 06, doğal kütle kesrinin %10'undan küçük katkıları dışarıda bırakır."),
        T(f"Precision model: interval I (−0.1 to {SPLIT:g} mg/kg) constant absolute s_I = {S_I_ABS:.6f} mg/kg (= {SPLIT:g} mg/kg × s′_I); interval II ({SPLIT:g} to {TOP:g} mg/kg) constant relative s′_I = {100 * S_I_REL:.4f} %. A single result is reported, so s_I is the precision component.",
          f"Kesinlik modeli: I. aralık (−0,1 ile {c(f'{SPLIT:g}')} mg/kg arası) sabit mutlak s_I = {c(f'{S_I_ABS:.6f}')} mg/kg (= {c(f'{SPLIT:g}')} mg/kg × s′_I); II. aralık ({c(f'{SPLIT:g}')} ile {c(f'{TOP:g}')} mg/kg arası) sabit bağıl s′_I = %{c(f'{100 * S_I_REL:.4f}')}. Tek sonuç raporlandığından kesinlik bileşeni s_I'dır."),
        T(f"Recovery: {N} mean recoveries, from the CRM, three spikes of a sediment without native analyte and 16 spikes of sediments with native analyte, each with u(R_k) from the spreadsheet (precision model and spike uncertainty). They range from {100 * min(R):.1f} % to {100 * max(R):.1f} %.",
          f"Geri kazanım: SRM'den, doğal analit içermeyen bir sedimentin üç katkısından ve doğal analit içeren sedimentlerin 16 katkısından {N} ortalama geri kazanım; her birinin u(R_k) değeri elektronik tablodandır (kesinlik modeli ve katkı belirsizliği). Değerler %{c(f'{100 * min(R):.1f}')} ile %{c(f'{100 * max(R):.1f}')} arasındadır."),
        T(f"χ² test (Eq. 16.2): χ² = {CHI2:.2f} > χ²(0.95; {N - 1}) = {CHI2_CRIT:.2f}; the recoveries are not compatible, so s(R) = {100 * S_R:.3f} % is added to the uncertainty of the mean recovery (MUPR 8.2.7).",
          f"χ² testi (Eş. 16.2): χ² = {c(f'{CHI2:.2f}')} > χ²(0,95; {N - 1}) = {c(f'{CHI2_CRIT:.2f}')}; geri kazanımlar uyumlu değildir, bu nedenle s(R) = %{c(f'{100 * S_R:.3f}')} ortalama geri kazanımın belirsizliğine eklenir (MUPR 8.2.7)."),
    ]
    if weighted:
        expl.append(T(f"Weighted mean recovery R̿ = Σ(R_k/u²)/Σ(1/u²) = {100 * mean:.3f} % with u(R̿) = (Σ1/u²)^−½ = {100 * u_mean:.3f} %; with s(R): u = {100 * u_used:.3f} %.",
                      f"Ağırlıklı ortalama geri kazanım R̿ = Σ(R_k/u²)/Σ(1/u²) = %{c(f'{100 * mean:.3f}')}, u(R̿) = (Σ1/u²)^−½ = %{c(f'{100 * u_mean:.3f}')}; s(R) ile: u = %{c(f'{100 * u_used:.3f}')}."))
    else:
        expl.append(T(f"Arithmetic mean recovery R̄ = ΣR_k/N = {100 * mean:.3f} % with u(R̄) = √Σu²(R_k)/N = {100 * u_mean:.3f} % (Eq. 12); with s(R): u = {100 * u_used:.3f} %. The test SS95(SP1) (u = 75 %) contributes most of u(R̄).",
                      f"Aritmetik ortalama geri kazanım R̄ = ΣR_k/N = %{c(f'{100 * mean:.3f}')}, u(R̄) = √Σu²(R_k)/N = %{c(f'{100 * u_mean:.3f}')} (Eş. 12); s(R) ile: u = %{c(f'{100 * u_used:.3f}')}. u(R̄)'nin büyük kısmı SS95(SP1) testinden (u = %75) gelir."))
    expl += [
        T(f"Significance (Eq. 13): |1 − R|/u = {t:.3f} > t(0.975; {NU}) = {K:.3f}; results are corrected, c = x/R. Coverage factor k = {K:.3f} from the minimum degrees of freedom of the components (ν = {NU}, the recoveries), as in the spreadsheet.",
          f"Anlamlılık (Eş. 13): |1 − R|/u = {c(f'{t:.3f}')} > t(0,975; {NU}) = {c(f'{K:.3f}')}; sonuçlar düzeltilir, c = x/R. Kapsam faktörü k = {c(f'{K:.3f}')}, elektronik tablodaki gibi bileşenlerin en küçük serbestlik derecesinden (ν = {NU}, geri kazanımlar)."),
        T(f"Results: x = 0.1 mg/kg → ({y1:.4f} ± {U1:.4f}) mg/kg; x = 10 mg/kg → ({y2:.3f} ± {U2:.3f}) mg/kg ({100 * U2 / y2:.1f} %).",
          f"Sonuçlar: x = 0,1 mg/kg → ({c(f'{y1:.4f}')} ± {c(f'{U1:.4f}')}) mg/kg; x = 10 mg/kg → ({c(f'{y2:.3f}')} ± {c(f'{U2:.3f}')}) mg/kg (%{c(f'{100 * U2 / y2:.1f}')})."),
    ]
    notes = [
        T(f"The spreadsheet reports ({SHEET[10][0]:.2f} ± {SHEET[10][1]:.2f}) mg/kg at 10 mg/kg and ({SHEET[0.1][0]:.3f} ± {SHEET[0.1][1]:.3f}) mg/kg at 0.1 mg/kg. It uses the weighted mean R̿ = {100 * R_W:.3f} % with the uncertainty of the arithmetic mean, √Σu²/N = {100 * a_u:.3f} % (research log F73), and does not divide the absolute precision of interval I by R̿ (F72). The text of the paper prints k = 2.01; the spreadsheet uses 2.093 (F71).",
          f"Elektronik tablo 10 mg/kg'da ({c(f'{SHEET[10][0]:.2f}')} ± {c(f'{SHEET[10][1]:.2f}')}) mg/kg ve 0,1 mg/kg'da ({c(f'{SHEET[0.1][0]:.3f}')} ± {c(f'{SHEET[0.1][1]:.3f}')}) mg/kg verir. Ağırlıklı ortalama R̿ = %{c(f'{100 * R_W:.3f}')} ile aritmetik ortalamanın belirsizliğini, √Σu²/N = %{c(f'{100 * a_u:.3f}')}, birlikte kullanır (araştırma günlüğü F73) ve I. aralığın mutlak kesinliğini R̿'ya bölmez (F72). Makale metni k = 2,01 yazar; tablo 2,093 kullanır (F71)."),
        T("The program divides absolute components of a recovery-corrected result by the recovery (y = x/R). The other variant of this example uses the "
          + ("arithmetic mean of MUPR Eq. (8)–(12)." if weighted else "weighted mean of MUPR Eq. (16.1) with its own uncertainty."),
          "Program, geri kazanımla düzeltilmiş sonucun mutlak bileşenlerini geri kazanıma böler (y = x/R). Bu örneğin diğer türü "
          + ("MUPR Eş. (8)–(12)'nin aritmetik ortalamasını kullanır." if weighted else "MUPR Eş. (16.1)'in ağırlıklı ortalamasını kendi belirsizliğiyle kullanır.")),
    ]
    recovery = {"name": "u(R̿)" if weighted else "u(R̄)",
                "description": T("recovery of fluoranthene, CRM and spiked sediments", "fluoranten geri kazanımı, SRM ve katkılı sedimentler"),
                "tests": [{"type": "given", "label": l, "recovery_percent": round(100 * r, 8), "u_percent": round(100 * u, 8)} for l, r, u in TESTS],
                "include_sd_of_recoveries": True}
    if weighted:
        recovery = {**{k: recovery[k] for k in ("name", "description", "tests")}, "mean": "weighted", "include_sd_of_recoveries": True}
    pid = f"bettencourt2026-fluoranthene-{kind}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid,
                      "title": T(f"Fluoranthene in river sediments, {name_en} (Bettencourt da Silva 2026)",
                                 f"Nehir sedimentlerinde fluoranten, {name_tr} (Bettencourt da Silva 2026)"),
                      "description": T("Top-down uncertainty of fluoranthene in river sediments from the in-house validation data of the MUPR spreadsheet: a two-interval precision model and 20 mean recoveries that are not compatible, so the dispersion of the recoveries is added and results are corrected for recovery.",
                                       "MUPR elektronik tablosunun laboratuvar içi validasyon verilerinden nehir sedimentlerinde fluorantenin top-down belirsizliği: iki aralıklı kesinlik modeli ve uyumlu olmayan 20 ortalama geri kazanım; bu nedenle geri kazanımların dağılımı eklenir ve sonuçlar geri kazanım için düzeltilir."),
                      "explanation": expl, "field": "chemistry",
                      "sources": [{"document": DOC, "section": SECTION, "url": URL}],
                      "tags": ["top-down", "recovery", "two-intervals", "environmental", "spreadsheet-comparison"],
                      "notes": notes},
         "topdown": {"quantity": "w(fluoranthene)", "unit": "mg/kg", "levels": [0.1, 10],
                     "precision": [
                         {"name": "s_I⟨I⟩", "data": {"type": "given", "sd": round(S_I_ABS, 10)}, "range": {"max": SPLIT}},
                         {"name": "s′_I⟨II⟩", "data": {"type": "given", "sd_percent": round(100 * S_I_REL, 10)}, "range": {"min": SPLIT}},
                     ],
                     "recovery": recovery,
                     "coverage_probability": 0.95, "coverage_dof": NU},
         "reporting": {"significant_digits": 2},
         "expected": {"topdown": {
             "recovery": {"mean_percent": chk(round(100 * mean, 5), 1e-4),
                          "standard_uncertainty_percent": chk(round(100 * u_used, 5), 1e-4),
                          "sd_percent": chk(round(100 * S_R, 5), 1e-4),
                          "statistic": chk(round(t, 5), 1e-4), "critical": chk(round(K, 5), 1e-4),
                          "chi_squared": chk(round(CHI2, 4), 1e-3), "significant": True},
             "levels": [{"level": 0.1, "value": chk(round(y1, 6), 1e-5), "standard_uncertainty": chk(round(u1, 6), 1e-5),
                         "expanded_uncertainty": chk(round(U1, 6), 1e-5)},
                        {"level": 10, "value": chk(round(y2, 5), 1e-4), "standard_uncertainty": chk(round(u2, 5), 1e-4),
                         "expanded_uncertainty": chk(round(U2, 5), 1e-4)}]},
             "reference": T(f"Recomputed from Supplementary Material 06 (independent script). The χ² = {CHI2:.4f}, s(R) = {100 * S_R:.4f} % and R̿ = {100 * R_W:.4f} % agree with the sheet Trueness (CM16, CO16, CK16); the uncertainties differ from the spreadsheet as explained in the notes.",
                            f"Ek Materyal 06'dan yeniden hesaplanmıştır (bağımsız betik). χ² = {c(f'{CHI2:.4f}')}, s(R) = %{c(f'{100 * S_R:.4f}')} ve R̿ = %{c(f'{100 * R_W:.4f}')} Trueness sayfasıyla (CM16, CO16, CK16) uyumludur; belirsizlikler notlarda açıklandığı gibi elektronik tablodan farklıdır.")}}
    out = Path("library/chemistry") / f"{pid}.json"
    out.write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(out, f"R={100 * mean:.4f}% u={100 * u_used:.4f}% t={t:.4f} 0.1->{y1:.5f}±{U1:.5f} 10->{y2:.4f}±{U2:.4f}")

for wtd in (True, False):
    project(wtd)
