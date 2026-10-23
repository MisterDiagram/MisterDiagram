//! Contributor-original instances from the Brief 31c native elision witness.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
use misterdiagram_model::{Cell, Shape, skeleton};
use misterdiagram_vsdx::{import, write};

#[test]
fn a_master_style_literal_does_not_hide_the_instance_style_override() {
    use misterdiagram_model::Style;
    let mut doc = skeleton::document(false);
    let mut source = skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap();
    source.sheet.cells.retain(|c| c.name != "LinePattern");
    source.line_style = Some(100);
    for (id, pattern) in [(100, "1"), (101, "2")] {
        let mut style = Style { id, ..Default::default() };
        style.sheet.cells.push(Cell { name: "LinePattern".into(), value: Some(pattern.into()), ..Default::default() });
        doc.styles.push(style);
    }
    let mut shape = Shape { id: 1, master: Some(2), line_style: Some(101), shape_type: Some("Shape".into()), ..Default::default() };
    shape.sheet.cells.push(Cell { name: "LinePattern".into(), value: Some("1".into()), ..Default::default() });
    doc.masters[0].shapes = vec![source].into();
    doc.pages[0].shapes.push(shape);
    let saved = import(&write(&doc).unwrap().bytes).unwrap();
    assert_eq!(saved.pages[0].shapes[0].sheet.cell("LinePattern").and_then(|c| c.value.as_deref()), Some("1"));
}

#[test]
fn authored_equal_literals_follow_native_elision_without_losing_distinct_overrides() {
    for line in [false, true] {
        for converted in [false, true] {
            let mut doc = skeleton::document(false);
            let mut source = skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap();
            // Include all root literal classes observed in the native witness.
            for name in ["BeginX", "BeginY", "EndX", "EndY", "LayerMember"] {
                if line || name == "LayerMember" {
                    source.sheet.cells.push(Cell { name: name.into(), value: Some("0".into()), ..Default::default() });
                }
            }
            let mut shape = Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
            shape.sheet.cells = source.sheet.cells.iter().filter(|c| c.formula.is_none()).cloned().collect();
            let expected_elisions: Vec<_> = shape.sheet.cells.iter().filter(|c| c.name != "LayerMember").map(|c| c.name.clone()).collect();
            if converted {
                shape.sheet.cells.push(Cell { name: "DropOnPageScale".into(), value: Some("0.1".into()), ..Default::default() });
            }
            doc.masters[0].shapes = vec![source].into();
            doc.pages[0].shapes.push(shape);
            let saved = import(&write(&doc).unwrap().bytes).unwrap();
            let local = &saved.pages[0].shapes[0].sheet;
            for name in expected_elisions {
                assert!(local.cell(&name).is_none(), "redundant literal {name}, line={line}, converted={converted}");
            }
            assert_eq!(local.cell("LayerMember").unwrap().value.as_deref(), Some("0"));
            assert_eq!(local.cell("DropOnPageScale").is_some(), converted);
        }
    }
}

#[test]
fn unit_formula_error_and_extension_overrides_are_not_equal_literals() {
    let mut doc = skeleton::document(false);
    let mut source = skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap();
    for name in ["UnitCase", "FormulaCase", "ErrorCase", "ExtensionCase", "DifferentCase"] {
        source.sheet.cells.push(Cell { name: name.into(), value: Some("4".into()), ..Default::default() });
    }
    let mut shape = Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
    shape.sheet.cells = source.sheet.cells.iter().filter(|c| c.name.ends_with("Case")).cloned().collect();
    for c in &mut shape.sheet.cells {
        match c.name.as_str() {
            "UnitCase" => c.units = Some("IN".into()),
            "FormulaCase" => c.formula = Some("2+2".into()),
            "ErrorCase" => c.error = Some("#VALUE!".into()),
            "ExtensionCase" => {
                c.attributes.insert("Custom".into(), "kept".into());
            }
            "DifferentCase" => c.value = Some("5".into()),
            _ => {}
        }
    }
    doc.masters[0].shapes = vec![source].into();
    doc.pages[0].shapes.push(shape);
    let saved = import(&write(&doc).unwrap().bytes).unwrap();
    let local = &saved.pages[0].shapes[0].sheet;
    for name in ["UnitCase", "FormulaCase", "ErrorCase", "ExtensionCase", "DifferentCase"] {
        assert!(local.cell(name).is_some(), "distinct override {name}");
    }
}

#[test]
fn untouched_import_keeps_its_redundant_source_literal() {
    use std::io::{Cursor, Read, Write};
    let mut doc = skeleton::document(false);
    doc.masters[0].shapes = vec![skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap()].into();
    let mut shape = Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
    shape.sheet.cells.push(Cell { name: "PinX".into(), value: Some("3".into()), ..Default::default() });
    doc.pages[0].shapes.push(shape);
    let written = write(&doc).unwrap();
    let mut input = zip::ZipArchive::new(Cursor::new(&written.bytes)).unwrap();
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for i in 0..input.len() {
        let mut part = input.by_index(i).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name() == "visio/pages/page1.xml" {
            let xml = String::from_utf8(bytes).unwrap();
            let changed = xml.replace("N=\"PinX\" V=\"3\"", "N=\"PinX\" V=\"5\"");
            assert_ne!(xml, changed);
            bytes = changed.into_bytes();
        }
        output.start_file(part.name(), zip::write::SimpleFileOptions::default()).unwrap();
        output.write_all(&bytes).unwrap();
    }
    let imported = import(&output.finish().unwrap().into_inner()).unwrap();
    let saved = import(&write(&imported).unwrap().bytes).unwrap();
    assert_eq!(saved.pages[0].shapes[0].sheet.cell("PinX").unwrap().value.as_deref(), Some("5"));
    assert_eq!(imported.parts["visio/pages/page1.xml"].bytes, saved.parts["visio/pages/page1.xml"].bytes);
}
