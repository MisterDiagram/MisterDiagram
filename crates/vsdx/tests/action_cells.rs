//! Passive Action cache behavior from the original M5.10 witness.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/action-cell-actions-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/setf-function
use misterdiagram_model::{Cell, Document, Master, Row, Section, Shape};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
const SETF: &str = "SETF(GETREF(User.Flag),NOT(User.Flag))";
const PLACEHOLDER: &str = "\"SETF_PLACEHOLDER\"";

fn carrier(id: u32, stored: u32, formula: &str) -> Shape {
    let mut shape = misterdiagram_model::skeleton::rectangle(id, 1., 1., 2., 1., "").unwrap();
    shape.sheet.sections.extend([
        Section {
            name: "Actions".into(),
            rows: vec![Row {
                name: Some("Toggle".into()),
                cells: vec![Cell {
                    name: "Action".into(),
                    value: Some(stored.to_string()),
                    formula: Some(formula.into()),
                    units: Some("BOOL".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        Section {
            name: "User".into(),
            rows: vec![Row {
                name: Some("Flag".into()),
                cells: vec![Cell { name: "Value".into(), value: Some("0".into()), ..Default::default() }],
                ..Default::default()
            }],
            ..Default::default()
        },
    ]);
    shape
}
fn input(doc: &Document, patches: usize) -> Document {
    // Patch original package bytes after writing, so new writer behavior cannot
    // silently repair the fixture before the unchanged-source save is exercised.
    let bytes = misterdiagram_vsdx::write(doc).unwrap().bytes;
    let package = misterdiagram_opc::Package::open(&bytes).unwrap();
    let mut output = misterdiagram_opc::PackageOut::default();
    let mut count = 0;
    for (name, bytes) in package.parts() {
        if name == "[Content_Types].xml" {
            continue;
        }
        let data = if name.ends_with(".xml") {
            let text = std::str::from_utf8(bytes).unwrap();
            count += text.matches("&quot;SETF_PLACEHOLDER&quot;").count();
            text.replace("&quot;SETF_PLACEHOLDER&quot;", SETF).into_bytes()
        } else {
            bytes.to_vec()
        };
        output.binary(name, &package.content_type(name).unwrap(), data);
    }
    assert_eq!(count, patches);
    misterdiagram_vsdx::import(&output.finish().unwrap()).unwrap()
}
fn verify_matrix(doc: &Document, width: f64, normalized: bool) {
    let mut evaluator = Evaluator::new(doc).unwrap();
    for master in [false, true] {
        for id in 5..=10 {
            let scope = if master { Scope::MasterShape { master: 42, shape: id } } else { Scope::PageShape { page: 0, shape: id } };
            let dependent = if master { Scope::MasterShape { master: 42, shape: id + 100 } } else { Scope::PageShape { page: 0, shape: id + 100 } };
            let stored = (id - 5) % 2;
            let setf = (7..=8).contains(&id);
            let expected = if setf { 0. } else { f64::from(stored) };
            assert_eq!(
                evaluator.evaluate(scope, "Actions.Toggle.Action").numeric().unwrap(),
                expected,
                "SETF Action must read zero without executing: {scope:?}"
            );
            assert_eq!(evaluator.evaluate(dependent, "Width").numeric().unwrap(), width * expected, "{dependent:?}");
            assert_eq!(evaluator.evaluate(scope, "User.Flag"), Value::Number(0.), "SETF must not mutate its target");
            if normalized {
                let cells = evaluator.source_named_cells(scope).unwrap();
                let cell = cells.iter().find(|cell| cell.name == "Actions.Toggle.Action").unwrap().cell;
                assert_eq!(cell.value.as_deref(), Some(if expected == 0. { "0" } else { "1" }), "SETF Action must save zero: {scope:?}");
                assert_eq!(cell.units.as_deref(), Some("BOOL"));
                if setf {
                    assert_eq!(cell.formula.as_deref(), Some(SETF));
                }
            }
        }
    }
}
#[test]
fn setf_actions_read_and_save_zero_without_side_effects() {
    let mut doc = misterdiagram_model::skeleton::document(false);
    let mut shapes = Vec::new();
    for (kind, formula) in ["NOT(\"Actions.Action[3]\")", PLACEHOLDER, "7"].into_iter().enumerate() {
        for stored in 0..=1 {
            let id = u32::try_from(kind).unwrap() * 2 + stored + 5;
            shapes.push(carrier(id, stored, formula));
            let mut dependent = misterdiagram_model::skeleton::rectangle(id + 100, 4., 1., 2., 1., "").unwrap();
            dependent.sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().formula =
                Some(format!("Sheet.{id}!Width*Sheet.{id}!Actions.Toggle.Action"));
            shapes.push(dependent);
        }
    }
    doc.pages[0].shapes = shapes.clone();
    doc.masters = vec![Master { id: 42, name_u: "OriginalActionCases".into(), shapes, ..Default::default() }];
    let mut doc = input(&doc, 4);
    verify_matrix(&doc, 2., false);
    let saved = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&doc).unwrap().bytes).unwrap();
    verify_matrix(&saved, 2., true);
    for shape in doc.pages[0].shapes.iter_mut().chain(doc.masters[0].shapes.iter_mut()).filter(|s| s.id <= 10) {
        shape.sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().value = Some("3".into());
    }
    verify_matrix(&doc, 3., false);
    let saved = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&doc).unwrap().bytes).unwrap();
    verify_matrix(&saved, 3., true);
}

#[test]
fn inherited_setf_action_ignores_local_stored_cache() {
    let mut doc = misterdiagram_model::skeleton::document(false);
    doc.masters = vec![Master { id: 42, name_u: "OriginalInheritedAction".into(), shapes: vec![carrier(5, 1, PLACEHOLDER)], ..Default::default() }];
    let mut instance = Shape { id: 10, master: Some(42), ..Default::default() };
    instance.sheet.sections.push(Section {
        name: "Actions".into(),
        rows: vec![Row {
            name: Some("Toggle".into()),
            cells: vec![Cell {
                name: "Action".into(),
                value: Some("1".into()),
                formula: Some("Inh".into()),
                units: Some("BOOL".into()),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    });
    doc.pages[0].shapes = vec![instance];
    let doc = input(&doc, 1);
    let scope = Scope::PageShape { page: 0, shape: 10 };
    let mut e = Evaluator::new(&doc).unwrap();
    assert_eq!(e.effective_formula(scope, "Actions.Toggle.Action").as_deref(), Some(SETF));
    assert_eq!(e.evaluate(scope, "Actions.Toggle.Action"), Value::Number(0.));
    assert_eq!(e.evaluate(scope, "User.Flag"), Value::Number(0.));
    let saved = misterdiagram_vsdx::import(&misterdiagram_vsdx::write(&doc).unwrap().bytes).unwrap();
    let mut e = Evaluator::new(&saved).unwrap();
    assert_eq!(e.evaluate(scope, "Actions.Toggle.Action"), Value::Number(0.));
    assert_eq!(e.evaluate(scope, "User.Flag"), Value::Number(0.));
    for scope in [scope, Scope::MasterShape { master: 42, shape: 5 }] {
        for c in e.source_named_cells(scope).unwrap().into_iter().filter(|c| c.name == "Actions.Toggle.Action") {
            assert_eq!(c.cell.value.as_deref(), Some("0"));
        }
    }
}

#[test]
fn passive_action_detection_is_parsed_bounded_and_case_insensitive() {
    for (formula, expected) in [
        ("setf(GETREF(User.Flag),NOT(User.Flag))".to_owned(), 0.),
        ("\"SETF(GETREF(User.Flag),1)\"".to_owned(), 1.),
        ("NOT(\"SETF(User.Flag,1)\")".to_owned(), 1.),
        ("SETF(".to_owned(), 1.),
        (format!("SETF({})", "x".repeat(65_537)), 1.),
    ] {
        let mut doc = misterdiagram_model::skeleton::document(false);
        doc.pages[0].shapes = vec![carrier(1, 1, &formula)];
        let mut e = Evaluator::new(&doc).unwrap();
        let scope = Scope::PageShape { page: 0, shape: 1 };
        assert_eq!(e.evaluate(scope, "Actions.Toggle.Action").numeric().unwrap(), expected);
        assert_eq!(e.evaluate(scope, "User.Flag"), Value::Number(0.));
    }
}
