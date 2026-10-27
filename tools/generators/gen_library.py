"""Writes library/<field>/<id>.json from the validated examples.

One-off authoring helper; the JSON files are the maintained source.
"""
import json, math, os, sys

ROOT = sys.argv[1]
SQ3 = math.sqrt(3)

JCGM100 = {"document": "JCGM 100:2008, Evaluation of measurement data — Guide to the expression of uncertainty in measurement (GUM)",
           "url": "https://www.bipm.org/documents/20126/2071204/JCGM_100_2008_E.pdf"}
JCGM101 = {"document": "JCGM 101:2008, Evaluation of measurement data — Supplement 1 to the GUM — Propagation of distributions using a Monte Carlo method",
           "url": "https://www.bipm.org/documents/20126/2071204/JCGM_101_2008_E.pdf"}
QUAM = {"document": "EURACHEM/CITAC Guide CG 4, Quantifying Uncertainty in Analytical Measurement, 3rd ed. (QUAM:2012.P1)",
        "url": "https://www.eurachem.org/images/stories/Guides/pdf/QUAM2012_P1.pdf"}
EA = {"document": "EA-4/02 M:2022, Evaluation of the Uncertainty of Measurement in Calibration",
      "url": "https://european-accreditation.org/wp-content/uploads/2018/10/EA-4-02.pdf"}


def src(base, section):
    d = dict(base)
    d["section"] = section
    return {"document": d["document"], "section": section, "url": d["url"]}


def T(en, tr):
    return {"en": en, "tr": tr}


def N(mean, sd):
    return {"type": "normal", "mean": mean, "sd": sd}


def NU(mean, U, k):
    return {"type": "normal", "mean": mean, "expanded": U, "k": k}


def R(value, hw):
    return {"type": "rectangular", "value": value, "half_width": hw}


def RL(lo, hi):
    return {"type": "rectangular", "lower": lo, "upper": hi}


def TRI(value, hw):
    return {"type": "triangular", "value": value, "half_width": hw}


def ARC(value, hw):
    return {"type": "arcsine", "value": value, "half_width": hw}


def C(v):
    return {"type": "constant", "value": v}


def TD(mean, scale, dof):
    return {"type": "t", "mean": mean, "scale": scale, "dof": dof}


def TA(obs):
    return {"type": "type_a", "observations": obs}


def inp(name, dist, unit=None, desc=None, dof=None):
    d = {"name": name}
    if unit:
        d["unit"] = unit
    if desc:
        d["description"] = T(*desc)
    d["distribution"] = dist
    if dof is not None:
        d["dof"] = dof
    return d


def mid(name, expr, unit=None, desc=None):
    d = {"name": name, "expression": expr}
    if unit:
        d["unit"] = unit
    if desc:
        d["description"] = T(*desc)
    return d


def chk(v, tol):
    return {"value": v, "tolerance": tol}


