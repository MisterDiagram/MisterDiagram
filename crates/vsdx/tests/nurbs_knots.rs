//! Contributor-original mixed-precision repeated knots, witnessed in Visio M5.13f.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/nurbs-function
//! https://learn.microsoft.com/en-us/office/client-developer/visio/nurbsto-row-geometry-section
use misterdiagram_geom::Rect;
use misterdiagram_model::{Cell, Document, Master, Row, Section};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
fn cell(name: &str, value: &str, formula: &str) -> Cell {
    Cell {
        name: name.into(),
        value: Some(value.into()),
        formula: (!formula.is_empty()).then(|| formula.into()),
        units: (name == "E").then(|| "NURBS".into()),
        ..Default::default()
    }
}
fn fixture(knot: &str, descending: bool) -> Document {
    let mut shape = misterdiagram_model::skeleton::rectangle(5, 0., 0., 3., 2., "").unwrap();
    shape.sheet.sections.retain(|s| s.name != "Geometry");
    shape.sheet.sections.push(Section {
        name: "User".into(),
        rows: vec![Row { name: Some("Knot".into()), cells: vec![cell("Value", knot, "")], ..Default::default() }],
        ..Default::default()
    });
    shape.sheet.sections.push(Section {
        name: "Geometry".into(),
        index: Some(0),
        cells: vec![cell("NoFill", "1", "")],
        rows: vec![
            Row { index: Some(1), row_type: Some("MoveTo".into()), cells: vec![cell("X", "0", ""), cell("Y", "0", "")], ..Default::default() },
            Row {
                index: Some(2),
                row_type: Some("NURBSTo".into()),
                cells: vec![
                    cell("X", "3", ""),
                    cell("Y", "1", ""),
                    cell("A", knot, if descending { "User.Knot-0.000001" } else { "User.Knot" }),
                    cell("B", "1", ""),
                    cell("C", "0", ""),
                    cell("D", "1", ""),
                    cell("E", "NURBS(2,2,1,1,1,1,0,1,2,0,1,1)", "NURBS(User.Knot*2,2,1,1,1,1,0,1,2,0,User.Knot,1)"),
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    });
    let mut doc = misterdiagram_model::skeleton::document(false);
    doc.masters = vec![Master { id: 1, name_u: "OriginalRepeatedKnot".into(), shapes: vec![shape], ..Default::default() }];
    misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&doc).unwrap().bytes).unwrap()
}
#[test]
fn constructor_precision_repeated_knots_render_without_losing_real_descents() {
    let scope = Scope::MasterShape { master: 1, shape: 5 };
    let clip = Rect::new(-10., -10., 10., 10.);
    let reference = fixture("1", false);
    let mut e = Evaluator::new(&reference).unwrap();
    let reference = e.geometry_paths(scope, clip).unwrap()[0].path.control_box();
    for knot in ["0.1234567890123457", "0.9876543210987654", "1.054321098765466", "1.234567890123457", "0.7138452034029296"] {
        let doc = fixture(knot, false);
        let mut e = Evaluator::new(&doc).unwrap();
        let paths = e.geometry_paths(scope, clip).unwrap_or_else(|err| panic!("mixed constructor precision knot {knot}: {err:?}"));
        assert_eq!(paths.len(), 1);
        assert!(paths[0].stroked() && !paths[0].filled());
        let bounds = paths[0].path.control_box();
        for (actual, expected) in [(bounds.x0, reference.x0), (bounds.y0, reference.y0), (bounds.x1, reference.x1), (bounds.y1, reference.y1)] {
            assert!((actual - expected).abs() < 1e-9, "affine knot scaling changed curve control bounds: {knot}");
        }
        let bad = fixture(knot, true);
        let mut e = Evaluator::new(&bad).unwrap();
        assert!(e.geometry_paths(scope, clip).is_err(), "genuinely descending knot {knot}");
    }
}
