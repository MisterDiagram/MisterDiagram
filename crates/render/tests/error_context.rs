//! Contributor-written error sheets exercise every diagnostic provider without private fixtures.
use misterdiagram_model::{Cell, Document, Master, Page, Row, Section, Shape, Sheet, Style};
use misterdiagram_render::scene::{self, Options};
fn formula(name: &str, expression: &str) -> Cell {
    Cell { name: name.into(), formula: Some(expression.into()), ..Default::default() }
}
fn doc() -> Document {
    Document {
        pages: vec![Page { id: 1, shapes: vec![Shape { id: 7, name_u: "Badge".into(), ..Default::default() }].into(), ..Default::default() }],
        ..Default::default()
    }
}
fn assert_context(doc: &Document, labels: &[&str]) {
    for cached_fallback in [false, true] {
        let error = scene::page(doc, 0, Options { cached_fallback, ..Default::default() }).unwrap_err();
        assert!(error.contains("#DIV/0!"), "{error}");
        for label in labels {
            assert!(error.contains(label), "missing {label} in {error}");
        }
    }
}
#[test]
fn errors_name_shape_page_document_master_style_and_layer_cells() {
    let mut d = doc();
    d.pages[0].shapes[0].sheet.cells.push(formula("PinX", "1/0"));
    assert_context(&d, &["page 1 shape 7", "Badge", "PinX"]);
    let mut d = doc();
    d.pages[0].sheet.cells.push(formula("PageWidth", "1/0"));
    assert_context(&d, &["page 1 pagesheet", "PageWidth"]);
    let mut d = doc();
    d.document_sheet.sections.push(Section {
        name: "User".into(),
        rows: vec![Row { name: Some("Fault".into()), cells: vec![formula("Value", "1/0")], ..Default::default() }],
        ..Default::default()
    });
    d.pages[0].sheet.cells.push(formula("PageWidth", "TheDoc!User.Fault"));
    assert_context(&d, &["page 1 pagesheet", "document sheet", "User.Fault"]);
    let mut d = doc();
    d.masters.push(Master {
        id: 9,
        shapes: vec![Shape {
            id: 2,
            name_u: "Source".into(),
            sheet: Sheet { cells: vec![formula("PinX", "1/0")], ..Default::default() },
            ..Default::default()
        }]
        .into(),
        ..Default::default()
    });
    d.pages[0].shapes[0].master = Some(9);
    assert_context(&d, &["page 1 shape 7", "Badge", "master 9 shape 2", "Source", "PinX"]);
    let mut d = doc();
    // Drawing style cells now recover their default in cached mode. A User cell on
    // the style sheet still tests a stopping error from that provider in both modes.
    d.styles.push(Style {
        id: 4,
        sheet: Sheet {
            sections: vec![Section {
                name: "User".into(),
                rows: vec![Row { name: Some("Fault".into()), cells: vec![formula("Value", "1/0")], ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    });
    d.pages[0].shapes[0].sheet.cells.push(formula("PinX", "Styles!Sheet.4!User.Fault"));
    assert_context(&d, &["page 1 shape 7", "Badge", "style sheet 4", "User.Fault"]);
    let mut d = doc();
    d.pages[0].sheet.sections.push(Section {
        name: "Layer".into(),
        rows: vec![Row { index: Some(0), cells: vec![formula("Visible", "1/0")], ..Default::default() }],
        ..Default::default()
    });
    d.pages[0].shapes[0].sheet.cells.push(formula("PinX", "ThePage!Layers.Visible"));
    assert_context(&d, &["page 1 shape 7", "Badge", "page 1 pagesheet", "Layers.Visible"]);
}

#[test]
fn handled_dependency_error_is_not_reported_as_the_uncaught_error() {
    for second in ["Missing", "1/0"] {
        let mut d = doc();
        d.document_sheet.sections.push(Section {
            name: "User".into(),
            rows: vec![
                Row { name: Some("A".into()), cells: vec![formula("Value", "1/0")], ..Default::default() },
                Row { name: Some("B".into()), cells: vec![formula("Value", second)], ..Default::default() },
            ],
            ..Default::default()
        });
        d.pages[0].shapes[0].sheet.cells.push(formula("PinX", "IFERROR(TheDoc!User.A,0)+TheDoc!User.B"));
        let error = scene::page(&d, 0, Options::default()).unwrap_err();
        assert!(!error.contains("User.A"), "{error}");
        assert!(error.contains("document sheet") && (error.contains("User.B") || error.contains("Missing")), "{error}");
    }
}

#[test]
fn handled_dependency_does_not_name_a_later_literal_arithmetic_failure() {
    let mut d = doc();
    d.document_sheet.sections.push(Section {
        name: "User".into(),
        rows: vec![Row { name: Some("A".into()), cells: vec![formula("Value", "1/0")], ..Default::default() }],
        ..Default::default()
    });
    d.pages[0].shapes[0].sheet.cells.push(formula("PinX", "IFERROR(TheDoc!User.A,0)+1/0"));
    let error = scene::page(&d, 0, Options::default()).unwrap_err();
    assert!(error.contains("PinX") && error.contains("#DIV/0!"), "{error}");
    assert!(!error.contains("User.A"), "{error}");
}

#[test]
fn saved_error_reference_does_not_blame_a_consumed_dependency() {
    let mut d = doc();
    let mut saved = formula("Value", "1/0");
    saved.value = Some("5".into());
    saved.error = Some("133".into());
    d.document_sheet.sections.push(Section {
        name: "User".into(),
        rows: vec![Row { name: Some("Recovered".into()), cells: vec![saved], ..Default::default() }],
        ..Default::default()
    });
    d.pages[0].shapes[0].sheet.cells.push(formula("PinX", "TheDoc!User.Recovered+1/0"));
    for cached_fallback in [false, true] {
        let error = scene::page(&d, 0, Options { cached_fallback, ..Default::default() }).unwrap_err();
        assert!(error.contains("PinX") && error.contains("#DIV/0!"), "{error}");
        assert!(!error.contains("User.Recovered"), "{error}");
    }
}
