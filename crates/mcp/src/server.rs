//! JSON-RPC 2.0 framing and the MCP lifecycle / tools / resources methods.

use std::io::{BufRead, Read, Write};
const MAX_MESSAGE: u64 = 4 * 1024 * 1024;

use serde_json::{Value, json};

use crate::backend::Backend;
use crate::tools::{call_tool, tool_definitions};

/// The MCP revision we implement.
pub const PROTOCOL_VERSION: &str = "2025-06-18";
const SUPPORTED_VERSIONS: &[&str] = &[PROTOCOL_VERSION, "2025-03-26", "2024-11-05"];

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;
const INTERNAL_ERROR: i64 = -32603;
const RESOURCE_NOT_FOUND: i64 = -32002;

const INSTRUCTIONS: &str =
    "MisterDiagram headless Visio core. Strict transactional authoring, inspection and export. Page coordinates are inches, y-up.";

/// Resource URIs.
pub const DOC_URI: &str = "misterdiagram://document";
pub const COMMANDS_URI: &str = "misterdiagram://commands";

/// An MCP server bound to one backend.
pub struct Server {
    backend: Box<dyn Backend>,
    initialized: bool,
}

fn response(id: Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

fn error(id: Value, code: i64, message: impl Into<String>) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message.into()}})
}

