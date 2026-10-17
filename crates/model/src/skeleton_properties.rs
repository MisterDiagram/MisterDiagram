//! Authored OPC properties observed in blank.vsdx and the five saved scratch files.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
use super::*;
const APP: &str = "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties";
const VT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";
fn value(name: &str, ns: &str, text: &str) -> XmlNode {
    XmlNode::Element(element_ns(name, ns, &[], vec![XmlNode::Text(text.into())]))
}
/// Refresh authored summary vectors after page/master construction, never on a read part.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub fn update_summary(doc: &Document, app: &mut XmlElement) -> Result<(), String> {
    let count = doc.pages.len().saturating_add(doc.masters.len());
    let bytes = doc.pages.iter().map(|p| p.name.len()).chain(doc.masters.iter().map(|m| m.name.len())).fold(0usize, usize::saturating_add);
    if count > 200_000 || bytes > 16 * 1024 * 1024 {
        return Err("summary metadata limit".into());
    }
    summary(doc, app);
    Ok(())
}
fn summary(doc: &Document, app: &mut XmlElement) {
    let mut pairs = vec![];
    for (label, count) in [("Pages", doc.pages.len()), ("Masters", doc.masters.len())] {
        if label == "Masters" && count == 0 {
            continue;
        }
        for child in [value("lpstr", VT, label), value("i4", VT, &count.to_string())] {
            pairs.push(XmlNode::Element(element_ns("variant", VT, &[], vec![child])));
        }
    }
    let headings = element_ns(
        "HeadingPairs",
        APP,
        &[],
        vec![XmlNode::Element(element_ns("vector", VT, &[("size", &pairs.len().to_string()), ("baseType", "variant")], pairs))],
    );
    // The saved metric witness lists foreground names before background names.
    let mut pages: Vec<_> = doc.pages.iter().collect();
    pages.sort_by_key(|p| p.attributes.get("Background").is_some_and(|v| v == "1" || v == "true"));
    let names: Vec<_> =
        pages.into_iter().map(|p| p.name.as_str()).chain(doc.masters.iter().map(|m| m.name.as_str())).map(|s| value("lpstr", VT, s)).collect();
    let titles = element_ns(
        "TitlesOfParts",
        APP,
        &[],
        vec![XmlNode::Element(element_ns("vector", VT, &[("size", &names.len().to_string()), ("baseType", "lpstr")], names))],
    );
    for new in [headings, titles] {
        if let Some(old) = app.children.iter_mut().find(|n| matches!(n, XmlNode::Element(e) if e.name == new.name && e.namespace == APP)) {
            *old = XmlNode::Element(new);
        } else {
            app.children.push(XmlNode::Element(new));
        }
    }
}
/// AppVersion is 16.0000: Visio rewrote 1.0000 but kept our authored Application name.
/// Dates are fresh UTC values; lastModifiedBy/lastPrinted remain Visio first-save stamps.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub(super) fn metadata(doc: &mut Document, metric: bool) {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let core_ns = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
    let dc = "http://purl.org/dc/elements/1.1/";
    let dt = "http://purl.org/dc/terms/";
    let language = doc.document_sheet.cell("DocLangID").and_then(|c| c.value.as_deref()).unwrap_or("en-US");
    let core = element_ns(
        "coreProperties",
        core_ns,
        &[("xmlns:cp", core_ns), ("xmlns:dc", dc), ("xmlns:dcterms", dt), ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")],
        vec![
            XmlNode::Element(element_ns("created", dt, &[("xsi:type", "dcterms:W3CDTF")], vec![XmlNode::Text(now.clone())])),
            XmlNode::Element(element_ns("modified", dt, &[("xsi:type", "dcterms:W3CDTF")], vec![XmlNode::Text(now.clone())])),
            value("language", dc, language),
        ],
    );
    doc.parts.insert("docProps/core.xml".into(), part(core, "application/vnd.openxmlformats-package.core-properties+xml"));
    let mut app = element_ns(
        "Properties",
        APP,
        &[("xmlns", APP), ("xmlns:vt", VT)],
        vec![
            value("Application", APP, "MisterDiagram"),
            value("ScaleCrop", APP, "false"),
            XmlNode::Element(element_ns("HeadingPairs", APP, &[], vec![])),
            XmlNode::Element(element_ns("TitlesOfParts", APP, &[], vec![])),
            value("LinksUpToDate", APP, "false"),
            value("SharedDoc", APP, "false"),
            value("HyperlinksChanged", APP, "false"),
            value("AppVersion", APP, "16.0000"),
        ],
    );
    // document() calls this with its fixed one-page, one-master initial model.
    // Later authoring uses the fallible refresh before serialization.
    summary(doc, &mut app);
    doc.parts.insert("docProps/app.xml".into(), part(app, "application/vnd.openxmlformats-officedocument.extended-properties+xml"));
    let ns = "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties";
    let mut children = vec![];
    for (i, (name, kind, text)) in [
        ("_VPID_ALTERNATENAMES", "lpwstr", ""),
        ("BuildNumberCreated", "i4", "1074417614"),
        ("BuildNumberEdited", "i4", "1074417614"),
        ("IsMetric", "bool", if metric { "true" } else { "false" }),
        ("TimeEdited", "filetime", now.as_str()),
    ]
    .into_iter()
    .enumerate()
    {
        children.push(XmlNode::Element(element_ns(
            "property",
            ns,
            &[("fmtid", "{D5CDD505-2E9C-101B-9397-08002B2CF9AE}"), ("pid", &(i + 2).to_string()), ("name", name)],
            vec![value(kind, VT, text)],
        )));
    }
    doc.parts.insert(
        "docProps/custom.xml".into(),
        part(
            element_ns("Properties", ns, &[("xmlns", ns), ("xmlns:vt", VT)], children),
            "application/vnd.openxmlformats-officedocument.custom-properties+xml",
        ),
    );
}
