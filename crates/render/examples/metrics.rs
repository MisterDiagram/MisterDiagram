#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
fn main() -> Result<(), String> {
    for file in std::env::args().skip(1) {
        let doc = diagramcraft_vsdx::read(&file).map_err(|e| e.to_string())?;
        let themes = diagramcraft_theme::Themes::from_document(&doc).map_err(|e| e.to_string())?;
        let mut e = diagramcraft_vsdx::evaluate::Evaluator::new(&doc)
            .map_err(|e| e.to_string())?
            .with_theme_host(&themes)
            .with_text_host(&diagramcraft_text::Host);
        let scopes: Vec<_> = e.scopes().collect();
        for scope in scopes {
            for named in e.source_named_cells(scope).map_err(|e| e.to_string())? {
                let formula = e.effective_formula(scope, &named.name).unwrap_or_default();
                if !formula.contains("TEXTHEIGHT") && !formula.contains("TEXTWIDTH") {
                    continue;
                }
                let expected = e.cached_cell(named.cell);
                let actual = e.evaluate_snapshot(scope, &named.name);
                let body = e.text_body(scope);
                let desc = body.map(|b| {
                    format!(
                        "text={:?} font={:?} paras={:?} insets={:?}",
                        b.paragraphs.iter().flat_map(|p| &p.runs).map(|r| r.text.as_str()).collect::<String>(),
                        b.paragraphs.first().map(|p| &p.end_props),
                        b.paragraphs.iter().map(|p| &p.props).collect::<Vec<_>>(),
                        b.body
                    )
                });
                println!("{file}\t{scope:?}\t{}\t{formula}\t{expected:?}\t{actual:?}\t{desc:?}", named.name);
            }
        }
    }
    Ok(())
}
