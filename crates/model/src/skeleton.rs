//! Authored document construction using observed ShapeSheet serialization defaults.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
use crate::*;
#[path = "skeleton_defaults.rs"]
mod defaults;
#[path = "skeleton_properties.rs"]
mod properties;
pub use properties::update_summary;
/// Format authored numeric caches with the observed 16 significant digits.
/// Read caches bypass this formatter. Witness: scratch metric PageHeight 297 mm.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
pub fn number_text(value: f64) -> Option<String> {
    if !value.is_finite() {
        return None;
    }
    if value == 0. {
        return Some("0".into());
    }
    // Visio removes arithmetic noise around short decimal coordinates when saving
    // authored routes. Preserve longer constants (for example 10/72 font sizes).
    // Witness: p4b__san-topology/saved.vsdx, Width 0.8500000000000001 -> 0.85.
    for digits in 0..=12 {
        let short = format!("{value:.digits$}").parse::<f64>().ok()?;
        if (short - value).abs() <= value.abs() * f64::EPSILON * 8. {
            return Some(short.to_string());
        }
    }
    format!("{value:.15e}").parse::<f64>().ok().map(|n| n.to_string())
}

/// Complete layer cells observed in generated/page/layers.vsdx; values follow
/// scratch__metric-background-layers/saved.vsdx, including its No Formula defaults.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/layer-section
pub fn layer(index: u32, name: &str, visible: bool) -> Result<Row, String> {
    if index >= 16_384 || name.len() > 65_536 {
        return Err("layer index or name limit".into());
    }
    Ok(row(
        &[("IX", &index.to_string())],
        [
            ("Name", name),
            ("Color", "0"),
            ("Status", "0"),
            ("Visible", if visible { "1" } else { "0" }),
            ("Print", "1"),
            ("Active", "0"),
            ("Lock", "0"),
            ("Snap", "0"),
            ("Glue", "0"),
            ("NameUniv", ""),
            ("ColorTrans", "0"),
        ]
        .into_iter()
        .map(|(n, v)| cell(n, Some(v), matches!(n, "Color" | "Status" | "Snap" | "Glue" | "NameUniv" | "ColorTrans").then_some("No Formula"), None))
        .collect(),
    ))
}

