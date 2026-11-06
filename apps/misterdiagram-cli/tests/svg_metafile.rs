//! Original metafile records exercise format-specific diagnostics on public surfaces.
#[path = "../../../crates/render/tests/support/wmf_document.rs"]
mod picture;
#[path = "../../../crates/render/tests/support/emf_text.rs"]
mod recipe;
use misterdiagram_engine::Session;
use misterdiagram_mcp::{headless::Headless, server::Server};
use serde_json::{Value, json};
use std::{path::Path, process::Command};
const WARNING: &str = "shape 7: 2 metafile text records kept as outlines (symbol font or glyph indexes)";
fn call(server: &mut Server, name: &str, args: Value) -> Value {
    server.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":args}})).unwrap()
}
#[test]
fn svg_metafile_warning_is_per_shape_and_only_on_editable_svg() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/svg-metafile-public").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("original.vsdx");
    let mut doc = picture::document(recipe::fixture());
    doc.pages[0].shapes[0]
        .foreign_data
        .as_mut()
        .unwrap()
        .attributes
        .insert("xmlns:r".into(), "http://schemas.openxmlformats.org/officeDocument/2006/relationships".into());
    std::fs::write(&input, misterdiagram_vsdx::write(&doc).unwrap().bytes).unwrap();
    let mut session = Session::default();
    session.execute("file.open", &json!({"path":input})).unwrap();
    let mut server = Server::new(Box::new(Headless::new()));
    server.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"metafile-test","version":"1"}}})).unwrap();
    assert_eq!(call(&mut server, "open", json!({"path":input}))["result"]["isError"], false);
    for (ext, outlines) in [("svg", false), ("svg", true), ("png", false), ("pdf", false), ("pptx", false)] {
        for surface in ["render", "convert", "file.export", "mcp-render", "mcp-export"] {
            let output = dir.join(format!("{surface}-{outlines}.{ext}"));
            let expected = usize::from(ext == "svg" && !outlines);
            let diagnostics = if surface == "file.export" {
                session.execute(surface, &json!({"path":output,"text_outlines":outlines})).unwrap().to_string()
            } else if surface.starts_with("mcp-") {
                let value = call(&mut server, surface.strip_prefix("mcp-").unwrap(), json!({"path":output,"text_outlines":outlines}));
                assert_eq!(value["result"]["isError"], false, "{value}");
                // The MCP reply includes the structured result and its text rendering.
                let text = value["result"]["content"][0]["text"].as_str().unwrap();
                text.to_string()
            } else {
                let mut command = Command::new(env!("CARGO_BIN_EXE_misterdiagram"));
                command.arg(surface).arg(&input).arg(&output);
                if outlines {
                    command.arg("--text-outlines");
                }
                let result = command.output().unwrap();
                assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
                format!("{}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr))
            };
            assert_eq!(diagnostics.matches(WARNING).count(), expected, "{surface} {ext} {outlines}: {diagnostics}");
            if ext == "svg" {
                assert_eq!(std::fs::read_to_string(&output).unwrap().matches("<text ").count(), if outlines { 0 } else { 2 });
            }
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
