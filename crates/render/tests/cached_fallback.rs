use misterdiagram_model::{Cell, Document, Page, Shape, Sheet};
use misterdiagram_render::scene::{self, Options};
fn cell(name: &str, value: &str, formula: Option<&str>) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), formula: formula.map(str::to_string), ..Default::default() }
}
/// MisterDiagram#78: a flowchart whose phase separators need LISTORDER rendered nothing at
/// all. User-facing renders now take the value Visio saved for such a cell and say so.
#[test]
fn render_uses_the_saved_value_where_a_function_is_missing_and_says_so() {
    let doc = Document {
        pages: vec![Page {
            id: 7,
            name: "Page-1".into(),
            name_u: "Page-1".into(),
            sheet: Sheet { cells: vec![cell("PageWidth", "8", None), cell("PageHeight", "6", None)], ..Default::default() },
            shapes: vec![Shape {
                id: 1,
                name: "Separator".into(),
                sheet: Sheet {
                    cells: vec![
                        cell("PinX", "2", Some("LISTORDER()*2")),
                        cell("PinY", "3", None),
                        cell("Width", "1", None),
                        cell("Height", "1", None),
                    ],
                    ..Default::default()
                },
                ..Default::default()
            }]
            .into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let error = scene::page(&doc, 0, Options::default()).unwrap_err();
    assert!(error.contains("page 7 shape 1") && error.contains("Separator") && error.contains("transform PinX"), "{error}");
    let scene = scene::page(&doc, 0, Options { cached_fallback: true, ..Default::default() }).unwrap();
    assert_eq!(scene.warnings.len(), 1, "{:?}", scene.warnings);
    let warning = scene.warnings[0].to_ascii_lowercase();
    assert!(warning.starts_with("page 7 shape 1 (separator) pinx: #name?") && warning.ends_with("; used the value saved in the file"), "{warning}");
}
