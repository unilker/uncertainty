"""Writes the JCGM 106:2012 library projects (run from the repository root)."""
import json
from pathlib import Path

DOC = "JCGM 106:2012, Evaluation of measurement data — The role of measurement uncertainty in conformity assessment"
URL = "https://www.bipm.org/documents/20126/2071204/JCGM_106_2012_E.pdf"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}

def project(name, title, desc, expl, sec, tags, notes, body, expected, ref):
    pid = f"jcgm106-{name}"
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "general",
                      "sources": [{"document": DOC, "section": sec, "url": URL}], "tags": ["jcgm106", "conformity-assessment"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p["conformity"] = body
    p["expected"] = {"conformity": expected, "reference": ref}
    Path(f"library/general/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

project("7.3-zener-diode",
  T("Breakdown voltage of a Zener diode: probability of conformity (JCGM 106 7.3)", "Zener diyotun kırılma gerilimi: uygunluk olasılığı (JCGM 106 7.3)"),
  T("JCGM 106:2012 7.3 Example 1: single upper tolerance limit V_b ≤ −5.40 V, measured −5.47 V with u = 0.05 V.",
    "JCGM 106:2012 7.3 Örnek 1: tek üst tolerans sınırı V_b ≤ −5,40 V, ölçülen −5,47 V, u = 0,05 V."),
  [T("Knowledge of V_b after the measurement is a normal PDF with expectation −5.47 V and standard deviation 0.05 V.",
     "Ölçümden sonra V_b hakkındaki bilgi, beklenen değeri −5,47 V ve standart sapması 0,05 V olan normal bir olasılık yoğunluğudur."),
   T("For an upper limit z = (T_U − y)/u = (−5.40 + 5.47)/0.05 = 1.40 and p_c = Φ(1.40) = 0.92 (Formulae 9, 10): a 92 % probability that the diode conforms.",
     "Üst sınır için z = (T_U − y)/u = (−5,40 + 5,47)/0,05 = 1,40 ve p_c = Φ(1,40) = 0,92 (Formül 9, 10): diyotun uygun olma olasılığı %92'dir.")],
  "7.3.3, Example 1", ["probability-of-conformity"], [],
  {"quantity": "V_b", "unit": "V", "upper": -5.40, "results": [{"name": "diode", "value": -5.47, "u": 0.05}]},
  {"results": {"diode": {"conformance_probability": chk(0.92, 0.005), "accepted": True}}},
  T("JCGM 106:2012 7.3, Example 1 (p_c = 0.92).", "JCGM 106:2012 7.3, Örnek 1 (p_c = 0,92)."))

project("7.3-bursting-strength",
  T("Bursting strength of a container: probability of conformity (JCGM 106 7.3)", "Bir kabın patlama dayanımı: uygunluk olasılığı (JCGM 106 7.3)"),
  T("JCGM 106:2012 7.3 Example 2: single lower tolerance limit B ≥ 490 kPa, destructive test b = 509.7 kPa with u = 8.6 kPa.",
    "JCGM 106:2012 7.3 Örnek 2: tek alt tolerans sınırı B ≥ 490 kPa, tahribatlı deney b = 509,7 kPa, u = 8,6 kPa."),
  [T("For a lower limit z = (y − T_L)/u = (509.7 − 490)/8.6 = 2.3 and p_c = Φ(2.3) = 0.99 (Formulae 8, 10).",
     "Alt sınır için z = (y − T_L)/u = (509,7 − 490)/8,6 = 2,3 ve p_c = Φ(2,3) = 0,99 (Formül 8, 10)."),
   T("The probability refers to the container before the test destroyed it: there is a 99 % probability that it conformed.",
     "Olasılık, deneyin yok ettiği kabın deneyden önceki durumuna aittir: uygun olmuş olma olasılığı %99'dur.")],
  "7.3.3, Example 2", ["probability-of-conformity"], [],
  {"quantity": "B", "unit": "kPa", "lower": 490, "results": [{"name": "container", "value": 509.7, "u": 8.6}]},
  {"results": {"container": {"conformance_probability": chk(0.99, 0.005), "accepted": True}}},
  T("JCGM 106:2012 7.3, Example 2 (p_c = 0.99).", "JCGM 106:2012 7.3, Örnek 2 (p_c = 0,99)."))

project("7.4-motor-oil-viscosity",
  T("Kinematic viscosity of a motor oil: two-sided tolerance (JCGM 106 7.4)", "Motor yağının kinematik viskozitesi: iki taraflı tolerans (JCGM 106 7.4)"),
  T("JCGM 106:2012 7.4 Example: SAE grade 40 oil at 100 °C, 12.5 mm²/s ≤ Y ≤ 16.3 mm²/s, measured 13.6 mm²/s with u = 1.8 mm²/s.",
    "JCGM 106:2012 7.4 Örnek: 100 °C'de SAE 40 yağ, 12,5 mm²/s ≤ Y ≤ 16,3 mm²/s, ölçülen 13,6 mm²/s, u = 1,8 mm²/s."),
  [T("p_c = Φ((T_U − y)/u) − Φ((T_L − y)/u) = Φ(1.5) − Φ(−0.61) = 0.93 − 0.27 = 0.66 (Formula 11).",
     "p_c = Φ((T_U − y)/u) − Φ((T_L − y)/u) = Φ(1,5) − Φ(−0,61) = 0,93 − 0,27 = 0,66 (Formül 11)."),
   T("The measured value lies in the tolerance interval and the sample is accepted under simple acceptance, but the probability of conformity is only 66 %: the uncertainty is large compared with the tolerance (C_m = 3.8/(4 × 1.8) = 0.53).",
     "Ölçülen değer tolerans aralığındadır ve basit kabulle numune kabul edilir; ancak uygunluk olasılığı yalnızca %66'dır: belirsizlik toleransa göre büyüktür (C_m = 3,8/(4 × 1,8) = 0,53).")],
  "7.4, Example", ["probability-of-conformity", "capability-index"], [],
  {"quantity": "Y", "unit": "mm²/s", "lower": 12.5, "upper": 16.3, "u": 1.8, "results": [{"name": "oil", "value": 13.6}]},
  {"results": {"oil": {"conformance_probability": chk(0.66, 0.005), "accepted": True}}, "capability_index": chk(0.528, 0.0005)},
  T("JCGM 106:2012 7.4, Example (p_c = 0.66); C_m from Formula (12).", "JCGM 106:2012 7.4, Örnek (p_c = 0,66); C_m Formül (12)'den."))

project("8.3-speed-limit",
  T("Speed limit enforcement: guarded rejection (JCGM 106 8.3.3)", "Hız sınırı denetimi: korumalı ret (JCGM 106 8.3.3)"),
  T("JCGM 106:2012 8.3.3 Example 1: speed limit 100 km/h, radar with relative standard uncertainty 2 %; a ticket only if the probability that the limit was exceeded is at least 99.9 %.",
    "JCGM 106:2012 8.3.3 Örnek 1: hız sınırı 100 km/h, bağıl standart belirsizliği %2 olan radar; yalnızca sınırın aşılmış olma olasılığı en az %99,9 ise ceza."),
  [T("Knowledge of the speed for a measured v is a normal PDF with standard deviation 0.02v.",
     "Ölçülen v için hız hakkındaki bilgi, standart sapması 0,02v olan normal bir olasılık yoğunluğudur."),
   T("The acceptance limit v_max satisfies (v_max − v₀)/(0.02 v_max) = z = 3.09, so v_max = v₀/(1 − 0.02z) = 100/(1 − 0.0618) ≈ 107 km/h.",
     "Kabul sınırı v_max, (v_max − v₀)/(0,02 v_max) = z = 3,09 eşitliğini sağlar; dolayısıyla v_max = v₀/(1 − 0,02z) = 100/(1 − 0,0618) ≈ 107 km/h."),
   T("The interval 100 km/h to 107 km/h is a guard band: a measured speed of 107 km/h or more gives at least 99.9 % probability that the limit was exceeded.",
     "100 km/h – 107 km/h aralığı bir koruma bandıdır: 107 km/h veya daha yüksek ölçülen hız, sınırın aşılmış olma olasılığını en az %99,9 yapar.")],
  "8.3.3, Example 1", ["guard-band", "guarded-rejection"],
  [T("The program solves the condition exactly with z = Φ⁻¹(0.999) = 3.0902, giving 106.59 km/h; the standard rounds to 107 km/h. The measured speeds 105 and 108 km/h illustrate both sides of the acceptance limit.",
     "Program koşulu z = Φ⁻¹(0,999) = 3,0902 ile tam çözer ve 106,59 km/h verir; standart 107 km/h'ye yuvarlar. 105 ve 108 km/h ölçülen hızlar kabul sınırının iki yanını gösterir.")],
  {"quantity": "v", "unit": "km/h", "upper": 100, "rule": "guarded_rejection", "probability": 0.999, "u_relative": 0.02,
   "results": [{"name": "car A", "value": 105}, {"name": "car B", "value": 108}]},
  {"acceptance_upper": chk(107, 0.5), "results": {"car A": {"accepted": True}, "car B": {"accepted": False}}},
  T("JCGM 106:2012 8.3.3, Example 1 (v_max ≈ 107 km/h).", "JCGM 106:2012 8.3.3, Örnek 1 (v_max ≈ 107 km/h)."))

project("8.3-nandrolone",
  T("Nandrolone in animal products: decision limit for a screening test (JCGM 106 8.3.3)", "Hayvansal ürünlerde nandrolon: tarama testi için karar sınırı (JCGM 106 8.3.3)"),
  T("JCGM 106:2012 8.3.3 Example 2: threshold 2.00 µg/L; a result is suspect if the probability that the threshold is exceeded is at least 95 %; reproducibility s = 0.20 µg/L from ten spiked blanks, t distribution with ν = 9.",
    "JCGM 106:2012 8.3.3 Örnek 2: eşik 2,00 µg/L; eşiğin aşılmış olma olasılığı en az %95 ise sonuç şüphelidir; on katkılı kör numuneden tekrar üretilebilirlik s = 0,20 µg/L, ν = 9 ile t dağılımı."),
  [T("The PDF for the concentration is a scaled and shifted t distribution with scale s = 0.20 µg/L and ν = 9 degrees of freedom (JCGM 101 6.4.9).",
     "Derişimin olasılık yoğunluğu, ölçeği s = 0,20 µg/L ve serbestlik derecesi ν = 9 olan ölçeklenmiş ve kaydırılmış t dağılımıdır (JCGM 101 6.4.9)."),
   T("Guard band g = t₀.₉₅;₉ × s = 1.83 × 0.20 = 0.37 µg/L and decision limit A = T + g = 2.37 µg/L (guarded rejection).",
     "Koruma bandı g = t₀,₉₅;₉ × s = 1,83 × 0,20 = 0,37 µg/L ve karar sınırı A = T + g = 2,37 µg/L (korumalı ret).")],
  "8.3.3, Example 2", ["guard-band", "guarded-rejection", "t-distribution"], [],
  {"quantity": "c", "unit": "µg/L", "upper": 2.00, "rule": "guarded_rejection", "probability": 0.95, "u": 0.20, "dof": 9},
  {"acceptance_upper": chk(2.37, 0.005)},
  T("JCGM 106:2012 8.3.3, Example 2 (A = 2.37 µg/L).", "JCGM 106:2012 8.3.3, Örnek 2 (A = 2,37 µg/L)."))

project("9.5-precision-resistors",
  T("Precision resistors: global consumer's and producer's risks (JCGM 106 9.5.3)", "Hassas dirençler: küresel tüketici ve üretici riskleri (JCGM 106 9.5.3)"),
  T("JCGM 106:2012 9.5.3 Example: 1500 Ω ± 0.2 Ω resistors from a process with σ = 0.12 Ω, inspected with u_m = 0.04 Ω and acceptance limits 1499.82 Ω and 1500.18 Ω.",
    "JCGM 106:2012 9.5.3 Örnek: σ = 0,12 Ω olan bir süreçten 1500 Ω ± 0,2 Ω dirençler, u_m = 0,04 Ω ile ve 1499,82 Ω ile 1500,18 Ω kabul sınırlarıyla denetlenir."),
  [T("Without inspection p_c = ∫ φ(η; 1500, 0.12²) dη over the tolerance interval = 0.90: about 10 % of the resistors would not conform (Formula 21).",
     "Denetim olmadan p_c = tolerans aralığı üzerinde ∫ φ(η; 1500, 0,12²) dη = 0,90: dirençlerin yaklaşık %10'u uygun olmazdı (Formül 21)."),
   T("Guarded acceptance with symmetric guard bands w = 0.02 Ω = 0.25U.",
     "w = 0,02 Ω = 0,25U simetrik koruma bantlarıyla korumalı kabul."),
   T("Formulae (19) and (20) by numerical integration: R_C = 1 % and R_P = 7 %. Of 100 resistors 83 are accepted correctly, 7 rejected falsely, 9 rejected correctly and 1 accepted falsely; of the accepted resistors 99 % conform.",
     "Formül (19) ve (20) sayısal integrasyonla: R_C = %1 ve R_P = %7. 100 dirençten 83'ü doğru olarak kabul, 7'si yanlışlıkla ret, 9'u doğru olarak ret, 1'i yanlışlıkla kabul edilir; kabul edilenlerin %99'u uygundur.")],
  "9.5.3, Example", ["global-risk", "guarded-acceptance"], [],
  {"quantity": "R", "unit": "Ω", "lower": 1499.8, "upper": 1500.2, "acceptance_lower": 1499.82, "acceptance_upper": 1500.18, "u": 0.04,
   "process": {"distribution": "normal", "mean": 1500, "sd": 0.12}},
  {"conforming": chk(0.90, 0.005), "consumer_risk": chk(0.01, 0.0005), "producer_risk": chk(0.07, 0.005)},
  T("JCGM 106:2012 9.5.3, Example (p_c ≈ 90 %, R_C ≈ 1 %, R_P ≈ 7 %).", "JCGM 106:2012 9.5.3, Örnek (p_c ≈ %90, R_C ≈ %1, R_P ≈ %7)."))

project("9.5-ball-bearings",
  T("Ball bearings: acceptance limit for a required consumer's risk (JCGM 106 9.5.4)", "Bilyalı rulmanlar: istenen tüketici riski için kabul sınırı (JCGM 106 9.5.4)"),
  T("JCGM 106:2012 9.5.4 Example: radial error motion ≤ 2 µm, gamma process PDF with mean 1 µm and standard deviation 0.5 µm, measuring system u_m = 0.25 µm; global consumer's risk at most 0.1 %.",
    "JCGM 106:2012 9.5.4 Örnek: radyal hata hareketi ≤ 2 µm, ortalaması 1 µm ve standart sapması 0,5 µm olan gama süreç dağılımı, ölçüm sistemi u_m = 0,25 µm; küresel tüketici riski en çok %0,1."),
  [T("The radial error motion is positive, so the process PDF is a gamma density with α = (1/0.5)² = 4 and λ = 1/0.5² = 4 µm⁻¹ (Annex B, Formula 22); 4.2 % of the bearings exceed 2 µm.",
     "Radyal hata hareketi pozitiftir; bu nedenle süreç dağılımı α = (1/0,5)² = 4 ve λ = 1/0,5² = 4 µm⁻¹ olan bir gama yoğunluğudur (Ek B, Formül 22); rulmanların %4,2'si 2 µm'yi aşar."),
   T("With A = T − 2r u_m, R_C(r) and R_P(r) follow from Formulae (23) and (24); R_C = 0.1 % is reached for r ≈ 0.65, so A ≈ 1.7 µm, with R_P ≈ 7.5 %.",
     "A = T − 2r u_m ile R_C(r) ve R_P(r), Formül (23) ve (24)'ten bulunur; R_C = %0,1, r ≈ 0,65 için sağlanır; dolayısıyla A ≈ 1,7 µm ve R_P ≈ %7,5'tir.")],
  "9.5.4, Example", ["global-risk", "guarded-acceptance", "gamma-prior"],
  [T("The text states the acceptance interval 0 ≤ Y_m ≤ A and Formula (23) for R_C includes the lower limit 0, but Formula (24) for R_P omits it. The printed R_P ≈ 7.5 % follows from (24); with the lower acceptance limit 0, R_P would be 8.9 % (research log F45). The project uses the acceptance interval Y_m ≤ A, which reproduces both R_C and R_P (the lower limit has no effect on R_C).",
     "Metin kabul aralığını 0 ≤ Y_m ≤ A olarak verir ve R_C için Formül (23) alt sınır 0'ı içerir; ancak R_P için Formül (24) bunu dışarıda bırakır. Basılı R_P ≈ %7,5 değeri (24)'ten çıkar; alt kabul sınırı 0 ile R_P %8,9 olurdu (araştırma günlüğü F45). Proje Y_m ≤ A kabul aralığını kullanır; bu hem R_C'yi hem R_P'yi yeniden üretir (alt sınırın R_C'ye etkisi yoktur).")],
  {"quantity": "radial error motion", "unit": "µm", "upper": 2.0, "rule": "guarded_acceptance", "target_consumer_risk": 0.001, "u": 0.25,
   "process": {"distribution": "gamma", "mean": 1.0, "sd": 0.5}},
  {"conforming": chk(0.958, 0.0005), "r": chk(0.65, 0.01), "acceptance_upper": chk(1.7, 0.05), "consumer_risk": chk(0.001, 0.00001), "producer_risk": chk(0.075, 0.001)},
  T("JCGM 106:2012 9.5.4, Example (4.2 % non-conforming, r ≈ 0.65, A ≈ 1.7 µm, R_P ≈ 7.5 %).", "JCGM 106:2012 9.5.4, Örnek (%4,2 uygunsuz, r ≈ 0,65, A ≈ 1,7 µm, R_P ≈ %7,5)."))
