//! Contributor-original regressions for metadata exposed by the P4a master-drop oracle.
use diagramcraft_model::{Cell, Row, Section, skeleton};
fn cell(name: &str, value: &str) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), ..Default::default() }
}
/// Relative coordinates are ratios; C is an angle and D is an axis ratio.
/// Microsoft's RelEllipticalArcTo example uses X/Y1, A1.5, B0.5, C15deg, D1.5.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/relellipticalarcto-row-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/ask-cell-shape-data-section
#[test]
fn relative_geometry_and_verify_have_com_units_and_slots() {
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 0., 0., 2., 3., "").unwrap();
    shape.sheet.sections = vec![
        Section {
            name: "Geometry".into(),
            index: Some(0),
            rows: vec![
                Row { index: Some(1), row_type: Some("RelMoveTo".into()), cells: vec![cell("X", "0"), cell("Y", "0.5")], ..Default::default() },
                Row {
                    index: Some(2),
                    row_type: Some("RelEllipticalArcTo".into()),
                    cells: vec![
                        cell("X", "1"),
                        cell("Y", "1"),
                        cell("A", "1.5"),
                        cell("B", "0.5"),
                        Cell { units: Some("DA".into()), ..cell("C", &(std::f64::consts::PI / 12.).to_string()) },
                        cell("D", "1.5"),
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        Section {
            name: "Property".into(),
            rows: vec![Row {
                name: Some("Owner".into()),
                index: Some(0),
                cells: vec![cell("Value", "0"), cell("Verify", "1")],
                ..Default::default()
            }],
            ..Default::default()
        },
    ];
    doc.pages[0].shapes.push(shape);
    let mut dump = Vec::new();
    diagramcraft_vsdx::dump::dump(&doc, diagramcraft_vsdx::dump::Selection::Shape(1), &mut dump).unwrap();
    let text = String::from_utf8(dump).unwrap();
    for (name, units) in [
        ("Geometry1.X1", "32"),
        ("Geometry1.Y1", "32"),
        ("Geometry1.X2", "32"),
        ("Geometry1.A2", "32"),
        ("Geometry1.B2", "32"),
        ("Geometry1.C2", "81"),
        ("Geometry1.D2", "32"),
        ("Prop.Owner.Verify", "32"),
    ] {
        let row = text.lines().map(|l| l.split('\t').collect::<Vec<_>>()).find(|r| r.get(5) == Some(&name)).unwrap();
        assert_eq!(row[8], units, "{name}");
        if name == "Prop.Owner.Verify" {
            assert_eq!(&row[2..5], &["243", "0", "7"]);
        }
        if name == "Geometry1.C2" {
            assert!((row[7].parse::<f64>().unwrap() - std::f64::consts::PI / 12.).abs() < 1e-6);
        }
    }
}
/// A cached palette hit still serializes a constant RGB formula as literal color.
/// Inherited and theme fallback colors retain their standard palette indices.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linecolor-cell-line-format-section
#[test]
fn palette_hits_drop_constant_rgb_formulas_and_preserve_inherited_standard_colors() {
    let mut doc = skeleton::document(false);
    doc.colors.insert(28, "#CC0000".into());
    let style = doc.styles.iter_mut().find(|s| s.id == 6).unwrap();
    let background = style.sheet.cells.iter_mut().find(|c| c.name == "FillBkgnd").unwrap();
    *background = Cell { formula: Some("THEMEVAL(\"FillColor2\",15)".into()), ..cell("FillBkgnd", "15") };
    let mut shape = skeleton::rectangle(1, 0., 0., 2., 1., "").unwrap();
    shape.sheet.cells.push(Cell { formula: Some("RGB(204,0,0)".into()), ..cell("FillForegnd", "1") });
    doc.pages[0].shapes.push(shape);
    let written = diagramcraft_vsdx::write(&doc).unwrap();
    let read = diagramcraft_vsdx::import(&written.bytes).unwrap();
    let color = read.pages[0].shapes[0].sheet.cell("FillForegnd").unwrap();
    assert_eq!(color.formula, None);
    assert_eq!(color.value.as_deref(), Some("#cc0000"));
    let style = read.styles.iter().find(|s| s.id == 6).unwrap();
    assert_eq!(style.sheet.cell("BevelDepthColor").unwrap().value.as_deref(), Some("1"));
    assert_eq!(style.sheet.cell("FillBkgnd").unwrap().value.as_deref(), Some("15"));
}
