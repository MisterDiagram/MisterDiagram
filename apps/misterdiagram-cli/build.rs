#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
//! Build identity for `--version`: the target triple Cargo builds for, and the commit when the
//! release workflow supplies one in MISTERDIAGRAM_COMMIT. Never runs git, so a source build is
//! reproducible and reports "unknown commit".

fn main() {
    println!("cargo::rerun-if-env-changed=MISTERDIAGRAM_COMMIT");
    println!("cargo::rustc-env=MISTERDIAGRAM_TARGET={}", std::env::var("TARGET").unwrap_or_default());
    if let Some(commit) = std::env::var_os("MISTERDIAGRAM_COMMIT") {
        let commit = commit.to_string_lossy();
        if commit.len() != 12 || !commit.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
            println!("cargo::error=MISTERDIAGRAM_COMMIT must be 12 lowercase hex digits, got {commit:?}");
        }
    }
}
