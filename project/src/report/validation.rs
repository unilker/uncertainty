//! Software validation report (ISO/IEC 17025:2017 7.11.2): the outcome of
//! rerunning the reference examples, for the laboratory's records.

use super::*;

/// The outcome of checking one project file.
#[derive(Clone, Debug)]
pub struct ValidationItem {
    pub file: String,
    pub project: Option<Project>,
    /// The comparisons with the reference values, or the error that stopped
    /// the evaluation.
    pub result: Result<Vec<CheckOutcome>, String>,
}

impl ValidationItem {
    fn passed(&self) -> Option<bool> {
        match &self.result {
            Err(_) => Some(false),
            Ok(c) if c.is_empty() => None,
            Ok(c) => Some(c.iter().all(|x| x.passed)),
        }
    }
}

struct VLabels {
    title: &'static str,
    purpose: &'static str,
    software: &'static str,
    version: &'static str,
    platform: &'static str,
    date: &'static str,
    options: &'static str,
    mcm_on: &'static str,
    mcm_off: &'static str,
    summary: &'static str,
    summary_text: &'static str,
    verdict_ok: &'static str,
    verdict_fail: &'static str,
    by_method: &'static str,
    method: &'static str,
    projects: &'static str,
    values: &'static str,
    results: &'static str,
    project: &'static str,
    source: &'static str,
    worst: &'static str,
    outcome: &'static str,
    passed: &'static str,
    failed: &'static str,
    no_reference: &'static str,
    error: &'static str,
    failures: &'static str,
    note: &'static str,
    signature: &'static str,
}

const V_EN: VLabels = VLabels {
    title: "Software validation report",
    purpose: "Record of the validation of the data-processing software before use and after changes (ISO/IEC 17025:2017 7.11.2). Every reference example is recomputed from its inputs and compared with the published values within a documented tolerance.",
    software: "Software",
    version: "Version",
    platform: "Platform",
    date: "Date",
    options: "Options",
    mcm_on: "Monte Carlo evaluations included (fixed seeds and numbers of trials as stored in the files)",
    mcm_off: "Monte Carlo evaluations skipped (--no-mcm)",
    summary: "Summary",
    summary_text: "{n} project files and {v} compared values. Passed: {p}; failed: {f}; without reference values: {w}.",
    verdict_ok: "All reference values are reproduced within their tolerances.",
    verdict_fail: "Some reference values are NOT reproduced; the software must not be used for these evaluations until the cause is found.",
    by_method: "Results by method",
    method: "Method",
    projects: "Projects",
    values: "Values",
    results: "Results by project",
    project: "Project",
    source: "Source",
    worst: "max ∣Δ∣/tol",
    outcome: "Outcome",
    passed: "passed",
    failed: "FAILED",
    no_reference: "no reference values",
    error: "error",
    failures: "Values outside their tolerance",
    note: "The column max ∣Δ∣/tol gives the largest ratio of the difference between computed and reference value to its tolerance in the file (≤ 1 passes). The tolerances reflect the rounding of the published values or, for Monte Carlo results, about five times the measured dispersion (research log D15).",
    signature: "Checked by: ____________________   Date: ____________   Signature: ____________________",
};

const V_TR: VLabels = VLabels {
    title: "Yazılım doğrulama raporu",
    purpose: "Veri işleme yazılımının kullanımdan önce ve değişikliklerden sonra doğrulanmasının kaydı (ISO/IEC 17025:2017 7.11.2). Her referans örnek girdilerinden yeniden hesaplanır ve yayımlanmış değerlerle belgelenmiş bir tolerans içinde karşılaştırılır.",
    software: "Yazılım",
    version: "Sürüm",
    platform: "Platform",
    date: "Tarih",
    options: "Seçenekler",
    mcm_on: "Monte Carlo değerlendirmeleri dahil (dosyalarda kayıtlı sabit tohumlar ve deneme sayıları)",
    mcm_off: "Monte Carlo değerlendirmeleri atlandı (--no-mcm)",
    summary: "Özet",
    summary_text: "{n} proje dosyası ve {v} karşılaştırılan değer. Başarılı: {p}; başarısız: {f}; referans değersiz: {w}.",
    verdict_ok: "Tüm referans değerler toleransları içinde yeniden üretilmektedir.",
    verdict_fail: "Bazı referans değerler yeniden üretilemiyor; neden bulunana kadar yazılım bu değerlendirmeler için kullanılmamalıdır.",
    by_method: "Yönteme göre sonuçlar",
    method: "Yöntem",
    projects: "Proje",
    values: "Değer",
    results: "Projelere göre sonuçlar",
    project: "Proje",
    source: "Kaynak",
    worst: "maks ∣Δ∣/tol",
    outcome: "Sonuç",
    passed: "başarılı",
    failed: "BAŞARISIZ",
    no_reference: "referans değer yok",
    error: "hata",
    failures: "Toleransı dışında kalan değerler",
    note: "maks ∣Δ∣/tol sütunu, hesaplanan ile referans değer arasındaki farkın dosyadaki toleransa en büyük oranını verir (≤ 1 başarılıdır). Toleranslar yayımlanmış değerlerin yuvarlamasını veya Monte Carlo sonuçları için ölçülen saçılımın yaklaşık beş katını yansıtır (araştırma günlüğü D15).",
    signature: "Kontrol eden: ____________________   Tarih: ____________   İmza: ____________________",
};

