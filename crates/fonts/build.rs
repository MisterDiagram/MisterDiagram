#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
//! Pinned OFL Latin faces are always embedded. Optional craft-fonts adds other scripts.
//! Build input is local only; builds never download fonts.
use std::fmt::Write as _;
use std::path::PathBuf;

/// Families MisterDiagram never embeds (AGENTS.md §1: no Adobe-authored visual design).
const EXCLUDED_FAMILIES: &[&str] = &["Source Sans 3", "Source Serif 4", "Source Han Sans", "Source Han Serif", "Noto Sans CJK SC"];

fn main() {
    println!("cargo::rerun-if-env-changed=CRAFT_FONTS_DIR");
    println!("cargo::rerun-if-env-changed=CRAFT_FONTS_REQUIRED");
    let local = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default()).join("assets");
    let fallback = local.join("carlito/Carlito-Regular.ttf");
    println!("cargo::rerun-if-changed={}", fallback.display());
    println!("cargo::rustc-env=MISTERDIAGRAM_FALLBACK_FONT={}", fallback.display());
    let mut src = String::from("pub static CRAFT_FONTS: &[CraftFont] = &[\n");
    for (dir, family) in [("carlito", "Carlito"), ("caladea", "Caladea"), ("tinos", "Tinos"), ("cousine", "Cousine")] {
        for (suffix, style) in [("Regular", "Regular"), ("Bold", "Bold"), ("Italic", "Italic"), ("BoldItalic", "Bold Italic")] {
            embedded(&mut src, &local.join(dir).join(format!("{family}-{suffix}.ttf")), family, style);
        }
    }
    embedded(&mut src, &local.join("arimo/Arimo[wght].ttf"), "Arimo", "Variable");
    embedded(&mut src, &local.join("arimo/Arimo-Italic[wght].ttf"), "Arimo", "Variable Italic");
    // The symbol face (#106): dingbats, geometric shapes, miscellaneous symbols and the extended
    // arrow and pictograph blocks; no Latin letters, so it only ever wins a symbol cluster.
    embedded_for(&mut src, &local.join("noto-sans-symbols2/NotoSansSymbols2-Regular.ttf"), "Noto Sans Symbols 2", "Regular", "Zsym");
    // An empty CRAFT_FONTS_DIR (the release workflow) means none: only the pinned Latin faces.
    if let Some(dir) = std::env::var_os("CRAFT_FONTS_DIR").filter(|d| !d.is_empty()).map(PathBuf::from) {
        match craft_fonts(&dir) {
            Ok(entries) => src.push_str(&entries),
            Err(e) if std::env::var_os("CRAFT_FONTS_REQUIRED").is_some() => {
                println!("cargo::error=CRAFT_FONTS_DIR={}: {e}", dir.display());
            }
            Err(e) => println!("cargo::warning=building without craft-fonts: CRAFT_FONTS_DIR={}: {e}", dir.display()),
        }
    }
    src.push_str("];\n");
    if fallback.is_file() {
        let _ = writeln!(src, "pub(crate) static FALLBACK_BYTES: &[u8] = include_bytes!({:?});", fallback);
    } else {
        src.push_str("pub(crate) static FALLBACK_BYTES: &[u8] = &[];\n");
    }
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default()).join("craft_fonts.rs");
    if let Err(e) = std::fs::write(&out, src) {
        println!("cargo::error=writing {}: {e}", out.display());
    }
}

/// One `CraftFont { .. }` initialiser per manifest line.
fn craft_fonts(dir: &std::path::Path) -> Result<String, String> {
    let mut out = String::new();
    // The Latin presentation fonts are optional within craft-fonts (older checkouts lack them).
    let latin = dir.join("fonts/latin-manifest.txt");
    watch_optional(&latin);
    if latin.exists() {
        out.push_str(&manifest(dir, &latin)?);
    }
    out.push_str(&manifest(dir, &dir.join("fonts/manifest.txt"))?);
    Ok(out)
}

fn manifest(dir: &std::path::Path, manifest: &std::path::Path) -> Result<String, String> {
    watch_optional(manifest);
    let text = std::fs::read_to_string(manifest).map_err(|e| format!("{}: {e}", manifest.display()))?;
    let wasm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|a| a == "wasm32");
    let mut out = String::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let f: Vec<&str> = line.split(" | ").map(str::trim).collect();
        let [family, style, file, scripts, ..] = f.as_slice() else {
            return Err(format!("malformed manifest line: {line}"));
        };
        // MisterDiagram's asset policy: no Adobe-authored typefaces, even openly licensed ones.
        if EXCLUDED_FAMILIES.contains(family) {
            continue;
        }
        if wasm && !matches!((*family, *style), ("BIZ UDPGothic", "Regular") | ("Inter", "Regular" | "Bold" | "Variable")) {
            continue;
        }
        let path = dir.join(file).canonicalize().map_err(|e| format!("{file}: {e}"))?;
        println!("cargo::rerun-if-changed={}", path.display());
        let scripts: Vec<String> = scripts.split(',').map(|s| format!("{:?}", s.trim())).collect();
        let _ = writeln!(
            out,
            "    CraftFont {{ family: {family:?}, style: {style:?}, scripts: &[{}], bytes: include_bytes!({:?}) }},",
            scripts.join(", "),
            path.display().to_string(),
        );
    }
    Ok(out)
}

fn embedded(out: &mut String, path: &std::path::Path, family: &str, style: &str) {
    embedded_for(out, path, family, style, "Latn");
}

fn embedded_for(out: &mut String, path: &std::path::Path, family: &str, style: &str, script: &str) {
    println!("cargo::rerun-if-changed={}", path.display());
    let _ = writeln!(out, "    CraftFont {{ family: {family:?}, style: {style:?}, scripts: &[{script:?}], bytes: include_bytes!({:?}) }},", path);
}

// Cargo treats a missing watched file as perpetually changed. Watch its nearest
// existing directory instead so adding an optional manifest still triggers a build.
fn watch_optional(path: &std::path::Path) {
    if let Some(existing) = path.ancestors().find(|p| p.exists()) {
        println!("cargo::rerun-if-changed={}", existing.display());
    }
}
