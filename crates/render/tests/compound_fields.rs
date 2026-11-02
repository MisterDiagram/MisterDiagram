//! Contributor-original field, evaluated through the normal scene and SVG paths.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/format-cell-text-fields-section
use misterdiagram_model::{Cell, Row, Section, skeleton};
use misterdiagram_render::{scene, svg};
#[test]
fn compound_field_exports_current_value_without_saved_text_or_warnings() {
    let mut doc = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 0., 0., 8., 2., "").unwrap();
    shape.text = Some(misterdiagram_vsdx::parse_xml(b"<Text><fld IX='0'>saved-stale-text</fld></Text>").unwrap());
    shape.sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().formula = Some("72 in".into());
    shape.sheet.sections.push(Section {
        name: "Field".into(),
        rows: vec![Row {
            index: Some(0),
            cells: vec![
                Cell { name: "Value".into(), formula: Some("Width".into()), units: Some("IN".into()), ..Default::default() },
                Cell { name: "Format".into(), value: Some("<,FEET/INCH>0.0 u".into()), units: Some("STR".into()), ..Default::default() },
            ],
            ..Default::default()
        }],
        ..Default::default()
    });
    doc.pages[0].shapes = vec![shape].into();
    for (value, expected) in [("72 in", "6 ft. 0.0 in."), ("73.5 in", "6 ft. 1.5 in.")] {
        doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().formula = Some(value.into());
        for cached_fallback in [false, true] {
            let options = scene::Options { cached_fallback, ..Default::default() };
            let scene = scene::page(&doc, 0, options).unwrap();
            assert!(scene.warnings.is_empty(), "{:?}", scene.warnings);
            let xml = svg::render(&scene, options).unwrap();
            assert!(xml.contains(expected), "{xml}");
            assert!(!xml.contains("saved-stale-text"));
        }
    }
}
