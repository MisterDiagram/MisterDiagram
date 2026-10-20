//! Retained-source limits use uncompressed parts, independent of ZIP size.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
use misterdiagram_engine::Session;
use misterdiagram_model::{Part, skeleton};
use serde_json::json;
use std::path::{Path, PathBuf};

const MIB: usize = 1024 * 1024;

fn fixture(directory: &Path, name: &str, media_size: usize) -> PathBuf {
    let mut document = skeleton::document(false);
    // Opaque contributor-original media, never decoded by stencil.open.
    document.parts.insert(
        "visio/media/zeros.bin".into(),
        Part { bytes: vec![0; media_size].into(), content_type: Some("application/octet-stream".into()), ..Default::default() },
    );
    let bytes = misterdiagram_vsdx::write_as(&document, misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes;
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
        error.contains("retained stencil source byte limit: needs 32.0 MiB, 96.1 MiB already retained, limit 128 MiB"),
        "combined source limit must report requested and retained sizes: {error}"
    );
    assert!(session.execute("stencil.masters", &json!({"stencil":3})).is_ok(), "refusal must retain earlier sources");
    assert!(session.execute("stencil.masters", &json!({"stencil":4})).is_err(), "refused source must not be retained");
    drop(session);

    let large = fixture(&directory, "over-128.vssx", 129 * MIB);
    let mut session = Session::new();
    let error = session.execute("stencil.open", &json!({"path":large})).unwrap_err().to_string();
    assert!(
        error.contains("retained stencil source byte limit: needs 129.0 MiB, 0.0 MiB already retained, limit 128 MiB"),
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
    if std::env::var_os("MISTERDIAGRAM_STENCIL_DOM_CHILD").is_none() {
        let output = std::process::Command::new("/usr/bin/time")
            .arg("-l")
            .arg(std::env::current_exe().unwrap())
            .args(["--exact", "six_dense_stencil_sources_stay_bounded", "--nocapture"])
            .env("MISTERDIAGRAM_STENCIL_DOM_CHILD", "1")
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
            bytes: xml.as_bytes().to_vec().into(),
            xml: Some(misterdiagram_vsdx::parse_xml(xml.as_bytes()).unwrap().into()),
            content_type: Some("application/vnd.ms-visio.master+xml".into()),
            ..Default::default()
        },
    );
    document.masters.push(misterdiagram_model::Master {
        id: 1,
        name: "Dense".into(),
        name_u: "Dense".into(),
        part: part.into(),
        ..Default::default()
    });
    let bytes = misterdiagram_vsdx::write_as(&document, misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes;
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

/// Repeated images are media, rather than retained structured source XML.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
#[test]
fn repeated_large_media_has_a_separate_bounded_stencil_budget() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../target/engine-tests/stencil-media-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let mut document = skeleton::document(false);
    // A contributor-original one-pixel, 24-bit BMP, padded like a large payload.
    let mut bmp = vec![0; 4 * MIB];
    bmp[..2].copy_from_slice(b"BM");
    bmp[2..6].copy_from_slice(&(58u32).to_le_bytes());
    bmp[10..14].copy_from_slice(&(54u32).to_le_bytes());
    bmp[14..18].copy_from_slice(&(40u32).to_le_bytes());
    bmp[18..22].copy_from_slice(&(1i32).to_le_bytes());
    bmp[22..26].copy_from_slice(&(1i32).to_le_bytes());
    bmp[26..28].copy_from_slice(&(1u16).to_le_bytes());
    bmp[28..30].copy_from_slice(&(24u16).to_le_bytes());
    for index in 0..40 {
        document.parts.insert(
            format!("visio/media/repeated{index}.bmp"),
            Part { bytes: bmp.clone().into(), content_type: Some("image/bmp".into()), ..Default::default() },
        );
    }
    let bytes = misterdiagram_vsdx::write_as(&document, misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes;
    for index in 25..40 {
        document.parts.remove(&format!("visio/media/repeated{index}.bmp"));
    }
    let medium = directory.join("medium.vsdx");
    std::fs::write(&medium, misterdiagram_vsdx::write(&document).unwrap().bytes).unwrap();
    drop(document);
    let path = directory.join("repeated.vssx");
    std::fs::write(&path, &bytes).unwrap();
    let mut session = Session::with_memory_limits(misterdiagram_engine::MemoryLimits::from_mib(1024).unwrap());
    let opened = session.execute("stencil.open", &json!({"path":path}));
    assert!(opened.is_ok(), "160 MiB repeated media must not consume the 128 MiB source budget: {opened:?}");
    assert_eq!(session.execute("stencil.open", &json!({"path":path})).unwrap()["stencil"], 1);
    let second = directory.join("second.vssx");
    std::fs::write(&second, &bytes).unwrap();
    let error = session.execute("stencil.open", &json!({"path":second})).unwrap_err().to_string();
    assert!(error.contains("retained stencil media byte limit") && error.contains("limit 256 MiB"), "{error}");
    assert!(session.execute("stencil.masters", &json!({"stencil":1})).is_ok());
    assert!(session.execute("stencil.masters", &json!({"stencil":2})).is_err(), "media refusal must be transactional");
    session.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":2,"height":1})).unwrap();
    let before = session.document().clone();
    let error = session.execute("file.open", &json!({"path":second,"discard":true})).unwrap_err().to_string();
    assert!(error.contains("media byte limit"), "drawing admission must charge open stencil media: {error}");
    assert_eq!(session.document(), &before, "failed drawing admission must roll back");
    drop(session);
    let mut session = Session::with_memory_limits(misterdiagram_engine::MemoryLimits::from_mib(1024).unwrap());
    session.execute("file.open", &json!({"path":path})).unwrap();
    let error = session.execute("stencil.open", &json!({"path":second})).unwrap_err().to_string();
    assert!(error.contains("media byte limit"), "both admission orders must enforce the same aggregate: {error}");
    assert_shared_drawing_edit(&mut session);
    drop(session);
    let mut session = Session::with_memory_limits(misterdiagram_engine::MemoryLimits::from_mib(1024).unwrap());
    session.execute("file.open", &json!({"path":medium})).unwrap();
    assert_shared_drawing_edit(&mut session);
    std::fs::remove_dir_all(directory).unwrap();
}

fn assert_shared_drawing_edit(session: &mut Session) {
    let before = session.document().clone();
    session.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":2,"height":1})).unwrap();
    let after = session.document().clone();
    assert_eq!(after.pages[0].shapes.len(), 1);
    for (name, part) in &before.parts {
        if part.content_type.as_deref() == Some("image/bmp") {
            assert!(part.bytes.ptr_eq(&after.parts[name].bytes), "unchanged picture must share its allocation: {name}");
        }
    }
    session.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(session.document(), &before);
    session.execute("edit.redo", &json!({})).unwrap();
    assert_eq!(session.document(), &after);
    let error = session
        .execute(
            "engine.batch",
            &json!({"commands":[
                {"command":"shape.draw.rect","params":{"x":4,"y":1,"width":2,"height":1}},
                {"command":"shape.move","params":{"id":999999,"x":0,"y":0}}
            ]}),
        )
        .unwrap_err()
        .to_string();
    assert!(error.contains("shape not found"), "{error}");
    assert_eq!(session.document(), &after, "last-item failure must restore the shared drawing");
}
