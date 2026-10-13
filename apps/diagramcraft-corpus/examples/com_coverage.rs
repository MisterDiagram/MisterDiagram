//! Read-only provenance split of the main COM oracle.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use std::{io::Read, path::Path};
fn membership(path: &str) -> Result<String, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.take(1024 * 1024 + 1).read_to_string(&mut text).map_err(|e| e.to_string())?;
    Ok(text)
}
fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let old = args.next().ok_or("usage: com_coverage OLD_MEMBERSHIP_FILE [COMBINED_MEMBERSHIP_FILE]")?;
    let combined = args.next();
    if args.next().is_some() {
        return Err("too many coverage arguments".into());
    }
    let mut c = diagramcraft_corpus::coverage::Coverage::from_list(&membership(&old)?)?;
    if let Some(combined) = combined {
        c = c.with_combined(&membership(&combined)?)?;
    }
    println!("{}", diagramcraft_corpus::coverage::compare(Path::new("corpus"), c)?);
    Ok(())
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}
