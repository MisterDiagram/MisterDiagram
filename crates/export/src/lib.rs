//! MisterDiagram export.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod model;
pub mod qa;
mod write;
pub use write::{Export, render};

/// A presentation has one colour scheme, selected from its first exported page.
pub fn colors(document: &misterdiagram_model::Document, page: usize) -> Result<[misterdiagram_color::Rgba; 12], String> {
    use misterdiagram_color::Rgba;
    use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
    let themes = misterdiagram_theme::Themes::from_document(document).map_err(|e| e.to_string())?;
    let page = document.pages.get(page).ok_or("page index out of range")?;
    let mut evaluator = Evaluator::new(document).map_err(|e| e.to_string())?.with_theme_host(&themes);
    let selector = evaluator.evaluate(Scope::Page(page.id), "ColorSchemeIndex").numeric().map_err(|e| e.to_string())?;
    if !selector.is_finite() || !(0. ..=f64::from(u32::MAX)).contains(&selector) || selector.fract() != 0. {
        return Err("invalid colour scheme selector".into());
    }
    if let Some(colors) = themes.resolved_color_scheme(selector as u32).map_err(|e| e.to_string())? {
        return Ok(colors);
    }
    // Unthemed drawings use their own colour table. No vendor theme is bundled.
    let mut colors = [Rgba::BLACK; 12];
    for (out, index) in colors.iter_mut().zip([0, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]) {
        *out = document.colors.get(&index).and_then(|s| Rgba::from_hex(s)).unwrap_or(if index == 1 { Rgba::WHITE } else { Rgba::BLACK });
    }
    Ok(colors)
}

/// Evaluate and export selected pages through the renderer's shared shape tree.
pub fn document(
    document: &misterdiagram_model::Document,
    page: Option<usize>,
    options: misterdiagram_render::scene::Options,
) -> Result<Export, String> {
    use misterdiagram_render::scene::{self, SceneBudget, evaluated};
    let mut budget = SceneBudget::default();
    let pages = if let Some(index) = page {
        vec![evaluated::page(document, index, options, &mut budget)?]
    } else {
        evaluated::foreground_pages(document, options, &mut budget)?
    };
    let selected = page.map(Ok).unwrap_or_else(|| scene::default_page_index(document))?;
    render(pages, &colors(document, selected)?, options)
}