pub const VISIO_NS: &str = "http://schemas.microsoft.com/office/visio/2012/main";
const VIS_REL: &str = "http://schemas.microsoft.com/visio/2010/relationships/";
const OFFICE_REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/";
fn attributes(values: &[(&str, &str)]) -> BTreeMap<String, String> {
    values.iter().map(|(k, v)| ((*k).into(), (*v).into())).collect()
}
fn element_ns(name: &str, ns: &str, attrs: &[(&str, &str)], children: Vec<XmlNode>) -> XmlElement {
    XmlElement { name: name.into(), namespace: ns.into(), attributes: attributes(attrs), children, ..Default::default() }
}
fn element(name: &str, attrs: &[(&str, &str)], children: Vec<XmlNode>) -> XmlElement {
    element_ns(name, VISIO_NS, attrs, children)
}
fn text_element(name: &str, value: &str) -> XmlNode {
    XmlNode::Element(element(name, &[], vec![XmlNode::Text(value.into())]))
}
fn cell(name: &str, value: Option<&str>, formula: Option<&str>, units: Option<&str>) -> Cell {
    Cell {
        name: name.into(),
        value: value.map(|v| {
            if units == Some("STR")
                || matches!(name, "Name" | "NameUniv" | "Prompt" | "Label" | "Comment" | "ShapeKeywords" | "HelpTopic" | "Copyright")
            {
                v.to_owned()
            } else {
                v.parse::<f64>().ok().and_then(number_text).unwrap_or_else(|| v.to_owned())
            }
        }),
        formula: formula.map(str::to_owned),
        units: units.map(str::to_owned),
        ..Default::default()
    }
}
fn row(attrs: &[(&str, &str)], cells: Vec<Cell>) -> Row {
    let a = attributes(attrs);
    Row {
        name: a.get("N").cloned(),
        index: a.get("IX").and_then(|s| s.parse().ok()),
        row_type: a.get("T").cloned(),
        deleted: a.get("Del").is_some_and(|v| v == "1" || v == "true"),
        attributes: a,
        cells,
    }
}
fn section(attrs: &[(&str, &str)], cells: Vec<Cell>, rows: Vec<Row>) -> Section {
    let a = attributes(attrs);
    Section {
        name: a.get("N").cloned().unwrap_or_default(),
        index: a.get("IX").and_then(|s| s.parse().ok()),
        attributes: a,
        cells,
        rows,
        ..Default::default()
    }
}
fn relation(id: usize, kind: &str, target: &str) -> Relationship {
    Relationship { id: format!("rId{id}"), type_uri: kind.into(), target: target.into(), external: false }
}
fn vis_relation(id: usize, kind: &str, target: &str) -> Relationship {
    relation(id, &format!("{VIS_REL}{kind}"), target)
}
fn part(root: XmlElement, content_type: &str) -> Part {
    Part { xml: Some(root), content_type: Some(content_type.into()), ..Default::default() }
}
/// Create a blank US or metric drawing, with six observed blank styles and connector style 7.
/// Style 5 is absent in both witnesses. No vendor theme, thumbnail or icon is embedded.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/facenames-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/facename-element-facenames_type-complextypevisio-xml
pub fn document(metric: bool) -> Document {
    let main = "visio/document.xml";
    let mut doc = Document {
        main_part: main.into(),
        document_sheet: defaults::document_sheet(),
        styles: defaults::styles(),
        colors: BTreeMap::from([(24, "#7F7F7F".into()), (25, "#000000".into()), (26, "#FFFFFF".into()), (27, "#0000FF".into())]),
        face_names: BTreeMap::from([(0, "Calibri".into())]),
        pages: vec![page(0, "Page-1", metric)],
        masters: vec![dynamic_connector()],
        ..Default::default()
    };
    doc.document_sheet.attributes = attributes(&[
        ("NameU", "TheDoc"),
        ("Name", "TheDoc"),
        ("IsCustomNameU", "1"),
        ("IsCustomName", "1"),
        ("LineStyle", "0"),
        ("FillStyle", "0"),
        ("TextStyle", "0"),
    ]);
    for style in &mut doc.styles {
        if style.id == 0 {
            for name in ["Copyright", "HelpTopic"] {
                style.sheet.cells.push(cell(name, Some(""), None, None));
            }
        }
        style.sheet.attributes = attributes(&[("IsCustomNameU", "1"), ("IsCustomName", "1")]);
    }
    doc.document_settings = Some(settings());
    for (name, root, kind) in [
        (main, "VisioDocument", "application/vnd.ms-visio.drawing.main+xml"),
        ("visio/pages/pages.xml", "Pages", "application/vnd.ms-visio.pages+xml"),
        ("visio/pages/page1.xml", "PageContents", "application/vnd.ms-visio.page+xml"),
        ("visio/masters/masters.xml", "Masters", "application/vnd.ms-visio.masters+xml"),
        ("visio/masters/master1.xml", "MasterContents", "application/vnd.ms-visio.master+xml"),
    ] {
        doc.parts.insert(name.into(), part(element(root, &[("xmlns", VISIO_NS), ("xml:space", "preserve")], vec![]), kind));
    }
    if let Some(p) = doc.parts.get_mut(main) {
        p.relationships = vec![
            vis_relation(2, "pages", "visio/pages/pages.xml"),
            vis_relation(1, "masters", "visio/masters/masters.xml"),
            vis_relation(3, "windows", "visio/windows.xml"),
        ];
        if let Some(root) = &mut p.xml {
            root.children.push(XmlNode::Element(element(
                "FaceNames",
                &[],
                vec![XmlNode::Element(element(
                    "FaceName",
                    &[
                        ("NameU", "Calibri"),
                        ("UnicodeRanges", "-469750017 -1040178053 9 0"),
                        ("CharSets", "536871423 0"),
                        ("Panose", "2 15 5 2 2 2 4 3 2 4"),
                        ("Flags", "357"),
                    ],
                    vec![],
                ))],
            )));
        }
    }
    if let Some(p) = doc.parts.get_mut("visio/pages/pages.xml") {
        p.relationships.push(vis_relation(1, "page", "visio/pages/page1.xml"));
    }
    if let Some(p) = doc.parts.get_mut("visio/masters/masters.xml") {
        p.relationships.push(vis_relation(1, "master", "visio/masters/master1.xml"));
    }
    doc.root_relationships = vec![
        relation(3, "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties", "docProps/core.xml"),
        vis_relation(1, "document", main),
        relation(5, &format!("{OFFICE_REL}custom-properties"), "docProps/custom.xml"),
        relation(4, &format!("{OFFICE_REL}extended-properties"), "docProps/app.xml"),
    ];
    doc.parts.insert("visio/windows.xml".into(), part(windows(), "application/vnd.ms-visio.windows+xml"));
    properties::metadata(&mut doc, metric);
    doc
}
/// Default settings observed in the blank document, with no UI customization payload.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/documentsettings-element-visiodocument_type-complextypevisio-xml
fn settings() -> XmlElement {
    element(
        "DocumentSettings",
        &[("TopPage", "0"), ("DefaultTextStyle", "3"), ("DefaultLineStyle", "3"), ("DefaultFillStyle", "3"), ("DefaultGuideStyle", "4")],
        vec![
            text_element("GlueSettings", "9"),
            text_element("SnapSettings", "65847"),
            text_element("SnapExtensions", "34"),
            text_element("SnapAngles", ""),
            text_element("DynamicGridEnabled", "1"),
            text_element("ProtectStyles", "0"),
            text_element("ProtectShapes", "0"),
            text_element("ProtectMasters", "0"),
            text_element("ProtectBkgnds", "0"),
        ],
    )
}
/// Page defaults come from the blank witness; metric pages use A4 physical dimensions.
/// Page view defaults are centred with ViewScale=-1, observed in corpus/blank/blank.vsdx.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagesheet-element-page_type-complextypevisio-xml
pub fn page(id: u32, name: &str, metric: bool) -> Page {
    let mut sheet = defaults::page_sheet();
    sheet.attributes = attributes(&[("LineStyle", "0"), ("FillStyle", "0"), ("TextStyle", "0")]);
    if metric {
        for c in &mut sheet.cells {
            match c.name.as_str() {
                "PageWidth" => c.value = number_text(210. / 25.4),
                "PageHeight" => c.value = number_text(297. / 25.4),
                "PageScale" | "DrawingScale" => {
                    c.value = number_text(1. / 25.4);
                    c.units = Some("MM".into());
                }
                _ => {}
            }
        }
    }
    Page {
        id,
        name: name.into(),
        name_u: name.into(),
        sheet,
        attributes: attributes(&[
            ("ViewScale", "-1"),
            ("ViewCenterX", &number_text(if metric { 105. / 25.4 } else { 4.25 }).unwrap_or_default()),
            ("ViewCenterY", &number_text(if metric { 148.5 / 25.4 } else { 5.5 }).unwrap_or_default()),
        ]),
        part: format!("visio/pages/page{}.xml", u64::from(id) + 1),
        ..Default::default()
    }
}
/// Synthetic master identities retain Visio's observed suffix on SaveAs.
/// Random first three groups remain contributor-authored; no vendor identity is copied.
/// Observed: corpus/_vm/out/accept/scratch__rectangle-text/saved.vsdx.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
fn master_unique_id() -> String {
    let random = uuid::Uuid::new_v4();
    let (a, b, c, _) = random.as_fields();
    let id = uuid::Uuid::from_fields(a, b, c, &[0x8e, 0x40, 0x00, 0x60, 0x8c, 0xf3, 0x05, 0xb2]);
    format!("{{{}}}", id.hyphenated()).to_uppercase()
}
/// Independently authored master using the probe's functional cell names and formulas.
/// Fresh identities replace both vendor GUIDs; no icon, help reference or copyright cell.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
pub fn dynamic_connector() -> Master {
    let guid = || format!("{{{}}}", uuid::Uuid::new_v4().hyphenated()).to_uppercase();
    Master {
        id: 2,
        name: "Dynamic connector".into(),
        name_u: "Dynamic connector".into(),
        unique_id: Some(master_unique_id()),
        base_id: Some(guid()),
        part: "visio/masters/master1.xml".into(),
        sheet: defaults::connector_page_sheet(),
        shapes: vec![Shape {
            id: 5,
            shape_type: Some("Shape".into()),
            line_style: Some(7),
            fill_style: Some(7),
            text_style: Some(7),
            sheet: {
                let mut sheet = defaults::connector_sheet();
                for name in ["Comment", "ShapeKeywords"] {
                    sheet.cells.retain(|c| c.name != name);
                }
                sheet
            },
            ..Default::default()
        }],
        attributes: attributes(&[
            ("IsCustomNameU", "1"),
            ("IsCustomName", "1"),
            ("Prompt", "Connect two shapes"),
            ("IconSize", "1"),
            ("AlignName", "2"),
            ("MatchByName", "1"),
            ("IconUpdate", "1"),
            ("PatternFlags", "0"),
            ("Hidden", "0"),
            ("MasterType", "541"),
        ]),
        ..Default::default()
    }
}
/// Authored rectangle with the same five-row geometry contract as the blank rectangle.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
pub fn rectangle(id: u32, x: f64, y: f64, width: f64, height: f64, text: &str) -> Result<Shape, String> {
    if ![x, y, width, height].iter().all(|v| v.is_finite() && v.abs() <= 1e6) || width <= 0. || height <= 0. || text.len() > 1024 * 1024 {
        return Err("rectangle bounds or text limit".into());
    }
    let mut cells: Vec<_> = [
        ("PinX", x + width / 2.),
        ("PinY", y + height / 2.),
        ("Width", width),
        ("Height", height),
        ("LocPinX", width / 2.),
        ("LocPinY", height / 2.),
        ("Angle", 0.),
        ("FlipX", 0.),
        ("FlipY", 0.),
        ("ResizeMode", 0.),
    ]
    .into_iter()
    .map(|(n, v)| cell(n, Some(&v.to_string()), None, None))
    .collect();
    if let Some(c) = cells.iter_mut().find(|c| c.name == "LocPinX") {
        c.formula = Some("Width*0.5".into());
    }
    if let Some(c) = cells.iter_mut().find(|c| c.name == "LocPinY") {
        c.formula = Some("Height*0.5".into());
    }
    let rows = [(0., 0.), (1., 0.), (1., 1.), (0., 1.), (0., 0.)]
        .into_iter()
        .enumerate()
        .map(|(i, (fx, fy))| {
            row(
                &[("T", if i == 0 { "MoveTo" } else { "LineTo" }), ("IX", &(i + 1).to_string())],
                vec![
                    cell("X", Some(&(width * fx).to_string()), Some(&if i == 4 { "Geometry1.X1".into() } else { format!("Width*{fx}") }), None),
                    cell("Y", Some(&(height * fy).to_string()), Some(&if i == 4 { "Geometry1.Y1".into() } else { format!("Height*{fy}") }), None),
                ],
            )
        })
        .collect();
    let geometry = section(
        &[("N", "Geometry"), ("IX", "0")],
        ["NoFill", "NoLine", "NoShow", "NoSnap", "NoQuickDrag"].into_iter().map(|n| cell(n, Some("0"), None, None)).collect(),
        rows,
    );
    Ok(Shape {
        id,
        shape_type: Some("Shape".into()),
        line_style: Some(3),
        fill_style: Some(3),
        text_style: Some(3),
        sheet: Sheet { cells, sections: vec![geometry], ..Default::default() },
        text: (!text.is_empty()).then(|| element("Text", &[], vec![XmlNode::Text(format!("{text}\n"))])),
        ..Default::default()
    })
}
/// Full window settings observed in corpus/blank/blank.vsdx.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/window-element-windows_type-complextypevisio-xml
fn windows() -> XmlElement {
    element(
        "Windows",
        &[("xmlns", VISIO_NS), ("xml:space", "preserve"), ("ClientWidth", "3840"), ("ClientHeight", "1614")],
        vec![XmlNode::Element(element(
            "Window",
            &[
                ("ID", "0"),
                ("WindowType", "Drawing"),
                ("WindowState", "1073741824"),
                ("WindowLeft", "-13"),
                ("WindowTop", "-58"),
                ("WindowWidth", "3866"),
                ("WindowHeight", "1685"),
                ("ContainerType", "Page"),
                ("Page", "0"),
                ("ViewScale", "-1"),
                ("ViewCenterX", "4.2538462786703"),
                ("ViewCenterY", "5.5076924693091"),
            ],
            [
                ("ShowRulers", "1"),
                ("ShowGrid", "0"),
                ("ShowPageBreaks", "1"),
                ("ShowGuides", "1"),
                ("ShowConnectionPoints", "1"),
                ("GlueSettings", "9"),
                ("SnapSettings", "65847"),
                ("SnapExtensions", "34"),
                ("SnapAngles", ""),
                ("DynamicGridEnabled", "1"),
                ("TabSplitterPos", "0.5"),
            ]
            .into_iter()
            .map(|(n, v)| text_element(n, v))
            .collect(),
        ))],
    )
}

