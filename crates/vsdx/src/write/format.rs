//! Drawing, stencil and template package kinds use distinct main-part content types.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveKind {
    Drawing,
    Stencil,
    Template,
}
impl SaveKind {
    pub fn content_type(self) -> &'static str {
        match self {
            Self::Drawing => "application/vnd.ms-visio.drawing.main+xml",
            Self::Stencil => "application/vnd.ms-visio.stencil.main+xml",
            Self::Template => "application/vnd.ms-visio.template.main+xml",
        }
    }
}
/// Save a document as a drawing, its document stencil, or a reusable drawing template.
/// The caller's document is unchanged. Stencils retain a PageSheet-only metadata entry,
/// as observed in BASIC_U.vssx, and an empty Windows collection.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pages-elementvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/masters-elementvisio-xml
pub fn write_as(doc: &Document, kind: SaveKind) -> Result<Written, WriteError> {
    write_as_with_hosts(doc, kind, None, None)
}
pub fn write_as_with_hosts(
    doc: &Document,
    kind: SaveKind,
    theme: Option<&dyn diagramcraft_sheet::host::ThemeHost>,
    text: Option<&dyn diagramcraft_model::text::TextHost>,
) -> Result<Written, WriteError> {
    validate::document(doc)?;
    let mut d = doc.clone();
    d.parts.entry(d.main_part.clone()).or_default().content_type = Some(kind.content_type().into());
    if kind == SaveKind::Stencil {
        let removed: BTreeSet<_> = d.pages.iter().filter(|p| !p.part.is_empty()).map(|p| p.part.clone()).collect();
        d.parts.retain(|name, _| !removed.contains(name) && !removed.iter().any(|p| diagramcraft_opc::rels_path(p) == *name));
        for part in d.parts.values_mut() {
            part.relationships.retain(|r| r.external || !removed.contains(&r.target));
        }
        d.root_relationships.retain(|r| r.external || !removed.contains(&r.target));
        let path = d.parts.get(&d.main_part).and_then(|p| related(&p.relationships, "pages")).unwrap_or_else(|| "visio/pages/pages.xml".into());
        let mut page = d.pages.first().cloned().unwrap_or_else(|| skeleton::page(0, "Page-1", false));
        page.shapes.clear();
        page.connects.clear();
        page.part.clear();
        // A document stencil has no background-page relationship.
        page.attributes.remove("BackPage");
        page.attributes.remove("Background");
        // PageSheet-only stencil entries have no drawing view. Visio preserves zeros.
        // Witness: scratch__three-masters/saved.vssx, visio/pages/pages.xml.
        for name in ["ViewScale", "ViewCenterX", "ViewCenterY"] {
            page.attributes.insert(name.into(), "0".into());
        }
        let mut pages = source(&d, &path).cloned().unwrap_or_else(|| node("Pages"));
        let mut entry = pages
            .elements()
            .find(|e| e.is_visio("Page") && e.attr("ID").and_then(|n| n.parse::<u32>().ok()) == Some(page.id))
            .cloned()
            .unwrap_or_else(|| node("Page"));
        set(&mut entry, "ID", Some(page.id.to_string()));
        replace(&mut entry, "Rel", vec![]);
        replace(&mut pages, "Page", vec![entry]);
        put(&mut d.parts, &path, pages, "application/vnd.ms-visio.pages+xml");
        if let Some(p) = d.parts.get_mut(&path) {
            p.relationships.retain(|r| r.external || r.type_uri != format!("{}page", crate::VIS_REL));
            if p.relationships.is_empty() {
                d.parts.remove(&diagramcraft_opc::rels_path(&path));
            }
        }
        if let Some(settings) = &mut d.document_settings {
            set(settings, "TopPage", Some(page.id.to_string()));
        }
        d.pages = vec![page];
        let windows = d.parts.get(&d.main_part).and_then(|p| related(&p.relationships, "windows"));
        if let Some(root) = windows.and_then(|p| d.parts.get_mut(&p)).and_then(|p| p.xml.as_mut()) {
            replace(root, "Window", vec![]);
        }
    }
    write_with_hosts(&d, theme, text)
}
