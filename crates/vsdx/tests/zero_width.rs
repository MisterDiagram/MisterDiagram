//! Contributor-original fixtures for the accepted M5.13e collapsed-stroke witnesses.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/ellipticalarcto-row-geometry-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/about-functions
use misterdiagram_geom::{BezPath, Rect};
use misterdiagram_model::{Cell, Master, Row, Section};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};

fn cell(name: &str, value: &str, formula: &str) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), formula: (!formula.is_empty()).then(|| formula.into()), ..Default::default() }
}

#[test]
fn action_zero_width_arcs_match_visio_collapsed_strokes() {
    for (name, angle, ratio, top) in [
        ("RatioSide", "0", "Width/Height", false),
        ("RatioTop", "0", "Width/Height", true),
        ("TinyAngle", "-6.921655089364709E-15", "Width/Height", false),
        (
            "ThetaNear",
            "_ELLIPSE_THETA(-1.5707963267949,1.0000202209969,19,2,Width,Height)",
            "_ELLIPSE_ECC(-1.5707963267949,1.0000202209969,19,2,Width,Height,Geometry1.C2)",
            false,
        ),
        (
            "ThetaCircle",
            "_ELLIPSE_THETA(-1.5707963267949,1,19,2,Width,Height)",
            "_ELLIPSE_ECC(-1.5707963267949,1,19,2,Width,Height,Geometry1.C2)",
            false,
        ),
    ] {
        let mut shape = misterdiagram_model::skeleton::rectangle(5, 0., 0., 2., 2., "").unwrap();
        *shape.sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap() = cell("Width", "0", "GUARD(2*Actions.Visible.Action)");
        shape.sheet.sections.retain(|s| s.name != "Geometry");
        shape.sheet.sections.push(Section {
            name: "Actions".into(),
            rows: vec![Row {
                name: Some("Visible".into()),
                cells: vec![cell("Action", "0", "NOT(\"Actions.Visible.Action\")")],
                ..Default::default()
            }],
            ..Default::default()
        });
        shape.sheet.sections.push(Section {
            name: "Geometry".into(),
            index: Some(0),
            cells: vec![cell("NoFill", "1", "")],
            rows: vec![
                Row {
                    index: Some(1),
                    row_type: Some("MoveTo".into()),
                    cells: vec![cell("X", "0", "Width*0.5"), cell("Y", if top { "1" } else { "0" }, if top { "Height*0.5" } else { "0" })],
                    ..Default::default()
                },
                Row {
                    index: Some(2),
                    row_type: Some("EllipticalArcTo".into()),
                    cells: vec![
                        cell("X", "0", "Width"),
                        cell("Y", if top { "1" } else { "2" }, if top { "Height*0.5" } else { "Height" }),
                        cell("A", "0", "Width"),
                        cell("B", if top { "2" } else { "1" }, if top { "Height" } else { "Height*0.5" }),
                        cell("C", "0", angle),
                        cell("D", "0", ratio),
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        let mut doc = misterdiagram_model::skeleton::document(false);
        doc.masters = vec![Master { id: 1, name_u: name.into(), shapes: vec![shape].into(), ..Default::default() }];
        // Exercise our own original package fixture, including import and unchanged save.
        let doc = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&doc).unwrap().bytes).unwrap();
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::MasterShape { master: 1, shape: 5 };
        assert_eq!(e.evaluate(scope, "Width"), Value::Number(0.), "{name}");
        if name.starts_with("Theta") {
            assert_eq!(e.evaluate(scope, "Geometry1.C2"), Value::Number(0.), "{name}");
            assert_eq!(e.evaluate(scope, "Geometry1.D2"), Value::Number(1.), "{name}");
        }
        let paths = e.geometry_paths(scope, Rect::new(-10., -10., 10., 10.)).unwrap_or_else(|err| panic!("{name}: {err:?}"));
        assert_eq!(paths.len(), 1, "{name}");
        let mut expected = BezPath::new();
        expected.move_to((0., if top { 1. } else { 0. }));
        if !name.starts_with("Theta") {
            expected.line_to((0., if top { 2. } else { 1. }));
        }
        expected.line_to((0., if top { 1. } else { 2. }));
        if top {
            expected.close_path();
        }
        assert_eq!(paths[0].path, expected, "{name}: collapsed arc must reach its control and end");
        assert!(paths[0].stroked() && !paths[0].filled());
    }
}
