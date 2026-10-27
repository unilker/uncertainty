"""ISO 13528:2022 E.1 (censored results) and E.5 (reference value from a CRM) library projects."""
import json, math
from pathlib import Path

DOC = "ISO 13528:2022, Statistical methods for use in proficiency testing by interlaboratory comparison"
URL = "https://www.iso.org/standard/78879.html"
def T(en, tr): return {"en": en, "tr": tr}
def chk(v, t): return {"value": v, "tolerance": t}

def write(pid, body, kind, title, desc, expl, sec, tags, notes, expected, ref, extra=None):
    p = {"format": "uncertainty-calculations/project", "version": 1,
         "metadata": {"id": pid, "title": title, "description": desc, "explanation": expl, "field": "chemistry",
                      "sources": [{"document": DOC, "section": sec, "url": URL}], "tags": ["iso13528", "proficiency-testing"] + tags}}
    if notes: p["metadata"]["notes"] = notes
    p[kind] = body
    if extra: p.update(extra)
    p["expected"] = {**expected, "reference": ref}
    Path(f"library/chemistry/{pid}.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n")

# E.1 — 23 results, 5 censored
E1 = [("A", "<", 10), ("B", "<", 10), ("C", "", 12), ("D", "", 19), ("E", "<", 20), ("F", "", 20), ("G", "", 23), ("H", "", 23),
      ("J", "", 25), ("K", "", 25), ("L", "", 26), ("M", "", 28), ("N", "", 28), ("P", "<", 30), ("Q", "", 28), ("R", "", 29),
      ("S", "", 30), ("T", "", 30), ("U", "", 31), ("V", "", 32), ("W", "", 32), ("Y", "", 45), ("Z", "<", 50)]
