//! SVG text defaults and outline opt-in across public command surfaces.
use misterdiagram_engine::Session;
use misterdiagram_mcp::{headless::Headless, server::Server};
use serde_json::{Value, json};
use std::{path::Path, process::Command};
fn call(server: &mut Server, name: &str, arguments: Value) -> Value {
    server.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":arguments}})).unwrap()
}
#[test]
fn cli_engine_and_mcp_share_svg_text_default_and_outline_flag() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/svg-text-public").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let drawing = dir.join("original.vsdx");
    let mut s = Session::default();
    s.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":3,"height":2})).unwrap();
    s.execute("shape.text", &json!({"id":1,"text":"Searchable line\nSecond line"})).unwrap();
    s.execute("file.save", &json!({"path":drawing})).unwrap();
    let mut server = Server::new(Box::new(Headless::new()));
    server.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"text-test","version":"1"}}})).unwrap();
    assert_eq!(call(&mut server, "open", json!({"path":drawing}))["result"]["isError"], false);
    for outlines in [false, true] {
        for surface in ["convert", "render", "file.export", "mcp-export", "mcp-render"] {
            let path = dir.join(format!("{surface}-{outlines}.svg"));
            if surface.starts_with("mcp-") {
                let mut args = json!({"path":path});
                if outlines {
                    args["text_outlines"] = json!(true);
                }
                let reply = call(&mut server, surface.strip_prefix("mcp-").unwrap(), args);
                assert_eq!(reply["result"]["isError"], false, "{reply}");
            } else if surface == "file.export" {
                let mut args = json!({"path":path});
                if outlines {
                    args["text_outlines"] = json!(true);
                }
                s.execute(surface, &args).unwrap();
            } else {
                let mut cmd = Command::new(env!("CARGO_BIN_EXE_misterdiagram"));
                cmd.arg(surface).arg(&drawing).arg(&path);
                if outlines {
                    cmd.arg("--text-outlines");
                }
                let result = cmd.output().unwrap();
                assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
            }
            let xml = std::fs::read_to_string(&path).unwrap();
            assert_eq!(xml.matches("<text ").count(), usize::from(!outlines), "{surface}");
            if !outlines {
                assert!(xml.contains("Searchable line"));
                assert!(xml.contains("Second line"));
            }
        }
    }
    assert!(s.execute("file.export", &json!({"path":dir.join("bad.svg"),"text_outlines":"yes"})).is_err());
    for tool in ["render", "export"] {
        assert_eq!(call(&mut server, tool, json!({"path":dir.join("bad.svg"),"text_outlines":1}))["result"]["isError"], true);
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn svg_flag_is_documented_and_duplicate_or_unknown_flags_are_usage_errors() {
    for args in [vec!["--help"], vec!["render", "--help"], vec!["convert", "--help"]] {
        let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args(args).output().unwrap();
        assert!(result.status.success());
        assert!(String::from_utf8_lossy(&result.stdout).contains("--text-outlines"));
    }
    for args in [
        vec!["convert", "in", "out", "--text-outlines", "--text-outlines"],
        vec!["convert", "in", "out", "--unknown"],
        vec!["render", "in", "out", "--text-outlines", "--text-outlines"],
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args(args).output().unwrap();
        assert_eq!(result.status.code(), Some(2));
    }
}
