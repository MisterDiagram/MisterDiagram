//! All MCP command routes share bounded defaults and preserve explicit inputs.
use misterdiagram_mcp::{backend::Backend, headless::Headless, server::Server, tools::call_tool};
use serde_json::{Value, json};

fn rpc(s: &mut Server, method: &str, params: Value) -> Value {
    s.handle(json!({"jsonrpc":"2.0","id":1,"method":method,"params":params})).unwrap()["result"].clone()
}
fn decode(v: Value) -> Value {
    assert_eq!(v["isError"], false, "{v}");
    serde_json::from_str(v["content"][0]["text"].as_str().unwrap()).unwrap()
}
fn call(s: &mut Server, name: &str, args: Value) -> Value {
    decode(rpc(s, "tools/call", json!({"name":name,"arguments":args})))
}
fn resource(s: &mut Server, uri: &str) -> Value {
    let r = rpc(s, "resources/read", json!({"uri":uri}));
    serde_json::from_str(r["contents"][0]["text"].as_str().unwrap()).unwrap()
}
fn drawing() -> Server {
    let mut s = Server::new(Box::new(Headless::new()));
    call(&mut s, "run_command", json!({"command":"shape.draw.rect","params":{"x":1,"y":1,"width":2,"height":1}}));
    s
}
#[test]
fn run_command_defaults_and_explicit_resource_forms() {
    let mut s = drawing();
    for (command, tool) in [("document.inspect", "inspect_document"), ("engine.commands", "list_commands")] {
        let expected = call(&mut s, tool, json!({}));
        for args in [json!({"command":command,"params":{}}), json!({"command":command})] {
            assert_eq!(call(&mut s, "run_command", args), expected);
        }
    }
    for (command, params, uri) in [
        ("document.inspect", json!({"sections":["listing"]}), "misterdiagram://document"),
        ("engine.commands", json!({"detail":"full"}), "misterdiagram://commands"),
    ] {
        assert_eq!(call(&mut s, "run_command", json!({"command":command,"params":params})), resource(&mut s, uri));
    }
    assert_eq!(call(&mut s, "run_command", json!({"command":"shape.inspect","params":{"id":1}})), call(&mut s, "inspect_shape", json!({"id":1})));
}
#[test]
fn batch_and_wrapped_batch_share_compact_results() {
    let mut s = drawing();
    let outline = call(&mut s, "inspect_document", json!({}));
    let summary = call(&mut s, "list_commands", json!({}));
    let items = json!([
        {"command":"document.inspect"},
        {"command":"engine.commands"},
        {"command":"engine.execute","params":{"command":"document.inspect"}}
    ]);
    let batch = call(&mut s, "batch", json!({"commands":items}));
    assert_eq!(batch["results"], json!([outline, summary, outline]));
    assert_eq!(call(&mut s, "run_command", json!({"command":"engine.batch","params":{"commands":items}})), batch);
}
struct Capture;
impl Backend for Capture {
    fn call(&mut self, command: &str, params: Value) -> Result<Value, String> {
        Ok(json!({"command":command,"params":params}))
    }
    fn has_ui(&self) -> bool {
        false
    }
    fn describe(&self) -> String {
        "capture".into()
    }
}
fn captured(tool: &str, args: Value) -> Value {
    decode(call_tool(&mut Capture, tool, &args).to_value())["params"].clone()
}
#[test]
fn explicit_inspection_parameters_are_unchanged_on_every_route() {
    for (command, tool, options) in [
        (
            "document.inspect",
            "inspect_document",
            vec![
                json!({"sections":[]}),
                json!({"sections":["text"]}),
                json!({"sections":["listing"]}),
                json!({"cells":[]}),
                json!({"cells":["Width"]}),
                json!({"sections":null}),
                json!({"cells":null}),
            ],
        ),
        (
            "engine.commands",
            "list_commands",
            vec![
                json!({"detail":"full"}),
                json!({"detail":"summary"}),
                json!({"ids":["shape.text"]}),
                json!({"ids":[]}),
                json!({"detail":null}),
                json!({"ids":null}),
            ],
        ),
    ] {
        for params in options {
            assert_eq!(captured(tool, params.clone()), params);
            let pair = json!({"command":command,"params":params});
            assert_eq!(captured("run_command", pair.clone()), pair);
            let alias = json!({"command":"engine.execute","params":pair});
            assert_eq!(captured("run_command", alias.clone()), alias);
            for item in [pair.clone(), alias] {
                let batch = json!({"commands":[item]});
                assert_eq!(captured("batch", batch.clone()), batch);
                let wrapped = json!({"command":"engine.batch","params":batch});
                assert_eq!(captured("run_command", wrapped.clone()), wrapped);
            }
        }
    }
}
#[test]
fn malformed_containers_discard_all_rewrites_and_keep_errors() {
    for bad in [
        json!(null),
        json!([]),
        json!("bad"),
        json!({"command":"document.inspect","params":"bad"}),
        json!({"command":"engine.execute","params":false}),
        json!({"command":"engine.batch","params":{"commands":"bad"}}),
    ] {
        let batch = json!({"commands":[{"command":"document.inspect"},bad]});
        assert_eq!(captured("batch", batch.clone()), batch);
        let wrapped = json!({"command":"engine.batch","params":batch});
        assert_eq!(captured("run_command", wrapped.clone()), wrapped);
    }
    let mut s = drawing();
    let r = rpc(&mut s, "tools/call", json!({"name":"run_command","arguments":{"command":"document.inspect","params":"bad"}}));
    assert_eq!(r["isError"], true);
    assert_eq!(r["content"][0]["text"], "invalid type for params");
    for args in [json!({"command":"engine.commands","params":null}), json!({"command":"engine.batch","params":{"commands":null}})] {
        assert_eq!(captured("run_command", args.clone()), args);
    }
}
#[test]
fn nesting_is_bounded_and_other_commands_are_unchanged() {
    let inner = json!({"command":"document.inspect"});
    let wrapped = json!({"command":"engine.execute","params":inner});
    assert_eq!(
        captured("run_command", wrapped),
        json!({"command":"engine.execute","params":{"command":"document.inspect","params":{"sections":["outline"]}}})
    );
    let mut too_deep = inner;
    for _ in 0..3 {
        too_deep = json!({"command":"engine.execute","params":too_deep});
    }
    assert_eq!(captured("run_command", too_deep.clone()), too_deep);
    for args in
        [json!({"command":"shape.inspect","params":{"id":1}}), json!({"command":"page.add"}), json!({"command":"unknown"}), json!({"command":17})]
    {
        assert_eq!(captured("run_command", args.clone()), args);
    }
}

