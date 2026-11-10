//! Font authoring uses face names while untouched imported literals remain intact.
use misterdiagram_engine::Session;
use misterdiagram_model::XmlElement;
use serde_json::{Value, json};
use std::io::{Cursor, Read};
fn run(s: &mut Session, command: &str, p: Value) -> Value {
    s.execute(command, &p).unwrap()
}
fn rect(s: &mut Session) -> Value {
    run(s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1}))["id"].clone()
}
fn save(s: &Session) -> Vec<u8> {
    misterdiagram_vsdx::write(s.document()).unwrap().bytes
}
fn part(bytes: &[u8], path: &str) -> String {
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut text = String::new();
    zip.by_name(path).unwrap().read_to_string(&mut text).unwrap();
    text
}
fn fonts(root: &XmlElement) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if node.name == "Cell" && node.attr("N") == Some("Font") {
            out.push((node.attr("V").unwrap().into(), node.attr("F").map(str::to_owned)));
        }
        stack.extend(node.elements());
    }
    out
}
fn page_fonts(bytes: &[u8]) -> Vec<(String, Option<String>)> {
    fonts(&misterdiagram_vsdx::parse_xml(part(bytes, "visio/pages/page1.xml").as_bytes()).unwrap())
}
fn dir(name: &str) -> std::path::PathBuf {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/font-name-tests").join(std::process::id().to_string()).join(name);
    std::fs::create_dir_all(&path).unwrap();
    path
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/font-cell-character-section
#[test]
fn authored_font_names_are_saved_and_reopened() {
    let mut s = Session::new();
    let a = rect(&mut s);
    let b = rect(&mut s);
    rect(&mut s);
    run(&mut s, "format.text", json!({"id":a,"font":"Arial"}));
    run(&mut s, "format.text", json!({"id":b,"font":"Courier New"}));
    let bytes = save(&s);
    let xml = part(&bytes, "visio/pages/page1.xml");
    assert!(xml.contains("<Cell N=\"Font\" V=\"Arial\"/>"));
    assert!(xml.contains("<Cell N=\"Font\" V=\"Courier New\"/>"));
    let cells = page_fonts(&bytes);
    assert_eq!(cells.len(), 2);
    assert!(cells.iter().all(|(value, formula)| value.parse::<i64>().is_err() && formula.is_none()));
    let path = dir("witness").join("drawing.vsdx");
    std::fs::write(&path, bytes).unwrap();
    let mut reopened = Session::new();
    run(&mut reopened, "file.open", json!({"path":path}));
    for (id, name) in [(a, "Arial"), (b, "Courier New")] {
        assert_eq!(run(&mut reopened, "shape.inspect", json!({"id":id,"sections":["style"]}))["style"]["text"]["font"], name);
        assert!(reopened.document().face_names.values().any(|face| face == name));
    }
}
#[test]
fn formatting_one_shape_preserves_other_numeric_and_named_font_cells() {
    let mut s = Session::new();
    rect(&mut s);
    rect(&mut s);
    rect(&mut s);
    run(&mut s, "sheet.set", json!({"id":1,"name":"Char.Font","value":0}));
    run(&mut s, "format.text", json!({"id":2,"font":"Arial"}));
    let before = save(&s);
    let page_before = part(&before, "visio/pages/page1.xml");
    let path = dir("preservation").join("old.vsdx");
    std::fs::write(&path, &before).unwrap();
    let mut reopened = Session::new();
    run(&mut reopened, "file.open", json!({"path":path}));
    let untouched = save(&reopened);
    assert_eq!(part(&untouched, "visio/pages/page1.xml"), page_before);
    // Whole document XML equality includes the original FaceNames bytes.
    assert_eq!(part(&untouched, "visio/document.xml"), part(&before, "visio/document.xml"));
    run(&mut reopened, "format.text", json!({"id":3,"font":"Courier New"}));
    let after = save(&reopened);
    let page_after = part(&after, "visio/pages/page1.xml");
    fn shape_bytes(page: &str, id: &str) -> String {
        let attribute = format!(" ID=\"{id}\"");
        let start = page
            .match_indices("<Shape ")
            .find_map(|(start, _)| {
                let tag = page[start..].split_once('>')?.0;
                tag.contains(&attribute).then_some(start)
            })
            .unwrap();
        let tail = &page[start..];
        let end = tail.find("</Shape>").unwrap() + "</Shape>".len();
        tail[..end].into()
    }
    for id in ["1", "2"] {
        assert_eq!(shape_bytes(&page_before, id), shape_bytes(&page_after, id));
    }
    assert!(page_fonts(&after).contains(&("0".into(), None)));
    assert!(page_fonts(&after).contains(&("Courier New".into(), None)));
}
#[test]
fn composition_and_paragraph_fonts_are_names_without_formulas() {
    let mut s = Session::new();
    run(&mut s, "shape.draw.rect", json!({"x":1,"y":1,"width":2,"height":1,"font":{"font":"Arial"}}));
    run(&mut s, "shape.draw.ellipse", json!({"x":4,"y":1,"width":2,"height":1,"font":{"font":"Courier New"}}));
    run(&mut s, "master.new_from_shape", json!({"id":1,"name":"Original font master"}));
    run(&mut s, "master.drop", json!({"master":"Original font master","x":7,"y":2,"font":{"font":"Courier New"}}));
    let p = rect(&mut s);
    run(&mut s, "shape.text", json!({"id":p,"paragraphs":[{"runs":[{"text":"First","font":"Arial"},{"text":"Second","font":"Courier New"}]}]}));
    let cells = page_fonts(&save(&s));
    assert!(cells.len() >= 5);
    assert!(cells.iter().all(|(name, formula)| matches!(name.as_str(), "Arial" | "Courier New") && formula.is_none()));
}
#[test]
fn imported_master_literal_fonts_are_remapped_by_name() {
    let mut source = Session::new();
    rect(&mut source);
    run(&mut source, "format.text", json!({"id":1,"font":"Arial"}));
    run(&mut source, "sheet.set", json!({"id":1,"name":"Char.Font","value":1}));
    run(&mut source, "master.new_from_shape", json!({"id":1,"name":"Original imported face"}));
    let path = dir("import").join("original.vssx");
    run(&mut source, "file.save", json!({"path":path}));
    let original = std::fs::read(&path).unwrap();
    let mut destination = Session::new();
    rect(&mut destination);
    run(&mut destination, "format.text", json!({"id":1,"font":"Courier New"}));
    run(&mut destination, "stencil.open", json!({"path":path}));
    let id = run(&mut destination, "master.drop", json!({"stencil":1,"master":"Original imported face","x":5,"y":2}))["id"].clone();
    assert_eq!(run(&mut destination, "shape.inspect", json!({"id":id,"sections":["style"]}))["style"]["text"]["font"], "Arial");
    let master = destination.document().masters.iter().find(|m| m.name == "Original imported face").unwrap();
    let bytes = save(&destination);
    let cells = fonts(&misterdiagram_vsdx::parse_xml(part(&bytes, &master.part).as_bytes()).unwrap());
    assert!(cells.contains(&("Arial".into(), None)));
    assert!(cells.iter().all(|(name, _)| name.parse::<u32>().is_err()));
    assert_eq!(std::fs::read(&path).unwrap(), original);
}
