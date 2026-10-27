"""Writes the EURAMET cg-18 v4.0 H1.4 and H2.4 library projects: uncertainty of a weighing result in use and
minimum weight (run from the repository root). Expected values are the printed ones where they follow from the
guide's formulas; this script recomputes every value independently and asserts the agreement."""
import json
import math
import statistics as st
from pathlib import Path

import numpy as np

DOC = "EURAMET Calibration Guide No. 18 v4.0 (2015), Guidelines on the Calibration of Non-Automatic Weighing Instruments"
URL = "https://www.euramet.org/publications-media-centre/calibration-guidelines"
R12, R3 = math.sqrt(12), math.sqrt(3)


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


def fit(points):
    """Weighted straight line through zero, p_j = 1/u²(E_j) (C2.2-18b): uncorrelated reference masses."""
    i = np.array([p[0] for p in points if p[0] > 0])
    e = np.array([p[1] for p in points if p[0] > 0])
    u2 = np.array([p[2] ** 2 + p[3] ** 2 for p in points if p[0] > 0])
    w = 1 / u2
    var = 1 / np.sum(w * i * i)
    return float(var * np.sum(w * i * e)), math.sqrt(var)


def evaluate(ex):
    a1, ua1 = fit(ex["points"])
    terms = {"u(a1)": ua1, "temp": ex["kt"] * ex["dt"] / R12, "ecc": ex["ecc"][0] / (ex["ecc"][1] * R3)}
    if ex.get("buoy_dt") is not None:
        terms["buoy"] = math.sqrt(1.07e-4 + 1.33e-6 * ex["buoy_dt"] ** 2) * 1.2 / 8000
    pts = sorted((p[0], p[1]) for p in ex["points"])
    q = [(b[1] - a[1]) / (b[0] - a[0]) for a, b in zip(pts, pts[1:])]
    terms["tare"] = (max(q) - min(q)) / R12
    beta2 = sum(v * v for v in terms.values())
    d0 = ex["intervals"][0][1]
    out = {"a1": a1, "ua1": ua1, "terms": terms, "beta2": beta2, "alpha2": [], "lower": [], "U_lower": [], "slope": [], "gl_lower": [], "gl_slope": []}
    lo = 0.0
    for mx, d, reps in ex["intervals"]:
        s = st.stdev(reps)
        a2 = d0 * d0 / 12 + d * d / 12 + s * s
        U = lambda r: 2 * math.sqrt(a2 + beta2 * r * r)
        b = (U(mx) - U(lo)) / (mx - lo)
        out["alpha2"].append(a2)
        out["lower"].append(lo)
        out["U_lower"].append(U(lo))
        out["slope"].append(b)
        out["gl_lower"].append(U(lo) + abs(a1) * lo)
        out["gl_slope"].append(b + abs(a1))
        lo = mx
    req, sf = ex["req"], ex["sf"]
    out["mw"] = out["gl_lower"][0] * sf / (req - out["gl_slope"][0] * sf)
    assert out["mw"] <= ex["intervals"][0][0]
    return out


