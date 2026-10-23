use crate::{MemoryLimits, Session};
use serde_json::json;
use std::path::{Path, PathBuf};

fn folder(name: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../target/debug/brief31-path-{}-{name}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(&root).unwrap();
    }
    std::fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap()
}
fn stencil(path: &Path) {
    let bytes = misterdiagram_vsdx::write_as(&crate::network::document().unwrap(), misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes;
    std::fs::write(path, bytes).unwrap();
}
#[test]
fn ordered_bare_name_extension_and_metadata_only_listing() {
    let root = folder("ordered");
    let first = root.join("first");
    let second = root.join("second");
    let missing_folder = root.join("not-created");
    std::fs::create_dir_all(first.join("nested")).unwrap();
    std::fs::create_dir_all(&second).unwrap();
    stencil(&first.join("network.vssx"));
    stencil(&second.join("network.vssx"));
    std::fs::write(first.join("bad.vssx"), b"not a ZIP").unwrap();
    std::fs::write(first.join("nested/hidden.vssx"), b"not a ZIP").unwrap();
    std::fs::write(second.join("ignored.txt"), b"ignored").unwrap();
    let mut s = Session::new();
    s.set_stencil_path(vec![missing_folder.clone(), first.clone(), second.clone()]).unwrap();
    let available = s.execute("stencil.available", &json!({})).unwrap();
    assert_eq!(available["files"].as_array().unwrap().len(), 3);
    assert_eq!(available["files"][0], json!({"name":"bad.vssx","folder":first,"size":9}));
    assert!(s.stencils.is_empty());
    let opened = s.execute("stencil.open", &json!({"path":"network"})).unwrap();
    assert_eq!(opened["path"], json!(first.join("network.vssx")));
    assert_eq!(s.execute("stencil.open", &json!({"path":"network.vssx"})).unwrap()["stencil"], 1);
    assert_eq!(s.execute("stencil.list", &json!({})).unwrap()["stencils"][1]["path"], opened["path"]);
    s.execute("file.new", &json!({})).unwrap();
    assert_eq!(s.execute("stencil.available", &json!({})).unwrap(), available);
    let explicit = s.execute("stencil.open", &json!({"path":second.join("network.vssx")})).unwrap();
    assert_eq!(explicit["path"], json!(second.join("network.vssx")));
    let missing = s.execute("stencil.open", &json!({"path":"absent"})).unwrap_err().to_string();
    assert!(
        missing.contains(missing_folder.to_str().unwrap()) && missing.contains(first.to_str().unwrap()) && missing.contains(second.to_str().unwrap()),
        "{missing}"
    );
    assert!(s.execute("stencil.open", &json!({"path":"hidden"})).is_err());
    assert_eq!(s.stencils.len(), 2);
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn refused_names_never_resolve_through_search_path() {
    let root = folder("refused");
    stencil(&root.join("bad..name.vssx"));
    let mut s = Session::new();
    s.set_stencil_path(vec![root.clone()]).unwrap();
    assert!(s.execute("stencil.open", &json!({"path":"bad..name"})).is_err());
    for name in ["..", ".", "../network", "sub/network", "sub\\network", "name\0bad", "C:network", "C:\\network"] {
        assert!(!super::stencil_path::bare(Path::new(name)), "{name:?}");
    }
    assert!(s.stencils.is_empty());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn path_configuration_is_bounded_and_admission_is_unchanged() {
    let root = folder("admission");
    let file = root.join("network.vssx");
    stencil(&file);
    let mut direct = Session::new();
    direct.execute("stencil.open", &json!({"path":file})).unwrap();
    let mut found = Session::new();
    found.set_stencil_path(vec![root.clone()]).unwrap();
    found.execute("stencil.open", &json!({"path":"network"})).unwrap();
    assert_eq!(direct.retained_xml_nodes(), found.retained_xml_nodes());
    let prior = found.stencil_path.clone();
    for bad in [vec![PathBuf::from("x"); 257], vec![PathBuf::from("x".repeat(32769))], vec![PathBuf::from("bad\0folder")]] {
        assert!(found.set_stencil_path(bad).is_err());
        assert_eq!(found.stencil_path, prior);
    }
    // Both resolution routes enter the same fixed retained-node gate.
    for bare in [false, true] {
        let mut s = Session::with_memory_limits(MemoryLimits::from_mib(8192).unwrap());
        s.set_stencil_path(vec![root.clone()]).unwrap();
        std::sync::Arc::make_mut(&mut s.document).retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES;
        let path = if bare { PathBuf::from("network") } else { file.clone() };
        let e = s.execute("stencil.open", &json!({"path":path})).unwrap_err().to_string();
        assert!(e.contains("retained XML node limit"), "{e}");
        assert!(s.stencils.is_empty());
    }
    std::fs::remove_dir_all(root).unwrap();
}
#[cfg(target_os = "linux")]
#[test]
fn non_utf8_search_folders_are_listed_without_panicking() {
    use std::os::unix::ffi::OsStringExt;
    let root = folder("unicode");
    let path = root.join(std::ffi::OsString::from_vec(b"folder-\xff".to_vec()));
    std::fs::create_dir(&path).unwrap();
    std::fs::write(path.join("unparsed.vssx"), b"bad").unwrap();
    let mut s = Session::new();
    s.set_stencil_path(vec![path]).unwrap();
    assert_eq!(s.execute("stencil.available", &json!({})).unwrap()["files"][0]["name"], "unparsed.vssx");
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn configuration_resident_refusal_restores_the_previous_path() {
    let mut s = Session::new();
    let baseline = s.retention().raw;
    s.set_stencil_path(vec![PathBuf::from("retained-folder")]).unwrap();
    assert!(s.retention().raw >= baseline + "retained-folder".len());
    let prior = s.stencil_path.clone();
    // Model an already-admitted session above a newly imposed tiny ceiling.
    // The setter must restore its configuration when reservation refuses it.
    s.memory = MemoryLimits::from_mib(1).unwrap();
    let error = s.set_stencil_path(vec![PathBuf::from("new-folder")]).unwrap_err().to_string();
    assert!(error.contains("session resident estimate"), "{error}");
    assert_eq!(s.stencil_path, prior);
}
#[test]
fn document_stencil_path_survives_a_host_working_directory_change() {
    const CHILD: &str = "MISTERDIAGRAM_PATH_CWD_TEST";
    if let Some(root) = std::env::var_os(CHILD) {
        let root = PathBuf::from(root);
        let a = root.join("a");
        let b = root.join("b");
        std::env::set_current_dir(&a).unwrap();
        let mut s = Session::new();
        s.execute("file.open", &json!({"path":"drawing.vsdx"})).unwrap();
        std::env::set_current_dir(&b).unwrap();
        assert_eq!(s.execute("stencil.list", &json!({})).unwrap()["stencils"][0]["path"], json!(a.join("drawing.vsdx")));
        s.execute("file.save", &json!({"path":"saved.vsdx"})).unwrap();
        std::env::set_current_dir(&a).unwrap();
        assert_eq!(s.execute("stencil.list", &json!({})).unwrap()["stencils"][0]["path"], json!(b.join("saved.vsdx")));
        return;
    }
    let root = folder("cwd");
    let bytes = misterdiagram_vsdx::write(&crate::files::blank_document(false)).unwrap().bytes;
    for name in ["a", "b"] {
        std::fs::create_dir(root.join(name)).unwrap();
        std::fs::write(root.join(name).join("drawing.vsdx"), &bytes).unwrap();
    }
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "stencil_path_tests::document_stencil_path_survives_a_host_working_directory_change"])
        .env(CHILD, &root)
        .status()
        .unwrap();
    assert!(status.success());
    std::fs::remove_dir_all(root).unwrap();
}
