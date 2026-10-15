//! The headless document command registry.

use diagramcraft_model::ShapeId;
use diagramcraft_model::Xfrm;
use serde::Serialize;
use serde_json::Value;

use crate::{EngineError, Result, Session};

pub type Run = fn(&mut Session, &Value) -> Result<Value>;
pub type Enabled = fn(&Session) -> std::result::Result<(), String>;

pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    /// Where it lives: `["Home", "Font"]` (ribbon tab and group) or `["Arrange"]` (menu).
    pub menu: &'static [&'static str],
    pub shortcut: Option<&'static str>,
    pub params: &'static str,
    pub enabled: Enabled,
    pub run: Run,
    pub journal: bool,
    pub undoable: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct CommandInfo {
    pub id: &'static str,
    pub label: &'static str,
    pub menu: Vec<&'static str>,
    pub shortcut: Option<&'static str>,
    pub params: &'static str,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

impl CommandSpec {
    pub fn info(&self, s: &Session) -> CommandInfo {
        let e = (self.enabled)(s);
        CommandInfo {
            id: self.id,
            label: self.label,
            menu: self.menu.to_vec(),
            shortcut: self.shortcut,
            params: self.params,
            enabled: e.is_ok(),
            disabled_reason: e.err(),
        }
    }
}

pub fn always(_: &Session) -> std::result::Result<(), String> {
    Ok(())
}
#[macro_export]
macro_rules! cmd {
    ($id:literal, $label:literal, [$($m:literal),*], $sc:expr, $params:literal, $en:expr, $run:expr) => {
        $crate::cmd::CommandSpec { id: $id, label: $label, menu: &[$($m),*], shortcut: $sc, params: $params, enabled: $en, run: $run, journal: true, undoable: true }
    };
    (query $id:literal, $label:literal, [$($m:literal),*], $sc:expr, $params:literal, $en:expr, $run:expr) => {
        $crate::cmd::CommandSpec { id: $id, label: $label, menu: &[$($m),*], shortcut: $sc, params: $params, enabled: $en, run: $run, journal: false, undoable: false }
    };
    (noundo $id:literal, $label:literal, [$($m:literal),*], $sc:expr, $params:literal, $en:expr, $run:expr) => {
        $crate::cmd::CommandSpec { id: $id, label: $label, menu: &[$($m),*], shortcut: $sc, params: $params, enabled: $en, run: $run, journal: true, undoable: false }
    };
}
pub use cmd;

pub fn command_specs() -> &'static [CommandSpec] {
    &[
        cmd!(noundo "file.new", "New drawing", ["File"], None,
            r#"{"type":"object","properties":{"metric":{"type":"boolean"}},"additionalProperties":false}"#, always, crate::files::new),
        cmd!(noundo "file.open", "Open drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768}},"required":["path"],"additionalProperties":false}"#, always, crate::files::open),
        cmd!(noundo "file.save", "Save drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768}},"required":["path"],"additionalProperties":false}"#, always, crate::files::save),
        cmd!(noundo "file.export", "Export drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768},"page":{"type":"integer","minimum":1,"description":"One-based page ordinal"},"scale":{"type":"number","exclusiveMinimum":0,"maximum":100}},"required":["path"],"additionalProperties":false}"#, always, crate::files::export),
        cmd!(noundo "edit.undo", "Undo", ["Edit"], None,
            r#"{"type":"object","additionalProperties":false}"#, always, Session::undo),
        cmd!(noundo "edit.redo", "Redo", ["Edit"], None,
            r#"{"type":"object","additionalProperties":false}"#, always, Session::redo),
        cmd!(
            "page.add",
            "page.add",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"name":{"type":"string","minLength":1,"maxLength":65536},"metric":{"type":"boolean"}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::pages::add
        ),
        cmd!(
            "page.delete",
            "page.delete",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000}},"required":[],"additionalProperties":false}"#,
            always,
            crate::pages::delete
        ),
        cmd!(
            "page.rename",
            "page.rename",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"name":{"type":"string","minLength":1,"maxLength":65536}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::pages::rename
        ),
        cmd!(
            "page.reorder",
            "page.reorder",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"index":{"type":"integer","minimum":1,"maximum":100000}},"required":["index"],"additionalProperties":false}"#,
            always,
            crate::pages::reorder
        ),
        cmd!(
            "page.size",
            "page.size",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["width","height"],"additionalProperties":false}"#,
            always,
            crate::pages::size
        ),
        cmd!(
            "page.scale",
            "page.scale",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"page_scale":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"drawing_scale":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["page_scale","drawing_scale"],"additionalProperties":false}"#,
            always,
            crate::pages::scale
        ),
        cmd!(
            "page.background",
            "page.background",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"background":{"type":["integer","null"]}},"required":["background"],"additionalProperties":false}"#,
            always,
            crate::pages::background
        ),
        cmd!(
            "layer.add",
            "layer.add",
            ["Layer"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"name":{"type":"string","minLength":1,"maxLength":65536}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::pages::layer_add
        ),
        cmd!(
            "layer.delete",
            "layer.delete",
            ["Layer"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"layer":{"type":"integer","minimum":0,"maximum":4294967294}},"required":["layer"],"additionalProperties":false}"#,
            always,
            crate::pages::layer_delete
        ),
        cmd!(
            "layer.rename",
            "layer.rename",
            ["Layer"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"layer":{"type":"integer","minimum":0,"maximum":4294967294},"name":{"type":"string","minLength":1,"maxLength":65536}},"required":["layer","name"],"additionalProperties":false}"#,
            always,
            crate::pages::layer_rename
        ),
        cmd!(
            "layer.assign",
            "layer.assign",
            ["Layer"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"layers":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"maxItems":16384}},"required":["id","layers"],"additionalProperties":false}"#,
            always,
            crate::pages::layer_assign
        ),
        cmd!(
            "shape.draw.rect",
            "shape.draw.rect",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"x":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"y":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["x","y","width","height"],"additionalProperties":false}"#,
            always,
            crate::shapes::rect
        ),
        cmd!(
            "shape.draw.ellipse",
            "shape.draw.ellipse",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"x":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"y":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["x","y","width","height"],"additionalProperties":false}"#,
            always,
            crate::shapes::ellipse
        ),
        cmd!(
            "shape.draw.line",
            "shape.draw.line",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"x1":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"y1":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"x2":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"y2":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["x1","y1","x2","y2"],"additionalProperties":false}"#,
            always,
            crate::shapes::line
        ),
        cmd!(
            "shape.move",
            "shape.move",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"x":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"y":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["id","x","y"],"additionalProperties":false}"#,
            always,
            crate::shapes::move_shape
        ),
        cmd!(
            "shape.resize",
            "shape.resize",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["id","width","height"],"additionalProperties":false}"#,
            always,
            crate::shapes::resize
        ),
        cmd!(
            "shape.rotate",
            "shape.rotate",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"angle":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["id","angle"],"additionalProperties":false}"#,
            always,
            crate::shapes::rotate
        ),
        cmd!(
            "shape.flip",
            "shape.flip",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"axis":{"type":"string","enum":["x","y"]}},"required":["id","axis"],"additionalProperties":false}"#,
            always,
            crate::shapes::flip
        ),
        cmd!(
            "shape.delete",
            "shape.delete",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::shapes::delete
        ),
        cmd!(
            "shape.order",
            "shape.order",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"order":{"type":"string","enum":["front","back","forward","backward"]}},"required":["id","order"],"additionalProperties":false}"#,
            always,
            crate::shapes::order
        ),
        cmd!(query "sheet.get", "sheet.get", ["Sheet"], None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"scope":{"type":"string","enum":["document","page","shape"]},"id":{"type":"integer","minimum":0,"maximum":4294967294},"name":{"type":"string","minLength":1,"maxLength":65536}},"required":["name"],"additionalProperties":false}"#, always, crate::sheet::get),
        cmd!(
            "sheet.set",
            "sheet.set",
            ["Sheet"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"scope":{"type":"string","enum":["document","page","shape"]},"id":{"type":"integer","minimum":0,"maximum":4294967294},"name":{"type":"string","minLength":1,"maxLength":65536},"value":{"type":["string","number"]},"formula":{"type":"string","minLength":1,"maxLength":65536},"units":{"type":"string","minLength":1,"maxLength":65536},"force":{"type":"boolean"}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::sheet::set_cell
        ),
        cmd!(
            "sheet.section.add",
            "sheet.section.add",
            ["Sheet"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"scope":{"type":"string","enum":["document","page","shape"]},"id":{"type":"integer","minimum":0,"maximum":4294967294},"section":{"type":"string","minLength":1,"maxLength":65536},"section_index":{"type":"integer","minimum":0,"maximum":4294967294}},"required":["section"],"additionalProperties":false}"#,
            always,
            crate::sheet::add_section
        ),
        cmd!(
            "sheet.row.add",
            "sheet.row.add",
            ["Sheet"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"scope":{"type":"string","enum":["document","page","shape"]},"id":{"type":"integer","minimum":0,"maximum":4294967294},"section":{"type":"string","minLength":1,"maxLength":65536},"section_index":{"type":"integer","minimum":0,"maximum":4294967294},"row_name":{"type":"string","minLength":1,"maxLength":65536},"index":{"type":"integer","minimum":0,"maximum":4294967294},"type":{"type":"string","minLength":1,"maxLength":65536}},"required":["section"],"additionalProperties":false}"#,
            always,
            crate::sheet::add_row
        ),
        cmd!(
            "shape.text",
            "shape.text",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"text":{"type":"string","maxLength":65536},"paragraphs":{"type":"array","maxItems":1024}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::content::shape_text
        ),
        cmd!(
            "shape.data.define",
            "shape.data.define",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"name":{"type":"string","minLength":1,"maxLength":65536},"label":{"type":"string"},"type":{"type":"integer","minimum":0,"maximum":4294967294},"format":{"type":"string"},"invisible":{"type":"boolean"},"sort_key":{"type":"string"}},"required":["id","name"],"additionalProperties":false}"#,
            always,
            crate::content::define
        ),
        cmd!(
            "shape.data.set",
            "shape.data.set",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"name":{"type":"string","minLength":1,"maxLength":65536},"value":{}},"required":["id","name","value"],"additionalProperties":false}"#,
            always,
            crate::content::set_data
        ),
        cmd!(
            "shape.hyperlink",
            "shape.hyperlink",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"name":{"type":"string","minLength":1,"maxLength":65536},"address":{"type":"string"},"sub_address":{"type":"string"},"description":{"type":"string"},"new_window":{"type":"boolean"}},"required":["id","name"],"additionalProperties":false}"#,
            always,
            crate::content::hyperlink
        ),
        cmd!(
            "format.fill",
            "format.fill",
            ["Format"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"color":{"type":"string","minLength":1,"maxLength":65536},"pattern":{"type":"integer","minimum":0,"maximum":4294967294},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::content::fill
        ),
        cmd!(
            "format.line",
            "format.line",
            ["Format"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"color":{"type":"string","minLength":1,"maxLength":65536},"pattern":{"type":"integer","minimum":0,"maximum":4294967294},"weight":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"begin_arrow":{"type":"integer","minimum":0,"maximum":4294967294},"end_arrow":{"type":"integer","minimum":0,"maximum":4294967294}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::content::line
        ),
        cmd!(
            "format.text",
            "format.text",
            ["Format"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"font":{"type":"string","minLength":1,"maxLength":65536},"size":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"color":{"type":"string","minLength":1,"maxLength":65536},"bold":{"type":"boolean"},"italic":{"type":"boolean"},"underline":{"type":"boolean"},"align":{"type":"integer","minimum":0,"maximum":4294967294}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::content::format_text
        ),
        cmd!(
            "shape.duplicate",
            "shape.duplicate",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294},"dx":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"dy":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::arrange::duplicate
        ),
        cmd!(
            "shape.group",
            "shape.group",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"ids":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"minItems":2,"maxItems":10000}},"required":["ids"],"additionalProperties":false}"#,
            always,
            crate::arrange::group
        ),
        cmd!(
            "shape.ungroup",
            "shape.ungroup",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"id":{"type":"integer","minimum":0,"maximum":4294967294}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::arrange::ungroup
        ),
        cmd!(
            "arrange.align",
            "arrange.align",
            ["Arrange"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"ids":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"maxItems":10000},"mode":{"type":"string","enum":["left","right","center","bottom","top","middle"]}},"required":["ids","mode"],"additionalProperties":false}"#,
            always,
            crate::arrange::align
        ),
        cmd!(
            "arrange.distribute",
            "arrange.distribute",
            ["Arrange"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"ids":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"maxItems":10000},"axis":{"type":"string","enum":["x","y"]}},"required":["ids","axis"],"additionalProperties":false}"#,
            always,
            crate::arrange::distribute
        ),
        cmd!(
            "arrange.same_size",
            "arrange.same_size",
            ["Arrange"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000},"ids":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"maxItems":10000},"axis":{"type":"string","enum":["width","height","both"]}},"required":["ids"],"additionalProperties":false}"#,
            always,
            crate::arrange::same_size
        ),
        cmd!(noundo "stencil.open", "stencil.open", ["Stencils"], None, r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":65536}},"required":["path"],"additionalProperties":false}"#, always, crate::stencils::open),
        cmd!(query "stencil.search", "stencil.search", ["Stencils"], None, r#"{"type":"object","properties":{"stencil":{"type":"integer","minimum":1,"maximum":64},"query":{"type":"string","minLength":1,"maxLength":65536},"limit":{"type":"integer","minimum":0,"maximum":4294967294}},"required":["query"],"additionalProperties":false}"#, always, crate::stencils::search),
        cmd!(query "stencil.masters", "stencil.masters", ["Stencils"], None, r#"{"type":"object","properties":{"stencil":{"type":"integer","minimum":1,"maximum":64},"limit":{"type":"integer","minimum":0,"maximum":4294967294}},"required":[],"additionalProperties":false}"#, always, crate::stencils::masters),
        cmd!(
            "master.drop",
            "master.drop",
            ["Stencils"],
            None,
            r#"{"type":"object","properties":{"stencil":{"type":"integer","minimum":1,"maximum":64},"master":{"type":["integer","string"]},"page":{"type":"integer","minimum":1},"x":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"y":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0}},"required":["master","x","y"],"additionalProperties":false}"#,
            always,
            crate::masters::drop_master
        ),
        cmd!(
            "master.new_from_shape",
            "master.new_from_shape",
            ["Stencils"],
            None,
            r#"{"type":"object","properties":{"id":{"type":"integer","minimum":0,"maximum":4294967294},"page":{"type":"integer","minimum":1},"name":{"type":"string","minLength":1,"maxLength":65536}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::masters::new_from_shape
        ),
    ]
}

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    command_specs().iter().find(|c| c.id == id)
}