fn response_limit() -> String {
    error(Value::Null, INTERNAL_ERROR, "MCP response exceeds cap").to_string()
}
struct ReplyBuffer(Vec<u8>);
impl Write for ReplyBuffer {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        // Reserve one byte for the newline framing emitted by serve.
        if self.0.len().saturating_add(bytes.len()) >= MAX_MESSAGE as usize {
            return Err(std::io::Error::other("MCP response exceeds cap"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Server {
    pub fn new(backend: Box<dyn Backend>) -> Self {
        Self { backend, initialized: false }
    }

    pub fn backend(&mut self) -> &mut dyn Backend {
        self.backend.as_mut()
    }

    /// Whether the client has sent `notifications/initialized`.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Serve newline-delimited JSON-RPC until `input` closes. Logs go to stderr only (stdout is
    /// the protocol stream).
    pub fn serve(&mut self, mut input: impl BufRead, mut output: impl Write) -> std::io::Result<()> {
        loop {
            let mut line = String::new();
            if input.by_ref().take(MAX_MESSAGE + 1).read_line(&mut line)? == 0 {
                break;
            }
            if line.len() as u64 > MAX_MESSAGE {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "MCP message exceeds cap"));
            }
            if let Some(reply) = self.handle_line(&line) {
                output.write_all(reply.as_bytes())?;
                output.write_all(b"\n")?;
                output.flush()?;
            }
        }
        Ok(())
    }

    /// Handle one line; returns the reply line (None for notifications and blank lines).
    pub fn handle_line(&mut self, line: &str) -> Option<String> {
        if line.len() as u64 > MAX_MESSAGE {
            return Some(error(Value::Null, PARSE_ERROR, "MCP message exceeds cap").to_string());
        }
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        match serde_json::from_str::<Value>(line) {
            Ok(Value::Array(batch)) => {
                // Legacy JSON-RPC batches share the same whole-message response cap.
                if batch.is_empty() {
                    return Some(error(Value::Null, INVALID_REQUEST, "empty batch").to_string());
                }
                let mut replies = ReplyBuffer(Vec::new());
                if replies.write_all(b"[").is_err() {
                    return Some(response_limit());
                }
                let mut count = 0usize;
                for message in batch {
                    if let Some(reply) = self.handle(message) {
                        if count > 0 && replies.write_all(b",").is_err() {
                            return Some(response_limit());
                        }
                        if serde_json::to_writer(&mut replies, &reply).is_err() {
                            return Some(response_limit());
                        }
                        count += 1;
                    }
                }
                if count == 0 {
                    return None;
                }
                if replies.write_all(b"]").is_err() {
                    return Some(response_limit());
                }
                Some(String::from_utf8(replies.0).unwrap_or_else(|_| response_limit()))
            }
            Ok(msg) => self.handle(msg).map(|reply| {
                let mut bytes = ReplyBuffer(Vec::new());
                if serde_json::to_writer(&mut bytes, &reply).is_err() {
                    return response_limit();
                }
                String::from_utf8(bytes.0).unwrap_or_else(|_| response_limit())
            }),
            Err(e) => Some(error(Value::Null, PARSE_ERROR, format!("parse error: {e}")).to_string()),
        }
    }

    /// Handle one JSON-RPC message; `None` for notifications and responses.
    pub fn handle(&mut self, msg: Value) -> Option<Value> {
        let Value::Object(o) = &msg else { return Some(error(Value::Null, INVALID_REQUEST, "message must be an object")) };
        let id = o.get("id").cloned();
        let Some(method) = o.get("method").and_then(Value::as_str) else {
            // A response to a server→client request (we send none): ignore. Anything else is invalid.
            if o.contains_key("result") || o.contains_key("error") {
                return None;
            }
            return Some(error(id.unwrap_or(Value::Null), INVALID_REQUEST, "missing `method`"));
        };
        let params = o.get("params").cloned().unwrap_or(Value::Null);
        let Some(id) = id else {
            self.notification(method);
            return None;
        };
        if !(id.is_string() || id.is_number()) {
            return Some(error(Value::Null, INVALID_REQUEST, "`id` must be a string or number"));
        }
        Some(match self.request(method, &params) {
            Ok(r) => response(id, r),
            Err((code, m)) => error(id, code, m),
        })
    }

    fn notification(&mut self, method: &str) {
        match method {
            "notifications/initialized" => self.initialized = true,
            "notifications/cancelled" | "notifications/progress" | "notifications/roots/list_changed" => {}
            other => log::debug!("ignoring notification {other}"),
        }
    }

    fn request(&mut self, method: &str, params: &Value) -> Result<Value, (i64, String)> {
        match method {
            "initialize" => {
                let asked = params.get("protocolVersion").and_then(Value::as_str).unwrap_or(PROTOCOL_VERSION);
                let version = if SUPPORTED_VERSIONS.contains(&asked) { asked } else { PROTOCOL_VERSION };
                Ok(json!({
                    "protocolVersion": version,
                    "capabilities": {"tools": {}, "resources": {}},
                    "serverInfo": {"name": "misterdiagram", "version": env!("CARGO_PKG_VERSION")},
                    "instructions": format!("{INSTRUCTIONS} Backend: {}", self.backend.describe()),
                }))
            }
            "ping" => Ok(json!({})),
            "tools/list" => Ok(json!({"tools": tool_definitions().map_err(|e| (INTERNAL_ERROR,e))?})),
            "tools/call" => {
                let name = params.get("name").and_then(Value::as_str).ok_or((INVALID_PARAMS, "missing tool `name`".to_string()))?;
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                Ok(call_tool(self.backend.as_mut(), name, &args).to_value())
            }
            "resources/list" => Ok(json!({"resources": [
                {"uri": DOC_URI, "name": "document", "title": "Active document", "description": "Pages, layers, evaluated shapes and connections", "mimeType": "application/json"},
                {"uri": COMMANDS_URI, "name": "commands", "title": "Commands", "description": "Every command: id, label, menu path, shortcut, params, enabled (engine.commands)", "mimeType": "application/json"},
            ]})),
            "resources/templates/list" => Ok(json!({"resourceTemplates": []})),
            "resources/read" => {
                let uri = params.get("uri").and_then(Value::as_str).ok_or((INVALID_PARAMS, "missing `uri`".to_string()))?;
                let v = match uri {
                    DOC_URI => self.backend.call("document.inspect", json!({})),
                    COMMANDS_URI => self.backend.call("engine.commands", json!({})),
                    _ => return Err((RESOURCE_NOT_FOUND, format!("resource not found: {uri}"))),
                }
                .map_err(|e| (INTERNAL_ERROR, e))?;
                Ok(json!({"contents": [{"uri": uri, "mimeType": "application/json", "text": serde_json::to_string_pretty(&v).unwrap_or_default()}]}))
            }
            other => Err((METHOD_NOT_FOUND, format!("method not found: {other}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headless::Headless;
    #[test]
    fn hostile_json_and_bounded_framing() {
        let mut s = Server::new(Box::new(Headless::new()));
        for text in ["{", "[]", "42", "{\"id\":null,\"method\":\"ping\"}"] {
            assert!(s.handle_line(text).is_some());
        }
        assert!(s.handle_line(&"[".repeat(200)).is_some());
        assert!(s.serve(std::io::Cursor::new(vec![b'x'; MAX_MESSAGE as usize + 1]), Vec::new()).is_err());
    }
    #[test]
    fn aggregate_and_escaped_responses_obey_the_framing_cap() {
        let mut s = Server::new(Box::new(Headless::new()));
        let requests = (0..200)
            .map(|id| json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{"name":"list_commands","arguments":{}}}))
            .collect::<Vec<_>>();
        let reply = s.handle_line(&serde_json::to_string(&requests).unwrap()).unwrap();
        assert!(reply.len() < MAX_MESSAGE as usize);
        assert!(reply.contains("MCP response exceeds cap"));
        // Escaped IDs can make the response larger than its already bounded input.
        let request = json!({"id":"x".repeat(MAX_MESSAGE as usize-100),"method":"tools/list"});
        let reply = s.handle_line(&request.to_string()).unwrap();
        assert!(reply.len() < MAX_MESSAGE as usize);
        assert!(reply.contains("MCP response exceeds cap"));
    }
    #[test]
    fn sixteen_headless_tools() {
        let mut s = Server::new(Box::new(Headless::new()));
        let v = s.handle(json!({"id":1,"method":"tools/list"})).unwrap();
        assert_eq!(v["result"]["tools"].as_array().unwrap().len(), 16);
    }
}
