//! `uncertainty` — command-line front end of Uncertainty Calculations
//! (Belirsizlik Hesaplamaları).

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};
use uncertainty_project::{self as project, report, Lang, Overrides, CITATION, SOFTWARE_NAME, SOFTWARE_VERSION};

const HELP: &str = "\
Uncertainty Calculations (Belirsizlik Hesaplamaları) — measurement uncertainty
evaluation by the GUM (JCGM 100) and the Monte Carlo method (JCGM 101), or
top-down from validation and quality-control data (Nordtest TR 537,
Eurachem/CITAC MUPR); statistics for proficiency testing (ISO 13528);
calibration with uncertainties in both variables (ISO 6143); conformity
assessment (JCGM 106); repeatability and reproducibility from interlaboratory
experiments (ISO 5725-2); models with several output quantities (JCGM 102);
control charts for internal quality control (Nordtest TR 569).

USAGE:
    uncertainty run <project.json> [options]
    uncertainty check <file-or-directory>... [--no-mcm] [--report FILE.md|FILE.html] [--lang en|tr]
    uncertainty template [--topdown | --proficiency | --calibration | --conformity | --precision | --multivariate | --control-chart]
    uncertainty catalogue <file-or-directory>... [--lang en|tr] [--output FILE]
    uncertainty --version | --help

RUN OPTIONS:
    --lang en|tr          report language (default en)
    --format md|html|json report format (default md, or from the --output extension)
    --output FILE         write the report to FILE instead of standard output
    --no-mcm              skip the Monte Carlo evaluation
    --trials N            fixed number of Monte Carlo trials (overrides the project)
    --seed S              random-number seed (overrides the project)

CHECK:
    Runs every project file (directories are searched recursively for *.json)
    and compares the results with the reference values stored in the file.
    The exit status is 1 if any comparison fails. With --report, writes a
    software validation report (ISO/IEC 17025 7.11.2) for the laboratory's
    records.

CATALOGUE:
    Writes the worked-examples chapter of the user guide (Markdown) from the
    given project files, recomputing every result (without Monte Carlo).

TEMPLATE:
    Prints a minimal project file to start from: a measurement model, with
    --topdown a top-down evaluation from raw validation data, or with
    --proficiency a proficiency test round (ISO 13528), with --calibration a
    calibration with uncertainties in both variables (ISO 6143), with
    --conformity a conformity assessment (JCGM 106), with --precision an
    interlaboratory precision experiment (ISO 5725-2), with --multivariate a
    model with several output quantities (JCGM 102), or with --control-chart
    a control chart for internal quality control (Nordtest TR 569).
";

const TEMPLATE: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Concentration of a calibration standard", "tr": "Kalibrasyon standardının derişimi" },
    "field": "chemistry",
    "sources": [ { "document": "EURACHEM/CITAC Guide CG 4, QUAM:2012", "section": "Example A1" } ]
  },
  "model": {
    "output": { "name": "c", "unit": "mg/L", "expression": "1000 * m * P / V" },
    "inputs": [
      { "name": "m", "unit": "mg", "distribution": { "type": "normal", "mean": 100.28, "sd": 0.05 } },
      { "name": "P", "distribution": { "type": "rectangular", "value": 0.9999, "half_width": 0.0001 } },
      { "name": "V", "unit": "mL", "distribution": { "type": "triangular", "value": 100.0, "half_width": 0.1 } }
    ]
  },
  "analysis": {
    "gum": { "coverage_factor": 2 },
    "mcm": { "adaptive": { "ndig": 2 }, "seed": 1 }
  },
  "reporting": { "significant_digits": 2 }
}
"#;

const TEMPLATE_TOPDOWN: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Ammonium in water (top-down)", "tr": "Suda amonyum (top-down)" },
    "field": "chemistry"
  },
  "topdown": {
    "quantity": "c(NH4-N)",
    "unit": "µg/L",
    "levels": [10, 100],
    "precision": [
      { "name": "u(Rw), control sample", "data": { "type": "series", "values": [101.2, 98.7, 100.4, 99.1, 102.3, 100.8], "relative": true } },
      { "name": "s_r, duplicates", "data": { "type": "duplicates", "pairs": [[7.46, 7.25], [9.01, 9.17], [3.60, 3.10]] },
        "range": { "max": 30 } }
    ],
    "bias": [
      { "kind": "proficiency_tests", "references": [
          { "label": "PT 1", "reference": 81, "mean": 83, "sd_reproducibility_percent": 10, "labs": 31 },
          { "label": "PT 2", "reference": 73, "mean": 75, "sd_reproducibility_percent": 7, "labs": 36 } ] }
    ],
    "components": [ { "name": "stock solution", "u_percent": 0.5 } ]
  },
  "reporting": { "significant_digits": 2 }
}
"#;

