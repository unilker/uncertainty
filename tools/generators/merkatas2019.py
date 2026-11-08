"""Writes the library project of Merkatas, Toman, Possolo & Schlamminger, Shades of dark uncertainty and
consensus value for the Newtonian constant of gravitation, Metrologia 56 (2019) 054001: Table 1 (16 measured
values) and the rows DL, ML, BG and LOP of Table 4. Run from the repository root.
Independent check: tools/research/consensus/merkatas2019.py."""
import json
from pathlib import Path

G = [("NIST-82", 6.67248, 0.00043), ("TR&D-96", 6.6729, 0.00050), ("LANL-97", 6.67398, 0.00070),
     ("UWash-00", 6.674255, 0.000092), ("BIPM-01", 6.67559, 0.00027), ("UWup-02", 6.67422, 0.00098),
     ("MSL-03", 6.67387, 0.00027), ("HUST-05", 6.67222, 0.00087), ("UZur-06", 6.67425, 0.00012),
     ("HUST-09", 6.67349, 0.00018), ("JILA-10", 6.67260, 0.00025), ("BIPM-14", 6.67554, 0.00016),
     ("LENS-14", 6.67191, 0.00099), ("UCI-14", 6.67435, 0.00013), ("HUST-TOS-18", 6.674184, 0.000078),
     ("HUST-AAF-18", 6.674484, 0.000078)]


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


