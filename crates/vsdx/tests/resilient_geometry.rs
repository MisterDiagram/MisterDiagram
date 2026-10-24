//! Contributor-original recovery and named-row cases. No private document content.
use misterdiagram_geom::Rect;
use misterdiagram_model::{Cell, Document, Master, Page, Row, Section, Shape, Sheet};
use misterdiagram_sheet::{Error, Value};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
fn value(name: &str, v: &str) -> Cell {
    Cell { name: name.into(), value: Some(v.into()), ..Default::default() }
}
fn broken(name: &str, saved: Option<&str>) -> Cell {
    Cell { name: name.into(), value: saved.map(str::to_string), formula: Some("REF()".into()), error: Some("55".into()), ..Default::default() }
}
fn row(index: u32, kind: &str, x: Cell) -> Row {
    Row { index: Some(index), row_type: Some(kind.into()), cells: vec![x, value("Y", "1")], ..Default::default() }
}
fn document(sections: Vec<Section>) -> Document {
    Document {
        pages: vec![Page {
            id: 1,
            shapes: vec![Shape {
                id: 7,
                name_u: "Original badge".into(),
                sheet: Sheet { cells: vec![value("Width", "3"), value("Height", "2")], sections, ..Default::default() },
                ..Default::default()
            }]
            .into(),
            ..Default::default()
        }],
        ..Default::default()
    }
}
const SCOPE: Scope = Scope::PageShape { page: 1, shape: 7 };
fn geometry(rows: Vec<Row>) -> Section {
    Section { name: "Geometry".into(), index: Some(0), rows, ..Default::default() }
}
fn clip() -> Rect {
    Rect::new(-10., -10., 10., 10.)
}
#[test]
fn every_bad_geometry_row_is_skipped_without_losing_the_tail() {
    for kind in ["MoveTo", "LineTo", "ArcTo", "EllipticalArcTo", "NURBSTo", "PolylineTo", "RelMoveTo", "RelLineTo", "RelCubBezTo", "RelQuadBezTo"] {
        let d = document(vec![geometry(vec![row(1, "MoveTo", value("X", "0")), row(2, kind, broken("X", None)), row(3, "LineTo", value("X", "2"))])]);
        let mut strict = Evaluator::new(&d).unwrap();
        assert_eq!(strict.geometry_paths(SCOPE, clip()).unwrap_err(), Error::Ref, "{kind}");
        let mut drawing = Evaluator::new(&d).unwrap().with_cached_fallback();
        let paths = drawing.geometry_paths(SCOPE, clip()).unwrap();
        assert_eq!(paths.len(), 1, "{kind}");
        assert_eq!(paths[0].path.elements().len(), 2, "{kind}");
        let (warnings, count) = drawing.take_fallback_warnings();
        assert_eq!(count, 1, "{kind}: {warnings:?}");
        assert!(warnings[0].contains("page 1 shape 7") && warnings[0].contains("Geometry1.X2"), "{warnings:?}");
    }
}
#[test]
fn saved_geometry_connections_and_style_errors_warn_but_strict_values_stay_errors() {
    let mut d = document(vec![
        geometry(vec![row(1, "MoveTo", value("X", "0")), row(2, "LineTo", broken("X", Some("2")))]),
        Section {
            name: "Connection".into(),
            rows: vec![Row { index: Some(0), cells: vec![broken("X", Some("3")), broken("Y", None)], ..Default::default() }],
            ..Default::default()
        },
    ]);
    d.pages[0].shapes[0].sheet.cells.push(broken("LineWeight", Some("0.025")));
    let mut strict = Evaluator::new(&d).unwrap();
    for name in ["Geometry1.X2", "Connections.X1", "LineWeight"] {
        assert_eq!(strict.evaluate(SCOPE, name), Value::Error(Error::Ref), "{name}");
    }
    let mut e = Evaluator::new(&d).unwrap().with_cached_fallback();
    assert_eq!(e.evaluate(SCOPE, "LineWeight"), Value::Number(0.025));
    assert_eq!(e.geometry_paths(SCOPE, clip()).unwrap()[0].path.elements().len(), 2);
    assert_eq!(e.evaluate(SCOPE, "Connections.X1"), Value::Number(3.));
    let (warnings, count) = e.take_fallback_warnings();
    assert!(count >= 4, "{warnings:?}");
    for name in ["Geometry1.X2", "Connections.X1", "Connections.Y1", "LineWeight"] {
        assert!(warnings.iter().any(|s| s.contains(name)), "{warnings:?}");
    }
    d.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "LineWeight").unwrap().value = None;
    let mut e = Evaluator::new(&d).unwrap().with_cached_fallback();
    assert!(e.evaluate(SCOPE, "LineWeight").numeric().unwrap() > 0.);
    assert!(e.take_fallback_warnings().0[0].contains("default"));
}
#[test]
fn invalid_section_flags_do_not_abort_other_sections() {
    for flag in ["NoFill", "NoLine", "NoShow"] {
        for saved in [None, Some("0")] {
            let mut first = geometry(vec![row(1, "MoveTo", value("X", "0")), row(2, "LineTo", value("X", "2"))]);
            let mut second = first.clone();
            second.index = Some(1);
            first.cells.push(broken(flag, saved));
            let d = document(vec![first, second]);
            let mut e = Evaluator::new(&d).unwrap().with_cached_fallback();
            assert_eq!(e.geometry_paths(SCOPE, clip()).unwrap().len(), if saved.is_some() { 2 } else { 1 });
            let (warnings, count) = e.take_fallback_warnings();
            assert_eq!(count, 1);
            assert!(warnings[0].contains(flag));
        }
    }
}
#[test]
fn local_named_user_row_and_units_only_override_retain_their_providers() {
    let xml=misterdiagram_vsdx::parse_xml(br#"<Shape xmlns="http://schemas.microsoft.com/office/visio/2012/main"><Section N="User"><Row LocalName="ScaleAdj"><Cell N="Value" V="1" F="IF(ThePage!PageScale/ThePage!DrawingScale&gt;0,1,2)"/></Row><Row N="Universal" LocalName="Local"><Cell N="Value" V="7"/></Row></Section></Shape>"#).unwrap();
    let mut source = Shape { id: 2, sheet: misterdiagram_vsdx::parse_sheet(&xml).unwrap(), ..Default::default() };
    source.sheet.sections.push(geometry(vec![row(1, "MoveTo", value("X", "3"))]));
    let mut d = document(vec![geometry(vec![Row {
        index: Some(1),
        cells: vec![Cell { name: "X".into(), units: Some("IN".into()), ..Default::default() }],
        ..Default::default()
    }])]);
    d.pages[0].shapes[0].master = Some(9);
    d.masters.push(Master { id: 9, shapes: vec![source].into(), ..Default::default() });
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(e.evaluate(SCOPE, "User.ScaleAdj"), Value::Number(1.));
    assert_eq!(e.evaluate(SCOPE, "User.Universal"), Value::Number(7.));
    assert_eq!(e.evaluate(SCOPE, "Geometry1.X1"), Value::Number(3.));
    assert_eq!(e.evaluate(SCOPE, "User.Local"), Value::Error(Error::Ref));
    assert!(misterdiagram_vsdx::parse_sheet(&misterdiagram_vsdx::parse_xml(br#"<Shape xmlns="http://schemas.microsoft.com/office/visio/2012/main"><Section N="User"><Row LocalName="Fault" IX="4294967296"/></Section></Shape>"#).unwrap()).is_err());
}

#[test]
fn unknown_row_type_warning_names_that_row_without_diagnostic_state() {
    for kind in [None, Some("UnknownOriginalType")] {
        let mut invalid = row(9, "LineTo", value("X", "1"));
        invalid.row_type = kind.map(str::to_string);
        let d = document(vec![geometry(vec![row(1, "MoveTo", value("X", "0")), invalid, row(10, "LineTo", value("X", "2"))])]);
        let mut e = Evaluator::new(&d).unwrap().with_cached_fallback();
        assert_eq!(e.geometry_paths(SCOPE, clip()).unwrap()[0].path.elements().len(), 2);
        let warnings = e.take_fallback_warnings().0;
        assert!(warnings[0].contains("Geometry1 row 9"), "{warnings:?}");
    }
}

#[test]
fn master_selection_includes_its_page_sheet_identity_and_values() {
    let d = Document {
        masters: vec![Master {
            id: 9,
            name_u: "Original master".into(),
            sheet: Sheet { cells: vec![value("PageWidth", "3")], ..Default::default() },
            ..Default::default()
        }],
        ..Default::default()
    };
    let mut out = Vec::new();
    misterdiagram_vsdx::dump::dump(&d, misterdiagram_vsdx::dump::Selection::Master(9), &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(
        text.lines()
            .any(|line| line.starts_with("0\tmaster:9:Original master/ThePage\t") && line.contains("\tPageWidth\t") && line.contains("\t3\t")),
        "{text}"
    );
}

#[test]
fn oversized_geometry_names_keep_the_limit_without_a_recovery_allocation() {
    let d = document(vec![]);
    let mut e = Evaluator::new(&d).unwrap().with_cached_fallback();
    let name = "Geometry".repeat(8193);
    assert_eq!(e.evaluate(SCOPE, &name), Value::Error(Error::Limit));
    assert_eq!(e.take_fallback_warnings(), (vec![], 0));
}
