//! Runs every project file of the example library (`library/`) and compares
//! the results with the reference values stored in the file.

use std::path::{Path, PathBuf};
use uncertainty_project::{check, parse, report, run, Lang, Overrides};

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

#[test]
fn every_library_example_reproduces_its_reference_values() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../library");
    let mut files = Vec::new();
    collect(&root, &mut files);
    files.sort();
    assert!(files.len() >= 203, "library has only {} files", files.len());
    let mut failures = Vec::new();
    for f in &files {
        let text = std::fs::read_to_string(f).unwrap();
        let p = parse(&text).unwrap_or_else(|e| panic!("{}: {e}", f.display()));
        let id = p.metadata.id.clone().unwrap_or_default();
        assert_eq!(
            Some(id.as_str()),
            f.file_stem().and_then(|s| s.to_str()),
            "{}: id must match the file name",
            f.display()
        );
        assert!(p.metadata.title.tr.is_some(), "{id}: Turkish title missing");
        assert!(!p.metadata.sources.is_empty(), "{id}: no source");
        let reference = p.expected.as_ref().and_then(|e| e.reference.as_ref());
        assert!(reference.is_some_and(|r| r.tr.is_some()), "{id}: bilingual reference text missing");
        let a = run(&p, &Overrides::default()).unwrap_or_else(|e| panic!("{id}: {e}"));
        let checks = check(&p, &a);
        assert!(!checks.is_empty(), "{id}: no reference values");
        for c in checks.iter().filter(|c| !c.passed) {
            failures.push(format!("{id}: {} = {} (reference {} ± {})", c.what, c.computed, c.expected, c.tolerance));
        }
        for lang in [Lang::En, Lang::Tr] {
            let md = report::markdown(&p, &a, lang, None);
            assert!(md.contains(uncertainty_project::CITATION), "{id}: citation missing from report");
            let html = report::html(&p, &a, lang, None);
            assert!(html.contains("<svg"), "{id}: charts missing from HTML report");
            assert!(!md.contains("{p}") && !md.contains("{v}"), "{id}: unfilled label template");
        }
    }
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
}

/// The worked-examples chapter of the user guide must match the library.
#[test]
fn guide_catalogue_is_up_to_date() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    collect(&repo.join("library"), &mut files);
    let ov = Overrides { skip_mcm: true, ..Overrides::default() };
    let items: Vec<_> = files
        .iter()
        .map(|f| {
            let p = parse(&std::fs::read_to_string(f).unwrap()).unwrap();
            let e = run(&p, &ov).unwrap();
            let rel = f.strip_prefix(&repo).unwrap().to_string_lossy().replace('\\', "/");
            (rel, p, e)
        })
        .collect();
    for (lang, code) in [(Lang::En, "en"), (Lang::Tr, "tr")] {
        let path = repo.join(format!("docs/guide/{code}/examples.md"));
        let committed = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(
            committed == report::catalogue(&items, lang),
            "{} is out of date: run `uncertainty catalogue library --lang {code} -o docs/guide/{code}/examples.md`",
            path.display()
        );
    }
}

/// The library bundled with the graphical interface must match `library/`.
#[test]
fn app_library_bundle_is_up_to_date() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    collect(&repo.join("library"), &mut files);
    files.sort();
    let bundle_path = repo.join("app/assets/library.json");
    let bundle: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&bundle_path).unwrap_or_else(|_| "[]".into())).unwrap();
    let expected: Vec<serde_json::Value> = files
        .iter()
        .map(|f| {
            let rel = f.strip_prefix(&repo).unwrap().to_string_lossy().replace('\\', "/");
            let project: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(f).unwrap()).unwrap();
            serde_json::json!({ "path": rel, "project": project })
        })
        .collect();
    assert!(
        bundle == serde_json::Value::Array(expected),
        "{} is out of date: run `dart run tool/bundle_library.dart` in app/",
        bundle_path.display()
    );
}
