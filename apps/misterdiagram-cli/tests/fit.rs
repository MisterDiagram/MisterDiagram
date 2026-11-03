//! Public fit flags, schemas and engine/MCP equivalence on an original drawing.
use misterdiagram_engine::Session;
use misterdiagram_mcp::{headless::Headless, server::Server};
use serde_json::{Value, json};
use std::process::Command;
fn call(s: &mut Server, name: &str, args: Value) -> Value {
    s.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":args}})).unwrap()
}
#[test]
fn all_public_export_paths_share_fit_and_margin() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/fit-public").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("original.vsdx");
    let mut s = Session::default();
    s.execute("shape.draw.rect", &json!({"x":-1,"y":3,"width":2,"height":1})).unwrap();
    s.execute("format.line", &json!({"id":1,"weight":12,"units":"PT"})).unwrap();
    s.execute("file.save", &json!({"path":input})).unwrap();
    let mut server = Server::new(Box::new(Headless::new()));
    server.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"fit-test","version":"1"}}})).unwrap();
    assert_eq!(call(&mut server, "open", json!({"path":input}))["result"]["isError"], false);
    for ext in ["png", "svg", "pdf"] {
        let expected = dir.join(format!("engine.{ext}"));
        s.execute("file.export", &json!({"path":expected,"fit":true,"margin":"0.25 in"})).unwrap();
        for surface in ["render", "convert", "mcp-render", "mcp-export"] {
            let output = dir.join(format!("{surface}.{ext}"));
            if let Some(tool) = surface.strip_prefix("mcp-") {
                let reply = call(&mut server, tool, json!({"path":output,"fit":true,"margin":"0.25 in"}));
                assert_eq!(reply["result"]["isError"], false, "{reply}");
            } else {
                let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram"))
                    .args([surface, "--fit", "--margin", "0.25 in"])
                    .arg(&input)
                    .arg(&output)
                    .output()
                    .unwrap();
                assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
            }
            assert_eq!(std::fs::read(&output).unwrap(), std::fs::read(&expected).unwrap(), "{surface}/{ext}");
        }
    }
    for p in [json!({"fit":"yes"}), json!({"fit":true,"margin":1}), json!({"margin":"1 in"}), json!({"fit":false,"margin":"1 in"})] {
        let mut p = p;
        p["path"] = json!(dir.join("bad.svg"));
        assert!(s.execute("file.export", &p).is_err());
        for tool in ["render", "export"] {
            assert_eq!(call(&mut server, tool, p.clone())["result"]["isError"], true);
        }
    }
    assert_eq!(call(&mut server, "export", json!({"path":dir.join("bad.vsdx"),"fit":true}))["result"]["isError"], true);
    let description = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args(["describe", "file.export"]).output().unwrap();
    let text = String::from_utf8(description.stdout).unwrap();
    assert!(text.contains("\"fit\"") && text.contains("\"margin\""));
    std::fs::remove_dir_all(dir).unwrap();
}
#[test]
fn invalid_flags_are_usage_errors_before_input_io() {
    for surface in ["render", "convert"] {
        for flags in [
            vec!["--fit", "--fit"],
            vec!["--fit", "--margin", "1 in", "--margin", "2 in"],
            vec!["--margin", "1 in"],
            vec!["--fit", "--margin"],
            vec!["--fit", "--margin", "-1 in"],
            vec!["--fit", "--margin", "NaN"],
            vec!["--fit", "--margin", "4 deg"],
        ] {
            let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args([surface, "missing.vsdx", "out.png"]).args(flags).output().unwrap();
            assert_eq!(result.status.code(), Some(2), "{}", String::from_utf8_lossy(&result.stderr));
        }
    }
    for args in [vec!["render", "--masters", "missing.vssx", "out.png", "--fit"], vec!["convert", "missing.vsdx", "out.vsdx", "--fit"]] {
        assert_eq!(Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args(args).output().unwrap().status.code(), Some(2));
    }
}