const TEMPLATE_PROFICIENCY: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Proficiency test round", "tr": "Yeterlilik testi turu" },
    "field": "chemistry"
  },
  "proficiency": {
    "quantity": "c(Pb)",
    "unit": "mg/kg",
    "results": [
      { "lab": "L01", "value": 10.2, "expanded": 0.8, "k": 2 },
      { "lab": "L02", "value": 9.7, "expanded": 0.9, "k": 2 },
      { "lab": "L03", "value": 10.5, "expanded": 1.0, "k": 2 },
      { "lab": "L04", "value": 12.9, "expanded": 0.6, "k": 2 },
      { "lab": "L05", "value": 9.9, "expanded": 0.7, "k": 2 },
      { "lab": "L06", "value": 10.1, "expanded": 1.2, "k": 2 }
    ],
    "assigned_value": { "method": "algorithm_a" },
    "sd_pt": { "method": "given", "percent": 10 }
  }
}
"#;

const TEMPLATE_CALIBRATION: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Calibration with uncertainties in both variables", "tr": "Her iki değişkende belirsizlikli kalibrasyon" },
    "field": "chemistry"
  },
  "calibration": {
    "analyte": "NO",
    "unit": "µmol/mol",
    "points": [
      { "name": "M1", "x": 4.5, "u_x": 0.045, "y": 0.1969, "u_y": 0.003938 },
      { "name": "M2", "x": 18.75, "u_x": 0.1875, "y": 0.7874, "u_y": 0.015748 },
      { "name": "M3", "x": 50.0, "u_x": 0.5, "y": 2.0228, "u_y": 0.040456 }
    ],
    "function": "linear",
    "samples": [ { "name": "S1", "y": 0.6, "u_y": 0.012 } ]
  }
}
"#;

const TEMPLATE_PRECISION: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Interlaboratory precision experiment", "tr": "Laboratuvarlar arası kesinlik deneyi" },
    "field": "chemistry"
  },
  "precision_experiment": {
    "quantity": "Mass fraction of analyte",
    "unit": "%",
    "laboratories": ["A", "B", "C", "D", "E"],
    "levels": [
      { "name": "1", "results": [[1.02, 1.05], [0.98, 1.00], [1.07, 1.04], [1.01, 0.99], [1.03, 1.06]] },
      { "name": "2", "results": [[5.10, 5.18], [4.95, 5.02], [5.21, 5.15], [5.04, 5.00], [5.12, 5.20]] }
    ]
  }
}
"#;

const TEMPLATE_MULTIVARIATE: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Resistance and reactance", "tr": "Direnç ve reaktans" },
    "field": "electrical"
  },
  "multivariate": {
    "outputs": [
      { "name": "R", "unit": "Ω", "expression": "V/I*cos(phi)" },
      { "name": "X", "unit": "Ω", "expression": "V/I*sin(phi)" }
    ],
    "inputs": [
      { "name": "V", "unit": "V", "distribution": { "type": "normal", "mean": 4.999, "sd": 0.0032 } },
      { "name": "I", "unit": "A", "distribution": { "type": "normal", "mean": 0.019661, "sd": 0.0000095 } },
      { "name": "phi", "unit": "rad", "distribution": { "type": "normal", "mean": 1.04446, "sd": 0.00075 } }
    ],
    "correlations": [ { "a": "V", "b": "I", "r": -0.36 }, { "a": "V", "b": "phi", "r": 0.86 }, { "a": "I", "b": "phi", "r": -0.65 } ]
  },
  "analysis": { "mcm": { "trials": 200000 } }
}
"#;

const TEMPLATE_CONFORMITY: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Conformity assessment", "tr": "Uygunluk değerlendirmesi" },
    "field": "general"
  },
  "conformity": {
    "quantity": "Y",
    "unit": "mm²/s",
    "lower": 12.5,
    "upper": 16.3,
    "rule": "guarded_acceptance",
    "r": 1,
    "u": 0.2,
    "results": [ { "name": "sample 1", "value": 13.6 }, { "name": "sample 2", "value": 16.1 } ]
  }
}
"#;

