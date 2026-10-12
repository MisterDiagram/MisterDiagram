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
fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.first().and_then(|a| a.to_str()) {
        Some("info") if args.len() == 2 => {
            let result = args.get(1).ok_or_else(|| "missing file".into()).and_then(|p| info(Path::new(p), &mut std::io::stdout().lock()));
            match result {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("error: {}", e.replace(['\n', '\r'], " "));
                    ExitCode::FAILURE
                }
            }
        }
        Some("sheet" | "render" | "convert" | "run" | "commands" | "describe" | "mcp" | "corpus") => {
            eprintln!("not implemented in Phase 0");
            ExitCode::from(2)
        }
        _ => {
            eprintln!("usage: diagramcraft-cli <info FILE|sheet|render|convert|run|commands|describe|mcp|corpus>");
            ExitCode::from(2)
        }
    }
}
