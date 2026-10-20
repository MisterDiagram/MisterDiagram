//! Panic rollback covers orchestration as well as the strict edit boundary.
use super::*;
use serde_json::json;

fn stencils_equal(a: &[stencils::Stencil], b: &[stencils::Stencil]) {
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b) {
        assert_eq!(a.path, b.path);
        assert!(Arc::ptr_eq(&a.document, &b.document));
    }
}
fn snapshots_equal(a: &VecDeque<Snapshot>, b: &VecDeque<Snapshot>) {
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b) {
        assert!(Arc::ptr_eq(&a.document, &b.document));
        assert_eq!(a.selection, b.selection);
        assert_eq!(a.path, b.path);
        assert_eq!(a.revision, b.revision);
        assert_eq!(a.stencils.is_some(), b.stencils.is_some());
        stencils_equal(a.stencils.as_deref().unwrap_or_default(), b.stencils.as_deref().unwrap_or_default());
    }
}
fn unchanged(a: &Session, b: &Session) {
    assert!(Arc::ptr_eq(&a.document, &b.document));
    assert_eq!(a.document(), b.document());
    stencils_equal(&a.stencils, &b.stencils);
    snapshots_equal(&a.undo, &b.undo);
    snapshots_equal(&a.redo, &b.redo);
    assert_eq!(a.journal.len(), b.journal.len());
    for (a, b) in a.journal.iter().zip(&b.journal) {
        assert_eq!(a.command, b.command);
        assert!(Arc::ptr_eq(&a.params, &b.params));
        assert_eq!(a.bytes, b.bytes);
    }
    assert_eq!(a.memory.mib(), b.memory.mib());
    assert_eq!(a.selection, b.selection);
    assert_eq!(a.path, b.path);
    assert_eq!(a.journal_bytes, b.journal_bytes);
    assert_eq!(a.dirty, b.dirty);
    assert_eq!(a.editing, b.editing);
    assert_eq!(a.revision, b.revision);
    assert_eq!(a.saved_revision, b.saved_revision);
    assert_eq!(a.save_generation, b.save_generation);
    assert_eq!(a.next_revision, b.next_revision);
    for (a, b) in [(&a.read_reservations, &b.read_reservations), (&a.stencil_reservations, &b.stencil_reservations)] {
        assert_eq!(a.len(), b.len());
        assert!(a.iter().zip(b).all(|(a, b)| Arc::ptr_eq(a, b)));
    }
}
fn prepared() -> Session {
    let mut s = Session::new();
    s.execute("stencil.open", &json!({"builtin":"network"})).unwrap();
    s.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":2,"height":1})).unwrap();
    s.execute("shape.move", &json!({"id":1,"x":3,"y":3})).unwrap();
    s.execute("edit.undo", &json!({})).unwrap();
    s.selection.shapes = vec![ShapeId(1)];
    s.read_reservations.push(s.document.clone());
    s.stencil_reservations.push(s.stencils[0].document.clone());
    s
}
fn check(point: &'static str, command: &str, params: Value) {
    for alias in [false, true] {
        let mut s = prepared();
        let before = s.clone();
        guard::PANIC_POINT.with(|p| p.set(Some(point)));
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if alias { s.execute("engine.execute", &json!({"command":command,"params":params})) } else { s.execute(command, &params) }
        }));
        guard::PANIC_POINT.with(|p| p.set(None));
        assert!(caught.is_ok(), "orchestration panic escaped Session::execute");
        let result = caught.unwrap();
        assert!(matches!(result, Err(EngineError::Internal(..))), "{result:?}");
        assert!(result.unwrap_err().to_string().contains(point));
        unchanged(&s, &before);
        s.execute("edit.redo", &json!({})).unwrap();
        assert_eq!(s.execute("sheet.get", &json!({"id":1,"name":"PinX"})).unwrap()["value"], "3");
    }
}
#[test]
fn composition_orchestration_panic_restores_every_session_field() {
    check("composition-orchestration", "shape.draw.rect", json!({"x":2,"y":2,"width":1,"height":1,"text":"Oak"}));
}
#[test]
fn batch_orchestration_panic_restores_every_session_field() {
    check(
        "batch-orchestration",
        "engine.batch",
        json!({"commands":[
            {"command":"stencil.close","params":{"stencil":1}},
            {"command":"page.add","params":{"name":"Transient"}},
            {"command":"shape.delete","params":{"id":1}}
        ]}),
    );
}
#[test]
fn nested_composition_panic_keeps_internal_error_and_batch_state() {
    check(
        "composition-orchestration",
        "engine.batch",
        json!({"commands":[
            {"command":"stencil.close","params":{"stencil":1}},
            {"command":"shape.draw.rect","params":{"x":2,"y":2,"width":1,"height":1,"text":"Oak"}}
        ]}),
    );
}

#[test]
fn edit_panic_restores_revision_and_save_bookkeeping_exactly() {
    let mut s = prepared();
    let before = s.clone();
    let result: Result<()> = s.edit(|candidate| {
        candidate.saved_revision = Some(999);
        candidate.save_generation += 1;
        candidate.next_revision += 1;
        std::panic::resume_unwind(Box::new("edit bookkeeping panic"))
    });
    assert!(matches!(result, Err(EngineError::Internal(..))));
    unchanged(&s, &before);
    s.execute("edit.redo", &json!({})).unwrap();
}