variants = [
  ("ignored", "‘<’ ignored", "‘<’ işareti yok sayılarak", lambda c, v: {"value": v}, 26.01, 7.23, 23,
   T("The ‘<’ signs are discarded and the five limits analysed as quantitative results (10, 10, 20, 30, 50).",
     "‘<’ işaretleri atılır ve beş sınır nicel sonuç olarak işlenir (10, 10, 20, 30, 50).")),
  ("deleted", "censored results deleted", "sansürlü sonuçlar çıkarılarak", lambda c, v: {"less_than": v}, 26.81, 5.29, 18,
   T("The five censored results are reported as ‘less than’ and excluded from the statistics (5.5.3), leaving p = 18.",
     "Beş sansürlü sonuç ‘küçüktür’ olarak bildirilir ve istatistiklerin dışında kalır (5.5.3); p = 18 kalır.")),
  ("half", "0.5 × the limit", "sınırın 0,5 katı", lambda c, v: {"value": 0.5 * v}, 23.95, 8.60, 23,
   T("Each censored result is replaced by half its limit (5, 5, 10, 15, 25).",
     "Her sansürlü sonuç sınırının yarısıyla değiştirilir (5, 5, 10, 15, 25).")),
]
for key, en, tr, conv, xs, ss, p_n, step in variants:
    results = [dict(lab=l, **(conv(c, v) if c == "<" else {"value": v})) for l, c, v in E1]
    half = key == "half"
    write(f"iso13528-e1-censored-{key}",
      {"quantity": "result", "results": results, "assigned_value": {"method": "algorithm_a"}, "sd_pt": {"method": "robust"},
       **({"algorithm_a": "converged"} if half else {})},
      "proficiency",
      T(f"Censored results in a PT round: {en} (ISO 13528 E.1)", f"Bir YT turunda sansürlü sonuçlar: {tr} (ISO 13528 E.1)"),
      T("ISO 13528:2022 E.1: 23 results of which five are ‘less than’ values; the robust average and standard deviation of Algorithm A depend on how the censored results are treated.",
        "ISO 13528:2022 E.1: beşi ‘küçüktür’ değeri olan 23 sonuç; Algoritma A'nın robust ortalaması ve standart sapması sansürlü sonuçların nasıl işlendiğine bağlıdır."),
      [T("23 participants, five of them reporting ‘<10’, ‘<10’, ‘<20’, ‘<30’ and ‘<50’ (Table E.1).",
         "23 katılımcı; beşi ‘<10’, ‘<10’, ‘<20’, ‘<30’ ve ‘<50’ bildirmiştir (Tablo E.1)."),
       step,
       T(f"Algorithm A (C.3.1) on the p = {p_n} values: x* = {xs:.2f}, s* = {ss:.2f}. The three treatments give x* from 23.95 to 26.81 and s* from 5.29 to 8.60: the proficiency testing provider has to decide the treatment beforehand (5.5.3.3).",
         f"p = {p_n} değer üzerinde Algoritma A (C.3.1): x* = {xs:.2f}, s* = {ss:.2f}".replace(".", ",") + ". Üç yaklaşım x* için 23,95–26,81 ve s* için 5,29–8,60 verir: yeterlilik testi sağlayıcısı yaklaşımı önceden belirlemelidir (5.5.3.3).")],
      "Annex E.1 (Table E.1)", ["censored-results", "algorithm-a"],
      [T("Table E.1 marks with ‘#’ the results outside x* ± 3s*; with the robust values here the z scores of those results exceed 3 in absolute value (Z = 50 when the ‘<’ is ignored, Y = 45 when the censored results are deleted).",
         "Tablo E.1, x* ± 3s* dışındaki sonuçları ‘#’ ile işaretler; buradaki robust değerlerle bu sonuçların z skorlarının mutlak değeri 3'ü aşar (‘<’ yok sayıldığında Z = 50, sansürlü sonuçlar çıkarıldığında Y = 45).")]
      + ([T("With the stopping rule of C.3.1 (third significant figure) Algorithm A stops after 11 iterations at x* = 23.96, s* = 8.59; iterated to convergence it gives x* = 23.959, s* = 8.600. Table E.1 prints 23.95 and 8.60, i.e. the converged values with x* truncated rather than rounded (research log F44). The project iterates to convergence and checks x* within 0.01.",
            "C.3.1'in durma kuralıyla (üçüncü anlamlı basamak) Algoritma A 11 iterasyondan sonra x* = 23,96, s* = 8,59'da durur; yakınsamaya kadar sürdürüldüğünde x* = 23,959, s* = 8,600 verir. Tablo E.1 23,95 ve 8,60 basar; yani yakınsamış değerler, x* yuvarlanmak yerine kesilmiş (araştırma günlüğü F44). Proje yakınsamaya kadar iterasyon yapar ve x*'ı 0,01 içinde kontrol eder.")] if half else []),
      {"proficiency": {"robust_mean": chk(xs, 0.01 if half else 0.005), "robust_sd": chk(ss, 0.005)}},
      T(f"ISO 13528:2022 Table E.1 (x* = {xs:.2f}, s* = {ss:.2f}).", f"ISO 13528:2022 Tablo E.1 (x* = {xs:.2f}, s* = {ss:.2f}).".replace(".", ",").replace("E,1", "E.1").replace("13528:2022", "13528:2022")))

# E.5 — reference value from a CRM
pt = [(20.5, 20.5), (21.1, 20.7), (21.5, 21.5), (22.3, 21.7), (22.7, 22.3), (23.6, 22.4), (20.9, 21.2), (21.4, 21.5), (23.5, 23.5), (22.3, 22.9),
      (23.5, 24.1), (22.5, 23.5), (22.5, 23.5), (23.4, 22.7), (24.0, 24.2), (24.5, 24.4), (24.8, 24.7), (24.7, 25.1), (24.9, 24.4), (27.2, 27.0)]
crm = [(19.0, 18.0), (19.8, 19.9), (21.0, 21.0), (21.0, 20.8), (20.5, 21.0), (20.3, 20.3), (21.5, 21.8), (21.9, 21.7), (21.0, 21.0), (22.0, 21.3),
       (20.8, 20.6), (21.0, 22.0), (21.0, 21.0), (22.0, 22.0), (22.1, 21.5), (22.3, 22.5), (22.0, 21.9), (21.9, 21.9), (22.4, 22.6), (24.5, 23.7)]
