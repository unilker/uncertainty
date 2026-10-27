"""Writes the EURAMET cg-18 v4.0 Example H4 library projects (run from the repository root)."""
import json
import math
from pathlib import Path

DOC = "EURAMET Calibration Guide No. 18 v4.0 (2015), Guidelines on the Calibration of Non-Automatic Weighing Instruments"
URL = "https://www.euramet.org/publications-media-centre/calibration-guidelines"


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


MC = [0.0, 50.000006, 99.999987, 149.999993, 200.000013, 250.000019, 300.0, 350.000006, 400.00001]
IND = [0.0, 50.000067, 100.0001, 150.000233, 200.000267, 250.0001, 300.0002, 350.000267, 400.0004]
W = {50: (0.015, 0.005), 100: (0.025, 0.010), 200: (0.045, 0.015)}  # u(m_c) = U/2 and u(m_D), mg
COMBO = [[], [50], [100], [100, 50], [200], [200, 50], [200, 100], [200, 100, 50], [200, 200]]
D = 0.1 / math.sqrt(12)


def points():
    out = []
    for j, combo in enumerate(COMBO):
        n = 50.0 * j
        uc = sum(W[c][0] for c in combo)
        ud = sum(W[c][1] for c in combo)
        b = (n * 1000) ** 2 * (1.09 - 1.2) * (1.09 - 1.2 - 2 * (1.045 - 1.2)) * 60.0 ** 2 / 8000.0 ** 4
        ecc = 0.10 * n / (2 * 200 * math.sqrt(3))
        ui = math.sqrt(0.030 ** 2 + D ** 2 + (D ** 2 if j else 0) + ecc ** 2)
        p = {"indication": IND[j], "reference": MC[j], "u_reference": round(math.sqrt(max(uc * uc + ud * ud + b, 0.0)), 6),
             "u_indication": round(ui, 6)}
        out.append(p)
    return out


EXPL = [
    T("Balance 400 g / 0.1 mg, adjusted immediately before calibration; nine test points 0 … 400 g with E2 weights (50 g, 100 g, 2 × 200 g), N = 3 cycles: E_j = I_j − m_c,j.",
      "Terazi 400 g / 0,1 mg, kalibrasyondan hemen önce ayarlanmış; E2 ağırlıklarla (50 g, 100 g, 2 × 200 g) 0 … 400 g arasında dokuz deney noktası, N = 3 çevrim: E_j = I_j − m_c,j."),
    T("u(I_j) from the repeatability s = 0.052 mg/√3 = 0.030 mg (7.1.1-6), the rounding d/√12 at zero and at the load, and the eccentricity 0.10 mg at 200 g (7.1.1-10): 0.042 … 0.077 mg.",
      "u(I_j), tekrarlanabilirlik s = 0,052 mg/√3 = 0,030 mg (7.1.1-6), sıfırda ve yükte yuvarlama d/√12 ve 200 g'da 0,10 mg eksantriklikten (7.1.1-10): 0,042 … 0,077 mg."),
    T("u(m_ref,j) from U(m_c)/2 and the drift of the weights, summed over the weights used, and the buoyancy term of 7.1.2-5b, which is negative here (ρ_a = 1.090, ρ_a1 = 1.045 kg/m³, ρ = (8000 ± 60) kg/m³): 0.014 … 0.077 mg.",
      "u(m_ref,j), U(m_c)/2 ve ağırlıkların sürüklenmesinden (kullanılan ağırlıklar üzerinden toplanarak) ve burada negatif olan 7.1.2-5b kaldırma kuvveti teriminden (ρ_a = 1,090, ρ_a1 = 1,045 kg/m³, ρ = (8000 ± 60) kg/m³): 0,014 … 0,077 mg."),
    T("U(e) = s sᵀ + diag(u²(I_j)) + s_m²·I (C2.2-3a, -3b, -3c); E(I) = a₁I by generalized least squares: â₁ = (XᵀPX)⁻¹XᵀPe, U(â) = (XᵀPX)⁻¹, χ²_obs = vᵀPv with ν = 9 − 1 = 8 (C2.2-6 to -9).",
      "U(e) = s sᵀ + diag(u²(I_j)) + s_m²·I (C2.2-3a, -3b, -3c); genelleştirilmiş en küçük karelerle E(I) = a₁I: â₁ = (XᵀPX)⁻¹XᵀPe, U(â) = (XᵀPX)⁻¹, χ²_obs = vᵀPv, ν = 9 − 1 = 8 (C2.2-6 … -9)."),
]
NOTES = [
    T("Table H4.1 lists e = 0.213, 0.274 and 0.181 mg at 100, 150 and 250 g, and Tables H4.6/H4.8 the indications 100.000 200, 150.000 267 and 250.000 200 g; the budget gives 0.113, 0.240 and 0.081 mg (100.000 100, 150.000 233, 250.000 100 g). The printed a₁, U(â), χ²_obs and residuals follow from the budget values, which the project uses (research log F70).",
      "Tablo H4.1 100, 150 ve 250 g'da e = 0,213, 0,274 ve 0,181 mg, Tablo H4.6/H4.8 ise 100,000 200, 150,000 267 ve 250,000 200 g göstergelerini verir; bütçe 0,113, 0,240 ve 0,081 mg (100,000 100, 150,000 233, 250,000 100 g) verir. Basılı a₁, U(â), χ²_obs ve artıklar bütçe değerlerinden çıkar; proje bunları kullanır (araştırma günlüğü F70)."),
    T("The buoyancy 'variance' of 7.1.2-5b is negative: it removes the part of u(ρ) already contained in the certificate value of the weights, calibrated at ρ_a1. It is a covariance correction, not a variance; u²(m_ref) stays positive.",
      "7.1.2-5b'nin kaldırma kuvveti 'varyansı' negatiftir: ρ_a1'de kalibre edilmiş ağırlıkların sertifika değerinde zaten bulunan u(ρ) payını çıkarır. Bu bir varyans değil kovaryans düzeltmesidir; u²(m_ref) pozitif kalır."),
]


