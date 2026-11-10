//! MCP-only defaults leave engine calls and resource reads full.
use misterdiagram_mcp::{headless::Headless, server::Server};
use serde_json::{Value, json};
fn rpc(s: &mut Server, method: &str, p: Value) -> Value {
    s.handle(json!({"jsonrpc":"2.0","id":1,"method":method,"params":p})).unwrap()["result"].clone()
}
fn call(s: &mut Server, name: &str, p: Value) -> Value {
    let v = rpc(s, "tools/call", json!({"name":name,"arguments":p}));
    assert_eq!(v["isError"], false, "{v}");
    serde_json::from_str(v["content"][0]["text"].as_str().unwrap()).unwrap()
}
fn command(s: &mut Server, name: &str, p: Value) -> Value {
    call(s, "run_command", json!({"command":name,"params":p}))
}
#[test]
fn compact_defaults_allow_explicit_full_forms() {
    let mut s = Server::new(Box::new(Headless::new()));
    let full = command(&mut s, "engine.commands", json!({"detail":"full"}));
    let short = call(&mut s, "list_commands", json!({}));
    assert_eq!(short.as_array().unwrap().len(), full.as_array().unwrap().len());
    assert!(short.as_array().unwrap().iter().all(|c| c["params"].is_array()));
    assert!(full.as_array().unwrap().iter().all(|c| c["params"].is_object()));
    assert_eq!(call(&mut s, "list_commands", json!({"detail":"full"})), full);
    assert!(call(&mut s, "list_commands", json!({"ids":["shape.text"]}))[0]["params"].is_object());
    assert!(call(&mut s, "list_commands", json!({"ids":["shape.text"],"detail":"summary"}))[0]["params"].is_array());
    command(&mut s, "shape.draw.rect", json!({"x":1,"y":2,"width":3,"height":1}));
    let outline = call(&mut s, "inspect_document", json!({}));
    assert!(outline["pages"][0]["links"].is_array());
    assert!(outline["pages"][0].get("layers").is_none());
    let full_doc = command(&mut s, "document.inspect", json!({"sections":["listing"]}));
    assert!(full_doc["pages"][0]["layers"].is_object());
    assert!(full_doc["pages"][0]["connects"].is_array());
    assert!(full_doc["pages"][0]["shapes"][0]["bounds"].is_object());
    for p in [json!({"sections":[]}), json!({"cells":[]})] {
        assert_eq!(call(&mut s, "inspect_document", p.clone()), command(&mut s, "document.inspect", p));
    }
    for (uri, expected) in [("misterdiagram://commands", full), ("misterdiagram://document", full_doc)] {
        let resource = rpc(&mut s, "resources/read", json!({"uri":uri}));
        assert_eq!(serde_json::from_str::<Value>(resource["contents"][0]["text"].as_str().unwrap()).unwrap(), expected);
    }
    assert_eq!(rpc(&mut s, "tools/list", json!({}))["tools"].as_array().unwrap().len(), 17);
}
