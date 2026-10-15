//! Retained-source limits use uncompressed parts, independent of ZIP size.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
use diagramcraft_engine::Session;
use diagramcraft_model::{Part, skeleton};
use serde_json::json;
use std::path::{Path, PathBuf};

const MIB: usize = 1024 * 1024;

fn fixture(directory: &Path, name: &str, media_size: usize) -> PathBuf {
    let mut document = skeleton::document(false);
    // Opaque contributor-original media, never decoded by stencil.open.
    document.parts.insert(
        "visio/media/zeros.bin".into(),
        Part { bytes: vec![0; media_size], content_type: Some("application/octet-stream".into()), ..Default::default() },
    );
    let bytes = diagramcraft_vsdx::write_as(&document, diagramcraft_vsdx::SaveKind::Stencil).unwrap().bytes;
    assert!(bytes.len() < MIB, "generated zero media should compress to a small fixture");
    let path = directory.join(name);
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn retained_stencil_source_accepts_over_32_mib_and_bounds_single_and_combined_sources() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../target/engine-tests/stencil-limits-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let small = fixture(&directory, "over-32.vssx", 32 * MIB + 1);
    let mut session = Session::new();
    let opened = session.execute("stencil.open", &json!({"path":small}));
    assert!(opened.is_ok(), "stencil with media over 32 MiB must open: {opened:?}");
    assert_eq!(opened.unwrap()["stencil"], 1);
    assert_eq!(session.execute("stencil.open", &json!({"path":small})).unwrap()["stencil"], 1, "same source is retained once");
    for index in 2..=3 {
        let path = directory.join(format!("copy-{index}.vssx"));
        std::fs::copy(&small, &path).unwrap();
        assert_eq!(session.execute("stencil.open", &json!({"path":path})).unwrap()["stencil"], index);
    }
    let fourth = directory.join("copy-4.vssx");
    std::fs::copy(&small, &fourth).unwrap();
    let error = session.execute("stencil.open", &json!({"path":fourth})).unwrap_err().to_string();
    assert!(
        error.contains("retained stencil source byte limit: copy-4.vssx needs 32.0 MiB, 96.1 MiB already open, limit 128 MiB"),
        "combined source limit must report requested and retained sizes: {error}"
    );
    assert!(session.execute("stencil.masters", &json!({"stencil":3})).is_ok(), "refusal must retain earlier sources");
    assert!(session.execute("stencil.masters", &json!({"stencil":4})).is_err(), "refused source must not be retained");
    drop(session);

    let large = fixture(&directory, "over-128.vssx", 129 * MIB);
    let mut session = Session::new();
    let error = session.execute("stencil.open", &json!({"path":large})).unwrap_err().to_string();
    assert!(
        error.contains("retained stencil source byte limit: over-128.vssx needs 129.0 MiB, 0.0 MiB already open, limit 128 MiB"),
        "single source limit must report requested and retained sizes: {error}"
    );
    assert_eq!(session.execute("stencil.open", &json!({"path":small})).unwrap()["stencil"], 1, "refusal must leave the session usable");
    std::fs::remove_dir_all(&directory).unwrap();
}
