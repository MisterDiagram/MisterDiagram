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
/// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
pub(crate) fn open(s: &mut Session, p: &Value) -> Result<Value> {
    let builtin = p.get("builtin").and_then(Value::as_str);
    if builtin.is_some() && p.get("path").is_some() {
        return Err(operation("supply path or builtin, not both"));
    }
    if builtin.is_some_and(|name| name != "network") {
        return Err(operation("unknown built-in stencil"));
    }
    let path = if builtin.is_some() { PathBuf::from("builtin:network") } else { crate::files::path(p, "path")?.canonicalize().map_err(operation)? };
    if let Some(index) = s.stencils.iter().position(|stencil| stencil.path == path) {
        return Ok(json!({"stencil":index+1,"masters":s.stencils.get(index).map(|s|s.document.masters.len())}));
    }
    if s.stencils.len() >= 64 {
        return Err(crate::retention::fixed_admission_error("stencil count", 1, s.stencils.len(), 64, "stencil"));
    }
    let document = if builtin.is_some() { crate::network::document()? } else { s.read_document(&path, true)? };
    let held_nodes = s.retained_xml_nodes();
    if held_nodes.saturating_add(document.retained_xml_nodes) > misterdiagram_vsdx::MAX_RETAINED_XML_NODES {
        return Err(crate::retention::fixed_admission_error(
            "XML node",
            document.retained_xml_nodes,
            held_nodes,
            misterdiagram_vsdx::MAX_RETAINED_XML_NODES,
            "nodes",
        ));
    }
    if !document.parts.get(&document.main_part).and_then(|p| p.content_type.as_deref()).is_some_and(|v| v.contains("stencil")) {
        return Err(operation("file is not a stencil"));
    }
    s.reserve(Some((&document, true)))?;
    let count = document.masters.len();
    s.stencils.push(Stencil { path, document: Arc::new(document) });
    Ok(json!({"stencil":s.stencils.len(),"masters":count}))
}
/// Open stencils survive file.new and file.open; index zero is the document stencil.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.documents
pub(crate) fn list(s: &mut Session, _: &Value) -> Result<Value> {
    let mut rows = vec![
        json!({"stencil":0,"name":"Document stencil","path":s.path.as_ref().map(|path| path.to_string_lossy()),"masters":s.document.masters.len()}),
    ];
    rows.extend(s.stencils.iter().enumerate().map(|(index, stencil)| {
        json!({"stencil":index+1,"name":stencil.path.file_name().unwrap_or(stencil.path.as_os_str()).to_string_lossy(),"path":stencil.path.to_string_lossy(),"masters":stencil.document.masters.len()})
    }));
    Ok(json!({"stencils":rows}))
}
/// Close an external stencil. Later external indices shift down by one.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.document.close
pub(crate) fn close(s: &mut Session, p: &Value) -> Result<Value> {
    let index =
        usize::try_from(uint(p, "stencil")?).map_err(operation)?.checked_sub(1).ok_or_else(|| operation("cannot close the document stencil"))?;
    if s.stencils.get(index).is_none() {
        return Err(operation("stencil not found"));
    }
    s.stencils.remove(index);
    Ok(json!({"closed":index+1}))
}
pub(crate) fn document(s: &Session, p: &Value) -> Result<Arc<Document>> {
    match p.get("stencil") {
        None => Ok(s.document.clone()),
        Some(value) if value.as_u64() == Some(0) => Ok(s.document.clone()),
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
fn entries(
    document: &Document,
    stencil: usize,
    query: Option<&str>,
    limit: usize,
    page_scale: Option<crate::master_scale::DrawingScale>,
) -> Result<Value> {
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
        let width = dimension(&mut e, "Width")?;
        let height = dimension(&mut e, "Height")?;
        let scale = crate::master_scale::DrawingScale::read(&mut e, Scope::Master(master.id))?;
        let drop_size = if let Some(page_scale) = page_scale {
            let factor = scale.factor(page_scale)?;
            let scaled = |v: Option<f64>| -> Result<Option<f64>> {
                v.map(|v| {
                    let n = v * factor;
                    if n.is_finite() && n.abs() <= 1e6 { Ok(n) } else { Err(operation("computed master dimension limit")) }
                })
                .transpose()
            };
            json!({"width":scaled(width)?,"height":scaled(height)?,"factor":factor})
        } else {
            json!("page-dependent")
        };
        result.push(json!({"stencil":stencil,"id":master.id,"name":master.name,"name_u":master.name_u,"width":width,"height":height,"scale":scale.json(),"drop_size":drop_size,"keywords":keywords}));
    }
    Ok(json!({"masters":result,"matched":matched}))
}
fn current_scale(s: &Session) -> Result<Option<crate::master_scale::DrawingScale>> {
    s.document
        .pages
        .get(s.selection.page)
        .map(|page| {
            let mut e = Evaluator::new(&s.document).map_err(operation)?;
            crate::master_scale::DrawingScale::read(&mut e, Scope::Page(page.id))
        })
        .transpose()
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.document.masters
pub(crate) fn masters(s: &mut Session, p: &Value) -> Result<Value> {
    let limit = p.get("limit").map(|_| uint(p, "limit")).transpose()?.unwrap_or(1000).min(1000) as usize;
    entries(document(s, p)?.as_ref(), p.get("stencil").and_then(Value::as_u64).unwrap_or(0) as usize, None, limit, current_scale(s)?)
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shapekeywords-cell-miscellaneous-section
pub(crate) fn search(s: &mut Session, p: &Value) -> Result<Value> {
    let query = name(p, "query")?;
    let limit = p.get("limit").map(|_| uint(p, "limit")).transpose()?.unwrap_or(1000).min(1000) as usize;
    let page_scale = current_scale(s)?;
    if p.get("stencil").is_some() {
        let index = usize::try_from(uint(p, "stencil")?).map_err(operation)?;
        return entries(document(s, p)?.as_ref(), index, Some(query), limit, page_scale);
    }
    let mut masters = Vec::new();
    let mut matched = 0usize;
    for (index, doc) in std::iter::once(&s.document).chain(s.stencils.iter().map(|stencil| &stencil.document)).enumerate() {
        let result = entries(doc, index, Some(query), limit.saturating_sub(masters.len()), page_scale)?;
        matched = matched.saturating_add(result.get("matched").and_then(Value::as_u64).unwrap_or(0) as usize);
        if let Some(rows) = result.get("masters").and_then(Value::as_array) {
            masters.extend(rows.iter().cloned());
        }
    }
    Ok(json!({"masters":masters,"matched":matched}))
}

#[cfg(test)]
mod retained_tests {
    use super::*;
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
    #[test]
    fn builtin_node_refusal_names_fixed_bound_and_preserves_inventory() {
        let mut s = Session::with_memory_limits(crate::MemoryLimits::from_mib(8192).unwrap());
        Arc::make_mut(&mut s.document).retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES;
        let before = s.document.clone();
        let error = s.execute("stencil.open", &json!({"builtin":"network"})).unwrap_err().to_string();
        assert!(error.contains("limit 4000000"), "{error}");
        assert!(error.contains("4000000 already retained"), "{error}");
        assert!(error.contains("needs "), "{error}");
        assert!(error.contains("--memory-limit and MISTERDIAGRAM_MEMORY_LIMIT do not change this fixed admission bound"), "{error}");
        assert!(s.stencils.is_empty());
        assert!(Arc::ptr_eq(&before, &s.document));
    }
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
    #[test]
    fn stencil_count_refusal_names_fixed_bound_and_preserves_inventory() {
        let mut s = Session::new();
        for i in 0..64 {
            s.stencils.push(Stencil { path: format!("original-count-{i}").into(), document: s.document.clone() });
        }
        let error = s.execute("stencil.open", &json!({"builtin":"network"})).unwrap_err().to_string();
        assert!(error.contains("needs 1 stencil, 64 already retained, limit 64"), "{error}");
        assert!(error.contains("--memory-limit and MISTERDIAGRAM_MEMORY_LIMIT do not change this fixed admission bound"), "{error}");
        assert_eq!(s.stencils.len(), 64);
    }
    #[cfg(unix)]
    #[test]
    fn stencil_listing_displays_non_utf8_paths_without_panicking() {
        use std::os::unix::ffi::OsStringExt;
        let path = PathBuf::from(std::ffi::OsString::from_vec(b"original-\xff.vssx".to_vec()));
        let mut s = Session::new();
        s.path = Some(path.clone());
        s.stencils.push(Stencil { path, document: s.document.clone() });
        let result = list(&mut s, &json!({})).unwrap();
        assert_eq!(result["stencils"][0]["path"], "original-\u{fffd}.vssx");
        assert_eq!(result["stencils"][1]["path"], "original-\u{fffd}.vssx");
        assert_eq!(result["stencils"][1]["name"], "original-\u{fffd}.vssx");
    }
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
