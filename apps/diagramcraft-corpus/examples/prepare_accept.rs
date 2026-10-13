//! Materialize the private P3 selection without modifying observation inputs.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use diagramcraft_render::{
    raster,
    scene::{self, Options},
};
use std::{
    fs,
    path::{Component, Path, PathBuf},
};
fn relative(value: &str, prefix: &str) -> Result<PathBuf, String> {
    let p = Path::new(value);
    if value.len() > 1024 || !p.starts_with(prefix) || p.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err("acceptance path outside its private directory".into());
    }
    Ok(p.to_path_buf())
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let expected_only = match args.as_slice() {
        [] => false,
        [arg] if arg == "--expected-only" => true,
        _ => return Err("usage: prepare_accept [--expected-only]".into()),
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize()?;
    let plan_path = root.join("corpus/_state/m3/accept-plan.json");
    if fs::metadata(&plan_path)?.len() > 1024 * 1024 {
        return Err("plan byte limit".into());
    }
    let plan: serde_json::Value = serde_json::from_slice(&fs::read(plan_path)?)?;
    let entries = plan.get("files").and_then(|v| v.as_array()).ok_or("plan files missing")?;
    if entries.len() != 230 {
        return Err("expected 230 acceptance files".into());
    }
    let mut names = std::collections::BTreeSet::new();
    let expected = root.join("corpus/_state/m3/accept-expected");
    fs::create_dir_all(&expected)?;
    if !expected.canonicalize()?.starts_with(&root) {
        return Err("expected directory symlink".into());
    }
    for entry in entries {
        let output = root.join(relative(entry.get("output").and_then(|v| v.as_str()).ok_or("output missing")?, "corpus/_state/m3/accept-inputs")?);
        let name = output.file_stem().and_then(|s| s.to_str()).ok_or("invalid name")?;
        if !names.insert(name.to_ascii_lowercase()) {
            return Err("duplicate acceptance name".into());
        }
        let source = match entry.get("source").and_then(|v| v.as_str()) {
            Some(s) => root.join(relative(s, "corpus")?),
            None => root.join("corpus/_state/m3/scratch").join(output.file_name().ok_or("filename")?),
        };
        if !source.canonicalize()?.starts_with(root.join("corpus")) {
            return Err("source symlink outside corpus".into());
        }
        let (doc, warning_count) = if expected_only {
            (diagramcraft_vsdx::read(&output)?, 0)
        } else {
            let before = diagramcraft_vsdx::read(&source)?;
            let theme = diagramcraft_theme::Themes::from_document(&before).map_err(|e| e.to_string())?;
            let written = diagramcraft_vsdx::write_with_hosts(&before, Some(&theme), Some(&diagramcraft_text::Host))?;
            fs::create_dir_all(output.parent().ok_or("output parent")?)?;
            if !output.parent().ok_or("parent")?.canonicalize()?.starts_with(&root)
                || fs::symlink_metadata(&output).is_ok_and(|m| m.file_type().is_symlink())
            {
                return Err("output symlink".into());
            }
            fs::write(&output, &written.bytes)?;
            (diagramcraft_vsdx::import(&written.bytes)?, written.warning_count)
        };
        let theme = diagramcraft_theme::Themes::from_document(&doc).map_err(|e| e.to_string())?;
        let dir = expected.join(name);
        fs::create_dir_all(&dir)?;
        if !dir.canonicalize()?.starts_with(&expected) {
            return Err("expected output symlink".into());
        }
        diagramcraft_vsdx::dump::dump_with_hosts(
            &doc,
            diagramcraft_vsdx::dump::Selection::All,
            Some(&theme),
            Some(&diagramcraft_text::Host),
            &mut fs::File::create(dir.join("computed.sheet.tsv"))?,
        )?;
        diagramcraft_vsdx::dump::dump_serialized(&doc, &mut fs::File::create(dir.join("serialized.sheet.tsv"))?)?;
        let mut evaluator = diagramcraft_vsdx::evaluate::Evaluator::new(&doc)
            .map_err(|e| e.to_string())?
            .with_theme_host(&theme)
            .with_text_host(&diagramcraft_text::Host);
        let mut images = vec![];
        let options = Options { alignment_extent: true, ..Default::default() };
        for (master, count) in [(false, doc.pages.len()), (true, doc.masters.len())] {
            for i in 0..count {
                if !master && doc.pages.get(i).is_none_or(|p| p.part.is_empty()) {
                    continue;
                }
                let id = if master { doc.masters.get(i).ok_or("master")?.id } else { doc.pages.get(i).ok_or("page")?.id };
                let empty = if master {
                    doc.masters.get(i).ok_or("master")?.shapes.is_empty()
                } else {
                    doc.pages.get(i).is_some_and(|p| p.shapes.is_empty() && !p.attributes.contains_key("BackPage"))
                };
                let stem = format!("{}-{id}", if master { "master" } else { "page" });
                let result = (|| -> Result<(), String> {
                    let scene =
                        if master { scene::master_prepared(&mut evaluator, i, options) } else { scene::page_prepared(&mut evaluator, i, options) }?;
                    let image = raster::render(&scene, options)?;
                    image.save(dir.join(format!("{stem}.png"))).map_err(|e| e.to_string())
                })();
                images.push(serde_json::json!({"stem":stem,"error":result.err(),"empty":empty}));
            }
        }
        fs::write(dir.join("images.json"), serde_json::to_vec_pretty(&images)?)?;
        eprintln!("prepared {} warnings={}", PathBuf::from(name).display(), warning_count);
    }
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

#[cfg(test)]
mod tests {
    #[test]
    fn hostile_plan_paths_are_rejected() {
        for p in ["/tmp/out", "corpus/../../out", "corpus/_state/m3/accept-inputs/../bad", "elsewhere/out"] {
            assert!(super::relative(p, "corpus/_state/m3/accept-inputs").is_err());
        }
        assert!(super::relative("corpus/_state/m3/accept-inputs/ok.vsdx", "corpus/_state/m3/accept-inputs").is_ok());
    }
}
