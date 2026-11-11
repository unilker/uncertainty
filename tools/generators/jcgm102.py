"""Writes the JCGM 102:2011 library projects (run from the repository root)."""
import json, math
from pathlib import Path

DOC = "JCGM 102:2011, Evaluation of measurement data — Supplement 2 to the “Guide to the expression of uncertainty in measurement” — Extension to any number of output quantities"
URL = "https://www.bipm.org/documents/20126/2071204/JCGM_102_2011_E.pdf"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def normal(m, s): return {"type": "normal", "mean": m, "sd": s}

def project(folder, name, title, desc, expl, sec, tags, notes, body, analysis, expected, ref):
    pid = f"jcgm102-{name}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": folder,
                      "sources": [{"document": DOC, "section": sec, "url": URL}], "tags": ["jcgm102", "multivariate"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    if name in EXTRA_SOURCES: p["metadata"]["sources"].append(EXTRA_SOURCES[name])
    p["multivariate"] = body
    p["analysis"] = analysis
    p["expected"] = {"multivariate": expected, "reference": ref}
    Path(f"library/{folder}/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

# The same examples are reworked in GUM-5 (Example 13); the second source is kept with the project.
GUM5 = "JCGM GUM-5:2026, Guide to the expression of uncertainty in measurement — Part 5: Examples"
EXTRA_SOURCES = {
    "9.5.2-resistance-thermometer": {"document": GUM5, "section": "Example 13, 13.2", "url": "https://doi.org/10.59161/YNLY8209"},
    "9.5.3-ten-temperatures": {"document": GUM5, "section": "Example 13, 13.3, Tables 13.4 and 13.5",
                               "url": "https://doi.org/10.59161/YNLY8209"},
}

def gum_mcm(trials=1_000_000, seed=1):
    return {"gum": {"coverage_probability": 0.95}, "mcm": {"trials": trials, "seed": seed, "validation_ndig": 2}}

# ------------------------------------------------------------------ 9.2 additive model
add_outputs = [{"name": "Y1", "expression": "X1 + X3"}, {"name": "Y2", "expression": "X2 + X3"}]
def additive(x3, sd3):
    return {"outputs": add_outputs, "inputs": [
        {"name": "X1", "distribution": normal(0, 1)}, {"name": "X2", "distribution": normal(0, 1)},
        {"name": "X3", "distribution": x3}]}
add_expl = [
    T("Bivariate additive model Y₁ = X₁ + X₃, Y₂ = X₂ + X₃ with independent inputs; X₃ is an effect common to both outputs (9.2.1).",
      "İki değişkenli toplamalı model Y₁ = X₁ + X₃, Y₂ = X₂ + X₃, bağımsız girdilerle; X₃ her iki çıktıda ortak olan bir etkidir (9.2.1)."),
    T("GUM framework: U_y = C_x U_x C_xᵀ with C_x = [[1, 0, 1], [0, 1, 1]] (formula 3); 95 % regions for N(y, U_y): ellipse with k_p = 2.45 (χ²₂) and rectangle with k_q = 2.24 (q = 0.975, Tables 1 and 2).",
      "GUM yaklaşımı: C_x = [[1, 0, 1], [0, 1, 1]] ile U_y = C_x U_x C_xᵀ (formül 3); N(y, U_y) için %95 bölgeler: k_p = 2,45 (χ²₂) ile elips ve k_q = 2,24 (q = 0,975) ile dikdörtgen (Tablo 1 ve 2)."),
    T("Monte Carlo method with 10⁶ trials: k_p from the draws sorted by their distance (y_r − y)ᵀ U_y⁻¹ (y_r − y) (7.7.2) and k_q from max_j |y_j,r − y_j|/u(y_j) (7.7.3); validation as in clause 8 with two significant digits.",
      "10⁶ denemeli Monte Carlo yöntemi: k_p, (y_r − y)ᵀ U_y⁻¹ (y_r − y) uzaklığına göre sıralanan çekilişlerden (7.7.2), k_q ise max_j |y_j,r − y_j|/u(y_j) ile (7.7.3); doğrulama Madde 8'deki gibi iki anlamlı basamakla."),
]
def add_expected(u, r, kp_mc, kq_mc, ok_e, ok_r):
    g = {"estimate": {"Y1": chk(0, 1e-12), "Y2": chk(0, 1e-12)}, "standard_uncertainty": {"Y1": chk(u, 0.0005), "Y2": chk(u, 0.0005)},
         "correlations": [{"between": ["Y1", "Y2"], "value": r, "tolerance": 0.0005}], "k_ellipsoid": chk(2.45, 0.005), "k_rectangle": chk(2.24, 0.005)}
    m = {"estimate": {"Y1": chk(0, 0.005 * u), "Y2": chk(0, 0.005 * u)}, "standard_uncertainty": {"Y1": chk(u, 0.0015 * u), "Y2": chk(u, 0.0015 * u)},
         "correlations": [{"between": ["Y1", "Y2"], "value": r, "tolerance": 0.003}], "k_ellipsoid": chk(kp_mc, 0.01), "k_rectangle": chk(kq_mc, 0.01)}
    return {"gum": g, "mcm": m, "ellipsoid_validated": ok_e, "rectangle_validated": ok_r}
r3 = math.sqrt(3)
project("general", "9.2.2-additive-gaussian",
  T("Additive bivariate model, Gaussian inputs (JCGM 102 9.2.2)", "Toplamalı iki değişkenli model, Gauss girdiler (JCGM 102 9.2.2)"),
  T("JCGM 102:2011 9.2.2: two outputs sharing a common Gaussian effect; the GUM results are exact and validated by the Monte Carlo method.",
    "JCGM 102:2011 9.2.2: ortak bir Gauss etkisini paylaşan iki çıktı; GUM sonuçları kesindir ve Monte Carlo yöntemiyle doğrulanır."),
  add_expl, "9.2.2 (Table 3)", ["coverage-region"], [],
  additive(normal(0, 1), 1), gum_mcm(seed=1), add_expected(1.414, 0.5, 2.45, 2.21, True, True),
  T("JCGM 102:2011 Table 3 (GUF and MCM with M = 10⁶).", "JCGM 102:2011 Tablo 3 (GUF ve M = 10⁶ ile MCM)."))
project("general", "9.2.3-additive-rectangular",
  T("Additive bivariate model, rectangular common effect (JCGM 102 9.2.3)", "Toplamalı iki değişkenli model, dikdörtgen ortak etki (JCGM 102 9.2.3)"),
  T("JCGM 102:2011 9.2.3: as 9.2.2 with the common effect X₃ rectangular (standard deviation 1); the GUM coverage factors are not validated.",
    "JCGM 102:2011 9.2.3: ortak etki X₃ dikdörtgen (standart sapma 1) olmak üzere 9.2.2 ile aynı; GUM kapsam faktörleri doğrulanmaz."),
  add_expl + [T("The estimates, standard uncertainties and the correlation coefficient agree, but k_p (2.38 against 2.45) and k_q (2.15 against 2.24) differ by more than the tolerance 0.05 (9.2.3.4).",
                "Tahminler, standart belirsizlikler ve korelasyon katsayısı uyuşur; ancak k_p (2,45'e karşı 2,38) ve k_q (2,24'e karşı 2,15) 0,05 toleransından fazla farklıdır (9.2.3.4).")],
  "9.2.3 (Table 4)", ["coverage-region"], [],
  additive({"type": "rectangular", "lower": -r3, "upper": r3}, 1), gum_mcm(seed=2), add_expected(1.414, 0.5, 2.38, 2.15, False, False),
  T("JCGM 102:2011 Table 4 (GUF and MCM with M = 10⁶).", "JCGM 102:2011 Tablo 4 (GUF ve M = 10⁶ ile MCM)."))
project("general", "9.2.4-additive-dominant-rectangular",
  T("Additive bivariate model, dominant rectangular common effect (JCGM 102 9.2.4)", "Toplamalı iki değişkenli model, baskın dikdörtgen ortak etki (JCGM 102 9.2.4)"),
  T("JCGM 102:2011 9.2.4: the common rectangular effect has standard deviation 3; r(y₁, y₂) = 0.9 and the GUM k_p and k_q are about 7 % and 20 % too large.",
    "JCGM 102:2011 9.2.4: ortak dikdörtgen etkinin standart sapması 3'tür; r(y₁, y₂) = 0,9 ve GUM k_p ile k_q değerleri yaklaşık %7 ve %20 büyüktür."),
  add_expl, "9.2.4 (Table 5)", ["coverage-region"], [],
  additive({"type": "rectangular", "lower": -3 * r3, "upper": 3 * r3}, 3), gum_mcm(seed=3), add_expected(3.162, 0.9, 2.28, 1.87, False, False),
  T("JCGM 102:2011 Table 5 (GUF and MCM with M = 10⁶).", "JCGM 102:2011 Tablo 5 (GUF ve M = 10⁶ ile MCM)."))

# ------------------------------------------------------------------ 9.3 co-ordinate transformation
polar_rows = {  # x1, r: GUF (y1, y2, u1, u2, r), MCM (y1, y2, u1, u2, r)
  (0.001, 0.0): ((0.001, 0.0, 0.010, 10.000, 0.0), (0.013, -0.001, 0.007, 1.744, 0.000)),
  (0.010, 0.0): ((0.010, 0.0, 0.010, 1.000, 0.0), (0.015, 0.000, 0.008, 1.117, 0.000)),
  (0.100, 0.0): ((0.100, 0.0, 0.010, 0.100, 0.0), (0.101, 0.000, 0.010, 0.101, 0.000)),
  (0.001, 0.9): ((0.001, 0.0, 0.010, 10.000, 0.9), (0.012, -0.556, 0.008, 1.599, -0.070)),
  (0.010, 0.9): ((0.010, 0.0, 0.010, 1.000, 0.9), (0.015, -0.343, 0.008, 0.903, 0.352)),
  (0.100, 0.9): ((0.100, 0.0, 0.010, 0.100, 0.9), (0.101, -0.009, 0.010, 0.102, 0.882)),
}
for (x1, r), (g, m) in polar_rows.items():
    corr = r != 0.0
    tag = f"{x1:.3f}".replace("0.", "x") + ("-correlated" if corr else "")
    body = {"outputs": [{"name": "R", "expression": "sqrt(X1^2 + X2^2)"}, {"name": "Theta", "unit": "rad", "expression": "atan2(X2, X1)"}],
            "inputs": [{"name": "X1", "distribution": normal(x1, 0.010)}, {"name": "X2", "distribution": normal(0.0, 0.010)}]}
    if corr: body["correlations"] = [{"a": "X1", "b": "X2", "r": r}]
    names = ["R", "Theta"]
    ge = {"estimate": {"R": chk(g[0], 1e-9), "Theta": chk(0.0, 1e-12)}, "standard_uncertainty": {"R": chk(g[2], 0.0005), "Theta": chk(g[3], 0.0005)},
          "correlations": [{"between": names, "value": g[4], "tolerance": 0.0005}]}
    me = {"estimate": {"R": chk(m[0], 0.0006), "Theta": chk(m[1], 0.006)}, "standard_uncertainty": {"R": chk(m[2], 0.0006), "Theta": chk(m[3], 0.004)},
          "correlations": [{"between": names, "value": m[4], "tolerance": 0.004}]}
    expl = [
        T(f"Cartesian to polar co-ordinates of a complex quantity Z = X₁ + iX₂: R = √(X₁² + X₂²), Θ = atan2(X₂, X₁) (expressions 25), with x₁ = {x1}, x₂ = 0, u(x₁) = u(x₂) = 0.010{' and r(x₁, x₂) = 0.9' if corr else ' and independent inputs'}.",
          f"Karmaşık bir Z = X₁ + iX₂ büyüklüğünün Kartezyen koordinatlardan kutupsal koordinatlara dönüşümü: R = √(X₁² + X₂²), Θ = atan2(X₂, X₁) (ifade 25); x₁ = {str(x1).replace('.', ',')}, x₂ = 0, u(x₁) = u(x₂) = 0,010{' ve r(x₁, x₂) = 0,9' if corr else ' ve bağımsız girdiler'}."),
        T("GUM framework: the sensitivity matrix at (x₁, 0) is [[1, 0], [0, 1/x₁]], so u(θ) = u(x₂)/x₁; this linearization fails when x₁ is comparable to u(x₁).",
          "GUM yaklaşımı: (x₁, 0) noktasında duyarlılık matrisi [[1, 0], [0, 1/x₁]] olduğundan u(θ) = u(x₂)/x₁'dir; x₁, u(x₁) ile karşılaştırılabilir olduğunda bu doğrusallaştırma geçersizdir."),
        T("Monte Carlo method with 10⁶ trials (the Supplement uses 10⁷): the distribution of Θ spreads over (−π, π] and, with correlated inputs near the origin, becomes bimodal (9.3.3.3); ellipses and rectangles are then inappropriate coverage regions (9.3.4.4).",
          "10⁶ denemeli Monte Carlo yöntemi (Ek 10⁷ kullanır): Θ'nın dağılımı (−π, π] aralığına yayılır ve başlangıç noktasının yakınında korelasyonlu girdilerle iki tepeli olur (9.3.3.3); bu durumda elips ve dikdörtgen uygun kapsam bölgeleri değildir (9.3.4.4)."),
    ]
    project("electrical", f"9.3-polar-{tag}",
      T(f"Cartesian to polar co-ordinates, x₁ = {x1}{', r = 0.9' if corr else ''} (JCGM 102 9.3)", f"Kartezyen koordinatlardan kutupsal koordinatlara, x₁ = {str(x1).replace('.', ',')}{', r = 0,9' if corr else ''} (JCGM 102 9.3)"),
      T(f"JCGM 102:2011 9.3: magnitude and phase of a complex quantity estimated at x₁ = {x1}, x₂ = 0; GUM and Monte Carlo results compared ({'Table 7' if corr else 'Table 6'}).",
        f"JCGM 102:2011 9.3: x₁ = {str(x1).replace('.', ',')}, x₂ = 0 tahminli karmaşık bir büyüklüğün genliği ve fazı; GUM ve Monte Carlo sonuçlarının karşılaştırılması ({'Tablo 7' if corr else 'Tablo 6'})."),
      expl, f"9.3 ({'Table 7' if corr else 'Table 6'})", ["complex-quantity", "nonlinear"], [],
      body, gum_mcm(seed=4), {"gum": ge, "mcm": me},
      T(f"JCGM 102:2011 {'Table 7' if corr else 'Table 6'}, x₁ = {x1} (MCM with M = 10⁷; tolerances include the Monte Carlo dispersion for 10⁶ trials).",
        f"JCGM 102:2011 {'Tablo 7' if corr else 'Tablo 6'}, x₁ = {str(x1).replace('.', ',')} (M = 10⁷ ile MCM; toleranslar 10⁶ deneme için Monte Carlo saçılımını içerir)."))

# ------------------------------------------------------------------ 9.4 resistance and reactance
rows = [[5.007, 19.663, 1.0456], [4.994, 19.639, 1.0438], [5.005, 19.640, 1.0468], [4.990, 19.685, 1.0428], [4.999, 19.678, 1.0433], [4.999, 19.661, 1.04446]]
imp_out = [{"name": "R", "unit": "Ω", "expression": "V/(I*1e-3)*cos(phi)"}, {"name": "X", "unit": "Ω", "expression": "V/(I*1e-3)*sin(phi)"}, {"name": "Z", "unit": "Ω", "expression": "V/(I*1e-3)"}]
imp_expl = [
    T("Ohm's law for an alternating current: R = (V/I) cos φ, X = (V/I) sin φ, Z = V/I (model 26), with n = 6 sets of simultaneous indication values of V, I and φ (Table 8; set 6 is the average of sets 1 to 5).",
      "Alternatif akım için Ohm yasası: R = (V/I) cos φ, X = (V/I) sin φ, Z = V/I (model 26); V, I ve φ'nin n = 6 eşzamanlı gösterge değeri takımıyla (Tablo 8; 6. takım 1–5. takımların ortalamasıdır)."),
    T("The estimates are the averages; the matrix of sums of squares and products M gives the covariance matrix of the averages and the correlation coefficients of Table 10.",
      "Tahminler ortalamalardır; kareler ve çarpımlar toplamı matrisi M, ortalamaların kovaryans matrisini ve Tablo 10'daki korelasyon katsayılarını verir."),
    T("Monte Carlo method: X is drawn from the multivariate t-distribution t_ν(x̄, S/n), ν = n − N = 3, S = M/ν (5.3.2.4). Because Z² = R² + X², U_y is singular in theory (9.4.2.3 note 2).",
      "Monte Carlo yöntemi: X, çok değişkenli t dağılımından t_ν(x̄, S/n), ν = n − N = 3, S = M/ν olarak çekilir (5.3.2.4). Z² = R² + X² olduğundan U_y teoride tekildir (9.4.2.3 not 2)."),
]
imp_note = [T("With ν = 3 the multivariate t-distribution has no fourth moment, so the Monte Carlo standard deviations converge slowly: with 10⁶ trials, seeds 1 to 6 give u(R) between 0.128 Ω and 0.136 Ω and r(R, X) between −0.580 and −0.600. The tolerances of the Monte Carlo values cover this spread; the printed MCM row is one such realization.",
              "ν = 3 olduğunda çok değişkenli t dağılımının dördüncü momenti yoktur; bu yüzden Monte Carlo standart sapmaları yavaş yakınsar: 10⁶ denemeyle 1–6 tohumları u(R) için 0,128 Ω ile 0,136 Ω, r(R, X) için −0,580 ile −0,600 arasında değerler verir. Monte Carlo değerlerinin toleransları bu yayılımı kapsar; basılı MCM satırı bu tür bir gerçekleşmedir."),
            T("Table 8 prints set 6 as φ = 1.0445 rad, the rounded average of sets 1 to 5 (1.04446). With the rounded value the estimate of R is 127.7307 Ω instead of the printed 127.732 Ω; the project uses 1.04446 rad, the value of Table 9 (research log F51).",
              "Tablo 8, 6. takımı φ = 1,0445 rad olarak, yani 1–5. takımların yuvarlanmış ortalaması (1,04446) olarak basar. Yuvarlanmış değerle R'nin tahmini basılı 127,732 Ω yerine 127,7307 Ω olur; proje Tablo 9'daki değer olan 1,04446 rad'ı kullanır (araştırma günlüğü F51).")]
mcm_imp = {"estimate": {"R": chk(127.732, 0.002), "X": chk(219.847, 0.004), "Z": chk(254.260, 0.004)},
           "standard_uncertainty": {"R": chk(0.130, 0.007), "X": chk(0.536, 0.012), "Z": chk(0.429, 0.008)},
           "correlations": [{"between": ["R", "X"], "value": -0.587, "tolerance": 0.035}, {"between": ["R", "Z"], "value": -0.482, "tolerance": 0.045}]}
for kind, name, row, us in (("mean", "", "GUF", (0.058, 0.241, 0.193)), ("t", "-alternative", "Alt. GUF", (0.130, 0.540, 0.431))):
    ge = {"estimate": {"R": chk(127.732, 0.0005), "X": chk(219.847, 0.0005), "Z": chk(254.260, 0.0005)},
          "standard_uncertainty": {"R": chk(us[0], 0.0005), "X": chk(us[1], 0.0005), "Z": chk(us[2], 0.0005)},
          "correlations": [{"between": ["R", "X"], "value": -0.588, "tolerance": 0.0005}, {"between": ["R", "Z"], "value": -0.485, "tolerance": 0.0005},
                           {"between": ["X", "Z"], "value": 1 - 0.749e-2, "tolerance": 0.000005}]}
    project("electrical", f"9.4-resistance-reactance{name}",
      T(f"Resistance, reactance and impedance from simultaneous indication values{', t covariance' if kind == 't' else ''} (JCGM 102 9.4)",
        f"Eşzamanlı gösterge değerlerinden direnç, reaktans ve empedans{', t kovaryansı' if kind == 't' else ''} (JCGM 102 9.4)"),
      T(f"JCGM 102:2011 9.4 (GUM H.2 extended): three output quantities from correlated averages; GUM framework with {'U_x = M/(n(n − 1))' if kind == 'mean' else 'the covariance matrix of the multivariate t-distribution'} and Monte Carlo method with the multivariate t-distribution.",
        f"JCGM 102:2011 9.4 (GUM H.2'nin genişletilmesi): korelasyonlu ortalamalardan üç çıktı büyüklüğü; {'U_x = M/(n(n − 1))' if kind == 'mean' else 'çok değişkenli t dağılımının kovaryans matrisi'} ile GUM yaklaşımı ve çok değişkenli t dağılımıyla Monte Carlo yöntemi."),
      imp_expl + ([T("The alternative GUM evaluation uses V(X) = M/((ν − 2)n), which agrees with the Monte Carlo results (9.4.2.5–9.4.2.7).",
                     "Alternatif GUM değerlendirmesi V(X) = M/((ν − 2)n) kullanır; bu Monte Carlo sonuçlarıyla uyuşur (9.4.2.5–9.4.2.7).")] if kind == "t" else []),
      "9.4 (Tables 8–11)", ["impedance", "multivariate-t", "indication-values"], imp_note,
      {"outputs": imp_out, "indications": [{"inputs": ["V", "I", "phi"], "units": ["V", "mA", "rad"], "values": rows, "covariance": kind}]},
      gum_mcm(seed=1), {"gum": ge, "mcm": mcm_imp},
      T(f"JCGM 102:2011 Table 11, rows {row} and MCM (M = 10⁶).", f"JCGM 102:2011 Tablo 11, {row} ve MCM satırları (M = 10⁶)."))

# ------------------------------------------------------------------ 9.5 resistance thermometer
th_inputs = [{"name": "R0", "unit": "Ω", "distribution": normal(99.99610, 0.00050)},
             {"name": "A", "unit": "°C⁻¹", "distribution": normal(0.0039096, 0.0000027)},
             {"name": "B", "unit": "°C⁻²", "distribution": normal(-6.0e-7, 1.1e-7)},
             {"name": "RS", "unit": "Ω", "distribution": normal(99.99947, 0.00010)}]
th_corr = [{"a": "R0", "b": "A", "r": -0.155}, {"a": "R0", "b": "B", "r": 0.092}, {"a": "A", "b": "B", "r": -0.959}]
th_expl = [
    T("Implicit model h(θ, X) = R₀(1 + Aθ + Bθ²) − r R_S = 0 (31): the thermometer resistance R(θ) compared with a standard resistor R_S by a bridge measuring r = R(θ)/R_S.",
      "Örtük model h(θ, X) = R₀(1 + Aθ + Bθ²) − r R_S = 0 (31): termometre direnci R(θ), r = R(θ)/R_S oranını ölçen bir köprüyle standart direnç R_S ile karşılaştırılır."),
    T("The calibration constants R₀, A and B are correlated (Table 13); θ is found by Newton's method and u(θ) from C_y U_y C_yᵀ = C_x U_x C_xᵀ with C_y = (A + 2Bθ)R₀ (6.3.1.3, 9.5.2.6).",
      "Kalibrasyon sabitleri R₀, A ve B koreledir (Tablo 13); θ Newton yöntemiyle, u(θ) ise C_y = (A + 2Bθ)R₀ ile C_y U_y C_yᵀ = C_x U_x C_xᵀ bağıntısından bulunur (6.3.1.3, 9.5.2.6)."),
]
project("temperature", "9.5.2-resistance-thermometer",
  T("Celsius temperature from a resistance thermometer, implicit model (JCGM 102 9.5.2)", "Direnç termometresiyle Celsius sıcaklığı, örtük model (JCGM 102 9.5.2)"),
  T("JCGM 102:2011 9.5.2: one temperature from a measured resistance ratio, with the correlated calibration constants of the thermometer.",
    "JCGM 102:2011 9.5.2: termometrenin korelasyonlu kalibrasyon sabitleriyle ölçülen bir direnç oranından tek bir sıcaklık."),
  th_expl, "9.5.2 (Tables 12, 13)", ["implicit-model", "resistance-thermometer"], [],
  {"implicit": True, "outputs": [{"name": "theta", "unit": "°C", "expression": "R0*(1 + A*theta + B*theta^2) - r*RS", "start": 20}],
   "inputs": th_inputs + [{"name": "r", "distribution": normal(1.0780057, 0.0000050)}], "correlations": th_corr},
  {"gum": {"coverage_probability": 0.95}, "mcm": None},
  {"gum": {"estimate": {"theta": chk(20.0232, 0.00005)}, "standard_uncertainty": {"theta": chk(0.0045, 0.00005)}}},
  T("JCGM 102:2011 9.5.2.5 and 9.5.2.6 (θ = 20.023 2 °C, u = 0.004 5 °C).", "JCGM 102:2011 9.5.2.5 ve 9.5.2.6 (θ = 20,023 2 °C, u = 0,004 5 °C)."))
ratios = [53, 150054, 300055, 450056, 600056, 780057, 900058, 1050059, 1200060, 780057]
temps = [0.0100, 3.8491, 7.6928, 11.5410, 15.3938, 20.0232, 23.1131, 26.9797, 30.8509, 20.0232]
uncs = [0.0018, 0.0027, 0.0040, 0.0046, 0.0047, 0.0045, 0.0046, 0.0060, 0.0089, 0.0045]
r10 = [0.054, 0.580, 0.691, 0.766, 0.847, 0.918, 0.841, 0.549, 0.264]
outs = [{"name": f"theta{j+1}", "unit": "°C", "expression": f"R0*(1 + A*theta{j+1} + B*theta{j+1}^2) - r{j+1}*RS", "start": 20} for j in range(10)]
ins = th_inputs + [{"name": f"r{j+1}", "distribution": normal(1 + ratios[j] * 1e-7, 50e-7)} for j in range(10)]
project("temperature", "9.5.3-ten-temperatures",
  T("Ten Celsius temperatures with one resistance thermometer, implicit model (JCGM 102 9.5.3)", "Tek direnç termometresiyle on Celsius sıcaklığı, örtük model (JCGM 102 9.5.3)"),
  T("JCGM 102:2011 9.5.3: ten temperatures measured with the same calibrated thermometer are correlated through its calibration constants (Tables 15 and 16).",
    "JCGM 102:2011 9.5.3: aynı kalibre termometreyle ölçülen on sıcaklık, kalibrasyon sabitleri üzerinden koreledir (Tablo 15 ve 16)."),
  th_expl + [T("The ten equations (32) share R₀, A, B and R_S; C_y is diagonal and C_x = [C_x⁽¹⁾ C_x⁽²⁾] (9.5.3.5). The uncertainty is smallest near 0 °C and grows above 25 °C, outside the calibration points (9.5.3.6).",
               "On denklem (32) R₀, A, B ve R_S'yi paylaşır; C_y köşegendir ve C_x = [C_x⁽¹⁾ C_x⁽²⁾] (9.5.3.5). Belirsizlik 0 °C yakınında en küçüktür ve kalibrasyon noktalarının dışında, 25 °C'nin üzerinde artar (9.5.3.6).")],
  "9.5.3 (Tables 14–16)", ["implicit-model", "resistance-thermometer", "correlated-outputs"],
  [T("Table 15 labels the seventh column θ6 instead of θ7.", "Tablo 15 yedinci sütunu θ7 yerine θ6 olarak adlandırır.")],
  {"implicit": True, "outputs": outs, "inputs": ins, "correlations": th_corr},
  {"gum": {"coverage_probability": 0.95}, "mcm": None},
  {"gum": {"estimate": {f"theta{j+1}": chk(temps[j], 0.00005) for j in range(10)},
           "standard_uncertainty": {f"theta{j+1}": chk(uncs[j], 0.00005) for j in range(10)},
           "correlations": [{"between": [f"theta{j+1}", "theta10"], "value": r10[j], "tolerance": 0.0005} for j in range(9)]
                           + [{"between": ["theta2", "theta3"], "value": 0.815, "tolerance": 0.0005}, {"between": ["theta2", "theta9"], "value": -0.358, "tolerance": 0.0005}]}},
  T("JCGM 102:2011 Tables 15 and 16.", "JCGM 102:2011 Tablo 15 ve 16."))
