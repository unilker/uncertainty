"""Writes the NIST TN 1900 library projects (run from the repository root)."""
import json, math
from pathlib import Path

URL = "https://doi.org/10.6028/NIST.TN.1900"
DOC = "NIST Technical Note 1900, Simple Guide for Evaluating and Expressing the Uncertainty of NIST Measurement Results (A. Possolo, 2015)"

def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}
def src(sec): return [{"document": DOC, "section": sec, "url": URL}]
def inp(name, unit, en, tr, dist):
    d = {"name": name}
    if unit: d["unit"] = unit
    d["description"] = T(en, tr); d["distribution"] = dist
    return d
def normal(m, s): return {"type": "normal", "mean": m, "sd": s}
def rect(lo, hi): return {"type": "rectangular", "lower": lo, "upper": hi}
def rect_hw(v, a): return {"type": "rectangular", "value": v, "half_width": a}
def arcsine_hw(v, a): return {"type": "arcsine", "value": v, "half_width": a}
def tri_hw(v, a): return {"type": "triangular", "value": v, "half_width": a}
def t3(m, u): return {"type": "t", "mean": m, "dof": 3, "scale": u / math.sqrt(3)}

def project(path, pid, title, desc, expl, field, section, tags, notes, model, analysis, expected, reporting=None):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": field,
                      "sources": src(section), "tags": ["nist-tn1900"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p["model"] = model; p["analysis"] = analysis
    if reporting: p["reporting"] = reporting
    p["expected"] = expected
    Path(path).write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

# ---------------------------------------------------------------- E1
project("library/mass/nist-tn1900-e1-weighing.json", "nist-tn1900-e1-weighing",
  T("Mass of a powder by weighing (NIST TN 1900 E1)", "Tartımla toz kütlesi (NIST TN 1900 E1)"),
  T("TN 1900 E1: mass of a powder from three weighings with a single-pan balance; linear model, GUM and MCM agree.",
    "TN 1900 E1: tek kefeli terazide üç tartımdan toz kütlesi; doğrusal model, GUM ve MCM aynı sonucu verir."),
  [T("The mass of the powder is m_P = c_P − c_E − (c_R2 − c_R1): the reading with the powder in the container, minus the reading of the empty container, minus the drift of the balance between two readings of a reference weight.",
     "Tozun kütlesi m_P = c_P − c_E − (c_R2 − c_R1) bağıntısıyla bulunur: toz dolu kabın okuması, boş kabın okuması ve referans ağırlığın iki okuması arasındaki terazi kaymasının farkı."),
   T("The standard uncertainties of c_P and c_E (0.005015 g) combine the uncertainty of the container (0.005 g) with those of the balance (calibration, repeatability, linearity, temperature: 0.1, 0.1, 0.2 and 0.3 mg); that of the drift term is 0.0005477 g (TN 1900 Exhibit 1).",
     "c_P ve c_E'nin standart belirsizlikleri (0,005015 g), kabın belirsizliğini (0,005 g) terazinin kalibrasyon, tekrarlanabilirlik, doğrusallık ve sıcaklık bileşenleriyle (0,1; 0,1; 0,2 ve 0,3 mg) birleştirir; kayma teriminin belirsizliği 0,0005477 g'dır (TN 1900 Şekil 1)."),
   T("All sensitivity coefficients are ±1, so u(m_P)² = u(c_P)² + u(c_E)² + u(c_R2 − c_R1)², u = 0.0071 g; the output is Gaussian and the Monte Carlo method gives the same 95 % interval [50.270, 50.298] g.",
     "Tüm duyarlılık katsayıları ±1 olduğundan u(m_P)² = u(c_P)² + u(c_E)² + u(c_R2 − c_R1)², u = 0,0071 g olur; çıktı normal dağılımlıdır ve Monte Carlo yöntemi aynı %95 aralığını, [50,270; 50,298] g, verir.")],
  "mass", "Example E1, Exhibit 1", ["linear", "balance"], None,
  {"output": {"name": "m_P", "unit": "g", "expression": "c_P - c_E - dc_R", "description": T("Mass of the powder", "Tozun kütlesi")},
   "inputs": [inp("c_P", "g", "Reading with the powder in the container", "Toz dolu kabın okuması", normal(53.768, 0.005015)),
              inp("c_E", "g", "Reading of the empty container", "Boş kabın okuması", normal(3.436, 0.005015)),
              inp("dc_R", "g", "Drift c_R2 − c_R1 of the reference-weight readings", "Referans ağırlık okumalarının kayması c_R2 − c_R1", normal(0.048, 0.0005477))]},
  {"gum": {}, "mcm": {"trials": 1000000, "seed": 1, "interval": "symmetric"}},
  {"gum": {"estimate": chk(50.284, 0.0005), "standard_uncertainty": chk(0.0071, 0.00005)},
   "mcm": {"estimate": chk(50.284, 0.0005), "standard_uncertainty": chk(0.0071, 0.00005),
           "interval_low": chk(50.270, 0.0005), "interval_high": chk(50.298, 0.0005), "gum_validated": True},
   "reference": T("NIST TN 1900 E1: m_P = 50.284 g, u = 0.0071 g, 95 % interval [50.270, 50.298] g.",
                  "NIST TN 1900 E1: m_P = 50,284 g, u = 0,0071 g, %95 aralığı [50,270; 50,298] g.")})

# ---------------------------------------------------------------- E2
temps = [18.75, 28.25, 25.75, 28.00, 28.50, 20.75, 21.00, 22.75, 18.50, 27.25, 20.75,
         26.50, 28.00, 23.25, 28.00, 21.75, 26.00, 26.50, 28.00, 33.25, 32.00, 29.50]
project("library/temperature/nist-tn1900-e2-surface-temperature.json", "nist-tn1900-e2-surface-temperature",
  T("Monthly mean of daily maximum temperature (NIST TN 1900 E2)", "Günlük en yüksek sıcaklığın aylık ortalaması (NIST TN 1900 E2)"),
  T("TN 1900 E2: an observation equation t_i = τ + E_i for 22 daily readings; Type A evaluation with 21 degrees of freedom.",
    "TN 1900 E2: 22 günlük okuma için t_i = τ + E_i gözlem denklemi; 21 serbestlik dereceli A tipi değerlendirme."),
  [T("The measurand τ is the mean of the true daily maxima in May 2012; each reading deviates from it by an error E_i, modelled as independent and Gaussian with mean 0 (the observation equation of TN 1900).",
     "Ölçülen büyüklük τ, Mayıs 2012'deki gerçek günlük en yüksek sıcaklıkların ortalamasıdır; her okuma ondan, ortalaması 0 olan bağımsız normal dağılımlı bir E_i hatası kadar sapar (TN 1900'ün gözlem denklemi)."),
   T("The estimate is the mean of the m = 22 readings, 25.59 °C; s = 4.09 °C, so u(τ) = s/√22 = 0.872 °C with ν = 21 (GUM 4.2.3).",
     "Tahmin, m = 22 okumanın ortalaması olan 25,59 °C'dir; s = 4,09 °C olduğundan u(τ) = s/√22 = 0,872 °C ve ν = 21'dir (GUM 4.2.3)."),
   T("The coverage factor for 95 % is the t quantile t₀.₉₇₅(21) = 2.08, giving [23.8, 27.4] °C. The Monte Carlo method draws τ from the scaled and shifted t₂₁ distribution of JCGM 101 6.4.9 and gives the same interval.",
     "%95 için kapsam faktörü t₀,₉₇₅(21) = 2,08 t dağılımı yüzdeliğidir ve aralık [23,8; 27,4] °C olur. Monte Carlo yöntemi τ'yu JCGM 101 6.4.9'un ölçeklenmiş ve kaydırılmış t₂₁ dağılımından çeker ve aynı aralığı verir.")],
  "temperature", "Example E2, Exhibit 2", ["type-a", "observation-equation", "student-t"],
  [T("The Monte Carlo standard deviation is s/√m · √(ν/(ν − 2)) = 0.917 °C, larger than u = 0.872 °C, because the t distribution of JCGM 101 6.4.9 has scale s/√m; the coverage intervals coincide.",
     "Monte Carlo standart sapması s/√m · √(ν/(ν − 2)) = 0,917 °C'dir ve u = 0,872 °C'den büyüktür, çünkü JCGM 101 6.4.9'un t dağılımının ölçeği s/√m'dir; kapsam aralıkları ise çakışır."),
   T("TN 1900 also gives the distribution-free Wilcoxon interval [23.6, 27.6] °C, which is not a GUM or Monte Carlo result and is not reproduced here.",
     "TN 1900 ayrıca dağılımdan bağımsız Wilcoxon aralığını, [23,6; 27,6] °C, verir; bu bir GUM ya da Monte Carlo sonucu olmadığından burada yeniden üretilmez.")],
  {"output": {"name": "tau", "unit": "°C", "expression": "t", "description": T("Mean daily maximum temperature in May 2012", "Mayıs 2012'deki ortalama günlük en yüksek sıcaklık")},
   "inputs": [inp("t", "°C", "Daily maximum readings of the thermometer", "Termometrenin günlük en yüksek sıcaklık okumaları", {"type": "type_a", "observations": temps})]},
  {"gum": {}, "mcm": {"trials": 1000000, "seed": 2, "interval": "symmetric"}},
  {"gum": {"estimate": chk(25.59, 0.005), "standard_uncertainty": chk(0.872, 0.0005), "coverage_factor": chk(2.08, 0.005),
           "effective_dof": chk(21, 0.001)},
   "mcm": {"interval_low": chk(23.8, 0.05), "interval_high": chk(27.4, 0.05)},
   "reference": T("NIST TN 1900 E2: t̄ = 25.6 °C, u = 0.872 °C, k = 2.08, 95 % interval (23.8, 27.4) °C; 25.59 °C from the text.",
                  "NIST TN 1900 E2: t̄ = 25,6 °C, u = 0,872 °C, k = 2,08, %95 aralığı (23,8; 27,4) °C; 25,59 °C metinden.")})

# ---------------------------------------------------------------- E3
project("library/physical/nist-tn1900-e3-falling-ball-viscometer.json", "nist-tn1900-e3-falling-ball-viscometer",
  T("Falling-ball viscometer (NIST TN 1900 E3)", "Düşen bilyalı viskozimetre (NIST TN 1900 E3)"),
  T("TN 1900 E3: dynamic viscosity of a NaOH solution by comparison with a calibration liquid; large relative uncertainties of the travel times make the output skewed.",
    "TN 1900 E3: bir NaOH çözeltisinin dinamik viskozitesi kalibrasyon sıvısıyla karşılaştırılarak; düşme sürelerinin büyük bağıl belirsizlikleri çıktıyı çarpık yapar."),
  [T("μ_M = μ_C (ρ_B − ρ_M)/(ρ_B − ρ_C) · t_M/t_C with μ_C = 4.63 mPa s, ρ_B = 2217, ρ_C = 810, ρ_M = 1180 kg/m³, t_C = 36.6 s and t_M = 61 s; all inputs Gaussian.",
     "μ_M = μ_C (ρ_B − ρ_M)/(ρ_B − ρ_C) · t_M/t_C; μ_C = 4,63 mPa s, ρ_B = 2217, ρ_C = 810, ρ_M = 1180 kg/m³, t_C = 36,6 s ve t_M = 61 s; tüm girdiler normal dağılımlı."),
   T("The relative standard uncertainties are 1 % for μ_C, 15 % for t_C and 10 % for t_M; the densities have u = 0.5 kg/m³. TN 1900 notes that these are deliberately large.",
     "Bağıl standart belirsizlikler μ_C için %1, t_C için %15 ve t_M için %10'dur; yoğunlukların u'su 0,5 kg/m³'tür. TN 1900 bu değerlerin bilerek büyük seçildiğini belirtir."),
   T("Substituting the estimates gives 5.69 mPa s and the first-order law of propagation u = 1.03 mPa s. Because 1/t_C is strongly non-linear over ±15 %, the Monte Carlo mean is 5.82 mPa s, u = 1.11 mPa s, and the 95 % interval [4.05, 8.39] mPa s is asymmetric about the estimate.",
     "Tahminlerin yerine konmasıyla 5,69 mPa s ve birinci mertebe yayılım yasasıyla u = 1,03 mPa s bulunur. 1/t_C, ±%15 aralığında belirgin biçimde doğrusal olmadığından Monte Carlo ortalaması 5,82 mPa s, u = 1,11 mPa s olur ve %95 aralığı, [4,05; 8,39] mPa s, tahmine göre asimetriktir."),
   T("The GUM interval 5.69 ± 2.01 mPa s is therefore not validated: its end points lie 0.38 mPa s and 0.69 mPa s below those of the Monte Carlo interval. The shortest Monte Carlo interval, [3.84, 8.05] mPa s, differs again from the probabilistically symmetric one used by TN 1900.",
     "Bu nedenle GUM aralığı, 5,69 ± 2,01 mPa s, doğrulanmaz: uç noktaları Monte Carlo aralığınınkilerin 0,38 mPa s ve 0,69 mPa s altındadır. En kısa Monte Carlo aralığı, [3,84; 8,05] mPa s, TN 1900'ün kullandığı olasılıksal simetrik aralıktan da farklıdır.")],
  "physical", "Example E3, Exhibit 3", ["non-linear", "skewed", "validation"],
  [T("TN 1900 states that the conventional propagation formula, as implemented in the NIST Uncertainty Machine, gives 1.11 mPa s, identical to the Monte Carlo value. The first-order law of propagation (GUM eq. 10) applied to the stated model and inputs gives 1.027 mPa s; with the second-order terms of GUM 5.1.2 it gives about 1.10 mPa s. The NIST Uncertainty Machine manual (v1.4, §9) repeats the statement, but the output it prints in Figure 5 gives u(y) = 1 mPa s for Gauss's formula, consistent with 1.027 mPa s: the 1.11 mPa s of the text is the Monte Carlo value.",
     "TN 1900, NIST Uncertainty Machine'de uygulanan klasik yayılım formülünün 1,11 mPa s verdiğini ve bunun Monte Carlo değeriyle aynı olduğunu belirtir. Birinci mertebe yayılım yasası (GUM eş. 10) verilen model ve girdilere uygulandığında 1,027 mPa s; GUM 5.1.2'nin ikinci mertebe terimleriyle yaklaşık 1,10 mPa s verir. NIST Uncertainty Machine kılavuzu (sürüm 1.4, §9) aynı ifadeyi tekrarlar; ancak Şekil 5'te bastığı çıktı Gauss formülü için u(y) = 1 mPa s verir ve bu 1,027 mPa s ile tutarlıdır: metindeki 1,11 mPa s Monte Carlo değeridir.")],
  {"output": {"name": "mu_M", "unit": "mPa s", "expression": "mu_C * (rho_B - rho_M) / (rho_B - rho_C) * t_M / t_C",
              "description": T("Dynamic viscosity of the NaOH solution", "NaOH çözeltisinin dinamik viskozitesi")},
   "inputs": [inp("mu_C", "mPa s", "Viscosity of the calibration liquid", "Kalibrasyon sıvısının viskozitesi", normal(4.63, 0.0463)),
              inp("rho_B", "kg/m³", "Density of the ball", "Bilyanın yoğunluğu", normal(2217, 0.5)),
              inp("rho_C", "kg/m³", "Density of the calibration liquid", "Kalibrasyon sıvısının yoğunluğu", normal(810, 0.5)),
              inp("rho_M", "kg/m³", "Density of the NaOH solution", "NaOH çözeltisinin yoğunluğu", normal(1180, 0.5)),
              inp("t_C", "s", "Travel time of the ball in the calibration liquid", "Bilyanın kalibrasyon sıvısındaki düşme süresi", normal(36.6, 5.49)),
              inp("t_M", "s", "Travel time of the ball in the NaOH solution", "Bilyanın NaOH çözeltisindeki düşme süresi", normal(61, 6.1))]},
  {"gum": {}, "mcm": {"trials": 2000000, "seed": 3, "interval": "symmetric"}},
  {"gum": {"estimate": chk(5.69, 0.005), "standard_uncertainty": chk(1.0269, 0.0005)},
   "mcm": {"estimate": chk(5.82, 0.01), "standard_uncertainty": chk(1.11, 0.01),
           "interval_low": chk(4.05, 0.01), "interval_high": chk(8.39, 0.02), "gum_validated": False},
   "reference": T("NIST TN 1900 E3: estimate 5.69 mPa s from the measurement equation; Monte Carlo 5.82 mPa s, u = 1.11 mPa s, 95 % interval [4.05, 8.39] mPa s. GUM u = 1.027 mPa s: first-order law of propagation evaluated analytically (see note).",
                  "NIST TN 1900 E3: ölçüm denkleminden tahmin 5,69 mPa s; Monte Carlo 5,82 mPa s, u = 1,11 mPa s, %95 aralığı [4,05; 8,39] mPa s. GUM u = 1,027 mPa s: birinci mertebe yayılım yasasının analitik değeri (nota bakınız).")})

# ---------------------------------------------------------------- E4
project("library/flow/nist-tn1900-e4-pitot-tube.json", "nist-tn1900-e4-pitot-tube",
  T("Airspeed with a Pitot tube (NIST TN 1900 E4)", "Pitot tüpüyle hava hızı (NIST TN 1900 E4)"),
  T("TN 1900 E4 (after Kline and McClintock, 1953): airspeed from the differential pressure, the air pressure and the temperature.",
    "TN 1900 E4 (Kline ve McClintock, 1953'ten): fark basıncı, hava basıncı ve sıcaklıktan hava hızı."),
  [T("v = √(2ΔR_sT/p), where the air density ρ = p/(R_sT) comes from the ideal gas law; R_s = 287.058 J kg⁻¹ K⁻¹ is the specific gas constant of dry air.",
     "v = √(2ΔR_sT/p); burada hava yoğunluğu ρ = p/(R_sT) ideal gaz yasasından gelir; R_s = 287,058 J kg⁻¹ K⁻¹ kuru havanın özgül gaz sabitidir."),
   T("Δ = 1.993 kPa (U-tube manometer), p = 101.4 kPa (Bourdon gauge) and T = 292.8 K (mercury-in-glass thermometer) with 95 % expanded uncertainties 0.025 kPa, 2.1 kPa and 0.11 K; the standard uncertainties are half of these.",
     "Δ = 1,993 kPa (U-tüp manometre), p = 101,4 kPa (Bourdon manometre) ve T = 292,8 K (cıvalı cam termometre); %95 genişletilmiş belirsizlikleri 0,025 kPa, 2,1 kPa ve 0,11 K'dir, standart belirsizlikler bunların yarısıdır."),
   T("For a power law the relative uncertainties add in quadrature with weights ½: u(v)/v = ½ √((u_Δ/Δ)² + (u_p/p)² + (u_T/T)²) = 0.61 %, dominated by the air pressure. The Monte Carlo interval is slightly shifted towards lower values (v ∝ p^−½), and the GUM interval is not validated for two significant digits of u (end-point differences 0.007 and 0.009 m/s against δ = 0.005 m/s), as the second-digit differences between the two intervals printed in TN 1900 also show.",
     "Üslü bir bağıntıda bağıl belirsizlikler ½ ağırlıkla kareler toplamı olarak birleşir: u(v)/v = ½ √((u_Δ/Δ)² + (u_p/p)² + (u_T/T)²) = %0,61; baskın katkı hava basıncınındır. Monte Carlo aralığı biraz daha düşük değerlere kaymıştır (v ∝ p^−½) ve GUM aralığı u'nun iki anlamlı basamağı için doğrulanmaz (uç nokta farkları 0,007 ve 0,009 m/s, δ = 0,005 m/s); TN 1900'de basılı iki aralık arasındaki ikinci basamak farkları da bunu gösterir.")],
  "flow", "Example E4", ["non-linear", "power-law", "pressure"],
  [T("With the stated equation and inputs, v = 57.48 m/s and u(v) = 0.348 m/s. TN 1900 prints v = 40.64 m/s and u(v) = 0.25 m/s, which are those values divided by √2, i.e. the result of v = √(ΔR_sT/p) without the factor 2. The relative uncertainty, 0.61 %, is the same in both.",
     "Verilen denklem ve girdilerle v = 57,48 m/s ve u(v) = 0,348 m/s bulunur. TN 1900 ise v = 40,64 m/s ve u(v) = 0,25 m/s basar; bunlar aynı değerlerin √2'ye bölünmüşüdür, yani 2 çarpanı olmadan v = √(ΔR_sT/p) bağıntısının sonucudur. Bağıl belirsizlik (%0,61) her ikisinde de aynıdır.")],
  {"output": {"name": "v", "unit": "m/s", "expression": "sqrt(2 * 1000 * Delta * R_s * T / (1000 * p))",
              "description": T("Airspeed", "Hava hızı")},
   "inputs": [inp("Delta", "kPa", "Difference between total and static pressure", "Toplam ve statik basınç farkı", normal(1.993, 0.0125)),
              inp("R_s", "J/(kg K)", "Specific gas constant of dry air", "Kuru havanın özgül gaz sabiti", {"type": "constant", "value": 287.058}),
              inp("T", "K", "Air temperature", "Hava sıcaklığı", normal(292.8, 0.055)),
              inp("p", "kPa", "Air pressure", "Hava basıncı", normal(101.4, 1.05))]},
  {"gum": {}, "mcm": {"trials": 1000000, "seed": 4, "interval": "symmetric"}},
  {"gum": {"estimate": chk(40.64 * math.sqrt(2), 0.01), "standard_uncertainty": chk(0.246 * math.sqrt(2), 0.002)},
   "mcm": {"interval_low": chk(40.17 * math.sqrt(2), 0.01), "interval_high": chk(41.13 * math.sqrt(2), 0.01), "gum_validated": False},
   "reference": T("NIST TN 1900 E4: v = 40.64 m/s, u(v) = 0.25 m/s (relative 0.61 %), Monte Carlo 95 % interval [40.17, 41.13] m/s, each multiplied by √2 to correct for the missing factor 2 (see note).",
                  "NIST TN 1900 E4: v = 40,64 m/s, u(v) = 0,25 m/s (bağıl %0,61), Monte Carlo %95 aralığı [40,17; 41,13] m/s; eksik 2 çarpanı için her biri √2 ile çarpılmıştır (nota bakınız).")})

# ---------------------------------------------------------------- E8
project("library/chemistry/nist-tn1900-e8-molecular-weight-co2.json", "nist-tn1900-e8-molecular-weight-co2",
  T("Molecular weight of carbon dioxide (NIST TN 1900 E8)", "Karbondioksitin molekül ağırlığı (NIST TN 1900 E8)"),
  T("TN 1900 E8: M_r(CO₂) = A_r(C) + 2A_r(O) from the IUPAC interval atomic weights; the output distribution is trapezoidal and the GUM interval is too wide.",
    "TN 1900 E8: IUPAC aralık atom ağırlıklarından M_r(CO₂) = A_r(C) + 2A_r(O); çıktı dağılımı yamuktur ve GUM aralığı gereğinden geniştir."),
  [T("The standard atomic weights are intervals: A_r(C) ∈ [12.0096, 12.0116] and A_r(O) ∈ [15.99903, 15.99977] (Wieser et al. 2013). Each is modelled as a rectangular distribution over its interval.",
     "Standart atom ağırlıkları aralıktır: A_r(C) ∈ [12,0096; 12,0116] ve A_r(O) ∈ [15,99903; 15,99977] (Wieser ve ark. 2013). Her biri kendi aralığı üzerinde dikdörtgen dağılımla modellenir."),
   T("The estimate is 12.0106 + 2 × 15.9994 = 44.0094 and u² = u(A_r(C))² + 4u(A_r(O))², u = 0.00072.",
     "Tahmin 12,0106 + 2 × 15,9994 = 44,0094'tür ve u² = u(A_r(C))² + 4u(A_r(O))², u = 0,00072 olur."),
   T("The sum of two rectangular variables has a symmetric trapezoidal distribution with half-widths 0.00174 (base) and 0.00026 (top). Its shortest 95 % interval is 44.0094 ± 0.00136 = [44.0080, 44.0108], narrower than the GUM interval ± 1.96u = ± 0.00141; the Monte Carlo method reproduces the exact interval.",
     "İki dikdörtgen değişkenin toplamı, yarı genişlikleri 0,00174 (taban) ve 0,00026 (üst) olan simetrik yamuk dağılıma sahiptir. En kısa %95 aralığı 44,0094 ± 0,00136 = [44,0080; 44,0108] olup GUM aralığı ± 1,96u = ± 0,00141'den dardır; Monte Carlo yöntemi tam aralığı yeniden üretir.")],
  "chemistry", "Example E8, Exhibit 12", ["rectangular", "trapezoidal", "atomic-weights", "validation"],
  [T("TN 1900 prints u = 0.000721 from the rounded components 0.0006 and 0.0002; the unrounded components 0.000577 and 0.000214 give 0.000718. The exact interval end points follow from the trapezoid: 44.0094 ± (0.00174 − √(0.2 × 0.001 × 0.00074)) = [44.008045, 44.010755].",
     "TN 1900, yuvarlanmış 0,0006 ve 0,0002 bileşenlerinden u = 0,000721 basar; yuvarlanmamış 0,000577 ve 0,000214 bileşenleri 0,000718 verir. Tam aralığın uç noktaları yamuk dağılımdan çıkar: 44,0094 ± (0,00174 − √(0,2 × 0,001 × 0,00074)) = [44,008045; 44,010755].")],
  {"output": {"name": "M_r", "expression": "A_C + 2 * A_O", "description": T("Relative molecular mass of CO₂", "CO₂'nin bağıl molekül kütlesi")},
   "inputs": [inp("A_C", None, "Standard atomic weight of carbon", "Karbonun standart atom ağırlığı", rect(12.0096, 12.0116)),
              inp("A_O", None, "Standard atomic weight of oxygen", "Oksijenin standart atom ağırlığı", rect(15.99903, 15.99977))]},
  {"gum": {}, "mcm": {"trials": 1000000, "seed": 8, "interval": "shortest"}},
  {"gum": {"estimate": chk(44.0094, 0.00005), "standard_uncertainty": chk(0.0007, 0.00005)},
   "mcm": {"interval_low": chk(44.008045, 0.00001), "interval_high": chk(44.010755, 0.00001), "gum_validated": False},
   "reference": T("NIST TN 1900 E8: M_r(CO₂) = 44.0094, u = 0.0007, exact shortest 95 % interval [44.0080, 44.0108], whose unrounded end points 44.008045 and 44.010755 follow from the trapezoidal distribution (see note); tolerance ≈ 5 × the Monte Carlo dispersion at M = 10⁶.",
                  "NIST TN 1900 E8: M_r(CO₂) = 44,0094, u = 0,0007, tam en kısa %95 aralığı [44,0080; 44,0108]; yuvarlanmamış uç noktalar 44,008045 ve 44,010755 yamuk dağılımdan çıkar (nota bakınız); tolerans M = 10⁶'da Monte Carlo saçılımının ≈ 5 katı.")},
  reporting={"significant_digits": 2})

# ---------------------------------------------------------------- E9
project("library/chemistry/nist-tn1900-e9-cadmium-standard.json", "nist-tn1900-e9-cadmium-standard",
  T("Cadmium calibration standard, Monte Carlo (NIST TN 1900 E9)", "Kadmiyum kalibrasyon standardı, Monte Carlo (NIST TN 1900 E9)"),
  T("TN 1900 E9: the QUAM A1 cadmium standard c = 1000 m P / V re-evaluated with the Monte Carlo method, with a rectangular purity and a triangular volume.",
    "TN 1900 E9: QUAM A1 kadmiyum standardı, c = 1000 m P / V, dikdörtgen saflık ve üçgen hacim dağılımıyla Monte Carlo yöntemiyle yeniden değerlendirilir."),
  [T("m = 100.28 mg (u = 0.05 mg, Gaussian), P = 0.9999 (rectangular on [0.9998, 1], u = 0.000058) and V = 100.0 mL (triangular, u = 0.07 mL); c = 1002.7 mg/L.",
     "m = 100,28 mg (u = 0,05 mg, normal), P = 0,9999 ([0,9998; 1] üzerinde dikdörtgen, u = 0,000058) ve V = 100,0 mL (üçgen, u = 0,07 mL); c = 1002,7 mg/L."),
   T("For a product and quotient the relative uncertainties add in quadrature (GUM 5.1.6): u(c)/c = √((0.05/100.28)² + (0.000058/0.9999)² + (0.07/100)²), u = 0.86 mg/L.",
     "Çarpım ve bölüm için bağıl belirsizlikler kareler toplamı olarak birleşir (GUM 5.1.6): u(c)/c = √((0,05/100,28)² + (0,000058/0,9999)² + (0,07/100)²), u = 0,86 mg/L."),
   T("The Monte Carlo method gives the same standard uncertainty and, to the digits printed in TN 1900, the same 95 % interval [1001.0, 1004.4] mg/L as with all inputs Gaussian. The triangular and rectangular inputs make the output slightly less spread in the tails: the Monte Carlo end points lie about 0.02–0.03 mg/L inside the GUM interval, so the GUM result is not validated for two significant digits of u (δ = 0.005 mg/L), although the difference has no practical importance.",
     "Monte Carlo yöntemi aynı standart belirsizliği ve TN 1900'de basılı basamaklara kadar, tüm girdiler normal dağılımlı alındığındakiyle aynı %95 aralığını, [1001,0; 1004,4] mg/L, verir. Üçgen ve dikdörtgen girdiler çıktının kuyruklarını biraz daraltır: Monte Carlo uç noktaları GUM aralığının yaklaşık 0,02–0,03 mg/L içindedir; bu nedenle GUM sonucu u'nun iki anlamlı basamağı için (δ = 0,005 mg/L) doğrulanmaz, ancak farkın pratik bir önemi yoktur.")],
  "chemistry", "Example E9", ["quam-a1", "triangular", "rectangular"],
  [T("TN 1900 describes V as a symmetric triangular distribution 'with mean 100 mL and standard deviation 0.7 mL'; only 0.07 mL, the value used in the same example and in QUAM A1, is consistent with the printed interval. The triangular half-width is 0.07 × √6 = 0.1715 mL.",
     "TN 1900, V'yi 'ortalaması 100 mL ve standart sapması 0,7 mL olan' simetrik üçgen dağılım olarak tanımlar; basılı aralıkla yalnızca aynı örnekte ve QUAM A1'de kullanılan 0,07 mL tutarlıdır. Üçgen yarı genişliği 0,07 × √6 = 0,1715 mL'dir."),
   T("The project 'quam-a1-cadmium-standard' builds the uncertainties of m, P and V from their components as in QUAM A1.",
     "'quam-a1-cadmium-standard' projesi m, P ve V'nin belirsizliklerini QUAM A1'deki gibi bileşenlerinden oluşturur.")],
  {"output": {"name": "c_Cd", "unit": "mg/L", "expression": "1000 * m * P / V", "description": T("Cadmium concentration of the standard", "Standardın kadmiyum derişimi")},
   "inputs": [inp("m", "mg", "Mass of cadmium", "Kadmiyum kütlesi", normal(100.28, 0.05)),
              inp("P", None, "Purity of the metal (mass fraction)", "Metalin saflığı (kütle kesri)", rect(0.9998, 1.0)),
              inp("V", "mL", "Volume of the solution", "Çözeltinin hacmi", tri_hw(100.0, round(0.07 * math.sqrt(6), 6)))]},
  {"gum": {}, "mcm": {"trials": 1000000, "seed": 9, "interval": "symmetric"}},
  {"gum": {"estimate": chk(1002.7, 0.05), "standard_uncertainty": chk(0.9, 0.05)},
   "mcm": {"interval_low": chk(1001.0, 0.05), "interval_high": chk(1004.4, 0.05), "gum_validated": False},
   "reference": T("NIST TN 1900 E9: c = 1002.7 mg/L, u ≈ 0.9 mg/L, Monte Carlo 95 % interval [1001.0, 1004.4] mg/L.",
                  "NIST TN 1900 E9: c = 1002,7 mg/L, u ≈ 0,9 mg/L, Monte Carlo %95 aralığı [1001,0; 1004,4] mg/L.")})

# ---------------------------------------------------------------- E11
project("library/electrical/nist-tn1900-e11-step-attenuator.json", "nist-tn1900-e11-step-attenuator",
  T("Microwave step attenuator, U-shaped inputs (NIST TN 1900 E11)", "Mikrodalga kademeli zayıflatıcı, U biçimli girdiler (NIST TN 1900 E11)"),
  T("TN 1900 E11: EA-4/02 S7 re-evaluated with the Monte Carlo method; the dominant arcsine mismatch term makes the distribution of a linear sum bimodal.",
    "TN 1900 E11: EA-4/02 S7 Monte Carlo yöntemiyle yeniden değerlendirilir; baskın arksinüs uyumsuzluk terimi doğrusal bir toplamın dağılımını iki tepeli yapar."),
  [T("L_X = L_S + δL_S + δL_D + δL_M + δL_K + δL_ib − δL_ia + δL_0b − δL_0a, the 30 dB step of the attenuator measured against a calibrated reference attenuator (EA-4/02 S7).",
     "L_X = L_S + δL_S + δL_D + δL_M + δL_K + δL_ib − δL_ia + δL_0b − δL_0a; zayıflatıcının 30 dB kademesi kalibre edilmiş bir referans zayıflatıcıya karşı ölçülür (EA-4/02 S7)."),
   T("L_S is the mean of four readings, 30.0402 dB, u = 0.0091 dB, modelled by TN 1900 as a t₃ variable rescaled to standard deviation 0.0091 dB. Mismatch (u = 0.02 dB), drift and leakage (±0.003 dB) are arcsine (U-shaped); δL_S and the reference detector resolutions are rectangular; the null detector terms are Gaussian (TN 1900 Exhibit 16).",
     "L_S dört okumanın ortalamasıdır, 30,0402 dB, u = 0,0091 dB; TN 1900 bunu standart sapması 0,0091 dB olacak şekilde ölçeklenmiş bir t₃ değişkeni olarak modeller. Uyumsuzluk (u = 0,02 dB), kayma ve sızıntı (±0,003 dB) arksinüs (U biçimli); δL_S ve referans dedektör çözünürlükleri dikdörtgen; sıfır dedektörü terimleri normal dağılımlıdır (TN 1900 Şekil 16)."),
   T("The Monte Carlo method gives 30.043 dB, u = 0.0224 dB and the 95 % interval [30.006, 30.081] dB. u²(δL_M) is 79 % of u²(L_X), so the output density has two modes: the conventional interval ± 2u is conservative (99 % effective coverage).",
     "Monte Carlo yöntemi 30,043 dB, u = 0,0224 dB ve %95 aralığı [30,006; 30,081] dB verir. u²(δL_M), u²(L_X)'in %79'udur; bu nedenle çıktı yoğunluğu iki tepelidir ve klasik ± 2u aralığı muhafazakârdır (etkin kapsam %99).")],
  "electrical", "Example E11, Exhibits 16 and 17", ["ea402-s7", "arcsine", "student-t", "bimodal"],
  [T("TN 1900 rescales the t₃ distribution of L_S to standard deviation u = 0.0091 dB (scale 0.0091/√3 dB). JCGM 101 6.4.9 instead uses the scale s/√n = 0.0091 dB, i.e. a standard deviation of 0.0158 dB. Because the GUM uncertainty of a t input is its scale, the GUM evaluation is switched off; the EA-4/02 GUM result is in 'ea402-s7-step-attenuator-30db'.",
     "TN 1900, L_S'nin t₃ dağılımını standart sapması u = 0,0091 dB olacak şekilde ölçekler (ölçek 0,0091/√3 dB). JCGM 101 6.4.9 ise ölçek olarak s/√n = 0,0091 dB alır, yani standart sapma 0,0158 dB olur. Bir t girdisinin GUM belirsizliği ölçeği olduğundan GUM değerlendirmesi kapatılmıştır; EA-4/02 GUM sonucu 'ea402-s7-step-attenuator-30db' projesindedir."),
   T("TN 1900 models the leakage δL_K as arcsine on ±0.003 dB (u = 0.0021 dB) and remarks that the 0.0017 dB of EA-4/02 S7.12 corresponds to a rectangular distribution, although EA-4/02 calls the term U-shaped.",
     "TN 1900, sızıntı terimi δL_K'yı ±0,003 dB üzerinde arksinüs olarak modeller (u = 0,0021 dB) ve EA-4/02 S7.12'deki 0,0017 dB'in dikdörtgen dağılıma karşılık geldiğini, oysa EA-4/02'nin terimi U biçimli olarak nitelediğini belirtir.")],
  {"output": {"name": "L_X", "unit": "dB", "expression": "L_S + dL_S + dL_D + dL_M + dL_K + dL_ib - dL_ia + dL_0b - dL_0a",
              "description": T("Attenuation of the 30 dB step", "30 dB kademesinin zayıflatması")},
   "inputs": [inp("L_S", "dB", "Mean of four attenuation differences", "Dört zayıflatma farkının ortalaması", t3(30.0402, 0.0091)),
              inp("dL_S", "dB", "Correction from the calibration of the reference attenuator", "Referans zayıflatıcının kalibrasyonundan düzeltme", rect_hw(0.003, round(0.0025 * math.sqrt(3), 7))),
              inp("dL_D", "dB", "Drift of the reference attenuator", "Referans zayıflatıcının kayması", arcsine_hw(0.0, round(0.0014 * math.sqrt(2), 7))),
              inp("dL_M", "dB", "Mismatch loss correction", "Uyumsuzluk kaybı düzeltmesi", arcsine_hw(0.0, round(0.02 * math.sqrt(2), 7))),
              inp("dL_K", "dB", "Leakage correction", "Sızıntı düzeltmesi", arcsine_hw(0.0, 0.003)),
              inp("dL_ia", "dB", "Resolution of the reference detector at 0 dB", "Referans dedektörün 0 dB'deki çözünürlüğü", rect_hw(0.0, round(0.0003 * math.sqrt(3), 7))),
              inp("dL_ib", "dB", "Resolution of the reference detector at 30 dB", "Referans dedektörün 30 dB'deki çözünürlüğü", rect_hw(0.0, round(0.0003 * math.sqrt(3), 7))),
              inp("dL_0a", "dB", "Resolution of the null detector at 0 dB", "Sıfır dedektörünün 0 dB'deki çözünürlüğü", normal(0.0, 0.002)),
              inp("dL_0b", "dB", "Resolution of the null detector at 30 dB", "Sıfır dedektörünün 30 dB'deki çözünürlüğü", normal(0.0, 0.002))]},
  {"gum": None, "mcm": {"trials": 2000000, "seed": 11, "interval": "symmetric", "coverage_probability": 0.95}},
  {"mcm": {"estimate": chk(30.043, 0.0005), "standard_uncertainty": chk(0.0224, 0.0002),
           "interval_low": chk(30.006, 0.0005), "interval_high": chk(30.081, 0.0005)},
   "reference": T("NIST TN 1900 E11: L_X = 30.043 dB, u = 0.0224 dB, 95 % interval [30.006, 30.081] dB (Monte Carlo, M = 10⁷).",
                  "NIST TN 1900 E11: L_X = 30,043 dB, u = 0,0224 dB, %95 aralığı [30,006; 30,081] dB (Monte Carlo, M = 10⁷).")},
  reporting={"significant_digits": 3})

# ---------------------------------------------------------------- E13
def e13(variant, p, lo, hi):
    student = variant == "student"
    dist = (lambda m, u: t3(m, u)) if student else normal
    pc = int(round(p * 100))
    pid = f"nist-tn1900-e13-thermal-expansion-{'student' if student else 'gaussian'}-{pc}"
    kind_en = "Student t₃ inputs" if student else "Gaussian inputs"
    kind_tr = "Student t₃ girdiler" if student else "normal girdiler"
    expl = [T("α = (L₁ − L₀)/(L₀(T₁ − T₀)) with L₀ = 1.4999 m at T₀ = 288.15 K and L₁ = 1.5021 m at T₁ = 373.10 K; u(L₀) = 0.0001 m, u(L₁) = 0.0002 m, u(T₀) = 0.02 K, u(T₁) = 0.05 K. The result is expressed in 10⁻⁶ K⁻¹.",
              "α = (L₁ − L₀)/(L₀(T₁ − T₀)); T₀ = 288,15 K'de L₀ = 1,4999 m ve T₁ = 373,10 K'de L₁ = 1,5021 m; u(L₀) = 0,0001 m, u(L₁) = 0,0002 m, u(T₀) = 0,02 K, u(T₁) = 0,05 K. Sonuç 10⁻⁶ K⁻¹ biriminde verilir."),
            T("The length difference, 2.2 mm, carries almost all the uncertainty: u(α) = 1.8 × 10⁻⁶ K⁻¹, about 10 % of α = 17.3 × 10⁻⁶ K⁻¹.",
              "Belirsizliğin neredeyse tamamı 2,2 mm'lik uzunluk farkından gelir: u(α) = 1,8 × 10⁻⁶ K⁻¹, yani α = 17,3 × 10⁻⁶ K⁻¹'in yaklaşık %10'u.")]
    if student:
        expl.append(T(f"Each estimate is taken as the mean of four repeated observations, so TN 1900 assigns t₃ distributions rescaled to standard deviations equal to the standard uncertainties. α and u(α) are unchanged, but the output is more peaked and heavier-tailed: the {pc} % interval is [{lo}, {hi}] × 10⁻⁶ K⁻¹.",
                      f"Her tahmin dört tekrar gözlemin ortalaması kabul edildiğinden TN 1900, standart sapmaları standart belirsizliklere eşit olacak şekilde ölçeklenmiş t₃ dağılımları atar. α ve u(α) değişmez, ancak çıktı daha sivri ve kuyrukları daha ağırdır: %{pc} aralığı [{str(lo).replace('.', ',')}; {str(hi).replace('.', ',')}] × 10⁻⁶ K⁻¹ olur."))
    else:
        expl.append(T(f"With Gaussian inputs the GUM and Monte Carlo results agree; the {pc} % interval from the percentiles of the Monte Carlo sample is [{lo}, {hi}] × 10⁻⁶ K⁻¹.",
                      f"Normal girdilerle GUM ve Monte Carlo sonuçları uyuşur; Monte Carlo örnekleminin yüzdeliklerinden %{pc} aralığı [{str(lo).replace('.', ',')}; {str(hi).replace('.', ',')}] × 10⁻⁶ K⁻¹ olur."))
    notes = [T("The companion projects nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} compare the two input models at 95 % and 99 %: with t₃ inputs the 95 % interval is shorter and the 99 % interval longer than with Gaussian inputs.",
               "Eşlik eden nist-tn1900-e13-thermal-expansion-{gaussian,student}-{95,99} projeleri iki girdi modelini %95 ve %99'da karşılaştırır: t₃ girdilerle %95 aralığı normal girdilere göre daha kısa, %99 aralığı ise daha uzundur.")]
    if student:
        notes.append(T("TN 1900 rescales each t₃ distribution to standard deviation u (scale u/√3), whereas JCGM 101 6.4.9 uses the scale s/√n = u. Because the GUM uncertainty of a t input is its scale, the GUM evaluation is switched off here; the Gaussian projects give the GUM result.",
                       "TN 1900 her t₃ dağılımını standart sapması u olacak şekilde ölçekler (ölçek u/√3); JCGM 101 6.4.9 ise ölçek olarak s/√n = u alır. Bir t girdisinin GUM belirsizliği ölçeği olduğundan burada GUM değerlendirmesi kapatılmıştır; GUM sonucu normal girdili projelerdedir."))
    expected = {"mcm": {"estimate": chk(17.3, 0.05), "standard_uncertainty": chk(1.8, 0.06 if student else 0.05),
                        "interval_low": chk(lo, 0.06), "interval_high": chk(hi, 0.06)},
                "reference": T(f"NIST TN 1900 E13 ({'Student' if student else 'Gaussian'} inputs): α = 1.73 × 10⁻⁵ K⁻¹, u = 0.18 × 10⁻⁵ K⁻¹, {pc} % interval ({lo / 10:.2f}, {hi / 10:.2f}) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷). Tolerances: half a unit of the printed digit plus the Monte Carlo dispersion" + ("; the sample standard deviation of t₃ inputs converges slowly (infinite fourth moment)." if student else "."),
                               f"NIST TN 1900 E13 ({'Student' if student else 'normal'} girdiler): α = 1,73 × 10⁻⁵ K⁻¹, u = 0,18 × 10⁻⁵ K⁻¹, %{pc} aralığı ({lo / 10:.2f}; {hi / 10:.2f}) × 10⁻⁵ K⁻¹ (Monte Carlo, M = 10⁷)".replace("0.", "0,").replace("1.", "1,").replace("2.", "2,") + ". Toleranslar: basılı son basamağın yarım birimi artı Monte Carlo saçılımı" + ("; t₃ girdilerde örneklem standart sapması yavaş yakınsar (dördüncü moment sonsuzdur)." if student else "."))}
    analysis = {"gum": None if student else {"coverage_probability": p},
                "mcm": {"trials": 4000000, "seed": 13, "interval": "symmetric", "coverage_probability": p}}
    if not student:
        expected["gum"] = {"estimate": chk(17.27, 0.005), "standard_uncertainty": chk(1.8, 0.05)}
    project(f"library/physical/{pid}.json", pid,
      T(f"Thermal expansion coefficient, {kind_en}, {pc} % (NIST TN 1900 E13)", f"Isıl genleşme katsayısı, {kind_tr}, %{pc} (NIST TN 1900 E13)"),
      T(f"TN 1900 E13: linear thermal expansion coefficient of a copper bar from two length measurements at two temperatures; {kind_en.lower()}, {pc} % coverage.",
        f"TN 1900 E13: bir bakır çubuğun doğrusal ısıl genleşme katsayısı, iki sıcaklıkta iki uzunluk ölçümünden; {kind_tr}, %{pc} kapsam."),
      expl, "physical", "Example E13, Exhibit 20", ["non-linear", "student-t" if student else "gaussian", "coverage-probability"], notes,
      {"output": {"name": "alpha", "unit": "10⁻⁶ K⁻¹", "expression": "1e6 * (L_1 - L_0) / (L_0 * (T_1 - T_0))",
                  "description": T("Linear thermal expansion coefficient of the copper bar", "Bakır çubuğun doğrusal ısıl genleşme katsayısı")},
       "inputs": [inp("L_0", "m", "Length at T₀", "T₀'daki uzunluk", dist(1.4999, 0.0001)),
                  inp("L_1", "m", "Length at T₁", "T₁'deki uzunluk", dist(1.5021, 0.0002)),
                  inp("T_0", "K", "Initial temperature", "Başlangıç sıcaklığı", dist(288.15, 0.02)),
                  inp("T_1", "K", "Final temperature", "Son sıcaklık", dist(373.10, 0.05))]},
      analysis, expected, reporting={"significant_digits": 2})

e13("gaussian", 0.95, 13.8, 20.7)
e13("gaussian", 0.99, 12.7, 21.8)
e13("student", 0.95, 14.0, 20.5)
e13("student", 0.99, 11.5, 23.0)
