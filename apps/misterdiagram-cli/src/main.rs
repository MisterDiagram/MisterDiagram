//! Headless MisterDiagram command line.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
mod author;
mod settings;
use misterdiagram_opc::{MAX_TOTAL, Package};
use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;
fn info(path: &Path, out: &mut impl Write) -> Result<(), String> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or_default().to_ascii_lowercase();
    if !matches!(ext.as_str(), "vsdx" | "vssx" | "vstx" | "vsdm" | "vssm" | "vstm") {
        return Err("expected .vsdx, .vssx, .vstx, .vsdm, .vssm or .vstm".into());
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
fn render(args: &[std::ffi::OsString]) -> Result<(), String> {
    use misterdiagram_render::{contact, pdf, raster, scene, svg};
    let mut positional = Vec::new();
    let mut page = None;
    let mut options = scene::Options { cached_fallback: true, ..Default::default() };
    let mut masters = false;
    let mut fit = false;
    let mut margin = None;
    let mut scale_seen = false;
    let mut font_names_seen = false;
    let mut i = 1;
    while let Some(arg) = args.get(i) {
        match arg.to_str() {
            Some("--page" | "--scale") => {
                let value = args.get(i + 1).and_then(|s| s.to_str()).ok_or("missing render option value")?;
                if arg == "--page" {
                    if page.is_some() {
                        return Err("usage: --page may be given once".into());
                    }
                    page = Some(value.parse::<usize>().map_err(|_| "invalid page")?);
                } else {
                    if scale_seen {
                        return Err("usage: --scale may be given once".into());
                    }
                    scale_seen = true;
                    options.scale = value.parse().map_err(|_| "invalid scale")?;
                }
                i += 1;
            }
            Some("--masters") => masters = true,
            Some("--fit") => {
                if fit {
                    return Err("usage: --fit may be given once".into());
                }
                fit = true;
            }
            Some("--margin") => {
                if margin.is_some() {
                    return Err("usage: --margin may be given once".into());
                }
                margin = Some(args.get(i + 1).and_then(|s| s.to_str()).ok_or("usage: --margin requires a measurement")?);
                i += 1;
            }
            Some("--font-names") => {
                if font_names_seen {
                    return Err("usage: --font-names may be given once".into());
                }
                font_names_seen = true;
                options.font_names = args
                    .get(i + 1)
                    .and_then(|s| s.to_str())
                    .ok_or("usage: --font-names requires fallback or exact")?
                    .parse()
                    .map_err(|e| format!("usage: {e}"))?;
                i += 1;
            }
            Some("--text-outlines") => {
                if options.text_outlines {
                    return Err("usage: --text-outlines may be given once".into());
                }
                options.text_outlines = true;
            }
            Some(s) if s.starts_with("--") => return Err("usage: unknown render option".into()),
            _ => positional.push(arg),
        }
        i += 1;
    }
    options = options.with_fit(fit, margin).map_err(|e| format!("usage: {e}"))?;
    if masters && fit {
        return Err("usage: --fit is for page exports, not master contact sheets".into());
    }
    let [input, output] = positional.as_slice() else {
        return Err(
            "usage: render FILE [--page N] [--scale S] [--text-outlines] [--font-names fallback|exact] [--fit [--margin M]] OUT.{png,svg,pdf,pptx}; render --masters STENCIL OUT.png"
                .into(),
        );
    };
    let path = Path::new(output);
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
    if !matches!(extension.as_str(), "png" | "svg" | "pdf" | "pptx") {
        return Err("usage: render output extension must be png, svg, pdf or pptx".into());
    }
    if masters && (extension != "png" || page.is_some()) {
        return Err("usage: master contact sheets require PNG and no page selector".into());
    }
    let doc = misterdiagram_vsdx::read(Path::new(input)).map_err(|e| e.to_string())?;
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    if masters {
        let (image, report) = contact::render(&doc)?;
        image.save(path).map_err(|e| e.to_string())?;
        let mut stderr = std::io::stderr().lock();
        for warning in &report.warnings {
            let _ = writeln!(stderr, "warning: {warning}");
        }
        let _ = writeln!(
            stderr,
            "contact {}: masters={} rendered={} blank={} failed={}",
            Path::new(input).display(),
            doc.masters.len(),
            report.rendered,
            report.blank,
            report.failed
        );
        if report.failed > 0 || report.blank > 0 {
            return Err("contact sheet contains blank or failed masters".into());
        }
        return Ok(());
    }
    let page = page.map(|ordinal| doc.page_index(ordinal).ok_or("page ordinal does not exist")).transpose()?;
    options.page_canvas = matches!(extension.as_str(), "svg" | "pdf" | "pptx");
    if extension == "pptx" {
        let output = misterdiagram_export::document(&doc, page, options)?;
        for warning in &output.warnings {
            let _ = writeln!(std::io::stderr().lock(), "warning: {warning}");
        }
        return std::fs::write(path, output.bytes).map_err(|e| e.to_string());
    }
    let mut budget = scene::SceneBudget::default();
    let scenes = if extension == "pdf" && page.is_none() {
        scene::foreground_pages_budgeted(&doc, options, &mut budget)?
    } else {
        vec![scene::page_budgeted(&doc, page.map(Ok).unwrap_or_else(|| scene::default_page_index(&doc))?, options, &mut budget)?]
    };
    for scene in &scenes {
        for warning in &scene.warnings {
            let _ = writeln!(std::io::stderr().lock(), "warning: {warning}");
        }
    }
    let first = scenes.first().ok_or("no foreground pages")?;
    match extension.as_str() {
        "png" => raster::render(first, options)?.save(path).map_err(|e| e.to_string()),
        "svg" => std::fs::write(path, svg::render(first, options)?).map_err(|e| e.to_string()),
        "pdf" => std::fs::write(path, pdf::render(&scenes, options)?).map_err(|e| e.to_string()),
        _ => Err("render output extension must be png, svg, pdf or pptx".into()),
    }
}
/// `misterdiagram 0.1.0-beta.4 (0123456789ab, aarch64-apple-darwin)`; the commit comes from the
/// release workflow (build.rs), so source builds say "unknown commit".
fn version_line(commit: Option<&str>, target: &str) -> String {
    format!("misterdiagram {} ({}, {target})", env!("CARGO_PKG_VERSION"), commit.unwrap_or("unknown commit"))
}
fn execute(args: &[std::ffi::OsString], out: &mut impl Write) -> Result<(), String> {
    if args.len() > 4096 || args.iter().try_fold(0usize, |n, arg| n.checked_add(arg.as_encoded_bytes().len())).is_none_or(|n| n > 8 * 1024 * 1024) {
        return Err("argument count or byte limit".into());
    }
    let (arguments, memory) = settings::parse(args, std::env::var_os("MISTERDIAGRAM_MEMORY_LIMIT"))?;
    let (arguments, stencil_path) = settings::stencil_path(&arguments, std::env::var_os("MISTERDIAGRAM_STENCIL_PATH"))?;
    let args = arguments.as_slice();
    match args.first().and_then(|a| a.to_str()) {
        Some("--help") if args.len() == 1 => writeln!(out, "misterdiagram: info, sheet, render, convert, run, commands, describe, corpus check, mcp\nrender FILE [--page N] [--scale S] [--text-outlines] [--font-names fallback|exact] [--fit [--margin M]] OUT.{{png,svg,pdf,pptx}}\nconvert IN OUT [--text-outlines] [--font-names fallback|exact] [--fit [--margin M]]\nSVG uses editable text by default; --text-outlines writes glyph paths. --font-names exact names only the document face.\n--fit crops all formats to foreground drawing content; --margin adds a physical length (for example 5 mm) on each side.").map_err(|e|e.to_string()),
        Some("render" | "convert") if args.len()==2 && args.get(1).is_some_and(|a|a=="--help") => writeln!(out, "render FILE [--page N] [--scale S] [--text-outlines] [--font-names fallback|exact] [--fit [--margin M]] OUT.{{png,svg,pdf,pptx}}\nconvert IN OUT [--text-outlines] [--font-names fallback|exact] [--fit [--margin M]]\nSVG uses editable text by default; --text-outlines writes glyph paths. --font-names exact names only the document face.\n--fit crops all formats to foreground drawing content; --margin adds a physical length (for example 5 mm) on each side.").map_err(|e|e.to_string()),
        Some("--version") if args.len() == 1 => {
            writeln!(out, "{}", version_line(option_env!("MISTERDIAGRAM_COMMIT"), env!("MISTERDIAGRAM_TARGET"))).map_err(|e| e.to_string())
        }
        Some("info") if args.len() == 2 => info(Path::new(args.get(1).ok_or("missing file")?), out),
        Some("sheet") if matches!(args.len(), 2..=4) => {
            use misterdiagram_vsdx::dump::Selection;
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
            let doc = misterdiagram_vsdx::read(Path::new(args.get(1).ok_or("missing file")?)).map_err(|e| e.to_string())?;
            let themes = misterdiagram_theme::Themes::from_document(&doc).map_err(|e| e.to_string())?;
            misterdiagram_vsdx::dump::dump_with_hosts(&doc, selection, Some(&themes), Some(&misterdiagram_text::Host), out)?;
            for warning in misterdiagram_render::scene::validate_foreign_parts(&doc, selection)? {
                let _ = writeln!(std::io::stderr().lock(), "warning: {warning}");
            }
            Ok(())
        }
        Some("corpus") if args.get(1).and_then(|a| a.to_str()) == Some("check") && args.len() == 2 => {
            let report = misterdiagram_corpus::run(Path::new("corpus"), false)?;
            writeln!(out, "{report}").map_err(|e| e.to_string())?;
            if report.passed() { Ok(()) } else { Err("corpus oracle gates failed".into()) }
        }
        Some("render") => render(args),
        Some("commands") => author::commands(args, out),
        Some("describe") => author::describe(args, out),
        Some("run") => author::run(args, out, memory, stencil_path),
        Some("convert") => author::convert(args, out, memory),
        Some("mcp") => {
            if args.len() != 1 {
                return Err("usage: misterdiagram mcp [--memory-limit MiB] [--stencil-path FOLDERS]".into());
            }
            let mut session = misterdiagram_engine::Session::with_memory_limits(memory);
            session.set_stencil_path(stencil_path).map_err(|e| e.to_string())?;
            let mut server = misterdiagram_mcp::server::Server::new(Box::new(misterdiagram_mcp::headless::Headless { session }));
            server.serve(std::io::stdin().lock(), out).map_err(|e| e.to_string())
        }
        _ => Err("usage: misterdiagram [--memory-limit MiB] [--stencil-path FOLDERS] <info FILE|sheet FILE|corpus check|render|convert|run|commands|describe|mcp>".into()),
    }
}
fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).take(4097).collect();
    match execute(&args, &mut std::io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let _ = writeln!(std::io::stderr().lock(), "error: {}", e.replace(['\n', '\r'], " "));
            if e.starts_with("usage:") { ExitCode::from(2) } else { ExitCode::FAILURE }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_is_the_crate_version() {
        let mut out = Vec::new();
        execute(&["--version".into()], &mut out).unwrap();
        let expected = format!(
            "misterdiagram {} ({}, {})\n",
            env!("CARGO_PKG_VERSION"),
            option_env!("MISTERDIAGRAM_COMMIT").unwrap_or("unknown commit"),
            env!("MISTERDIAGRAM_TARGET")
        );
        assert_eq!(String::from_utf8(out).unwrap(), expected);
        assert!(!env!("MISTERDIAGRAM_TARGET").is_empty());
        assert!(execute(&["--version".into(), "extra".into()], &mut Vec::new()).is_err());
    }
    #[test]
    fn version_line_forms() {
        let v = env!("CARGO_PKG_VERSION");
        assert_eq!(version_line(None, "x86_64-unknown-linux-musl"), format!("misterdiagram {v} (unknown commit, x86_64-unknown-linux-musl)"));
        assert_eq!(version_line(Some("0123456789ab"), "aarch64-apple-darwin"), format!("misterdiagram {v} (0123456789ab, aarch64-apple-darwin)"));
    }
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
    #[test]
    fn sheet_output_write_failure_stays_an_error() {
        struct Refuse;
        impl Write for Refuse {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "test writer closed"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/brief66-sheet-stream").join(std::process::id().to_string());
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("original.vsdx");
        misterdiagram_engine::Session::default().execute("file.save", &serde_json::json!({"path":path})).unwrap();
        let error = execute(&["sheet".into(), path.into_os_string()], &mut Refuse).unwrap_err();
        assert!(error.contains("cannot write ShapeSheet dump"), "{error}");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
