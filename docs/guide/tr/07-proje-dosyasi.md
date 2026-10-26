# 7. Proje dosyası başvurusu

Proje bir JSON dosyasıdır. Bilinmeyen alanlar hatadır; böylece yanlış yazılmış bir ad asla fark edilmeden
kalmaz. Metin alanları (başlıklar, tanımlar, açıklamalar, notlar, referans metinleri) düz metin (İngilizce) veya
`{ "en": "…", "tr": "…" }` olarak yazılır. Alan adları programın her dilinde aynıdır (İngilizce).

## 7.1 Üst düzey

| Alan | Zorunlu | İçerik |
|---|---|---|
| `format` | evet | `"uncertainty-calculations/project"` |
| `version` | evet | `1` |
| `metadata` | evet | başlık ve belgeleme (7.2) |
| `model` | altısından biri | ölçüm modeli (7.3) |
| `topdown` | altısından biri | top-down değerlendirme (7.6) |
| `proficiency` | altısından biri | yeterlilik testi turu, ISO 13528 (7.7) |
| `calibration` | altısından biri | her iki değişkende belirsizlikli kalibrasyon, ISO 6143 (7.8) |
| `precision_experiment` | altısından biri | laboratuvarlar arası kesinlik deneyi, ISO 5725-2 (7.10) |
| `multivariate` | altısından biri | birden çok çıktı büyüklüklü model, JCGM 102 (7.11) |
| `conformity` | `model` ile veya tek başına | uygunluk değerlendirmesi, JCGM 106 (7.9) |
| `analysis` | hayır | GUM ve Monte Carlo seçenekleri (7.4); model ve çok çıktılı projelerde |
| `reporting` | hayır | raporlanan sonucun yuvarlanması (7.5) |
| `expected` | hayır | toleranslarıyla referans değerler (7.13) |

## 7.2 `metadata`

`title` (zorunlu), `id` (kütüphanede: dosya adı), `description`, `explanation` (adımlar listesi), `field`
(`chemistry`, `mass`, `length`, …), `sources` (`{ "document", "section", "url" }` listesi), `tags`, `notes`
(liste).

## 7.3 `model`

```json
"model": {
  "output": { "name": "c", "unit": "mg/L", "expression": "1000 * m * P / V", "description": "…" },
  "intermediates": [ { "name": "V", "expression": "V_nom + dV_cal", "unit": "mL" } ],
  "inputs": [ { "name": "m", "unit": "mg", "description": "…", "distribution": { … }, "dof": 50 } ],
  "correlations": [ { "a": "X1", "b": "X2", "r": 0.9 } ]
}
```

Bir girdinin `dof` alanı GUM yaklaşımındaki serbestlik derecesidir (varsayılan ∞; A tipi dağılımlar bunu
kendileri belirler).

### Dağılımlar (`"type"`)

| `type` | Alanlar |
|---|---|
| `constant` | `value` |
| `normal` | `mean` ile birlikte ya `sd` ya da `expanded` ve `k` |
| `t` | `mean`, `dof` ile birlikte ya `scale` ya da `expanded` ve `k` |
| `rectangular`, `triangular`, `arcsine` | `lower` ve `upper`, ya da `value` ve `half_width` |
| `trapezoidal` | yukarıdaki gibi sınırlar ve `beta` (tepe ve taban yarı genişliklerinin oranı) |
| `curvilinear_trapezoid` | yukarıdaki gibi sınırlar ve `d` (sınırların belirsizliği) |
| `exponential` | `mean` |
| `gamma` | `shape`, `scale` |
| `type_a` | `observations` (liste) |
| `type_a_pooled` | `mean`, `pooled_sd`, `pooled_dof`, `n` |

## 7.4 `analysis`

```json
"analysis": {
  "gum": { "coverage_probability": 0.95, "coverage_factor": null, "higher_order": false, "dof_rounding": "truncate",
           "kragten": false },
  "mcm": { "trials": 1000000, "adaptive": { "ndig": 2, "tolerance_divisor": 1, "max_trials": 10000000 },
           "coverage_probability": null, "seed": 1, "interval": "shortest", "validation_ndig": 2 }
}
```

