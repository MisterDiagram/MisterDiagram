//! All public PPTX entry points and hostile options on an original drawing.
use misterdiagram_engine::Session;
use misterdiagram_mcp::{headless::Headless, server::Server};
use misterdiagram_opc::Package;
use serde_json::{Value, json};
use std::process::Command;

fn call(server: &mut Server, args: Value) -> Value {
    server.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"export","arguments":args}})).unwrap()
}
fn count(path: &std::path::Path) -> usize {
    Package::open(&std::fs::read(path).unwrap()).unwrap().xml("ppt/presentation.xml").unwrap().root.child("sldIdLst").unwrap().elements().count()
}
#[test]
fn pptx_public_surfaces_and_hostile_parameters() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/pptx-public").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("original.vsdx");
    let mut session = Session::default();
    session.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":2,"height":1})).unwrap();
    session.execute("page.add", &json!({"name":"Another original page"})).unwrap();
    session.execute("file.save", &json!({"path":input})).unwrap();
    let mut server = Server::new(Box::new(Headless::new()));
    server.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"pptx-test","version":"1"}}})).unwrap();
    server.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"open","arguments":{"path":input}}})).unwrap();
    for surface in ["engine", "render", "convert", "mcp"] {
        let output = dir.join(format!("{surface}.PPTX"));
        match surface {
            "engine" => {
                session.execute("file.export", &json!({"path":output})).unwrap();
            }
            "mcp" => assert_eq!(call(&mut server, json!({"path":output}))["result"]["isError"], false),
            _ => {
                let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).arg(surface).arg(&input).arg(&output).output().unwrap();
                assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
            }
        }
        assert_eq!(count(&output), 2, "{surface}");
    }
    let selected = dir.join("selected.pptx");
    session.execute("file.export", &json!({"path":selected,"page":2})).unwrap();
    assert_eq!(count(&selected), 1);
    for extra in [
        json!({"page":0}),
        json!({"page":999999999999u64}),
        json!({"page":"1"}),
        json!({"scale":0}),
        json!({"scale":101}),
        json!({"fit":"yes"}),
        json!({"margin":"5 mm"}),
    ] {
        let mut args = extra;
        args["path"] = json!(selected);
        let before = std::fs::read(&selected).unwrap();
        assert!(session.execute("file.export", &args).is_err());
        assert_eq!(call(&mut server, args)["result"]["isError"], true);
        assert_eq!(std::fs::read(&selected).unwrap(), before);
    }
    for value in ["0", "999999999999", "invalid"] {
        let result = Command::new(env!("CARGO_BIN_EXE_misterdiagram")).args(["render", "--page", value]).arg(&input).arg(&selected).output().unwrap();
        assert!(!result.status.success());
        assert_eq!(count(&selected), 1);
    }
    std::fs::remove_dir_all(dir).unwrap();
}
