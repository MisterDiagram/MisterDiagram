//! Raster comparison of editable SVG with the unchanged PNG scene.
//! Usage: svg_raster DRAWING.vsdx [PAGE] [MAD_CEILING]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#[path = "../../../crates/render/tests/support/svg_fonts.rs"]
mod svg_fonts;
use misterdiagram_render::{raster, scene, svg};
fn run() -> Result<bool, String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let file = args.first().ok_or("usage: svg_raster DRAWING.vsdx [PAGE] [MAD_CEILING]")?;
    let doc = misterdiagram_vsdx::read(file).map_err(|e| e.to_string())?;
    let page = args
        .get(1)
        .map(|s| s.parse::<usize>().map_err(|e| e.to_string()).and_then(|n| n.checked_sub(1).ok_or("page is one-based".into())))
        .transpose()?
        .unwrap_or(scene::default_page_index(&doc)?);
    let limit = args.get(2).map(|s| s.parse::<f64>().map_err(|e| e.to_string())).transpose()?.unwrap_or(2.);
    if !limit.is_finite() || limit < 0. {
        return Err("invalid pixel ceiling".into());
    }
    let opts = scene::Options { cached_fallback: true, ..Default::default() };
    let scene = scene::page(&doc, page, opts)?;
    let png = raster::render(&scene, opts)?;
    let xml = svg::render(&scene, opts)?;
    let tree = resvg::usvg::Tree::from_str(&xml, &svg_fonts::options()).map_err(|e| e.to_string())?;
    let mut pixels = resvg::tiny_skia::Pixmap::new(png.width(), png.height()).ok_or("raster allocation")?;
    pixels.fill(resvg::tiny_skia::Color::WHITE);
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixels.as_mut());
    let mut sum = 0u64;
    let mut different = 0usize;
    for (a, b) in png.pixels().zip(pixels.pixels()) {
        let diffs = [
            a.0.first().copied().unwrap_or(0).abs_diff(b.red()),
            a.0.get(1).copied().unwrap_or(0).abs_diff(b.green()),
            a.0.get(2).copied().unwrap_or(0).abs_diff(b.blue()),
        ];
        sum += diffs.iter().map(|n| u64::from(*n)).sum::<u64>();
        different += usize::from(diffs.iter().any(|n| *n > 16));
    }
    let count = f64::from(png.width()) * f64::from(png.height());
    let mad = sum as f64 / (count * 3.);
    let fraction = different as f64 / count;
    println!("{file}\t{}\t{mad:.9}\t{fraction:.9}\t{limit:.9}\t{}", page + 1, mad <= limit);
    Ok(mad <= limit)
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(true) => std::process::ExitCode::SUCCESS,
        Ok(false) => std::process::ExitCode::FAILURE,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}
