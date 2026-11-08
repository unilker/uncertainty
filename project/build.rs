//! Records the identity of the build for the reports (ISO/IEC 17025:2017
//! 7.11.2, 7.5): the git commit (with "-modified" when the working tree had
//! uncommitted changes), its date and the compiler version. Builds outside a
//! git checkout report "unknown".

use std::process::Command;

fn run(cmd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8(out.stdout).ok()?.trim().to_string();
    (!s.is_empty()).then_some(s)
}

fn main() {
    let commit = run("git", &["rev-parse", "--short=12", "HEAD"]).map(|c| {
        let modified = run("git", &["status", "--porcelain", "--untracked-files=no"]).is_some();
        if modified {
            format!("{c}-modified")
        } else {
            c
        }
    });
    let date = run("git", &["log", "-1", "--format=%cs"]);
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let rustc = run(&rustc, &["--version"]);
    println!("cargo:rustc-env=UC_GIT_COMMIT={}", commit.unwrap_or_else(|| "unknown".into()));
    println!("cargo:rustc-env=UC_COMMIT_DATE={}", date.unwrap_or_else(|| "unknown".into()));
    println!("cargo:rustc-env=UC_RUSTC={}", rustc.unwrap_or_else(|| "unknown".into()));
    for p in ["../.git/HEAD", "../.git/index", "../.git/refs/heads"] {
        println!("cargo:rerun-if-changed={p}");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
