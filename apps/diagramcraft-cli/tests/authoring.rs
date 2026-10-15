//! CLI command sequencing, conversion and hostile argument regressions.
use std::{
    path::PathBuf,
    process::{Command, Output},
};
fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_diagramcraft-cli")).args(args).output().unwrap()
}
fn dir() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/cli-authoring-tests");
    std::fs::create_dir_all(&path).unwrap();
    path
}
#[test]
fn commands_and_describe_publish_parseable_schemas() {
    let output = cli(&["commands"]);
    assert!(output.status.success());
    let commands: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(commands.as_array().unwrap().len(), diagramcraft_engine::cmd::command_specs().len());
    for command in commands.as_array().unwrap() {
        let schema: serde_json::Value = serde_json::from_str(command["params"].as_str().unwrap()).unwrap();
        assert_eq!(schema["type"], "object");
    }
    let output = cli(&["describe", "connect.dynamic"]);
    assert!(output.status.success());
    let info: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(info["id"], "connect.dynamic");
}
#[test]
fn run_uses_one_session_and_convert_dispatches_by_extension() {
    let directory = dir();
    let drawing = directory.join("sequence.vsdx");
    let drawing = drawing.to_str().unwrap();
    let output = cli(&[
        "run",
        "--cmd",
        "file.new={}",
        "--cmd",
        r#"shape.draw.rect={"x":1,"y":1,"width":2,"height":1}"#,
        "--cmd",
        r#"shape.move={"id":1,"x":4,"y":5}"#,
        "--save",
        drawing,
    ]);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let doc = diagramcraft_vsdx::read(drawing).unwrap();
    assert_eq!(doc.pages[0].shapes[0].sheet.cell("PinX").unwrap().value.as_deref(), Some("4"));
    for extension in ["vssx", "vstx", "svg", "png", "pdf"] {
        let path = directory.join(format!("converted.{extension}"));
        let output = cli(&["convert", drawing, path.to_str().unwrap()]);
        assert!(output.status.success(), "{extension}: {}", String::from_utf8_lossy(&output.stderr));
        assert!(std::fs::metadata(path).unwrap().len() > 10);
    }
    let path = directory.join("opened.vsdx");
    let output = cli(&["run", "--open", drawing, "--cmd", r#"shape.text={"id":1,"text":"New text"}"#, "--save", path.to_str().unwrap()]);
    assert!(output.status.success());
    assert!(diagramcraft_vsdx::read(path).unwrap().pages[0].shapes[0].text.as_ref().unwrap().text().contains("New text"));
}
#[test]
fn hostile_arguments_fail_cleanly_for_each_new_subcommand() {
    let malformed = format!("file.new={}{}", "[".repeat(200), "]".repeat(200));
    for args in [
        vec!["commands", "extra"],
        vec!["describe"],
        vec!["describe", "unknown"],
        vec!["describe", "file.new", "extra"],
        vec!["run"],
        vec!["run", "--cmd"],
        vec!["run", "--cmd", "file.new"],
        vec!["run", "--cmd", "unknown={}"],
        vec!["run", "--cmd", "file.new={bad}"],
        vec!["run", "--cmd", "file.new=[]"],
        vec!["run", "--cmd", &malformed],
        vec!["run", "--open", "missing.vsdx"],
        vec!["run", "--open", "a", "--open", "b"],
        vec!["run", "--save", "a", "--save", "b"],
        vec!["convert"],
        vec!["convert", "missing.vsdx", "out.png"],
        vec!["convert", "missing.vsdx", "out.exe"],
        vec!["convert", "missing.vsdx", "out.png", "extra"],
    ] {
        let output = cli(&args);
        assert!(!output.status.success(), "{args:?}");
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert_eq!(stderr.lines().count(), 1, "{args:?}: {stderr}");
        assert!(!stderr.contains("panicked"));
    }
    let args = vec!["commands"; 4097];
    assert!(!cli(&args).status.success());
}
#[test]
fn failing_command_does_not_write_the_final_save() {
    let path = dir().join("preserved.vsdx");
    std::fs::write(&path, b"sentinel").unwrap();
    let output = cli(&["run", "--cmd", r#"shape.move={"id":999,"x":1,"y":1}"#, "--save", path.to_str().unwrap()]);
    assert!(!output.status.success());
    assert_eq!(std::fs::read(path).unwrap(), b"sentinel");
}
#[cfg(unix)]
#[test]
fn non_utf8_authoring_argument_returns_an_error() {
    use std::os::unix::ffi::OsStringExt;
    let output = Command::new(env!("CARGO_BIN_EXE_diagramcraft-cli"))
        .args([std::ffi::OsString::from("describe"), std::ffi::OsString::from_vec(vec![255])])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
}