fn method_of(p: &Project) -> &'static str {
    if p.error_function.is_some() {
        "EURAMET cg-18 App. C"
    } else if p.sampling.is_some() {
        "Eurachem UfS:2019"
    } else if p.control_chart.is_some() {
        "Nordtest TR 569"
    } else if p.multivariate.is_some() {
        "JCGM 102"
    } else if p.precision_experiment.is_some() {
        "ISO 5725-2"
    } else if p.calibration.is_some() {
        "ISO 6143"
    } else if p.proficiency.is_some() {
        "ISO 13528"
    } else if p.topdown.is_some() {
        "top-down"
    } else if p.model.is_some() {
        if p.conformity.is_some() {
            "GUM/MCM + JCGM 106"
        } else {
            "GUM/MCM"
        }
    } else {
        "JCGM 106"
    }
}

/// The validation report in Markdown. `mcm` tells whether Monte Carlo
/// evaluations were run; `generated` is the date.
pub fn validation_markdown(items: &[ValidationItem], mcm: bool, lang: Lang, generated: &str) -> String {
    let d = match lang {
        Lang::En => &V_EN,
        Lang::Tr => &V_TR,
    };
    let mut s = String::new();
    let _ = writeln!(s, "# {}\n", d.title);
    let _ = writeln!(s, "{}\n", d.purpose);
    let _ = writeln!(s, "| | |\n|---|---|");
    let _ = writeln!(s, "| {} | {SOFTWARE_NAME} |", d.software);
    let _ = writeln!(s, "| {} | {SOFTWARE_VERSION} |", d.version);
    let _ = writeln!(s, "| {} | {} {} |", d.platform, std::env::consts::OS, std::env::consts::ARCH);
    let _ = writeln!(s, "| {} | {generated} |", d.date);
    let _ = writeln!(s, "| {} | {} |\n", d.options, if mcm { d.mcm_on } else { d.mcm_off });

    let n = items.len();
    let values: usize = items.iter().map(|i| i.result.as_ref().map_or(0, Vec::len)).sum();
    let p = items.iter().filter(|i| i.passed() == Some(true)).count();
    let f = items.iter().filter(|i| i.passed() == Some(false)).count();
    let w = items.iter().filter(|i| i.passed().is_none()).count();
    let _ = writeln!(s, "## {}\n", d.summary);
    let _ = writeln!(
        s,
        "{}\n",
        d.summary_text
            .replace("{n}", &n.to_string())
            .replace("{v}", &values.to_string())
            .replace("{p}", &p.to_string())
            .replace("{f}", &f.to_string())
            .replace("{w}", &w.to_string())
    );
    let _ = writeln!(s, "**{}**\n", if f == 0 { d.verdict_ok } else { d.verdict_fail });

    // By method.
    let mut methods: Vec<(&str, usize, usize, usize)> = Vec::new();
    for i in items {
        let m = i.project.as_ref().map_or("—", method_of);
        let k = match methods.iter().position(|x| x.0 == m) {
            Some(k) => k,
            None => {
                methods.push((m, 0, 0, 0));
                methods.len() - 1
            }
        };
        methods[k].1 += 1;
        methods[k].2 += i.result.as_ref().map_or(0, Vec::len);
        if i.passed() == Some(false) {
            methods[k].3 += 1;
        }
    }
    let _ = writeln!(s, "## {}\n", d.by_method);
    let _ = writeln!(s, "| {} | {} | {} | {} |\n|---|---:|---:|---|", d.method, d.projects, d.values, d.outcome);
    for (m, np, nv, nf) in &methods {
        let _ = writeln!(
            s,
            "| {m} | {np} | {nv} | {} |",
            if *nf == 0 { d.passed.to_string() } else { format!("{nf} {}", d.failed) }
        );
    }
    let _ = writeln!(s);

    let _ = writeln!(s, "## {}\n", d.results);
    let _ = writeln!(
        s,
        "| {} | {} | {} | {} | {} |\n|---|---|---:|---:|---|",
        d.project, d.source, d.values, d.worst, d.outcome
    );
    for i in items {
        let id = i.project.as_ref().and_then(|p| p.metadata.id.clone()).unwrap_or_else(|| i.file.clone());
        let src = i
            .project
            .as_ref()
            .and_then(|p| p.metadata.sources.first())
            .map(|x| {
                let doc = x.document.split(',').next().unwrap_or(&x.document);
                match &x.section {
                    Some(sec) => format!("{doc}, {sec}"),
                    None => doc.to_string(),
                }
            })
            .unwrap_or_default();
        let (count, worst, outcome) = match &i.result {
            Err(e) => (0, String::from("—"), format!("{}: {}", d.error, e.replace('|', "/"))),
            Ok(c) if c.is_empty() => (0, String::from("—"), d.no_reference.to_string()),
            Ok(c) => {
                let r = c
                    .iter()
                    .map(|x| {
                        let diff = (x.computed - x.expected).abs();
                        if x.tolerance > 0.0 {
                            diff / x.tolerance
                        } else if diff == 0.0 {
                            0.0
                        } else {
                            f64::INFINITY
                        }
                    })
                    .fold(0.0_f64, |a, b| if b.is_nan() { f64::INFINITY } else { a.max(b) });
                (c.len(), fmt_num(r, 2, lang), if c.iter().all(|x| x.passed) { d.passed } else { d.failed }.to_string())
            }
        };
        let _ = writeln!(s, "| `{id}` | {src} | {count} | {worst} | {outcome} |");
    }
    let _ = writeln!(s, "\n{}\n", d.note);

    let failures: Vec<(&ValidationItem, &CheckOutcome)> =
        items.iter().flat_map(|i| i.result.iter().flatten().filter(|c| !c.passed).map(move |c| (i, c))).collect();
    if !failures.is_empty() {
        let l = labels(lang);
        let _ = writeln!(s, "## {}\n", d.failures);
        let _ = writeln!(s, "| {} | | {} | {} | ± |\n|---|---|---:|---:|---:|", d.project, l.computed, l.expected);
        for (i, c) in failures {
            let id = i.project.as_ref().and_then(|p| p.metadata.id.clone()).unwrap_or_else(|| i.file.clone());
            let _ = writeln!(
                s,
                "| `{id}` | {} | {} | {} | {} |",
                c.what,
                fmt_num(c.computed, 6, lang),
                fmt_num(c.expected, 6, lang),
                fmt_num(c.tolerance, 3, lang)
            );
        }
        let _ = writeln!(s);
    }
    let _ = writeln!(s, "---\n");
    let _ = writeln!(s, "{}\n", d.signature);
    let _ = writeln!(s, "{} {SOFTWARE_NAME} {SOFTWARE_VERSION}, {generated}.", labels(lang).generated_by);
    s
}

