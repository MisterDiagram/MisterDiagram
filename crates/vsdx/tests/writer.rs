use diagramcraft_model::{Cell, Part, Row, Section, XmlElement, XmlNode, skeleton};
use diagramcraft_vsdx::{import, write};
#[test]
fn authored_skeleton_writes_complete_opc_and_recalculates_local_cells() {
    let mut d = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "Authored <text> & content").unwrap();
    let width = shape.sheet.cells.iter_mut().find(|c| c.name == "Width").unwrap();
    width.formula = Some("Height*3".into());
    width.value = Some("999".into());
    shape.sheet.cells.push(Cell { name: "FillForegnd".into(), formula: Some("RGB(204,0,0)".into()), value: Some("0".into()), ..Default::default() });
    shape.sheet.cells.push(Cell { name: "LineColor".into(), formula: Some("THEMEVAL()".into()), value: Some("Themed".into()), ..Default::default() });
    d.pages[0].shapes.push(shape);
    let written = write(&d).unwrap();
    let a = import(&written.bytes).unwrap();
    assert_eq!(a.parts.len(), 14);
    assert_eq!(a.pages[0].shapes[0].sheet.cell("Width").unwrap().value.as_deref(), Some("3"));
    let c = a.pages[0].shapes[0].sheet.cell("FillForegnd").unwrap();
    assert_eq!(c.value.as_deref(), Some("#cc0000"));
    assert!(c.formula.is_none());
    let c = a.pages[0].shapes[0].sheet.cell("LineColor").unwrap();
    assert_eq!(c.value.as_deref(), Some("Themed"));
    assert_eq!(c.formula.as_deref(), Some("THEMEVAL()"));
    assert_eq!(a.pages[0].shapes[0].text.as_ref().unwrap().text(), "Authored <text> & content\n");
    assert_eq!(a.styles.iter().map(|s| s.id).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 6, 7]);
    assert_eq!(a.parts["docProps/custom.xml"].xml.as_ref().unwrap().elements().find(|e| e.attr("name") == Some("IsMetric")).unwrap().text(), "false");
}
#[test]
fn unknown_parts_and_extensions_survive_modelled_edits() {
    let mut d = skeleton::document(false);
    d.parts.insert(
        "custom/opaque.bin".into(),
        Part { bytes: vec![0, 255, 128, 1, 2], content_type: Some("application/octet-stream".into()), ..Default::default() },
    );
    let mut e = XmlElement { name: "Extra".into(), namespace: "urn:diagramcraft:fixture".into(), ..Default::default() };
    e.attributes.insert("xmlns".into(), e.namespace.clone());
    e.children.push(XmlNode::Text("opaque extension".into()));
    d.parts.get_mut("visio/document.xml").unwrap().xml.as_mut().unwrap().children.push(XmlNode::Element(e));
    let bytes = write(&d).unwrap().bytes;
    let mut a = import(&bytes).unwrap();
    a.pages[0].name = "Changed".into();
    let b = import(&write(&a).unwrap().bytes).unwrap();
    assert_eq!(b.parts["custom/opaque.bin"].bytes, d.parts["custom/opaque.bin"].bytes);
    assert!(
        b.parts["visio/document.xml"]
            .xml
            .as_ref()
            .unwrap()
            .elements()
            .any(|e| e.namespace == "urn:diagramcraft:fixture" && e.text() == "opaque extension")
    );
    assert_eq!(b.pages[0].name, "Changed");
}
#[test]
fn instances_keep_only_local_overrides_and_master_relationships() {
    let mut d = skeleton::document(false);
    let mut s = diagramcraft_model::Shape { id: 1, master: Some(2), shape_type: Some("Shape".into()), ..Default::default() };
    s.sheet.cells.push(Cell { name: "BeginX".into(), value: Some("2".into()), ..Default::default() });
    d.pages[0].shapes.push(s);
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert_eq!(a.pages[0].shapes[0].sheet.cells.len(), 1);
    assert_eq!(a.parts["visio/pages/page1.xml"].relationships.iter().filter(|r| r.type_uri.ends_with("/master")).count(), 1);
}
#[test]
fn unsupported_recalculation_is_visible_without_fabricating_a_cache() {
    let mut d = skeleton::document(false);
    d.document_sheet.cells.push(Cell {
        name: "UnsupportedFixture".into(),
        formula: Some("NOTIMPLEMENTEDHERE()".into()),
        value: Some("23".into()),
        ..Default::default()
    });
    let written = write(&d).unwrap();
    assert!(written.warning_count > 0);
    assert!(written.warnings.iter().any(|w| w.contains("UnsupportedFixture")));
    let a = import(&written.bytes).unwrap();
    assert_eq!(a.document_sheet.cell("UnsupportedFixture").unwrap().value.as_deref(), Some("23"));
}
#[test]
fn writer_rejects_hostile_models_before_expansion() {
    let mut d = skeleton::document(false);
    d.pages.push(d.pages[0].clone());
    assert!(write(&d).is_err());
    let mut d = skeleton::document(false);
    d.parts.insert("../escape.bin".into(), Part::default());
    assert!(write(&d).is_err());
    let mut d = skeleton::document(false);
    d.pages[0].name = "bad\0name".into();
    assert!(write(&d).is_err());
    let mut d = skeleton::document(false);
    d.document_sheet.sections = vec![Section { name: "Geometry".into(), rows: vec![Row::default(); 100_000], ..Default::default() }];
    assert!(write(&d).is_err());
    let mut d = skeleton::document(false);
    d.document_sheet.cells.push(Cell { name: "TooLarge".into(), value: Some("x".repeat(17 * 1024 * 1024)), ..Default::default() });
    assert!(write(&d).is_err());
    let mut d = skeleton::document(false);
    let mut e = XmlElement { name: "Deep".into(), namespace: skeleton::VISIO_NS.into(), ..Default::default() };
    for _ in 0..300 {
        e = XmlElement { name: "Deep".into(), namespace: skeleton::VISIO_NS.into(), children: vec![XmlNode::Element(e)], ..Default::default() };
    }
    d.document_settings = Some(e);
    assert!(write(&d).is_err());
}
#[test]
fn private_probe_writes_reads_and_preserves_opaque_payloads() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let d = diagramcraft_vsdx::read(root.join("generated/basic/0001-probe.vsdx")).unwrap();
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert_eq!(a.pages.len(), d.pages.len());
    assert_eq!(a.masters.len(), d.masters.len());
    assert_eq!(a.parts.keys().collect::<Vec<_>>(), d.parts.keys().collect::<Vec<_>>());
    for (name, part) in &d.parts {
        if part.xml.is_none() && name != "[Content_Types].xml" && !name.ends_with(".rels") {
            assert_eq!(a.parts[name].bytes, part.bytes, "{name}");
        }
    }
}

