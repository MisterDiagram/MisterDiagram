//! CLI command sequencing, conversion and hostile argument regressions.
use std::{
    path::PathBuf,
    process::{Command, Output},
};
fn cli(args: &[&str]) -> std::io::Result<Output> {
    Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args(args).output()
}
/// CreateProcessW caps its UTF-16 command line at 32,767 characters including NUL.
/// Only the known oversized cases may accept Windows ERROR_FILENAME_EXCED_RANGE
/// (206) before launch; delivered arguments must receive the CLI's own rejection.
/// https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
fn oversized_rejected(args: &[&str], diagnostic: &str) {
    assert!(args.iter().map(|arg| arg.encode_utf16().count() + 1).sum::<usize>() > 32767);
    match cli(args) {
        Err(error) if cfg!(windows) && error.raw_os_error() == Some(206) => {}
        Err(error) => panic!("unexpected CLI spawn failure: {error}"),
        Ok(output) => {
            assert_eq!(output.status.code(), Some(1));
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert_eq!(stderr.lines().count(), 1, "{stderr}");
            assert!(stderr.contains(diagnostic), "{stderr}");
            assert!(!stderr.contains("panicked"));
        }
    }
}
fn dir() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/cli-authoring-tests");
    std::fs::create_dir_all(&path).unwrap();
    path
}
#[test]
fn commands_and_describe_publish_parseable_schemas() {
    let output = cli(&["commands"]).unwrap();
    assert!(output.status.success());
    let commands: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(commands.as_array().unwrap().len(), misterdiagram_engine::cmd::command_specs().len());
    for command in commands.as_array().unwrap() {
        let schema = &command["params"];
        assert!(schema.is_object());
        assert_eq!(schema["type"], "object");
    }
    for id in ["connect.dynamic", "engine.execute", "engine.commands", "document.inspect", "shape.inspect"] {
        assert!(commands.as_array().unwrap().iter().any(|c| c["id"] == id));
        let output = cli(&["describe", id]).unwrap();
        assert!(output.status.success());
        let info: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(info["id"], id);
    }
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
    ])
    .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let doc = misterdiagram_vsdx::read(drawing).unwrap();
    assert_eq!(doc.pages[0].shapes[0].sheet.cell("PinX").unwrap().value.as_deref(), Some("4"));
    for extension in ["vssx", "vstx", "svg", "png", "pdf"] {
        let path = directory.join(format!("converted.{extension}"));
        let output = cli(&["convert", drawing, path.to_str().unwrap()]).unwrap();
        assert!(output.status.success(), "{extension}: {}", String::from_utf8_lossy(&output.stderr));
        assert!(std::fs::metadata(path).unwrap().len() > 10);
    }
    let path = directory.join("opened.vsdx");
    let output = cli(&["run", "--open", drawing, "--cmd", r#"shape.text={"id":1,"text":"New text"}"#, "--save", path.to_str().unwrap()]).unwrap();
    assert!(output.status.success());
    assert!(misterdiagram_vsdx::read(path).unwrap().pages[0].shapes[0].text.as_ref().unwrap().text().contains("New text"));
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
        let output = cli(&args).unwrap();
        assert!(!output.status.success(), "{args:?}");
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert_eq!(stderr.lines().count(), 1, "{args:?}: {stderr}");
        assert!(!stderr.contains("panicked"));
    }
    let args = vec!["commands"; 4097];
    oversized_rejected(&args, "argument count or byte limit");
}
#[test]
fn oversized_path_is_rejected_before_file_access() {
    let path = format!("{}.vsdx", "x".repeat(32769));
    oversized_rejected(&["run", "--open", &path], "path length is out of bounds");
}
#[test]
fn failing_command_does_not_write_the_final_save() {
    let path = dir().join("preserved.vsdx");
    std::fs::write(&path, b"sentinel").unwrap();
    let output = cli(&["run", "--cmd", r#"shape.move={"id":999,"x":1,"y":1}"#, "--save", path.to_str().unwrap()]).unwrap();
    assert!(!output.status.success());
    assert_eq!(std::fs::read(path).unwrap(), b"sentinel");
}
#[cfg(unix)]
#[test]
fn non_utf8_authoring_argument_returns_an_error() {
    use std::os::unix::ffi::OsStringExt;
    let output = Command::new(env!("CARGO_BIN_EXE_misterdiagram"))
        .args([std::ffi::OsString::from("describe"), std::ffi::OsString::from_vec(vec![255])])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
}
#[test]
fn authored_numeric_looking_strings_survive_cli_save_and_readback() {
    let path = dir().join("numeric-looking-strings.vsdx");
    let path = path.to_str().unwrap();
    let output = cli(&[
        "run",
        "--cmd",
        "file.new={}",
        "--cmd",
        r#"shape.draw.rect={"x":1,"y":1,"width":1,"height":1}"#,
        "--cmd",
        r#"shape.data.define={"id":1,"name":"Part","label":"007","prompt":"1e3","format":"0.00","sort_key":"01"}"#,
        "--cmd",
        r#"layer.add={"name":"0042"}"#,
        "--save",
        path,
    ])
    .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    for (scope, id, name, expected) in [
        ("shape", Some(1), "Prop.Part.Label", "007"),
        ("shape", Some(1), "Prop.Part.Prompt", "1e3"),
        ("shape", Some(1), "Prop.Part.Format", "0.00"),
        ("shape", Some(1), "Prop.Part.SortKey", "01"),
        ("page", None, "Layers.Name", "0042"),
    ] {
        let mut params = serde_json::json!({"scope":scope,"name":name});
        if let Some(id) = id {
            params["id"] = serde_json::json!(id);
        }
        let command = format!("sheet.get={params}");
        let output = cli(&["run", "--open", path, "--cmd", &command]).unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["result"]["cached"], expected, "{name}: {result}");
        assert_eq!(result["result"]["value"], expected, "{name}: {result}");
    }
}

#[test]
fn mcp_stdio_python_client_matches_cli_and_strict_batches() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let result = Command::new(if cfg!(windows) { "python" } else { "python3" })
        .arg(root.join("tools/mcp/test_stdio.py"))
        .env("MISTERDIAGRAM_CLI", env!("CARGO_BIN_EXE_misterdiagram"))
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .unwrap();
    assert!(result.status.success(), "{}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}

#[test]
fn batch_file_rebuilds_references_and_rejects_hostile_input() {
    let dir = dir().join("batch-file");
    std::fs::create_dir_all(&dir).unwrap();
    let recipe = dir.join("commands.json");
    let drawing = dir.join("drawing.vsdx");
    std::fs::write(
        &recipe,
        serde_json::to_vec(&serde_json::json!({"commands":[
            {"command":"shape.draw.rect","as":"host","params":{"x":1,"y":1,"width":2,"height":1,"text":"Cedar"}},
            {"command":"shape.move","params":{"id":"$host","x":5,"y":4}}
        ]}))
        .unwrap(),
    )
    .unwrap();
    let result = cli(&["run", "--batch", recipe.to_str().unwrap(), "--save", drawing.to_str().unwrap()]).unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let d = misterdiagram_vsdx::read(&drawing).unwrap();
    assert_eq!(d.pages[0].shapes[0].sheet.cell("PinX").unwrap().value.as_deref(), Some("5"));
    for data in [b"null".to_vec(), b"[]".to_vec(), b"{broken".to_vec(), vec![b' '; 1024 * 1024 + 1]] {
        std::fs::write(&recipe, data).unwrap();
        let result = cli(&["run", "--batch", recipe.to_str().unwrap()]).unwrap();
        assert_eq!(result.status.code(), Some(1));
        assert!(!String::from_utf8_lossy(&result.stderr).contains("panicked"));
    }
}
