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
| `model` | ikisinden biri | ölçüm modeli (7.3) |
| `topdown` | ikisinden biri | top-down değerlendirme (7.6) |
| `analysis` | hayır | GUM ve Monte Carlo seçenekleri (7.4); top-down projelerde dikkate alınmaz |
| `reporting` | hayır | raporlanan sonucun yuvarlanması (7.5) |
| `expected` | hayır | toleranslarıyla referans değerler (7.8) |

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
  "gum": { "coverage_probability": 0.95, "coverage_factor": null, "higher_order": false, "dof_rounding": "truncate" },
  "mcm": { "trials": 1000000, "adaptive": { "ndig": 2, "tolerance_divisor": 1, "max_trials": 10000000 },
           "coverage_probability": null, "seed": 1, "interval": "shortest", "validation_ndig": 2 }
}
```

Yazılmayan alanlar gösterilen varsayılanları alır. `coverage_factor`, `coverage_probability`'yi; `adaptive`,
`trials`'ı geçersiz kılar; Monte Carlo kapsam olasılığı varsayılan olarak GUM sonucununkidir. Yöntemlerden
birini atlamak için `"gum": null` veya `"mcm": null` yazınız.

## 7.5 `reporting`

`significant_digits` (varsayılan 2), `decimal_places` (anlamlı basamak yerine), `rounding` (`half_even`,
`half_up`, `up`).

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

## 7.7 Hatalar

Program, alanı adıyla belirten bir iletiyle şunları reddeder: bilinmeyen alanlar; hem `model` hem `topdown`
içeren veya hiçbirini içermeyen proje; tutarsız seçenekler (örneğin `sd` ile birlikte `expanded`); `levels`
verilmemiş mutlak top-down bileşenleri; hiçbir bileşenin geçerli olmadığı bir düzey; Monte Carlo yönteminde
korelasyonlu normal olmayan girdiler; sonlu olmayan veriler.

## 7.8 `expected`

Kendini denetleyen projeler için referans değerler; her değer `{ "value": …, "tolerance": … }` biçimindedir ve
`uncertainty check` ile denetlenir. `reference`, değerlerin nereden geldiğini belirten metindir.

* `gum`: `estimate`, `standard_uncertainty`, `expanded_uncertainty`, `coverage_factor`, `effective_dof`
* `mcm`: `estimate`, `standard_uncertainty`, `interval_low`, `interval_high`, `gum_validated` (true/false)
* `topdown`:
  * `components`: `{ "<ad>": değer }` — bir bileşenin standart belirsizliği (bağılsa yüzde)
  * `bias`: `{ "<ad>": { "mean", "rms", "mean_u_reference" } }`
  * `recovery`: `mean_percent`, `standard_uncertainty_percent`, `sd_percent`, `statistic`, `critical`,
    `chi_squared`, `significant`
  * `levels`: `{ "level", "value", "standard_uncertainty", "expanded_uncertainty",
    "relative_standard_percent", "relative_expanded_percent" }` listesi