#[test]
fn malformed_batches_preserve_earlier_budget_errors() {
    let mut s = drawing();
    call(&mut s, "run_command", json!({"command":"shape.data.define","params":{"id":1,"name":"Payload","label":"Payload"}}));
    call(&mut s, "run_command", json!({"command":"shape.data.set","params":{"id":1,"name":"Payload","value":"x".repeat(65530)}}));
    let legacy =
        rpc(&mut s, "tools/call", json!({"name":"run_command","arguments":{"command":"document.inspect","params":{"sections":["listing"]}}}));
    assert_eq!(legacy["isError"], true);
    let message = legacy["content"][0]["text"].as_str().unwrap();
    assert!(message.contains("65536 byte budget"));
    let expected = format!("batch command index 0 (command 1): {message}");
    for bad in [json!(null), json!({"command":"document.inspect","params":"bad"}), json!({"command":"engine.execute","params":false})] {
        let batch = json!({"commands":[{"command":"document.inspect"},bad]});
        for (name, args) in [("batch", batch.clone()), ("run_command", json!({"command":"engine.batch","params":batch}))] {
            let result = rpc(&mut s, "tools/call", json!({"name":name,"arguments":args}));
            assert_eq!(result["isError"], true);
            assert_eq!(result["content"][0]["text"], expected);
        }
    }
}