#[test]
fn missing_and_authored_png_thumbnails_are_explicit_host_cases() {
    let mut d = skeleton::document(false);
    let missing = import(&write(&d).unwrap().bytes).unwrap();
    assert!(!missing.root_relationships.iter().any(|r| r.type_uri.ends_with("/thumbnail")));
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(2, 2, image::Rgba([255, 255, 255, 255])).write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    let png = bytes.into_inner();
    d.parts.insert("docProps/thumbnail.png".into(), Part { bytes: png.clone(), content_type: Some("image/png".into()), ..Default::default() });
    d.root_relationships.push(diagramcraft_model::Relationship {
        id: "rId5".into(),
        type_uri: "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail".into(),
        target: "docProps/thumbnail.png".into(),
        external: false,
    });
    let output = import(&write(&d).unwrap().bytes).unwrap();
    assert_eq!(output.parts["docProps/thumbnail.png"].bytes, png);
    assert_eq!(image::load_from_memory(&output.parts["docProps/thumbnail.png"].bytes).unwrap().width(), 2);
    // Host structural evidence only. Visio's acceptance is measured in M3.5.
}

#[test]
fn deleted_inherited_members_preserve_repeated_sentinel_ids() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let d = diagramcraft_vsdx::read(root.join("ms-shipped/1033/ASSOCIATIONS_UML_CLASS_M.vstx")).unwrap();
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let group = a.pages[0].shapes.iter().find(|s| s.id == 242).unwrap();
    let deleted: Vec<_> =
        group.shapes.iter().filter(|s| s.id == u32::MAX).map(|s| (s.master_shape, s.sheet.attributes.get("Del").map(String::as_str))).collect();
    assert_eq!(deleted, vec![(Some(7), Some("1")), (Some(9), Some("1"))]);
}

