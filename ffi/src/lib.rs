//! C interface of *KAPSAM* for the graphical user
//! interface (Flutter, via `dart:ffi`).
//!
//! Every function takes and returns UTF-8 JSON strings, so the interface
//! stays small and the project file remains the single document model. A
//! returned string is owned by the caller and must be released with
//! [`uc_free`]. Panics are caught and returned as errors.
//!
//! Responses have the form `{"ok": true, …}` or `{"ok": false, "error": "…"}`.

use serde_json::{json, Value};
use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};
use uncertainty_project::{self as project, report, Lang, Overrides, CITATION, SOFTWARE_NAME, SOFTWARE_VERSION};

const TEMPLATE_MODEL: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": { "title": { "en": "New measurement", "tr": "Yeni ölçüm" } },
  "model": {
    "output": { "name": "y", "expression": "x1 + x2" },
    "inputs": [
      { "name": "x1", "distribution": { "type": "normal", "mean": 1.0, "sd": 0.01 } },
      { "name": "x2", "distribution": { "type": "rectangular", "value": 0.0, "half_width": 0.02 } }
    ]
  },
  "analysis": { "gum": { "coverage_factor": 2 }, "mcm": { "trials": 200000, "seed": 1 } }
}"#;

const TEMPLATE_TOPDOWN: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": { "title": { "en": "New top-down evaluation", "tr": "Yeni top-down değerlendirme" } },
  "topdown": {
    "quantity": "c",
    "unit": "mg/L",
    "levels": [1.0],
    "precision": [ { "name": "u(Rw)", "data": { "type": "given", "sd_percent": 2.0 } } ],
    "bias": [ { "kind": "crm", "references": [ { "label": "CRM", "reference": 1.0, "u_reference_percent": 1.0, "mean": 1.01, "sd_percent": 2.0, "n": 10 } ] } ]
  }
}"#;

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
}"#;

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
}"#;

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
}"#;

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
}"#;

const TEMPLATE_CONSENSUS: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Consensus value of an interlaboratory comparison", "tr": "Laboratuvarlar arası karşılaştırmanın uzlaşı değeri" },
    "field": "chemistry"
  },
  "consensus": {
    "measurand": "w(Ni)",
    "unit": "mg/kg",
    "results": [
      { "name": "Lab A", "value": 1.930, "u": 0.110 }, { "name": "Lab B", "value": 1.984, "u": 0.020 },
      { "name": "Lab C", "value": 2.022, "u": 0.023 }, { "name": "Lab D", "value": 2.050, "u": 0.020 },
      { "name": "Lab E", "value": 2.077, "u": 0.035 }, { "name": "Lab F", "value": 2.150, "u": 0.030 }
    ],
    "linear_pool": true,
    "bayes": {},
    "trials": 100000
  }
}
"#;

const TEMPLATE_SAMPLING: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Uncertainty from sampling by the duplicate method", "tr": "Çift numune yöntemiyle numune alma belirsizliği" },
    "field": "chemistry"
  },
  "sampling": {
    "quantity": "Dissolved iron in groundwater, one well",
    "unit": "mg/L",
    "duplicates": [
      [0.815, 0.834, 0.912, 0.893], [1.8, 1.83, 1.94, 1.93], [1.69, 1.68, 1.79, 1.77],
      [2.62, 2.61, 2.83, 2.84], [1.66, 1.63, 1.58, 1.59], [1.52, 1.53, 1.47, 1.50]
    ],
    "labels": ["99.474", "99.468", "99.469", "99.916", "99.327", "99.371"]
  }
}
"#;

const TEMPLATE_ERROR_FUNCTION: &str = r#"{
  "format": "uncertainty-calculations/project",
  "version": 1,
  "metadata": {
    "title": { "en": "Error approximation function of a balance", "tr": "Terazinin hata yaklaşım fonksiyonu" },
    "field": "mass"
  },
  "error_function": {
    "instrument": "Electronic balance, Max 400 g, d = 0.1 mg",
    "unit": "g",
    "error_unit": "mg",
    "error_factor": 1000,
    "through_zero": true,
    "s_m": 0.05,
    "points": [
      {"indication": 0.0, "reference": 0.0, "u_reference": 0.0, "u_indication": 0.041633},
      {"indication": 50.000067, "reference": 50.000006, "u_reference": 0.014201, "u_indication": 0.051174},
      {"indication": 100.0001, "reference": 99.999987, "u_reference": 0.023057, "u_indication": 0.052678},
      {"indication": 150.000233, "reference": 149.999993, "u_reference": 0.037282, "u_indication": 0.055095},
      {"indication": 200.000267, "reference": 200.000013, "u_reference": 0.038426, "u_indication": 0.05831},
      {"indication": 250.0001, "reference": 250.000019, "u_reference": 0.052835, "u_indication": 0.062199},
      {"indication": 300.0002, "reference": 300.0, "u_reference": 0.06152, "u_indication": 0.066646},
      {"indication": 350.000267, "reference": 350.000006, "u_reference": 0.075871, "u_indication": 0.071545},
      {"indication": 400.0004, "reference": 400.00001, "u_reference": 0.076852, "u_indication": 0.076811}
    ],
    "readings": [ { "reading": 120 } ]
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
}"#;

