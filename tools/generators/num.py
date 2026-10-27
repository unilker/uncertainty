"""Writes library projects from the NIST Uncertainty Machine manual and GUM H.2."""
import json, math
from pathlib import Path

NUM = {"document": "T. Lafarge, A. Possolo, NIST Uncertainty Machine — User's Manual, version 1.4 (NIST, 2020)",
       "url": "https://uncertainty.nist.gov/NISTUncertaintyMachine-UserManual.pdf"}
GUM = {"document": "JCGM 100:2008, Evaluation of measurement data — Guide to the expression of uncertainty in measurement",
       "url": "https://www.bipm.org/documents/20126/2071204/JCGM_100_2008_E.pdf"}

def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def src(d, sec): return {**{"document": d["document"]}, "section": sec, "url": d["url"]}
def inp(name, unit, en, tr, dist):
    d = {"name": name}
    if unit: d["unit"] = unit
    d["description"] = T(en, tr); d["distribution"] = dist
    return d
def normal(m, s): return {"type": "normal", "mean": m, "sd": s}

def project(path, pid, title, desc, expl, field, sources, tags, notes, model, analysis, expected, reporting=None):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": field,
                      "sources": sources, "tags": tags}}
    if notes: p["metadata"]["notes"] = notes
    p["model"] = model; p["analysis"] = analysis
    if reporting: p["reporting"] = reporting
    p["expected"] = expected
    Path(path).write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

# ------------------------------------------------------------- ABC
project("library/general/num-3-ratio-abc.json", "num-3-ratio-abc",
  T("Product and ratio y = ab/c with a wide triangular divisor (NIST Uncertainty Machine §3)",
    "Geniş üçgen bölenli çarpım ve oran y = ab/c (NIST Uncertainty Machine §3)"),
  T("NUM manual §3: Gaussian a, rectangular b and a triangular divisor c with a 30 % relative uncertainty; the first-order GUM result is far from the Monte Carlo result.",
    "NUM kılavuzu §3: normal a, dikdörtgen b ve bağıl belirsizliği %30 olan üçgen bölen c; birinci mertebe GUM sonucu Monte Carlo sonucundan çok uzaktır."),
  [T("a ~ N(32, 0.5²); b rectangular with mean 0.9 and standard deviation 0.025 (half-width 0.025√3); c symmetric triangular with mean 1 and standard deviation 0.3 (half-width 0.3√6 = 0.735).",
     "a ~ N(32, 0,5²); b ortalaması 0,9 ve standart sapması 0,025 olan dikdörtgen (yarı genişlik 0,025√3); c ortalaması 1 ve standart sapması 0,3 olan simetrik üçgen (yarı genişlik 0,3√6 = 0,735)."),
   T("Gauss's formula at the estimates: y = 32 × 0.9 / 1 = 28.8 and u(y)/y = √((0.5/32)² + (0.025/0.9)² + 0.3²), u(y) = 8.7; c contributes 99 % of the variance.",
     "Tahminlerde Gauss formülü: y = 32 × 0,9 / 1 = 28,8 ve u(y)/y = √((0,5/32)² + (0,025/0,9)² + 0,3²), u(y) = 8,7; varyansın %99'u c'den gelir."),
   T("Because 1/c is strongly convex over [0.27, 1.73], the Monte Carlo mean is 32.2, the standard deviation 13 and the 95 % interval (18.2, 67): the distribution of y is markedly right-skewed and the GUM interval 28.8 ± 17.1 is not validated.",
     "1/c, [0,27; 1,73] aralığında belirgin biçimde dışbükey olduğundan Monte Carlo ortalaması 32,2, standart sapması 13 ve %95 aralığı (18,2; 67) olur: y'nin dağılımı belirgin biçimde sağa çarpıktır ve GUM aralığı 28,8 ± 17,1 doğrulanmaz.")],
  "general", [src(NUM, "§3, Figure 1")], ["nist-uncertainty-machine", "non-linear", "triangular", "skewed", "validation"],
  [T("The text of §3 says that the GUM method gives y = 32.2 and u(y) = 12.5, but the output of the NIST Uncertainty Machine printed in Figure 1 gives y = 28.8 and u(y) = 8.7 for Gauss's formula (32.2 and 13 are the Monte Carlo mean and standard deviation). The program reproduces the values of Figure 1.",
     "§3'ün metni GUM yönteminin y = 32,2 ve u(y) = 12,5 verdiğini söyler; ancak Şekil 1'de basılı NIST Uncertainty Machine çıktısı Gauss formülü için y = 28,8 ve u(y) = 8,7 verir (32,2 ve 13 Monte Carlo ortalaması ve standart sapmasıdır). Program Şekil 1'deki değerleri yeniden üretir.")],
  {"output": {"name": "y", "expression": "a * b / c", "description": T("Output quantity", "Çıktı büyüklüğü")},
   "inputs": [inp("a", None, "Gaussian factor", "Normal çarpan", normal(32, 0.5)),
              inp("b", None, "Rectangular factor", "Dikdörtgen çarpan", {"type": "rectangular", "value": 0.9, "half_width": round(0.025 * math.sqrt(3), 7)}),
              inp("c", None, "Triangular divisor", "Üçgen bölen", {"type": "triangular", "value": 1.0, "half_width": round(0.3 * math.sqrt(6), 6)})]},
  {"gum": {}, "mcm": {"trials": 2000000, "seed": 3, "interval": "symmetric"}},
  {"gum": {"estimate": chk(28.8, 0.05), "standard_uncertainty": chk(8.7, 0.05)},
   "mcm": {"estimate": chk(32.2, 0.05), "standard_uncertainty": chk(13.0, 0.5),
           "interval_low": chk(18.2, 0.05), "interval_high": chk(67.0, 0.5), "gum_validated": False},
   "reference": T("NIST Uncertainty Machine manual §3, Figure 1 (Gauss's formula y = 28.8, u = 8.7; Monte Carlo, M = 10⁶: mean 32.2, sd 13, 95 % interval (18.2, 67)).",
                  "NIST Uncertainty Machine kılavuzu §3, Şekil 1 (Gauss formülü y = 28,8, u = 8,7; Monte Carlo, M = 10⁶: ortalama 32,2, standart sapma 13, %95 aralığı (18,2; 67)).")},
  reporting={"significant_digits": 2})

