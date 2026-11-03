//! Exact output-model raster check for one local acceptance drawing.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let input = args.next().ok_or("usage: model_check FILE.vsdx")?;
    if args.next().is_some() {
        return Err("usage: model_check FILE.vsdx".into());
    }
    let report = misterdiagram_export::qa::file(std::path::Path::new(&input))?;
    print!("{report}");
    if !report.passed() {
        return Err("PPTX output-model comparison failed".into());
    }
    Ok(())
}
