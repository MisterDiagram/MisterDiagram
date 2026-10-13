//! Authored document construction using observed ShapeSheet serialization defaults.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
use crate::*;
#[path = "skeleton_defaults.rs"]
mod defaults;
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
        value: value.map(str::to_owned),
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
        attributes: a,
        cells,
        ..Default::default()
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
pub fn document(metric: bool) -> Document {
    let main = "visio/document.xml";
    let mut doc = Document {
        main_part: main.into(),
        document_sheet: defaults::document_sheet(),
        styles: defaults::styles(),
        colors: BTreeMap::from([(24, "#7f7f7f".into())]),
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
            vis_relation(1, "masters", "visio/masters/masters.xml"),
            vis_relation(2, "pages", "visio/pages/pages.xml"),
            vis_relation(3, "windows", "visio/windows.xml"),
        ];
    }
    if let Some(p) = doc.parts.get_mut("visio/pages/pages.xml") {
        p.relationships.push(vis_relation(1, "page", "visio/pages/page1.xml"));
    }
    if let Some(p) = doc.parts.get_mut("visio/masters/masters.xml") {
        p.relationships.push(vis_relation(1, "master", "visio/masters/master1.xml"));
    }
    doc.root_relationships = vec![
        vis_relation(1, "document", main),
        relation(2, "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties", "docProps/core.xml"),
        relation(3, &format!("{OFFICE_REL}extended-properties"), "docProps/app.xml"),
        relation(4, &format!("{OFFICE_REL}custom-properties"), "docProps/custom.xml"),
    ];
    doc.parts.insert("visio/windows.xml".into(), part(windows(), "application/vnd.ms-visio.windows+xml"));
    metadata(&mut doc, metric);
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
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagesheet-element-page_type-complextypevisio-xml
pub fn page(id: u32, name: &str, metric: bool) -> Page {
    let mut sheet = defaults::page_sheet();
    sheet.attributes = attributes(&[("LineStyle", "0"), ("FillStyle", "0"), ("TextStyle", "0")]);
    if metric {
        for c in &mut sheet.cells {
            match c.name.as_str() {
                "PageWidth" => c.value = Some((210. / 25.4f64).to_string()),
                "PageHeight" => c.value = Some((297. / 25.4f64).to_string()),
                "PageScale" | "DrawingScale" => {
                    c.value = Some((1. / 25.4f64).to_string());
                    c.units = Some("MM".into());
                }
                _ => {}
            }
        }
    }
    Page { id, name: name.into(), name_u: name.into(), sheet, part: format!("visio/pages/page{}.xml", u64::from(id) + 1), ..Default::default() }
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
        unique_id: Some(guid()),
        base_id: Some(guid()),
        part: "visio/masters/master1.xml".into(),
        sheet: defaults::connector_page_sheet(),
        shapes: vec![Shape {
            id: 5,
            shape_type: Some("Shape".into()),
            line_style: Some(7),
            fill_style: Some(7),
            text_style: Some(7),
            sheet: defaults::connector_sheet(),
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
            ("MasterType", "0"),
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
/// Window metadata is authored, with portable dimensions rather than copied desktop geometry.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/windows-elementvisio-xml
fn windows() -> XmlElement {
    element(
        "Windows",
        &[("xmlns", VISIO_NS), ("ClientWidth", "1200"), ("ClientHeight", "800")],
        vec![XmlNode::Element(element(
            "Window",
            &[
                ("ID", "0"),
                ("WindowType", "Drawing"),
                ("WindowState", "1073741824"),
                ("WindowLeft", "0"),
                ("WindowTop", "0"),
                ("WindowWidth", "1200"),
                ("WindowHeight", "800"),
                ("ContainerType", "Page"),
                ("Page", "0"),
                ("ViewScale", "1"),
            ],
            vec![],
        ))],
    )
}
/// Authored OPC properties, including the observed numeric build and metric flag fields.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
fn metadata(doc: &mut Document, metric: bool) {
    let core_ns = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
    let core = element_ns("coreProperties", core_ns, &[("xmlns", core_ns)], vec![]);
    doc.parts.insert("docProps/core.xml".into(), part(core, "application/vnd.openxmlformats-package.core-properties+xml"));
    let app_ns = "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties";
    let app = element_ns(
        "Properties",
        app_ns,
        &[("xmlns", app_ns)],
        vec![
            XmlNode::Element(element_ns("Application", app_ns, &[], vec![XmlNode::Text("Diagramcraft".into())])),
            XmlNode::Element(element_ns("AppVersion", app_ns, &[], vec![XmlNode::Text("1.0000".into())])),
        ],
    );
    doc.parts.insert("docProps/app.xml".into(), part(app, "application/vnd.openxmlformats-officedocument.extended-properties+xml"));
    let custom_ns = "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties";
    let vt = "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";
    let mut children = vec![];
    for (i, (name, kind, value)) in [
        ("_VPID_ALTERNATENAMES", "lpwstr", ""),
        ("BuildNumberCreated", "i4", "1074417614"),
        ("BuildNumberEdited", "i4", "1074417614"),
        ("IsMetric", "bool", if metric { "true" } else { "false" }),
    ]
    .into_iter()
    .enumerate()
    {
        children.push(XmlNode::Element(element_ns(
            "property",
            custom_ns,
            &[("fmtid", "{D5CDD505-2E9C-101B-9397-08002B2CF9AE}"), ("pid", &(i + 2).to_string()), ("name", name)],
            vec![XmlNode::Element(element_ns(kind, vt, &[], vec![XmlNode::Text(value.into())]))],
        )));
    }
    doc.parts.insert(
        "docProps/custom.xml".into(),
        part(
            element_ns("Properties", custom_ns, &[("xmlns", custom_ns), ("xmlns:vt", vt)], children),
            "application/vnd.openxmlformats-officedocument.custom-properties+xml",
        ),
    );
}

/// Add dynamic glue between two top-level shapes using page-space cached endpoints.
/// The caller supplies endpoints from its geometry; routing remains a host operation.
/// Observed formulas and ObjType: corpus/generated/connector/dynamic-glue.vsdx.
pub fn glue_dynamic(page: &mut Page, id: u32, master: u32, targets: [u32; 2], endpoints: [[f64; 2]; 2]) -> Result<(), String> {
    if page.shapes.len() >= 100_000
        || page.connects.len() > 99_998
        || endpoints.iter().flatten().any(|v| !v.is_finite() || v.abs() > 1e6)
        || page.shapes.iter().any(|s| s.id == id)
        || targets.iter().any(|target| page.shapes.iter().filter(|s| s.id == *target).count() != 1)
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
    let connector = Shape {
        id,
        master: Some(master),
        shape_type: Some("Shape".into()),
        sheet: Sheet {
            cells: vec![
                cell("BeginX", Some(&bx.to_string()), Some(forward), None),
                cell("BeginY", Some(&by.to_string()), Some(forward), None),
                cell("EndX", Some(&ex.to_string()), Some(reverse), None),
                cell("EndY", Some(&ey.to_string()), Some(reverse), None),
                cell("BegTrigger", Some("2"), Some(&format!("_XFTRIGGER(Sheet.{from}!EventXFMod)")), None),
                cell("EndTrigger", Some("2"), Some(&format!("_XFTRIGGER(Sheet.{to}!EventXFMod)")), None),
            ],
            ..Default::default()
        },
        ..Default::default()
    };
    for shape in &mut page.shapes {
        if targets.contains(&shape.id) {
            shape.sheet.cells.retain(|c| c.name != "ObjType");
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
