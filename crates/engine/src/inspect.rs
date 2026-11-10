//! Headless control queries use the same evaluated document as authoring.
use crate::{Result, Session, edit::*, operation};
use misterdiagram_geom::Rect;
use misterdiagram_model::Connect;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Map, Value, json};

mod commands;
mod style;
pub(crate) use commands::commands;

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

// Charge serialized entries before retaining them in a response. Leave room for
// the MCP text-content envelope, which escapes the JSON a second time.
struct Budget(usize);
impl std::io::Write for Budget {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0 = self
            .0
            .checked_sub(bytes.len())
            .ok_or_else(|| std::io::Error::other("inspection exceeds 65536 byte budget; use page, find, ids, cells or sections filters"))?;
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
        if result.contains_key(&name) {
            continue;
        }
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
const SECTIONS: &[&str] = &["xform", "text", "connection_points", "data", "glue", "cells"];
struct Filter {
    sections: Vec<String>,
    requested: Vec<String>,
    minimal: bool,
    detailed: bool,
    explicit_sections: bool,
    cells_present: bool,
}
impl Filter {
    fn parse(p: &Value, document: bool) -> Result<Self> {
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
        let sections = match p.get("sections") {
            Some(Value::Array(names)) if names.len() <= 8 => names
                .iter()
                .map(|n| {
                    n.as_str()
                        .filter(|n| SECTIONS.contains(n) || *n == "style" || document && *n == "outline")
                        .map(str::to_owned)
                        .ok_or_else(|| operation("invalid inspection section"))
                })
                .collect::<Result<Vec<_>>>()?,
            Some(_) => return Err(operation("sections must contain at most eight section names")),
            None if p.get("cells").is_some() => vec!["cells".into()],
            None if document => ["text", "data", "glue"].map(str::to_owned).to_vec(),
            None => SECTIONS.iter().map(|n| (*n).into()).collect(),
        };
        if sections.iter().any(|s| s == "outline") && sections.len() != 1 {
            return Err(operation("outline cannot be combined with other sections"));
        }
        Ok(Self {
            sections,
            requested,
            minimal: p.get("cells").is_some() || p.get("sections").is_some(),
            detailed: !document || p.get("sections").is_some(),
            explicit_sections: p.get("sections").is_some(),
            cells_present: p.get("cells").is_some(),
        })
    }
    fn has(&self, name: &str) -> bool {
        self.sections.iter().any(|n| n == name)
    }
}
fn summary(e: &mut Evaluator<'_>, scope: Scope, filter: &Filter, links: &[Connect], budget: &mut Budget) -> Result<Value> {
    let shape = e.shape(scope).ok_or_else(|| operation("shape not found"))?;
    let id = shape.id;
    let r = bounds(e, scope)?;
    let mut value = json!({"id":id,"bounds":{"x":r.x0,"y":r.y0,"width":r.width(),"height":r.height()}});
    if filter.has("outline") {
        value = json!({"id":id,"bounds":[round4(r.x0),round4(r.y0),round4(r.width()),round4(r.height())]});
        if !shape.name_u.is_empty() {
            value["name"] = json!(shape.name_u);
        }
        if let Some(master) = shape.master {
            value["master"] = json!(master);
        }
        if shape.shape_type.as_deref().is_some_and(|kind| kind != "Shape") {
            value["type"] = json!(shape.shape_type);
        }
        let text = e.text_body(scope).map_err(operation)?.text();
        if !text.is_empty() {
            value["text"] = json!(text);
        }
    }
    if !filter.minimal {
        value["name"] = json!(shape.name);
        value["name_u"] = json!(shape.name_u);
        value["master"] = json!(shape.master);
        value["master_shape"] = json!(shape.master_shape);
        value["type"] = json!(shape.shape_type);
    }
    budget.charge(&value)?;
    if filter.has("xform") {
        value["xform"] = cells(e, scope, XFORM.iter().map(|s| (*s).into()), budget)?;
    }
    if filter.has("text") {
        let text = json!(e.text_body(scope).map_err(operation)?.text());
        budget.charge(&text)?;
        value["text"] = text;
        if filter.detailed {
            let names = ["TxtPinX", "TxtPinY", "TxtLocPinX", "TxtLocPinY", "TxtWidth", "TxtHeight", "TxtAngle"]
                .into_iter()
                .filter(|n| !filter.explicit_sections || e.cell(scope, n).is_some())
                .map(str::to_owned)
                .collect::<Vec<_>>();
            if !names.is_empty() {
                value["text_cells"] = cells(e, scope, names, budget)?;
            }
        }
    }
    if filter.has("data") {
        value["data"] = section(e, scope, "Property", budget)?;
    }
    if filter.has("connection_points") {
        value["connection_points"] = section(e, scope, "Connection", budget)?;
    }
    if filter.has("glue") {
        let names = GLUE.iter().filter(|n| e.cell(scope, n).is_some()).map(|n| (*n).into()).collect::<Vec<_>>();
        if filter.detailed {
            value["glue"] = cells(e, scope, names, budget)?;
        }
        value["connects"] = connects(links.iter().filter(|c| c.from_sheet == id || c.to_sheet == id), budget)?;
    }
    if filter.has("style") {
        let resolved = style::style(e, scope);
        budget.charge(&resolved)?;
        value["style"] = resolved;
    }
    if filter.has("cells") || filter.cells_present {
        for name in &filter.requested {
            if e.cell(scope, name).is_none() {
                return Err(operation(format!("cell not found: {name}")));
            }
        }
        value["cells"] = cells(e, scope, filter.requested.clone(), budget)?;
    }
    Ok(value)
}
fn round4(n: f64) -> f64 {
    (n * 10000.).round() / 10000.
}
fn folded(text: &str) -> String {
    let mut result = String::new();
    let mut whitespace = false;
    for character in text.chars() {
        if character.is_whitespace() {
            if !whitespace {
                result.push(' ');
            }
            whitespace = true;
        } else {
            result.push(character);
            whitespace = false;
        }
    }
    result.to_lowercase()
}
fn bounded(value: Value) -> Result<Value> {
    Budget(64 * 1024).charge(&value)?;
    Ok(value)
}
/// Inspect pages, layers, flattened shape identities and Connects without editing.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
pub(crate) fn document(s: &mut Session, p: &Value) -> Result<Value> {
    let page_filter = p.get("page").map(|_| page_index(s, p)).transpose()?;
    let filter = Filter::parse(p, true)?;
    if p.get("ids").is_some() && p.get("find").is_some() {
        return Err(operation("ids and find cannot be combined"));
    }
    if p.get("ids").is_some() && page_filter.is_none() {
        return Err(operation("ids requires page"));
    }
    let ids = match p.get("ids") {
        None => None,
        Some(Value::Array(ids)) if !ids.is_empty() && ids.len() <= 256 => Some(
            ids.iter()
                .map(|id| id.as_u64().and_then(|id| u32::try_from(id).ok()).ok_or_else(|| operation("invalid shape id")))
                .collect::<Result<Vec<_>>>()?,
        ),
        _ => return Err(operation("ids must contain 1 through 256 shape ids")),
    };
    let find = p.get("find").and_then(Value::as_str).map(folded);
    let targeted = ids.is_some() || find.is_some();
    let themes = misterdiagram_theme::Themes::from_document(&s.document).map_err(operation)?;
    let mut e = Evaluator::new(&s.document).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    if filter.has("style") {
        e = e.with_cached_fallback();
    }
    let mut budget = Budget(64 * 1024);
    let mut pages = Vec::new();
    for (ordinal, (index, page)) in s.document.page_order().enumerate() {
        if page_filter.is_some_and(|f| f != index) {
            continue;
        }
        let mut shapes = Vec::new();
        let mut scopes = e.scopes().filter(|v| matches!(v, Scope::PageShape {page: id,..} if *id == page.id)).collect::<Vec<_>>();
        if let Some(ids) = &ids {
            scopes = ids
                .iter()
                .map(|id| {
                    scopes
                        .iter()
                        .find(|scope| matches!(scope, Scope::PageShape {shape, ..} if shape == id))
                        .copied()
                        .ok_or_else(|| operation(format!("shape not found: {id}")))
                })
                .collect::<Result<Vec<_>>>()?;
        }
        for scope in scopes {
            if let Some(find) = &find {
                let name_matches = e.shape(scope).is_some_and(|shape| folded(&shape.name_u).contains(find));
                if !name_matches && !folded(&e.text_body(scope).map_err(operation)?.text()).contains(find) {
                    continue;
                }
            }
            let value = summary(&mut e, scope, &filter, &page.connects, &mut budget)?;
            shapes.push(value);
        }
        let mut value = if filter.has("outline") {
            let mut links = Vec::new();
            let mut linked = std::collections::BTreeSet::new();
            for shape in &shapes {
                if !linked.insert(shape["id"].as_u64()) {
                    continue;
                }
                let mut link = json!({"id":shape["id"]});
                for c in page.connects.iter().filter(|c| Some(u64::from(c.from_sheet)) == shape["id"].as_u64()) {
                    let end = match c.from_cell.as_deref() {
                        Some("BeginX" | "BeginY") => "begin",
                        Some("EndX" | "EndY") => "end",
                        _ => continue,
                    };
                    link[end] = json!(c.to_sheet);
                }
                if link.as_object().is_some_and(|v| v.len() > 1) {
                    budget.charge(&link)?;
                    links.push(link);
                }
            }
            let header = json!({"page":ordinal+1,"id":page.id,"name":page.name});
            budget.charge(&header)?;
            json!({"page":ordinal+1,"id":page.id,"name":page.name,"links":links})
        } else {
            let mut header = json!({"page":ordinal+1,"id":page.id,"name":page.name,"name_u":page.name_u});
            let layers = if targeted { None } else { Some(section(&mut e, Scope::Page(page.id), "Layer", &mut budget)?) };
            budget.charge(&header)?;
            if let Some(layers) = layers {
                header["layers"] = layers;
            }
            header
        };
        let returned = shapes.iter().filter_map(|shape| shape["id"].as_u64()).collect::<std::collections::BTreeSet<_>>();
        value["shapes"] = json!(shapes);
        if filter.has("glue") {
            value["connects"] = connects(
                page.connects
                    .iter()
                    .filter(|c| !targeted || returned.contains(&u64::from(c.from_sheet)) || returned.contains(&u64::from(c.to_sheet))),
                &mut budget,
            )?;
        }
        pages.push(value);
    }
    bounded(
        json!({"pages":pages,"selection":{"page":s.document.page_ordinal(s.selection.page),"ids":s.selection.shapes.iter().map(|id| id.0).collect::<Vec<_>>()}}),
    )
}
/// Evaluated XForm, text, Shape Data, connection rows and glue, plus requested cells.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.cellsu
/// https://learn.microsoft.com/en-us/office/client-developer/visio/text-element-shapesheet_type-complextypevisio-xml
pub(crate) fn shape(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, _, scope) = shape_scope(s, p)?;
    let filter = Filter::parse(p, false)?;
    let themes = misterdiagram_theme::Themes::from_document(&s.document).map_err(operation)?;
    let mut e = Evaluator::new(&s.document).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    if filter.has("style") {
        e = e.with_cached_fallback();
    }
    let mut budget = Budget(64 * 1024);
    let page = s.document.pages.get(index).ok_or_else(|| operation("page missing"))?;
    let mut value = summary(&mut e, scope, &filter, &page.connects, &mut budget)?;
    if !filter.minimal {
        value["page"] = json!(page_ordinal(s, index)?);
    }
    bounded(value)
}
