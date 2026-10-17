//! Declared string columns retain spelling even when U is absent.
use misterdiagram_model::{Cell, Document, Row, Section, Sheet};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};

fn cell(name: &str, value: &str) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), ..Default::default() }
}
fn document(section: &str, column: &str, value: &str) -> Document {
    Document {
        document_sheet: Sheet {
            sections: vec![Section {
                name: section.into(),
                rows: vec![Row { name: Some("Example".into()), index: Some(1), cells: vec![cell(column, value)], ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    }
}
#[test]
fn every_declared_string_column_preserves_literal_spelling() {
    for (section, columns) in [
        ("Property", &["Label", "Prompt", "Format", "SortKey"][..]),
        ("Layer", &["Name", "NameUniv", "Password"][..]),
        ("User", &["Prompt"][..]),
        ("Controls", &["Prompt"][..]),
        ("Connection", &["Prompt"][..]),
        ("Paragraph", &["BulletStr"][..]),
        ("Hyperlink", &["Description", "Address", "SubAddress", "ExtraInfo", "Frame"][..]),
        ("Actions", &["Menu", "Prompt", "TagName", "ButtonFace", "SortKey"][..]),
        ("ActionTag", &["TagName", "Description", "ButtonFace"][..]),
        ("Reviewer", &["Name", "Initials"][..]),
        ("Annotation", &["Comment"][..]),
    ] {
        for column in columns {
            for text in ["007", "1e3", "0.00", "01", "#ff0000", "Themed"] {
                let doc = document(section, column, text);
                let mut e = Evaluator::new(&doc).unwrap();
                let name = e.named_cells(Scope::Document)[0].name.clone();
                assert_eq!(e.evaluate(Scope::Document, &name), Value::Text(text.into()), "{section}.{column}: {text}");
            }
        }
    }
    for name in ["Comment", "Copyright", "HelpTopic", "ShapeKeywords", "LayerMember"] {
        let doc = Document { document_sheet: Sheet { cells: vec![cell(name, "007")], ..Default::default() }, ..Default::default() };
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(Scope::Document, name), Value::Text("007".into()), "{name}");
    }
}
#[test]
fn numeric_columns_and_property_values_keep_their_types() {
    for (section, column) in [("Property", "Type"), ("Field", "Format"), ("Hyperlink", "NewWindow"), ("Actions", "Checked"), ("Layer", "Color")] {
        let doc = document(section, column, "007");
        let mut e = Evaluator::new(&doc).unwrap();
        let name = e.named_cells(Scope::Document)[0].name.clone();
        assert_eq!(e.evaluate(Scope::Document, &name), Value::Number(7.), "{section}.{column}");
    }
    for (kind, unit, expected) in [(0, Some("STR"), Value::Text("007".into())), (2, None, Value::Number(7.))] {
        let mut doc = document("Property", "Value", "007");
        let row = &mut doc.document_sheet.sections[0].rows[0];
        row.cells[0].units = unit.map(str::to_owned);
        row.cells.push(cell("Type", &kind.to_string()));
        let mut e = Evaluator::new(&doc).unwrap();
        assert_eq!(e.evaluate(Scope::Document, "Prop.Example"), expected);
    }
}
#[test]
fn string_formulas_still_evaluate_and_errors_and_size_limits_survive() {
    let mut doc = document("Property", "Label", "stale");
    doc.document_sheet.sections[0].rows[0].cells[0].formula = Some("\"00\"&\"7\"".into());
    let mut e = Evaluator::new(&doc).unwrap();
    assert_eq!(e.evaluate(Scope::Document, "Prop.Example.Label"), Value::Text("007".into()));
    for (text, error) in [("x".repeat(65_537), None), ("007".into(), Some("#VALUE!"))] {
        let mut doc = document("Property", "Label", &text);
        doc.document_sheet.sections[0].rows[0].cells[0].error = error.map(str::to_owned);
        let mut e = Evaluator::new(&doc).unwrap();
        assert!(matches!(e.evaluate(Scope::Document, "Prop.Example.Label"), Value::Error(_)));
        assert!(matches!(e.resolve(Scope::Document, "Prop.Example.Label"), Value::Error(_)));
    }
}

#[test]
fn inherited_and_aliased_string_cells_and_evalcell_keep_text() {
    use misterdiagram_model::{Master, Page, Shape};
    let mut master_sheet = document("Property", "Label", "007").document_sheet;
    master_sheet.cells.push(Cell {
        name: "User.Read".into(),
        value: Some("stale".into()),
        formula: Some("EVALCELL(Prop.Example.Label)".into()),
        ..Default::default()
    });
    let mut layer_sheet = document("Layer", "NameUniv", "0042").document_sheet;
    layer_sheet.sections[0].rows[0].index = Some(0);
    let doc = Document {
        masters: vec![Master { id: 5, shapes: vec![Shape { id: 1, sheet: master_sheet, ..Default::default() }], ..Default::default() }],
        pages: vec![Page { id: 1, sheet: layer_sheet, shapes: vec![Shape { id: 2, master: Some(5), ..Default::default() }], ..Default::default() }],
        ..Default::default()
    };
    let mut e = Evaluator::new(&doc).unwrap();
    let scope = Scope::PageShape { page: 1, shape: 2 };
    assert_eq!(e.evaluate(scope, "Prop.Example.Label"), Value::Text("007".into()));
    assert_eq!(e.evaluate(scope, "User.Read"), Value::Text("007".into()));
    for name in ["Layers.NameUniv", "Layers.NameUniv[1]"] {
        assert_eq!(e.evaluate(Scope::Page(1), name), Value::Text("0042".into()));
    }
}

#[test]
fn saved_inherited_string_cache_keeps_its_distinct_spelling() {
    use misterdiagram_model::{Row, Section, Shape};
    let mut doc = misterdiagram_model::skeleton::document(false);
    fn label(value: &str, formula: &str) -> Section {
        Section {
            name: "Property".into(),
            rows: vec![Row {
                name: Some("Test".into()),
                cells: vec![Cell { name: "Label".into(), value: Some(value.into()), formula: Some(formula.into()), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        }
    }
    doc.masters[0].shapes[0].sheet.sections = vec![label("01", "IF(Width>1,\"1\",\"01\")")];
    let mut shape = Shape { id: 1, master: Some(2), ..Default::default() };
    shape.sheet.cells.push(Cell { name: "Width".into(), value: Some("2".into()), ..Default::default() });
    shape.sheet.sections.push(label("1", "Inh"));
    doc.pages[0].shapes.push(shape);
    let saved = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&doc).unwrap().bytes).unwrap();
    let mut evaluator = Evaluator::new(&saved).unwrap();
    let scope = Scope::PageShape { page: 0, shape: 1 };
    let locals = evaluator.source_named_cells(scope).unwrap();
    let cache = locals.iter().find(|n| n.name == "Prop.Test.Label").expect("distinct inherited string cache must survive save");
    assert_eq!(cache.cell.value.as_deref(), Some("1"));
    assert_eq!(cache.cell.formula.as_deref(), Some("Inh"));
    assert_eq!(evaluator.evaluate(scope, "Prop.Test.Label"), Value::Text("1".into()));
}
