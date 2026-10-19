//! New/open require explicit consent before losing unsaved document edits.
use misterdiagram_engine::Session;
use serde_json::{Value, json};
fn run(s: &mut Session, command: &str, p: Value) -> Value {
    s.execute(command, &p).unwrap()
}
#[test]
fn new_and_open_refuse_dirty_documents_and_discard_is_explicit() {
    let mut s = Session::new();
    assert!(!s.has_unsaved_changes());
    run(&mut s, "file.new", json!({}));
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/unsaved-tests").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("clean.vsdx");
    run(&mut s, "file.save", json!({"path":path}));
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    assert!(s.has_unsaved_changes());
    let before = s.document().clone();
    let journal = s.journal().len();
    for (command, p) in [
        ("file.new", json!({})),
        ("file.new", json!({"discard":false})),
        ("file.open", json!({"path":path})),
        ("file.open", json!({"path":path,"discard":"true"})),
    ] {
        let error = s.execute(command, &p).unwrap_err().to_string();
        assert!(error.contains("discard"), "{error}");
        assert_eq!(s.document(), &before);
        assert_eq!(s.journal().len(), journal);
        assert!(s.has_unsaved_changes());
    }
    assert!(s.execute("file.open", &json!({"path":dir.join("missing.vsdx"),"discard":true})).is_err());
    assert_eq!(s.document(), &before);
    assert!(s.has_unsaved_changes());
    run(&mut s, "file.open", json!({"path":path,"discard":true}));
    assert!(!s.has_unsaved_changes());
    assert!(s.document().pages[0].shapes.is_empty());
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    run(&mut s, "file.new", json!({"discard":true}));
    assert!(!s.has_unsaved_changes());
    assert!(s.document().pages[0].shapes.is_empty());
}
#[test]
fn save_undo_redo_reads_and_failed_batches_track_the_saved_revision() {
    let mut s = Session::new();
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/unsaved-revision-tests").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("saved.vsdx");
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    run(&mut s, "file.save", json!({"path":path}));
    assert!(!s.has_unsaved_changes());
    run(&mut s, "stencil.open", json!({"builtin":"network"}));
    assert!(!s.has_unsaved_changes());
    run(&mut s, "shape.text", json!({"id":1,"text":"changed"}));
    assert!(s.has_unsaved_changes());
    run(&mut s, "shape.inspect", json!({"id":1,"cells":["Width"]}));
    assert!(s.has_unsaved_changes());
    run(&mut s, "file.export", json!({"path":dir.join("preview.svg")}));
    assert!(s.has_unsaved_changes());
    run(&mut s, "edit.undo", json!({}));
    assert!(!s.has_unsaved_changes());
    run(&mut s, "edit.redo", json!({}));
    assert!(s.has_unsaved_changes());
    run(&mut s, "edit.undo", json!({}));
    let before = s.document().clone();
    let error = s
        .execute_batch(&json!({"commands":[{"command":"shape.text","params":{"id":1,"text":"lost"}},{"command":"file.new"}]}))
        .unwrap_err()
        .to_string();
    assert!(error.contains("index 1") && error.contains("discard"));
    assert_eq!(s.document(), &before);
    assert!(!s.has_unsaved_changes());
    run(&mut s, "file.open", json!({"path":path}));
    assert!(!s.has_unsaved_changes());
}
