//! Contributor-written opaque object exercises every public drawing entry point.
use misterdiagram_mcp::{headless::Headless, server::Server};
use misterdiagram_opc::{PackageOut, RelsOut};
use serde_json::json;
use std::{path::Path, process::Command};
fn original() -> Vec<u8> {
    let mut p = PackageOut::default();
    p.xml_part(
        "visio/document.xml",
        "application/vnd.ms-visio.drawing.main+xml",
        br#"<VisioDocument xmlns="http://schemas.microsoft.com/office/visio/2012/main"/>"#.to_vec(),
    );
    p.xml_part("visio/pages/pages.xml","application/vnd.ms-visio.pages+xml",br#"<Pages xmlns="http://schemas.microsoft.com/office/visio/2012/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><Page ID="0" NameU="Original"><PageSheet><Cell N="PageWidth" V="4"/><Cell N="PageHeight" V="4"/></PageSheet><Rel r:id="rId1"/></Page></Pages>"#.to_vec());
    p.xml_part("visio/pages/page1.xml","application/vnd.ms-visio.page+xml",br#"<PageContents xmlns="http://schemas.microsoft.com/office/visio/2012/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><Shapes><Shape ID="7" NameU="Original opaque object" Type="Foreign"><Cell N="Width" V="2"/><Cell N="Height" V="1"/><Cell N="PinX" V="2"/><Cell N="PinY" V="2"/><Cell N="LocPinX" V="1"/><Cell N="LocPinY" V="0.5"/><ForeignData ForeignType="Object" ObjectType="17" ObjectWidth="2" ObjectHeight="1" ShowAsIcon="1"><Rel r:id="rId1"/></ForeignData></Shape></Shapes></PageContents>"#.to_vec());
    p.binary("visio/embeddings/original.bin", "application/octet-stream", vec![1, 3, 5, 7, 9]);
    for (owner, kind, target) in [
        ("", "http://schemas.microsoft.com/visio/2010/relationships/document", "visio/document.xml"),
        ("visio/document.xml", "http://schemas.microsoft.com/visio/2010/relationships/pages", "pages/pages.xml"),
        ("visio/pages/pages.xml", "http://schemas.microsoft.com/visio/2010/relationships/page", "page1.xml"),
        ("visio/pages/page1.xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject", "../embeddings/original.bin"),
    ] {
        let mut rels = RelsOut::default();
        rels.add(kind, target, false);
        p.rels(owner, &rels);
    }
    p.finish().unwrap()
}
fn call(server: &mut Server, name: &str, args: serde_json::Value) -> serde_json::Value {
    let response = server.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":args}})).unwrap();
    assert!(response.get("error").is_none(), "{response}");
    assert_eq!(response["result"]["isError"], false, "{response}");
    response
}
#[test]
fn garbage_foreign_object_recovers_in_render_convert_and_mcp_export_but_sheet_reports_it() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/brief37-foreign-cli").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("original.vsdx");
    std::fs::write(&input, original()).unwrap();
    for command in ["render", "convert"] {
        for ext in ["png", "svg", "pdf"] {
            let path = dir.join(format!("{command}.{ext}"));
            let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).arg(command).arg(&input).arg(&path).output().unwrap();
            assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
            assert!(std::fs::metadata(path).unwrap().len() > 10);
            let messages = format!("{}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
            assert!(messages.contains("ObjectType=17") && messages.contains("placeholder"), "{messages}");
        }
    }
    let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).arg("sheet").arg(&input).output().unwrap();
    assert_eq!(result.status.code(), Some(1));
    let error = String::from_utf8_lossy(&result.stderr);
    assert!(error.contains("shape 7") && error.contains("ObjectType=17"), "{error}");
    let mut server = Server::new(Box::new(Headless::new()));
    server.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"original-test","version":"1"}}})).unwrap();
    call(&mut server, "open", json!({"path":input}));
    for ext in ["png", "svg", "pdf"] {
        let path = dir.join(format!("mcp.{ext}"));
        let response = call(&mut server, "export", json!({"path":path}));
        assert!(response.to_string().contains("ObjectType=17") && response.to_string().contains("placeholder"), "{response}");
        assert!(std::fs::metadata(path).unwrap().len() > 10);
    }
    std::fs::remove_dir_all(&dir).unwrap();
}
