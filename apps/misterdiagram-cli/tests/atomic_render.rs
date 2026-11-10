//! Original synthetic drawings exercise publication through the public CLI and engine.
use misterdiagram_engine::Session;
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

struct Fixture {
    dir: PathBuf,
    input: PathBuf,
    session: Session,
}
impl Fixture {
    fn new(label: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/atomic-render").join(format!(
            "{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).unwrap();
        let input = dir.join("original.vsdx");
        let mut session = Session::default();
        let shape = session.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":2,"height":1})).unwrap();
        session.execute("master.new_from_shape", &json!({"id":shape["id"],"name":"Original rectangle"})).unwrap();
        session.execute("file.save", &json!({"path":input})).unwrap();
        Self { dir, input, session }
    }
    fn render(&self, output: &Path, masters: bool, extra: &[&str]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_misterdiagram"));
        command.arg("render");
        if masters {
            command.arg("--masters");
        }
        command.args(extra).arg(&self.input).arg(output).output().unwrap()
    }
    fn no_temporaries(&self) {
        assert!(fs::read_dir(&self.dir).unwrap().all(|e| !e.unwrap().file_name().to_string_lossy().starts_with(".misterdiagram-")));
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}
#[cfg(unix)]
fn succeeded(result: &Output) {
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
}
#[cfg(unix)]
fn mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path).unwrap().permissions().mode() & 0o777
}

#[cfg(unix)]
#[test]
fn render_replaces_symlinks_for_all_five_outputs() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new("symlinks");
    for (index, (ext, masters)) in [("png", false), ("svg", false), ("pdf", false), ("pptx", false), ("png", true)].into_iter().enumerate() {
        let target = f.dir.join(format!("old-{index}"));
        let output = f.dir.join(format!("link-{index}.{ext}"));
        let expected = f.dir.join(format!("regular-{index}.{ext}"));
        fs::write(&target, b"keep the old target byte for byte").unwrap();
        symlink(&target, &output).unwrap();
        let regular = f.render(&expected, masters, &[]);
        succeeded(&regular);
        let linked = f.render(&output, masters, &[]);
        succeeded(&linked);
        assert!(fs::symlink_metadata(&output).unwrap().file_type().is_file());
        assert_eq!(fs::read(&target).unwrap(), b"keep the old target byte for byte");
        assert_eq!(fs::read(&output).unwrap(), fs::read(&expected).unwrap());
        assert_eq!(linked.stderr, regular.stderr);
        f.no_temporaries();
    }
}

#[cfg(unix)]
#[test]
fn render_save_and_export_use_creation_mode_and_preserve_existing_modes() {
    use std::os::unix::fs::PermissionsExt;
    let mut f = Fixture::new("modes");
    let control = f.dir.join("plain-created");
    fs::File::create(&control).unwrap();
    let expected = mode(&control);
    eprintln!("plain creation mode: {expected:04o}");
    for (index, (ext, masters)) in [("png", false), ("svg", false), ("pdf", false), ("pptx", false), ("png", true)].into_iter().enumerate() {
        let path = f.dir.join(format!("render-{index}.{ext}"));
        succeeded(&f.render(&path, masters, &[]));
        assert_eq!(mode(&path), expected, "render {ext}, masters={masters}");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        succeeded(&f.render(&path, masters, &[]));
        assert_eq!(mode(&path), 0o600);
    }
    for (command, ext) in [("file.save", "vsdx"), ("file.export", "png"), ("file.export", "svg"), ("file.export", "pdf"), ("file.export", "pptx")] {
        let path = f.dir.join(format!("{command}.{ext}"));
        f.session.execute(command, &json!({"path":path})).unwrap();
        assert_eq!(mode(&path), expected, "{command} {ext}");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        f.session.execute(command, &json!({"path":path})).unwrap();
        assert_eq!(mode(&path), 0o600);
    }
    f.no_temporaries();
}

#[cfg(unix)]
#[test]
fn render_errors_leave_existing_output_and_no_temporary() {
    use std::os::unix::fs::PermissionsExt;
    struct RestoreMode(PathBuf, fs::Permissions);
    impl Drop for RestoreMode {
        fn drop(&mut self) {
            let _ = fs::set_permissions(&self.0, self.1.clone());
        }
    }
    let f = Fixture::new("failures");
    let path = f.dir.join("existing.png");
    fs::write(&path, b"original output bytes").unwrap();
    let original = fs::metadata(&f.dir).unwrap().permissions();
    let restore = RestoreMode(f.dir.clone(), original);
    fs::set_permissions(&f.dir, fs::Permissions::from_mode(0o555)).unwrap();
    let probe = fs::OpenOptions::new().write(true).create_new(true).open(f.dir.join("permission-probe"));
    // A privileged runner may bypass Unix mode bits; do not claim that case exercised refusal.
    if probe.is_err() {
        eprintln!("read-only parent refused the temporary-file probe");
        let result = f.render(&path, false, &[]);
        assert_eq!(result.status.code(), Some(1));
        assert_eq!(fs::read(&path).unwrap(), b"original output bytes");
    } else {
        eprintln!("runner bypasses read-only directory permissions; refusal case unavailable");
    }
    drop(restore);
    f.no_temporaries();
    let result = f.render(&path, false, &["--page", "99"]);
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&result.stderr).contains("page ordinal does not exist"));
    assert_eq!(fs::read(&path).unwrap(), b"original output bytes");
    f.no_temporaries();
}

#[test]
fn blank_contact_sheet_is_published_before_failure() {
    let f = Fixture::new("blank");
    let mut doc = f.session.document().clone();
    doc.masters.retain(|master| master.name == "Original rectangle");
    assert_eq!(doc.masters.len(), 1);
    doc.masters[0].shapes.clear();
    doc.masters[0].name = "Empty original".into();
    doc.masters[0].name_u = "Empty original".into();
    let id = doc.masters[0].id;
    fs::write(&f.input, misterdiagram_vsdx::write(&doc).unwrap().bytes).unwrap();
    let path = f.dir.join("contact.png");
    let result = f.render(&path, true, &[]);
    assert_eq!(result.status.code(), Some(1));
    assert!(image::load_from_memory(&fs::read(&path).unwrap()).is_ok());
    assert_eq!(
        String::from_utf8(result.stderr).unwrap(),
        format!(
            "warning: master {id} Empty original: blank drawing area\ncontact {}: masters=1 rendered=0 blank=1 failed=0\nerror: contact sheet contains blank or failed masters\n",
            f.input.display()
        )
    );
    f.no_temporaries();
}
