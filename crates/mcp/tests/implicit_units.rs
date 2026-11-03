//! Public regression for #83 on a rectangle authored entirely through MCP.
use misterdiagram_mcp::{headless::Headless, server::Server};
use serde_json::{Value, json};
fn call(s: &mut Server, name: &str, args: Value) -> Value {
    let reply = s.handle(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":args}})).unwrap();
    assert!(reply.get("error").is_none(), "{reply}");
    assert_eq!(reply["result"]["isError"], false, "{reply}");
    serde_json::from_str(reply["result"]["content"][0]["text"].as_str().unwrap()).unwrap()
}
#[test]
fn mcp_drawn_rectangle_formats_implicit_square_inches() {
    let mut s = Server::new(Box::new(Headless::new()));
    s.handle(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"units-test","version":"1"}}})).unwrap();
    call(&mut s, "run_command", json!({"command":"shape.draw.rect","params":{"x":1,"y":2,"width":2,"height":0.7}}));
    assert!(call(&mut s, "sheet_get", json!({"id":1,"name":"Width"}))["units"].is_null());
    call(&mut s, "run_command", json!({"command":"sheet.section.add","params":{"id":1,"section":"User"}}));
    call(&mut s, "run_command", json!({"command":"sheet.row.add","params":{"id":1,"section":"User","row_name":"Area"}}));
    call(&mut s, "sheet_set", json!({"id":1,"name":"User.Area","formula":"FORMAT(Width*Height,\"0.00 u^2\")"}));
    assert_eq!(call(&mut s, "sheet_get", json!({"id":1,"name":"User.Area"}))["value"], "1.40 in.^2");
    call(&mut s, "sheet_set", json!({"id":1,"name":"Width","formula":"8"}));
    assert_eq!(call(&mut s, "sheet_get", json!({"id":1,"name":"User.Area"}))["value"], "5.60 in.^2");
}
