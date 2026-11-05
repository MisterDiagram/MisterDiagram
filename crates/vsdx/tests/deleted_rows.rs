use misterdiagram_model::{Cell, Document, Master, Page, Row, Section, Shape, Sheet, Style};
use misterdiagram_sheet::{Error, Value};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope, cell_name};

fn value(name: &str, text: &str) -> Cell {
    Cell { name: name.into(), value: Some(text.into()), ..Default::default() }
}
fn section(name: &str, rows: Vec<Row>) -> Section {
    Section { name: name.into(), rows, ..Default::default() }
}
fn row(index: u32, columns: &[&str]) -> Row {
    Row { index: Some(index), cells: columns.iter().map(|name| value(name, "7")).collect(), ..Default::default() }
}
fn tombstone(index: u32) -> Row {
    Row { index: Some(index), deleted: true, ..Default::default() }
}
fn inherited(source: Section, local: Section) -> Document {
    Document {
        masters: vec![Master {
            id: 1,
            shapes: vec![Shape { id: 1, sheet: Sheet { sections: vec![source], ..Default::default() }, ..Default::default() }].into(),
            ..Default::default()
        }],
        pages: vec![Page {
            id: 0,
            shapes: vec![Shape { id: 1, master: Some(1), sheet: Sheet { sections: vec![local], ..Default::default() }, ..Default::default() }].into(),
            ..Default::default()
        }],
        ..Default::default()
    }
}
const SHAPE: Scope = Scope::PageShape { page: 0, shape: 1 };

/// Row identity is independent of the spelling of its cells.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
#[test]
fn deleted_rows_hide_all_column_families_and_keep_neighbors() {
    for (name, columns) in [
        ("Geometry", vec!["X", "Y"]),
        ("FillGradient", vec!["GradientStopColor", "GradientStopColorTrans", "GradientStopPosition"]),
        ("LineGradient", vec!["GradientStopColor", "GradientStopColorTrans", "GradientStopPosition"]),
        ("Character", vec!["Size", "Style", "Q1"]),
        ("Paragraph", vec!["HorzAlign", "IndFirst"]),
        ("Field", vec!["Value", "Format"]),
        ("Layer", vec!["Name", "Visible"]),
        ("Connection", vec!["X", "Y", "DirX", "DirY", "Type", "AutoGen", "A", "B", "C", "D"]),
        ("Scratch", vec!["X", "A"]),
        ("Tabs", vec!["Position", "Alignment"]),
        ("User", vec!["Value", "Prompt"]),
        ("Property", vec!["Value", "Label"]),
        ("Control", vec!["X", "Y"]),
        ("Action", vec!["Menu", "Action"]),
        ("UnknownRows", vec!["Value"]),
    ] {
        for deleted in [0, 1, 9] {
            let source = section(name, [0, 1, 9].map(|index| row(index, &columns)).to_vec());
            let removed = source.rows.iter().find(|r| r.index == Some(deleted)).unwrap();
            let names: Vec<_> = removed.cells.iter().map(|c| cell_name(&source, removed, 0, c).unwrap()).collect();
            let doc = inherited(source.clone(), section(name, vec![tombstone(deleted)]));
            for cached in [false, true] {
                let mut e = Evaluator::new(&doc).unwrap();
                if cached {
                    e = e.with_cached_fallback();
                }
                for cell in &names {
                    assert!(e.cell(SHAPE, cell).is_none(), "{name} IX{deleted} {cell}");
                    assert_eq!(e.evaluate(SHAPE, cell), Value::Error(Error::Ref), "{name} IX{deleted} {cell} cached={cached}");
                    assert!(!e.resolved_named_cells(SHAPE).iter().any(|n| n.name == *cell));
                }
                let (warnings, _) = e.take_fallback_warnings();
                assert!(warnings.is_empty(), "{name} IX{deleted}: {warnings:?}");
                for live in source.rows.iter().filter(|r| r.index != Some(deleted)) {
                    for cell in &live.cells {
                        let name = cell_name(&source, live, 0, cell).unwrap();
                        assert!(e.cell(SHAPE, &name).is_some(), "neighbor {name}");
                    }
                }
            }
        }
    }
}

/// An IX tombstone also addresses a named provider row with that index.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
#[test]
fn index_tombstone_removes_named_provider() {
    let mut provider = row(7, &["Value", "Prompt"]);
    provider.name = Some("NamedStop".into());
    let doc = inherited(section("User", vec![provider]), section("User", vec![tombstone(7)]));
    let mut e = Evaluator::new(&doc).unwrap().with_cached_fallback();
    for name in ["User.NamedStop", "User.NamedStop.Prompt"] {
        assert!(e.cell(SHAPE, name).is_none());
        assert_eq!(e.evaluate(SHAPE, name), Value::Error(Error::Ref));
    }
}

