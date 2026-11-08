"""Writes the library projects of Possolo & Meija, Measurement Uncertainty: A Reintroduction, 2nd ed.
(Sistema Interamericano de Metrología, 2022; CC BY-SA 4.0). Run from the repository root.
Printed values are recomputed and asserted where the model allows it.
"""
import json
import math
from pathlib import Path

DOC = "Possolo & Meija, Measurement Uncertainty: A Reintroduction, 2nd ed., SIM, Montevideo (2022)"
URL = "https://doi.org/10.4224/1tqz-b038"


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


def write(field, pid, body, meta, expected, ref, analysis=None):
    section = meta.pop("section")
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "field": field, **meta, "sources": [{"document": DOC, "section": section, "url": URL}]},
         **body}
    if analysis is not None:
        p["analysis"] = analysis
    p["expected"] = {**expected, "reference": ref}
    Path(f"library/{field}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("wrote", pid)


def median(v):
    v = sorted(v)
    n = len(v)
    return v[n // 2] if n % 2 else 0.5 * (v[n // 2 - 1] + v[n // 2])


# --- Consensus Building: Hubble–Lemaître constant (pp. 146–151) -------------------------------------------
H0 = [("h0li", 72.50, 2.20), ("des", 67.80, 1.30), ("ryan", 69.32, 1.42), ("hst-riess19", 74.03, 1.42),
      ("flat", 67.40, 6.10), ("lv", 70.30, 5.15), ("h0l6", 73.30, 1.75), ("hst-freedman19", 69.80, 1.90),
      ("rpr", 73.50, 1.40), ("dutta", 70.30, 1.35), ("sh3", 76.80, 2.60), ("stri", 74.20, 2.85),
      ("mega", 73.90, 3.00), ("jaeg", 75.80, 5.05), ("muk", 67.60, 4.25), ("sedg", 67.40, 1.00),
      ("denz", 71.80, 3.59), ("bax", 73.50, 5.30), ("hstga", 73.20, 1.30), ("hstgb", 72.10, 2.00),
      ("free", 69.80, 1.70), ("sh0es", 73.30, 1.40)]
xs = [v for _, v, _ in H0]
us = [s for _, _, s in H0]
m = sum(xs) / len(xs)
sd = math.sqrt(sum((v - m) ** 2 for v in xs) / (len(xs) - 1))
assert round(sd, 2) == 2.76 and median(us) == 1.95 and round(math.sqrt(2.76 ** 2 - 1.95 ** 2), 2) == 1.95
gamma = round(1.4826 * median([abs(v - median(xs)) for v in xs]), 5)  # R mad(): 1.4826 × median |x − median|
z = (71.69 - 67.36) / math.sqrt(0.66 ** 2 + 0.54 ** 2)
assert round(z, 1) == 5.1
write("physical", "possolo-meija-2022-hubble-lemaitre-constant-laplace",
      {"consensus": {"measurand": "H₀", "unit": "(km/s)/Mpc",
                     "results": [{"name": n, "value": v, "u": s} for n, v, s in H0],
                     "bayes": {"tau_median": gamma, "mu_mean": 67.4, "mu_sd": 100, "effects": "laplace"},
                     "trials": 100000, "seed": 1}},
      {"title": T("Hubble–Lemaître constant from 22 mutually inconsistent estimates: Bayesian consensus with Laplace effects (Possolo & Meija 2022)",
                  "Birbiriyle tutarsız 22 kestirimden Hubble–Lemaître sabiti: Laplace etkili Bayesçi uzlaşı (Possolo & Meija 2022)"),
       "description": T("Recent estimates of H₀ (Planck's excluded) are more dispersed than their uncertainties suggest. The model H₀,j = H₀ + λ_j + ε_j with Laplace experiment effects λ_j (standard deviation τ), as the NIST Decision Tree recommends, and Gaussian errors with the reported u_j blends them into a consensus value.",
                        "H₀'ın son kestirimleri (Planck hariç) belirsizliklerinin öngördüğünden daha fazla dağılır. NIST Decision Tree'nin önerdiği gibi Laplace deney etkileri λ_j (standart sapma τ) ve bildirilen u_j'li normal hatalarla H₀,j = H₀ + λ_j + ε_j modeli bunları bir uzlaşı değerinde birleştirir."),
       "explanation": [T(f"The standard deviation of the 22 estimates is {sd:.2f} and the median of their uncertainties 1.95 (km/s)/Mpc, so a preliminary τ is √(2.76² − 1.95²) = 1.95 (km/s)/Mpc.",
                         f"22 kestirimin standart sapması {c(f'{sd:.2f}')}, belirsizliklerinin ortancası 1,95 (km/s)/Mpc'dir; ön kestirim τ = √(2,76² − 1,95²) = 1,95 (km/s)/Mpc."),
                       T(f"Priors as in the Stan code of the book: H₀ ~ N(67.4, 100²); τ half-Cauchy with median mad(x) = 1.4826 × median|x − median(x)| = {gamma} (km/s)/Mpc. The Laplace effects have scale τ/√2, so that τ is their standard deviation.",
                         f"Önseller kitabın Stan kodundaki gibidir: H₀ ~ N(67,4, 100²); τ ortancası mad(x) = 1,4826 × ortanca|x − ortanca(x)| = {c(gamma)} (km/s)/Mpc olan yarım Cauchy. Laplace etkilerinin ölçeği τ/√2'dir, böylece τ bunların standart sapmasıdır."),
                       T("The likelihood of each estimate is the convolution of a Laplace and a Gaussian density, which has a closed form in erfc; the posterior of (H₀, τ) is evaluated on a two-dimensional grid instead of by MCMC (the book: Stan, 4 chains of 10⁶ iterations).",
                         "Her kestirimin olabilirliği bir Laplace ve bir normal yoğunluğun evrişimidir ve erfc ile kapalı bir biçimi vardır; (H₀, τ) sonsalı MCMC yerine iki boyutlu bir ızgarada hesaplanır (kitap: Stan, 10⁶ yinelemeli 4 zincir)."),
                       T(f"Posterior: H₀ = 71.69 (km/s)/Mpc, u = 0.66 (km/s)/Mpc, median τ = 2.03 (km/s)/Mpc, as printed. Compared with Planck's 67.36(54), z = (71.69 − 67.36)/√(0.66² + 0.54²) = {z:.1f}: the Hubble tension.",
                         f"Sonsal: basılı değerlerle aynı, H₀ = 71,69 (km/s)/Mpc, u = 0,66 (km/s)/Mpc, τ ortancası 2,03 (km/s)/Mpc. Planck'ın 67,36(54) değeriyle karşılaştırma: z = (71,69 − 67,36)/√(0,66² + 0,54²) = {c(f'{z:.1f}')}; Hubble gerilimi.")],
       "section": "Consensus Building, Hubble-Lemaître Constant, pp. 146–151",
       "tags": ["possolo-meija-2022", "consensus", "dark-uncertainty", "bayes", "laplace", "nist-decision-tree", "cosmology"],
       "notes": [T("Labels follow the book; the two studies labelled hst are distinguished by their first authors (Riess 2019, Freedman 2019). Mildly asymmetric uncertainties were replaced in the book by the geometric mean of the left and right uncertainties.",
                   "Etiketler kitabı izler; hst etiketli iki çalışma ilk yazarlarıyla ayrılmıştır (Riess 2019, Freedman 2019). Kitapta hafif asimetrik belirsizlikler sol ve sağ belirsizliklerin geometrik ortalamasıyla değiştirilmiştir."),
                 T("The DerSimonian–Laird estimate (Gaussian effects) is shown for comparison.",
                   "Karşılaştırma için DerSimonian–Laird kestirimi (normal etkiler) de gösterilmiştir.")]},
      {"consensus": {"bayes_mean": chk(71.69, 0.006), "bayes_u": chk(0.66, 0.006), "bayes_tau_median": chk(2.03, 0.01)}},
      T("Possolo & Meija (2022), p. 150: posterior mean 71.69 (km/s)/Mpc, standard deviation 0.66 (km/s)/Mpc, median of τ 2.03 (km/s)/Mpc (Stan). numpy grid (2000 × 1500): 71.6915, 0.6603, 2.0336.",
        "Possolo & Meija (2022), s. 150: sonsal ortalama 71,69 (km/s)/Mpc, standart sapma 0,66 (km/s)/Mpc, τ ortancası 2,03 (km/s)/Mpc (Stan). numpy ızgara (2000 × 1500): 71,6915, 0,6603, 2,0336."))


def inp(name, mean, sd, unit, en, tr):
    return {"name": name, "unit": unit, "description": T(en, tr), "distribution": {"type": "normal", "mean": mean, "sd": sd}}


# --- Sums, Products, and Ratios: plasma anion gap (pp. 16–18) -----------------------------------------------
uag = math.sqrt(1.48 ** 2 + 0.04 ** 2 + 0.72 ** 2 + 0.84 ** 2)
assert round(uag, 2) == 1.85
write("chemistry", "possolo-meija-2022-plasma-anion-gap",
      {"model": {"output": {"name": "AG", "unit": "mmol/L", "expression": "c_Na + c_K - c_Cl - c_HCO3",
                            "description": T("Plasma anion gap Δc_AG", "Plazma anyon açığı Δc_AG")},
                 "inputs": [inp("c_Na", 137, 1.48, "mmol/L", "Amount concentration of Na⁺", "Na⁺ miktar derişimi"),
                            inp("c_K", 4, 0.04, "mmol/L", "Amount concentration of K⁺", "K⁺ miktar derişimi"),
                            inp("c_Cl", 106, 0.72, "mmol/L", "Amount concentration of Cl⁻", "Cl⁻ miktar derişimi"),
                            inp("c_HCO3", 10, 0.84, "mmol/L", "Amount concentration of HCO₃⁻", "HCO₃⁻ miktar derişimi")]}},
      {"title": T("Plasma anion gap: a linear combination of four ion concentrations (Possolo & Meija 2022)",
                  "Plazma anyon açığı: dört iyon derişiminin doğrusal birleşimi (Possolo & Meija 2022)"),
       "description": T("Δc_AG = c(Na⁺) + c(K⁺) − c(Cl⁻) − c(HCO₃⁻) for one patient (White 2008); the uncertainties are reported as standard deviations.",
                        "Bir hasta için Δc_AG = c(Na⁺) + c(K⁺) − c(Cl⁻) − c(HCO₃⁻) (White 2008); belirsizlikler standart sapma olarak bildirilmiştir."),
       "explanation": [T("For a linear combination of uncorrelated inputs the variances add, whatever the signs: u²(Δc_AG) = 1.48² + 0.04² + 0.72² + 0.84² = (1.85 mmol/L)².",
                         "İlişkisiz girdilerin doğrusal birleşiminde varyanslar, işaretler ne olursa olsun toplanır: u²(Δc_AG) = 1,48² + 0,04² + 0,72² + 0,84² = (1,85 mmol/L)²."),
                       T("Δc_AG = 25 mmol/L; with Gaussian inputs the true value lies within 25 ± 1.85 mmol/L with about 68 % probability.",
                         "Δc_AG = 25 mmol/L; normal girdilerle gerçek değer yaklaşık %68 olasılıkla 25 ± 1,85 mmol/L içindedir.")],
       "section": "Sums, Products, and Ratios: Plasma Anion Gap, pp. 16–18",
       "tags": ["possolo-meija-2022", "clinical-chemistry", "linear", "electrolytes"], "notes": []},
      {"gum": {"estimate": chk(25, 1e-9), "standard_uncertainty": chk(1.85, 0.005)}},
      T("Possolo & Meija (2022), p. 17: Δc_AG = 25 mmol/L, u = 1.85 mmol/L.", "Possolo & Meija (2022), s. 17: Δc_AG = 25 mmol/L, u = 1,85 mmol/L."))

# --- Storage tank (pp. 18–24) -----------------------------------------------------------------------------
V = math.pi * 8.40 ** 2 * 32.50
uV = V * math.hypot(2 * 0.03 / 8.40, 0.07 / 32.50)
assert round(V) == 7204 and round(uV) == 54
write("length", "possolo-meija-2022-storage-tank-volume",
      {"model": {"output": {"name": "V", "unit": "m³", "expression": "pi * R^2 * H",
                            "description": T("Internal volume of a cylindrical storage tank", "Silindirik depolama tankının iç hacmi")},
                 "inputs": [inp("R", 8.40, 0.03, "m", "Radius, from the diameter of the roof measured with a tape", "Yarıçap, çatının şeritle ölçülen çapından"),
                            inp("H", 32.50, 0.07, "m", "Height, with a plumb line from the edge of the roof", "Yükseklik, çatı kenarından çekül ile")]}},
      {"title": T("Volume of a cylindrical storage tank: product of powers, Gauss's formula and Monte Carlo (Possolo & Meija 2022)",
                  "Silindirik depolama tankının hacmi: kuvvetler çarpımı, Gauss formülü ve Monte Carlo (Possolo & Meija 2022)"),
       "description": T("V = πR²H with R = 8.40(3) m and H = 32.50(7) m.", "V = πR²H; R = 8,40(3) m ve H = 32,50(7) m."),
       "explanation": [T(f"For a product of powers the relative variances add with the squares of the exponents: (u(V)/V)² = (2 × 0.03/8.40)² + (0.07/32.50)², so u(V) = {uV:.1f} m³ ≈ 54 m³ with V = {V:.0f} m³.",
                         f"Kuvvetler çarpımında bağıl varyanslar üslerin kareleriyle toplanır: (u(V)/V)² = (2 × 0,03/8,40)² + (0,07/32,50)²; V = {V:.0f} m³ ile u(V) = {c(f'{uV:.1f}')} m³ ≈ 54 m³."),
                       T("The Monte Carlo method with 10⁶ draws gives 7204 m³ give or take 54 m³ and a distribution indistinguishable from a Gaussian one.",
                         "10⁶ çekilişli Monte Carlo yöntemi 7204 m³ ± 54 m³ ve normalden ayırt edilemeyen bir dağılım verir.")],
       "section": "Sums, Products, and Ratios: Storage Tank, pp. 18–24",
       "tags": ["possolo-meija-2022", "volume", "product-of-powers", "monte-carlo"], "notes": []},
      {"gum": {"estimate": chk(7204, 0.5), "standard_uncertainty": chk(54, 0.5)},
       "mcm": {"estimate": chk(7204, 0.5), "standard_uncertainty": chk(54, 0.5)}},
      T("Possolo & Meija (2022), pp. 21–24: V = 7204 m³, give or take 54 m³ (Gauss's formula and Monte Carlo, K = 10⁶).",
        "Possolo & Meija (2022), s. 21–24: V = 7204 m³ ± 54 m³ (Gauss formülü ve Monte Carlo, K = 10⁶)."),
      analysis={"gum": {}, "mcm": {"trials": 1000000, "seed": 1, "interval": "symmetric"}})

# --- Wheatstone bridge (pp. 24–26) ------------------------------------------------------------------------
RE, RF, RG, RH = 951.0, 997.0, 909.0, 225.2e3
RU = RG * RF * (1 / RE + 1 / RH)
c2 = [(RG * RF / RE ** 2 * 5) ** 2, (RU / RF * 5) ** 2, (RU / RG * 0.5) ** 2, (RG * RF / RH ** 2 * 2.3e3) ** 2]
uRU = math.sqrt(sum(c2))
shares = [v / sum(c2) for v in c2]
assert round(RU) == 957 and round(uRU) == 7 and [round(100 * s) for s in shares[:2]] == [52, 48]
write("electrical", "possolo-meija-2022-wheatstone-bridge",
      {"model": {"output": {"name": "R_U", "unit": "Ω", "expression": "R_G * R_F * (1/R_E + 1/R_H)",
                            "description": T("Resistance of the unknown resistor U of a balanced Wheatstone bridge", "Dengelenmiş Wheatstone köprüsündeki bilinmeyen U direncinin değeri")},
                 "inputs": [inp("R_E", 951, 5, "Ω", "Decade resistor E (EICO, 0.5 %)", "Dekad direnç E (EICO, %0,5)"),
                            inp("R_F", 997, 5, "Ω", "Fixed resistor F", "Sabit direnç F"),
                            inp("R_G", 909, 0.5, "Ω", "Decade resistor G (General Radio, 0.05 %)", "Dekad direnç G (General Radio, %0,05)"),
                            inp("R_H", 225.2e3, 2.3e3, "Ω", "Resistance substitution box H, in parallel with E", "E'ye paralel direnç ikame kutusu H")]}},
      {"title": T("Wheatstone bridge: a measurement model that is not a product of powers (Possolo & Meija 2022)",
                  "Wheatstone köprüsü: kuvvetler çarpımı olmayan bir ölçüm modeli (Possolo & Meija 2022)"),
       "description": T("At balance R_U/R_G = R_F/R_EH, with R_EH = 1/(R_E⁻¹ + R_H⁻¹) the joint resistance of E and H in parallel, so R_U = R_G R_F (R_E⁻¹ + R_H⁻¹).",
                        "Dengede R_U/R_G = R_F/R_EH; R_EH = 1/(R_E⁻¹ + R_H⁻¹), paralel E ve H'nin ortak direnci; dolayısıyla R_U = R_G R_F (R_E⁻¹ + R_H⁻¹)."),
       "explanation": [T(f"Gauss's formula with the partial derivatives −R_G R_F/R_E², R_U/R_F, R_U/R_G and −R_G R_F/R_H² gives R_U = {RU:.1f} Ω and u(R_U) = {uRU:.2f} Ω ≈ 7 Ω.",
                         f"−R_G R_F/R_E², R_U/R_F, R_U/R_G ve −R_G R_F/R_H² kısmi türevleriyle Gauss formülü R_U = {c(f'{RU:.1f}')} Ω ve u(R_U) = {c(f'{uRU:.2f}')} Ω ≈ 7 Ω verir."),
                       T(f"Contributions to u²(R_U): E {100 * shares[0]:.0f} %, F {100 * shares[1]:.0f} %, G {100 * shares[2]:.1f} %, H {100 * shares[3]:.3f} %, as in the NIST Uncertainty Machine output quoted by the book (E 52 %, F 48 %, G 0.6 %, H 0.004 %).",
                         f"u²(R_U)'ya katkılar: E %{100 * shares[0]:.0f}, F %{100 * shares[1]:.0f}, G %{c(f'{100 * shares[2]:.1f}')}, H %{c(f'{100 * shares[3]:.3f}')}; kitabın aktardığı NIST Uncertainty Machine çıktısıyla aynı (E %52, F %48, G %0,6, H %0,004).")],
       "section": "Sums, Products, and Ratios: Wheatstone Bridge, pp. 24–26",
       "tags": ["possolo-meija-2022", "resistance", "bridge", "nist-uncertainty-machine"], "notes": [
           T("The book notes that a lognormal model, which respects the positivity of resistance, gives practically the same results at these relative uncertainties (below 5 %).",
             "Kitap, direncin pozitifliğine uyan lognormal modelin bu bağıl belirsizliklerde (%5'in altında) pratikte aynı sonucu verdiğini belirtir.")]},
      {"gum": {"estimate": chk(957, 0.5), "standard_uncertainty": chk(7, 0.5)}},
      T("Possolo & Meija (2022), p. 25: R_U = 957 Ω, u(R_U) ≈ 7 Ω; contributions F 48 %, G 0.6 %, E 52 %, H 0.004 %.",
        "Possolo & Meija (2022), s. 25: R_U = 957 Ω, u(R_U) ≈ 7 Ω; katkılar F %48, G %0,6, E %52, H %0,004."))

# --- Atomic weight of lead in NRC HIPB-1 (pp. 26–29) -----------------------------------------------------
Ar = {"204": 203.9730435, "206": 205.9744652, "207": 206.9758968, "208": 207.9766520}
R = {"R4": (0.06300, 0.00003), "R6": (1.3314, 0.0002), "R8": (2.5193, 0.0004)}
S = 1 + R["R4"][0] + R["R6"][0] + R["R8"][0]
ArPb = (Ar["204"] * R["R4"][0] + Ar["206"] * R["R6"][0] + Ar["207"] + Ar["208"] * R["R8"][0]) / S
assert round(ArPb, 4) == 207.1791
den = "(1 + R4 + R6 + R8)"
write("chemistry", "possolo-meija-2022-atomic-weight-lead-hipb1",
      {"multivariate": {"outputs": [
          {"name": "x204", "unit": "mol/mol", "expression": f"R4/{den}"},
          {"name": "x206", "unit": "mol/mol", "expression": f"R6/{den}"},
          {"name": "x207", "unit": "mol/mol", "expression": f"1/{den}"},
          {"name": "x208", "unit": "mol/mol", "expression": f"R8/{den}"},
          {"name": "ArPb", "unit": "1", "expression": f"(A204*R4 + A206*R6 + A207 + A208*R8)/{den}"}],
          "inputs": [inp("R4", 0.06300, 0.00003, "mol/mol", "Isotope ratio n(²⁰⁴Pb)/n(²⁰⁷Pb), MC-ICP-MS (U95 % = 0.000 06, k = 2)", "İzotop oranı n(²⁰⁴Pb)/n(²⁰⁷Pb), MC-ICP-MS (U95 % = 0,000 06, k = 2)"),
                     inp("R6", 1.3314, 0.0002, "mol/mol", "Isotope ratio n(²⁰⁶Pb)/n(²⁰⁷Pb) (U95 % = 0.0004)", "İzotop oranı n(²⁰⁶Pb)/n(²⁰⁷Pb) (U95 % = 0,0004)"),
                     inp("R8", 2.5193, 0.0004, "mol/mol", "Isotope ratio n(²⁰⁸Pb)/n(²⁰⁷Pb) (U95 % = 0.0008)", "İzotop oranı n(²⁰⁸Pb)/n(²⁰⁷Pb) (U95 % = 0,0008)"),
                     inp("A204", Ar["204"], 1.2e-6, "1", "Relative atomic mass of ²⁰⁴Pb (AME 2020)", "²⁰⁴Pb'nin bağıl atom kütlesi (AME 2020)"),
                     inp("A206", Ar["206"], 1.2e-6, "1", "Relative atomic mass of ²⁰⁶Pb (AME 2020)", "²⁰⁶Pb'nin bağıl atom kütlesi (AME 2020)"),
                     inp("A207", Ar["207"], 1.2e-6, "1", "Relative atomic mass of ²⁰⁷Pb (AME 2020)", "²⁰⁷Pb'nin bağıl atom kütlesi (AME 2020)"),
                     inp("A208", Ar["208"], 1.2e-6, "1", "Relative atomic mass of ²⁰⁸Pb (AME 2020)", "²⁰⁸Pb'nin bağıl atom kütlesi (AME 2020)")]}},
      {"title": T("Atomic weight and isotopic composition of lead in NRC HIPB-1 from three isotope ratios (Possolo & Meija 2022)",
                  "Üç izotop oranından NRC HIPB-1'deki kurşunun atom ağırlığı ve izotop bileşimi (Possolo & Meija 2022)"),
       "description": T("Tong et al. (2019) measured ²⁰⁴Pb/²⁰⁷Pb, ²⁰⁶Pb/²⁰⁷Pb and ²⁰⁸Pb/²⁰⁷Pb in the high-purity lead HIPB-1. The amount fractions follow from the ratios and the closure Σx = 1; A_r(Pb) is their weighted average of the isotopic masses.",
                        "Tong vd. (2019) yüksek saflıktaki kurşun HIPB-1'de ²⁰⁴Pb/²⁰⁷Pb, ²⁰⁶Pb/²⁰⁷Pb ve ²⁰⁸Pb/²⁰⁷Pb oranlarını ölçtü. Miktar kesirleri oranlardan ve Σx = 1 kapanışından çıkar; A_r(Pb) izotop kütlelerinin bu kesirlerle ağırlıklı ortalamasıdır."),
       "explanation": [T("Three ratios and the closure determine the four amount fractions exactly, x(²⁰⁴Pb) = R₄/(1 + R₄ + R₆ + R₈) and so on, so the constrained maximum-likelihood fit of the book (R solnp) returns these explicit expressions; the analytic Jacobian replaces the 5 × 10⁴ refits of its Monte Carlo.",
                         "Üç oran ve kapanış dört miktar kesrini tam olarak belirler, x(²⁰⁴Pb) = R₄/(1 + R₄ + R₆ + R₈) vb.; bu nedenle kitabın kısıtlı en çok olabilirlik uyumu (R solnp) bu açık ifadeleri verir; analitik Jakobiyen, kitaptaki Monte Carlo'nun 5 × 10⁴ yeniden uyumunun yerini alır."),
                       T("The closure induces correlations between the amount fractions (Chayes 1971), strongly negative between ²⁰⁶Pb and ²⁰⁸Pb (−0.90).",
                         "Kapanış miktar kesirleri arasında korelasyon doğurur (Chayes 1971); ²⁰⁶Pb ile ²⁰⁸Pb arasında güçlü biçimde negatiftir (−0,90)."),
                       T(f"A_r(Pb) = {ArPb:.5f} with u = 0.000 084 and U95 % = 0.000 17 (printed 0.0002); the uncertainties of the isotopic masses contribute almost nothing.",
                         f"A_r(Pb) = {c(f'{ArPb:.5f}')}, u = 0,000 084 ve U95 % = 0,000 17 (basılı 0,0002); izotop kütlelerinin belirsizlikleri neredeyse hiç katkı vermez.")],
       "section": "Sums, Products, and Ratios: Atomic Weight of Lead, pp. 26–29",
       "tags": ["possolo-meija-2022", "isotope-ratios", "atomic-weight", "closure", "correlation", "multivariate"],
       "notes": [T("The book's correlations come from 5 × 10⁴ Monte Carlo refits; two of them (²⁰⁴Pb–²⁰⁶Pb −0.02 and ²⁰⁴Pb–²⁰⁸Pb −0.17) differ from the linearised values (−0.011, −0.179) by about two Monte Carlo standard errors (research log F96).",
                   "Kitaptaki korelasyonlar 5 × 10⁴ Monte Carlo yeniden uyumundan gelir; ikisi (²⁰⁴Pb–²⁰⁶Pb −0,02 ve ²⁰⁴Pb–²⁰⁸Pb −0,17) doğrusallaştırılmış değerlerden (−0,011, −0,179) yaklaşık iki Monte Carlo standart hatası kadar farklıdır (araştırma günlüğü F96)."),
                 T("Users of HIPB-1 should rely on the certificate (Meija et al. 2020); the book's data reduction differs slightly from it.",
                   "HIPB-1 kullanıcıları sertifikaya (Meija vd. 2020) dayanmalıdır; kitabın veri indirgemesi ondan biraz farklıdır.")]},
      {"multivariate": {"gum": {
          "estimate": {"ArPb": chk(207.1791, 0.00005)},
          "standard_uncertainty": {"x204": chk(0.000006, 0.0000005), "x206": chk(0.000037, 0.0000005),
                                   "x207": chk(0.000019, 0.0000005), "x208": chk(0.000045, 0.0000005), "ArPb": chk(0.0001, 0.00002)},
          "correlations": [{"between": ["x204", "x206"], "value": -0.02, "tolerance": 0.015},
                           {"between": ["x204", "x207"], "value": 0.13, "tolerance": 0.01},
                           {"between": ["x206", "x207"], "value": 0.18, "tolerance": 0.01},
                           {"between": ["x204", "x208"], "value": -0.17, "tolerance": 0.015},
                           {"between": ["x206", "x208"], "value": -0.90, "tolerance": 0.01},
                           {"between": ["x207", "x208"], "value": -0.58, "tolerance": 0.01}]}}},
      T("Possolo & Meija (2022), p. 29: u(x) = 0.000 006, 0.000 037, 0.000 019, 0.000 045 mol/mol; correlations −0.02, +0.13, +0.18, −0.17, −0.90, −0.58 (Monte Carlo, 5 × 10⁴); A_r(Pb) = 207.1791, U95 % = 0.0002 (u = U/2; tolerance of the correlations: Monte Carlo error).",
        "Possolo & Meija (2022), s. 29: u(x) = 0,000 006, 0,000 037, 0,000 019, 0,000 045 mol/mol; korelasyonlar −0,02, +0,13, +0,18, −0,17, −0,90, −0,58 (Monte Carlo, 5 × 10⁴); A_r(Pb) = 207,1791, U95 % = 0,0002 (u = U/2; korelasyon toleransı: Monte Carlo hatası)."),
      analysis={"gum": {}, "mcm": {"trials": 200000, "seed": 1}})