fn into_c(v: Value) -> *mut c_char {
    let s = v.to_string();
    // JSON text never contains a NUL byte (it would be escaped as \u0000).
    CString::new(s).expect("JSON contains no NUL").into_raw()
}

fn error(message: impl Into<String>) -> Value {
    json!({ "ok": false, "error": message.into() })
}

/// Reads a C string; a null pointer is an empty string.
///
/// # Safety
/// `p` must be null or point to a NUL-terminated string valid for reads.
unsafe fn read(p: *const c_char) -> Result<String, String> {
    if p.is_null() {
        return Ok(String::new());
    }
    CStr::from_ptr(p).to_str().map(str::to_owned).map_err(|_| "argument is not valid UTF-8".to_string())
}

fn guarded(f: impl FnOnce() -> Value) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(v) => into_c(v),
        Err(_) => into_c(error("internal error (panic) — please report it with the project file")),
    }
}

/// Software name, version and citation.
#[no_mangle]
pub extern "C" fn uc_version() -> *mut c_char {
    guarded(|| json!({ "ok": true, "name": SOFTWARE_NAME, "version": SOFTWARE_VERSION, "citation": CITATION }))
}

/// A starting project: `kind` is `"model"` (default), `"topdown"`,
/// `"proficiency"`, `"calibration"`, `"conformity"`, `"precision"`, `"multivariate"`, `"control_chart"`, `"sampling"`, `"error_function"` or `"consensus"`.
///
/// # Safety
/// `kind` must be null or a valid NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn uc_template(kind: *const c_char) -> *mut c_char {
    let kind = read(kind);
    guarded(move || match kind.as_deref() {
        Ok("topdown") => json!({ "ok": true, "project": TEMPLATE_TOPDOWN }),
        Ok("proficiency") => json!({ "ok": true, "project": TEMPLATE_PROFICIENCY }),
        Ok("calibration") => json!({ "ok": true, "project": TEMPLATE_CALIBRATION }),
        Ok("conformity") => json!({ "ok": true, "project": TEMPLATE_CONFORMITY }),
        Ok("precision") => json!({ "ok": true, "project": TEMPLATE_PRECISION }),
        Ok("multivariate") => json!({ "ok": true, "project": TEMPLATE_MULTIVARIATE }),
        Ok("control_chart") => json!({ "ok": true, "project": TEMPLATE_CONTROL_CHART }),
        Ok("sampling") => json!({ "ok": true, "project": TEMPLATE_SAMPLING }),
        Ok("error_function") => json!({ "ok": true, "project": TEMPLATE_ERROR_FUNCTION }),
        Ok("consensus") => json!({ "ok": true, "project": TEMPLATE_CONSENSUS }),
        Ok("" | "model") => json!({ "ok": true, "project": TEMPLATE_MODEL }),
        Ok(other) => error(format!("unknown template '{other}'")),
        Err(e) => error(e.clone()),
    })
}

/// Checks a project file without evaluating it.
///
/// # Safety
/// `project` must be null or a valid NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn uc_validate(project_json: *const c_char) -> *mut c_char {
    let text = read(project_json);
    guarded(move || match text.and_then(|t| project::parse(&t).map_err(|e| e.to_string())) {
        Ok(_) => json!({ "ok": true }),
        Err(e) => error(e),
    })
}

/// Evaluates a project. `options` (JSON, may be null):
/// `{"lang": "en"|"tr", "skip_mcm": bool, "trials": n, "seed": s,
///   "markdown": bool, "html": bool, "generated": "date text"}`.
/// The response holds `results` (as `kapsam run --format json`,
/// including the comparison with reference values), `reported` (the result
/// statements in the requested language), `warnings` (in that language)
/// and the requested reports.
///
/// # Safety
/// Both arguments must be null or valid NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn uc_evaluate(project_json: *const c_char, options_json: *const c_char) -> *mut c_char {
    let text = read(project_json);
    let opts = read(options_json);
    guarded(move || evaluate(text, opts))
}

