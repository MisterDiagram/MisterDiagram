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
    s.execute("format.text", &json!({"id":1,"font":"Calibri"})).unwrap();
    s.execute("file.save", &json!({"path":drawing})).unwrap();
    let mut server = Server::new(Box::new(Headless::new()));
    server.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"text-test","version":"1"}}})).unwrap();
    assert_eq!(call(&mut server, "open", json!({"path":drawing}))["result"]["isError"], false);
    for mode in ["default", "outlines", "exact", "fallback", "exact-outlines"] {
        let outlines = mode.contains("outlines");
        let exact = mode.starts_with("exact");
        for surface in ["convert", "render", "file.export", "mcp-export", "mcp-render"] {
            let path = dir.join(format!("{surface}-{mode}.svg"));
            if surface.starts_with("mcp-") {
                let mut args = json!({"path":path});
                if exact || mode == "fallback" {
                    args["font_names"] = json!(if exact { "exact" } else { "fallback" });
                }
                if outlines {
                    args["text_outlines"] = json!(true);
                }
                let reply = call(&mut server, surface.strip_prefix("mcp-").unwrap(), args);
                assert_eq!(reply["result"]["isError"], false, "{reply}");
            } else if surface == "file.export" {
                let mut args = json!({"path":path});
                if exact || mode == "fallback" {
                    args["font_names"] = json!(if exact { "exact" } else { "fallback" });
                }
                if outlines {
                    args["text_outlines"] = json!(true);
                }
                s.execute(surface, &args).unwrap();
            } else {
                let mut cmd = Command::new(env!("CARGO_BIN_EXE_misterdiagram"));
                cmd.arg(surface).arg(&drawing).arg(&path);
                if exact || mode == "fallback" {
                    cmd.args(["--font-names", if exact { "exact" } else { "fallback" }]);
                }
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
                assert!(xml.contains("&apos;Calibri&apos;"));
                assert_eq!(xml.contains("Carlito"), !exact, "{surface} {mode}");
            }
        }
    }
    assert!(s.execute("file.export", &json!({"path":dir.join("bad.svg"),"text_outlines":"yes"})).is_err());
    for tool in ["render", "export"] {
        assert_eq!(call(&mut server, tool, json!({"path":dir.join("bad.svg"),"text_outlines":1}))["result"]["isError"], true);
    }
    for value in [json!("unknown"), json!(true), json!(1), Value::Null, json!({}), json!([])] {
        assert!(s.execute("file.export", &json!({"path":dir.join("bad.svg"),"font_names":value})).is_err());
        for tool in ["render", "export"] {
            assert_eq!(call(&mut server, tool, json!({"path":dir.join("bad.svg"),"font_names":value}))["result"]["isError"], true);
        }
    }
    let tools = server.handle(json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})).unwrap();
    for tool in ["render", "export"] {
        let schema = tools["result"]["tools"].as_array().unwrap().iter().find(|t| t["name"] == tool).unwrap();
        assert_eq!(schema["inputSchema"]["properties"]["font_names"]["enum"], json!(["fallback", "exact"]));
        assert_eq!(schema["inputSchema"]["properties"]["font_names"]["default"], "fallback");
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn svg_flag_is_documented_and_duplicate_or_unknown_flags_are_usage_errors() {
    for args in [vec!["--help"], vec!["render", "--help"], vec!["convert", "--help"]] {
        let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args(args).output().unwrap();
        assert!(result.status.success());
        assert!(String::from_utf8_lossy(&result.stdout).contains("--text-outlines"));
        assert!(String::from_utf8_lossy(&result.stdout).contains("--font-names"));
    }
    for surface in ["render", "convert"] {
        for tail in [vec!["--font-names"], vec!["--font-names", "bogus"], vec!["--font-names", "exact", "--font-names", "fallback"]] {
            let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args([surface, "in", "out"]).args(tail).output().unwrap();
            assert_eq!(result.status.code(), Some(2));
        }
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

#[test]
fn authored_three_item_list_exports_three_bullet_paths() {
    use misterdiagram_render::{raster, scene, svg};
    let mut session = Session::default();
    session.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":3,"height":2,"fill":{"pattern":0},"line":{"pattern":0}})).unwrap();
    session.execute("shape.text", &json!({"id":1,"text":"First item\nSecond item\nThird item"})).unwrap();
    session.execute("sheet.set", &json!({"id":1,"name":"Para.Bullet","formula":"1"})).unwrap();
    let scene = scene::page(session.document(), 0, scene::Options::default()).unwrap();
    let before = raster::render(&scene, scene::Options::default()).unwrap();
    let xml = svg::render(&scene, scene::Options::default()).unwrap();
    assert_eq!(xml.matches('•').count(), 0);
    assert_eq!(xml.matches("<path").count(), 3);
    assert_eq!(xml.matches("<text ").count(), 1);
    assert_eq!(before, raster::render(&scene, scene::Options { text_outlines: true, ..Default::default() }).unwrap());
}