// ---------- param helpers ----------

pub fn bad(cmd: &str, msg: impl Into<String>) -> EngineError {
    EngineError::BadParams { cmd: cmd.into(), msg: msg.into() }
}
pub fn f64_param(p: &Value, key: &str) -> Option<f64> {
    p.get(key).and_then(Value::as_f64).filter(|v| v.is_finite())
}
pub fn f64_or(p: &Value, key: &str, default: f64) -> f64 {
    f64_param(p, key).unwrap_or(default)
}
pub fn bool_param(p: &Value, key: &str) -> Option<bool> {
    p.get(key).and_then(Value::as_bool)
}
pub fn bool_or(p: &Value, key: &str, default: bool) -> bool {
    bool_param(p, key).unwrap_or(default)
}
pub fn str_param<'a>(p: &'a Value, key: &str) -> Option<&'a str> {
    p.get(key).and_then(Value::as_str)
}
pub fn usize_param(p: &Value, key: &str) -> Option<usize> {
    p.get(key).and_then(Value::as_u64).and_then(|v| usize::try_from(v).ok())
}
pub fn id_param(p: &Value, key: &str) -> Option<ShapeId> {
    p.get(key).and_then(Value::as_u64).and_then(|v| u32::try_from(v).ok()).map(ShapeId)
}
pub fn ids_param(p: &Value, key: &str) -> Option<Vec<ShapeId>> {
    p.get(key).and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_u64).filter_map(|v| u32::try_from(v).ok()).map(ShapeId).collect())
}
/// `[x, y, w, h]` in inches.
pub fn rect_param(p: &Value, key: &str) -> Option<Xfrm> {
    let a = p.get(key)?.as_array()?;
    let f = |i: usize| a.get(i).and_then(Value::as_f64).filter(|v| v.is_finite());
    Some(Xfrm::new(f(0)?, f(1)?, f(2)?.abs(), f(3)?.abs()))
}
pub fn with_param(p: &Value, key: &str, v: Value) -> Value {
    let mut m = p.as_object().cloned().unwrap_or_default();
    m.insert(key.to_string(), v);
    Value::Object(m)
}