/// Add dynamic glue between two top-level shapes using page-space cached endpoints.
/// The caller supplies endpoints from its geometry; the instance carries a straight route.
/// Observed formulas and ObjType: corpus/generated/connector/dynamic-glue.vsdx.
pub fn glue_dynamic(page: &mut Page, id: u32, master: u32, targets: [u32; 2], endpoints: [[f64; 2]; 2]) -> Result<(), String> {
    if page.shapes.len() >= 100_000
        || page.connects.len() > 99_998
        || endpoints.iter().flatten().any(|v| !v.is_finite() || v.abs() > 1e6)
        || page.shapes.iter().any(|s| s.id == id)
        || targets.iter().any(|target| {
            page.shapes.iter().filter(|s| s.id == *target && !s.sheet.attributes.get("Del").is_some_and(|d| d == "1" || d == "true")).count() != 1
        })
        || page.shapes.iter().any(|s| targets.contains(&s.id) && s.sheet.cells.len() >= 20_000)
    {
        return Err("dynamic glue target, coordinate or collection limit".into());
    }
    let mut stack: Vec<_> = page.shapes.iter().map(|s| (s, 0usize)).collect();
    let mut ids = std::collections::BTreeSet::new();
    let mut count = 0usize;
    while let Some((shape, depth)) = stack.pop() {
        count += 1;
        let tombstone =
            shape.id == u32::MAX && shape.master_shape.is_some() && shape.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true");
        if shape.id == id
            || depth > 128
            || count >= 100_000
            || (!tombstone && !ids.insert(shape.id))
            || count.saturating_add(stack.len()).saturating_add(shape.shapes.len()) >= 100_000
        {
            return Err("dynamic glue shape IDs, nesting or count limit".into());
        }
        stack.extend(shape.shapes.iter().map(|s| (s, depth + 1)));
    }
    let [begin, end] = endpoints;
    let [from, to] = targets;
    let forward = "_WALKGLUE(BegTrigger,EndTrigger,WalkPreference)";
    let reverse = "_WALKGLUE(EndTrigger,BegTrigger,WalkPreference)";
    let [bx, by] = begin;
    let [ex, ey] = end;
    let width = ex - bx;
    let delta_y = ey - by;
    let height = if delta_y == 0. { 0.25 } else { delta_y };
    let height_formula = if delta_y == 0. { "GUARD(0.25DL)" } else { "GUARD(EndY-BeginY)" };
    let layer_section = page.sheet.sections.iter().position(|s| s.name == "Layer" && !s.deleted);
    let existing_layer = layer_section.and_then(|i| page.sheet.sections.get(i)).and_then(|s| {
        s.rows.iter().enumerate().find_map(|(i, r)| {
            (!r.deleted && r.cells.iter().any(|c| matches!(c.name.as_str(), "Name" | "NameUniv") && c.value.as_deref() == Some("Connector")))
                .then(|| r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)))
        })
    });
    let layer_index = existing_layer.unwrap_or_else(|| {
        layer_section.and_then(|i| page.sheet.sections.get(i)).map_or(0, |s| {
            s.rows.iter().enumerate().map(|(i, r)| r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX))).max().map_or(0, |n| n.saturating_add(1))
        })
    });
    if layer_index >= 16_384 || (existing_layer.is_none() && layer_section.is_none() && page.sheet.sections.len() >= 16_384) {
        return Err("dynamic glue layer limit".into());
    }
    if width.abs() > 1e6 || height.abs() > 1e6 {
        return Err("dynamic glue derived coordinate limit".into());
    }
    let extra = if existing_layer.is_some() { 0 } else { 11 };
    glue_sheet_capacity(&page.sheet, extra)?;
    if extra != 0
        && layer_section.and_then(|i| page.sheet.sections.get(i)).is_some_and(|section| {
            section.rows.len() >= 16_384
                || section.cells.len().saturating_add(section.rows.iter().fold(0usize, |n, r| n.saturating_add(r.cells.len()))).saturating_add(extra)
                    > 16_384
        })
    {
        return Err("dynamic glue layer section limit".into());
    }
    for shape in page.shapes.iter().filter(|s| targets.contains(&s.id)) {
        glue_sheet_capacity(&shape.sheet, usize::from(shape.sheet.cell("ObjType").is_none()))?;
    }
    let connector = Shape {
        id,
        master: Some(master),
        shape_type: Some("Shape".into()),
        sheet: Sheet {
            cells: vec![
                cell("PinX", Some(&((bx + ex) * 0.5).to_string()), Some("Inh"), None),
                cell("PinY", Some(&((by + ey) * 0.5).to_string()), Some("Inh"), None),
                cell("Width", Some(&width.to_string()), Some("GUARD(EndX-BeginX)"), None),
                cell("Height", Some(&height.to_string()), Some(height_formula), None),
                cell("LocPinX", Some(&(width * 0.5).to_string()), Some("Inh"), None),
                cell("LocPinY", Some(&(height * 0.5).to_string()), Some("Inh"), None),
                cell("TxtPinX", Some(&(width * 0.5).to_string()), Some("Inh"), None),
                cell("TxtPinY", Some(&(height * 0.5).to_string()), Some("Inh"), None),
                cell("ConFixedCode", Some("6"), None, None),
                cell("LayerMember", Some(&layer_index.to_string()), None, None),
                cell("BeginX", Some(&bx.to_string()), Some(forward), None),
                cell("BeginY", Some(&by.to_string()), Some(forward), None),
                cell("EndX", Some(&ex.to_string()), Some(reverse), None),
                cell("EndY", Some(&ey.to_string()), Some(reverse), None),
                cell("BegTrigger", Some("2"), Some(&format!("_XFTRIGGER(Sheet.{from}!EventXFMod)")), None),
                cell("EndTrigger", Some("2"), Some(&format!("_XFTRIGGER(Sheet.{to}!EventXFMod)")), None),
            ],
            sections: vec![
                section(
                    &[("N", "Control")],
                    vec![],
                    vec![row(
                        &[("N", "TextPosition")],
                        vec![
                            cell("X", Some(&(width * 0.5).to_string()), None, None),
                            cell("Y", Some(&(height * 0.5).to_string()), None, None),
                            cell("XDyn", Some(&(width * 0.5).to_string()), Some("Inh"), None),
                            cell("YDyn", Some(&(height * 0.5).to_string()), Some("Inh"), None),
                        ],
                    )],
                ),
                section(
                    &[("N", "Geometry"), ("IX", "0")],
                    vec![],
                    vec![
                        row(&[("T", "MoveTo"), ("IX", "1")], vec![cell("Y", Some(&((height - delta_y) * 0.5).to_string()), None, None)]),
                        row(
                            &[("T", "LineTo"), ("IX", "2")],
                            vec![
                                cell("X", Some(&width.to_string()), None, None),
                                cell("Y", Some(&((height + delta_y) * 0.5).to_string()), None, None),
                            ],
                        ),
                        row(&[("T", "LineTo"), ("IX", "3"), ("Del", "1")], vec![]),
                    ],
                ),
            ],
            ..Default::default()
        },
        ..Default::default()
    };
    if existing_layer.is_none() {
        let new_row = row(
            &[("IX", &layer_index.to_string())],
            [
                ("Name", "Connector"),
                ("Color", "255"),
                ("Status", "0"),
                ("Visible", "1"),
                ("Print", "1"),
                ("Active", "0"),
                ("Lock", "0"),
                ("Snap", "1"),
                ("Glue", "1"),
                ("NameUniv", "Connector"),
                ("ColorTrans", "0"),
            ]
            .into_iter()
            .map(|(n, v)| cell(n, Some(v), None, None))
            .collect(),
        );
        if let Some(section) = layer_section.and_then(|i| page.sheet.sections.get_mut(i)) {
            section.rows.push(new_row);
        } else {
            page.sheet.sections.push(section(&[("N", "Layer")], vec![], vec![new_row]));
        }
    }
    for shape in &mut page.shapes {
        if targets.contains(&shape.id) {
            shape.sheet.cells.retain(|c| !c.name.eq_ignore_ascii_case("ObjType"));
            shape.sheet.cells.push(cell("ObjType", Some("1"), None, None));
        }
    }
    page.shapes.push(connector);
    for (name, target, part) in [("BeginX", from, 9), ("EndX", to, 12)] {
        page.connects.push(Connect {
            from_sheet: id,
            from_cell: Some(name.into()),
            from_part: Some(part),
            to_sheet: target,
            to_cell: Some("PinX".into()),
            to_part: Some(3),
            ..Default::default()
        });
    }
    Ok(())
}

