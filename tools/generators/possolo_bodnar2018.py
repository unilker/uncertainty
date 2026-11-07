"""Writes the library project of Possolo & Bodnar, Approximate Bayesian evaluations of measurement uncertainty,
Metrologia 55 (2018) 147, section 4.4: chromium in soil (ISO Guide 35 Example C.1 data, reprinted in Table 3).
Run from the repository root."""
import json
import statistics as st
import sys
from pathlib import Path

sys.path.insert(0, "tools/research/homogeneity")
from cr_soil import W  # noqa: E402


def T(en, tr):
    return {"en": en, "tr": tr}


def chk(v, t):
    return {"value": v, "tolerance": t}


def c(x):
    return str(x).replace(".", ",")


means = [st.mean(b) for b in W]
sx = st.stdev(means)
sw = (sum(st.variance(b) for b in W) / len(W)) ** 0.5
ss = (sx ** 2 - sw ** 2 / 3) ** 0.5
sig_med = st.median([st.stdev(b) for b in W])
assert round(ss, 2) == 3.93 and round(sw, 2) == 2.87 and round(st.mean(means), 2) == 121.62
p = {"format": "uncertainty-calculations/project", "version": 1,
     "metadata": {"id": "possolo-bodnar-2018-chromium-soil-homogeneity-bayes", "field": "chemistry",
                  "title": T("Chromium in a soil reference material, 20 bottles in triplicate: ANOVA and a hierarchical Bayesian model (Possolo & Bodnar 2018; ISO Guide 35 C.1 data)",
                             "Bir toprak referans malzemesinde krom, üçer tekrarlı 20 şişe: ANOVA ve hiyerarşik Bayes modeli (Possolo & Bodnar 2018; ISO Guide 35 C.1 verileri)"),
                  "description": T("w_ij = ω + β_j + ε_ij with bottle effects β_j ~ N(0, τ²) and errors ε_ij ~ N(0, σ²). The value assigned to the material is the estimate of ω; its uncertainty includes the between-bottle and the within-bottle variability.",
                                   "w_ij = ω + β_j + ε_ij; şişe etkileri β_j ~ N(0, τ²), hatalar ε_ij ~ N(0, σ²). Malzemeye atanan değer ω'nın kestirimidir; belirsizliği şişeler arası ve şişe içi değişkenliği içerir."),
                  "explanation": [T(f"One-way ANOVA: s_bb = {ss:.2f} mg/kg and s_r = {sw:.2f} mg/kg, the values of ISO Guide 35 and of REML (R lme); ω = 121.62 mg/kg with u = s_x/√20 = 0.95 mg/kg.",
                                    f"Tek yönlü ANOVA: s_bb = {c(f'{ss:.2f}')} mg/kg ve s_r = {c(f'{sw:.2f}')} mg/kg, ISO Guide 35 ve REML (R lme) değerleri; ω = 121,62 mg/kg, u = s_x/√20 = 0,95 mg/kg."),
                                  T(f"Bayesian model as in the paper: ω flat (N(0, 10⁵)); τ half-Cauchy with median the standard deviation of the bottle averages, {sx:.4f} mg/kg; σ half-Cauchy with median the median of the within-bottle standard deviations, {sig_med:.4f} mg/kg.",
                                    f"Makaledeki Bayes modeli: ω düz (N(0, 10⁵)); τ ortancası şişe ortalamalarının standart sapması olan yarım Cauchy, {c(f'{sx:.4f}')} mg/kg; σ ortancası şişe içi standart sapmaların ortancası olan yarım Cauchy, {c(f'{sig_med:.4f}')} mg/kg."),
                                  T("The paper samples the posterior by MCMC (R2jags): ω = 121.62 mg/kg, u = 1 mg/kg; its Laplace approximation gives 0.98 mg/kg. The program integrates ω and the bottle effects out exactly and evaluates (τ, σ) on a grid.",
                                    "Makale sonsalı MCMC ile örnekler (R2jags): ω = 121,62 mg/kg, u = 1 mg/kg; Laplace yaklaşımı 0,98 mg/kg verir. Program ω'yı ve şişe etkilerini kesin olarak integre eder, (τ, σ)'yı ızgarada hesaplar.")],
                  "tags": ["possolo-bodnar-2018", "iso-guide-35", "homogeneity", "reference-material", "bayes", "anova"], "notes": [
                      T("No σ_pt is given: the ISO 13528 criteria are not evaluated (a reference-material homogeneity study).",
                        "σ_pt verilmemiştir: ISO 13528 ölçütleri değerlendirilmez (referans malzeme homojenlik çalışması).")],
                  "sources": [{"document": "Possolo & Bodnar, Approximate Bayesian evaluations of measurement uncertainty, Metrologia 55 (2018) 147–157", "section": "§4.4, Tables 3 and 4", "url": "https://doi.org/10.1088/1681-7575/aaa5be"}]},
     "proficiency": {"quantity": "w(Cr)", "unit": "mg/kg",
                     "homogeneity": {"items": [list(b) for b in W],
                                     "bayes": {"tau_scale": round(sx, 6), "sigma_scale": round(sig_med, 6)}}},
     "expected": {"proficiency": {"homogeneity": {"mean": chk(121.62, 0.005), "s_s": chk(3.93, 0.005), "s_w": chk(2.87, 0.005),
                                                  "mu_mean": chk(121.62, 0.01), "mu_sd": chk(1.0, 0.05)}},
                  "reference": T("Possolo & Bodnar (2018), §4.4 and Table 4: REML 121.62 (0.95) mg/kg, τ 3.93 and σ 2.87 mg/kg (= s_bb and s_r of ISO Guide 35); Bayesian (MCMC) 121.62 (1) mg/kg, Laplace 121.62 (0.98) mg/kg.",
                                 "Possolo & Bodnar (2018), §4.4 ve Tablo 4: REML 121,62 (0,95) mg/kg, τ 3,93 ve σ 2,87 mg/kg (= ISO Guide 35 s_bb ve s_r); Bayes (MCMC) 121,62 (1) mg/kg, Laplace 121,62 (0,98) mg/kg.")}}
Path("library/chemistry/possolo-bodnar-2018-chromium-soil-homogeneity-bayes.json").write_text(json.dumps(p, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("wrote possolo-bodnar-2018-chromium-soil-homogeneity-bayes")
