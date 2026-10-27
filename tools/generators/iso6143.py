"""Writes the ISO 6143:2025 Annex B library projects (run from the repository root)."""
import json
from pathlib import Path

DOC = "ISO 6143:2025, Gas analysis — Comparison methods for determining and checking the composition of calibration gas mixtures"
URL = "https://www.iso.org/standard/84420.html"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def rel(v, r): return chk(v, abs(v) * r)

def project(name, title, desc, expl, sec, tags, notes, body, expected, ref):
    pid = f"iso6143-{name}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "chemistry",
                      "sources": [{"document": DOC, "section": sec, "url": URL}], "tags": ["iso6143", "gas-analysis", "calibration"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p["calibration"] = body
    p["reporting"] = {"significant_digits": 2}
    expected["reference"] = ref
    p["expected"] = {"calibration": {k: v for k, v in expected.items() if k != "reference"}, "reference": ref}
    Path(f"library/chemistry/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

def points(rows, prefix="M"):
    return [{"name": f"{prefix}{i + 1}", "x": x, "u_x": ux, "y": y, "u_y": uy} for i, (x, ux, y, uy) in enumerate(rows)]

# ------------------------------------------------------------------ Example 1
ex1 = [(4.5, 0.045, 0.1969, 0.003938), (18.75, 0.1875, 0.7874, 0.015748), (50, 0.5, 2.0228, 0.040456)]
project("b1-no-uv-linear",
  T("NO in N₂ by UV absorption: straight-line analysis function (ISO 6143 Example 1)", "N₂ içinde NO, UV absorpsiyonu: doğrusal analiz fonksiyonu (ISO 6143 Örnek 1)"),
  T("ISO 6143:2025 B.2.2: three reference gas mixtures (5 µmol/mol to 50 µmol/mol) with uncertainties in both the contents and the UV responses, a straight-line analysis function by generalized least squares and the contents of three mixtures with their covariances.",
    "ISO 6143:2025 B.2.2: hem içeriklerinde hem de UV yanıtlarında belirsizlik bulunan üç referans gaz karışımı (5 µmol/mol – 50 µmol/mol), genelleştirilmiş en küçük karelerle doğrusal analiz fonksiyonu ve üç karışımın içerikleri ile kovaryansları."),
  [T("Steps A–G of ISO 6143 6.1: the responses are the means of ten measurements, u(y) their standard deviations of the mean; the relative uncertainty of both x and y is 1 % (contents) and 2 % (responses).",
     "ISO 6143 6.1 adım A–G: yanıtlar on ölçümün ortalamaları, u(y) ortalamanın standart sapmalarıdır; x'in bağıl belirsizliği %1, y'ninki %2'dir."),
   T("Step H: x = b₀ + b₁y is fitted by minimising S = Σ[(x̂ᵢ − xᵢ)²/u²(xᵢ) + (ŷᵢ − yᵢ)²/u²(yᵢ)] over b and the adjusted responses ŷᵢ (Annex A): b₀ = −0.3575 µmol/mol, b₁ = 24.612 µmol/mol.",
     "Adım H: x = b₀ + b₁y, S = Σ[(x̂ᵢ − xᵢ)²/u²(xᵢ) + (ŷᵢ − yᵢ)²/u²(yᵢ)] toplamı b ve düzeltilmiş yanıtlar ŷᵢ üzerinden en küçüklenerek uydurulur (Ek A): b₀ = −0,3575 µmol/mol, b₁ = 24,612 µmol/mol."),
   T("Validation (6.2.2): S_res = 0.674 against the acceptance limit 2(n − N − 1) = 2, and the largest weighted deviation Γ = 0.568 ≤ 2: the straight line is compatible with the data.",
     "Doğrulama (6.2.2): S_res = 0,674, kabul sınırı 2(n − N − 1) = 2 ile karşılaştırılır; en büyük ağırlıklı sapma Γ = 0,568 ≤ 2: doğru, verilerle uyumludur."),
   T("Steps I–K (6.3): for each mixture x = G(y) and u²(x) = (∂G/∂y)²u²(y) + Σ (∂G/∂bⱼ)(∂G/∂bₗ)u(bⱼ, bₗ); the three results are correlated through the common calibration.",
     "Adım I–K (6.3): her karışım için x = G(y) ve u²(x) = (∂G/∂y)²u²(y) + Σ (∂G/∂bⱼ)(∂G/∂bₗ)u(bⱼ, bₗ); üç sonuç ortak kalibrasyon nedeniyle koreledir.")],
  "Annex B.2.2 (Tables B.1–B.6)", ["generalized-least-squares", "errors-in-variables"],
  [T("The covariances of the three results printed in B.2.2 (1.16 × 10⁻², 1.48 × 10⁻², 1.37 × 10⁻¹ (µmol/mol)²) are reproduced; the report gives them as correlation coefficients.",
     "B.2.2'de basılan üç sonucun kovaryansları (1,16 × 10⁻², 1,48 × 10⁻², 1,37 × 10⁻¹ (µmol/mol)²) yeniden üretilir; rapor bunları korelasyon katsayıları olarak verir.")],
  {"analyte": "NO", "unit": "µmol/mol", "response_unit": "absorbance", "points": points(ex1), "function": "linear",
   "samples": [{"name": "S1", "y": 0.258, "u_y": 0.00516}, {"name": "S2", "y": 0.6, "u_y": 0.012}, {"name": "S3", "y": 1.8, "u_y": 0.036}]},
  {"params": [rel(-3.5747e-1, 2e-4), rel(2.4612e1, 5e-5)], "u_params": [rel(1.5716e-1, 5e-4), rel(4.8048e-1, 5e-4)],
   "cov_params": [rel(-5.6921e-2, 5e-4)], "s_res": rel(0.6743, 5e-4), "gamma": rel(0.568, 1e-3), "admissible": True,
   "samples": {"S1": {"x": rel(5.9923, 5e-5), "u": rel(1.6377e-1, 5e-4)}, "S2": {"x": rel(14.409, 5e-5), "u": rel(3.5599e-1, 5e-4)},
               "S3": {"x": rel(43.943, 5e-5), "u": rel(1.1631, 5e-4)}}},
  T("ISO 6143:2025 B.2.2, parameters and Tables B.4 and B.6 (B_LEAST).", "ISO 6143:2025 B.2.2, parametreler ile Tablo B.4 ve B.6 (B_LEAST)."))

# ------------------------------------------------------------------ Example 2
ex2 = [(1.5e-3, 9e-4, 60, 35), (0.1888, 4.5e-4, 7786, 135.7), (1.990, 4e-3, 81700, 36.7), (3.796, 3.9e-2, 156200, 223.2),
       (5.677, 1.25e-2, 233300, 137.2), (7.118, 1.25e-2, 293000, 245.5), (9.210, 2.0e-2, 380600, 125.1), (10.90, 2.5e-2, 449700, 321.8)]
samples2 = [{"name": "S1", "y": 70000, "u_y": 40}, {"name": "S2", "y": 370000, "u_y": 200}]
gc_steps = [
  T("Eight calibration points: a blank (M1, x = 0.0015 %) and seven reference gas mixtures; N₂ in a synthetic natural gas by gas chromatography with a thermal conductivity detector (peak areas y).",
    "Sekiz kalibrasyon noktası: bir kör (M1, x = %0,0015) ve yedi referans gaz karışımı; sentetik doğal gazda N₂, ısıl iletkenlik dedektörlü gaz kromatografisiyle (pik alanları y)."),
]
project("b2-n2-natural-gas-linear",
  T("N₂ in natural gas by GC-TCD: straight line (ISO 6143 Example 2)", "Doğal gazda N₂, GC-TCD: doğrusal fonksiyon (ISO 6143 Örnek 2)"),
  T("ISO 6143:2025 B.2.3: generalized least-squares straight line through eight calibration points with uncertainties in both variables, the comparison with the other analysis functions and the contents of two mixtures.",
    "ISO 6143:2025 B.2.3: her iki değişkende belirsizlikli sekiz kalibrasyon noktasından genelleştirilmiş en küçük kareler doğrusu, diğer analiz fonksiyonlarıyla karşılaştırma ve iki karışımın içerikleri."),
  gc_steps + [
   T("Straight line: b₀ = 3.981 × 10⁻⁴ %, b₁ = 2.4285 × 10⁻⁵ %/area; S_res = 6.05 (limit 2(n − N − 1) = 12) and Γ = 1.63 ≤ 2: admissible.",
     "Doğru: b₀ = 3,981 × 10⁻⁴ %, b₁ = 2,4285 × 10⁻⁵ %/alan; S_res = 6,05 (sınır 2(n − N − 1) = 12) ve Γ = 1,63 ≤ 2: kabul edilebilir."),
   T("The comparison table shows that the quadratic polynomial fits better (Γ = 0.87); ISO 6143 6.2.2 b) recommends the simplest admissible function when no physical model demands more.",
     "Karşılaştırma tablosu ikinci derece polinomun daha iyi uyduğunu gösterir (Γ = 0,87); ISO 6143 6.2.2 b), fiziksel bir model daha fazlasını gerektirmiyorsa kabul edilebilir en basit fonksiyonu önerir.")],
  "Annex B.2.3 (Tables B.7, B.8)", ["generalized-least-squares", "errors-in-variables", "gas-chromatography"],
  [],
  {"analyte": "N₂", "unit": "%", "response_unit": "peak area", "points": points(ex2), "function": "linear", "samples": samples2},
  {"params": [rel(3.9810e-4, 1e-3), rel(2.4285e-5, 5e-5)], "u_params": [rel(1.1458e-3, 1e-3), rel(2.4159e-8, 1e-3)],
   "cov_params": [rel(-7.2739e-12, 1e-3)], "s_res": rel(6.0455, 1e-3), "gamma": rel(1.6266, 1e-3), "admissible": True,
   "samples": {"S1": {"x": rel(1.7004, 1e-4), "u": rel(2.0244e-3, 1e-3)}, "S2": {"x": rel(8.9859, 1e-4), "u": rel(9.9712e-3, 1e-3)}}},
  T("ISO 6143:2025 B.2.3, linear parameters and Table B.8 (B_LEAST).", "ISO 6143:2025 B.2.3, doğrusal parametreler ve Tablo B.8 (B_LEAST)."))

project("b2-n2-natural-gas-quadratic",
  T("N₂ in natural gas by GC-TCD: second-order polynomial (ISO 6143 Example 2)", "Doğal gazda N₂, GC-TCD: ikinci derece polinom (ISO 6143 Örnek 2)"),
  T("ISO 6143:2025 B.2.3: the same calibration data with a second-order polynomial analysis function.",
    "ISO 6143:2025 B.2.3: aynı kalibrasyon verileri, ikinci derece polinom analiz fonksiyonuyla."),
  gc_steps + [
   T("x = b₀ + b₁y + b₂y²: b₂ = −4.09 × 10⁻¹³ with u(b₂) = 1.9 × 10⁻¹³; S_res = 1.40 (limit 10) and Γ = 0.87.",
     "x = b₀ + b₁y + b₂y²: b₂ = −4,09 × 10⁻¹³, u(b₂) = 1,9 × 10⁻¹³; S_res = 1,40 (sınır 10) ve Γ = 0,87."),
   T("The first-order coefficient changes by less than 0.5 % and the contents of the two mixtures agree with the straight-line results within their uncertainties; the standard leaves the choice to 6.2.2 (simplest admissible function).",
     "Birinci derece katsayısı %0,5'ten az değişir ve iki karışımın içerikleri belirsizlikleri içinde doğrusal sonuçlarla uyumludur; standart seçimi 6.2.2'ye bırakır (kabul edilebilir en basit fonksiyon).")],
  "Annex B.2.3 (Table B.9)", ["generalized-least-squares", "errors-in-variables", "gas-chromatography"],
  [],
  {"analyte": "N₂", "unit": "%", "response_unit": "peak area", "points": points(ex2), "function": "quadratic", "samples": samples2},
  {"params": [rel(-1.3111e-4, 2e-3), rel(2.4401e-5, 2e-4), rel(-4.0865e-13, 2e-3)],
   "u_params": [rel(1.175e-3, 2e-3), rel(5.901e-8, 2e-3), rel(1.895e-13, 2e-3)],
   "cov_params": [rel(-2.057e-11, 2e-3), rel(4.667e-17, 2e-3), rel(-1.020e-20, 2e-3)],
   "s_res": rel(1.3964, 1e-3), "gamma": rel(0.8664, 1e-3), "admissible": True,
   "samples": {"S1": {"x": rel(1.7059, 1e-4), "u": rel(3.2911e-3, 1e-3)}, "S2": {"x": rel(8.9723, 1e-4), "u": rel(1.1761e-2, 1e-3)}}},
  T("ISO 6143:2025 B.2.3, polynomial coefficients and Table B.9 (B_LEAST).", "ISO 6143:2025 B.2.3, polinom katsayıları ve Tablo B.9 (B_LEAST)."))

project("b2-n2-natural-gas-covariances",
  T("N₂ in natural gas by GC-TCD: correlated reference mixtures (ISO 6143 Example 2)", "Doğal gazda N₂, GC-TCD: korelasyonlu referans karışımlar (ISO 6143 Örnek 2)"),
  T("ISO 6143:2025 B.2.3: the straight line again, now with the covariances of reference mixtures that share their history (a dilution series and a common set of primary standards).",
    "ISO 6143:2025 B.2.3: yine doğru; bu kez geçmişi ortak olan referans karışımların (bir seyreltme serisi ve ortak bir birincil standart seti) kovaryanslarıyla."),
  gc_steps + [
   T("M7 was diluted from M4 (factor about 3): u(x₄, x₇) = 0.00016 %²; M5 and M8 were certified with a common calibration: u(x₅, x₈) = 0.0001 %².",
     "M7, M4'ten seyreltilmiştir (yaklaşık 3 kat): u(x₄, x₇) = 0,00016 %²; M5 ve M8 ortak bir kalibrasyonla sertifikalandırılmıştır: u(x₅, x₈) = 0,0001 %²."),
   T("The covariances enter only the propagation (Formula A.7): the parameters are unchanged, u(b₁) rises from 2.416 × 10⁻⁸ to 2.550 × 10⁻⁸ and u(x) of the two mixtures by 3 % to 4 %.",
     "Kovaryanslar yalnızca yayılıma girer (Formül A.7): parametreler değişmez, u(b₁) 2,416 × 10⁻⁸'den 2,550 × 10⁻⁸'e, iki karışımın u(x) değerleri %3–4 artar.")],
  "Annex B.2.3 (Table B.10)", ["generalized-least-squares", "errors-in-variables", "gas-chromatography", "correlation"],
  [],
  {"analyte": "N₂", "unit": "%", "response_unit": "peak area", "points": points(ex2),
   "covariances": [{"between": ["M4", "M7"], "value": 0.00016}, {"between": ["M5", "M8"], "value": 0.0001}],
   "function": "linear", "compare": False, "samples": samples2},
  {"params": [rel(3.9810e-4, 1e-3), rel(2.4285e-5, 5e-5)], "u_params": [rel(1.1463e-3, 1e-3), rel(2.5497e-8, 1e-3)],
   "cov_params": [rel(-7.6010e-12, 1e-3)],
   "samples": {"S1": {"x": rel(1.7004, 1e-4), "u": rel(2.0926e-3, 1e-3)}, "S2": {"x": rel(8.9859, 1e-4), "u": rel(1.0406e-2, 1e-3)}}},
  T("ISO 6143:2025 B.2.3, parameters with covariances and Table B.10 (B_LEAST).", "ISO 6143:2025 B.2.3, kovaryanslı parametreler ve Tablo B.10 (B_LEAST)."))

# ------------------------------------------------------------------ Example 3
ex3 = [(1.0006, 0.00134, 963.7988, 14), (1.0010, 0.0011, 966.2585, 12.5), (1.9995, 0.0032, 1912.5692, 16.8), (3.0018, 0.0034, 2846.9306, 11.6),
       (3.9982, 0.0058, 3754.9386, 10.8), (4.0043, 0.0058, 3764.5905, 12.1), (4.9981, 0.0058, 4647.1511, 14.4), (5.9961, 0.0078, 5529.9991, 12.1),
       (7.9974, 0.0084, 7246.0082, 11.9), (7.9995, 0.010, 7240.4767, 10.7), (9.9980, 0.012, 8884.6955, 10.1), (10.0060, 0.0104, 8902.6916, 14.1)]
ndir = T("Twelve reference gas mixtures from 1 µmol/mol to 10 µmol/mol NO in N₂ measured by non-dispersive infrared absorption; the response is clearly non-linear: the straight line gives S_res = 273 (limit 20) and Γ = 6.84 > 2.",
         "N₂ içinde 1 µmol/mol – 10 µmol/mol NO içeren on iki referans gaz karışımı, dağılmasız kızılötesi absorpsiyonla ölçülmüştür; yanıt belirgin biçimde doğrusal değildir: doğru S_res = 273 (sınır 20) ve Γ = 6,84 > 2 verir.")
project("b3-no-ndir-power",
  T("NO in N₂ by NDIR: power function (ISO 6143 Example 3)", "N₂ içinde NO, NDIR: üslü fonksiyon (ISO 6143 Örnek 3)"),
  T("ISO 6143:2025 B.2.4: a non-linear NDIR calibration with twelve points; the power analysis function x = b₀ + b₁y^(1+b₂) fitted by generalized least squares.",
    "ISO 6143:2025 B.2.4: on iki noktalı doğrusal olmayan bir NDIR kalibrasyonu; x = b₀ + b₁y^(1+b₂) üslü analiz fonksiyonu genelleştirilmiş en küçük karelerle uydurulur."),
  [ndir,
   T("Power function: b₂ = 0.0850 ± 0.0051; S_res = 8.38 (limit 18) and Γ = 1.16 ≤ 2: admissible.",
     "Üslü fonksiyon: b₂ = 0,0850 ± 0,0051; S_res = 8,38 (sınır 18) ve Γ = 1,16 ≤ 2: kabul edilebilir."),
   T("Mixture with y = 4950.6 ± 11: x = 5.3456 µmol/mol, u(x) = 0.0142 µmol/mol.",
     "y = 4950,6 ± 11 olan karışım: x = 5,3456 µmol/mol, u(x) = 0,0142 µmol/mol.")],
  "Annex B.2.4 (Tables B.11, B.12)", ["generalized-least-squares", "errors-in-variables", "non-linear"],
  [T("For this non-linear function the parameter covariances are computed by implicit differentiation of the minimum condition, the exact form of the numerical differentiation of A.2; they agree with B_LEAST within 0.3 %. A Monte Carlo simulation of the whole calibration (4000 refits, research log F42) gives u(x) = 0.0142 µmol/mol as well.",
     "Bu doğrusal olmayan fonksiyon için parametre kovaryansları, en küçük koşulunun kapalı türevlenmesiyle, yani A.2'deki sayısal türevlemenin tam biçimiyle hesaplanır; B_LEAST ile %0,3 içinde uyumludur. Tüm kalibrasyonun Monte Carlo benzetimi (4000 yeniden uydurma, araştırma günlüğü F42) de u(x) = 0,0142 µmol/mol verir.")],
  {"analyte": "NO", "unit": "µmol/mol", "response_unit": "NDIR signal", "points": points(ex3), "function": "power",
   "samples": [{"name": "S1", "y": 4950.6, "u_y": 11}]},
  {"params": [rel(0.12130, 2e-3), rel(5.1211e-4, 2e-3), rel(8.4990e-2, 2e-3)],
   "u_params": [rel(1.8095e-2, 3e-3), rel(2.4143e-5, 3e-3), rel(5.0692e-3, 3e-3)],
   "cov_params": [rel(-3.9719e-7, 5e-3), rel(8.2385e-5, 5e-3), rel(-1.2233e-7, 5e-3)],
   "s_res": rel(8.3804, 1e-3), "gamma": rel(1.1594, 1e-3), "admissible": True,
   "samples": {"S1": {"x": rel(5.3456, 1e-4), "u": rel(1.4169e-2, 2e-3)}}},
  T("ISO 6143:2025 B.2.4, power-function parameters and Table B.12 (B_LEAST).", "ISO 6143:2025 B.2.4, üslü fonksiyon parametreleri ve Tablo B.12 (B_LEAST)."))

project("b3-no-ndir-exponential",
  T("NO in N₂ by NDIR: exponential function (ISO 6143 Example 3)", "N₂ içinde NO, NDIR: üstel fonksiyon (ISO 6143 Örnek 3)"),
  T("ISO 6143:2025 B.2.4: the same NDIR calibration with the exponential analysis function x = b₀ + b₁e^(b₂y), the function of choice in the standard; the parameter uncertainties printed there are known to be wrong.",
    "ISO 6143:2025 B.2.4: aynı NDIR kalibrasyonu, standardın tercih ettiği x = b₀ + b₁e^(b₂y) üstel analiz fonksiyonuyla; orada basılan parametre belirsizliklerinin yanlış olduğu bilinmektedir."),
  [ndir,
   T("Exponential function: b₀ = −47.96, b₁ = 47.97, b₂ = 2.128 × 10⁻⁵ (b₀ and b₁ almost cancel: their correlation is −1.000); S_res = 0.657 (limit 18) and Γ = 0.353, the best fit of the three-parameter functions considered in the standard (the comparison table adds the quadratic polynomial, Γ = 0.44, which the standard does not try).",
     "Üstel fonksiyon: b₀ = −47,96, b₁ = 47,97, b₂ = 2,128 × 10⁻⁵ (b₀ ile b₁ neredeyse birbirini götürür: korelasyonları −1,000'dir); S_res = 0,657 (sınır 18) ve Γ = 0,353; standardın denediği üç parametreli fonksiyonların en iyi uyumu (karşılaştırma tablosu standardın denemediği ikinci derece polinomu da ekler, Γ = 0,44)."),
   T("Mixture with y = 4950.6 ± 11: x = 5.3357 µmol/mol, u(x) = 0.0142 µmol/mol; the power function gives 5.3456 µmol/mol, within one standard uncertainty.",
     "y = 4950,6 ± 11 olan karışım: x = 5,3357 µmol/mol, u(x) = 0,0142 µmol/mol; üslü fonksiyon 5,3456 µmol/mol verir, bir standart belirsizlik içinde.")],
  "Annex B.1 and B.2.4 (Tables B.11, B.12)", ["generalized-least-squares", "errors-in-variables", "non-linear", "erratum"],
  [T("ISO 6143 B.1 and B.2.4 state that B_LEAST gives incorrect parameter uncertainties for the exponential function (found at METAS). The printed u(b) = 8.14 × 10⁻², 7.40 × 10⁻², 3.71 × 10⁻¹¹ are about 39 times too small; this program gives 3.18, 3.17 and 1.27 × 10⁻⁶, in agreement with ODRPACK (scipy.odr) and a Monte Carlo simulation of the calibration (3.13, 3.12, 1.24 × 10⁻⁶).",
     "ISO 6143 B.1 ve B.2.4, B_LEAST'in üstel fonksiyon için yanlış parametre belirsizlikleri verdiğini belirtir (METAS'ta bulunmuştur). Basılan u(b) = 8,14 × 10⁻², 7,40 × 10⁻², 3,71 × 10⁻¹¹ değerleri yaklaşık 39 kat küçüktür; bu program 3,18, 3,17 ve 1,27 × 10⁻⁶ verir; ODRPACK (scipy.odr) ve kalibrasyonun Monte Carlo benzetimiyle (3,13, 3,12, 1,24 × 10⁻⁶) uyumludur."),
   T("Table B.12 gives u(x) = 0.01328 µmol/mol for the exponential function; first-order propagation with the correct covariances gives 0.01425 and the Monte Carlo simulation 0.01406 µmol/mol: the printed value is 6 % to 7 % too low, more than the 'about 3 %' the standard allows for iteration settings (research log F42). The project checks the parameters and x against the standard and u(x) against the independent calculations.",
     "Tablo B.12 üstel fonksiyon için u(x) = 0,01328 µmol/mol verir; doğru kovaryanslarla birinci mertebe yayılım 0,01425, Monte Carlo benzetimi 0,01406 µmol/mol verir: basılı değer %6–7 küçüktür; bu, standardın iterasyon ayarları için öngördüğü 'yaklaşık %3'ten fazladır (araştırma günlüğü F42). Proje parametreleri ve x'i standarda, u(x)'i bağımsız hesaplara göre kontrol eder.")],
  {"analyte": "NO", "unit": "µmol/mol", "response_unit": "NDIR signal", "points": points(ex3), "function": "exponential",
   "samples": [{"name": "S1", "y": 4950.6, "u_y": 11}]},
  {"params": [rel(-47.963, 2e-3), rel(47.969, 2e-3), rel(2.1283e-5, 2e-3)],
   "u_params": [rel(3.1816, 5e-3), rel(3.1705, 5e-3), rel(1.2739e-6, 5e-3)],
   "s_res": rel(0.6572, 2e-3), "gamma": rel(0.3529, 2e-3), "admissible": True,
   "samples": {"S1": {"x": rel(5.3357, 1e-4), "u": rel(1.4246e-2, 3e-3)}}},
  T("ISO 6143:2025 B.2.4, exponential parameters, S_res, Γ and x of Table B.12 (B_LEAST); u(b) and u(x) from ODRPACK (scipy.odr, cov_beta) for the same data.",
    "ISO 6143:2025 B.2.4, üstel parametreler, S_res, Γ ve Tablo B.12'deki x (B_LEAST); u(b) ve u(x) aynı veriler için ODRPACK'ten (scipy.odr, cov_beta)."))
