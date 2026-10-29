//! CLI commands share the engine's session, schema validation and rollback boundary.
use misterdiagram_engine::{MemoryLimits, Session};
use serde_json::{Value, json};
use std::{
    ffi::OsString,
    io::{Read, Write},
};
fn output(out: &mut impl Write, value: &impl serde::Serialize) -> Result<(), String> {
    serde_json::to_writer(&mut *out, value).map_err(|e| e.to_string())?;
    writeln!(out).map_err(|e| e.to_string())
}
fn text(arg: &OsString) -> Result<&str, String> {
    arg.to_str().ok_or_else(|| "command argument must be UTF-8".into())
}
/// List schemas directly from the engine registry.
pub fn commands(args: &[OsString], out: &mut impl Write) -> Result<(), String> {
    if args.len() != 1 {
        return Err("usage: commands".into());
    }
    output(out, &Session::new().commands())
}
/// Describe one registered command and its JSON params schema.
pub fn describe(args: &[OsString], out: &mut impl Write) -> Result<(), String> {
    if args.len() != 2 {
        return Err("usage: describe ID".into());
    }
    let id = text(args.get(1).ok_or("missing command ID")?)?;
    let session = Session::new();
    let spec = misterdiagram_engine::cmd::find_command(id).ok_or_else(|| format!("unknown command {id}"))?;
    output(out, &spec.info(&session))
}
/// Execute a bounded sequence in one session, with optional open and final save.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.application.beginundoscope
pub fn run(args: &[OsString], out: &mut impl Write, memory: MemoryLimits, stencil_path: Vec<std::path::PathBuf>) -> Result<(), String> {
    let mut open = None;
    let mut save = None;
    let mut commands = Vec::<(&str, Value)>::new();
    let mut index = 1usize;
    while let Some(arg) = args.get(index) {
        let value = args.get(index + 1).ok_or("usage: run [--open FILE] [--cmd ID=JSON | --batch FILE] ... [--save FILE]")?;
        match text(arg)? {
            "--open" if open.is_none() => open = Some(text(value)?),
            "--save" if save.is_none() => save = Some(text(value)?),
            "--batch" => {
                if commands.len() >= 1024 {
                    return Err("command count limit".into());
                }
                let file = std::fs::File::open(value).map_err(|e| format!("batch input: {e}"))?;
                let mut data = Vec::new();
                file.take(1024 * 1024 + 1).read_to_end(&mut data).map_err(|e| format!("batch input: {e}"))?;
                if data.len() > 1024 * 1024 {
                    return Err("batch file exceeds 1 MiB limit".into());
                }
                let params: Value = serde_json::from_slice(&data).map_err(|e| format!("invalid batch JSON: {e}"))?;
                commands.push(("engine.batch", params));
            }
            "--cmd" => {
                if commands.len() >= 1024 {
                    return Err("command count limit".into());
                }
                let command = text(value)?;
                if command.len() > 1024 * 1024 {
                    return Err("command byte limit".into());
                }
                let (id, params) = command.split_once('=').ok_or("command must be ID=JSON")?;
                if misterdiagram_engine::cmd::find_command(id).is_none() {
                    return Err(format!("unknown command {id}"));
                }
                let params = serde_json::from_str(params).map_err(|e| format!("invalid command JSON: {e}"))?;
                commands.push((id, params));
            }
            _ => {
                return Err(
                    "usage: run [--open FILE] [--cmd ID=JSON | --batch FILE] ... [--save FILE] (options may occur once except --cmd/--batch)".into(),
                );
            }
        }
        index = index.checked_add(2).ok_or("argument count overflow")?;
    }
    if open.is_none() && save.is_none() && commands.is_empty() {
        return Err("usage: run [--open FILE] [--cmd ID=JSON | --batch FILE] ... [--save FILE]".into());
    }
    let mut session = Session::with_memory_limits(memory);
    session.set_stencil_path(stencil_path).map_err(|e| e.to_string())?;
    if let Some(path) = open {
        session.execute("file.open", &json!({"path":path})).map_err(|e| e.to_string())?;
    }
    for (id, params) in commands {
        let value = session.execute(id, &params).map_err(|e| e.to_string())?;
        output(out, &json!({"command":id,"result":value}))?;
    }
    if let Some(path) = save {
        let value = session.execute("file.save", &json!({"path":path})).map_err(|e| e.to_string())?;
        output(out, &json!({"command":"file.save","result":value}))?;
    }
    Ok(())
}
/// Convert a document through the same file.open/save/export commands as scripting.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.document.saveas
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
pub fn convert(args: &[OsString], out: &mut impl Write, memory: MemoryLimits) -> Result<(), String> {
    let flags = args.iter().skip(1).filter(|a| *a == "--text-outlines").count();
    if flags > 1 || args.iter().skip(1).any(|a| a.to_str().is_some_and(|s| s.starts_with("--") && s != "--text-outlines")) {
        return Err("usage: convert IN OUT [--text-outlines]".into());
    }
    let text_outlines = flags == 1;
    let positional: Vec<_> = args.iter().skip(1).filter(|a| *a != "--text-outlines").collect();
    let [input, output_path] = positional.as_slice() else {
        return Err("usage: convert IN OUT [--text-outlines]".into());
    };
    let input = text(input)?;
    let output_path = text(output_path)?;
    let extension = std::path::Path::new(output_path).extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let command = match extension.as_str() {
        "vsdx" | "vssx" | "vstx" => "file.save",
        "png" | "svg" | "pdf" => "file.export",
        _ => return Err("convert output must be vsdx, vssx, vstx, png, svg or pdf".into()),
    };
    let mut session = Session::with_memory_limits(memory);
    session.execute("file.open", &json!({"path":input})).map_err(|e| e.to_string())?;
    let mut params = json!({"path":output_path});
    if command == "file.export" {
        params["text_outlines"] = json!(text_outlines);
    }
    let value = session.execute(command, &params).map_err(|e| e.to_string())?;
    output(out, &value)
}