EX = {
    "h1-a": {
        "title": T("Balance 220 g / 0.1 mg, not adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H1.4/A)",
                   "Terazi 220 g / 0,1 mg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H1.4/A)"),
        "section": "Appendix H1.1/A to H1.4/A; 7.4, 7.5; Appendix G",
        "instrument": "Electronic balance, Max 220 g, d = 0.1 mg",
        # indication, error, u(m_ref) (7.1.2-5e budget), u(I) in g
        "points": [(0.0, 0.0, 0.0, 0.000118), (50.0004, 0.0004, 0.000107, 0.000124), (100.0006, 0.0007, 0.000205, 0.000134),
                   (150.0009, 0.0010, 0.000312, 0.000149), (220.0014, 0.0013, 0.000459, 0.000175)],
        "intervals": [(220.0, 0.0001, [100.0006, 100.0003, 100.0005, 100.0004, 100.0005])],
        "kt": 1.5e-6, "dt": 3.0, "buoy_dt": 3.0, "ecc": (0.0002, 100.0), "req": 0.01, "sf": 3.0,
        "printed": {"a1": 6.709e-6, "ua1": 1.242e-6, "temp": 1.299e-6, "buoy": 1.636e-6, "tare": 1.072e-6, "ecc": 1.155e-6,
                    "alpha2": [1.467e-8], "beta2": 8.390e-12, "U_lower": [2.422e-4], "slope": [4.796e-6], "gl_slope": [1.150e-5], "mw": 0.0729},
        "budget": "7.1.2-5e (room temperature variation 5 K)",
    },
    "h1-b": {
        "title": T("Balance 220 g / 0.1 mg, adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H1.4/B)",
                   "Terazi 220 g / 0,1 mg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H1.4/B)"),
        "section": "Appendix H1.1/B to H1.4/B; 7.4, 7.5; Appendix G",
        "instrument": "Electronic balance, Max 220 g, d = 0.1 mg",
        "points": [(0.0, 0.0, 0.0, 0.000118), (50.0000, 0.0, 0.00003, 0.000124), (99.9998, -0.0001, 0.000049, 0.000134),
                   (149.9999, 0.0, 0.000079, 0.000149), (220.0000, -0.0001, 0.000123, 0.000175)],
        "intervals": [(220.0, 0.0001, [100.0006, 100.0003, 100.0005, 100.0004, 100.0005])],
        "kt": 1.5e-6, "dt": 3.0, "buoy_dt": 3.0, "ecc": (0.0002, 100.0), "req": 0.01, "sf": 3.0,
        "printed": {"a1": -3.895e-7, "ua1": 6.337e-7, "temp": 1.299e-6, "buoy": 1.636e-6, "tare": 5.774e-7, "ecc": 1.154e-6,
                    "alpha2": [1.466e-8], "beta2": 6.433e-12, "U_lower": [2.422e-4], "slope": [4.090e-6], "gl_slope": [4.479e-6], "mw": 0.0727},
        "budget": "7.1.2-5c (adjusted immediately before calibration)",
    },
    "h2-a": {
        "title": T("Multi-interval scale 12/30/60 kg, not adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H2.4/A)",
                   "Çok aralıklı tartı aleti 12/30/60 kg, kalibrasyondan önce ayarlanmamış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H2.4/A)"),
        "section": "Appendix H2.1/A to H2.4/A; 7.4, 7.5; Appendix G",
        "instrument": "Multi-interval scale, Max 12/30/60 kg, d = 2/5/10 g",
        "points": [(0.0, 0.0, 0.0, 1.238), (10000.0, 0.0, 0.151, 1.545), (19995.0, -5.0, 0.290, 3.464), (39990.0, -10.0, 0.581, 4.950),
                   (59990.0, -10.0, 0.904, 5.909)],
        "intervals": [(12000.0, 2.0, [9998, 10000, 9998, 10000, 10000]), (30000.0, 5.0, [24995, 25000, 24995, 24995, 25000]),
                      (60000.0, 10.0, [24995, 25000, 24995, 24995, 25000])],
        "kt": 2e-6, "dt": 3.0, "buoy_dt": None, "ecc": (5.0, 20000.0), "req": 0.01, "sf": 2.0,
        "printed": {"a1": -1.717e-4, "ua1": 6.459e-5, "temp": 1.732e-6, "tare": 1.444e-4, "ecc": 1.443e-4,
                    "alpha2": [1.867, 9.917, 16.167], "beta2": 4.589e-8, "U_lower": [2.733, 10.190, 20.311], "slope": [2.574e-4, 3.434e-4, 3.923e-4],
                    "gl_lower": [2.733, 10.190, 20.311], "gl_slope": [4.291e-4, 5.151e-4, 5.641e-4], "mw": 598.0},
        "budget": "7.1.2-5d (no information on the air density)",
    },
    "h2-b": {
        "title": T("Multi-interval scale 12/30/60 kg, adjusted before calibration: uncertainty of a weighing result and minimum weight (EURAMET cg-18 H2.4/B)",
                   "Çok aralıklı tartı aleti 12/30/60 kg, kalibrasyondan önce ayarlanmış: tartım sonucunun belirsizliği ve minimum tartım (EURAMET cg-18 H2.4/B)"),
        "section": "Appendix H2.1/B to H2.4/B; 7.4, 7.5; Appendix G",
        "instrument": "Multi-interval scale, Max 12/30/60 kg, d = 2/5/10 g",
        "points": [(0.0, 0.0, 0.0, 1.065), (10000.0, 0.0, 0.106, 1.410), (20000.0, 0.0, 0.198, 3.082), (40000.0, 0.0, 0.397, 4.690),
                   (60000.0, 0.0, 0.635, 5.694)],
        "intervals": [(12000.0, 2.0, [10000, 10000, 9998, 10000, 10000]), (30000.0, 5.0, [25000, 25000, 25000, 24995, 25000]),
                      (60000.0, 10.0, [25000, 25000, 25000, 24995, 25000])],
        "kt": 2e-6, "dt": 3.0, "buoy_dt": None, "ecc": (5.0, 20000.0), "req": 0.01, "sf": 2.0,
        "printed": {"a1": 0.0, "ua1": 6.043e-5, "temp": 1.732e-6, "tare": 0.0, "ecc": 1.443e-4,
                    "alpha2": [1.467, 7.417, 13.667], "beta2": 2.449e-8, "U_lower": [2.422, 6.616, 11.951], "slope": [1.706e-4, 2.355e-4, 2.744e-4],
                    "gl_lower": [2.422, 6.616, 11.951], "gl_slope": [1.706e-4, 2.355e-4, 2.744e-4], "mw": 502.0},
        "budget": "7.1.2-5c (adjusted immediately before calibration)",
    },
}