# ------------------------------------------------------------- GUM H.2
import numpy as _np
_V = [5.007, 4.994, 5.005, 4.990, 4.999]; _I = [19.663, 19.639, 19.640, 19.685, 19.678]; _P = [1.0456, 1.0438, 1.0468, 1.0428, 1.0433]
_C = _np.cov(_np.vstack([_V, _I, _P])) / 5
_s = _np.sqrt(_np.diag(_C)); _r = _C / _np.outer(_s, _s)
def _h2_inputs(s, means):
    return [inp("V", "V", "Mean of five voltage readings", "Beş gerilim okumasının ortalaması", normal(means[0], s[0])),
            inp("I", "mA", "Mean of five current readings", "Beş akım okumasının ortalaması", normal(means[1], s[1])),
            inp("phi", "rad", "Mean of five phase-angle readings", "Beş faz açısı okumasının ortalaması", normal(means[2], s[2]))]
H2_IN = _h2_inputs([round(float(x), 9) for x in _s], [4.9990, 19.6610, 1.04446])
H2_COR = [{"a": "V", "b": "I", "r": round(float(_r[0, 1]), 6)}, {"a": "V", "b": "phi", "r": round(float(_r[0, 2]), 6)}, {"a": "I", "b": "phi", "r": round(float(_r[1, 2]), 6)}]
H2_IN_ROUNDED = _h2_inputs([0.0032, 0.0095, 0.00075], [4.9990, 19.6610, 1.04446])
H2_COR_ROUNDED = [{"a": "V", "b": "I", "r": -0.36}, {"a": "V", "b": "phi", "r": 0.86}, {"a": "I", "b": "phi", "r": -0.65}]
H2_EXPL = [T("Five simultaneous sets of readings of the voltage V across an element, the current I through it and the phase angle φ give the means of GUM Table H.2 with s(x̄) = 0.0032 V, 0.0095 mA and 0.00075 rad, and the correlation coefficients r(V, I) = −0.36, r(V, φ) = 0.86, r(I, φ) = −0.65 (GUM 5.2.3). The project uses these values unrounded, recomputed from the five sets of Table H.2 (s(x̄) = 0.003209 V, 0.009471 mA, 0.000752 rad; r = −0.3553, 0.8576, −0.6451).",
             "Bir devre elemanının uçlarındaki V gerilimi, içinden geçen I akımı ve φ faz açısının beş eşzamanlı okuma takımı, GUM Tablo H.2'deki ortalamaları s(x̄) = 0,0032 V, 0,0095 mA ve 0,00075 rad ile ve r(V, I) = −0,36, r(V, φ) = 0,86, r(I, φ) = −0,65 korelasyon katsayılarıyla verir (GUM 5.2.3). Proje bu değerleri Tablo H.2'deki beş takımdan yeniden hesaplanmış, yuvarlanmamış biçimde kullanır (s(x̄) = 0,003209 V, 0,009471 mA, 0,000752 rad; r = −0,3553; 0,8576; −0,6451)."),
           T("Approach 1 of GUM H.2.3 evaluates the model at the means and propagates with Equation (16), which includes the covariance terms 2c_ic_ju(x_i)u(x_j)r(x_i, x_j).",
             "GUM H.2.3'ün 1. yaklaşımı modeli ortalamalarda hesaplar ve kovaryans terimlerini, 2c_ic_ju(x_i)u(x_j)r(x_i, x_j), içeren Eşitlik (16) ile yayar.")]
