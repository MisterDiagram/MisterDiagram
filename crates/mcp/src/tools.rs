//! Phase 0 command discovery and invocation tools.
use crate::backend::Backend;
use serde_json::{Value, json};
pub struct ToolResult(Result<Value, String>);
impl ToolResult {
    pub fn to_value(self) -> Value {
        match self.0 {
            Ok(v) => json!({"content":[{"type":"text","text":v.to_string()}],"isError":false}),
            Err(e) => json!({"content":[{"type":"text","text":e}],"isError":true}),
        }
    }
}
pub fn tool_definitions() -> Value {
    json!([
     {"name":"list_commands","description":"List available engine commands","inputSchema":{"type":"object","properties":{}}},
     {"name":"run_command","description":"Run an engine command","inputSchema":{"type":"object","properties":{"command":{"type":"string"},"params":{"type":"object"}},"required":["command"]}}
    ])
}
pub fn call_tool(b: &mut dyn Backend, name: &str, args: &Value) -> ToolResult {
    ToolResult(match name {
        "list_commands" => b.call("engine.commands", json!({})),
        "run_command" => b.call("engine.execute", args.clone()),
        _ => Err(format!("unknown tool: {name}")),
    })
}
