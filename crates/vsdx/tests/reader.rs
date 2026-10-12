//! Reader contracts on contributor-authored packages and local private corpus data.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use diagramcraft_opc::{PackageOut, RelsOut};
use diagramcraft_vsdx::{import, parse_sheet, parse_xml};
fn package(document: &str, page: &str, namespace: &str, macro_enabled: bool) -> Vec<u8> {
    let mut p = PackageOut::default();
    let mut rel = RelsOut::default();
    rel.add("http://schemas.microsoft.com/visio/2010/relationships/document", "visio/document.xml", false);
    p.rels("", &rel);
    let kind = if macro_enabled { "application/vnd.ms-visio.drawing.macroEnabled.main+xml" } else { "application/vnd.ms-visio.drawing.main+xml" };
    p.xml_part("visio/document.xml", kind, format!("<VisioDocument xmlns='{namespace}'>{document}</VisioDocument>").into_bytes());
    let mut rel = RelsOut::default();
    rel.add("http://schemas.microsoft.com/visio/2010/relationships/pages", "pages/pages.xml", false);
    p.rels("visio/document.xml", &rel);
    p.xml_part("visio/pages/pages.xml","application/vnd.ms-visio.pages+xml",format!("<Pages xmlns='{namespace}' xmlns:r='http://schemas.openxmlformats.org/officeDocument/2006/relationships'><Page ID='7' Name='Page'><PageSheet><Cell N='PageWidth' V='11'/></PageSheet><Rel r:id='rId1'/></Page></Pages>").into_bytes());
    let mut rel = RelsOut::default();
    rel.add("http://schemas.microsoft.com/visio/2010/relationships/page", "page1.xml", false);
    p.rels("visio/pages/pages.xml", &rel);
    p.xml_part(
        "visio/pages/page1.xml",
        "application/vnd.ms-visio.page+xml",
        format!("<PageContents xmlns='{namespace}'>{page}</PageContents>").into_bytes(),
    );
    p.binary("custom/opaque.bin", "application/octet-stream", vec![0, 255, 13, 10]);
    p.xml_part("custom/unknown.xml", "application/xml", b"<x>  unknown &#65; </x>".to_vec());
    if macro_enabled {
        p.binary("visio/vbaProject.bin", "application/vnd.ms-office.vbaProject", b"never expose".to_vec());
    }
    p.finish().unwrap()
}
#[test]
fn reads_shape_cells_sections_text_and_unknown_content() {
    for ns in ["http://schemas.microsoft.com/office/visio/2012/main", "http://schemas.microsoft.com/office/visio/2011/1/core"] {
        let b = package(
            "<Colors><ColorEntry IX='24' RGB='#123456'/></Colors><FaceNames><FaceName NameU='Example'/></FaceNames><DocumentSheet><Cell N='DocLangID' V='1033'/></DocumentSheet>",
            "<Shapes><Shape ID='3' Type='Group' Name='Local' NameU='Universal' IsCustomName='1' Master='2' MasterShape='9' LineStyle='1' FillStyle='2' TextStyle='3'><Cell N='Width' V='2' U='IN' F='1+1' E='#REF!'><RefBy ID='7' T='Page'/></Cell><Cell N='Missing'/><Section N='Geometry' IX='0' Del='0'><Row IX='1' T='MoveTo'><Cell N='X' V='0'/></Row><Row N='Named' Del='1'/></Section><Trigger N='EventXFMod'/><Text><cp IX='0'/>Hello<pp IX='1'/><tp IX='2'/><fld IX='3'>world</fld></Text><ForeignData ForeignType='Bitmap'/><Shapes><Shape ID='4'/></Shapes><x:Cell xmlns:x='urn:extension' N='Width' V='999'/></Shape></Shapes><Connects><Connect FromSheet='3' FromCell='BeginX' FromPart='9' ToSheet='4' ToCell='PinX' ToPart='3'/></Connects>",
            ns,
            false,
        );
        let d = import(&b).unwrap();
        assert_eq!(d.pages.len(), 1);
        let p = &d.pages[0];
        assert_eq!(p.id, 7);
        assert_eq!(p.connects.len(), 1);
        let s = &p.shapes[0];
        assert_eq!((s.master, s.master_shape), (Some(2), Some(9)));
        assert_eq!(s.sheet.cells.len(), 2);
        assert_eq!(s.sheet.cells[1].value, None);
        assert_eq!(s.sheet.cells[0].error.as_deref(), Some("#REF!"));
        assert_eq!(s.sheet.sections[0].rows[0].row_type.as_deref(), Some("MoveTo"));
        assert!(s.sheet.sections[0].rows[1].deleted);
        assert_eq!(s.shapes[0].id, 4);
        assert_eq!(s.text.as_ref().unwrap().text(), "Helloworld");
        assert_eq!(d.colors[&24], "#123456");
        assert_eq!(d.face_names[&0], "Example");
        assert_eq!(d.parts["custom/opaque.bin"].bytes, [0, 255, 13, 10]);
        assert_eq!(d.parts["custom/unknown.xml"].bytes, b"<x>  unknown &#65; </x>");
        assert!(d.parts["custom/unknown.xml"].xml.is_none());
        assert_eq!(d.parts["visio/pages/page1.xml"].bytes, diagramcraft_opc::Package::open(&b).unwrap().get("visio/pages/page1.xml").unwrap());
    }
}
#[test]
fn macro_payload_removed_and_flagged() {
    let b = package("", "", "http://schemas.microsoft.com/office/visio/2012/main", true);
    let d = import(&b).unwrap();
    assert!(d.macros_removed);
    assert!(!d.parts.keys().any(|s| s.ends_with("vbaProject.bin")));
}
#[test]
fn hostile_xml_and_sheet_entry_points() {
    for b in [b"".as_slice(), b"<Shape>", b"<a><b></a>", b"<!DOCTYPE x><x/>", b"<x:a/>", b"<a xmlns:x='u' xmlns:y='u' x:p='1' y:p='2'/>"] {
        assert!(parse_xml(b).is_err(), "{b:?}");
    }
    let deep = format!("{}{}", "<a>".repeat(300), "</a>".repeat(300));
    assert!(parse_xml(deep.as_bytes()).is_err());
    for attr in ["IX='4294967296'", "IX='-1'", "Del='maybe'"] {
        let xml = format!("<Shape xmlns='http://schemas.microsoft.com/office/visio/2012/main'><Section N='Geometry' {attr}/></Shape>");
        assert!(parse_sheet(&parse_xml(xml.as_bytes()).unwrap()).is_err());
    }
    assert!(diagramcraft_vsdx::read("/a-file-that-does-not-exist-diagramcraft").is_err());
}
#[test]
fn namespace_rebinding_cannot_supply_visio_cells() {
    let xml=parse_xml(b"<Shape xmlns='http://schemas.microsoft.com/office/visio/2012/main'><Cell xmlns='urn:other' N='Width' V='42'/><Cell N='Height' V='3'/></Shape>").unwrap();
    let s = parse_sheet(&xml).unwrap();
    assert_eq!(s.cells.len(), 1);
    assert_eq!(s.cells[0].name, "Height");
}
#[test]
fn public_sheet_parser_bounds_caller_built_trees() {
    use diagramcraft_model::{XmlElement, XmlNode};
    let mut e = XmlElement::default();
    for _ in 0..300 {
        e = XmlElement {
            name: "Trigger".into(),
            namespace: "http://schemas.microsoft.com/office/visio/2012/main".into(),
            children: vec![XmlNode::Element(e)],
            ..Default::default()
        };
    }
    assert!(parse_sheet(&e).is_err());
}
#[test]
fn every_named_section_is_retained() {
    let names = [
        "Hyperlink",
        "Property",
        "User",
        "Actions",
        "Control",
        "Controls",
        "Connection",
        "Geometry",
        "Character",
        "Paragraph",
        "Tabs",
        "Scratch",
        "Layer",
        "LayerMem",
        "FillGradient",
        "LineGradient",
    ];
    let body:String=names.iter().map(|n|format!("<Section N='{n}'><Row N='Example' IX='2' LocalName='local'><Cell N='Label' V='value'/><Cell N='Type' V='0'/><Cell N='Format' V='format'/><Cell N='Invisible' V='1'/><Cell N='SortKey' V='2'/></Row></Section>")).collect();
    let e = parse_xml(format!("<Shape xmlns='http://schemas.microsoft.com/office/visio/2012/main'>{body}</Shape>").as_bytes()).unwrap();
    let s = parse_sheet(&e).unwrap();
    assert_eq!(s.sections.len(), names.len());
    for (section, name) in s.sections.iter().zip(names) {
        assert_eq!(section.name, name);
        assert_eq!(section.rows[0].cells.len(), 5);
        assert_eq!(section.rows[0].attributes["LocalName"], "local");
    }
}
#[test]
fn inherited_empty_page_without_rel_is_valid() {
    let b = package("", "", "http://schemas.microsoft.com/office/visio/2012/main", false);
    let p = diagramcraft_opc::Package::open(&b).unwrap();
    let mut out = std::io::Cursor::new(Vec::new());
    {
        use std::io::Write;
        let mut z = zip::ZipWriter::new(&mut out);
        for (n, d) in p.parts() {
            z.start_file(n, zip::write::SimpleFileOptions::default()).unwrap();
            if n == "visio/pages/pages.xml" {
                z.write_all(String::from_utf8_lossy(d).replace("<Rel r:id='rId1'/>", "").as_bytes()).unwrap();
            } else {
                z.write_all(d).unwrap();
            }
        }
        z.finish().unwrap();
    }
    let d = import(out.get_ref()).unwrap();
    assert_eq!(d.pages.len(), 1);
    assert!(d.pages[0].shapes.is_empty());
    assert_eq!(d.pages[0].sheet.cells[0].name, "PageWidth");
}
#[test]
fn known_metadata_trees_and_relationship_targets_are_available() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/generated/basic/0001-probe.vsdx");
    if !path.exists() {
        return;
    }
    let d = diagramcraft_vsdx::read(path).unwrap();
    assert!(d.parts["docProps/core.xml"].xml.is_some());
    assert!(d.parts["docProps/app.xml"].xml.is_some());
    assert!(
        d.parts
            .values()
            .filter(|p| p.content_type.as_deref() == Some("application/vnd.openxmlformats-officedocument.theme+xml"))
            .all(|p| p.xml.is_some())
    );
    assert_eq!(d.root_relationships.len(), 5);
    assert!(!d.masters.is_empty());
    assert!(!d.styles.is_empty());
    assert!(d.document_settings.is_some());
}
