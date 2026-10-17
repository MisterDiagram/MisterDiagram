//! Page and layer commands.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.pages
use crate::{Result, Session, edit::*, operation};
use misterdiagram_model::{Document, Section, Shape};
use serde_json::{Value, json};

/// https://learn.microsoft.com/en-us/office/vba/api/visio.pages.add
pub(crate) fn add(s: &mut Session, p: &Value) -> Result<Value> {
    let title = name(p, "name")?.to_owned();
    let metric = match p.get("metric") {
        None => false,
        Some(Value::Bool(v)) => *v,
        _ => return Err(operation("metric must be boolean")),
    };
    s.edit_document(|doc, selection| {
        if doc.pages.len() >= 100_000 || doc.pages.iter().any(|v| v.name_u.eq_ignore_ascii_case(&title)) {
            return Err(operation("page count limit or duplicate name"));
        }
        let id = doc.pages.iter().map(|p| p.id).max().map_or(Some(0), |v| v.checked_add(1)).ok_or_else(|| operation("page ID overflow"))?;
        doc.pages.push(misterdiagram_model::skeleton::page(id, &title, metric));
        selection.page = doc.pages.len() - 1;
        selection.shapes.clear();
        Ok(json!({"page":selection.page+1,"id":id}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.delete
pub(crate) fn delete(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    s.edit_document(|doc, selection| {
        if doc.pages.len() <= 1 {
            return Err(operation("cannot delete the last page"));
        }
        let id = doc.pages.get(index).ok_or_else(|| operation("page not found"))?.id.to_string();
        doc.pages.remove(index);
        for page in &mut doc.pages {
            if page.attributes.get("BackPage") == Some(&id) {
                page.attributes.remove("BackPage");
            }
        }
        if selection.page == index {
            selection.page = index.min(doc.pages.len() - 1);
            selection.shapes.clear();
        } else if selection.page > index {
            selection.page -= 1;
        }
        if let Some(settings) = doc.document_settings.as_mut()
            && settings.attr("TopPage") == Some(id.as_str())
            && let Some(first) = doc.pages.first()
        {
            settings.attributes.insert("TopPage".into(), first.id.to_string());
        }
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.nameu
pub(crate) fn rename(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let title = name(p, "name")?.to_owned();
    s.edit_document(|doc, _| {
        if doc.pages.iter().enumerate().any(|(i, v)| i != index && v.name_u.eq_ignore_ascii_case(&title)) {
            return Err(operation("duplicate page name"));
        }
        let page = page_mut(doc, index)?;
        page.name = title.clone();
        page.name_u = title;
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.index
pub(crate) fn reorder(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let destination = usize::try_from(uint(p, "index")?).map_err(operation)?.checked_sub(1).ok_or_else(|| operation("index is one-based"))?;
    if s.document.pages.get(index).is_some_and(|p| p.attributes.get("Background").is_some_and(|v| v == "1" || v == "true")) {
        return Err(operation("background pages cannot be reordered"));
    }
    if destination >= s.document.pages.len() {
        return Err(operation("destination page index does not exist"));
    }
    s.edit_document(|doc, selection| {
        let selected = doc.pages.get(selection.page).map(|p| p.id);
        let page = doc.pages.remove(index);
        doc.pages.insert(destination, page);
        selection.page = doc.pages.iter().position(|p| Some(p.id) == selected).unwrap_or(0);
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagewidth-cell-page-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pageheight-cell-page-properties-section
pub(crate) fn size(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let width = number(p, "width")?;
    let height = number(p, "height")?;
    if width <= 0. || height <= 0. {
        return Err(operation("page dimensions must be positive"));
    }
    s.edit_document(|doc, _| {
        let page = page_mut(doc, index)?;
        set_number(&mut page.sheet, "PageWidth", width)?;
        set_number(&mut page.sheet, "PageHeight", height)?;
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagescale-cell-page-properties-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/drawingscale-cell-page-properties-section
pub(crate) fn scale(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let page_scale = number(p, "page_scale")?;
    let drawing_scale = number(p, "drawing_scale")?;
    if page_scale <= 0. || drawing_scale <= 0. {
        return Err(operation("scale values must be positive"));
    }
    s.edit_document(|doc, _| {
        let sheet = &mut page_mut(doc, index)?.sheet;
        set_number(sheet, "PageScale", page_scale)?;
        set_number(sheet, "DrawingScale", drawing_scale)?;
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.backpage
pub(crate) fn background(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let background = match p.get("background") {
        None | Some(Value::Null) => None,
        _ => Some(usize::try_from(uint(p, "background")?).map_err(operation)?.checked_sub(1).ok_or_else(|| operation("background is one-based"))?),
    };
    if background == Some(index) {
        return Err(operation("page cannot be its own background"));
    }
    if let Some(background) = background {
        let source = s.document.pages.get(index).ok_or_else(|| operation("page not found"))?.id;
        let mut current = s.document.pages.get(background).ok_or_else(|| operation("background not found"))?;
        let mut seen = std::collections::BTreeSet::new();
        loop {
            if current.id == source || !seen.insert(current.id) || seen.len() > 128 {
                return Err(operation("background cycle or depth limit"));
            }
            let Some(next) = current.attributes.get("BackPage") else { break };
            let id = next.parse::<u32>().map_err(operation)?;
            current = s.document.pages.iter().find(|p| p.id == id).ok_or_else(|| operation("background page not found"))?;
        }
    }
    s.edit_document(|doc, _| {
        let target = if let Some(i) = background {
            let page = page_mut(doc, i)?;
            page.attributes.insert("Background".into(), "1".into());
            Some(page.id.to_string())
        } else {
            None
        };
        let page = page_mut(doc, index)?;
        if let Some(id) = target {
            page.attributes.insert("BackPage".into(), id);
        } else {
            page.attributes.remove("BackPage");
        }
        Ok(json!({}))
    })
}
fn layer_section(doc: &mut Document, index: usize) -> Result<&mut Section> {
    page_mut(doc, index)?.sheet.sections.iter_mut().find(|sec| sec.name == "Layer" && !sec.deleted).ok_or_else(|| operation("page has no layers"))
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.layers.add
pub(crate) fn layer_add(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let title = name(p, "name")?.to_owned();
    s.edit_document(|doc, _| {
        let page = page_mut(doc, index)?;
        if !page.sheet.sections.iter().any(|sec| sec.name == "Layer" && !sec.deleted) {
            if page.sheet.sections.len() >= 16384 {
                return Err(operation("section limit"));
            }
            page.sheet.sections.push(Section { name: "Layer".into(), ..Default::default() });
        }
        let sec = layer_section(doc, index)?;
        if sec
            .rows
            .iter()
            .any(|r| !r.deleted && r.cells.iter().any(|c| c.name == "Name" && c.value.as_ref().is_some_and(|v| v.eq_ignore_ascii_case(&title))))
        {
            return Err(operation("duplicate layer name"));
        }
        let id = sec
            .rows
            .iter()
            .enumerate()
            .map(|(i, r)| r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)))
            .max()
            .map_or(Some(0), |v| v.checked_add(1))
            .ok_or_else(|| operation("layer index overflow"))?;
        let mut row = misterdiagram_model::skeleton::layer(id, &title, true).map_err(operation)?;
        // Layers.Add defaults, observed in generated/basic/0001-probe.sheet.tsv.
        for c in &mut row.cells {
            match c.name.as_str() {
                "Color" => c.value = Some("255".into()),
                "Snap" | "Glue" => c.value = Some("1".into()),
                _ => {}
            }
        }
        sec.rows.push(row);
        Ok(json!({"layer":id}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.layer.name
pub(crate) fn layer_rename(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let id = uint(p, "layer")?;
    let title = name(p, "name")?.to_owned();
    s.edit_document(|doc, _| {
        let sec = layer_section(doc, index)?;
        if sec.rows.iter().enumerate().any(|(i, r)| {
            r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)) != id
                && !r.deleted
                && r.cells.iter().any(|c| c.name == "Name" && c.value.as_ref().is_some_and(|v| v.eq_ignore_ascii_case(&title)))
        }) {
            return Err(operation("duplicate layer name"));
        }
        let row = sec
            .rows
            .iter_mut()
            .enumerate()
            .find(|(i, r)| r.index.unwrap_or(u32::try_from(*i).unwrap_or(u32::MAX)) == id && !r.deleted)
            .map(|(_, r)| r)
            .ok_or_else(|| operation("layer not found"))?;
        let cell = row.cells.iter_mut().find(|c| c.name == "Name").ok_or_else(|| operation("layer has no name"))?;
        cell.value = Some(title);
        cell.formula = None;
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/layermember-cell-layer-membership-section
pub(crate) fn layer_assign(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let layers = p.get("layers").and_then(Value::as_array).filter(|a| a.len() <= 16384).ok_or_else(|| operation("layers must be a bounded array"))?;
    let ids = layers
        .iter()
        .map(|v| v.as_u64().and_then(|v| u32::try_from(v).ok()).ok_or_else(|| operation("invalid layer index")))
        .collect::<Result<Vec<_>>>()?;
    s.edit_document(|doc, _| {
        if !ids.is_empty() {
            let sec = layer_section(doc, index)?;
            for id in &ids {
                if !sec.rows.iter().enumerate().any(|(i, r)| !r.deleted && r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)) == *id) {
                    return Err(operation("layer not found"));
                }
            }
        }
        let value = ids.iter().map(u32::to_string).collect::<Vec<_>>().join(";");
        set(&mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet, "LayerMember", &value, None)?;
        Ok(json!({}))
    })
}
fn remove_membership(shapes: &mut [Shape], id: u32, depth: usize) -> Result<()> {
    if depth > 128 {
        return Err(operation("shape nesting limit"));
    }
    for shape in shapes {
        if let Some(cell) = shape.sheet.cells.iter_mut().find(|c| c.name == "LayerMember") {
            let mut ids = Vec::new();
            for token in cell.value.as_deref().unwrap_or("").split(';').filter(|s| !s.is_empty()) {
                let value = token.parse::<u32>().map_err(operation)?;
                if value != id {
                    ids.push(if value > id { value - 1 } else { value }.to_string());
                }
            }
            cell.value = Some(ids.join(";"));
            cell.formula = None;
        }
        remove_membership(&mut shape.shapes, id, depth + 1)?;
    }
    Ok(())
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.layer.delete
pub(crate) fn layer_delete(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let id = uint(p, "layer")?;
    s.edit_document(|doc, _| {
        let sec = layer_section(doc, index)?;
        let position = sec
            .rows
            .iter()
            .enumerate()
            .position(|(i, r)| !r.deleted && r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)) == id)
            .ok_or_else(|| operation("layer not found"))?;
        sec.rows.remove(position);
        for (i, row) in sec.rows.iter_mut().enumerate() {
            let old = row.index.unwrap_or(u32::try_from(i).map_err(operation)?);
            if old > id {
                row.index = Some(old - 1);
            }
        }
        remove_membership(&mut page_mut(doc, index)?.shapes, id, 0)?;
        Ok(json!({}))
    })
}