def write(name, sm, title, desc, expl, expected, ref, notes=()):
    pid = f"cg18-{name}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": EXPL + expl, "field": "mass",
                      "sources": [{"document": DOC, "section": "Appendix H4; Appendix C2.2", "url": URL}],
                      "tags": ["euramet-cg18", "balance", "weighing-instrument", "calibration", "error-function", "generalized-least-squares", "validation"],
                      "notes": NOTES + list(notes)},
         "error_function": {"instrument": "Electronic balance, Max 400 g, d = 0.1 mg", "unit": "g", "error_unit": "mg", "error_factor": 1000,
                            "points": points(), "through_zero": True, "s_m": sm},
         "expected": {"error_function": expected, "reference": ref}}
    Path(f"library/mass/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")


desc = T("Error approximation function E = a₁I of a 400 g balance from nine test points with correlated reference masses.",
         "Korelasyonlu referans kütlelerle dokuz deney noktasından 400 g'lık bir terazinin hata yaklaşım fonksiyonu E = a₁I.")
write("h4-400g-error-function", 0.0,
      T("Balance 400 g / 0.1 mg: error approximation function E = a₁I, first fit (EURAMET cg-18 H4)", "Terazi 400 g / 0,1 mg: hata yaklaşım fonksiyonu E = a₁I, ilk uyum (EURAMET cg-18 H4)"),
      desc,
      [T("Without a model term: a₁ = 0.000 83 mg/g, U(â) = 5.109·10⁻⁸ (mg/g)², u(a₁) = 0.000 23 mg/g; χ²_obs = 12.5 > ν = 8: the χ² test C2.2-2a fails.",
         "Model terimi olmadan: a₁ = 0,000 83 mg/g, U(â) = 5,109·10⁻⁸ (mg/g)², u(a₁) = 0,000 23 mg/g; χ²_obs = 12,5 > ν = 8: χ² testi C2.2-2a başarısız."),
       T("The program also finds the smallest model terms: s_m = 0.044 mg for C2.2-2a and 0.22 mg for C2.2-2b; the guide uses the round values 0.05 and 0.25 mg.",
         "Program en küçük model terimlerini de bulur: C2.2-2a için s_m = 0,044 mg ve C2.2-2b için 0,22 mg; kılavuz 0,05 ve 0,25 mg yuvarlak değerlerini kullanır.")],
      {"coefficients": [chk(0.00083, 5e-6)], "u_coefficients": [chk(0.000226, 5e-7)], "chi_squared": chk(12.5, 0.05), "chi_squared_passed": False},
      T("EURAMET cg-18 v4.0 H4.4, s_m = 0: a₁ = 0.000 83 mg/g, U(â) = 5.109·10⁻⁸ (mg/g)², u(a₁) = 0.000 23 mg/g, χ²_obs = 12.5 (test fails).",
        "EURAMET cg-18 v4.0 H4.4, s_m = 0: a₁ = 0,000 83 mg/g, U(â) = 5,109·10⁻⁸ (mg/g)², u(a₁) = 0,000 23 mg/g, χ²_obs = 12,5 (test başarısız)."))
