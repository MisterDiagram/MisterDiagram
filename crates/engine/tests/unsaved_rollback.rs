use misterdiagram_engine::Session;
use serde_json::{Value, json};
fn run(s: &mut Session, c: &str, p: Value) -> Value {
    s.execute(c, &p).unwrap()
}
fn out(name: &str) -> std::path::PathBuf {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/review-unsaved").join(std::process::id().to_string()).join(name);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    p
}
#[test]
fn failed_save_and_failed_discard_preserve_dirty_revision() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    let path = out("directory.vsdx");
    std::fs::create_dir_all(&path).unwrap();
    let before = s.document().clone();
    let journal = s.journal().len();
    assert!(s.execute("file.save", &json!({"path":path})).is_err());
    assert!(s.execute("file.new", &json!({"discard":true,"metric":42})).is_err());
    assert_eq!(s.document(), &before);
    assert_eq!(s.journal().len(), journal);
    assert!(s.has_unsaved_changes());
    run(&mut s, "edit.undo", json!({}));
    assert!(!s.has_unsaved_changes());
    run(&mut s, "edit.redo", json!({}));
    assert!(s.has_unsaved_changes());
}
#[test]
fn saved_branch_ids_and_compound_revisions_stay_distinct() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1,"text":"first","fill":{"color":"#123456"}}));
    run(&mut s, "file.save", json!({"path":out("branch.vsdx")}));
    assert!(!s.has_unsaved_changes());
    run(&mut s, "edit.undo", json!({}));
    assert!(s.has_unsaved_changes());
    run(&mut s, "edit.redo", json!({}));
    assert!(!s.has_unsaved_changes());
    run(&mut s, "edit.undo", json!({}));
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1,"text":"second","fill":{"color":"#123456"}}));
    assert!(s.has_unsaved_changes());
    run(&mut s, "file.save", json!({"path":out("branch.vsdx")}));
    s.execute_batch(&json!({"commands":[{"command":"shape.inspect","params":{"id":1,"cells":["Width"]}},{"command":"stencil.open","params":{"builtin":"network"}}]})).unwrap();
    assert!(!s.has_unsaved_changes());
    run(&mut s, "edit.undo", json!({}));
    assert!(!s.has_unsaved_changes());
    run(&mut s, "edit.redo", json!({}));
    assert!(!s.has_unsaved_changes());
}
#[test]
fn failing_batch_that_overwrites_saved_file_must_not_leave_restored_document_clean() {
    let mut s = Session::new();
    let path = out("overwritten.vsdx");
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    run(&mut s, "file.save", json!({"path":path}));
    let original = s.document().clone();
    let result=s.execute_batch(&json!({"commands":[{"command":"shape.text","params":{"id":1,"text":"replacement saved to disk"}},{"command":"file.save","params":{"path":path}},{"command":"shape.text","params":{"id":999,"text":"fail"}}]}));
    assert!(result.is_err());
    assert_eq!(s.document(), &original);
    let mut disk = Session::new();
    run(&mut disk, "file.open", json!({"path":path}));
    assert_ne!(disk.document(), s.document(), "file output survives failed batch, as documented");
    assert!(s.has_unsaved_changes(), "restored original document is no longer saved anywhere; new/open currently discard it without permission");
    for (command, params) in [("file.new", json!({})), ("file.open", json!({"path":path}))] {
        assert!(s.execute(command, &params).unwrap_err().to_string().contains("discard"));
        assert_eq!(s.document(), &original);
    }
    run(&mut s, "edit.undo", json!({}));
    assert!(s.has_unsaved_changes());
    run(&mut s, "edit.redo", json!({}));
    assert!(s.has_unsaved_changes());
    run(&mut s, "file.save", json!({"path":path}));
    assert!(!s.has_unsaved_changes());
}

#[test]
fn final_batch_response_budget_after_save_keeps_rollback_dirty() {
    let mut s = Session::new();
    let path = out("budget-overwrite.vsdx");
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    run(&mut s, "file.save", json!({"path":path}));
    let original = s.document().clone();
    let registry = run(&mut s, "engine.commands", json!({}));
    let escaped_entry = serde_json::to_string(&registry.to_string()).unwrap().len();
    let count = 1024 * 1024 / escaped_entry + 1;
    let mut results = vec![json!({"id":1}), json!({"bytes":10000,"warnings":[],"warning_count":0})];
    results.extend(std::iter::repeat_n(registry, count));
    let payload = json!({"results":results,"references":{}});
    assert!(serde_json::to_vec(&payload).unwrap().len() < 1024 * 1024);
    let envelope = json!({"content":[{"type":"text","text":payload.to_string()}],"isError":false});
    assert!(serde_json::to_vec(&envelope).unwrap().len() > 1024 * 1024);
    let mut commands =
        vec![json!({"command":"shape.text","params":{"id":1,"text":"saved but rollback"}}), json!({"command":"file.save","params":{"path":path}})];
    commands.extend((0..count).map(|_| json!({"command":"engine.commands"})));
    let error = s.execute_batch(&json!({"commands":commands})).unwrap_err().to_string();
    assert!(error.contains("budget") && error.contains("fewer batch"), "{error}");
    assert_eq!(s.document(), &original);
    assert!(s.has_unsaved_changes());
    assert!(s.execute("file.new", &json!({})).unwrap_err().to_string().contains("discard"));
    let mut disk = Session::new();
    run(&mut disk, "file.open", json!({"path":path}));
    assert_ne!(disk.document(), s.document());
}
