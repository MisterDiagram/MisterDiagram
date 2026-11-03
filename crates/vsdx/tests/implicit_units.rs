//! Original document-host tests from private field/brief66-2026-11-03/implicit-native.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/format-function
use misterdiagram_model::{Cell, Row, Section, skeleton};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
fn cell(name: &str, value: &str) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), ..Default::default() }
}
fn document(scale: &str, drawing: &str) -> misterdiagram_model::Document {
    let mut d = skeleton::document(false);
    for (name, value) in [("PageScale", scale), ("DrawingScale", drawing)] {
        d.pages[0].sheet.cells.retain(|c| c.name != name);
        d.pages[0].sheet.cells.push(cell(name, value));
    }
    let mut s = skeleton::rectangle(1, 1., 2., 2., 0.7, "").unwrap();
    for (name, value) in [
        ("TxtWidth", "2"),
        ("Char.Size", "0.16666666666666666"),
        ("LineWeight", "0.010416666666666666"),
        ("Para.IndLeft", "0"),
        ("User.Original", "2.5"),
        ("User.Display", "0"),
    ] {
        s.sheet.cells.retain(|c| c.name != name);
        s.sheet.cells.push(cell(name, value));
    }
    s.sheet.sections.push(Section {
        name: "Property".into(),
        rows: (0..8)
            .map(|n| Row {
                name: Some(format!("Original{n}")),
                cells: vec![cell("Type", &n.to_string()), cell("Value", "2.5")],
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    });
    // Model defaults must not silently provide saved U to this regression.
    for c in &mut s.sheet.cells {
        c.units = None;
    }
    d.pages[0].shapes.push(s);
    d
}
fn display(e: &mut Evaluator<'_>, formula: &str) -> Value {
    let scope = Scope::PageShape { page: 0, shape: 1 };
    e.set_formula(scope, "User.Display", formula).unwrap();
    e.evaluate(scope, "User.Display")
}
#[test]
fn implicit_units_match_all_native_page_modes_and_width_edits() {
    for (scale, drawing) in [("1", "1"), ("0.03937007874015748", "0.03937007874015748"), ("1", "120")] {
        let d = document(scale, drawing);
        let mut e = Evaluator::new(&d).unwrap();
        for edited in [false, true] {
            if edited {
                e.set_formula(Scope::PageShape { page: 0, shape: 1 }, "Width", "8").unwrap();
                e.set_formula(Scope::PageShape { page: 0, shape: 1 }, "TxtWidth", "Width").unwrap();
            }
            let width = if edited { "8.00" } else { "2.00" };
            let loc = if edited { "4.00" } else { "1.00" };
            for (name, value, unit) in [
                ("Width", width, "in."),
                ("Height", "0.70", "in."),
                ("PinX", "2.00", "in."),
                ("PinY", "2.35", "in."),
                ("LocPinX", loc, "in."),
                ("Angle", "0.00", "deg."),
                ("TxtWidth", width, "in."),
                ("Char.Size", "12.00", "pt."),
                ("LineWeight", "0.75", "pt."),
                ("Para.IndLeft", "0.00", "in."),
                ("Geometry1.X2", width, "in."),
                ("User.Original", "2.50", ""),
            ] {
                for upper in [false, true] {
                    let token = if upper { "U" } else { "u" };
                    let unit = if upper { unit.to_ascii_uppercase() } else { unit.into() };
                    assert_eq!(
                        display(&mut e, &format!("FORMAT({name},\"0.00 {token}\")")),
                        Value::Text(format!("{value} {unit}")),
                        "{scale}/{drawing}/{edited}/{name}/{token}"
                    );
                }
            }
            for n in 0..8 {
                for token in ["u", "U"] {
                    assert_eq!(display(&mut e, &format!("FORMAT(Prop.Original{n},\"0.00 {token}\")")), Value::Text("2.50 ".into()));
                }
            }
        }
    }
}
#[test]
fn explicit_units_win_area_formats_and_formatex_source_constants_stay_typed() {
    let mut d = document("1", "1");
    let mut e = Evaluator::new(&d).unwrap();
    for (formula, expected) in [
        ("FORMAT(Width*Height,\"0.00 u^2\")", "1.40 in.^2"),
        ("FORMATEX(Width,\"0.00 u\",\"cm\",\"in\")", "2.00 in."),
        ("FORMATEX(2,\"0.00 u\",\"cm\",\"in\")", "0.79 in."),
        ("FORMATEX(2,\"0.00 u\",\"mm\",\"in\")", "0.08 in."),
    ] {
        assert_eq!(display(&mut e, formula), Value::Text(expected.into()), "{formula}");
    }
    drop(e);
    d.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().units = Some("MM".into());
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(display(&mut e, "FORMAT(Width,\"0.00 u\")"), Value::Text("50.80 mm.".into()));
}
#[test]
fn unknown_and_relative_geometry_cells_do_not_acquire_length_units() {
    let mut d = document("1", "1");
    d.pages[0].shapes[0].sheet.sections.iter_mut().find(|s| s.name == "Geometry").unwrap().rows[1].row_type = Some("RelLineTo".into());
    d.pages[0].shapes[0].sheet.cells.push(cell("User.Width", "2"));
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(display(&mut e, "FORMAT(User.Width,\"0.00 u\")"), Value::Text("2.00 ".into()));
    assert_eq!(display(&mut e, "FORMAT(Geometry1.X2,\"0.00 u\")"), Value::Text("2.00 ".into()));
    for name in ["Geometry0.X2", "Geometry4294967296.X2", "Geometry1.X4294967296"] {
        assert!(display(&mut e, &format!("FORMAT({name},\"0.00 u\")")).error().is_some());
    }
}
#[test]
fn reference_examples_and_cached_numbers_do_not_gain_numeric_conversion() {
    let mut d = document("1", "120");
    let width = d.pages[0].shapes[0].sheet.cells.iter().find(|c| c.name == "Width").unwrap().clone();
    let mut e = Evaluator::new(&d).unwrap();
    let scope = Scope::PageShape { page: 0, shape: 1 };
    assert_eq!(e.evaluate(scope, "Width").numeric().unwrap(), 2.);
    for (f, want) in [
        ("FORMAT(1cm/4,\"0.000 u\")", "0.250 cm."),
        ("FORMAT(1cm/4,\"0.00 U\")", "0.25 CM."),
        ("FORMAT(1cm/4,\"0.0 u\")", "0.3 cm."),
        ("FORMAT(2,\"0.00 u\")", "2.00 "),
    ] {
        assert_eq!(display(&mut e, f), Value::Text(want.into()));
    }
    drop(e);
    assert_eq!(d.pages[0].shapes[0].sheet.cells.iter().find(|c| c.name == "Width").unwrap(), &width);
    d.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().units = Some("UnknownUnit".into());
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(e.evaluate(scope, "Width").numeric().unwrap(), 2.);
    let _bounded = display(&mut e, "FORMAT(Width,\"0.00 u\")");
    e.set_formula(scope, "Width", "Height").unwrap();
    e.set_formula(scope, "Height", "Width").unwrap();
    assert!(display(&mut e, "FORMAT(Width,\"0.00 u\")").error().is_some());
}
#[test]
fn non_shape_cells_with_similar_names_remain_untyped() {
    let mut d = document("1", "1");
    d.document_sheet.cells.extend([cell("Width", "2"), cell("Display", "0")]);
    let mut e = Evaluator::new(&d).unwrap();
    e.set_formula(Scope::Document, "Display", "FORMAT(Width,\"0.00 u\")").unwrap();
    assert_eq!(e.evaluate(Scope::Document, "Display"), Value::Text("2.00 ".into()));
}
#[test]
fn inherited_and_local_explicit_units_precede_implicit_defaults() {
    let mut d = document("1", "1");
    let mut master = skeleton::rectangle(5, 0., 0., 2., 1., "").unwrap();
    master.sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().units = Some("MM".into());
    d.masters[0].shapes = vec![master].into();
    d.pages[0].shapes[0].master = Some(2);
    d.pages[0].shapes[0].sheet.cells.retain(|c| c.name != "Width");
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(display(&mut e, "FORMAT(Width,\"0.00 u\")"), Value::Text("50.80 mm.".into()));
    drop(e);
    d.pages[0].shapes[0].sheet.cells.push(Cell { name: "Width".into(), formula: Some("Inh".into()), units: Some("CM".into()), ..Default::default() });
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(display(&mut e, "FORMAT(Width,\"0.00 u\")"), Value::Text("5.08 cm.".into()));
}
#[test]
fn section_row_aliases_keep_character_and_paragraph_units() {
    let mut d = document("1", "1");
    for (section, column, value) in [("Character", "Size", "0.25"), ("Paragraph", "IndLeft", "0.5")] {
        d.pages[0].shapes[0].sheet.sections.push(Section {
            name: section.into(),
            rows: vec![Row { index: Some(1), cells: vec![cell(column, value)], ..Default::default() }],
            ..Default::default()
        });
    }
    let mut e = Evaluator::new(&d).unwrap();
    assert_eq!(display(&mut e, "FORMAT(Char.Size[2],\"0.00 u\")"), Value::Text("18.00 pt.".into()));
    assert_eq!(display(&mut e, "FORMAT(Para.IndLeft[2],\"0.00 u\")"), Value::Text("0.50 in.".into()));
}