def h2(pid, name, title_en, title_tr, expr, desc_en, desc_tr, y, u, extra_expl, mcm=None, inputs=None, cor=None, notes=None, u_tol=0.0005, expl0=None, gum_ref=True):
    expl = (expl0 if expl0 is not None else H2_EXPL) + extra_expl
    expected = {"gum": {"estimate": chk(y, 0.0005), "standard_uncertainty": chk(u, u_tol)}}
    sources = [src(GUM, "H.2, Tables H.2 and H.3")]
    ref_en = f"JCGM 100:2008 Table H.3: {name} = {y} Ω, u_c = {u} Ω" if gum_ref else "GUM: law of propagation with the rounded inputs (0.0700 Ω)"
    ref_tr = f"JCGM 100:2008 Tablo H.3: {name} = {str(y).replace('.', ',')} Ω, u_c = {str(u).replace('.', ',')} Ω" if gum_ref else "GUM: yuvarlanmış girdilerle yayılım yasası (0,0700 Ω)"
    if mcm:
        expected["mcm"] = mcm["checks"]
        sources.append(src(NUM, "§10, Figure 6"))
        ref_en += "; " + mcm["en"]; ref_tr += "; " + mcm["tr"]
    expected["reference"] = T(ref_en + ".", ref_tr + ".")
    project(f"library/electrical/{pid}.json", pid, T(title_en, title_tr),
      T(f"GUM H.2: {desc_en} from correlated simultaneous measurements of voltage, current and phase angle.",
        f"GUM H.2: gerilim, akım ve faz açısının korelasyonlu eşzamanlı ölçümlerinden {desc_tr}."),
      expl, "electrical", sources, ["gum", "correlation", "validation"] + (["nist-uncertainty-machine"] if mcm else []), notes,
      {"output": {"name": name, "unit": "Ω", "expression": expr, "description": T(desc_en.capitalize(), desc_tr.capitalize())},
       "inputs": inputs or H2_IN, "correlations": cor or H2_COR},
      {"gum": {}, "mcm": {"trials": 1000000, "seed": 2, "interval": "symmetric"}}, expected, reporting={"significant_digits": 2})

ROUND_NOTE = [T("Because the covariance terms cancel most of the variance of R, u_c(R) is sensitive to the rounding of the correlation coefficients: with the two-digit values printed in Table H.2 the law of propagation gives 0.0700 Ω instead of 0.0711 Ω (project 'num-10-resistance', which reproduces the NIST Uncertainty Machine). X and Z change by less than 0.1 %.",
                "Kovaryans terimleri R'nin varyansının büyük kısmını götürdüğünden u_c(R), korelasyon katsayılarının yuvarlanmasına duyarlıdır: Tablo H.2'de basılı iki basamaklı değerlerle yayılım yasası 0,0711 Ω yerine 0,0700 Ω verir (NIST Uncertainty Machine'i yeniden üreten 'num-10-resistance' projesi). X ve Z %0,1'den az değişir.")]
