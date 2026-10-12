//! Lossless Visio package data and typed ShapeSheet records. Page coordinates are inches, y-up.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShapeId(pub u32);
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Xfrm {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
impl Xfrm {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}

/// Namespace-expanded XML retained alongside original part bytes for future lossless writing.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
#[derive(Clone, Debug, Default, PartialEq)]
pub struct XmlElement {
    pub name: String,
    pub namespace: String,
    pub attributes: BTreeMap<String, String>,
    pub attribute_namespaces: BTreeMap<String, String>,
    pub children: Vec<XmlNode>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum XmlNode {
    Element(XmlElement),
    Text(String),
}
impl XmlElement {
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(String::as_str)
    }
    pub fn is_visio(&self, name: &str) -> bool {
        self.name == name
            && matches!(
                self.namespace.as_str(),
                "http://schemas.microsoft.com/office/visio/2012/main" | "http://schemas.microsoft.com/office/visio/2011/1/core"
            )
    }
    pub fn elements(&self) -> impl Iterator<Item = &Self> {
        self.children.iter().filter_map(|n| match n {
            XmlNode::Element(e) => Some(e),
            _ => None,
        })
    }
    pub fn child(&self, name: &str) -> Option<&Self> {
        self.elements().find(|e| e.is_visio(name))
    }
    pub fn text(&self) -> String {
        let mut text = String::new();
        let mut stack = vec![self.children.iter()];
        while let Some(iter) = stack.last_mut() {
            match iter.next() {
                Some(XmlNode::Text(s)) => text.push_str(s),
                Some(XmlNode::Element(e)) => stack.push(e.children.iter()),
                None => {
                    stack.pop();
                }
            }
        }
        text
    }
}

/// OPC relationship, including media, thumbnails and external targets.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-page_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Relationship {
    pub id: String,
    pub type_uri: String,
    pub target: String,
    pub external: bool,
}
/// Original bytes preserve unknown parts and unknown XML exactly; `xml` is a read-only structured view.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Part {
    pub bytes: Vec<u8>,
    pub content_type: Option<String>,
    pub xml: Option<XmlElement>,
    pub relationships: Vec<Relationship>,
}

/// Cell N, V, F, U and E are preserved without interpreting cached values as formulas.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cell {
    pub name: String,
    pub value: Option<String>,
    pub formula: Option<String>,
    pub units: Option<String>,
    pub error: Option<String>,
    pub attributes: BTreeMap<String, String>,
    pub ref_by: Vec<XmlElement>,
}
/// Section N, IX and Del; all named sections use the same lossless row model.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Section {
    pub name: String,
    pub index: Option<u32>,
    pub deleted: bool,
    pub cells: Vec<Cell>,
    pub rows: Vec<Row>,
    pub attributes: BTreeMap<String, String>,
}
/// Row N, IX, T, Del and LocalName are retained, including local deletion markers.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Row {
    pub name: Option<String>,
    pub index: Option<u32>,
    pub row_type: Option<String>,
    pub deleted: bool,
    pub cells: Vec<Cell>,
    pub attributes: BTreeMap<String, String>,
}
/// Generic sheet used by document, page, style, master and shape records.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/documentsheet-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/trigger-elementvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/refby-element-cell_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sheet {
    pub cells: Vec<Cell>,
    pub sections: Vec<Section>,
    pub triggers: Vec<XmlElement>,
    pub attributes: BTreeMap<String, String>,
}
impl Sheet {
    pub fn cell(&self, name: &str) -> Option<&Cell> {
        self.cells.iter().find(|c| c.name.eq_ignore_ascii_case(name))
    }
}
/// Shape ID, Type, Master, MasterShape, style IDs, Name, NameU, IsCustomName and all extension attributes.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Shape {
    pub id: u32,
    pub name: String,
    pub name_u: String,
    pub shape_type: Option<String>,
    pub master: Option<u32>,
    pub master_shape: Option<u32>,
    pub line_style: Option<u32>,
    pub fill_style: Option<u32>,
    pub text_style: Option<u32>,
    pub sheet: Sheet,
    pub shapes: Vec<Shape>,
    pub text: Option<XmlElement>,
    pub foreign_data: Option<XmlElement>,
}
/// StyleSheet ID and formatting inheritance attributes are preserved in `sheet.attributes`.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    pub id: u32,
    pub name: String,
    pub name_u: String,
    pub line_style: Option<u32>,
    pub fill_style: Option<u32>,
    pub text_style: Option<u32>,
    pub sheet: Sheet,
}
/// Connect FromSheet, FromCell, FromPart, ToSheet, ToCell and ToPart.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Connect {
    pub from_sheet: u32,
    pub from_cell: Option<String>,
    pub from_part: Option<i32>,
    pub to_sheet: u32,
    pub to_cell: Option<String>,
    pub to_part: Option<i32>,
    pub attributes: BTreeMap<String, String>,
}
/// Page ID, Name, NameU, background and view attributes, plus the related contents.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagesheet-element-page_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Page {
    pub id: u32,
    pub name: String,
    pub name_u: String,
    pub sheet: Sheet,
    pub shapes: Vec<Shape>,
    pub connects: Vec<Connect>,
    pub part: String,
    pub attributes: BTreeMap<String, String>,
}
/// Master ID, UniqueID, BaseID, Icon and master contents; all metadata attributes retained.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/icon-element-master_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Master {
    pub id: u32,
    pub name: String,
    pub name_u: String,
    pub unique_id: Option<String>,
    pub base_id: Option<String>,
    pub icon: Option<XmlElement>,
    pub sheet: Sheet,
    pub shapes: Vec<Shape>,
    pub connects: Vec<Connect>,
    pub part: String,
    pub attributes: BTreeMap<String, String>,
}
/// Visio document, stencil or template; package metadata and unknown content remain in `parts`.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/documentsettings-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colorentry-element-colors_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/facenames-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/facename-element-facenames_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/eventlist-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/windows-elementvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/mastershortcut-element-masters_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    pub main_part: String,
    pub document_sheet: Sheet,
    pub document_settings: Option<XmlElement>,
    pub event_list: Option<XmlElement>,
    pub styles: Vec<Style>,
    pub pages: Vec<Page>,
    pub masters: Vec<Master>,
    pub master_shortcuts: Vec<XmlElement>,
    pub colors: BTreeMap<u32, String>,
    pub face_names: BTreeMap<u32, String>,
    pub parts: BTreeMap<String, Part>,
    pub root_relationships: Vec<Relationship>,
    pub macros_removed: bool,
}