/// Selected style tombstones replace older style providers, preserving explicit rows.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
#[test]
fn selected_style_deletion_replaces_master_style_provider() {
    for (section_name, column, cell) in
        [("FillGradient", "GradientStopPosition", "FillGradientStops.GradientStopPosition[2]"), ("Character", "Size", "Char.Size[2]")]
    {
        for explicit in ["none", "master", "shape", "disabled"] {
            let inherited_row = section(section_name, vec![row(1, &[column])]);
            let mut doc = Document {
                styles: vec![
                    Style { id: 1, sheet: Sheet { sections: vec![inherited_row.clone()], ..Default::default() }, ..Default::default() },
                    Style {
                        id: 2,
                        fill_style: Some(1),
                        text_style: Some(1),
                        sheet: Sheet { sections: vec![section(section_name, vec![tombstone(1)])], ..Default::default() },
                        ..Default::default()
                    },
                ],
                masters: vec![Master {
                    id: 1,
                    shapes: vec![Shape { id: 1, fill_style: Some(1), text_style: Some(1), ..Default::default() }].into(),
                    ..Default::default()
                }],
                pages: vec![Page {
                    id: 0,
                    shapes: vec![Shape { id: 1, master: Some(1), fill_style: Some(2), text_style: Some(2), ..Default::default() }].into(),
                    ..Default::default()
                }],
                ..Default::default()
            };
            if explicit == "master" {
                doc.masters[0].shapes[0].sheet.sections.push(inherited_row.clone());
            }
            if explicit == "shape" {
                doc.pages[0].shapes[0].sheet.sections.push(inherited_row);
            }
            if explicit == "disabled" {
                doc.styles[1].sheet.cells.push(value("EnableFillProps", "0"));
                doc.styles[1].sheet.cells.push(value("EnableTextProps", "0"));
            }
            let mut e = Evaluator::new(&doc).unwrap().with_cached_fallback();
            let actual = e.evaluate(SHAPE, cell);
            if explicit == "none" {
                assert_eq!(actual, Value::Error(Error::Ref), "{section_name}");
                assert!(!e.resolved_named_cells(SHAPE).iter().any(|n| n.name == cell));
            } else {
                assert_eq!(actual, Value::Number(7.), "{section_name} {explicit}");
            }
        }
    }
}

/// A removed section stays absent under cached evaluation; a closer live row can return.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
#[test]
fn deleted_section_and_reintroduced_row_keep_the_same_availability() {
    let source = section("FillGradient", vec![row(0, &["GradientStopPosition"]), row(1, &["GradientStopPosition"])]);
    let mut deleted = section("FillGradient", vec![]);
    deleted.deleted = true;
    let mut doc = inherited(source.clone(), deleted);
    let mut e = Evaluator::new(&doc).unwrap().with_cached_fallback();
    assert!(e.section_row_indices(SHAPE, "FillGradient", 0).is_empty());
    for name in ["FillGradientStops.GradientStopPosition", "FillGradientStops.GradientStopPosition[2]"] {
        assert!(e.cell(SHAPE, name).is_none());
        assert_eq!(e.evaluate(SHAPE, name), Value::Error(Error::Ref));
    }
    assert!(e.take_fallback_warnings().0.is_empty());
    doc.masters[0].shapes[0].sheet.sections = vec![section("FillGradient", vec![tombstone(0)])];
    doc.pages[0].shapes[0].sheet.sections = vec![section("FillGradient", vec![row(0, &["GradientStopPosition"])])];
    let mut e = Evaluator::new(&doc).unwrap().with_cached_fallback();
    assert_eq!(e.evaluate(SHAPE, "FillGradientStops.GradientStopPosition"), Value::Number(7.));
    assert!(e.cell(SHAPE, "FillGradientStops.GradientStopColorTrans").is_some());
}

/// Hostile row indices are checked before naming or allocation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
#[test]
fn extreme_deleted_row_index_is_bounded() {
    for section_name in ["FillGradient", "Character", "Connection", "UnknownRows"] {
        let doc = inherited(section(section_name, vec![]), section(section_name, vec![tombstone(u32::MAX)]));
        let e = Evaluator::new(&doc).unwrap();
        assert!(!e.section_row_indices(SHAPE, section_name, 0).contains(&u32::MAX));
        let doc = inherited(section(section_name, vec![row(u32::MAX, &["Value"])]), section(section_name, vec![]));
        assert!(Evaluator::new(&doc).is_err());
    }
}

/// Implicit named slots use the merged inventory, not the provider's row position.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
#[test]
fn indexed_deletion_uses_merged_named_row_slots() {
    let mut explicit = row(7, &["Value"]);
    explicit.name = Some("Keep".into());
    let implicit = Row { name: Some("Gone".into()), cells: vec![value("Value", "9")], ..Default::default() };
    let doc = inherited(section("User", vec![explicit, implicit]), section("User", vec![tombstone(0)]));
    let e = Evaluator::new(&doc).unwrap();
    assert!(e.cell(SHAPE, "User.Gone").is_none());
    assert!(e.cell(SHAPE, "User.Keep").is_some());
    assert_eq!(e.section_row_indices(SHAPE, "User", 0), vec![7]);
}

/// Indexed cells cannot regain defaults through an unrelated N attribute.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
#[test]
fn indexed_tombstone_name_does_not_override_its_index() {
    for (section_name, column) in [("FillGradient", "GradientStopPosition"), ("Character", "Size"), ("Geometry", "X")] {
        for index in [0, 1] {
            let source = section(section_name, vec![row(index, &[column])]);
            let name = cell_name(&source, &source.rows[0], 0, &source.rows[0].cells[0]).unwrap();
            let mut deleted = tombstone(index);
            deleted.name = Some("Unrelated".into());
            let doc = inherited(source, section(section_name, vec![deleted]));
            let mut e = Evaluator::new(&doc).unwrap().with_cached_fallback();
            assert!(e.cell(SHAPE, &name).is_none(), "{name}");
            assert_eq!(e.evaluate(SHAPE, &name), Value::Error(Error::Ref));
            assert!(e.take_fallback_warnings().0.is_empty());
        }
    }
}
