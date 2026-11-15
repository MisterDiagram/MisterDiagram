//! Contributor-original stored-error witnesses, built from engine commands.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
use super::*;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::json;
use std::io::{Cursor, Read, Write};

fn run(s: &mut Session, command: &str, params: Value) -> Value {
    s.execute(command, &params).unwrap_or_else(|e| panic!("{command}: {e}"))
}
fn bytes(s: &Session) -> Vec<u8> {
    misterdiagram_vsdx::write::write(s.document()).unwrap().bytes
}
fn patch(s: &Session, change: impl Fn(&str) -> String) -> Session {
    let input = bytes(s);
    let mut source = zip::ZipArchive::new(Cursor::new(input)).unwrap();
    let mut out = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for i in 0..source.len() {
        let mut entry = source.by_index(i).unwrap();
        let mut data = Vec::new();
        entry.read_to_end(&mut data).unwrap();
        if entry.name().unwrap() == "visio/pages/page1.xml" {
            data = change(std::str::from_utf8(&data).unwrap()).into_bytes();
        }
        out.start_file(entry.name().unwrap(), zip::write::SimpleFileOptions::default()).unwrap();
        out.write_all(&data).unwrap();
    }
    let document = misterdiagram_vsdx::import(&out.finish().unwrap().into_inner()).unwrap();
    Session { document: Arc::new(document), ..Session::new() }
}
fn control(glued: bool) -> Session {
    let mut s = Session::new();
    for x in [2., 5.] {
        run(&mut s, "shape.draw.rect", json!({"x":x,"y":5,"width":1.5,"height":1,"text":"box"}));
    }
    if glued {
        run(&mut s, "shape.connection.add", json!({"id":1,"x":"Width*0.5","y":"Height*0.5"}));
        run(&mut s, "shape.draw.line", json!({"x1":4,"y1":3,"x2":6,"y2":3}));
        run(&mut s, "connect.static", json!({"id":3,"end":"begin","target":1,"point":"Connections.X1"}));
        run(&mut s, "connect.dynamic", json!({"id":3,"end":"end","target":2}));
    }
    patch(&s, str::to_owned)
}
fn division(error: bool) -> Session {
    patch(&control(true), |xml| {
        xml.replacen("F=\"Height*0.5\" N=\"Y\"", if error { "E=\"#DIV/0!\" F=\"Height/0\" N=\"Y\"" } else { "F=\"Height/0\" N=\"Y\"" }, 1)
    })
}
fn cell(s: &Session, id: u32, name: &str) -> misterdiagram_model::Cell {
    let e = Evaluator::new(s.document()).unwrap();
    e.cell(Scope::PageShape { page: s.document.pages[0].id, shape: id }, name).unwrap().clone()
}
fn number(s: &Session, id: u32, name: &str) -> f64 {
    Evaluator::new(s.document())
        .unwrap()
        .with_cached_fallback()
        .evaluate(Scope::PageShape { page: s.document.pages[0].id, shape: id }, name)
        .numeric()
        .unwrap()
}
fn counts() -> (usize, usize) {
    publication::COUNTS.with(|c| *c.borrow())
}
fn reset() {
    publication::COUNTS.with(|c| *c.borrow_mut() = (0, 0));
}
fn refused(s: &mut Session, params: Value, message: &str) {
    let before = s.clone();
    let old = bytes(s);
    assert_eq!(s.execute("sheet.set", &params).unwrap_err().to_string(), message);
    assert_eq!(bytes(s), old);
    assert_eq!(s.document, before.document);
    assert_eq!(s.revision, before.revision);
    assert_eq!(s.saved_revision, before.saved_revision);
    assert_eq!(s.next_revision, before.next_revision);
    assert_eq!(s.route_warnings, before.route_warnings);
    assert_eq!(s.selection, before.selection);
    assert_eq!(s.undo.len(), before.undo.len());
    assert_eq!(s.redo.len(), before.redo.len());
    assert_eq!(s.journal.len(), before.journal.len());
}
#[test]
fn stored_errors_control_and_empty_predecessor_do_not_retry() {
    let mut s = control(true);
    reset();
    run(&mut s, "document.inspect", json!({}));
    run(&mut s, "shape.text", json!({"id":2,"text":"control"}));
    assert_eq!(counts(), (1, 0));
    reset();
    refused(&mut s, json!({"id":1,"name":"Connections.Y1","formula":"Height/0"}), "#DIV/0!");
    assert_eq!(counts(), (1, 1));
}
#[test]
fn stored_errors_connection_moves_and_keeps_both_error_attribute_forms() {
    for error in [false, true] {
        let mut s = division(error);
        assert_eq!(cell(&s, 1, "Connections.Y1").formula.as_deref(), Some("Height/0"));
        let original = cell(&s, 1, "Connections.Y1");
        run(&mut s, "document.inspect", json!({}));
        run(&mut s, "shape.inspect", json!({"id":1,"sections":["xform","connection_points"],"cells":["Connections.Y1"]}));
        reset();
        let reply = run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
        assert!(reply.get("warnings").is_none());
        assert_eq!(number(&s, 3, "BeginX"), 3.25);
        assert_eq!(number(&s, 3, "BeginY"), 5.5);
        assert_eq!(cell(&s, 1, "Connections.Y1"), original);
        if !error {
            assert_eq!(counts(), (2, 1));
        }
    }
}
#[test]
fn stored_errors_reject_new_and_rewritten_failing_cells() {
    let mut s = division(false);
    refused(&mut s, json!({"id":1,"name":"Connections.X1","formula":"Width/0"}), "#DIV/0!");
    refused(&mut s, json!({"id":1,"name":"Connections.Y1","formula":"Height/(1-1)"}), "#DIV/0!");
}
#[test]
fn stored_errors_cycle_inspection_edit_and_formulas_survive() {
    let mut s = patch(&control(false), |xml| {
        xml.replace("<Cell N=\"PinX\" V=\"2.75\"/>", "<Cell N=\"PinX\" V=\"2.75\" F=\"Sheet.2!PinX-3\"/>")
            .replace("<Cell N=\"PinX\" V=\"5.75\"/>", "<Cell N=\"PinX\" V=\"5.75\" F=\"Sheet.1!PinX+3\"/>")
    });
    let before = [cell(&s, 1, "PinX"), cell(&s, 2, "PinX")];
    assert!(before.iter().all(|c| c.formula.is_some()));
    run(&mut s, "document.inspect", json!({}));
    run(&mut s, "shape.draw.rect", json!({"x":8,"y":1,"width":1,"height":1}));
    assert_eq!([cell(&s, 1, "PinX"), cell(&s, 2, "PinX")], before);
}
#[test]
fn stored_errors_undo_redo_restore_each_serialized_state() {
    let mut s = division(false);
    let before = bytes(&s);
    run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
    let after = bytes(&s);
    assert_ne!(before, after);
    run(&mut s, "edit.undo", json!({}));
    assert_eq!(bytes(&s), before);
    run(&mut s, "edit.redo", json!({}));
    assert_eq!(bytes(&s), after);
}
#[test]
fn stored_errors_writer_preserves_noncanonical_value_and_error_after_dirty_dependency() {
    let mut s = patch(&division(true), |xml| xml.replace("F=\"Height/0\" N=\"Y\" V=\"0.5\"", "F=\"Height/0\" N=\"Y\" V=\"0.5000\""));
    // Also force a strict failure in the same transaction so numeric E references
    // cannot bypass the restricted writer path under the legacy reference rule.
    s = patch(&s, |xml| xml.replacen("F=\"Width*0.5\" N=\"X\"", "F=\"Width/0\" N=\"X\"", 1));
    let original = cell(&s, 1, "Connections.Y1");
    assert_eq!(original.value.as_deref(), Some("0.5000"));
    run(&mut s, "shape.resize", json!({"id":1,"width":2,"height":2}));
    let actual = cell(&s, 1, "Connections.Y1");
    assert_eq!((actual.formula, actual.value, actual.error, actual.units), (original.formula, original.value, original.error, original.units));
}
#[test]
fn stored_errors_save_and_export_report_unique_fallbacks() {
    let mut s = patch(&division(false), |xml| xml.replacen("F=\"Width*0.5\" N=\"X\"", "F=\"Width/0\" N=\"X\"", 1));
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/brief73-tests");
    std::fs::create_dir_all(&dir).unwrap();
    for (command, extension) in
        [("file.save", "vsdx"), ("file.export", "svg"), ("file.export", "pdf"), ("file.export", "png"), ("file.export", "pptx")]
    {
        let path = dir.join(format!("recovery-{}.{extension}", std::process::id()));
        let reply = run(&mut s, command, json!({"path":path}));
        let warnings = reply["warnings"].as_array().unwrap();
        assert_eq!(reply["warning_count"].as_u64(), Some(warnings.len() as u64));
        let saved: Vec<_> = warnings.iter().filter_map(Value::as_str).filter(|s| s.contains("; used the value saved in the file")).collect();
        assert_eq!(saved.len(), 2, "{reply}");
        assert_eq!(saved.len(), saved.iter().collect::<std::collections::BTreeSet<_>>().len());
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn stored_errors_partial_connection_row_without_direction_allows_edits() {
    let s = division(false);
    let mut s = patch(&s, |xml| {
        let (before, connection) = xml.split_once("<Section N=\"Connection\">").unwrap();
        let connection =
            connection.replacen("</Section>", "<Row IX=\"1\" T=\"Connection\"><Cell N=\"X\" V=\"0.2\"/><Cell N=\"Y\" V=\"0.3\"/></Row></Section>", 1);
        format!("{before}<Section N=\"Connection\">{connection}")
    });
    let errors = publication::stored_errors(&s.document).unwrap();
    assert!(!errors.is_empty());
    let mut candidate = s.document().clone();
    misterdiagram_route::GlueHost::prepare_with_stored_errors(s.document(), &mut candidate, false, Arc::new(errors)).unwrap();
    let original = cell(&s, 1, "Connections.Y1");
    run(&mut s, "shape.text", json!({"id":2,"text":"changed"}));
    run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
    assert_eq!(number(&s, 3, "BeginX"), 3.25);
    assert_eq!(number(&s, 3, "BeginY"), 5.5);
    assert_eq!(cell(&s, 1, "Connections.Y1"), original);
}

// Contributor-original connection rows; no package fixture is required.
// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
fn partial_connection(master: bool, directions: bool, perimeter: bool) -> Session {
    let mut s = control(true);
    let doc = Arc::make_mut(&mut s.document);
    let shape = &mut doc.pages[0].shapes[0];
    let section = shape.sheet.sections.iter_mut().find(|section| section.name == "Connection").unwrap();
    let mut cells = vec![
        misterdiagram_model::Cell { name: "X".into(), value: Some(if perimeter { "0" } else { "0.2" }.into()), ..Default::default() },
        misterdiagram_model::Cell { name: "Y".into(), value: Some("0.3".into()), ..Default::default() },
    ];
    if directions {
        cells.extend(["DirX", "DirY"].map(|name| misterdiagram_model::Cell { name: name.into(), value: Some("0".into()), ..Default::default() }));
    }
    section.rows.push(misterdiagram_model::Row { index: Some(1), row_type: Some("Connection".into()), cells, ..Default::default() });
    if master {
        let mut source = shape.clone();
        source.id = 10;
        doc.masters[0].shapes = vec![source].into();
        shape.master = Some(doc.masters[0].id);
        shape.master_shape = Some(10);
        for row in &mut shape.sheet.sections.iter_mut().find(|section| section.name == "Connection").unwrap().rows {
            for cell in &mut row.cells {
                cell.formula = Some("Inh".into());
            }
        }
    }
    patch(&s, |xml| {
        if !master {
            return xml.to_owned();
        }
        // Import explicit local Inh cells, which the normal authoring writer omits.
        let x = if perimeter { "0" } else { "0.2" };
        let directions = if directions { r#"<Cell N="DirX" F="Inh" V="0"/><Cell N="DirY" F="Inh" V="0"/>"# } else { "" };
        let section = format!(
            r#"<Section N="Connection"><Row IX="0" T="Connection"><Cell N="X" F="Inh" V="0.75"/><Cell N="Y" F="Inh" V="0.5"/></Row><Row IX="1" T="Connection"><Cell N="X" F="Inh" V="{x}"/><Cell N="Y" F="Inh" V="0.3"/>{directions}</Row></Section>"#
        );
        assert!(!xml.split_once("</Shape>").unwrap().0.contains(r#"<Section N="Connection">"#));
        xml.replacen("<Text>", &format!("{section}<Text>"), 1)
    })
}
fn connection_section(s: &Session) -> misterdiagram_model::Section {
    s.document.pages[0].shapes[0].sheet.sections.iter().find(|section| section.name == "Connection").unwrap().clone()
}
#[test]
fn connection_rows_partial_static_glue_moves_and_roundtrips() {
    for master in [false, true] {
        for directions in [false, true] {
            let mut s = partial_connection(master, directions, false);
            let original = connection_section(&s);
            for name in
                if directions { vec!["Connections.Type[2]"] } else { vec!["Connections.DirX[2]", "Connections.DirY[2]", "Connections.Type[2]"] }
            {
                assert_eq!(
                    s.execute("sheet.get", &json!({"id":1,"name":name})).unwrap_err().to_string(),
                    "cell does not exist; add its section and row first"
                );
            }
            let reply = run(&mut s, "shape.draw.line", json!({"x1":6,"y1":3,"x2":7,"y2":3}));
            let id = reply["id"].as_u64().unwrap() as u32;
            run(&mut s, "connect.static", json!({"id":id,"end":"end","target":1,"point":"Connections.X2"}));
            assert!((number(&s, id, "EndX") - 2.2).abs() < 1e-9);
            assert!((number(&s, id, "EndY") - 5.3).abs() < 1e-9);
            run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
            assert_eq!(number(&s, 3, "BeginX"), 3.25);
            assert_eq!(number(&s, 3, "BeginY"), 5.5);
            assert!((number(&s, id, "EndX") - 2.7).abs() < 1e-9);
            assert!((number(&s, id, "EndY") - 5.3).abs() < 1e-9);
            assert_eq!(connection_section(&s), original);
            let reopened = patch(&s, str::to_owned);
            assert_eq!(connection_section(&reopened), original);
        }
    }
}
#[test]
fn connection_rows_partial_dynamic_glue_matches_explicit_zero() {
    for master in [false, true] {
        let partial = partial_connection(master, false, true);
        let mut explicit = partial.clone();
        let row = &mut Arc::make_mut(&mut explicit.document).pages[0].shapes[0]
            .sheet
            .sections
            .iter_mut()
            .find(|section| section.name == "Connection")
            .unwrap()
            .rows[1];
        row.cells.extend(["DirX", "DirY", "Type"].map(|name| misterdiagram_model::Cell {
            name: name.into(),
            value: Some("0".into()),
            ..Default::default()
        }));
        let mut endpoints = vec![];
        for mut s in [partial, explicit] {
            run(&mut s, "connect.dynamic", json!({"id":3,"end":"begin","target":1}));
            let before = [number(&s, 3, "BeginX"), number(&s, 3, "BeginY")];
            run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
            endpoints.push((before, [number(&s, 3, "BeginX"), number(&s, 3, "BeginY")]));
        }
        assert_eq!(endpoints[0], endpoints[1]);
        assert!((endpoints[0].1[0] - 2.5).abs() < 1e-9);
        assert!((endpoints[0].1[1] - 5.3).abs() < 1e-9);
    }
}
#[test]
fn connection_rows_present_direction_error_refuses_and_restores() {
    for name in ["Connections.DirX", "Connections.DirY", "Connections.Type"] {
        let mut s = partial_connection(false, false, false);
        refused(&mut s, json!({"id":1,"name":name,"formula":"Height/0"}), "#DIV/0!");
    }
}
#[test]
fn connection_rows_stored_direction_error_keeps_saved_value() {
    for error in [false, true] {
        let mut s = partial_connection(false, true, true);
        s = patch(&s, |xml| {
            xml.replacen(
                "<Cell N=\"DirX\" V=\"0\"/>",
                if error { "<Cell E=\"#DIV/0!\" F=\"Height/0\" N=\"DirX\" V=\"1\"/>" } else { "<Cell F=\"Height/0\" N=\"DirX\" V=\"1\"/>" },
                1,
            )
        });
        let original = cell(&s, 1, "Connections.DirX");
        assert_eq!(original.formula.as_deref(), Some("Height/0"));
        run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
        assert_eq!(number(&s, 3, "BeginX"), 3.25);
        assert_eq!(cell(&s, 1, "Connections.DirX"), original);
        assert_eq!(cell(&patch(&s, str::to_owned), 1, "Connections.DirX"), original);
    }
}

// Both row forms share the same glue contract; these drawings are original.
// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
fn abcd_connection(master: bool, named: bool, full: bool, perimeter: bool) -> Session {
    let mut s = partial_connection(master, false, perimeter);
    let doc = Arc::make_mut(&mut s.document);
    let convert = |shape: &mut misterdiagram_model::Shape, values: bool| {
        let row = shape
            .sheet
            .sections
            .iter_mut()
            .find(|section| section.name == "Connection")
            .unwrap()
            .rows
            .iter_mut()
            .find(|row| row.index == Some(1))
            .unwrap();
        row.row_type = Some("ConnectionABCD".into());
        row.attributes.insert("T".into(), "ConnectionABCD".into());
        if named {
            row.name = Some("Port".into());
            row.attributes.insert("N".into(), "Port".into());
        }
        if values {
            row.cells.extend([("A", -1), ("B", 0), ("C", 0), ("D", 0)].map(|(name, value)| misterdiagram_model::Cell {
                name: name.into(),
                value: Some(value.to_string()),
                ..Default::default()
            }));
        }
    };
    convert(&mut doc.pages[0].shapes[0], full && !master);
    if master {
        convert(&mut doc.masters[0].shapes[0], full);
    }
    patch(&s, |xml| {
        if !master {
            return xml.to_owned();
        }
        // Import explicit inherited rows after the authoring writer omits them.
        let x = if perimeter { "0" } else { "0.2" };
        let name = if named { r#" N="Port""# } else { "" };
        let section = format!(
            r#"<Section N="Connection"><Row IX="0" T="Connection"><Cell N="X" F="Inh" V="0.75"/><Cell N="Y" F="Inh" V="0.5"/></Row><Row IX="1" T="ConnectionABCD"{name}><Cell N="X" F="Inh" V="{x}"/><Cell N="Y" F="Inh" V="0.3"/></Row></Section>"#
        );
        assert!(!xml.split_once("</Shape>").unwrap().0.contains(r#"<Section N="Connection">"#));
        xml.replacen("<Text>", &format!("{section}<Text>"), 1)
    })
}
#[test]
fn abcd_rows_static_glue_moves_and_roundtrips_without_added_cells() {
    for master in [false, true] {
        for named in [false, true] {
            for full in [false, true] {
                let mut s = abcd_connection(master, named, full, full);
                let original = connection_section(&s);
                for column in ["A", "B", "C"] {
                    let name = if named { format!("Connections.Port.{column}") } else { format!("Connections.{column}[2]") };
                    if full {
                        assert_eq!(number(&s, 1, &name), if column == "A" { -1. } else { 0. });
                    } else {
                        assert_eq!(
                            s.execute("sheet.get", &json!({"id":1,"name":name})).unwrap_err().to_string(),
                            "cell does not exist; add its section and row first"
                        );
                    }
                }
                let reply = run(&mut s, "shape.draw.line", json!({"x1":6,"y1":3,"x2":7,"y2":3}));
                let id = reply["id"].as_u64().unwrap() as u32;
                let point = if named { "Connections.Port.X" } else { "Connections.X2" };
                run(&mut s, "connect.static", json!({"id":id,"end":"end","target":1,"point":point}));
                assert!((number(&s, id, "EndX") - if full { 2.0 } else { 2.2 }).abs() < 1e-9);
                assert!((number(&s, id, "EndY") - 5.3).abs() < 1e-9);
                run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
                assert_eq!(number(&s, 3, "BeginX"), 3.25);
                assert_eq!(number(&s, 3, "BeginY"), 5.5);
                assert!((number(&s, id, "EndX") - if full { 2.5 } else { 2.7 }).abs() < 1e-9);
                assert!((number(&s, id, "EndY") - 5.3).abs() < 1e-9);
                assert_eq!(connection_section(&s), original);
                assert_eq!(connection_section(&patch(&s, str::to_owned)), original);
            }
        }
    }
}
#[test]
fn abcd_rows_dynamic_glue_matches_ordinary_direction_and_kind() {
    for (a, b, c) in [(-1., 0., 0.), (0., 1., 0.), (-1., 0.5, 2.), (-1., 0., 1.)] {
        let mut abcd = abcd_connection(false, false, true, true);
        let row = &mut Arc::make_mut(&mut abcd.document).pages[0].shapes[0]
            .sheet
            .sections
            .iter_mut()
            .find(|section| section.name == "Connection")
            .unwrap()
            .rows[1];
        for (name, value) in [("A", a), ("B", b), ("C", c)] {
            row.cells.iter_mut().find(|cell| cell.name == name).unwrap().value = Some(value.to_string());
        }
        let mut ordinary = abcd.clone();
        let row = &mut Arc::make_mut(&mut ordinary.document).pages[0].shapes[0]
            .sheet
            .sections
            .iter_mut()
            .find(|section| section.name == "Connection")
            .unwrap()
            .rows[1];
        row.row_type = Some("Connection".into());
        row.attributes.insert("T".into(), "Connection".into());
        for cell in &mut row.cells {
            cell.name = match cell.name.as_str() {
                "A" => "DirX",
                "B" => "DirY",
                "C" => "Type",
                "D" => "AutoGen",
                other => other,
            }
            .into();
        }
        let mut endpoints = vec![];
        for mut s in [abcd, ordinary] {
            run(&mut s, "connect.dynamic", json!({"id":3,"end":"begin","target":1}));
            let before = [number(&s, 3, "BeginX"), number(&s, 3, "BeginY")];
            run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
            endpoints.push((before, [number(&s, 3, "BeginX"), number(&s, 3, "BeginY")]));
        }
        assert_eq!(endpoints[0], endpoints[1]);
    }
}
#[test]
fn abcd_rows_present_failing_cells_refuse_and_restore() {
    for named in [false, true] {
        for column in ["A", "B", "C"] {
            let mut s = abcd_connection(false, named, true, true);
            let name = if named { format!("Connections.Port.{column}") } else { format!("Connections.{column}[2]") };
            refused(&mut s, json!({"id":1,"name":name,"formula":"Height/0"}), "#DIV/0!");
        }
    }
}
#[test]
fn abcd_rows_stored_a_error_keeps_saved_value_and_formula() {
    for error in [false, true] {
        let s = abcd_connection(false, false, true, true);
        let mut s = patch(&s, |xml| {
            xml.replacen(
                r#"<Cell N="A" V="-1"/>"#,
                if error { r##"<Cell E="#DIV/0!" F="Height/0" N="A" V="-1"/>"## } else { r#"<Cell F="Height/0" N="A" V="-1"/>"# },
                1,
            )
        });
        let original = cell(&s, 1, "Connections.A[2]");
        assert_eq!(original.formula.as_deref(), Some("Height/0"));
        run(&mut s, "connect.dynamic", json!({"id":3,"end":"begin","target":1}));
        run(&mut s, "shape.move", json!({"id":1,"x":3.25,"y":5.5}));
        assert!((number(&s, 3, "BeginX") - 2.5).abs() < 1e-9);
        assert!((number(&s, 3, "BeginY") - 5.3).abs() < 1e-9);
        assert_eq!(cell(&s, 1, "Connections.A[2]"), original);
        assert_eq!(cell(&patch(&s, str::to_owned), 1, "Connections.A[2]"), original);
    }
}
