//! Immutable source reuse retains the ordinary strict serializer and reader contract.
use misterdiagram_model::{Part, skeleton};
use misterdiagram_vsdx::{MAX_RETAINED_XML_NODES, import, import_with_open_nodes, import_written_parts, write, write::write_parts_with_glue_host};

#[test]
fn unchanged_parts_share_but_edited_page_and_picture_replace_their_source() {
    let mut original = skeleton::document(false);
    original.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "Original").unwrap());
    original.parts.insert(
        "visio/media/original.bin".into(),
        Part { bytes: vec![17, 31, 47].into(), content_type: Some("image/x-original-test".into()), ..Default::default() },
    );
    let original = import(&write(&original).unwrap().bytes).unwrap();
    let mut changed = original.clone();
    changed.pages[0].shapes[0].sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap().value = Some("3".into());
    changed.parts.get_mut("visio/media/original.bin").unwrap().bytes = vec![61, 71].into();
    let ordinary = import(&write(&changed).unwrap().bytes).unwrap();
    let reused = import_written_parts(write_parts_with_glue_host(&changed, None, None, None).unwrap(), 0).unwrap();
    assert_eq!(reused, ordinary);
    assert_eq!(reused.retained_xml_nodes, ordinary.retained_xml_nodes);
    assert!(!reused.parts[&reused.pages[0].part].bytes.ptr_eq(&original.parts[&original.pages[0].part].bytes));
    assert_eq!(reused.parts["visio/media/original.bin"].bytes.as_ref(), &[61, 71]);
    assert_eq!(original.parts["visio/media/original.bin"].bytes.as_ref(), &[17, 31, 47]);
    let reused_again = import_written_parts(write_parts_with_glue_host(&reused, None, None, None).unwrap(), 0).unwrap();
    assert!(reused_again.parts["visio/media/original.bin"].bytes.ptr_eq(&reused.parts["visio/media/original.bin"].bytes));
    assert!(reused_again.pages[0].shapes.ptr_eq(&reused.pages[0].shapes));
    assert!(reused_again.parts[&reused.pages[0].part].xml.as_ref().unwrap().ptr_eq(reused.parts[&reused.pages[0].part].xml.as_ref().unwrap()));
    for (name, part) in &reused.parts {
        if reused_again.parts[name].bytes == part.bytes {
            assert!(reused_again.parts[name].bytes.ptr_eq(&part.bytes), "unchanged {name}");
        }
    }
}

#[test]
fn cached_import_keeps_the_exact_logical_node_refusal() {
    let doc = import(&write(&skeleton::document(false)).unwrap().bytes).unwrap();
    let held = MAX_RETAINED_XML_NODES - doc.retained_xml_nodes + 1;
    let bytes = write(&doc).unwrap().bytes;
    let ordinary = import_with_open_nodes(&bytes, held).unwrap_err().to_string();
    let cached = import_written_parts(write_parts_with_glue_host(&doc, None, None, None).unwrap(), held).unwrap_err().to_string();
    assert_eq!(cached, ordinary);
    assert!(cached.contains("retained XML node limit"));
}

#[test]
fn hostile_replaced_source_cannot_borrow_an_admitted_memo() {
    let mut doc = import(&write(&skeleton::document(false)).unwrap().bytes).unwrap();
    doc.parts.get_mut(&doc.main_part.clone()).unwrap().bytes = b"<!DOCTYPE broken [".to_vec().into();
    assert!(write_parts_with_glue_host(&doc, None, None, None).is_err());
}