def project(id, field, title, desc, explanation, sources, output, inputs, *, intermediates=(), correlations=(),
            gum=None, mcm=None, reporting=None, expected=None, notes=(), tags=()):
    meta = {"id": id, "title": T(*title), "description": T(*desc),
            "explanation": [T(*e) for e in explanation], "field": field, "sources": sources}
    if tags:
        meta["tags"] = list(tags)
    if notes:
        meta["notes"] = [T(*n) for n in notes]
    model = {"output": output}
    if intermediates:
        model["intermediates"] = list(intermediates)
    model["inputs"] = list(inputs)
    if correlations:
        model["correlations"] = [{"a": a, "b": b, "r": r} for a, b, r in correlations]
    p = {"format": "uncertainty-calculations/project", "version": 1, "metadata": meta, "model": model,
         "analysis": {"gum": gum if gum is not None else {}, "mcm": mcm if mcm is not None else {"trials": 1000000}}}
    if reporting:
        p["reporting"] = reporting
    if expected:
        p["expected"] = expected
    path = os.path.join(ROOT, field, id + ".json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(p, f, ensure_ascii=False, indent=2)
        f.write("\n")


def out(name, expr, unit=None, desc=None):
    d = {"name": name}
    if unit:
        d["unit"] = unit
    d["expression"] = expr
    if desc:
        d["description"] = T(*desc)
    return d


K2 = {"coverage_factor": 2}

# ---------------------------------------------------------------- JCGM 101 §9.2
ADD_EXPL = [
    ("The model Y = X1 + X2 + X3 + X4 is linear with unit sensitivity coefficients, so the GUM law of propagation "
     "gives u(y)² = Σ u(xᵢ)² exactly.",
     "Y = X1 + X2 + X3 + X4 modeli doğrusaldır ve tüm duyarlılık katsayıları 1'dir; bu nedenle GUM belirsizlik "
     "yayılım yasası u(y)² = Σ u(xᵢ)² sonucunu tam olarak verir."),
]
for (sid, title, desc, dists, interval, expl, exp, notes, ndig, seed) in [
    ("jcgm101-9.2.2-additive-gaussian",
     ("Additive model with Gaussian inputs", "Normal dağılımlı girdilerle toplamsal model"),
     ("JCGM 101 9.2.2: four independent inputs, each N(0, 1).",
      "JCGM 101 9.2.2: her biri N(0, 1) olan dört bağımsız girdi."),
     [N(0.0, 1.0)] * 4, "symmetric",
     [("The output is exactly Gaussian, N(0, 2²); the GUM 95 % interval ±1.96 × 2 = ±3.92 is exact and the Monte "
       "Carlo method reproduces it, so the GUM result is validated.",
       "Çıktı tam olarak normal dağılımlıdır, N(0, 2²); GUM %95 aralığı ±1,96 × 2 = ±3,92 tamdır, Monte Carlo yöntemi "
       "bunu yeniden üretir ve GUM sonucu doğrulanır.")],
     {"gum": {"estimate": chk(0.0, 1e-12), "standard_uncertainty": chk(2.0, 1e-12)},
      "mcm": {"estimate": chk(0.0, 0.01), "standard_uncertainty": chk(2.0, 0.005), "interval_low": chk(-3.92, 0.02),
              "interval_high": chk(3.92, 0.02), "gum_validated": True},
      "reference": "JCGM 101:2008 Table 2 (exact: N(0, 4)); MCM tolerances ≈ 5 × Monte Carlo dispersion at M = 10⁶."},
     [], 2, 1),
    ("jcgm101-9.2.3-additive-rectangular",
     ("Additive model with rectangular inputs", "Dikdörtgen dağılımlı girdilerle toplamsal model"),
     ("JCGM 101 9.2.3: four independent inputs, each rectangular with standard deviation 1.",
      "JCGM 101 9.2.3: her biri standart sapması 1 olan dikdörtgen dağılımlı dört bağımsız girdi."),
     [R(0.0, SQ3)] * 4, "symmetric",
     [("The output PDF is a convolution of four rectangular PDFs: its 95 % endpoints are ±2√3[2 − (3/5)^{1/4}] = ±3.88 "
       "(JCGM 101 Annex E), slightly inside the GUM interval ±3.92.",
       "Çıktı olasılık yoğunluk fonksiyonu dört dikdörtgen dağılımın konvolüsyonudur: %95 uç noktaları "
       "±2√3[2 − (3/5)^{1/4}] = ±3,88'dir (JCGM 101 Ek E), GUM aralığı ±3,92'nin biraz içindedir."),
      ("With two significant digits of u(y) = 2.0 the numerical tolerance is δ = 0.05; the exact endpoint "
       "difference 3.920 − 3.881 = 0.039 is below δ, so the GUM interval is validated — but only just.",
       "u(y) = 2,0 için iki anlamlı basamakta sayısal tolerans δ = 0,05'tir; uç noktalar arasındaki kesin fark "
       "3,920 − 3,881 = 0,039, δ'dan küçük olduğundan GUM aralığı doğrulanır, ancak sınıra çok yakındır.")],
     {"gum": {"estimate": chk(0.0, 1e-12), "standard_uncertainty": chk(2.0, 1e-12)},
      "mcm": {"standard_uncertainty": chk(2.0, 0.005), "interval_low": chk(-3.8812, 0.01),
              "interval_high": chk(3.8812, 0.01), "gum_validated": True},
      "reference": "JCGM 101:2008 Table 3 and Annex E (analytic endpoints ±3.8812)."},
     [("JCGM 101 Table 3 reports the GUM interval validated in one of its two adaptive runs and not in the other "
       "(9.2.3.4): with d ≈ 0.039 against δ = 0.05, Monte Carlo noise in the endpoints can decide the outcome.",
       "JCGM 101 Tablo 3, iki uyarlamalı çalışmasından birinde GUM aralığını doğrulanmış, diğerinde doğrulanmamış "
       "olarak verir (9.2.3.4): δ = 0,05'e karşı d ≈ 0,039 olduğunda sonucu uç noktalardaki Monte Carlo gürültüsü belirleyebilir.")], 2, 2),
    ("jcgm101-9.2.4-additive-dominant-rectangular",
     ("Additive model with a dominant rectangular input", "Baskın dikdörtgen girdili toplamsal model"),
     ("JCGM 101 9.2.4: as 9.2.3 but X4 has standard deviation 10.",
      "JCGM 101 9.2.4: 9.2.3 ile aynı, ancak X4'ün standart sapması 10'dur."),
     [R(0.0, SQ3)] * 3 + [R(0.0, 10 * SQ3)], "symmetric",
     [("The output is nearly rectangular: its 95 % interval is about ±17.0, while the GUM interval (normal "
       "assumption) is ±19.9 — the central limit theorem does not apply when one input dominates.",
       "Çıktı neredeyse dikdörtgen dağılımlıdır: %95 aralığı yaklaşık ±17,0 iken GUM aralığı (normal varsayımı) "
       "±19,9'dur; bir girdi baskın olduğunda merkezi limit teoremi geçerli olmaz."),
      ("The GUM result is not validated for n_dig = 2 (δ = 0.5) but would be for n_dig = 1 (δ = 5).",
       "GUM sonucu n_dig = 2 için (δ = 0,5) doğrulanmaz, n_dig = 1 için (δ = 5) doğrulanırdı.")],
     {"gum": {"standard_uncertainty": chk(10.149, 0.0005)},
      "mcm": {"standard_uncertainty": chk(10.15, 0.06), "interval_low": chk(-17.0, 0.06),
              "interval_high": chk(17.0, 0.06), "gum_validated": False},
      "reference": "JCGM 101:2008 Table 4 (u(y) = √103 = 10.149)."},
     [], 2, 3),
]:
    project(sid, "general", title, desc, ADD_EXPL + expl, [src(JCGM101, sid.split("-")[1])],
            out("Y", "X1 + X2 + X3 + X4"),
            [inp(f"X{i + 1}", d) for i, d in enumerate(dists)],
            gum={}, mcm={"trials": 2000000 if "rectangular" in sid else 1000000, "seed": seed, "interval": interval,
                         "validation_ndig": ndig},
            expected=exp, notes=notes, tags=["jcgm101", "validation", "linear"])

# ---------------------------------------------------------------- JCGM 101 §9.3 mass
MASS_INPUTS = [
    inp("m_Rc", N(100000.0, 0.05), "mg", ("Conventional mass of the reference weight", "Referans ağırlığın konvansiyonel kütlesi")),
    inp("dm_Rc", N(1.234, 0.02), "mg", ("Mass of the small weight added to balance", "Dengelemek için eklenen küçük ağırlığın kütlesi")),
    inp("rho_a", R(1.2, 0.1), "kg/m³", ("Air density", "Hava yoğunluğu")),
    inp("rho_W", R(8000.0, 1000.0), "kg/m³", ("Density of the weight W", "W ağırlığının yoğunluğu")),
    inp("rho_R", R(8000.0, 50.0), "kg/m³", ("Density of the reference weight R", "R referans ağırlığının yoğunluğu")),
]
MASS_EXPR = "(m_Rc + dm_Rc) * (1 + (rho_a - 1.2) * (1/rho_W - 1/rho_R)) - 100000"
MASS_EXPL = [
    ("The conventional mass of W follows from weighing against R in air: m_W = (m_R,c + δm_R,c)[1 + (ρ_a − ρ_a0)(1/ρ_W − 1/ρ_R)], "
     "reported as the deviation δm = m_W − 100 g (in mg).",
     "W ağırlığının konvansiyonel kütlesi, havada R ile karşılaştırmalı tartımdan elde edilir: "
     "m_W = (m_R,c + δm_R,c)[1 + (ρ_a − ρ_a0)(1/ρ_W − 1/ρ_R)]; sonuç δm = m_W − 100 g sapması olarak (mg) verilir."),
    ("At the estimates, ρ_a = ρ_a0 = 1.2 kg/m³, so the sensitivity coefficients of the three densities are zero: a "
     "first-order evaluation ignores the buoyancy correction entirely (u = 0.0539 mg).",
     "Tahmin değerlerinde ρ_a = ρ_a0 = 1,2 kg/m³ olduğundan üç yoğunluğun duyarlılık katsayıları sıfırdır: birinci "
     "mertebe değerlendirme kaldırma kuvveti düzeltmesini tamamen yok sayar (u = 0,0539 mg)."),
]
project("jcgm101-9.3-mass-calibration", "mass",
        ("Mass calibration (first-order GUM)", "Kütle kalibrasyonu (birinci mertebe GUM)"),
        ("JCGM 101 9.3: calibration of a 100 g weight by comparison with a reference weight; first-order GUM vs MCM.",
         "JCGM 101 9.3: 100 g'lık bir ağırlığın referans ağırlıkla karşılaştırılarak kalibrasyonu; birinci mertebe GUM ile MCM karşılaştırması."),
        MASS_EXPL + [
            ("The Monte Carlo method propagates the full distributions and gives u = 0.0755 mg: the first-order GUM "
             "result is not validated even for one significant digit (n_dig = 1, δ = 0.005 mg).",
             "Monte Carlo yöntemi dağılımların tamamını yayar ve u = 0,0755 mg verir: birinci mertebe GUM sonucu tek "
             "anlamlı basamak için bile (n_dig = 1, δ = 0,005 mg) doğrulanmaz."),
            ("The project 'jcgm101-9.3-mass-calibration-higher-order' repeats the evaluation with the second-order "
             "terms of GUM 5.1.2, which are validated.",
             "'jcgm101-9.3-mass-calibration-higher-order' projesi, değerlendirmeyi GUM 5.1.2'nin ikinci mertebe "
             "terimleriyle tekrarlar; bu sonuç doğrulanır.")],
        [src(JCGM101, "9.3, Tables 6 and 7")],
        out("dm", MASS_EXPR, "mg", ("Deviation of the conventional mass of W from 100 g", "W'nin konvansiyonel kütlesinin 100 g'dan sapması")),
        MASS_INPUTS, gum={}, mcm={"trials": 2000000, "seed": 4, "interval": "symmetric", "validation_ndig": 1},
        expected={"gum": {"estimate": chk(1.2340, 0.00005), "standard_uncertainty": chk(0.0539, 0.00005)},
                  "mcm": {"estimate": chk(1.233998, 0.0002), "standard_uncertainty": chk(0.075484, 0.0002),
                          "interval_low": chk(1.08443, 0.0006), "interval_high": chk(1.38359, 0.0006),
                          "gum_validated": False},
                  "reference": "GUM: JCGM 101:2008 Table 6. MCM: independent 2 × 10⁸-trial reference "
                               "(validation/reference_values.json); tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶."},
        notes=[("JCGM 101 Table 6 reports the shortest interval [1.0834, 1.3825] from M = 0.72 × 10⁶; the output PDF is "
                "symmetric about 1.2340, so the probabilistically symmetric interval is used here (far less dispersed).",
                "JCGM 101 Tablo 6, M = 0,72 × 10⁶ ile en kısa aralığı [1,0834; 1,3825] olarak verir; çıktı dağılımı "
                "1,2340 etrafında simetrik olduğundan burada çok daha az saçılımlı olan olasılıksal simetrik aralık kullanılmıştır.")],
        tags=["jcgm101", "validation", "non-linear", "buoyancy"])
project("jcgm101-9.3-mass-calibration-higher-order", "mass",
        ("Mass calibration (GUM with second-order terms)", "Kütle kalibrasyonu (ikinci mertebe terimli GUM)"),
        ("JCGM 101 9.3: the same model evaluated with the higher-order terms of GUM 5.1.2.",
         "JCGM 101 9.3: aynı model, GUM 5.1.2'nin yüksek mertebe terimleriyle değerlendirilmiştir."),
        MASS_EXPL + [
            ("The second-order terms ½ΣΣ(∂²f/∂xᵢ∂xⱼ)²u²(xᵢ)u²(xⱼ) + ... capture the products (ρ_a − 1.2)(1/ρ_W − 1/ρ_R) and "
             "give u = 0.0750 mg, in agreement with the Monte Carlo result: validated for n_dig = 1.",
             "İkinci mertebe terimler (ρ_a − 1,2)(1/ρ_W − 1/ρ_R) çarpımlarını yakalar ve u = 0,0750 mg verir; bu "
             "değer Monte Carlo sonucuyla uyumludur ve n_dig = 1 için doğrulanır."),
            ("GUM 5.1.2 note derives these terms for Gaussian inputs; the program warns because the densities are "
             "rectangular. The Monte Carlo method has no such restriction.",
             "GUM 5.1.2 notu bu terimleri normal dağılımlı girdiler için türetir; yoğunluklar dikdörtgen dağılımlı "
             "olduğundan program uyarı verir. Monte Carlo yönteminde böyle bir kısıt yoktur.")],
        [src(JCGM101, "9.3, Table 6"), src(JCGM100, "5.1.2")],
        out("dm", MASS_EXPR, "mg", ("Deviation of the conventional mass of W from 100 g", "W'nin konvansiyonel kütlesinin 100 g'dan sapması")),
        MASS_INPUTS, gum={"higher_order": True},
        mcm={"trials": 2000000, "seed": 4, "interval": "symmetric", "validation_ndig": 1},
        expected={"gum": {"estimate": chk(1.2340, 0.00005), "standard_uncertainty": chk(0.0750, 0.00005)},
                  "mcm": {"standard_uncertainty": chk(0.075484, 0.0002), "interval_low": chk(1.08443, 0.0006),
                          "interval_high": chk(1.38359, 0.0006), "gum_validated": True},
                  "reference": "JCGM 101:2008 Table 6 (GUF2) and the independent MCM reference."},
        tags=["jcgm101", "validation", "non-linear", "higher-order"])

# ---------------------------------------------------------------- JCGM 101 §9.4 comparison loss
LOSS = [
    (0.000, 0.0, [50.0, 50.0, 0.0, 149.787], [0.2, 0.25, 0.001, 0.9], 50.0),
    (0.010, 0.0, [150.0, 111.803, 0.0, 366.005], [0.5, 0.4, 0.003, 1.5], 112.0),
    (0.050, 0.0, [2550.0, 502.494, 1593.570, 3548.570], [2.5, 1.0, 16.0, 17.0], 502.0),
    (0.000, 0.9, [50.006, 67.278, 0.0, 185.095], [0.2, 0.45, 0.001, 1.1], 0.0),
    (0.010, 0.9, [150.021, 120.531, 12.630, 397.486], [0.5, 0.6, 1.7, 2.5], 100.0),
    (0.050, 0.9, [2550.08, 504.542, 1624.033, 3554.431], [2.5, 1.2, 25.0, 25.0], 500.0),
]
for i, (x1, r, ref, tol, gum_u) in enumerate(LOSS):
    corr = r != 0.0
    sid = f"jcgm101-9.4-comparison-loss-x{int(round(x1 * 1000)):03d}" + ("-correlated" if corr else "")
    expl = [
        ("The comparison loss in microwave power meter calibration is δY = X₁² + X₂², where X₁ and X₂ are the real "
         "and imaginary parts of the voltage reflection coefficient, each with u = 0.005.",
         "Mikrodalga güç ölçer kalibrasyonundaki karşılaştırma kaybı δY = X₁² + X₂²'dir; X₁ ve X₂ gerilim yansıma "
         "katsayısının gerçel ve sanal kısımlarıdır ve her birinin u değeri 0,005'tir."),
        ("Because ∂δY/∂X₂ = 2x₂ = 0 (and ∂δY/∂X₁ = 0 when x₁ = 0), the first-order GUM result understates or even "
         "misses the uncertainty; the output PDF is a (scaled) non-central χ² distribution, strongly asymmetric.",
         "∂δY/∂X₂ = 2x₂ = 0 (ve x₁ = 0 iken ∂δY/∂X₁ = 0) olduğundan birinci mertebe GUM sonucu belirsizliği "
         "olduğundan küçük verir, hatta tamamen kaçırır; çıktı dağılımı ölçekli merkezî olmayan χ² dağılımıdır ve "
         "belirgin biçimde asimetriktir."),
        ("Because the PDF is asymmetric and bounded below by zero, the shortest 95 % coverage interval is reported.",
         "Dağılım asimetrik olup alttan sıfırla sınırlı olduğundan en kısa %95 kapsam aralığı raporlanır."),
    ]
    if corr:
        expl.append(("With r(X₁, X₂) = 0.9 the inputs are drawn from a bivariate Gaussian (Cholesky factor). The "
                     "first-order GUM result is unaffected by the correlation because c₂ = 0; second-order terms are "
                     "not available for correlated inputs (GUM 5.1.2 note), so only the MCM captures it.",
                     "r(X₁, X₂) = 0,9 iken girdiler iki değişkenli normal dağılımdan (Cholesky çarpanı) üretilir. "
                     "c₂ = 0 olduğundan birinci mertebe GUM sonucu korelasyondan etkilenmez; korelasyonlu girdiler "
                     "için ikinci mertebe terimler tanımlı değildir (GUM 5.1.2 notu), bu nedenle etkiyi yalnızca MCM yakalar."))
    else:
        expl.append(("The GUM result here includes the second-order terms (higher_order), which for this quadratic "
                     "model give the exact variance; the 95 % interval based on a symmetric distribution is still "
                     "inappropriate.",
                     "Buradaki GUM sonucu ikinci mertebe terimleri içerir; bu kuadratik model için varyansı tam olarak "
                     "verir, ancak simetrik dağılım varsayımına dayalı %95 aralığı yine de uygun değildir."))
    project(sid, "electrical",
            (f"Comparison loss, x₁ = {x1}" + (", r = 0.9" if corr else ""),
             f"Karşılaştırma kaybı, x₁ = {str(x1).replace('.', ',')}" + (", r = 0,9" if corr else "")),
            ("JCGM 101 9.4: comparison loss in the calibration of a microwave power meter (values in units of 10⁻⁶).",
             "JCGM 101 9.4: mikrodalga güç ölçer kalibrasyonunda karşılaştırma kaybı (değerler 10⁻⁶ biriminde)."),
            expl, [src(JCGM101, "9.4, Table 9" if corr else "9.4, Table 8")],
            out("dY", "1e6 * (X1^2 + X2^2)", "10⁻⁶", ("Comparison loss", "Karşılaştırma kaybı")),
            [inp("X1", N(x1, 0.005), None, ("Real part of the reflection coefficient", "Yansıma katsayısının gerçel kısmı")),
             inp("X2", N(0.0, 0.005), None, ("Imaginary part of the reflection coefficient", "Yansıma katsayısının sanal kısmı"))],
            correlations=[("X1", "X2", 0.9)] if corr else [],
            gum={} if corr else {"higher_order": True},
            mcm={"trials": 2000000, "seed": (30 if corr else 10) + i % 3, "coverage_probability": 0.95},
            expected={"gum": {"standard_uncertainty": chk(gum_u, 0.5)},
                      "mcm": {"estimate": chk(ref[0], tol[0]), "standard_uncertainty": chk(ref[1], tol[1]),
                              "interval_low": chk(ref[2], tol[2]), "interval_high": chk(ref[3], tol[3])},
                      "reference": ("JCGM 101:2008 Table 9 and an independent 2 × 10⁸-trial reference" if corr else
                                    "JCGM 101:2008 Table 8 and the analytic non-central χ² solution (Annex F)")
                                   + "; tolerances ≈ 5 × engine dispersion at M = 2 × 10⁶."},
            tags=["jcgm101", "validation", "non-linear", "asymmetric"] + (["correlation"] if corr else []))

# ---------------------------------------------------------------- GUM H.1 / JCGM 101 §9.5 gauge block
GB_EXPL = [
    ("The length of an end gauge is obtained by comparison with a standard of the same nominal length (50 mm): "
     "l = l_s + d − l_s(δα θ + α_s δθ), with θ = θ̄ + Δ the temperature deviation from 20 °C.",
     "Bir uç mastarının boyu, aynı anma boyundaki (50 mm) bir standartla karşılaştırılarak bulunur: "
     "l = l_s + d − l_s(δα θ + α_s δθ); burada θ = θ̄ + Δ, 20 °C'den sıcaklık sapmasıdır."),
    ("At the estimates δα = 0, δθ = 0, θ = −0.1 °C, the sensitivity coefficients of α_s, θ̄ and Δ vanish: their "
     "contributions appear only in second order (GUM H.1.7).",
     "Tahmin değerlerinde (δα = 0, δθ = 0, θ = −0,1 °C) α_s, θ̄ ve Δ'nın duyarlılık katsayıları sıfırdır; katkıları "
     "yalnızca ikinci mertebede ortaya çıkar (GUM H.1.7)."),
]
project("gum-h1-end-gauge", "length",
        ("End-gauge calibration (GUM H.1)", "Uç mastarı kalibrasyonu (GUM H.1)"),
        ("JCGM 100 Annex H.1: calibration of a 50 mm end gauge; Welch–Satterthwaite effective degrees of freedom and k for 99 %.",
         "JCGM 100 Ek H.1: 50 mm'lik bir uç mastarının kalibrasyonu; Welch–Satterthwaite etkin serbestlik derecesi ve %99 için k."),
        GB_EXPL + [
            ("Degrees of freedom: l_s 18, d 25.6, δα 50 (10 % reliable), δθ 2 (50 % reliable, GUM G.4.2). "
             "The Welch–Satterthwaite formula gives ν_eff = 16.7, truncated to 16, and k₉₉ = t₀.₉₉(16) = 2.92.",
             "Serbestlik dereceleri: l_s 18, d 25,6, δα 50 (%10 güvenilir), δθ 2 (%50 güvenilir, GUM G.4.2). "
             "Welch–Satterthwaite formülü ν_eff = 16,7 verir; 16'ya kesilir ve k₉₉ = t₀,₉₉(16) = 2,92 olur."),
            ("u_c = 32 nm and U₉₉ = 2.92 × 32 nm = 93 nm; l = (50.000 838 ± 0.000 093) mm.",
             "u_c = 32 nm ve U₉₉ = 2,92 × 32 nm = 93 nm; l = (50,000 838 ± 0,000 093) mm.")],
        [src(JCGM100, "H.1, Table H.1")],
        out("l", "l_s + d - l_s*(d_alpha*theta + alpha_s*d_theta)", "nm", ("Length of the end gauge", "Uç mastarının boyu")),
        [inp("l_s", N(50000623.0, 25.0), "nm", ("Length of the standard (calibration certificate)", "Standardın boyu (kalibrasyon sertifikası)"), 18),
         inp("d", N(215.0, 9.7), "nm", ("Measured length difference (mean of 5 readings, comparator)", "Ölçülen boy farkı (5 okumanın ortalaması, komparatör)"), 25.6),
         inp("alpha_s", R(11.5e-6, 2e-6), "1/°C", ("Thermal expansion coefficient of the standard", "Standardın ısıl genleşme katsayısı")),
         inp("theta_bar", N(-0.1, 0.2), "°C", ("Mean temperature deviation of the bed", "Tablanın ortalama sıcaklık sapması")),
         inp("Delta", ARC(0.0, 0.5), "°C", ("Cyclic temperature variation", "Periyodik sıcaklık değişimi")),
         inp("d_alpha", R(0.0, 1e-6), "1/°C", ("Difference of expansion coefficients", "Genleşme katsayıları farkı"), 50),
         inp("d_theta", R(0.0, 0.05), "°C", ("Temperature difference of the gauges", "Mastarlar arası sıcaklık farkı"), 2)],
        intermediates=[mid("theta", "theta_bar + Delta", "°C", ("Temperature deviation from 20 °C", "20 °C'den sıcaklık sapması"))],
        gum={"coverage_probability": 0.99}, mcm={"trials": 1000000, "seed": 40},
        reporting={"significant_digits": 2},
        expected={"gum": {"estimate": chk(50000838.0, 0.5), "standard_uncertainty": chk(32.0, 0.5),
                          "effective_dof": chk(16.7, 0.1), "coverage_factor": chk(2.92, 0.005),
                          "expanded_uncertainty": chk(93.0, 0.6)},
                  "reference": "JCGM 100:2008 H.1.4–H.1.6 (u_c = 32 nm, ν_eff = 16.7, k = 2.92, U = 93 nm)."},
        notes=[("The GUM multiplies the rounded u_c = 32 nm (2.92 × 32 = 93 nm); the unrounded value 31.7 nm gives "
                "U = 92.5 nm. Including the second-order terms 11.7 and 1.7 nm (H.1.7) raises u_c to 34 nm.",
                "GUM yuvarlanmış u_c = 32 nm değerini çarpar (2,92 × 32 = 93 nm); yuvarlanmamış 31,7 nm ile "
                "U = 92,5 nm bulunur. İkinci mertebe 11,7 ve 1,7 nm terimleri (H.1.7) eklendiğinde u_c 34 nm'ye çıkar."),
               ("The Monte Carlo result treats d as Gaussian; JCGM 101 9.5 (projects jcgm101-9.5-*) uses t "
                "distributions and curvilinear trapezoids for the inputs instead.",
                "Monte Carlo sonucu d'yi normal dağılımlı kabul eder; JCGM 101 9.5 (jcgm101-9.5-* projeleri) bunun "
                "yerine girdiler için t dağılımları ve eğrisel yamuk dağılımlar kullanır.")],
        tags=["gum", "validation", "welch-satterthwaite", "dimensional"])

GB_INPUTS = [
    inp("Ls", TD(50000623.0, 25.0, 18.0), "nm", ("Length of the standard at 20 °C", "Standardın 20 °C'deki boyu")),
    inp("D", TD(215.0, 6.0, 24.0), "nm", ("Mean difference of five indications", "Beş göstergenin ortalama farkı")),
    inp("d1", TD(0.0, 4.0, 5.0), "nm", ("Random effects of the comparator", "Komparatörün rastgele etkileri")),
    inp("d2", TD(0.0, 7.0, 8.0), "nm", ("Systematic effects of the comparator", "Komparatörün sistematik etkileri")),
    inp("alpha_s", RL(9.5e-6, 13.5e-6), "1/°C", ("Thermal expansion coefficient of the standard", "Standardın ısıl genleşme katsayısı")),
    inp("theta0", N(-0.1, 0.2), "°C", ("Mean temperature deviation", "Ortalama sıcaklık sapması")),
    inp("Delta", {"type": "arcsine", "lower": -0.5, "upper": 0.5}, "°C", ("Cyclic temperature variation", "Periyodik sıcaklık değişimi")),
    inp("d_alpha", {"type": "curvilinear_trapezoid", "lower": -1e-6, "upper": 1e-6, "d": 0.1e-6}, "1/°C",
        ("Difference of expansion coefficients (limits known to ±10 %)", "Genleşme katsayıları farkı (sınırlar ±%10 belirsiz)"), 50),
    inp("d_theta", {"type": "curvilinear_trapezoid", "lower": -0.05, "upper": 0.05, "d": 0.025}, "°C",
        ("Temperature difference of the gauges (limits known to ±50 %)", "Mastarlar arası sıcaklık farkı (sınırlar ±%50 belirsiz)"), 2),
]
for kind, expr, title in [
    ("approximate", "Ls + D + d1 + d2 - Ls*(d_alpha*(theta0 + Delta) + alpha_s*d_theta) - 50000000",
     ("Gauge block calibration, approximate model", "Mastar kalibrasyonu, yaklaşık model")),
    ("nonlinear", "(Ls*(1 + alpha_s*(theta0 + Delta - d_theta)) + D + d1 + d2) / (1 + (alpha_s + d_alpha)*(theta0 + Delta)) - 50000000",
     ("Gauge block calibration, non-linear model", "Mastar kalibrasyonu, doğrusal olmayan model")),
]:
    project(f"jcgm101-9.5-gauge-block-{kind}", "length", title,
            ("JCGM 101 9.5: the GUM H.1 gauge block with the input PDFs assigned by JCGM 101 (t, arc sine, curvilinear trapezoid); 99 % coverage.",
             "JCGM 101 9.5: girdilere JCGM 101'in atadığı dağılımlarla (t, ark sinüs, eğrisel yamuk) GUM H.1 mastarı; %99 kapsam."),
            GB_EXPL + [
                ("Type A inputs become scaled and shifted t distributions (JCGM 101 6.4.9); limits that are themselves "
                 "inexactly known become curvilinear trapezoids (6.4.3), whose GUM degrees of freedom express the "
                 "reliability of the limits.",
                 "A tipi girdiler ölçekli ve kaydırılmış t dağılımlarına dönüşür (JCGM 101 6.4.9); kendisi tam bilinmeyen "
                 "sınırlar eğrisel yamuk dağılımlara dönüşür (6.4.3) ve GUM serbestlik dereceleri bu sınırların "
                 "güvenilirliğini ifade eder."),
                ("The MCM gives u = 36 nm (GUM: 32 nm) because the product terms δα·θ and α_s·δθ are neglected to first "
                 "order; the 99 % shortest interval is [745, 932] nm relative to 50 mm.",
                 "MCM u = 36 nm verir (GUM: 32 nm), çünkü δα·θ ve α_s·δθ çarpım terimleri birinci mertebede ihmal "
                 "edilir; %99 en kısa aralık 50 mm'ye göre [745, 932] nm'dir.")],
            [src(JCGM101, "9.5, Tables 10 and 11"), src(JCGM100, "H.1")],
            out("dL", expr, "nm", ("Deviation of the gauge length from 50 mm", "Mastar boyunun 50 mm'den sapması")),
            GB_INPUTS, gum={"coverage_probability": 0.99}, mcm={"trials": 1000000, "seed": 40},
            expected={"gum": {"estimate": chk(838.0, 0.5), "standard_uncertainty": chk(32.0, 0.5),
                              "coverage_factor": chk(2.92, 0.005)},
                      "mcm": {"estimate": chk(838.0, 0.5), "standard_uncertainty": chk(36.0, 0.5),
                              "interval_low": chk(745.0, 1.5), "interval_high": chk(932.0, 1.5)},
                      "reference": "JCGM 101:2008 Table 11 (values rounded to 1 nm)."},
            tags=["jcgm101", "validation", "dimensional", "t-distribution", "curvilinear-trapezoid"])

# ---------------------------------------------------------------- EURACHEM QUAM
A1_INPUTS = [
    inp("P", R(0.9999, 0.0001), None, ("Purity of the cadmium metal (certificate: 99.99 ± 0.01 %)", "Kadmiyum metalinin saflığı (sertifika: %99,99 ± 0,01)")),
    inp("m", N(100.28, 0.05), "mg", ("Mass of the cadmium (weighing by difference)", "Kadmiyumun kütlesi (fark tartımı)")),
]
A1_EXPL = [
    ("c_Cd = 1000 · m · P / V: 100.28 mg of Cd of purity 0.9999 dissolved and made up to 100 mL.",
     "c_Cd = 1000 · m · P / V: saflığı 0,9999 olan 100,28 mg Cd çözülüp 100 mL'ye tamamlanır."),
    ("The purity is given as limits without a confidence level: rectangular distribution, u(P) = 0.0001/√3.",
     "Saflık güven düzeyi belirtilmeden sınırlar olarak verilmiştir: dikdörtgen dağılım, u(P) = 0,0001/√3."),
]
project("quam-a1-cadmium-standard", "chemistry",
        ("Preparation of a cadmium calibration standard", "Kadmiyum kalibrasyon standardının hazırlanması"),
        ("EURACHEM/CITAC QUAM Example A1: 1000 mg/L Cd standard from high-purity metal (summary budget).",
         "EURACHEM/CITAC QUAM Örnek A1: yüksek saflıkta metalden 1000 mg/L Cd standardı (özet bütçe)."),
        A1_EXPL + [
            ("The volume of the 100 mL flask combines calibration, repeatability and temperature effects into "
             "u(V) = 0.07 mL (see the project with the volume sub-model).",
             "100 mL'lik balonun hacmi kalibrasyon, tekrarlanabilirlik ve sıcaklık etkilerini birleştirerek "
             "u(V) = 0,07 mL verir (hacim alt modeli içeren projeye bakınız)."),
            ("The volume dominates; u_c = 0.86 mg/L and U = 2 × 0.86 ≈ 1.7 mg/L. The guide rounds u_c to 0.9 and "
             "reports (1002.7 ± 1.8) mg/L.",
             "Baskın katkı hacimdir; u_c = 0,86 mg/L ve U = 2 × 0,86 ≈ 1,7 mg/L. Rehber u_c'yi 0,9'a yuvarlar ve "
             "(1002,7 ± 1,8) mg/L olarak raporlar.")],
        [src(QUAM, "Example A1, Tables A1.2 and A1.3")],
        out("c_Cd", "1000 * m * P / V", "mg/L", ("Concentration of the calibration standard", "Kalibrasyon standardının derişimi")),
        A1_INPUTS + [inp("V", N(100.0, 0.07), "mL", ("Volume of the flask", "Balonun hacmi"))],
        gum=K2, mcm={"trials": 1000000, "seed": 1},
        expected={"gum": {"estimate": chk(1002.7, 0.05), "standard_uncertainty": chk(0.8633, 0.001)},
                  "reference": "QUAM:2012 Table A1.3 (spreadsheet value u = 0.8633 mg/L)."},
        notes=[("The guide's spreadsheet (Kragten) value 0.8633 mg/L differs from the exact first-order value 0.8638 "
                "mg/L only by the finite-difference approximation; the reported result is unchanged.",
                "Rehberin tablolama (Kragten) değeri 0,8633 mg/L ile kesin birinci mertebe değeri 0,8638 mg/L arasındaki "
                "fark yalnızca sonlu fark yaklaşımından kaynaklanır; raporlanan sonuç değişmez."),
               ("Doubling the rounded u_c (0.9 → U = 1.8 mg/L) is more conservative than 2 × 0.86 = 1.7 mg/L; "
                "the program rounds U itself to two significant digits.",
                "Yuvarlanmış u_c'nin iki katını almak (0,9 → U = 1,8 mg/L) 2 × 0,86 = 1,7 mg/L'den daha temkinlidir; "
                "program U'nun kendisini iki anlamlı basamağa yuvarlar.")],
        tags=["quam", "validation", "gravimetric", "volumetric"])
project("quam-a1-cadmium-standard-volume", "chemistry",
        ("Cadmium standard with the volume sub-model", "Hacim alt modelli kadmiyum standardı"),
        ("QUAM Example A1 with the volume of the flask modelled from its three components.",
         "Balon hacmi üç bileşeninden modellenmiş QUAM Örnek A1."),
        A1_EXPL + [
            ("V = V_nom + δV_cal + δV_rep + δV_temp: calibration ±0.1 mL (triangular, 0.04 mL), repeatability "
             "s = 0.02 mL, temperature ±4 °C × 2.1 × 10⁻⁴ °C⁻¹ × 100 mL = ±0.084 mL (rectangular, 0.05 mL) → u(V) = 0.066 mL.",
             "V = V_nom + δV_cal + δV_rep + δV_temp: kalibrasyon ±0,1 mL (üçgen, 0,04 mL), tekrarlanabilirlik "
             "s = 0,02 mL, sıcaklık ±4 °C × 2,1 × 10⁻⁴ °C⁻¹ × 100 mL = ±0,084 mL (dikdörtgen, 0,05 mL) → u(V) = 0,066 mL."),
            ("Rectangular and triangular components dominate, so the output PDF has lighter tails than a Gaussian: the "
             "Monte Carlo interval is about 0.015 mg/L shorter at each end, and the GUM interval is validated only for "
             "one significant digit of u.",
             "Dikdörtgen ve üçgen bileşenler baskın olduğundan çıktı dağılımının kuyrukları normal dağılımdan daha "
             "incedir: Monte Carlo aralığı her uçta yaklaşık 0,015 mg/L daha kısadır ve GUM aralığı u'nun yalnızca "
             "bir anlamlı basamağı için doğrulanır.")],
        [src(QUAM, "Example A1, A1.4")],
        out("c_Cd", "1000 * m * P / V", "mg/L", ("Concentration of the calibration standard", "Kalibrasyon standardının derişimi")),
        A1_INPUTS + [
            inp("V_nom", C(100.0), "mL", ("Nominal volume", "Anma hacmi")),
            inp("dV_cal", TRI(0.0, 0.1), "mL", ("Flask calibration (±0.1 mL)", "Balon kalibrasyonu (±0,1 mL)")),
            inp("dV_rep", N(0.0, 0.02), "mL", ("Filling repeatability", "Doldurma tekrarlanabilirliği")),
            inp("dV_temp", R(0.0, 0.084), "mL", ("Temperature effect (±4 °C)", "Sıcaklık etkisi (±4 °C)"))],
        intermediates=[mid("V", "V_nom + dV_cal + dV_rep + dV_temp", "mL", ("Volume of the flask", "Balonun hacmi"))],
        gum=K2, mcm={"trials": 2000000, "seed": 9, "interval": "symmetric"},
        expected={"gum": {"estimate": chk(1002.7, 0.05), "expanded_uncertainty": chk(1.7, 0.05)},
                  "reference": "QUAM:2012 A1.4: u(V) = 0.066 mL; U = 2u_c = 1.7 mg/L before the guide's rounding of u_c to 0.9 mg/L."},
        notes=[("The guide rounds u(V) = 0.066 mL to 0.07 mL and u_c to 0.9 mg/L before multiplying by k; with unrounded "
                "values u_c = 0.835 mg/L and U = 1.7 mg/L, against the printed (1002.7 ± 1.8) mg/L.",
                "Rehber k ile çarpmadan önce u(V) = 0,066 mL'yi 0,07 mL'ye ve u_c'yi 0,9 mg/L'ye yuvarlar; yuvarlanmamış "
                "değerlerle u_c = 0,835 mg/L ve U = 1,7 mg/L bulunur, basılı sonuç ise (1002,7 ± 1,8) mg/L'dir.")],
        tags=["quam", "sub-model", "volumetric", "mcm-validation"])

project("quam-a2-naoh-standardisation", "chemistry",
        ("Standardising a sodium hydroxide solution", "Sodyum hidroksit çözeltisinin ayarlanması"),
        ("QUAM Example A2: NaOH standardised against potassium hydrogen phthalate (KHP); summary budget with the guide's standard uncertainties.",
         "QUAM Örnek A2: NaOH çözeltisinin potasyum hidrojen ftalata (KHP) karşı ayarlanması; rehberin standart belirsizlikleriyle özet bütçe."),
        [("c_NaOH = 1000 · m_KHP · P_KHP / (M_KHP · V_T) · rep (mol/L), where rep is a repeatability factor with relative "
          "standard uncertainty 0.05 %.",
          "c_NaOH = 1000 · m_KHP · P_KHP / (M_KHP · V_T) · rep (mol/L); burada rep, bağıl standart belirsizliği %0,05 "
          "olan tekrarlanabilirlik çarpanıdır."),
         ("For a product/quotient model, relative uncertainties combine in quadrature: "
          "u_c/c = √[(0.0005)² + (0.00013/0.3888)² + (0.00029)² + (0.0038/204.2212)² + (0.013/18.64)²].",
          "Çarpım/bölüm modelinde bağıl belirsizlikler kareler toplamının karekökü ile birleşir: "
          "u_c/c = √[(0,0005)² + (0,00013/0,3888)² + (0,00029)² + (0,0038/204,2212)² + (0,013/18,64)²]."),
         ("The titration volume and the repeatability dominate; c = (0.10214 ± 0.00020) mol/L with k = 2.",
          "Titrasyon hacmi ve tekrarlanabilirlik baskındır; k = 2 ile c = (0,10214 ± 0,00020) mol/L.")],
        [src(QUAM, "Example A2, Tables A2.2 and A2.3")],
        out("c_NaOH", "1000 * m_KHP * P_KHP / (M_KHP * V_T) * rep", "mol/L", ("Concentration of the NaOH solution", "NaOH çözeltisinin derişimi")),
        [inp("rep", N(1.0, 0.0005), None, ("Repeatability factor", "Tekrarlanabilirlik çarpanı")),
         inp("m_KHP", N(0.3888, 0.00013), "g", ("Mass of KHP", "KHP kütlesi")),
         inp("P_KHP", N(1.0, 0.00029), None, ("Purity of KHP", "KHP saflığı")),
         inp("M_KHP", N(204.2212, 0.0038), "g/mol", ("Molar mass of KHP", "KHP'nin molar kütlesi")),
         inp("V_T", N(18.64, 0.013), "mL", ("Titration volume of NaOH", "NaOH titrasyon hacmi"))],
        gum=K2, mcm={"trials": 1000000, "seed": 21},
        expected={"gum": {"estimate": chk(0.102136, 5e-7), "standard_uncertainty": chk(0.000099, 5e-7)},
                  "reference": "QUAM:2012 Table A2.3 (c = 0.102136 mol/L, u_c = 0.000099 mol/L)."},
        tags=["quam", "validation", "titration"])

lin = 0.15e-3
project("quam-a2-naoh-standardisation-submodels", "chemistry",
        ("NaOH standardisation with sub-models", "Alt modellerle NaOH ayarlanması"),
        ("QUAM Example A2 with mass, molar mass and volume built from their components.",
         "Kütle, molar kütle ve hacmin bileşenlerinden oluşturulduğu QUAM Örnek A2."),
        [("m_KHP = m_gross − m_tare; each weighing carries a balance linearity term ±0.15 mg (rectangular).",
          "m_KHP = m_brüt − m_dara; her tartım ±0,15 mg'lık terazi doğrusallık terimi (dikdörtgen) içerir."),
         ("M_KHP = 8A_C + 5A_H + 4A_O + A_K from IUPAC atomic weights; the atoms of one element are fully correlated, "
          "which the model expresses by multiplying the single atomic weight by the number of atoms.",
          "M_KHP = 8A_C + 5A_H + 4A_O + A_K, IUPAC atom ağırlıklarından; bir elementin atomları tam korelasyonludur ve "
          "model bunu tek atom ağırlığını atom sayısıyla çarparak ifade eder."),
         ("V_T = 18.64 mL + burette calibration (±0.03 mL, triangular) + temperature effect "
          "(19 mL × 2.1 × 10⁻⁴ °C⁻¹ × 3 °C / 1.96, normal).",
          "V_T = 18,64 mL + büret kalibrasyonu (±0,03 mL, üçgen) + sıcaklık etkisi "
          "(19 mL × 2,1 × 10⁻⁴ °C⁻¹ × 3 °C / 1,96, normal)."),
         ("Result: (0.1021 ± 0.0002) mol/L with k = 2, as in the guide.",
          "Sonuç: rehberdeki gibi k = 2 ile (0,1021 ± 0,0002) mol/L.")],
        [src(QUAM, "Example A2, A2.4")],
        out("c_NaOH", "1000 * m_KHP * P_KHP / (M_KHP * V_T) * rep", "mol/L", ("Concentration of the NaOH solution", "NaOH çözeltisinin derişimi")),
        [inp("rep", N(1.0, 0.0005), None, ("Repeatability factor", "Tekrarlanabilirlik çarpanı")),
         inp("m_gross", R(60.5450, lin), "g", ("Container and KHP", "Kap ve KHP")),
         inp("m_tare", R(60.1562, lin), "g", ("Empty container", "Boş kap")),
         inp("P_KHP", R(1.0, 0.0005), None, ("Purity of KHP (1.0000 ± 0.0005)", "KHP saflığı (1,0000 ± 0,0005)")),
         inp("A_C", R(12.0107, 0.0008), "g/mol", ("Atomic weight of C", "C atom ağırlığı")),
         inp("A_H", R(1.00794, 0.00007), "g/mol", ("Atomic weight of H", "H atom ağırlığı")),
         inp("A_O", R(15.9994, 0.0003), "g/mol", ("Atomic weight of O", "O atom ağırlığı")),
         inp("A_K", R(39.0983, 0.0001), "g/mol", ("Atomic weight of K", "K atom ağırlığı")),
         inp("dV_cal", TRI(0.0, 0.03), "mL", ("Burette calibration", "Büret kalibrasyonu")),
         inp("dV_temp", N(0.0, 19.0 * 2.1e-4 * 3.0 / 1.96), "mL", ("Temperature effect (±3 °C at 95 %)", "Sıcaklık etkisi (%95'te ±3 °C)"))],
        intermediates=[mid("m_KHP", "m_gross - m_tare", "g", ("Mass of KHP", "KHP kütlesi")),
                       mid("M_KHP", "8*A_C + 5*A_H + 4*A_O + A_K", "g/mol", ("Molar mass of KHP", "KHP'nin molar kütlesi")),
                       mid("V_T", "18.64 + dV_cal + dV_temp", "mL", ("Titration volume", "Titrasyon hacmi"))],
        gum=K2, mcm={"trials": 1000000, "seed": 21}, reporting={"significant_digits": 1},
        expected={"gum": {"estimate": chk(0.102136, 5e-7), "expanded_uncertainty": chk(0.0002, 0.00005)},
                  "reference": "QUAM:2012 A2.4–A2.6, result (0.1021 ± 0.0002) mol/L."},
        notes=[("Finding: the guide rounds 0.15/√3 = 0.087 mg to 0.09 mg, giving u(m) = 0.13 mg; unrounded u(m) = 0.12 mg. "
                "Likewise u(V_T) = 0.0137 mL (guide 0.013). The unrounded combined uncertainty is 0.000101 mol/L "
                "(guide 0.000099); the reported result is unchanged.",
                "Bulgu: rehber 0,15/√3 = 0,087 mg değerini 0,09 mg'ye yuvarlayarak u(m) = 0,13 mg bulur; yuvarlanmamış "
                "u(m) = 0,12 mg'dir. Benzer şekilde u(V_T) = 0,0137 mL'dir (rehber 0,013). Yuvarlanmamış birleşik "
                "belirsizlik 0,000101 mol/L'dir (rehber 0,000099); raporlanan sonuç değişmez.")],
        tags=["quam", "sub-model", "titration", "atomic-weights"])

project("quam-a3-hcl-titration", "chemistry",
        ("Acid–base titration of hydrochloric acid", "Hidroklorik asidin asit–baz titrasyonu"),
        ("QUAM Example A3: HCl titrated with NaOH that was itself standardised against KHP.",
         "QUAM Örnek A3: KHP'ye karşı ayarlanmış NaOH ile HCl titrasyonu."),
        [("c_HCl = 1000 · m_KHP · P_KHP · V_T2 / (V_T1 · M_KHP · V_HCl) · rep: V_T1 is the NaOH volume in the "
          "standardisation, V_T2 in the titration of 15 mL of HCl.",
          "c_HCl = 1000 · m_KHP · P_KHP · V_T2 / (V_T1 · M_KHP · V_HCl) · rep: V_T1 ayarlamadaki, V_T2 ise 15 mL "
          "HCl'nin titrasyonundaki NaOH hacmidir."),
         ("The repeatability of the whole two-step procedure is 0.1 % (rep), the largest single contribution.",
          "İki adımlı işlemin tamamının tekrarlanabilirliği %0,1'dir (rep); en büyük tekil katkı budur."),
         ("c = (0.1014 ± 0.0004) mol/L with k = 2.", "k = 2 ile c = (0,1014 ± 0,0004) mol/L.")],
        [src(QUAM, "Example A3, Tables A3.2 and A3.3")],
        out("c_HCl", "1000 * m_KHP * P_KHP * V_T2 / (V_T1 * M_KHP * V_HCl) * rep", "mol/L", ("Concentration of HCl", "HCl derişimi")),
        [inp("rep", N(1.0, 0.001), None, ("Repeatability factor", "Tekrarlanabilirlik çarpanı")),
         inp("m_KHP", N(0.3888, 0.00012), "g", ("Mass of KHP", "KHP kütlesi")),
         inp("P_KHP", N(1.0, 0.00029), None, ("Purity of KHP", "KHP saflığı")),
         inp("V_T2", N(14.89, 0.014), "mL", ("NaOH volume for the HCl titration", "HCl titrasyonu için NaOH hacmi")),
         inp("V_T1", N(18.64, 0.015), "mL", ("NaOH volume for the standardisation", "Ayarlama için NaOH hacmi")),
         inp("M_KHP", N(204.2212, 0.0038), "g/mol", ("Molar mass of KHP", "KHP'nin molar kütlesi")),
         inp("V_HCl", N(15.0, 0.011), "mL", ("Aliquot of HCl", "HCl hacmi"))],
        gum=K2, mcm={"trials": 1000000, "seed": 3}, reporting={"significant_digits": 1},
        expected={"gum": {"estimate": chk(0.101387, 5e-7), "standard_uncertainty": chk(0.00018276, 5e-7),
                          "expanded_uncertainty": chk(0.0004, 0.00005)},
                  "reference": "QUAM:2012 Table A3.3 (u_c = √3.34 × 10⁻⁸ mol/L) and result (0.1014 ± 0.0004) mol/L."},
        notes=[("Finding: summary Table A3.1 states u_c = 0.00016 mol/L, but its own inputs give 0.00019 mol/L; 0.00016 "
                "is the result of the triple determination (Table A3.4).",
                "Bulgu: Özet Tablo A3.1 u_c = 0,00016 mol/L verir, ancak kendi girdileri 0,00019 mol/L sonucunu verir; "
                "0,00016 üçlü tayinin sonucudur (Tablo A3.4).")],
        tags=["quam", "validation", "titration"])

project("quam-a4-pesticides-combination", "chemistry",
        ("Organophosphorus pesticides in bread: combination", "Ekmekte organofosforlu pestisitler: birleştirme"),
        ("QUAM Example A4: the relative uncertainty from in-house validation data (precision, recovery, homogeneity) combined in a multiplicative model at P = 1.",
         "QUAM Örnek A4: laboratuvar içi validasyon verilerinden (kesinlik, geri kazanım, homojenlik) elde edilen bağıl belirsizliğin P = 1'de çarpımsal modelde birleştirilmesi."),
        [("P_op = F_I · F_hom / Rec, evaluated at nominal values: precision 0.27 (from 15 duplicates), recovery "
          "0.90 with u = 0.043 (correction applied because t = 2.31 > t_crit), homogeneity 0.20.",
          "P_op = F_I · F_hom / Rec, anma değerlerinde değerlendirilir: kesinlik 0,27 (15 mükerrer ölçümden), geri "
          "kazanım 0,90 ve u = 0,043 (t = 2,31 > t_kritik olduğundan düzeltme uygulanır), homojenlik 0,20."),
         ("Relative uncertainty: √(0.27² + 0.048² + 0.20²) = 0.34; at P = 1.1111 the guide's spreadsheet gives u = 0.377.",
          "Bağıl belirsizlik: √(0,27² + 0,048² + 0,20²) = 0,34; P = 1,1111'de rehberin tablolaması u = 0,377 verir."),
         ("With relative uncertainties of 20–27 % the model is visibly non-linear: the Monte Carlo distribution is "
          "skewed and its standard deviation differs from the GUM value.",
          "%20–27 bağıl belirsizliklerle model belirgin biçimde doğrusal değildir: Monte Carlo dağılımı çarpıktır ve "
          "standart sapması GUM değerinden farklıdır.")],
        [src(QUAM, "Example A4, Tables A4.4 and A4.5")],
        out("P_op", "F_I * F_hom / Rec", None, ("Pesticide level relative to the measured value", "Ölçülen değere göre pestisit düzeyi")),
        [inp("F_I", N(1.0, 0.27), None, ("Intermediate precision factor", "Ara kesinlik çarpanı")),
         inp("Rec", N(0.9, 0.043), None, ("Mean recovery", "Ortalama geri kazanım")),
         inp("F_hom", N(1.0, 0.2), None, ("Sample inhomogeneity factor", "Numune homojensizliği çarpanı"))],
        gum={}, mcm={"trials": 1000000, "seed": 4},
        expected={"gum": {"estimate": chk(1.1111, 0.00005), "standard_uncertainty": chk(0.377, 0.0005)},
                  "reference": "QUAM:2012 Table A4.5 (P = 1.1111, u = 0.377)."},
        notes=[("Finding: Table A4.2 contains typographical errors in derived columns (mean 0.75 for 0.075, difference "
                "−0.10 for −0.01, mean 0.10 for 0.01); the guide quotes t_crit = 2.021, i.e. t(40), for 41 degrees of "
                "freedom (t(41) = 2.0195). Neither changes the conclusion.",
                "Bulgu: Tablo A4.2'nin türetilmiş sütunlarında yazım hataları vardır (0,075 yerine ortalama 0,75, −0,01 "
                "yerine fark −0,10, 0,01 yerine ortalama 0,10); rehber 41 serbestlik derecesi için t_kritik = 2,021, yani "
                "t(40) kullanır (t(41) = 2,0195). Hiçbiri sonucu değiştirmez.")],
        tags=["quam", "validation-data", "recovery", "non-linear"])

project("quam-a5-cadmium-release", "chemistry",
        ("Cadmium released from ceramic ware", "Seramik kaplardan salınan kadmiyum"),
        ("QUAM Example A5: empirical method (BS 6748); amount of Cd leached per unit area.",
         "QUAM Örnek A5: ampirik yöntem (BS 6748); birim alan başına çözünen Cd miktarı."),
        [("r = c₀ · V_L / a_V · f_acid · f_time · f_temp (mg/dm²): c₀ from a straight-line AAS calibration "
          "(u(c₀) = 0.018 mg/L, see calibration module), leach volume 0.332 L, surface area 5.73 dm².",
          "r = c₀ · V_L / a_V · f_asit · f_süre · f_sıcaklık (mg/dm²): c₀ doğrusal AAS kalibrasyonundan "
          "(u(c₀) = 0,018 mg/L, kalibrasyon modülüne bakınız), özütleme hacmi 0,332 L, yüzey alanı 5,73 dm²."),
         ("The f factors represent the effects of the empirical method's conditions (acid concentration, time, "
          "temperature); for an empirical method they are 1 by definition but carry uncertainty.",
          "f çarpanları ampirik yöntemin koşullarının (asit derişimi, süre, sıcaklık) etkilerini temsil eder; ampirik "
          "yöntemde tanım gereği 1'dir ancak belirsizlik taşırlar."),
         ("The calibration (c₀) and the temperature factor dominate; r = (0.015 ± 0.003) mg/dm² with k = 2.",
          "Kalibrasyon (c₀) ve sıcaklık çarpanı baskındır; k = 2 ile r = (0,015 ± 0,003) mg/dm².")],
        [src(QUAM, "Example A5, Tables A5.3 and A5.4")],
        out("r", "c0 * V_L / a_V * f_acid * f_time * f_temp", "mg/dm²", ("Cd released per unit area", "Birim alan başına salınan Cd")),
        [inp("c0", N(0.26, 0.018), "mg/L", ("Cd concentration in the leach solution", "Özütleme çözeltisindeki Cd derişimi")),
         inp("V_L", N(0.332, 0.0018), "L", ("Volume of the leachate", "Özütleme çözeltisinin hacmi")),
         inp("a_V", N(5.73, 0.19), "dm²", ("Surface area of the vessel", "Kabın yüzey alanı")),
         inp("f_acid", N(1.0, 0.0008), None, ("Acid concentration factor", "Asit derişimi çarpanı")),
         inp("f_time", N(1.0, 0.001), None, ("Leaching time factor", "Özütleme süresi çarpanı")),
         inp("f_temp", N(1.0, 0.06), None, ("Temperature factor", "Sıcaklık çarpanı"))],
        gum=K2, mcm={"trials": 1000000, "seed": 5}, reporting={"significant_digits": 1},
        expected={"gum": {"estimate": chk(0.015065, 5e-7), "standard_uncertainty": chk(0.001465, 6e-6),
                          "expanded_uncertainty": chk(0.003, 0.0005)},
                  "reference": "QUAM:2012 Table A5.4 (r = 0.015065 mg/dm², spreadsheet u = 0.001465) and result (0.015 ± 0.003) mg/dm²."},
        notes=[("Finding: a_V = πd²/4 with d = 2.70 dm (the guide's formula shows 2.77, a typo); its components 0.042 and "
                "0.146 dm² combine to 0.152 dm², not the 0.19 dm² printed and used here. With 0.152 dm², u_c = 0.00144 "
                "mg/dm² instead of 0.00147; the reported result is unchanged.",
                "Bulgu: a_V = πd²/4, d = 2,70 dm (rehberin formülünde 2,77 yazım hatası); bileşenleri 0,042 ve 0,146 dm² "
                "olup 0,152 dm² verir, burada da kullanılan basılı 0,19 dm² değil. 0,152 dm² ile u_c = 0,00147 yerine "
                "0,00144 mg/dm² bulunur; raporlanan sonuç değişmez."),
               ("The exact first-order u_c is 0.001470 mg/dm²; the spreadsheet value 0.001465 differs because of the "
                "1/a_V non-linearity in the finite differences.",
                "Kesin birinci mertebe u_c 0,001470 mg/dm²'dir; tablolama değeri 0,001465, sonlu farklardaki 1/a_V "
                "doğrusalsızlığı nedeniyle farklıdır.")],
        tags=["quam", "validation", "empirical-method", "aas"])

project("quam-a7-assay-standard", "chemistry",
        ("Lead assay standard for IDMS", "IDMS için kurşun tayin standardı"),
        ("QUAM Example A7, Eq. (8): amount content of the primary lead assay standard prepared gravimetrically.",
         "QUAM Örnek A7, Eş. (8): gravimetrik olarak hazırlanmış birincil kurşun tayin standardının madde miktarı içeriği."),
        [("c_z = 10⁶ · (m₂/d₂) · m₁ · w / (d₁ · M): a mass m₁ of lead of purity w dissolved, diluted by mass (d₁, d₂) "
          "and an aliquot m₂ taken; M is the molar mass of lead in the assay material.",
          "c_z = 10⁶ · (m₂/d₂) · m₁ · w / (d₁ · M): saflığı w olan m₁ kütlesindeki kurşun çözülür, kütlece seyreltilir "
          "(d₁, d₂) ve m₂ kadar alınır; M tayin malzemesindeki kurşunun molar kütlesidir."),
         ("All inputs are Gaussian with the standard uncertainties of Table A7.3; c_z = 0.092605 µmol/g, u = 0.000028 µmol/g.",
          "Tüm girdiler Tablo A7.3'teki standart belirsizliklerle normal dağılımlıdır; c_z = 0,092605 µmol/g, u = 0,000028 µmol/g.")],
        [src(QUAM, "Example A7, Eq. (8), Table A7.3")],
        out("c_z", "1e6 * m2 / d2 * m1 * w / d1 / M", "µmol/g", ("Amount content of the assay standard", "Tayin standardının madde miktarı içeriği")),
        [inp("m1", N(0.36544, 0.00005), "g", ("Mass of lead", "Kurşun kütlesi")),
         inp("d1", N(196.14, 0.03), "g", ("First dilution (total mass)", "Birinci seyreltme (toplam kütle)")),
         inp("m2", N(1.0292, 0.0002), "g", ("Aliquot of the first dilution", "Birinci seyreltmeden alınan kütle")),
         inp("d2", N(99.931, 0.01), "g", ("Second dilution (total mass)", "İkinci seyreltme (toplam kütle)")),
         inp("w", N(0.99999, 0.000005), None, ("Purity of the lead", "Kurşunun saflığı")),
         inp("M", N(207.2104, 0.0010), "g/mol", ("Molar mass of lead in the assay material", "Tayin malzemesindeki kurşunun molar kütlesi"))],
        gum=K2, mcm={"trials": 1000000, "seed": 7},
        expected={"gum": {"estimate": chk(0.092605, 1e-6), "standard_uncertainty": chk(0.000028, 5e-7)},
                  "reference": "QUAM:2012 Example A7, Table A7.3."},
        tags=["quam", "validation", "idms", "gravimetric"])

S8 = math.sqrt(8)
idms_inputs = [
    inp("c_z", N(0.092605, 0.000028), "µmol/g", ("Amount content of the assay standard", "Tayin standardının madde miktarı içeriği")),
    inp("m_x", N(1.0440, 0.0002), "g", ("Mass of sample in blend b", "b karışımındaki numune kütlesi")),
    inp("m_y", N(1.1360, 0.0002), "g", ("Mass of spike in blend b", "b karışımındaki işaretleyici kütlesi")),
    inp("m_y2", N(1.0654, 0.0002), "g", ("Mass of spike in blend b'", "b' karışımındaki işaretleyici kütlesi")),
    inp("m_z", N(1.1029, 0.0002), "g", ("Mass of assay standard in blend b'", "b' karışımındaki tayin standardı kütlesi")),
]
typeA = [("K0_b", 0.9987, 0.0025), ("K0_b2", 0.9983, 0.0025), ("K0_x1", 0.9992, 0.0025), ("K0_x3", 1.0004, 0.0035),
         ("K0_x4", 1.001, 0.006), ("K0_y1", 0.9999, 0.0025), ("K0_z1", 0.9989, 0.0025), ("K0_z3", 0.9993, 0.0035),
         ("K0_z4", 1.0002, 0.006), ("R_b", 0.29360, 0.00073), ("R_b2", 0.5050, 0.0013), ("R_x1", 2.1402, 0.0054),
         ("R_x3", 0.9142, 0.0032), ("R_x4", 0.05901, 0.00035), ("R_y1", 0.00064, 0.00004), ("R_z1", 2.1429, 0.0054),
         ("R_z3", 0.9147, 0.0032), ("R_z4", 0.05870, 0.00035)]
for n, x, u in typeA:
    what = ("Mass-bias correction factor (measured)", "Kütle ayrımı düzeltme çarpanı (ölçülen)") if n.startswith("K0") else \
        ("Measured isotope amount ratio", "Ölçülen izotop madde miktarı oranı")
    idms_inputs.append(inp(n, N(x, u / S8), None, (f"{what[0]}; u = {u}/√8", f"{what[1]}; u = {str(u).replace('.', ',')}/√8")))
idms_inputs.append(inp("c_blank", N(4.5e-7, 4.0e-7 / S8), "µmol/g", ("Procedure blank; u = 4.0 × 10⁻⁷/√8", "Yöntem körü; u = 4,0 × 10⁻⁷/√8")))
keys = ["b", "b2", "x1", "x3", "x4", "y1", "z1", "z3", "z4"]
for k in keys:
    idms_inputs.append(inp(f"Kbias_{k}", N(0.0, 0.001), None, ("Bias of the mass-bias correction", "Kütle ayrımı düzeltmesinin sapması")))
project("quam-a7-lead-double-idms", "chemistry",
        ("Lead in water by double isotope dilution", "Suda kurşunun çift izotop seyreltmesiyle tayini"),
        ("QUAM Example A7, Eq. (5): double IDMS with mass-bias-corrected isotope ratios (replicate 1).",
         "QUAM Örnek A7, Eş. (5): kütle ayrımı düzeltilmiş izotop oranlarıyla çift IDMS (1. tekrar)."),
        [("c_x = c_z · (m_y/m_x) · (m_z/m_y') · (K_y1R_y1 − K_bR_b)/(K_bR_b − K_x1R_x1) · (K_b'R_b' − K_z1R_z1)/(K_y1R_y1 − K_b'R_b') "
          "· ΣK_xiR_xi/ΣK_ziR_zi − c_blank, with every K = K₀ + K_bias.",
          "c_x = c_z · (m_y/m_x) · (m_z/m_y') · (K_y1R_y1 − K_bR_b)/(K_bR_b − K_x1R_x1) · (K_b'R_b' − K_z1R_z1)/(K_y1R_y1 − K_b'R_b') "
          "· ΣK_xiR_xi/ΣK_ziR_zi − c_kör; her K = K₀ + K_sapma."),
         ("Type A uncertainties of the ratios and K₀ are divided by √8 (mean of eight measurements); the bias terms "
          "K_bias (u = 0.001 each) contribute about 38 % of the variance.",
          "Oranların ve K₀'ın A tipi belirsizlikleri √8'e bölünür (sekiz ölçümün ortalaması); K_sapma terimleri "
          "(her biri u = 0,001) varyansın yaklaşık %38'ini oluşturur."),
         ("c_x = 0.053738 µmol/g with u_c = 0.00018 µmol/g.", "c_x = 0,053738 µmol/g, u_c = 0,00018 µmol/g.")],
        [src(QUAM, "Example A7, Eq. (5), Tables A7.4–A7.8")],
        out("c_x", "c_z * m_y/m_x * m_z/m_y2 * (K_y1*R_y1 - K_b*R_b)/(K_b*R_b - K_x1*R_x1) * (K_b2*R_b2 - K_z1*R_z1)/(K_y1*R_y1 - K_b2*R_b2) * sum_x/sum_z - c_blank",
            "µmol/g", ("Amount content of lead in the water sample", "Su numunesindeki kurşunun madde miktarı içeriği")),
        idms_inputs,
        intermediates=[mid(f"K_{k}", f"K0_{k} + Kbias_{k}") for k in keys] + [
            mid("sum_x", "K_x1*R_x1 + 1 + K_x3*R_x3 + K_x4*R_x4"), mid("sum_z", "K_z1*R_z1 + 1 + K_z3*R_z3 + K_z4*R_z4")],
        gum=K2, mcm={"trials": 1000000, "seed": 8},
        expected={"gum": {"estimate": chk(0.053738, 3e-6), "standard_uncertainty": chk(0.00018, 5e-6)},
                  "reference": "QUAM:2012 Example A7, Table A7.8 (replicate 1)."},
        notes=[("The reported (0.05370 ± 0.00036) µmol/g uses the mean of four replicates; without the division by √8 "
                "the experimental u_c would be 0.00041 µmol/g.",
                "Raporlanan (0,05370 ± 0,00036) µmol/g dört tekrarın ortalamasını kullanır; √8'e bölme yapılmasaydı "
                "deneysel u_c 0,00041 µmol/g olurdu.")],
        tags=["quam", "validation", "idms", "isotope-ratios"])

# ---------------------------------------------------------------- EA-4/02
def ea(id, field, title, desc, expl, section, output, inputs, expected, **kw):
    kw.setdefault("gum", K2)
    kw.setdefault("mcm", {"trials": 1000000, "seed": 1})
    project(id, field, title, desc, expl, [src(EA, section)], output, inputs, expected=expected,
            tags=["ea-4/02", "calibration", "validation"] + kw.pop("tags", []), **kw)


ea("ea402-s2-weight-10kg", "mass", ("Calibration of a 10 kg weight", "10 kg'lık bir ağırlığın kalibrasyonu"),
   ("EA-4/02 S2: OIML class M1 weight calibrated against an F2 standard with a mass comparator (ABBA).",
    "EA-4/02 S2: OIML M1 sınıfı ağırlığın F2 standarda karşı kütle komparatörüyle (ABBA) kalibrasyonu."),
   [("m_X = m_S + δm_D + δm + δm_C + δB (deviations from 10 kg, in mg).",
     "m_X = m_S + δm_D + δm + δm_C + δB (10 kg'dan sapmalar, mg)."),
    ("The observed difference is the mean of three ABBA comparisons (+10, +30, +20 mg); its standard uncertainty uses "
     "the pooled standard deviation s_p = 25 mg from an earlier evaluation: u(δm) = 25/√3 = 14.4 mg.",
     "Gözlenen fark üç ABBA karşılaştırmasının ortalamasıdır (+10, +30, +20 mg); standart belirsizliği daha önceki "
     "bir değerlendirmeden birleştirilmiş standart sapma s_p = 25 mg ile hesaplanır: u(δm) = 25/√3 = 14,4 mg."),
    ("Drift ±15 mg, comparator ±10 mg and air buoyancy ±10 mg (1 × 10⁻⁶ of the nominal value) are rectangular.",
     "Sürüklenme ±15 mg, komparatör ±10 mg ve hava kaldırma kuvveti ±10 mg (anma değerinin 1 × 10⁻⁶'sı) dikdörtgen dağılımlıdır."),
    ("m_X = 10 000.025 kg ± 58 mg (k = 2).", "m_X = 10 000,025 kg ± 58 mg (k = 2).")],
   "S2", out("m_X", "m_S + dm_D + dm + dm_C + dB", "mg", ("Deviation of the weight from 10 kg", "Ağırlığın 10 kg'dan sapması")),
   [inp("m_S", NU(5.0, 45.0, 2), "mg", ("Standard (certificate: +5 mg, U = 45 mg, k = 2)", "Standart (sertifika: +5 mg, U = 45 mg, k = 2)")),
    inp("dm_D", R(0.0, 15.0), "mg", ("Drift of the standard since its last calibration", "Standardın son kalibrasyondan beri sürüklenmesi")),
    inp("dm", N(20.0, 25.0 / SQ3), "mg", ("Observed difference (mean of 3, s_p = 25 mg)", "Gözlenen fark (3 ölçüm ortalaması, s_p = 25 mg)")),
    inp("dm_C", R(0.0, 10.0), "mg", ("Comparator eccentricity and magnetic effects", "Komparatör eksantrikliği ve manyetik etkiler")),
    inp("dB", R(0.0, 10.0), "mg", ("Air buoyancy", "Hava kaldırma kuvveti"))],
   {"gum": {"estimate": chk(25.0, 1e-9), "standard_uncertainty": chk(29.2, 0.1), "expanded_uncertainty": chk(58.0, 0.6)},
    "reference": "EA-4/02 M:2022 S2.10–S2.11 (u = 29.2 mg, U = 58 mg)."},
   notes=[("Finding: the guide combines contributions rounded to three significant figures (14.4 for 14.43 mg), giving "
           "29.2 mg and U = 58 mg; unrounded, u = 29.26 mg and U = 58.5 mg (59 mg when rounded to two digits).",
           "Bulgu: rehber üç anlamlı basamağa yuvarlanmış katkıları (14,43 yerine 14,4 mg) birleştirerek 29,2 mg ve "
           "U = 58 mg bulur; yuvarlanmamış değerlerle u = 29,26 mg ve U = 58,5 mg'dir (iki basamakta 59 mg).")])

r_obs = [1.0000104, 1.0000107, 1.0000106, 1.0000103, 1.0000105]
ea("ea402-s3-resistor-10kohm", "electrical", ("Calibration of a 10 kΩ standard resistor", "10 kΩ'luk standart direncin kalibrasyonu"),
   ("EA-4/02 S3: four-terminal resistor calibrated by substitution against a standard, using a DMM as a ratio device.",
    "EA-4/02 S3: dört uçlu direncin, oran cihazı olarak sayısal multimetre kullanılarak standartla yer değiştirme yöntemiyle kalibrasyonu."),
   [("R_X = (R_S + δR_D + δR_TS) · r_C · r − δR_TX (values in mΩ relative to 10 kΩ).",
     "R_X = (R_S + δR_D + δR_TS) · r_C · r − δR_TX (değerler 10 kΩ'a göre mΩ)."),
    ("The ratio r is the mean of five observations: a Type A evaluation (s/√5 = 0.071 × 10⁻⁶, 4 degrees of freedom).",
     "r oranı beş gözlemin ortalamasıdır: A tipi değerlendirme (s/√5 = 0,071 × 10⁻⁶, 4 serbestlik derecesi)."),
    ("The drift of the standard (+20 ± 10 mΩ) and the temperature effects are rectangular; the ratio correction r_C "
     "is triangular (±1 × 10⁻⁶).",
     "Standardın sürüklenmesi (+20 ± 10 mΩ) ve sıcaklık etkileri dikdörtgen; oran düzeltmesi r_C üçgen (±1 × 10⁻⁶) dağılımlıdır."),
    ("R_X = (10 000.178 ± 0.017) Ω (k = 2).", "R_X = (10 000,178 ± 0,017) Ω (k = 2).")],
   "S3", out("R_X", "(R_S + dR_D + dR_TS) * r_C * r - dR_TX", "mΩ", ("Resistance of the unknown (mΩ, offset 10 kΩ = 10⁷ mΩ)", "Bilinmeyen direnç (mΩ)")),
   [inp("R_S", NU(10000053.0, 5.0, 2), "mΩ", ("Standard resistor (certificate)", "Standart direnç (sertifika)")),
    inp("dR_D", R(20.0, 10.0), "mΩ", ("Drift since last calibration", "Son kalibrasyondan beri sürüklenme")),
    inp("dR_TS", R(0.0, 2.75), "mΩ", ("Temperature effect on the standard", "Standarda sıcaklık etkisi")),
    inp("r_C", TRI(1.0, 1.0e-6), None, ("Correction of the ratio (DMM)", "Oran düzeltmesi (multimetre)")),
    inp("r", TA(r_obs), None, ("Observed ratio R_iX/R_iS (5 readings)", "Gözlenen oran R_iX/R_iS (5 okuma)")),
    inp("dR_TX", R(0.0, 5.5), "mΩ", ("Temperature effect on the unknown", "Bilinmeyene sıcaklık etkisi"))],
   {"gum": {"estimate": chk(10000178.0, 0.5), "standard_uncertainty": chk(8.33, 0.005), "expanded_uncertainty": chk(17.0, 0.5)},
    "reference": "EA-4/02 M:2022 S3.10–S3.11 (u = 8.33 mΩ, U = 0.017 Ω)."},
   tags=["type-a"])

ea("ea402-s4-gauge-block-50mm", "length", ("Calibration of a 50 mm gauge block", "50 mm'lik mastar bloğunun kalibrasyonu"),
   ("EA-4/02 S4: grade 0 steel gauge block compared with a calibrated standard; a product of two zero-mean quantities needs second-order terms.",
    "EA-4/02 S4: 0 sınıfı çelik mastar bloğunun kalibre edilmiş standartla karşılaştırılması; iki sıfır ortalamalı büyüklüğün çarpımı ikinci mertebe terim gerektirir."),
   [("l_X = l_S + δl_D + δl + δl_C − L(ᾱ·δt + δα·θ) − δl_V, in nm relative to 50 mm (L = 50 mm).",
     "l_X = l_S + δl_D + δl + δl_C − L(ᾱ·δt + δα·θ) − δl_V, 50 mm'ye göre nm cinsinden (L = 50 mm)."),
    ("δα (difference of expansion coefficients) and θ (deviation from 20 °C) both have zero expectation, so their "
     "product has zero first-order sensitivity: its contribution L·u(δα)·u(θ) = 11.8 nm appears only through the "
     "second-order terms (S4.13), which the project enables.",
     "δα (genleşme katsayıları farkı) ve θ (20 °C'den sapma) beklenen değerleri sıfırdır; çarpımlarının birinci "
     "mertebe duyarlılığı sıfırdır: katkısı L·u(δα)·u(θ) = 11,8 nm yalnızca ikinci mertebe terimlerle ortaya çıkar "
     "(S4.13); proje bunları etkinleştirir."),
    ("l_X = 49.999 926 mm ± 69 nm (k = 2).", "l_X = 49,999 926 mm ± 69 nm (k = 2).")],
   "S4", out("l_X", "l_S + dl_D + dl + dl_C - 50000000*(alpha*dt + d_alpha*theta) - dl_V", "nm", ("Length of the gauge block", "Mastar bloğunun boyu")),
   [inp("l_S", N(50000020.0, 15.0), "nm", ("Standard (certificate: 50.000 02 mm, U = 30 nm)", "Standart (sertifika: 50,000 02 mm, U = 30 nm)")),
    inp("dl_D", TRI(0.0, 30.0), "nm", ("Drift of the standard", "Standardın sürüklenmesi")),
    inp("dl", N(-94.0, 12.0 / math.sqrt(5)), "nm", ("Observed difference (mean of 5, s_p = 12 nm)", "Gözlenen fark (5 ölçüm ortalaması, s_p = 12 nm)")),
    inp("dl_C", R(0.0, 32.0), "nm", ("Comparator linearity and offset", "Komparatörün doğrusallık ve sıfır hatası")),
    inp("alpha", C(11.5e-6), "1/K", ("Mean expansion coefficient", "Ortalama genleşme katsayısı")),
    inp("dt", R(0.0, 0.05), "K", ("Temperature difference of the blocks", "Bloklar arası sıcaklık farkı")),
    inp("d_alpha", TRI(0.0, 2e-6), "1/K", ("Difference of expansion coefficients", "Genleşme katsayıları farkı")),
    inp("theta", R(0.0, 0.5), "K", ("Deviation of the mean temperature from 20 °C", "Ortalama sıcaklığın 20 °C'den sapması")),
    inp("dl_V", R(0.0, 6.7), "nm", ("Non-central contact", "Merkez dışı temas"))],
   {"gum": {"estimate": chk(49999926.0, 0.5), "standard_uncertainty": chk(34.3, 0.05), "expanded_uncertainty": chk(69.0, 0.5)},
    "reference": "EA-4/02 M:2022 S4.12–S4.14 (u = 34.3 nm, U = 69 nm)."},
   gum={"coverage_factor": 2, "higher_order": True}, tags=["higher-order", "dimensional"])

cs, cs0, cx, cx0 = 0.077, 0.189, 0.026, 0.039
ea("ea402-s5-thermocouple-furnace", "temperature", ("Thermocouple calibration: furnace temperature", "Termokupl kalibrasyonu: fırın sıcaklığı"),
   ("EA-4/02 S5 (first budget): temperature of a furnace at 1000 °C measured with a type N reference thermocouple.",
    "EA-4/02 S5 (birinci bütçe): 1000 °C'deki fırının sıcaklığının N tipi referans termokupl ile ölçülmesi."),
   [("t_X = t_S(V_iS + δV_iS1 + δV_iS2 + δV_R − δt_0S/C_S0) + δt_S + δt_D + δt_F, linearised with the sensitivity "
     "C_S = 0.077 °C/µV of the reference thermocouple and C_S0 = 0.189 °C/µV at the reference junction.",
     "t_X = t_S(V_iS + δV_iS1 + δV_iS2 + δV_R − δt_0S/C_S0) + δt_S + δt_D + δt_F; referans termokuplun duyarlılığı "
     "C_S = 0,077 °C/µV ve referans eklemde C_S0 = 0,189 °C/µV ile doğrusallaştırılmıştır."),
    ("The non-uniformity of the furnace temperature (±1 °C, rectangular) dominates: u(t_X) = 0.641 °C.",
     "Fırın sıcaklığının homojen olmaması (±1 °C, dikdörtgen) baskındır: u(t_X) = 0,641 °C.")],
   "S5", out("t_X", f"t_S + {cs}*(V_iS1 + V_iS2 + V_R) - {cs}/{cs0}*t_0S + dt_S + dt_D + dt_F", "°C", ("Temperature of the furnace", "Fırının sıcaklığı")),
   [inp("t_S", N(1000.5, 0.10), "°C", ("Temperature from the reference thermocouple", "Referans termokupldan sıcaklık")),
    inp("V_iS1", NU(0.0, 2.0, 2), "µV", ("Voltmeter calibration", "Voltmetre kalibrasyonu")),
    inp("V_iS2", R(0.0, 0.5), "µV", ("Voltmeter resolution", "Voltmetre çözünürlüğü")),
    inp("V_R", R(0.0, 2.0), "µV", ("Contact parasitic voltages", "Kontak parazitik gerilimleri")),
    inp("t_0S", R(0.0, 0.1), "°C", ("Reference temperature of the cold junction", "Soğuk eklemin referans sıcaklığı")),
    inp("dt_S", NU(0.0, 0.3, 2), "°C", ("Calibration of the reference thermocouple", "Referans termokupl kalibrasyonu")),
    inp("dt_D", R(0.0, 0.3), "°C", ("Drift of the reference thermocouple", "Referans termokuplun sürüklenmesi")),
    inp("dt_F", R(0.0, 1.0), "°C", ("Non-uniformity of the furnace", "Fırın sıcaklığının homojen olmaması"))],
   {"gum": {"estimate": chk(1000.5, 1e-9), "standard_uncertainty": chk(0.641, 0.0005), "expanded_uncertainty": chk(1.3, 0.05)},
    "reference": "EA-4/02 M:2022 S5.9 (u(t_X) = 0.641 °C)."}, tags=["thermocouple"])

ea("ea402-s5-thermocouple-emf", "temperature", ("Thermocouple calibration: emf of the unknown", "Termokupl kalibrasyonu: bilinmeyenin emk'sı"),
   ("EA-4/02 S5 (second budget): emf of the type N thermocouple under calibration at 1000 °C.",
    "EA-4/02 S5 (ikinci bütçe): kalibre edilen N tipi termokuplun 1000 °C'deki emk'sı."),
   [("V_X = V_iX + δV_iX1 + δV_iX2 + δV_R + δV_LX + Δt/C_X − δt_0X/C_X0, where Δt = t − t_X is the difference between "
     "the calibration point (1000 °C) and the furnace temperature of the first budget, with u = 0.641 °C.",
     "V_X = V_iX + δV_iX1 + δV_iX2 + δV_R + δV_LX + Δt/C_X − δt_0X/C_X0; burada Δt = t − t_X, kalibrasyon noktası "
     "(1000 °C) ile birinci bütçedeki fırın sıcaklığı arasındaki farktır ve u = 0,641 °C'dir."),
    ("The furnace temperature dominates (24.65 µV of 25.0 µV); V_X = (36 229 ± 50) µV (k = 2).",
     "Fırın sıcaklığı baskındır (25,0 µV'nin 24,65 µV'u); V_X = (36 229 ± 50) µV (k = 2).")],
   "S5", out("V_X", f"V_iX + V_iX1 + V_iX2 + V_R + V_LX + dt/{cx} - t_0X/{cx0}", "µV", ("Emf of the thermocouple at 1000 °C", "Termokuplun 1000 °C'deki emk'sı")),
   [inp("V_iX", N(36248.0, 1.6), "µV", ("Mean voltmeter reading", "Ortalama voltmetre okuması")),
    inp("V_iX1", N(0.0, 1.0), "µV", ("Voltmeter calibration", "Voltmetre kalibrasyonu")),
    inp("V_iX2", R(0.0, 0.5), "µV", ("Voltmeter resolution", "Voltmetre çözünürlüğü")),
    inp("V_R", R(0.0, 2.0), "µV", ("Contact parasitic voltages", "Kontak parazitik gerilimleri")),
    inp("V_LX", R(0.0, 5.0), "µV", ("Extension leads", "Uzatma kabloları")),
    inp("dt", N(-0.5, 0.641), "°C", ("Calibration point minus furnace temperature", "Kalibrasyon noktası eksi fırın sıcaklığı")),
    inp("t_0X", R(0.0, 0.1), "°C", ("Reference junction temperature", "Referans eklem sıcaklığı"))],
   {"gum": {"estimate": chk(36229.0, 0.5), "standard_uncertainty": chk(25.0, 0.05), "expanded_uncertainty": chk(50.0, 0.5)},
    "reference": "EA-4/02 M:2022 S5.10–S5.12 (u(V_X) = 25.0 µV, U = 50 µV)."},
   notes=[("Finding: the printed contribution of δt (24.5 µV) is inconsistent with 0.641 °C × 38.46 µV/°C = 24.65 µV; the "
           "printed total 25.0 µV follows from 24.65. The estimate of Δt is −0.5 °C (printed 0.5 °C), giving "
           "V_X = 36 248 − 19.2 = 36 229 µV as stated.",
           "Bulgu: δt'nin basılı katkısı (24,5 µV) 0,641 °C × 38,46 µV/°C = 24,65 µV ile tutarsızdır; basılı toplam "
           "25,0 µV, 24,65 değerinden elde edilir. Δt'nin tahmini −0,5 °C'dir (basılı 0,5 °C); bu, belirtilen "
           "V_X = 36 248 − 19,2 = 36 229 µV sonucunu verir.")],
   tags=["thermocouple"])

obs = [(1.0001, 0.9924, 1.0001, 0.9698), (1.0000, 0.9942, 1.0000, 0.9615), (0.9999, 0.9953, 1.0001, 0.9792)]
p_obs = [round(sr * xc / (sc * xr), 12) for sr, sc, xr, xc in obs]
ea("ea402-s6-power-sensor-18ghz", "electrical", ("Calibration of a power sensor at 18 GHz", "Bir güç sensörünün 18 GHz'de kalibrasyonu"),
   ("EA-4/02 S6: calibration factor of a thermistor power sensor against a standard sensor; mismatch terms are U-shaped.",
    "EA-4/02 S6: bir termistör güç sensörünün kalibrasyon faktörünün standart sensöre karşı belirlenmesi; uyumsuzluk terimleri U biçimlidir."),
   [("K_X = (K_S + δK_D) · (M_Sr·M_Xc)/(M_Sc·M_Xr) · p_Cr · p_Cc · p, where p is the observed power ratio.",
     "K_X = (K_S + δK_D) · (M_Sr·M_Xc)/(M_Sc·M_Xr) · p_Cr · p_Cc · p; p gözlenen güç oranıdır."),
    ("Mismatch factors have limits 1 ± 2|Γ_G||Γ| and an arc sine (U-shaped) distribution because the phase is unknown "
     "(u = a/√2).",
     "Uyumsuzluk çarpanlarının sınırları 1 ± 2|Γ_G||Γ|'dır; faz bilinmediği için ark sinüs (U biçimli) dağılım "
     "kullanılır (u = a/√2)."),
    ("p is a Type A estimate from three repeated measurements (2 degrees of freedom), which gives ν_eff ≈ 310.",
     "p, üç tekrarlı ölçümden A tipi tahmindir (2 serbestlik derecesi); bu ν_eff ≈ 310 verir."),
    ("K_X = 0.933 ± 0.032 (k = 2).", "K_X = 0,933 ± 0,032 (k = 2).")],
   "S6", out("K_X", "(K_S + dK_D) * M_Sr * M_Xc / (M_Sc * M_Xr) * p_Cr * p_Cc * p", None, ("Calibration factor of the sensor", "Sensörün kalibrasyon faktörü")),
   [inp("K_S", NU(0.957, 0.011, 2), None, ("Calibration factor of the standard", "Standardın kalibrasyon faktörü")),
    inp("dK_D", R(-0.001, 0.002), None, ("Drift of the standard", "Standardın sürüklenmesi")),
    inp("M_Sr", ARC(1.0, 2 * 0.02 * 0.02), None, ("Mismatch, standard, reference port", "Uyumsuzluk, standart, referans kapısı")),
    inp("M_Sc", ARC(1.0, 2 * 0.07 * 0.10), None, ("Mismatch, standard, 18 GHz", "Uyumsuzluk, standart, 18 GHz")),
    inp("M_Xr", ARC(1.0, 2 * 0.02 * 0.02), None, ("Mismatch, unknown, reference port", "Uyumsuzluk, bilinmeyen, referans kapısı")),
    inp("M_Xc", ARC(1.0, 2 * 0.07 * 0.12), None, ("Mismatch, unknown, 18 GHz", "Uyumsuzluk, bilinmeyen, 18 GHz")),
    inp("p_Cr", N(1.0, 0.00142), None, ("Power meter, reference ratio", "Güç ölçer, referans oranı")),
    inp("p_Cc", N(1.0, 0.000142), None, ("Power meter, 18 GHz ratio", "Güç ölçer, 18 GHz oranı")),
    inp("p", TA(p_obs), None, ("Observed power ratio (3 measurements)", "Gözlenen güç oranı (3 ölçüm)"))],
   {"gum": {"estimate": chk(0.933, 0.0005), "standard_uncertainty": chk(0.01623, 0.0001),
            "expanded_uncertainty": chk(0.032, 0.0005), "effective_dof": chk(310.0, 15.0)},
    "reference": "EA-4/02 M:2022 S6.12–S6.13 (u = 0.0162, U = 0.032, ν_eff ≈ 310)."},
   notes=[("The heading of the example says 19 GHz; the text and the result refer to 18 GHz.",
           "Örneğin başlığında 19 GHz yazmaktadır; metin ve sonuç 18 GHz'e aittir."),
          ("With three observations the t distribution for p has ν = 2 and no finite variance; the Monte Carlo standard "
           "deviation is therefore unstable, while the coverage interval remains well defined (JCGM 101 6.4.9).",
           "Üç gözlemle p için t dağılımı ν = 2'dir ve sonlu varyansı yoktur; bu nedenle Monte Carlo standart sapması "
           "kararsızdır, kapsam aralığı ise iyi tanımlıdır (JCGM 101 6.4.9).")],
   tags=["type-a", "u-shaped", "rf"])

gs = gl = 0.03
s11a, s11b, s22a, s22b, s21a, s21b = 0.05, 0.09, 0.01, 0.01, 0.95, 0.031
u_lm = 8.686 / math.sqrt(2) * math.sqrt(gs * gs * (s11a ** 2 + s11b ** 2) + gl * gl * (s22a ** 2 + s22b ** 2)
                                        + gs * gs * gl * gl * (s21a ** 4 + s21b ** 4))
ea("ea402-s7-step-attenuator-30db", "electrical", ("Calibration of a coaxial step attenuator (30 dB)", "Koaksiyel kademeli zayıflatıcının kalibrasyonu (30 dB)"),
   ("EA-4/02 S7: 30 dB step of a coaxial attenuator at 10 GHz, with mismatch from S-parameters.",
    "EA-4/02 S7: koaksiyel zayıflatıcının 10 GHz'de 30 dB kademesi; uyumsuzluk S parametrelerinden."),
   [("L_X = L_S + δL_S + δL_D + δL_M + δL_K + δL_ib − δL_ia + δL_0b − δL_0a (dB).",
     "L_X = L_S + δL_S + δL_D + δL_M + δL_K + δL_ib − δL_ia + δL_0b − δL_0a (dB)."),
    ("L_S is the mean of four observations (Type A, 3 degrees of freedom); the mismatch term is computed from the "
     "reflection coefficients of source, load and attenuator (Eq. S7.2): u(δL_M) = 0.0198 dB.",
     "L_S dört gözlemin ortalamasıdır (A tipi, 3 serbestlik derecesi); uyumsuzluk terimi kaynak, yük ve zayıflatıcının "
     "yansıma katsayılarından hesaplanır (Eş. S7.2): u(δL_M) = 0,0198 dB."),
    ("L_X = (30.043 ± 0.045) dB (k = 2).", "L_X = (30,043 ± 0,045) dB (k = 2).")],
   "S7", out("L_X", "L_S + dL_S + dL_D + dL_M + dL_K + dL_ib - dL_ia + dL_0b - dL_0a", "dB", ("Attenuation of the 30 dB step", "30 dB kademesinin zayıflatması")),
   [inp("L_S", TA([30.033, 30.058, 30.018, 30.052]), "dB", ("Observed attenuation (4 readings)", "Gözlenen zayıflatma (4 okuma)")),
    inp("dL_S", NU(0.003, 0.005, 2), "dB", ("Calibration of the standard", "Standardın kalibrasyonu")),
    inp("dL_D", R(0.0, 0.002), "dB", ("Drift of the standard", "Standardın sürüklenmesi")),
    inp("dL_M", N(0.0, u_lm), "dB", ("Mismatch (Eq. S7.2)", "Uyumsuzluk (Eş. S7.2)")),
    inp("dL_K", R(0.0, 0.003), "dB", ("Leakage", "Kaçak")),
    inp("dL_ia", R(0.0, 0.0005), "dB", ("Resolution, reference setting", "Çözünürlük, referans ayarı")),
    inp("dL_ib", R(0.0, 0.0005), "dB", ("Resolution, attenuation setting", "Çözünürlük, zayıflatma ayarı")),
    inp("dL_0a", N(0.0, 0.002), "dB", ("Noise, reference setting", "Gürültü, referans ayarı")),
    inp("dL_0b", N(0.0, 0.002), "dB", ("Noise, attenuation setting", "Gürültü, zayıflatma ayarı"))],
   {"gum": {"estimate": chk(30.043, 0.0005), "standard_uncertainty": chk(0.0223, 0.00015), "expanded_uncertainty": chk(0.045, 0.0006)},
    "reference": "EA-4/02 M:2022 S7.12–S7.13 (u = 0.0223 dB, U = 0.045 dB)."},
   notes=[("Finding: the budget prints the contribution of δL_ib as 0.0019 dB although its standard uncertainty is "
           "0.0003 dB; the printed total 0.0223 dB includes the 0.0019 value (0.0222 dB without it). The reported "
           "result is unchanged.",
           "Bulgu: bütçe δL_ib'nin katkısını 0,0019 dB olarak verir, oysa standart belirsizliği 0,0003 dB'dir; basılı "
           "toplam 0,0223 dB bu 0,0019 değerini içerir (onsuz 0,0222 dB). Raporlanan sonuç değişmez.")],
   tags=["type-a", "rf"])

ea("ea402-s9-dmm-100v", "electrical", ("Calibration of a hand-held digital multimeter at 100 V", "El tipi sayısal multimetrenin 100 V'ta kalibrasyonu"),
   ("EA-4/02 S9: error of indication of a 3½-digit DMM; the resolution term dominates and the output is nearly rectangular.",
    "EA-4/02 S9: 3½ basamaklı multimetrenin gösterge hatası; çözünürlük terimi baskındır ve çıktı neredeyse dikdörtgen dağılımlıdır."),
   [("E_X = V_iX − V_S + δV_iX − δV_S (V); the resolution of 0.1 V gives ±0.05 V (rectangular), u = 0.029 V.",
     "E_X = V_iX − V_S + δV_iX − δV_S (V); 0,1 V çözünürlük ±0,05 V (dikdörtgen), u = 0,029 V verir."),
    ("Because one rectangular term dominates (ratio of the others 0.22 ≤ 0.3), EA-4/02 uses k = 0.95·√3 = 1.65 for "
     "95 % instead of k = 2: U = 0.05 V (S9.14).",
     "Tek bir dikdörtgen terim baskın olduğundan (diğerlerinin oranı 0,22 ≤ 0,3) EA-4/02 %95 için k = 2 yerine "
     "k = 0,95·√3 = 1,65 kullanır: U = 0,05 V (S9.14)."),
    ("The Monte Carlo 95 % interval shows directly that ±2u = ±0.06 V overstates the coverage; no rule of thumb is needed.",
     "Monte Carlo %95 aralığı ±2u = ±0,06 V'un kapsamı fazla gösterdiğini doğrudan ortaya koyar; pratik bir kurala gerek kalmaz.")],
   "S9", out("E_X", "V_iX - V_S + dV_iX - dV_S", "V", ("Error of indication", "Gösterge hatası")),
   [inp("V_iX", C(100.1), "V", ("Indication of the DMM", "Multimetrenin göstergesi")),
    inp("V_S", NU(100.0, 0.002, 2), "V", ("Voltage of the calibrator", "Kalibratörün gerilimi")),
    inp("dV_iX", R(0.0, 0.05), "V", ("Resolution of the DMM", "Multimetrenin çözünürlüğü")),
    inp("dV_S", R(0.0, 0.011), "V", ("Calibrator: specification and drift", "Kalibratör: şartname ve sürüklenme"))],
   {"gum": {"estimate": chk(0.1, 1e-9), "standard_uncertainty": chk(0.030, 0.0005)},
    "reference": "EA-4/02 M:2022 S9.12–S9.14 (u = 0.030 V)."},
   mcm={"trials": 1000000, "seed": 9, "coverage_probability": 0.95}, reporting={"significant_digits": 1},
   tags=["dominant-rectangular"])

ea("ea402-s10-vernier-calliper", "length", ("Calibration of a vernier calliper at 150 mm", "Kumpasın 150 mm'de kalibrasyonu"),
   ("EA-4/02 S10: error of a vernier calliper measured with a gauge block; two rectangular terms dominate (trapezoidal output).",
    "EA-4/02 S10: kumpas hatasının mastar bloğuyla ölçülmesi; iki dikdörtgen terim baskındır (yamuk çıktı)."),
   [("E_X = l_iX − l_S + L·ᾱ·Δt + δl_iX + δl_M (µm).", "E_X = l_iX − l_S + L·ᾱ·Δt + δl_iX + δl_M (µm)."),
    ("The resolution (±25 µm) and the mechanical effects (±50 µm) are rectangular; their convolution is trapezoidal "
     "with β = (50 − 25)/(50 + 25) = 0.33, giving k = 1.83 for 95 % (Eq. S10.9) and U ≈ 60 µm.",
     "Çözünürlük (±25 µm) ve mekanik etkiler (±50 µm) dikdörtgen dağılımlıdır; konvolüsyonları β = (50 − 25)/(50 + 25) = 0,33 "
     "olan yamuk dağılımdır ve %95 için k = 1,83 (Eş. S10.9), U ≈ 60 µm verir.")],
   "S10", out("E_X", "l_iX - l_S + 150000*11.5e-6*dt + dl_iX + dl_M", "µm", ("Error of indication", "Gösterge hatası")),
   [inp("l_iX", C(150100.0), "µm", ("Indication", "Gösterge")),
    inp("l_S", R(150000.0, 0.8), "µm", ("Length of the gauge block", "Mastar bloğunun boyu")),
    inp("dt", R(0.0, 2.0), "K", ("Temperature difference calliper–block", "Kumpas–blok sıcaklık farkı")),
    inp("dl_iX", R(0.0, 25.0), "µm", ("Resolution of the scale", "Skala çözünürlüğü")),
    inp("dl_M", R(0.0, 50.0), "µm", ("Mechanical effects (force, flatness, parallelism)", "Mekanik etkiler (kuvvet, düzlemsellik, paralellik)"))],
   {"gum": {"estimate": chk(100.0, 1e-9), "standard_uncertainty": chk(32.0, 0.5)},
    "reference": "EA-4/02 M:2022 S10.10–S10.12 (u = 32 µm)."},
   mcm={"trials": 1000000, "seed": 10, "coverage_probability": 0.95}, tags=["dominant-rectangular", "trapezoidal"])

ea("ea402-s11-block-calibrator-180c", "temperature", ("Calibration of a temperature block calibrator at 180 °C", "Sıcaklık blok kalibratörünün 180 °C'de kalibrasyonu"),
   ("EA-4/02 S11: temperature of the block measured with a calibrated platinum resistance thermometer (values in mK).",
    "EA-4/02 S11: bloğun sıcaklığının kalibre edilmiş platin direnç termometresiyle ölçülmesi (değerler mK)."),
   [("t_X = t_S + δt_S + δt_D − δt_iX + δt_R + δt_A + δt_H + δt_V (mK).",
     "t_X = t_S + δt_S + δt_D − δt_iX + δt_R + δt_A + δt_H + δt_V (mK)."),
    ("Axial inhomogeneity (±250 mK) and radial differences (±100 mK) dominate: trapezoid with β = 0.43; EA-4/02 "
     "quotes k = 1.81 (Eq. S10.9 gives 1.80); t_X = (180.1 ± 0.3) °C.",
     "Eksenel homojensizlik (±250 mK) ve radyal farklar (±100 mK) baskındır: β = 0,43 olan yamuk; EA-4/02 k = 1,81 verir "
     "(Eş. S10.9 1,80 verir); t_X = (180,1 ± 0,3) °C.")],
   "S11", out("t_X", "t_S + dt_S + dt_D - dt_iX + dt_R + dt_A + dt_H + dt_V", "mK", ("Temperature of the block (in mK)", "Bloğun sıcaklığı (mK)")),
   [inp("t_S", NU(180100.0, 30.0, 2), "mK", ("Reference thermometer reading", "Referans termometre okuması")),
    inp("dt_S", N(0.0, 10.0), "mK", ("Calibration of the reference thermometer", "Referans termometrenin kalibrasyonu")),
    inp("dt_D", R(0.0, 40.0), "mK", ("Drift of the reference thermometer", "Referans termometrenin sürüklenmesi")),
    inp("dt_iX", R(0.0, 50.0), "mK", ("Resolution of the calibrator display", "Kalibratör ekranının çözünürlüğü")),
    inp("dt_R", R(0.0, 100.0), "mK", ("Radial temperature differences", "Radyal sıcaklık farkları")),
    inp("dt_A", R(0.0, 250.0), "mK", ("Axial inhomogeneity", "Eksenel homojensizlik")),
    inp("dt_H", R(0.0, 50.0), "mK", ("Hysteresis", "Histerezis")),
    inp("dt_V", R(0.0, 30.0), "mK", ("Temperature variation during measurement", "Ölçüm sırasında sıcaklık değişimi"))],
   {"gum": {"estimate": chk(180100.0, 1e-9), "standard_uncertainty": chk(164.0, 0.5)},
    "reference": "EA-4/02 M:2022 S11.9–S11.11 (u = 164 mK)."},
   mcm={"trials": 1000000, "seed": 11, "coverage_probability": 0.95},
   notes=[("Finding: the guide quotes k = 1.81 for β = 0.43; Eq. (S10.9) gives 1.80. The reported (180.1 ± 0.3) °C is "
           "unchanged.",
           "Bulgu: rehber β = 0,43 için k = 1,81 verir; Eş. (S10.9) 1,80 verir. Raporlanan (180,1 ± 0,3) °C değişmez.")],
   tags=["dominant-rectangular", "trapezoidal"])

VX = 200.0 * (1 + 51e-6 * (15 - 20)) * (1 + 0.15e-3 * (16 - 15)) * (1 - 0.46e-6 * 500)
ea("ea402-s12-water-meter-volume", "flow", ("Water meter calibration: reference volume", "Su sayacı kalibrasyonu: referans hacim"),
   ("EA-4/02 S12 (first budget): volume passed through a household water meter, measured in a volumetric standard at 2500 L/h.",
    "EA-4/02 S12 (birinci bütçe): 2500 L/h'te bir konut tipi su sayacından geçen hacmin hacimsel standartta ölçülmesi."),
   [("V_X = V_S[1 + β_S(t_S − 20)][1 + γ_W(t_X − t_S)][1 − κ_W p_X], correcting the standard's reading to the "
     "temperature and pressure of the water at the meter.",
     "V_X = V_S[1 + β_S(t_S − 20)][1 + γ_W(t_X − t_S)][1 − κ_W p_X]; standardın okuması sayaçtaki suyun sıcaklık ve "
     "basıncına düzeltilir."),
    ("The temperature of the water at the meter (±2 °C) gives the largest correction uncertainty: c(t_X) = 0.030 L/K.",
     "Sayaçtaki su sıcaklığı (±2 °C) en büyük düzeltme belirsizliğini verir: c(t_X) = 0,030 L/K."),
    ("V_X = 199.93 L with u = 0.109 L.", "V_X = 199,93 L, u = 0,109 L.")],
   "S12", out("V_X", "(V_iS + dV_iS)*(1 + beta_S*(t_S - 20))*(1 + gamma_W*(t_X - t_S))*(1 - kappa_W*p_X)", "L", ("Volume at the meter", "Sayaçtaki hacim")),
   [inp("V_iS", NU(200.0, 0.2, 2), "L", ("Indication of the volumetric standard", "Hacimsel standardın göstergesi")),
    inp("dV_iS", R(0.0, 0.02), "L", ("Reading of the standard", "Standardın okunması")),
    inp("beta_S", R(51e-6, 0.5e-6), "1/K", ("Cubic expansion coefficient of the standard", "Standardın hacimsel genleşme katsayısı")),
    inp("t_S", R(15.0, 2.0), "°C", ("Water temperature in the standard", "Standarttaki su sıcaklığı")),
    inp("gamma_W", R(0.15e-3, 0.005e-3), "1/K", ("Expansion coefficient of water", "Suyun genleşme katsayısı")),
    inp("t_X", R(16.0, 2.0), "°C", ("Water temperature at the meter", "Sayaçtaki su sıcaklığı")),
    inp("kappa_W", R(0.46e-6, 0.005e-6), "1/kPa", ("Compressibility of water", "Suyun sıkıştırılabilirliği")),
    inp("p_X", R(500.0, 50.0), "kPa", ("Gauge pressure at the meter", "Sayaçtaki gösterge basıncı"))],
   {"gum": {"estimate": chk(199.93, 0.005), "standard_uncertainty": chk(0.109, 0.0005)},
    "reference": "EA-4/02 M:2022 S12.9 (V_X = 199.93 L, u = 0.109 L)."}, tags=["volume"])

u_rect = 0.1 / SQ3
u_ex = math.sqrt(2 * (u_rect / VX) ** 2 + (200.0 / VX ** 2 * 0.1090) ** 2)
ea("ea402-s12-water-meter-error", "flow", ("Water meter calibration: relative error of one run", "Su sayacı kalibrasyonu: tek çalışmanın bağıl hatası"),
   ("EA-4/02 S12 (second budget): relative error of indication of the meter for a single run.",
    "EA-4/02 S12 (ikinci bütçe): tek bir çalışma için sayacın bağıl gösterge hatası."),
   [("e_X = (V_iX + δV_iX2 − δV_iX1)/V_X − 1: the meter's start and stop readings each carry a resolution term ±0.1 L.",
     "e_X = (V_iX + δV_iX2 − δV_iX1)/V_X − 1: sayacın başlangıç ve bitiş okumalarının her biri ±0,1 L çözünürlük terimi taşır."),
    ("V_X comes from the first budget (199.93 L, u = 0.109 L).", "V_X birinci bütçeden gelir (199,93 L, u = 0,109 L).")],
   "S12", out("e_X", "(V_iX + dV_iX2 - dV_iX1)/V_X - 1", None, ("Relative error of indication", "Bağıl gösterge hatası")),
   [inp("V_iX", C(200.0), "L", ("Indicated volume", "Gösterilen hacim")),
    inp("dV_iX1", R(0.0, 0.1), "L", ("Resolution at start", "Başlangıçta çözünürlük")),
    inp("dV_iX2", R(0.0, 0.1), "L", ("Resolution at stop", "Bitişte çözünürlük")),
    inp("V_X", N(VX, 0.109), "L", ("Volume at the meter (first budget)", "Sayaçtaki hacim (birinci bütçe)"))],
   {"gum": {"estimate": chk(0.0003, 0.00006), "standard_uncertainty": chk(0.00068, 0.000005)},
    "reference": "EA-4/02 M:2022 S12.10 (e_X = 0.0003, u = 0.68 × 10⁻³)."}, tags=["volume"])

runs = [0.0003, 0.0005, 0.0022]
ea("ea402-s12-water-meter-mean-error", "flow", ("Water meter calibration: mean of three runs", "Su sayacı kalibrasyonu: üç çalışmanın ortalaması"),
   ("EA-4/02 S12 (third budget): mean relative error of three runs, with the Welch–Satterthwaite formula and k from the t-distribution.",
    "EA-4/02 S12 (üçüncü bütçe): üç çalışmanın ortalama bağıl hatası; Welch–Satterthwaite formülü ve t dağılımından k."),
   [("e_X,av = ē_X + δe_X: the scatter of the three runs (s = 0.001, u = s/√3, 2 degrees of freedom) is added to the "
     "uncertainty of a single run (0.68 × 10⁻³).",
     "e_X,ort = ē_X + δe_X: üç çalışmanın saçılımı (s = 0,001, u = s/√3, 2 serbestlik derecesi) tek çalışmanın "
     "belirsizliğine (0,68 × 10⁻³) eklenir."),
    ("s = 0.00104, so u = s/√3 = 0.60 × 10⁻³ (EA-4/02 prints s = 0.001); combined u = 0.91 × 10⁻³ with ν_eff = 10, so k = t₀.₉₅₄₅(10) = 2.28 (EA-4/02 Table E.1) and U = 0.002.",
     "s = 0,00104 ve u = s/√3 = 0,60 × 10⁻³ (EA-4/02 s = 0,001 yazar); birleşik u = 0,91 × 10⁻³ ve ν_eff = 10 olduğundan k = t₀,₉₅₄₅(10) = 2,28 (EA-4/02 Tablo E.1) ve U = 0,002.")],
   "S12", out("e_Xav", "e_X + de_X", None, ("Mean relative error of indication", "Ortalama bağıl gösterge hatası")),
   [inp("e_X", N(0.0, u_ex), None, ("Error of a single run (uncertainty from the second budget)", "Tek çalışmanın hatası (belirsizlik ikinci bütçeden)")),
    inp("de_X", TA(runs), None, ("Relative errors of the three runs (Type A)", "Üç çalışmanın bağıl hataları (A tipi)"))],
   {"gum": {"standard_uncertainty": chk(0.00091, 0.00001), "coverage_factor": chk(2.28, 0.005),
            "expanded_uncertainty": chk(0.002, 0.0002)},
    "reference": "EA-4/02 M:2022 S12.11–S12.13 (u = 0.91 × 10⁻³, ν_eff = 10, k = 2.28, U = 0.002)."},
   gum={"coverage_probability": 0.9545}, reporting={"significant_digits": 1}, tags=["welch-satterthwaite"])

ds, dx = 40000, 90000
ea("ea402-s13-ring-gauge-temperature", "length", ("Ring gauge calibration: temperature correction", "Halka mastar kalibrasyonu: sıcaklık düzeltmesi"),
   ("EA-4/02 S13: temperature sub-budget of the ring-gauge comparison; products of zero-mean temperature deviations with expansion coefficients.",
    "EA-4/02 S13: halka mastar karşılaştırmasının sıcaklık alt bütçesi; sıfır ortalamalı sıcaklık sapmalarının genleşme katsayılarıyla çarpımları."),
   [("δl_T = [D_S(α_S − α_R) − D_X(α_X − α_R)]Δt_A + D_S α_S δt_S − D_X α_X δt_X + (D_S − D_X) α_R δt_R (µm), "
     "with D_S = 40 mm and D_X = 90 mm.",
     "δl_T = [D_S(α_S − α_R) − D_X(α_X − α_R)]Δt_A + D_S α_S δt_S − D_X α_X δt_X + (D_S − D_X) α_R δt_R (µm); "
     "D_S = 40 mm ve D_X = 90 mm."),
    ("All temperature deviations have zero expectation, so the first term has zero first-order sensitivity; the "
     "second-order terms give the exact variance of the products: 0.018, 0.053, 0.12, 0.066 µm → u(δl_T) = 0.15 µm.",
     "Tüm sıcaklık sapmalarının beklenen değeri sıfırdır; bu nedenle birinci terimin birinci mertebe duyarlılığı sıfırdır; "
     "ikinci mertebe terimler çarpımların varyansını tam verir: 0,018, 0,053, 0,12, 0,066 µm → u(δl_T) = 0,15 µm.")],
   "S13", out("dl_T", "l_TA + l_TS - l_TX + l_TR", "µm", ("Temperature correction", "Sıcaklık düzeltmesi")),
   [inp("a_S", R(11.5e-6, 1e-6), "1/K", ("Expansion coefficient of the setting ring", "Ayar halkasının genleşme katsayısı")),
    inp("a_X", R(11.5e-6, 1e-6), "1/K", ("Expansion coefficient of the ring gauge", "Halka mastarın genleşme katsayısı")),
    inp("a_R", R(11.5e-6, 1e-6), "1/K", ("Expansion coefficient of the machine scale", "Makine skalasının genleşme katsayısı")),
    inp("dt_A", R(0.0, 0.5), "K", ("Deviation of the mean temperature from 20 °C", "Ortalama sıcaklığın 20 °C'den sapması")),
    inp("dt_S", R(0.0, 0.2), "K", ("Temperature difference, setting ring", "Sıcaklık farkı, ayar halkası")),
    inp("dt_X", R(0.0, 0.2), "K", ("Temperature difference, ring gauge", "Sıcaklık farkı, halka mastar")),
    inp("dt_R", R(0.0, 0.2), "K", ("Temperature difference, scale", "Sıcaklık farkı, skala"))],
   {"gum": {"estimate": chk(0.0, 1e-12), "standard_uncertainty": chk(0.15, 0.005)},
    "reference": "EA-4/02 M:2022 S13.5 (u(δl_T) = 0.15 µm)."},
   intermediates=[mid("l_TA", f"({ds}*(a_S - a_R) - {dx}*(a_X - a_R))*dt_A"), mid("l_TS", f"{ds}*a_S*dt_S"),
                  mid("l_TX", f"{dx}*a_X*dt_X"), mid("l_TR", f"({ds} - {dx})*a_R*dt_R")],
   gum={"coverage_factor": 2, "higher_order": True}, tags=["higher-order", "dimensional"])

obs13 = [49.99935, 49.99911, 49.99972, 49.99954, 49.99996]
m13 = sum(obs13) / 5
s13 = math.sqrt(sum((x - m13) ** 2 for x in obs13) / 4) * 1000 / math.sqrt(5)
u_dl = math.sqrt(s13 ** 2 + 0.25 ** 2)
uc = 20 / SQ3
dlp = 2 * (1 / dx - 1 / ds) * uc * uc
u_dlp = math.sqrt(16 / 5 * (1 / dx ** 2 + 1 / ds ** 2)) * uc * uc
ea("ea402-s13-ring-gauge-90mm", "length", ("Calibration of a 90 mm ring gauge", "90 mm'lik halka mastarın kalibrasyonu"),
   ("EA-4/02 S13: diameter of a ring gauge compared with a 40 mm setting ring on a one-coordinate measuring machine (µm relative to 90 mm).",
    "EA-4/02 S13: halka mastar çapının tek koordinatlı ölçüm makinesinde 40 mm'lik ayar halkasıyla karşılaştırılması (90 mm'ye göre µm)."),
   [("d_X = d_S + Δl + δl_0 + δl_i + δl_T + δl_P + δl_E + δl_A: the measured diameter difference Δl (Type A from 5 "
     "observations, 0.15 µm) and the zeroing of the comparator (0.25 µm) dominates together with the temperature term of the sub-budget (0.15 µm).",
     "d_X = d_S + Δl + δl_0 + δl_i + δl_T + δl_P + δl_E + δl_A: ölçülen çap farkı Δl (5 gözlemden A tipi, 0,15 µm) ve "
     "komparatörün sıfırlanması (0,25 µm) ile alt bütçedeki sıcaklık terimi (0,15 µm) baskındır."),
    ("The coaxiality error is a second-order (cosine) effect with a non-zero mean: δl_P = 2(1/D_X − 1/D_S)u²(Δc) = "
     "−0.004 µm, u = 0.0065 µm.",
     "Eş eksenlilik hatası sıfırdan farklı ortalamalı ikinci mertebe (kosinüs) etkidir: δl_P = 2(1/D_X − 1/D_S)u²(Δc) = "
     "−0,004 µm, u = 0,0065 µm."),
    ("u(d_X) = 0.41 µm; U = 0.82 µm, which EA-4/02 rounds up to 0.9 µm: d_X = (90.000 3 ± 0.000 9) mm. The project "
     "uses one significant digit with rounding up to reproduce this.",
     "u(d_X) = 0,41 µm; U = 0,82 µm, EA-4/02 bunu yukarı yuvarlayarak 0,9 µm verir: d_X = (90,000 3 ± 0,000 9) mm. "
     "Proje bunu yeniden üretmek için tek anlamlı basamak ve yukarı yuvarlama kullanır.")],
   "S13", out("d_X", "d_S + dl + dl_0 + dl_i + dl_T + dl_P + dl_E + dl_A", "µm", ("Diameter deviation from 90 mm", "Çapın 90 mm'den sapması")),
   [inp("d_S", NU(0.7, 0.2, 2), "µm", ("Setting ring diameter deviation (40 mm)", "Ayar halkası çap sapması (40 mm)")),
    inp("dl", TA([round((x - 50) * 1000, 6) for x in obs13]), "µm", ("Observed diameter difference relative to 50 mm (5 readings)", "50 mm'ye göre gözlenen çap farkı (5 okuma)")),
    inp("dl_0", N(0.0, 0.25), "µm", ("Zeroing of the comparator (s_p(0) = 0.25 µm)", "Komparatörün sıfırlanması (s_p(0) = 0,25 µm)")),
    inp("dl_i", R(0.0, 0.375), "µm", ("Machine scale calibration and linearity", "Makine skalasının kalibrasyonu ve doğrusallığı")),
    inp("dl_T", N(0.0, 0.148), "µm", ("Temperature correction (sub-budget)", "Sıcaklık düzeltmesi (alt bütçe)")),
    inp("dl_P", N(round(dlp, 6), round(u_dlp, 6)), "µm", ("Coaxiality (cosine) error", "Eş eksenlilik (kosinüs) hatası")),
    inp("dl_E", R(0.0, 0.03), "µm", ("Elastic deformation", "Elastik deformasyon")),
    inp("dl_A", R(0.0, 0.02), "µm", ("Abbe error", "Abbe hatası"))],
   {"gum": {"estimate": chk(0.7 - 0.464 + round(dlp, 6), 1e-9), "standard_uncertainty": chk(0.411, 0.01),
            "expanded_uncertainty": chk(0.82, 0.02)},
    "reference": "EA-4/02 M:2022 S13.10–S13.12 (u = 0.411 µm, U = 0.82 µm reported as 0.9 µm; the tolerance covers u(Δl) = 0.29 µm here vs 0.30 µm printed); estimate recomputed "
                 "from the observations, 0.7 − 0.464 − 0.0037 µm (printed 90.000 25 mm, see notes)."},
   reporting={"significant_digits": 1, "rounding": "up"},
   notes=[("Finding (affects the reported value): the budget uses Δl = 49.999 55 mm instead of the mean 49.999 536 mm "
           "of the observations and δl_P = +0.004 µm instead of −0.0037 µm, and so obtains d_X = 90.000 25 mm, reported "
           "as 90.000 3 mm. From the data, d_X = 90.000 232 mm, i.e. (90.000 2 ± 0.000 9) mm. The difference, 0.1 µm, is "
           "well inside U.",
           "Bulgu (raporlanan değeri etkiler): bütçe, gözlemlerin ortalaması olan 49,999 536 mm yerine Δl = 49,999 55 mm "
           "ve −0,0037 µm yerine δl_P = +0,004 µm kullanarak d_X = 90,000 25 mm bulur ve 90,000 3 mm olarak raporlar. "
           "Verilerden d_X = 90,000 232 mm, yani (90,000 2 ± 0,000 9) mm elde edilir. 0,1 µm'lik fark U'nun çok içindedir."),
          ("Findings: the mean Δl is 49.999 54 mm but the budget uses 49.999 55; the text quotes s = 0.18 µm for the "
           "standard deviation of a single observation (0.33 µm from the data, 0.15 µm for the mean); √(0.15² + 0.25²) "
           "= 0.29 µm (printed 0.30); the coaxiality correction is −0.004 µm (budget +0.000 004 mm).",
           "Bulgular: Δl ortalaması 49,999 54 mm iken bütçe 49,999 55 kullanır; metin tek gözlemin standart sapması için "
           "s = 0,18 µm verir (veriden 0,33 µm, ortalama için 0,15 µm); √(0,15² + 0,25²) = 0,29 µm'dir (basılı 0,30); "
           "eş eksenlilik düzeltmesi −0,004 µm'dir (bütçede +0,000 004 mm).")],
   tags=["dimensional", "rounding"])
print("ok")
