//! Field findings from the 2026-10-26 tester batch (MisterDiagram#101, #105, #107), each
//! witnessed against Visio 2021 (private corpus field/vahegan-2026-10-26): guides never
//! print or export, a closed path carries no line ends, and the SVG page is transparent.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
//! https://learn.microsoft.com/en-us/office/client-developer/visio/infiniteline-row-geometry-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/endarrow-cell-line-format-section
use kurbo::Shape as _;
use misterdiagram_model::{Attributes, Cell, Document, Row, Sheet, skeleton};
use misterdiagram_render::{scene, svg};

fn set(sheet: &mut Sheet, name: &str, value: impl ToString) {
    sheet.cells.retain(|c| c.name != name);
    sheet.cells.push(Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() });
}
fn page() -> Document {
    let mut doc = skeleton::document(false);
    set(&mut doc.pages[0].sheet, "PageWidth", 8);
    set(&mut doc.pages[0].sheet, "PageHeight", 6);
    doc.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap());
    doc
}
/// A horizontal line through (0, 0) with direction (1, 0), dotted (pattern 23) and hairline,
/// as Visio writes a guide; `shape_type` decides whether it is a guide or an ordinary shape.
fn infinite_line(id: u32, shape_type: &str) -> misterdiagram_model::Shape {
    let mut shape = skeleton::rectangle(id, 0.5, 3., 5., 1., "").unwrap();
    shape.shape_type = Some(shape_type.into());
    set(&mut shape.sheet, "Height", 0);
    set(&mut shape.sheet, "LocPinY", 0);
    set(&mut shape.sheet, "LinePattern", 23);
    set(&mut shape.sheet, "LineWeight", 0);
    set(&mut shape.sheet, "FillPattern", 0);
    let cell = |name: &str, value: f64| Cell { name: name.into(), value: Some(value.to_string()), ..Default::default() };
    shape.sheet.sections[0].rows = vec![Row {
        index: Some(1),
        row_type: Some("InfiniteLine".into()),
        attributes: Attributes::from([("IX".into(), "1".into()), ("T".into(), "InfiniteLine".into())]),
        cells: vec![cell("X", 0.), cell("Y", 0.), cell("A", 1.), cell("B", 0.)],
        ..Default::default()
    }];
    shape
}
fn ops(doc: &Document) -> usize {
    scene::page(doc, 0, scene::Options::default()).unwrap().ops.len()
}

#[test]
fn guides_are_not_painted() {
    let plain = page();
    let mut with_guide = page();
    with_guide.pages[0].shapes.push(infinite_line(2, "Guide"));
    assert_eq!(ops(&with_guide), ops(&plain), "a Guide shape adds no scene operation");
}

#[test]
fn infinite_line_rows_are_clipped_to_the_page_before_dash_expansion() {
    let mut doc = page();
    doc.pages[0].shapes.push(infinite_line(2, "Shape"));
    let scene = scene::page(&doc, 0, scene::Options::default()).expect("a dotted infinite line stays inside the dash budget");
    let widest = scene
        .ops
        .iter()
        .filter_map(|op| match op {
            scene::Op::Stroke { path, .. } | scene::Op::HairlineStroke { path, .. } => Some(path.bounding_box().width()),
            _ => None,
        })
        .fold(0., f64::max);
    assert!(widest > 7.9 && widest < 10.5, "the line spans the 8 in page plus one inch of margin each side, not 20,000 in: {widest}");
}

#[test]
fn closed_paths_draw_no_line_ends() {
    let plain = page();
    let mut arrows = page();
    for name in ["BeginArrow", "EndArrow"] {
        set(&mut arrows.pages[0].shapes[0].sheet, name, 5);
    }
    assert_eq!(ops(&arrows), ops(&plain), "arrow cells on a closed rectangle paint nothing extra");

    let open = |arrow: f64| {
        let mut doc = page();
        let shape = &mut doc.pages[0].shapes[0];
        shape.sheet.sections[0].rows.truncate(2);
        set(&mut shape.sheet, "FillPattern", 0);
        set(&mut shape.sheet, "EndArrow", arrow);
        doc
    };
    assert!(ops(&open(5.)) > ops(&open(0.)), "the same cell on an open line still paints its arrowhead");
}

#[test]
fn svg_has_no_background_rectangle() {
    let scene = scene::page(&page(), 0, scene::Options::default()).unwrap();
    let out = svg::render(&scene, scene::Options::default()).unwrap();
    assert!(out.starts_with("<svg "), "{}", &out[..out.len().min(80)]);
    assert!(!out.contains("width=\"100%\""), "no page-sized background element: {}", &out[..out.len().min(200)]);
}