def tol(v):
    """Half a unit of the fourth significant digit, plus rounding of the inputs."""
    if v == 0:
        return 1e-12
    return 10 ** (math.floor(math.log10(abs(v))) - 3) * 1.5


def g(x, sig=4):
    return f"{x:.{sig}g}"


for key, ex in EX.items():
    r = evaluate(ex)
    pr = ex["printed"]
    deviations = []  # (what, printed, recomputed)

    def compare(name, got, printed, t=None):
        t = t if t is not None else tol(printed)
        if abs(got - printed) > t:
            deviations.append((name, printed, got))
            return False
        return True

    compare("a1", abs(r["a1"]), abs(pr["a1"]), 1.5e-3 * max(abs(pr["a1"]), 1e-9))
    compare("u(a1)", r["ua1"], pr["ua1"], max(tol(pr["ua1"]), 1e-3 * pr["ua1"]))
    for t in ("temp", "buoy", "tare", "ecc"):
        if t in pr:
            compare(t, r["terms"].get(t, 0.0), pr[t], max(tol(pr[t]), 1.2e-3 * pr[t]) if pr[t] else 1e-12)
    for i, v in enumerate(pr["alpha2"]):
        compare(f"alpha2[{i}]", r["alpha2"][i], v, 2e-3 * v)
    compare("beta2", r["beta2"], pr["beta2"], 2e-3 * pr["beta2"])
    for name in ("U_lower", "slope", "gl_lower", "gl_slope"):
        for i, v in enumerate(pr.get(name, [])):
            compare(f"{name}[{i}]", r[name][i], v, 2e-3 * v)
    compare("minimum weight", r["mw"], pr["mw"], 1e-3 * pr["mw"] + (5e-5 if pr["mw"] < 1 else 0.5))
    known = {"h1-a": set(), "h1-b": {"tare", "beta2", "slope[0]", "gl_slope[0]"}, "h2-a": {"U_lower[1]", "U_lower[2]"}, "h2-b": set()}[key]
    got_dev = {d[0] for d in deviations}
    assert got_dev == known, (key, deviations)

    exp = lambda name, i=None: r[name] if i is None else r[name][i]
    n = len(ex["intervals"])
    expected = {
        "coefficients": [chk(r["a1"], 1.5e-3 * max(abs(pr["a1"]), 1e-9) if pr["a1"] else 1e-12)],
        "u_coefficients": [chk(pr["ua1"], max(tol(pr["ua1"]), 1e-3 * pr["ua1"]))],
        "in_use": {
            "alpha_squared": [chk(pr["alpha2"][i], 2e-3 * pr["alpha2"][i]) for i in range(n)],
            "beta_squared": chk(pr["beta2"], 2e-3 * pr["beta2"]) if "beta2" not in known else chk(r["beta2"], 1e-4 * r["beta2"]),
            "expanded_at_lower": [chk(pr["U_lower"][i], 2e-3 * pr["U_lower"][i]) if f"U_lower[{i}]" not in known else chk(r["U_lower"][i], 1e-4 * r["U_lower"][i]) for i in range(n)],
            "slope": [chk(pr["slope"][i], 2e-3 * pr["slope"][i]) if f"slope[{i}]" not in known else chk(r["slope"][i], 1e-4 * r["slope"][i]) for i in range(n)],
            "global_slope": [chk(pr["gl_slope"][i], 2e-3 * pr["gl_slope"][i]) if f"gl_slope[{i}]" not in known else chk(r["gl_slope"][i], 1e-4 * r["gl_slope"][i]) for i in range(n)],
            "minimum_weight": chk(pr["mw"], 1e-3 * pr["mw"] + (5e-5 if pr["mw"] < 1 else 0.5)) if key != "h1-b" else chk(round(r["mw"], 6), 5e-6),
        },
    }
    if "gl_lower" in pr:
        expected["in_use"]["global_at_lower"] = [chk(pr["gl_lower"][i], 2e-3 * pr["gl_lower"][i]) for i in range(n)]

    multi = n > 1
    unit = "g"
    intervals = [{"max": mx, "d": d, "repeatability": reps} for mx, d, reps in ex["intervals"]]
    s_vals = [st.stdev(reps) for _, _, reps in ex["intervals"]]
    in_use = {"intervals": intervals, "eccentricity": {"deviation": ex["ecc"][0], "load": ex["ecc"][1]},
              "temperature": {"coefficient": ex["kt"], "delta_t": ex["dt"]}, "tare": True,
              "requirement": ex["req"], "safety_factor": ex["sf"],
              "readings": [mx for mx, _, _ in ex["intervals"]]}
    if ex["buoy_dt"] is not None:
        in_use["buoyancy"] = {"delta_t": ex["buoy_dt"]}

    terms_en = (f"Relative terms (in use): u(a₁) = {g(r['ua1'])} from the fit (C2.2-16d), temperature K_T·ΔT/√12 = {g(r['terms']['temp'])} (K_T = {g(ex['kt'], 2)}/K, ΔT = {ex['dt']:g} K: the built-in adjustment acts at 3 K, 7.4.3-1)"
                + (f", buoyancy √(1.07·10⁻⁴ + 1.33·10⁻⁶·ΔT²)·ρ₀/ρ_c = {g(r['terms']['buoy'])} (7.4.3-4)" if "buoy" in r["terms"] else ", buoyancy not relevant")
                + f", tare (q_E,max − q_E,min)/√12 = {g(r['terms']['tare'])} from the test points (7.4.4-5), eccentricity ΔI_ecc,max/(L_ecc√3) = {g(r['terms']['ecc'])} (7.4.4-10); β_W² = {g(r['beta2'])}.")
    terms_tr = (f"Bağıl terimler (kullanımda): uyumdan u(a₁) = {c(g(r['ua1']))} (C2.2-16d), sıcaklık K_T·ΔT/√12 = {c(g(r['terms']['temp']))} (K_T = {c(g(ex['kt'], 2))}/K, ΔT = {c(f'{ex[chr(100)+chr(116)]:g}')} K: yerleşik ayar 3 K'de devreye girer, 7.4.3-1)"
                + (f", kaldırma kuvveti √(1,07·10⁻⁴ + 1,33·10⁻⁶·ΔT²)·ρ₀/ρ_c = {c(g(r['terms']['buoy']))} (7.4.3-4)" if "buoy" in r["terms"] else ", kaldırma kuvveti ilgili değil")
                + f", dara deney noktalarından (q_E,maks − q_E,min)/√12 = {c(g(r['terms']['tare']))} (7.4.4-5), eksantriklik ΔI_ecc,maks/(L_ecc√3) = {c(g(r['terms']['ecc']))} (7.4.4-10); β_W² = {c(g(r['beta2']))}.")
    alpha_en = ", ".join(f"{g(a)} g²" for a in r["alpha2"])
    alpha_tr = ", ".join(f"{c(g(a))} g²" for a in r["alpha2"])
    expl = [
        T(f"{ex['instrument']}. Error function E_appr = a₁R through zero (C2.2-16), fitted with the weights p_j = 1/u²(E_j) of the budget {ex['budget']} (C2.2-18b, uncorrelated reference masses): a₁ = {g(r['a1'])}, u(a₁) = {g(r['ua1'])}.",
          f"{ex['instrument']}. Sıfırdan geçen hata fonksiyonu E_yakl = a₁R (C2.2-16), {ex['budget']} bütçesinin p_j = 1/u²(E_j) ağırlıklarıyla uydurulmuştur (C2.2-18b, korelasyonsuz referans kütleler): a₁ = {c(g(r['a1']))}, u(a₁) = {c(g(r['ua1']))}."),
        T(f"Absolute terms per interval: α_W² = d₀²/12 + d²/12 + s² = {alpha_en} (7.4.1-6), with s from the repeatability test{'s' if multi else ''} ({', '.join(g(s, 4) for s in s_vals)} g) and d₀ = d₁.",
          f"Aralık başına mutlak terimler: α_W² = d₀²/12 + d²/12 + s² = {alpha_tr} (7.4.1-6); s tekrarlanabilirlik deney{'lerin' if multi else 'in'}den ({', '.join(c(g(s, 4)) for s in s_vals)} g), d₀ = d₁."),
        T(terms_en, terms_tr),
        T("U(W) = 2√(α_W² + β_W²R²) (7.4.5-2, 7.5.1-2b) is replaced by straight lines through U(W) at the interval limits (7.5.2-3d" + (", -3f per interval" if multi else "") + f"), and the global uncertainty without correction of the readings adds |a₁|R: U_gl(W) = {g(r['gl_lower'][0])} g + {g(r['gl_slope'][0])}·R in the first interval (7.5.2-3e).",
          "U(W) = 2√(α_W² + β_W²R²) (7.4.5-2, 7.5.1-2b), aralık sınırlarındaki U(W) değerlerinden geçen doğrularla değiştirilir (7.5.2-3d" + (", aralık başına -3f" if multi else "") + f"); okumalar düzeltilmeden genel belirsizlik |a₁|R'yi ekler: ilk aralıkta U_gl(W) = {c(g(r['gl_lower'][0]))} g + {c(g(r['gl_slope'][0]))}·R (7.5.2-3e)."),
        T(f"Minimum weight for Req = {ex['req'] * 100:g} % and the safety factor SF = {ex['sf']:g}: R_min,SF = α_gl·SF/(Req − β_gl·SF) = {g(r['mw'], 4)} g (G-9).",
          f"Req = %{c(f'{ex[chr(114)+chr(101)+chr(113)] * 100:g}')} ve güvenlik faktörü SF = {c(f'{ex[chr(115)+chr(102)]:g}')} için minimum tartım: R_min,SF = α_gl·SF/(Req − β_gl·SF) = {c(g(r['mw'], 4))} g (G-9)."),
    ]
    notes = [T("The uncertainty of a weighing result is advice for the user under assumed conditions of use; it is not part of the calibration results (7.4).",
               "Tartım sonucunun belirsizliği, varsayılan kullanım koşullarında kullanıcıya yönelik öneridir; kalibrasyon sonuçlarının parçası değildir (7.4).")]
    if key == "h1-b":
        notes.append(T(f"The guide prints u_rel(δR_Tare) = 5.774·10⁻⁷. The slopes between the test points are 0, −2·10⁻⁶, +2·10⁻⁶ and −1.43·10⁻⁶, so (q_E,max − q_E,min)/√12 = 4·10⁻⁶/√12 = {g(r['terms']['tare'])} (7.4.4-5); the printed value corresponds to a width of 2·10⁻⁶. The project uses the formula: β_W² = {g(r['beta2'])} instead of 6.433·10⁻¹², U_gl(W) slope {g(r['gl_slope'][0])} instead of 4.479·10⁻⁶, minimum weight {g(r['mw'], 4)} g instead of 0.0727 g (research log F76).",
                       f"Kılavuz u_bağıl(δR_Dara) = 5,774·10⁻⁷ basar. Deney noktaları arasındaki eğimler 0, −2·10⁻⁶, +2·10⁻⁶ ve −1,43·10⁻⁶'dır; dolayısıyla (q_E,maks − q_E,min)/√12 = 4·10⁻⁶/√12 = {c(g(r['terms']['tare']))} (7.4.4-5); basılı değer 2·10⁻⁶ genişliğe karşılık gelir. Proje formülü kullanır: β_W² = 6,433·10⁻¹² yerine {c(g(r['beta2']))}, U_gl(W) eğimi 4,479·10⁻⁶ yerine {c(g(r['gl_slope'][0]))}, minimum tartım 0,0727 g yerine {c(g(r['mw'], 4))} g (araştırma günlüğü F76)."))
        notes.append(T("The guide prints a₁ = 3.895·10⁻⁷ without its sign; the errors (−0.1 mg at 100 g and 220 g) give a₁ = −3.895·10⁻⁷. Only |a₁| enters the global uncertainty.",
                       "Kılavuz a₁ = 3,895·10⁻⁷ değerini işaretsiz basar; hatalar (100 g ve 220 g'da −0,1 mg) a₁ = −3,895·10⁻⁷ verir. Genel belirsizliğe yalnızca |a₁| girer."))
    if key == "h2-a":
        notes.append(T(f"The guide prints the first-order U(W) of the second and third intervals as 10.190 g + 3.434·10⁻⁴(R − 12 000 g) and 20.311 g + 3.923·10⁻⁴(R − 30 000 g). 10.190 g and 20.311 g are the global uncertainties at 12 000 g and 30 000 g, U(W) + |a₁|R; U(W) itself is {g(r['U_lower'][1])} g and {g(r['U_lower'][2])} g there. The slopes and the global lines are right (research log F75).",
                       f"Kılavuz ikinci ve üçüncü aralığın birinci mertebe U(W)'sini 10,190 g + 3,434·10⁻⁴(R − 12 000 g) ve 20,311 g + 3,923·10⁻⁴(R − 30 000 g) olarak basar. 10,190 g ve 20,311 g, 12 000 g ve 30 000 g'daki genel belirsizliklerdir, U(W) + |a₁|R; U(W)'nin kendisi orada {c(g(r['U_lower'][1]))} g ve {c(g(r['U_lower'][2]))} g'dır. Eğimler ve genel belirsizlik doğruları doğrudur (araştırma günlüğü F75)."))
        notes.append(T("The guide prints a₁ = 1.717·10⁻⁴ without its sign; the errors are negative (−5 g at 20 kg, −10 g at 40 and 60 kg).",
                       "Kılavuz a₁ = 1,717·10⁻⁴ değerini işaretsiz basar; hatalar negatiftir (20 kg'da −5 g, 40 ve 60 kg'da −10 g)."))
    if key in ("h1-a", "h2-a"):
        notes.append(T("H1/A fits the errors with the budget of 7.1.2-5e (room temperature variation), H2/A with the most conservative budget of 7.1.2-5d; the printed a₁ and u(a₁) reproduce only with these choices.",
                       "H1/A hataları 7.1.2-5e bütçesiyle (oda sıcaklığı değişimi), H2/A ise en tutucu 7.1.2-5d bütçesiyle uydurur; basılı a₁ ve u(a₁) yalnızca bu seçimlerle yeniden elde edilir."))
    pid = f"cg18-{key}-weighing-result"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": ex["title"],
                      "description": T("Uncertainty of a weighing result in use, global uncertainty without correction of the readings and minimum weight from the calibration results and the assumed conditions of use.",
                                       "Kalibrasyon sonuçlarından ve varsayılan kullanım koşullarından kullanımda tartım sonucunun belirsizliği, okumalar düzeltilmeden genel belirsizlik ve minimum tartım."),
                      "explanation": expl, "field": "mass",
                      "sources": [{"document": DOC, "section": ex["section"], "url": URL}],
                      "tags": ["euramet-cg18", "balance", "weighing-instrument", "uncertainty-in-use", "minimum-weight", "error-function", "validation"],
                      "notes": notes},
         "error_function": {"instrument": ex["instrument"], "unit": unit, "error_unit": unit,
                            "points": [{"indication": i, "error": e, "u_reference": um, "u_indication": ui} for i, e, um, ui in ex["points"]],
                            "through_zero": True, "reference_correlation": "none", "in_use": in_use},
         "expected": {"error_function": expected,
                      "reference": T(f"EURAMET cg-18 v4.0 {ex['section'].split(';')[0].replace('Appendix ', '')}: printed values of the uncertainty of a weighing result and the minimum weight"
                                     + ("; values affected by F75/F76 are the recomputed ones (see notes)." if known else "."),
                                     f"EURAMET cg-18 v4.0 {ex['section'].split(';')[0].replace('Appendix ', 'Ek ')}: tartım sonucunun belirsizliğinin ve minimum tartımın basılı değerleri"
                                     + ("; F75/F76'dan etkilenen değerler yeniden hesaplanmış olanlardır (notlara bakınız)." if known else "."))}}
    Path(f"library/mass/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")
    print(pid, "a1", g(r["a1"]), "u", g(r["ua1"]), "beta2", g(r["beta2"]), "U0", [g(x) for x in r["U_lower"]], "mw", g(r["mw"]), "deviations", deviations)
