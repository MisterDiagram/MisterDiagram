use misterdiagram_model::skeleton;
use misterdiagram_vsdx::{SaveKind, import, write, write_as};
#[test]
fn authored_stencil_and_template_retain_their_distinct_contracts() {
    let mut doc = skeleton::document(false);
    doc.pages[0].shapes.push(skeleton::rectangle(1, 1., 1., 2., 1., "Original page").unwrap());
    doc.pages.push(skeleton::page(1, "Second page", false));
    let stencil = import(&write_as(&doc, SaveKind::Stencil).unwrap().bytes).unwrap();
    assert_eq!(stencil.parts[&stencil.main_part].content_type.as_deref(), Some(SaveKind::Stencil.content_type()));
    assert_eq!(stencil.masters.len(), doc.masters.len());
    assert_eq!(stencil.pages.len(), 1);
    assert!(stencil.pages[0].part.is_empty());
    assert!(stencil.pages[0].shapes.is_empty());
    assert!(!stencil.parts.contains_key("visio/pages/page1.xml"));
    assert!(!stencil.parts.contains_key("visio/pages/_rels/pages.xml.rels"));
    let again = import(&write(&stencil).unwrap().bytes).unwrap();
    assert_eq!(again.parts.keys().collect::<Vec<_>>(), stencil.parts.keys().collect::<Vec<_>>());
    let template = import(&write_as(&doc, SaveKind::Template).unwrap().bytes).unwrap();
    assert_eq!(template.parts[&template.main_part].content_type.as_deref(), Some(SaveKind::Template.content_type()));
    assert_eq!(template.pages.len(), 2);
    assert_eq!(template.pages[0].shapes[0].text, doc.pages[0].shapes[0].text);
    assert_eq!(doc.pages.len(), 2);
    assert_eq!(doc.pages[0].shapes.len(), 1);
}
#[test]
fn existing_shipped_stencil_and_template_write_and_reopen() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    for (file, kind) in [("BASIC_U.vssx", SaveKind::Stencil), ("BASFLO_U.vstx", SaveKind::Template)] {
        let doc = misterdiagram_vsdx::read(root.join("ms-shipped/1033").join(file)).unwrap();
        let out = import(&write_as(&doc, kind).unwrap().bytes).unwrap();
        assert_eq!(out.parts[&out.main_part].content_type.as_deref(), Some(kind.content_type()));
        assert_eq!(out.masters.len(), doc.masters.len());
        assert_eq!(out.masters.iter().map(|m| &m.name_u).collect::<Vec<_>>(), doc.masters.iter().map(|m| &m.name_u).collect::<Vec<_>>());
    }
}
#[test]
fn format_conversion_rejects_hostile_models_before_cloning() {
    let mut doc = skeleton::document(false);
    doc.pages.push(doc.pages[0].clone());
    for kind in [SaveKind::Drawing, SaveKind::Stencil, SaveKind::Template] {
        assert!(write_as(&doc, kind).is_err());
    }
}

#[test]
fn stencil_preserves_extensions_and_reconciles_the_selected_page() {
    use misterdiagram_model::{Part, Relationship, XmlElement, XmlNode};
    let mut doc = skeleton::document(false);
    doc.pages.push(skeleton::page(1, "Selected", false));
    doc.document_settings.as_mut().unwrap().attributes.insert("TopPage".into(), "1".into());
    doc.parts.insert(
        "custom/opaque.bin".into(),
        Part { bytes: vec![1, 2, 3].into(), content_type: Some("application/octet-stream".into()), ..Default::default() },
    );
    let pages = doc.parts.get_mut("visio/pages/pages.xml").unwrap();
    pages.xml.as_mut().unwrap().children.push(XmlNode::Element(XmlElement {
        name: "Extra".into(),
        namespace: "urn:fixture".into(),
        children: vec![XmlNode::Text("kept".into())],
        ..Default::default()
    }));
    pages.relationships.push(Relationship {
        id: "extra".into(),
        type_uri: "urn:fixture".into(),
        target: "custom/opaque.bin".into(),
        external: false,
    });
    let stencil = import(&write_as(&doc, SaveKind::Stencil).unwrap().bytes).unwrap();
    let again = import(&write_as(&stencil, SaveKind::Stencil).unwrap().bytes).unwrap();
    assert_eq!(again.document_settings.as_ref().unwrap().attr("TopPage"), Some("0"));
    assert!(again.parts["visio/pages/pages.xml"].relationships.iter().any(|r| r.id == "extra"));
    assert_eq!(again.parts["custom/opaque.bin"].bytes.as_slice(), [1, 2, 3]);
    assert!(again.parts["visio/pages/pages.xml"].xml.as_ref().unwrap().elements().any(|e| e.namespace == "urn:fixture" && e.text() == "kept"));
}
