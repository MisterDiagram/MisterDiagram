//! Session command and hostile-input regressions.
//! Hostile-params loop adapted from deckcraft crates/engine/src/tests.rs,
//! be14248577b92059eb2b8c0c9460cceffbc1f01d, MIT OR Apache-2.0.
use misterdiagram_engine::{EngineError, Session};
use serde_json::json;
use std::path::{Path, PathBuf};

fn output(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/engine-tests").join(std::process::id().to_string()).join(name);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
#[test]
fn every_command_survives_empty_and_hostile_params() {
    for params in [
        json!({}),
        json!(null),
        json!([1, 2]),
        json!({"index":99999,"ids":[999999],"id":-1,"text":"x","rect":"bad","color":5,"size":1e308}),
        json!({"path":"\0","metric":"bad","page":18446744073709551615u64,"scale":1e308}),
    ] {
        for spec in misterdiagram_engine::command_specs() {
            let mut s = Session::new();
            let before = s.document().clone();
            let result = s.execute(spec.id, &params);
            assert!(!matches!(result, Err(EngineError::Internal(..))), "{}: {result:?}", spec.id);
            if result.is_err() {
                assert_eq!(s.document(), &before, "{}", spec.id);
            }
        }
    }
}
#[test]
fn registry_schemas_are_json_and_ids_are_unique() {
    let mut ids = std::collections::BTreeSet::new();
    let s = Session::new();
    assert!(s.commands().len() >= 6);
    for command in s.commands() {
        assert!(ids.insert(command.id));
        let schema: serde_json::Value = serde_json::from_str(command.params).unwrap();
        assert_eq!(schema["type"], "object");
    }
}
#[test]
fn failed_edit_and_validation_restore_exact_saved_bytes() {
    let mut s = Session::new();
    let before = misterdiagram_vsdx::write(s.document()).unwrap().bytes;
    let error: misterdiagram_engine::Result<()> = s.edit_document(|doc, selection| {
        doc.pages[0].name = "half edited".into();
        selection.page = usize::MAX;
        Err(EngineError::Operation("deliberate failure".into()))
    });
    assert!(error.is_err());
    assert_eq!(s.selection().page, 0);
    assert!(s.journal().is_empty());
    assert_eq!(misterdiagram_vsdx::write(s.document()).unwrap().bytes, before);
    let error = s.edit_document(|doc, _| {
        doc.pages.push(doc.pages[0].clone());
        Ok(())
    });
    assert!(error.is_err(), "duplicate page ID must fail transaction validation");
    assert_eq!(misterdiagram_vsdx::write(s.document()).unwrap().bytes, before);
}
#[test]
fn panic_restores_real_document() {
    let mut s = Session::new();
    let before = s.document().clone();
    let result: misterdiagram_engine::Result<()> = s.edit_document(|doc, _| {
        doc.pages.clear();
        panic!("seeded command panic");
    });
    assert!(matches!(result, Err(EngineError::Internal(..))));
    assert_eq!(s.document(), &before);
    assert!(s.execute("absent", &json!({})).is_err());
}
#[test]
fn file_formats_and_exports_use_the_session() {
    let dir = output("formats");
    let mut s = Session::new();
    s.execute("file.new", &json!({"metric":true})).unwrap();
    s.edit_document(|doc, _| {
        doc.pages[0].shapes.push(misterdiagram_model::skeleton::rectangle(1, 1., 2., 3., 1., "Original text").unwrap());
        Ok(())
    })
    .unwrap();
    for ext in ["vsdx", "vssx", "vstx"] {
        let file = dir.join(format!("drawing.{ext}"));
        s.execute("file.save", &json!({"path":file})).unwrap();
        let doc = misterdiagram_vsdx::read(file).unwrap();
        assert!(doc.parts[&doc.main_part].content_type.as_ref().unwrap().contains(match ext {
            "vssx" => "stencil",
            "vstx" => "template",
            _ => "drawing",
        }));
    }
    for ext in ["png", "svg", "pdf"] {
        let file = dir.join(format!("drawing.{ext}"));
        s.execute("file.export", &json!({"path":file})).unwrap();
        let bytes = std::fs::read(file).unwrap();
        assert!(bytes.len() > 100);
        match ext {
            "png" => assert!(bytes.starts_with(b"\x89PNG")),
            "pdf" => assert!(bytes.starts_with(b"%PDF")),
            _ => assert!(String::from_utf8(bytes).unwrap().contains("<svg")),
        }
    }
}
#[test]
fn invalid_export_preserves_existing_destination() {
    let dir = output("atomic");
    let path = dir.join("drawing.png");
    std::fs::write(&path, b"existing output").unwrap();
    let mut s = Session::new();
    assert!(s.execute("file.export", &json!({"path":path,"scale":1e308})).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), b"existing output");
    assert!(s.execute("file.open", &json!({"path":path})).is_err());
    assert!(s.journal().is_empty());
}
#[test]
fn probe_session_open_save_reopen_is_strictly_identical() {
    let input = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated/basic/0001-probe.vsdx");
    if !input.exists() {
        eprintln!("skip: private probe absent");
        return;
    }
    let mut s = Session::new();
    s.execute("file.open", &json!({"path":input})).unwrap();
    let before = s.document().clone();
    let path = output("probe").join("probe.vsdx");
    s.execute("file.save", &json!({"path":path})).unwrap();
    let saved = std::fs::read(&path).unwrap();
    s.execute("file.open", &json!({"path":path})).unwrap();
    let a = misterdiagram_theme::Themes::from_document(&before).unwrap();
    let b = misterdiagram_theme::Themes::from_document(s.document()).unwrap();
    let report = misterdiagram_vsdx::roundtrip::compare(&before, s.document(), Some(&a), Some(&b), Some(&misterdiagram_text::Host)).unwrap();
    eprintln!("session probe strict roundtrip: {}/{} ({:.6}%)", report.matched, report.total, report.percent());
    assert_eq!(report.matched, report.total);
    let result: misterdiagram_engine::Result<()> = s.edit_document(|doc, _| {
        doc.pages[0].shapes.clear();
        Err(EngineError::Operation("rollback".into()))
    });
    assert!(result.is_err());
    s.execute("file.save", &json!({"path":path})).unwrap();
    assert_eq!(std::fs::read(path).unwrap(), saved);
}