d = [round((a + b) / 2 - (c + e) / 2, 2) for (a, b), (c, e) in zip(pt, crm)]
n = len(d); m = sum(d) / n; s = math.sqrt(sum((x - m) ** 2 for x in d) / (n - 1)); u = math.sqrt(0.26**2 + s**2 / n)
write("iso13528-e5-los-angeles-reference-value",
  {"output": {"name": "x_pt", "unit": "LA units", "expression": "x_CRM + d"},
   "inputs": [{"name": "x_CRM", "unit": "LA units", "description": T("certified value of the CRM", "CRM'nin sertifikalı değeri"),
               "distribution": {"type": "normal", "mean": 21.62, "sd": 0.26}},
              {"name": "d", "unit": "LA units", "description": T("average difference PT item − CRM over 20 samples", "20 numunede ortalama fark YT numunesi − CRM"),
               "distribution": {"type": "type_a", "observations": d}}]},
  "model",
  T("Assigned value from a comparison with a CRM: Los Angeles value of aggregates (ISO 13528 E.5)", "CRM ile karşılaştırmadan atanan değer: agregaların Los Angeles değeri (ISO 13528 E.5)"),
  T("ISO 13528:2022 E.5 (7.5): the proficiency test item and a similar CRM are tested in duplicate on 20 samples; x_pt is the certified value plus the average difference, u(x_pt) combines the CRM uncertainty and the standard uncertainty of the difference.",
    "ISO 13528:2022 E.5 (7.5): yeterlilik testi numunesi ve benzer bir CRM 20 numunede ikişer kez test edilir; x_pt sertifikalı değer artı ortalama farktır, u(x_pt) CRM belirsizliğini ve farkın standart belirsizliğini birleştirir."),
  [T("Certified value of the CRM 21.62 LA units with standard uncertainty 0.26 LA units, which already includes inhomogeneity, transport and long-term stability.",
     "CRM'nin sertifikalı değeri 21,62 LA birimi, standart belirsizliği 0,26 LA birimi; bu belirsizlik homojensizlik, taşıma ve uzun süreli kararlılığı zaten içerir."),
   T(f"Differences of the duplicate means, PT item − CRM, for the 20 samples (Table E.8): mean d = {m:.3f}, s = {s:.3f}, u(d) = s/√20 = {s / math.sqrt(n):.3f} LA units.",
     f"20 numune için ikili ortalamaların farkları, YT numunesi − CRM (Tablo E.8): ortalama d = {m:.3f}, s = {s:.3f}, u(d) = s/√20 = {s / math.sqrt(n):.3f} LA birimi.".replace(".", ",").replace("E,8", "E.8")),
   T(f"x_pt = 21.62 + {m:.2f} = {21.62 + m:.2f} LA units and u(x_pt) = √(0.26² + {s / math.sqrt(n):.3f}²) = {u:.2f} LA units.",
     f"x_pt = 21,62 + {m:.2f} = {21.62 + m:.2f} LA birimi ve u(x_pt) = √(0,26² + {s / math.sqrt(n):.3f}²) = {u:.2f} LA birimi.".replace(".", ","))],
  "Annex E.5 (Table E.8)", ["reference-value", "crm"],
  [T("The differences are recomputed from the duplicate results of Table E.8 and agree with its last column. The Monte Carlo method uses the scaled and shifted t distribution with 19 degrees of freedom for d (JCGM 101 6.4.9).",
     "Farklar Tablo E.8'deki ikili sonuçlardan yeniden hesaplanır ve tablonun son sütunuyla uyumludur. Monte Carlo yöntemi d için 19 serbestlik dereceli ölçeklenmiş ve kaydırılmış t dağılımını kullanır (JCGM 101 6.4.9).")],
  {"gum": {"estimate": chk(23.35, 0.005), "standard_uncertainty": chk(0.35, 0.005)}},
  T("ISO 13528:2022 E.5 (x_pt = 23.35, u(x_pt) = 0.35 LA units; Table E.8: d = 1.73, s = 1.07, u(d) = 0.24).",
    "ISO 13528:2022 E.5 (x_pt = 23,35, u(x_pt) = 0,35 LA birimi; Tablo E.8: d = 1,73, s = 1,07, u(d) = 0,24)."),
  extra={"analysis": {"gum": {"coverage_factor": 2}, "mcm": {"trials": 1000000, "seed": 13528}}, "reporting": {"significant_digits": 2}})
print(d, m, s, u)