/// Bound cells and rows before extending a glue target or the connector layer.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
fn glue_sheet_capacity(sheet: &Sheet, extra: usize) -> Result<(), String> {
    let mut total = sheet.cells.len().saturating_add(extra);
    if sheet.sections.len() > 16_384 {
        return Err("dynamic glue section limit".into());
    }
    for section in &sheet.sections {
        if section.rows.len() > 16_384 {
            return Err("dynamic glue row limit".into());
        }
        let mut cells = section.cells.len();
        for row in &section.rows {
            cells = cells.saturating_add(row.cells.len());
        }
        if cells > 16_384 {
            return Err("dynamic glue section cell limit".into());
        }
        total = total.saturating_add(cells);
    }
    if total > 65_536 {
        return Err("dynamic glue sheet cell limit".into());
    }
    // Reserve an upper bound for the fixed new cell/row names and literal values.
    if glue_metadata_bytes(sheet)?.saturating_add(extra.saturating_mul(128)).saturating_add(128) > 16 * 1024 * 1024 {
        return Err("dynamic glue sheet byte limit".into());
    }
    Ok(())
}

// Preflight metadata bytes before adding a small fixed set of glue cells.
fn glue_metadata_bytes(sheet: &Sheet) -> Result<usize, String> {
    fn attrs(a: &std::collections::BTreeMap<String, String>) -> usize {
        a.iter().fold(0usize, |n, (k, v)| n.saturating_add(k.len()).saturating_add(v.len()))
    }
    fn xml(e: &XmlElement, depth: usize, nodes: &mut usize) -> Result<usize, String> {
        *nodes = nodes.saturating_add(1);
        if depth > 128 || *nodes > 4_000_000 {
            return Err("dynamic glue XML limit".into());
        }
        let mut bytes =
            e.name.len().saturating_add(e.namespace.len()).saturating_add(attrs(&e.attributes)).saturating_add(attrs(&e.attribute_namespaces));
        for child in &e.children {
            bytes = bytes.saturating_add(match child {
                XmlNode::Text(s) => s.len(),
                XmlNode::Element(c) => xml(c, depth + 1, nodes)?,
            });
            if bytes > 16 * 1024 * 1024 {
                return Err("dynamic glue sheet byte limit".into());
            }
        }
        Ok(bytes)
    }
    fn cells(cells: &[Cell], nodes: &mut usize) -> Result<usize, String> {
        let mut bytes = 0usize;
        for c in cells {
            bytes = bytes.saturating_add(c.name.len()).saturating_add(attrs(&c.attributes));
            for s in [&c.value, &c.formula, &c.units, &c.error].into_iter().flatten() {
                bytes = bytes.saturating_add(s.len());
            }
            for e in &c.ref_by {
                bytes = bytes.saturating_add(xml(e, 0, nodes)?);
            }
            if bytes > 16 * 1024 * 1024 {
                return Err("dynamic glue sheet byte limit".into());
            }
        }
        Ok(bytes)
    }
    let mut nodes = 0;
    let mut bytes = attrs(&sheet.attributes).saturating_add(cells(&sheet.cells, &mut nodes)?);
    for section in &sheet.sections {
        bytes =
            bytes.saturating_add(section.name.len()).saturating_add(attrs(&section.attributes)).saturating_add(cells(&section.cells, &mut nodes)?);
        for row in &section.rows {
            bytes = bytes.saturating_add(attrs(&row.attributes)).saturating_add(cells(&row.cells, &mut nodes)?);
            for s in [&row.name, &row.row_type].into_iter().flatten() {
                bytes = bytes.saturating_add(s.len());
            }
        }
        if bytes > 16 * 1024 * 1024 {
            return Err("dynamic glue sheet byte limit".into());
        }
    }
    for e in &sheet.triggers {
        bytes = bytes.saturating_add(xml(e, 0, &mut nodes)?);
    }
    Ok(bytes)
}

