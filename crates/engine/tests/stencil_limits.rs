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

/// Six dense XML sources remain within the existing source-byte stencil budget.
/// The macOS child isolates RSS from other tests and from the test runner.
#[test]
fn six_dense_stencil_sources_stay_bounded() {
    #[cfg(target_os = "macos")]
    if std::env::var_os("DIAGRAMCRAFT_STENCIL_DOM_CHILD").is_none() {
        let output = std::process::Command::new("/usr/bin/time")
            .arg("-l")
            .arg(std::env::current_exe().unwrap())
            .args(["--exact", "six_dense_stencil_sources_stay_bounded", "--nocapture"])
            .env("DIAGRAMCRAFT_STENCIL_DOM_CHILD", "1")
            .output()
            .unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(output.status.success(), "{stderr}");
        let rss: u64 = stderr.lines().find(|l| l.contains("maximum resident set size")).unwrap().split_whitespace().next().unwrap().parse().unwrap();
        eprintln!("six 25000-element stencils peak RSS: {rss} bytes (limit 130 MiB)");
        assert!(rss < 130 * 1024 * 1024, "{stderr}");
        return;
    }
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../target/engine-tests/stencil-dom-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let mut document = skeleton::document(false);
    document.masters.clear();
    let part = "visio/masters/master1.xml";
    let xml = format!(
        "<MasterContents xmlns='http://schemas.microsoft.com/office/visio/2012/main'><Shapes/>{}</MasterContents>",
        "<a b='1'/>".repeat(25_000)
    );
    document.parts.insert(
        part.into(),
        Part {
            bytes: xml.as_bytes().to_vec(),
            xml: Some(diagramcraft_vsdx::parse_xml(xml.as_bytes()).unwrap()),
            content_type: Some("application/vnd.ms-visio.master+xml".into()),
            ..Default::default()
        },
    );
    document.masters.push(diagramcraft_model::Master {
        id: 1,
        name: "Dense".into(),
        name_u: "Dense".into(),
        part: part.into(),
        ..Default::default()
    });
    let bytes = diagramcraft_vsdx::write_as(&document, diagramcraft_vsdx::SaveKind::Stencil).unwrap().bytes;
    drop(document);
    let mut session = Session::new();
    for index in 1..=6 {
        let path = directory.join(format!("dense-{index}.vssx"));
        std::fs::write(&path, &bytes).unwrap();
        let opened = session.execute("stencil.open", &json!({"path":path})).unwrap();
        assert_eq!(opened["stencil"], index);
        assert_eq!(opened["masters"], 1);
    }
    std::fs::remove_dir_all(directory).unwrap();
}
