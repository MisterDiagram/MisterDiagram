//! Bounded session stencil documents and master discovery.
use crate::{Result, Session, edit::*, operation};
use diagramcraft_model::{Document, Master};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
use std::{path::PathBuf, sync::Arc};
#[derive(Clone)]
pub(crate) struct Stencil {
    pub path: PathBuf,
    pub document: Arc<Document>,
}
const MAX_SOURCE_BYTES: usize = 128 * 1024 * 1024;
fn source_size(doc: &Document) -> usize {
    doc.parts.values().fold(0usize, |n, p| n.saturating_add(p.bytes.len()))
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
pub(crate) fn open(s: &mut Session, p: &Value) -> Result<Value> {
    let path = crate::files::path(p, "path")?.canonicalize().map_err(operation)?;
    if let Some(index) = s.stencils.iter().position(|stencil| stencil.path == path) {
        return Ok(json!({"stencil":index+1,"masters":s.stencils.get(index).map(|s|s.document.masters.len())}));
    }
    if s.stencils.len() >= 64 {
        return Err(operation("stencil count limit"));
    }
    let document = diagramcraft_vsdx::read(&path).map_err(operation)?;
    if !document.parts.get(&document.main_part).and_then(|p| p.content_type.as_deref()).is_some_and(|v| v.contains("stencil")) {
        return Err(operation("file is not a stencil"));
    }
    let needed = source_size(&document);
    let already_open = s.stencils.iter().fold(0usize, |n, s| n.saturating_add(source_size(&s.document)));
    if needed.saturating_add(already_open) > MAX_SOURCE_BYTES {
        let name = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy();
        return Err(operation(format!(
            "retained stencil source byte limit: {name} needs {:.1} MiB, {:.1} MiB already open, limit {} MiB",
            needed as f64 / (1024. * 1024.),
            already_open as f64 / (1024. * 1024.),
            MAX_SOURCE_BYTES / (1024 * 1024),
        )));
    }
    let count = document.masters.len();
    s.stencils.push(Stencil { path, document: Arc::new(document) });
    Ok(json!({"stencil":s.stencils.len(),"masters":count}))
}
pub(crate) fn document(s: &Session, p: &Value) -> Result<Arc<Document>> {
    match p.get("stencil") {
        None => Ok(s.document.clone()),
        Some(_) => {
            let i = usize::try_from(uint(p, "stencil")?).map_err(operation)?.checked_sub(1).ok_or_else(|| operation("stencil is one-based"))?;
            s.stencils.get(i).map(|s| s.document.clone()).ok_or_else(|| operation("stencil not found"))
        }
    }
}
pub(crate) fn find<'a>(document: &'a Document, p: &Value) -> Result<&'a Master> {
    let selector = p.get("master").ok_or_else(|| operation("master is required"))?;
    document
        .masters
        .iter()
        .find(|master| match selector {
            Value::String(name) => master.name_u.eq_ignore_ascii_case(name) || master.name.eq_ignore_ascii_case(name),
            _ => selector.as_u64() == Some(u64::from(master.id)),
        })
        .ok_or_else(|| operation("master not found"))
}
fn entries(document: &Document, query: Option<&str>, limit: usize) -> Result<Value> {
    let themes = diagramcraft_theme::Themes::from_document(document).map_err(operation)?;
    let mut e = Evaluator::new(document).map_err(operation)?.with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
    let words = query.map(|s| s.to_lowercase().split_whitespace().map(str::to_owned).collect::<Vec<_>>()).unwrap_or_default();
    let mut result = vec![];
    let mut matched = 0;
    for master in &document.masters {
        let scope = master.shapes.first().map(|shape| Scope::MasterShape { master: master.id, shape: shape.id });
        let keywords = scope.map(|scope| e.evaluate(scope, "ShapeKeywords").text()).unwrap_or_default();
        let haystack = format!("{} {} {keywords}", master.name, master.name_u).to_lowercase();
        if !words.iter().all(|word| haystack.contains(word)) {
            continue;
        }
        matched += 1;
        if result.len() >= limit {
            continue;
        }
        let dimension = |e: &mut Evaluator<'_>, name: &str| scope.map(|scope| e.evaluate(scope, name).numeric().map_err(operation)).transpose();
        result.push(json!({"id":master.id,"name":master.name,"name_u":master.name_u,"width":dimension(&mut e,"Width")?,"height":dimension(&mut e,"Height")?,"keywords":keywords}));
    }
    Ok(json!({"masters":result,"matched":matched}))
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.document.masters
pub(crate) fn masters(s: &mut Session, p: &Value) -> Result<Value> {
    let limit = p.get("limit").map(|_| uint(p, "limit")).transpose()?.unwrap_or(1000).min(1000) as usize;
    entries(document(s, p)?.as_ref(), None, limit)
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shapekeywords-cell-miscellaneous-section
pub(crate) fn search(s: &mut Session, p: &Value) -> Result<Value> {
    let query = name(p, "query")?;
    let limit = p.get("limit").map(|_| uint(p, "limit")).transpose()?.unwrap_or(1000).min(1000) as usize;
    entries(document(s, p)?.as_ref(), Some(query), limit)
}
