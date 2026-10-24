//! Contributor-original controls for the native untyped numeric Cell body import form.
use misterdiagram_model::Document;
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::{
    evaluate::{Evaluator, Scope},
    parse_sheet, parse_xml,
};

#[test]
fn numeric_body_is_literal_and_explicit_value_wins() {
    let xml = parse_xml(
        br#"<DocumentSheet xmlns="http://schemas.microsoft.com/office/visio/2012/main">
<Cell N="User.Body">42</Cell><Cell N="User.Attribute" V="17">42</Cell>
<Cell N="User.Formula" F="2+3" V="5">42</Cell><Cell N="User.EmptyAttribute" V="">42</Cell>
<Cell N="User.Date" U="DATE">42</Cell><Cell N="User.Invalid">not-a-number</Cell>
<Cell N="User.Nested"><Foreign>42</Foreign></Cell><Cell N="User.Nonfinite">NaN</Cell>
</DocumentSheet>"#,
    )
    .unwrap();
    let d = Document { document_sheet: parse_sheet(&xml).unwrap(), ..Default::default() };
    let mut e = Evaluator::new(&d).unwrap();
    for (name, expected) in
        [("Body", 42.), ("Attribute", 17.), ("Formula", 5.), ("EmptyAttribute", 0.), ("Date", 0.), ("Invalid", 0.), ("Nested", 0.), ("Nonfinite", 0.)]
    {
        assert_eq!(e.evaluate(Scope::Document, &format!("User.{name}")), Value::Number(expected), "{name}");
    }
}

#[test]
fn hostile_cell_body_is_bounded_before_copying() {
    let xml = format!(
        "<DocumentSheet xmlns='http://schemas.microsoft.com/office/visio/2012/main'><Cell N='User.Body'>{}</Cell></DocumentSheet>",
        "1".repeat(65537)
    );
    assert!(parse_sheet(&parse_xml(xml.as_bytes()).unwrap()).is_err());
}
