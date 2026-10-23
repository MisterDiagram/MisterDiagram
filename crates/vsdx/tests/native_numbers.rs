//! Contributor-original numeric caches from the Brief31c native witness.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
use misterdiagram_model::{Cell, Row, Section, Shape, skeleton};
use misterdiagram_vsdx::{import, write};

#[test]
fn converted_instance_formula_caches_use_native_decimals_before_and_after_edit() {
    let mut doc = skeleton::document(false);
    doc.masters[0].shapes = vec![skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap()].into();
    let mut shape = Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
    shape.sheet.cells.extend([
        Cell { name: "DropOnPageScale".into(), value: Some("8.000064000511999".into()), ..Default::default() },
        Cell {
            name: "Width".into(),
            value: Some("0".into()),
            formula: Some("80.00064000512".into()),
            units: Some("IN".into()),
            ..Default::default()
        },
    ]);
    shape.sheet.sections.push(Section {
        name: "User".into(),
        rows: vec![Row {
            name: Some("Text".into()),
            cells: vec![Cell { name: "Value".into(), value: Some("00080.0006400051200".into()), units: Some("STR".into()), ..Default::default() }],
            ..Default::default()
        }],
        ..Default::default()
    });
    doc.pages[0].shapes.push(shape);
    let mut saved = import(&write(&doc).unwrap().bytes).unwrap();
    assert_eq!(saved.pages[0].shapes[0].sheet.cell("Width").unwrap().value.as_deref(), Some("80.00064000512001"));
    let width = saved.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap();
    width.formula = Some("8.000064000512".into());
    let edited = import(&write(&saved).unwrap().bytes).unwrap();
    assert_eq!(edited.pages[0].shapes[0].sheet.cell("Width").unwrap().value.as_deref(), Some("8.000064000511999"));
    let row = &edited.pages[0].shapes[0].sheet.sections.iter().find(|s| s.name == "User").unwrap().rows[0];
    assert_eq!(row.cells[0].value.as_deref(), Some("00080.0006400051200"));
}

#[test]
fn converted_descendant_preserves_declared_strings_and_special_caches() {
    let mut doc = skeleton::document(false);
    let mut master = skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap();
    master.shape_type = Some("Group".into());
    master.shapes.push(skeleton::rectangle(7, 0., 0., 2., 1., "").unwrap());
    doc.masters[0].shapes = vec![master].into();
    let mut root = Shape { id: 1, master: Some(2), shape_type: Some("Group".into()), ..Default::default() };
    root.sheet.cells.push(Cell { name: "DropOnPageScale".into(), value: Some("0.1".into()), ..Default::default() });
    let mut child = Shape { id: 2, master_shape: Some(7), shape_type: Some("Shape".into()), ..Default::default() };
    child.sheet.cells.extend([
        Cell { name: "Width".into(), value: Some("80.00064000512".into()), ..Default::default() },
        Cell { name: "Comment".into(), value: Some("00080.0006400051200".into()), ..Default::default() },
        Cell { name: "ThemedCase".into(), value: Some("Themed".into()), ..Default::default() },
        Cell { name: "ErrorCase".into(), value: Some("00080.0006400051200".into()), error: Some("#VALUE!".into()), ..Default::default() },
    ]);
    for (section, column) in [("Property", "Label"), ("Hyperlink", "Address"), ("User", "Prompt"), ("Character", "Font")] {
        child.sheet.sections.push(Section {
            name: section.into(),
            rows: vec![Row {
                name: Some("Text".into()),
                cells: vec![Cell { name: column.into(), value: Some("00080.0006400051200".into()), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        });
    }
    root.shapes.push(child);
    doc.pages[0].shapes.push(root);
    let saved = import(&write(&doc).unwrap().bytes).unwrap();
    let sheet = &saved.pages[0].shapes[0].shapes[0].sheet;
    assert_eq!(sheet.cell("Width").unwrap().value.as_deref(), Some("80.00064000512001"));
    for name in ["Comment", "ErrorCase"] {
        assert_eq!(sheet.cell(name).unwrap().value.as_deref(), Some("00080.0006400051200"), "{name}");
    }
    assert_eq!(sheet.cell("ThemedCase").unwrap().value.as_deref(), Some("Themed"));
    for section in sheet.sections.iter().filter(|s| matches!(s.name.as_str(), "Property" | "Hyperlink" | "User" | "Character")) {
        assert_eq!(section.rows[0].cells[0].value.as_deref(), Some("00080.0006400051200"), "{}", section.name);
    }
}

#[test]
fn invalid_conversion_markers_do_not_select_native_numbers() {
    for marker in [
        Cell { formula: Some("1".into()), ..Default::default() },
        Cell { error: Some("#VALUE!".into()), ..Default::default() },
        Cell { units: Some("STR".into()), ..Default::default() },
    ] {
        let mut doc = skeleton::document(false);
        doc.masters[0].shapes = vec![skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap()].into();
        let mut shape = Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
        shape.sheet.cells.extend([
            Cell { name: "DropOnPageScale".into(), value: Some("0.1".into()), ..marker },
            Cell { name: "Width".into(), value: Some("0".into()), formula: Some("80.00064000512".into()), ..Default::default() },
        ]);
        doc.pages[0].shapes.push(shape);
        let saved = import(&write(&doc).unwrap().bytes).unwrap();
        assert_eq!(saved.pages[0].shapes[0].sheet.cell("Width").unwrap().value, skeleton::number_text(80.00064000512));
    }
}

#[test]
fn untouched_converted_import_preserves_original_literal_spelling() {
    use std::io::{Cursor, Read, Write};
    let mut doc = skeleton::document(false);
    doc.masters[0].shapes = vec![skeleton::rectangle(5, 0., 0., 10., 4., "").unwrap()].into();
    let mut shape = Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
    shape.sheet.cells.extend([
        Cell { name: "DropOnPageScale".into(), value: Some("0.1".into()), ..Default::default() },
        Cell { name: "Width".into(), value: Some("80.00064000512".into()), ..Default::default() },
    ]);
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
            let changed = xml.replace("80.00064000512001", "00080.0006400051200");
            assert_ne!(xml, changed);
            bytes = changed.into_bytes();
        }
        output.start_file(part.name(), zip::write::SimpleFileOptions::default()).unwrap();
        output.write_all(&bytes).unwrap();
    }
    let imported = import(&output.finish().unwrap().into_inner()).unwrap();
    let saved = import(&write(&imported).unwrap().bytes).unwrap();
    assert_eq!(saved.pages[0].shapes[0].sheet.cell("Width").unwrap().value.as_deref(), Some("00080.0006400051200"));
    assert_eq!(imported.parts["visio/pages/page1.xml"].bytes, saved.parts["visio/pages/page1.xml"].bytes);
}
