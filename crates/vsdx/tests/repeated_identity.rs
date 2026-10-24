//! Original minimal identities, with no third-party XML or saved documents.
use misterdiagram_model::{Cell, Document, Master, Page, Shape, Sheet};
use misterdiagram_sheet::{Error, Value};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
fn cell(name: &str, value: &str, formula: Option<&str>) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), formula: formula.map(str::to_string), ..Default::default() }
}
fn shape(id: u32, cells: Vec<Cell>) -> Shape {
    Shape { id, sheet: Sheet { cells, ..Default::default() }, ..Default::default() }
}
#[test]
fn duplicate_shape_id_retains_first_depth_first_xml_occurrence() {
    let mut group = shape(3, vec![]);
    group.shapes = vec![shape(42, vec![cell("Width", "2", None)])];
    let d = Document {
        pages: vec![Page { id: 1, shapes: vec![group, shape(42, vec![cell("Width", "81", None)])].into(), ..Default::default() }],
        ..Default::default()
    };
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(e.evaluate(Scope::PageShape { page: 1, shape: 42 }, "Width"), Value::Number(2.));
    assert_eq!(d.pages[0].shapes.len(), 2, "source occurrences remain preserved");
}
#[test]
fn repeated_master_ids_have_last_xml_display_mapping_and_explicit_strict_errors() {
    let mut source = shape(5, vec![cell("Width", "10", None), cell("Height", "3", None), cell("PinX", "5", Some("Width/2"))]);
    source.sheet.cells.push(Cell { name: "Comment".into(), value: Some("Themed".into()), units: Some("STR".into()), ..Default::default() });
    source.shapes = vec![shape(6, vec![cell("PinX", "5", Some("Sheet.5!Width/2"))])];
    let mut root = shape(1, vec![cell("Width", "100", None), cell("PinX", "11", Some("Inh"))]);
    root.master = Some(9);
    let mut unique = shape(2, vec![]);
    unique.master_shape = Some(6);
    let mut earlier = shape(20, vec![cell("Width", "40", None), cell("PinX", "13", Some("Inh"))]);
    earlier.master_shape = Some(5);
    let mut later = shape(10, vec![cell("Width", "20", None), cell("PinX", "12", Some("Inh"))]);
    later.master_shape = Some(5);
    root.shapes = vec![unique, earlier, later];
    let d = Document {
        masters: vec![Master { id: 9, shapes: vec![source].into(), ..Default::default() }],
        pages: vec![Page { id: 1, shapes: vec![root].into(), ..Default::default() }],
        ..Default::default()
    };
    let scope = |shape| Scope::PageShape { page: 1, shape };
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(e.display_reference(scope(2), "PinX", "Sheet.5!Width").unwrap(), "Sheet.10!Width");
    for id in [1, 2, 20, 10] {
        assert!(matches!(e.evaluate(scope(id), "PinX"), Value::Error(Error::Unsupported(_))), "shape {id}");
    }
    assert_eq!(e.evaluate(scope(20), "Width"), Value::Number(40.));
    for id in [1, 20, 10] {
        assert_eq!(e.evaluate(scope(id), "Height"), Value::Number(3.));
        assert_eq!(e.evaluate(scope(id), "Comment"), Value::Text("Themed".into()));
    }
    let mut e = Evaluator::new(&d).unwrap().with_cached_fallback();
    for (id, expected) in [(1, 11.), (2, 5.), (20, 13.), (10, 12.)] {
        assert_eq!(e.evaluate(scope(id), "PinX"), Value::Number(expected), "shape {id}");
    }
    let (warnings, count) = e.take_fallback_warnings();
    assert_eq!(count, 4);
    assert!(warnings.iter().all(|w| w.contains("repeated MasterShape") && w.contains("used the value saved")));
    // A local formula is executable even on an otherwise repeated master source.
    e.set_formula(scope(20), "PinX", "Width/2").unwrap();
    assert_eq!(e.evaluate(scope(20), "PinX"), Value::Number(20.));
}

#[test]
fn detached_inheritance_and_missing_coordinates_are_native_literals() {
    let xml = misterdiagram_vsdx::parse_xml(br#"<Shape xmlns="http://schemas.microsoft.com/office/visio/2012/main"><Section N="Geometry" IX="0"><Cell N="NoShow" F="Inh" V="1"/><Row IX="1" T="MoveTo"><Cell N="X" V="2"/></Row><Row IX="2" T="LineTo" Del="1"/></Section><Cell N="Unresolved" F="Inh"/></Shape>"#).unwrap();
    let d = Document {
        pages: vec![Page {
            id: 1,
            shapes: vec![Shape { id: 7, sheet: misterdiagram_vsdx::parse_sheet(&xml).unwrap(), ..Default::default() }].into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let scope = Scope::PageShape { page: 1, shape: 7 };
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(e.evaluate(scope, "Geometry1.NoShow"), Value::Number(1.));
    assert_eq!(e.evaluate(scope, "Geometry1.Y1"), Value::Number(0.));
    for name in ["Unresolved", "Geometry1.Y2", "Geometry1.Y99", "Geometry4294967296.Y1", "Geometry1.Y4294967296"] {
        assert_eq!(e.evaluate(scope, name), Value::Error(Error::Ref), "{name}");
    }
    assert_eq!(e.take_fallback_warnings(), (vec![], 0));
}

#[test]
fn unindexed_field_override_is_ignored_but_indexed_override_takes_effect() {
    let section = |index, value: &str| misterdiagram_model::Section {
        name: "Field".into(),
        rows: vec![misterdiagram_model::Row { index, cells: vec![cell("Value", value, None)], ..Default::default() }],
        ..Default::default()
    };
    let mut source = shape(5, vec![]);
    source.sheet.sections.push(section(Some(0), "27"));
    for (index, expected) in [(None, 27.), (Some(0), 91.)] {
        let mut instance = shape(1, vec![]);
        instance.master = Some(9);
        instance.sheet.sections.push(section(index, "91"));
        let d = Document {
            masters: vec![Master { id: 9, shapes: vec![source.clone()].into(), ..Default::default() }],
            pages: vec![Page { id: 1, shapes: vec![instance].into(), ..Default::default() }],
            ..Default::default()
        };
        let mut e = Evaluator::new(&d).unwrap();
        assert_eq!(e.evaluate(Scope::PageShape { page: 1, shape: 1 }, "Fields.Value"), Value::Number(expected));
    }
}