Yazılmayan alanlar gösterilen varsayılanları alır. `coverage_factor`, `coverage_probability`'yi; `adaptive`,
`trials`'ı geçersiz kılar; Monte Carlo kapsam olasılığı varsayılan olarak GUM sonucununkidir. Yöntemlerden
birini atlamak için `"gum": null` veya `"mcm": null` yazınız.

## 7.5 `reporting`

`significant_digits` (varsayılan 2), `decimal_places` (anlamlı basamak yerine), `rounding` (`half_even`,
`half_up`, `up`), `uncorrected_bias` (model projeleri: `bias`, `u_bias`, `strategy` `ue` | `rssu` | `rssu_k` |
`sumu` | `sumu_max`; bölüm 6.3).

## 7.6 `topdown`

```json
"topdown": {
  "quantity": "c(NH4-N)", "unit": "µg/L",
  "levels": [10, 100],
  "precision": [ … ], "bias": [ … ], "recovery": { … }, "components": [ … ],
  "coverage_factor": 2
}
```

`coverage_factor` (varsayılan 2) yerine: `coverage_dof` ile `coverage_probability` (t dağılımından k). Her
kesinlik, sapma, geri kazanım ve ek bileşen bir `"range": { "min": …, "max": … }` (min dahil, max hariç) ve bir
`description` alabilir. Bileşen adları benzersiz olmalıdır.

### Kesinlik girdileri

`{ "name": …, "data": { "type": … }, "range", "mean_of", "to_relative_at", "to_absolute_at" }`; `data`:

| `type` | Alanlar |
|---|---|
| `series` | `values`, `relative` |
| `groups` | `groups`: `{ "values" }` veya `{ "sd", "n" }` / `{ "sd_percent", "n" }` listesi; `relative` |
| `duplicates` | `pairs` (`[x1, x2]` listesi), `relative`, `method` (`pooled`, `normalised_differences`, `mean_range`), `min_mean`, `max_mean` |
| `control_limits` | `limit` veya `limit_percent`, `k` (varsayılan 2) |
| `given` | `sd` veya `sd_percent`, `dof` |
| `linear` | `points` (`[c, s]` listesi) veya `intercept` ve `slope`; `dilution_factor` |
| `continuous` | `sd0`, `rsd1_percent` |
| `reproducibility` | `sd_reproducibility` veya `sd_between_labs`, `sd_repeatability`, `lab_repeatability` (hepsi isteğe bağlı `_percent`), `replicates`; `method_bias`: `{ "labs", "replicates", "u_reference" }`; `bias_check`: `{ "bias", "sd_within", "n" }` |
| `staggered_nested` | `results` (`[y₁, y₂, y₃]` satırları), `estimate` (`repeatability`, `intermediate`, `reproducibility`), `relative` |

`mean_of`: `{ "replicates": n, "days": p, "repeatability" | "repeatability_percent" }`.
`to_relative_at`: bir sayı veya `"mean"`; `to_absolute_at`: bir sayı.

### Sapma girdileri

`{ "name": "u(bias)", "kind": "crm" | "proficiency_tests" | "recovery", "relative": true, "references": [ … ],
"u_reference" | "u_reference_percent" | "u_reference_components_percent" }` — son üçü tüm referanslarda ortak
bir referans belirsizliği verir. Her referans:

| Alan | Anlamı |
|---|---|
| `label` | rapordaki adı |
| `reference` | sertifikalı veya atanan değer C |
| `u_reference`, `u_reference_percent` veya `k` ile `expanded` | C'nin belirsizliği |
| `labs` ile `sd_reproducibility` veya `sd_reproducibility_percent` | yeterlilik testi: u(C) = s_R/√n_Lab |
| `measured` veya `sd`/`sd_percent` ve `n` ile `mean` | laboratuvarın sonuçları |
| `bias`, `bias_percent`, `recovery_percent` | doğrudan verilen sapma |

### Geri kazanım

`{ "name", "tests": [ … ], "include_sd_of_recoveries": false, "dof", "confidence": 0.95, "correction":
"if_significant" | "always" | "never" }`; deneyler:

| `type` | Alanlar |
|---|---|
| `reference` | `measured` veya `mean`, `sd`, `n`; `reference`; `u_reference` veya `k` ile `expanded` |
| `spiked` | `after` veya `mean_after`, `sd_after`, `n_after`; `before` için aynısı; `spike`; `u_spike` veya `u_spike_percent` |
| `recoveries` | `values_percent` veya `mean_percent`, `sd_percent`, `n` |

