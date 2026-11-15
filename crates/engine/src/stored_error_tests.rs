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
fn stored_errors_partial_connection_row_without_direction_remains_refused() {
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
    assert_eq!(
        misterdiagram_route::GlueHost::prepare_with_stored_errors(s.document(), &mut candidate, false, Arc::new(errors)).err().unwrap(),
        "#REF!"
    );
    let before = bytes(&s);
    assert_eq!(s.execute("shape.text", &json!({"id":2,"text":"changed"})).unwrap_err().to_string(), "#DIV/0!");
    assert_eq!(bytes(&s), before);
}