const TEMPLATE_CONTROL_CHART: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "X-chart of a control sample", "tr": "Kontrol numunesinin X-grafiği" },
    "field": "chemistry"
  },
  "control_chart": {
    "quantity": "Zn, control solution 60.0 µg/L",
    "unit": "µg/L",
    "values": [64.5, 66.3, 61.1, 59.7, 57.4, 56.2, 58.4, 58.2, 63.0, 59.5, 56.0, 59.4, 60.2, 62.9, 60.5, 60.8, 61.5, 58.5, 58.9, 60.5],
    "reference": 60.0,
    "limits": { "basis": "statistical" }
  }
}
"#;

#[derive(Clone, Copy, PartialEq)]
enum Format {
    Markdown,
    Html,
    Json,
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        None | Some("-h" | "--help" | "help") => {
            print!("{HELP}");
            Ok(true)
        }
        Some("-V" | "--version" | "version") => {
            println!("{SOFTWARE_NAME} {SOFTWARE_VERSION}\n{CITATION}");
            Ok(true)
        }
        Some("template") if args.get(1).is_some_and(|a| a == "--topdown") => {
            print!("{TEMPLATE_TOPDOWN}");
            Ok(true)
        }
        Some("template") if args.get(1).is_some_and(|a| a == "--proficiency") => {
            print!("{TEMPLATE_PROFICIENCY}");
            Ok(true)
        }
        Some("template") if args.get(1).is_some_and(|a| a == "--conformity") => {
            print!("{TEMPLATE_CONFORMITY}");
            Ok(true)
        }
        Some("template") if args.get(1).is_some_and(|a| a == "--multivariate") => {
            print!("{TEMPLATE_MULTIVARIATE}");
            Ok(true)
        }
        Some("template") if args.get(1).is_some_and(|a| a == "--precision") => {
            print!("{TEMPLATE_PRECISION}");
            Ok(true)
        }
        Some("template") if args.get(1).is_some_and(|a| a == "--control-chart") => {
            print!("{TEMPLATE_CONTROL_CHART}");
            Ok(true)
        }
        Some("template") if args.get(1).is_some_and(|a| a == "--calibration") => {
            print!("{TEMPLATE_CALIBRATION}");
            Ok(true)
        }
        Some("template") => {
            print!("{TEMPLATE}");
            Ok(true)
        }
        Some("run") => cmd_run(&args[1..]),
        Some("check") => cmd_check(&args[1..]),
        Some("catalogue" | "catalog") => cmd_catalogue(&args[1..]),
        Some(other) => Err(format!("unknown command '{other}' (see uncertainty --help)")),
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

fn value<'a>(args: &'a [String], i: &mut usize, flag: &str) -> Result<&'a str, String> {
    *i += 1;
    args.get(*i).map(String::as_str).ok_or_else(|| format!("{flag} needs a value"))
}

fn load(path: &Path) -> Result<project::schema::Project, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    project::parse(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn cmd_run(args: &[String]) -> Result<bool, String> {
    let mut file = None;
    let mut lang = Lang::En;
    let mut format = None;
    let mut output: Option<PathBuf> = None;
    let mut ov = Overrides::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--lang" => lang = value(args, &mut i, "--lang")?.parse()?,
            "--format" => {
                format = Some(match value(args, &mut i, "--format")? {
                    "md" | "markdown" => Format::Markdown,
                    "html" => Format::Html,
                    "json" => Format::Json,
                    other => return Err(format!("unknown format '{other}' (use md, html or json)")),
                })
            }
            "--output" | "-o" => output = Some(value(args, &mut i, "--output")?.into()),
            "--no-mcm" => ov.skip_mcm = true,
            "--trials" => {
                let n: u64 = value(args, &mut i, "--trials")?.parse().map_err(|_| "--trials needs an integer")?;
                if n < 2 {
                    return Err("--trials must be at least 2".into());
                }
                ov.trials = Some(n as usize);
            }
            "--seed" => ov.seed = Some(value(args, &mut i, "--seed")?.parse().map_err(|_| "--seed needs an integer")?),
            a if a.starts_with('-') => return Err(format!("unknown option '{a}'")),
            a if file.is_none() => file = Some(PathBuf::from(a)),
            a => return Err(format!("unexpected argument '{a}'")),
        }
        i += 1;
    }
    let file = file.ok_or("run needs a project file")?;
    let format = format.unwrap_or_else(|| {
        match output.as_ref().and_then(|o| o.extension()).and_then(|e| e.to_str()).map(str::to_ascii_lowercase) {
            Some(e) if e == "html" || e == "htm" => Format::Html,
            Some(e) if e == "json" => Format::Json,
            _ => Format::Markdown,
        }
    });
    let p = load(&file)?;
    let a = project::run(&p, &ov).map_err(|e| format!("{}: {e}", file.display()))?;
    let generated = today();
    let text = match format {
        Format::Markdown => report::markdown(&p, &a, lang, Some(&generated)),
        Format::Html => report::html(&p, &a, lang, Some(&generated)),
        Format::Json => serde_json::to_string_pretty(&project::results_json(&p, &a)).map_err(|e| e.to_string())? + "\n",
    };
    match output {
        Some(path) => std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?,
        None => print!("{text}"),
    }
    Ok(true)
}

