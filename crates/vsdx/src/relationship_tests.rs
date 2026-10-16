// Review 1a relationship regressions, scaled with Charles Hood's permission.
// MIT OR Apache-2.0. Source recorded in ATTRIBUTION.md.
use super::*;
use std::io::Write;

fn package(kind: &str, count: usize, repeat_collection: usize, repeat_content: bool) -> Vec<u8> {
    package_with_mode(kind, count, repeat_collection, repeat_content, "Internal")
}
fn package_with_mode(kind: &str, count: usize, repeat_collection: usize, repeat_content: bool, mode: &str) -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default();
    let ns = "http://schemas.microsoft.com/office/visio/2012/main";
    let collection = format!("{kind}s");
    let title = if kind == "page" { "Page" } else { "Master" };
    let mut listing = format!("<{title}s xmlns='{ns}' xmlns:r='{ATTR_REL_NS}'>");
    let mut rels = format!("<Relationships xmlns='{REL_NS}'>");
    for i in 0..count {
        listing.push_str(&format!("<{title} ID='{i}'><Rel r:id='r{i}'/></{title}>"));
        let target = if repeat_content { 0 } else { i };
        rels.push_str(&format!("<Relationship Id='r{i}' Type='{VIS_REL}{kind}' Target='{kind}{target}.xml'/>"));
    }
    listing.push_str(&format!("</{title}s>"));
    rels.push_str("</Relationships>");
    let mut document_rels = format!("<Relationships xmlns='{REL_NS}'>");
    for i in 0..repeat_collection {
        document_rels.push_str(&format!("<Relationship Id='r{i}' Type='{VIS_REL}{collection}' Target='{collection}/{collection}.xml'/>"));
    }
    document_rels.push_str("</Relationships>");
    let mut parts = vec![
        ("[Content_Types].xml".into(), "<Types xmlns='http://schemas.openxmlformats.org/package/2006/content-types'><Default Extension='xml' ContentType='application/xml'/><Default Extension='rels' ContentType='application/vnd.openxmlformats-package.relationships+xml'/><Override PartName='/visio/document.xml' ContentType='application/vnd.ms-visio.drawing.main+xml'/></Types>".into()),
        ("_rels/.rels".into(), format!("<Relationships xmlns='{REL_NS}'><Relationship Id='r0' Type='{VIS_REL}document' TargetMode='{mode}' Target='visio/document.xml'/></Relationships>")),
        ("visio/document.xml".into(), format!("<VisioDocument xmlns='{ns}'/>")),
        ("visio/_rels/document.xml.rels".into(), document_rels),
        (format!("visio/{collection}/{collection}.xml"), listing),
        (format!("visio/{collection}/_rels/{collection}.xml.rels"), rels),
    ];
    for i in 0..if repeat_content { 1 } else { count } {
        parts.push((format!("visio/{collection}/{kind}{i}.xml"), format!("<{title}Contents xmlns='{ns}'><Shapes/></{title}Contents>")));
    }
    for (name, xml) in parts {
        zip.start_file(name, options).unwrap();
        zip.write_all(xml.as_bytes()).unwrap();
    }
    zip.finish().unwrap().into_inner()
}

#[test]
fn many_pages_with_many_relationships_parse_each_table_once() {
    for kind in ["page", "master"] {
        RELATIONSHIP_PARSES.with(|n| n.set(0));
        let doc = import(&package(kind, 64, 1, false)).unwrap();
        assert_eq!(doc.pages.len() + doc.masters.len(), 64);
        assert_eq!(RELATIONSHIP_PARSES.with(|n| n.get()), 3, "each relationship part must be parsed once");
    }
}

#[test]
fn repeated_pages_relationships_do_not_multiply_pages() {
    for kind in ["page", "master"] {
        let doc = import(&package(kind, 8, 2, false)).unwrap();
        assert_eq!(doc.pages.len() + doc.masters.len(), 8, "duplicate collection target");
        let doc = import(&package(kind, 8, 2, true)).unwrap();
        assert_eq!(doc.pages.len() + doc.masters.len(), 1, "duplicate content target");
    }
}

#[test]
fn indexed_relationships_keep_package_case_insensitive_lookup() {
    use std::io::Read;
    for kind in ["page", "master"] {
        let bytes = package(kind, 3, 1, false);
        let mut input = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let mut output = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        for i in 0..input.len() {
            let mut part = input.by_index(i).unwrap();
            let mut text = String::new();
            part.read_to_string(&mut text).unwrap();
            if part.name().ends_with(".rels") {
                let mut changed = String::new();
                let mut pieces = text.split("Target='");
                changed.push_str(pieces.next().unwrap());
                for piece in pieces {
                    let (target, rest) = piece.split_once('\'').unwrap();
                    changed.push_str("Target='");
                    changed.push_str(&target.to_ascii_uppercase());
                    changed.push('\'');
                    changed.push_str(rest);
                }
                text = changed;
            }
            output.start_file(part.name(), zip::write::SimpleFileOptions::default()).unwrap();
            output.write_all(text.as_bytes()).unwrap();
        }
        let doc = import(&output.finish().unwrap().into_inner()).unwrap();
        assert_eq!(doc.pages.len() + doc.masters.len(), 3);
        assert_eq!(doc.parts[&format!("visio/{kind}s/{kind}s.xml")].relationships.len(), 3);
    }
}

#[test]
fn root_external_relationship_validation_is_preserved() {
    for mode in ["External", "external", "eXtErNaL"] {
        let bytes = package_with_mode("page", 1, 1, false, mode);
        assert!(Package::open(&bytes).unwrap().validate_visio().is_err());
        assert!(import(&bytes).unwrap_err().to_string().contains("missing document relationship"));
    }
}