#[cfg(test)]
mod glue_byte_regression {
    use super::*;
    #[test]
    fn replacing_empty_objtype_reserves_value_growth() {
        let mut page = page(0, "Page", false);
        page.shapes = vec![
            Shape {
                id: 1,
                sheet: Sheet { cells: vec![cell("ObjType", None, None, None), cell("Comment", Some(""), None, None)], ..Default::default() },
                ..Default::default()
            },
            Shape { id: 2, ..Default::default() },
        ];
        let remaining = 16 * 1024 * 1024 - glue_metadata_bytes(&page.shapes[0].sheet).unwrap();
        page.shapes[0].sheet.cells[1].value = Some("x".repeat(remaining));
        assert_eq!(glue_metadata_bytes(&page.shapes[0].sheet).unwrap(), 16 * 1024 * 1024);
        let before = page.clone();
        assert!(glue_dynamic(&mut page, 3, 2, [1, 2], [[1., 0.5], [2., 0.5]]).is_err());
        assert_eq!(page, before);
    }
}

#[cfg(test)]
mod authored_decimal_regression {
    /// Visio's SAN re-save removes arithmetic tails but retains long constants.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    #[test]
    fn short_decimal_coordinates_and_long_constants_are_distinct() {
        for (input, expected) in
            [(0.8500000000000001, "0.85"), (5.700000000000001, "5.7"), (-0.4249999999999998, "-0.425"), (10. / 72., "0.1388888888888889")]
        {
            assert_eq!(super::number_text(input).as_deref(), Some(expected));
        }
        assert!(super::number_text(f64::NAN).is_none());
        assert!(super::number_text(f64::INFINITY).is_none());
        assert_ne!(super::number_text(0.85000001).as_deref(), Some("0.85"));
    }
}
