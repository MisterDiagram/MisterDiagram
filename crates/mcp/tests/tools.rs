//! Exercise every tool through JSON-RPC, including actual parameter effects.
use misterdiagram_mcp::{headless::Headless, server::Server};
use serde_json::{Value, json};
fn rpc(s: &mut Server, method: &str, params: Value) -> Value {
    let value = s.handle(json!({"jsonrpc":"2.0","id":1,"method":method,"params":params})).unwrap();
    assert!(value.get("error").is_none(), "{value}");
    value["result"].clone()
}
fn call(s: &mut Server, name: &str, args: Value) -> Value {
    let value = rpc(s, "tools/call", json!({"name":name,"arguments":args}));
    assert_eq!(value["isError"], false, "{name}: {value}");
    serde_json::from_str(value["content"][0]["text"].as_str().unwrap()).unwrap()
}
fn command(s: &mut Server, name: &str, params: Value) -> Value {
    call(s, "run_command", json!({"command":name,"params":params}))
}
#[test]
fn lifecycle_and_all_fourteen_tools_honor_parameters() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/mcp-tools").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let mut s = Server::new(Box::new(Headless::new()));
    let init = rpc(&mut s, "initialize", json!({"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1"}}));
    assert_eq!(init["serverInfo"]["name"], "misterdiagram");
    assert_eq!(init["protocolVersion"], "2025-06-18");
    assert!(s.handle(json!({"jsonrpc":"2.0","method":"notifications/initialized"})).is_none());
    assert!(s.is_initialized());
    let list = rpc(&mut s, "tools/list", json!({}));
    assert_eq!(list["tools"].as_array().unwrap().len(), 14);
    for tool in list["tools"].as_array().unwrap() {
        assert_eq!(tool["inputSchema"]["type"], "object");
        let name = tool["name"].as_str().unwrap();
        for args in [Value::Null, json!([]), json!({"unexpected":true})] {
            let failed = rpc(&mut s, "tools/call", json!({"name":name,"arguments":args}));
            assert_eq!(failed["isError"], true, "{name}: {failed}");
        }
    }
    let commands = call(&mut s, "list_commands", json!({}));
    assert!(commands.as_array().unwrap().iter().any(|c| c["id"] == "shape.inspect"));
    command(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}));
    command(&mut s, "shape.draw.rect", json!({"x":5,"y":1,"width":2,"height":1}));
    call(&mut s, "sheet_set", json!({"id":1,"name":"Width","value":2.5}));
    assert_eq!(call(&mut s, "sheet_get", json!({"id":1,"name":"Width"}))["value"], "2.5");
    let shape = call(&mut s, "inspect_shape", json!({"id":2,"cells":["PinX"]}));
    assert_eq!(shape["id"], 2);
    assert_eq!(shape["cells"]["PinX"]["value"], "6");
    let batch = call(
        &mut s,
        "batch",
        json!({"commands":[{"command":"shape.text","params":{"id":1,"text":"Orchard"}},{"command":"page.add","params":{"name":"Second"}}]}),
    );
    assert_eq!(batch["results"].as_array().unwrap().len(), 2);
    let inspected = call(&mut s, "inspect_document", json!({"page":2}));
    assert_eq!(inspected["pages"].as_array().unwrap().len(), 1);
    assert_eq!(inspected["pages"][0]["name"], "Second");
    let found = call(&mut s, "stencil_search", json!({"query":"Dynamic"}));
    assert!(found.to_string().contains("Dynamic"));
    assert!(!call(&mut s, "stencil_search", json!({"query":"DoesNotExist"})).to_string().contains("Dynamic"));
    let dropped = call(&mut s, "drop_master", json!({"page":1,"master":2,"x":0,"y":0}));
    assert_eq!(dropped["id"], 3);
    command(&mut s, "shape.connection.add", json!({"page":1,"id":1,"x":"Width","y":"Height/2"}));
    call(&mut s, "connect", json!({"page":1,"id":3,"end":"begin","target":1,"mode":"static","point":"Connections.X1"}));
    call(&mut s, "connect", json!({"page":1,"id":3,"end":"end","target":2,"mode":"dynamic"}));
    let connector = call(&mut s, "inspect_shape", json!({"page":1,"id":3}));
    assert_eq!(connector["connects"].as_array().unwrap().len(), 2);
    assert_eq!(connector["connects"][0]["to_cell"], "Connections.X1");
    let saved = dir.join("saved.vsdx");
    call(&mut s, "save", json!({"path":saved}));
    assert!(saved.exists());
    for ext in ["vsdx", "png", "svg", "pdf"] {
        let path = dir.join(format!("export.{ext}"));
        call(&mut s, "export", json!({"path":path}));
        assert!(std::fs::metadata(path).unwrap().len() > 100);
    }
    let path = dir.join("render.png");
    call(&mut s, "render", json!({"path":path,"page":1,"scale":2}));
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
    command(&mut s, "file.new", json!({}));
    call(&mut s, "open", json!({"path":saved}));
    assert_eq!(call(&mut s, "inspect_document", json!({}))["pages"].as_array().unwrap().len(), 2);
    assert!(call(&mut s, "inspect_shape", json!({"page":1,"id":1}))["text"].as_str().unwrap().contains("Orchard"));
    let doc = call(&mut s, "inspect_document", json!({}));
    let failed = rpc(
        &mut s,
        "tools/call",
        json!({"name":"batch","arguments":{"commands":[
        {"command":"shape.move","params":{"page":1,"id":1,"x":3,"y":3}},
        {"command":"shape.text","params":{"page":1,"id":1,"text":"Rollback"}},
        {"command":"sheet.set","params":{"id":999,"name":"Width","value":1}},
        {"command":"file.new"},{"command":"file.new"}]}}),
    );
    assert_eq!(failed["isError"], true);
    assert!(failed.to_string().contains("index 2 (command 3)"));
    assert_eq!(call(&mut s, "inspect_document", json!({})), doc, "failed batch must restore full inspection");
}

#[test]
fn saved_batch_matches_individual_tool_calls_with_only_permitted_masks() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/mcp-equivalence").join(std::process::id().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let commands = json!([
        {"command":"shape.draw.rect","params":{"x":1,"y":1,"width":2,"height":1}},
        {"command":"master.new_from_shape","params":{"id":1,"name":"Maple"}},
        {"command":"master.drop","params":{"master":"Maple","x":5,"y":4}},
        {"command":"shape.move","params":{"id":2,"x":6,"y":5}}
    ]);
    for batch in [false, true] {
        let mut server = Server::new(Box::new(Headless::new()));
        if batch {
            call(&mut server, "batch", json!({"commands":commands}));
        } else {
            for command in commands.as_array().unwrap() {
                call(&mut server, "run_command", command.clone());
            }
        }
        call(&mut server, "save", json!({"path":dir.join(if batch {"batch.vsdx"} else {"strict.vsdx"})}));
    }
    let result = std::process::Command::new(if cfg!(windows) { "python" } else { "python3" })
        .arg(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/bench/compare.py"))
        .arg(dir.join("strict.vsdx"))
        .arg(dir.join("batch.vsdx"))
        .output()
        .unwrap();
    assert!(result.status.success(), "{}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}