p = {"format": "uncertainty-calculations/project", "version": 1,
     "metadata": {"id": "merkatas2019-newtonian-constant-g-shades", "field": "physical",
                  "title": T("Newtonian constant of gravitation from 16 measurements: DerSimonian–Laird with Knapp–Hartung, maximum likelihood, hierarchical Bayes and Linear Pool (Merkatas et al. 2019)",
                             "16 ölçümden Newton çekim sabiti: Knapp–Hartung düzeltmeli DerSimonian–Laird, en çok olabilirlik, hiyerarşik Bayes ve doğrusal havuz (Merkatas vd. 2019)"),
                  "description": T("The data of Koepke et al. (2017) with JILA-10 corrected (6.672 60 ± 0.000 25) and the two HUST results of 2018 (time-of-swing and angular-acceleration-feedback). Merkatas et al. compare fourteen procedures; four of them are implemented in the program.",
                                   "Koepke vd. (2017) verileri; JILA-10 düzeltilmiş (6,672 60 ± 0,000 25) ve 2018'deki iki HUST sonucu (salınım süresi ve açısal ivme geri beslemesi) eklenmiştir. Merkatas vd. on dört yöntemi karşılaştırır; bunların dördü programda vardır."),
                  "explanation": [T("DL (Table 4): DerSimonian–Laird with the Knapp–Hartung standard uncertainty and t₁₅ interval: 6.673 99, u 0.000 25, (6.673 46, 6.674 53), τ 0.000 56.",
                                    "DL (Tablo 4): Knapp–Hartung standart belirsizliği ve t₁₅ aralığıyla DerSimonian–Laird: 6,673 99, u 0,000 25, (6,673 46, 6,674 53), τ 0,000 56."),
                                  T("ML: Gaussian maximum likelihood, 6.673 90, u 0.000 25, τ 0.000 91; the moment estimate of DL gives a markedly smaller τ.",
                                    "ML: normal en çok olabilirlik, 6,673 90, u 0,000 25, τ 0,000 91; DL'nin moment kestirimi belirgin biçimde daha küçük bir τ verir."),
                                  T("BG: the NICOB hierarchical model (MCMC in the paper, exact grid here): 6.673 89, u 0.000 27, (6.673 33, 6.674 42), posterior median of τ 0.000 95.",
                                    "BG: NICOB hiyerarşik modeli (makalede MCMC, burada kesin ızgara): 6,673 89, u 0,000 27, (6,673 33, 6,674 42), τ'nun sonsal ortancası 0,000 95."),
                                  T("LOP: Linear Pool, 6.673 77, u 0.001 17, (6.671 27, 6.675 77). The spread of the four consensus values (0.000 22) is comparable to their standard uncertainties: the choice of procedure is itself a source of uncertainty.",
                                    "LOP: doğrusal havuz, 6,673 77, u 0,001 17, (6,671 27, 6,675 77). Dört uzlaşı değerinin yayılımı (0,000 22) standart belirsizlikleriyle karşılaştırılabilir: yöntem seçimi kendisi bir belirsizlik kaynağıdır.")],
                  "tags": ["merkatas-2019", "nicob", "consensus", "dark-uncertainty", "dersimonian-laird", "knapp-hartung", "maximum-likelihood", "bayes", "linear-pool", "newtonian-constant"],
                  "notes": [T("Not implemented: Birge's multiplicative models (BRE, BRM, BRQ), the CODATA expansion (MTE), Mandel–Paule, REML, Student-t effects, the Laplace model of the paper (Laplace effects and errors), and the Bayesian mixture model (BMM).",
                              "Uygulanmayanlar: Birge çarpımsal modelleri (BRE, BRM, BRQ), CODATA genişletmesi (MTE), Mandel–Paule, REML, Student-t etkileri, makalenin Laplace modeli (Laplace etkileri ve hataları) ve Bayesçi karışım modeli (BMM).")],
                  "sources": [{"document": "Merkatas, Toman, Possolo & Schlamminger, Shades of dark uncertainty and consensus value for the Newtonian constant of gravitation, Metrologia 56 (2019) 054001", "section": "Tables 1 and 4", "url": "https://doi.org/10.1088/1681-7575/ab3365"}]},
     "consensus": {"measurand": "G", "unit": "10⁻¹¹ m³ kg⁻¹ s⁻²",
                   "results": [{"name": n, "value": v, "u": s} for n, v, s in G],
                   "linear_pool": True, "maximum_likelihood": True, "bayes": {}, "trials": 100000, "seed": 1},
     "expected": {"consensus": {"mean": chk(6.67399, 0.000006), "u_knapp_hartung": chk(0.00025, 0.000006), "tau": chk(0.00056, 0.000006),
                                "ml_mean": chk(6.67390, 0.000006), "ml_u": chk(0.00025, 0.000006),
                                "bayes_mean": chk(6.67389, 0.00001), "bayes_u": chk(0.00027, 0.000006),
                                "bayes_lower": chk(6.67333, 0.00002), "bayes_upper": chk(6.67442, 0.00002),
                                "bayes_tau_median": chk(0.00095, 0.00002),
                                "linear_pool_mean": chk(6.67377, 0.000008), "linear_pool_u": chk(0.00117, 0.000008),
                                "linear_pool_lower": chk(6.67127, 0.00003), "linear_pool_upper": chk(6.67577, 0.00003)},
                  "reference": T("Merkatas et al. (2019), Table 4, rows DL, ML, BG and LOP. ML u from the paper's table; BG and LOP from MCMC and Monte Carlo (tolerances: half a unit plus the sampling error of MCMC, about 3 × 10⁻⁶ for the mean and 10⁻⁵ for the limits; exact grid 6.673 896, 0.000 271, (6.673 342, 6.674 415), median τ 0.000 936).",
                                 "Merkatas vd. (2019), Tablo 4, DL, ML, BG ve LOP satırları. BG ve LOP MCMC ve Monte Carlo'dan (toleranslar: yarım birim artı MCMC örnekleme hatası, ortalama için yaklaşık 3 × 10⁻⁶, sınırlar için 10⁻⁵; kesin ızgara 6,673 896, 0,000 271, (6,673 342, 6,674 415), τ ortancası 0,000 936).")}}
Path("library/physical/merkatas2019-newtonian-constant-g-shades.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("wrote merkatas2019-newtonian-constant-g-shades")
