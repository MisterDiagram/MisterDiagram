//! Lossless Visio package data and typed ShapeSheet records. Page coordinates are inches, y-up.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use std::collections::BTreeMap;
mod compact;
pub use compact::{Attributes, XmlString};

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

/// XmlString-expanded XML retained alongside original part bytes for future lossless writing.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
#[derive(Clone, Debug, Default, PartialEq)]
pub struct XmlElement {
    pub name: XmlString,
    pub namespace: XmlString,
    pub attributes: Attributes,
    pub attribute_namespaces: Attributes,
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
    /// Relationship identifier used by r:id.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-page_type-complextypevisio-xml
    pub id: String,
    /// OPC relationship type URI.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-page_type-complextypevisio-xml
    pub type_uri: String,
    /// OPC relationship target.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-page_type-complextypevisio-xml
    pub target: String,
    /// Whether TargetMode is External.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-page_type-complextypevisio-xml
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
    /// N attribute: universal cell name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub name: String,
    /// V attribute: optional cached value, preserving an empty value.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub value: Option<String>,
    /// F attribute: formula or inheritance sentinel.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub formula: Option<String>,
    /// U attribute: optional unit code.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub units: Option<String>,
    /// E attribute: optional saved error.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub error: Option<String>,
    /// All remaining XML attributes, preserved by name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub attributes: Attributes,
    /// RefBy child elements.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub ref_by: Vec<XmlElement>,
}
/// Section N, IX and Del; all named sections use the same lossless row model.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Section {
    /// N attribute: section name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
    pub name: String,
    /// IX attribute: optional repeated-section index.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
    pub index: Option<u32>,
    /// Del attribute: inherited-section deletion marker.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
    pub deleted: bool,
    /// Section-level Cell child elements.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
    pub cells: Vec<Cell>,
    /// Row child elements.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
    pub rows: Vec<Row>,
    /// All remaining XML attributes, preserved by name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
    pub attributes: Attributes,
}
/// Row N, IX, T, Del and LocalName are retained, including local deletion markers.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Row {
    /// N attribute: optional named-row identifier.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
    pub name: Option<String>,
    /// IX attribute: optional logical row index.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
    pub index: Option<u32>,
    /// T attribute: optional row type.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
    pub row_type: Option<String>,
    /// Del attribute: inherited-row deletion marker.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
    pub deleted: bool,
    /// Cell child elements.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
    pub cells: Vec<Cell>,
    /// All remaining XML attributes, preserved by name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
    pub attributes: Attributes,
}
/// Generic sheet used by document, page, style, master and shape records.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/documentsheet-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/trigger-elementvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/refby-element-cell_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sheet {
    /// Cell child elements.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/documentsheet-element-visiodocument_type-complextypevisio-xml
    pub cells: Vec<Cell>,
    /// Section child elements.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/documentsheet-element-visiodocument_type-complextypevisio-xml
    pub sections: Vec<Section>,
    /// Trigger child elements.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/documentsheet-element-visiodocument_type-complextypevisio-xml
    pub triggers: Vec<XmlElement>,
    /// All remaining XML attributes, preserved by name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/documentsheet-element-visiodocument_type-complextypevisio-xml
    pub attributes: Attributes,
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
    /// ID attribute, scoped to the page or master.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub id: u32,
    /// Name attribute: localized shape name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub name: String,
    /// NameU attribute: universal shape name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub name_u: String,
    /// Type attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub shape_type: Option<String>,
    /// Master attribute: referenced master identifier.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub master: Option<u32>,
    /// MasterShape attribute: referenced shape inside a master.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub master_shape: Option<u32>,
    /// LineStyle attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub line_style: Option<u32>,
    /// FillStyle attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub fill_style: Option<u32>,
    /// TextStyle attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub text_style: Option<u32>,
    /// ShapeSheet cells and sections.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub sheet: Sheet,
    /// Nested Shapes child element.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub shapes: Vec<Shape>,
    /// Text child element, including run markers.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub text: Option<XmlElement>,
    /// ForeignData child element.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
    pub foreign_data: Option<XmlElement>,
}
/// StyleSheet ID and formatting inheritance attributes are preserved in `sheet.attributes`.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    /// ID attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
    pub id: u32,
    /// Name attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
    pub name: String,
    /// NameU attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
    pub name_u: String,
    /// LineStyle attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
    pub line_style: Option<u32>,
    /// FillStyle attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
    pub fill_style: Option<u32>,
    /// TextStyle attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
    pub text_style: Option<u32>,
    /// StyleSheet cells and sections.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheet-element-stylesheets_type-complextypevisio-xml
    pub sheet: Sheet,
}
/// Connect FromSheet, FromCell, FromPart, ToSheet, ToCell and ToPart.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Connect {
    /// FromSheet attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
    pub from_sheet: u32,
    /// FromCell attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
    pub from_cell: Option<String>,
    /// FromPart attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
    pub from_part: Option<i32>,
    /// ToSheet attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
    pub to_sheet: u32,
    /// ToCell attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
    pub to_cell: Option<String>,
    /// ToPart attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
    pub to_part: Option<i32>,
    /// All remaining XML attributes, preserved by name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
    pub attributes: Attributes,
}
/// Page ID, Name, NameU, background and view attributes, plus the related contents.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pagesheet-element-page_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Page {
    /// ID attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
    pub id: u32,
    /// Name attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
    pub name: String,
    /// NameU attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
    pub name_u: String,
    /// PageSheet child element.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
    pub sheet: Sheet,
    /// Shapes in the related PageContents part.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
    pub shapes: Vec<Shape>,
    /// Connect records in the related PageContents part.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
    pub connects: Vec<Connect>,
    /// Resolved PageContents part name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
    pub part: String,
    /// All remaining XML attributes, preserved by name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
    pub attributes: Attributes,
}
/// Master ID, UniqueID, BaseID, Icon and master contents; all metadata attributes retained.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/icon-element-master_type-complextypevisio-xml
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Master {
    /// ID attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub id: u32,
    /// Name attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub name: String,
    /// NameU attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub name_u: String,
    /// UniqueID attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub unique_id: Option<String>,
    /// BaseID attribute.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub base_id: Option<String>,
    /// Icon child element retained for local observation.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub icon: Option<XmlElement>,
    /// Related master PageSheet.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub sheet: Sheet,
    /// Shapes in the related MasterContents part.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub shapes: Vec<Shape>,
    /// Connect records in the related MasterContents part.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub connects: Vec<Connect>,
    /// Resolved MasterContents part name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub part: String,
    /// All remaining XML attributes, preserved by name.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
    pub attributes: Attributes,
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
    /// Logical XML nodes charged by the reader, including package metadata.
    pub retained_xml_nodes: usize,
    /// OPC part containing VisioDocument.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub main_part: String,
    /// DocumentSheet child element.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub document_sheet: Sheet,
    /// DocumentSettings child element.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub document_settings: Option<XmlElement>,
    /// EventList child element.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub event_list: Option<XmlElement>,
    /// StyleSheet records in StyleSheets.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub styles: Vec<Style>,
    /// Pages reached through the package relationships.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub pages: Vec<Page>,
    /// Masters reached through the package relationships.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub masters: Vec<Master>,
    /// MasterShortcut child elements.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub master_shortcuts: Vec<XmlElement>,
    /// ColorEntry values from the Colors table.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub colors: BTreeMap<u32, String>,
    /// Font names from FaceName records.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub face_names: BTreeMap<u32, String>,
    /// Original package parts and structured views.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub parts: BTreeMap<String, Part>,
    /// Package-root relationships.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub root_relationships: Vec<Relationship>,
    /// Whether VBA package content was removed during import.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
    pub macros_removed: bool,
}

pub mod text;

pub mod skeleton;