#[test]
fn shipped_missing_thumbnail_relationship_does_not_fabricate_a_part() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let d = diagramcraft_vsdx::read(root.join("ms-shipped/1033/AWSANALYTICS_M.vssx")).unwrap();
    assert!(!d.parts.contains_key("docProps/thumbnail.emf"));
    assert!(d.root_relationships.iter().any(|r| r.target == "docProps/thumbnail.emf"));
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert!(!a.root_relationships.iter().any(|r| r.target == "docProps/thumbnail.emf"));
    assert!(!a.parts.contains_key("docProps/thumbnail.emf"));
}

#[test]
fn serialization_does_not_apply_setf_actions_to_later_caches() {
    let mut d = skeleton::document(false);
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
    shape.sheet.cells.push(Cell {
        name: "Query".into(),
        formula: Some("SETF(GETREF(Width),9)+Width".into()),
        value: Some("0".into()),
        ..Default::default()
    });
    shape.sheet.cells.push(Cell { name: "Read".into(), formula: Some("Width*2".into()), value: Some("0".into()), ..Default::default() });
    d.pages[0].shapes.push(shape);
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert_eq!(a.pages[0].shapes[0].sheet.cell("Query").unwrap().value.as_deref(), Some("9"));
    assert_eq!(a.pages[0].shapes[0].sheet.cell("Read").unwrap().value.as_deref(), Some("4"));
    assert_eq!(a.pages[0].shapes[0].sheet.cell("Width").unwrap().value.as_deref(), Some("2"));
}

#[test]
fn stencil_page_sheet_does_not_gain_a_content_part() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    if !root.exists() {
        eprintln!("skip: private corpus/ is absent");
        return;
    }
    let d = diagramcraft_vsdx::read(root.join("ms-shipped/1033/BASIC_U.vssx")).unwrap();
    assert_eq!(d.pages.len(), 1);
    assert!(d.pages[0].part.is_empty());
    let a = import(&write(&d).unwrap().bytes).unwrap();
    assert_eq!(a.parts.keys().collect::<Vec<_>>(), d.parts.keys().collect::<Vec<_>>());
    assert!(a.pages[0].part.is_empty());
}

#[test]
fn destination_collisions_cannot_discard_a_page_or_opaque_payload() {
    let mut d = skeleton::document(false);
    let mut second = d.pages[0].clone();
    second.id = 1;
    d.pages.push(second);
    assert!(write(&d).unwrap_err().to_string().contains("duplicate modelled part"));
    let mut d = skeleton::document(false);
    d.parts.insert(
        "custom/opaque.bin".into(),
        Part { bytes: vec![1, 2, 3], content_type: Some("application/octet-stream".into()), ..Default::default() },
    );
    d.pages[0].part = "custom/opaque.bin".into();
    assert!(write(&d).unwrap_err().to_string().contains("overwrite"));
    d.parts.get_mut("custom/opaque.bin").unwrap().bytes.clear();
    assert!(write(&d).unwrap_err().to_string().contains("overwrite"));
}
#[test]
fn non_sheet_collections_are_bounded_before_cloning() {
    let mut d = skeleton::document(false);
    d.pages[0].connects = vec![diagramcraft_model::Connect::default(); 100_001];
    assert!(write(&d).unwrap_err().to_string().contains("connect count"));
    let mut d = skeleton::document(false);
    d.colors = (0..65537).map(|i| (i, "#000000".into())).collect();
    assert!(write(&d).unwrap_err().to_string().contains("palette/font count"));
    let mut d = skeleton::document(false);
    d.document_sheet.sections = vec![Section::default(); 16_385];
    assert!(write(&d).unwrap_err().to_string().contains("section limit"));
}