fn evaluate(text: Result<String, String>, opts: Result<String, String>) -> Value {
    let text = match text {
        Ok(t) => t,
        Err(e) => return error(e),
    };
    let opts: Value = match opts {
        Ok(o) if o.trim().is_empty() => json!({}),
        Ok(o) => match serde_json::from_str(&o) {
            Ok(v) => v,
            Err(e) => return error(format!("invalid options: {e}")),
        },
        Err(e) => return error(e),
    };
    let lang = match opts.get("lang").and_then(Value::as_str).unwrap_or("en").parse::<Lang>() {
        Ok(l) => l,
        Err(e) => return error(e),
    };
    let ov = Overrides {
        skip_mcm: opts.get("skip_mcm").and_then(Value::as_bool).unwrap_or(false),
        trials: opts.get("trials").and_then(Value::as_u64).map(|n| n as usize),
        seed: opts.get("seed").and_then(Value::as_u64),
    };
    if ov.trials.is_some_and(|n| n < 2) {
        return error("trials must be at least 2");
    }
    let p = match project::parse(&text) {
        Ok(p) => p,
        Err(e) => return error(e.to_string()),
    };
    let e = match project::run(&p, &ov) {
        Ok(e) => e,
        Err(e) => return error(e.to_string()),
    };
    let generated = opts.get("generated").and_then(Value::as_str);
    let mut out = json!({
        "ok": true,
        "results": project::results_json(&p, &e),
        "reported": report::reported_statements(&p, &e, lang),
        "warnings": e
            .model()
            .and_then(|a| a.gum.as_ref())
            .map(|g| g.warnings.iter().map(|w| report::warning_text(w, lang)).collect::<Vec<_>>())
            .unwrap_or_default(),
    });
    if opts.get("markdown").and_then(Value::as_bool).unwrap_or(false) {
        out["markdown"] = json!(report::markdown(&p, &e, lang, generated));
    }
    if opts.get("html").and_then(Value::as_bool).unwrap_or(false) {
        out["html"] = json!(report::html(&p, &e, lang, generated));
    }
    if opts.get("pdf").and_then(Value::as_bool).unwrap_or(false) {
        out["pdf_base64"] = json!(base64(&report::pdf_report(&p, &e, lang, generated)));
    }
    out
}

/// Standard base64 (RFC 4648) of `data`, for the PDF report in the JSON reply.
fn base64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for c in data.chunks(3) {
        let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if c.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if c.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    out
}

/// Releases a string returned by this library.
///
/// # Safety
/// `s` must be null or a pointer returned by a function of this library that
/// has not been freed yet.
#[no_mangle]
pub unsafe extern "C" fn uc_free(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(p: *mut c_char) -> Value {
        let v = serde_json::from_str(unsafe { CStr::from_ptr(p) }.to_str().unwrap()).unwrap();
        unsafe { uc_free(p) };
        v
    }

    #[test]
    fn version_templates_and_evaluation() {
        assert_eq!(call(uc_version())["version"], SOFTWARE_VERSION);
        for kind in [
            "model",
            "topdown",
            "proficiency",
            "calibration",
            "conformity",
            "precision",
            "multivariate",
            "control_chart",
            "sampling",
            "error_function",
            "consensus",
        ] {
            let k = CString::new(kind).unwrap();
            let t = call(unsafe { uc_template(k.as_ptr()) });
            let project = CString::new(t["project"].as_str().unwrap()).unwrap();
            assert_eq!(call(unsafe { uc_validate(project.as_ptr()) })["ok"], true);
            let opts = CString::new(r#"{"lang": "tr", "html": true, "trials": 20000}"#).unwrap();
            let r = call(unsafe { uc_evaluate(project.as_ptr(), opts.as_ptr()) });
            assert_eq!(r["ok"], true, "{r}");
            assert!(r["html"].as_str().unwrap().contains("<svg"));
            assert!(!r["reported"].as_array().unwrap().is_empty());
        }
        let k = CString::new("model").unwrap();
        let t = call(unsafe { uc_template(k.as_ptr()) });
        let project = CString::new(t["project"].as_str().unwrap()).unwrap();
        let r = call(unsafe { uc_evaluate(project.as_ptr(), std::ptr::null()) });
        assert!(r["results"]["mcm"]["histogram"]["counts"].as_array().unwrap().len() > 10);
        assert_eq!(r["results"]["gum_pdf"].as_array().unwrap().len(), 201);
    }

    #[test]
    fn errors_are_reported_not_raised() {
        let bad = CString::new("{ not json").unwrap();
        let r = call(unsafe { uc_evaluate(bad.as_ptr(), std::ptr::null()) });
        assert_eq!(r["ok"], false);
        assert!(r["error"].as_str().unwrap().contains("invalid project file"));
        let r = call(unsafe { uc_evaluate(std::ptr::null(), std::ptr::null()) });
        assert_eq!(r["ok"], false);
        let k = CString::new("nonsense").unwrap();
        assert_eq!(call(unsafe { uc_template(k.as_ptr()) })["ok"], false);
    }

    #[test]
    fn base64_matches_rfc_4648() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }
}