write("h4-400g-error-function-sm-0.05", 0.05,
      T("Balance 400 g / 0.1 mg: error approximation function with s_m = 0.05 mg, χ² test passed (EURAMET cg-18 H4)", "Terazi 400 g / 0,1 mg: s_m = 0,05 mg ile hata yaklaşım fonksiyonu, χ² testi geçer (EURAMET cg-18 H4)"),
      desc,
      [T("With s_m = 0.05 mg on the diagonal of U(e): a₁ = 0.000 84 mg/g, U(â) = 5.637·10⁻⁸ (mg/g)², u(a₁) = 0.000 24 mg/g; χ²_obs = 7.3 ≤ 8 (C2.2-2a passes).",
         "U(e)'nin köşegeninde s_m = 0,05 mg ile: a₁ = 0,000 84 mg/g, U(â) = 5,637·10⁻⁸ (mg/g)², u(a₁) = 0,000 24 mg/g; χ²_obs = 7,3 ≤ 8 (C2.2-2a geçer)."),
       T("u(E_appr,j) = I_j·u(a₁) (C2.2-11): 0.012 mg at 50 g … 0.095 mg at 400 g. The residuals at 150 g (−0.114 mg) and 250 g (+0.129 mg) exceed U(E_appr) = 2u: the stricter test C2.2-2b fails at two points (Table H4.6).",
         "u(E_yakl,j) = I_j·u(a₁) (C2.2-11): 50 g'da 0,012 mg … 400 g'da 0,095 mg. 150 g'daki (−0,114 mg) ve 250 g'daki (+0,129 mg) artıklar U(E_yakl) = 2u'yu aşar: daha katı C2.2-2b testi iki noktada başarısızdır (Tablo H4.6).")],
      {"coefficients": [chk(0.00084, 5e-6)], "u_coefficients": [chk(0.00024, 5e-6)], "chi_squared": chk(7.3, 0.05), "chi_squared_passed": True,
       "residual_failures": 2,
       "u_approximated": [chk(0.0, 5e-4)] + [chk(v, 5e-4) for v in (0.012, 0.024, 0.036, 0.047, 0.059, 0.071, 0.083, 0.095)]},
      T("EURAMET cg-18 v4.0 H4.4, s_m = 0.05 mg: a₁ = 0.000 84 mg/g, U(â) = 5.637·10⁻⁸ (mg/g)², χ²_obs = 7.3; u(E_appr) and the residual test of Table H4.6.",
        "EURAMET cg-18 v4.0 H4.4, s_m = 0,05 mg: a₁ = 0,000 84 mg/g, U(â) = 5,637·10⁻⁸ (mg/g)², χ²_obs = 7,3; Tablo H4.6'nın u(E_yakl) değerleri ve artık testi."))
write("h4-400g-error-function-sm-0.25", 0.25,
      T("Balance 400 g / 0.1 mg: error approximation function with s_m = 0.25 mg, residual test passed (EURAMET cg-18 H4)", "Terazi 400 g / 0,1 mg: s_m = 0,25 mg ile hata yaklaşım fonksiyonu, artık testi geçer (EURAMET cg-18 H4)"),
      desc,
      [T("To meet C2.2-2b at every point, s_m is raised to 0.25 mg: a₁ = 0.000 84 mg/g, U(â) = 1.745·10⁻⁷ (mg/g)², u(a₁) = 0.000 42 mg/g; all residuals lie within U(E_appr) (Table H4.8).",
         "C2.2-2b'yi her noktada sağlamak için s_m 0,25 mg'a çıkarılır: a₁ = 0,000 84 mg/g, U(â) = 1,745·10⁻⁷ (mg/g)², u(a₁) = 0,000 42 mg/g; tüm artıklar U(E_yakl) içindedir (Tablo H4.8)."),
       T("The price is a much larger uncertainty of the approximated error: U(E_appr) = 0.334 mg at 400 g instead of 0.190 mg.",
         "Bedeli, yaklaşık hatanın çok daha büyük belirsizliğidir: 400 g'da U(E_yakl) 0,190 mg yerine 0,334 mg.")],
      {"coefficients": [chk(0.00084, 5e-6)], "u_coefficients": [chk(0.00042, 5e-6)], "chi_squared_passed": True, "residual_failures": 0,
       "u_approximated": [chk(0.0, 5e-4)] + [chk(v, 5e-4) for v in (0.021, 0.042, 0.063, 0.084, 0.104, 0.125, 0.146, 0.167)]},
      T("EURAMET cg-18 v4.0 H4.4, s_m = 0.25 mg: a₁ = 0.000 84 mg/g, U(â) = 1.745·10⁻⁷ (mg/g)², u(a₁) = 0.000 42 mg/g; u(E_appr) of Table H4.8.",
        "EURAMET cg-18 v4.0 H4.4, s_m = 0,25 mg: a₁ = 0,000 84 mg/g, U(â) = 1,745·10⁻⁷ (mg/g)², u(a₁) = 0,000 42 mg/g; Tablo H4.8'in u(E_yakl) değerleri."),
      notes=[T("Table H4.8 prints the e values of Table H4.1 (0.213, 0.274, 0.181 mg) next to residuals computed from the budget values.",
               "Tablo H4.8, bütçe değerlerinden hesaplanmış artıkların yanında Tablo H4.1'in e değerlerini (0,213, 0,274, 0,181 mg) basar.")])
print("written")