#[test]
fn undo_redo_restores_document_and_new_edits_clear_redo() {
    let mut s = Session::new();
    let initial = s.document().clone();
    s.edit_document(|doc, _| {
        doc.pages[0].name = "Renamed".into();
        Ok(())
    })
    .unwrap();
    let edited = s.document().clone();
    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(s.document(), &initial);
    s.execute("edit.redo", &json!({})).unwrap();
    assert_eq!(s.document(), &edited);
    s.execute("edit.undo", &json!({})).unwrap();
    s.edit_document(|doc, _| {
        doc.pages[0].name = "Other".into();
        Ok(())
    })
    .unwrap();
    assert_eq!(s.execute("edit.redo", &json!({})).unwrap()["changed"], false);
}

#[cfg(unix)]
#[test]
fn saving_preserves_private_permissions_and_creates_private_files() {
    use std::os::unix::fs::PermissionsExt;
    let dir = output("permissions");
    let path = dir.join("private.vsdx");
    std::fs::write(&path, b"old").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let mut session = Session::new();
    session.execute("file.save", &json!({"path":path})).unwrap();
    assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640)).unwrap();
    session.execute("file.save", &json!({"path":path})).unwrap();
    assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o640);
    let new = dir.join("new.vsdx");
    let _ = std::fs::remove_file(&new);
    session.execute("file.save", &json!({"path":new})).unwrap();
    assert_eq!(std::fs::metadata(&new).unwrap().permissions().mode() & 0o777, 0o600);
}

#[test]
fn command_specific_bad_inputs_preserve_history() {
    let mut s = Session::new();
    for name in ["First", "Second"] {
        s.edit_document(|doc, _| {
            doc.pages[0].name = name.into();
            Ok(())
        })
        .unwrap();
    }
    s.execute("edit.undo", &json!({})).unwrap();
    let before = s.document().clone();
    let journal = s.journal().len();
    let dir = output("bad-inputs");
    let truncated = dir.join("truncated.vsdx");
    std::fs::write(&truncated, b"PK\x03\x04").unwrap();
    for (command, params) in [
        ("file.open", json!({"path": truncated})),
        ("file.save", json!({"path": dir.join("bad.extension")})),
        ("file.save", json!({"path": dir.join("missing/sub/output.vsdx")})),
        ("file.export", json!({"path": dir.join("out.png"), "page": u64::MAX})),
        ("file.export", json!({"path": dir.join("out.png"), "scale": 100.})),
        ("file.new", json!({"metric": 1})),
        ("edit.undo", json!({"unknown":true})),
        ("edit.redo", json!({"unknown":true})),
    ] {
        let error = s.execute(command, &params).unwrap_err();
        assert!(!matches!(error, EngineError::Internal(..)), "{command}: {error}");
        assert_eq!(s.document(), &before);
        assert_eq!(s.journal().len(), journal);
    }
    let result: misterdiagram_engine::Result<()> = s.edit_document(|doc, _| {
        doc.pages.clear();
        Err(EngineError::Operation("failed with history".into()))
    });
    assert!(result.is_err());
    assert_eq!(s.document(), &before);
    assert_eq!(s.execute("edit.redo", &json!({})).unwrap()["changed"], true);
    assert_eq!(s.document().pages[0].name, "Second");
    s.execute("edit.undo", &json!({})).unwrap();
    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(s.document().pages[0].name, "Page-1");
}

#[test]
fn session_recalculates_dependent_cache_and_undo_redo_restore_it() {
    let mut s = Session::new();
    s.edit_document(|doc, _| {
        doc.pages[0].shapes.push(misterdiagram_model::skeleton::rectangle(1, 0., 0., 2., 1., "").unwrap());
        Ok(())
    })
    .unwrap();
    let before = s.document().clone();
    s.edit_document(|doc, _| {
        let width = doc.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap();
        width.value = Some("6".into());
        width.formula = None;
        Ok(())
    })
    .unwrap();
    let after = s.document().clone();
    assert_eq!(after.pages[0].shapes[0].sheet.cells.iter().find(|c| c.name == "LocPinX").unwrap().value.as_deref(), Some("3"));
    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(s.document(), &before);
    s.execute("edit.redo", &json!({})).unwrap();
    assert_eq!(s.document(), &after);
}
