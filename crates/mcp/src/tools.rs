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
    ("open", "file.open", "Use this shortcut to open a drawing interactively; equivalent to run_command file.open."),
    ("save", "file.save", "Use this shortcut to save a drawing, stencil or template deliverable; equivalent to run_command file.save."),
    (
        "export",
        "file.export",
        "Use for deliverables in VSDX, PNG, SVG or PDF; equivalent to run_command file.save/file.export. Use render for previews.",
    ),
    (
        "render",
        "file.export",
        "Use for PNG, SVG or PDF previews in the render-and-look loop; equivalent to run_command file.export. Use export for deliverables.",
    ),
    ("list_commands", "engine.commands", "Use to discover command schemas before run_command; equivalent to run_command engine.commands."),
    (
        "run_command",
        "engine.execute",
        "Use for any registered command, especially operations without a dedicated shortcut; dedicated tools expose the same engine behavior.",
    ),
    (
        "batch",
        "engine.batch",
        "Use instead of repeated run_command calls for an atomic interactive edit with references and one undo step; equivalent to run_command engine.batch. Use a CLI script for hundreds of data-generated commands.",
    ),
    (
        "inspect_document",
        "document.inspect",
        "Use for a document overview with page/sections/cells filters; equivalent to run_command document.inspect. Use inspect_shape for one shape.",
    ),
    ("inspect_shape", "shape.inspect", "Use for focused shape inspection with cells/sections filters; equivalent to run_command shape.inspect."),
    ("sheet_get", "sheet.get", "Use for one evaluated ShapeSheet cell with formula/cache/units; equivalent to run_command sheet.get."),
    ("sheet_set", "sheet.set", "Use for a direct ShapeSheet cell edit; equivalent to run_command sheet.set."),
    (
        "stencil_available",
        "stencil.available",
        "Use to list .vssx filenames, folders and sizes on the search path without opening packages; equivalent to run_command stencil.available.",
    ),
    ("stencil_list", "stencil.list", "Use to discover current stencil indices before dropping/searching; equivalent to run_command stencil.list."),
    (
        "stencil_close",
        "stencil.close",
        "Use to release an external stencil; equivalent to run_command stencil.close. Later stencil indices shift down.",
    ),
    ("stencil_search", "stencil.search", "Use to find masters with query/limit filters; equivalent to run_command stencil.search."),
    (
        "drop_master",
        "master.drop",
        "Use to drop and optionally label/style one master; equivalent to run_command master.drop. Use batch for linked creation.",
    ),
    (
        "connect",
        "connect.dynamic",
        "Use to glue one end of an existing connector; equivalent to run_command connect.static/connect.dynamic. Use run_command connector.add to create and glue both ends.",
    ),
];
fn schema(name: &str, command: &str) -> Result<Value, String> {
    let spec = misterdiagram_engine::find_command(command).ok_or_else(|| format!("missing engine command {command}"))?;
    let mut schema: Value = serde_json::from_str(spec.params).map_err(|e| e.to_string())?;
    if name == "connect" {
        schema["properties"]["mode"] = json!({"type":"string","enum":["static","dynamic"],"description":"Glue mode: static connection-point glue or dynamic whole-shape glue; unitless."});
        schema["properties"]["point"] = json!({"type":"string","minLength":1,"maxLength":65536,"description":"Universal connection X-cell name on the target shape, for example Connections.X1."});
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