h2("gum-h2-resistance", "R", "Resistance from correlated V, I and φ (GUM H.2)", "Korelasyonlu V, I ve φ'den direnç (GUM H.2)",
   "1000 * V / I * cos(phi)", "the resistance R = (V/I) cos φ", "R = (V/I) cos φ direnci", 127.732, 0.071,
   [T("For R = (V/I) cos φ the covariance terms reduce the variance: the sum of the squared contributions alone is about 3.6 times u_c²(R), and u_c(R) = 0.071 Ω. Approach 2 (R computed from each set, GUM Table H.4) gives the same value.",
      "R = (V/I) cos φ için kovaryans terimleri varyansı azaltır: katkıların karelerinin toplamı tek başına u_c²(R)'nin yaklaşık 3,6 katıdır ve u_c(R) = 0,071 Ω olur. 2. yaklaşım (R'nin her takımdan hesaplanması, GUM Tablo H.4) aynı değeri verir.")],
   notes=ROUND_NOTE)
h2("num-10-resistance", "R", "Resistance with the rounded values of GUM Table H.2 (NIST Uncertainty Machine §10)",
   "GUM Tablo H.2'nin yuvarlanmış değerleriyle direnç (NIST Uncertainty Machine §10)",
   "1000 * V / I * cos(phi)", "the resistance R = (V/I) cos φ", "R = (V/I) cos φ direnci", 127.732, 0.0700,
   [T("This project enters the rounded values printed in GUM Table H.2, as the NIST Uncertainty Machine manual does. The strong cancellation of the covariance terms makes u_c(R) = 0.0700 Ω, against 0.071 Ω from the unrounded data (project 'gum-h2-resistance'); the Monte Carlo method with a trivariate Gaussian distribution agrees with the law of propagation.",
      "Bu proje, NIST Uncertainty Machine kılavuzunun yaptığı gibi GUM Tablo H.2'de basılı yuvarlanmış değerleri girer. Kovaryans terimlerinin güçlü sadeleşmesi u_c(R) = 0,0700 Ω verir; yuvarlanmamış verilerden ise 0,071 Ω çıkar ('gum-h2-resistance' projesi). Üç değişkenli normal dağılımla Monte Carlo yöntemi yayılım yasasıyla uyuşur.")],
   mcm={"checks": {"estimate": chk(127.732, 0.0005), "standard_uncertainty": chk(0.0699, 0.0005),
                   "interval_low": chk(127.595, 0.0015), "interval_high": chk(127.869, 0.0015), "gum_validated": True},
        "en": "NIST Uncertainty Machine manual §10 (M = 10⁶): Gauss's formula u = 0.07 Ω; mean 127.732 Ω, sd 0.0699 Ω, 95 % interval (127.595, 127.869) Ω",
        "tr": "NIST Uncertainty Machine kılavuzu §10 (M = 10⁶): Gauss formülü u = 0,07 Ω; ortalama 127,732 Ω, standart sapma 0,0699 Ω, %95 aralığı (127,595; 127,869) Ω"},
   inputs=H2_IN_ROUNDED, cor=H2_COR_ROUNDED, u_tol=0.00005, gum_ref=False,
   expl0=[T("Five simultaneous sets of readings of V, I and φ give the means of GUM Table H.2 with s(x̄) = 0.0032 V, 0.0095 mA and 0.00075 rad and r(V, I) = −0.36, r(V, φ) = 0.86, r(I, φ) = −0.65, entered here as printed.",
            "V, I ve φ'nin beş eşzamanlı okuma takımı GUM Tablo H.2'deki ortalamaları s(x̄) = 0,0032 V, 0,0095 mA ve 0,00075 rad ile ve r(V, I) = −0,36, r(V, φ) = 0,86, r(I, φ) = −0,65 ile verir; değerler burada basıldığı gibi girilmiştir.")])
h2("gum-h2-reactance", "X", "Reactance from correlated V, I and φ (GUM H.2)", "Korelasyonlu V, I ve φ'den reaktans (GUM H.2)",
   "1000 * V / I * sin(phi)", "the reactance X = (V/I) sin φ", "X = (V/I) sin φ reaktansı", 219.847, 0.295, u_tol=0.0006, notes=[T("Approach 1 with the unrounded data gives u_c(X) = 0.29558 Ω, approach 2 0.29549 Ω; GUM Table H.3 prints 0.295, so the tolerance is 0.0006 Ω.", "Yuvarlanmamış verilerle 1. yaklaşım u_c(X) = 0,29558 Ω, 2. yaklaşım 0,29549 Ω verir; GUM Tablo H.3 0,295 bastığından tolerans 0,0006 Ω'dur.")], extra_expl=
   [T("For X = (V/I) sin φ the correlation of V and φ increases the variance; u_c(X) = 0.295 Ω (relative 0.13 %).",
      "X = (V/I) sin φ için V ile φ arasındaki korelasyon varyansı artırır; u_c(X) = 0,295 Ω (bağıl %0,13).")])
