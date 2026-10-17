// Review 1b: synthetic round-trip repro, ported with Charles Hood's permission.
// MIT OR Apache-2.0. Source recorded in ATTRIBUTION.md.
use misterdiagram_model::skeleton;
use misterdiagram_vsdx::{import, write};

/// Authored string-typed cells (Shape Data Label/Prompt/Format/SortKey, Layer Name) whose text
/// happens to parse as a number must retain their original spelling on save.
#[test]
fn r1b_authored_numeric_looking_strings_keep_their_spelling() {
    use misterdiagram_model::{Cell, Row, Section};
    let mut d = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "x").unwrap();
    let c = |n: &str, v: &str| Cell { name: n.into(), value: Some(v.into()), ..Default::default() };
    shape.sheet.sections.push(Section {
        name: "Property".into(),
        rows: vec![Row {
            name: Some("Part".into()),
            cells: vec![
                Cell { name: "Value".into(), value: Some("".into()), units: Some("STR".into()), ..Default::default() },
                c("Label", "007"),
                c("Prompt", "1e3"),
                c("Format", "0.00"),
                c("SortKey", "01"),
            ],
            ..Default::default()
        }],
        ..Default::default()
    });
    d.pages[0].shapes.push(shape);
    d.pages[0].sheet.sections.push(Section { name: "Layer".into(), rows: vec![skeleton::layer(0, "0042", true).unwrap()], ..Default::default() });
    let r = import(&write(&d).unwrap().bytes).unwrap();
    let row = &r.pages[0].shapes[0].sheet.sections.iter().find(|s| s.name == "Property").unwrap().rows[0];
    let got: Vec<_> = row.cells.iter().skip(1).map(|c| (c.name.clone(), c.value.clone().unwrap_or_default())).collect();
    let layer =
        r.pages[0].sheet.sections.iter().find(|s| s.name == "Layer").unwrap().rows[0].cells.iter().find(|c| c.name == "Name").unwrap().value.clone();
    eprintln!("{got:?} layer={layer:?}");
    assert_eq!(
        got,
        vec![("Label".into(), "007".into()), ("Prompt".into(), "1e3".into()), ("Format".into(), "0.00".into()), ("SortKey".into(), "01".into())]
    );
    assert_eq!(layer.as_deref(), Some("0042"));
}

#[test]
fn edited_string_literals_and_other_string_sections_keep_spelling() {
    use misterdiagram_model::{Cell, Row, Section};
    let mut d = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "x").unwrap();
    for name in ["Comment", "ShapeKeywords", "Copyright", "HelpTopic"] {
        shape.sheet.cells.push(Cell { name: name.into(), value: Some("old".into()), ..Default::default() });
    }
    for (section, names) in [
        ("Hyperlink", &["Description", "Address", "SubAddress", "ExtraInfo", "Frame"][..]),
        ("Action", &["Menu", "Prompt"][..]),
        ("Property", &["Label", "Prompt", "Format", "SortKey"][..]),
    ] {
        shape.sheet.sections.push(Section {
            name: section.into(),
            rows: vec![Row {
                name: Some("Original".into()),
                cells: names.iter().map(|n| Cell { name: (*n).into(), value: Some("old".into()), ..Default::default() }).collect(),
                ..Default::default()
            }],
            ..Default::default()
        });
    }
    d.pages[0].shapes.push(shape);
    let mut layer = skeleton::layer(0, "old", true).unwrap();
    layer.cells.iter_mut().find(|c| c.name == "NameUniv").unwrap().value = Some("old".into());
    d.pages[0].sheet.sections.push(Section { name: "Layer".into(), rows: vec![layer], ..Default::default() });
    let mut d = import(&write(&d).unwrap().bytes).unwrap();
    let sheet = &mut d.pages[0].shapes[0].sheet;
    for c in sheet.cells.iter_mut().chain(sheet.sections.iter_mut().flat_map(|s| s.rows.iter_mut().flat_map(|r| &mut r.cells))) {
        if c.value.as_deref() == Some("old") {
            c.value = Some("0001e2".into());
        }
    }
    for c in &mut d.pages[0].sheet.sections.iter_mut().find(|s| s.name == "Layer").unwrap().rows[0].cells {
        if matches!(c.name.as_str(), "Name" | "NameUniv") {
            c.value = Some("0042".into());
            c.formula = None;
        }
    }
    let r = import(&write(&d).unwrap().bytes).unwrap();
    for (before, after) in d.pages[0].shapes[0].sheet.cells.iter().zip(&r.pages[0].shapes[0].sheet.cells) {
        if before.value.as_deref() == Some("0001e2") {
            assert_eq!(before.value, after.value, "{}", before.name);
        }
    }
    for section in r.pages[0].shapes[0].sheet.sections.iter().filter(|s| matches!(s.name.as_str(), "Hyperlink" | "Action" | "Property")) {
        for c in &section.rows[0].cells {
            assert_eq!(c.value.as_deref(), Some("0001e2"), "{}.{}", section.name, c.name);
        }
    }
    for c in &r.pages[0].sheet.sections.iter().find(|s| s.name == "Layer").unwrap().rows[0].cells {
        if matches!(c.name.as_str(), "Name" | "NameUniv") {
            assert_eq!(c.value.as_deref(), Some("0042"), "{}", c.name);
        }
    }
}
