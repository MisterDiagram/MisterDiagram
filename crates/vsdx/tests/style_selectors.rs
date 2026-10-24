//! Contributor-original style inheritance controls, constructed without package assets.
//! https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/59214758-549c-4f99-a4de-de2bf5fb08d3
//! https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/64509af3-e393-4ac1-9474-949c50d4b06f
use misterdiagram_model::{Cell, Document, Master, Page, Shape, Sheet, Style};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};

fn cell(name: &str, value: &str) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), ..Default::default() }
}

fn selectors(variation: &str, line: &str, text: &str) -> Sheet {
    Sheet { cells: vec![cell("QuickStyleVariation", variation), cell("EnableLineProps", line), cell("EnableTextProps", text)], ..Default::default() }
}

fn document() -> Document {
    Document {
        styles: vec![
            Style { id: 201, sheet: selectors("4", "1", "1"), ..Default::default() },
            Style { id: 202, sheet: selectors("2", "0", "0"), ..Default::default() },
            Style { id: 203, sheet: selectors("8", "1", "1"), ..Default::default() },
            Style { id: 204, line_style: Some(201), fill_style: Some(202), text_style: Some(203), ..Default::default() },
        ],
        pages: vec![Page {
            id: 0,
            shapes: vec![Shape { id: 1, line_style: Some(201), fill_style: Some(202), text_style: Some(203), ..Default::default() }].into(),
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[test]
fn variation_uses_fill_style_when_the_three_style_chains_disagree() {
    let doc = document();
    let mut evaluator = Evaluator::new(&doc).unwrap();
    assert_eq!(evaluator.evaluate(Scope::PageShape { page: 0, shape: 1 }, "QuickStyleVariation"), Value::Number(2.));
}

#[test]
fn inclusion_cells_use_fill_style_without_disabling_other_selected_styles() {
    let mut doc = document();
    doc.styles[0].sheet.cells.push(cell("LinePattern", "3"));
    doc.styles[2].sheet.cells.push(cell("VerticalAlign", "2"));
    let mut evaluator = Evaluator::new(&doc).unwrap();
    let scope = Scope::PageShape { page: 0, shape: 1 };
    for (name, expected) in [("EnableLineProps", 0.), ("EnableTextProps", 0.), ("LinePattern", 3.), ("VerticalAlign", 2.)] {
        assert_eq!(evaluator.evaluate(scope, name), Value::Number(expected), "{name}");
    }
}

#[test]
fn variation_follows_nested_fill_styles_but_omitted_style_flags_stay_enabled() {
    let mut doc = document();
    doc.pages[0].shapes[0].fill_style = Some(204);
    let scope = Scope::PageShape { page: 0, shape: 1 };
    let mut evaluator = Evaluator::new(&doc).unwrap();
    for target in [Scope::Style(204), scope] {
        for (name, expected) in [("QuickStyleVariation", 2.), ("EnableLineProps", 1.), ("EnableTextProps", 1.)] {
            assert_eq!(evaluator.evaluate(target, name), Value::Number(expected), "{target:?} {name}");
        }
    }
    doc.styles[3].sheet = selectors("6", "1", "0");
    let mut evaluator = Evaluator::new(&doc).unwrap();
    for (name, expected) in [("QuickStyleVariation", 6.), ("EnableLineProps", 1.), ("EnableTextProps", 0.)] {
        assert_eq!(evaluator.evaluate(scope, name), Value::Number(expected), "{name}");
    }
}

#[test]
fn selector_style_changes_preserve_master_and_local_overrides() {
    let scope = Scope::PageShape { page: 0, shape: 1 };
    for (name, master_value, local_value) in [("QuickStyleVariation", "12", "6"), ("EnableLineProps", "1", "0"), ("EnableTextProps", "1", "0")] {
        let mut doc = document();
        doc.masters.push(Master { id: 10, shapes: vec![Shape { id: 11, fill_style: Some(201), ..Default::default() }].into(), ..Default::default() });
        doc.pages[0].shapes[0].master = Some(10);
        // Instance style overrides the master's style-origin selector.
        let mut evaluator = Evaluator::new(&doc).unwrap();
        let expected = if name == "QuickStyleVariation" { 2. } else { 0. };
        assert_eq!(evaluator.evaluate(scope, name), Value::Number(expected), "style {name}");
        // A master-local cell remains stronger than the instance's style.
        doc.masters[0].shapes[0].sheet.cells.push(cell(name, master_value));
        let mut evaluator = Evaluator::new(&doc).unwrap();
        assert_eq!(evaluator.evaluate(scope, name), Value::Number(master_value.parse().unwrap()), "master {name}");
        // An explicit local cell wins, while Inh restores the master cell.
        doc.pages[0].shapes[0].sheet.cells.push(cell(name, local_value));
        let mut evaluator = Evaluator::new(&doc).unwrap();
        assert_eq!(evaluator.evaluate(scope, name), Value::Number(local_value.parse().unwrap()), "local {name}");
        doc.pages[0].shapes[0].sheet.cells[0].formula = Some("Inh".into());
        let mut evaluator = Evaluator::new(&doc).unwrap();
        assert_eq!(evaluator.evaluate(scope, name), Value::Number(master_value.parse().unwrap()), "Inh {name}");
    }
}
