//! Contributor-original colour table verifies constructor diagnostics at public entry points.
use misterdiagram_mcp::{headless::Headless, server::Server};
use misterdiagram_opc::{PackageOut, RelsOut};
use serde_json::{Value, json};
use std::{path::Path, process::Command};

fn original() -> Vec<u8> {
    let mut package = PackageOut::default();
    let colours: String = (0..=65536).map(|i| format!("<ColorEntry IX='{i}' RGB='#010203'/>")).collect();
    package.xml_part(
        "visio/document.xml",
        "application/vnd.ms-visio.drawing.main+xml",
        format!("<VisioDocument xmlns='http://schemas.microsoft.com/office/visio/2012/main'><Colors>{colours}</Colors></VisioDocument>").into_bytes(),
    );
    package.xml_part("visio/pages/pages.xml", "application/vnd.ms-visio.pages+xml",
        br#"<Pages xmlns="http://schemas.microsoft.com/office/visio/2012/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><Page ID="1"><Rel r:id="rId1"/></Page></Pages>"#.to_vec());
    package.xml_part("visio/pages/page1.xml", "application/vnd.ms-visio.page+xml",
        br#"<PageContents xmlns="http://schemas.microsoft.com/office/visio/2012/main"><Shapes><Shape ID="1"><Cell N="Width" V="1"/><Cell N="Height" V="1"/></Shape></Shapes></PageContents>"#.to_vec());
    for (owner, kind, target) in
        [("", "document", "visio/document.xml"), ("visio/document.xml", "pages", "pages/pages.xml"), ("visio/pages/pages.xml", "page", "page1.xml")]
    {
        let mut rels = RelsOut::default();
        rels.add(&format!("http://schemas.microsoft.com/visio/2010/relationships/{kind}"), target, false);
        package.rels(owner, &rels);
    }
    package.finish().unwrap()
}
fn call(server: &mut Server, name: &str, arguments: Value) -> Value {
    server.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":arguments}})).unwrap()
}
#[test]
fn structural_limit_detail_reaches_render_convert_export_sheet_and_mcp() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/brief42-limits").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("original.vsdx");
    std::fs::write(&input, original()).unwrap();
    let expected = "evaluator: colours cap of 65,536 exceeded while reading document colours (65,537 colours)";
    for command in ["render", "convert", "sheet"] {
        let mut process = Command::new(env!("CARGO_BIN_EXE_misterdiagram"));
        process.arg(command).arg(&input);
        if command != "sheet" {
            process.arg(dir.join(format!("{command}.png")));
        }
        let result = process.output().unwrap();
        assert_eq!(result.status.code(), Some(1));
        let message = String::from_utf8_lossy(&result.stderr);
        assert!(message.contains(expected), "{command}: {message}");
    }
    let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram"))
        .arg("run")
        .arg("--open")
        .arg(&input)
        .arg("--cmd")
        .arg(format!("file.export={}", json!({"path":dir.join("export.svg")})))
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&result.stderr).contains(expected), "{}", String::from_utf8_lossy(&result.stderr));
    let mut server = Server::new(Box::new(Headless::new()));
    server.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"original-limit-test","version":"1"}}})).unwrap();
    let opened = call(&mut server, "open", json!({"path":input}));
    assert_eq!(opened["result"]["isError"], false, "{opened}");
    for (tool, args) in [
        ("render", json!({"path":dir.join("mcp.png")})),
        ("export", json!({"path":dir.join("mcp.svg")})),
        ("export", json!({"path":dir.join("mcp.pdf")})),
        ("inspect_shape", json!({"page":1,"id":1,"cells":["Width"]})),
        ("sheet_get", json!({"page":1,"id":1,"name":"Width"})),
        ("run_command", json!({"command":"file.export","params":{"path":dir.join("command.png")}})),
    ] {
        let response = call(&mut server, tool, args);
        assert_eq!(response["result"]["isError"], true, "{tool}: {response}");
        assert!(response.to_string().contains(expected), "{tool}: {response}");
    }
    // Package writing has an earlier independent palette/font admission check.
    let response = call(&mut server, "export", json!({"path":dir.join("mcp.vsdx")}));
    assert_eq!(response["result"]["isError"], true);
    assert!(response.to_string().contains("cannot write Visio package: palette/font count limit"), "{response}");
    std::fs::remove_dir_all(dir).unwrap();
}
