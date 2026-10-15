//! Text, shape data, hyperlinks and formatting commands.
use crate::{Result, Session, edit::*, operation};
use diagramcraft_model::{Cell, Document, Row, Section, Sheet, XmlElement, XmlNode};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn text<'a>(p: &'a Value, key: &str, default: &'a str) -> Result<&'a str> {
    match p.get(key) {
        None => Ok(default),
        Some(Value::String(v)) if v.len() <= 65536 && !v.contains('\0') => Ok(v),
        _ => Err(operation(format!("invalid {key}"))),
    }
}
fn string_cell(name: &str, value: &str) -> Cell {
    let formula = format!("\"{}\"", value.replace('"', "\"\""));
    Cell { units: Some("STR".into()), ..cell(name, value, Some(&formula)) }
}
fn property_string(name: &str, value: &str) -> Cell {
    // Literal string values have no formula; only Property.Value has U=STR.
    // Witness: generated/data/property-rows.vsdx.
    Cell { units: (name == "Value").then(|| "STR".into()), ..cell(name, value, None) }
}
fn section<'a>(sheet: &'a mut Sheet, name: &str) -> Result<&'a mut Section> {
    let at = if let Some(i) = sheet.sections.iter().position(|s| s.name == name && !s.deleted) {
        i
    } else {
        if sheet.sections.len() >= 16384 {
            return Err(operation("section limit"));
        }
        sheet.sections.push(Section { name: name.into(), ..Default::default() });
        sheet.sections.len() - 1
    };
    sheet.sections.get_mut(at).ok_or_else(|| operation("section missing"))
}
fn row_name(p: &Value) -> Result<&str> {
    name(p, "name")?
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
        .then(|| name(p, "name"))
        .ok_or_else(|| operation("invalid universal row name"))?
}
/// Full authored row observed in generated/data/property-rows.vsdx.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/langid-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/type-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/label-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/format-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/invisible-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sortkey-cell-shape-data-section
pub(crate) fn define(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let name = row_name(p)?.to_owned();
    let kind = p.get("type").map(|_| uint(p, "type")).transpose()?.unwrap_or(0);
    if kind > 7 {
        return Err(operation("property type must be 0 through 7"));
    }
    let label = text(p, "label", &name)?;
    let format = text(p, "format", "")?;
    let prompt = text(p, "prompt", "")?;
    let sort_key = text(p, "sort_key", "")?;
    let invisible = p.get("invisible").and_then(Value::as_bool).unwrap_or(false);
    let value = if matches!(kind, 0 | 1 | 4) {
        property_string("Value", if kind == 1 { format.split(';').next().unwrap_or("") } else { "" })
    } else {
        cell("Value", "0", None)
    };
    let cells = vec![
        value,
        property_string("Label", label),
        cell("Type", &kind.to_string(), None),
        property_string("Format", format),
        cell("Invisible", if invisible { "1" } else { "0" }, None),
        property_string("SortKey", sort_key),
        property_string("Prompt", prompt),
        cell("Verify", "0", None),
        cell("DataLinked", "0", None),
        cell("LangID", "en-US", None),
        cell("Calendar", "0", None),
    ];
    s.edit_document(|doc, _| {
        let sec = section(&mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet, "Property")?;
        if sec.rows.len() >= 65536 || sec.rows.iter().any(|r| r.name.as_ref().is_some_and(|v| v.eq_ignore_ascii_case(&name))) {
            return Err(operation("property already exists or row limit"));
        }
        sec.rows.push(Row { name: Some(name), cells, ..Default::default() });
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/value-cell-shape-data-section
pub(crate) fn set_data(s: &mut Session, p: &Value) -> Result<Value> {
    let (_, _, scope) = shape_scope(s, p)?;
    let name = row_name(p)?;
    let value = p.get("value").ok_or_else(|| operation("missing value"))?;
    let cell_name = format!("Prop.{name}");
    let mut value_address = crate::sheet::address(&s.document, scope, &cell_name)?;
    let mut e = diagramcraft_vsdx::evaluate::Evaluator::new(&s.document).map_err(operation)?;
    let kind = e.evaluate(scope, &format!("{cell_name}.Type")).numeric().map_err(operation)?;
    if !(0. ..=7.).contains(&kind) || kind.fract() != 0. {
        return Err(operation("invalid property type"));
    }
    let kind = kind as u32;
    let mut format_change = None;
    let new = match kind {
        0 | 1 | 4 => {
            let value = value.as_str().filter(|v| v.len() <= 65536).ok_or_else(|| operation("property requires a bounded string"))?;
            if kind == 1 || kind == 4 {
                let format_name = format!("{cell_name}.Format");
                let old = e.evaluate(scope, &format_name).text();
                if !old.split(';').any(|item| item == value) {
                    if kind == 1 {
                        return Err(operation("value is not in the fixed list"));
                    }
                    if old.len().saturating_add(value.len()).saturating_add(1) > 65536 {
                        return Err(operation("list format limit"));
                    }
                    let mut address = crate::sheet::address(&s.document, scope, &format_name)?;
                    address.cell = property_string("Format", &if old.is_empty() { value.into() } else { format!("{old};{value}") });
                    format_change = Some(address);
                }
            }
            property_string("Value", value)
        }
        3 => cell("Value", if value.as_bool().ok_or_else(|| operation("property requires boolean"))? { "1" } else { "0" }, None),
        _ => {
            let n = value.as_f64().filter(|n| n.is_finite()).ok_or_else(|| operation("property requires numeric internal units"))?;
            cell("Value", &n.to_string(), None)
        }
    };
    value_address.cell = new;
    s.edit_document(|doc, _| {
        let sheet = crate::sheet::sheet_mut(doc, scope)?;
        crate::sheet::install(sheet, value_address)?;
        if let Some(address) = format_change {
            crate::sheet::install(sheet, address)?;
        }
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.hyperlinks.add
/// https://learn.microsoft.com/en-us/office/client-developer/visio/address-cell-hyperlinks-section
pub(crate) fn hyperlink(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let name = row_name(p)?.to_owned();
    let address = text(p, "address", "")?;
    let sub = text(p, "sub_address", "")?;
    let description = text(p, "description", "")?;
    if address.is_empty() && sub.is_empty() {
        return Err(operation("hyperlink requires address or sub_address"));
    }
    let cells = vec![
        string_cell("Address", address),
        string_cell("SubAddress", sub),
        string_cell("Description", description),
        cell("NewWindow", if p.get("new_window").and_then(Value::as_bool).unwrap_or(false) { "1" } else { "0" }, None),
        cell("Default", "0", None),
        cell("Invisible", "0", None),
    ];
    s.edit_document(|doc, _| {
        let sec = section(&mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet, "Hyperlink")?;
        let row = Row { name: Some(name.clone()), cells, ..Default::default() };
        if let Some(old) = sec.rows.iter_mut().find(|r| r.name.as_ref() == Some(&name)) {
            *old = row;
        } else {
            if sec.rows.len() >= 65536 {
                return Err(operation("hyperlink row limit"));
            }
            sec.rows.push(row);
        }
        Ok(json!({}))
    })
}
fn color(value: &str, name: &str) -> Result<Cell> {
    let hex = value
        .strip_prefix('#')
        .filter(|v| v.len() == 6 && v.bytes().all(|b| b.is_ascii_hexdigit()))
        .ok_or_else(|| operation("color must be #RRGGBB"))?;
    let rgb = u32::from_str_radix(hex, 16).map_err(operation)?;
    Ok(cell(name, &format!("#{hex}"), Some(&format!("RGB({},{},{})", (rgb >> 16) & 255, (rgb >> 8) & 255, rgb & 255))))
}
fn font(doc: &mut Document, name: &str) -> Result<Cell> {
    if name.is_empty() || name.len() > 1024 {
        return Err(operation("invalid font family"));
    }
    if !doc.face_names.values().any(|v| v.eq_ignore_ascii_case(name)) {
        if doc.face_names.len() >= 65536 {
            return Err(operation("font count limit"));
        }
        let id = doc.face_names.keys().max().map_or(Some(0), |v| v.checked_add(1)).ok_or_else(|| operation("font ID limit"))?;
        doc.face_names.insert(id, name.into());
    }
    let ordinal = doc.face_names.values().position(|v| v.eq_ignore_ascii_case(name)).ok_or_else(|| operation("font missing"))?;
    Ok(cell("Font", &ordinal.to_string(), None))
}
fn character(doc: &mut Document, p: &Value, old_style: u32) -> Result<Vec<Cell>> {
    let mut cells = vec![];
    if let Some(v) = p.get("font") {
        cells.push(font(doc, v.as_str().ok_or_else(|| operation("font must be a family name"))?)?);
    }
    if p.get("size").is_some() {
        let size = number(p, "size")?;
        if !(0. ..=10000.).contains(&size) || size == 0. {
            return Err(operation("font size must be in (0,10000] points"));
        }
        cells.push(cell("Size", &(size / 72.).to_string(), None));
    }
    if let Some(v) = p.get("color") {
        cells.push(color(v.as_str().ok_or_else(|| operation("invalid color"))?, "Color")?);
    }
    let mut style = old_style;
    for (key, bit) in [("bold", 1), ("italic", 2), ("underline", 4)] {
        if let Some(v) = p.get(key) {
            if v.as_bool().ok_or_else(|| operation("style flag must be boolean"))? {
                style |= bit;
            } else {
                style &= !bit;
            }
        }
    }
    if ["bold", "italic", "underline"].iter().any(|key| p.get(key).is_some()) {
        cells.push(cell("Style", &style.to_string(), None));
    }
    Ok(cells)
}
fn element(name: &str, attributes: BTreeMap<String, String>, children: Vec<XmlNode>) -> XmlElement {
    XmlElement {
        name: name.into(),
        namespace: "http://schemas.microsoft.com/office/visio/2012/main".into(),
        attributes,
        children,
        ..Default::default()
    }
}
/// Formatting markers are recorded even when the format equals the inherited value.
/// Originals: master/from-shape; formatted character marker: SAN Visio re-save.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/text-element-shapesheet_type-complextypevisio-xml
fn text_marker(shape: &mut diagramcraft_model::Shape, name: &str) {
    if let Some(text) = &mut shape.text {
        if text.children.iter().any(|n| matches!(n, XmlNode::Element(e) if e.name == name)) {
            return;
        }
        let at = if name == "pp" && matches!(text.children.first(), Some(XmlNode::Element(e)) if e.name == "cp") { 1 } else { 0 };
        text.children.insert(at, XmlNode::Element(element(name, BTreeMap::from([("IX".into(), "0".into())]), vec![])));
    }
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.text
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cp-element-text_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pp-element-text_type-complextypevisio-xml
pub(crate) fn shape_text(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    if p.get("text").is_some() == p.get("paragraphs").is_some() {
        return Err(operation("supply text or paragraphs"));
    }
    s.edit_document(|doc, _| {
        let mut chars = vec![];
        let mut paras = vec![];
        let mut children = vec![];
        let structured = p.get("paragraphs").is_some();
        if let Some(paragraphs) = p.get("paragraphs") {
            let paragraphs = paragraphs
                .as_array()
                .filter(|a| !a.is_empty() && a.len() <= 1024)
                .ok_or_else(|| operation("paragraphs must contain 1 through 1024 items"))?;
            for (i, para) in paragraphs.iter().enumerate() {
                let object = para.as_object().ok_or_else(|| operation("paragraph must be an object"))?;
                if object.keys().any(|k| !matches!(k.as_str(), "runs" | "align")) {
                    return Err(operation("unknown paragraph property"));
                }
                let align = para.get("align").map(|_| uint(para, "align")).transpose()?.unwrap_or(1);
                if align > 4 {
                    return Err(operation("invalid paragraph alignment"));
                }
                paras.push(Row {
                    index: Some(u32::try_from(i).map_err(operation)?),
                    cells: vec![cell("HorzAlign", &align.to_string(), None)],
                    ..Default::default()
                });
                children.push(XmlNode::Element(element("pp", BTreeMap::from([("IX".into(), i.to_string())]), vec![])));
                let runs = para
                    .get("runs")
                    .and_then(Value::as_array)
                    .filter(|a| !a.is_empty() && a.len() <= 1024)
                    .ok_or_else(|| operation("paragraph needs bounded runs"))?;
                for run in runs {
                    if chars.len() >= 4096 {
                        return Err(operation("character run limit"));
                    }
                    let object = run.as_object().ok_or_else(|| operation("run must be an object"))?;
                    if object.keys().any(|k| !matches!(k.as_str(), "text" | "font" | "size" | "color" | "bold" | "italic" | "underline")) {
                        return Err(operation("unknown character property"));
                    }
                    let value = text(run, "text", "")?;
                    let n = chars.len();
                    let cells = character(doc, run, 0)?;
                    chars.push(Row { index: Some(u32::try_from(n).map_err(operation)?), cells, ..Default::default() });
                    children.push(XmlNode::Element(element("cp", BTreeMap::from([("IX".into(), n.to_string())]), vec![])));
                    children.push(XmlNode::Text(value.into()));
                }
                children.push(XmlNode::Text("\n".into()));
            }
        } else {
            let value = text(p, "text", "")?;
            children.push(XmlNode::Text(if value.is_empty() || value.ends_with('\n') { value.into() } else { format!("{value}\n") }));
        }
        let shape = shape_mut(&mut page_mut(doc, index)?.shapes, id)?;
        shape.text = Some(element("Text", BTreeMap::new(), children));
        if structured {
            shape.sheet.sections.retain(|s| !matches!(s.name.as_str(), "Character" | "Paragraph"));
            shape.sheet.sections.push(Section { name: "Character".into(), rows: chars, ..Default::default() });
            shape.sheet.sections.push(Section { name: "Paragraph".into(), rows: paras, ..Default::default() });
        }
        let mut evaluator = diagramcraft_vsdx::evaluate::Evaluator::new(doc).map_err(operation)?;
        let body = evaluator.text_body(scope).map_err(operation)?;
        diagramcraft_model::text::TextHost::measure(&diagramcraft_text::Host, &body, None).map_err(operation)?;
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillforegnd-cell-fill-format-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillpattern-cell-fill-format-section
pub(crate) fn fill(s: &mut Session, p: &Value) -> Result<Value> {
    format_paint(s, p, false)
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linecolor-cell-line-format-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
pub(crate) fn line(s: &mut Session, p: &Value) -> Result<Value> {
    format_paint(s, p, true)
}
fn format_paint(s: &mut Session, p: &Value, line: bool) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let mut cells = vec![];
    if let Some(v) = p.get("color") {
        cells.push(color(v.as_str().ok_or_else(|| operation("invalid color"))?, if line { "LineColor" } else { "FillForegnd" })?);
    }
    for (key, name, max) in
        [("pattern", if line { "LinePattern" } else { "FillPattern" }, 255), ("begin_arrow", "BeginArrow", 45), ("end_arrow", "EndArrow", 45)]
    {
        if p.get(key).is_some() {
            let v = uint(p, key)?;
            if v > max {
                return Err(operation("format value out of range"));
            }
            cells.push(cell(name, &v.to_string(), None));
        }
    }
    for (key, name, max) in [("transparency", if line { "LineColorTrans" } else { "FillForegndTrans" }, 1.), ("weight", "LineWeight", 1000.)] {
        if p.get(key).is_some() {
            let v = number(p, key)?;
            if !(0. ..=max).contains(&v) {
                return Err(operation("format value out of range"));
            }
            cells.push(cell(name, &v.to_string(), None));
        }
    }
    if cells.is_empty() {
        return Err(operation("no format properties supplied"));
    }
    s.edit_document(|doc, _| {
        let sheet = &mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet;
        for c in cells {
            if let Some(old) = sheet.cells.iter_mut().find(|v| v.name == c.name) {
                *old = c;
            } else {
                sheet.cells.push(c);
            }
        }
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/size-cell-character-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/style-cell-character-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/horzalign-cell-paragraph-section
pub(crate) fn format_text(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    if p.get("align").is_some() && uint(p, "align")? > 4 {
        return Err(operation("invalid paragraph alignment"));
    }
    let old_align = evaluated(&s.document, scope, "Para.HorzAlign")?;
    let old = evaluated(&s.document, scope, "Char.Style")?;
    if !(0. ..=u32::MAX as f64).contains(&old) {
        return Err(operation("invalid character style"));
    }
    let mut evaluator = diagramcraft_vsdx::evaluate::Evaluator::new(&s.document).map_err(operation)?;
    let indices = evaluator
        .resolved_named_cells(scope)
        .iter()
        .filter(|c| c.section.is_some_and(|section| section.name == "Character"))
        .filter_map(|c| c.row.map(|r| r.index.unwrap_or(0)))
        .chain([0])
        .collect::<std::collections::BTreeSet<_>>();
    let mut styles = BTreeMap::new();
    for row in indices {
        let name = if row == 0 {
            "Char.Style".into()
        } else {
            format!("Char.Style[{}]", row.checked_add(1).ok_or_else(|| operation("character row limit"))?)
        };
        let value = evaluator.evaluate(scope, &name).numeric().map_err(operation)?;
        if !value.is_finite() || !(0. ..=u32::MAX as f64).contains(&value) {
            return Err(operation("invalid character style"));
        }
        let mut value = value as u32;
        for (key, bit) in [("bold", 1), ("italic", 2), ("underline", 4)] {
            if let Some(flag) = p.get(key).and_then(Value::as_bool) {
                if flag {
                    value |= bit;
                } else {
                    value &= !bit;
                }
            }
        }
        styles.insert(row, value);
    }
    s.edit_document(|doc, _| {
        let cells = character(doc, p, old as u32)?;
        let sheet = &mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet;
        if !cells.is_empty() {
            let sec = section(sheet, "Character")?;
            for index in styles.keys() {
                if !sec.rows.iter().any(|r| r.index.unwrap_or(0) == *index) {
                    sec.rows.push(Row { index: Some(*index), ..Default::default() });
                }
            }
            for row in &mut sec.rows {
                for original in &cells {
                    let mut c = original.clone();
                    if c.name == "Style" {
                        c.value = Some(styles.get(&row.index.unwrap_or(0)).copied().unwrap_or(old as u32).to_string());
                    }
                    if let Some(old) = row.cells.iter_mut().find(|v| v.name == c.name) {
                        *old = c;
                    } else {
                        row.cells.push(c);
                    }
                }
            }
        }

        if p.get("align").is_some() && (f64::from(uint(p, "align")?) != old_align || sheet.sections.iter().any(|section| section.name == "Paragraph"))
        {
            let align = uint(p, "align")?;
            if align > 4 {
                return Err(operation("invalid paragraph alignment"));
            }
            let sec = section(sheet, "Paragraph")?;
            if !sec.rows.iter().any(|r| r.index.unwrap_or(0) == 0) {
                sec.rows.push(Row { index: Some(0), ..Default::default() });
            }
            for row in &mut sec.rows {
                let c = cell("HorzAlign", &align.to_string(), None);
                if let Some(old) = row.cells.iter_mut().find(|v| v.name == c.name) {
                    *old = c;
                } else {
                    row.cells.push(c);
                }
            }
        } else if cells.is_empty() && p.get("align").is_none() {
            return Err(operation("no text format properties supplied"));
        }
        let shape = shape_mut(&mut page_mut(doc, index)?.shapes, id)?;
        if !cells.is_empty() {
            text_marker(shape, "cp");
        }
        if p.get("align").is_some() {
            text_marker(shape, "pp");
        }
        Ok(json!({}))
    })
}
