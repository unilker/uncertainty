"""Writes the library projects of Koepke, Lafarge, Possolo & Toman, "Consensus building for interlaboratory
studies, key comparisons, and meta-analysis", Metrologia 54 (2017) S34–S62, with the procedures of the NIST
Consensus Builder (user manual, 2020, Appendix A). Run from the repository root.

The DerSimonian–Laird estimate is recomputed here and the printed consensus values are asserted. The
bootstrap and Linear Pool reference values were computed independently with numpy (100 000 bootstrap
replicates, 10⁶ Linear Pool draws; tools/research/consensus/koepke2017.py).
"""
import json
import math
from pathlib import Path

DOC = "Koepke, Lafarge, Possolo & Toman, Consensus building for interlaboratory studies, key comparisons, and meta-analysis, Metrologia 54 (2017) S34–S62"
URL = "https://doi.org/10.1088/1681-7575/aa6c0e"
NICOB = "NIST Consensus Builder user manual (2020 Nov 12), Appendix A"
NICOB_URL = "https://consensus.nist.gov"


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


def dl(x, u):
    w = [1 / a ** 2 for a in u]
    s1, s2 = sum(w), sum(a * a for a in w)
    xw = sum(a * b for a, b in zip(w, x)) / s1
    q = sum(a * (b - xw) ** 2 for a, b in zip(w, x))
    tau2 = max(0.0, (q - len(x) + 1) / (s1 - s2 / s1))
    ws = [1 / (tau2 + a * a) for a in u]
    mu = sum(a * b for a, b in zip(ws, x)) / sum(ws)
    um = 1 / math.sqrt(sum(ws))
    ukh = um * math.sqrt(sum(a * (b - mu) ** 2 for a, b in zip(ws, x)) / (len(x) - 1))
    return mu, um, math.sqrt(tau2), q, ukh