#[test]
fn recalculated_custom_colours_never_reference_a_transient_palette_index() {
    let mut d = skeleton::document(false);
    let mut s = skeleton::rectangle(1, 1., 1., 2., 1., "").unwrap();
    s.sheet.cells.push(Cell {
        name: "FillForegnd".into(),
        formula: Some("GUARD(RGB(123,45,67))".into()),
        value: Some("0".into()),
        ..Default::default()
    });
    d.pages[0].shapes.push(s);
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let c = a.pages[0].shapes[0].sheet.cell("FillForegnd").unwrap();
    assert_eq!(c.formula.as_deref(), Some("GUARD(RGB(123,45,67))"));
    assert_eq!(c.value.as_deref(), Some("#7b2d43"));
}

#[test]
fn escaped_sheet_size_and_expanded_attribute_duplicates_are_rejected() {
    let mut d = skeleton::document(false);
    d.document_sheet.cells.push(Cell { name: "Payload".into(), value: Some("&".repeat(4 * 1024 * 1024)), ..Default::default() });
    assert!(write(&d).unwrap_err().to_string().contains("sheet byte limit"));
    let mut d = skeleton::document(false);
    let e = d.parts.get_mut("visio/windows.xml").unwrap().xml.as_mut().unwrap();
    for (k, v) in [("xmlns:a", "urn:fixture"), ("xmlns:b", "urn:fixture"), ("a:id", "1"), ("b:id", "2")] {
        e.attributes.insert(k.into(), v.into());
    }
    assert!(write(&d).unwrap_err().to_string().contains("duplicate expanded attribute"));
    let mut d = skeleton::document(false);
    let mut second = skeleton::page(1, "Other", false);
    second.part.clear();
    d.pages.push(second);
    d.parts.insert("visio/pages/page2.xml".into(), Part { bytes: b"opaque-original".to_vec(), ..Default::default() });
    assert!(write(&d).unwrap_err().to_string().contains("overwrite"));
}

#[test]
fn deleted_sentinel_extensions_stay_with_their_master_shape() {
    let mut d = skeleton::document(false);
    for id in [7, 9] {
        let mut s = diagramcraft_model::Shape { id: u32::MAX, master_shape: Some(id), ..Default::default() };
        s.sheet.attributes.insert("Del".into(), "1".into());
        d.pages[0].shapes.push(s);
    }
    d.parts.get_mut("visio/pages/page1.xml").unwrap().xml = Some(diagramcraft_vsdx::parse_xml(format!(
        "<PageContents xmlns='{}'><Shapes><Shape ID='4294967295' MasterShape='7' Del='1'><Extra xmlns='urn:fixture'>seven</Extra></Shape><Shape ID='4294967295' MasterShape='9' Del='1'><Extra xmlns='urn:fixture'>nine</Extra></Shape></Shapes></PageContents>", skeleton::VISIO_NS).as_bytes()).unwrap());
    let a = import(&write(&d).unwrap().bytes).unwrap();
    let shapes = a.parts["visio/pages/page1.xml"].xml.as_ref().unwrap().child("Shapes").unwrap();
    let text: Vec<_> = shapes.elements().map(XmlElement::text).collect();
    assert_eq!(text, ["seven", "nine"]);
}
