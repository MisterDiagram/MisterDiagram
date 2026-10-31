//! Contributor-original canvas checks across CLI, engine and MCP.
use misterdiagram_engine::Session;
use misterdiagram_mcp::{headless::Headless, server::Server};
use serde_json::{Value, json};
use std::{path::Path, process::Command};
fn call(server: &mut Server, name: &str, arguments: Value) -> Value {
    server.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":arguments}})).unwrap()
}
#[test]
fn public_exports_use_the_same_page_canvas_including_empty_pages() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/canvas-public").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    for empty in [false, true] {
        let input = dir.join(format!("original-{empty}.vsdx"));
        let mut s = Session::default();
        if !empty {
            s.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":1,"height":0.5})).unwrap();
        }
        s.execute("file.save", &json!({"path":input})).unwrap();
        let mut server = Server::new(Box::new(Headless::new()));
        server.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"canvas-test","version":"1"}}})).unwrap();
        assert_eq!(call(&mut server, "open", json!({"path":input}))["result"]["isError"], false);
        for ext in ["svg", "pdf"] {
            let mut baseline = None;
            for surface in ["render", "convert", "file.export", "mcp-export"] {
                let output = dir.join(format!("{surface}-{empty}.{ext}"));
                if surface == "mcp-export" {
                    assert_eq!(call(&mut server, "export", json!({"path":output}))["result"]["isError"], false);
                } else if surface == "file.export" {
                    s.execute(surface, &json!({"path":output})).unwrap();
                } else {
                    let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).arg(surface).arg(&input).arg(&output).output().unwrap();
                    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
                }
                let bytes = std::fs::read(output).unwrap();
                if ext == "svg" {
                    let xml = std::str::from_utf8(&bytes).unwrap();
                    assert!(xml.contains("width=\"8.5in\" height=\"11in\""), "{surface}: {xml}");
                }
                if let Some(prior) = &baseline {
                    assert_eq!(&bytes, prior, "{surface}/{ext}/{empty}");
                } else {
                    baseline = Some(bytes);
                }
            }
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
