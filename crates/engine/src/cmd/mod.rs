//! The headless document command registry.

use misterdiagram_model::ShapeId;
use misterdiagram_model::Xfrm;
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
    #[serde(serialize_with = "serialize_schema")]
    pub params: &'static str,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

fn serialize_schema<S: serde::Serializer>(schema: &&str, serializer: S) -> std::result::Result<S::Ok, S::Error> {
    let value: Value = serde_json::from_str(schema).map_err(serde::ser::Error::custom)?;
    value.serialize(serializer)
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
        cmd!(query "layout.check", "Check topology", ["Layout"], None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"root":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Root shape ID on the selected page; batch accepts $name."},"fit_page":{"type":"boolean","description":"Resize or check page bounds for equal fitted margins around the selected tree."}},"additionalProperties":false}"#,
            always, crate::layout::check),
        cmd!(
            "layout.tree",
            "Arrange rooted tree",
            ["Layout"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"root":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Root shape ID on the selected page; batch accepts $name."},"edges":{"type":"array","maxItems":9999,"items":{"type":"array","minItems":2,"maxItems":2,"items":{"type":"integer","minimum":0,"maximum":4294967294}},"description":"Ordered [parent, child] shape-ID pairs on the selected page; batch accepts $name in each position."},"ids":{"type":"array","minItems":1,"maxItems":10000,"items":{"type":"integer"},"description":"Ordered page-scoped shape IDs; batch accepts $name in each entry."},"region":{"type":"object","properties":{"x":{"type":"number","description":"X coordinate of the bottom-left corner, in page inches, increasing rightward."},"y":{"type":"number","description":"Y coordinate of the bottom-left corner, in page inches, increasing upward."},"width":{"type":"number","description":"Width in internal inches along the local x axis of the shape or page."},"height":{"type":"number","description":"Height in internal inches along the local y axis of the shape or page."}},"required":["x","y","width","height"],"additionalProperties":false,"description":"Layout region in page inches, anchored at its bottom-left corner, with y increasing upward."},"gutter_x":{"type":"number","minimum":0,"maximum":1000000,"description":"Horizontal gap between alignment boxes in page inches."},"gutter_y":{"type":"number","minimum":0,"maximum":1000000,"description":"Vertical gap between alignment boxes in page inches."},"fit_page":{"type":"boolean","description":"Resize or check page bounds for equal fitted margins around the selected tree."},"margin":{"type":"number","minimum":0,"maximum":1000000,"description":"Equal page margin around fitted content, in inches; default 0.5."}},"required":["root"],"additionalProperties":false}"#,
            always,
            crate::layout::tree
        ),
        cmd!(
            "layout.rows",
            "Arrange rows",
            ["Layout"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"ids":{"type":"array","items":{"type":"integer"},"minItems":1,"maxItems":10000,"description":"Ordered page-scoped shape IDs; batch accepts $name in each entry."},"counts":{"type":"array","items":{"type":"integer","minimum":1},"minItems":1,"maxItems":10000,"description":"Positive number of shapes in each row, listed top to bottom; unitless counts."},"region":{"type":"object","properties":{"x":{"type":"number","description":"X coordinate of the bottom-left corner, in page inches, increasing rightward."},"y":{"type":"number","description":"Y coordinate of the bottom-left corner, in page inches, increasing upward."},"width":{"type":"number","description":"Width in internal inches along the local x axis of the shape or page."},"height":{"type":"number","description":"Height in internal inches along the local y axis of the shape or page."}},"required":["x","y","width","height"],"additionalProperties":false,"description":"Layout region in page inches, anchored at its bottom-left corner, with y increasing upward."},"gutter_x":{"type":"number","minimum":0,"maximum":1000000,"description":"Horizontal gap between alignment boxes in page inches."},"gutter_y":{"type":"number","minimum":0,"maximum":1000000,"description":"Vertical gap between alignment boxes in page inches."}},"required":["ids","counts"],"additionalProperties":false}"#,
            always,
            crate::layout::rows
        ),
        cmd!(
            "layout.fan",
            "Fan hub connections",
            ["Layout"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"hub":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Hub shape ID on the selected page; batch accepts $name."},"spokes":{"type":"array","items":{"type":"integer"},"minItems":1,"maxItems":10000,"description":"Ordered page-scoped spoke shape IDs; all parallel links are assigned points. Batch accepts $name."}},"required":["hub","spokes"],"additionalProperties":false}"#,
            always,
            crate::layout::fan
        ),
        cmd!(
            "connect.sides",
            "Glue facing sides",
            ["Glue"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"source":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Source shape ID on the selected page; batch accepts $name."},"target":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Target shape ID on the selected page; batch accepts $name."}},"required":["id","source","target"],"additionalProperties":false}"#,
            always,
            crate::layout::sides
        ),
        cmd!(
            "connector.add",
            "Create and glue connector",
            ["Glue"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"from":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Begin-end target shape ID on the selected page; batch accepts $name."},"to":{"type":"integer","minimum":0,"maximum":4294967294,"description":"End-end target shape ID on the selected page; batch accepts $name."},"glue":{"type":"string","enum":["static","dynamic"],"description":"Static facing-side glue by default, or dynamic whole-shape glue. Automatic parallel links use distinct static points at both ends."},"from_point":{"type":"string","minLength":1,"maxLength":256,"description":"Explicit connection X-cell name on from; requires static glue and to_point."},"to_point":{"type":"string","minLength":1,"maxLength":256,"description":"Explicit connection X-cell name on to; requires static glue and from_point."},"line":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"units":{"type":"string","enum":["IN","PT","MM","CM","FT","in","pt","mm","cm","ft"],"default":"IN","description":"Line weight unit: IN inches (default), PT points, MM millimetres, CM centimetres or FT feet."},"weight":{"description":"Line width in the selected units, inches by default; centered on the stroke path.","type":"number","minimum":-1000000.0,"maximum":1000000.0},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."},"begin_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the begin endpoint."},"end_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the end endpoint."}},"additionalProperties":false,"description":"Line formatting object, using format.line parameters without id or page."}},"required":["from","to"],"additionalProperties":false}"#,
            always,
            crate::composition::connector_command
        ),
        cmd!(query "engine.batch", "Execute an atomic command batch", ["Control"], None,
            r#"{"type":"object","properties":{"commands":{"type":"array","minItems":1,"maxItems":1024,"items":{"type":"object","properties":{"command":{"type":"string","minLength":1,"maxLength":256,"description":"Registered engine command ID, such as shape.draw.rect."},"params":{"type":"object","description":"Parameters using the selected command schema; shape IDs may use earlier batch references."},"as":{"type":"string","minLength":1,"maxLength":64,"description":"Batch-local name for this command result id, 1 to 64 ASCII letters, digits, underscore, hyphen or dot."}},"required":["command"],"additionalProperties":false},"description":"Ordered command items; one transaction and undo step, at most 1024 items."}},"required":["commands"],"additionalProperties":false}"#, always, Session::execute_batch),
        cmd!(query "engine.execute", "Execute an engine command", ["Control"], None,
            r#"{"type":"object","properties":{"command":{"type":"string","minLength":1,"maxLength":256,"description":"Registered engine command ID, such as shape.draw.rect."},"params":{"type":"object","description":"Parameters using the selected command schema; shape IDs may use earlier batch references."}},"required":["command"],"additionalProperties":false}"#, always, crate::inspect::execute),
        cmd!(query "engine.commands", "List engine commands", ["Control"], None,
            r#"{"type":"object","properties":{},"additionalProperties":false}"#, always, crate::inspect::commands),
        cmd!(query "document.inspect", "Inspect document", ["Control"], None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal to inspect; omission inspects all pages."},"sections":{"type":"array","maxItems":6,"items":{"type":"string","enum":["xform","text","connection_points","data","glue","cells"]},"description":"Inspection groups to include beyond id and bounds; an empty array requests only those two fields."},"cells":{"type":"array","maxItems":256,"items":{"type":"string","minLength":1,"maxLength":256},"description":"Requested universal ShapeSheet cell names; with no sections filter returns only id, bounds and these cells."}},"additionalProperties":false}"#, always, crate::inspect::document),
        cmd!(query "shape.inspect", "Inspect shape", ["Control"], None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967295,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"cells":{"type":"array","maxItems":256,"items":{"type":"string","minLength":1,"maxLength":256},"description":"Requested universal ShapeSheet cell names; with no sections filter returns only id, bounds and these cells."},"sections":{"type":"array","maxItems":6,"items":{"type":"string","enum":["xform","text","connection_points","data","glue","cells"]},"description":"Inspection groups to include beyond id and bounds; an empty array requests only those two fields."}},"required":["id"],"additionalProperties":false}"#, always, crate::inspect::shape),
        cmd!(noundo "file.new", "New drawing", ["File"], None,
            r#"{"type":"object","properties":{"metric":{"type":"boolean","description":"Select metric page defaults; geometry coordinates still use inches, and numeric cells use their cell-specific internal units."},"discard":{"type":"boolean","description":"Explicitly allow replacing a document with unsaved changes; default false."}},"additionalProperties":false}"#, always, crate::files::new),
        cmd!(noundo "file.open", "Open drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768,"description":"Input or output filesystem path; relative paths resolve from the process working directory."},"discard":{"type":"boolean","description":"Explicitly allow replacing a document with unsaved changes; default false."}},"required":["path"],"additionalProperties":false}"#, always, crate::files::open),
        cmd!(noundo "file.save", "Save drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768,"description":"Input or output filesystem path; relative paths resolve from the process working directory."}},"required":["path"],"additionalProperties":false}"#, always, crate::files::save),
        cmd!(noundo "file.export", "Export drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768,"description":"Input or output filesystem path; relative paths resolve from the process working directory."},"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; omitted PDF exports all foreground pages, other formats use the current page."},"scale":{"type":"number","exclusiveMinimum":0,"maximum":100,"description":"Unitless render scale multiplier; 1 is the default 96 dpi preview."}},"required":["path"],"additionalProperties":false}"#, always, crate::files::export),
        cmd!(noundo "edit.undo", "Undo", ["Edit"], None,
            r#"{"type":"object","additionalProperties":false}"#, always, Session::undo),
        cmd!(noundo "edit.redo", "Redo", ["Edit"], None,
            r#"{"type":"object","additionalProperties":false}"#, always, Session::redo),
        cmd!(
            "page.add",
            "page.add",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"Compatibility parameter; the new page is appended and becomes current regardless of this value."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal UTF-8 name for this page."},"metric":{"type":"boolean","description":"Select metric page defaults; geometry coordinates still use inches, and numeric cells use their cell-specific internal units."}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::pages::add
        ),
        cmd!(
            "page.delete",
            "page.delete",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."}},"required":[],"additionalProperties":false}"#,
            always,
            crate::pages::delete
        ),
        cmd!(
            "page.rename",
            "page.rename",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal UTF-8 name for this page."}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::pages::rename
        ),
        cmd!(
            "page.reorder",
            "page.reorder",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"index":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based destination page position; the existing page is moved to this ordinal."}},"required":["index"],"additionalProperties":false}"#,
            always,
            crate::pages::reorder
        ),
        cmd!(
            "page.size",
            "page.size",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Width in internal inches along the local x axis of the shape or page."},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Height in internal inches along the local y axis of the shape or page."}},"required":["width","height"],"additionalProperties":false}"#,
            always,
            crate::pages::size
        ),
        cmd!(
            "page.scale",
            "page.scale",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"page_scale":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"PageScale length in internal inches; must be positive."},"drawing_scale":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"DrawingScale length in internal inches; must be positive."}},"required":["page_scale","drawing_scale"],"additionalProperties":false}"#,
            always,
            crate::pages::scale
        ),
        cmd!(
            "page.background",
            "page.background",
            ["Page"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"background":{"type":["integer","null"],"description":"One-based ordinal of the background page; null clears the background; this parameter is required."}},"required":["background"],"additionalProperties":false}"#,
            always,
            crate::pages::background
        ),
        cmd!(
            "layer.add",
            "layer.add",
            ["Layer"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal UTF-8 name for this layer."}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::pages::layer_add
        ),
        cmd!(
            "layer.delete",
            "layer.delete",
            ["Layer"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"layer":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Zero-based layer row index on the selected page."}},"required":["layer"],"additionalProperties":false}"#,
            always,
            crate::pages::layer_delete
        ),
        cmd!(
            "layer.rename",
            "layer.rename",
            ["Layer"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"layer":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Zero-based layer row index on the selected page."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal UTF-8 name for this layer."}},"required":["layer","name"],"additionalProperties":false}"#,
            always,
            crate::pages::layer_rename
        ),
        cmd!(
            "layer.assign",
            "layer.assign",
            ["Layer"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"layers":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"maxItems":16384,"description":"Zero-based layer row indices on the selected page."}},"required":["id","layers"],"additionalProperties":false}"#,
            always,
            crate::pages::layer_assign
        ),
        cmd!(
            "shape.draw.rect",
            "shape.draw.rect",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"x":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"X coordinate of the bottom-left corner, in page inches, increasing rightward."},"y":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Y coordinate of the bottom-left corner, in page inches, increasing upward."},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Width in internal inches along the local x axis of the shape or page."},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Height in internal inches along the local y axis of the shape or page."},"fill":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."}},"additionalProperties":false,"description":"Fill formatting object, using format.fill parameters without id or page."},"line":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"units":{"type":"string","enum":["IN","PT","MM","CM","FT","in","pt","mm","cm","ft"],"default":"IN","description":"Line weight unit: IN inches (default), PT points, MM millimetres, CM centimetres or FT feet."},"weight":{"description":"Line width in the selected units, inches by default; centered on the stroke path.","type":"number","minimum":-1000000.0,"maximum":1000000.0},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."},"begin_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the begin endpoint."},"end_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the end endpoint."}},"additionalProperties":false,"description":"Line formatting object, using format.line parameters without id or page."},"font":{"type":"object","properties":{"font":{"type":"string","minLength":1,"maxLength":65536,"description":"Font family name, resolved through the document FaceNames table; literal UTF-8 text."},"size":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Character size in points (1 point = 1/72 inch)."},"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"bold":{"type":"boolean","description":"Set or clear bold character formatting; boolean."},"italic":{"type":"boolean","description":"Set or clear italic character formatting; boolean."},"underline":{"type":"boolean","description":"Set or clear underlined character formatting; boolean."},"align":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless horizontal text alignment: 0 left, 1 center, 2 right, 3 justify, 4 force justify."}},"additionalProperties":false,"description":"Text formatting object using format.text parameters without id or page; size is in points."},"text":{"type":"string","maxLength":65536,"description":"Literal UTF-8 shape text; inside batch, $$ escapes a literal leading $."}},"required":["x","y","width","height"],"additionalProperties":false}"#,
            always,
            crate::shapes::rect
        ),
        cmd!(
            "shape.draw.ellipse",
            "shape.draw.ellipse",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"x":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"X coordinate of the bottom-left corner, in page inches, increasing rightward."},"y":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Y coordinate of the bottom-left corner, in page inches, increasing upward."},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Width in internal inches along the local x axis of the shape or page."},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Height in internal inches along the local y axis of the shape or page."},"fill":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."}},"additionalProperties":false,"description":"Fill formatting object, using format.fill parameters without id or page."},"line":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"units":{"type":"string","enum":["IN","PT","MM","CM","FT","in","pt","mm","cm","ft"],"default":"IN","description":"Line weight unit: IN inches (default), PT points, MM millimetres, CM centimetres or FT feet."},"weight":{"description":"Line width in the selected units, inches by default; centered on the stroke path.","type":"number","minimum":-1000000.0,"maximum":1000000.0},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."},"begin_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the begin endpoint."},"end_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the end endpoint."}},"additionalProperties":false,"description":"Line formatting object, using format.line parameters without id or page."},"font":{"type":"object","properties":{"font":{"type":"string","minLength":1,"maxLength":65536,"description":"Font family name, resolved through the document FaceNames table; literal UTF-8 text."},"size":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Character size in points (1 point = 1/72 inch)."},"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"bold":{"type":"boolean","description":"Set or clear bold character formatting; boolean."},"italic":{"type":"boolean","description":"Set or clear italic character formatting; boolean."},"underline":{"type":"boolean","description":"Set or clear underlined character formatting; boolean."},"align":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless horizontal text alignment: 0 left, 1 center, 2 right, 3 justify, 4 force justify."}},"additionalProperties":false,"description":"Text formatting object using format.text parameters without id or page; size is in points."},"text":{"type":"string","maxLength":65536,"description":"Literal UTF-8 shape text; inside batch, $$ escapes a literal leading $."}},"required":["x","y","width","height"],"additionalProperties":false}"#,
            always,
            crate::shapes::ellipse
        ),
        cmd!(
            "shape.draw.line",
            "shape.draw.line",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"x1":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"X coordinate of the begin endpoint, in page inches (x right, y up)."},"y1":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Y coordinate of the begin endpoint, in page inches (x right, y up)."},"x2":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"X coordinate of the end endpoint, in page inches (x right, y up)."},"y2":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Y coordinate of the end endpoint, in page inches (x right, y up)."},"fill":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."}},"additionalProperties":false,"description":"Fill formatting object, using format.fill parameters without id or page."},"line":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"units":{"type":"string","enum":["IN","PT","MM","CM","FT","in","pt","mm","cm","ft"],"default":"IN","description":"Line weight unit: IN inches (default), PT points, MM millimetres, CM centimetres or FT feet."},"weight":{"description":"Line width in the selected units, inches by default; centered on the stroke path.","type":"number","minimum":-1000000.0,"maximum":1000000.0},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."},"begin_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the begin endpoint."},"end_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the end endpoint."}},"additionalProperties":false,"description":"Line formatting object, using format.line parameters without id or page."},"font":{"type":"object","properties":{"font":{"type":"string","minLength":1,"maxLength":65536,"description":"Font family name, resolved through the document FaceNames table; literal UTF-8 text."},"size":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Character size in points (1 point = 1/72 inch)."},"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"bold":{"type":"boolean","description":"Set or clear bold character formatting; boolean."},"italic":{"type":"boolean","description":"Set or clear italic character formatting; boolean."},"underline":{"type":"boolean","description":"Set or clear underlined character formatting; boolean."},"align":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless horizontal text alignment: 0 left, 1 center, 2 right, 3 justify, 4 force justify."}},"additionalProperties":false,"description":"Text formatting object using format.text parameters without id or page; size is in points."},"text":{"type":"string","maxLength":65536,"description":"Literal UTF-8 shape text; inside batch, $$ escapes a literal leading $."}},"required":["x1","y1","x2","y2"],"additionalProperties":false}"#,
            always,
            crate::shapes::line
        ),
        cmd!(
            "shape.move",
            "shape.move",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"x":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"X coordinate of the shape pin, in parent-coordinate inches, increasing rightward; page coordinates for top-level shapes."},"y":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Y coordinate of the shape pin, in parent-coordinate inches, increasing upward; page coordinates for top-level shapes."}},"required":["id","x","y"],"additionalProperties":false}"#,
            always,
            crate::shapes::move_shape
        ),
        cmd!(
            "shape.resize",
            "shape.resize",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Width in internal inches along the local x axis of the shape or page."},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Height in internal inches along the local y axis of the shape or page."}},"required":["id","width","height"],"additionalProperties":false}"#,
            always,
            crate::shapes::resize
        ),
        cmd!(
            "shape.rotate",
            "shape.rotate",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"angle":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Rotation in radians about the shape pin; positive angles are counterclockwise."}},"required":["id","angle"],"additionalProperties":false}"#,
            always,
            crate::shapes::rotate
        ),
        cmd!(
            "shape.flip",
            "shape.flip",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"axis":{"type":"string","enum":["x","y"],"description":"Axis selector relative to the shape or arrangement, as enumerated."}},"required":["id","axis"],"additionalProperties":false}"#,
            always,
            crate::shapes::flip
        ),
        cmd!(
            "shape.delete",
            "shape.delete",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::shapes::delete
        ),
        cmd!(
            "shape.order",
            "shape.order",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"order":{"type":"string","enum":["front","back","forward","backward"],"description":"Z-order operation within the shape parent: front, back, forward or backward."}},"required":["id","order"],"additionalProperties":false}"#,
            always,
            crate::shapes::order
        ),
        cmd!(query "sheet.get", "sheet.get", ["Sheet"], None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"scope":{"type":"string","enum":["document","page","shape"],"description":"Sheet owner: document, page or shape; defaults to shape."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal ShapeSheet cell name on the selected sheet."}},"required":["name"],"additionalProperties":false}"#, always, crate::sheet::get),
        cmd!(
            "sheet.set",
            "sheet.set",
            ["Sheet"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"scope":{"type":"string","enum":["document","page","shape"],"description":"Sheet owner: document, page or shape; defaults to shape."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal ShapeSheet cell name on the selected sheet."},"value":{"type":["string","number"],"description":"Literal value: numbers use the addressed cell internal units, strings remain strings."},"formula":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal ShapeSheet formula; unit-bearing literals may specify units explicitly."},"units":{"type":"string","minLength":1,"maxLength":65536,"description":"ShapeSheet display unit code; bare numeric value remains in internal units for the cell."},"force":{"type":"boolean","description":"Allow replacing a GUARD-protected formula; default false."}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::sheet::set_cell
        ),
        cmd!(
            "sheet.section.add",
            "sheet.section.add",
            ["Sheet"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"scope":{"type":"string","enum":["document","page","shape"],"description":"Sheet owner: document, page or shape; defaults to shape."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"section":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal ShapeSheet section name on the selected sheet."},"section_index":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Zero-based section IX for repeated sections such as Geometry."}},"required":["section"],"additionalProperties":false}"#,
            always,
            crate::sheet::add_section
        ),
        cmd!(
            "sheet.row.add",
            "sheet.row.add",
            ["Sheet"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"scope":{"type":"string","enum":["document","page","shape"],"description":"Sheet owner: document, page or shape; defaults to shape."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"section":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal ShapeSheet section name on the selected sheet."},"section_index":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Zero-based section IX for repeated sections such as Geometry."},"row_name":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal name for the new named ShapeSheet row."},"index":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Row IX written to the selected section; follows that section row numbering."},"type":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal ShapeSheet row type name, for example LineTo."}},"required":["section"],"additionalProperties":false}"#,
            always,
            crate::sheet::add_row
        ),
        cmd!(
            "shape.text",
            "shape.text",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"text":{"type":"string","maxLength":65536,"description":"Literal UTF-8 shape text; inside batch, $$ escapes a literal leading $."},"paragraphs":{"type":"array","maxItems":1024,"items":{"type":"object","properties":{"align":{"type":"integer","minimum":0,"maximum":4,"description":"Unitless horizontal text alignment: 0 left, 1 center, 2 right, 3 justify, 4 force justify."},"runs":{"type":"array","minItems":1,"maxItems":1024,"items":{"type":"object","properties":{"text":{"type":"string","maxLength":65536,"description":"Literal UTF-8 shape text; inside batch, $$ escapes a literal leading $."},"font":{"type":"string","maxLength":1024,"description":"Font family name, resolved through the document FaceNames table; literal UTF-8 text."},"size":{"type":"number","description":"Character size in points (1 point = 1/72 inch)."},"color":{"type":"string","description":"Literal sRGB color in #RRGGBB notation."},"bold":{"type":"boolean","description":"Set or clear bold character formatting; boolean."},"italic":{"type":"boolean","description":"Set or clear italic character formatting; boolean."},"underline":{"type":"boolean","description":"Set or clear underlined character formatting; boolean."}},"additionalProperties":false},"description":"Ordered character runs within this paragraph."}},"required":["runs"],"additionalProperties":false},"description":"Ordered paragraphs with runs and optional unitless alignment codes; replaces shape text."}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::content::shape_text
        ),
        cmd!(
            "shape.data.define",
            "shape.data.define",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal Shape Data row name, without the Prop. prefix."},"label":{"type":"string","description":"Human-readable Shape Data label; literal UTF-8 text."},"prompt":{"type":"string","description":"Literal UTF-8 help text for the data or connection row."},"type":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Shape Data type index, 0 through 7."},"format":{"type":"string","description":"Shape Data format string; semicolon-separated items for list-valued properties."},"invisible":{"type":"boolean","description":"Hide this Shape Data row from display; boolean."},"sort_key":{"type":"string","description":"Literal UTF-8 Shape Data sort key."}},"required":["id","name"],"additionalProperties":false}"#,
            always,
            crate::content::define
        ),
        cmd!(
            "shape.data.set",
            "shape.data.set",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal Shape Data row name, without the Prop. prefix."},"value":{"description":"Shape Data value matching its declared type; numbers use internal units."}},"required":["id","name","value"],"additionalProperties":false}"#,
            always,
            crate::content::set_data
        ),
        cmd!(
            "shape.hyperlink",
            "shape.hyperlink",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal UTF-8 name for this hyperlink."},"address":{"type":"string","description":"Hyperlink URL or file address; literal UTF-8 text."},"sub_address":{"type":"string","description":"Destination within the hyperlink target, such as a page name."},"description":{"type":"string","description":"Literal UTF-8 hyperlink display text."},"new_window":{"type":"boolean","description":"Open hyperlink target in a new window; boolean metadata."}},"required":["id","name"],"additionalProperties":false}"#,
            always,
            crate::content::hyperlink
        ),
        cmd!(
            "format.fill",
            "format.fill",
            ["Format"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::content::fill
        ),
        cmd!(
            "format.line",
            "format.line",
            ["Format"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"units":{"type":"string","enum":["IN","PT","MM","CM","FT","in","pt","mm","cm","ft"],"default":"IN","description":"Line weight unit: IN inches (default), PT points, MM millimetres, CM centimetres or FT feet."},"weight":{"description":"Line width in the selected units, inches by default; centered on the stroke path.","type":"number","minimum":-1000000.0,"maximum":1000000.0},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."},"begin_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the begin endpoint."},"end_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the end endpoint."}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::content::line
        ),
        cmd!(
            "format.text",
            "format.text",
            ["Format"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"font":{"type":"string","minLength":1,"maxLength":65536,"description":"Font family name, resolved through the document FaceNames table; literal UTF-8 text."},"size":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Character size in points (1 point = 1/72 inch)."},"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"bold":{"type":"boolean","description":"Set or clear bold character formatting; boolean."},"italic":{"type":"boolean","description":"Set or clear italic character formatting; boolean."},"underline":{"type":"boolean","description":"Set or clear underlined character formatting; boolean."},"align":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless horizontal text alignment: 0 left, 1 center, 2 right, 3 justify, 4 force justify."}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::content::format_text
        ),
        cmd!(
            "shape.duplicate",
            "shape.duplicate",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"dx":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Horizontal translation in inches along the parent axes, relative to the original shape; page axes for top-level shapes."},"dy":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Vertical translation in inches along the parent axes, positive upward, relative to the original shape; page axes for top-level shapes."}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::arrange::duplicate
        ),
        cmd!(
            "shape.group",
            "shape.group",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"ids":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"minItems":2,"maxItems":10000,"description":"Ordered page-scoped shape IDs; batch accepts $name in each entry."}},"required":["ids"],"additionalProperties":false}"#,
            always,
            crate::arrange::group
        ),
        cmd!(
            "shape.ungroup",
            "shape.ungroup",
            ["Shape"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."}},"required":["id"],"additionalProperties":false}"#,
            always,
            crate::arrange::ungroup
        ),
        cmd!(
            "arrange.align",
            "arrange.align",
            ["Arrange"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"ids":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"maxItems":10000,"description":"Ordered page-scoped shape IDs; batch accepts $name in each entry."},"mode":{"type":"string","enum":["left","right","center","bottom","top","middle"],"description":"Arrangement mode or glue mode, as enumerated; unitless."}},"required":["ids","mode"],"additionalProperties":false}"#,
            always,
            crate::arrange::align
        ),
        cmd!(
            "arrange.distribute",
            "arrange.distribute",
            ["Arrange"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"ids":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"maxItems":10000,"description":"Ordered page-scoped shape IDs; batch accepts $name in each entry."},"axis":{"type":"string","enum":["x","y"],"description":"Axis selector relative to the shape or arrangement, as enumerated."}},"required":["ids","axis"],"additionalProperties":false}"#,
            always,
            crate::arrange::distribute
        ),
        cmd!(
            "arrange.same_size",
            "arrange.same_size",
            ["Arrange"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"maximum":100000,"description":"One-based page ordinal; defaults to the current session page."},"ids":{"type":"array","items":{"type":"integer","minimum":0,"maximum":4294967294},"maxItems":10000,"description":"Ordered page-scoped shape IDs; batch accepts $name in each entry."},"axis":{"type":"string","enum":["width","height","both"],"description":"Axis selector relative to the shape or arrangement, as enumerated."}},"required":["ids"],"additionalProperties":false}"#,
            always,
            crate::arrange::same_size
        ),
        cmd!(noundo "stencil.open", "stencil.open", ["Stencils"], None, r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":65536,"description":"Input or output filesystem path; relative paths resolve from the process working directory."},"builtin":{"type":"string","enum":["network"],"description":"Contributor-original builtin stencil name; network exposes eleven semantic masters."}},"required":[],"additionalProperties":false}"#, always, crate::stencils::open),
        cmd!(noundo "stencil.close", "stencil.close", ["Stencils"], None, r#"{"type":"object","properties":{"stencil":{"type":"integer","minimum":1,"maximum":64,"description":"Open stencil index: 0 is the document stencil, external indices start at 1."}},"required":["stencil"],"additionalProperties":false}"#, always, crate::stencils::close),
        cmd!(query "stencil.list", "stencil.list", ["Stencils"], None, r#"{"type":"object","properties":{},"required":[],"additionalProperties":false}"#, always, crate::stencils::list),
        cmd!(query "stencil.search", "stencil.search", ["Stencils"], None, r#"{"type":"object","properties":{"stencil":{"type":"integer","minimum":0,"maximum":64,"description":"Open stencil index: 0 is the document stencil, external indices start at 1."},"query":{"type":"string","minLength":1,"maxLength":65536,"description":"Case-insensitive substring of a master name; limits search across selected open stencils."},"limit":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Maximum number of returned masters; unitless count."}},"required":["query"],"additionalProperties":false}"#, always, crate::stencils::search),
        cmd!(query "stencil.masters", "stencil.masters", ["Stencils"], None, r#"{"type":"object","properties":{"stencil":{"type":"integer","minimum":0,"maximum":64,"description":"Open stencil index: 0 is the document stencil, external indices start at 1."},"limit":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Maximum number of returned masters; unitless count."}},"required":[],"additionalProperties":false}"#, always, crate::stencils::masters),
        cmd!(
            "master.drop",
            "master.drop",
            ["Stencils"],
            None,
            r#"{"type":"object","properties":{"stencil":{"type":"integer","minimum":0,"maximum":64,"description":"Open stencil index: 0 is the document stencil, external indices start at 1."},"master":{"type":["integer","string"],"description":"Master ID or semantic/universal name in the selected stencil; not a page shape ID."},"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"x":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"X coordinate of the shape pin, in page inches, increasing rightward."},"y":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Y coordinate of the shape pin, in page inches, increasing upward."},"width":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Width in internal inches along the local x axis of the shape or page."},"height":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Height in internal inches along the local y axis of the shape or page."},"fill":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."}},"additionalProperties":false,"description":"Fill formatting object, using format.fill parameters without id or page."},"line":{"type":"object","properties":{"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"pattern":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio fill or line pattern index; 0 means none."},"units":{"type":"string","enum":["IN","PT","MM","CM","FT","in","pt","mm","cm","ft"],"default":"IN","description":"Line weight unit: IN inches (default), PT points, MM millimetres, CM centimetres or FT feet."},"weight":{"description":"Line width in the selected units, inches by default; centered on the stroke path.","type":"number","minimum":-1000000.0,"maximum":1000000.0},"transparency":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Unitless transparency from 0 (opaque) to 1 (transparent)."},"begin_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the begin endpoint."},"end_arrow":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless Visio arrow type index at the end endpoint."}},"additionalProperties":false,"description":"Line formatting object, using format.line parameters without id or page."},"font":{"type":"object","properties":{"font":{"type":"string","minLength":1,"maxLength":65536,"description":"Font family name, resolved through the document FaceNames table; literal UTF-8 text."},"size":{"type":"number","minimum":-1000000.0,"maximum":1000000.0,"description":"Character size in points (1 point = 1/72 inch)."},"color":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal sRGB color in #RRGGBB notation."},"bold":{"type":"boolean","description":"Set or clear bold character formatting; boolean."},"italic":{"type":"boolean","description":"Set or clear italic character formatting; boolean."},"underline":{"type":"boolean","description":"Set or clear underlined character formatting; boolean."},"align":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Unitless horizontal text alignment: 0 left, 1 center, 2 right, 3 justify, 4 force justify."}},"additionalProperties":false,"description":"Text formatting object using format.text parameters without id or page; size is in points."},"text":{"type":"string","maxLength":65536,"description":"Literal UTF-8 shape text; inside batch, $$ escapes a literal leading $."}},"required":["master"],"additionalProperties":false}"#,
            always,
            crate::masters::drop_master
        ),
        cmd!(
            "master.new_from_shape",
            "master.new_from_shape",
            ["Stencils"],
            None,
            r#"{"type":"object","properties":{"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Literal UTF-8 name for this master."}},"required":["name"],"additionalProperties":false}"#,
            always,
            crate::masters::new_from_shape
        ),
        cmd!(
            "shape.connection.add",
            "shape.connection.add",
            ["Glue"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"x":{"type":["number","string"],"description":"Connection point local x coordinate in inches or a universal ShapeSheet formula, relative to the shape local origin."},"y":{"type":["number","string"],"description":"Connection point local y coordinate in inches or a universal ShapeSheet formula, relative to the shape local origin."},"dir_x":{"type":"number","minimum":-1000000,"maximum":1000000,"description":"Unitless local x direction component for the connection point."},"dir_y":{"type":"number","minimum":-1000000,"maximum":1000000,"description":"Unitless local y direction component for the connection point."},"type":{"type":"integer","minimum":0,"maximum":2,"description":"Connection type: 0 inward, 1 outward, 2 both; unitless."},"name":{"type":"string","minLength":1,"maxLength":65536,"description":"Optional universal connection row name; omitted for an indexed row."},"auto_gen":{"type":"boolean","description":"Mark the connection row as automatically generated using the standard AutoGen cell."},"prompt":{"type":"string","maxLength":65536,"description":"Literal UTF-8 help text for the data or connection row."}},"required":["id","x","y"],"additionalProperties":false}"#,
            always,
            crate::glue::connection
        ),
        cmd!(
            "connect.static",
            "connect.static",
            ["Glue"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"target":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Target shape ID on the selected page; batch accepts $name."},"point":{"type":"string","minLength":1,"maxLength":65536,"description":"Universal connection X-cell name on the target, for example Connections.X1."},"end":{"type":"string","enum":["begin","end"],"description":"Connector endpoint: begin or end; independent of shape ordering."}},"required":["id","target","point","end"],"additionalProperties":false}"#,
            always,
            crate::glue::static_glue
        ),
        cmd!(
            "connect.dynamic",
            "connect.dynamic",
            ["Glue"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."},"target":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Target shape ID on the selected page; batch accepts $name."},"end":{"type":"string","enum":["begin","end"],"description":"Connector endpoint: begin or end; independent of shape ordering."},"preference":{"type":"string","enum":["horizontal","vertical"],"description":"Dynamic glue direction preference: horizontal or vertical in page space."}},"required":["id","target","end"],"additionalProperties":false}"#,
            always,
            crate::glue::dynamic_glue
        ),
        cmd!(
            "route.reroute",
            "route.reroute",
            ["Glue"],
            None,
            r#"{"type":"object","properties":{"page":{"type":"integer","minimum":1,"description":"One-based page ordinal; defaults to the current session page."},"id":{"type":"integer","minimum":0,"maximum":4294967294,"description":"Page-scoped shape ID, not an array index; batch accepts $name."}},"required":[],"additionalProperties":false}"#,
            always,
            crate::glue::reroute
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
