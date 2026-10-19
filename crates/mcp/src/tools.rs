//! MCP tools backed by the engine's public command schemas.
use crate::backend::Backend;
use serde_json::{Value, json};
const TOOL_RESULT_BYTES: usize = 1024 * 1024;
pub struct ToolResult(Result<Value, String>);
struct ResultBudget(usize);
impl std::io::Write for ResultBudget {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0 = self.0.checked_sub(bytes.len()).ok_or_else(|| std::io::Error::other("tool result budget"))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn error_result(message: &str) -> Value {
    json!({"content":[{"type":"text","text":message}],"isError":true})
}
impl ToolResult {
    pub fn to_value(self) -> Value {
        let result = match self.0 {
            Ok(v) => {
                if serde_json::to_writer(ResultBudget(TOOL_RESULT_BYTES), &v).is_err() {
                    return error_result(
                        "tool result exceeds 1048576 byte budget; use cells/sections/page inspection filters, query/limit for stencil search, or fewer batch commands",
                    );
                }
                json!({"content":[{"type":"text","text":v.to_string()}],"isError":false})
            }
            Err(e) => error_result(&e),
        };
        if serde_json::to_writer(ResultBudget(TOOL_RESULT_BYTES), &result).is_err() {
            return error_result(
                "tool result exceeds 1048576 byte budget; use cells/sections/page inspection filters, query/limit for stencil search, or fewer batch commands",
            );
        }
        result
    }
}
const TOOLS: &[(&str, &str, &str)] = &[
    ("open", "file.open", "Open a drawing"),
    ("save", "file.save", "Save a drawing, stencil or template"),
    ("export", "file.export", "Export VSDX, SVG, PNG or PDF (chosen by path extension)"),
    ("render", "file.export", "Render PNG, SVG or PDF"),
    ("list_commands", "engine.commands", "List engine commands and schemas"),
    ("run_command", "engine.execute", "Execute one strict engine command"),
    ("batch", "engine.batch", "Run strict commands atomically as one undo step"),
    ("inspect_document", "document.inspect", "Inspect pages, layers, shapes and Connects"),
    ("inspect_shape", "shape.inspect", "Inspect evaluated shape cells, text, data and glue"),
    ("sheet_get", "sheet.get", "Read an evaluated ShapeSheet cell"),
    ("sheet_set", "sheet.set", "Set a ShapeSheet cell"),
    ("stencil_list", "stencil.list", "List the document stencil and all open external stencils"),
    ("stencil_close", "stencil.close", "Close an external stencil; later stencil indices shift down"),
    ("stencil_search", "stencil.search", "Search open stencils and the document stencil"),
    ("drop_master", "master.drop", "Drop a master instance"),
    ("connect", "connect.dynamic", "Glue a connector end statically or dynamically"),
];
fn schema(name: &str, command: &str) -> Result<Value, String> {
    if name == "batch" {
        return Ok(json!({"type":"object","properties":{"commands":{"type":"array","minItems":1,"maxItems":1024,
            "items":{"type":"object","properties":{"command":{"type":"string","minLength":1,"maxLength":256},"params":{"type":"object"}},"required":["command"],"additionalProperties":false}}},"required":["commands"],"additionalProperties":false}));
    }
    let spec = misterdiagram_engine::find_command(command).ok_or_else(|| format!("missing engine command {command}"))?;
    let mut schema: Value = serde_json::from_str(spec.params).map_err(|e| e.to_string())?;
    if name == "connect" {
        schema["properties"]["mode"] = json!({"type":"string","enum":["static","dynamic"]});
        schema["properties"]["point"] = json!({"type":"string","minLength":1,"maxLength":65536});
    }
    Ok(schema)
}
pub fn tool_definitions() -> Result<Value, String> {
    TOOLS
        .iter()
        .map(|(name, command, description)| Ok(json!({"name":name,"description":description,"inputSchema":schema(name,command)?})))
        .collect::<Result<Vec<_>, String>>()
        .map(Value::Array)
}
pub fn call_tool(b: &mut dyn Backend, name: &str, args: &Value) -> ToolResult {
    ToolResult((|| {
        let (_, command, _) = TOOLS.iter().find(|(n, _, _)| *n == name).ok_or_else(|| format!("unknown tool: {name}"))?;
        if !args.is_object() {
            return Err("tool arguments must be an object".into());
        }
        let mut command = *command;
        let mut params = args.clone();
        if name == "connect" {
            let mode = args
                .get("mode")
                .map(|v| v.as_str().ok_or("mode must be static or dynamic"))
                .transpose()?
                .unwrap_or(if args.get("point").is_some() { "static" } else { "dynamic" });
            if mode == "static" {
                command = "connect.static";
            } else if mode != "dynamic" {
                return Err("mode must be static or dynamic".into());
            }
            if let Some(p) = params.as_object_mut() {
                p.remove("mode");
            }
        }
        if name == "export"
            && args
                .get("path")
                .and_then(Value::as_str)
                .and_then(|s| std::path::Path::new(s).extension())
                .is_some_and(|s| s.eq_ignore_ascii_case("vsdx"))
        {
            command = "file.save";
        }
        b.call(command, params)
    })())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oversized_tool_results_fail_instead_of_truncating() {
        for v in [json!("x".repeat(TOOL_RESULT_BYTES)), json!("\"".repeat(TOOL_RESULT_BYTES / 3))] {
            let result = ToolResult(Ok(v)).to_value();
            assert_eq!(result["isError"], true);
            let message = result["content"][0]["text"].as_str().unwrap();
            assert!(message.contains("budget") && message.contains("sections"), "{message}");
            assert!(result.to_string().len() < 1024);
        }
        let result = ToolResult(Ok(json!({"kept":"complete"}))).to_value();
        assert_eq!(result["isError"], false);
        assert_eq!(serde_json::from_str::<Value>(result["content"][0]["text"].as_str().unwrap()).unwrap(), json!({"kept":"complete"}));
    }
}