def write(pid, title, description, explanation, notes, section, tags, measurand, unit, labs, expected, ref, lp=True, ml=False):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "field": "physical", "title": title, "description": description,
                      "explanation": explanation, "tags": tags, "notes": notes,
                      "sources": [{"document": DOC, "section": section, "url": URL},
                                  {"document": NICOB, "section": "§A.2, §A.4, §A.5", "url": NICOB_URL}]},
         "consensus": {"measurand": measurand, "unit": unit,
                       "results": [{"name": n, "value": v, "u": s} for n, v, s in labs],
                       "linear_pool": lp, **({"maximum_likelihood": True} if ml else {}), "trials": 100000, "seed": 1},
         "expected": {"consensus": expected, "reference": ref}}
    Path(f"library/physical/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


# --- Newtonian constant of gravitation (Table 1, Table 2) -------------------------------------------------
G = [("NIST-82", 6.67248, 0.00043), ("TR&D-96", 6.6729, 0.0005), ("LANL-97", 6.67398, 0.00070),
     ("UWash-00", 6.674255, 0.000092), ("BIPM-01", 6.67559, 0.00027), ("UWup-02", 6.67422, 0.00098),
     ("MSL-03", 6.67387, 0.00027), ("HUST-05", 6.67222, 0.00087), ("UZur-06", 6.67425, 0.00012),
     ("HUST-09", 6.67349, 0.00018), ("JILA-10", 6.67234, 0.00014), ("BIPM-14", 6.67554, 0.00016),
     ("LENS-14", 6.67191, 0.00099), ("UCI-14", 6.67435, 0.00013)]
mu, um, tau, q, ukh = dl([a[1] for a in G], [a[2] for a in G])
assert round(um, 5) == 0.00028 and round(tau, 6) == 0.000946 and round(q, 1) == 319.1
# Printed 6.673 80; Table 1 gives 6.673 790 (one unit in the last printed digit, research log F93).
assert abs(mu - 6.67380) < 1.1e-5
xbar = sum(a[1] for a in G) / len(G)
ulp = math.sqrt(sum(a[2] ** 2 for a in G) / len(G) + sum((a[1] - xbar) ** 2 for a in G) / len(G))
assert round(xbar, 5) == 6.67367 and round(ulp, 5) == 0.00124
write("koepke2017-newtonian-constant-g",
      T("Newtonian constant of gravitation: consensus value of 14 measurements (DerSimonian–Laird and Linear Pool; Koepke et al. 2017)",
        "Newton çekim sabiti: 14 ölçümün uzlaşı değeri (DerSimonian–Laird ve doğrusal havuz; Koepke vd. 2017)"),
      T("The 14 measurements of G used in the CODATA 2014 adjustment are mutually inconsistent (Cochran's Q = 319 with 13 degrees of freedom): the measured values are about four times more dispersed than the typical stated uncertainty. The random-effects model recognises this dark uncertainty τ.",
        "CODATA 2014 düzeltmesinde kullanılan 14 G ölçümü birbiriyle tutarsızdır (Cochran Q = 319, serbestlik derecesi 13): ölçülen değerler tipik beyan edilen belirsizliğin yaklaşık dört katı dağılır. Rastgele etkiler modeli bu karanlık belirsizliği (τ) tanır."),
      [T(f"Cochran's Q about the weighted mean with weights 1/u_j² is {q:.1f}; τ̂² = (Q − 13)/(Σu⁻² − Σu⁻⁴/Σu⁻²) gives τ̂ = {tau * 1e4:.5f} × 10⁻¹⁵ m³ kg⁻¹ s⁻², more than three times the median stated uncertainty 0.00027.",
         f"Ağırlıkları 1/u_j² olan ağırlıklı ortalama etrafında Cochran Q = {c(f'{q:.1f}')}; τ̂² = (Q − 13)/(Σu⁻² − Σu⁻⁴/Σu⁻²) bağıntısı τ̂ = {c(f'{tau * 1e4:.5f}')} × 10⁻¹⁵ m³ kg⁻¹ s⁻² verir; bu, beyan edilen belirsizliklerin ortancası 0,00027'nin üç katından fazladır."),
       T(f"With weights 1/(τ̂² + u_j²) the consensus value is {mu:.6f} and u_DL = 1/√Σw = {um:.6f} (× 10⁻¹¹ m³ kg⁻¹ s⁻²). The weights are far more even than those of the conventional weighted mean, where UWash-00 alone would carry most of the weight.",
         f"1/(τ̂² + u_j²) ağırlıklarıyla uzlaşı değeri {c(f'{mu:.6f}')}, u_DL = 1/√Σw = {c(f'{um:.6f}')} (× 10⁻¹¹ m³ kg⁻¹ s⁻²). Ağırlıklar, UWash-00'ün tek başına ağırlığın çoğunu taşıyacağı alışılmış ağırlıklı ortalamaya göre çok daha dengelidir."),
       T("The parametric bootstrap (NICOB §A.2) recognises that τ̂ rests on few degrees of freedom: τ is drawn from the gamma approximation of the distribution of Q, new values are drawn from N(μ̂, τ² + u_j²), and the DerSimonian–Laird estimate is recomputed; the standard deviation and the 2.5 % and 97.5 % percentiles of the replicates give u and the 95 % coverage interval.",
         "Parametrik bootstrap (NICOB §A.2), τ̂'nun az serbestlik derecesine dayandığını hesaba katar: τ, Q'nun dağılımının gama yaklaşımından çekilir, yeni değerler N(μ̂, τ² + u_j²) dağılımından çekilir ve DerSimonian–Laird kestirimi yeniden hesaplanır; tekrarların standart sapması ile %2,5 ve %97,5 yüzdelikleri u'yu ve %95 kapsam aralığını verir."),
       T(f"The Linear Pool mixes the 14 Gaussian distributions with equal weights: its mean is the arithmetic mean {xbar:.5f} and its standard deviation √(mean u² + mean (x − x̄)²) = {ulp:.5f}, more than four times u_DL; the mixture is multimodal (Koepke et al., Figure 2).",
         f"Doğrusal havuz 14 normal dağılımı eşit ağırlıkla karıştırır: ortalaması aritmetik ortalama {c(f'{xbar:.5f}')}, standart sapması √(ort. u² + ort. (x − x̄)²) = {c(f'{ulp:.5f}')} olup u_DL'nin dört katından büyüktür; karışım çok tepelidir (Koepke vd., Şekil 2).")],
      [T("The printed DerSimonian–Laird value 6.673 80 is one unit in the last digit above the value recomputed from Table 1, 6.673 790 (research log F93). The printed interval (6.673 20, 6.674 40) agrees with the bootstrap percentiles rounded to 10⁻⁴.",
         "Basılı DerSimonian–Laird değeri 6,673 80, Tablo 1'den yeniden hesaplanan 6,673 790 değerinin son basamağında bir birim yukarıdadır (araştırma günlüğü F93). Basılı aralık (6,673 20, 6,674 40), 10⁻⁴'e yuvarlanmış bootstrap yüzdelikleriyle uyuşur."),
       T("The correlations between NIST-82 and LANL-97 (0.351) and between HUST-05 and HUST-09 (0.134) are ignored, as in the DerSimonian–Laird row of Table 2.",
         "NIST-82 ile LANL-97 (0,351) ve HUST-05 ile HUST-09 (0,134) arasındaki korelasyonlar, Tablo 2'nin DerSimonian–Laird satırında olduğu gibi göz ardı edilir.")],
      "§7.1, Tables 1 and 2", ["koepke-2017", "nicob", "consensus", "dark-uncertainty", "dersimonian-laird", "linear-pool", "newtonian-constant"],
      "G", "10⁻¹¹ m³ kg⁻¹ s⁻²", G,
      {"mean": chk(6.67380, 0.000011), "u": chk(0.00028, 0.000005), "tau": chk(0.000946, 0.0000005), "q": chk(319.1, 0.05),
       "u_knapp_hartung": chk(0.0003066, 0.0000001),
       "bootstrap_u": chk(0.000279, 0.000006), "interval_lower": chk(6.67322, 0.00001), "interval_upper": chk(6.67435, 0.00001),
       "linear_pool_mean": chk(6.67367, 0.000006), "linear_pool_u": chk(0.00124, 0.000006),
       "linear_pool_lower": chk(6.67118, 0.00003), "linear_pool_upper": chk(6.67579, 0.00003),
       "ml_mean": chk(6.67378, 0.000005), "ml_u": chk(0.00030, 0.000005),
       "degrees_of_equivalence": {"UWash-00": {"d": chk(0.000465, 0.0000005)}, "LENS-14": {"d": chk(-0.001880, 0.0000005)}}},
      T("Koepke et al. (2017) Table 2: DerSimonian–Laird 6.673 80, u 0.000 28, (6.673 20, 6.674 40); Linear Pool 6.673 67, 0.001 24, (6.671 18, 6.675 79); maximum likelihood 6.673 78, 0.000 30. τ, Q, u_KH, the bootstrap u and percentiles and D_j: independent numpy computation (10⁵ replicates; the printed interval is consistent with them rounded to 10⁻⁴).",
        "Koepke vd. (2017) Tablo 2: DerSimonian–Laird 6,673 80, u 0,000 28, (6,673 20, 6,674 40); doğrusal havuz 6,673 67, 0,001 24, (6,671 18, 6,675 79); en çok olabilirlik 6,673 78, 0,000 30. τ, Q, u_KH, bootstrap u ve yüzdelikleri ile D_j: bağımsız numpy hesabı (10⁵ tekrar; basılı aralık bunların 10⁻⁴'e yuvarlanmışıyla uyumludur)."), ml=True)

# --- Half-lives of ¹³⁷Cs and ⁹⁰Sr (Table 3, Table 4; MacMahon et al. 2004) ----------------------------------
A = 365.25
CS = [("WT55", 9715, 146), ("Br55", 10957, 146), ("Fa61", 11103, 146), ("Fl62", 10994, 256), ("Go63", 10840, 18),
      ("Ri63", 10665, 110), ("Le65", 11220, 47), ("Fl65a", 10921, 183), ("Fl65b", 11286, 256), ("Ha70", 11191, 157),
      ("Em72", 11023, 37), ("DP73", 11020.8, 4.1), ("Co73", 11034, 29), ("GS78", 10906, 33), ("Ho80", 11009, 11),
      ("MT80", 10967.8, 4.5), ("Go92", 10940.8, 6.9), ("Un02", 11018.3, 9.5), ("Sc04", 10970, 20)]
SR = [("WT55", 10120, 150), ("An58", 10700, 580), ("Fl65a", 10230, 150), ("Fl65b", 10410, 330), ("Ho77", 10636, 88),
      ("La78", 10282, 12), ("Ra83", 10588, 91), ("Ko89", 10665, 37), ("Ma94", 10561, 14), ("WL96", 10495, 4),
      ("Sc04", 10557, 11)]
REF = {"cs137": (30.04, 0.04, 29.94, 0.93, (29.9467, 30.1288), (26.571, 31.234), 0.044609, 0.928438),
       "sr90": (28.73, 0.11, 28.68, 0.77, (28.5009, 28.9593), (27.168, 30.341), 0.110185, 0.772442)}
for key, labs, nuc, nuc_tr in (("cs137", CS, "¹³⁷Cs", "¹³⁷Cs"), ("sr90", SR, "⁹⁰Sr", "⁹⁰Sr")):
    x, u = [a[1] for a in labs], [a[2] for a in labs]
    mu, um, tau, q, ukh = dl(x, u)
    m_p, u_p, lpm_p, lpu_p, (lo, hi), (llo, lhi), ub, ulp = REF[key]
    n = len(labs)
    xbar = sum(x) / n
    assert round(mu / A, 2) == m_p and round(um / A, 2) == u_p and round(xbar / A, 2) == lpm_p
    write(f"koepke2017-half-life-{key}",
          T(f"Half-life of {nuc}: consensus value of {n} discrepant measurements (DerSimonian–Laird and Linear Pool; Koepke et al. 2017)",
            f"{nuc_tr} yarı ömrü: birbiriyle tutarsız {n} ölçümün uzlaşı değeri (DerSimonian–Laird ve doğrusal havuz; Koepke vd. 2017)"),
          T(f"The {n} published half-lives compiled by MacMahon et al. (2004) are heterogeneous (Q = {q:.0f} with {n - 1} degrees of freedom). The consensus value is computed in days, as the data are given; Koepke et al. report it in years of 365.25 d.",
            f"MacMahon vd. (2004) tarafından derlenen {n} yayımlanmış yarı ömür heterojendir (Q = {q:.0f}, serbestlik derecesi {n - 1}). Uzlaşı değeri, veriler verildiği gibi gün biriminde hesaplanır; Koepke vd. bunu 365,25 günlük yıl cinsinden bildirir."),
          [T(f"DerSimonian–Laird: μ̂ = {mu:.1f} d = {mu / A:.3f} a, u_DL = {um:.1f} d = {um / A:.3f} a, τ̂ = {tau:.1f} d.",
             f"DerSimonian–Laird: μ̂ = {c(f'{mu:.1f}')} gün = {c(f'{mu / A:.3f}')} a, u_DL = {c(f'{um:.1f}')} gün = {c(f'{um / A:.3f}')} a, τ̂ = {c(f'{tau:.1f}')} gün."),
           T(f"The Linear Pool gives {xbar / A:.2f} a with standard uncertainty {ulp:.2f} a: the mixture of the discrepant results is much wider than the random-effects estimate, which down-weights the results with large stated uncertainties less than the plain weighted mean would.",
             f"Doğrusal havuz {c(f'{xbar / A:.2f}')} a ve {c(f'{ulp:.2f}')} a standart belirsizlik verir: tutarsız sonuçların karışımı rastgele etkiler kestiriminden çok daha geniştir."),
           T("The degrees of equivalence D_j = x_j − μ̂ with U95 %(D_j) from the bootstrap show which results lie beyond the range allowed by the model (NICOB §A.5.1; Koepke et al., Figure 4).",
             "Bootstrap'tan U95 %(D_j) ile eşdeğerlik dereceleri D_j = x_j − μ̂, hangi sonuçların modelin izin verdiği aralığın dışında kaldığını gösterir (NICOB §A.5.1; Koepke vd., Şekil 4).")],
          [T("Several sources share a label in Table 3 (Fl65); they are distinguished here as Fl65a, Fl65b in the order of the table.",
             "Tablo 3'te birkaç kaynak aynı etiketi taşır (Fl65); burada tablodaki sıraya göre Fl65a, Fl65b olarak ayrılmıştır.")],
          "§7.2, Tables 3 and 4", ["koepke-2017", "nicob", "consensus", "dark-uncertainty", "dersimonian-laird", "linear-pool", "half-life", "radionuclide"],
          f"T½({nuc})", "d", labs,
          {"mean": chk(round(m_p * A, 2), round(0.005 * A, 2)), "u": chk(round(u_p * A, 2), round(0.005 * A, 2)),
           "tau": chk(round(tau, 2), 0.01), "q": chk(round(q, 2), 0.01),
           "bootstrap_u": chk(round(ub * A, 1), 0.4), "interval_lower": chk(round(lo * A, 1), 1.5), "interval_upper": chk(round(hi * A, 1), 1.5),
           # Tolerances: half a unit in the last printed digit plus three Monte Carlo standard errors of 10⁵ draws
           # (sd/√N ≈ 1 d for the mean; ≈ 6 d for a 2.5 % tail percentile of the mixture).
           "linear_pool_mean": chk(round(lpm_p * A, 2), round(0.005 * A + 3 * ulp * A / math.sqrt(1e5), 1)),
           "linear_pool_u": chk(round(lpu_p * A, 2), round(0.005 * A + 3 * ulp * A / math.sqrt(2e5), 1)),
           "linear_pool_lower": chk(round(llo * A, 1), 20.0), "linear_pool_upper": chk(round(lhi * A, 1), 20.0)},
          T(f"Koepke et al. (2017) Table 4: DerSimonian–Laird {m_p} a, u {u_p} a; Linear Pool {lpm_p} a, u {lpu_p} a (× 365.25 d, tolerance half a unit in the last printed digit, plus the Monte Carlo error of 10⁵ draws for the Linear Pool). τ, Q, the bootstrap u and percentiles and the Linear Pool percentiles: independent numpy computation.",
            f"Koepke vd. (2017) Tablo 4: DerSimonian–Laird {c(m_p)} a, u {c(u_p)} a; doğrusal havuz {c(lpm_p)} a, u {c(lpu_p)} a (× 365,25 gün, tolerans son basamağın yarım birimi; doğrusal havuzda buna 10⁵ çekilişin Monte Carlo hatası eklenir). τ, Q, bootstrap u ve yüzdelikleri ile doğrusal havuz yüzdelikleri: bağımsız numpy hesabı."))
