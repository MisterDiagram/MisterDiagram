//! COM-compatible ShapeSheet TSV output from the parsed model and resolved cells.
use crate::evaluate::{Evaluator, NamedCell, Scope};
use diagramcraft_model::{Cell, Document, Shape};
use diagramcraft_sheet::Value;
use std::{collections::BTreeMap, io::Write};

/// Page selection is one-based ordinal; master and shape selection use Visio IDs.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.pages.item
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selection {
    All,
    Document,
    Page(u32),
    Master(u32),
    Shape(u32),
}

/// Emit the oracle's nine columns without consulting any expected dump at runtime.
/// Page selections retain the document, styles and masters used by that page.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.resultiu
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.formulau
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.name
pub fn dump(document: &Document, selection: Selection, out: &mut impl Write) -> Result<(), String> {
    dump_with_theme(document, selection, None, out)
}
/// Emit a dump using the application-provided theme service.
pub fn dump_with_theme(
    document: &Document,
    selection: Selection,
    host: Option<&dyn diagramcraft_sheet::host::ThemeHost>,
    out: &mut impl Write,
) -> Result<(), String> {
    dump_with_hosts(document, selection, host, None, out)
}
/// Emit a dump using injected theme and text services.
pub fn dump_with_hosts(
    document: &Document,
    selection: Selection,
    host: Option<&dyn diagramcraft_sheet::host::ThemeHost>,
    text: Option<&dyn diagramcraft_model::text::TextHost>,
    out: &mut impl Write,
) -> Result<(), String> {
    let mut evaluator = Evaluator::new(document).map_err(|e| e.to_string())?;
    if let Some(host) = host {
        evaluator = evaluator.with_theme_host(host);
    }
    if let Some(host) = text {
        evaluator = evaluator.with_text_host(host);
    }
    let sheets = selected(document, selection)?;
    writeln!(out, "shape_id\tshape_name\tsection\trow\tcell\tname\tformula\tresult_iu\tunits").map_err(io_error)?;
    for (scope, id, identity) in sheets {
        dump_sheet(&mut evaluator, scope, id, &identity, out)?;
    }
    Ok(())
}
fn io_error(e: std::io::Error) -> String {
    format!("cannot write ShapeSheet dump: {e}")
}
fn selected(doc: &Document, selection: Selection) -> Result<Vec<(Scope, u32, String)>, String> {
    let page = match selection {
        Selection::Page(n) => Some(
            doc.pages
                .get(usize::try_from(n.checked_sub(1).ok_or("page ordinal must start at 1")?).map_err(|_| "page ordinal too large")?)
                .ok_or("page ordinal not found")?
                .id,
        ),
        _ => None,
    };
    let mut result = SelectionList::default();
    if matches!(selection, Selection::All | Selection::Document | Selection::Page(_)) {
        result.push((Scope::Document, 0, "document/TheDoc".into()))?;
    }
    if matches!(selection, Selection::All | Selection::Page(_)) {
        for style in &doc.styles {
            if name(&style.name_u, &style.name).len() > 65536 {
                return Err("dump identity limit".into());
            }
            result.push((Scope::Style(style.id), style.id, format!("style/{}", name(&style.name_u, &style.name))))?;
        }
    }
    if matches!(selection, Selection::All | Selection::Page(_) | Selection::Master(_)) {
        for master in &doc.masters {
            if matches!(selection,Selection::Master(n) if master.id!=n) {
                continue;
            }
            if name(&master.name_u, &master.name).len() > 65536 {
                return Err("dump identity limit".into());
            }
            let prefix = format!("master:{}:{}", master.id, name(&master.name_u, &master.name));
            add_shapes(doc, &master.shapes, &prefix, None, Some(master.id), None, &mut result)?;
        }
    }
    if matches!(selection, Selection::All | Selection::Page(_) | Selection::Shape(_)) {
        for p in &doc.pages {
            if page.is_some_and(|id| id != p.id) {
                continue;
            }
            let pname = name(&p.name_u, &p.name);
            if pname.len() > 65536 {
                return Err("dump identity limit".into());
            }
            if !matches!(selection, Selection::Shape(_)) {
                result.push((Scope::Page(p.id), p.id, format!("pagesheet:{}:{pname}/ThePage", p.id)))?;
            }
            add_shapes(
                doc,
                &p.shapes,
                &format!("page:{}:{pname}", p.id),
                Some(p.id),
                None,
                match selection {
                    Selection::Shape(n) => Some(n),
                    _ => None,
                },
                &mut result,
            )?;
        }
    }
    if result.entries.is_empty() {
        return Err("selected sheet not found".into());
    }
    Ok(result.entries)
}
#[derive(Default)]
struct SelectionList {
    entries: Vec<(Scope, u32, String)>,
    bytes: usize,
}
impl SelectionList {
    fn push(&mut self, value: (Scope, u32, String)) -> Result<(), String> {
        self.bytes = self.bytes.checked_add(value.2.len()).ok_or("dump identity overflow")?;
        if self.entries.len() >= 100_000 || self.bytes > 64 * 1024 * 1024 || value.2.len() > 65536 {
            return Err("dump identity limit".into());
        }
        self.entries.push(value);
        Ok(())
    }
}
fn name<'a>(universal: &'a str, local: &'a str) -> &'a str {
    if universal.is_empty() { local } else { universal }
}
/// Universal shape names default to Sheet.ID, with inherited master names for instances.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.nameu
fn add_shapes(
    doc: &Document,
    shapes: &[Shape],
    prefix: &str,
    page: Option<u32>,
    master: Option<u32>,
    filter: Option<u32>,
    result: &mut SelectionList,
) -> Result<(), String> {
    if shapes.len() > 100_000 || prefix.len() > 65536 {
        return Err("shape traversal limit".into());
    }
    let prefix: std::sync::Arc<str> = prefix.into();
    let mut stack: Vec<_> = shapes.iter().rev().map(|shape| (shape, prefix.clone(), 0usize)).collect();
    let mut count = 0usize;
    let mut path_bytes = prefix.len();
    while let Some((shape, parent, depth)) = stack.pop() {
        count += 1;
        if depth > 128 || count > 100000 || parent.len() > 65536 {
            return Err("shape traversal limit".into());
        }
        if shape.sheet.attributes.get("Del").is_some_and(|s| s == "1") {
            continue;
        }
        if name(&shape.name_u, &shape.name).len() > 65536 {
            return Err("dump identity limit".into());
        }
        let shape_name = if !shape.name_u.is_empty() {
            shape.name_u.clone()
        } else if !shape.name.is_empty() {
            shape.name.clone()
        } else if let Some(m) = shape.master.and_then(|id| doc.masters.iter().find(|m| m.id == id)) {
            if name(&m.name_u, &m.name).len() > 65536 {
                return Err("dump identity limit".into());
            }
            format!("{}.{}", name(&m.name_u, &m.name), shape.id)
        } else {
            format!("Sheet.{}", shape.id)
        };
        path_bytes = path_bytes.checked_add(parent.len().saturating_add(shape_name.len()).saturating_add(1)).ok_or("dump identity overflow")?;
        if path_bytes > 64 * 1024 * 1024 {
            return Err("dump identity limit".into());
        }
        let identity = format!("{parent}/{shape_name}");
        let scope = match (page, master) {
            (Some(page), _) => Scope::PageShape { page, shape: shape.id },
            (_, Some(master)) => Scope::MasterShape { master, shape: shape.id },
            _ => return Err("missing shape scope".into()),
        };
        if filter.is_none_or(|id| id == shape.id) {
            result.push((scope, shape.id, identity.clone()))?;
        }
        if stack.len().saturating_add(shape.shapes.len()) > 100_000 {
            return Err("shape traversal limit".into());
        }
        let identity: std::sync::Arc<str> = identity.into();
        for child in shape.shapes.iter().rev() {
            stack.push((child, identity.clone(), depth + 1));
        }
    }
    Ok(())
}
#[derive(Clone)]
struct Slot<'a> {
    name: String,
    cell: Option<&'a Cell>,
    src: (u16, u16, u16),
    units: u16,
    synthetic: Option<Value>,
}
fn dump_sheet(evaluator: &mut Evaluator<'_>, scope: Scope, id: u32, identity: &str, out: &mut impl Write) -> Result<(), String> {
    let mut slots = BTreeMap::new();
    for named in evaluator.resolved_named_cells(scope) {
        if let Some((s, r, c, mut u)) = metadata(evaluator, scope, &named) {
            if u == 73
                && !(s == 1 && r == 10 && matches!(c, 4 | 5))
                && let Scope::MasterShape { master, .. } = scope
            {
                // A master's drawing units come from its own PageSheet.
                if let Some(cell) = evaluator.sheet(Scope::Master(master)).and_then(|sheet| sheet.cells.iter().find(|c| c.name == "DrawingScale")) {
                    u = unit_code(cell, u);
                }
            }
            // Probe and blank COM Style.Section traversal omits XML gradient sections and LayerMember.
            // https://learn.microsoft.com/en-us/office/vba/api/visio.style.section
            if matches!(scope, Scope::Style(_)) && (matches!(s, 248 | 249) || (s == 1 && r == 6)) {
                continue;
            }
            if let Some(previous) =
                slots.insert((s, r, c), Slot { name: named.name.clone(), cell: Some(named.cell), src: (s, r, c), units: u, synthetic: None })
                && !previous.name.eq_ignore_ascii_case(&named.name)
            {
                return Err(format!("conflicting cells at SRC {s},{r},{c}: {} and {}", previous.name, named.name));
            }
        }
    }
    supplement(&mut slots)?;
    for slot in slots.into_values() {
        if slot.name.len() > 65536 || identity.len() > 65536 {
            return Err("dump text exceeds limit".into());
        }
        if slot.cell.is_some_and(|c| c.value.as_ref().is_some_and(|v| v.len() > 65536) || c.formula.as_ref().is_some_and(|f| f.len() > 65536)) {
            return Err("dump cell text exceeds limit".into());
        }
        let value = slot.synthetic.clone().unwrap_or_else(|| evaluator.evaluate_snapshot(scope, &slot.name));
        let formula = match slot.cell {
            Some(cell) => match evaluator.effective_formula(scope, &slot.name).filter(|f| f != "No Formula" && f != "Inh") {
                Some(f) => rebind_formula(evaluator, scope, &slot.name, &f),
                None => constant_formula(cell, slot.units),
            },
            None => {
                if slot.name.ends_with(".Path") || slot.units == 231 {
                    String::new()
                } else {
                    "0".into()
                }
            }
        };
        let numeric = match value {
            Value::Text(_) => "0".into(),
            Value::Point(..) => "0".into(),
            v => match v.numeric() {
                Ok(v) => v.to_string(),
                Err(e) => e.to_string(),
            },
        };
        writeln!(
            out,
            "{id}\t{}\t{}\t{}\t{}\t{}\t{}\t{numeric}\t{}",
            escape(identity),
            slot.src.0,
            slot.src.1,
            slot.src.2,
            escape(&display_name(&slot.name)),
            escape(&formula),
            slot.units
        )
        .map_err(io_error)?;
    }
    Ok(())
}
fn display_name(name: &str) -> String {
    if let Some(tail) = name.strip_prefix("Layers.Lock")
        && (tail.is_empty() || tail.starts_with('['))
    {
        return format!("Layers.Locked{tail}");
    }
    name.to_owned()
}
fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\t', "\\t").replace('\r', "\\r").replace('\n', "\\n")
}
fn rebind_formula(evaluator: &Evaluator<'_>, scope: Scope, name: &str, formula: &str) -> String {
    // Only reference tokens outside string literals are candidates. Preserve all other spelling.
    let mut result = String::new();
    let mut cursor = 0;
    let mut quoted = false;
    let bytes = formula.as_bytes();
    while cursor < bytes.len() {
        if bytes.get(cursor) == Some(&b'"') {
            quoted = !quoted;
            result.push('"');
            cursor += 1;
            continue;
        }
        if !quoted && formula.get(cursor..).is_some_and(|s| s.starts_with("Sheet.")) {
            let start = cursor;
            cursor += 6;
            while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
                cursor += 1;
            }
            if bytes.get(cursor) == Some(&b'!') {
                cursor += 1;
                let reference = format!("{}Width", formula.get(start..cursor).unwrap_or(""));
                if let Ok(mapped) = evaluator.display_reference(scope, name, &reference) {
                    result.push_str(mapped.strip_suffix("Width").unwrap_or(&mapped));
                    continue;
                }
            }
            result.push_str(formula.get(start..cursor).unwrap_or(""));
            continue;
        }
        let Some(ch) = formula.get(cursor..).and_then(|s| s.chars().next()) else { break };
        result.push(ch);
        cursor += ch.len_utf8();
    }
    result
}
fn constant_formula(cell: &Cell, units: u16) -> String {
    let value = cell.value.as_deref().unwrap_or("");
    if cell.attributes.contains_key("VirtualPath") {
        return String::new();
    }
    if cell.attributes.contains_key("ComSection") {
        return cell.formula.clone().unwrap_or_default();
    }
    if units == 231 || cell.units.as_deref() == Some("STR") {
        return value.to_owned();
    }
    if let Some(hex) = value.strip_prefix('#').filter(|s| s.len() == 6)
        && let Ok(rgb) = u32::from_str_radix(hex, 16)
    {
        return format!("RGB({},{},{})", (rgb >> 16) & 255, (rgb >> 8) & 255, rgb & 255);
    }
    let Ok(n) = value.parse::<f64>() else { return value.to_owned() };
    let (factor, suffix) = match units {
        33 => (100.0, "%"),
        50 => (72.0, " pt"),
        51 => (6.0, " p"),
        65 | 73 => (1.0, " in"),
        66 | 67 => (1.0 / 12.0, " ft"),
        69 => (2.54, " cm"),
        70 => (25.4, " mm"),
        71 => (0.0254, " m"),
        81 => (180.0 / std::f64::consts::PI, " deg"),
        83 => (1.0, " rad"),
        _ => (1.0, ""),
    };
    format!("{}{suffix}", n * factor)
}
/// Logical cell positions follow Visio's enums, never sparse XML vector positions.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.vissectionindices
/// https://learn.microsoft.com/en-us/office/vba/api/visio.visrowindices
/// https://learn.microsoft.com/en-us/office/vba/api/visio.viscellindices
/// https://learn.microsoft.com/en-us/office/vba/api/visio.visunitcodes
fn metadata(evaluator: &Evaluator<'_>, scope: Scope, n: &NamedCell<'_>) -> Option<(u16, u16, u16, u16)> {
    let (base, index) = if let Some((base, tail)) = n.name.rsplit_once('[') {
        (base, tail.strip_suffix(']').and_then(|s| s.parse::<u16>().ok()).and_then(|n| n.checked_sub(1)))
    } else {
        (n.name.as_str(), None)
    };
    if let Some((s, r, c, u)) = Evaluator::baseline_metadata(scope, base) {
        return Some((s, index.unwrap_or(r), c, unit_code(n.cell, u)));
    }
    if n.section.is_none() {
        if let Some(index) = n
            .name
            .strip_prefix("Geometry")
            .and_then(|s| s.strip_suffix(".Path"))
            .and_then(|s| s.parse::<u16>().ok())
            .filter(|i| (1..=230).contains(i))
        {
            return Some((index + 9, 0, 4, 32));
        }
        let (row, col, u) = match n.name.as_str() {
            "PinX" => (1, 0, 73),
            "PinY" => (1, 1, 73),
            "Width" => (1, 2, 73),
            "Height" => (1, 3, 73),
            "BeginX" => (4, 0, 73),
            "BeginY" => (4, 1, 73),
            "EndX" => (4, 2, 73),
            "EndY" => (4, 3, 73),
            "LayerMember" => (6, 0, 231),
            "TxtPinX" => (12, 0, 73),
            "TxtPinY" => (12, 1, 73),
            "TxtWidth" => (12, 2, 73),
            "TxtHeight" => (12, 3, 73),
            "TxtLocPinX" => (12, 4, 73),
            "TxtLocPinY" => (12, 5, 73),
            "TxtAngle" => (12, 6, 81),
            "TxtFlipX" => (12, 7, 32),
            "TxtFlipY" => (12, 8, 32),
            _ => return None,
        };
        return Some((1, row, col, unit_code(n.cell, u)));
    }
    let section = n.section?;
    let s = match section.name.as_str() {
        "Character" => 3,
        "Paragraph" => 4,
        "Tabs" => 5,
        "Scratch" => 6,
        "Connection" => 7,
        "Field" => 8,
        "Control" | "Controls" => 9,
        "Geometry" => u16::try_from(section.index.unwrap_or(0)).ok()?.checked_add(10).filter(|n| *n < 240)?,
        "Action" | "Actions" => 240,
        "Layer" => 241,
        "User" => 242,
        "Property" => 243,
        "Hyperlink" => 244,
        "Reviewer" => 245,
        "Annotation" => 246,
        "SmartTag" => 247,
        "LineGradient" => 248,
        "FillGradient" => 249,
        _ => return None,
    };
    let r = match n.row {
        Some(row) => u16::try_from(
            evaluator
                .effective_row_index(scope, section, row)
                .or(row.index)
                .unwrap_or_else(|| section.rows.iter().position(|r| std::ptr::eq(r, row)).and_then(|n| u32::try_from(n).ok()).unwrap_or(0)),
        )
        .ok()?,
        None => 0,
    };
    let c = column(s, &n.cell.name, n.row.is_none())?;
    let default = match s {
        7 => match c {
            0..=3 => 73,
            6 => 231,
            _ => 32,
        },
        240 => {
            if matches!(c, 0..=2 | 14..=16) {
                231
            } else {
                32
            }
        }
        6 => {
            if c < 2 {
                73
            } else {
                32
            }
        }
        8 => 32,
        9 => {
            if c < 4 {
                73
            } else if c == 8 {
                231
            } else {
                32
            }
        }
        10..=239 => {
            if n.row.is_none() {
                32
            } else {
                73
            }
        }
        241 => match c {
            0 | 1 | 10 => 231,
            2 => 251,
            11 => 33,
            _ => 32,
        },
        242 => {
            if c == 1 {
                231
            } else {
                32
            }
        }
        243 => {
            if matches!(c, 1..=4) {
                231
            } else {
                32
            }
        }
        244 => 231,
        _ => 32,
    };
    Some((s, r, c, unit_code(n.cell, default)))
}
fn column(section: u16, name: &str, section_cell: bool) -> Option<u16> {
    let names: &[&str] = match section {
        6 => &["X", "Y", "A", "B", "C", "D"],
        7 => &["X", "Y", "DirX", "DirY", "Type", "AutoGen", "Prompt", "F", "G", "H", "I", "J", "K", "L"],
        8 => &["Value", "_Unused1", "Format", "Type", "UICat", "UICod", "UIFmt", "Calendar", "G", "H", "ObjectKind"],
        9 => &["X", "Y", "XDyn", "YDyn", "XCon", "YCon", "CanGlue", "Type", "Prompt"],
        10..=239 => {
            if section_cell {
                &["NoFill", "NoLine", "NoShow", "NoSnap", "Path", "NoQuickDrag"]
            } else {
                &["X", "Y", "A", "B", "C", "D", "E"]
            }
        }
        240 => &[
            "Menu",
            "_Unused1",
            "_Unused2",
            "Action",
            "Checked",
            "Disabled",
            "ReadOnly",
            "Invisible",
            "BeginGroup",
            "FlyoutChild",
            "_Unused10",
            "_Unused11",
            "_Unused12",
            "_Unused13",
            "TagName",
            "ButtonFace",
            "SortKey",
        ],
        241 => &["Name", "Password", "Color", "Status", "Visible", "Print", "Active", "Lock", "Snap", "Glue", "NameUniv", "ColorTrans"],
        242 => &["Value", "Prompt"],
        243 => &[
            "Value",
            "Prompt",
            "Label",
            "Format",
            "SortKey",
            "Type",
            "Invisible",
            "Ask",
            "DataLinked",
            "_Unused9",
            "_Unused10",
            "_Unused11",
            "_Unused12",
            "_Unused13",
            "LangID",
            "Calendar",
        ],
        244 => &["Description", "Address", "SubAddress", "ExtraInfo", "Frame", "NewWindow", "_Unused6", "Default", "Invisible"],
        245 => &["Name", "Initials", "Color", "ReviewerID", "CurrentIndex"],
        246 => &["X", "Y", "ReviewerID", "MarkerIndex", "Date", "Comment", "LangID"],
        247 => &["X", "Y", "TagName", "XJustify", "YJustify", "DisplayMode", "ButtonFace", "Disabled"],
        248 | 249 => &["GradientStopColor", "GradientStopColorTrans", "GradientStopPosition"],
        _ => return None,
    };
    names.iter().position(|n| n.eq_ignore_ascii_case(name)).and_then(|n| u16::try_from(n).ok())
}
fn unit_code(cell: &Cell, fallback: u16) -> u16 {
    match cell.units.as_deref() {
        Some("STR") => 231,
        Some("BOOL" | "NUM") => 32,
        Some("PCT") => 33,
        Some("PT") => 50,
        Some("P") => 51,
        Some("CM") => 69,
        Some("MM") => 70,
        Some("M") => 71,
        Some("FT") => 66,
        Some("IN") => 65,
        Some("DL") => 73,
        Some("DEG") => 81,
        Some("RAD") => 83,
        Some("DATE") => 40,
        Some("GUID") => 95,
        Some("NURBS") => 138,
        Some("POLYLINE") => 139,
        _ => fallback,
    }
}
fn supplement(slots: &mut BTreeMap<(u16, u16, u16), Slot<'_>>) -> Result<(), String> {
    let existing: Vec<_> = slots.values().cloned().collect();
    for item in existing {
        let (s, r, _) = item.src;
        if (10..240).contains(&s) {
            for (column, field) in ["NoFill", "NoLine", "NoShow", "NoSnap", "Path", "NoQuickDrag"].iter().enumerate() {
                let c = u16::try_from(column).map_err(|_| "geometry column limit")?;
                slots.entry((s, 0, c)).or_insert_with(|| Slot {
                    name: format!("Geometry{}.{}", s - 9, field),
                    cell: None,
                    src: (s, 0, c),
                    units: 32,
                    synthetic: Some(Value::Number(0.0)),
                });
            }
        }
        if s == 1 && r == 12 {
            for (c, name) in [(7, "TxtFlipX"), (8, "TxtFlipY"), (9, "Sect1.H13")] {
                slots.entry((s, r, c)).or_insert_with(|| Slot {
                    name: name.into(),
                    cell: None,
                    src: (s, r, c),
                    units: 32,
                    synthetic: Some(Value::Number(0.0)),
                });
            }
        }
        if s == 7 {
            // Visio exposes all fourteen columns even where XML omits empty cells.
            for (c, field) in ["X", "Y", "DirX", "DirY", "Type", "AutoGen", "Prompt", "F", "G", "H", "I", "J", "K", "L"].iter().enumerate() {
                let c = u16::try_from(c).map_err(|_| "connection column limit")?;
                let name = crate::evaluate::connection_name(u32::from(r), field).map_err(|e| e.to_string())?;
                slots.entry((s, r, c)).or_insert_with(|| Slot {
                    name,
                    cell: None,
                    src: (s, r, c),
                    units: if c == 6 { 231 } else { 32 },
                    synthetic: Some(Value::Number(0.0)),
                });
            }
        }
        if s == 9 {
            let base = item
                .name
                .strip_suffix(".XDyn")
                .or_else(|| item.name.strip_suffix(".YDyn"))
                .or_else(|| item.name.strip_suffix(".XCon"))
                .or_else(|| item.name.strip_suffix(".YCon"))
                .or_else(|| item.name.strip_suffix(".CanGlue"))
                .or_else(|| item.name.strip_suffix(".Prompt"))
                .or_else(|| item.name.strip_suffix(".Type"))
                .or_else(|| item.name.strip_suffix(".Y"))
                .unwrap_or(&item.name);
            slots.entry((s, r, 7)).or_insert_with(|| Slot {
                name: format!("{base}.Type"),
                cell: None,
                src: (s, r, 7),
                units: 32,
                synthetic: Some(Value::Number(0.0)),
            });
        }
        if s == 240 {
            let base = item.name.rsplit_once('.').map(|(base, _)| base).unwrap_or(&item.name);
            let base = if item.src.2 == 0 { item.name.as_str() } else { base };
            for (c, name, units) in [
                (1, format!("{base}._Unused1"), 231),
                (2, format!("{base}._Unused2"), 231),
                (10, format!("{base}I2{}", u32::from(r) + 1), 32),
                (11, format!("{base}J2{}", u32::from(r) + 1), 32),
                (12, format!("{base}K2{}", u32::from(r) + 1), 32),
                (13, format!("{base}L2{}", u32::from(r) + 1), 32),
            ] {
                slots.entry((s, r, c)).or_insert_with(|| Slot { name, cell: None, src: (s, r, c), units, synthetic: Some(Value::Number(0.0)) });
            }
        }
        if s == 241 {
            let suffix = if r == 0 { String::new() } else { format!("[{}]", u32::from(r) + 1) };
            slots.entry((s, r, 1)).or_insert_with(|| Slot {
                name: format!("Layers.Password{suffix}"),
                cell: None,
                src: (s, r, 1),
                units: 231,
                synthetic: Some(Value::Text(String::new())),
            });
        }
        if s == 242 {
            let base = item.name.strip_suffix(".Prompt").unwrap_or(&item.name);
            slots.entry((s, r, 1)).or_insert_with(|| Slot {
                name: format!("{base}.Prompt"),
                cell: None,
                src: (s, r, 1),
                units: 231,
                synthetic: Some(Value::Text(String::new())),
            });
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn hostile_group_name_fanout_is_bounded_before_dump_output() {
        let mut doc = diagramcraft_model::skeleton::document(false);
        let group = diagramcraft_model::Shape {
            name_u: "x".repeat(64000),
            shapes: (1..90001).map(|id| diagramcraft_model::Shape { id, ..Default::default() }).collect(),
            ..Default::default()
        };
        doc.pages[0].shapes.push(group);
        assert!(super::selected(&doc, super::Selection::All).is_err());
        assert!(super::selected(&doc, super::Selection::Shape(89999)).is_err());
    }
    use super::*;
    use diagramcraft_model::{Page, Sheet};
    #[test]
    fn basic_rectangle_corpus_has_every_com_identity_and_unit() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated/master");
        if !std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus").exists() {
            eprintln!("local corpus absent: basic rectangle COM dump test skipped");
            return;
        }
        let document = crate::import(&std::fs::read(root.join("basic-rect.vsdx")).unwrap()).unwrap();
        let mut out = Vec::new();
        dump(&document, Selection::Page(1), &mut out).unwrap();
        let actual = String::from_utf8(out).unwrap();
        let rows: BTreeMap<_, _> = actual
            .lines()
            .skip(1)
            .map(|line| {
                let fields: Vec<_> = line.split('\t').collect();
                (fields[..6].join("\t"), fields[8])
            })
            .collect();
        let expected = std::fs::read_to_string(root.join("basic-rect.sheet.tsv")).unwrap();
        for line in expected.lines().skip(1) {
            let fields: Vec<_> = line.split('\t').collect();
            let key = fields[..6].join("\t");
            assert_eq!(rows.get(&key).copied(), Some(fields[8]), "{key}");
        }
    }
    #[test]
    fn explicit_inches_and_connection_columns() {
        assert_eq!(unit_code(&Cell { units: Some("IN".into()), ..Default::default() }, 73), 65);
        assert_eq!(unit_code(&Cell::default(), 73), 73);
        assert_eq!(unit_code(&Cell { units: Some("DL".into()), ..Default::default() }, 32), 73);
        assert_eq!(column(7, "Prompt", false), Some(6));
        assert_eq!(crate::evaluate::connection_name(1, "DirX").unwrap(), "Connections.DirX[2]");
        assert_eq!(crate::evaluate::connection_name(0, "H").unwrap(), "Connections.H1");
        assert!(crate::evaluate::connection_name(u32::MAX, "X").is_err());
    }
    #[test]
    fn invalid_selection_and_writer_errors_are_returned() {
        let doc = Document::default();
        let mut out = Vec::new();
        assert!(dump(&doc, Selection::Page(0), &mut out).is_err());
        assert!(out.is_empty());
        assert!(dump(&doc, Selection::Master(u32::MAX), &mut out).is_err());
        struct Fail;
        impl Write for Fail {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("closed writer"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        assert!(dump(&doc, Selection::Document, &mut Fail).unwrap_err().contains("closed writer"));
    }
    #[test]
    fn page_ordinal_preserves_xml_identity_and_escaping() {
        let doc = Document {
            pages: vec![Page {
                id: 0,
                name_u: "a\t\\\nb".into(),
                shapes: vec![Shape {
                    id: 2,
                    sheet: Sheet { cells: vec![Cell { name: "Width".into(), value: Some("2".into()), ..Default::default() }], ..Default::default() },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut out = Vec::new();
        dump(&doc, Selection::Page(1), &mut out).unwrap();
        let out = String::from_utf8(out).unwrap();
        assert!(out.contains("document/TheDoc"));
        assert!(out.contains("pagesheet:0:a\\t\\\\\\nb/ThePage"));
        assert!(out.contains("2\tpage:0:a\\t\\\\\\nb/Sheet.2\t1\t1\t2\tWidth\t2 in\t2\t73"));
        assert!(out.lines().all(|line| line.split('\t').count() == 9));
    }
    #[test]
    fn style_com_surface_omits_gradient_and_layer_membership_only() {
        use diagramcraft_model::{Row, Section, Style};
        let sheet = Sheet {
            cells: vec![Cell { name: "LayerMember".into(), value: Some("0".into()), ..Default::default() }],
            sections: vec![Section {
                name: "FillGradient".into(),
                rows: vec![Row {
                    index: Some(0),
                    cells: vec![Cell { name: "GradientStopColor".into(), value: Some("1".into()), ..Default::default() }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        };
        let doc = Document {
            styles: vec![Style { id: 0, name_u: "No Style".into(), sheet: sheet.clone(), ..Default::default() }],
            pages: vec![Page { id: 0, name_u: "Page-1".into(), shapes: vec![Shape { id: 1, sheet, ..Default::default() }], ..Default::default() }],
            ..Default::default()
        };
        let mut out = Vec::new();
        dump(&doc, Selection::All, &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(!text.lines().any(|line| line.contains("style/No Style") && (line.contains("GradientStop") || line.contains("LayerMember"))));
        assert!(text.lines().any(|line| line.contains("page:0:Page-1/Sheet.1") && line.contains("GradientStopColor")));
        assert!(text.lines().any(|line| line.contains("page:0:Page-1/Sheet.1") && line.contains("LayerMember")));
    }
    #[test]
    fn sparse_named_row_overrides_keep_master_slots_and_deleted_rows_compact() {
        use diagramcraft_model::{Master, Row, Section};
        let row = |name: &str, value: &str| Row {
            name: Some(name.into()),
            cells: vec![Cell { name: "Value".into(), value: Some(value.into()), ..Default::default() }],
            ..Default::default()
        };
        let section = |rows| Section { name: "User".into(), rows, ..Default::default() };
        let mut explicit = row("D", "5");
        explicit.index = Some(7);
        let doc = Document {
            masters: vec![Master {
                id: 2,
                shapes: vec![Shape {
                    id: 5,
                    sheet: Sheet { sections: vec![section(vec![row("A", "1"), row("B", "2")])], ..Default::default() },
                    ..Default::default()
                }],
                ..Default::default()
            }],
            pages: vec![Page {
                id: 0,
                name_u: "Page-1".into(),
                shapes: vec![
                    Shape {
                        id: 20,
                        master: Some(2),
                        sheet: Sheet { sections: vec![section(vec![row("B", "3"), row("C", "4")])], ..Default::default() },
                        ..Default::default()
                    },
                    Shape {
                        id: 21,
                        master: Some(2),
                        sheet: Sheet {
                            sections: vec![section(vec![
                                Row { name: Some("A".into()), deleted: true, ..Default::default() },
                                row("B", "3"),
                                row("C", "4"),
                                explicit,
                            ])],
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut out = Vec::new();
        dump(&doc, Selection::Page(1), &mut out).unwrap();
        let out = String::from_utf8(out).unwrap();
        let mut rows = BTreeMap::new();
        for line in out.lines().skip(1) {
            let fields: Vec<_> = line.split('\t').collect();
            if fields.get(2) == Some(&"242") && fields.get(4) == Some(&"0") {
                rows.insert((fields[0], fields[5]), (fields[3], fields[7]));
            }
        }
        for (id, name, row, value) in [
            ("20", "User.A", "0", "1"),
            ("20", "User.B", "1", "3"),
            ("20", "User.C", "2", "4"),
            ("21", "User.B", "0", "3"),
            ("21", "User.C", "1", "4"),
            ("21", "User.D", "7", "5"),
        ] {
            assert_eq!(rows.get(&(id, name)), Some(&(row, value)));
        }
        assert!(!rows.contains_key(&("21", "User.A")));
    }
    #[test]
    fn enum_geometry_units_and_unicode_strings() {
        assert_eq!(column(10, "D", false), Some(5));
        assert_eq!(column(240, "Action", false), Some(3));
        assert_eq!(constant_formula(&Cell { value: Some("#cc0000".into()), ..Default::default() }, 251), "RGB(204,0,0)");
        assert_eq!(escape("é\t\n\\"), "é\\t\\n\\\\");
    }
}