h2("gum-h2-impedance", "Z", "Impedance magnitude from correlated V and I (GUM H.2)", "Korelasyonlu V ve I'dan empedans büyüklüğü (GUM H.2)",
   "1000 * V / I", "the impedance magnitude Z = V/I", "Z = V/I empedans büyüklüğü", 254.260, 0.236,
   [T("For Z = V/I, GUM Equation (H.8c) gives u_r²(Z) = u_r²(V) + u_r²(I) − 2u_r(V)u_r(I)r(V, I); with r(V, I) = −0.36 the last term adds to the variance, and u_c(Z) = 0.236 Ω.",
      "Z = V/I için GUM Eşitlik (H.8c) u_r²(Z) = u_r²(V) + u_r²(I) − 2u_r(V)u_r(I)r(V, I) verir; r(V, I) = −0,36 olduğundan son terim varyansı artırır ve u_c(Z) = 0,236 Ω olur.")])

# ------------------------------------------------------------- Stefan–Boltzmann
project("library/physical/num-11-stefan-boltzmann.json", "num-11-stefan-boltzmann",
  T("Stefan–Boltzmann constant from CODATA 2010 values (NIST Uncertainty Machine §11)",
    "CODATA 2010 değerlerinden Stefan–Boltzmann sabiti (NIST Uncertainty Machine §11)"),
  T("NUM manual §11: σ = 32π⁵hR⁴R∞⁴/(15A_r(e)⁴M_u⁴c⁶α⁸) from the 2010 CODATA values; the molar gas constant R contributes almost all the uncertainty.",
    "NUM kılavuzu §11: 2010 CODATA değerlerinden σ = 32π⁵hR⁴R∞⁴/(15A_r(e)⁴M_u⁴c⁶α⁸); belirsizliğin neredeyse tamamı molar gaz sabiti R'den gelir."),
  [T("Inputs: h = 6.626 069 57(29) × 10⁻³⁴ J s, R = 8.314 462 1(75) J mol⁻¹ K⁻¹, R∞ = 10 973 731.568 539(55) m⁻¹, A_r(e) = 5.485 799 094 6(22) × 10⁻⁴, α = 7.297 352 569 8(24) × 10⁻³; M_u = 10⁻³ kg/mol and c = 299 792 458 m/s are exact. The result is expressed in 10⁻⁸ W m⁻² K⁻⁴.",
     "Girdiler: h = 6,626 069 57(29) × 10⁻³⁴ J s, R = 8,314 462 1(75) J mol⁻¹ K⁻¹, R∞ = 10 973 731,568 539(55) m⁻¹, A_r(e) = 5,485 799 094 6(22) × 10⁻⁴, α = 7,297 352 569 8(24) × 10⁻³; M_u = 10⁻³ kg/mol ve c = 299 792 458 m/s kesindir. Sonuç 10⁻⁸ W m⁻² K⁻⁴ biriminde verilir."),
   T("For a product of powers the relative variance is Σ(p_i u_r(x_i))²: R enters with power 4 and u_r(R) = 9.0 × 10⁻⁷, so u_r(σ) = 3.6 × 10⁻⁶ and u(σ) = 2.05 × 10⁻¹³ W m⁻² K⁻⁴; the other inputs contribute less than 0.02 %.",
     "Üslerin çarpımında bağıl varyans Σ(p_i u_r(x_i))²'dir: R 4. kuvvetle girer ve u_r(R) = 9,0 × 10⁻⁷ olduğundan u_r(σ) = 3,6 × 10⁻⁶ ve u(σ) = 2,05 × 10⁻¹³ W m⁻² K⁻⁴ olur; diğer girdilerin katkısı %0,02'nin altındadır.")],
  "physical", [src(NUM, "§11, Table 5 and Figure 7")], ["nist-uncertainty-machine", "codata", "power-law"],
  [T("The CODATA adjustment correlates these constants; as noted in the manual, including the correlations does not change u(σ) at the reported digit. Since the 2019 redefinition of the SI, h is exact and σ = 5.670 374 419… × 10⁻⁸ W m⁻² K⁻⁴ has no uncertainty.",
     "CODATA ayarlaması bu sabitleri korelasyonlu kılar; kılavuzda belirtildiği gibi korelasyonların hesaba katılması u(σ)'yı raporlanan basamakta değiştirmez. SI'nın 2019'daki yeniden tanımından beri h kesindir ve σ = 5,670 374 419… × 10⁻⁸ W m⁻² K⁻⁴ belirsizlik taşımaz.")],
  {"output": {"name": "sigma", "unit": "10⁻⁸ W m⁻² K⁻⁴",
              "expression": "1e8 * 32 * pi^5 * h * R^4 * R_inf^4 / (15 * A_e^4 * (1e-3)^4 * 299792458^6 * alpha^8)",
              "description": T("Stefan–Boltzmann constant", "Stefan–Boltzmann sabiti")},
   "inputs": [inp("h", "J s", "Planck constant", "Planck sabiti", normal(6.62606957e-34, 0.00000029e-34)),
              inp("R", "J/(mol K)", "Molar gas constant", "Molar gaz sabiti", normal(8.3144621, 0.0000075)),
              inp("R_inf", "1/m", "Rydberg constant", "Rydberg sabiti", normal(10973731.568539, 0.000055)),
              inp("A_e", None, "Relative atomic mass of the electron", "Elektronun bağıl atom kütlesi", normal(5.4857990946e-4, 0.0000000022e-4)),
              inp("alpha", None, "Fine-structure constant", "İnce yapı sabiti", normal(7.2973525698e-3, 0.0000000024e-3))]},
  {"gum": {}, "mcm": {"trials": 1000000, "seed": 11, "interval": "symmetric"}},
  {"gum": {"estimate": chk(5.6703725, 0.00000005), "standard_uncertainty": chk(0.0000205, 0.0000005)},
   "mcm": {"standard_uncertainty": chk(0.0000205, 0.0000005), "interval_low": chk(5.670332, 0.0000005),
           "interval_high": chk(5.6704126, 0.0000005), "gum_validated": True},
   "reference": T("NIST Uncertainty Machine manual §11, Figure 7: σ = 5.670 3725 × 10⁻⁸, u(σ) = 2.05 × 10⁻¹³ W m⁻² K⁻⁴ (Gauss's formula and Monte Carlo), 95 % interval (5.670 332, 5.670 412 6) × 10⁻⁸ W m⁻² K⁻⁴.",
                  "NIST Uncertainty Machine kılavuzu §11, Şekil 7: σ = 5,670 3725 × 10⁻⁸, u(σ) = 2,05 × 10⁻¹³ W m⁻² K⁻⁴ (Gauss formülü ve Monte Carlo), %95 aralığı (5,670 332; 5,670 412 6) × 10⁻⁸ W m⁻² K⁻⁴.")},
  reporting={"significant_digits": 3})

