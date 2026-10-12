//! Headless Diagramcraft command line.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use diagramcraft_opc::{MAX_TOTAL, Package};
use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;
fn info(path: &Path, out: &mut impl Write) -> Result<(), String> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or_default().to_ascii_lowercase();
    if !matches!(ext.as_str(), "vsdx" | "vssx" | "vstx" | "vsdm") {
        return Err("expected .vsdx, .vssx, .vstx or .vsdm".into());
    }
    let f = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    f.take(MAX_TOTAL + 1).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_TOTAL {
        return Err("compressed package exceeds size cap".into());
    }
    let p = Package::open(&bytes).map_err(|e| e.to_string())?;
    p.validate_visio().map_err(|e| e.to_string())?;
    for (name, data) in p.parts() {
        let ct = if name == "[Content_Types].xml" {
            "application/xml".into()
        } else {
            p.content_type(name).ok_or_else(|| format!("no content type for {name}"))?
        };
        writeln!(out, "{name}\t{ct}\t{}", data.len()).map_err(|e| e.to_string())?;
    }
    for r in p.rels("").items {
        writeln!(out, "relationship\t{}\t{}\t{}\texternal={}", r.id, r.kind, r.target, r.external).map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn execute(args: &[std::ffi::OsString], out: &mut impl Write) -> Result<(), String> {
    match args.first().and_then(|a| a.to_str()) {
        Some("info") if args.len() == 2 => info(Path::new(args.get(1).ok_or("missing file")?), out),
        Some("sheet") if matches!(args.len(), 2..=4) => {
            use diagramcraft_vsdx::dump::Selection;
            let selection = match args.get(2).and_then(|a| a.to_str()) {
                None if args.len() == 2 => Selection::All,
                Some("--document") if args.len() == 3 => Selection::Document,
                Some(flag @ ("--page" | "--master" | "--shape")) if args.len() == 4 => {
                    let n = args
                        .get(3)
                        .and_then(|a| a.to_str())
                        .ok_or("missing numeric selector")?
                        .parse::<u32>()
                        .map_err(|_| "invalid numeric selector")?;
                    match flag {
                        "--page" => Selection::Page(n),
                        "--master" => Selection::Master(n),
                        _ => Selection::Shape(n),
                    }
                }
                _ => return Err("usage: sheet FILE [--shape N | --page N | --master N | --document]".into()),
            };
            let doc = diagramcraft_vsdx::read(Path::new(args.get(1).ok_or("missing file")?)).map_err(|e| e.to_string())?;
            let themes = diagramcraft_theme::Themes::from_document(&doc).map_err(|e| e.to_string())?;
            diagramcraft_vsdx::dump::dump_with_theme(&doc, selection, Some(&themes), out)
        }
        Some("corpus") if args.get(1).and_then(|a| a.to_str()) == Some("check") && args.len() == 2 => {
            let report = diagramcraft_corpus::run(Path::new("corpus"), false)?;
            writeln!(out, "{report}").map_err(|e| e.to_string())?;
            if report.passed() { Ok(()) } else { Err("corpus rates are below the frozen floors".into()) }
        }
        Some("render" | "convert" | "run" | "commands" | "describe" | "mcp") => Err("not implemented in Phase 1".into()),
        _ => Err("usage: diagramcraft-cli <info FILE|sheet FILE|corpus check|render|convert|run|commands|describe|mcp>".into()),
    }
}
fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).take(17).collect();
    match execute(&args, &mut std::io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let _ = writeln!(std::io::stderr().lock(), "error: {}", e.replace(['\n', '\r'], " "));
            if e == "not implemented in Phase 1" || e.starts_with("usage:") { ExitCode::from(2) } else { ExitCode::FAILURE }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hostile_cli_selectors() {
        for args in [
            vec!["sheet"],
            vec!["sheet", "missing", "--page", "-1"],
            vec!["sheet", "missing", "--page", "4294967296"],
            vec!["sheet", "missing", "--document", "extra"],
            vec!["corpus", "check", "extra"],
        ] {
            let args = args.into_iter().map(std::ffi::OsString::from).collect::<Vec<_>>();
            assert!(execute(&args, &mut Vec::new()).is_err());
        }
    }
}
