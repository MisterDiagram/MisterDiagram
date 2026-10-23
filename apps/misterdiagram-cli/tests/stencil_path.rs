//! CLI and stdio MCP share the ordered search path on every supported platform.
use serde_json::{Value, json};
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
#[test]
fn cli_and_mcp_flag_override_environment_and_list_without_parsing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../target/debug/stencil-path-cli-{}", std::process::id()));
    let first = root.join("first");
    let second = root.join("second");
    std::fs::create_dir_all(&first).unwrap();
    std::fs::create_dir_all(&second).unwrap();
    let first = first.canonicalize().unwrap();
    let second = second.canonicalize().unwrap();
    std::fs::write(first.join("unparsed.vssx"), b"not a package").unwrap();
    let binary = env!("CARGO_BIN_EXE_misterdiagram");
    let source = first.join("original.vssx");
    let r = Command::new(binary)
        .args([
            "run",
            "--cmd",
            "stencil.open={\"builtin\":\"network\"}",
            "--cmd",
            "master.drop={\"stencil\":1,\"master\":\"network.router\"}",
            "--save",
        ])
        .arg(&source)
        .output()
        .unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    std::fs::copy(&source, second.join("original.vssx")).unwrap();
    let path = std::env::join_paths([&first, &second]).unwrap();
    let r = Command::new(binary)
        .env("MISTERDIAGRAM_STENCIL_PATH", &second)
        .args(["run", "--stencil-path"])
        .arg(&path)
        .args(["--cmd", "stencil.open={\"path\":\"original\"}", "--cmd", "stencil.list={}"])
        .output()
        .unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let rows: Vec<Value> = String::from_utf8(r.stdout).unwrap().lines().map(|s| serde_json::from_str(s).unwrap()).collect();
    assert_eq!(rows[0]["result"]["path"], json!(source));
    assert_eq!(rows[1]["result"]["stencils"][1]["path"], json!(source));
    let input=[
      json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}),
      json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
      json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"stencil_available","arguments":{}}}),
      json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"run_command","arguments":{"command":"stencil.open","params":{"path":"original"}}}}),
    ].map(|v|v.to_string()+"\n").concat();
    let mut child = Command::new(binary)
        .env("MISTERDIAGRAM_STENCIL_PATH", &second)
        .args(["mcp", "--stencil-path"])
        .arg(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let rows: Vec<Value> = String::from_utf8(output.stdout).unwrap().lines().map(|s| serde_json::from_str(s).unwrap()).collect();
    assert_eq!(rows.len(), 3);
    let available: Value = serde_json::from_str(rows[1]["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(available["files"].as_array().unwrap().len(), 3);
    assert!(available["files"].as_array().unwrap().iter().any(|f| f["name"] == "unparsed.vssx" && f["size"] == 13));
    let opened: Value = serde_json::from_str(rows[2]["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(opened["path"], json!(source));
    std::fs::remove_dir_all(root).unwrap();
}