/// The validation report as a self-contained HTML page.
pub fn validation_html(items: &[ValidationItem], mcm: bool, lang: Lang, generated: &str) -> String {
    let md = validation_markdown(items, mcm, lang, generated);
    let title = match lang {
        Lang::En => V_EN.title,
        Lang::Tr => V_TR.title,
    };
    page(title, &md_to_html(&md), lang)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outcome(what: &str, computed: f64, expected: f64, tolerance: f64) -> CheckOutcome {
        CheckOutcome {
            what: what.into(),
            computed,
            expected,
            tolerance,
            passed: (computed - expected).abs() <= tolerance,
        }
    }

    #[test]
    fn summary_and_failures() {
        let items = vec![
            ValidationItem {
                file: "a.json".into(),
                project: None,
                result: Ok(vec![outcome("gum.estimate", 1.0, 1.0, 0.01)]),
            },
            ValidationItem {
                file: "b.json".into(),
                project: None,
                result: Ok(vec![
                    outcome("gum.estimate", 2.0, 2.0, 0.01),
                    outcome("gum.standard_uncertainty", 0.5, 0.4, 0.05),
                ]),
            },
            ValidationItem { file: "c.json".into(), project: None, result: Ok(vec![]) },
            ValidationItem { file: "d.json".into(), project: None, result: Err("x | y".into()) },
        ];
        let md = validation_markdown(&items, false, Lang::En, "2026-10-26");
        assert!(
            md.contains("4 project files and 3 compared values. Passed: 1; failed: 2; without reference values: 1.")
        );
        assert!(md.contains("NOT reproduced"));
        assert!(md.contains("| `b.json` |  | 2 | 2.0 | FAILED |"));
        assert!(md.contains("| `a.json` |  | 1 | 0 | passed |"));
        assert!(md.contains("error: x / y"));
        assert!(md.contains("| `b.json` | gum.standard_uncertainty | 0.500000 | 0.400000 | 0.0500 |"));
        assert!(md.contains("--no-mcm"));
        let tr = validation_markdown(&items[..1], true, Lang::Tr, "2026-10-26");
        assert!(tr.contains("Tüm referans değerler") && !tr.contains("Toleransı dışında"));
        assert!(validation_html(&items, true, Lang::En, "2026-10-26").contains("<h1>Software validation report</h1>"));
    }
}