# ------------------------------------------------------------- End gauge (GUM H.1 as in NUM §8)
project("library/length/num-8-end-gauge-gaussian.json", "num-8-end-gauge-gaussian",
  T("End-gauge calibration with Gaussian inputs (NIST Uncertainty Machine §8)",
    "Normal girdilerle uç mastarı kalibrasyonu (NIST Uncertainty Machine §8)"),
  T("NUM manual §8: GUM H.1 with the six inputs of the GUM's first-order budget, all modelled as Gaussian; the Monte Carlo method finds the second-order contributions that Gauss's formula misses.",
    "NUM kılavuzu §8: GUM H.1'in birinci mertebe bütçesindeki altı girdiyle, hepsi normal dağılımlı; Monte Carlo yöntemi Gauss formülünün kaçırdığı ikinci mertebe katkıları bulur."),
  [T("l = l_S + d − l_S(δα·θ + α_S·δθ) with l_S = 50 000 623(25) nm, d = 215(9.7) nm, δα = 0(0.58 × 10⁻⁶) °C⁻¹, θ = −0.1(0.41) °C, α_S = 11.5(1.2) × 10⁻⁶ °C⁻¹ and δθ = 0(0.029) °C (GUM Table H.1), all Gaussian.",
     "l = l_S + d − l_S(δα·θ + α_S·δθ); l_S = 50 000 623(25) nm, d = 215(9,7) nm, δα = 0(0,58 × 10⁻⁶) °C⁻¹, θ = −0,1(0,41) °C, α_S = 11,5(1,2) × 10⁻⁶ °C⁻¹ ve δθ = 0(0,029) °C (GUM Tablo H.1), hepsi normal dağılımlı."),
   T("At the estimates δα = δθ = 0, so θ and α_S have zero sensitivity coefficients and Gauss's formula gives u(l) = 31.7 nm; the program warns that the second-order products δα·θ and α_S·δθ are missed.",
     "Tahminlerde δα = δθ = 0 olduğundan θ ve α_S'nin duyarlılık katsayıları sıfırdır ve Gauss formülü u(l) = 31,7 nm verir; program δα·θ ve α_S·δθ ikinci mertebe çarpımlarının kaçırıldığı uyarısını verir."),
   T("The Monte Carlo method gives a standard deviation of 33.9 nm and the 99 % probabilistically symmetric interval (50 000 749, 50 000 927) nm, close to the GUM's own interval (50 000 745, 50 000 931) nm (k = 2.92 from ν_eff = 16.7). The interval of this project's GUM evaluation, 50 000 838 ± 82 nm (k = 2.58), is too short by 7 nm at each end and is not validated.",
     "Monte Carlo yöntemi 33,9 nm standart sapma ve %99 olasılıksal simetrik aralığı (50 000 749; 50 000 927) nm verir; bu, GUM'un kendi aralığına, (50 000 745; 50 000 931) nm (ν_eff = 16,7'den k = 2,92), yakındır. Bu projenin GUM değerlendirmesinin aralığı, 50 000 838 ± 82 nm (k = 2,58), her uçta 7 nm kısadır ve doğrulanmaz.")],
  "length", [src(NUM, "§8, Table 3 and Figure 3"), src(GUM, "H.1, Table H.1")], ["nist-uncertainty-machine", "gum", "second-order", "validation"],
  [T("The project 'gum-h1-end-gauge' reproduces the GUM's own evaluation (θ split into its mean and cyclic part, Welch–Satterthwaite, k = 2.92).",
     "'gum-h1-end-gauge' projesi GUM'un kendi değerlendirmesini yeniden üretir (θ ortalama ve çevrimsel kısmına ayrılır, Welch–Satterthwaite, k = 2,92).")],
  {"output": {"name": "l", "unit": "nm", "expression": "l_S + d - l_S * (d_alpha * theta + alpha_S * d_theta)",
              "description": T("Length of the end gauge at 20 °C", "Uç mastarının 20 °C'deki uzunluğu")},
   "inputs": [inp("l_S", "nm", "Length of the reference standard", "Referans standardın uzunluğu", normal(50000623, 25)),
              inp("d", "nm", "Measured difference in length", "Ölçülen uzunluk farkı", normal(215, 9.7)),
              inp("d_alpha", "1/°C", "Difference of the expansion coefficients", "Genleşme katsayılarının farkı", normal(0, 0.58e-6)),
              inp("theta", "°C", "Deviation of the temperature from 20 °C", "Sıcaklığın 20 °C'den sapması", normal(-0.1, 0.41)),
              inp("alpha_S", "1/°C", "Expansion coefficient of the standard", "Standardın genleşme katsayısı", normal(11.5e-6, 1.2e-6)),
              inp("d_theta", "°C", "Temperature difference between the gauges", "Mastarlar arasındaki sıcaklık farkı", normal(0, 0.029))]},
  {"gum": {"coverage_probability": 0.99}, "mcm": {"trials": 2000000, "seed": 8, "interval": "symmetric"}},
  {"gum": {"estimate": chk(50000838, 0.5), "standard_uncertainty": chk(31.7, 0.05)},
   "mcm": {"estimate": chk(50000837.9, 0.1), "standard_uncertainty": chk(33.9, 0.1),
           "interval_low": chk(50000749, 1.5), "interval_high": chk(50000927, 1.5), "gum_validated": False},
   "reference": T("NIST Uncertainty Machine manual §8, Figure 3 (Gauss's formula 50 000 838 nm, u = 31.7 nm; Monte Carlo, M = 10⁶: mean 50 000 837.9 nm, sd 33.9 nm, 99 % symmetric interval (50 000 749, 50 000 927) nm).",
                  "NIST Uncertainty Machine kılavuzu §8, Şekil 3 (Gauss formülü 50 000 838 nm, u = 31,7 nm; Monte Carlo, M = 10⁶: ortalama 50 000 837,9 nm, standart sapma 33,9 nm, %99 simetrik aralık (50 000 749; 50 000 927) nm).")},
  reporting={"significant_digits": 2})
