//! Bounded session stencil documents and master discovery.
use crate::{Result, Session, edit::*, operation};
use misterdiagram_model::{Document, Master};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
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
    let document = misterdiagram_vsdx::read_with_open_nodes(&path, s.retained_xml_nodes()).map_err(operation)?;
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
    let themes = misterdiagram_theme::Themes::from_document(document).map_err(operation)?;
    let mut e = Evaluator::new(document).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
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

#[cfg(test)]
mod retained_tests {
    use super::*;
    #[test]
    fn document_and_stencils_share_admission_and_refusal_rolls_back() {
        let directory =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../target/engine-tests/retained-session-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let bytes = misterdiagram_vsdx::write_as(&crate::files::blank_document(false), misterdiagram_vsdx::SaveKind::Stencil).unwrap().bytes;
        let nodes = misterdiagram_vsdx::import(&bytes).unwrap().retained_xml_nodes;
        let first = directory.join("first.vssx");
        let second = directory.join("second.vssx");
        std::fs::write(&first, &bytes).unwrap();
        std::fs::write(&second, &bytes).unwrap();
        let mut s = Session::new();
        // Model an already-admitted population without allocating millions of test
        // nodes. The full-size CLI session edge is measured separately.
        Arc::make_mut(&mut s.document).retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES - (2 * nodes - 1);
        s.execute("stencil.open", &json!({"path":first})).unwrap();
        let already = s.retained_xml_nodes();
        s.execute("stencil.open", &json!({"path":first})).unwrap();
        assert_eq!(s.retained_xml_nodes(), already, "reopening reuses the reservation");
        let error = s.execute("stencil.open", &json!({"path":second})).unwrap_err().to_string();
        assert!(error.contains("retained XML node limit: found 4000001, limit 4000000"), "{error}");
        assert!(error.contains(&format!("already open {already}")), "{error}");
        assert_eq!(s.stencils.len(), 1);
        assert_eq!(s.retained_xml_nodes(), already);
        s.execute("file.new", &json!({})).unwrap();
        assert_eq!(s.retained_xml_nodes(), nodes, "new document releases the old document reservation");
        s.execute("stencil.open", &json!({"path":second})).unwrap();
        assert_eq!(s.retained_xml_nodes(), 2 * nodes, "distinct paths retain distinct documents");

        let drawing = directory.join("drawing.vsdx");
        let drawing_bytes = misterdiagram_vsdx::write(&crate::files::blank_document(false)).unwrap().bytes;
        let drawing_nodes = misterdiagram_vsdx::import(&drawing_bytes).unwrap().retained_xml_nodes;
        std::fs::write(&drawing, drawing_bytes).unwrap();
        let mut reverse = Session::new();
        reverse.execute("stencil.open", &json!({"path":first})).unwrap();
        Arc::make_mut(&mut reverse.stencils[0].document).retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES - drawing_nodes + 1;
        let before = reverse.retained_xml_nodes();
        let error = reverse.execute("file.open", &json!({"path":drawing})).unwrap_err().to_string();
        assert!(error.contains(&format!("already open {before}")), "{error}");
        assert_eq!(reverse.retained_xml_nodes(), before);
        assert!(reverse.path.is_none());
        Arc::make_mut(&mut reverse.stencils[0].document).retained_xml_nodes -= 1;
        reverse.execute("file.open", &json!({"path":drawing})).unwrap();
        assert_eq!(reverse.retained_xml_nodes(), misterdiagram_vsdx::MAX_RETAINED_XML_NODES);
        std::fs::remove_dir_all(directory).unwrap();
    }
}
