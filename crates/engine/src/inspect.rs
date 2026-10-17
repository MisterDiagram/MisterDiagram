//! Headless control queries use the same evaluated document as authoring.
use crate::{Result, Session, edit::*, operation};
use misterdiagram_geom::Rect;
use misterdiagram_model::Connect;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Map, Value, json};

const XFORM: &[&str] = &["PinX", "PinY", "Width", "Height", "LocPinX", "LocPinY", "Angle", "FlipX", "FlipY"];
const GLUE: &[&str] = &["BeginX", "BeginY", "EndX", "EndY", "BegTrigger", "EndTrigger", "GlueType", "WalkPreference"];

/// A transport-independent alias. Do not nest edits or create an extra undo step.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.application.beginundoscope
pub(crate) fn execute(s: &mut Session, p: &Value) -> Result<Value> {
    let command = name(p, "command")?;
    if command == "engine.execute" {
        return Err(operation("engine.execute cannot invoke itself"));
    }
    s.execute(command, p.get("params").unwrap_or(&json!({})))
}
pub(crate) fn commands(s: &mut Session, _: &Value) -> Result<Value> {
    serde_json::to_value(s.commands()).map_err(operation)
}

// Charge serialized entries before retaining them in a response. Leave room for
// the MCP text-content envelope, which escapes the JSON a second time.
struct Budget(usize);
impl std::io::Write for Budget {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0 = self.0.checked_sub(bytes.len()).ok_or_else(|| std::io::Error::other("inspection response limit"))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl Budget {
    fn charge(&mut self, value: &Value) -> Result<()> {
        serde_json::to_writer(self, value).map_err(operation)
    }
}
fn cell(e: &mut Evaluator<'_>, scope: Scope, name: &str) -> Value {
    let source = e.cell(scope, name);
    json!({"value":e.evaluate(scope,name).text(), "formula":source.and_then(|c| c.formula.as_deref()),
        "cached":source.and_then(|c| c.value.as_deref()), "units":source.and_then(|c| c.units.as_deref())})
}
fn cells(e: &mut Evaluator<'_>, scope: Scope, names: impl IntoIterator<Item = String>, budget: &mut Budget) -> Result<Value> {
    let mut result = Map::new();
    for name in names {
        let value = cell(e, scope, &name);
        budget.charge(&json!([name, value]))?;
        result.insert(name, value);
    }
    Ok(Value::Object(result))
}
fn section(e: &mut Evaluator<'_>, scope: Scope, section: &str, budget: &mut Budget) -> Result<Value> {
    let names =
        e.resolved_named_cells(scope).into_iter().filter(|n| n.section.is_some_and(|s| s.name == section)).map(|n| n.name).collect::<Vec<_>>();
    cells(e, scope, names, budget)
}
/// Connect records retain the source/target cell and part, distinguishing static
/// connection-point glue from dynamic whole-shape glue.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
fn connects<'a>(items: impl Iterator<Item = &'a Connect>, budget: &mut Budget) -> Result<Value> {
    let mut result = Vec::new();
    for c in items {
        let value = json!({"from_shape":c.from_sheet,"from_cell":c.from_cell,"from_part":c.from_part,
            "to_shape":c.to_sheet,"to_cell":c.to_cell,"to_part":c.to_part});
        budget.charge(&value)?;
        result.push(value);
    }
    Ok(json!(result))
}
/// Alignment-box bounds are in page inches, including every enclosing transform.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-transform-section
pub(crate) fn bounds(e: &mut Evaluator<'_>, scope: Scope) -> Result<Rect> {
    let w = e.evaluate(scope, "Width").numeric().map_err(operation)?;
    let h = e.evaluate(scope, "Height").numeric().map_err(operation)?;
    let r = e.shape_to_page(scope).map_err(operation)?.transform_rect_bbox(Rect::new(0., 0., w, h));
    if ![r.x0, r.y0, r.x1, r.y1].iter().all(|v| v.is_finite() && v.abs() <= 1e6) {
        return Err(operation("invalid inspection bounds"));
    }
    Ok(r)
}
fn summary(e: &mut Evaluator<'_>, scope: Scope, budget: &mut Budget) -> Result<Value> {
    let shape = e.shape(scope).ok_or_else(|| operation("shape not found"))?;
    let r = bounds(e, scope)?;
    let text = e.text_body(scope).map_err(operation)?.text();
    let mut value = json!({"id":shape.id,"name":shape.name,"name_u":shape.name_u,"master":shape.master,
        "master_shape":shape.master_shape,"type":shape.shape_type,"bounds":{"x":r.x0,"y":r.y0,"width":r.width(),"height":r.height()},"text":text});
    budget.charge(&value)?;
    value["data"] = section(e, scope, "Property", budget)?;
    Ok(value)
}
/// Inspect pages, layers, flattened shape identities and Connects without editing.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
pub(crate) fn document(s: &mut Session, p: &Value) -> Result<Value> {
    let filter = p.get("page").map(|_| page_index(s, p)).transpose()?;
    let themes = misterdiagram_theme::Themes::from_document(&s.document).map_err(operation)?;
    let mut e = Evaluator::new(&s.document).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let mut budget = Budget(1024 * 1024);
    let mut pages = Vec::new();
    for (index, page) in s.document.pages.iter().enumerate() {
        if filter.is_some_and(|f| f != index) {
            continue;
        }
        let mut shapes = Vec::new();
        let scopes = e.scopes().filter(|v| matches!(v, Scope::PageShape {page: id,..} if *id == page.id)).collect::<Vec<_>>();
        for scope in scopes {
            let mut value = summary(&mut e, scope, &mut budget)?;
            let id = e.shape(scope).ok_or_else(|| operation("shape missing"))?.id;
            value["connects"] = connects(page.connects.iter().filter(|c| c.from_sheet == id || c.to_sheet == id), &mut budget)?;
            shapes.push(value);
        }
        let layers = section(&mut e, Scope::Page(page.id), "Layer", &mut budget)?;
        let connections = connects(page.connects.iter(), &mut budget)?;
        let mut value = json!({"page":index+1,"id":page.id,"name":page.name,"name_u":page.name_u});
        budget.charge(&value)?;
        value["layers"] = layers;
        value["shapes"] = json!(shapes);
        value["connects"] = connections;
        pages.push(value);
    }
    Ok(json!({"pages":pages,"selection":{"page":s.selection.page+1,"ids":s.selection.shapes.iter().map(|id| id.0).collect::<Vec<_>>()}}))
}
/// Evaluated XForm, text, Shape Data, connection rows and glue, plus requested cells.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.cellsu
/// https://learn.microsoft.com/en-us/office/client-developer/visio/text-element-shapesheet_type-complextypevisio-xml
pub(crate) fn shape(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    let requested = match p.get("cells") {
        None => Vec::new(),
        Some(Value::Array(names)) if names.len() <= 256 => names
            .iter()
            .map(|n| {
                n.as_str()
                    .filter(|n| !n.is_empty() && n.len() <= 256 && !n.contains('\0'))
                    .map(str::to_owned)
                    .ok_or_else(|| operation("invalid inspection cell name"))
            })
            .collect::<Result<Vec<_>>>()?,
        _ => return Err(operation("cells must contain at most 256 cell names")),
    };
    let themes = misterdiagram_theme::Themes::from_document(&s.document).map_err(operation)?;
    let mut e = Evaluator::new(&s.document).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    for name in &requested {
        if e.cell(scope, name).is_none() {
            return Err(operation(format!("cell not found: {name}")));
        }
    }
    let mut budget = Budget(1024 * 1024);
    let mut value = summary(&mut e, scope, &mut budget)?;
    value["page"] = json!(index + 1);
    value["xform"] = cells(&mut e, scope, XFORM.iter().map(|s| (*s).into()), &mut budget)?;
    value["text_cells"] = cells(
        &mut e,
        scope,
        ["TxtPinX", "TxtPinY", "TxtLocPinX", "TxtLocPinY", "TxtWidth", "TxtHeight", "TxtAngle"].map(str::to_owned),
        &mut budget,
    )?;
    value["connection_points"] = section(&mut e, scope, "Connection", &mut budget)?;
    let names = GLUE.iter().filter(|n| e.cell(scope, n).is_some()).map(|n| (*n).into()).collect::<Vec<_>>();
    value["glue"] = cells(&mut e, scope, names, &mut budget)?;
    value["cells"] = cells(&mut e, scope, requested, &mut budget)?;
    let page = s.document.pages.get(index).ok_or_else(|| operation("page missing"))?;
    value["connects"] = connects(page.connects.iter().filter(|c| c.from_sheet == id || c.to_sheet == id), &mut budget)?;
    Ok(value)
}