fn collect(path: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_dir() {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(path)
            .map_err(|e| format!("{}: {e}", path.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        entries.sort();
        for e in entries {
            if e.is_dir() || e.extension().is_some_and(|x| x == "json") {
                collect(&e, out)?;
            }
        }
    } else {
        out.push(path.to_path_buf());
    }
    Ok(())
}

fn cmd_check(args: &[String]) -> Result<bool, String> {
    let mut ov = Overrides::default();
    let mut files = Vec::new();
    let mut report_path: Option<PathBuf> = None;
    let mut lang = Lang::En;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--no-mcm" => ov.skip_mcm = true,
            "--report" => report_path = Some(value(args, &mut i, "--report")?.into()),
            "--lang" => lang = value(args, &mut i, "--lang")?.parse()?,
            o if o.starts_with('-') => return Err(format!("unknown option '{o}'")),
            p => collect(Path::new(p), &mut files)?,
        }
        i += 1;
    }
    if files.is_empty() {
        return Err("check needs at least one file or directory".into());
    }
    let (mut passed, mut failed, mut without) = (0, 0, 0);
    let mut items = Vec::new();
    for f in &files {
        let loaded = load(f);
        let outcome = loaded.clone().and_then(|p| project::run(&p, &ov).map(|a| (p, a)).map_err(|e| e.to_string()));
        let file = f.display().to_string().replace('\\', "/");
        match outcome {
            Err(e) => {
                failed += 1;
                println!("ERROR {}: {e}", f.display());
                items.push(report::ValidationItem { file, project: loaded.ok(), result: Err(e) });
            }
            Ok((p, a)) => {
                let checks = project::check(&p, &a);
                if checks.is_empty() {
                    without += 1;
                    println!("  --  {} (no reference values)", f.display());
                } else if checks.iter().all(|c| c.passed) {
                    passed += 1;
                    println!("  ok  {} ({} values)", f.display(), checks.len());
                } else {
                    failed += 1;
                    println!("FAIL  {}", f.display());
                    for c in checks.iter().filter(|c| !c.passed) {
                        println!(
                            "        {}: computed {} , reference {} ± {}",
                            c.what, c.computed, c.expected, c.tolerance
                        );
                    }
                }
                items.push(report::ValidationItem { file, project: Some(p), result: Ok(checks) });
            }
        }
    }
    println!("\n{passed} passed, {failed} failed, {without} without reference values ({} files)", files.len());
    if let Some(path) = report_path {
        let html = path.extension().is_some_and(|e| e.eq_ignore_ascii_case("html") || e.eq_ignore_ascii_case("htm"));
        let date = today();
        let text = if html {
            report::validation_html(&items, !ov.skip_mcm, lang, &date)
        } else {
            report::validation_markdown(&items, !ov.skip_mcm, lang, &date)
        };
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        println!("validation report written to {}", path.display());
    }
    Ok(failed == 0)
}

fn cmd_catalogue(args: &[String]) -> Result<bool, String> {
    let mut lang = Lang::En;
    let mut output: Option<PathBuf> = None;
    let mut files = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--lang" => lang = value(args, &mut i, "--lang")?.parse()?,
            "--output" | "-o" => output = Some(value(args, &mut i, "--output")?.into()),
            o if o.starts_with('-') => return Err(format!("unknown option '{o}'")),
            p => collect(Path::new(p), &mut files)?,
        }
        i += 1;
    }
    if files.is_empty() {
        return Err("catalogue needs at least one file or directory".into());
    }
    let ov = Overrides { skip_mcm: true, ..Overrides::default() };
    let mut items = Vec::new();
    for f in &files {
        let p = load(f)?;
        let e = project::run(&p, &ov).map_err(|e| format!("{}: {e}", f.display()))?;
        items.push((f.display().to_string().replace('\\', "/"), p, e));
    }
    let text = report::catalogue(&items, lang);
    match output {
        Some(path) => std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?,
        None => print!("{text}"),
    }
    Ok(true)
}

/// Today's date (UTC) as YYYY-MM-DD, from the system clock.
fn today() -> String {
    let days = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() / 86_400).unwrap_or(0) as i64;
    // Civil-from-days (H. Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}
