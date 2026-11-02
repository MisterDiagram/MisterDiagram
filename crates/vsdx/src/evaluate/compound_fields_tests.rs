//! Current field values must supersede saved field text, with no fallback warning.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/format-cell-text-fields-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/value-cell-text-fields-section
use crate::evaluate::{Evaluator, Scope};
use misterdiagram_model::{Cell, Document, Page, Row, Section, Shape};
#[path = "../../../sheet/tests/support/compound_cases.rs"]
mod cases;
fn drawing(picture: &str, unit: &str) -> Document {
    let mut shape = Shape { id: 1, text: Some(crate::parse_xml(b"<Text><fld IX='0'>saved-stale-text</fld></Text>").unwrap()), ..Default::default() };
    shape.sheet.cells.push(Cell { name: "Width".into(), formula: Some("123 in".into()), units: Some("IN".into()), ..Default::default() });
    shape.sheet.sections.push(Section {
        name: "Field".into(),
        rows: vec![Row {
            index: Some(0),
            cells: vec![
                Cell {
                    name: "Value".into(),
                    formula: Some("Width".into()),
                    value: Some("999".into()),
                    units: Some(unit.into()),
                    ..Default::default()
                },
                Cell { name: "Format".into(), value: Some(picture.into()), units: Some("STR".into()), ..Default::default() },
            ],
            ..Default::default()
        }],
        ..Default::default()
    });
    Document { pages: vec![Page { id: 1, shapes: vec![shape].into(), ..Default::default() }], ..Default::default() }
}
#[test]
fn native_compound_fields_recalculate_in_strict_and_cached_modes() {
    let scope = Scope::PageShape { page: 1, shape: 1 };
    let mut failures = Vec::new();
    for c in cases::CASES.iter().filter(|c| c.expected != "#VALUE!") {
        let doc = drawing(c.picture, c.field_unit);
        for fallback in [false, true] {
            let mut e = Evaluator::new(&doc).unwrap();
            if fallback {
                e = e.with_cached_fallback();
            }
            // Populate the cached field and its dependencies before editing Width.
            let before = e.text_string(scope, 0).unwrap();
            assert_ne!(before, "saved-stale-text");
            assert_eq!(e.take_fallback_warnings(), (vec![], 0));
            e.set_formula(scope, "Width", c.value).unwrap();
            let string = e.text_string(scope, 0);
            let body = e.text_body(scope).map(|b| b.paragraphs.iter().map(|p| p.text()).collect::<Vec<_>>().join("\n"));
            if string.as_deref() != Ok(c.expected) || body.as_deref() != Ok(c.expected) {
                failures.push(format!(
                    "{} fallback={fallback} value={} picture={}: string={string:?}, body={body:?}, expected {:?}",
                    c.id, c.value, c.picture, c.expected
                ));
            }
            assert_eq!(e.take_fallback_warnings(), (vec![], 0), "{}", c.id);
        }
    }
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn rejected_compound_fields_retain_explicit_fallback() {
    let scope = Scope::PageShape { page: 1, shape: 1 };
    for c in cases::CASES.iter().filter(|c| c.expected == "#VALUE!") {
        let doc = drawing(c.picture, c.field_unit);
        let mut strict = Evaluator::new(&doc).unwrap();
        strict.set_formula(scope, "Width", c.value).unwrap();
        assert!(strict.text_body(scope).is_err(), "{}", c.id);
        let mut fallback = Evaluator::new(&doc).unwrap().with_cached_fallback();
        fallback.set_formula(scope, "Width", c.value).unwrap();
        assert_eq!(fallback.text_body(scope).unwrap().paragraphs[0].text(), "saved-stale-text", "{}", c.id);
        assert_eq!(fallback.take_fallback_warnings().1, 1, "{}", c.id);
    }
}