Anlamlılık testinin varsayılan serbestlik derecesi deneyler üzerinden Σ(nᵢ − 1)'dir; bu toplam sıfırsa veya
s(R) dahil edilmişse N − 1'dir.

### Ek bileşenler

`{ "name", "u" | "u_percent" | "half_width" | "half_width_percent", "distribution": "rectangular" |
"triangular", "multiplicity": 1, "range" }`.

## 7.7 `proficiency`

```json
"proficiency": {
  "quantity": "atrazine", "unit": "mg/L",
  "results": [ { "lab": "L01", "value": 0.262, "expanded": 0.03, "k": 2, "method": "GC-MS" },
               { "lab": "L17", "less_than": 0.015 } ],
  "assigned_value": { "method": "algorithm_a" },
  "sd_pt": { "method": "robust" }
}
```

| Alan | İçerik |
|---|---|
| `results` | her katılımcı için bir giriş: `lab` (benzersiz), `value` ya da `less_than`/`greater_than` (sansürlü, skorlanmaz), `u` ya da `k` ile `expanded` (varsayılan k = 2), `method` |
| `assigned_value` | `method`: `algorithm_a` (x*, u = `u_factor`·s*/√p, `u_factor` 1,25), `median` (u = 1,25·`scale`/√p, `scale` `niqr` veya `made`), `mean` (u = s/√p), `reference` (`value` ile `u` ya da `expanded` ve `k`); isteğe bağlı `u_hom`, `u_trans`, `u_stab` kareler toplamıyla eklenir (ISO 13528 Formül 3) |
| `sd_pt` | `method`: `robust` (s*), `made`, `niqr`, `sd` ya da `value` veya `percent` (x_pt'nin yüzdesi) ile `given` |
| `delta_e`, `delta_e_percent` | PA skorları için ölçüm hatası izni δ_E (varsayılan 3σ_pt) |
| `coverage_factor` | E_n için U(x_pt)'nin k değeri (varsayılan 2; referans değerin verilmiş genişletilmiş belirsizliği olduğu gibi kullanılır) |
| `uncertainty_screening` | `u_min`, `u_max`: sayı ya da `"u_assigned"`, `"1.5_robust_sd"`, `"1.5_sd_pt"` (ISO 13528 9.8) |
| `homogeneity` | `items` (her numune için tekrar sonuçları listesi, sayıları eşit), `sd_pt` veya `sd_pt_percent_of_mean` (varsayılan: turun σ_pt'si) |
| `stability` | `values`; `reference` (varsayılan: homojenlik ortalaması); Formül (B.18) için `u_reference` ve `u_values` |
| `algorithm_a` | `"third_significant_figure"` (C.3.1'in durma kuralı, varsayılan) veya `"converged"` |

Turdan önceki bir homojenlik ve kararlılık çalışması için `results` boş bırakılabilir.

## 7.8 `calibration`

```json
"calibration": {
  "analyte": "NO", "unit": "µmol/mol", "response_unit": "absorbans",
  "points": [ { "name": "M1", "x": 4.5, "u_x": 0.045, "y": 0.1969, "u_y": 0.003938 }, … ],
  "covariances": [ { "between": ["M4", "M7"], "value": 0.00016 } ],
  "function": "linear",
  "samples": [ { "name": "S1", "y": 0.6, "u_y": 0.012 } ]
}
```

| Alan | İçerik |
|---|---|
| `points` | referans karışımlar: `name` (benzersiz), standart belirsizliği `u_x` olan içerik `x`, belirsizliği `u_y` (> 0) olan yanıt `y` |
| `covariances` | adlandırılmış iki noktanın içeriklerinin isteğe bağlı kovaryansları u(xᵢ, xₕ) |
| `function` | `linear`, `quadratic`, `cubic`, `power` veya `exponential` (bölüm 10) |
| `compare` | diğer fonksiyonları da uydurup listele (varsayılan `true`) |
| `samples` | analiz edilecek karışımlar: `name`, `y`, `u_y` |
| `coverage_factor` | numunelerin genişletilmiş belirsizliklerinin k değeri (varsayılan 2) |
| `target_uncertainty` | 6.2.3'ün üst sınırıyla karşılaştırılan kabul edilebilir standart belirsizlik |
| `drift` | `reference` (bir nokta adı), ortalama yanıtlar `before` ve `after`, `n` ve `n_calib` (varsayılan 10) |
| `validation` | uyumsuz bileşimli referans karışımlar: `name`, `y`, `u_y`, `x_ref`, `u_x_ref` (Formül 13) |

## 7.9 `conformity`

`model` bölümünün yanında (sonuç değerlendirilir) veya bir projenin tek bölümü olarak (JCGM 106; bölüm 11).

| Alan | İçerik |
|---|---|
| `lower`, `upper` | tolerans sınırları (en az biri) |
| `rule` | `simple_acceptance` (varsayılan), `guarded_acceptance`, `guarded_rejection` |
| `r` / `guard_band` / `probability` | koruma bandı w = r·2u (varsayılan r = 1), doğrudan w veya sınırdaki olasılığın verildiği kabul sınırları |
| `acceptance_lower`, `acceptance_upper` | doğrudan verilen kabul sınırları (kural bunlardan çıkarılır) |
| `distribution` | model projeleri: `normal` (varsayılan) veya `t` (etkin serbestlik derecesi) |
| `quantity`, `unit`, `u` veya `u_relative`, `dof`, `results` | kendi başına projeler: özellik, ölçüm sistemi, ölçülen değerler (`name`, `value`, isteğe bağlı `u`) |
| `process`, `target_consumer_risk` | küresel riskler: `distribution` `normal` veya `gamma`, `mean`, `sd`; istenen R_C (r'yi bulur) |

## 7.10 `precision_experiment`

ISO 5725-2'nin temel yöntemi (bölüm 12).

```json
"precision_experiment": {
  "quantity": "Kömürün kükürt içeriği", "unit": "%",
  "laboratories": ["1", "2", "3"],
  "levels": [ { "name": "1", "results": [[0.71, 0.71, 0.70], [0.69, 0.67, 0.68], [0.66, 0.65, 0.69]] } ],
  "exclude": [ { "laboratory": "3", "level": "1", "reason": "yanlış numune" } ]
}
```

| Alan | İçerik |
|---|---|
| `laboratories` | adlar (benzersiz), her düzeydeki sonuçların sırasıyla |
| `levels` | `name` (benzersiz) ve `results`: her laboratuvar için bir liste, boş hücre için `[]` (Form A) |
| `exclude` | inceleme sonrası dışarıda bırakılan sonuçlar: `laboratory`, isteğe bağlı `level` (yoksa tüm düzeyler), isteğe bağlı `result` (1'den başlar; yoksa tüm hücre), `reason` |
| `method` | `anova` (Formül 23–31, varsayılan) veya `reml` (Ek B.2) |
| `form_decimals` | Form B ve C'ye kaydedilen hücre ortalamaları ve standart sapmalarının ondalık sayısı; standardın örneklerinde olduğu gibi testlerde ve varyanslarda kullanılır (yoksa tam duyarlılık) |
| `relationships` | üç veya daha fazla düzey varken uydurulan ilişkiler: `"I"`, `"II"`, `"III"`, `"IV"` (varsayılan dördü; hiçbiri için `[]`) |

## 7.11 `multivariate`

Birden çok çıktı büyüklüğü olan bir model (JCGM 102; bölüm 13). `analysis` seçenekleri geçerlidir:
`gum.coverage_probability` ve `mcm` (`trials`, `adaptive`, `seed`, `validation_ndig`, `coverage_probability`).

```json
"multivariate": {
  "outputs": [ { "name": "R", "unit": "Ω", "expression": "V/I*cos(phi)" }, { "name": "X", "unit": "Ω", "expression": "V/I*sin(phi)" } ],
  "indications": [ { "inputs": ["V", "I", "phi"], "units": ["V", "A", "rad"], "values": [[5.007, 0.019663, 1.0456], …], "covariance": "mean" } ]
}
```

| Alan | İçerik |
|---|---|
| `outputs` | `name`, `unit`, `expression`: f_j(X) veya `implicit` ise h_j(Y, X) = 0 denklemi; `start`: örtük modelin başlangıç değeri |
| `implicit` | Newton yöntemiyle çözülen h(Y, X) = 0 modeli için `true` (varsayılan `false`) |
| `inputs`, `correlations`, `intermediates` | `model` bölümündeki gibi (7.3) |
| `indications` | vektör gösterge değeri serilerinden girdi büyüklükleri: `inputs` (adlar), isteğe bağlı `units`, `values` (takım başına bir satır, n > N), `covariance` `mean` (M/(n(n − 1)), varsayılan) veya `t` (M/((ν − 2)n)); Monte Carlo yöntemi bunları çok değişkenli t dağılımından çeker |

## 7.12 Hatalar

Program, alanı adıyla belirten bir iletiyle şunları reddeder: bilinmeyen alanlar; `model`, `topdown`,
`proficiency`, `calibration`, `precision_experiment` ve `multivariate` bölümlerinden birden fazlasını içeren veya hiçbirini içermeyen proje; tutarsız seçenekler (örneğin `sd` ile birlikte `expanded`); `levels`
verilmemiş mutlak top-down bileşenleri; hiçbir bileşenin geçerli olmadığı bir düzey; Monte Carlo yönteminde
korelasyonlu normal olmayan girdiler; sonlu olmayan veriler.

## 7.13 `expected`

Kendini denetleyen projeler için referans değerler; her değer `{ "value": …, "tolerance": … }` biçimindedir ve
`uncertainty check` ile denetlenir. `reference`, değerlerin nereden geldiğini belirten metindir.

* `gum`: `estimate`, `standard_uncertainty`, `expanded_uncertainty`, `coverage_factor`, `effective_dof`,
  `kragten_uncertainty`
* `mcm`: `estimate`, `standard_uncertainty`, `interval_low`, `interval_high`, `gum_validated` (true/false)
* `topdown`:
  * `components`: `{ "<ad>": değer }` — bir bileşenin standart belirsizliği (bağılsa yüzde)
  * `bias`: `{ "<ad>": { "mean", "rms", "mean_u_reference" } }`
  * `recovery`: `mean_percent`, `standard_uncertainty_percent`, `sd_percent`, `statistic`, `critical`,
    `chi_squared`, `significant`
  * `levels`: `{ "level", "value", "standard_uncertainty", "expanded_uncertainty",
    "relative_standard_percent", "relative_expanded_percent" }` listesi
* `proficiency`: `assigned_value`, `u_assigned`, `sd_pt`, `median`, `made`, `niqr`, `robust_mean`, `robust_sd`,
  `iterations`, `mean`, `sd`; `scores`: `{ "<laboratuvar>": { "d", "d_percent", "pa", "z", "z_prime", "zeta", "en" } }`;
  `homogeneity`: `mean`, `s_x`, `s_w`, `s_s`, `criterion`, `sufficient`; `stability`: `difference`, `sufficient`
* `calibration`: `params`, `u_params` ve `cov_params` (listeler; kovaryanslar (0,1), (0,2), …, (1,2), … sırasıyla),
  `s_res`, `gamma`, `admissible` (true/false), `samples`: `{ "<ad>": { "x", "u" } }`, `drift_passed`
* `conformity`: `acceptance_lower`, `acceptance_upper`, `capability_index`, `r`, `conforming`, `accepted`,
  `consumer_risk`, `producer_risk`; `results`: `{ "<ad>": { "conformance_probability", "accepted" } }`
* `precision_experiment`: `levels`: `{ "<düzey>": { "p", "m", "s_r", "s_L", "s_R", "cochran", "grubbs_low",
  "grubbs_high", "grubbs_double_low", "grubbs_double_high", "final_cochran" } }` (`p` bir tamsayıdır; test
  istatistikleri tüm sonuçlara, `final_cochran` kalan sonuçlara aittir); `relationships`:
  `{ "of": "s_r" | "s_R", "relationship": "I" … "IV", "params": [değerler] }` listesi
* `multivariate`: `gum` ve `mcm`, her biri `estimate` ve `standard_uncertainty` (`{ "<çıktı>": değer }`), `correlations`
  (`{ "between": [a, b], "value", "tolerance" }` listesi), `k_ellipsoid`, `k_rectangle`, `smallest_region_area` (Monte
  Carlo, iki çıktı) ile; `ellipsoid_validated`, `rectangle_validated` (true/false)
